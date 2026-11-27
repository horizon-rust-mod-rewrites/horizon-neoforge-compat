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

#[test]
fn unavailable_host_inventory_never_offers_feature_or_split_support() {
    use horizon_neoforge_compat::modded::ModdedQuery;
    let profile = profile();
    let mut endpoint = profile.begin().unwrap();
    let replies = endpoint.receive("neoforge:register", &[0]).unwrap();
    let query = ModdedQuery::decode(&replies[0].body).unwrap();
    // No host implementation snapshot exists. Neither enabled peer flags nor
    // the checked pack fingerprint establish an implemented inventory.
    assert!(query.phases.is_empty());
    assert_eq!(replies[0].body, [0]);
}

#[test]
fn peer_announcements_cannot_enable_unimplemented_required_tasks() {
    for channel in [
        "neoforge:feature_flags",
        "neoforge:feature_flags_ack",
        "neoforge:split",
        "neoforge:extensible_enum_data",
        "neoforge:registry_data_map_sync",
    ] {
        let profile = profile();
        let mut endpoint = profile.begin().unwrap();
        endpoint
            .receive("minecraft:register", channel.as_bytes())
            .unwrap();
        assert_eq!(endpoint.receive("neoforge:register", &[0]).unwrap()[0].body, [0]);
        endpoint.receive("neoforge:network", &[0]).unwrap();
        assert!(endpoint.receive(channel, &[0]).is_err(), "{channel}");
        assert!(endpoint.receive("c:version", &[1, 1]).is_err());
        assert!(endpoint.finish().is_err());
    }
}

#[test]
fn peer_setup_cannot_select_a_feature_inventory_the_host_never_attested() {
    use horizon_neoforge_compat::{
        codec::Id,
        modded::{ConnectionProtocol, ModdedSetup, NetworkChannel},
    };
    use std::collections::BTreeMap;
    let channels = ["neoforge:feature_flags", "neoforge:feature_flags_ack"]
        .into_iter()
        .map(|name| {
            let id = Id::parse(name).unwrap();
            (id.clone(), NetworkChannel { id, chosen_version: "1".into() })
        })
        .collect();
    let body = ModdedSetup {
        phases: BTreeMap::from([(ConnectionProtocol::Configuration, channels)]),
    }.encode().unwrap();
    let profile = profile();
    let mut endpoint = profile.begin().unwrap();
    endpoint.receive("neoforge:register", &[0]).unwrap();
    assert!(endpoint.receive("neoforge:network", &body).is_err());
    assert!(endpoint.receive("neoforge:network", &[0]).is_err());
    assert!(endpoint.finish().is_err());
}

#[test]
fn actual_bridge_reset_is_terminal_and_new_factory_epoch_is_independent() {
    let factory = CompatFactory {
        pack_sha256: [7; 32],
        registrations: Vec::new,
    };
    let mut first = factory.create([7; 32]).unwrap();
    first.receive("neoforge:register", &[0]).unwrap();
    first.reset();
    first.reset(); // cleanup must remain safe on repeated exit paths
    assert!(first.receive("neoforge:network", &[0]).is_err());
    assert!(first.finish().is_err());
    let mut fresh = factory.create([7; 32]).unwrap();
    assert_eq!(fresh.receive("neoforge:register", &[0]).unwrap()[0].body, [0]);
    fresh.receive("neoforge:network", &[0]).unwrap();
    assert!(fresh.finish().is_err());
}

#[test]
fn inventory_is_checked_before_registration_provider_runs() {
    fn must_not_run() -> Vec<horizon_neoforge_compat::negotiation::Registration> {
        panic!("registration provider must not run for an invalid inventory")
    }
    let factory = CompatFactory {
        pack_sha256: [7; 32],
        registrations: must_not_run,
    };
    assert_eq!(factory.create([8; 32]).err(), Some(Failure("inventory_mismatch")));
    let zero = CompatFactory { pack_sha256: [0; 32], registrations: must_not_run };
    assert_eq!(zero.create([0; 32]).err(), Some(Failure("inventory_mismatch")));
}
