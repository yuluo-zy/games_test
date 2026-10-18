# RESTORE — unlock-tinyglade-deps：GameConfig + AvailableMonitors/VideoMode + 平台依赖

> agent: `unlock-tinyglade-deps`，2026-10-17（402 中断后继续，上下文保留）。
> 范围：解 `crates/tiny-glade` 的阻断——`GameConfig`（core Gap）、
> `AvailableMonitors`/`VideoMode`、平台依赖（steamworks/sentry/minidump/rfd/egui-winit 等）。
> 红线遵守：Ghidra 持久 B 库 **shared 只读**（未开 `--write`、未改库）；
> 未改 rhapsody/其他游戏模块；只改 country-core 与 tiny-glade（本 crate）。
> 证据：`work/unlock-tinyglade-deps/`（fresh Ghidra job + 结果 JSON + 逐函数 `.c`/反汇编）。

## 三态回执

| 类别 | 数量 | 内容 |
|---|---|---|
| **完成（实现）** | 3 | `GameConfig` 真实字段/serde；`AvailableMonitors`/`AvailableMonitor`/`VideoMode`/`MonitorMatch` 布局；`MonitorMatch`/`VideoMode` 的 `FromWinit` 转换 |
| **完成（合同）** | 2 | `GraphicsWorkarounds`（原属 rhapsody，不可改他模块 → country-core 合同 seam）；`GameConfig.border_signifier_enabled`（glade-border 消费者未改名 → 镜像 seam） |
| **完成（依赖）** | 10 | sentry / sentry-types / egui-winit / egui_extras / rfd / minidump / minidump-common / steamworks / minidumper / crash-handler（版本/来源全部对齐原 B 锁） |
| **登记（依赖）** | 0 | 网络可达（crates.io HTTP 200）；未启用成员，故无缺包 |
| **阻塞/未实现（本批范围外）** | 4 | `GameLoop::run/main_loop/extract_available_monitors_info`、`setup_window`、`find_best_fullscreen_video_mode`、rfd/egui-winit 的实际接线（依赖已就位，函数体仍 todo） |

**`cargo check -p tiny-glade`：通过**（1 既有 `special_module_name` 警告）。
**`cargo test -p tiny-glade --lib`：9 passed / 0 failed。**
**`cargo test -p country-core --lib game_config render_config::tests`：14 passed / 0 failed**
（本批新增 8 个布局/serde 测试）。
`cargo check -p system-glade-border -p system-wall-constructor`：通过（GameConfig 合同字段兼容）。

## 1. fresh 证据（Ghidra shared 只读）

