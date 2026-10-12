pub use tiny_glade_reconstruction::*;
use tiny_glade_reconstruction::building_wall_platform_stair_rows::*;
use std::{
    io::{self, BufRead},
    sync::Arc,
};
fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}
fn main() {
    for line in io::stdin().lock().lines() {
        let line = line.unwrap();
        let mut p = line.split_whitespace();
        let parse = |v: &str| f32::from_bits(u32::from_str_radix(v, 16).unwrap());
        let shape = p.next().unwrap();
        let shape =
            building_wall_state::WallShapeSnapshot::from_observed(std::array::from_fn(|i| {
                u8::from_str_radix(&shape[i * 2..i * 2 + 2], 16).unwrap()
            }));
        let wall = building_wall_state::WallId(p.next().unwrap().parse().unwrap());
        let salt: u32 = p.next().unwrap().parse().unwrap();
        let varying = p.next().unwrap() == "1";
        let fix = p.next().unwrap() == "1";
        let minimum_y = parse(p.next().unwrap());
        let maximum_y = parse(p.next().unwrap());
        let maximum = parse(p.next().unwrap());
        let extra = parse(p.next().unwrap());
        let coordinates = std::array::from_fn(|_| {
            (
                p.next().unwrap().parse::<usize>().unwrap(),
                parse(p.next().unwrap()),
            )
        });
        let parts = p.collect::<Vec<_>>();
        let points = parts
            .chunks_exact(3)
            .map(|v| [parse(v[0]), parse(v[1]), parse(v[2])])
            .collect::<Vec<_>>();
        let encoded = (0..512 * 512)
            .map(|i| {
                ((if varying {
                    (i % 512) as u32 * 131 + (i / 512) as u32 * 71
                } else {
                    0
                }) + salt) as u16
            })
            .collect::<Vec<_>>();
        let terrain = building_wall_collection::TerrainHeightGrid512 {
            encoded: Arc::from(encoded),
            minimum_y,
            maximum_y,
            sampling_fix: fix,
        };
        let cell = building_wall_platform_stairs::platform_stair_cell(
            building_wall_platform_stairs::WallPlatformStairCellSource {
                prepared_positions: &points,
                coordinates,
                maximum_y: maximum,
                constrained_extra: extra,
                terrain: &terrain,
                index_offset: 0,
            },
        )
        .unwrap();
        match platform_stair_rows(&shape, wall, &cell) {
            Ok(rows) => println!(
                "{}",
                rows.iter().map(|r| hex(r)).collect::<Vec<_>>().join(";")
            ),
            Err(e) => println!("error {e:?}"),
        }
    }
}
