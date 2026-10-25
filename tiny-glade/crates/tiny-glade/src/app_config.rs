//! B `crates/tiny-glade/src/app_config.rs`（CLI、游戏配置、视频设置事件、Vulkan 白名单）。
//!
//! 恢复依据（fresh Ghidra shared 2026-10-17，证据在
//! `recovery/agent-team/work/restorer-tinyglade-C/`）：
//! - `CliArgs::to_input_recording_opts` `0x140353390`
//! - `impl$5::from_arg_matches_mut` `0x1403560c0` / `impl$6::augment_args`
//!   `0x140357070`（clap derive 胶水；字段/长名/类型由 remove_one 序列与 `.rdata` 复原）
//! - `get_cli_args` `0x1403534c0`
//! - `settings_file_path`（inline，源行 166）
//! - `get_app_config` `0x140353990`
//! - `write_game_config_to_disk` `0x140354240`
//! - `update_vulkan_allowlist_defaults` `0x140354650`
//! - `write_settings_file` `0x140354d70`（Bevy system:
//!   `fn(Res<GameConfig>, Res<PanicReproState>)`，PDB 类型候选）
//!
//! ## 逐语句复原要点（fresh 反汇编 + `.rdata` 逐字节）
//! - `settings_file_path()` = `paths::config().join("settings.ron")`
//!   （`.rdata 0x1429c3860`，len 0xc）。
//! - `get_app_config`：
//!   1. 路径不存在（`stat` 任意 Err）→ 源 `paths::installed().join(
//!      "default_settings.ron")`；源也不存在 → `anyhow!("Unable to initialize
//!      settings: {} does not exist.", src)`；否则 `fs::copy` 并以同一文案
//!      `with_context`（`get_app_config::closure$0`，源行 190）；
//!      成功且 `max_level > Warn` → `log::info!("Initialized settings {} -> {}", src, dst)`。
//!   2. `fs::read_to_string` 失败 **或** RON 解析失败 → `GameConfig::default()`
//!      （原版两条路径都静默回落，均无 error 日志；A 报告此处曾误记 log::error）。
//!   3. `if cli.skip_splash || cli.repro_panic.is_some()`
//!      → `config.dev.skip_splash = true; config.dev.skip_welcome = true`
//!      （反汇编 `mov word [rbp+0x1ac], 0x101`，config base +0xE4/+0xE5；
//!      门 `[cli+0x148]`=skip_splash bool、`[cli+0xa8]`=repro_panic Option<PathBuf>）。
//!   4. `config.upgrade()`；`Ok(config)`。
//! - `write_game_config_to_disk(&GameConfig)`：
//!   `ron::Options::default().to_string_pretty(config, PrettyConfig::default())`
//!   **`.unwrap()` panic**；`fs::write` 失败 → `log::error!(
//!   "Failed to write settings to path {}: {}", path, err)`（源行 230:69）。
//! - `write_settings_file(config, panic_repro_state)`：
//!   路径先算出；`!panic_repro_state.is_inactive()`（0x140833230 ICF 体
//!   `cmp byte,0; sete`）→ `log::info!("Panic repro settings override in action;
//!   NOT saving {}", path)`（`cmp max_level,3` = Info）；否则写盘。
//! - `update_vulkan_allowlist_defaults(&mut GameConfig)`：
//!   1. `paths::installed().join("assets/vulkan_allowlist_defaults.ron")`
//!      （len 0x24）读文件；读失败 → `log::error!("Error reading {}: {}", path, e)`。
//!   2. `ron::from_str::<Vec<VulkanLayerAllowlistDefaults>>`（字段 `version`,`add`，
//!      `.rdata 0x1429b59c0`）；失败 → `log::error!(
//!      "Error deserializing Vulkan allowlist defaults: {}", e)`。
//!   3. `filter(|d| { max_version = max(max_version, d.version);
//!      d.version > allowlist.version })` → `flat_map(|d| d.add)` → extend
//!      （反汇编 0x1402b93e0 内联闭包：`*max = max(*max, ver);
//!      if allowlist.version < ver` 才保留）。
//!   4. `allowlist.version = max(version, max_version)`（`cmova`）。
//!   5. `entries.sort(); entries.dedup();`（稳定排序；比较器 = enum
//!      判别式后字符串，`memcmp(min(len))` 后比 len — 与 `Ord` 派生一致）。
//!   6. 若条目数变化且 `max_level > Warn` → `log::info!("Vulkan allowlist
//!      defaults have been updated.")`（注意原文带句点，len 0x2c）。
//!   7. `write_game_config_to_disk(config)`。
//! - `CliArgs::to_input_recording_opts()`：
//!   `if let Some(p) = record_input { Record(p.clone()) }
//!    else if let Some(p) = replay_input { Replay(p.clone()) } else { None }`
//!   （反汇编 0x140353390：先测 +0x68，再测 +0x88；输出 tag 1/2，
//!    payload = PathBuf；B 版 PathBuf 为 0x20 B）。
//!
//! ## 胶水/平台缝
//! - clap derive（`Parser`/`Args`/`FromArgMatches` 表）是宏生成胶水，按
//!   remove_one 顺序与 `.rdata` 长名恢复 23 个字段，不逐语句审计。
//! - `tg_iced::dialog::MessageDialog` 的原生 `show()` 为平台 GUI；本 crate 的
//!   tg-iced 在无 `native-gui` feature 时不提供后端，故经 [`CliErrorDialogBackend`]
//!   缝保留，默认实现只记录（不伪造原生窗口）。
//!
//! ## 已登记差异
//! - `to_input_recording_opts` 在 B 中还把 record/replay Option 载荷第 4 个
//!   qword 的低字节原样拷入输出（`[cli+0x80]` / `[cli+0xa0]`）。该字节属于
//!   B 版 `PathBuf`（0x20 B 表示）的内部字段；本 crate 的 std PathBuf 无该
//!   字段，源级以 `PathBuf::clone` 等价表达（无语义分歧）。
//! - `crash_reporter_send_to_sentry` 的 `.rdata` 带 `"false"` 默认串与值名，
//!   按取值 bool 恢复；其余 bool 为 `SetTrue` 旗标（derive 胶水）。

