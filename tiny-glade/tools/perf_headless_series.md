# Cross-source headless A/B series v2 (prospective review proposal)

This tool plans commands and evaluates existing captures. It never executes
Cargo, a target executable, xperf, a logger, or a benchmark. It is separate from
`paired_overhead`; feature-overhead comparisons retain their existing semantics.
No result sets `production_16ms_acceptance=true` or accepts the entire candidate.

## Inputs and execution

Freeze two **clean release**, cpu-probe/default-mimalloc builds under the same
measurement checkout's `.perf/headless-builds/`. They may have different source
commits, content, EXE/PDB identities. Features, compiler, Cargo.lock, allocator,
build command/environment, worker/priority settings and runtime DLL hashes must
match. Requested-target completion is sufficient; unrelated Cargo targets are
not asserted to have passed. Each build needs its original `source.json`.

Algorithms are each source's compiled production defaults; their implementation
is the A/B treatment. The common harness reports
`algorithm_selection=compiled_source_defaults_no_runtime_ablation`,
`harness=common-cpu-workload-v1` and unavailable management diagnostics. Its
mod.rs, workload_probe.rs and gpu_diagnostic.rs source hashes must be identical
across both frozen sources and bound before sampling. Runtime-mode fields must
be None, and frozen mode-control capabilities must actually be unavailable
(default_modes=None, probe_runtime_controls=[]), as produced by the unmodified
freezer. No four-boolean default_modes or runtime values are invented.

None is accepted only under this versioned common-source marker plus each
predeclared source review, archived source/EXE identity, matching common3 SHA and
absent disable controls. Ordinary legacy None reports remain rejected. The
existing runtime controller clears inherited TG settings; this planner emits
no disable switch and rejects any such switch in the declaration. Clean session
reports must explicitly have measurement_class=clean_cpu,
clean_cpu_acceptance_eligible=true and gpu_diagnostic=None. GPU and management
diagnostics cannot inherit clean eligibility from run.json alone.

Example (commands are examples, not executed by this document):

```powershell
python -B tools/perf_headless_series.py plan windows-ab-NEWID `
  --baseline-build .perf/headless-builds/BASELINE/build.json `
  --candidate-build .perf/headless-builds/CANDIDATE/build.json `
  --calibration .perf/clock-policy/windows-etw-qpc-calibrated-v1.json `
  --baseline-producer-review .perf/producer-reviews/BASELINE-capability.json `
  --candidate-producer-review .perf/producer-reviews/CANDIDATE-capability.json `
  --output .perf/series/windows-ab-NEWID.json
```

The declaration uses exclusive creation. It binds its comparator, Python,
runtime measurement tools, calibration evidence, frozen source and binary/PDB
identities. Existing run directories are rejected; old captures cannot be
retrospectively enrolled because each capture's prospective FILETIME intent must
postdate the series declaration. Unmodified tools' per-capture intent and raw
trace checks remain mandatory.

The manifest contains 40 argv arrays in fixed order. First execute only the
first 24 `command` entries, sequentially: for pairs **AB, BA, AB**, visit static,
many_draws, local_edit, continuous_edit; run the selected A/B order within each
scene. Every run uses 600 warmup frames, exactly 3000 valid observations,
1280x720, frozen 1/60-step/world-scale1 workload, ETW, the same calibrated policy,
mandatory Microsoft CSwitch reference, and lossless CSV archive. No debug,
diagnostics, runtime ablations, mode-off, missing counts, dropped samples, failed
initialization or missing frames qualify.

After a capture and its exact workload evidence exist, execute its manifest
`audit_command`. `audit-run` performs **offline** strict raw-trace re-accounting
with the frozen implementation, not a new capture. The replay verifies the
parser's zero-lost-event, matching scheduling/boundary/lifecycle, owner2%,
official-complete-PID2% and clock-evidence requirements. It preserves the raw
GetProcessTimes failed flags; it never corrects counters. Microsoft is an
independent implementation over the **same event stream**, not an independent
clock. The replay must equal both cpu.json and run.json's embedded account.
Only then is a new per-run `series-audit.json` created, exclusively.

```powershell
python -B tools/perf_headless_series.py audit-run .perf/series/windows-ab-NEWID.json --ordinal 1
python -B tools/perf_headless_series.py evaluate .perf/series/windows-ab-NEWID.json `
  --output .perf/series/windows-ab-NEWID-result.json
