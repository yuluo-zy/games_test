# 并行、所有权与增量传输重构（2026-10-23）

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

## 不变量与目标

- 保留声明/编译/执行在渲染线程；游戏最多领先一帧。CPU 消息完成与 GPU 完成分开。
- 游戏拥有可变 CPU 业务数据，渲染拥有 GPU 分配、图会话、导入缓存和历史。
- 静态资产共享不可变 Arc 版本；小数据复制；大型动态数据移动所有权或仅复制脏区间。
- 所有消息、上传、重试、缓存及空闲池按实际容量计入有界预算。
- 不通过 unsafe Send/Sync 让并行声明共享可变图。

## 帧与资产协议

QueuedFrame/RenderSide交接App代际、资产/组件增量、相机、egui与截图状态；FrameFeedback返回确认、类型化写回、错误与可复用容器。渲染器和图会话常驻worker。

资产条目身份带代际，发布版本单调推进。分别确认接收、CPU 应用、上传提交、历史写入提交、GPU 完成、呈现。重复操作不重复应用；上传成功后编译失败不得重放上传/创建/事件。旧 App 写回不得污染新 World；旧截图必须结算。

写回使用字段合并、源版本检查、计数增量和消费确认；存在性也需要版本。游戏可见变化必须触发 change detection，提取版本单独管理。

初始化/管线准备和App激活采用现有owner协议；Rc图在线程内构造/销毁，Exclusive已删除。OS限定窗口操作保留主线程。

## 提取与上传

- SSBO 影子留在游戏侧，只传声明需要的元数据和脏区间；Raster 保留 scratch，传脏瓦片。
- 网格、纹理、VAO、Placement 按资产发布新增/更新/删除；GpuWorld 分离逻辑实例与 GPU 表。
- CPU 只读大视图仅为实际消费者注册；写时复制粒度为资产/页，不整库复制。
- 内部可变写入推进显式版本。合并相邻/重叠区间，不填补大空洞。
- 高频组件统一走生产者变更/删除日志；旧 tick 扫描接口已删除，首次/换 World/日志过期才重建。
- 业务 GPU flush/mapped 写入迁为 CPU 编辑及上传请求。
- 上传区分完整替换与有序增量；增量不能随 epoch 取消，也不能跨帧读边界合并。完整替换仅可取消未被已接受帧依赖的版本。
- OwnedUploadPayload 移动字节所有权，按 capacity 记账；失败必须返还未接收数据。
- 小型动态数据用帧槽；大稀疏修改用范围依赖；整体重写用有界多版本；安全的帧槽直接映射继续保留。
- 旧物理版本与上传租约按真实完成点回收；超大上传分块。队列满时避免排空线程阻塞在自己的生产请求。

## 并行与缓存

DrawBuilder 从 RasterPass 可变借用解耦：Prepare 确定附件/反射/逻辑资源与公共值；Collect 使用只读上下文和任务独占常量/输出；Commit 按原 pass 和稳定 draw 顺序分配槽、重映射索引、登记访问。生产Collect与分块录制统一走任务池；owner按稳定顺序合并结果和提交。

保持 draw/discard、透明/模板/间接绘制与资源内容语义。失败回收未提交批次，检查结果顺序确定。

模板缓存只存稳定布局/绑定/模板，动态值逐帧填充；依赖不完整的回调不开缓存。不缓存图句柄、帧槽和上传切片；命中仍声明资源访问。默认 32MiB LRU，App/设备/shader/资产/附件版本参与失效。

ECS 自动边与业务边分离，调度重建重新计算；覆盖新系统与 Commands 可见性。525 对仅是历史观测。诊断不得改变执行器策略。

ECS/声明/录制共用 tg-compute，默认 clamp(核数-2, 1, 8)，可覆盖。首帧与轻帧同样走任务池；任务数有界，工作任务不阻塞等待同池任务。

## 事件循环与相机

Update → AwaitRender → Finalize；每帧只更新一次。等待返回事件循环，不重复 redraw；结果发布后唤醒，等待前复查避免丢通知。resize 带代际、重建视觉数据但保留增量；暂停/关闭/线程异常都有独立退出路径。

晚更新从模拟相机基线预览新输入，不消费游戏输入、不再次推进 rig 平滑。精确拖拽/笔刷/放置当帧冻结改变操作投影的晚输入，操作绑定输入序号与相机版本。视觉悬停可重算但不能修改业务。回放只用已记录输入。

最终相机统一用于剔除/LOD/阴影/速度/TAA。拆开模拟常量与相机常量，历史按对应图像写入成功提交推进，不以 present 成败决定；失败/重建/跳变按历史有效性处理。

## 实施与验收

| 阶段 | 内容 | 当前状态（2026-10-24） |
|---|---|---|
| 0 | 基线、计数、写回与调度边界修复 | 全工作区全 target 检查通过；原侧夹具已恢复/重采集到项目 tests/fixtures |
| 1 | 版本/确认/重试、owned 上传 | 五类资产逐条目 ACK、失败保留、幂等/乱序、删除与真实全队列完成点已接线并验证 |
| 2 | 渲染器常驻、逐库所有权、启动/切换 | 常驻线程、Rc 线程内生命周期与随帧异步 App 激活已实现，真实双 App 交接通过 |
| 3 | 增量实体提取、池复用、删除完整快照 | 生产组件日志、Raster 已知瓦片范围、SSBO shadow/脏区间保留重试已实现；旧扫描入口已删除 |
| 4 | DrawBuilder拆分与并行Collect | 已接生产；最新强制 30 帧实际 540 个 Collect/420 个跨 pass 任务；唯一生产并行路线待最终性能验收 |
| 5 | ECS 重建、池粒度、声明缓存 | 重建顺序、共享池、32 MiB 反射布局 LRU、跨 pass Collect 与帧槽复用已实现；两种分辨率 1060 帧稳态通过 |
| 6 | 非阻塞控制、晚相机与交互 | 邮箱/代际 ACK、OS proxy 唤醒及异步全屏尺寸恢复已实现；真实窗口/关闭/晚输入通过，完整交互矩阵待验 |
| 7 | 整链、长跑、性能 | GPU、窗口及两种分辨率 1060 帧正确性通过；输入闭环及统一复验通过；系统分配器试测已执行，生产 mimalloc 四场景矩阵正在重验 |


每阶段测试后继续。验证不同/重叠上传区间、替换/增量混合、预算和失败原子性；资源删除重建、旧代际反馈；调度新增/重建/Commands；不同并行分块与缓存一致性；事件唤醒/resize/暂停/关闭；相机交互/回放/历史；既有 rhapsody、桥接、Raster、VAO、加载、截图和真实 present 测试。

性能覆盖静态、大量 draw、局部/连续编辑、加载切换；预热后至少 1000 帧、三轮，测 CPU/GPU p50/p95/p99、Prepare/Collect/Commit、复制/上传字节、分配、扫描量、排队/依赖等待与内存高水位。静态帧不复制大库；局部传输随脏数据增长；稳定容量复用；长跑有界。真机轻场景p95回退超过5%时继续优化唯一并行路线；软件 Vulkan 只证明正确性。


## 历史实施证据（以下完成/待办均限当时构建）

## 已落实的修正（2026-10-23，工作区，尚未提交）

### 调度、写回与上传基础

- ECS 每次重建按当前显式依赖重新排序并补冲突边，覆盖新系统；显式依赖可以改变原顺序。修复初始化时移出 `Schedules` 导致的资源重建断言。Commands 的业务可见性仍要求显式同步边，不能以冲突对数量证明完整正确性。
- 写回校验来源 World、资源添加/修改 tick 和存在性，避免覆盖游戏侧后续编辑、删除后复活、旧 App 写回；正常触发 change detection。缺少目标资源时保留待移交队列。
- 上传队列接受移动所有权的 Vec，按 capacity 计账，拒收返还原分配。有序增量跨 epoch 不取消、不合并；生产 BufferHost 的脏区间重试复用退回数据。
- 提取记录资源复制数、组件扫描/复制/删除数；这些是工作量计数，未当作堆字节或帧时间。

### 稀疏 SSBO 与 Raster