use std::path::PathBuf;

use anyhow::Context;
use bevy_ecs::change_detection::Res;
use bevy_ecs::resource::Resource;
use clap::Parser;
use country_core::game_config::GameConfig;
use country_core::input::panic_repro_state::PanicReproState;
use rhapsody::{
    interop::GraphicsInstanceLayerAllowlist, interop::GraphicsInstanceLayerAllowlistEntry,
};

/// `settings.ron`（`.rdata 0x1429c3860`，len 0xc）。
pub const SETTINGS_FILE: &str = "settings.ron";
/// `default_settings.ron`（`.rdata 0x1429c386c`，len 0x14）。
pub const DEFAULT_SETTINGS_FILE: &str = "default_settings.ron";
/// `assets/vulkan_allowlist_defaults.ron`（`.rdata 0x1429c39e0`，len 0x24）。
pub const VULKAN_ALLOWLIST_DEFAULTS_FILE: &str = "assets/vulkan_allowlist_defaults.ron";

/// clap 解析失败对话框的描述前缀（`.rdata 0x1429c3808`，72 B）。
pub const CLI_ERROR_PREFIX: &str =
    "Invalid Launch Options\nPlease check Tiny Glade ⚙ Properties in Steam\n\n";

/// clap 解析失败对话框标题（`.rdata 0x1429c37f8`，16 B）。
pub const CLI_ERROR_TITLE: &str = "Tiny Glade error";

/// B 0x1404e3380 reads the byte at EventInstance +8 and ORs it into pending
/// flags. Main-loop consumes bit 1 for vsync and bits 0/2 for display changes.
#[derive(Debug, Default, Clone, Copy, bevy_ecs::event::Event)]
pub struct ApplyVideoSettingsCmd(pub u8);

// One event identity for onboarding/settings producers and the OS consumer.
pub use country_core::resources::events::WriteSettingsFileCmd;

/// `tiny_glade::app_config::PendingVideoSettingChanges`（B 资源；字段未取证）。
#[derive(Debug, Default)]
pub struct PendingVideoSettingChanges {
    _private: (),
}

/// `CliArgs` 的输入录制选择（`to_input_recording_opts` 输出）。
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InputRecordingOpts {
    /// `--record-input`（tag 1）。
    Record(PathBuf),
    /// `--replay-input`（tag 2）。
    Replay(PathBuf),
}

