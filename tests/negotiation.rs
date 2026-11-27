use horizon_neoforge_compat::{
    codec::{Id, Phase},
    negotiation::*,
    Error, Result,
};
fn id(s: &str) -> Id {
    Id::parse(s).unwrap()
}
fn empty(body: &[u8]) -> Result<()> {
    if body.is_empty() {
        Ok(())
    } else {
        Err(Error::Invalid("empty required"))
    }
}
fn reg(optional: bool, direction: Direction) -> Registration {
    Registration {
        phase: Phase::Configuration,
        id: id("test:task"),
        version: "1".into(),
        direction,
        optional,
        max_body: 0,
        receive: if direction != Direction::Serverbound {
            Some(empty)
        } else {
            None
        },
        validate_send: if direction != Direction::Clientbound {
            Some(empty)
        } else {
            None
        },
    }
}
fn selected() -> Selection {
    Selection {
        phase: Phase::Configuration,
        key: id("test:task"),
        id: id("test:task"),
        version: "1".into(),
    }
}
fn started(optional: bool) -> Negotiation {
    let mut n = Negotiation::new(vec![reg(optional, Direction::Clientbound)]).unwrap();
    assert_eq!(n.on_query().unwrap().len(), 1);
    n
}
#[test]
fn missing_callback_not_advertised() {
    let mut r = reg(true, Direction::Bidirectional);
    r.validate_send = None;
    assert!(Negotiation::new(vec![r]).is_err());
}
#[test]
fn server_query_empty_is_not_server_offer() {
    let mut n = started(false);
    assert_eq!(n.state(), State::AwaitSetup);
    n.on_setup(vec![selected()]).unwrap();
    assert_eq!(n.state(), State::Configuring);
    n.receive_configuration(&id("test:task"), &[]).unwrap();
    assert!(n.finish_configuration().is_err());
    assert!(n.accepted().is_empty());
}
#[test]
fn optional_absence_is_not_authority() {
    let mut n = started(true);
    n.on_setup(vec![]).unwrap();
    assert!(n.receive_configuration(&id("test:task"), &[]).is_err());
    assert_eq!(n.state(), State::Failed);
}
#[test]
fn required_absence_is_terminal() {
    let mut n = started(false);
    assert!(n.on_setup(vec![]).is_err());
    assert_eq!(n.state(), State::Failed);
    assert!(n.on_setup(vec![selected()]).is_err());
}
#[test]
fn mismatches_and_duplicates_do_not_partially_install() {
    for mode in 0..5 {
        let mut n = started(true);
        let mut bad = selected();
        match mode {
            0 => bad.version = "2".into(),
            1 => bad.key = id("test:other"),
            2 => {
                bad.key = id("test:other");
                bad.id = id("test:other");
            }
            3 => bad.phase = Phase::Play,
            _ => {}
        }
        let list = if mode == 4 {
            vec![selected(), selected()]
        } else {
            vec![bad]
        };
        assert!(n.on_setup(list).is_err());
        assert!(n.accepted().is_empty());
    }
}
#[test]
fn phase_direction_size_and_handler_failure_poison_epoch() {
    for mode in 0..3 {
        let mut n = started(false);
        n.on_setup(vec![selected()]).unwrap();
        let result = match mode {
            0 => n.validate_configuration_send(&id("test:task"), &[]),
            1 => n.receive_configuration(&id("test:task"), &[0]),
            _ => n.receive_configuration(&id("test:other"), &[]),
        };
        assert!(result.is_err());
        assert_eq!(n.state(), State::Failed);
        assert!(n.accepted().is_empty());
    }
}
#[test]
fn close_revokes_capabilities() {
    let mut n = started(false);
    n.on_setup(vec![selected()]).unwrap();
    n.close();
    assert_eq!(n.state(), State::Closed);
    assert!(n.accepted().is_empty());
    assert!(n.on_query().is_err());
}
#[test]
fn ordering_is_terminal() {
    let mut n = Negotiation::new(vec![]).unwrap();
    assert!(n.on_setup(vec![]).is_err());
    let mut n = started(false);
    assert!(n.on_query().is_err());
    assert!(n.on_setup(vec![selected()]).is_err());
}
#[test]
fn optional_does_not_relax_matching_flow_or_version() {
    let r = reg(true, Direction::Clientbound);
    let mut n = Negotiation::new(vec![r]).unwrap();
    let a = n.on_query().unwrap().remove(0);
    assert!(compatible(&a, &a));
    let mut b = a.clone();
    b.direction = Direction::Bidirectional;
    assert!(!compatible(&a, &b));
    b = a.clone();
    b.version = "2".into();
    assert!(!compatible(&a, &b));
}
#[test]
fn bootstrap_channel_cannot_be_replaced() {
    let mut r = reg(true, Direction::Clientbound);
    r.id = id("neoforge:register");
    assert!(Negotiation::new(vec![r]).is_err());
}
