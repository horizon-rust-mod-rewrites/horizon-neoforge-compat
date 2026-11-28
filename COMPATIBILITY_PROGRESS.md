# Compatibility layer completion tracker

**Evidence-backed checklist completion: 30% (30/100 points).**

**Ready to begin individual mod rewrites: NO.**

Updated: 2026-11-28. Rubric: v1.

## What this percentage means

- A conservative, human-weighted acceptance checklist, NOT measured engineering effort, elapsed time, ETA, test coverage or percentage of mods supported.
- Only fully evidenced **complete** rows earn their fixed points. Partial/in-progress/blocked rows earn zero. Tracker/documentation work itself earns no points.
- The denominator is the full reusable baseline layer, including semantic content and isolated live proof, not just negotiation. Scope: MC **1.21.1**, protocol **767**, NeoForge **21.1.248** only.
- 100% means ready to BEGIN separate mod rewrites against the documented supported contract; it does not implement or guarantee support for roughly 70 mods or every NeoForge API.
- New requirements must be added, not hidden as exclusions to reach 100%. Rebalance weights to 100 with a rubric-version bump and explain the change in the ledger; percentages across rubric versions are not directly comparable.
- A green metadata check validates arithmetic, evidence-file presence and prerequisite consistency only. Humans must review evidence truth, scope and freshness; older passing SHAs do not validate newer code.

## Latest executed engine checkpoint

