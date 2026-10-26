# 当前进度与未完成任务（最新证据更新：2026-10-27）

## 最新进展：2026-10-27 / native f39dc61；engineering fa98f8d

此次 native 验证 checkpoint 是 clean `f39dc61eef85edebda2f491b4d31d9ff9e85c144`，当前工程 HEAD 已为 `fa98f8d4367cc6ce24fd5d7a5e03cc82b36008d7`，不能把 f39 证据扩大到后续改动。最终 Tiny/workspace 类型检查或全部原生验收仍未完成。

- 已通过范围：clean 6d1cf8a 的 Rhapsody observer 2 tests、有效且清空的 private Job；canonical getters 之前集成 Rhapsody cpu-probe/default all-target 检查 30.87s / 29.91s；之后 clean f39 同两种 Rhapsody all-target 类型检查 31.52s / 31.00s 也通过，但不覆盖 Tiny、country-core、workspace 或新原生执行。Python 63 tests / 13.846s。旧 Job 清理失败保留。
- 编译阻塞：Tiny observer build 申请 14,942,224 bytes 失败，最新 Tiny cpu/surface metadata 检查在 country-core 申请 262,144 bytes 失败；均未运行 GPU，也没有最终类型检查通过结论。
- 待运行：canonical 5 tests、local surface Rust 6 tests、pending-acquire 9 tests，以及未执行的 Tiny observer 2 tests。pending9 libtest 编译申请 2,097,152 bytes 失败、artifact/test_exit=null，没执行 tests/GPU。suballocation Drop 2 tests 已原生运行：round1 断言通过但 Job drain 超时整体无效，cached round2 同 EXE/PDB、2 passed / 0 ignored、自然空 Job、valid=true。默认 unused Drop 仍有 1 次/64B Rust 分配，fragmented reclaim 33 次/2,640B；不据此声称无分配或 CPU 收益。先解除资源约束再串行补验；ignored/零命中不计通过。
- c98 严格水冰像素仍失败。记录输入/常量/history/order 相等，但 Nani DEVICE_LOCAL GPU bytes 不可读，不能宣称 GPU 输入全等。新 canonical / 同前缀定位不得改判旧失败。
- c98 trace 的 502 bricks 分为 OFF 3 段 / ON 4 段，两个 Wall pass 各 92 scopes 解释 184 次 dispatch 差值；111 个其他 pass 分布与 Nani 不变。无 receipt join，raw CPU 两轮仍无效，未取得收益。14 行 graph-command 与 14 行 graph-chunk 差异仍保留。
- `b0c7a3d` 工具 Python 135/44 tests、`fa98f8d` surface LF 修复后 94 tests 通过；旧 91 tests / 2 ERROR 另保留。工具安装不是 common4 隔离基线激活或新 binary/GPU 通过。`c1dd0b3` EffectsHost history-valid 极性修复另增 1 个 8-step native 生命周期测试，只有源审/语法解析，未 Tiny 类型检查或 GPU，加入待验清单。
- fatal-OOM/automatic retry 未应用，Painterly Ready/progress、menu/LUT 事务仍开放。阶段 5 完全有界、Release ETW/≤1% 开销、四场景 600/3000 AB/BA/AB、16ms、严格像素、压力长跑、macOS 和用户暂缓 WSI 继续开放。

控制会话早期 commit 1.4–1.8GiB 和后续 4,824,317,952 bytes 样本作为历史保留；最新 reader 样本仅 1,208,508,416 bytes（约 1.13GiB），主线程已停止进一步 native/重 Tiny 构建。immediate Job 无配置内存限额不代表所有祖先，也不能当失败 compiler peak。请用户手动关闭不用的高内存程序、释放约 8GB 可提交内存的问题待答复，没有请求或应用 pagefile 更改，未终止无关应用或推送。OOM [设计说明](C:/game/docs/performance/recoverable-compile-oom-design-20261027.txt)/[源锚点](C:/game/docs/performance/recoverable-compile-oom-source-anchors-20261027.json)只是设计，不是恢复实现。详细范围及 SHA 见[本次证据](C:/game/docs/performance/evidence-update-f39dc61-20261027.md)和 [status](C:/game/docs/performance/status.json) 的 `latest_progress_20261027_f39dc61`。

