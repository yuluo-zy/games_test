//! Small reproducible cold/warm-cache timing probe; fixture creation is not timed.
use anyhow::Result;
use asset_pipe::texture::compile_texture_assets;
use image::{Rgba, RgbaImage};
use std::{
    path::PathBuf,
    time::{Instant, SystemTime, UNIX_EPOCH},
};

struct Fixture(PathBuf);
impl Drop for Fixture {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

fn main() -> Result<()> {
    let fixture = Fixture(std::env::temp_dir().join(format!(
        "tg-pipeline-bench-{}-{}",
        std::process::id(),
        SystemTime::now().duration_since(UNIX_EPOCH)?.as_nanos()
    )));
    let source_root = fixture.0.join("assets-src/textures");
    std::fs::create_dir_all(&source_root)?;
    std::fs::write(
        source_root.join("assets.ron"),
        "(default:(compression:None))",
    )?;
    for index in 0..32 {
        RgbaImage::from_fn(256, 256, |x, y| Rgba([x as u8, y as u8, index, 255]))
            .save(source_root.join(format!("image-{index}.png")))?;
    }
    let started = Instant::now();
    let (names, _) = compile_texture_assets(&fixture.0)?;
    let cold = started.elapsed();
    let mut warm = Vec::new();
    for _ in 0..5 {
        let started = Instant::now();
        compile_texture_assets(&fixture.0)?;
        warm.push(started.elapsed());
    }
    warm.sort();
    println!(
        "files={} dimensions=256x256 compression=None cold_us={} warm_median_us={}",
        names.len(),
        cold.as_micros(),
        warm[warm.len() / 2].as_micros()
    );
    Ok(())
}
