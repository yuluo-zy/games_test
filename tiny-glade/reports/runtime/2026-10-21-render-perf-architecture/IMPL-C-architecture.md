# 第 C 层实施规格:C1 保留图、C2 GPU 驱动绘制、C3 pass 合并

2026-10-21。只读分析,未构建、未运行、未改源码。对应 `ANALYSIS.md` §3 "第 C 层" 与 §5 线程架构。所有行号以当前工作区为准(`crates/tiny-glade/src/*`、`crates/country-core/src/*`、`crates/rhapsody/src/*`、`shaders/*`)。标注【未核实】的句子是推断或未在代码中找到直接证据。

本文假设 A 层(A1 单帧在飞、A2 异步上传、A4 VAO 快照一次)与 B 层(B1 sync bind 增量化、B2 提交不再 replay)已经或将要落地;C1/C2 的收益以 `render_stages` 的 `declare_ms + record_submit_ms < 3 ms` 为门(ANALYSIS §3 顺序与验收门第 4 条)。

---

## 0. 一页总览

| 项 | 今天每帧做什么 | 改后每帧做什么 | 预期消灭的成本 |
|---|---|---|---|
| C1 保留图 | `begin_frame` 新建 `RenderGraph`,W1..W54 全部重新声明(160 pass、3270 转换),计划缓存命中仍要做整图 `matches_declaration` 比较,再 `instantiate_validated_plan` | 宿主持有 `RetainedGraph { Arc<CompiledPlan> + Vec<Box<dyn PassRecorder>> }`;每帧只算一个 `GraphShapeKey`,相同则 `begin_frame_with(plan, FrameInputs)` 直接实例化;pass 回调从 `FrameParams` 读本帧数据 | 声明阶段 O(160 pass + 全部 access) 的构图、校验、字符串键、`frame_sampled_table` 克隆、`raster_has_previous_contents` 扫描;debug 基线 227 ms 的 declare 大部分 |
| C2 GPU 驱动绘制 | 光栅 pass 对每个 draw:`DrawBuilder::new` 分配常量块、`bind_common` 写 5 个 uniform、`bindless_parameters` 建 Vec + 查 image_view + `prepare` 写参数页 + `cmd_push_constants`,再录一个 draw | 每个 draw list 一个 `DrawParamsSsbo`(列表 revision 变化时才重写);每个 (管线状态, VAO) 桶一次 `cmd_bind_pipeline` + 一次 `vkCmdDrawIndexedIndirectCount`;shader 用 `DrawIndex` 读参数 | 录制阶段 Shadows×3 92.7 ms、G-buffer 87.8 ms、G-buffer OC/2 45.4 ms(debug/lavapipe) |
| C3 pass 合并 | Nani cull & bucket ×10、Clear buffer ×6、Shadows ×3 各是独立 pass,各自一次屏障计算与命令缓冲切分 | 同形 pass 合并为单 pass 多 dispatch / 多 rendering section | pass 数 160 → ≈ 145;屏障与 PGO 记录项同比减少 |

---

## C1. 保留图(Retained Graph)

### C1.1 现状

**(1) 每帧从零建图。** `crates/tiny-glade/src/production_renderer.rs:510-557`:

```rust
fn compile_full_world_graph(renderer, world, lists, extent, egui) -> rhapsody::Result<CompiledGameGraph> {
    let started = renderer.timing_start();
    let gpu = &mut renderer.egui;
    let result = renderer.bridge.build_frame(|frame| {
        frame.with_main_prelude(world, |frame, world| {
            build_full_world_stage(frame, world, lists, extent, None, Some((gpu, egui)))
        })
    });
    renderer.timing_end(TimedStage::Declare, started);
    let (output, built) = result?;
    ...
    let prewarm = renderer.bridge.shader_backend().prewarm(PIPELINE_PREWARM_TIMEOUT);
    ...
    let result = renderer.bridge.compile_built(built);
```

`crates/tiny-glade/src/render_bridge.rs:254-294` `build_frame`:

```rust
let graph = self.session.borrow_mut().begin_frame()?;
let graph_id = graph.graph_id();
self.pending_build.set(Some(graph_id));
let declarer = Rc::new(FrameDeclarer { session: self.session.clone(), scope: FrameScope { .. } });
let mut frame = CoreGraphFrame { bridge: self, declarer, graph: Some(Rc::new(RefCell::new(graph))), failed: false };
let value = build(&mut frame)?;
...
Ok((value, BuiltCoreGraph { session, pending_build, graph: Some(graph.build()) }))
```

`crates/rhapsody/src/rg/vulkan.rs:2412-2456` `begin_frame`:每帧清空 `history_validity`、`ledger`、`specifications`、`synchronization`、`graph_waits`、`graph_signals`、`submissions`,然后 `let graph = graph::RenderGraph::new();`(第 2451 行)。

**(2) 声明序列。** `crates/tiny-glade/src/world_graph.rs:91-159` `build_world` 顺序调用 W1..W54(`begin_depth_history` → `upload_world_rasters` → … → `build_world_postprocess`)。每个阶段函数在声明期直接读 `World` 资源并把值烘焙进闭包:例如第 146-150 行

```rust
let video = &world.get_resource::<GameConfig>().ok_or_else(|| missing("GameConfig"))?.video;
let history_mode = if video.upscaling_mode == UpscalingMode::Tgsr {
    ColorHistoryMode::Tgsr { display_dims: world.get_resource::<FrameRenderConfig>()...extent_8 }
} else { ColorHistoryMode::Taa };
```

第 155 行 `let skip_refraction = world.resource::<GameConfig>().dev.demo;`。

**(3) 回调类型是捕获式闭包。** `crates/rhapsody/src/rg/graph.rs:197`:

```rust
pub type RenderCallback = Box<dyn for<'a> FnMut(super::executor::PassContext<'a>) -> bool + Send>;
```

`crates/rhapsody/src/rg/pass_builder.rs:400-411` `render_with_context` 把 `FnOnce` 包成 `Option<FnOnce>` 的 `FnMut`;`graph.rs:237-242` `RenderGraph { declaration: GraphDeclaration, callbacks: Vec<RenderCallback>, retire_callbacks }`,声明与回调一一对应(`executor.rs:850-852` `callback_count != graph.passes.len()` 即报 `CallbackCount`)。

**(4) 回调捕获了什么(三类代表 pass)。**

- 光栅 pass(Shadows / G-buffer):`crates/tiny-glade/src/raster_host.rs:1551-1725`。闭包 `move |ctx|` 捕获 `prepared: Vec<PreparedDraw>`(每个元素含 `DrawPacket`——其中 `bindings: HashMap<u32, GpuOwned>`、`accesses`、`unknown_bytes`——以及 `Arc<RasterEntry>`、索引/顶点句柄、`external: TempList<ExternalDrawBinding>`,见 `raster_host.rs:718-734`)、`shared: Arc<RasterShared>`、`colors`、`bindings`、`device`。录制时逐 draw `try_get` 管线(1561)、`cmd_bind_pipeline`(1679)、`bindless_parameters(...)`(1685)、`parameters.bind_pass`(1701)。闭包内 `shared.shaders.frame_globals_snapshot()`(1674)取本帧相机常量块——这是闭包里唯一"运行期取本帧数据"的地方,其余全在声明期烘焙。
- compute pass(Nani cull & bucket):`crates/country-core/src/systems/render_loop/compute_passes/gpu_generate_draw_lists.rs:229-239`

  ```rust
  pub fn cull_and_bucket_dispatch(api: &mut dyn ComputeDispatchApi, items: &[(PushConstants, u32)]) {
      for (pc, count) in items { api.push_constants(&pc.to_bytes()); api.dispatch_3d(DispatchArgs { x: *count, .. }); }
  }
  ```

  由 `.dispatch_with(move |api| cull_and_bucket_dispatch(api, &main))`(第 963 行)装进 `DispatchMode::With(Box<dyn FnOnce(&mut dyn ComputeDispatchApi) + Send>)`(`compute_pass_builder.rs:180`)。`Vec<(PushConstants, u32)>` 在声明期由 `owned_cull_batch`(768)从 `GpuWorld` 算出;HZB 维度经 `.uniform(H_HZB_DIMS, res.hzb_dims)`(311)在声明期写入动态常量块。
- 后处理(DoF/TAA):`crates/tiny-glade/src/world_postprocess.rs:95-219`。声明期读 `GameConfig.video`(102-106)、`AppMode`(107-111)、`ImageFormationConfigs`(112-118)、`FrameRenderConfig`(119-121);第 150-169 行按 `dof_enabled` 决定是否声明 `generate_mips` + `calc_dof` 两组 pass;第 187-211 行按 `use_tgsr` 选 `calculate_tgsr` 或 `calculate_taa`。`sharpen_amount`、`bypass_clamp`、`history_global` 作为 uniform 值烘焙进 pass。

**(5) 计划缓存已存在但不省声明。** `crates/rhapsody/src/rg/executor.rs:865-909`:每帧 `plan_cache.try_get_matching(|key| key.0.definition.matches_declaration(&graph, dependency, context))`。`crates/rhapsody/src/rg/plan.rs:140-205` `matches_declaration` 逐资源比较 `sharing/name/descriptor/origin`,逐 pass 比较 `name/queue_class/side_effect/accesses`,再 `graph.validate()` 与 `validate_matching_initialization`——命中也是 O(资源 + pass + access) 的完整比较,且前提是整张 `GraphDeclaration` 已经被重新构造出来。命中后 `instantiate_validated_plan`(`graph.rs:922-934`)→ `instantiate_with_storage`(936-1035):按 `plan.retained_passes` 过滤 pass、`prepare_transient_resources`、`populate_resources_into`、填 `history_validity`、建 `physical_mapping`。

**(6) 声明阶段附带的重复工作**(ANALYSIS §2.3):`SceneRasterHost::new` 每次 `refresh_draw_vaos`(`scene_raster.rs:72`,W23/25/30/34/43/44/46/53 共 8 次)、`frame_sampled_table` 每个光栅 pass 克隆全部视图并逐个 `import_texture`(`raster_host.rs:1403-1437`)、`validate_graph` 两次(`vulkan.rs:2932` 与 `:1158`)、`format!` 命名键(`vulkan.rs:2678/2739/2774-2776`)。

结论:今天"计划缓存命中"只省了 `compile_definition`(349 → 11 ms),声明本身(227 ms debug)一点没省。保留图的目标是把声明期整体换成"一次形状键比较"。

### C1.2 目标设计

#### (a) 宿主侧对象

新模块 `crates/tiny-glade/src/retained_graph/`(拆 `mod.rs`、`shape_key.rs`、`frame_params.rs`、`recorders/`),核心类型:

