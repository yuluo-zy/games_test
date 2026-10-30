# Tiny Glade code map

Dedicated local **Neo4j Community 5.26.32** graph with identity-bound LLVM assembly
shards. SQLite is an extraction/checkpoint manifest, not a graph-backend fallback.
The original binaries, PDB exports, existing evidence and shared Ghidra/ReVa
databases are preserved. This tool does not move production Rust code.

For another computer, use the [portable setup and transfer guide](../../../../tools/code-map/README.md).
It includes the launcher, hashed dependency lock, complete local evidence bundles,
checkpoint rebuild and offline database dump/load. Cloning Git alone omits the data.

Execution results and acceptance limits:
[`2026-10-28 report`](../../evidence/code-map-2026-10-28/REPORT.md).
Latest [query optimization results](../../evidence/code-map-2026-10-28/OPTIMIZATION.md)
supersede the original hot-query timings. End-to-end recovery efficiency and
whole-software semantic review remain unverified.

## Current scope

- Raw extraction remains complete. The business graph applies the global
  `graph_filter.py` projection: mechanical/serde/trait helpers and unresolved
  new/get/insert/remove name associations are omitted across all modules/versions.
  Mixed physical aliases retain the real business symbol. Raw byte evidence and
  the catalog remain available outside Neo4j. Existing stores can be migrated by
  `prune-mechanical --apply`; deletion publishes a new snapshot and invalidates
  the old snapshots instead of silently changing a published identity.

- B: union of retained PDB procedures, identity-bound Ghidra functions and PE
  unwind coverage ranges. Unwind entries are not automatically functions.
- A/C: retained metadata and existing roof assembly only; no new full extraction.
- Loose `.asm`/disassembly files under the retained evidence and report roots are
  captured by `archive-existing`. They remain `version=unbound` until independently
  bound to an original binary; filename hints are never promoted to proof.
- Direct machine calls, indirect unresolved sites, tail candidates, aliases,
  PDB inline instances, source modules and types remain distinct.
- `chain.json` contains a **curated** roof/collision/global-ray semantic chain.
  Importing it proves annotation preservation, not automatic discovery or a new
  behavioral execution. Rust source hashes are current bindings, not test receipts.
- `module-reviews` produces the remaining module audit queue. It does not mark
  a source-mapped module as semantically recovered.

## Local layout and requirements

Runtime files default to `<workspace>/tools/code-map-runtime/`,
already ignored by the workspace. `inventory.json` binds the inputs;
`catalog.sqlite3` stores intervals, aliases and checkpoints; `assembly/*.zip`
contains independently hashed records. Neo4j data, transaction logs and local
credentials live in the same runtime directory. Credentials are mode 0600.

Python 3.11.8+, LLVM (X86 shared library and objdump), and Java 21 are required.
Host paths are configurable in `tools/code-map.local.json` or `CODEMAP_*`
environment variables; global Java settings are unchanged. The PE/PDB collector
is vendored with its original hash, so no personal skill directory is required.
Only `serve` needs `mcp==1.30.0`; the existing MCP project interpreter already has
it. Core commands use the Python standard library.

Initial inventory requires **50 GiB free**. Subsequent batches retain at least
5 GiB free. Capacity estimates are sampled; actual use is reported separately.
Do not delete evidence or shared build caches to satisfy this prerequisite.

## Reproduce / resume

Run from `/Users/yuluo/Desktop/game`. Use the same input identities and collector
revision. Mutation phases run sequentially and CLI writers hold a local lock.

