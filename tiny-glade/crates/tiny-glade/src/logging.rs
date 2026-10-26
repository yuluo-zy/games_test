//! B `crates/tiny-glade/src/logging.rs`（fern 日志初始化 / 时间格式）。
//!
//! 恢复依据（fresh Ghidra shared 2026-10-17，证据在
//! `recovery/agent-team/work/restorer-tinyglade-C/`）：
//! - `formatted_date_time`（源行 11，内联于 closure$1）
//! - `formatted_time`（源行 30，内联于 closure$0）
//! - `set_up_logging` `0x1402e4760`（2,253 B）
//! - `set_up_logging::closure$0` `0x1402e53f0`（stdout 行格式，用 `formatted_time`）
//! - `set_up_logging::closure$1` `0x1402e5740`（文件行格式，用 `formatted_date_time`）
//! - `set_up_logging::closure$2` `0x1402e5a00` 区域（源行 123）
//!
//! ## 逐语句复原
//! ```text
//! now = OffsetDateTime::now_utc()
//! offset = UtcOffset::local_offset_at(now)         // Ok 时存全局，供格式化复用
//! colored::control::set_virtual_terminal(true)     // 反汇编 mov cl,1
//! dispatch = fern::Dispatch::new()
//!     .level(<参数1 LevelFilter>)                   // main 传 3 = Info
//!     .format(|out, message, record| {
//!         out.finish(format_args!(
//!             "{} {} [{}] [frame:{}] {}",
//!             formatted_time(), record.level(), record.target(), TICK_IDX, message))
//!     })
//!     .chain(std::io::stdout())
//! if <参数2 bool> {
//!     dispatch = dispatch
//!         .level_for("bevy_app", Warn)
//!         .level_for("bevy_ecs", Warn)
//!         .level_for("hotwatch", Warn)
//!         .level_for("cosmic_text", Warn)
//!         .level_for("gpu_allocator", Debug)        // 栈参数 2/2/2/2/4
//!     file = OpenOptions::new().write(true).create(true).truncate(true)
//!         .open(paths::tmp().join("log.txt")).unwrap()   // 失败 unwrap panic
//!     dispatch = dispatch.chain(fern::Dispatch::new().format(<date_time 闭包>).chain(file))
//! }
//! dispatch.apply().map_err(|e| anyhow!("Failed setting up the crash reporter logger: {}", e))
//! ```
//!
//! ## 时间格式（`.rdata` 的 `BorrowedFormatItem` 数组逐项解码，24 B/项）
//! - `formatted_time`（本地偏移可用）：`[hour]:[minute]:[second].[subsecond
//!   digits:3]`（7 项）。
//! - `formatted_time`（UTC 回落）：同串 + 末尾 1 字节字面量 `\u{1}`
//!   （8 项；`.rdata 0x1429b4a98` 指向字节 0x01，已用同版本 time 复现字节一致）。
//! - `formatted_date_time`（本地偏移可用）：
//!   `[year]-[month]-[day]T[hour]:[minute]:[second].[subsecond digits:3]
//!    [offset_hour sign:mandatory]:[offset_minute]`（16 项）。
//! - `formatted_date_time`（UTC 回落）：同串以字面量 `Z` 结尾（14 项）。
//! 四条格式描述均与 `time 0.3.36` 的 `parse_borrowed::<2>` 字节逐一吻合。
//!
//! ## 胶水/缝
//! - fern/colored/std::fs 调用为平台/依赖胶水，按原调用顺序接线。
//! - B 的 `country_core::systems::TICK_IDX` 静态未在共享 crate 落库；
//!   本文件提供同布局的 `TICK_IDX` 镜像（AtomicU32），落地后应改为
//!   `pub use country_core::systems::TICK_IDX`。
//! - 文件打开的三个 bool 由 Windows `OpenOptions` 布局（3 个置 1）判定为
//!   `write + create + truncate`（等价 `File::create`）；share_mode=7 为默认。

use std::sync::atomic::{AtomicU32, Ordering};

use time::format_description::BorrowedFormatItem;
use time::macros::format_description;
use time::{OffsetDateTime, UtcOffset};

/// B `country_core::systems::TICK_IDX` 的镜像 seam（帧计数，u32）。
pub static TICK_IDX: AtomicU32 = AtomicU32::new(0);

/// 本地时区偏移（`set_up_logging` 时探测；`None` = UTC）。
static LOCAL_OFFSET: std::sync::OnceLock<UtcOffset> = std::sync::OnceLock::new();

fn local_offset() -> Option<UtcOffset> {
    LOCAL_OFFSET.get().copied()
}

