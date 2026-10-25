//! Wire schema and the validation boundary before scheduling or filesystem use.
use crate::model::{ContentId, EventInterval, ScheduledEvent};
use anyhow::{Context, Result, ensure};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Event {
    pub start: i64,
    pub end: i64,
    pub cid: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct EventsResponse {
    pub events: Vec<Event>,
}

impl EventsResponse {
    /// Validate the entire response atomically while preserving server order.
    /// The HTTP layer must bound body bytes before deserializing this structure.
    pub fn validate(
        &self,
        server_minus_local: i64,
        max_events: usize,
    ) -> Result<Vec<ScheduledEvent>> {
        ensure!(
            self.events.len() <= max_events,
            "event count exceeds limit of {max_events}"
        );
        self.events
            .iter()
            .enumerate()
            .map(|(index, event)| {
                let cid: ContentId = event
                    .cid
                    .parse()
                    .with_context(|| format!("event {index}: invalid content ID"))?;
                let interval =
                    EventInterval::from_unix_seconds(event.start, event.end, server_minus_local)
                        .with_context(|| format!("event {index}: invalid interval"))?;
                Ok(ScheduledEvent {
                    cid,
                    start_time: interval.start_time(),
                    end_time: interval.end_time(),
                })
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn event(start: i64, end: i64) -> Event {
        Event {
            start,
            end,
            cid: "a".repeat(64),
        }
    }
    #[test]
    fn validation_preserves_order_and_enforces_limits() {
        let response = EventsResponse {
            events: vec![event(10, 20), event(1, 2)],
        };
        assert!(response.validate(0, 1).is_err());
        let events = response.validate(0, 2).unwrap();
        assert!(events[0].start_time > events[1].start_time);
        assert!(
            EventsResponse { events: vec![] }
                .validate(0, 0)
                .unwrap()
                .is_empty()
        );
    }
    #[test]
    fn invalid_members_reject_the_whole_response() {
        let mut bad_cid = event(1, 2);
        bad_cid.cid = "../escape".into();
        for invalid in [bad_cid, event(2, 1), event(1, 1), event(i64::MIN, 1)] {
            let response = EventsResponse {
                events: vec![event(1, 2), invalid],
            };
            assert!(response.validate(1, 2).is_err());
        }
    }
}
