# crates/tiny-glade B 恢复报告（批次 4，restorer-tinyglade-B）

日期：2026-10-17。范围：窗口/渲染 glue、崩溃上报、GUI 面板、截图与杂项（用户所列文件）。

证据：Ghidra 持久 B 库 `tiny-glade.exe`（SHA256 cf048e…6323），`--worker shared` 只读；每函数 fresh `get_functions`；关键常量用 `read_memory`/`r2 -a x86 -b 64` 复核。所有 VA 为 B。

判等方法：按 `planning/2026-10-16-logic-verify-protocol.md` 重建分支/循环/调用/数据流，并逐条对照 8 条分歧检查表。

## 汇总

| 判等 | 重建 | 胶水-非业务 | 未实现 | 合计 |
|---|---|---|---|---|
| 18 | 20 | 9 | 26 | 73 |

口径：`判等`=分支/循环/调用/数据流与 fresh 反编译逐条对应；`重建`=主体逻辑已按反编译重建但仍有已登记缺口（SDK/字段/schema）；`胶水`=derive/注册/纯转发/平台 SDK/空体；`未实现`=逻辑未还原到可判等，附分歧点。

## 函数表

| 文件 | 函数 | VA | verdict | 证据/分歧点 |
|---|---|---|---|---|
| `winit_bevy_glue.rs` | `winit_to_bevy_keycode` | 0x1400fb850 | 判等 | KeyEvent+0x58=PhysicalKey 判别；表 0x14294c58c 逐项=b+1；NativeKeyCode 五变体逐项拷贝 |
| `egui_backend.rs` | `EguiBackend::end_frame` | 0x1400dd910 | 判等 | FIX-A2：显式 drop viewport 表、逐 Shape drop 旧 Vec 后 swap 新 shapes、append 后 take 合并 delta；self 布局 +0 shapes/+0x18 delta/+0x60 Context |
| `egui_backend.rs` | `EguiBackend::draw` | 0x1400ddc80 | 胶水 | GPU 纹理上传/顶点索引提交为后端 glue；仅保留 tessellate+delta 编排 |
| `dlss_plugin.rs` | `DlssPlugin::build` | 0x14003bb80 | 胶水 | 该地址为 ICF 共享空体（单字节 RET，1581 个引用），build 为空注册 |
| `vram.rs` | `calculate_max_resolution_scale` | 0x1404f7f20 | 判等 | 常量 47.7399/146.109/10912/992811760/372245456/1365057216 与 bytes→GiB 均从 .rdata 读取；含原版 /0.0 |
| `vram.rs` | `calculate_resolution_scale_mem_estimate_text` | 0x1404f8240 | 重建 | 格式串文案 iGPU/GPU+SYS 与 GiB 换算逐常量对应；`{:.2}` 精度说明符未在 fmt::Arguments 中完全解码 |
| `aftermath.rs` | `NvAftermathDelegate::default` | 0x140244460 | 判等 | FIX-A3：逻辑移入 impl Default；Error 级 "Error deleting stale {path}: {error}" |
| `aftermath.rs` | `NvAftermathDelegate::dumped` | 0x140244790 | 重建 | zip 写入+crc32+日志结构已还原；SDK dump 文件清单由调用方提供 |
| `aftermath.rs` | `NvAftermathDelegate::shader_debug_info` | 0x140245340 | 判等 | FIX-A4：max_level>Debug 时 Trace 日志 "Got NV shader debug info! Ident: {:?}"（Some(identifier)）→ HashMap 插入 |
| `aftermath.rs` | `NvAftermathDelegate::description` | 0x14003bb80 | 胶水 | 空体（ICF） |
| `crash_pickup/crash_payload.rs` | `Serialize` fields plane (RecoveredSave/RecoveredSession/BacktracePayload/CrashRecovered/SaveRecoveryError/CrashPickupMessage) | 0x14003e250/0x14003e7c0/0x14003eb80/0x14003ec80/0x14003e350/0x14003eda0 | 判等 | 字段名由 `.rdata` 逐项解出：temp_save_path/session；session/folder/backup_folder；backtrace/message；breadcrumbs/report_folder/screenshot/save/inputs/log/profile/settings/gpu_crash_dump/rhapsody_log；GameNotYetInitialized/Other；Panic{payload}/recovered |
| `crash_pickup/mod.rs` | `CrashReporter::spawn_server` | 0x14015aeb0 | 重建 | temp_dir+uuid socket、current_exe+spawn+Child::id+client::start；SDK 参数字面量未解码 |
| `crash_pickup/mod.rs` | `ClientHandle::send_message` | 0x14015ac60 | 判等 | FIX-A6：serde_json::to_vec(...).unwrap()（unwrap_failed 文案一致）；成功走 send_message_impl |
| `crash_pickup/client.rs` | `client::start` | 0x14048a170 | 重建 | 50ms 重试/超时判断逐指令；reader 线程与 crash_handler::attach 为 SDK glue |
| `crash_pickup/server.rs` | `server::run` | 0x1402aee80 | 重建 | minidumper 事件循环骨架；SDK accept/dispatch 为 glue |
| `crash_pickup/server.rs` | `server::create_minidump_file` | 0x1402ad880 | 重建 | 临时文件名模板未在反编译中给出字面量 |
| `crash_pickup/server.rs` | `server::on_minidump_created` | 0x1402adae0 | 重建 | 记录路径并回传 client；SDK 回包形态 glue |
| `crash_pickup/server.rs` | `server::on_message` | 0x1402ae1f0 | 判等 | FIX-A7：from_slice(...).unwrap()（坏包 panic，原版 unwrap_failed）→分发 |
| `crash_pickup/server.rs` | `on_client_connected/disconnected` | - | 胶水 | 仅日志 |
| `crash_pickup/minidump_to_sentry.rs` | `minidump_to_crash_trace` | 0x1404b2fb0 | 未实现 | minidump crate 流解析为外部 SDK；仅有 MDMP 头/流数校验的 fallback parser |
| `crash_pickup/minidump_to_sentry.rs` | `crash_trace_to_sentry_event` | 0x1404b17d0 | 重建 | crash_reason→message、frames、modules→breadcrumbs；tag 键未取证 |
| `panic_reporter/mod.rs` | `setup_panic_hook` | 0x1400db5b0 | 重建 | take_hook→Box→set_hook 已还原；hook 闭包内 deadlock 检查/日志细节未全量 |
| `panic_reporter/mod.rs` | `setup_panic_hook::closure` | 0x1400db6b0 | 重建 | PANIC 日志→reverse backtrace→recover→ClientHandle::send_message→旧 hook |
| `panic_reporter/mod.rs` | `recover_game_state_after_crash` | 0x1400dbc30 | 重建 | 目录 tmp/panics/panic_{ts}、上下文串、8 个 dump 顺序（save/screenshot/rhapsody/inputs/log/settings/profile/gpu）逐调用对应；尾部 save/backup 路径消费面未闭合 |
| `panic_reporter/panic_dump.rs` | `dump_log_file` | 0x1403acc70 | 判等 | tmp/log.txt→report/log.txt，context 逐字 |
| `panic_reporter/panic_dump.rs` | `dump_inputs` | 0x1403ace70 | 未实现 | inputs.json 的 InputStream serde 形态未取证；当前用 InputFrameSerde 视图+计数 |
| `panic_reporter/panic_dump.rs` | `dump_save` | 0x1403ad640 | 未实现 | save.json 载荷字段名/历史结构未取证；当前写 ActiveSession+两路径 |
| `panic_reporter/panic_dump.rs` | `dump_settings_file` | 0x1403ae050 | 判等 | FIX-A8：config/settings.ron→report/<file_name>()（.unwrap()） |
| `panic_reporter/panic_dump.rs` | `dump_profile_file` | 0x1403ae2f0 | 未实现 | game_profile::profile_file_path 未在 country-core 恢复；当前 config/profile.ron |
| `panic_reporter/panic_dump.rs` | `dump_gpu_crash_dump_file` | 0x1403ae570 | 判等 | tmp/gpu-crash-dump.zip→report/ |
| `panic_reporter/panic_dump.rs` | `dump_rhapsody_log` | 0x1403ae770 | 判等 | FIX-A9：rhapsody::dump_log_vec()→fs::write(report/render.log) + context "Writing rhapsody log" |
| `panic_reporter/panic_report.rs` | `on_game_crashed` | 0x1403ae940 | 未实现 | 大体内联：manifest 校验/minidump trace/Sentry/tempdir/rename 顺序已列，但路径常量与打包结构未取证 |
| `panic_reporter/panic_screenshot.rs` | `create_graphics_resources` | 0x1405898a0 | 胶水 | rhapsody mapped buffer 创建/存放；设备侧 glue |
| `panic_reporter/panic_screenshot.rs` | `capture_image` | 0x140589c90 | 未实现 | 230400 像素缩放/dim 掩码已还原；compute pass 绑定与 dispatch 未接 |
| `panic_reporter/panic_screenshot.rs` | `dump_screenshot` | 0x14058a060 | 判等 | FIX-A10：screenshot_just_before_the_panic.jpg、RGB8 JPEG、两条尺寸文案 + context saving image + "Graphics resources not available" |
| `panic_reporter/sentry_reporter.rs` | `panic_info_to_message` | 0x1404b54c0 | 未实现 | 反编译仅剩 vtable 调用；payload→String 的精确路径未定 |
| `panic_reporter/sentry_reporter.rs` | `panic_info_to_sentry_event` | 0x1404b5550 | 未实现 | 大体内联；tags/fingerprint/extra 键名未取证 |
| `panic_reporter/sentry_reporter.rs` | `create_sentry_attachment` | 0x1404b4d50 | 重建 | file_name+读文件已还原；content_type 映射为推断 |
| `panic_reporter/sentry_reporter.rs` | `report_crash_event_to_sentry` | 0x1404b3e30 | 胶水 | Sentry scope/transport 为 SDK glue |
| `panic_reporter/reverse_backtrace.rs` | `ReverseBacktrace::fmt` | 0x1402cfc30 | 胶水 | 纯格式化适配器（reverse 行序） |
| `hang_watchdog.rs` | `hang_watchdog` | 0x140365e20 | 重建 | 20s 周期/1s 轮询/同 tick abort 已还原；thread_priority 与 deadlock dump 为 glue |
| `hang_watchdog.rs` | `spawn_hang_watchdog_thread` | 0x140365be0 | 判等 | Builder::name("hang watchdog").spawn |
| `integrity_report_main.rs` | `verify_package_integrity_and_show_report` | 0x1402552b0 | 重建 | manifest 校验+耗时日志+错误串已还原；tg_integrity_report GUI 已删除→trait |
| `screenshot.rs` | `gather_screenshot_requests` | 0x14031af10 | 重建 | ACTIVE mutex/文本分配/active 插入已还原；rhapsody 纹理分配为 glue |
| `screenshot.rs` | `handle_screenshot_opportunity` | 0x14031d140 | 重建 | 按 purpose/tag 匹配并回读；texture desc 维度分支简化 |
| `screenshot.rs` | `finish_screenshot_capture` | 0x14031ea20 | 重建 | state 1/0→readback→2、counter 递减、retain 已还原 |
| `screenshot.rs` | `hurry_active_screenshots_on_shutdown` | 0x14031eed0 | 重建 | active→state 1、join writer 线程；预算 Instant 差值结构未接 |
| `screenshot.rs` | `write_image_with_callback_impl` | 0x1403206a0 | 判等 | FIX-A11：长度校验；parent 两次（第二次 None=error）；jpg→RGBA→RGB+JpegEncoder::new_with_quality，其他→from_path+write_buffer；回调 Result<[u32;2],String> |
| `rollercoaster.rs` | `rollercoaster_update` | 0x1403b7f20 | 未实现 | 26KB 样条/朝向/实例写入体；曲线与 render instance 字段未闭合 |
| `rollercoaster.rs` | `create_cart_chain` | 0x1403be1a0 | 未实现 | 弧长/间距/变换栈未逐语句还原 |
| `rollercoaster.rs` | `CoasterAudioInstance::new` | 0x1403b6880 | 判等 | FIX-A12：volume<0.03→None；instantiate→set_volume→逐 (&str,f32)（0x18 步长）；失败日志 Error 级 |
| `rollercoaster.rs` | `RollercoasterState::new/plugin build` | - | 胶水 | 默认状态与注册 |
| `theme_fetcher_plugin.rs` | `update_daily_theme` | 0x1403d1750 | 未实现 | disk JSON→DailyThemeInfo/credits/ImageBuffer/SessionMeta 顺序已列；schema 与 channel 发布未取证 |
| `theme_fetcher_plugin.rs` | `plugin build/system` | - | 胶水 | Bevy 注册 |
| `fs_util.rs` | `copy_folder` | 0x140355800 | 判等 | create_dir_all→read_dir→is_dir&&!is_symlink→递归/复制；两处 monotization 同体 |
| `vulkan_config_main.rs` | `run_vulkan_configuration_app` | 0x1403e74b0 | 重建 | InstanceLoader/implicit_layers/GUI/写配置顺序已还原；winit+iced GUI 与 GameConfig 写为缺口 |
| `gui/confirm_incoming_panic_repro.rs` | `confirm_incoming_panic_repro` | 0x1400ed280 | 判等 | FIX-A13：状态机 state 2→第一框写 +0x1c/state=3；state 1 + pending Some + +0x48+1==frame_count→第二框→state=2；标题 😅；描述 227B 前缀+消息+69B 后缀 |
| `gui/gui_debug.rs` | `date_spinner` | 0x140148b40 | 重建 | DragValue+0.1 speed；范围描述符 0x1429d3710 未解码 |
| `gui/gui_debug.rs` | `do_gui_debug` | 0x140147e00 | 未实现 | egui Window 体/子面板未还原 |
| `gui/gui_debug.rs` | `do_freecam_save` | 0x1401485c0 | 未实现 | 按钮/文本/输入快照字段未落库 |
| `gui/gui_debug.rs` | `do_camera_snapshot_ui` | 0x140148cb0 | 未实现 | 水平布局/标签面板未还原 |
| `gui/gui_controls.rs` | `key_bindings_sub_menu` | 0x1400854a0 | 未实现 | 17 个 lang_lib 文案与键位行表未还原 |
| `gui/gui_controls.rs` | `show_control_tweakers` | 0x140088240 | 未实现 | Tweakable* 描述表未还原 |
| `gui/gui_controls.rs` | `show_gui_controls` | 0x1402ef5e0 | 未实现 | 总面板未还原 |
| `gui/gui_error_log.rs` | `do_gui_error_log` | 0x140160410 | 未实现 | Window::show_dyn 闭包未还原 |
| `gui/gui_tod.rs` | `do_time_of_day_screen` | 0x1405cc7f0 | 未实现 | MEI 节点树/WriteSettingsFileCmd 未还原 |
| `gui/gui_tod.rs` | `tod_circle` | 0x140334130 | 未实现 | 极坐标圆盘布局/命中未还原 |
| `gui/language_menu.rs` | `do_gui_language_menu` | 0x140150bb0 | 未实现 | 语言行表/global id/文案未还原 |
| `gui/gui_link_to_official_game.rs` | `display_link_to_official_game` | 0x1403d0c70 | 未实现 | MEI 模板/链接目标未还原 |
| `gui/gui_escape_menu.rs` | `do_gui_escape_menu` | 0x1403cf9b0 | 未实现 | MEI 节点树/17 闭包/EscMenu 状态机未还原 |
| `gui/gui_escape_menu.rs` | `wishlist_button` | 0x1403d07f0 | 未实现 | MenuButtonView/文案/Steam 资源未还原 |
| `gui/gui_escape_menu.rs` | `open_community_server` | 0x1403d09d0 | 未实现 | obfstr 29B URL 未解码；调用壳已还原 |
| `gui/gui_escape_menu.rs` | `open_newsletter` | 0x1403d0af0 | 未实现 | obfstr 68B URL 未解码 |

