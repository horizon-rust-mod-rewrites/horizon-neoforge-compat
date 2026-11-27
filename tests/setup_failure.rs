use horizon_neoforge_compat::{
    codec::{Id, MAX_BODY},
    setup_failure::{
        NbtData, SetupFailure, MAX_DEPTH, MAX_NODES, MAX_STRING_BYTES, MAX_TOTAL_STRING_BYTES,
    },
    Error,
};

fn vi(mut n: u32) -> Vec<u8> {
    let mut out = Vec::new();
    loop {
        let b = (n & 127) as u8;
        n >>= 7;
        out.push(if n == 0 { b } else { b | 128 });
        if n == 0 {
            return out;
        }
    }
}
fn utf(raw: &[u8]) -> Vec<u8> {
    let mut out = (raw.len() as u16).to_be_bytes().to_vec();
    out.extend(raw);
    out
}
fn string(raw: &[u8]) -> Vec<u8> {
    let mut out = vec![8];
    out.extend(utf(raw));
    out
}
fn body(entries: &[(&str, Vec<u8>)]) -> Vec<u8> {
    let mut out = vi(entries.len() as u32);
    for (id, tag) in entries {
        out.extend(vi(id.len() as u32));
        out.extend(id.as_bytes());
        out.extend(tag);
    }
    out
}
fn one(tag: Vec<u8>) -> Vec<u8> {
    body(&[("demo:bad", tag)])
}
fn named(name: &[u8], tag: &[u8]) -> Vec<u8> {
    let mut out = vec![tag[0]];
    out.extend(utf(name));
    out.extend(&tag[1..]);
    out
}
fn compound(entries: &[(&[u8], Vec<u8>)]) -> Vec<u8> {
    let mut out = vec![10];
    for (key, tag) in entries {
        out.extend(named(key, tag));
    }
    out.push(0);
    out
}
fn root(b: &[u8]) -> NbtData {
    SetupFailure::decode(b).unwrap().reasons()[&Id::parse("demo:bad").unwrap()].clone()
}

