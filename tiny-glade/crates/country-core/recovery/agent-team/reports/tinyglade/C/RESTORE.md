# crates/tiny-glade C 恢复报告（配置/设置/日志/GUI，restorer-tinyglade-C）

日期：2026-10-18（工作起始 2026-10-17）。范围：`app_config.rs`、`logging.rs`、
`load_language_library.rs`、`icc.rs`、`gui/settings.rs` 及 `gui/` 八个面板文件。
所有 VA 为 B（`tiny-glade.exe`，SHA256 `cf048e27…6323`）。

**工具与红线**：Ghidra 持久 B 库 `--worker shared` **只读**（未开 `--write`、
未修改数据库/脚本）；未改 `rhapsody`/`country-core`/其他 systems 模块。
fresh 证据落盘 `recovery/agent-team/work/restorer-tinyglade-C/`
（`0x<VA>_<name>.c`、`result*.json`、反汇编用 r2 定点）。

## 三态回执

| 类别 | 数量 | 说明 |
|---|---|---|
| 判等 | 12 | `to_input_recording_opts`、`get_cli_args`、`settings_file_path`、`get_app_config`、`write_game_config_to_disk`、`update_vulkan_allowlist_defaults`、`set_up_logging`、`formatted_time`、`formatted_date_time`、`open_community_server`、`open_newsletter`、`date_spinner` |
| 重建（含已登记 seam） | 4 | `write_settings_file`（`Res<PanicReproStateResource>` seam）、`load_language_library`（Steam trait seam）、`get_monitor_icc_profile`（winapi target + 非 Windows 平台缝）、`confirm_incoming_panic_repro`（对话框缝 + `Option` 哨兵极性登记） |
| 胶水-legitimate | 8 | clap derive 三件（`impl$4::command`/`impl$5::from_arg_matches_mut`/`impl$6::augment_args`）、fern 两个 format 闭包、`SettingsMenuPlugin::build` 注册、rfd 对话框缝、colored/winit 平台调用 |
| 未实现 | 23 | 明细见函数表（均为 UI/MEI 或字段语义未取证；附分歧点） |

**构建**：`cargo check -p tiny-glade` 通过（仅 3 个既有警告：`special_module_name`、
`screenshot.rs` 的 unused import/unused mut，非本组文件）。
`cargo test -p tiny-glade --lib`：**14 passed / 0 failed**（本组新增 4 个测试）。

## 函数表（fresh 反编译 verdict）