## 关键判等证据

1. `winit_to_bevy_keycode`：`winit::event::KeyEvent` 布局用 `offset_of!` 实测 `physical_key@0x58`；`word[+0x58]==5` 为 `PhysicalKey::Code` 的 niche；表 `0x14294c58c[ b^0x80 ] == b+1 (b<=193)`，与 winit 0.29→bevy_input 0.16 同名变体一一对应；非 Code 分支按同构 `NativeKeyCode` 五变体拷贝。

2. `vram`：`.rdata 0x1429ecb70..0x1429ecbcc` 逐 double 读取：`-47.7399274329577`、`146.1091661136655`、`10912.0`、`±992811760.0`、`±372245456.0`、`±1365057216.0`、`2^-30`。离散/统一分支与 `(scaled*10).floor()*0.1` 均与反汇编一致；保留原版 host 分支 `/0.0`（`xmm11` 在 0x1404f7fd4 清零后未再写）。

3. `panic_dump`：`.rdata` 串 `log.txt`、`Copying the log file`、`settings.ron`、`Copying the settings file`、`gpu-crash-dump.zip`、`render.log`、`Writing rhapsody log`、`inputs.json`、`save.json`；`recover_game_state_after_crash` 的 dump 顺序按调用点 189/195/213/231/249/267/285/303 固定。

