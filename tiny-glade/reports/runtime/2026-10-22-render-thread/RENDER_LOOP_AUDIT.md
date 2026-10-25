# 完整渲染循环复核（2026-10-23）

## 当前执行状态（2026-10-26，CPU16ms仍开放）

最新V31干净测量（600预热+1000帧、真实OS线程CPU，GPU/睡眠排除）：静态 **8.24ms**、大量绘制 **22.66ms**，主线程分别1.73/2.02ms。相同输入同轮V30为14.66/19.84ms；只有一组，重场景本组增加约14.2%，**没有证明稳定收益**。完整诊断下的CPU p95为22.17/43.93ms，包含日志和探针开销，不能当干净p95。

当前源码在V31冻结后又完成两形状CPU投影复用、零报告字节压力回收与一次分配重试；删除无消费者wait_submission、单线程CLI空开关、VRAM历史别名及不可达同步上传fallback；减少indirect临时分配与重复FrameConstants声明。这部分尚未重新测发布收益，不能拿V31数字代表最终源码。

完整已完成项、逐轮数据、验证范围及按依赖排序的优化清单统一见 [CPU当前状态与后续任务](CPU_STATUS_2026-10-26.md)。下一步先完成有界CPU-only逐帧记录与三轮四场景对照，再依据新栈优化ledger、projection和submit scratch；Collect调度及可变owner绘制循环、HostOOM退役无限等待、F032回收配额和长跑仍开放。没有保留旧接口或串行fallback作为回退方案；未宣布所有draw均已迁移worker或60FPS验收通过。

## 历史：V29 live mesh-bin 与上传接纳验证（2026-10-26）

描述符缓存三轮交错对照已完成：static emit/deduplicate 均值 10.262/10.269ms，many_draws 44.466/44.927ms；各轮方向不一致，没有可重复收益。生产已删除缓存状态、失效接口及过时合同，始终使用唯一绑定路径。详见 [决策与原始轮次](performance-2026-10-24/cpu-descriptor-production-decision-20261026.json)。

已实施 Raster 显式 binding-access 映射，删除二次搜索；Canopy/Trunk 按 live mesh-bin 事件计数收紧命令上界。发现并修复配套接纳缺口：positions/data 任一上传未接纳时保留脏范围，在旧帧 settle 后、提取前重试；全部接纳才滚动实例计数，避免发布新上界配旧 GPU 数据。普通帧只读 pending 标志，不增加实例扫描或验证层。旧 void/try_gpu_flush 双入口已统一为返回 Result 的 gpu_flush；删除空壳绑定接口、默认零 mesh-part 构造、历史别名与无消费者 helper。

当前 country-core 全库 2885 通过、1 忽略；tiny-glade 全库 270 通过、5 忽略。上传拒绝用例覆盖 positions 已接纳/data 重试，以及 mesh0/1→7 缩小上界时 positions 被拒后无新写入重试。两处 History App 夹具补真实 Game 状态和 onboarding 事件；方向性 cull-id 断言改在 set40 与 set41 之间观察，整帧后验证清理，无生产调度绕过。原 Nani 2 项（含65实例、1/3真实part）、Raster像素2项、bindless ABI 5项和工作区全 target 检查通过，见 [原始验证记录](performance-2026-10-24/cpu-bin-admission-verification-20261026.json)。当前源码的优化构建和 CPU 收益测量尚未完成，**重场景 CPU16ms 未达成验收**。

下一步先冻结并测量本批，再按新热点推进唯一有序提交类型和 scratch 复用。atlas 热重载未接线 API 的完整 remap 合同仍待独立处理；不将冷端排序修复视为完整热重载支持。


## 历史：CPU 测量条件修复（2026-10-26）

生产路径继续为唯一并行模型。新增 u32 别名回归 163 项、Raster host 13 项及 210 游戏帧窗口通过；资产确定性修复另有 17 项加载测试通过。旧 V26/V27 对照因 atlas 注册顺序不同已停止收益归因。现在 Prefab.parts 在发布前按 part 名排序，atlas 初载/重载共用按 prefab 名排序的装配入口；四组 210 帧短跑的 static / many_draws 输入签名分别完全相同。

外部采样随后发现测量进程被后台降级：同一 V28 实验镜像、同一 static 输入与阶段诊断，原轮次主/渲染线程 49/50 个快照的 priority/maxpriority 为 4/4；仅为测试进程传入 Foundation 参数后，39/39 个快照恢复为主线程 46/63、渲染线程 31/63，渲染 CPU 均值由 36.976 降至 10.931ms。**这是调度条件对照，不是渲染算法收益。** 旧轮没有该字段时不能追溯认定 App Nap。见 [调度证据](performance-2026-10-24/cpu-scheduling-condition-audit-20261026.json)。

CPU 工具现在要求短跑输入参考，在预热前发现差异立即取消，并在进程外拒绝被后台节流的验收样本；没有新增游戏热路径检查或系统持久偏好修改。V28 的 descriptor 试验控制仅存在 `/tmp` 测量副本，生产无该开关。正常调度的首组 static 阶段诊断为 emit 10.931、deduplicate 9.297ms；关闭阶段日志的三轮交错对照及大量绘制仍在进行，**CPU16ms最终验收未完成**。见 [当前测量](performance-2026-10-24/cpu-descriptor-awake-measurements-20261026.json)。

本节早先候选状态已由顶部实施记录更新；有序提交类型与 scratch 复用仍待实施。atlas 热重载还存在未接线 API 的 part-count/remap/容量合同缺口，不能将冷端排序修复称为完整热重载修复。


## 最新核查更正（2026-10-25，稳定输入与绑定容量）

**CPU 16ms 目标仍未验收，V24/V25 的跨版本性能收益尚未得到可比输入证明。** 复查发现旧压力夹具直接选 ECS 查询前 32 项，实际绘制列表在相同二进制的不同轮次也可能变化；静态 forward 列表亦有差异。因此下文历史单组下降百分比仅保留为观测记录，不能归因为本批优化收益。

当前夹具在预热前按 CPU mesh 内容、shader、transform 与 pass 语义排序并记录 schema 2 输入签名；采样器强制要求新签名，保留原始线程 CPU 计数及同一次查询获得的调度状态。签名覆盖范围有限，不等于完整 GPU 场景等价。

新异构压力输入暴露了生产 pass 绑定别名超过 255 时的 panic。本轮将 Raster/Compute 的 CPU 别名索引及内容意图表统一为 u32，保持现有 8 字节 access 记录，未增加 GPU descriptor 容量或保留旧窄索引接口。当前 pass builder 163 项、Raster host 13 项通过；真实窗口压力夹具完成 210 游戏帧并正常退出（1522 项实际 draw-list entries、480 克隆），后续优化构建正在冻结。压力释放与投影池候选尚未接入生产。

