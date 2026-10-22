//! `tiny_glade::hang_watchdog` — main-thread hang detector.
//!
//! Recovered VAs: `hang_watchdog` `0x140365e20`,
//! `spawn_hang_watchdog_thread` `0x140365be0`.
//!
//! Machine-code constants / strings:
//! * thread name bytes `"hang wat"` + `"chdog"` → `"hang watchdog"`;
//! * watchdog interval `20` s (`mov r8d, 0x14` into `Instant::add`);
//! * poll step `1` s (`mov ecx, 1` into `thread::sleep`);
//! * `"The frame index has not changed in {:?}"` (`0x1429c5968`);
//! * `"Deadlock detected"` (`0x1429c5830`);
//! * `"🛑 Deadlock :"` (`0x1429c58c0`), `"Thread "`/`" backtrace:\n"`
//!   (`0x1429c5908`/`0x1429c5918`);
//! * `"🔥 Triggering an intentional crash..."` (`0x1429c5878`);
//! * `"hang_watchdog thread priority successfully set"` (`0x1429c5738`) and
//!   `"Error setting priority of hang_watchdog thread: "` (`0x1429c57d1`);
//! * on a detected hang the original logs and executes `int 0x29` (fail-fast).
//!
//! `country_core::systems::TICK_IDX` is not present in the recovered
//! `country-core` crate; the equivalent atomic is defined here and must be
//! bumped by the game loop. B reads it as a 32-bit `int`.

use std::sync::atomic::{AtomicI32, Ordering};
use std::time::{Duration, Instant};

/// Watchdog period (`0x14` seconds).
pub const WATCHDOG_INTERVAL: Duration = Duration::from_secs(20);
/// Poll step inside the period (`1` second).
pub const WATCHDOG_POLL: Duration = Duration::from_secs(1);
/// Log target used by every watchdog message (`0x1429c57b8`, len 0x1a).
const TARGET: &str = "tiny_glade::hang_watchdog";

/// Frame counter observed by the watchdog.
///
/// B: `country_core::systems::TICK_IDX` (`0x143634d8c`), read as `int`.
pub static TICK_IDX: AtomicI32 = AtomicI32::new(0);

/// Advance the frame counter (called once per rendered frame).
pub fn tick() {
    TICK_IDX.fetch_add(1, Ordering::Relaxed);
}

/// B `tiny_glade::hang_watchdog::hang_watchdog` (`0x140365e20`).
///
/// ```text
/// let mut frame_index = TICK_IDX;
/// let mut deadline = Instant::now() + 20s;
/// loop {
///     while Instant::now() < deadline { sleep(1s); }
///     if let Some(deadlocks) = check_deadlock() {
///         if !deadlocks.is_empty() {
///             log "Deadlock detected"; log each group/thread backtrace;
///             log "🔥 Triggering an intentional crash...";
///             fail_fast();
///         }
///     }
///     if TICK_IDX == frame_index {
///         log "The frame index has not changed in {20s}";
///         log "🔥 Triggering an intentional crash...";
///         fail_fast();
///     }
///     deadline = Instant::now() + 20s;
///     frame_index = TICK_IDX;
/// }
/// ```
///
/// The original is `-> !` because both terminal paths call `int 0x29`; Rust's
/// `std::process::abort()` is the same `__fastfail(7)` on Windows.
pub fn hang_watchdog() -> ! {
    set_thread_priority();

    let mut frame_index = TICK_IDX.load(Ordering::Relaxed);
    let mut deadline = Instant::now() + WATCHDOG_INTERVAL;
    loop {
        loop {
            std::thread::sleep(WATCHDOG_POLL);
            if Instant::now() >= deadline {
                break;
            }
        }

        #[cfg(feature = "deadlock-detection")]
        report_deadlocks();

        let current = TICK_IDX.load(Ordering::Relaxed);
        if current == frame_index {
            log::error!(
                target: TARGET,
                "The frame index has not changed in {:?}",
                WATCHDOG_INTERVAL
            );
            log::info!(target: TARGET, "🔥 Triggering an intentional crash...");
            std::process::abort();
        }

        frame_index = current;
        deadline = Instant::now() + WATCHDOG_INTERVAL;
    }
}

/// Deadlock half of the watchdog tick (`deadlock-detection` builds only).
#[cfg(feature = "deadlock-detection")]
fn report_deadlocks() {
    // parking_lot 的 `deadlock_detection` 特性下 `check_deadlock()` 返回
    // `Vec<Vec<DeadlockedThread>>`（旧版曾返回 `Option<_>`）。
    let deadlocks = parking_lot::deadlock::check_deadlock();
    if !deadlocks.is_empty() {
        log::error!(target: TARGET, "Deadlock detected");
        for (index, threads) in deadlocks.iter().enumerate() {
            log::error!(target: TARGET, "🛑 Deadlock :{}", index);
            for (thread_index, thread) in threads.iter().enumerate() {
                log::error!(
                    target: TARGET,
                    "Thread {} backtrace:\n{:?}",
                    thread_index,
                    thread.backtrace()
                );
            }
        }
        log::info!(target: TARGET, "🔥 Triggering an intentional crash...");
        std::process::abort();
    }
}

/// Priority setup is platform glue (`thread_priority` crate in B).
///
/// `ThreadPriority::set_for_current` returns `Ok` → info log
/// `"hang_watchdog thread priority successfully set"`; `Err` → error log
/// `"Error setting priority of hang_watchdog thread: {error}"`. The crate is not
/// part of the reconstructed workspace, so no call is made.
fn set_thread_priority() {
    // blocker: thread_priority dependency not restored in this workspace.
}

/// B `spawn_hang_watchdog_thread` (`0x140365be0`).
///
/// `thread::Builder::new().name("hang watchdog".into()).spawn(hang_watchdog)`.
pub fn spawn_hang_watchdog_thread() -> std::io::Result<std::thread::JoinHandle<()>> {
    std::thread::Builder::new()
        .name("hang watchdog".to_string())
        .spawn(|| hang_watchdog())
}
