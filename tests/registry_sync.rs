//! Synthetic offline task tests; ModelHost is a tiny atomic semantic store for
//! testing the contract, NOT a Horizon host, advertised handler, or mod adapter.
use horizon_neoforge_compat::{
    codec::{Id, MAX_BODY},
    registry::{Snapshot, MAX_ALIASES, MAX_REGISTRY_ENTRIES},
    registry_sync::{
        FrozenRegistrySyncCompletedPayload as Completed, FrozenRegistrySyncStartPayload as Start,
        RegistrySync, RegistrySyncHost, SyncLimits, SyncState, COMPLETED_CHANNEL,
        MAX_SYNC_REGISTRIES,
    },
    Error, Result,
};
use std::{
    cell::Cell,
    collections::{BTreeMap, BTreeSet},
    panic::{catch_unwind, AssertUnwindSafe},
};
fn id(name: &str) -> Id {
    Id::parse(name).unwrap()
}
fn start(names: &[&str]) -> Vec<u8> {
    Start {
        to_access: names.iter().map(|s| id(s)).collect(),
    }
    .encode()
    .unwrap()
}
fn snapshot(registry: &str, entries: &[(i32, &str)], aliases: &[(&str, &str)]) -> Snapshot {
    Snapshot {
        registry: id(registry),
        ids: entries.iter().map(|(n, s)| (*n, id(s))).collect(),
        aliases: aliases.iter().map(|(a, b)| (id(a), id(b))).collect(),
    }
}
fn a() -> Snapshot {
    snapshot(
        "test:a",
        &[(7, "test:alpha")],
        &[("test:old", "test:alpha")],
    )
}
fn b() -> Snapshot {
    snapshot("test:b", &[(300, "test:beta")], &[])
}
#[derive(Debug, Clone, PartialEq, Eq)]
enum Meaning {
    Alpha,
    Requires(Id, Id),
}
#[derive(Debug, Clone, PartialEq, Eq)]
struct Applied {
    entries: BTreeMap<i32, Meaning>,
    aliases: BTreeMap<Id, Id>,
}
type Batch = BTreeMap<Id, Applied>;
struct ModelHost {
    supported: BTreeSet<Id>,
    catalog: BTreeMap<Id, BTreeMap<Id, Meaning>>,
    visible: Batch,
    visible_epoch: Option<u64>,
    validations: Cell<usize>,
    commits: usize,
    unavailable: bool,
    fail_commit: bool,
    panic_validate: bool,
    panic_commit: bool,
}
impl ModelHost {
    fn new() -> Self {
        Self {
            supported: BTreeSet::from([id("test:a"), id("test:b")]),
            catalog: BTreeMap::from([
                (
                    id("test:a"),
                    BTreeMap::from([(id("test:alpha"), Meaning::Alpha)]),
                ),
                (
                    id("test:b"),
                    BTreeMap::from([(
                        id("test:beta"),
                        Meaning::Requires(id("test:a"), id("test:alpha")),
                    )]),
                ),
            ]),
            // Sentinel proves failure doesn't clear or partially replace old state.
            visible: BTreeMap::from([(
                id("test:previous"),
                Applied {
                    entries: BTreeMap::from([(999, Meaning::Alpha)]),
                    aliases: BTreeMap::new(),
                },
            )]),
            visible_epoch: Some(40),
            validations: Cell::new(0),
            commits: 0,
            unavailable: false,
            fail_commit: false,
            panic_validate: false,
            panic_commit: false,
        }
    }
}
impl RegistrySyncHost for ModelHost {
    type Prepared = Batch;
    fn supported_registries(&self) -> Result<&BTreeSet<Id>> {
        if self.unavailable {
            return Err(Error::Handler("semantic host unavailable".into()));
        }
        Ok(&self.supported)
    }
    fn validate_content(&self, _: u64, snapshots: &BTreeMap<Id, Snapshot>) -> Result<Batch> {
        self.validations.set(self.validations.get() + 1);
        assert!(!self.panic_validate, "validation panic before publication");
        let mut prepared = BTreeMap::new();
        for (registry, snapshot) in snapshots {
            let catalog = self.catalog.get(registry).ok_or_else(|| {
                Error::Handler(format!("no semantic implementation for {registry}"))
            })?;
            let mut entries = BTreeMap::new();
            for (number, name) in &snapshot.ids {
                let meaning = catalog
                    .get(name)
                    .ok_or_else(|| Error::Handler(format!("unknown semantic content {name}")))?;
                if let Meaning::Requires(target_registry, target_name) = meaning {
                    if !snapshots
                        .get(target_registry)
                        .is_some_and(|s| s.ids.values().any(|name| name == target_name))
                    {
                        return Err(Error::Handler("missing semantic dependency".into()));
                    }
                }
                entries.insert(*number, meaning.clone());
            }
            // Snapshot has already validated alias resolution; retain it as part
            // of this model store's applied semantics rather than ignoring it.
            prepared.insert(
                registry.clone(),
                Applied {
                    entries,
                    aliases: snapshot.aliases.clone(),
                },
            );
        }
        Ok(prepared)
    }
    fn commit_atomically(&mut self, epoch: u64, prepared: Batch) -> Result<()> {
        self.commits += 1;
        assert!(!self.panic_commit, "commit panic before publication");
        if self.fail_commit {
            return Err(Error::Handler("atomic host refused commit".into()));
        }
        // Entire actual model content replaced in one synchronous barrier; no
        // callbacks/other observers can run between these assignments.
        self.visible = prepared;
        self.visible_epoch = Some(epoch);
        Ok(())
    }
}
fn task_with(host: ModelHost, limits: SyncLimits) -> RegistrySync<ModelHost> {
    RegistrySync::new(41, host, limits).unwrap()
}
fn task() -> RegistrySync<ModelHost> {
    task_with(ModelHost::new(), SyncLimits::default())
}
fn stage_two(task: &mut RegistrySync<ModelHost>) {
    task.start(&start(&["test:a", "test:b"])).unwrap();
    task.snapshot(&b().encode().unwrap()).unwrap(); // arrival order irrelevant
    task.snapshot(&a().encode().unwrap()).unwrap();
}
fn assert_terminal(task: &mut RegistrySync<ModelHost>, first: Error) {
    assert_eq!(task.state(), SyncState::Poisoned);
    assert_eq!(task.failure(), Some(&first));
    assert_eq!(task.staged_count(), 0);
    assert_eq!(task.start(&[0]), Err(first.clone()));
    assert_eq!(task.snapshot(&a().encode().unwrap()), Err(first.clone()));
    assert_eq!(task.complete(&[]), Err(first));
}