下一步：真实窗口验证 → 用相同夹具、相同 u32 修复构建描述符去重开关差分的两份实验二进制（生产不增加运行时开关）→ 交错重复 CPU 测量 → 再处理已定位的重复扫描、提交 scratch 与压力回收热点。现有短跑和锁顺序审查不足以宣布整链无死锁。


## 当前状态（2026-10-25，CPU 专项续修）

生产只有共享任务池的并行 Collect/录制路径。**CPU16ms目标尚未达到。** 两个只读审查agent、中央修改测试。V23的64项回归与全target检查通过；V23信用批未显示render收益；后续V24单组观测静态35.236→20.796ms、多draw40.558→33.226ms，输入一致性未得到证明，不能据此认定收益。descriptor原生绑定去重的正确性已验证，收益正在用新夹具重测。

| 范围 | 已落实 | 当前验收边界 |
|---|---|---|
| 泛型与并行 | CpuPool<T>、GpuUse、scratch、PhysicalIdentity、collections、map_chunks 共用；Serial/Inline/Adaptive 录制、旧错误旁路等已删除 | owner 有序提交与完成发布仍是同一协议；worker owner-transfer/end 的借用型泛型优化尚未实施 |
| 上传与确认 | 有界接纳、保脏重试、事务、五类逐条 ACK、真实完成点、旧 World 隔离 | 既有 GPU/窗口结果保留原源码范围；本轮没有扩充工具/Undo 业务验收声明 |
| 增量提取 | 当前生产 mirror_journaled/component_journaled，静态帧不扫描组件 tick；Raster 生产者记录已知脏区间 | 旧报告中 Changed 全量扫描待迁移属于历史状态，不应继续写成当前缺口 |
| resize 与全屏 | 唯一呈现坐标映射、macOS layer frame 与隐式动作事务修复已落实 | V18/V19 原生全屏 2940×1846，0/5427240 错误像素；本轮更新旧呈现测试为当前缩放/翻转的全目标像素断言 |
| V20 预算优化 | BindlessLease 元数据在页准入一次计费，保留 slot/owner 析构顺序；零字节与相同容量 reservation 避免无效锁 | 严格 Vulkan 12 项通过/1 ignored；预算 CPU 11 项通过。首次错误 filter 的 0 项不算通过 |
| V21 CPU 优化 | Collect 不可变查询快照；投影复用强 owner；不可变提交检查一次；ABI owner 快路；相同 compute push 复用；同回调 pipeline/vertex/index 绑定去重；全零顶点源按容量复用 | 加载/Raster/呈现/host 22 项 GPU 回归通过；扩容 owner 1 项、compute CPU 4 项与应用库/程序/测试目标检查均通过；首组干净 CPU 数据见下节，不能用代码变化代替收益验收 |

### CPU 指标与证据修正

1. **约 54–56 ms/帧是所有线程的 CPU 工作量之和**。历史接近 10 ms 的值分别是反馈完成 wall 9.491 ms 或 record+submit wall 10.235 ms，口径和构建不同，不能直接得出 CPU 从 10 ms 退化到 54 ms，也不能据此排除回退。并行总工作量、单线程实际 CPU 时间、并行关键路径必须分开。
2. 测量脚本在游戏进程外读取 Darwin 线程 user/system CPU 累计值；排除 GPU 执行和休眠等待，仍包含忙等。约 80 ms CPU 自检与线程 CPU clock 比值 1.0015，80 ms 休眠仅推进约 0.104 ms。每秒快照给出均值，**不是每帧 p95 或整链关键路径**，没有新增游戏热路径验证。
3. 本轮 CPU-only 阶段诊断仍开启日志，不能当生产验收。V20 static/many 的 tg-render 均值分别 48.489/48.489 ms，总 CPU 80.759/94.273 ms；V19 对应 42.945/45.910 ms，总 CPU 73.751/94.521 ms。环境与日志成本尚未隔离，不据此宣称优化有可靠收益。见 [CPU 续修数据](performance-2026-10-24/cpu-chain-followup-20261025.json)。
4. 旧 GPU timestamp profile 的队列提交样本含大量 timestamp 编码；不能视为纯生产提交成本。V20 CPU-only 有效栈 6608 样本，原生 submit/Metal 编码 1717（25.98%），Rust owner 转移 175（2.65%），command end 包含 normalize 仅 5（0.076%）；inclusive 父子项不相加。全零 Vertex_Color 的逐 draw 新建/上传占渲染线程 184/3420（5.38%）样本，其中 168 进入原生提交，已作为本批明确热点修复。
5. 发现 macOS external debug-map 对象被后续构建覆盖：新 V19 profile 出现 V20 才有的 LeaseSlot 与错位源码行，**该次 Rust 源码归因作废**，OS CPU 计数和独立原生驱动栈仍有效。V20 已在下一次构建前冻结匹配 UUID 的 dSYM，后续 CPU profiling 只接纳哈希匹配的符号。55 个临时 C/asm 对象缺失告警保留，不能宣称完整 native DWARF；Rust 对象无此缺失。历史已采集 profile 不一概作废。

### 当前验证与后续顺序

V21 受影响渲染库投影/ABI/提交 12 项，以及加载/Raster/呈现/host 22 项实际 GPU 测试已通过。故障记录保留：旧测试安装路径失效；修正路径后暴露旧“不缩放”预期；零源创建最初的 Option<&Vec> 泛型参数不匹配。改用当前安装入口、全图精确像素断言与切片参数后重跑，不保留兼容路径，不放宽像素判据。日志见 [本轮验证](performance-2026-10-24/cpu-chain-verification-20261025.json)。

1. V21 二进制、源码、匹配 dSYM 的冻结，以及首组 CPU-only 诊断/无日志对照均已完成；继续实现参数预算 credit 与常量批量领取后，再构建下一批并复测。60 帧目标按 CPU 侧 16.67 ms 关键路径判断，不能拿总 CPU 或 GPU wall 时间替代。
2. 新栈优先指向参数预算和动态常量争用，descriptor 绑定去重或每 chunk owner-transfer/end 迁移排在其后。零源复用仍按每图导入、实际范围和真实生命周期工作；无全局命令状态猜测、无串行回退、无新逐 draw 验证层。当前扩容单测不替代“旧图 GPU 在途时扩容”的完整组合验收。
3. F-032 压力回收绕过配额、F-201 投影驱逐与 F-192 current_version 扫描仍未修；常量池搜索/锁、启动检查只列候选，未证明为当前主要热点。继续核实，不标全模块 bug 清零。
4. 完成大量绘制、局部/连续编辑的正式重复矩阵；loading_switch 仍缺实际切换驱动。稳定/效果/resize 各 30 分钟及至少 10000 帧、独立硬件队列/Windows 仍未验收。短跑通过不等于已排除所有死锁。此前 V18/V19 静态对照方向反转的数据继续保留，不发布可靠 FPS 增益比例。

