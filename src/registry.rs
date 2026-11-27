//! FrozenRegistryPayload + RegistrySnapshot wire codec and explicit map construction.
//! Does not mutate Horizon, decode blockstate palettes, or apply NeoForge snapshots.
use crate::{
    codec::{Id, Reader, Writer},
    Error, Result,
};
use std::collections::{BTreeMap, BTreeSet};
pub const MAX_REGISTRY_ENTRIES: usize = 4_096;
pub const MAX_ALIASES: usize = 1_024;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Snapshot {
    pub registry: Id,
    pub ids: BTreeMap<i32, Id>,
    pub aliases: BTreeMap<Id, Id>,
}
impl Snapshot {
    pub fn validate(&self) -> Result<()> {
        if self.ids.len() > MAX_REGISTRY_ENTRIES || self.aliases.len() > MAX_ALIASES {
            return Err(Error::Limit("registry entries"));
        }
        let mut names = BTreeSet::new();
        for (id, name) in &self.ids {
            if *id < 0 {
                return Err(Error::Invalid("negative registry ID"));
            }
            if !names.insert(name) {
                return Err(Error::Duplicate(name.to_string()));
            }
        }
        for name in self.aliases.keys() {
            if names.contains(name) {
                return Err(Error::Duplicate(name.to_string()));
            }
            self.resolve(name)?;
        }
        Ok(())
    }
    fn resolve<'a>(&'a self, name: &'a Id) -> Result<&'a Id> {
        let mut name = name;
        let mut seen = BTreeSet::new();
        while let Some(next) = self.aliases.get(name) {
            if !seen.insert(name) {
                return Err(Error::Invalid("alias cycle"));
            }
            name = next;
        }
        if !self.ids.values().any(|n| n == name) {
            return Err(Error::Missing(name.to_string()));
        }
        Ok(name)
    }
    pub fn decode(bytes: &[u8]) -> Result<Self> {
        let mut r = Reader::new(bytes)?;
        let registry = r.id()?;
        let mut ids = BTreeMap::new();
        for _ in 0..r.count(MAX_REGISTRY_ENTRIES)? {
            let id = r.varint()?;
            let name = r.id()?;
            if ids.insert(id, name).is_some() {
                return Err(Error::Duplicate(format!("registry ID {id}")));
            }
        }
        let mut aliases = BTreeMap::new();
        for _ in 0..r.count(MAX_ALIASES)? {
            let from = r.id()?;
            let to = r.id()?;
            if aliases.insert(from.clone(), to).is_some() {
                return Err(Error::Duplicate(from.to_string()));
            }
        }
        r.finish()?;
        let snapshot = Self {
            registry,
            ids,
            aliases,
        };
        snapshot.validate()?;
        Ok(snapshot)
    }
    pub fn encode(&self) -> Result<Vec<u8>> {
        self.validate()?;
        let mut w = Writer::new();
        w.id(&self.registry)?;
        w.count(self.ids.len(), MAX_REGISTRY_ENTRIES)?;
        for (id, name) in &self.ids {
            w.varint(*id)?;
            w.id(name)?;
        }
        w.count(self.aliases.len(), MAX_ALIASES)?;
        for (from, to) in &self.aliases {
            w.id(from)?;
            w.id(to)?;
        }
        Ok(w.finish())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RegistryMap {
    pub registry: Id,
    server_to_local: BTreeMap<i32, u32>,
    local_to_server: BTreeMap<u32, i32>,
}
impl RegistryMap {
    /// Atomic construction: every server name must resolve to explicit caller-supplied
    /// local semantics. No vanilla fallback or invented IDs; no partial map escapes.
    /// Aliases are validated metadata, not a license to synthesize absent local entries.
    pub fn build(snapshot: &Snapshot, local: &BTreeMap<Id, u32>) -> Result<Self> {
        snapshot.validate()?;
        if local.len() > MAX_REGISTRY_ENTRIES {
            return Err(Error::Limit("local registry"));
        }
        let mut server_to_local = BTreeMap::new();
        let mut local_to_server = BTreeMap::new();
        for (server, name) in &snapshot.ids {
            let local_id = *local
                .get(name)
                .ok_or_else(|| Error::Missing(name.to_string()))?;
            if local_to_server.insert(local_id, *server).is_some() {
                return Err(Error::Duplicate(format!("local ID {local_id}")));
            }
            server_to_local.insert(*server, local_id);
        }
        Ok(Self {
            registry: snapshot.registry.clone(),
            server_to_local,
            local_to_server,
        })
    }
    pub fn to_local(&self, server: i32) -> Result<u32> {
        self.server_to_local
            .get(&server)
            .copied()
            .ok_or_else(|| Error::Missing(format!("{} server ID {server}", self.registry)))
    }
    pub fn to_server(&self, local: u32) -> Result<i32> {
        self.local_to_server
            .get(&local)
            .copied()
            .ok_or_else(|| Error::Missing(format!("{} local ID {local}", self.registry)))
    }
}
