# Credit-conscious continuation: start here

Continue existing private draft PR #1, branch
`feat/neoforge-21.1.248-early-handshake`; always verify current remote HEAD.
Read AGENTS.md and linked handoffs, then this file. Keep every commit attributed:

    AI-Assisted-By: QX AI assistant

Current feature-flag checkpoint: bounded codec plus real one-shot exact equality
against explicit host inventory. 12 new tests passed debug/release. See
`docs/FEATURE_FLAGS.md`. Loader routing/registration is next, not yet claimed.
No individual mods, main-branch updates, merges, force-pushes or visibility changes.

Resource-aware priorities:
1. Full Horizon + companion build and runtime regressions remain FIRST on a
   sufficiently large build machine. Use integration/check_full.py and read
   docs/horizon-continuation.md. Previous environment could not hold the full
   checkout/dependencies; do not repeat large downloads without checking space.
2. In constrained environments finish one bounded task at a time: implement
   explicit feature-flag inventory wiring with both selected channel directions,
   terminal mismatch/duplicate/error cleanup, and no default host claims.
3. Then remaining config/data-map/extensible-enum tasks, safe split transport
   integration, and real atomic bidirectional registry/content application.
   Follow actual dependencies; numeric ID maps are not semantic content support.
4. Isolated exact NeoForge baseline, then minimal fixture mod, never production
   first. Remove neither finish latch until required behavior genuinely works.

Latest pre-checkpoint core evidence: 128 main/driver tests debug/release, strict
Clippy, warning-denied docs, fixtures, upstream verification. Actual Java query/
setup equivalence is already done; do not redo that research without cause.
The client STILL CANNOT JOIN. Full engine/TCP, semantic publication and live join
checks are unexecuted, not implicitly passing. Save small coherent commits and
update this file after each milestone. Do not depend on another session's local
compiler/download/source directories surviving.
