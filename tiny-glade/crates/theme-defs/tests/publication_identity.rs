//! Producer publication compares the whole result (IPI 0x3F76 and 0x3F7A).
use std::time::{Duration, UNIX_EPOCH};
use theme_defs::{ActiveEvent, EventDataError};

#[test]
fn active_event_identity_includes_both_full_precision_times_and_directory() {
    let event = ActiveEvent {
        data_folder: "cache/blob-a".into(),
        start_time: UNIX_EPOCH + Duration::new(1, 200),
        end_time: UNIX_EPOCH + Duration::new(3, 400),
    };
    assert_eq!(event, event.clone());
    let mut changed = event.clone();
    // Windows SystemTime has 100 ns resolution; preserve a real subsecond change.
    changed.start_time += Duration::from_nanos(100);
    assert_ne!(event, changed);
    let mut changed = event.clone();
    changed.end_time += Duration::from_nanos(100);
    assert_ne!(event, changed);
    let mut changed = event.clone();
    changed.data_folder = "cache/blob-b".into();
    assert_ne!(event, changed);
}

#[test]
fn result_identity_distinguishes_unit_states_and_owned_failure_text() {
    let pending = EventDataError::Pending;
    let inactive = EventDataError::NoActiveEvent;
    assert_ne!(pending, inactive);
    for text in ["", "invalid blob", "主题读取失败"] {
        let error = EventDataError::Error(text.to_owned());
        assert_eq!(error, error.clone());
        assert_ne!(error, pending);
        assert_ne!(error, inactive);
    }
    assert_ne!(
        EventDataError::Error("a".into()),
        EventDataError::Error("b".into())
    );
    let result: Result<ActiveEvent, EventDataError> = Err(EventDataError::Error("".into()));
    assert_ne!(result, Err(pending));
    assert_ne!(result, Err(inactive));
}
