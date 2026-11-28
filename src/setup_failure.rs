//! Bounded, lossless setup-failure **NBT data**, NOT a Component renderer or
//! semantic Component validator. See provenance/setup-failure.md for evidence.
//! Decoding data never authorizes negotiation, a reply, or configuration finish.
//! Consumers must treat every field as untrusted; do not execute click/hover
//! events, resolve selectors/NBT, fetch URLs, or interpolate into markup/logs.
use crate::{
    codec::{Id, Reader},
    modded::MAX_FAILURE_REASONS,
    Error, Result,
};
use std::collections::{BTreeMap, BTreeSet};

/// Local bounds, intentionally smaller than the upstream trusted codec.
/// Depth counts the root as one; nodes include array elements and all tags.
pub const MAX_DEPTH: usize = 32;
pub const MAX_NODES: usize = 4_096;
pub const MAX_COLLECTION_LENGTH: usize = 4_096;
pub const MAX_STRING_BYTES: usize = 8_192;
pub const MAX_TOTAL_STRING_BYTES: usize = 16_384;

/// Java readUTF data. UTF-16 units preserve lone surrogates; raw bytes preserve
/// even noncanonical encodings accepted by Java. No lossy display conversion.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NbtString {
    modified_utf8: Vec<u8>,
    utf16: Vec<u16>,
}
impl NbtString {
    pub fn modified_utf8(&self) -> &[u8] {
        &self.modified_utf8
    }
    pub fn utf16(&self) -> &[u16] {
        &self.utf16
    }
}

/// All NBT payload types except End (only a compound terminator). Floats retain
/// their exact IEEE bits, including NaN payloads. Compounds preserve wire order;
/// duplicate UTF-16 keys are rejected rather than overwritten as Java does.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NbtData {
    Byte(i8),
    Short(i16),
    Int(i32),
    Long(i64),
    FloatBits(u32),
    DoubleBits(u64),
    ByteArray(Vec<i8>),
    String(NbtString),
    List {
        element_type: u8,
        values: Vec<NbtData>,
    },
    Compound(Vec<(NbtString, NbtData)>),
    IntArray(Vec<i32>),
    LongArray(Vec<i64>),
}

/// Structurally decoded failure data, NOT semantically validated Components.
/// The original complete body is retained for exact forensic roundtrips (even
/// map ordering, shorthand IDs and nonminimal VarInts); there is no send API.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SetupFailure {
    reasons: BTreeMap<Id, NbtData>,
    original_body: Vec<u8>,
}
impl SetupFailure {
    pub fn reasons(&self) -> &BTreeMap<Id, NbtData> {
        &self.reasons
    }
    pub fn original_body(&self) -> &[u8] {
        &self.original_body
    }
    /// Whole custom-payload body, without packet framing or channel ID.
    /// Rejects invalid NBT, duplicate map/compound keys and non-Component root
    /// kinds. A string, nonempty list, or compound is only a Component *envelope*:
    /// content variants, style, events and registry semantics are NOT validated.
    pub fn decode(bytes: &[u8]) -> Result<Self> {
        let mut r = Reader::new(bytes)?;
        let count = r.count(MAX_FAILURE_REASONS)?;
        let mut budget = Budget::default();
        let mut reasons = BTreeMap::new();
        for _ in 0..count {
            let id = r.id()?;
            if reasons.contains_key(&id) {
                return Err(Error::Duplicate(id.to_string()));
            }
            // Network NBT: one type byte and payload, NOT a named root and NOT
            // JSON, compressed NBT, a VarInt string or a registry-prefixed value.
            let kind = r.byte()?;
            let value = budget.tag(&mut r, kind, 1)?;
            component_envelope(&value)?;
            reasons.insert(id, value);
        }
        r.finish()?;
        Ok(Self {
            reasons,
            original_body: bytes.to_vec(),
        })
    }
}

/// Only envelope validation, deliberately not a claim of Component semantics.
/// Lists recursively contain Component envelopes. Heterogeneous NbtOps wrapper
/// compounds remain opaque data, like all other compounds; never render them.
fn component_envelope(value: &NbtData) -> Result<()> {
    match value {
        NbtData::String(_) | NbtData::Compound(_) => Ok(()),
        NbtData::List { values, .. } if !values.is_empty() => {
            for value in values {
                component_envelope(value)?;
            }
            Ok(())
        }
        _ => Err(Error::Invalid("failure Component NBT envelope")),
    }
}

