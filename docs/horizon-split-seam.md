# Concrete split decoder integration milestone

Baseline remains MC 1.21.1 / protocol 767 / NeoForge 21.1.248 and Horizon
`ae632edf48f6cae3ba52a5c81523ec712488c9cc`. No commits/pushes, workflow changes,
full-driver changes, host inventory inventions, flag opt-ins, mod rewrites or
finish-gate changes belong to this milestone. The client still cannot join.

## Follow-up: terminal ingress bypass fixed

A focused source review found two real bypasses after a rejected reconstructed
original poisoned the reader. `read_from` still read from its caller-owned source,
allocated/refilled the cleared receive buffer and incremented bytes-in; guarding
only `recv`/`buffered` did not prevent those side effects. `read_raw` allocated
before finding the detached stream, and a zero-length raw read could even succeed.

The actual dependency's connection implementation now checks the terminal bit at
both public entry points, before source I/O or allocation. Live readers retain
their old paths; the shared packet-ID parser, allowlist, adapter, size policy and
phase rules are unchanged. The real packet-enum decoder already checks complete
body consumption; no second parser or speculative trailing-byte fix was added.
The inspected list codec checks counts against remaining bytes, and the custom
payload codec retains its existing directional limits.

Three additional dependency-local tests were added to the existing protocol
framing filter (not a model or facade):

- A detached reader refuses external reads without consuming a Cursor, changing
  counters or refilling its buffer; raw reads of zero and `usize::MAX` refuse
  before success/allocation, and normal/buffered reads remain terminal.
- A successful reconstructed known-packs original leaves already-prefetched socket
  bytes and counters unchanged; ordinary receive then delivers that packet once.
- Actual codecs reject trailing keepalive/ping/known-packs bytes, truncated pack
  strings/channel length, and impossible/negative counts without adapter phase
  changes or queued packets; attached peer/writer observe transport shutdown.

Expected selected suites are now **14 companion / 6 protocol framing** (the latter
is five connection tests plus the existing adapter TCP test). These are authored
test counts, NOT executed Rust results. The full driver now requires minimum floors **14 / 6**; these are
selection requirements rather than executed new Rust evidence. Early-configuration/network filters are
unchanged. Both finish gates remain; no split/flag opt-in or join support added.

Locally executed for this follow-up: all **35 Python tests**, **six fixtures**,
all nine available original-source hash checks, exact six-file patch application
and edited-byte comparison, duplicate refusal, reverse check/application and
original restoration, byte-identical regeneration, and whitespace checking.
Only four small audit source fetches were attempted (three succeeded, one 404),
then the packet macro module was fetched separately; the fetched custom-payload
codec matches its existing pin hash. No compiler/dependency/full-source downloads.
No Rust/rustfmt/full-engine execution is claimed; hosted compilation remains
required. Parent-owned handoff, checksums, workflows and full driver were not
edited; nothing was committed or published.

**Detached close limitation:** after `Connection::detach`, the actual socket is
owned externally. The seam can poison the reader but cannot shut that socket or
its independent writer down through `Stream::Detached`. The future caller must
retain/use a real closer (and reset Loader); no production caller is wired yet.
The new ingress guard prevents subsequent source consumption, not external socket
closure. Attached-stream rejection still shuts down both transport directions.

## Why the transport alternative

The host-inventory audit does not establish an implemented-feature inventory or
an enforced implementation-profile lifetime. A new wrapper around enabled names,
peer announcements, an empty list or the pack fingerprint would not fix that.
Feature-flag opt-in therefore stays disabled. This milestone instead implements
a concrete actual-dependency decoder boundary needed before safe split dispatch.

## Source and behavior

The patch modifies the pinned `horizon-protocol` connection implementation:

- Socket plain/inflated packet paths and reconstructed originals share the real
  Reader packet-ID extraction; framing/decompression are NOT applied twice.
- `ConnReader::decode_reassembled_configuration` accepts an owned entire original
  packet, checks the 8 MiB ceiling, exact adapter version and BOTH read/write
  configuration phases, maps the wire ID with the real table, then validates
  allowed bodies with actual configuration codecs before one adapter invocation.
- Only opaque custom payloads, keepalive, ping and known packs are admitted.
  Content packets (including registries and tags), unknown IDs, finish and nested
  `neoforge:split` payloads fail before adaptation. This specifically avoids
  presenting vanilla numeric translation as modded content support.
