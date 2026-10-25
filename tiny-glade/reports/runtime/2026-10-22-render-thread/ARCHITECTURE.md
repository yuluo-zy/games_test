# 渲染线程架构(主线:World 只属于游戏线程,GPU 状态只属于渲染线程)

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

## 线程与所有权
| 线程 | 独占 | 每帧工作 |
|---|---|---|
| 游戏线程 | 游戏 ECS World | 更新 N+1(目标:ECS 多线程执行器) |
| 交接点 | 两个 World 同时可见(渲染线程空闲时) | 提取 N:游戏 World → render World(增量/拷贝) |
| 渲染线程 | render World + ProductionRenderer(会话、交换链、资源池) | 声明 → 编译 → 录制(并行块)→ 提交 → 获取 → 呈现图 → 呈现 → 回收 |
| 任务池 | 无 | 录制分块、ECS 系统、后台任务 |

不保留串行路径:渲染只在渲染线程发生。游戏线程领先渲染线程至多 1 帧(交接点即背压)。

## 交接协议(`render_thread.rs`)
1. 游戏线程按提交读边界放行后更新 N+1，与已提交GPU工作及渲染收尾重叠。
2. `RenderThread::reclaim()` 取回 `RenderSide`(render World + 渲染器),应用帧 N 的反馈(截图结果、统计、错误、提交/呈现标记)到游戏 World。
3. `ExtractSet::run(game, &mut side.world)` 提取 N+1。
4. `RenderThread::submit(side)` 交给渲染线程。
`RenderThread` 通过线程内 factory 构造和销毁渲染器、图会话及其 `Rc`；跨线程只交接可发送的帧包和结果，旧 `Exclusive` 包装已删除。生产 App 激活随帧交给常驻 worker 并 ACK；清理/关闭保留明确的 owner 边界。

## 声明/帧初始化读写的游戏 World 资源清点(提取分类)
次数为调用点数(world_graph / world_postprocess / scene_* / world_geometry / world_vegetation / frame_setup / production_* / render_loop)。

**A. 每帧小数据 → 拷贝进 render World**:GameConfig(14) FrameRenderConfig(9) Time(7) EscMenuState(7) FrameConstants(5,含写) WindowSize(4) ImageFormationConfigs(4) LutGalleryState(4,含写) VermeerState(3) ShaderRefactoringState(3) GladeSettings(3) VermeerDebugControl(2,含写) PainterlyState(2) MainCamera(2) AppMode(2) ActiveSession(2) CameraMatrices GameClock GladeThemeId RoofGravityFrame WallBrickUploadStatus State<AppLoadingState> ActiveIccProfile Events<ShowDemoBorderOverlay> DebugGui StartupVisualTargets StartupUiVisualTrace GameplayApplication

**B. 大的持久库 → 所有权移到渲染侧,游戏侧发命令/增量**:AssetTextureLibrary(8,帧初始化写) AssetSsboLibrary(8,写) AssetVAOLibrary(4) AssetPlacementDataLibrary(4) GpuWorld(resource_scope) Cobblestone3Raster(6,写) LivePathRaster(4,写) PathRaster(2,写) GardenRaster(2) WaterRaster GradingLuts GpuLutTracker TextRenderer LoadingGpuResources(2) ApplicationShaderLibrary RegisteredApplicationPrograms AssetLibraryRuntime MetaCommander LogoAnimation FeedbackReportBg SavedLoadingBackgroundTexture(移除) LutGalleryPreviewOwner(移除) CompletedGameplayWarmup VermeerPassLocals

**C. 材质回调 `func(builder, params, &mut World)`**:列表 pass 的每 draw 回调拿 `&mut World`,迁到渲染侧后改为读 render World(回调读到的资源必须属于 A/B)。


## 历史设计与验证（2026-10-22至24，以下状态不代表当前版本）

## 迁移步骤
1. ✅ 运行时:`RenderThread`/`RenderSide`/`ExtractSet`/反馈通道(本提交)。
2. ✅ 渲染器常驻线程：`CoreRenderBridge` 与 `Rc` 会话在线程内构造/销毁，不通过 `Exclusive` 移动。
3. A 类逐个注册 `ExtractSet::copy::<T>()`;B 类逐库改为渲染侧常驻(游戏侧写入改为命令队列,渲染侧在提取时应用)。
4. `GameLoop::frame`:update → reclaim → extract → submit;`render_inner` 整体在渲染线程以 render World 运行;截图/统计/错误经反馈回到游戏 World。
5. ECS 多线程执行器(解决 525 对顺序冲突)。
6. 渲染线程内:并行声明(只读 render World)、保留图、统一任务池;晚绑定相机降低输入延迟。

