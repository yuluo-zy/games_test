# 原版注册函数清单与宿主接线核对（2026-10-20，第二版）

本版取代上一版：上一版把 `draw_water` 判为"不属于 WaterPlugin"、并在宿主新增了 `water_plugin.rs` / `glade_border_plugin.rs`，两点都被机器码证据推翻，已更正。

## 方法

Ghidra 库（`reverse-engineering/tiny-glade/scripts/reverse_tools/linux/`，PE `.pdata` 全量建函数 + PDB 命名）加 PyGhidra 脚本：

1. **按地址顺序读调用**：对每个注册函数读出 `add_systems / in_set / run_if / chain* / register_event / insert_resource`，避开反编译对移动值的顺序歧义。
2. **类型名取自单体里的字符串**：`register_event<T>`、`insert_resource<T>` 的单体里，开头那段 rodata 字符串就是 `T` 的类型名；`Box<dyn System>` 的 vtable 旁边是系统函数的类型名。据此可以读出"每个注册点注册了哪个类型/系统"。
3. **类型 → 创建点**：扫描全部 `insert_resource` 单体，建立"类型名 → 单体 → 调用点"表，直接回答"谁创建了资源 T"。

## 原版的注册结构（源码目录归属）

原版不是"一个大 `run_game` 里手写所有注册"，而是按 crate/模块拆成 `App` 扩展 trait 和 `Plugin`：

| 位置 | 函数 |
|---|---|
| `tiny_glade::app_systems` | `WallSystems::{add_wall_systems 0x140493120, add_wall_events 0x1404936b0}` |
| `country_core::resources` | `RenderingResources::insert_rendering_resources 0x140940190`、`CustomResources::insert_resources 0x140940d80` |
| `country_core::startup` | `StartupSystems::add_game_startup_systems 0x140941110` |
| `country_core::systems::*` | `AudioSystems`、`BackendSystems`、`HistorySystems`、`IvySystems`、`SaveLoadSystems`、`SteamIntegration`、`TerrainSystems`、`UiSystems`、`StagesUtil::add_stages`、`WallShapeUxSystems` |
| `system_roof` / `timberframe` / `system_onboarding` / `system_color` | `RoofSystems`、`TimberframeSystems::add_timberframe`、`OnboardingSystems::add_onboarding`、`ColorSystems::add_color 0x140651870` |
| `system_water` / `system_glade_border` / `system_tree` / `forest` / `system_clutter` / `system_critters` … | `<X as Plugin>::build`（共 46 个，清单见上一版） |

`GameSystemSet` 是 `country_core::systems::GameSystemSet`（符号表有 `impl SystemSet for country_core::systems::GameSystemSet`）。宿主原先在 `run_game.rs` 另定义了一个同名 u8 类型，类型身份不同，系统 crate 因此没法按原版直接写 `in_set(GameSystemSet(..))`。本次已把它放回 country-core，宿主改为 `pub use` 重导出。

## 本次落地

