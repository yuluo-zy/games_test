# crates/tiny-glade 任务清单（应用/集成层）

## 元信息

| 项 | 值 |
|---|---|
| B 原路径 | `crates/tiny-glade` |
| package | `tiny-glade` |
| 规模 | 45 文件 / 170 本 ns 函数 / 31 闭包 / **112 业务函数** |
| 难度 | 高（集成面广；大量平台/后端胶水） |
| 批次 | 4（最后） |
| 上游依赖 | 全部 systems/* + `country-core` + `rhapsody` + `utils` + `lang-lib` + `paths` + `asset-pipe` + `tg-*` + `theme-*` + 平台后端（winit/steamworks/sentry/minidump/DLSS/FSR/egui） |
| 复现工程现状 | MODULE_PROGRESS：**未逐项评估**，无本机源码；仅登记原源码结构 |

> 说明：本 crate 大量函数是**平台/后端胶水**，逆向价值低于游戏业务。完成度应把「平台胶水」与「游戏业务」分开记。若目标是可运行游戏则必需。

## 任务表（按 B src 层级）

### 启动 / 主循环

| ID | B 源文件 | 任务内容（关键函数） | 依赖 | 现状/证据 | 验收 | 状态 |
|---|---|---|---|---|---|---|
| UM-APP-01 | `src/main.rs` | 恢复 `main`、`mock_crash_reporter`、`log_time_to_first_frame`：进程入口与启动顺序。 | 全部 | 无 | 原入口对照 + 启动顺序一致 | 未开始 |
| UM-APP-02 | `src/run_game.rs` | 恢复 `run_game`、`run_game_thread`、`make_temp_window` + 两处 Plugin build。 | bevy_app | 无 | 原入口对照 | 未开始 |
| UM-APP-03 | `src/game_loop.rs` | 恢复 `GameLoop::{new, run, main_loop, extract_available_monitors_info}`、`gather_pending_video_setting_changes`。 | winit/bevy | 无 | 原入口对照 + 主循环顺序一致 | 未开始 |
| UM-APP-04 | `src/event_loop.rs` | 恢复 `run_event_loop`、`EguiEventIo::{new, take_egui_input}`。 | winit/egui | 无 | 原入口对照 | 未开始 |
| UM-APP-05 | `src/render_loop.rs` | 恢复 `render`、`render_world`、`handle_screenshot_error`：帧渲染编排（54 pass 调用见 render-passes 证据）。 | rhapsody、country-core | 有 pass 清单分析 | 原入口对照 + 帧顺序一致 | 未开始 |
| UM-APP-06 | `src/setup.rs` | 恢复 `setup_window`、`find_best_fullscreen_video_mode`、`from_winit` 两单态。 | winit | 无 | 原指令样例 | 未开始 |
| UM-APP-07 | `src/fps_limiter.rs` | 恢复 `FpsLimiter::frame`。 | — | 无 | 原指令样例 | 未开始 |
| UM-APP-08 | `src/app_systems.rs` | 恢复 `add_wall_systems`、`add_wall_events`、`walls_on_glade_unload`。 | core | 无 | 原注册顺序对照 | 未开始 |
| UM-APP-09 | `src/decorator_plugin.rs` | 恢复 `impl Plugin::build`、`decorators_on_glade_unload`。 | decorator | 无 | 原注册顺序对照 | 未开始 |

### 窗口 / 渲染 glue

| ID | B 源文件 | 任务内容（关键函数） | 依赖 | 现状/证据 | 验收 | 状态 |
|---|---|---|---|---|---|---|
| UM-APP-10 | `src/winit_bevy_glue.rs` | 恢复 `winit_to_bevy_keycode`：输入映射。 | winit/bevy_input | 无 | 映射表与原一致 | 未开始 |
| UM-APP-11 | `src/egui_backend.rs` | 恢复 `EguiBackend::{end_frame, draw}`。 | egui/rhapsody | 无 | 原入口对照 | 未开始 |
| UM-APP-12 | `src/dlss_plugin.rs` | 恢复 `dlss_plugin` 的 `Plugin::build`（DLSS 后端）。 | DLSS SDK | 无 | 原入口对照；SDK 后端后置 | 未开始 |
| UM-APP-13 | `src/vram.rs` | 恢复 `calculate_max_resolution_scale`、`calculate_resolution_scale_mem_estimate_text`。 | GPU 信息 | 无 | 原指令样例 | 未开始 |
| UM-APP-14 | `src/aftermath.rs` | 恢复 `dumped`、`shader_debug_info`（GPU 崩溃分析回调）。 | aftermath SDK | 无 | 原入口对照 | 未开始 |

### 崩溃 / 上报

| ID | B 源文件 | 任务内容（关键函数） | 依赖 | 现状/证据 | 验收 | 状态 |
|---|---|---|---|---|---|---|
| UM-APP-15 | `src/crash_pickup/{mod,client,server,minidump_to_sentry}.rs` | 恢复 `CrashReporter::spawn_server`、`ClientHandle::send_message`、`client::start`、`server::run` + 4 回调、`minidump_to_crash_trace`、`crash_trace_to_sentry_event`。 | minidump/sentry | 无 | 原入口对照 | 未开始 |
| UM-APP-16 | `src/panic_reporter/{mod,panic_report}.rs` | 恢复 `setup_panic_hook`、`recover_game_state_after_crash`、`on_game_crashed`。 | — | 无 | 原入口对照 | 未开始 |
| UM-APP-17 | `src/panic_reporter/panic_dump.rs` | 恢复 7 个 `dump_*`（log/inputs/save/settings/profile/gpu_crash_dump/rhapsody_log）。 | 存档/设置 | 无 | 原入口对照 | 未开始 |
| UM-APP-18 | `src/panic_reporter/{panic_screenshot,sentry_reporter}.rs` | 恢复 `panic_screenshot::{create_graphics_resources, capture_image, dump_screenshot}`、`sentry_reporter::{create_sentry_attachment, report_crash_event_to_sentry, panic_info_to_message, panic_info_to_sentry_event}`。 | rhapsody/sentry | 无 | 原入口对照 | 未开始 |
| UM-APP-19 | `src/hang_watchdog.rs`、`src/integrity_report_main.rs` | 恢复 `hang_watchdog`、`spawn_hang_watchdog_thread`、`verify_package_integrity_and_show_report`。 | tg-package-integrity | 无 | 原入口对照 | 未开始 |

### 设置 / 配置

| ID | B 源文件 | 任务内容（关键函数） | 依赖 | 现状/证据 | 验收 | 状态 |
|---|---|---|---|---|---|---|
| UM-APP-20 | `src/app_config.rs` | 恢复 `get_cli_args`、`get_app_config`、`write_game_config_to_disk`、`write_settings_file`、`update_vulkan_allowlist_defaults`、`CliArgs::to_input_recording_opts`。 | clap/serde | 无 | 原入口对照 + 配置文件 schema 一致 | 未开始 |
| UM-APP-21 | `src/gui/settings.rs` | 恢复 `do_gui_escape_menu_settings`、`ui_settings_{game,audio,video,ui,developer,controls}`、`LastAppliedVideoSettings::needs_apply`、`detect_konami_code`。 | toolbar/egui | 无 | 原入口对照 | 未开始 |
| UM-APP-22 | `src/logging.rs`、`src/load_language_library.rs`、`src/icc.rs` | 恢复 `set_up_logging`、`load_language_library`、`get_monitor_icc_profile`。 | fern/lang-lib/ICC | 无 | 原入口对照 | 未开始 |

### GUI 面板

| ID | B 源文件 | 任务内容（关键函数） | 依赖 | 现状/证据 | 验收 | 状态 |
|---|---|---|---|---|---|---|
| UM-APP-23 | `src/gui/gui_escape_menu.rs`、`gui_link_to_official_game.rs`、`language_menu.rs` | 恢复 `do_gui_escape_menu`、`wishlist_button`、`open_community_server`、`open_newsletter`、`display_link_to_official_game`、`do_gui_language_menu`。 | egui | 无 | 原入口对照 | 未开始 |
| UM-APP-24 | `src/gui/gui_debug.rs`、`gui_controls.rs`、`gui_error_log.rs`、`gui_tod.rs`、`confirm_incoming_panic_repro.rs` | 恢复 `do_gui_debug`、`do_camera_snapshot_ui`、`do_freecam_save`、`date_spinner`、`key_bindings_sub_menu`、`show_control_tweakers`、`do_gui_error_log`、`do_time_of_day_screen`、`tod_circle`、`confirm_incoming_panic_repro`。 | egui/toolbar | 无 | 原入口对照 | 未开始 |
| UM-APP-25 | `src/screenshot.rs` | 恢复 `gather_screenshot_requests`、`handle_screenshot_opportunity`、`finish_screenshot_capture`、`hurry_active_screenshots_on_shutdown`、`read_back_screenshot`、`write_image_with_callback_impl`。 | rhapsody/image | 无 | 原入口对照 | 未开始 |

### 杂项

| ID | B 源文件 | 任务内容（关键函数） | 依赖 | 现状/证据 | 验收 | 状态 |
|---|---|---|---|---|---|---|
| UM-APP-26 | `src/rollercoaster.rs` | 恢复 `rollercoaster_update`、`create_cart_chain`、`CoasterAudioInstance::new`、`RollercoasterState::new`、插件 build。 | 曲线/音频 | 无 | 原指令样例 | 未开始 |
| UM-APP-27 | `src/theme_fetcher_plugin.rs`、`src/fs_util.rs`、`src/vulkan_config_main.rs` | 恢复 `theme_fetcher_plugin` 的 build + `update_daily_theme`、`copy_folder`、`run_vulkan_configuration_app`。 | theme-fetcher | 无 | 原入口对照 | 未开始 |

## 卡点

- 依赖全部上层 + core + rhapsody；必须最后做。
- 平台后端（steamworks/DLSS/FSR/sentry/minidump/egui/ICC）多为外部 SDK 胶水，未恢复的保留显式接口，不填猜测默认值。
- `render_loop` 的帧顺序已有 `evidence/render-passes/` 分析（54 pass 调用 + 20 步帧外层 + 8 条件分支），可直接复用。
- 已删除 crate（crash-reporter-gui / feedback-reporter / tg-integrity-report / geforcenow）对应的接线不再纳入。

## 验收标准

- 启动链 ≥ 原入口对照；主循环/帧顺序与原一致；平台胶水与游戏业务分开记完成度。
- 游戏可启动、进主循环、跑到帧提交（GPU 画面验收可后置）。