/// `[hour]:[minute]:[second].[subsecond digits:3]`（7 项，本地）。
const TIME_FORMAT: &[BorrowedFormatItem<'_>] =
    format_description!("[hour]:[minute]:[second].[subsecond digits:3]");

/// 同串 + `\u{1}`（8 项，UTC 回落；原文末尾 1 字节字面量）。
const TIME_FORMAT_UTC: &[BorrowedFormatItem<'_>] = format_description!(
    "[hour]:[minute]:[second].[subsecond digits:3]\u{1}"
);

/// 日期时间（16 项，本地偏移）。
const DATE_TIME_FORMAT: &[BorrowedFormatItem<'_>] = format_description!(
    "[year]-[month]-[day]T[hour]:[minute]:[second].[subsecond digits:3][offset_hour sign:mandatory]:[offset_minute]"
);

/// 日期时间（14 项，UTC）。
const DATE_TIME_FORMAT_UTC: &[BorrowedFormatItem<'_>] = format_description!(
    "[year]-[month]-[day]T[hour]:[minute]:[second].[subsecond digits:3]Z"
);

/// `formatted_time`（源行 30）：本地偏移可用则转到本地，否则 UTC。
fn formatted_time() -> String {
    let now = OffsetDateTime::now_utc();
    match local_offset() {
        Some(offset) => now.to_offset(offset).format(TIME_FORMAT).unwrap(),
        None => now.format(TIME_FORMAT_UTC).unwrap(),
    }
}

/// `formatted_date_time`（源行 11）：同上，带日期与偏移。
fn formatted_date_time() -> String {
    let now = OffsetDateTime::now_utc();
    match local_offset() {
        Some(offset) => now.to_offset(offset).format(DATE_TIME_FORMAT).unwrap(),
        None => now.format(DATE_TIME_FORMAT_UTC).unwrap(),
    }
}

/// `set_up_logging` `0x1402e4760` —— `Result<(), anyhow::Error>`。
///
/// `level` = main 传入的 `LevelFilter`（`0x1403dd06d`：`ecx=3` = Info）；
/// `file_logging` = 第二参数 bool（`in_DL`）。
struct RenderLog;
impl log::Log for RenderLog {
    fn enabled(&self, _: &log::Metadata<'_>) -> bool { true }
    fn log(&self, record: &log::Record<'_>) {
        rhapsody::record_log(format_args!("{}", record.args()));
    }
    fn flush(&self) {}
}

pub fn set_up_logging(
    level: log::LevelFilter,
    file_logging: bool,
) -> anyhow::Result<()> {
    let now = OffsetDateTime::now_utc();
    if let Ok(offset) = UtcOffset::local_offset_at(now) {
        let _ = LOCAL_OFFSET.set(offset);
    }

    // `mov cl,1; call colored::control::set_virtual_terminal`（colored 1.9.3
    // 仅在 Windows 构建提供该函数；平台胶水）。
    #[cfg(windows)]
    {
        let _ = colored::control::set_virtual_terminal(true);
    }

    rhapsody::init_logging();
    let mut dispatch = fern::Dispatch::new()
        .level(level)
        .format(|out, message, record| {
            out.finish(format_args!(
                "{} {} [{}] [frame:{}] {}",
                formatted_time(),
                record.level(),
                record.target(),
                TICK_IDX.load(Ordering::Relaxed),
                message
            ))
        })
        .chain(std::io::stdout())
        .chain(Box::new(RenderLog) as Box<dyn log::Log>);

    if file_logging {
        dispatch = dispatch
            .level_for("bevy_app", log::LevelFilter::Warn)
            .level_for("bevy_ecs", log::LevelFilter::Warn)
            .level_for("hotwatch", log::LevelFilter::Warn)
            .level_for("cosmic_text", log::LevelFilter::Warn)
            .level_for("gpu_allocator", log::LevelFilter::Debug);

        // `.rdata 0x1429b4d10` = "log.txt"（len 7）。
        let file = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(paths::tmp().join("log.txt"))
            .unwrap();

        dispatch = dispatch.chain(
            fern::Dispatch::new()
                .format(|out, message, record| {
                    out.finish(format_args!(
                        "{} {} [{}] [frame:{}] {}",
                        formatted_date_time(),
                        record.level(),
                        record.target(),
                        TICK_IDX.load(Ordering::Relaxed),
                        message
                    ))
                })
                .chain(file),
        );
    }

    dispatch
        .apply()
        .map_err(|e| anyhow::anyhow!("Failed setting up the crash reporter logger: {e}"))
}
