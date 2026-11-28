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

// Full-engine-only tests use actual Horizon dependencies. The standalone checker
// sets compat_isolated; no fake Horizon API is supplied in that test mode.
#[allow(unexpected_cfgs)]
mod actual_engine_transport {
    #[cfg(not(compat_isolated))]
    mod tests {
        use horizon_protocol::{
            PROTOCOL_VERSION,
            connection::Connection,
            packets::{Empty, configuration as cfg, handshake, login},
        };
        use crate::net::early_configuration::Profile;
        use std::sync::Arc;

        // Concrete split transport/actual decoder seam tests. These require the actual
        // companion dependency wired by check_full.py, not a substitute protocol model.
        fn split_test_configuration_pair() -> (Connection, Connection) {
            let (client, server) = horizon_protocol::connection::Pipe::pair();
            let mut client = Connection::local(client);
            let mut server = Connection::local(server);
            client.adapt(horizon_protocol::adapter::ServerVersion::V1_21_1);
            client.send(&handshake::Serverbound::Intention(handshake::Intention {
                protocol: PROTOCOL_VERSION,
                host: "localhost".into(),
                port: 25565,
                intent: handshake::LOGIN,
            }).encode()).unwrap();
            server.recv().unwrap();
            server.send(&[2]).unwrap();
            client.recv().unwrap();
            client.send(&login::Serverbound::LoginAcknowledged(Empty).encode()).unwrap();
            server.recv().unwrap();
            (client, server)
        }

        fn split_fragment(marker: u8, content: &[u8]) -> Vec<u8> {
            let mut w = horizon_protocol::buf::Writer::new();
            w.varint((content.len() + 1) as i32);
            w.bytes(&[marker]);
            w.bytes(content);
            w.data
        }

        fn reassemble_test_packet(packet: &[u8]) -> Vec<u8> {
            let mut transport = horizon_neoforge_compat::split::Reassembler::new();
            // Even the packet ID is incomplete until later fragments when multibyte.
            assert!(transport.receive(&split_fragment(1, &packet[..1]), 0).unwrap().is_none());
            transport.receive(&split_fragment(2, &packet[1..]), 1).unwrap().unwrap()
        }

        #[test]
        fn split_real_core_to_actual_decoder_and_loader() {
            use crate::neoforge_compat_bridge::CompatFactory;
            let (mut client, mut server) = split_test_configuration_pair();
            // Original protocol-767 packet: wire id + channel + query body. No frame.
            let packet = cfg::Clientbound::CustomPayload(cfg::CustomPayload {
                channel: "neoforge:register".into(),
                data: vec![0],
            }).encode();
            let original = reassemble_test_packet(&packet);
            // A queued socket packet MUST NOT be consumed while decoding the original.
            server.send(&[14, 0]).unwrap();
            let (id, body) = client.reader.decode_reassembled_configuration(original).unwrap();
            let cfg::Clientbound::CustomPayload(payload) = cfg::Clientbound::decode(id, &body).unwrap() else {
                panic!("expected reconstructed custom payload");
            };
            let profile = Profile::neoforge("1.21.1", 767, "21.1.248", [7; 32], vec![],
                Arc::new(CompatFactory { pack_sha256: [7; 32], registrations: Vec::new })).unwrap();
            let mut endpoint = profile.begin().unwrap();
            let replies = endpoint.receive(&payload.channel, &payload.data).unwrap();
            assert_eq!(replies.len(), 1);
            assert_eq!(replies[0].channel, "neoforge:register");
            assert_eq!(replies[0].body, [0]); // still no split/flag inventory offer
            assert_eq!(client.recv().unwrap(), (15, vec![0]));
            assert!(endpoint.finish().is_err());
        }

        #[test]
        fn split_nonidentity_id_is_adapted_exactly_once() {
            let (mut client, _server) = split_test_configuration_pair();
            // Wire 14 is SelectKnownPacks; pinned 15 is SelectKnownPacks. Interpreting
            // the original with the pinned decoder, or adapting twice, gets this wrong.
            let packet = reassemble_test_packet(&[14, 0]);
            let (id, body) = client.reader.decode_reassembled_configuration(packet).unwrap();
            assert_eq!(id, 15);
            assert_eq!(cfg::Clientbound::decode(id, &body).unwrap(),
                cfg::Clientbound::SelectKnownPacks(cfg::SelectKnownPacks { packs: vec![] }));
        }

