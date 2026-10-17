# rhapsody 重构进度（2026-10-17）

本文件是当前实现与验收状态的唯一登记入口。范围/门槛见 [DELIVERY_PLAN.md](DELIVERY_PLAN.md)，下一步工作见 [REFACTOR_PLAN.md](REFACTOR_PLAN.md)。用户提供 Vulkan/MoltenVK 环境后，已恢复本机 Apple M3 的 GPU 与 macOS 窗口验证；Windows 运行验收仍后置。硬件验证不阻止其余源码重构。

## 交付状态总表

里程碑状态：PLANNED / IN_PROGRESS / PASS / FAIL / BLOCKED。具体检查可记 UNSUPPORTED，但必做硬件路径缺验证时里程碑不能 PASS。实现程度另列，避免把“存在类型”当成“已交付”。

| 版本 / 里程碑 | 状态 | 当前证据与主要缺口 |
| --- | --- | --- |
| V0 / M0 | IN_PROGRESS | Win32/macOS 宿主共用生产帧体；Apple M3 窗口 360 帧、4 素材、读回与 resize/minimize/restore 已实跑，校验 error/warning 0；最终屏幕像素与键盘检查缺 Computer Use 权限，Windows 未实跑 |
| V1 / M1 | IN_PROGRESS | 唯一设备、精确 last_use/退役及统一错误已接；MoltenVK 枚举、通用族回退规划与 OPTIMAL 图像初态已修。Apple M3 实测独立队列锁隔离、导入校验、真实 timeline、1000 帧计数稳定；完整目标矩阵/对照仍待闭合 |
| V2 / M2 | IN_PROGRESS | 新增严格呈现能力策略与显式 shutdown；正常生产路径要求 maintenance1，不能证明关闭的兼容路径保留所有权并返回错误。S03/S04 窗口各 360 帧关闭完成且保留代际 0；完整故障与目标平台矩阵仍待收尾 |
| V3 / M3 | IN_PROGRESS | 图分层、类型化写版本与独立检查器已接；Session-owned 命名持久/temporal 槽及 ctx.history_valid 已接 tiny 宿主，旧公开 u64 temporal/owner 和 rg/state 替代路径已删除。新接口 20 项严格 GPU 领域检查通过，最终快照仍须统一复验 |
| V4 / M4 | IN_PROGRESS | 帧/worker/family 池、CPU arena、有界传输/上传任务合并、全局预算及有界常量页已接。固定资源真实 Vulkan Session 的 begin/execute/wait 全部 0 次分配，构图 32 次单列；常规回收配额、传输任务与 CPU 存储预算已有领域回归。宿主与图 native 范围表已统一并通过关键原生回归；旧手工保活和不可失败收集接口已删除，完整最终矩阵及性能验收待完成 |
| V5 / M5 | IN_PROGRESS | 整图/子图/内容结果缓存已接实际执行；碰撞与失效故障用例、实际结果缓存 GPU 检查和 S06 360 帧像素检查通过；最终冻结源码/性能矩阵待完成 |
| V6 / M6 | IN_PROGRESS | 默认 suballocator 的瞬态 backing 已接唯一所有权和真实 native spare 预算；native/alias 9 项严格 GPU 检查通过。更大 S05 案例 360 帧确认 native 保留内存从 576 MiB/3 blocks 降到 320 MiB/2 blocks；最终快照及性能 A/B 仍待复跑，优化默认关闭 |
| V7 / M7 | IN_PROGRESS | VkRenderPass/VkFramebuffer 及其缓存/Legacy 生产路径已删除；光栅要求已启用动态渲染。全局 heap 的正式 shader ABI、参数块和真实资源租约已接 tiny 与文字消费者；完整 shader 资产转换、热重载和参数 ABI 领域检查通过。重复编辑脚本、冷启动测量与最终性能矩阵待验；旧 A/B 二进制保留作独立参考 |
| V8 / M8 | IN_PROGRESS | 按真实 QueueId 分批、配对 ownership、跨队列 timeline 与 Concurrent 范围依赖已接；真实三族图 4 项 GPU 回归及 Exclusive/Concurrent 窗口各 360 帧通过，须复验最终统一接口快照 |
| V9 / M9 | IN_PROGRESS | managed checksum/refcount/资源序号已改 typed weak store；BufferRecord/SlotView、TextureRecord、原始字节桥、每帧 descriptor arena 及未接线 CPU FrameRing 已删除。命令使用 typed RecordingId；PGO 名称改为共享且可释放的 Arc 字符串。最终全特性/默认/无默认矩阵、三种 30 分钟长跑与 300/3000×3 性能对照尚未执行，不能登记完整交付 |

当前没有 M0–M9 的完整 PASS。P0 及历史测试通过只证明当时覆盖的范围，不提升本表里程碑。

### 最新收口与审计发现

