use std::io::{self, BufRead};
use tiny_glade_reconstruction::{
    building_wall_platform_segmentation::*, building_wall_platform_stairs::*,
    building_wall_segmentation::*, building_wall_state::PublicWallBuildingState, surface::Curve2,
};
fn h(v: &[u8]) -> String {
    v.iter().map(|b| format!("{b:02x}")).collect()
}
fn main() {
    for l in io::stdin().lock().lines() {
        let l = l.unwrap();
        let mut p = l.split_whitespace();
        let shape: u32 = p.next().unwrap().parse().unwrap();
        let special: u8 = p.next().unwrap().parse().unwrap();
        let max = f32::from_bits(u32::from_str_radix(p.next().unwrap(), 16).unwrap());
        let rc: usize = p.next().unwrap().parse().unwrap();
        let n: usize = p.next().unwrap().parse().unwrap();
        let nb: usize = p.next().unwrap().parse().unwrap();
        let mut f = || f32::from_bits(u32::from_str_radix(p.next().unwrap(), 16).unwrap());
        let raw = Curve2::try_new((0..rc).map(|_| [f(), f()]).collect(), false).unwrap();
        let mut positions = Vec::new();
        let mut us = Vec::new();
        let mut values = Vec::new();
        let mut arch = Vec::new();
        for _ in 0..n {
            positions.push([f(), f(), f()]);
            us.push(f());
            values.push(f());
            arch.push(f());
        }
        let blockers: Vec<_> = (0..nb)
            .map(|_| WallPlatformStairBlocker {
                coordinate: [f(), f()],
                width: f(),
            })
            .collect();
        let mut public = [0; 672];
        public[0x258..0x25c].copy_from_slice(&shape.to_le_bytes());
        public[0x264..0x268].copy_from_slice(&0.2f32.to_le_bytes());
        public[0x26c..0x270].copy_from_slice(&2f32.to_le_bytes());
        public[0x270..0x274].copy_from_slice(&3f32.to_le_bytes());
        public[0x28c..0x290].copy_from_slice(&raw.length.to_le_bytes());
        public[0x296] = special;
        public[0x108] = 1;
        public[0x10c..0x110].copy_from_slice(&max.to_le_bytes());
        let snap = PublicWallBuildingState::from_observed(public);
        let masks = wall_path_segmentation(WallSegmentationInput {
            prepared_positions: &positions,
            path_values: &values,
            shape: &snap.shape(),
            observed_special_flag: special,
        })
        .unwrap();
        let roof = [0; 88];
        let platform =
            split_platform_wall_masks(&public, &positions, &values, &masks, Some(&roof)).unwrap();
        match prepare_platform_stair_segments(WallPlatformStairSource {
            raw_curve: &raw,
            public: &public,
            prepared_positions: &positions,
            prepared_u: &us,
            segment_values: &values,
            arch_heights: &arch,
            steps: &platform.steps,
            blockers: &blockers,
        }) {
            Ok(segments) => println!(
                "{}",
                segments
                    .iter()
                    .map(|s| {
                        let mut b = Vec::new();
                        b.extend_from_slice(&(s.heights.len() as u64).to_le_bytes());
                        for x in &s.heights {
                            b.extend_from_slice(&x.to_le_bytes());
                        }
                        b.extend_from_slice(&(s.positions.len() as u64).to_le_bytes());
                        for v in &s.positions {
                            for x in v {
                                b.extend_from_slice(&x.to_le_bytes());
                            }
                        }
                        b.extend_from_slice(&(s.points_u.len() as u64).to_le_bytes());
                        for x in &s.points_u {
                            b.extend_from_slice(&x.to_le_bytes());
                        }
                        for i in s.source_indices {
                            b.extend_from_slice(&(i as u64).to_le_bytes());
                        }
                        b.extend_from_slice(&s.range.observed_bytes());
                        h(&b)
                    })
                    .collect::<Vec<_>>()
                    .join(";")
            ),
            Err(e) => println!("error {e:?}"),
        }
    }
}
