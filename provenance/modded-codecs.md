# Modded payload codecs: implementation and evidence handoff

Baseline **Minecraft 1.21.1 / protocol 767 / NeoForge 21.1.248 only**.
This is codec work, not a negotiation controller, game adapter, joined client,
or a compatibility/support claim. No individual mods were rewritten.

## Public API

The library exports these codecs through its `modded` module.

- `ConnectionProtocol::{Handshaking, Play, Status, Login, Configuration}`:
  `ordinal() -> i32`, `from_ordinal(i32) -> Result<Self>`. All five exact wire
  enum values are represented. These are NOT textual protocol IDs, packet IDs,
  or phase indices invented for this crate.
- `From<codec::Phase> for ConnectionProtocol` and
  `TryFrom<ConnectionProtocol> for codec::Phase`: conversion to the existing
  configuration/play-only controller phase fails for the other three values.
  **The negotiation controller MUST apply that restriction itself.**
- `Flow::{Serverbound, Clientbound}` with the same ordinal conversion methods.
  `Option<Flow>::None` denotes bidirectional registration.
- `QueryComponent { id: Id, version: String, flow: Option<Flow>, optional: bool }`.
  The version is a REQUIRED string; an empty wire string is allowed. No optional
  version-presence byte exists here. `ModdedNetworkComponent` is a different type.
- `ModdedQuery { phases: BTreeMap<ConnectionProtocol, BTreeMap<Id, QueryComponent>> }`.
- `NetworkChannel { id: Id, chosen_version: String }`.
- `ModdedSetup { phases: BTreeMap<ConnectionProtocol, BTreeMap<Id, NetworkChannel>> }`.
- Both payload types implement `Default`, `Clone`, `Debug`, `PartialEq`, `Eq`,
  `decode(&[u8]) -> Result<Self>` and `encode(&self) -> Result<Vec<u8>>`.
- `QUERY_CHANNEL`, `SETUP_CHANNEL`, `FAILURE_CHANNEL` are string constants.
- `reject_setup_failure(&[u8]) -> Result<()>` **always returns Err**, including
  for a valid empty reason map. Treat every invocation as terminal. This is NOT
  a failure-reason decoder or full wire-validation oracle. Do not acknowledge it.
- Bounds: `MAX_PROTOCOLS=5`, `MAX_TOTAL_COMPONENTS=1024`,
  `MAX_FAILURE_REASONS=1024`; existing `MAX_BODY=32767`, `MAX_ID=256`,
  `MAX_STRING=1024` are reused.

The indexed query representation intentionally disallows same-phase duplicate
IDs, even when differing component fields would make two records unequal in the
upstream Java HashSet. Both writers require each outer ID to match its record ID.
An ID may occur in both configuration and play; that is not a duplicate.
A decoded setup does not prove agreement with any offer, required/optional/flow
policy, handler availability, registry state, or legal task ordering.

## Audited wire layouts

All inputs/outputs are **custom-payload bodies only**. V=signed VarInt,
R=ResourceLocation string, S=UTF-8 string with VarInt byte count, B=one boolean.
Map/set sizes are V. Ordering is not semantically significant; Rust writers
use BTreeMap order (protocol ordinal, then normalized ResourceLocation).

`neoforge:register`:

```
phase count V
  phase ordinal V
  component count V
    ID R
    version S                  # required, NOT optional
    flow present B
      if present: PacketFlow ordinal V
    optional B
```

`neoforge:network` (NetworkPayloadSetup):

```
phase count V
  phase ordinal V
  channel count V
    outer channel ID R
    NetworkChannel.id R        # another ID, not elided
    NetworkChannel.chosenVersion S
```

Empty payload maps are `00`, not zero bytes. Empty inner maps are preserved.
ConnectionProtocol uses `ByteBufCodecs.idMapper` over `values()[index]` and
`ordinal`. PacketFlow uses `NeoForgeStreamCodecs.enumCodec` and
FriendlyByteBuf enum methods. Neither uses a textual enum name on the wire.

`neoforge:modded_network_setup_failed` exact upstream *composition*:

```
reason count V
  channel/mod ID R
  ComponentSerialization.TRUSTED_CONTEXT_FREE_STREAM_CODEC Component
```

**Explicit remaining gap:** the Component is a
context-free trusted codec via NBT, not JSON text and not an S field. This milestone does NOT implement the full Component/NBT schema, styles, translations, hover
values, variants or semantic validation. No opaque byte blob is passed off as a
validated Component. There is no failure encoder or reasons-returning decoder.
The terminal rejection helper bounds the body/count, checks the first ID on a
nonempty map, and then returns UnsupportedTask without consuming the Component.
Malformed component bytes, later duplicate IDs and trailing bytes in nonempty
maps are *not classified separately*: all fail terminally as unsupported. For
zero reasons it checks trailing bytes before returning UnsupportedTask. No input
can produce success. This is honest fail-closed refusal, **not completion of the
original full setup-failure codec milestone**. It cannot display server-provided
reasons. A real Component codec and differential tests remain required.

