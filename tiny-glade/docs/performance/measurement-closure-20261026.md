# CPU measurement closure: current evidence and limits

**One prospective-policy debug clock-quality round is valid. Release performance
and complete measurement/Collect acceptance remain open.** Current committed source is `370f795`; demand plan `7c42f3a` and the
test-only timeline fixture are committed. Fresh default/probe all-target checks
are running; no pass is claimed.
See [current evidence](C:/game/docs/performance/evidence-update-3e3b29c-20261026.md)
for exact counts, failures and paths, and [status](C:/game/docs/performance/status.json)
for machine-readable source/binary boundaries.

## Newly completed quality run

`.perf/headless-runs/calibrated-clock-smoke-a2a8fce-1/run.json` uses the frozen
a2a8fce debug EXE, 600 warmup/600 samples and 601 executed frames. The prospective
`windows-etw-qpc-calibrated-v1` intent was bound before child start. Run/account
valid are true. Owner ETW/GetThreadTimes differs by 1.2780%; process/GetProcessTimes
by 0.3694%; custom/Microsoft full-process CSwitch by 0.0010603%. All 16 TID
lifecycles are covered. Microsoft is an independent implementation over the same
ETW events, not a second independent clock. No raw counter is corrected.

The completed run manifest records lossless CSV archival: 2,010,015,522 raw bytes
to 246,037,246 gzip bytes, decompressed roundtrip identity verified and ETL retained.
This document did not rehash/read those large trace files.

This is debug quality validation, not a clean Release baseline, probe-off≤1%,
four-scene 600/3,000 comparison, dedicated production render-thread 16ms acceptance
or WSI acceptance. Headless App/update and rendering share the owner; other
process CPU includes workers, FMOD and driver/runtime helpers.

The formal policy evidence SHA is
`6a3274f4d8753d30b04348a701804bf4a796788c5ab11b11cbd140ce467330ae`.
Commit a2a8fce qualifies future captures only. Commit 3e3b29c ships portable
predeclaration/calibration sources, with 24 actual tests passed and zero skipped.
New machines generate fresh declarations and six new captures; they do not
reuse this machine's old bundle. Original helper/calibration/evidence files stay
unchanged. Same-model CPU/OS discrimination of the existing policy CLI remains
an explicitly documented limit; portable host binding is not physical attestation.

## Old strict rounds remain invalid

The original frozen Release ON/OFF test pair belongs to 29095eb, not current
source. Only cpu-probe differs; mimalloc, jobs=1, incremental off and lock/toolchain
identity match. Both requested integration-test artifacts completed, but entire
Cargo invocations did not pass: owned Job termination stopped unneeded targets.
Metadata and EXE/PDB hashes remain in status and
`.perf/headless-builds/closure-feature-pair.json`.

The 60/300 strict smoke failed both owner/process cumulative checks. The 600/3000
strict round passed owner 0.53423%, failed process 4.75882% and remains invalid.
Its custom/Microsoft same-trace process difference 0.0003502% never overrode that
OS failure. New prospective policy and valid new smoke do not reclassify old runs.

The old external game diagnostic's roughly 1.56% aggregate hid signed per-thread
opposition. Three predeclared long continuous/burst pairs subsequently completed:
continuous process signed errors +0.2859/+0.5298/+0.1883%; burst
-11.4130/-13.7380/-18.7441%. All burst coarse flags remain failures, with complete
coverage and matched hash work. The earlier one-worker quick burst overcounted,
opposite these long undercounts. No universal correction factor is inferred.

## Same-workload constant counters; no time-gain claim

Actual a2a8fce ON/OFF 30-frame diagnostics match input SHA
`3581acb812ff507d1d371b9d578eab2d2e065d097ee9221e9db691681f965e50`.
Autonomous animation means input_unchanged=false; paired endpoint/trajectory
identity matches. All 12 per-frame management delta counters match; the 13th
cache-stock slot stays 118 on both sides and is not a per-frame unique count.

Batch ON creates 7,503 successful nonempty CPU backing owners/419,244 bytes,
enters pool locks 12,296 times and scans 2,311,395 idle entries; OFF creates none,
enters 23,040 times and scans 777,150. End idle stocks are 1,024/248. These are
scoped backing counters, not malloc/Vulkan allocations or exclusive CPU times.
Fewer lock entries do not establish a benefit while allocation/scanning grows.
The demand-driven follow-up is committed in 7c42f3a; targeted consumers pass,
but current all-target checks and fresh real bills/performance remain open.