| 原函数 | 现在的位置 | 机器码证据 |
|---|---|---|
| `WaterPlugin::build` 0x1406501d0 | `crates/systems/water/src/lib.rs` | `add_systems(Update, (draw_water::<C>.in_set(0x10), record_water_history::<S>.in_set(0x22)))`；`update_water_audio_grid.run_if(Option<Res<WaterAudioGrid>>).in_set(0x24)` 独立注册；`insert_resource<WaterRaster>` 无条件；3 个事件。**`draw_water` 在第一条 tuple 里**（旧版结论错误）。 |
| `GladeBorderPlugin::build` 0x140650ab0 | `crates/systems/glade-border/src/lib.rs` | `(display_glade_border, feedback_input_out_of_border).chain_ignore_deferred().in_set(0x24)`；2 个事件。 |
| `ColorSystems::add_color` 0x140651870 | `crates/systems/color/src/lib.rs` | 15 个事件；`init_color_ids_buffer` 另行注册（调度标签未识别，按 Startup）；Update 里 `(replay_color.in_set(0x28), 18 个系统.chain_ignore_deferred().in_set(0x1f))`，链内顺序取自 `chain_ignore_deferred` 单体 0x140ed1780。原版有 `set_wall_style`、没有 `color_cursor_icon`。 |
| `init_color_ids_buffer` 0x142290750 | `crates/systems/color/src/set_wall_color_id.rs` | 四个颜色 SSBO（Wall/Roof/Gate/Halftimber）的 `add_new` + CPU 表插入。旧版只插 CPU 表，漏了 GPU 缓冲。 |
| `WallSystems::add_wall_events`（部分）| `crates/tiny-glade/src/app_systems.rs` | `insert_resource(EraserSizeMultiplier(1.0))`（调用点 0x1404940c5，值 0x142925904 = 1.0）；已定位类型的事件/资源（含 `OutlineWallCmd`）。**其余约 20 个事件/资源在本仓库的定义位置尚未对上，未注册**（清单见函数注释）。 |
| `MeiPlugin::build` 里的加载系统 | `crates/tiny-glade/src/mei_host.rs` | `load_configs("assets/ui")` → `load_font_table("assets/fonts/table.ron")`；路径是原函数里的常量。 |
| `insert_rendering_resources` / `insert_resources` 里宿主漏掉的 | `crates/tiny-glade/src/run_game.rs`（宿主已有的手工镜像段） | `TerrainEditingData`（0x1409403c3）、`ActiveRasterStrokesStorage`（0x140940882）、`GladeColorCustomization`（0x140940f7e）、`BrushSizes`（`PathPlugin::build` 0x140b691c0，初值字节 0.8/2.0/3.0 与 core 常量一致）。 |
| `GladeBorder` 创建系统 0x140af2200 | `crates/country-core/src/resources/glade_border.rs` | 匿名系统：非 demo → `Rectangle2d(130×130, 居中, 轴对齐)`；demo → `Circle2d(r=15)`；经 `InsertResource` 命令插入。常量已按字节核对（0x142a7b658=130.0、0x1429cd06c=15.0）。 |
| 纹理全量加载 | `crates/tiny-glade/src/loading_graph.rs` | 原版有 `TextureLoadingFilter` trait，`LoadOnlyBootupTextures`（0x1409e6950）只用于 loading App。gameplay App 此前也套了同一白名单，388 张纹理里只有开机那几张可用，UI 按名查找 `menu/settings_icon` 直接 panic。现在 gameplay 载入全部，并删掉了宿主为 `tony_mc_mapface` / `menu/active_lut` 单独硬编码的补丁。 |

## 未闭合（逐项）

- **`GladeBorder` 创建系统的注册点**：系统体已确认，但它是匿名系统，vtable 没有可查的引用，原版在哪个调度注册没有定位。现按 Startup 注册。
- **`init_color_ids_buffer`、MeiPlugin 加载系统的调度标签**：`Schedules::add_systems` 的标签烤进单体实例，未识别。PreStartup 与 Startup 对首帧 Update 等价，但不是读出来的。
- **`TerrainEditingData` 构造里 +0x38 的 qword 1**：重建的结构里没有对应字段，`Default` 复现了其余部分，这一项未解释。
- **`add_wall_events` 约 20 个事件/资源**、**`add_wall_systems` 整体**（仍是 `todo!`）。
- **gameplay 纹理加载的过滤器**：`should_load` 常量真值内联后没有独立符号，"全量加载"是由 trait 名称和 UI 按名查找推出的，不是从调用点读出的。纹理目前同步加载，原版是异步任务（性能项）。
- **设备预算**：宿主和测试用的 `DeviceConfig` 默认 1 GiB 设备预算，装不下全量纹理加上 Nani/地形缓冲（`OutOfDeviceMemory`）。生产路径同样用默认值，真机上需要核对原版的预算策略（`vram.rs`）。
- `Raycasting`、`WaterAudioGrid` 的生产创建者；`ShowDemoBorderOverlay` 事件注册。
- 其余约 40 个 `Plugin::build` 与十几个 `App` 扩展 trait 尚未逐个对照。