### 最新干净 CPU 对照与后续源码修复

V21 已完成一次8分26秒优化构建，源码指纹无漂移，匹配UUID dSYM已冻结。V20/V21各自的新CPU-only profile均使用匹配符号；旧V19失真的Rust源码栈不参与对照。每场景一组V20→V21，600预热+1000测量帧，阶段日志、GPU timestamp、栈采样全部关闭：

| 场景 | V20渲染线程CPU | V21渲染线程CPU | V20→V21总CPU工作量 |
|---|---:|---:|---:|
| static | 39.067 ms/帧 | 36.281 ms/帧 | 66.092→61.895 ms/帧 |
| many_draws | 42.616 ms/帧 | 43.126 ms/帧 | 82.930→80.353 ms/帧 |

这是实际user+kernel CPU均值，排除GPU执行与休眠等待；不是每帧p95或整条并行关键路径。静态渲染线程下降7.13%，大量绘制反而增加1.20%；只有一组且场景/输入控制仍需完善，**整体收益未定，60帧CPU目标未达标**。不能用总CPU下降代替关键线程改善。见[干净对照](performance-2026-10-24/cpu-clean-comparison-20261025.json)。

V21稳定采样已无逐draw零源staging/setup、Collect AssetTables锁及managed buffer弱owner查询链。剩余争用主要来自参数builder预算预留/释放、全池统计和动态常量分配；不能把GPU驱动draw-state的全部成本归因于descriptor绑定调用。V21阶段诊断的1000静态/1500大量绘制帧分别实际观察Collect任务20000/30000、跨pass3000/13500，上传容量让出均0。

随后源码已修Raster同值消费反馈污染change tick：唯一WriteBack合同接收Bevy Mut，不提前DerefMut；只在消费状态或rasterized确实变化时写入。无整张像素比较，保留提交失败、晚游戏写入、旧World/资源替换保护。7反馈+27线程/交换+10快照测试共44项及应用库/程序/全测试目标检查通过。另保持描述符统计的精确Mutex事件事务，只在分配时更新总/当前kind峰值，释放/耗尽不再扫描四表；9项并发峰值/注册失败回归通过。**这两处后续修改尚未进入V21测量，不宣称已取得性能收益。**

动态常量批量领取已随V22通过正确性回归；下一批优先落实有界参数预算credit，沿用现有CpuPool和owner生命周期；逐draw借还仍会重复记账，不能当作解决争用。独立原子计数会破坏精确同时峰，未采用。worker保活收尾迁移、descriptor显式状态去重继续按新证据排序；F-032/F-201/F-192、正式重复矩阵和窗口长跑仍开放。

### V22 并发与常量分配续修（正确性已验证，性能待测）

保持同一并行架构，新增以下修复：HostFlushQueue在native flush真正结束后才清dirty，避免释放VkDeviceMemory或另一提交越过尚未完成的flush；失败上传检查统一slots→local锁顺序；TextureHost延后释放只持表owner，打断取消时的boundary引用环；光标四处同步映射写迁到既有owned上传，容量不足保留字节、没有新输入也重试。Raster参数块在同一回调一次取得dynamic锁，仍逐块使用正式allocator；临时列表在锁外按现有TempList一次准入预算。

15项flush、29项host、17项主循环、2项Raster、5项ABI，共68项通过，应用lib/bins/全tests目标检查通过。两项旧测试的per-lease预算假设失效已迁到页准入预算与额外slot部分失败撤销，首次失败日志保留。见[续修证据](performance-2026-10-24/cpu-concurrency-followup-20261025.json)。具体锁序条件与flush并发已受控复现测试；未复现游戏整链死锁，不宣称已排除所有死锁。

V22优化构建与CPU-only三轮交错对照进行中，以上改动和此前Raster反馈/统计缩短尚不能报收益。剩余优先参数builder有界credit、重复绑定错误前cache增长、IndexedCountAdapter旧bare-heap入口、静止PrevTransform无谓发布，以及正常截图等待GPU。基准当前仅核对480克隆/实际draw数量，没有来源内容签名；三轮仍须保留该可比性限制。

### V23 参数唯一入口与 CPU credit（64项回归通过，收益待测）

参数构造统一到 GroupReservation：删除裸heap构造、Option<pages>分支、旧Prepared::with_push；IndexedCount、文字、window_lab和GPU夹具均迁同一入口。CPU参数额度沿既有CpuPool按回调块准入，ABI上界只在不可变package构造时计算，逐draw只消费本地credit；重复slot在cache/descriptor副作用前拒绝。静止PrevTransform不再触发组件日志，普通截图复用已准入像素缓冲并以0超时检查真实GPU完成。

4项credit、5项drawable、17项截图（含真实GPU阻塞/重试/恰好一次交付）、5项ABI、29项host、2项原始blur、2项原始文字像素，共64项有效通过，tiny-glade/rhapsody/tg-text全target检查通过。首次截图过滤器误命中0项已纠正，0项运行不计验收；先前编译错误/磁盘不足日志保留。[验证与边界](performance-2026-10-24/cpu-credit-followup-20261025.json)。V23 release冻结进行中，不把这些改动计入V22收益。

V22严格窗口的单轮CPU线程时钟：静态tg-render22.285ms/帧、多draw39.414ms/帧；不含GPU执行和睡眠，未达到16ms。测量有间歇约1秒resolve等待，独立栈确认可达MoltenVK nextDrawable但未抓到长等待本身；不能把该候选认定根因。旧V21/V22三轮计划已停止，只保留单轮诊断，非正式60帧结论。[原始绑定汇总](performance-2026-10-24/cpu-concurrency-partial-cpu-20261025.json)。下一批在V23实测后处理projection交替缓存与submit scratch，禁止新旧兼容双路径。

### V23 首组实测结果（未证明渲染线程收益）