| # | 目标 | VA | 产物 |
|---|---|---|---|
| 1 | `GameConfig` serialize | 0x1402E73B0 | `ghidra-gameconfig-result.json` → `0x1402e73b0.c` |
| 2 | `GameConfig` FieldVisitor `visit_str` | 0x1402E7DE0 | `0x1402e7de0.c` |
| 3 | `GameConfig` Default | 0x140914880 | `0x140914880.c` |
| 4 | `GameConfig::upgrade`（内联 VideoSettings::upgrade） | 0x14090DF90 | `0x14090df90.c` |
| 5 | `DevSettings` serialize / FieldVisitor | 0x1402E7650 / 0x1402E7F20 | `0x1402e7650.c`、`0x1402e7f20.c` |
| 6 | `GameSettings` serialize / FieldVisitor / Default | 0x1402E7940 / 0x1402E8090 / 0x140914850 | `0x1402e7940.c`、`0x1402e8090.c`、`0x140914850.c` |
| 7 | `ScreenshotFormat` serialize | 0x1402E7D10 | 反汇编：0→"Jpeg"(0x1429B57A4,len4)、1→"Png"(0x1429B57A8,len3) |
| 8 | `VideoSettings` serialize / Default / Version 内联 | 0x14003A110 / 0x140968E80 / 0x14034F0F0 | `v2_0x14003a110.c`、`v2_0x140968e80.c`、`ghidra-nest-result.json` |
| 9 | `DisplayConfig` serialize / FieldVisitor / Default | 0x140039E00 / 0x14003AE40 / 0x140968E50 | `d_0x140039e00.c`、`d_0x140968e50.c` |
| 10 | `VideoMode` serialize / `MonitorMatch` serialize | 0x14003AAA0 / 0x140039C10 | `vs2_disasm_1/2`（字段名+偏移） |
| 11 | `GraphicsWorkarounds` serialize（rhapsody 原类型） | 0x140082A00 | `d_0x140082a00.c` + 4 个字段名 |
| 12 | `AudioSettings` serialize | 0x140168910 | 4 × f32 字段名 |
| 13 | `CameraRotationPivot` serialize / FieldVisitor | 0x14003AC90 / 0x14003B6A0 | 6 变体名（跳表 0x14292C04C + 字符串池 0x14292BB7C） |
| 14 | `AvailableMonitors::clear` | 0x140949C40 | `win_0x140949c40.c` + 反汇编（stride 0x40） |
| 15 | `MonitorMatch::from_winit` / `VideoMode::from_winit` | 0x140175450 / 0x140175990 | `win_0x140175450.c`、`win_0x140175990.c` + 反汇编 |
| 16 | 字段名 `.rdata` 逐字节 | — | `ghidra-strings*`、`ghidra-vsstrings*`、`ghidra-neststr*`、`ghidra-astr*` 结果 JSON |

## 2. GameConfig（落库 `country-core/src/game_config.rs`）

fresh `GameConfig` serialize（0x1402E73B0）与 FieldVisitor（0x1402E7DE0）逐字段判等：

| serde 序 | 字段 | 类型（原路径） | 内存偏移（O） |
|---|---|---|---|
| 0 | `video` | `render::render_config::VideoSettings` | +0x00（0xC8） |
| 1 | `audio` | `resources::audio_settings::AudioSettings` | +0xF8（0x10） |
| 2 | `ui` | `resources::ui_settings::UiSettings` | +0xE8（0x10） |
| 3 | `dev` | `game_config::DevSettings` | +0xE0（8 B） |
| 4 | `game` | `game_config::GameSettings` | +0x108（0x20） |
| 5 | `lang_override` | `Option<String>` | +0xC8（0x18） |

* `lang_override` 的 canonical 名来自 serialize；FieldVisitor 另有 `lang`(len 5)、
  `overwrite_lang`(len 14) 两个别名分支 → `#[serde(alias="lang", alias="overwrite_lang")]`。
* 容器 `#[serde(default)]`：B 发行 `default_settings.ron` 只有 `(tutorial: true,)`，
  未知字段被忽略、其余走默认（已加测试）。
* `upgrade` 0x14090DF90 = 内联 `VideoSettings::upgrade`：
  `if migrated&1==0 { f=legacy_scale; v=legacy_version; legacy_version=0;
  if v!=0 && (f!=1.0 || f.is_nan()) && 0.125<f { migrated=1; migrated_scale=f } }`。
* `DevSettings`（8 bool，@+0..+7，顺序即 serde 顺序）：
  `capture_crashes,report_crashes,internal_hotkeys,day_night_cycle,skip_splash,skip_welcome,demo,enable_asset_pipeline`；
  默认 `0x01000101` → T,T,F,T,F,F,F,F。`DevSettings::live_link` @0x14090DF40 读 +7
  = `enable_asset_pipeline`（方法名与字段名不同，已按物理字节落）。
* `GameSettings`（11 字段）与默认：sensitivity=1.0、movement=1.0、filter=0.3、fov=45.0、
  tod_speed=1.0、两个 pivot=None、invert=false、ScreenshotFormat=Jpeg、hide_border=false。
  两个 pivot 都是 `Option<CameraRotationPivot>`（PDB 无 `CameraZoomPivot` 实体），
  None 时 `skip_serializing_if`。
