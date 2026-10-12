use std::io::{self, BufRead};
use tiny_glade_reconstruction::{building_wall_state, roof_state, surface};
#[path = "../../building/wall_segmentation.rs"]
mod building_wall_segmentation;
#[path = "../../building/wall_platform_segmentation.rs"]
mod platform;
fn hx(v: &[u8]) -> String {
    v.iter().map(|b| format!("{b:02x}")).collect()
}
fn main() {
    for line in io::stdin().lock().lines() {
        let line = line.unwrap();
        let mut p = line.split_whitespace();
        let shape_tag: u32 = p.next().unwrap().parse().unwrap();
        let special = p.next().unwrap().parse().unwrap();
        let max = f32::from_bits(u32::from_str_radix(p.next().unwrap(), 16).unwrap());
        let flag: u8 = p.next().unwrap().parse().unwrap();
        let roof_word = p.next().unwrap();
        let count: usize = p.next().unwrap().parse().unwrap();
        let mut public = [0; 672];
        public[0x258..0x25c].copy_from_slice(&shape_tag.to_le_bytes());
        public[0x264..0x268].copy_from_slice(&0.2f32.to_le_bytes());
        public[0x26c..0x270].copy_from_slice(&2f32.to_le_bytes());
        public[0x270..0x274].copy_from_slice(&3f32.to_le_bytes());
        public[0x296] = special;
        public[0x108] = flag;
        public[0x10c..0x110].copy_from_slice(&max.to_le_bytes());
        public[0x114..0x118].copy_from_slice(&99f32.to_le_bytes());
        let mut roof = [0; 88];
        let roof = if roof_word == "none" {
            None
        } else {
            roof[0x30..0x34]
                .copy_from_slice(&u32::from_str_radix(roof_word, 16).unwrap().to_le_bytes());
            Some(&roof)
        };
        let mut points = Vec::new();
        let mut vals = Vec::new();
        for _ in 0..count {
            let mut f = || f32::from_bits(u32::from_str_radix(p.next().unwrap(), 16).unwrap());
            points.push([f(), f(), f()]);
            vals.push(f());
        }
        let shape = building_wall_state::PublicWallBuildingState::from_observed(public).shape();
        let masks = building_wall_segmentation::wall_path_segmentation(
            building_wall_segmentation::WallSegmentationInput {
                prepared_positions: &points,
                path_values: &vals,
                shape: &shape,
                observed_special_flag: special,
            },
        )
        .unwrap();
        match platform::split_platform_wall_masks(&public, &points, &vals, &masks, roof) {
            Ok(out) => {
                let masks = out
                    .processed_masks
                    .iter()
                    .map(|m| {
                        format!(
                            "{},{},{},{:08x}",
                            hx(&m.flags),
                            m.coords
                                .iter()
                                .map(|v| hx(&v.observed_bytes()))
                                .collect::<String>(),
                            hx(&m.range.observed_bytes()),
                            m.length.to_bits()
                        )
                    })
                    .collect::<Vec<_>>()
                    .join(";");
                let steps = out
                    .steps
                    .iter()
                    .map(|s| {
                        format!(
                            "{},{},{}",
                            s.start,
                            s.end,
                            hx(&s.source_range.observed_bytes())
                        )
                    })
                    .collect::<Vec<_>>()
                    .join(";");
                println!("{masks}|{steps}");
            }
            Err(e) => println!("error {e:?}"),
        }
    }
}