## 前一轮进度快照（原文保留）


## 最新进展：2026-10-27 / c98be8e

**最新 checkpoint：c98be8e。Native Collect 真实恢复夹具已通过；新 trace/Wall/Nani 定向检查通过，正式 CPU 收益、16ms 与严格水冰像素仍未验收。**

- [Native Collect 隔离运行](C:/game/.perf/native-collect-exact-validation-3/result.json) `valid=true`，exact 1 pass / 0 ignored，0.87s、零 VUID、源码前后不变；测试源码 SHA256 `b872baddc9db36a1be84fde46a9213c9f05b6379c4aaf038ab29f043314f7ed3`。579 个真实 worker draws、12 tasks / 2 waves，A 在途时取消未提交 B，冷计划真实预算拒收在 worker 前发生，恢复后完整像素等于参考。这是指定原生夹具验证，不能代替代表性草/UI/水冰场景正确性。
- 本批 Rust 定向检查分别为 trace 6、Wall source 3、Nani observer 3；Python parser 8、trace-source controller 9，均实际执行通过。Nani 的 3 个 CPU 测试没有运行水冰 GPU 夹具；范围观察不能冒充 native dispatch 或完整 GPU 输入证据。
- [rich debug 冻结](C:/game/.perf/headless-builds/rich-workload-debug-c98be8e/build.json)：clean `c98be8e`，源码内容 `3ba1628503802a31bc83e90850aa9729d80a8d8b43491d430fd58f71e8f5d855`，EXE `064462bf22ced894f19848bf7dffc81f726cde663e9fb8f5207847450141092a`，PDB `b35671b6d6c92c53651119102ac1aa23559909971ad1f4931b5cdc4513772f8f`。requested target 已完成；整个 Cargo invocation 未通过/未被接受，不能扩称全工程验证。
- [rich-workload-debug-c98be8e-trace-1](C:/game/.perf/binding-trace-source/rich-workload-debug-c98be8e-trace-1/summary.json) 同 EXE binding OFF/ON 60 warmup / 30 samples 已完成：诊断结构有效，两轮 trace 有效，但两个 raw run 均 `valid=false`，不具 CPU 验收资格。31 行 native 差异均为本轮 OFF 355 / ON 357 direct dispatch；完整 actual 有 59 项差异、Wall source 有 1,333 项差异，二者均不相等。方向与旧 OFF 357 / ON 355 pair 相反；源输入差异是调查发现，不能据此归因 binding 或声称 CPU 收益。trace 是录制 scope，Wall 是 prepare 前源输入；不建立不存在的提交 receipt join，也不改判旧工作量失败。
- [baseline Release 导入](C:/game/.perf/formal-baseline-import-000e7cc-execute.log)已完成，source `000e7cc8abd332d7508480c1d6384a3d64b9e6a9`、EXE `483998fb7ef50b0a4d5e1563f8cb97a02c1ee5e25556ec8eb343d7b99f52ec9d`；candidate Release 尚待构建冻结。baseline/candidate common 同步的工作量 helper SHA256 为 `3890c544d3f79c2d8a542acf064d70ef66cdae351a5b33d297d6bd2b6359eae4`（原补丁 4107fbf，baseline 000e7cc，candidate common 6612）。portable importer 26 与 series/import seal 88 tests 分别通过；旧失败日志保留，导入通过不是性能结果。

最新诊断改动之后尚未重跑 default/probe workspace 全目标检查；下方旧通过记录仅覆盖各自源码。Release 600/3000 ETW、探针开销 ≤1%、四场景 AB/BA/AB、全路径成本转移审查、严格水冰像素和完整压力/长跑仍待验。用户暂缓的 WSI 兼容、macOS 与旧浮点失败保持开放；尚未推送远程。

当前依赖顺序：完成本次 trace/source 定位并保留全部差异 → 相同 common/helper 下冻结 candidate Release → 有效 ETW/开销闭环 → 四场景同工作量正式对照 → 代表性像素、完整压力与长跑 → 达标后发布。任何诊断先行通过均不越过后续门槛。

