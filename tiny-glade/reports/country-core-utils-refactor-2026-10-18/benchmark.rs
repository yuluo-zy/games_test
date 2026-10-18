//! 直接比较历史与当前 DirtyTiles 源码；只衡量矩形置位，不推断完整游戏帧率。
#![allow(dead_code)]
#[path = "benchmark-baseline.rs"]
mod baseline;
#[path = "../../crates/country-core/src/utils/dirty_tiles.rs"]
mod current;
#[path = "../../crates/country-core/src/utils/conservative_line_raster.rs"]
pub mod conservative_line_raster;
mod utils { pub use crate::conservative_line_raster; }
use std::{hint::black_box, time::Instant};
fn main() {
    const ITERATIONS: u32 = 100_000;
    for (name, rectangle) in [("full", [-65.0,-65.0,65.0,65.0]), ("medium",[-20.0,-20.0,20.0,20.0]), ("small",[-1.0,-1.0,1.0,1.0])] {
        let mut old_samples = Vec::new();
        let mut new_samples = Vec::new();
        for _ in 0..5 {
            let started = Instant::now();
            for _ in 0..ITERATIONS {
                let mut tiles = baseline::DirtyTiles::new_clear();
                tiles.rasterize_world_rect(black_box(rectangle));
                black_box(tiles.rows());
            }
            old_samples.push(started.elapsed().as_nanos() / u128::from(ITERATIONS));
            let started = Instant::now();
            for _ in 0..ITERATIONS {
                let mut tiles = current::DirtyTiles::new_clear();
                tiles.rasterize_world_rect(black_box(rectangle));
                black_box(tiles.rows());
            }
            new_samples.push(started.elapsed().as_nanos() / u128::from(ITERATIONS));
        }
        old_samples.sort_unstable(); new_samples.sort_unstable();
        println!("{name}: baseline={} ns/op current={} ns/op ratio={:.2}x", old_samples[2], new_samples[2], old_samples[2] as f64 / new_samples[2] as f64);
    }
}