```rust
/// 决定图拓扑的全部宿主状态。每帧从 World 算一次,相等 ⇒ 复用 RetainedGraph。
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct GraphShapeKey {
    pub kind: ProductionGraphKind,                // Main{world_rendered, primary_rt} / Loading / Minimal
    pub world_extent: [u32; 2],                   // production_renderer.rs:315-328
    pub display_extent: [u32; 2],                 // production_renderer.rs:374-397 (FrameRenderConfig.extent_8)
    pub upscaling: UpscalingMode,                 // world_graph.rs:147
    pub dof_enabled: bool, pub multires_dof: bool,// world_postprocess.rs:150-154, 163
    pub photomode: bool,                          // AppMode native_tag == 0x13
    pub demo: bool,                               // GameConfig.dev.demo
    pub glade: GladeType,                         // ActiveSession.meta.glade → select_water_surface
    pub decision: MainWorldDecision,              // select_main_world (world_graph.rs:48-78)
    pub debug_rt: bool,                           // RENDER_OVERRIDE_SHOW_DEBUG_RT_VIEW
    pub freeze_occlusion_culling: bool,           // 常量 false,仍入键
    pub capture_purpose: u8,                      // screenshot::active_capture_purpose
    pub ui_visible: bool, pub logo_overlay: bool, pub cursor_visible: bool, // WorldTailControls
    pub shader_refactoring_disabled: bool,        // scene_raster.rs:92-101
    pub list_shape: [u64; DRAW_LIST_COUNT],       // 每列表 (shader, variant, vao, 外部绑定资源集) 的哈希,见 C1.3 第 13 条
    pub nani_subsets: u16,                        // 非空 GpuWorld 子集位掩码(gpu_generate_draw_lists.rs:954-1010)
    pub texture_table_revision: u64,              // AssetTextureLibrary 表变化计数(frame_sampled_table 的输入)
    pub egui_textures_revision: u64,              // egui 纹理增删
    pub device_generation: u64,                   // PlanContext.device_generation
}

/// 一张常驻图。plan 与 recorders 按 plan.retained_passes 对齐。
pub struct RetainedGraph {
    pub key: GraphShapeKey,
    pub plan: Arc<rhapsody::rg::CompiledPlan>,
    /// plan.definition.resources 里 Import 来源资源的逻辑下标 → 本帧 token 的求值器
    pub imports: Vec<ImportSlot>,
    /// 与 plan.definition.passes(原始声明序)一一对应;被裁剪的 pass 持 None
    pub passes: Vec<Option<RetainedPass>>,
    pub exports: ExportLayout,                    // 最终 MSAA 导出的逻辑下标与期望格式
    pub outputs: RetainedOutputs,                 // BuiltMainStage::texture 等逻辑下标
    built_at_frame: u64,
    stats: RetainedStats,                         // 复用次数、失效原因直方图
}

pub struct RetainedPass {
    pub name: Arc<str>,
    pub recorder: Box<dyn PassRecorder>,
}

pub enum ImportSlot {
    /// 资产库/受管表里的持久对象:token 固定,首帧解析后不变
    Fixed { token: u64, initial_access: u8 },
    /// 会话命名资源(named_texture/named_buffer):每帧向 session 要 token
    Named { key: Arc<str> },
    /// temporal 乒乓对的某一侧
    Temporal { key: Arc<str>, side: TemporalSide /* Current | Previous */ },
    /// 交换链图像(仅展示图)
    Swapchain,
}

pub trait PassRecorder: Send {
    /// 录制。只能读 `frame`,不得再读 World。
    fn record(&mut self, ctx: &rhapsody::rg::PassContext<'_>, frame: &FrameParams<'_>) -> rhapsody::Result<()>;
    /// 可选:本 pass 对 FrameParams 的依赖集合,用于测试/诊断
    fn reads(&self) -> FrameParamsMask { FrameParamsMask::ALL }
}
```

`FrameParams` 是"所有 pass 回调在录制期能看到的本帧数据"的唯一入口。字段由现有闭包捕获物归纳而来(C1.1 第 4 条):

```rust
pub struct FrameParams<'f> {
    pub frame_counter: u64,
    pub time: f32,                                        // Res<Time> elapsed → CommonDrawParams.time
    pub globals: Arc<crate::frame_setup::FrameGlobals>,   // frame_constants(b0 space1)与采样表
    pub constants: &'f Mutex<rhapsody::resource::DynamicConstants>,
    pub page_cursor: Mutex<rhapsody::bindless::BindlessPageCursor>,
    pub lists: &'f DrawLists,                             // C2 前:逐 draw 路径仍从这里取 entry
    pub list_revision: [u32; DRAW_LIST_COUNT],
    pub draw_params: Option<&'f DrawParamsTables>,        // C2 后:每列表 SSBO/桶表
    pub temporal: TemporalFrame,                          // 每个 pair 的 history_valid / parity
    pub controls: FrameControls,                          // 见下
    pub nani: NaniFrame,                                  // owned_cull_batch 的 Vec<(PushConstants,u32)>、HZB 维度
    pub egui: Option<EguiFrame>,                          // 本帧 ClippedPrimitive 与纹理增量
}

/// 今天在声明期烘焙成 uniform 的"值"。全部是 Copy。
#[derive(Clone, Copy)]
pub struct FrameControls {
    pub app_mode_tag: u32,                 // world_geometry.rs:59-60 grass_compute
    pub dof_focus: dof::FocusParams, pub dof_lens: dof::CameraLens, pub dof_lock: Option<u32>, pub mouse_pos: Option<[f32;2]>, // world_geometry.rs:83-98
    pub bokeh_anisotropy: f32, pub sharpen: f32, pub bypass_clamp: u8, pub disable_temporal_history: bool, // world_postprocess.rs:112-118, 195-209
    pub window: [u32; 2], pub cursor_ss: [f32; 2],
    pub hzb_dims: [u32; 2],
    pub time_of_day: ..., pub camera_mode: u8,
}
```

**类型边界说明**:`PassRecorder` 不持 `&World`、不持 `Rc`。它持有的是 (i) 逻辑资源下标(`u32`,等价于今天 `ResourceHandle::raw()`)、(ii) `Arc<RasterEntry>` / `Arc<ComputeEntry>`、(iii) 静态绑定表(binding slot → 逻辑资源 + 视图描述)、(iv) 从 `FrameControls` 取值并写 uniform 的小函数。所有 `managed.texture_for(id)` / `buffer_for(id)` 都在 `record` 里按逻辑下标解析(与今天 `raster_host.rs:1574-1576`、`2191-2193` 一致,只是 id 来自 recorder 字段而不是闭包捕获)。

三类 recorder 的形态:

```rust
/// 光栅 pass(C2 前的过渡形态:仍逐 draw,但 draw 来自 FrameParams.lists)
pub struct ListRasterRecorder {
    pub spec: ListPassStatic,              // name、colors/depth 逻辑下标、viewport 下标、mode、variant、flag
    pub list: usize,                       // DrawLists 下标
    pub table_reads: Vec<(u32, AccessRange)>,
    pub pipelines: Vec<(Arc<RasterEntry>, DrawStaticBindings)>, // 与 list_shape 对齐,按列表序
}
/// compute pass
pub struct ComputeRecorder {
    pub entry: Arc<ComputeEntry>,
    pub bindings: Vec<StaticBinding>,      // 逻辑资源/视图/访问
    pub uniforms: Box<dyn Fn(&FrameParams<'_>, &mut UniformBlockWriter) + Send + Sync>,
    pub dispatch: RetainedDispatch,        // Groups/Threads/IndirectRaw/With(Box<dyn Fn(&FrameParams, &mut dyn ComputeDispatchApi) + Send + Sync>)
}
/// 传输 pass(Clear buffer / Clear image / copy)
pub struct TransferRecorder { pub op: TransferOp /* FillBuffer{id} | ClearColor{id} | Copy{..} */ }
```

compute 侧的 `uniforms` 闭包把 `.uniform(H_HZB_DIMS, dims)` 这类声明期写值改成录制期写值:`|f, w| w.write(H_HZB_DIMS, f.controls.hzb_dims)`。它不捕获本帧对象,只捕获名字哈希与取值路径,因此跨帧复用。

#### (a') 每帧调用序列

```
ProductionRenderHost::frame():
  1. build_draw_lists()                              // 不变(A4 后 VAO 快照一次)
  2. key = GraphShapeKey::from_world(world, lists, kind)
  3. if retained.is_none() || retained.key != key || retained.dirty.is_some():
        retained = RetainedGraph::build(bridge, world, lists, key)   // 走今天的 build_frame 路径一次(见 C1.6),
                                                                      // 返回 Arc<CompiledPlan> + recorders
  4. frame_params = FrameParams::collect(world, lists, controls)      // O(常数) 读 World
  5. inputs = retained.frame_inputs(session)          // ImportSlot → token,temporal 配对,history_valid
  6. prepared = session.begin_frame_with(&retained.plan, inputs)?    // 新 API,C1.5
  7. receipt = session.execute_prepared_with(prepared, &mut retained.passes, &frame_params)?  // 新 API
  8. present / acquire 不变
```

第 2 步的键计算代价:~25 个标量字段 + 21 个列表哈希(每列表 O(entries) 的 FxHash,但 `build_draw_lists` 本来就 O(entries))+ 一个 `GpuWorld` 子集位掩码。目标 < 0.1 ms。

#### (b) 失效规则

见 C1.3。

#### (c) temporal parity

见 C1.4。

#### (d) rhapsody API

见 C1.5。

### C1.3 失效规则:哪些宿主事件改变拓扑

原则:**不靠事件钩子,靠键比较。** 任何改变 `GraphShapeKey` 的事件都会在下一帧第 2 步被发现;`RetainedGraph::invalidate(reason)` 只用于"键算不出来"的情况(设备重建、OOM 重置、手动)。下表列出每个事件、它在今天代码里改变拓扑的证据、入键字段、以及是否能避免失效。

