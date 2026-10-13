use anyhow::{Context, Result};
use asset::texture::{
    blob::{TEXTURE_MAGIC, TEXTURE_VERSION, TextureAssetBlob},
    desc::{CompressionMethod, Gamma, MipChain, TextureAssetDesc, TextureOpts},
};
use image::{DynamicImage, RgbaImage, imageops::FilterType};
use relative_path::RelativePathBuf;
use serde::{Deserialize, Serialize};
use std::{borrow::Cow, collections::HashMap, io::Read, path::Path, time::SystemTime};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TextureAssetMeta {
    pub desc: TextureAssetDesc,
    pub src_mtime: SystemTime,
}
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct TextureAssetLibraryMeta {
    pub assets: HashMap<RelativePathBuf, TextureAssetMeta>,
}

/// Original scalar thresholds/constants (B 0x1417bcd20), including the unordered NaN branch.
pub fn srgb_oetf(x: f32) -> f32 {
    if x <= 0.0031308 {
        x * 12.92
    } else {
        x.powf(0.41666666) * 1.055 + (-0.055)
    }
}
pub fn srgb_eotf(x: f32) -> f32 {
    if x > 0.04045 {
        ((x + 0.055) / 1.055).powf(2.4)
    } else {
        x / 12.92
    }
}

pub fn mip_count(dims: [u32; 2], chain: MipChain) -> u32 {
    assert!(dims[0] > 0 && dims[1] > 0, "dims[0] > 0 && dims[1] > 0");
    let full = 32 - dims[0].max(dims[1]).leading_zeros();
    match chain {
        MipChain::None => 1,
        MipChain::Full => full,
        MipChain::Limited(n) => n.min(full),
    }
}
pub fn pad_edges(image: &RgbaImage, dims: [u32; 2]) -> RgbaImage {
    assert!(dims[0] >= image.width() && dims[1] >= image.height());
    RgbaImage::from_fn(dims[0], dims[1], |x, y| {
        *image.get_pixel(x.min(image.width() - 1), y.min(image.height() - 1))
    })
}
/// Discriminants are private compression modes: BC5=0, BC4=1, BC7=2.
pub fn compress(image: &RgbaImage, mode: u8) -> Result<Vec<u8>> {
    anyhow::ensure!(
        image.width() & 3 == 0 && image.height() & 3 == 0,
        "Compression dimensions must be block aligned"
    );
    match mode {
        0 => {
            let data: Vec<u8> = image.pixels().flat_map(|p| [p[0], p[3]]).collect();
            let surface = intel_tex_2::RgSurface {
                data: &data,
                width: image.width(),
                height: image.height(),
                stride: image.width() * 2,
            };
            Ok(intel_tex_2::bc5::compress_blocks(&surface))
        }
        1 => {
            let data: Vec<u8> = image.pixels().map(|p| p[0]).collect();
            let surface = intel_tex_2::RSurface {
                data: &data,
                width: image.width(),
                height: image.height(),
                stride: image.width(),
            };
            Ok(intel_tex_2::bc4::compress_blocks(&surface))
        }
        2 => {
            let surface = intel_tex_2::RgbaSurface {
                data: image.as_raw(),
                width: image.width(),
                height: image.height(),
                stride: image.width() * 4,
            };
            let settings = bc7_settings(image.pixels().all(|p| p[3] == 255));
            Ok(intel_tex_2::bc7::compress_blocks(&settings, &surface))
        }
        _ => anyhow::bail!("Unsupported compression mode {mode}"),
    }
}
/// Original constants at B 0x1417bcff1..0x1417bd076 select the dependency's basic preset.
pub fn bc7_settings(opaque: bool) -> intel_tex_2::bc7::EncodeSettings {
    if opaque {
        intel_tex_2::bc7::opaque_basic_settings()
    } else {
        intel_tex_2::bc7::alpha_basic_settings()
    }
}

