# Current development handoff

Start with `PROGRESS.md` and the updated README, then read the historical
handoff below. Query/setup codecs, a handler-backed early Loader controller,
registry task staging/host barriers, and a concrete Horizon patch now exist.
No successful finish or join exists. Next: failure Component/NBT codec + genuine
Java differential execution; full pinned Horizon + companion typecheck/runtime
and TCP tests; real host semantic registry/content application; remaining
config/data-map/enum/flag tasks and split transport; isolated baseline join.
Do not remove finish latches as a shortcut and do not rewrite individual mods.

---
# Handoff for the next development chat

## Start here

The archive is an independent Rust crate with no external crate dependencies.
Read README first. `cargo test --offline --locked` on a normal native Rust setup
should exercise the library and CLI; actual recorded validation used musl.
The **existing code does not join any NeoForge server**, even at the baseline.
Do not spend the next chat rewriting Create, Ponder, Sable, or all ~70 mods.

First implement the early engine bootstrap plus complete modded negotiation
codec and reference differential tests; next implement registry synchronization
with genuinely applied semantic maps. Follow HORIZON_INTEGRATION.md. The current
common protocol submachine is useful in isolation, but not the first/only
NeoForge handshake phase. It intentionally rejects `neoforge:register`.

## Inputs still required (do not guess versions)

Provide an inventory from the exact target server, not the latest project pages:

- Actual NeoForge build **after the last auto-update**, its artifact SHA-256,
  loader/FML inventory and server-side startup log. 21.1.248 is our source pin,
  not evidence that today's running server still uses it.
- Exact **Create jar filename, full version, SHA-256 and download origin** for
  MC 1.21.1 NeoForge, plus the source tag/commit matching that jar.
- Exact **Ponder jar filename/version/SHA-256**, including whether it is bundled
  or external, and any transitive runtime libraries that affect registration.
- Exact **Sable jar filename/version/SHA-256**, matching source ref if available,
  native/physics dependencies and negotiated features. No version is inferred
  from the main branch or guessed from a pack name.
- All remaining mod jar filenames, versions, sides (client/server/both), hashes,
  required dependencies and the exact server pack/config snapshot. Separate
  server-only mods from genuinely required client behavior.
- A sanitized, permissioned configuration transcript or instrumented reference
  endpoint: phase, flow, channel, body bytes, supported registration metadata,
  registry snapshots and relevant disconnect text. Remove auth/session secrets.
  Never capture credentials, login tokens or private chat unnecessarily.
- Client-side desired feature scope for the first adapter and legal approval for
  any license-restricted protocol investigation. A mod list alone does not
  establish that every registered payload may safely be ignored.

A sample inventory schema (not a functioning mod/loader manifest):

```json
{
  "minecraft": "1.21.1",
  "protocol": 767,
  "neoforge": "REQUIRED_ACTUAL_BUILD",
  "mods": [{"filename": "REQUIRED.jar", "version": "REQUIRED", "sha256": "REQUIRED", "source_ref": "REQUIRED", "side": "REQUIRED"}]
}
```

## Shared infrastructure vs future adapters

Keep transport framing, loader negotiation, task gating, payload routing, content
ID mapping, bounded queues, diagnostics and epoch cleanup in the shared layer.
Keep a mod's payload fields, registries, configuration interpretation and game
semantics in an adapter. The optional callback API here only models bounded
notification receivers; it cannot express a complete stateful mod adapter,
required channel/version/flow negotiation or replies. Extend it deliberately
with typed contexts, lifecycle hooks and async/main-thread work barriers; do
not fill that gap with no-op “handlers.”

**Create/Ponder:** supplied repos were read at overview level only:
https://github.com/Creators-of-Create/Create and
https://github.com/Creators-of-Create/Ponder . These pages establish the projects,
not the user's jar versions or payload schemas. Future work must first inventory
the exact jar's registrations and required channel versions, then pick one
bounded feature path. No Create/Ponder implementation or assets are copied here.
Ponder's guide rendering is not a prerequisite for pretending all Create network
messages are implemented: any required unimplemented behavior must still fail.

**Sable:** https://github.com/ryanhcode/sable identifies dynamic sub-levels and
warns it is intrusive. Its overview names **PolyForm Shield 1.0.0**. No Sable
implementation was copied, translated or vendored. Obtain appropriate legal
review before a replacement/adapter effort; reading a public repo does not
resolve license permissions. A future adapter needs explicit sub-level identity,
chunk/entity coordinate and lifecycle semantics, not channel-name registration
alone. This milestone supplies none of those semantics.

## Auto-update policy

Keep source pins and fixture transcripts per loader build. Reject all nonexact
builds by default (including 21.1.249); for a new build, retrieve its public source,
diff the network/registry and upstream primitive-codec dependencies, run reference
vectors and the permissioned join suite, and only then add a new evidence-bearing
profile. Do not rename an untested build “compatible.” The profile also needs the
pack inventory hash; a stable loader build does not freeze mod protocol versions.

## Suggested next milestone completion criteria

1. Independent Java/Rust codec equivalence for full modded query/setup and failure.
2. Early native engine endpoint verified with isolated transport tests, then a
   real Horizon compile; no mod manifest until a real loadable package exists.
3. Registry sync task applied atomically to a deliberately small supported
   content set; explicit refusals for every other required registry/codec.
4. Baseline server configuration reaches a genuine validated barrier with no
   ignored task. Only then allow play and validate a minimal world connection.
5. One exact-version adapter smoke test, not an entire pack rewrite.

No live-server validation, account credentials, server address, mod jars or
pack assets were available or used for the current milestone.
