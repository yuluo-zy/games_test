# Windows performance work

## GPU pass-level test plan: 2026-10-27

[GPU pass 级性能测试方案](gpu-pass-diagnostic-plan-20261027.md): serialised (isolated) per-pass GPU spans and per-pass pipeline statistics (`TG_GPU_ISOLATE=1`, `TG_GPU_STATS=1`, on top of `TG_GPU_TIMING=1`), the static / many_draws measurement matrix, interpretation rules, and the required same-machine comparison with the original game. The plan is not yet executed on a physical GPU; it reports no new numbers and marks no gate complete.

## Current WIP publication checkpoint: 2026-10-27

The user explicitly requested committing and pushing the current implementation and unfinished-work list before full acceptance. [Current measured result and remaining work](current-version-p95-20261027.md): static headless render-phase owner CPU p95 **12.6120ms** (3,000 samples); whole-owner p95 15.7644ms. Owner checks passed, while the original full-process capture remains failed. The other three current-version scenes, full correctness/performance/pressure acceptance and production-window verification remain pending. No relative speedup is established.

The former memory block has lifted and the probe/surface workspace all-target typecheck passed at bd7e109. The following checkpoints are retained at their historical source boundaries.

## Latest evidence: 2026-10-27 / native f39dc61; engineering fa98f8d

Native verification checkpoint `f39dc61eef85edebda2f491b4d31d9ff9e85c144` commits pending-acquire W2,
same-prefix original-format surface comparison, Drop heap diagnostics and optional
owner-preserving canonical layout preparation. These implementations still need
final-source Tiny/workspace compilation and native acceptance.
Engineering HEAD is now `fa98f8d4367cc6ce24fd5d7a5e03cc82b36008d7`;
f39's native evidence does not verify subsequent tools or EffectsHost changes.

Two Rhapsody observer tests passed on clean 6d1cf8a with a valid empty private Job;
the original Job-cleanup failure is retained. Integrated Rhapsody all-target
cpu-probe/default checks passed in 30.87s/29.91s before the canonical getters.
Final f39 Rhapsody all-target typechecks subsequently passed in 31.52s/31.00s;
this does not cover Tiny, country-core, the workspace or new native tests.
Python 63 tests passed in 13.846s. Tiny fresh observer build and final canonical
metadata check failed on compiler allocations of 14,942,224 / 262,144 bytes;
neither ran GPU. New canonical 5 / local-surface 6 / pending-acquire 9 tests remain
unexecuted, plus the two Tiny observer tests. Pending9's libtest build failed
allocating 2,097,152 bytes, without tests/GPU. The heap2 native diagnostics did run:
cached round2 is valid (2 passed / 0 ignored); round1 passed its assertions but
remains invalid due to owned-Job drain timeout. Default suballocation still
allocates 1/64B at unused Drop and 33/2,640B at fragmented reclaim;
this is no CPU-ms claim. EffectsHost's separate history-validity fix adds one
8-step native lifecycle test, still unexecuted and not Tiny-typechecked.

New tool-only rounds passed 135/44/94 Python tests; the original surface 91-test
round with two errors is retained. Qualified tool/importer byte identities were
preserved. Installed diagnostic tools do not activate an isolated common4
baseline or accept a new binary/GPU run.

The c98 whole-scene water/ice strict pixels still fail; Nani GPU bytes were
Unavailable, so complete GPU-input equality is unproven. Wall 3-vs-4 sparse ranges
explain the c98 trace's two-pass dispatch difference, without a receipt join or
CPU-gain evidence. No old invalid run is reclassified. Common4's isolated baseline
is inactive; fatal-OOM/automatic retry are not implemented. Fully bounded stage 5, formal CPU/16ms, pixels,
long runs, macOS and deferred WSI remain open. No settings changes or push occurred.
Latest available commit is about 1.13GiB; further native/heavy Tiny builds are
stopped, awaiting the user's manual memory-release response.

See [scoped evidence](C:/game/docs/performance/evidence-update-f39dc61-20261027.md),
[remaining work](C:/game/docs/performance/remaining-tasks-20261026.md) and
[status.json](C:/game/docs/performance/status.json), active section
`latest_progress_20261027_f39dc61`. Earlier sections below are historical.

## Previous checkpoint (preserved verbatim)


## Latest checkpoint: 2026-10-27 / c98be8e

Native Collect's isolated strict GPU recovery test passed (1/0/0, 0.87s,
zero VUID): real 579-worker-draw/12-task/2-wave workload, cancellation while A
remains submitted, real budget refusal before workers and complete pixel recovery.
New targeted checks passed: Rust trace 6 / Wall source 3 / Nani observer 3,
Python parser 8 / trace-source controller 9. These do not accept shipped surface
pixels, full GPU inputs, CPU gains or 16ms.

The clean c98be8e rich debug EXE is frozen (064462bf22ced894f19848bf7dffc81f726cde663e9fb8f5207847450141092a).
Only its requested target completed; the whole Cargo invocation is not accepted.
Its static 60/30 trace/source OFF/ON investigation completed with valid diagnostic
structure but both raw CPU runs invalid: all 31 rows differ (OFF 355 / ON 357
direct dispatch), with 59 full-actual and 1,333 Wall-source field differences.
These source differences do not prove binding causality; the direction reverses
the old pair. The original mismatch and strict water/ice failures remain invalid.
Baseline common 000e7cc Release is frozen/imported; candidate Release is
pending. Both common trees share workload helper SHA256 3890c544d3f79c2d8a542acf064d70ef66cdae351a5b33d297d6bd2b6359eae4.
Portable importer 26 and series/import binding 88 tests passed separately.

