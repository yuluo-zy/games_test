//! Validated theme identifiers, schedules, and consumer state.
use anyhow::{Context, Result, ensure};
use std::{
    fmt,
    path::PathBuf,
    str::FromStr,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

/// Canonical lowercase BLAKE3 digest. Construction never accepts a path or URL.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ContentId(String);

impl ContentId {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl FromStr for ContentId {
    type Err = anyhow::Error;
    fn from_str(value: &str) -> Result<Self> {
        ensure!(
            value.len() == 64
                && value
                    .bytes()
                    .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b)),
            "content ID must be exactly 64 lowercase hexadecimal characters"
        );
        Ok(Self(value.to_owned()))
    }
}

impl fmt::Display for ContentId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ActiveEvent {
    pub data_folder: PathBuf,
    pub start_time: SystemTime,
    pub end_time: SystemTime,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ThemeState {
    Idle,
    Pending,
    Active(ActiveEvent),
    Failed(String),
}

/// An ordered half-open local-time interval.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EventInterval {
    start_time: SystemTime,
    end_time: SystemTime,
}

impl EventInterval {
    pub fn new(start_time: SystemTime, end_time: SystemTime) -> Result<Self> {
        ensure!(start_time < end_time, "event start must precede event end");
        Ok(Self {
            start_time,
            end_time,
        })
    }

    /// Reject invalid wire order and arithmetic/platform overflow. Negative local
    /// seconds clamp to the epoch; an interval collapsed by clamping is invalid.
    pub fn from_unix_seconds(start: i64, end: i64, server_minus_local: i64) -> Result<Self> {
        ensure!(start < end, "event start must precede event end");
        let convert = |timestamp: i64| -> Result<SystemTime> {
            let local = timestamp
                .checked_sub(server_minus_local)
                .context("event clock adjustment overflow")?;
            UNIX_EPOCH
                .checked_add(Duration::from_secs(local.max(0) as u64))
                .context("event time exceeds platform SystemTime range")
        };
        Self::new(convert(start)?, convert(end)?)
    }

    pub fn start_time(&self) -> SystemTime {
        self.start_time
    }
    pub fn end_time(&self) -> SystemTime {
        self.end_time
    }
    pub fn is_active(&self, now: SystemTime) -> bool {
        self.start_time <= now && now < self.end_time
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ScheduledEvent {
    pub cid: ContentId,
    pub start_time: SystemTime,
    pub end_time: SystemTime,
}

impl ScheduledEvent {
    pub fn is_active(&self, now: SystemTime) -> bool {
        self.start_time <= now && now < self.end_time
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn content_ids_require_canonical_digest() {
        let value = "0123456789abcdef".repeat(4);
        let cid: ContentId = value.parse().unwrap();
        assert_eq!(cid.as_str(), value);
        assert_eq!(cid.to_string(), value);
        for invalid in [
            String::new(),
            "a".repeat(63),
            "a".repeat(65),
            "A".repeat(64),
            "g".repeat(64),
            "../".repeat(22),
        ] {
            assert!(invalid.parse::<ContentId>().is_err(), "{invalid}");
        }
    }
    #[test]
    fn interval_checks_order_adjustment_and_boundaries() {
        let interval = EventInterval::from_unix_seconds(10, 20, 5).unwrap();
        for (seconds, active) in [(4, false), (5, true), (14, true), (15, false)] {
            assert_eq!(
                interval.is_active(UNIX_EPOCH + Duration::from_secs(seconds)),
                active
            );
        }
        let clamped = EventInterval::from_unix_seconds(-1, 1, 0).unwrap();
        assert_eq!(clamped.start_time(), UNIX_EPOCH);
        for (start, end, offset) in [
            (1, 1, 0),
            (2, 1, 0),
            (-2, -1, 0),
            (i64::MIN, 1, 1),
            (0, i64::MAX, -1),
        ] {
            assert!(EventInterval::from_unix_seconds(start, end, offset).is_err());
        }
    }
}