* `ScreenshotFormat`：`Jpeg=0`/`Png=1`，Display "JPEG"/"PNG"。

### VideoSettings（`render/render_config.rs`，22 字段，fresh 0x14003A110）

`version(VideoSettingsVersion)`, `display(DisplayConfig)`,
`world_resolution_scale(Option<f32>, None 跳过)`, `vsync_enable`, `dof_enable`,
`photomode_dof_enable`, `upscaling_mode`, `upscaling_preset`, `fps_limit`, `sharpen`,
`brightness_shift`, `gamma_shift`, `last_known_graphics_devices(Vec)`,
`graphics_device_uuid(Option<Uuid>)`, `graphics_validation`, `use_aftermath`,
`use_aftermath_checkpoints`, `graphics_find_crash`, `color_management`,
`render_upside_down`, `vulkan_layer_allowlist(GraphicsInstanceLayerAllowlist)`,
`workarounds(GraphicsWorkarounds)`。
默认 0x140968E80：vsync/dof/photomode=true、upscaling=Off/Quality、fps=60、
world_resolution_scale=None、uuid=None、aftermath/crash=false、preset=Quality。

`DisplayConfig`：`fullscreen`(+0x44,true)、`allow_exclusive_fullscreen`(+0x45,true)、
`fullscreen_config`(+0x30,Borderless)、`windowed_resolution`(+0x28,[1920,1080])、
`monitor`(+0,None)。

### 合同 seam

| seam | 原因 | 解锁条件 |
|---|---|---|
| `render_config::GraphicsWorkarounds` | 原类型在 `rhapsody::raw::workarounds`，红线“不改他模块” | rhapsody 落地该类型后改 `pub use` 并删除本定义 |
| `GameConfig.border_signifier_enabled` (`#[serde(skip)]`) | glade-border `display_glade_border` 仍读旧字段名；原字节 +0x121 = `game.hide_glade_border_outline` | glade-border 改读 `config.game.hide_glade_border_outline` 后删除 |
| `resources::audio_settings::AudioSettings` | 原模块此前缺失；`systems::audio::audio_update::AudioSettings` 是 5 音量占位（不同实体） | audio_update 改用原类型后删除占位 |

## 3. AvailableMonitors / VideoMode（落库）

* `resources::window::AvailableMonitors { monitors: Vec<AvailableMonitor> }`：
  clear 0x140949C40 证明 `Vec ptr@+8 / len@+0x10`，元素 stride 0x40。
* `resources::window::AvailableMonitor { video_modes: Vec<VideoMode> @0x00,
  monitor: MonitorMatch @0x18 }`：drop 循环证明 Vec<VideoMode> cap@+0/ptr@+8/len@+0x10
  （元素 16 B）与 MonitorMatch 的 String cap@+0x18/ptr@+0x20。
* `render_config::VideoMode { resolution:[u32;2]@0, bit_depth:u16@0xc,
  refresh_rate_millihertz:u32@8 }`（O，16 B；serde 名 fresh）。
* `render_config::MonitorMatch { name:Option<String>@0, physical_position:[i32;2]@0x18,
  physical_size:[u32;2]@0x20 }`（O，40 B）——**纠正**旧实现 `name: String`。
* `tiny_glade::setup::FromWinit<T>` + 两个 `from_winit`（0x140175450 / 0x140175990）：
  判等（winit fork 公开等价面 `name/position/size`、`size/bit_depth/refresh_rate_millihertz`）。
  单测断言 `size_of::<VideoMode>()==16`、`size_of::<MonitorMatch>()==40` 与 serde 字段名。

## 4. 平台依赖（`Cargo.toml` `[workspace.dependencies]`，身份 = 原 B 锁）

