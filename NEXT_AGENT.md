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