4. `confirm_incoming_panic_repro`：`.rdata 0x14293d588`(140B) 与 `0x14293d614`(201B)+`0x14293d6dd`(33B) 逐字节读取，`sync_point_count()` 门与 `-1` 参数一致。

5. `fs_util::copy_folder`：Windows 属性判断 `FILE_ATTRIBUTE_DIRECTORY(0x10)`/reparse 与 `is_dir()&&!is_symlink()` 等价；递归/复制两分支逐语句对应。


## 8 条分歧检查表命中情况

- 已修正：`ClientHandle::send_message` 序列化失败原版 `unwrap_failed`（panic），初版返回 Err，已改为 `expect`（第 1 条）。

- 已登记未闭合：`dump_inputs`/`dump_save` 载荷 schema（第 3 条）、`on_game_crashed` 结构（第 3 条）、`minidump_to_crash_trace` callee 转发到 SDK parser（第 5 条）、`regex`/`todo` 均为显式未实现而非近似体。

- 未发现自行编造事件/命令（第 4 条）；未发现条件极性反向；返回结构均保留原多值/结构面或显式 trait。


## 构建与测试

- `cargo check -p tiny-glade`：通过（仅 `pub mod main;` 的 `special_module_name` 警告，来自并行的 restorer-tinyglade-A）。

