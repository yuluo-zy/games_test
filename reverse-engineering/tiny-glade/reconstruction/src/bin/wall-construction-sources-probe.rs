use std::io::{self, BufRead};
use tiny_glade_reconstruction::{building_wall_gable, building_wall_state, roof_state, surface};
#[path = "../../building/wall_construction_sources.rs"]
mod sources;
use building_wall_state::{WallId, WallShapeSnapshot};
fn float(s: &str) -> f32 {
    f32::from_bits(u32::from_str_radix(s, 16).unwrap())
}
fn bytes<const N: usize>(s: &str) -> [u8; N] {
    std::array::from_fn(|i| u8::from_str_radix(&s[i * 2..i * 2 + 2], 16).unwrap())
}
fn main() {
    for line in io::stdin().lock().lines() {
        let line = line.unwrap();
        let mut p = line.split_whitespace();
        match p.next().unwrap() {
            "policy" => {
                let wall = WallId(p.next().unwrap().parse().unwrap());
                let count = p.next().unwrap().parse().unwrap();
                let length = float(p.next().unwrap());
                let clutter = p.next().unwrap().parse().unwrap();
                let style = p.next().unwrap().parse().unwrap();
                let special = p.next().unwrap().parse().unwrap();
                let base = float(p.next().unwrap());
                let extra = float(p.next().unwrap());
                let maximum = p.next().unwrap();
                let maximum = if maximum == "none" {
                    None
                } else {
                    Some(float(maximum))
                };
                let shape = WallShapeSnapshot::from_observed(bytes(p.next().unwrap()));
                let roof = p.next().unwrap();
                let roof = if roof == "none" {
                    None
                } else {
                    Some(bytes::<88>(roof))
                };
                let heights: Vec<_> = p.map(float).collect();
                match sources::construction_policy(
                    wall,
                    count,
                    length,
                    clutter,
                    style,
                    &shape,
                    base,
                    extra,
                    maximum,
                    roof.as_ref(),
                    special,
                ) {
                    Ok(policy) => {
                        let top = match policy.top {
                            sources::ConstructionTopPolicy::Plain => "2 - - - - -".into(),
                            sources::ConstructionTopPolicy::Gable {
                                height,
                                intervals,
                                tag,
                            } => format!(
                                "{tag} {:08x} {}",
                                height.to_bits(),
                                intervals
                                    .iter()
                                    .flatten()
                                    .map(|v| format!("{:08x}", v.to_bits()))
                                    .collect::<Vec<_>>()
                                    .join(" ")
                            ),
                        };
                        println!(
                            "{} {} {} {:08x} {:08x} {:08x} {} | {}",
                            policy.kind,
                            policy.seed,
                            policy.brick_source_id,
                            policy.base.to_bits(),
                            policy.row_end_perturbation.to_bits(),
                            policy.wall_float.to_bits(),
                            top,
                            policy
                                .constructor_arch_heights(&heights)
                                .iter()
                                .map(|v| format!("{:08x}", v.to_bits()))
                                .collect::<Vec<_>>()
                                .join(" ")
                        );
                    }
                    Err(error) => println!("error {error:?}"),
                }
            }
            "kind" => {
                let version = p.next().unwrap().parse().unwrap();
                let style = p.next().unwrap().parse().unwrap();
                let roof = match p.next().unwrap() {
                    "0" => sources::ConstructionRoof::Absent,
                    "1" => sources::ConstructionRoof::Pitched,
                    "2" => sources::ConstructionRoof::Flat { has_rim: true },
                    "3" => sources::ConstructionRoof::Flat { has_rim: false },
                    _ => panic!("roof predicate"),
                };
                println!("{}", sources::construction_kind(version, style, roof));
            }
            "seed" => {
                let wall = WallId(p.next().unwrap().parse().unwrap());
                let count = p.next().unwrap().parse().unwrap();
                println!(
                    "{} {}",
                    sources::source_seed(wall, count),
                    sources::brick_source_id(wall)
                );
            }
            "perturb" => {
                let tag = p.next().unwrap().parse().unwrap();
                let version = p.next().unwrap().parse().unwrap();
                let length = float(p.next().unwrap());
                println!(
                    "{:08x}",
                    sources::row_end_perturbation(tag, version, length).to_bits()
                );
            }
            "options" => {
                let tag: u32 = p.next().unwrap().parse().unwrap();
                let radius = float(p.next().unwrap());
                let mut shape = [0; 28];
                shape[..4].copy_from_slice(&tag.to_le_bytes());
                shape[12..16].copy_from_slice(&radius.to_le_bytes());
                let options =
                    sources::circle_construction_options(&WallShapeSnapshot::from_observed(shape));
                println!(
                    "{} {}",
                    options
                        .width_padding
                        .map_or("-".into(), |v| format!("{:08x}", v.to_bits())),
                    options
                        .thickness_scale
                        .map_or("-".into(), |v| format!("{:08x}", v.to_bits()))
                );
            }
            other => panic!("unknown source probe {other}"),
        }
    }
}
