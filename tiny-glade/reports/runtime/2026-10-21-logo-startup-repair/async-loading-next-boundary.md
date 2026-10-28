# 下一轮异步准备的最小安全边界

2026-10-21，只读审计。已完整阅读同目录 `review-20261021-nonblocking-prewarm.md`；不修改共享生产文件、不运行 Cargo/GPU、不启动 agent。当前 background decode + native texture owner 修复保持冻结。本报告针对启动/渲染性能，不涉及 toolbar、地形或建筑业务实现。

## 结论：可一轮实施的窄修改

**正常 splash 路径，仅把 gameplay 那份基础 shader 文件读取与程序注册移到现有 GameplayPreparation worker 的第一次 SubApp.update 之前。** 保留 loading 的同步最小 bootstrap、全部原算法、所有 prewarm、原 world warmup 和原完成门。

这不要求移动 CoreRenderBridge/Rc、创建第二 GPU owner、拆分已经声明的图，也不需要把 Timeout 当 Ready。改动闭包约为 `production_renderer.rs`、`game_loop.rs`、`run_game.rs` 和一个邻接的基础程序注册 helper/测试。建议中央完成当前真实窗口验收后再分配独占窗口实施。

收益必须准确描述：移走一份同步 shader **读取/注册**；**不保证消除首次 graph 解码、编译等待、GPU 提交/完成等待，也没有证据表明它能解决当前所有稳态慢帧**。

## 必须修正的耗时分类

- `ApplicationGpu::new`（当前 production_renderer.rs:649）调用 COMMON14、UI6、blit3、painterly2、Vermeer2，共27个 raster 程序。
- 每个经 core `startup::load_shader_into_library` → `compile_shader_variants`（render/shader.rs:163）同步调用2次 `load_compiled_raster` → `ShaderHost::read`（render_host.rs:1787）→ fs::read。每App正常54次，两App108次文件读，不计后续 Startup 其它程序或同名再次请求。
- **这里通常没有实际 pipeline decode/compile**：`RasterShaderProgramVariant::new` 只保存 Arc<CompiledShaderPipeline> 与空 cache。`RhCompiled::decode`、reflection/layout 和 request_raster 发生首次 `get_or_generate`（shader.rs:135）→ ShaderHost::create_raster_pipeline（render_host.rs:1881）时，多在图声明。
- 相比之下，ComputeShaderProgram::new 会进入 compute pipeline 创建；本27项基础 bootstrap 是 raster，不能把两者混记。
- 文件缓存是否命中、54次读取的字节量/耗时、decode/编译等待各占多少，当前没有分项实测。旧诊断16.516s worker首次update与18.557s warmup是混合阶段墙钟，不能算主线程I/O总量。

## Owner与消费者前提核查

| 安装/创建入口 | 是否在注册期消费基础 shader | 窄修改后的前提 |
|---|---|---|
| CoreRenderBridge::new，render_bridge.rs:60 | 否；创建HostTables、RasterShared和本线程Rc session | 原线程保留所有owner及Rc；worker只借原Arc库/ShaderHost |
| RasterShared::new_with_shader_library，raster_host.rs:477 | 否；保存同一Arc库，创建AttachmentCache/DynamicConstants | 不新建第二RasterShared或动态常量池 |
| EguiGpu::new，egui_gpu.rs:39 | 否；只保存device/tables与空texture map | shader在draw时才按名查询；workerready前不绘制gameplay |
| CoreRenderBridge::install_asset_libraries，render_bridge.rs:174 | 否；创建preset textures、安装AssetLibraryRuntime/GraphicsDevice | GPU资源创建仍在原main路径；本轮不声称其也已异步 |
| install_gameplay_gpu_startup，gameplay_startup.rs:380 | 未见安装时读取基础shader；有立即GpuWorld::new和watcher构造 | 立即GPU/OS对象不挪动；完整基础注册必须早于首次update |
| init_populate_ssbo_library、common textures/fonts/LUT、prefabs、atlases、decorators、Nani、MEI，gameplay_startup.rs:578–646 | 注册为PreStartup，不在installer当场执行 | initializer hook先完成全部基础程序，再调用一次原SubApp.update |
| menu_background_host::install:51 | 仅注册PreStartup shader loader | hook早于PreStartup，保原名称/顺序，不把它挪进基础27表 |
| session_preview_host::install:62 | 仅注册PreStartup initialize；该函数随后读textures/shaders | hook先完成，不改变其Inputs依赖 |
| world_terrain::install_terrain_startup:434；install_grass_startup:286 | Startup/OnEnter注册 | worker初次update仍执行原顺序 |
| gameplay_startup::install_world_visual_startups:291 | clouds/stars/billboard/loading/debug注册Startup；snow/water/ivy注册OnEnter | 同上；不删分支/实体 |
| world_lighting::install_deferred_startup:242 | Startup注册 | 同上 |
| GpuLutPlugin::build，resources/gpu_lut.rs:124 | 只insert GpuLutTracker | 无需提前读取shader |
| audio_host::install | 有同步Studio初始化/独立bank worker，但不读这27个shader | 本轮不挪音频owner或依赖 |