```

Default evaluation hashes the audited run/session/cpu/workload/ETL/CSV archive/
official output/intent/Vulkan bytes against their audit seals. It does not parse
multi-GB CSV repeatedly. `evaluate --replay` instead replays every raw trace
directly, with no existing seals required. SHA seals detect accidental edits and
cross-run reuse; these are not cryptographic attestation against someone forging
both all artifacts and seals. Merely changing `valid=true`, or a percentile in
cpu.json, cannot pass an existing seal or raw replay.

Evaluation exit codes are0 for the headless CPU gate,1 for a hard rejection,2
for invalid/missing evidence,3 for the predeclared five-pair extension, and4 for
an inconclusive five-pair result;5 indicates pending workload review. Plan/audit exit0 describes only that operation;
it does not mean a candidate passed performance or production acceptance.

All four scenes and all selected pairs must be present, ordered by their true
QPC envelopes and prospective UTC/FILETIME provenance. No round or anomalous
tail is filtered. A/B and repeated rounds must have identical complete semantic
input-before AND input-after (including autonomous transform trajectory), draw
schedule, workload signature, driver/hardware, source-default selection contract
and settings.

## Receipt workload contract and source-audit prerequisites

This is a new prospective `headless-cross-source-series-v2` declaration. Old v1
plans, seals, and the frozen47-test prototype are not reclassified. The runtime
six-file measurement/clock tools remain unchanged; the additive fixture output
is consumed by a separately hashed `perf_headless_workload.py` adapter.

The actual fixture now emits `actual_workload.schema=headless-actual-workload-v1`.
It has samples+1 full receipt observations: row0 is retained as true raw workload
and a CPU attribution seed; rows1..N match N CPU observations. The adapter checks
header/row/native-command/Collect/device valid flags, any overflow/errors,
capacity/truncation, receipts, raw cumulative/delta conservation, stable queue
manager/device identities, and no unobserved Collect tail. None/missing is never
converted into zero. Errors in warmup remain sticky. Process-local manager IDs
and device generations are preserved and validated within each run, then omitted
from cross-process semantic equality.

The counter producer is **not trusted merely because its symbol is nonempty**.
Both builds require explicit, predeclared `--*-producer-review` references to a
`headless-producer-capability-v1` document containing:

- `raw_schema=headless-actual-workload-v1`, source_commit and source_content_sha256;
- `algorithm_selection=compiled_source_defaults_no_runtime_ablation`;
- `source_files`: repo-relative source paths mapped to immutable archived source
  path/SHA references, each SHA also present in the frozen build's source.json;
  all three common harness files must be included;
- `review_report`: path/SHA of an independently prepared
  `headless-producer-source-review-v1` source-audit result;
- the exact raw `coverage` object and seven reviewed `fields` groups:
  native_commands, draw_list_lengths, graph_retained_passes, graph_recording,
  collect, queued_buffer_h2d, device_upload.

Each fields group has a `meaning` equal to the adapter's FIELD_GROUPS constant,
nonempty symbols, and source_paths belonging to that exact archived source set.
The audit report binds identical source/commit/content, source_sha256 map,
fields and coverage, no unresolved findings, plus explicit checks of receipt
attribution, successful-prefix failure invalidation and arithmetic overflow.
The audit additionally binds algorithm_selection, runtime_ablation_supported=false
and controller_disable_controls_absent_checked=true.
Changing just a sidecar symbol, coverage text, review report, archived source or
capability file breaks a checked SHA or a frozen reviewed-field comparison.

This is source-audit provenance and integrity, not machine proof of semantic
correctness or cryptographic attestation against forged audit reports. A genuine
independent review must exist **before** declaring a real series. The tool does
not synthesize an approval or certify an unaudited counter implementation.

A `workload-proof.json` in each completed run directory contains:

```json
{
  "schema": "headless-actual-workload-proof-v1",
  "run_sha256": "SHA256",
  "session_sha256": "SHA256",
  "executable_sha256": "SHA256",
  "producer_review": {"path": "PREDECLARED_CAPABILITY", "sha256": "SHA256"},
  "raw_session_field": "actual_workload",
  "raw": {"schema": "headless-actual-workload-v1", "...": "exact raw session object"},
  "cost_difference_explanation": "optional analysis, never approval"
}
```

`raw` must exactly copy the frozen session object, including failed flags,
errors, originals, exclusions, cumulative counters and seed row. A hand-written
sidecar without that raw producer cannot qualify.

The hard A/B semantic comparison uses full input-before AND input-after,
clock/camera/script/transform trajectory and schedule, plus actual per-receipt
21 draw-list lengths, retained graph passes and the six native draw API
invocation categories. These API counts are **not GPU draw/instance counts**:
indirect API invocation1 can execute many or zero GPU draws. No field is named
or reported as if API calls establish indirect GPU count-buffer/argument work.
The current schema lacks a complete indirect argument/count identity proof;
that remains an explicit pending item rather than invented evidence.

Payload comparison is separate and exact per source and receipt: graph staging
H2D, immutable-producer H2D, cached D2D, queued-buffer H2D successful-submit
payload, and device texture setup/direct/transfer plus buffer setup payload.
No unrelated categories are summed into a fake complete upload total. The raw
coverage explicitly says `no_all_upload_total=true` and excludes mapped CPU
writes, device-to-host readback and clear/fill commands. This adapter preserves
that declaration and returns **pending workload coverage**. It does not silently
reinterpret those exclusions as a complete original-plan upload gate.

Any H2D/D2D payload difference requires manual review/pending. Free text cannot
make differing payload equivalent. Actual Collect jobs/waves, graph chunks,
dispatch API calls and queue jobs are implementation costs. They are compared
and reported, with optional explanation and mandatory difference review, rather
than hard equalities that prevent a legitimate reduction in redundant CPU work.
The current unchanged64-draw page policy predicts matching task counts, so a
change is still a signal needing source analysis. Cross-pass tasks/waves are
subsets of total Collect work, and graph recording chunks are a different
counter domain. None are added together or labeled worker CPU.

Even a numerically passing CPU series yields `pending_workload_review` when
coverage, payload differences or task/cost differences remain unresolved.
`render_numeric_gate_passed` preserves the measured numeric result while
`headless_cpu_gate_passed=false` and `candidate_accepted=false` avoid a false
complete acceptance. The tool contains no free-text override to clear pending.
A future stronger producer/consumer contract requires new source review,
versioned capabilities and a new prospective declaration, not edits to old runs.

## Hard gates and proposed five-pair extension

User-specified hard gates are applied to every pair/scene independently:

- Candidate **render phase owner CPU p95 <=16ms**. This is headless phase CPU,
  not the production render thread.
- Candidate render p95 and p99 are each <=105% of their paired baseline.
  A hard failure immediately rejects the candidate; more rounds cannot erase it.
- Raw ETW exclusive running CPU is used. No phase wall-time sum, parallel-worker
  elapsed-time sum or sum of percentiles substitutes for per-frame CPU.

The user's plan asks for extension near the thresholds but does not specify
numbers. The following are this tool's **prospective review proposal**, fixed in
the manifest before capture, not thresholds chosen after seeing the result:

At three complete pairs, extend to five if any scene has render p95 or p99 paired
regression >=3%, a max-minus-min paired regression spread >=3 percentage points,
any candidate p95 >=15.2ms, or a candidate-p95 range >=0.8ms. Only a
`needs_five_pairs` result authorizes the predeclared remaining BA, AB orders.
All scenes extend, both builds get the same two extra runs per scene, and the
first three rounds remain in the result. Partial, untriggered or selective
extensions are rejected.

At five pairs, all original hard gates still hold. In addition each scene's
render p95 and p99 paired run-ratios needs a **one-sided 95% Student-t upper bound
<=5%**, with t(4)=2.131846786326649. A failed bound yields `inconclusive`; no
automatic sixth round, selective retries or threshold change is proposed. Runs,
not 3000 autocorrelated frame observations, are CI units. This small-sample
model is a predeclared uncertainty check, not a universal population guarantee.

The tool separately reports mean/p50/p95/p99 for App, full-host owner, process
and non-owner CPU, plus paired ratios. Other-metric increases over5% produce
**cost-shift flags requiring investigation**, not extra invented hard failures.
Absence of such a flag is not proof that no work moved. Non-owner means every
other PID thread (including FMOD/audio, driver and runtime), not worker-only CPU.
`candidate_accepted=false` and `cost_shift_review_required=true` remain explicit.
Pixel correctness, probe overhead, GPU time, long-run/window stability and WSI
remain separate acceptance work.

## Offline tests

```powershell
python -B -m unittest discover -s tools -p test_perf_headless_series.py -v
```

Tests use synthetic counters, clocks, producer-review archives and CPU values solely to validate admission
and decision logic. The small real-smoke fixture is an exact field subset with
original-file SHA references, used only to prove debug/600-frame rejection.
Neither fixture nor the synthetic review documents are performance, genuine source-review or workload correctness evidence.
