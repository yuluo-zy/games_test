use std::io::{self, BufRead};
use tiny_glade_reconstruction::building_wall_spatial_index::{
    segment_cells, world_space_to_grid_coord_unclamped,
};
fn main() {
    for line in io::stdin().lock().lines() {
        let line = line.unwrap();
        let w: Vec<_> = line.split_whitespace().collect();
        let f = |s: &str| f32::from_bits(u32::from_str_radix(s, 16).unwrap());
        if w[0] == "coord" {
            let [x, z] = world_space_to_grid_coord_unclamped([f(w[1]), f(w[2])]);
            println!("{x} {z}");
        } else {
            let cells = segment_cells([f(w[1]), f(w[2])], [f(w[3]), f(w[4])], f(w[5]));
            println!(
                "{}",
                cells
                    .iter()
                    .map(|c| c.to_string())
                    .collect::<Vec<_>>()
                    .join(",")
            );
        }
    }
}