```sh
CODEMAP="$PWD/reverse-engineering/tiny-glade/tools/code-map"
python3 "$CODEMAP/bootstrap.py"
python3 "$CODEMAP/code_map.py" runtime start

# Only when no verified Ghidra export is present; choose your own unique worker.
python3 "$CODEMAP/export_job.py" --worker codemap-YOUR-UNIQUE-NAME
# Verify success/timedOut/stdoutTruncated and export SHA, then normally stop
# only your worker using the existing AI_REVERSE_TOOLS.md lifecycle commands.

# Inventory is exclusive-create. Existing inventory is reused, not rebuilt silently.
python3 "$CODEMAP/code_map.py" inventory
python3 "$CODEMAP/code_map.py" capacity
python3 "$CODEMAP/code_map.py" enrich
python3 "$CODEMAP/code_map.py" associated
python3 "$CODEMAP/code_map.py" archive-existing
python3 "$CODEMAP/code_map.py" dispatch-boundaries
python3 "$CODEMAP/code_map.py" extract
python3 "$CODEMAP/code_map.py" import
python3 "$CODEMAP/code_map.py" optimize-search
python3 "$CODEMAP/code_map.py" verify --full
python3 "$CODEMAP/code_map.py" benchmark --repetitions 30
python3 "$CODEMAP/code_map.py" export-map
python3 "$CODEMAP/code_map.py" module-reviews
python3 "$CODEMAP/code_map.py" byte-coverage
python3 "$CODEMAP/code_map.py" cfg --key function:B:8cdc60
python3 "$CODEMAP/code_map.py" receipt-audit
python3 "$CODEMAP/code_map.py" freeze-evidence
```

`extract` resumes missing ranges; blob corruption is an error, never a silent
cache miss. An interrupted ZIP shard is checked by member contents before reuse.
`import` MERGEs deterministic IDs into an unpublished snapshot and publishes only
after all links resolve. Interrupted snapshots remain unavailable to public tools.
Repeated unchanged import returns without rebuilding. Preserve old snapshots and
evidence when deliberately migrating identities or schemas.

Extraction is incremental by range. Changed graph inputs currently materialize a
new full snapshot; unchanged import checks the normalized data fingerprint and
skips materialization. Static-node sharing and finer graph update granularity are
not implemented. Old materialized snapshots increase disk and index costs.

On hosts where detached subprocesses are reclaimed, run the server in a dedicated
terminal with the explicit Java 21 environment and `bin/neo4j console`. Stop it
normally with Ctrl-C. The setup does not install a system-wide auto-start service.
Local socket access may require the execution environment's reviewed permission.

Validated on 2026-10-29: `runtime start` returned success, but the execution host
reclaimed the detached process and subsequent queries returned connection refused.
Keep this command running in a managed terminal instead, then verify an actual
`query search` before notifying workers that the graph is available:

```sh
JAVA_HOME=/Users/yuluo/Library/Java/JavaVirtualMachines/corretto-21.0.5/Contents/Home \
NEO4J_HOME="$PWD/tools/code-map-runtime/neo4j-community-5.26.32" \
tools/code-map-runtime/neo4j-community-5.26.32/bin/neo4j console
```

An existing healthy console must be reused. Connection refusal alone does not
justify reconfiguring credentials, importing a replacement snapshot or rebuilding
the database. Keep the published snapshot identity in the recovery receipt; local
runtime files remain ignored. If the managed terminal closes, check status and
restore this same instance before continuing graph-dependent analysis.

Neo4j Browser: <http://127.0.0.1:17474/>; Bolt:
`bolt://127.0.0.1:17687`. Authentication is enabled. Read credentials locally from
`connection.json`; do not paste them into logs or reports.

## Queries and code maps

```sh
python3 "$CODEMAP/code_map.py" query search --args '{"text":"0x1408cdc60"}'
python3 "$CODEMAP/code_map.py" query state_users --args '{"key":"state:B:WorldRoofTable"}'
python3 "$CODEMAP/code_map.py" query paths --args '{"key":"function:B:21a3220","target":"function:B:8cdc60","depth":4,"kinds":["PRODUCES","CONSUMES","WRITES","READS"]}'
python3 "$CODEMAP/code_map.py" query assembly --args '{"key":"function:B:8cc870","limit":100}'
python3 "$CODEMAP/code_map.py" query evidence_pack --args '{"key":"function:B:8cdc60"}' --output /private/tmp/ray-evidence.json
python3 "$CODEMAP/code_map.py" query search --args '{"version":"unbound","text":"m01-global-ray"}'
```

