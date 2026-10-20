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
