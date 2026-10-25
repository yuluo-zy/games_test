use std::{
    io::{self, BufRead},
    sync::Arc,
};
use tiny_glade_reconstruction::{
    building_wall_collection, building_wall_height_command::*, surface,
};
fn decode<const N: usize>(s: &str) -> [u8; N] {
    std::array::from_fn(|i| u8::from_str_radix(&s[i * 2..i * 2 + 2], 16).unwrap())
}
fn hex(b: &[u8]) -> String {
    b.iter().map(|v| format!("{v:02x}")).collect()
}
fn main() {
    for line in io::stdin().lock().lines() {
        let line = line.unwrap();
        let p: Vec<_> = line.split_whitespace().collect();
        let request = AdjustWallHeightRequest::from_observed(&decode(p[0]));
        let f = |s: &str| f32::from_bits(u32::from_str_radix(s, 16).unwrap());
        let mut height = f(p[1]);
        let kind = p[2].parse().unwrap();
        let roof = if p[4] == "none" { None } else { Some(f(p[4])) };
        let clamp = if p[3] == "0" {
            None
        } else {
            Some(ResolvedWallHeightClamp {
                roof_height_01: roof,
                special_flag: p[5].parse().unwrap(),
            })
        };
        let curve = surface::Curve2 {
            points: vec![[f(p[8]), f(p[9])], [f(p[10]), f(p[11])]],
            points_u: vec![0., 1.],
            length: 1.,
        };
        let terrain = building_wall_collection::TerrainHeightGrid512 {
            encoded: Arc::from(vec![0; 512 * 512]),
            minimum_y: f(p[7]),
            maximum_y: f(p[7]),
            sampling_fix: false,
        };
        let outcome = apply_resolved_height_command(
            &request,
            &mut height,
            kind,
            &curve,
            &terrain,
            clamp,
            f(p[6]),
        );
        match outcome {
            Err(e) => println!("error {e:?}"),
            Ok(None) => println!("none {:08x}", height.to_bits()),
            Ok(Some(o)) => {
                let anchor = o
                    .anchor_world
                    .iter()
                    .flat_map(|v| v.to_le_bytes())
                    .collect::<Vec<_>>();
                let feedback = match o.feedback {
                    WallHeightFeedback::WallMaterial {
                        observed_kind,
                        observed_flag,
                        ..
                    } => format!("M{observed_kind}:{observed_flag}"),
                    WallHeightFeedback::Building {
                        observed_kind,
                        intensity,
                        ..
                    } => format!("B{observed_kind}:{:08x}", intensity.to_bits()),
                };
                println!(
                    "some {:08x} {} {} {}",
                    height.to_bits(),
                    o.authority_updates.len(),
                    hex(&anchor),
                    feedback
                );
            }
        }
    }
}
