//! Offscreen pixel baselines, shared resource reuse, and editor-to-shader colors.
//! Set TG_TEXT_ORIGINAL_ROOT and TG_TEXT_REQUIRE_GPU=1 to require assets and a GPU;
//! otherwise unavailable resources produce an explicit skip.
#![cfg(feature = "gpu")]
use ash::vk;
use rhapsody::{
    bindless::{BindlessConfig, BindlessHeap},
    raw::{
        device::Device,
        setup::init_texture_data,
        texture::{Texture, TextureDesc},
    },
};
use std::{
    path::{Path, PathBuf},
    sync::Arc,
};
use tg_text::{
    AtlasDescriptor, ImageScaling, PendingGlyphUpload, Primitive, RenderBatch, ShapeStream,
    gpu::{GpuTextConfig, GpuTextRenderer},
};
#[test]
fn production_text_bindless_matches_original_pixels_and_reuses_bounded_resources() {
    let required = std::env::var("TG_TEXT_REQUIRE_GPU").is_ok_and(|value| value == "1");
    let Some(root) = std::env::var_os("TG_TEXT_ORIGINAL_ROOT").map(PathBuf::from) else {
        assert!(!required, "TG_TEXT_ORIGINAL_ROOT is required");
        eprintln!("SKIP text GPU: original shader root is unavailable");
        return;
    };
    let device = match Device::new_headless(&rhapsody::device::DeviceConfig::headless()) {
        Ok(device) => device,
        Err(error) => {
            assert!(!required, "required text GPU initialization: {error}");
            eprintln!("SKIP text GPU: {error}");
            return;
        }
    };
    let shader =
        std::fs::read(root.join("compiled-shaders/_text.raster.hlsl_b903e43ffb3da915.bin"))
            .expect("verified original text shader");
    let blue = Arc::new(
        Texture::new(
            device.clone(),
            &TextureDesc::new_2d(1, 1, vk::Format::R8G8B8A8_UNORM)
                .usage(vk::ImageUsageFlags::SAMPLED | vk::ImageUsageFlags::TRANSFER_DST),
        )
        .unwrap(),
    );
    init_texture_data(&device, &blue, &[128; 4]).unwrap();
    let atlas = AtlasDescriptor {
        width: 2,
        height: 2,
        generation: 1,
    };
    let upload = PendingGlyphUpload {
        x: 0,
        y: 0,
        width: 2,
        height: 2,
        generation: 1,
        rgba: vec![
            255, 255, 255, 0, 255, 255, 255, 255, 255, 255, 255, 0, 255, 255, 255, 255,
        ],
    };
    // The original shader maps a logical height of 1000 to the target height.
    // Express the fixture in those units to cover the intended pixel rectangles.
    let logical = 1000. / 32.;
    let solid = Primitive::rect(
        [2. * logical, 2. * logical],
        [8. * logical, 8. * logical],
        u32::MAX,
    );
    let mut glyph = Primitive::rect(
        [16. * logical, 4. * logical],
        [8. * logical, 8. * logical],
        u32::MAX,
    );
    glyph.stream = 0;
    glyph.uv_extent = [2., 2.];
    let batch = RenderBatch {
        primitives: vec![solid, glyph],
        shape_words: Vec::new(),
        glyph_uploads: Vec::new(),
    };
    let reference_path = std::env::var_os("TG_TEXT_REFERENCE_PATH").map(PathBuf::from);
    let heap = BindlessHeap::new(device.clone(), BindlessConfig::default()).unwrap();
    let mut prior = reference_path
        .as_ref()
        .filter(|path| path.exists())
        .map(|path| std::fs::read(path).expect("read existing text pixel reference"));
    for index in 0..2 {
        let path = std::env::temp_dir().join(format!(
            "tg-text-pipeline-{}-{index}.cache",
            std::process::id()
        ));
        let mut renderer = match GpuTextRenderer::new_with_heap(
            device.clone(),
            &shader,
            blue.clone(),
            heap.clone(),
            GpuTextConfig {
                pipeline_cache_path: Some(path.clone()),
                ..Default::default()
            },
        ) {
            Ok(renderer) => renderer,
            Err(error) => {
                assert!(!required, "required text GPU capabilities: {error}");
                eprintln!("SKIP text GPU: {error}");
                return;
            }
        };
        // Invalid CPU batches fail before touching frame resources; the valid
        // submission immediately afterwards still starts at reset count one.
        let mut invalid = batch.clone();
        invalid.primitives[0].pos_min[0] = f32::NAN;
        assert!(unsafe { renderer.render_batch_offscreen(32, 32, &invalid, atlas, &[]) }.is_err());
        let mut invalid_upload = upload.clone();
        invalid_upload.x = atlas.width;
        assert!(
            unsafe { renderer.render_offscreen(32, 32, &batch, atlas, &[invalid_upload], &[]) }
                .is_err()
        );
        let mut heap_slots = None;
        for frame in 0..3 {
            let uploads = if frame == 0 {
                std::slice::from_ref(&upload)
            } else {
                &[]
            };
            let result =
                unsafe { renderer.render_offscreen(32, 32, &batch, atlas, uploads, &[]) }.unwrap();
            let pixel = |x: usize, y: usize| &result.rgba[(y * 32 + x) * 4..(y * 32 + x) * 4 + 4];
            if pixel(5, 5) != [255; 4] {
                let nonzero = result
                    .rgba
                    .as_chunks::<4>()
                    .0
                    .iter()
                    .enumerate()
                    .filter(|(_, pixel)| pixel[3] != 0)
                    .map(|(index, _)| (index % 32, index / 32))
                    .collect::<Vec<_>>();
                eprintln!(
                    "text nonzero count={} first={:?}",
                    nonzero.len(),
                    &nonzero[..nonzero.len().min(32)]
                );
            }
            assert_eq!(pixel(5, 5), [255; 4]);
            for y in 3..9 {
                for x in 3..9 {
                    assert_eq!(pixel(x, y), [255; 4]);
                }
            }
            assert_eq!(pixel(16, 8)[3], 0);
            assert!(pixel(23, 8)[3] > 0);
            assert_eq!(pixel(31, 31), [0; 4]);
            assert!(result.stats.dynamic_rendering);
            assert_eq!(result.stats.compiled_pipeline_requests, 1);
            assert_eq!(result.stats.command_pool_resets, frame + 1);
            assert!(result.stats.bindless_live_slots > 0);
            assert_eq!(
                *heap_slots.get_or_insert(result.stats.bindless_live_slots),
                result.stats.bindless_live_slots
            );
            assert_eq!(result.stats.dynamic_constant_page_creations, 1);
            assert_eq!(
                result.stats.transfer_buffer_creations, 2,
                "staging and readback are reused on warm frames"
            );
            if let Some(expected) = &prior {
                assert_eq!(&result.rgba, expected);
            } else {
                prior = Some(result.rgba);
            }
        }
        renderer.save_pipeline_cache().unwrap();
        assert!(std::fs::metadata(&path).unwrap().len() > 8);
        drop(renderer);
        let _ = std::fs::remove_file(path);
    }
    if let Some(path) = reference_path
        && !path.exists()
    {
        std::fs::write(path, prior.expect("verified text pixels"))
            .expect("save text pixel reference");
    }
}

