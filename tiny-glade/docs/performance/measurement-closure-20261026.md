# CPU measurement closure and static-management investigation

**The measurement pipeline is implemented, but its acceptance gate remains open.**
The 600-warmup/3,000-sample run passes the owner cumulative check (0.53423%) and
fails the process cumulative check (4.75882%). The whole performance round is
invalid. Current ownership/Collect source changes are applied. Their checked snapshots
have passed their recorded default/probe all-target compilation; corrected
core16/graph5 and the corrected strict library retry pass; valid A/B is pending. No whole-frame speedup or
16ms acceptance is claimed.

## Source and verification boundaries

`29095eb143a029eb32edef56176bc1b6e3577c77` committed the opt-in shipped-session
headless fixture, phase/completion correlation and actual workload accounting.
`27f77c5` committed the ETW accounting and target-only freezing tool corrections.
Subsequent committed candidates are `44956f8` (exclusive CPU storage owners),
`b79443e` (reflection scope), `cca55ad` (typed VAO lookup), `6ecdb2f` (scoped
wrapper reuse), `19c52eb` (view generation/FrameGlobals), `709d717` (task-batched
constants), `a36fbd3` (bounded Commit scratch) and `6a85ec8` (first declaration
error retention and public into_parts four-tuple consumers). Follow-up validation
fixtures are committed in `f4e3e04`; `fb778bd` commits the scoped surface
reference control and worker-order tests. `e38ae7f` adds management-mode
identity, constant/scratch diagnostic bills and per-API QPC read brackets without
correcting the legacy CPU envelope. `fa90e4a` adds original rocky-lake Autumn
water/Winter ice capture through the production graph. This real fixture has not
run. Source is committed through `fa90e4a`; no new candidate is frozen or
performance-accepted.

Earlier checked snapshots passed default/probe all-target compilation. The
Graph API's default all-target check passed in 2m 40s and its probe all-target
check passed in 2m 36s. The corrected strict Vulkan core16 retry passed **16/0/0** and
the first-error graph tests passed **5/0/0**, with no VUID in either run. The
original 14-pass/2-fail core run and OOM-to-UninitializedRead diagnostic remain
failed historical evidence; the retry preserves the original error assertion.

The earlier full Rhapsody library run reported **458 logical passes, 0 logical failures
and 2 ignored**, yet forced validation emitted
`VUID-VkBufferMemoryBarrier2-buffer-09095`: **the full validation gate failed**.
Synthetic fixtures submitted fake family 1-to-0 transfers to this single-family
GPU. The pending_acquire.rs/transfer.rs fixture corrections were committed in
`f4e3e04`. The latest complete strict rerun passed **458/0/2 ignored**, with
no VUID or Validation Error anywhere in the log. The original failure and
intermediate fixture failure are retained; ignored tests are not passes. Real
distinct-family GPU execution remains unverified. Neither
validation disabling nor a hardware skip substitutes for these gates.

The earlier 453-pass Rhapsody snapshot and 8 CPU storage/bytes/scratch successes
remain separate evidence. Tiny targets passed 9 default constant-pool, 6 scope,
3 typed VAO, 3 native-constant and 4 texture-lifetime tests; original shader GPU
passed 2 and VAO scope 4 with actual strict Vulkan and no VUID in those runs.
The latest strict rerun is separate evidence; it does not rewrite that earlier
full-library validation failure. Country pass_builder passed 166 tests (including
3 scratch/rejection tests), lease ownership passed 1 and draw-passes passed 262,
all with 0 failures and 0 ignored.

The current tools commit is `c5a6125`: 76 tests passed, 0 skipped, with the owned
Job smoke enabled. It adds verified lossless trace CSV archival for completed
new captures; original ETL, decompressed identity and byte counts are retained.
It does not modify old runs or acceptance flags. The earlier `a84ad66` 63/0
summary and original 62/1 log remain historical evidence. The separate external
thread-accounting helper has 17 passing tests; it is diagnostic instrumentation,
not a rendering/performance pass.