## 前一轮进度快照（原文保留，以上方最新状态为准）

**短程真实工作量与独立GPU诊断已实跑；最新CPU smoke没有ETW，严格水冰像素仍失败，Release收益与16ms未验收。**
证据见[测量补线与真实常量账单](C:/game/docs/performance/evidence-update-workload-probe-20261026.md)。新binding实现仍是实验候选，不能从旧PROFILE热点或锁次数减少宣布CPU收益。

## 最新原始运行证据

- `workload-clean-smoke-47bdd14-1`：debug，60帧预热/30个观察帧，session与actual_workload均有效；31行含seed，30观察实际720个Collect joined任务/630 waves，cross-pass90任务是subset。测试1 pass/0 ignored，但无ETW，run.valid=false，不能接受逐帧CPU/p95或探针开销。
- `workload-probe-47bdd14-1`：独立GPU诊断run/session/workload/GPU均valid；91帧、720时间戳，每帧样本数量等于该帧实际expected_commands，最终skips/pending/retained均0，VUID0。它不是干净CPU轮；命令跨度sum/max不是全帧GPU延迟，production16ms=false。
- `lake-full-constants-0c6bc69-1`：inline/Collect各1 pass/0 ignored/0 VUID；水、冰各30帧完整已记录input、1060个u32 FrameConstants和pass调用次数相等。但水20/30帧分别改变89,205/169,503像素，最大通道差3/7；冰6,958/10,890，最大差3/5，严格RGBA失败。完整CPU常量相等不证明所有GPU输入/历史相等。

## 当前依赖队列

1. 保留最新严格像素失败，定位最早分歧的GPU输入/历史/阶段；用同模式重复与冷启动/另行声明的稳态诊断区分问题，零容差不变。不能用两个测试进程exit0替代像素pair通过。
2. 审查binding scratch/单entry快照实验的真实owner、预算/回收、取消/错误与顺序；验证完整后再冻结对照。无收益验证前不把该候选写成默认优化完成。
3. 隔离旧生产源码只移植相同测量探针，使用同字节common3 harness与各source自己的compiled defaults；完成source/EXE/PDB/工具/输入身份和Release冻结。旧debug/new短smoke均不能替代正式600预热/3000有效ETW样本及probe feature-off<=1%。
4. 四场景固定1280×720、scale1、1/60和原画质，按AB/BA/AB开展三对独立比较；完整语义/轨迹/实际native draw API与pass、H2D/D2D payload分别核验，任务/录制成本变化单列调查，不能靠成本转移满足指标。近阈值扩测规则须预先声明。
5. 四场景每轮render CPU p95<=16ms，p95/p99回退>5%拒绝；同时复核App、owner、全部process/non-owner关键路径。完整上传与indirect参数证据、source审查capabilities及像素正确性未闭环时保持待验。
6. 完整压力/故障，以及稳定负载、效果切换、resize各至少30分钟且10000帧的稳定性仍待验。既有rollercoaster浮点断言保持失败边界，WSI兼容按用户指示暂缓，macOS待验。
7. 全部要求达成后才发布新远程分支；当前没有推送。保留原历史和所有失败，不以局部测试通过宣称整轮优化完成。

## 历史进度快照（以下旧pending不覆盖上方最新证据）

下面保留此前完整文本及原始失败，供源码与证据追溯；其中“待实跑/正在检查”等是原冻结时状态，不是最新状态。

# 当前进度与未完成任务（2026-10-26）

**当前测量源码已提交至c7f4dc7，默认/probe全目标检查通过；Release性能与16ms仍未验收。**
最新证据见[测量补线与真实常量账单](C:/game/docs/performance/evidence-update-workload-probe-20261026.md)。下面历史段落保留各自源码与原失败，不能用旧pending覆盖最新结果，也不能把局部通过当最终验收。

当前优先队列：新冻结EXE的真实计数/GPU闭环 → 绑定scratch/快照定向对照 → 相同common harness的Release探针开销与四场景AB → 完整压力与长跑。水冰同模式漂移仍需定位，原有浮点断言、WSI用户暂缓、macOS待验。远程尚未推送。

## 本轮真实进展

