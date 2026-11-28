# Milestone attempt published — full-engine approval/evidence pending

Runtime source: `a1fdf5c83a36f3035b94a12a9477c4cef19d097f`. Local checks rerun at that EXACT SHA pass;
see docs/validation-owner-a1fdf5c-local.md. Full-engine inspection is blocked on
approval to read GitHub workflow runs filtered by that SHA. No hosted run ID or
success is inferred. Local engine build was safely refused by disk preflight.
Do not award points or substitute earlier hosted evidence. **30/100, CANNOT JOIN**.
The following implementation note describes this runtime source; the new commit
above it is documentation/evidence only, not an additional runtime change.

# Milestone attempt — detached cancellation / epoch resource policy (validation pending)

Remote HEAD verified as `74b357d4d17c4715083a1892aa2eb435756748d7` before edits.
Substantive continuation is in the actual Horizon patch, not a synthetic owner:

- Detached reader/writer halves retain a real physical shutdown capability from
  the writer handle already present at detach. Closers share terminal state;
  shutdown revokes buffered reads and new preparations/sends, including empty raw
  operations. `take` leaves an inert placeholder with no old-socket capability.
- Explicit-profile configuration now has a cumulative decoded-ingress + encoded-
  egress policy: 16,384 packets, 64MiB, absolute 120s lifetime, only tighten-able.
  Ordinary packets count; whole batches are charged before publication; write
  waits are capped by remaining lifetime and rechecked afterwards. Actual reader
  loops check the absolute deadline, including per-fill socket timeouts, so an
  incomplete frame/continuous trickle cannot refresh it indefinitely.
- Automatic Transfer on this early path refuses before publishing a profile-less
  reconnect event. The user must explicitly reselect a compatible profile; this
  is safe refusal, NOT implemented NeoForge transfer/reconnect support.
- Ten additional regressions: five actual owner/policy tests, four actual protocol
  tests (including detached TCP compression/encryption matrix), one endpoint
  policy-boundary test. Existing suites retained, no ignore added.

Local pin/patch/reverse/restoration + endpoint9/table1, isolated companion18 and
Python driver35, metadata28, core172 EACH debug/release, strict Clippy/fmt/docs
and six fixtures have passed. Full actual-engine compilation/tests are PENDING for
this new runtime source until its OWN standard hosted run completes. Previous
run46161098482 validates d0c0527 only; do NOT substitute it for this source.

**30/100, zero new criterion points; owner remains incomplete; CANNOT JOIN.**
Still needed: one serialization domain for BOTH adapter transitions and queues,
prepared-original and stale-work epoch fencing; genuine bidirectional negotiated
split grants/routing; connection-wide (including login/play/adapter output) budgets,
awaited asynchronous host effects, complete profile-safe lifecycle/reconnect, and
all remaining task/content/live prerequisites. Low-level infallible detached
framing APIs still require caller serialization; closing a socket is not a peer
acknowledgment. External detached pollers must bound their own external I/O.
A synchronous bridge callback is still a bounded-work contract, not forcibly
preemptible host execution. Both finish refusals and default split/flag opt-outs
remain. No production server tests, mod rewrites, merge, force push, visibility
change or schedules. Use the same draft PR and required AI commit footer.

--- Previous exact-SHA checkpoint follows ---

# Exact-SHA continuation checkpoint — awaited early writes PASSED

- Source commit: `d0c05270057bc69da8082a229dc265f5d4929bdf`.
- Full standard hosted validation: https://github.com/horizon-rust-mod-rewrites/horizon-neoforge-compat/actions/runs/46161098482 — **SUCCESS**.
- Artifact ZIP SHA-256 verified before reading:
  `6e3c62d48b20e55e6db1d2232665179674ec0a0e4b09ae7d94a29ef98b674060`.
- Retained `integration/hosted-46161098482-{core,full-check,full-driver,scope,patch,companion-isolated}.txt`.
- Actual full pinned patched client + companion typecheck/lock audit pass.
  **16 companion / 9 early configuration / 12 ordered-owner / 14 protocol
  framing / 81 network-filter passed, 3 ignored**. Selected suites only, NOT all
  engine tests; isolated endpoint/companion counts overlap and are not additive.
- Core **172 tests EACH debug/release**, strict Clippy/fmt/warning-denied docs,
  35 integration-driver tests and six synthetic fixtures passed. Metadata push
  run46161103360 passed with28 progress-validator tests + full checksums.
- Warning ledger: five existing upstream horizon-mod-wasm unused functions;
  patched client unnecessary `mut`, unused `at_payload`, epoch field/accessors; test-only unexpected
  `compat_isolated` cfg and unused epoch accessor. These were NOT hidden and do
  not constitute the missing synchronization/epoch implementation.

## Runtime changes earned (not checklist completion)