| # | 事件 | 证据(今天拓扑随之变化) | 键字段 / `InvalidateReason` | 能否不失效 |
|---|---|---|---|---|
| 1 | 世界渲染决策(菜单开/关、静态捕获复用、profiling) | `world_graph.rs:48-78` `select_main_world`;`production_renderer.rs:427-445` 三条分支(完整 W1..54 / primary_rt_debug / dummy) | `decision`, `debug_rt` → `Reason::MainDecision` | 否;三种形状各保留一张 `RetainedGraph`(`HashMap<GraphShapeKey, RetainedGraph>`,上限 4 张,LRU) |
| 2 | TAA ↔ TGSR | `world_graph.rs:147-150`;`world_postprocess.rs:133-211`(history 名、尺寸、pass 集合都变) | `upscaling` → `Reason::Upscaling` | 否 |
| 3 | DoF 开关(含 photomode 独立开关)、multires_dof | `world_postprocess.rs:150-169`:`generate_mips` + `calc_dof` 存在与否 | `dof_enabled`, `multires_dof` → `Reason::Dof` | 否(可选优化:DoF 关时仍声明但 `calc_dof` 走 0 组 dispatch——会改变 TAA 输入资源,不等价,不做) |
| 4 | photomode 进出 | `world_postprocess.rs:107-111`;`world_geometry.rs:76,89-91`(autofocus 分支) | `photomode` | 否 |
| 5 | demo 标志 | `world_graph.rs:155` skip_refraction;`world_vegetation.rs:34-39`;`world_terrain.rs:401-416`;`world_postprocess.rs:337,881,1257` | `demo` | 启动后不变;入键只为安全 |
| 6 | 水面/冰面(季节,即"冬季切换") | `world_surface.rs:67` `select_water_surface(controls.glade)`;`d_main_tests.rs:451` `water_and_ice_plans_are_selected_by_season` | `glade` → `Reason::Season` | 否。【未核实】是否还有其它冬季专属 pass(未找到 `winter` 符号,季节只经 `GladeType` 进入图) |
| 7 | 世界分辨率 / 显示分辨率 / `world_resolution_scale` | `production_renderer.rs:315-328, 374-397`;所有 Create 资源 descriptor 变 → `CacheReason::Descriptor`(`plan.rs:105-112`) | `world_extent`, `display_extent` → `Reason::Resolution` | 否 |
| 8 | 截图机会 pass | `production_renderer.rs:495` `shared_texture_capture::opportunity(frame, world, 6, ..)`;`world_postprocess.rs:1226, 1257-1258, 1312`;pass 记录里的 "Capture screenshot buffer" | `capture_purpose` → `Reason::Capture` | **可避免**:截图是偶发事件,为它重建一次图不值得;改为固定声明 capture pass、但 recorder 在 `capture_purpose == 7`(无请求)时录空。需要确认 capture pass 的读访问不会引入额外屏障——它读最终 MSAA 导出,和 export 同布局【未核实】。第一版入键(简单正确),第二版再固定 |
| 9 | UI 可见性(Vermeer / photomode)、logo、自定义光标 | `world_postprocess.rs:1262-1266` `if ui_visible { … }`;`resolve_world_tail_controls` 328-340 | `ui_visible`, `logo_overlay`, `cursor_visible` | 否 |
| 10 | egui 内容变化 | `production_renderer.rs:480-493` `gpu.draw(frame, backend, ..)`:pass 数固定("UI" ×2),但 draw 的纹理集可能变 | `egui_textures_revision`(只在纹理增删时变;形状变化不入键) | **大部分帧不失效**:UI pass recorder 每帧读 `FrameParams.egui` 的 primitive 列表 |
| 11 | shader refactoring 开关 | `scene_raster.rs:92-101`;`d_main_tests.rs:270` `main_pass_skips_forward_subpasses_when_shader_refactoring_is_not_disabled` | `shader_refactoring_disabled` | 否 |
| 12 | 空列表跳过 pass | `d_main_tests.rs:343` `late_passes_are_skipped_when_their_list_is_empty`;`main_pass.rs:235-245` | 并入 `list_shape[i] == EMPTY_HASH` | **可避免**(推荐):保留图模式下始终声明这些 pass,列表空则 recorder 录 0 个 draw。一个只有 LOAD 附件、无 draw 的 `vkCmdBeginRendering/End` 是 no-op,像素等价;代价是空 pass 的屏障仍会插入(与 LOAD 布局一致,通常无转换)。此偏差要在像素等价测试中显式覆盖(C1.8) |
| 13 | draw list 内容形状(实体增删、材质/VAO 变化) | `raster_host.rs:1455-1523` 把每个 draw 的 indirect 缓冲、frame_constants、外部绑定、索引、顶点都声明为 pass 访问 ⇒ 多一个用了新纹理的实体就多一条访问 | `list_shape[i]` = FxHash(sorted (shader, variant, vao, external binding 资源 id 集合)) → `Reason::DrawListShape(i)` | 编辑时会失效,相机移动/时间流逝不失效(transform、time 不入键)。C2 落地后访问集合缩成"每列表 SSBO + 该列表材质纹理集合",失效频率进一步降低 |
| 14 | Nani 子集出现/消失、实例数从 0 变非 0 | `gpu_generate_draw_lists.rs:954-1010`:`Some(subset)`/`continue` 与 `count == 0 → continue` 决定 cull pass 数 | `nani_subsets` 位掩码(含"非空") → `Reason::NaniSubsets` | 首次放置某类对象时失效一次;C3 合并后 cull 变单 pass 多 dispatch,此项可从键中删除 |
| 15 | temporal reset(`should_reset_temporal`) | `world_graph.rs:67-69`;`vulkan.rs:2836-2849` `reset_temporal` 只改 `history_valid` | 不入键 | 不失效:`FrameParams.temporal[name].history_valid=false` |
| 16 | 相机移动、时间、光照方向、编辑模式(AppMode)、鼠标位置 | 只改 uniform 值 / push constants(`world_geometry.rs:59-60` mode 作为 uniform) | 不入键 | 不失效——这是保留图的主要收益场景 |
| 17 | 纹理表变化(资产热更新、texture_watch 线程) | `raster_host.rs:1403-1437` 每 pass 声明 `frame_sampled_table()` 全部视图范围 | `texture_table_revision` → `Reason::TextureTable` | 热更新极少,失效可接受 |
| 18 | 设备代际 / OOM 重试 / `reset_all_temporals` / `clear_idle_resource_pool` | `vulkan.rs:2851-2890`;`graph.rs:898-905` 代际不符即拒绝 | `device_generation`;并显式 `invalidate(Reason::Device)` | 否 |
| 19 | 加载图 / 主图 / 展示图 | `ProductionGraphKind`;展示图在 A1 后并入主图,否则独立一张 `RetainedGraph` | `kind` | 各自一张 |
| 20 | 建筑上传 W11(`world_terrain.rs:72-78`) | 不改拓扑;是 A2 的异步上传 | 不入键 | 不失效 |

`RetainedGraph::invalidate`:

```rust
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum InvalidateReason { MainDecision, Upscaling, Dof, Photomode, Season, Resolution, Capture, Ui, EguiTextures, ShaderRefactoring, DrawListShape(u8), NaniSubsets, TextureTable, Device, Manual }
impl RetainedGraphs {
    pub fn invalidate(&mut self, reason: InvalidateReason) { self.dirty = Some(reason); self.stats.invalidations[reason] += 1; }
}
```

`render_stages` 日志增加 `retained_hit={0|1} retained_reason={..}`,与 `plan_cache_hit` 并列。验收期望:稳态编辑外 `retained_hit=1` 连续 ≥ 30 帧。

### C1.4 temporal parity 不触发新计划

**现状。** `vulkan.rs:2749-2802` `temporal_texture_with_sharing`:注册两个命名资源 `pair/{len}:{name}:0` 和 `:1`,`named.state.get_or_create_owned_pair(graph, key, ..)` 按会话 parity 决定哪一个是 current、哪一个是 previous,各自以 `ImportOwned { token }` 的来源声明进图;`history_validity.insert(token, valid)`(2796-2798)。`temporal.rs:24-46` `TemporalTexturePair { current, previous, parity }`,`history_valid()` 返回 previous 的 `reused`。`compile_frame`(`vulkan.rs:2583-2588`)在编译前 `import_into` 并 `export_temporal(&mut graph)`——给 current 追加一条 `0x37` 导出。执行后 `pending_parity` 在确认提交时翻转(`temporal.rs:229-241`)。

**关键事实:计划与 parity 无关。** `plan.rs:23-28` `ResourceDefinition { sharing, name, descriptor, origin: ResourceOriginKind }` 只记 `ImportOwned`,不记 token;`matches_declaration`(`plan.rs:160-170`)比较的也是 name/descriptor/origin。导出集合 `exports`(`plan.rs:89`)记逻辑下标与访问码,不记 token。所以同一个 `CompiledPlan` 对两种 parity 都有效——今天的计划缓存在 TAA 乒乓下连续命中(26/28)正是这个原因。

**保留图下的做法。**

1. `RetainedGraph.imports` 对一对 temporal 记两个 `ImportSlot::Temporal { key, side }`,逻辑下标固定(current 永远是声明时的那个下标)。
2. 每帧 `frame_inputs(session)` 调用新 API `session.resolve_temporal_pair(key, desc) -> TemporalBinding { current_token, previous_token, history_valid, parity }`(C1.5 第 4 项):内部等价于 `get_or_create_owned_pair` 但不需要 `RenderGraph`,只做 token 查找、`requested()` 登记与 `history_validity.insert`。
3. `FrameInputs.imports[current_index] = ImportBinding { token: current_token, initial_access: <confirmed_access of that token>, history_valid: Some(current_history_valid) }`,previous 同理。初始访问来自 `Record.confirmed_access`(`temporal.rs:118-121`),这正是 ARCHITECTURE §4.4 所说"选择作为运行时参数的状态则由绑定阶段重新校验并生成边界屏障"。
4. `history_valid` 不进计划、不进键:`PassContext::history_valid(id)`(`executor.rs:515`)继续从 `FrameInstanceTables.history_validity` 读;`instantiate_with_storage` 第 1007-1015 行按 token 从 `backend.history_validity(token)` 填表,保留图路径不变。pass recorder 通过 `frame.temporal[name].history_valid` 或 `ctx.history_valid(id)` 二选一(推荐后者,零迁移)。
5. 导出:plan 的 `exports` 里已经含 `export_temporal` 加的 `0x37` 条目(因为 `RetainedGraph::build` 走的是今天的 `compile_frame`,见 C1.6),`validate_compiled_exports`(`vulkan.rs:2596-2600`)在 `begin_frame_with` 中照常执行。
6. `reset_temporal` / `should_reset_temporal`:只改 `history_invalid`,不影响 plan。

需要注意的一个边界:`get_or_create_owned_pair` 在描述符变化(分辨率)时重建资源——此时 `GraphShapeKey.world_extent/display_extent` 已变,整图重建,不冲突。

### C1.5 rhapsody 必须新增的 API,以及保持不变的部分

**新增(全部在 `crates/rhapsody/src/rg/`):**

1. `executor.rs` — 回调抽象从闭包放宽为 trait:

   ```rust
   pub trait PassRecord: Send {
       fn record(&mut self, context: PassContext<'_>) -> bool;
   }
   impl<F: FnMut(PassContext<'_>) -> bool + Send> PassRecord for F { fn record(&mut self, c: PassContext<'_>) -> bool { self(c) } }
   // execute_prepared_validated / execute_prepared_inner / record_compiled 的 `callbacks: &mut [impl FnMut(..) + Send]` 改为 `&mut [impl PassRecord]`
   ```

   宿主侧每帧构造 `Vec<RecorderCall<'_>>`(`struct RecorderCall<'a> { recorder: &'a mut dyn PassRecorder, frame: &'a FrameParams<'a> }`,实现 `PassRecord`),长度 = `plan.definition.passes.len()`(被裁剪的 pass 给一个 `Noop`)。零 Box、零分配(用帧 arena 或 `SmallVec<[_; 192]>`)。并行录制(`RecordingPolicy::Chunked`)要求 `Send`:`FrameParams` 内的 `Mutex<BindlessPageCursor>` 与 `&Mutex<DynamicConstants>` 满足;`DrawLists` 只读。

2. `vulkan.rs` — 跳过声明/校验/查找的帧入口:

   ```rust
   pub struct ImportBinding { pub logical: u32, pub token: u64, pub initial_access: u8, pub history_valid: Option<bool> }
   pub struct FrameInputs { pub imports: SmallVec<[ImportBinding; 64]>, pub gated: Option<u32> /* 交换链 first-use 门 */ }
   impl VulkanGraphSession {
       /// 等价于 begin_frame 的会话清理(2418-2450)+ compile_frame 的命名资源准备(2583-2600)+ compile_built 的
       /// reserve_frame/receipt_storage(2933-2960),但不构造 RenderGraph、不 validate_graph、不查计划缓存、不做 matches_declaration。
       pub fn begin_frame_with(&mut self, plan: &Arc<CompiledPlan>, inputs: FrameInputs) -> crate::Result<PreparedSessionFrame>;
   }
   ```

   内部:`instantiate_from_plan(plan, inputs, pool, backend)`(新,`graph.rs`)—— 用 `plan.definition` 直接合成 `GraphDeclaration`(resources 的 `source` 由 `inputs.imports` 填,Create 保持 Create;passes 取 `plan.definition.passes`;exports/gated 来自 definition),然后调用现有 `instantiate_validated_plan`(922)。合成声明的成本是 O(资源 + pass) 的 memcpy,没有哈希、没有字符串、没有校验;后续可用 `FrameStoragePool` 缓存该声明。`validate_matching_initialization` 只对 `inputs.imports` 中 `initial_access` 与上次不同的资源执行(今天在 `matches_declaration` 末尾对全图执行)。

   `PreparedSessionFrame.callbacks` 字段在保留路径为空;`execute_prepared` 增加变体 `execute_prepared_with(frame, recorders: &mut [impl PassRecord])`,内部把 `callbacks.len()` 校验改为 `recorders.len() == plan.definition.passes.len()`。

3. `vulkan.rs` — 一次性取出计划与导入布局:

   ```rust
   pub struct RetainedPlan { pub plan: Arc<CompiledPlan>, pub imports: Vec<(u32 /*logical*/, ImportOrigin)>, pub exports: Vec<u16> }
   pub enum ImportOrigin { Owned { token: u64 }, Unowned { token: u64 }, Named { key: Arc<str> }, TemporalCurrent { key: Arc<str> }, TemporalPrevious { key: Arc<str> } }
   impl VulkanGraphSession {
       /// 走完整 compile_frame,但把 Arc<CompiledPlan> 与命名/temporal 的逻辑下标映射交给宿主。
       pub fn compile_frame_retained(&mut self, built: BuiltGraph) -> crate::Result<(PreparedSessionFrame, RetainedPlan)>;
   }
   ```

   `named.state` 已经知道每个请求对应的逻辑句柄(`TemporalRecordState::RawTexture(handle)`,`temporal.rs:72-76`),从那里导出 `ImportOrigin`。