        #[test]
        fn split_bad_originals_terminally_close_reader() {
            let recursive = cfg::Clientbound::CustomPayload(cfg::CustomPayload {
                channel: "neoforge:split".into(), data: vec![2, 1, 1],
            }).encode();
            for original in [
                vec![], vec![0x80], vec![0xff, 0xff, 0xff, 0xff, 0x0f],
                vec![0x80, 0x02, 0], // unknown multibyte wire ID, not a native ID
                vec![4], // truncated KeepAlive
                vec![3], // FinishConfiguration must not advance the adapter
                recursive,
                vec![0; horizon_neoforge_compat::split::MAX_PACKET_BYTES + 1],
            ] {
                let (mut client, mut server) = split_test_configuration_pair();
                server.send(&[14, 0]).unwrap();
                assert!(client.reader.decode_reassembled_configuration(original).is_err());
                assert!(client.reader.decode_reassembled_configuration(vec![14, 0]).is_err());
                assert!(client.reader.buffered().is_err());
                assert!(client.recv().is_err(), "queued socket input cannot revive rejected epoch");
            }
            // A fresh connection is independent of all the rejected epochs.
            let (mut client, _server) = split_test_configuration_pair();
            assert!(client.reader.decode_reassembled_configuration(vec![14, 0]).is_ok());
        }

        #[test]
        fn split_requires_exact_adapter_and_both_config_phases() {
            use horizon_protocol::adapter::ServerVersion;
            for version in [None, Some(ServerVersion::V1_21_4), Some(ServerVersion::V1_21_1)] {
                let (pipe, _peer) = horizon_protocol::connection::Pipe::pair();
                let mut client = Connection::local(pipe);
                if let Some(version) = version { client.adapt(version); }
                // No handshake/login/config transition yet, even for the correct pin.
                assert!(client.reader.decode_reassembled_configuration(vec![14, 0]).is_err());
            }
            let (mut client, mut server) = split_test_configuration_pair();
            // Ordinary socket FinishConfiguration has already advanced the read phase.
            server.send(&[3]).unwrap();
            client.recv().unwrap();
            assert!(client.reader.decode_reassembled_configuration(vec![14, 0]).is_err());
            let (mut client, _server) = split_test_configuration_pair();
            // A write-side transition alone also invalidates the seam.
            client.send(&cfg::Serverbound::FinishConfiguration(Empty).encode()).unwrap();
            assert!(client.reader.decode_reassembled_configuration(vec![14, 0]).is_err());
        }

        #[test]
        fn split_partial_timeout_never_reaches_decoder() {
            use horizon_neoforge_compat::split::{Reassembler, MAX_ASSEMBLY_MILLIS};
            let (mut client, mut server) = split_test_configuration_pair();
            let mut transport = Reassembler::new();
            assert!(transport.receive(&split_fragment(1, &[14]), 0).unwrap().is_none());
            assert!(transport.check_deadline(MAX_ASSEMBLY_MILLIS).is_err());
            assert!(transport.receive(&split_fragment(2, &[0]), MAX_ASSEMBLY_MILLIS).is_err());
            // No original escaped: real socket decoder remains independent. Future host
            // wiring must disconnect/reset on transport failure, not continue like this.
            server.send(&[14, 0]).unwrap();
            assert_eq!(client.recv().unwrap(), (15, vec![0]));
        }
    }
}

// Real TCP/framing tests for the prepared outbound wire boundary. They deliberately
// bypass Loader split routing: that production route is still unavailable.
#[allow(unexpected_cfgs)]
mod actual_engine_outbound_transport {
    #[cfg(not(compat_isolated))]
    mod tests {
        use horizon_neoforge_compat::{
            modded::{ConnectionProtocol, Flow},
            split::{OutboundPacket, OutboundSplitter, Reassembler},
        };
        use horizon_protocol::{
            PROTOCOL_VERSION,
            connection::Connection,
            packets::{Empty, configuration as cfg, handshake, login},
        };
        use std::{net::{TcpListener, TcpStream}, time::Duration};

        fn configured_tcp() -> (Connection, Connection) {
            let listener = TcpListener::bind("127.0.0.1:0").unwrap();
            let stream = TcpStream::connect(listener.local_addr().unwrap()).unwrap();
            let (peer, _) = listener.accept().unwrap();
            let mut client = Connection::from_stream(stream).unwrap();
            let mut server = Connection::from_stream(peer).unwrap();
            client.reader.set_read_timeout(Some(Duration::from_secs(3))).unwrap();
            server.reader.set_read_timeout(Some(Duration::from_secs(3))).unwrap();
            client.adapt(horizon_protocol::adapter::ServerVersion::V1_21_1);
            client.send(&handshake::Serverbound::Intention(handshake::Intention {
                protocol: PROTOCOL_VERSION,
                host: "localhost".into(),
                port: 25565,
                intent: handshake::LOGIN,
            }).encode()).unwrap();
            server.recv().unwrap();
            server.send(&[2]).unwrap();
            client.recv().unwrap();
            client.send(&login::Serverbound::LoginAcknowledged(Empty).encode()).unwrap();
            server.recv().unwrap();
            (client, server)
        }

