use horizon_neoforge_compat::split::*;
fn frame(marker: u8, content: &[u8]) -> Vec<u8> {
    let mut n = content.len() + 1;
    let mut out = vec![];
    loop {
        let b = (n & 127) as u8;
        n >>= 7;
        out.push(b | if n > 0 { 128 } else { 0 });
        if n == 0 {
            break;
        }
    }
    out.push(marker);
    out.extend_from_slice(content);
    out
}
#[test]
fn actual_array_prefix_not_raw_marker() {
    let p = Part::decode(&[3, 1, 0x80, 1]).unwrap();
    assert_eq!(p.marker, Marker::First);
    assert_eq!(p.content, [0x80, 1]);
    assert!(Part::decode(&[1, 0x80, 1]).is_err());
}
#[test]
fn reconstructs_whole_packet_including_multibyte_id() {
    let mut r = Reassembler::new();
    assert_eq!(r.receive(&frame(1, &[0x80]), 0).unwrap(), None);
    assert_eq!(r.receive(&frame(0, &[1, 10]), 1).unwrap(), None);
    assert_eq!(
        r.receive(&frame(2, &[20]), 2).unwrap(),
        Some(vec![0x80, 1, 10, 20])
    );
    assert!(!r.pending());
}
#[test]
fn consecutive_packets_do_not_share_bytes() {
    let mut r = Reassembler::new();
    for i in 0..10 {
        r.receive(&frame(1, &[i]), i as u64).unwrap();
        assert_eq!(
            r.receive(&frame(2, &[i]), i as u64).unwrap(),
            Some(vec![i, i])
        );
    }
}
#[test]
fn orphan_middle_or_last_is_terminal() {
    for m in [0, 2] {
        let mut r = Reassembler::new();
        assert!(r.receive(&frame(m, &[1]), 0).is_err());
        assert!(r.failed());
        assert!(!r.pending());
        assert!(r.receive(&frame(1, &[1]), 1).is_err());
    }
}
#[test]
fn new_first_cannot_silently_drop_previous_packet() {
    let mut r = Reassembler::new();
    r.receive(&frame(1, &[1]), 0).unwrap();
    assert!(r.receive(&frame(1, &[2]), 1).is_err());
    assert!(!r.pending());
}
#[test]
fn flags_are_not_combinable() {
    for m in [3, 4, 255] {
        assert!(Part::decode(&frame(m, &[1])).is_err());
    }
}
#[test]
fn empty_fragments_rejected() {
    for b in [vec![], vec![0], vec![1, 1], vec![1, 2]] {
        assert!(Part::decode(&b).is_err());
    }
}
#[test]
fn malformed_varints_and_trailing_bytes() {
    for b in [
        vec![255, 255, 255, 255, 15],
        vec![128, 128, 128, 128, 16],
        vec![128, 128, 128, 128, 128],
        vec![2, 1, 7, 9],
    ] {
        assert!(Part::decode(&b).is_err());
    }
}
#[test]
fn every_truncation_rejected() {
    let b = frame(1, &[42; 300]);
    for i in 0..b.len() {
        assert!(Part::decode(&b[..i]).is_err());
    }
}
#[test]
fn per_part_budget() {
    assert!(Part::decode(&frame(1, &vec![0; MAX_PART_BYTES])).is_ok());
    assert!(Part::decode(&frame(1, &vec![0; MAX_PART_BYTES + 1])).is_err());
}
#[test]
fn aggregate_byte_budget_and_no_partial_output() {
    let mut r = Reassembler::new();
    let b = frame(1, &vec![0; MAX_PART_BYTES]);
    r.receive(&b, 0).unwrap();
    let b = frame(0, &vec![0; MAX_PART_BYTES]);
    for _ in 1..8 {
        r.receive(&b, 0).unwrap();
    }
    assert!(r.receive(&frame(2, &[1]), 0).is_err());
    assert!(r.failed());
    assert!(!r.pending());
}
#[test]
fn aggregate_part_budget() {
    let mut r = Reassembler::new();
    r.receive(&frame(1, &[1]), 0).unwrap();
    for _ in 1..MAX_PARTS {
        r.receive(&frame(0, &[1]), 0).unwrap();
    }
    assert!(r.receive(&frame(2, &[1]), 0).is_err());
}
#[test]
fn timeout_applies_without_incoming_packet() {
    let mut r = Reassembler::new();
    r.receive(&frame(1, &[1]), 10).unwrap();
    r.check_deadline(10 + MAX_ASSEMBLY_MILLIS - 1).unwrap();
    assert!(r.check_deadline(10 + MAX_ASSEMBLY_MILLIS).is_err());
    assert!(r.failed());
    assert!(!r.pending());
}
#[test]
fn more_parts_do_not_extend_deadline() {
    let mut r = Reassembler::new();
    r.receive(&frame(1, &[1]), 0).unwrap();
    r.receive(&frame(0, &[2]), MAX_ASSEMBLY_MILLIS - 1).unwrap();
    assert!(r.receive(&frame(2, &[3]), MAX_ASSEMBLY_MILLIS).is_err());
}
#[test]
fn clock_backwards_fails_closed() {
    let mut r = Reassembler::new();
    r.check_deadline(100).unwrap();
    assert!(r.check_deadline(99).is_err());
}
#[test]
fn elapsed_clock_near_u64_limit_does_not_overflow() {
    let mut r = Reassembler::new();
    r.receive(&frame(1, &[1]), u64::MAX - 1).unwrap();
    assert!(r.receive(&frame(2, &[2]), u64::MAX).unwrap().is_some());
}
#[test]
fn close_is_terminal_and_fresh_instance_required() {
    let mut r = Reassembler::new();
    r.receive(&frame(1, &[1]), 0).unwrap();
    r.close();
    assert!(!r.pending());
    assert!(r.receive(&frame(2, &[2]), 1).is_err());
    let mut r = Reassembler::new();
    r.receive(&frame(1, &[3]), 2).unwrap();
    assert_eq!(r.receive(&frame(2, &[4]), 3).unwrap(), Some(vec![3, 4]));
}
#[test]
fn decode_error_clears_pending_assembly() {
    let mut r = Reassembler::new();
    r.receive(&frame(1, &[1]), 0).unwrap();
    assert!(r.receive(&[2, 1], 1).is_err());
    assert!(!r.pending());
    assert!(r.failed());
}

