# Bounded allocation-pressure reclamation

Proactive eviction, the one allocation retry and suballocation-to-dedicated fallback share one lazy completion snapshot and one ReclamationBudget (default 4,096 scans / 1,024 destructions). An exhausted allowance is not refreshed by another eviction call in the same allocation. Later maintenance or a separate allocation may continue. Zero reported cache bytes still triggers bounded physical-owner retirement. DeviceLost is a terminal completion state. Explicit shutdown/drain keeps the complete drain API.

Strict Vulkan verification: budget_eviction 2/2, graph_cache_eviction 3/3, and one new quota test: nine queued owners with a 3-scan / 2-destroy policy, four eviction calls in one session destroy exactly two; subsequent independent maintenance drains the rest, with zero idle waits. Logs: .perf/pressure-gpu-tests.log and .perf/pressure-quota-tests.log.

The subsequent reserved-node ownership migration is implemented as a source candidate; see retirement-reservations-20261026.md for its guarantees, tests and the remaining legacy raw-handle compatibility fallback. Window frame-time and sustained pressure acceptance remain unmeasured.