| package | 版本 | source | checksum（原锁） |
|---|---|---|---|
| sentry | 0.31.7 | crates.io | `0097a48cd1999d983909f07cb03b15241c5af29e5e679379efac1c06296abecc` |
| sentry-types | 0.31.7 | crates.io | `0e663b3eb62ddfc023c9cf5432daf5f1a4f6acb1df4d78dd80b740b32dd1a740` |
| egui-winit | 0.27.2 | crates.io | `2e3da0cbe020f341450c599b35b92de4af7b00abde85624fd16f09c885573609` |
| egui_extras | 0.27.2 | crates.io | `1b78779f35ded1a853786c9ce0b43fe1053e10a21ea3b23ebea411805ce41593` |
| rfd | 0.10.0 | crates.io | `0149778bd99b6959285b0933288206090c50e2327f47a9c463bfdbf45c8823ea` |
| minidump | 0.21.1 | crates.io | `64cb8353fdcd59c0caaf7525cb83b29ee02e6eac4c85d5a7944a9773d0f33f2e` |
| minidump-common | 0.21.1 | crates.io | `1bb6eaf88cc770fa58e6ae721cf2e40c2ca6a4c942ae8c7aa324d680bd3c6717` |
| steamworks (+steamworks-sys 0.11.0) | 0.11.0 | git PounceLight/steamworks-rs `d05eaa60db328156aeee172b0b157f3f69c70b30` | —（git） |
| minidumper 0.8.2 + crash-handler 0.6.1 | — | git PounceLight/crash-handling `acd03d6967e428100f37454766110e82118f5d7c` | —（git） |

说明：
* `tiny-glade/src/main.rs` 注释的 “rfd 0.15” 与原锁不符，已按锁取 **0.10.0**。
* 原锁无 `sentry-panic`，本批未加。
* workspace 级条目在无成员启用时不被 Cargo 解析（已用最小 workspace 实验确认），
  因此 `cargo check -p tiny-glade` 不触发下载；需要启用时把对应条目加到成员
  `Cargo.toml` 即按锁解析。网络核验：`static.crates.io` HTTP 200，无离线缺包。

## 5. 修改文件

| 文件 | 变更 |
|---|---|
| `crates/country-core/src/game_config.rs` | 重写为真实 `GameConfig`/`DevSettings`/`GameSettings`/`ScreenshotFormat` + serde + upgrade + 4 测试 |
| `crates/country-core/src/render/render_config.rs` | 纠正 `MonitorMatch`（Option<String>）；新增 `VideoMode`/`VideoSettingsVersion`/`DisplayConfig`/`GraphicsWorkarounds`/`VideoSettings` + 5 测试 |
| `crates/country-core/src/resources/audio_settings.rs` | 新增原类型（4 × f32） |
| `crates/country-core/src/resources/mod.rs` | `pub mod audio_settings;` |
| `crates/country-core/src/resources/window.rs` | 新增 `AvailableMonitors`/`AvailableMonitor` |
| `crates/country-core/src/systems/main_camera.rs` | 新增 `CameraRotationPivot`（6 变体） |
| `crates/country-core/src/lib.rs` | `pub mod main_camera;` |
| `crates/tiny-glade/src/setup.rs` | `FromWinit` + `MonitorMatch`/`VideoMode` 转换 |
| `Cargo.toml` | 10 个平台依赖（workspace） |

## 6. 剩余（本批不硬写）

1. `GameLoop::run/main_loop/extract_available_monitors_info`：winit 事件面/EnumDisplaySettingsW
   批量反编译未逐语句闭合（类型已就位，函数仍 `todo!`）。
2. `setup_window` / `find_best_fullscreen_video_mode`：24 KB / 40 KB，需 ReVa 切片。
3. `rfd`/`egui-winit`/`sentry`/`minidump`/`steamworks` 的实际调用接线：依赖身份已就位，
   函数体仍在 `main.rs`/`crash_pickup`/`event_loop` 的已登记 todo。
4. `VideoSettings.migration` 私有槽名称 I（字节行为 O）；`GraphicsWorkarounds` 与
   `border_signifier_enabled` 两个合同 seam 见 §2。
