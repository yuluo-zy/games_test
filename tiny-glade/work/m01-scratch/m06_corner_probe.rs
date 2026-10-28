#[path = "D:/game/tiny-glade/crates/systems/wall-constructor/src/utils/wall_corners.rs"]
mod original_owner;
use std::io::{self, BufRead};
fn main() {
    for line in io::stdin().lock().lines() {
        let line = line.unwrap();
        let v: Vec<f32> = line.split_whitespace().map(|s| s.parse().unwrap()).collect();
        println!("{}", original_owner::check_angle(glam::Vec2::new(v[0],v[1]),glam::Vec2::new(v[2],v[3]),glam::Vec2::new(v[4],v[5])));
    }
}
