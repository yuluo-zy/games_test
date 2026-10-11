use std::{
    io::{self, BufRead},
    sync::Arc,
};
use tiny_glade_reconstruction::{
    building_wall_collection::TerrainHeightGrid512, building_wall_platform_segmentation::*,
    building_wall_platform_stairs::*, building_wall_segmentation::*,
    building_wall_state::PublicWallBuildingState, surface::Curve2,
};
fn main() {
    for line in io::stdin().lock().lines() {
        let line = line.unwrap();
        let mut p = line.split_whitespace();
        let parse = |s: &str| f32::from_bits(u32::from_str_radix(s, 16).unwrap());
        let shape: u32 = p.next().unwrap().parse().unwrap();
        let special: u8 = p.next().unwrap().parse().unwrap();
        let maximum = parse(p.next().unwrap());
        let extra = parse(p.next().unwrap());
        let salt: u32 = p.next().unwrap().parse().unwrap();
        let varying = p.next().unwrap() == "1";
        let fix = p.next().unwrap() == "1";
        let minimum_y = parse(p.next().unwrap());
        let maximum_y = parse(p.next().unwrap());
        let rc: usize = p.next().unwrap().parse().unwrap();
        let n: usize = p.next().unwrap().parse().unwrap();
        let nr: usize = p.next().unwrap().parse().unwrap();
        let raw = Curve2::try_new(
            (0..rc)
                .map(|_| [parse(p.next().unwrap()), parse(p.next().unwrap())])
                .collect(),
            false,
        )
        .unwrap();
        let mut positions = Vec::new();
        let mut us = Vec::new();
        let mut values = Vec::new();
        let mut arch = Vec::new();
        for _ in 0..n {
            positions.push(std::array::from_fn(|_| parse(p.next().unwrap())));
            us.push(parse(p.next().unwrap()));
            values.push(parse(p.next().unwrap()));
            arch.push(parse(p.next().unwrap()));
        }
        let rows: Vec<_> = (0..nr)
            .map(|_| {
                let s = p.next().unwrap();
                WallPlatformDecoratorObserved(std::array::from_fn(|i| {
                    u8::from_str_radix(&s[i * 2..i * 2 + 2], 16).unwrap()
                }))
            })
            .collect();
        let encoded: Vec<u16> = (0..512 * 512)
            .map(|i| {
                ((if varying {
                    (i % 512) as u32 * 131 + (i / 512) as u32 * 71
                } else {
                    0
                }) + salt) as u16
            })
            .collect();
        let terrain = TerrainHeightGrid512 {
            encoded: Arc::from(encoded),
            minimum_y,
            maximum_y,
            sampling_fix: fix,
        };
        let mut public = [0; 672];
        public[0x258..0x25c].copy_from_slice(&shape.to_le_bytes());
        public[0x25c..0x260].copy_from_slice(&1.0f32.to_le_bytes());
        public[0x26c..0x270].copy_from_slice(&6f32.to_le_bytes());
        public[0x270..0x274].copy_from_slice(&4f32.to_le_bytes());
        public[0x28c..0x290].copy_from_slice(&raw.length.to_le_bytes());
        public[0x296] = special;
        public[0x108] = 1;
        public[0x10c..0x110].copy_from_slice(&maximum.to_le_bytes());
        public[0x284..0x288].copy_from_slice(&extra.to_le_bytes());
        let result = (|| {
            let blockers = collect_platform_stair_blockers(&raw, &terrain, &rows)?;
            let snap = PublicWallBuildingState::from_observed(public);
            let masks = wall_path_segmentation(WallSegmentationInput {
                prepared_positions: &positions,
                path_values: &values,
                shape: &snap.shape(),
                observed_special_flag: special,
            })
            .map_err(WallPlatformStairError::Segmentation)?;
            let platform =
                split_platform_wall_masks(&public, &positions, &values, &masks, Some(&[0; 88]))
                    .unwrap();
            let segments = prepare_platform_stair_segments(WallPlatformStairSource {
                raw_curve: &raw,
                public: &public,
                prepared_positions: &positions,
                prepared_u: &us,
                segment_values: &values,
                arch_heights: &arch,
                steps: &platform.steps,
                blockers: &blockers,
            })?;
            let geometry = platform_stair_geometry(&public, &positions, &us, &segments, &terrain)?;
            let holes = platform_stair_holes(&raw, maximum, &segments)?;
            let mut records = Vec::new();
            for cell in &geometry.cells {
                records.extend(tiny_glade_reconstruction::building_wall_platform_stair_rows::platform_stair_rows(&snap.shape(),tiny_glade_reconstruction::building_wall_state::WallId(27),cell).map_err(|_|WallPlatformStairError::InvalidInput)?);
            }
            let length: f32 = positions
                .windows(2)
                .map(|p| (glam::Vec3::from_array(p[0]) - glam::Vec3::from_array(p[1])).length())
                .sum();
            let mut stats = material_stats_for_update(length, special, masks.len());
            stats.add_stairs_length(geometry.stairs_length);
            Ok::<_, WallPlatformStairError>((geometry, holes, records, stats))
        })();
        match result {
            Ok((output, holes, records, stats)) => {
                let mut b = output.stairs_length.to_le_bytes().to_vec();
                b.extend_from_slice(&(output.cells.len() as u64).to_le_bytes());
                for cell in output.cells {
                    for x in [cell.top, cell.render_bottom] {
                        b.extend_from_slice(&x.to_le_bytes());
                    }
                    b.extend_from_slice(&cell.rise_count.to_le_bytes());
                    b.extend_from_slice(&cell.depth.to_le_bytes());
                    for v in cell.vertices {
                        for x in v {
                            b.extend_from_slice(&x.to_le_bytes());
                        }
                    }
                    for i in cell.indices {
                        b.extend_from_slice(&i.to_le_bytes());
                    }
                }
                b.extend_from_slice(&(holes.len() as u64).to_le_bytes());
                for hole in holes {
                    for x in hole.center.into_iter().chain(hole.dimensions) {
                        b.extend_from_slice(&x.to_le_bytes());
                    }
                    b.push(hole.kind);
                }
                b.extend_from_slice(&(records.len() as u64).to_le_bytes());
                for record in records {
                    b.extend_from_slice(&record);
                }
                for x in [
                    stats.regular_path_length,
                    stats.special_path_length,
                    stats.stairs_length,
                ] {
                    b.extend_from_slice(&x.to_le_bytes());
                }
                b.extend_from_slice(&stats.segmentation_count.to_le_bytes());
                println!(
                    "{}",
                    b.iter().map(|x| format!("{x:02x}")).collect::<String>()
                );
            }
            Err(e) => println!("error {e:?}"),
        }
    }
}
