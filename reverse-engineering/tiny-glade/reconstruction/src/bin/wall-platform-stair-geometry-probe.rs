use std::{
    io::{self, BufRead},
    sync::Arc,
};
use tiny_glade_reconstruction::{
    building_wall_collection::TerrainHeightGrid512, building_wall_platform_stairs::*,
};
fn main() {
    for line in io::stdin().lock().lines() {
        let line = line.unwrap();
        let mut p = line.split_whitespace();
        let salt: u32 = p.next().unwrap().parse().unwrap();
        let varying = p.next().unwrap() == "1";
        let fix = p.next().unwrap() == "1";
        let parse = |s: &str| f32::from_bits(u32::from_str_radix(s, 16).unwrap());
        let minimum_y = parse(p.next().unwrap());
        let maximum_y = parse(p.next().unwrap());
        let maximum = parse(p.next().unwrap());
        let extra = parse(p.next().unwrap());
        let offset: u32 = p.next().unwrap().parse().unwrap();
        let coords = std::array::from_fn(|_| {
            (
                p.next().unwrap().parse::<usize>().unwrap(),
                parse(p.next().unwrap()),
            )
        });
        let points: Vec<_> = p
            .collect::<Vec<_>>()
            .chunks_exact(3)
            .map(|v| [parse(v[0]), parse(v[1]), parse(v[2])])
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
        match platform_stair_cell(WallPlatformStairCellSource {
            prepared_positions: &points,
            coordinates: coords,
            maximum_y: maximum,
            constrained_extra: extra,
            terrain: &terrain,
            index_offset: offset,
        }) {
            Ok(cell) => {
                let mut bytes = Vec::new();
                for x in [cell.top, cell.render_bottom] {
                    bytes.extend_from_slice(&x.to_le_bytes());
                }
                bytes.extend_from_slice(&cell.rise_count.to_le_bytes());
                bytes.extend_from_slice(&cell.depth.to_le_bytes());
                for v in cell.vertices {
                    for x in v {
                        bytes.extend_from_slice(&x.to_le_bytes());
                    }
                }
                for i in cell.indices {
                    bytes.extend_from_slice(&i.to_le_bytes());
                }
                println!(
                    "{}",
                    bytes.iter().map(|b| format!("{b:02x}")).collect::<String>()
                );
            }
            Err(e) => println!("error {e:?}"),
        }
    }
}