Public tools: `search`, `neighbors`, `paths`, `state_users`, `assembly`,
`evidence_pack`, `module_map`, `coverage`, `unresolved`. Default depth 2, maximum
depth 6, maximum 200 results and a 10-second database transaction timeout. Results
carry a snapshot, evidence and explicit truncation/pagination. Assembly views
deduplicate identical decoded bytes across ranges while retaining provenance.
Evidence packs provide bounded file excerpts plus a follow-up assembly query.
They follow outgoing dependencies by default; use `direction: "both"` or `"in"`
for caller analysis. This prevents a high-fan-in function's callers from hiding
its immediate dependencies on the first page.
Complete lists/maps are generated by management commands rather than unbounded
MCP results. `module-map.json` and `.graphml` contain source-attributed dependency
maps; multiple source attributions do not establish exclusive business ownership.
`cfg` exports a candidate basic-block graph for a selected PDB range. Indirect
transfers, missing targets, EH and noreturn behavior remain unresolved; a decoded
range is never reported as a proven complete CFG.

Browser example (replace the snapshot using `Publication`):

```cypher
MATCH (pub:Publication {id:'current'})
MATCH p=(a:Entity {snapshot:pub.snapshot,key:'function:B:8cdc60'})-[r:LINK]-(b)
WHERE r.kind IN ['READS','HAS_CONTRACT','CALLS']
RETURN p LIMIT 100
```

## Agent / MCP entry

```sh
/Users/yuluo/Desktop/game/tools/mcp-servers/ghidra-mcp/.venv/bin/python \
  /Users/yuluo/Desktop/game/reverse-engineering/tiny-glade/tools/code-map/code_map.py serve
```

This is a stdio MCP server. It exposes only fixed read operations, with no caller
Cypher and no shell execution. Registration in an agent's global configuration is
left to its owner; this task does not rewrite global MCP settings.

## Verification and interpretation

```sh
python3 -m unittest discover -s "$CODEMAP" -p test_code_map.py -v
```

The independent chain check uses fixed original PDB ranges and a separate
`llvm-objdump` process. Full verification rechecks all extracted B instruction
bytes and reported gaps; direct-call targets are checked against original rel32
encodings. Fault tests cover identity, corrupted blobs, incomplete instructions,
query bounds and parameterization. Persistence and MCP integration checks are
recorded separately in the execution report.

The paired benchmark freezes 12 navigation questions and runs 30 hot repetitions.
Its baseline retrieves original report anchors with `rg`. It reports first-query
and hot p50/p95, output bytes and failures. It **does not** substitute these
measurements for end-to-end agent reasoning time, actual model tokens or manual
steps. All plan efficiency criteria remain unproven until a complete paired
recovery task demonstrates them. Whole-software semantic completion requires
reviewed producer/state/consumer/lifecycle contracts for every in-scope module.
`receipt-audit` compares old receipt file hashes to the current workspace;
`freeze-evidence` preserves the reviewed chain's exact report/source bytes in a
hash-addressed local proof store. Historical game passes are not current passes.

## Query acceleration

Run `optimize-search` after publishing a new snapshot. It builds a derived,
snapshot-specific substring index; until complete, searches use the original
full-text path. Short names are indexed directly. Long Rust names are preserved
and indexed in overlapping 8,000-character chunks, covering every accepted search
string of at most 512 characters without truncating the original name.

Local HTTP connections are reused per thread and expire after five idle seconds.
Configuration changes invalidate the cached connection settings. A failed
transaction is never automatically replayed. Snapshot completion is checked on
every request, in the same HTTP transaction as a guarded read; evidence hashes
are still recomputed for every public query.

Regression commands:

```sh
python3 "$CODEMAP/verify_search.py"
python3 "$CODEMAP/profile_queries.py" current --repetitions 30
```
