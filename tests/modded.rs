//! Literal synthetic wire vectors, NOT Java-generated fixtures or captured packets.
use horizon_neoforge_compat::codec::{Id, Phase, MAX_BODY, MAX_STRING};
use horizon_neoforge_compat::modded::{
    reject_setup_failure, ConnectionProtocol as Protocol, Flow, ModdedQuery, ModdedSetup,
    NetworkChannel, QueryComponent, FAILURE_CHANNEL, MAX_FAILURE_REASONS, MAX_PROTOCOLS,
    MAX_TOTAL_COMPONENTS, QUERY_CHANNEL, SETUP_CHANNEL,
};
use horizon_neoforge_compat::Error;
use std::collections::BTreeMap;

fn id(s: &str) -> Id {
    Id::parse(s).unwrap()
}
fn hex(s: &str) -> Vec<u8> {
    s.split_whitespace()
        .map(|v| u8::from_str_radix(v, 16).unwrap())
        .collect()
}
fn varint(out: &mut Vec<u8>, n: i32) {
    let mut n = n as u32;
    loop {
        let byte = (n & 127) as u8;
        n >>= 7;
        out.push(if n == 0 { byte } else { byte | 128 });
        if n == 0 {
            break;
        }
    }
}
fn string(out: &mut Vec<u8>, s: &str) {
    varint(out, s.len() as i32);
    out.extend_from_slice(s.as_bytes());
}
fn query() -> ModdedQuery {
    let c = QueryComponent {
        id: id("x:a"),
        version: "1".into(),
        flow: None,
        optional: true,
    };
    ModdedQuery {
        phases: [(Protocol::Configuration, [(c.id.clone(), c)].into())].into(),
    }
}
fn setup() -> ModdedSetup {
    let c = NetworkChannel {
        id: id("x:a"),
        chosen_version: "1".into(),
    };
    ModdedSetup {
        phases: [(Protocol::Play, [(c.id.clone(), c)].into())].into(),
    }
}