4. `vulkan.rs` — 无图的命名/temporal 解析:

   ```rust
   pub struct TemporalBinding { pub current: u64, pub previous: u64, pub current_history_valid: bool, pub history_valid: bool, pub parity: u32 }
   pub fn resolve_temporal_pair(&mut self, name: &str, desc: TextureDesc) -> crate::Result<TemporalBinding>;
   pub fn resolve_named_texture(&mut self, name: &str, desc: TextureDesc) -> crate::Result<(u64, bool)>;
   pub fn resolve_named_buffer(&mut self, name: &str, desc: BufferDesc, zero_initialized: bool) -> crate::Result<(u64, bool)>;
   ```

   实现:把 `request_named`(2803-2834)与 `temporal_texture_with_sharing`(2762-2802)中"登记 + `history_validity.insert`"的部分抽出,去掉 `graph.texture_id(handle)` 转换;`named.state.requested()` 仍被填充,供 `begin_frame_with` 的 `import_into` 使用。

5. `plan.rs` — `CompiledPlan` 增加 `pub shape_hash: u64`(定义的 64 位哈希),供宿主做 `Arc::ptr_eq` 之外的诊断,也是 B5 的一部分。

6. `executor.rs` — `PassContext` 不变(`492-502`)。保留图路径仍提供 `managed`、`compiled`、`history_valid`。

**保持不变:**

- 规划器与 `CompiledPlan` 内容(`plan.rs:308-319`):`range_program`、`retained_passes`、`batches`、`queue_schedule`。
- `instantiate_validated_plan` 之后的一切:池分配、`history_validity` 填表、`sync_plan::bind`(B1 增量化后更省)、`record_compiled`、批次提交、`end_frame`、present。
- `PassContext::image_layout_for_range`、`retain_object`、`report_error`。
- 旧的 `begin_frame`/`compile_frame`/`execute_prepared`:作为 `TG_RETAINED_GRAPH=0` 的路径和 `RetainedGraph::build` 的内部路径继续存在。

**依赖于 B 层的点**:B1(bind 增量化)决定 `begin_frame_with` 之后 `bind` 是 O(导入数)还是 O(U×S);B4(去字符串)让 `GraphDeclaration` 合成不再 `String::clone`。没有 B 层,C1 只省声明不省实例化。

### C1.6 迁移方案

**总策略:先"录制一次"再"参数化"。** `RetainedGraph::build` 第一版不要求任何 W 阶段改写——它用今天的 `build_frame` 路径声明并编译一次(`compile_frame_retained`),拿到 `Arc<CompiledPlan>` 与导入布局;recorders 由一个"适配器"提供:对尚未迁移的阶段,recorder 就是今天的闭包(`LegacyRecorder(RenderCallback)`),但闭包只能用一次(`FnOnce` 语义,`pass_builder.rs:404-408`),所以第一版只能"同一帧内"复用——这证明接口而不省工作。真正的收益来自逐阶段把闭包改成 `PassRecorder`,顺序如下。

| 阶段 | W 范围 | 涉及文件 | 改法 | 验收 |
|---|---|---|---|---|
| M0 基建 | — | `retained_graph/*`、rhapsody C1.5 第 1-5 项、`production_renderer.rs` 第 2-7 步、`TG_RETAINED_GRAPH` 开关 | `RetainedGraph::build` + `GraphShapeKey` + `FrameParams`;所有 pass 走 `LegacyRecorder`,每帧仍重建(`retained_hit` 恒 0) | rhapsody 单测:`begin_frame_with` 与 `compile_frame` 对同一 `BuiltGraph` 产出相同 `physical_mapping`/`batches`;像素测试基线不变 |
| M1 Shadow 组 | W23(Shadows×3)+ W22 Nani cull 第一轮 + W24 terrain lighting | `scene_raster.rs:117-119`、`shadow_pass.rs:27-47`、`d_main_support.rs:258-308`、`raster_host.rs:1551-1725`(拆成 `ListRasterRecorder::record`)、`gpu_generate_draw_lists.rs:229-239,906-1010`(`ComputeRecorder`) | 光栅:把 `run_list_pass` 拆成"声明期:静态 spec + 访问集合"与"录制期:遍历 `frame.lists.list(0)` 建 packet/绑定参数"。注意 `DrawBuilder::new` 今天在声明期分配动态常量(`draw_builder.rs:353-376`),迁移后在录制期分配——`DynamicConstants` 已是 `Mutex`,并行录制安全;`bind_common` 的 `time` 改读 `frame.time` | `shipped_session_steady_state_multi_frame` 的 `multiframe-frame{02,10,30}.png` 与保留图关/开逐像素相等(C1.8) |
| M2 G-buffer 组 | W25 main_pass(G-buffer、velocity、emissive、Forward…)、W26 HZB、W27-29 wall pass2 / Nani 第二轮 / grass、W30 OC/2、W31-34 | `main_pass.rs`、`world_walls.rs`、`world_geometry.rs:54-104`、`generate_hzb` | 同 M1;`grass_compute` 的 `mode` 与 `calculate_dof_autofocus` 的 `FocusParams/CameraLens` 进 `FrameControls` | 同上 + `main_pass_runs_six_subpasses_in_order_with_observed_attachments` 等 d_main_tests 仍过(它们测声明期形状,不受影响) |
| M3 Lighting 组 | W35-44(half-res、reprojection、SSGI、RT GI、ReSTIR、screen shadows、deferred raster) | `world_lighting.rs:107-236`、`scene_compute.rs` 对应 host 方法 | temporal 对(`TAA_HISTORY`/`TGSR_HISTORY`/`PREV_DEPTH`)改走 `ImportSlot::Temporal`;`history.fresh as u8`(178, 220)改读 `ctx.history_valid` | 乒乓 30 帧 `retained_hit=1`;`WorldHistoryObservation` 序列与基线一致 |
| M4 Post 组 | W45-54 + tail(grading/blur/final_post/UI/egui/capture) | `world_postprocess.rs:95-219, 1061-1340`、`egui_gpu.rs`、`screenshot` | DoF/TGSR 分支进键;egui recorder 读 `frame.egui`;capture recorder 第一版入键 | 全图 `retained_hit=1`;`declare_ms` < 1 ms(release 真机) |
| M5 前段 | W1-W21(terrain / vegetation / geometry compute、W11 上传) | `world_terrain.rs`、`world_vegetation.rs`、`world_geometry.rs:30-50,109-180` | 多数是 `ComputeRecorder` 模板化;W11 上传依赖 A2 | 同上 |

每个 M 阶段结束时 `TG_RETAINED_GRAPH=0` 必须仍能跑且像素相同——因为旧闭包路径不删,只在 `RetainedGraph::build` 时使用。M5 完成后,`LegacyRecorder` 删除,旧路径只保留 `build_frame` 用于构建保留图。

**kill switch**:`TG_RETAINED_GRAPH`(默认 1;`0` 走旧路径)。实现位置 `production_renderer.rs` `compile_full_world_graph` 入口;读取一次缓存到 `ProductionRenderer`,与 `TG_FRAME_TIMING`(`game_loop.rs:135`)同风格。

### C1.7 改动清单

rhapsody(`crates/rhapsody/src/rg/`):

- `executor.rs`:`PassRecord` trait;`execute_prepared_validated`/`execute_prepared_inner`/`record_compiled` 泛型参数改 `impl PassRecord`;`PreparedGraph` 不变。
- `graph.rs`:`instantiate_from_plan(plan, inputs, pool, backend)`;`ImportBinding`;`validate_matching_initialization` 的增量版本(只校验变化的导入)。
- `vulkan.rs`:`begin_frame_with`、`execute_prepared_with`、`compile_frame_retained`、`resolve_temporal_pair`、`resolve_named_texture/buffer`;`begin_frame` 的清理段(2418-2450)抽成私有 `begin_frame_housekeeping()` 两边共用。
- `plan.rs`:`CompiledPlan.shape_hash`。
- `temporal.rs`:`TemporalRenderGraphState::get_or_create_owned_pair` 增加无图变体 `resolve_owned_pair(key, descriptor, owner) -> TemporalPair`。
- 测试:`tests/` 新增 `retained_frame.rs`:同一 `BuiltGraph` 经 `compile_frame` 与 `compile_frame_retained + begin_frame_with` 得到相同 `Snapshot`;temporal 乒乓 4 帧 token 交替且 plan `Arc::ptr_eq`;`CallbackCount` 等价错误仍返回。

tiny-glade(`crates/tiny-glade/src/`):

- 新 `retained_graph/{mod,shape_key,frame_params,recorders/{raster,compute,transfer,legacy}}.rs`。
- `production_renderer.rs`:`compile_full_world_graph` 分支;`RenderStageTiming` 增 `shape_key_ms`、`retained_hit`;`finish_render` 日志行增字段。
- `render_bridge.rs`:`CoreRenderBridge::begin_retained_frame(&Arc<CompiledPlan>, FrameInputs)`、`execute_retained(prepared, recorders, &FrameParams)`。
- `raster_host.rs`:1551-1725 的闭包体抽成 `fn record_list_pass(ctx, frame, static: &ListPassStatic, pipelines: &[..])`,供 `LegacyRecorder` 与 `ListRasterRecorder` 共用;`PreparedDraw` 的构造拆成"静态部分"(entry、table_reads、外部绑定资源 id)与"动态部分"(packet、常量块)。
- `scene_raster.rs`/`scene_compute.rs`:每个 host 方法增加 `retained_*` 变体,返回 recorder 而非直接 `env.record`。
- country-core:`compute_pass_builder.rs` 增加 `DispatchMode::WithFrame(Box<dyn Fn(&dyn FrameParamsView, &mut dyn ComputeDispatchApi) + Send + Sync>)`(country-core 不依赖 tiny-glade,用一个只读 trait `FrameParamsView` 暴露 `controls`);`d_main_support.rs` `run_list_pass` 拆声明/录制两半。
- `game_loop.rs`:`TG_RETAINED_GRAPH` 读取。

### C1.8 验证

1. **像素等价(主门)**。复用 `crates/tiny-glade/tests/original_gameplay_chain.rs:755` `shipped_session_steady_state_multi_frame`(命令:`cargo test -p tiny-glade --features native-audio --test original_gameplay_chain shipped_session`,`TG_MULTIFRAME_OUT` 输出 `multiframe-frame{02,03,04,05,10,20,30}.png`,见 `reports/runtime/2026-10-19-main-loop-render-recovery/MULTIFRAME-BASELINE-2026-10-20.md:3-5`)。新增测试 `retained_graph_matches_rebuilt_path`:同一 shipped session 跑两遍 30 帧,一遍 `TG_RETAINED_GRAPH=0`、一遍 `=1`,对 CAPTURES `[1,2,3,4,9,19,29]` 的 RGBA 逐字节比较,阈值 0。两遍起点都是冷 TAA 历史、相同 `Time` 步进(测试里 `driver.app_mut().update()` 固定步长【未核实:Time 是否用真实时钟;若是,需要注入固定 dt】),因此应逐位相同。lavapipe 的 compute 调度是确定性的【未核实】。
2. **形状键稳定性**:静止 30 帧 `retained_hit` = 29/30;放置一面墙 → 恰好一次 `Reason::DrawListShape`/`NaniSubsets`;切换 DoF → 一次 `Reason::Dof`。
3. **temporal**:`WorldHistoryObservation.color_history_valid` 序列 `[false, true, true, ...]` 与基线一致;`parity` 交替。
4. **性能门**:真机 release `render_stages` `declare_ms` 从基线(待第 0 步测量)降到 < 1 ms;`compile_ms` ≈ 实例化时间;`record_submit_ms` 不变(C2 负责)。
5. **错误路径**:recorder 返回 `Err` → `ctx.report_error`,事务取消,下一帧 `retained` 仍可复用(不应因一次录制错误丢图);设备丢失 → `invalidate(Device)`。
6. **现有单测**不受影响:d_main_tests、`main_selection_tests`、rhapsody 计划缓存测试。

### C1.9 风险

