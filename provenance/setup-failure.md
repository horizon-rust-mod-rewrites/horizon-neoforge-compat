# Setup-failure NBT data: exact baseline, evidence and limitations

Baseline: **NeoForge 21.1.248, Minecraft 1.21.1, protocol 767**.
Audit and execution date: 2026-11-27. No individual mod work or join claims.

## What was implemented (and what was not)

`setup_failure::SetupFailure::decode` consumes a complete custom-payload body.
Its `reasons()` are **lossless NBT data, NOT rendered or semantically validated
Minecraft Components**. This is the explicitly bounded data-representation
fallback, not completion of every Component content/style/event codec.

- All twelve non-End NBT types, ordered compound entries, homogeneous lists,
  signed numbers/arrays, raw float/double bits, and Java modified-UTF-8 strings
  are retained. Each string exposes original bytes and UTF-16 units, preserving
  embedded NUL, supplementary characters, lone surrogates, and even noncanonical
  encodings accepted by Java `readUTF`. No lossy Unicode conversion occurs.
- `original_body()` retains the exact input, including map order, nonminimal
  VarInts, identifier shorthand and string encodings. No failure send/encoder API
  is added. This is forensic retention, not permission to echo untrusted input.
- Invalid NBT types, lengths, encodings, missing terminators, duplicate keys,
  truncation and trailing data fail. Root scalar/array/End values and empty
  Component lists fail envelope checking. Nonempty lists recursively require
  string/list/compound envelopes. Nested NBT lists may be empty.
- **Envelope checking is NOT Component validation.** Compound contents are
  opaque, including NbtOps heterogeneous-list wrapper compounds. For example,
  `{}` is structurally valid NBT and is intentionally representable here even
  though the actual Java Component codec rejects it. Unknown content variants,
  invalid style/event fields, selectors, registry references and translation
  arguments are not certified. Do not use a successful data decode as a
  Component validation oracle or a safe-rendering assertion.
- No translation lookup, click/hover execution, URL fetching, selector/NBT
  resolution, text flattening or HTML/terminal rendering is performed. Errors
  contain fixed diagnostics or bounded ASCII ResourceLocations, not reason text.
  A future UI must implement its own explicit escaping and safe display policy.

`Loader` now decodes this channel atomically, saves the complete result behind
`setup_failure()`, and **always returns `Error::SetupFailed`**, including for a
zero-reason map. Normal failure cleanup revokes accepted channels/announcements;
there is no outgoing acknowledgment, retry, success or finish path. Invalid
bodies save no partial reasons. Diagnostics survive terminal errors for caller
inspection, and `close()` discards them; a new Loader starts empty. Production
glue must inspect/copy diagnostics before closing if it needs to preserve them.
The existing legacy `modded::reject_setup_failure` helper is unchanged and still
always rejects; it is not the new data decoder. No split transport channel was
advertised/routed: only the requested `pub mod split;` declaration was added.

## Bounds (local policy, not upstream maxima)

| Budget | Limit |
|---|---:|
| Complete body | existing `codec::MAX_BODY` = 32,767 bytes |
| Reasons | existing `modded::MAX_FAILURE_REASONS` = 1,024 |
| ResourceLocation | existing 256-byte ASCII/normalization policy |
| Depth, root counted as one | 32 |
| Aggregate nodes, across **all reasons** | 4,096 |
| Individual list/array length | 4,096 and remaining node budget |
| One NBT string/name, encoded bytes | 8,192 |
| All NBT strings/names, encoded bytes | 16,384 |

Every tag costs one node, including empty containers; array scalar elements
also cost one each. Compound names count toward both string limits. Lists check
the declared count against remaining nodes before reading children; arrays
charge all elements before reading them. Counts use checked/nonnegative
conversion; no peer-sized preallocation. Node/depth/string budgets are shared
across roots, not reset per map entry. Body retention is at most 32,767 bytes.
Decoded strings use at most one UTF-16 unit per encoded byte and retain original
bytes; temporary duplicate-key sets store bounded copies of those units.
Container/vector/B-tree overhead is bounded by the aggregate node/reason caps.
Runtime is bounded by body parsing and bounded key comparisons; no unbounded
recursion, decompression or follow-up I/O is triggered by data.