## 验证

- 全 workspace `cargo check --workspace --all-targets --features country-core/native-audio` 通过。
- 真实 gameplay App（含真实 intro 存档、真实资产、lavapipe 设备）现在能跑过 `Startup`/`PreStartup` 并进入存档加载；**单个系统参数校验失败的清单已清空**，下一个暴露的是存档回放阶段的 `adjust_wall_height_impl: wall WallId(29) missing from InnerWalls`（未解决）。
- 本机验证方式：Linux 缺 `libfmod`/`libfmodstudio`，用仓库外的链接垫片（`~/tools/fmod-link-stub`，所有 FMOD 调用返回 `FMOD_ERR_INTERNAL`）满足链接，原版 `AudioCore::None` 分支生效。**音频链路未经验证。**
- 为让 Linux 走同一份代码，宿主新增了 `native-audio` feature，把原先写死 Windows 的音频接线（`cfg(windows)`）改为 `cfg(any(windows, feature = "native-audio"))`；Windows 行为不变。

## 附录 A：`WallSystems::add_wall_systems`（0x140493120）完整注册表（机器码取证，2026-10-20）

方法：调度标签取自 add_systems 实例内 `Schedules::entry` 的 label vtable Debug::fmt 字符串（0x1429f1e78="Update"、0x1429f1eb0="Startup"、0x1429b7ca0=`OnExit<S>` 且 DL=1 即 `AppLoadingState::Game`）；set 号取 in_set 前立即数（`chain_ignore_deferred` 之后的 in_set 经 `Interner::intern(vtable 0x14292af98 = GameSystemSet)`）；系统名取 tuple 实例内 `LEA` 的类型名串，字符串池里粘连，按 crate 前缀切分。chain1 的 Vec 槽位顺序已与读写来源核对，其余 chain 按 `into_configs` 调用顺序，未逐个复查 Vec 写入。

| # | 调用点 | 标签 | 内容 | set |
|---|---|---|---|---|
| 1 | 0x14049315b | Update | `ui_arch_controls` | 17 |
| 2 | 0x140493177 | Update | `restore_archived_decorators`（泛型 C 未闭合） | 42 |
| 3 | 0x1404932c7 | Update | 外层 `chain_ignore_deferred(chain1, chain2)` | 19 |
| — | 0x1404932eb | — | `insert_resource(OverhangSupportDirtySubscriptions)`，值三个 qword (0,8,0)=`Vec::new()` | — |
| 4 | 0x1404932f6 | **Startup** | `overhang_subscribe_to_tabloids` | 无 |
| 5 | 0x140493385 | Update | chain：detect_overhang_support_segments, construct_circle_supports, construct_rectangle_columns, maintain_stone_column_colliders, flush_rapier_colliders | 24 |
| 6 | 0x140493414 | Update | chain：update_gates_hashmap, construct_gates, construct_fences, construct_platform_stairs, place_pillars, construct_walls, construct_floors, debug_viz_threshold_curve, structure_outline | 28 |
| 7 | 0x140493430 | Update | `construct_rectangle_brackets` | 36 |
| 8 | 0x1404934bf | Update | chain：history_record_adjust_wall_height, history_record_wall_changed, history_record_merge_walls, history_record_auto_clutter | 34 |
| 9 | 0x1404934db | Update | **普通 tuple（非 chain）**：gpu_flush_brick_buffers, gpu_flush_plank_buffers | 44 |
| 10 | 0x14049356a | Update | chain：create_plaster, maintain_plaster, curved_halftimber_maintain_entities | 32 |
| 11 | 0x140493574 | **OnExit(Game)** | `walls_on_glade_unload` | 无 |

