//! Formatting uses a cached fixed UTC offset, initialized explicitly at startup.
//!
//! Before initialization, or when the platform cannot discover an offset, UTC
//! is used. Refresh after timezone/DST changes. Historical timestamps also use
//! this cached offset; this is not a timezone database or historical DST lookup.
use std::{
    sync::atomic::{AtomicI32, Ordering},
    time::SystemTime,
};
use time::{OffsetDateTime, UtcOffset};

// One independent scalar: readers do not need to synchronize any other state.
static LOCAL_OFFSET_SECONDS: AtomicI32 = AtomicI32::new(0);

/// Refresh the fixed offset from the current local time, falling back to UTC.
pub fn init_local_time() {
    let offset = UtcOffset::local_offset_at(OffsetDateTime::now_utc()).unwrap_or(UtcOffset::UTC);
    LOCAL_OFFSET_SECONDS.store(offset.whole_seconds(), Ordering::Relaxed);
}

/// Convert an instant using the latest cached offset without taking a lock.
///
/// # Panics
/// Panics if `time` is outside the date range supported by the `time` crate.
pub fn system_time_to_local_time(time: SystemTime) -> OffsetDateTime {
    let offset = UtcOffset::from_whole_seconds(LOCAL_OFFSET_SECONDS.load(Ordering::Relaxed))
        .expect("cached offset was validated before publication");
    OffsetDateTime::from(time).to_offset(offset)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn cached_utc_conversion_preserves_instants_and_subseconds() {
        for instant in [
            SystemTime::UNIX_EPOCH,
            SystemTime::UNIX_EPOCH - Duration::new(1, 250_000_000),
            SystemTime::UNIX_EPOCH + Duration::new(1_700_000_000, 123_456_789),
        ] {
            let converted = system_time_to_local_time(instant);
            assert_eq!(SystemTime::from(converted), instant);
            assert_eq!(converted.offset(), UtcOffset::UTC);
        }
    }
}