- `SparseRenderBuffer` 只持 GPU 分配租约、每块实例数、不可变可见位图；不持 CPU 实例数组、空闲表、脏表或废弃分配队列。渲染快照不再能经 `get_mut<T>` 取得游戏侧存储对象。
- 墙体、屋顶、Sparkle、Nani 稀疏剔除改读 `SparseDrawView`；绘制范围算法与游戏侧共用。墙体高亮按排除集合过滤，不再临时 hide/show 共享位图，也不再误把本来隐藏的块恢复为可见。
- 以当前 Brick 容量计算，单次快照不再复制 `0x200000 × 96 = 192 MiB` 实例影子；保留的块计数与可见表约 72 KiB。屋顶瓦片影子另减少 16 MiB。此为数据结构尺寸计算，不是实测帧率收益；变化的库仍需复制元数据表。
- Raster 的 composite、scratch、tile hashes 改用独立 Arc 版本，静态快照共享分配；编辑时按缓冲写时复制。composite/scratch 提交改为版本共享。修复旧快照 selector 指向 scratch、但 scratch 为空的问题；小型瓦片暂存区保持有效，发布器与脏标志各侧独立。
- Raster 目前是整张缓冲的 COW（512×512 时 256 KiB），不是脏瓦片增量提取。仍需后续按瓦片移交和容量池优化。
- `AssetSsboLibrary::register(&self)` 推进独立注册版本，`resource_with_revision` 将其与 ECS tick 合并判断。所有 SSBO 声明清单已接入，覆盖共享引用注册后 ECS tick 不变的情况；静态库保持复用。

### 常驻线程、提交确认与事件循环（后续增量）

- `RenderThread::spawn` 移交 Send 工厂，在工作线程构造、使用、销毁可含 Rc 的 renderer。只有 RenderSide、控制闭包及结果过线程，已删除 `Exclusive<T>` 和 unsafe Send。窗口平台操作仍留主线程；启动失败、控制异常、关闭均回收线程。
- 帧请求、GUI 数据及 CPU 读取租约随 RenderSide 移交；GUI tessellation 和 DPI 在游戏侧按帧固定，避免渲染读取已被下一帧改变的 Context。
- BufferHost/TextureHost 加入 CPU 声明至提交的读取边界，发送前封闭，完成或异常时释放，防止队列中的帧尚未开始时游戏提前修改物理资源。这是保守互斥：游戏侧物理写入仍可能等待，并非 GPU 完成确认。裸 mapped 指针等绕过宿主的写者及帧槽/多版本迁移仍待完成。
- 网格共享引用注册和纹理描述符发布接入显式版本。静态重复查询不推进版本；提取仍结合 World 的资源添加/修改 tick。
- 写回分为 CPU 结果与实际提交后的消费确认；失败时清除相关渲染副本，使未变的源资源也能重新提取。地形回读请求按捕获数量确认并保留期间新请求，校验来源 World 和资源添加 tick。
- 时序历史先生成候选；异步路径的准备不提前修改游戏历史。图像写入实际提交后才确认候选和帧号，后续 present 失败不撤销已提交历史；旧 World、删除重建及游戏侧历史重置拒收旧结果。
- AwaitRender 每帧只执行一次 Update，未完成时返回事件循环，结果发布后唤醒并继续 Finalize。控制操作、上传和部分准备仍可能等待，尚未实现所有路径非阻塞。

### 下一段工作及边界

1. 审计其余内部可变资产与全部发布/删除路径，补齐操作序号、幂等确认及有界回收。现有三类库的显式版本不代表全资产协议已完成。
2. 将保守 CPU 读边界迁为按帧槽/范围/物理版本的 GPU 写入协议，覆盖 mapped 写者。CPU 快照隔离、GPU 租约保活、有序上传都不能单独保证 GPU 物理内容的帧版本隔离。
3. DrawBuilder 的 Prepare/Collect/Commit 和单 pass 并行已实现；继续覆盖跨 pass 依赖层，保持图所有线程上的稳定 Commit。默认开关需通过真机性能验收。
4. 补齐非阻塞控制流程、真正晚相机及交互一致性、缓存与真机性能长跑；现有提交确认仍需全游戏失败路径验证。

### 常驻线程增量验证

### Collect 实现与后续审查（本次继续，未完成验收）

- DrawBuilder 改为借用 DrawDeclarations、查询宿主和视口，不再保留整个 RasterPass 的可变借用。Collect 宿主不持图或 GPU 常量分配器；任务只返回拥有所有权的 DrawBatch。Commit 在图所有线程上按连续区间原顺序执行，分配常量并重映射绑定槽及间接缓冲。
- 逻辑 SSBO ID 位于真实图的 u16 资源域之外，Commit 逐项解析。保留 discard 的访问、未命中 shader binding 的 SSBO 访问和非缓存访问；只有原 pass 缓存本来会去重的访问被去除。纹理缓存访问模式或同名缓冲来源跨区间不一致时回到原串行声明。
- 常量 Vec 移入批次的租约，不在 Collect 克隆字节。默认不启用新路径；`TG_DRAW_COLLECT=serial|parallel` 用于等价／性能对照，`TG_DRAW_COLLECT_CHUNK` 默认为 64。最多 8 个共享池任务、4096 个 draw、32 MiB 常量字节；超出范围回退原路径。这个常量预算不是整个 FramePacket 的完整容量预算。
- 审查发现保守上传门禁与共享任务池组合可能形成循环等待。新增临时 ECS/Collect 准入门禁：Update 活跃时渲染 inline Collect；Collect 活跃时新 Update 保留 GUI 输入并返回事件循环，等结果后继续。不能因此宣称完整上传协议或所有路径非阻塞已经完成。
- 新增 Prepare/Collect/Commit 时间及任务计数，合并到现有帧诊断。依赖层之间的并行 Collect、上传帧版本、真正晚相机仍未完成；布局 LRU 已在随后增量中接入。
- 56 项 DrawBuilder 测试通过，包括新加的多线程独占批次合并、discard、常量数据、缓存槽及间接缓冲等价。随后对独立 DrawDeclarations 的变更继续复验。
- 核心库全量：2876 通过、1 ignored、0 失败（`/tmp/render-country-full.log`）。真实 GPU 新增逐像素等价测试发现第二帧的 native initial range 超出声明错误，正在定位，未绕过检查。历史夹具接入完整无窗口 GPU 资产所有者，不跳过生产墙体／屋顶 flush；仍在补齐共享启动资源。
- 额外修复：转发事件校验来源 World；组件提取查询切换 World 时重建；已销毁游戏实体的渲染镜像随删除回收，避免空实体长期积累。

以下是此前常驻线程的独立验证记录，不代替本次 Collect 的最终验收。

日志 `/tmp/render-resident-validation.log`：`render_snapshots` 5、`core_render_bridge` 10、`geometry_gpu` 1、`compile_before_submit` 1、`original_scene_compute_gpu` 10、`core_raster_draw` 11，共 38 项通过。新增常驻线程真实 GPU 上传/读回连续 32 帧；线程单元测试 11 项通过。

日志 `/tmp/render-resident-present.log`：Xvfb + lavapipe 下生产 GameFrameDriver 加载/呈现 3 帧、原生 WSI present 3 次及关闭通过。完整游戏模式先修复异步 Gameplay App 缺 `SteamdeckScreenshotMsgScreen`，再修复预热直接使用游戏 World 时错误要求 `Present<TextRenderer>` 标记，保留实际 producer 或提取标记两条合法路径。

随后 `/tmp/render-resident-gameplay-trace.log` 确认常驻渲染线程的羊材质辅助函数缺 `Sheep`（同路径也读取 `SheepWalkRecord`）。补齐两项清单，增加生产提取后调用实际材质读取函数的隔离/更新/删除回归；缺组件错误附带类型及实体。异常后关闭改为明确失败状态并 join，避免再次 reclaim 已损坏的线程反复 panic。最新复验日志 `/tmp/render-resident-gameplay-final.log`；其中线程、提取、读取边界和时序历史的 18 项针对性测试通过。

该次完整流程已执行 13 帧非空游戏图，但原生 present 验收因主线程读取旧交换链代而失败：`validate_swapchain_and_acquire_image(&mut Arc<Swapchain>)` 可替换 Arc，常驻包装不能保留启动时的副本。现改为在替换后发布当前 Arc；`/tmp/render-resident-gameplay-present.log` 复验通过：149 帧加载、13 帧非空游戏场景、13 次游戏阶段原生 SUCCESS/SUBOPTIMAL present，OS 关闭、保存线程与 GPU 退出完成。此为 Xvfb/lavapipe 下加载→游戏→关闭的短流程验收，不是全部交互、窗口往返或真机性能验收。

日志 `/tmp/render-core-history-check.log`：核心库 52 项通过，覆盖相机常量/延后提交历史、纹理发布版本、Raster COW 和笔画提交等。同步修复测试侧 Arc 写入、嵌套模块导入及索引类型的编译问题。最近完整 tiny-glade lib 为 164 通过、6 项 history/intro 失败；补齐 CPU 启动资源后，这 6 项仍缺生产墙体所需的 `WallConstructorOutput`/`BrickTransformSsbo` 等所有者，需要完整夹具，不以空壳资源或跳过生产系统冒充通过。

`cargo check --offline --locked -p tiny-glade -p country-core --all-targets` 通过，日志 `/tmp/render-final-alltargets-check.log`；现有警告仍保留。`git diff --check` 通过。

