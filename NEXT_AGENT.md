# Credit-conscious continuation: start here

Continue existing draft PR #1, branch
`feat/neoforge-21.1.248-early-handshake`; always verify current remote HEAD.
Read AGENTS.md and linked handoffs, then this file. Keep every commit attributed:

    AI-Assisted-By: QX AI assistant

## Confirmed reconstructed-decoder hosted success

Run https://github.com/horizon-rust-mod-rewrites/horizon-neoforge-compat/actions/runs/46083513758
completed SUCCESS for tested SHA `89adc878a4d96b1ccb102326f3c73f3a5d39224e`.
Downloaded artifact SHA256 verified:
`e0e69113821562dfe0f3a210079586c69bd9af957f0b3385275fe287911db86d`.
Retained `integration/hosted-46083513758-{full-check,full-driver,scope,core}.txt`.
Actual full patched client plus companion typechecks and selected tests passed:
**14 companion, 9 early configuration, 3 protocol (TCP + decoder), 81 network
filter / 3 ignored**. Core still 153 tests EACH debug/release; strict Clippy,
fmt/docs, 35 Python and six fixtures passed. This is selected suites, NOT entire
upstream tests or engine UI/live join. Both finish gates remain; CANNOT JOIN.

Run 46082288102 also completed SUCCESS for earlier SHA
`70ba5a4737d2fcb669738ee49dac5dcdb330939f`: 9 companion, 9 early, 1 TCP,
81 network / 3 ignored; validates original five boundary tests, floors/lock audit.
Artifact SHA256 verified:
`b35fb58b461ac0d6b3ec41cfbd659100f3e190108a79e9c7f6a701beeac38cde`.
Retained its own full-check/full-driver/scope/core transcripts, not overwritten.

Newer terminal ingress fix SHA `7d053a65902af967c053ab65c32c6d59c058d444`
requires its OWN run 46084373021, last pending. Its three additional protocol
tests and 6-test floor are NOT proven by the 3-test older success. Inspect actual
jobs/logs before advancing claims. Bounded outbound fragmentation work is under
investigation separately; no enabled split/flag or invented host inventory.

## Terminal decoder ingress hardening checkpoint

Follow-up source review found that reconstructed-packet rejection poisoned recv
and buffered reads, but public read_from could still consume caller-owned input,
refill buffers and increment counters. read_raw allocated before finding the
closed stream, and zero-length raw reads could succeed. Both now refuse before
I/O/allocation. Added three actual dependency-local tests covering external/raw
terminal reads, preservation of prefetched packet bytes/counters on success, and
real-codec malformed/trailing/count rejection with attached transport shutdown.
Full-driver floors are now 14 companion / 6 protocol / 9 early / 81 network.
These new Rust tests are NOT executed locally. Both finish refusals and default
split/flag refusals remain. Client STILL CANNOT JOIN.

Executed locally: 35 Python tests, six fixtures, syntax, whitespace/checksums;
narrow exact-source hash and patch apply/reverse/restoration/regeneration checks.
Last current-head run 46083513758 (SHA 89adc878a4d96b1ccb102326f3c73f3a5d39224e)
had no jobs yet when inspected. Older 46082288102 was still in its full engine
step, with preflight/core/provenance/isolated checks successful. A background
CI inspection is retaining completed/failing evidence; no new full success is
inferred. This commit requires its own SHA's run; verify remote before edits.

Detached socket caveat: the socket returned by Connection::detach belongs to its
caller; reader poisoning cannot shut that externally owned socket or independent
writer. Future production wiring must keep a real closer and reset Loader on
all failures. No production split ingress/timer/authority/queue route is enabled.

## Concrete split decoder seam checkpoint

Substantive six-file Horizon patch now adds a concrete protocol-reader boundary
for reconstructed protocol-767 configuration originals. It shares actual packet
ID extraction with ordinary socket reads, requires both configuration phases,
allows only opaque payload/keepalive/ping/known-packs, adapts once, and rejects
finish, recursive split, unknown and content packets before adapter mutation.
Any seam error closes/detaches transport and poisons subsequent reads. No split
ingress, negotiated authority, timer/queue dispatch or outbound splitter is wired;
no new channel is offered. Feature opt-in remains disabled; no host inventory
was invented. Both core and engine finish refusals remain. STILL CANNOT JOIN.
See `docs/horizon-split-seam.md` for exact scope and actual-source audit.

