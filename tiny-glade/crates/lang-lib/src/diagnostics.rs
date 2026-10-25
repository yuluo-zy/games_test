//! Call application loggers without recursively emitting localization warnings.
use std::cell::Cell;

thread_local! {
    static LOGGING: Cell<bool> = const { Cell::new(false) };
}

struct LoggingGuard<'a>(&'a Cell<bool>);

impl Drop for LoggingGuard<'_> {
    fn drop(&mut self) {
        self.0.set(false);
    }
}

pub(crate) fn warn(log: impl FnOnce()) {
    LOGGING.with(|logging| {
        if logging.replace(true) {
            return;
        }
        // A logger may look up text, reload a pack, or panic. Restore the guard
        // during unwinding and keep all localization warning paths protected.
        let _guard = LoggingGuard(logging);
        log();
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_panicking_logger_does_not_disable_future_diagnostics() {
        assert!(std::panic::catch_unwind(|| warn(|| panic!("logger failed"))).is_err());
        let called = Cell::new(false);
        warn(|| {
            called.set(true);
            warn(|| panic!("recursive warning must be suppressed"));
        });
        assert!(called.get());
    }
}
