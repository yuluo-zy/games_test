# Actual workload and separate GPU command-span diagnostics

The existing shipped-session App, CoreRenderBridge and production renderer remain the only render host. The four scripted workloads, raw semantic manifests and transform trajectory are unchanged. Default production builds have no new probe path. The feature-gated test fixture stores bounded Copy observations and serializes after the measured CPU endpoint.

`actual_workload` is an additive `headless-actual-workload-v1` section. Its `samples+1` rows bind `world_tick`, successful graph `receipt` and independent `attempt` to the existing CPU observations. Every warmup frame updates cumulative baselines; row zero contains one frame's workload rather than the accumulated warmup. Mode-off and feature-off reports contain `null`, which is unavailable evidence and must never be interpreted as zero work.

Each row records eight native API-call categories, retained/culled pass counts and 21 draw-list lengths, graph recorded commands/chunks/batches, and independent joined Collect tasks/waves. Cross-pass Collect values are subsets. API calls are distinct from the GPU's number of draws, instances or primitives; indirect calls count once regardless of their argument/count buffers. Graph recording chunks are distinct from Collect jobs.

GPU copy scopes remain separate:

| Source | What is recorded |
| --- | --- |
| Graph staging H2D | Actual copy commands from successful complete graph receipts |
| Graph immutable producer H2D | Actual immutable producer copy commands from successful complete graph receipts |
| Graph cached D2D | Actual device-to-device copy bytes, independent of H2D |
| Queued buffer H2D | Successful TransferManager submit payload lengths, with process-local manager identity |
| Device texture setup/direct/transfer | Successful native producer submission payloads, separate monotonic sources |
| Device buffer setup | Successful buffer setup submission payloads |

Mapped CPU writes, D2H readback and clear/fill commands are excluded from GPU H2D copy bytes. Native-submitted payload is not GPU completion, requested bytes, dirty bytes, allocated capacity, aligned staging size or logical uniform bytes. Device source atomics are sampled independently at owner boundaries; the snapshot is not an atomic whole-device phase measurement. A real successful submission prefix remains counted if a later producer operation fails, but that failure invalidates the entire fixture run.

The same BufferHost Arc is retained from before warmup until report creation. Lazy `(manager=0,jobs=0,bytes=0)` may become one real nonzero manager. A nonzero manager replacement/reset, device-generation change, counter regression, overflow, failed Collect wave/commit, incomplete native-command workload, missing completion association, insufficient frames or bounded-storage exhaustion invalidates evidence. IDs/generations stay in raw provenance and are excluded from cross-process semantic identity.

The common baseline/candidate CPU harness payload is staged in `common_cpu_harness/manifest.json`. Its three Rust files must be byte identical across the pair. It rejects candidate-only management bills and contains no candidate optimization accessors. `algorithm_selection="compiled_source_defaults_no_runtime_ablation"` reports that the reviewed source defaults are selected; absent runtime-management booleans do not fabricate four successful controls. The candidate's richer diagnostic harness remains separate.

`TG_GPU_DIAGNOSTIC=1` is a separate experiment. It configures the existing timestamp ring once before workload warmup, with pass spans disabled. Ring pending capacity is 4096 and retained capacity is 4096; the CPU capture storage defaults to `min(total_frames*128,2000000)` and is independently bounded. These capacities are not guarantees of coverage: any skips, eviction, overflow or missing samples invalidate the run. No settings silently reduce actual command count.

Every raw `GpuCommandTiming` is retained, including multiple commands with the same `(queue,completion_value)`. Exact successful receipt points associate samples with warmup/measured frames. Each frame must have exactly `ExecutionStats.commands_recorded` command-span samples. The final ring must be fully drained with zero pending/skips/retained samples, and cumulative `completed` must equal all drained samples. The report keeps the raw pre-drain statistics as well as the exactly computed post-drain state. It records actual timestamp period and per-queue valid bits. Queue command-span sums/maxima are **not** whole-frame latency or GPU active time.

The existing ring schema has no unique native command ID. Cardinality and point/epoch checks detect missing/extra/unmatched samples; they cannot distinguish an equal-cardinality replacement of two commands sharing a completion point. No stronger command-identity claim is made.

`perf_gpu_diagnostic.py` is a new standalone controller. It does not call Cargo, build an executable, alter the six frozen CPU runtime-policy tools, start ETW, or accept clean CPU/WSI results. It verifies frozen EXE/PDB/FMOD/source inventory identities, binds its own SHA256, clears inherited TG/RHAPSODY settings before adding its explicit diagnostic configuration, and uses the existing ready/go handshake and exact ignored test entry. Exactly one executed passing test, zero ignored tests and no Vulkan validation diagnostic are required. Report/log/handshake SHA sidecars are written after success or failure. The session and all raw timing/receipt associations are revalidated before `run.json.valid` can become true.

Frozen builds are accepted through either a completed whole-Cargo build or the existing reviewed target-only protocol. The target-only route requires a clean committed source inventory containing the predeclared audited `perf_headless.py` and `perf_build_target.py` SHA256 values; the freezer's bound before/after inventory guard establishes build-time source equality. Later edits to the current workspace do not invalidate that frozen EXE. It independently checks the requested compiler-artifact/JSONL hashes and exact target/profile/features/EXE/PDB/command identity, the owned wrapper/Cargo event order, no compiler errors, and the same private-Job lifecycle proving assignment before Cargo, intentional termination after the completed artifact and zero remaining owned processes. A wrapper that naturally exits in the narrow artifact/termination race is accepted without promoting the result to whole-Cargo success. Dirty source, unknown freezer revisions, missing proof, unfinished artifacts, residual owned processes or contradictory whole-build metadata are rejected.

Target-only evidence explicitly leaves `entire_cargo_build_passed=false` and `remaining_targets_are_not_accepted=true`. It never claims that extra bins or workspace all-target checks passed. Those checks remain independent project acceptance requirements.

Example after the main agent freezes the appropriate common diagnostic-capable executable:

```powershell
python tools/perf_gpu_diagnostic.py --build-json .perf/headless-builds/BUILD/build.json --output .perf/gpu-diagnostics/RUN --scene static --warmup 600 --samples 3000
```

Run this separately from clean CPU captures. This tool has 24 synthetic Python validation tests so far; the Rust helpers have CPU-only tests pending the main agent's serial compilation. Actual GPU diagnostic smoke and four-scene coverage remain pending until explicitly run and checked.