全部构建与临时依赖位于本地磁盘；不使用网络挂载目录。上述增量验证不替代完整游戏、长跑、真实 GPU 性能验收；下方保留此前快照阶段的独立验证记录。

## 验证记录（2026-10-23）

### 后续多角度审查与实际修复（继续工作中）

- 原生 token：高两位为外部宿主域，不能推动内部自动 token 进入该域。此前外部缓冲 token 会与资源池旧图像 token 冲突；修复后串行/Collect 交替四帧逐像素一致。
- 描述符生命周期：页缓存强持有 lease，并以资源引用数判存活；不同范围/视图会互相保活。改为有界弱引用索引（最多 4096 键/页、预算覆盖桶容量），实际参数组和 GPU 命令持有强引用。新增同一原生分配 8192 个范围的回收/复用和无自环回归通过。30 帧原版场景的描述符高水位约 2376，随后保持；旧路径第 30 帧约 11404 且继续增长。这是 lavapipe 正确性观测，不是帧率收益结论。
- UI 渲染：普通图标与颜色按钮 shader 选择反了。发行 SPIR-V 的 uniform 名及 FNV 键确认无 CustomizationIcon 应使用 ui_2d，有则使用 ui_2d_color_button；修复后实际回放叉号像素回归通过。
- 数据提取：稀疏实例块计数/可见表未变时复用同一不可变 Arc；仍比较公开元数据以覆盖绕过 writer 的既有写者。实例字节更新不会分配新 draw 表。旧快照与隐藏、回收、新版本隔离回归通过。
- 常量所有权：16 MiB / 1024 块有界 CPU 空闲池按 Vec 容量及索引容量计账；Commit、回退、取消和 discard 归还拥有的字节。复用时清零可选 uniform。GPU 常量页继续按原生完成点独立回收。
- 映射上传：MappedStorageBuffer 的 cpu_write 可持原生写入租约，蝴蝶/萤火虫生产写入现在经过 CPU 读取边界及 Rhapsody 的实际 GPU 范围依赖；租约保活分配，删除不必要的 unsafe Send/Sync。新增跨线程写入门禁及宿主销毁后的租约回归通过。直接访问公开 mapped 的 Raster 暂存/回读等路径仍待迁移，不能据此声称完整物理版本协议已实现。
- 取消析构：CPU 读取租约移到 RenderSide 的首字段，先于 World 释放。断连退回/未执行帧的 GPU 所有者析构不会等待自身仍持有的门禁；专项测试通过。
- 性能诊断：AwaitRender 返回事件循环后保留原帧起点和 Update 耗时，单独记录 yield_wait_ms，避免把恢复调用的几微秒误记为完整帧耗时；同时新增每帧 GPU/CPU 内存、描述符和常量页容量诊断。该项正随新构建复验。

本段增量日志：`/tmp/render-country-final-full.log` 核心库 2827 通过（含原资产 ignored 测试）；`/tmp/render-mapped-owner-regression.log` 主程序单元 180、Raster 12、宿主 18、快照 5 全通过；`/tmp/render-bindless-review.log` 后端单元 421、bindless 6/1 ignored、ABI 5、setup 4 通过，随后新增范围索引回归通过（`/tmp/render-bindless-index.log`）。`/tmp/render-gpu-full-fixed.log` 原版游戏链 6 项全部通过，含 33 个历史文件 / 2120 个条目与真实 30 帧场景；截图与回放叉号分别通过。独立启动测试先对齐生产显存预算，随后发现 MEI 要求安装目录作为 cwd，已修正并排队复验。以上日志对应各增量构建，尚未代替最后一次统一回归。

强制并行完整游戏日志 `/tmp/render-parallel-timing.log`：每个观察帧派发 16 个 Collect 任务，13 次非空世界原生呈现及关闭通过。该增量构建的 1000 帧窗口长跑已通过：`/tmp/render-parallel-1000.log` 记录 1001 次世界原生呈现及 OS 关闭/保存/GPU 退出；最新增量仍需复验。

全量后端初次执行的多队列分支因 lavapipe 缺少独立计算队列失败，必须保留为当前环境不能验收的硬件能力项。其余发现的宿主预算故障注入、完成状态轮询和命令回收测试已修正；尚需统一复验。全部文件和构建均位于本地磁盘。

本轮在 Linux / Mesa lavapipe 上执行，`RHAPSODY_REQUIRE_VULKAN=1` 禁止因设备不可用而跳过。`core_raster_draw` 开启 Vulkan validation；其旧夹具遗漏帧资源声明器，已补齐，未放宽图内容或生命周期检查。

| 本轮测试目标 | 结果 | 覆盖 |
|---|---|---|
| `render_snapshots` | 4 通过 | CPU 编辑/回收后的旧快照、GPU 租约保活、并发高亮过滤、Raster COW/scratch/发布隔离、共享引用注册及删除重建 |
| `geometry_gpu` | 1 通过 | 真实墙砖脏块上传/读回、静态复用、更新与回收；游戏库销毁后快照保活真实 GPU 分配 |
| `compile_before_submit` | 1 通过 | 编译未提交、取消、后续帧真实写入 |
| `core_render_bridge` | 9 通过 | 导入、惰性库资源、取消/异常、帧租约、旧帧回收、App 隔离 |
| `original_scene_compute_gpu` | 10 通过 | 六类 Raster、水/花园、地形、重投影、SPD、DOF、TAA/TGSR、预热历史、风格化 |
| `core_raster_draw` | 11 通过 | 真实像素、索引/深度/外部资源、双帧在途连续 30 帧、temporal resize/取消、设备错误 |

本轮共 36 项通过，`git diff --check` 通过。日志：`/tmp/render-snapshot-final-tests.log`、`/tmp/render-raster-final-tests.log`。不是全游戏验收，也没有真机 p50/p95/p99 性能结论。

此前基础修正的验证：rhapsody lib 420 通过/2 ignored，传输 GPU 8 通过，render_thread 8 通过，ECS 重建 3 通过；本轮未修改 rhapsody/ECS 实现、未重复这些全量检查。此前 tiny-glade lib 还有 6 项 history/intro 测试因缺 `TerrainHeightsData` 失败，本轮未修复、未计为通过。

复现本轮命令（工作目录 tiny-glade；缓存源是已有的 USTC 离线缓存，不发起代理请求）：

```bash
unset HTTP_PROXY HTTPS_PROXY ALL_PROXY http_proxy https_proxy all_proxy CARGO_HTTP_PROXY
export NO_PROXY='*' no_proxy='*'
export CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=1
export PKG_CONFIG_PATH=/tmp/render-build-deps/sysroot/usr/lib/x86_64-linux-gnu/pkgconfig:/tmp/render-build-deps/sysroot/usr/share/pkgconfig
export PKG_CONFIG_SYSROOT_DIR=/tmp/render-build-deps/sysroot
export LIBRARY_PATH=/tmp/render-build-deps/sysroot/usr/lib/x86_64-linux-gnu
export LD_LIBRARY_PATH=/tmp/render-build-deps/sysroot/usr/lib/x86_64-linux-gnu
export VK_LAYER_PATH=/tmp/render-build-deps/sysroot/usr/share/vulkan/explicit_layer.d
export RHAPSODY_VULKAN_LIBRARY=/usr/lib/x86_64-linux-gnu/libvulkan.so.1
export RHAPSODY_REQUIRE_VULKAN=1 VK_ICD_FILENAMES=/usr/share/vulkan/icd.d/lvp_icd.json
cargo --config 'source.crates-io.replace-with="cached-ustc"' \
  --config 'source.cached-ustc.registry="sparse+https://mirrors.ustc.edu.cn/crates.io-index/"' \
  test --offline --locked -p tiny-glade --no-fail-fast \
  --test render_snapshots --test geometry_gpu --test core_render_bridge \
  --test compile_before_submit --test original_scene_compute_gpu --test core_raster_draw
```

系统开发库和验证层仅解包至 `/tmp/render-build-deps/sysroot`；另一台机器需准备对应依赖。为完成链接清理过 `tiny-glade` 包的可再生成构建产物，未清理源码、资产和用户改动。

### 最新增量：资源身份、诊断与组件删除（2026-10-23）

