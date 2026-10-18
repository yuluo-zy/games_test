# crates/tiny-glade 启动/主循环/配置/日志 — RESTORE（restorer-tinyglade-A，批次 4）

日期：2026-10-17
角色：`restorer-tinyglade-A`（B 版，package `tiny-glade`）
工具：Ghidra 持久库 `tiny-glade.exe`（B patched，sha256 `cf048e27…`），
`--worker restorer-batch4-a`，**只读 shared 访问，未使用 `--write`，数据库未修改**。
反编译证据落盘：`recovery/agent-team/work/restorer-batch4-A/`（`0x<VA>_<name>.c`、
`disasm_*.txt`、`result*.json`）。

## 复核修正（reviewer-A-tinyglade）

- **FIX-A1（render OOM panic，已修）**：`country_core::systems::render_loop::render_frame::out_of_video_memory`
  原实现手工拼 `Some(({u32}, {extent:?}))`，与原生
  `core::option::impl$46::fmt<tuple<ash::vk::Format,array<u32,2>>>` 的 Debug 形状不符。
  现改为 `Option<(ash::vk::Format, [u32;2])>` 整体 `{info:?}`（内部 u32 经
  `vk::Format::from_raw` 还原），实测输出 `Some((B8G8R8A8_SRGB, [1920, 1080]))`；
  `should_panic` 期望串与新增格式断言已同步。待 reviewer 二次复核。
  注：该实现文件位于 country-core 的委托落点（reviewer 指定修正），未改其他模块。

## 三态回执

| 类别 | 数量 | 说明 |
|---|---|---|
| 判等 | 6 | `log_time_to_first_frame`、`FpsLimiter::frame`、`is_supported_steam_language`（内联白名单提取）、`render`、`render_world`、`handle_screenshot_error`（后三者复用 country-core 已判等实现并接归属导出面） |
| 重建 | 0 | — |
| 胶水（legitimate） | 5 | `run_game::impl$1::build`（空实现合并）、`impl$2::build`（输入插件注册，转发上游 `bevy_input::InputPlugin`，差异已登记）、`CliArgs` 的 `impl$5::from_arg_matches_mut`/`impl$6::augment_args`（clap derive）、`render_loop` 的导出接线 |
| 未实现 | 44 | 明细见下表（按函数计；`set_up_logging` 的两个格式化闭包分别计入）；均记录分歧点/阻断条件，无近似体冒充 |

`cargo check -p tiny-glade`：**通过**（2026-10-17 22:40 复跑；仅有
`special_module_name` 警告——B `src/main.rs` 作为模块挂载，`autobins=false`）。
`cargo test -p tiny-glade --lib`：**8 passed / 0 failed**（本组 5 个测试 +
既有 render_host/winit_bevy_glue 测试）。
运行期间同 crate 的 restorer-tinyglade-B 在途文件曾造成短暂编译错误，已由其
完成后恢复；最终状态以上述复跑为准。本组新增依赖：`puffin`、
`spin_sleep = "=1.1.1"`；`autobins = false`（见 `main.rs` 说明）。

## 逐函数判定表