#[test]
fn start_literal_wire_order_and_empty_codec() {
    // Independent literal specification: count=2, string lengths=6,6.
    let bytes = b"\x02\x06test:b\x06test:a";
    let decoded = Start::decode(bytes).unwrap();
    assert_eq!(decoded.to_access, vec![id("test:b"), id("test:a")]);
    assert_eq!(decoded.encode().unwrap(), bytes);
    assert_eq!(Start { to_access: vec![] }.encode().unwrap(), [0]);
    assert_eq!(Start::decode(&[0]).unwrap().to_access.len(), 0);
    assert!(Start::decode(&[]).is_err()); // empty list is not empty body
    assert_eq!(Completed::decode(&[]), Ok(Completed));
    assert!(Completed.encode().is_empty());
    assert_eq!(Completed::decode(&[0]), Err(Error::Trailing));
}
#[test]
fn start_every_truncation_trailing_and_malformed() {
    let bytes = b"\x02\x06test:b\x06test:a";
    for n in 0..bytes.len() {
        assert!(Start::decode(&bytes[..n]).is_err(), "prefix {n}");
    }
    let mut trailing = bytes.to_vec();
    trailing.push(0);
    assert_eq!(Start::decode(&trailing), Err(Error::Trailing));
    for malformed in [
        &[255, 255, 255, 255, 15][..], // negative count
        &[129, 2],                     // 257 registries
        &[128, 128, 128, 128, 16],     // VarInt overflow
        &[1, 255, 255, 255, 255, 15],
        &[1, 1, 255], // invalid UTF-8
        b"\x01\x03X:a",
        b"\x01\x00",
    ] {
        assert!(Start::decode(malformed).is_err(), "{malformed:?}");
    }
}
#[test]
fn duplicate_start_names_rejected_on_wire_and_encode() {
    assert!(matches!(
        Start::decode(b"\x02\x01a\x0bminecraft:a"),
        Err(Error::Duplicate(_))
    ));
    assert!(matches!(
        Start {
            to_access: vec![id("a"), id("minecraft:a")]
        }
        .encode(),
        Err(Error::Duplicate(_))
    ));
}
#[test]
fn start_count_and_body_boundaries() {
    let payload = Start {
        to_access: (0..MAX_SYNC_REGISTRIES)
            .map(|n| id(&format!("t:r{n}")))
            .collect(),
    };
    assert_eq!(Start::decode(&payload.encode().unwrap()).unwrap(), payload);
    let mut too_many = payload;
    too_many.to_access.push(id("t:extra"));
    assert!(too_many.encode().is_err());
    let huge_names = Start {
        to_access: (0..MAX_SYNC_REGISTRIES)
            .map(|n| id(&format!("t:{n:0250}")))
            .collect(),
    };
    assert!(huge_names.encode().is_err());
    assert!(Start::decode(&vec![0; MAX_BODY + 1]).is_err());
    assert!(Completed::decode(&vec![0; MAX_BODY + 1]).is_err());
}
#[test]
fn no_publication_until_complete_atomic_barrier() {
    let mut sync = task();
    let old = sync.host().visible.clone();
    assert_eq!(sync.epoch(), 41);
    assert_eq!(sync.state(), SyncState::AwaitStart);
    stage_two(&mut sync);
    assert_eq!(sync.host().visible, old);
    assert_eq!(sync.host().validations.get(), 0);
    assert_eq!(sync.host().commits, 0);
    assert_eq!(sync.expected_registries().len(), 2);
    assert_eq!(sync.staged_count(), 2);
    assert_eq!(sync.usage().entries, 2);
    assert_eq!(sync.usage().aliases, 1);
    let ack = sync.complete(&[]).unwrap();
    // Merely returning the acknowledgment already guarantees publication.
    assert_eq!(sync.host().visible_epoch, Some(ack.epoch()));
    assert_eq!(ack.epoch(), 41);
    assert_eq!(ack.channel(), COMPLETED_CHANNEL);
    assert!(ack.body().is_empty());
    assert_eq!(sync.state(), SyncState::Committed);
    assert_eq!(sync.staged_count(), 0);
    assert_eq!(sync.host().validations.get(), 1);
    assert_eq!(sync.host().commits, 1);
    assert_eq!(sync.host().visible.len(), 2);
    assert_eq!(
        sync.host().visible[&id("test:a")].entries[&7],
        Meaning::Alpha
    );
    assert_eq!(sync.host().visible[&id("test:a")].aliases, a().aliases);
    assert_eq!(
        sync.usage().wire_bytes,
        start(&["test:a", "test:b"]).len()
            + a().encode().unwrap().len()
            + b().encode().unwrap().len()
    );
}
#[test]
fn empty_expected_set_still_requires_host_validation_and_commit() {
    let mut sync = task();
    sync.start(&[0]).unwrap();
    let ack = sync.complete(&[]).unwrap();
    assert_eq!(ack.epoch(), 41);
    assert!(sync.host().visible.is_empty());
    assert_eq!(sync.host().validations.get(), 1);
    assert_eq!(sync.host().commits, 1);
}
#[test]
fn unsupported_host_refuses_even_empty_task() {
    let mut host = ModelHost::new();
    host.unavailable = true;
    let mut sync = task_with(host, SyncLimits::default());
    let old = sync.host().visible.clone();
    let error = sync.start(&[0]).unwrap_err();
    assert_terminal(&mut sync, error);
    assert_eq!(sync.host().visible, old);
    assert_eq!(sync.host().commits, 0);
}
#[test]
fn unsupported_expected_registry_fails_at_start() {
    let mut sync = task();
    let error = sync
        .start(&start(&["test:a", "test:unsupported"]))
        .unwrap_err();
    assert!(matches!(error, Error::UnsupportedTask(_)));
    assert_terminal(&mut sync, error);
    assert_eq!(sync.host().validations.get(), 0);
}
#[test]
fn duplicate_unexpected_and_missing_snapshots_are_terminal() {
    let mut duplicate = task();
    duplicate.start(&start(&["test:a", "test:b"])).unwrap();
    duplicate.snapshot(&a().encode().unwrap()).unwrap();
    let error = duplicate.snapshot(&a().encode().unwrap()).unwrap_err();
    assert!(matches!(error, Error::Duplicate(_)));
    assert_terminal(&mut duplicate, error);

    let mut unexpected = task();
    unexpected.start(&start(&["test:a"])).unwrap();
    let error = unexpected.snapshot(&b().encode().unwrap()).unwrap_err();
    assert_terminal(&mut unexpected, error);

    let mut missing = task();
    missing.start(&start(&["test:a", "test:b"])).unwrap();
    missing.snapshot(&a().encode().unwrap()).unwrap();
    let error = missing.complete(&[]).unwrap_err();
    assert!(matches!(error, Error::Missing(_)));
    assert_terminal(&mut missing, error);
    for sync in [&duplicate, &unexpected, &missing] {
        assert_eq!(sync.host().validations.get(), 0);
        assert_eq!(sync.host().commits, 0);
        assert!(sync.host().visible.contains_key(&id("test:previous")));
    }
}
#[test]
fn all_out_of_order_calls_poison_including_replays_after_commit() {
    for method in 0..2 {
        let mut sync = task();
        let error = if method == 0 {
            sync.snapshot(&a().encode().unwrap()).unwrap_err()
        } else {
            sync.complete(&[]).unwrap_err()
        };
        assert_terminal(&mut sync, error);
    }
    let mut sync = task();
    sync.start(&[0]).unwrap();
    let error = sync.start(&[0]).unwrap_err();
    assert_terminal(&mut sync, error);
    for method in 0..3 {
        let mut sync = task();
        stage_two(&mut sync);
        let _ack = sync.complete(&[]).unwrap();
        let applied = sync.host().visible.clone();
        let error = match method {
            0 => sync.start(&[0]).unwrap_err(),
            1 => sync.snapshot(&a().encode().unwrap()).unwrap_err(),
            _ => sync.complete(&[]).unwrap_err(),
        };
        assert_terminal(&mut sync, error);
        assert_eq!(sync.host().commits, 1);
        assert_eq!(sync.host().visible, applied); // revocation is host teardown's duty
    }
}
#[test]
fn malformed_lifecycle_bodies_poison_without_commit() {
    let mut sync = task();
    let error = sync.start(&[]).unwrap_err();
    assert_terminal(&mut sync, error);
    let mut sync = task();
    sync.start(&start(&["test:a"])).unwrap();
    let error = sync.snapshot(&[0]).unwrap_err();
    assert_terminal(&mut sync, error);
    let mut sync = task();
    stage_two(&mut sync);
    let error = sync.complete(&[0]).unwrap_err();
    assert_terminal(&mut sync, error);
    assert_eq!(sync.host().commits, 0);
}
#[test]
fn structural_alias_failures_never_reach_semantic_host() {
    // Literal snapshot test:a, 0 IDs, 1 alias test:x -> test:x (cycle).
    let cycle = b"\x06test:a\x00\x01\x06test:x\x06test:x";
    let dangling = b"\x06test:a\x00\x01\x06test:x\x06test:y";
    let duplicate_id = b"\x06test:a\x02\x00\x06test:x\x00\x06test:y\x00";
    for bytes in [cycle.as_slice(), dangling, duplicate_id] {
        let mut sync = task();
        sync.start(&start(&["test:a"])).unwrap();
        let error = sync.snapshot(bytes).unwrap_err();
        assert_terminal(&mut sync, error);
        assert_eq!(sync.host().validations.get(), 0);
        assert_eq!(sync.host().commits, 0);
    }
}
#[test]
fn semantic_unknown_entry_refused_without_partial_publication() {
    let mut sync = task();
    let old = sync.host().visible.clone();
    sync.start(&start(&["test:a", "test:b"])).unwrap();
    sync.snapshot(&a().encode().unwrap()).unwrap();
    sync.snapshot(
        &snapshot("test:b", &[(0, "test:unknown")], &[])
            .encode()
            .unwrap(),
    )
    .unwrap();
    let error = sync.complete(&[]).unwrap_err();
    assert_terminal(&mut sync, error);
    assert_eq!(sync.host().visible, old);
    assert_eq!(sync.host().validations.get(), 1);
    assert_eq!(sync.host().commits, 0);
}
#[test]
fn cross_registry_semantic_dependencies_must_be_validated() {
    let mut sync = task();
    sync.start(&start(&["test:b"])).unwrap();
    sync.snapshot(&b().encode().unwrap()).unwrap();
    let error = sync.complete(&[]).unwrap_err();
    assert_eq!(error, Error::Handler("missing semantic dependency".into()));
    assert_terminal(&mut sync, error);
    assert_eq!(sync.host().commits, 0);
}
#[test]
fn explicit_commit_failure_keeps_all_old_state_and_never_returns_ack() {
    let mut host = ModelHost::new();
    host.fail_commit = true;
    let old = host.visible.clone();
    let mut sync = task_with(host, SyncLimits::default());
    stage_two(&mut sync);
    let error = sync.complete(&[]).unwrap_err();
    assert_terminal(&mut sync, error);
    assert_eq!(sync.host().visible, old);
    assert_eq!(sync.host().visible_epoch, Some(40));
    assert_eq!(sync.host().validations.get(), 1);
    assert_eq!(sync.host().commits, 1);
}
#[test]
fn caught_host_panics_cannot_reopen_epoch() {
    for in_commit in [false, true] {
        let mut host = ModelHost::new();
        host.panic_validate = !in_commit;
        host.panic_commit = in_commit;
        let old = host.visible.clone();
        let mut sync = task_with(host, SyncLimits::default());
        stage_two(&mut sync);
        assert!(catch_unwind(AssertUnwindSafe(|| sync.complete(&[]))).is_err());
        assert_eq!(sync.state(), SyncState::Poisoned);
        assert_eq!(sync.staged_count(), 0);
        assert!(sync.complete(&[]).is_err());
        assert_eq!(sync.host().visible, old);
        assert_eq!(sync.host().validations.get(), 1);
        assert_eq!(sync.host().commits, usize::from(in_commit));
    }
}
#[test]
fn registry_budget_is_separate_from_wire_codec_limit() {
    let limits = SyncLimits {
        max_registries: 1,
        ..SyncLimits::default()
    };
    let mut sync = task_with(ModelHost::new(), limits);
    let error = sync.start(&start(&["test:a", "test:b"])).unwrap_err();
    assert_terminal(&mut sync, error);
    let limits = SyncLimits {
        max_registries: MAX_SYNC_REGISTRIES + 1,
        ..limits
    };
    assert!(RegistrySync::new(1, ModelHost::new(), limits).is_err());
}
#[test]
fn aggregate_entry_budget_exact_boundary_and_cross_registry_overflow() {
    let limits = SyncLimits {
        max_total_entries: 1,
        ..SyncLimits::default()
    };
    let mut sync = task_with(ModelHost::new(), limits);
    sync.start(&start(&["test:a", "test:b"])).unwrap();
    sync.snapshot(&a().encode().unwrap()).unwrap();
    assert_eq!(sync.usage().entries, 1);
    let error = sync.snapshot(&b().encode().unwrap()).unwrap_err();
    assert_eq!(error, Error::Limit("registry sync total entries"));
    assert_terminal(&mut sync, error);
    assert_eq!(sync.host().commits, 0);
    let mut sync = task_with(ModelHost::new(), limits);
    sync.start(&start(&["test:a"])).unwrap();
    sync.snapshot(&a().encode().unwrap()).unwrap();
    let _ack = sync.complete(&[]).unwrap();
}
#[test]
fn aggregate_alias_budget_crosses_registries_and_accepts_exact_boundary() {
    let limits = SyncLimits {
        max_total_aliases: 1,
        ..SyncLimits::default()
    };
    let second = snapshot(
        "test:b",
        &[(300, "test:beta")],
        &[("test:old_b", "test:beta")],
    );
    let mut sync = task_with(ModelHost::new(), limits);
    sync.start(&start(&["test:a", "test:b"])).unwrap();
    sync.snapshot(&a().encode().unwrap()).unwrap();
    assert_eq!(sync.usage().aliases, 1);
    let error = sync.snapshot(&second.encode().unwrap()).unwrap_err();
    assert_eq!(error, Error::Limit("registry sync total aliases"));
    assert_terminal(&mut sync, error);
    let mut sync = task_with(ModelHost::new(), limits);
    stage_two(&mut sync); // exactly one alias across both registries
    let _ack = sync.complete(&[]).unwrap();
}
#[test]
fn aggregate_wire_bytes_count_received_not_canonical_encoding() {
    // Nonminimal positive count is accepted by existing Reader policy. It must
    // consume two bytes of budget, not the one byte its re-encoding would use.
    let mut sync = task_with(
        ModelHost::new(),
        SyncLimits {
            max_total_wire_bytes: 1,
            ..SyncLimits::default()
        },
    );
    let error = sync.start(&[128, 0]).unwrap_err();
    assert_terminal(&mut sync, error);
    let mut sync = task_with(
        ModelHost::new(),
        SyncLimits {
            max_total_wire_bytes: 2,
            ..SyncLimits::default()
        },
    );
    sync.start(&[128, 0]).unwrap();
    assert_eq!(sync.usage().wire_bytes, 2);
    let _ack = sync.complete(&[]).unwrap();

    let bytes = start(&["test:a", "test:b"]).len()
        + a().encode().unwrap().len()
        + b().encode().unwrap().len();
    for delta in [0, 1] {
        let mut sync = task_with(
            ModelHost::new(),
            SyncLimits {
                max_total_wire_bytes: bytes - delta,
                ..SyncLimits::default()
            },
        );
        sync.start(&start(&["test:a", "test:b"])).unwrap();
        sync.snapshot(&a().encode().unwrap()).unwrap();
        let result = sync.snapshot(&b().encode().unwrap());
        if delta == 0 {
            result.unwrap();
            let _ack = sync.complete(&[]).unwrap();
        } else {
            assert_terminal(&mut sync, result.unwrap_err());
        }
    }
}
#[test]
fn per_snapshot_limits_not_relaxed_by_large_aggregate_budgets() {
    // Start/snapshot bounds still apply even when aggregate budgets are huge.
    let limits = SyncLimits {
        max_total_entries: usize::MAX,
        max_total_aliases: usize::MAX,
        max_total_wire_bytes: usize::MAX,
        ..SyncLimits::default()
    };
    let mut sync = task_with(ModelHost::new(), limits);
    sync.start(&start(&["test:a"])).unwrap();
    let error = sync.snapshot(&vec![0; MAX_BODY + 1]).unwrap_err();
    assert_terminal(&mut sync, error);
    for (entries, aliases) in [(MAX_REGISTRY_ENTRIES + 1, 0), (0, MAX_ALIASES + 1)] {
        // Counts can be tested without allocating entries: the reader rejects
        // the over-limit count BEFORE trying to decode the absent contents.
        let mut bytes = b"\x06test:a".to_vec();
        for count in [entries, aliases] {
            let mut n = count;
            loop {
                let b = (n & 127) as u8;
                n >>= 7;
                bytes.push(if n == 0 { b } else { b | 128 });
                if n == 0 {
                    break;
                }
            }
        }
        let mut sync = task_with(ModelHost::new(), limits);
        sync.start(&start(&["test:a"])).unwrap();
        let error = sync.snapshot(&bytes).unwrap_err();
        assert_terminal(&mut sync, error);
    }
}
#[test]
fn new_epoch_does_not_inherit_prior_staging_or_failure() {
    let mut old = task();
    old.start(&start(&["test:a", "test:b"])).unwrap();
    old.snapshot(&a().encode().unwrap()).unwrap();
    assert!(old.complete(&[]).is_err());
    drop(old);
    let mut fresh = RegistrySync::new(42, ModelHost::new(), SyncLimits::default()).unwrap();
    assert_eq!(fresh.staged_count(), 0);
    assert_eq!(fresh.usage().wire_bytes, 0);
    fresh.start(&start(&["test:a", "test:b"])).unwrap();
    fresh.snapshot(&b().encode().unwrap()).unwrap();
    // No previous epoch's test:a snapshot can satisfy a missing registry.
    let error = fresh.complete(&[]).unwrap_err();
    assert!(matches!(&error, Error::Missing(message) if message.contains("test:a")));
    assert_terminal(&mut fresh, error);
    let mut fresh = RegistrySync::new(43, ModelHost::new(), SyncLimits::default()).unwrap();
    stage_two(&mut fresh);
    assert_eq!(fresh.complete(&[]).unwrap().epoch(), 43);
    let host = fresh.into_host();
    assert_eq!(host.visible_epoch, Some(43));
}
#[test]
fn deterministic_malformed_smoke_corpus_has_no_panics() {
    let mut seed = 0x51a7_u32;
    for len in (0..128).cycle().take(4096) {
        let bytes: Vec<_> = (0..len)
            .map(|_| {
                seed ^= seed << 13;
                seed ^= seed >> 17;
                seed ^= seed << 5;
                seed as u8
            })
            .collect();
        let _ = Start::decode(&bytes);
        let _ = Completed::decode(&bytes);
        let mut sync = task();
        sync.start(&start(&["test:a", "test:b"])).unwrap();
        if let Err(error) = sync.snapshot(&bytes) {
            assert_terminal(&mut sync, error);
        }
    }
}