The historical 445 Rhapsody unit successes, 55 integration successes, 260 draw-pass
successes and 30-frame original gameplay run belong to `53bbbef` and earlier
source. They remain useful prior evidence but do not verify the new changes.
The old 2 ignored Rhapsody diagnostics never counted as passes. Fixture/tooling
checks before freezing and 45 Python tests after the tooling corrections likewise
do not establish current-source rendering correctness or performance gains.

## What the fixture measures

The existing shipped-session App and production world-graph path share one
headless owner. ETW-intersected QPC phase boundaries isolate exclusive owner CPU;
App/update is accounted separately. Other target-process threads include task
workers, audio/FMOD and driver/runtime helpers; `non_owner_cpu_ms` is not a
worker-pool-only measurement. This is not the production dedicated rendering
thread and does not replace WSI acceptance.

The fixture uses default mimalloc, original quality, caller-owned reusable
DrawLists and fixed 1/60 simulation. It records fixed-capacity v2 completion points
and serializes after the interval. A ready/go handshake brackets kernel tracing
after warmup. The controller stops only a session it successfully started. Trace
loss, clock/CPU discrepancies, incomplete samples, identity mismatch and workload
mismatch invalidate a run; no abnormal tail is silently removed.

Four-scene stimuli preserve the existing real ECS clones and bounded PathRaster
producer protocol: source clones after frame 10, scene declaration at 199, edits
from 200, local edits every 30 frames or continuous edits every frame. Summaries
separate logical edited bytes, actual producer dirty spans, collected uniform
writes, binding inputs and native owner lifetimes. Producer-emitted dirty bytes
are not accepted/submitted/completed GPU upload bytes.

The no-edit intro is not immutable: three sheep and six accessories animate.
Both raw endpoint manifests, clock/camera bits and ordinary-mesh Transform
trajectory are retained. Paired runs compare the same script and simulation
origin; normal evolution within a run is permitted. Transforms are not deleted
from identity and simulation is not frozen. A raw atlas-part ordinal difference
remains unresolved until associated with actual geometry; removing mesh_index
alone cannot establish equal work.

## Frozen Release artifacts actually produced

Both builds have source commit `29095eb143a029eb32edef56176bc1b6e3577c77` and
content SHA256 `f7468a9e3df20837c620e6a4b00751d5c40389efb9c2c11fa9f4790eb7860590`.
They share Rust 1.97.1/x86_64-pc-windows-msvc, release profile, default mimalloc,
one build job, incremental disabled, identical build environment/hardware and
Cargo.lock SHA256 `61808d35effd5095a81591fa326e63adc5c3a2439f459fc3656742bdc87ede36`.
Only cpu-probe differs. Full manifests are
[ON build.json](C:/game/.perf/headless-builds/closure-base-on/build.json),
[OFF build.json](C:/game/.perf/headless-builds/closure-base-off/build.json) and
[feature-pair.json](C:/game/.perf/headless-builds/closure-feature-pair.json).

| Artifact | SHA256 |
|---|---|
|ON original_gameplay_chain-00f90e0721e8c60b.exe|`7113e4960d6e972a8e9f258699eb29a043a1a8b718d5d488d390e22ac523fdeb`|
|ON PDB|`e26076fe3f8938e0b0a72f4869a4f73cdb1f4b529a2f5d1dd6f4498dcd58bee9`|
|OFF original_gameplay_chain-9002b51216012401.exe|`a98475d2d8ae27a7f68a96372be8ff1eeb82f253f49758e3c1bcff34dac717c6`|
|OFF PDB|`a90ad7c334ee71e8e9a4cbac554b5c7262b87259cc004c2e8bc0a4f30e1b0bf9`|

Each requested integration-test compiler artifact completed and was verified.
Cargo also selected game binaries. Their unnecessary compiler processes were
terminated only in the owned Windows Job after the requested target was ready.
Consequently `requested_test_targets_completed=true` and
`entire_cargo_builds_passed=false` are both accurate. These artifacts do not
contain the later ownership candidates, and neither their identity nor their
test success transfers to a new source revision.

