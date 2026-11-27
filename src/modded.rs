//! Bounded modded query/setup payload bodies for NeoForge 21.1.248 / MC 1.21.1.
//!
//! No packet framing, negotiation decisions, handler installation, or task acknowledgement.
//! See provenance/modded-codecs.md for evidence and intentionally stricter local policy.
//! Setup-failure Components use NBT, NOT strings. See crate::setup_failure for
//! bounded lossless NBT data (not full Component semantics). The legacy
//! [`reject_setup_failure`] helper remains a conservative terminal rejection.
use crate::codec::{Id, Phase, Reader, Writer, MAX_CHANNELS, MAX_STRING};
use crate::{Error, Result};
use std::collections::BTreeMap;

pub const QUERY_CHANNEL: &str = "neoforge:register";
pub const SETUP_CHANNEL: &str = "neoforge:network";
pub const FAILURE_CHANNEL: &str = "neoforge:modded_network_setup_failed";
/// Total entries across ALL phases, not a separate budget per phase.
pub const MAX_TOTAL_COMPONENTS: usize = MAX_CHANNELS;
pub const MAX_PROTOCOLS: usize = 5;
pub const MAX_FAILURE_REASONS: usize = MAX_CHANNELS;

/// Exact Minecraft 1.21.1 enum ordinals, NOT the protocol's textual IDs or packet IDs.
/// All five wire values are represented; a controller must separately restrict phases.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ConnectionProtocol {
    Handshaking,
    Play,
    Status,
    Login,
    Configuration,
}
impl ConnectionProtocol {
    pub fn ordinal(self) -> i32 {
        match self {
            Self::Handshaking => 0,
            Self::Play => 1,
            Self::Status => 2,
            Self::Login => 3,
            Self::Configuration => 4,
        }
    }
    pub fn from_ordinal(value: i32) -> Result<Self> {
        match value {
            0 => Ok(Self::Handshaking),
            1 => Ok(Self::Play),
            2 => Ok(Self::Status),
            3 => Ok(Self::Login),
            4 => Ok(Self::Configuration),
            _ => Err(Error::Invalid("ConnectionProtocol ordinal")),
        }
    }
}
impl From<Phase> for ConnectionProtocol {
    fn from(value: Phase) -> Self {
        match value {
            Phase::Configuration => Self::Configuration,
            Phase::Play => Self::Play,
        }
    }
}
impl TryFrom<ConnectionProtocol> for Phase {
    type Error = Error;
    fn try_from(value: ConnectionProtocol) -> Result<Self> {
        match value {
            ConnectionProtocol::Configuration => Ok(Self::Configuration),
            ConnectionProtocol::Play => Ok(Self::Play),
            _ => Err(Error::Invalid("non-negotiable modded phase")),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Flow {
    Serverbound,
    Clientbound,
}
impl Flow {
    pub fn ordinal(self) -> i32 {
        match self {
            Self::Serverbound => 0,
            Self::Clientbound => 1,
        }
    }
    pub fn from_ordinal(value: i32) -> Result<Self> {
        match value {
            0 => Ok(Self::Serverbound),
            1 => Ok(Self::Clientbound),
            _ => Err(Error::Invalid("PacketFlow ordinal")),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QueryComponent {
    pub id: Id,
    /// Required string; empty is a wire value, not an absent version.
    pub version: String,
    /// None means bidirectional, NOT unknown or unsupported.
    pub flow: Option<Flow>,
    pub optional: bool,
}

/// A wire set is indexed by ID locally. Duplicate IDs (even unequal records) fail.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ModdedQuery {
    pub phases: BTreeMap<ConnectionProtocol, BTreeMap<Id, QueryComponent>>,
}

fn read_bool(r: &mut Reader<'_>) -> Result<bool> {
    match r.byte()? {
        0 => Ok(false),
        1 => Ok(true),
        _ => Err(Error::Invalid("noncanonical boolean")),
    }
}
fn add_entries(total: &mut usize, count: usize) -> Result<()> {
    *total = total
        .checked_add(count)
        .ok_or(Error::Limit("total modded components"))?;
    if *total > MAX_TOTAL_COMPONENTS {
        return Err(Error::Limit("total modded components"));
    }
    Ok(())
}
fn read_phase(r: &mut Reader<'_>) -> Result<ConnectionProtocol> {
    ConnectionProtocol::from_ordinal(r.varint()?)
}
impl ModdedQuery {
    pub fn decode(bytes: &[u8]) -> Result<Self> {
        let mut r = Reader::new(bytes)?;
        let count = r.count(MAX_PROTOCOLS)?;
        let mut phases = BTreeMap::new();
        let mut total = 0;
        for _ in 0..count {
            let phase = read_phase(&mut r)?;
            if phases.contains_key(&phase) {
                return Err(Error::Duplicate(format!("query phase {phase:?}")));
            }
            let count = r.count(MAX_TOTAL_COMPONENTS)?;
            add_entries(&mut total, count)?;
            let mut components = BTreeMap::new();
            for _ in 0..count {
                let id = r.id()?;
                if components.contains_key(&id) {
                    return Err(Error::Duplicate(id.to_string()));
                }
                let version = r.string(MAX_STRING)?;
                let flow = if read_bool(&mut r)? {
                    Some(Flow::from_ordinal(r.varint()?)?)
                } else {
                    None
                };
                let optional = read_bool(&mut r)?;
                components.insert(
                    id.clone(),
                    QueryComponent {
                        id,
                        version,
                        flow,
                        optional,
                    },
                );
            }
            phases.insert(phase, components);
        }
        r.finish()?;
        Ok(Self { phases })
    }
    pub fn encode(&self) -> Result<Vec<u8>> {
        let mut w = Writer::new();
        w.count(self.phases.len(), MAX_PROTOCOLS)?;
        let mut total = 0;
        for (phase, components) in &self.phases {
            add_entries(&mut total, components.len())?;
            w.varint(phase.ordinal())?;
            w.count(components.len(), MAX_TOTAL_COMPONENTS)?;
            for (key, component) in components {
                if key != &component.id {
                    return Err(Error::Invalid("query key/component ID mismatch"));
                }
                w.id(&component.id)?;
                w.string(&component.version, MAX_STRING)?;
                w.byte(u8::from(component.flow.is_some()))?;
                if let Some(flow) = component.flow {
                    w.varint(flow.ordinal())?;
                }
                w.byte(u8::from(component.optional))?;
            }
        }
        Ok(w.finish())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NetworkChannel {
    /// Redundant with the enclosing map key on the wire. A mismatch fails locally.
    pub id: Id,
    pub chosen_version: String,
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ModdedSetup {
    pub phases: BTreeMap<ConnectionProtocol, BTreeMap<Id, NetworkChannel>>,
}
impl ModdedSetup {
    pub fn decode(bytes: &[u8]) -> Result<Self> {
        let mut r = Reader::new(bytes)?;
        let count = r.count(MAX_PROTOCOLS)?;
        let mut phases = BTreeMap::new();
        let mut total = 0;
        for _ in 0..count {
            let phase = read_phase(&mut r)?;
            if phases.contains_key(&phase) {
                return Err(Error::Duplicate(format!("setup phase {phase:?}")));
            }
            let count = r.count(MAX_TOTAL_COMPONENTS)?;
            add_entries(&mut total, count)?;
            let mut channels = BTreeMap::new();
            for _ in 0..count {
                let key = r.id()?;
                if channels.contains_key(&key) {
                    return Err(Error::Duplicate(key.to_string()));
                }
                let id = r.id()?;
                if id != key {
                    return Err(Error::Invalid("setup key/channel ID mismatch"));
                }
                let chosen_version = r.string(MAX_STRING)?;
                channels.insert(key, NetworkChannel { id, chosen_version });
            }
            phases.insert(phase, channels);
        }
        r.finish()?;
        Ok(Self { phases })
    }
    pub fn encode(&self) -> Result<Vec<u8>> {
        let mut w = Writer::new();
        w.count(self.phases.len(), MAX_PROTOCOLS)?;
        let mut total = 0;
        for (phase, channels) in &self.phases {
            add_entries(&mut total, channels.len())?;
            w.varint(phase.ordinal())?;
            w.count(channels.len(), MAX_TOTAL_COMPONENTS)?;
            for (key, channel) in channels {
                if key != &channel.id {
                    return Err(Error::Invalid("setup key/channel ID mismatch"));
                }
                w.id(key)?;
                w.id(&channel.id)?;
                w.string(&channel.chosen_version, MAX_STRING)?;
            }
        }
        Ok(w.finish())
    }
}

/// Terminal rejection for an incoming setup-failure payload. This is NOT a Component
/// decoder. Exact upstream layout: map count VarInt, then repeated ResourceLocation
/// and TRUSTED_CONTEXT_FREE_STREAM_CODEC Component (network NBT, not JSON/UTF-8).
/// Bounded header/first key are checked for diagnostics, then nonempty maps fail
/// unsupported WITHOUT consuming or trusting Component bytes. Even an empty failure
/// map is terminal. No Ok path exists; callers must poison/disconnect their epoch.
/// In particular, do not use this function as a successful wire-validation oracle.
pub fn reject_setup_failure(bytes: &[u8]) -> Result<()> {
    let mut r = Reader::new(bytes)?;
    let count = r.count(MAX_FAILURE_REASONS)?;
    if count == 0 {
        r.finish()?;
    } else {
        r.id()?;
    }
    Err(Error::UnsupportedTask(format!(
        "{FAILURE_CHANNEL}: terminal setup failure; NBT Component decoding not implemented"
    )))
}