/// `tiny_glade::app_config::CliArgs`（clap derive）。
///
/// 字段顺序 = `from_arg_matches_mut` 的 `remove_one` 调用顺序（0x1403560c0），
/// 类型 = 对应 `remove_one<T>` 单态；长名/值名取自 `.rdata` 0x1429c3b82 起。
#[derive(Parser, Debug, Default)]
#[command(name = "Tiny Glade", version = "0.2.0")]
pub struct CliArgs {
    /// `--crash-reporter-server`（`Option<String>`，remove_one<String>）。
    #[arg(long = "crash-reporter-server", value_name = "CRASH_REPORTER_SERVER")]
    pub crash_reporter_server: Option<String>,
    /// `--crash-reporter-send-to-sentry`（取值 bool，`.rdata` 默认串 "false"）。
    #[arg(
        long = "crash-reporter-send-to-sentry",
        value_name = "CRASH_REPORTER_SEND_TO_SENTRY",
        default_value_t = false
    )]
    pub crash_reporter_send_to_sentry: bool,
    /// `--ignore-vram-budget` / 别名 `--disable-vram-budget`。
    #[arg(
        long = "ignore-vram-budget",
        visible_alias = "disable-vram-budget",
        value_name = "IGNORE_VRAM_BUDGET"
    )]
    pub ignore_vram_budget: bool,
    #[arg(long = "track-gpu-allocations", value_name = "TRACK_GPU_ALLOCATIONS")]
    pub track_gpu_allocations: bool,
    #[arg(
        long = "disable-steam-overlay-hooks",
        value_name = "DISABLE_STEAM_OVERLAY_HOOKS"
    )]
    pub disable_steam_overlay_hooks: bool,
    #[arg(long = "select-gpu", value_name = "SELECT_GPU")]
    pub select_gpu: bool,
    #[arg(long = "configure-vulkan", value_name = "CONFIGURE_VULKAN")]
    pub configure_vulkan: bool,
    #[arg(long = "verify-integrity", value_name = "VERIFY_INTEGRITY")]
    pub verify_integrity: bool,
    #[arg(long = "max-game-threads", value_name = "MAX_GAME_THREADS")]
    pub max_game_threads: Option<usize>,
    #[arg(
        long = "single-threaded-executor",
        value_name = "SINGLE_THREADED_EXECUTOR"
    )]
    pub single_threaded_executor: bool,
    #[arg(long = "override-save-root", value_name = "OVERRIDE_SAVE_ROOT")]
    pub override_save_root: Option<PathBuf>,
    #[arg(
        long = "override-save-root-user-id",
        value_name = "OVERRIDE_SAVE_ROOT_USER_ID"
    )]
    pub override_save_root_user_id: Option<u64>,
    #[arg(long = "record-input", value_name = "RECORD_INPUT")]
    pub record_input: Option<PathBuf>,
    #[arg(long = "replay-input", value_name = "REPLAY_INPUT")]
    pub replay_input: Option<PathBuf>,
    #[arg(long = "repro-panic", value_name = "REPRO_PANIC")]
    pub repro_panic: Option<PathBuf>,
    #[arg(long = "panic-repro-rewind", value_name = "PANIC_REPRO_REWIND")]
    pub panic_repro_rewind: Option<usize>,
    #[arg(long = "skip-splash", value_name = "SKIP_SPLASH")]
    pub skip_splash: bool,
    #[arg(long = "fmod-asynchronous", value_name = "FMOD_ASYNCHRONOUS")]
    pub fmod_asynchronous: bool,
    #[arg(long = "detect-hang", value_name = "DETECT_HANG")]
    pub detect_hang: bool,
    /// `--mc`：meta commander 脚本文件。
    #[arg(long = "mc", value_name = "MC")]
    pub mc: Option<PathBuf>,
    #[arg(long = "dump-system-schedule", value_name = "DUMP_SYSTEM_SCHEDULE")]
    pub dump_system_schedule: Option<PathBuf>,
    #[arg(long = "print-render-log", value_name = "PRINT_RENDER_LOG")]
    pub print_render_log: Option<PathBuf>,
    #[arg(long = "mock-crash-reporter", value_name = "MOCK_CRASH_REPORTER")]
    pub mock_crash_reporter: Option<String>,
    /// Local host adapter: keep this launch windowed, including video-setting changes.
    /// This temporary choice is never written over the saved display preferences.
    #[arg(long = "windowed")]
    pub windowed: bool,
    /// Local host adapter: physical window size; implies --windowed.
    #[arg(long = "window-size", value_name = "WIDTHxHEIGHT")]
    pub window_size: Option<LaunchWindowSize>,
}

