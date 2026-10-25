use anyhow::{Context, Result};
use asset::texture::{
    blob::TextureAssetBlob,
    desc::{CompressionMethod, Gamma, MipChain, TextureAssetDesc, TextureOpts},
};
use image::{ColorType, DynamicImage, ImageBuffer, Pixel, RgbaImage, imageops::FilterType};
use std::borrow::Cow;

/// Convert linear RGB to sRGB; alpha channels must not pass through this transfer.
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

pub fn mip_count(dimensions: [u32; 2], chain: MipChain) -> Result<u32> {
    anyhow::ensure!(
        dimensions[0] > 0 && dimensions[1] > 0,
        "Texture dimensions must be nonzero"
    );
    let full = 32 - dimensions[0].max(dimensions[1]).leading_zeros();
    Ok(match chain {
        MipChain::None => 1,
        MipChain::Full => full,
        MipChain::Limited(n) => {
            anyhow::ensure!(n > 0, "Texture mip limit must be nonzero");
            n.min(full)
        }
    })
}
/// Extend a nonempty image by repeating its bottom and right edges.
pub fn pad_edges(image: &RgbaImage, dimensions: [u32; 2]) -> Result<RgbaImage> {
    padded_pixels(image, dimensions)
}

fn padded_pixels<P: Pixel>(
    image: &ImageBuffer<P, Vec<P::Subpixel>>,
    dimensions: [u32; 2],
) -> Result<ImageBuffer<P, Vec<P::Subpixel>>> {
    anyhow::ensure!(
        image.width() > 0 && image.height() > 0,
        "Cannot pad an empty image"
    );
    anyhow::ensure!(
        dimensions[0] >= image.width() && dimensions[1] >= image.height(),
        "Padded dimensions cannot shrink the source image"
    );
    let elements = super::layout::image_elements(
        dimensions[0],
        dimensions[1],
        P::CHANNEL_COUNT,
        size_of::<P::Subpixel>(),
    )?;
    let mut pixels = Vec::new();
    pixels.try_reserve_exact(elements)?;
    pixels.resize(elements, image.get_pixel(0, 0).channels()[0]);
    let mut padded = ImageBuffer::<P, _>::from_raw(dimensions[0], dimensions[1], pixels)
        .context("Invalid padded image layout")?;
    for (x, y, pixel) in padded.enumerate_pixels_mut() {
        *pixel = *image.get_pixel(x.min(image.width() - 1), y.min(image.height() - 1));
    }
    Ok(padded)
}
/// Discriminants are private compression modes: BC5=0, BC4=1, BC7=2.
pub fn compress(image: &RgbaImage, mode: u8) -> Result<Vec<u8>> {
    anyhow::ensure!(
        image.width() > 0 && image.height() > 0,
        "Compression dimensions must be nonzero"
    );
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
                stride: image
                    .width()
                    .checked_mul(2)
                    .context("RG stride overflows u32")?,
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
                stride: image
                    .width()
                    .checked_mul(4)
                    .context("RGBA stride overflows u32")?,
            };
            let settings = bc7_settings(image.pixels().all(|p| p[3] == 255));
            Ok(intel_tex_2::bc7::compress_blocks(&settings, &surface))
        }
        _ => anyhow::bail!("Unsupported compression mode {mode}"),
    }
}
/// Byte size of one 4x4 block for the private compression modes
/// (BC5=0, BC4=1, BC7=2). DDS/BC format constants, not a game algorithm.
pub fn block_bytes(mode: u8) -> Result<usize> {
    Ok(match mode {
        0 | 2 => 16, // BC5 and BC7 use 16-byte blocks.
        1 => 8,      // BC4 uses 8-byte blocks.
        _ => anyhow::bail!("Unsupported compression mode {mode}"),
    })
}
/// Select the BC7 quality preset, using the cheaper opaque path when possible.
pub fn bc7_settings(opaque: bool) -> intel_tex_2::bc7::EncodeSettings {
    if opaque {
        intel_tex_2::bc7::opaque_basic_settings()
    } else {
        intel_tex_2::bc7::alpha_basic_settings()
    }
}

