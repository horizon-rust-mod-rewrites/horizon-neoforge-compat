# Latest exact-SHA hosted validation: outbound milestone PASSED

Run: https://github.com/horizon-rust-mod-rewrites/horizon-neoforge-compat/actions/runs/46087704109
Tested compatibility SHA: `fb0294b4976bd78d4ee4cbfce44a6f049e0a4307`.
Horizon pin: `ae632edf48f6cae3ba52a5c81523ec712488c9cc`.
Artifact SHA-256 verified: `357207d45ab75c4e2cf94485f31f661915498617bb9fc51db73e570ed3670f5f`.
Retained `integration/hosted-46087704109-{core,full-check,full-driver,scope}.txt`.

- Full actual patched client + companion typechecks passed; pinned lock audit passed.
- **16 actual companion**, **9 early configuration**, **14 protocol framing**,
  **81 network-filter passed / 3 ignored**. Selected suites, NOT all upstream tests.
- **171 core tests EACH debug and release**, including **36 split tests**;
  strict Clippy, fmt, warning-denied docs, 35 Python tests and six fixtures passed.
- New actual outbound preparation tests and the TCP split/compression/encryption
  matrix REALLY executed. No production split route, Java split equivalence,
  complete loader tasks, semantic content application, UI or live join is proven.

Default split/feature-flag opt-ins remain disabled. Both finish refusals remain.
**The client STILL CANNOT JOIN.** Newer evidence/documentation-only commits do not
change the tested source and deliberately do not trigger another full build.
See NEXT_AGENT.md for precise continuation priorities. Older contradictory
counts/pending statements below are historical, not current validation status.

---

# Concrete outbound configuration preparation (not enabled)

Baseline: Minecraft 1.21.1 / protocol 767 / NeoForge 21.1.248, Horizon
`ae632edf48f6cae3ba52a5c81523ec712488c9cc`. The six-file integration patch now
includes `ConnWriter::prepare_serverbound_configuration`, complementary to the
existing reconstructed-original reader. **Both finish gates remain. Default
split/feature-flag opt-in stays disabled. The client STILL CANNOT JOIN.**

## Why not call the normal writer?

The actual `ConnWriter::send` calls `Session::serverbound`, frames/compresses/
encrypts the translated body, changes counters, writes the socket and drains
adapter-generated outgoing bodies. `Session::serverbound` independently reads
its writing phase and can change phases or invoke stateful play translation.
Calling either for preparation would risk effects before a complete bounded
split batch exists. A replacement/cloned Session would not prove ownership of
the real connection, phases or queued packets.

The new preparation method instead uses the existing shared mapping API
`ServerVersion::serverbound_id(State::Configuration, id)`. Exact-pin inspection
of `adapter/translate.rs` establishes why this is sufficient ONLY for its narrow
allowlist: custom payload, keepalive, pong and known packs reach the byte-preserving
configuration fallback. Client information has a real 1.21.1 conversion and is
excluded; older resource-pack conversion and all play/content translation are
also excluded. This is not a second general-purpose translator.

The generated protocol-767 configuration serverbound table maps these four IDs
2/4/5/7 identically. There is no nonidentity outbound configuration case in this
allowlist. Inbound known-packs 14 -> 15 remains separate evidence.

## Preparation contract

Input is one complete PINNED-version unframed original (VarInt packet ID + body).
Output is one complete WIRE-version unframed original for `OutboundSplitter`.
Before decoding/mapping the method requires:

- Non-memory transport, no older-client Session, an actual exact `V1_21_1` server
  adapter, and BOTH reading/writing phases in configuration.
- Nonempty input, at most 8 MiB, a valid packet ID and an allowlisted kind.
- Actual directional `cfg::Serverbound::decode`, including full consumption.
  Opaque data uses the actual 32,767-byte fallback cap, excluding channel.
- No recursive `neoforge:split`, and no engine-owned/control channel identified
  by the existing `is_reserved_channel` authority. Namespace normalization comes
  from the actual identifier codec, not a second string parser.

Allowed kinds are opaque custom payload, signed-i64 keepalive, signed-i32 pong,
and known-packs list. Cookies, client information, resource-pack responses,
finish, custom clicks/code of conduct, unknown and content packets refuse even
when a numeric mapping exists. Known-packs preparation is not authorization to
claim pack support. Opaque payload preparation does not authorize its channel.

Only the packet ID is canonicalized with the actual VarInt writer. Body bytes
are copied unchanged after validation, retaining valid noncanonical payload
VarInts. Output reservation is fallible and at most the input size because all
four exact mapped IDs are one byte. Existing codec internals still allocate;
this change does not provide allocator-failure injection for those internals.

Success and failure do not write, frame, encrypt, change counters/thresholds,
mutate the shared Session or consume either adapter queue. Errors are PURE
refusals, not automatic transport shutdown. A detached network writer works,
but detached status does not attest socket liveness. The future host owns
terminal close/reset and must invalidate its Loader epoch on failure.

