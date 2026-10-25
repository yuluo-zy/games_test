> 历史说明：2026-10-17 文档整理前的 README 快照。以下功能、缺口和验证次数只表示原记录时点；当前模块说明请读 [README](README.md)。

# country-core

当前恢复入口：[按源码目录的依赖与推进顺序](recovery/README.md)、[逐目录进度](recovery/DIRECTORIES.md)、[逐文件源码/证据/消费者台账](recovery/inventory.json)。以 B PDB 的 539 个文件身份为主键；旧重建目录、章节和辅助适配文件分别登记，不混算完成度。下文早期快照不能替代该台账的限定核验状态。

2026-10-14 基础合同续轮已在 core 内补齐窗口尺寸、UI 设置 serde/default、菜单读取谓词、亮度表、事件消费、共享 Transform 转发和 DirtyTiles 世界坐标转换；统一 A/B draw 高亮资源，并恢复真实 ECS `wall_highlight` 写入者。完整 core 仍未复现完毕，GUI 载荷、完整 PublicWalls、启动/调度与其它业务链仍按目录推进。

游戏侧渲染声明、资产注册和已经恢复的 CPU 领域规则。目录按 B 版原功能树组织，正常构建直接编译这些实现；完整 GPU 桥、游戏 ECS 场景和未知业务不靠占位算法补齐。

此前正式工程这里只保留了墙高、屋顶等 8 个局部文件，确实漏掉了 Git 中后续提交的渲染恢复。此次按 Git 记录重新整理了已实现函数及其必要输入/后端接口，没有直接搬入旧的整树骨架。

## 功能与代码位置

