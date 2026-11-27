//! Optional common-protocol receivers only. This is NOT a NeoForge PayloadRegistrar
//! replacement. Required channels/versions/flows need future modded-network negotiation.
use crate::{
    codec::{Id, Phase, MAX_BODY, MAX_CHANNELS},
    Error, Result,
};
use std::collections::{BTreeMap, BTreeSet};

pub type Handler = fn(&[u8]) -> Result<()>;
struct Receiver {
    max_body: usize,
    handler: Handler,
}
#[derive(Default)]
pub struct Extensions {
    receivers: BTreeMap<(Phase, Id), Receiver>,
}
impl Extensions {
    pub fn new() -> Self {
        Self::default()
    }
    /// Cannot advertise metadata without a callable receiver. Handlers must validate
    /// their entire body, use bounded work, and never acknowledge unimplemented tasks.
    pub fn register_optional_receiver(
        &mut self,
        phase: Phase,
        id: Id,
        max_body: usize,
        handler: Handler,
    ) -> Result<()> {
        let ns = id.as_str().split(':').next().unwrap_or("");
        if ["minecraft", "neoforge", "c", "horizon"].contains(&ns) {
            return Err(Error::Invalid("reserved namespace"));
        }
        if max_body > MAX_BODY {
            return Err(Error::Limit("extension body"));
        }
        let key = (phase, id);
        if self.receivers.contains_key(&key) {
            return Err(Error::Duplicate(key.1.to_string()));
        }
        if self.receivers.len() >= MAX_CHANNELS {
            return Err(Error::Limit("extensions"));
        }
        self.receivers.insert(key, Receiver { max_body, handler });
        Ok(())
    }
    pub fn channels(&self, phase: Phase) -> BTreeSet<Id> {
        self.receivers
            .keys()
            .filter(|(p, _)| *p == phase)
            .map(|(_, id)| id.clone())
            .collect()
    }
    pub fn dispatch(&self, phase: Phase, id: &Id, body: &[u8]) -> Result<()> {
        let receiver = self
            .receivers
            .get(&(phase, id.clone()))
            .ok_or_else(|| Error::UnsupportedTask(id.to_string()))?;
        if body.len() > receiver.max_body {
            return Err(Error::Limit("extension body"));
        }
        (receiver.handler)(body)
    }
}