- Tested source SHA: `7becb053b2d75f4a8978decbd8ecf94fb9d0dd95`; distinct from newer tracking/docs commits.
- [Successful hosted run](https://github.com/horizon-rust-mod-rewrites/horizon-neoforge-compat/actions/runs/46137932076); [retained selected-engine evidence](integration/hosted-46137932076-full-check.txt).
- 172 core tests EACH debug/release, including 37 split tests; 16 companion / 9 early / 14 protocol / 81 network-filter passed, 3 ignored. Selected suites only; overlapping isolated tests are not additional coverage.
- **Current runtime: CANNOT JOIN.** Production split routing, complete tasks, real semantic content publication, launcher/UI and live interoperability remain unproven. Default split/feature-flag opt-ins remain disabled; both finish refusals remain.

## Milestone percentages

| Milestone | Earned / allocated points | Completion |
| --- | ---: | ---: |
| Exact baseline and reproducibility | 6 / 6 | 100.0% |
| Negotiation, routing and diagnostics | 8 / 12 | 66.7% |
| Ordered production transport and split support | 6 / 20 | 30.0% |
| Required configuration tasks and host behavior | 2 / 16 | 12.5% |
| Atomic bidirectional semantic content compatibility | 2 / 24 | 8.3% |
| Actual engine validation and usable integration | 6 / 8 | 75.0% |
| Isolated live interoperability and completion proof | 0 / 10 | 0.0% |
| Readiness to BEGIN separate mod rewrites | 0 / 4 | 0.0% |

## Acceptance checklist

Evidence checkpoint SHAs identify where the referenced record/source was reviewed, not necessarily the original execution SHA. Read each retained transcript for what actually ran. Source/test files support inspection but do not by themselves prove execution.

### Exact baseline and reproducibility

| ID / deliverable | Points | Status | Acceptance / retained evidence |
| --- | ---: | --- | --- |
| `pins` — Exact source and version pins | 2 | complete | Exact loader, Minecraft ordinals and Horizon sources are identified, checksummed and reproducibly verified. Evidence: [provenance/neoforge-21.1.248.json](provenance/neoforge-21.1.248.json) (checkpoint `fb0294b`); [provenance/horizon.json](provenance/horizon.json) (checkpoint `fb0294b`); [integration/hosted-46087704109-full-check.txt](integration/hosted-46087704109-full-check.txt) (checkpoint `fb0294b`) |
| `licenses` — License separation and attribution | 1 | complete | Existing core/patch/reference provenance, notices and AI commit policy are recorded; review new imports separately. Evidence: [THIRD_PARTY_NOTICES.md](THIRD_PARTY_NOTICES.md) (checkpoint `fb0294b`); [integration/NOTICE.md](integration/NOTICE.md) (checkpoint `fb0294b`) |
| `reproduction` — Bounded reproducible validation | 3 | complete | Resource preflight, pin/lock audit and core/full-engine validation run with retained exact-SHA results. Prerequisites: `pins`. Evidence: [integration/hosted-46087704109-core.txt](integration/hosted-46087704109-core.txt) (checkpoint `fb0294b`); [integration/hosted-46087704109-full-check.txt](integration/hosted-46087704109-full-check.txt) (checkpoint `fb0294b`) |

### Negotiation, routing and diagnostics

| ID / deliverable | Points | Status | Acceptance / retained evidence |
| --- | ---: | --- | --- |
| `codecs` — Bounded query/setup and common codecs | 3 | complete | Bounded common/query/setup codecs and negative/truncation regressions pass. Evidence: [integration/hosted-46087704109-core.txt](integration/hosted-46087704109-core.txt) (checkpoint `fb0294b`); [docs/WIRE.md](docs/WIRE.md) (checkpoint `fb0294b`) |
| `java-query` — Independent Java query/setup comparison | 3 | complete | Actual pinned Java/Rust query/setup comparison executes; policy differences are documented. Does NOT cover split transport. Prerequisites: `codecs`. Evidence: [tests/reference/results.json](tests/reference/results.json) (checkpoint `fb0294b`); [tests/reference/validation.txt](tests/reference/validation.txt) (checkpoint `fb0294b`) |
| `selection` — Handler-backed selection and fail-closed Loader | 2 | complete | Exact phase/version/direction selection, ordering, epoch cleanup and finish refusal are tested. Prerequisites: `codecs`. Evidence: [integration/hosted-46087704109-core.txt](integration/hosted-46087704109-core.txt) (checkpoint `fb0294b`); [tests/loader.rs](tests/loader.rs) (checkpoint `fb0294b`); [tests/negotiation.rs](tests/negotiation.rs) (checkpoint `fb0294b`) |
| `diagnostics` — Safe actionable failure diagnostics | 2 | in progress | Decode and safely expose bounded Component/NBT failure semantics with phase/channel context; raw NBT alone is insufficient. No completion evidence yet. |
| `routing` — Complete required payload routing inventory | 2 | in progress | Audit every baseline required payload/task and both directions; registration never supplies authority; unsupported required behavior terminates. No completion evidence yet. |

### Ordered production transport and split support

| ID / deliverable | Points | Status | Acceptance / retained evidence |
| --- | ---: | --- | --- |
| `assembly` — Bounded inbound split primitive | 2 | complete | Fallible bounded reassembly, deadlines and lifecycle tests execute. This is not production ingress. Evidence: [integration/hosted-46087704109-core.txt](integration/hosted-46087704109-core.txt) (checkpoint `fb0294b`); [src/split.rs](src/split.rs) (checkpoint `fb0294b`) |
| `splitter` — Bounded outbound split primitive | 2 | complete | Original/envelope limits, capacity and split boundary tests execute. This is not production emission. Evidence: [integration/hosted-46087704109-core.txt](integration/hosted-46087704109-core.txt) (checkpoint `fb0294b`); [tests/split.rs](tests/split.rs) (checkpoint `fb0294b`) |
| `adapter-seams` — Actual adapter and TCP split seams | 2 | complete | Actual preparation/decoder and TCP compression/encryption matrix passes without queue/cipher side effects; not a production route. Evidence: [integration/hosted-46087704109-full-check.txt](integration/hosted-46087704109-full-check.txt) (checkpoint `fb0294b`); [docs/horizon-split-outbound-seam.md](docs/horizon-split-outbound-seam.md) (checkpoint `fb0294b`) |
| `owner` — Single live ordered connection owner | 4 | not started | One owner serializes both phase transitions, Loader epoch, ingress, preparation, queued writes and close/reset; idle timer cannot race them. Prerequisites: `adapter-seams`, `selection`. No completion evidence yet. |
| `split-authority` — Negotiated bidirectional split authority | 2 | blocked | Actual selected setup plus real host handlers grants both needed directions per epoch; no registration/threshold inference. Prerequisites: `owner`. No completion evidence yet. |
| `transport-budgets` — Connection-wide bounded admission and idle expiry | 3 | blocked | Whole-traffic/pending budgets, idle assembly deadline, fallible whole-batch atomic admission and serialized output are enforced by real owner. Prerequisites: `owner`, `assembly`, `splitter`. No completion evidence yet. |
| `transport-runtime` — Production ingress/emission and awaited completion | 3 | blocked | Actual ingress and direct framing route are enabled only with authority; dispatch/write completion is awaited; failures close socket and reset Loader without ack. Prerequisites: `split-authority`, `transport-budgets`. No completion evidence yet. |
| `java-split` — Java split codec/transport interoperability | 2 | not started | Pinned independent Java split reference and malformed/limit/threshold cases validate both directions against Rust. No completion evidence yet. |

### Required configuration tasks and host behavior

| ID / deliverable | Points | Status | Acceptance / retained evidence |
| --- | ---: | --- | --- |
| `staging` — Registry staging and semantic commit barrier | 1 | complete | Bounded epoch staging and mandatory semantic validation/atomic commit barriers pass synthetic-host tests; no real content host claimed. Evidence: [integration/hosted-46087704109-core.txt](integration/hosted-46087704109-core.txt) (checkpoint `fb0294b`); [tests/registry_sync.rs](tests/registry_sync.rs) (checkpoint `fb0294b`) |
| `flag-primitive` — Opt-in feature-flag equality primitive | 1 | complete | Bounded equality and negotiated data/ack task routing tests pass; default stays off without genuine host inventory. Evidence: [integration/hosted-46087704109-core.txt](integration/hosted-46087704109-core.txt) (checkpoint `fb0294b`); [tests/feature_flags_loader.rs](tests/feature_flags_loader.rs) (checkpoint `fb0294b`) |
| `config-files` — Config-file task and enforced host config | 3 | not started | Exact bounded config task codecs/order and actual host enforcement/ack behavior are independently tested, with explicit refusal for unsupported semantics. No completion evidence yet. |
| `data-maps` — Registry data-map tasks | 3 | not started | Exact baseline data-map inventory/codecs and host semantic apply/ack behavior are implemented and tested. No completion evidence yet. |
| `enums` — Extensible-enum task | 3 | not started | Exact enum codec/order and actual host enum semantics/equality are implemented and tested; no invented empty inventory. No completion evidence yet. |
| `host-flags` — Host-owned implemented-feature inventory | 3 | blocked | Capability/profile inventory is tied to implementation lifetime, enforced by host behavior and wired to real flag task; enabled peer/world names are not evidence. Prerequisites: `flag-primitive`, `owner`. No completion evidence yet. |
| `task-order` — Complete task ordering and reconnect lifecycle | 2 | blocked | Real baseline task graph, required-handler failure, duplicate/out-of-order/timeout and epoch reset cases execute without silent acknowledgment. Prerequisites: `routing`, `config-files`, `data-maps`, `enums`, `host-flags`, `transport-runtime`. No completion evidence yet. |

### Atomic bidirectional semantic content compatibility

| ID / deliverable | Points | Status | Acceptance / retained evidence |
| --- | ---: | --- | --- |
| `numeric-maps` — Explicit bidirectional numeric-map infrastructure | 2 | complete | Sparse IDs, aliases and atomic map construction tested; not semantic registry publication. Evidence: [integration/hosted-46087704109-core.txt](integration/hosted-46087704109-core.txt) (checkpoint `fb0294b`); [tests/core.rs](tests/core.rs) (checkpoint `fb0294b`) |
| `content-transaction` — Real host inventory and atomic publication | 4 | blocked | Define supported content inventory and implement transactional both-way semantic validation/publication across real host packet paths, with rollback/refusal. Prerequisites: `staging`, `numeric-maps`, `owner`. No completion evidence yet. |
| `items` — Items and data components | 4 | blocked | Real inbound/outbound item stacks, components and nested content are translated semantically with independent/round-trip tests. Prerequisites: `content-transaction`. No completion evidence yet. |
| `blocks` — Blocks, states, chunks and palettes | 5 | blocked | Real block/state/chunk/palette paths apply semantic mappings in both directions, including unsupported-content failures. Prerequisites: `content-transaction`. No completion evidence yet. |
| `entities` — Entities and serializers | 3 | blocked | Real entity types, metadata serializers and payload paths use verified semantic adapters both ways. Prerequisites: `content-transaction`. No completion evidence yet. |
| `recipes-tags` — Recipes, tags and remaining content-bearing paths | 4 | blocked | Audit complete packet/content surface; recipes, tags, menus and remaining relevant IDs/semantics are implemented or explicitly rejected before finish. Prerequisites: `content-transaction`, `items`, `blocks`, `entities`. No completion evidence yet. |
| `resource-semantics` — Resource and datapack semantic parity | 2 | blocked | Known-pack/resource/datapack claims correspond to actual implemented host semantics, not hash/name equality or forged pack support. Prerequisites: `content-transaction`. No completion evidence yet. |

### Actual engine validation and usable integration

| ID / deliverable | Points | Status | Acceptance / retained evidence |
| --- | ---: | --- | --- |
| `engine-build` — Full patched actual-client and companion typecheck | 3 | complete | Full pinned actual client and wired companion typecheck; upstream locked graph unchanged except local dependency-free core. Evidence: [integration/hosted-46087704109-full-check.txt](integration/hosted-46087704109-full-check.txt) (checkpoint `fb0294b`) |
| `engine-tests` — Selected actual-engine TCP and lifecycle regressions | 3 | complete | Recorded selected suites execute: 16 companion, 9 early, 14 protocol, 81 network-filter passed/3 ignored; not all upstream tests. Evidence: [integration/hosted-46087704109-full-check.txt](integration/hosted-46087704109-full-check.txt) (checkpoint `fb0294b`); [integration/hosted-46087704109-core.txt](integration/hosted-46087704109-core.txt) (checkpoint `fb0294b`) |
| `launcher` — Usable opt-in launcher/runtime wiring | 2 | blocked | Reproducible real entry-point/profile wiring with ownership and cleanup is exercised; not merely a standalone checker/patch. Preserve/renegotiate explicit profile on reconnect, transfer and reconfiguration; no vanilla fallback. Prerequisites: `owner`, `transport-runtime`, `task-order`. No completion evidence yet. |

### Isolated live interoperability and completion proof

| ID / deliverable | Points | Status | Acceptance / retained evidence |
| --- | ---: | --- | --- |
| `baseline-join` — Isolated exact NeoForge baseline join | 3 | blocked | Only after all prerequisites: exact 21.1.248 isolated server completes real configuration and representative play with retained sanitized logs and exact versions. Prerequisites: `task-order`, `java-split`, `diagnostics`, `items`, `blocks`, `entities`, `recipes-tags`, `resource-semantics`, `launcher`. No completion evidence yet. |
| `fixture-join` — Minimal fixture-mod join and content round trips | 3 | blocked | Small licensed fixture exercises required payloads/tasks and semantic content in both directions on isolated server; no production-first testing. Prerequisites: `baseline-join`. No completion evidence yet. |
| `live-lifecycle` — Real disconnect, failure and reconnect matrix | 2 | blocked | Timeout, malformed/unsupported/mismatch, mid-write failure and clean reconnect produce terminal cleanup and no stale state; compression/encryption covered. Prerequisites: `baseline-join`. No completion evidence yet. |
| `finish-proof` — Reviewed completion gate and end-to-end repeatability | 2 | blocked | Review both finish refusals only after actual complete prerequisites; replace with proven gating, rerun regressions and live matrix on exact final source SHA. No unconditional unlock. Prerequisites: `fixture-join`, `live-lifecycle`. No completion evidence yet. |

### Readiness to BEGIN separate mod rewrites

| ID / deliverable | Points | Status | Acceptance / retained evidence |
| --- | ---: | --- | --- |
| `adapter-api` — Stable reusable adapter contract | 1 | blocked | Document and exercise a stable extension API with per-connection state, negotiated sends/replies, configuration AND play routing, lifecycle/reset hooks, async completion barriers, budgets, diagnostics and exact supported/rejected content/version constraints. Prerequisites: `finish-proof`. No completion evidence yet. |
| `fixture-example` — Minimal reusable extension example | 1 | blocked | Publish a tested minimal fixture/example showing a real handler/content extension without claiming any real mod rewritten. Prerequisites: `fixture-join`, `adapter-api`. No completion evidence yet. |
| `release-repro` — Reproducible installation and diagnostics | 1 | blocked | Document installation/build/profile selection, isolated test procedure, failure triage and tested release SHA; no user credentials/game assets. Prerequisites: `finish-proof`, `launcher`. No completion evidence yet. |
| `scope-review` — Final support/license and rewrite handoff review | 1 | blocked | Review all acceptance evidence, supported baseline and exclusions; record explicit readiness to start separate mod projects with per-mod source/license/version review. No universal 70-mod claim. Prerequisites: `adapter-api`, `fixture-example`, `release-repro`. No completion evidence yet. |

## Immediate next implementation gate

Implement `owner` first. See [ordered connection acceptance plan](docs/ordered-connection-owner.md). A new timer, phase snapshot or split opt-in alone is not that owner. Then prove split authority, admission/deadlines and awaited production dispatch/write completion. Keep both finish refusals throughout this work.

## Updating this tracker

1. Edit `progress/compatibility.json`; attach retained repository-relative evidence and its reviewed checkpoint SHA. Complete prerequisites first. Regressions must downgrade affected rows and any dependent completed rows.
2. Record the exact tested source SHA in every new execution record. Keep implementation, tests, CI results and live-server proof distinct. Update `last_engine_validation` only after inspecting that exact run.
3. Run `python3 progress/report.py` then `python3 progress/report.py --check` and `python3 -m unittest discover -s progress -p "test_*.py" -v`.
4. Update the change ledger and NEXT_AGENT.md; refresh SHA256SUMS. Use the same draft PR with the required AI attribution footer.
5. Before reporting 100%, review ALL acceptance criteria and final exact-source runtime/live evidence. This file never grants Loader authority or toggles a finish gate.

## Change ledger

- 2026-11-28, rubric v1: initial 100-point baseline, **30% complete**. Credits existing exact-version codec/infrastructure and selected hosted engine evidence only. New tracker/owner plan adds **0 compatibility points**. No new protocol implementation or live execution is claimed.

<!-- Generated by progress/report.py; edit JSON and renderer, then regenerate. -->
