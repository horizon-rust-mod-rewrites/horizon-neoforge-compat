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

Initial checkpoint: transport-independent primitive only. No implicit Loader
advertisement, Horizon host inventory, full-engine compilation, Java differential
feature-flag execution or live join is claimed. Synthetic unit tests check literal
wire bytes, exact set equality, budgets, truncation, ordering and terminal cleanup.
The existing two finish refusals remain unchanged. Follow NEXT_AGENT.md for the
latest continuation checkpoint and remaining work.
