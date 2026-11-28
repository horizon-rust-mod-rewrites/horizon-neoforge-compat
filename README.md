## Latest validated continuation: awaited early writes

[Full hosted run 46161098482](https://github.com/horizon-rust-mod-rewrites/horizon-neoforge-compat/actions/runs/46161098482) passed at source `d0c05270057bc69da8082a229dc265f5d4929bdf`:
actual pinned client/companion typechecks,12 ordered-owner regressions and existing
selected suites pass. Early Loader/engine replies now wait for bounded worker
write completion; cancellation and every early terminal exit close transport and
reset the bridge. Full ordered phase/epoch/preparation ownership is STILL partial.
See NEXT_AGENT.md for exact counts, verified retained evidence and remaining gates.
**30/100 evidenced completion, NOT50%; still CANNOT JOIN.** No finish gate or
default split/feature-flag opt-out was removed.

# Full compatibility completion tracker

See [COMPATIBILITY_PROGRESS.md](COMPATIBILITY_PROGRESS.md): **30%** of the initial evidence-backed, weighted acceptance checklist. This is not an effort estimate or mod-support percentage. **Not ready to begin individual mod rewrites; CANNOT JOIN.** The tracker records remaining transport ownership, required tasks, semantic content, actual-client integration and isolated live proof; tracker metadata does not implement runtime support.
---
## Latest exact-SHA hosted validation: focused fixes PASSED

Run: https://github.com/horizon-rust-mod-rewrites/horizon-neoforge-compat/actions/runs/46137932076
Tested compatibility SHA: `7becb053b2d75f4a8978decbd8ecf94fb9d0dd95`.
Horizon pin: `ae632edf48f6cae3ba52a5c81523ec712488c9cc`.
Downloaded artifact SHA-256 verified: `51e780c2b15dedfe656528ea6995f067b205b816eccb3d8cc14584146b259f27`.
Retained `integration/hosted-46137932076-{core,full-check,full-driver,scope}.txt`.

- Full actual patched client + companion typechecks, pinned lock audit and selected engine regressions passed.
- **16 actual companion**, **9 early configuration**, **14 protocol framing**,
  **81 network-filter passed / 3 ignored**. Selected suites only, not all upstream tests.
- **172 core tests EACH debug and release**, including **37 split tests**;
  strict Clippy, fmt, warning-denied docs, 35 integration-driver tests and six fixtures passed.
- Both progress-metadata push/PR workflows passed with **28 validator tests**.
- This checkpoint adds malformed metadata-shape validation, exact-maximum inbound
  split-boundary coverage and a portable scratch-directory example. No production
  runtime routing was changed.

Default split/feature-flag opt-ins remain disabled. Both finish refusals remain.
**The client STILL CANNOT JOIN.** No production split routing, complete loader
tasks, semantic content application, UI or live server join is proven. See
NEXT_AGENT.md for current continuation priorities. Older checkpoint sections below
are historical where they differ.
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
# Horizon → NeoForge compatibility: handshake development

**NOT a loadable mod. CANNOT JOIN a NeoForge server yet. No live-server validation.**

Shared Rust infrastructure targeting **Minecraft 1.21.1 / protocol 767 /
NeoForge 21.1.248 only**, plus an opt-in early configuration patch for pinned
Horizon `ae632edf48f6cae3ba52a5c81523ec712488c9cc`. No individual mods are rewritten.
This advances milestone 0.1; it does **not** complete the full handshake milestone.

## Implemented

- Existing bounded common/Dinnerbone registration and frozen-snapshot codecs,
  explicit bidirectional numeric registry-map infrastructure, offline CLI.
- Bounded `neoforge:register` query and `neoforge:network` setup codecs. Required
  version strings, optional packet direction, redundant setup IDs, aggregate
  counts and duplicate checks. Exact MC enum ordinals audited from official
  1.21.1 classfiles; not guessed from neighboring releases.
- Handler-backed offers and atomic validation of the server-selected channel
  set. Required absence, undeclared IDs, version/phase/direction mismatch and
  callback failure terminate the epoch. Optional absence disables the channel.
- `loader::Loader`: early query → offer → setup → common-task routing with ingress
  budgets and fail-closed cleanup. The server selects channels. Registration
  announcements never grant modded-channel authority. It never authorizes play.
- Registry start/completed codecs and bounded epoch-local staging. A mandatory
  host semantic validator and synchronous atomic commit must both succeed before
  a completion acknowledgment is returned. **No actual Horizon content host is
  supplied**; numeric maps alone do not establish content support.
- Applicable Horizon source patch: explicit profile, early native routing before
  Horizon negotiation, suppressed Horizon offers, protocol-767 adapter selection,
  reset on exit, and a hard finish gate. Companion bridge exercises the actual
  core against the actual patched endpoint. **Full Horizon was not compiled.**

## Validation actually performed

Rust 1.85.1, Linux x86_64 musl, dependency-free crate:

- **90 tests pass in debug and release**, strict Clippy, formatting, API docs.
- Exact upstream NeoForge source artifact/checksum and MC ordinal audit.
- Patch application/reversal, endpoint and protocol-table checks; actual
  library/companion/patched-endpoint contract tests. See integration validation.
- Fixtures remain **synthetic**. Classfile parsing is not Java codec execution.

Reproduce on a normal Rust installation:

```sh
cargo test --offline --locked
cargo test --offline --locked --release
cargo clippy --offline --all-targets -- -D warnings
cargo fmt --check
python3 fixtures/generate.py --check
python3 provenance/verify_upstream.py
```

The recorded runs used `--target x86_64-unknown-linux-musl` and a configured
linker; GNU/native and other platforms were not run. For the pinned Horizon
source, follow `integration/README.md`, including `check.py` and `check_bridge.py`.
No compiler, upstream game jar, world, credentials or build output is committed.

## Remaining blockers — do not mistake tests for a join

1. Setup-failure Component/NBT reason decoding is missing. Every failure payload
   terminates the epoch; it is **not** treated as an empty/successful setup.
2. No genuine Java/Rust differential codec harness or captured baseline transcript.
3. No integrated config-file, data-map, extensible-enum, feature-flag or split-packet
   task execution. Required unsupported payloads fail, never silently acknowledge.
4. No actual atomic application to Horizon's bidirectional content decoders:
   items/components, block states, chunks/palettes, entities, recipes, tags etc.
5. The companion bridge is not launcher-wired, a packaged mod, or a fully compiled
   Horizon integration. Engine runtime/framing tests still need full dependencies.
6. No isolated NeoForge server join, minimal fixture-mod join, or production test.

Both the Rust endpoint and Horizon patch deliberately refuse configuration
completion. No placeholder required handlers, forged Horizon acceptance, wildcard
21.1.x compatibility, or ~70-mod support claims are included.

## Read next

- `AGENTS.md` — mandatory development/commit policy and read order.
- `docs/PROGRESS.md` — current implementation boundaries and next work.
- `docs/WIRE.md`, `provenance/modded-codecs.md` — layouts, evidence and local limits.
- `integration/README.md` — source patch, license separation, tests and gaps.
- `docs/VALIDATION.md` — current and historical validation records.
- `docs/HANDOFF.md` — target inventory and future adapter constraints.

The repository remains private. See `LICENSE`, `THIRD_PARTY_NOTICES.md`, and
`integration/NOTICE.md`; the Horizon patch has separate licensing.