## Strict local safety policy

- Counts must be nonnegative; invalid/negative enum ordinals, VarInt overflow,
  invalid strings/IDs, truncation and trailing bytes fail query/setup decoding.
- No preallocation based on peer counts. At most five outer maps, with one total
  budget of 1024 records summed across phases, enforced before reading a phase's
  records. Same bounds apply to writers. Body bound limits aggregate string work
  and memory. Limits can reject real packs and are not upstream maxima.
- Duplicate phases and IDs fail instead of overwriting/deduplicating. The ID
  comparison uses existing shorthand normalization. Query wire sets are indexed
  by component ID locally, intentionally stronger than Java record equality.
- Setup outer/inner ID mismatch fails although upstream can represent it.
- Boolean bytes must be exactly 00 or 01. Netty's broader nonzero-is-true inputs
  are intentionally rejected, not claimed equivalent.
- Existing Reader/Writer string/ID policy is retained: byte limits (not Java
  UTF-16-unit limits), UTF-8 and ASCII ID grammar, minimal writer VarInts, accepted
  non-overflowing short-overlong reader VarInts. Empty version strings are valid
  at codec level; registration/version agreement is controller policy.

## Source and ordinal evidence

`modded-codecs.json` records exact additional source hashes and official Minecraft
artifact metadata. The existing NeoForge artifact SHA-256 was rechecked against
its provenance and publisher checksum, and all 18 previously listed extracted
sources were checked against the verified archive. The seven codec-related files
listed in the new manifest, including NeoForgeStreamCodecs, were additionally
checked directly against that same archive. No upstream source is vendored.

Official Minecraft 1.21.1 version metadata was obtained via Mojang's version
manifest. Its server bundle and server mappings were downloaded from their
metadata URLs and checked against publisher SHA-1; SHA-256 hashes were recorded.
The inner server jar supplied obfuscated classes `vu` and `zh`; official mappings
identify them as ConnectionProtocol and PacketFlow. Python parsed each classfile
constant pool and asserted **both** its `<clinit>` enum constructor ordinal
arguments and synthetic values-array order. Thus the ordinals are grounded in
the exact 1.21.1 class bytes, not a guess from adjacent versions:

| ConnectionProtocol | ordinal | PacketFlow | ordinal |
|---|---:|---|---:|
| HANDSHAKING | 0 | SERVERBOUND | 0 |
| PLAY | 1 | CLIENTBOUND | 1 |
| STATUS | 2 | | |
| LOGIN | 3 | | |
| CONFIGURATION | 4 | | |

The public version-specific mapping summaries at
https://mappings.dev/1.21.1/net/minecraft/network/ConnectionProtocol.html and
https://mappings.dev/1.21.1/net/minecraft/network/protocol/PacketFlow.html were
also inspected, but exact official classfile assertions are the stronger evidence.

Reproduce the bytecode assertions using
`python3 tests/reference/audit_minecraft.py <outside-repository-scratch-directory>`.
It downloads official metadata/server/mappings (roughly 60 MB); do not place game
jars in the repository. This parser is narrowly scoped audit code, not a general
JVM verifier or an implementation of the network codecs. It was actually run and
passed. The server jar/classfiles and official mappings are not redistributed.

## Validation actually executed

Rust 1.85.1, x86_64-unknown-linux-musl with its bundled linker:

- New `tests/modded.rs`: **14 passed debug, 14 passed release**.
- Existing tests core (30) and CLI (3): all **33 passed release** in the same
  scratch integration copy. Other new modules were not included in that isolated run.
- Strict Clippy for the library + new modded test target: passed, `-D warnings`.
- rustfmt check for the two owned Rust files: passed.
- Enum classfile initializer/values-array audit: passed against verified 1.21.1.

Coverage includes literal independently specified **synthetic** query/setup
vectors, every prefix truncation of populated examples, all protocol/flow
ordinals, all three flow-presence cases, empty maps/versions, duplicate phases
and IDs, normalized duplicates, key/record mismatches, negative/invalid counts,
noncanonical booleans, invalid UTF-8/IDs, VarInt overflow, total-count/string/body
budgets on both writer and reader, trailing bytes, and a 4096-input deterministic
malformed corpus. This is a smoke corpus, not coverage-guided fuzzing or a proof.

**No Java was executed.** No JVM/JDK was available. An attempted official OpenJDK
download hit the workspace disk budget and its incomplete archive was removed.
Classfile parsing is NOT genuine Java reference execution. No independent Java
query/setup codec differential tests were performed; they remain a gap, along
with complete failure decoding. There was no socket transcript, server join,
Horizon compile or registry/content application. These distinctions must be preserved in overall validation and support statements.
