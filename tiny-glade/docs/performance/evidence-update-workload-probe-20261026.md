# 测量补线与真实常量账单（2026-10-26）

测量源码提交 `20de20d`；完整帧常量观测提交 `c7f4dc7`。Windows Release 收益、探针开销和最终 16ms 门槛仍未验收；没有推送远程。

## 最新真实运行（2026-10-27更新；原始运行UTC日期2026-10-26）

以下证据直接核对run/session/manifest、日志与原始RGBA，均保留冻结二进制身份；未启动新Cargo、GPU或性能运行。

**真实工作量短smoke，尚无CPU时间验收。** [run](C:/game/.perf/headless-runs/workload-clean-smoke-47bdd14-1/run.json) / [session](C:/game/.perf/headless-runs/workload-clean-smoke-47bdd14-1/session.json) 来自 `47bdd144bd786f9cc3d25fb7cd607a7ca02cc135` 的debug EXE，SHA256 `9a0b5c5e1b35abcd54d65c24f0a69f0311fefd10ad8040413ef95c7450c5c568`。60帧预热、30观察、31原始行；session和actual_workload均valid、errors空、未截断。30观察内720个Collect joined任务/630 waves，cross-pass90任务为subset。测试1 pass/0 ignored、零VUID；但etw=false，runner.valid=false，原因原文为“No scheduling trace: diagnostic only; coarse CPU cannot establish frame percentiles”。不能把session.valid或clean_cpu eligibility字段改写成CPU验收通过。

**独立GPU命令时间戳链路有效。** [GPU run](C:/game/.perf/gpu-diagnostics/workload-probe-47bdd14-1/run.json) / [session](C:/game/.perf/gpu-diagnostics/workload-probe-47bdd14-1/session.json) 使用同一EXE/PDB：run、session、actual_workload和gpu_diagnostic均valid。含预热及seed的91帧共720个原始timestamp样本，独立逐帧复核sample_frame_indices数量等于expected_commands，后者合计720；最终completed720，capacity_skips/pending/retained_samples均0。日志1 pass/0 ignored/0 VUID，clean_cpu_acceptance_eligible=false、production_16ms_acceptance=false。该轮证明本次短程GPU关联/排空有效；不是整帧GPU延迟、干净CPU、四场景或长跑验收。

**完整常量相同，严格水冰像素仍失败。** [pair](C:/game/.perf/surface-runs/lake-full-constants-0c6bc69-1/pair.json) 绑定source `0c6bc6956993292a03a6e216cab1f402ad48a16a`，EXE SHA256 `3dd4530adf390523157c84eaae3678af2280df984018056ac862f31e213325e5`。两个孩子各1 pass/0 ignored/0 VUID，Collect每季实际30 worker calls/30 tasks。四个manifest中，水和冰各30帧的完整已记录input字典、1060个u32 FrameConstants及pass_call_counts逐帧相等。独立重读八个RGBA文件并核SHA256/逐像素统计，结果与原报告一致：

| 场景 | 帧20改变像素/最大通道差 | 帧30改变像素/最大通道差 | 严格结果 |
|---|---:|---:|---|
| 水 | 89,205 / 3 | 169,503 / 7 | 失败 |
| 冰 | 6,958 / 3 | 10,890 / 5 | 失败 |

[water](C:/game/.perf/surface-runs/lake-full-constants-0c6bc69-1/water-compare.json) / [ice](C:/game/.perf/surface-runs/lake-full-constants-0c6bc69-1/ice-compare.json) 的passes=false、exact_pixels_equal=false。input_mismatches中只有frame20/30_fixture_digest_declarations_differ，即输出像素的fixture摘要声明不同；不能据此新增“CPU常量不等”结论。完整CPU常量相等也不证明所有GPU输入、历史或其他未观测状态相等，更不能证明Collect无差异。早先592f655相同模式漂移与所有原始失败均保持原判，零容差不变。

最新短smoke没有提供Release600/3000、probe<=1%、四场景AB或整帧收益证据。binding scratch/单entry快照复用当前是实验候选，仍需正确性/原生owner/预算/取消/顺序检查及相同输入干净CPU对照；旧PROFILE热点或局部计数减少不代表该候选已经有收益。

