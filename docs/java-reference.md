# Executed Java/Rust codec reference — query/setup

Baseline: **Minecraft 1.21.1 / protocol 767 / NeoForge 21.1.248 only**.
Recorded run: 2026-11-27, Linux x86_64, Java 21, Rust 1.95.0, musl target.

This adds **genuine upstream-codec execution**, not another handwritten Java
serializer, not just a classfile/ordinal audit, and not a server transcript.
The exact NeoForge source codec initializers are extracted and mechanically
name-adapted, compiled, and executed against the actual official MC 1.21.1
classfiles and the metadata-pinned libraries. Rust independently executes the
library's public `ModdedQuery` and `ModdedSetup` encode/decode methods.

**Still cannot join.** No finish latch, handler, controller or game behavior is
changed. Failure Component/NBT execution is deliberately **not covered** by this
harness; that work is separate. No individual mod support is claimed.

## Files and ownership

- `tests/reference/prepare.py`: verified external inputs, source extraction,
  adaptation and Java compilation.
- `tests/reference/CodecReference.java`: test-only semantic model/line driver;
  delegates wire operations to the generated upstream codec classes.
- `tests/reference/differential.py`: bounded seeded corpus and assertions,
  two encoders × two decoders, known policy differences and recorded hashes.
- `tests/reference/results.json`: machine-readable actual release-run evidence.
- `tests/reference/validation.txt`: commands/results and environment record.
- `src/bin/codec-reference.rs`: autodiscovered dependency-free Rust test driver.

No Cargo manifest change was necessary. Existing reference ordinal-audit code
was left untouched. Shared status/provenance documents were not rewritten, so
historical statements elsewhere saying no Java execution describe the previous
milestone, not this new query/setup evidence. This source snapshot has no `.git`
metadata: no local/remote branch verification was possible, and no commit/push
was attempted.

## Exact input verification

Preparation downloads only into a caller-selected directory **outside the
repository**. It verifies:

- NeoForge `21.1.248` sources archive SHA-256
  `b4b912457549ff586873ce78b39b08af96f11a0e3a54c6b77576c6a25907083f`,
  also checked against the publisher's `.sha256` response.
- Each selected source hash in `provenance/modded-codecs.json`.
- Official 1.21.1 version metadata SHA-256
  `bb0a1413744530df54451e4f973bac81b1e27477999e79cfe7e66aaf6d4db6e6`.
- Official server bundle SHA-1 `59353fb40c36d304f2035d51e7d6e6baa98dc05c`
  and official mappings SHA-1 `03f8985492bda0afc0898465341eb0acef35f570`.
- Extracted server runtime SHA-256
  `c301de10f575027d13eac18c7f34409d60648cf56a35d566aa1f530ff617840a`.
- Library artifact SHA-1 values from the pinned official metadata. Only the
  headless codec dependency subset is downloaded; no native libraries/assets.

Official artifacts originate at `piston-meta.mojang.com`,
`piston-data.mojang.com`, and library URLs in that verified metadata. NeoForge
sources originate at
`https://maven.neoforged.net/releases/net/neoforged/neoforge/21.1.248/neoforge-21.1.248-sources.jar`.

The JVM download used in the recorded run was Azul Zulu Java 21.0.13.1+2:

```
https://cdn.azul.com/zulu/bin/zulu21.54.201-ca-jdk21.0.13.1-linux_x64.tar.gz
SHA256 510696f7da795e08b48432268173b060a49a6a3b05430690e776b5f3ddecac5e
```

The downloaded archive digest matched Azul's `sha256_hash` at
`https://api.azul.com/metadata/v1/zulu/packages/d96f18fa-c2c1-4e39-adfd-52e3d7a6195e`.
TLS/publisher checksums were used; an independent OpenPGP verification was not
performed. Any trusted Java **21 JDK** can be supplied for reproduction.
The preparation script checks javac 21, and the Java driver checks runtime 21.

## Source audit and precisely what was adapted

The following are retained directly from the verified upstream source, rather
than retyped wire-codec logic:

