use std::io::{self, BufRead};
use std::sync::Arc;
use tiny_glade_reconstruction::building_wall_collection;
#[path = "../../building/wall_path_preparation.rs"]
mod preparation;
fn floats(s: &str) -> Vec<f32> {
    (0..s.len())
        .step_by(8)
        .map(|i| f32::from_bits(u32::from_str_radix(&s[i..i + 8], 16).unwrap()))
        .collect()
}
fn main() {
    for line in io::stdin().lock().lines() {
        let line = line.unwrap();
        let mut p = line.split_whitespace();
        let scalar = floats(p.next().unwrap())[0];
        let sampling = p.next().unwrap() == "1";
        // Probe provides explicit encoded input; production APIs do not create
        // this test resource or assume it is empty.
        let pattern = p.next().unwrap();
        let encoded: [Vec<u8>; 4] = if pattern.contains(':') {
            let patches: Vec<Vec<i32>> = pattern
                .split(',')
                .map(|patch| patch.split(':').map(|s| s.parse().unwrap()).collect())
                .collect();
            std::array::from_fn(|channel| {
                (0..512 * 512)
                    .map(|i| {
                        let x = i % 512;
                        let z = i / 512;
                        patches
                            .iter()
                            .filter(|fields| {
                                channel as i32 == fields[0]
                                    && (x as i32) >= fields[1]
                                    && (x as i32) < fields[2]
                                    && (z as i32) >= fields[3]
                                    && (z as i32) < fields[4]
                            })
                            .last()
                            .map_or(0, |fields| fields[5] as u8)
                    })
                    .collect()
            })
        } else {
            let code: u8 = pattern.parse().unwrap();
            std::array::from_fn(|_| {
                let mut v = vec![0; 512 * 512];
                v[0] = code;
                v
            })
        };
        let paths =
            preparation::PathRasters512::from_encoded(encoded.map(Arc::from), sampling).unwrap();
        let samples: Vec<_> = p.map(floats).collect();
        let points: Vec<_> = samples.iter().map(|v| [v[0], v[1], v[2]]).collect();
        let relative: Vec<_> = samples.iter().map(|v| v[3]).collect();
        match preparation::prepare_wall_path(&points, &relative, scalar, &paths) {
            Ok(out) => println!(
                "{} {} {:08x} {} {}",
                out.positions.len(),
                out.segment_values.len(),
                out.length.to_bits(),
                out.positions
                    .iter()
                    .flatten()
                    .chain(out.points_u.iter())
                    .map(|v| format!("{:08x}", v.to_bits()))
                    .collect::<Vec<_>>()
                    .join(""),
                out.segment_values
                    .iter()
                    .map(|v| format!("{:08x}", v.to_bits()))
                    .collect::<Vec<_>>()
                    .join("")
            ),
            Err(e) => println!("error {e:?}"),
        }
    }
}