- `cargo test -p tiny-glade --lib`：9 passed；其中新增 `winit_bevy_glue::tests::key_code_mapping_matches_b_table` 与 `fs_util::tests::copies_nested_tree`。


## 依赖接线

- 新增 `zip=0.6.6`、`crc32fast=1.3.2`（B Cargo.lock 同版本）、`bevy_input=0.16.0`、`egui=0.27.2`、`winit=0.29`、`uuid/tempfile/crossbeam-channel/serde/serde_json/image/once_cell`；均按 B `original-Cargo.lock` 身份。

- 外部 SDK（minidumper/crash_handler/sentry/aftermath/dlss/rfd/tg_integrity_report）未引入，统一以显式 trait 保留接口，不填猜测默认值。

## 宏生成/胶水函数分类（按协议不逐函数计）

- `crash_payload` 的 `Serialize`/`Deserialize` 实现（0x14003e250…0x14003f900）是 serde derive 生成体；字段平面已由 `.rdata` 键表逐项解出并落到 `crash_payload.rs`（见函数表）。`Deserialize` 端复用同一 derive 合同。
- `crash_pickup`/`panic_reporter` 的 `impl$N::fmt`/`drop_in_place`/`default`（derive/Drop）按 glue 处理；`aftermath::impl$1::description`、`dlss_plugin::build` 为单字节 RET 空体。
- Bevy/Bevy-ECS 注册（`init_resource`、`add_systems`）、`Plugin::build` 空体、trait 转发按 glue 处理。
- 本 crate 的 `main.rs`、`app_config.rs`、`game_loop.rs`、`event_loop.rs`、`render_loop.rs`、`setup.rs`、`gui/settings.rs` 等由 restorer-tinyglade-A 并行恢复，本报告未改动、未计入本批函数数。