- 所有 BufferHost / TextureHost 分配使用全局唯一代数，编码到 ID。只接收 ManagedHandle / ID 的图和材质入口也验证代数，拒绝另一个 App 的同号槽位及释放后复用。旧物理 owner 仍由已有 GPU 租约保活；分配身份不等同于内容多版本。新增真实 GPU 跨宿主 / 槽位复用测试。
- AssetVAOLibrary 按不可变 Arc<VaoEntry> 发布绑定元数据；克隆不再逐项复制绑定 Vec。全局发布版本防止两个克隆分支各自重建一次得到相同 revision。Raster 快照可复用相同 publication 的原生视图，命中仍校验宿主代数，不复活已释放缓冲。
- ProductionRenderThread 的诊断读取改为固定容量的最新快照，不再通过同步 call 取值而等待后续在途帧。快照携带对应交换链代的原生成功计数，窗口验收按来源 World / receipt 去重，避免异步缓存导致重计或跨代比较。三帧 WSI 夹具验证读取诊断后仍保留 in_flight。
- 组件镜像消费 Bevy 删除事件；静态帧不再改写 seen 表并再次遍历删除差分。删除记录过期时显式回退核对，保留变化 tick 扫描，尚非所有生产者的修改日志。回收最后一个镜像组件留下的空壳；切换目标渲染 World 强制重新提取，包含已存在同类型资源的目标。变更组件批量插入并复用 pending 容量。
- 启动 GPU 夹具只执行实际 PreStartup/Startup/可选 PostStartup，并显式调用生产 MEI flush。完整 Update 由真实 GameLoop / original_gameplay_chain 验证。Nani 断言逐个核对实际注册 mesh，其他新增加的 brush/UI preview 不再误触发固定总数断言。

增量验证：`/tmp/render-nonblocking-vao-final.log` 主程序 180、启动 2、VAO 1 及真实 WSI 三帧通过。`/tmp/render-asset-generations-final.log` 后端桥 10、启动 2、宿主 19、VAO 1 通过；其两个旧测试仍断言“复用槽位后 ID 相等”，已改为低 32 位槽位相等、完整 ID 不同，并在 `/tmp/render-unified-final.log` 复验主程序 180、Raster 12、桥 10、几何 1、截图 1、启动 2、宿主 19、回放 1、VAO 1 全通过。该次历史测试因命令漏设 TG_INSTALL_ROOT 失败，补上原资产目录后的专项 `/tmp/render-history-assets-final.log` 已通过；不修改测试放宽验收。核心库新构建含原资产完整 2828 通过（`/tmp/render-country-final-2828.log`）。删除日志增量主程序 182、快照 5 和 WSI 三帧通过（`/tmp/render-journal-final.log`）；随后的批量插入 / 目标资源重提取与完整 tiny-glade 回归正在执行。

后端全量已结束：`/tmp/render-rhapsody-final-full.log` 共 88 个目标，803 通过、7 失败、10 ignored；七个失败均为要求独立计算队列而设备初始化报 Unsupported，不能列为本机已通过。最新构建窗口往返使用独立本地 Xvfb 与 xfwm4（DISPLAY=:2）；先前增量的 DISPLAY=:1 长跑已通过，不能代替最新构建回归。


### 完整循环审计增量

用户授权的只读 agent 已完成从主循环到回收的核对并复核修复；详见 [RENDER_LOOP_AUDIT.md](RENDER_LOOP_AUDIT.md)。新增工作纳入原阶段：阶段 5 补录制准入和任务窃取隔离；阶段 6 拆加载 readiness/交接等待、后续迁移控制消息；阶段 1/2 缩窄物理读边界与异步冷管线准备；阶段 6/7 补运行时尺寸发布、实际重试相机历史及完整窗口往返。上述新代码仍须以最新日志验收。

当前确定未完成项继续保留：完整资产序号/幂等确认与物理内容版本、普通影子/生产者变化日志、Raster 脏瓦片、跨 pass Collect、晚输入相机/交互、非阻塞窗口控制、异步冷管线、自动分辨率 VRAM/adaptive 控制器、最终全量与硬件性能验收。历史全量 tiny-glade 只剩 30 帧描述符稳态峰值断言失败，正在分析；不通过扩大固定容差将它记成成功。


### 后续执行节点（2026-10-23，用户要求明确进度点）

整体未完成。以下替代模糊的“继续优化”，每节点必须更新实现、验证、缺陷、下一步四项。

| 节点 | 内容 | 当前状态与出口 |
|---|---|---|
| 1 | 当前缺陷闭环：telegraph旧UI路径、SSBO导出、控制邮箱 | telegraph 3项通过；SSBO稳定导出已修，专项待跑；主程序194单元、桥10、宿主19、默认WSI三帧通过。完整窗口异步代际验收尚未通过 |
| 2 | 冷管线Prepare/Pending/Ready/Execute | 有界deferred API、帧状态机、Update准入握手、取消及resolve依赖已接入；后端421+async11+parallel graph3通过，主程序194及默认WSI通过。随后新增的后台预热取消/关闭收尾尚未重编译验证；队列饱和及真实窗口回归待做 |
| 3 | 窗口异步控制完整闭环 | 有界最新状态、代际ACK、错误发布、忙帧不reclaim通知已实现；关闭优先及prepared/截图取消已接入，最新收尾待验证。窗口往返夹具已修正等待新代际的时机，完整往返待复验 |
| 4 | 上传/资产协议与物理内容版本 | 先前owned Vec/有序增量/部分ACK已实现；完整操作序号/幂等/范围与有界物理版本待完成；随后移除过宽门禁 |
| 5 | 增量提取与减少复制 | 稀疏视图/Arc共享/删除日志已实现；普通影子、生产者修改日志、Raster脏瓦片、容器池待完成 |
| 6 | 跨pass并行与稳定缓存 | 单pass实际共享池Collect 480任务/30帧通过；跨pass依赖层、PGO稳态及默认策略仍待完成 |
| 7 | 晚相机、交互投影、自适应比例 | 实际重试历史/手动尺寸已修；晚输入一致性及VRAM adaptive controller待完成 |
| 8 | 统一全量回归和多角度review | 修改收敛后统一；四场景1000帧三轮、资源/错误/窗口/取消/串并行等价；软件正确性和真机p95对照分开 |

当前优先完成节点1–3，再推进4–7，最后执行8。旧增量1000帧长跑已在1800秒时限结束：加载151帧、游戏809帧，未完成验收。该次不是最新冷管线代码，且Collect实际任务数为0，不能作为所有并行路径验收。此前另一增量1001帧通过也不替代最新版本回归。


### 当前推进记录：关闭、自适应与跨 pass（2026-10-23）

本节更新上表的增量事实，整体仍未完成：

- 节点 1–3：`/tmp/render-window-admission-final.log` 完整窗口循环通过（13 次世界原生呈现、代际 7、4 次实际 OS resize）；`/tmp/render-loading-cancel-final.log` 与 `/tmp/render-suspended-cancel-final.log` 均通过。加载/挂起状态关闭无需恢复渲染，保存和 GPU 所有者已收尾。Update 准入现在在主图提交后、WSI 尾部前释放共享读边界。
- 冷队列：`/tmp/render-deferred-pressure-tests.log` 5 项管线单元通过，含确定性饱和/重复票据/回收；该命令过滤了 async integration（0 项），不能误记为其通过。新增 owned SPIR-V 容量检查仍需单独执行。
- 自适应：canonical `AdaptiveRenderState` 已接入 resident Prepare；预算恢复使用 250ms 等待、指数退避及恢复观察期；OOM 锁定有效倍率。GameConfig 保持用户请求，字段反馈保护 World/资源身份、较新请求、resize 和 flag_20。无 VK_EXT_memory_budget 时显式采取平台适配：不把未知预算当作 0，也不拿 heap capacity 冒充预算。`/tmp/render-adaptive-tests.log` 主程序 199 项通过。
- 同次核心库 2891 通过、1 失败、1 ignored；失败定位为 SSBO 导出排序同 resource 编号未按 generation 次级排序，已修待复验。测试数量包含当前工作区其他模块新增测试，不代表本重构新增数量。
- 跨 pass：三个阴影级联已接入有界共享池收集和稳定顺序 Commit，增加独立 cross_pass_tasks 计数。第一轮真实游戏测试发现提前构造 RasterPass 后回退违反必须 render 的协议；已改为先完成工厂/反射/收集，再创建图 pass。`/tmp/render-cross-pass-tests.log` 的游戏链 2 失败不能计为通过；Raster 12、快照 5 通过。后续修复正在 `/tmp/render-projection-cross-pass.log` 强制并行复验。
- 通用 shutdown 已改成停止并归还 queued/Pending 帧，避免只在 production cancel 路径安全；新增专门回归待跑。PreUpdate 不再每帧无谓标记 Adaptive changed。
- Raster 所有权进一步收窄：生产提取投影只保留 dirty bitmap/消费标志，像素与 hash 仍由游戏拥有；GPU 数据来自原 staging SSBO。这样不因渲染快照使下一笔编辑触发整块 COW，普通完整 Clone 继续保持原语义。新增实际提取测试验证 0 CPU 像素保留及游戏编辑地址稳定，当前待验证。