use horizon_neoforge_compat::modded::{ConnectionProtocol, Flow};
use horizon_neoforge_compat::Error;

fn outbound(flow: Flow, threshold: usize) -> OutboundSplitter {
    OutboundSplitter::new(flow, ConnectionProtocol::Configuration, threshold).unwrap()
}
fn split_bodies(result: OutboundPacket<'_>) -> Vec<Vec<u8>> {
    match result {
        OutboundPacket::Split(bodies) => bodies,
        OutboundPacket::Unsplit(_) => panic!("expected actual split payload bodies"),
    }
}
fn roundtrip_bodies(bodies: &[Vec<u8>], original: &[u8]) {
    assert!((2..=256).contains(&bodies.len()));
    let mut receiver = Reassembler::new();
    for (i, body) in bodies.iter().enumerate() {
        let output = receiver.receive(body, i as u64).unwrap();
        if i + 1 == bodies.len() {
            assert_eq!(output.as_deref(), Some(original));
        } else {
            assert_eq!(output, None);
        }
    }
    assert!(!receiver.pending());
    assert!(!receiver.failed());
}

#[test]
fn outbound_literal_golden_both_directions() {
    // Independent specification bytes, NOT encoded with frame() or Part::decode.
    // Full wire ID 128 is 80 01; threshold 30 reserves 26, content width 4.
    let original = [
        0x80, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e,
        0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d,
        0x1e,
    ];
    let expected: &[&[u8]] = &[
        &[5, 1, 0x80, 0x01, 0x02, 0x03],
        &[5, 0, 0x04, 0x05, 0x06, 0x07],
        &[5, 0, 0x08, 0x09, 0x0a, 0x0b],
        &[5, 0, 0x0c, 0x0d, 0x0e, 0x0f],
        &[5, 0, 0x10, 0x11, 0x12, 0x13],
        &[5, 0, 0x14, 0x15, 0x16, 0x17],
        &[5, 0, 0x18, 0x19, 0x1a, 0x1b],
        &[4, 2, 0x1c, 0x1d, 0x1e],
    ];
    for flow in [Flow::Clientbound, Flow::Serverbound] {
        let bodies = split_bodies(outbound(flow, 30).encode(&original).unwrap());
        assert_eq!(bodies.len(), expected.len());
        for (body, literal) in bodies.iter().zip(expected) {
            assert_eq!(body.as_slice(), *literal);
        }
        roundtrip_bodies(&bodies, &original);
    }
}

