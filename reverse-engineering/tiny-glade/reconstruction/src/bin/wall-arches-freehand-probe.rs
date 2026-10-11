pub use tiny_glade_reconstruction::{
    building_wall_construction_sources, building_wall_constructor, building_wall_segmentation,
    building_wall_state, building_wall_structures, surface,
};
#[path = "../../building/wall_arches.rs"]
pub mod building_wall_arches;
use building_wall_arches::*;
use building_wall_segmentation::*;
use std::io::{self, BufRead};
#[path = "../../building/wall_arches_freehand.rs"]
mod wall_arches_freehand;
use wall_arches_freehand::*;
fn bytes(s: &str) -> Vec<u8> {
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16).unwrap())
        .collect()
}
fn floats(s: &str) -> Vec<f32> {
    bytes(s)
        .chunks_exact(4)
        .map(|b| f32::from_le_bytes(b.try_into().unwrap()))
        .collect()
}
fn hex(b: &[u8]) -> String {
    b.iter().map(|x| format!("{x:02x}")).collect()
}
fn fh(f: &[f32]) -> String {
    hex(&f.iter().flat_map(|x| x.to_le_bytes()).collect::<Vec<_>>())
}
fn gate_canonical(r: &ConstructGateCmd120Projection) -> Vec<u8> {
    let s = &r.segment;
    let mut b = Vec::new();
    b.extend_from_slice(&(s.heights.len() as u64).to_le_bytes());
    for f in &s.heights {
        b.extend_from_slice(&f.to_le_bytes());
    }
    b.extend_from_slice(&(s.positions.len() as u64).to_le_bytes());
    for p in &s.positions {
        for f in p {
            b.extend_from_slice(&f.to_le_bytes());
        }
    }
    b.extend_from_slice(&(s.points_u.len() as u64).to_le_bytes());
    for f in &s.points_u {
        b.extend_from_slice(&f.to_le_bytes());
    }
    for i in s.source_indices {
        b.extend_from_slice(&(i as u64).to_le_bytes());
    }
    b.extend_from_slice(&s.range.observed_bytes());
    b.extend_from_slice(&r.wall_id.0.to_le_bytes());
    b.extend_from_slice(&r.ordinal.to_le_bytes());
    b
}
fn main() {
    for line in io::stdin().lock().lines() {
        let line = line.unwrap();
        let w: Vec<_> = line.split_whitespace().collect();
        if w[0] == "callback" {
            let p = floats(w[5]);
            let a = glam::Vec3::from_array(p[..3].try_into().unwrap());
            let b = glam::Vec3::from_array(p[3..].try_into().unwrap());
            let u = floats(w[6]);
            let r = emit_arch_brick(
                a,
                b,
                u.try_into().unwrap(),
                building_wall_state::WallId(w[1].parse().unwrap()),
                floats(w[2])[0],
                w[3].parse().unwrap(),
                w[4].parse().unwrap(),
            );
            println!("{}", hex(&r.observed_bytes()));
            continue;
        }
        let _shape_tag: u32 = w[1].parse().unwrap();
        let observed_294 = w[2].parse().unwrap();
        let has_roof = w[3] == "1";
        let public_284 = floats(w[4])[0];
        let wall_id = building_wall_state::WallId(w[5].parse().unwrap());
        let source_curve_length = floats(w[6])[0];
        let positions: Vec<[f32; 3]> = floats(w[7])
            .chunks_exact(3)
            .map(|p| p.try_into().unwrap())
            .collect();
        let us = floats(w[8]);
        let values = floats(w[9]);
        let mut arch = floats(w[10]);
        let mut third = floats(w[11]);
        let masks: Vec<_> = if w[12] == "none" {
            Vec::new()
        } else {
            w[12]
                .split(';')
                .map(|s| {
                    let x: Vec<_> = s.split(',').collect();
                    let r = bytes(x[2]);
                    let coords = bytes(x[1])
                        .chunks_exact(8)
                        .map(|c| ThresholdedCurveCoord {
                            point_index: u32::from_le_bytes(c[..4].try_into().unwrap()),
                            fraction: f32::from_le_bytes(c[4..].try_into().unwrap()),
                        })
                        .collect();
                    WallPathSegmentationMask {
                        flags: bytes(x[0]),
                        coords,
                        range: WallLoopRange {
                            start: i32::from_le_bytes(r[..4].try_into().unwrap()),
                            end: i32::from_le_bytes(r[4..8].try_into().unwrap()),
                            modulus: i32::from_le_bytes(r[8..].try_into().unwrap()),
                        },
                        length: floats(x[3])[0],
                    }
                })
                .collect()
        };
        let input = FreehandWallArchesInput {
            masks: &masks,
            prepared_positions: &positions,
            prepared_u: &us,
            path_values: &values,
            public_288: floats(w[13])[0],
            clutter_version: w[14].parse().unwrap(),
            public_284,
            observed_294,
            source_curve_length,
            wall_id,
            has_roof,
        };
        match construct_freehand_wall_arches(input, &mut arch, &mut third) {
            Ok(r) => println!(
                "{} {} {} {} {} {}",
                r.instances.len(),
                r.instances
                    .iter()
                    .map(|b| hex(&b.observed_bytes()))
                    .collect::<Vec<_>>()
                    .join(","),
                fh(&arch),
                fh(&third),
                r.segment_count,
                if r.gate_commands.is_empty() {
                    "none".to_string()
                } else {
                    r.gate_commands
                        .iter()
                        .map(|r| hex(&gate_canonical(r)))
                        .collect::<Vec<_>>()
                        .join(";")
                }
            ),
            Err(e) => println!("error {e:?}"),
        }
    }
}
