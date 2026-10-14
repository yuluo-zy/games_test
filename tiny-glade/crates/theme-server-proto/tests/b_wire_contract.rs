//! B's serde schema policies, from original 0x140615f70 and 0x140617820.
//! Uses original serde/serde_json versions; no hand-written decoder or HTTP.
use theme_server_proto::{ActiveEvent, EventsResponse};

#[test]
fn wire_keeps_signed_seconds_unvalidated_cids_and_response_order() {
    let json = r#"{"events":[{"start":9223372036854775807,"end":-9223372036854775808,"cid":""},{"start":-1,"end":-2,"cid":"目录/任意字符串"}]}"#;
    let response: EventsResponse = serde_json::from_str(json).unwrap();
    assert_eq!(response.events[0].start, i64::MAX);
    assert_eq!(response.events[0].end, i64::MIN);
    assert_eq!(response.events[0].cid, "");
    assert_eq!(response.events[1].start, -1);
    assert_eq!(response.events[1].end, -2);
    assert_eq!(response.events[1].cid, "目录/任意字符串");
    assert_eq!(serde_json::to_string(&response).unwrap(), json);
}

#[test]
fn struct_sequences_use_start_end_cid_then_events() {
    let from_sequence: EventsResponse =
        serde_json::from_str(r#"[[[10,20,"a"],[-5,-1,"b"]]]"#).unwrap();
    let from_map: EventsResponse = serde_json::from_str(
        r#"{"events":[{"cid":"a","end":20,"start":10},{"start":-5,"cid":"b","end":-1}]}"#,
    )
    .unwrap();
    assert_eq!(from_sequence, from_map);
    assert_eq!(
        serde_json::from_str::<EventsResponse>("[[]]")
            .unwrap()
            .events,
        []
    );
}

#[test]
fn unknown_nested_fields_are_ignored_at_both_levels() {
    let value: EventsResponse = serde_json::from_str(r#"{"future":{"nested":[true,null,{}]},"events":[{"start":1,"end":2,"cid":"a","future":[{"events":"ignored"}]}]}"#).unwrap();
    assert_eq!(
        value.events,
        [ActiveEvent {
            start: 1,
            end: 2,
            cid: "a".into()
        }]
    );
}

#[test]
fn all_known_fields_are_required_and_duplicates_are_errors() {
    for (json, field) in [
        (r#"{"end":2,"cid":"a"}"#, "start"),
        (r#"{"start":1,"cid":"a"}"#, "end"),
        (r#"{"start":1,"end":2}"#, "cid"),
    ] {
        let error = serde_json::from_str::<ActiveEvent>(json)
            .unwrap_err()
            .to_string();
        assert!(
            error.starts_with(&format!("missing field `{field}`")),
            "{error}"
        );
    }
    let error = serde_json::from_str::<EventsResponse>("{}")
        .unwrap_err()
        .to_string();
    assert!(error.starts_with("missing field `events`"), "{error}");
    for (json, field) in [
        (r#"{"start":1,"start":2,"end":3,"cid":"a"}"#, "start"),
        (r#"{"start":1,"end":2,"end":3,"cid":"a"}"#, "end"),
        (r#"{"start":1,"end":2,"cid":"a","cid":"b"}"#, "cid"),
    ] {
        let error = serde_json::from_str::<ActiveEvent>(json)
            .unwrap_err()
            .to_string();
        assert!(
            error.starts_with(&format!("duplicate field `{field}`")),
            "{error}"
        );
    }
    let error = serde_json::from_str::<EventsResponse>(r#"{"events":[],"events":[]}"#)
        .unwrap_err()
        .to_string();
    assert!(error.starts_with("duplicate field `events`"), "{error}");
}

#[test]
fn wrong_types_overflow_and_wrong_sequence_lengths_are_rejected() {
    for json in [
        r#"{"start":9223372036854775808,"end":2,"cid":"a"}"#,
        r#"{"start":-9223372036854775809,"end":2,"cid":"a"}"#,
        r#"{"start":1.0,"end":2,"cid":"a"}"#,
        r#"{"start":"1","end":2,"cid":"a"}"#,
        r#"{"start":null,"end":2,"cid":"a"}"#,
        r#"{"start":1,"end":false,"cid":"a"}"#,
        r#"{"start":1,"end":2,"cid":null}"#,
        r#"{"start":1,"end":2,"cid":1}"#,
        "[]",
        "[1]",
        "[1,2]",
        r#"[1,2,"a",4]"#,
    ] {
        assert!(serde_json::from_str::<ActiveEvent>(json).is_err(), "{json}");
    }
    for json in [
        "[]",
        "[[],[]]",
        r#"{"events":null}"#,
        r#"{"events":{}}"#,
        r#"{"events":[null]}"#,
        r#"{"events":[]}{}"#,
    ] {
        assert!(
            serde_json::from_str::<EventsResponse>(json).is_err(),
            "{json}"
        );
    }
}