#[test]
fn outbound_unsplit_borrows_exact_original_including_nonminimal_id() {
    for flow in [Flow::Clientbound, Flow::Serverbound] {
        let encoder = outbound(flow, 30);
        for length in 1..=30 {
            let mut original = vec![0x55; length];
            if length >= 2 {
                original[..2].copy_from_slice(&[0x80, 0]);
            }
            match encoder.encode(&original).unwrap() {
                OutboundPacket::Unsplit(bytes) => {
                    assert_eq!(bytes, original);
                    assert_eq!(bytes.as_ptr(), original.as_ptr());
                }
                OutboundPacket::Split(_) => panic!("must not force small originals into fragments"),
            }
        }
    }
}

#[test]
fn outbound_rejects_empty_truncated_negative_and_overflow_ids_even_unsplit() {
    let encoder = outbound(Flow::Serverbound, 30);
    for original in [
        vec![],
        vec![0x80],
        vec![0x80, 0x80],
        vec![0x80, 0x80, 0x80],
        vec![0x80, 0x80, 0x80, 0x80],
        vec![0xff, 0xff, 0xff, 0xff, 0x0f], // negative packet ID
        vec![0x80, 0x80, 0x80, 0x80, 0x08], // sign bit
        vec![0x80, 0x80, 0x80, 0x80, 0x10], // overflow
        vec![0x80, 0x80, 0x80, 0x80, 0x80, 0],
    ] {
        assert!(encoder.encode(&original).is_err(), "{original:?}");
    }
    // Structural validation is not an assertion that the host implements this ID.
    let largest = [0xff, 0xff, 0xff, 0xff, 7];
    assert_eq!(
        encoder.encode(&largest).unwrap(),
        OutboundPacket::Unsplit(&largest)
    );
}

#[test]
fn outbound_configuration_only_and_invalid_thresholds_refuse() {
    for flow in [Flow::Clientbound, Flow::Serverbound] {
        for protocol in [
            ConnectionProtocol::Handshaking,
            ConnectionProtocol::Play,
            ConnectionProtocol::Status,
            ConnectionProtocol::Login,
        ] {
            assert!(OutboundSplitter::new(flow, protocol, 30).is_err());
        }
        for limit in (0..=26).chain([MAX_PACKET_BYTES + 1, usize::MAX]) {
            assert!(OutboundSplitter::new(flow, ConnectionProtocol::Configuration, limit).is_err());
        }
        assert!(OutboundSplitter::new(flow, ConnectionProtocol::Configuration, 27).is_ok());
        assert!(
            OutboundSplitter::new(flow, ConnectionProtocol::Configuration, MAX_PACKET_BYTES)
                .is_ok()
        );
    }
}

#[test]
fn outbound_literal_id_spans_fragments_and_last_has_one_byte() {
    let mut original: Vec<u8> = (0..28).collect();
    original[..2].copy_from_slice(&[0x80, 1]);
    let bodies = split_bodies(outbound(Flow::Serverbound, 27).encode(&original).unwrap());
    assert_eq!(bodies.len(), 28);
    assert_eq!(bodies[0], [2, 1, 0x80]);
    assert_eq!(bodies[1], [2, 0, 1]);
    assert_eq!(bodies[27], [2, 2, 27]);
    roundtrip_bodies(&bodies, &original);
}