/// B `with_rgba8`: filtering is done in the selected working space, transfer is RGB-only.
fn with_rgba8(image: &DynamicImage, gamma: Gamma) -> Cow<'_, RgbaImage> {
    match gamma {
        // B's linear with_rgba8 specializations hand the existing RGBA8 buffer
        // directly to compression. Borrow it rather than cloning every texel.
        Gamma::Linear => image
            .as_rgba8()
            .map(Cow::Borrowed)
            .unwrap_or_else(|| Cow::Owned(image.to_rgba8())),
        Gamma::Srgb => {
            let mut float = image.to_rgba32f();
            for p in float.pixels_mut() {
                for channel in &mut p.0[..3] {
                    *channel = srgb_oetf(*channel);
                }
            }
            Cow::Owned(DynamicImage::ImageRgba32F(float).into_rgba8())
        }
    }
}

fn encode_level(image: &DynamicImage, gamma: Gamma, compressed: bool, mode: u8) -> Result<Vec<u8>> {
    let rgba = with_rgba8(image, gamma);
    if compressed {
        compress(&rgba, mode)
    } else {
        Ok(rgba.into_owned().into_raw())
    }
}

/// B 0x1417bc930. Each mip samples the base image; padded texels repeat the right/bottom edge.
/// The original deliberately materializes this helper's result as RGBA8 even for sRGB input.
fn next_level(base: &DynamicImage, dimensions: [u32; 2], level: u32, block: u32) -> DynamicImage {
    let width = (dimensions[0] >> level).max(1);
    let height = (dimensions[1] >> level).max(1);
    let resized = base
        .resize_exact(width, height, FilterType::CatmullRom)
        .into_rgba8();
    DynamicImage::ImageRgba8(pad_edges(
        &resized,
        [align(width, block), align(height, block)],
    ))
}
fn align(value: u32, block: u32) -> u32 {
    value + (block - value % block) % block
}

/// Private regular-image producer, B 0x1417bbc10. Public `image` and Intel compressors
/// supply codec/filter/block algorithms; this recovers the game's transfer/mip/padding policy.
pub fn process_regular_image(desc: &TextureAssetDesc) -> Result<TextureAssetBlob> {
    let decoded = image::io::Reader::open(&desc.path)
        .with_context(|| format!("Opening {:?}", desc.path))?
        .decode()
        .with_context(|| format!("Decoding {:?}", desc.path))?;
    process_decoded_image(decoded, &desc.opts)
}

pub fn process_decoded_image(
    decoded: DynamicImage,
    opts: &TextureOpts,
) -> Result<TextureAssetBlob> {
    let compressed =
        opts.compression != CompressionMethod::None && decoded.width() > 3 && decoded.height() > 3;
    let block = if compressed { 4 } else { 1 };
    let mode = match opts.compression {
        CompressionMethod::Color => 2,
        CompressionMethod::Grayscale => 1,
        CompressionMethod::None => 3,
    };
    let mut base = match opts.gamma {
        Gamma::Linear => DynamicImage::ImageRgba8(decoded.into_rgba8()),
        Gamma::Srgb => {
            let mut rgba = decoded.into_rgba32f();
            for p in rgba.pixels_mut() {
                for channel in &mut p.0[..3] {
                    *channel = srgb_eotf(*channel);
                }
            }
            DynamicImage::ImageRgba32F(rgba)
        }
    };
    let dims = opts.resize.unwrap_or([base.width(), base.height()]);
    let dims = [align(dims[0], block), align(dims[1], block)];
    if opts.resize.is_some() || dims != [base.width(), base.height()] {
        base = base.resize_exact(dims[0], dims[1], FilterType::CatmullRom);
    }
    let count = mip_count(dims, opts.mips);
    let mut data = encode_level(&base, opts.gamma, compressed, mode)?;
    for level in 1..count {
        let mip = next_level(&base, dims, level, block);
        data.extend(encode_level(&mip, opts.gamma, compressed, mode)?);
    }
    let format = match (compressed, opts.compression, opts.gamma) {
        (false, _, Gamma::Linear) => 37,
        (false, _, Gamma::Srgb) => 43,
        (true, CompressionMethod::Color, Gamma::Linear) => 145,
        (true, CompressionMethod::Color, Gamma::Srgb) => 146,
        (true, CompressionMethod::Grayscale, Gamma::Linear) => 139,
        (true, CompressionMethod::Grayscale, Gamma::Srgb) => {
            panic!("sRGB grayscale compression is not implemented")
        }
        (true, CompressionMethod::None, _) => unreachable!(),
    };
    Ok(TextureAssetBlob {
        data,
        dimensions: [dims[0], dims[1], 1],
        mips: count as u16,
        layers: 1,
        format,
        kind: 2,
    })
}

