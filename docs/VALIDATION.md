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
