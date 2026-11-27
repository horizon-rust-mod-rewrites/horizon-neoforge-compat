# Credit-conscious continuation: start here

Continue existing draft PR #1, branch
`feat/neoforge-21.1.248-early-handshake`; always verify current remote HEAD.
Read AGENTS.md and linked handoffs, then this file. Keep every commit attributed:

    AI-Assisted-By: QX AI assistant

## Companion inventory-boundary regression checkpoint

Added five actual companion contract tests: unavailable host inventory offers
nothing; peer announcements cannot enable feature flags/split/required tasks;
peer-selected flag pairs cannot forge a host attestation; actual bridge reset is
idempotent/terminal with fresh independent construction; inventory is checked
BEFORE invoking registration callbacks. Default behavior unchanged, no invented
inventory or enabled flag profile, both finish gates unchanged.

Local: 28 Python driver tests, six fixture checks and diff whitespace checks pass.
New Rust tests are NOT yet executed locally (no compiler installed); hosted CI
must compile/run them in both isolated and full-engine companion filters. Prior
hosted success below applies only to its tested SHA, not these new tests.

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
## Documentation synchronization checkpoint

Main read-order docs now start with executed hosted evidence, distinguishing
full library typechecking from linked tests/runtime, and explicitly demoting
contradictory historical counts/private status. No Rust, patch, workflow or
behavior changes. Verified documentation links and `git diff --check` locally.
The 1.97 run https://github.com/horizon-rust-mod-rewrites/horizon-neoforge-compat/actions/runs/46079113415
is not complete at the last observed job status; no success is inferred.

## First executed full-host build: typecheck PASSED, tests toolchain-blocked

Actual run: https://github.com/horizon-rust-mod-rewrites/horizon-neoforge-compat/actions/runs/46065611717
(commit `5ff746f78e9ff9524e6c344127182044bebb2943`, Ubuntu GNU, Rust 1.95.0).
Disk was ample: ~89.6 GB free; complete exact upstream pin verified all 12,354
blobs with zero missing. BOTH full patched horizon-client-lib typechecks passed,
including the actual companion; audited lock change added only the local core.
This supersedes historical claims that full compilation was never reached.

The FIRST full client test invocation was refused before compilation because
locked test-only Wasmtime 50 / Cranelift 0.137 require Rust **1.97.0**, despite
upstream workspace declaring 1.95 minimum. Do not downgrade/update the pinned
dependency graph or use --ignore-rust-version. Hosted toolchain now upgraded to
exact 1.97.0 to satisfy the locked graph. Subsequent run remains unverified.
Full-engine test compilation, configure/writer runtime, TCP and default net
regressions DID NOT execute in this first run. No joining/content claim.

Executed in the first run: core 153 tests in EACH debug/release, strict Clippy,
formatting, warning-denied docs; 15 then-present Python tests; six fixtures;
27 source hashes, patch/reversal/restoration; 8 isolated endpoint tests, actual
protocol-table test; 12 isolated companion tests (including those same endpoint
cases; do not sum). Text evidence retained here as `integration/hosted-first-*.txt`,
not just seven-day Actions artifacts. Both finish gates and production code unchanged.

## Full-driver regression safety checkpoint

Added 13 dependency-free synthetic lock/wiring regression tests (28 Python tests
now pass together). These retain the actual companion/test-module bytes, verify
explicit local path wiring, and reject changed dependency versions, checksums,
sources, removed packages, unexpected additions, registry substitution or newly
introduced dependencies. These are tooling tests, NOT Cargo/engine validation.
Transcript: `integration/validation-full-driver-contracts.txt`.

Read-only exact-pin host audit also confirms that received enabled-feature lists,
world/candidate flags and generic flag-condition evaluators are NOT authoritative
inventories of implemented modded flags. `FeatureFlags::iter()` even synthesizes
vanilla on empty backing storage. Do not replace unknown inventory with empty,
or use the server's enabled list/pack fingerprint as local support evidence.
Companion opt-in stays disabled pending a genuine host-owned capability snapshot
bound to the actual implementation/profile lifetime; see
`docs/horizon-host-inventory-audit.md`. Both finish gates stay unchanged.

## Hosted workflow parser correction

Initial run https://github.com/horizon-rust-mod-rewrites/horizon-neoforge-compat/actions/runs/46065494193
failed BEFORE any job was allocated: job-level env cannot reference `runner`.
Corrected the three job-level paths to workspace siblings using `github.workspace`
(which is valid there); step-level `runner.temp` remains valid. This initial
failure is workflow parsing only, NOT compilation evidence. Added actionlint to
local validation beyond YAML/bash parsing; hosted build still pending.

## Hosted full-build checkpoint (2026-11-27)

Owner explicitly requested public visibility (now public), then ongoing work
with frequent attributed commits on this SAME draft PR, without pausing after
each checkpoint. This supersedes historical private-only wording; do not change
visibility again without a new owner request. Remote start verified at
`5591f0bd33ca9160e073ab4fc05f9e36f498ee7c`.