#[derive(Default)]
struct Budget {
    nodes: usize,
    string_bytes: usize,
}
impl Budget {
    fn nodes(&mut self, n: usize) -> Result<()> {
        self.nodes = self
            .nodes
            .checked_add(n)
            .filter(|n| *n <= MAX_NODES)
            .ok_or(Error::Limit("failure NBT nodes"))?;
        Ok(())
    }
    fn string(&mut self, r: &mut Reader<'_>) -> Result<NbtString> {
        let len = u16::from_be_bytes(read(r)?) as usize;
        if len > MAX_STRING_BYTES {
            return Err(Error::Limit("failure NBT string"));
        }
        self.string_bytes = self
            .string_bytes
            .checked_add(len)
            .filter(|n| *n <= MAX_TOTAL_STRING_BYTES)
            .ok_or(Error::Limit("failure NBT strings"))?;
        let mut raw = Vec::new();
        for _ in 0..len {
            raw.push(r.byte()?);
        }
        let utf16 = modified_utf8(&raw)?;
        Ok(NbtString {
            modified_utf8: raw,
            utf16,
        })
    }
    fn length(&self, r: &mut Reader<'_>) -> Result<usize> {
        let len = i32::from_be_bytes(read(r)?);
        if len < 0 {
            return Err(Error::Invalid("negative failure NBT length"));
        }
        let len = len as usize;
        if len > MAX_COLLECTION_LENGTH || len > MAX_NODES - self.nodes {
            return Err(Error::Limit("failure NBT collection"));
        }
        Ok(len)
    }
    fn tag(&mut self, r: &mut Reader<'_>, kind: u8, depth: usize) -> Result<NbtData> {
        if depth > MAX_DEPTH {
            return Err(Error::Limit("failure NBT depth"));
        }
        self.nodes(1)?;
        Ok(match kind {
            1 => NbtData::Byte(r.byte()? as i8),
            2 => NbtData::Short(i16::from_be_bytes(read(r)?)),
            3 => NbtData::Int(i32::from_be_bytes(read(r)?)),
            4 => NbtData::Long(i64::from_be_bytes(read(r)?)),
            5 => NbtData::FloatBits(u32::from_be_bytes(read(r)?)),
            6 => NbtData::DoubleBits(u64::from_be_bytes(read(r)?)),
            7 => {
                let len = self.length(r)?;
                self.nodes(len)?;
                let mut values = Vec::new();
                for _ in 0..len {
                    values.push(r.byte()? as i8);
                }
                NbtData::ByteArray(values)
            }
            8 => NbtData::String(self.string(r)?),
            9 => {
                let element_type = r.byte()?;
                let len = self.length(r)?;
                if element_type > 12 || (element_type == 0 && len != 0) {
                    return Err(Error::Invalid("failure NBT list type"));
                }
                let mut values = Vec::new();
                for _ in 0..len {
                    values.push(self.tag(r, element_type, depth + 1)?);
                }
                NbtData::List {
                    element_type,
                    values,
                }
            }
            10 => {
                let mut entries = Vec::new();
                let mut keys = BTreeSet::new();
                loop {
                    let kind = r.byte()?;
                    if kind == 0 {
                        break;
                    }
                    if kind > 12 {
                        return Err(Error::Invalid("failure NBT tag type"));
                    }
                    // Charge/check the child before allocating its name.
                    if self.nodes == MAX_NODES {
                        return Err(Error::Limit("failure NBT nodes"));
                    }
                    let key = self.string(r)?;
                    if !keys.insert(key.utf16.clone()) {
                        return Err(Error::Invalid("duplicate failure NBT key"));
                    }
                    entries.push((key, self.tag(r, kind, depth + 1)?));
                }
                NbtData::Compound(entries)
            }
            11 => {
                let len = self.length(r)?;
                self.nodes(len)?;
                let mut values = Vec::new();
                for _ in 0..len {
                    values.push(i32::from_be_bytes(read(r)?));
                }
                NbtData::IntArray(values)
            }
            12 => {
                let len = self.length(r)?;
                self.nodes(len)?;
                let mut values = Vec::new();
                for _ in 0..len {
                    values.push(i64::from_be_bytes(read(r)?));
                }
                NbtData::LongArray(values)
            }
            _ => return Err(Error::Invalid("failure NBT tag type")),
        })
    }
}
fn read<const N: usize>(r: &mut Reader<'_>) -> Result<[u8; N]> {
    let mut bytes = [0; N];
    for byte in &mut bytes {
        *byte = r.byte()?;
    }
    Ok(bytes)
}

/// DataInput.readUTF's grammar, NOT standard UTF-8. Java accepts raw NUL and
/// overlong two/three-byte encodings, but not four-byte UTF-8. Surrogate units
/// are valid and kept individually, including unpaired ones.
fn modified_utf8(raw: &[u8]) -> Result<Vec<u16>> {
    let mut units = Vec::new();
    let mut pos = 0;
    while pos < raw.len() {
        let first = raw[pos];
        pos += 1;
        let (mut unit, tails) = match first {
            0x00..=0x7f => (u16::from(first), 0),
            0xc0..=0xdf => (u16::from(first & 0x1f), 1),
            0xe0..=0xef => (u16::from(first & 0x0f), 2),
            _ => return Err(Error::Invalid("failure NBT modified UTF-8")),
        };
        for _ in 0..tails {
            let next = *raw
                .get(pos)
                .ok_or(Error::Invalid("failure NBT modified UTF-8"))?;
            if next & 0xc0 != 0x80 {
                return Err(Error::Invalid("failure NBT modified UTF-8"));
            }
            unit = (unit << 6) | u16::from(next & 0x3f);
            pos += 1;
        }
        units.push(unit);
    }
    Ok(units)
}
