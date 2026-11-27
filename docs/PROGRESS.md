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
