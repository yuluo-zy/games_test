# Wall sparse-layout trace evidence — 2026-10-27, v1

This supplements the [current status](status.json) and [remaining work](remaining-tasks-20261026.md).
Full source, binary, inventory and raw-file SHA256 identities are in the
[versioned summary](trace-source-wall-layout-20261027.json).

The `rich-workload-debug-c98be8e-trace-1` investigation completed on source
`c98be8eafd524c6a66215b91603cd7fd1f349be1`, using the same frozen debug EXE
`064462bf22ced894f19848bf7dffc81f726cde663e9fb8f5207847450141092a`.
Its requested integration-test target completed; the entire Cargo invocation
is **not** accepted. Static OFF/ON each ran 60 warmup frames and 30 observation
intervals (31 rows including baseline), with the rich bill and native trace,
without ETW. Both children executed exactly one passing test, zero ignored,
with no Vulkan validation diagnostics. All sealed artifact inventories were
independently rechecked against their actual files.

| Observed quantity | OFF | ON |
|---|---:|---:|
| Brick instances in every measured Wall source observation | 502 | 502 |
| Physical normal Wall ranges | 3 | 4 |
| Direct dispatch API calls in every measured actual-workload row | 355 | 357 |
| `Generate wall draw lists`: calls per recorded scope | 3 | 4 |
| `Generate wall draw lists OC/2`: calls per recorded scope | 3 | 4 |
| Recorded scopes for each of those names | 92 | 92 |
| Direct dispatch calls across the complete trace | 32,656 | 32,840 |

The full trace contains 184 Wall scopes per role. Every ON Wall scope records
four calls; every OFF Wall scope records three. These distributions explain all
184 additional direct dispatches: `2 passes × 92 scopes × (4 − 3) = 184`. Other ten native/copy global
totals match. Of 113 display-name profiles, only these two Wall names change;
the other 111, including every Nani profile, retain equal invocation counts
and native/copy distributions. Names and pass indices are diagnostic locators,
not cross-process semantic identities; all original scope identities remain
in the raw traces.

The ordered normal ranges are stable through all 31 measured observations:

- OFF: `(640,65), (736,47), (896,390)`.
- ON: `(448,303), (768,6), (1056,65), (1216,128)`.

Highlighted ranges are empty in both roles. The shared input-manifest SHA256 is
`1f7faae359e05c49b1ac65bfbbe9c321cc7fb6abb93648eeff4811fcc9f317fc`.
It does not cover identical physical Wall layout; the new source observation
exposes that gap. The source arrays and the real callbacks' per-scope native
counts localize this run's dispatch difference to Wall range fragmentation.
The specific upstream owner creation/allocation order remains unproven.

The actual-workload comparison also retains graph command/chunk differences
in 14 of 31 observations (14 fields of each kind). Total trace chunks are
705/718. These are recorded management-cost differences, retained for later
cost analysis; they are not removed to make an old comparison pass. All other
normalized actual-workload fields match.

Both original `run.json` files retain `valid=false`: no scheduling trace was
captured. The new series' `valid=true` means diagnostic integrity only. This
is not clean CPU/percentile evidence, a CPU gain, production 16ms acceptance,
equal GPU payload proof or pixel equivalence. Native traces include startup
and warmup and have no receipt association; the Wall source's receipt keys
cannot invent a join for the separate native trace. Equal brick counts do not
prove identical brick payloads. The earlier `5767a40` 357/355 mismatch remains
invalid and is not retrospectively attributed or accepted.

Next, prepare a new canonical common fixture with controlled real startup
owner/allocation order, then freeze both variants and verify exact raw ranges,
native workload, actual payloads and pixels. This is planned, not completed.
Preserve scene content, effects and brick work; do not renumber reported ranges
or credit startup-layout changes as binding-reuse gains.
