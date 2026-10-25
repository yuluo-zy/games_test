//! B theme transport contracts from the ThemeFetcher producer and game consumer.
//! Names of fields/error variants below are semantic reconstruction names, not PDB
//! field names. SystemTime/PathBuf definitions are reused from std.
use std::path::PathBuf;
use std::time::SystemTime;

/// Data available for a currently active event. IPI 0x3F7A nests two
/// std::time::SystemTime::eq inline sites then std::path::PathBuf::eq.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ActiveEvent {
    pub data_folder: PathBuf,
    pub start_time: SystemTime,
    pub end_time: SystemTime,
}

/// Observed error families. Names are inferred from producer branches:
/// initial per-event cache, downloaded/validation error text, no active interval.
/// No serde wire representation is claimed by this enum.
#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error)]
pub enum EventDataError {
    #[error("event data is pending")]
    Pending,
    #[error("{0}")]
    Error(String),
    #[error("no active event")]
    NoActiveEvent,
}

/// Pure policy helper for the observed half-open interval; this helper is new,
/// the original embeds the same comparison in ThemeFetcher::run.
pub fn is_active_interval(start_time: SystemTime, end_time: SystemTime, now: SystemTime) -> bool {
    start_time <= now && now < end_time
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn interval_end_is_exclusive_and_payload_survives_clone() {
        let t = |s| std::time::UNIX_EPOCH + std::time::Duration::from_secs(s);
        assert!(!is_active_interval(t(10), t(20), t(9)));
        assert!(is_active_interval(t(10), t(20), t(10)));
        assert!(is_active_interval(t(10), t(20), t(19)));
        assert!(!is_active_interval(t(10), t(20), t(20)));
        assert!(!is_active_interval(t(20), t(10), t(15)));
        let event = ActiveEvent {
            data_folder: "theme-data".into(),
            start_time: t(10),
            end_time: t(20),
        };
        assert_eq!(event.clone(), event);
    }
}