Duplicate reason IDs compare normalized identifiers. Duplicate compound names
compare decoded Java UTF-16 units, so differently encoded names with equal
units cannot evade detection. This intentionally rejects Java map-overwrite
inputs. Negative lengths and unknown empty-list element types also fail rather
than reproduce more permissive or exception-dependent upstream behavior.
The lower bounds can reject real diagnostic payloads and must not be described
as complete interoperability with upstream's trusted/unlimited codec.

## Exact source and classfile audit

NeoForge artifact:
`https://maven.neoforged.net/releases/net/neoforged/neoforge/21.1.248/neoforge-21.1.248-sources.jar`

SHA-256 (downloaded bytes and publisher `.sha256` checked independently):
`b4b912457549ff586873ce78b39b08af96f11a0e3a54c6b77576c6a25907083f`.
`net/neoforged/neoforge/network/payload/ModdedNetworkSetupFailedPayload.java`:
`8eb12a4ec315f0463ed71b9be1f64e21defb49526ea1a9f43a5236605dd1612a`.
Its exact composition is a VarInt-sized map of ResourceLocations and
`ComponentSerialization.TRUSTED_CONTEXT_FREE_STREAM_CODEC` values. It is not a
JSON string map and has no success indicator.

Official MC downloads and publisher SHA-1 were verified using the existing
`tests/reference/audit_minecraft.py` in external scratch storage. Metadata and
server/mappings SHA-256 match `modded-codecs.json`:

- Version metadata: `bb0a1413744530df54451e4f973bac81b1e27477999e79cfe7e66aaf6d4db6e6`.
- Server bundle: `e3bc55693e93cda0188f2e60aea28113fc647c5e85a15fa3d1b347349231b4bb`.
- Inner server: `c301de10f575027d13eac18c7f34409d60648cf56a35d566aa1f530ff617840a`.
- Official mappings: `9d0b04bead421c8229aff14b534432bbc927bea642e7c8593d1276b8df8ba53f`.

Exact 1.21.1 official mappings plus `javap -c -p` on those classfiles establish:

1. `ComponentSerialization` (`xb`) static initializer assigns field `f` via
   `ByteBufCodecs.fromCodecTrusted(CODEC)` (`yv.a(Codec)`). `createCodec` permits
   string, nonempty recursive Component list, or variant/style compound.
2. `ByteBufCodecs` (`yv`) constructs a tag codec and maps through
   `CODEC.parse(NbtOps.INSTANCE, tag)` / `encodeStart`. Its nested `$5` delegates
   tag reads/writes to `FriendlyByteBuf` (`vw`), rejecting null/End.
3. `FriendlyByteBuf` delegates to `NbtIo.readAnyTag` / `writeAnyTag` (`uo.b` /
   `uo.a`). These read/write **one tag ID then its payload**, with no root name,
   no NBT compression and no registry prefix. `readUnnamedTag` is a different
   method and must not be substituted.
4. `TagTypes` (`vb`) fixes tag ordinals 0..12 as End, Byte, Short, Int, Long,
   Float, Double, ByteArray, String, List, Compound, IntArray, LongArray.
5. Primitive tag loaders call Java DataInput fixed-width big-endian reads.
   StringTag `$1` calls `readUTF` (u16 byte length + Java modified UTF-8).
   ListTag `$1` reads element byte + signed i32 count and then untagged values;
   CompoundTag `$1` reads child type, `readUTF` name, child payload until End.
   Array loaders read signed i32 lengths and the corresponding scalar elements.
6. `NbtOps` (`up`) supports heterogeneous-list wrapper compounds. Their data is
   preserved without interpreting wrappers as a rendered Component.

Selected audited exact class SHA-256 values (no classfiles redistributed):

