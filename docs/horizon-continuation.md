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
