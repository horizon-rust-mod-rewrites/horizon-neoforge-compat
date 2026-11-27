# Horizon → NeoForge compatibility: milestone 0.1

**Real Rust code, not a mod, agent, grid, or pack rewrite. NOT loadable or
drop-in. CANNOT JOIN a NeoForge server yet. No live-server validation.**

This is a deliberately narrow, dependency-free compatibility core for reuse by
future Horizon adapters. It implements verified common-networking payload codecs,
a fail-closed common-task submachine, optional handler-backed registrations,
and a frozen-registry snapshot codec plus explicit ID-map infrastructure.
It does **not** implement the full NeoForge handshake. In particular,
`neoforge:register` and `neoforge:network` are unsupported, not stub-successes.

## Target and evidence

- Minecraft **1.21.1**, protocol **767**.
- Source-audited loader baseline **NeoForge 21.1.248** only. “Source-audited” does
  **not** mean end-to-end compatible. No 21.1.x wildcard compatibility claim.
- Horizon inspected at `ae632edf48f6cae3ba52a5c81523ec712488c9cc`.
- Exact official Maven source artifact, publisher checksum, and individual
  inspected-source hashes: `provenance/neoforge-21.1.248.json`.
- Full source-artifact SHA-256:
  `b4b912457549ff586873ce78b39b08af96f11a0e3a54c6b77576c6a25907083f`.

## Implemented and tested

| Component | Actual behavior |
|---|---|
| `codec` | Bounded VarInt/UTF-8/ResourceLocation primitives; `c:version`, `c:register`, NUL-delimited `minecraft:register` / `unregister`; deterministic writers |
| `session` | Version-1 common protocol replies; common play-channel registration; Dinnerbone union/removal tracking; strict ordering and terminal failure |
| `extension` | Explicit optional receivers, phase and size limits, duplicate/reserved-namespace rejection; no channel declaration without a callback |
| `registry` | Exact audited frozen-registry field sequence; alias validation; explicit server↔local numeric maps built atomically from caller-supplied names |
| CLI | Offline fixture inspection and common-task replay, explicitly reports NOT joined |
| Tests | 33 integration tests (30 core + 3 CLI), 6 literal synthetic fixtures, truncation checks and 4,096 deterministic malformed inputs across 4 decoders |

`finish_configuration()` **always returns an unsupported error**. Registry
wire decoding is available offline but the session never advertises or
acknowledges frozen-registry tasks: applying a snapshot is not implemented.
Unknown tasks never become success. All session errors poison that epoch.

## Build and reproduce

Rust 1.85.1 was actually used; no Rust crates need downloading. On a normal Rust
installation with a native linker:

```sh
cargo test --offline --locked
cargo test --offline --locked --release
cargo clippy --offline --all-targets -- -D warnings
cargo fmt --check
cargo run --offline --bin compat-inspect -- common-demo
cargo run --offline --bin compat-inspect -- frozen fixtures/frozen-items.bin
python3 fixtures/generate.py --check
```

Actual validation used the `x86_64-unknown-linux-musl` target and Rust's bundled
linker, because no system C compiler was available. Both debug and release test
runs passed. Native GNU-target execution was **not** validated. See
`docs/VALIDATION.md` for commands, limits, and logs. Python is needed only for
fixture/source verification, not the Rust library or CLI. Optional network check:

```sh
python3 provenance/verify_upstream.py
```

## Not implemented / no implication of support

- Full NeoForge modded channel/version/flow negotiation or setup failure decoding.
- Registry task lifecycle, committing maps to Horizon, block-state property
  mapping, chunks/palettes, item/component codecs, entity serializers, recipes,
  menus, attachments, or rendering semantics for modded content.
- Config-file synchronization, data-map negotiation, extensible enums, feature
  flags, split-packet reconstruction, connection-aware codec changes.
- TCP/login/authentication/compression/encryption or a plugin/mod loader bridge.
- Create, Ponder, Sable, any of the ~70 rewrites, or any assets.
- A public Horizon pre-negotiation hook. Existing hooks were inspected; a
  precise engine-integration design is supplied **instead of an untestable patch**.

## Read next

1. `docs/WIRE.md` — exact supported field layouts and stricter local policy.
2. `docs/HORIZON_INTEGRATION.md` — inspected seams, adapter directionality,
   blockers, intended patch locations and required acceptance tests.
3. `docs/HANDOFF.md` — next milestone order, jar/version inputs still needed,
   Create/Sable adapter boundaries and license constraints.
4. `docs/VALIDATION.md` — what actually ran vs what remains unverified.

No Horizon source checkout was modified. This milestone is being published to
the private compatibility repository with development handoff documentation.
No fake mod manifest is included. See `THIRD_PARTY_NOTICES.md` and `LICENSE`.
