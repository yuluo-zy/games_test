//! `tiny_glade::panic_reporter::reverse_backtrace`.
//!
//! B `reverse_backtrace` is a formatting adapter: the panic hook logs
//! `"Reverse backtrace:\n{...}"` with a `backtrace::Backtrace` (the `backtrace`
//! crate, `backtrace::capture::Backtrace::new()` at `0x140043000`) and the
//! `impl$0::fmt` formatter (`0x1402cfc30` `call_once` wrapper → closure body
//! `0x1402d0590`). The recovered body is pure formatting over the captured
//! frames; the frame data itself comes from the backtrace dependency.

use std::fmt;

/// Backtrace wrapper that renders frames in reverse order (innermost first).
pub struct ReverseBacktrace<'a>(pub &'a backtrace::Backtrace);

impl fmt::Debug for ReverseBacktrace<'_> {
    // 构建门修复（restorer-tinyglade-E）：panic hook 以 `{:?}` 记录该适配器；
    // 原 `impl$0::fmt` 的行反转语义保留在 Debug 上。
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, formatter)
    }
}

impl fmt::Display for ReverseBacktrace<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        // The formatter walks the frame list in reverse (the closure that renders
        // a single frame is `reverse_backtrace::impl$0::fmt::closure_0`); the
        // `Debug` rendering of the crate is one frame per line, so reversing the
        // lines is the same textual result.
        let rendered = format!("{:?}", self.0);
        let mut lines = rendered.lines().collect::<Vec<_>>();
        lines.reverse();
        for line in lines {
            writeln!(formatter, "{line}")?;
        }
        Ok(())
    }
}