1. `ModdedNetworkQueryPayload`: record fields and full `STREAM_CODEC` initializer.
2. `ModdedNetworkQueryComponent`: record fields and full four-field composite.
3. `ModdedNetworkPayload`: record fields and full setup-wrapper composite.
4. `NetworkPayloadSetup`: record fields and full phase/nested-map composite.
5. `NetworkChannel`: record fields and full ID/chosen-version composite.
6. `NeoForgeStreamCodecs`: the **exact** `enumCodec` method, including its
   anonymous codec that calls the actual FriendlyByteBuf enum methods.

Unrelated helper methods (`fromRegistry`, registration constructor,
negotiation-result conversion, lookup helpers), annotations, imports, payload
interfaces and ID/type constants are omitted. They are not on these codec call
graphs. Original copyright/SPDX headers are preserved in external generated
files. No Java wire primitive, map, set, optional, boolean, enum, string or
ResourceLocation codec is replaced with test code.

Mechanical names, all tied to and checked against official 1.21.1 mappings:

| Named symbol | Exact official runtime symbol |
|---|---|
| ConnectionProtocol | `vu` |
| PacketFlow | `zh` |
| FriendlyByteBuf | `vw` |
| ByteBufCodecs | `yv` |
| StreamCodec | `yx` |
| ResourceLocation | `akr` |
| StreamCodec.composite | `yx.a` (matching overloads) |
| ByteBufCodecs map/collection/optional/idMapper | `yv.a` (matching overloads) |
| STRING_UTF8 / BOOL | `yv.l` / `yv.b` |
| ResourceLocation.STREAM_CODEC / parse | `akr.b` / `akr.a` |
| FriendlyByteBuf.readEnum / writeEnum | `vw.b` / `vw.a` |

Generated NeoForge records and test Java are placed in the default package
because Mojang's actual obfuscated classes are there. Java refuses signed and
unsigned classes in the same package, so preparation makes a **local-only
unsigned copy** of the official runtime, removing only signature entries from
META-INF. It does **not** remap, decompile, patch or recompile Minecraft classes.
The runner compares **every .class byte sequence** in that copy with the
SHA-256-pinned official runtime before executing tests. The result records the
number of unchanged classes and hashes of the six relevant MC classes.

All adaptations, exact original source digests and generated-source digests
are saved in external `source-adaptation.json` and copied into the results.
This is source-extracted NeoForge codec execution, **not** execution of an
unmodified complete NeoForge installation or proof that unrelated loader
bootstrap/registration code runs. Inspect the generated sources when auditing.

## Reproduce

Use Linux, Python 3.12+, a Java 21 JDK, and a Rust toolchain satisfying this
crate's manifest. Network access is required during preparation. Allow at
least 400 MiB external scratch space in addition to the JDK and Rust build
space. Never run these assertion-based tests with Python `-O`/`PYTHONOPTIMIZE`.
Do not put proprietary game jars, adapted upstream sources or build outputs
inside the checkout or redistribute them.

From the checkout root, with `JAVA_HOME` pointing to your Java 21 JDK:

```sh
SCRATCH="$(mktemp -d "${TMPDIR:-/tmp}/codec-reference.XXXXXX")"
export CARGO_TARGET_DIR="$SCRATCH/rust-target"
python3 tests/reference/prepare.py --scratch "$SCRATCH" --java-home "$JAVA_HOME"
cargo test --offline --locked --release --bin codec-reference
cargo build --offline --locked --release --bin codec-reference
python3 tests/reference/differential.py \
  --scratch "$SCRATCH" --java-home "$JAVA_HOME" \
  --rust-bin "$CARGO_TARGET_DIR/release/codec-reference" \
  --report "$SCRATCH/results.json"
```

Set `TMPDIR` to an external scratch directory with at least 400 MiB available
if the system temporary directory is too small; do not place generated jars,
adapted sources or build outputs inside the checkout.

For musl, add `--target x86_64-unknown-linux-musl` to both cargo commands, use
your configured musl linker, and point `--rust-bin` to
`$CARGO_TARGET_DIR/x86_64-unknown-linux-musl/release/codec-reference` instead.
The recorded run used rust-lld and Rust 1.95.0
`59807616e 2026-04-14`; ordinary GNU/native linking was not tested here.
The Java process uses `-Xmx128m` and each subprocess batch has a 90-second
hard timeout. A process failure or missing output fails the suite. Test input
lengths/counts are deliberately bounded; this is not a general hostile-input
JVM fuzzing service. The test-driver line interface is for trusted local test
control, not network exposure.