| 正式模块 | 概述及已恢复功能 | 仍需完成 |
|---|---|---|
| `src/resources/walls/{inner_wall_state,inner_walls}.rs`、`src/systems/wall/wall_state/` | B 权威墙集合、ID 分配/显式恢复、字段编辑、新建/修改/删除通知，以及 core 内真实资源驱动的采样几何与规则墙高度缓存；原指令确认 freehand_ty 在 +0x66，+0x60 为石膏剥离量 | 当前发布输出是限定 CPU 资源 `WallPublicationState`；完整 PublicWalls、地形脏区路由、空间/屋顶/历史消费者和 serde 仍未接通；见 [本轮审计](../../provenance/core-wall-authority-2026-10-14.json) |
| `src/render/pass_builder/` | Shader URI、反射绑定查找、资源读写访问声明、uniform、compute dispatch、光栅附件、绘制包及 draw builder 状态/快照规则 | 与真实 rhapsody graph/recording/descriptor 的统一适配；全部原入口 ABI 和整链行为验收 |
| `src/render/` | Typed/Mapped/Instance/Shadowed/Sparse buffer 的容量、上传区间、脏范围及稀疏分配；Mesh/VAO、纹理包解析、shader 程序缓存、view/frame/灯光常量 | GPU 对象生命周期接线、实际上传/管线创建；部分描述字段语义仍只有调用点合同 |
| `src/render/nani/` | 27 个真实子集、实例 ID/稠密索引与数据更新、mesh atlas、meshlet 切块/属性打包、包围球、间接绘制及 cull shader 描述；`instance_updates.rs`：各 `nani_update_*` 系统的实例载荷构建函数 `build_*`（plant/solid_vertex_color/albedo_map/window_glass/bricks/wren/gothic/flower/curved_halftimber/stone_floor/lantern/plaster/cloth/smoke/balcony_door/uber/velocity_only/stairs_floor）及打包辅助、50 项测试 | `nani/systems.rs` 已落码（maintain、20 个 update、flush，行视图 + 纯函数，见 [R8](../../../reverse-engineering/tiny-glade/evidence/country-core-audit/R8_nani_systems.md)）；仍缺 19 个 gap 组件的真实字段布局、`startup_nani`/`load_mesh_atlases`/`reload_mesh_src`；证据见 [R2_nani_instance_updates.md](../../../reverse-engineering/tiny-glade/evidence/country-core-audit/R2_nani_instance_updates.md)；原整帧验证 |
| `src/asset_libraries/` | Mesh/VAO/shader/SSBO/texture 的名字与类型索引、注册/重载策略、颜色槽、placement data、audio trigger collection；文件监听策略 | 完整资产启动生产者、网格 JSON 加载 schema；原 frozen 本地库来源与实现身份 |
| `src/systems/render_loop/compute_passes/` | 地形/水/建筑栅格、草地/植被、光照/GI、HZB/mips、TAA/TGSR、DoF、读取及间接绘制等 pass 的参数、绑定、资源尺寸、顺序和 dispatch 规则；`DofAutofocus`/`DofFocusReadback`/`ShadowMaskHistogram`/`ShadowMaskMean` 四个 SSBO 的真实元素布局（HLSL + `new_storage_buffer` 汇编核对，`BUFFER_LEN>0` 的 `new_storage_buffer` 形态） | shader_refactoring 状态 6–9、reset_indirect_args 临时区等源合同中的未恢复分支；真实 GPU backend |
| `src/systems/render_loop/draw_passes/` | 主、阴影、透明、水/冰、UI 等 raster pass，draw shader 绑定及已恢复 draw callbacks | 不完整游戏资源的正式类型生产者；各 `*View` 目前是显式输入视图，不能称完整原资源 ABI |
| `src/systems/render_loop/` | `render_world` / `render_frame` CPU 帧编排、Painterly/Vermeer、预览/LUT gallery/session selection 的参数和分支策略 | `FramePasses` 到真实各 pass/backend 的消费桥；LUT 文件加载与全游戏场景 |
| `src/resources/` | 已验证墙高规则、屋顶/窗口局部规则，渲染资源布局/默认值、会话主题谓词及明确的渲染输入视图 | 完整墙/存档/会话资源图、组件身份和未知字段 |
| `src/resources/grading_lut.rs`、`grading_lut_cube.rs` | 调色 LUT 的 `.cube` 加载：逐行解析（TITLE/DOMAIN_MIN·MAX/LUT_1D_SIZE 拒绝/LUT_3D_SIZE/数据行，含原错误文案与行号上下文）、RGB9E5 打包（`utils::rgb9e5`）、`TextureDesc::new_3d(..,0x7b).sampled()` 建 3D 纹理 | `load_unwrapped`（PNG）、`GradingLuts::load`（闭包 0x140ae5bb0 未反编译）、`ColorGradingState::flattened`；`TextureDesc` 未建模维度码 `+0x1e`；证据见 [R1_grading_lut.md](../../../reverse-engineering/tiny-glade/evidence/country-core-audit/R1_grading_lut.md) |
| `src/utils/dirty_tiles.rs`、`src/resources/{world_raster,world_raster_defs,brush_sizes,active_raster_stroke,terrain_height}.rs` | 世界栅格与地形的 CPU 侧：统一的 64×64 `DirtyTiles`（建筑碰撞的 `DirtyTiles64` 现为其别名）；`WorldRaster<512>` 的双缓冲、`rasterize_stroke`（zeno，三种混合）、瓦片哈希（ahash 回退，全零瓦片 = 初始状态字 `0x07954dbb`）、`copy_to_staging` 与脏位图发布；6 个命名栅格和 `AllWorldRasterWrite`；`BrushSizes`；6 槽 `ActiveRasterStroke`（`add_point` 状态机、固化、10 遍平滑）；`TerrainHeightsData` 的双线性采样、`copy_from_gpu` 与 32 步 + 7 次二分的 `intersect_ray`。逐项见 [R5](../../../reverse-engineering/tiny-glade/evidence/country-core-audit/R5_world_raster_terrain.md) | `draw_raster` 状态机（`rasterize_world_space_stroke` 与 `conservative_line_raster` 已在 `utils/`，见 [R10](../../../reverse-engineering/tiny-glade/evidence/country-core-audit/R10_conservative_line_raster.md)，缺 EXE 差分验证）、存档 serde、`+0x29b` 置位点、`cell_from_world_space`、`draw_raster` 状态机、`mouse_terrain_raycast`、`TerrainRockMask`；渲染 pass 里的栅格旗标替身尚未接线到本类型 |
| `src/utils/{project_cursor_onto_ray,load_json,geo,debug_viz,child_wall}.rs`、`src/utils/mod.rs` | 屏幕空间换算（`ProportionalScreenSpace`、`project_onto_ray`、`world_space_to_screen_space`）、JSON 网格加载、`mesh_from_pts` / `update_curve_mesh` / `rewire_enclosed_curve`、`debug_viz` 三类构造器、`ChildWall::slice`。逐项见 [R6](../../../reverse-engineering/tiny-glade/evidence/country-core-audit/R6_utils_screen_space.md)、[R7](../../../reverse-engineering/tiny-glade/evidence/country-core-audit/R7_utils_remainder.md) | `cell_from_world_space`；`get_last_event`/`drain_events`（bevy 资源访问，归 ECS 层）、`audio_utils`（归音频）、`debug_viz::*::show`（`Commands` 闭包） |
| `src/startup/startup_terrain.rs` | `grid_gen_cw` / `grid_gen_ccw`（地形网格三角形索引，4 列分块顺序、CW/CCW 两种缠绕）。见 [R11](../../../reverse-engineering/tiny-glade/evidence/country-core-audit/R11_startup_terrain_grid.md) | startup 其余约 60 个函数（`load_prefabs`、`startup_terrain` 本体、`startup_walls`……），依赖 ECS 组件层与资产库 |
| `src/resources/gothic_window_profile.rs` | Gothic 半老虎窗/门的两组完整轮廓函数；6 个变体共 41 个点逐 f32 位核对现有原码常量 | Rust 参数宽度、原 panic 文案和完整开口消费者 ABI 未由这些常量测试证明 |
| `src/systems/camera_rig/` | `default_vertical_fov_degrees`；`MainCamera`（0x4e0 B）的已核对部分：13 模式槽与 `previous`、fov 偏移表、`store_parameters`、宽高比；Free 相机（dolly 0.6.0 `CameraRig`，`match_normal`/位置/yaw-pitch/update）。见 [R9](../../../reverse-engineering/tiny-glade/evidence/country-core-audit/R9_camera.md) | `MainCamera::new`、`main_camera_update` 跳表、六个 body、`translate_screen_space`、快照插值 |
| `src/systems/mode_manager.rs` | 模式切换中枢的已恢复部分：`esc_menu_canvas` 悬停早退、`Events` 读游标排空并只处理最后一条请求、分发器 `check_if_can_switch`（0x140afa8a0）两个分支、`send_mode_just_changed_event`（0x140a875e0）、类别闭包 env_3 / env_14 / 通用推进。`mode_manager` 调分发器时 `force` 恒为真（0x140a86c94），此前文档有误。见 [ui-input/ALGORITHMS](../../../reverse-engineering/tiny-glade/evidence/country-core-deep/ui-input/ALGORITHMS.md) §2.1 | 按模式 id 的跳表（0x142ad890c）逐块、`AppMode` 完整变体表、`hash("esc_menu_canvas")`（0x1405bdda0）、timelapse 闭包 env_0、各事件的原类型名 |
| `src/resources/toolbar.rs` | `Toolbar`/`ToolbarCategoryModes`/`ModeEntry` 布局，类别推进规则 `advance_category`（含越界 panic 与哨兵 0x15） | `Toolbar::new` 各类别模式表、`contains`/`switch_mode` 逐变体比较（`contains` 是 derive(PartialEq)，不是区间匹配）、`ModeEntry` 变体负载语义；`mod_key` 的 `Modifiers` 位定义与 `update_mod_key_state` 地址未核实 |
| `src/systems/camera_rig/normal_body.rs` | `NormalCamera` body（0xb8 B）的 `zoom`（0x140911280，按汇编手解；系数 0.05，最小距离表 `[3, 40]`，锚点水平距离须 `< 130`）、`rotate_yaw_pitch`（0x140949cc0 + 求和辅助 0x141625db0）、`constrain_to_be_above_terrain`（0x140949dc0）。`zoom` 跳表只有 6 项，Free/Walk/Butterfly 为空操作。见 [ALGORITHMS](../../../reverse-engineering/tiny-glade/evidence/country-core-deep/camera-time/ALGORITHMS.md) §1.4 / §2.5 | `update` 0x140949fe0、`translate_screen_space` 0x140949e70（旋转四元数来源未证）、`load_parameters`、`MainCamera::new`、`camera_snapshot`、`main_camera_update` 跳表 |
| `src/resources/game_clock.rs` | `time_update`（0x140922c30）的计时部分：门控（tag 10/7/2）、前两帧 1/60 上限、`Duration` 累加与两处 panic、`f64` 总时长、60 分之一刻进位与 `frac`。`nanos` 在 `+0x8`（此前文档写 `+0x4` 有误） | 重放帧的下标运算（调用方传 `DeltaSource::Replay`）、各字段原资源名 |
| `src/resources/logo_animation.rs` | 启动 logo 五阶段进度：`build` 默认值（时长 `[1.15, 0.15, 0.65, 1e10, 0.5]`）、`play`/`is_waiting`/`is_finished`/`finish_wait`、`animate_logo`（0x140b1a6b0，进度下界 `f32::MIN_POSITIVE`，阶段 0/2 在进度恰为 0 时返回音效）。布局是 `progress[5]` + `durations[5]` | 音效引擎调用（返回 `LogoAudioEvent` 由调用方播）、`startup_animated_logo` 注册 |
| `src/resources/save_file.rs` | 存档的已恢复纯逻辑：10 个存档节的键与预留字节（第 9 项键是 `"snapshots"`）、影子文件 4 KB 脏块位图（原版用 `bitvec`）、`ensure_current_snapshot` 追加编辑（仅在偏移表非空时写 `",\n"`）、读档两处颜色 ID 升级（位 48 写 999，位 40 做 `{1→2,2→5,4→8,5→3,8→4}`）、版本开关（`>= 0x1a` / `> 0x1a`）。见 [ALGORITHMS](../../../reverse-engineering/tiny-glade/evidence/country-core-deep/session-save/ALGORITHMS.md) §2.3–§2.7 | 备份节拍器、`CurrentSaveFile` 三种形态、存档线程与快照 GC 的 I/O、`AsyncSaveCmd` 0/1/2、`SessionMeta` 字段含义 |
| `src/asset_libraries/transient_mesh.rs` | `TransientMesh` 组件与延迟删除链：`new_with_registry`（仅 `Arc::clone`）、`Drop` 入队、`drain_meshes_to_delete`（锁内遍历、只清 `len` 保留容量、中毒即 panic）、`delete_dropped_transient_meshes`（先网格库后 VAO 库）。注册表是 `AssetMeshLibrary.transient_meshes`，不是独立 `Res`。57 个组件的 `register_required_components` **全部没有 required 组件**，所以不存在需要接线的映射 | `cache_prev_object_transforms`、`InstancedWall` 上传、cube/plane 网格生成 |
| `src/systems/ivy.rs` | `closest_segment_index`（0x14094c800）：过滤方向（仅两端都 `> max` 才跳过）、到直线的垂直距离、初始半径 100、`lerp(extra_a, extra_b, t)`、`t` 越界 panic。此前文档的过滤方向与距离度量均与机器码不符 | `ivy_grower` 10 相跳表、生长/修剪系统 |
| `src/systems/paths.rs` | `DrawState::update`（0x140ae4bd0）：三参数（`mode_id` 做 `u8` 回绕减 2，`raw` 在 `r ∈ {1,2}` 时写入）、笔刷常量 0.5525 / 0.8 | `draw_path` 主链（9 个 `draw_raster` 单态） |
| `src/systems/collision.rs` | `SubworldIndex`：17 槽、Terrain=5（位标志 `0x20`）、空句柄判别 `0x11` | 其余 16 个槽的名字映射、`flush_colliders`/`raycast`/`spherecast`（依赖 rapier 接线）、`StairsState`（依赖 petgraph `GraphMap` 与 `iter_edges_and_stubs` 语义） |
| `src/resources/audio_instance.rs` | `start_if_not_playing` 判定（掩码 `0xb`，状态按 `& 31`）、`AudioLoopPlayer::request_play`（FMOD 经 `LoopInstance` 注入；实例步长 0x30） | `MusicPlayer::update`（阈值常量语义未决）、`Bank::load` 等 FMOD I/O、Steam 动作映射 |
| `src/input/` | 顶层 `input` 模块：`action_set`（46 个动作名/槽号逐项与二进制比对；`DigitalActionState` 四个谓词；12 B 记录的 `deconstruct`/`reconstruct`；serde 形态只写非默认槽）、`input_stream`（`add_frame`/`on_history_sync`/`drop_oldest_sync_point`，帧与同步点环）、`ActiveInputModeTracker`（250 ms 冷却、截止时间比较）、`InputRecordReplayMode`（5 个变体与 `Display`）、`PanicReproState::is_inactive`。纠错：第 40 个 digital 槽名是 `cinematic_camera_ctrls`；`add_frame` 硬上限是 49999；`drop_oldest_sync_point` 在无同步点时弹空所有帧；`DrawState`/模式名以 `Display` 为准。见 [ALGORITHMS](../../../reverse-engineering/tiny-glade/evidence/country-core-deep/input/ALGORITHMS.md) | `MainActionSet::just_pressed` 的 46 槽映射、`InputFrame`/`InputStream` 的 serde 形态与 `try_load`、`translate_input` 等 backend 系统 |
| `src/geometry/` | `cube`（24 个顶点的位置/法线/UV 与 36 个索引由汇编符号模拟还原，标签 `"Box"`，输入布局 `[x0,x1,y0,y1,z0,z1]`）、`sphere`（复用 `hexasphere 12.0.0`；上限 79 细分、顶点数 `(n+1)²·10+2`、UV 闭包 0x141306330 = `[atan2(z,x)/(-2π)+0.5, acos(y)/π]`、只有位置/法线/UV 三个属性、标签 `"Sphere"`；`acos`/`atan2` 与 Windows CRT 可能差 1 ulp，未验证）、`plane`（标签 `"Plane"`，输入只有一个 f32，索引 `[0,2,1,0,3,2]`）、`instanced_wall`（`BrickSourceId` 的 `{0,1,2,3,11}` 掩码重映射与 `with_transient_stair_edge_id`，`InstancedWall` 的 `new`/`update`/`upload_to_storage_buffer`，写入器缺失与 `Out of bricks.` 两处 panic）。纠错：文档写成 `{位置,UV,颜色}` 的 0x18 B 顶点，实际是 32 B（位置+法线+UV）、8 角实际是 24 顶点 | 无 |
| `src/components/registry.rs`、`drawable.rs` | 57 个组件的名字、注册函数地址、克隆策略（11 个 `Custom`、46 个 `Default`）；**57 个组件都没有 required 组件**（大小只有 0x6f0/0x700 两种，且无一个调用别的组件的注册函数）；`NaniInstanceMeshDesc::from`（part 索引原样保留） | 各组件的字段（大多未恢复）、`cache_prev_object_transforms`（0x14082bd80）、`DrawableMeshBundle::build` 的共享尾调目标 |
| `src/utils/audio_utils.rs` 及 utils 审核修正 | `AudioLoop`（`current`/`start_if_not_playing`/`stop`，u32 回绕的毫秒计时）。独立审核后修正：`load_json_as_mesh` 补 `source_file`、空 buffer 断言、`Float2/3/4` 取前 N 个分量、属性名先整体校验、`read_attribute_from_json` 返回 `Result`；`debug_viz` 颜色改为 6 变体枚举；线光栅端点排序对 NaN 不交换；`geo` 补 `x0y`/`x_h_y` | `AudioCore::instantiate_loop`；`_hypotf` 与 glibc `hypotf` 的 1 ulp 差异未验证 |
| `src/startup/{load_textures,startup_session,init_photomode_state,load_glade_audio}.rs` | 纹理白名单谓词（`Path::starts_with`，demo 只换末项）、起始存档模板目录（demo/full）与 `copy_dir_all`（跳过 `intro`）、`on_enable_photomode`（字节偏移 `+8/+0xa/+0xc`，文档写的 `+10/+11` 有误）、glade 音频常量 | 所有真正加载资产、生成实体的启动系统（依赖 ECS 与资产库）、`load_game_session` 本体、28 个启动系统的注册 |
| `src/recovered_building/history_snapshots.rs` | `History` 快照表：无分支二分 + "取更近者"（平局取后者）、`current_snapshot`/`set_current_snapshot`（已存在则 panic）/`closest_resolved_snapshot`、`insert_starting_edit`（`cursor += 1`、链首插入）、两个 diff 空判定（读容量）、`is_default_stair_width` | `add_decorator_diff` 的 checksum 校验与边循环、`history_process` 的命令生成与去重校验、`ReplayBuffer`、各领域 `replay_*` 系统 |
| `src/resources/time_of_day.rs` | `remap_hour_to_fmod_hour`（0x140a74790）：夏季 6 个关键帧、冬季 x/y 各 8 个关键帧（此前文档写 x=7、y=9 有误） | `time_of_day` 系统本体（0x140a73460）、`TodSequence::snapshot_at_tod` |
| `src/systems/weather_system.rs` | `weather_system`（0x1408e5b00）：冬季降雪强度随 10 分钟正弦涨落并钳到 `[0, 1]` | `SnowIntensity` 前 0xc 字节 |
| `src/systems/`（原 recovered_building 已迁入） | 早期原指令/状态证据约束的建筑、墙、门窗、历史和生命周期 CPU 实现 | 详见子目录 README；这些是已解析输入的 CPU 投影，与渲染侧类型不是已证明相同的原 ABI |
| `src/systems/`（原 recovered_systems 已迁入） | 树删除条件、水栅格闭包、动物 locomotion predicate、墙音频 kind 表/音乐边界、clutter 删除 schema 的已闭合 CPU 子函数 | 各系统完整状态机、真实查询/资源、音频和 replay 消费；详见子目录来源与范围 |
| `src/utils/` | 稳定 ID 压缩和世界栅格采样已委托共享 `utils`；本地仅保留消费接口适配及自有 Transform | [索引/采样原码验收](../../../reverse-engineering/tiny-glade/evidence/utils-gap-recovery-2026-10-14/REPORT.md)；Transform 未恢复的 `look_at`/`abs_diff_eq` 未纳入 |

