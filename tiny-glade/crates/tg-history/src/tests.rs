use super::*;
use serde_json::Value;

#[test]
fn preserves_large_ids_unknown_commands_and_cursor_tail() {
    let bytes = br#"{"Version":27,"History":{"chain_index":0,"edit_chain":[{"edits":[]},{"edits":[{"Roof":{"Future":{"id":18446744073709551615}}},{"FutureWorld":{"id":11519500871291546481}}]}],"snapshots":[{"chain_index":0,"snapshot_id":11519500871291546481}]},"Extension":{"x":9}}"#;
    let document = HistoryDocument::parse(bytes).unwrap();
    let serialized = document.to_json().unwrap();
    assert_eq!(
        document.raw(),
        &serde_json::from_slice::<Value>(&serialized).unwrap()
    );
    assert_eq!(
        document
            .raw()
            .pointer("/History/snapshots/0/snapshot_id")
            .unwrap()
            .as_u64(),
        Some(11519500871291546481)
    );
    assert_eq!(
        document
            .raw()
            .pointer("/History/edit_chain/1/edits/0/Roof/Future/id")
            .unwrap()
            .as_u64(),
        Some(u64::MAX)
    );
    let summary = document.summary();
    assert_eq!(summary.entries, 2);
    assert_eq!(summary.chain_index, 0);
    assert_eq!(summary.unknown_roof_commands.len(), 1);
    assert!(document.roof_commands().next().is_none());
}

#[test]
fn invalid_enum_shape_returns_pointer() {
    let bad = br#"{"Version":17,"History":{"chain_index":0,"edit_chain":[{"edits":[{"Roof":{},"Wall":[]}]}],"snapshots":[]}}"#;
    let error = HistoryDocument::parse(bad).unwrap_err();
    assert!(error.path() == "/History/edit_chain/0/edits/0");
}

#[test]
fn recognizes_observed_command_families_and_keeps_future_tags() {
    let tags = [
        "WallNewSystem",
        "Wall",
        "Roof",
        "DecoratorV5",
        "Terrain",
        "Tree",
        "Color",
        "Stairs",
        "Path",
        "Water",
        "Garden",
        "Cobblestone1",
        "Cobblestone2",
        "FutureCommand",
    ];
    let edits = tags
        .iter()
        .map(|tag| format!("{{\"{tag}\":{{\"opaque\":true}}}}"))
        .collect::<Vec<_>>()
        .join(",");
    let bytes = format!(
        "{{\"Version\":27,\"History\":{{\"chain_index\":0,\"edit_chain\":[{{\"edits\":[{edits}]}}],\"snapshots\":[]}}}}"
    );
    let document = HistoryDocument::parse(bytes.as_bytes()).unwrap();
    let commands = document.commands().collect::<Vec<_>>();

    assert_eq!(commands.len(), tags.len());
    assert!(matches!(
        commands[0].kind,
        HistoryCommandKind::WallNewSystem
    ));
    assert!(matches!(commands[1].kind, HistoryCommandKind::Wall));
    assert!(matches!(
        commands[12].kind,
        HistoryCommandKind::Cobblestone2
    ));
    assert_eq!(
        commands[13].kind,
        HistoryCommandKind::Unknown("FutureCommand")
    );
    assert_eq!(commands[13].payload["opaque"], true);
    assert_eq!(
        document.summary().unknown_commands,
        vec!["/History/edit_chain/0/edits/13/FutureCommand"]
    );
}

#[test]
fn exposes_entry_sidecars_and_snapshot_refs_without_replay_assumptions() {
    let bytes = br#"{"Version":27,"History":{"chain_index":0,"history_start":-1,"edit_chain":[{"edits":[{"Wall":{"opaque":1}}],"decorators":{"diffs":[]},"stairs":{"hash_before":7,"hash_after":8}},{"edits":[]}],"snapshots":[{"chain_index":0,"snapshot_id":11519500871291546481}]}}"#;
    let document = HistoryDocument::parse(bytes).unwrap();
    let entries = document.history_entries().collect::<Vec<_>>();
    let snapshots = document.snapshot_references().collect::<Vec<_>>();
    let summary = document.summary();

    assert_eq!(entries.len(), 2);
    assert!(entries[0].decorators.is_some());
    assert!(entries[0].stairs.is_some());
    assert_eq!(
        entries[0].commands().next().unwrap().kind,
        HistoryCommandKind::Wall
    );
    assert_eq!(document.history_start().unwrap(), -1);
    assert_eq!(snapshots[0].snapshot_id, 11519500871291546481);
    assert_eq!(summary.entries_after_chain_index, 1);
    assert_eq!(summary.snapshot_references, 1);
}

