# Windows performance work

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