`recovered_building/nani_solid_vertex_color.rs` 是早期已解析输入的 CPU 数据编码规则；`render/nani/` 管理实际渲染声明和实例集合。二者角色不同，并存不表示原内存布局、生产者链或 ECS 类型身份已经统一。

## 逆向来源

- Git `664f6a65`：`feat(country-core): 全量恢复 21 工作包并整合为可编译 crate`。其大部分是路径/类型骨架，不能整体当成已恢复功能。
- Git `b5965593`：`feat(country-core): 渲染模块 A1–A4/N/D/C/S/H 按原机器码恢复，附统一测试台与审计工具`。此次渲染实现整理的主要依据。
- 实现来源：[reconstruction/country-core/src/](../../../reverse-engineering/tiny-glade/reconstruction/country-core/src)。原模块树与每个函数的 B 地址、O/I/U 注释保留在源码中。
- 原始取证及合同：[reconstruction/country-core-render/](../../../reverse-engineering/tiny-glade/reconstruction/country-core-render) 下 `wp*/contracts`、原机器码/反编译材料。
- 独立审计：[AUDIT_REPORT.md](../../../reverse-engineering/tiny-glade/evidence/country-core-audit/AUDIT_REPORT.md)、[L0_NOTES.md](../../../reverse-engineering/tiny-glade/evidence/country-core-audit/L0_NOTES.md)、[B_SUMMARY.md](../../../reverse-engineering/tiny-glade/evidence/country-core-audit/B_SUMMARY.md) 和同目录 A3/B 分域报告。
- 旧测试挂载清单：[l0-harness](../../../reverse-engineering/tiny-glade/tools/country-core-audit/l0-harness)。这里只用它核对遗漏的真实源码和已有测试；没有引入 `parking_lot_shim`、`slotmap_shim`、`half_shim`。
- 逐文件映射、原输入哈希和工程适配记录见 [render-source-map.json](render-source-map.json)。建筑来源单列于 `src/recovered_building/README.md`。
- 对旧 `country-core/src` 的 595 份源码另作覆盖清单，逐文件裁决了 41 份未映射且无显式 Gap/TODO 的候选，见 [country-source-coverage.json](country-source-coverage.json)。该扫描仍区分空体、写死示例、仅数据/容器转发、停止候选和实际已恢复函数；含显式缺口文件并未因此被整体宣布无可用函数。
- 相机叶来源：`reconstruction/camera/schema.rs`（Git `8107b475`）；原码证据 `evidence/parallel-camera/B-decompile/140911e30.c`。

