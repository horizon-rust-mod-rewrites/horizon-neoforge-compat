//! Handler-backed modded channel selection. This is NOT a configuration task
//! executor: a valid channel setup alone never authorizes entering play.
use crate::{
    codec::{Id, Phase, MAX_BODY, MAX_CHANNELS, MAX_STRING},
    Error, Result,
};
use std::collections::{BTreeMap, BTreeSet};

/// Packet direction is relative to the server, not relative to this endpoint.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Clientbound,
    Serverbound,
    Bidirectional,
}
pub type BodyValidator = fn(&[u8]) -> Result<()>;

/// Each declared direction needs a real implementation. For serverbound-only
/// channels the validator must check a fully encoded outbound body, not send it.
/// Stateful game semantics and asynchronous task barriers belong to the host.
pub struct Registration {
    pub phase: Phase,
    pub id: Id,
    pub version: String,
    pub direction: Direction,
    pub optional: bool,
    pub max_body: usize,
    pub receive: Option<BodyValidator>,
    pub validate_send: Option<BodyValidator>,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Offer {
    pub phase: Phase,
    pub id: Id,
    pub version: String,
    pub direction: Direction,
    pub optional: bool,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Selection {
    pub phase: Phase,
    pub key: Id,
    pub id: Id,
    pub version: String,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State {
    AwaitQuery,
    AwaitSetup,
    Configuring,
    Failed,
    Closed,
}

pub struct Negotiation {
    state: State,
    registrations: BTreeMap<(Phase, Id), Registration>,
    accepted: BTreeSet<(Phase, Id)>,
}
impl Negotiation {
    /// Construct a fresh connection epoch. Closed/failed instances cannot restart.
    pub fn new(registrations: Vec<Registration>) -> Result<Self> {
        if registrations.len() > MAX_CHANNELS {
            return Err(Error::Limit("modded registrations"));
        }
        let mut entries = BTreeMap::new();
        for registration in registrations {
            if registration.version.is_empty() || registration.version.len() > MAX_STRING {
                return Err(Error::Invalid("channel version"));
            }
            if registration.max_body > MAX_BODY {
                return Err(Error::Limit("channel body"));
            }
            let callbacks = (
                registration.receive.is_some(),
                registration.validate_send.is_some(),
            );
            let expected = match registration.direction {
                Direction::Clientbound => (true, false),
                Direction::Serverbound => (false, true),
                Direction::Bidirectional => (true, true),
            };
            if callbacks != expected {
                return Err(Error::Invalid("direction requires exact handler coverage"));
            }
            // Bootstrap channels must never be replaceable by adapter callbacks.
            if matches!(
                registration.id.as_str(),
                "neoforge:register"
                    | "neoforge:network"
                    | "neoforge:modded_network_setup_failed"
                    | "minecraft:register"
                    | "minecraft:unregister"
                    | "c:version"
                    | "c:register"
            ) {
                return Err(Error::Invalid("reserved bootstrap channel"));
            }
            let key = (registration.phase, registration.id.clone());
            if entries.insert(key.clone(), registration).is_some() {
                return Err(Error::Duplicate(key.1.to_string()));
            }
        }
        Ok(Self {
            state: State::AwaitQuery,
            registrations: entries,
            accepted: BTreeSet::new(),
        })
    }
    pub fn state(&self) -> State {
        self.state
    }
    pub fn accepted(&self) -> &BTreeSet<(Phase, Id)> {
        &self.accepted
    }
    fn poison<T>(&mut self, result: Result<T>) -> Result<T> {
        if result.is_err() {
            self.state = State::Failed;
            self.accepted.clear();
        }
        result
    }
    /// Call only after the complete server query body was decoded. The server's
    /// baseline query is empty; it is a request for OUR offers, not server offers.
    /// No local negotiation against the query's empty map is performed.
    pub fn on_query(&mut self) -> Result<Vec<Offer>> {
        let result = if self.state != State::AwaitQuery {
            Err(Error::State("duplicate or late modded query"))
        } else {
            self.state = State::AwaitSetup;
            Ok(self
                .registrations
                .values()
                .map(|r| Offer {
                    phase: r.phase,
                    id: r.id.clone(),
                    version: r.version.clone(),
                    direction: r.direction,
                    optional: r.optional,
                })
                .collect())
        };
        self.poison(result)
    }
    /// Validate the server-selected set atomically. The setup wire format has no
    /// direction: directions come from our handler-backed offer, never registration
    /// announcements. Missing optional channels are disabled, not dispatched.
    pub fn on_setup(&mut self, selections: Vec<Selection>) -> Result<()> {
        let result = self.setup_inner(selections);
        self.poison(result)
    }
    fn setup_inner(&mut self, selections: Vec<Selection>) -> Result<()> {
        if self.state != State::AwaitSetup {
            return Err(Error::State("setup without query or repeated"));
        }
        if selections.len() > MAX_CHANNELS {
            return Err(Error::Limit("selected channels"));
        }
        let mut next = BTreeSet::new();
        for selected in selections {
            if selected.key != selected.id {
                return Err(Error::Invalid("setup key and channel ID disagree"));
            }
            let key = (selected.phase, selected.id);
            let registration = self.registrations.get(&key).ok_or_else(|| {
                Error::Missing(format!("undeclared channel {} {}", key.0.as_str(), key.1))
            })?;
            if selected.version != registration.version {
                return Err(Error::Invalid("selected channel version mismatch"));
            }
            if !next.insert(key.clone()) {
                return Err(Error::Duplicate(key.1.to_string()));
            }
        }
        for (key, registration) in &self.registrations {
            if !registration.optional && !next.contains(key) {
                return Err(Error::Missing(format!(
                    "required channel {} {}",
                    key.0.as_str(),
                    key.1
                )));
            }
        }
        self.accepted = next;
        self.state = State::Configuring;
        Ok(())
    }
    /// Ingress to already-negotiated configuration handlers only. This does not
    /// implement task completion and deliberately cannot dispatch play packets.
    pub fn receive_configuration(&mut self, id: &Id, body: &[u8]) -> Result<()> {
        let result = self.dispatch(id, body, true);
        self.poison(result)
    }
    pub fn validate_configuration_send(&mut self, id: &Id, body: &[u8]) -> Result<()> {
        let result = self.dispatch(id, body, false);
        self.poison(result)
    }
    fn dispatch(&self, id: &Id, body: &[u8], incoming: bool) -> Result<()> {
        if self.state != State::Configuring {
            return Err(Error::State("channel setup not active"));
        }
        let key = (Phase::Configuration, id.clone());
        if !self.accepted.contains(&key) {
            return Err(Error::UnsupportedTask(id.to_string()));
        }
        let r = &self.registrations[&key];
        if body.len() > r.max_body {
            return Err(Error::Limit("channel body"));
        }
        let callback = if incoming { r.receive } else { r.validate_send };
        callback.ok_or(Error::Invalid("channel direction"))?(body)
    }
    /// No build of this channel selector can authorize play by itself.
    pub fn finish_configuration(&mut self) -> Result<()> {
        let result = Err(Error::UnsupportedTask(
            "host task/semantic-registry barrier is not integrated".into(),
        ));
        self.poison(result)
    }
    /// Terminal failure discards active capabilities; the host must disconnect.
    pub fn fail(&mut self) {
        self.accepted.clear();
        self.state = State::Failed;
    }
    /// Disconnect, transfer and reconfigure must drop this epoch. Construct a new
    /// instance (including fresh host state) instead of reusing registrations here.
    pub fn close(&mut self) {
        self.accepted.clear();
        self.registrations.clear();
        self.state = State::Closed;
    }
}

/// Reference rule for preflight/differential tests. Actual selection belongs to
/// the server. Optionality permits absence ONLY; it never excuses a mismatched
/// version or direction when both endpoints declare the same channel.
pub fn compatible(server: &Offer, client: &Offer) -> bool {
    server.phase == client.phase
        && server.id == client.id
        && server.version == client.version
        && server.direction == client.direction
}
