//! Render a text fragment and rounded rectangle into a reusable offscreen target.
use ash::vk;
use rhapsody::{
    interop::create_headless_device, resource::Texture, resource::TextureDesc,
    resource::init_texture_data,
};
use std::{path::PathBuf, sync::Arc};
use tg_text::{
    FontFamily, FontMetrics, FontTable, ShapeStream, TextError, TextFragment, TextRenderer,
    gpu::{GpuTextConfig, GpuTextRenderer},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = std::env::args_os().skip(1);
    let (Some(resources), Some(output)) = (args.next(), args.next()) else {
        return Err("usage: text_gpu <game-resource-directory> <output-directory>".into());
    };
    if args.next().is_some() {
        return Err("expected two arguments".into());
    }
    let resources = PathBuf::from(resources);
    let output = PathBuf::from(output);
    std::fs::create_dir_all(&output)?;
    let device = create_headless_device()?;
    let noise = Arc::new(Texture::new(
        device.clone(),
        &TextureDesc::new_2d(1, 1, vk::Format::R8G8B8A8_UNORM)
            .usage(vk::ImageUsageFlags::SAMPLED | vk::ImageUsageFlags::TRANSFER_DST),
    )?);
    init_texture_data(&device, &noise, &[128; 4])?;
    let shader =
        std::fs::read(resources.join("compiled-shaders/_text.raster.hlsl_b903e43ffb3da915.bin"))?;
    let mut gpu = GpuTextRenderer::new_with_config(
        device,
        &shader,
        noise,
        GpuTextConfig {
            pipeline_cache_path: Some(output.join("text.pipeline-cache")),
            ..Default::default()
        },
    )?;
    let mut text = TextRenderer::new();
    text.load_font_from_file(resources.join("assets/fonts/ubuntu/Ubuntu-Regular.ttf"))?;
    let fonts = Arc::new(FontTable::from_ron(
        "(fonts:{Label:(family:\"Ubuntu\",scale:1.0)})",
    )?);
    let mut fragment = TextFragment::new(FontMetrics::new(48., 60.), FontFamily::Label, fonts)?;
    fragment.set_text(&mut text, "Tiny Glade\nReusable text rendering")?;
    let rounded = ShapeStream::new()
        .fill_color(u32::MAX)
        .corner_radius(20.)
        .encode_no_images()?;
    let mut first_pixels = None;
    for frame in 0..2 {
        // Growth invalidates every fragment emitted in the current frame. Rebuild
        // the whole frame; the configured atlas bounds make retries finite.
        let mut retries = 0;
        let batch = loop {
            text.begin_frame();
            text.rect_with_color([20., 20.], [700., 200.], 0x6080_a0ff, &rounded)?;
            match fragment.render(&mut text, [370., 120.], [650., 180.], u32::MAX) {
                Err(TextError::AtlasChanged) if retries < 16 => retries += 1,
                result => {
                    result?;
                    break text.consume()?;
                }
            }
        };
        // The adapter owns its atlas. External sampled images must remain
        // immutable until completion, including when a timed wait returns early.
        let rendered =
            unsafe { gpu.render_batch_offscreen(1000, 1000, &batch, text.atlas(), &[]) }?;
        if let Some(pixels) = &first_pixels {
            assert_eq!(&rendered.rgba, pixels);
            assert_eq!(rendered.stats.upload_regions, 0);
        } else {
            std::fs::write(output.join("text.rgba"), &rendered.rgba)?;
            first_pixels = Some(rendered.rgba.clone());
        }
        println!(
            "frame {frame}: {} primitives, {} uploads, {} transfer buffers",
            batch.primitives.len(),
            rendered.stats.upload_regions,
            rendered.stats.transfer_buffer_creations
        );
    }
    gpu.save_pipeline_cache()?;
    std::fs::write(
        output.join("text.json"),
        "{\"width\":1000,\"height\":1000,\"format\":\"RGBA8\"}\n",
    )?;
    Ok(())
}
