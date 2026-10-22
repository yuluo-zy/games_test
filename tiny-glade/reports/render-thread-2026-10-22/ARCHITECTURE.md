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