## 2026-10-22 第一阶段已落地(分相主线)

- 实测约束:第 N+1 帧编译依赖第 N 帧提交后确认的状态(`confirm_exports`、结果缓存 `commit`),声明也要用会话导入,因此本阶段渲染线程只接手编译之后的后半帧。
- 游戏线程:update(与上一帧后半段并行)→ `settle_render`(回收渲染器、截图记账、反馈与错误)→ `render_front`(绘制列表 + 声明 + 编译)→ `RenderThread::start`。
- 渲染线程(`tg-render`):`render_back`,即执行主图 → 取图 → 呈现图(不读游戏 World,沿用主图的帧全局量)→ present → 统计/节奏 → 收帧。
- 串行 `render` 只保留为 country-core 单测用的组合函数(`#[cfg(test)]`)。
- 截图 `submitted`/`finish` 改到下一次交接时在游戏线程执行;最后一帧在关机时结算。
- 渲染线程 panic 后线程退出,`RenderThread` 标记 poisoned,后续回收直接报错,不会阻塞。
- 下一阶段:拆分 rhapsody 会话(乐观提交),让声明/编译也能与上一帧执行并行;B 类库迁移仍按上文清单进行。

## 第二阶段(声明+编译上渲染线程)实施方案 — 2026-10-23 清点结论

前提:用户要求按主线前向实现(业务系统未接入,不以当前帧时间为准)。声明、编译、执行都在渲染线程顺序执行,不需要拆 rhapsody 会话。

### 前半帧读写清点(要点)
- 实体查询 7 组:draw_lists(Vao/Shader/Transform/各 pass 标记)、refresh_draw_vaos(AssetHandle<Vao>)、world_terrain(Transform+ObjectGrassPush;地形实体)、world_geometry(Roof*)、world_postprocess(Ui2d*)、world_walls(InstancedWall)、draw 回调逐实体组件约 25 种(ColorId、PrevTransformView、*MapView、Sheep/Duck、各 *View 等)。
- 前半帧对 World 的写(渲染侧拷贝会丢):GameConfig 分辨率(OOM)、GpuWorld import/export、GpuLutTracker、AssetTextureLibrary dset/presets、AssetSsboLibrary RG 缓存与 GroundedBuildings 上传、6 个 Raster 的 staging/reupload 标记、TerrainEditingData、ScheduledTerrainReadback、ShaderRefactoringState/PainterlyState/Vermeer*、MenuBackground/Thumbnail、截图 Requests、LoadingGpuResources 等。
- 大库在游戏更新中的写者:SSBO 库(set 14/16/19/22/28/36/40/44、PostUpdate sync_wall_bricks 等约 20 个系统,多为 staging 映射写/flush);纹理库(set 33/44、PostUpdate Reload);VAO 库(set 44、PostUpdate Rebuild/Reclaim);GpuWorld(set 40–44 nani);Raster(draw_path/water/garden、set 40 replay、set 33 clear)。Placement 库只在启动/卸载写。

### 迁移步骤(按顺序,每步可独立提交)
1. **渲染 World 与实体提取**:render World 建镜像实体(游戏 Entity → 渲染 Entity 映射),每帧按变更提取上表组件;A 类小资源用 `ExtractSet::copy_changed` 全部注册(补 Clone)。
2. **大库改为共享 + 写入时机收口**:6 个库与 6 个 Raster 包成 `Arc<RwLock<_>>` 资源,两个 World 持同一份;游戏更新期只允许"CPU 侧"写,所有改变 GPU 可见状态的 flush/staging 写收进新调度 `RenderSync`,在交接点(渲染线程空闲)于游戏线程运行;渲染线程只在声明期读锁。
3. **前半帧写回**:上表"前半帧写"改为写渲染侧状态,需要回到游戏侧的(OOM 分辨率、Vermeer/Painterly 进度、截图、TerrainReadback tick、StartupRevealFinished 事件)经 `FrameFeedback::defer` 回放到游戏 World。
4. **材质回调**:`func(builder, params, &mut World)` 改读 render World(组件来自步骤 1,资源来自 A/B)。
5. **整体切换**:`render_front` 移入 `render_back_frame` 之前,在渲染线程以 render World 运行;游戏线程每帧 = update → settle → extract → start。删除游戏线程上的前半帧调用。
6. **ECS 多线程**:为 525 对冲突按当前单线程拓扑序补显式顺序(Bevy 0.16 `ScheduleBuildPass` 可在 flatten 后加边),打开 `ecs-multithreaded`。
7. **渲染线程内并行**:声明分组并行(只读 render World)、保留图扩展、统一 `tg-compute` 任务池、交接时晚锁相机。