#[test]
fn literal_network_nbt_string_not_json_or_named_root() {
    let bytes = b"\x01\x08demo:bad\x08\x00\x03bad";
    let decoded = SetupFailure::decode(bytes).unwrap();
    assert_eq!(decoded.original_body(), bytes);
    match root(bytes) {
        NbtData::String(s) => {
            assert_eq!(s.modified_utf8(), b"bad");
            assert_eq!(s.utf16(), &[98, 97, 100]);
        }
        other => panic!("{other:?}"),
    }
    // There is no u16 empty root name after the type.
    assert!(SetupFailure::decode(&one(vec![8, 0, 0, 0, 3, b'b', b'a', b'd'])).is_err());
    assert!(SetupFailure::decode(&one(b"{\"text\":\"bad\"}".to_vec())).is_err());
}
#[test]
fn structured_data_is_not_rendered_or_semantically_validated() {
    let tag = compound(&[
        (b"translate", string(b"neoforge.network.negotiation.failed")),
        (b"with", vec![9, 8, 0, 0, 0, 1, 0, 3, b'b', b'a', b'd']),
        (
            b"clickEvent",
            compound(&[
                (b"action", string(b"open_url")),
                (b"value", string(b"https://invalid.test/")),
            ]),
        ),
        (b"unknown", vec![1, 1]),
    ]);
    let bytes = one(tag);
    let decoded = SetupFailure::decode(&bytes).unwrap();
    assert_eq!(decoded.original_body(), bytes);
    let NbtData::Compound(entries) = root(&bytes) else {
        panic!()
    };
    assert_eq!(entries.len(), 4);
    assert_eq!(
        entries[0].0.utf16(),
        "translate".encode_utf16().collect::<Vec<_>>()
    );
    assert!(matches!(
        entries[1].1,
        NbtData::List {
            element_type: 8,
            ..
        }
    ));
    assert!(matches!(entries[2].1, NbtData::Compound(_)));
    // An empty compound is valid NBT *data*, not a valid Component. This API
    // deliberately does not certify Component content variants or style.
    assert!(SetupFailure::decode(&one(vec![10, 0])).is_ok());
}
#[test]
fn every_nbt_payload_type_and_numeric_bits_are_preserved() {
    let tags = vec![
        vec![1, 255],
        vec![2, 128, 0],
        vec![3, 128, 0, 0, 0],
        vec![4, 128, 0, 0, 0, 0, 0, 0, 0],
        vec![5, 127, 192, 0, 1],
        vec![6, 127, 248, 0, 0, 0, 0, 0, 1],
        vec![7, 0, 0, 0, 2, 128, 127],
        string(b""),
        vec![9, 0, 0, 0, 0, 0],
        vec![10, 0],
        vec![11, 0, 0, 0, 1, 255, 255, 255, 255],
        vec![12, 0, 0, 0, 1, 255, 255, 255, 255, 255, 255, 255, 255],
    ];
    let mut tag = vec![10];
    for (n, value) in tags.iter().enumerate() {
        tag.extend(named(&[b'a' + n as u8], value));
    }
    tag.push(0);
    let bytes = one(tag);
    for end in 0..bytes.len() {
        assert!(
            SetupFailure::decode(&bytes[..end]).is_err(),
            "all-types prefix {end}"
        );
    }
    let NbtData::Compound(entries) = root(&bytes) else {
        panic!()
    };
    assert_eq!(
        entries.iter().map(|e| e.1.clone()).collect::<Vec<_>>(),
        vec![
            NbtData::Byte(-1),
            NbtData::Short(i16::MIN),
            NbtData::Int(i32::MIN),
            NbtData::Long(i64::MIN),
            NbtData::FloatBits(0x7fc00001),
            NbtData::DoubleBits(0x7ff8000000000001),
            NbtData::ByteArray(vec![-128, 127]),
            entries[7].1.clone(),
            NbtData::List {
                element_type: 0,
                values: vec![]
            },
            NbtData::Compound(vec![]),
            NbtData::IntArray(vec![-1]),
            NbtData::LongArray(vec![-1]),
        ]
    );
}
#[test]
fn mutf8_preserves_null_supplementary_lone_surrogates_and_java_overlongs() {
    for (encoded, units) in [
        (vec![0xc0, 0x80], vec![0]),
        (vec![0], vec![0]),
        (
            vec![0xed, 0xa0, 0xbd, 0xed, 0xb8, 0x80],
            vec![0xd83d, 0xde00],
        ),
        (vec![0xed, 0xa0, 0x80], vec![0xd800]),
        (vec![0xc1, 0x81], vec![65]),
        (vec![0xe0, 0x81, 0x81], vec![65]),
    ] {
        let bytes = one(string(&encoded));
        let NbtData::String(value) = root(&bytes) else {
            panic!()
        };
        assert_eq!(value.modified_utf8(), encoded);
        assert_eq!(value.utf16(), units);
        assert_eq!(SetupFailure::decode(&bytes).unwrap().original_body(), bytes);
    }
    for raw in [
        vec![0x80],
        vec![0xc0],
        vec![0xc0, 0x41],
        vec![0xe1, 0x80],
        vec![0xe1, 0x41, 0x80],
        vec![0xf0, 0x9f, 0x98, 0x80],
        vec![0xff],
    ] {
        assert!(SetupFailure::decode(&one(string(&raw))).is_err(), "{raw:?}");
    }
}
#[test]
fn duplicate_keys_compare_decoded_java_units_not_encoding() {
    assert!(SetupFailure::decode(&one(compound(&[
        (b"a", string(b"one")),
        (b"a", string(b"two"))
    ])))
    .is_err());
    assert!(SetupFailure::decode(&one(compound(&[
        (b"A", string(b"one")),
        (&[0xc1, 0x81], string(b"two"))
    ])))
    .is_err());
    assert!(SetupFailure::decode(&body(&[
        ("demo:bad", string(b"one")),
        ("demo:bad", string(b"two"))
    ]))
    .is_err());
    assert!(SetupFailure::decode(&body(&[
        ("bad", string(b"one")),
        ("minecraft:bad", string(b"two"))
    ]))
    .is_err());
}
#[test]
fn list_and_compound_component_envelopes() {
    let mut tag = vec![9, 8, 0, 0, 0, 2];
    tag.extend(utf(b"one"));
    tag.extend(utf(b"two"));
    assert!(SetupFailure::decode(&one(tag)).is_ok());
    for tag in [
        vec![0],
        vec![1, 0],
        vec![3, 0, 0, 0, 0],
        vec![9, 0, 0, 0, 0, 0],
        vec![9, 1, 0, 0, 0, 1, 0],
    ] {
        assert!(SetupFailure::decode(&one(tag)).is_err());
    }
}
#[test]
fn malformed_nbt_types_lengths_and_terminators() {
    for tag in [
        vec![13],
        vec![255],
        vec![10, 13],
        vec![10, 8, 0, 0, 0, 0],
        vec![9, 0, 0, 0, 0, 1],
        vec![9, 13, 0, 0, 0, 0],
        vec![9, 8, 255, 255, 255, 255],
        compound(&[(b"x", vec![7, 255, 255, 255, 255])]),
        compound(&[(b"x", vec![11, 127, 255, 255, 255])]),
        compound(&[(b"x", vec![12, 0, 0, 16, 1])]),
    ] {
        assert!(SetupFailure::decode(&one(tag.clone())).is_err(), "{tag:?}");
    }
}
#[test]
fn all_prefix_truncations_of_populated_and_empty_maps_fail() {
    let bytes = body(&[
        (
            "demo:bad",
            compound(&[
                (b"text", string(b"bad")),
                (b"data", vec![12, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1]),
            ]),
        ),
        ("demo:other", string(b"other")),
    ]);
    assert!(SetupFailure::decode(&bytes).is_ok());
    for end in 0..bytes.len() {
        assert!(SetupFailure::decode(&bytes[..end]).is_err(), "prefix {end}");
    }
    assert!(SetupFailure::decode(&[]).is_err());
    assert!(SetupFailure::decode(&[0]).unwrap().reasons().is_empty());
    let mut trailing = bytes;
    trailing.push(0);
    assert_eq!(SetupFailure::decode(&trailing), Err(Error::Trailing));
    assert_eq!(SetupFailure::decode(&[0, 0]), Err(Error::Trailing));
}
#[test]
fn depth_boundary_and_deep_attack_are_bounded() {
    fn nested(depth: usize) -> Vec<u8> {
        let mut tag = string(b"leaf");
        for _ in 1..depth {
            tag = compound(&[(b"", tag)]);
        }
        tag
    }
    assert!(SetupFailure::decode(&one(nested(MAX_DEPTH))).is_ok());
    assert_eq!(
        SetupFailure::decode(&one(nested(MAX_DEPTH + 1))),
        Err(Error::Limit("failure NBT depth"))
    );
    assert!(SetupFailure::decode(&one(nested(1_000))).is_err());
    // Lists use the same depth counter as compounds.
    let mut tag = string(b"leaf");
    for _ in 1..MAX_DEPTH {
        let mut next = vec![9, tag[0], 0, 0, 0, 1];
        next.extend(&tag[1..]);
        tag = next;
    }
    assert!(SetupFailure::decode(&one(tag.clone())).is_ok());
    let mut extra = vec![9, tag[0], 0, 0, 0, 1];
    extra.extend(&tag[1..]);
    assert!(SetupFailure::decode(&one(extra)).is_err());
}
#[test]
fn node_and_array_budgets_are_aggregate() {
    fn array(n: usize) -> Vec<u8> {
        let mut tag = vec![7];
        tag.extend((n as i32).to_be_bytes());
        tag.resize(5 + n, 0);
        tag
    }
    // A compound + byte-array tag consume two nodes before their scalar data.
    assert!(SetupFailure::decode(&one(compound(&[(b"x", array(MAX_NODES - 2))]))).is_ok());
    assert!(SetupFailure::decode(&one(compound(&[(b"x", array(MAX_NODES - 1))]))).is_err());
    let a = compound(&[(b"x", array(MAX_NODES / 2 - 2))]);
    assert!(SetupFailure::decode(&body(&[("a", a.clone()), ("b", a.clone())])).is_ok());
    assert!(
        SetupFailure::decode(&body(&[("a", a.clone()), ("b", a), ("c", string(b""))])).is_err()
    );
    let mut list = vec![9, 8];
    list.extend(((MAX_NODES - 1) as i32).to_be_bytes());
    list.resize(6 + 2 * (MAX_NODES - 1), 0);
    assert!(SetupFailure::decode(&one(list)).is_ok());
}
#[test]
fn string_budgets_are_per_string_and_aggregate_across_reasons_and_names() {
    let max = string(&vec![b'x'; MAX_STRING_BYTES]);
    assert!(SetupFailure::decode(&one(max.clone())).is_ok());
    assert!(SetupFailure::decode(&one(string(&vec![b'x'; MAX_STRING_BYTES + 1]))).is_err());
    assert_eq!(MAX_TOTAL_STRING_BYTES, 2 * MAX_STRING_BYTES);
    assert!(SetupFailure::decode(&body(&[("a", max.clone()), ("b", max.clone())])).is_ok());
    assert!(SetupFailure::decode(&body(&[
        ("a", max.clone()),
        ("b", max.clone()),
        ("c", string(b"x"))
    ]))
    .is_err());
    assert!(SetupFailure::decode(&body(&[
        ("a", max.clone()),
        ("b", compound(&[(b"x", max)]))
    ]))
    .is_err());
    assert!(SetupFailure::decode(&one(vec![8, 255, 255])).is_err());
}
#[test]
fn map_body_identifier_and_varint_limits() {
    for bytes in [
        vec![255, 255, 255, 255, 15],
        vi(1_025),
        vec![128, 128, 128, 128, 16],
        vec![128; 5],
        vec![0; MAX_BODY + 1],
        vec![1, 1, 255, 8, 0, 0],
        body(&[("UPPER:bad", string(b"bad"))]),
    ] {
        assert!(SetupFailure::decode(&bytes).is_err());
    }
    // Nonminimal count and shorthand identifier are retained exactly.
    let bytes = b"\x81\x00\x03bad\x08\x00\x01x";
    let decoded = SetupFailure::decode(bytes).unwrap();
    assert_eq!(decoded.original_body(), bytes);
    assert!(decoded
        .reasons()
        .contains_key(&Id::parse("minecraft:bad").unwrap()));
    let ids: Vec<_> = (0..1_024).map(|n| format!("d:k{n}")).collect();
    let entries: Vec<_> = ids.iter().map(|id| (id.as_str(), string(b""))).collect();
    assert_eq!(
        SetupFailure::decode(&body(&entries))
            .unwrap()
            .reasons()
            .len(),
        1_024
    );
}
#[test]
fn deterministic_malformed_smoke_corpus_does_not_panic() {
    let mut state = 0x7543_21ab_u32;
    for n in 0..4_096 {
        let mut bytes = Vec::new();
        for _ in 0..n % 257 {
            state ^= state << 13;
            state ^= state >> 17;
            state ^= state << 5;
            bytes.push(state as u8);
        }
        let _ = SetupFailure::decode(&bytes);
        let _ = SetupFailure::decode(&one(bytes.clone()));
        if !bytes.is_empty() {
            let _ = SetupFailure::decode(&one(compound(&[(b"x", bytes)])));
        }
    }
}

#[test]
fn executed_java_component_vectors_preserve_wire_data_not_semantic_normalization() {
    // These exact bodies were accepted by the official 1.21.1 Component codec
    // through the source-extracted NeoForge failure map. Java normalizes both
    // {text:"bad"} and ["one","two"] when re-encoding. We retain their input.
    // Execution method, source adaptation, hashes and limits: provenance doc.
    fn hex(s: &str) -> Vec<u8> {
        s.as_bytes()
            .chunks_exact(2)
            .map(|c| u8::from_str_radix(std::str::from_utf8(c).unwrap(), 16).unwrap())
            .collect()
    }
    for encoded in [
        "00",
        "010864656d6f3a626164080003626164",
        "010864656d6f3a6261640a08000474657874000362616400",
        "010864656d6f3a62616409080000000200036f6e65000374776f",
        "010864656d6f3a626164080002c080",
        "010864656d6f3a626164080003eda080",
    ] {
        let bytes = hex(encoded);
        let decoded = SetupFailure::decode(&bytes).unwrap();
        assert_eq!(decoded.original_body(), bytes);
    }
}