/// Explicit host launch size, independent of the original persisted schema.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LaunchWindowSize(pub [u32; 2]);

impl std::str::FromStr for LaunchWindowSize {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let size = value.split_once('x').and_then(|(width, height)| {
            Some([width.parse::<u32>().ok()?, height.parse::<u32>().ok()?])
        });
        match size {
            Some([width, height]) if width > 0 && height > 0 => Ok(Self([width, height])),
            _ => Err(format!("window size must be positive WIDTHxHEIGHT, got {value:?}")),
        }
    }
}

/// Per-process display policy. Preserve other settings and monitor changes when
/// saving, while restoring only the fields temporarily overridden at launch.
#[derive(Resource, Clone, Debug)]
pub struct LaunchDisplayOverride {
    original: country_core::render::render_config::DisplayConfig,
    resolution: Option<[u32; 2]>,
    pub force_windowed: bool,
}

impl LaunchDisplayOverride {
    /// Explicit CLI takes precedence over legacy standalone environment options.
    /// --windowed alone uses a bounded 960x540 physical window. Existing
    /// TG_WINDOW_SIZE remains supported; merely having TG_FULLSCREEN retains
    /// its historical meaning of disabling the standalone windowed default.
    pub fn resolve(
        cli: &CliArgs,
        display: &country_core::render::render_config::DisplayConfig,
        standalone: bool,
        env_fullscreen_present: bool,
        env_window_size: Option<&str>,
    ) -> anyhow::Result<Option<Self>> {
        let explicit = cli.windowed || cli.window_size.is_some();
        let resolution = if let Some(size) = cli.window_size {
            Some(size.0)
        } else if explicit {
            Some([960, 540])
        } else if standalone && env_window_size.is_some() {
            Some(env_window_size.unwrap().parse::<LaunchWindowSize>().map_err(anyhow::Error::msg)?.0)
        } else {
            None
        };
        let force_windowed = explicit || (standalone && !env_fullscreen_present);
        Ok((force_windowed || resolution.is_some()).then(|| Self {
            original: display.clone(),
            resolution,
            force_windowed,
        }))
    }

    pub fn apply(&self, display: &mut country_core::render::render_config::DisplayConfig) {
        if self.force_windowed {
            display.fullscreen = false;
            display.allow_exclusive_fullscreen = false;
        }
        if let Some(size) = self.resolution {
            display.windowed_resolution = size;
        }
    }

    pub fn persistence_config(&self, config: &GameConfig) -> GameConfig {
        let mut saved = config.clone();
        if self.force_windowed {
            saved.video.display.fullscreen = self.original.fullscreen;
            saved.video.display.allow_exclusive_fullscreen = self.original.allow_exclusive_fullscreen;
        }
        if self.resolution.is_some() {
            saved.video.display.windowed_resolution = self.original.windowed_resolution;
        }
        saved
    }
}

impl CliArgs {
    /// `CliArgs::to_input_recording_opts` `0x140353390`。
    ///
    /// 先测 `record_input`（+0x68），再测 `replay_input`（+0x88）；
    /// 命中即克隆路径并给 tag 1/2，两者皆 None → None。
    pub fn to_input_recording_opts(&self) -> Option<InputRecordingOpts> {
        if let Some(path) = &self.record_input {
            Some(InputRecordingOpts::Record(path.clone()))
        } else if let Some(path) = &self.replay_input {
            Some(InputRecordingOpts::Replay(path.clone()))
        } else {
            None
        }
    }
}

/// clap 解析错误对话框的原生呈现缝。
///
/// B 版直接调用 `tg_iced::dialog::MessageDialog::show()`（iced 原生窗口）；
/// 无 `native-gui` 的构建无后端，故只接收已构造好的对话框。
pub trait CliErrorDialogBackend {
    fn show_cli_error(&mut self, dialog: tg_iced::dialog::MessageDialog);
}

/// 默认后端：丢弃对话框（平台 GUI 胶水；解析与退出逻辑仍完整执行）。
#[derive(Debug, Default)]
pub struct NoopCliErrorDialogBackend;