#[test]
fn audited_enum_ordinals_and_phase_conversion() {
    let protocols = [
        Protocol::Handshaking,
        Protocol::Play,
        Protocol::Status,
        Protocol::Login,
        Protocol::Configuration,
    ];
    assert_eq!(MAX_PROTOCOLS, protocols.len());
    for (n, protocol) in protocols.into_iter().enumerate() {
        assert_eq!(protocol.ordinal(), n as i32);
        assert_eq!(Protocol::from_ordinal(n as i32).unwrap(), protocol);
        let q = ModdedQuery {
            phases: [(protocol, BTreeMap::new())].into(),
        };
        assert_eq!(q.encode().unwrap(), [1, n as u8, 0]);
        assert_eq!(ModdedQuery::decode(&q.encode().unwrap()).unwrap(), q);
        let s = ModdedSetup {
            phases: [(protocol, BTreeMap::new())].into(),
        };
        assert_eq!(ModdedSetup::decode(&s.encode().unwrap()).unwrap(), s);
    }
    for invalid in [-1, 5, i32::MAX, i32::MIN] {
        assert!(Protocol::from_ordinal(invalid).is_err());
        let mut b = vec![1];
        varint(&mut b, invalid);
        b.push(0);
        assert!(ModdedQuery::decode(&b).is_err());
        assert!(ModdedSetup::decode(&b).is_err());
    }
    assert_eq!(
        Protocol::from(Phase::Configuration),
        Protocol::Configuration
    );
    assert_eq!(Phase::try_from(Protocol::Play).unwrap(), Phase::Play);
    assert!(Phase::try_from(Protocol::Login).is_err());
    assert_eq!(Flow::from_ordinal(0).unwrap(), Flow::Serverbound);
    assert_eq!(Flow::from_ordinal(1).unwrap(), Flow::Clientbound);
    for n in [-1, 2, 127, i32::MAX] {
        assert!(Flow::from_ordinal(n).is_err());
    }
}
#[test]
fn payload_names_and_empty_maps() {
    assert_eq!(QUERY_CHANNEL, "neoforge:register");
    assert_eq!(SETUP_CHANNEL, "neoforge:network");
    assert_eq!(FAILURE_CHANNEL, "neoforge:modded_network_setup_failed");
    assert_eq!(ModdedQuery::default().encode().unwrap(), [0]);
    assert_eq!(ModdedSetup::default().encode().unwrap(), [0]);
    assert_eq!(ModdedQuery::decode(&[0]).unwrap(), ModdedQuery::default());
    assert_eq!(ModdedSetup::decode(&[0]).unwrap(), ModdedSetup::default());
    assert!(ModdedQuery::decode(&[]).is_err());
    assert!(ModdedSetup::decode(&[]).is_err());
}
#[test]
fn literal_query_and_setup_vectors() {
    let q = hex("01 04 01 03 78 3a 61 01 31 00 01");
    assert_eq!(query().encode().unwrap(), q);
    assert_eq!(ModdedQuery::decode(&q).unwrap(), query());
    let s = hex("01 01 01 03 78 3a 61 03 78 3a 61 01 31");
    assert_eq!(setup().encode().unwrap(), s);
    assert_eq!(ModdedSetup::decode(&s).unwrap(), setup());
    // Same component ID is legal in DIFFERENT phases; counts are per map on wire.
    let q = hex("02 01 01 03 78 3a 61 01 31 01 00 00 04 01 03 78 3a 61 00 01 01 01");
    let decoded = ModdedQuery::decode(&q).unwrap();
    assert_eq!(decoded.encode().unwrap(), q);
    assert_eq!(
        decoded.phases[&Protocol::Play][&id("x:a")].flow,
        Some(Flow::Serverbound)
    );
    assert_eq!(
        decoded.phases[&Protocol::Configuration][&id("x:a")].flow,
        Some(Flow::Clientbound)
    );
}
#[test]
fn every_query_and_setup_truncation_and_trailing_byte() {
    for b in [
        query().encode().unwrap(),
        hex("01 04 01 03 78 3a 61 01 31 01 01 00"),
    ] {
        for n in 0..b.len() {
            assert!(ModdedQuery::decode(&b[..n]).is_err(), "prefix {n}");
        }
        let mut b = b;
        b.push(0);
        assert_eq!(ModdedQuery::decode(&b), Err(Error::Trailing));
    }
    let mut b = setup().encode().unwrap();
    for n in 0..b.len() {
        assert!(ModdedSetup::decode(&b[..n]).is_err(), "prefix {n}");
    }
    b.push(0);
    assert_eq!(ModdedSetup::decode(&b), Err(Error::Trailing));
    for b in [&[0, 0][..], &[0, 255][..]] {
        assert_eq!(ModdedQuery::decode(b), Err(Error::Trailing));
        assert_eq!(ModdedSetup::decode(b), Err(Error::Trailing));
    }
}
#[test]
fn negative_overlarge_and_overflow_counts() {
    for bad in [
        hex("ff ff ff ff 0f"),
        vec![6],
        hex("80 80 80 80 10"),
        hex("80 80 80 80 80 00"),
    ] {
        assert!(ModdedQuery::decode(&bad).is_err());
        assert!(ModdedSetup::decode(&bad).is_err());
        let mut inner = vec![1, 4];
        inner.extend_from_slice(&bad);
        assert!(ModdedQuery::decode(&inner).is_err());
        assert!(ModdedSetup::decode(&inner).is_err());
    }
    let mut b = vec![1, 4];
    varint(&mut b, (MAX_TOTAL_COMPONENTS + 1) as i32);
    assert!(ModdedQuery::decode(&b).is_err());
    assert!(ModdedSetup::decode(&b).is_err());
    // Six valid inner entries require data, so the header alone must truncate.
    assert!(ModdedQuery::decode(&[1, 4, 6]).is_err());
}
#[test]
fn duplicate_phases_and_ids_never_overwrite() {
    for b in [hex("02 04 00 04 00"), hex("02 01 00 01 00")] {
        assert!(matches!(ModdedQuery::decode(&b), Err(Error::Duplicate(_))));
        assert!(matches!(ModdedSetup::decode(&b), Err(Error::Duplicate(_))));
    }
    let mut q = query().encode().unwrap();
    q[2] = 2;
    let record = q[3..].to_vec();
    q.extend_from_slice(&record);
    assert!(matches!(ModdedQuery::decode(&q), Err(Error::Duplicate(_))));
    // Differing versions are still duplicate IDs, not a legal alternate offer.
    *q.last_mut().unwrap() = 0;
    assert!(matches!(ModdedQuery::decode(&q), Err(Error::Duplicate(_))));
    let mut s = setup().encode().unwrap();
    s[2] = 2;
    let record = s[3..].to_vec();
    s.extend_from_slice(&record);
    assert!(matches!(ModdedSetup::decode(&s), Err(Error::Duplicate(_))));
    // Normalized ResourceLocations collide too.
    let q = hex("01 04 02 01 61 00 00 00 0b 6d 69 6e 65 63 72 61 66 74 3a 61 00 00 00");
    assert!(matches!(ModdedQuery::decode(&q), Err(Error::Duplicate(_))));
}
#[test]
fn bad_booleans_flows_strings_and_ids() {
    for n in [2, 127, 255] {
        let mut b = query().encode().unwrap();
        b[9] = n;
        assert!(ModdedQuery::decode(&b).is_err());
        b[9] = 0;
        b[10] = n;
        assert!(ModdedQuery::decode(&b).is_err());
    }
    for n in [-1, 2, 128, i32::MAX] {
        let mut b = hex("01 04 01 03 78 3a 61 01 31 01");
        varint(&mut b, n);
        b.push(0);
        assert!(ModdedQuery::decode(&b).is_err());
    }
    for b in [
        hex("01 04 01 03 58 3a 61 01 31 00 00"),
        hex("01 04 01 03 78 3a 61 01 ff 00 00"),
        hex("01 04 01 03 78 3a 61 ff ff ff ff 0f"),
    ] {
        assert!(ModdedQuery::decode(&b).is_err());
    }
    let mut b = setup().encode().unwrap();
    b[12] = 255;
    assert!(ModdedSetup::decode(&b).is_err());
    b = hex("01 01 01 03 78 3a 61 03 78 3a 61 ff ff ff ff 0f");
    assert!(ModdedSetup::decode(&b).is_err());
}
#[test]
fn required_empty_version_and_unicode_version_are_wire_values() {
    for version in ["", "1", "é\0🐾"] {
        let mut q = query();
        q.phases
            .get_mut(&Protocol::Configuration)
            .unwrap()
            .get_mut(&id("x:a"))
            .unwrap()
            .version = version.into();
        assert_eq!(ModdedQuery::decode(&q.encode().unwrap()).unwrap(), q);
        let mut s = setup();
        s.phases
            .get_mut(&Protocol::Play)
            .unwrap()
            .get_mut(&id("x:a"))
            .unwrap()
            .chosen_version = version.into();
        assert_eq!(ModdedSetup::decode(&s.encode().unwrap()).unwrap(), s);
    }
}
#[test]
fn mismatched_redundant_ids_rejected_both_directions() {
    let mut b = setup().encode().unwrap();
    b[10] = b'b';
    assert!(matches!(ModdedSetup::decode(&b), Err(Error::Invalid(_))));
    let mut s = setup();
    s.phases
        .get_mut(&Protocol::Play)
        .unwrap()
        .get_mut(&id("x:a"))
        .unwrap()
        .id = id("x:b");
    assert!(s.encode().is_err());
    let mut q = query();
    q.phases
        .get_mut(&Protocol::Configuration)
        .unwrap()
        .get_mut(&id("x:a"))
        .unwrap()
        .id = id("x:b");
    assert!(q.encode().is_err());
}
fn many_query(n: usize) -> ModdedQuery {
    let components = (0..n)
        .map(|n| {
            let id = id(&format!("x:{n}"));
            (
                id.clone(),
                QueryComponent {
                    id,
                    version: String::new(),
                    flow: None,
                    optional: false,
                },
            )
        })
        .collect();
    ModdedQuery {
        phases: [(Protocol::Play, components)].into(),
    }
}
fn many_setup(n: usize) -> ModdedSetup {
    let channels = (0..n)
        .map(|n| {
            let id = id(&format!("x:{n}"));
            (
                id.clone(),
                NetworkChannel {
                    id,
                    chosen_version: String::new(),
                },
            )
        })
        .collect();
    ModdedSetup {
        phases: [(Protocol::Play, channels)].into(),
    }
}
#[test]
fn entry_budget_boundary_and_cross_phase_total() {
    let mut q = many_query(MAX_TOTAL_COMPONENTS);
    assert_eq!(ModdedQuery::decode(&q.encode().unwrap()).unwrap(), q);
    assert!(many_query(MAX_TOTAL_COMPONENTS + 1).encode().is_err());
    let mut s = many_setup(MAX_TOTAL_COMPONENTS);
    assert_eq!(ModdedSetup::decode(&s.encode().unwrap()).unwrap(), s);
    assert!(many_setup(MAX_TOTAL_COMPONENTS + 1).encode().is_err());
    // Each phase is individually under budget, but total is not. Decoder checks
    // before trying to allocate/read a second phase's record.
    let mut qb = q.encode().unwrap();
    qb[0] = 2;
    qb.extend_from_slice(&[4, 1]);
    assert_eq!(
        ModdedQuery::decode(&qb),
        Err(Error::Limit("total modded components"))
    );
    let mut sb = s.encode().unwrap();
    sb[0] = 2;
    sb.extend_from_slice(&[4, 1]);
    assert_eq!(
        ModdedSetup::decode(&sb),
        Err(Error::Limit("total modded components"))
    );
    q.phases.insert(
        Protocol::Configuration,
        query().phases.remove(&Protocol::Configuration).unwrap(),
    );
    s.phases.insert(
        Protocol::Configuration,
        setup().phases.remove(&Protocol::Play).unwrap(),
    );
    assert!(q.encode().is_err());
    assert!(s.encode().is_err());
}
#[test]
fn string_and_body_budgets_apply_to_decoders_and_writers() {
    let mut q = query();
    let c = q
        .phases
        .get_mut(&Protocol::Configuration)
        .unwrap()
        .get_mut(&id("x:a"))
        .unwrap();
    c.version = "a".repeat(MAX_STRING);
    assert_eq!(ModdedQuery::decode(&q.encode().unwrap()).unwrap(), q);
    q.phases
        .get_mut(&Protocol::Configuration)
        .unwrap()
        .get_mut(&id("x:a"))
        .unwrap()
        .version
        .push('a');
    assert!(q.encode().is_err());
    let mut b = hex("01 04 01 03 78 3a 61");
    string(&mut b, &"a".repeat(MAX_STRING + 1));
    b.extend_from_slice(&[0, 0]);
    assert!(ModdedQuery::decode(&b).is_err());
    let mut s = setup();
    s.phases
        .get_mut(&Protocol::Play)
        .unwrap()
        .get_mut(&id("x:a"))
        .unwrap()
        .chosen_version = "a".repeat(MAX_STRING + 1);
    assert!(s.encode().is_err());
    let mut b = hex("01 01 01 03 78 3a 61 03 78 3a 61");
    string(&mut b, &"a".repeat(MAX_STRING + 1));
    assert!(ModdedSetup::decode(&b).is_err());
    let b = vec![0; MAX_BODY + 1];
    assert!(ModdedQuery::decode(&b).is_err());
    assert!(ModdedSetup::decode(&b).is_err());
    let mut q = many_query(33);
    for components in q.phases.values_mut() {
        for c in components.values_mut() {
            c.version = "a".repeat(MAX_STRING);
        }
    }
    assert_eq!(q.encode(), Err(Error::Limit("body")));
    let mut s = many_setup(33);
    for channels in s.phases.values_mut() {
        for c in channels.values_mut() {
            c.chosen_version = "a".repeat(MAX_STRING);
        }
    }
    assert_eq!(s.encode(), Err(Error::Limit("body")));
}
#[test]
fn noncanonical_short_varints_accepted_but_writer_canonical() {
    assert_eq!(
        ModdedQuery::decode(&hex("80 00"))
            .unwrap()
            .encode()
            .unwrap(),
        [0]
    );
    assert_eq!(
        ModdedSetup::decode(&hex("81 00 84 00 80 00"))
            .unwrap()
            .encode()
            .unwrap(),
        [1, 4, 0]
    );
}
#[test]
fn failure_is_explicitly_unsupported_terminal_not_a_fake_string_codec() {
    // Includes an empty reason map, NBT string tag and a fake UTF-8 reason.
    for b in [
        hex("00"),
        hex("01 03 78 3a 61 08 00 02 6e 6f"),
        hex("01 03 78 3a 61 02 6e 6f"),
    ] {
        assert!(matches!(
            reject_setup_failure(&b),
            Err(Error::UnsupportedTask(_))
        ));
    }
    assert_eq!(reject_setup_failure(&[]), Err(Error::Truncated));
    assert_eq!(reject_setup_failure(&[0, 0]), Err(Error::Trailing));
    assert!(reject_setup_failure(&hex("ff ff ff ff 0f")).is_err());
    assert!(reject_setup_failure(&hex("01 03 78")).is_err());
    assert!(reject_setup_failure(&vec![0; MAX_BODY + 1]).is_err());
    let mut b = Vec::new();
    varint(&mut b, (MAX_FAILURE_REASONS + 1) as i32);
    assert!(matches!(reject_setup_failure(&b), Err(Error::Limit(_))));
}
#[test]
fn deterministic_malformed_corpus_never_panics_or_escapes_terminal_failure() {
    let mut state = 0x4731_27ab_u32;
    for n in 0..4096 {
        let b: Vec<_> = (0..n % 193)
            .map(|_| {
                state ^= state << 13;
                state ^= state >> 17;
                state ^= state << 5;
                state as u8
            })
            .collect();
        if let Ok(q) = ModdedQuery::decode(&b) {
            assert_eq!(ModdedQuery::decode(&q.encode().unwrap()).unwrap(), q);
        }
        if let Ok(s) = ModdedSetup::decode(&b) {
            assert_eq!(ModdedSetup::decode(&s.encode().unwrap()).unwrap(), s);
        }
        assert!(reject_setup_failure(&b).is_err());
    }
}