现有主路径已经证明 `CoreRenderOwners` 的 HostTables、RasterShared、AssetShaderLibrary 可由 worker使用：`warmup_owners` 捕获这些原Arc，worker `from_owners` 创建自己的Rc session，CompletedGameplayWarmup以真实完成的原生资源返回。窄修改沿用这些Arc；**绝不发送含Rc<RefCell>的CoreRenderBridge或ApplicationGpu**。

## 必须改的具体API和调用关系

### 1. 提取原27项基础程序注册，保原顺序与失败

新增邻接helper，例如：

```rust
fn register_application_programs(
    library: &Arc<AssetShaderLibrary>,
    backend: &Arc<ShaderHost>,
) -> anyhow::Result<RegisteredApplicationPrograms>;
```

`RegisteredApplicationPrograms` 保存真实 `final_post` handle 和对应 `Arc<AssetShaderLibrary>` 身份，可作为World资源返回。只有27项全部成功才构造，不能先插bool再补资源。继续调用core的 fallible `startup::load_shader_into_library`，复用原 COMMON_SHADERS/UI_SHADERS/BLIT_SHADERS 表；painterly/Vermeer的现有 `ShaderLoader` 适配器记录第一错误并在函数出口返回，正常成功时调用次序完全保持。不要用现有 `.expect` 把普通文件错误变成生产panic；当前生产panic hook有abort路径，不能承诺catch_unwind一定恢复。

原 `ApplicationGpu::new` 改成“建owner”+调用此helper，供eager路径复用；新增内部“只建owner、基础程序待worker”构造。EguiGpu、Bridge、Device与句柄命名空间不变。真实句柄由原库分配，不能直接假定final_post raw slot=4。

### 2. 保留当前公共构造的eager合同，显式只对普通splash启用deferred

保留 `ProductionRenderer::new` 的当前同步合同（headless/独立test caller不变）；增加 `new_for_async_gameplay_preparation` 或内部明确enum参数，避免让任意调用者得到无法渲染的隐式半初始化renderer。

GameLoop::new在创建renderer之前读取原 `GameConfig.dev.skip_splash`：普通splash使用deferred gameplay constructor；skip-splash继续原eager路径。loading `activate_application` 创建新ApplicationGpu仍用原eager构造。初始deferred gameplay owner只能安装资源和移交worker，任何试图提前render必须报明确未准备错误，不能以库为空切换到别的mode。

### 3. 给现有GameplayPreparation一个首update前的一次性hook

```rust
fn spawn_with_initializer(
    gameplay: SubApp,
    initialize: impl FnOnce(&mut World, &AtomicBool) -> anyhow::Result<()> + Send + 'static,
    warm_up: impl FnOnce(&mut World) -> anyhow::Result<()> + Send + 'static,
) -> anyhow::Result<Self>;
```

保留原 `spawn(gameplay,warm_up)` 为no-op initializer包装（既有tests与其它caller保持合同）。worker进入现有GameplayAppOwner保护与正常资源校验后，cancel检查→initialize一次→cancel检查→原SubApp.update循环→真实LoadingGameComplete→原完整warmup→真实GPU完成→发布。phase日志单独标“register gameplay base shader programs”，不能把它记成首次App.update时间。

