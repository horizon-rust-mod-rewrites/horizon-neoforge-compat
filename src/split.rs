//! Bounded NeoForge 21.1.248 split transport, before packet-ID adaptation.
//!
//! Reassembly returns an entire original Minecraft packet including its wire ID;
//! outbound encoding takes that same unframed representation and returns actual
//! split payload bodies or an unchanged borrowed original. The host must decode
//! originals exactly once in the same protocol/phase, forbid recursive splitting,
//! and await handling before accepting configuration completion. This module
//! grants no channel authority; Loader does not advertise neoforge:split yet.
use crate::{Error, Result};

pub const CHANNEL: &str = "neoforge:split";
pub const MAX_PART_BYTES: usize = 1_048_576;
pub const MAX_PACKET_BYTES: usize = 8 * 1_048_576;
pub const MAX_PARTS: usize = 256;
pub const MAX_ASSEMBLY_MILLIS: u64 = 30_000;

/// First/middle/last markers are separate values, not combinable flags.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Marker {
    First,
    Middle,
    Last,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Part<'a> {
    pub marker: Marker,
    pub content: &'a [u8],
}
impl<'a> Part<'a> {
    /// Decode the VarInt-prefixed byte[] payload, including its one-byte marker.
    /// Separate from the common 32767-byte body policy: split transport frames
    /// need a larger, still bounded budget. No allocation occurs here.
    pub fn decode(body: &'a [u8]) -> Result<Self> {
        if body.len() > MAX_PART_BYTES + 6 {
            return Err(Error::Limit("split frame"));
        }
        let mut length = 0u32;
        let mut prefix = 0;
        for i in 0..5 {
            let b = *body.get(i).ok_or(Error::Truncated)?;
            if i == 4 && b & 0xf0 != 0 {
                return Err(Error::VarInt);
            }
            length |= u32::from(b & 0x7f) << (7 * i);
            if b & 0x80 == 0 {
                prefix = i + 1;
                break;
            }
        }
        if prefix == 0 {
            return Err(Error::VarInt);
        }
        if length < 2 {
            return Err(Error::Invalid("empty split fragment"));
        }
        if length as usize > MAX_PART_BYTES + 1 {
            return Err(Error::Limit("split part"));
        }
        let payload = body
            .get(prefix..prefix + length as usize)
            .ok_or(Error::Truncated)?;
        if prefix + length as usize != body.len() {
            return Err(Error::Trailing);
        }
        let marker = match payload[0] {
            1 => Marker::First,
            0 => Marker::Middle,
            2 => Marker::Last,
            _ => return Err(Error::Invalid("split marker")),
        };
        Ok(Self {
            marker,
            content: &payload[1..],
        })
    }
}