#[test]
fn decodes_the_four_observed_roof_variants() {
    let bytes = br#"{"Version":27,"History":{"chain_index":3,"edit_chain":[{"edits":[{"Roof":{"CreateRoof":{"wall_index":7,"color_id":9,"params":{"height_01":0.1,"profile_01":0.2,"eave_length_01":0.3,"ridge_length_01":0.4,"ridge_dir":"Width","tip_offset_01":[0.5,0.6]}}}},{"Roof":{"DeleteRoof":{"wall_index":7,"color_id":9,"params":{"height_01":0.1,"profile_01":0.2,"eave_length_01":0.3,"ridge_length_01":0.4,"ridge_dir":"Length","tip_offset_01":[0.5,0.6]}}}},{"Roof":{"VisualEdit":{"wall_index":7,"prev_height":0.1,"new_height":0.2,"prev_profile":0.3,"new_profile":0.4,"prev_eave":0.5,"new_eave":0.6,"prev_ridge":0.7,"new_ridge":0.8,"prev_tip_offset":[0.0,0.1],"new_tip_offset":[0.2,0.3]}}},{"Roof":{"SwitchRoofRidgeDir":{"wall_index":7,"from":"Width","to":"Length"}}}]}],"snapshots":[]}}"#;
    let document = HistoryDocument::parse(bytes).unwrap();
    let commands = document
        .roof_commands()
        .collect::<Result<Vec<_>, _>>()
        .unwrap();

    assert_eq!(commands.len(), 4);
    assert!(matches!(commands[0].command, RoofCommand::CreateRoof(_)));
    assert!(matches!(commands[1].command, RoofCommand::DeleteRoof(_)));
    assert!(matches!(commands[2].command, RoofCommand::VisualEdit(_)));
    assert!(matches!(
        commands[3].command,
        RoofCommand::SwitchRoofRidgeDir(_)
    ));
    assert_eq!(commands[3].entry_index, 0);
    assert_eq!(commands[3].edit_index, 3);
}

fn document_value() -> serde_json::Value {
    serde_json::json!({
        "Version": 27,
        "History": {"chain_index": 0, "edit_chain": [{"edits": []}], "snapshots": []}
    })
}

fn parse_value(value: &serde_json::Value) -> HistoryDocument {
    HistoryDocument::parse(&serde_json::to_vec(value).unwrap()).unwrap()
}

#[test]
fn validates_structure_and_integer_boundaries_with_located_errors() {
    use serde_json::json;
    use std::error::Error;

    let cases = [
        ("/Version", json!(-1), "/Version"),
        ("/History", json!(null), "/History"),
        (
            "/History/chain_index",
            json!(2147483648_u64),
            "/History/chain_index",
        ),
        (
            "/History/chain_index",
            json!(-2147483649_i64),
            "/History/chain_index",
        ),
        ("/History/edit_chain", json!({}), "/History/edit_chain"),
        (
            "/History/edit_chain/0",
            json!(null),
            "/History/edit_chain/0/edits",
        ),
        (
            "/History/edit_chain/0/edits",
            json!({}),
            "/History/edit_chain/0/edits",
        ),
        (
            "/History/edit_chain/0/edits",
            json!([{"Wall": null, "Roof": null}]),
            "/History/edit_chain/0/edits/0",
        ),
        (
            "/History/edit_chain/0/edits",
            json!([{"Roof": {}}]),
            "/History/edit_chain/0/edits/0/Roof",
        ),
        ("/History/snapshots", json!([false]), "/History/snapshots/0"),
        (
            "/History/snapshots",
            json!([{"chain_index": 2147483648_u64, "snapshot_id": 1}]),
            "/History/snapshots/0/chain_index",
        ),
        (
            "/History/snapshots",
            json!([{"chain_index": 0, "snapshot_id": -1}]),
            "/History/snapshots/0/snapshot_id",
        ),
    ];
    for (replace_path, replacement, error_path) in cases {
        let mut value = document_value();
        *value.pointer_mut(replace_path).unwrap() = replacement;
        let error = HistoryDocument::parse(&serde_json::to_vec(&value).unwrap()).unwrap_err();
        assert_eq!(error.path(), error_path, "{value}");
        assert!(matches!(error.kind(), DecodeErrorKind::Expected(_)));
        assert!(error.source().is_none());
    }

    let error = HistoryDocument::parse(b"{").unwrap_err();
    assert_eq!(error.path(), "");
    let source = error
        .source()
        .unwrap()
        .downcast_ref::<serde_json::Error>()
        .unwrap();
    assert_eq!(source.classify(), serde_json::error::Category::Eof);

    for cursor in [i32::MIN, -1, i32::MAX] {
        let mut value = document_value();
        value["History"]["chain_index"] = json!(cursor);
        let document = parse_value(&value);
        assert_eq!(document.chain_index(), cursor);
        assert_eq!(
            document.summary().entries_after_chain_index,
            usize::from(cursor < 0)
        );
    }
}

