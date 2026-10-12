use std::{
    io::{self, BufRead},
    sync::Arc,
};
use tiny_glade_reconstruction::{
    building_wall_collection::{
        DirtyPublicationSubscription, RawWallPublicationSource, TerrainHeightGrid512,
        collect_raw_wall_publication, dirty_tile_candidate, terrain_requires_republication,
    },
    building_wall_state::WallShapeSnapshot,
    surface::Curve2,
};
fn main() {
    for line in io::stdin().lock().lines() {
        let line = line.unwrap();
        let w: Vec<_> = line.split_whitespace().collect();
        let f = |s: &str| f32::from_bits(u32::from_str_radix(s, 16).unwrap());
        if w[0] == "dirty" {
            let kind = if w[1] == "primary" {
                DirtyPublicationSubscription::Primary
            } else {
                DirtyPublicationSubscription::Secondary
            };
            println!(
                "{}",
                u8::from(dirty_tile_candidate(kind, f(w[2]), w[3] == "1"))
            );
            continue;
        }
        let grid: Vec<u16> = (0..512 * 512)
            .map(|i| ((i as u32 * 73 + (i as u32 / 512) * 101) & 65535) as u16)
            .collect();
        let terrain = TerrainHeightGrid512 {
            encoded: Arc::from(grid),
            minimum_y: f(w[2]),
            maximum_y: f(w[3]),
            sampling_fix: w[1] == "1",
        };
        if w[0] == "terrain" {
            println!(
                "{:08x}",
                terrain.sample_at_world_xz(f(w[4]), f(w[5])).to_bits()
            );
        } else if w[0] == "republication" {
            let outline: Vec<_> = w[5..]
                .chunks_exact(3)
                .map(|p| [f(p[0]), f(p[1]), f(p[2])])
                .collect();
            println!(
                "{}",
                u8::from(terrain_requires_republication(&outline, f(w[4]), &terrain))
            );
        } else {
            let points: Vec<_> = w[6..].chunks_exact(2).map(|p| [f(p[0]), f(p[1])]).collect();
            let source = RawWallPublicationSource {
                curve: Curve2::try_new(points, false).unwrap(),
                curve_kind_tag: w[4].parse().unwrap(),
                terrain_floor: f(w[5]),
                shape: WallShapeSnapshot::from_observed([0; 28]),
                style: 0,
                extra: 1.0,
                special: false,
                observed_path_scalar: 0.0,
                clutter_version: 0,
        observed_source_flag_64:0,
            };
            match collect_raw_wall_publication(&source, &terrain) {
                Ok(c) => {
                    print!(
                        "{} {:08x} {:08x} ",
                        c.outline.len(),
                        c.original_curve_length.to_bits(),
                        c.mapped_curve_length.to_bits()
                    );
                    for p in c.outline.iter() {
                        for v in p {
                            print!("{:08x}", v.to_bits());
                        }
                    }
                    print!(" ");
                    for u in c.outline_u {
                        print!("{:08x}", u.to_bits());
                    }
                    print!(" ");
                    for y in c.terrain_y {
                        print!("{:08x}", y.to_bits());
                    }
                    println!();
                }
                Err(e) => println!("error {e:?}"),
            }
        }
    }
}