/// One inbound connection, direction and protocol phase. An error poisons the
/// instance and releases partial data; only constructing a NEW instance begins
/// another epoch. `now` must be host monotonic elapsed milliseconds (not wall time).
/// The host MUST call `check_deadline` from its timer even when no traffic arrives.
#[derive(Debug, Default)]
pub struct Reassembler {
    buffer: Vec<u8>,
    parts: usize,
    started: Option<u64>,
    last_now: Option<u64>,
    failed: bool,
}
impl Reassembler {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn pending(&self) -> bool {
        self.started.is_some()
    }
    pub fn failed(&self) -> bool {
        self.failed
    }
    fn poison(&mut self) {
        self.buffer = Vec::new();
        self.parts = 0;
        self.started = None;
        self.failed = true;
    }
    /// A timeout or backwards clock is terminal, including on otherwise idle input.
    pub fn check_deadline(&mut self, now: u64) -> Result<()> {
        let result = self.check_time(now);
        if result.is_err() {
            self.poison();
        }
        result
    }
    fn check_time(&mut self, now: u64) -> Result<()> {
        if self.failed {
            return Err(Error::State("split epoch failed"));
        }
        if self.last_now.is_some_and(|last| now < last) {
            return Err(Error::State("split clock moved backwards"));
        }
        self.last_now = Some(now);
        if self
            .started
            .is_some_and(|start| now - start >= MAX_ASSEMBLY_MILLIS)
        {
            return Err(Error::State("split assembly timeout"));
        }
        Ok(())
    }
    pub fn receive(&mut self, body: &[u8], now: u64) -> Result<Option<Vec<u8>>> {
        let result = self.receive_inner(body, now);
        if result.is_err() {
            self.poison();
        }
        result
    }
    fn receive_inner(&mut self, body: &[u8], now: u64) -> Result<Option<Vec<u8>>> {
        self.check_time(now)?;
        let part = Part::decode(body)?;
        match (self.pending(), part.marker) {
            (false, Marker::First) => self.started = Some(now),
            (true, Marker::Middle | Marker::Last) => (),
            _ => return Err(Error::State("split fragment ordering")),
        }
        self.parts += 1;
        if self.parts > MAX_PARTS {
            return Err(Error::Limit("split parts"));
        }
        if self
            .buffer
            .len()
            .checked_add(part.content.len())
            .is_none_or(|n| n > MAX_PACKET_BYTES)
        {
            return Err(Error::Limit("split packet"));
        }
        self.buffer.extend_from_slice(part.content);
        if part.marker == Marker::Last {
            self.parts = 0;
            self.started = None;
            Ok(Some(std::mem::take(&mut self.buffer)))
        } else {
            Ok(None)
        }
    }
    /// Phase change/reconfigure/disconnect invalidates even a completed epoch.
    pub fn close(&mut self) {
        self.poison();
    }
}

/// Conservative outbound payload-body ceilings, INCLUDING the array VarInt and
/// marker, excluding the channel identifier and outer packet ID. These match the
/// future Horizon opaque configuration-payload path; NeoForge's registered split
/// codec itself uses an unbounded byte array. They are NOT negotiation authority.
pub const SERVERBOUND_BODY_BYTES: usize = 32_767;
pub const CLIENTBOUND_BODY_BYTES: usize = 1_048_576;

// GenericPacketSplitter.determineMaxPayloadSize: 1 packet-ID byte, 15 bytes
// for the length-prefixed "neoforge:split", 1 marker, 5 VarInt bytes, 4 bytes.
// Retain its conservative reservation, NOT a new wire header. With our bounds,
// even a FIVE-byte outer packet ID fits: 5 + 15 + 3 + 1 = 24 <= 26.
const SPLIT_ENVELOPE_RESERVATION: usize = 26;
const MAX_OUTBOUND_BYTES: usize = MAX_PACKET_BYTES + MAX_PARTS * 6;

/// An unsplit original is borrowed unchanged, never encoded as a single First
/// (or invented First|Last) fragment. Split values are complete payload BODIES:
/// VarInt(array length), marker, content. They are not Minecraft socket frames.
#[derive(Debug, PartialEq, Eq)]
pub enum OutboundPacket<'a> {
    Unsplit(&'a [u8]),
    Split(Vec<Vec<u8>>),
}

/// Concrete bounded encoder for configuration originals, not an enabled channel
/// or a host writer. Construct per host protocol/flow/size-limit epoch. The host
/// supplies the original-packet threshold from its real compression/framing
/// path; no guessed compressor constants or default transport profile are used.
#[derive(Debug)]
pub struct OutboundSplitter {
    packet_limit: usize,
    part_content: usize,
}
impl OutboundSplitter {
    pub fn new(
        flow: crate::modded::Flow,
        protocol: crate::modded::ConnectionProtocol,
        packet_limit: usize,
    ) -> Result<Self> {
        if protocol != crate::modded::ConnectionProtocol::Configuration {
            return Err(Error::State("outbound split requires configuration"));
        }
        if !(SPLIT_ENVELOPE_RESERVATION + 1..=MAX_PACKET_BYTES).contains(&packet_limit) {
            return Err(Error::Limit("split packet threshold"));
        }
        let body_limit = match flow {
            crate::modded::Flow::Serverbound => SERVERBOUND_BODY_BYTES,
            crate::modded::Flow::Clientbound => CLIENTBOUND_BODY_BYTES,
        };
        let mut part_content = (packet_limit - SPLIT_ENVELOPE_RESERVATION)
            .min(MAX_PART_BYTES)
            .min(body_limit);
        // At most four decrements: marker plus the length VarInt. Calculate
        // the actual body size, not just the content or the byte-array size.
        while part_content + 1 + varint_bytes(part_content + 1) > body_limit {
            part_content -= 1;
        }
        Ok(Self {
            packet_limit,
            part_content,
        })
    }