- **录制期分配 vs 声明期分配**。今天 `DrawBuilder::new` 在声明期拿 `DynamicConstants` 块、`frame_sampled_table` 在声明期导入纹理;迁到录制期后,并行录制线程同时访问 `DynamicConstants`(已 `Mutex`)和 `BindlessPageCursor`(每线程一个游标更好)。锁争用需测;B6(一页一个 cell)是前提。
- **`matches_declaration` 的隐式校验消失**。它每帧跑 `validate_matching_initialization`(未定义内容读取检查)。保留路径只对变化的导入做;若宿主在不改键的情况下改变了某个导入的初始状态(例如资产库重建纹理但 token 不变),校验会漏。缓解:`ImportSlot::Fixed` 记录 `initial_access`,与 `confirmed_access` 不一致时整图失效。
- **形状键遗漏**。任何没入键却影响拓扑的状态都会造成"用旧图画新世界"——通常表现为 `managed.texture_for(id)` 失败或 `plan_check` 拒绝,而不是静默错图,因为逻辑下标越界。但访问范围变化(同一资源不同 mip 范围)不会越界,会静默漏屏障。缓解:debug 构建下每 N 帧用旧路径重建一次并 `definition == plan.definition` 断言(`graph.rs:911-915` 已有此比较)。
- **多张保留图的显存**:每张图的瞬态资源来自池,不常驻;`CompiledPlan` 本身 < 1 MiB(`plan_cache` 上限 16 MiB/128 项,`executor.rs:1559`)。
- **与 A1 的耦合**:展示图并入主图后键多一项 `swapchain_format/extent`;交换链重建即失效。
- **PGO 分块**(A6)以 pass 名统计;保留图 pass 名不变,统计可延续。

---

## C2. GPU 驱动绘制

### C2.1 现状

**(1) draw list 结构。** `crates/country-core/src/systems/render_loop/draw_passes/draw_lists.rs:47` `pub const DRAW_LIST_COUNT: usize = 21;`,`:113-114` `pub struct DrawLists { lists: [Vec<DrawEntry>; DRAW_LIST_COUNT] }`,`:138-142` 每帧按 `sort_key`(i32)稳定排序。元素 `(DrawItem, DrawListFn)`(`draw_list.rs:131`),`DrawItem { params: EntityDrawParams { topology, entity, shader_name, transform: DrawTransform }, vao, shader, sort_key, stencil_ref }`(`draw_lists.rs:320-346` `push_row`)。`sort_key` 来自分发表常量(`draw_lists.rs:888-1098`:1000/3100/3300/3500/…/15012),即**按材质分类而不是按实例**,同一材质的所有实体 key 相同,排序后天然成桶。宿主每帧 `build_draw_lists(world, shaders, &mut lists); lists.sort(); refresh_draw_vaos(..)`(`production_renderer.rs:1409-1421`)。

**(2) 每 draw 的声明期工作。** `d_main_support.rs:258-308` `run_list_pass`:

```rust
for (item, func) in items {
    let mut state = pass.host.default_raster_state();
    let packet = item.to_draw_packet(state);
    let mut builder = DrawBuilder::new(&mut pass, packet, spec.variant);      // 分配动态常量块(draw_builder.rs:353-376)
    let mut builder = builder.bind_common::<ColorLibrarySsbo>(&params, &item.params.transform); // 5 个 uniform + 预设纹理 + 颜色库(1062-1090)
    ...
    func(builder, &item.params, &mut *ctx.world);                              // 材质回调:再绑若干纹理/SSBO,最后 finish(DrawKind)
}
pass.render();
```

`bind_common`(`draw_builder.rs:1062-1090`)写 `BIND_COMMON_FLAG_UNIFORM`、`BIND_COMMON_MODEL_MATRIX`、`BIND_COMMON_FLAG_INT_UNIFORM`、`BIND_COMMON_TIME`、`BIND_COMMON_VIEWPORT`,并绑定一张预设纹理与颜色库 SSBO。材质回调以 `multi_draw_indexed_indirect`(`draw_builder.rs:956-966`,→ `DrawKind::IndexedIndirect`,索引数取静态表 `INDEXED_INDIRECT_INDEX_COUNTS`,85-90)或 `nani_draw`(`render/nani/draw_ext.rs:124-200`,→ `DrawKind::IndexedIndirectCount`,`count_offset = list_index << 2`)或普通 `draw_indexed` 结束。

**(3) 每 draw 的录制期工作。** `raster_host.rs:1676-1722`:

```rust
for (draw, pipeline) in prepared.iter().zip(&pipelines) {
    unsafe { device.raw_device().cmd_bind_pipeline(command, GRAPHICS, pipeline.raw_handle()) };
    let parameters = match bindless_parameters(&shared, &ctx, draw, &bindings, managed, &mut page_cursor, frozen_globals.clone()) { .. };
    if let Err(error) = unsafe { parameters.bind_pass(&ctx, pipeline.clone()) } { .. }
    ... set_stencil_ref / set_line_width ...
    if let Err(error) = record_draw(&mut api, draw, managed, count_form) { .. }
}
```

`bindless_parameters`(`raster_host.rs:2144-2269`):`let mut resources = Vec::new();` → 遍历 `packet.bindings`(uniform 块 lease 校验)→ 遍历 `draw.external`(`managed.texture_for` + `texture.image_view(desc)` + `draw.entry.bindings.image(..)`)→ 遍历 `packet.accesses`(再一次 `texture_for` / `image_view` / `image_for_pass`)→ `snapshot_with_globals(globals)` → `snapshot.prepare(cursor, push, resources, None, &shared.dynamic)`(`render_host.rs:1291-1331+`:`resource_requirements`、`cursor.prepare(|group| …)`、全局页准备、采样器页准备)。`BindlessParameters::finish`(`bindless_parameters.rs:411-449`):

```rust
let chunk = constants.allocate_with_data(&self.bytes)?;   // 参数块写进动态常量
let lease = self.heap.parameter_buffer(&chunk)?;            // 堆槽位
let parameter_index = lease.index.index;
```

`bind_pass`(493-508)→ `heap.bind_pass` + `cmd_push_constants(.., 0, &parameter_index.to_le_bytes())`(539-552)。`record_count(RecordCount::NativePushCalls, 1)` 等计数器说明这条路已经被剖析过。

**(4) shader 侧如何收参数。** shader-compiler 的 bindless 降级(`crates/rhapsody/crates/shader-compiler/src/bindless.rs`):`BindlessAbi { original_push_bytes, parameter_bytes, bindings: Vec<BindingSlot { location, kind, array, parameter_offset, parameter_count }> }`(21-35);参数块 = 原 push 字节(4 对齐)+ 每个绑定一个 u32 堆下标(60-104);`parameter_index`(282-300)从 push constant 第 0 字读参数块下标,`resource_index`(302-315)`= params[block][indices_member + slot]`。即:**一个 draw = 一个参数块 = 一次 push + 一次常量分配 + 一次堆租约**。

HLSL 侧(`shaders/nani_solid_vertex_color_bricks.raster.hlsl`):`cbuffer frame_constants : register(b0, space1)`(57)、`cbuffer $Globals : register(b13, space0)`(88,即 `bind_common` 写的那块 uniform)、`ByteAddressBuffer instance_indirection_buffer : t14`、`nani_instance_data : t15`(100-101)、`uint gl_InstanceIndex : SV_InstanceID;`(125)。415 个 HLSL 文件中没有任何 `DrawIndex`/`gl_DrawID` 使用(grep 无命中);Nani 系列用 `SV_InstanceID` 索引实例数据。`billboard_plant.raster.hlsl:44` 注释"$Globals members (implicit cbuffer at set0/b13)"。

**(5) 设备能力。** `crates/rhapsody/src/device/caps.rs:75-79`:`MultiDrawIndirect = 28`、`DrawIndirectFirstInstance = 29`、`DrawIndirectCount = 30`;`init.rs:550-556` 探测、`:706-708` 启用。**没有** `shaderDrawParameters`(Vulkan 1.1 `VkPhysicalDeviceVulkan11Features::shaderDrawParameters`,SPIR-V `DrawParameters` capability)。`raster_host.rs:1675` `plain_indirect_needs_count_form(&device)` 与 `rg/indirect_count.rs` 的 `IndexedCountAdapter`(85-160)说明不支持 `DrawIndirectCount` 的设备已有 compute 降级路径。

**(6) 已有的 GPU 剔除。** `gpu_generate_draw_lists.rs:906-1010`:两轮(`world_graph.rs:122` 无 HZB、`:136-137` 有 HZB)。每轮:clear draw lists / histograms / visibility → `prepare_unsorted_draw_lists` → `Nani cull & bucket`(Main + 各子集,每个一个 pass,`dispatch_with` 逐子集 push + dispatch)→ histogram prefix → atlas idx prefix / fill / `Nani generate draws`(写 `VkDrawIndexedIndirectCommand` 列表)+ `NaniDrawListLengthSsbo` 计数。消费端 `nani_draw`(`draw_ext.rs:124-200`)发 `IndexedIndirectCount { buffer, count_buffer, args }`,一次 `vkCmdDrawIndexedIndirectCount` 画一个子集。**结论:Nani 子集已经是 GPU 驱动的;C2 的对象是 21 个 CPU 列表里其余的 draw(bricks/grass/water/stairs/pebbles/billboards/critters 等),它们每个都是一次 `IndexedIndirect`(1 条命令)或 `draw_indexed`,但参数准备是逐 draw 的。**

### C2.2 目标设计

#### (a) 每列表 `DrawParamsSsbo`

```hlsl
// shaders/common/draw_params.inc.hlsl(新)
struct DrawParams {                 // 128 B,std430,16 B 对齐
    float4x4 model;                 // +0   64 B   DrawTransform::compute_matrix()
    uint     material_id;           // +64         DrawPacket.variant << 8 | stencil_ref(unknown_b1)
    uint     common_flag;           // +68         CommonDrawFlag(0..4;阴影 cascade+2 由 pass 块覆盖,见 (e))
    uint     color_library_slot;    // +72         bind_color_library 的 SSBO 堆下标
    uint     preset_texture;        // +76         BIND_COMMON_PRESET_ON/OFF 纹理堆下标
    uint     texture_slots[8];      // +80  32 B   材质回调 bind_texture_* 的堆下标(按 binding 序;不足填 0xFFFFFFFF)
    uint     buffer_slots[4];       // +112 16 B   材质回调 read_storage_buffer 的堆下标
};
struct DrawParamsHeader { uint revision; uint draw_count; uint stride; uint pad; };   // 16 B
// 缓冲布局:[Header][DrawParams × N][VkDrawIndexedIndirectCommand × N (20 B,stride 32 对齐)][uint count × bucket_count]
```

Rust 侧(`crates/tiny-glade/src/gpu_driven/draw_params.rs`,新):

```rust
#[repr(C, align(16))] pub struct DrawParamsGpu { model: [[f32;4];4], material_id: u32, common_flag: u32, color_library_slot: u32, preset_texture: u32, texture_slots: [u32;8], buffer_slots: [u32;4] }
const _: () = assert!(size_of::<DrawParamsGpu>() == 128);

pub struct DrawListGpu {
    pub revision: u32,                       // 与 DrawListRevision 相同即跳过写
    pub params: Arc<Buffer>,                 // GpuOnly,STORAGE | INDIRECT | TRANSFER_DST;容量按 2 的幂增长
    pub buckets: Vec<DrawBucket>,            // 见 (c)
    pub commands_offset: u64, pub counts_offset: u64,
    pub staging: UploadTicket,               // A2 的异步上传,不等待
}
pub struct DrawParamsTables { pub lists: [Option<DrawListGpu>; DRAW_LIST_COUNT] }
```

**列表 revision 定义。** `DrawListRevision(u64) = FxHash( for entry in list: (shader.id, variant, vao.id, transform bits (12×f32 原始位), stencil_ref, entity, 材质回调的绑定结果哈希) )`。其中"材质回调的绑定结果"今天只有在执行 `func(builder, params, world)` 后才知道(它读 World 决定绑哪张纹理);C2 要求每个材质回调拆成"纯函数 `bind_static(&EntityDrawParams, &World) -> MaterialBindings`"(C2.3 改动清单第 4 项),revision 把 `MaterialBindings` 的堆下标序列也哈希进去。transform 入哈希意味着动画实体每帧都使 revision 变化——但变化的只是该条目,见下。

