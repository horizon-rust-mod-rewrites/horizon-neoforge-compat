# Horizon integration assessment — DESIGN ONLY, NO PATCH APPLIED

Pinned checkout: https://github.com/rdbtCVS/horizon at
`ae632edf48f6cae3ba52a5c81523ec712488c9cc`. Source hashes are in
`provenance/horizon.json`. Locations below refer to that commit, not latest.

## What was actually inspected

1. `crates/horizon-client/src/net.rs`, `configure()` (~1740–2000): it starts
   Horizon's own payload epoch and `horizon:payloads` negotiation **before** its
   receive loop. It answers built-in configuration messages directly. In the
   generic CustomPayload branch it calls `receive_application()` before brand
   handling. Unknown unowned messages can emerge as `Other`, after which they
   receive no general NeoForge task handling. Silently going through this path
   is not compatibility.
2. `receive_application()` (~1670), and `raw_payloads/incoming.rs::classify()`:
   before an accepted Horizon payload epoch, a locally owned channel is refused
   as NotNegotiated; an unowned channel falls through. Raw channels are delivered
   only in **server-side play**, not early client configuration.
3. `crates/horizon-mod-api/src/parts/payloads.rs::declare_payload()`: payload
   declarations are owner-namespaced, versioned, phase/direction-specific, and
   require handlers. They use Horizon negotiation, not NeoForge's server-owned
   modded negotiation. Registering `neoforge:*` as a normal third-party mod's
   payload is not a justified bootstrap strategy.
4. `crates/horizon-mod-vocab/src/hooks/client_packet.rs`: the public hook
   explicitly excludes configuration and network-thread handled packets. A
   play-packet observer cannot answer early loader negotiation or gate finish.
5. `horizon-protocol/src/connection.rs::Connection::adapt()`, `ConnReader::recv()`:
   an old **server** is adapted with `adapter::Session`. Its clientbound packets
   are translated before configure() sees them. `serve_older()` uses
   `ClientSession` for a server talking to an old **client**; these are opposite
   directions and must not be confused.
6. `horizon-protocol/src/adapter.rs`, `adapter/v1_21_1.rs`: protocol 767 is already
   identified; packet ID tables translate to Horizon's pinned 777/26.3 model.
   Reusing the adapter is necessary; inserting raw 1.21.1 packet IDs in net.rs
   would double/mis-map packet IDs.
7. `adapter/session.rs::clientbound()` updates its reading phase when it sees
   FinishConfiguration, before net.rs acts on it. It also logs/falls through on
   translation errors. `adapter/translate.rs` and `translate/v1_21_1.rs` implement
   vanilla conversions, including vanilla registry processing. These are NOT
   application of NeoForge frozen registry IDs to every content-bearing path.
   Reverse `client_session/step_1_21_1.rs` and `synced.rs` were also inspected for
   directionality and registry-map behavior; they do not supply a loader bridge.

## Why there is no integration patch in this milestone

An in-place patch would be testable only after a real pre-negotiation engine
route and full loader task gate exist. This library cannot finish configuration;
adding it as a normal mod would either fail early or misleadingly appear to
load without answering the server. Horizon itself was not built here. An
untested patch or fake mod package would not meet the requested honesty bar.
No public hook found in these inspected seams supplies the necessary early
client bootstrap. This is not a claim that no possible engine extension exists.

## Concrete next patch design

- Add an opt-in **connection profile**, selected before configure() starts,
  containing MC/protocol, exact loader version, pack fingerprint, and bounded
  adapter registry. Do not infer loader version from brand or c:version.
  Auto-updating servers need new inventory attestations, not a wildcard match.
- Add a native engine-owned early configuration endpoint in net.rs: initialize
  it before the current `has_mod_declarations()/negotiation.initial()` send.
  In NeoForge mode do not send `horizon:payloads`, build identity, mod-list or
  other Horizon-only offers speculatively. Keep the existing path unchanged
  for Horizon peers and vanilla peers with no explicit NeoForge profile.
- Route `neoforge:*`, `c:version`, `c:register`, Dinnerbone registration, and
  explicitly declared adapter payloads to that endpoint **before** generic
  `receive_application()`. Do not route brand/keep-alives/resource packs to this
  strict submachine: they still belong to the engine. Unknown NeoForge/adapter
  configuration tasks must disconnect with a structured diagnostic.
- Add complete audited modded query/setup codecs; validate phase enum ordinals,
  version equality, optionality and flow rules. The upstream server chooses the
  accepted set. Install only handler-backed accepted channels; never substitute
  c:register for neoforge:register.
- Preserve custom-payload body bytes through the existing protocol-767 packet
  adapter; add transport tests proving no unexpected transformation, no double
  packet-ID remapping and appropriate bounded-codec limits.
- Stage frozen snapshots in an epoch-local map. Validate **all** expected
  registries, aliases and semantic availability, then commit atomically. Only
  acknowledge frozen_registry_sync_completed once the commit succeeds. A map
  returned by RegistryMap::build() alone cannot justify that acknowledgement.
- Introduce an explicit registry conversion boundary covering both directions:
  items/components, blockstate properties and palettes, chunks, entities and
  serializer IDs, menus, recipes, tags, attachments and adapter payload internals.
  Avoid composing vanilla numeric mappings with modded server IDs blindly.
  Server block registry IDs do not by themselves describe blockstate layouts.
- Gate net.rs's FinishConfiguration branch **before** its queued-writer protocol
  transition and finish ack. The adapter's read phase may already have changed;
  keep the connection quiescent and disconnect on failure, never continue reading
  play or attempting a reset after that failure. Preserve keep-alive service,
  ordering, ingress budgets and queued-main-work completion during valid tasks.
- Distinguish NeoForge epoch acceptance from Horizon epoch acceptance. Do not
  fabricate `horizon:payloads` Accepted metadata to open Horizon transport.
  Add a native adapter delivery route with the same ownership/backpressure
  safeguards, rather than disabling them globally.

## Acceptance tests before calling the engine patch usable

- Exact baseline NeoForge loopback transcript from reference Java implementation;
  compare every implemented field with independent encoders/decoders.
- Pre-Horizon-negotiation query reaches the new endpoint; undeclared channels
  never reach callbacks; no Horizon offers leak to vanilla/NeoForge profiles.
- Unknown required channel/version/flow fails, optional absent channels omitted;
  common/Dinnerbone registrations never grant modded channels automatically.
- Registry missing entry/alias cycle/duplicate/unsupported blockstate fails
  atomically; **no ack** and no partially changed Horizon content IDs.
- All IDs translated exactly once through protocol 767 and the content boundary;
  data-map/enum/flag/config tasks never silently pass.
- Finish before task completion fails; failure sends no finish ack; reconfigure,
  disconnect, reconnect and proxy transfer discard prior epochs/maps.
- Vanilla and Horizon regressions plus actual socket tests; then a clean
  21.1.248 server, then the exact user pack with permissioned server testing.

This design is a handoff, not an implementation or validated Horizon patch.
