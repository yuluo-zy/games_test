# 完整渲染循环复核（2026-10-23）

## 当前状态（2026-10-25）

生产只保留并行 Collect/录制和常驻渲染线程，**整批尚未结项**。MEI/egui 已归入同一有界上传入口；V14 三轮真实晚相机各180世界帧通过。当前优先修复用户报告的 resize 黑边和延迟；正式性能采样暂停，不能以历史窗口通过代替本轮验收。

| 范围 | 已落实 | 当前验收边界 |
|---|---|---|
| 所有权与泛型 | CpuPool<T>、GpuUse、scratch、PhysicalIdentity、collections 共用；Rc会话在线程内构造销毁 | 领域回归通过；owner执行仍约10.87次分配/帧，整帧零分配未达成 |
| 并行与旧入口 | Serial/Inline/Adaptive、门禁、旧提交和bool/take_error旁路、同步ShaderHost::prewarm已删除 | 测试以tests/support显式推进唯一Pending协议；无串行回退 |
| 上传与确认 | 有界接纳、保脏重试、事务保护、五类逐条ACK、真实完成点、旧World隔离；MEI拒收批次保留 | native90两项实际GPU回归通过；native91的24项混合native/CPU/ABI回归通过 |
| 窗口与呈现 | 邮箱/代际ACK、异步激活、物理/内部尺寸分离；最终resolve现映射源/目标像素 | red96证明64×64→97×83第64列读黑；green98八组及green102十二组实际GPU全像素检查通过；真实窗口正式版时延复验待完成 |
| 增量与pass | 生产组件日志、Raster已知范围、阴影/主G-buffer/OC2/late并行Collect | 完整业务工具矩阵及正式性能尚未覆盖 |

### 当前证据与限制

- V12两次原生图像revision冲突已由red82确定MEI绕过读边界。MEI/egui现共用TextureHost::upload_regions，容量拒收保留唯一owned批次；V14三轮修正后的原生DeviceEvent晚相机输入均通过。V13早期假输入依赖偶发OS动作的通过不算确定性验收。
- V14窗口长跑在第1552世界帧失败于同图像initial access 0x11/0x22冲突。截图目标由TextureHost初始化为AnyShaderRead，声明已修正为0x22；后续真实窗口复验仍需完成。V14 stable压力700帧通过；effects在刺激前因原生全屏状态假设失败，夹具现等待真实窗口态；该失败不算效果性能结论。
- 原生NSView/CAMetalLayer几何在resize和全屏往返时立即一致（red94窗口检查通过），没有据此修改layer布局。实际GPU red96证明最终shader按目标像素直接Load旧尺寸源图会读黑。当前应用捆绑唯一resize-aware呈现包，沿用现有反射、bindless、Pending、Collect和提交链；测试夹具新增首张正确尺寸呈现的2秒上限，无生产验证层。
- 最新全工作区全目标检查为check92（早于本次resize改动）。green102通过12组彩色象限/同尺寸/缩放/翻转全像素GPU检查，capture99截图GPU用例通过。V16调试版真实窗口在全屏resize后2.632秒才发布正确尺寸，严格2秒门槛失败；该帧图编译1.961秒。V17正式优化版正在构建并将复用同一冻结二进制验收，尚未发布生产时延通过结论。
- V11带计时的200预热+1000帧是诊断；有效CPU profiler为10秒997Hz、8511样本。队列提交约占23%叶样本，回读约43样本，不能将回读批处理当主热点。外部sample只取得2个采样周期，不用于排序。
- 正式性能矩阵和三类30分钟窗口长跑未完成。保留M3/MoltenVK硬件边界，不宣称独立硬件队列、Windows或全部226条审计已关闭。检查与失败日志见[持久清单](../../../crates/rhapsody/reports/refactor/2026-10-24-generic-pools/checks.json)，冻结构建与窗口数据见[性能目录](performance-2026-10-24/)。

### 接下来按依赖推进

1. 完成resize彩色像素、截图状态、真实窗口首帧时延与持续resize验收；失败先修正确性，不恢复串行或旧呈现包。
2. 复验完整窗口控制、取消、晚相机及实际Collect/跨pass任务，统一检查当前源码。
3. 按实测热点优化：同硬件300帧预热、至少3000帧测量、三轮；阶段计时、CPU栈、GPU时间独立诊断，正式计时关闭插桩并绑定相同二进制身份。
4. 稳定、效果切换、resize分别30分钟且至少10000帧；核对内存上限、p95/p99的5%门槛与未关闭审计项。整批交付继续保持未完成。

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
