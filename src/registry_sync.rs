//! Bounded frozen-registry task infrastructure, NOT host content support or a join path.
//!
//! Audited against the official NeoForge 21.1.248 sources artifact (SHA-256
//! `b4b912457549ff586873ce78b39b08af96f11a0e3a54c6b77576c6a25907083f`),
//! verified against its official Maven publisher checksum. Extracted files matched
//! the archive. Paths below are under `net/neoforged/neoforge/`:
//!
//! * `network/payload/FrozenRegistrySyncStartPayload.java`:
//!   `b94ae9b7be2b068cbb1a052978b459766e3388d1cd1e5fb0d6e44aa50eda3f8f`
//!   declares a ResourceLocation list: VarInt count followed by identifiers.
//! * `network/payload/FrozenRegistrySyncCompletedPayload.java`:
//!   `6481b4edfd5de78b8c3aabc350a77f037e2742ae4ce6ae95c18ae8ba857135de`
//!   declares a unit codec: exactly zero body bytes under our strict framing.
//! * `network/configuration/SyncRegistries.java`:
//!   `3474ae1804eedf820ffd66fe8d34c3ac30d6659bb983472aa7f7c3589a3f9dce`
//!   sends start, snapshots, then completed. ClientPayloadHandler (pinned in
//!   provenance) checks missing registries and applies snapshots before replying;
//!   ServerPayloadHandler (`6b4340f4f87b19bfd403a32d277c45e40f664d7c33c9b55f7b3b71f3a9e82873`)
//!   finishes the task upon that reply.
//!
//! No Java differential execution is implied. Fixtures are synthetic. Duplicate
//! starts/names/snapshots, unexpected snapshots, trailing bytes, and all invalid
//! ordering fail CLOSED here, rather than emulating upstream collection overwrite
//! behavior. Existing Snapshot structural/alias checks and per-body limits apply.
//! Aggregate limits are LOCAL safety policy, not upstream pack compatibility.
//!
//! A RegistryMap is only numeric translation infrastructure. It is NEVER proof
//! of item/component, blockstate/chunk, entity, recipe, or other host semantics.
//! Only an explicit host semantic validation and atomic application barrier may
//! produce a CompletionAcknowledgment. There is deliberately no default host.
//! The surrounding connection must gate negotiation/configuration independently,
//! serialize delivery, and disconnect on error/panic/ack-send failure. No channels
//! are advertised or handlers installed by this module. Drop this task and its
//! host's connection-local effects on disconnect/reconfigure; create a fresh task
//! with a fresh epoch. Epochs are local tags, NOT fields on the wire: the transport
//! must exclude stale queued traffic itself. This module has no timeouts, packet
//! splitting, transport, global epoch allocator, or Horizon content implementation.

use crate::{
    codec::{Id, Reader, Writer},
    registry::Snapshot,
    Error, Result,
};
use std::collections::{BTreeMap, BTreeSet};

pub const START_CHANNEL: &str = "neoforge:frozen_registry_sync_start";
pub const SNAPSHOT_CHANNEL: &str = "neoforge:frozen_registry";
pub const COMPLETED_CHANNEL: &str = "neoforge:frozen_registry_sync_completed";
/// Hard local ceiling for the start list, independent of configurable budgets.
pub const MAX_SYNC_REGISTRIES: usize = 256;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FrozenRegistrySyncStartPayload {
    /// Wire order retained. Duplicate normalized names are rejected locally.
    pub to_access: Vec<Id>,
}
impl FrozenRegistrySyncStartPayload {
    pub fn decode(bytes: &[u8]) -> Result<Self> {
        let mut r = Reader::new(bytes)?;
        let count = r.count(MAX_SYNC_REGISTRIES)?;
        let mut to_access = Vec::with_capacity(count);
        let mut seen = BTreeSet::new();
        for _ in 0..count {
            let id = r.id()?;
            if !seen.insert(id.clone()) {
                return Err(Error::Duplicate(id.to_string()));
            }
            to_access.push(id);
        }
        r.finish()?;
        Ok(Self { to_access })
    }
    pub fn encode(&self) -> Result<Vec<u8>> {
        let mut w = Writer::new();
        w.count(self.to_access.len(), MAX_SYNC_REGISTRIES)?;
        let mut seen = BTreeSet::new();
        for id in &self.to_access {
            if !seen.insert(id) {
                return Err(Error::Duplicate(id.to_string()));
            }
            w.id(id)?;
        }
        Ok(w.finish())
    }
}