### 第一阶段验证记录(2026-10-23)
rhapsody 全部测试通过;tiny-glade:compile_before_submit、core_render_bridge(9)、original_loading_gpu、original_loading_present(3 帧真实 present + 关机)通过。lib 测试中 history_runtime/intro_history 失败为既有问题(测试缺 TerrainHeightsData,存档历史不在范围内)。

## 第二阶段详细计划与进度(2026-10-23 更新)

目标:游戏线程每帧只做"更新 → 交接(回收 + 写回 + 提取)→ 启动";声明、编译、执行、呈现全部在渲染线程。

### 第二阶段初始每帧流程（历史记录；后续验证见修订计划）
1. 游戏线程 `App::update`(与上一帧渲染并行)。
2. `GameFrameDriver::settle_render`:回收渲染侧,执行 `FrameFeedback.apply`(写回),计数与错误。
3. `ExtractSet::run(game, render)`:按 tick 拷贝资源、镜像实体组件、事件/状态/存在性、渲染侧所有资源、截图请求移交;写入 `ExtractedAt` 并推进渲染 World tick。
4. `ProductionRenderer::queue_frame`:在游戏线程用游戏 World 做 App 激活,交出本帧 egui(`EguiBackend::hand_over`)。
5. 渲染线程 `render_back_frame`:`render_front`(绘制列表+声明+编译,读渲染 World)→ `WriteBackSet::run`(生成写回)→ `render_back`(执行/取图/呈现/present/节奏)。
6. App 切换时 `reset_render_exchange`:新渲染 World + 新交接集(实体 id 只属于一个 World)。

### 初始步骤落地记录（后续修正以修订计划为准）
| 步骤 | 内容 | 状态 |
|---|---|---|
| 1 | 渲染 World 与实体提取:`RenderExtract` trait,各声明模块的 `render_extract` 清单,`ExtractSet` 显式 tick、实体镜像(同 id、换代替换)、事件、状态、存在性 `Present<R>`、渲染侧所有 | ✅ 已提交 d7792767d |
| 2 | 大库快照:SSBO(含稀疏/映射)、网格 Arc 共享、GpuWorld 仅 GPU 视图、纹理、VAO、摆放、Raster、MEI(Borrowed)、ICC(Arc)、LUT 等 | ✅ 已提交 d7792767d |
| 3 | 前半帧写回:`write_back`(GameConfig、Vermeer*、Painterly、LutGallery、Thumbnail、ShaderRefactoring、SavedLoadingBackground)、`write_back_with`(6 个 Raster 的上传/消费标志、TerrainEditingData.rasterized)、地形回读按消耗量相减、`StartupRevealFinished` 事件转发;截图请求移交渲染侧,提交/读回直接在渲染 World 完成 | ✅ 代码完成,待提交 |
| 4 | 材质回调读渲染 World:回调经 `DrawPassCtx.world` 读取,声明在渲染 World 上运行后自然满足;所需组件/资源已在 draw_callbacks_a/b 的清单中 | ✅ 随步骤 5 生效 |
| 5 | 整体切换:前半帧移入渲染线程,游戏线程删除前半帧调用 | ✅ 代码完成,待提交 |
| 6 | ECS 多线程执行器:各调度先单线程构建一次,读取冲突对,按执行顺序经 `ScheduleBuildPass` 加边后切 MultiThreaded(`ecs_order.rs`),`ecs-multithreaded` 进入默认特性 | ✅ 已提交 301141bbd |
| 7 | 渲染线程内并行 | 🔶 部分完成,见下 |
| 8 | 测试与修复:全量单测、GPU 集成测试、冒烟、截图、真机测量;修复清单遗漏(运行时 "missing X")、写回遗漏 | 🔶 已分阶段执行；完整回归和真机性能验收未完成，见修订计划 |

