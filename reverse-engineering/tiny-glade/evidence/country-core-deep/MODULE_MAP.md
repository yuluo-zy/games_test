# country-core 功能模块地图（代码目录 → 功能）

数据：SOURCE_MAP.json（523 文件 / 3106 业务函数）+ chapters 章首文件作用。更新：2026-10-15。

## asset_libraries（12 文件 / 387 业务函数）

### asset_libraries/top（12 文件 / 387 业务函数）

- `asset_libraries/ssbo_library.rs` [334] — **类型化 SSBO 注册表**——`AssetSsboLibrary = FrozenMap<SsboId, Box<BufferState>>` + 按类型擦除的
- `asset_libraries/texture_library.rs` [13] — **纹理资产库**——`AssetTextureLibrary`（0xc8 B）：textures Vec + 名字表 `HashMap<NameHash, NamedTexture,
- `asset_libraries/mesh_library.rs` [12] — **网格资产库**——`AssetMeshLibrary`：`FrozenMap<Handle<Mesh>, Box<Mesh>>` 主表 + 名字表
- `asset_libraries/shader_library.rs` [6] — **着色器资产库**——`AssetShaderLibrary`（0xe0 B）：raster/compute 两张 `FrozenMap` + 两张名字表
- `asset_libraries/color_library.rs` [5] — **颜色库 SSBO**——`ColorLibrarySsbo`（37×Vec4 颜色槽，0x250 B）+ JSON 反序列化（槽名→下标）+ bevy 系统
- `asset_libraries/vao_library.rs` [4] — **顶点数组对象（VAO）资产库**——`AssetVAOLibrary`（0x60 B）：`HashMap<Handle<Vao>, VaoEntry>`（
- `asset_libraries/placement_data_library.rs` [3] — **放置数据库**——`AssetPlacementDataLibrary`（0x60 B）：表 A = `HashMap<Handle<PlacementDataSsbo>,
- `asset_libraries/audio_trigger_library.rs` [2] — **音频触发盒（AABB）库**——`AudioTriggerLibrary = [AudioTriggerCollection; 1]`（按 kind 索引的 SlotMap 数组），
- `asset_libraries/configwatch.rs` [2] — **配置热重载监视器**——`ConfigWatch { hotwatch: Option<GenericHotwatch>, pending: Arc<Mutex<Option<String>>> }`：
- `asset_libraries/meshwatch.rs` [2] — **网格文件热重载监视器**——`MeshWatch { watched: Arc<Mutex<HashSet<PathBuf>>>, watch_lock: Mutex<()>,
- `asset_libraries/mod.rs` [2] — **Handle 资产核心类型**——`Asset { name: String }`、`HandleId::new`（全局 `HANDLE_COUNT` 原子 0x143634cd0 取号）、
- `asset_libraries/texture_watch.rs` [2] — **纹理文件热重载监视器**——`TextureWatch { hotwatch: Option<GenericHotwatch>, changed:

## components（3 文件 / 78 业务函数）

### components/top（3 文件 / 78 业务函数）

- `components/mod.rs` [54] — **游戏组件集总模块：~45 个 ECS 组件类型（marker / 参数组件）。本文件主要业务面 = 每个组件的 `register_required_components`（Bevy required-components 接线）。组件类型清单已由 reconstruction mod.rs 按 WP6 恢复（含 typed gap）。**
- `components/drawable.rs` [19] — **nani 可绘制组件族：`NaniInstanceMeshDesc`（命名/图集 part 选择）、`NaniActiveInstance`(=InstanceKey)、`NaniSolidTexture`/`PrevTransform`/`AlbedoMap`/`AlphaMap`、`DrawableMeshBundle`。业务面 = 组件注册 + `cache_prev_object_transforms` 系统 + `From<MeshAtlasPart>`。**
- `components/transient_mesh.rs` [5] — **瞬时网格组件与注册表：`TransientMeshRegistry`（Mutex+待删 Vec）、`TransientMesh` 组件（持 Arc 注册表，Drop 时登记删除）。backend `delete_dropped_transient_meshes` 每帧排空释放 GPU 网格。**

## geometry（4 文件 / 13 业务函数）

### geometry/top（4 文件 / 13 业务函数）

- `geometry/instanced_wall.rs` [9] — **砖实例墙的几何载体与 GPU 上传**（MODULE_GAP 判 0% 覆盖，本节补齐）。`BrickSourceId`（墙体砖 vs 楼梯瞬态砖）、`InstancedWall::update`（换缓冲）、`upload_to_storage_buffer`（砖变换→SparseInstanceBuffer，stride 0x60）。布局 O 见各函数。
- `geometry/sphere.rs` [2] — **程序化球体几何（icosphere）**（MODULE_GAP 判 0% 覆盖，本节补齐）。`sphere::try_from` 从序列化数据构建细分球体，依赖 `hexasphere-12.0.0`（公开 crate，直接复用不逆向库实现）。
- `geometry/cube.rs` [1] — **待推断**（由章节 agent 按函数名+证据填写）
- `geometry/plane.rs` [1] — **待推断**（由章节 agent 按函数名+证据填写）

## input（4 文件 / 28 业务函数）

### input/top（4 文件 / 28 业务函数）

- `input/action_set.rs` [13] — **输入动作集状态机**（MainActionSet<A=Vec2, D=DigitalActionState> / ActionSetAction /
- `input/input_stream.rs` [10] — **有界输入录制环（trailer 回放）**。`InputStream`（64 B）= 两个 VecDeque
- `input/mod.rs` [4] — **活跃输入模式跟踪**。`ActiveInputModeTracker` = 0x20 B `{u64 @0, u32 @8, u64 @0x10,
- `input/panic_repro_state.rs` [1] — **崩溃复现状态机**。`PanicReproState::is_inactive` 检查首字节判别 == 0。

## render（37 文件 / 782 业务函数）

### render/top（37 文件 / 782 业务函数）

- `render/nani/gpu_world.rs` [152] — GpuWorld：CPU↔GPU 实例桥（27×0xc0 RwLock<Option<GpuWorldSubset>> 槽 + total_instance_count + Box<NaniDrawListParams> + 两个 TypedStorageBuffer）；OwnedGpuWorldSubset = 每 subset 的稀疏实例仓库（IndexCondenser + 紧凑 key 数组 + 类型化数据缓冲 + min/max 原子）。
- `render/pass_builder/compute_pass_builder.rs` [117] — ComputePass：400B compute pass 声明层（bindings / access_ranges / external_bindings / PassBuilder / UniformBinder / 着色器注册表 / 设备 / PassResources）+ dispatch 家族（模式 0..4）+ bind_texture（RG/受管/外部路径）。O 锚点见 src 头注。
- `render/pass_builder/pass_resource_access.rs` [117] — SSBO 访问登记三件套（read/write/write_concurrent_access_buffer<T>，53/37/9 实例 = 3 个泛型体）+ 纹理访问 trait（TextureAccess：RG 已导入 / 受管 Handle / TexturePreset / WriteTextureUnchecked，as_texture 返回 3 标签枚举）。
- `render/ssbo.rs` [113] — **GPU 存储缓冲包装原语**——`TypedStorageBuffer<T>`（40 B，GPU-only 元素缓冲）、`InstanceBuffer<T>`（48 B =
- `render/pass_builder/draw_builder.rs` [67] — DrawBuilder：0xf8B 按值消费的 draw 声明器（NoDropOnlyTake<DrawPacket> + Option<DynamicConstantsBinding> + &mut RasterPass + Arc<CompiledShaderPipeline> + flag@+0xf0），draw 种类 0/1/2/3/4/5/7/8，终结操作无返回值。
- `render/shadowed_buffer.rs` [39] — **CPU 影子 + 脏区间 GPU 刷新的双缓冲原语**——`ShadowedBuffer<T>`（72 B）= `DirtyIndexRange`（AtomicUsize
- `render/sparse_instance_buffer.rs` [31] — **GPU 驱动稀疏实例化**——`SparseInstanceBuffer<T>`（0x90 B，32 实例/块分块 + byte_map 活跃计数 +
- `render/pass_builder/shader_uri.rs` [29] — ShaderUri 解析（三次 memrchr：':'→entry、'/'→文件名、'.'→stem，无点 unwrap）+ AsComputeShaderSource trait（&str/String 通用实现 + 18 个游戏实现：9 个 nani cull shaders + DoF×5 + bin_lights + GaussBlur + TgsrMain + GenerateStrokes）。
- `render/mesh.rs` [26] — **CPU 网格（Mesh）+ 顶点属性值（VertexAttributeValues）**——`Mesh`（0x78 = 120 B）：索引 Vec<u32> +
- `render/nani/systems.rs` [22] — nani ECS 系统族：nani_maintain_instances（删→建→改三阶段）+ 20 个 nani_update_*（组件→实例载荷打包 + update_instance_data）+ nani_flush_instance_updates（27 槽冲刷 + 计数滚动）。CPU 侧全部 O。
- `render/pass_builder/uniform_binder.rs` [15] — UniformBinder：uniform 块映射内存 + 按 64 位名字哈希查偏移写值（四道守卫静默返回；bool 写 u32；越界 slice_start_index_len_fail）。
- `render/render_config.rs` [6] — **待推断**（由章节 agent 按函数名+证据填写）
- `render/shader.rs` [6] — **着色器程序加载/缓存/变体面**——`ComputeShaderProgram`（0xa0 B）、`RasterShaderProgram`（0xd8 B）、
- `render/nani/instance_data.rs` [6] — 每类 nani 实例的 GPU 布局类型：元素尺寸 O（0x30/0x40/0x60/0x70 分组），字段布局 U（归 nani_update_* 与 shader 侧）；CurvedHalfTimberInstanceData 三个构造器逐字节 O；WindowGlassFlags / NaniUberInstanceFlags / CurvedHalfTimberCaps 位域。
- `render/nani/trimesh.rs` [5] — Trimesh / TrimeshBuilder：CPU 三角网格增量构造（≤64 三角形 meshlet 切块、属性对齐降序、build 打包 GPU 缓冲并算包围球）。
- `render/pass_builder/draw_stream.rs` [5] — DrawPacket（0xb8B 绘制包，种类判别@+0x40）/ RasterPass（~0x3e8B 光栅 pass：附件数组 + RenderPass 去重缓存）/ DrawStream（无过程空壳）+ TextureBindingKey（u64：0x4000_0000_0000_0000|id，视图异或 calculate_hash）。
- `render/pass_builder/bind_resources.rs` [4] — BindGameResources trait：把颜色库（ColorLibrarySsbo）与软件 BVH 组（RtTriangles/RtTriangleMeta/RtBvh + WallColorIds）绑定进 ComputePass / DrawBuilder（O：0x1405896e0 / 0x1415fc230 / 0x1416286f0 / 0x1416289d0；B_i_api §3 复核槽名与哈希）。
- `render/raster_single.rs` [3] — **待推断**（由章节 agent 按函数名+证据填写）
- `render/nani/mesh_atlas.rs` [3] — MeshAtlas / MeshAtlasPart：把多个 Mesh 合并进一张大网格（≤0x40 part；属性名集一致断言；重复名 panic；包围半径）。
- `render/atmosphere.rs` [2] — **太阳/天空调色板**——`SunSkyColors`（48 B = 3×Vec4）+ `DerivedAtmosphere`（4×Vec3 序列化类型）。
- `render/directional_light.rs` [2] — **方向光**——`DirectionalLight`（~0x78 B）：look-at 视图基 + 位置/目标/距离，`look_at` 建基、
- `render/view_constants.rs` [2] — **视图常量 + 视口常量构建器**——`ViewConstants`（~0x320 B：4×4 矩阵链 + 像素偏移 + 视口尺寸 +
- `render/nani/buffers.rs` [2] — nani 绘制参数：MAX_DRAWS_PER_LIST 27 项表（0x142aa14c0，与 max_instances 两张表逐值相同）+ NaniDrawListParams（0x70B = 4 Vec<u32> + 4 u32 汇总）+ NaniDrawListOffsetDefaults（0x1b0B = [u32;108]）+ 7 个 SSBO 载体。
- `render/frame_constants.rs` [1] — **帧常量 UBO 载荷**——`FrameConstants`（主视图 + 太阳级联视图，逐帧写入 `frame_constants` UBO
- `render/fsr_plugin.rs` [1] — **FidelityFX Super Resolution 的 bevy 插件接线**——PDB 仅 1 个过程 `plugin_impl::impl$0::build`，
- `render/load_texture.rs` [1] — **编译纹理镜像加载器**——`load_compiled_texture_image(path, …) -> Result<Texture, _>`：拼编译镜像路径 →
- `render/mod.rs` [1] — **render 模块根**——模块装配 + `GraphicsDeviceResource`（`Arc<GraphicsDevice>` bevy 资源）+
- `render/vao.rs` [1] — **顶点数组对象（Vao/VaoBinding）+ `build_vao`**——从属性表构建 GPU 顶点缓冲 + 绑定列表。
- `render/nani/draw_ext.rs` [1] — NaniDrawExt：DrawBuilder 的 nani GPU-driven 间接绘制扩展（nani_draw 0x140904ae0，1428B：subset 读锁 → list_index=kind*4|变体 → 三个间接 SSBO 绑定 → count/raw 分派）。
- `render/pass_builder/framebuffer_binding.rs` [1] — ColorAttachment（0x20B 枚举：tag0 引用@+8 / tag1 视图字@+4 引用@+0x18；访问 0x36/0x2e）/ DepthAttachment（tag0 可写 0x2f / tag1 只读 0x17 / 其它无深度；+0x10 清除 u32、+0x14 f32）。
- `render/pass_builder/pass_resources.rs` [1] — PassResources：PassBuilder::render 的 UserData（56B = 7 个帧级资源指针；new_from 按 7 个 128 位 TypeId 查 ECS world 资源表）。
- `render/adaptive_render_state.rs` [胶水/内联] — **待推断**（由章节 agent 按函数名+证据填写）
- `render/dirty_index_range.rs` [胶水/内联] — **待推断**（由章节 agent 按函数名+证据填写）
- `render/indirect.rs` [胶水/内联] — **间接绘制/分派命令包装**——`DrawIndexedIndirectCommand`（20 B = `VkDrawIndexedIndirectCommand`）、
- `render/nani.rs` [胶水/内联] — **待推断**（由章节 agent 按函数名+证据填写）
- `render/uniform.rs` [胶水/内联] — **待推断**（由章节 agent 按函数名+证据填写）
- `render/pass_builder/draw_list.rs` [胶水/内联] — DrawListItem / EntityDrawParams：CPU draw-list 项（0x80B 元组 = 0x50B params 首字段 + vao + shader + i32 排序键 + 模板参考 + 回调 fn；无 PDB 独立过程，全部内联进宿主 pass）。

## resources（122 文件 / 828 业务函数）

### resources/active_raster_stroke.rs（1 文件 / 4 业务函数）

- `resources/active_raster_stroke.rs` [4] — **活动栅格笔画：ActiveRasterStroke 0x470B 记录（三列 Vec：固化曲线/进行中输入点/平滑后曲线 + 弧长 + 笔刷/模式/blend 标志 + 双脏位图）与 InProgressRasterStroke 流体输入状态机（add_point/solidify）。draw_raster 与 rasterize_terrain_stroke 的输入源。**

### resources/audio_resources.rs（1 文件 / 19 业务函数）

- `resources/audio_resources.rs` [19] — **音频核心状态：FMOD Studio 内核、银行表、AudioInstance 实例模型、全局失败报告注册表。**

### resources/audio_settings.rs（1 文件 / 1 业务函数）

- `resources/audio_settings.rs` [1] — **音频音量设置（GameConfig 内嵌，RON serde；4 个 f32 音量字段 O）。**

### resources/brush_sizes.rs（1 文件 / 6 业务函数）

- `resources/brush_sizes.rs` [6] — **笔刷尺寸资源：path@+0 / water@+4 / garden@+8 三个 f32 槽位，各自 set=clamp / reset=默认。draw_water/draw_path/draw_garden 读取对应槽位。**

### resources/clutter_meshes.rs（1 文件 / 11 业务函数）

- `resources/clutter_meshes.rs` [11] — **杂饰网格类型表：ClutterMeshTy 56 资产路径变体+删除标记组件**

### resources/color_icon.rs（1 文件 / 17 业务函数）

- `resources/color_icon.rs` [17] — **调色菜单图标组件族 + 亮度值枚举（三变体→{0.25,1.0,4.0} 表）。**

### resources/color_id.rs（1 文件 / 9 业务函数）

- `resources/color_id.rs` [9] — **颜色索引资源族：ColorId(u32)+墙/门/半木三表+SSBO 载体。**

### resources/color_id_roof.rs（1 文件 / 1 业务函数）

- `resources/color_id_roof.rs` [1] — **屋顶颜色 ID SSBO。**

### resources/critters（1 文件 / 11 业务函数）

- `resources/critters/sheep.rs` [11] — **Sheep 组件族（serde 存档+快照；业务体在 systems/critters）**

### resources/critters.rs（1 文件 / 22 业务函数）

- `resources/critters.rs` [22] — **小动物组件/资源族：Critter/Firefly/Butterfly/Duck/Wren/BirdSpotManager 等（CPU 状态+FMOD+池）**

### resources/critters/sheep（1 文件 / 3 业务函数）

- `resources/critters/sheep/control.rs` [3] — **SheepControlSnapshot/SheepSnapshot（serde 快照）**

### resources/cursor_terrain_raycast.rs（1 文件 / 1 业务函数）

- `resources/cursor_terrain_raycast.rs` [1] — **光标地形射线资源：{tag u8@0, pos Vec3@4}，Option<Vec3> 语义。地形类工具（water/garden/path/terrain）读取光标落点。**

### resources/daily.rs（1 文件 / 2 业务函数）

- `resources/daily.rs` [2] — **每日主题**——`DailyThemeInfo`：default 置两个 u64=0x8000000000000000 哨兵；

### resources/drawing_input_smoother.rs（1 文件 / 1 业务函数）

- `resources/drawing_input_smoother.rs` [1] — **绘制输入平滑器：把徒手原始输入折线做弧长均匀重采样 + 最小间距抽稀，输出 (x,z) 点对序列（drawing 类工具的输入预处理）。**

### resources/editing_highlight.rs（1 文件 / 1 业务函数）

- `resources/editing_highlight.rs` [1] — **编辑高亮资源（wall_highlight 写 rgba 的载体）。**

### resources/feedback_report_resources.rs（1 文件 / 6 业务函数）

- `resources/feedback_report_resources.rs` [6] — **反馈报告资源：截图 Arc 双槽+元数据区+状态机+选择字节。**

### resources/glade_border.rs（1 文件 / 3 业务函数）

- `resources/glade_border.rs` [3] — **小树林边界：GladeBorder 判定世界位置/形状是否在可编辑边界内（is_pos_inside=跳板→is_shape_inside 主体；依赖 utils Circle2d/CurveU——u∉[0,1) panic 'Tried to init CurveU from an invalid u: '）。**

### resources/glade_color_customization.rs（1 文件 / 3 业务函数）

- `resources/glade_color_customization.rs` [3] — **谷地自定义配色（serde 存档；默认字节 O）。**

### resources/glade_settings.rs（1 文件 / 2 业务函数）

- `resources/glade_settings.rs` [2] — **谷地设置存档**——`GladeSettings` 默认布局 {+0x0 u32=0、+0x8 f32=0.01、

### resources/gothic_window_profile.rs（1 文件 / 2 业务函数）

- `resources/gothic_window_profile.rs` [2] — **Gothic 窗/门拱剖面曲线（size 1/2/3→Vec<Vec2> 点列）。**

### resources/gpu_lut.rs（1 文件 / 3 业务函数）

- `resources/gpu_lut.rs` [3] — **GPU LUT 生成（brdf_fg/water_noise 两条目 tracker + 计算 pass）。**

### resources/grading_lut.rs（1 文件 / 8 业务函数）

- `resources/grading_lut.rs` [8] — **颜色分级 LUT：完整 .cube 解析器 + assets/luts 目录装载。**

### resources/gui.rs（1 文件 / 14 业务函数）

- `resources/gui.rs` [14] — GUI 状态资源族：EscMenuState 8B {sub u32@0, disc u32@4}（is_open: qword∉{2,0xa} 或 0xa 时 sub!=1；is_closed: qword==2 或 (0xa 且 sub!=0xff)；cast_ui_shadows: qword∈{3,4} 或 (disc7&&sub1)；should_show_world: 0xfe..0x106 状态位掩码 0xaf/0x50 判定；can_limit_frame_rate: !(disc7&&sub1)）；NextEscMenuState::set（哨兵 0xb；字符串@+0x30 释放 + 16B@+0x50 拷贝）、PrevEscMenuState::get（0xb→None）；LoadMenuThumbnailMetrics、VideoSettingsSubMenu、DebugGui::log_to_screen、ShowDebugRaycastGeo、DebugTriggerProxyVis。

### resources/history（12 文件 / 83 业务函数）

- `resources/history/mod.rs` [32] — 权威历史缓冲核心（S05）：History 0x78B 布局（edit_chain/snapshots/pending/计数/nonce/cursor/history_start）、undo/redo/录制/快照方法族、MultiEdit/HistoryEdit/DecoratorDiffTable/StairsDiffTable、HistoryChainIndex/HistoryEditChain、HistorySnapshotId/MaybePendingSnapshotId/HistoryChainSnapshots、EditType 与 MultiEdit 的 serde 巨函数（deserialize 合并区 0x140b3ad60-0x140b3bc83）。
- `resources/history/decorator_edit.rs` [9] — 装饰历史编辑域：DecoratorHistoryEdit/Diff/State、DecoratorHistoryState::From（0x140b3f930，preview 类型 panic）、is_default_* 谓词（lantern_brightness==stair_width==1.0f，0x140b3f910 ICF）、convert_before_after_states_to_changes（0x140b3fab0，按 style 字节分变体比较浮点输出变更集）、send_state_diff_events（0x1417bdae0，replay_pop_bucket 消费）、cull_allow_list；手写 serde visitor 族。
- `resources/history/wall_compound_edit.rs` [8] — 墙复合历史编辑域：WallHistoryEdit 操作枚举（ChangeFreehandHeightType/PlasterPeelAmount/ChangePlasterPeelAmount/MorphPremadeShape/MorphFreehandWall…，niche 判别首字段）、WallHistoryCompoundEdit（0x108B 负载）、ConcurrentEdits、CompoundEditBuilder（new_edit_sequence/next/build 0x140a82d50）、panic_deprecated_operation 0x140a82e20、update_compatibility（内联）、gather_wall_ids；手写 serde visitor 族（deserialize 0x141401960 等 ~40 变体）。
- `resources/history/roof_edit.rs` [6] — 屋顶编辑负载：RoofEdit 操作枚举（FlatRoofRimStyleChange/SwitchRoofRidgeDir 等 tag）手写 serde visitor 族（serialize 0x140259e30、deserialize 0x140b38b80、visit_str 0x140b392d0/0x140b39400/0x140b394f0/0x140b395d0）。重建侧缺此模块。
- `resources/history/terrain_edit.rs` [6] — 地形编辑负载：TerrainEdit（判别 0/1/2：New/Edit/Move stroke；duration=clamp(Σ点距*0.01,0.4,3.0) 内联、revert 0.99 进度阈值+stroke 表清 active、reapply、update_terrain_curve 0x140859740 hash 查表+点集拷贝+fold+全局时间 lerp）手写 serde（serialize 0x1404f41b0/0x1417c1590、deserialize 0x1417c1ac0、visit_str 0x1417c23e0）。
- `resources/history/tree_edit.rs` [5] — 树编辑负载：TreeEdit（Place/Move + tree{id,position,scale,rotation_degrees,prefab_name,is_preplaced}）、missing_prefab 0x1409e6210、TreeCanopyMaxCullDistance 手写 serde visitor 族（serialize 0x14003d4e0/0x1417c2770/0x1417c2600/0x1417c2a50、deserialize 0x1417c2f60）。
- `resources/history/replay_cmd_buffer.rs` [4] — 回放命令缓冲：ReplayBuffer（桶 Vec cap-first 元素 0x40 + 活跃计数）、ReplayBucket 0x40B（decorators/stairs Arc + cmds Vec 0x30B）、ReplayCmd 0x30B（判别@0/instant@8/duration@c/负载@10/elapsed@18/undo@1c）、pop_front/front_mut_bucket/make_animations_instant/len。
- `resources/history/color_edit.rs` [3] — 颜色编辑负载：ColorEdit（目标 Wall/Roof/Halftimber/Decorator/StairRailing/FlatRoof/StairLadder/Brightness + from/to + 目标 id；字段 chimney_h_offset/allow_connection）手写 serde visitor 族（serialize 0x14003c5d0 / deserialize 0x1417befe0，visit_str 0x1417bf8a0）。
- `resources/history/path_edit.rs` [3] — 路径/鹅卵石光栅编辑负载：RasterEdit（points/brush_size/is_additive/blend_mode；ReplayCmd 判别 0/1/2 的 Arc 负载）+ RasterEditSerialized 手写 serde（serialize 0x1404896f0/0x1416e8340、deserialize 0x1416e8510、From 0x140a976b0）。
- `resources/history/wall_history.rs` [3] — 墙历史事件接口：History 的 WallHistory trait 实现（ev_new_wall 0x140a83300：断言 curve 长度非零、构建 0x108B WallHistoryEdit + FUN_141696f50 校验 → History::add(tag 7)；ev_height_changed 0x140a83f80：EditType::new_height_changed（0x140a83fb0，niche+2 判别）→ History::add）。
- `resources/history/auto_clutter_edit.rs` [2] — 自动杂物编辑负载：AutoClutterEdit（墙派生装饰记录；host wall_id + decorator_ids）的手写 serde visitor 族（serialize 0x14003d300 / deserialize 0x140a18500）。
- `resources/history/stairs_history_edit.rs` [2] — 楼梯历史编辑负载：StairsHistoryEdit/StairsHistoryDiff（边对 before/after + hash；运行时 StairsDiffTable 96B = 两 Vec<u32> 边对 cap-first）手写 serde（serialize 0x140259af0、deserialize 0x140a18ee0）。

### resources/image_formation.rs（1 文件 / 7 业务函数）

- `resources/image_formation.rs` [7] — **画面形成配置：三档 0x40B config+当前档 kind@+0xc0（缺省档=1）。**

### resources/in_progress_curve.rs（1 文件 / 3 业务函数）

- `resources/in_progress_curve.rs` [3] — **编辑中曲线（徒手墙原始输入曲线）：raw 输入点累积、验证与终结曲线计算。wall 域（freehand 墙）为主消费者。**

### resources/ivy（4 文件 / 7 业务函数）

- `resources/ivy/ivy_storage.rs` [4] — **常春藤折线存储：Ivy(VecDeque<IvyPoint 40B>)+IvyStorage(HashMap+RNG)**
- `resources/ivy/ivy_direction_proposer.rs` [1] — **生长方向噪声提议器：双 FastNoise 混合→θ∈[-40°,+40°]**
- `resources/ivy/ivy_leaf_storage.rs` [1] — **常春藤叶存储：IvyLeaf 104B GPU 实例参数集**
- `resources/ivy/wall_coord.rs` [1] — **墙坐标→世界空间（PublicWallState 曲线采样+法向偏移）**

### resources/loading_screen.rs（1 文件 / 2 业务函数）

- `resources/loading_screen.rs` [2] — 启动加载屏：LoadingScreenPlugin（注册 LoadingScreenProgress + Startup/Update 系统）+ 加载屏网格与着色器。startup_loading_screen 加载 mesh "loading_screen"(hash 0x42ac8c61) + shader "loading_screen/loading_screen.raster.hlsl"并 spawn (DrawableMeshBundle, DesignatedRenderPass(0x12), DontCastShadow)；update_loading_screen 为内联函数（无独立符号），逐帧推进加载进度。

### resources/logo_animation.rs（1 文件 / 7 业务函数）

- `resources/logo_animation.rs` [7] — 动画 logo 资源与系统：5 阶段揭示状态机（progress[5]@+0..+0x14 + durations[5]@+0x14..+0x28），阶段 0 播 FMOD event:/UI/Logo/Scribble、阶段 2 播 event:/UI/Logo/Branches；wait/finish 握手（+0x0c 等待进度、+0x10 完成揭示）；startup_animated_logo 加载 tiny_glade_logo/logo_branches JSON 网格与 raster.hlsl 着色器。

### resources/mouse_capture.rs（1 文件 / 1 业务函数）

- `resources/mouse_capture.rs` [1] — 光标捕获命令处理：读 CursorCaptureCmd（utils::get_last_event 取最后一条）；未捕获→CursorCapturedPosition={1,pos}+WindowFocusEvent{focused=0}+EscMenuState{2,2}；捕获→{0}+{focused=1}+EscMenuState{3,2}。命令族 CursorCaptureCmd/OsCursorCaptureCmd/OsCursorVisibilityCmd/CustomCursorSsboCreatedCmd。

### resources/mouse_terrain_raycast.rs（1 文件 / 3 业务函数）

- `resources/mouse_terrain_raycast.rs` [3] — **鼠标→地形拾取链：CameraCursorRay {tag u32@0, pos@4, dir@0x10, length@0x1c} 相机光标射线 + camera_mouse_ray 屏幕→世界射线 + mouse_terrain_raycast 地形拾取（xz 平面快路径 / TerrainHeightsData::intersect_ray 高度场求交 / y=0 平面比距取近 / closure$0 实例网格精化）。**

### resources/onboarding.rs（1 文件 / 1 业务函数）

- `resources/onboarding.rs` [1] — 新手引导：WelcomeScreenState（is_closed = 判别==2，0x1409a9140 折叠处证实）+ NotifyHintSystemSnapping 事件 + TutorialOp 事件（tiny_glade settings 系统消费，PDB 注解证实）。

### resources/photo_mode.rs（1 文件 / 3 业务函数）

- `resources/photo_mode.rs` [3] — 拍照模式资源：PhotoModeCameraMode u8 {0..3}（next 0→1→2→3→0；next_demo 3→panic 'this camera mode is not part of the demo!'）；PhotoModeCmd 事件；PhotoModePlugin 注册；PhotomodeState::on_enable_photomode f32@0=3.0/1.0(demo)+u32@4=-1。与 CameraMode 槽 6..12 联动见 round2 CLOSURE（写入点在 toolbar::photomode，skeleton 缺该 crate）。

### resources/placement_data.rs（1 文件 / 3 业务函数）

- `resources/placement_data.rs` [3] — **放置数据 SSBO 元素（position/scale/angle→Affine3A）。**

### resources/prefab.rs（1 文件 / 4 业务函数）

- `resources/prefab.rs` [4] — prefab 库资源：hashbrown HashMap<PrefabName, Prefab>（RandomState；桶 0x28B = hash 8B + 键 16B + 值 16B；Prefab 值为 {ptr,len} 胖指针类——reconstruction 的 Vec 布局存疑）+ Bevy 组件注册。

### resources/preplaced_autoclutter.rs（1 文件 / 0 业务函数）

- `resources/preplaced_autoclutter.rs` [胶水/内联] — **预放置自动杂饰资源（default 内联，无独立过程）**

### resources/raster_stroke.rs（1 文件 / 2 业务函数）

- `resources/raster_stroke.rs` [2] — **栅格笔刷点集：RasterStroke {points: Arc<[Vec3]>, radius: f32} + iter_points_xz 迭代器。rasterize_stroke 的输入（Map<Iter<Vec3>, iter_points_xz>）。**

### resources/read_mouse_depth_value.rs（1 文件 / 1 业务函数）

- `resources/read_mouse_depth_value.rs` [1] — **鼠标深度回读：MouseDepthValue {w f32@0, xyz@+8, 帧哨兵@+0x40}，来自 MouseDepthValueSsbo（2×float4）的 GPU→CPU 映射回读。**

### resources/render（3 文件 / 16 业务函数）

- `resources/render/mesh_atlas_library.rs` [11] — **MeshAtlasLibrary 资源**——按"世界子集(world subset)→图集(atlas)→名称→网格"组织 GPU 网格图集，
- `resources/render/mod.rs` [4] — **Affine3Packed 打包仿射矩阵**——渲染用紧凑 4x3（48 字节）仿射表示，供 GPU 实例/图集使用。
- `resources/render/spatial_resolve_offsets.rs` [1] — **SpatialResolveOffsets 资源**——`default_value`（0x140b31d10）把一张 ~0x808 字节的偏移常量表

### resources/rocky_terrain.rs（1 文件 / 2 业务函数）

- `resources/rocky_terrain.rs` [2] — **岩质地层网格：RockyTerrainMesh 资源 + JSON 中间结构（LoadedRockyTerrainVertex/Triangle）→ 两个 SSBO（RockyTerrainVertexSSBO/TriangleSSBO），displace_rocky_terrain 每帧 GPU 变形。**

### resources/roofs（10 文件 / 58 业务函数）

- `resources/roofs/roof_shape.rs` [28] — **屋顶形状权威类型**——`Roof` 组件（0x58 布局）、`RoofShapeParams`（28 字节参数块）、
- `resources/roofs/roof_trigger_volume.rs` [7] — **RoofTriggerVolume 组件（屋顶触发体/碰撞网格）**——存三角网格（triangles/normals/uvs 三个 cap-first Vec），
- `resources/roofs/roof_ridge.rs` [6] — **屋顶脊线尺寸与几何**——`RoofRidgeDims`（脊线最小/指定轴尺寸）、局部尖顶中心、世界脊线点/线段分支、
- `resources/roofs/roof_visual_shape.rs` [6] — **屋顶可视形状查询族**——世界位置→height01/截面、扩展上下界（RoofBoundsDims）。
- `resources/roofs/roof_animation.rs` [5] — **RoofAnimation 组件**（屋顶瓦/结构动画状态，16 字节零化初值；`is_being_animated` 读首个 f32 字段）。
- `resources/roofs/roof_noise.rs` [2] — **屋顶顶点噪声位移**——`gpunoise` 是 shader 风格 2D value-noise 的 CPU 复刻（floor + 4 角 Wang 哈希 + smoothstep 插值），
- `resources/roofs/roof_tiles.rs` [2] — **RoofTile 组件（屋顶瓦片实例）**——本文件只含组件声明/注册面；
- `resources/roofs/roof_tile_sim.rs` [2] — **RoofTileSimulation 组件**（屋顶瓦片物理/坠落模拟状态）。`new` 构造：+0 状态 u32、+4 计时 f32=入参、
- `resources/roofs/commands.rs` [胶水/内联] — **屋顶编辑命令枚举**——`RoofCommand` 系（建造/修改/删除屋顶的指令，经 history roof_edit 执行）。
- `resources/roofs/dormer_tiles.rs` [胶水/内联] — **DormerTiles 资源/组件**（天窗瓦片集合，存屋顶天窗处生成的瓦片）。本文件仅 `impl$1::default` 一个派生胶水，

### resources/rt（3 文件 / 13 业务函数）

- `resources/rt/mod.rs` [11] — **RT 世界 CPU 侧**——`RtMesh`/`RtMeshHandle`/`RtWorld`（哈希表容器，key=WallId/RtMeshHandle）、
- `resources/rt/bvh.rs` [1] — **GPU BVH 构建**——`build_gpu_bvh(triangles, count)` 把三角数组构建成线性 BVH 节点数组（CPU→GPU 前的布局）。
- `resources/rt/bvh_builder.rs` [1] — **RtBvhBuilderGlue 后台构建器**——独立线程 worker 消费 RtWorld 脏网格列表，把脏三角构建为 GPU BVH。

### resources/session.rs（1 文件 / 28 业务函数）

- `resources/session.rs` [28] — 会话资源层：GladeType 主题判别（0=Summer/1=Olden/2=Flowery/3=Autumn/4=Winter，O）、SessionId 与保存目录协议（saves()/backups()/会话文件三件套 history.json+screenshot.jpg+meta.json）、SessionMeta 元数据（meta.json，serde 字段 {state,glade,mutators,image}）、NextSession 会话请求（0xa0B：on_load 命令 Vec + kind niche + uuid）、SavedSession(List) 会话列表/删除（deleted_saves 两段删除）。

### resources/shader_refactoring_state.rs（1 文件 / 1 业务函数）

- `resources/shader_refactoring_state.rs` [1] — **着色器重构开关状态。**

### resources/shape_intersections.rs（1 文件 / 0 业务函数）

- `resources/shape_intersections.rs` [胶水/内联] — **ShapeIntersections 标记/容器资源**——`ShapeIntersections::default`（行 8）唯一业务面是 `Default`。

### resources/shutdown.rs（1 文件 / 1 业务函数）

- `resources/shutdown.rs` [1] — **游戏关闭命令（bevy 系统）。**

### resources/stairs（7 文件 / 48 业务函数）

- `resources/stairs/stairs_state.rs` [27] — **权威楼梯结构图：petgraph GraphMap（@+0x28）+端点/脏边集合(@+0x148)/备份快照 diff（历史撤销用）**
- `resources/stairs/stairs_assembly_params.rs` [8] — **手工楼梯装配参数族：锚点/对齐/边标识（供 stairs_assemble 系统消费）**
- `resources/stairs/structural_graph.rs` [7] — **楼梯支撑碰撞体图：span↔collider 双向映射+支撑高+GraphMap（拱/托架装配的支撑结构）**
- `resources/stairs/visual.rs` [4] — **楼梯可视化资源：MagicSupports/StairsVisualState/地板 mesh 块**
- `resources/stairs/invalidate.rs` [1] — **脏楼梯伪影失效系统：脏边→撤碰撞体/RT 网格/视觉/装配缓存**
- `resources/stairs/stairs_removed_supports.rs` [1] — **已移除楼梯支撑存档集（serde）**
- `resources/stairs/stairs_collider.rs` [胶水/内联] — **楼梯碰撞体注册标记资源（rapier 边界，无独立函数）**

### resources/stairs_walls.rs（1 文件 / 1 业务函数）

- `resources/stairs_walls.rs` [1] — 楼梯-墙支柱提议（StairsPillarProposals）：楼梯与墙交界处的支柱放置提议集合（add 0x140901270）。

### resources/steam.rs（1 文件 / 9 业务函数）

- `resources/steam.rs` [9] — **Steam 动作集定义：GameControls 28 动作/Menu 2 动作/CottageWindow+GothicWindow 层。**

### resources/terrain_editing.rs（1 文件 / 6 业务函数）

- `resources/terrain_editing.rs` [6] — **地形编辑数据：TerrainEditingData {HashMap<TerrainStrokeId, TerrainStroke>（数据@+8/掩码@+0x10/计数@+0x20）, last_stroke_id u32@+0x38}；new_stroke 上限 0x3fffff。编辑笔画的生命周期管理。**

### resources/terrain_height.rs（1 文件 / 5 业务函数）

- `resources/terrain_height.rs` [5] — **地形高度场：TerrainHeightsData<T> 泛型资源双实例（TerrainHeightSsbo 权威 / VisualTerrainHeightSsbo 视觉）。u16 512×512 网格@+0x08，min/max f32@+0x50/54（默认 -2.5/24.0）；世界采样/射线求交/GPU 回拷差分。全地形系统的数据底座。**

### resources/terrain_rock_mask.rs（1 文件 / 1 业务函数）

- `resources/terrain_rock_mask.rs` [1] — **岩体掩码：512×512 位栅格（8192 u32 字，按位打包），世界范围 ±130。GPU 回拷 TerrainRockMaskSsbo 后 CPU 查询（nani 摆放/碰撞过滤）。**

### resources/terrain_stroke.rs（1 文件 / 3 业务函数）

- `resources/terrain_stroke.rs` [3] — **地形笔画：TerrainStroke 点列（Vec<Vec3>，上限 1024）+ 插入/形变末点规则（0.001 单点阈值 + spacing 比较）。terrain 编辑数据单元。**

### resources/texture_presets.rs（1 文件 / 1 业务函数）

- `resources/texture_presets.rs` [1] — **纹理预设绑定类型（TexturePreset 数值槽/TextureFromDisk 名字哈希）。**

### resources/time.rs（1 文件 / 0 业务函数）

- `resources/time.rs` [胶水/内联] — **游戏时间资源门面**——`GameTime` 实测布局：{game_time 累计 Duration（secs@+0x0/

### resources/time_of_day.rs（1 文件 / 5 业务函数）

- `resources/time_of_day.rs` [5] — **昼夜数据资源**——`TodSequence`（entries: Vec<TodSequenceEntry>，0x50B 步长：

### resources/toolbar.rs（1 文件 / 7 业务函数）

- `resources/toolbar.rs` [7] — 工具栏资源：Toolbar {categories: Vec<ToolbarCategoryModes> (cap,ptr,len), current u8@0x18}；ToolbarCategoryModes 0x20B {modes Vec, current u64@0x18}；ModeEntry 0x10B {cat u32, mode u8, pad, arg u64}。new 建 11 类（含 id0 类别；Roofs 为 Decor 0x38..0x3f 区间模式）；contains 的 装饰区间 = 0x38..=0x3f；switch_mode 入参 = (&mut Toolbar, &record 16B)；serde 按类别名 （'Path'…）序列化（input replay 持久化）。

### resources/top（1 文件 / 3 业务函数）

- `resources/mod.rs` [3] — **country-core 顶层资源注册入口**——游戏启动/新会话时把渲染资源、SSBO 库与业务资源插入 Bevy App。

### resources/tree_storage.rs（1 文件 / 7 业务函数）

- `resources/tree_storage.rs` [7] — **树木放置存储资源**——`TreeStorage` 是一个 16 字节条目的哈希表容器（存 `TreeId→Tree`），

### resources/ui_settings.rs（1 文件 / 8 业务函数）

- `resources/ui_settings.rs` [8] — UI 设置：UiSettings 7 字段 {icon_size_mult f32@0, custom_cursor_size f32@4, toolbar_alignment u8@8, auto_hide_toolbar u8@9, input_lag_reduction u8@10, game_logo u8@11, custom_cursor u8@12}；ToolbarAlignment = Bottom/Left/Right/Top（串表 0x142acec66）；默认 {1.0, 24.0, 0x101, 0}（auto_hide=true）。pretty serde_json 持久化。

### resources/walls（25 文件 / 298 业务函数）

- `resources/walls/decorator_storage.rs` [60] — 装饰存储：按目的地分桶（+0x0 墙 HashMap<WallId,Decorators 0x90B>、+0x90 地形、+0x120 楼梯、+0x18/+0x48 其余、+0x78 为 0x18B{DecoratorId,ptr} 查找索引、+0x1b0 可写标志）；支持快照(take_snapshot，16B XOR 掩码)、修改追踪、墙合并/分裂时装饰迁移（d2s 同步）。
- `resources/walls/decorator.rs` [56] — 装饰附着/信息类型族：DecoratorInfo 72B 记录、DecoratorId(u32)/DecoratorIdGen、DecoratorDst 目的地联合（tag@+0x22）、DecoratorExtraInfo 附加信息联合、DecoratorType 判别族（0x38..0x3f 位运算谓词）、DecoratorRank、DecoratorRotationXZ、TransientDecoratorIdGen、装饰操作事件族。
- `resources/walls/inner_walls.rs` [26] — 内部墙集合资源：HashMap<WallId, WallState 0x70>（条目步长 0x78）+ +0xc0 脏代计数器 + +0x30 墙工厂子结构；new_wall/new_wall_with_id/get/get_unwrapped/remove/max_id/reset_max_id 与 _try_update_state 闭包族（6+3 实例，写偏移即 WallState 布局证据）。
- `resources/walls/decorator_archivist.rs` [22] — 装饰归档/剪枝资源：墙合并/分裂/擦除前对墙属装饰做快照归档，提供 cull 放行策略（任意/地形/楼梯）与 before/after 差异提取；interactions 集合在 +0x90。
- `resources/walls/decorator_subtype.rs` [20] — 装饰子类型网格库：DecoratorSubtype(u8, 空=0x1a)、DecoratorLibKey{subtype,variant} 复合键、DecoratorMeshLibrary 4 表（collision/interaction/outline/collision_outline）、DerivedDecoratorSubtypes(HashMap<u32,Arc<DerivedDecoratorInfo>>)。
- `resources/walls/material_stats.rs` [18] — 墙材质统计：7 特征长度 prev@+0x28..+0x40/cur@+0x90..+0xa8（plain/lintels/arches/栅栏/gates/stairs/sewers）、8 段/柱计数 u32@+0xac..+0xc8、seed u64@+0x88、pillar placements Vec@+0x70{cap,ptr,len}（自旋锁@+0x68 保护）；add_*_length 为 CAS 原子累加（加本特征扣 plain 预算）；which_material_changed_the_most = 7 对 |cur-prev| argmax（plain 带 -0.01 bias）。
- `resources/walls/wall_holes.rs` [17] — 墙门窗洞存储：WallHoleStorage 0x68B 两阶段（raw Vec 20B {center,dims,kind} 到 finalized 20B {Aabb2,kind,origin}）+ sum/xor 指纹 + wall_length/closed；finalize 按 maxX 排序、闭合时跨 seam 平移副本（origin 0/1）；clear_for_next_frame 迁入 PrevWallHoles。
- `resources/walls/public_wall_state.rs` [12] — 公共墙状态 PublicWallState 0x2a0B：raw Curve2{ptr@+8,len@+0x10,长@+0x30}、3D 曲线@+0x88、高度数组{ptr@+0xe0,len@+0xe8}、max_y/gable 缓存{+0x108/+0x10c, +0x110/+0x114}、形状记录 28B@+0x258（0=Circle/1=Rectangle/2=Freehand）、发布长度@+0x28c、style@+0x299；方法面含 wall_space/u_space（Option<f32>，u_space=1.0f 常量）与锚高投影。
- `resources/walls/inner_wall_state.rs` [11] — 内部墙状态权威：WallState 0x70B（enclosure 记录@+0x38、height f32@+0x54、terrain floor@+0x58、freehand_ty u8@+0x60、arch_type u8@+0x64、removed_supports u8@+0x67、双 Vec 曲线）+ WallStyle/StructureType/ArchType/FreehandHeightType/PlasterPeelAmount 枚举族与兼容 serde（13 字段名表）。
- `resources/walls/mod.rs` [10] — 墙资源族根模块：全局墙 ID（WallId u64 newtype）、Floor 楼板组件、WallHighlight 高亮资源、HighlightColor 高亮色枚举，以及模块级 split_height_into_skirts 墙裙边高度带计算（height>=5.9 起 4.5 每 +3.5）。
- `resources/walls/wall_trigger_volume.rs` [9] — 墙触发体积：quad 数组 {ptr@+8,len@+0x10}（元素 0x60B）到 get_tris 输出 0x48B 三角形；recompute_quads 重算网格（panic !triangles/!mesh_uvs/!normals.is_empty）；形状编辑拖拽触发体。
- `resources/walls/public_walls.rs` [8] — 公共墙集合：HashMap<WallId, PublicWallState>（0x2a8 步长）+ wall_entities_map@+0x30 + roofs map + simplified_curve_2d 简化 2D 曲线（publish/spatial_hash/clamp 三处断言 is_valid）；WallEntities/WallPlasters 资源。
- `resources/walls/cached_decorator_transforms.rs` [7] — 装饰物缓存变换资源：墙高/几何变化后装饰重定位的缓存；transform 入口对缓存 3x3 矩阵做有限性（NaN/Inf）校验并 panic（0x140a1ada0）。
- `resources/walls/wall_attachment.rs` [7] — 墙锚类型：WallAttachmentAnchor（platform WallId/wall_height_relative_y/anchor_ws[3]/corner/is_bottom_door/side）、WallAttachmentSide 0/1、WallCornerAttachment（0=None, 1..=4=矩形角 id+1）；serde 兼容层。
- `resources/walls/dormers.rs` [3] — 老虎窗洞 SSBO 记录：DormerHoleSsbo 48B（gothic 4×f32 尾区 / cottage 2×f32+f64 尾区）与 DormerHoleCountPerCellSsbo 计数。
- `resources/walls/plaster.rs` [3] — 石膏墙组件（Plaster）与剥离动画资源（AnimatedPlasterPeel）+ Recalculate* 命令族；渲染域消费（WP6），无独立业务函数。
- `resources/walls/wall_path_segment.rs` [3] — 墙路径段（WallPathSegment）：墙切分/轮廓预备中间态，含预备点数组与 2D/3D 长度（pt_count/length_ws/length_ws_2d）。
- `resources/walls/curved_halftimber.rs` [2] — 弯曲半木墙组件（CurvedHalfTimber，含 16 段端帽掩码）与 drawable 集合资源；渲染域消费，无独立业务函数。
- `resources/walls/decorator_outline.rs` [2] — 装饰描边数据（DecoratorOutline{尺寸+0x20..,位置+0x30..,判别}/DecoratorCollisionOutline{Lines(Vec<Vec3>)/Sphere}）与调试显示（Mat3+平移逐顶点变换）。
- `resources/walls/wall_stone_columns.rs` [1] — 石柱碰撞存储（StoneColumnCollisionStorage）：墙体石柱的碰撞数据存储（v2 新反编译 0x140a78880）。
- `resources/walls/wooden_gates.rs` [1] — 木门资源：木门/栅栏门相关状态（无独立业务函数，渲染/构造域消费）。
- `resources/walls/arches.rs` [胶水/内联] — 拱段/过梁段的跨帧状态容器（ArchSegment/ArchSegments/LintelSegment/LintelSegments/BuildingHadArchesLastFrame）：由 system_wall_constructor::construct_arches 写回，供跨帧差分，无独立业务函数。
- `resources/walls/decorator_collision.rs` [胶水/内联] — 装饰物理碰撞体集合（DecoratorPhysicsColliders）：parry3d 0.17.0 复用边界，无独立业务函数。
- `resources/walls/elevation_support_stats.rs` [胶水/内联] — 升降支撑统计（ElevationSupportStats/ElevationSupports）：墙高调整的支撑统计，供 audio elevation_supports_audio 消费；无独立业务函数。
- `resources/walls/in_progress_wall.rs` [胶水/内联] — 编辑中墙：InProgressWall 组件与 InProgressWallResource（Option 语义）；徒手墙绘制输入到预备轮廓到提交的前置状态（finish_wall 取 .wall.take()）。

### resources/weather.rs（1 文件 / 0 业务函数）

- `resources/weather.rs` [胶水/内联] — **天气资源占位**——`Weather` 空资源（wp21 misc-resources：O 名、`impl$0::default`

### resources/window.rs（1 文件 / 6 业务函数）

- `resources/window.rs` [6] — **窗口尺寸/光标资源（WindowSize 头 8B=Vec2 f32×2）。**

### resources/window_lights.rs（1 文件 / 2 业务函数）

- `resources/window_lights.rs` [2] — **窗光/灯资源族（WindowLight 组件/LightInfo/Global*Control）。**

### resources/world_raster.rs（1 文件 / 22 业务函数）

- `resources/world_raster.rs` [22] — **世界栅格核心容器 WorldRaster<512>（composite/scratch 双缓冲 u8 栅格 + tabloid DirtyTiles 脏瓦片协议 + 订阅发布）与 world_raster_defs 子模块（命名栅格资源 + AllWorldRasterRead/Write 全栅格 SystemParam）。水/路/鹅卵石/花园六种栅格的公共基础设施；栅格化走 zeno 依赖。**

## startup（35 文件 / 60 业务函数）

### startup/top（35 文件 / 60 业务函数）

- `startup/load_prefabs.rs` [7] — **预制体（prefab）入库：网格资产生产者。读 `assets/prefabs` 目录，经 `load_json_as_mesh` 处理顶点属性（appear_pos/prim_center/prim_normals/Vertex_Normal/Vertex_Position）注册进 mesh 库；含 PrefabWatcher（热重载）与插件 build。**
- `startup/mod.rs` [6] — **startup 模块总入口：`add_game_startup_systems`（GameStartup 插件，注册全部启动系统）+ shader/mesh 入库 loader（load_shader_into_library、load_mesh_into_library）。**
- `startup/load_mesh_atlases.rs` [5] — **网格图集（mesh atlas）生产者：生成程序化图集 mesh parts（stone_floor/plaster/stairs_floor/smoke）+ 载入 nani 网格（assets/nani_meshes.ron）+ 地形网格。是渲染 pass 的网格资产输入。**
- `startup/clearing.rs` [3] — **clearing（林间空地）初始化/反初始化：载入 clearing 网格与 shader、反初始化时恢复音频 snapshot。**
- `startup/load_textures.rs` [3] — **纹理载入：过滤（should_load）并载入启动纹理（logo/brush/loading/menu 相关），构建 GladeTextureMappings（textures.json 语义键→纹理文件映射）。**
- `startup/startup_terrain.rs` [3] — **地形启动：载入地形/岩石/楼梯/鹅卵石/植被网格与 shader，生成地形网格（cw/ccw），spawn 地形实体。**
- `startup/load_glade_audio.rs` [2] — **glade 音频载入：按 glade theme 载入对应 FMOD 音乐事件与音效。**
- `startup/load_glade_tod_and_weather.rs` [2] — **glade ToD/天气 与雪粒子载入（按 theme）。**
- `startup/load_shaders.rs` [2] — **shader 载入：UI shader 与主渲染 shader 入库（经 startup/mod.rs 的 load_shader_into_library）。**
- `startup/startup_nani.rs` [2] — **nani（GPU 实例渲染后端）启动与热重载：创建世界子集、检查运行时限制。**
- `startup/startup_session.rs` [2] — **游戏会话启动：起始 build 选择（full/demo/intro）、会话目录创建、meta.json 读写、历史/快照载入。**
- `startup/debug.rs` [1] — **debug 启动系统：载入调试可视化/测试网格（debug 球体、decorator 测试网格、stair_step、particle_plane）。**
- `startup/init_color_library.rs` [1] — **颜色库初始化：从 assets/colors.json 载入默认调色板。**
- `startup/init_photomode_state.rs` [1] — **照片模式状态初始化与启用。**
- `startup/load_audio.rs` [1] — **音频系统启动：注册 audio 源/资源容器。**
- `startup/load_glade_grading.rs` [1] — **glade 分级 LUT 载入（按 theme）。**
- `startup/spawn_brush_previews.rs` [1] — **画笔预览实体生成：载入 ~13 个 brush 预览网格并 spawn 预览实体（鼠标光标笔刷）。**
- `startup/spawn_human_ref.rs` [1] — **人类尺寸参考实体：spawn 一个参考立方体（human_ref_cube）。**
- `startup/spawn_preplaced_trees.rs` [1] — **预置树生成：deserialize trees.json，spawn clearing 外围预置树。**
- `startup/startup_blit.rs` [1] — **blit 阶段 shader 载入（交换链 resolve/srgb decode）。**
- `startup/startup_critters.rs` [1] — **小动物（critters）资源载入：butterfly/duck/splash 网格 + shader + 音效。**
- `startup/startup_deferred_raster.rs` [1] — **deferred raster 阶段 shader 载入。**
- `startup/startup_garden_brush.rs` [1] — **花园笔刷资源载入：garden_stone + 低模花。**
- `startup/startup_grass.rs` [1] — **草地资源载入：grass blade 网格 + grass shader。**
- `startup/startup_ivy.rs` [1] — **常春藤资源载入：ivy 分支/叶/花网格 + instanced shader。**
- `startup/startup_meadow.rs` [1] — **草地/meadow 植物载入：plants.json + 低模花 + meadow_plant_shader。**
- `startup/startup_roof.rs` [1] — **屋顶资源载入：roof_spire/support_beam 网格 + shader + 删除笔刷。**
- `startup/startup_sheep.rs` [1] — **羊资源载入：羊动画网格 + 附属物（heart/umbrella/floatie）+ shader + 羊叫音效。**
- `startup/startup_sky_and_sun.rs` [1] — **天空/太阳资源载入：skydome mesh + shader。**
- `startup/startup_time_of_day.rs` [1] — **ToD 启动：载入默认时刻。**
- `startup/startup_walls.rs` [1] — **墙启动：载入 wall brick LOD shader，生成 half_cube 网格。**
- `startup/startup_water.rs` [1] — **水面资源载入：water_plane 网格 + 水 shader 族 + 睡莲/芦苇/水石网格。**
- `startup/glade_themes/flowery.rs` [1] — **Flowery 主题资源载入：主题植物（lavender/低模花）+ 树配置。**
- `startup/glade_themes/olden.rs` [1] — **Olden 主题资源载入：主题树配置 + clover 花。**
- `startup/panic_reporter_startup.rs` [胶水/内联] — **panic 报告器启动：设置 Rust panic hook（上报到反馈系统）。**

## systems（298 文件 / 862 业务函数）

### systems/arrow_gizmo_animation.rs（1 文件 / 1 业务函数）

- `systems/arrow_gizmo_animation.rs` [1] — **箭头 gizmo 拉伸动画**——`EnableArrowGizmoStretch` 组件实体的双 f32 拉伸值：

### systems/audio（31 文件 / 57 业务函数）

- `systems/audio/path_audio.rs` [4] — **小路/卵石绘制音效（CobblestoneDraw 循环 + 4 个 snappy 循环）。**
- `systems/audio/roof_audio.rs` [4] — **屋顶编辑音效：3 循环 + 位置/音量即时实例辅助。**
- `systems/audio/building_audio.rs` [3] — **建筑编辑音效：PlayBuildingAudio 事件（瞬时 detach/attach/peek）与每帧 7 个循环参数的 FMOD 驱动。**
- `systems/audio/mod.rs` [3] — **音频模块注册：16 个事件类型 + 运行期/启动期系统。**
- `systems/audio/water_ambience_audio.rs` [3] — **水面环境音：按水网格逐格装配的 Amb 循环播放器。**
- `systems/audio/audio_update.rs` [2] — **每帧/启动期音频更新：把 AudioSettings 音量推给 5 个 FMOD VCA，并把相机（或默认）位姿作为 3D 听者推给 Studio。**
- `systems/audio/decorator_audio.rs` [2] — **装饰物音效：16 个循环 loop（窗/灯/烟囱/旗帜）+ 按距离衰减的瞬时播放。**
- `systems/audio/forest_audio.rs` [2] — **森林生长音效（1 个循环）。**
- `systems/audio/game_loading_audio.rs` [2] — **加载界面音频：snapshot:/Loading 快照 + event:/UI/Loading。**
- `systems/audio/garden_audio.rs` [2] — **花园绘制/擦除音效（4 循环 + update_draw_audio 模板）。**
- `systems/audio/menu_audio.rs` [2] — **菜单 UI 音效：PauseMenu 快照 + 13 个 UI 事件。**
- `systems/audio/playermade_stairs_audio.rs` [2] — **玩家自制楼梯音效（6 循环 + kind≤18 分派）。**
- `systems/audio/replay_audio.rs` [2] — **撤销/重做/拒绝输入音效。**
- `systems/audio/signifier_audio.rs` [2] — **UI 符号流音效（1 循环 + 4 瞬发 + 21 参数更新）。**
- `systems/audio/terrain_audio.rs` [2] — **地形编辑音效（4 循环 + 编辑状态机）。**
- `systems/audio/timberframe_audio.rs` [2] — **木骨架音效（PlayTimberframeSound）。**
- `systems/audio/tree_audio.rs` [2] — **树木音效（4 循环 + 最近树距离排序）。**
- `systems/audio/trigger_audio.rs` [2] — **音频触发器：相机穿过树木触发盒的响度（4 实例 + audibility 平滑）。**
- `systems/audio/wall_audio_fmod.rs` [2] — **墙体音频主系统：10 循环 + kind 0-23 分派 + 帧尾 start/stop。**
- `systems/audio/water_audio.rs` [2] — **水体绘制/擦除/结冰音效（4 循环 + mode）。**
- `systems/audio/clutter_audio.rs` [1] — **杂物摆放音效：PlayClutterSoundCmd kind(0..0x26=38)→event:/SFX/Cosmetics/* 瞬时播放。**
- `systems/audio/elevation_supports_audio.rs` [1] — **抬升支撑（木架/柱）出现音效。**
- `systems/audio/firstperson_steps.rs` [1] — **第一人称脚步声：GroundType→FMOD 参数 + FirstPersonOff 全局参数。**
- `systems/audio/ivy_audio.rs` [1] — **常春藤生长音效（单瞬发）。**
- `systems/audio/sheep_audio.rs` [1] — **绵羊反应音效（kind 0-7 + 参数 FirstPersonOff）。**
- `systems/audio/terrain_stairs_audio.rs` [1] — **地形楼梯音效（SuppressPathSound + StairsCounter 状态 + StairDraw_OneShot 瞬发）。**
- `systems/audio/wall_audio_orchestrator.rs` [1] — **墙体材质音频协调器（RequestWallMaterialAudio）。**
- `systems/audio/wall_audio_path.rs` [1] — **墙体路径绘制音效。**
- `systems/audio/wall_audio_ui.rs` [1] — **墙体 UI 音效（PlayWallUiSound kind 分派）。**
- `systems/audio/wall_audio_utils.rs` [1] — **墙体循环播放器（AudioLoopPlayer，10 循环的公共实现）。**
- `systems/audio/weather_audio.rs` [胶水/内联] — **天气/季节全局音频参数（Weather→Studio 全局参数）。**

### systems/audio/music（3 文件 / 11 业务函数）

- `systems/audio/music/music_player.rs` [8] — **音乐播放器：曲目池 + 4 状态机（含节拍同步）+ 快照触发。**
- `systems/audio/music/music_system.rs` [2] — **音乐系统：richness/NewGladeTime 全局参数 + 播放器驱动。**
- `systems/audio/music/music_debug_gui.rs` [1] — **音乐调试 egui 面板（开发用）。**

### systems/backend（12 文件 / 15 业务函数）

- `systems/backend/flush_screen_log.rs` [3] — **屏幕日志队列。全局静态 `Mutex<Vec<(String, LogType)>>`（0x14362bfe8 起），`log_error_to_screen`/`log_warning_to_screen` 把消息压入队列，`flush_error_log_to_screen` 系统排空队列并按类型着色写入 `ScreenLog` 资源（最近 5 条环）上屏。**
- `systems/backend/mod.rs` [3] — **后端系统族插件注册。`BackendPlugin` 三入口：`add_backend_systems`（App::add_systems 注册 ~7 个后端系统并配置 GameSystemSet）、`insert_backend_resources`（3 分支状态机插入后端资源/按路径装载）、`add_backend_events`（注册 6 个事件/资源类型）。含 `TimedFrameEvent`。**
- `systems/backend/icc.rs` [2] — **窗口 ICC 色彩配置。`IccProfilePlugin::build` 注册调度（add_systems 路径）；`update_window_icc_profile` 系统监听显示器 ICC 变更 → lcms2-5.6.0 `cmsOpenProfileFromMemTHR` 读配置文件字节 → `generate_icc_lut`（内联）对具名颜色（terrain_1/terrain_2/wall_moss/wall_bricks/terrain_highlight/night_tint）生成 LUT 写入 ColorLibrarySSBO。**
- `systems/backend/vao_update.rs` [2] — **VAO 构建/重建系统。`build_missing_vaos` 遍历无 VAO 的网格建立 VAO（空网格跳过并日志）；`rebuild_vaos` 遍历全部网格按绑定变化重建 VAO（无 VAO 网格警告）。**
- `systems/backend/configwatch.rs` [1] — **配置热载系统。每帧检查 `ConfigWatch`（互斥队列）是否有待处理的配置变更；有则日志 `'Configwatch: detected changes: N'` 并调用配置重载助手 `FUN_14093b530`（读 `assets/gladestate`+`colors.json` 路径 → 重载 ColorLibrary / SSBO 库），随后重载生效。**
- `systems/backend/grow_grass.rs` [1] — **草地生长"仪式"系统。启动后计时 300 秒（`_UNK_142aa5140`=300.0f32），到时校验一组 15B 数据（XOR 解码静态）的 64B 哈希一致后通过 EventWriter 发送 8B 事件并置 grown 标志；涉及等待后台线程句柄（CloseHandle）。属一次性视觉过渡，非植被模拟。**
- `systems/backend/meshwatch.rs` [1] — **网格热载系统。每帧锁 `MeshWatch` 互斥队列，日志 `'Meshwatch: detected changes: N'`，按位图索引遍历变更网格并调 `AssetMeshLibrary::reload_mesh`（`FUN_14092d8d0`，JSON 网格重载）；panic `Result::unwrap()`（meshwatch.rs:43）。**
- `systems/backend/prepare_frame_constants.rs` [1] — **帧常量 CPU 组装。每帧把帧序号写 `frame_constants+0x108 = idx & 3`，按 `USE_TAA_JITTER` 全局（cRam000000014362c0a0）用帧号采样 jitter 表（0x142ae4fc0 = {0,3,1,2,4,...}，`%table_len`），并组装 4 份 `ViewConstants`（`FUN_140b18f60` = ViewportConstantBuilder::build）与 800B 帧常量块。**
- `systems/backend/texture_watch.rs` [1] — **纹理热载系统。每帧锁 `TextureWatch` 互斥队列，日志 `'Texture watch: detected changes: N'`，按位图索引遍历变更纹理并调 `FUN_140b73a50`（递归位图树遍历重载助手）逐项重载；错误日志 `'Error processing textures: '`；panic `Result::unwrap()`（texture_watch.rs:43）。**
- `systems/backend/game_frame_index.rs` [胶水/内联] — **帧序号资源。`game_frame_index` 为全内联函数（dependencies.json inline_instances [560, 40168]/[1611, 2310144]），无独立物理符号；其资源 `GameFrameIndex` 由 `prepare_frame_constants` 消费（写 `frame_constants+0x108 = idx & 3`）。**
- `systems/backend/panic_reporter_fetch_active_session.rs` [胶水/内联] — **崩溃报告取活动会话。`panic_reporter_fetch_active_session` 全内联（PDB inline 记录），向崩溃报告器提供当前活动 session 标识（`ActiveSession` 资源）。**
- `systems/backend/transient_mesh.rs` [胶水/内联] — **瞬时网格后端系统。`delete_dropped_transient_meshes` 全内联（无独立物理符号），为 `TransientMeshRegistry::drain_meshes_to_delete`（0x1412e9e70）的消费闭包，排空待删网格句柄并释放 GPU 网格；组件在 backend 每帧运行。**

### systems/backend/input（7 文件 / 9 业务函数）

- `systems/backend/input/input_record_replay.rs` [3] — 输入录制/回放状态机：mode@+0x40（^0x8000000000000000 后 case 0/2/3/4 = Idle/Replaying/Recording/Finished）。Idle 满足条件即 store camera + 克隆 ToolbarCategoryModes + 重建流；Replaying 按 0xd0B InputFrame 写回状态（@+0x30 u64/@+0x38 u32/@+0x3c u8/@+0x40 双 f32/@+0x44 除数/@+0x58..0xc0 十 qword）；越界 panic 'Out of bounds access'。
- `systems/backend/input/input_replay_for_trailers.rs` [2] — trailer 用输入录制/回放：配置字节@+0xfc==1 且命中按键表时在 Idle/Recording/Replaying 间切换（'Start recording'/'Finished recording'/'Replay...' 日志），do_input_replay_ui 提供 egui 面板（'Enable input recording'、'`O` - start/stop recording'、tmp/input_replay.json 保存）。
- `systems/backend/input/calculate_input_delta.rs` [1] — 输入增量计算：对每个 analog action 按活动/死区状态（kind 0/1/2 = released/moving/deadzone）累加 cursor delta（死区 0.1/0.05），输出 {tag,value} 对 + 10 qword 原始镜像（wp20 合同复核）。
- `systems/backend/input/reset_input_state.rs` [1] — 输入状态复位：清零并重排 InputTranslationState（11 qword 置换）、按最后 sync point 修剪双 VecDeque、ButtonInput::reset_all<KeyCode>()（wp20 合同复核）。
- `systems/backend/input/translate_input.rs` [1] — 输入映射主系统：Analog→ActionSetValues、Digital→ActionSetStates、MouseMoved 事件消费（wp20 合同复核，34KB 反编译已落盘）。
- `systems/backend/input/translate_input_to_cursor_moved.rs` [1] — 录制事件双队列（stride 0x18，tag@8：0=mouse_moved/1=moved/2=跳过）→ 累加游标 delta，NaN 安全clamp，推 InputCursorMoved（0x10B {id, Vec2}，bevy 0.16 EventInstance {id@0, event@8} 布局）并累加进输出状态 @+0x58/+0x60。
- `systems/backend/input/update_cursor_position.rs` [胶水/内联] — 游标位置合并（全内联无独立符号）：把 InputCursorMoved 合并进游标位置资源。

### systems/brush_preview.rs（1 文件 / 3 业务函数）

- `systems/brush_preview.rs` [3] — **笔刷预览：BrushPreview 组件（克隆行为=判别 2 + 克隆函数指针 0x1412eff70）+ brush_preview 系统（预览网格跟随光标）。**

### systems/camera_rig（5 文件 / 26 业务函数）

- `systems/camera_rig/free.rs` [9] — **FreeCamera**——dolly rig（Position+YawPitch+Smooth(1,1)）封装：driver 按 typeid
- `systems/camera_rig/walk.rs` [9] — **WalkCamera（行走相机）**——dolly rig（Position+YawPitch，无 Smooth）+ 碰撞滑行：
- `systems/camera_rig/butterfly.rs` [4] — **ButterflyCamera（蝴蝶追踪相机）**——dolly rig（Position+Rotation+Smooth(2,predictive)+
- `systems/camera_rig/normal.rs` [4] — **NormalCamera（轨道相机）**——Normal/TiltShift/TopDown 三种模式共用的轨道 body：
- `systems/camera_rig/camera_body.rs` [胶水/内联] — **相机 body 共享类型与 trait**——`CameraBody` 接口（rotate_yaw_pitch/update）、

### systems/camera_rig.rs（1 文件 / 21 业务函数）

- `systems/camera_rig.rs` [21] — **相机装配中心**——`MainCamera`（0x4e0B：@+0x0 矩阵块 0x100B、@+0x100/1b8/270/2f0/390/448

### systems/clear_canvas.rs（1 文件 / 1 业务函数）

- `systems/clear_canvas.rs` [1] — **清空画布系统**——事件驱动（EventReader 修剪流）：日志 "Clearing the canvas." →

### systems/collision（3 文件 / 81 业务函数）

- `systems/collision/world.rs` [75] — **rapier3d 碰撞世界胶水：17 子世界槽、ray/sphere/shape 查询、dirty 瓦片生命周期（flush/publish/QueryPipeline::update）、collider upsert/remove 泛型族、parry 网格数据构造**
- `systems/collision/intersect_meshes.rs` [4] — **网格-网格相交检测：parry QBVH 两两三角相交→交点段合并为折线→伪影图+tabloid 发布**
- `systems/collision/visualize_colliders.rs` [2] — **debug 碰撞体线框可视化**

### systems/collision.rs（1 文件 / 1 业务函数）

- `systems/collision.rs` [1] — **碰撞插件（systems/collision 模块入口）：impl$0::build 注册碰撞资源/系统（round3 O）**

### systems/debug（3 文件 / 8 业务函数）

- `systems/debug/sheep_controls.rs` [5] — **绵羊调试控制（拾取/快照）。**
- `systems/debug/human_scale_ref.rs` [3] — **人体比例参考调试可视化（多模式跳表）。**
- `systems/debug/lazy_grid_debug.rs` [胶水/内联] — **惰性网格调试开关（全内联）。**

### systems/despawn.rs（1 文件 / 1 业务函数）

- `systems/despawn.rs` [1] — **关卡拆除实体清理**——Query 迭代，world 字节 @+0x108 二分：A 支（stride 0xa0、

### systems/garden（2 文件 / 2 业务函数）

- `systems/garden/draw_garden.rs` [1] — **花园画笔系统：工具 id 分发（9=橡皮/17=花丛），半径=笔刷+1.5，界内校验后走 draw_raster<GardenRasterStagingSsbo> 绘制花园掩码。**
- `systems/garden/record_garden_history.rs` [1] — **花园编辑历史记录：把本帧 12B 编辑单元封装 Arc 进 History::add。**

### systems/garden.rs（1 文件 / 1 业务函数）

- `systems/garden.rs` [1] — **花园系统插件：build 注册 draw_garden/record_garden_history/replay 等系统与 GardenRaster 资源。**

### systems/history（15 文件 / 32 业务函数）

- `systems/history/replay_raster.rs` [9] — 光栅回放泛型系统 replay_raster<T,R,F>（Path/C1/C2/C3/Garden/Water 六实例）：过滤前桶命令，undo 清缓冲+重绘存储 stroke，redo 画 stroke 并发绘制事件；panic “!cmd.edit.stroke.points.is_empty()”。附 RasterReplayCmd（forward/backward、stroke 切段 get_stroke_from_0_to_t、时长 duration_based_on_stroke_length）与 RasterReplayResources SystemParam。
- `systems/history/history_process.rs` [5] — 历史命令中枢系统：消费 HistoryCmd 事件流（undo/redo/interrupt），移动 History 游标后对目标 MultiEdit 调用 replay_edit 展开 ReplayCmd 桶（undo 时反转），并把桶 push 进 ReplayBuffer；同时消费第二事件流（16B 记录，byte[+9] 跳表 0x142ae5420）做命令校验/动画即时化/移除，录制中仅记日志。generate_replay_cmds_from_edit 为按 HistoryEdit.tag 分派（跳表 0x142c5f248）的共享核心。
- `systems/history/mod.rs` [3] — history 系统族注册面：add_history_systems / insert_history_resources / add_history_events 三个 Bevy App 扩展胶水（注册 ~10 个回放/录制系统、插入 History/ReplayBuffer 等资源、添加 HistoryCmd 等事件）。AppMode 回放判别 0xd 的边界定义也在此。
- `systems/history/replay_terrain.rs` [3] — 地形回放系统：前桶判别 3 命令；TerrainReplayCmd::forward/backward 构造 duration=clamp(Σ点距*0.01,0.4,3.0)（无点 0.3，stroke_length 有限断言）；undo(+0x1c)==0→reapply，!=0→revert；动画 ease_out。
- `systems/history/replay_wall.rs` [3] — 墙回放系统：前桶判别 4 命令，每 WallReplayEditCmd（0x18B）经 process（0x140abfb30，自递归，8 参）分派应用（instant/动画两路径）；done 标志 @+0x15；全部完成时经 niche vtable 0x142ab4910 收集已改墙。附 _deprecated_morph_curve（已弃用形态曲线应用）。
- `systems/history/signifier_replay_active.rs` [2] — 回放激活指示器系统：ReplayModeElapsedTime（enum2$ 判别）活跃时累加，对每个带 SignifierReplayIndex 组件的实体把 Transform 朝光标目标做弹性抖动（ease_out_elastic_jiggle=0.5*exp2(-10t)*sin(7.853982t)）；register_required_components 注册必需组件。
- `systems/history/finish_replay.rs` [1] — 回放自然完成/非活跃回放的收尾系统：检测 ReplayBuffer 与待处理命令双空后把 RestoreAppTo 记录的 AppMode 写回、解锁游戏指挥官（MetaCommander 哨兵弹栈）、恢复工具栏模式，并向绵羊系统发 CmdUnfreezeSheep。日志“History replay has naturally finished…”/“History replay was not active…”。
- `systems/history/history_recording.rs` [1] — 输入录制启停系统族：history_start_recording / history_finish_recording 驱动 History::start_input_recording / finish_input_recording（0x1409dd790 为 finish，多事件游标 + AppMode 检查）。
- `systems/history/replay_auto_clutter.rs` [1] — 自动杂物回放系统：消费前桶判别 9 命令，undo==0 插墙集（HashMap<WallId,()>），!=0 从 DeletedAutoClutter erase (DecoratorId,DecoratorId)；instant/动画节奏。
- `systems/history/replay_mode.rs` [1] — 回放节奏系统：检测 AppMode 进出 0xd(Replay)（log started/finished）、RestoreAppTo 记录回放前模式、长按快进（HoldToFastReplay）时发 redo HistoryCmd、驱动 ReplayModeElapsedTime 计时。
- `systems/history/replay_path.rs` [1] — 路径/鹅卵石光栅回放：replay_raster 的 4 个领域实例（DirtPath/Cobblestone1-3）+ generate_path_replay_cmds（0x1416286d0，niche 判别 0..3 过滤后尾调共享核心 0x1414433xx）。
- `systems/history/replay_pop_bucket.rs` [1] — 空桶弹出系统：前桶 cmds 空时 ReplayBuffer::pop_front，应用桶的装饰 diff（restore_decorators，FUN_14155ba30）与楼梯 diff（restore_backup/apply_diff 链 1408ff210→140900ba0→14157cec0→140b50cc0→140b50d60→1408fefe0），Arc drop 收尾并 flush colliders/archivist。
- `systems/history/replay_stairs.rs` [1] — 楼梯回放系统：前桶判别 10 命令，undo==0 插边集（StairsRemovedSupports），!=0 从边集 remove_entry；查附挂装饰（DecoratorStorage::lookup_decorator_info + CachedDecoratorTransforms）并发 PlayStairsAudioCmd。
- `systems/history/replay_garden.rs` [胶水/内联] — 花园光栅回放：replay_raster<GardenRasterStagingSsbo, WorldRaster<512>, GardenFilter> 的薄包装 + 命令生成闭包（generate_replay_cmds_from_edit 宿主 0x14143d2c0）。
- `systems/history/replay_water.rs` [胶水/内联] — 水面光栅回放：replay_raster<WaterRasterStagingSsbo, WorldRaster<512>, WaterFilter> 薄包装 + 命令生成闭包（宿主 0x141444c70）。

### systems/ivy（9 文件 / 17 业务函数）

- `systems/ivy/ivy_grower.rs` [4] — **常春藤生长：10 相状态机（grower 7.9KB 单函数）沿墙推进折线**
- `systems/ivy/ivy_pruner.rs` [4] — **常春藤修剪：墙变更后点有效性校验+按位置移除**
- `systems/ivy/mod.rs` [3] — **ivy 插件（资源/系统/事件注册）**
- `systems/ivy/ivy_branch_thickness_animation.rs` [1] — **分支粗细动画**
- `systems/ivy/ivy_leaf_animation.rs` [1] — **叶动画**
- `systems/ivy/ivy_leaf_spawner.rs` [1] — **叶生成：消费段完成消息沿末段放叶**
- `systems/ivy/ivy_send_to_gpu_leaf.rs` [1] — **叶 GPU 上传（SSBO）**
- `systems/ivy/ivy_send_to_gpu_segments.rs` [1] — **段 GPU 上传（SSBO）**
- `systems/ivy/ivy_spawner.rs` [1] — **常春藤生成：随机根点/光标旁根点**

### systems/look_at_camera.rs（1 文件 / 3 业务函数）

- `systems/look_at_camera.rs` [3] — **LookAtCamera 组件驱动**——每帧把带 `LookAtCamera` 组件的实体的 Transform

### systems/main_camera.rs（1 文件 / 8 业务函数）

- `systems/main_camera.rs` [8] — **主相机系统插件**——`MainCamera`（0x4e0B 资源）的每帧驱动：快照按模式分发

### systems/mesh_follow_terrain.rs（1 文件 / 1 业务函数）

- `systems/mesh_follow_terrain.rs` [1] — **网格贴地系统：Query<(&MeshFollowTerrain, &mut Transform, &Handle<Mesh>)> + 地形资源；取 mesh 'Vertex_Position' 属性计算位置 → 采样地形高度 → Transform.y 更新；句柄无效 panic 'MeshFollowTerrain got an invalid mesh handle'。**

### systems/mod_key.rs（1 文件 / 3 业务函数）

- `systems/mod_key.rs` [3] — Modifiers 修饰键状态与吸附禁用开关：update_mod_key_state 系统读 ButtonInput<KeyCode>，按 7 组键跳表计算 ctrl/shift 组合 + 工具栏 Decor 模式区间（0x38..0x3f）判定，写 Modifiers{u32@0, byte@1=disable_snapping}（含 f32 阈值/按键沿触发）。

### systems/mode_manager.rs（1 文件 / 21 业务函数）

- `systems/mode_manager.rs` [21] — AppMode 模式状态机中枢：哈希 esc_menu_canvas 阻塞早退、排空 RequestAppModeChange 双缓冲队列、check_if_can_switch 校验/延后（pending=0xd Replay）、按模式 id 跳表分发到工具栏类别闭包 env_3..14、send_mode_just_changed 事件（prev/current AppMode 对）。AppMode 判别：2=Timelapse、6..9=Wall 形状、10+5k=Tool(类别 k，视觉序)、0xd=Replay；事件队列 = bevy 0.16 Events 双 VecDeque（EventInstance {id u64,event}，官方 registry 布局）。

### systems/paths（4 文件 / 13 业务函数）

- `systems/paths/draw_path.rs` [9] — **路径绘制：UI 拖拽→DrawState 状态机→WorldRaster<512> 四变体绘制+互擦**
- `systems/paths/path_brush_preview_size.rs` [2] — **路径笔刷预览尺寸（按 UI 模式）**
- `systems/paths/mod.rs` [1] — **paths 插件**
- `systems/paths/record_path_history.rs` [1] — **路径/石板编辑历史记录（4 命令→撤销历史）**

### systems/prefab.rs（1 文件 / 10 业务函数）

- `systems/prefab.rs` [10] — prefab 实例化系统族：spawn（命令→库查找→RT 实例+音频触发）、update（GPU 实例数据刷新）、despawn（内容清理+实体销毁）、hot_reload（watch→加载 JSON→despawn→audio→rt 全链重建）；PrefabPlugin 注册 2 个 Bevy 类型。

### systems/render_loop（4 文件 / 17 业务函数）

- `systems/render_loop/vermeer.rs` [9] — Vermeer 风格化笔触揭示链（会话开始动画）：`VermeerState`（enum2$，判别 +0x48：0 待采集/1 已采集/2 未启用；+8 Option<揭示源纹理>，+0x3e==7=None；+0x54/+0x55 bool）、`activate`/`deactivate`/`into_disable_hold`/`is_active`/`should_freeze_camera` 状态机、`VermeerPlugin::build`、`vermeer_pass`（compute+raster 链）、`vermeer_render_from_target`、`startup_vermeer`。reconstruction `reconstruction/country-core/src/systems/render_loop/vermeer.rs` 已按机器码全量恢复（O）。
- `systems/render_loop/mod.rs` [4] — render_loop 域公共工具：`RenderContext::finish`（渲染上下文终结/资源释放）、`clear_buffer`/`clear_rg_buffer`/`clear_rg_color_image`（向 GPU 命令队列压入 ClearBuffer/ClearImage 命令包）、`rg_buffer_copy_from_slice<u32>`（CPU 切片→GPU 缓冲的 BufferMemcpy 命令）。被 gpu_generate_draw_lists、read_terrain_data 等系统复用。
- `systems/render_loop/lut_gallery.rs` [2] — 摄影模式 LUT 画廊：`LutGalleryPlugin::build` 注册插件，`render_photomode_lut_gallery` 渲染 LUT 缩略图画廊（每项 `lut_gallery_thumbnail` 纹理）。
- `systems/render_loop/menu_background.rs` [2] — 主菜单背景：`MenuBackgroundPlugin::build` 注册插件，`render_menu_bg` 渲染菜单背景（高斯模糊链 `/gauss_blur.cs.hlsl:cs_main`）。

### systems/render_loop/compute_passes（40 文件 / 68 业务函数）

- `systems/render_loop/compute_passes/gpu_generate_draw_lists.rs` [13] — Nani GPU 绘制列表生成（最高价值模块）：`gpu_generate_draw_lists`（W21/W22/W27/W28，CPU 侧编排第 1/2 轮 GPU draw-list 生成），`cull_and_bucket_external`/`create_cull_and_bucket_pass`/`fill_sorted_instance_list`/`prefix_sum_unsorted_instances_per_atlas_entry`/`group_draw_cmd_prefix_sum`/`generate_draw_lists`（RenderSublistsIter 迭代链），`generate_wall_draw_lists`/`generate_wall_draw_lists_pass2`（墙 draw-list 两轮）。全部 O（wp5 §1 + 本次反编译复证）。
- `systems/render_loop/compute_passes/upload_world_raster.rs` [6] — 世界光栅上传（CPU 光栅 staging → GPU 纹理）：`upload_raster_staging_to_texture<PathRaster>` 等 6 单态（W1：PathRaster/Cobblestone1/2/3/WaterRaster/GardenRaster）。
- `systems/render_loop/compute_passes/grass.rs` [4] — 草地生成：`grass_precompute`（瓦片布局预计算）、`grass_compute`（W29，草地实例生成 + DrawIndirect）、`register_required_components`（bevy 组件注册）、`clone_behavior`（克隆行为胶水）。`0x140002370` 两行是 bevy_ecs `is_exclusive`/派生 clone 共享地址（非业务）。
- `systems/render_loop/compute_passes/apply_screen_shadows.rs` [2] — 屏幕空间阴影应用：`apply_screen_shadows`（W42，应用屏幕阴影到光照）、`compute_shadow_mask_mean`（W47，阴影掩码均值统计回读）。
- `systems/render_loop/compute_passes/blur_pyramid.rs` [2] — 模糊金字塔原语（复用型，非独立帧 pass）：`bidirectional_blur_pyramid`（双向 = 下采样 + 反向上采样）、`blur_pyramid`（下采样）、`rev_blur_pyramid`（反向）。供 GI/nani/后处理复用。
- `systems/render_loop/compute_passes/compute_garden.rs` [2] — 花园掩码/植被生成：`garden_rasterize_from_cpu`（W12，CPU 花园掩码 → GPU 光栅）、`garden_spawn_vegetation`（W16，花园植被程序化生成）。
- `systems/render_loop/compute_passes/compute_path_masks.rs` [2] — 路径掩码计算：`compute_path_masks`（W10，合并路径排除区/方向收缩/可视化掩码）、`blur_path_mask`/`blur_exclusion_mask`（高斯模糊路径掩码）。
- `systems/render_loop/compute_passes/compute_water.rs` [2] — 水面掩码/水上植被生成：`water_rasterize_from_cpu`（W2，CPU 水面掩码 → GPU）、`water_spawn_vegetation`（W13，水面植被程序化生成：lilypads/reeds/boats/waterwheels）。
- `systems/render_loop/compute_passes/dof.rs` [2] — 景深：`dof_prepare`（W49，CoC/半分辨率预处理）、`calc_dof`（W51，多分辨率或全分辨率 DoF 模糊）、`calc_dof_multires`/`calc_dof_fullres`（内联实现）。
- `systems/render_loop/compute_passes/generate_mips.rs` [2] — MIP 生成（SPD 单趟）：`generate_mips<WriteTextureUnchecked>` / `generate_mips<TexturePreset>`（W50，两单态按访问模式/纹理预设）。
- `systems/render_loop/compute_passes/taa.rs` [2] — 时域抗锯齿：`calculate_taa`（W52，TAA 时域积累）、`calculate_tgsr`（TGSR 升采样超分，替代 TAA）。
- `systems/render_loop/compute_passes/apply_volumetrics.rs` [1] — 体积光/雾应用：`apply_volumetrics`（W48，应用体积效果到帧）。
- `systems/render_loop/compute_passes/bin_lights.rs` [1] — 灯光分箱：`bin_lights`（W40，把灯源 bin 到 3D 单元格网格，供延迟光照/ReSTIR 查询）。
- `systems/render_loop/compute_passes/displace_rocky_terrain.rs` [1] — 岩质地形位移：`displace_rocky_terrain`（W4，按 CPU 岩质地层掩码在 GPU 位移地形高度）。
- `systems/render_loop/compute_passes/dof_autofocus.rs` [1] — DoF 自动对焦：`calculate_dof_autofocus`（W32，按主相机焦点/窗口宽高比计算对焦参数写回 SSBO）。
- `systems/render_loop/compute_passes/eval_gi_screen_radiance.rs` [1] — GI 屏幕辐射度求值：`eval_gi_screen_radiance`（W39，GI 候选→屏幕辐射度 + 时域滤波）。
- `systems/render_loop/compute_passes/generate_hzb.rs` [1] — HZB 生成：`generate_hzb`（W26，单趟 SPD 深度层级 Z-buffer，供遮挡剔除/nani cull 第 2 轮）。
- `systems/render_loop/compute_passes/half_res.rs` [1] — 半分辨率深度/法线提取：`extract_half_res_depth_normals`（W35，从主 G-buffer 深度+模板提取半分辨率深度/法线，供 GI/后处理）。
- `systems/render_loop/compute_passes/highlight_active_terrain_stroke.rs` [1] — 活跃地形笔触高亮：`highlight_active_terrain_stroke`（W8，当前地形笔触高亮掩码生成 + 高斯模糊）。
- `systems/render_loop/compute_passes/leaf_particles.rs` [1] — 树叶粒子计算：`leaf_particles_compute`（W18，树叶粒子模拟/更新）。
- `systems/render_loop/compute_passes/mouse_depth_value.rs` [1] — 鼠标深度回读：`mouse_depth_compute`（W33，把鼠标位置写入深度值 SSBO）。
- `systems/render_loop/compute_passes/mouse_read_terrain_stroke_index.rs` [1] — 鼠标地形笔触索引回读：`mouse_read_terrain_stroke_index`（W6，读鼠标下地形笔触索引 SSBO）。
- `systems/render_loop/compute_passes/path_pebbles.rs` [1] — 路径卵石生成：`path_pebbles_compute`（W17，沿路径生成卵石实例）。
- `systems/render_loop/compute_passes/prefix_scan.rs` [1] — 前缀扫描原语：`inclusive_prefix_scan_u32_1m`（≤1M u32 的 inclusive 前缀和，grass/nani 复用）。
- `systems/render_loop/compute_passes/primary_rt_debug.rs` [1] — 主光线追踪调试视图：`primary_rt_debug`（调试用：把 RT 结果/地形/灯光绑定到调试输出）。
- `systems/render_loop/compute_passes/rasterize_buildings.rs` [1] — 建筑掩码光栅：`rasterize_buildings`（W11，把建筑/地面物体光栅到掩码纹理 + 高斯模糊 + 动画掩码）。
- `systems/render_loop/compute_passes/rasterize_terrain_stroke.rs` [1] — 地形笔触光栅：`rasterize_terrain_stroke`（W3，把 CPU 地形笔触光栅到地形掩码/高度 + 高斯模糊 + 高度图重算）。
- `systems/render_loop/compute_passes/ray_trace_gi.rs` [1] — GI 光线追踪：`ray_trace_gi`（W38，软件 BVH GI 候选追踪 + 辐射度更新，供 eval_gi_screen_radiance 滤波）。
- `systems/render_loop/compute_passes/read_bush_positions_for_cpu.rs` [1] — 灌木位置 CPU 回读：`read_bush_positions_for_cpu`（W15，把 GPU 灌木 DrawIndirect/位置回读到 CPU SSBO）。
- `systems/render_loop/compute_passes/read_terrain_data.rs` [1] — 地形数据回读：`read_terrain_data`（W5，组合读地形高度/岩质掩码/水面掩码/花园掩码四个回读 dispatch），`read_terrain_height`/`read_terrain_rock_mask`/`read_water_mask`/`read_garden_mask`（内联实现）。
- `systems/render_loop/compute_passes/reprojection.rs` [1] — 重投影图：`calculate_reprojection_map`（W36，深度→重投影映射图，供时域滤波）。
- `systems/render_loop/compute_passes/reset_indirect_args.rs` [1] — 间接参数重置：`reset_indirect_args`（清空/重置间接绘制参数缓冲）。
- `systems/render_loop/compute_passes/restir_di.rs` [1] — ReSTIR DI（直接光时空复用）：`restir_sample_direct_lights`（W41，ReSTIR reservoir 采样直接光 + 时域复用）。
- `systems/render_loop/compute_passes/roof_tiles_gravity.rs` [1] — 屋顶瓦片重力模拟：`roof_tiles_gravity_compute`（W19，屋顶瓦片受重力位移/掉落模拟）。
- `systems/render_loop/compute_passes/shader_refactoring.rs` [1] — 着色器重构对比：`render_shader_refactoring`（W54，抓取/对比新旧 shader 输出的调试 pass）。
- `systems/render_loop/compute_passes/spawn_vegetation_around_buildings.rs` [1] — 建筑周围植被生成：`spawn_vegetation_around_buildings`（W14，沿建筑周边生成灌木/花）。
- `systems/render_loop/compute_passes/ssgi.rs` [1] — 屏幕空间全局光照：`calculate_ssgi`（W37，SSGI 求值 + 时域滤波 + 上采样）。
- `systems/render_loop/compute_passes/stairs.rs` [1] — 楼梯（岩质台阶）生成：`stairs`（W20，按地形生成楼梯实例 + 间接绘制参数）。
- `systems/render_loop/compute_passes/terrain_lighting_texture.rs` [1] — 地形光照纹理：`calc_terrain_lighting_texture`（W24，按地形高度/阴影计算地形光照贴图）。
- `systems/render_loop/compute_passes/visibility_queries.rs` [1] — 可见性查询：`run_visibility_queries`（W31，GPU 可见性查询回读）。

### systems/render_loop/draw_passes（16 文件 / 31 业务函数）

- `systems/render_loop/draw_passes/common_draw_params.rs` [5] — 共享 draw 绑定参数（所有 draw pass 复用的绑定）：`CommonDrawParams::new`（帧时间 uniform）、`DrawBuilder::bind_common`（shadowmap + Transform 矩阵 + uniform）、墙色/屋顶色自定义、砖擦除高亮、石/木地板纹理绑定。
- `systems/render_loop/draw_passes/transparency_pass.rs` [5] — 透明/水面 pass 族：`refraction_copy_pass`（W44，折射拷贝）、`water_surface_pass`（W45 非冬季，水面 + 软件 BVH 光线步进 + deferred rt）、`ice_surface_pass`（W45 冬季，冰面）、`pre_taa_transparency_pass`（W46，TAA 前透明）、`post_taa_transparency_pass`（W53，TAA 后透明）。
- `systems/render_loop/draw_passes/draw_lists.rs` [3] — CPU 绘制列表容器与构建：`DrawLists`（21 个 `Vec<DrawEntry>`，按图层索引 DRAW_LIST_COUNT=21，wp5 O）、`DrawLists::new`（0x1409ba340）、`build_draw_lists`（Bevy 系统：puffin scope + World 查询 + 按材质 shader 名分派构造 `DrawData::new*` 项）、`DrawLists::sort`（逐图层排序）。大量胶水为按材质族分派的 `DrawData::new*` 单态闭包。
- `systems/render_loop/draw_passes/main_pass.rs` [3] — 主不透明 pass 族：`main_pass`（W25，G-buffer 几何 pass）、`main_pass_oc2`（W30，HZB 后二次遮挡判定的 OC/2 分辨率 pass）、`late_forward_and_deferred_pass`（W34，延迟光照 + nani 前向组合）。均用 `RasterPass::new` + `CommonDrawParams::new` + 遍历 `DrawListItem` → `to_draw_packet` + `with_stencil_write`（主 pass 写 stencil）。
- `systems/render_loop/draw_passes/final_post.rs` [2] — 最终后处理（最终图）：`final_post`（色调映射/grading LUT/ICC LUT）、`resolve_to_swapchain`（MSAA 解析到交换链）。
- `systems/render_loop/draw_passes/logo_overlay.rs` [2] — Logo 叠加层：`LogoOverlayPlugin::build`、`startup`（启动建资源）、`render_logo_overlay`（启动/加载屏 logo 渲染）。
- `systems/render_loop/draw_passes/ui_2d_pass.rs` [2] — 最终图 UI 2D：`ui_2d_pass`（UI 2D 几何/文本光栅）、`mei_pass`（MEI 2D 后处理，UI 阴影合并）。
- `systems/render_loop/draw_passes/ui_pass.rs` [2] — UI pass（世界图内 UI 层）：`ui_pass`、`loading_screen_pass`（加载屏光栅）。
- `systems/render_loop/draw_passes/custom_cursor_pass.rs` [1] — 自定义光标：`custom_cursor_pass`（最终图光标光栅 pass）。
- `systems/render_loop/draw_passes/deferred_raster.rs` [1] — 延迟光照光栅：`deferred_raster`（W43，把 G-buffer 反照率/法线 + 光照累积到光照目标，读 SSGI/屏幕阴影/ReSTIR/灯光网格）。
- `systems/render_loop/draw_passes/feedback_report_bg.rs` [1] — 反馈报告背景：`render_feedback_report_bg`（反馈报告 UI 背景渲染）。
- `systems/render_loop/draw_passes/foliage_exclusion_mask.rs` [1] — 植被排除掩码光栅：`rasterize_foliage_exclusion_mask`（W7，把建筑/墙体光栅到排除掩码，供植被避开）。
- `systems/render_loop/draw_passes/object_grass_push.rs` [1] — 物体草地推入：`object_grass_push_pass`（W9，把相机/光标附近草地实例推入 ObjectGrass 渲染缓冲）。
- `systems/render_loop/draw_passes/shadow_pass.rs` [1] — 阴影图光栅：`shadow_pass`（W23，太阳阴影 atlas 层光栅）。
- `systems/render_loop/draw_passes/wireframe_pass.rs` [1] — 线框调试 pass：`wireframe_pass`（世界图线框叠加）。
- `systems/render_loop/draw_passes/material_id.rs` [胶水/内联] — 材质 ID 类型定义（`MaterialId` 类型 + fmt 实现，无业务函数）。

### systems/render_loop/draw_passes/draw_shaders（16 文件 / 36 业务函数）

- `systems/render_loop/draw_passes/draw_shaders/instanced_critters.rs` [5] — 小动物实例绘制：蝴蝶/萤火虫/萤火虫辉光/鸭/鹪鹩五个 draw 函数（draw_lists 分派目标）。
- `systems/render_loop/draw_passes/draw_shaders/instanced_ivy.rs` [4] — 常春藤实例绘制：`ivy_leaf_shader`（共享实现）、`draw_instanced_ivy_branch_shader`、`draw_instanced_ivy_leaf_shader`、`draw_instanced_ivy_flower_shader`（后两者为带 hash 常量的薄 thunk）。
- `systems/render_loop/draw_passes/draw_shaders/water_planes_draw.rs` [4] — 水面平面/水生植物绘制：`water_planes_draw`、`lilypad_draw`（睡莲叶）、`lilypad_flower_draw`（睡莲花）、`reed_draw`（芦苇）。
- `systems/render_loop/draw_passes/draw_shaders/common_includes.rs` [3] — draw/compute 共用绑定 include：`bind_terrain_heightmap`、`bind_terrain_lighting`、`bind_water_object_push`（ComputePass/DrawBuilder 两单态）、`bind_garden_exclusion`。
- `systems/render_loop/draw_passes/draw_shaders/instanced_roof.rs` [3] — 屋顶实例绘制：`draw_instanced_roof_shader_nani`、`draw_instanced_roof_spire_shader`（尖顶）、`draw_instanced_roof_support_beam_shader`（支撑梁）。
- `systems/render_loop/draw_passes/draw_shaders/billboard_forest.rs` [2] — Billboard 森林绘制 shader：`BillboardForestPlugin::build`、`startup_billboard_forest`（启动加载 `/billboard_forest.raster.hlsl`）。
- `systems/render_loop/draw_passes/draw_shaders/clearing_shaders.rs` [2] — 林中空地树木 shader：`tree_trunk`、`tree_canopy`（树干/树冠绘制，draw_lists 分派目标）。
- `systems/render_loop/draw_passes/draw_shaders/clouds.rs` [2] — 云绘制 shader：`CloudsPlugin::build`、`startup_clouds`（启动加载 `/cloud.raster.hlsl`）。
- `systems/render_loop/draw_passes/draw_shaders/draw_bricks.rs` [2] — 砖绘制：`indirect_draw_bricks<0>` / `<1>`（两单态，间接绘制砖块实例）。
- `systems/render_loop/draw_passes/draw_shaders/grass_shader.rs` [2] — 草地绘制 shader：`draw_common`（公共绑定）、`draw_grass_shader`、`draw_grass_lod1_shader`（LOD1 草地）。
- `systems/render_loop/draw_passes/draw_shaders/stars.rs` [2] — 星星绘制 shader：`StarsPlugin::build`、`startup_stars`（启动加载 `/star.raster.hlsl`）。
- `systems/render_loop/draw_passes/draw_shaders/floor_shader.rs` [1] — 地板 shader：`draw_floor_shader`（木/石地板绘制）。
- `systems/render_loop/draw_passes/draw_shaders/leaf_particle_draw.rs` [1] — 树叶粒子绘制：`leaf_particle_draw`（draw_lists 分派目标，draw_instanced 树叶粒子）。
- `systems/render_loop/draw_passes/draw_shaders/path_pebble_shader.rs` [1] — 路径卵石绘制：`path_pebble_draw`（draw_lists 分派目标，draw_instanced 卵石）。
- `systems/render_loop/draw_passes/draw_shaders/rocky_terrain_draw.rs` [1] — 岩质地形绘制：`rocky_terrain_shader`（岩质地形网格绘制）。
- `systems/render_loop/draw_passes/draw_shaders/stairs_draw.rs` [1] — 楼梯绘制：`stairs_draw`（岩质台阶实例绘制，间接参数）。

### systems/render_loop/painterly（1 文件 / 4 业务函数）

- `systems/render_loop/painterly/mod.rs` [4] — painterly 风格化笔触揭示链（渐进笔触会话动画）：`PainterlyState`（enum2$，判别 +8：Default=2 未启用；变体 0/1 载荷 `{progress f64@0, extent [u32;2]@0xc, timer f32@0x14}`）、`PainterlyPlugin::build`、`painterly_pass`（compute+raster 链）、`startup_painterly`。reconstruction `reconstruction/country-core/src/systems/render_loop/painterly.rs` 已按机器码全量恢复（O）。

### systems/render_loop/session_selection（1 文件 / 3 业务函数）

- `systems/render_loop/session_selection/mod.rs` [3] — 会话选择界面渲染（邻域）：SessionSelectionPlugin 注册；thumbnails_startup / render_session_selection_previews 渲染存档缩略图（从磁盘 screenshot.jpg + meta.json 生成 painterly 缩略图池）。

### systems/save（11 文件 / 52 业务函数）

- `systems/save/save_game.rs` [19] — 主存档系统：save_game（6 段 SystemParam；节拍器内联；capture→Arc→set_current_snapshot(0x140a84760)→发送 AsyncSaveCmd::CaptureSnapshot(9)）；SaveFileSection 10 变体节协议（json_key/padding 表 @0x142ace328/0x142ace378/0x142ace3c8）；save_game_to_file 手工拼 JSON（'{'、"History":{、edit 间 ",
- `systems/save/save_thread.rs` [9] — 存档线程：save_thread 主循环（脏 shadow 文件逐 tick flush → mutex+condvar 等待≤1s → 错误列表 diff/日志 → recv 命令）；execute_save_cmd = AsyncSaveCmd 分发（3=OpenFile/4=CloseFile/5=ChangeSection/6=ReplaceEdits/7=BackupSave/8=ManualBackup/9=CaptureSnapshot/10=Flush/11=Shutdown）；snapshot_gc_thread 清理未引用快照；CurrentSaveFile 增量历史写（Edit appended to save file + mark_dirty 协议）。
- `systems/save/snapshot.rs` [6] — 快照捕获/加载：capture_game_snapshot 巨型系统（184KB）串化全 Query 为 JSON 文档（节序 Version→Walls→Roofs→Decorators→WallColors→GateColors→Terrain(flate2 deflate)→Trees→Raster）；capture_game_snapshot_save_data 构建 0x400B 结构并返回 Arc；validate_and_load_closest_snapshot_data 解析快照 JSON（parsing 全串+Version mismatch）；init_game_state_from_snapshot 应用数据。
- `systems/save/load_completion_checker.rs` [4] — 读档完成检查：SessionLoadReadiness（is_ok/deny_load 常量折叠）、is_session_ok_to_load / check_load_completion = 会话文件存在性检查（save_folder_path + File::create 探测）。
- `systems/save/shadow_file.rs` [4] — 影子文件原子写协议：ShadowFile{+0x28/+0x30 数据 (ptr,len), +0x40/+0x48 文件状态（MIN 哨兵+句柄）, +0x68 脏块计数(8 槽 &7)}；create 初始化；flush = 打开 <path>.tmp→Write data→Setting file length→Sync data→Save file successfully opened and written to disk；sync_all = Persist temp file（Asking the OS to sync writes to disk / flushed_block_count）；mark_dirty(start,end) 记录脏字节区间。
- `systems/save/save_backup_list.rs` [3] — 备份存档列表：SaveBackupList::new_from_session 扫描备份目录（文件名前缀 history-auto）建列表；BackupSaveFileKind::is_corrupted（tag==4）。
- `systems/save/load_game.rs` [2] — 读档主系统：LoadGameManager 状态机（LoadGameState::LoadGameData）→ 写 loading.tmp → NextSessionKind 解码（1=parse_saved_game_data 含 upgrade 备份、2=on_load 命令回放+内存快照）→ PostGameLoadCommand 回放（SwitchAppMode 忽略）→ 旧版颜色 ID 升级（{1→2,2→5,4→8,5→3,8→4}）→ extract_max_wall_id_from_history。
- `systems/save/mod.rs` [2] — save 系统族入口：add_save_load_systems / add_save_load_events_and_resources 注册存档系统与事件。
- `systems/save/save_file_identity.rs` [2] — 存档文件身份识别：try_approx_parse 读文件搜字面量 "Identity" 后解析 JSON 值；derive_fallback 失败回退（文件元数据派生）。
- `systems/save/snapshot_gc.rs` [1] — 快照 GC 辅助：parse_save_file_snapshots 从存档文件解析快照引用集合，供 snapshot_gc_thread 判定未引用快照。
- `systems/save/save_backup_metronome.rs` [胶水/内联] — 备份节拍器（全内联文件，无独立符号）：累积活动游戏时间，达阈值触发备份。O 数值（内联于 save_game 0x140a3f470 行 507-591）：{counter u32@+0x140, 累计 f32@+0x144 clamp[0,1], 阈值 f32@+0x148}；仅阈值<60.0 时累计；档位表 {300,600,3600,7200}s（5min/10min/1h/2h）@0x142a9f348；断言 active_game_delta_seconds >= 0.0。

### systems/steam（2 文件 / 17 业务函数）

- `systems/steam/mod.rs` [16] — **Steam 集成：客户端资源/覆盖层/截图/输入映射。**
- `systems/steam/startup_steam.rs` [1] — **Steam 启动系统（首帧一次性资源插入）。**

### systems/terrain（9 文件 / 12 业务函数）

- `systems/terrain/mod.rs` [3] — **地形系统模块注册：add_terrain_systems/insert_terrain_resources/add_terrain_events 三个注册函数 + 子模块声明 + 跨模块 ECS 类型门面。**
- `systems/terrain/delayed_terrain_changed_msg.rs` [2] — **延迟地形变更：ScheduledTerrainReadback {counter AtomicUsize@0, flag bool@8, delays Vec<u32>}；每帧递减 delays→0 触发 GPU 高度回拷 copy_from_gpu×2（权威+视觉），延迟 2 帧发 OnTerrainChange。**
- `systems/terrain/deferred_terrain_history.rs` [1] — **延迟地形历史：PendingTerrainHistory 脏标记（0x8000000000000002 哨兵）→ 首次变化构建历史记录 History::add + DebugHistoryMsg。**
- `systems/terrain/draw_terrain_stroke.rs` [1] — **笔画变化转发：EvTerrainStrokeChanged 事件 → RecalculateHeightMapCmd + PlayTerrainSound。**
- `systems/terrain/edit_terrain_stroke.rs` [1] — **编辑地形笔画：EditTerrainStrokeCmd/SetTerrainStrokePtsCmd 事件 → TerrainEditingData 查询（缺失 panic）→ falloff/height 应用（Absolute 直写/Delta 限幅）→ RecalculateHeightMapCmd + 声音。**
- `systems/terrain/terrain_hover_idx.rs` [1] — **地形笔画 hover：AppMode 位掩码 0x421 门 → MouseTerrainRaycastSsbo 读笔画 id（+0x10，-1 哨兵）→ 水位/光标/距离 0.5 门控 → 写 HoveredTerrainStrokeId。**
- `systems/terrain/ui_draw_terrain.rs` [1] — **UI 绘制地形：高亮/编辑模式点按 → new_stroke + EvTerrainStrokeChanged；界外反馈（DisplayOutOfBorder/FeedbackInputOutOfBorder）。**
- `systems/terrain/ui_edit_terrain.rs` [1] — **UI 编辑地形：按 hover 笔画 id 生成 EditTerrainStrokeCmd/SetTerrainStrokePtsCmd；维护 MouseActivityTimer/TerrainStrokeParamsCache。ui-input 域共享。**
- `systems/terrain/ui_terrain_focus_mode.rs` [1] — **地形聚焦模式 UI：进入/退出地形聚焦的状态机（167KB 反编译几乎全内联），4 个闭包（曲线采样辅助/高亮/OutlineWallCmd 状态转换）。ui-input 域共享。**

### systems/time_of_day.rs（1 文件 / 2 业务函数）

- `systems/time_of_day.rs` [2] — **昼夜系统**——13 参数系统：太阳角/小时双插值（exp(−3·dt) 平滑、wrap ±π/±0.5）、

### systems/time_update.rs（1 文件 / 1 业务函数）

- `systems/time_update.rs` [1] — **游戏时钟系统**——推进 56B 时钟 {secs,nanos,flag@+0x6,total f64@+0x10,

### systems/timelapse_replay.rs（1 文件 / 4 业务函数）

- `systems/timelapse_replay.rs` [4] — **延时摄影录制/回放状态机**——`TimelapseState` 字节 0=idle/1=recording/2=finished；

### systems/top（1 文件 / 4 业务函数）

- `systems/mod.rs` [4] — **合并模块树（wp1-20 系统模块 re-export）+ `GameSystemSet` 枚举（1 字节判别）+ `is_gameplay_active` 谓词。GameSystemSet 的 dyn SystemSet 方法（dyn_clone/as_dyn_eq/dyn_hash）确认判别为 1 字节。**

### systems/ui（4 文件 / 7 业务函数）

- `systems/ui/debug_visualize_volumes.rs` [3] — 调试可视化：三角形体积线框/球体调试绘制 + 屋顶 UV 线框；DebugRoofs/DebugVolume 资源 + 组件注册（bevy 胶水）。
- `systems/ui/ui_update.rs` [2] — UI 主状态机：遍历 UiState 注册表（HashMap<WidgetId, Arc<WidgetParams>>），每 widget 算 get_ui_volume（世界空间体积）、hover/click/drag 状态转移（HoverState/DragState，按键码 0x5c/0x2f 分支）、PlayUiSound 触发、每帧清交点表。
- `systems/ui/hover_intersection.rs` [1] — 悬停相交检测：par_chunk_map 射线×体积（(WidgetId,&WidgetParams)）→ Vec<WidgetIntersection> + 按 layer/distance 排序的 Vec<(&WorldCanvasLayer, WidgetIntersection)>，结果写入 UiState 上下文。
- `systems/ui/mod.rs` [1] — country-core UI 面：GlobalId（u64 name-hash）、UiSystems::add_ui 插件（Update 调度链 + UiState/UiNavState(1e9ns)/UiSignifierStream(0x170B) 资源 + PlayUiSound/PlayPhotomodeSound 事件注册）。

### systems/ui/resources（3 文件 / 60 业务函数）

- `systems/ui/resources/ui_state.rs` [26] — UiState 权威注册表：RwLock<HashMap<WidgetId, Arc<WidgetParams>>>（锁字@+0x60、表@+0x68）；UiWidgetContext（hover@0x18/click@0x19/id@0x38/交点载荷@0x28..0x58）；clearing_canvas 专用 widget（130×130 体积，tag 4）。
- `systems/ui/resources/widget_params.rs` [20] — WidgetParams 0xb0B 参数块：默认 u32 组 {0,8,0,1,0,2,3,3}@{0,8,0x10,0x18,0x20,0x30,0x48,0x60}、UiSounds 4×24B、WidgetVolumeShape 32B {tag,layer,size}、u16@0xa8=0x101、flags 字节@0xa9（bit0=no_history_recording、bit2=allow_negative_distance）；UiFocus u32 {None=0,Toolbar=1,Mei=2,World=3}。
- `systems/ui/resources/slider3d.rs` [14] — Slider3d：bevy SystemParam（646 new_archetype callers），包装 UiState/UiSignifierStream/CameraCursorRay/游标相机资源 + 拖拽 EventWriter；Slider3dState 世界空间线段拖拽状态；project_cursor_onto_line 游标投影。

### systems/ui/signifiers（4 文件 / 24 业务函数）

- `systems/ui/signifiers/mod.rs` [18] — Signifier 指示符工厂族：SignifierEntityBuilder 0x90B（默认 shader 'vertex_color_shader'@0x60..0x70、position Vec2@0x7c、默认 {0,1.0f64,0x100 u16@0x40,0.2f32@0x78}）；build 三分支（0=命名网格/1=transient/2=mesh_from_pts）生成实体 + SignifierAnimation；SignifierFactory 40B。
- `systems/ui/signifiers/animation.rs` [2] — 指示符动画：signifier_animation 推进 SignifierAnimation::t 并按缓动 scale/alpha（父挂体积原点用 WidgetVolumeShape::get_origin）；signifier_animation_t 每帧 tick。
- `systems/ui/signifiers/signifier_create.rs` [2] — UiSignifierStream：0x170B 无锁环形流（read cursor@+0x160、write counter@+0x168、0x28B 槽）；write 自旋追加；signifier_create 每帧排空并生成指示符实体。（重建代码写成 Vec<UiAppendSignifier>是错误的——真实是环形流。）
- `systems/ui/signifiers/signifier_mark_for_delete.rs` [2] — 指示符删除：signfier_mark_for_delete（PDB 拼写如此）按 signifier_config_hash + UiState 标记删除；despawn_marked 销毁 SignifierMarkedForDeletion 实体。

### systems/ui_2d_follow_mouse.rs（1 文件 / 1 业务函数）

- `systems/ui_2d_follow_mouse.rs` [1] — 2D UI 标记跟随鼠标：双分支 Query（标志@+0x108），源索引 @+0x94（stride 0xa0）/@+0x10（stride 0x48）选每实体槽，写入游标 Vec2 + u32 layer（wp20 合同复核）。

### systems/ui_camera_mode_gizmos.rs（1 文件 / 1 业务函数）

- `systems/ui_camera_mode_gizmos.rs` [1] — **相机模式 gizmo 签名符（小窗口）**——四重门槛：AppMode∈{2,10}（10 需 flag=0）、

### systems/ui_clutter（7 文件 / 8 业务函数）

- `systems/ui_clutter/decoclutter_show_pins.rs` [3] — 显示/隐藏已固定 deco-clutter 图钉（DecoClutterLock、ShowPinnedDecoClutter；wp20 合同）。
- `systems/ui_clutter/debug_spawn_clutter.rs` [1] — 调试批量生成 clutter：消费 DebugSpawnClutterCmd/DebugSpawnAllClutterCmd 事件批量生成。
- `systems/ui_clutter/defered_autoclutter_history.rs` [1] — 延迟记录 auto-clutter 历史（RecordAutoClutterHistory 事件 → history）。
- `systems/ui_clutter/rmb_autoclutter.rs` [1] — 右键移除 auto-clutter（handle_rmb_autoclutter；wp20 合同）。
- `systems/ui_clutter/ui_autoclutter.rs` [1] — 右键把 auto-clutter 转 deco-clutter（InstaCreateDeco/InstaCreateStairPoint/StartPlaceDecoClutterInteraction/EvRightClickedAutoClutter；wp20 合同）。
- `systems/ui_clutter/ui_focus_decoclutter.rs` [1] — 焦点模式 deco-clutter 移动/旋转（RequestMoveClutter/RequestRotateDecorator/LastClonedClutter，4 个 closure；wp20 合同）。
- `systems/ui_clutter/mod.rs` [胶水/内联] — **待推断**（由章节 agent 按函数名+证据填写）

### systems/ui_edit_focus.rs（1 文件 / 3 业务函数）

- `systems/ui_edit_focus.rs` [3] — 编辑焦点系统：按模式 id 分发（mode<6 注册 'edit_focus_canvas' widget（tag5/f32::MAX）；mode 1/2 跟随路径过 FUN_140a1c930 谓词；0x2a8 反向步长遍历 wall 数组并断言 simplified_curve_2d 有效性；mode==10 特判；4 张旋转跳表 = 24 变体 GeoSource 匹配；结尾推 {tag0, 计数} 0x18B 事件）。geo_src_to_edit_subject(_w_deco_clutter) 做 GeoSource→EditSubject 映射。

### systems/ui_edit_freehand_walls.rs（1 文件 / 1 业务函数）

- `systems/ui_edit_freehand_walls.rs` [1] — 自由手绘墙编辑：拖拽笔划进 EditFreehandWallCache，松手提交墙段给 wall-constructor（wp20 合同）。

### systems/ui_focus_mode_shape_height.rs（1 文件 / 1 业务函数）

- `systems/ui_focus_mode_shape_height.rs` [1] — 焦点模式形状高度 gizmo：游标射线投影调整所选形状高度，LerpedGizmoPosition 记录 lerp 位置（wp20 合同）。

### systems/ui_move_freehand_walls.rs（1 文件 / 1 业务函数）

- `systems/ui_move_freehand_walls.rs` [1] — 移动自由墙：拖拽移动选中自由墙段，MoveWallCache 记录在途移动（wp20 合同）。

### systems/ui_move_shapes.rs（1 文件 / 2 业务函数）

- `systems/ui_move_shapes.rs` [2] — 移动形状 gizmo：widget 名 'move_shape'；ui_move_shapes 主系统（12.8KB）处理 gizmo 拖拽→形状位置；move_gizmo_and_trigger_vol 施加 delta 并触发 volume 重算；MoveShapeCache 记录在途移动。

### systems/ui_raise_shape.rs（1 文件 / 2 业务函数）

- `systems/ui_raise_shape.rs` [2] — 抬升形状：widget 名 'outline_wall_bottom_'/'arrow_raise_shape_'/'raise_wall_elevation_'；impl_raise_shape 用 ProportionalScreenSpaceCoord::project_onto_ray 算抬升量 + History::add 记录。

### systems/utils.rs（1 文件 / 1 业务函数）

- `systems/utils.rs` [1] — **systems 层调度辅助。`add_stages` 用 `enum_iterator::All<GameSystemSet>` 枚举全部系统集并调用 `App::configure_sets(Update, ...)`。**

### systems/utils_intersect.rs（1 文件 / 1 业务函数）

- `systems/utils_intersect.rs` [1] — **线 × 形状重叠判定。`line_x_shape_overlap` 对圆形（Circle2d 射线相交 + t∈[0,1]）或矩形（OBB 两轴 orientation_mat2 + Aabb2 射线相交）做射线求交，t 值 fmod 回绕；返回 `{hit, t1, t2}`。**

### systems/wall（13 文件 / 17 业务函数）

- `systems/wall/deferred_wall_history.rs` [4] — 延迟墙历史记录：PendingWallChanged::try_new 把前一墙首/末端点投影到下一曲线（approximate_curve_u：分母下限 1e-8、严格更小、并列取首段、clamp01、严格 low<high，否则 i64::MIN 哨兵）到 12B StateChange range；history_record_wall_changed/merge 记 ConcurrentEdits 组；CompoundEditBuilder::wall_deleted 记 Delete 组。
- `systems/wall/ui_continue_wall.rs` [2] — 续画墙 UI：光标接近已结束墙端点到 InProgressWall::new 续画（GladeBorder::is_pos_inside + 装饰 cull + WallContinueState::display_signifier/finish）；suppress_continue_wall 在修饰键/光标移动时抑制（Res<GameTime>）。
- `systems/wall/calc_wall_bend.rs` [1] — 墙弯折角重算系统：消费 OnWallChanged，读发布曲线前 4 段法向 delta 相邻点积平均到 acos 到 clamp<=1.0（点>5 才算，否则 0），写 WallBendChangedEvent（16B 行）。
- `systems/wall/delete_wall.rs` [1] — 墙删除系统：消费 OnWallDeleted（16B 行），从 PublicWalls.wall_entities_map(+0x30)/WallEntities/WallPlasters 三 map remove_entry，Commands::despawn 墙实体与逐石膏实体（两个类型 id）。
- `systems/wall/draw_wall.rs` [1] — 徒手墙绘制系统（最大生命周期系统）：平滑输入空早退；SnappingMode 吸附光标位置；InProgressWall::new 起笔；CurveNursery 累积笔画；InProgressCurve::add_weight_to_the_last_point；Curve::crashy_new_from_points 到 InnerWalls::update_wall_state_curve + update_freehand_ty。4 ResMut（InnerWalls/InProgressWallResource/CurveNursery/InProgressCurve）。
- `systems/wall/finish_wall.rs` [1] — 墙收尾系统：消费 FinishWall，取进行中墙（无则 panic 'wall_manager has no temp curve'），mark_as_non_enclosed + update_wall_state_enclosure，CompoundEditBuilder 记历史（Create(seed) 到 StateChange）；5 ResMut（含 History）。
- `systems/wall/merge_walls.rs` [1] — 墙合并系统：检测共线/相邻墙（地形采样），Curve::crashy_new_from_points 合并曲线，InnerWalls::new_wall；经 DecoratorArchivist::decorator_cull_allow_any 放行装饰归档重挂。
- `systems/wall/set_wall_style.rs` [1] — 墙样式命令系统：三事件读器（SetWallStyleCmd 0x18B/SetFreehandHeightTypeCmd 0x20B/SetPlasterPeelAmountCmd 0x18B）；style 到 update_style + CompoundEditBuilder 历史；freehand ty 到 update_freehand_ty+update_arch_type；plaster peel 到 update_plaster_peel；样式改变后写 0x20B RequestWallMaterialAudio {tag=0x10, style, pos}。
- `systems/wall/structure_outline.rs` [1] — 结构描边系统：消费 OutlineWallCmd/OutlineStairsCmd，Esc 菜单关闭才画；逐墙 ShapeParameters（as_rectangle/as_circle/as_curve2/center/expand/perimeter）+ 曲线采样到 UiSignifierStream 描边 signifier（outline_wall_top_/bottom_/roof_ridge_/roof_side_/circle_*/rect_corner_ 名前缀）。
- `systems/wall/ui_draw_wall.rs` [1] — 墙绘制 UI 入口：glade 内拖动且非清画布到 RecordMouseInputIntoWall 转发进 CurveNursery；释放到 WallDrawState::finish + FinishWall。5 ResMut（含 WallDrawState/CurveNursery）。
- `systems/wall/upload_instanced_wall_ssbo.rs` [1] — 实例化墙砖 SSBO 上载：AssetSsboLibrary::get_mut<BrickTransformSsbo>（缺失 unwrap panic）；逐 InstancedWall 脏区间（+0x108 compact 标志双路径）upload_to_storage_buffer；SparseInstanceBuffer::gpu_flush。GPU 前段。
- `systems/wall/wall_highlight.rs` [1] — 墙高亮系统：消费 HighlightWallCmd，WallEntities 查实体，写 WallHighlight{entity, wall, ty}；EditingHighlight rgba 按 ty 四常量选择。
- `systems/wall/wall_splitter.rs` [1] — 墙分裂系统：光标拖过墙到 get_curve_segment_from_u/get_u_at_coord 求 split u 到 ChildWall::new 两子墙到 InnerWalls::new_wall；命中处 WallStyle::convert_to_stone_version；装饰经归档资源重挂（split_wall/split_wall_scissors signifier）。

### systems/wall/adjust_wall_height（5 文件 / 10 业务函数）

- `systems/wall/adjust_wall_height/ui_adjust_wall_height_mode.rs` [4] — 墙高调整模式 UI：clamp_wall_height（机器码 = PublicWalls 查墙到 roof_entity 到 bool；数值钳制策略见 S02 height_rules，名实冲突待裁决）、history_record_adjust_wall_height（ev_height_changed）、get_closest_point_u_to_cursor（points_with_u/segments_with_u 投影）、模式进出 + 光线拾取 + TestCube + 装饰地形 cull。
- `systems/wall/adjust_wall_height/adjust_wall_ui_mode.rs` [2] — 墙高 widget/曲线 UI：UiWallHeight{widget,signifier,new} + adjust_wall_height_curve 插值曲线（widget 层）。
- `systems/wall/adjust_wall_height/signifier_animation.rs` [2] — 墙高 signifier 动画：平屋顶无 rim（Roof::is_flat_w_no_rim）到 adjust_wall_height_curve 重算 + crashy_new_from_points；否则 SignifierAnimation::t 驱动。
- `systems/wall/adjust_wall_height/adjust_wall_height_impl.rs` [1] — 墙高命令执行：AdjustWallHeightCmd 0x30B{ty u32@+0x10, height f32@+0x14, WallId@+0x18, follow-terrain}；clamp 回退命令高度；follow-terrain 采样 TerrainHeightsData；预置形状到 _try_update_state<update_wall_state_height>（×2）+ 0x28B 事件{WallId,pos+height,u32,t,3}，否则 0x20B RequestWallMaterialAudio。
- `systems/wall/adjust_wall_height/ui_adjust_wall_height.rs` [1] — 墙高 UI 入口：未拖动时选中预置墙到 UiWallHeight::new 开高度控件；roof flat/no-rim/动画分支；写 AdjustWallHeightCmd。

### systems/wall/eraser（4 文件 / 12 业务函数）

- `systems/wall/eraser/archivist.rs` [7] — 擦除归档器：WallEraserArchivist 累积 original/split/rewire 事件（cache_if_original_wall/shrink_wall/add_split_event/add_fully_erased_event），finish 经 CompoundEditBuilder::build 编译 WallHistoryCompoundEdit；断言 original_wall_state.curve.is_valid()/!before_wall.curve.is_empty()；ChildWalls/WallHistoryEdits。
- `systems/wall/eraser/ui_bulldozer.rs` [3] — 推土机 UI 入口：wall_eraser_bulldoze_condition = AppMode{tag==9, +4==3}；handle_decorators 排空墙属装饰 + terrain/stair cull；can_reconnect_stairs_if_midpoint_deleted；StairsState::disconnect_edge；命中到 AnimateWallBulldozeCmd 入 buffer（RoofDelete FIRST + WallCompound StateChange+Delete 双组）。
- `systems/wall/eraser/animate_wall_bulldoze.rs` [1] — 墙推土动画：AnimateWallBulldozeBuffer 152B 命令行{Source112, WallId+0x70, low/high+0x78/+0x7c, anchor+0x80, phase+0x8c, finished+0x90}；每帧 phase=clamp01(phase+dt/0.2)（先推进后应用 CubicInOut）；增长写 Source 切片+enclosure tag2+style；完成移除 authority（128 场景/768 全 RET 先验验证）。
- `systems/wall/eraser/wall_eraser.rs` [1] — 徒手墙擦除系统：拖笔选段到 WallEraserArchivist::affected_walls/has_anything_been_affected 到 InnerWalls::get_unwrapped/update_wall_state(_curve/_enclosure) + new_wall；determine_freehand_height_ty、wall_height_based_on_length、adjust_wall_height_if_erased_flat_roof_w_no_rims（PublicWalls::roof_entity ×4）；ChildWall::new 子墙。

### systems/wall/shapes（4 文件 / 6 业务函数）

- `systems/wall/shapes/shape_placement.rs` [2] — 形状放置参数：光标地形位到 ShapePlacement::new_unsnapped；closest_segment_to_snap_to（WallSpatialHash 查询 + distance_segment_point）；snap_to_roof_ridge（Roof::pivot_3d/RoofRidgeDims::calc）。
- `systems/wall/shapes/shape_placement_signifiers.rs` [2] — 形状放置 signifier：吸附时 preview_snapping_line（ShapeParameters::expand + Curve::get_tangent_at_u/try_slice）+ 形状 outline；断言 !u.is_nan()/!u.is_infinite()。
- `systems/wall/shapes/calc_shape_height.rs` [1] — 形状预览高度计算：shape{tag=6,ty} ty1=circle/2=rectangle 否则高度 0；as_curve2 到 crashy_resample_remap_per_segment 地形采样；base rect 6.0/circle 8.0；height-ty 2 到 +地形否则 avg+base-0.46；final-(max-0.46) clamp[0.7,14.0] 写预览+0x40 + time_continuous_lerp。
- `systems/wall/shapes/mod.rs` [1] — 形状子模块根：add_wall_shape_ux 注册 CalcShapeHeight/PreviewCircle/PreviewRectangle/ShapePlacement 四资源。

### systems/wall/shapes/circle（4 文件 / 6 业务函数）

- `systems/wall/shapes/circle/ui_create_circle.rs` [2] — 圆形绘制 UI：CameraCursorRay::intersect_with_xz_plane 拖拽半径到 PreviewCircle；circle_signifier（custom shader）。
- `systems/wall/shapes/circle/ui_edit_circle.rs` [2] — 圆形编辑 UI：update_circle_radius（拖触发体 WallTriggerVolume::is_valid/edit_shape_default）到 update_wall_state_curve/update_wall_state_enclosure；finish_circle_edit 历史。
- `systems/wall/shapes/circle/create_circle.rs` [1] — 圆形墙创建：CreateCircleCmd 到 Circle2d::as_curve2 到 crashy_new_from_points、enclosure=circle、style/elevation、RoofShapeParams::from_wall_shape + roof_height_01_from_wall_shape 到 CreateRoofCmd、历史。
- `systems/wall/shapes/circle/ui_rotate_circle.rs` [1] — 圆形旋转 UI：world_space_to_screen_space + DebugLine 旋转到更新曲线/enclosure；history 复合编辑。

### systems/wall/shapes/circle/signifiers（2 文件 / 5 业务函数）

- `systems/wall/shapes/circle/signifiers/edit_arrow.rs` [3] — 圆形编辑箭头 signifier：Query 驱动箭头位姿（edit_circle_ 前缀）。
- `systems/wall/shapes/circle/signifiers/edit_line.rs` [2] — 圆形编辑半径线 signifier：Query 驱动半径线位姿。

### systems/wall/shapes/rectangle（8 文件 / 14 业务函数）

- `systems/wall/shapes/rectangle/ui_edit_rectangle_orientation.rs` [4] — 矩形朝向编辑 UI：box_trigger_volume/skewed_box_trigger_vol 旋转到 EditRectangleCmd/FinishEditRectangleCmd；StairsPillarProposals 变体。
- `systems/wall/shapes/rectangle/create_rectangle.rs` [2] — 矩形墙创建：CreateRectangleCmd 到 Rectangle2d::as_curve2 + smoothstep 角 + ridge_length_01_from_rectangle_ratio + roof_height_01_from_wall_shape 到 CreateRoofCmd、历史。
- `systems/wall/shapes/rectangle/signifier_edit_dims_line.rs` [2] — 矩形尺寸编辑线 signifier（rectangle_edit_dims_line_ 前缀）。
- `systems/wall/shapes/rectangle/ui_create_rectangle.rs` [2] — 矩形绘制 UI：rectangle_dims_from_cursor_pos + get_rectangle_orientation 到 PreviewRectangle。
- `systems/wall/shapes/rectangle/ui_edit_rectangle_dims.rs` [2] — 矩形尺寸编辑 UI：get_distance_to_edge 拖边到新 dims 到 EditRectangleCmd；decorator_cull_allow_any。
- `systems/wall/shapes/rectangle/edit_rectangle.rs` [1] — 矩形编辑命令：EditRectangleCmd 到 update_wall_state_curve + update_wall_state_enclosure。
- `systems/wall/shapes/rectangle/finish_edit_rectangle.rs` [1] — 矩形编辑收尾：UiState80 快照先发；维度 state1/朝向 state2；改变判据 abs(acos(prev·cur))*57.29578>0.01 或维度差>0.01；记 ConcurrentEdits + niche8000000000000004；无变化到 DecoratorArchivist 恢复（192 全原调用验证）。
- `systems/wall/shapes/rectangle/mod.rs` [胶水/内联] — rectangle 形状子模块根。

### systems/wall/shapes/rectangle/ui_modes（2 文件 / 4 业务函数）

- `systems/wall/shapes/rectangle/ui_modes/edit_dims.rs` [2] — 矩形尺寸编辑 widget/signifier/trigger_volume（UiRectangleDimsEdit，widget 层）。
- `systems/wall/shapes/rectangle/ui_modes/edit_orientation.rs` [2] — 矩形朝向编辑 widget/signifier/trigger_volume（UiRectangleOrientationEdit，widget 层）。

### systems/wall/shapes/utils（1 文件 / 1 业务函数）

- `systems/wall/shapes/utils/shape_signifier_lerp.rs` [1] — 形状 signifier 插值：ShapeVal{interpolate,get_radius,get_rectangle_dims}、ShapeSignifierLerp::lerp_radius——circle/rectangle 值间插值（glam 复用）。

### systems/wall/snapping（7 文件 / 10 业务函数）

- `systems/wall/snapping/snapping_maintain.rs` [3] — 吸附维护：snapping_maintain = mod key 切换 + NotifyHintSystemSnapping（跳表分发）；check_for_disable_snapping = mode 在 {2,3} 到写事件；snapping_signifiers 双查询画段/端预览线。
- `systems/wall/snapping/telegraph_walls_are_merged.rs` [2] — 合并预告线系统：PlayLineAcrossWall{a,b} 到 Curve::crashy_new_from_points+crashy_slice 连线两墙端点到 LineAcrossWallSignifier（animation_duration/custom_shader）。
- `systems/wall/snapping/mod.rs` [1] — 吸附模块根：SnappingMode{mode u8@0, snapped bool@1}（mode 1=段吸附 2=端吸附）+ +0x20 声音状态字节；is_active/position_to_snap_to。
- `systems/wall/snapping/snapping_audio.rs` [1] — 吸附音效系统：mode==2 且声音状态字节(+0x20)!=2，或 mode==3 且 <=1 到写 PlayWallUiSound。
- `systems/wall/snapping/snapping_init.rs` [1] — 吸附初始化：重置 mode/snapped；DebugGui 禁用或进行中墙空返回；首段长<=1.65 返回；后续点投影首段 <=1.5 到 mode=1+snapped。
- `systems/wall/snapping/snapping_to_ends.rs` [1] — 端吸附系统：非 enclosure 公共墙；InProgressCurve::first_raw_input_point 投影墙曲线（get_pos_at_coord/get_pos_at_u/start_pos）到 find_closest_end_to_snap_to（u=0/1）到 mode=2。
- `systems/wall/snapping/snapping_to_segments.rs` [1] — 段吸附系统：raw_input_curve_length_ws；closest_segment_to_snap_to（get_curve_segment_from_u/get_tangent_at_u + distance_segment_point）最近段到 mode=1。

### systems/wall/valid_enclosure（1 文件 / 20 业务函数）

- `systems/wall/valid_enclosure/mod.rs` [20] — 有效围合 ShapeParameters：Curve/Circle/Rectangle 枚举（tag@+0 位 0 判别、载荷@+4）；center/as_curve2/as_curve3/as_circle/as_rectangle/contains_point/contains_shape/perimeter/get_normal_at_u/get_pos_at_u/get_closest_u_from_pos/expand；serde 名 IsEnclosedNoYesPremadeShapeIsEnclosedV2。

### systems/wall/wall_state（6 文件 / 28 业务函数）

- `systems/wall/wall_state/wall_spatial_hash.rs` [23] — 墙空间哈希结构：固定 64×64 格（walls-audit）；add_segment 逐格填 0x38B 成员 {WallId, 段数据}（格条目 0x30B 链表）；remove_wall_id；get_cells_from_aabb/circle/rectangle2d/segment 栅格化查询（断言 RESAMPLING<cell_size 与 bbx 越界）。
- `systems/wall/wall_state/debug_viz_threshold_curve.rs` [1] — 墙高阈值曲线调试：逐公共墙沿发布曲线画 DebugSphere。
- `systems/wall/wall_state/publish_public_wall_state.rs` [1] — 公共墙状态发布（权威流）：14 系统参数；收集 lazy 更新源到逐墙重建 public 记录（源曲线点/形状/style halftimber/material_stats/SparseInstanceBuffer::allocate/哈希段维护）；AnimateWallBulldozeBuffer::contains 保留动画墙；发 OnWallChanged/OnWallDeleted（16B 行）；断言 !is_adding 与 w.simplified_curve_2d.is_valid()。
- `systems/wall/wall_state/spatial_hash_debug.rs` [1] — 空间哈希调试可视化：DebugGui 开时逐格 DebugRectangle2d + 成员线（MouseDepthValue 平面）。
- `systems/wall/wall_state/spatial_hash_update.rs` [1] — 空间哈希同步系统：OnWallChanged 到按 simplified_curve_2d 逐点 get_pos_at_coord/sample_at_coord 建段到 add_segment（缺墙 panic 'SpatialQuery received OnWallChanged with wall index'）；OnWallDeleted 到 remove_wall_id。断言 (0.0..=1.0).contains(&t)。
- `systems/wall/wall_state/wall_roof_hashmap.rs` [1] — 墙到屋顶映射维护：Query<(Entity,&Roof,Option<...>)> 遍历，维护 PublicWalls.roof_entity 与 WallEntities.wall_entity；预置形状无 roof 到清除。

### systems/weather_system.rs（1 文件 / 1 业务函数）

- `systems/weather_system.rs` [1] — **冬季雪强振荡**——`glade_type@+0x58==4`（冬）时，10 分钟正弦

### systems/world_raster.rs（1 文件 / 18 业务函数）

- `systems/world_raster.rs` [18] — **世界栅格系统：record_raster_history（栅格历史记录）、upload_world_raster_buffers_to_staging（脏流门 + 6 光栅发布到 staging SSBO）、draw_raster<T> 泛型状态机（10 实例：Path/Cobblestone1-3/Garden/Water，start_new→add_point→resample_and_smooth→rasterize_stroke→copy_to_staging 链）。**

## utils（8 文件 / 68 业务函数）

### utils/top（8 文件 / 68 业务函数）

- `utils/dirty_tiles.rs` [27] — **64×64 世界脏瓦片位图（512 B）+ tabloid State/StateDelta 协议**。`DirtyTiles` 同时是
- `utils/debug_viz.rs` [17] — **调试可视化图元**（DebugSphere / DebugColor / DebugRectangle3d / DebugRectangle2d /
- `utils/geo.rs` [6] — **曲线/网格几何重建**。`update_curve_mesh`（4 泛型实例）把曲线点迭代器生成曲线网格
- `utils/load_json.rs` [5] — **JSON→Mesh 网格加载器 + JSON 属性读取**。`load_json_as_mesh`（4 泛型实例：
- `utils/mod.rs` [5] — **游戏自有杂项工具**（事件注册表读取、地图边界、屏幕投影、资源取值、标题转换）。
- `utils/audio_utils.rs` [4] — **循环音效播放包装**。`AudioLoop` 是**自包含游标结构（非 Entity 非 Resource，≈0x38 B）**，
- `utils/project_cursor_onto_ray.rs` [3] — **屏幕比例空间 + 光标→世界射线投影**。`ProportionalScreenSpace` = 16 B
- `utils/child_wall.rs` [1] — **墙分割子墙工具**。`ChildWall {a, b}` 子墙；`slice` 读 a.y/a.z 两标量分别
