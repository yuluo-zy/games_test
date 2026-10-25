# country-core 逐模块索引

本索引配合 [主 README](../../README.md) 阅读。按当前源码目录列出非测试 Rust 文件；目录职责由人工整理，文件“用途线索”摘自源码注释，类型/接口从源码提取。**源码注释中的“已实现”“全证”等是原注释的范围声明，本索引不据此重新授予验收。**

本次列出 578 个文件、60 个源码目录。源码文件计数不是原作文件恢复率：包含辅助/兼容文件，也包含仅合同或未完成函数。测试文件不作业务模块计数；内联模块另列。此处按磁盘源码列举，不是编译器可达性报告；类型/接口为词法导航，是否被 cfg 启用或正式调用须查父模块和函数体。

## 目录导航

- [asset_libraries](#asset-libraries)
- [components](#components)
- [crate 根](#crate-root)
- [geometry](#geometry)
- [input](#input)
- [systems/wall/construction](#systemswallconstruction)
- [resources/critters](#resources-critters)
- [render](#render)
- [render/nani](#render-nani)
- [render/pass_builder](#render-pass-builder)
- [resources](#resources)
- [resources/critters](#resources-critters)
- [resources/critters/sheep](#resources-critters-sheep)
- [resources/history](#resources-history)
- [resources/ivy](#resources-ivy)
- [resources/render](#resources-render)
- [resources/roofs](#resources-roofs)
- [resources/roofs/geometry](#resources-roofs-geometry)
- [resources/stairs](#resources-stairs)
- [resources/walls](#resources-walls)
- [startup](#startup)
- [systems/audio](#systems-audio)
- [systems/audio/music](#systems-audio-music)
- [systems/backend](#systems-backend)
- [systems/backend/input](#systems-backend-input)
- [systems](#systems)
- [systems/camera_rig](#systems-camera-rig)
- [systems/collision](#systems-collision)
- [systems/debug](#systems-debug)
- [systems/garden](#systems-garden)
- [systems/history](#systems-history)
- [systems/ivy](#systems-ivy)
- [systems/mode_manager](#systems-mode-manager)
- [systems/render_loop/compute_passes](#systems-render-loop-compute-passes)
- [systems/render_loop/draw_passes](#systems-render-loop-draw-passes)
- [systems/render_loop/draw_passes/draw_shaders](#systems-render-loop-draw-passes-draw-shaders)
- [systems/render_loop/draw_passes/transparency_pass](#systems-render-loop-draw-passes-transparency-pass)
- [systems/render_loop](#systems-render-loop)
- [systems/render_loop/resources](#systems-render-loop-resources)
- [systems/render_loop/session_selection](#systems-render-loop-session-selection)
- [systems/save](#systems-save)
- [systems/steam](#systems-steam)
- [systems/terrain](#systems-terrain)
- [systems/ui](#systems-ui)
- [systems/ui/resources](#systems-ui-resources)
- [systems/ui/signifiers](#systems-ui-signifiers)
- [systems/ui_clutter](#systems-ui-clutter)
- [systems/wall](#systems-wall)
- [systems/wall/eraser](#systems-wall-eraser)
- [systems/wall/shapes](#systems-wall-shapes)
- [systems/wall/shapes/circle](#systems-wall-shapes-circle)
- [systems/wall/shapes/circle/signifiers](#systems-wall-shapes-circle-signifiers)
- [systems/wall/shapes/rectangle](#systems-wall-shapes-rectangle)
- [systems/wall/shapes/rectangle/ui_modes](#systems-wall-shapes-rectangle-ui-modes)
- [systems/wall/shapes/utils](#systems-wall-shapes-utils)
- [systems/wall/snapping](#systems-wall-snapping)
- [systems/wall/valid_enclosure](#systems-wall-valid-enclosure)
- [systems/wall/wall_state](#systems-wall-wall-state)
- [utils](#utils)
- [utils/components](#utils-components)

<a id="asset-libraries"></a>


## asset_libraries

业务流程、所有权与调度边界见 [模块 README](../../src/asset_libraries/README.md)，本轮问题及验证见 [代码审查](../../src/asset_libraries/REVIEW.md)。

- **功能**：资产注册、查找和热重载。
- **设计**：名字/类型键→稳定句柄→资源；文件监听与资产持有分开。
- **业务**：载入网格/纹理/shader/颜色/摆放/音频触发数据，再供启动和渲染读取。

| 源码模块 | 用途线索（来自源码） | 主要类型 / 接口 |
|---|---|---|
| [audio_trigger_library](../../src/asset_libraries/audio_trigger_library.rs) | `asset_libraries/audio_trigger_library.rs` —— 音频触发盒库（A3 逐函数核对版）。 | `AudioTriggerHandle`、`AudioTriggerKind`、`AudioTriggerCollection`、`AudioTriggerLibrary`、`boxes`、`get_triggers`、`add_trigger`、`update_trigger_shape` 等 |
| [color_library](../../src/asset_libraries/color_library.rs) | `asset_libraries/color_library.rs` —— 调色板 SSBO `ColorLibrarySsbo` 及其 JSON 装载（A3 逐函数核对版）。 | `ColorLibrarySsbo`、`ColorFromJson`、`GladeTheme`、`slot_index`、`deserialize_from`、`deserialize_from_path`、`dir_name`、`theme_colors_path` 等 |
| [configwatch](../../src/asset_libraries/configwatch.rs) | `ConfigWatch` —— 配置文件热重载监视器（layer A3）。 | `Pending`、`ConfigWatch`、`on_event`、`new`、`watch`、`take_pending` |
| [mesh_library](../../src/asset_libraries/mesh_library.rs) | `AssetMeshLibrary` — 网格注册表（layer A3，逐函数对照原版机器码重写）。 | `MeshAsset`、`AssetMeshLibrary`、`new`、`transient_meshes`、`add`、`get`、`get_mut`、`get_by_name` 等 |
| [meshwatch](../../src/asset_libraries/meshwatch.rs) | `MeshWatch` — 网格源文件热重载监视器（layer A3，逐函数对照原版机器码重写）。 | `MeshWatch`、`new`、`watch`、`on_debounced_event`、`changed_paths`、`is_watching` |
| [mod（本目录入口）](../../src/asset_libraries/mod.rs) | Global handle-id counter (O: `AssetMeshLibrary::add` / `AssetShaderLibrary::add_*` increment `HANDLE_COUNT`). | `HandleId`、`Handle`、`Asset`、`new`、`from_raw`、`raw`、`is_valid`、`id` 等 |
| [placement_data_library](../../src/asset_libraries/placement_data_library.rs) | `asset_libraries/placement_data_library.rs` —— 放置数据（prefab 摆放记录）注册表（A3 逐函数核对版）。 | `AssetPlacementDataLibrary`、`new`、`load` |
| [shader_library](../../src/asset_libraries/shader_library.rs) | `AssetShaderLibrary` —— shader 程序注册表（layer A3）。 | `ComputeShaderSourceHash`、`NoHashHasher`、`NoHashBuildHasher`、`AssetShaderLibrary`、`from_hash`、`new`、`add_raster`、`add_compute` 等 |
| [ssbo_library](../../src/asset_libraries/ssbo_library.rs) | `AssetSsboLibrary` — 类型化存储缓冲（SSBO）注册表（layer A3）。 | `Ssbo`、`SsboType`、`SsboId`、`BufferState`、`InstanceInfo`、`AssetSsboLibrary`、`of`、`new` 等 |
| [texture_library](../../src/asset_libraries/texture_library.rs) | `AssetTextureLibrary` — 纹理注册表 + bindless 描述符表 + 渲染图“预设纹理”（layer A3，已逐函数对照机器码）。 | `NoHashHasher`、`NoHashBuildHasher`、`AssetIndex`、`NamedTexture`、`DescriptorIndex`、`ImportedTexture`、`PresetTextureState`、`DescriptorMap` 等 |
| [texture_watch](../../src/asset_libraries/texture_watch.rs) | `TextureWatch` — 纹理文件热重载监视器（layer A3，已逐函数对照机器码；证据见 `evidence/country-core-audit/A3_texture.md`）。 | `DebouncedEvent`、`HotwatchError`、`GenericHotwatch`、`ChangedSet`、`TextureWatch`、`new_with_custom_delay`、`watch`、`fire` 等 |
| [transient_mesh](../../src/asset_libraries/transient_mesh.rs) | `transient_mesh.rs` — `TransientMesh` 组件与"延迟删除网格"链（B `components::transient_mesh` + `backend::delete_dropped_transient_meshes`）。 | `TransientMeshRegistry`、`TransientMesh`、`new_with_registry`、`mesh_handle`、`drain_meshes_to_delete`、`delete_dropped_transient_meshes` |
| [vao_library](../../src/asset_libraries/vao_library.rs) | `AssetVAOLibrary` — 网格 → VAO 注册表（layer A3，逐函数对照原版机器码重写）。 | `VaoEntry`、`AssetVAOLibrary`、`new`、`has_vao`、`get`、`add`、`rebuild_vao`、`remove` |

<a id="components"></a>


## components

- **功能**：实体的绘制、树和注册数据。
- **设计**：组件附着于 Entity；注册/克隆策略与系统行为分开。
- **业务**：实例生成时附加组件，更新和回收由消费者执行。

| 源码模块 | 用途线索（来自源码） | 主要类型 / 接口 |
|---|---|---|
| [drawable](../../src/components/drawable.rs) | `drawable.rs` — nani 可绘制组件族里已恢复的部分（B `components::drawable`）。 | `AtlasPartIndex`、`NaniInstanceMeshDesc`、`NaniBundle`、`DrawableMeshBundle`、`tag` |
| [mod（本目录入口）](../../src/components/mod.rs) | Observed zero-sized rendering component tags (B components/mod.rs). | `SemiTransparent`、`FlatLighting`、`DashedLineLength`、`BackgroundTerrain`、`DontReceiveContactShadows`、`OverwriteRoughness` |
| [registry](../../src/components/registry.rs) | `registry.rs` — 游戏自有组件的注册表（B `components::*`，57 个组件的 `register_required_components` 与 `clone_behavior`）。 | `CloneBehavior`、`ComponentEntry`、`required_components` |
| [trees](../../src/components/trees.rs) | trees.rs — 树组件族（组 A restorer 接线；B skeleton `components/` 的树/预制体 组件，`components/registry.rs` 已登记名称串）。 | `TreeMarkedForDeletion`、`TreeAnimation`、`TreeMoveBend`、`TreeComponent`、`ConvertedPreplacedTree`、`Ui2dExtent`、`Ui2dImage`、`UiDrawOrder` |

<a id="crate-root"></a>


## crate 根

- **功能**：crate 导出、配置/档案、元命令与依赖边界。
- **设计**：顶层模块按 lib.rs 导出；配置、档案和跨系统队列分离。
- **业务**：初始化配置并连接各业务域；Gap 和兼容类型只标记边界。

| 源码模块 | 用途线索（来自源码） | 主要类型 / 接口 |
|---|---|---|
| [frozen_map](../../src/frozen_map.rs) | `frozen::FrozenMap` 的调用面占位（外部库 `frozen`，离线缓存缺失，见 `Cargo.toml` 注释）。 | `FrozenMap`、`new`、`get`、`get_mut`、`remove`、`insert`、`len`、`is_empty` 等 |
| [game_config](../../src/game_config.rs) | game_config.rs — `country_core::game_config` 游戏配置资源。 | `GameConfig`、`DevSettings`、`ScreenshotFormat`、`GameSettings`、`upgrade`、`live_link`、`developer_hotkeys_enabled` |
| [game_profile](../../src/game_profile.rs) | B `game_profile.rs` — 玩家档案（0x208B=520B；ron）。 | `KeyBinding`、`GameKeyBindings`、`TutorialCooldowns`、`GameProfile`、`default_cooldowns`、`increment_new_glade_count`、`mark_tutorial_op_complete`、`reset_hint_flags` 等 |
| [lib（crate 导出）](../../src/lib.rs) | Game-facing rendering declarations and recovered CPU domain rules. Modules follow the original B functional source tree. GPU recording is supplied through explicit backend traits; see README.md for unresolved seams. | `Gap` |
| [meta_commander](../../src/meta_commander.rs) | B `meta_commander.rs` — 元命令队列。 | `MetaCommand`、`MetaCommandFence`、`MetaCommander`、`CommanderOutcome`、`new_pair_blocked`、`unblock`、`is_blocked`、`as_payload` 等 |
| [name_hash](../../src/name_hash.rs) | `utils::name_hash::NameHash`（本地 `utils` crate，O 0x140c9cd80，46 B）。 | `NameHash`、`compute` |
| [reused](../../src/reused.rs) | Original hotwatch fork adapter; event mapping is game-facing glue. | `GenericHotwatch`、`new_with_custom_delay`、`watch` |
| [rhapsody_contract](../../src/rhapsody_contract.rs) | rhapsody 设备的游戏侧接线缝（D 组接入，WIRING 契约 `D-rhapsody/rhapsody-WIRING.md` §1/§3.1）。 | `GraphicsDevice`、`DescriptorSet`、`ManagedHandle`、`Handle`、`ReadOnlyHandle`、`ResourceHandle`、`Extent2D`、`RenderPassDesc` 等 |
| [rhapsody_types](../../src/rhapsody_types.rs) | 外部库 `rhapsody::raw::{texture,shader}` / `shader_pipeline` 在本 crate 里的**唯一**占位类型。 | `Error`、`TextureDesc`、`TextureViewDesc`、`Texture`、`CompiledShaderPipeline`、`RasterPipelineDesc`、`RasterPipelineKey`、`RasterPipeline` 等 |
| [utils_contract](../../src/utils_contract.rs) | Existing game-owned utility types, reused by the recovered render layer. | 内部实现 / 模块装配（见源码） |

<a id="geometry"></a>


## geometry

- **功能**：基础网格和实例墙编码。
- **设计**：顶点属性/索引生成与 GPU 上传边界分离。
- **业务**：将尺寸和砖源参数变成可绘制几何。

| 源码模块 | 用途线索（来自源码） | 主要类型 / 接口 |
|---|---|---|
| [cube](../../src/geometry/cube.rs) | `cube.rs` — 轴对齐盒网格（B `geometry::cube`，`From&lt;Box&gt; for Mesh`，0x140a05530，3643 B）。 | `CubeBox`、`cube_mesh` |
| [instanced_wall](../../src/geometry/instanced_wall.rs) | `instanced_wall.rs` — 砖实例墙的几何载体与上传（B `geometry::instanced_wall`）。 | `BrickSourceId`、`InstancedWall`、`BrickTransformSsbo`、`from_color_id`、`from_wall_id`、`with_transient_stair_edge_id`、`new`、`update` 等 |
| [mod（本目录入口）](../../src/geometry/mod.rs) | `country_core::geometry` — 程序化几何与砖实例墙载体（B `geometry::*`）。 | 内部实现 / 模块装配（见源码） |
| [plane](../../src/geometry/plane.rs) | `plane.rs` — XZ 平面四边形网格（B `geometry::plane`，`From&lt;Plane&gt; for Mesh`，0x140a06370，1934 B）。 | `plane_mesh` |
| [sphere](../../src/geometry/sphere.rs) | `sphere.rs` — 二十面体球网格（B `geometry::sphere`，`TryFrom&lt;Sphere&gt; for Mesh`，0x140a06b00）。 | `Sphere`、`SphereError`、`icosphere_vertex_count`、`sphere_uv`、`new`、`to_mesh` |

<a id="input"></a>


## input

- **功能**：动作集合、录制帧与复现状态。
- **设计**：平台输入转换为稳定动作模型；帧与历史同步点关联。
- **业务**：键鼠/Steam/回放进入同一工具输入面。

| 源码模块 | 用途线索（来自源码） | 主要类型 / 接口 |
|---|---|---|
| [action_set](../../src/input/action_set.rs) | `action_set.rs` — 动作集（B `input::action_set`）。 | `DigitalActionState`、`ActionSetAction`、`MainActionSet`、`MainActionSetSerde`、`ActionSetValues`、`ActionSetStates`、`pressed`、`released` 等 |
| [input_stream](../../src/input/input_stream.rs) | `input_stream.rs` — 输入录制环（B `input::input_stream`）。 | `InputTranslationState`、`InputStreamSyncPoint`、`InputStream`、`toolbar_current`、`toolbar_modes_header`、`translation_head`、`camera_params`、`from_sync_resources` 等 |
| [mod（本目录入口）](../../src/input/mod.rs) | 顶层 `input` 模块（B `input::{action_set, input_stream, mod, panic_repro_state}`）。 | `ActiveInputMode`、`ActiveInputModeTracker`、`InputRecordReplayMode`、`is_controller`、`new_default_controller`、`new_default_mouse_keyboard`、`mode`、`from_niche` 等 |
| [panic_repro_state](../../src/input/panic_repro_state.rs) | `panic_repro_state.rs` — `PanicReproState::is_inactive`（B 0x140833230：`cmp byte [rcx], 0; sete al`）。 | `PanicReproState`、`is_inactive` |

<a id="recovered-building"></a>


## render

- **功能**：GPU 数据组织和游戏侧渲染接口。
- **设计**：类型化缓冲、mesh、shader、纹理、常量与后端接口分开。
- **业务**：将世界更新转为上传、绑定和绘制描述。

| 源码模块 | 用途线索（来自源码） | 主要类型 / 接口 |
|---|---|---|
| [atmosphere](../../src/render/atmosphere.rs) | `SunSkyColors` — sun/sky color palette (WP4-owned). | `SunSkyColors`、`lerp` |
| [device](../../src/render/device.rs) | `render/device.rs` — `GraphicsDeviceResource`（B `country_core::render::GraphicsDeviceResource`）。 | `GraphicsDeviceResource`、`new`、`device` |
| [directional_light](../../src/render/directional_light.rs) | `DirectionalLight` — sun light with a look-at view basis (layer A1). | `DirectionalLight`、`look_at`、`set_direction_towards_light` |
| [dirty_index_range](../../src/render/dirty_index_range.rs) | `DirtyIndexRange` — 原子闭区间脏窗口（layer A1）。 | `DirtyIndexRange`、`empty`、`add`、`is_empty`、`to_range`、`clear` |
| [execution_host](../../src/render/execution_host.rs) | rhapsody 执行链宿主（D 组接入，WIRING §2.7 / §5.2 步骤 4）。 | `RhapsodyExecutionHost`、`new`、`begin_frame`、`execute_graph`、`complete_primary`、`wait_idle_and_recycle_resources`、`new_render_graph`、`pass_api_from_context` 等 |
| [frame_constants](../../src/render/frame_constants.rs) | `FrameConstants` — per-frame GPU constant block (layer A1). | `RenderViewIndex`、`FrameConstants`、`is_shadow_view`、`sun_cascade`、`should_reset_temporal`、`view`、`main_view`、`sun_view` |
| [indirect](../../src/render/indirect.rs) | indirect.rs — Vulkan indirect command wrappers. O/I/U tags per `contracts/indirect.md`. All methods inline (0 standalone procs). | `DrawIndexedIndirectCommand`、`DrawIndirectCommand`、`DispatchIndirectCommand`、`empty`、`w_vertex_count`、`w_instance_count` |
| [load_texture](../../src/render/load_texture.rs) | `load_compiled_texture_image` — 加载预编译的纹理文件（layer A1）。 | `TextureDevice`、`CompiledImage`、`AlignedFile`、`parse_compiled_image`、`compiled_texture_path`、`read`、`bytes`、`load_compiled_texture_image` |
| [mesh](../../src/render/mesh.rs) | `Mesh` / `VertexAttributeValues` — CPU mesh container (layer A1). | `VertexAttributeValues`、`SourceFile`、`CollisionBbx`、`Mesh`、`array_length`、`stride`、`size_in_bytes`、`append` 等 |
| [mesh_atlas](../../src/render/mesh_atlas.rs) | `render/mesh_atlas.rs` -- 仅为兼容旧路径保留。 | 内部实现 / 模块装配（见源码） |
| [mod（本目录入口）](../../src/render/mod.rs) | 第 0 层文件的挂载点（相对路径指向真实源文件，不拷贝）。 | 内部实现 / 模块装配（见源码） |
| [raster_single](../../src/render/raster_single.rs) | raster_single.rs — `raster_single_object`（单颜色目标 / 全屏三角形的光栅 pass 助手）。 | `raster_single_object` |
| [render_config](../../src/render/render_config.rs) | 画质 / 显示设置的枚举与小函数（layer A1）。 | `FullscreenConfig`、`UpscalingPreset`、`ImageQualityPreset`、`UpscalingMode`、`MonitorMatch`、`VideoMode`、`VideoSettingsVersion`、`DisplayConfig` 等 |
| [rg_helpers](../../src/render/rg_helpers.rs) | 渲染图（render graph）的三个小 pass 辅助（layer A4 里被 A1 调用的部分）。 | `ResourceLease`、`RenderPassApi`、`StagingAlloc`、`StagingCopy`、`RenderFn`、`UploadContentVersion`、`RenderGraph`、`new` 等 |
| [shader](../../src/render/shader.rs) | Shader 程序的加载 / 缓存层（layer A1，游戏自有部分）。 | `ShaderBackend`、`ComputeShaderProgram`、`RasterShaderProgramVariant`、`RasterShaderProgram`、`new`、`try_new`、`cached`、`get_or_generate` 等 |
| [shadowed_buffer](../../src/render/shadowed_buffer.rs) | `ShadowedBuffer&lt;T&gt;` — CPU 影子数组 + 脏区间上传（layer A1）。 | `ShadowedBuffer`、`new`、`get_mut`、`copy`、`write`、`gpu_flush` |
| [sparse_instance_buffer](../../src/render/sparse_instance_buffer.rs) | `SparseInstanceBuffer&lt;T&gt;` / `SparseInstanceBufferWriter&lt;T&gt;` / `SparseInstanceBufferAllocation&lt;T&gt;`（layer A2）。 | `SparseSsbo`、`ChunkId`、`DrawRange`、`Allocation`、`AllocationHandle`、`SparseInstanceBufferAllocation`、`MeshAtlasAllocation`、`SparseInstanceBuffer` 等 |
| [ssbo](../../src/render/ssbo.rs) | `render::ssbo` — typed GPU buffer wrappers (layer A1). | `Buffer`、`BufferDesc`、`GraphicsDevice`、`ResourceHandle`、`StorageElement`、`InstanceCounted`、`BufferWriteError`、`TypedStorageBuffer` 等 |
| [utils](../../src/render/utils.rs) | `render/utils.rs` — `calculate_hash` (referenced by wp1 `shader_uri` / `draw_stream` as `crate::render::utils::calculate_hash`). | `calculate_hash` |
| [vao](../../src/render/vao.rs) | `Vao` / `VaoBinding` / `build_vao`（layer A1，0x140909690，463 B）。 | `VaoBinding`、`Vao`、`build_vao` |
| [view_constants](../../src/render/view_constants.rs) | `ViewConstants` / `ViewportConstantBuilder` — per-view GPU constants (layer A1). | `ViewConstants`、`ViewportConstantBuilder`、`set_pixel_offset`、`build` |

<a id="render-nani"></a>


## render/nani

- **功能**：实例世界、实例载荷、图集和剔除绘制。
- **设计**：稳定逻辑 ID 映射稠密索引；不同实例类型分别打包。
- **业务**：新增/修改/移除实例→更新缓冲→剔除→间接绘制。

| 源码模块 | 用途线索（来自源码） | 主要类型 / 接口 |
|---|---|---|
| [buffers](../../src/render/nani/buffers.rs) | buffers.rs — nani 绘制参数与 SSBO（B 阶段按机器码重写，n_world）。 | `NaniDrawListParams`、`DrawListParams`、`NaniDrawListOffsetDefaults`、`nani_calculate_draw_list_params`、`calculate_with_flags`、`compute` |
| [cull_shaders](../../src/render/nani/cull_shaders.rs) | cull_shaders.rs — nani 剔除/分桶 compute pass 结构体（B 阶段按机器码重写）。 | `NaniCullAndBucketFamily`、`NaniCullShader`、`path`、`source_uri`、`for_kind` |
| [draw_ext](../../src/render/nani/draw_ext.rs) | draw_ext.rs — DrawBuilder 的 nani 扩展（B 阶段按机器码重写）。 | `NaniIndirectMode`、`NaniDrawPlan`、`NaniDrawExt`、`GpuWorldResource`、`uses_indirect_count`、`nani_list_index`、`nani_draw_plan` |
| [gpu_world](../../src/render/nani/gpu_world.rs) | gpu_world.rs — Nani GPU-driven 实例世界的 CPU 侧管理（B 阶段按机器码重写，n_world）。 | `InstanceDataMarker`、`InstanceHandle`、`InstanceKey`、`GpuWorldSubsetKind`、`GpuWorldSubsetDesc`、`ExternalGpuWorldSubset`、`GpuWorldSubset`、`InstanceCullData` 等 |
| [instance_data](../../src/render/nani/instance_data.rs) | instance_data.rs — 每类 nani 实例的 GPU 布局类型（B 阶段按机器码重写，n_world）。 | `CurvedHalfTimberInstanceData`、`CurvedHalfTimberCaps`、`NaniUberInstanceFlags`、`WindowGlassFlags`、`marker`、`new_curved_xy`、`new_curved_xz`、`new_straight` 等 |
| [instance_updates](../../src/render/nani/instance_updates.rs) | `instance_updates.rs` — nani 各 `nani_update_*` 系统的**实例载荷构建函数**（B 阶段按机器码恢复）。 | `InstanceUpdate`、`NaniCompactKey`、`FlowerRaster`、`RoofWords`、`FloorWords`、`TangentAtU0`、`LanternParamsView`、`PlasterWallInputs` 等 |
| [mesh_atlas](../../src/render/nani/mesh_atlas.rs) | `MeshAtlas` — 把多个 `Mesh` 合并进一张大网格（layer A2）。 | `MeshAtlasPart`、`ComputedLayout`、`MeshAtlas`、`NaniMeshAtlasPartInfo`、`new`、`len`、`is_empty`、`part_index` 等 |
| [mod（本目录入口）](../../src/render/nani/mod.rs) | render/nani.rs — nani GPU-driven 渲染模块根（WP3）。 | `warn_once_in`、`warn_once_key` |
| [systems](../../src/render/nani/systems.rs) | `render::nani::systems` — nani 的 ECS 系统层：`nani_maintain_instances`、20 个 `nani_update_*`、 `nani_flush_instance_updates`（B 构建 0x14206f300 / 0x142070ac0..0x14207fcc0 / 0x142080640）。 | `NaniMeshSelector`、`NaniInstanceDesc`、`NaniActiveInstanceCollection`、`NewInstanceRow`、`ChangedInstanceRow`、`TransformRow`、`SolidVertexColorRow`、`ColorIdRow` 等 |
| [trimesh](../../src/render/nani/trimesh.rs) | trimesh.rs — `Trimesh` / `TrimeshBuilder`（B 阶段按机器码重写，n_world）。 | `BuilderChunk`、`TrimeshChunk`、`Trimesh`、`TrimeshBuilder`、`index_count`、`builder`、`chunks`、`input_max_index` 等 |

<a id="render-pass-builder"></a>
| [nani_solid_vertex_color](../../src/render/nani/solid_vertex_color.rs) | SolidVertexColor's private upload schema, recovered from the original B writer call. Parallel implementation: build/native/World verification deferred. Quaternion/matrix operations belong to the pinned glam dependency. | `SolidVertexColorCpuProjection`、`project_solid_vertex_color` |


## render/pass_builder

- **功能**：渲染 pass 的声明和绘制流构建。
- **设计**：声明资源读写、附件、shader、uniform 及 draw/dispatch。
- **业务**：业务系统构建 pass，图/设备后端负责实际执行。

| 源码模块 | 用途线索（来自源码） | 主要类型 / 接口 |
|---|---|---|
| [bind_resources](../../src/render/pass_builder/bind_resources.rs) | bind_resources.rs — BindGameResources trait（bind_color_library / bind_ray_tracing / bind_shadowmap）。 | `PassResourceSlot`、`BindGameResources`、`bind_color_library_impl`、`bind_ray_tracing_impl`、`bind_color_library_compute`、`bind_ray_tracing_compute` |
| [compute_pass_builder](../../src/render/pass_builder/compute_pass_builder.rs) | compute_pass_builder.rs — ComputePass 声明层。 | `AccessRange`、`ResourceRegistry`、`RgBuffer`、`ExternalSlot`、`ExternalBinding`、`ComputePassDevice`、`ComputePassInit`、`WorldAccess` 等 |
| [draw_builder](../../src/render/pass_builder/draw_builder.rs) | draw_builder.rs — DrawBuilder（按值消费的 draw 声明器）。 | `BrickEraseHighlightInputs`、`DrawCommand`、`NoDropOnlyTake`、`UniformValue`、`DrawTransform`、`CommonDrawFlag`、`CommonDrawParams`、`BindTexture` 等 |
| [draw_list](../../src/render/pass_builder/draw_list.rs) | draw_list.rs — `DrawListItem` / `EntityDrawParams`（CPU draw-list 项）。 | `DrawTopology`、`EntityDrawParams`、`DrawListFn`、`DrawListItem`、`DrawListEntry`、`tag`、`payload`、`new` 等 |
| [draw_stream](../../src/render/pass_builder/draw_stream.rs) | draw_stream.rs — DrawPacket / RasterPass / DrawPassRenderCheck / TextureBindingKey 与 DrawBuilder 所需的宿主边界。 | `RasterState`、`DescriptorSetBinding`、`GpuOwned`、`ResourceAccess`、`DrawKind`、`DrawPacket`、`TextureBindingKey`、`ToTextureDrawStreamBindingKey` 等 |
| [framebuffer_binding](../../src/render/pass_builder/framebuffer_binding.rs) | framebuffer_binding.rs — ColorAttachment / DepthAttachment（RasterPass 附件描述）。 | `AttachmentTexture`、`ColorViewWords`、`ColorAttachment`、`DepthClear`、`DepthAttachment`、`tag`、`desc`、`handle_mut` 等 |
| [pass_resource_access](../../src/render/pass_builder/pass_resource_access.rs) | pass_resource_access.rs — SSBO 访问登记与纹理访问 trait。 | `PassBinding`、`ResourceAccess`、`RecordingPassBuilder`、`SsboType`、`TextureDevice`、`TextureBinding`、`TextureCtx`、`TextureAccess` 等 |
| [pass_resources](../../src/render/pass_builder/pass_resources.rs) | pass_resources.rs — PassResources（`PassBuilder::render&lt;UserData = PassResources&gt;` 的用户数据）。 | `TypeId128`、`PassResources`、`ResourceLookup`、`new_from`、`shader_library`、`texture_library`、`ssbo_library` |
| [rg_resources](../../src/render/pass_builder/rg_resources.rs) | rg_resources.rs — RG（渲染图）资源创建接口（B 阶段 i_api 新增）。 | `RgError`、`TemporalTexture`、`TemporalTexturePair`、`TemporalBuffer`、`RgResources`、`RecordingRg`、`RgKind`、`into_backend_error` |
| [shader_uri](../../src/render/pass_builder/shader_uri.rs) | shader_uri.rs — ShaderUri 解析与 AsComputeShaderSource trait。 | `ShaderUri`、`ComputePipelineDesc`、`ShaderDefine`、`ShaderDefValue`、`AsShaderDefValue`、`ComputeShaderSource`、`ComputeShaderSourceHash`、`AsComputeShaderSource` 等 |
| [uniform_binder](../../src/render/pass_builder/uniform_binder.rs) | uniform_binder.rs — UniformBinder（uniform 块的映射内存 + 按名字哈希写入）。 | `UniformBlockInfo`、`UniformReflection`、`UniformAllocator`、`UniformBinder`、`UniformBytes`、`new`、`data`、`write_uniform` 等 |

<a id="resources"></a>


## resources

- **功能**：游戏拥有状态与跨域数据合同。
- **设计**：Resource、集合、ID、事件和序列化结构；具体字段以文件为准。
- **业务**：由启动或域系统生产，供交互、历史、保存和渲染消费。

| 源码模块 | 用途线索（来自源码） | 主要类型 / 接口 |
|---|---|---|
| [active_raster_stroke](../../src/resources/active_raster_stroke.rs) | active_raster_stroke.rs — 世界栅格工具（路径/鹅卵石/水/花园）的进行中笔画。 | `InProgressRasterStroke`、`ActiveRasterStroke`、`ActiveRasterStrokesStorage`、`solid`、`input`、`length`、`fluid_started`、`fluid_cursor` 等 |
| [audio_instance](../../src/resources/audio_instance.rs) | `audio_instance.rs` — 音频实例的已恢复判定与循环播放器（B `resources::audio_resources` / `wall_audio_utils`）。 | `PlaybackState`、`LoopInstance`、`AudioLoopPlayer`、`should_start_if_not_playing`、`should_start`、`new`、`current`、`request_play` 等 |
| [audio_resources](../../src/resources/audio_resources.rs) | B `resources/audio_resources.rs`: the FMOD audio core state — AudioInstance model, bank table and the AudioCore (FMOD Studio) producer. | `EventInstance`、`AudioInstance`、`AudioLoopDesc`、`AudioSamplesLoadedPromise`、`Bank`、`Studio`、`VcaHandle`、`ListenerAttributes` 等 |
| [audio_settings](../../src/resources/audio_settings.rs) | `country_core::resources::audio_settings::AudioSettings`（B 原模块路径；组 A 未落库）。 | `AudioSettings` |
| [brush_sizes](../../src/resources/brush_sizes.rs) | brush_sizes.rs — 路径/水/花园三个工具的笔刷尺寸（`resources/brush_sizes.rs`）。 | `BrushSizes`、`reset_path`、`set_path`、`reset_water`、`set_water`、`reset_garden`、`set_garden` |
| [color_icon](../../src/resources/color_icon.rs) | B brightness selector, 0x140b06770. Variant spellings are not available; preserve the three observed tags rather than inventing a Custom(f32) variant. | `ColorMenuBrightnessValue`、`CustomizationIcon`、`ColorIcon`、`WallStyleIcon`、`CustomizationIconMarkedForDeletion`、`from_tag`、`tag`、`to_f32` 等 |
| [color_id](../../src/resources/color_id.rs) | ColorId newtype observed in B components and the render consumers. | `ColorId`、`WallColorIds`、`GateColorIds`、`HalftimberColorIds`、`RoofColorIds`、`SetWallColorIdCmd`、`SetGateColorIdCmd`、`SetHalftimberColorIdCmd` 等 |
| [color_id_roof](../../src/resources/color_id_roof.rs) | `resources::color_id_roof` — 屋顶颜色 SSBO 元素（B 原模块路径 `country_core::resources::color_id_roof`）。 | `RoofColorIdSsbo` |
| [cursor_terrain_raycast](../../src/resources/cursor_terrain_raycast.rs) | cursor_terrain_raycast.rs — 光标地形射线（WP8 恢复 + ① T1 契约补全）。 | `CursorTerrainRaycast`、`new`、`update`、`position`、`distance` |
| [editing_highlight](../../src/resources/editing_highlight.rs) | B EditingHighlight: three float color channels and enabled byte at +0xc. clear's 13-byte store sequence is 0x14097689c..0x1409768ab. | `EditingHighlight`、`clear` |
| [events](../../src/resources/events.rs) | events.rs — `country_core::resources::events` 命令事件族（组 A restorer 接线）。 | `ConstructTimberframeCmd`、`ModeJustChanged`、`EscapeMenuJustOpened`、`LightLanternCmd`、`LightWindowCmd`、`FinishInputRecordingCmd`、`DisplayRollercoasterPrompt`、`WriteSettingsFileCmd` 等 |
| [game_clock](../../src/resources/game_clock.rs) | `game_clock.rs` — `time_update`（B 0x140922c30）写入的 0x38 B 游戏时钟。 | `DeltaSource`、`ClockGate`、`GameClockError`、`GameClock`、`from_menu`、`runs`、`update` |
| [garden_mask](../../src/resources/garden_mask.rs) | `garden_mask.rs` — 花园 mask 回读 SSBO（B：`GardenMaskSsbo`，O 名）。 | `GardenMaskSsbo` |
| [glade_border](../../src/resources/glade_border.rs) | B border queries 0x140ad2950 / 0x140ad2970. Circle borders use a strict distance comparison; rectangle borders sample a circle at step 1.0. | `BorderShape`、`GladeBorder`、`DisplayOutOfBorder`、`FeedbackInputOutOfBorder`、`ShowDemoBorderOverlay`、`NotInDemoHint`、`is_pos_inside`、`is_shape_inside` |
| [glade_color_customization](../../src/resources/glade_color_customization.rs) | B `resources/glade_color_customization.rs` — 15 字段颜色定制（≥0x48B）。 | `GladeColorCustomization` |
| [glade_settings](../../src/resources/glade_settings.rs) | B GladeSettings: the eleven-key serde schema at 0x142aceb88 and native default at 0x1408c2240. Padding and Rust enum niches are not settings. | `GladeSettings`、`try_load`、`load` |
| [gothic_window_profile](../../src/resources/gothic_window_profile.rs) | gothic_window_profile.rs — Gothic 窗/门拱剖面曲线（WP21）。 | `gothic_half_dormer_profile_curve`、`gothic_door_profile_curve` |
| [grading_lut](../../src/resources/grading_lut.rs) | `grading_lut.rs` — 调色 LUT：`.cube` / 展平 PNG → 3D 纹理（`load_cube` O 0x140b01050、 `load_unwrapped` O 0x140b00bc0、`GradingLuts::load` O 0x140b01df0，均 fresh 反编译 2026-10-17）。 | `GradingLut`、`GradingLutMeta`、`TextureRecord`、`GradingLuts`、`load_cube`、`load_unwrapped`、`load_unwrapped_from_path`、`new` 等 |
| [grading_lut_cube](../../src/resources/grading_lut_cube.rs) | `.cube` 调色 LUT 解析（`GradingLut::load_cube` 0x140b01050 的纯逻辑部分，不触设备）。 | `CubeLut`、`texel_bytes`、`parse_rgb`、`parse_cube` |
| [gui](../../src/resources/gui.rs) | B GUI state and menu policies. Native tags are exposed only through the view; owned Rust enums preserve behavior without promising the original compiler ABI. | `EguiContextResource`、`PendingEguiContextResource`、`PhotomodeScreen`、`EscapeMenuJustClosed`、`EscMenuState`、`NextEscMenuState`、`PrevEscMenuState`、`LoadMenuState` 等 |
| [image_formation](../../src/resources/image_formation.rs) | image_formation.rs — 画面形成配置（WP21；B 阶段 i_infra 按机器码修正布局）。 | `ImageFormationConfig`、`ImageFormationConfigs`、`default_plain`、`default_third`、`flattened`、`with_process_flag`、`initial`、`current` 等 |
| [in_progress_curve](../../src/resources/in_progress_curve.rs) | B `resources/in_progress_curve.rs`: the freehand raw-input curve resource. | `InProgressCurve`、`new`、`try_add_raw_input_points`、`clear`、`first_raw_input_point`、`last_raw_input_point`、`raw_input_curve_length_ws`、`is_raw_input_curve_valid` 等 |
| [logo_animation](../../src/resources/logo_animation.rs) | `logo_animation.rs` — 启动 logo 的五阶段进度（B `resources::logo_animation`）。 | `LogoAudioEvent`、`LogoAnimation`、`path`、`play`、`is_waiting`、`is_finished`、`finish_wait`、`animate` |
| [mod（本目录入口）](../../src/resources/mod.rs) | 未提供独立用途总述；通过下列源码类型/接口定位。职责按本节领域合同理解，不据文件名推断完成度。 | `PlacementDataSsbo` |
| [mouse_capture](../../src/resources/mouse_capture.rs) | Native Option&lt;Vec2&gt;; the translator tests its discriminant, and keeps the capture anchor owned by mouse capture rather than moving the window cursor. | `CursorCapturedPosition` |
| [mouse_terrain_raycast](../../src/resources/mouse_terrain_raycast.rs) | Screen-space f32 position read by B 0x140b17a80. This is distinct from the world-space CursorTerrainRaycast, and from integer WindowSize dimensions. | `CursorPositionSS`、`CameraCursorRay`、`intersect_with_xz_plane` |
| [onboarding](../../src/resources/onboarding.rs) | `resources::onboarding` — 教学/提示系统通知事件族（B `system_onboarding` 消费）。 | `NotifyHintSystemShapeEditWasPerformed`、`NotifyHintSystemCameraMoved`、`NotifyHintSystemSnapping`、`NotifyHintSystemTerrain`、`NotifyHintSystemWindow`、`NotifyHintSystemToolWasJustUsed`、`NotifyHintSystemSomethingWasErased`、`NotifyHintSystemTerrainSteepness` 等 |
| [path_stroke](../../src/resources/path_stroke.rs) | path_stroke.rs — `country_core::resources::path_stroke` 路径笔画事件（组 A restorer 接线；`system_tree::ui_delete_tree` 0x141bd2510 的 `EventReader&lt;PathDrawEvent&gt;` 面——删除树 UI 以路径笔画为擦除输入）。 | `PathDrawEvent`、`PathEraseEvent` |
| [photo_mode](../../src/resources/photo_mode.rs) | B photo camera selection, distinct from MainCamera's internal camera modes. | `PhotoModeCameraMode`、`next`、`next_demo` |
| [prefab](../../src/resources/prefab.rs) | prefab.rs — 预制体引用组件 + 预制体库（`PrefabLibrary`，fresh 反编译 2026-10-17）。 | `PrefabInstance`、`Prefab`、`PrefabPart`、`PrefabAudioTrigger`、`PrefabLibrary`、`PrefabsModifiedCmd`、`PrefabWatchPlugin`、`albedo_for_season` 等 |
| [raster_stroke](../../src/resources/raster_stroke.rs) | Owned raster-history payload, B record_raster_history 0x140957990. The outer history edit holds Arc&lt;RasterStroke&gt;; points are Arc&lt;[Vec3]&gt;. | `RasterStroke`、`radius`、`iter_points_xz` |
| [render_input_views](../../src/resources/render_input_views.rs) | Explicit input views for recovered draw callbacks. These expose only fields observed by those callbacks and are not authoritative game resource schemas. | `IvyDrawView` |
| [render_ssbo](../../src/resources/render_ssbo.rs) | `resources::render_ssbo` — frame-pass SSBO 载体（wp5 的 typed-gap 占位 + B 阶段 i_infra 恢复的真实库侧类型）。 | `LanternSsbo`、`MouseDepthValueSsbo` |
| [rocky_terrain](../../src/resources/rocky_terrain.rs) | `rocky_terrain.rs` — 岩质地层网格资源（B 0x1408bbf50 `RockyTerrainMesh::load`，O）。 | `LoadedRockyTerrainVertex`、`LoadedRockyTerrainTriangle`、`RockyTerrainVertexSsbo`、`RockyTerrainTriangleSsbo`、`RockyTerrainMesh`、`RockyTerrainJson`、`load` |
| [rt](../../src/resources/rt.rs) | `resources/rt` — 光线追踪（ray-tracing）世界与 GPU BVH（B `resources/rt`，13 登记）。 | `GpuRtTriangle`、`RtMesh`、`RtMeshHandle`、`DirtyCow`、`RtWorld`、`RtTriangleSsbo`、`RtTriangleMetaSsbo`、`RtBvhRecord` 等 |
| [save_file](../../src/resources/save_file.rs) | `save_file.rs` — 存档文件的已恢复纯逻辑：节表、影子文件脏块位图、追加编辑、读档升级与版本开关。 | `SaveFileSection`、`DirtyBlocks`、`ShadowBuffer`、`LoadUpgradeFlags`、`LoadVersionGates`、`json_key`、`padding`、`clear` 等 |
| [session](../../src/resources/session.rs) | B session owners. Metadata field names follow the original serializers. | `SessionId`、`GladeType`、`SessionMutator`、`SessionMeta`、`ActiveSession`、`SessionLoadAttempt`、`SessionMetaSource`、`NextSessionKind` 等 |
| [sparkle](../../src/resources/sparkle.rs) | `resources::sparkle` — 火花粒子稀疏实例元素（`SparkleSsbo`）。 | `SparkleSsbo` |
| [stairs_walls](../../src/resources/stairs_walls.rs) | `stairs_walls.rs` — 楼梯-墙支柱提议（B `resources/stairs_walls.rs`，1 业务 + 4 胶水）。 | `StairsPillarProposal`、`StairsPillarProposals`、`StairsArchSupportRequests`、`StairsBracketsRequests`、`from_observed`、`observed`、`add`、`drain_all` 等 |
| [steam](../../src/resources/steam.rs) | `resources/steam.rs` — Steam 动作集定义与 SteamInput 资源。 | `SteamSdk`、`AnalogActionData`、`DigitalActionData`、`SteamActionSetDigitalAction`、`DigitalActionSetActionType`、`SteamMainActionSet`、`SteamMainActionSetState`、`SteamMenuActionSet` 等 |
| [terrain_editing](../../src/resources/terrain_editing.rs) | One ECS owner for the already recovered tg-terrain stroke map/ID policy. The original startup initializer is separate; explicit state is required. | `TerrainEditingData` |
| [terrain_height](../../src/resources/terrain_height.rs) | terrain_height.rs — `TerrainHeightsData&lt;T&gt;`：512×512 的 u16 地形高度场的 CPU 镜像。 | `TerrainRayHit`、`TerrainHeightsData`、`new`、`heights`、`heights_mut`、`min_height`、`max_height`、`set_height_range` 等 |
| [terrain_rock_mask](../../src/resources/terrain_rock_mask.rs) | `terrain_rock_mask.rs` — 岩石 mask 回读资源（B 0x1409ea800 `sample_at_world_xz`，O 结构）。 | `TerrainRockMaskSsbo`、`sample_at_world_xz` |
| [texture_presets](../../src/resources/texture_presets.rs) | `resources::texture_presets` — 纹理预设绑定类型（O 名，字段 typed gap）。 | `TextureFromDisk`、`TexturePreset`、`PresetTexture`、`texture`、`usage` |
| [time](../../src/resources/time.rs) | B `GameTime` resource consumed at +0x28 by GUI log decay and roof animation. The existing clock implementation follows `time_update` (0x140922c30). Re-export preserves one Bevy resource identity across these consumers. | 内部实现 / 模块装配（见源码） |
| [time_of_day](../../src/resources/time_of_day.rs) | `time_of_day.rs` — 一天中的小时重映射 + 昼夜序列/光照快照资源 (B `country_core::resources::time_of_day`, 0x140a74790 等). | `TodPlaybackState`、`TimeOfDay`、`TimeOfDayState`、`TodScreen`、`TodDevConfig`、`TodInputSeam`、`TodWeatherSeam`、`TodSpeedBiasGlobal` 等 |
| [toolbar](../../src/resources/toolbar.rs) | B Toolbar constructor (0x140af8c50), contains (0x140af89d0), switch_mode (0x140af9850), and cycling closures (0x140af9f30..0x140afaeE0). | `ToolbarCategory`、`ToolbarCategoryModes`、`Toolbar`、`from_index`、`contains`、`last_selected_mode`、`next_mode`、`new` 等 |
| [tree_storage](../../src/resources/tree_storage.rs) | Authoritative tree-index → entity owner (B 0x140a38460..0x140a38600). The native table uses std::HashMap; its bucket layout is not game logic. | `TreeStorageIndex`、`TreeStorageEntry`、`TreeStorage`、`InteractableForestTree`、`InteractableForestPlacements`、`MoveTreeCmd`、`HideTreesinAreaCmd`、`HideTreesNextToWallCmd` 等 |
| [ui_settings](../../src/resources/ui_settings.rs) | B UiSettings schema and defaults. Original writes at 0x140b12f10 and serialization at 0x140a46ad0 supersede the old field/default table. | `ToolbarAlignment`、`UiSettings`、`is_horizontal` |
| [water](../../src/resources/water.rs) | water.rs — `country_core::resources::water` 水内容资源模块（组 A restorer 接线）。 | `RippleSourceSsbo`、`WaterDrawEvent`、`WaterEraseEvent` |
| [water_exclude_clutter_mask](../../src/resources/water_exclude_clutter_mask.rs) | `water_exclude_clutter_mask.rs` — 水位排除杂物 mask 回读 SSBO（B：`WaterExcludeClutterMaskSsbo`，O 名）。 | `WaterExcludeClutterMaskSsbo` |
| [water_mask](../../src/resources/water_mask.rs) | water_mask.rs — `country_core::resources::water_mask` 水 mask 资源（WP7 恢复）。 | `WaterMaskSsbo` |
| [window](../../src/resources/window.rs) | B WindowSize stores two u32 values (set 0x140949c00). as_vec2 (0x140949c10) zero-extends and converts each to f32; it is not a bitcast. | `AvailableMonitors`、`AvailableMonitor`、`WindowFocusEvent`、`CursorMovedScreenSpace`、`WindowSize`、`clear`、`new`、`set` 等 |
| [world_raster](../../src/resources/world_raster.rs) | world_raster.rs — 世界光栅缓冲族。 | `UploadWorldRasterBuffersToStagingCmd`、`RasterBufferRef`、`TileHasher`、`DirtyTilePublisher`、`StrokeBlendMode`、`WorldRaster`、`new`、`bilinear_sample_at_world_xz` 等 |
| [world_raster_defs](../../src/resources/world_raster_defs.rs) | world_raster_defs.rs — 命名世界栅格与 `AllWorldRasterWrite`。 | `NamedRasterData`、`WorldRasterResources`、`AllWorldRasterRead`、`AllWorldRasterWrite`、`new`、`would_write_succeed`、`write_named`、`write` 等 |

<a id="resources-critters"></a>


## resources/critters

- **功能**：动物的 core 侧状态合同。
- **设计**：当前 sheep 状态、快照与控制数据，不代替完整动物 owner。
- **业务**：捕获和恢复羊状态，连接保存与调试控制。

| 源码模块 | 用途线索（来自源码） | 主要类型 / 接口 |
|---|---|---|
| [mod（本目录入口）](../../src/resources/critters/mod.rs) | Original B critter data. Species owners are added only with recovered fields. | `CritterGoal` |
| [sheep](../../src/resources/critters/sheep.rs) | B Sheep's recovered movement state, stored in the utils `Transform` translation slot (O: capture 0x1412f03f0 / apply 0x1416f24f0). | `CmdCheckForSheepInsideBuilding`、`SheepMoveState`、`Sheep`、`to_translation`、`from_translation`、`being_petted` |

<a id="resources-critters-sheep"></a>
| [critters](../../../systems/critters/src/components.rs) | system-critters raw locomotion predicates from B collision consumers. Species names and original enum layout remain unknown; this is a CPU input projection, not a replacement Bevy component or an eight-byte ABI claim. | `CritterLocomotion`、`samples_rock_mask`、`skips_water_sample`、`skips_public_walls`、`wall_hash_only` |


## resources/critters/sheep

- **功能**：羊控制与快照的子合同。
- **设计**：动物控制状态与完整动物 AI 分离。
- **业务**：为羊的调试控制、保存及恢复提供数据接口。

| 源码模块 | 用途线索（来自源码） | 主要类型 / 接口 |
|---|---|---|
| [control](../../src/resources/critters/sheep/control.rs) | B control snapshots: native serializer 0x1416f9eb0/0x1416f9fd0, capture iterator 0x1412f03f0, restore consumer 0x1416f24f0. | `SheepSnapshot`、`SheepControlSnapshot`、`capture_sheep_snapshot`、`apply_sheep_snapshot`、`sheep_section_json` |

<a id="resources-history"></a>


## resources/history

- **功能**：各类编辑载荷与回放命令缓冲。
- **设计**：按域定义 edit，历史资源统一管理链、游标和事务。
- **业务**：记录用户操作，撤销/重做时向相应域发回放命令。

| 源码模块 | 用途线索（来自源码） | 主要类型 / 接口 |
|---|---|---|
| [auto_clutter_edit](../../src/resources/history/auto_clutter_edit.rs) | B tag-13 expansion at 0x14143fc1e: command tag 9, duration bits | `AutoClutterReplayCmd`、`expanded_shared`、`make_instant` |
| [color_edit](../../src/resources/history/color_edit.rs) | B `resources/history/color_edit.rs` — 颜色编辑负载（fresh 0x14003c5d0 serialize / 0x1417bf8a0 visit_str / 0x1417befe0 deserialize）。 | `ColorTarget`、`ColorEdit`、`native_discriminant`、`table_index`、`key`、`from_key`、`id_key`、`target` |
| [decorator_edit](../../src/resources/history/decorator_edit.rs) | B `resources/history/decorator_edit.rs` — 装饰历史编辑/状态/差分类型（章节 O）。 | `DecoratorHistoryEdit`、`DecoratorHistoryState`、`DecoratorHistoryDiff`、`DecoratorStorageAddrHistoryState`、`DecoratorStorageHistoryDiff`、`DecoratorHistoryDiffMap`、`DecoratorStateWithStorage`、`DecoratorBeforeAfterWithStorage` 等 |
| [mod（本目录入口）](../../src/resources/history/mod.rs) | Live history uses the recovered transaction owner directly. Edit tags are verified against original B Debug dispatch 0x140a842c0 / 0x142ad88d0. Variants whose payloads remain unresolved are not represented by empty data. | `History`、`HistoryIdentity`、`EditType`、`empty_glade`、`debug_display`、`debug_set_history_start`、`generate`、`mutate` 等 |
| [path_edit](../../src/resources/history/path_edit.rs) | B `resources/history/path_edit.rs` — 路径/鹅卵石光栅编辑负载（章节 O；13 函数）。 | `RasterEdit`、`RasterEditSerialized`、`from_points` |
| [replay_cmd_buffer](../../src/resources/history/replay_cmd_buffer.rs) | Mixed owner commands in a single front bucket (B replay_roof 0x141a37ad0). This does not split each edit into an outer bucket. Auxiliary decorator and stair owners must be recovered before accepting their nonempty buckets. | `HistoryReplayError`、`ReplayCommand`、`ColorReplayCmd`、`DecoratorReplayCmd`、`ReplayBucketAux`、`ReplayBucket`、`CommandTiming`、`ReplayBuffer` 等 |
| [stairs_history_edit](../../src/resources/history/stairs_history_edit.rs) | B `resources/history/stairs_history_edit.rs` — 楼梯编辑负载（章节 O；12 函数）。 | `StairsEdge`、`StairsHistoryEdit`、`StairsReplayCmd`、`edge`、`new`、`new_with_kind`、`is_done`、`expanded` 等 |
| [tree_edit](../../src/resources/history/tree_edit.rs) | B `resources/history/tree_edit.rs` — 树编辑负载（章节 O；25 函数）。 | `TreeEditKind`、`TreeEditData`、`TreeEdit`、`TreeCanopyMaxCullDistance`、`InsertTreeCmd`、`DeleteTreeCmd`、`SpawnPreplacedTrees`、`TreeReplayCmd` 等 |

<a id="resources-ivy"></a>
| [building_history_transactions](../../src/resources/history/building_transactions.rs) | B building history owner expansion over the already recovered transaction ledger. Roof owners and proven new/deleted Wall concurrent groups are executable. The original replay_edit expands forward, then reverses the *commands*, not arbitrary edits. | `WallHeightHistoryEdit`、`WallHeightReplayRequest`、`RoofHistoryEdit`、`BuildingHistoryError`、`RoofVisualHistoryRecord`、`BuildingEdit`、`BuildingReplayOutput`、`RoofReplayCommand` 等 |
| [history_snapshots](../../src/resources/history/snapshots.rs) | `history_snapshots.rs` — `History` 的快照表、起始编辑与空判定（B `resources::history`）。 | `HistoryChainSnapshotPayload`、`nearest_snapshot_index`、`current_snapshot`、`set_current_snapshot`、`resolve_snapshot`、`closest_resolved_snapshot`、`insert_starting_edit`、`is_decorator_history_diff_empty` 等 |
| [history_transactions](../../src/resources/history/transactions.rs) | Observed B History recording/branch/cursor policy, using normal Rust owners. This is not an ABI replica or a decoder of the unresolved auxiliary owners. | `HistoryRecordingValidation`、`HistoryEntry`、`ResolvedHistorySnapshot`、`ObservedHistoryLedger`、`HistoryTransactionError`、`HistoryCommit`、`HistoryCursorStep`、`HistoryReplaySelection` 等 |
| [wall_history_edit](../../src/resources/history/wall_edit.rs) | B Wall compound snapshot business. Vec/Arc/ledger ownership remains std. | `MorphPremadeShapeEdit`、`WallHistoryMetadata`、`WallAuthoritySnapshot`、`WallHistoryEdit`、`UnresolvedWallEditPayload`、`ConcurrentEdits`、`WallCompoundHistoryEdit`、`CompoundEditBuilder` 等 |
| [clutter](../../src/resources/history/auto_clutter_edit.rs) | country-core AutoClutterEdit data contract: externally tagged Delete{id:u64}. Live deletion/history consumers reuse this type through resources::history. | `AutoClutterEdit` |


## resources/ivy

- **功能**：常春藤植株、枝叶存储、墙面坐标和方向策略。
- **设计**：株与叶分开存储，坐标和方向 proposer 独立。
- **业务**：生长/修剪系统读取和修改状态，动画及上传系统消费。

| 源码模块 | 用途线索（来自源码） | 主要类型 / 接口 |
|---|---|---|
| [ivy_direction_proposer](../../src/resources/ivy/ivy_direction_proposer.rs) | B default 0x1412343b0, get_direction 0x141234590. The original bracket-noise 0.8.2 and perchance 0.4.0 own noise and random generation. | `IvyDirectionProposer`、`IvySpawnNoise`、`new`、`get_direction` |
| [ivy_leaf_storage](../../src/resources/ivy/ivy_leaf_storage.rs) | B add 0x1408fe280 and point-removal predicate 0x1412f54b1..15. | `IvyLeafRng`、`IvyLeaf`、`IvyLeafInput`、`IvyLeafStorage`、`new`、`add`、`get`、`all_leaves` 等 |
| [ivy_storage](../../src/resources/ivy/ivy_storage.rs) | B 0x140869460..0x140869941: owned polyline, root and spawn state. | `IvyId`、`IvyPointId`、`IvyPoint`、`IvyGrowthSegment`、`Ivy`、`IvyStorage`、`add_point`、`point_count` 等 |
| [mod（本目录入口）](../../src/resources/ivy/mod.rs) | Live ivy authority. Native Vec/HashMap/RNG implementations remain dependencies. | 内部实现 / 模块装配（见源码） |
| [wall_coord](../../src/resources/ivy/wall_coord.rs) | B 0x140b199e0: measured wall distance and Y → world X/Y/Z. | `WallCoord`、`from_wall_u`、`into_world_space` |

<a id="resources-render"></a>


## resources/render

- **功能**：共享渲染资源。
- **设计**：网格图集与空间解析偏移作为明确的共享输入。
- **业务**：供实例和帧渲染定位网格及采样偏移。

| 源码模块 | 用途线索（来自源码） | 主要类型 / 接口 |
|---|---|---|
| [mesh_atlas_library](../../src/resources/render/mesh_atlas_library.rs) | `resources/render/mesh_atlas_library.rs` -- mesh 图集库资源（A3 res_render 按机器码重写）。 | `PrefabNameRef`、`PrefabName`、`AtlassedMeshName`、`PrefabMeshAtlasPart`、`WorldSubsetMeshAtlas`、`MeshAtlasesModifiedCmd`、`MeshAtlasLibrary`、`new` 等 |
| [mod（本目录入口）](../../src/resources/render/mod.rs) | `resources/render/mod.rs` -- resources::render 族（A3 res_render）。 | `Affine3Packed`、`from_mat3_translation`、`from_cols`、`add_y` |
| [spatial_resolve_offsets](../../src/resources/render/spatial_resolve_offsets.rs) | `resources/render/spatial_resolve_offsets.rs` -- 空间解析偏移表 SSBO（A3 res_render 按机器码重写）。 | `SpatialResolveOffsetsSsbo`、`default_value`、`taps` |

<a id="resources-roofs"></a>


## resources/roofs

- **功能**：屋顶状态、瓦片、动画及渲染载荷。
- **设计**：参数/几何与瓦片模拟、SSBO、触发体分离。
- **业务**：墙和屋顶参数变化后重建几何，维护视觉、瓦片和删除动画。

| 源码模块 | 用途线索（来自源码） | 主要类型 / 接口 |
|---|---|---|
| [commands](../../src/resources/roofs/commands.rs) | Roof commands and downstream notifications. Bevy owns event retention. | `CreateRoofCmd`、`DeleteRoofCmd`、`SwitchRoofDirectionCmd`、`SetFlatRoofStyleCmd`、`SetFlatRoofRimStyleCmd`、`EditRoofCmd`、`RoofIsInvalid`、`ResetRoofs` 等 |
| [dormer_tiles](../../src/resources/roofs/dormer_tiles.rs) | `dormer_tiles.rs` — DormerTiles 资源（B `resources/roofs/dormer_tiles.rs`，1 胶水）。 | `DormerTiles` |
| [mod（本目录入口）](../../src/resources/roofs/mod.rs) | 屋顶事件注册（原 `systems/roof` 的注册面；事件类型属于 core， 由 system-roof 的 `add_roof_events_and_resources`（0x1406305f0）调用）。 | `register_roof_events` |
| [roof_animation](../../src/resources/roofs/roof_animation.rs) | B 0x1408dbc40 / 0x1421a2da0: two scalar animation channels. The consuming query strides by eight bytes; two zeroed XMM return registers do not imply a sixteen-byte component (the historical chapter did so). | `RoofAnimation`、`RoofMarkedForDeletion`、`from_inner`、`inner`、`advance_observed`、`is_being_animated`、`values` |
| [roof_buffers](../../src/resources/roofs/roof_buffers.rs) | `roof_buffers.rs` — RoofBuffers 资源 + WoodenPlankSsbo 稀疏梁实例缓冲元素（B `resources/roofs/roof_buffers.rs`）。 | `RoofBuffers`、`WoodenPlankSsbo`、`new` |
| [roof_noise](../../src/resources/roofs/roof_noise.rs) | B `resources/roofs/roof_noise.rs` — 屋顶顶点噪声位移（章节 O；8 函数）。 | `hash1`、`hash_combine2`、`hash2`、`uint_to_u01_float`、`lerp`、`noise`、`apply_roof_noise`、`revert_roof_noise` |
| [roof_ridge](../../src/resources/roofs/roof_ridge.rs) | `roof_ridge.rs` — 屋顶脊线尺寸与几何（B `resources/roofs/roof_ridge.rs`，7 业务）。 | `RoofRidgeDims`、`new`、`get`、`from`、`calc`、`to_world_space`、`get_inner`、`max_dim` 等 |
| [roof_shape](../../src/resources/roofs/roof_shape.rs) | B Roof component and its observed 0x58-byte game fields. Geometry is shared with the recovered system-roof crate. The byte projection preserves appearance fields whose original Rust enum schema is not yet known. | `Roof`、`RoofIntersectionSection`、`RoofShapeParams`、`GableRoofStyle`、`FlatRoofStyle`、`RoofUvFailure`、`RoofUvColliderIndex`、`from_observed` 等 |
| [roof_tile_sim](../../src/resources/roofs/roof_tile_sim.rs) | Owns the B tile simulation state initialized by create_roof. | `RoofTileSimulation`、`from_inner`、`new`、`observed`、`update_wall_height`、`replace_observed` |
| [roof_tile_ssbo](../../src/resources/roofs/roof_tile_ssbo.rs) | `roof_tile_ssbo.rs` — `RoofTileSsbo` 稀疏实例缓冲元素（B `resources/roof_tile_ssbo.rs`；本 crate 按目录规则落在 `resources/roofs/`）。 | `RoofTileSsbo` |
| [roof_tiles](../../src/resources/roofs/roof_tiles.rs) | `roof_tiles.rs` — RoofTile/RoofTiles 组件（B `resources/roofs/roof_tiles.rs`）。 | `RoofTile`、`RoofTiles`、`draw_ranges` |
| [roof_trigger_volume](../../src/resources/roofs/roof_trigger_volume.rs) | B triangle, normal and UV ownership; generic Vec storage is reused. | `RoofTriggerVolume`、`triangles`、`normals`、`uvs`、`set_triangles`、`set_normals`、`set_uvs`、`get_position_from_mesh_uv` |
| [roof_visual_shape](../../src/resources/roofs/roof_visual_shape.rs) | `roof_visual_shape.rs` — 屋顶可视形状查询族（B `roofs/roof_visual_shape.rs`，6 业务）。 | `RoofBoundsDims`、`as_circle`、`as_rectangle`、`get_roof_shape_from_pos_ws_clamped`、`get_height_01_unclamped_from_ws`、`get_roof_shape`、`get_roof_expanded_top_and_bottom` |

<a id="resources-roofs-geometry"></a>
| [roof_associations](../../src/resources/roofs/associations.rs) | wall_roof_hashmap business policies over standard HashMap, not hash probing. | `AssociationError`、`AssociationAction`、`ResolvedRoofAssociation`、`RoofAssociations`、`AssociationChanges`、`association_action`、`entity`、`has_roof` 等 |
| [roof_components](../../src/resources/roofs/components.rs) | Game-owned creation and animation values; dependency Vec/ECS storage excluded. | `RoofTileSimulation`、`EmptyRoofTriggerVolume`、`RoofAnimation`、`PreparedRoofGameplayComponents`、`RoofTriggerVolume`、`from_observed`、`initialize`、`bytes` 等 |
| [roof_creation](../../src/resources/roofs/creation.rs) | B create_roof: game initialization after successful public-wall resolution. ECS spawn, deferred commands and subsequent animation/drawable bundles are outside this recovered initialization range, never handwritten here. | `RoofCreateRequestObserved`、`RoofCreationError`、`RoofCreationOutcome`、`from_observed`、`from_replay_event`、`wall_id`、`record`、`initialize_resolved_roof` |
| [roof_customization](../../src/resources/roofs/customization.rs) | B system_color::set_roof_customization's resolved game consumers. PublicWalls/ECS resolution and event/Arc/History container implementations remain dependency/owner boundaries. Missing associations/query rows skip. | `FlatRoofStyleObserved`、`FlatRoofStyleRequest`、`FlatRoofRimRequest`、`RoofCustomizationSound`、`RoofRimHistoryMutation`、`RoofCustomizationOutcome`、`from_observed`、`bytes` 等 |
| [roof_deletion](../../src/resources/roofs/deletion.rs) | Game request matching and animation-completion removal policy; Commands reused. | `RoofDeletionMark`、`match_deletion_request`、`should_despawn` |
| [roof_direction](../../src/resources/roofs/direction.rs) | Original ridge-direction event consumer after wall/entity/query resolution. The Bevy storage and writer buffers are dependencies; notification policy is game logic. | `RoofDirectionRequest`、`RoofDirectionChange`、`apply_resolved_direction` |
| [roof_invalid_notifications](../../src/resources/roofs/invalid_notifications.rs) | B process_invalid_roof_notifications over real resolved Query rows. Query iteration, EventReader/EventWriter, Arc and History remain std/Bevy owners. Every returned History mutation must reach an explicitly configured ledger; producing the mutation is not a… | `InvalidRoofRow`、`InvalidRoofDeletion`、`process_invalid_roof_notifications` |
| [roof_lifecycle](../../src/resources/roofs/lifecycle.rs) | Links recovered creation, committed deletion marks, animation and removal. Library command flushes are explicit boundaries, not an invented scheduler. | `ResolvedRoofLifecycle`、`prepare`、`consume_delete_requests`、`deletion_mark_committed`、`advance` |
| [roof_spatial](../../src/resources/roofs/spatial.rs) | B update_roof_spatial's game update after PublicWalls/ECS have resolved state. Library entity tables, queries and event-buffer implementation are excluded. | `SpatialUpdateError`、`RoofGeometryChange`、`RoofSpatialOutcome`、`synchronize_resolved_roof` |


## resources/roofs/geometry

- **功能**：屋顶几何构造与编辑规则。
- **设计**：参数→屋脊/表面/边/面/端盖→瓦片；回放与编辑规则单列。
- **业务**：修改屋顶外形后重新计算派生几何。

| 源码模块 | 用途线索（来自源码） | 主要类型 / 接口 |
|---|---|---|
| [edit_rules](../../src/resources/roofs/geometry/edit_rules.rs) | Game-owned edit_roof rules recovered from RE-MCP decompilation and original instructions. ECS lookup/MessageWriter storage is outside this module. | `ScalarEdit`、`TipEdit`、`RoofEditEvent`、`EditChanges`、`EditFeedback`、`HeightCrossing`、`EditOutcome`、`from_observed` 等 |
| [mod（本目录入口）](../../src/resources/roofs/geometry/mod.rs) | Roof geometry recovered from the original binary, hosted in country-core. | 内部实现 / 模块装配（见源码） |
| [parameters](../../src/resources/roofs/geometry/parameters.rs) | B RoofShapeParams field mapping from original serialization instructions. See evidence/multi-version/B-parameter-consumers and B-parameter-findings.md. This adapter preserves bytes whose semantics/defaults are not established. | `RoofParameterBlock`、`from_bytes`、`bytes`、`from_roof`、`write_to_roof`、`replace_serialized_fields`、`direction`、`profile_01` 等 |
| [pipeline](../../src/resources/roofs/geometry/pipeline.rs) | Restored CPU roof generator source chain. No ECS, replay or GPU execution. | `RoofBuildError`、`RoofTileBatch`、`assemble_roof_tiles_observed` |
| [rectangular_caps](../../src/resources/roofs/geometry/rectangular_caps.rs) | Ridge-cap TileInstance producer, recovered from original rectangular roof generator 0x1421afc02..0x1421b062d. The observed entry derives the complete ridge endpoint preamble from the original rectangle and Roof bytes. | `CapInput`、`cap_endpoints_from_observed`、`assemble_caps_from_observed`、`assemble_ridge_caps` |
| [rectangular_edges](../../src/resources/roofs/geometry/rectangular_edges.rs) | Original rectangular roof's four hip/eave strip loops (0x1421af190..afbfb). Prepared frames come from the original rectangle-bound and transform chain. | `EdgeFrame`、`frames_from_observed`、`assemble_edges_from_observed`、`assemble_rectangular_edges`、`assemble_edge` |
| [rectangular_faces](../../src/resources/roofs/geometry/rectangular_faces.rs) | Original rectangular face-tile loop, separated at its recovered row-vector boundary. Row interpolation and whole-roof assembly are composed by callers. | `FaceRowsInput`、`PreparedRectangularFace`、`prepare_rectangular_faces_observed`、`assemble_faces_from_observed`、`assemble_rectangular_face_tiles` |
| [replay](../../src/resources/roofs/geometry/replay.rs) | B RoofReplayCmd::process VisualEdit branch, with event writers already resolved. Does not implement history cursor selection, ECS lookup or event scheduling. | `ObservedReplayEvent`、`VisualReplayState`、`write_observed_fields`、`observed_tag`、`step_visual_replay_resolved`、`from_observed`、`bytes`、`elapsed` 等 |
| [ridge](../../src/resources/roofs/geometry/ridge.rs) | Recovered original RoofRidgeDims numeric algorithms. The byte interface keeps the observed ABI layout distinct from a claim about the original Rust type. | `RidgeDims`、`RidgeGeometry`、`RidgeError`、`ridge_dims_calc`、`ridge_dims_from_observed`、`calc_roof_tip_center_ls_observed`、`roof_tip_offset_xz_observed`、`inverse_ridge_01_param` 等 |
| [surface](../../src/resources/roofs/geometry/surface.rs) | Recovered from Tiny Glade's matched RNE/PDB, module 498. These functions preserve observed f32 evaluation order. Raw fields retain offsets until the original type layout is independently recovered. | `RoofShapeParams`、`RoofSection`、`CurveError`、`Curve2`、`write_observed`、`section`、`profile_exponent`、`get_roof_shape` 等 |
| [tiles](../../src/resources/roofs/geometry/tiles.rs) | Original Tiny Glade roof tile layout helpers recovered from RNE/PDB. No original Rust ABI is asserted; these are explicit numerical APIs. | `TileRng`、`CircularRowPlan`、`CircularTileInput`、`RoofTileRecord`、`ExpandedRoofBounds`、`ObservedRectangle`、`RectangularTileContext`、`next_f32` 等 |

<a id="resources-stairs"></a>


## resources/stairs

- **功能**：楼梯权威图、结构、备份和派生数据。
- **设计**：节点/边、dirty 边、checksum、备份标志及视觉/碰撞分离。
- **业务**：增删连接、恢复快照并失效下游楼梯组件。

| 源码模块 | 用途线索（来自源码） | 主要类型 / 接口 |
|---|---|---|
| [invalidate](../../src/resources/stairs/invalidate.rs) | `invalidate.rs` — 脏楼梯伪影失效系统（B `resources/stairs/invalidate.rs`，1 业务）。 | `invalidate_dirty_stair_artifacts` |
| [mod（本目录入口）](../../src/resources/stairs/mod.rs) | 未提供独立用途总述；通过下列源码类型/接口定位。职责按本节领域合同理解，不据文件名推断完成度。 | 内部实现 / 模块装配（见源码） |
| [stairs_assembly_params](../../src/resources/stairs/stairs_assembly_params.rs) | B stairs assembly geometry leaves. The full assembly-request owner and its graph-to-geometry producer are separate; no missing request fields default. | `StairWallAnchor`、`ObservedStairNodeAnchor`、`AlignKind`、`PreferredNodeAlignment`、`ObservedAlignKind`、`new`、`wall_normal`、`from_observed` 等 |
| [stairs_collider](../../src/resources/stairs/stairs_collider.rs) | `stairs_collider.rs` — 楼梯碰撞体注册标记资源（B `resources/stairs/stairs_collider.rs`， 1 胶水，无独立业务函数）。 | `StairsCollider` |
| [stairs_removed_supports](../../src/resources/stairs/stairs_removed_supports.rs) | `stairs_removed_supports.rs` — 已移除楼梯支撑存档集（B `resources/stairs/ stairs_removed_supports.rs`，1 业务全 serde 胶水）。 | `StairsRemovedSupports`、`record`、`erase`、`contains` |
| [stairs_state](../../src/resources/stairs/stairs_state.rs) | B authoritative player stairs graph (0x1408fe7b0..0x140901260). petgraph 0.8.1 owns the graph. The game's self-pair represents an *isolated* node, not an additional persistent edge table. +0x148 is only a dirty set. Original endpoint names/remaining payload… | `StairGraphEdge`、`TransientStairEdgeId`、`TransientStairEdgeIdAllocator`、`ObservedStairEndpoint`、`StairsSnapshot`、`StairsHistoryDiff`、`StairsState`、`new` 等 |
| [structural_graph](../../src/resources/stairs/structural_graph.rs) | `structural_graph.rs` — 楼梯支撑碰撞体图（B `resources/stairs/structural_graph.rs`， 7 业务 + 1 胶水）。 | `StairColliderHandle`、`StairsStructuralGraph`、`new`、`as_subworld`、`clear`、`upsert_span`、`upsert_support_source_collider`、`add_edge_by_span` 等 |
| [visual](../../src/resources/stairs/visual.rs) | `visual.rs` — 楼梯可视化资源（B `resources/stairs/visual.rs`，4 业务 + 6 胶水）。 | `MagicSupports`、`StairsEdgeVisual`、`StairsVisualState`、`StairsVisualComponent`、`clear`、`remove_edge`、`reset_edge`、`reset_edge_with` 等 |

<a id="resources-walls"></a>


## resources/walls

- **功能**：墙权威/公开状态和建筑附着数据。
- **设计**：内墙状态、变更集合、公开缓存、洞口和装饰存储分离。
- **业务**：创建/编辑/删除墙并发布给几何、屋顶、碰撞和历史消费者。

| 源码模块 | 用途线索（来自源码） | 主要类型 / 接口 |
|---|---|---|
| [arches](../../src/resources/walls/arches.rs) | 拱/过梁跨帧状态（walls/arches.rs）。 | `ArchSegment`、`ArchSegments`、`BuildingHadArchesLastFrame`、`LintelSegment`、`LintelSegments`、`ArchSegmentMarker` |
| [cached_decorator_transforms](../../src/resources/walls/cached_decorator_transforms.rs) | B `resources/walls/cached_decorator_transforms.rs` — 装饰变换缓存资源（原 11 函数： 业务 7）。 | `CachedDecoratorTransform`、`CachedDecoratorTransformStorage`、`CachedDecoratorTransforms`、`from_components`、`to_components`、`get`、`insert`、`clear` 等 |
| [curved_halftimber](../../src/resources/walls/curved_halftimber.rs) | 曲面半木构资源（walls/curved_halftimber.rs）。 | `CurvedHalfTimberDrawables` |
| [decorator](../../src/resources/walls/decorator.rs) | B `resources/walls/decorator.rs` — 装饰附着/信息类型族（原 131 函数：业务 56）。 | `DecoratorType`、`DecoratorId`、`DecoratorIdGen`、`TransientDecoratorIdGen`、`DecoratorRank`、`DecoratorLocator`、`ClutterVersion`、`DecoratorExtraInfo` 等 |
| [decorator_archivist](../../src/resources/walls/decorator_archivist.rs) | Shared archive owner; reuse the existing storage/restore implementation. | `DecoratorCullPolicy`、`decorator_cull_allow_any`、`is_decorator_cull_allowed`、`all_decorators`、`all_storages`、`decorator_cull_allow_wall`、`decorator_cull_allow_stairs`、`extract_diff` 等 |
| [decorator_storage](../../src/resources/walls/decorator_storage.rs) | B `resources/walls/decorator_storage.rs` — 装饰权威存储（原 114 函数：业务 60）。 | `DecoratorStorageAddr`、`DecoratorToStorageMap`、`DecoratorMaybeMut`、`Decorators`、`DecoMoveToStorage`、`DecoratorAddrSnapshot`、`DecoratorStorageSnapshot`、`DecoratorColliderRemover` 等 |
| [decorator_subtype](../../src/resources/walls/decorator_subtype.rs) | B `resources/walls/decorator_subtype.rs` — 装饰子类型网格库（原 49 函数：业务 20）。 | `DecoratorSubtype`、`IntoDecoratorSubtype`、`DecoratorLibKey`、`DecoratorCollision`、`DerivedDecoratorInfo`、`DerivedDecoratorSubtypes`、`DecoratorMeshLibrary`、`DecoratorSubtypes` 等 |
| [elevation_support_stats](../../src/resources/walls/elevation_support_stats.rs) | 升降支撑统计（walls/elevation_support_stats.rs）。 | `ElevationSupportStats`、`ElevationSupports` |
| [height_rules](../../src/resources/walls/height_rules.rs) | Game-owned wall height rules recovered from the supplied original RNE. Map/query lookup is deliberately represented by resolved game state, rather than by a local reimplementation of hashbrown/Bevy or their memory ABI. | `WallHeightBounds`、`HeightCacheError`、`wall_height_bounds`、`clamp_resolved_wall_height`、`cached_wall_max_y`、`cached_gable_max_y`、`flat_roof_y` |
| [inner_wall_state](../../src/resources/walls/inner_wall_state.rs) | Owned B wall authority. Field roles come from the InnerWalls update stores; this representation does not reproduce native Vec headers or Rust ABI. | `StructureType`、`FreehandHeightType`、`WallStyle`、`WallState`、`is_fence`、`is_wall`、`toggle`、`is_follow_terrain` 等 |
| [inner_walls](../../src/resources/walls/inner_walls.rs) | B InnerWalls ownership and notifications, using the real map/set dependency. New (+0x30), edited (+0x60) and removed (+0x90) IDs stay distinct. | `WallChanges`、`InnerWalls`、`InnerWallStateRecord`、`wall_id`、`wall_state_bytes`、`wall_state`、`height`、`freehand_type` 等 |
| [material_stats](../../src/resources/walls/material_stats.rs) | 墙材质统计（walls/material_stats.rs）。 | `MaterialStats`、`set_arches_segments_count`、`add_arches_length`、`add_gate_length`、`set_gates_segments_count` |
| [mod（本目录入口）](../../src/resources/walls/mod.rs) | B eraser multiplier read as one f32. Startup policy remains with its owner. Wall identity (B 0x140b34f20): zero is invalid. | `EraserSizeMultiplier`、`WallId`、`WallHighlight`、`HighlightColor`、`get_inner`、`is_valid`、`to_rgb`、`split_height_into_skirts` |
| [plaster](../../src/resources/walls/plaster.rs) | 石膏资源族（walls/plaster.rs）。 | `Plaster`、`AnimatedPlasterPeel`、`CreatePlasterCmd`、`DeletePlasterCmd`、`RecalculatePlasterCmd` |
| [public_wall_state](../../src/resources/walls/public_wall_state.rs) | Owned CPU fields of B PublicWallState. Container headers and GPU handles are not reconstructed. Publication, path preparation and height consumers share this record; its projection is only the previously identified byte fields. | `PublicWallState`、`collect`、`simplified_curve_2d`、`assert_valid`、`prepare_paths`、`shape`、`is_premade_shape`、`try_get_wall_shape` 等 |
| [public_walls](../../src/resources/walls/public_walls.rs) | Public wall ownership, associations and publication notifications. B's maps use hashbrown; their native layout is left to that dependency. | `OnWallChanged`、`OnWallDeleted`、`WallPublicationEvent`、`WallPublicationDelta`、`WallPublicationDirty`、`PublicWalls`、`WallEntities`、`WallPlasters` 等 |
| [shape_alignment](../../src/resources/walls/shape_alignment.rs) | Downstream hosts for the two alignment record streams written by the wall shape editors (fresh 2026-10-17). | `AlignKindMapEntry`、`WallShapeAlignmentWrites`、`decorator_ids`、`push_preferred_node_alignment`、`push_align_kind_entry` |
| [wall_holes](../../src/resources/walls/wall_holes.rs) | Actual hole ownership. Geometric finalization/hash policy is shared with the native-validated game leaf; no duplicate approximation of that algorithm. | `WallHoles`、`PrevWallHoles`、`wall_holes`、`add`、`extend`、`finalize`、`clear_for_next_frame` |
| [wall_path_segment](../../src/resources/walls/wall_path_segment.rs) | 墙路径段（walls/wall_path_segment.rs）。 | `WallPathSegment`、`pt_count`、`length_ws`、`length_ws_2d` |
| [wall_stone_columns](../../src/resources/walls/wall_stone_columns.rs) | 石柱碰撞存储（walls/wall_stone_columns.rs）。 | `StoneColumn`、`StoneColumnCollisionStorage`、`StoneColumnsChanged`、`clear_and_get_mut` |
| [wall_trigger_volume](../../src/resources/walls/wall_trigger_volume.rs) | B 0x1408eae40/0x1408e9ed0: wall quads and cached default collision triangles. Quad is 120 meaningful bytes, not the old report's guessed 96 bytes. The corner order is [bottom/start, bottom/end, top/start, top/end]. Freehand second faces reverse bottom/top s… | `Quad`、`WallTriggerError`、`WallTriggerVolume`、`is_valid`、`default_tris`、`triangle_normals`、`triangle_uvs`、`recompute` |
| [wooden_gates](../../src/resources/walls/wooden_gates.rs) | 木门表（walls/wooden_gates.rs）。 | `Gates`、`clear`、`for_wall_mut` |

<a id="startup"></a>
| [building_colors](../../src/resources/walls/color_selection.rs) | `create_rectangle::get_building_and_roof_colors` (0x140a94880) — the building/roof colour selection policy shared by the circle and rectangle creation systems. | `PublicWallColorHost`、`PublicWallColorSource`、`RoofSparseRecord`、`RoofPresenceBitset`、`RoofChunkEntry`、`RoofColorEntry`、`RoofChunk`、`RoofColorComponentTables` 等 |
| [wall_cache](../../src/resources/walls/height_cache.rs) | B init_wall_height_and_arch_segment_height: regular circle/rectangle cache. Positions are the published, resampled world outline, not raw control points. | `RegularWallHeightCache`、`CacheBuildError`、`regular_wall_height_cache`、`write_projection`、`gable_maximum` |
| [wall_collection](../../src/resources/walls/publication_source.rs) | Game-owned raw wall publication collection (B publisher/Curve remap). Containers, float operations and curve ownership reuse existing libraries. | `TerrainHeightGrid512`、`RawWallPublicationSource`、`CollectedWallPublication`、`CollectionError`、`DirtyPublicationSubscription`、`sample_at_world_xz`、`collect_raw_wall_publication`、`terrain_requires_republication` 等 |
| [wall_freeform_cache](../../src/resources/walls/freeform_height_cache.rs) | B's complete resolved tag&gt;=2 base-cache consumer. Input positions are the prepared Public Curve3 (+90/+98), not the initial terrain-mapped Curve3. Standard Vec/allocator storage is reused; segmentation ownership is external. | `FreeformWallHeightCache`、`FreeformCacheError`、`PreviewRectangleObserved`、`PreviewCircleObserved`、`ShapeHeightInputs`、`AppliedShapeHeight`、`ShapeHeightOutcome`、`ShapeHeightError` 等 |
| [wall_gable](../../src/resources/walls/gable_heights.rs) | Entire B regular-wall gable arch-height consumer after public-wall/roof lookup. Rectangle perimeter and the rebuilt XZ CurveU are game-owned algorithms; standard Vec, glam inputs, and powf are reused. No ECS/container ABI is ported. | `GableError`、`GableArchHeights`、`rectangle_point_u`、`apply_gable_arch_heights` |
| [wall_spatial_index](../../src/resources/walls/spatial_index.rs) | Game WallSpatialHash membership producer; std containers own storage. B's original bucket grid is 64 × 64 over the strict (-65,65) world bounds. | `WallSpatialIndex`、`world_space_to_grid_coord_unclamped`、`grid_coord_to_cell_index`、`cell_index_of_world_point`、`segment_cells`、`remove`、`replace_curve`、`wall_cells` 等 |
| [wall_state](../../src/resources/walls/publication_fields.rs) | Game fields read by B PublicWalls::get and update_roof_spatial. This is a projection of observed game state, not a hashbrown/Bevy ABI. | `WallId`、`WallReadError`、`WallShapeSnapshot`、`PublicWallBuildingState`、`validate_public_wall_read`、`from_observed`、`bytes`、`tag` 等 |


## startup

- **功能**：启动资产、会话和部分系统注册。
- **设计**：先资源/事件/资产，再系统；宿主提供设备及完整生命周期。
- **业务**：准备场景元数据、预制体、网格、纹理、音频和初始参数。

| 源码模块 | 用途线索（来自源码） | 主要类型 / 接口 |
|---|---|---|
| [init_photomode_state](../../src/startup/init_photomode_state.rs) | B PhotomodeState constructor (0x1408705b0) and enable reset (0x140870570). Offsets document native provenance; this type does not promise its Rust ABI. | `PhotomodeState`、`new`、`on_enable_photomode` |
| [load_glade_audio](../../src/startup/load_glade_audio.rs) | `load_glade_audio.rs` — glade 音频的常量（B `startup::load_glade_audio`，串池 0x142aa0c33 起，逐字节核对）。 | `audio_json_path`、`halloween_theme_flag`、`load_glade_audio` |
| [load_prefabs](../../src/startup/load_prefabs.rs) | `startup::load_prefabs` — 预制体资产载入与热重载（O：`crates/country-core/src/startup/load_prefabs.rs`）。 | `PrefabWatcher`、`new`、`watch_prefab`、`watch_atlas`、`take_changed`、`is_relevant_event`、`reload_prefabs`、`load_prefabs_impl` |
| [load_textures](../../src/startup/load_textures.rs) | `load_textures.rs` — 启动期纹理白名单（B `startup::load_textures::should_load`，0x1409e6950，汇编逐条核对）。 | `should_load` |
| [mod（本目录入口）](../../src/startup/mod.rs) | `country_core::startup` — 启动期生产者。只放已经对着 EXE 核对过的纯函数部分； 载入网格/着色器/实体生成的系统（`load_prefabs`、`startup_terrain` 本体等）依赖 ECS 与资产库，尚未做。 `startup_ivy` / `startup_garden_brush` 的网格注册链（路径/名字已对 B 图像 read_memory 核实）已实现； shader 与 spawn 记录依赖渲染管线，登记待接线。 | `load_shader_into_library`、`add_game_startup_systems` |
| [run_game](../../src/startup/run_game.rs) | B `startup/run_game` — 宿主可调的核心启动链（order 8 收口）。 | `new_session_on_load_commands`、`build_new_session_on_load`、`register_core_startup` |
| [startup_garden_brush](../../src/startup/startup_garden_brush.rs) | B `startup_garden_brush` 0x1415f87c0: per brush asset, in order — `load_json_as_mesh&lt;&str&gt;` → `load_mesh_into_library` → `load_shader_into_library&lt;&str&gt;` → `Commands::spawn_empty()` + one `insert` record. Two iterations; the second `insert` carr… | `GardenBrushHost`、`register_brush_mesh`、`startup_garden_brush`、`run_garden_brush` |
| [startup_ivy](../../src/startup/startup_ivy.rs) | B `startup_ivy` 0x140a21a30: loads the ivy branch/leaf/flower meshes into the asset mesh library. | `startup_ivy` |
| [startup_session](../../src/startup/startup_session.rs) | `startup_session.rs` — 新会话的起始存档目录与复制（B `startup::startup_session`）。 | `starting_build_dir`、`copy_dir_all`、`load_game_session` |
| [startup_terrain](../../src/startup/startup_terrain.rs) | `startup::startup_terrain::{grid_gen_cw, grid_gen_ccw}` — 地形网格的三角形索引。 | `grid_gen_cw`、`grid_gen_ccw` |

<a id="systems-audio"></a>


## systems/audio

- **功能**：领域音频与环境声音更新。
- **设计**：领域事件、音频实例/循环和 backend 分层。
- **业务**：墙/地形/装饰/动物/天气/UI 操作转换为播放和音量请求。

| 源码模块 | 用途线索（来自源码） | 主要类型 / 接口 |
|---|---|---|
| [audio_update](../../src/systems/audio/audio_update.rs) | B `audio_update.rs`: per-frame / boot audio update — push AudioSettings volumes to the 5 FMOD VCAs and the camera (or default) pose as the 3D listener to Studio. | `AudioSettings`、`CameraRotation`、`CameraPosition`、`EditingSpeed`、`camera_listener`、`audio_update_system`、`boot_audio_update_system` |
| [building_audio](../../src/systems/audio/building_audio.rs) | B `building_audio.rs`: building edit sounds — 7 loops + detach/attach/peak one-shots. | `BuildingAudioState`、`new`、`building_audio_init_system`、`building_audio` |
| [clutter_audio](../../src/systems/audio/clutter_audio.rs) | B `clutter_audio.rs`: clutter-placement sounds — 39-kind dispatch into `event:/SFX/Cosmetics/*` one-shots. | `clutter_audio_system` |
| [decorator_audio](../../src/systems/audio/decorator_audio.rs) | B `decorator_audio.rs`: decorator (window/lantern/chimney/flag/clutter) edit sounds — 16 loops driven by `PlayDecoratorSoundCmd`. | `DecoratorAudioState`、`new`、`decorator_audio_init_system`、`decorator_one_shot`、`decorator_audio_system` |
| [elevation_supports_audio](../../src/systems/audio/elevation_supports_audio.rs) | B `elevation_supports_audio.rs` 0x140ad51c0: elevated-shape supports (wooden brackets / brick / wooden columns) one-shot placement sounds. | `elevation_supports_audio` |
| [firstperson_steps](../../src/systems/audio/firstperson_steps.rs) | B `firstperson_steps.rs`: first-person footstep sounds — the walking loop, the `FirstPerson` / `GroundType` global parameters and the terrain-raster material sampling. | `FirstPersonViewAudioState`、`PlayerMoving`、`new`、`firstperson_steps_init_system`、`firstperson_steps` |
| [forest_audio](../../src/systems/audio/forest_audio.rs) | B `forest_audio.rs`: forest-growth sound — a single `event:/SFX/ForestGrow` loop driven by the presence of interactable forest trees. | `ForestTree`、`ForestAudioState`、`new`、`forest_audio_init_system`、`forest_audio_system` |
| [game_loading_audio](../../src/systems/audio/game_loading_audio.rs) | B `game_loading_audio.rs`: loading-screen audio — `snapshot:/Loading` snapshot + `event:/UI/Loading` one-shots. | `GameLoadingAudioState`、`new`、`init_game_loading_audio`、`loading_audio` |
| [garden_audio](../../src/systems/audio/garden_audio.rs) | B `garden_audio.rs`: garden draw/erase sounds — 4 loops driven by the native 4-loop gate + volume modulation (decomp-0x140a89480, O). | `GardenAudioState`、`new`、`garden_audio_init_system`、`garden_loop_flags`、`garden_path_size`、`update_draw_audio_volume`、`garden_audio_system` |
| [ivy_audio](../../src/systems/audio/ivy_audio.rs) | B `ivy_audio.rs` 0x140ad5350: a single ivy-growth sound, played once per growth trigger. | `ivy_audio_init_system`、`ivy_audio_system` |
| [menu_audio](../../src/systems/audio/menu_audio.rs) | B `menu_audio.rs`: pause-menu snapshot + 13 menu UI one-shots. | `MenuAudioState`、`new`、`menu_audio_init_system`、`menu_audio` |
| [mod（本目录入口）](../../src/systems/audio/mod.rs) | B `systems/audio/mod.rs`: audio-domain event types, the async loaded-audio package and the two registration entry points (`add_audio_events` / `add_audio_systems`). | `PlayWallAudio`、`PlayBuildingAudio`、`PlayStairsAudioCmd`、`PlayClutterSoundCmd`、`PlaySheepReactSound`、`PlayMenuUiSound`、`PlayTreeSoundCmd`、`PlayReplayAudio` 等 |
| [path_audio](../../src/systems/audio/path_audio.rs) | B `path_audio.rs`: path / cobblestone draw sounds — CobblestoneDraw loop + 4 snappy loops, driven by the shared `update_draw_audio` template. | `PathDrawEvent`、`PathEraseEvent`、`PathEditEvent`、`PathAudioState`、`new`、`path_audio_init_system`、`path_audio_system` |
| [playermade_stairs_audio](../../src/systems/audio/playermade_stairs_audio.rs) | B `playermade_stairs_audio.rs`: player-made stairs sounds — 6 loops + kind ≤ 18 dispatch. | `StairsAudioState`、`new`、`stairs_audio_init_system`、`stairs_audio_system` |
| [replay_audio](../../src/systems/audio/replay_audio.rs) | B `replay_audio.rs`: undo/redo/deny-input sounds. | `ReplayAudioState`、`new`、`replay_audio_init_system`、`replay_audio_system` |
| [roof_audio](../../src/systems/audio/roof_audio.rs) | B `roof_audio.rs`: roof edit sounds — 3 loops (RoofShape / RoofTipMove / InterRoofMerlonsRearrange) plus the two one-shot helpers shared with the wall audio systems (`instantiate_audio_w_position[_and_volume]`). | `RoofEditSound`、`RoofAudioState`、`instantiate_audio_w_position`、`instantiate_audio_w_position_and_volume`、`new`、`roof_audio_init_system`、`roof_audio_system` |
| [sheep_audio](../../src/systems/audio/sheep_audio.rs) | B `sheep_audio.rs`: sheep reaction sounds — kind 0..7 dispatch. | `SheepAudioState`、`new`、`sheep_audio_init_system`、`sheep_audio_system` |
| [signifier_audio](../../src/systems/audio/signifier_audio.rs) | B `signifier_audio.rs`: UI signifier stream sounds — 1 loop + 4 one-shots. | `SignifierAudioState`、`new`、`signifier_audio_init_system`、`signifier_audio_system` |
| [terrain_audio](../../src/systems/audio/terrain_audio.rs) | B `terrain_audio.rs`: terrain edit sounds — 4 loops + the terrain edit event dispatch. | `TerrainAudioState`、`new`、`terrain_audio_init_system`、`terrain_audio_system` |
| [terrain_stairs_audio](../../src/systems/audio/terrain_stairs_audio.rs) | B `terrain_stairs_audio.rs`: terrain-stair draw sounds — a `event:/SFX/StairDraw_OneShot` ring plus per-stroke one-shot instances. | `TerrainStairsAudioEvent`、`StairsRasterValue`、`StairsCounter`、`TerrainStairsAudioState`、`new`、`terrain_stairs_audio_init_system`、`terrain_stairs_audio_system` |
| [timberframe_audio](../../src/systems/audio/timberframe_audio.rs) | B `timberframe_audio.rs`: timberframe edit sounds. | `TimberframeAudioState`、`timberframe_one_shot`、`new`、`timberframe_audio`、`timberframe_audio_init_system` |
| [tree_audio](../../src/systems/audio/tree_audio.rs) | B `tree_audio.rs`: tree edit / ambience sounds — 4 loops + TreePlace / TreeDelete one-shots. | `TreeAmbience`、`TreeAudioState`、`new`、`tree_audio_init_system`、`tree_audio_system` |
| [trigger_audio](../../src/systems/audio/trigger_audio.rs) | B `trigger_audio.rs`: audio trigger boxes — camera movement segment × `AudioTriggerLibrary` intersection drives the camera rustle / wind / whoosh and the InForest snapshot. | `TriggerAudioState`、`new`、`trigger_audio_init_system`、`trigger_audio_system` |
| [wall_audio_fmod](../../src/systems/audio/wall_audio_fmod.rs) | B `wall_audio_fmod.rs`: the wall audio main system — 10 loops + kind 0..23 dispatch + frame-end start/stop. | `WallAudioState`、`new`、`wall_audio_init_system`、`wall_audio_fmod`、`inverse_lerp` |
| [wall_audio_orchestrator](../../src/systems/audio/wall_audio_orchestrator.rs) | B `wall_audio_orchestrator.rs`: wall material audio coordination. | `WallMaterialChange`、`WallMaterialAudioHost`、`wall_audio_orchestrator` |
| [wall_audio_path](../../src/systems/audio/wall_audio_path.rs) | B `wall_audio_path.rs`: wall-path audio producer. | `WallPathStreamEventA`、`WallPathStreamEventB`、`WallPathIdEvent`、`WallPathAudioEntry`、`WallPathAudioOut`、`wall_audio_path` |
| [wall_audio_ui](../../src/systems/audio/wall_audio_ui.rs) | B `wall_audio_ui.rs`: wall UI sounds — `PlayWallUiSound` kind dispatch into positioned one-shots (with the parameterized kinds 6/0xd/0xe/0x10). | `wall_audio_ui` |
| [wall_audio_utils](../../src/systems/audio/wall_audio_utils.rs) | B `wall_audio_utils.rs`: the wall loop player (`AudioLoopPlayer`). | `new_player` |
| [water_ambience_audio](../../src/systems/audio/water_ambience_audio.rs) | B `water_ambience_audio.rs`: water ambience — per-water-grid-cell Amb loops. | `WaterGridCell`、`WaterAudioGrid`、`WaterAmbienceAudio`、`load_index_unchecked`、`new`、`water_ambience_audio_init_system`、`water_ambience_audio_system` |
| [water_audio](../../src/systems/audio/water_audio.rs) | B `water_audio.rs`: water draw / erase / ice sounds — 4 loops. | `WaterAudioState`、`new`、`water_audio_init_system`、`water_audio_system` |
| [weather_audio](../../src/systems/audio/weather_audio.rs) | B `weather_audio.rs`: weather/season global audio parameters. | `Weather`、`weather_audio_system` |

<a id="systems-audio-music"></a>
| [audio](../../src/systems/audio/wall_kinds.rs) | country-core systems/audio：wall_audio_fmod 的 PlayWallAudio kind 0–23 逐条表（B 主基线，反汇编级证据草稿） | `WallKind`、`wall_kind`、`snap_off_boundaries`、`advance_new_glade_time` |
| [water](../../../systems/water/src/water_audio_grid/bit_projection.rs) | system-water CPU leaves: rolling audio bits and resolved raster-write callbacks. No world-raster constructor, guessed sampling grid, raycast or zeno replacement. | `WriteMode`、`frame_bit`、`apply_audio_bit`、`select_write_mode`、`coverage_to_alpha`、`brush_fixed`、`apply_write` |


## systems/audio/music

- **功能**：音乐播放器、曲库、队列和节拍更新。
- **设计**：状态机和随机队列与 AudioCore/native backend 分离。
- **业务**：加载场景曲目、切换、平滑、节拍响应及调试。

| 源码模块 | 用途线索（来自源码） | 主要类型 / 接口 |
|---|---|---|
| [audio_core_backend](../../src/systems/audio/music/audio_core_backend.rs) | `impl MusicBackend for AudioCore&lt;S&gt;` — SPEC §2.4 方案 A（后端收敛）。 | `AudioCoreEvent` |
| [glade_audio](../../src/systems/audio/music/glade_audio.rs) | `GladeAudio` — glade 音频描述（B `startup::load_glade_audio::GladeAudio`， 72 B = 9 qword）。 | `GladeAudio`、`load` |
| [load_glade_audio](../../src/systems/audio/music/load_glade_audio.rs) | `load_glade_audio` — glade 音频 producer（B `startup::load_glade_audio` 系统 0x1408be360）。 | `load_glade_audio`、`configure_glade_audio` |
| [mod（本目录入口）](../../src/systems/audio/music/mod.rs) | 未提供独立用途总述；通过下列源码类型/接口定位。职责按本节领域合同理解，不据文件名推断完成度。 | 内部实现 / 模块装配（见源码） |
| [music_debug_gui](../../src/systems/audio/music/music_debug_gui.rs) | B 0x14083c4a0 and window closure 0x14083b480, vtable @0x142a93400. | `music_debug_gui` |
| [music_player](../../src/systems/audio/music/music_player.rs) | B 0x1408b5500–0x1408b7681: music ownership, playlist and transition policy. | `StopMode`、`MusicEvent`、`MusicBackend`、`MusicInstance`、`PlayerState`、`MusicPlayer`、`TimelineBeat`、`TimelineSnapshot` 等 |
| [music_system](../../src/systems/audio/music/music_system.rs) | B 0x140a71f20 / 0x140a72810. The PDB parameters distinguish GameTime from bevy Time&lt;()&gt;, and EscMenuState from AppMode. | `MusicAudioState`、`init`、`music_audio_init_system`、`configure_music_consumers`、`preview_glade_parameter`、`music_audio_system`、`update_music_audio` |
| [native](../../src/systems/audio/music/native.rs) | Real adapter for the original Cargo.lock libfmod 2.222.6. The caller supplies its initialized Studio (with banks loaded) and retains responsibility for Studio shutdown. This module does not simulate playback or missing banks. | `FmodMusicBackend`、`FmodMusicEvent` |

<a id="systems-backend"></a>
| [audio](../../src/systems/audio/music/parameters.rs) | country-core systems/audio：wall_audio_fmod 的 PlayWallAudio kind 0–23 逐条表（B 主基线，反汇编级证据草稿） | `WallKind`、`wall_kind`、`snap_off_boundaries`、`advance_new_glade_time` |


## systems/backend

- **功能**：平台交互到游戏系统的适配。
- **设计**：输入、GUI、拍照与日志分别连接宿主。
- **业务**：接收设备输入并呈现菜单/拍照界面和屏幕日志。

| 源码模块 | 用途线索（来自源码） | 主要类型 / 接口 |
|---|---|---|
| [flush_screen_log](../../src/systems/backend/flush_screen_log.rs) | B's thread-safe producer buffer and main-thread screen-log consumer. | `log_error_to_screen`、`log_warning_to_screen`、`flush_error_log_to_screen` |
| [gui](../../src/systems/backend/gui.rs) | GUI resource consumers. The renderer supplies an egui frame/context; these systems use the same DebugGui resource as the existing draw callbacks. | `publish_pending_egui_context`、`show_screen_log`、`show_debug_controls`、`configure_gui_consumers` |
| [mod（本目录入口）](../../src/systems/backend/mod.rs) | 未提供独立用途总述；通过下列源码类型/接口定位。职责按本节领域合同理解，不据文件名推断完成度。 | 内部实现 / 模块装配（见源码） |
| [photomode](../../src/systems/backend/photomode.rs) | Independently recovered photomode consumers. Camera, audio, focus readback and widget dependencies of the large original do_photomode remain separate. | `update_photomode_visibility`、`configure_photomode_visibility` |

<a id="systems-backend-input"></a>


## systems/backend/input

- **功能**：输入翻译、增量、光标与录制回放。
- **设计**：原始事件→动作状态→光标事件→帧末重置；回放可替换输入源。
- **业务**：让键鼠、Steam 和录制数据驱动相同编辑行为。

| 源码模块 | 用途线索（来自源码） | 主要类型 / 接口 |
|---|---|---|
| [button_input](../../src/systems/backend/input/button_input.rs) | `button_input.rs` — 键鼠按钮输入抽象（crate 无 `bevy_input` 依赖的等价层）。 | `KeyButtonInput`、`MouseButtonInput`、`pressed`、`just_pressed`、`reset_all`、`just_pressed_any`、`just_released` |
| [calculate_input_delta](../../src/systems/backend/input/calculate_input_delta.rs) | B 0x1416feeb0: bool input edges and held/repeating digital actions. The old chapter called the 0.1/0.05 branch an analog deadzone; native code applies it to DigitalActionState tag 2. Analog values are copied unchanged. | `UiHistoryButtonPressed`、`is_redo`、`digital_delta`、`uses_ui_auto_repeat`、`update_action_states`、`calculate_input_delta` |
| [input_record_replay](../../src/systems/backend/input/input_record_replay.rs) | `input_record_replay.rs` — 录制/回放状态机（B 0x1409423d0 / 0x140941a30 / 0x140942280）。 | `InputReplayCommand`、`InputFrame`、`InputFrameSerde`、`RecordingFile`、`InputReplayInitialState`、`InputRecordReplayState`、`InputReplayOutputState`、`InputTranslationStateResource` 等 |
| [input_replay_for_trailers](../../src/systems/backend/input/input_replay_for_trailers.rs) | `input_replay_for_trailers.rs` — trailer 专用输入录制/回放（B 0x14083c620 / 0x14083cdd0， O）。 | `InputReplayForTrailers`、`record_and_replay_input_for_trailers`、`load_camera_parameters`、`do_input_replay_ui` |
| [mod（本目录入口）](../../src/systems/backend/input/mod.rs) | `systems::backend::input` — 后端输入链：动作增量、光标合成、录制/回放状态机与键盘映射。 | `register_input_events`、`add_input_cpu_consumers` |
| [mod_key](../../src/systems/backend/input/mod_key.rs) | `mod_key.rs` — 修饰键状态与吸附禁用开关（B `systems::mod_key`）。 | `Modifiers`、`ModKeyConfig`、`disable_snapping`、`is_disable_snapping_just_pressed`、`update_mod_key_state` |
| [reset_input_state](../../src/systems/backend/input/reset_input_state.rs) | `reset_input_state.rs` — 输入状态复位（B 0x140942b00，O）。 | `reset_input_state` |
| [translate_input](../../src/systems/backend/input/translate_input.rs) | `translate_input.rs` — 输入映射主系统（B 0x1417cd0a0，O 全解）。 | `MouseMoved`、`SteamInputActive`、`DemoModeEnabled`、`ActionBinding`、`BindingsTable`、`MouseWheel`、`bound_key`、`translate_input` |
| [translate_input_to_cursor_moved](../../src/systems/backend/input/translate_input_to_cursor_moved.rs) | B 0x140b17a80. Real EventReader order, per-event clamping and separate input-space/window-space deltas. | `InputCursorMoved`、`InputCursorDelta`、`fold_cursor_events`、`translate_input_to_cursor_moved` |
| [update_cursor_position](../../src/systems/backend/input/update_cursor_position.rs) | `update_cursor_position.rs` — 游标位置合并（B PDB 签名 @0x1413ee760，正文全内联）。 | `CursorPosition`、`new`、`as_vec2`、`fold_cursor_moved`、`fold_cursor_state`、`update_cursor_position` |

<a id="systems"></a>


## systems

- **功能**：领域更新系统。
- **设计**：消费动作/事件，读写权威资源，发布派生结果。
- **业务**：执行编辑、历史、保存、模式、相机、昼夜和世界更新。

| 源码模块 | 用途线索（来自源码） | 主要类型 / 接口 |
|---|---|---|
| [brush_preview](../../src/systems/brush_preview.rs) | `brush_preview.rs` — 笔刷预览（B 0x140865050，O）。 | `BrushPreview`、`OverwriteBrushPreview`、`brush_size`、`brush_preview`、`register_required_components` |
| [collision](../../src/systems/collision.rs) | `collision.rs` — 碰撞世界的已锚定常量（B `systems::collision::world`）。 | `SubworldIndex`、`BirdNestCandidateLocation`、`BirdNestCandidateLocations`、`MeshIntersectionArtifacts`、`MeshIntersectionGraph`、`CollisionPlugin`、`from_usize_unchecked`、`slot` 等 |
| [history](../../src/systems/history.rs) | B draw_raster emits this payload-free event before the tool's history callback. Event consumption remains with the history processor. | `DebugHistoryMsg`、`PendingHistoryCommands`、`HistoryControlRecord`、`PendingHistoryControl`、`HistoryControlProcessed`、`HistoryReplayFailed`、`ReplayBucketFinished`、`HistoryCpuStage` 等 |
| [ivy](../../src/systems/ivy.rs) | `ivy.rs` — 常春藤几何：`closest_segment_index`（B 0x14094c800，666 B，反编译 + 汇编逐条核对）。 | `WallSegment`、`ClosestSegment`、`ParameterOutOfRange`、`closest_segment_index` |
| [main_camera](../../src/systems/main_camera.rs) | `country_core::systems::main_camera`（B 原模块路径）。 | `CameraRotationPivot` |
| [mode_manager](../../src/systems/mode_manager.rs) | B mode manager: complete input ordering and toolbar transitions from original 0x140a86b60..0x140a875d7; thirteen five-entry ActionState jump tables. History::is_recording 0x140a86970 is the guard (not an event-queue length). Event/audio/history payload owne… | `RequestAppModeChange`、`RequestToolbarCategoryChange`、`LoadingGameComplete`、`RestoreAppTo`、`HistoryCommand`、`RequestHistoryCommand`、`UiHistoryPlaySelectAnimation`、`ClearCanvasCmd` 等 |
| [paths](../../src/systems/paths.rs) | `paths.rs` — 路径绘制状态（B `systems::paths::DrawState::update`，0x140ae4bd0，汇编逐条核对）。 | `DrawState`、`PathRastersResources`、`SetSizeBasedOnAppModePath`、`PathPinnedClutterCircle`、`PathRasterResources`、`PathRasterSet`、`PathEventWriters`、`RecordPathEditToHistoryCmd` 等 |
| [prefab](../../src/systems/prefab.rs) | prefab.rs — 预制体实例/生成/更新/回收（B `systems/prefab`，fresh 反编译 2026-10-17）。 | `PrefabTransform`、`CullInstanceSource`、`GpuInstanceRecord`、`AudioTriggerRecord`、`PrefabInstanceData`、`SpawnPrefabCmd`、`DespawnPrefabCmd`、`PrefabSpawnRequest` 等 |
| [time_of_day](../../src/systems/time_of_day.rs) | `systems/time_of_day.rs` — 昼夜主系统（B 0x140a73460）。 | `TodLightingSeam`、`advance_angle`、`advance_hour`、`advance_playback`、`advance_tod_hour`、`sun_position_at`、`suncalc_pos_to_vec`、`time_of_day` |
| [ui_edit_focus](../../src/systems/ui_edit_focus.rs) | `systems/ui_edit_focus.rs` — GeoSource → EditSubject 映射（B 原版函数体还原）。 | `GeoSource`、`from_hit_handle`、`tag`、`payload_u32`、`payload_u64`、`payload_u64_hi`、`geo_src_to_edit_subject`、`geo_src_to_edit_subject_w_deco_clutter` |
| [weather_system](../../src/systems/weather_system.rs) | `weather_system.rs` — 冬季降雪强度（B 0x1408e5b00，全证）。 | `SnowIntensity`、`weather_system` |
| [world_raster](../../src/systems/world_raster.rs) | B 0x140957bf0 consumes upload requests through its EventReader cursor. Native global scheduling and history command producers are tracked separately. | `RasterStrokeCmd`、`RasterDrawResources`、`record_raster_history`、`draw_raster`、`upload_world_raster_buffers_to_staging` |

<a id="systems-camera-rig"></a>


## systems/camera_rig

- **功能**：主相机/自由相机和 normal body。
- **设计**：dolly rig、模式参数、视图与投影分开。
- **业务**：移动和切换视角，保存/恢复相机参数。

| 源码模块 | 用途线索（来自源码） | 主要类型 / 接口 |
|---|---|---|
| [free_camera](../../src/systems/camera_rig/free_camera.rs) | `systems::camera_rig::free_camera` — Free 模式相机（dolly `CameraRig` 加三个访问器）。 | `FreeCamera`、`match_normal`、`get_position`、`get_yaw_pitch`、`update` |
| [main_camera](../../src/systems/camera_rig/main_camera.rs) | `systems::camera_rig::main_camera` — `MainCamera`（0x4e0 B 资源）的已核对部分。 | `CameraMode`、`CameraRigParameters`、`MainCameraMoved`、`ActiveRotation`、`MainCamera`、`new`、`previous`、`to_bytes` 等 |
| [mod（本目录入口）](../../src/systems/camera_rig/mod.rs) | Only the closed camera-schema leaf is included. Camera mode controllers, projection adapters and the incomplete standalone camera package are excluded. `main_camera` / `free_camera` hold the pieces of `MainCamera` verified against the B EXE (see `R9_camera.…` | `CameraRigParametersCache`、`CameraModeStates`、`CameraBodyVtable`、`CurrentDyn`、`CameraState`、`default_vertical_fov_degrees`、`current_dyn`、`current_body_offset` 等 |
| [normal_body](../../src/systems/camera_rig/normal_body.rs) | `normal_body.rs` — `NormalCamera` body（0xb8 B，位于 `MainCamera + 0x100 / 0x1b8 / 0x390`）的已核对运算。 | `ActiveRotation`、`ZoomBody`、`ZoomClampPanic`、`NormalCameraBody`、`for_mode`、`zoom`、`rotate_yaw_pitch`、`constrain_to_be_above_terrain` |

<a id="systems-collision"></a>


## systems/collision

- **功能**：碰撞世界、相交图和调试显示。
- **设计**：分槽 registry + Rapier/parry 查询；网格相交算法与显示分开。
- **业务**：更新 collider、拾取、球扫、生成相交折线；native-physics 控制实际物理查询。

| 源码模块 | 用途线索（来自源码） | 主要类型 / 接口 |
|---|---|---|
| [intersect_meshes](../../src/systems/collision/intersect_meshes.rs) | `systems/collision/intersect_meshes.rs` — 网格对网格相交（B `intersect_meshes`， LOGIC.md 未实现登记段：21 函数，业务 4）。 | `VertexId`、`PolylinePoint`、`Polyline`、`Segment`、`should_intersect_collider_meshes`、`intersect_mesh_mesh`、`find_mesh_mesh_intersections`、`intersect_triangle_pair` 等 |
| [visualize_colliders](../../src/systems/collision/visualize_colliders.rs) | `systems/collision/visualize_colliders.rs` — collider 调试可视化（B `visualize_colliders`， LOGIC.md 未实现登记段：6 函数，业务 2）。 | `DebugDrawRequest`、`visualize_colliders`、`visualize_shape` |
| [world](../../src/systems/collision/world.rs) | `systems/collision/world.rs` — rapier3d 碰撞世界（B 170 函数，核心查询族 O）。 | `SubworldColliderHandle`、`RaycastWorldResult`、`RaycastSimpleResult`、`ColliderDiff`、`ColliderDelta`、`RaycastMeshData`、`RaycastGeoSource`、`RaycastMesh` 等 |

<a id="systems-debug"></a>
| [collision_query_policy](../../src/systems/collision/query_policy.rs) | Original17-subworld query dispatch policy, implemented without validation. Actual intersection operations belong to Rapier/Parry. Geometry/source maps for selected subworlds are required; missing ones are never assumed empty. | `CollisionRay`、`RaycastGeoSourceProjection`、`SubworldRayHit`、`CollisionRayHit`、`CollisionQueryGap`、`CollisionSubworldQueries`、`tag`、`platform_stair` 等 |


## systems/debug

- **功能**：调试控制。
- **设计**：独立 Resource/系统，不能代替正常游戏状态机。
- **业务**：当前主要用于羊状态捕获和控制。

| 源码模块 | 用途线索（来自源码） | 主要类型 / 接口 |
|---|---|---|
| [mod（本目录入口）](../../src/systems/debug/mod.rs) | `systems::debug` — 调试域（中央仲裁批准 2026-10-16 落 sheep_controls）。 | 内部实现 / 模块装配（见源码） |
| [sheep_controls](../../src/systems/debug/sheep_controls.rs) | `systems/debug/sheep_controls.rs` — B 真实 Sheep query 所有者。 | `SheepControl`、`sheep_controls`、`apply_sheep_snapshot` |

<a id="systems-garden"></a>


## systems/garden

- **功能**：花园绘制和历史生产。
- **设计**：画笔/模式/边界约束输入，绘制与记录分开。
- **业务**：绘制花园区域并生成撤销所需记录。

| 源码模块 | 用途线索（来自源码） | 主要类型 / 接口 |
|---|---|---|
| [draw_garden](../../src/systems/garden/draw_garden.rs) | B 0x140ae4620, checked against original RNE instructions. | `GardenRasterResources`、`draw_garden` |
| [mod（本目录入口）](../../src/systems/garden/mod.rs) | B garden drawing and recording, connected to the shared raster and history owners. Original global plugin schedule and pin rendering are not implied. | `GardenDrawEvent`、`GardenEraseEvent`、`GardenPinnedClutterCircle`、`GardenCpuStage`、`route_garden_replay_feedback`、`register_garden_events`、`add_garden_cpu_systems`、`build` |
| [record_garden_history](../../src/systems/garden/record_garden_history.rs) | B 0x141695e10 consumes all pending notifications and records slot five once. | `RecordGardenEditToHistoryCmd`、`record_garden_history` |

<a id="systems-history"></a>


## systems/history

- **功能**：历史录制、结束回放和各域回放消费者。
- **设计**：记录/回放状态分离；wall/stairs/raster/decorator/color/clutter/mode 各自消费命令。
- **业务**：撤销或重做时将历史编辑应用到对应权威状态，完成后退出回放。

| 源码模块 | 用途线索（来自源码） | 主要类型 / 接口 |
|---|---|---|
| [finish_replay](../../src/systems/history/finish_replay.rs) | B `systems/history/finish_replay.rs` — 回放自然结束（RE-HISTORY-WRAPUP §3，O）。 | `is_replay_naturally_finished`、`finish_replay` |
| [recording](../../src/systems/history/recording.rs) | Roof recording writes the same History owner used by mode_manager and replay. UI record notifications are separate from live edit commands so replaying an edit does not record another copy of itself. | `RecordRoofVisualHistory`、`RecordRoofDirectionHistory`、`HistoryRecordingFailed`、`record_roof_history`、`record_roof_visual_history`、`record_roof_direction_history`、`register_recording_events`、`history_finish_recording` |
| [replay_auto_clutter](../../src/systems/history/replay_auto_clutter.rs) | B 0x140aa4a60. Only matching commands in the front mixed bucket advance. | `advance_auto_clutter`、`replay_auto_clutter` |
| [replay_color](../../src/systems/history/replay_color.rs) | B `systems/history/replay_color.rs` — 颜色回放命令消费（结构模型）。 | `ColorHistoryReplayApply`、`replay_color` |
| [replay_decorator](../../src/systems/history/replay_decorator.rs) | B `systems/history/replay_decorator.rs` — 装饰回放命令消费（结构模型）。 | `DecoratorHistoryReplayApply`、`replay_decorator` |
| [replay_mode](../../src/systems/history/replay_mode.rs) | B `systems/history/replay_mode.rs` — 回放模式进出检测 + HoldToFastReplay （fresh 0x140a7c550 逐段；含 `time_continuous_lerp` 0x140c904e0）。 | `ReplayFocusEvent`、`ReplayFastHoldInput`、`HoldToFastReplay`、`is_replay_mode`、`replay_mode_transition`、`time_continuous_lerp`、`nan_aware_max`、`engaged` 等 |
| [replay_raster](../../src/systems/history/replay_raster.rs) | B 0x1409f4650 and its five sibling instances. Undo rebuilds the composite from the shared history, stopping at Arc identity, then animates the remaining prefix in scratch. No second history ledger or saved fake pixel snapshot. | `RasterReplayParams`、`RasterDomain`、`RasterReplayCmd`、`RasterReplayDirty`、`RasterReplayScratch`、`RasterReplayFeedback`、`get_param`、`new` 等 |
| [replay_stairs](../../src/systems/history/replay_stairs.rs) | `replay_stairs.rs` — 楼梯回放系统（B `systems/history/replay_stairs.rs`，1 业务）。 | `replay_stairs` |
| [replay_wall](../../src/systems/history/replay_wall.rs) | Original wall replay groups applied to the one live wall authority. | `WallHistoryReplayAudio`、`WallHistoryRestoreColor`、`register_wall_replay_events`、`replay_wall` |

<a id="systems-ivy"></a>
| [wall_history_replay](../../src/systems/history/wall_replay_interpreter.rs) | B timed new-wall state growth and original internal concurrent-group boundary. Whole native process proof is kept in evidence/building-wall-history. | `WallReplayError`、`WallReplayCommand`、`TimedWallReplayOutcome`、`WallConcurrentReplayOutcome`、`advance_wall_groups`、`advance_wall_groups_observed`、`expand_wall_compound`、`advance` |
| [wall_history_transition](../../src/systems/history/wall_replay_transition.rs) | B additional StateChange operation branches; ABI proof is separate from the already accepted new-wall producer and timed operation0 implementation. | `advance_wall_transition` |


## systems/ivy

- **功能**：常春藤生成、生长、修剪、枝叶动画和上传准备。
- **设计**：株/叶资源分离，系统按生成→生长→动画→上传组织。
- **业务**：消费墙面坐标和方向提议，维护枝叶生命周期并发布 GPU 数据。

| 源码模块 | 用途线索（来自源码） | 主要类型 / 接口 |
|---|---|---|
| [ivy_branch_thickness_animation](../../src/systems/ivy/ivy_branch_thickness_animation.rs) | B 0x140995a80. This is point-count based; it does not read GameTime. | `animate_branch_thickness`、`ivy_branch_thickness_animation` |
| [ivy_grower](../../src/systems/ivy/ivy_grower.rs) | B `ivy_grower` 0x14094a910: the 10-phase state machine whose growth closure (guard body 0x14094ac51) advances pending segments and proposes new wall-anchored segments. Completed segments feed the leaf spawner through `IvySegmentDoneGrowingMsg`. | `GrowthProposal`、`GrowerState`、`advance_growth`、`start_growth_segment`、`get_wall_double_sided_normal`、`wall_segment_from_spatial`、`wall_length`、`wall_arch_height` 等 |
| [ivy_leaf_animation](../../src/systems/ivy/ivy_leaf_animation.rs) | B 0x140b0e380: delayed growth and endpoint-thickness propagation. | `IvyLeafIsGrowing`、`ivy_leaf_animation` |
| [ivy_leaf_spawner](../../src/systems/ivy/ivy_leaf_spawner.rs) | B 0x140996080; business body 0x1409964f6..0x140996e9a. | `IvySegmentDoneGrowingMsg`、`spawn_segment_leaf`、`ivy_leaf_spawner` |
| [ivy_pruner](../../src/systems/ivy/ivy_pruner.rs) | Location command consumer B 0x140999330, producer B 0x140999860. The separate wall-change validity pruner (0x140997410) re-validates each ivy's tail/root against the wall spatial hash plus the arch height band. | `RemoveIvyCmd`、`at_location`、`remove_ivy_at_location`、`apply_remove_ivy_cmd`、`ivy_pruner_cmd`、`is_point_still_valid`、`ivy_pruner` |
| [ivy_send_to_gpu_leaf](../../src/systems/ivy/ivy_send_to_gpu_leaf.rs) | B `ivy_send_to_gpu_leaf` 0x1408c1580: collect all ivy leaf instances and upload 48-byte GPU rows to the IvyLeafSsbo / IvyFlowerSsbo buffers. | `IvyLeafGpu`、`IvyFlowerGpu`、`pack_leaf`、`pack_flower`、`upload_ivy_flowers`、`upload_ivy_leaves` |
| [ivy_send_to_gpu_segments](../../src/systems/ivy/ivy_send_to_gpu_segments.rs) | B `ivy_send_to_gpu_segments` 0x140a9b4e0: collect every ivy growth segment and upload 48-byte GPU rows to the IvySegmentSsbo buffer. | `IvySegmentGpu`、`pack_ivy_segments`、`upload_ivy_segments` |
| [ivy_spawner](../../src/systems/ivy/ivy_spawner.rs) | B `ivy_spawner` 0x14094cf90: root spawning — a cursor-root pass and a random-root pass, both validated against the nearest wall segment before `IvyStorage::spawn_ivy` (0x1408696f0) commits the root. | `is_point_inside_wall_segment`、`try_ivy_root_next_to_cursor`、`try_random_ivy_root`、`ivy_spawner` |
| [registration](../../src/systems/ivy/registration.rs) | Local CPU animation registration; global growth/spawn scheduling is separate. | `register_ivy_storage`、`insert_ivy_resources`、`add_ivy_events`、`add_ivy_animation_systems`、`add_ivy_cpu_systems`、`add_ivy_systems` |

<a id="systems-mode-manager"></a>


## systems/mode_manager

- **功能**：模式管理器的类型合同。
- **设计**：类型和状态表达从模式转换逻辑分离。
- **业务**：为主模式管理文件提供一致的模式/事件数据。

| 源码模块 | 用途线索（来自源码） | 主要类型 / 接口 |
|---|---|---|
| [types](../../src/systems/mode_manager/types.rs) | Original B enum/serde identities recovered from serializers and native equality. AppMode: 0x140afc7c0, 0x140afde20; EditSubject: 0x140afc4a0; DecoratorType: 0x140a478f0; ClutterMeshTy names: 0x142cda7c8 (56 entries). | `WallShape`、`EraserLayer`、`EditSubject`、`ClutterMeshTy`、`DecoratorType`、`AppMode`、`from_native_tag`、`native_tag` 等 |

<a id="systems-render-loop-compute-passes"></a>


## systems/render_loop/compute_passes

- **功能**：计算 pass。
- **设计**：各 pass 明确尺寸、资源读写、绑定、dispatch 和输出。
- **业务**：地形/建筑/水/植被更新、光照/GI、可见性、TAA/DoF 与回读。

| 源码模块 | 用途线索（来自源码） | 主要类型 / 接口 |
|---|---|---|
| [apply_screen_shadows](../../src/systems/render_loop/compute_passes/apply_screen_shadows.rs) | `apply_screen_shadows.rs` — W42 `apply_screen_shadows`（O 0x140a39e70，2163 B）、 W47 `compute_shadow_mask_mean`（O 0x140a3a6f0，497 B）。 | `ShadowMaskHistogramSsbo`、`ShadowMaskMeanSsbo`、`ScreenShadowInputs`、`apply_screen_shadows`、`compute_shadow_mask_mean` |
| [apply_volumetrics](../../src/systems/render_loop/compute_passes/apply_volumetrics.rs) | `apply_volumetrics.rs` — 体积光（W48，`apply_volumetrics` 0x1408a44c0，2009 B）。 | `volumetric_scales`、`apply_volumetrics` |
| [bin_lights](../../src/systems/render_loop/compute_passes/bin_lights.rs) | `bin_lights.rs` — W40 光源分格（O：`bin_lights` 0x1409e7770，3891 B；`BinLights::as_compute_pipeline` 0x1409e8db0）。 | `BinLights`、`LightBinning`、`pipeline_desc`、`bin_lights` |
| [blur_pyramid](../../src/systems/render_loop/compute_passes/blur_pyramid.rs) | `blur_pyramid.rs` — 可复用的模糊金字塔原语（非独立帧 pass）。 O：`blur_pyramid` 0x1409ee1e0（3623 B）、`bidirectional_blur_pyramid` 0x1409ed860（2423 B；内联了 `rev_blur_pyramid`， PDB 里没有它的独立过程）。 | `half_res`、`div_extent`、`pyramid_mips`、`blur_pyramid`、`rev_blur_pyramid`、`bidirectional_blur_pyramid` |
| [compute_garden](../../src/systems/render_loop/compute_passes/compute_garden.rs) | `compute_garden.rs` — W12 `garden_rasterize_from_cpu`（O 0x140a22140，2616 B）、 W16 `garden_spawn_vegetation`（O 0x140a22b80，5234 B）。 | `BushSsbo`、`FlowerSsbo`、`Flower03Ssbo`、`GardenStoneSsbo`、`garden_rasterize_from_cpu`、`garden_spawn_vegetation` |
| [compute_path_masks](../../src/systems/render_loop/compute_passes/compute_path_masks.rs) | `compute_path_masks.rs` — W10 `compute_path_masks`（O 0x1409014e0，7779 B）与 `blur_path_mask`（O 0x140903350，2364 B）。 | `PathStrokeQueues`、`with_words`、`nonempty`、`blur_path_mask`、`compute_path_masks` |
| [compute_water](../../src/systems/render_loop/compute_passes/compute_water.rs) | compute_water.rs — W2 `water_rasterize_from_cpu`、W13 `water_spawn_vegetation`。 | `LilypadSsbo`、`LilypadFlowerSsbo`、`ReedSsbo`、`SeasonByte`、`WaterRasterStaging`、`BlurStep`、`WaterVegetationSsbos`、`world_to_water_grid` 等 |
| [displace_rocky_terrain](../../src/systems/render_loop/compute_passes/displace_rocky_terrain.rs) | `displace_rocky_terrain.rs` — W4 岩质地层位移（O：`displace_rocky_terrain` 0x140a3ee80，1518 B）。 | `RockyTerrainMeshHead`、`group_count`、`displace_rocky_terrain` |
| [dof](../../src/systems/render_loop/compute_passes/dof.rs) | `dof.rs` — Depth-of-field compute chain（W49 `dof_prepare`，W51 `calc_dof`）。 | `DofSource`、`IndirectArgsType`、`DofPrepared`、`DofConfig`、`DofOutput`、`index`、`buffer_offset`、`tile_offsets_initial_values` 等 |
| [dof_autofocus](../../src/systems/render_loop/compute_passes/dof_autofocus.rs) | `dof_autofocus.rs` — DoF autofocus（W32，O：`calculate_dof_autofocus` 0x140a17880，2714 B）。 | `DofAutofocusSsbo`、`DofFocusReadbackSsbo`、`WindowSize`、`FocusParams`、`CameraLens`、`AutofocusInputs`、`AutofocusUniforms`、`aspect_ratio` 等 |
| [eval_gi_screen_radiance](../../src/systems/render_loop/compute_passes/eval_gi_screen_radiance.rs) | `eval_gi_screen_radiance.rs` — 屏幕辐射度求值 / 重投影 / 滤波（W39，0x140a44cd0，7066 B）。 | `GiScreenRadiance`、`eval_gi_screen_radiance` |
| [generate_hzb](../../src/systems/render_loop/compute_passes/generate_hzb.rs) | `generate_hzb.rs` — W26 层级深度缓冲（FidelityFX SPD，O：`generate_hzb` 0x1409286f0，2666 B）。 | `Hzb`、`hzb_dim`、`all_mip_levels`、`generate_hzb` |
| [generate_mips](../../src/systems/render_loop/compute_passes/generate_mips.rs) | `generate_mips.rs` — W50 通用 mip 链（FidelityFX SPD 单 pass，两个单态化： `generate_mips&lt;TexturePreset&gt;` 0x140a3a8f0 / `generate_mips&lt;WriteTextureUnchecked&gt;` 0x1400fb890，机器码同形）。 | `div_ceil_tile`、`last_view_base_mip`、`spd_push_constants`、`spd_atomic_buffer`、`bind_with_view`、`individual_mips_view`、`last_mip_view`、`generate_mips` |
| [gpu_generate_draw_lists](../../src/systems/render_loop/compute_passes/gpu_generate_draw_lists.rs) | `gpu_generate_draw_lists.rs` — Nani GPU 驱动 draw-list 生成（W22/W28）+ 墙 draw-list（W21/W27）。 | `IndirectArgsType`、`PushConstants`、`NaniSsbos`、`CullBucketResources`、`ExternalBufferHandle`、`WallSsbos`、`slot`、`buffer_offset` 等 |
| [grass](../../src/systems/render_loop/compute_passes/grass.rs) | `grass.rs` — W29 GPU 草叶生成（O：`grass_compute` 0x140a60280，8537 B；`ObjectGrassPush` 的 `Component` 实现 `ObjectGrassPush` 是**单个 f32**（O：`object_grass_push_pass` 的 map 闭包 `movss xmm1,[r8]`；`register_component&lt;ObjectGrassPush&gt;` 0x140a5fc10 的 `Layout` = size 4… | `GrassBladeDataSsbo`、`ObjectGrassPush`、`GladeThemeId`、`brush_index`、`grass_compute` |
| [half_res](../../src/systems/render_loop/compute_passes/half_res.rs) | `half_res.rs` — 半分辨率深度/法线提取（W35，`extract_half_res_depth_normals` 0x1408f6180，1850 B）。 | `GBuffer`、`HalfRes`、`div_up_extent`、`extract_half_res_depth_normals` |
| [highlight_active_terrain_stroke](../../src/systems/render_loop/compute_passes/highlight_active_terrain_stroke.rs) | `highlight_active_terrain_stroke.rs` — 高亮当前编辑的地形笔画（W8，0x140a58450，3531 B）。 | `AddTerrainHighlightCmd`、`highlight_stroke_index`、`highlight_active_terrain_stroke` |
| [leaf_particles](../../src/systems/render_loop/compute_passes/leaf_particles.rs) | `leaf_particles.rs` — W18 leaf particles（O：`leaf_particles_compute` 0x1408f4d00，1154 B）。 | `LeafParticleSsbo`、`ParticleEmitterSsbo`、`LoadedLeafParticleCount`、`LoadedParticleEmitterCount`、`clamp_emitter_count`、`leaf_particles_compute` |
| [mod（本目录入口）](../../src/systems/render_loop/compute_passes/mod.rs) | `country_core::systems::render_loop::compute_passes` — frame compute passes. | 内部实现 / 模块装配（见源码） |
| [mouse_depth_value](../../src/systems/render_loop/compute_passes/mouse_depth_value.rs) | `mouse_depth_value.rs` — 鼠标处深度回读（W33，`mouse_depth_compute` 0x140a01620，1742 B）。 | `filter_box_y_frac`、`mouse_depth_compute` |
| [mouse_read_terrain_stroke_index](../../src/systems/render_loop/compute_passes/mouse_read_terrain_stroke_index.rs) | `mouse_read_terrain_stroke_index.rs` — 鼠标所指地形笔画下标回读（W6，0x1409768b0，1000 B）。 | `MouseTerrainRaycastSsbo`、`push_constants_for`、`mouse_read_terrain_stroke_index` |
| [pass_kit](../../src/systems/render_loop/compute_passes/pass_kit.rs) | `pass_kit.rs` — compute pass 声明层的**唯一**共用环境（B 阶段 i_infra 合并 c1 `pass_kit` / c2 `c2_support` / c3 `indirect_params::pass_env` 三份等价实现；非原版模块，原版里这些调用都是内联的）。 | `ResourceView`、`PassFrame`、`PassEnv`、`DynSource`、`binding_hash`、`imm`、`get_resource`、`unwrap_resource` 等 |
| [path_pebbles](../../src/systems/render_loop/compute_passes/path_pebbles.rs) | `path_pebbles.rs` — 路面鹅卵石摆放（W17，`path_pebbles_compute` 0x140958f70，1496 B）。 | `PebblePlacementSsbo`、`path_pebbles_compute` |
| [prefix_scan](../../src/systems/render_loop/compute_passes/prefix_scan.rs) | `prefix_scan.rs` — u32 inclusive 前缀和（上限 1M 项，三段式；`inclusive_prefix_scan_u32_1m` 0x14098bf60，1811 B）。 | `SizedRgBuffer`、`PrefixScan`、`subtotal_buffer_bytes`、`inclusive_prefix_scan_u32_1m` |
| [primary_rt_debug](../../src/systems/render_loop/compute_passes/primary_rt_debug.rs) | `primary_rt_debug.rs` — 主光线（软件光追）调试视图（`primary_rt_debug` 0x1409beca0，1650 B）。 | `primary_rt_debug` |
| [rasterize_buildings](../../src/systems/render_loop/compute_passes/rasterize_buildings.rs) | `rasterize_buildings.rs` — 建筑遮罩栅格化 + 模糊 + 动画（W11，`rasterize_buildings` 0x140a32770，4791 B）。 | `GroundedBuildingsSsbo`、`GroundedSource`、`MaskDesc`、`to_element`、`upload_buildings`、`declare_passes`、`rasterize_buildings` |
| [rasterize_terrain_stroke](../../src/systems/render_loop/compute_passes/rasterize_terrain_stroke.rs) | `rasterize_terrain_stroke.rs` — W3 地形笔画栅格化 + 高斯模糊 + 地形导数。 | `GaussBlur`、`RecalculateHeightMapCmd`、`RasterStroke`、`StrokeInfo`、`StrokeAllocator`、`StrokeUpload`、`ActiveStrokes`、`StrokeEditCounts` 等 |
| [ray_trace_gi](../../src/systems/render_loop/compute_passes/ray_trace_gi.rs) | `ray_trace_gi.rs` — 软件 BVH 追踪 GI + ReSTIR 更新（W38，`ray_trace_gi` 0x140a01cf0，7850 B）。 | `RestirTextures`、`RayTraceGi`、`ray_trace_gi` |
| [read_bush_positions_for_cpu](../../src/systems/render_loop/compute_passes/read_bush_positions_for_cpu.rs) | `read_bush_positions_for_cpu.rs` — 灌木位置回读 CPU（W15，0x140866540，696 B）。 | `BushCpuSsbo`、`read_bush_positions_for_cpu` |
| [read_terrain_data](../../src/systems/render_loop/compute_passes/read_terrain_data.rs) | read_terrain_data.rs — W5 CPU 地形/水面/花园回读（O：`read_terrain_data` 0x14098c680，5917 B）。 | `VisualHeightRasterPlan`、`VisualHeightRaster`、`ScheduledTerrainReadback`、`ReadbackSsbos`、`TerrainReadbackSsbos`、`pack_water_mask_bits`、`pack_water_mask_word`、`on_terrain_modified` 等 |
| [reprojection](../../src/systems/render_loop/compute_passes/reprojection.rs) | `reprojection.rs` — 重投影图（W36，`calculate_reprojection_map` 0x140a0d190，2410 B）。 | `PrevDepth`、`ReprojectionMap`、`calculate_reprojection_map` |
| [reset_indirect_args](../../src/systems/render_loop/compute_passes/reset_indirect_args.rs) | `reset_indirect_args.rs` — 每帧重置四个 indirect-args 缓冲（`reset_indirect_args` 0x14090a870，3044 B； 闭包 `call_once` 0x14169a830，1842 B）。**不是 `ComputePass`**，是一个纯传输 pass。 | `IndirectResetGraph`、`dispatch_init`、`draw_indexed_init`、`draw_init`、`atomic_init`、`reset_indirect_args` |
| [restir_di](../../src/systems/render_loop/compute_passes/restir_di.rs) | `restir_di.rs` — ReSTIR 直接光照蓄水池时间复用（W41，`restir_sample_direct_lights` 0x140a6fec0，2583 B）。 | `LightGrid`、`RestirDi`、`restir_sample_direct_lights` |
| [roof_tiles_gravity](../../src/systems/render_loop/compute_passes/roof_tiles_gravity.rs) | `roof_tiles_gravity.rs` — W19 屋顶瓦片重力模拟（O：`roof_tiles_gravity_compute` 0x140a29b10，1793 B； 迭代器 `from_iter` 0x14130d8e0、dispatch 闭包 `call_once` 0x1415f9210，各按其机器码对照）。 | `PushConstants`、`RoofTileRow`、`RoofGravityFrame`、`to_bytes`、`with_value`、`collect_push_constants`、`roof_tiles_gravity_compute` |
| [shader_refactoring](../../src/systems/render_loop/compute_passes/shader_refactoring.rs) | `shader_refactoring.rs` — 着色器重构对拍（开发用；W54，`render_shader_refactoring` 0x14098f9f0，4231 B）。 | `RefactoringState`、`StateMut`、`FrameHandle`、`RefactoringRun`、`classify`、`next_state`、`render_shader_refactoring`、`try_render_shader_refactoring` |
| [spawn_boats](../../src/systems/render_loop/compute_passes/spawn_boats.rs) | `spawn_boats.rs` — 船 / 水车 spawn 的 SSBO 载体（O：该骨架文件在 PDB 里**没有任何过程**，只有 `BoatSsbo` 一个命名实体）。 | `BoatSsbo` |
| [spawn_vegetation_around_buildings](../../src/systems/render_loop/compute_passes/spawn_vegetation_around_buildings.rs) | `spawn_vegetation_around_buildings.rs` — 建筑周围生成灌木与花（W14，0x1408e4cc0，3394 B）。 | `spawn_vegetation_around_buildings` |
| [ssgi](../../src/systems/render_loop/compute_passes/ssgi.rs) | `ssgi.rs` — 屏幕空间 GI（W37，`calculate_ssgi` 0x1408743b0，4903 B）：三个 compute pass + 时间滤波历史。 | `Ssgi`、`calculate_ssgi` |
| [stairs](../../src/systems/render_loop/compute_passes/stairs.rs) | `stairs.rs` — W20 岩质地形台阶生成（O：`stairs` 0x140a68a50，2286 B）。 | `StairsTransformSsbo`、`stairs` |
| [taa](../../src/systems/render_loop/compute_passes/taa.rs) | `taa.rs` — 时间抗锯齿 / TGSR（W52，二选一）。 | `TgsrMain`、`HistoryTexture`、`TaaInputs`、`TgsrInputs`、`new`、`history_valid_uniform`、`calculate_taa`、`calculate_tgsr` |
| [terrain_lighting_texture](../../src/systems/render_loop/compute_passes/terrain_lighting_texture.rs) | `terrain_lighting_texture.rs` — W24 地形光照纹理计算 + mips（O：`calc_terrain_lighting_texture` 0x140a51440，2290 B）。 | `CameraMatrices`、`with_inverse_view`、`eye_position`、`calc_terrain_lighting_texture` |
| [upload_world_raster](../../src/systems/render_loop/compute_passes/upload_world_raster.rs) | `upload_world_raster.rs` — W1 `upload_raster_staging_to_texture&lt;T&gt;` ×6（O：0x1403d8bd0 PathRaster、 0x1403d90f0 Cobblestone3Raster、0x1403d9610 WaterRaster、0x1403d9b30 GardenRaster、0x1403da050 Cobblestone1Raster、 0x1403da570 Cobblestone2Raster；六个实例机器码同形，各… | `RasterFlags`、`RasterKind`、`new`、`clear_dirty_rows`、`consume_mask_dirty`、`format_suffix`、`upload_world_raster` |
| [visibility_queries](../../src/systems/render_loop/compute_passes/visibility_queries.rs) | `visibility_queries.rs` — 萤火虫可见性查询（W31，`run_visibility_queries` 0x140a2f2e0，904 B）。 | `run_visibility_queries` |

<a id="systems-render-loop-draw-passes"></a>


## systems/render_loop/draw_passes

- **功能**：光栅 pass、draw list 和回调。
- **设计**：附件、材质 shader、场景读取与绘制提交分开。
- **业务**：主/阴影/透明/UI/光标/水冰/最终输出。

| 源码模块 | 用途线索（来自源码） | 主要类型 / 接口 |
|---|---|---|
| [common_draw_params](../../src/systems/render_loop/draw_passes/common_draw_params.rs) | `common_draw_params.rs` — 所有 draw pass 共用的逐帧绑定参数。 | `BrickEraseInputs`、`new`、`from_world`、`to_a4`、`bind_brick_erase_highlight`、`bind_brick_erase_highlight_from_world`、`bind_wall_color_customization`、`bind_stone_floor_textures` 等 |
| [custom_cursor_pass](../../src/systems/render_loop/draw_passes/custom_cursor_pass.rs) | `custom_cursor_pass.rs` — 自定义光标 pass（0x1409d1710，2455 B）。着色器 `custom_cursor`（`/custom_cursor.raster.hlsl`）。 | `CustomCursorSsbo`、`window_uniform`、`overlay_state`、`custom_cursor_pass`、`custom_cursor_pass_from_world` |
| [d_main_support](../../src/systems/render_loop/draw_passes/d_main_support.rs) | `d_main_support.rs` — draw_passes 共用的 pass 宿主层与“列表驱动”运行器（d_main 新增，无独立 PDB 过程）。 | `ColorKind`、`ColorSpec`、`DepthSpec`、`ShaderLookup`、`NoShaders`、`DrawPassCtx`、`ItemMode`、`ExtraBind` 等 |
| [deferred_raster](../../src/systems/render_loop/draw_passes/deferred_raster.rs) | `deferred_raster.rs` — W43 延迟光照（`deferred_raster` 0x1409d20b0，6203 B）。**完整恢复**（B_d_gap：资源句柄接线）。 | `LanternBuffer`、`SpatialResolveOffsetsBuffer`、`DeferredInputs`、`shader_uri`、`materials`、`bind_all`、`entry_state`、`final_state` 等 |
| [draw_callbacks_a](../../src/systems/render_loop/draw_passes/draw_callbacks_a.rs) | `draw_callbacks_a.rs` — `build_draw_lists` 回调的**前一半**本体（d_cb_a，gap 序号 1..=35）。 | `LibView`、`Env6In`、`Env27In`、`Env2In`、`Env57In`、`Env0In`、`Env23In`、`Env22In` 等 |
| [draw_callbacks_a_views](../../src/systems/render_loop/draw_passes/draw_callbacks_a_views.rs) | `draw_callbacks_a` 用到的**视图 / stub 类型**（只含被读字段；偏移为机器码观测）。见 `draw_callbacks_a.rs` 模块文档。 | `AlbedoMapView`、`AlphaMapView`、`NormalMapView`、`WallFlagClothView`、`RoofFlagClothView`、`WoodenGateView`、`SplashParticleView`、`ArrowGizmoView` 等 |
| [draw_callbacks_b](../../src/systems/render_loop/draw_passes/draw_callbacks_b.rs) | `draw_callbacks_b.rs` — `build_draw_lists` 回调（`closure_env$N` 与 `draw_shaders::*` 直接函数）的**后一半**本体（d_cb_b）。 | `MainCameraView`、`WeatherView`、`TreeCanopyMaxCullDistanceView`、`RoofHighlight`、`ActiveSessionView`、`LogoAnimationView`、`RoofFlagClothView`、`CritterControl` 等 |
| [draw_lists](../../src/systems/render_loop/draw_passes/draw_lists.rs) | `draw_lists.rs` — CPU draw-list 容器、排序与构建（`build_draw_lists` 全量恢复，B_d_gap）。 | `DesignatedRenderPass`、`UiPass`、`WireframePass`、`DontCastShadow`、`OcclusionCulled`、`DrawLists`、`PassKind`、`ShaderDispatch` 等 |
| [feedback_report_bg](../../src/systems/render_loop/draw_passes/feedback_report_bg.rs) | `feedback_report_bg.rs` — 反馈报告背景 pass（0x1408a5a70，1231 B）。着色器 `blit_stretch_decode_srgb` （`/logo_overlay.raster.hlsl` 同一路径族，名字取自字符串池）。 | `FeedbackReportBg`、`render_feedback_report_bg` |
| [final_post](../../src/systems/render_loop/draw_passes/final_post.rs) | `final_post.rs` — 最终后处理 / 解析到交换链（B_d_gap：`final_post` 资源句柄接线完整恢复）。 | `ShadowMaskMeanBuffer`、`GradingFloats`、`LutSelect`、`GradingConfig`、`FinalPostEnv`、`FinalPostArgs`、`scale_class`、`msaa_color_desc` 等 |
| [foliage_exclusion_mask](../../src/systems/render_loop/draw_passes/foliage_exclusion_mask.rs) | `foliage_exclusion_mask.rs` — W7 植被排除遮罩光栅（`rasterize_foliage_exclusion_mask` 0x1409cb7b0，2743 B）。 | `StairsFoliageExclusion`、`rasterize_foliage_exclusion_mask` |
| [logo_overlay](../../src/systems/render_loop/draw_passes/logo_overlay.rs) | `logo_overlay.rs` — 启动 logo 叠加（`render_logo_overlay` 0x1408a5f50，1151 B；`LogoOverlayPlugin::build` 0x1408a5f40，8 B）。 | `LogoOverlayPlugin`、`build`、`render_logo_overlay` |
| [main_pass](../../src/systems/render_loop/draw_passes/main_pass.rs) | `main_pass.rs` — W25 `main_pass`（0x140833bc0，7794 B）、W30 `main_pass_oc2`（0x140835a40，6498 B）、 W34 `late_forward_and_deferred_pass`（0x1408373b0，2177 B）。 | `MainPassTargets`、`with_stencil_write_state`、`main_pass`、`main_pass_oc2`、`late_forward_and_deferred_pass` |
| [material_id](../../src/systems/render_loop/draw_passes/material_id.rs) | `material_id.rs` — 延迟光照的材质标签（`core::fmt::impl$74::fmt&lt;MaterialId&gt;` 0x14085da60，41 B）。 | `MaterialId` |
| [mod（本目录入口）](../../src/systems/render_loop/draw_passes/mod.rs) | `country_core::systems::render_loop::draw_passes` — frame raster passes. | 内部实现 / 模块装配（见源码） |
| [object_grass_push](../../src/systems/render_loop/draw_passes/object_grass_push.rs) | `object_grass_push.rs` — W9 物体推草 pass（`object_grass_push_pass` 0x1409daaa0，3955 B）。着色器 `grass_object_push`。 | `Vec4Uniform`、`PushRadiusInputs`、`cursor_push`、`push_radius`、`assemble_push_objects`、`collect_push_objects`、`object_grass_push_pass` |
| [shadow_pass](../../src/systems/render_loop/draw_passes/shadow_pass.rs) | `shadow_pass.rs` — W23 阴影级联光栅（0x1409cc270，2210 B）。 | `shadow_pass` |
| [transparency_pass](../../src/systems/render_loop/draw_passes/transparency_pass.rs) | transparency_pass.rs — W44 折射拷贝 / W45 水·冰表面 / W46 pre-TAA 与 W53 post-TAA 透明。 | `WaterSurfaceKind`、`PlanKind`、`PlanOp`、`SurfacePlan`、`select_water_surface`、`refraction_copy_pass`、`pre_taa_transparency_pass`、`post_taa_transparency_pass` 等 |
| [ui_2d_pass](../../src/systems/render_loop/draw_passes/ui_2d_pass.rs) | `ui_2d_pass.rs` — 最终图 UI 2D pass（`ui_2d_pass` 0x1409c4470，8843 B）与 MEI pass（`mei_pass` 0x1409c6700，3255 B）。 | `ShapeStreamSsbo`、`TextPrimSsbo`、`Vec2Uniform`、`Vec3Uniform`、`Ui2dImageRef`、`ImageView`、`CustomizationIconRead`、`UiInteractionStateRead` 等 |
| [ui_pass](../../src/systems/render_loop/draw_passes/ui_pass.rs) | `ui_pass.rs` — UI pass（0x140837c40）与 loading-screen pass（0x140837ff0），各 929 B，机器码仅列表偏移不同。 | `ui_pass`、`loading_screen_pass` |
| [wireframe_pass](../../src/systems/render_loop/draw_passes/wireframe_pass.rs) | `wireframe_pass.rs` — 调试线框 pass（0x1409ccb20，929 B）。 | `wireframe_pass` |

<a id="systems-render-loop-draw-passes-draw-shaders"></a>


## systems/render_loop/draw_passes/draw_shaders

- **功能**：对象和材质对应的 shader 声明。
- **设计**：每种对象组织自己的输入/绑定和程序。
- **业务**：草、屋顶、砖、楼梯、动物、常春藤、天空和水绘制。

| 源码模块 | 用途线索（来自源码） | 主要类型 / 接口 |
|---|---|---|
| [billboard_forest](../../src/systems/render_loop/draw_passes/draw_shaders/billboard_forest.rs) | billboard_forest.rs — 远景 billboard 森林（startup 域，非帧 draw）。 | `BillboardForest`、`build`、`startup_billboard_forest` |
| [clearing_shaders](../../src/systems/render_loop/draw_passes/draw_shaders/clearing_shaders.rs) | clearing_shaders.rs — 空地里的树：树干 / 树冠（nani GPU-driven 路径）。 | `TreeTrunkInputs`、`TreeCanopyInputs`、`canopy_param_offset`、`canopy_factor`、`blend_value`、`tree_state`、`tree_trunk`、`tree_canopy` |
| [clouds](../../src/systems/render_loop/draw_passes/draw_shaders/clouds.rs) | clouds.rs — 云层（startup 域，非帧 draw）。 | `CloudPlugin`、`build`、`startup_clouds` |
| [common_includes](../../src/systems/render_loop/draw_passes/draw_shaders/common_includes.rs) | common_includes.rs — 跨 pass 共享的 `bind_*` 助手（`DrawBuilder` / `ComputePass` 两套）。 | `DrawModeExt`、`bind_preset`、`bind_disk`、`bind_terrain_heightmap`、`bind_terrain_lighting`、`bind_water_object_push` |
| [common_includes_compute](../../src/systems/render_loop/draw_passes/draw_shaders/common_includes_compute.rs) | common_includes_compute.rs — `common_includes` 里 `ComputePass` 实例的两个助手（拆出独立文件只是为了让 `DrawBuilder` 一侧的单元测试不必拖入 compute 全套；PDB 里它们同属 `draw_shaders::common_includes`）。 | `bind_water_object_push_compute`、`bind_garden_exclusion_compute` |
| [draw_bricks](../../src/systems/render_loop/draw_passes/draw_shaders/draw_bricks.rs) | draw_bricks.rs — 墙砖 LOD0 / LOD1 间接绘制（`_wall_wall_brick_lod{0,1}.raster`）。 | `BrickProxy`、`BrickEntity`、`BrickGate`、`lod1_mode_bits`、`gate`、`bricks_mode`、`indirect_draw_bricks`、`indirect_draw_bricks_with_mode` |
| [floor_shader](../../src/systems/render_loop/draw_passes/draw_shaders/floor_shader.rs) | floor_shader.rs — 地形地面（`_terrain_editing_floor.raster`）。 | `floor_state`、`draw_floor_shader` |
| [grass_shader](../../src/systems/render_loop/draw_passes/draw_shaders/grass_shader.rs) | grass_shader.rs — 草地 LOD0 / LOD1（`_grass.raster` / `_grass_lod1.raster`）。 | `draw_common`、`draw_grass_shader`、`draw_grass_lod1_shader` |
| [instanced_critters](../../src/systems/render_loop/draw_passes/draw_shaders/instanced_critters.rs) | instanced_critters.rs — 蝴蝶 / 萤火虫 / 萤火虫光晕 / 鸭 / 鹪鹩 的实例绘制。 | `DuckInputs`、`draw_instanced_butterfly_shader`、`firefly_is_hidden`、`draw_instanced_firefly_shader`、`draw_instanced_firefly_glow_shader`、`model_matrix`、`draw_duck_shader`、`draw_wren_shader` |
| [instanced_ivy](../../src/systems/render_loop/draw_passes/draw_shaders/instanced_ivy.rs) | instanced_ivy.rs — 常春藤实例绘制（枝 / 叶 / 花）。 | `branch_instance_count`、`draw_instanced_ivy_branch_shader`、`ivy_leaf_shader`、`draw_instanced_ivy_leaf_shader`、`draw_instanced_ivy_flower_shader` |
| [instanced_roof](../../src/systems/render_loop/draw_passes/draw_shaders/instanced_roof.rs) | instanced_roof.rs — 屋顶瓦 / 尖顶 / 支撑梁的实例绘制。 | `RoofNaniParam`、`uniforms`、`roof_state`、`draw_instanced_roof_shader_nani`、`draw_instanced_roof_spire_shader`、`draw_instanced_roof_support_beam_shader` |
| [leaf_particle_draw](../../src/systems/render_loop/draw_passes/draw_shaders/leaf_particle_draw.rs) | leaf_particle_draw.rs — 落叶粒子（`_leaf_particle.raster`）。 | `leaf_particle_draw` |
| [mod（本目录入口）](../../src/systems/render_loop/draw_passes/draw_shaders/mod.rs) | `crate::systems::render_loop::draw_passes::draw_shaders` — per-shader draw 组合与 startup 系统。 | 内部实现 / 模块装配（见源码） |
| [path_pebble_shader](../../src/systems/render_loop/draw_passes/draw_shaders/path_pebble_shader.rs) | path_pebble_shader.rs — 路径石子（`_path_pebble.raster`）。 | `path_pebble_draw` |
| [rocky_terrain_draw](../../src/systems/render_loop/draw_passes/draw_shaders/rocky_terrain_draw.rs) | rocky_terrain_draw.rs — 岩石地形（`_rocky_terrain.raster`）。 | `rocky_state`、`rocky_terrain_shader` |
| [ssbo_tags](../../src/systems/render_loop/draw_passes/draw_shaders/ssbo_tags.rs) | ssbo_tags.rs — draw_shaders 里被 `DrawBuilder::read_storage_buffer&lt;T&gt;` 读取的 SSBO 类型标签。 | `DrawSsbo`、`ReadSsboExt`、`InstanceBufferView`、`SsboInstanceView`、`AtomicCounterTag`、`DrawIndirectTag`、`DrawIndexedIndirectTag`、`instance_buffer` 等 |
| [stairs_draw](../../src/systems/render_loop/draw_passes/draw_shaders/stairs_draw.rs) | stairs_draw.rs — 楼梯（`_stairs.raster`）。 | `StairsEntityData`、`stairs_flag`、`stairs_draw` |
| [stars](../../src/systems/render_loop/draw_passes/draw_shaders/stars.rs) | stars.rs — 星空（startup 域，非帧 draw）。 | `StarsPlugin`、`build`、`startup_stars` |
| [startup_quad](../../src/systems/render_loop/draw_passes/draw_shaders/startup_quad.rs) | startup_quad.rs — `stars` / `clouds` / `billboard_forest` 三个 startup 系统共用的“单面片网格 + shader + spawn”骨架。 | `StartupMesh`、`StartupBundle`、`StartupHost`、`AddStartupSystem`、`QuadSpec`、`Rec`、`Systems`、`quad_window_indices` 等 |
| [water_planes_draw](../../src/systems/render_loop/draw_passes/draw_shaders/water_planes_draw.rs) | water_planes_draw.rs — 水面网格与水生植被 draw（`_water_plane` / `_lilypad` / `_lilypad_flower` / `_reed`）。 | `WaterPlanesInputs`、`water_color`、`all_nonzero`、`water_state`、`water_planes_draw`、`water_planes_draw_rt`、`lilypad_draw`、`lilypad_flower_draw` 等 |

<a id="systems-render-loop-draw-passes-transparency-pass"></a>


## systems/render_loop/draw_passes/transparency_pass

- **功能**：透明表面绘制。
- **设计**：透明材质支持与水/冰表面逻辑分离。
- **业务**：构造透明通道中水面和冰面的绘制请求。

| 源码模块 | 用途线索（来自源码） | 主要类型 / 接口 |
|---|---|---|
| [surface_ice](../../src/systems/render_loop/draw_passes/transparency_pass/surface_ice.rs) | `surface_ice.rs` — W45 冰表面 `ice_surface_pass`（0x1409c08d0，12711 B）。**真实实现**（B_d_water）。 | `ice_surface_pass` |
| [surface_support](../../src/systems/render_loop/draw_passes/transparency_pass/surface_support.rs) | `surface_support.rs` — 水 / 冰表面 pass（`water_surface_pass` 0x1409bf800、`ice_surface_pass` 0x1409c08d0）共用的接缝与步骤助手。 | `SurfaceCompute`、`SurfaceInputs`、`SurfaceEnv`、`Cp`、`new`、`op`、`create_buffer`、`create_texture` 等 |
| [surface_water](../../src/systems/render_loop/draw_passes/transparency_pass/surface_water.rs) | `surface_water.rs` — W45 水表面 `water_surface_pass`（0x1409bf800，4292 B）。**真实实现**（B_d_water）。 | `water_surface_pass` |

<a id="systems-render-loop"></a>


## systems/render_loop

- **功能**：整帧渲染组织和菜单/预览画面。
- **设计**：帧级编排、参数配置、pass 和 backend 分层。
- **业务**：读取世界/相机→组织上传/计算/光栅/后处理/回读。

| 源码模块 | 用途线索（来自源码） | 主要类型 / 接口 |
|---|---|---|
| [indirect_params](../../src/systems/render_loop/indirect_params.rs) | `indirect_params.rs` — GPU indirect-args 缓冲类型（`country_core::systems::render_loop::indirect_params`）。 | 内部实现 / 模块装配（见源码） |
| [lut_gallery](../../src/systems/render_loop/lut_gallery.rs) | `lut_gallery.rs` — photomode LUT gallery thumbnails. | `LutGalleryInner`、`LutGalleryState`、`LutName`、`LutGalleryHost`、`CellUniforms`、`LutGalleryPlugin`、`cell_uniforms`、`render_photomode_lut_gallery` 等 |
| [menu_background](../../src/systems/render_loop/menu_background.rs) | `menu_background.rs` — menu background blit (boot/menu fullscreen). | `AppModeView`、`ManagedTexture`、`MenuBackgroundResources`、`MenuBackgroundHost`、`MenuBackgroundPlugin`、`menu_bg_mode_allows`、`take_pending_into_current`、`render_menu_bg` 等 |
| [mod（本目录入口）](../../src/systems/render_loop/mod.rs) | `country_core::systems::render_loop` — frame-pass composition. | `RecordedRgCommand`、`RecordedRgPass`、`RgBufferInfo`、`RgImageInfo`、`RenderContext`、`DynamicStagingAllocator`、`with_staging`、`finish` 等 |
| [painterly](../../src/systems/render_loop/painterly.rs) | `painterly.rs` — painterly stylized stroke-reveal chain (progressive strokes). | `TexRef`、`BufRef`、`ShaderHandle`、`TextureDesc`、`BufferDesc`、`TemporalTexture`、`TemporalBuffer`、`HostError` 等 |
| [render_frame](../../src/systems/render_loop/render_frame.rs) | `tiny_glade::render_loop::render`（0x1404e8010，16279 B）与 `handle_screenshot_error`（0x1404ebfb0，337 B）。 | `Level`、`RenderMode`、`SwapchainInfo`、`Acquire`、`RenderHost`、`Ring4`、`FramePacer`、`from_byte` 等 |
| [render_world](../../src/systems/render_loop/render_world.rs) | `tiny_glade::render_loop::render_world` (0x1404ec110, 4314 B) — 世界渲染图的整帧 pass 编排。 | `TextureDesc`、`RgTexture`、`TemporalTexture`、`TemporalPair`、`HalfResTargets`、`GBufferTargets`、`HzbHandles`、`LightBins` 等 |
| [vermeer](../../src/systems/render_loop/vermeer.rs) | `vermeer.rs` — Vermeer stylized stroke-reveal chain (session-start animation). | `GenerateStrokes`、`VermeerPhase`、`ManagedTexture`、`VermeerState`、`VermeerDebugControl`、`VermeerPlugin`、`VermeerApp`、`VermeerHost` 等 |

<a id="systems-render-loop-resources"></a>


## systems/render_loop/resources

- **功能**：帧渲染消费资源。
- **设计**：显式渲染输入，不默认代表完整权威状态。
- **业务**：当前包括动物绘制输入。

| 源码模块 | 用途线索（来自源码） | 主要类型 / 接口 |
|---|---|---|
| [critters](../../src/systems/render_loop/resources/critters.rs) | `render_loop/resources/critters.rs` — critter 的 GPU 实例缓冲元素类型（四个 SSBO）。 | `ButterflySsbo`、`FireflySsbo`、`FireflyVisibilitySsbo`、`SheepAnimationSsbo`、`from_components`、`from_translation` |
| [mod（本目录入口）](../../src/systems/render_loop/resources/mod.rs) | 未提供独立用途总述；通过下列源码类型/接口定位。职责按本节领域合同理解，不据文件名推断完成度。 | 内部实现 / 模块装配（见源码） |

<a id="systems-render-loop-session-selection"></a>


## systems/render_loop/session_selection

- **功能**：会话选择渲染。
- **设计**：页面状态、布局/缩略图和绘制输入分离。
- **业务**：展示可选场景与选中状态。

| 源码模块 | 用途线索（来自源码） | 主要类型 / 接口 |
|---|---|---|
| [mod（本目录入口）](../../src/systems/render_loop/session_selection/mod.rs) | `session_selection/mod.rs` — session-selection preview thumbnails. | `PooledThumbnail`、`ThumbnailTextures`、`CarouselState`、`SelectionModeView`、`SessionRow`、`RgbaImage`、`RefreshError`、`DecodeOutcome` 等 |

<a id="systems-save"></a>


## systems/save

- **功能**：保存、加载、备份和快照。
- **设计**：游戏线程采集→异步命令→影子文件；解析与域应用分开。
- **业务**：自动/同步保存、快照 GC、版本处理、on_load 回放；完整往返仍未闭合。

| 源码模块 | 用途线索（来自源码） | 主要类型 / 接口 |
|---|---|---|
| [load_completion_checker](../../src/systems/save/load_completion_checker.rs) | B `systems/save/load_completion_checker.rs` — 读档完成检查。 | `SessionLoadReadiness`、`DisplayCorruptedSaveFileWarning`、`LoadingCompletionState`、`is_ok`、`deny_load`、`check_load_completion`、`check_load_completion_in`、`is_session_ok_to_load` 等 |
| [load_game](../../src/systems/save/load_game.rs) | B `systems/save/load_game.rs` — 读档主系统。 | `LoadGameState`、`LoadGameManager`、`LoadGameSource`、`ParsedGameData`、`TimeOfDaySection`、`HistorySectionJson`、`PostGameLoadCommand`、`LoadingGameComplete` 等 |
| [mod（本目录入口）](../../src/systems/save/mod.rs) | Recovered save-file codec, mutable shadow and incremental write consumers, plus the asynchronous capture thread. The original game-thread capture system that assembles GameSaveData from ~9 domain resources is not registered here yet (domain owners pending);… | `add_save_load_systems`、`add_save_load_events_and_resources`、`add_save_load_systems_impl` |
| [save_backup_list](../../src/systems/save/save_backup_list.rs) | B 0x140ae8330, 0x140ae8470 and filter-map closure 0x14166df80. | `BackupSaveFileKind`、`BackupSaveFile`、`SaveBackupList`、`is_auto`、`is_corrupted`、`iter_backup_files`、`new_from_session`、`new_from_directory` 等 |
| [save_backup_metronome](../../src/systems/save/save_backup_metronome.rs) | B save_game 0x140a3fc15..0x140a3fd36, 0x140a4100b..0x140a410da. 字段：counter u32@+0x140、累计 f32@+0x144、阈值 f32@+0x148。reviewer 复核： **阈值门（+0x148 &lt; 60.0 才累计）在累计块内**，不属 AutoSaveState 本体。 | `SaveBackupMetronome`、`BackupDecision`、`new_from_backup_list`、`from_backup_counter`、`counter`、`active_game_time`、`active_game_threshold`、`set_active_game_threshold` 等 |
| [save_file_identity](../../src/systems/save/save_file_identity.rs) | B `systems/save/save_file_identity.rs` — 存档文件身份识别。 | `SaveFileIdentity`、`try_approx_parse_save_file_identity`、`derive_fallback_save_file_identity`、`parse_save_file_identity` |
| [save_game](../../src/systems/save/save_game.rs) | B 0x141750c90 + 0x141751c90: JSON envelope and section/edit offsets. Domain values are serialized by their owning modules; this layer does not substitute fabricated wall/roof/snapshot payloads for those values. | `SaveFileStructure`、`SerializedJson`、`GameSaveSections`、`AsyncSaveCmd`、`CaptureSnapshotRequested`、`SaveGameSyncData`、`SnapshotCaptureState`、`SaveGameCmd` 等 |
| [save_thread](../../src/systems/save/save_thread.rs) | Mutable file portion of CurrentSaveFile; native offsets +0x10 shadow, +0x90 history offsets, +0xa8 section ranges. | `DeferredSection`、`UnsavedFileEntry`、`UnsavedFileList`、`AutoSaveError`、`AutosaveErrorState`、`CurrentSaveFile`、`SaveThreadHandle`、`from_path` 等 |
| [shadow_file](../../src/systems/save/shadow_file.rs) | B 0x1409184d0 / 0x1409193d0 / 0x14091a230. Creation eagerly writes a same-directory NamedTempFile and persists it. On failure the full memory image survives, all blocks stay dirty and flush retries. | `ShadowFile`、`ShadowFileWriter`、`create`、`data`、`path`、`directory`、`last_error`、`is_open` 等 |
| [snapshot](../../src/systems/save/snapshot.rs) | B `systems/save/snapshot.rs` — 快照捕获/加载。 | `SnapshotTreesEntry`、`SnapshotDomainData`、`SnapshotSaveData`、`LoadedSnapshotData`、`SnapshotLoadResources`、`SnapshotDomainDescriptor`、`SnapshotPrefix`、`WallStateJson` 等 |
| [snapshot_gc](../../src/systems/save/snapshot_gc.rs) | B 0x141286730 selective reference reader and 0x14172b280 GC worker. | `SaveFileSnapshot`、`parse_save_file_snapshots`、`snapshot_gc_thread` |

<a id="systems-steam"></a>


## systems/steam

- **功能**：Steam 启动适配。
- **设计**：平台接口与游戏资源分开。
- **业务**：建立 SteamInput 状态和动作句柄。

| 源码模块 | 用途线索（来自源码） | 主要类型 / 接口 |
|---|---|---|
| [mod（本目录入口）](../../src/systems/steam/mod.rs) | `systems/steam/mod.rs` — Steam 集成：客户端资源/覆盖层/截图/输入映射。 | `SteamClientResource`、`AppSteamIntegration`、`PrevSteamInputState`、`SteamScreenshotRequests`、`OsWindowCmd`、`SteamOverlayOpenResources`、`show_floating_gamepad_text_input`、`is_floating_gamepad_text_input_open` 等 |
| [startup_steam](../../src/systems/steam/startup_steam.rs) | `systems/steam/startup_steam.rs` — Steam 启动系统（首帧一次性资源插入）。 | `startup_steam` |

<a id="systems-terrain"></a>


## systems/terrain

- **功能**：地形交互、通知、历史与回放。
- **设计**：UI、stroke 修改、hover/focus、延迟通知各自成系统。
- **业务**：造型/改参数→编辑状态→聚合变更→历史/回放。

| 源码模块 | 用途线索（来自源码） | 主要类型 / 接口 |
|---|---|---|
| [deferred_terrain_history](../../src/systems/terrain/deferred_terrain_history.rs) | `deferred_terrain_history.rs` — 延迟地形历史（B 0x140ac9770，O）。 | `PendingTerrainHistory`、`TerrainEdit`、`DebugHistoryMsg`、`history_record_terrain` |
| [delayed_terrain_changed_msg](../../src/systems/terrain/delayed_terrain_changed_msg.rs) | B 0x140aabdd0. At most one pair of CPU copies per frame; expiration is a two-frame software delay, not a fabricated GPU completion fence. | `DelayedUpdateWallsMsgBuffer`、`OnTerrainChange`、`delayed_terrain_changed_msg` |
| [draw_terrain_stroke](../../src/systems/terrain/draw_terrain_stroke.rs) | B 0x140a03ba0 coalesces all unread stroke changes into one notification. | `EvTerrainStrokeChanged`、`terrain_stroke_changed` |
| [edit_terrain_stroke](../../src/systems/terrain/edit_terrain_stroke.rs) | B 0x14094e000 consumes point replacements before parameter edits. Missing IDs are logged and skipped; old chapter's "missing stroke panic" was wrong. | `EditTerrainStrokeCmd`、`SetTerrainStrokePtsCmd`、`edit_terrain_stroke` |
| [mod（本目录入口）](../../src/systems/terrain/mod.rs) | Recovered CPU terrain event consumers, tool UI producers and delayed mapped-height readback. GPU execution and original global schedule remain separate. | `PlayTerrainSound`、`TerrainCpuStage`、`native_tag`、`register_terrain_events`、`add_terrain_cpu_systems`、`add_terrain_tool_systems` |
| [replay_terrain](../../src/systems/terrain/replay_terrain.rs) | `replay_terrain.rs` — 地形回放（B 0x140995150，O；`TerrainReplayCmd::forward/backward` 0x140994f30 / 0x140995040，O）。 | `TerrainReplayCmd`、`forward`、`backward`、`eased`、`make_instant`、`adjacent_distance_sum`、`replay_terrain`、`terrain_replay_from_edit` 等 |
| [terrain_hover_idx](../../src/systems/terrain/terrain_hover_idx.rs) | `terrain_hover_idx.rs` — 地形悬停笔画索引（B 0x140aff720，O）。 | `HoveredTerrainStrokeId`、`MouseTerrainHoverReadback`、`TerrainWaterSample`、`terrain_hover_idx` |
| [ui_draw_terrain](../../src/systems/terrain/ui_draw_terrain.rs) | `ui_draw_terrain.rs` — 地形绘制 UI（B 0x140a9fdc0）。 | `TerrainHoverInput`、`TerrainDrawState`、`ui_draw_terrain` |
| [ui_edit_terrain](../../src/systems/terrain/ui_edit_terrain.rs) | `ui_edit_terrain.rs` — 地形编辑 UI（B 0x140aef1f0）。 | `TerrainWindowFocus`、`StairsPlacementEntry`、`StairsPlacementBucket`、`TerrainEditHistoryPending`、`TerrainEditUiState`、`TerrainWidgetResponseTable`、`ui_edit_terrain` |
| [ui_terrain_focus_mode](../../src/systems/terrain/ui_terrain_focus_mode.rs) | `ui_terrain_focus_mode.rs` — 地形聚焦模式（B 0x1409a4fb0）。 | `TerrainFocusRaycast`、`TerrainFocusHover`、`TerrainFocusSelection`、`TerrainFocusTrigger`、`DrawTopology`、`SignifierDrawTopology`、`stroke_focus_circle`、`ui_terrain_focus_mode` |

<a id="systems-ui"></a>


## systems/ui

- **功能**：3D 控件及操作标记。
- **设计**：控件资源与标记的创建/动画/删除分开。
- **业务**：围绕选中对象展示可拖动的操作入口。

| 源码模块 | 用途线索（来自源码） | 主要类型 / 接口 |
|---|---|---|
| [mod（本目录入口）](../../src/systems/ui/mod.rs) | Recovered widget registration/query owner. Input arbitration and the other widget volumes remain separate work; no default interaction is fabricated. | 内部实现 / 模块装配（见源码） |

<a id="systems-ui-resources"></a>


## systems/ui/resources

- **功能**：3D slider 和 widget 状态。
- **设计**：UI 状态、几何参数、拖动约束独立持有。
- **业务**：将拾取和拖动转换为控件值变化。

| 源码模块 | 用途线索（来自源码） | 主要类型 / 接口 |
|---|---|---|
| [mod（本目录入口）](../../src/systems/ui/resources/mod.rs) | 未提供独立用途总述；通过下列源码类型/接口定位。职责按本节领域合同理解，不据文件名推断完成度。 | 内部实现 / 模块装配（见源码） |
| [slider3d](../../src/systems/ui/resources/slider3d.rs) | `slider3d.rs` — 3D 世界空间滑块（B 0x140a9bd40 `project_cursor_onto_line`，O）。 | `Slider3dMatrices`、`Slider3dState`、`into_drag_state`、`project_cursor_onto_line` |
| [ui_state](../../src/systems/ui/resources/ui_state.rs) | B clearing_canvas_w_layer_and_sound 0x14092df50 and dragged 0x14092df10. This owner represents the recovered canvas registry and interaction query fields, not the complete native UiState memory layout. | `DragState`、`WidgetId`、`UiInteraction`、`ClearingCanvasWidget`、`UiState`、`UiWidgetContext`、`is_some`、`is_started` 等 |
| [widget_params](../../src/systems/ui/resources/widget_params.rs) | `widget_params.rs` — WidgetParams 0xb0 B 参数块（B 0x1409a8e10 起，O）。 | `PlayUiSound`、`UiNavState`、`UiFocus`、`WidgetVolumeShape`、`UiSounds`、`WidgetFlags`、`WidgetParams`、`variant4` 等 |

<a id="systems-ui-signifiers"></a>


## systems/ui/signifiers

- **功能**：操作标记生命周期。
- **设计**：factory/create→animation→mark_for_delete。
- **业务**：在编辑对象周围创建提示，更新后清理失效标记。

| 源码模块 | 用途线索（来自源码） | 主要类型 / 接口 |
|---|---|---|
| [animation](../../src/systems/ui/signifiers/animation.rs) | `animation.rs` — 指示符动画（B 0x140a69340 `signifier_animation_t` / 0x140a69c30 `signifier_animation`，O）。 | `SignifierAnimation`、`SignifierParentToVolumeOrigin`、`signifier_animation_t`、`signifier_animation` |
| [mod（本目录入口）](../../src/systems/ui/signifiers/mod.rs) | `signifiers/mod.rs` — 指示符流/生成/动画/回收（O：ui-input DIGEST）。 | `register_signifier_resources`、`add_signifier_systems` |
| [signifier_create](../../src/systems/ui/signifiers/signifier_create.rs) | `signifier_create.rs` — UiSignifierStream + signifier_create（B 0x140b2cf90/0x140b2cfa0，O）。 | `UiAppendSignifier`、`SignifierId`、`UiSignifierEntry`、`UiSignifierStream`、`new`、`from_raw_qwords`、`write`、`write_raw` 等 |
| [signifier_factory](../../src/systems/ui/signifiers/signifier_factory.rs) | `signifier_factory.rs` — 指示符记录工厂与实体构建器（O，2026-10-17 coregap-1 fresh 反编译）。 | `SignifierClosure`、`SignifierFactory`、`SignifierEntityBuilder`、`new`、`from_closure`、`has_closure`、`name`、`wake` 等 |
| [signifier_mark_for_delete](../../src/systems/ui/signifiers/signifier_mark_for_delete.rs) | `signifier_mark_for_delete.rs` — 指示符删除（B 0x140929160 `signfier_mark_for_delete` / 0x140929a90 `despawn_marked`，O；PDB 拼写 `signfier`）。 | `SignifierMarkedForDeletion`、`SignifierMinimumLifespan`、`signfier_mark_for_delete`、`despawn_marked` |

<a id="systems-ui-clutter"></a>


## systems/ui_clutter

- **功能**：自动杂物的交互和历史。
- **设计**：权威删除集合、延迟历史、右键/调试生成和 pin 分离。
- **业务**：放置/旋转/删除场景杂物，再通过历史撤销重做。

| 源码模块 | 用途线索（来自源码） | 主要类型 / 接口 |
|---|---|---|
| [autoclutter_state](../../src/systems/ui_clutter/autoclutter_state.rs) | `autoclutter_state.rs` — AutoClutter 自动摆放状态与记录布局（B `system_clutter::autoclutter`， O 2026-10-17 coregap-1 fresh 反编译 `add_region` 0x140ee3fd0 / `mark_dirty` 0x140ee48c0 / `flush_candidates_pending_removal` 0x140ee4bf0）。 | `AutoClutterRegion`、`AutoClutterSpawn`、`AutoClutterObject`、`AutoClutterCandidate`、`Rectangle3d`、`AutoClutterState`、`is_equivalent`、`key` |
| [debug_spawn_clutter](../../src/systems/ui_clutter/debug_spawn_clutter.rs) | `debug_spawn_clutter.rs` — 调试批量生成 clutter（B 0x1409a1f50，O）。 | `DebugSpawnClutterCmd`、`DebugSpawnAllClutterCmd`、`SpawnAutoClutter`、`DebugClutterEntityCounter`、`spiral_position`、`debug_spawn_all_clutter` |
| [decoclutter_show_pins](../../src/systems/ui_clutter/decoclutter_show_pins.rs) | `decoclutter_show_pins.rs` — 显示/隐藏已固定 deco-clutter 图钉（B 0x1412357c0）。 | `PinnedDecoClutter`、`PinnedDecoGeometry`、`PinnedDecoGeometryTable`、`ShowPinnedDecoClutter`、`PinAnimation`、`PinnedDecoClutterBuffers`、`PinAnimationTable`、`PinVisibilityToggle` 等 |
| [defered_autoclutter_history](../../src/systems/ui_clutter/defered_autoclutter_history.rs) | B 0x1409a9160 consumes every unread ID and appends EditType tag 13. The original function does not open or finish a recording session. | `RecordAutoClutterHistory`、`defered_autoclutter_history` |
| [edit_autoclutter](../../src/systems/ui_clutter/edit_autoclutter.rs) | Resolved-ID adapter for the real deletion and deferred recording owners. This is not the still-unrestored native RMB picking/conversion function. | `DeleteAutoClutterCmd`、`delete_auto_clutter` |
| [mod（本目录入口）](../../src/systems/ui_clutter/mod.rs) | Automatic clutter deletion authority, the recovered ID/history path and the picking/visible-consumption producers (rmb/ui_autoclutter/debug spawn). | `DeletedAutoClutter`、`AutoClutterRecord`、`AutoClutterEntityIndex`、`is_deleted` |
| [registration](../../src/systems/ui_clutter/registration.rs) | Local resolved-ID edit/record order; caller maintains events and supplies History&lt;S&gt;. The global original UI schedule has not been reconstructed. | `register_ui_clutter_resources`、`add_ui_clutter_cpu_systems` |
| [rmb_autoclutter](../../src/systems/ui_clutter/rmb_autoclutter.rs) | `rmb_autoclutter.rs` — 右键移除 auto-clutter（B 0x140976ca0，O）。 | `RmbAutoClutterHit`、`RmbAutoClutterDirect`、`EvRightClickedAutoClutter`、`fmod_2pi`、`get_clutter_rot_in_roof_space`、`handle_rmb_autoclutter` |
| [ui_autoclutter](../../src/systems/ui_clutter/ui_autoclutter.rs) | `ui_autoclutter.rs` — 右键把 auto-clutter 转 deco-clutter（B 0x14099ca60）。 | `DisplayChildAutoClutterInfo`、`AutoClutterChild`、`AutoClutterChildTable`、`AutoClutterParentMap`、`maintain`、`parent_of`、`ui_convert_autoclutter_to_decoclutter` |
| [ui_focus_decoclutter](../../src/systems/ui_clutter/ui_focus_decoclutter.rs) | `ui_focus_decoclutter.rs` — deco-clutter 聚焦（B 0x1409a92c0）。 | `DecoClutterFocusState`、`DecoClutterFocusUi`、`RequestRotateDecorator`、`RequestMoveClutter`、`ui_focus_decoclutter` |

<a id="systems-wall"></a>


## systems/wall

- **功能**：墙编辑、构造、发布、删除与历史。
- **设计**：输入工具与权威状态修改分开，再更新几何/空间/屋顶缓存。
- **业务**：建墙、修改轮廓/高度、删除并同步相关消费者。

| 源码模块 | 用途线索（来自源码） | 主要类型 / 接口 |
|---|---|---|
| [adjust_wall_height](../../src/systems/wall/adjust_wall_height.rs) | B AdjustWallHeightCmd transport into the actual InnerWalls authority. | `AdjustWallHeightCmd`、`WallHeightAudio`、`WallHeightCommandFailed`、`WallCursorHit`、`UiWallHeight`、`register_wall_height_events`、`adjust_wall_height`、`lifted_points` 等 |
| [calc_wall_bend](../../src/systems/wall/calc_wall_bend.rs) | B 0x140a8a710: the final four terrain-mapped XZ directions drive the bend scalar event. The historical reconstruction used the wrong end of the curve. | `WallBendChangedEvent`、`compute_bend`、`calc_wall_bend` |
| [construct_wall_bricks](../../src/systems/wall/construct_wall_bricks.rs) | CPU brick section of the original external system-wall-constructor owner (B 0x140757800), exposed here so restored authority has an executable consumer. The seeded producer and trigger geometry are real. GPU storage and RT side effects are not represented b… | `RecalculateWallBricks`、`WallBricksChanged`、`WallBrickBuildError`、`WallBrickBuildState`、`WallBrickRequests`、`build_wall_bricks`、`finalize_wall_holes`、`construct_wall_bricks` |
| [decorator_wall_events](../../src/systems/wall/decorator_wall_events.rs) | 墙发布 → 装饰存储迁移衔接（RE-DECORATOR-STORAGE §5，O 链）。 | `decorator_wall_events`、`decorator_wall_change_pending` |
| [delete_wall](../../src/systems/wall/delete_wall.rs) | B 0x140aac1e0: publication has already removed the public geometry record; this consumer removes the roof association and despawns wall/plaster entities. | `delete_wall` |
| [history_recording](../../src/systems/wall/history_recording.rs) | Wall compound records enter the same history transaction as roof/raster. | `RecordWallHistory`、`ColorHistoryRecord`、`RecordColorHistory`、`new_wall`、`deleted_wall`、`register_wall_recording_events`、`payload`、`record_color_history` 等 |
| [mod（本目录入口）](../../src/systems/wall/mod.rs) | Core wall consumers. These systems register no inferred original schedule. | `SetWallStyleCmd`、`SetFreehandHeightTypeCmd`、`SetPlasterPeelAmountCmd` |
| [registration](../../src/systems/wall/registration.rs) | Local CPU registration, not a reconstruction of the original global schedule. | `WallCpuStage`、`register_wall_cpu_resources`、`add_wall_cpu_systems`、`clear_wall_holes_for_next_frame` |
| [structure_outline](../../src/systems/wall/structure_outline.rs) | structure_outline.rs — 结构轮廓命令（组 A restorer 接线；BASELINE 参数面 `systems::wall::structure_outline::OutlineWallCmd` / `OutlineStairsCmd`—— system_color 的 ui_color_wall_and_roof 0x141c9ab70 / ui_color_windows_and_flags 0x1422b0e00 发射（拾取命中 → 结构轮廓高亮）。载荷语义 U）。 | `OutlineWallCmd`、`OutlineStairsCmd` |
| [wall_highlight](../../src/systems/wall/wall_highlight.rs) | B 0x140981390. Reset the kind, consume only the final unread command, then resolve the wall entity. Missing targets leave the previous IDs/color intact. | `HighlightWallCmd`、`wall_highlight` |

<a id="systems-wall-eraser"></a>
| [wall_height_command](../../src/systems/wall/height_command.rs) | Original B AdjustWallHeightCmd consumer after authority/public/query owners are resolved. The command's observed flag controls audio policy; this system has no History/AppMode parameter and does not record a history edit itself. | `AdjustWallHeightRequest`、`ResolvedWallHeightClamp`、`WallHeightFeedback`、`WallHeightCommandOutcome`、`WallHeightCommandError`、`from_observed`、`apply_resolved_height_command` |


## systems/wall/eraser

- **功能**：擦墙/推土机和拆除动画。
- **设计**：编辑、备份归档、UI 和视觉变化分开。
- **业务**：删除墙并保留历史恢复所需状态。

| 源码模块 | 用途线索（来自源码） | 主要类型 / 接口 |
|---|---|---|
| [animate_wall_bulldoze](../../src/systems/wall/eraser/animate_wall_bulldoze.rs) | B `systems/wall/eraser/animate_wall_bulldoze.rs`: wall bulldoze animation. | `AnimateWallBulldozeBuffer`、`new`、`is_empty`、`contains`、`push`、`retain_finished`、`animate_wall_bulldoze` |
| [archivist](../../src/systems/wall/eraser/archivist.rs) | B `systems/wall/eraser/archivist.rs`: wall eraser archivist（3-map 装配）。 | `OriginalWallId`、`RewireEnclosedWall`、`ErasedRange`、`EraserWallState`、`ChildWalls`、`WallEraserArchivist`、`WallHistoryEdits`、`remove` 等 |
| [mod（本目录入口）](../../src/systems/wall/eraser/mod.rs) | B `systems/wall/eraser/mod.rs`: wall eraser submodule root. | `AnimateWallBulldozeCmd`、`WallBulldozeAudio`、`WallEraserStroke` |
| [ui_bulldozer](../../src/systems/wall/eraser/ui_bulldozer.rs) | B `systems/wall/eraser/ui_bulldozer.rs`: bulldozer UI entry. | `BulldozerRayPick`、`is_bulldozer_layer`、`wall_eraser_bulldoze_condition`、`bulldoze_signifier`、`wall_eraser_bulldoze`、`enqueue_bulldoze_command`、`hammer_cursor_icon` |
| [wall_eraser](../../src/systems/wall/eraser/wall_eraser.rs) | B `systems/wall/eraser/wall_eraser.rs`: freehand wall eraser. | `wall_eraser` |

<a id="systems-wall-shapes"></a>
| [wall_bulldoze](../../src/systems/wall/eraser/bulldoze.rs) | B resolved wall-bulldoze business: actual snapshot groups and animation. UI hit-testing/selection supplies the observed wall, slice range and anchor. | `WallBulldozeHistory`、`WallBulldozeCommand`、`WallBulldozeOutcome`、`record_wall_bulldoze`、`advance` |


## systems/wall/shapes

- **功能**：规则形状创建和编辑。
- **设计**：共享放置/高度算法；圆和矩形各自状态/控件。
- **业务**：拖拽生成形状、调整尺寸/方向并提交。

| 源码模块 | 用途线索（来自源码） | 主要类型 / 接口 |
|---|---|---|
| [calc_shape_height](../../src/systems/wall/shapes/calc_shape_height.rs) | B `systems/wall/shapes/calc_shape_height.rs`: shape preview height. | `TimberframeConstructionParamsCache`、`calc_shape_height` |
| [mod（本目录入口）](../../src/systems/wall/shapes/mod.rs) | B `systems/wall/shapes/mod.rs`: shape submodule root. | `SelectedWallShape`、`PreviewCircle`、`PreviewRectangle`、`ShapePlacement`、`CalcShapeHeight`、`CreateCircleCmd`、`CreateRectangleCmd`、`EditCircleCmd` 等 |
| [shape_placement](../../src/systems/wall/shapes/shape_placement.rs) | B `systems/wall/shapes/shape_placement.rs`: shape placement parameters. | `RoofRidgeSnap`、`new_unsnapped`、`shape_placement_params`、`snap_to_roof_ridge` |
| [shape_placement_signifiers](../../src/systems/wall/shapes/shape_placement_signifiers.rs) | B `systems/wall/shapes/shape_placement_signifiers.rs`: placement preview. | `preview_snapping_line`、`shape_placement_signifiers` |

<a id="systems-wall-shapes-circle"></a>
| [wall_authoring](../../src/systems/wall/shapes/authoring.rs) | Original B shape creation inputs and InnerWalls authority mutations. std owns IDs/maps/dirty sets; geometry uses the existing recovered helpers. UI acceptance and selected building colors remain separate real consumers. | `AuthoringShape`、`CreationSettingsObserved`、`PreviousBuildingObserved`、`SelectedCreationAppearance`、`CreateShapeRequest`、`AuthoredWall`、`AuthoredBuilding`、`RectangleEditOutcome` 等 |


## systems/wall/shapes/circle

- **功能**：圆形墙创建和编辑。
- **设计**：create、UI create/edit/rotate 与 signifier 分开。
- **业务**：拖拽圆心/半径，旋转并更新圆形墙。

| 源码模块 | 用途线索（来自源码） | 主要类型 / 接口 |
|---|---|---|
| [create_circle](../../src/systems/wall/shapes/circle/create_circle.rs) | B `systems/wall/shapes/circle/create_circle.rs`: circle wall creation. | `circle_roof_height_01`、`create_circle` |
| [mod（本目录入口）](../../src/systems/wall/shapes/circle/mod.rs) | B `systems/wall/shapes/circle/mod.rs`: circle shape submodule root. | 内部实现 / 模块装配（见源码） |
| [ui_create_circle](../../src/systems/wall/shapes/circle/ui_create_circle.rs) | B `systems/wall/shapes/circle/ui_create_circle.rs`: circle drawing preview. | `CircleCanvasInput`、`CircleSignifierEnv`、`circle_signifier`、`circle_signifier_curve`、`ui_create_circle` |
| [ui_edit_circle](../../src/systems/wall/shapes/circle/ui_edit_circle.rs) | B `systems/wall/shapes/circle/ui_edit_circle.rs`: circle radius editing. | `CircleEditState`、`show_full_circle_signifier`、`ui_edit_circle` |
| [ui_rotate_circle](../../src/systems/wall/shapes/circle/ui_rotate_circle.rs) | B `systems/wall/shapes/circle/ui_rotate_circle.rs`: circle wall rotation. | `rotate_circle_wall`、`ui_rotate_circle` |

<a id="systems-wall-shapes-circle-signifiers"></a>


## systems/wall/shapes/circle/signifiers

- **功能**：圆形编辑的线和箭头标记。
- **设计**：标记几何与实体宿主边界分开。
- **业务**：提示当前半径、方向和拖动入口。

| 源码模块 | 用途线索（来自源码） | 主要类型 / 接口 |
|---|---|---|
| [edit_arrow](../../src/systems/wall/shapes/circle/signifiers/edit_arrow.rs) | B `systems/wall/shapes/circle/signifiers/edit_arrow.rs`: circle edit arrow. | `signifier_edit_circle_arrow`、`register_required_components`、`clone_behavior` |
| [edit_line](../../src/systems/wall/shapes/circle/signifiers/edit_line.rs) | B `systems/wall/shapes/circle/signifiers/edit_line.rs`: circle radius line. | `signifier_edit_circle_line`、`register_required_components` |
| [mod（本目录入口）](../../src/systems/wall/shapes/circle/signifiers/mod.rs) | B `systems/wall/shapes/circle/signifiers/mod.rs`: circle edit signifiers. | 内部实现 / 模块装配（见源码） |

<a id="systems-wall-shapes-rectangle"></a>


## systems/wall/shapes/rectangle

- **功能**：矩形墙创建、尺寸/方向编辑和结束处理。
- **设计**：创建、编辑、finish 与尺寸/方向 UI 分离。
- **业务**：拖拽矩形、调整边长或朝向，再结束编辑。

| 源码模块 | 用途线索（来自源码） | 主要类型 / 接口 |
|---|---|---|
| [create_rectangle](../../src/systems/wall/shapes/rectangle/create_rectangle.rs) | B `systems/wall/shapes/rectangle/create_rectangle.rs`: rectangle wall creation. | `ridge_length_01_from_rectangle_ratio`、`rectangle_roof_height_01`、`create_rectangle` |
| [edit_rectangle](../../src/systems/wall/shapes/rectangle/edit_rectangle.rs) | B `systems/wall/shapes/rectangle/edit_rectangle.rs`: rectangle edit apply. | `edit_rectangle` |
| [finish_edit_rectangle](../../src/systems/wall/shapes/rectangle/finish_edit_rectangle.rs) | B `systems/wall/shapes/rectangle/finish_edit_rectangle.rs`: rectangle edit finish. | `RectangleEditUiState`、`finish_edit_rectangle` |
| [mod（本目录入口）](../../src/systems/wall/shapes/rectangle/mod.rs) | B `systems/wall/shapes/rectangle/mod.rs`: rectangle shape submodule root. | `get_rectangle_orientation` |
| [signifier_edit_dims_line](../../src/systems/wall/shapes/rectangle/signifier_edit_dims_line.rs) | B `systems/wall/shapes/rectangle/signifier_edit_dims_line.rs`: dims edit line signifier (`rectangle_edit_dims_line_` prefix). | `signifier_edit_dims`、`register_required_components` |
| [ui_create_rectangle](../../src/systems/wall/shapes/rectangle/ui_create_rectangle.rs) | B `systems/wall/shapes/rectangle/ui_create_rectangle.rs`: rectangle drawing preview. | `RectangleCanvasInput`、`RectangleSignifierEnv`、`rectangle_signifier`、`rectangle_signifier_curve`、`ui_create_rectangle` |
| [ui_edit_rectangle_dims](../../src/systems/wall/shapes/rectangle/ui_edit_rectangle_dims.rs) | B `systems/wall/shapes/rectangle/ui_edit_rectangle_dims.rs`: rectangle dimension editing. | `RectangleDimsEditInput`、`distance_to_edge`、`do_ui_edit_rectangle_dims`、`ui_edit_rectangle_dims` |
| [ui_edit_rectangle_orientation](../../src/systems/wall/shapes/rectangle/ui_edit_rectangle_orientation.rs) | B `systems/wall/shapes/rectangle/ui_edit_rectangle_orientation.rs`: rectangle orientation editing. | `RectangleOrientationEditInput`、`skewed_box_trigger_vol`、`box_trigger_volume`、`do_ui_edit_rectangle_rotate`、`ui_edit_rectangle_orientation` |

<a id="systems-wall-shapes-rectangle-ui-modes"></a>
| [wall_rectangle_edit](../../src/systems/wall/shapes/rectangle/edit_decision.rs) | B finish_edit_rectangle business decision and actual enclosure owner. EventReader cursor/Bevy writes and DecoratorArchivist storage remain owners. | `RectangleEditState`、`RectangleFinishDecision`、`RectangleFinishError`、`tag`、`wall`、`finish_rectangle_edit_decision` |


## systems/wall/shapes/rectangle/ui_modes

- **功能**：矩形编辑子模式。
- **设计**：尺寸与方向控件分离。
- **业务**：将拖动意图转换为尺寸或朝向修改。

| 源码模块 | 用途线索（来自源码） | 主要类型 / 接口 |
|---|---|---|
| [edit_dims](../../src/systems/wall/shapes/rectangle/ui_modes/edit_dims.rs) | B `systems/wall/shapes/rectangle/ui_modes/edit_dims.rs`: dims edit widget. | `ui_rectangle_dims_edit_signifier`、`ui_rectangle_dims_edit_widget` |
| [edit_orientation](../../src/systems/wall/shapes/rectangle/ui_modes/edit_orientation.rs) | B `systems/wall/shapes/rectangle/ui_modes/edit_orientation.rs`: orientation edit widget. | `ui_rectangle_orientation_edit_widget`、`ui_rectangle_orientation_edit_signifier` |
| [mod（本目录入口）](../../src/systems/wall/shapes/rectangle/ui_modes/mod.rs) | B `systems/wall/shapes/rectangle/ui_modes/mod.rs`: rectangle edit widgets. | 内部实现 / 模块装配（见源码） |

<a id="systems-wall-shapes-utils"></a>


## systems/wall/shapes/utils

- **功能**：形状操作标记插值。
- **设计**：共享插值辅助。
- **业务**：使形状编辑提示随参数连续变化。

| 源码模块 | 用途线索（来自源码） | 主要类型 / 接口 |
|---|---|---|
| [mod（本目录入口）](../../src/systems/wall/shapes/utils/mod.rs) | B `systems/wall/shapes/utils/mod.rs`: shape utility submodule. | 内部实现 / 模块装配（见源码） |
| [shape_signifier_lerp](../../src/systems/wall/shapes/utils/shape_signifier_lerp.rs) | B `systems/wall/shapes/utils/shape_signifier_lerp.rs`: shape signifier interpolation. | `ShapeVal`、`get_radius`、`get_rectangle_dims`、`tag`、`lerp_shape` |

<a id="systems-wall-snapping"></a>


## systems/wall/snapping

- **功能**：绘墙吸附及提示。
- **设计**：初始化、维持、端点/线段检测和提示音分开。
- **业务**：自由绘墙靠近既有几何时对齐并提示合并。

| 源码模块 | 用途线索（来自源码） | 主要类型 / 接口 |
|---|---|---|
| [mod（本目录入口）](../../src/systems/wall/snapping/mod.rs) | B `systems/wall/snapping/mod.rs`: wall-end/segment snapping mode resource. | `SnappingModePublished`、`SnappingMode`、`DisableSnapping`、`new`、`is_active`、`position_to_snap_to` |
| [snapping_audio](../../src/systems/wall/snapping/snapping_audio.rs) | B `systems/wall/snapping/snapping_audio.rs`: snap sound dedup/emission. | `snapping_audio` |
| [snapping_init](../../src/systems/wall/snapping/snapping_init.rs) | B `systems/wall/snapping/snapping_init.rs`: segment-snap candidate setup. | `SnappingDebugDisabled`、`snapping_init` |
| [snapping_maintain](../../src/systems/wall/snapping/snapping_maintain.rs) | B `systems/wall/snapping/snapping_maintain.rs`: mode maintenance, disable-notification and the snapping preview signifiers. | `DisableSnappingInput`、`check_for_disable_snapping`、`snapping_maintain`、`snapping_signifiers` |
| [snapping_to_ends](../../src/systems/wall/snapping/snapping_to_ends.rs) | B `systems/wall/snapping/snapping_to_ends.rs`: wall-end snapping. | `snapping_to_ends` |
| [snapping_to_segments](../../src/systems/wall/snapping/snapping_to_segments.rs) | B `systems/wall/snapping/snapping_to_segments.rs`: wall segment snapping. | `snapping_to_segments` |
| [telegraph_walls_are_merged](../../src/systems/wall/snapping/telegraph_walls_are_merged.rs) | B `systems/wall/snapping/telegraph_walls_are_merged.rs`: merge preview line. | `TelegraphTimers`、`new`、`telegraph_walls_are_merged` |

<a id="systems-wall-valid-enclosure"></a>


## systems/wall/valid_enclosure

- **功能**：围合有效性。
- **设计**：曲线/结构几何判定。
- **业务**：判断墙是否形成有效封闭结构。

| 源码模块 | 用途线索（来自源码） | 主要类型 / 接口 |
|---|---|---|
| [mod（本目录入口）](../../src/systems/wall/valid_enclosure/mod.rs) | B `systems/wall/valid_enclosure/mod.rs`: premade-shape enclosure parameters. | `Shape`、`ShapeParameters`、`IsEnclosed`、`tag`、`center`、`perimeter`、`get_pos_at_u`、`get_normal_at_u` 等 |

<a id="systems-wall-wall-state"></a>


## systems/wall/wall_state

- **功能**：墙公开状态、空间索引和屋顶关联发布。
- **设计**：派生缓存消费权威变更。
- **业务**：让空间查询、屋顶和构造系统读取新墙状态。

| 源码模块 | 用途线索（来自源码） | 主要类型 / 接口 |
|---|---|---|
| [mod（本目录入口）](../../src/systems/wall/wall_state/mod.rs) | 未提供独立用途总述；通过下列源码类型/接口定位。职责按本节领域合同理解，不据文件名推断完成度。 | 内部实现 / 模块装配（见源码） |
| [publish_public_wall_state](../../src/systems/wall/wall_state/publish_public_wall_state.rs) | CPU ownership and real ECS publication consumers for B 0x140a8b530. The ordinary pipeline consumes the named WorldRaster buffers and owns one PublicWalls resource. The former isolated CPU API is a compatibility facade. | `WallPublicationTerrain`、`WallPublicationState`、`WallPublicationSubscriptions`、`WallPathRasterSources`、`WallPathRasterCache`、`get`、`failures`、`consume` 等 |
| [wall_roof_hashmap](../../src/systems/wall/wall_state/wall_roof_hashmap.rs) | 墙↔屋顶关联事件（systems/wall/wall_state/wall_roof_hashmap.rs）。 | `build_wall_roof_hashmap` |
| [wall_spatial_hash](../../src/systems/wall/wall_state/wall_spatial_hash.rs) | B wall spatial membership and segment data. Standard containers replace only dependency storage; endpoint projection and cell coverage retain game policy. | `WallSpatialSegment`、`WallSpatialHash`、`terrain_height_closest_to_ws_pos`、`remove_wall_id`、`add_segment`、`segments_for_wall`、`segments_in_cell`、`candidates` 等 |

<a id="utils"></a>
| [wall_height_authority](../../src/systems/wall/wall_state/height_authority.rs) | B InnerWalls::update_wall_state_height's successful game row mutation. std HashMap lookup/dirty storage and Bevy resource change ticks belong to actual dependencies/callers. This owner does not clamp or compare heights. Source+54 is the editable height inpu… | `WallHeightAuthorityOutcome`、`assign_resolved_height`、`write_resolved_source_height` |
| [wall_hole_invalidation](../../src/systems/wall/wall_state/hole_invalidation.rs) | Actual RecalculateWallHoles reader in the original wall publisher. InnerWalls lookup uses standard owner APIs; duplicates remain ordered. | `forward_recalculation` |
| [wall_path_preparation](../../src/systems/wall/wall_state/path_preparation.rs) | B sample_wall_path_intersections: explicit four encoded path raster owners. Uses actual selected buffers for attenuation, smoothing and threshold cuts. | `PathRasters512`、`PathPreparationError`、`PreparedWallPath`、`from_encoded`、`encoded`、`is_empty`、`sample_sum`、`prepare_wall_path` |
| [wall_pending_changed](../../src/systems/wall/wall_state/pending_changed.rs) | Actual PendingWallChanged.try_new and history_record_wall_changed producers. | `PendingWallChanged`、`PendingWallChangedError`、`approximate_curve_coordinate`、`approximate_curve_u`、`try_pending_wall_changed`、`take_pending_wall_changed` |
| [wall_publication](../../src/systems/wall/wall_state/publication_policy.rs) | Recovered game publication reductions and fields; Arc/Vec/hashbrown excluded. | `WallPublicationMetrics`、`WallPublicationCore`、`publication_metrics`、`write_projection` |


## utils

- **功能**：游戏侧公共辅助函数。
- **设计**：优先复用共享 utils；本地保留自有转换/规则。
- **业务**：脏格、屏幕投影、网格读取、音频辅助和调试绘制。

| 源码模块 | 用途线索（来自源码） | 主要类型 / 接口 |
|---|---|---|
| [audio_utils](../../src/utils/audio_utils.rs) | `audio_utils.rs` — `AudioLoop`：一圈 FMOD 事件实例的循环播放器（B `utils::audio_utils`）。 | `StopMode`、`LoopEntry`、`Timestamp`、`AudioLoop`、`new`、`current`、`start_if_not_playing`、`stop` |
| [child_wall](../../src/utils/child_wall.rs) | `country_core::utils::child_wall` — 子墙（父墙上的一段 `[a, b]` 参数区间）。 | `ChildWall`、`new`、`slice` |
| [conservative_line_raster](../../src/utils/conservative_line_raster.rs) | `conservative_line_raster::conservative_plot_line` — 带半径的保守线段栅格化。 | `conservative_plot_line` |
| [debug_viz](../../src/utils/debug_viz.rs) | `country_core::utils::debug_viz` — 调试绘制的构造器（builder）部分。 | `DebugColor`、`DebugSphere`、`DebugRectangle3d`、`DebugLine`、`tag`、`rgb`、`as_array`、`sphere_mesh_name` 等 |
| [dirty_tiles](../../src/utils/dirty_tiles.rs) | `country_core::utils::dirty_tiles` — 64×64 位的脏瓦片位图（0x200 B）。 | `DirtyTiles`、`DirtyTileRect`、`is_empty`、`world_rect_cells`、`cell_from_world_space`、`new_clear`、`new_all_dirty`、`from_rows` 等 |
| [float_grid](../../src/utils/float_grid.rs) | RasterBufferRef&lt;512&gt; consumer of shared B bilinear sampling policy. 0x140a14df0: byte values /255, original lerp order and edge extrapolation. Branch byte 0x143638fac (pointer slot 0x14362c308) is caller state. Its producer is `load_game` (`version &g…` | 内部实现 / 模块装配（见源码） |
| [geo](../../src/utils/geo.rs) | `country_core::utils::geo` — 曲线/点集到网格的几何辅助。 | `mesh_from_pts`、`update_curve_mesh`、`x0y`、`x_h_y`、`rewire_enclosed_curve` |
| [index_condenser](../../src/utils/index_condenser.rs) | Consumer adapter for the shared recovered index contract. B insert inline IPI0x74ee rejects occupied IDs; get/removal repair stale hints. The original typed marker remains local, while ID/state policy lives in the utils crate. | `IndexCondenser`、`new`、`len`、`is_empty`、`contains_id`、`insert`、`push`、`id_at` 等 |
| [load_json](../../src/utils/load_json.rs) | `country_core::utils::load_json` — JSON 网格加载器。 | `read_attribute_from_json`、`load_json_as_mesh`、`try_load_json_as_mesh` |
| [mod（本目录入口）](../../src/utils/mod.rs) | `utils::calculate_hash&lt;T: Hash&gt;`（O：0x1405bdda0 单态 `Sip13Rounds` = std `DefaultHasher`（SipHash-1-3，零密钥）；`render::utils::calculate_hash` 的同名实现）。 | `get_res`、`get_res_mut`、`drain_events`、`get_last_event`、`world_space_to_screen_space` |
| [project_cursor_onto_ray](../../src/utils/project_cursor_onto_ray.rs) | project_cursor_onto_ray.rs — 屏幕比例空间与“把光标投回射线”。 | `ProportionalScreenSpace`、`ProportionalScreenSpaceCoord`、`from_window_size`、`transform_screen_space`、`project_onto_ray` |

<a id="utils-components"></a>
| [dirty_tiles](../../src/utils/dirty_tiles.rs) | Original B collision DirtyTiles rectangle policy. The type is the one original `country_core::utils::dirty_tiles::DirtyTiles`; the collision code keeps its historical `DirtyTiles64` name for the same 64x64 bit grid. | 内部实现 / 模块装配（见源码） |


## utils/components

- **功能**：共享组件适配。
- **设计**：保持同一 Transform 实现/类型边界。
- **业务**：向 ECS 消费者提供变换行为。

| 源码模块 | 用途线索（来自源码） | 主要类型 / 接口 |
|---|---|---|
| [transform](../../src/utils/components/transform.rs) | utils/components/transform.rs — 游戏自有 `Transform`（WP6 恢复）。 | `Transform`、`identity`、`from_translation`、`with_translation`、`from_scale`、`from_translation_scale`、`compute_affine`、`compute_matrix` 等 |


## 内联模块和兼容路径

下面是源码中不对应独立文件的显式模块声明。它们的职责沿用所在文件；这里只登记位置，不把每个 trait impl 或泛型单态另算业务模块。测试内联模块不列。

| 所在文件 | 内联模块 | 角色 |
|---|---|---|
| [asset_libraries/texture_watch.rs](../../src/asset_libraries/texture_watch.rs) | `hotwatch_contract`、`hotwatch_contract` | 定义/装配/边界模块，具体行为见父文件和主 README |
| [lib.rs](../../src/lib.rs) | `typed_gap`、`systems` | 定义/装配/边界模块，具体行为见父文件和主 README |
| [render/load_texture.rs](../../src/render/load_texture.rs) | `boundary` | 定义/装配/边界模块，具体行为见父文件和主 README |
| [render/mod.rs](../../src/render/mod.rs) | `pass_builder` | 定义/装配/边界模块，具体行为见父文件和主 README |
| [render/pass_builder/draw_stream.rs](../../src/render/pass_builder/draw_stream.rs) | `test_host` | 定义/装配/边界模块，具体行为见父文件和主 README |
| [render/pass_builder/pass_resources.rs](../../src/render/pass_builder/pass_resources.rs) | `type_ids` | 定义/装配/边界模块，具体行为见父文件和主 README |
| [render/rg_helpers.rs](../../src/render/rg_helpers.rs) | `rg_boundary` | 定义/装配/边界模块，具体行为见父文件和主 README |
| [render/shader.rs](../../src/render/shader.rs) | `boundary` | 定义/装配/边界模块，具体行为见父文件和主 README |
| [render/ssbo.rs](../../src/render/ssbo.rs) | `rhapsody_boundary`、`usage`、`tests` | 定义/装配/边界模块，具体行为见父文件和主 README |
| [resources/mod.rs](../../src/resources/mod.rs) | `placement_data` | 定义/装配/边界模块，具体行为见父文件和主 README |
| [reused.rs](../../src/reused.rs) | `watch` | 定义/装配/边界模块，具体行为见父文件和主 README |
| [systems/history.rs](../../src/systems/history.rs) | `history_process` | 定义/装配/边界模块，具体行为见父文件和主 README |
| [systems/paths.rs](../../src/systems/paths.rs) | `path_brush_preview_size`、`draw_path`、`record_path_history` | 定义/装配/边界模块，具体行为见父文件和主 README |
| [systems/render_loop/compute_passes/pass_kit.rs](../../src/systems/render_loop/compute_passes/pass_kit.rs) | `binding` | 定义/装配/边界模块，具体行为见父文件和主 README |
| [systems/render_loop/draw_passes/draw_lists.rs](../../src/systems/render_loop/draw_passes/draw_lists.rs) | `list`、`bodies` | 定义/装配/边界模块，具体行为见父文件和主 README |
| [systems/render_loop/draw_passes/draw_shaders/startup_quad.rs](../../src/systems/render_loop/draw_passes/draw_shaders/startup_quad.rs) | `test_host` | 定义/装配/边界模块，具体行为见父文件和主 README |
| [systems/render_loop/draw_passes/main_pass.rs](../../src/systems/render_loop/draw_passes/main_pass.rs) | `names` | 定义/装配/边界模块，具体行为见父文件和主 README |
| [systems/render_loop/draw_passes/transparency_pass/surface_ice.rs](../../src/systems/render_loop/draw_passes/transparency_pass/surface_ice.rs) | `format` | 定义/装配/边界模块，具体行为见父文件和主 README |
| [systems/render_loop/draw_passes/transparency_pass/surface_support.rs](../../src/systems/render_loop/draw_passes/transparency_pass/surface_support.rs) | `names` | 定义/装配/边界模块，具体行为见父文件和主 README |
| [systems/render_loop/indirect_params.rs](../../src/systems/render_loop/indirect_params.rs) | `ssbo_ids` | 定义/装配/边界模块，具体行为见父文件和主 README |
| [systems/render_loop/render_frame.rs](../../src/systems/render_loop/render_frame.rs) | `msgs` | 定义/装配/边界模块，具体行为见父文件和主 README |
| [systems/render_loop/render_world.rs](../../src/systems/render_loop/render_world.rs) | `formats`、`names`、`panic_lines` | 定义/装配/边界模块，具体行为见父文件和主 README |
| [utils/mod.rs](../../src/utils/mod.rs) | `components` | 定义/装配/边界模块，具体行为见父文件和主 README |


## 实现与验证边界

本目录不是新证据报告，也不增加恢复进度。字段、接口名字和注释能帮助定位，但是否有真实 producer/consumer、删除/历史/保存链，仍须读函数体及 [核验合同](../../recovery/contracts.json)。

维护时先更新主 README 的设计/业务说明，再核对新增源码是否进入本索引；不要将编译、源码数量或本表行数作为原作等价的证明。

## resources/walls/decorator

按功能归属迁入的共享实现；证据和验收边界保持不变。

| 源码模块 | 用途线索（来自源码） | 主要类型 / 接口 |
|---|---|---|
| [decorator_info](../../src/resources/walls/decorator/anchors.rs) | UNVERIFIED B DecoratorInfo72 business-field decoding and placement source. Static original accessors/serializers/stores only; no new native proof yet. A uses destination tag+21; C uses+32 and payload+10. This is B-only layout. | `NativeDecoratorId`、`DecoratorInfoObserved`、`WallAttachmentAnchorObserved`、`WallAttachmentObserved`、`WallOpeningPolicy`、`DecoratorInfoError`、`DecoratorAnchorError`、`DecoratorAnchorContext` 等 |
| [decorator_placement](../../src/resources/walls/decorator/placement.rs) | B wall-space placement and collision metadata. Asset decoding and standard containers stay in their upstream libraries. Roof/corner/dormer attachment selection is deliberately a separate, unresolved consumer. | `DecoratorLibKey`、`CollisionBoundingBox`、`WallDecoratorRequest`、`WallDecoratorPlacement`、`PlacementError`、`metrics`、`wall_decorator_key`、`b_wall_collision_asset_name` 等 |


## resources/walls/decorator_subtype

按功能归属迁入的共享实现；证据和验收边界保持不变。

| 源码模块 | 用途线索（来自源码） | 主要类型 / 接口 |
|---|---|---|
| [decorator_subtype](../../src/resources/walls/decorator_subtype/placement.rs) | B wall destination subtype consumer. Actual roof and published wall caches are input; roof UV destinations and trigger construction are separate work. | `WallWindowKind`、`WallDecoratorSubtypeContext`、`SubtypeError`、`wall_opening_family`、`b_wall_subtype_collision_asset_name`、`nearest_segment`、`is_half_dormer`、`derive_wall_subtype` 等 |


## systems/wall/construction

按功能归属迁入的共享实现；证据和验收边界保持不变。

| 源码模块 | 用途线索（来自源码） | 主要类型 / 接口 |
|---|---|---|
| [fences](../../src/systems/wall/construction/fences.rs) | UNVERIFIED B fence CPU owner. Original private game policy is recovered here; Perchance, vector math, allocation and Bevy transport remain dependencies. | `FenceError`、`FencePost`、`FenceCpuOutput`、`FenceSegmentInput`、`FenceConstructionInput`、`split_fence_angles`、`process_fence_segment`、`construct_fences_cpu` |
| [gates](../../src/systems/wall/construction/gates.rs) | Original gate CPU consumer. Game mesh, source fields and lifecycle policy are projected separately from Bevy commands and Country asset containers. This module is under recovery; whole-owner differential acceptance is pending. | `GateConstructionError`、`GateConstructionInput`、`GateMeshProjection`、`WoodenGateProjection`、`GateShapeProjection`、`GateCurve3`、`GateNaniProjection`、`GateCpuOutput` 等 |
| [wall_arches](../../src/systems/wall/construction/arches.rs) | B regular-wall arches: actual mask -&gt; profile -&gt; walker -&gt;96B CPU records. Reuse std/glam and the recovered mask/segment/source-ID owners. | `WallArchesError`、`RegularWallArchesInput`、`ConstructGateCmd120Projection`、`WallArchesOutput`、`emit_arch_brick`、`construct_regular_wall_arches` |
| [wall_arches_freehand](../../src/systems/wall/construction/freehand_arches.rs) | B freehand arches: actual mask -&gt; endpoint profile -&gt; walker -&gt; CPU records and the actual third-height/resmoothing cache consumers. | `FreehandWallArchesInput`、`construct_freehand_wall_arches` |
| [wall_construction_sources](../../src/systems/wall/construction/parameters.rs) | Game policy at the B construct_walls -&gt; WallConstructor::from_curve call. Prepared path ownership is explicit: initial terrain mapping and subsequent segmentation are different original PublicWallState fields. | `ConstructionRoof`、`CircleConstructionOptions`、`PreparedConstructionArrays`、`ConstructionSourceError`、`ConstructionTopPolicy`、`WallConstructionPolicy`、`PlainConstructionPolicy`、`construction_kind` 等 |
| [wall_constructor](../../src/systems/wall/construction/bricks.rs) | Original wall constructor hole consumer. std owns the proposal queue. Producer/96-byte emission remain separate links; no mesh defaults invented. | `WallBrick`、`WallConstructionError`、`WallRow`、`WallBrickInstance`、`WallEmissionContext`、`NativeRowHoleCandidates`、`observed_bytes`、`resolve_hole_overlap` 等 |
| [wall_platform_collision](../../src/systems/wall/construction/platform_collision.rs) | Platform-stair collision mesh identity policy from the B native owner. std owns SipHash; rapier/parry/slotmap own physics and storage algorithms. This module does not claim the original collider backend or query is restored. | `PlatformStairColliderKey`、`ColliderPose`、`PlatformStairIndexedMesh`、`PlatformStairColliderSpec`、`ColliderUpsertAction`、`PlatformStairCollisionBackend`、`PlatformStairOwnedCollider`、`PlatformColliderUpdate` 等 |
| [wall_platform_rapier_backend](../../src/systems/wall/construction/platform_rapier_backend.rs) | Actual dependency-backed platform collider storage and query adapter. Original private selection/lifecycle policy is in wall_platform_collision. DirtyTiles rasterization is a required caller-supplied owner, never defaulted. | `PlatformRapierHandle`、`PlatformColliderDiff`、`PlatformRapierError`、`PlatformRapierMesh`、`PlatformRapierBackend`、`PlatformRayHit`、`original_discriminant`、`new` 等 |
| [wall_platform_segmentation](../../src/systems/wall/construction/platform_segmentation.rs) | B platform stair thresholding between raw masks and segment consumers. This produces a separate processed mask list and step ranges; it neither overwrites raw calc masks nor writes the third construction-height array. | `WallPlatformStepRange`、`WallPlatformSegmentation`、`WallPlatformSegmentationError`、`split_platform_wall_masks` |
| [wall_platform_stair_rows](../../src/systems/wall/construction/platform_stair_rows.rs) | Actual platform stair render curves and shared96B wall producer consumer. Geometry/colliders/holes are separate already-proven original consumers. | `WallPlatformStairRowsError`、`WallPlatformStairFootprint`、`platform_stair_footprint`、`platform_stair_gpu_instance`、`platform_stair_rows` |
| [wall_platform_stairs](../../src/systems/wall/construction/platform_stairs.rs) | B actual platform stair consumers. Step ranges are local to the preceding mask; actual manual-stair blockers remove their horizontal neighbourhood. Shared materialization and wall instance producers remain shared owners. | `WallPlatformStairBlocker`、`WallPlatformStairSource`、`WallPlatformStairError`、`WallPlatformDecoratorObserved`、`WallPlatformStairCellSource`、`WallPlatformStairCell`、`WallPlatformStairGeometry`、`collect_platform_stair_blockers` 等 |
| [wall_production](../../src/systems/wall/construction/production.rs) | Original WallConstructor producer. Standard RNG/curve math are reused. Decoded construction kinds, top bounds and fence fit remain explicit inputs. | `PlainWallProductionInput`、`WallProductionError`、`WallTopBounds`、`WallProductionBranch`、`produce_plain_wall`、`produce_wall_kind`、`produce_wall` |
| [wall_segmentation](../../src/systems/wall/construction/segmentation.rs) | B game WallPathSegmentationMask producer and original local curve coordinates. Vec/HashMap/Arc ownership and LoopRange's modulo arithmetic reuse Rust std. | `WallSegmentationError`、`WallLoopRange`、`ThresholdedCurveCoord`、`WallPathSegmentationMask`、`WallSegmentationInput`、`WallPathMaterialStatsUpdate`、`for_closed`、`for_open` 等 |
| [wall_structures](../../src/systems/wall/construction/structures.rs) | B wall mask -&gt; materialized wall segments, before actual lintel consumers. This game owner reuses the recovered LoopRange/coordinate projection. | `WallStructuresError`、`WallSegmentSource`、`MaterializedWallSegment`、`LintelSupportSource`、`WallWoodenPlankInstance64B`、`materialize_wall_segments`、`prepare_lintel_support`、`rdp_support_indices` 等 |


## resources/walls/wall_holes

按功能归属迁入的共享实现；证据和验收边界保持不变。

| 源码模块 | 用途线索（来自源码） | 主要类型 / 接口 |
|---|---|---|
| [opening_rules](../../src/resources/walls/wall_holes/opening_rules.rs) | Decorator → wall-hole production after asset collision and wall placement resolution. Geometry assets, wall-space projection and visual/GPU builders remain separate consumers; this module never accepts precomputed holes. | `OpeningFamily`、`DecoratorCollisionMetrics`、`DecoratorOpeningInput`、`OpeningRuleError`、`generate_wall_opening` |
| [wall_holes](../../src/resources/walls/wall_holes/geometry.rs) | Game WallHoleStorage lifecycle and geometry policy. std owns hashing/sort/Vec. | `RawWallHole`、`FinalizedWallHole`、`PreviousWallHoles`、`HoleFingerprint`、`HoleStoragePhase`、`HoleStorageError`、`WallHoleStorage`、`raw_hole_hash` 等 |


## resources/walls/decorator_archivist

按功能归属迁入的共享实现；证据和验收边界保持不变。

| 源码模块 | 用途线索（来自源码） | 主要类型 / 接口 |
|---|---|---|
| [wall_decorator_archivist](../../src/resources/walls/decorator_archivist/changes.rs) | B DecoratorArchivist::extract_and_restore_to_storage and tracked_clone_from. Hash maps, sets, allocation and collision-handle drops use Rust/std owners. | `DecoratorStorageAddress`、`ObservedDecoratorInfo`、`TrackedDecorators`、`DecoratorStorage`、`DecoratorArchivist`、`DecoratorRestoreError`、`restore_archived_decorators` |


## 工作区 system-tree

按功能归属迁入的共享实现；证据和验收边界保持不变。

| 源码模块 | 用途线索（来自源码） | 主要类型 / 接口 |
|---|---|---|
| [tree](../../../systems/tree-tool/src/process_tree_cmds.rs) | system-tree::despawn_trees, B 0x1422686d0: the resolved deletion predicate. The caller supplies the original marker/query result and animation field. | `should_despawn_tree` |


## resources/world_raster

按功能归属迁入的共享实现；证据和验收边界保持不变。

| 源码模块 | 用途线索（来自源码） | 主要类型 / 接口 |
|---|---|---|
| [water](../../src/resources/world_raster/blend_policy.rs) | system-water CPU leaves: rolling audio bits and resolved raster-write callbacks. No world-raster constructor, guessed sampling grid, raycast or zeno replacement. | `WriteMode`、`frame_bit`、`apply_audio_bit`、`select_write_mode`、`coverage_to_alpha`、`brush_fixed`、`apply_write` |