| 文件 | 函数 | VA | verdict | 证据/分歧点 |
|---|---|---|---|---|
| `app_config.rs` | `CliArgs::to_input_recording_opts` | 0x140353390 | 判等 | 先测 `record_input`(+0x68)，再 `replay_input`(+0x88)；tag 1/2，payload=PathBuf；否则 None。输出第 4 个 qword 低字节属 B 版 0x20 B PathBuf 内部字段（源级 `clone` 等价，已登记） |
| `app_config.rs` | `get_cli_args` | 0x1403534c0 | 判等 | `CliArgs::try_parse()`；Err → `MessageDialog::new(500,200)` + `set_level(Error)` + title `"Tiny Glade error"` + description `"Invalid Launch Options\nPlease check Tiny Glade ⚙ Properties in Steam\n\n" + err`；`Error::exit()` |
| `app_config.rs` | `settings_file_path`（inline，源行 166） | — | 判等 | `paths::config().join("settings.ron")` |
| `app_config.rs` | `get_app_config` | 0x140353990 | 判等 | 见下“关键证据 1” |
| `app_config.rs` | `write_game_config_to_disk` | 0x140354240 | 判等 | `ron::Options::default().to_string_pretty(config, PrettyConfig::default())`；序列化失败 `unwrap` panic；写失败 `log::error!("Failed to write settings to path {}: {}")` |
| `app_config.rs` | `update_vulkan_allowlist_defaults` | 0x140354650 | 判等 | 见“关键证据 2” |
| `app_config.rs` | `write_settings_file` | 0x140354d70 | 重建 | PDB 类型候选 `fn(Res<GameConfig>, Res<PanicReproState>)`；`!is_inactive()` → `log::info!("Panic repro settings override in action; NOT saving {}")`（门 `max_level>=Info`），否则写盘。共享 `PanicReproState` 无 `Resource` derive → `PanicReproStateResource` seam |
| `logging.rs` | `formatted_time`（inline，源行 30） | — | 判等 | 本地偏移：`[hour]:[minute]:[second].[subsecond digits:3]`（7 项）；UTC：同串 + 末尾字面量 `\u{1}`（8 项，逐字节复现） |
| `logging.rs` | `formatted_date_time`（inline，源行 11） | — | 判等 | 本地：`[year]-[month]-[day]T[hour]:[minute]:[second].[subsecond digits:3][offset_hour sign:mandatory]:[offset_minute]`（16 项）；UTC：同串以 `Z` 结尾（14 项） |
| `logging.rs` | `set_up_logging` | 0x1402e4760 | 判等 | 见“关键证据 3” |
| `load_language_library.rs` | `load_language_library` | 0x140323230 | 重建 | 见“关键证据 4”；Steam 客户端以 `SteamAppsLanguage` trait seam 保留 |
| `icc.rs` | `get_monitor_icc_profile` | 0x1403ac400 | 重建 | Windows：`monitor.name().unwrap()` → `CString::new(name).unwrap()`（含 NUL panic）→ `CreateICA` → `GetICMProfileA(size=0x800)` → 失败 `DeleteDC`+`anyhow!("Failed to obtain the monitor's ICC profile: {last_os_error}")` → `CString::from_vec_with_nul(...).context("The ICC profile could not be obtained")` → `into_string().context("Invalid ICC profile path")` → `DeleteDC`。**两个 CString 错误路径原版不 DeleteDC（泄漏），保持**。非 Windows：显式平台错误 |
| `gui/settings.rs` | `SettingsMenuPlugin::build` | 0x140245570 | 胶水 | `insert_resource(LastAppliedVideoSettings::default())`、`insert_resource(AvailableMonitors::default())`、`add_event::<ApplyVideoSettingsCmd>()` |
| `gui/settings.rs` | `do_gui_escape_menu_settings` | 0x140245790 | 未实现 | egui/MEI 面板；Tweakable*/lang 键/闭包环境未取证 |
| `gui/settings.rs` | `ui_settings_game` | 0x140247060 | 未实现 | egui UI |
| `gui/settings.rs` | `ui_settings_audio` | 0x1402475d0 | 未实现 | 4×`TweakableFloatWithStep`；语言键/步长未取证 |
| `gui/settings.rs` | `ui_settings_video` | 0x140247870 | 未实现 | 10,058 B egui UI |
| `gui/settings.rs` | `ui_settings_ui` | 0x14024aa80 | 未实现 | Tweakable*、`WindowFocusEvent`、`_DEV_OVERRIDE_IS_DEMO_MODE` 门；文案未取证 |
| `gui/settings.rs` | `ui_settings_developer` | 0x14024ae60 | 未实现 | egui UI |
| `gui/settings.rs` | `ui_settings_controls` | 0x14024b120 | 未实现 | egui UI |
| `gui/settings.rs` | `LastAppliedVideoSettings::needs_apply` | 0x140245630 | 未实现 | 偏移/分支已解（见下“未能闭合”），字段名与 payload 语义未取证 |
| `gui/settings.rs` | `LastAppliedVideoSettings::populate_empty_from`（源行 70） | inline | 未实现 | PDB 仅 `Option::get_or_insert_with` 内联证据 |
| `gui/settings.rs` | `detect_konami_code` | 0x14024b370 | 未实现 | 序列已解为 bevy `KeyCode` 判别式；HashMap 桶顺序与 0 值跳过项未闭合 |
| `gui/gui_escape_menu.rs` | `do_gui_escape_menu` | 0x1403cf9b0 | 未实现 | MEI 8 节点/15 闭包状态机 |
| `gui/gui_escape_menu.rs` | `wishlist_button` | 0x1403d07f0 | 未实现 | MenuButtonView image/lang/linearstep 常量 |
| `gui/gui_escape_menu.rs` | `open_community_server` | 0x1403d09d0 | 判等 | obfstr 解码 → `https://discord.gg/jcae7kejne` → `open_overlay_or_browser_to_webpage` |
| `gui/gui_escape_menu.rs` | `open_newsletter` | 0x1403d0af0 | 判等 | obfstr 解码 → `https://dashboard.mailerlite.com/forms/10395/78076204263933906/share` |
| `gui/confirm_incoming_panic_repro.rs` | `confirm_incoming_panic_repro` | 0x1400ed280 | 重建 | 见“关键证据 5” |
| `gui/gui_debug.rs` | `date_spinner` | 0x140148b40 | 判等 | `ui.add(DragValue::new(value).speed(0.1))`；范围描述符 0x1429d3710 数值未解码（已登记） |
| `gui/gui_debug.rs` | `do_gui_debug` | 0x140147e00 | 未实现 | egui Window + 子面板；闭包树未还原 |
| `gui/gui_debug.rs` | `do_freecam_save` | 0x1401485c0 | 未实现 | 输入快照/DebugGui 字段未落库 |
| `gui/gui_debug.rs` | `do_camera_snapshot_ui` | 0x140148cb0 | 未实现 | horizontal_with_main_wrap/label 组合未还原 |
| `gui/gui_controls.rs` | `key_bindings_sub_menu` | 0x1400854a0 | 未实现 | 5,898 B；17 lang 键 + 键位行表未还原 |
| `gui/gui_controls.rs` | `show_control_tweakers` | 0x140088240 | 未实现 | Tweakable* 描述表未还原 |
| `gui/gui_controls.rs` | `show_gui_controls` | — | 未实现 | **0x1402ef5e0 不是函数起点**（fresh get_functions not found）；VA 待重定位 |
| `gui/gui_error_log.rs` | `do_gui_error_log` | 0x140160410 | 未实现 | Window::new + show_dyn 闭包体 |
| `gui/gui_tod.rs` | `do_time_of_day_screen` | 0x1405cc7f0 | 未实现 | MEI 模板/`WriteSettingsFileCmd` 顺序 |
| `gui/gui_tod.rs` | `tod_circle<u64>` | 0x140334130 | 未实现 | 666 B 极坐标圆盘（此前“7 KB”记录有误） |
| `gui/language_menu.rs` | `do_gui_language_menu` | 0x140150bb0 | 未实现 | 语言行表/global id/文案 |
| `gui/gui_link_to_official_game.rs` | `display_link_to_official_game` | 0x1403d0c70 | 未实现 | 线性插值 + 节点 id `^0xb1695320004fb443` + 3 次 child_with_global_id + 2 个 with_slot；链接/标签常量未取证 |