#[test]
fn outbound_exhaustive_small_thresholds_lengths_and_remainders() {
    // Every remainder, exact threshold, threshold+1, exact divisions and one-byte
    // tails for all widths 1..=74. Assertions do not rely on our inbound decoder.
    for flow in [Flow::Clientbound, Flow::Serverbound] {
        for threshold in 27..=100 {
            let encoder = outbound(flow, threshold);
            for length in 1..=400 {
                let original: Vec<u8> = (0..length).map(|i| (i % 127) as u8).collect();
                let result = encoder.encode(&original);
                if length <= threshold {
                    assert_eq!(result.unwrap(), OutboundPacket::Unsplit(&original));
                    continue;
                }
                let width = threshold - 26;
                let count = (length - 1) / width + 1;
                if count > 256 {
                    assert_eq!(result.unwrap_err(), Error::Limit("split parts"));
                    continue;
                }
                let bodies = split_bodies(result.unwrap());
                assert_eq!(bodies.len(), count);
                for (i, body) in bodies.iter().enumerate() {
                    let start = i * width;
                    let end = ((i + 1) * width).min(length);
                    let marker = if i == 0 {
                        1
                    } else if i + 1 == count {
                        2
                    } else {
                        0
                    };
                    let mut expected = vec![(end - start + 1) as u8, marker];
                    expected.extend_from_slice(&original[start..end]);
                    assert_eq!(body, &expected);
                    assert!(body.len() + 20 <= threshold); // max outer ID + channel
                }
                roundtrip_bodies(&bodies, &original);
            }
        }
    }
}

#[test]
fn outbound_literal_array_length_varint_boundaries() {
    // Array lengths 127, 128, 16383, 16384; marker is inside, not before, array.
    let cases: &[(usize, &[u8])] = &[
        (126, &[0x7f, 1]),
        (127, &[0x80, 0x01, 1]),
        (16_382, &[0xff, 0x7f, 1]),
        (16_383, &[0x80, 0x80, 0x01, 1]),
    ];
    for &(width, prefix) in cases {
        let original = vec![0x55; width + 27];
        let bodies = split_bodies(
            outbound(Flow::Clientbound, width + 26)
                .encode(&original)
                .unwrap(),
        );
        let mut first = prefix.to_vec();
        first.extend_from_slice(&original[..width]);
        let mut last = vec![0x1c, 2]; // independent literal: 27 content + marker
        last.extend_from_slice(&[0x55; 27]);
        assert_eq!(bodies, [first, last]);
        roundtrip_bodies(&bodies, &original);
    }
}

#[test]
fn outbound_directional_caps_include_varint_and_marker() {
    let original = vec![0x55; 2 * 1_048_576 + 1];
    for (flow, cap, content, prefix) in [
        (Flow::Serverbound, 32_767, 32_763, [0xfc, 0xff, 0x01, 1]),
        (
            Flow::Clientbound,
            1_048_576,
            1_048_572,
            [0xfd, 0xff, 0x3f, 1],
        ),
    ] {
        let bodies = split_bodies(outbound(flow, 2 * 1_048_576).encode(&original).unwrap());
        assert_eq!(bodies[0].len(), cap);
        assert_eq!(&bodies[0][..4], &prefix);
        assert_eq!(&bodies[0][4..], &original[..content]);
        assert!(bodies.iter().all(|body| body.len() <= cap));
        assert!(bodies.iter().all(|body| body.len() + 20 <= 2 * 1_048_576));
        roundtrip_bodies(&bodies, &original);
    }
}

#[test]
fn outbound_exact_part_limit_and_next_part_rejected_without_batch() {
    let encoder = outbound(Flow::Clientbound, 27);
    let allowed = vec![0x55; 256];
    let bodies = split_bodies(encoder.encode(&allowed).unwrap());
    assert_eq!(bodies.len(), 256);
    assert_eq!(bodies[0], [2, 1, 0x55]);
    assert_eq!(bodies[255], [2, 2, 0x55]);
    roundtrip_bodies(&bodies, &allowed);
    assert_eq!(
        encoder.encode(&[0x55; 257]).unwrap_err(),
        Error::Limit("split parts")
    );
    assert_eq!(split_bodies(encoder.encode(&allowed).unwrap()), bodies);
}