同条件单轮clean对照（各600帧预热+1000帧测量、关闭GPU timestamp/阶段计时/CPU栈）：静态V22→V23为36.120→37.390ms，多draw为51.753→51.674ms，均为tg-render用户+内核CPU时钟，不含GPU执行/睡眠。当前批不能记为渲染线程收益。先前V22静态22.285ms跨轮波动明显，只保留历史测量边界，不用它与当前V23直接算优化率。新profile显示HostState预算热点明显淡出，但原生indirect编码仍占主要成本。见[对照与限制](performance-2026-10-24/cpu-credit-comparison-20261025.json)。

下一修改仅收紧fine Count的Owned活跃实例上界，保留真实GPU计数器；将用Canopy/TreeTrunk每组65输入、单mesh part的原shader夹具验证不会截断跨32实例组的命令。projection缓存、typed submit scratch与descriptor原生绑定去重仍未实施，不计为已修。

### V24 fine Count 活跃输入上界（14项通过，收益待测）

Count绘制在唯一packet终结入口保留真实GPUcounter，将Owned最大命令数收紧到同一渲染快照instance_count与原容量的较小者。不能用mesh_part_count：65个相同part实例会跨3个分组生成3条命令。Raw仍按原mesh parts，External仍用其producer容量；没有新旧双接口或串行回退。

10项CPU决策/builder、2项原Nani GPU、2项真实Raster测试通过。Canopy/TreeTrunk原shader65输入/单part实际生成3命令，验证非重叠区间覆盖与全部65indirection；没有把None-HZB首轮冒称OC/2。旧夹具初始导入状态冲突修正为实际0x35，仍断原cull读0x1a，首次失败保留。[证据与边界](performance-2026-10-24/cpu-nani-bound-followup-20261025.json)。V24正在单独冻结/实测，尚未宣称收益或16ms达标。

### V24 首组有效收益（仍未达到16ms）

clean静态V23→V24为35.236→20.796ms/帧（单组约41%下降），多draw为40.558→33.226ms/帧（约18%）。这些是用户+内核CPU线程时钟，不含GPU执行与睡眠；各预热600+测量1000帧，仅一组有效配对。首次多draw两轮被本任务慢目录查询干扰，已明确作废并整组补跑，未混入收益统计；替补目录round2不表示已有两组有效验收。[有效对照及原始绑定](performance-2026-10-24/cpu-nani-bound-comparison-20261025.json)。

实际停帧现场97.83%的tg-render wall样本位于CoreAnimation nextDrawable→semaphore_timedwait，六个worker均休眠，未见Rust互锁环；只确认等待位置，drawable耗尽/不可用的上游原因未定。该捕获只作用于独立诊断，不能拿它的耗时当CPU执行或正式性能。descriptor同command/layout/set原生去重已进入下一批源码，GPU ABI已覆盖A/A′/B/A/raw改绑恢复；测试收尾后再冻结V25，尚未计收益。projection缓存仍待压力退役零报告协议一并处理，typed提交scratch尚未实施。

### V25 原生descriptor去重（18项通过，实测待完成）

现有唯一图/裸命令绑定接口共用同一私有录制状态，只跳相同command/bind point/layout/set/真实page的原生bind。所有lease保活、访问准入、每draw参数索引push均继续；begin/reset和callback边界清状态，原生外部改绑/secondary执行通过明确失效边界恢复。无新增高频验证层、旧构造入口或串行回退。

1项状态边界、5项ABI、8项保活、2项Raster、2项文字像素共18项有效通过，全target检查通过。GPU序列A/A′/B/A/外部有效set改绑后invalidate恢复得到[42,43,44,42,42]；故障时外部有效参数会给124，不依赖未初始化描述符。两项旧Timeout容量断言改为当前Exhausted后重跑通过，首次失败不抹去。[证据](performance-2026-10-24/cpu-descriptor-followup-20261025.json)。V25 release冻结/CPU对照排队，不能提前计收益；当前目标仍开放。

## 历史审查与故障记录（以下“当前”均指记录当日构建）

### 2026-10-24历史核查

生产链已闭合上传失败保护与库级确认重试，但整体验收未结项：基础页容量/显存长跑已经修复并通过；四场景三轮性能及完整组合矩阵仍需完成。已落实：

- 上传事务 prepare/finish/check 均接入真实写入与图提交；中途失败保留完整范围，不允许图消费部分版本。Owned 预算满立即拒收，producer 保脏让出，下层压力保存游标；取消后不形成 boundary/job Arc 环。7 项边界测试及真实多段 OOM/队列饱和/取消用例通过。
- ACK 不再取走当前库版本；相同版本的失败帧可在后续提交推进。纹理接同一链；GPU 完成取真实全队列 SyncSet，不依赖存在回读源。无窗口游戏夹具接同一发布器；旧 World 反馈拒绝。
- ECS_COLLECT_GATE 已删除，不能再用“池线程超额”保证进展。实际 30 帧强制并行得到 540/420 个 Collect/跨 pass 任务。首个 1060 帧在资源归属检查通过后失败于页容量增长，页复用减少冗余页，复验仍因第 383 帧 GPU 重叠导致容量 3633→7266 失败；真实 drain 后 actual=expected=[8,0,6,54]，未放宽既有断言。
- macOS 窗口死锁的线程采样确认：主线程等待 worker 控制调用，worker 的 Window::request_redraw 同步等待 AppKit 主线程。改用 EventLoopProxy 唤醒。全屏退出期间立即 setContentSize 还会干扰原生异步动画，已改为窗口态后恢复尺寸；完整窗口周期通过。
- 真实双 App→游戏窗口、加载中关闭、挂起不恢复直接关闭、resize/零尺寸/focus/全屏往返均通过。真实晚相机两次预览命中，释放和取消后的 capture idle 验证通过。App 激活后续已随帧异步移至 worker，真实窗口通过；完整截图/切换/回放组合待验。
- 最新生产 `cargo check --workspace --lib --bins --examples --locked --offline` 通过。全 target 检查中途被清理删除的原侧夹具阻断；按用户要求恢复/重采集到 workspace tests/fixtures/native 后已通过。原侧差分 extra 1、animation 2、undo 2、UI 3、photo 1、audio 12 通过；固定来源和边界单独记录，不使用候选输出生成 expected。

本轮硬件为 Apple M3/MoltenVK，设置真实 GPU 必需并提供验证层搜索路径；具体测试是否启用由测试配置决定。历史 lavapipe 的 Unsupported 和 `/tmp/render-*.log` 成果保留原范围，不代表本轮重新通过。持久日志、命令与源码身份见 [验证目录](validation-2026-10-24/manifest.json)。并行默认仍等待四场景三轮性能验收。生产默认没有 Vulkan 验证层，本轮也未新增；专项验证层只用于测试。上传事务正常帧是失败计数快速门，失败解除即恢复，不保留持续全表扫描。