## 关键证据

### 1. `get_app_config`（0x140353990）
- `settings.ron`（0xc）、`default_settings.ron`（0x14）、
  `"Unable to initialize settings: "`（31）+ `" does not exist."`（16）、
  `"Initialized settings "`（21）+ `" -> "`（4）（`.rdata 0x1429c3860…3900`）。
- 路径不存在 → 复制默认；默认源缺失/复制失败返回 `Err`（同一文案 context）。
- 读文件失败 **与** RON 解析失败 **都静默回落到 `GameConfig::default()`，无 error 日志**
  （此前 A 报告写有 `log::error`，与 fresh 反汇编不符）。
- 之后：`if cli.skip_splash || cli.repro_panic.is_some()` →
  `mov word [config+0xE4], 0x101` = `dev.skip_splash=true; dev.skip_welcome=true`
  （dev 基址 0xE0；bool 序 capture/report/internal/day_night/skip_splash/skip_welcome/...）。
- `config.upgrade()` 后 `Ok(config)`。

### 2. `update_vulkan_allowlist_defaults`（0x140354650）
- `assets/vulkan_allowlist_defaults.ron`（0x24）；读失败
  `log::error!("Error reading {}: {}")`（".rdata 0x1429c3ad0/0x1429c39a1"）。
- 反序列化失败 `log::error!("Error deserializing Vulkan allowlist defaults: {}")`。
- `VulkanLayerAllowlistDefaults { version: u32, add: Vec<GraphicsInstanceLayerAllowlistEntry> }`
  （`.rdata 0x1429b59ae` 字段名；元素内存 {Vec@0, version@0x18}，stride 0x20）。
- 内联闭包（0x1402b93e0）：`max_version=max(max_version, d.version)`；
  `d.version > allowlist.version` 才保留；`flat_map(add)`；extend。
- `allowlist.version = max(version, max_version)`（`cmova`）。
- 条目数≥2 时稳定排序+dedup；比较器 0x1405c1000 = 同变体 `memcmp(min(len))` 后比
  len，跨变体比 tag（0=Layer,1=Vendor）→ 与 `Ord` 派生一致；dedup 0x1402b6460 = `PartialEq`。
- 条目数变化且 `max_level>Warn` → `log::info!("Vulkan allowlist defaults have been updated.")`
  （**带句点**，44 B）；随后无条件写盘。

### 3. `set_up_logging`（0x1402e4760）
- `now_utc` → `UtcOffset::local_offset_at` → 全局 ok 字节 + 3 B offset
  （0x143634b80..83）；`colored::control::set_virtual_terminal(true)`（`mov cl,1`）。
