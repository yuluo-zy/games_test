pub use tiny_glade_reconstruction::{building_wall_segmentation, building_wall_state};
#[path = "../../building/wall_structures.rs"]
mod wall_structures;
use building_wall_segmentation::*;
use std::io::{self, BufRead};
use wall_structures::*;
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
fn canonical(s: &MaterializedWallSegment) -> Vec<u8> {
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
    for index in s.source_indices {
        b.extend_from_slice(&(index as u64).to_le_bytes());
    }
    b.extend_from_slice(&s.range.observed_bytes());
    b
}
fn main() {
    for line in io::stdin().lock().lines() {
        let line = line.unwrap();
        let w: Vec<_> = line.split_whitespace().collect();
        if w[0] == "chain" {
            let shape_tag: u32 = w[1].parse().unwrap();
            let mut public = [0; 672];
            public[0x258..0x25c].copy_from_slice(&shape_tag.to_le_bytes());
            for (at, v) in [
                (0x25c, 1.0f32),
                (0x260, 0.),
                (0x264, 0.2),
                (0x268, 0.3),
                (0x26c, 2.),
                (0x270, 3.),
            ] {
                public[at..at + 4].copy_from_slice(&v.to_le_bytes());
            }
            public[0x296] = 1;
            public[0x108] = 1;
            public[0x10c..0x110].copy_from_slice(&floats(w[2])[0].to_le_bytes());
            let positions: Vec<[f32; 3]> = floats(w[6])
                .chunks_exact(3)
                .map(|p| p.try_into().unwrap())
                .collect();
            let us = floats(w[7]);
            let values = floats(w[8]);
            let arch = floats(w[9]);
            let mut third = floats(w[10]);
            let shape = building_wall_state::PublicWallBuildingState::from_observed(public).shape();
            let masks = wall_path_segmentation(WallSegmentationInput {
                prepared_positions: &positions,
                path_values: &values,
                shape: &shape,
                observed_special_flag: 1,
            })
            .unwrap();
            let roof = [0; 88];
            let processed=tiny_glade_reconstruction::building_wall_platform_segmentation::split_platform_wall_masks(&public,&positions,&values,&masks,Some(&roof)).unwrap();
            let mut rows = Vec::new();
            for mask in &processed.processed_masks {
                rows.extend(
                    construct_lintel_supports(
                        WallSegmentSource {
                            mask,
                            segment_values: &values,
                            arch_heights: &arch,
                            prepared_positions: &positions,
                            prepared_u: &us,
                            regular: shape_tag < 2,
                            closed: shape_tag != 2,
                        },
                        floats(w[3])[0],
                        floats(w[4])[0],
                        w[5].parse().unwrap(),
                        &mut third,
                    )
                    .unwrap(),
                );
            }
            let thirdbytes: Vec<_> = third.iter().flat_map(|x| x.to_le_bytes()).collect();
            println!(
                "{} {} {}",
                rows.len(),
                rows.iter()
                    .map(|r| hex(&r.observed_bytes()))
                    .collect::<Vec<_>>()
                    .join(","),
                hex(&thirdbytes)
            );
            continue;
        }
        if w[0] == "support" {
            let heights = floats(w[3]);
            let ground_positions: Vec<[f32; 3]> = floats(w[4])
                .chunks_exact(3)
                .map(|p| p.try_into().unwrap())
                .collect();
            let raised_positions: Vec<[f32; 3]> = floats(w[5])
                .chunks_exact(3)
                .map(|p| p.try_into().unwrap())
                .collect();
            let n = heights.len();
            let mut flags = vec![0; n];
            flags[0] = 8;
            flags[n - 1] = 8;
            let mut source_coords = vec![
                ThresholdedCurveCoord {
                    point_index: 0,
                    fraction: 0.
                };
                n
            ];
            source_coords[n - 1] = ThresholdedCurveCoord {
                point_index: (n - 2) as u32,
                fraction: 1.,
            };
            let source = LintelSupportSource {
                heights,
                ground_positions,
                raised_positions,
                flags,
                source_coords,
            };
            let rows =
                lintel_support_instances(&source, w[1].parse().unwrap(), w[2].parse().unwrap());
            println!(
                "{} {}",
                rows.len(),
                rows.iter()
                    .map(|r| hex(&r.observed_bytes()))
                    .collect::<Vec<_>>()
                    .join(",")
            );
            continue;
        }
        if w[0] == "rdp" {
            let points: Vec<[f32; 3]> = floats(w[1])
                .chunks_exact(3)
                .map(|p| p.try_into().unwrap())
                .collect();
            let indices = rdp_support_indices(&points);
            println!(
                "{}",
                indices
                    .iter()
                    .map(usize::to_string)
                    .collect::<Vec<_>>()
                    .join(",")
            );
            continue;
        }
        let full = w[0] == "lintel";
        let w = if full { &w[1..] } else { &w[..] };
        let flags = bytes(w[5]);
        let coordbytes = bytes(w[6]);
        let coords = coordbytes
            .chunks_exact(8)
            .map(|b| ThresholdedCurveCoord {
                point_index: u32::from_le_bytes(b[..4].try_into().unwrap()),
                fraction: f32::from_le_bytes(b[4..].try_into().unwrap()),
            })
            .collect();
        let mask = WallPathSegmentationMask {
            flags,
            coords,
            range: WallLoopRange {
                start: w[2].parse().unwrap(),
                end: w[3].parse().unwrap(),
                modulus: w[4].parse().unwrap(),
            },
            length: 0.,
        };
        let positions: Vec<[f32; 3]> = floats(w[7])
            .chunks_exact(3)
            .map(|p| p.try_into().unwrap())
            .collect();
        let us = floats(w[8]);
        let values = floats(w[9]);
        let arch = floats(w[10]);
        if full {
            let mut third = floats(w[14]);
            let input = WallSegmentSource {
                mask: &mask,
                segment_values: &values,
                arch_heights: &arch,
                prepared_positions: &positions,
                prepared_u: &us,
                regular: w[1] == "1",
                closed: w[0] == "1",
            };
            match construct_lintel_supports(
                input,
                floats(w[11])[0],
                floats(w[12])[0],
                w[13].parse().unwrap(),
                &mut third,
            ) {
                Ok(rows) => {
                    let thirdbytes: Vec<_> = third.iter().flat_map(|x| x.to_le_bytes()).collect();
                    println!(
                        "{} {} {}",
                        rows.len(),
                        rows.iter()
                            .map(|r| hex(&r.observed_bytes()))
                            .collect::<Vec<_>>()
                            .join(","),
                        hex(&thirdbytes)
                    );
                }
                Err(e) => println!("error {e:?}"),
            }
            continue;
        }
        match materialize_wall_segments(WallSegmentSource {
            mask: &mask,
            segment_values: &values,
            arch_heights: &arch,
            prepared_positions: &positions,
            prepared_u: &us,
            regular: w[1] == "1",
            closed: w[0] == "1",
        }) {
            Ok(rows) => println!(
                "{} {}",
                rows.len(),
                rows.iter()
                    .map(|r| hex(&canonical(r)))
                    .collect::<Vec<_>>()
                    .join(",")
            ),
            Err(e) => println!("error {e:?}"),
        }
    }
}