Earlier reflection 777→246, VAO query 190→128 and wrapper 63→0 changes are also
path counts. Prepared draws 361, constant takes 384 and writes 19,636 B are retained;
none is a whole-frame speedup or proof that all resource work is unchanged.

## Correctness and scene gates

c81975c padding tools pass 15; f734239 explicit native view usage/cache identity
passes native 3/range 1/full Rhapsody 458 with 2 ignored, strict logs without VUID.
Ignored tests never count as passed. Earlier Graph/core and Country successes
retain their own snapshots and do not verify the newer committed demand-plan source.

The new tiny full probe run is 306 passed/1 failed/5 ignored, so the suite fails.
Related pool 14/host 4/recorder successes do not erase it. Rollercoaster case4327
also fails in a pre-change EXE with unchanged source/fixture; retain its assertion
and treat platform floating precision as a separate open contract. This full run
also exposes old mapped-raster VkEvent VUID 01158/09543. The legal cfg(test) timeline correction is committed in 370f795 and its exact
strict test passed 1/0/0 without VUID. The fresh complete direct-EXE rerun also
has zero VUID, but still fails the unchanged rollercoaster assertion. Production
bytes outside the test module are unchanged; the full suite is not passed.

Country's initial E0507 test-borrow compile failure is retained. After the clone
fix, pass_builder passed 168; the initial draw-pass retry had 264 passed/1 failed, including the new
cross-pass tagged-slices test. The missing preset 0x11/0x12 setup was corrected in the new test fixture,
following the existing Rig without changing production or assertions. Fresh cross-pass exact 1 and complete draw-pass 265 retries passed, with
0 failures and 0 ignored. This does not make the tiny full suite pass.

The original lake inline fixture actually ran about 166 s and returned test success,
but packed-sampling-view VUID made the controller correctly invalidate the run.
Collect never started and no pair comparison exists. Representative water/ice ROI
is visible; valid same-scene pixel equivalence still needs new frozen artifacts
and actual reruns after the targeted view fix.

An isolated 24369d5c baseline is prepared in staging: exact 29095eb production
source plus QPC measurement brackets/7 tools, not yet built. Next gates are current
correctness recovery, final source/binary freezing, valid water/ice comparison,
Release/probe overhead, four scenes and final AB/BA/AB with p95 ≤16 ms/no >5% p95/p99
regression, plus pressure/fault/long runs. WSI compatibility is user-deferred;
macOS compile/runtime remains unverified. No remote push has occurred.

Latest fixture retries passed: .perf/constant-demand-cross-pass-fixed.log exact 1, and .perf/constant-demand-country-draw-passes-fixed.log 265/0/0. Earlier E0507 and 264/1 failure logs remain historical failures; tiny full and current all-target checks remain unaccepted.

## 最后一次冻结前更新：7c42f3a /370f795

需求计划已提交；timeline 修复仅在 cfg(test) 模块内，生产字节不变。
[mapped timeline 严格定向](C:/game/.perf/mapped-raster-timeline-strict.log) exact 1 pass/0 fail/0 ignored/0 VUID。
[修复后完整 direct-EXE probe lib](C:/game/.perf/constant-demand-tiny-lib-probe-strict-after-timeline.log)
为 306 pass/1 旧 rollercoaster fail/5 ignored，**整库仍失败**。只读按
`VUID-|Validation Error:|\bVALIDATION_ERROR\b` 复核实际日志为零验证错误；此前 wrapper
宽泛匹配普通 `producer_validation_error` 测试名称造成的异常不是新 VUID，也不能把真实断言失败改成通过。
[default 全目标](C:/game/.perf/constant-demand-workspace-default-check.log) /
[probe 全目标](C:/game/.perf/constant-demand-workspace-probe-check.log) 正在运行，均 pending。
本批结束后停止实际 docs 写入，供 root 提交/冻结；后续新证据先 staging。

## 全目标检查更新

`370f795` 源码的默认工作区全目标检查通过（2m42s），独立 `tiny-glade/cpu-probe` 全目标检查通过（36.39s），日志分别为 `.perf/constant-demand-workspace-default-check.log`、`.perf/constant-demand-workspace-probe-check.log`。该检查不替代真实帧账单、水/冰像素或 Release 性能验收。