/// Wire value only; constructing/encoding this is NOT evidence of task completion.
/// Transport integrations must use the acknowledgment returned by `complete`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FrozenRegistrySyncCompletedPayload;
impl FrozenRegistrySyncCompletedPayload {
    pub fn decode(bytes: &[u8]) -> Result<Self> {
        Reader::new(bytes)?.finish()?;
        Ok(Self)
    }
    pub fn encode(&self) -> Vec<u8> {
        Vec::new()
    }
}

/// Aggregate limits across the entire task, in addition to Snapshot's individual
/// entry/alias caps and the codec's 32,767-byte body cap. Zero budgets are legal
/// (e.g. an empty start still needs one wire byte). Values are trusted host policy;
/// registries cannot exceed MAX_SYNC_REGISTRIES. Arithmetic is checked even for
/// usize::MAX budgets. Wire bytes include the start and all snapshot bodies as
/// RECEIVED, not re-encoded sizes; transport framing is excluded. Byte and count
/// caps jointly bound staging memory, including normalized identifier expansion;
/// bytes are not an exact Rust heap-allocation accounting or a host-work budget.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SyncLimits {
    pub max_registries: usize,
    pub max_total_entries: usize,
    pub max_total_aliases: usize,
    pub max_total_wire_bytes: usize,
}
impl Default for SyncLimits {
    fn default() -> Self {
        Self {
            max_registries: MAX_SYNC_REGISTRIES,
            max_total_entries: 16_384,
            max_total_aliases: 4_096,
            max_total_wire_bytes: 1_048_576,
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct SyncUsage {
    pub wire_bytes: usize,
    pub entries: usize,
    pub aliases: usize,
}

/// Implement only for a genuinely supported host content subset. All methods are
/// mandatory: no blanket implementation, numeric-map adapter, or default success.
///
/// CONTRACT (enforced by the implementor, not provable by this trait):
/// * supported_registries explicitly enumerates registries whose full required
///   semantics the host implements; it must remain stable throughout this task.
///   An unavailable/unsupported host returns Err, even for an empty task.
/// * validate_content validates EVERY entry, alias, and cross-registry dependency
///   against actual host semantics, not just name/number availability. It must
///   prepare the WHOLE batch without changing any externally visible host state.
///   Unsupported entries/properties/components/etc. must return Err. Prepared data
///   must remain valid through commit (host must prevent concurrent invalidation).
/// * commit_atomically applies the WHOLE prepared batch to actual packet/content
///   consumers in this epoch, including all required main-thread/queued work.
///   Ok means the work is FINISHED and visible atomically, never merely queued.
///   Err must leave ALL visible state unchanged (rollback is the host's duty).
///   Panics must not publish partial state either. Host-owned resources must have
///   appropriate bounded-work/allocation policies of their own.
///
/// A dishonest implementation can violate this contract: this infrastructure
/// cannot observe/prove engine semantics, implement rollback, or make a sequence
/// of engine writes atomic. The host must also revoke epoch state on disconnect.
pub trait RegistrySyncHost {
    type Prepared;
    fn supported_registries(&self) -> Result<&BTreeSet<Id>>;
    fn validate_content(
        &self,
        epoch: u64,
        snapshots: &BTreeMap<Id, Snapshot>,
    ) -> Result<Self::Prepared>;
    fn commit_atomically(&mut self, epoch: u64, prepared: Self::Prepared) -> Result<()>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SyncState {
    AwaitStart,
    Staging,
    Committed,
    Poisoned,
}

/// Produced only after the synchronous atomic host barrier succeeds. Not Clone:
/// a task cannot manufacture multiple successful acknowledgments by replaying
/// completion. The caller still owns exactly-once transport delivery policy.
#[derive(Debug, PartialEq, Eq)]
#[must_use = "send only on this epoch's configuration transport, after successful commit"]
pub struct CompletionAcknowledgment {
    epoch: u64,
}
impl CompletionAcknowledgment {
    pub fn epoch(&self) -> u64 {
        self.epoch
    }
    pub fn channel(&self) -> &'static str {
        COMPLETED_CHANNEL
    }
    pub fn body(&self) -> &'static [u8] {
        &[]
    }
}

/// Exactly one task, owning its host so callers cannot substitute another host
/// between validation and commit. Not Clone; no reset/retry on a failed epoch.
/// Each input method consumes one complete, already-routed custom-payload body.
/// Every returned error poisons the task and discards staged snapshots. A panic
/// during processing also leaves it Poisoned (unwinding is not caught here).
pub struct RegistrySync<H: RegistrySyncHost> {
    epoch: u64,
    host: H,
    limits: SyncLimits,
    state: SyncState,
    failure: Option<Error>,
    expected: BTreeSet<Id>,
    staged: BTreeMap<Id, Snapshot>,
    usage: SyncUsage,
}
impl<H: RegistrySyncHost> RegistrySync<H> {
    pub fn new(epoch: u64, host: H, limits: SyncLimits) -> Result<Self> {
        if limits.max_registries > MAX_SYNC_REGISTRIES {
            return Err(Error::Limit("registry sync registry budget"));
        }
        Ok(Self {
            epoch,
            host,
            limits,
            state: SyncState::AwaitStart,
            failure: None,
            expected: BTreeSet::new(),
            staged: BTreeMap::new(),
            usage: SyncUsage::default(),
        })
    }
    pub fn epoch(&self) -> u64 {
        self.epoch
    }
    pub fn state(&self) -> SyncState {
        self.state
    }
    /// First returned processing error; None if failure was an unwinding panic.
    pub fn failure(&self) -> Option<&Error> {
        self.failure.as_ref()
    }
    /// Accepted usage so far; failed inputs may be charged before their rejection.
    pub fn usage(&self) -> SyncUsage {
        self.usage
    }
    pub fn expected_registries(&self) -> &BTreeSet<Id> {
        &self.expected
    }
    pub fn staged_count(&self) -> usize {
        self.staged.len()
    }
    /// Inspection only; the host contract forbids interior-mutable invalidation.
    pub fn host(&self) -> &H {
        &self.host
    }
    /// Consumes this task; does not clear/revoke host effects. Host/connection
    /// teardown must do that before any new epoch is used.
    pub fn into_host(self) -> H {
        self.host
    }
    pub fn start(&mut self, bytes: &[u8]) -> Result<()> {
        self.step(SyncState::AwaitStart, SyncState::Staging, |this| {
            this.charge_bytes(bytes.len())?;
            let start = FrozenRegistrySyncStartPayload::decode(bytes)?;
            if start.to_access.len() > this.limits.max_registries {
                return Err(Error::Limit("registry sync registries"));
            }
            let supported = this.host.supported_registries()?;
            for registry in &start.to_access {
                if !supported.contains(registry) {
                    return Err(Error::UnsupportedTask(format!(
                        "{START_CHANNEL}: unsupported registry {registry}"
                    )));
                }
            }
            this.expected = start.to_access.into_iter().collect();
            Ok(())
        })
    }
    pub fn snapshot(&mut self, bytes: &[u8]) -> Result<()> {
        self.step(SyncState::Staging, SyncState::Staging, |this| {
            this.charge_bytes(bytes.len())?;
            let snapshot = Snapshot::decode(bytes)?;
            if !this.expected.contains(&snapshot.registry) {
                return Err(Error::Handler(format!(
                    "{SNAPSHOT_CHANNEL}: unexpected snapshot {}",
                    snapshot.registry
                )));
            }
            if this.staged.contains_key(&snapshot.registry) {
                return Err(Error::Duplicate(snapshot.registry.to_string()));
            }
            let entries = bounded_add(
                this.usage.entries,
                snapshot.ids.len(),
                this.limits.max_total_entries,
                "registry sync total entries",
            )?;
            let aliases = bounded_add(
                this.usage.aliases,
                snapshot.aliases.len(),
                this.limits.max_total_aliases,
                "registry sync total aliases",
            )?;
            this.usage.entries = entries;
            this.usage.aliases = aliases;
            this.staged.insert(snapshot.registry.clone(), snapshot);
            Ok(())
        })
    }
    /// Accept the server's empty completed body, validate the ENTIRE staged set,
    /// and synchronously commit it. Nothing escapes as an ack before commit Ok.
    /// Empty expected sets still invoke both host validation and commit; a unit
    /// codec alone never grants completion. All errors, including after commit
    /// (e.g. a replay), are terminal; the connection must disconnect/revoke state.
    pub fn complete(&mut self, bytes: &[u8]) -> Result<CompletionAcknowledgment> {
        self.step(SyncState::Staging, SyncState::Committed, |this| {
            this.charge_bytes(bytes.len())?;
            FrozenRegistrySyncCompletedPayload::decode(bytes)?;
            for registry in &this.expected {
                if !this.staged.contains_key(registry) {
                    return Err(Error::Missing(format!("{SNAPSHOT_CHANNEL}: {registry}")));
                }
            }
            let supported = this.host.supported_registries()?;
            for registry in &this.expected {
                if !supported.contains(registry) {
                    return Err(Error::UnsupportedTask(format!(
                        "{COMPLETED_CHANNEL}: unsupported registry {registry}"
                    )));
                }
            }
            // Move out before host callbacks: unwinding also drops all snapshots.
            let snapshots = std::mem::take(&mut this.staged);
            let prepared = this.host.validate_content(this.epoch, &snapshots)?;
            this.host.commit_atomically(this.epoch, prepared)?;
            Ok(CompletionAcknowledgment { epoch: this.epoch })
        })
    }
    fn charge_bytes(&mut self, bytes: usize) -> Result<()> {
        self.usage.wire_bytes = bounded_add(
            self.usage.wire_bytes,
            bytes,
            self.limits.max_total_wire_bytes,
            "registry sync total wire bytes",
        )?;
        Ok(())
    }
    fn step<T>(
        &mut self,
        required: SyncState,
        next: SyncState,
        f: impl FnOnce(&mut Self) -> Result<T>,
    ) -> Result<T> {
        if self.state == SyncState::Poisoned {
            self.staged.clear();
            return Err(self
                .failure
                .clone()
                .unwrap_or(Error::State("registry sync poisoned by panic")));
        }
        let correct = self.state == required;
        // Set BEFORE any parsing or host callback, so a caught panic cannot retry.
        self.state = SyncState::Poisoned;
        let result = if correct {
            f(self)
        } else {
            Err(Error::State("registry sync task ordering"))
        };
        match result {
            Ok(value) => {
                self.state = next;
                Ok(value)
            }
            Err(error) => {
                self.staged.clear();
                self.failure = Some(error.clone());
                Err(error)
            }
        }
    }
}
fn bounded_add(current: usize, added: usize, max: usize, name: &'static str) -> Result<usize> {
    current
        .checked_add(added)
        .filter(|n| *n <= max)
        .ok_or(Error::Limit(name))
}