## 相同输入的常量管理成本

四轮 debug 诊断都使用相同输入轨迹 SHA256 `3581acb812ff507d1d371b9d578eab2d2e065d097ee9221e9db691681f965e50`，预热60帧，31次累计观测形成30个增量。冻结 EXE/PDB、源码与原始报告 SHA 见 [机器可读账单](C:/game/docs/performance/constant-demand-592f655.json)。它们不是有效的干净 CPU 时间轮。

| 30帧累计增量 | 旧批量 ON | 旧 OFF | 需求计划 ON | 当前 OFF |
|---|---:|---:|---:|---:|
| 成功新增 backing | 7,503 | 0 | 0 | 0 |
| 新 backing 容量字节 | 419,244 | 0 | 0 | 0 |
| 锁次数 | 12,296 | 23,040 | 1,590 | 23,040 |
| 空闲项扫描 | 2,311,395 | 777,150 | 73,538 | 777,886 |

需求计划 ON 真实执行720个已规划任务，未规划任务和预热后计划缺口均0，idle inventory 262块/14,668B保持稳定。OFF未准备需求计划。全部四轮每帧均为246次反射查询、384次常量领取、19,636B collected uniform写入、1,301个绑定输入、361次bindless准备、128个VAO entity查询与0个新wrapper。最后的118 reflection variants是库存 gauge，不能当每帧资源数量。

backing计数只统计常量池成功的非空CpuBytes物理capacity创建，不是所有malloc尝试、native usable bytes或峰值内存。扫描/锁减少不能换算成整帧CPU百分比。

## 新测量链及验证

native API调用、图录制任务、Collect joined任务、queued buffer真实submit及四类设备上传各自保留身份与作用域。图内staging H2D、immutable producer H2D、缓存D2D分列。错误、部分提交、容量溢出、身份回退及缺失关联均无效；真实native提交后的后续失败仍保留已执行prefix。mapped CPU writes、D2H和clear/fill不属于这些GPU H2D copy字节，不假称所有逻辑上传总量。

新增数据均为feature-only的固定Copy观测，结束后序列化；GPU时间戳是独立诊断，保留同完成点的多个命令样本。命令跨度的sum/max不是全帧GPU延迟。六份已校准运行工具保持原SHA。

- [默认全目标检查](C:/game/.perf/workload-probe-workspace-default-check.log)：通过，2m45s。
- [cpu-probe全目标检查](C:/game/.perf/workload-probe-workspace-enabled-check.log)：通过，2m39s。
- [严格Rhapsody库](C:/game/.perf/workload-probe-rhapsody-lib-strict.log)：473 pass、2 ignored，零VUID；ignored不计为通过。
- [执行与状态消费者](C:/game/.perf/workload-probe-rhapsody-consumers-fixed-strict.log)：27+1 pass、0 ignored。新Partial夹具补真实依赖及第二次提交失败；旧缓存测试保留每帧精确成员、次数和帧顺序，仅移除无依赖回调的到达顺序假设。原失败日志保留。
- [Collect计数](C:/game/.perf/workload-probe-country-collect-counters.log)：8 pass；[draw-passes](C:/game/.perf/workload-probe-country-draw-passes.log)：264 pass。此前265轮启用了test-support，本轮少的唯一项是该feature内的surface scoped reference，不把过滤缺项计为通过。
- [真实fixture关联单测](C:/game/.perf/workload-probe-fixture-unit-tests.log)：8 pass；[Tiny宿主](C:/game/.perf/workload-probe-tiny-host-tests.log)：4 pass，均0 ignored。
- [A/B工具](C:/game/.perf/workload-probe-series-tool-tests.log)：71 pass、0 skip。GPU控制器当前24 pass。工具测试不替代真实运行。

## 水冰仍未验收