Preserved merged HEAD647f430 newer owner work after verifying remote HEAD, and
inspected its ACTUAL failed runs first. Fixed obsolete structural assertions and
three stale checksum entries; retained raw failed-job evidence. Full compilation
had never started in those failed runs; old7becb05 was not owner evidence.

The actual early configure route now awaits a bounded queued-worker write_all
completion receipt for Loader batches AND engine replies (keepalive/pong/etc.)
before advancing dispatch. Queue insertion is no longer treated as completion.
No shared writer lock is held while waiting. Timeout, dropped receipt or write
failure closes the physical transport and prevents subsequent queued jobs from
draining. Retirement refuses while an old receipt could still cancel its socket.
Whole-batch admission charges retained inner + outer capacities and overhead,
checks packet count/overflow, and exposes no packets on rejection. Early-path
RAII owns the reader borrow and closes both directions on every early exit,
including missing shared writer and bootstrap/factory failure. It also discards
buffered originals and refuses subsequent reads, even zero-length raw reads.
Bridge reset is immediate and exactly-once on terminate; ordinary traffic polls
the assembly deadline instead of indefinitely postponing an idle-only check.

Eight new adversarial tests exercise actual worker/entry paths, including blocked
pipe completion, timeout, drop cancellation, retirement, peer failure, bounded
admission, missing-writer/old-buffer invalidation, bootstrap closure and TCP compression on/off × encryption on/off.
Two inherited owner tests had inconsistent bridge inputs/reset expectations;
those are fixed, not silently skipped. No production split authority is granted.

### Independent follow-up review and validation ledger

First source `dabf9c5b86e1c92442b1019abbef3b6da51ff4cc` also PASSED full
run46159257282 (11 owner tests). Its own retained six transcripts remain separate;
ZIP SHA256 c94cfb0cdd34814351be40d3e0b36d2edacdaab41d3cc8ce24fdadc042397667.
Post-test review found an uncovered real teardown edge: the first guard could
return before construction if the shared writer was missing, and a closer alone
did not invalidate buffered old-epoch packets. Second source d0c0527 fixes both
with the borrowed-reader RAII/shutdown boundary and an extra actual-entry test.
The top run46161098482 validates that SECOND exact source; do not combine suites
or substitute the first run's result. Evidence-only commits following it change
no compiled source/patch/driver/workflow. The unused-mut warning remains recorded,
not suppressed; it is cosmetic and not a reason to claim phase ownership solved.

## Still incomplete: 30/100, CANNOT JOIN

The owner criterion is now **in_progress**, earns ZERO points, and weights and
acceptance criteria are unchanged. This is a real runtime prerequisite slice,
NOT the complete single ordered connection owner and NOT 50% completion.
Remaining owner work: one serialization domain across BOTH adapter transitions,
prepared originals, adapter-generated queues, stale work and real socket/epoch;
detached-socket closer ownership; profile-safe reconnect/reconfiguration/transfer;
connection-wide traffic/pending budgets and genuine negotiated split grants.
A write_all return is NOT peer receipt, flush or asynchronous host-task completion.
The stack guard intentionally closes even on a future accidental Ok: revisit only
with a separately proven conditional completion/play lifecycle milestone.

Both core/engine finish refusals and default split/flag opt-outs remain. No mod
rewrites, live/production testing, merge, force push, visibility or schedule change.
Next steps: finish the owner above, then split authority/budgets/emission, actual
host configuration tasks, atomic both-way semantic content, isolated exact-version
interop. Do not claim any of those from this successful selected-suite run.

## Local validation/resource reassessment

Exact Rust1.97.0 minimal components and audited upstream sparse HEAD were verified;
no C linker, so local checks use musl + shipped direct linker. All29 audited source
hashes, patch apply/reverse/restoration, four structural assertions, endpoint8 +
table1, isolated companion17 (overlaps endpoint), core172 each debug/release,
strict Clippy/fmt/docs, driver35, metadata28 and fixtures6 pass. A duplicate-copy
disk exhaustion was recovered before successful execution; no repo data was lost.
Full-engine execution is the hosted result above, not a local-build claim. Exact
source unchanged by this following evidence-only commit. Preserve this distinction.

--- Older handoff follows (historical where contradicted above) ---

# Completion tracking checkpoint — 2026-11-28

See [COMPATIBILITY_PROGRESS.md](COMPATIBILITY_PROGRESS.md): initial rubric v1
remains **30% evidence-backed checklist completion**, NOT effort/ETA/coverage or
mod support. Only fully completed weighted criteria earn points; partial work
earns zero. Readiness to begin separate mod rewrites remains **NO**. The fixes in
7becb05 add no compatibility points and do not change production protocol code.