仍待完成：完整上传序号/幂等与物理内容多版本、生产者变更日志、晚输入相机/交互、跨 pass 默认性能策略、最终统一全量及长跑。上述投影属于减少复制的实现变更，尚不等于完整上传协议或真机性能验收。


上述修复的验证结果：

- `/tmp/render-projection-cross-pass.log`：主程序 200、Raster GPU 12、真实游戏链 6、快照 6 全通过；30 帧实际共享池任务 1200，其中跨 pass 90。GPU 完成后描述符 actual/expected 都是 `[10, 0, 8, 72]`。强制 `TG_DRAW_COLLECT=parallel`、chunk=1、`TG_REQUIRE_CROSS_PASS=1`，没有用 inline 回退冒充跨 pass 验收。
- `/tmp/render-core-projection-full.log`：核心库本次 feature 集合 2833、资产库 9 全通过，包含 ignored 资产用例；SSBO 稳定导出修复通过。
- `/tmp/render-deferred-async-12.log`：异步管线 12 项全部通过，包含新增 owned SPIR-V 容量预算/稳定完成票据。
- 只读 agent 复核了六种实际 Raster 消费者，无 CPU 像素读取遗漏；跨 pass 工厂/反射失败、取消、异常析构与图 owner 边界未发现新的明确缺陷。
- 在最后一次小调整中，shutdown 对已有 idle packet 直接取回，避免停止线程后再 poll 断开的完成通道；该调整随 `/tmp/render-adaptive-window-final.log` 复验。该日志还验证最新完整窗口循环、截图与场景计算。

这组通过结果不替代剩余物理版本设计、晚相机、所有场景长跑或真实硬件性能门槛。


### 最新窗口闭环与物理版本接入约束

- 最新窗口测试曾暴露新增自适应依赖可选 PhysicalDevice 包装的问题；实际 WSI Device 未挂接该包装。改为 Device 缓存真实驱动 MemoryBudgetSnapshot，已有 refresh_memory_budget 发布，读取不触发额外 Vulkan 查询、不使用 allocator 配额冒充预算。`/tmp/render-device-budget-window-fixed.log`：主程序 201 项，完整窗口循环通过（loading 163/world 13/native 13、代际 7、实际 resize 4、OS close/save/GPU shutdown）。
- 该 panic 同时暴露 Prepare 异常未打开 Update 门禁，主循环被准备等待状态遮住。两个 panic 返回分支现先释放 active 读边界，再打开门禁，发布异常并唤醒；新增专项通过。随后主循环的准备等待及 Update 前增加非阻塞完成/异常 poll，防止先跑一次 Update 才传播错误；最新单元复验日志 `/tmp/render-panic-gate-final.log`。
- 原始预算缓存后端测试 `/tmp/render-budget-cache-confirmed.log` 5 项通过；`/tmp/render-budget-cache-final.log` 的 async 12 和预算快照 2 通过，其 device_budget 夹具最初错误假定初始化不查询预算，修正为比较最近实际查询后复验通过。

物理内容多版本的后续实现必须遵守本轮只读审计确认的约束：

1. 在 queue_frame 的现有读边界封口点捕获不可变 FrameResources，明确持有 logical ID→physical owner。CoreGraphFrame/FrameBufferGraph/DeviceHost/compute external slots/RasterPassHost/CollectHost/VAO binding 都必须解析同一个 frame view，不能遗漏路径后落回下一帧 live slot。
2. frame 内新建资源使用显式 local overlay，旧 ID 的 miss 必须报错，不能静默读取 producer 的新版本。图 token 使用实际 physical owner 的既有 import_frame_buffer/texture 路径，不能把同一 logical token 反复绑定不同 owner。
3. 首批仅限生产者显式登记、CPU 权威写入且无长期 mapping/BDA/GPU 写入的缓冲。SparseInstanceBuffer 已在 flush 传入完整 data slice 与 dirty ranges：有旧 frame pin 时从同一既存 CPU 全源初始化新 physical owner，无 pin 时仍只传 dirty；不额外增加整库 CPU 影子，也不按 STORAGE_BUFFER usage 猜测可迁移性。
4. 显式 frame pin 计数不能以 Arc::strong_count 代替。上传成功后原子发布 current；release 只撤销 current logical entry，旧帧、已接受上传、GPU receipt 的旧物理 owner 保持到真实完成。
5. 上传队列的 same_key/epoch 已同时比较 physical Arc identity 和 logical UploadKey，新旧 physical owner 天然隔离；不能用全局 cancel_before 清掉旧帧已接受工作。
6. MappedHostWriter/长期 mapped slice、CustomCursor、GpuToCpu 回读、嵌入 BDA 的 Trimesh/chunks，以及 roof gravity/Nani/草叶花园等 GPU 写资源，首批保留稳定 owner 与现有同步。Egui 还有直接 get+record_texture_regions 写入，纹理版本迁移不能遗漏。

以上约束已在下面的多人并行增量中开始接入生产；验证范围以最新日志为准，节点 4 的全部上传协议仍未完成。


### 用户授权多 agent 后的实现与验证（2026-10-23）

三名 agent 分别负责物理资源版本、生产者变更日志、晚相机；主 agent 负责跨 pass 扩展、主循环集成、统一构建及交叉审查。构建产物、临时文件与日志全部位于本地磁盘，不使用网络挂载。

- 物理资源：queue_frame 封口后冻结 logical ID→physical owner，Bridge/Raster/compute/VAO 使用同帧解析器；egui 在冻结后创建的 IB/VB 已通过 frame-local host 登记。帧完成、取消、shutdown 释放冻结视图。显式 CPU 权威的 sparse buffers 可在旧帧 pin 下 fork；映射/BDA/GPU 写路径继续稳定 owner 和读边界，不宣称全部 Update 已与 Prepare 重叠。退休 owner 按逻辑字节和 owner 数量限额，不能称为精确 Vulkan 分配字节预算。
- 资产提取：SSBO/Mesh/VAO 已接真实有界生产者日志和复用目的库的增量更新。日志过期或源 World 改变显式重建，稀疏可见性共享引用修改会通知日志。普通 SSBO CPU 影子与 Raster 像素仍只由游戏侧拥有。
- 组件提取：静态 Handle/DrawTopology/pass 标记已接 immutable 组件类型级 insert/remove hook 和增量镜像；新增 replace/remove/despawn、溢出、换 World 测试，当前统一构建中。Transform 等可变组件仍保留 tick 扫描，不把此子链当作所有组件已完成日志化。
- 跨 pass：已从三个阴影级联扩展到主 G-buffer、OC/2 与 late pass，保持原顺序 Commit 和附件/依赖语义；当前正在运行真实游戏链。性能默认策略尚未经过真机 p95 对照。
- 晚相机：已实现渲染侧独立 MainCamera 投影、非消费累计输入 mailbox、水位/边界校验和一次 latch；只在无精确编辑的持续旋转状态启用，点击/绑定/模式变化冻结。actual camera history 仍以真实提交为准。已添加随成功 GPU 提交发布的 late_camera_input 诊断，真实窗口注入场景待验证。

验证事实：

- `/tmp/render-incremental-core-full.log`：核心库 2838 单元 + 9 资产库集成全通过（含原资产 ignored 用例）。
- `/tmp/render-parallel-integration.log`：主程序 208、桥 10、宿主 19、快照 8 全通过。新增 GPU 用例实际读取旧/新 physical owner 内容，验证取消/代际复用/严格缺项/local overlay，以及预算不足时等待旧边界；晚相机幂等、交互冻结、重试历史用例通过。
- `/tmp/render-main-cross-pass-integrated.log`：新组件 hook 与扩展跨 pass 的下一轮真实链回归进行中，不能提前记为通过。

整体仍未完成：新实现统一全量、完整上传序号/确认协议、尚未迁移的可变组件生产者、窗口晚输入实际命中、最终多场景长跑及真机性能验收仍需继续。



### 并行/非阻塞收尾任务（2026-10-23 晚起，Windows 本机 + 本机 Vulkan）

历史说明：此前一轮把回读消费者改成 `try_cpu_read`（等 GPU 完成才读、回调内持提交锁），方向错误：游戏线程与渲染提交互相加锁，且 GPU 每帧改写时长期读不到数据。已整体回退，按下表重做。下表保留2026-10-24状态快照；当前进度以上方唯一任务入口为准。