`lake-correctness-592f655-1`两个孩子各1 pass/0 ignored/0 VUID，Collect在每季实际发生30次worker调用/30 draws/30 tasks。严格输入及原始RGBA比较失败，不能放行。另 `lake-inline-repeat-592f655-1` 同模式重复也在两季复现了像素漂移，且所有已记录语义输入相同：水20/30帧改变4,363/92,796像素，最大通道差2/5；冰5,862/5,937，最大差5/4。

因此原pair差异不能单独归咎Collect。PGO名字来自哈希表，仅应比较排序多重集而非冒充命令顺序。新夹具保存完整0x1090常量的1060个u32 bit及实际pass调用次数，未改场景、Shader、等待策略或零容差。此前完整常量EXE待实跑的状态已由上方0c6bc69证据更新：记录的完整常量相等但像素仍失败，其他GPU输入/历史仍未证明一致。旧30帧是GPU冷启动；90次App update没有推进GPU历史。

## 后续

隔离旧Windows生产源码仅移植同测量探针，已独立审查并提交 `ef5568b`，未带候选反射/VAO/常量/scratch优化，未构建。formal baseline/candidate必须使用同字节common3 harness，保留各自编译源码默认算法，不能伪造相同runtime开关。

独立PROFILE已留下真实ETL与本地符号导出，汇总脚本失败和原strict时钟无效记录均保留；离线恢复只能支持定位。当时导出绑定热点实现与检查源码逐字相同，支持研究callback资源scratch和单entry快照复用；这些方向当前进入实验候选，未有可比干净CPU收益或最终放行。COW/view缓存仍缺强证据，不能把PROFILE定位写成默认优化已完成。

继续按新冻结EXE真实工作量/GPU闭环、Release探针开销、四场景交替A/B、压力与长跑推进。完整上传/间接绘制参数证明、原有rollercoaster浮点断言、真实窗口WSI和macOS等开放项不因局部通过而消失。只有满足用户完成条件后才发布远程新分支。

### 最新附注：binding与压力回归（独立核验原始记录）

[binding系列](C:/game/.perf/binding-scope-diagnostics/binding-scope-debug-5767a40-static-1/summary.json) 绑定source `5767a4073d0adbb8c9bd2daee93f6e7930a25ca4`、debug EXE `4fdccdc45faeaffb53f1ebacb3ae1796ddaf3bdb432387b99c30f81332320397`。OFF/ON各60预热、30观察帧，session及actual_workload各自有效、前后输入轨迹跨两轮相等；但两轮均无ETW且为诊断，run无效，系列也因实际工作量不同而无效：31行direct_dispatch_api_calls逐行为357/355，row18 recording chunks为8/7。不能据此计算CPU收益。

排除row0后30帧账本差分：OFF/ON Vec容量增长事件13,530/720，增长后容量字节累计3,115,200/249,600，snapshot builds10,830/7,080、hits0/3,750；callbacks均840、builds+hits均10,830，failed/shape fallback均0。这里的容量累计不是malloc字节、峰值内存或CPU时长；只证明本次诊断少了相关事件，仍需相同真实工作量的正式对照。

[压力配额](C:/game/.perf/stage5-pressure-reclaim-strict.log) 3 pass、[退役预留](C:/game/.perf/stage5-retirement-reservation-strict.log) 7 pass，均0 failed/ignored，原日志未见VUID。这些是定向正确性回归，不能代替完整故障矩阵、长跑、像素一致性或Release四场景验收。所有旧失败保持原判。

最新工程checkpoint为`366311c`：binding候选已提交`5767a40`，压力/退役定向测试已提交`54120b6`，publication policy为`366311c`。Root报告本次两次检查范围为workspace --all-targets（default/cpu-probe）；独立核对[default日志](C:/game/.perf/binding-stage5-surface-workspace-default-check.log)和[enabled日志](C:/game/.perf/binding-stage5-surface-workspace-enabled-check.log)，均有dev-profile完成记录（2m43/2m41）且无编译error。[surface comparator v3](C:/game/.perf/surface-comparator-v3-tests.log) 实际20 tests通过、0 skip；源码接线编译已通过，新GPU复验尚未运行。工作量一致性修复与逐pass诊断仍在推进。编译/比较器单测通过不改变上述像素失败和无CPU收益结论。
