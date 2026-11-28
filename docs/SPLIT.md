# Bounded split transport primitives — not yet advertised

Exact baseline: NeoForge 21.1.248 / Minecraft 1.21.1 / protocol 767.
`src/split.rs` supplies borrowed payload decoding, one-direction/phase/connection
reassembly, and a concrete bounded outbound configuration encoder. Audited source
digests: `provenance/split.json`. Neither primitive enables a channel, supplies
negotiation authority, acknowledges tasks, or permits configuration completion.
**Both finish refusals remain. The client still cannot join.**

## Outbound preparation and inbound resource follow-up

The actual Horizon patch now supplies a narrow, effect-free configuration writer
preparation method. Two authored full-engine tests connect it to this splitter
and real TCP/framing/compression/encryption paths. No production split caller is
enabled. See [the outbound seam contract](horizon-split-outbound-seam.md).

Inbound growth now reserves fallibly before extending, avoiding an infallible
allocation attempt under memory pressure and geometric over-reservation near the
aggregate cap. Reservation failure follows the existing terminal cleanup path.
Five additional core regressions cover irregular exact-8-MiB completion/ownership,
per-assembly deadlines without resetting the epoch clock, invalid LARGE IDs,
fragmented nonminimal IDs, and adjacent directional-cap saturation thresholds.
There are now **36 authored split tests / 171 expected total core tests**, not
new observed passes. Allocation-failure injection itself remains untested.
New follow-up Rust execution, fmt and Clippy remain pending hosted CI.

## Exact wire evidence

Source artifact hash is pinned in that manifest and the existing NeoForge manifest.
For this outbound implementation, only the three source members in `split.json`
were retrieved using bounded HTTP ranges (17,164 extracted bytes total); all three
member SHA-256 values matched the existing audit. No whole artifact, compiler,
Java game files, dependencies or source checkout was downloaded or added here.

`SplitPacketPayload.STREAM_CODEC` delegates to `UNBOUNDED_BYTE_ARRAY`, whose
FriendlyByteBuf `readByteArray` / `writeByteArray` include a VarInt byte length.
Thus the marker is NOT the first byte of the payload body: it is the first byte
of the length-prefixed array. Each output body is precisely:

    VarInt(content length + 1), marker, content

`GenericPacketSplitter.encode` uses first=1, middle=0, last=2, and slices the full
encoded Minecraft packet including its wire packet ID. It preserves originals
at/below its active packet limit, also explicitly passing through a one-part
result. There is no first|last=3 representation, single-fragment encoding,
fragment index/count, repeated original packet ID, or channel ID inside content.
`createPacket` selects the custom-payload envelope by encoder flow;
`commonBidirectional` registers the payload. This implementation restricts itself
to configuration rather than claiming play support.

`determineMaxPayloadSize` reserves **26 bytes**: one outer packet-ID byte,
15 bytes for the length-prefixed `neoforge:split`, one marker, five VarInt bytes,
and four further bytes. This is a conservative size calculation, NOT an extra
wire header. The actual output only includes the array length, marker and slice.
Under our local caps its array VarInt is at most three bytes; even a five-byte
outer packet ID plus channel plus body fits that reservation (5+15+3+1=24).

Upstream chooses its original-packet limit according to the actual compressor's
presence (uncompressed allowance when compression follows the splitter, compressed
allowance otherwise). No such host pipeline has been wired here, so the new API
requires the caller's real active packet threshold rather than guessing Minecraft
compression constants or installing a default transport profile.

**Intentional differences:** upstream takes `buf.array()` and computes a slice's
last extent using that backing array's length, although its part count uses
`readableBytes()`. This primitive accepts an exact unframed packet slice and NEVER
copies unused backing capacity, pads a final part or leaks bytes beyond that
slice. No claim of reproducing that allocator-dependent behavior is made. Local
size/ordering policy is stricter, and directional caps can produce smaller/more
parts than the Java splitter. These are synthetic source-derived wire tests, not
Java differential split evidence or captured server traffic.

## Outbound API and limits (new Rust execution pending)

`OutboundSplitter::new(flow, protocol, packet_limit)` requires an explicit
`modded::Flow` (serverbound/clientbound, no bidirectional/unknown flow), the
configuration protocol, and a threshold in **27..=8,388,608**. Other phases,
zero/too-small/overflow-sized limits refuse before any output. There is no Default.
Construction is a size profile, **not proof** of an active host phase or channel.
Discard/reconstruct it when the host's phase/flow/compressor/connection changes.

`encode(original)` takes **the full encoded wire ID + packet body**, before any
socket length framing, compression or encryption. It rejects empty input,
truncated/negative/overflowing packet-ID VarInts and originals over 8 MiB. It does
not remap IDs, demand a canonical VarInt, or pretend that a syntactically valid ID
is implemented. Bodies/IDs must already be encoded with the actual configuration
codec for the correct wire direction/version. The packet-ID VarInt may cross a
fragment boundary and is preserved byte-for-byte.

- At/below `packet_limit`, returns `OutboundPacket::Unsplit` borrowing the exact
  original slice, with **no copy/allocation and no marker**. The original must
  travel by the host's ordinary packet path, not as a split custom payload.
  Directional fragment caps do not force smaller originals to split.