Machine-readable source: `progress/compatibility.json`; regenerate/check with
`python3 progress/report.py` / `python3 progress/report.py --check`. The current
dependency-free metadata suite has **28 tests**. Its lightweight CI is separate
from full-engine CI and cannot prove protocol compatibility.

## Latest exact-SHA hosted validation — SUCCESS

- Full run: https://github.com/horizon-rust-mod-rewrites/horizon-neoforge-compat/actions/runs/46137932076
- Tested compatibility SHA: `7becb053b2d75f4a8978decbd8ecf94fb9d0dd95`.
- Verified downloaded artifact SHA-256: `51e780c2b15dedfe656528ea6995f067b205b816eccb3d8cc14584146b259f27`.
- Retained `integration/hosted-46137932076-{core,full-check,full-driver,scope}.txt`.
- Full pinned patched client and actual companion typechecks, lock audit and selected engine regressions passed. **16 companion / 9 early configuration / 14 protocol framing / 81 network-filter passed, 3 ignored**; selected tests only.
- **172 core tests EACH debug/release**, including **37 split tests**; strict Clippy, fmt, warning-denied docs, 35 integration-driver tests and six fixtures passed. Metadata push and PR runs also passed with 28 progress tests.
- This commit adds malformed metadata validation, exact-maximum inbound split coverage and portable scratch-path instructions; no production runtime routing changed.

Next substantive runtime work remains the ordered connection owner, specified
in [docs/ordered-connection-owner.md](docs/ordered-connection-owner.md). It covers
both phases/epoch, real ingress and queued output, idle expiry, atomic batch
admission, actual completion, real transport closure and profile-safe reconnect/
reconfiguration. Then prove split authority, task host effects, atomic both-way
semantic content and isolated live interoperability; do not skip to mod rewrites.

Keep default split/flags disabled and both finish refusals. The latest run does
NOT test a NeoForge join or semantic content application. **CANNOT JOIN.** The
previous exact-SHA checkpoint below is historical runtime evidence.

---

# Continue SAME draft PR #1 — outbound milestone fully hosted-tested

## Rules and exact baseline

- Existing PR: https://github.com/horizon-rust-mod-rewrites/horizon-neoforge-compat/pull/1
- Branch: `feat/neoforge-21.1.248-early-handshake`. Verify ACTUAL remote HEAD before
  editing AND publishing; preserve newer work. Never merge or force push.
- EVERY commit footer: `AI-Assisted-By: QX AI assistant`.
- MC1.21.1 / protocol767 / NeoForge21.1.248 only. Horizon pin
  `ae632edf48f6cae3ba52a5c81523ec712488c9cc`. No auto-updated pack assumption.
- Public repository was authorized for standard hosted CI. No visibility changes,
  new schedules or cadence changes without consent. No credentials/game files/
  toolchains/build outputs in Git. No individual mod rewrites.
- Both core AND engine finish refusals MUST remain until genuinely complete and
  proven. Default split and feature-flag opt-ins remain DISABLED. CANNOT JOIN.

## Latest executed hosted evidence (authoritative)

SUCCESS: https://github.com/horizon-rust-mod-rewrites/horizon-neoforge-compat/actions/runs/46087704109
Tested SHA: `fb0294b4976bd78d4ee4cbfce44a6f049e0a4307`.
Artifact SHA256: `357207d45ab75c4e2cf94485f31f661915498617bb9fc51db73e570ed3670f5f`.
Verified downloaded ZIP bytes against GitHub's artifact digest before extraction.
Retained integration/hosted-46087704109-{core,full-check,full-driver,scope}.txt.

- Full patched actual client compiles before/after actual companion wiring.
  Exact upstream graph retained; only dependency-free local core is added.
  Complete source pin: 12,354 blobs, zero missing; 29 audit hashes verified.
- **16 companion / 9 early configuration / 14 protocol / 81 network-filter
  passed, 3 ignored.** Selected suites, NOT all upstream tests. Ignored tests
  are not passes; isolated contracts overlap and must not be summed again.
- **171 core tests EACH debug/release**, including all36 split tests. Strict
  Clippy, fmt, warning-denied docs, 35 Python and six synthetic fixtures passed.
- Eight new actual protocol preparation tests passed, including unchanged queues/
  phases/counters/cipher, malformed bodies, pure finish refusal and detach.
- Both new actual companion TCP tests passed: real preparation -> core splitter ->
  actual envelope/socket framing -> actual serverbound codec -> reassembly in all
  four compression/encryption combinations; real Loader reply stays unsplit and
  finish still refuses. These synthetic sockets are NOT a NeoForge join.
- Five upstream horizon-mod-wasm unused-function warnings remain, not hidden.

This final checkpoint changes ONLY evidence/handoff/docs/checksums. It does NOT
change compiled source, test source, patch, driver or workflow from the tested SHA.
By explicit workflow path exclusions it should not trigger another expensive
full build. Keep the tested SHA distinct from newer documentation-only HEAD.

