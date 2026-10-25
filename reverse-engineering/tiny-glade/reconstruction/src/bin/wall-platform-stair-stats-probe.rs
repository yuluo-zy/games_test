use std::io::{self, BufRead};
use tiny_glade_reconstruction::building_wall_segmentation::material_stats_for_update;
fn main() {
    for line in io::stdin().lock().lines() {
        let line = line.unwrap();
        let mut p = line.split_whitespace();
        let f = |s: &str| f32::from_bits(u32::from_str_radix(s, 16).unwrap());
        let length = f(p.next().unwrap());
        let special: u8 = p.next().unwrap().parse().unwrap();
        let count: usize = p.next().unwrap().parse().unwrap();
        let mut stats = material_stats_for_update(length, special, count);
        for delta in p {
            stats.add_stairs_length(f(delta));
        }
        println!(
            "{:08x}{:08x}{:08x}{:08x}",
            stats.regular_path_length.to_bits(),
            stats.special_path_length.to_bits(),
            stats.segmentation_count,
            stats.stairs_length.to_bits()
        );
    }
}