- Above that threshold, returns `OutboundPacket::Split(Vec<Vec<u8>>)` containing
  actual complete split payload bodies in order, with at least two nonempty
  parts, first=1/middle=0/last=2. No bytes escape incrementally.
- Content width starts at `packet_limit - 26`, then is reduced to the existing
  1 MiB per-content bound and the applicable payload-body cap **including the
  array VarInt and marker**. Serverbound body cap is 32,767 (maximum content
  **32,763**); clientbound body cap is 1,048,576 (maximum content **1,048,572**).
  These conservative local caps match the inspected Horizon opaque configuration
  payload path. They are NOT claimed to be a limit in NeoForge's registered
  `UNBOUNDED_BYTE_ARRAY` codec, nor proof of any future registered codec integration.
- Aggregate original bytes remain at most 8 MiB, parts at most 256. Serverbound
  fragmentation consequently supports at most **8,387,328** original bytes
  (32,763*256), possibly fewer for a smaller threshold; an 8 MiB fragmented
  serverbound original refuses rather than exceeding the part budget.
- The full count and aggregate encoded-byte upper bound are checked **before
  output allocation**. Payload bytes are bounded by `8 MiB + 256*6`; at most 256
  Vec headers plus the bounded outer Vec are additionally needed. All Vec storage
  uses fallible reservation. Size/count/allocation errors return only an error;
  any privately built bodies are dropped. No iterator, callback or network write
  can expose a partial batch. Allocator-failure injection is not tested.

The host still must establish negotiated phase/direction authority, forbid
recursive split/finish/phase-changing and unsupported semantic packets, check its
real framing/compression constraints, and bound pending queued batches/traffic.
A numeric threshold alone does not validate an original custom payload's own codec
limits. Queue the complete result atomically, serialize it with other originals
and phase changes, and on write failure disconnect/reset rather than send an
invented success acknowledgment. The primitive cannot enforce those host-owned
conditions from opaque bytes and is **not connected to the outbound writer**.

## Inbound policy (unchanged)

Nonempty content, known standalone markers, first before middle/last, no replacing
a pending first, 1 MiB content per part, 8 MiB per assembly, 256 parts, 30 s total
assembly deadline (never refreshed by traffic). Every error releases partial bytes
and permanently poisons that epoch. New connections need fresh instances. The host
supplies monotonic elapsed milliseconds, checks deadlines on its timer even without
incoming packets, invalidates on phase change/disconnect/reconfiguration, and
refuses finish while pending.

This module intentionally uses a separate transport budget, not the common
32767-byte body policy. Inbound decoding retains its existing larger bounded
budget, and is NOT newly tightened to the outbound directional policy. Reassembly
returns nothing until last. The host must bound its queue and total connection
traffic, reject recursively split packets, decode an original in its original
phase and protocol exactly once, then await actual handling. No production host
wiring is supplied. Loader does NOT advertise or accept `neoforge:split`;
unsupported split traffic still disconnects instead of being acknowledged/ignored.

## Tests and actual validation

The existing **18 inbound tests** previously passed debug/release. This change adds
**13 outbound Rust tests**, for **31 split tests authored**:

- Independent literal multi-part fixtures for both directions, first/middle/last,
  wire ID 128, a five-byte ID spanning parts, and a one-byte final fragment.
- Literal array VarInts at lengths 127/128 and 16,383/16,384, plus literal directional
  maximum-size prefixes. Expected fixtures do not call this encoder or the inbound
  frame helper/decoder; round trips through the real reassembler are additional.
- Exhaustive 59,200 cases: both directions, all thresholds 27..=100 and original
  lengths 1..=400, covering every small remainder, threshold equality/+1, exact
  divisions, part-count refusals and one-byte tails. Expected body bytes are
  checked independently, not merely encoder/decoder agreement.
- Exact directional body budgets, maximum parts/next part, maximum aggregate/next
  byte, malformed IDs, invalid thresholds/phases and borrowed unchanged originals.

Executed locally for this change: **35 existing Python driver tests pass**, **six
existing fixture checks pass**, and `git diff --check` passes. The three exact
NeoForge source-member hashes above were checked. **No compiler or rustfmt is
installed: the 13 new Rust tests, Rust formatting, strict Clippy and hosted compile
are pending actual execution.** No new Rust test is reported as passing. These
checks do not demonstrate actual socket writing, Java equivalence, production
split ingress, complete task execution or a NeoForge join. CI evidence/publishing,
patch/workflows, parent handoff and checksums remain parent-owned and untouched by
this implementation.

## Concrete protocol-767 decoder seam (separate work)

The Horizon patch adds `ConnReader::decode_reassembled_configuration` in the actual
pinned protocol dependency. It takes a complete unframed original packet, shares
packet-ID extraction with the ordinary/compressed socket path, checks both adapter
phases and exact version, and adapts the original wire ID once. This is a concrete
transport boundary, not an enabled split handler. Default offers and Loader split
refusal are unchanged.

Its deliberately narrow allowlist is opaque custom payloads, keepalive, ping and
known packs. Unknown, finish, registry/tag/content packets and recursively split
custom payloads refuse before adaptation. Rejection poisons subsequent reads and
shuts down attached transport. Detached sockets require an external real closer.
No content-semantic support is inferred. See `horizon-split-seam.md` for authored
actual-dependency tests, evidence scope, and remaining integration requirements.
This outbound change does not edit that patch or claim its pending tests passed.
