# 3e3b29c：测量、常量与真实场景证据快照

本文只记录已发生的验证，不代表 Release 性能或最终验收通过。当前提交
`370f7959e0a9a47d15f84477760f4a01147680fb`；本摘要沿用原文件名，新增需求计划 `7c42f3a` 与仅 cfg(test) timeline `370f795` 的已提交/验证证据。

## 新的前瞻时钟质量轮

[calibrated-clock-smoke-a2a8fce-1](C:/game/.perf/headless-runs/calibrated-clock-smoke-a2a8fce-1/run.json)
使用已冻结的 a2a8fce debug EXE、600 预热/600 samples、601 executed frames。
前瞻 `windows-etw-qpc-calibrated-v1` policy 在 child 启动前绑定，run/account 均为 valid=true。

| 检查 | 真实结果 |
|---|---:|
| owner ETW 对 GetThreadTimes | 1.2780%，通过 2% |
| 进程 ETW 对 GetProcessTimes | 0.3694%，原始差异保留 |
| 自算进程对 Microsoft CSwitch | 0.0010603%，同事件流的独立实现 |
| TID 生命周期 | 16 个，覆盖完整 |
| CSV 无损归档 | 2,010,015,522 →246,037,246 bytes，roundtrip_verified=true，ETL 保留 |

归档值来自已完成的小 run manifest，未重新读取/哈希巨大 ETL/CSV。
Microsoft 对照不是独立时钟。headless App/update 与 renderer 共用 owner；其他线程
包含 worker、FMOD、驱动等，不能统称 worker。这一 debug 质量轮不通过 Release、
16ms、feature-off 均值影响 ≤1%、四场景 600/3000 或 WSI 门槛。
旧严格轮仍 invalid（长轮 owner 0.53423%/process 4.75882%），不追认、不覆盖失败。

正式 policy evidence SHA 为
`6a3274f4d8753d30b04348a701804bf4a796788c5ab11b11cbd140ce467330ae`。
`3e3b29c` 已提交可移植校准 helper，24 项实际测试通过、0 skip；新机器用新声明及新六轮。
旧 helper/calibration/evidence 原样保留，不能把新源码身份冒充旧实跑。

## 常量批量路径的真实计数对照

[ON](C:/game/.perf/headless-runs/ownership-bill-alloc-on-1/diagnostic.json) /
[OFF](C:/game/.perf/headless-runs/ownership-bill-alloc-off-1/diagnostic.json) 均为 a2a8fce 的
30 个差分帧。两侧 input SHA 均为
`3581acb812ff507d1d371b9d578eab2d2e065d097ee9221e9db691681f965e50`。
动物继续动画，input_unchanged=false，但两侧起止/轨迹输入身份一致。

| 30 帧账单 | 常量批量 ON | 常量批量 OFF |
|---|---:|---:|
| 新成功非空 CPU backing | 7,503 | 0 |
| 新 CPU backing bytes | 419,244 | 0 |
| pool lock entries | 12,296 | 23,040 |
| pool scanned idle entries | 2,311,395 | 777,150 |
| 结束时 idle blocks（存量） | 1,024 | 248 |

这不是 malloc 调用或 Vulkan/native 分配计数。当前批量路径减少锁次数，同时增加
backing 创建和扫描；没有时间收益结论，不能仅凭一个改善指标默认放行。
新的需求驱动预领计划正针对这些额外管理工作，已在 `7c42f3a` 提交；新真实账单/性能仍待验证。

两侧 12 个逐帧差分计数相同：reflection query/hit 246/246、constant takes 384、
collected writes 19,636B、resource inputs 1,301、prepared draws 361、VAO full refresh 2、
list entries 344、queried entities 128、unique visits 128、publications 3、new wrappers 0。
第 13 个原始槽是 cache 存量，两侧始末/最大均为 118，不是逐帧独立 variant 数。
较早的 777→246、190→128、63→0 也只是计数变化，不能当 CPU/FPS 收益。

## 正确性验证与保留失败

- c81975c 的 padding 工具 15 项通过；capture 尺寸与 renderer 对齐后的 inner extent 分开校验。
- f734239 修复显式 native image view usage/attachment cache 身份：native 3、range 1、
  full Rhapsody 458 pass/2 ignored，严格 Vulkan 日志无 VUID；ignored 不算通过。
- 需求计划下 tiny full probe 为 306 pass/1 fail/5 ignored，整库失败。相关 pool 14、host 4、
  recorder 成功不能覆盖整库失败。rollercoaster case4327 的 1 ULP 差异在旧 EXE 同样失败，
  源码/fixture 与 29095eb 相同；保留原断言，列为既有浮点复现缺口，而非本轮性能收益。
- 同轮旧 mapped_raster_gpu_tests 触发 VkEvent VUID 01158/09543；合法 timeline 的 cfg(test) 夹具已在 370f795 提交，exact 1/0/0 严格零 VUID；生产模块外字节相同，
  不称 strict 全部通过，不关闭验证或放宽错误检查。
- Country 首轮 E0507 test-borrow 编译失败保留；clone 修正后 pass_builder 168 pass/0 ignored。
  较早 draw-passes 为 264 pass/1 fail/0 ignored，失败项是 cross_pass_constant_plans_preserve_tagged_slices_across_job_and_wave_boundaries；
  失败日志保留。补齐 bind_common 所需 preset 0x11/0x12 的新增测试夹具后，cross-pass exact 1 与全 draw-passes 265 均通过、0 ignored，未改生产或断言。历史 Country 166/1/262 只属于其旧快照。

## 水/冰真实运行与下一步

[lake-correctness-a2a8fce-1/pair.json](C:/game/.perf/surface-runs/lake-correctness-a2a8fce-1/pair.json)
中 inline 真实运行约 166s、test returncode=0，但 packed sampling view 的 VUID 使 gate 正确
判为 invalid；Collect 未启动，comparisons 为空。ROI 已看见代表性水/冰，尚无有效 pair/pixel 结论。
f734239 的定向修复成功不能替代重新冻结和实际原场景重跑。

隔离 baseline `24369d5c` 已在
[windows-baseline-29095eb-brackets](C:/Users/liyu/.codex/visualizations/2026/10/26/01a17714-b237-7ed3-8e87-7aeaa53c4a51/windows-baseline-29095eb-brackets)
准备：生产源码精确 29095eb，仅加测量 QPC brackets/7 tools，尚未构建或验收。
接下来先修复并复验当前错误，冻结新 baseline/candidate，跑水/冰有效严格对照、
Release/probe 开关、四场景与长跑。WSI 按用户要求暂缓，macOS 待验。
全部请求完成后才允许按保留真实 ancestry 的策略发布新分支；仍未 push。

新增测试夹具补齐 bind_common 必需 preset 0x11/0x12 后，[cross-pass 复验](C:/game/.perf/constant-demand-cross-pass-fixed.log) exact 1 pass/0 ignored，[draw-passes 全套复验](C:/game/.perf/constant-demand-country-draw-passes-fixed.log) 265 pass/0 fail/0 ignored；builder 168 pass 保持。生产/断言未改，旧失败仍保留；不代表 tiny full 或当前全目标通过。

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