**写入策略。** `build_draw_lists` 之后、声明/录制之前(`production_renderer.rs:1409-1421` 的 `build_draw_lists` 内),对每个列表:

1. `revision == gpu.revision` → 不写。
2. 否则按条目比较旧哈希数组 `entry_hash[i]`,只把变化的条目拷进 staging(`UploadManager` ticket,A2),连同它的 `VkDrawIndexedIndirectCommand`;条目数或桶划分变化则整表重写。
3. 计数缓冲每桶一个 `uint`,CPU 写(第一版无 GPU 剔除,count = 桶长度)。

典型帧(相机动、无编辑):所有静态实体 revision 不变 → 零上传;只有动画实体(critters 走 instanced 路径,不在此列【未核实】)重写各自条目。

#### (b) shader 改动:按 `DrawIndex` 取参数

```hlsl
// 每个受影响的 .raster.hlsl 顶点入口
[[vk::builtin("DrawIndex")]] uint draw_id : DRAWID;          // DXC 支持的 vk::builtin,映射 SPIR-V BuiltIn DrawIndex【未核实:需在 dxc 版本上确认】
StructuredBuffer<DrawParams> draw_params : register(t30, space0);   // 经 bindless 降级成堆下标,放在 pass 参数块
...
DrawParams dp = draw_params[draw_id];
float4x4 model = dp.model;                                   // 替代 $Globals.model_matrix
Texture2D tex = g_textures[NonUniformResourceIndex(dp.texture_slots[0])];
```

- `DrawIndex` 需要设备特性 `shaderDrawParameters`(Vulkan 1.1 核心可选特性;lavapipe 支持【未核实】,Intel UHD 630 ANV 支持【未核实】)。在 `caps.rs` 加 `ShaderDrawParameters = 33`(`ALL` 数组长度 34),`init.rs` 探测 `VkPhysicalDeviceVulkan11Features::shader_draw_parameters` 并启用。
- **回退路径**(不支持 `shaderDrawParameters`):命令的 `firstInstance = draw 下标`,shader 用 `SV_InstanceID`(DXC 映射到 `InstanceIndex`,含 `firstInstance`【未核实:DXC 对 SV_InstanceID 的映射是 InstanceIndex 而非 InstanceId】)减去 0 取 draw 下标——只对非实例化 draw 可用(`instanceCount == 1`);实例化 draw(Nani、critters)本来就不走这条路。需要 `DrawIndirectFirstInstance`(已启用,caps 29)。
- `$Globals`(b13)里 `time`/`viewport`/`flag` 这些**每 pass 常量**移到 pass 参数块的 push 区(今天 `original_push_bytes` 允许任意大小,`bindless.rs:40-49`);`frame_constants`(b0 space1)不动。
- `bind_common` 对应的 shader 侧宏 `BIND_COMMON_*` 改为从 `dp` 读;`color_library` cbuffer(`nani_solid_vertex_color_bricks.raster.hlsl:504`)改为 `ByteAddressBuffer` 堆下标 `dp.color_library_slot`——cbuffer 不能按堆下标动态索引【未核实:现有 bindless 降级是否已把 cbuffer 降成 SSBO】。
- 受影响 shader 数:21 个列表涉及的材质 shader(bricks、grass×2、water_planes、stairs、pebble、billboards、reed、waterfall、cloth、roof 等),估计 30-40 个文件【未核实】。每个改动是机械的:删 `$Globals` 成员引用、加 `draw_params[draw_id]`。

#### (c) CPU 录制:按桶一次 `vkCmdDrawIndexedIndirectCount`

**桶键。** 一次 `vkCmdDrawIndexedIndirectCount` 的全部命令共享:绑定的管线、索引缓冲、顶点缓冲、动态状态(stencil ref、line width)。因此

```rust
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct BucketKey {
    pub shader: u64, pub variant: u8,               // → Arc<RasterEntry> / 管线
    pub state: u32, pub blend_bits: u32,            // RasterState 全部位(含 stencil test、polygon mode)
    pub stencil_ref: u8,                            // 动态状态,相同才能合桶
    pub vao: u64,                                   // 索引缓冲 + 顶点缓冲组(raster_host.rs:1865-1870 按 binding 绑定)
    pub index_type: u8,
}
pub struct DrawBucket { pub key: BucketKey, pub first: u32, pub len: u32, pub entry: Arc<RasterEntry>, pub pass_params_slot: u32 }
```

桶的形成:列表已按 `sort_key` 排序(材质连续);对排序后的条目做 run-length 分组,相邻键相同入同桶。桶数上界 = 唯一 `(材质, VAO)` 对数。**VAO 是限制项**:`DrawKind::IndexedIndirect` 的静态表(`INDEXED_INDIRECT_INDEX_COUNTS`)说明这些材质画的是固定网格(每材质 1-3 个网格),所以桶数 ≈ 材质数 × 小常数;而按实体各自持 VAO 的材质(例如实体自带网格)每个 VAO 一桶,退化为 O(实体)——桶数真实分布需要用 `lists={:?}` 日志(`production_renderer.rs:939`)统计【未核实】。退化桶仍比今天便宜:每桶只有 `bind_pipeline + bind_vertex/index + 1 indirect`,没有参数准备。

**录制序列(每个光栅 pass):**

```
begin_prepared(attachments) ; set_default_view_and_scissor ; set_stencil_ref(0)
pass_params = prepare_pass_params(ctx, frame)        // 一次:frame_globals 页 + 采样器页 + 纹理表 + DrawParamsSsbo 堆下标 + 本 pass push(viewport, time, cascade flag)
for bucket in frame.draw_params.lists[list].buckets:
    cmd_bind_pipeline(bucket.entry.pipeline.try_get()?)     // 相邻桶同管线则跳过
    pass_params.bind_pass(ctx, pipeline)                     // 一次 push:参数块下标
    if bucket.key.stencil_ref != cur: set_stencil_ref
    bind_index_buffer(vao.index) ; bind_vertex_buffer_by_binding_point(..)
    draw_indexed_indirect_count(params_buf, commands_offset + bucket.first*32, params_buf, counts_offset + bucket_index*4, bucket.len, 32)
end_render_pass
```

对比今天:每 draw 的 `bindless_parameters`(Vec、2-3 次 `image_view`、`prepare`、`finish` 的常量分配 + 堆租约)、`cmd_push_constants`、`record_draw` 全部消失;`try_get` 从每 draw 变每桶。

**访问声明。** 今天 `raster_host.rs:1455-1523` 逐 draw 声明 indirect/索引/顶点/外部绑定访问。C2 后每 pass 声明:`DrawParamsSsbo`(`0x02` indirect 读 + storage 读)、该列表材质纹理集合(来自桶的 `MaterialBindings` 并集)、各桶 VAO 的索引/顶点范围、纹理表读(不变)。访问数从 O(draw) 降到 O(桶 + 材质纹理),这也是 C1 形状键 `list_shape` 变小的原因。

**不支持 `DrawIndirectCount` 的设备**:沿用 `plain_indirect_needs_count_form`/`IndexedCountAdapter` 的现有降级(`rg/indirect_count.rs`),或直接 `vkCmdDrawIndexedIndirect(count = bucket.len)`——第一版没有 GPU 剔除,count 恒等于 len,`MultiDrawIndirect` 足够;`IndirectCount` 形式留给第二版 GPU 剔除。

#### (d) 与现有 Nani 剔除的关系

- Nani 子集**不改**:它们已经是一次 `IndexedIndirectCount`(`draw_ext.rs:124-200`),参数通过 `nani_instance_data`/`instance_indirection_buffer` 按 `SV_InstanceID` 取(shader 100-125 行)。C2 只把这些 draw 的"pass 级参数块"从每 draw 准备改为每桶准备——`nani_draw` 的 draw 仍各自一桶(每子集一个 shader/VAO)。
- 可复用的部分:`IndexedCountRequest`/`IndexedCountAdapter`(`indirect_count.rs:28-160`)的降级形式;`reset_indirect_args`(`render_bridge.rs:690`)清零 `DrawIndexedIndirectBuffer`/`AtomicCounterBuffer` 的机制,可扩展为清零各列表 count 缓冲(当第二版加入 GPU 剔除时)。
- 第二版 GPU 剔除(非本文范围):每列表一个 compute "CPU list cull"(视锥 + HZB,输入 `DrawParams.model` 与网格包围盒),输出 `count × bucket` 与紧凑后的命令。输入/输出布局与 (a) 已兼容(命令区与计数区就是为此预留的),与 Nani 的 `Draw -> dispatch args` 模式相同。

#### (e) 阴影级联 ×3 共用一张参数 SSBO

`shadow_pass.rs:27-47`:三个 pass 都遍历 `lists.list(SHADOW_LIST /* 0 */)`,差异仅 `depth.layer = cascade`、`flag: (cascade + 2) as u8`、`variant: 1`。因此:

- 三个 pass 共用 `DrawParamsTables.lists[0]`(同一 SSBO、同样的桶表)。
- `common_flag` 的 cascade 覆盖不写进 `DrawParams`,而写进**pass 参数块的 push 区**(`PassPush { viewport: [u32;2], time: f32, common_flag_override: u32 /* 0xFFFFFFFF = 用 dp.common_flag */ }`);shader `uint flag = push.flag_override != 0xFFFFFFFF ? push.flag_override : dp.common_flag;`。
- 桶表相同 ⇒ 三个 pass 的录制完全一样,只是 `set_stencil_ref`、管线 variant=1。CPU 从 3 × O(draw) 变 3 × O(桶)。
- C3 第三项进一步把三个 pass 合成一个 pass 的三个 rendering section。

#### (f) 迁移顺序与开关

1. **M-GD0 基建**:`gpu_driven/{draw_params, buckets, upload}.rs`;`caps.rs` 特性;`DrawListRevision` 计算接进 `build_draw_lists`;`TG_GPU_DRIVEN`(默认 1,`0` 走逐 draw)。此时 SSBO 写了但没人用——用 debug 断言比较"SSBO 中第 i 条参数"与"逐 draw 路径为第 i 个 draw 准备的参数块"字节相等(除堆下标外),作为 ABI 等价测试。
2. **M-GD1 Shadows**:列表 0;shader 只涉及投影到阴影图的材质 variant 1(bricks、grass、roof、billboards 等的阴影变体)。`ListRasterRecorder` 增加 `GpuDrivenRasterRecorder` 分支。像素门:`multiframe-frame*.png` 逐字节相等(阴影图是深度,主图差异直接可见)。
3. **M-GD2 G-buffer**:`main_pass.rs` 的 "G-buffer" 子 pass(`ItemMode::Stencil`,stencil_ref 来自 `item.stencil_ref` → 入桶键)与 "G-buffer OC/2"。像素门同上。
4. **M-GD3** 其余列表(velocity、emissive、Forward、transparency、UI、wireframe):同样机制,但 UI/wireframe 列表 draw 数少,收益小,最后做。
5. 删除逐 draw 路径前保留两个 release 周期。

### C2.3 改动清单

rhapsody:

- `device/caps.rs`:`Feature::ShaderDrawParameters`;`device/init.rs`:Vulkan 1.1 特性探测/启用。
- `rg/pass_api.rs`:已有 `draw_indexed_indirect_count`(588)、`bind_vertex_buffer_by_binding_point`(435);无需新增。
- `bindless_parameters.rs`:`PreparedBindlessParameters::with_push` 已支持复用租约只换 push(468);C2 的"每桶同参数块、不同 stencil"不需要新 API。
- 测试:`tests/bindless_shader_abi.rs` 增一个带 `DrawIndex` 的顶点 shader 降级用例,确认 `DrawParameters` capability 不被降级器剥掉。

shader(`shaders/`):