1. **测量质量**：a2a8fce debug、600 warm/600 samples/601 executed 的前瞻 policy 轮 valid=true。
   owner OS 差异 1.2780%、process 0.3694%、Microsoft process 0.0010603%，16 TID 生命周期完整；
   CSV 经 roundtrip 校验无损压缩。旧 600/3000 strict 轮 process 4.75882% 失败仍为 invalid。
2. **管理计数与需求计划**：常量 on/off 有同输入/轨迹的真实 30 帧计数；ON 少 lock，却有 7,503 次
   新 backing/419,244B、2,311,395 次扫描，OFF 新 backing 0/扫描 777,150。没有时间收益结论。
   针对此问题的需求驱动预领计划已在 `7c42f3a` 提交；定向消费者通过后，仍须当前全目标检查与新真实账单复验。
3. **native view 与 padding**：f734239 的 native 3/range 1/full Rhapsody 458+2 ignored 严格无 VUID；
   c81975c padding 工具 15 pass。3e3b29c 可移植校准包 24 pass/0 skip，新机器必须新六轮。
4. **失败不掩盖**：tiny full probe 306 pass/1 fail/5 ignored，最新 timeline 修复后同整库实跑零 VUID，但仍因旧 rollercoaster 失败。
   rollercoaster case4327 在旧 EXE 也失败，保持原断言，单列既有浮点复现缺口；整库仍失败。
   Country 修正新增测试夹具后 pass_builder 168、cross-pass exact 1、draw-passes 265 均通过且 0 ignored；tiny full/当前全目标仍未放行。
5. **真实水冰**：inline 原湖场景实际跑完但 VUID 导致 invalid，Collect 未启动；ROI 有真实水/冰，
   尚无有效像素 pair。修复后的定向用例不能替代原场景重跑。

## 已落地的源码候选及验收边界

| 项目 | 当前状态 |
|---|---|
| 图版本查询 | 50d8885 committed current_version 索引；旧定向查询收益仅局部诊断，非帧收益 |
| 所有权缩锁/retire/reclaim | b792e4c/c5171f3/67ddafa 已提交；真实 owner、预算、预留退役节点语义保留；完整持续压力待验 |
| 反射共享 | b79443e scope 不可变 Arc/CPU backing 准入，单 draw 仍可走原 resolver；新账单每帧 246 query/hit，未证明 CPU 收益 |
| VAO 查询/包装 | cca55ad/6ecdb2f typed lookup 与同帧 wrapper 复用，真实 generation/owner 重验；计数 128 queried/0 wrappers，仍需四场景时间对照 |
| 常量与 scratch | 709d717/a36fbd3 已提交；真实 on/off 暴露库存扫描/新 backing 问题；需求驱动方案 7c42f3a 已提交，当前全目标/新账单/性能待验，不因锁减少宣布优化成功 |
| view/FrameGlobals | 19c52eb/f734239 正确性修复；真实 native usage 与 cache 身份保留；370f795 仅 cfg(test) timeline 夹具已通过定向严格复验 |
| Collect | 1da8d40/fb778bd 草/UI/水冰分页、worker-local 收集/稳定 commit；core16/graph5 为历史检查快照成功，新需求计划 cross-pass/265 draw-passes 已通过，完整场景与全目标仍待验 |
| 真实水冰/校准交付 | fa90e4a 原场景夹具、6ff4b47 精确像素工具、c81975c 尺寸语义、3e3b29c portable helpers 已提交；原场景 pair 未验收 |

## 按依赖推进的开放项

1. 需求计划 cross-pass/265 draw-passes 已通过，继续公开消费者/当前全目标检查；合法 timeline cfg(test) 夹具已提交并 exact 1 pass/0 ignored/0 VUID，保持断言；
   复验严格 GPU；将既有 rollercoaster 精度缺口独立登记，不能通过放宽断言把 full suite 染绿。
2. 完成需求驱动常量库存方案的正确性/取消/预算恢复；重跑相同输入 on/off 账单，确认 backing、
   锁、扫描等完整成本，再决定默认路径。绑定模板/上传/COW/静态描述等仍先测后定。
