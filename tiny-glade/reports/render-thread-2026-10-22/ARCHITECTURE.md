# 渲染线程架构(主线:World 只属于游戏线程,GPU 状态只属于渲染线程)

## 线程与所有权
| 线程 | 独占 | 每帧工作 |
|---|---|---|
| 游戏线程 | 游戏 ECS World | 更新 N+1(目标:ECS 多线程执行器) |
| 交接点 | 两个 World 同时可见(渲染线程空闲时) | 提取 N:游戏 World → render World(增量/拷贝) |
| 渲染线程 | render World + ProductionRenderer(会话、交换链、资源池) | 声明 → 编译 → 录制(并行块)→ 提交 → 获取 → 呈现图 → 呈现 → 回收 |
| 任务池 | 无 | 录制分块、ECS 系统、后台任务 |

不保留串行路径:渲染只在渲染线程发生。游戏线程领先渲染线程至多 1 帧(交接点即背压)。

## 交接协议(`render_thread.rs`)
1. 游戏线程 `update N+1`(与渲染线程的帧 N 并行)。
2. `RenderThread::reclaim()` 取回 `RenderSide`(render World + 渲染器),应用帧 N 的反馈(截图结果、统计、错误、提交/呈现标记)到游戏 World。
3. `ExtractSet::run(game, &mut side.world)` 提取 N+1。
4. `RenderThread::submit(side)` 交给渲染线程。
`RenderSide` 作为整体移动;其内部的 `Rc` 只在它内部被克隆,因此整体跨线程移动是安全的(见 `Exclusive`)。

## 声明/帧初始化读写的游戏 World 资源清点(提取分类)
次数为调用点数(world_graph / world_postprocess / scene_* / world_geometry / world_vegetation / frame_setup / production_* / render_loop)。

**A. 每帧小数据 → 拷贝进 render World**:GameConfig(14) FrameRenderConfig(9) Time(7) EscMenuState(7) FrameConstants(5,含写) WindowSize(4) ImageFormationConfigs(4) LutGalleryState(4,含写) VermeerState(3) ShaderRefactoringState(3) GladeSettings(3) VermeerDebugControl(2,含写) PainterlyState(2) MainCamera(2) AppMode(2) ActiveSession(2) CameraMatrices GameClock GladeThemeId RoofGravityFrame WallBrickUploadStatus State<AppLoadingState> ActiveIccProfile Events<ShowDemoBorderOverlay> DebugGui StartupVisualTargets StartupUiVisualTrace GameplayApplication

**B. 大的持久库 → 所有权移到渲染侧,游戏侧发命令/增量**:AssetTextureLibrary(8,帧初始化写) AssetSsboLibrary(8,写) AssetVAOLibrary(4) AssetPlacementDataLibrary(4) GpuWorld(resource_scope) Cobblestone3Raster(6,写) LivePathRaster(4,写) PathRaster(2,写) GardenRaster(2) WaterRaster GradingLuts GpuLutTracker TextRenderer LoadingGpuResources(2) ApplicationShaderLibrary RegisteredApplicationPrograms AssetLibraryRuntime MetaCommander LogoAnimation FeedbackReportBg SavedLoadingBackgroundTexture(移除) LutGalleryPreviewOwner(移除) CompletedGameplayWarmup VermeerPassLocals

**C. 材质回调 `func(builder, params, &mut World)`**:列表 pass 的每 draw 回调拿 `&mut World`,迁到渲染侧后改为读 render World(回调读到的资源必须属于 A/B)。

## 迁移步骤
1. ✅ 运行时:`RenderThread`/`RenderSide`/`ExtractSet`/反馈通道(本提交)。
2. 渲染器 `Send`:`CoreRenderBridge` 的 `Rc<RefCell<Session>>`、`Rc<Cell<..>>` 随 `RenderSide` 整体移动(`Exclusive`)。
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

### 当前每帧流程(已实现,未测试)
1. 游戏线程 `App::update`(与上一帧渲染并行)。
2. `GameFrameDriver::settle_render`:回收渲染侧,执行 `FrameFeedback.apply`(写回),计数与错误。
3. `ExtractSet::run(game, render)`:按 tick 拷贝资源、镜像实体组件、事件/状态/存在性、渲染侧所有资源、截图请求移交;写入 `ExtractedAt` 并推进渲染 World tick。
4. `ProductionRenderer::queue_frame`:在游戏线程用游戏 World 做 App 激活,交出本帧 egui(`EguiBackend::hand_over`)。
5. 渲染线程 `render_back_frame`:`render_front`(绘制列表+声明+编译,读渲染 World)→ `WriteBackSet::run`(生成写回)→ `render_back`(执行/取图/呈现/present/节奏)。
6. App 切换时 `reset_render_exchange`:新渲染 World + 新交接集(实体 id 只属于一个 World)。

