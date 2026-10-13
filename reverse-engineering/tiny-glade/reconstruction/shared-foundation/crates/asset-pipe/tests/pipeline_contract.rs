use asset::texture::{
    blob::{TEXTURE_MAGIC, TEXTURE_VERSION, TextureAssetBlob},
    desc::*,
};
use asset_pipe::texture::*;
use image::{DynamicImage, Rgba, RgbaImage};
use relative_path::{RelativePath, RelativePathBuf};
use std::path::{Path, PathBuf};

struct Fixture(PathBuf);
impl Fixture {
    fn new() -> Self {
        let root = std::env::temp_dir().join(format!(
            "tiny-glade-asset-policy-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        std::fs::create_dir(&root).unwrap();
        Self(root)
    }
    fn write(&self, path: &str, bytes: impl AsRef<[u8]>) {
        let path = self.0.join(path);
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        std::fs::write(path, bytes).unwrap();
    }
}
impl Drop for Fixture {
    fn drop(&mut self) {
        std::fs::remove_dir_all(&self.0).unwrap();
    }
}
fn opts(gamma: Gamma, mips: MipChain, compression: CompressionMethod) -> TextureOpts {
    TextureOpts {
        gamma,
        mips,
        compression,
        resize: None,
    }
}
fn image(w: u32, h: u32) -> DynamicImage {
    DynamicImage::ImageRgba8(RgbaImage::from_fn(w, h, |x, y| {
        Rgba([(x * 13) as u8, (y * 17) as u8, 63, 255])
    }))
}

#[test]
fn block_decision_uses_source_dimensions_and_mips_have_edge_padding() {
    let opaque = bc7_settings(true);
    let alpha = bc7_settings(false);
    assert_eq!(
        (
            opaque.channels,
            opaque.fast_skip_threshold_mode1,
            opaque.fast_skip_threshold_mode3,
            opaque.fast_skip_threshold_mode7
        ),
        (3, 12, 8, 0)
    );
    assert_eq!(
        (
            alpha.channels,
            alpha.fast_skip_threshold_mode1,
            alpha.fast_skip_threshold_mode3,
            alpha.fast_skip_threshold_mode7
        ),
        (4, 12, 8, 4)
    );
    assert_eq!(opaque.refine_iterations, [2, 2, 2, 2, 2, 2, 2, 0]);
    assert_eq!(alpha.refine_iterations, [2; 8]);
    assert!(opaque.skip_mode2 && alpha.skip_mode2);
    let full = process_decoded_image(
        image(5, 6),
        &opts(Gamma::Linear, MipChain::Full, CompressionMethod::Color),
    )
    .unwrap();
    assert_eq!(full.dimensions, [8, 8, 1]);
    assert_eq!(full.mips, 4);
    assert_eq!(full.format, 145);
    // 8x8 base has four BC7 blocks; 4x4,2x2 and1x1 mips each occupy one block.
    assert_eq!(full.data.len(), 7 * 16);
    let tiny = process_decoded_image(
        image(3, 6),
        &TextureOpts {
            resize: Some([9, 10]),
            ..opts(Gamma::Linear, MipChain::None, CompressionMethod::Color)
        },
    )
    .unwrap();
    assert_eq!(tiny.dimensions, [9, 10, 1]);
    assert_eq!(tiny.format, 37);
    assert_eq!(tiny.data.len(), 9 * 10 * 4);
    let gray = process_decoded_image(
        image(4, 4),
        &opts(Gamma::Linear, MipChain::Full, CompressionMethod::Grayscale),
    )
    .unwrap();
    assert_eq!(gray.format, 139);
    assert_eq!(gray.data.len(), 3 * 8);
}
#[test]
fn transfer_preserves_alpha_and_gamma_selects_wire_format() {
    let rgba = RgbaImage::from_pixel(2, 2, Rgba([13, 127, 240, 73]));
    let srgb = process_decoded_image(
        DynamicImage::ImageRgba8(rgba.clone()),
        &opts(Gamma::Srgb, MipChain::None, CompressionMethod::None),
    )
    .unwrap();
    assert_eq!(srgb.format, 43);
    assert_eq!(srgb.data, rgba.into_raw());
    assert_eq!(mip_count([8, 2], MipChain::Limited(0)), 0);
    assert_eq!(mip_count([8, 2], MipChain::Limited(12)), 4);
    let padding = pad_edges(
        &RgbaImage::from_fn(2, 2, |x, y| Rgba([x as u8, y as u8, 0, 255])),
        [4, 4],
    );
    assert_eq!(*padding.get_pixel(3, 3), Rgba([1, 1, 0, 255]));
}
#[test]
fn srgb_grayscale_is_rejected_at_the_original_boundary() {
    assert!(
        std::panic::catch_unwind(|| process_decoded_image(
            image(4, 4),
            &opts(Gamma::Srgb, MipChain::None, CompressionMethod::Grayscale)
        ))
        .is_err()
    );
    assert_eq!(
        process_decoded_image(
            image(3, 4),
            &opts(Gamma::Srgb, MipChain::None, CompressionMethod::Grayscale)
        )
        .unwrap()
        .format,
        43
    );
}
fn blob(value: u8) -> TextureAssetBlob {
    TextureAssetBlob {
        data: vec![value; 16],
        dimensions: [4, 4, 1],
        mips: 1,
        layers: 1,
        format: 145,
        kind: 2,
    }
}
#[test]
fn arrays_preserve_layer_order_and_reject_mixed_headers() {
    let merged = assemble_array(vec![blob(7), blob(9)]).unwrap();
    assert_eq!((merged.layers, merged.kind), (2, 3));
    assert_eq!(merged.data, [vec![7; 16], vec![9; 16]].concat());
    let mut wrong = blob(0);
    wrong.dimensions[0] = 8;
    assert!(assemble_array(vec![blob(0), wrong]).is_err());
    let mut wrong = blob(0);
    wrong.mips = 2;
    assert!(assemble_array(vec![blob(0), wrong]).is_err());
    let mut wrong = blob(0);
    wrong.format = 146;
    assert!(assemble_array(vec![blob(0), wrong]).is_err());
}
#[test]
fn dds_keeps_the_author_layer_zero_rule_and_original_format_whitelist() {
    use ddsfile::*;
    for (dxgi, vk) in [
        (DxgiFormat::R9G9B9E5_SharedExp, 123),
        (DxgiFormat::BC1_UNorm_sRGB, 134),
        (DxgiFormat::BC3_UNorm, 137),
        (DxgiFormat::BC3_UNorm_sRGB, 138),
        (DxgiFormat::BC5_UNorm, 141),
        (DxgiFormat::BC5_SNorm, 142),
    ] {
        let mut dds = Dds::new_dxgi(NewDxgiParams {
            height: 4,
            width: 4,
            depth: None,
            format: dxgi,
            mipmap_levels: Some(2),
            array_layers: Some(2),
            caps2: None,
            is_cubemap: false,
            resource_dimension: D3D10ResourceDimension::Texture2D,
            alpha_mode: AlphaMode::Unknown,
        })
        .unwrap();
        let n = dds.get_data(0).unwrap().len();
        dds.data.fill(63);
        let out = process_dds(dds).unwrap();
        assert_eq!((out.format, out.kind, out.mips, out.layers), (vk, 2, 2, 2));
        assert_eq!(
            out.data.len(),
            n,
            "only layer zero copied, not all array layers"
        );
    }
    for (format, vk) in [(D3DFormat::DXT1, 134), (D3DFormat::DXT5, 138)] {
        let dds = Dds::new_d3d(NewD3dParams {
            height: 4,
            width: 4,
            depth: None,
            format,
            mipmap_levels: None,
            caps2: None,
        })
        .unwrap();
        assert_eq!(process_dds(dds).unwrap().format, vk);
    }
    let unsupported = Dds::new_dxgi(NewDxgiParams {
        height: 4,
        width: 4,
        depth: None,
        format: DxgiFormat::BC7_UNorm,
        mipmap_levels: None,
        array_layers: None,
        caps2: None,
        is_cubemap: false,
        resource_dimension: D3D10ResourceDimension::Texture2D,
        alpha_mode: AlphaMode::Unknown,
    })
    .unwrap();
    assert!(process_dds(unsupported).is_err());
}
#[test]
fn compilation_scan_inheritance_cache_and_rebuild_close() {
    let fixture = Fixture::new();
    fixture.write(
        "assets-src/textures/assets.ron",
        r#"(default:(gamma:Linear,resize:None,mips:None,compression:None))"#,
    );
    let source = fixture.0.join("assets-src/textures/a.png");
    image(4, 4).save(&source).unwrap();
    fixture.write("compiled-assets/textures/list.ron", "[]");
    let (names, meta) = compile_texture_assets(&fixture.0).unwrap();
    assert_eq!(names, vec![RelativePathBuf::from("a")]);
    let output = fixture.0.join("compiled-assets/textures/a.texture");
    let original = std::fs::read(&output).unwrap();
    assert_eq!(TextureAssetBlob::load(&original).unwrap().format, 37);
    let mut header = TEXTURE_MAGIC.to_le_bytes().to_vec();
    header.extend(TEXTURE_VERSION.to_le_bytes());
    std::fs::write(&output, &header).unwrap(); // Valid cache header intentionally permits a missing payload.
    let desc = meta.assets[RelativePath::new("a")].desc.clone();
    compile_texture_asset(
        RelativePath::new("a"),
        desc,
        &fixture.0.join("compiled-assets/textures"),
        meta.assets.get(RelativePath::new("a")),
    )
    .unwrap();
    assert_eq!(std::fs::read(&output).unwrap(), header);
    header[4] = 7;
    std::fs::write(&output, &header).unwrap();
    let (_, next) = compile_texture_assets(&fixture.0).unwrap();
    assert_eq!(std::fs::read(&output).unwrap(), original);
    assert_eq!(next.assets.len(), 1);
    assert_eq!(
        compile_or_load_texture_assets(&fixture.0, false).unwrap(),
        names
    );
}
#[test]
fn scanner_arrays_require_consecutive_indices_and_name_normalization() {
    let fixture = Fixture::new();
    fixture.write("assets.ron", "(default:(compression:None))");
    fixture.write("array_color/assets.ron", "(default:(gamma:Linear))");
    image(2, 2)
        .save(fixture.0.join("array_color/0.png"))
        .unwrap();
    image(2, 2)
        .save(fixture.0.join("array_color/1.png"))
        .unwrap();
    fixture.write("notes.txt", "ignored");
    let found = asset::texture::scan::scan_asset_tree(&fixture.0).unwrap();
    let array = &found[RelativePath::new("color")];
    assert_eq!(array.layers.len(), 2);
    assert_eq!(array.layers[0].opts.compression, CompressionMethod::None);
    std::fs::rename(
        fixture.0.join("array_color/1.png"),
        fixture.0.join("array_color/3.png"),
    )
    .unwrap();
    assert!(asset::texture::scan::scan_asset_tree(&fixture.0).is_err());
}
#[test]
fn shipped_read_only_index_is_available_without_source_images() {
    let root = Path::new("D:/game/ljxsj_92385/Tiny Glade");
    let names = compile_or_load_texture_assets(root, false).unwrap();
    assert_eq!(names.len(), 388);
    assert!(names.windows(2).all(|n| n[0] < n[1]));
}

#[test]
fn failed_job_joins_other_tasks_and_leaves_metadata_and_list_unchanged() {
    let fixture = Fixture::new();
    fixture.write(
        "assets-src/textures/assets.ron",
        "(default:(compression:None))",
    );
    image(4, 4)
        .save(fixture.0.join("assets-src/textures/good.png"))
        .unwrap();
    fixture.write("assets-src/textures/bad.png", "not an image");
    fixture.write(
        "tmp/textures.meta.ron",
        "invalid previous metadata must remain on failure",
    );
    fixture.write("compiled-assets/textures/list.ron", "[\"old\"]");
    assert!(compile_texture_assets(&fixture.0).is_err());
    let good = std::fs::read(fixture.0.join("compiled-assets/textures/good.texture")).unwrap();
    assert_eq!(TextureAssetBlob::load(&good).unwrap().dimensions, [4, 4, 1]);
    assert_eq!(
        std::fs::read_to_string(fixture.0.join("tmp/textures.meta.ron")).unwrap(),
        "invalid previous metadata must remain on failure"
    );
    assert_eq!(
        std::fs::read_to_string(fixture.0.join("compiled-assets/textures/list.ron")).unwrap(),
        "[\"old\"]"
    );
}

#[test]
fn panicking_job_joins_other_tasks_before_unwinding_and_does_not_persist_indices() {
    let fixture = Fixture::new();
    fixture.write(
        "assets-src/textures/assets.ron",
        r#"(default:(compression:None),assets:{"panic":(gamma:sRGB,compression:Grayscale)})"#,
    );
    image(4, 4)
        .save(fixture.0.join("assets-src/textures/good.png"))
        .unwrap();
    image(4, 4)
        .save(fixture.0.join("assets-src/textures/panic.png"))
        .unwrap();
    fixture.write("compiled-assets/textures/list.ron", "[\"old\"]");
    assert!(std::panic::catch_unwind(|| compile_texture_assets(&fixture.0)).is_err());
    let good = std::fs::read(fixture.0.join("compiled-assets/textures/good.texture")).unwrap();
    assert_eq!(TextureAssetBlob::load(&good).unwrap().dimensions, [4, 4, 1]);
    assert!(!fixture.0.join("tmp/textures.meta.ron").exists());
    assert_eq!(
        std::fs::read_to_string(fixture.0.join("compiled-assets/textures/list.ron")).unwrap(),
        "[\"old\"]"
    );
}