- 新 `common/draw_params.inc.hlsl`;各材质 raster shader 的 `$Globals` 成员引用改 `dp.*`;加 `draw_id` 输入。按 M-GD1/2 顺序分批。
- 保真度工具链(`tiny-glade-shader-fidelity-tools` 记忆:dxc/spirv-cross、`sweep_all`/`batch_lift`)重新跑受影响 shader 的编译与反射。

country-core:

- `render/pass_builder/draw_builder.rs`:`bind_common` 拆成 `common_params_for(item) -> CommonDrawParamsGpu`(纯函数)+ 旧路径;`DrawBuilder::finish` 产出 `DrawKind` 时额外导出 `IndirectCommand`(索引数、首索引、顶点偏移)。
- `draw_passes/draw_callbacks_{a,b}.rs`、`draw_shaders/*.rs`:每个材质回调拆出 `bind_static(&EntityDrawParams, &World) -> MaterialBindings { textures: SmallVec<[BindTexture;8]>, buffers: SmallVec<[SsboRef;4]>, kind: DrawKind }`;旧 `func(builder, params, world)` 改为 `bind_static` + `apply(builder, bindings)`,两条路径共用。这是 C2 工作量最大的一项(回调数 ≈ 50【未核实:`DISPATCH` 表行数】)。
- `draw_passes/draw_lists.rs`:`DrawLists` 增 `revisions: [u64; 21]`、`entry_hashes: [Vec<u64>; 21]`,在 `sort()` 后计算。

tiny-glade:

- 新 `gpu_driven/{mod, draw_params, buckets, upload, recorder}.rs`。
- `raster_host.rs`:`GpuDrivenRasterRecorder::record`(新),`record_list_pass` 分支;声明期访问集合改为按桶/材质。
- `render_host.rs`:`ShaderBindings::prepare_pass_params`(把今天 `prepare` 的"全局页 + 采样器页 + 纹理表"部分与"每 draw 资源"部分分离;前者已被 `FrameGlobals` 缓存,`frame_setup.rs:32-56`)。
- `production_renderer.rs`:`build_draw_lists` 后调用 `DrawParamsTables::sync(&lists, &mut tables, upload)`;`render_stages` 增 `draw_params_upload_bytes`、`buckets`。
- `game_loop.rs`:`TG_GPU_DRIVEN`。

### C2.4 验证

1. **ABI 等价**(M-GD0):debug 断言——逐 draw 路径为 draw i 生成的参数块(`PreparedBindlessParameters.bytes`,`bindless_parameters.rs:451-456`)与 `DrawParams[i]` 对应字段一致(model 矩阵位相等、纹理堆下标集合相等)。
2. **像素等价**:`retained_graph_matches_rebuilt_path` 的 GPU-driven 版 `gpu_driven_matches_per_draw_path`,同样 7 帧逐字节比较。深度/阴影是确定性的;G-buffer 的 stencil 写入顺序与今天相同(列表序 = 命令序)。
3. **桶统计**:30 帧日志 `buckets={list: n}`,与 `lists={:?}`(`production_renderer.rs:939`)的条目数对比,得出实际压缩比;若某列表桶数 ≈ 条目数,说明 VAO 不共享,记录为第二版"顶点拉取"的候选。
4. **上传量**:静止帧 `draw_params_upload_bytes == 0`;放置一面墙后只上传变化条目。
5. **性能门**:真机 release `record_submit_ms` 中 Shadows×3 + G-buffer 的份额降到接近 0;总 `record_submit_ms` < 2 ms。
6. **回退**:`TG_GPU_DRIVEN=0` 全帧像素相同;无 `shaderDrawParameters` 的设备自动走 `firstInstance` 回退并像素相同(lavapipe 可通过 `VK_*` 环境变量禁特性模拟【未核实】)。
7. **现有测试**:`crates/tiny-glade/tests/core_raster_draw.rs`、`original_raster_gpu.rs`、country-core `d_main_tests.rs`(它们测声明期 `log.submissions[].draws[]`,改为也接受桶化提交记录)。

### C2.5 风险

- **DXC `vk::builtin("DrawIndex")` 可用性**与 `shaderDrawParameters` 在目标集显上的支持面;两者都有回退(firstInstance),但回退只覆盖 `instanceCount == 1`。
- **cbuffer → SSBO**:`color_library`、`$Globals` 从 cbuffer 改为按堆下标索引的 StructuredBuffer,uniform 布局(std140)与 std430 对齐差异会改变 `float3` 成员读取;用 `float4` 对齐的显式布局避免。
- **材质回调读 World**:`bind_static` 必须是纯函数且能被 revision 哈希覆盖;漏掉某个 World 依赖 ⇒ SSBO 过期 ⇒ 错纹理。缓解:debug 下每 N 帧用旧路径重算全表并比较。
- **桶退化**:实体私有 VAO 的材质桶数 = 实体数;虽仍省参数准备,但命令数不减。统计后决定是否做网格图集/顶点拉取。
- **动态常量页压力转移**:逐 draw 参数块消失后 `DynamicConstants` 用量大降,但 `DrawParamsSsbo` 成为 GpuOnly 缓冲,依赖 A2 的异步上传;若 A2 未落地,`update_buffer_bytes` 的 `_wait` 会把收益抵消。
- **`DrawKind::MultiInstanced`、`NoVao`、egui 的 `unknown_bytes` push**(`raster_host.rs:1890-1927`,`2248-2250`):不进桶,保留逐 draw 录制(数量极少)。

---

## C3. pass 合并候选

来源:`reports/runtime/2026-10-19-main-loop-render-recovery/multiframe/pass-record-last-frame.txt`(debug/lavapipe/480×270 最后一帧,109 个不同名字,160 个 pass)。三组同形 pass 的声明位置与合并可行性如下。合并的判据:**合并后 pass 的访问集合 = 各成员访问集合的并集,且成员之间没有"写 → 读"的依赖**;满足则屏障语义不变(规划器对并集只在 pass 前插一次屏障,成员间无需屏障)。

### C3.1 Nani cull & bucket ×10(0.062 ms 单次,×10)

**声明。** `crates/country-core/src/systems/render_loop/compute_passes/gpu_generate_draw_lists.rs:255-316` `create_cull_and_bucket_pass`:`env.new_pass("Nani cull & bucket", source)` + 绑定序(`filtered_instance_allocator`、`filtered_instance_buffer`、`filtered_instance_meta`、`output_histogram` 四个并发写 `CULL_WRITE_GROUP`,`atlas_part_info`、`draw_list_info` 外部,`prev/next_visibility` 读/并发写,HZB 变体多 `hzb_tex` 读 + 维度 uniform)。记录位置 `:950-1010`:Main 一次(`owned_cull_batch` → `dispatch_with(cull_and_bucket_dispatch)`),然后对 `GpuWorldSubsetKind::all()` 中存在的子集各一次(`record_sparse_cull` 831 RoofTile/WoodenPlank;`record_external_gpu_cull` 861 Flower/Bush,这两个前面各有一个 "Draw -> dispatch args" pass,`FRAME_PASS_ORDER` 355-359;`is_special_cull` 729-740 的 Owned 子集 Plaster/Smoke/CurvedHalfTimber/StairsFloor)。调用点 `world_graph.rs:122`(第一轮,无 HZB)与 `:136-137`(第二轮,HZB)。10 = 每轮 5【未核实:具体是 Main + RoofTile + WoodenPlank + Flower + Bush,还是含 Owned 特殊子集;取决于该存档】。

**可否单 pass 多 dispatch。** 每个成员:写访问全部是同一 `CULL_WRITE_GROUP` 的并发写且 `preserve_storage_buffer_contents`(规划器已把它们当作可并发的同组写,成员之间**本来就没有屏障**——这正是 "concurrent" 组的含义);读访问是外部缓冲与 `prev_visibility`、HZB。差异只在 (i) 计算管线不同(`NaniCullAndBucketFamily::for_kind`,每子集一个 shader 变体)、(ii) 外部子集多一个 `instance_buffer` 读、(iii) Flower/Bush 用 `dispatch_indirect` 读 "Draw -> dispatch args" 写的参数。因此:

- **Main + RoofTile + WoodenPlank + Owned 特殊子集** → 合并为 1 个 pass "Nani cull & bucket (batch)":访问 = 并集(多出的 `instance_buffer` 读无害);回调内循环 `bind_pipeline(variant) → push → dispatch`。`ComputeRecorder` 需支持"多管线 dispatch 列表"(`RetainedDispatch::Multi(Vec<(Arc<ComputeEntry>, DispatchMode)>)`);compute 参数块每管线一个(ABI 不同)。
- **Flower、Bush**:它们读的 dispatch args 由紧前的 "Draw -> dispatch args" 写,这是真实的写→读依赖,不能并进上一个 pass。但两个 "Draw -> dispatch args" 之间无依赖、两个 cull 之间无依赖 ⇒ 合并成 "Draw -> dispatch args (Flower+Bush)" 1 个 + "Nani cull & bucket (external)" 1 个。
- 结果:每轮 5 cull + 2 args = 7 pass → 3 pass;两轮 14 → 6。屏障语义:并集访问与原序列在"pass 前一次 `vkCmdPipelineBarrier2`"模型下等价;唯一变化是 Owned 特殊子集的写现在和 Main 的写在同一 pass,而它们本来就是同组并发写(SPIR-V 原子分配器,注释 "Original SPIR-V atomics consume these original clear producers")。
- 顺带:C1 的 `nani_subsets` 键可删——子集增减只改 dispatch 列表,不改 pass。

### C3.2 Clear buffer ×6(0.017 ms 单次,×6)

**声明。** `crates/country-core/src/render/rg_helpers.rs:118-125`:

```rust
pub fn clear_rg_buffer(graph: &mut dyn RenderGraph, buffer: u32) {
    graph.add_pass("Clear buffer", buffer, ACCESS_TRANSFER_WRITE, Box::new(move |api| api.fill_buffer(buffer, None, 0)));
}
```

调用点(非测试):`gpu_generate_draw_lists.rs:939`(每轮清 prev/next 可见性位图之一,共 2)、`compute_passes/bin_lights.rs:150`(`Light indirection grid`)、`compute_passes/grass.rs:309`(`grasslet_bundle_ranges`)、`draw_passes/transparency_pass/surface_water.rs:50` → `surface_support.rs:216`(透明计数缓冲)、`systems/render_loop/mod.rs:143`(`RenderContext` 包装,调用方【未核实】)。第六处【未核实】(候选:`resources/world_raster.rs:669/672` `clear_buffer(0/1)` 经另一包装;或 mod.rs:143 的调用方)。

**可否合并。** 六个 clear 的目标互不相同,各自紧邻其首个消费者。合并成"帧首一个 Clear buffers pass"的条件是:每个目标在原 clear 点之前**没有任何写者**。对图内 `create_buffer` 的瞬态缓冲(grid、ranges、count)成立——它们的第一次访问就是 clear。对 Nani 可见性位图(`ssbo_library().get_rg_buffer::<NaniPrevVisibilityBitmaskSsbo>` 持久缓冲):第一轮清 Next,第二轮清 Prev【依 939-947 行的 `if hzb.is_some()` 分支】;第二轮之前第一轮的 cull 已经读过 Prev(`H_PREV_VISIBILITY` 读)——把第二轮的 clear 提前到帧首会让第一轮读到零位图,**语义改变**。所以:

- 可安全合并:bin_lights grid、grass ranges、transparency count、第一轮可见性 clear → 1 个 pass(帧首或第一个消费者之前任意点),`fill_buffer` ×4 在一个回调里。
- 不可提前:第二轮可见性 clear 必须留在第一轮 cull 之后;它可以与第二轮的 "Nani clear draw lists"/"Nani clear histograms"(同是 transfer 写,`FRAME_PASS_ORDER` 349-351)合并为第二轮的一个 clear pass。
- 代价:合并到帧首会把瞬态缓冲的生命周期起点提前,瞬态别名规划的峰值可能上升(§4.5);收益是 6 → 2 个 pass、屏障从 6 次 transfer-write 转换变 2 次批量。鉴于单次只有 0.017 ms(debug),此项优先级最低,主要价值是减少 pass 数对 §2.4 O(N·) 项的放大。