impl CliErrorDialogBackend for NoopCliErrorDialogBackend {
    fn show_cli_error(&mut self, _dialog: tg_iced::dialog::MessageDialog) {}
}

/// `get_cli_args` `0x1403534c0`。
///
/// `CliArgs::try_parse()`；失败时构造 `MessageDialog::new(500, 200)`：
/// `set_level(MessageLevel::Error)`、`set_title("Tiny Glade error")`、
/// `set_description("Invalid Launch Options\n…\n\n" + err)`，随后
/// `clap::Error::exit()`（不返回）。
pub fn get_cli_args() -> CliArgs {
    get_cli_args_with(&mut NoopCliErrorDialogBackend)
}

/// 带对话框后端的 `get_cli_args`（B 版无此参数；后端为平台胶水缝）。
pub fn get_cli_args_with(backend: &mut impl CliErrorDialogBackend) -> CliArgs {
    match CliArgs::try_parse() {
        Ok(args) => args,
        Err(err) => {
            let dialog = tg_iced::dialog::MessageDialog::new(500, 200)
                .set_level(tg_iced::dialog::MessageLevel::Error)
                .set_title(CLI_ERROR_TITLE)
                .set_description(format!("{CLI_ERROR_PREFIX}{err}"));
            backend.show_cli_error(dialog);
            err.exit();
        }
    }
}

/// `settings_file_path`（源行 166，内联）：`paths::config().join("settings.ron")`。
pub fn settings_file_path() -> PathBuf {
    paths::config().join(SETTINGS_FILE)
}

/// `get_app_config` `0x140353990` —— `Result<GameConfig, anyhow::Error>`。
///
/// 读/解析失败静默回落 `GameConfig::default()`；仅在初始化源缺失或复制失败时
/// 返回错误。之后按 CLI 覆盖 `dev.skip_splash/skip_welcome` 并 `upgrade()`。
pub fn get_app_config(cli: &CliArgs) -> anyhow::Result<GameConfig> {
    let path = settings_file_path();
    if !path.exists() {
        let source = paths::installed().join(DEFAULT_SETTINGS_FILE);
        if !source.exists() {
            return Err(anyhow::anyhow!(
                "Unable to initialize settings: {} does not exist.",
                source.display()
            ));
        }
        std::fs::copy(&source, &path).with_context(|| {
            format!(
                "Unable to initialize settings: {} does not exist.",
                source.display()
            )
        })?;
        if log::max_level() > log::LevelFilter::Warn {
            log::info!(
                "Initialized settings {} -> {}",
                source.display(),
                path.display()
            );
        }
    }

    let mut config = match std::fs::read_to_string(&path) {
        Ok(text) => ron::from_str::<GameConfig>(&text).unwrap_or_default(),
        Err(_) => GameConfig::default(),
    };

    if cli.skip_splash || cli.repro_panic.is_some() {
        config.dev.skip_splash = true;
        config.dev.skip_welcome = true;
    }
    config.upgrade();
    Ok(config)
}

/// `write_game_config_to_disk` `0x140354240`。
///
/// 序列化失败是 **`unwrap()` panic**（原版 `core::result::unwrap_failed`）；
/// 写失败只记 `log::error!`，不返回错误。
pub fn write_game_config_to_disk(config: &GameConfig) {
    let path = settings_file_path();
    let text = ron::Options::default()
        .to_string_pretty(config, ron::ser::PrettyConfig::default())
        .unwrap();
    if let Err(err) = std::fs::write(&path, text) {
        log::error!(
            "Failed to write settings to path {}: {}",
            path.display(),
            err
        );
    }
}

/// `write_settings_file` `0x140354d70` —— 纯逻辑体。
///
/// B 的调度形态是 Bevy system `fn(Res<GameConfig>, Res<PanicReproState>)`；
/// 共享 `PanicReproState` 尚无 `Resource` derive（不属于本模块，未改），
/// 故以引用签名落地，另见 [`write_settings_file_system`] 适配器。
///
/// `!PanicReproState::is_inactive()` 时只记 info（panic repro 覆盖生效，不写盘）；
/// 否则写配置。
pub fn write_settings_file(config: &GameConfig, panic_repro_state: &PanicReproState) {
    let path = settings_file_path();
    if !panic_repro_state.is_inactive() {
        if log::max_level() >= log::LevelFilter::Info {
            log::info!(
                "Panic repro settings override in action; NOT saving {}",
                path.display()
            );
        }
    } else {
        write_game_config_to_disk(config);
    }
}