## 纳入与排除边界

函数已经恢复，而其 GPU 消费方尚未接通，不是删除该函数的理由。因此 pass 声明/CPU 算法正常纳入，后端用 `GraphicsDevice`、`ShaderBackend`、`TextureDevice`、`RgResources`、`PassHost`、`FramePasses` 等明确接口提供。

`rhapsody_contract` / `rhapsody_types` 描述当前游戏侧 API 边界，不能当成 rhapsody 库实现。生产路径中的未连接操作明确报错或 panic；不会返回虚构管线、零页映射或成功空上传。CPU 记录型图/设备只在 `cfg(test)` 中存在。动态 staging 和 buffer import/export 需要调用方提供后端。真实 rhapsody 已恢复实现在相邻包，完整桥仍待开发。

`IvyDrawView` 和 `LeafParticleCountView` 仅承载已观察的 draw 输入；没有拷贝旧 IvyStorage 的猜测增长器或 GladeSettings 的空加载方法。后者保留原 callback 的 `u64` 输入，避免旧 `u32` 字段适配截断。`FrozenMap` 暂为稳定引用容器的工程适配，不计作原 frozen 库恢复；其上游身份和完整语义仍待核实。

未纳入 grading LUT 中仍是 stub 的部分（`.cube` 加载已纳入，见上表）、旧 loading/logo 启动骨架、全游戏 nani ECS 系统、未确认的 bucket/marker/世界栅格类型及整树 `Gap` 资源。原二进制本身的 `TODO(vulkan)` panic 和不支持格式的 panic 是已观察行为，保留并在对应函数注释中说明。

