# Windows CPU 计数差异审计（2026-10-26）

本轮 `closure-etw-smoke` **不通过测量闭环验收**：ETW 与 `GetThreadTimes` / `GetProcessTimes` 的累计 CPU 差异均超过既定 2%。独立 Microsoft `xperf -a cswitch -util -thread -exc_dpcisr` 与自算 owner 时间高度一致，支持调度区间重建正确，但不能覆盖累计 OS 计数检查失败。16ms 性能门槛和 2% 测量门槛均保持不变。

这是现有真实 shipped-session headless 夹具的时钟审计。App 与渲染共用 owner；不等于生产窗口渲染线程验收，也未证明 WSI 可用或性能优化收益。实际本轮为预热 60 帧、300 个有效观察帧、301 个 envelope 执行帧；完整 v2 记录为 300 clean，无截断。使用冻结 `closure-base-on`、源码提交 `29095eb143a029eb32edef56176bc1b6e3577c77`，EXE SHA256 为 `7113e4960d6e972a8e9f258699eb29a043a1a8b718d5d488d390e22ac523fdeb`。

## 原始证据与身份

原始文件位于 [closure-etw-smoke](C:/game/.perf/headless-runs/closure-etw-smoke)。生成物保持在被忽略的 `.perf`，本摘要纳入版本管理。

| 文件 | SHA256 |
|---|---|
| [session.json](C:/game/.perf/headless-runs/closure-etw-smoke/session.json) | `cae13c9618d10df973061c44202447215094c23b13d3f01630f7bbb9e4f63c1d` |
| [schedule.csv](C:/game/.perf/headless-runs/closure-etw-smoke/schedule.csv) | `fb8398582a88b717d5e89e8129dc3b77944f3f2590882a3a824d3f478fcf6d24` |
| [schedule.etl](C:/game/.perf/headless-runs/closure-etw-smoke/schedule.etl) | `9d846ca22e61f49cb70d029984471c3f4e656ae7855f5e7c96bf7330f5ccf646` |
| [official-owner.csv](C:/game/.perf/headless-runs/closure-etw-smoke/official-owner.csv) | `20830ded808174f1c2fa41d1b8f8436eeb429d6849b7ae02e2548ae391f216d3` |
| [cpu.json](C:/game/.perf/headless-runs/closure-etw-smoke/cpu.json) | `ff52ac2ea99d12a988a2424d49d92538af16f4eb20967ac5c3ec51a9a8ba17f1` |
| [run.json](C:/game/.perf/headless-runs/closure-etw-smoke/run.json) | `2f86ebe099bf1a57657ff6319aaf9c0e70038b324031db98547cce08685a6513` |

CSV 为 582,297,686 字节。独立后处理逐行解析，未把整个 CSV 装入内存。脚本及报告暂存在当前任务输出目录：

- [audit_cpu_trace.py](C:/Users/liyu/.codex/visualizations/2026/10/26/01a17714-b237-7ed3-8e87-7aeaa53c4a51/audit_cpu_trace.py)。独立解析 CSwitch 状态机与迁移，复用现有确定性的区间 union/subtract/Timeline；算法同源的部分不能冒充第三方独立实现。
- [基础独立审计](C:/Users/liyu/.codex/visualizations/2026/10/26/01a17714-b237-7ed3-8e87-7aeaa53c4a51/closure-etw-smoke-independent-audit.json)。与当轮自算相同，只扣 `DPC`、`Interrupt`。
- [含 Timer DPC 的审计](C:/Users/liyu/.codex/visualizations/2026/10/26/01a17714-b237-7ed3-8e87-7aeaa53c4a51/closure-etw-smoke-with-timer-dpc-audit.json)。额外检查 `DPCTmr`，不改原始结果。
- [原始身份清单](C:/Users/liyu/.codex/visualizations/2026/10/26/01a17714-b237-7ed3-8e87-7aeaa53c4a51/closure-etw-smoke-evidence-hashes.json)。

## 调度重建与中断扣除

PID 7200，owner TID 32360，QPC 频率 10,000,000 Hz；envelope 为 `[2619726537812, 2620161414096]`，墙钟 43,487.6284ms。原始 ETW 严格包围 envelope，报告 Events Lost=0、Buffers lost=0。

