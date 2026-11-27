// SPDX-License-Identifier: MIT
//! Same actual-core/companion/endpoint tests in the isolated and full-engine runs.
//! Synthetic bodies; not a Java oracle, captured transcript or live join.
use super::CompatFactory;
use crate::net::early_configuration::*;
use std::sync::Arc;

fn profile() -> Profile {
    Profile::neoforge(
        "1.21.1",
        767,
        "21.1.248",
        [7; 32],
        vec![],
        Arc::new(CompatFactory {
            pack_sha256: [7; 32],
            registrations: Vec::new,
        }),
    )
    .unwrap()
}

#[test]
fn actual_wire_query_setup_common_then_finish_refusal() {
    let profile = profile();
    let mut endpoint = profile.begin().unwrap();
    let query = endpoint.receive("neoforge:register", &[0]).unwrap();
    assert_eq!(query.len(), 1);
    assert_eq!(query[0].body, [0]);
    let setup = endpoint.receive("neoforge:network", &[0]).unwrap();
    assert_eq!(setup[0].channel, "minecraft:register");
    assert_eq!(
        endpoint.receive("c:version", &[1, 1]).unwrap()[0].body,
        [1, 1]
    );
    assert!(endpoint.finish().is_err());
    assert!(endpoint.receive("c:version", &[1, 1]).is_err());
}

#[test]
fn unknown_required_task_and_reconnect_cleanup() {
    let profile = profile();
    let mut endpoint = profile.begin().unwrap();
    endpoint.receive("neoforge:register", &[0]).unwrap();
    endpoint.receive("neoforge:network", &[0]).unwrap();
    assert!(endpoint
        .receive("neoforge:extensible_enum_data", &[0])
        .is_err());
    drop(endpoint);
    let mut fresh = profile.begin().unwrap();
    assert!(fresh.receive("neoforge:network", &[0]).is_err());
}

#[test]
fn failure_payload_never_becomes_success() {
    let profile = profile();
    let mut endpoint = profile.begin().unwrap();
    assert!(endpoint
        .receive("neoforge:modded_network_setup_failed", &[0])
        .is_err());
}

#[test]
fn inventory_mismatch_never_constructs_loader_state() {
    let factory = CompatFactory {
        pack_sha256: [7; 32],
        registrations: Vec::new,
    };
    assert_eq!(
        factory.create([8; 32]).err(),
        Some(Failure("inventory_mismatch"))
    );
    let zero = CompatFactory {
        pack_sha256: [0; 32],
        registrations: Vec::new,
    };
    assert_eq!(
        zero.create([0; 32]).err(),
        Some(Failure("inventory_mismatch"))
    );
}
