//! B `crates/tiny-glade/src/main.rs`（进程入口 / 崩溃上报 mock / 首帧计时）。
//!
//! 恢复依据（fresh 反编译，Ghidra shared 2026-10-17；证据在
//! `recovery/agent-team/work/restorer-batch4-A/`）：
//! - `tiny_glade::main` `0x1403db340`（10,129 B）
//! - `tiny_glade::log_time_to_first_frame` `0x1403db200`（312 B）
//! - `tiny_glade::mock_crash_reporter` `0x1403df140`（681 B）
//!
//! ## 已恢复
//! `log_time_to_first_frame` 与其在 `main` 中写入的 `STARTUP_INSTANT` 全局
//! （`0x143634b88` = `Instant.secs`，`0x14362b0a8` = `Instant.nanos`，
//! `nanos == 1_000_000_000` 为 `None` niche）逐指令判等。
//!
//! Startup order is now backed by untouched B RNE disassembly in task
//! TG-135f736ac5c9d1ccab/main-chain-boot-20261018/evidence/main-rne.asm.
//! Steam initialization failure remains fatal in `main`; an embedding with a
//! real SDK can call `main_with_steam`. `standalone_main` is explicitly a local
//! host adapter. Minidumper/Sentry and the deleted configuration/report GUIs
//! remain explicit platform gaps; none are represented by fake success.
//!
//! ## 文件名说明
//! This file remains a library module; explicit binaries live in `src/bin/`.

use std::{cell::UnsafeCell, time::Instant};

/// `main` 中 `STARTUP_INSTANT` 的两个全局写入（O：`0x143634b88` / `0x14362b0a8`）。
///
/// 原版用非原子 `MOV` 写 `Instant` 的两个字段；这里用 `UnsafeCell` 保留
/// 同样的单线程/非同步语义与字段布局（`Option<Instant>` 的 niche 也一致）。
struct StartupInstantCell(UnsafeCell<Option<Instant>>);
// SAFETY: 与原版一致——仅在进程启动（main）与首帧（game loop）各访问一次，
// 两者有 happens-before（同一线程/初始化屏障），不存在并发读写。
unsafe impl Sync for StartupInstantCell {}

static STARTUP_INSTANT: StartupInstantCell = StartupInstantCell(UnsafeCell::new(None));

/// 对应 `tiny_glade::main` 中 `0x1403db372`/`0x1403db379` 的两条 `MOV`
/// （`main` 在调用 `run_game` 前写入 `Instant::now()`）。
pub fn set_startup_instant(instant: Instant) {
    // SAFETY: 见 `StartupInstantCell` 的注释。
    unsafe {
        *STARTUP_INSTANT.0.get() = Some(instant);
    }
}

/// `tiny_glade::log_time_to_first_frame` `0x1403db200`（判等）。
///
/// 反编译逐语句：
/// 1. `old_nanos = STARTUP_INSTANT.nanos; STARTUP_INSTANT.nanos = 1e9`（`take`）；
/// 2. `if old_nanos != 1e9`（`Some`）；
/// 3. `if 2 < MAX_LOG_LEVEL_FILTER`（`LevelFilter::Warn < max_level()`，即 Info 及以上）；
/// 4. `elapsed = old_instant.elapsed()`；
/// 5. `log::log!(target: "tiny_glade", Level::Info, "time to first frame: {:?}", elapsed)`。
///
/// 文案 `"time to first frame: "` 共 21 字节（O：长度 `0x15`，见
/// `0x1429d39f8` 的 `{ptr,0x15}` 元组）；target 为 `"tiny_glade"`（长度 10，
/// O：`0x1429d3a40`）。
pub fn log_time_to_first_frame() {
    // SAFETY: 见 `StartupInstantCell` 的注释。
    let instant = unsafe { (*STARTUP_INSTANT.0.get()).take() };
    if let Some(instant) = instant {
        if log::max_level() > log::LevelFilter::Warn {
            log::info!(
                target: "tiny_glade",
                "time to first frame: {:?}",
                instant.elapsed()
            );
        }
    }
}

/// B process entry. Steam failure is fatal in the original normal launch
/// branch; do not silently turn a missing SDK into a successful offline game.
/// A platform embedding supplies the actual initialized SDK through
/// [`main_with_steam`]. The explicit `tiny-glade-host` binary is a separate
/// macOS host adapter and does not claim this Steam initialization was restored.
pub fn main() -> anyhow::Result<()> {
    set_startup_instant(Instant::now());
    utils::local_time::init_local_time();
    let cli = crate::app_config::get_cli_args();
    anyhow::ensure!(
        cli.crash_reporter_server.is_none(),
        "Crash-reporter server requires the original minidumper platform backend"
    );
    // main.c's ordinary branch calls AppSteamIntegration::try_new before
    // initialize_paths. Its Err branch returns an error (not an offline mode).
    let _steam = country_core::systems::steam::AppSteamIntegration::try_new(|_| None)?;
    unreachable!("the default build has no Steam SDK factory")
}