| 检查 | 实际结果 |
|---|---:|
| CSwitch 事件（不含表头） | 1,171,721 |
| 普通 DPC / Interrupt 事件 | 35,275 / 23,583 |
| envelope 内所有 CPU 相邻 CSwitch 的 New→下一次 Old 不一致 | 0 |
| 每 CPU QPC 回退 | 0 |
| 目标进程在 envelope 内缺失或未闭合的运行区间 | 0 |
| owner 使用 CPU | 0–7，全部 8 核 |
| owner 闭合运行段 / CPU 迁移 | 32,436 / 2,344 |
| owner 同时在两个 CPU 运行的重叠 | 0 |
| 目标进程具有运行区间的线程 | 25 |

按每 CPU 对 DPC/ISR 区间取并集后扣除，避免嵌套中断重复扣除：

| 累计 CPU | owner | 整个目标进程 |
|---|---:|---:|
| CSwitch 原始运行时间 | 3,521.1550ms | 13,056.0209ms |
| 普通 DPC/ISR 扣除量 | 4.8208ms | 43.5962ms |
| 当轮已报告 ETW CPU | 3,516.3342ms | 13,012.4247ms |
| GetThreadTimes / GetProcessTimes | 3,671.8750ms | 12,484.3750ms |
| ETW − OS 累计值 | −155.5408ms | +528.0497ms |
| 原报告相对差异 | 4.2360% | 4.0580% |

Microsoft 独立 cswitch 结果为 owner 3,516.3390ms，和自算相差 0.0048ms（0.0001365%）。线程迁移没有造成区间遗失或重复记账；DPC/ISR 扣除量也不足以解释 155.5ms / 528.0ms 差异。即使完全不扣中断，两项差异仍不能通过 2%。

另发现当前解析器没有处理 Timer DPC 的 `DPCTmr` 标签。本 trace 有 553 次，总 inclusive 时长 2.874ms；补入同一每 CPU 并集后，仅额外减少 owner 0.017ms、进程 0.145ms，得到 3,516.3172ms / 13,012.2797ms。这应登记为测量完整性修复，但不是本轮累计差异的解释，也不能作为放行理由。没有修改原始报告或工程工具。

