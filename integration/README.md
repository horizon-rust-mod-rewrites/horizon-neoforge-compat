# Horizon early-configuration patch — bootstrap only, hard finish refusal

Target: public Horizon `ae632edf48f6cae3ba52a5c81523ec712488c9cc`, Minecraft
1.21.1 / protocol 767, exact caller-attested NeoForge 21.1.248 only.

**Applicable source patch, not a mod. Cannot complete configuration or join.**
The engine's hard finish latch is intentional: even an
optimistic bridge cannot enter play without the missing bidirectional modded
content conversion boundary. Do not remove it merely because negotiation codecs
pass tests. There is no production placeholder Bridge in the patch.

## First hosted result

Run [46065611717](https://github.com/horizon-rust-mod-rewrites/horizon-neoforge-compat/actions/runs/46065611717)
verified the complete pin and passed BOTH full patched-client typechecks,
including actual companion wiring and lock audit. Rust 1.95.0 then refused the
first full test build: locked Wasmtime/Cranelift test dependencies need 1.97.0.
The workflow now installs 1.97.0 without changing upstream dependencies. Engine
runtime/TCP/default regressions remain unexecuted in that run. Core 153 tests per
profile, Clippy/docs/fmt and isolated checks passed on GNU. Exact retained
transcripts: `hosted-first-*.txt`; see NEXT_AGENT.md. No finish gate removed.

## Hosted full check

`.github/workflows/horizon-full.yml` runs on a standard public GitHub Ubuntu
runner, with Rust 1.97.0 (locked test dependencies require it), disk checks BEFORE downloads, complete pinned upstream
source, locked host-target test dependencies and the existing full driver. Its
read-only job retains textual evidence for seven days (never source/game data).
A workflow definition is NOT a passing result. Inspect each run and its
`full-check.txt` artifact; full compilation/runtime remains pending until an
actual run completes successfully. No join or content-semantic evidence is
provided by this workflow. See NEXT_AGENT.md for the current run status.

## Disk-first follow-up checkpoint (2026-11-27)

The next machine again has only **1 GiB total disk**. No full source/toolchain/
dependency downloads were repeated; **full compilation and engine/TCP/default
regressions remain unexecuted**. This checkpoint changes only driver safety,
tests and handoff evidence, not the patch or production bridge/core behavior.

Before obtaining the complete pinned upstream checkout, check the intended
scratch filesystem without creating a tree or invoking Git/Cargo:

```sh
python3 integration/check_full.py --scratch "$NEW_SCRATCH_TREE" --disk-preflight-only
python3 -m unittest discover -s integration -p 'test_check_full.py' -v
```

The scratch parent must already exist and the scratch path must not exist.
Disk-only mode does not inspect an optional source argument and makes no source,
toolchain, dependency, build, engine/TCP or join claim. Its default 3 GiB lower
bound omits the not-yet-downloaded source, download caches and toolchain; allow
ample extra space. Passing is NOT a build-size guarantee. Never reduce the
reservation just to bypass a known undersized machine. Normal mode now performs
this cheap check first, then repeats it with actual source-copy bytes after the
existing pin/blob verification. All build commands and lock auditing remain.

Executed here: **15 Python safety tests**, actual low-disk refusal from both CLI
modes, Python syntax checks, six existing synthetic fixtures. Driver tests use
mock disk sizes/source verification and forbid subprocess execution. These are
not engine or Rust/companion tests; previous Rust results were not rerun.
See `validation-disk-preflight.txt` and updated `../NEXT_AGENT.md`.

## Continuation validation (2026-11-27; supersedes historical counts below)

- Official checksum-verified Rust **1.95.0** installed; actual executed target is
  Linux x86_64 musl. No native GNU or full-engine build is claimed.
- `continuation-check.txt`: all 27 pinned-source hashes, patch/reversal/restoration,
  four structural checks, **8 endpoint tests + 1 actual packet-table test** pass.
- The isolated companion checker now uses a private temporary build target, compact
  debug profile and one build job, never the repository target.
- `continuation-bridge.txt`: **12 tests pass (the same 8 endpoint + 4 companion
  contract tests)**. Shared `bridge_contract_tests.rs` runs against the actual
  Loader/glue/endpoint; these counts must not be summed as independent tests.
- Fixed two integration boundary issues: caught unwinding external bridge panics
  cannot resume the epoch; invalid/overlong identifiers cannot be reflected in
  payload-refusal diagnostics. New executed behavior tests cover both.
- Added `check_full.py` to copy/verify a complete clean pin, apply the patch,
  typecheck the engine, then wire this exact companion **inside the actual
  client library**, audit its local-only lockfile change, and run companion,
  configure/writer, TCP and default configuration regressions. It refuses empty
  test selections. It never edits the input trees or commits/pushes anything.
- **Full-driver Cargo execution was NOT reached.** Its real preflight refused:
  the shared volume is capped at 1 GiB and full source extraction/checkouts had
  already failed with ENOSPC. A verified sparse pin is retained for small tests;
  this is not a complete working tree. See `continuation-full-preflight.txt`.
  Python syntax, preparation and lock-audit helpers ran (`continuation-tooling.txt`),
  not the full Cargo subprocess path. No dependency/engine/TCP result is invented.

On a machine with a normal complete Rust >=1.95 installation (including host
standard library and native build dependencies), a complete clean source pin,
network dependencies and several GiB available:

```sh
python3 integration/check_full.py "$HORIZON_PIN" --scratch "$NEW_SCRATCH_TREE"
# Optional --target TARGET; the host standard library is still needed.
# The scratch tree and full-check.log are retained on failure.
```

The disk reservation defaults to 3 GiB after source-copy space; this is a
conservative preflight reservation, **not a measured upper bound** for Horizon.
Full Horizon/companion typecheck, engine/TCP/default regressions, semantic content
application and live joins remain unvalidated here. Both finish latches remain.
No individual mods were changed. See `../docs/horizon-continuation.md` for toolchain
availability, source/PR identity and executed-versus-unexecuted checks.

## Historical milestone record

## Integration status

Based on verified compatibility commit `5df54856212fcf66432edbbd6e08df56b438b3be`.
The source patch leaves the original Horizon audit checkout unchanged. It is
reviewable development infrastructure, not a packaged launcher feature.
`compat_bridge.rs` connects the actual core Loader to the patched endpoint;
`check_bridge.py` compiles that real combination in isolation. **Neither patch
nor companion has been compiled with Horizon's full dependency graph.**

## What the patch changes

- Adds engine-owned `net::early_configuration` with explicit `Profile`, `Factory`,
  `Bridge`, body-only `Reply`, and stable-code `Failure`. No dependency on this
  compatibility crate in the engine; the trait is the extension boundary.
- Adds `Session::connect_neoforge(..., profile)`; existing connect paths retain
  their prior default behavior. Explicit mode pins `Connection::adapt(V1_21_1)`
  before login rather than relying on status inference or older-server settings.
- Constructs a fresh endpoint before Horizon negotiation. Suppresses the initial
  brand, Horizon payload/build offers and every Horizon capability branch in
  explicit mode, including when the peer claims the `horizon` brand. A received
  brand is only reported to the game, never treated as loader/build attestation.
- Configuration custom payloads go to the early route **before** all Horizon
  special cases and generic application dispatch. Routes all `neoforge:*`, common
  and Dinnerbone channels plus explicit adapter channels; all other custom
  channels except engine-owned brand are refused. The adapter allowlist never
  expands because a registration list mentions a channel. Unknown packet variants
  reaching configure are refused in this mode too.
- Does not replace vanilla keepalive, ping, cookie, resource-pack, registry, tag,
  known-pack, transfer or disconnect handling. Registry handling is still vanilla
  staging, NOT application of NeoForge semantic maps.
- Uses body-only replies through the existing queued writer and protocol adapter,
  exactly once. Validates the entire returned batch before queuing its first
  packet; maximum 32 replies, 65,536 total body+channel bytes, 32,767 bytes per body.
  Inputs use the same conservative body cap (not upstream's full registry limit).
- Refuses finish before Horizon epoch acceptance, writer phase transition or ack.
  On refusal the normal run unwinds and shuts down; no next read into play. It
  never fabricates Horizon Accepted metadata. A stack guard calls `reset()` on
  every endpoint exit (failure, EOF, disconnect, transfer, or barrier refusal).

The pre-existing Horizon payload transport is still captured at session startup;
its **accepted** epoch is never opened in explicit mode. Ordinary mod packets
therefore get no new send authority. No generic ownership checks are disabled.
The profile has no CLI/UI/global registration and is not automatically inherited
on transfer/reconnect. Callers must reselect and attest each destination. Because
finish is always refused, production explicit mode cannot reach reconfiguration;
the isolated engine test also exercises a second configuration invocation to
check fresh construction/reset. Existing default reconfiguration is unchanged.

## Bridge contract

The public trait has three mandatory methods (no success defaults):

- `receive(channel, body) -> Result<Vec<Reply>>`: synchronous bounded processing
  on the network thread. A required unimplemented/unknown task must fail. Only
  send real handler-backed replies selected by the server's negotiation. Common
  or Dinnerbone registration does not grant modded channels. The engine owns
  channel/packet framing; bodies contain neither prefix.
- `finish() -> Result<()>`: verify all required task/handler/mapping/main-work
  conditions. A bridge error is propagated first; if it returns success the
  engine still fails with `modded_play_boundary_not_implemented`.
- `reset()`: discard all epoch-local negotiation, pending work, staged maps and
  grants. Never retain mutable state across factory-created connections. It is
  also called when a successful bridge finish meets the engine's hard latch.

`Factory::create(pack_sha256)` returns a **new independent** bridge per
configuration. A failed factory must clean up its own partially constructed
state. `Profile::neoforge` validates exact baseline, nonzero inventory hash and
bounded, unique, nonreserved, syntactically valid adapter IDs (max 1,024).
An inventory hash is supplied out-of-band and is not authenticated by this seam.
A declared adapter ID is routing authority, **not** evidence of an implemented
required handler: the real bridge must enforce phase/flow/version and acceptance.

`Failure(&'static str)` deliberately exposes only a stable diagnostic code. The
engine adds `configuration/<channel>` or `configuration/finish` context. The
sample may map a rich core `Error` to a static transport failure and separately
record a **sanitized** full structured error in private local diagnostic logging.
Do not log raw payloads, cookies, auth/session tokens, account details or pack
secrets; bound/escape any peer-controlled identifiers or strings before logging.
Retain the original error locally for debugging rather than putting it verbatim
in a public disconnect reason. The sample and logging policy must be honored by integrators.

No asynchronous/main-thread completion API, accepted-channel play delivery,
registry commit hook or content translator is supplied here. Synchronous bridge
work must be bounded; pending asynchronous/main work must cause finish refusal,
not block the receive loop or be acknowledged speculatively.

## Apply to a clean scratch checkout

Use a disposable checkout of the exact source pin, not the shared audit checkout:

```sh
git -C "$HORIZON_COPY" rev-parse HEAD
# expected: ae632edf48f6cae3ba52a5c81523ec712488c9cc
git -C "$HORIZON_COPY" apply --check "$COMPAT/integration/horizon-early-configuration.patch"
git -C "$HORIZON_COPY" apply "$COMPAT/integration/horizon-early-configuration.patch"
```

`make_patch.py CLEAN_PIN EDITED_COPY` is an optional dev-only regeneration script;
it writes only this contribution's patch and does not apply it. Do not regenerate
from unreviewed or mismatched source. `check.py` verifies the 14 existing
provenance hashes plus 13 extra source/guidance/license hashes before using them.

## Automated checks actually executed

```sh
python3 integration/check.py "$HORIZON_PIN" --rustc "$RUSTC" \
  --target x86_64-unknown-linux-musl --linker "$RUST_LLD"
```

On Rust 1.85.1 using the musl standard library and bundled linker:

- exact HEAD and **all 14** `provenance/horizon.json` hashes matched;
- 13 additional integration/source/guidance/license hashes matched;
- clean patch application, duplicate application refusal, reverse applicability,
  byte-exact restoration, and four **structural** checks passed;
- six actual dependency-free Rust endpoint behavior tests passed;
- one Rust test of the **actual pinned generated protocol-767 packet table**
  passed (both directions, including a nonidentity known-pack clientbound ID);
- changed Rust files were formatted/parse-checked by rustfmt 1.85.1 with edition
  2024, without recursively rewriting unchanged upstream files.

`validation.txt` is the actual checker output. The checker only copies files it
needs to a temporary directory, never patches its input tree, and rechecks input
provenance after completion. Structural checks are external patch-ordering
contracts, explicitly not runtime engine evidence or upstream parity harnesses.

## Companion check — executed successfully

After the pin/application checker, run the companion against the **actual core
Loader, actual companion glue, and actual patched endpoint**:

```sh
export PATH="$RUST_BIN:$PATH"
export CARGO_TARGET_X86_64_UNKNOWN_LINUX_MUSL_LINKER="$RUST_LLD"
python3 integration/check_bridge.py
```

`RUST_BIN` is the directory containing your Rust executables; `RUST_LLD` is your
trusted musl-capable linker executable. The companion defaults to the
`x86_64-unknown-linux-musl` target. This command was rerun successfully here:
**9 tests passed (6 endpoint + 3 cross-component)**. Actual output is recorded in
`bridge-validation.txt`. Cross-component tests exercise modded query/setup,
common version handling, required-task/failure refusal, reconnect cleanup and
finish refusal. The main crate passes **90 tests in debug and release**; its transcript is
`../docs/validation-current.txt`.

This is a real isolated compilation of the endpoint/bridge/core contract, **not
compilation of Horizon's full dependency graph**, configure loop, queued writer,
TCP framing, or a live join. Full configuration finish remains a hard refusal in
both the core and engine. The companion source and checker retain their own MIT notices.

## Included but NOT executed full-engine tests

The patch includes:

1. `early_configuration_net_tests`: isolated actual configure/queued-writer path,
   synthetic protocol-767 memory transport, early c:version reply, Horizon-brand
   offer suppression, keepalive/ping service, no accepted Horizon epoch, no finish
   ack, no read beyond finish, and fresh reset on a second invocation.
2. `neoforge_configuration_framing_tests`: actual localhost **TCP** connection
   adapter/framing in both directions. Checks the rewritten 767 handshake,
   opaque custom-payload bytes (identifier prefix exactly once), serverbound
   packet ID 2/clientbound 1, and nonidentity known-pack mapping 14→15. Uses an
   abbreviated synthetic login-state transition, **not** a real login/loader
   transcript. Memory Pipe uses different framing and is not TCP evidence.

With a suitable current toolchain, native headers and all dependencies available,
run in the patched scratch Horizon checkout:

```sh
cargo check --locked -p horizon-client-lib --no-default-features
cargo test --locked -p horizon-client-lib --no-default-features --lib early_configuration
cargo test --locked -p horizon-protocol --lib neoforge_configuration_framing_tests
# Existing default vanilla/Horizon configuration regression tests, including reconfiguration:
cargo test --locked -p horizon-client-lib --no-default-features --lib net::tests::
```

**These commands are required follow-up, not passing results.** The actual offline
check failed during dependency resolution: `bytemuck` absent from the local
registry (`cargo-check.txt`). Available rustc is 1.85.1; the pinned Horizon
workspace additionally declares **Rust 1.95 / edition 2024**, so it needs a newer
compiler as well. No dependencies/toolchains were downloaded to fake a build;
no full engine typecheck, Clippy, engine runtime test, Java differential test,
real loader transcript, live server join, or game-content test was performed.
Full Horizon + companion bridge compilation remains unverified; the isolated
actual-core/companion/endpoint compilation below passed. Packet-table evidence
alone does not establish actual TCP byte preservation; the TCP test must run.

## Licensing

See `NOTICE.md` and `LICENSES/`. The Horizon patch (including added Rust source and
Rust tests) is **LGPL-3.0-or-later**, not covered by the compatibility repository's
blanket MIT statement. Unchanged LGPL and incorporated GPL texts are included.
Python checker/regenerator code is MIT under the compatibility repository license.
The shared checkout remains unchanged; the patch is the corresponding editable
modification source. No Minecraft jars, assets or decompiled code are included.
