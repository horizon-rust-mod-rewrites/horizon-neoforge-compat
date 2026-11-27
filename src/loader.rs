//! Early, transport-independent NeoForge query/setup endpoint. No successful
//! finish path exists until host task execution and semantic content integration
//! are implemented. This does not replace the legacy common-only Session API.
use crate::{
    check_baseline,
    codec::{self, CommonRegister, CommonVersion, Id, Phase},
    modded::{self, Flow, ModdedQuery, ModdedSetup, QueryComponent},
    negotiation::{Direction, Negotiation, Offer, Registration, Selection, State},
    session::Outbound,
    setup_failure::SetupFailure,
    Error, Result,
};
use std::collections::{BTreeMap, BTreeSet};

const MAX_EPOCH_MESSAGES: usize = 8_192;
const MAX_EPOCH_BODY_BYTES: usize = 4 * 1_048_576;

pub struct Loader {
    negotiation: Negotiation,
    offers: Vec<Offer>,
    failed: bool,
    setup_failure: Option<SetupFailure>,
    common_version: bool,
    common_registered: bool,
    peer_common: BTreeSet<Id>,
    peer_dinnerbone: BTreeSet<Id>,
    messages: usize,
    bytes: usize,
}
impl Loader {
    pub fn new(
        minecraft: &str,
        protocol: i32,
        loader: &str,
        registrations: Vec<Registration>,
    ) -> Result<Self> {
        check_baseline(minecraft, protocol, loader)?;
        Ok(Self {
            negotiation: Negotiation::new(registrations)?,
            offers: Vec::new(),
            failed: false,
            setup_failure: None,
            common_version: false,
            common_registered: false,
            peer_common: BTreeSet::new(),
            peer_dinnerbone: BTreeSet::new(),
            messages: 0,
            bytes: 0,
        })
    }
    pub fn state(&self) -> State {
        if self.failed {
            State::Failed
        } else {
            self.negotiation.state()
        }
    }
    /// Untrusted, lossless NBT diagnostics from a completely decoded terminal
    /// setup failure. Not rendered or semantically validated Components. Retained
    /// after failure for inspection; close/new epoch discards them. Malformed
    /// payloads expose no partial data. This never confers channel authority.
    pub fn setup_failure(&self) -> Option<&SetupFailure> {
        self.setup_failure.as_ref()
    }
    pub fn accepted(&self) -> &BTreeSet<(Phase, Id)> {
        self.negotiation.accepted()
    }
    pub fn peer_common(&self) -> &BTreeSet<Id> {
        &self.peer_common
    }
    pub fn peer_dinnerbone(&self) -> &BTreeSet<Id> {
        &self.peer_dinnerbone
    }
    /// Whole payload body only. Every error is terminal. Outputs are a validated
    /// bounded batch; the host must disconnect if any queued send subsequently fails.
    pub fn receive(&mut self, channel: &str, bytes: &[u8]) -> Result<Vec<Outbound>> {
        if self.failed {
            return Err(Error::State("loader epoch failed"));
        }
        // Set before any adapter callback; a caught unwinding panic cannot retry.
        self.failed = true;
        let result = self.receive_inner(channel, bytes);
        if result.is_ok() {
            self.failed = false;
        } else {
            self.negotiation.fail();
            self.offers.clear();
            self.peer_common.clear();
            self.peer_dinnerbone.clear();
        }
        result
    }
    fn receive_inner(&mut self, channel: &str, bytes: &[u8]) -> Result<Vec<Outbound>> {
        if bytes.len() > codec::MAX_BODY {
            return Err(Error::Limit("body"));
        }
        self.messages = self
            .messages
            .checked_add(1)
            .filter(|n| *n <= MAX_EPOCH_MESSAGES)
            .ok_or(Error::Limit("epoch messages"))?;
        self.bytes = self
            .bytes
            .checked_add(bytes.len())
            .filter(|n| *n <= MAX_EPOCH_BODY_BYTES)
            .ok_or(Error::Limit("epoch bytes"))?;
        let channel = Id::parse(channel)?;
        match channel.as_str() {
            modded::QUERY_CHANNEL => {
                let query = ModdedQuery::decode(bytes)?;
                // All exact-baseline server queries are empty. Refuse other
                // shapes instead of accidentally treating them as server offers.
                if !query.phases.is_empty() {
                    return Err(Error::Invalid("nonempty baseline server query"));
                }
                self.offers = self.negotiation.on_query()?;
                let mut phases = BTreeMap::new();
                for offer in &self.offers {
                    let flow = match offer.direction {
                        Direction::Clientbound => Some(Flow::Clientbound),
                        Direction::Serverbound => Some(Flow::Serverbound),
                        Direction::Bidirectional => None,
                    };
                    phases
                        .entry(offer.phase.into())
                        .or_insert_with(BTreeMap::new)
                        .insert(
                            offer.id.clone(),
                            QueryComponent {
                                id: offer.id.clone(),
                                version: offer.version.clone(),
                                flow,
                                optional: offer.optional,
                            },
                        );
                }
                reply(channel, ModdedQuery { phases }.encode()?)
            }
            modded::SETUP_CHANNEL => {
                let setup = ModdedSetup::decode(bytes)?;
                let mut selected = Vec::new();
                for (phase, channels) in setup.phases {
                    let phase = Phase::try_from(phase)?;
                    for (key, ch) in channels {
                        selected.push(Selection {
                            phase,
                            key,
                            id: ch.id,
                            version: ch.chosen_version,
                        });
                    }
                }
                self.negotiation.on_setup(selected)?;
                // Declaration does not confer channel authority. Advertise only
                // builtins actually handled here, plus accepted incoming config.
                let mut listening = BTreeSet::new();
                for name in [
                    modded::QUERY_CHANNEL,
                    modded::SETUP_CHANNEL,
                    modded::FAILURE_CHANNEL,
                    "c:version",
                    "c:register",
                    "minecraft:register",
                    "minecraft:unregister",
                ] {
                    listening.insert(Id::parse(name)?);
                }
                for offer in &self.offers {
                    if offer.phase == Phase::Configuration
                        && offer.direction != Direction::Serverbound
                        && self
                            .negotiation
                            .accepted()
                            .contains(&(offer.phase, offer.id.clone()))
                    {
                        listening.insert(offer.id.clone());
                    }
                }
                reply(
                    Id::parse("minecraft:register")?,
                    codec::encode_dinnerbone(&listening)?,
                )
            }
            modded::FAILURE_CHANNEL => {
                self.setup_failure = Some(SetupFailure::decode(bytes)?);
                Err(Error::SetupFailed)
            }
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
                for id in codec::decode_dinnerbone(bytes)? {
                    self.peer_dinnerbone.remove(&id);
                }
                Ok(vec![])
            }
            "c:version" => {
                self.require_configuring()?;
                if self.common_version {
                    return Err(Error::State("repeated common version"));
                }
                if !CommonVersion::decode(bytes)?.versions.contains(&1) {
                    return Err(Error::Invalid("no common networking version 1"));
                }
                self.common_version = true;
                reply(channel, CommonVersion { versions: vec![1] }.encode()?)
            }
            "c:register" => {
                self.require_configuring()?;
                if !self.common_version || self.common_registered {
                    return Err(Error::State("common register ordering"));
                }
                let registration = CommonRegister::decode(bytes)?;
                if registration.version != 1 || registration.phase != Phase::Play {
                    return Err(Error::Invalid("common register v1/play required"));
                }
                let channels = self
                    .offers
                    .iter()
                    .filter(|o| {
                        o.phase == Phase::Play
                            && o.optional
                            && o.direction != Direction::Serverbound
                    })
                    .map(|o| o.id.clone())
                    .collect();
                self.peer_common = registration.channels;
                self.common_registered = true;
                reply(
                    channel,
                    CommonRegister {
                        version: 1,
                        phase: Phase::Play,
                        channels,
                    }
                    .encode()?,
                )
            }
            _ => {
                self.negotiation.receive_configuration(&channel, bytes)?;
                Ok(vec![])
            }
        }
    }
    fn require_configuring(&self) -> Result<()> {
        if self.negotiation.state() == State::Configuring {
            Ok(())
        } else {
            Err(Error::State("common task before modded setup"))
        }
    }
    /// Registry/task barriers and all play-semantic boundaries remain missing.
    /// Neither an empty setup nor successful optional callbacks can bypass this.
    pub fn finish_configuration(&mut self) -> Result<()> {
        self.failed = true;
        self.negotiation.finish_configuration()
    }
    pub fn close(&mut self) {
        self.failed = true;
        self.setup_failure = None;
        self.negotiation.close();
        self.offers.clear();
        self.peer_common.clear();
        self.peer_dinnerbone.clear();
    }
}
fn reply(channel: Id, body: Vec<u8>) -> Result<Vec<Outbound>> {
    Ok(vec![Outbound { channel, body }])
}
