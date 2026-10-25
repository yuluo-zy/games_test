use std::io::{self, BufRead};
use tiny_glade_reconstruction::{
    building_roof_associations as associations, building_roof_components as components,
    building_roof_deletion as deletion, building_wall_publication as publication,
    building_wall_state as wall,
};
fn decode<const N: usize>(text: &str) -> [u8; N] {
    assert_eq!(text.len(), N * 2);
    std::array::from_fn(|i| u8::from_str_radix(&text[i * 2..i * 2 + 2], 16).unwrap())
}
fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}
fn float(s: &str) -> f32 {
    f32::from_bits(u32::from_str_radix(s, 16).unwrap())
}
fn main() {
    for line in io::stdin().lock().lines() {
        let line = line.unwrap();
        let p: Vec<_> = line.split_whitespace().collect();
        match p[0] {
            "direction" => {
                use tiny_glade_reconstruction::building_roof_direction::*;
                let mut roof = decode(p[1]);
                let wall = wall::WallId(u64::from_str_radix(p[2], 16).unwrap());
                let entity = u64::from_str_radix(p[3], 16).unwrap();
                let direction = p[4].parse().unwrap();
                let mut tick = 0;
                let outcome = apply_resolved_direction(
                    &mut roof,
                    RoofDirectionRequest { wall, direction },
                    entity,
                    &mut tick,
                    0x12345678,
                );
                println!(
                    "{} {:08x} {} {} {:016x} {:016x} {:08x},{:08x},{:08x}",
                    hex(&roof),
                    tick,
                    outcome.geometry_kind,
                    outcome.sound_kind,
                    entity,
                    wall.0,
                    outcome.sound_pivot[0].to_bits(),
                    outcome.sound_pivot[1].to_bits(),
                    outcome.sound_pivot[2].to_bits()
                );
            }
            "gable_arch" => {
                let roof = decode(p[1]);
                let base = float(p[2]);
                let length = float(p[3]);
                let half = p[4] == "1";
                let positions: Vec<_> = p[5..]
                    .chunks_exact(3)
                    .map(|v| [float(v[0]), float(v[1]), float(v[2])])
                    .collect();
                let result =
                    tiny_glade_reconstruction::building_wall_gable::apply_gable_arch_heights(
                        &positions,
                        &vec![base; positions.len()],
                        base,
                        length,
                        half,
                        &roof,
                    )
                    .unwrap();
                println!(
                    "{:08x} {}",
                    result.maximum_y.to_bits(),
                    result
                        .heights
                        .iter()
                        .map(|f| format!("{:08x}", f.to_bits()))
                        .collect::<Vec<_>>()
                        .join(",")
                );
            }
            "cache" => {
                let shape = wall::WallShapeSnapshot::from_observed(decode(p[1]));
                let positions: Vec<_> = p[3..]
                    .chunks_exact(3)
                    .map(|v| [float(v[0]), float(v[1]), float(v[2])])
                    .collect();
                let cache =
                    tiny_glade_reconstruction::building_wall_cache::regular_wall_height_cache(
                        &positions,
                        &shape,
                        float(p[2]),
                    )
                    .unwrap();
                println!(
                    "{:08x} {}",
                    cache.maximum_y.to_bits(),
                    cache
                        .arch_segment_heights
                        .iter()
                        .map(|f| format!("{:08x}", f.to_bits()))
                        .collect::<Vec<_>>()
                        .join(",")
                );
            }
            "gable_max" => {
                let roof = decode(p[2]);
                println!(
                    "{:08x}",
                    tiny_glade_reconstruction::building_wall_cache::gable_maximum(
                        float(p[1]),
                        Some(&roof)
                    )
                    .to_bits()
                );
            }
            "projection" => {
                let mut bytes = decode(p[1]);
                let shape = wall::WallShapeSnapshot::from_observed(decode(p[2]));
                let metrics = publication::WallPublicationMetrics {
                    minimum_relative_height: float(p[6]),
                    minimum_position_y: float(p[7]),
                    maximum_position_y: float(p[8]),
                    constrained_extra: float(p[9]),
                };
                publication::WallPublicationCore {
                    source_point_count: u64::from_str_radix(p[3], 16).unwrap(),
                    source_curve_length: float(p[4]),
                    shape: &shape,
                    style: p[5].parse().unwrap(),
                    metrics,
                }
                .write_projection(&mut bytes);
                println!("{}", hex(&bytes));
            }
            "cycle" => {
                use tiny_glade_reconstruction::{
                    building_roof_creation::RoofCreateRequestObserved,
                    building_roof_lifecycle::ResolvedRoofLifecycle,
                };
                let request = RoofCreateRequestObserved::from_observed(decode(p[1]));
                let public = wall::PublicWallBuildingState::from_observed(decode(p[2]));
                let mut lifecycle = ResolvedRoofLifecycle::prepare(
                    0x100000029,
                    decode(p[3]),
                    decode(p[4]),
                    &request,
                    &public,
                )
                .unwrap();
                lifecycle.advance(float(p[5]), 1);
                let marking = lifecycle
                    .consume_delete_requests(&[request.wall_id()])
                    .unwrap();
                let grown = hex(lifecycle.components.animation.bytes());
                lifecycle.deletion_mark_committed();
                let removed = lifecycle.advance(float(p[6]), 2).is_some();
                println!(
                    "{} {} {} {} {} {} {:08x}",
                    hex(&lifecycle.components.roof),
                    hex(lifecycle.components.tile_simulation.bytes()),
                    grown,
                    hex(lifecycle.components.animation.bytes()),
                    marking.sound_event_kind,
                    u8::from(removed),
                    lifecycle.animation_tick
                );
            }
            "tile" => {
                let v = components::RoofTileSimulation::initialize(decode(p[1]), float(p[2]));
                println!("{}", hex(v.bytes()));
            }
            "animation" => {
                let mut a = components::RoofAnimation::from_observed(decode(p[1]));
                let roof = decode(p[2]);
                let marked = p[3] == "1";
                let mut tick = 0xaaaaaaaa;
                a.advance(&roof, marked, float(p[4]), &mut tick, 0x12345678);
                println!(
                    "{} {tick:08x} {} {}",
                    hex(a.bytes()),
                    u8::from(a.is_being_animated()),
                    u8::from(deletion::should_despawn(marked, &a))
                );
            }
            "association" => {
                let w = if p[1] == "none" {
                    None
                } else {
                    Some(wall::PublicWallBuildingState::from_observed(decode(p[1])))
                };
                let phase = p[3].parse().unwrap();
                let result =
                    associations::association_action(w.as_ref(), p[2] == "1", phase, p[4] == "1");
                let text = match result {
                    Ok(associations::AssociationAction::Insert) => "insert",
                    Ok(associations::AssociationAction::KeepDeleting) => "keep",
                    Ok(associations::AssociationAction::RequestDeletion) => "delete",
                    Err(_) => "panic",
                };
                println!("{text}");
            }
            "publication" => {
                let extra = float(p[1]);
                let special = p[2] == "1";
                let values: Vec<f32> = p[3..].iter().map(|s| float(s)).collect();
                let relative: Vec<_> = values.chunks_exact(2).map(|v| v[0]).collect();
                let points: Vec<_> = values.chunks_exact(2).map(|v| [0., v[1], 0.]).collect();
                let m = publication::publication_metrics(&relative, &points, extra, special);
                println!(
                    "{:08x} {:08x} {:08x} {:08x}",
                    m.minimum_relative_height.to_bits(),
                    m.minimum_position_y.to_bits(),
                    m.maximum_position_y.to_bits(),
                    m.constrained_extra.to_bits()
                );
            }
            "mark" => {
                let roof = decode(p[1]);
                let ids: Vec<_> = p[3..]
                    .iter()
                    .map(|s| wall::WallId(u64::from_str_radix(s, 16).unwrap()))
                    .collect();
                if let Some(mark) = deletion::match_deletion_request(
                    &ids,
                    u64::from_str_radix(p[2], 16).unwrap(),
                    &roof,
                ) {
                    println!(
                        "{:016x} {:016x} {:08x} {:08x} {:08x} {}",
                        mark.entity,
                        mark.wall_id.0,
                        mark.pivot[0].to_bits(),
                        mark.pivot[1].to_bits(),
                        mark.pivot[2].to_bits(),
                        mark.sound_event_kind
                    );
                } else {
                    println!("-");
                }
            }
            _ => panic!("unknown lifecycle probe"),
        }
    }
}