## Real ETW/OS closure results: invalid rounds retained

The 60/300 smoke ran with stage logs disabled and passed its native fixture 1/1,
zero ignored. Its recorder exported 300 complete clean samples. Performance
acceptance nevertheless failed: owner error 4.236%, process error 4.058%, each
above 2%. Microsoft's independent owner integration agreed closely, which did
not override the OS-counter failures. Evidence:
[short run.json](C:/game/.perf/headless-runs/closure-etw-smoke/run.json).

The subsequent 600/3,000 run used the SAME frozen ON executable. Its envelope
contains 3,001 executed frames, wall time 474,606.775 ms. Its actual cumulative values:

| Quantity | Actual value |
|---|---:|
|ETW owner / GetThreadTimes|38,263.2384 ms /38,468.7500 ms|
|Owner relative error|0.53423%, this check passes|
|ETW process / GetProcessTimes|147,930.3571 ms /140,890.6250 ms|
|Process relative error|4.75882%, fails; round remains invalid|
|Microsoft full-process CSwitch total (PID 29044)|147,929.8390 ms|
|Own process integration vs Microsoft|0.5181 ms; 0.0003502%|

The official process total includes all 20 nonzero-time target threads with unique
TIDs, not just the owner. Both ETW integrations read the same scheduling source;
their agreement cannot replace the independent cumulative OS accounting gate.
The 7.0397-second process discrepancy grew with the longer run and cannot be
explained away as merely one endpoint tick. Warmup length and simulated origin
also differ between these runs, so they are not an optimization A/B pair.

Keep [long run.json](C:/game/.perf/headless-runs/closure-static-long-1/run.json),
[raw session.json](C:/game/.perf/headless-runs/closure-static-long-1/session.json),
[CPU reconstruction](C:/game/.perf/headless-runs/closure-static-long-1/cpu.json),
ETL/CSV and the official integration. Exact file identities and residuals are
listed in [the clock investigation](C:/game/docs/performance/windows-clock-accounting-20261026.md).
No new endpoint metadata is backfilled into these old binary results.

External per-API QPC brackets and retained per-TID kernel/user snapshots now
exist in the separate diagnostic below. More predeclared repeat calibration and
a prospective clock-policy review are still needed before fresh clean A/B; do not relax 2%, discard
tail samples or use only the passing owner check. Complete feature-off overhead
must also pass both paired ratio confidence bounds and aggregate ratio-of-means
within 1%. That gate remains open.

## Unchanged draw/resource management investigation

The initial 30-interval debug diagnostic, before final clock/trajectory hookup,
had valid recorder 30/30 but failed old endpoint equality because of the moving
sheep/accessories. Its per-frame deltas were 777 reflection queries (all cache hits),
384 constant-block takes, 19,636 collected uniform-write bytes, 1,301 resource
inputs, 361 bindless-draw preparations, and 2 full VAO refreshes. Those refreshes
summed to 344 list entries, 190 queried entities, 128 unique visits,
3 publications and 63 new wrappers. The 118 cached variants are cache stock,
not per-frame unique variants; 128 VAO unique visits across two refreshes are
not a distinct whole-frame physical-resource count.
These are path counts from that source, not exclusive CPU costs or measured
gains. Keep [the failed debug log](C:/game/.perf/closure-debug-smoke.log) and its
session output. Final callback wall times from the old 30-frame gameplay run are
also not summable across recording threads as owner CPU p95.

Applied current candidates, compile-checked but not yet correctness/performance-accepted as a combined source:

- An immutable 32 MiB bounded reflection scope shared by admission/worker, with CPU
  backing and rehash-peak credit. Per-draw returned Arc clones and the original
  global reflection cache still exist.
- Typed-entity VAO lookup with fallback for unresolved legacy lists and strict
  physical generation checks; scoped full/subset/full wrapper reuse preserves
  both refresh consumers and the terrain replacement boundary.
- Task-owned opaque CPU constants and bulk pool return; owner bulk native-range
  allocation is bounded to 64 items and writes after releasing its mutex.