| 函数 | VA | 类别 | verdict | 分歧点 / 阻断 |
|---|---|---|---|---|
| `tiny_glade::log_time_to_first_frame` | 0x1403db200 | 业务 | **判等** | 全局 `Option<Instant>`（secs@0x143634b88 / nanos@0x14362b0a8，niche 1e9）；`max_level() > Warn` 门；target `"tiny_glade"`；文案 `"time to first frame: {:?}"`（长度 0x15）。已在 `src/main.rs` 实现并测试 |
| `FpsLimiter::frame` | 0x140270850 | 业务 | **判等** | `used = min(elapsed, 2/fps)`；`acc += 1/fps - used`；clamp ±1/fps；`acc > 0` 时 `spin_sleep::sleep(Duration::from_secs_f32(acc))`；`last_frame` 尾部无条件更新；profile scope `"fps limiter"`。已在 `src/fps_limiter.rs` 实现并测试 |
| `load_language_library` 内联白名单 | 0x140323230 内 | 业务 | **判等（提取谓词）** | 15 个 Steam 语言名逐字节还原（`koreana`/`schinese`/`brazilian` 等），`src/load_language_library.rs` + 测试 |
| `render` | 0x1404e8010 | 业务 | **判等（委托）** | 逻辑已在 `country_core::systems::render_loop::render_frame::render`（周报：render_loop 需重做 0）；本 crate 以 `src/render_loop.rs` 导出，宿主设备走 `RenderHost` 缝 |
| `render_world` | 0x1404ec110 | 业务 | **判等（委托）** | 同上，`render_world.rs` 54 pass 顺序 + 分支 + panic 文案 |
| `handle_screenshot_error` | 0x1404ebfb0 | 业务 | **判等（委托）** | 同上；`Some(err)` → `log::error!` + drop |
| `run_game::impl$2::build` | 0x1405aeea0 | 胶水 | 胶水-legitimate | 5 事件 + 2 资源 + 2 系统（`InputSystem`）注册；本实现转发 `bevy_input::InputPlugin`，上游 0.16 另有手势/gamepad 注册（差异登记，不影响原消费面） |
| `run_game::impl$1::build` | 0x14003bb80 | 胶水 | 胶水-legitimate | 8 B 空实现合并 |
| `CliArgs::from_arg_matches_mut` / `augment_args` | 0x1403560c0 / 0x140357070 | 胶水 | 胶水-legitimate | clap derive 生成表；不计独立业务 |
| `main` | 0x1403db340 | 启动编排 | 未实现 | 已解 CALL 顺序（见 `src/main.rs` 文档）；缺 sentry/minidump/steamworks/rfd/crash-reporter-gui；PDB `void()` 占位 |
| `mock_crash_reporter` | 0x1403df140 | 平台胶水 | 未实现 | `ron::from_str::<CrashAdvice>` + `crash_reporter_gui::run_crash_reporter_gui`；crate 已删除 |
| `run_game` | 0x140593b70 | 启动 | 未实现 | Ghidra 反编译超时（23,863 B）；缺 winit fork/GraphicsDevicesForSurface/28 系统 tuple |
| `run_game_thread` | 0x14059d450 | 启动 | 未实现 | Ghidra 反编译超时（39,776 B）；库内可接部分见 `country_core::startup::run_game` |
| `make_temp_window` | 0x1405ae010 | winit 胶水 | 未实现 | 已反编译（11.5 KB）；窗口属性/事件面未接线 |
| `GameLoop::new` | 0x1404d8100 | 构造 | 未实现 | 纯字段 memcpy 已逐条解出（见 `src/game_loop.rs`）；字段类型/AvailableMonitors 等未落库 |
| `GameLoop::run` | 0x1404d8250 | 主循环 | 未实现 | 18 KB 反编译，winit 事件面 |
| `GameLoop::main_loop` | 0x1404d9580 | 主循环 | 未实现 | 111 KB 反编译，需 ReVa 切片 |
| `GameLoop::extract_available_monitors_info` | 0x1404e2470 | 平台胶水 | 未实现 | winit 监视器枚举 + 资源类型缺失 |
| `game_loop::impl$3::build` | 0x1404e3370 | 胶水 | 未实现 | 一行 `add_systems(PostUpdate, EventReader<ApplyVideoSettingsCmd>)`；系统体未恢复 |
| `gather_pending_video_setting_changes` | 0x1404e3380 | 业务 | 未实现 | 两级 Vec 排空 + `|= byte@+8` + 计数 + Mutex poison panic + state==2 wake（字节级已解）；`PendingVideoSettingChanges` 与全局标志类型未锚定 |
| `run_event_loop` | 0x14026e4a0 | 事件泵 | 未实现 | 38 KB winit/egui 胶水；`GameEvent`/`OsWindowEvent` 字段未落库 |
| `EguiEventIo::new` | 0x14026de50 | 胶水 | 未实现 | 0x138 B 堆块布局/字段语义未锚定 |
| `EguiEventIo::take_egui_input` | 0x14026dfd0 | 胶水 | 未实现 | `Mutex<egui_winit::State>` + puffin scope；缺 `egui-winit 0.27.2` |
| `setup_window` | 0x1400436b0 | winit 胶水 | 未实现 | 24 KB；窗口设置类型缺失 |
| `find_best_fullscreen_video_mode` | 0x1400453c0 | 业务/平台 | 未实现 | 40 KB；`AvailableVideoMode`/`RefreshRateMillihertz` 未落库 |
| `setup::impl$0::from_winit` | 0x140175450 | 胶水 | 未实现 | 监视器名/尺寸/refresh 转换已解；输出类型未锚定 |
| `setup::impl$1::from_winit` | 0x140175990 | 胶水 | 未实现 | `VideoMode::size` + `+0x18 u32` + `+0x1c u16` 转换已解；字段名未锚定 |
| `app_systems::add_wall_systems` | 0x140493120 | 注册 | 未实现 | 7 组注册顺序 + 泛型签名已解（见 `src/app_systems.rs`）；系统函数名/ResMut 面未逐个锚定 |
| `app_systems::add_wall_events` | 0x1404936b0 | 注册 | 未实现 | 39 KB 事件表未逐项解出 |
| `walls_on_glade_unload` | 0x1402b1cb0 | 业务 | 未实现 | 15 KB 卸载清理，未逐语句重建 |
| `decorator_plugin::impl$0::build` | 0x140342c10 | 注册 | 未实现 | 113 KB；整体转发 `add_decorator_systems` 会改变顺序/集合（callee 转发错），不采用 |
| `decorators_on_glade_unload` | 0x140347090 | 业务 | 未实现 | 40 KB 卸载清理 |
| `CliArgs::to_input_recording_opts` | 0x140353390 | 业务 | 未实现 | `field@0x68/0x88 → tag 1/2 + string + byte` 已解；`CliArgs` 字段名/偏移语义未锚定 |
| `get_cli_args` | 0x1403534c0 | 平台胶水 | 未实现 | clap `try_parse` → 失败 `tg_iced::MessageDialog` → `Error::exit` |
| `get_app_config` | 0x140353990 | 业务 | 未实现 | `settings.ron` / `default_settings.ron` / copy / RON 流程已解；`country_core::game_config::GameConfig` 是 Gap 占位（无 serde/字段） |
| `write_game_config_to_disk` | 0x140354240 | 业务 | 未实现 | `ron::to_string_pretty::<GameConfig>` + `fs::write`；同上 serde 缺口 |
| `write_settings_file` | 0x140354d70 | 业务 | 未实现 | `GameProfile::is_initial()` 门 → 写配置；serde 缺口 |
| `update_vulkan_allowlist_defaults` | 0x140354650 | 业务 | 未实现 | 读 `assets/vulkan_allowlist_defaults.ron`、flat_map/sort/dedup/write 流程已解；类型字段未锚定 |
| `logging::set_up_logging` | 0x1402e4760 | 业务 | 未实现 | fern 输出链/`crash_reporter_log.txt`/5 个 `level_for` 已解；`time` format_description 常量与模块名未解码；PDB 参数占位 |
| `logging` closure$0/$1 | 0x1402e53f0 / 0x1402e5740 | 胶水 | 未实现 | 时间/行格式化闭包，依赖上项常量 |
| `load_language_library` 本体 | 0x140323230 | 业务 | 未实现 | Steam 查询 + locale 代码映射；缺 steamworks，名称↔代码配对未锚定（白名单已判等） |
| `icc::get_monitor_icc_profile` | 0x1403ac400 | 平台胶水 | 未实现 | `CreateICA/GetICMProfileA(0x800)/DeleteDC` 流程已解；winapi target 依赖与错误 context 文案未锚定 |
| `gui::settings::impl$0::build` | 0x245570 | 注册 | 未实现 | egui 菜单插件注册面 |
| `LastAppliedVideoSettings::needs_apply` | 0x245630 | 业务 | 未实现 | Option 双哨兵 + `memcmp` 结构已解；字段语义未锚定 |
| `do_gui_escape_menu_settings` | 0x245790 | UI | 未实现 | egui 设置菜单；`VideoSettings` 面未闭合 |
| `ui_settings_game` | 0x247060 | UI | 未实现 | egui |
| `ui_settings_audio` | 0x2475d0 | UI | 未实现 | egui |
| `ui_settings_video` | 0x247870 | UI | 未实现 | egui（53 KB 反编译） |
| `ui_settings_ui` | 0x24aa80 | UI | 未实现 | egui |
| `ui_settings_developer` | 0x24ae60 | UI | 未实现 | egui |
| `ui_settings_controls` | 0x24b120 | UI | 未实现 | egui |
| `detect_konami_code` | 0x24b370 | 业务 | 未实现 | 目标序列 `[0x53,0x50,0x50,0x51,0x52,0x51,0x52,0x15,0x14,0x3a]`、命中 0xb 帧、`SHOW_DEVELOPER_SETTINGS=1` 已登记；按键位图来源/全局开关未接线 |