set 19 chain1（18 成员，0x14050d290）：wall_splitter, wall_eraser_bulldoze（唯一 run_if：`wall_eraser_bulldoze_condition`）, wall_eraser, animate_wall_bulldoze, ui_draw_wall, suppress_continue_wall, ui_continue_wall, snapping_init, snapping_to_ends, snapping_to_segments, check_for_disable_snapping, snapping_audio, snapping_signifiers, snapping_maintain, draw_wall, finish_wall, merge_walls, telegraph_walls_are_merged。
set 19 chain2（16 成员，0x140335b30）：ui_adjust_wall_height_signifier_animation, ui_adjust_wall_height_mode, adjust_wall_height_impl, do_ui_arch_controls, ui_lintel_controls, wall_highlight, publish_public_wall_state, delete_wall, spatial_hash_update, spatial_hash_debug, calc_wall_bend, debug_force_update_arches, gpu_clear_wall_gate_pieces, sample_wall_path_intersections, calculate_path_wall_segmentation（原版函数名；本仓库叫 `calculate_wall_path_segmentation`）, init_wall_height_and_arch_segment_height。

### 本仓库缺失的原版系统（未恢复，不得用空函数冒充）
`wall_splitter`、`merge_walls`、`ui_adjust_wall_height_signifier_animation`、`ui_adjust_wall_height_mode`、`spatial_hash_debug`、`debug_viz_threshold_curve`、`history_record_adjust_wall_height`、`history_record_merge_walls`、`gpu_flush_brick_buffers`（原路径 `upload_instanced_wall_ssbo`）、`structure_outline` 系统函数。
本仓库是空壳或 `todo!`：`gpu_clear_wall_gate_pieces`、`debug_force_update_arches`、`gpu_flush_plank_buffers`、`overhang_subscribe_to_tabloids`、`construct_circle_supports`、`construct_rectangle_columns`、`construct_rectangle_brackets`、`detect_overhang_support_segments`。
等价性未闭合：`adjust_wall_height_impl` vs 本仓库 `adjust_wall_height`；`construct_walls` vs `construct_wall_bricks` 别名；`flush_rapier_colliders` 只有 `RaycastWorld` 方法，无系统包装；`OverhangSupportDirtySubscriptions` 本仓库字段是 HashMap，原版值为 `Vec::new()`。

### 宿主现状差异
宿主 `run_game.rs` 手写 set19 的 12 成员（含宿主适配 `synchronize_wall_publication_terrain`、`collect_wall_publication_dirty`、`prepare_wall_brick_assembly_owners`、`delete_constructor_output`、`clear_constructor_planks`——原版 chain 中没有）与 set28 的 2 成员（含宿主适配 `finalize_wall_holes`）；`startup_construction_runtime.rs` 另挂 set28 的 5 成员。原版 set 17/42/24/34/32/44/36 与 Startup 的组在宿主完全没有。

## 附录 B：设备预算（原版取证，2026-10-20）
原版没有可配置的设备内存预算：`GpuAllocator::new`（0x140c0d980）参数里无预算，`allocate`（0x140c0da40）无"总量>上限即 OOM"的分支；`PhysicalDevice::{device_local,host}_memory_budget_bytes`（0x140c006a0/0x140c00890）只被 `vram::calculate_max_resolution_scale`（0x1404f7f20）与设置界面显示文本使用，预算只用来限制渲染分辨率缩放上限（`run_game` 0x1405967c6 存 `[state+0x184]`）。本仓库 `production_renderer.rs:570-577` 用 `DeviceConfig::default()`（1 GiB/512 MiB，rhapsody 自有策略）是偏差。`vram::calculate_max_resolution_scale` 在本仓库无生产调用点。未闭合：原版纹理加载的显存策略、`allocation_sizes`、`[device+0x1a]` 来源。

## 附录 C：墙分段链注册复核与未接系统表（2026-10-21）