- Exclusive, device-budgeted Commit/remap scratch over the existing bounded CPU
  pool, with bounded capacity/growth and reset graph-local state on return.
- View creation outside host-table locks with generation/physical-owner recheck,
  plus immutable FrameGlobals take/replace and old-owner drop outside locks.
- Test-support-only genuine water/ice inline reference, stable per-draw order,
  transparent pixel, real unsubmitted-graph cancellation and budget/write failure
  recovery tests; original rocky-lake Autumn/Winter fixture with raw captures.

These are implemented candidates with targeted unit/GPU successes, including
the corrected core16 and graph5 runs. The corrected full-library
strict rerun also passes; complete scene correctness/performance is not accepted. The view generation fix closes a correctness race and can incur duplicate
native candidates. The representative lake fixtures still require successful execution; the
corrected core budget/recovery suite passed, while complete scene acceptance is open. Mid-worker
cancellation has no existing protocol; supported tests cover completed Collect
before submission. Remaining larger upload/COW/binding/static-description changes
require actual lock/copy/repetition evidence before implementation.

## Remaining platform, performance and delivery gates

Valid closed measurements must precede per-candidate CPU-cost reductions and the
four-scene comparisons. All four scenarios still need three valid 600/3,000 runs,
matching actual draw/task/upload/input evidence, followed by final AB/BA/AB,
p95≤16ms in every scenario/run and p95/p99 regression checks. Representative
water/ice pixel/command equivalence, cancellation/recovery, pressure/fault and
30-minute/10,000-frame stability runs remain open. WSI compatibility is deferred
by user instruction; macOS compilation/runtime remains unverified.

The user authorized delivery to `git@github.com:yuluo-zy/games_test.git` after all
requested work is complete. No push has occurred. Real upstream ancestry is
remote `8fd2a3c` → local `6957077f9`; publish a new codex/perf-windows-20261026 branch
preserving that history. Map only post-import stage changes under tiny-glade/,
explicitly handle four CRLF/LF import-byte differences, preserve omitted RE/assets/
reports, and verify mapped source identity and remote state before publishing.
Never force the independent Cgame import root over upstream history.

## External per-thread accounting evidence; no policy change

The staged external helper ran the old frozen `29095eb` EXE, not a new candidate.
Its game run used 600 warmup frames and 600 samples. The external ready/go-to-exit
window was 86.3495494 s, including post-envelope manifests, drain and shutdown;
the inner fixture envelope contains 601 executed frames. They are different
windows and cannot be substituted for the failed 3,000-sample inner run.

All 21 native thread objects were retained through termination and read with
their own QPC brackets. Coverage was complete: no missing running threads,
unresolved births, ID reuse or overflow. Thread OS deltas summed exactly to both
process sandwich values, 26,796.8750 ms. API windows were microseconds and the
process/thread window contributions were about a millisecond at most.

| Diagnostic | OS ms | ETW ms | Result |
|---|---:|---:|---|
|Quick continuous hash worker|953.1250|947.9598|0.5419% difference|
|Same-workload quick burst hash worker|1453.1250|993.6653|OS **overcounts** by 459.4597 ms, 31.6187%; fails|
|Game external process before sandwich|26796.8750|27222.4667|1.5634% aggregate difference|
|Game external process after sandwich|26796.8750|27220.3017|1.5556% aggregate difference|

The quick pair matched input bytes, chunks, workload SHA and final worker
digests. One pair demonstrates the pattern-dependent discrepancy; it is not a
repeat calibration or a universal correction factor. In the game diagnostic,
compute(0) has OS minus ETW +591.1493 ms (19.0118%), while an unnamed thread and
the BVH worker have -818.4044 ms and -666.8491 ms. Opposing thread errors partially
cancel in the process aggregate. Its approximately 1.56% does not establish
per-thread accuracy or full closure. Microsoft process CSwitch totals agree
with the custom integration to 0.000971%, but use the same trace source.

