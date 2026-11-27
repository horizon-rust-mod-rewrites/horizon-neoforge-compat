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
