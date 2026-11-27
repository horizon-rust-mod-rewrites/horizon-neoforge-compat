// SPDX-License-Identifier: MIT
//! Companion glue for a patched Horizon client. Add the compatibility crate as
//! an explicit path/git dependency, then include this module in horizon-client.
//! This is NOT wired into the launcher or a loadable mod. Full engine compilation
//! remains required. check_bridge.py exercises this actual glue against the
//! actual patched endpoint and compatibility library without Horizon dependencies.
use crate::net::early_configuration::{Bridge, Factory, Failure, Reply, Result};
use horizon_neoforge_compat::{loader::Loader, negotiation::Registration};

/// Constructs fresh handler state for each connection. This version only accepts
/// stateless body callbacks; no stateful required mod/task adapters are supplied.
pub struct CompatFactory {
    pub pack_sha256: [u8; 32],
    pub registrations: fn() -> Vec<Registration>,
}
impl Factory for CompatFactory {
    fn create(&self, inventory: [u8; 32]) -> Result<Box<dyn Bridge>> {
        if self.pack_sha256 == [0; 32] || inventory != self.pack_sha256 {
            return Err(Failure("inventory_mismatch"));
        }
        let loader = Loader::new("1.21.1", 767, "21.1.248", (self.registrations)())
            .map_err(|_| Failure("invalid_compatibility_registrations"))?;
        Ok(Box::new(CompatBridge(Some(loader))))
    }
}
struct CompatBridge(Option<Loader>);
impl Bridge for CompatBridge {
    fn receive(&mut self, channel: &str, body: &[u8]) -> Result<Vec<Reply>> {
        let loader = self
            .0
            .as_mut()
            .ok_or(Failure("closed_compatibility_epoch"))?;
        loader
            .receive(channel, body)
            .map(|out| {
                out.into_iter()
                    .map(|reply| Reply {
                        channel: reply.channel.to_string(),
                        body: reply.body,
                    })
                    .collect()
            })
            .map_err(|_| Failure("compatibility_payload_rejected"))
    }
    fn finish(&mut self) -> Result<()> {
        self.0
            .as_mut()
            .ok_or(Failure("closed_compatibility_epoch"))?
            .finish_configuration()
            .map_err(|_| Failure("compatibility_tasks_or_content_incomplete"))
    }
    fn reset(&mut self) {
        if let Some(mut loader) = self.0.take() {
            loader.close();
        }
    }
}
