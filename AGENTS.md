# Full hosted validation PASSED — 2026-11-27

Run: https://github.com/horizon-rust-mod-rewrites/horizon-neoforge-compat/actions/runs/46079113415
Tested compatibility commit: `7d4e934327ff833ec8d57707e6d8f25962c07c61`.
Upstream Horizon: `ae632edf48f6cae3ba52a5c81523ec712488c9cc`.
Rust/Cargo 1.97.0, GNU x86_64, Ubuntu 24.04, no default client features.
Downloaded artifact SHA256 verified:
`2d89b8910bdcb7c04bb8413338b1601ac4150222ec5be9dd70a130df3f4b8770`.

Retained transcript `integration/hosted-success-full-driver.txt` ends in PASS
after source re-verification. `integration/hosted-success-full-check.txt` proves:

- Complete pin: 12,354 blobs, zero missing; 27 provenance/integration hashes.
- Full patched client typechecks before/after ACTUAL companion wiring, with
  audited local-only core dependency addition; upstream locked graph unchanged.
- Linked full client test binary: **4 actual companion contract tests pass**.
- **9 early-configuration tests pass**, including real configure/queued-writer
  integration and engine hard finish refusal without forged epoch/ack.
- **1 protocol-767 bidirectional TCP payload-framing test passes**.
- Network filter: **81 passed, 3 ignored, 0 failed**: 52 direct net tests pass,
  3 direct net daily tests ignored, plus 29 datapack/LOD/perf tests pass.
  The filter matches module substrings; not all 81 are direct net tests.
- Core: **153 tests in EACH debug and release**, strict Clippy, fmt/docs pass.
- 28 Python driver tests, six fixtures, patch/reverse/source-restoration and
  isolated companion checks pass. Isolated endpoint tests overlap the above:
  do not add them to full-engine counts as independent coverage.

This is selected full-engine test execution, NOT all 4,624 client-library or all
593 protocol tests. Five upstream horizon-mod-wasm unused-function warnings
remain; upstream was not changed to hide them. No runtime UI, production server,
full task completion, semantic registry/content conversion, split host wiring
or NeoForge join is validated. **The client STILL CANNOT JOIN. Both finish gates
remain.** Newer commits after the tested SHA are documentation/evidence only.

Next: genuine host-owned capability/profile evidence before flag inventory
opt-in (see `docs/horizon-host-inventory-audit.md`), remaining configuration tasks,
split transport and atomic bidirectional semantic content application. Exact
NeoForge 21.1.248 isolated server then fixture mod only after prerequisites;
no production-first testing or individual mod rewrites. See NEXT_AGENT.md first.
Older contradictory status below is historical. Same draft PR; attributed
commits only, verify remote HEAD, never merge/force-push. The owner authorized
public visibility for standard hosted CI; no further visibility change implied.

---
# Latest checkpoint: opt-in feature-flag task

See `NEXT_AGENT.md` first for credit-conscious continuation priorities.
Bounded feature-flag equality now has explicit opt-in Loader routing, negotiated
data/ack pair authority and terminal mismatch/ordering cleanup. No default host
inventory, flag-controlled game semantics or Horizon companion wiring is invented.
**153 main/driver tests pass debug/release**, strict Clippy and isolated companion
regressions pass. See `docs/FEATURE_FLAGS.md` and its validation transcript.
Still cannot join; full-engine build, real content mapping and other tasks remain.

---
# Continuation evidence (2026-11-27)

Continues existing draft PR #1 from verified remote head
`3331751dc938ba2fdd430dd28ead49d343f612b2`, without rewriting individual mods.
**Still cannot complete configuration or join.**

- Loader now retains bounded, lossless setup-failure **NBT data**, never a
  successful setup or safe-to-render Component. See `provenance/setup-failure.md`.
  Component semantics/rendering remain incomplete; every failure is terminal.
- Genuine source-extracted NeoForge query/setup codecs executed against actual
  official MC 1.21.1 classfiles: 256 models, 1,024 cross-language semantic checks,
  1,711 shared invalid/truncation cases, 9 classified policy differences and 6
  writer-limit cases. See `docs/java-reference.md` and `tests/reference/results.json`.
- Bounded split reassembly primitive, 18 tests; not yet advertised or wired to
  Horizon packet decoding. See `docs/SPLIT.md`.
- Horizon integration boundary hardening, executed endpoint/bridge tests and a
  reproducible full-build driver; full-engine build/runtime remains unexecuted.
  See `integration/README.md` and `docs/horizon-continuation.md`.
- Main crate **128 tests pass debug and release** with Rust 1.95.0 / x86_64 musl.
  Formatting, warning-denied docs, six fixtures and source verification pass.
  Strict Clippy also passes in this run.
  Current run: `docs/validation-continuation.txt`. Full engine, real content publication, remaining
  task execution, split host integration and live joins remain outstanding.

Earlier status below is historical where it conflicts with this continuation.
No finish latch was removed and no full-handshake completion is claimed.

---
# Current continuation status (supersedes milestone 0.1 truth below)