## Substantive implementation completed this run

1. Inspected actual prior runs first. Retained full SUCCESS46084373021 for SHA
   7d053a65902af967c053ab65c32c6d59c058d444 (14/9/6/81, core153 each).
2. Fixed actual outbound77e9cb5 run46085995030 rustfmt failures in isolated commit
   35649b888233a0a4713d9b6ffb434ddc3b6e2d8c, retaining failure text.
3. Added actual protocol ConnWriter::prepare_serverbound_configuration in the
   existing six-file patch. Uses actual codec + existing shared exact-version ID
   mapping for opaque payload/keepalive/pong/known-packs only; both config phases,
   real exact adapter and network transport required. Rejects finish/content,
   recursion and reserved channels. No Session/cipher/counters/queue/socket effect.
   This is a pure preparation boundary, NOT production split integration.
4. Added eight actual protocol and two actual companion TCP regressions, plus
   fallible inbound reassembly growth and five core boundary/lifecycle regressions.
   Independent static audit found no additional demonstrated code bug.
5. Real rustfmt diagnostics for new core tests were repaired in testedfb0294b.
   Fast preflight now catches formatting before native/full-source/build inputs.
   Hosted text/integration Markdown changes no longer start a whole full build;
   source/tests/patch/driver/workflow changes still do.

See docs/horizon-split-outbound-seam.md, docs/horizon-split-seam.md, docs/SPLIT.md.
The latter's earlier pending sections are historical; top exact-SHA block wins.

## Honest intermediate run ledger

- 46085995030 (77e9cb5): FAILED on two formatting differences; no outbound Rust
  execution inferred. Actual core failure text and scope retained.
- 46087222005 (35649b8): **CANCELLED intentionally** after core166 each/31split,
  strict checks, isolated/provenance passed, once the substantive SHA was queued.
  Avoided redundant full-engine compilation; partial full-check retained and NOT
  called passing engine evidence. Its own core/scope/driver text retained.
- 46087488085 (6acc9dc): FAILED fast fmt prerequisite on three new-test differences;
  full job SKIPPED before large inputs. Exact diagnostic excerpt retained.
- 46087704109 (fb0294b): completed full SUCCESS; authoritative counts above.

## Next substantive work, in dependency order

FIRST verify remote HEAD, read AGENTS.md and this checkpoint, inspect newest
actual Actions run/jobs/logs by tested SHA, and fix genuine failures with text
evidence. Never infer a new SHA's success from older runs. Preserve coherent
commits and refresh this handoff BEFORE long/expensive operations.

1. Establish one real ordered connection owner across BOTH adapter phases, socket
   ingress, prepared outbound originals and queued writes. Phase snapshots are
   not synchronization. Preparation can accept detached NETWORK writers but
   cannot attest a live socket or epoch; externally owned sockets need a closer.
2. Prove negotiated bidirectional split channel authority; do not treat a
   registration list, decoder/preparer success or transport threshold as authority.
   Add production ingress/outbound only under that owner and genuine authority.
3. Enforce assembly deadlines from an IDLE timer, whole-connection traffic and
   pending budgets, atomic whole-batch admission, serialized envelope output, and
   awaited real dispatch/write completion. Close/reset transport AND Loader on
   failures; no invented ack/finish. Existing QueuedWriter already has a16MiB
   ordinary byte-channel budget, but does NOT provide these split guarantees.
4. Complete remaining required configuration tasks with actual enforced host
   behavior. No placeholders/silent ignores. Read docs/horizon-host-inventory-audit.md:
   enabled names, peer/world flags, a pack hash or an invented empty set are NOT
   an implemented-feature inventory. Genuine host-owned capability/profile
   evidence tied to implementation lifetime is still absent; keep opt-in off.
5. Atomic BOTH-WAY SEMANTIC registry/content application across real packet paths,
   not numeric IDs alone. Reuse completed Java differential research; split Java
   codec/transport interoperability remains distinct from existing query/setup.
6. Only after prerequisites, isolated exact NeoForge21.1.248, then minimal fixture
   mod. No production-first tests. UI/live join still never tested.

## Resource and handoff discipline

Local disk was1GiB; no full upstream/compiler/dependency downloads attempted.
Reuse standard hosted full workflow; preserve disk reservations. Small source
retrievals must match exact-pin hashes. No local compiler is available. New
Rust execution in this run occurred on hosted CI, not the local Python checks.
Maintain SHA256SUMS for changed files. Keep branch fast-forward and reviewable.
Allocator-failure injection remains untested despite fallible split reservations.

Earlier detailed chronological checkpoints are archived at
[docs/continuation-history-2026-11-28.md](docs/continuation-history-2026-11-28.md); historical reference only, not required current truth.
AGENTS.md retains original policy/read-order and historical design material.