Primary parser 证据另见 [Microsoft PerfView KernelTraceEventParser.cs](https://github.com/microsoft/perfview/blob/main/src/TraceEvent/Parsers/KernelTraceEventParser.cs#L2283)：普通 DPC opcode 68 和 TimerDPC opcode 69 注册同一个 `DPCTraceData`，其 [ElapsedTimeMSec](https://github.com/microsoft/perfview/blob/main/src/TraceEvent/Parsers/KernelTraceEventParser.cs#L8772) 使用事件完成时间减去 `InitialTimeQPC`，因此两种 DPC 使用相同端点语义。

## 量化与时间区间

300 个相邻 completion 的原始 OS CPU 差值，其最大公约数严格为 15,625,000ns，即 15.625ms：

| OS completion CPU 增量 | 次数 |
|---|---:|
| 0ms | 136 |
| 15.625ms | 105 |
| 31.25ms | 55 |
| 46.875ms | 3 |
| 156.25ms | 1 |

没有裁掉 156.25ms 的尾部样本。这个分布直接证明本机返回值的实际量化远粗于 FILETIME 的 100ns 单位，不能用这些逐帧差值建立可信 p95。

从首个 completion 开始累计，OS−ETW 残差随区间改变方向：

| 后续帧数 | OS CPU | ETW owner CPU | OS−ETW |
|---|---:|---:|---:|
| 30 | 250.0000ms | 342.5352ms | −92.5352ms |
| 100 | 1,000.0000ms | 1,094.5562ms | −94.5562ms |
| 150 | 1,812.5000ms | 1,817.7792ms | −5.2792ms |
| 200 | 2,343.7500ms | 2,389.2739ms | −45.5239ms |
| 250 | 3,140.6250ms | 2,956.2655ms | +184.3595ms |
| 300 | 3,656.2500ms | 3,508.1023ms | +148.1477ms |

这里使用首个 completion 作起点，因此总值与包含第一个执行帧的 envelope 不同；两种区间没有混算。残差不是固定 QPC 换算倍率，也不像仅一次起点偏移。

夹具 envelope 依次读取 QPC、线程 CPU、进程 CPU；completion 则先读线程 CPU、再读 QPC。envelope 起点到第一帧 QPC 只有 4.2µs，末 completion 到 envelope 末 QPC 只有 3.9µs，普通顺序读钟差距明显小于本轮差异。**现有末端没有 OS API 返回后的 QPC，尚不能数学上完整排除末端 API 期间的极端调度；应补每个 envelope OS 读取的前后 QPC，公开读取窗口。** 不能把未记录的窗口当作已测得为零，也没有证据把 155.5ms 误差修正为某个固定边界补偿。

## 官方资料与可验证解释

[GetThreadTimes](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getthreadtimes) 和 [GetProcessTimes](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getprocesstimes) 规定 FILETIME 使用 100ns 单位；没有承诺相同的实际采样精度。进程计数覆盖全部线程，与只统计 owner 的结果必须分别比较。

Mark Russinovich 的官方历史说明 [Inside the Windows Vista Kernel: Part 1](https://learn.microsoft.com/en-us/previous-versions/technet-magazine/cc162494(v=msdn.10))，在 “CPU Cycle Counting” / “Watching Process CPU Usage” 中解释：间隔时钟记账可能把整个 tick 记给被中断的线程，实际该线程未必运行整个间隔；Vista 公平调度改用 cycle counter，并不表示诊断 CPU Time 已换成同一精确计数。这是量化记账可能产生错归的机制依据；它是历史文档，不是 Windows 10 当前内部实现的完整契约。本机版本 10.0.19045 的实际行为仍须通过本机数据验证。

[Microsoft CPU Analysis](https://learn.microsoft.com/en-us/windows-hardware/test/wpt/cpu-analysis) 将 CPU Usage (Precise) 建立在上下文切换、ready 与 switch-out 时间线上，并分别说明 DPC/ISR 的 fragment、inclusive/exclusive 时长及 Regular/Timer DPC 类型，支持这里按 CPU 重建并扣除中断的审计方向。

目前最强解释候选是 **15.625ms CPU 记账与短促、反复调度工作之间的采样混叠/统计误差**，不是调度区间积分丢失。作为可行性检查，把同一 owner 原始运行区间投影到 15.625ms 周期采样网格，在 512 种相位下得到 195–254 个记账 tick，其中确实存在与 OS envelope 的 235 tick 相等的相位；但 trace 没有记录真实 clock/IPI 记账相位，因此这只是候选解释的可行性，不是因果证明，不替换任何验收。

不得把 `QueryThreadCycleTime` 返回的 cycles 按一个固定 CPU 频率换成毫秒。其[官方说明](https://learn.microsoft.com/en-us/windows/win32/api/realtimeapiset/nf-realtimeapiset-querythreadcycletime)明确不应这样换算；CPU/计时器实现与频率行为存在差异。

## 后续验证与放行规则

1. 已安排同一冻结 ON EXE 的 600 帧预热、3,000 帧长采样。保留 owner 和整个进程的 OS 与 ETW 原始累计值、独立 Microsoft cswitch 对照、全部样本和失败状态；长采样不预先视为通过。
2. 比较长短区间以及不重叠分段残差：误差是否随累计 CPU 增长而收敛、是否持续同向或呈稳定周期关系。更长采样能降低某些采样方差，但相关/混叠误差不保证按时长自动消失。
3. 补 envelope OS API 前后 QPC，验证实际读钟窗口；修复 `DPCTmr` 扣除完整性，并跑对应合成区间测试及真实 trace 对照。新增原始字段应保持旧报告可读，不静默改写已有文件。
4. 若长采样仍超过 2%，继续保持该轮无效，安排独立的较长代表性计数校准或更明确的 OS 记账诊断，再确定可满足现有标准的测量方法；不得挑选通过的尾段、删异常样本、只以 Microsoft cswitch 一致覆盖 GetTimes 失败，或暗中调大容差。
5. 本调查不改变 Windows timer、线程/进程优先级、驱动、画质、场景规模或实际工作量。未改变 16ms 最终性能门槛。

以下已追加长采样真实结果。当前结论仍为：**调度积分获得强交叉证据；累计 OS 计数一致性未达标，闭环未放行。**


## 追加：600 帧预热、3,000 帧长采样

[closure-static-long-1](C:/game/.perf/headless-runs/closure-static-long-1) 使用相同冻结 ON EXE，预热 600 帧、采样 3,000 帧，envelope 执行 3,001 帧、墙钟 474,606.775ms。该轮 **仍为 invalid**，只剩进程累计 OS 计数差异一项失败。短轮预热 60、长轮预热 600，真实模拟起点不同；这里比较的是时钟一致性，不是相同输入的性能 A/B，不能计算优化收益。

| 指标 | 长采样实际值 |
|---|---:|
| owner ETW / GetThreadTimes | 38,263.2384ms / 38,468.7500ms |
| owner 相对差异 | 0.53423%，本项低于 2% |
| 进程 ETW / GetProcessTimes | 147,930.3571ms / 140,890.6250ms |
| 进程相对差异 | 4.75882%，仍超过 2% |
| 进程 ETW − OS | +7,039.7321ms |
| Microsoft owner cswitch | 38,262.6710ms |
| 自算 owner 与 Microsoft 差异 | 0.5674ms，0.001483% |
| Microsoft 全进程 cswitch（按 PID 29044 汇总） | 147,929.8390ms |
| 自算进程与 Microsoft 差异 | 0.5181ms，0.0003502% |

进程独立对照不是仅查 owner：从 `official-owner.csv` 按完整 PID 29044 匹配所有行，确认 TID 唯一并汇总，得到 20 个非零时间线程。与自算进程积分也高度一致，说明 7.04 秒差异不能归因为“只交叉验证了 owner、进程聚合另有未经检验的积分”。但两个实现都读取同一 ETW 调度源，这个一致性不能替代独立 OS 累计记账的失败项。

owner 从首 completion 开始的累计残差仍有明显波动：

| 后续帧数 | OS−ETW owner |
|---|---:|
| 300 | +102.0661ms |
| 600 | +445.6021ms |
| 1,000 | +637.5612ms |
| 1,500 | +555.5748ms |
| 2,000 | +527.5935ms |
| 2,500 | +470.2493ms |
| 3,000 | +183.3264ms |

延长采样使 owner 的本轮最终相对差异低于门槛，但进程差异从短轮 4.0580% 变为长轮 4.7588%，绝对差从 528.05ms 增长到 7,039.73ms。因此不能把整体问题解释为只有短采样/一个 tick 的端点取整，也不能假设继续增加帧数必然解决。下一步需补 session 两端 OS API 各自的 QPC brackets，并在独立校准中核对各 TID 的 OS kernel/user 累计值与 ETW，区分计数接口的粗粒度记账、线程集合和其他系统偏差。当前没有这样的逐线程 OS 两端快照，不能凭 ETW 线程耗时猜定是哪条 worker 的 OS 记账有误。

[小文件后处理与长轮身份摘要](C:/Users/liyu/.codex/visualizations/2026/10/26/01a17714-b237-7ed3-8e87-7aeaa53c4a51/closure-static-long-1-clock-summary.json) 由 `cpu.json` 的逐帧 owner 积分和 `session.json` 的原始累计 OS 点生成，没有第二次完整解析 7.82GB CSV；仅顺序读取原始文件计算身份哈希。原始 CPU 结果、样本及失败标记均保留。

| 长采样文件 | SHA256 |
|---|---|
| [session.json](C:/game/.perf/headless-runs/closure-static-long-1/session.json) | `248eba23958f9054639c7f98f450e4839c6c56336dc86a581949a99c6f234b43` |
| [cpu.json](C:/game/.perf/headless-runs/closure-static-long-1/cpu.json) | `82ef94917183feb3a74213a9b7b2fff54965c46eb10d4cbcc2d5f43460069768` |
| [official-owner.csv](C:/game/.perf/headless-runs/closure-static-long-1/official-owner.csv) | `9d9fdedf56f555ab0cc659073d4ab655c8c33a72998c032d4634e3f926a03fde` |
| [run.json](C:/game/.perf/headless-runs/closure-static-long-1/run.json) | `61272362573e3edf4cfaa302651e5837bb15a2f34b6f726bfe39ae969df15368` |
| [schedule.etl](C:/game/.perf/headless-runs/closure-static-long-1/schedule.etl) | `d8105c02bdc2e5b528b5da5b3cd06f217dd6bebf89de42bf0cab2aa58e045705` |
| [schedule.csv](C:/game/.perf/headless-runs/closure-static-long-1/schedule.csv) | `404ecb10040a4aa022b4b207ad014d93ea7873a07789208c46a8aeef00824832` |

补丁已暂存，尚未把新的端点元数据或 Timer DPC 结果追填成旧二进制的实际数据。长轮没有 QPC brackets，不能声称已验证新读窗修复。**最终放行状态不变：累计进程 OS 对照仍失败，完整测量闭环未通过。**