#[test]
fn production_text_runtime_image_array_matches_original_pixels() {
    let required = std::env::var("TG_TEXT_REQUIRE_GPU").is_ok_and(|value| value == "1");
    let Some(root) = std::env::var_os("TG_TEXT_ORIGINAL_ROOT").map(PathBuf::from) else {
        assert!(!required, "TG_TEXT_ORIGINAL_ROOT is required");
        eprintln!("SKIP text GPU image: original shader root is unavailable");
        return;
    };
    let device = match Device::new_headless(&rhapsody::device::DeviceConfig::headless()) {
        Ok(device) => device,
        Err(error) => {
            assert!(!required, "required text GPU image initialization: {error}");
            eprintln!("SKIP text GPU image: {error}");
            return;
        }
    };
    let texture = |width, height, bytes: &[u8]| {
        let texture = Arc::new(
            Texture::new(
                device.clone(),
                &TextureDesc::new_2d(width, height, vk::Format::R8G8B8A8_UNORM)
                    .usage(vk::ImageUsageFlags::SAMPLED | vk::ImageUsageFlags::TRANSFER_DST),
            )
            .unwrap(),
        );
        init_texture_data(&device, &texture, bytes).unwrap();
        texture
    };
    let blue = texture(1, 1, &[128; 4]);
    let images = [
        texture(1, 1, &[0, 0, 0, 255]),
        texture(
            2,
            2,
            &[
                255, 0, 0, 255, 0, 255, 0, 255, 0, 0, 255, 255, 255, 255, 255, 255,
            ],
        ),
    ];
    let shader =
        std::fs::read(root.join("compiled-shaders/_text.raster.hlsl_b903e43ffb3da915.bin"))
            .unwrap();
    let mut renderer = GpuTextRenderer::new(device, &shader, blue).unwrap();
    let logical = 1000. / 32.;
    let mut primitive = Primitive::rect(
        [4. * logical, 4. * logical],
        [16. * logical, 16. * logical],
        u32::MAX,
    );
    let shape = ShapeStream::new()
        .image(1u32, ImageScaling::Stretch)
        .encode_no_images()
        .unwrap();
    primitive.stream = shape.words().len() as u32;
    let batch = RenderBatch {
        primitives: vec![primitive],
        shape_words: shape.0,
        glyph_uploads: vec![],
    };
    let path = std::env::var_os("TG_TEXT_REFERENCE_PATH")
        .map(PathBuf::from)
        .map(|path| path.with_extension("image.rgba"));
    let mut reference = path
        .as_ref()
        .filter(|path| path.exists())
        .map(|path| std::fs::read(path).unwrap());
    for _ in 0..3 {
        let result = unsafe {
            renderer.render_offscreen(
                32,
                32,
                &batch,
                AtlasDescriptor {
                    width: 1,
                    height: 1,
                    generation: 1,
                },
                &[],
                &images,
            )
        }
        .unwrap();
        let center = &result.rgba[(12 * 32 + 12) * 4..(12 * 32 + 12) * 4 + 4];
        assert!(
            center[..3].iter().any(|value| *value > 0),
            "index one must sample the colored image rather than the black decoy"
        );
        assert!(center[3] > 0);
        assert_eq!(&result.rgba[..4], &[0; 4]);
        if let Some(reference) = &reference {
            assert_eq!(&result.rgba, reference);
        } else {
            reference = Some(result.rgba);
        }
    }
    assert_editor_color_boundary(&mut renderer, &root);
    if let Some(path) = path
        && !path.exists()
    {
        std::fs::write(path, reference.unwrap()).unwrap();
    }
}