#[test]
fn summarizing_does_not_force_lazy_roof_decoding() {
    use serde_json::json;
    use std::error::Error;

    let mut value = document_value();
    value["History"]["edit_chain"][0]["edits"] = json!([
        {"Roof": {"SwitchRoofRidgeDir": {"wall_index": 7, "from": "Width", "to": "Length"}}},
        {"Roof": {"VisualEdit": {}}},
        {"Roof": {"Future": {"opaque": true}}},
        {"Roof": {"SwitchRoofRidgeDir": {"wall_index": 8, "from": "Length", "to": "Width"}}}
    ]);
    let bytes = serde_json::to_vec(&value).unwrap();
    let document = HistoryDocument::parse(&bytes).unwrap();
    let summary =
        serde_json::from_str::<serde_json::Value>(&summarize_json(&bytes).unwrap()).unwrap();
    assert!(summary.get("replay_verified").is_none());
    assert_eq!(summary["roof_command_counts"]["VisualEdit"], 1);
    assert_eq!(
        summary["unknown_roof_commands"].as_array().unwrap().len(),
        1
    );

    let mut commands = document.roof_commands();
    assert_eq!(commands.next().unwrap().unwrap().edit_index, 0);
    let error = commands.next().unwrap().unwrap_err();
    assert_eq!(error.path(), "/History/edit_chain/0/edits/1/Roof");
    assert!(error.source().unwrap().is::<serde_json::Error>());
    assert!(error.to_string().contains("missing field"));
    assert_eq!(commands.next().unwrap().unwrap().edit_index, 3);
    assert!(commands.next().is_none());
    assert_eq!(document.raw(), &value);
}

#[test]
fn borrowed_views_preserve_identity_and_escape_unknown_tag_pointers() {
    use serde_json::json;

    let mut value = document_value();
    value["History"]["edit_chain"][0] = json!({
        "edits": [{"Future~/Command": {"id": u64::MAX}}, {"Roof": {"Future~/Roof": {"id": u64::MAX}}}],
        "decorators": {"unknown": [1, 2, 3]},
        "stairs": null
    });
    value["History"]["snapshots"] =
        json!([{"chain_index": 0, "snapshot_id": u64::MAX, "extension": [1, 2]}]);
    let document = parse_value(&value);
    let entry = document.history_entries().next().unwrap();
    assert!(std::ptr::eq(
        entry.raw,
        &document.raw()["History"]["edit_chain"][0]
    ));
    assert!(std::ptr::eq(
        entry.decorators.unwrap(),
        &entry.raw["decorators"]
    ));
    assert_eq!(entry.stairs, Some(&json!(null)));
    let command = entry.commands().next().unwrap();
    assert!(std::ptr::eq(
        command.payload,
        document.commands().next().unwrap().payload
    ));
    assert!(std::ptr::eq(
        command.payload,
        &entry.raw["edits"][0]["Future~/Command"]
    ));
    assert!(std::ptr::eq(
        document.snapshot_references().next().unwrap().raw,
        &document.raw()["History"]["snapshots"][0]
    ));
    let summary = document.summary();
    assert_eq!(
        summary.unknown_commands,
        ["/History/edit_chain/0/edits/0/Future~0~1Command"]
    );
    assert_eq!(
        summary.unknown_roof_commands,
        ["/History/edit_chain/0/edits/1/Roof/Future~0~1Roof"]
    );
    for pointer in summary
        .unknown_commands
        .iter()
        .chain(&summary.unknown_roof_commands)
    {
        assert_eq!(document.raw().pointer(pointer).unwrap()["id"], u64::MAX);
    }
}

#[test]
fn snapshot_selection_handles_resolution_ties_duplicates_and_extreme_indices() {
    use serde_json::json;

    let mut value = document_value();
    value["History"]["chain_index"] = json!(4);
    value["History"]["snapshots"] = json!([
        {"chain_index": 6, "snapshot_id": 6},
        {"chain_index": 2, "snapshot_id": 2},
        {"chain_index": 4, "snapshot_id": 4},
        {"chain_index": 6, "snapshot_id": 66},
        {"chain_index": i32::MIN, "snapshot_id": 1},
        {"chain_index": i32::MAX, "snapshot_id": u64::MAX}
    ]);
    let document = parse_value(&value);
    assert_eq!(
        document.current_snapshot_reference().unwrap().snapshot_id,
        4
    );
    assert_eq!(
        document
            .closest_resolved_snapshot(4, |id| id != 4)
            .unwrap()
            .snapshot_id,
        6
    );
    assert_eq!(
        document
            .closest_resolved_snapshot(4, |id| id == 2)
            .unwrap()
            .snapshot_id,
        2
    );
    assert!(document.closest_resolved_snapshot(4, |_| false).is_none());
    assert_eq!(
        document
            .closest_resolved_snapshot(i32::MIN, |_| true)
            .unwrap()
            .chain_index,
        i32::MIN
    );
    assert_eq!(
        document
            .closest_resolved_snapshot(i32::MAX, |_| true)
            .unwrap()
            .snapshot_id,
        u64::MAX
    );
    assert_eq!(
        document
            .closest_resolved_snapshot(i32::MAX, |id| id == 1)
            .unwrap()
            .chain_index,
        i32::MIN
    );

    value["History"]["chain_index"] = json!(5);
    assert!(parse_value(&value).current_snapshot_reference().is_none());
    value["History"]["snapshots"] = json!([]);
    let empty = parse_value(&value);
    assert!(empty.current_snapshot_reference().is_none());
    assert!(empty.closest_resolved_snapshot(5, |_| true).is_none());
}