## Required ownership before production use

Phase checks are observations, NOT a shared Session lock. The future host must
serialize preparation, whole-batch admission, writes and BOTH phase transitions;
rechecking or using a temporary Session cannot supply that atomic ownership.
There is no epoch token, once-only runtime marker or enabled caller here.

1. Establish genuine negotiated serverbound configuration split authority and
   the actual framing/compressor threshold under the ordered connection owner.
2. Prepare once; split the returned full wire original once.
3. Frame `Unsplit` directly through the wire-body path. Do NOT call ordinary
   `send`/`Session::serverbound` on it again. For fragments, encode the actual
   split envelopes and map their envelope ID once, then use that same direct
   framing path. A split envelope deliberately cannot enter preparation again.
4. Bound and publish the complete batch atomically, preserving serialization with
   other originals and phase changes. Apply compression/framing/encryption and
   counters only at emission. Preserve independent adapter queue ordering.
5. Await real dispatch/write completion as required. Any admission/write failure
   must close/reset, never create a success/task/finish acknowledgment.

The inspected existing engine `QueuedWriter` already has a 16 MiB byte-channel
budget and disconnects on ordinary-send backpressure. That does NOT establish
atomic publication of a multi-envelope split batch, completion awaiting, phase
ownership or an idle assembly timer. Do not replace it with an unbounded queue
or claim those missing guarantees from the existing ordinary send API.

## Authored actual-dependency tests — execution pending

Eight new protocol connection tests cover:

1. All four allowlisted kinds match independent literal wire bytes and the real
   ordinary adapter result; opaque bytes, nonempty known packs and signed order.
2. Opaque bodies at 0/32,767 pass, 32,768 refuses; recursive/reserved/normalized
   brand channels refuse.
3. Actual malformed/truncated/trailing IDs/bodies/counts/strings and oversized
   originals refuse; ID canonicalization and unchanged payload VarInts.
4. Every other modeled configuration kind and unknown/finish refuse without a
   phase change, ack or socket shutdown; subsequent preparation remains usable.
5. Missing/current/wrong adapter, either wrong phase, older-client mode and actual
   configured memory transport refuse. Wrong-version configured state is real.
6. Repeated success/refusal preserves all counters, socket bytes, thresholds and
   cipher state; subsequent real compressed/encrypted frames match a control.
7. Actual detached network preparation performs no socket I/O.
8. Real finish/interact adapter translation seeds actual incoming/outgoing queues;
   real transitions return to configuration; preparation preserves both queues
   byte-for-byte against an independently seeded control. No fake queue setters.

Two additional full-engine companion tests connect the actual preparation method
to the actual core splitter and actual TCP/framing paths:

- Fragmented opaque payload crosses real socket writes and serverbound envelope
  decoding, then reassembles byte-exactly, in all four compression/encryption
  combinations. A subsequent ordinary keepalive still works.
- A real Loader query reply remains borrowed/unsplit, crosses actual TCP without
  an invented marker, and its endpoint still refuses finish.

These are authored regressions, NOT observed passes. The full-driver floors are
now **16 companion / 14 protocol / 9 early / 81 network-filter**. They do not enable
production split routing, demonstrate Java split equivalence or validate a join.
The isolated checker still excludes actual-engine modules explicitly, without
substitute Horizon APIs.

## Source provenance and local checks

Available original-source files were rehashed against the existing manifests.
Only small exact-pin source retrievals were used; no full source, compiler,
dependency graph, game file or toolchain was downloaded locally. Additional audit
source identities (paths relative to `crates/horizon-protocol/`):

| Source | Bytes | SHA-256 |
| --- | ---: | --- |
| `src/buf.rs` | 34,410 | `a96f3e0b59b0d7e1a468fd0f7e8be0d37d842e35767336cb58527d71212c2179` |
| `src/packets/mod.rs` | 6,044 | `8e2aeb0e7602b432267dbc2f126f82241692f0845c783919774769d89ccd39dc` |
| `src/ids.rs` | 22,066 | `3d8d58c5de5e949aca9a0fe3bad6ade35432e6395006c738c48fe3eb3fa409d6` |
| Client `src/net/queued_writer.rs` | 14,190 | `d0cad2382a80d0fb1600b937facc45b813d80089dba83634f2e37ea4b778410e` |

Executed locally: 35 Python tests, six fixtures, whitespace checks and exact-pin
hash checks. Regenerated six-file patch cleanly applies, matches edited bytes,
refuses duplicate application, reverses and restores original bytes. New Rust
compile/tests, rustfmt and Clippy remain pending hosted execution. No production
server or isolated NeoForge server was started. Completed older CI applies only
to its own tested SHA; consult NEXT_AGENT.md for exact retained run evidence.
