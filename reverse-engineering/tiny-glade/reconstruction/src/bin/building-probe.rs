//! Focused B building-state differential runner; no ECS/container emulation.
use std::io::{self, BufRead};
use tiny_glade_reconstruction::{building_roof_spatial as spatial, building_wall_state as wall};
fn decode<const N: usize>(text: &str) -> [u8; N] {
    assert_eq!(text.len(), N * 2);
    std::array::from_fn(|i| u8::from_str_radix(&text[i * 2..i * 2 + 2], 16).unwrap())
}
fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}
fn main() {
    for line in io::stdin().lock().lines() {
        let line = line.unwrap();
        let p: Vec<_> = line.split_whitespace().collect();
        match p[0] {
            "chain" => {
                use tiny_glade_reconstruction::{building_roof_creation as creation, roof_replay};
                let mut replay = roof_replay::VisualReplayState::from_observed(decode(p[1]));
                let payload = decode(p[2]);
                let mut roof = decode(p[3]);
                let mut related = decode(p[4]);
                let original_wall = wall::PublicWallBuildingState::from_observed(decode(p[5]));
                let updated_wall = wall::PublicWallBuildingState::from_observed(decode(p[6]));
                let entity = u64::from_str_radix(p[7], 16).unwrap();
                let event = replay
                    .advance_discrete_observed(&payload, 0.25)
                    .unwrap()
                    .unwrap();
                let request = creation::RoofCreateRequestObserved::from_replay_event(&event)
                    .expect("create event");
                let initialized =
                    creation::initialize_resolved_roof(&mut roof, &request, &original_wall)
                        .unwrap();
                let initial_bytes = hex(&roof);
                let mut roof_tick = 0xaaaaaaaa;
                let mut related_tick = 0xbbbbbbbb;
                let outcome = spatial::synchronize_resolved_roof(
                    &mut roof,
                    &mut related,
                    &updated_wall,
                    request.wall_id(),
                    entity,
                    &mut roof_tick,
                    &mut related_tick,
                    0x12345678,
                )
                .unwrap();
                let events = outcome
                    .geometry_changes
                    .iter()
                    .map(|e| {
                        format!(
                            "{:016x}:{:016x}:{}",
                            e.roof_entity, e.wall_id.0, e.observed_kind
                        )
                    })
                    .collect::<Vec<_>>()
                    .join(",");
                println!(
                    "{} {} {} {} {} {} {:08x} {:08x} {} {}",
                    hex(replay.bytes()),
                    initial_bytes,
                    u8::from(initialized.pitched),
                    initialized
                        .gable_notification
                        .map(|id| format!("{:016x}", id.0))
                        .unwrap_or("-".into()),
                    hex(&roof),
                    hex(&related),
                    roof_tick,
                    related_tick,
                    outcome.geometry_changes.len(),
                    if events.is_empty() { "-" } else { &events }
                );
            }
            "create" => {
                use tiny_glade_reconstruction::building_roof_creation as creation;
                let mut roof = decode(p[1]);
                let request = creation::RoofCreateRequestObserved::from_observed(decode(p[2]));
                let wall = wall::PublicWallBuildingState::from_observed(decode(p[3]));
                match creation::initialize_resolved_roof(&mut roof, &request, &wall) {
                    Ok(outcome) => println!(
                        "{} {} {}",
                        hex(&roof),
                        u8::from(outcome.pitched),
                        outcome
                            .gable_notification
                            .map(|id| format!("{:016x}", id.0))
                            .unwrap_or("-".into())
                    ),
                    Err(_) => println!("panic"),
                }
            }
            "guard" => {
                let count = u64::from_str_radix(p[1], 16).unwrap();
                let height = f32::from_bits(u32::from_str_radix(p[2], 16).unwrap());
                println!(
                    "{}",
                    if wall::validate_public_wall_read(count, height).is_ok() {
                        "ok"
                    } else {
                        "panic"
                    }
                );
            }
            "spatial" => {
                let mut roof = decode(p[1]);
                let mut related = decode(p[2]);
                let wall = wall::PublicWallBuildingState::from_observed(decode(p[3]));
                let wall_id = wall::WallId(u64::from_str_radix(p[4], 16).unwrap());
                let entity = u64::from_str_radix(p[5], 16).unwrap();
                let mut roof_tick = 0xaaaaaaaa;
                let mut related_tick = 0xbbbbbbbb;
                match spatial::synchronize_resolved_roof(
                    &mut roof,
                    &mut related,
                    &wall,
                    wall_id,
                    entity,
                    &mut roof_tick,
                    &mut related_tick,
                    0x12345678,
                ) {
                    Ok(outcome) => {
                        let events = outcome
                            .geometry_changes
                            .iter()
                            .map(|e| {
                                format!(
                                    "{:016x}:{:016x}:{}",
                                    e.roof_entity, e.wall_id.0, e.observed_kind
                                )
                            })
                            .collect::<Vec<_>>()
                            .join(",");
                        println!(
                            "{} {} {:08x} {:08x} {} {}",
                            hex(&roof),
                            hex(&related),
                            roof_tick,
                            related_tick,
                            outcome.geometry_changes.len(),
                            if events.is_empty() { "-" } else { &events }
                        );
                    }
                    Err(_) => println!("panic"),
                }
            }
            _ => panic!("unknown building probe"),
        }
    }
}
