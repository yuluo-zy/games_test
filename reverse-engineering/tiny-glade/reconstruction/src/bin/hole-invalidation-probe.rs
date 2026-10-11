use std::io::{self, BufRead};
use tiny_glade_reconstruction::{
    building_wall_hole_invalidation::forward_recalculation, building_wall_state::WallId,
};
fn main() {
    for line in io::stdin().lock().lines() {
        let line = line.unwrap();
        let p: Vec<_> = line.split_whitespace().collect();
        let id = WallId(u64::from_str_radix(p[0], 16).unwrap());
        println!(
            "{}",
            forward_recalculation(id, p[1] == "1")
                .map(|id| format!("{:016x}", id.0))
                .unwrap_or_else(|| "-".into())
        );
    }
}
