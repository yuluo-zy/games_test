# 当前进度与未完成任务（2026-10-26）

**已取得一轮有效的前瞻时钟 debug 质量验证；Release 性能、常量新方案和 Collect 完整验收仍未完成。**
不能报告整帧提升或 16ms 达标。当前已提交至 `370f795`：需求计划 `7c42f3a` 与仅 cfg(test) timeline 夹具 `370f795` 均已提交；新默认/probe 全目标检查仍 pending。
完整新证据和原始链接见 [本批证据摘要](C:/game/docs/performance/evidence-update-3e3b29c-20261026.md)，
精确机器状态见 [status.json](C:/game/docs/performance/status.json)。

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