- `Dispatch::new().level(LevelFilter)`（main 传 3=Info）`.format(闭包)` `.chain(stdout)`。
- 行格式 pieces `["", " ", " [", "] [frame:", "] ", ""]` →
  `"{time} {level} [{target}] [frame:{TICK_IDX}] {message}"`；stdout 用
  `formatted_time`，文件用 `formatted_date_time`。
- `file_logging` 时 5 个 `level_for`：`bevy_app`/`bevy_ecs`/`hotwatch`/`cosmic_text`=Warn(2)、
  `gpu_allocator`=Debug(4)；文件 `paths::tmp().join("log.txt")`
  （`.rdata 0x1429b4d10`，7 B；**不是** `crash_reporter_log.txt`），
  `OpenOptions` 三个 bool 置 1（write/create/truncate），打开失败 `unwrap` panic。
- `apply()` 失败：`anyhow!("Failed setting up the crash reporter logger: {e}")`。
- 时间格式描述与 `time 0.3.36::format_description::parse_borrowed::<2>` 逐项字节一致
  （含 UTC 时间串末尾的 0x01 字面量）。

### 4. `load_language_library`（0x140323230）
- 参数：`steam: Option<…>`（None → english）+ `lang_override: Option<&str>`
  （`cmovne rcx,rdi`；**Some 时直接作为 locale 代码，跳过白名单**）。
- Steam 路径：`GetCurrentGameLanguage` → lossy → `to_lowercase`。
- 白名单 15 项（按长度 6/7/8/9 分支、逐字节比较）。
- **名称→代码映射已完整解出**（此前记录“配对未锚定”是错的）：每个分支
  `lea rcx,[locale_table+off]` 指到 5 字节代码：
  english→en-US(0x1429bd63b)、italian→it-IT、koreana→ko-KR、russian→ru-RU、
  spanish→es-ES、swedish→sv-SE、turkish→tr-TR、french→fr-FR、german→de-DE、
  polish→pl-PL、japanese→ja-JP、schinese→zh-CN、brazilian→pt-BR、
  norwegian→no-NO、ukrainian→uk-UA（代码表 0x1429bd709 起）。
- 未支持：`log::warn!("Unsupported Steam language {}. Falling back to {}",
  language, "en-US")`，回落 en-US；最后 `lang_lib::load_locale(code).unwrap()`。

### 5. `confirm_incoming_panic_repro`（0x1400ed280）
- 状态字节（`*arg0`）：`2` → 第一个对话框（标题 **`😅`** 4 B，0x14293d868；
  描述 = 227 B 前缀 + message + 69 B 后缀），`show()` 返回值丢弃，状态→`3`；
  `1` 且 `[arg1+0x40]` 命中指针 niche 哨兵 `0x8000_0000_0000_0002` 且
  `[arg1+0x48]+1 == frame_count()` → 第二个对话框（标题 30 B
  `"Don't panic, it's only a panic"`，描述 151 B），状态→`2`。
- `[arg1+0x10] = [arg1+0x1c]` 是 Bevy `ResMut` change-detection，**不是**对话框结果。
- message：`sync_point_count()<=1` → 140 B 固定串；否则 201 B 头 + `n-1` + 33 B 尾。
  `.rdata` 0x14293d588/0x14293d614/0x14293d6dd/0x14293d720(0xe3)/0x14293d803(0x45)。
- 分歧登记：`Option` 哨兵极性（该 fork 的 niche）未由类型证据闭合，
  当前按 None 处理；rfd 按钮集合为平台胶水（trait seam）。

### 6. obfstr URL 解码
`xref::inner` 的地址计算已复原并用于解码：
- community（0x1403cf2e0）：`ptr = rcx + ((rol64(key+(-0x3cbce6c8) ^ 0x3d0e2e9c + 0x7961cf63,7) ^ low32) & 0xffff)`，
  数据 0x1429d1f47，29 B；XOR 密钥见文件头注释。
- newsletter（0x1403cf310）：数据 0x1429d1f64，68 B。
两串 UTF-8 均可打印，见常量 `COMMUNITY_SERVER_URL` / `NEWSLETTER_URL`。

## 未能闭合（未实现原因）

1. **egui/MEI UI 面**：6 个 `ui_settings_*` 与 `do_gui_escape_menu_settings`、
   `gui_*` 面板的控件描述符（步长/范围/枚举变体）、`lang_lib` 键、模板槽常量
   位于 `.rdata` 静态与闭包环境；仅凭内联调用序列不能逐语句判等。
   `show_gui_controls` 的 A 报告 VA `0x1402ef5e0` 经 fresh 查询不存在。
