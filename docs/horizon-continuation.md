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
regressions. Retained logs: `../integration/hosted-first-*.txt`.
Engine/TCP/default runtime results of the 1.97 run are still pending verification.
The client STILL CANNOT JOIN. Both finish gates remain. Actual host inventory,
semantic content application and remaining required tasks are not implemented.
See `../NEXT_AGENT.md` and `horizon-host-inventory-audit.md` before
continuing; peer/world enabled flags are not evidence of implemented local flags.
The owner explicitly requested public repository visibility for hosted CI.
All contradictory counts/"never compiled"/private wording BELOW are historical,
not the latest claim. No main changes, merges, force pushes or individual mods.

---
# Horizon integration continuation — 2026-11-27

## Outcome

Integration-only continuation, not full NeoForge handshake support. **Cannot join.**
Both core and engine finish refusal remain. No mods, launcher wiring, semantic
content host, commits, pushes or changes to repository visibility were made.
The other worker owns setup-failure/core codec changes; this report does not
claim that worker's validation as integration evidence.

Read-only GitHub verification confirmed private, open draft PR #1 at
`3331751dc938ba2fdd430dd28ead49d343f612b2`, branch
`feat/neoforge-21.1.248-early-handshake`, base `main` at
`5df54856212fcf66432edbbd6e08df56b438b3be`. The supplied compatibility directory is
an extracted source tree without `.git`, not a locally verifiable Git checkout.
No local commit/rebase/remote mutation was attempted.

## Real fixes

1. Endpoint sets failed state **before** invoking the external bridge, clearing
   it only after complete successful batch validation. A caught unwinding bridge
   panic can no longer retry/finish the same epoch. Executed regression checks
   catch the panic, reject subsequent receive and finish, and observe reset.
2. Payload-refusal diagnostic channel context now substitutes `<invalid-channel>`
   for invalid or >256-byte identifiers. Horizon's earlier identifier decoder
   permits much longer names; those no longer get reflected unbounded in this
   diagnostic. Valid bounded identifier context remains. Executed regression
   covers long and control-containing identifiers.
3. Extracted companion contract tests into a real shared Rust module: same
   tests now usable in isolation and inside the real client library. Added
   inventory mismatch/zero-hash refusal regression. No fake handler added.
4. Isolated bridge checker uses its **own temporary Cargo target**, compact
   profile and one job; it no longer writes or relies on the repository target.
5. Added `integration/check_full.py`: complete-pin/blob and disk preflight,
   disposable tree application, actual client typecheck, real companion module
   wiring, offline local-dependency-only lock audit, and nonempty-filter checks
   for companion/engine/TCP/default configuration tests. This driver is ready
   for a larger build host but **its Cargo path has not run here**.

## Actually executed

Rust 1.95.0, Linux x86_64 musl, bundled rust-lld:

- `integration/check.py`: all **14 provenance + 13 integration hashes**, exact
  upstream HEAD, patch application/reversal/byte restoration, four structural
  checks, **8 endpoint behavior tests + 1 actual pinned protocol-table test** pass.
- `integration/check_bridge.py`: **12 tests pass**: the same eight endpoint
  tests, plus four actual core/companion/endpoint tests. Not independent counts.
- Both above rerun after integration edits; logs are
  `integration/continuation-check.txt` and `integration/continuation-bridge.txt`.
- Rust formatting checks of all five patched Rust files and companion/test
  module pass. Python syntax compilation passes.
- Full-driver preparation helper exercised against actual pinned client
  manifest/root: wires exact companion and shared test bytes. Lock audit tested
  with actual pinned lock data plus the expected local addition; changed
  dependency versions and unexpected additions rejected. See
  `integration/continuation-tooling.txt`. This is **not a full Cargo build**.
- Actual `check_full.py` preflight ran and refused before build. See
  `integration/continuation-full-preflight.txt`.

## Why full compile/runtime checks are unexecuted

The shared persistent volume is **1 GiB** total. Public pinned Horizon archive
extraction and a subsequent filtered Git checkout each hit **ENOSPC**. The full
tracked source is 189,381,347 bytes / 12,354 blobs, before dependencies or build
artifacts. Checker's retained sparse pin has 28 verified blobs, 12,326 absent;
full preflight observed only 43,405,312 free bytes. The 3 GiB build reservation
is a conservative preflight policy, not a measured build upper bound.

The temporary failed extraction and downloaded blob pack were removed to release
space; the sparse pin and edited patch files remain for small reproducible checks.
No other worker's downloads/Java files or root source/test files were deleted.
Never present partial source as a complete checkout or dependency-free endpoint
compilation as the full engine. Full-driver refuses incomplete trees explicitly.

**Not executed:** full Horizon or full Horizon+companion typecheck, configure /
queued-writer runtime, actual TCP framing test, default vanilla/Horizon net
regressions, full Clippy, semantic registry/content application, isolated
NeoForge baseline/minimal-mod/live joins. No success inferred from test code
being included. No entire-handshake completion or play claim.

## Toolchain availability (scratch only; not redistributed)

Official Rust distribution channel manifest was verified against official
`.sha256`; each downloaded component against manifest SHA-256. Channel digest:
`821ff14e4c4a1cbe1e8915f35aff0a3fbbdf8d293ad48ab8f31e3b0440c581f9`.
Components were downloaded from `https://static.rust-lang.org/dist/2026-04-16/`.
Rustc: `1.95.0 (59807616e 2026-04-14)`; Cargo:
`1.95.0 (f2d3ce0bd 2026-03-21)`.

Usable compiler/Cargo/rustfmt binaries and musl standard library are retained
under the temporary toolchains directory. The scratch environment helper is
`toolchains/env.sh` relative to the shared temporary area; source that helper to
reproduce the small checks. **GNU host std was removed to recover quota**, so
normal host builds/build scripts do not work without restoring it. No C compiler
or native development headers were initially present. Full Horizon needs a
complete host toolchain, native dependencies and substantially more disk.
Java was unnecessary for this worker's integration-only checks; no Java install
was done here (independent codec worker manages its Java reference environment).

For follow-up use a complete clean pinned public checkout and a normal Rust
>=1.95 setup on a larger build host:

```sh
python3 integration/check_full.py "$HORIZON_PIN" --scratch "$NEW_SCRATCH_TREE"
```

Read `integration/README.md` for the original full-engine test command inventory,
license separation, trait scope and intentionally unsupported tasks. Keep both
finish latches until real required tasks and content semantics genuinely work.