### 2026-10-24 续修核对

窗口黑边的直接原因是缩小的最终图送进按窗口坐标整数 Load 的呈现 shader。最终图与 UI 已改用物理窗口尺寸，内部世界和 TGSR 仍按比例渲染；WSI 取图边界处理 resize 旧帧。6 项尺寸单测、120 帧窗口周期和另一次 400 帧真实窗口通过。原生窗口截图确认过渡结束后的场景/UI 铺满内容区；新测试第一轮曾抓到全屏返回旧尺寸，保留失败后修复，没有用旧的“present 成功”代替像素检查。

App 激活和 WriteBackSet 随帧交给 worker，成功后 ACK；取消保持下帧重试资格。Raster 生产者直接记录已知瓦片范围，移除其整段编辑前副本与重复 CPU 比较；核心库 2844 通过、1 ignored。通用组件 Changed tick 扫描和逐资产条目 ACK 仍未全部替换。

早期 descriptor arena 两次 1060 帧页容量通过、显存失败；后续已定位为每帧临时最终图和动态常量的冷启动容量未覆盖 GPU 重叠。最终 MSAA 图按在途槽复用现有 named owner，常量池按帧轮换且保留全部租约/SyncSet 规则；named 发布保留显式 whole-export 布局。480×270 和 1280×720 两次 1060 帧均通过原有全部断言，较大场景 GPU 重叠峰值 4390，每次实际 Collect/跨 pass 任务 19080/14840。最新 300 帧窗口和原生截图、全 target 检查通过。后续与日志边界见 [续修记录](PARALLEL_REFACTOR.md#2026-10-24-续修窗口黑边异步激活与-raster-写入)。

## 2026-10-23 历史记录

## 已发现并修正

1. **共享池录制权限漏洞（P1）**：Collect 有 ECS 准入，图录制没有；默认 TaskPool scope 会在渲染线程执行无关 ECS 任务，可能以渲染线程身份越过下一帧上传的读取门禁。`rg/parallel.rs` 改用 `scope_with_executor(false, None)`，只在调用方执行本 scope 的任务，稳定索引合并；`render_bridge.rs` 两个 execute 入口同时取得准入，竞争时 Inline，避免共享池全被上传任务占用时互等。RAII 恢复 runner，保留 chunk/计划键；正常、错误、unwind 均需验证。agent 对补丁复核通过。后端单元 421、p0 2、分块 8、并行图 3 通过；宿主回归在执行。
2. **加载帧无条件 reclaim**：`GameLoop::frame` 原先每次准备轮询前 settle，即使 gameplay 未完成，也迫使加载 Update 等待渲染。拆出 `ready_to_handoff`，未就绪仅推进 Logo/准备状态，真正换 App 前才 settle。保留旧 World 的反馈与资源移交顺序；代码已修，完整加载/窗口循环待复验。
3. **手动缩放与原版帧尺寸发布缺失**：补齐两 App 的 PreUpdate、resize、OOM 的统一刷新，output 截断，inner 从未截断值乘 TGSR 比例后按 32 对齐，限制 32..10912；相机提交采用重试后的实际渲染候选。原版本地证据 `frame-resolution-recovery-v1/native-pack-v2/mainloop-zero-adaptive-resolution.asm.txt` 地址 1404dd1f0..1404dd6ac。None 模式保留已有 scale，避免每帧强制重置 1；**原版 VRAM/adaptive 调节器尚未恢复**。
4. **诊断导致隐式等待**：frame_observation 已改为固定容量的发布快照；读取诊断不再通过 worker.call 回收当前帧。对应三帧 WSI 验证通过。`frame_present_complete` 仍是同步接口，不能据此称所有诊断都非阻塞。
5. **mapped 宿主写入错误走异步 staging**：即时 CPU 读可能看见旧内容，重复 GPU 测试捕获此竞态。可映射宿主路径改用经过边界/范围验证的 ManagedBuffer::write_bytes；减少 staging 复制与提交。宿主 19 项已通过，重复压力测试仍需统计。一次原生 SIGSEGV 的因果尚未确认，不能仅凭重跑成功称已根治。

## 2026-10-23 历史串行或等待（由后续实施替代）

| 项目 | 当前影响 | 后续修正与边界 |
|---|---|---|
| ReadLease 包住完整 render_back | 取交换链、呈现、GPU 限流期间仍挡住下一帧物理写入 | 先明确最后物理读取点与版本，再缩窄；不能直接提前 drop |
| 冷管线 prewarm / 编译队列退避 | 首帧和新组合可同步等待，最长 180 秒；还占渲染读边界 | 恢复异步 Prepare/Ready，保留真实错误/取消及幂等上传，不直接删等待 |
| AwaitRender 先返回再处理 OS 队列 | 关闭/窗口控制可延后，resize/focus/vsync/suspend 同步 call 仍 reclaim | 独立控制消息/代际协议，保证关闭与暂停唤醒和已接受帧结算 |
| 后台 gameplay.update 不走同一准入 | 当前独立 App/BufferHost，未证明存在同资源循环等待 | 明确独立资源隔离；若共享写入或任务依赖变化必须统一准入 |
| 跨 pass Collect 尚未并行 | 已支持单 pass 连续 draw 的稳定 Collect/Commit，依赖层仍串行 | 建显式 Prepare/Collect 依赖后按层并行，保持原提交顺序 |
| 普通 SSBO 影子与修改扫描 | 稀疏视图/删除日志/Arc 版本已优化，普通影子与变更 tick 扫描仍有成本 | 生产者变更日志、脏区间/瓦片、所有权消息，回退必须有计数 |
| 完整资产上传协议未完成 | CPU 门禁仍承载安全性，不等于物理多版本或 GPU fence | 单调操作序号、幂等分阶段确认、范围依赖与有界多版本 |
| 真正晚相机未完成 | 候选历史已按实际提交反馈，未包含晚输入及精确交互冻结 | 统一剔除/LOD/阴影/TAA 相机版本，交互绑定输入序号 |

## 已核实的主路径与验收限制

- 主游戏声明/编译留在常驻渲染线程，没有发现回到原游戏线程声明的主路径。
- ECS 冲突边重建、共享池、独占 Collect 输出、稳定 Commit、App 代际反馈、资源身份检查已接入；它们不证明完整上传版本协议或全部相机功能完成。
- Inline 回退也会产生多个命令 chunk；`parallel_batches` 不能直接解释为实际并行任务数或并行率。
- 旧增量构建已通过真实 GameLoop 1001 次世界呈现、关闭/保存/GPU 退出。最新改动需重新长跑；1000 帧三轮四场景及真机 p95 对照尚未完成。
- 该历史轮次环境为 Mesa lavapipe，后端全量有 7 项因缺少独立计算队列而 Unsupported；不得计为通过。默认并行 Collect 仍关闭，等待真实硬件性能验收。
- 所有源码、资产读取、构建、日志和显示测试仅使用本地磁盘；网络命令遵守清除进程代理的用户偏好。


## 实际窗口回归暴露的旧路径

`/tmp/render-window-cycle-raw-diagnostic.log` 确认全屏返回后消费 `Raw(UiAppendSignifier { tag: 5, kind: 5, ... })`，来源是 `telegraph_walls_are_merged`。旧实现遍历所有墙，把曲线端点误装为 UI 原始记录；inactive UI 保留流，后续首次悬停消费时 panic。

审计 agent 结合真实注册签名、完整函数与本地机器码确认：原版无 UiSignifierStream 参数，按 PlayLineAcrossWall 事件直接创建 dashed_line 实体，更新自身 Curve3/elapsed，0.75秒后回收。已在原模块增量修正，复用 SignifierEntityBuilder、TransientMesh、dirty mesh 上传；flag 非零反向的是 easing 输入，墙删除不提前终止已有动画，音频 kind5 用实际曲线首点。删除了错误的全墙计时资源；新增无事件无生产、事件/音频/真实网格/墙删除后完成和资源回收、方向边界测试。该修复仍待最新构建及完整窗口复验。其他形状、地形和装饰交互的 Raw/无闭包 Factory 尚未全部恢复，不能泛化为已经清除所有旧路径。

## 后续验证进展

- `/tmp/render-drained-ownership-final.log`：189 主程序单元、原版游戏链 6 项（含真实30帧和全部历史）、WSI三帧通过。描述符断言改为真实 GPU drain 后逐类 live 数精确等于当前 FrameGlobals/SamplerPages 所有者数，同时验证页容量稳态；未增加固定容差。瞬时峰值可受录制分配及完成时序影响，不再把它任意固定为第10帧峰值+16。
- `/tmp/render-host-mapped-repeat-final.log`：19项宿主GPU测试连续20轮通过，未再次出现即时映射读取竞态或原生崩溃。此前一次SIGSEGV因果仍未确认。
- 最新强制Collect长跑 `/tmp/render-parallel-resolution-1000.log` 仍在进行。当前记录显示准入竞争让Collect回退内联，**启用开关不等于覆盖实际并行**；该运行只能算对应回退路径长跑。图计划缓存每帧GraphStructure miss正在进一步审计。
- 修正诊断采样：declare_probe 以前只在打印帧排空，使第100帧行混入此前未打印帧累计数据；现改为每个启用计时的帧排空。全局原子计数仍可包含同期后台准备，不能作为严格单App归因或真机性能结论。


### 新增闭环（2026-10-23）

- 修复通用 worker shutdown 永久等待 Pending：先停止并唤醒，worker 归还已接受的 queued/Pending 包并释放读边界；已有 idle 包无需再次 poll。新增 Pending shutdown 单元已通过，idle 竞态修正随最新窗口回归。
- 自适应/OOM 统一使用 canonical AdaptiveRenderState 和 FrameRenderConfig effective scale；用户 GameConfig 目标不被覆盖。字段反馈保留期间 resize 与 flag_20，并拒绝较新倍率请求、其他 World 或资源重建。
- shadow 三级联跨 pass 并行收集，根图稳定提交。实际 30 帧强制运行记录 cross_pass_tasks=90；第一次回退遗漏 render 的断言已修复并通过真实游戏链。
- Raster 生产快照只提取消费元数据，原 CPU 像素/hash 不进入渲染 World，避免快照导致下一笔整块 COW；agent 复核真实消费者仅访问 flags/dirty/staging SSBO。6 项快照测试通过。

完整物理内容多版本仍未实现；当前共享读准入保持正确性，不能提前移除。无独立 compute queue 的 lavapipe 仍不能验收对应硬件能力用例，也不能作为真实 GPU p95 性能结论。


### 多 agent 交叉复核：上传提交与映射读写（2026-10-23）

本轮新发现并进入实现的实际缺陷，不能由先前稳态通过结果排除：

1. **上传入队 ACK 与提交失败**：Sparse flush 在队列接纳后清 dirty，旧 TransferManager 非 Timeout 提交失败会删除 payload。提交期 OOM 后若渲染降分辨率重试，会缺失先前已确认接纳的数据。已新增 opt-in `flush_uploads_preserving_oom`，保留原队列 payload/physical owner/FIFO/ticket，不加副本且该调用立即报错；明确恢复后才再次 pump。DeviceLost/InvalidUsage 仍终态。`/tmp/render-upload-mapped-regression.log` transfers 9 项通过，含真实 allocator host-budget 提交 OOM 后恢复、不同 logical key 同 physical owner FIFO 的 GPU 回读测试；host_buffer_ranges 既有 3 项通过。
2. **多 chunk 中途失败**：后续 chunk 入队/pressure flush 失败可能留下已接受 prefix；稳定 owner 的上层调用若仅记录错误继续运行，图可能看到部分更新。仍在实现 producer write 事务完整性检查，不能将上面的 OOM 恢复当作该问题已修。
3. **CPU 直接写 GPU 映射**：world_raster、history replay 与 upload_to_staging 宏直接 copy_to_staging 修改映射，旧 Update gate 仅覆盖 CPU 声明/提交，不保证 GPU 已读完。生产入口已迁移受真实 GPU range hazard 保护的映射 closure，复用现有映射、不加整块 CPU shadow。新增未 signal timeline 阻塞 GPU copy 的测试已写，tiny lib 尚待执行。
4. **CPU 直接读 GPU 映射**：鼠标深度、地形延迟复制、stroke index、mask 等实际消费者仍可能读取尚在 GPU 写入的映射。底层 `try_cpu_read` 已提供完成检查与 invalidate，Busy 不执行读取 closure；正在迁移消费者，保留旧 CPU 值/重试请求。不能用“两次 Update 延时”作为完成 fence，也不能按 MAPPED_MEMORY 数值单独判断方向（MouseTerrainRaycast 使用 memory=2，仍由 GPU 写入）。

其他已验证增量：`/tmp/render-main-cross-pass-integrated.log` Raster 12 和实际游戏链 6 通过；`/tmp/render-expanded-forced-parallel.log` 30 帧强制并行通过，540 个共享池 Collect 任务、420 个跨 pass 任务，GPU 完成后的描述符 actual=expected=[8,0,5,72]。该结果覆盖扩展主/OC2/late pass，但不覆盖之后的映射/上传修复，也不是硬件性能验收。

窗口晚相机测试的两次构建曾分别失败于测试 helper 漏 `Ok(())`、新 GPU 测试 read_buffer 长度 u64/usize 类型不符；均已修，等待新一轮统一构建。未将这两次日志记为通过。


### 2026-10-24 控制与截图组合收尾

修复渲染 World 被替换时未完成截图的回调丢失：请求 owner 退出时发送一次取消；已有 writer 保持 join，未增加帧内验证层。16 项截图单测、真实 GPU 取消图后重新提交并回读红色像素通过。真实双 App 窗口运行 180 个世界帧，六次截图跨 resize、暂停恢复、零尺寸和全屏往返成功交付，关闭前未提交请求取消，全部回调恰好一次。App 请求隔离另由生产提取/替换 World 单测覆盖。OS resize/fullscreen 与注入语义事件的边界保留；精确交互/完整回放尚未验收。证据见 validation-2026-10-24/manifest.json 的 control_combinations。用户指定后续顺序：可变组件日志 → 逐条目 ACK → pass 迁移核查 → 精确交互/回放 → 性能优化。


### 2026-10-24 可变组件生产者日志收尾

当前生产提取中的 37 个组件声明接入结构钩子与可变生产者日志，实际 81 处查询写入、20 处直接写入。ComponentWrite 委托 Bevy 的查询访问规则，在首次可变借用时登记实体；静态帧不扫描组件 tick。首次/换 World/有界日志过期仍重建，通用 component() 保留 tick 路径；未来新增生产者需沿同一写入入口，不宣称任意外部 get_mut 都可被自动拦截。原作签名注释保持原样。

验证：all-target 检查通过；生产者日志专项 3、提取/线程 25、核心库全量 2847（1 ignored）、受影响模块 181、真实 GPU 游戏链 6 全通过。模块/游戏链首次构建因磁盘耗尽失败，清理 10 月 22 日前的失效 country_core 增量缓存后重跑通过，失败日志保留；未删除源码/夹具/证据，未放宽断言。证据和源码哈希见 manifest.json 的 component_producers。下一项为逐资产条目 ACK。


### 2026-10-24 逐条目 ACK、兼容入口清理与 pass 复核

五类资产（SSBO/Mesh/VAO/Texture/Placement）统一通过 AssetSnapshot 发布单调条目操作及 Upsert/Remove。失败帧保留确认资格，成功提交转入真实 SyncSet 完成队列；重复/乱序消息、删除后重建、日志过期、资源替换与旧 World 隔离均覆盖。180 世界帧的实际窗口已确认五类资产 GPU 完成，并交付六次截图和一次关闭取消。该窗口运行早于本节兼容入口删除，后续整链按各日志源码范围分别记录。

按用户要求删除旧组件 tick 扫描、旧库级增量提取入口、Placement 未记日志表修改分支、TG_SYNC_UPLOAD/同步映射写接口、Raster 独立程序表/注册接口和 clear-only 世界降级链。Placement 表已私有化，所有消费者使用只读接口。保留日志溢出重建、容量拒收/延后和失败重试，它们是同一协议的正确性恢复；没有增加生产验证层。Vulkan 的资源格式/布局兼容性语义不是新旧业务路径，不作名称替换式删除。

清理后的验证：核心库 2847 通过、1 ignored；主程序 226；真实 GPU Raster 12、host 19、snapshots 10、capture 1、gameplay 6 通过。核心库比清理前少的三项仅测试已删除的 clear-only fallback；原有业务像素、范围空洞、真实完成、内存和长跑断言未放宽。旧同步测试已迁到 FIFO owned 协议，精确断言三段提交/零替换合并，并保留原样的全部字节与空洞比较。

pass 源码核查未发现遗漏的世界图生产迁移：W1–W54 仍是唯一世界图，阴影/主 G-buffer/OC2/late 走共享池 Collect。最新 30 帧真实链实际观察 Collect 90、跨 pass 90；无窗口使用同一 ReadbackPublisher，30 次提交收到第 29 帧深度与 29 次发布，不以开关代替执行证据。更早 chunk=1 的 540/420 以及两次 1060 帧属于各自历史设置范围。

本轮另捕获 pinned winit 的真实 AppKit 全屏失败回调自锁：恢复前未释放 shared_state，恢复函数再次锁同一互斥量。项目内使用同一精确提交的源码，仅补释放锁；锁定依赖及文件哈希有 provenance。真实回调故障注入已返回，实际 OS 全屏往返随后通过。失败日志和线程采样保留。

接下来：完成桌面完整输入录制/保存/回放与交互矩阵的实际窗口验收，统一复验最新源码，再做四场景三轮硬件性能测量。当前新增 CLI 接线使用现有版本 3 Rust 宿主文件协议，不宣称原版 JSON 逐字兼容；旧版本不保留兼容解码路径。证据见 validation-2026-10-24/manifest.json 的 entry_ack_and_compatibility_cleanup 与 pass-migration-audit.json。


### 2026-10-24 桌面完整输入流闭环

删除“仅恢复首帧”的旧 CLI 阻断，桌面进入实际 gameplay App 后通过现有 InputReplayCommand 启动录制或加载；加载/预热 App 不消耗请求。首次真实输入更新后只核对一次启动结果，退出时完整保存一次。使用现有版本 3 Rust 宿主格式，旧版本不兼容解码；这不代表已恢复原版 JSON 字段/编码。取消每个录制帧的整套数值验证，文件接纳/保存边界统一校验，时钟继续保持原版必要的不变量与失败副作用。

真实窗口已录制 180 世界帧/182 完整输入帧，实际绑定/捕获/模拟相机旋转和正常关闭成功。录制期间晚预览冻结，避免文件外的新相机姿态；另一轮实际窗口完整回放全部 182 帧，动作、f32 时间位和捕获锚点逐帧精确一致且恰好一次，181 世界帧后关闭/save/GPU shutdown 成功。桌面 owner 单测 3、相机/历史矩阵 8 通过。矩阵覆盖编辑 subject、笔刷/工具模式、其它数字/模拟动作、菜单、捕获、插值、Vermeer、录制/回放；除正常相机旋转外，这些矩阵是生产入口 CPU 测试，不冒充每种工具的实际窗口编辑/Undo 全链验收。

窗口输入夹具放在项目 tests/fixtures/runtime/render-thread-input-v3.json，并保存运行来源。最新统一回归仍进行中：发现旧空 Replay 测试预期与原版时钟断言相冲突，已改为精确验证失败边界与消费者不执行；并保留失败日志。收尾后再开始正式四场景性能验收，当前 Debug 正确性结果不替代性能数据。


### 2026-10-24 本轮统一回归结果与剩余边界

最新 all-target 检查退出 0；country-core 2850 通过/1 ignored，tiny-glade 230、input_pipeline 17、compile_before_submit GPU 1、core_render_bridge GPU 10、host GPU 19、snapshots 10、实际 gameplay GPU 6 全通过。强制 chunk=1 的 30 帧实际 Collect=540、跨 pass=420；深度回读发布 29 次。前面的失败日志全部保留，不能拿早期源码哈希替代本轮输入范围。记录见 manifest.json 的 unified_followup。

渲染主链已经删除本轮发现的新旧并行路径；扩大回查仍有历史入口需要继续处理：TimeOfDayState 类型别名、旧 startup tuple 的 mesh-only sky helper、与实际 ECS 完成观察器分开的 check_load_completion 纯文件 helper、append_fence_release 会话别名，以及需沿真实消费者确认的 Steam/history 兼容投影。它们尚未全部清理，不能声明全项目已无兼容接口。

剩余按顺序：继续收拢上述历史入口；补需要的实际窗口笔刷/拖拽/放置交互矩阵（CPU 冻结矩阵与 182 帧实际回放已过，尚不代表每种工具/Undo 的窗口业务全链）；随后完成四类场景、三轮、每轮预热后至少 1000 帧的 Release 硬件测量并据瓶颈优化。并行仍显式启用，未作默认策略改变。

### 2026-10-24 历史入口清理与 Release 性能实施

前节列出的历史入口已删除：TimeOfDayState、append_fence_release、纯文件 check_load_completion 包装器和 LoadingCompletionState 镜像、六个 CPU mesh-only 启动函数及其注册表。Steam 只维护 actions_by_type；history 装饰物/楼梯差异只归属具体待提交或已提交条目。完整设备启动仍由 gameplay_startup 的 OnEnter(Game) 链负责。

追加核查删除了全局 STARTUP_DEMO_MODE/set_startup_demo_mode（会话只读当前 World 的 GameConfig）、旧字节/类型音频加载器与路径包装器、MusicPlayer 的重复季节标量、无消费者的 WorldAssetHost/register_mesh_file，以及快照序列化/解压的冗余包装和无消费者的试验解码器。音频测试改走生产 load_session_audio，配置读取失败按已确认的原作默认描述策略核对。原作存档版本迁移属于游戏业务，继续保留。

清理后全工作区 all-target 检查通过；country-core 2847 通过/1 ignored，toolbar 生产菜单 5 通过/1 ignored，加载完成与会话控制各 1 通过。首次命令缺少 TG_INSTALL_ROOT、测试缺少 GameConfig，以及并发接口迁移造成旧 toolbar 测试编译失败的日志均保留；修正测试消费者后复验通过。实现清单和来源见 performance-2026-10-24/cleanup-implementation.json。

现有真实窗口夹具增加测试侧工作负载：静态原始场景、真实 ECS/资产 owner 的多 drawable 压力、间歇与连续局部 Raster 写入（完整 staging/上传/GPU 消费链），以及每进程实际 loading→gameplay 交接。Raster 场景不冒充窗口笔刷/Undo 验收；交接启动耗时单独统计，不冒充 1000 次 App 切换。固定 1280×720、画质、1/60 模拟步长，关闭 vsync 和 FPS 限制。profile_render.py 保存二进制 SHA、源码指纹、原始日志、逐帧吞吐分位数及进程 CPU/RSS 快照；阶段计时、GPU timestamp 和调用栈只在独立诊断运行开启。正式测量进度与已发现失败见后续性能条目；并行默认策略仍未改变。


### 2026-10-24 上传批处理、计时与导入容量复验

上传队列按现有字节/region 预算合并最多 64 个物理目标，共用一个实际 staging 槽与提交；保持同目标 FIFO、优先级、失败 payload 和提交确认语义。GPU 计时配置改由 begin_frame 唯一维护，重复同配置不重置 pending queries。专项库测试 426 通过/2 ignored，上传 10、GPU timing 4、host 27、bridge 10、snapshots 10、compile-before-submit 1 通过，范围绑定 performance-2026-10-24/upload-optimization-implementation.json。

冻结二进制三组静态串行配对（AB/BA/AB，每轮预热 200 + 测量 1000 帧）全部改善。三轮 FPS 中位数 15.45 → 22.91，三轮 p95 中位数 98.75 → 60.28 ms。此批窗口夹具使用系统分配器，后续发现与正式程序 mimalloc 不同，因此不作为正式程序性能验收。数据在 paired-upload/；帧间隔来自真实渲染反馈，不代表 GPU 完成或屏幕延迟。CPU 利用率上升，不能据此宣称 CPU 开销已降低。独立静态和多 drawable 诊断各测量 1000 帧，上传容量等待均为 0；冷启动仍出现少量等待。固定 1280×720 输出/world scale=1，实际 GPU span 已恢复非空。

完整验收尚未通过：baseline 的间歇编辑串行和 loading-switch 并行第 3 轮失败，上传优化版的间歇编辑串行第 3 轮也出现导入 Timeout。所有失败保留，不能按成功样本补成三轮通过。真实两槽 GPU 用例复现了“GPU 已完成但 session 未退役，导入容量未释放”的缺口；容量耗尽时改为现有非阻塞 collect_completed_frames，再重试一次，正常路径不新增检查。已完成回收、在飞保护、上传与 GPU 计时共 18 项专项通过；冻结 V2 Release 的间歇编辑串行和 loading-switch 并行各三轮通过，共 6000 个测量帧、每轮另预热 200 帧，原故障未再出现。范围见 import-admission-implementation.json；其它场景仍由最终矩阵验收。

详细录制诊断的 30 个测量图中，CompleteGroup 累计约 9.69 ms/帧、GroupLockWait 约 0.09 ms/帧；这些是跨录制线程累计且含嵌套区间，不能相加为帧耗时。动态常量锁仍覆盖参数描述符分配/驱动更新，作为下一处缩短临界区的优化对象。系统原生 sample 有效样本很少，只作栈方向证据；不据此发布热点占比或“无死锁”结论。
