# RESTORE — startup/系统注册（卡G，组 state，order 8 收口）

> restorer-state，2026-10-16。全局收口：startup 缺文件批量 + lib.rs 顺序核对 + run_game 入口接线。

## 本批新增实现

| 项 | Rust 位置 | 证据 |
|---|---|---|
| `load_shader_into_library` | `startup/mod.rs`（clone name + `RasterShaderProgram::new_draw`(compile_shader_variants) + `AssetShaderLibrary::add_raster`） | 0x140b3fe40<&str>/0x140b40210<String>/0x140b405e0（replace） |
| `add_game_startup_systems<B>` | `startup/mod.rs`（GameStartup 注册合同：委托 `register_core_startup`；原 28-系统 tuple 目标登记） | 0x140941110（O） |
| `register_core_startup<B>` | 新 `startup/run_game.rs`（宿主可调 core 链：会话流 build + 存档事件 + MetaCommander/会话切换 + 历史事件 + 音频 init + 历史回放链 + LoadGameManager） | run_game_thread 0x1405a3440 结构 + 卡2/C 衔接 |
| lib.rs 系统注册顺序核对 | `lib.rs` systems 模块声明齐全（audio/backend/camera_rig/collision/ivy/mode_manager/paths/render_loop/roof/weather/world_raster/history/garden/ui/ui_clutter/terrain/time_of_day/save/wall）；执行顺序目标登记于 `add_game_startup_systems` | README 依赖图 order 8 |

## 与卡C/卡2 衔接

- 会话切换链：`register_core_startup` 插入 MetaCommander + `configure_session_switch_systems`（commander_update_system）+ LoadGameManager（load_game 系统消费）。
- 历史：`register_history_events` + `add_history_cpu_systems`（回放链）。
- 存档：`add_save_load_events_and_resources`（事件/资源）。
- 音频：`music_audio_init_system::<B>`（B=宿主 MusicBackend；feature native-audio 提供 FmodMusicBackend）。

## 仍缺/阻塞（宿主/域）

- 原 28-系统 tuple 中域系统（墙/屋顶/地形/水/常春藤/杂物 spawn）由各域恢复后经
  `add_game_startup_systems` 并入；不注册空系统。
- `load_mesh_into_library` 0x140b1aca0（MeshWatch + AssetMeshLibrary::add）依赖资产库
  完整接线，登记待接。
- run_game_thread 3 命令载荷（rbx+0xb00）仍待定点（卡2 残留）。
- startup 缺文件批量（27）：多数为域系统注册胶水，随各域闭合；`clearing/debug/
  panic_reporter_startup/init_color_library` 等纯注册可随后补。

## 卡R8（startup 缺文件批量 27）追加

**已实现核对**：init_photomode_state/load_glade_audio/load_textures/startup_garden_brush/
startup_ivy/startup_session/startup_terrain + mod.rs 已存在；卡G `register_core_startup`/
`add_game_startup_systems` 已建 core 链接线。

**缺 27 文件分类（inventory + 章节）**：

| 类别 | 文件 | 依赖状态 |
|---|---|---|
| 域注册胶水（0 函数/组件注册，依赖域类型） | startup_roof / startup_sheep / startup_time_of_day / startup_walls / startup_water / glade_themes/flowery / glade_themes/olden | 随各域（墙/屋顶/羊/时间/水/主题）恢复后并入；组件类型在域侧 |
| 资产库绑定（mesh/shader/prefab 载入，依赖 AssetMesh/Shader/Prefab 库） | load_audio / load_glade_grading / load_glade_tod_and_weather / load_mesh_atlases / load_prefabs / load_shaders / startup_blit / clearing / debug / spawn_brush_previews / spawn_human_ref / spawn_preplaced_trees / startup_critters / startup_deferred_raster / startup_grass / startup_meadow / startup_nani / startup_sky_and_sun | 资产库/render 管线在途（render/asset 组）；`load_shader_into_library`/`load_mesh_into_library` 入口已备（卡G） |
| 反馈/panic（std hook） | panic_reporter_startup（行 22 I，无机器码证据） | feedback_report 域待接 |
| 调色板 | init_color_library（assets/colors.json → color library 资源，0x14082c000 O） | color library 资源在 render 侧，待接 |

**诚实预算判定**：27 文件多为域注册胶水 + 资产库绑定，依赖墙/屋顶/羊/水/时间/主题/render/
asset 各域在途类型；本批登记分类与依赖状态，未伪造实现。`load_shader_into_library` 入口
（卡G）供 load_shaders/startup_blit 复用。

**衔接**：`register_core_startup`（卡G）预留域系统并入缝（`add_game_startup_systems` 由
各域在域类型恢复后填充）。

## 编译/测试状态

- `cargo check -p country-core --offline`：零错误；我的文件零警告。
- `cargo test --lib`：**1626/1**（唯一失败=pre-existing geometry/sphere 浮点精度，
  未改文件）。基线较 1624/3（wall 组 decorator 在途）已恢复。

## 合同/状态建议

- inventory：startup 模块 → `partial_implemented`（入口合同 + core 链接线；域系统注册
  待各域）。
- DISPATCH：卡G 交 reviewer 终验。
