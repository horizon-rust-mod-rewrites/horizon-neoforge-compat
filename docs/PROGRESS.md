# Full compatibility completion tracker

See [COMPATIBILITY_PROGRESS.md](../COMPATIBILITY_PROGRESS.md): **30%** of the initial evidence-backed, weighted acceptance checklist. This is not an effort estimate or mod-support percentage. **Not ready to begin individual mod rewrites; CANNOT JOIN.** The tracker includes remaining transport ownership, required tasks, semantic content, actual-client integration and isolated live proof. This checkpoint adds tracking/acceptance criteria only, not runtime support.

---

# Latest exact-SHA hosted validation: outbound milestone PASSED

Run: https://github.com/horizon-rust-mod-rewrites/horizon-neoforge-compat/actions/runs/46087704109
Tested compatibility SHA: `fb0294b4976bd78d4ee4cbfce44a6f049e0a4307`.
Horizon pin: `ae632edf48f6cae3ba52a5c81523ec712488c9cc`.
Artifact SHA-256 verified: `357207d45ab75c4e2cf94485f31f661915498617bb9fc51db73e570ed3670f5f`.
Retained `integration/hosted-46087704109-{core,full-check,full-driver,scope}.txt`.

- Full actual patched client + companion typechecks passed; pinned lock audit passed.
- **16 actual companion**, **9 early configuration**, **14 protocol framing**,
  **81 network-filter passed / 3 ignored**. Selected suites, NOT all upstream tests.
- **171 core tests EACH debug and release**, including **36 split tests**;
  strict Clippy, fmt, warning-denied docs, 35 Python tests and six fixtures passed.
- New actual outbound preparation tests and the TCP split/compression/encryption
  matrix REALLY executed. No production split route, Java split equivalence,
  complete loader tasks, semantic content application, UI or live join is proven.

Default split/feature-flag opt-ins remain disabled. Both finish refusals remain.
**The client STILL CANNOT JOIN.** Newer evidence/documentation-only commits do not
change the tested source and deliberately do not trigger another full build.
See NEXT_AGENT.md for precise continuation priorities. Older contradictory
counts/pending statements below are historical, not current validation status.

---

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
# Handshake development progress

Baseline: MC 1.21.1, Java protocol 767, NeoForge 21.1.248;
Horizon ae632edf48f6cae3ba52a5c81523ec712488c9cc.
Starting compatibility commit: 5df54856212fcf66432edbbd6e08df56b438b3be,
verified against the remote branch and every local tracked blob before work.
No individual mods were rewritten. The repository remains private.

## What is real now

- `modded`: query/setup codecs, all exact protocol/flow ordinals, aggregate
  entry bounds, strict duplicate/mismatch/truncation validation; body-only bytes.
- `negotiation`: callback-backed direction/version offers, atomic validation of
  server setup, mandatory required channels, optional absence, per-body bounds,
  terminal failed epochs. These callbacks are stateless body processing, not
  sufficient APIs for stateful/asynchronous game adapters.
- `loader`: connects the real wire codecs to selection and common task handling.
  Server query is an empty map request, not a set of server offers. The server
  chooses setup. Dinnerbone/common registration cannot grant modded channels.
  Total ingress budget: 8,192 messages / 4 MiB bodies in one configuration epoch.
  Conservative baseline-only policy rejects nonempty server query maps and
  nonconfiguration/play setup phases. Close/failure revokes grants.
- `registry_sync`: start/completed codecs, staging with aggregate wire/entry/alias
  budgets, mandatory supported-registry/semantic-validation/atomic-commit host
  contract. A completion token is issued only after synchronous commit. There
  is no actual Horizon host, and the loader does NOT advertise these tasks.
- `integration`: actual source patch for early engine-owned endpoint, explicit
  exact profile and pack hash, protocol-767 adaptation, no Horizon offer leak,
  finish latch, exit reset and bounded whole reply batches. Original checkout
  remains clean. MIT companion connects actual Loader and patch in contract tests.

## Evidence and honest limits

90 main-crate tests pass debug/release, strict Clippy, rustfmt and docs.
Patch application/reversal/provenance checks pass; six endpoint and one actual
protocol-table test pass plus four structural checks. Companion: nine tests
(same six endpoint plus three cross-component). These are not live transcripts.
MC ordinals were checked against exact official classfile constructors and
values-array order; no Java codec was executed. Full Horizon compile was blocked
by absent offline dependencies, and available Rust 1.85.1 is below pinned
workspace Rust 1.95. Included engine-loop and TCP tests are NOT executed evidence.

No fake handler or setup-success path was added for NBT Components or required
unsupported tasks. Setup failure always rejects. Full configuration completion
is still refused in both core and engine; **the client still cannot join**.
The old Session/CLI remains common-subset-only by design, not silently upgraded.
Registry unit tests use explicit synthetic hosts, not actual Horizon semantics.

## Next implementation order

1. Implement bounded exact setup-failure Component/NBT decoding and genuine
   Java/Rust reference codec execution (not self-roundtrips or classfile parsing).
2. Build/typecheck the patched Horizon and companion with its required toolchain
   and dependencies; execute included actual configure/writer, TCP and existing
   vanilla/Horizon regressions. Preserve real diagnostics safely in production glue.
3. Supply a real synchronous/queued-main-work-aware semantic host and atomic
   content publication for a deliberately supported subset. Map every affected
   packet/content direction exactly once, not merely item IDs. Connect registry
   lifecycle only after this exists; do not advertise it beforehand.
4. Implement and validate config, data-map, extensible-enum, feature-flag tasks,
   split packets, task scheduling/timeouts and transport/backpressure integration.
5. Test isolated exact baseline NeoForge, then one minimal fixture mod. Only
   remove both finish latches once required tasks and semantic boundaries are
   genuinely complete and demonstrated by real logs. Never test production first.
6. Obtain actual auto-updated server build and exact pack/jar hashes for later
   pack testing. This need does not block pinned-baseline implementation.

Keep all assistant-created commits, including merge/squash commits, attributed:

    AI-Assisted-By: QX AI assistant