/// B's DDS route is a pass-through of layer zero, not a recompression pipeline.
/// Resize/gamma/mip/compression options deliberately do not modify a DDS source.
pub fn process_dds_image(path: &Path) -> Result<TextureAssetBlob> {
    let file = std::fs::File::open(path).with_context(|| format!("Opening DDS {path:?}"))?;
    process_dds(ddsfile::Dds::read(file).with_context(|| format!("Reading DDS {path:?}"))?)
}
pub fn process_dds(dds: ddsfile::Dds) -> Result<TextureAssetBlob> {
    use ddsfile::{D3D10ResourceDimension as R, D3DFormat as L, DxgiFormat as D};
    let format = match dds.get_dxgi_format() {
        Some(D::R9G9B9E5_SharedExp) => 123,
        Some(D::BC1_UNorm_sRGB) => 134,
        Some(D::BC3_UNorm) => 137,
        Some(D::BC3_UNorm_sRGB) => 138,
        Some(D::BC5_UNorm) => 141,
        Some(D::BC5_SNorm) => 142,
        // These legacy FourCC cases have fixed gamma in the original producer.
        None => match dds.get_d3d_format() {
            Some(L::DXT1) => 134,
            Some(L::DXT5) => 138,
            _ => match dds.header.spf.fourcc {
                Some(ddsfile::FourCC(0x32495441)) => 141,
                _ => anyhow::bail!("Unsupported DDS format"),
            },
        },
        other => anyhow::bail!("Unsupported DDS format {other:?}"),
    };
    let (kind, layers) = if let Some(header) = &dds.header10 {
        let kind = match header.resource_dimension {
            R::Texture1D => 0,
            R::Texture2D => 2,
            R::Texture3D => 4,
            R::Unknown | R::Buffer => anyhow::bail!("DDS dimension is not a texture"),
        };
        (
            kind,
            u16::try_from(header.array_size).context("DDS array has too many layers")?,
        )
    } else {
        assert_eq!(dds.get_depth(), 1, "Legacy DDS depth must be one");
        (2, 1)
    };
    let data = dds.get_data(0).context("DDS layer zero data")?.to_vec();
    Ok(TextureAssetBlob {
        data,
        dimensions: [dds.get_width(), dds.get_height(), dds.get_depth()],
        mips: dds.get_num_mipmap_levels() as u16,
        layers,
        format,
        kind,
    })
}

/// Array layers carry identical format/dimensions/mip count and remain in layer-major order.
pub fn assemble_array(layers: Vec<TextureAssetBlob>) -> Result<TextureAssetBlob> {
    let count = u16::try_from(layers.len()).context("Texture array has too many layers")?;
    let mut it = layers.into_iter();
    let mut result = it.next().context("Texture array has no layers")?;
    for layer in it {
        anyhow::ensure!(
            layer.dimensions == result.dimensions,
            "Texture array dimensions mismatch"
        );
        anyhow::ensure!(
            layer.mips == result.mips,
            "Texture array mip count mismatch"
        );
        anyhow::ensure!(
            layer.format == result.format,
            "Texture array format mismatch"
        );
        anyhow::ensure!(layer.kind == result.kind, "Texture array kind mismatch");
        result.data.extend(layer.data);
    }
    result.layers = count;
    result.kind = 3;
    Ok(result)
}
pub fn process_texture_asset(desc: &TextureAssetDesc) -> Result<TextureAssetBlob> {
    if !desc.layers.is_empty() {
        assemble_array(
            desc.layers
                .iter()
                .map(process_regular_image)
                .collect::<Result<_>>()?,
        )
    } else if desc.path.extension().is_some_and(|s| s == "dds") {
        process_dds_image(&desc.path)
    } else {
        process_regular_image(desc)
    }
}

