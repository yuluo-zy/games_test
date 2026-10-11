use std::io::{self, BufRead};
use tiny_glade_reconstruction::{building_wall_state, surface};
#[path = "../../building/wall_segmentation.rs"]
mod segmentation;
use segmentation::*;
fn main() {
    for line in io::stdin().lock().lines() {
        let line = line.unwrap();
        let mut p = line.split_whitespace();
        match p.next().unwrap() {
            "stats" => {
                let length = f32::from_bits(u32::from_str_radix(p.next().unwrap(), 16).unwrap());
                let special = p.next().unwrap().parse().unwrap();
                let count = p.next().unwrap().parse().unwrap();
                let stats = material_stats_for_update(length, special, count);
                println!(
                    "{:08x} {:08x} {}",
                    stats.regular_path_length.to_bits(),
                    stats.special_path_length.to_bits(),
                    stats.segmentation_count
                );
            }
            "range" => {
                let closed = p.next().unwrap() == "1";
                let start = p.next().unwrap().parse().unwrap();
                let end = p.next().unwrap().parse().unwrap();
                let count = p.next().unwrap().parse().unwrap();
                let range = if closed {
                    WallLoopRange::for_closed(start, end, count)
                } else {
                    WallLoopRange::for_open(start, end, count)
                };
                match range {
                    Ok(range) => println!(
                        "{} {}",
                        range.len(),
                        (0..range.len())
                            .map(|i| range.index_at(i).unwrap().to_string())
                            .collect::<Vec<_>>()
                            .join(",")
                    ),
                    Err(e) => println!("error {e:?}"),
                }
            }
            "coord" => {
                let index = p.next().unwrap().parse().unwrap();
                let fraction = f32::from_bits(u32::from_str_radix(p.next().unwrap(), 16).unwrap());
                let count = p.next().unwrap().parse().unwrap();
                match ThresholdedCurveCoord::for_point_count(index, fraction, count) {
                    Ok(coord) => println!("{} {:08x}", coord.point_index, coord.fraction.to_bits()),
                    Err(e) => println!("error {e:?}"),
                }
            }
            "full" => {
                let shape_tag: u32 = p.next().unwrap().parse().unwrap();
                let special = p.next().unwrap().parse().unwrap();
                let count: usize = p.next().unwrap().parse().unwrap();
                let fields: Vec<_> = (0..3)
                    .map(|_| f32::from_bits(u32::from_str_radix(p.next().unwrap(), 16).unwrap()))
                    .collect();
                let mut shape = [0; 28];
                shape[..4].copy_from_slice(&shape_tag.to_le_bytes());
                for (at, value) in [
                    (4, 1f32),
                    (8, 0f32),
                    (12, fields[0]),
                    (16, 0.3),
                    (20, fields[1]),
                    (24, fields[2]),
                ] {
                    shape[at..at + 4].copy_from_slice(&value.to_le_bytes());
                }
                let shape = building_wall_state::WallShapeSnapshot::from_observed(shape);
                let mut points = Vec::new();
                let mut values = Vec::new();
                for _ in 0..count {
                    let mut f =
                        || f32::from_bits(u32::from_str_radix(p.next().unwrap(), 16).unwrap());
                    points.push([f(), f(), f()]);
                    values.push(f());
                }
                match wall_path_segmentation(WallSegmentationInput {
                    prepared_positions: &points,
                    path_values: &values,
                    shape: &shape,
                    observed_special_flag: special,
                }) {
                    Ok(masks) => println!(
                        "{}",
                        masks
                            .iter()
                            .map(|m| format!(
                                "{},{},{},{:08x}",
                                m.flags
                                    .iter()
                                    .map(|v| format!("{v:02x}"))
                                    .collect::<String>(),
                                m.coords
                                    .iter()
                                    .map(|v| v
                                        .observed_bytes()
                                        .iter()
                                        .map(|b| format!("{b:02x}"))
                                        .collect::<String>())
                                    .collect::<String>(),
                                m.range
                                    .observed_bytes()
                                    .iter()
                                    .map(|b| format!("{b:02x}"))
                                    .collect::<String>(),
                                m.length.to_bits()
                            ))
                            .collect::<Vec<_>>()
                            .join(";")
                    ),
                    Err(e) => println!("error {e:?}"),
                }
            }
            "empty" => {
                let shape_tag: u32 = p.next().unwrap().parse().unwrap();
                let special = p.next().unwrap().parse().unwrap();
                let count: usize = p.next().unwrap().parse().unwrap();
                let mut shape = [0; 28];
                shape[..4].copy_from_slice(&shape_tag.to_le_bytes());
                shape[12..16].copy_from_slice(&1f32.to_le_bytes());
                shape[20..24].copy_from_slice(&2f32.to_le_bytes());
                shape[24..28].copy_from_slice(&3f32.to_le_bytes());
                let shape = building_wall_state::WallShapeSnapshot::from_observed(shape);
                let points: Vec<_> = (0..count).map(|i| [i as f32, 0., (i % 3) as f32]).collect();
                let values: Vec<_> = p
                    .map(|v| f32::from_bits(u32::from_str_radix(v, 16).unwrap()))
                    .collect();
                match wall_path_segmentation(WallSegmentationInput {
                    prepared_positions: &points,
                    path_values: &values,
                    shape: &shape,
                    observed_special_flag: special,
                }) {
                    Ok(masks) => println!("{}", masks.len()),
                    Err(e) => println!("error {e:?}"),
                }
            }
            other => panic!("unknown segmentation probe {other}"),
        }
    }
}
