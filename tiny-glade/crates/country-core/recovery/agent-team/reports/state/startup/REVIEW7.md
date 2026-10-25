# REVIEW7 — startup（组 state 卡G 终验：系统注册入口 + run_game 接线）

> 审查员：reviewer-state，2026-10-16。复核 restorer 卡G 落地（add_game_startup_systems /
> register_core_startup / lib.rs 顺序 / 卡2-C 衔接）。
> 编译基线实测：`cargo check -p country-core --offline` **零错误**（3 警告）；全 crate 测试
> **1626/1**，唯一失败 = `geometry::sphere::tests::uv_formula_matches_the_closure`
> （components-geometry 组，既知）；**基线已从 1624/3 恢复**（wall decorator in-flight 已收口）。

## 逐项复核

### 1. add_game_startup_systems<B>（startup/mod.rs）
| 项 | verdict | 说明 |
|---|---|---|
| GameStartup 注册合同 0x140941110 | **OK** | 函数存在，委托 `run_game::register_core_startup::<B>`；注释登记原 28-系统 tuple（含 AssetTextureLibrary::create_preset_textures + 4 子组构建器）与 audio startup。**不注册空系统**（域系统由各域恢复后并入），诚实。 |
| load_shader_into_library（0x140b3fe40） | **OK**（微差） | `RasterShaderProgram::new_draw`（0x140938240 内调 compile_shader_variants 0x1409383b0）→ `AssetShaderLibrary::add_raster(program, false)`；失败 unwrap panic 与原版一致。**微差**：name 用 `&'static str`（String 重载 0x140b40210 未建模；"clone 名字"语义不同）。 |
| load_mesh_into_library | GAP（登记） | 头注声明（O）但未实现；网格/实体生成依赖 ECS 与资产库，诚实待接线。 |

### 2. register_core_startup<B>（startup/run_game.rs）
| 项 | verdict | 说明 |
|---|---|---|
| SessionFlowPlugin::build（会话流） | OK | ActiveSession/SavedSessionList/NextSession/AutoSaveState + 会话事件。 |
| add_save_load_events_and_resources（存档） | OK | LoadGameManager/LoadingCompletionState/存档读档事件（init_resource 幂等，与 SessionFlowPlugin 共存）。 |
| MetaCommander + configure_session_switch_systems（卡2） | OK | init_resource::<MetaCommander> + 会话切换系统注册。 |
| register_history_events + add_history_cpu_systems（卡C） | OK | 历史事件集 + CPU 回放链（ReplayBuffer speed 由宿主设置）。 |
| music_audio_init_system::<B> | OK | B = 宿主 MusicBackend（feature native-audio 提供 FmodMusicBackend）。 |
| LoadGameManager 资源 | OK | init_resource 供 load_game 消费。 |

### 3. lib.rs 顺序核对
| 项 | verdict | 说明 |
|---|---|---|
| systems 模块声明齐全 | **OK** | lib.rs 声明 audio/backend/brush_preview/camera_rig/collision/ivy/mode_manager/paths/render_loop/roof/weather_system/world_raster/history/garden/ui/ui_clutter/terrain/time_of_day/save/wall + meta_commander/game_profile/startup/resources/utils。 |
| 执行顺序目标登记 | **OK** | 原 28-系统 tuple 顺序登记于 add_game_startup_systems 注释（contract）；实际挂载顺序由 register_core_startup 给出（会话→存档→命令器→历史→音频→读档资源）。未冒充原 28 序。 |

### 4. 卡2/C 衔接
| 项 | verdict | 说明 |
|---|---|---|
| load_game_session（LoadGameManager） | OK | LoadGameManager 资源已注册；load_game.rs 的 advance_load_game/load_game_session 由读档系统消费（系统级接线仍待）。 |
| commander_update（MetaCommander） | OK | MetaCommander 资源 + configure_session_switch_systems 已注册（卡2 会话切换链）。 |
| 历史注册定位 | OK | register_history_events + add_history_cpu_systems 在 register_core_startup 中调用（卡C replay 族）。 |

## 结论与 DISPATCH 建议
- **卡G → 闭合（系统注册入口 + run_game 接线）**。add_game_startup_systems/register_core_startup
  把状态组的会话/存档/历史/命令器/音频/读档资源与可挂载系统统一注册；lib.rs 模块齐全；执行顺序目标诚实登记。
- 残留登记（不阻塞）：原 28-系统 tuple 的**域系统**（墙/屋顶/地形/水/常春藤…）与精确顺序、
  load_mesh_into_library、load_shader_into_library 的 String 重载/clone 语义、run_game_thread
  本体的完整装配。
- 模块状态建议：save/session/history/startup → **review_passed（限定已实现范围）**；
  完整 core 验收仍 blocked（各域系统批量并入 28 序 + run_game_thread 本体 + 全局调度）。