Latest diagnostics have not received fresh default/probe workspace all-target
checks. Release ETW/overhead/four-scene A/B, strict surface pixels, complete
pressure/long runs, WSI (user deferred) and macOS remain open. No push occurred.
See [latest tasks](C:/game/docs/performance/remaining-tasks-20261026.md) and
[latest identities/evidence](C:/game/docs/performance/status.json), whose
latest_progress_20261027 section supersedes the historical snapshots below.

## Historical progress snapshots (retained at their original source boundaries)

Source: codex/render-cpu-cleanup-20261026 at 6957077f9; independent Cgame repository.
Current committed source is 370f795: demand plan 7c42f3a and the test-only
timeline fixture 370f795 are committed. Fresh all-target checks remain pending;
performance is unaccepted. No remote is configured and no push has occurred. After all requested
work completes, authorized delivery is a new codex/perf-windows-20261026 branch
at git@github.com:yuluo-zy/games_test.git, preserving true 8fd→695 ancestry and
mapping stage changes under tiny-glade/ without deleting omitted RE/assets.

One new prospective-clock debug quality run is valid: 600 warm / 600 samples,
owner OS error 1.2780%, process 0.3694%, Microsoft process 0.0010603%, 16 TIDs complete.
Old strict rounds remain invalid. This is not Release, probe-off≤1%, 16ms or
four-scene acceptance. Actual constant ON/OFF counters expose fewer lock entries
but more backing creation/scanning; no time gain is claimed. Current tiny full
probe fails 306/1/5 ignored; Country draw-passes now passes 265 after a test-fixture correction. Real lake inline ran but
VUID invalidated the gate; Collect/pixel pair is still unverified.

Read [current evidence](C:/game/docs/performance/evidence-update-3e3b29c-20261026.md),
[remaining tasks](C:/game/docs/performance/remaining-tasks-20261026.md),
[measurement closure](C:/game/docs/performance/measurement-closure-20261026.md)
and [status/identities](C:/game/docs/performance/status.json).

## Acceptance and reproducibility

Four scenes are static, actual draw-list≥1000 many_draws, local raster edit every
30 frames and continuous edit every frame. Use 1280×720, scale 1, fixed 1/60,
unchanged quality/content, existing mimalloc/dependencies/driver, VSync off and no
FPS cap. Each scene needs 600 warmup / 3000 valid samples, three runs. Final AB/BA/AB
requires render CPU p95≤16ms in every scene/run; p95/p99 regression>5% rejects.
Pressure, faults and three 30-minute/10000-frame stability runs remain required.
GPU-init failures, zero tests, ignored tests and VUID logs do not count as passes.

Artifacts are ignored under .perf; source/docs summaries are versioned. Use
TG_INSTALL_ROOT for external original assets. Do not copy game binaries/caches/RE.
WSI swapchainMaintenance1 compatibility is deferred by user request; headless
success does not replace production render-thread/window acceptance. macOS is pending.

## Tools

Use tools/perf_headless.py for explicit test-artifact freezing, prospective-clock
capture and accounting. Keep each tested source commit/content hash, EXE/PDB,
tools, input/trajectory and clock intent/evidence identity. Requested test target
completion is separate from a full Cargo invocation passing.

The portable [thread-clock instructions](C:/game/tools/thread_clock/README.md)
declare a fresh calibration without starting it; the printed command separately
runs six new captures. Portable 24 tests, padding 15 tests, policy 100 tests and prior
archive 76 tests are tool evidence at their checked sources, not renderer performance.
Old helpers/evidence are unchanged; new policy never retroactively blesses old runs.

Correctness gates include native view usage 3/range 1/full Rhapsody 458+2 ignored
without VUID at f734239. The mapped timeline exact test now passes; full tiny still fails its preexisting
rollercoaster assertion. Corrected Country targets pass and are
recorded separately. The isolated exact-production 29095eb baseline with measurement
brackets is prepared in staging but unbuilt. Complete requested work before push.

新增测试夹具补齐 bind_common 必需 preset 0x11/0x12 后，[cross-pass 复验](C:/game/.perf/constant-demand-cross-pass-fixed.log) exact 1 pass/0 ignored，[draw-passes 全套复验](C:/game/.perf/constant-demand-country-draw-passes-fixed.log) 265 pass/0 fail/0 ignored；builder 168 pass 保持。生产/断言未改，旧失败仍保留；不代表 tiny full 或当前全目标通过。

Source is now 370f795 (demand plan 7c42f3a committed). The mapped timeline exact test passes 1/0/0; full direct-EXE tiny rerun 306/1/5 ignored has zero VUID but still fails. Fresh workspace default/probe all-target checks are running, not passed.

## 全目标检查更新

`370f795` 源码的默认工作区全目标检查通过（2m42s），独立 `tiny-glade/cpu-probe` 全目标检查通过（36.39s），日志分别为 `.perf/constant-demand-workspace-default-check.log`、`.perf/constant-demand-workspace-probe-check.log`。该检查不替代真实帧账单、水/冰像素或 Release 性能验收。