Read `docs/PROGRESS.md` and updated README along with the read order below.
This branch adds modded query/setup codecs, an early Loader controller,
handler-backed server selection validation, bounded registry task staging with
mandatory semantic-host commit barrier, and an applicable Horizon early endpoint
patch + companion. Main crate: 90 tests pass debug/release. Patch and companion
contract tests pass in isolation. **Still cannot join.** Full Horizon typecheck,
Java differential codecs, setup-failure NBT Component decoding, actual Horizon
content application, remaining required loader tasks and live joins are missing.
Both core and engine retain hard finish refusal. No individual mod rewrites.
Exact baseline and mandatory AI commit footer policy below remain unchanged.

---
# Development handoff: read before changing this repository

## Goal and user decisions
Build a reusable Horizon Rust client compatibility layer for Minecraft 1.21.1 NeoForge servers so roughly 70 future mod rewrites share loader negotiation, routing, registry translation, and diagnostics. Do NOT rewrite Create, Ponder, Sable, or other mods in this phase. Future mod rewrites will be developed in separate chats.

Intended GitHub destination: organization `horizon-rust-mod-rewrites`, repository `horizon-neoforge-compat`. Keep the repository private unless the owner explicitly requests otherwise. The private repository was created for this handoff. Verify the remote branch and commit before starting work; do not assume an upload succeeded merely because files exist locally.

User's Java client uses NeoForge 21.1.248. The server auto-updates; its actual running build is unknown. Target baseline MC 1.21.1, Java protocol 767, NeoForge 21.1.248. Do not claim all 21.1.x builds compatible.

## Commit policy: mandatory
Every assistant-created commit must disclose AI involvement. Include this footer in EVERY commit, including initial commits and any merge/squash commits you create:

    AI-Assisted-By: QX AI assistant

Do not invent a co-author account or email. Use truthful authorized attribution. Prefer feature branches and reviewable pull requests after initial repository setup; do not force-push or overwrite existing work.

## Read in this order
1. README.md: actual scope and limitations.
2. docs/VALIDATION.md: recorded checks, commands, and limitations.
3. docs/WIRE.md: supported codecs and local policy.
4. docs/HORIZON_INTEGRATION.md: inspected engine integration points and blockers.
5. docs/HANDOFF.md: implementation order, exact-version inputs, adapter boundaries.
6. provenance/neoforge-21.1.248.json and THIRD_PARTY_NOTICES.md.
7. Source and tests themselves. Documentation is not proof of end-to-end functionality.

## Current truth
This is an independent, dependency-free Rust core, NOT a loadable Horizon mod. It CANNOT JOIN a NeoForge server. Full `neoforge:register` / `neoforge:network` negotiation is not implemented. Configuration completion intentionally errors. No Horizon patch has been applied. ID maps exist as infrastructure but are not applied across Horizon's packet/content decoders.

Implemented subset: bounded common negotiation/registration codecs, fail-closed common-task submachine, optional handler-backed extensions, frozen-registry snapshot decoding and explicit atomic registry maps, offline diagnostic CLI and synthetic fixtures.

Prior milestone recorded 33 passing tests in both debug and release with Rust 1.85.1, Linux x86_64 musl, plus formatting, strict Clippy, docs, fixture and source checksums. Re-run checks in your environment. There has been NO live-server join validation or Java/Rust differential reference harness. Synthetic fixtures are not server-captured evidence.

Inspected Horizon source: https://github.com/rdbtCVS/horizon at ae632edf48f6cae3ba52a5c81523ec712488c9cc. This project is not that repository, and the full checkout is not bundled here. Retrieve the pinned public source as needed. Do not rely on previous chat-local directories or downloaded tooling remaining available.

## Next implementation milestone
1. Audit the exact loader source pin and trace complete negotiation/task ordering; add bounded codecs and independent reference equivalence tests for modded registration, setup, and failures.
2. Implement the early Horizon connection bootstrap using the documented integration seams. Check the 1.21.1 version adapter in both directions. Apply the smallest justified core patch if native-mod hooks are insufficient; do not promise a standalone mod without proving that surface exists.
3. Connect actual registry synchronization atomically to a narrowly supported content set. Applying an item map does not establish correct block states, components, chunks, entity serializers or recipes.
4. Validate against an isolated baseline NeoForge server, then a minimal fixture mod. Do not test first against the user's auto-updating production server.
5. Only allow configuration completion/play once required handlers, tasks and mappings are genuinely implemented. Record actual logs and distinguish codec tests, integration tests, and live joins.

Never silently acknowledge unsupported tasks, register placeholder handlers, spoof full mod support, or ignore required payloads. Use bounded inputs and explicit diagnostics with phase/channel/registry context. Reset all connection-specific state on disconnect/reconnect.

The exact production server build and jar versions/hashes will be needed for pack testing, but are not a reason to block implementing/testing the pinned loader baseline.

## Later projects, not current scope
- https://github.com/Creators-of-Create/Create
- https://github.com/Creators-of-Create/Ponder
- https://github.com/ryanhcode/sable

These links identify projects, not exact deployed versions. No implementations or assets from those mods were copied. Sable identifies a PolyForm Shield license: review licensing before copying/translating implementation or distributing a replacement. Do not assume private visibility removes obligations.

## Repository hygiene
Exclude target/, downloaded compilers, upstream source archives, credentials, Minecraft jars/assets, server worlds and logs containing personal data. Keep Cargo.lock, source, tests, synthetic fixtures, provenance metadata, docs and reproduction scripts. Never commit authentication tokens. Maintain truthful milestone status as features are completed.