GameLoop在同一原Bridge取 `hosts().shaders.clone()` 与 `shader_library().clone()`，捕获到hook；hook先验证World的SharedAssetShaderLibrary与捕获库Arc::ptr_eq，再注册并把真实结果插入同World。不能用Global静态缓存，把一个App的shader/program id搬到另一个App。

取消在注册中途到来时，可在两个文件读取/程序注册之间检查标志；不能强杀正在执行的fs::read。现有cancel_and_join/GameplayAppOwner负责保存线程和SubApp释放。失败不发布SubApp、不进入warmup、不推进Logo完成门；保持第一失败原因。未启动线程的捕获闭包丢弃路径也必须继续由GameplayAppOwner保护。

### 4. 正确回填renderer final_post slot

现 `final_post_shader` 只在ApplicationGpu/ProductionRenderer的存储、交换及公开getter中使用；真正world tail按库名查询final_post（world_postprocess.rs:1157）。不能因为当前无外部getter caller而遗留None。

在handoff后的 `activate_application`，完成原bridge swap/CompletedGameplayWarmup adoption时，从同World的RegisteredApplicationPrograms取真实handle并验证库Arc/名字映射，再回填当前ApplicationGpu/renderer slot。初始deferred renderer尚未移交worker时的 `activate_application` 只允许安装/停放owner；不能误把它当已准备。eager独立caller继续使用原已存在slot，无需伪造新marker。

## 验证负例：这轮实现必须具备

1. Recording ShaderBackend证实正常27项程序和两变体路径的顺序、名字、desc全部等于原表；注册期不得调用create_raster_pipeline。final_post返回handle与库真实名字查询一致。
2. 缺任一必需编译文件：返回原路径错误；不插Registered结果、不调用首次App.update、不warmup、不发布。不能用None代替缺文件。
3. initializer只执行一次，并严格先于PreStartup/Startup；原完整加载与ready门仍执行。不得把hook作为每帧重试执行。
4. 取消在hook前/中/后、线程spawn失败、接收端关闭：无未关闭保存owner、不发布部分App。当前生产abort panic边界单独声明，不能以CPU unwind测试概括真实abort。
5. 错ShaderHost/错库/错Device身份：在交接或初始化前拒绝；不能套新句柄到旧RasterShared。
6. eager/skip-splash与独立renderer测试仍有完整27项程序；普通loading自己那份程序仍存在、独立且未重注册。
7. 实际普通splash进程日志需分别给主线程/worker的54次读取字节/耗时、worker注册时间、首次update、warmup与handoff。真实GPU图/像素与saved background支路继续验，不把“少了main读取”算启动完整验收。

## 之后另做的loading boot CPU准备边界

`loading_graph::initialize_common_assets`目前palette→WalkDir boot texture读取/解析/上传→TextRenderer.new+所有字体读取/解析同步发生在loading首次Startup。已有 `TextRenderer` 是Send+Sync Resource，`load_font_from_file`实际read Vec后调用公开 `load_font_data(Vec<u8>)`，这提供真实owned CPU边界；纹理也可后台持文件Vec+原desc/数据范围，由main按原顺序上传，不必再解码。

但把整段搬线程不是一次简单替换：Startup其它消费者需要同World真实ColorLibrary/AssetTextureLibrary/TextRenderer/MEI，Logo尚未初始化时现有handoff poll要求Logo存在。安全下一阶段应先启动单个有界CPU准备任务，在**第一次loading App.update之前**非阻塞gate，继续OS close/resize；CPU结果就绪后按原顺序main commit真实GPU资源，再允许Startup一次。大纹理需channel/字节预算分批，禁止把所有文件无限堆到Vec；不能recv阻塞Startup或用空Font/Texture库通过。

若仅搬font读取而仍在Startup中recv，主线程依然阻塞，不算完成。CPU预读/解析还不能保证GPU上传不压Logo；最新raw/setup已是no-wait+SyncPoint，但使用Graphics queue。要另测上传CPU与GPU依赖，复用既有TransferManager能力，不能重写rhapsody或删mip/层/zero-fill。

## 管线等待：保持当前安全边界

