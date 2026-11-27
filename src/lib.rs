//! Bounded loader negotiation infrastructure, NOT a joining client or loadable mod.
//! All byte slices are payload bodies (no channel, packet ID, compression or framing).
#![forbid(unsafe_code)]
pub mod codec;
pub mod extension;
pub mod loader;
pub mod modded;
pub mod negotiation;
pub mod registry;
pub mod registry_sync;
pub mod session;

pub const MINECRAFT: &str = "1.21.1";
pub const PROTOCOL: i32 = 767;
pub const NEOFORGE: &str = "21.1.248";

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    Limit(&'static str),
    Truncated,
    VarInt,
    Utf8,
    Identifier(String),
    Trailing,
    Duplicate(String),
    Invalid(&'static str),
    UnsupportedTask(String),
    UnsupportedBaseline {
        minecraft: String,
        protocol: i32,
        loader: String,
    },
    State(&'static str),
    Missing(String),
    Handler(String),
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "NeoForge compatibility milestone: {self:?}")
    }
}
impl std::error::Error for Error {}
pub type Result<T> = std::result::Result<T, Error>;

/// Exact source-audited baseline only; not a claim of end-to-end interoperability.
/// No wildcard allowlist for auto-updated 21.1.x servers.
pub fn check_baseline(minecraft: &str, protocol: i32, loader: &str) -> Result<()> {
    if (minecraft, protocol, loader) == (MINECRAFT, PROTOCOL, NEOFORGE) {
        Ok(())
    } else {
        Err(Error::UnsupportedBaseline {
            minecraft: minecraft.into(),
            protocol,
            loader: loader.into(),
        })
    }
}