### 已知风险与待验证点(测试阶段逐项确认)
- 清单由静态扫描生成:经辅助函数间接读取的资源可能遗漏,表现为渲染线程报 "missing X";补到对应模块的 `render_extract`。
- 快照语义:CPU 状态隔离不能保证 GPU 内容隔离。游戏 N+1 的 flush/mapped 写入与渲染 N 并行时，仍须按帧读边界、物理版本与 GPU 完成点管理；旧实现的并发方式不能作为正确性证明。
- 快照开销:本轮已移除稀疏缓冲 CPU 影子快照、将 Raster 快照缩为消费元数据、像素留在游戏侧；普通 ShadowedBuffer、其他库和元数据表仍需继续审计与测量，详见修订计划。
- 渲染侧写入且游戏也读的状态若漏登记写回,游戏侧会停在旧值(如画廊预览、缩略图之外的 UI 状态)。
- 截图:请求在交接时移交,游戏侧 `has_requests/active_capture_purpose` 只看到尚未移交的请求。
- 加载 App 与游戏 App 共用同一套交接集,切换时重建。
- `WorldHistoryObservation`(存档历史,不在范围)渲染侧写入不回传。

### 步骤 7 明细(2026-10-23)
- ✅ draw 回调改为只读渲染 World(`DrawListFn` 第三参 `&World`):声明期不能经回调改 World,是逐 draw 并行的前提。
- ✅ 统一任务池:`tg-compute` 同时服务游戏线程的 ECS 执行器与渲染线程的并行录制;默认线程数为 `clamp(核数-2, 1, 8)`，可配置覆盖。
- ✅ 晚相机已有 mailbox、一次 latch、精确交互冻结及提交历史；2026-10-24 真实窗口通过两次等待期输入预览、释放/取消与关闭。回放和完整交互矩阵仍待验收。
- ✅ 声明已拆为 Prepare、只读 Collect、稳定 Commit；阴影、主 G-buffer、OC/2 和 late pass 已接共享池及跨 pass Collect。30 帧真实链观察到 540 个 Collect 任务、420 个跨 pass 任务；默认策略仍等待性能验收。
- ⛔ 保留图扩展:现有计划缓存几乎全命中;再进一步需要跨帧保留声明结果(按 pass 输入哈希跳过声明),依赖上一条的 collect/commit 拆分。


### 2026-10-24 续修架构状态

当前每帧主线程只路由已注册 HostTables；App 激活资源与 WriteBackSet 随 QueuedFrame 交接，worker 成功激活后发布 ACK，取消帧保持激活重试。上文第二阶段初始流程中的同步激活属于历史状态。

最终合成/UI 使用物理窗口 extent_0；缩小的世界 extent_8 只服务世界/TGSR，内部 extent_10 保持对齐。原呈现 shader 不拉伸输入，因此 resize 录制竞态在唯一 WSI 取图边界处理。原生窗口截图与 120/400 世界帧窗口周期已经确认铺满窗口，未新增 Vulkan 验证层或逐 draw 校验。

Raster 已知瓦片写入范围直接进入 MappedStorageBuffer 的可复用脏列表，避免整段 before 副本与重复比较；拒收保脏协议不变。描述符 arena 随真实 frames-in-flight 轮转，仍由租约/SyncSet 控制物理复用。早期两次 1060 帧容量稳定但显存失败，随后已闭合最终 MSAA 输出及动态常量的帧槽复用。最终输出沿用 session named owner，whole-export 保留消费者布局；常量页在帧准备边界轮换，CPU/Weak 租约和真实 GPU SyncSet 仍约束复用。480×270 与 1280×720 的 1060 帧原断言均通过，后者实际 GPU 重叠峰值 4390；最新 300 帧窗口与原生截图通过。生产者日志剩余覆盖、条目 ACK、组合回放和四场景性能仍待完成。详情与有效证据范围见修订计划的续修节。


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