/// Original successful launch branch with a real platform-owned Steam client.
/// `language` is ISteamApps::GetCurrentGameLanguage, not a guessed OS locale.
pub fn main_with_steam(
    steam: country_core::systems::steam::AppSteamIntegration,
    language: String,
) -> anyhow::Result<()> {
    set_startup_instant(Instant::now());
    utils::local_time::init_local_time();
    let cli = crate::app_config::get_cli_args();
    launch(cli, Some((steam, language)))
}

/// Explicit local host adapter: same recovered configuration/language/window/
/// App/render path, without claiming the original Steam SDK is present.
pub fn standalone_main() -> anyhow::Result<()> {
    set_startup_instant(Instant::now());
    utils::local_time::init_local_time();
    launch(crate::app_config::get_cli_args(), None)
}

fn launch(
    cli: crate::app_config::CliArgs,
    steam: Option<(country_core::systems::steam::AppSteamIntegration, String)>,
) -> anyhow::Result<()> {
    use anyhow::Context;
    anyhow::ensure!(
        cli.crash_reporter_server.is_none(),
        "Crash-reporter server requires the original minidumper platform backend"
    );
    let installation = crate::run_game::installation_root()?;
    let user_id = cli
        .override_save_root_user_id
        .or_else(|| steam.as_ref().map(|(integration, _)| integration.user_id()))
        // Account zero is exclusively the explicit standalone-host account.
        .unwrap_or(0);
    paths::initialize_paths_with_installation(
        user_id,
        cli.override_save_root.as_deref(),
        &installation,
    )?;
    // The original assumes installed assets relative to its launch directory.
    // The local host accepts an explicit installation root instead of copying
    // or synthesizing packaged assets next to target/debug.
    std::env::set_current_dir(&installation)
        .with_context(|| format!("Setting installation directory {}", installation.display()))?;
    let mut config = crate::app_config::get_app_config(&cli)?;
    crate::logging::set_up_logging(log::LevelFilter::Info, true)?;
    if config.dev.capture_crashes {
        // main's spawn_server error branch logs and continues. No fake client
        // or child process is created when minidumper is unavailable.
        log::error!(
            "Crash reporter failed to initialize: original minidumper transport is unavailable in this host"
        );
    }
    crate::app_config::update_vulkan_allowlist_defaults(&mut config);
    // The original writes MVK_CONFIG_USE_METAL_ARGUMENT_BUFFERS=1 here
    // (@0x1403dd564..0x1403dd57d). Rust 2024 requires it before threads;
    // both binary adapters publish it at process entry, before calling us.
    if cli.repro_panic.is_none() {
        crate::panic_reporter::setup_panic_hook();
    }
    struct LanguageSource<'a>(&'a str);
    impl crate::load_language_library::SteamAppsLanguage for LanguageSource<'_> {
        fn current_game_language(&self) -> String {
            self.0.to_owned()
        }
    }
    let language = steam.as_ref().map(|(_, language)| LanguageSource(language));
    crate::load_language_library::load_language_library(
        language
            .as_ref()
            .map(|source| source as &dyn crate::load_language_library::SteamAppsLanguage),
        config.lang_override.as_deref(),
    );
    country_core::game_config::set_developer_hotkeys_enabled(config.dev.internal_hotkeys);
    rhapsody::init_logging();
    if cli.verify_integrity {
        // Execute the real verification and propagate failures. The deleted
        // original report GUI is not represented by an empty GUI backend.
        tg_package_integrity::verify_package_manifest(&installation)
            .map_err(|error| anyhow::anyhow!("Package integrity verification failed: {error:?}"))?;
        return Ok(());
    }
    anyhow::ensure!(
        !cli.configure_vulkan,
        "The original Vulkan configuration GUI backend has not been recovered"
    );
    anyhow::ensure!(
        cli.mock_crash_reporter.is_none(),
        "The original mock crash reporter GUI backend has not been recovered"
    );
    crate::run_game::run_game(
        config,
        cli,
        steam.map(|(integration, _)| integration.client),
    )
}

/// `tiny_glade::mock_crash_reporter` `0x1403df140` —— 未实现（crate 已删除）。
pub fn mock_crash_reporter() -> anyhow::Result<()> {
    anyhow::bail!("The original crash_reporter_gui backend has not been recovered")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn take_is_idempotent() {
        set_startup_instant(Instant::now());
        log_time_to_first_frame();
        // 第二次 take 已经是 None，不 panic。
        log_time_to_first_frame();
    }
}
