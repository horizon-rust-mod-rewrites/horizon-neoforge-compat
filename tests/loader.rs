use horizon_neoforge_compat::{
    codec::{self, Id, Phase},
    loader::Loader,
    modded::*,
    negotiation::{Direction, Registration, State},
    Error, Result,
};
use std::collections::BTreeMap;
fn id(s: &str) -> Id {
    Id::parse(s).unwrap()
}
fn loader() -> Loader {
    Loader::new("1.21.1", 767, "21.1.248", vec![]).unwrap()
}
fn empty(b: &[u8]) -> Result<()> {
    if b.is_empty() {
        Ok(())
    } else {
        Err(Error::Invalid("empty"))
    }
}
fn configure(l: &mut Loader) {
    assert_eq!(l.receive(QUERY_CHANNEL, &[0]).unwrap()[0].body, [0]);
    let out = l.receive(SETUP_CHANNEL, &[0]).unwrap();
    assert_eq!(out[0].channel, id("minecraft:register"));
    let ids = codec::decode_dinnerbone(&out[0].body).unwrap();
    assert!(ids.contains(&id(QUERY_CHANNEL)));
    assert!(!ids.iter().any(|id| id.as_str().starts_with("horizon:")));
}
#[test]
fn empty_setup_can_negotiate_but_never_join() {
    let mut l = loader();
    configure(&mut l);
    assert_eq!(l.state(), State::Configuring);
    assert_eq!(l.receive("c:version", &[1, 1]).unwrap()[0].body, [1, 1]);
    l.receive("c:register", &[1, 4, b'p', b'l', b'a', b'y', 0])
        .unwrap();
    assert!(l.finish_configuration().is_err());
    assert_eq!(l.state(), State::Failed);
}
#[test]
fn no_setup_without_query() {
    let mut l = loader();
    assert!(l.receive(SETUP_CHANNEL, &[0]).is_err());
    assert!(l.receive(QUERY_CHANNEL, &[0]).is_err());
}
#[test]
fn nonempty_server_query_refused() {
    let mut l = loader();
    assert!(l.receive(QUERY_CHANNEL, &[1, 4, 0]).is_err());
    assert_eq!(l.state(), State::Failed);
}
#[test]
fn dinnerbone_cannot_grant_authority() {
    let mut l = loader();
    l.receive("minecraft:register", b"demo:task\0").unwrap();
    configure(&mut l);
    assert!(l.receive("demo:task", &[]).is_err());
    assert!(l.peer_dinnerbone().is_empty());
}
#[test]
fn setup_failure_and_unknown_tasks_terminal() {
    for channel in [
        FAILURE_CHANNEL,
        "neoforge:extensible_enum_data",
        "demo:unknown",
    ] {
        let mut l = loader();
        configure(&mut l);
        assert!(l.receive(channel, &[0]).is_err());
        assert_eq!(l.state(), State::Failed);
    }
}
#[test]
fn wire_offer_direction_version_and_callback_are_real() {
    let registration = Registration {
        phase: Phase::Configuration,
        id: id("demo:task"),
        version: "7".into(),
        direction: Direction::Clientbound,
        optional: false,
        max_body: 0,
        receive: Some(empty),
        validate_send: None,
    };
    let mut l = Loader::new("1.21.1", 767, "21.1.248", vec![registration]).unwrap();
    let out = l.receive(QUERY_CHANNEL, &[0]).unwrap();
    let query = ModdedQuery::decode(&out[0].body).unwrap();
    let component = &query.phases[&ConnectionProtocol::Configuration][&id("demo:task")];
    assert_eq!(component.flow, Some(Flow::Clientbound));
    assert_eq!(component.version, "7");
    assert!(!component.optional);
    let setup = ModdedSetup {
        phases: BTreeMap::from([(
            ConnectionProtocol::Configuration,
            BTreeMap::from([(
                id("demo:task"),
                NetworkChannel {
                    id: id("demo:task"),
                    chosen_version: "7".into(),
                },
            )]),
        )]),
    };
    l.receive(SETUP_CHANNEL, &setup.encode().unwrap()).unwrap();
    assert!(l.receive("demo:task", &[]).unwrap().is_empty());
    assert!(l.receive("demo:task", &[0]).is_err());
    assert!(l.accepted().is_empty());
}
#[test]
fn invalid_phase_setup_is_not_silently_ignored() {
    let mut l = loader();
    l.receive(QUERY_CHANNEL, &[0]).unwrap();
    assert!(l.receive(SETUP_CHANNEL, &[1, 2, 0]).is_err());
}
#[test]
fn repeated_and_malformed_messages_poison() {
    for bytes in [&[][..], &[0, 0], &[1, 4]] {
        let mut l = loader();
        assert!(l.receive(QUERY_CHANNEL, bytes).is_err());
        assert_eq!(l.state(), State::Failed);
    }
    let mut l = loader();
    l.receive(QUERY_CHANNEL, &[0]).unwrap();
    assert!(l.receive(QUERY_CHANNEL, &[0]).is_err());
}
#[test]
fn epoch_budget_limits_repeated_announcements() {
    let mut l = loader();
    for _ in 0..8192 {
        l.receive("minecraft:register", &[]).unwrap();
    }
    assert!(l.receive("minecraft:register", &[]).is_err());
}
#[test]
fn baseline_and_close_are_strict() {
    assert!(Loader::new("1.21.1", 767, "21.1.249", vec![]).is_err());
    let mut l = loader();
    configure(&mut l);
    l.close();
    assert!(l.receive(QUERY_CHANNEL, &[0]).is_err());
}