Added 5 full-engine companion tests + 2 protocol tests, using actual dependencies.
New Rust tests have NOT compiled/executed locally. Full-driver passing floors now
14 companion / 3 protocol framing / 9 early configuration / 81 network-filter.
The isolated checker explicitly excludes engine-dependent cases, retaining the
9 companion + 8 endpoint contracts without replacement/mock protocol APIs.
Locally executed: 35 Python tests, six fixtures, syntax/whitespace checks, narrow
exact-source hash checks and patch apply/duplicate/reverse/restoration/regeneration.
No large source/toolchain/dependency downloads; respect disk reservations.

Last inspected run 46082288102 (tested SHA 70ba5a4737d2fcb669738ee49dac5dcdb330939f)
has successful preflight, core, provenance/isolated, and dependency-fetch steps;
full engine step remains IN PROGRESS. New split code requires its OWN SHA's run;
do not infer its success from that older run. Verify latest remote HEAD and find
newest run by SHA. Retain completed actual evidence and fix genuine CI failures
first. SHA256SUMS had nine stale pre-existing entries; reconciled current tracked
file digests as checksum maintenance, not as source/runtime validation.

## Current CI inspection and retained evidence (2026-11-27 23:29 UTC)

Verified PR #1 remains open/draft and remote HEAD is
`70ba5a4737d2fcb669738ee49dac5dcdb330939f` before editing. Current-SHA run:
https://github.com/horizon-rust-mod-rewrites/horizon-neoforge-compat/actions/runs/46082288102
Its **companion-preflight succeeded**, executing 35 Python tests, six fixtures,
and 17 isolated actual Rust contracts: **9 companion + 8 endpoint**. Thus the
five new companion boundary tests have now actually executed in isolation.
Retained test-step log: `integration/hosted-46082288102-companion-preflight.txt`.
Full integration for this SHA is still IN PROGRESS, NOT validated. The new
selection floors and lock-audit changes still await this full-driver execution.

Prior run 46081103967 completed SUCCESS at tested SHA
`16d6ce33ea333efc1c5be968a20508472bc3aca7`. Downloaded artifact digest verified:
`a33a47f0f4d123c71c23fe5fdf0e763e643596d60d740e0c08cce7d13dcea2cc`.
Retained `integration/hosted-46081103967-{full-check,full-driver,scope}.txt`.
Actual selected full-engine counts remain **4 companion, 9 early configuration,
1 TCP framing, 81 network filter passed / 3 ignored**. This older result does
NOT prove the nine-companion suite or newer driver changes in the current SHA.
Runs 46082076294, 46082027794 and 46081949222 are CANCELLED, not passing.
No genuine current CI failure found; no speculative fix or cancelled-run success
claim. Both finish gates remain; STILL CANNOT JOIN. Substantive next integration
work is being investigated separately; no host flag inventory has been invented.

## Lock-audit ambiguity hardening / handoff checkpoint

The full-driver lock audit now rejects duplicate package identities in either
input instead of silently losing entries during dictionary construction, and
rejects changed top-level lock format/metadata. Two new synthetic regression
tests cover both graphs and metadata/version mutation. Local 35 Python tests,
six fixtures, actionlint and whitespace validation pass. No Rust or engine
behavior change; new actual companion Rust tests still need hosted execution.

Latest observed CI: run 46082076294 for 025097c4c is PENDING, with no jobs yet;
46081103967 for documentation/evidence commit 16d6ce33e is IN PROGRESS. Intermediate
runs 46081949222 and 46082027794 were CANCELLED, not passing. Do not assume the
new preflight or nine companion tests have executed. This next pushed commit
will enqueue its own run; inspect current branch HEAD and jobs before more work.
No full source/compiler/dependency downloads in the 1 GiB local environment.

## Fast actual-companion CI prerequisite

Added a ten-minute standard hosted prerequisite that runs actual isolated
companion/endpoint Rust contracts plus Python/fixtures BEFORE the costly full
Horizon job. Uses the same pinned checkout action and Rust 1.97.0; no source
bundle, native packages, secrets, paid runners or game server needed. The full
engine run remains mandatory and now depends on this fast gate. This is not a
replacement for full engine tests. Local actionlint and all 33 Python tests/six
fixtures pass; Rust execution remains pending hosted CI for these commits.

## Regression selection floor checkpoint

Full driver now rejects unexpectedly shrunken passing suites, not just zero-test
filters: minimum companion 9 (including five new boundary tests), early endpoint
9, protocol framing 1, network filter 81. Ignored tests never count as passes.
These are selection floors, NOT new executed Rust results. Five new Python
summary-parser tests exercise empty/shrunken/failed/ignored summaries and compare
historical retained transcript counts. Local 33 Python tests and six fixtures
pass. New Rust tests/full-driver changes still need hosted execution; no new
join capability or host attestation was enabled. Preserve full CI evidence and
record its actual outcome rather than inferring success from prior SHA.

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
