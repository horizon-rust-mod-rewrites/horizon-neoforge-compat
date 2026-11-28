# Feature-flag equality task — exact baseline only

Targets MC 1.21.1 / protocol 767 / NeoForge 21.1.248.
`provenance/feature-flags.json` pins exact source-file hashes in the verified
NeoForge source artifact. No upstream implementation is redistributed.

Source audit: FeatureFlagDataPayload uses a ResourceLocation set collection
codec: VarInt count, then that many identifiers. FeatureFlagAcknowledgePayload
uses a unit codec (zero bytes, not `00`). NetworkInitialization registers both
configuration channels as optional, version `1`, data clientbound and ack
serverbound. CheckFeatureFlags compares the COMPLETE modded flag sets for exact
equality before replying. It does not ask the client to adopt server flag names.

`feature_flags` supplies bounded codecs and a terminal one-shot equality task.
The host supplies an immutable, complete inventory of actually implemented modded
flags. An empty set is an explicit host assertion, never an implicit default.
This does not allocate flag IDs, implement flag-controlled content, substitute
for vanilla enabled-feature handling or validate a host's honesty. Equality is
only this task's condition, never permission to finish configuration/play.

Local stricter policy: 1024 flags, existing 256-byte identifier/32767-byte body
ceilings, duplicate normalized IDs fail instead of Java HashSet deduplication,
trailing bytes fail and repeated task data poisons the epoch. Constructor checks
aggregate encoding limits before an offer can be made. Mismatch/malformed data
produces no acknowledgment. Host must disconnect on failed acknowledgment send,
invalidate on reconfigure/disconnect and construct a fresh task next epoch.

## Loader integration checkpoint

`Loader::new_with_feature_flags` explicitly opts into real data/ack registration
(version 1, optional, configuration, clientbound/serverbound respectively).
Caller registrations cannot override the same configuration channel. The default
constructor and Horizon companion remain unchanged; neither invents an inventory.

The server must select both task directions or neither. Half-selected setups
fail atomically. Both absent remains an optional disabled task, never dispatchable.
Dinnerbone announcements never grant authority. Incoming task data must pass
negotiated direction/body validation, then the stateful inventory comparison;
outgoing ack must pass negotiated serverbound authorization and exact unit codec
validation before being returned. Only selected incoming data is announced as a
listener. Errors revoke all grants and task-local checked status. Repeated data
is terminal. Close discards the inventory; reconnect needs a new endpoint.
`feature_flags_checked()` never grants configuration completion and returns false
once the loader fails/closes. Actual transport must disconnect on send failure.

12 codec/task tests + 13 Loader integration tests passed debug and release;
complete suite now 153 main/driver tests in each profile. Strict Clippy, rustfmt,
warning-denied docs and 12 existing companion/endpoint contract tests pass.
Evidence: `docs/validation-feature-flags.txt`. Flags tests are synthetic and
source-audited, NOT genuine Java differential feature-flag tests or server captures.
No actual Horizon host inventory/semantics, full-engine compile or live join.
No individual mods or finish refusals changed. The client STILL CANNOT JOIN.