### 步骤与状态
| 步骤 | 内容 | 状态 |
|---|---|---|
| 1 | 渲染 World 与实体提取:`RenderExtract` trait,各声明模块的 `render_extract` 清单,`ExtractSet` 显式 tick、实体镜像(同 id、换代替换)、事件、状态、存在性 `Present<R>`、渲染侧所有 | ✅ 已提交 d7792767d |
| 2 | 大库快照:SSBO(含稀疏/映射)、网格 Arc 共享、GpuWorld 仅 GPU 视图、纹理、VAO、摆放、Raster、MEI(Borrowed)、ICC(Arc)、LUT 等 | ✅ 已提交 d7792767d |
| 3 | 前半帧写回:`write_back`(GameConfig、Vermeer*、Painterly、LutGallery、Thumbnail、ShaderRefactoring、SavedLoadingBackground)、`write_back_with`(6 个 Raster 的上传/消费标志、TerrainEditingData.rasterized)、地形回读按消耗量相减、`StartupRevealFinished` 事件转发;截图请求移交渲染侧,提交/读回直接在渲染 World 完成 | ✅ 代码完成,待提交 |
| 4 | 材质回调读渲染 World:回调经 `DrawPassCtx.world` 读取,声明在渲染 World 上运行后自然满足;所需组件/资源已在 draw_callbacks_a/b 的清单中 | ✅ 随步骤 5 生效 |
| 5 | 整体切换:前半帧移入渲染线程,游戏线程删除前半帧调用 | ✅ 代码完成,待提交 |
| 6 | ECS 多线程执行器:各调度先单线程构建一次,读取冲突对,按执行顺序经 `ScheduleBuildPass` 加边后切 MultiThreaded(`ecs_order.rs`),`ecs-multithreaded` 进入默认特性 | ✅ 已提交 301141bbd |
| 7 | 渲染线程内并行 | 🔶 部分完成,见下 |
| 8 | 测试与修复:全量单测、GPU 集成测试、冒烟、截图、真机测量;修复清单遗漏(运行时 "missing X")、写回遗漏 | ⏳ 全部实现后统一进行 |

### 已知风险与待验证点(测试阶段逐项确认)
- 清单由静态扫描生成:经辅助函数间接读取的资源可能遗漏,表现为渲染线程报 "missing X";补到对应模块的 `render_extract`。
- 快照语义:游戏在更新 N+1 写的 GPU 映射内存与渲染线程录制 N 并行,和原先"GPU 执行 N 与更新 N+1 并行"同类;CPU 侧快照保证声明看到一致的帧 N 数据。
- 每帧快照开销:SSBO 库中 ShadowedBuffer/稀疏缓冲的 CPU 影子整块复制,需实测;必要时改为 Arc 写时复制或去掉渲染侧不读的影子。
- 渲染侧写入且游戏也读的状态若漏登记写回,游戏侧会停在旧值(如画廊预览、缩略图之外的 UI 状态)。
- 截图:请求在交接时移交,游戏侧 `has_requests/active_capture_purpose` 只看到尚未移交的请求。
- 加载 App 与游戏 App 共用同一套交接集,切换时重建。
- `WorldHistoryObservation`(存档历史,不在范围)渲染侧写入不回传。

### 步骤 7 明细(2026-10-23)
- ✅ draw 回调改为只读渲染 World(`DrawListFn` 第三参 `&World`):声明期不能经回调改 World,是逐 draw 并行的前提。
- ✅ 统一任务池:`tg-compute` 同时服务游戏线程的 ECS 执行器与渲染线程的并行录制;默认线程数改为核数-2(游戏线程、渲染线程各占一核)。
- ✅ 晚锁相机:相机等每帧数据在交接点(启动渲染前的最后时刻)提取,已是游戏线程上能取到的最新值。
- ⛔ 声明分组并行:未做。声明期 `RasterPass`/`DrawBuilder` 经 `Rc<RefCell<RenderGraph>>` 与会话 `Rc<RefCell<VulkanGraphSession>>` 写入,单线程类型。要并行需先把每个 pass 的声明改成"先在线程本地生成 draw 包(只读渲染 World + 只读库快照),再按 pass 顺序串行提交到图",即 `run_list_pass` 拆为 并行 collect + 串行 commit;`DrawBuilder` 需要能脱离 `pass.host` 构造。属 rhapsody/桥接层改动,单独立项。
- ⛔ 保留图扩展:现有计划缓存几乎全命中;再进一步需要跨帧保留声明结果(按 pass 输入哈希跳过声明),依赖上一条的 collect/commit 拆分。
