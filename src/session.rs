//! Configuration-only common-protocol submachine. Does NOT complete the enclosing
//! NeoForge handshake and deliberately has NO successful finish/join API.
use crate::{
    check_baseline,
    codec::{self, CommonRegister, CommonVersion, Id, Phase},
    extension::Extensions,
    Error, Result,
};
use std::collections::BTreeSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State {
    AwaitVersion,
    AwaitRegister,
    CommonTasksOnly,
    Failed,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Outbound {
    pub channel: Id,
    pub body: Vec<u8>,
}
pub struct Session {
    state: State,
    extensions: Extensions,
    peer_common: BTreeSet<Id>,
    peer_dinnerbone: BTreeSet<Id>,
}
impl Session {
    /// Loader identity is an out-of-band requirement: neither brand nor c:version
    /// proves a NeoForge build. The host must obtain the real server inventory.
    pub fn new(
        minecraft: &str,
        protocol: i32,
        loader: &str,
        extensions: Extensions,
    ) -> Result<Self> {
        check_baseline(minecraft, protocol, loader)?;
        Ok(Self {
            state: State::AwaitVersion,
            extensions,
            peer_common: BTreeSet::new(),
            peer_dinnerbone: BTreeSet::new(),
        })
    }
    pub fn state(&self) -> State {
        self.state
    }
    pub fn peer_common(&self) -> &BTreeSet<Id> {
        &self.peer_common
    }
    pub fn peer_dinnerbone(&self) -> &BTreeSet<Id> {
        &self.peer_dinnerbone
    }
    /// Safe declarations for this submachine, not a complete NeoForge initial set.
    /// Never send this list as evidence of full NeoForge capability.
    pub fn implemented_configuration_channels(&self) -> BTreeSet<Id> {
        let mut ids = self.extensions.channels(Phase::Configuration);
        for s in [
            "c:version",
            "c:register",
            "minecraft:register",
            "minecraft:unregister",
        ] {
            ids.insert(Id::parse(s).expect("constant valid identifier"));
        }
        ids
    }
    /// Any malformed, unknown, unsupported or out-of-order task poisons the epoch.
    /// Host must disconnect on Err; do not drop the error and keep configuring.
    pub fn receive(&mut self, channel: &str, bytes: &[u8]) -> Result<Vec<Outbound>> {
        if self.state == State::Failed {
            return Err(Error::State("epoch failed"));
        }
        let result = self.receive_inner(channel, bytes);
        if result.is_err() {
            self.state = State::Failed;
        }
        result
    }
    fn receive_inner(&mut self, channel: &str, bytes: &[u8]) -> Result<Vec<Outbound>> {
        if bytes.len() > codec::MAX_BODY {
            return Err(Error::Limit("body"));
        }
        let id = Id::parse(channel)?;
        let reply = |body| {
            Ok(vec![Outbound {
                channel: id.clone(),
                body,
            }])
        };
        match id.as_str() {
            "minecraft:register" => {
                let incoming = codec::decode_dinnerbone(bytes)?;
                let mut next = self.peer_dinnerbone.clone();
                next.extend(incoming);
                if next.len() > codec::MAX_CHANNELS {
                    return Err(Error::Limit("peer channels"));
                }
                self.peer_dinnerbone = next;
                Ok(vec![])
            }
            "minecraft:unregister" => {
                let incoming = codec::decode_dinnerbone(bytes)?;
                for id in incoming {
                    self.peer_dinnerbone.remove(&id);
                }
                Ok(vec![])
            }
            "c:version" => {
                if self.state != State::AwaitVersion {
                    return Err(Error::State("duplicate or late c:version"));
                }
                let version = CommonVersion::decode(bytes)?;
                if !version.versions.contains(&1) {
                    return Err(Error::Invalid("no common networking version 1"));
                }
                let body = CommonVersion { versions: vec![1] }.encode()?;
                self.state = State::AwaitRegister;
                reply(body)
            }
            "c:register" => {
                if self.state != State::AwaitRegister {
                    return Err(Error::State("c:register before version or repeated"));
                }
                let register = CommonRegister::decode(bytes)?;
                // NeoForge ignores register.version, but this milestone is stricter.
                if register.version != 1 || register.phase != Phase::Play {
                    return Err(Error::Invalid("only common register v1 play supported"));
                }
                let body = CommonRegister {
                    version: 1,
                    phase: Phase::Play,
                    channels: self.extensions.channels(Phase::Play),
                }
                .encode()?;
                self.peer_common = register.channels;
                self.state = State::CommonTasksOnly;
                reply(body)
            }
            _ => {
                if self.state != State::CommonTasksOnly {
                    return Err(Error::UnsupportedTask(id.to_string()));
                }
                self.extensions.dispatch(Phase::Configuration, &id, bytes)?;
                Ok(vec![])
            }
        }
    }
    /// No path can accidentally turn common-task success into a NeoForge join.
    pub fn finish_configuration(&mut self) -> Result<()> {
        self.state = State::Failed;
        Err(Error::UnsupportedTask("finish_configuration: full modded negotiation, registry application, data maps, enums and flags are not implemented".into()))
    }
}