主路径world/loading/presentation仍调用global-host prewarm；其等待与with_compile_backpressure是另一协议，当前没有安全的Pending-frame continuation。不得在本轮窄修改里删除wait、吞Timeout、重声明有状态图或复用已消费回调。`PipelineTicket::poll`已有Pending/Ready/Failed，但宿主需要保存原BuiltCoreGraph、App更新/egui/截图/采集副作用与WSI lease，按独审报告另行实施。

一轮窄修改可顺便增加只读阶段计时：ShaderHost::read文件数/字节/实际I/O时间；图声明decode/request时间；queue-full退避时间；prewarm未完成ticket数与实际等待；compile/record/submit CPU；warmup completion wait。日志聚合采样，不每文件持续刷屏，也不把“prewarm总时长”误标为磁盘加载或GPU执行时间。

## 本阶段实现与验证交接（2026-10-21）

已冻结 `application_programs.rs`、`production_renderer.rs`、`run_game.rs` 的窄实现。普通 splash gameplay 在现有 async load worker 中注册全部基础 27 程序、54 次变体文件读取，严格位于首次 App.update 前。loading App、skip-splash、公开 eager constructor 保持完整同步注册；原表、描述、变体及最终 final_post handle 均复用真实库。线程只捕获原 AssetShaderLibrary/ShaderHost Arc，不发送 bridge 的 Rc session。完整成功后才插 RegisteredApplicationPrograms，并在原 warmup 完成交接时校验 owner 和真实名字映射后回填 renderer slot。

取消在每次文件读取前检查，无法中断已经进入的 OS read。当前读取失败会保留具体 program/path 上下文和 backend 原 error，禁止 update/warmup/发布。helper 不丢弃 backend 已有 typed source；但真实 ShaderHost::read 的既有格式化 anyhow 错误已经丢失 typed I/O source，此项不应据 CPU fake-backend 测试宣称已恢复。部分库归失败 App 所有，现有 GameplayAppOwner 先关闭并 join save worker 再释放 World；不重试半注册库。生产 panic hook 可能 abort，因此不声称 catch_unwind 能恢复真实生产 panic。

新增 CPU 回归（由中央顺序运行；本 agent 未运行 Cargo/GPU）：`application_programs::tests` 三项覆盖 27/54 顺序和 desc/真实 final_post、缺第二变体停止且无首程序插入、预取消无读取；`save_shutdown_preparation_tests` 新增三项覆盖初始化失败无 update/warmup/发布、单次初始化先于真实首次 update（随后真实缺 catalog 拒绝）、中途取消停止并 join save。CPU RecordingBackend 的 payload 不可执行 GPU，若 registration 请求 pipeline 立即拒绝；不能把这些测试算 GPU 准备完成证据。

`TG_FRAME_TIMING=1` 增加 gameplay 前 3 帧和每 30 帧 `render_stages`：原 main graph declare / global-host prewarm / physical compile / record-submit，collect，frames-in-flight 的 PresentTicket.wait，实际 acquire，presentation resolve inclusive，present，以及 queue_trim 实际耗时和请求预算。每条带 gameplay_n/graph/receipt/submitted/present_queued/error；resolve 总量含其原声明、prewarm、compile、execute，不能与 main 阶段重复加总。关闭开关无新增 Instant 采样。原 wait、timeout、cancel_built、错误传播及帧节奏未删除。

另修正 `last_gpu_frame_ns` 口径：原 0x140beaae0 为两个 GPU timestamp 差乘 timestampPeriod；当前 session.last_known_frame_duration_ns 是提交到 collect retirement 的墙钟间隔，含 CPU 间隔/节流，不能输入原 GPU pacer。因此宿主在真实 top-level GPU span 尚未恢复时返回 None，仅诊断记录 `observed_completion_wall_ms`。不以 pass sum/max 替代，不改变 core FramePacer、用户开关、PresentTicket、frames_in_flight 或 queue trim 算法。此项是确定口径修复，尚不能据此宣称已定位低 FPS 根因。

仍需中央 Release 运行同完整场景、画质、实际 saved background 分支，测 worker 注册时间与启动生命周期、stage timing，并按根 `PERFORMANCE-STANDARD.md` 验收最低 60 FPS、合格 120 FPS。此阶段仅异步第二套基础 shader 文件读/注册，**没有宣称 pipeline decode/prewarm/纹理上传或完整 boot 资源已异步化**。
