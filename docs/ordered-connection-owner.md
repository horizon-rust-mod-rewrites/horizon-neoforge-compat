# Next production milestone: one ordered connection owner

Status: **acceptance/design plan only; no production owner implemented here**.
This continues NEXT_AGENT.md's first dependency, not a new handshake claim.
Target remains MC1.21.1 / protocol767 / NeoForge21.1.248, pinned Horizon
`ae632edf48f6cae3ba52a5c81523ec712488c9cc`.

## Why the existing seams are not enough

The actual inbound decoder, pure outbound preparer and synthetic TCP matrix pass
at `fb0294b4976bd78d4ee4cbfce44a6f049e0a4307`; see
[horizon-split-seam.md](horizon-split-seam.md),
[horizon-split-outbound-seam.md](horizon-split-outbound-seam.md) and retained
`integration/hosted-46087704109-full-check.txt`. Those tests assemble boundaries
manually. They do not install production routing or provide shared ownership.

Reading both adapter phases is not synchronization. Detached NETWORK preparation
cannot attest a live socket/epoch. Existing QueuedWriter's ordinary 16MiB budget
is not atomic fragmented-batch admission, idle expiry or awaited completion.
No runtime caller may infer split authority from a registration list, packet
threshold, successful decoding or preparation.

## Ownership contract to implement in the actual client

Choose an owner architecture only after inspecting the actual connection and
adapter code at the pin. An event-loop/actor is a possible approach, not an
implemented API or mandate. Its serialization domain MUST encompass:

- Both adapter phase transitions and the exact connection/configuration epoch.
- Socket ingress, reconstructed-original decoding and actual host dispatch.
- Negotiated authority, Loader state, staging and revocable host inventory.
- Outbound original preparation, adapter-generated inbound/outbound queues,
  whole-batch reservation, serialized framing and queued/socket write completion.
- Idle timers, cancellation, detached-socket closer and terminal teardown.

Tag pending work and grants with an epoch. Reject stale callbacks, timer events,
prepared originals and batches after disconnect/reconnect or reconfiguration.
Async work completion must report to that owner; a new mutex around only the
split assembler does not serialize the connection. Do not hold an engine/main
thread lock across awaits or let cyclic wait dependencies deadlock teardown.

## Actual production-path acceptance matrix

| Area | Required assertions |
| --- | --- |
| Phase/epoch ordering | Deterministic barriers interleave ingress, outbound preparation, BOTH adapter transitions, queued replies and disconnect. No original observes a mixed phase/epoch; stale work has no effects on its successor. |
| Negotiated authority | Exact selected setup and real installed handlers provide each direction's grant. Missing, wrong-direction, wrong-phase and stale grants refuse before effects. Unregister/revoke cannot leave usable cached grants. |
| Exactly-once transforms | Prepare an original once, split once, map each envelope once. Unsplit prepared originals enter direct framing, never ordinary adapter translation again. Preserve both adapter-queue orders. |
| Atomic admission | Reserve the whole batch under the owner. Budget/allocation/backpressure failure exposes zero fragments. Bound original count, envelope count, pending bytes, connection traffic and deadlines including ordinary traffic. Never introduce an unbounded queue. |
| Serialized emission | No other original or phase transition interleaves within an admitted fragment batch. Compression/framing/encryption/counters occur at actual emission, not pure preparation. |
| Awaited completion | A queue insertion is not dispatch/write success. Required task progress waits for actual host/main-thread and transport completion. Cancellation or failure after partial output terminates; no ambiguous replay or invented acknowledgment. |
| Idle deadline | Partial assembly expires without any subsequent incoming packet. Ordinary traffic or duplicate fragments cannot extend its absolute lifetime. Timer cancellation/replacement is epoch-safe. |
| Terminal cleanup | Malformed, unsupported, admission/dispatch/write, timeout and phase failures revoke grants, reset Loader/content and close the actual transport, including externally owned detached sockets. One terminal cause, no later success/ack. |
| Lifecycle/profile | Exercise reconnect, transfer and configuration re-entry. Retain or deliberately renegotiate the explicit NeoForge profile; never accidentally fall through the ordinary `configure(..., None)` wrapper to vanilla behavior. |
| Regression scope | Actual runtime entry points, not a second synthetic owner façade. Exercise split/unsplit cases and TCP across compression on/off × encryption on/off; preserve vanilla/Horizon routes. |

Record exact tested source SHA, tests run, ignored tests, failure diagnostics and
retained output. Update the tracker only for criteria actually satisfied. Do not
call the owner complete on phase snapshots or standalone splitter tests alone.

## Follow-on boundaries (not solved by the owner)

1. Audit a full exact-version required/conditional configuration task matrix:
   registration/negotiation, frozen registries, config files, data maps, extensible
   enums, feature flags and split. For every task record authority, ordering,
   bounds, real host effects, async barriers, timeout and failure/ack behavior.
2. Host feature evidence must describe real enforced implementations, distinguish
   unavailable from genuinely known-empty, bind to implementation lifetime and
   revoke stale profiles. Peer/world flags and pack hashes are not evidence.
3. Publish real both-way semantic content atomically: items/components, block
   states/chunks/palettes, entities/serializers, recipes/tags/menus and remaining
   content-bearing paths. Numeric maps alone do not supply those semantics.
4. Define a usable extension contract: per-connection state, negotiated sends and
   replies, configuration AND play routing, lifecycle/reset hooks, asynchronous
   completion and bounded diagnostics. Current stateless callbacks are not this.
5. Prove isolated exact loader-only and minimal fixture-mod interoperability with
   meaningful bidirectional world interaction and reconnect/failure coverage.
   Conditional tasks need controlled fixtures even if loader-only omits them.

Both core and engine finish refusals remain during owner/task/content work. Once
all prerequisites genuinely exist, replace hard refusals with reviewed conditional
completion barriers in a dedicated milestone, and validate those barriers with
live tests before claiming completion. Never remove the refusals unconditionally
or forge finish simply to obtain a join log. Use isolated exact-version servers,
not the auto-updating production pack. No individual mod rewrite belongs here.
