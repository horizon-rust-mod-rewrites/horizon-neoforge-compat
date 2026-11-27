use horizon_neoforge_compat::{
    check_baseline, codec::*, extension::Extensions, registry::*, session::*, Error, Result,
};
use std::collections::{BTreeMap, BTreeSet};
fn id(s: &str) -> Id {
    Id::parse(s).unwrap()
}
fn session() -> Session {
    Session::new("1.21.1", 767, "21.1.248", Extensions::new()).unwrap()
}
fn common(s: &mut Session) {
    s.receive(
        "c:version",
        include_bytes!("../fixtures/common-version-v1.bin"),
    )
    .unwrap();
    s.receive(
        "c:register",
        include_bytes!("../fixtures/common-register-empty-play.bin"),
    )
    .unwrap();
}
fn snapshot() -> Snapshot {
    Snapshot::decode(include_bytes!("../fixtures/frozen-items.bin")).unwrap()
}
fn accept_empty(b: &[u8]) -> Result<()> {
    if b.is_empty() {
        Ok(())
    } else {
        Err(Error::Handler("expected empty".into()))
    }
}
#[test]
fn version_golden() {
    let bytes = include_bytes!("../fixtures/common-version-v1.bin");
    let v = CommonVersion::decode(bytes).unwrap();
    assert_eq!(v.versions, vec![1]);
    assert_eq!(v.encode().unwrap(), bytes);
}
#[test]
fn signed_varint_golden() {
    let bytes = include_bytes!("../fixtures/common-version-signed.bin");
    let v = CommonVersion::decode(bytes).unwrap();
    assert_eq!(v.versions, vec![1, 127, -1]);
    assert_eq!(v.encode().unwrap(), bytes);
}
#[test]
fn register_golden() {
    let bytes = include_bytes!("../fixtures/common-register-demo.bin");
    let v = CommonRegister::decode(bytes).unwrap();
    assert_eq!(v.version, 1);
    assert_eq!(v.phase, Phase::Play);
    assert_eq!(v.channels, BTreeSet::from([id("demo:ping")]));
    assert_eq!(v.encode().unwrap(), bytes);
}
#[test]
fn dinnerbone_golden() {
    let bytes = include_bytes!("../fixtures/dinnerbone-demo.bin");
    let v = decode_dinnerbone(bytes).unwrap();
    assert_eq!(v, BTreeSet::from([id("demo:ping"), id("demo:pong")]));
    assert_eq!(encode_dinnerbone(&v).unwrap(), bytes);
}
#[test]
fn dinnerbone_shorthand_and_empty() {
    assert_eq!(
        decode_dinnerbone(b"\0stone\0minecraft:stone\0\0").unwrap(),
        BTreeSet::from([id("minecraft:stone")])
    );
    assert_eq!(
        decode_dinnerbone(b"demo:ping").unwrap(),
        BTreeSet::from([id("demo:ping")])
    );
}
#[test]
fn invalid_dinnerbone_is_not_silently_skipped() {
    for bytes in [b"Demo:bad".as_slice(), b"demo:\xff", b"demo:a:b"] {
        assert!(decode_dinnerbone(bytes).is_err());
    }
}
#[test]
fn identifier_grammar() {
    assert_eq!(id(":stone"), id("minecraft:stone"));
    assert_eq!(id("stone"), id("minecraft:stone"));
    for s in ["", "demo:", "DEMO:a", "demo:a b", "demo:a:b", "démø:a"] {
        assert!(Id::parse(s).is_err(), "{s}");
    }
    assert!(Id::parse(&"a".repeat(MAX_ID + 1)).is_err());
}
#[test]
fn overlong_varint_and_overflow() {
    for b in [&[0x80; 6][..], &[0xff, 0xff, 0xff, 0xff, 0x10], &[0x80]] {
        assert!(CommonVersion::decode(b).is_err());
    }
}
#[test]
fn negative_and_large_counts() {
    assert!(CommonVersion::decode(&[0xff, 0xff, 0xff, 0xff, 0x0f]).is_err());
    assert!(CommonVersion::decode(&[33]).is_err());
    assert!(CommonVersion {
        versions: vec![1; 33]
    }
    .encode()
    .is_err());
}
#[test]
fn body_limit() {
    let b = vec![0; MAX_BODY + 1];
    assert!(CommonVersion::decode(&b).is_err());
    assert!(CommonRegister::decode(&b).is_err());
    assert!(decode_dinnerbone(&b).is_err());
    assert!(Snapshot::decode(&b).is_err());
}
#[test]
fn trailing_bytes_rejected() {
    assert_eq!(CommonVersion::decode(&[1, 1, 0]), Err(Error::Trailing));
    let mut bytes = include_bytes!("../fixtures/common-register-demo.bin").to_vec();
    bytes.push(0);
    assert_eq!(CommonRegister::decode(&bytes), Err(Error::Trailing));
    let mut bytes = snapshot().encode().unwrap();
    bytes.push(0);
    assert_eq!(Snapshot::decode(&bytes), Err(Error::Trailing));
}
#[test]
fn every_prefix_truncation_rejected() {
    let v = include_bytes!("../fixtures/common-register-demo.bin");
    for n in 0..v.len() {
        assert!(CommonRegister::decode(&v[..n]).is_err(), "prefix {n}");
    }
    let v = include_bytes!("../fixtures/frozen-items.bin");
    for n in 0..v.len() {
        assert!(Snapshot::decode(&v[..n]).is_err(), "prefix {n}");
    }
}
#[test]
fn malformed_string_utf8_and_count() {
    assert_eq!(CommonRegister::decode(&[1, 1, 255, 0]), Err(Error::Utf8));
    assert!(CommonRegister::decode(&[1, 0xff, 0x7f]).is_err());
    assert!(CommonRegister::decode(&[1, 1, b'x', 0]).is_err());
}
#[test]
fn duplicate_register_fails() {
    let mut b = vec![1, 4, b'p', b'l', b'a', b'y', 2];
    b.extend_from_slice(&[3, b'd', b':', b'a', 3, b'd', b':', b'a']);
    assert!(matches!(
        CommonRegister::decode(&b),
        Err(Error::Duplicate(_))
    ));
}
#[test]
fn channel_count_and_writer_body_bounds() {
    let ids: BTreeSet<_> = (0..MAX_CHANNELS)
        .map(|n| id(&format!("demo:{n:0250}")))
        .collect();
    assert!(CommonRegister {
        version: 1,
        phase: Phase::Play,
        channels: ids.clone()
    }
    .encode()
    .is_err());
    assert!(encode_dinnerbone(&ids).is_err());
    let bytes = b"d:a\0".repeat(MAX_CHANNELS + 1);
    assert!(decode_dinnerbone(&bytes).is_err());
}
#[test]
fn baseline_is_exact_not_wildcard() {
    assert!(check_baseline("1.21.1", 767, "21.1.248").is_ok());
    for tuple in [
        ("1.21.1", 767, "21.1.249"),
        ("1.21.1", 767, "21.1.247"),
        ("1.21.1", 768, "21.1.248"),
        ("1.21", 767, "21.1.248"),
    ] {
        assert!(check_baseline(tuple.0, tuple.1, tuple.2).is_err());
    }
}
#[test]
fn common_submachine_replies_without_join_claim() {
    let mut s = session();
    let out = s.receive("c:version", &[2, 2, 1]).unwrap();
    assert_eq!(out[0].channel, id("c:version"));
    assert_eq!(out[0].body, [1, 1]);
    assert_eq!(s.state(), State::AwaitRegister);
    let out = s
        .receive(
            "c:register",
            include_bytes!("../fixtures/common-register-demo.bin"),
        )
        .unwrap();
    assert!(CommonRegister::decode(&out[0].body)
        .unwrap()
        .channels
        .is_empty());
    assert!(s.peer_common().contains(&id("demo:ping")));
    assert_eq!(s.state(), State::CommonTasksOnly);
    assert!(s.finish_configuration().is_err());
    assert_eq!(s.state(), State::Failed);
}
#[test]
fn no_compatible_version_poisoned() {
    let mut s = session();
    assert!(s.receive("c:version", &[1, 2]).is_err());
    assert_eq!(s.state(), State::Failed);
    assert!(s.receive("c:version", &[1, 1]).is_err());
}
#[test]
fn register_before_version_and_repeated_version_refused() {
    let mut s = session();
    assert!(s
        .receive(
            "c:register",
            include_bytes!("../fixtures/common-register-empty-play.bin")
        )
        .is_err());
    let mut s = session();
    s.receive("c:version", &[1, 1]).unwrap();
    assert!(s.receive("c:version", &[1, 1]).is_err());
}
#[test]
fn unsupported_common_phase_and_version_refused() {
    for reg in [
        CommonRegister {
            version: 2,
            phase: Phase::Play,
            channels: BTreeSet::new(),
        },
        CommonRegister {
            version: 1,
            phase: Phase::Configuration,
            channels: BTreeSet::new(),
        },
    ] {
        let mut s = session();
        s.receive("c:version", &[1, 1]).unwrap();
        assert!(s.receive("c:register", &reg.encode().unwrap()).is_err());
    }
}
#[test]
fn modded_and_unknown_tasks_never_acknowledged() {
    for channel in [
        "neoforge:register",
        "neoforge:network",
        "neoforge:frozen_registry",
        "neoforge:frozen_registry_sync_completed",
        "neoforge:config_file",
        "neoforge:known_registry_data_maps",
        "neoforge:extensible_enum_data",
        "neoforge:feature_flags",
        "unknown:task",
    ] {
        let mut s = session();
        common(&mut s);
        assert!(
            matches!(s.receive(channel, &[]), Err(Error::UnsupportedTask(_))),
            "{channel}"
        );
        assert_eq!(s.state(), State::Failed);
    }
}
#[test]
fn dinnerbone_union_unregister_and_no_negotiation_side_effect() {
    let mut s = session();
    s.receive("minecraft:register", b"demo:ping\0").unwrap();
    s.receive("minecraft:register", b"demo:pong\0").unwrap();
    assert_eq!(s.peer_dinnerbone().len(), 2);
    s.receive("minecraft:unregister", b"demo:ping").unwrap();
    assert_eq!(s.peer_dinnerbone().len(), 1);
    assert_eq!(s.state(), State::AwaitVersion);
    assert!(s.peer_common().is_empty());
}
#[test]
fn extension_cannot_reserve_core_namespaces_or_duplicate() {
    let mut e = Extensions::new();
    for n in ["minecraft:x", "neoforge:x", "c:x", "horizon:x"] {
        assert!(e
            .register_optional_receiver(Phase::Play, id(n), 0, accept_empty)
            .is_err());
    }
    assert!(e
        .register_optional_receiver(Phase::Play, id("demo:ping"), MAX_BODY + 1, accept_empty)
        .is_err());
    e.register_optional_receiver(Phase::Play, id("demo:ping"), 0, accept_empty)
        .unwrap();
    assert!(e
        .register_optional_receiver(Phase::Play, id("demo:ping"), 0, accept_empty)
        .is_err());
}
#[test]
fn only_handler_backed_channels_advertised() {
    let mut e = Extensions::new();
    e.register_optional_receiver(Phase::Play, id("demo:ping"), 0, accept_empty)
        .unwrap();
    let mut s = Session::new("1.21.1", 767, "21.1.248", e).unwrap();
    assert!(!s
        .implemented_configuration_channels()
        .contains(&id("demo:ping")));
    assert!(!s
        .implemented_configuration_channels()
        .contains(&id("neoforge:frozen_registry")));
    s.receive("c:version", &[1, 1]).unwrap();
    let out = s
        .receive(
            "c:register",
            include_bytes!("../fixtures/common-register-empty-play.bin"),
        )
        .unwrap();
    assert_eq!(
        CommonRegister::decode(&out[0].body).unwrap().channels,
        BTreeSet::from([id("demo:ping")])
    );
}
#[test]
fn extension_dispatch_enforces_phase_size_and_failure() {
    let mut e = Extensions::new();
    e.register_optional_receiver(Phase::Configuration, id("demo:notice"), 1, accept_empty)
        .unwrap();
    assert!(e.dispatch(Phase::Play, &id("demo:notice"), &[]).is_err());
    assert!(e
        .dispatch(Phase::Configuration, &id("demo:notice"), &[0, 0])
        .is_err());
    let mut s = Session::new("1.21.1", 767, "21.1.248", e).unwrap();
    common(&mut s);
    assert!(s.receive("demo:notice", &[]).unwrap().is_empty());
    assert!(matches!(
        s.receive("demo:notice", &[0]),
        Err(Error::Handler(_))
    ));
    assert_eq!(s.state(), State::Failed);
}
#[test]
fn snapshot_golden() {
    let s = snapshot();
    assert_eq!(s.registry, id("demo:items"));
    assert_eq!(s.ids.get(&300), Some(&id("demo:b")));
    assert_eq!(s.aliases.get(&id("demo:old")), Some(&id("demo:a")));
    assert_eq!(
        s.encode().unwrap(),
        include_bytes!("../fixtures/frozen-items.bin")
    );
}
#[test]
fn maps_are_explicit_bidirectional_and_total() {
    let s = snapshot();
    let local = BTreeMap::from([(id("demo:a"), 40), (id("demo:b"), 7)]);
    let m = RegistryMap::build(&s, &local).unwrap();
    assert_eq!(m.to_local(300).unwrap(), 7);
    assert_eq!(m.to_server(40).unwrap(), 0);
    assert!(m.to_local(1).is_err());
    assert!(m.to_server(99).is_err());
    assert!(RegistryMap::build(&s, &BTreeMap::from([(id("demo:a"), 40)])).is_err());
    assert!(RegistryMap::build(
        &s,
        &BTreeMap::from([(id("demo:a"), 40), (id("demo:b"), 40)])
    )
    .is_err());
}
#[test]
fn snapshot_duplicate_names_negative_ids_and_alias_cycles() {
    let mut s = snapshot();
    s.ids.insert(301, id("demo:a"));
    assert!(s.validate().is_err());
    let mut s = snapshot();
    s.ids.insert(-1, id("demo:c"));
    assert!(s.validate().is_err());
    let mut s = snapshot();
    s.aliases.insert(id("demo:a"), id("demo:b"));
    assert!(s.validate().is_err());
    let mut s = snapshot();
    s.aliases.insert(id("demo:a1"), id("demo:a2"));
    s.aliases.insert(id("demo:a2"), id("demo:a1"));
    assert!(s.validate().is_err());
    let mut s = snapshot();
    s.aliases
        .insert(id("demo:missing_alias"), id("demo:missing"));
    assert!(s.validate().is_err());
}
#[test]
fn snapshot_duplicate_wire_id_refused() {
    let mut bytes = include_bytes!("../fixtures/frozen-items.bin").to_vec();
    // offset 20 is second ID's 0xac 0x02. Replace with 0 and remove continuation.
    assert_eq!(bytes[20], 0xac);
    bytes[20] = 0;
    bytes.remove(21);
    assert!(matches!(Snapshot::decode(&bytes), Err(Error::Duplicate(_))));
}
#[test]
fn bounded_deterministic_malformed_corpus_no_panics() {
    let mut seed = 0x12345678u32;
    for len in 0..256 {
        for _ in 0..16 {
            let bytes: Vec<_> = (0..len)
                .map(|_| {
                    seed ^= seed << 13;
                    seed ^= seed >> 17;
                    seed ^= seed << 5;
                    seed as u8
                })
                .collect();
            let _ = CommonVersion::decode(&bytes);
            let _ = CommonRegister::decode(&bytes);
            let _ = decode_dinnerbone(&bytes);
            let _ = Snapshot::decode(&bytes);
        }
    }
}