### C3.3 Shadows ×3(92.7 ms 单次,×3)

**声明。** `crates/tiny-glade/src/scene_raster.rs:117-119` `shadow_pass` → `crates/country-core/src/systems/render_loop/draw_passes/shadow_pass.rs:27-47`:

```rust
for cascade in 0..SHADOW_CASCADES {
    let atlas = ctx.host().rg_texture(SHADOW_ATLAS_PRESET);
    let spec = ListPass { name: SHADOW_PASS_NAME, colors: &[], depth: DepthSpec::Write { tex: atlas, layer: cascade, clear: Some(1.0) }, viewport: atlas, flag: true, mode: ItemMode::Shadow { flag: (cascade + 2) as u8 }, variant: 1, extra: None };
    run_list_pass(ctx, &spec, lists.list(SHADOW_LIST));
}
```

调用点 `world_graph.rs:123`。测试 `d_main_tests.rs:374` `shadow_pass_runs_three_cascades_with_layer_clear_and_variant_one` 断言 `names == ["Shadows","Shadows","Shadows"]`。

**可否单 pass。** 三个 pass 写同一张图集的不同 layer(深度附件,各自 `clear 1.0`),读的资源集合完全相同(同一列表、同样的材质纹理、同一 frame_constants)。成员之间无写→读依赖(各写各的 layer)。合并成一个 pass "Shadows" 的方式:

- 声明:深度写访问范围 `AccessRange::Texture { base_array_layer: 0, layer_count: 3 }`(今天是三条 `layer: i` 范围);其余访问取并集 = 任一成员的集合。
- 录制:回调里三次 `begin_prepared(attachments[layer=i]) … end_render_pass`。`RhRenderPassApi` 允许多次 `begin_dynamic_rendering`/`end_render_pass`(`pass_api.rs:390-405`);`attachments.lock().prepare_for_rendering(..)` 需对三个 layer 视图各准备一次(`raster_host.rs:1646-1657`)。
- 屏障语义:规划器对 `[0,3)` 范围一次转换到 `DEPTH_ATTACHMENT_OPTIMAL`,与今天三次 `[i,1)` 转换的最终状态一致;下游读阴影图集(`H_SHADOW_ATLAS` 采样)看到的是整个范围的一次转换。唯一差别:今天的 range_state 以 layer 为粒度跟踪,合并后以 `[0,3)` 为粒度——这正是 ANALYSIS §3 C3 所说"范围级精确性只在 debug 校验器里保留"。
- 更进一步(可选):层化渲染——一次 `vkCmdBeginRendering` 的 `layerCount = 3`,顶点 shader 以 `SV_RenderTargetArrayIndex` 选 layer,几何实例化 3 份(`instanceCount × 3` 或 multiview)。需要 `shaderOutputLayer`/`VK_KHR_multiview`,并改所有阴影 variant shader;GPU 侧省两次 render pass 切换,CPU 侧与三 section 方案相同。C2 落地后 CPU 已是 O(桶),此项只剩 GPU 收益,留给 C5 的真机数据决定。
- 与 C2 的叠加:三 section 共用同一 `DrawParamsSsbo` 与桶表(C2.2(e)),cascade flag 走每 section 的 push。

### C3.4 其它同形候选(来自同一记录,顺带列出)

| pass | 次数 | 声明位置 | 合并性 |
|---|---|---|---|
| `Nani clear draw lists` / `Nani clear histograms` / `Nani draw cmd histogram clear` / `Nani draw cmd subtotals clear` | 4+2+2+2 | `gpu_generate_draw_lists.rs` 各 `clear_*` 函数 | 同轮内全部是 transfer/compute 清零且无互相依赖 → 每轮 1 个 pass |
| `Draw -> dispatch args` ×4 | 4 | 同上 355-359 + grass | 见 C3.1 |
| `rev_blurN` ×7 / `blurN` ×6 | 13 | `world_postprocess` 的 blur 链 | **不可**合并:每级读上一级输出(真实链式依赖) |
| `UI` ×2 | 2 | egui + ui_pass | 不同目标/时机,不合并 |
| `Generate mips` ×2 | 2 | DoF 与 HZB | 各自是链式 mip 依赖,不合并 |

C3.1-C3.3 合计:160 → 约 145 个 pass(Nani 14→6,Clear 6→2,Shadows 3→1,Nani 其它 clear 10→2)。

### C3.5 改动清单与验证

- country-core `gpu_generate_draw_lists.rs`:`record_cull_batch(env, items: Vec<(NaniCullSource, CullBatch)>)` 取代逐子集 `env.record`;`compute_pass_builder.rs` 增 `DispatchMode::MultiPipeline(Vec<(ShaderSource, DispatchMode)>)`;tiny-glade `compute_frame.rs` 的 compute host 支持一 pass 多管线。
- country-core `shadow_pass.rs`:`ListPass.depth = DepthSpec::WriteLayers { tex, layers: 0..3, clear }`;`d_main_support.rs` `run_list_pass` 支持多 section;`raster_host.rs` 回调按 section 循环。
- `rg_helpers.rs`:`clear_rg_buffers(graph, &[u32])`。
- 测试:`d_main_tests.rs:374` 改为断言单个 "Shadows" 且 `depth.layer_count == 3`;`gpu_generate_draw_lists.rs:1749` 的 `base.builder.name` 断言改批量形式;像素等价测试(C1.8)覆盖三项合并;rhapsody `plan_check` 在 debug 下校验并集访问与原序列的最终 range state 一致(新增一个对比两张图最终 `RangeStateTable` 的单测)。
- 风险:Shadows 合并改变了 range_state 粒度,若某下游 pass 只读 layer 0 并依赖 layer 1/2 仍处于旧布局(不太可能,阴影图集整体采样),`plan_check` 会报;Nani 合并后一个子集的 shader 错误会让整批失败而非单个子集——错误信息里要带子集名。

---

## 附录 A. 引用位置索引

| 主题 | 位置 |
|---|---|
| 每帧建图入口 | `crates/tiny-glade/src/production_renderer.rs:404-508`(`build_full_world_stage`)、`:510-557`(`compile_full_world_graph`)、`:1409-1421`(`build_draw_lists`) |
| 帧桥 | `crates/tiny-glade/src/render_bridge.rs:254-294`(`build_frame`)、`:318-339`(`compile_built`/`execute_compiled`)、`:493-498`(`CoreGraphFrame`)、`:617-709`(`with_main_prelude`) |
| 会话 | `crates/rhapsody/src/rg/vulkan.rs:2412-2456`(`begin_frame`)、`:2551-2616`(`compile_frame`)、`:2749-2802`(`temporal_texture`)、`:2803-2834`(`request_named`)、`:2836-2849`(`reset_temporal`)、`:2916-2999`(`compile_built`)、`:3001-3066`(`execute_prepared`) |
| 图与计划 | `crates/rhapsody/src/rg/graph.rs:197-198`(回调类型)、`:237-260`(`RenderGraph`/`BuiltGraph`)、`:840-872`(`FrameInstance`)、`:874-920`(`compile_graph`/`instantiate_plan`)、`:922-1035`(`instantiate_validated_plan`/`instantiate_with_storage`);`crates/rhapsody/src/rg/plan.rs:23-28, 73-95, 140-205, 211-299, 308-319`;`crates/rhapsody/src/rg/pass_builder.rs:396-426`;`crates/rhapsody/src/rg/executor.rs:492-502`(`PassContext`)、`:515-523`(`history_valid`)、`:841-920`(`compile_graph` 与计划缓存)、`:949-1073`(`execute_prepared_*`) |
| temporal | `crates/rhapsody/src/rg/temporal.rs:11-130`;`crates/tiny-glade/src/world_lighting.rs:83-102, 157-168, 178, 220`;`crates/tiny-glade/src/world_graph.rs:80-86, 154` |
| 世界图阶段 | `crates/tiny-glade/src/world_graph.rs:91-159`;`world_geometry.rs:54-104`;`world_postprocess.rs:61-95, 95-219, 307-340, 1226-1334`;`world_surface.rs:36-71`;`scene_raster.rs:61-157`;`scene_compute.rs:54-80, 201-227` |
| 光栅宿主 | `crates/tiny-glade/src/raster_host.rs:718-734`(`PreparedDraw`)、`:946`、`:1403-1437`(纹理表读)、`:1455-1523`(逐 draw 访问)、`:1551-1725`(回调)、`:1857-1950`(`record_draw`)、`:2144-2269`(`bindless_parameters`);`render_host.rs:832-843`(`RasterEntry`)、`:1291-1331`(`ShaderBindings::prepare`)、`:1713-1718` |
| bindless ABI | `crates/rhapsody/src/bindless_parameters.rs:223-231, 411-470, 493-552`;`crates/rhapsody/src/bindless.rs:1387-1445`;`crates/rhapsody/crates/shader-compiler/src/bindless.rs:21-35, 40-104, 282-315` |
| draw list / builder | `crates/country-core/src/systems/render_loop/draw_passes/draw_lists.rs:47, 113-142, 185-200, 320-346`;`draw_list.rs:118-136`;`d_main_support.rs:151-176, 191-211, 258-308`;`shadow_pass.rs:17-47`;`main_pass.rs:1-60, 100-245`;`render/pass_builder/draw_builder.rs:85-90, 236-244, 304-307, 340-376, 956-999, 1062-1090`;`draw_stream.rs:121-156, 180-209` |
| Nani | `crates/country-core/src/systems/render_loop/compute_passes/gpu_generate_draw_lists.rs:140-150, 229-239, 255-316, 349-363, 729-740, 768-900, 906-1010`;`render/nani/draw_ext.rs:124-200`;`crates/rhapsody/src/rg/indirect_count.rs:28-160` |
| 设备能力 | `crates/rhapsody/src/device/caps.rs:60-135`;`device/init.rs:550-556, 706-708` |
| shader | `shaders/nani_solid_vertex_color_bricks.raster.hlsl:57, 88, 100-101, 125, 504`;`shaders/billboard_plant.raster.hlsl:44, 206` |
| Clear buffer | `crates/country-core/src/render/rg_helpers.rs:118-125`;`bin_lights.rs:150`;`grass.rs:309`;`transparency_pass/surface_water.rs:50`、`surface_support.rs:216`;`render_loop/mod.rs:143` |
| 测试 | `crates/tiny-glade/tests/original_gameplay_chain.rs:755-960`(`shipped_session_steady_state_multi_frame`);`crates/country-core/src/systems/render_loop/draw_passes/d_main_tests.rs:233, 270, 343, 374, 451`;`reports/runtime/2026-10-19-main-loop-render-recovery/MULTIFRAME-BASELINE-2026-10-20.md:3-5, 53` |

## 附录 B. 未核实项汇总

1. C1.3 #6:除 `GladeType` 经 `select_water_surface` 外是否还有冬季专属 pass。
2. C1.3 #8:截图 capture pass 固定声明后读访问与导出布局的关系。
3. C1.8:多帧测试的 `Time` 步进是否确定;lavapipe compute 调度的确定性。
4. C2.2(b):DXC `[[vk::builtin("DrawIndex")]]`、`SV_InstanceID → InstanceIndex` 映射;lavapipe/ANV 对 `shaderDrawParameters` 的支持;现有 bindless 降级是否已把 cbuffer 转为 SSBO。
5. C2.2(c):各列表唯一 (材质, VAO) 对数——决定桶压缩比。
6. C2.3:材质回调数量(`DISPATCH` 表行数);受影响 shader 文件数。
7. C2.4 #6:lavapipe 上模拟缺特性的方法。
8. C2.5:critters 是否走独立 instanced 路径。
9. C3.1:该存档中 10 个 cull pass 的具体子集组成。
10. C3.2:第六个 Clear buffer 的调用点。
