# Current milestone attempt — full owner not yet complete

Published runtime [`a1fdf5c`](https://github.com/horizon-rust-mod-rewrites/horizon-neoforge-compat/commit/a1fdf5c83a36f3035b94a12a9477c4cef19d097f)
adds detached cancellation, bounded absolute configuration policy and safe
explicit-profile Transfer refusal. Local checks pass; actual full-engine hosted
results await authorized inspection. See NEXT_AGENT.md and
`docs/validation-owner-a1fdf5c-local.md` for exact evidence and blockers.
**30/100, zero newly completed criteria; CANNOT JOIN.** Both finish refusals and
default split/flag opt-outs remain. Earlier successful runs validate their own
recorded source, not this new runtime patch.

# Current continuation validation — 46161098482 PASSED

Exact source `d0c05270057bc69da8082a229dc265f5d4929bdf`. See NEXT_AGENT.md and retained
`integration/hosted-46161098482-*.txt` for artifact digest, exact suite counts,
warning ledger and limitations. New runtime slice: bounded awaited early batch
writes and terminal physical closure/reset; full owner remains incomplete and no
additional checklist points are awarded. **30/100, CANNOT JOIN**, both finish
refusals/default split and flag opt-outs remain. Older checkpoints below do not
supersede this exact-SHA result.

--- Historical validation records ---

# Latest exact-SHA hosted validation: focused fixes PASSED

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
- Progress-metadata push/PR workflows passed with 28 validator tests. New focused
  additions cover malformed metadata shapes and the exact inbound split-part limit.

Default split/feature-flag opt-ins remain disabled. Both finish refusals remain.
**The client STILL CANNOT JOIN.** Production split routing, complete loader tasks,
semantic content application, UI and live join remain unproven. See NEXT_AGENT.md
for current continuation priorities. Older checkpoint sections below are historical.

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
# Current development validation (2026-11-27)

Supersedes the historical 0.1 status below. Main crate: **90 tests passed in
both debug and release**, Rust 1.85.1 / x86_64 musl. Strict Clippy, formatting,
API docs, fixture checks and upstream verification passed. Recorded current
commands/output: `validation-current.txt`. New coverage: 14 modded-codec tests,
10 selection tests, 10 integrated loader tests and 23 registry lifecycle tests,
in addition to the original 33. Synthetic fixtures are not real transcripts.

Horizon patch checker: 4 structural checks, 6 Rust endpoint tests, 1 actual
protocol-table test, patch application/reversal and provenance checks passed.
Companion compilation: 9 tests (the same 6 endpoint tests plus 3 cross-component
checks) passed. Do not sum these as independent endpoint tests.
See `../integration/validation.txt` and `../integration/bridge-validation.txt`.
Exact MC enum classfile audit passed; no genuine Java codec execution occurred.

**Not validated:** full Horizon compilation/typecheck (offline missing dependency;
pinned workspace also needs Rust 1.95), included full-engine/TCP tests, live
NeoForge server join, real semantic registry application. Setup-failure NBT
Component decoding and required loader tasks remain missing. All finish paths
still refuse; this is not a completed handshake or a joining client.

---
# Historical milestone 0.1 record

# Validation and reproducibility

## Actually executed

- Rust **1.85.1 (4eb161250 2025-03-15)**; Cargo 1.85.1; Linux x86_64 musl target.
- Public official Rust distribution downloaded and checksum-verified. Components
  were extracted without executing a repository or tooling install script.
- Rust's bundled `rust-lld` used; no native system C compiler was available.
- Debug and release test suites: **33 passed, 0 failed** in each final run.
- `cargo clippy --all-targets -- -D warnings`: passed on that target.
- `cargo fmt --check`: passed. Rust API docs built successfully.
- Fixture generator `--check`: six literal synthetic fixtures matched.
- Public official NeoForge source artifact and 18 source-file hashes reverified
  against `provenance/neoforge-21.1.248.json` and publisher checksum.
- An initial native-link attempt failed (`cc` unavailable); it is not counted
  as a passing platform. Source and final tests were validated via musl instead.

The final transcript is `validation.txt` (absolute project paths normalized to `<project>`). No test result was inferred from
code inspection. Ordinary GNU/native toolchains, other operating systems and
other Rust versions were not executed here.

## Reproduce with a normal musl-capable Rust installation

Install Rust 1.85.1 and its x86_64-unknown-linux-musl standard library using your
trusted normal toolchain manager. Supply a suitable linker; if using bundled
rust-lld explicitly, point `CARGO_TARGET_X86_64_UNKNOWN_LINUX_MUSL_LINKER` at its
installed executable. Do not copy host-specific paths from another machine.

```sh
cargo fmt --check
cargo test --offline --locked --target x86_64-unknown-linux-musl
cargo test --offline --locked --release --target x86_64-unknown-linux-musl
cargo clippy --offline --target x86_64-unknown-linux-musl --all-targets -- -D warnings
cargo doc --offline --no-deps --target x86_64-unknown-linux-musl
python3 fixtures/generate.py --check
python3 provenance/verify_upstream.py
cargo run --offline --target x86_64-unknown-linux-musl --bin compat-inspect -- common-demo
```

The package has no third-party Rust dependencies. Cargo.lock is included. A
source-only archive is shipped: no compiler, prebuilt native code, target cache,
upstream Java code, server/client jar or game assets. Generated API documentation
can be reproduced locally; the archive contains human-written handoff docs.

## Test interpretation and limits

Core tests cover golden bytes, sparse ID 300 VarInt, signed -1 VarInt, invalid
length/count/UTF-8/identifier/VarInt inputs, trailing bytes, every truncation of
registration and snapshot examples, duplicate metadata, alias cycles/dangling
aliases, body/writer budgets, state ordering, failed-epoch retention, unsupported
loader tasks, exact build gating, optional receiver registration and atomic map
construction. CLI tests exercise offline inspection, no-join output and usage
errors. The deterministic xorshift corpus uses 4,096 byte strings across four
decoders; it is a smoke corpus, **not** coverage-guided fuzzing, a proof of memory
safety, or a substitute for adversarial protocol review.

Synthetic fixtures were specified independently as literal hex in Python, then
compared against Rust codecs. There has been **no Java differential execution**,
real packet capture comparison, Horizon compilation, socket/transport integration,
mod loading, registry application in a world, or live-server test.

The source artifact includes NeoForge's wire-codec compositions. Minecraft's
primitive-codec implementation was not downloaded or executed. The Rust
primitive subset follows standard Minecraft VarInt/string/ResourceLocation
conventions and uses explicitly documented stricter byte/grammar limits; broader
primitive equivalence still needs the future differential harness.

## Official tooling hashes (not redistributed)

Rust 1.85.1 x86_64-unknown-linux-gnu distribution:
`https://static.rust-lang.org/dist/2025-03-18/rust-1.85.1-x86_64-unknown-linux-gnu.tar.xz`
SHA-256 `912c76f602f5f630138fca2388b2688c2c1aabcf823d71890b2b9155f743792e`.

Musl std component:
`https://static.rust-lang.org/dist/2025-03-18/rust-std-1.85.1-x86_64-unknown-linux-musl.tar.xz`
SHA-256 `a2a99b60e86214a836e53396f3a0716a48724902b9dd8c69f146897efc76691e`.