Added `.github/workflows/horizon-full.yml`: standard public GitHub ubuntu-24.04
runner only, 90-minute timeout, read-only token, no stored checkout credentials,
SHA-pinned checkout/artifact actions, no secrets, no paid larger runner. Checks
10 GiB free before large inputs, installs Rust 1.95.0/native headers, reruns core
and Python safety tests, retrieves the COMPLETE exact Horizon pin, checks patch
and isolated bridge, fetches locked host-target dependencies INCLUDING dev crates,
and invokes the existing full integration driver. Its logs only (no game data or
source archives) are retained as a seven-day artifact. Workflow triggers only on
relevant source/integration/workflow pushes to this branch, not doc-only updates.
No merges, force pushes, main changes, fake tasks or finish-gate removal.

Local executed evidence: workflow YAML and every bash snippet parsed; read-only
permissions/standard runner checked; 15 Python tests and six synthetic fixtures
pass; `git diff --check` passes. Full hosted run is PENDING, not passing evidence.
No large local downloads retried on the still-1-GiB machine. Review actual Actions
results and retained `ci-evidence/full-check.txt` before any success claim; fix
real compilation/runtime failures in small tested commits. Full engine/companion,
TCP and Rust reruns are NOT yet executed at this checkpoint. Client cannot join.

## Disk-first continuation checkpoint (2026-11-27)

Remote draft PR #1 HEAD was verified at
`17058127b69ef9e9077843b905808dbc8fd72873` before this checkpoint; no newer work
was present then. Verify it again before continuing. The current machine is
still capped at **1 GiB total disk**, below even the existing 3 GiB build
reservation. No full Horizon source/toolchain/dependency downloads were retried.
**Full patched Horizon + companion compilation and engine/TCP/default regressions
remain NOT EXECUTED. The client STILL CANNOT JOIN.**

Focused milestone: `integration/check_full.py` now supports a read-only
`--disk-preflight-only` check WITHOUT an upstream checkout, so the next worker
can refuse an undersized machine before downloading anything large. Normal mode
also checks disk before source verification, and rechecks with actual source-copy
bytes before copying. Nonfinite/too-small reservations are rejected. Defaults,
pins, patch, companion, Loader, inventories and both finish gates are unchanged.

Executed: **15 dependency-free Python safety tests**, actual disk-only AND normal
entry-point low-disk refusals (exit 1, no scratch tree), syntax checks, and all
six existing synthetic fixtures. Tests mock filesystem sizes/source verification
and forbid subprocesses; they are NOT full engine or Rust validation. The prior
153 debug/release Rust tests and companion/Clippy results were NOT rerun in this
checkpoint. Evidence: `integration/validation-disk-preflight.txt`.

First on the next machine (scratch parent must exist):

```sh
python3 integration/check_full.py --scratch "$NEW_SCRATCH_TREE" --disk-preflight-only
```

A pass is only a lower-bound disk check, NOT proof the build will fit. Do NOT
lower the reservation to squeeze this 1 GiB machine through. Provision ample
additional disk for the complete source, Rust >=1.95 host/native toolchain,
dependencies and retained build outputs; then obtain the clean pinned Horizon
checkout and run the existing full path:

```sh
python3 -m unittest discover -s integration -p 'test_check_full.py' -v
python3 integration/check_full.py "$HORIZON_PIN" --scratch "$NEW_SCRATCH_TREE"
```

After actual full-build evidence, follow priorities below for genuine Horizon
host inventory/profile wiring and remaining tasks. Existing Java query/setup
research is completed; do not repeat it. No production server tests or individual
mod rewrites. Keep the same draft PR and mandatory AI commit footer.

Current feature-flag checkpoint: bounded codecs + exact host-inventory equality
are wired into explicit opt-in `Loader::new_with_feature_flags`. Both server-
selected channel directions must be present or both absent. Only exact equality
returns a validated zero-byte ack; mismatch/ordering/direction errors poison the
epoch and revoke grants. `feature_flags_checked()` is task-local evidence only.
153 core/driver tests pass debug/release, strict Clippy, formatting, warning-denied
docs, plus the 12-test isolated companion regression. See docs/FEATURE_FLAGS.md.
Existing constructor/companion DO NOT assume or provide a host inventory.
No individual mods, main-branch updates, merges, force-pushes or visibility changes.

Resource-aware priorities:
1. Full Horizon + companion build and runtime regressions remain FIRST on a
   sufficiently large build machine. Use integration/check_full.py and read
   docs/horizon-continuation.md. Previous environment could not hold the full
   checkout/dependencies; do not repeat large downloads without checking space.
2. Feature flags are now opt-in core-wired. Next supply an ACTUAL Horizon host
   inventory and deliberate companion profile wiring before enabling it there.
   Nonempty flag names are not game semantics. Java differential feature-flag
   coverage is not yet added; query/setup reference execution already exists.
3. Then remaining config/data-map/extensible-enum tasks, safe split transport
   integration, and real atomic bidirectional registry/content application.
   Follow actual dependencies; numeric ID maps are not semantic content support.
4. Isolated exact NeoForge baseline, then minimal fixture mod, never production
   first. Remove neither finish latch until required behavior genuinely works.

Historical pre-feature core evidence: 128 main/driver tests debug/release, strict
Clippy, warning-denied docs, fixtures, upstream verification. Actual Java query/
setup equivalence is already done; do not redo that research without cause.
The client STILL CANNOT JOIN. Full engine/TCP, semantic publication and live join
checks are unexecuted, not implicitly passing. Save small coherent commits and
update this file after each milestone. Do not depend on another session's local
compiler/download/source directories surviving.
