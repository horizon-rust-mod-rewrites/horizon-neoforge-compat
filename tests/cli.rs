use std::process::Command;
#[test]
fn demo_explicitly_does_not_join() {
    let out = Command::new(env!("CARGO_BIN_EXE_compat-inspect"))
        .arg("common-demo")
        .output()
        .unwrap();
    assert!(out.status.success());
    assert!(String::from_utf8(out.stdout)
        .unwrap()
        .contains("NOT joined"));
}
#[test]
fn fixture_inspection() {
    let out = Command::new(env!("CARGO_BIN_EXE_compat-inspect"))
        .args([
            "frozen",
            concat!(env!("CARGO_MANIFEST_DIR"), "/fixtures/frozen-items.bin"),
        ])
        .output()
        .unwrap();
    assert!(out.status.success());
    assert!(String::from_utf8(out.stdout)
        .unwrap()
        .contains("demo:items"));
}
#[test]
fn bad_usage_has_nonzero_exit() {
    let out = Command::new(env!("CARGO_BIN_EXE_compat-inspect"))
        .output()
        .unwrap();
    assert_eq!(out.status.code(), Some(2));
}