/// B cache gate compares the whole descriptor, source path mtime and eight-byte file header.
/// It intentionally does not validate the archived payload on a cache hit.
pub fn cache_is_current(
    desc: &TextureAssetDesc,
    mtime: SystemTime,
    previous: Option<&TextureAssetMeta>,
    header: &[u8],
) -> bool {
    previous.is_some_and(|p| p.desc == *desc && p.src_mtime == mtime)
        && header.len() >= 8
        && header[..4] == TEXTURE_MAGIC.to_le_bytes()
        && header[4..8] == TEXTURE_VERSION.to_le_bytes()
}

/// Behavioral port of the per-asset entry; destination is explicit instead of changing cwd.
pub fn compile_texture_asset(
    name: &relative_path::RelativePath,
    desc: TextureAssetDesc,
    output_root: &Path,
    previous: Option<&TextureAssetMeta>,
) -> Result<TextureAssetMeta> {
    let mtime = std::fs::metadata(&desc.path)
        .with_context(|| format!("Metadata {:?}", desc.path))?
        .modified()?;
    let path = name.with_extension("texture").to_path(output_root);
    if path.exists() && previous.is_some_and(|p| p.desc == desc && p.src_mtime == mtime) {
        let mut header = [0u8; 8];
        std::fs::File::open(&path)?
            .read_exact(&mut header)
            .context("Reading magic and version")?;
        if cache_is_current(&desc, mtime, previous, &header) {
            return Ok(TextureAssetMeta {
                desc,
                src_mtime: mtime,
            });
        }
    }
    let blob = process_texture_asset(&desc)?;
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    std::fs::write(&path, blob.to_file_bytes()?).with_context(|| format!("Writing {path:?}"))?;
    Ok(TextureAssetMeta {
        desc,
        src_mtime: mtime,
    })
}

/// Source scan, tolerant previous metadata, sorted names, rebuilt metadata and persistent index.
/// Original `easy_parallel::Parallel::each(...).run()` executes one task per descriptor,
/// joins every task, then exposes results in submission order. No private thread pool is used.
pub fn compile_texture_assets(
    root: &Path,
) -> Result<(Vec<RelativePathBuf>, TextureAssetLibraryMeta)> {
    let meta_path = root.join("tmp/textures.meta.ron");
    let previous: TextureAssetLibraryMeta = std::fs::read_to_string(&meta_path)
        .ok()
        .and_then(|s| ron::from_str(&s).ok())
        .unwrap_or_default();
    let descriptors = asset::texture::scan::scan_asset_tree(&root.join("assets-src/textures"))?;
    let mut names: Vec<_> = descriptors.keys().cloned().collect();
    names.sort();
    let output_root = root.join("compiled-assets/textures");
    let results = easy_parallel::Parallel::new()
        .each(descriptors, |(name, desc)| {
            let result =
                compile_texture_asset(&name, desc, &output_root, previous.assets.get(&name));
            (name, result)
        })
        .run();
    let mut meta = TextureAssetLibraryMeta::default();
    for (name, result) in results {
        meta.assets.insert(name, result?);
    }
    std::fs::create_dir_all(meta_path.parent().unwrap())?;
    std::fs::write(meta_path, ron::to_string(&meta)?)?;
    let list_path = root.join("compiled-assets/textures/list.ron");
    std::fs::create_dir_all(list_path.parent().unwrap())?;
    std::fs::write(list_path, ron::to_string(&names)?)?;
    Ok((names, meta))
}

/// B loads the shipped index first, compiling only when requested and the source tree exists.
pub fn compile_or_load_texture_assets(
    root: &Path,
    allow_compile: bool,
) -> Result<Vec<RelativePathBuf>> {
    let text = std::fs::read_to_string(root.join("compiled-assets/textures/list.ron"))?;
    let shipped: Vec<RelativePathBuf> = ron::from_str(&text)?;
    if allow_compile && root.join("assets-src/textures").exists() {
        Ok(compile_texture_assets(root)?.0)
    } else {
        Ok(shipped)
    }
}