Optional JVM bootstrap: download the pinned Azul archive above outside the
checkout, verify the shown SHA-256 with `sha256sum`, then extract with your
normal archive tool and set JAVA_HOME. The recorded constrained workspace used
only bin/java, bin/javac, bin/javap, lib (excluding .jsa/.zip and ct.sym), conf,
legal and release entries; the full JDK is simpler when space permits.

## Assertions and recorded results

The release run passed:

- **256 valid models**, Java encode → Java/Rust decode and Rust encode →
  Java/Rust decode: **1,024 semantic comparisons**. Both independent encoders
  actually execute, not just one encoder with another language's self-roundtrip.
- Two independent literal byte anchors in both encoders (query and setup).
- **1,711** deduplicated shared invalid/truncation inputs rejected by both.
  Every proper byte prefix is tested for selected outputs from **both**
  encoders; selected vectors include populated/multiphase cases. The precise
  deduplicated count may vary with upstream identity-map/set iteration order.
- **9** upstream-accepts/Rust-rejects decode cases classified explicitly:
  noncanonical boolean values (optional and flow presence), duplicate phases,
  duplicate query records/setup keys, setup outer/inner ID mismatch and
  UTF-8-byte-over-limit versions.
- **6** Java-encodable/Rust-writer-refused boundary models: per-string, aggregate
  entry and aggregate body limits for each payload kind.
- **256** bounded seeded malformed smoke inputs: every Rust acceptance must
  agree with Java. This particular smoke run had **zero** acceptances; its
  significance is refusal/no process failure, not valid-input coverage.
- Three Rust driver-envelope unit tests pass in release.

Valid models exercise all five phase ordinals, both PacketFlow ordinals and
absent flow, both optionality values, empty outer/inner maps, repeated IDs in
*different* phases, Unicode (including supplementary characters), control
characters, empty versions, 127/128-byte string boundaries, 1,024-byte versions,
256-byte identifiers, 1,000 entries across five phases, the exact 1,024 aggregate
entry bound, and 31 long-version channels just below the body ceiling.

Map/set iteration order is not a protocol invariant. The Java identity maps,
hash maps and hash sets are **not rewritten to sort wire output**. Each decoder
returns a canonical semantic model (phase ordinal then ASCII ID), so equivalence
is checked semantically rather than demanding identical multi-entry byte order.
Empty phases remain represented. The Java driver emits a distinct mismatched-ID
model rather than hiding a setup outer/inner mismatch. Only full-body trailing
consumption is imposed by its framing wrapper, not attributed to StreamCodec.

The local text envelope is three tab-separated fields, action (`encode` or
`decode`), kind (`query` or `setup`), and data. Decode data is body hex. Encode
models are semicolon-separated `P:ordinal` then `Q:idhex:versionhex:flow:optional`
or `S:idhex:versionhex`; all text fields are UTF-8 hex, flow absence is -1.
Responses start `REF`, then `OK`/`ERR`, then result or error classification.
Neither harness path decides negotiation success or authorizes channels/play.

## Limits and remaining evidence gaps

- This proves bounded query/setup codec agreement, **not all possible strings,
  malformed encodings or collection behavior**. Known stricter Rust policy is
  retained, not relaxed to match permissive Java behavior.
- Failure payload/NBT Components, common payloads, registry snapshots and all
  other tasks are outside this differential suite. Existing tests elsewhere
  are not silently reclassified as Java reference tests.
- No complete loader runtime, packet framing/compression/encryption, socket
  transcript, live server, actual Horizon build, semantic registry application,
  world join or individual mod was tested in this workstream.
- No proprietary class/source/archive/binary is included in the repository.
  NeoForge's upstream LGPL-2.1-only source is fetched/adapted only in external
  scratch storage. Audit licensing before redistributing any adapted outputs.
- No nonexact NeoForge build, actual auto-updated production server or user's
  pack inventory is validated. Keep every existing fail-closed boundary.
