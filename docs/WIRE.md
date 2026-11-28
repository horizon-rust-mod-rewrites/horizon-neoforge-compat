# Continuation evidence (2026-11-27)

Continues existing draft PR #1 from verified remote head
`3331751dc938ba2fdd430dd28ead49d343f612b2`, without rewriting individual mods.
**Still cannot complete configuration or join.**

- Loader now retains bounded, lossless setup-failure **NBT data**, never a
  successful setup or safe-to-render Component. See `provenance/setup-failure.md`.
  Component semantics/rendering remain incomplete; every failure is terminal.
- Genuine source-extracted NeoForge query/setup codecs executed against actual
  official MC 1.21.1 classfiles: 256 models, 1,024 cross-language semantic checks,
  1,711 shared invalid/truncation cases, 9 classified policy differences and 6
  writer-limit cases. See `docs/java-reference.md` and `tests/reference/results.json`.
- Bounded split reassembly primitive, 18 tests; not yet advertised or wired to
  Horizon packet decoding. See `docs/SPLIT.md`.
- Horizon integration boundary hardening, executed endpoint/bridge tests and a
  reproducible full-build driver; full-engine build/runtime remains unexecuted.
  See `integration/README.md` and `docs/horizon-continuation.md`.
- Main crate **128 tests pass debug and release** with Rust 1.95.0 / x86_64 musl.
  Formatting, warning-denied docs, six fixtures and source verification pass.
  Strict Clippy also passes in this run.
  Current run: `docs/validation-continuation.txt`. Full engine, real content publication, remaining
  task execution, split host integration and live joins remain outstanding.

Earlier status below is historical where it conflicts with this continuation.
No finish latch was removed and no full-handshake completion is claimed.

---
# Current modded codec additions

The historical subset below is extended by `src/modded.rs` and
`../provenance/modded-codecs.md` (exact field layouts and evidence). Query
component versions are **required strings**, not optional values. Exact MC
1.21.1 enum ordinals are audited. Query/setup are implemented; NBT Component
setup-failure reasons are not and terminate the epoch. `registry_sync` adds
bounded start/completed codecs and staging, but no Horizon semantic host.
The old common-only Session remains conservative; use Loader for modded
query/setup. Neither endpoint can finish configuration.

---
# Wire contract: verified subset, not a full loader handshake

All functions consume or produce **only the custom-payload body**. The Minecraft
packet ID, channel ResourceLocation prefix, packet length/compression/encryption,
and configuration/play packet-state transitions belong to the host transport.
Do not prepend a second channel or send an empty body where a count is required.

## Evidence

The official artifact is
https://maven.neoforged.net/releases/net/neoforged/neoforge/21.1.248/neoforge-21.1.248-sources.jar
and its publisher `.sha256` was compared to the downloaded bytes. The manifest
records hashes of all 18 inspected source files. No Java reference codec was run.
These source declarations establish field order; fixtures are manually specified
small examples, NOT captured traffic or differential Java-generated goldens.

Paths below are relative to `net/neoforged/neoforge/` inside that artifact.

## Implemented layouts

`V` = Minecraft signed i32 VarInt (unsigned continuation representation, up to
5 bytes), `S` = VarInt UTF-8 byte length then bytes, `R` = ResourceLocation via S.
Collection/map counts are V. Counts must be nonnegative under local policy.

| Payload | Field sequence | Audited definition |
|---|---|---|
| `c:version` | count V, count × version V | `network/payload/CommonVersionPayload.java` |
| `c:register` | version V, protocol S (`play` or `configuration`), count V, count × R | `network/payload/CommonRegisterPayload.java` |
| `minecraft:register`, `minecraft:unregister` | ASCII IDs separated by zero bytes; encoder terminates every ID; reader also accepts unterminated final ID | `network/payload/DinnerboneProtocolUtils.java`, MinecraftRegister/UnregisterPayload |
| `neoforge:frozen_registry` (offline codec only) | registry R, ID-map count V, pairs (numeric ID V, entry R), alias-map count V, pairs (alias R, target R) | `network/payload/FrozenRegistryPayload.java`, `registries/RegistrySnapshot.java` |

The registry alias map follows the ID map; it is not NBT, not a vanilla
RegistryData packet, and not an array implicitly indexed by arrival order.
IDs may be sparse. RegistryMap maps entries by supplied canonical names, does
not invent IDs for aliases, and does not implement NeoForge registry mutation.

NeoForge NetworkRegistry supports common version list `[1]`. Its common version
handler replies with that list in configuration. Its common register handler
replies with version 1, protocol `play`, and optional play receivers. It replaces
the peer common-channel set. Dinnerbone registration instead unions with the
existing channel set; unregister removes named channels. The submachine implements
those separate operations, but not a complete NeoForge connection lifecycle.

## Deliberately stricter LOCAL policy

- Total body: **32,767 bytes**, even clientbound. This is a conservative milestone
  limit, not a claim about upstream registry or clientbound maximums.
- Max versions 32; registration entries 1,024; ID strings 256 bytes (including
  normalized namespace); protocol string 1,024 bytes; registry IDs 4,096; aliases
  1,024. Bounds can reject real packs, especially large registries; raise them
  only with an aggregate memory budget and packet-split handling.
- UTF-8 validation and ASCII ResourceLocation grammar; shorthand normalizes to
  `minecraft:`. Empty paths are rejected. Non-ASCII identifiers are rejected.
  String limits are byte limits, not Java UTF-16-unit limits.
- Fifth VarInt byte overflow is rejected; noncanonical shorter-overlong VarInts
  that still fit i32 may decode, but writers always emit minimal encoding.
- Extra trailing bytes fail; negative counts fail before allocation.
- CommonRegister duplicate IDs fail, while upstream HashSet deduplicates them.
- Dinnerbone empty segments ignored and duplicates deduplicated as upstream does;
  invalid segments fail rather than upstream's log-and-ignore behavior. Max
  segment count includes duplicate segments to bound work.
- Snapshot duplicate IDs/names, negative IDs, alias/canonical-name collisions,
  dangling aliases, and cycles fail. This is additional semantic validation,
  not byte-for-byte reproduction of upstream map-overwrite behavior.
- The session accepts common register **v1/play only**, after c:version, once.
  Upstream currently ignores the register.version field and has less strict
  ordering. This is a deliberately restricted submachine, not an emulator of
  every legal common-protocol sequence.

## Explicit gaps rather than speculative layouts

`ModdedNetworkQueryPayload` (`neoforge:register`) is a phase-keyed map of
query-component sets, not the Dinnerbone string list or c:register. Components
carry ID, version string, optional flow, optionality. Its empty query map is
encoded with a count (`00`), **not an actually empty byte slice**. The upstream
class comment saying “no data” must not be used to invent an empty-body codec.

`ModdedNetworkPayload` (`neoforge:network`) uses NetworkPayloadSetup: phase-keyed
maps, outer channel key R, then NetworkChannel (another R plus chosen-version S).
Do not confuse ModdedNetworkComponent's optional version with NetworkChannel's
required string. Implementing that codec requires verifying the exact Minecraft
ConnectionProtocol and PacketFlow enum ordinals and primitive codec behavior for
this baseline, then adding independent reference tests. No speculative ordinal
constants or partial success for these messages are present in this milestone.

The server negotiates channels; this project does not invent an accepted set.
No full NeoForge channels are advertised. The frozen-registry codec's existence
never means a snapshot was applied: Session rejects that task explicitly.
