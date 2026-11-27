//! Bounded NeoForge 21.1.248 split-packet reassembly, before packet-ID adaptation.
//!
//! The output is an entire original Minecraft packet (including its packet ID),
//! NOT a custom-payload body. The host must decode it exactly once in the same
//! protocol/phase, forbid recursively split packets, and await handling before
//! accepting configuration completion. This module grants no channel authority;
//! the Loader deliberately does not advertise neoforge:split yet.
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