## 关键证据与交叉核对

- `log_time_to_first_frame`：反汇编 48 条逐条核对（`0x1403db207` 读 secs、
  `0x1403db20e` 读/`0x1403db214` 写 nanos=0x3b9aca00、`0x1403db224` JZ 退出、
  `0x1403db233` 读 `MAX_LOG_LEVEL_FILTER`、`0x1403db241` JBE 退出、
  `0x1403db24c` `Instant::elapsed`、`0x1403db32a` log 调用）；
  `main` 的写入点 `0x1403db372/0x1403db379` 由 xrefs 确认。
- 字符串/常量读取（read_memory）：`"time to first frame: "`(0x15)、target
  `"tiny_glade"`(0xa)、Location `main.rs:59:9`；app_config 区
  `settings.ron`/`default_settings.ron`/`assets/vulkan_allowlist_defaults.ron`
  及全部错误文案；logging 区 `crash_reporter_log.txt` 与两条失败文案。
- `add_wall_systems`：反汇编 + 泛型签名（`get_functions` 返回的
  `in_set<...>` 全名）还原 7 组注册；Location 静态（`0x1429df088` 等）给出
  app_systems.rs 行号 68/74/166/184。
- `FpsLimiter::frame`：反汇编确认 `Duration::from_secs_f32` panic 冷路径
  （0x140270cd4）与 `spin_sleep` 内联的 `timeBeginPeriod/timeEndPeriod`。

