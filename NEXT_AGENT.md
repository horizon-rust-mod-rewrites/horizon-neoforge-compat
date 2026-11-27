# Credit-conscious continuation: start here

Continue existing draft PR #1, branch
`feat/neoforge-21.1.248-early-handshake`; always verify current remote HEAD.
Read AGENTS.md and linked handoffs, then this file. Keep every commit attributed:

    AI-Assisted-By: QX AI assistant

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