- Rejected input shuts down/detaches an attached stream, clears buffered socket bytes and
  terminally poisons ordinary recv, buffered reads and repeated seam calls. The
  terminal check precedes queued adapter delivery. Fresh connections are separate.
- Successful decoding returns one adapted packet without reading a socket packet,
  queuing a response, or granting channel/task authority. Caller still must await
  actual handling. Socket counters are not incremented twice for reconstructed
  originals whose fragments were already received.

This is a concrete decoder, NOT a negotiated split handler. There is intentionally
no production caller admitting split fragments yet. Actual default network reads
use its shared packet-ID parser, and full-engine tests connect the real core
reassembler through the concrete reader into actual loader handling. No new
abstract bridge facade or replacement/mock packet decoder was introduced.

## Narrow exact-pin inspection

Only individual public source files were retrieved, not a checkout/archive,
compiler or dependency graph. Their hashes matched the existing provenance:
client net.rs, protocol connection.rs, adapter.rs, adapter/session.rs, adapter/translate.rs,
adapter/translate/v1_21_1.rs, packets/configuration.rs, packets/custom_payload.rs,
and the generated adapter/v1_21_1.rs table. The connection source's existing
SHA-256 is `7fd746be12b4bfae94c831c1d6ad515810cfe0c62ccac1874976247813973b83`.
Original-source hashes remain original-source hashes; patch bytes are separately
tracked by SHA256SUMS. No existing Java differential work was repeated.

## Added tests (NOT locally Rust-executed)

Five full-engine-only tests live in the actual companion contract module, included
by the existing full driver. They use the real core reassembler, protocol reader,
Session, configuration codecs and Loader, not independent models:

1. Split query original reaches actual Loader; query still offers no split/flags,
   queued socket known-packs packet is untouched, and finish still refuses.
2. Wire known-packs ID 14 becomes pinned ID 15 once, and decodes as known packs.
3. Empty/truncated/negative/unknown/oversized/malformed/recursive/finish originals
   permanently invalidate reads; a fresh connection remains independent.
4. Absent/wrong adapters, preconfiguration, and either phase having left
   configuration refuse the seam.
5. Partial reassembly times out without an original escaping to the decoder.

Two dependency-local tests use the existing protocol framing test filter to
verify finish refusal before a phase change/ack, and refusal of a valid registry
packet before content translation. Independent patch testing remains possible:
full-engine-only companion tests are excluded explicitly by the isolated checker,
without fake Horizon declarations. Full companion filter should now execute
14 tests (9 existing + 5); protocol framing originally selected 3 (1 existing
TCP + 2 new), now 6 after the follow-up above. The full driver now requires
14 companion and 6 protocol passes; these are selection floors, NOT executed new
Rust test results. Existing early endpoint filter stays at 9.

## Remaining prerequisites

No timer/queue/negotiated split ingress or outbound splitter is connected. The
future caller must serialize fragment/socket/phase handling, bound overall
traffic and queue budgets, check deadlines on an idle timer, disconnect/reset
both transport and Loader on failures, and await real handling. Content-bearing
split originals deliberately remain unsupported pending atomic semantic host
integration. Both finish refusals remain. No live NeoForge server was run.

## Executed locally

- Starting local/remote branch HEAD matched `70ba5a4737d2fcb669738ee49dac5dcdb330939f`.
- Disk checked before retrieval (1 GiB volume); only nine individually fetched
  public source files were hash-verified against existing manifests. No full
  source/compiler/dependency download or server launch.
- All 35 existing dependency-free Python driver tests and six synthetic fixture
  checks passed. Python syntax compilation passed for the modified utilities.
- Six-file patch: clean application, exact edited bytes, duplicate-application
  refusal, reverse check and original byte restoration passed on the narrow
  original-source scratch tree. Regeneration is byte-reproducible.
- `git diff --check` passed. No Rust compiler/rustfmt was available; no new Rust,
  isolated companion, full-engine, TCP or server result is claimed.
- Only this milestone's changed/new file checksums were refreshed. The checkout
  already had nine stale SHA256SUMS entries for unchanged handoff/evidence/driver
  files (AGENTS, NEXT_AGENT, README, HORIZON_INTEGRATION, PROGRESS, VALIDATION,
  horizon-continuation, integration/README and check_full). Those were not silently
  normalized as part of this source change; parent owns their reconciliation.
