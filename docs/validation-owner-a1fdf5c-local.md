# Local validation — a1fdf5c83a36f3035b94a12a9477c4cef19d097f

Full ordered-owner acceptance: NOT COMPLETE. Checklist remains 30/100.
The next documentation-only commit does not change this tested runtime source.

## Executed local checks

- Exact pinned Horizon HEAD ae632edf48f6cae3ba52a5c81523ec712488c9cc,
  14 provenance + 15 integration hashes verified before/after isolated patch check.
- Clean patch application, duplicate refusal, reverse applicability and byte-exact
  source restoration: PASS. Four structural contracts PASS (not runtime proof).
- Actual dependency-free endpoint: 9 passed. Actual generated protocol table: 1.
- Actual core/companion/endpoint isolated contract: 18 passed, zero failures.
  This overlaps endpoint tests; do not sum them as independent coverage.
- Core: 172 tests in EACH debug/release; strict Clippy all-targets, core fmt,
  warning-denied docs: PASS. Rust 1.97.0, x86_64-unknown-linux-musl.
- All nine actual modified upstream Rust files: rustfmt --check edition2024 PASS.
- Python integration drivers: 35 PASS; progress metadata validators: 28 PASS;
  generated progress consistency: PASS; six synthetic fixtures verified.
- Existing isolated warning remains: unexpected test-only compat_isolated cfg.
  Isolated endpoint checks do not typecheck the actual client owner/socket code.

Patch SHA-256: 9a94d741a1e0365c3ce998ecca7cf01b23e50c15a6e6e29f06e9712f1a224785

## Not executed / pending

Full local engine build was NOT STARTED: disk preflight reported 137,494,528
available bytes versus the existing 10,737,418,240-byte minimum build reservation.
No large dependency download, resource preparation, game or server was started.

The runtime push is eligible for the existing standard hosted workflow. Reading
its exact-SHA workflow-run list was blocked awaiting GitHub approval in this run.
No run ID, full-client compilation, actual ten new engine/endpoint regression
results, artifact digest or hosted success is inferred. Next authorized action:
inspect existing repository workflow runs filtered by head SHA a1fdf5c83a36f3035b94a12a9477c4cef19d097f;
then retain the run/jobs/logs and verify its artifact digest before claiming PASS.
No new dispatch is needed if the existing push-triggered run already exists.
Older hosted run46161098482 is evidence ONLY for d0c0527, never for this source.

## Runtime scope and blockers

Actual implementation: detached physical cancellation/shared terminal fencing;
early-profile cumulative decoded ingress + encoded egress policy; absolute
per-reader lifetime with per-fill timeouts; whole-batch budget refusal before
queue admission; write waits capped by lifetime; profile-less automatic Transfer
refusal. Ten added regressions are included, but nine require actual engine
execution still pending (one new isolated endpoint test has run locally).

This does NOT serialize both adapter transitions/generated queues, fence all
prepared/stale work, install production split routing or genuine bidirectional
grants, provide connection-wide login/play/adapter-output budgets, await real
asynchronous host work, or implement NeoForge reconnect/transfer/reconfiguration
lifecycle. A callback cannot be forcibly preempted; detached external pollers
still bound their own external I/O; infallible framing needs caller ownership.
All task/content/live prerequisites remain. Both finish refusals and default
split/feature-flag opt-outs remain; CANNOT JOIN; no individual mod rewrites.