/// `PanicReproState` 的 Bevy 资源包装（共享类型缺 `Resource` derive 的 seam）。
#[derive(Resource, Debug, Default)]
pub struct PanicReproStateResource(pub PanicReproState);

/// `write_settings_file` 的 Bevy system 适配（B 直接使用 `Res<PanicReproState>`）。
pub fn write_settings_file_system(
    config: Res<GameConfig>,
    panic_repro_state: Res<PanicReproStateResource>,
    launch_display: Option<Res<LaunchDisplayOverride>>,
) {
    if let Some(policy) = launch_display {
        write_settings_file(&policy.persistence_config(&config), &panic_repro_state.0);
    } else {
        write_settings_file(&config, &panic_repro_state.0);
    }
}

/// `VulkanLayerAllowlistDefaults`（`.rdata 0x1429b59ae`：字段 `version`,`add`）。
#[derive(Debug, Clone, serde::Deserialize)]
pub struct VulkanLayerAllowlistDefaults {
    pub version: u32,
    pub add: Vec<GraphicsInstanceLayerAllowlistEntry>,
}

/// `update_vulkan_allowlist_defaults` `0x140354650`。
pub fn update_vulkan_allowlist_defaults(config: &mut GameConfig) {
    let path = paths::installed().join(VULKAN_ALLOWLIST_DEFAULTS_FILE);
    let text = match std::fs::read_to_string(&path) {
        Ok(text) => text,
        Err(err) => {
            if log::max_level() != log::LevelFilter::Off {
                log::error!("Error reading {}: {}", path.display(), err);
            }
            return;
        }
    };
    let defaults = match ron::from_str::<Vec<VulkanLayerAllowlistDefaults>>(&text) {
        Ok(defaults) => defaults,
        Err(err) => {
            if log::max_level() != log::LevelFilter::Off {
                log::error!("Error deserializing Vulkan allowlist defaults: {}", err);
            }
            return;
        }
    };

    let allowlist: &mut GraphicsInstanceLayerAllowlist = &mut config.video.vulkan_layer_allowlist;
    let old_len = allowlist.entries.len();
    let mut max_version = 0u32;
    let new_items: Vec<_> = defaults
        .into_iter()
        .filter(|defaults| {
            max_version = max_version.max(defaults.version);
            allowlist.version < defaults.version
        })
        .flat_map(|defaults| defaults.add)
        .collect();
    allowlist.entries.extend(new_items);
    allowlist.version = allowlist.version.max(max_version);

    // 原版稳定排序：同变体按字符串比较，跨变体按判别式（0=Layer,1=Vendor），
    // 与 `derive(Ord)` 的派生顺序一致（反汇编比较器 0x1405c1000：
    // 先比 tag，再 memcmp(min(len)) 后比 len）。rhapsody 的镜像类型未派生
    // Ord，故在此显式给出同一比较器。
    allowlist.entries.sort_by(|a, b| match (a, b) {
        (
            GraphicsInstanceLayerAllowlistEntry::Layer(left),
            GraphicsInstanceLayerAllowlistEntry::Layer(right),
        )
        | (
            GraphicsInstanceLayerAllowlistEntry::Vendor(left),
            GraphicsInstanceLayerAllowlistEntry::Vendor(right),
        ) => left.cmp(right),
        (
            GraphicsInstanceLayerAllowlistEntry::Layer(_),
            GraphicsInstanceLayerAllowlistEntry::Vendor(_),
        ) => core::cmp::Ordering::Less,
        (
            GraphicsInstanceLayerAllowlistEntry::Vendor(_),
            GraphicsInstanceLayerAllowlistEntry::Layer(_),
        ) => core::cmp::Ordering::Greater,
    });
    allowlist.entries.dedup_by(|a, b| a == b);

    if old_len != allowlist.entries.len() && log::max_level() > log::LevelFilter::Warn {
        log::info!("Vulkan allowlist defaults have been updated.");
    }
    write_game_config_to_disk(config);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cli() -> CliArgs {
        CliArgs::default()
    }

    #[test]
    fn input_recording_prefers_record_then_replay() {
        let mut c = cli();
        assert_eq!(c.to_input_recording_opts(), None);
        c.replay_input = Some(PathBuf::from("replay.bin"));
        assert_eq!(
            c.to_input_recording_opts(),
            Some(InputRecordingOpts::Replay(PathBuf::from("replay.bin")))
        );
        c.record_input = Some(PathBuf::from("record.bin"));
        assert_eq!(
            c.to_input_recording_opts(),
            Some(InputRecordingOpts::Record(PathBuf::from("record.bin")))
        );
    }

    #[test]
    fn explicit_windowed_overrides_fullscreen_environment_and_large_saved_resolution() {
        let cli = CliArgs::try_parse_from(["tiny-glade", "--windowed"]).unwrap();
        let mut config = GameConfig::default();
        config.video.display.fullscreen = true;
        config.video.display.allow_exclusive_fullscreen = true;
        config.video.display.windowed_resolution = [3840, 2160];
        let policy = LaunchDisplayOverride::resolve(
            &cli, &config.video.display, false, true, Some("4096x2160"),
        ).unwrap().unwrap();
        policy.apply(&mut config.video.display);
        assert!(!config.video.display.fullscreen);
        assert!(!config.video.display.allow_exclusive_fullscreen);
        assert_eq!(config.video.display.windowed_resolution, [960, 540]);
        // Applying a later video setting must obey the same explicit policy.
        config.video.display.fullscreen = true;
        policy.apply(&mut config.video.display);
        assert!(!config.video.display.fullscreen);
    }

    #[test]
    fn launch_size_is_explicit_windowed_and_rejects_invalid_extents() {
        let cli = CliArgs::try_parse_from(["tiny-glade", "--window-size", "480x270"]).unwrap();
        let display = country_core::render::render_config::DisplayConfig::default();
        let policy = LaunchDisplayOverride::resolve(&cli, &display, true, true, Some("bad")).unwrap().unwrap();
        let mut actual = display;
        policy.apply(&mut actual);
        assert!(!actual.fullscreen);
        assert_eq!(actual.windowed_resolution, [480, 270]);
        for invalid in ["0x540", "960x0", "-1x540", "960X540", "960x540x1", "4294967296x1"] {
            assert!(CliArgs::try_parse_from(["tiny-glade", "--window-size", invalid]).is_err());
        }
    }

    #[test]
    fn legacy_standalone_window_size_stays_supported() {
        let cli = CliArgs::default();
        let display = country_core::render::render_config::DisplayConfig::default();
        let policy = LaunchDisplayOverride::resolve(&cli, &display, true, false, Some("480x270")).unwrap().unwrap();
        let mut actual = display.clone();
        policy.apply(&mut actual);
        assert!(!actual.fullscreen);
        assert_eq!(actual.windowed_resolution, [480, 270]);
        assert!(LaunchDisplayOverride::resolve(&cli, &display, false, false, Some("bad")).unwrap().is_none());
        assert!(LaunchDisplayOverride::resolve(&cli, &display, true, false, Some("0x1")).is_err());
    }

    #[test]
    fn temporary_display_override_preserves_other_settings_on_save() {
        let cli = CliArgs::try_parse_from(["tiny-glade", "--window-size", "480x270"]).unwrap();
        let mut config = GameConfig::default();
        let original = config.video.display.clone();
        let policy = LaunchDisplayOverride::resolve(&cli, &original, true, false, None).unwrap().unwrap();
        policy.apply(&mut config.video.display);
        config.dev.skip_welcome = true;
        // A real monitor change and unrelated preferences remain writable.
        config.video.display.monitor = Some(country_core::render::render_config::MonitorMatch {
            name: Some("Runtime monitor".to_owned()),
            physical_position: [960, 0],
            physical_size: [1920, 1080],
        });
        let saved = policy.persistence_config(&config);
        assert_eq!(saved.video.display.fullscreen, original.fullscreen);
        assert_eq!(saved.video.display.allow_exclusive_fullscreen, original.allow_exclusive_fullscreen);
        assert_eq!(saved.video.display.windowed_resolution, original.windowed_resolution);
        assert_eq!(saved.video.display.monitor, config.video.display.monitor);
        assert!(saved.dev.skip_welcome);
        assert_eq!(config.video.display.windowed_resolution, [480, 270]);
        assert!(!config.video.display.fullscreen);
    }
}
