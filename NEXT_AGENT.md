# Credit-conscious continuation: start here

Continue existing private draft PR #1, branch
`feat/neoforge-21.1.248-early-handshake`; always verify current remote HEAD.
Read AGENTS.md and linked handoffs, then this file. Keep every commit attributed:

    AI-Assisted-By: QX AI assistant

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
