use horizon_neoforge_compat::{
    codec::{Id, MAX_BODY},
    feature_flags::*,
};
use std::collections::BTreeSet;
fn flags(names: &[&str]) -> BTreeSet<Id> {
    names.iter().map(|n| Id::parse(n).unwrap()).collect()
}
fn body(names: &[&str]) -> Vec<u8> {
    FeatureFlagData {
        modded_flags: flags(names),
    }
    .encode()
    .unwrap()
}
#[test]
fn literal_anchor_and_empty() {
    assert_eq!(body(&[]), [0]);
    assert_eq!(body(&["a:b"]), [1, 3, b'a', b':', b'b']);
    assert_eq!(
        FeatureFlagData::decode(&[1, 3, b'a', b':', b'b'])
            .unwrap()
            .modded_flags,
        flags(&["a:b"])
    );
}
#[test]
fn wire_order_does_not_change_set() {
    let b = [2, 3, b'a', b':', b'z', 3, b'a', b':', b'b'];
    assert_eq!(
        FeatureFlagData::decode(&b).unwrap().modded_flags,
        flags(&["a:b", "a:z"])
    );
}
#[test]
fn empty_equality_is_explicit_and_ack_has_no_count() {
    let mut t = FeatureFlagTask::new(flags(&[])).unwrap();
    assert_eq!(t.receive(&[0]).unwrap(), Vec::<u8>::new());
    assert_eq!(t.state(), State::Checked);
    assert!(validate_ack(&[]).is_ok());
    assert!(validate_ack(&[0]).is_err());
}
#[test]
fn populated_exact_equality() {
    let mut t = FeatureFlagTask::new(flags(&["demo:a", "demo:b"])).unwrap();
    assert!(t.receive(&body(&["demo:b", "demo:a"])).unwrap().is_empty());
    assert_eq!(t.state(), State::Checked);
}
#[test]
fn both_missing_local_and_missing_remote_refused() {
    for (local, remote) in [
        (vec![], vec!["demo:a"]),
        (vec!["demo:a"], vec![]),
        (vec!["demo:a"], vec!["demo:b"]),
    ] {
        let mut t = FeatureFlagTask::new(flags(&local)).unwrap();
        assert!(t.receive(&body(&remote)).is_err());
        assert_eq!(t.state(), State::Failed);
        assert!(t.receive(&body(&local)).is_err());
    }
}
#[test]
fn duplicate_delivery_is_terminal() {
    let mut t = FeatureFlagTask::new(flags(&[])).unwrap();
    t.receive(&[0]).unwrap();
    assert!(t.receive(&[0]).is_err());
    assert_eq!(t.state(), State::Failed);
}
#[test]
fn duplicate_normalized_names_refused() {
    assert!(FeatureFlagData::decode(&[
        2, 1, b'x', 11, b'm', b'i', b'n', b'e', b'c', b'r', b'a', b'f', b't', b':', b'x'
    ])
    .is_err());
}
#[test]
fn every_prefix_and_trailing_refused() {
    let b = body(&["a:b", "a:c"]);
    for n in 0..b.len() {
        assert!(FeatureFlagData::decode(&b[..n]).is_err());
    }
    let mut b = b;
    b.push(0);
    assert!(FeatureFlagData::decode(&b).is_err());
}
#[test]
fn malformed_input_poisons_task() {
    for b in [
        vec![],
        vec![255, 255, 255, 255, 15],
        vec![128, 128, 128, 128, 16],
        vec![1, 1, 255],
        vec![1, 3, b'A', b':', b'b'],
        vec![0, 0],
        vec![0; MAX_BODY + 1],
    ] {
        let mut t = FeatureFlagTask::new(flags(&[])).unwrap();
        assert!(t.receive(&b).is_err());
        assert_eq!(t.state(), State::Failed);
        assert!(t.receive(&[0]).is_err());
    }
}
#[test]
fn count_budget_and_constructor_bound() {
    let good = (0..MAX_FLAGS)
        .map(|i| Id::parse(&format!("a:{i}")).unwrap())
        .collect();
    let good = FeatureFlagData { modded_flags: good };
    assert!(FeatureFlagData::decode(&good.encode().unwrap()).is_ok());
    let bad = (0..=MAX_FLAGS)
        .map(|i| Id::parse(&format!("a:{i}")).unwrap())
        .collect();
    assert!(FeatureFlagTask::new(bad).is_err());
}
#[test]
fn constructor_checks_aggregate_bytes() {
    let names = (0..200)
        .map(|i| Id::parse(&format!("a:{i}{}", "z".repeat(200))).unwrap())
        .collect();
    assert!(FeatureFlagTask::new(names).is_err());
}
#[test]
fn close_and_fresh_epoch() {
    let mut t = FeatureFlagTask::new(flags(&[])).unwrap();
    t.close();
    assert_eq!(t.state(), State::Closed);
    assert!(t.receive(&[0]).is_err());
    let mut fresh = FeatureFlagTask::new(flags(&[])).unwrap();
    assert!(fresh.receive(&[0]).is_ok());
}