| Official name / obfuscated class | SHA-256 |
|---|---|
| ComponentSerialization / xb | `770a79c4c2d6c9e06e0bdb4475020eeabd31ee4fd8f0be93e9f4dff3dc24ea3f` |
| ByteBufCodecs / yv | `39a150fb2ef2a95792b4315a5924c98a7e62b7b8ff7058cba3274caaff0cb223` |
| tag codec / yv$5 | `38a425057005a87258fe7b8bb28e8debb2ddbdfa847184a3b5b0133d3967c5d6` |
| FriendlyByteBuf / vw | `6412f24ac18b1dcfbb4cec0d1e8e553f2eb631f492a777d3af8a1e7c324d5a09` |
| NbtIo / uo | `24c6803a86c744f5fc1edb771215a72b8abef2d1c747544a5bba1fafce0a06e9` |
| StringTag loader / uw$1 | `e053dc1a0d0ea64e70386626083f9e95b9a894a70f3d642ccaeaf7c3c5e43071` |
| ListTag loader / uh$1 | `93c8ca3e3b1d64f7f46afce82df198c474f48cb0733d3af419eaec0cf9428c88` |
| CompoundTag loader / ub$1 | `0eae35e324eeb2c75fdd2c2e3d8a5dd074975168269fd0491095b5509249418a` |
| TagTypes / vb | `40ac400e22bbd498ba5c9f6ac4eecf8a4abf6a217e4545d14f0a50036ad28eb9` |
| NbtOps / up | `92da031ee7131b5538e42b0e02a0f64f09066a85302278331d233f2af0bef45b` |

## Limited genuine Java reference execution

Unlike the earlier classfile-only audit, **Java was actually executed here**:
Azul Zulu OpenJDK 21.0.13.1+2-LTS, runtime Zulu21.54+201-CA. The external runtime
and verified dependency artifacts prepared by the parallel reference workstream
were reused read-only. No Minecraft/NeoForge implementation is vendored by this
change. This is eight manually selected reference cases, not a complete
failure/Component differential harness, randomized equivalence proof or capture.

Preparation was performed in scratch storage **outside the repository**:

- Extract only the exact record declaration and `STREAM_CODEC` initializer
  from the checksum-verified NeoForge source above, dropping the payload
  interface, channel/type declarations and unrelated `type()` method.
- Add `import java.util.*;` and substitute official names with mapped names:
  `ComponentSerialization.TRUSTED_CONTEXT_FREE_STREAM_CODEC` -> `xb.f`,
  `ResourceLocation.STREAM_CODEC` -> `akr.b`, `StreamCodec.composite` -> `yx.a`,
  `ByteBufCodecs.map` -> `yv.a`, `FriendlyByteBuf` -> `vw`,
  `ResourceLocation` -> `akr`, `Component` -> `wz`, `StreamCodec` -> `yx`.
  The extracted/adapted source SHA-256 was
  `2bbfed1303c540ff2db27b8955617843be9853270ffd35056f0420c292246e83`.
  Only names/API linkage changed; map and value codec composition were retained.
- Compile this extracted record and an original test driver with
  `javac -proc:none -cp '<unsigned-runtime.jar>:<verified-libraries>/*'`.
  The runtime is a scratch copy of the exact official inner server with only
  signature metadata removed so unsigned default-package test classes can
  link; game class bytes are unchanged (same procedure as reference preparation).
- The driver calls `ab.a(); akt.a();` (detect version; Bootstrap.bootStrap),
  creates `new vw(Unpooled.wrappedBuffer(HexFormat.of().parseHex(body)))`, calls
  the extracted record's `STREAM_CODEC.decode`, rejects remaining bytes, and
  calls `STREAM_CODEC.encode` on a new `vw(Unpooled.buffer())`.
- Run with `java -Xmx384m -cp '<driver-classes>:<unsigned-runtime.jar>:<verified-libraries>/*'`.
  Keep driver classes in a separate directory from downloaded version metadata;
  a top-level metadata `version.json` shadows the game's own resource otherwise.
  No world/server socket/account was opened; bootstrap initializes game registries.

Observed bodies (all use one `demo:bad` reason except the empty map):

| Input body, hex | Actual Java outcome |
|---|---|
| `00` | accepted, re-encoded `00` |
| `010864656d6f3a626164080003626164` | string `bad`, accepted; identical output |
| `010864656d6f3a6261640a08000474657874000362616400` | compound `{text:"bad"}`, accepted; normalized to the preceding string body |
| `010864656d6f3a62616409080000000200036f6e65000374776f` | list `["one","two"]`, accepted; output below |
| `010864656d6f3a6261640a00` | empty compound, Component DecoderException (NBT data decoder deliberately preserves it) |
| `010864656d6f3a6261640100` | byte root, Component DecoderException (our envelope check also rejects) |
| `010864656d6f3a626164080002c080` | modified-UTF-8 NUL, accepted; identical output |
| `010864656d6f3a626164080003eda080` | lone high surrogate, accepted; identical output |

