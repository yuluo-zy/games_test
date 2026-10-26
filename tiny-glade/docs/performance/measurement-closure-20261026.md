# CPU measurement closure and static-management investigation

The existing shipped-session App and production world-graph path now have an explicit opt-in CPU fixture. It uses default mimalloc and caller-owned reusable DrawLists; no alternative renderer or reduced visual workload is introduced. App/update and rendering share one headless owner, so only ETW-intersected phase CPU can isolate rendering work. This never replaces production WSI acceptance.

The fixture warms the real workload before a ready/go handshake, records fixed-capacity QPC phase boundaries and v2 completion points, and serializes after the interval. Envelope-only feature-off and runtime-off controls retain the same workload. Windows thread/process CPU clocks are shared with the original recorder; raw cumulative readings remain visible. The controller starts tracing only after warmup and stops only a kernel session it successfully started. Trace loss, missing identities, clock/CPU discrepancy, incomplete samples and workload mismatch invalidate a run.

Four-scene stimuli reuse the existing true ECS clones and bounded PathRaster producers. The measured interval includes the identical workload validator/trajectory hash on both builds. Counts distinguish logical edits, producer dirty spans, collected uniform writes, binding inputs and native owner lifetimes; these are not interchangeable upload-byte metrics.

The original intro is not strictly static: three sheep and their six accessories animate. Both raw endpoint manifests, fixed-step clock/camera bits, and ordinary-mesh Transform trajectory are retained. Paired runs must match all of them; the measurement cannot silently delete moving transforms or freeze simulation. Unchanged draws/resources are investigated separately from autonomous changes. A raw ordinal-to-atlas ambiguity, if found, remains invalid until resolved to actual geometry.

Initial debug diagnostic (30 intervals, pre-final clock/trajectory hookup): v2 recorder was valid 30/30 but the old endpoint-equality assertion correctly exposed those moving transforms. Global reflection queries were 777/frame (hits; 118 cached variants), constant-block takes 384/frame, binding resource inputs 1301/frame, VAO full refreshes 2/frame, publications 3/frame, newly allocated snapshots about63/frame. These are local path counts, not CPU costs or gain percentages. The failed diagnostic remains .perf/closure-debug-smoke.log and session.json.

Validation before freezing: probe/default fixture checks passed; 11 recorder/stage tests passed; 35 Python accounting/session-safety/identity/noise tests passed. Real ETW/game CPU reconciliation, complete feature-off overhead <=1%, frozen optimized baseline, per-candidate A/B, all four-scene and water/ice acceptance remain open until raw successful runs exist.

Prepared candidate work is staged independently by agents: reflection scope, typed-entity VAO lookup and same-frame snapshot reuse; constant task/bulk commit and bounded scratch; view-generation correctness and globals lock scope; real transparent/cancel/budget and lake Autumn/Winter tests. They are not yet applied. Measured hotspots determine promotion; correctness and pixel equivalence remain separate gates.

## First frozen release run: acceptance still open

The source baseline is `29095eb143a029eb32edef56176bc1b6e3577c77`, content SHA256
`f7468a9e3df20837c620e6a4b00751d5c40389efb9c2c11fa9f4790eb7860590`.
The probe-on EXE is `7113e4960d6e972a8e9f258699eb29a043a1a8b718d5d488d390e22ac523fdeb`;
the probe-off EXE is `a98475d2d8ae27a7f68a96372be8ff1eeb82f253f49758e3c1bcff34dac717c6`.
The pair has matching source, lockfile, release profile, mimalloc, toolchain,
build environment and hardware; only cpu-probe differs. Evidence is retained
in `.perf/headless-builds/closure-feature-pair.json`.

Each requested integration-test artifact completed successfully. Cargo also
automatically selected game binaries; the unnecessary owned compiler/Job was
stopped after the test artifact completed. This is not a successful complete
Cargo build or production-window acceptance. The target-only builder records
these outcomes separately and verifies the copied EXE/PDB/DLL and test entry.

`closure-etw-smoke` ran 60 warmup frames and 300 valid observation frames, with
phase logs disabled. The native test passed 1/1 with zero ignored tests; v2
recording has 300 clean samples, no dropped records and no truncation.
However, **the performance round is invalid**: cumulative owner CPU differs
from GetThreadTimes by 4.236%, and process CPU differs from GetProcessTimes by
4.058%, exceeding the 2% crosscheck gate. ETW reconstructed owner CPU agrees
with Microsoft's independent CSwitch integration to 0.000137%; this agreement
does not override the failed OS-counter checks. A 600-warmup/3000-sample run
and an independent clock/accounting investigation are in progress. No speedup
or 16 ms acceptance is inferred from this invalid round.

The `non_owner_cpu_ms` field aggregates every other target-process thread,
including task workers, audio/FMOD, and driver/runtime helpers. It is not a
worker-pool-only counter. App and renderer still share the headless owner.

The accounting fix uses DPC/ISR event QPC as the end of the interrupted
interval, as in Microsoft's KernelTraceEventParser. It retains raw ETL and
cumulative clocks. Probe overhead acceptance requires both its paired ratio
confidence interval and aggregate ratio of means to stay within 1%.

After the accounting and target-builder changes, 45 Python tests passed with
`TG_BUILD_JOB_SMOKE=1`, including actual owned-Job isolation. This validates
the tools, not the outstanding game-performance or window gates.
