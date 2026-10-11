use tiny_glade_reconstruction::{building_wall_constructor, surface};
#[path = "../../building/wall_production.rs"]
mod wall_production;
use std::io::{self, BufRead};
use surface::Curve2;
use wall_production::*;
fn bytes(s: &str) -> Vec<u8> {
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16).unwrap())
        .collect()
}
fn f(b: &[u8], i: usize) -> f32 {
    f32::from_le_bytes(b[i..i + 4].try_into().unwrap())
}
fn hex(b: &[u8]) -> String {
    b.iter().map(|v| format!("{v:02x}")).collect()
}
fn main() {
    for l in io::stdin().lock().lines() {
        let l = l.unwrap();
        let mut p = l.split_whitespace();
        let first = p.next().unwrap();
        let mut branch = WallProductionBranch {
            kind: 2,
            top: WallTopBounds::PLAIN,
            fence_fit: false,
        };
        let (kind, seed) = if first == "variant" {
            let kind = p.next().unwrap().parse::<u8>().unwrap();
            let b = bytes(p.next().unwrap());
            branch = WallProductionBranch {
                kind,
                top: WallTopBounds {
                    height: f(&b, 0),
                    intervals: [[f(&b, 4), f(&b, 8)], [f(&b, 12), f(&b, 16)]],
                    tag: b[20],
                },
                fence_fit: b[21] != 0,
            };
            (kind, p.next().unwrap().parse::<u64>().unwrap())
        } else if first == "kind" {
            (
                p.next().unwrap().parse::<u8>().unwrap(),
                p.next().unwrap().parse::<u64>().unwrap(),
            )
        } else {
            (2, first.parse::<u64>().unwrap())
        };
        let source_id = p.next().unwrap().parse().unwrap();
        let length = p.next().unwrap().parse().unwrap();
        let holes_length = p.next().unwrap().parse().unwrap();
        let closed = p.next().unwrap() == "1";
        let n: usize = p.next().unwrap().parse().unwrap();
        let cfg = bytes(p.next().unwrap());
        let samples: Vec<_> = (0..n).map(|_| bytes(p.next().unwrap())).collect();
        let curve = Curve2 {
            points: samples.iter().map(|b| [f(b, 0), f(b, 4)]).collect(),
            points_u: samples.iter().map(|b| f(b, 8)).collect(),
            length,
        };
        let arch: Vec<_> = samples.iter().map(|b| f(b, 12)).collect();
        let second: Vec<_> = samples.iter().map(|b| f(b, 16)).collect();
        let third: Vec<_> = samples.iter().map(|b| f(b, 20)).collect();
        let holes: Vec<_> = p
            .map(|v| {
                let b = bytes(v);
                (std::array::from_fn(|i| f(&b, i * 4)), b[16])
            })
            .collect();
        let input = PlainWallProductionInput {
            curve: &curve,
            arch_heights: &arch,
            second_heights: &second,
            third_heights: &third,
            holes: &holes,
            holes_length,
            closed,
            seed,
            brick_source_id: source_id,
            base: f(&cfg, 0),
            row_end_perturbation: f(&cfg, 4),
            width_padding: if cfg[20] != 0 { Some(f(&cfg, 8)) } else { None },
            thickness_scale: if cfg[21] != 0 {
                Some(f(&cfg, 12))
            } else {
                None
            },
            wall_float: f(&cfg, 16),
        };
        match if first == "variant" {
            produce_wall(input, branch)
        } else if kind == 2 {
            produce_plain_wall(input)
        } else {
            produce_wall_kind(input, kind)
        } {
            Ok(out) => println!(
                "{} {}",
                out.len(),
                out.into_iter()
                    .map(|v| hex(&v.observed_bytes()))
                    .collect::<Vec<_>>()
                    .join(",")
            ),
            Err(e) => println!("error {e:?}"),
        }
    }
}