2. **`LastAppliedVideoSettings`**：PDB 仅有类型名（字段 unknown）。
   `needs_apply` 反汇编给出：`self[0].unwrap()`（None → panic）、
   `byte@0xa8` 不等 → 结果位、`Option`@0x38（双 None 哨兵 0x8000…0000/…0001）、
   `len@0x48 + memcmp(ptr@0x40)`、`0x68 u32 / 0x6c u64 / 0x74 u32 / 0x78 u16 /
   0x7c bool / 0x7d bool` 比较、`and al,6` 结果位。字段语义（分辨率/刷新率/模式）
   无 PDB 字段名，不能命名或断言 payload。
3. **`detect_konami_code`**：系统签名 `fn(Res<ButtonInput<KeyCode>>, Local<usize>)`；
   序列目标值已解为 `KeyCode` 判别式：0x53/0x50/0x51/0x52/0x15/0x14/0x3a =
   ArrowUp/ArrowDown/ArrowLeft/ArrowRight/KeyB/KeyA/Enter。未闭合点：HashMap
   控制字节桶迭代顺序、12 B 桶条目偏移、序列第 11 项/0 值“跳过”记录与
   `SHOW_DEVELOPER_SETTINGS` 全局写点。按硬标准不写近似状态机。
4. **额外同文件 PDB 函数**（不在本批任务清单，未计入 23 项）：
   `ui_settings_keybindings`（含 9 闭包）、`ui_settings_video_main/display/image`、
   `get_quailty_presets`、`render_gpu_allocator_gui`、`do_tod_save_load_ui`、
   `do_tod_sequence_entry`、`parabola`/`remap_angle_to_tod`/`remap_tod_to_angle`、
   `TodCircleView::{delta,ui_confirm}`、settings 的 `impl$2..impl$6 fmt/from`。

## 对既有报告的更正

1. A：`get_app_config` 读/解析失败 **无** `log::error`（静默 default）。
2. A：`write_settings_file` 的门是 `PanicReproState::is_inactive()`（ICF 体
   `cmp byte,0; sete`），日志级别是 `log::info`（`max_level>=3`），路径为
   `settings.ron`；不是 `GameProfile::is_initial` + `log::debug`。
3. A/B：`set_up_logging` 文件是 `tmp/log.txt`；5 个 `level_for` 名字已解出；
   UTC 时间格式末尾有 0x01 字面量。
4. A：`load_language_library` 的名称↔代码配对 **已解出**；第二参数是
   `lang_override`（Some 时直用代码）。
5. B：`confirm_incoming_panic_repro` 的标题/描述与状态机需重做：首对话框标题是
   `😅`、描述带 227/69 B 包装；`show()` 结果丢弃（+0x1c 写是 change-detection）。
6. B：两个 obfstr URL **已解码**（discord / mailerlite）。
7. B：GUI 函数 VA 被截断成 `0x245570` 等，实际为 `0x140245570`；
   `show_gui_controls 0x1402ef5e0` 不是函数起点。

## 交付文件

- `crates/tiny-glade/src/app_config.rs`（CLI/CliArgs/配置读写/Vulkan 白名单）
- `crates/tiny-glade/src/logging.rs`（fern 初始化 + 时间格式）
- `crates/tiny-glade/src/load_language_library.rs`（Steam→locale + 映射）
- `crates/tiny-glade/src/icc.rs`（Windows ICC 读取 + 平台缝）
- `crates/tiny-glade/src/gui/settings.rs`（插件注册 + 未实现登记）
- `crates/tiny-glade/src/gui/gui_escape_menu.rs`（URL 解码判等）
- `crates/tiny-glade/src/gui/confirm_incoming_panic_repro.rs`（状态机/文案修正）
- `crates/tiny-glade/src/gui/{gui_debug,gui_controls,gui_error_log,gui_tod,
  language_menu,gui_link_to_official_game}.rs`（fresh 证据与分歧点更新）

## 依赖变更（对齐 `provenance/original-Cargo.lock`）

| 依赖 | 版本 | 用途 |
|---|---|---|
| `clap`（derive） | `=4.4.6` | `CliArgs`（B lock 同版） |
| `fern` | `=0.6.1` | 日志分发 |
| `colored` | `=1.9.3` | Windows VT 序列 |
| `tg-iced`（本地） | — | `MessageDialog` 构造（原生 show 为缝） |
| `winapi`（`cfg(windows)`） | `=0.3.9` | `CreateICA/GetICMProfileA/DeleteDC` |

`time` 启用 `formatting, macros, local-offset`（B 使用 `format_description!`
与本地偏移探测）。