// Exercise the typed editor boundary as well as synthetic shader input. Neutral
// white fixtures cannot detect an ARGB/RGBA channel-order mismatch.
fn assert_editor_color_boundary(gpu: &mut GpuTextRenderer, resources: &Path) {
    use cosmic_text::{Attrs, AttrsList, Family};
    use tg_text::{
        Color, Cursor, Edit, FontFamily, FontMetrics, FontTable, Selection, TextFragment,
        TextRenderer,
    };

    let mut renderer = TextRenderer::with_atlas_size(128, 128).unwrap();
    renderer
        .load_font_from_file(resources.join("assets/fonts/ubuntu/Ubuntu-Regular.ttf"))
        .unwrap();
    let table =
        Arc::new(FontTable::from_ron(r#"(fonts:{Label:(family:"Ubuntu",scale:1.0)})"#).unwrap());
    let mut fragment =
        TextFragment::new(FontMetrics::new(48., 60.), FontFamily::Label, table).unwrap();
    fragment.set_text(&mut renderer, "M").unwrap();
    fragment.editor_mut().with_buffer_mut(|buffer| {
        let attrs = AttrsList::new(
            Attrs::new()
                .family(Family::Name("Ubuntu"))
                .color(Color::rgba(32, 192, 96, 128)),
        );
        for line in &mut buffer.lines {
            line.set_attrs_list(attrs.clone());
        }
    });
    for selected in [false, true] {
        if selected {
            let editor = fragment.editor_mut();
            editor.set_cursor(Cursor::new(0, 1));
            editor.set_selection(Selection::Normal(Cursor::new(0, 0)));
        }
        renderer.begin_frame();
        fragment
            .render(&mut renderer, [250., 100.], [400., 120.], u32::MAX)
            .unwrap();
        let batch = renderer.consume().unwrap();
        assert!(
            batch
                .primitives
                .iter()
                .filter(|primitive| primitive.stream == 0)
                .all(|primitive| primitive.color == 0x20c0_6080)
        );
        if selected {
            assert!(batch.primitives.iter().any(|primitive| primitive.stream == u32::MAX && primitive.color == 0xffff_ff80));
        }
        let output =
            unsafe { gpu.render_batch_offscreen(1000, 1000, &batch, renderer.atlas(), &[]) }
                .unwrap();
        let pixels = output.rgba.as_chunks::<4>().0;
        if selected {
            assert!(
                pixels.iter().any(|pixel| pixel[0] == pixel[1]
                    && pixel[1] == pixel[2]
                    && (127..=129).contains(&pixel[3])),
                "selection background must retain half opacity and neutral color"
            );
        } else {
            let strongest = pixels.iter().max_by_key(|pixel| pixel[3]).unwrap();
            assert!(
                (20..=129).contains(&strongest[3]),
                "editor alpha must reach the shader alpha channel: {strongest:?}"
            );
            assert!(
                strongest[1] > strongest[0] && strongest[1] > strongest[2],
                "green editor text must remain green: {strongest:?}"
            );
        }
    }
}
