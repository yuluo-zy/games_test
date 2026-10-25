#[path = "../../building/wall_constructor.rs"]
mod wall_constructor;
use std::{
    collections::VecDeque,
    io::{self, BufRead},
};
use wall_constructor::{cut_wall_bricks, WallBrick};
fn bytes(s: &str) -> Vec<u8> {
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16).unwrap())
        .collect()
}
fn f(b: &[u8], o: usize) -> f32 {
    f32::from_bits(u32::from_le_bytes(b[o..o + 4].try_into().unwrap()))
}
fn hex(b: &[u8]) -> String {
    b.iter().map(|v| format!("{v:02x}")).collect()
}
fn main() {
    for line in io::stdin().lock().lines() {
        let line = line.unwrap();
        let mut p = line.split_whitespace();
        let cmd = p.next().unwrap();
        if cmd == "partition" {
            let b = bytes(p.next().unwrap());
            let u = [f(&b, 0), f(&b, 4)];
            let length: f32 = p.next().unwrap().parse().unwrap();
            let seed: u64 = p.next().unwrap().parse().unwrap();
            let n: usize = p.next().unwrap().parse().unwrap();
            let i: usize = p.next().unwrap().parse().unwrap();
            match wall_constructor::split_plain_row_horizontal(u, length, seed, n, i) {
                Ok(out) => println!(
                    "{} {}",
                    out.len(),
                    out.iter()
                        .map(|v| hex(&v.to_le_bytes()))
                        .collect::<Vec<_>>()
                        .join(",")
                ),
                Err(e) => println!("error {e:?}"),
            }
            continue;
        }
        if cmd == "row-holes" {
            let b = bytes(p.next().unwrap());
            let row = std::array::from_fn(|i| f(&b, i * 4));
            let length: f32 = p.next().unwrap().parse().unwrap();
            let height: f32 = p.next().unwrap().parse().unwrap();
            let holes: Vec<_> = p
                .map(|v| {
                    let b = bytes(v);
                    std::array::from_fn(|i| f(&b, i * 4))
                })
                .collect();
            let out =
                wall_constructor::select_native_row_holes(row, &holes, length, height).unwrap();
            println!(
                "{} {}",
                out.source_indices
                    .iter()
                    .map(|v| v.to_string())
                    .collect::<Vec<_>>()
                    .join(","),
                out.suffix_min_x
                    .iter()
                    .map(|v| hex(&v.to_le_bytes()))
                    .collect::<Vec<_>>()
                    .join(",")
            );
            continue;
        }
        if cmd == "chain" {
            let raw = bytes(p.next().unwrap());
            let row_height = f(&raw, 0);
            let draw = f(&raw, 4);
            let b = bytes(p.next().unwrap());
            let brick = WallBrick {
                aabb: std::array::from_fn(|i| f(&b, i * 4)),
                deformation: std::array::from_fn(|i| f(&b, 16 + i * 4)),
                seed: u32::from_le_bytes(b[32..36].try_into().unwrap()),
                split_y: b[36],
                split_x: b[37],
            };
            let b = bytes(p.next().unwrap());
            let c = wall_constructor::WallEmissionContext {
                curve_ends: [[f(&b, 0), f(&b, 4)], [f(&b, 8), f(&b, 12)]],
                row_u: [f(&b, 16), f(&b, 20)],
                length: f(&b, 24),
                height_scale: f(&b, 28),
                width_padding: f(&b, 32),
                depth_axis: std::array::from_fn(|i| f(&b, 36 + i * 4)),
                surface: std::array::from_fn(|i| f(&b, 48 + i * 4)),
                material: u32::from_le_bytes(b[60..64].try_into().unwrap()),
                flags: 0,
            };
            let hs: Vec<_> = p
                .map(|v| {
                    let b = bytes(v);
                    std::array::from_fn(|i| f(&b, i * 4))
                })
                .collect();
            let out = wall_constructor::construct_resolved_plain_wall_proposal(
                brick, &hs, c, row_height, draw,
            )
            .unwrap();
            println!(
                "{} {}",
                out.len(),
                out.into_iter()
                    .map(|v| hex(&v.observed_bytes()))
                    .collect::<Vec<_>>()
                    .join(",")
            );
            continue;
        }
        if cmd == "proposal" {
            let b = bytes(p.next().unwrap());
            let brick = WallBrick {
                aabb: std::array::from_fn(|i| f(&b, i * 4)),
                deformation: std::array::from_fn(|i| f(&b, 16 + i * 4)),
                seed: u32::from_le_bytes(b[32..36].try_into().unwrap()),
                split_y: b[36],
                split_x: b[37],
            };
            let raw = bytes(p.next().unwrap());
            let hs: Vec<_> = p
                .map(|v| {
                    let b = bytes(v);
                    std::array::from_fn(|i| f(&b, i * 4))
                })
                .collect();
            match wall_constructor::consume_wall_proposal(
                brick,
                &hs,
                f(&raw, 0),
                f(&raw, 4),
                f(&raw, 8),
                raw[16] != 0,
                f(&raw, 12),
            ) {
                Ok(out) => println!(
                    "{} {}",
                    out.len(),
                    out.iter()
                        .map(|b| hex(&b.observed_bytes()))
                        .collect::<Vec<_>>()
                        .join(",")
                ),
                Err(e) => println!("error {e:?}"),
            }
            continue;
        }
        if cmd == "normalize" {
            let length: f32 = p.next().unwrap().parse().unwrap();
            let height: f32 = p.next().unwrap().parse().unwrap();
            let closed = p.next().unwrap() == "1";
            let holes: Vec<_> = p
                .map(|v| {
                    let b = bytes(v);
                    (std::array::from_fn(|i| f(&b, i * 4)), b[16])
                })
                .collect();
            match wall_constructor::normalize_wall_holes(&holes, length, height, closed) {
                Ok(out) => println!(
                    "{} {}",
                    out.len(),
                    out.into_iter()
                        .map(|a| hex(&a.into_iter().flat_map(f32::to_le_bytes).collect::<Vec<_>>()))
                        .collect::<Vec<_>>()
                        .join(",")
                ),
                Err(e) => println!("error {e:?}"),
            }
            continue;
        }
        if cmd == "emit" || cmd == "plain" {
            let b = bytes(p.next().unwrap());
            let brick = WallBrick {
                aabb: std::array::from_fn(|i| f(&b, i * 4)),
                deformation: std::array::from_fn(|i| f(&b, 16 + i * 4)),
                seed: u32::from_le_bytes(b[32..36].try_into().unwrap()),
                split_y: b[36],
                split_x: b[37],
            };
            let b = bytes(p.next().unwrap());
            let c = wall_constructor::WallEmissionContext {
                curve_ends: [[f(&b, 0), f(&b, 4)], [f(&b, 8), f(&b, 12)]],
                row_u: [f(&b, 16), f(&b, 20)],
                length: f(&b, 24),
                height_scale: f(&b, 28),
                width_padding: f(&b, 32),
                depth_axis: std::array::from_fn(|i| f(&b, 36 + i * 4)),
                surface: std::array::from_fn(|i| f(&b, 48 + i * 4)),
                material: u32::from_le_bytes(b[60..64].try_into().unwrap()),
                flags: u32::from_le_bytes(b[64..68].try_into().unwrap()),
            };
            match if cmd == "plain" {
                wall_constructor::emit_plain_wall_brick(brick, c)
            } else {
                wall_constructor::emit_resolved_wall_brick(brick, c)
            } {
                Some(v) => println!("{}", hex(&v.observed_bytes())),
                None => println!("skip"),
            }
            continue;
        }
        if cmd == "trim" {
            let scale: f32 = p.next().unwrap().parse().unwrap();
            let length: f32 = p.next().unwrap().parse().unwrap();
            let special = p.next().unwrap() == "1";
            let n: usize = p.next().unwrap().parse().unwrap();
            let bs: Vec<_> = (0..n).map(|_| f(&bytes(p.next().unwrap()), 0)).collect();
            let pairs: Vec<_> = p
                .map(|v| {
                    let b = bytes(v);
                    [f(&b, 0), f(&b, 4)]
                })
                .collect();
            let us: Vec<_> = pairs.iter().map(|p| p[0]).collect();
            let hs: Vec<_> = pairs.iter().map(|p| p[1]).collect();
            match wall_constructor::trim_rows(&bs, &us, &hs, scale, length, special) {
                Ok(out) => println!(
                    "{} {}",
                    out.len(),
                    out.iter()
                        .map(|v| hex(&v.observed_bytes()))
                        .collect::<Vec<_>>()
                        .join(",")
                ),
                Err(e) => println!("error {e:?}"),
            }
            continue;
        }
        if cmd == "rows" {
            let scale: f32 = p.next().unwrap().parse().unwrap();
            let seed: u64 = p.next().unwrap().parse().unwrap();
            let h: Vec<_> = p.map(|v| f(&bytes(v), 0)).collect();
            let out = wall_constructor::split_into_random_rows(&h, scale, seed);
            println!(
                "{} {}",
                out.len(),
                out.iter()
                    .map(|v| hex(&v.to_le_bytes()))
                    .collect::<Vec<_>>()
                    .join(",")
            );
            continue;
        }
        assert_eq!(cmd, "cut");
        let count: usize = p.next().unwrap().parse().unwrap();
        let holes: Vec<[f32; 4]> = (0..count)
            .map(|_| {
                let b = bytes(p.next().unwrap());
                std::array::from_fn(|i| f(&b, i * 4))
            })
            .collect();
        let bricks: VecDeque<_> = p
            .map(|v| {
                let b = bytes(v);
                WallBrick {
                    aabb: std::array::from_fn(|i| f(&b, i * 4)),
                    deformation: std::array::from_fn(|i| f(&b, 16 + i * 4)),
                    seed: u32::from_le_bytes(b[32..36].try_into().unwrap()),
                    split_y: b[36],
                    split_x: b[37],
                }
            })
            .collect();
        match cut_wall_bricks(bricks, &holes) {
            Ok(b) => println!(
                "{} {}",
                b.len(),
                b.into_iter()
                    .map(|v| hex(&v.observed_bytes()))
                    .collect::<Vec<_>>()
                    .join(",")
            ),
            Err(e) => println!("error {e:?}"),
        }
    }
}