    /// `original` is the full encoded wire packet ID + body, BEFORE framing,
    /// compression or encryption. No IDs are translated here. The caller must
    /// bind bytes to the actual configuration codec, reject recursive split and
    /// phase-changing/unsupported packets, and possess negotiated authority.
    /// Only the nonnegative packet-ID VarInt is structurally validated here.
    ///
    /// All size/count arithmetic is preflighted before output allocation. Every
    /// allocation is fallible and every error returns no output; there is no
    /// callback, partial iterator or socket side effect. Queue the resulting
    /// batch atomically under a host budget, or discard it in full on failure.
    pub fn encode<'a>(&self, original: &'a [u8]) -> Result<OutboundPacket<'a>> {
        if original.len() > MAX_PACKET_BYTES {
            return Err(Error::Limit("split packet"));
        }
        validate_original_id(original)?;
        if original.len() <= self.packet_limit {
            return Ok(OutboundPacket::Unsplit(original));
        }
        let parts = original.len().div_ceil(self.part_content);
        if parts > MAX_PARTS {
            return Err(Error::Limit("split parts"));
        }
        // Mirrors GenericPacketSplitter's explicit one-part pass-through. Our
        // conservative part cap normally makes this unreachable, not a license
        // to synthesize a one-part First/Last representation.
        if parts == 1 {
            return Ok(OutboundPacket::Unsplit(original));
        }
        let overhead = parts
            .checked_mul(1 + varint_bytes(self.part_content + 1))
            .ok_or(Error::Limit("split allocation"))?;
        if original
            .len()
            .checked_add(overhead)
            .is_none_or(|n| n > MAX_OUTBOUND_BYTES)
        {
            return Err(Error::Limit("split allocation"));
        }
        let mut bodies = Vec::new();
        bodies
            .try_reserve_exact(parts)
            .map_err(|_| Error::Limit("split allocation"))?;
        for (index, content) in original.chunks(self.part_content).enumerate() {
            let length = content.len() + 1;
            let mut body = Vec::new();
            body.try_reserve_exact(varint_bytes(length) + length)
                .map_err(|_| Error::Limit("split allocation"))?;
            let mut n = length;
            loop {
                let byte = (n & 127) as u8;
                n >>= 7;
                body.push(byte | if n == 0 { 0 } else { 128 });
                if n == 0 {
                    break;
                }
            }
            body.push(if index == 0 {
                1
            } else if index == parts - 1 {
                2
            } else {
                0
            });
            body.extend_from_slice(content);
            bodies.push(body);
        }
        Ok(OutboundPacket::Split(bodies))
    }
}

fn varint_bytes(mut n: usize) -> usize {
    let mut count = 1;
    while n >= 128 {
        count += 1;
        n >>= 7;
    }
    count
}

fn validate_original_id(original: &[u8]) -> Result<()> {
    if original.is_empty() {
        return Err(Error::Invalid("empty split original"));
    }
    for i in 0..5 {
        let byte = *original.get(i).ok_or(Error::Truncated)?;
        if i == 4 && byte & 0xf8 != 0 {
            return Err(Error::VarInt);
        }
        if byte & 0x80 == 0 {
            return Ok(());
        }
    }
    Err(Error::VarInt)
}