追加覆盖核查发现 Gothic 门曲线旧转录存在 4 个错误：`-0.148 → -1.48`、`1.58 → 1.57`、`-1.58 → -1.57`、`-0.6 → -1.65`。正式函数已按 `wp21/evidence/g8-misc-2.result.json` 的原 f32/qword 常量纠正，六组完整点表使用独立原常量 fixture 逐位验证。旧 `delete_wall` 的两张 Entity map 所有权存在合同/保存反编译冲突，未猜测双 despawn 算法；原因登记于覆盖清单。

公开依赖直接使用原 B 版本，包括 Bevy ECS 0.16.0、glam 0.29.3、half 2.4.1、bytemuck 1.16.0、slotmap 1.0.6、nohash-hasher 0.2.0、relative-path 1.9.0 和原 hotwatch Git commit。没有迁入旧手写 NoHashHasher / RelativePathBuf 代替实现；生产 watcher 直接收到原 notify 事件，测试注入器只在 `cfg(test)` 中使用。Bevy 分叉仓库/commit 仍未知，使用这些 API 不等于已恢复引擎分叉。

## 验证与开发

2026-10-14 core 墙首批：新增 7 项原指令 fixture/World 链测试通过，现有库内 1349 项通过，整体 check/fmt 通过。原指令验收仅含字段写入、参数化构造、默认拱高比较和计数器 reset。Rust 1.98.1 严格 Clippy 遇到既有 tg-history `map_or` 与 11 处 `chunks_exact` 样式建议；core 使用 `--no-deps` 并仅禁用旧 `chunks_exact_to_as_chunks` 建议后检查通过，无生产源码 lint 放宽。上层 runtime 未改。