| # | 历史任务 | 2026-10-24状态快照（非当前） |
|---|---|---|
| 1 | 回退错误回读迁移 | 已落实，生产消费者读取发布副本 |
| 2 | GPU 回读消息化 | 生产与无窗口夹具接同一发布器，真实 GPU 测试通过 |
| 3 | 生产者 owned 上传/背压 | 已实现有界接纳、拒收保脏及准备轮询；多段失败保护与压力测试通过；旧同步回退与写入 API 已删除 |
| 4 | 移除 ECS_COLLECT_GATE | 已删除，强制并行整链实际任务数通过 |
| 5 | 资产序号、确认、幂等 | 五类资产逐条目协议、失败重试、删除/重建、旧 World 隔离及真实 GPU 完成通过 |
| 6 | 增量提取、Raster、容器复用 | 37 类生产组件统一日志；旧 tick 提取接口已删除，Raster 直接登记已知瓦片范围 |
| 7 | 非阻塞 resize/App/截图控制 | 异步 App 激活、窗口控制、六次 GPU 截图/关闭取消及 182 帧完整输入回放通过 |
| 8 | 跨 pass 收尾 | 四类生产 Raster pass 已迁移；30 帧实际任务通过，480×270/1280×720 的 1060 帧稳态通过 |
| 9 | 晚相机 | mailbox/latch/冻结/历史已实现，真实窗口两次预览命中；完整交互/回放矩阵待验 |
| 10 | 统一验收与报告 | 既有 all-target/GPU/夹具差分及两种分辨率长跑通过；最新输入/统一回归已通过；本批兼容 helper 已清理；完整工具窗口矩阵未覆盖，生产分配器四场景性能正在收尾 |

以下任务 2–6 的正文保留 2026-10-23 当时实现记录；其中预算等待/池线程超额、取走 ACK、无窗口不发布及无回读源不能确认完成的限制，已由 2026-10-24 实现替代。

#### 任务 2：回读消息化（2026-10-23 历史记录）

- 游戏侧 `country_core::render::gpu_readback::GpuReadbacks`：每类型保留最近一份完整副本（16 字节对齐 owned 存储）、产生它的渲染已提交帧号、发布次数；旧帧结果拒收，被替换的容器返还渲染侧复用。`submitted_frame` 是游戏已确认的最新提交帧。
- 渲染侧 `tiny_glade::gpu_readback::ReadbackPublisher`（`complete_render_feedback` 中，位于提交写回之后）：每个成功提交帧对游戏真正消费的 7 类回读（岩石遮罩、鼠标地形笔刷、鼠标深度、DoF 对焦、bush、两类地形高度）发出 `TransferManager::read_buffer`；后续帧收尾非阻塞 `try_read`，完成的数据连同来源 WorldId、帧号经 `FrameFeedback` 移交。地形高度只在 `copy_issued` 的帧（或首次、或上次因暂存槽满欠下时）复制。暂存 48 槽 × 512 KiB 上限、惰性分配；槽满跳过并计数，未完成复制超过 40 个时丢最旧。`BufferHost::convert` 对主机可见缓冲追加 `TRANSFER_SRC`。
- 消费者改读发布数据，不再触碰映射、不拿 GPU 锁、不等待：地形笔刷索引/悬停/绘制、鼠标深度、脚步声、Nani 预置花、动物移动/避障、蝴蝶、森林、bush 网格（同一 revision 不重复处理）、拍照对焦。时序敏感项按帧号判定：地形高度等到"发出复制那一帧"之后的数据；预置树转换等到生成请求之后提交的帧的岩石遮罩。首份数据到达前使用缓冲初值（全零 / 索引 -1），与原版首帧前读到的初始内容相同。地形绘制恢复原版"先建笔画再读回读"的顺序。
- 验证：`cargo check --workspace --all-targets --exclude toolbar` 与 `-p toolbar --lib` 通过；country-core 相关 369 项单元、clutter/critters/forest/tree 共 88 项通过；新增真实 Vulkan（Windows 本机设备）测试：复制→非阻塞完成→仅向来源 World 发布、发布后映射改变不影响副本，通过。
- 已知范围：只有生产 `ProductionRenderer` 路径发布；无窗口 `render_headless_gameplay_frame` 等夹具不发布，依赖回读结果的 GPU 集成测试需要接入同一发布器后再复验（任务 10）。`render_host::accepted_sparse_upload_survives_submit_oom...` 在本机失败于其测试辅助 `read_buffer` 的 usage/范围校验（GpuOnly 稀疏缓冲），与本改动无关，留待任务 10 定位。
#### 任务 3：生产者写入（2026-10-23 历史记录，预算分支已替代）

- `UploadBoundary::write_or_defer`：其他线程持有帧读租约时，写入作为拥有字节的作业入队并立即返回；租约释放（主图提交后的准入、取消或 shutdown）在重新开放边界前按 FIFO 执行全部作业。之后的直接写入因此仍排在先前延后的写入之后；租约持有者（渲染线程）自身的 render-local 写入照常立即执行、排在下一帧作业之前。延后字节上限 64 MiB，超出才回退为等待并计数（`BoundaryStats.budget_waits`）。作业失败记入粘滞错误，由下一次写入返回，不静默丢失。
- 改走 `write_or_defer` 的路径：`BufferHost::update_buffer_bytes`（同步校验、紧凑 owned 负载）、`BufferHost::release`、`MappedHostWriter::write`、`TextureHost::release`、`TextureHost::update_texture`（owned 像素副本）。`update_persistent_descriptor_set` 只做校验，去掉边界。显式同步的 `update_buffer_bytes_wait` 与 `MappedHostWriter::with_write` 保留等待语义，生产 `MappedStorageBuffer` 已不再调用后者。
- 主机可见目标（含 TRANSFER_DST）不再由 CPU 写持久映射（那要等在途 GPU 读完），改为与设备本地缓冲相同的有序 GPU 复制，由下一帧 `flush_pending_uploads` 在图执行前提交。没有 TRANSFER_DST 的主机可见缓冲仍按旧路径写映射。
- `MappedStorageBuffer`（有宿主写入器时）持有游戏侧 CPU 影子：编辑在影子上运行，按 256 B 块比较出变化区间，只上传变化部分（Raster 暂存区的脏瓦片编辑因此只传脏瓦片）。影子首次编辑时从映射初值播种（CPU→GPU 内存不被 GPU 写）。`cpu_contents()` 返回影子；本地/测试映射仍原地编辑。
- 验证：`render_upload_boundary` 新增"生产者不等待、延后写按序在边界重开前执行、失败由下次写报告"单元；按新语义改写 4 个旧"必须阻塞"测试：萤火虫映射写入在租约期间立即返回、释放并 flush 后 GPU 内容正确；COW 预算耗尽时写入延后而旧帧仍读旧字节；主机可见缓冲经 GPU 复制更新；Raster 暂存编辑不等待真实 GPU 读、其复制排在该读之后（读到 11，之后映射为 22）。真实 Vulkan 下 tiny-glade lib 211 通过；country-core lib 2841 通过；critters/clutter/forest/tree 全部通过；集成 core_render_bridge 10、render_snapshots 10、compile_before_submit 1、geometry_gpu 1 通过。
- 环境限制：本机无 Vulkan 验证层，`tests/render_host.rs`（19）与 `core_raster_draw`（11）在设备初始化即失败，未能验收；`gui_tod` 缺原版资产；`render_host::accepted_sparse_upload…` 的测试辅助对无 TRANSFER_SRC 的 GpuOnly 缓冲调用 `read_buffer`，为既有夹具问题。三者均留待任务 10。
#### 任务 4：移除 ECS_COLLECT_GATE（2026-10-23 历史记录，超额入队已替代）

- 门禁存在的原因是"Update 的池任务可能阻塞在渲染帧的读租约上，而渲染的 Collect/录制任务排在它们之后"的循环等待。任务 3 之后生产者不再等待租约，唯一剩余的等待点是延后预算（64 MiB）耗尽：现在只有非池线程会等；`tg-compute` 池线程超额入队并计入 `budget_overflows`，从而在任何情况下都不形成池内循环等待。
- 删除 `ECS_COLLECT_GATE` 及其全部使用：`render_bridge::with_recording_admission`（execute_built / execute_compiled 直接执行，录制保持配置的并行 runner）、`raster_host::try_parallel_collect` 的 try_lock、`GameFrameDriver::update_if_admitted` 的让出分支。Update 与共享池 Collect/录制可以同时进行。
- 同时去掉 Update 前的"准备期准入"让出（`render.update_allowed()` 为假时保留输入返回事件循环）：其注释写明唯一原因是生产者会在冷管线编译期间阻塞于该包的读租约，现已不成立。模拟在冷管线编译期间继续推进；`update_allowed` API 与渲染线程内部状态机保持不变（窗口循环仍用它决定是否唤醒）。
- 测试：新增"池线程超出预算也不等待"单元；改写 `collect_admission…` 为"无准入门禁、保留输入合并、只更新一次"；删除只验证门禁回退的 `recording_admission_falls_back…`。tiny-glade lib 211 通过（剩余 2 项为任务 3 已记录的环境/夹具问题）；core_render_bridge 10、render_snapshots 10、compile_before_submit 1、geometry_gpu 1 通过。强制并行完整游戏短流程需原资产与窗口环境，留待任务 10。
#### 任务 5：资产版本确认（2026-10-23 历史记录，ACK 生命周期已修正）