#[test]
fn outbound_serverbound_maximum_parts_is_stricter_than_aggregate() {
    let encoder = outbound(Flow::Serverbound, 1_048_576);
    let allowed = vec![0x55; 32_763 * 256];
    let bodies = split_bodies(encoder.encode(&allowed).unwrap());
    assert_eq!(bodies.len(), 256);
    assert!(bodies.iter().all(|body| body.len() == 32_767));
    roundtrip_bodies(&bodies, &allowed);
    let one_more = vec![0x55; 32_763 * 256 + 1];
    assert_eq!(
        encoder.encode(&one_more).unwrap_err(),
        Error::Limit("split parts")
    );
    let aggregate = vec![0x55; MAX_PACKET_BYTES];
    assert_eq!(
        encoder.encode(&aggregate).unwrap_err(),
        Error::Limit("split parts")
    );
}

#[test]
fn outbound_clientbound_exact_aggregate_and_next_byte() {
    let encoder = outbound(Flow::Clientbound, 2 * 1_048_576);
    let original = vec![0x55; MAX_PACKET_BYTES];
    let bodies = split_bodies(encoder.encode(&original).unwrap());
    assert_eq!(bodies.len(), 9);
    assert_eq!(bodies[8].len(), 34);
    assert_eq!(&bodies[8][..2], &[33, 2]); // 32-byte final content
    assert_eq!(&bodies[8][2..], &[0x55; 32]);
    assert!(bodies.iter().map(Vec::len).sum::<usize>() <= MAX_PACKET_BYTES + 256 * 6);
    roundtrip_bodies(&bodies, &original);
    let too_large = vec![0x55; MAX_PACKET_BYTES + 1];
    assert_eq!(
        encoder.encode(&too_large).unwrap_err(),
        Error::Limit("split packet")
    );
}

#[test]
fn outbound_unsplit_policy_is_not_overridden_by_directional_fragment_caps() {
    let original = vec![0x55; MAX_PACKET_BYTES];
    for flow in [Flow::Clientbound, Flow::Serverbound] {
        // This is NOT permission to put 8 MiB in an ordinary custom payload:
        // the host must validate the original's actual packet type and codec.
        let encoder = outbound(flow, MAX_PACKET_BYTES);
        match encoder.encode(&original).unwrap() {
            OutboundPacket::Unsplit(bytes) => assert_eq!(bytes.as_ptr(), original.as_ptr()),
            OutboundPacket::Split(_) => panic!("threshold equality must preserve the original"),
        }
    }
}

#[test]
fn outbound_keeps_full_five_byte_id_and_payload_bytes_without_padding() {
    let mut original = vec![0xff, 0xff, 0xff, 0xff, 7];
    original.extend_from_slice(&[0; 26]);
    let bodies = split_bodies(outbound(Flow::Clientbound, 30).encode(&original).unwrap());
    assert_eq!(bodies[0], [5, 1, 0xff, 0xff, 0xff, 0xff]);
    assert_eq!(bodies[1], [5, 0, 7, 0, 0, 0]);
    assert_eq!(bodies[7], [4, 2, 0, 0, 0]);
    assert_eq!(
        bodies.iter().map(|b| b.len() - 2).sum::<usize>(),
        original.len()
    );
    roundtrip_bodies(&bodies, &original);
}

#[test]
fn inbound_irregular_growth_reaches_exact_aggregate_without_padding_or_leaks() {
    let mut receiver = Reassembler::new();
    let first = vec![0x11; MAX_PART_BYTES - 17];
    assert_eq!(receiver.receive(&frame(1, &first), 0).unwrap(), None);
    for byte in 2..=8 {
        assert_eq!(
            receiver
                .receive(&frame(0, &vec![byte; MAX_PART_BYTES]), u64::from(byte))
                .unwrap(),
            None
        );
    }
    let packet = receiver.receive(&frame(2, &[0x99; 17]), 9).unwrap().unwrap();
    assert_eq!(packet.len(), MAX_PACKET_BYTES);
    assert_eq!(&packet[..first.len()], first.as_slice());
    for byte in 2..=8 {
        let start = first.len() + usize::from(byte - 2) * MAX_PART_BYTES;
        assert!(packet[start..start + MAX_PART_BYTES]
            .iter()
            .all(|value| *value == byte));
    }
    assert_eq!(&packet[MAX_PACKET_BYTES - 17..], &[0x99; 17]);
    assert!(!receiver.pending());
    assert!(!receiver.failed());
    receiver.receive(&frame(1, &[7]), 10).unwrap();
    assert_eq!(receiver.receive(&frame(2, &[8]), 11).unwrap(), Some(vec![7, 8]));
    // Ownership of the first result is independent of subsequent assemblies.
    assert_eq!(packet.len(), MAX_PACKET_BYTES);
    assert_eq!(packet[0], 0x11);
}