结论先行：任务前提"主循环只挂了 split/arches/lintels/flush"已过期。`WallSystems::add_wall_systems`（app_systems.rs，提交 b602f922b）已经被 `run_game.rs:965` 调用，附录 A 的 set 19 chain2 里的 `sample_wall_path_intersections`、`calculate_wall_path_segmentation`、`init_wall_height_and_arch_segment_height` 已在主循环里，顺序与原版一致。`wall_constructor_plugin::register_cpu_systems`（及 `lib.rs::add_wall_constructor_systems`）确实无调用者，且它的顺序（init 在前）与原版相反，**不是**原版注册结构，不应接线。本次没有改变任何注册，只补了调度顺序测试。

### 原版注册证据（B，Ghidra 12.1.3）

方法：`add_wall_systems` 0x140493120 里 0x14049318b/0x140493197 依次调用 chain1 `chain_ignore_deferred`（0x14050d290）与 chain2（0x140335b30）；0x140493246 `mov byte [rbp-0x60],0x13`（set 19）；外层 0x1404932c7 `add_systems`。chain2 的 16 个成员在 0x140335cb7..0x140335d68 依次调用各自的 `into_configs` 实例，每个实例内 `LEA` 的类型名串即系统名（下表地址为该 `into_configs` 实例）。

| 系统（原名） | 插件/注册函数 | Schedule | set | 链内位置 | run_if | 地址证据 |
|---|---|---|---|---|---|---|
| gpu_clear_wall_gate_pieces | add_wall_systems | Update | 19 | chain2 第 13 | 无 | into_configs 0x140368e90（调用点 0x140335d47） |
| sample_wall_path_intersections | 同上 | Update | 19 | chain2 第 14 | 无 | 0x1404e6e90（调用点 0x140335d53） |
| calculate_path_wall_segmentation（本仓库名 calculate_wall_path_segmentation） | 同上 | Update | 19 | chain2 第 15 | 无 | 0x140174db0（调用点 0x140335d5f） |
| init_wall_height_and_arch_segment_height | 同上 | Update | 19 | chain2 第 16（末位） | 无 | 0x140538010（调用点 0x140335d68） |

- chain2 全序（调用点顺序）：signifier_animation(0x140335cb7 `Box<dyn System>` 实例) → ui_adjust_wall_height_mode(cc3) → adjust_wall_height_impl(ccf) → do_ui_arch_controls(cdb) → ui_lintel_controls(ce7) → wall_highlight(cf3) → publish_public_wall_state(cff) → delete_wall(d0b) → spatial_hash_update(d17) → spatial_hash_debug(d23) → calc_wall_bend(d2f) → debug_force_update_arches(d3b) → gpu_clear_wall_gate_pieces(d47) → sample(d53) → calculate(d5f) → init(d68)。外层 `(chain1, chain2).chain_ignore_deferred().in_set(19)`，故 chain1 全部在 chain2 之前。
- 三者的 into_configs 实例各只有一处调用者（0x140335b30 本身），**未发现 Startup 或其它 schedule 的第二处注册**（含存档读取后的 startup 路径）；这是按 into_configs 调用者查出的，未对全程序 LEA 类型名串做穷举扫描。
- 三者本身无 `run_if`；整条链唯一的 run_if 是 chain1 的 `wall_eraser_bulldoze_condition`。注册点处也没有 `AppLoadingState::Game` 或 history/replay 条件（这类条件若存在，在 set 层面的 `configure_sets`/`run_game_thread`，不在此注册点）。
- 未复核：chain 的 Vec 槽位写入（顺序取自 into_configs 调用顺序，附录 A 对 chain1 复核过，对 chain2 仍只按调用顺序）。
- 相邻的 set 22 组（`split_wall_path_segment_into_platform_stairs`、`construct_arches`、`construct_lintels`，在 startup_construction_runtime.rs）沿用既有注释里的证据（set 22 位于 roof 关系/invalid-roof 更新之后），本次未重新取证。

### 依赖关系