- 操作序号：沿用各资产库已有的单调发布版本（SSBO/网格/VAO/纹理的 `snapshot_revision` 等，经 `copy_incremental` 提取）。同一来源 World 内只增不减，渲染侧对已应用的 (World, 版本) 不重复应用；已有副本就地增量更新而不重建。
- 分阶段确认 `country_core::render::asset_ack::RenderAssetAcks`（游戏 World 资源）：`applied`（渲染副本已应用）→ `submitted`（引用该版本的帧成功提交）→ `gpu_complete`（该帧 GPU 完成）。`copy_incremental` 每次应用记入渲染 World 的 `AppliedAssetRevisions`；`ReadbackPublisher::finish_frame` 取走它，随反馈确认 applied/submitted；GPU 完成以同队列上帧后发出的回读复制完成为证（之后提交的同队列 fence 完成意味着该帧已完成），无需额外等待或 fence。
- 失败路径：编译/提交失败的帧只确认 applied，不推进 submitted/gpu_complete；增量库的渲染副本不回退，重试不重放已接受的上传/创建。确认按来源 WorldId 过滤，旧 App 不污染新 World。
- 测试：`asset_ack` 阶段推进/失败/乱序重复完成单元；`render_thread` 新增"同一版本只应用一次、再版本就地更新且确认"；真实 Vulkan `gpu_readback` 测试扩展为验证 applied/submitted、未提交帧只推进 applied、回读完成后 gpu_complete 推进。
- 范围说明：粒度是"库版本"，不是单个资产条目；条目级 RenderAssetId 代际由既有宿主分配代数（任务前已实现）承担。没有回读源的 App（加载界面）不产生 gpu_complete 确认。
#### 任务 6：限定增量优化（2026-10-23 历史记录）

- 可变组件（Transform、动画等经 `&mut C` 写入）：Bevy 钩子不观察可变借用写入，change tick 是目前唯一完整的变化记录，不能宣称已有生产者日志。`ExtractSet::mirror` 改为显式 tick 的 `Changed<C>` 过滤查询（`query_unchecked_manual_with_ticks`，与原 `is_newer_than(last_run, this_run)` 语义一致）：静态帧不为每行构造 `Ref`、不进入复制循环，只有变化行被克隆；首帧/换 World 仍走完整复制。`component_rows_scanned` 改为按匹配原型的实体数统计（不迭代）。删除日志（既有）、immutable 组件钩子日志（既有）保持。
- Raster 脏瓦片：任务 3 后 Raster 暂存区写入在游戏侧 CPU 影子上执行，按 256 B 块差分上传；`copy_to_staging` 只改写脏瓦片（每瓦片 64 B 连续），因此每帧 GPU 传输量随脏瓦片数增长而非整张 256 KiB。提取侧像素仍只由游戏拥有（既有投影）。
- 容器池/减少复制：单区间的上传负载（`update_buffer_bytes` 单 range、`MappedHostWriter::write`）现在直接移交给有界上传队列，不再在队列入口二次复制；回读容器（任务 2）由游戏返还复用；镜像 pending Vec 保留容量（既有）。
- 测试：`extraction_counts…` 扩展为验证可变写入经过滤查询被复制、静态帧 0 复制且扫描计数正确；tiny-glade lib 212 通过（剩余 2 项见任务 3 记录），GPU 集成 22 项通过。

## 2026-10-24 实施与验证

本轮沿现有 Rust API、资源 owner 与常驻渲染架构收尾，未新建候选渲染器。以下实现和验证取代前文过时状态；没有把 CPU 测试、设置开关或历史日志当作完整 GPU/窗口验收。

### 本轮接入的生产路径

1. **编译入口**：补齐 `window_lab` 的 `CompilerConfig.max_deferred_pending`、`system-decorator` 的 `half` 依赖。修复整理后的原版资产路径。全工作区 `cargo check --workspace --all-targets --locked --offline` 已通过。中途工作区清理删除 rounds，曾造成编译失败；按用户要求将九份固定夹具恢复/重采集到 workspace `tests/fixtures/native`，不再依赖逆向目录。五份从锁定指纹的历史测试程序常量区恢复，四份通过原 B RNE 指令重采集（含解析后的资源/查询存储边界），未调用候选 Rust 生成 expected。来源、执行边界与哈希见 [夹具说明](../../../tests/fixtures/native/README.md)。历史失败日志保留，生产 `--lib --bins --examples` 也单独通过。
2. **上传事务与失败原子性**：真实更新先登记完整范围，终态失败保存范围与错误；`poll_pending_uploads` 在图提交前拒绝不完整版本。覆盖整个原失败范围的重试才可解除保护，局部成功不能掩盖先前失败。多段中途 OOM 的真实 GPU 用例验证部分数据不被图消费、窄重试拒绝、完整重试后 GPU 字节正确。
3. **有界背压**：边界按 payload capacity、closure 和队列项计入 64 MiB/4096 作业；所有线程在容量不足时立即拒收，不再等待预算或池线程超额。作业保存 payload、区间及游标，下层暂满后保持 FIFO，不重放已接纳前缀；每次准备轮询有限推进传输。普通 producer 不因压力 flush 或同步回退。render-local 作业先于下一帧写入，读租约存续时只排空 local，避免提前覆盖当前帧。BufferHost 作业不反向持有边界，取消压力场景无 Arc 环。
4. **生产者保脏重试**：MappedStorageBuffer、ShadowedBuffer、SparseInstanceBuffer 在 Exhausted 时保留 CPU 权威数据及完整脏区间；AssetSsboLibrary 在下次提取前重试。相邻/重叠范围合并，空洞不上传。GameFrameDriver 让回 OS 循环，不重复 Update；`ECS_COLLECT_GATE` 保持删除。显式同步兼容 API 仍允许等待，不能泛化为所有 Rust 调用都无等待。
5. **库级 ACK 闭环**：AppliedAssetRevisions 保留当前库版本，直到替换/删除或切换来源 World；失败帧不再消费确认资格。纹理登记同一链。实际主图回执即刻记录，后续导出失败也不丢提交事实。gpu_complete 轮询主图真实 SyncSet 的全部队列，累计检查点有界合并，加载 App 无回读源也能完成；重复静态确认不增长 in_flight。旧 World 反馈拒绝。
6. **回读与窗口**：无窗口游戏夹具复用生产 ReadbackPublisher。去掉 ToolbarPlugin 外重复的 hover→ui_update 注册。worker 唤醒改用 EventLoopProxy，修复 macOS 主线程同步等 worker、worker 的 request_redraw 又同步等 AppKit 的死锁。全屏退出后等待窗口态再恢复尺寸，避免在原生动画期间修改 content size。
7. **长跑页复用**：1060 帧首次运行在第 262/295 帧新增描述符页，失败于既有容量稳态断言。页池此前把仍被 cursor 持有的页全部排除，即使有空槽也另建页；改为优先空闲页、再在既有 group 事务锁下尝试持有页，并串行化扩容和复查。GPU ABI 测试新增多个同时存活 cursor，保留参数不可变与实际 dispatch 结果断言。复验覆盖完整 1060 帧，仍在第 383 帧 GPU 重叠增加时从 3633 增到 7266，失败于原稳态断言。真实 drain 后 actual=expected=[8,0,6,54]，峰值 live=4290。复用改善不能等同于容量验收；没有放宽断言。

### 验证环境与边界

Apple M3 / macOS / MoltenVK 1.4.2，显式启用本机 Vulkan ICD 与验证层路径；真实 GPU 测试设置 `RHAPSODY_REQUIRE_VULKAN=1`，初始化失败即失败。沙箱内 Metal 不可用，GPU 与窗口验证使用获准的沙箱外执行。全程本地磁盘与 `--locked --offline`，资产来自 `B版本内容`。验证层仅由专项测试显式启用；生产 InstanceBuilder.layers 为空、DeviceConfig.validation=false，本轮没有加入生产 Vulkan 验证层或逐 draw 检查层。事务保护正常帧只读失败计数，修复/释放后归零，只有失败期间检查受影响版本；新增测试确认恢复快速路径。本轮 Debug 正确性结果不作为 Release 性能结论。

