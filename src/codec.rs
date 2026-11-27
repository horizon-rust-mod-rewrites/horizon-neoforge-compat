//! NeoForge sources are indexed in docs/WIRE.md. Bounds are LOCAL safety policy,
//! deliberately stricter than upstream; rejecting input is never permission to skip a task.
use crate::{Error, Result};
use std::collections::BTreeSet;

pub const MAX_BODY: usize = 32_767; // Conservative, fits serverbound common custom payloads.
pub const MAX_CHANNELS: usize = 1_024;
pub const MAX_ID: usize = 256;
pub const MAX_STRING: usize = 1_024;
pub const MAX_VERSIONS: usize = 32;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Id(String);
impl Id {
    /// ResourceLocation shorthand is normalized to minecraft:. ASCII grammar only.
    pub fn parse(s: &str) -> Result<Self> {
        if s.len() > MAX_ID {
            return Err(Error::Limit("identifier"));
        }
        let (namespace, path) = s.split_once(':').unwrap_or(("minecraft", s));
        let namespace = if namespace.is_empty() {
            "minecraft"
        } else {
            namespace
        };
        fn base(b: u8) -> bool {
            b.is_ascii_lowercase() || b.is_ascii_digit() || b"_.-".contains(&b)
        }
        if path.is_empty()
            || !namespace.bytes().all(base)
            || !path.bytes().all(|b| base(b) || b == b'/')
        {
            return Err(Error::Identifier(s.into()));
        }
        let normalized = format!("{namespace}:{path}");
        if normalized.len() > MAX_ID {
            return Err(Error::Limit("identifier"));
        }
        Ok(Self(normalized))
    }
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
impl std::fmt::Display for Id {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

pub(crate) struct Reader<'a> {
    bytes: &'a [u8],
    pos: usize,
}
impl<'a> Reader<'a> {
    pub(crate) fn new(bytes: &'a [u8]) -> Result<Self> {
        if bytes.len() > MAX_BODY {
            return Err(Error::Limit("body"));
        }
        Ok(Self { bytes, pos: 0 })
    }
    pub(crate) fn byte(&mut self) -> Result<u8> {
        let b = *self.bytes.get(self.pos).ok_or(Error::Truncated)?;
        self.pos += 1;
        Ok(b)
    }
    pub(crate) fn varint(&mut self) -> Result<i32> {
        let mut out = 0u32;
        for n in 0..5 {
            let b = self.byte()?;
            if n == 4 && b & 0xf0 != 0 {
                return Err(Error::VarInt);
            }
            out |= ((b & 0x7f) as u32) << (7 * n);
            if b & 0x80 == 0 {
                return Ok(out as i32);
            }
        }
        Err(Error::VarInt)
    }
    pub(crate) fn count(&mut self, max: usize) -> Result<usize> {
        let n = self.varint()?;
        if n < 0 || n as usize > max {
            return Err(Error::Limit("count"));
        }
        Ok(n as usize)
    }
    pub(crate) fn string(&mut self, max: usize) -> Result<String> {
        let n = self.count(max)?;
        let end = self.pos.checked_add(n).ok_or(Error::Limit("string"))?;
        let bytes = self.bytes.get(self.pos..end).ok_or(Error::Truncated)?;
        let s = std::str::from_utf8(bytes).map_err(|_| Error::Utf8)?;
        self.pos = end;
        Ok(s.to_owned())
    }
    pub(crate) fn id(&mut self) -> Result<Id> {
        Id::parse(&self.string(MAX_ID)?)
    }
    pub(crate) fn finish(self) -> Result<()> {
        if self.pos != self.bytes.len() {
            Err(Error::Trailing)
        } else {
            Ok(())
        }
    }
}
pub(crate) struct Writer(Vec<u8>);
impl Writer {
    pub(crate) fn new() -> Self {
        Self(Vec::new())
    }
    pub(crate) fn byte(&mut self, b: u8) -> Result<()> {
        if self.0.len() == MAX_BODY {
            return Err(Error::Limit("body"));
        }
        self.0.push(b);
        Ok(())
    }
    pub(crate) fn varint(&mut self, n: i32) -> Result<()> {
        let mut n = n as u32;
        loop {
            let b = (n & 0x7f) as u8;
            n >>= 7;
            self.byte(if n == 0 { b } else { b | 0x80 })?;
            if n == 0 {
                return Ok(());
            }
        }
    }
    pub(crate) fn count(&mut self, n: usize, max: usize) -> Result<()> {
        if n > max {
            return Err(Error::Limit("count"));
        }
        self.varint(n as i32)
    }
    pub(crate) fn string(&mut self, s: &str, max: usize) -> Result<()> {
        self.count(s.len(), max)?;
        for b in s.bytes() {
            self.byte(b)?;
        }
        Ok(())
    }
    pub(crate) fn id(&mut self, id: &Id) -> Result<()> {
        self.string(id.as_str(), MAX_ID)
    }
    pub(crate) fn finish(self) -> Vec<u8> {
        self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommonVersion {
    pub versions: Vec<i32>,
}
impl CommonVersion {
    pub fn decode(bytes: &[u8]) -> Result<Self> {
        let mut r = Reader::new(bytes)?;
        let count = r.count(MAX_VERSIONS)?;
        let mut versions = Vec::with_capacity(count);
        for _ in 0..count {
            versions.push(r.varint()?);
        }
        r.finish()?;
        Ok(Self { versions })
    }
    pub fn encode(&self) -> Result<Vec<u8>> {
        let mut w = Writer::new();
        w.count(self.versions.len(), MAX_VERSIONS)?;
        for v in &self.versions {
            w.varint(*v)?;
        }
        Ok(w.finish())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Phase {
    Configuration,
    Play,
}
impl Phase {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Configuration => "configuration",
            Self::Play => "play",
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommonRegister {
    pub version: i32,
    pub phase: Phase,
    pub channels: BTreeSet<Id>,
}
impl CommonRegister {
    pub fn decode(bytes: &[u8]) -> Result<Self> {
        let mut r = Reader::new(bytes)?;
        let version = r.varint()?;
        let phase = match r.string(MAX_STRING)?.as_str() {
            "configuration" => Phase::Configuration,
            "play" => Phase::Play,
            _ => return Err(Error::Invalid("common register protocol")),
        };
        let count = r.count(MAX_CHANNELS)?;
        let mut channels = BTreeSet::new();
        for _ in 0..count {
            let id = r.id()?;
            if !channels.insert(id.clone()) {
                return Err(Error::Duplicate(id.to_string()));
            }
        }
        r.finish()?;
        Ok(Self {
            version,
            phase,
            channels,
        })
    }
    pub fn encode(&self) -> Result<Vec<u8>> {
        let mut w = Writer::new();
        w.varint(self.version)?;
        w.string(self.phase.as_str(), MAX_STRING)?;
        w.count(self.channels.len(), MAX_CHANNELS)?;
        for id in &self.channels {
            w.id(id)?;
        }
        Ok(w.finish())
    }
}

/// minecraft:register / minecraft:unregister: ASCII ResourceLocations separated by NUL.
/// Upstream ignores malformed entries. Here they fail CLOSED instead; empty entries
/// are ignored and duplicates deduplicated just like upstream. Missing final NUL is legal.
pub fn decode_dinnerbone(bytes: &[u8]) -> Result<BTreeSet<Id>> {
    Reader::new(bytes)?;
    let mut ids = BTreeSet::new();
    let mut entries = 0;
    for part in bytes.split(|b| *b == 0) {
        if part.is_empty() {
            continue;
        }
        entries += 1;
        if entries > MAX_CHANNELS {
            return Err(Error::Limit("channels"));
        }
        if part.len() > MAX_ID {
            return Err(Error::Limit("identifier"));
        }
        let s = std::str::from_utf8(part).map_err(|_| Error::Utf8)?;
        ids.insert(Id::parse(s)?);
    }
    Ok(ids)
}
pub fn encode_dinnerbone(ids: &BTreeSet<Id>) -> Result<Vec<u8>> {
    if ids.len() > MAX_CHANNELS {
        return Err(Error::Limit("channels"));
    }
    let mut w = Writer::new();
    for id in ids {
        for b in id.as_str().bytes() {
            w.byte(b)?;
        }
        w.byte(0)?;
    }
    Ok(w.finish())
}