// Filtering uses linear RGB; encode RGB for storage without altering alpha.
fn with_rgba8(image: &DynamicImage, gamma: Gamma) -> Cow<'_, RgbaImage> {
    match gamma {
        // Borrow existing RGBA pixels to avoid a full-image copy before compression.
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

// Filter in the working color space. In particular, preserve linear float precision
// for sRGB until the final transfer function; early u8 quantization crushes dark mips.
fn next_level(
    base: &DynamicImage,
    dimensions: [u32; 2],
    level: u32,
    block: u32,
) -> Result<DynamicImage> {
    let width = (dimensions[0] >> level).max(1);
    let height = (dimensions[1] >> level).max(1);
    let resized = resize_filtered(base, width, height)?;
    let padded = [align(width, block)?, align(height, block)?];
    if padded == [width, height] {
        return Ok(resized);
    }
    match resized {
        DynamicImage::ImageRgba32F(pixels) => {
            Ok(DynamicImage::ImageRgba32F(padded_pixels(&pixels, padded)?))
        }
        image => Ok(DynamicImage::ImageRgba8(pad_edges(
            &image.into_rgba8(),
            padded,
        )?)),
    }
}
fn ensure_finite(channels: &[f32]) -> Result<()> {
    anyhow::ensure!(
        channels.iter().all(|value| value.is_finite()),
        "Floating image channels must be finite"
    );
    Ok(())
}

// The archive stores straight alpha. Filtering requires premultiplied linear RGB
// so invisible source colors cannot darken the visible edge of a translucent pixel.
fn resize_filtered(image: &DynamicImage, width: u32, height: u32) -> Result<DynamicImage> {
    super::layout::image_elements(width, height, 4, size_of::<f32>())?;
    super::layout::image_elements(image.width(), image.height(), 4, size_of::<f32>())?;
    let mut pixels = image.to_rgba32f();
    let mut translucent = false;
    for pixel in pixels.pixels_mut() {
        let alpha = pixel[3];
        translucent |= alpha < 1.0;
        for channel in &mut pixel.0[..3] {
            *channel *= alpha;
        }
        ensure_finite(&pixel.0)?;
    }
    // image's float resampler clips channels to [0, 1]. Negative-lobe filters
    // can clip alpha independently of premultiplied RGB and create color halos.
    // Use nonnegative triangle weights for coverage; opaque color keeps Catmull-Rom.
    let filter = if translucent {
        FilterType::Triangle
    } else {
        FilterType::CatmullRom
    };
    let mut resized = DynamicImage::ImageRgba32F(pixels)
        .resize_exact(width, height, filter)
        .into_rgba32f();
    for pixel in resized.pixels_mut() {
        ensure_finite(&pixel.0)?;
        // Undo premultiplication before clamping the stored coverage.
        let alpha = pixel[3];
        pixel[3] = alpha.clamp(0.0, 1.0);
        for channel in &mut pixel.0[..3] {
            *channel = if alpha > 0.0 { *channel / alpha } else { 0.0 };
        }
        ensure_finite(&pixel.0)?;
    }
    Ok(DynamicImage::ImageRgba32F(resized))
}

fn align(value: u32, block: u32) -> Result<u32> {
    value
        .checked_add((block - value % block) % block)
        .context("Texture block alignment overflows u32")
}

/// Decode a PNG or JPEG and apply its texture options.
pub fn process_regular_image(desc: &TextureAssetDesc) -> Result<TextureAssetBlob> {
    let decoded = image::io::Reader::open(&desc.path)
        .with_context(|| format!("Opening {:?}", desc.path))?
        .decode()
        .with_context(|| format!("Decoding {:?}", desc.path))?;
    process_decoded_image(decoded, &desc.opts)
}

pub fn process_decoded_image(
    decoded: DynamicImage,
    options: &TextureOpts,
) -> Result<TextureAssetBlob> {
    // The image dependency panics when converting NaN to integer channels.
    // Validate floating inputs before any narrowing or color-space conversion.
    match &decoded {
        DynamicImage::ImageRgb32F(pixels) => ensure_finite(pixels.as_raw())?,
        DynamicImage::ImageRgba32F(pixels) => ensure_finite(pixels.as_raw())?,
        _ => {}
    }
    anyhow::ensure!(
        decoded.width() > 0 && decoded.height() > 0,
        "Source image dimensions must be nonzero"
    );
    let compressed = options.compression != CompressionMethod::None
        && decoded.width() > 3
        && decoded.height() > 3;
    let block = if compressed { 4 } else { 1 };
    let dimensions = options
        .resize
        .unwrap_or([decoded.width(), decoded.height()]);
    anyhow::ensure!(
        dimensions[0] > 0 && dimensions[1] > 0,
        "Texture dimensions must be nonzero"
    );
    let dimensions = [align(dimensions[0], block)?, align(dimensions[1], block)?];
    super::layout::image_elements(dimensions[0], dimensions[1], 4, size_of::<f32>())?;
    // Alignment can increase the full mip chain, so use the final dimensions.
    let mip_levels = mip_count(dimensions, options.mips)?;
    anyhow::ensure!(
        !(compressed
            && options.compression == CompressionMethod::Grayscale
            && options.gamma == Gamma::Srgb),
        "sRGB grayscale compression is not supported"
    );
    let mode = match options.compression {
        CompressionMethod::Color => 2,
        CompressionMethod::Grayscale => 1,
        CompressionMethod::None => 3,
    };
    let mut base = match options.gamma {
        Gamma::Linear => {
            if matches!(
                decoded.color(),
                ColorType::L8 | ColorType::La8 | ColorType::Rgb8 | ColorType::Rgba8
            ) {
                DynamicImage::ImageRgba8(decoded.into_rgba8())
            } else {
                // Keep 16-bit/float samples until filtering has finished; early
                // quantization changes averages and can erase low-level detail.
                DynamicImage::ImageRgba32F(decoded.into_rgba32f())
            }
        }
        Gamma::Srgb => {
            let mut rgba = decoded.into_rgba32f();
            for p in rgba.pixels_mut() {
                for channel in &mut p.0[..3] {
                    *channel = srgb_eotf(*channel);
                }
                ensure_finite(&p.0)?;
            }
            DynamicImage::ImageRgba32F(rgba)
        }
    };
    if dimensions != [base.width(), base.height()] {
        base = resize_filtered(&base, dimensions[0], dimensions[1])?;
    }
    let data = if mip_levels == 1 && !compressed && options.gamma == Gamma::Linear {
        // Without additional mips, transfer ownership of the pixels to the archive.
        base.into_rgba8().into_raw()
    } else {
        let mut data = encode_level(&base, options.gamma, compressed, mode)?;
        for level in 1..mip_levels {
            let mip = next_level(&base, dimensions, level, block)?;
            data.extend(encode_level(&mip, options.gamma, compressed, mode)?);
        }
        data
    };
    let format = match (compressed, options.compression, options.gamma) {
        (false, _, Gamma::Linear) => 37,
        (false, _, Gamma::Srgb) => 43,
        (true, CompressionMethod::Color, Gamma::Linear) => 145,
        (true, CompressionMethod::Color, Gamma::Srgb) => 146,
        (true, CompressionMethod::Grayscale, Gamma::Linear) => 139,
        (true, CompressionMethod::Grayscale, Gamma::Srgb) => {
            unreachable!("validated before encoding")
        }
        (true, CompressionMethod::None, _) => unreachable!(),
    };
    Ok(TextureAssetBlob {
        data,
        dimensions: [dimensions[0], dimensions[1], 1],
        mips: mip_levels as u16,
        layers: 1,
        format,
        kind: 2,
    })
}
