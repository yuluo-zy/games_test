# 测量补线与真实常量账单（2026-10-26）

测量源码提交 `20de20d`；完整帧常量观测提交 `c7f4dc7`。Windows Release 收益、探针开销和最终 16ms 门槛仍未验收；没有推送远程。

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

因此原pair差异不能单独归咎Collect。PGO名字来自哈希表，仅应比较排序多重集而非冒充命令顺序。新夹具保存完整0x1090常量的1060个u32 bit及实际pass调用次数，未改场景、Shader、等待策略或零容差。新EXE和完整GPU输入仍待实跑。旧30帧是GPU冷启动；90次App update没有推进GPU历史。

## 后续

隔离旧Windows生产源码仅移植同测量探针，已独立审查并提交 `ef5568b`，未带候选反射/VAO/常量/scratch优化，未构建。formal baseline/candidate必须使用同字节common3 harness，保留各自编译源码默认算法，不能伪造相同runtime开关。

独立PROFILE已留下真实ETL与本地符号导出，汇总脚本失败和原strict时钟无效记录均保留；离线恢复只能支持定位。绑定热点实现与当前逐字相同，支持研究callback资源scratch和单entry快照复用；COW/view缓存证据弱，尚无默认路径改动或CPU收益结论。

继续按新冻结EXE真实工作量/GPU闭环、Release探针开销、四场景交替A/B、压力与长跑推进。完整上传/间接绘制参数证明、原有rollercoaster浮点断言、真实窗口WSI和macOS等开放项不因局部通过而消失。只有满足用户完成条件后才发布远程新分支。
