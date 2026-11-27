# Bounded split transport primitive — not yet advertised

Exact baseline: NeoForge 21.1.248 / Minecraft 1.21.1 / protocol 767.
`src/split.rs` supplies borrowed payload decoding and one-direction, one-phase,
one-connection reassembly. Audited source digests: `provenance/split.json`.

Source artifact hash is pinned in that manifest and the existing NeoForge manifest.
`SplitPacketPayload.STREAM_CODEC` delegates to `UNBOUNDED_BYTE_ARRAY`, whose
FriendlyByteBuf `readByteArray` / `writeByteArray` include a VarInt byte length.
Thus the marker is NOT the first byte of the payload body: it is the first byte
of the length-prefixed array. `GenericPacketSplitter` uses first=1, middle=0,
last=2, and slices the full encoded Minecraft packet, including its packet ID.
There is no fragment index/count, packet ID prefix outside that reconstructed
packet, or per-fragment channel ID in its content.

Local stricter policy: nonempty content, known standalone markers, first before
middle/last, no replacing a pending first, 1 MiB content per part, 8 MiB per
assembly, 256 parts, 30 s total assembly deadline (never refreshed by traffic).
Every error releases partial bytes and permanently poisons that epoch. New
connections need fresh instances. The host supplies monotonic elapsed milliseconds,
checks deadlines on its timer even without incoming packets, invalidates on
phase change/disconnect/reconfiguration, and refuses finish while pending.

This module intentionally uses a separate transport budget, not the common
32767-byte body policy. It allocates only bounded reconstructed packets, returns
nothing until last, and cannot authorize a modded channel or configuration finish.
The host must bound its queue and total connection traffic, reject recursively
split packets, decode a reconstructed original packet in its original phase and
protocol exactly once, then await actual handling. No production host wiring
is supplied. Loader does NOT advertise or accept `neoforge:split`; unsupported
split traffic still disconnects rather than being acknowledged or ignored.

18 tests passed debug/release: exact array framing, multibyte packet ID
preservation, sequential messages, every prefix truncation, bad lengths/markers,
ordering, per-part/aggregate budgets, clock reversal, timeout, terminal cleanup
and fresh epochs. These are synthetic transport-unit tests, not Java differential
split comparisons, actual Horizon socket decoding or a server join. No outbound
splitter, compression/encryption/framing or recursive-dispatch host is supplied.