## Reviewer-A 12 条 FIX 复核（2026-10-17）

| FIX | 结果 | 落点 |
|---|---|---|
| A2 | 已修 | `egui_backend.rs`（viewport/旧 Shape 销毁、shapes swap、delta append/take） |
| A3 | 已修 | `aftermath.rs` `impl Default` + Error 日志 |
| A4 | 已修 | `aftermath.rs` max_level>Debug + Trace Some(identifier) |
| A5 | 已修 | `crash_payload.rs` `Vec<sentry_types::protocol::v7::Breadcrumb>` |
| A6 | 已修 | `crash_pickup/mod.rs` `.unwrap()` |
| A7 | 已修 | `crash_pickup/server.rs` `.unwrap()`（坏包 panic） |
| A8 | 已修 | `panic_dump.rs` `file_name().unwrap()` |
| A9 | 已修 | `panic_dump.rs` `fs::write(dump_log_vec())` + context |
| A10 | 已修 | `panic_screenshot.rs` jpg/RGB8/两条文案/saving image/资源文案 |
| A11 | 已修 | `screenshot.rs` 扩展名分派/第二次 parent None=error/回调 `[u32;2]` |
| A12 | 已修 | `rollercoaster.rs` `&[(&str,f32)]` + Error 日志 |
| A13 | 已修 | `gui/confirm_incoming_panic_repro.rs` 状态机/😅 标题/227+69 包裹 |

注：A1（render OOM Debug 形状）属 restorer-tinyglade-A 文件，本批未改。