This evidence constrains omitted-thread/process-sum and read-window explanations
for this capture. It supports workload-pattern-dependent accounting distortion;
it does not prove a particular modern Windows internal charging algorithm.
The original 2% flags and final 16 ms gate are unchanged. Older invalid rounds
remain invalid. The longer, predeclared repeated calibration below is complete, but an explicit
prospective policy must still be reviewed before fresh clean baseline/candidate runs; no favorable
subwindow or aggregate replaces a failed criterion retrospectively.

Evidence: [quick calibration summary](C:/game/.perf/thread-cpu-calibrations/thread-clock-quick-1/summary.json),
[game thread report](C:/game/.perf/thread-cpu-diagnostics/thread-clock-game-w600-n600-1/thread-cpu.json),
[native counters](C:/game/.perf/thread-cpu-diagnostics/thread-clock-game-w600-n600-1/native-counters.json)
and [detailed actual-results report](C:/Users/liyu/.codex/visualizations/2026/10/26/01a17714-b237-7ed3-8e87-7aeaa53c4a51/thread-diagnostic/actual-results-20261026.md).

## Predeclared long clock calibration; raw failures retained

All six synthetic runs in the predeclared AB/BA/AB series are complete. Each
uses 4 workers and 4,096 chunks of 1 MiB per worker; burst waits are 10 ms.
Every run has complete coverage and the same workload SHA. Signed percentages
below use the report's relative-error denominator and the OS-minus-ETW sign.

| Pair/order | Pattern | OS process ms | ETW process ms | Signed error | Existing 2% |
|---|---|---:|---:|---:|---|
|1 A|continuous|68000.0000|67805.6161|+0.2859%|pass|
|1 B|burst|65125.0000|73515.2785|-11.4130%|fail|
|2 B|burst|63406.2500|73504.3007|-13.7380%|fail|
|2 A|continuous|68125.0000|67764.0904|+0.5298%|pass|
|3 A|continuous|66390.6250|66265.6266|+0.1883%|pass|
|3 B|burst|59109.3750|72744.6982|-18.7441%|fail|

Each continuous primary worker also stays within the existing 2%; each burst
primary worker fails it. Here OS undercounts the burst runs, whereas the earlier
one-worker quick burst overcounted. This excludes a universal sign or correction
factor. It does not certify the real game's measurement window. The policy is
unaccepted, old invalid runs remain invalid, and neither the 2% flags nor final
16 ms/performance gates have changed.

Evidence: [long-series small summary](C:/game/.perf/thread-cpu-calibrations/thread-clock-4w-long-1/summary.json),
[corrected strict core16](C:/game/.perf/declaration-rejection-core-raster.log),
[strict graph5](C:/game/.perf/declaration-rejection-five-strict.log), and
[failed full-library validation](C:/game/.perf/declaration-rejection-rhapsody-lib.log).

## Pre-freeze source and validation snapshot

`f4e3e04` commits the native-safe synthetic ownership fixtures, and `fb778bd`
commits the scoped surface reference control and ordered worker tests. The
[latest full-library strict retry](C:/game/.perf/rhapsody-validation-fixtures-fixed-2.log)
passes 458/0/2 ignored with no VUID/Validation Error. The
[new Graph probe check](C:/game/.perf/declaration-rejection-probe-check.log)
passes in 2m 36s. Country [pass_builder 166](C:/game/.perf/candidate-country-pass-builder.log),
[lease 1](C:/game/.perf/candidate-country-1.log) and
[draw-passes 262](C:/game/.perf/candidate-country-262.log) pass with 0 ignored.
The real production-graph lake fixture committed in `fa90e4a` has not run;
representative water/ice GPU pixel comparisons remain pending. `e38ae7f` records
actual management modes and cumulative CPU API QPC brackets without adjusting
reported CPU values. These new fixture commits do not validate the old frozen EXE
or turn an invalid old measurement into a valid one.

The prospective clock policy exists only in staging with 92 tests. Independent
review found a P2 issue under correction; policy approval, real closure and
performance acceptance are pending. Old gates and invalid runs are unchanged.
Actual progress-document writes stop after this batch for stable source freezing;
later evidence is staged first.
