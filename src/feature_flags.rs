//! Exact-baseline feature-flag equality task, not feature implementation.
//! Local inventory must come from the actual host, never copied from the peer.
//! No global defaults, play permission, feature-ID allocation or game semantics.
use crate::{
    codec::{Id, Reader, Writer, MAX_CHANNELS},
    Error, Result,
};
use std::collections::BTreeSet;
pub const DATA_CHANNEL: &str = "neoforge:feature_flags";
pub const ACK_CHANNEL: &str = "neoforge:feature_flags_ack";
pub const MAX_FLAGS: usize = MAX_CHANNELS;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FeatureFlagData {
    pub modded_flags: BTreeSet<Id>,
}
impl FeatureFlagData {
    pub fn decode(bytes: &[u8]) -> Result<Self> {
        let mut r = Reader::new(bytes)?;
        let count = r.count(MAX_FLAGS)?;
        let mut modded_flags = BTreeSet::new();
        for _ in 0..count {
            let id = r.id()?;
            if !modded_flags.insert(id.clone()) {
                return Err(Error::Duplicate(id.to_string()));
            }
        }
        r.finish()?;
        Ok(Self { modded_flags })
    }
    pub fn encode(&self) -> Result<Vec<u8>> {
        let mut w = Writer::new();
        w.count(self.modded_flags.len(), MAX_FLAGS)?;
        for id in &self.modded_flags {
            w.id(id)?;
        }
        Ok(w.finish())
    }
}
/// Unit codec under strict complete-payload framing. Not an acknowledgment sender.
pub fn validate_ack(bytes: &[u8]) -> Result<()> {
    Reader::new(bytes)?.finish()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State {
    AwaitData,
    Checked,
    Failed,
    Closed,
}
/// A successful equality comparison permits only this task's empty acknowledgment.
/// The host attests that its complete modded flag inventory is semantically real.
/// Names alone do not implement flags; an empty set is an EXPLICIT host assertion,
/// not an inferred fallback. Discard task on epoch end and on failed ack transport.
pub struct FeatureFlagTask {
    local: BTreeSet<Id>,
    state: State,
}
impl FeatureFlagTask {
    pub fn new(local_modded_flags: BTreeSet<Id>) -> Result<Self> {
        // Preflight both count and aggregate body bounds before any offer.
        FeatureFlagData {
            modded_flags: local_modded_flags.clone(),
        }
        .encode()?;
        Ok(Self {
            local: local_modded_flags,
            state: State::AwaitData,
        })
    }
    pub fn state(&self) -> State {
        self.state
    }
    /// Called ONLY after server-selected inbound and outbound channels are active.
    /// Checks equality in both directions; a remote subset is not sufficient.
    /// Errors are terminal and clear inventory. Duplicate delivery is refused.
    pub fn receive(&mut self, body: &[u8]) -> Result<Vec<u8>> {
        if self.state != State::AwaitData {
            self.fail();
            return Err(Error::State("feature flag task ordering"));
        }
        self.state = State::Failed;
        match FeatureFlagData::decode(body) {
            Ok(remote) if remote.modded_flags == self.local => {
                self.state = State::Checked;
                Ok(Vec::new())
            }
            Ok(_) => {
                self.fail();
                Err(Error::Invalid("modded feature flag inventory mismatch"))
            }
            Err(error) => {
                self.fail();
                Err(error)
            }
        }
    }
    pub fn fail(&mut self) {
        self.local.clear();
        self.state = State::Failed;
    }
    pub fn close(&mut self) {
        self.local.clear();
        self.state = State::Closed;
    }
}