在 workspace 根执行：

```powershell
cargo check -p country-core --locked --offline
cargo test -p country-core --locked --offline
cargo clippy -p country-core --all-targets --locked --offline -- -D warnings
```

本轮已以真实依赖编译；启用 `native-physics` 的 1142 项库内测试全部通过，默认及 `native-physics` 的全部目标严格 Clippy 通过。已有机器码/O/I/U 和隔离测试证据的范围仍然各自有效；测试通过不能替代原二进制整链、GPU 输出或完整游戏验收。新增/迁移建筑测试的当前数目以 workspace 验证结果为准。

恢复源码保留机器码的 NaN 比较、max/min、运算顺序和调用合同。准确的文件级 lint 允许项登记于 [lint-preservation.json](lint-preservation.json)：numeric 项保留 `max/min` 的 NaN 行为、比较方向、浮点常量和操作数顺序；API 项保留原调用形状及缓冲范围；style/fixture 项保留取证文档和测试表达。没有允许整个 Clippy，也没有通过更改数值算法来消除报告。

默认与全部目标的严格 Clippy 已通过。`native-physics` 是旧建筑 CPU 实现到原 Rapier 依赖的可选适配，不是物理引擎重写；对应状态见建筑子目录和 workspace 验证结果。

| `src/asset_libraries/mesh_library.rs` + `utils/load_json.rs` | `AssetMeshLibrary::reload_mesh_src`（0x14092d8d0）接上 `try_load_json_as_mesh`（Result 单态 0x140b0eeb0：读文件/解析失败 panic，属性读取错误返回 `Err` 并被记录为 `couldnt reload a mesh`）。其余 asset_libraries 未引用地址均为 SSBO 泛型单态（`add_new`/`get_rg_buffer`/`as_any` 等），由泛型实现覆盖 | 无 |
