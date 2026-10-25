//! 真实发行字体与原 SPIR-V 的独立离屏验收；不启动游戏消费者。
use ash::vk;
use rhapsody::raw::{
    device::VulkanDevice,
    init::create_headless_device,
    setup::init_texture_data,
    texture::{Texture, TextureDesc},
};
use std::{
    path::{Path, PathBuf},
    sync::Arc,
    time::Instant,
};
use tg_text::{
    ImageScaling, ImageSource, RawShapeStream, ShapeStream, TextFragment, TextRenderer,
    gpu::{GpuTextRenderer, RenderedText},
};
fn texture(
    device: &Arc<VulkanDevice>,
    w: u32,
    h: u32,
    data: &[u8],
) -> Result<Arc<Texture>, Box<dyn std::error::Error>> {
    let desc = TextureDesc::new_2d(w, h, vk::Format::R8G8B8A8_UNORM)
        .usage(vk::ImageUsageFlags::SAMPLED | vk::ImageUsageFlags::TRANSFER_DST);
    let texture = Arc::new(Texture::new(device.clone(), &desc)?);
    init_texture_data(device, &texture, data)?;
    Ok(texture)
}
fn save(root: &Path, name: &str, frame: &RenderedText) -> std::io::Result<()> {
    std::fs::write(root.join(format!("{name}.rgba")), &frame.rgba)
}
fn colored(frame: &RenderedText) -> usize {
    frame.rgba.chunks_exact(4).filter(|p| p[3] != 0).count()
}
fn pixel(frame: &RenderedText, x: usize, y: usize) -> &[u8] {
    let i = (y * frame.width as usize + x) * 4;
    &frame.rgba[i..i + 4]
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    if args.len() != 2 {
        return Err("usage: text_gpu <original-game-root> <output-directory>".into());
    }
    let game = PathBuf::from(&args[0]);
    let output = PathBuf::from(&args[1]);
    std::fs::create_dir_all(&output)?;
    eprintln!("PHASE device");
    let device = create_headless_device()?;
    let properties = unsafe {
        device
            .instance
            .instance
            .get_physical_device_properties(device.physical)
    };
    let device_name = unsafe { std::ffi::CStr::from_ptr(properties.device_name.as_ptr()) }
        .to_string_lossy()
        .to_string();
    let shader_path = game.join("compiled-shaders/_text.raster.hlsl_b903e43ffb3da915.bin");
    let shader = std::fs::read(&shader_path)?;
    let blue = texture(&device, 256, 256, &vec![128; 256 * 256 * 4])?;
    let sprite = texture(
        &device,
        2,
        2,
        &[
            255, 0, 0, 255, 0, 255, 0, 255, 0, 0, 255, 255, 255, 255, 255, 255,
        ],
    )?;
    eprintln!("PHASE shader construct");
    let mut gpu = GpuTextRenderer::new(device.clone(), &shader, blue.clone())?;
    let mut renderer = TextRenderer::new();
    let fonts = [
        "ubuntu/Ubuntu-Regular.ttf",
        "noto sans/NotoSans-Regular.ttf",
        "noto sans sc/NotoSansSC-Regular.ttf",
        "noto sans kr/NotoSansKR-Regular.ttf",
        "noto emoji/NotoColorEmoji.ttf",
    ];
    for path in fonts {
        renderer.load_font_from_file(game.join("assets/fonts").join(path))?;
    }
    let rtl_font = PathBuf::from("C:/Windows/Fonts/arial.ttf");
    renderer.load_font_from_file(&rtl_font)?;
    let mut rows = Vec::new();
    eprintln!("PHASE empty frame");
    let atlas = renderer.atlas();
    let empty = unsafe { gpu.render_offscreen(1000, 1000, &renderer.consume(), atlas, &[], &[]) }?;
    assert_eq!(colored(&empty), 0);
    rows.push(serde_json::json!({"case":"empty","colored_pixels":0,"submits":empty.stats.submits}));
    // 哨兵直接区分原 shader 的 glyph 与 solid 路径，不能仅检查有色像素总数。
    let probe_atlas = tg_text::AtlasDescriptor {
        width: 1024,
        height: 1024,
        generation: 900,
    };
    let mut mask = Vec::new();
    for _y in 0..16 {
        for x in 0..16 {
            mask.extend([255, 255, 255, if x < 8 { 0 } else { 255 }]);
        }
    }
    let probe_upload = tg_text::PendingGlyphUpload {
        x: 0,
        y: 0,
        width: 16,
        height: 16,
        rgba: mask,
        generation: 900,
    };
    let solid = tg_text::Primitive::rect([20., 20.], [100., 100.], u32::MAX);
    let mut glyph = tg_text::Primitive::rect([200., 200.], [160., 160.], u32::MAX);
    glyph.stream = 0;
    glyph.uv_min = [0., 0.];
    glyph.uv_extent = [16., 16.];
    let probe_batch = tg_text::RenderBatch {
        primitives: vec![solid, glyph],
        shape_words: Vec::new(),
    };
    let probe = unsafe {
        gpu.render_offscreen(1000, 1000, &probe_batch, probe_atlas, &[probe_upload], &[])
    }?;
    assert_eq!(pixel(&probe, 50, 50), [255, 255, 255, 255]);
    assert_eq!(pixel(&probe, 220, 280)[3], 0);
    assert!((1..255).contains(&pixel(&probe, 340, 280)[3]));
    rows.push(serde_json::json!({"case":"original_stream_sentinels","solid_alpha":pixel(&probe,50,50)[3],"glyph_zero_alpha":pixel(&probe,220,280)[3],"glyph_opaque_alpha":pixel(&probe,340,280)[3]}));

    eprintln!("PHASE text prepare");
    let mut fragment = TextFragment::new(&mut renderer, 48., 60.);
    fragment.set_family("Ubuntu");
    fragment.set_text(
        &mut renderer,
        "Tiny Glade 你好 한글 😀\nCombining: e\u{301} | RTL: אבג",
    );
    let mut cold_pixels = Vec::new();
    for frame in 0..2 {
        eprintln!("PHASE text frame {frame}");
        renderer.begin_frame();
        fragment.render(&mut renderer, [30., 30.], u32::MAX);
        let uploads = renderer.upload_pending_glyphs();
        let atlas = renderer.atlas();
        let batch = renderer.consume();
        let start = Instant::now();
        let rendered = unsafe { gpu.render_offscreen(1000, 1000, &batch, atlas, &uploads, &[]) }?;
        let elapsed: u64 = start.elapsed().as_micros().try_into().unwrap_or(u64::MAX);
        assert!(colored(&rendered) > 100);
        if frame == 0 {
            cold_pixels = rendered.rgba.clone();
            save(&output, "text", &rendered)?;
        } else {
            assert!(uploads.is_empty(), "warm glyphs must not upload again");
            assert_eq!(rendered.rgba, cold_pixels);
        }
        rows.push(serde_json::json!({"case":if frame==0{"text_cold"}else{"text_warm"},"glyph_primitives":batch.primitives.iter().filter(|p|p.stream==0).count(),"colored_pixels":colored(&rendered),"upload_regions":rendered.stats.upload_regions,"upload_bytes":rendered.stats.upload_bytes,"submits":rendered.stats.submits,"elapsed_us":elapsed}));
    }
    eprintln!("PHASE shapes");
    renderer.begin_frame();
    let mut raw = RawShapeStream::new();
    raw.append(
        &ShapeStream::new().fill_color(u32::MAX).corner_radius(20.),
        &mut |_| 0,
    );
    renderer.rect_with_color([20., 200.], [100., 100.], u32::MAX, &raw);
    let mut sprite_raw = RawShapeStream::new();
    sprite_raw.append(
        &ShapeStream::new().image(ImageSource::Index(0), ImageScaling::Stretch),
        &mut |_| 0,
    );
    renderer.rect_with_color([200., 200.], [100., 100.], u32::MAX, &sprite_raw);
    let batch = renderer.consume();
    let shape_frame = unsafe {
        gpu.render_offscreen(
            1000,
            1000,
            &batch,
            renderer.atlas(),
            &[],
            std::slice::from_ref(&sprite),
        )
    }?;
    assert_eq!(pixel(&shape_frame, 70, 250), [255, 255, 255, 255]);
    assert_eq!(pixel(&shape_frame, 20, 200)[3], 0);
    assert!(pixel(&shape_frame, 225, 225)[0] > 100);
    save(&output, "shapes", &shape_frame)?;
    rows.push(serde_json::json!({"case":"rounded_rect_and_bindless_image","colored_pixels":colored(&shape_frame),"submits":shape_frame.stats.submits}));
    // 四种原 image scaling 都执行真实 shader；均使用明确的 resolver 纹理。
    for scaling in [
        ImageScaling::Stretch,
        ImageScaling::FitHeight,
        ImageScaling::BlurExtend,
        ImageScaling::Nine,
    ] {
        renderer.begin_frame();
        let mut ops = RawShapeStream::new();
        ops.append(
            &ShapeStream::new().image(ImageSource::Index(0), scaling),
            &mut |_| 0,
        );
        renderer.rect_with_color([20., 400.], [160., 100.], u32::MAX, &ops);
        let batch = renderer.consume();
        let scaled_image = unsafe {
            gpu.render_offscreen(
                1000,
                1000,
                &batch,
                renderer.atlas(),
                &[],
                std::slice::from_ref(&sprite),
            )
        }?;
        assert!(colored(&scaled_image) > 100);
        rows.push(serde_json::json!({"case":"image_scaling","mode":format!("{scaling:?}"),"colored_pixels":colored(&scaled_image)}));
    }
    // 新 GPU backend 不能悄悄接受仍在 CPU cache 中、却从未上传的 glyph。
    renderer.begin_frame();
    fragment.render(&mut renderer, [30., 30.], u32::MAX);
    let cached = renderer.consume();
    let mut recreated = GpuTextRenderer::new(device.clone(), &shader, blue.clone())?;
    assert!(
        unsafe { recreated.render_offscreen(1000, 1000, &cached, renderer.atlas(), &[], &[]) }
            .is_err()
    );
    renderer.reset_glyph_cache();
    renderer.begin_frame();
    fragment.render(&mut renderer, [30., 30.], u32::MAX);
    let recovered_uploads = renderer.upload_pending_glyphs();
    let recovered = renderer.consume();
    let recovered_frame = unsafe {
        recreated.render_offscreen(
            1000,
            1000,
            &recovered,
            renderer.atlas(),
            &recovered_uploads,
            &[],
        )
    }?;
    assert!(colored(&recovered_frame) > 100);
    rows.push(serde_json::json!({"case":"gpu_recreation_requires_reupload","upload_regions":recovered_frame.stats.upload_regions,"passed":true}));

    renderer.set_pixels_per_point(2.);
    renderer.begin_frame();
    fragment.render(&mut renderer, [30., 30.], u32::MAX);
    let uploads = renderer.upload_pending_glyphs();
    assert!(!uploads.is_empty());
    let atlas = renderer.atlas();
    let batch = renderer.consume();
    let scaled = unsafe { gpu.render_offscreen(1000, 1000, &batch, atlas, &uploads, &[]) }?;
    assert!(colored(&scaled) > 100);
    save(&output, "scale", &scaled)?;
    rows.push(serde_json::json!({"case":"scale_invalidates_atlas","generation":atlas.generation,"upload_regions":scaled.stats.upload_regions,"colored_pixels":colored(&scaled)}));
    let mut stale = uploads[0].clone();
    stale.generation = atlas.generation.wrapping_sub(1);
    assert!(unsafe { gpu.render_offscreen(1000, 1000, &batch, atlas, &[stale], &[]) }.is_err());
    let empty_after_error =
        unsafe { gpu.render_offscreen(1000, 1000, &Default::default(), atlas, &[], &[]) }?;
    assert_eq!(colored(&empty_after_error), 0);
    rows.push(serde_json::json!({"case":"stale_upload_rejected_then_empty_frame","passed":true}));
    let report = serde_json::json!({"status":"passed_actual_original_shader_gpu","device":device_name,"vendor_id":properties.vendor_id,"device_id":properties.device_id,"width":1000,"height":1000,"shader":shader_path,"fonts":fonts,"supplemental_rtl_font":rtl_font,"supplemental_font_scope":"independent RTL validation only; not original font initialization","blue_noise":"constant test fixture","cases":rows,"scope":"candidate library pipeline using original color SPIR-V; not original game consumer or complete original CPU runtime comparison"});
    std::fs::write(
        output.join("gpu-validation.json"),
        serde_json::to_vec_pretty(&report)?,
    )?;
    println!("{report}");
    Ok(())
}