- [正式渲染性能模式](README.md#渲染框架性能运行模式)：`tests/p0_perf_baseline.rs` 新增同场景的 `render_perf_prepare` / `render_perf_strict_acceptance` / `render_perf_throughput`。性能路径默认 300 预热、3000 采样、3 次重复、3 帧在飞；只在采样帧异步读回，不在每帧等待 GPU。报告单列 CPU begin/build/compile/record/submit、帧槽等待、GPU command/pass span、帧间隔 p50/p95/p99、CPU/GPU 每帧分配和峰值。吞吐入口拒绝 debug、validation layer、缺失/过期验收；参考与验收写入 `target/render-perf/`。Window Lab 增加 `--vsync on|off`，报告记录请求值和实际 `present_mode`。2026-10-17 在 Apple M3 / MoltenVK 1.4.2 / release 实测：严格验收 8 帧通过（最大 1 个边缘像素差、非法像素 0）；headless 3×3000 帧为 3135/2982/1823 FPS，帧 p50 231/225/348µs、CPU p50 64/73/161µs、帧槽等待 p50 159/142/22µs，每轮 50/50 抽样读回通过、约 66 次 CPU 分配/帧、1 次 GPU 分配/帧、GPU 峰值 197,520B；64×64 场景 GPU command span 低于 MoltenVK 计时分辨率（报告记录零跨度和采样缺失）。cache off 对照为 2745/3213/2979 FPS、734 次 CPU 分配/帧、compile p50 约 39–49µs（cache on 约 1.9–4.6µs），50/50 读回通过。Window Lab showcase vsync on 150 帧 p50 16.51ms / p95 16.98 / p99 17.65，图框架 p50 0.084ms、build+begin 4.25ms、present 11.95ms；vsync off 实际 `IMMEDIATE`，150 帧 p50 3.97ms / p95 20.52 / p99 23.46，1024 帧运行 724 个稳定样本 p50 7.93ms / p95 12.37 / p99 12.84 后在 present/FrameLoop 等待超时（validation 0），该失败保留，不当作通过。测试同时修复了 timing 查询未收割时命令缓冲被复用（`command_for_worker` 跳过仍被追踪的命令）、performance 模式关闭 validation 时未启用 debug utils、以及测试聚合长期持有 ReceiptStorage 槽位导致的 Timeout。Window Lab 报告目前只把 GPU timing 样本匹配到最终合成图，四路 producer 的 GPU 归属仍不完整；V9 性能门仍未登记 PASS。
- [综合播放器性能定位](reports/refactor/2026-10-17-v9/showcase-profile/README.md)：原 debug 实测约 7 FPS，主要耗在 CPU 建筑逐像素/矩形参考；改独立行区间 painter 与完整版本复用、旋转覆盖掩码缓存，保留每帧完整像素比对。同 debug 120 帧约 19.6 FPS；仅播放器 opt-level=3、框架/依赖仍 dev 时约 38.7 FPS，三次 validation error/warning 均 0。25 项 CPU 回归含 64 组新旧 oracle 对照通过；这只是短程定位，不是 300/3000×3 正式性能门。后续主要空间为上传准备、合并四路图/提交和异步校验流水线，尚未实现。
- 按用户要求新增并默认启用持续综合场景 `showcase`：每帧共同运行细分旋转网格/后处理、真实上一帧历史预览、三队列依赖链、持续循环的局部建筑/阴影/字形版本更新及大批上传，GPU 合成后逐字节核验。普通播放低频穿插窗口变化，高频压力仍显式选择。24 项 CPU 与严格 Clippy 通过；首轮 360 帧四路 producer/像素验证、3 物理队列、11 上传轮次、141 次缩放和 28 次最小化恢复均通过，validation 0、退出资源归零。[最终默认入口 960 帧复验](reports/refactor/2026-10-17-v9/showcase-final/validation.json) 已通过：四路各 960 帧、历史 960 次、效果发布 3 次、上传 29 轮、3 物理队列、窗口尺寸事件 6 次并完成最小化恢复；validation error/warning 均 0，退出资源/CPU/预算归零，构建与运行后源码指纹一致。此为综合功能验收，不代替 30 分钟长跑与性能门。此前旧场景的长跑存在失败记录，不能沿用启动状态或功能通过声称长跑完成。
- [当前 Window Lab 功能验收](reports/refactor/2026-10-17-v9/final-window-retention/checks.json) 17/17 场景 PASS，覆盖素材、WSI 压力/重复恢复、动态光栅/重复热重载、temporal、transfer、缓存、aliasing 与 Off/Prefer/Require Exclusive/Concurrent；实际像素、校验和退出资源检查全部通过。稳定、效果、缩放三种 30 分钟且至少 10000 帧长跑已启动，完成前不登记 PASS；三进程并行只用于正确性压力，不作为性能数据。
- [当前下游验收](reports/refactor/2026-10-17-v9/final-consumers-retention/checks.json) 默认/无默认全目标、三包全目标、country 全特性库、country 渲染 393 / render-loop 644 / pass-kit 12 / shader-factory 11、严格 Clippy、tiny GPU 26 与文字 GPU 2 全部 PASS。shader 包测试 22 项退出成功，但两项 DXC 实编译因编译器缺失跳过，整个 shader 检查仍为 BLOCKED。完整三配置矩阵为 629/626/620 项无测试失败且 validation 0，期间共享 country 源码变化，仍保留 INVALID_SOURCE_CHANGED，不提升为冻结快照 PASS。
- 所有 rhapsody 与 tiny/text 下游旧 `keep_alive*`、`collect_uses`、`track_borrowed` 调用已迁移且无兼容别名。图访问自动持有真实资源，查询池/管线等对象走拒绝 memory domain 的 `retain_object`；受控视图与 bindless 租约先校验实际声明或 raw Access patch 再登记。RawCB owners/uses、各资源独立 SyncCell Arc 都已接真实容量预算，失败保持错误类别并阻止提交。
- tiny 普通上传与结果缓存 miss 改为构图前导入精确 staging 范围；country 多源 reset 保留单 pass 的显式 copy 数据包。索引绘制在声明期导入实际 index bytes，录制不再从宿主表兜底取 buffer；新增释放宿主句柄后仍正确绘制的真实下游回归，等待当前下游重编验证。
- tiny compute/raster 的外部 buffer、sampled/storage image 也已迁到声明期解析与帧作用域导入，精确范围、视图和布局进入正式访问声明，回调只解析图内资源。[下游严格 GPU 复验](reports/refactor/2026-10-17-v9/retention-consumers-followups/manifest.json) 为 raster 10、compute/上传宿主 14、文字 2 项全部通过，validation/SKIP 0，构建及运行源码指纹稳定。包含实际读取外部 SSBO/纹理、写 storage image、索引绘制及宿主句柄释放后的像素检查。首次 raster 的 9 条 VUID 原始记录保留；修复以实际已启用 geometry/tessellation 特性正规化同步范围，未修改原始访问表。
- 无实际生产消费者的旧 `DescriptorSetState`、PoolQueue/Lease/DiscardPile、descriptor 写入/绑定转换旁路及 Device/country 转发已删除。正式 bindless 保留有界 Pool/Set；持久 Set owner rows 在 native update 前完成 fallible 准入。内部 descriptor/command pool、shader/pipeline、semaphore 与内建 sampler 缓存的独立 CPU 分配补入同一预算，最终测试待合并复验。
- [迁移审计映射](reports/refactor/2026-10-17-v9/retention-migration-audit/README.md) 列出已删除入口、实际消费者、新协议和 native 测试。完整矩阵首轮另发现旧回读 fixture 的 Opaque 状态及两个过时的队列拒绝预期，已保留[失败与中断记录](reports/refactor/2026-10-17-v9/final-matrix-retention/interrupted.json)。纹理回读已迁正式精确访问并保持 12 项全部字节断言通过；[实际队列交接复验](reports/refactor/2026-10-17-v9/retention-queue-followup/manifest.json) 3 项通过，核验真实 release/acquire、提交回执、原生等待与最终范围所有权，validation/SKIP 0。当前继续完整矩阵复验。
- 图 native 范围同步进入统一接线：实际资源 `SyncCell` 持有唯一范围状态，静态 RangeProgram 与每帧 BoundSynchronization 驱动串行、worker 和 ownership 录制；旧 ResourceStateCell、提交事务外的物理状态发布及 recorder 的 whole-resource 推导已移除。原生 setup/texture upload、bindless 和两个 CPU 故障/分配适配器同步迁移。新增混合 mip/layer 布局、部分批次失败恢复、真实不同队列族范围 ownership 及 bindless 声明覆盖用例；[范围与资源登记领域回归](reports/refactor/2026-10-17-v9/retention-migration/manifest.json) 已构建并执行：CPU lib 311、真实三族范围与 partial 恢复、上传、bindless、帧作用域导入、资源登记容量/失败、附件预算、常量页及 CPU/GPU 执行零分配门全部通过；无 validation/SKIP，源码及二进制身份已保存。此后仍有窄修，不替代最终冻结验收。
- 管线缓存文件的临时路径冲突不再清理非本次创建的文件；query owner 与独立 SyncCell 的真实 CPU 存储补入预算。[短组领域回归](reports/refactor/2026-10-17-v9/final-short-followups/checks.json) 为 CPU 3+1、真实 GPU timing 4/cache 4、Window CPU 24 全部通过、零 validation/SKIP。运行时未注册的新范围模块草稿仍在变化，整源码指纹不稳定；保存二进制只证明这一短组，不能提升为最终冻结矩阵。
- 统一最终矩阵首次运行因独立审计发现光栅缓存键长度错误而中断，见 [保留记录](reports/refactor/2026-10-17-v9/final-matrix-interrupted-cache-key/interrupted.json)。不能把这轮部分成功登记为最终快照。
- V7 修复光栅同键无法合并的问题：[真实并发回归](reports/refactor/2026-10-17-v9/v7-raster-key/async-pipeline-strict.json) 包含 8 个并发及 64 次后续请求，只编译一次，预算不重复累计。compute 工作组大小加入热重载执行合同；[11 项异步管线检查](reports/refactor/2026-10-17-v9/v7-local-size/async-pipeline-strict.json) 覆盖三维变更拒绝、旧版本保留和恢复。
- 磁盘管线缓存读取、提取均使用携带实际 capacity 预算的字节对象；保存流式写固定头与驱动数据，避免第二份大副本。4 项实际 GPU 缓存检查通过，包含预算不足保留原文件与数据跨 cache/device 所有者存活；仍待统一最终矩阵归档。
- [常规回收配额报告](reports/refactor/2026-10-17-v9-reclamation/VALIDATION.md)：CPU 263 项、实际 GPU 14 项领域检查通过。normal/pressure 配额限制每次维护 tick 的扫描与销毁，已提交对象不再在 Drop 偷跑回收；显式排空与关闭保留完整语义。未提交对象仍立即释放，不能据此宣称所有 Drop 的全帧总耗时都有上限。
- [上传任务领域证据](reports/refactor/2026-10-17-v9-transfer-queue/native-build-manifest.json)：规范化键、脏区合并、优先级、epoch 取消、真实成功点发布与 readback lease CPU 预算已接；Tiny BufferHost 的实际稀疏 GPUOnly 更新通过大于单槽容量的分批、空洞保留与错误不提交检查。该二进制之后源码继续变化，证据不替代最终冻结矩阵。
- 下游完整 CPU 上传源已接正式 subgraph/result 缓存，以完整 bytes 和显式 source/version 作为合同；未知业务依赖仍走明确 uncached 路径，不自动缓存任意 compute/raster 结果。实际 Tiny/country 7 帧 A/B 检查通过，最终归档和统一矩阵待完成。
- 逐 pass GPU query 已接串行及 worker 真实录制，命令 span 与 pass span 分开；失败取消/复用领域检查 3 项通过。Window 报告保留最多 1024 条带 pass 名称的样本，性能历史 A/B 使用相同 command-only 观测配置。队列取锁耗时与退役年龄新增观测，最终合并复验待完成。
- 屏障现用单份 scope 依实际能力分派 Barrier2/Legacy；新 Legacy 图测试发现 NONE 阶段不合法，已修回退转换，修后严格对照仍待归档。当时独立审计发现的 whole-resource 图 native 缺口已由唯一范围表和 BoundSynchronization 替换；当前已删除旧 initial/pre/export 诊断投影，后续统一矩阵须验证最终源码。
- 验收工具补入实际 playlist/PPM 素材指纹、A/B 素材/读回/设备一致性和长跑末段 CPU/native 内存包络增长门；增加独立 subgraph/result 性能 A/B 及 Prefer 队列场景。最终 300/3000×3 与三种 30 分钟长跑尚未执行。

## 本轮收敛中的新增证据

此轮源码仍持续变化，以下是领域验证，不替代最终冻结源码矩阵：

- V6 默认 native suballocation / transient aliasing 的 9 项严格 GPU 检查、回滚/碎片与独立释放检查通过，见[运行摘要](../../reports/refactor/2026-10-17-v6/native-suballocation-summary.json)。实际 native block 容量按依赖分配器统计；小预算走现有 dedicated 策略，不声称依赖支持任意 block size。
- S08 已在实际 graphics/compute/transfer 三族运行：[Exclusive](../../reports/refactor/2026-10-17-v7/s08-exclusive/report.json) 与 [Concurrent](../../reports/refactor/2026-10-17-v7/s08-concurrent/report.json) 各 360 presented / 360 verified、同步校验 error 0、关闭完成且保留 WSI 代际 0。报告中的 GPU 数值是各队列命令 timestamp span 的和，不能解释为整帧 GPU 延迟。
- 原文字 shader 的真实 solid/glyph 非零像素、透明区域及冷暖结果一致检查通过，见[动态路径日志](reports/refactor/2026-10-17-v6/tg-text-dynamic-gpu.log)。曾发现并修复硬编码 binding、原 runtime 8192 超出 layout 支持，以及测试夹具逻辑坐标尺度错误；原失败日志保留。其正式全局 heap 接线仍须复验。
- 完整上传已覆盖立方体全部面、cube array、奇数尺寸小 mip、全部 BC 格式的公共格式布局与 staging 对齐；GPU 计时环和多队列图通过对应领域检查。新增 CPU 用例发现 multi-plane 误接入，已明确拒绝，最终整组复跑待完成。
- V3 命名历史资源：类型化资源、失败/重试、reset/resize、共享方式及真实 48 帧历史读回等 20 项严格 GPU 领域检查通过，见[命名 temporal 报告](../../reports/refactor/2026-10-17-v9-named-temporal/README.md)。下游分配错误桥已改为 fallible，最终合并快照复验待完成。
- V9 托管资源改用统一 `ResourceStore` 的 typed key 和 Weak slot，避免设备资源强引用环；本轮托管/图基础与 lib 共 266 项 CPU 检查通过。随后删除的 Buffer/Texture 字节桥及描述符 arena 仍须纳入最终矩阵，不能沿用此前测试计数作为新快照结论。
- [帧表分配报告](reports/refactor/2026-10-17-v9-frame-storage/REPORT.md) 保存 CPU ExecutionChain 的实际 System 分配计数及前后诊断；报告明确区分图构建、CPU 执行和仍须验证的实际 Vulkan Session。随后 PGO 名字所有权迁移需要复跑同一硬断言。
- 实际 Vulkan Session 的新基线已用 2 个真实导入 buffer、8 个 copy pass、300 帧预热和 128 帧采样测得：execute 为 86 次/9758B，begin_frame 为 0，构图 32 次/3467B、完成等待 2 次/64B 分列；双 buffer 的 64B 输出均一致，验证层 error/warning 0。[基线计数](reports/refactor/2026-10-17-v9-frame-storage/real-session-baseline-counts.json) 与匹配二进制/栈另存。此结果证明 CPU adapter 的 0 次不能代替生产路径；正在分别收口访问表、只读回执、资源投影与提交临时存储。
- 同一真实 Session 夹具随后按栈修复为 27 次/2912B，再到缓存执行、begin_frame、完成等待全部 0 次/0B；每轮均有二进制身份和计数，见[0 分配领域结果](reports/refactor/2026-10-17-v9-frame-storage/real-session-ownerpool-counts.json)。另以强制零分配硬断言复跑通过；构图仍为 32 次/3467B 单列。此构建之后的 k-way 合并、提交 API 配置及最终统一快照仍须复跑，诊断 p95/p99 不替代安静环境的 V9 性能门。
- [重复窗口领域报告](reports/refactor/2026-10-17-v9/window-repeat-domain/README.md)：900 帧实际完成 3 轮错误/合法 shader 编辑；另一次 900 帧完成 368 次 resize 和 73 次最小化/恢复。两次验证层 error 0，关闭后已登记资源、CPU 存储和预算归零。该构建早于实际 Session 最后收口，不能替代最终长跑。
- [托管元数据预算报告](reports/refactor/2026-10-17-v9-metadata-budget/README.md) 保存 27 项领域检查（20 项实际 Vulkan、7 项 CPU），包含 typed weak slot/inner/raw payload 的容量计费、在飞视图缓存、失败不发布与最后所有者释放。预算对象持有统计而非 Device，错误保留 OOM/Timeout 类别。
- 正式全局 shader ABI、热重载逻辑 ABI、设备丢失、完整上传/计时/帧池及 tiny compute 实消费者领域组通过；实际 core Raster 8 项包含 30 帧两在飞动画、双绘制顺序深度重叠及类型/错误检查，像素差 0。原文字 2 项与旧路径两个真实 4096B 参考逐字节一致，见[文字共享 heap 报告](reports/refactor/2026-10-17-v9-text-bindless/README.md)。
- [bindless 领域报告](reports/refactor/2026-10-17-v9/bindless-domain/README.md) 保存实际二进制、指纹与初始夹具失败/修复。实际本机素材 361 包/575 stage、B 素材 404 包/638 stage 全部转换并通过 Vulkan 1.2 SPIR-V 校验；六个正式窗口场景各 360 帧通过，含热重载、历史、两种三族共享及大 S05。此构建早于随后 FrameStorage/RecordingId/CPU 对象预算改动，不能替代最终矩阵。
- `tools/refactor_checks.py` / `tools/window_lab_checks.py` 保存二进制身份、源指纹与有效性；缺环境、验证层错误、源变化和跳过均单独记录。屏幕像素/键盘权限与 Windows 仍保留 BLOCKED。

## 2026-10-17：图分层、资源契约与后续开发

本节接续此前 V1/V2 断点，开发进行中。后续源码仍在变化，下面的具体运行仅证明对应二进制；待本批收敛后统一保存最终源码指纹与完整矩阵。

- 图规划与物理绑定分离；计划不保存 token、实际资源、回调、导入初态或完成值。ExecutionChain 缓存不可变计划，每帧验证导入与契约并重新绑定边界。未消费的纯 pass 被裁剪，回调按原始索引映射，副作用显式声明。
- 图句柄带作用域；typed BufferId/TextureId 保存写版本。首次读取、过期版本、范围越界、跨图绑定及无来源导出在提交前拒绝；0x36/0x37 读改写保留旧内容来源。usage 推导不再对调用方的非法图像访问 panic。
- 独立检查器使用空间分区重放内容来源，独立核验依赖可达性、反依赖、裁剪、符号屏障、生命周期、usage 和批次；256 组包含部分区间读写/读改写的小图与保守解释器输出一致。
- 首轮真实 GPU 检查发现 temporal 首帧误发布未初始化 previous 成员，已修成只确认完整初始化且实际导出的成员。生产 temporal 三角形 48 帧参考像素、47 次历史逐字节校验通过，见本批 `temporal-triangle-gpu.log`；部分写及失败帧不会伪造有效历史。
- [S03 报告](reports/refactor/2026-10-17-v3-v4/s03/report.json)：360 帧真实光栅、compute 后处理和参考验证，脚本完成，验证层 error 0；显式呈现 shutdown 完成、保留代际 0。
- [S04 报告](reports/refactor/2026-10-17-v3-v4/s04/report.json)：360 帧 GPU 当前/上一帧历史读回，显式 reset、录制失败与下一帧恢复检查，验证层 error 0；显式呈现 shutdown 完成、保留代际 0。最终屏幕像素仍受 Computer Use 权限阻塞。
- V4 正在把真实命令池/描述符池接到帧域，并以资源提交门闭合“读取完成快照后出现新提交”的宿主读写竞态。Buffer 便捷读写等待本资源，裸映射入口改为显式 unsafe；异步传输使用固定容量的独立 mapped 槽，读回 ticket 存活时不能复用其槽。

本节不把开发中的 V4–V8 或尚未运行的最终矩阵登记为 PASS。

### V4–V7 新增真实运行证据

- [池、预算与传输 GPU 日志](reports/refactor/2026-10-17-v3-v4/pools-budget-transfer-gpu.log)：7 项真实命令/描述符池测试、2 项异步传输测试、1 项内存生命周期测试通过。在飞/retiring 资源持续计费，完成前不允许 reset 或槽位复用；受限预算 OOM 后等待实际完成可恢复。
- [S07 窗口报告](reports/refactor/2026-10-17-v3-v4/s07/report.json)：360 帧传输与呈现；两槽容量每帧检查耗尽，89 次恢复，178 个延迟读回块通过；实际 staging committed 4 MiB，验证层 error 0，呈现关闭完成且保留代际 0。
- [V6 GPU 日志](reports/refactor/2026-10-17-v6/transient-aliasing-gpu.log)：4 项测试通过；已知可复用案例 committed 从 65808 降到 49296 字节，实际单次 backing 分配/释放，像素一致；同时 pending 的两帧 backing 独立，独立 DAG 分支不别名，部分提交不会发布后续别名的初态。
- [动态渲染 A/B](reports/refactor/2026-10-17-v7/dynamic-compare/report.json)：360 帧，动态/传统各 180 帧，180 组相同输入读回逐字节一致；[纯动态路径](reports/refactor/2026-10-17-v7/dynamic-only/report.json) 360 帧，旧 render pass/framebuffer 计数均 0。两次运行校验 error 0，关闭完成；屏幕像素检查仍为 BLOCKED。
- [异步管线 GPU 日志](reports/refactor/2026-10-17-v7/async-pipeline-gpu.log)：6 项测试通过，包括并发同键合并、有界 key 表、淘汰后的管线仍可执行、损坏磁盘缓存重建、错误编辑保留旧管线、接口不兼容拒绝及显式帧边界发布。完整窗口 shader 脚本和 bindless 尚在验证。

## 基础模块与生产接线核对

| 模块 | 当前程度 | 不能由此推断的完成项 |
| --- | --- | --- |
| `sync/`、`device/queue_config.rs` | 已接生产提交/回收基础；有历史验证 | 不代表资源都使用精确 last_use 或完整多队列图调度 |
| `device/{caps,policy,generation,init}.rs` | 配置与能力已在当前工作树接创建设备 | 设备本体及创建策略已合并；目标硬件复验仍后置 |
| `resource/{handle,tracked,sharing}.rs`、`rg/temporal.rs` | typed weak store、SyncCell、图 ID 与 Session-owned 命名历史已接实际宿主 | 最终合并源码仍须统一复验 |
| `rg/vulkan.rs` 的 `FramePoolKey` / `FrameCommandPool` | 真实帧 × worker × family Vulkan 池，完成前拒绝整池 reset | 原独立 CPU FrameRing 无消费者，已删除；Session 稳态分配仍在收口 |
| `device/budget.rs`、局部池与 raw/managed 资源 | native backing、CPU 元数据、缓存与在飞/退役容量已接实际预算 | 不可见驱动分配不纳入本账本；最终压力/长跑与性能矩阵待验 |

## 2026-10-17：V1 收敛与 V2 生产接线

本轮前半继续现有工作树，不回退其他任务的改动；当时 GPU/窗口检查尚未恢复。以下先保留源码交付记录，随后补充 MoltenVK 实测，不把存在类型、CPU 测试或宿主源码类型检查解释为 M1/M2 完整 PASS。

- **V1 旧机制删除**：删除 `raw/disposal.rs`、独立通用 ResourceManager 的兼容桶、旧队列选择模型、设备级闲置命令池副本、未消费的一次性 GpuGraphRun 和重复查询实现；共享设备与显式设备选择入口统一使用同一能力协商。`queue_plan` 不再是可缺失状态。
- **V1 错误与所有权**：公开 shader、descriptor、命令、设备等待、调试与查询入口使用统一错误，保留实际 OOM/DeviceLost；池耗尽/碎片/NOT_READY 只驱动相应池控制流。SPIR-V 头和 raster 参数前置校验。命令池/描述符池底层分配先于最后的 Device 引用析构，延迟副本仍由 Device 在销毁前排空。
- **V1 最终入口审查**：共同提交入口检查所有设备时间线等待，拒绝尚未提交的值，覆盖独立 raw 命令提交。附件超限、重复深度、非法纹理 builder、尺寸除零、动态常量页越界与帧缓冲尺寸错误返回 InvalidUsage；CubeArray 层数使用 checked_mul，图像/视图描述、附件、上传与宿主完整传播 Result。未使用的格式 panic wrapper 删除。显存剥离 Device 引用仅允许 crate 内部执行。
- **V1 句柄并发**：每一代槽位有独立 ControlBlock，get 只修改已升级 inner 的引用计数；旧 inner 不能改变复用槽的新一代计数。代际耗尽不复用，实际组标识拓宽为 u64，历史 ABI 投影超范围显式报错。
- **V2 事务**：FrameTransaction 在录制前准备批次状态与回执容量；成功提交后发布固定存储中的完成点/物理访问状态。部分提交返回结构化执行记录，会话保留在飞资源与退役回调；取消未提交命令不等待成功批次。主错误优先，退役回调即使有一个 panic，其余仍各执行一次。
- **V2 下一帧合同**：Session 与直连 ExecutionChain 共享声明预检和部分失败布局恢复；同 token 或同物理 handle 的重复导入在提交前拒绝。部分失败历史失效，新历史仅在成功确认导出后有效；pair parity 在确认时提交、取消时恢复。triangle_animation_graph 改走生产 temporal 会话。
- **V2 呈现**：按图像持有 present 二值信号量；maintenance1 fence 与无扩展 reacquire/代际证明分开登记。取消 acquire、已消费后的 present OOM、DeviceLost 和活动租约阻止重建分别处理。Window Lab 使用 `FrameLoop.execute` 连接主图与展示，country-core 执行宿主使用 VulkanGraphSession。
- **V2 清理与背压**：render、cancel recovery、present OOM cleanup 的成功提交点立即进入固定容量 GPU 使用集合；清理等待超时仍保留其所有权。WSI/fence/补救调用观察 DeviceLost 后锁定设备终态，后续操作不再调用 GPU。替换前先收集旧代际证明，未完成旧代际上限为 4，满时返回 Timeout 并保留当前链，重试继续收集；不以 queue-idle 伪造呈现完成。
- **V2 帧结果与故障注入**：公开 FrameOutcome，以实际代际变化区分 Recreated、无图 Skipped 和正常 Presented；Window Lab 依据正式结果记录重建，包括重建后暂未取得图像的情况。生产 execute 直接调用可注入的内部帧协议；CPU adapter 验证 acquire/graph/display/present/cancel 各阶段错误、display/retire panic、回调恰好一次、主错误优先、普通失败后下一帧恢复与 DeviceLost 终态。底层真实 Vulkan/窗口故障矩阵另行验证。
- **查询收尾**：查询池由命令实际保活并按 last_use 退役。end 只录制时间戳，完成后无 WAIT 读取；不会在命令尚未提交时同步等查询。显式计时入口可用，尚未自动为每张图插入首尾计时。

实现限制单列：无 maintenance1 且呈现引擎不再提供可用证明时，数量背压保证有界，但不能保证继续重建；当前生产默认要求 maintenance1，显式兼容策略无法证明关闭时返回错误并保留所有权，已删除旧 queue-idle teardown fallback。这一能力边界不能被当作目标平台验收通过。运行时代际证明按 [Khronos swapchain recreation](https://docs.vulkan.org/samples/latest/samples/api/swapchain_recreation/README.html) 建模，maintenance1 fence 与失败入队分类分别参照 [呈现 fence](https://docs.vulkan.org/refpages/latest/refpages/source/VkSwapchainPresentFenceInfoKHR.html) 和 [queue present](https://docs.vulkan.org/refpages/latest/refpages/source/vkQueuePresentKHR.html)。

本轮最终命令、源码指纹与完整日志见 [checks.json](reports/refactor/2026-10-17-v1-v2/checks.json)，复跑入口为同目录 `run_checks.py`；旧日期日志只证明旧日期对应源码。故障覆盖包括首批/后续提交失败、主错误保留、部分终态、重复导入拒绝、temporal 失效与 parity 回滚、退役回调 panic、跨代引用隔离和呈现状态分类。

### 2026-10-17 最终源码检查

环境：macOS `aarch64-apple-darwin`，rustc/cargo 1.98.1，真实 workspace 离线检查。进程结果包含缺 Vulkan/DXC 时的提前返回，不能直接解释为相应 GPU/shader 编译路径通过。

| 检查 | 结果与范围 |
| --- | --- |
| rhapsody 全特性 lib/tests | 390 passed、0 failed、1 ignored，43 个测试进程；包含环境跳过 |
| rhapsody 默认特性 lib/tests | 387 passed、0 failed、1 ignored，43 个测试进程；包含环境跳过 |
| shader-compiler / shader-pipeline 全特性 lib/tests | 22 passed、0 failed；DXC 缺失路径不计实际编译验证 |
| rhapsody 默认 / 无默认特性全目标 check | 均通过 |
| rhapsody + tg-text 全特性全目标 check | 通过 |
| rhapsody 严格 Clippy | 全特性 lib/tests、`--no-deps -- -D warnings` 通过 |
| Window Lab Windows 宿主源码类型检查 | 通过；临时 wrapper 在 macOS 编译 app/win32 源码后删除，只验证源码类型，不认证 Windows ABI、链接或运行 |
| 含 tiny-glade / country-core 的消费者检查 | FAIL：当前并行恢复的 terrain 源码有 4 项编译错误：`draw_terrain_stroke.rs` 未导入 `ui_edit_terrain`，以及 `ui_edit_terrain.rs` 的 hovered.id / stroke_id i64-i32 比较与赋值冲突。本轮未修改这些文件；tiny-glade 最新消费者检查仍待外部错误解除 |
| 格式与补丁检查 | 本轮关键源码 rustfmt 检查及 `git diff --check` 通过 |
| Windows / Vulkan / 性能 | BLOCKED，按用户要求后置；没有新增硬件 PASS |

共享工作树在矩阵期间仍有 country-core `ssbo.rs` 的并行改动，因此报告的整组 `source_unchanged_during_matrix` 为 false；每条命令保留各自的前后指纹，不把这组检查宣称为整个工作区静止快照。rhapsody 源码已冻结，后续 Window Lab 源码检查及严格 Clippy 对应最终指纹；外部消费者的失败仍保留。

### 2026-10-17：MoltenVK 恢复实测与后续修复

以上 CPU 矩阵是本轮前半的快照。本节接续用户新提供的 Vulkan 环境：沙箱外 `vulkaninfo` 已确认 Apple M3、MoltenVK 1.4.2、Vulkan Instance 1.4.363 / 设备 1.4.357、Khronos 校验层 1.4.363。普通执行沙箱无法访问 Metal，不能把该权限限制解释为安装失败。组件与原始能力记录见 [设备摘要](reports/refactor/2026-10-17-moltenvk/vulkaninfo-summary.txt)、[完整能力](reports/refactor/2026-10-17-moltenvk/vulkaninfo-full.txt) 和 [shader 工具版本](reports/refactor/2026-10-17-moltenvk/shader-tools.json)。

本轮新增源码接线：

- 实例自动成对启用 portability 枚举扩展与 flag，补齐 surface capabilities 扩展依赖，并验证最终扩展集合；headless 自动补入依赖不误启用 WSI maintenance。设备已有的 portability subset 协商继续复用。
- 队列规划保留专用族和 graphics 族空槽优先，再使用其它通用族；M3 的 4 个通用族各 1 队列能实际创建 graphics/compute/transfer 三个独立队列。新增真实回归证明 graphics 持锁时 compute 仍能提交并完成。
- OPTIMAL 图像统一从 UNDEFINED 创建，与 staging 上传首屏障一致，修复完整 GPU 回归发现的 VUID-12478。迁移 mapped-buffer、UUID 及部分提交测试中残留的旧 panic / 立即退役预期。
- 关键 GPU fixture 支持 `RHAPSODY_REQUIRE_VULKAN=1`，初始化失败直接失败，不再静默早退。真实 timeline 测试复用生产设备创建，移除另一套裸初始化。
- Window Lab 新增 AppKit/CAMetalLayer 宿主；Win32/macOS 共用事件回调内的原帧体、FrameLoop、素材和报告。`--wsi-stress` 与原 `--script` 独立，按观察到的 extent 记录 100 次 resize 和 20 次完整最小化/恢复，不将发出请求计为完成。

| 本机检查 | 实测结果 / 证据 |
| --- | --- |
| 全特性 lib/tests + Khronos / 同步验证 | 401 passed、0 failed、1 ignored，44 个测试进程；日志没有 Vulkan validation error / sync hazard。包含 CPU 用例，不能称为 401 个 GPU 测试；唯一 SKIP 文本来自故意注入缺 loader 的 CPU fixture 回归。见 [完整日志](reports/refactor/2026-10-17-moltenvk/all-features-gpu-final.log) |
| 默认特性 lib/tests + Khronos / 同步验证 | 398 passed、0 failed、1 ignored，44 个测试进程；见 [默认日志](reports/refactor/2026-10-17-moltenvk/default-gpu.log) |
| 核心 V1/V2 GPU 回归 | 实际设备配置、跨设备导入、timeline、延迟退役、部分失败后恢复均通过；真实独立队列锁隔离通过。见 [核心日志](reports/refactor/2026-10-17-moltenvk/core-gpu-final.log) |
| 1000 帧会话稳态 | 在飞相位 resources 7→7、commands 2→2；排空相位 resources 0→0、pending frames 0→0；缓存命令保持 2。见同上日志 |
| 光栅 / temporal / 辅助队列 / 校验层 | 三角形像素、temporal 乒乓、辅助队列执行及大图验证通过；正向验证报告 error/warning 0。见 [光栅日志](reports/refactor/2026-10-17-moltenvk/raster-validation.log)、[同步验证](reports/refactor/2026-10-17-moltenvk/synchronization-validation.log) |
| macOS 窗口基线 | 360 presented / 360 verified、4 素材、resize 4 / recreate 3、最小化恢复脚本完成，validation error/warning 0。见 [报告](reports/refactor/2026-10-17-moltenvk/window/report.json)、[GPU 读回](reports/refactor/2026-10-17-moltenvk/window/readback.png) |
| macOS WSI 压力 | 1000 presented / 1000 verified；100 次 resize、20 次最小化及 20 次恢复均观察完成，140 条有界事件记录；同步验证 error/warning 0，含 teardown。见 [压力报告](reports/refactor/2026-10-17-moltenvk/wsi-stress/report.json) |
| 构建 / lint / 消费者收尾 | 无默认特性全目标 check、rhapsody/tg-text/tiny-glade 全特性全目标 check、另含 country-core 的四包 lib check 均通过；rhapsody 全特性全目标严格 Clippy 通过；Window Lab CPU 测试 6/6。此前 terrain 的并行编译错误已由外部工作解除，本轮没有修改其源码 |
| 最终屏幕像素与键盘检查 | BLOCKED：Computer Use 返回权限未授予；不以离屏读回冒充屏幕截图。两次额外可见运行的自动读回通过，但未登记键盘/人工显示 PASS |
| MoltenVK headless 耗尽超时 | 未验证：不再通过超额 acquire 构造不具可移植性的耗尽条件；合法首个 acquire 的零超时转发及独立于 graphics 锁已实跑。未在生产伪造 NOT_READY 掩盖驱动行为 |

`VK_LAYER_VALIDATE_SYNC=1` 用于最终完整回归和压力运行。应用报告新增实例请求与实际设备查询的 API 字段；MoltenVK 在该应用的 Vulkan 1.2 实例中返回 1.2.357，与 `vulkaninfo` 使用 1.4 实例时的查询分开记录。shader 工具可用不等于已补完 DXC 专用构建协议。

本轮汇总见 [gpu-validation.json](reports/refactor/2026-10-17-moltenvk/gpu-validation.json)。最终矩阵的命令、结果、源码指纹与日志见 [final-checks.json](reports/refactor/2026-10-17-moltenvk/final-checks.json)，该矩阵期间模块源码/配置/素材指纹保持一致；同目录 `run_final_checks.py` 可复跑默认测试及构建/lint 矩阵。初次失败日志作为历史保留，最终结论使用带 `final` 的修复后日志。

M0/M1/M2 仍为 IN_PROGRESS：窗口屏幕/键盘检查、Windows 对照、无 maintenance1 严格关闭、完整底层故障注入及性能对照不能由本机通过的部分替代。V3–V9 源码继续按交付计划推进。

## 2026-10-16 接线记录（历史范围，已由上节接续）

- 新增 `DeviceConfig` / `DevicePreference` 创建设备入口，接入 headless 和 Window Lab；队列、验证层要求、设备选择、必需能力、分配/录制策略、在飞帧数进入实际配置。支持与启用分开记录，描述符实现当前仍为 PerSet。
- 合并 `VulkanDevice` / `GraphicsDevice` 为唯一 `Device`，删除 `from_device` / `finish_creation` 恒等转换和无调用者的 `create_raw_device`。设备托管存储用 Weak<Device>；render-pass 缓存仅保存资源键，内建 sampler 缓存为 Weak，避免资源反向持有形成 Arc 环。
- Buffer、Texture/View、Sampler、RenderPass、Framebuffer、Pipeline/Layout、DescriptorSet/Layout/Pool 已携带设备代际和 SyncCell。命令按 pass 声明与屏障实际操作数保活，成功提交后登记所用资源；跨设备/无效代际在提交前拒绝。setup 上传、独立命令和呈现命令均接入登记。
- 命令分配与描述符池的底层存储不持有 Arc<Device>，可随 Retirement 延迟销毁；PendingCommand drop 不做 fence / device idle 等待。描述符池仍在飞时拒绝 reset，分配器保留忙池并从其他池分配；整池优化和预算归 V4。
- 图执行器移除 SequenceRuntime、资源影子引用计数与连续序号桶；回执直接使用 SyncPoint，聚合为 SyncSet。删除 SubmissionEngine、DeviceState/QueueState、InOrderRetire、CommandBufferSeqTracker、BucketKeyWitness 和设备 setup_slot。录制序号暂作绑定诊断元数据，不参与 GPU 完成判定。
- 提交前预留命令/回执/在飞帧容量，提交成功后资源登记不分配且不调用用户代码。部分提交后等待失败会保留 execution 和退役回调，未提交命令仍取消，主错误不被清理错误覆盖；这尚不是 V2 完整 FrameTransaction / PresentTicket。
- 下游迁移覆盖 country-core 实际执行宿主、tiny-glade、tg-text；country-core 完成回收通过设备真实 poll，不能手填完成值。旧提交模拟器的 16 个测试随实现删除；生产失败、跨队列隔离测试位于 rg_fixes_cpu，设备时间线测试继续保留。
- 共用提交入口按实际启用能力走 `vkQueueSubmit2`，否则保留旧 submit 路径；公开 Wait/Signal/SubmitBatch 提供二值与设备时间线等待，成功才发布完成点。裸 Vulkan 命令/信号量入口有 unsafe 生命周期契约，尚不是完整 FrameTransaction。
- 资源导入检查设备代际、描述符/usage、缺失 token；temporal 导入错误向上传播，校验尽量前置到录制/提交之前。无效 generation 被拒绝，修复 token 到达上限时的溢出求值。
- 图会话使用设备配置的录制策略和在飞帧上限；默认仍为 Serial。帧数限制不等于已经实现整池复位。
- 新增 `tests/import_contract.rs`、`tests/device_config.rs`，迁移已有 bind_import 调用的错误处理。新测试文件存在不代表其 GPU 路径已经执行。

### 2026-10-16 检查记录（macOS，真实 workspace）

此前离线解析缺失依赖、crates.io DNS 和临时 manifest 限制已解除；本轮下载缺失的公开依赖后直接检查真实 workspace。持久日志与命令清单：[checks.json](reports/refactor/2026-10-16-v1/checks.json)。使用 rustc / cargo 1.98.1，host `aarch64-apple-darwin`。

| 检查 | 记录与范围 |
| --- | --- |
| rhapsody 默认 / 无默认特性，全目标 check | 均通过；Windows cfg 不在当前目标内 |
| rhapsody + tg-text + tiny-glade，全特性 / 全目标 check | 通过，包含实际下游测试和示例的编译 |
| 四包全特性库检查（另含 country-core） | 通过；已有 country-core / roof 警告保留 |
| 四包全目标检查 | FAIL：未修改的 country-core `tests/wall_highlight.rs` 构造 `CursorTerrainRaycast` 缺 `delta` / `speed`；不计为全工作区通过 |
| rhapsody 全特性 lib/tests 进程 | 367 passed、0 failed、1 ignored；其中无 GPU 的提前返回不计 GPU PASS。之后针对性测试覆盖新增退役用例、描述符校验和最后的存储改动，详见日志 |
| 生产执行链 CPU 故障注入 | `rg_fixes_cpu` 15 项通过；覆盖提交失败不发布点、部分提交只等待成功点、跨队列完成隔离、等待失败仍取消未提交命令并保留主错误/在飞 execution |
| 托管槽与描述符池 CPU 验证 | 设备存储释放无需销毁桶、旧键不能复活，且不再登记旧 tracker；忙描述符池不 reset、不提前复用 |
| 严格 Clippy | `--all-features --lib --tests --no-deps -- -D warnings` 通过，未放宽 lint；修正原有固定 chunk 迭代的 Rust 1.98 lint |
| Vulkan / Windows 窗口 / 性能 | BLOCKED：缺 Vulkan loader / Windows 环境；`device_config`、`import_contract` 和新增真实退役用例均未执行 GPU 路径；1000 帧稳态与 V0 360 帧窗口验收未补齐 |

以上为上一轮断点；已完成/替代部分由 2026-10-17 节说明。V3–V9 仍以 DELIVERY_PLAN 的完整范围和验收门为准。

## 既有 P1–P3 接线（历史轮次，仍需按新版本门槛复验）

### 已接线范围

- 设备创建为每个实际队列建立一条时间线。图执行、独立命令、setup 上传与交换链提交使用同一入口；提交成功后才递增已提交值，不以 CPU signal 填补失败值，不追加退役用空提交。不同会话共享设备完成点。
- 移除全局 `queue_lock`。提交、present、queue idle 按实际队列加锁；device idle 按 QueueId 稳定顺序锁住全部队列。acquire 只使用交换链锁；同一缓冲的宿主拷贝使用对象锁。
- `QueueConfig` 实际控制 `vkCreateDevice` 请求数量；默认仅一条通用队列，`Prefer` 回退为类别别名，`Require` 无法满足时拒绝。逻辑队列顺序与时间线槽位保持一致。通用族必须支持图形与计算，避免 Compute 别名落到无计算能力的族。
- `Retirement<Doomed>` 取代设备 epoch graveyard。旧资源尚未携带精确 last_use，因此保守等待 drop 时所有队列的已提交快照；这不是最终的逐资源依赖方案。
- 图执行器不再持有 `DeviceState`、复制 `SubmissionEngine` 状态或解析日志。移除 `attach_graphics_device`、`mirror_submission_state` 与无调用者的旧图提交入口；下游相应迁移。
- 移除 SequenceRuntime 无界字符串日志和 recorder 字符串统计；使用类型化命令序号与屏障计数。生命周期资源记录实际回收。
- 默认一个普通图批次使用一个命令缓冲，初始/内部/导出屏障共用录制。显式并行模式保留。
- 图编译/会话公共错误使用统一 Error，设备资源创建及录制调用保留原始设备错误类别。声明校验覆盖数量、资源索引与导出类型；mip 校验使用完整 u32，允许恰好覆盖剩余 mip，拒绝空范围。
- 录制回调失败或 panic：取消未提交命令、关闭序号缺口、释放本帧引用并执行退役回调；已有批次成功提交时先等待这些真实完成点，再回收。GPU 已产生的副作用不会撤销。普通失败后可以继续下一帧。
- 修复 Windows 全特性 Clippy 检查涉及的 Aftermath 原始指针入口契约，调用方以 unsafe 明确承担回调数据有效期；调整 DXGI 冗余返回。

## 仍开放的技术领域

- V1：Apple M3 的真实队列隔离与 1000 帧稳态已验证；其它目标平台、完整关闭计数与对照仍待验。CPU 记录元数据与历史 ABI 投影不参与 GPU 完成判断。
- V2：无扩展严格最终关闭证明、完整 acquire/record/submit/present/reclaim 故障矩阵与所有窗口宿主迁移仍要闭合；旧代际已实施数量背压，字节预算接线归 V4；不得据当前代码宣称 M2 PASS。
- V3：类型化图 ID、写版本、命名 temporal、GraphDefinition / CompiledPlan / FrameInstance 和独立检查器。
- V4：真实帧 × worker × family 池、Upload/ReadbackManager、区间同步和预算闭环。
- V5–V8：完整缓存、真实内存需求瞬态计划、bindless/dynamic rendering、异步管线/热重载、多队列 ownership 与偏序别名。
- V9：替代路径总清理、下游总验收、长跑和同机性能 A/B。窗口/GPU 实测按用户指示后置。

## 历史验证：P0 清理与离屏基线

从原开发计划移入，仅保留历史口径：删除 9 个 oracle 测试文件和 45 个原机器码/布局/panic 文案测试函数；当时测试为 298 passed + 2 ignored。P0 稳定性用例观察到 events 从 4324 增长至 27504，后续接线轮移除该无界日志。

原 `p0_perf_baseline` 为 6 pass、300 帧、去掉 10 帧预热；原表如下。这与 V9 的 300 帧预热、3000 帧采样、3 次重复标准不同，不能替代最终同机 A/B。

| 构建 | mean | p50 | p95 | 命令缓冲/帧 | 屏障命令/转换 | vkQueueSubmit/帧 |
| --- | --- | --- | --- | --- | --- | --- |
| release | 0.30 ms | 0.29 ms | 0.41 ms | 3.03 | 5 / 11 | 1 |
| debug | 0.45 ms | 0.44 ms | 0.56 ms | 3.11 | 5 / 11 | 1 |

## 历史验证：P1–P3 接线轮

以下保留原记录，环境为 Windows、rustc 1.97.1，命令从 `tiny-glade/` 执行。它们不覆盖本轮 macOS 工作树中的设备配置、submit2 和导入变更，也不是 M1 的完整窗口验收报告。

| 检查 | 结果 |
| --- | --- |
| 三个渲染包 `cargo test --lib --tests --all-features --no-fail-fast` | 330 passed，0 failed，1 ignored（手动性能用例） |
| 三个渲染包 `cargo test --lib --tests --no-fail-fast` | 327 passed，0 failed，1 ignored |
| 三个渲染包 `cargo clippy --no-deps --lib --tests [--all-features] -- -D warnings` | 默认与全特性均通过 |
| `cargo build -p tiny-glade -p tg-text --features tg-text/gpu --all-targets` | 通过；保留 country-core/roof 原有警告 |
| 1000 帧实际 Vulkan 稳定性 | 在飞稳态资源记录 7→7、命令 2→2；排空后资源与生命周期桶均为 0 |
| 失败与共享状态 | 回调 false/panic、部分批次已提交后失败、两个会话共享时间线、设备丢失传播、拒绝外部 signal 设备时间线均通过 |
| 配置队列 | 实际创建与规划/类别别名匹配；本设备没有辅助队列，跨族 GPU 读回明确跳过。CPU 队列族规划单测通过 |

三个渲染包指 `rhapsody`、`shader-compiler`、`shader-pipeline`。该轮最终检查限定 `--lib --tests`，因为当时新增示例的 `window_lab/win32.rs` / `app.rs` 尚未齐备；因此这些历史结果不覆盖 Window Lab。示例后续状态另见其验收记录。

单独执行 `cargo test -p rhapsody --all-features --test p0_perf_baseline -- --ignored --nocapture`：300 帧、丢弃 10 帧预热、6 pass，debug mean 0.628 ms / p50 0.327 ms / p95 1.942 ms；每帧 1 个命令缓冲、5 条屏障命令 / 11 个转换、1 次 vkQueueSubmit、1 个提交批次。该次测量未控制系统负载，也未在同机重跑旧版本，**不作为 CPU 耗时优于旧实现的证据**。

该历史轮次的接口迁移同步修改 `country-core/src/render/execution_host.rs` 与 `tiny-glade/tests/core_raster_draw.rs`。宿主改用真实辅助队列句柄，队列族池上界改为最大族号 + 1；另外对 `country-core/src/game_profile.rs` 做了局部编译修复：Debug 输出 packed 字段前先按值拷贝，保留布局与序列化行为。其他已有工作区改动未回退。

原始运行日志位于工作区 `tiny-glade/rhapsody-refactor-*.log`；这些是本机运行记录，不属于架构实现。

## 当前阻塞与下一步

- Windows 运行验收继续后置；macOS/MoltenVK 的 GPU 与自动窗口检查已恢复，不能替代 Windows ABI/运行结果。最终屏幕像素和键盘检查缺 Computer Use 权限。
- 继续先闭合 V1/V2 的源码审查、故障边界和消费者；源代码工作不以缺硬件为停工条件。
- 后续按 DELIVERY_PLAN 推进 V3–V9；每版完成状态只在本文件登记。