#[test]
fn inbound_new_assembly_gets_own_deadline_but_never_resets_epoch_clock() {
    let mut receiver = Reassembler::new();
    receiver.receive(&frame(1, &[1]), 0).unwrap();
    receiver.receive(&frame(2, &[2]), MAX_ASSEMBLY_MILLIS - 1).unwrap();
    let second_start = 2 * MAX_ASSEMBLY_MILLIS;
    receiver.check_deadline(second_start).unwrap();
    receiver.receive(&frame(1, &[3]), second_start).unwrap();
    receiver
        .check_deadline(second_start + MAX_ASSEMBLY_MILLIS - 1)
        .unwrap();
    assert!(receiver
        .receive(&frame(2, &[4]), second_start + MAX_ASSEMBLY_MILLIS)
        .is_err());
    assert!(receiver.failed());
    assert!(!receiver.pending());
    assert!(receiver.receive(&frame(1, &[5]), 0).is_err());

    let mut receiver = Reassembler::new();
    receiver.receive(&frame(1, &[1]), 10).unwrap();
    receiver.receive(&frame(2, &[2]), 11).unwrap();
    // Completion frees bytes/deadline, not the monotonic epoch clock.
    assert!(receiver.receive(&frame(1, &[3]), 10).is_err());
    assert!(receiver.failed());
    assert!(!receiver.pending());
}

#[test]
fn outbound_invalid_ids_cannot_bypass_validation_by_requiring_fragmentation() {
    let encoder = outbound(Flow::Serverbound, 30);
    for prefix in [
        [0xff, 0xff, 0xff, 0xff, 0x0f],
        [0x80, 0x80, 0x80, 0x80, 0x08],
        [0x80, 0x80, 0x80, 0x80, 0x10],
        [0x80, 0x80, 0x80, 0x80, 0x80],
    ] {
        let mut original = vec![0x55; 400];
        original[..5].copy_from_slice(&prefix);
        assert_eq!(encoder.encode(&original).unwrap_err(), Error::VarInt);
    }
    // Rejected originals expose no batch and cannot poison this stateless encoder.
    let valid = vec![0x55; 400];
    roundtrip_bodies(&split_bodies(encoder.encode(&valid).unwrap()), &valid);
}

#[test]
fn outbound_fragmented_nonminimal_id_preserves_exact_original_encoding() {
    let mut original = vec![0x55; 31];
    original[..2].copy_from_slice(&[0x80, 0]);
    let bodies = split_bodies(outbound(Flow::Serverbound, 30).encode(&original).unwrap());
    assert_eq!(bodies[0], [5, 1, 0x80, 0, 0x55, 0x55]);
    assert_eq!(bodies[7], [4, 2, 0x55, 0x55, 0x55]);
    roundtrip_bodies(&bodies, &original);
}

#[test]
fn outbound_adjacent_thresholds_at_directional_content_saturation() {
    for (flow, cap, maximum_content) in [
        (Flow::Serverbound, 32_767, 32_763),
        (Flow::Clientbound, 1_048_576, 1_048_572),
    ] {
        let saturation = maximum_content + 26;
        for threshold in saturation - 1..=saturation + 1 {
            let width = (threshold - 26).min(maximum_content);
            let original = vec![0x55; threshold + 1];
            let bodies = split_bodies(outbound(flow, threshold).encode(&original).unwrap());
            assert_eq!(bodies.len(), 2);
            assert_eq!(bodies[0].len(), width + 4); // three-byte length + marker
            assert!(bodies.iter().all(|body| body.len() <= cap));
            assert!(bodies.iter().all(|body| body.len() + 20 <= threshold));
            assert_eq!(&bodies[0][4..], &original[..width]);
            assert_eq!(&bodies[1][2..], &original[width..]);
            roundtrip_bodies(&bodies, &original);
        }
    }
}
