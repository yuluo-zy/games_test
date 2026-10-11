use std::io::{self, BufRead};
use tiny_glade_reconstruction::{
    building_history_transactions::*, history_transactions::*, roof_replay::write_observed_event,
};
fn decode<const N: usize>(s: &str) -> [u8; N] {
    assert_eq!(s.len(), N * 2);
    std::array::from_fn(|i| u8::from_str_radix(&s[i * 2..i * 2 + 2], 16).unwrap())
}
fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}
fn state(h: &BuildingHistoryTransactions) -> String {
    let l = h.ledger();
    let groups = l
        .entries
        .iter()
        .map(|entry| {
            entry
                .edits
                .iter()
                .map(|edit| match edit {
                    BuildingEdit::Roof(roof) => hex(roof.payload()),
                    BuildingEdit::Wall(_) => "wall".into(),
                    BuildingEdit::UnresolvedOwner { observed_tag } => format!("?{observed_tag}"),
                })
                .collect::<Vec<_>>()
                .join(",")
        })
        .collect::<Vec<_>>()
        .join(";");
    format!(
        "{} {} {} {} {}",
        l.cursor,
        l.undo_anchor,
        l.mutation_nonce,
        h.pending().map(|e| e.edits.len() as i64).unwrap_or(-1),
        if groups.is_empty() { "-" } else { &groups }
    )
}
fn error(error: BuildingHistoryError) -> String {
    match error {
        BuildingHistoryError::Transaction(e) => format!(
            "E{}",
            match e {
                HistoryTransactionError::ReplayRecordingForbidden => 1,
                HistoryTransactionError::RecordingNotStarted => 2,
                HistoryTransactionError::InvalidCursor => 3,
                HistoryTransactionError::CursorNotAtCommittedTail => 4,
                HistoryTransactionError::AuxiliaryReplayOwnerRequired => 5,
            }
        ),
        BuildingHistoryError::UnrestoredWallExpansion => "unrestored-wall-expansion".into(),
        BuildingHistoryError::AuxiliaryReplayOwnerRequired => "E5".into(),
        BuildingHistoryError::InvalidRoofTag(tag) => format!("T{tag}"),
        BuildingHistoryError::UnresolvedOwner(tag) => format!("O{tag}"),
    }
}
fn main() {
    for line in io::stdin().lock().lines() {
        let line = line.unwrap();
        let fields: Vec<_> = line.split_whitespace().collect();
        let mut h = BuildingHistoryTransactions::from_observed(ObservedHistoryLedger {
            entries: vec![],
            cursor: -1,
            undo_anchor: -1,
            resolved_snapshots: vec![],
            observed_count_58: 0,
            mutation_nonce: 777,
        });
        let mut next = 9000u64;
        let mut nonce = || {
            next += 1;
            next
        };
        let mut output = vec![];
        if fields[0] == "wall_height" {
            let edit = WallHeightHistoryEdit {
                wall_id: fields[1].parse().unwrap(),
                from: f32::from_bits(u32::from_str_radix(fields[2], 16).unwrap()),
                to: f32::from_bits(u32::from_str_radix(fields[3], 16).unwrap()),
            };
            let mut arc = [0xa5; 264];
            edit.write_observed_arc_fields(&mut arc);
            let mut event = [0xa5; 40];
            edit.instant_request(fields[4] == "1")
                .write_observed_fields(&mut event);
            println!("{} {}", hex(&arc[..40]), hex(&event));
            continue;
        }
        if fields[0] == "bucket" {
            h.start_input_recording(0).unwrap();
            for payload in fields[3].split(',') {
                h.add_with_nonce_provider(
                    BuildingEdit::Roof(RoofHistoryEdit::from_observed(decode(payload)).unwrap()),
                    &mut nonce,
                )
                .unwrap();
            }
            h.finish_with_nonce_provider(&mut nonce).unwrap();
            let undo = fields[1] == "1";
            if !undo {
                h.cursor_step(HistoryCursorStep::Undo, true).unwrap();
            }
            let mut selection = h
                .cursor_step(
                    if undo {
                        HistoryCursorStep::Undo
                    } else {
                        HistoryCursorStep::Redo
                    },
                    fields[2] == "1",
                )
                .unwrap()
                .unwrap();
            for dt in fields[4].split(',') {
                let dt = f32::from_bits(u32::from_str_radix(dt, 16).unwrap());
                let mut counts = [0usize; 5];
                for event in advance_roof_bucket(&mut selection.commands, dt).unwrap() {
                    match event {
                        BuildingReplayOutput::Discrete(event) => {
                            counts[event.observed_tag() as usize] += 1
                        }
                        BuildingReplayOutput::Visual(_) => counts[2] += 1,
                        BuildingReplayOutput::NoEvent => {}
                    }
                }
                let remaining = selection
                    .commands
                    .iter()
                    .map(|command| {
                        format!(
                            "{}:{}",
                            hex(command.payload()),
                            hex(&command.state().bytes()[16..22])
                        )
                    })
                    .collect::<Vec<_>>()
                    .join(",");
                output.push(format!(
                    "{}|{}",
                    counts
                        .iter()
                        .map(|c| c.to_string())
                        .collect::<Vec<_>>()
                        .join(","),
                    if remaining.is_empty() {
                        "-"
                    } else {
                        &remaining
                    }
                ));
            }
            println!("{}", output.join(" / "));
            continue;
        }
        for op in fields[0].split('/') {
            let result: Result<String, BuildingHistoryError> =
                if let Some(mode) = op.strip_prefix('s') {
                    h.start_input_recording(mode.parse().unwrap())
                        .map(|_| "ok".into())
                } else if let Some(mode) = op.strip_prefix('t') {
                    Ok(if h.try_start_input_recording(mode.parse().unwrap()) {
                        "1"
                    } else {
                        "0"
                    }
                    .into())
                } else if let Some(payload) = op.strip_prefix('a') {
                    RoofHistoryEdit::from_observed(decode(payload))
                        .and_then(|edit| {
                            h.add_with_nonce_provider(BuildingEdit::Roof(edit), &mut nonce)
                        })
                        .map(|_| "ok".into())
                } else if let Some(roof) = op.strip_prefix('x') {
                    h.add_with_nonce_provider(
                        BuildingEdit::Roof(RoofHistoryEdit::delete_from_roof(&decode(roof))),
                        &mut nonce,
                    )
                    .map(|_| "ok".into())
                } else if let Some(records) = op.strip_prefix('v') {
                    let records: Vec<[u8; 152]> = if records.is_empty() {
                        vec![]
                    } else {
                        records.split(',').map(decode).collect()
                    };
                    if let Some(edit) = collect_last_visual_history(&records) {
                        h.add_with_nonce_provider(edit, &mut nonce)
                            .map(|_| "1".into())
                    } else {
                        Ok("0".into())
                    }
                } else if let Some(records) = op.strip_prefix('d') {
                    let records: Vec<[u8; 24]> = if records.is_empty() {
                        vec![]
                    } else {
                        records.split(',').map(decode).collect()
                    };
                    let edits = collect_direction_history(&records);
                    let n = edits.len();
                    let mut result = Ok(n.to_string());
                    for edit in edits {
                        if let Err(e) = h.add_with_nonce_provider(edit, &mut nonce) {
                            result = Err(e);
                            break;
                        }
                    }
                    result
                } else if op == "f" {
                    h.finish_with_nonce_provider(&mut nonce)
                        .map(|commit| if commit.is_some() { "1" } else { "0" }.into())
                } else if op.starts_with('u') || op.starts_with('r') {
                    let undo = op.starts_with('u');
                    let instant = op.ends_with('!');
                    h.cursor_step(
                        if undo {
                            HistoryCursorStep::Undo
                        } else {
                            HistoryCursorStep::Redo
                        },
                        instant,
                    )
                    .and_then(|selection| {
                        let Some(mut selection) = selection else {
                            return Ok("none".into());
                        };
                        let mut events = vec![];
                        for command in &mut selection.commands {
                            let before = command.state().bytes();
                            let identity = format!(
                                "{}:{}:{}",
                                hex(command.payload()),
                                hex(&before[..8]),
                                before[20]
                            );
                            let event = command.advance(0.)?;
                            let event = match event {
                                BuildingReplayOutput::Discrete(event) => {
                                    let mut b = [0xa5; 64];
                                    event.write_observed_fields(&mut b);
                                    let end = match event.observed_tag() {
                                        0 => 61,
                                        1 | 3 => 17,
                                        4 => 19,
                                        _ => unreachable!(),
                                    };
                                    format!("{}:{}", event.observed_tag(), hex(&b[8..end]))
                                }
                                BuildingReplayOutput::Visual(event) => {
                                    let mut b = [0xa5; 64];
                                    write_observed_event(&event, &mut b);
                                    format!("2:{}", hex(&b[..53]))
                                }
                                BuildingReplayOutput::NoEvent => "-".into(),
                            };
                            let after = command.state().bytes();
                            events.push(format!(
                                "{identity}:{event}:{}:{}",
                                hex(&after[16..20]),
                                hex(&after[20..22])
                            ));
                        }
                        Ok(format!(
                            "{}:{}:{}",
                            selection.entry_index,
                            u8::from(undo),
                            events.join(",")
                        ))
                    })
                } else {
                    panic!("Unknown probe operation")
                };
            output.push(format!("{}|{}", result.unwrap_or_else(error), state(&h)));
        }
        println!("{}", output.join(" / "));
    }
}
