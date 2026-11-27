# Hosted validation status — supersedes historical records below

The full patched Horizon client library **and actual companion passed Cargo
TYPECHECKING** on the complete pinned source in hosted run
[46065611717](https://github.com/horizon-rust-mod-rewrites/horizon-neoforge-compat/actions/runs/46065611717).
This is not linked test-binary compilation or runtime evidence. The first full
engine test build was refused because locked Wasmtime/Cranelift dev dependencies
require Rust 1.97, above the workspace's declared 1.95 minimum. CI now uses exact
1.97.0; do not alter the upstream lock graph or ignore its compiler requirements.

That first GNU run also passed 153 core tests in EACH debug/release, strict Clippy,
format/docs, synthetic fixtures, source/patch checks and isolated companion
regressions. Retained logs: `integration/hosted-first-*.txt`.
Engine/TCP/default runtime results of the 1.97 run are still pending verification.
The client STILL CANNOT JOIN. Both finish gates remain. Actual host inventory,
semantic content application and remaining required tasks are not implemented.
See `NEXT_AGENT.md` and `docs/horizon-host-inventory-audit.md` before
continuing; peer/world enabled flags are not evidence of implemented local flags.
The owner explicitly requested public repository visibility for hosted CI.
All contradictory counts/"never compiled"/private wording BELOW are historical,
not the latest claim. No main changes, merges, force pushes or individual mods.

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