3. 从隔离 24369d5c baseline 与最终候选冻结 Release EXE/PDB；完成有效 600/3000、探针开关 ≤1%、
   phase 成本账单与 source/binary/input 身份。debug 质量通过不替代这些门槛。
4. 重新冻结并跑严格的原湖 Autumn 水/Winter 冰 inline/Collect 相同场景像素与命令对照，
   包括真实取消、CPU/native 拒收与恢复；已有 invalid pair 原样保留。
5. 四场景 static、actual draw-list≥1000 many_draws、每30帧局部编辑、每帧连续编辑，固定1280×720、
   scale1、1/60、原画质；每轮600预热/3000有效帧，各三轮。起止/轨迹/draw/task/dirty/upload身份一致。
6. 最终 AB/BA/AB；四场景每轮 render CPU p95≤16ms，p95/p99回退>5%拒绝；
   压力/故障和三类30分钟且10000帧稳定性/窗口生命周期仍待验。WSI兼容用户暂缓，macOS待验。
7. 全部请求完成后再发布：保留8fd→695真实祖先，将b91之后变更映射tiny-glade/，保留RE/额外路径；
   不force覆盖源分支，最终新分支codex/perf-windows-20261026，尚未push。

## 历史证据与快照

旧53bbbef及以前的445 Rhapsody/55集成/260 draw-pass/30帧720 Collect/90 cross-pass是历史证据。
之后 Graph default/probe、core16/graph5、Rhapsody458与Country166/1/262成功也各有源码边界，
不能覆盖当前 dirty 需求计划。旧 check1/2/3、core14/2、OOM被UninitializedRead、fake queue-family VUID、
短/长 strict clock invalid、真实水冰 invalid、tiny full失败及Country失败均保留在status及原始.perf日志。
当前 docs 写完后暂停实际写入以稳定后续冻结；新资料先 staging。

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

### 最新附注：binding与压力回归（独立核验原始记录）

[binding系列](C:/game/.perf/binding-scope-diagnostics/binding-scope-debug-5767a40-static-1/summary.json) 绑定source `5767a4073d0adbb8c9bd2daee93f6e7930a25ca4`、debug EXE `4fdccdc45faeaffb53f1ebacb3ae1796ddaf3bdb432387b99c30f81332320397`。OFF/ON各60预热、30观察帧，session及actual_workload各自有效、前后输入轨迹跨两轮相等；但两轮均无ETW且为诊断，run无效，系列也因实际工作量不同而无效：31行direct_dispatch_api_calls逐行为357/355，row18 recording chunks为8/7。不能据此计算CPU收益。

排除row0后30帧账本差分：OFF/ON Vec容量增长事件13,530/720，增长后容量字节累计3,115,200/249,600，snapshot builds10,830/7,080、hits0/3,750；callbacks均840、builds+hits均10,830，failed/shape fallback均0。这里的容量累计不是malloc字节、峰值内存或CPU时长；只证明本次诊断少了相关事件，仍需相同真实工作量的正式对照。

[压力配额](C:/game/.perf/stage5-pressure-reclaim-strict.log) 3 pass、[退役预留](C:/game/.perf/stage5-retirement-reservation-strict.log) 7 pass，均0 failed/ignored，原日志未见VUID。这些是定向正确性回归，不能代替完整故障矩阵、长跑、像素一致性或Release四场景验收。所有旧失败保持原判。

最新工程checkpoint为`366311c`：binding候选已提交`5767a40`，压力/退役定向测试已提交`54120b6`，publication policy为`366311c`。Root报告本次两次检查范围为workspace --all-targets（default/cpu-probe）；独立核对[default日志](C:/game/.perf/binding-stage5-surface-workspace-default-check.log)和[enabled日志](C:/game/.perf/binding-stage5-surface-workspace-enabled-check.log)，均有dev-profile完成记录（2m43/2m41）且无编译error。[surface comparator v3](C:/game/.perf/surface-comparator-v3-tests.log) 实际20 tests通过、0 skip；源码接线编译已通过，新GPU复验尚未运行。工作量一致性修复与逐pass诊断仍在推进。编译/比较器单测通过不改变上述像素失败和无CPU收益结论。
