use std::{
    io::{self, BufRead},
    sync::Arc,
};
use tiny_glade_reconstruction::{
    building_wall_collection::TerrainHeightGrid512, building_wall_platform_stairs::*,
    surface::Curve2,
};
fn main() {
    for line in io::stdin().lock().lines() {
        let line = line.unwrap();
        let mut p = line.split_whitespace();
        let count: usize = p.next().unwrap().parse().unwrap();
        let salt: u32 = p.next().unwrap().parse().unwrap();
        let varying: bool = p.next().unwrap() == "1";
        let fix = p.next().unwrap() == "1";
        let parse = |s: &str| f32::from_bits(u32::from_str_radix(s, 16).unwrap());
        let minimum_y = parse(p.next().unwrap());
        let maximum_y = parse(p.next().unwrap());
        let points = (0..count)
            .map(|_| [parse(p.next().unwrap()), parse(p.next().unwrap())])
            .collect();
        let raw = Curve2::try_new(points, false).unwrap();
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
        let rows: Vec<_> = p
            .map(|s| {
                WallPlatformDecoratorObserved(std::array::from_fn(|i| {
                    u8::from_str_radix(&s[i * 2..i * 2 + 2], 16).unwrap()
                }))
            })
            .collect();
        match collect_platform_stair_blockers(&raw, &terrain, &rows) {
            Ok(values) => println!(
                "{}",
                values
                    .iter()
                    .map(|v| v
                        .coordinate
                        .iter()
                        .chain(std::iter::once(&v.width))
                        .flat_map(|x| x.to_le_bytes())
                        .map(|b| format!("{b:02x}"))
                        .collect::<String>())
                    .collect::<String>()
            ),
            Err(WallPlatformStairError::InvalidStairExtraInfo) => println!("invalid_extra"),
            Err(e) => println!("error {e:?}"),
        }
    }
}