Java's normalized list output was:
`010864656d6f3a6261640a09000565787472610800000001000374776f0800047465787400036f6e6500`.
Our decoder instead preserves the original input exactly, as required of an NBT
**data** representation. It does not claim Java Component re-encoding semantics.
The six accepted inputs are exercised literally in `tests/setup_failure.rs`.

A separate original Java driver also executed JDK `DataInputStream.readUTF` on
all thirteen string cases in the Rust modified-UTF-8 test. It prepended the u16
encoded-byte length, called `readUTF`, and printed UTF-16 `charAt` units. Actual
results: `c080` and `00` -> `0000`; `eda0bdedb880` -> `d83d de00`; `eda080` ->
`d800`; `c181` and `e08181` -> `0041`. Inputs `80`, `c0`, `c041`, `e180`,
`e14180`, `f09f9880`, `ff` all threw UTFDataFormatException. Thus permissive
noncanonical Java encodings and rejection of standard four-byte UTF-8 are
execution-checked, not inferred from Rust self-roundtrips.

Initial execution attempts failed before initialization (missing bootstrap,
then metadata resource shadowing); these were diagnosed and corrected, and are
not counted as successful Component comparisons. This limited manual execution
is separate from the ongoing query/setup differential workstream.

## Rust validation actually executed

Rust 1.95.0 (`59807616e`), `x86_64-unknown-linux-musl`, bundled rust-lld;
`cargo test --offline --locked --target x86_64-unknown-linux-musl
--test setup_failure --test loader`, debug and release:

- **14 setup-failure tests and 13 Loader tests pass** in both profiles.
- Owned Rust files checked with rustfmt. `cargo doc --offline --locked
  --target x86_64-unknown-linux-musl --no-deps` passed with
  `RUSTDOCFLAGS='-D warnings'`. No new Clippy execution is claimed; this
  installed toolchain did not include Clippy at validation time.
- Tests cover all twelve payload types; literal network-NBT roots; original
  byte preservation; nested style/event data without execution; Java modified
  UTF-8/NUL/surrogates/overlongs and malformed encodings; decoded-key duplicates;
  root/list-envelope errors; invalid types/counts/lengths/terminators; every
  prefix truncation of populated all-type and multi-reason bodies; trailing
  bytes; exact depth/node/string/reason boundaries and aggregate budgets; huge
  declared lengths; deeply nested attacks; body/identifier/VarInt limits;
  4,096 deterministic malformed samples in three framing contexts (smoke only).
- Loader tests cover pre/post-setup terminal failure, zero reasons, no partial
  data, no retries/finish, prior handler grant revocation, retention and close.

A first smoke-test run exposed an empty-input indexing bug in its **test fixture
builder**, not the decoder. The fixture builder was corrected and all tests
rerun. An earlier build attempt hit the shared sandbox disk quota before
compilation completed; it is not counted as validation.

## Remaining limitations and ownership

No full Component semantics/renderer or broad Java failure differential corpus.
No packet capture, socket test, live NeoForge join, actual Horizon diagnostic UI,
content mapping, full loader task support or configuration-success authorization
is supplied by this change. The client still cannot join. Historical README,
WIRE and progress claims were not edited because they belong to other work.
This document supersedes their statement that Loader cannot retain failure data,
not their warnings about incomplete Components/handshake/integration.

Owned changes only: new `src/setup_failure.rs`, `tests/setup_failure.rs`, this
provenance document; additions in `src/lib.rs`, `src/loader.rs`, `tests/loader.rs`.
The `split` implementation/tests and toolchain/reference work are separate.
There was no `.git` directory in this supplied archive checkout, so remote/HEAD
verification required by AGENTS.md was unavailable; no commit or push was made.
All downloaded references, extracted source, disassembly and build scratch for
this work stayed outside the repository. Existing source notices/licenses still
apply; no game implementation/assets or upstream Java source are redistributed.
