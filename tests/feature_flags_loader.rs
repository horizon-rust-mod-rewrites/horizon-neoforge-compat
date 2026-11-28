use horizon_neoforge_compat::{
    codec::{self, Id, Phase},
    feature_flags::*,
    loader::Loader,
    modded::*,
    negotiation::{Direction, State},
};
use std::collections::{BTreeMap, BTreeSet};
fn id(s: &str) -> Id {
    Id::parse(s).unwrap()
}
fn loader() -> Loader {
    Loader::new_with_feature_flags("1.21.1", 767, "21.1.248", vec![], BTreeSet::new()).unwrap()
}
fn setup(names: &[&str]) -> Vec<u8> {
    ModdedSetup {
        phases: BTreeMap::from([(
            ConnectionProtocol::Configuration,
            names
                .iter()
                .map(|n| {
                    (
                        id(n),
                        NetworkChannel {
                            id: id(n),
                            chosen_version: "1".into(),
                        },
                    )
                })
                .collect(),
        )]),
    }
    .encode()
    .unwrap()
}
fn configure(l: &mut Loader) {
    l.receive(QUERY_CHANNEL, &[0]).unwrap();
    l.receive(SETUP_CHANNEL, &setup(&[DATA_CHANNEL, ACK_CHANNEL]))
        .unwrap();
}
#[test]
fn opt_in_offers_exact_real_handler_directions() {
    let mut l = loader();
    let out = l.receive(QUERY_CHANNEL, &[0]).unwrap();
    let q = ModdedQuery::decode(&out[0].body).unwrap();
    let ch = &q.phases[&ConnectionProtocol::Configuration];
    assert_eq!(ch.len(), 2);
    for (name, flow) in [
        (DATA_CHANNEL, Flow::Clientbound),
        (ACK_CHANNEL, Flow::Serverbound),
    ] {
        assert_eq!(ch[&id(name)].flow, Some(flow));
        assert_eq!(ch[&id(name)].version, "1");
        assert!(ch[&id(name)].optional);
    }
}
#[test]
fn no_default_inventory_or_advertisement() {
    let mut l = Loader::new("1.21.1", 767, "21.1.248", vec![]).unwrap();
    assert_eq!(l.receive(QUERY_CHANNEL, &[0]).unwrap()[0].body, [0]);
    l.receive(SETUP_CHANNEL, &[0]).unwrap();
    assert!(l.receive(DATA_CHANNEL, &[0]).is_err());
}
#[test]
fn only_selected_inbound_is_announced() {
    let mut l = loader();
    l.receive(QUERY_CHANNEL, &[0]).unwrap();
    let out = l
        .receive(SETUP_CHANNEL, &setup(&[DATA_CHANNEL, ACK_CHANNEL]))
        .unwrap();
    let announced = codec::decode_dinnerbone(&out[0].body).unwrap();
    assert!(announced.contains(&id(DATA_CHANNEL)));
    assert!(!announced.contains(&id(ACK_CHANNEL)));
}
#[test]
fn sends_exact_empty_ack_after_real_equality_but_no_finish() {
    let mut l = loader();
    configure(&mut l);
    assert!(!l.feature_flags_checked());
    let out = l.receive(DATA_CHANNEL, &[0]).unwrap();
    assert_eq!(out.len(), 1);
    assert_eq!(out[0].channel, id(ACK_CHANNEL));
    assert!(out[0].body.is_empty());
    assert!(l.feature_flags_checked());
    assert!(l.finish_configuration().is_err());
    assert!(!l.feature_flags_checked());
    assert!(l.accepted().is_empty());
}
#[test]
fn nonempty_host_inventory_must_match() {
    let flags = BTreeSet::from([id("demo:real")]);
    let body = FeatureFlagData {
        modded_flags: flags.clone(),
    }
    .encode()
    .unwrap();
    let mut l = Loader::new_with_feature_flags("1.21.1", 767, "21.1.248", vec![], flags).unwrap();
    configure(&mut l);
    assert!(l.receive(DATA_CHANNEL, &body).unwrap()[0].body.is_empty());
}
#[test]
fn mismatch_or_malformed_revokes_grants_and_never_retries() {
    for b in [
        FeatureFlagData {
            modded_flags: BTreeSet::from([id("demo:remote")]),
        }
        .encode()
        .unwrap(),
        vec![],
        vec![0, 0],
    ] {
        let mut l = loader();
        configure(&mut l);
        assert!(l.receive(DATA_CHANNEL, &b).is_err());
        assert!(!l.feature_flags_checked());
        assert!(l.accepted().is_empty());
        assert!(l.peer_dinnerbone().is_empty());
        assert!(l.receive(DATA_CHANNEL, &[0]).is_err());
        assert!(l.finish_configuration().is_err());
    }
}
#[test]
fn half_selected_pair_is_refused_atomically() {
    for names in [vec![DATA_CHANNEL], vec![ACK_CHANNEL]] {
        let mut l = loader();
        l.receive(QUERY_CHANNEL, &[0]).unwrap();
        assert!(l.receive(SETUP_CHANNEL, &setup(&names)).is_err());
        assert!(l.accepted().is_empty());
        assert_eq!(l.state(), State::Failed);
    }
}
#[test]
fn optional_absence_does_not_enable_dispatch() {
    let mut l = loader();
    l.receive(QUERY_CHANNEL, &[0]).unwrap();
    l.receive(SETUP_CHANNEL, &[0]).unwrap();
    assert!(!l.feature_flags_checked());
    assert!(l.receive(DATA_CHANNEL, &[0]).is_err());
}
#[test]
fn no_pre_setup_or_wrong_direction() {
    for ch in [DATA_CHANNEL, ACK_CHANNEL] {
        let mut l = loader();
        assert!(l.receive(ch, &[0]).is_err());
        let mut l = loader();
        configure(&mut l);
        if ch == ACK_CHANNEL {
            assert!(l.receive(ch, &[]).is_err());
        }
    }
}
#[test]
fn duplicate_and_close_revoke_check() {
    let mut l = loader();
    configure(&mut l);
    l.receive(DATA_CHANNEL, &[0]).unwrap();
    assert!(l.receive(DATA_CHANNEL, &[0]).is_err());
    assert!(!l.feature_flags_checked());
    let mut l = loader();
    configure(&mut l);
    l.receive(DATA_CHANNEL, &[0]).unwrap();
    l.close();
    assert!(!l.feature_flags_checked());
    assert!(l.receive(DATA_CHANNEL, &[0]).is_err());
    let mut fresh = loader();
    configure(&mut fresh);
    assert!(!fresh.feature_flags_checked());
}
#[test]
fn announced_channels_do_not_grant_task_pair() {
    let mut l = loader();
    l.receive(
        "minecraft:register",
        format!("{DATA_CHANNEL}\0{ACK_CHANNEL}\0").as_bytes(),
    )
    .unwrap();
    l.receive(QUERY_CHANNEL, &[0]).unwrap();
    l.receive(SETUP_CHANNEL, &[0]).unwrap();
    assert!(l.receive(DATA_CHANNEL, &[0]).is_err());
}
#[test]
fn caller_cannot_replace_builtin_handlers() {
    let registration = horizon_neoforge_compat::negotiation::Registration {
        phase: Phase::Configuration,
        id: id(DATA_CHANNEL),
        version: "1".into(),
        direction: Direction::Clientbound,
        optional: true,
        max_body: 0,
        receive: Some(validate_ack),
        validate_send: None,
    };
    assert!(Loader::new_with_feature_flags(
        "1.21.1",
        767,
        "21.1.248",
        vec![registration],
        BTreeSet::new()
    )
    .is_err());
}
#[test]
fn unsupported_baseline_still_refused() {
    assert!(
        Loader::new_with_feature_flags("1.21.1", 767, "21.1.249", vec![], BTreeSet::new()).is_err()
    );
}