        #[test]
        fn outbound_actual_preparation_split_tcp_compression_encryption_and_reassembly() {
            for compressed in [false, true] {
                for encrypted in [false, true] {
                    let (mut client, mut server) = configured_tcp();
                    if compressed {
                        client.set_compression(64);
                        server.set_compression(64);
                    }
                    if encrypted {
                        client.enable_encryption(&[19; 16]);
                        server.enable_encryption(&[19; 16]);
                    }
                    let packet = cfg::Serverbound::CustomPayload(cfg::CustomPayload {
                        channel: "fixture:opaque".into(),
                        data: (0..2_000).map(|i| (i % 251) as u8).collect(),
                    });
                    let pinned = packet.try_encode().unwrap();
                    let wire = client.writer.prepare_serverbound_configuration(&pinned).unwrap();
                    assert_eq!(wire, pinned); // actual protocol-767 custom payload id is 2
                    let splitter = OutboundSplitter::new(
                        Flow::Serverbound, ConnectionProtocol::Configuration, 128).unwrap();
                    let OutboundPacket::Split(bodies) = splitter.encode(&wire).unwrap() else {
                        panic!("fixture must exercise actual fragmentation");
                    };
                    let count = bodies.len();
                    assert!(count > 2);
                    for body in bodies {
                        // The registered split envelope uses the actual directional codec.
                        // At the exact pin custom_payload is wire id 2, unchanged from pinned.
                        // Frame prepared wire bytes DIRECTLY, never through send()/Session again.
                        let envelope = cfg::Serverbound::CustomPayload(cfg::CustomPayload {
                            channel: "neoforge:split".into(), data: body,
                        }).try_encode().unwrap();
                        assert_eq!(envelope[0], 2);
                        let contents = client.writer.contents(&envelope).unwrap();
                        client.writer.send_contents(&contents).unwrap();
                    }
                    let mut receiver = Reassembler::new();
                    let mut original = None;
                    for index in 0..count {
                        let (id, body) = server.recv().unwrap();
                        let cfg::Serverbound::CustomPayload(payload) =
                            cfg::Serverbound::decode(id, &body).unwrap() else {
                                panic!("expected the actual split envelope");
                            };
                        assert_eq!(payload.channel, "neoforge:split");
                        let result = receiver.receive(&payload.data, index as u64).unwrap();
                        if index + 1 == count { original = result; }
                        else { assert!(result.is_none()); }
                    }
                    let original = original.unwrap();
                    assert_eq!(original, wire);
                    assert_eq!(cfg::Serverbound::decode(2, &original[1..]).unwrap(), packet);
                    // A subsequent ordinary original still crosses the adapter exactly once.
                    let keepalive = cfg::Serverbound::KeepAlive(cfg::KeepAlive { id: 41 });
                    client.send(&keepalive.try_encode().unwrap()).unwrap();
                    let (id, body) = server.recv().unwrap();
                    assert_eq!(cfg::Serverbound::decode(id, &body).unwrap(), keepalive);
                }
            }
        }

        #[test]
        fn outbound_actual_loader_reply_unsplit_tcp_and_finish_still_refuses() {
            let (mut client, mut server) = configured_tcp();
            let profile = super::super::profile();
            let mut endpoint = profile.begin().unwrap();
            let replies = endpoint.receive("neoforge:register", &[0]).unwrap();
            assert_eq!(replies.len(), 1);
            assert_eq!(replies[0].body, [0]); // no invented split or flag authority
            let splitter = OutboundSplitter::new(
                Flow::Serverbound, ConnectionProtocol::Configuration, 128).unwrap();
            for reply in replies {
                let packet = cfg::Serverbound::CustomPayload(cfg::CustomPayload {
                    channel: reply.channel, data: reply.body,
                });
                let pinned = packet.try_encode().unwrap();
                let wire = client.writer.prepare_serverbound_configuration(&pinned).unwrap();
                let OutboundPacket::Unsplit(original) = splitter.encode(&wire).unwrap() else {
                    panic!("small actual loader reply must not acquire a split marker");
                };
                assert_eq!(original.as_ptr(), wire.as_ptr());
                let contents = client.writer.contents(original).unwrap();
                client.writer.send_contents(&contents).unwrap();
                let (id, body) = server.recv().unwrap();
                assert_eq!(cfg::Serverbound::decode(id, &body).unwrap(), packet);
            }
            assert!(endpoint.finish().is_err());
            assert!(endpoint.receive("neoforge:network", &[0]).is_err());
        }
    }
}