`split_wall_path_segment_into_platform_stairs`（set 22）对每个 `OnWallChanged` 墙从 `WallPathSegmentationMasks` 里 `expect` 取原始掩码，该资源只有 `calculate_wall_path_segmentation`（set 19）写入，所以后者必须在前面，已满足。

### 接线与测试

- 无新增注册。新增测试 `run_game::startup_construction_runtime::segmentation_chain_tests::segmentation_chain_is_scheduled_in_original_order`：按宿主的 0..44 set 链、`add_wall_systems` 与 `startup_construction_runtime::install` 构建 Update 调度并初始化，读出拓扑序，断言每个系统恰好注册一次，且 `adjust_wall_height_impl < gpu_clear_wall_gate_pieces < sample < calculate < init < split < construct_arches < construct_lintels`。
- 这只证明调度存在与相对顺序；"在 Game 状态下确实执行"由 `main_chain_audit`（240 帧、真实 gameplay App）证明：0 个系统参数校验失败。三者是否在该 App 里实际产出（如 `WallPathSegmentationMasks` 有内容）没有单独断言。

### `sample_wall_path_intersections` 是占位，不是"可用实现"

`crates/systems/wall-constructor/src/segmentation/sample_wall_path_intersections.rs` 的函数体只对每个事件做 `walls.get_mut` + `let _ = curve.length`，无任何输出；原函数 0x14074c720 为 39.6KB，参数面含未落地的 `PathRastersResources`。它现在已被 `add_wall_systems` 挂在原版位置上，**属于空操作占位，其业务（路径光栅与墙轮廓交点采样）未恢复，不能算闭合**。目前 calculate/init 不读它的输出，所以不影响下游，但其真实输出消费者未查。副作用：`ResMut<PublicWalls>` 被 DerefMut，会写变更 tick（经 grep，没有 `Changed<PublicWalls>`/`is_changed` 消费者）。

### 原版在此注册而本仓库未接的系统（均不接，不用空函数冒充）

| 系统 | 原版位置 | 未接原因 |
|---|---|---|
| construct_circle_supports / construct_rectangle_columns / detect_overhang_support_segments / maintain_stone_column_colliders | set 24 chain | 体内 `todo!` 或缺失 |
| construct_rectangle_brackets | set 36 | `todo!` |
| construct_platform_stairs、construct_floors | set 28 chain | `todo!` / 缺失 |
| construct_walls | set 28 chain | 已接（别名等价性未闭合，见附录 A） |
| create_plaster、maintain_plaster、curved_halftimber_maintain_entities | set 32 | `todo!` |
| place_pillars、construct_playermade_stairs 系、construct_playermade_stair_pillars_visuals | wall-constructor lib.rs 里列有，原版 add_wall_systems 的 set 28 链不含 place_pillars 之外的 playermade 系 | playermade 系的原版注册点未定位；`lib.rs` 的 `add_wall_constructor_systems` 把它们放进 Bricks 阶段并非原版结构 |
| gpu_clear_wall_gate_pieces、debug_force_update_arches、gpu_flush_plank_buffers、overhang_subscribe_to_tabloids | set 19 / 44 / Startup | 空壳或 `todo!`（以附录 A 为准；本表未逐个重核当前代码状态） |
| wall_splitter、merge_walls、ui_adjust_wall_height_signifier_animation、ui_adjust_wall_height_mode、spatial_hash_debug、history_record_adjust_wall_height、history_record_merge_walls、gpu_flush_brick_buffers | set 19 / 34 / 44 | 缺系统实现（附录 A 缺失清单；其中部分在 app_systems.rs 已有路径，以代码为准） |
| `wall_constructor_plugin::register_cpu_systems` / `add_wall_constructor_systems` 整体 | 无对应原版函数 | 非原版结构且含 `todo!` 系统，挂上会 panic；该 plugin 文件只有 `build`（5 个资源）被使用 |

注：本表中"已接/空壳"状态摘自附录 A 与本次代码阅读，仅 sample/calculate/init/split 四项做了逐行核对。
