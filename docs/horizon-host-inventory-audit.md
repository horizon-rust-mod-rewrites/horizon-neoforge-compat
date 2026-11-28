# Horizon implemented-feature inventory boundary audit

Read-only source audit of Horizon **ae632edf48f6cae3ba52a5c81523ec712488c9cc**.
Not runtime, whole-tree absence proof, game-semantic validation or a server join.
The existing Java query/setup differential work was not repeated.

## Conclusion

The inspected APIs describe ENABLED feature names and consumers of those names,
not the complete inventory of actually implemented modded flags required by
NeoForge's equality task. Do not supply an empty set merely because no inventory
API was established. Unknown and known-empty are different assertions. The
existing companion correctly leaves feature-flag opt-in disabled.

## Exact-pin evidence

Paths and approximate line numbers refer only to the pin above. Browse sources at
`https://github.com/rdbtCVS/horizon/blob/ae632edf48f6cae3ba52a5c81523ec712488c9cc/`
followed by the path.

- `crates/horizon-protocol/src/packets/configuration.rs:40` represents
  `UpdateEnabledFeatures` as peer-provided identifier strings.
- `crates/horizon-client/src/net.rs:1909-1911` forwards those names as
  `Event::Features`; `game/network/remote/raw_ingress.rs:247-250` assigns them to
  `remote.features`. Paths beginning `game/` here are relative to that same
  client `src/` directory.
- `game/modding/native_level_world.rs:174-180`, `Game::enabled_feature_ids()`,
  returns remote or local-world enabled features. Using remote state as local
  implementation evidence would allow the peer to choose its own validation.
- `crates/horizon-mod-vocab/src/world/types.rs:603-643`, `FeatureFlags`, stores
  supplied names and tests membership. `iter()` synthesizes `minecraft:vanilla`
  for empty backing storage. Empty enabled storage is NOT a no-modded-support
  certificate. `Game::level_enabled_features()` wraps the enabled state, not an
  implementation registry (`native_level_world.rs:418-420`).
- `crates/horizon-world/src/conditions.rs:53-67,420-428,495-503` captures owning-load
  enabled context, performs membership checks and evaluates
  `neoforge:feature_flags_enabled`. These ARE genuine generic data-load semantics,
  but do not enumerate concrete modded flags with complete implemented behavior.
- `game/modding/load_conditions.rs:79-114` reports candidate context and passes
  supplied features into conditions. `game/modding/mods/mod.rs:79-80` records
  `root_feature_flags` under which content was prepared. Enabled load context is
  not a list of every implemented, including disabled, modded flag.
- Inspected `crates/horizon-mod-api/src/parts/shared.rs:40-87` (`PartRegistrar`)
  and `crates/horizon-mod-vocab/src/parts/registration_access.rs:9-19`
  (`RegistrationKeyKind`) did not establish an authoritative implementation
  inventory surface. This is bounded inspection, not a claim about every file.

The compatibility core already requires an explicit attestation in
`Loader::new_with_feature_flags`; equality cannot validate the host's honesty.
The companion's checked pack fingerprint is not that inventory.

## Safe next scope (not implemented by this audit)

Introduce a host-owned capability snapshot with explicit unavailable versus
known-inventory states. Bind it to the loaded implementation/profile lifetime,
not peer flags, world state, namespaces or the pack hash alone. An empty-only
restricted profile is possible only if the host actually establishes/enforces
that restriction. Nonempty names require separately audited concrete semantics.
Keep default construction unchanged; construct fresh tasks per configuration.

Extend the ACTUAL companion contract tests before opt-in: unknown inventory
never creates an offer; known-empty can select both directions and acknowledge
only data `[0]` with zero bytes; any nonempty mismatch fails terminally; enabled
state cannot mutate implementation evidence; stale/mismatched snapshots refuse
before offers; reconnect invalidates old state; failed transport resets the epoch.
Successful equality must still fail full configuration completion. Execute these
inside the full patched engine as well as isolated checks. Neither finish gate
nor unsupported task policy changes. No individual mod implementation.