| 验证 | 实际结果与范围 |
|---|---|
| country-core lib | 2843 通过、1 ignored；含新增拒收保脏重试 |
| tiny-glade lib + GPU 桥接回归 | 222 + compile_before_submit 1 + Raster 12 + bridge 10 + geometry 1 + host 19 + snapshots 10 通过；含多段失败、相同版本 ACK 重试、无回读源完成、压力和取消 |
| bindless_shader_abi | 页复用修改后 5 通过；多个同时持有 cursor，真实 GPU 参数/结果校验 |
| 原版真实游戏链 | 6 通过；30 帧及 33 个历史/2120 个条目；强制 Collect 实际任务 540、跨 pass 420 |
| 基本真实窗口 | loading→gameplay 13 个世界帧、OS 关闭、保存线程/GPU shutdown 通过 |
| 窗口生命周期 | 加载中关闭、挂起后不恢复直接关闭通过；全屏修复后 12 个世界帧通过 resize、暂停恢复、零尺寸设置落盘、focus、全屏往返、代际更新和关闭 |
| 晚相机真实窗口 | 2 次预览、1264 次输入 motion；采样/消费序号 (38,44)，释放/取消后 capture idle；173 个世界帧后关闭通过 |
| 首次 1060 帧 | 失败：容量由 7266 增至 8477、9688；实际任务 19080/跨 pass 14840；真实 GPU drain 后 descriptor actual=expected=[8,0,6,72]，不能记长跑通过 |
| 修复后 1060 帧 | 仍失败：第 383 帧容量 3633→7266；实际任务 19080/14840，真实 drain 后 actual=expected=[8,0,6,54]；保留失败，不记长跑通过 |
| 全 target 检查 | 夹具迁入项目后通过，生产构建也通过；早先缺失输入的失败日志保留 |
| 夹具差分与集成 | country-core extra 1/animation 2、toolbar undo 2/UI 3/photo tree 1、decorator audio 12 全通过；修复音频测试缺静音事件初始化，生产逻辑未变 |
| 四类场景性能 | 未执行；须先完成统一输入/回归，再预热后每场景至少 1000 帧、三轮硬件测量 |

持久证据位于 [validation-2026-10-24](validation-2026-10-24/)，命令、退出状态、日志 SHA256、源码快照及变更范围见该目录的 `manifest.json`。各增量测试按对应源码范围记载，不能把早期日志整体绑定到最终源码。

### 接下来的任务（按用户指定顺序）

1. 窗口/控制组合：本轮功能验收已通过；最新输入修改后做统一回归。
2. 可变组件日志（用户任务 3）：生产接线与回归通过，旧扫描入口删除。
3. 逐条目 ACK（用户任务 4）：接线与分项 GPU/窗口验收通过。
4. pass 迁移复核：已核查，未发现遗漏生产世界 pass；具体源码和实测任务见文末。
5. 精确交互/完整输入流（用户任务 2）：当前版本 3 的完整窗口录制/保存及 182 帧精确回放通过；owner 3、相机/历史矩阵 8 通过，最新统一回归通过；剩余历史 helper 清理和实际窗口工具交互范围见文末。
6. 统一最新源码回归并更新本组三份报告。
7. 四场景、三轮、每轮预热后至少 1000 帧的硬件测量，再按瓶颈优化。Debug 正确性长跑不作为性能验收；轻场景 p95 回退超过 5% 不默认开启并行。

本轮夹具重建的首轮音频恢复地址有误，精确差分立即失败；已改为原常量地址 0x100122bb9，重建后 65536 组全部通过。旧失败日志保留。原测试缺新静音事件资源导致的 5 项初始化失败也已补齐并复验；不是生产算法改写。


### 2026-10-24 续修：窗口黑边、异步激活与 Raster 写入

- **窗口黑边已定位并修正**：最终合成/UI 目标误取 `FrameRenderConfig.extent_8`（缩小的世界尺寸），而原呈现 shader 使用 `SV_Position` 做整数 texel Load，不执行拉伸。最终合成改取物理窗口 `extent_0`；世界、TGSR 历史及对齐后的内部尺寸仍按现有比例生成。另处理 OS 在录制期间 resize 的竞态：仅在取得交换链图像这一必要边界比较最终图与目标尺寸，旧尺寸帧归还图像、等下一次提取，不增加逐 draw/pass 验证层。
- 真实窗口回归新增最终输出尺寸断言；75% 世界比例与 TGSR 下，514×354 窗口内部仍是 385×265 / 288×192，最终合成和交换链均为 514×354。120 帧窗口周期通过；另一次原生窗口截图运行完成 400 个世界帧、全屏往返与关闭。截图由 macOS `screencapture -l <实际窗口ID>` 读取，加载和过渡完成后的世界场景都铺满内容区。首轮新断言确实抓到全屏返回的旧帧 2940×1846 → 514×354，不掩盖为成功；修复后通过。一次与大规模编译重叠的截图运行在 180 秒达到 56 世界帧后超时，单独保留，不作为性能数据。
- **App 激活已移到渲染 worker**：主线程只切换已注册的 canonical HostTables，并把激活资源与 WriteBackSet 随帧交给 worker；实际 GPU owner 激活成功后发布原子 ACK。取消未激活的帧仍保留激活需求，下帧重发。初始化仍在启动边界完成。真实双 App 窗口周期通过，完整截图/切换/回放组合仍未全部验收。
- **Raster 已知写入范围接通**：生产绘制、history replay 及两处六域上传宏直接登记变化瓦片的 64 B 范围；初始化清空登记完整范围。MappedStorageBuffer 复用 CPU shadow 与脏范围容器，取消 Raster 每次编辑的整段 `before` 复制和 256 B 再比较；拒收仍保留权威字节与脏范围。通用未知范围编辑 API 保留原语义。country-core 2844 通过、1 ignored，含范围/空洞/拒收重试；首次未传安装根的运行有 10 项资产路径失败，传正确 `TG_INSTALL_ROOT` 后全部通过。
- **长跑问题已完成针对性修复**：描述符 arena、最终 MSAA 输出和动态常量都按真实在途帧槽复用。最初两次 descriptor arena 1060 帧已过页容量，但仍失败于显存；随后无编译干扰的复验捕获首次增长：最终图 2244608 B、5 个 1476 B 缓冲、一个 262144 B 常量页，减去已退休的 518400 B 截图存储，恰为此前 1995732 B 峰值差。最终图改用现有 session named owner 按在途槽复用；named 自动发布保留消费者已显式声明的 whole-export 布局，避免同一图像重复导出为不兼容布局。常量池在帧资源准备时轮换 arena，不重置存活 CPU/Weak 租约或未完成 GPU 页。未放宽任何长跑断言。

续修日志、窗口原图及各自源码边界记在现有验证目录的 follow-up 记录中；早期通过记录不提升为本轮完整验收。最新结果及剩余顺序如下；不能沿用修复前的显存失败作为当前结论。并行仍保持显式启用。


### 2026-10-24 资源帧槽收尾（最新验证）

| 当前源码验证 | 实际结果 |
|---|---|
| 480×270、1060 帧强制并行 | 通过原有全部断言；页容量 7266；GPU drain actual=expected=[6,0,3,54]；显存第 12 帧 1094674236 → 末帧 1094155836 B，native blocks 109 → 109 |
| 1280×720、1060 帧强制并行 | 通过原有全部断言；真实 GPU 重叠描述符峰值 4390；页容量 7266；显存 1264625980 → 1260946960 B，native blocks 110 → 110；GPU drain actual=expected=[6,0,3,54] |
| 两种分辨率实际并行任务 | 每次 19080 个共享池 Collect、14840 个跨 pass 任务；不是只设置环境开关 |
| 真实窗口最终复验 | 300 个世界帧、75% 世界比例 + TGSR、resize、暂停/恢复、零尺寸设置、focus、全屏往返、OS 关闭与保存/GPU shutdown 通过；实际原生窗口截图确认 UI/世界铺满内容区 |
| rhapsody 必要语义测试 | named whole-export 布局与相关时态回归 16 通过；常量 arena 与真实所有权/SyncSet 规则 14 通过；早先 descriptor arena ABI 5 通过，分阶段记录源码范围 |
| 最新真实游戏链 / 主程序单元 | 6 项真实链通过；223 项主程序测试通过，含真实 GPU 已知范围 Raster 的旧帧读/新写顺序 |
| 全工作区 all-target 检查 | 当前资源帧槽源码通过；--locked --offline |

截图：[最新实际窗口](validation-2026-10-24/tg-window-frame-slots-final.png)。所有早先显存失败、首次布局冲突、typed handle 转换编译错误、误用测试过滤器导致 0 项运行的记录均保留；0 项运行不计通过。最终图与常量的修复均是资源复用，未增加生产验证层；WSI 尺寸处理只在必要的取图边界，Raster 不再整段比较。

本节通过的是基础正确性与两种分辨率稳态，不是四场景三轮性能验收，也不是全部交互/回放矩阵完成声明。并行继续保持显式启用。


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