## 交付文件

`crates/tiny-glade/src/`：
- `fps_limiter.rs`（判等，含 3 个单元测试）
- `main.rs`（`log_time_to_first_frame` 判等 + 启动计时；`main`/`mock_crash_reporter` 未实现）
- `render_loop.rs`（render/render_world/handle_screenshot_error 归属导出，逻辑复用 country-core）
- `run_game.rs`（输入插件胶水 + 3 个未实现入口）
- `game_loop.rs`、`event_loop.rs`、`setup.rs`、`app_systems.rs`、
  `decorator_plugin.rs`、`app_config.rs`、`logging.rs`、
  `load_language_library.rs`（白名单判等 + 本体未实现）、`icc.rs`
- `gui/mod.rs`、`gui/settings.rs`
- `Cargo.toml`：`autobins = false`；新增 `puffin`、`spin_sleep = "=1.1.1"`
- `lib.rs`：挂载上述模块

## 归属其他组的阻断（不跨组修改）

1. **country-core / game_config 组**：`GameConfig` 仍是 `Gap` 占位（无真实字段、
   无 `Serialize/Deserialize`），阻断 `get_app_config`/`write_game_config_to_disk`/
   `write_settings_file`/`update_vulkan_allowlist_defaults`。
2. **host/render 组**：`AvailableMonitors`、`AvailableGraphicsDevices`、
   `AvailableVideoMode` 等监视器/设备类型未落库，阻断 `GameLoop`、`setup`。
3. **system-decorator 组**：`add_decorator_systems` 只接部分系统，
   `decorators_on_glade_unload` 未恢复，阻断 `decorator_plugin::build` 的逐注册项判等。
4. **依赖面**：`steamworks`（load_language_library）、`sentry`/`minidump*`/`rfd`
   （main）、`egui-winit 0.27.2`（EguiEventIo）、`crash-reporter-gui`（已删除，
   mock_crash_reporter）。
5. **同 crate 并发**：restorer-tinyglade-B 在途文件（`vram.rs`、`screenshot.rs`、
   `panic_reporter/*`、`winit_bevy_glue.rs`）在 22:36–22:39 造成短暂编译错误；
   其完成后 22:40 复跑 `cargo check -p tiny-glade` 已通过，测试 8/8 通过。

## 复现

```sh
cd /Users/yuluo/Desktop/game/games_test/tiny-glade
cargo check -p tiny-glade
cargo test  -p tiny-glade --lib   # fps_limiter / main / load_language_library 测试
```

反编译证据与 job/result：
`crates/country-core/recovery/agent-team/work/restorer-batch4-A/`（`job*.json`、
`result*.json`、各 `0x*.c`、`disasm_*.txt`）。
