//! `load_compiled_texture_image` — load a pre-compiled texture image (WP4-owned).
//!
//! Skeleton bundle: `skeleton/B/crates/country-core/src/render/load_texture.rs`
//! (source_sha256 `a31916ce…`, 1 proc). Decompile `A_0x140917860.c`.

use std::path::{Path, PathBuf};

use crate::rhapsody_contract::{GraphicsDevice, Texture, TextureDesc};

/// O: header magic validated in `load_compiled_texture_image` (0x140917860).
const COMPILED_IMAGE_MAGIC: u32 = 0x4850_2183;
/// O: header field at +4 must equal 8.
const COMPILED_IMAGE_HEADER_VERSION: u32 = 8;

/// `load_compiled_texture_image(path, device, …) -> Result<Texture, _>`
///
/// (O: 0x140917860, 2323B)
/// 1. puffin scope `"load_compiled_texture_image"`.
/// 2. Path = `path.to_path_buf()` joined, `with_extension(…)` (compiled-image path).
/// 3. debug-log; `std::fs::File::open` (io error → panic), `memmap2` mmap + file_len.
/// 4. Validate: 8-byte alignment, magic == `0x48502183`, header +4 == 8,
///    tail mip-count u16 < 0xff.
/// 5. `TextureDesc::new().sampled().mip_levels(..).array_elements(..)`,
///    `ustr::Ustr::from(path)`, `GraphicsDevice::create_texture<&[u8]>`.
/// 6. Return the 0x40-byte `Texture` (reused rhapsody type).
pub fn load_compiled_texture_image(
    path: &Path,
    device: &GraphicsDevice,
) -> Result<Texture, crate::rhapsody_contract::TextureError> {
    let _scope = crate::puffin_contract::scope("load_compiled_texture_image");

    let mut file_path = path.to_path_buf();
    // O: `Path::_join` + `_with_extension` on the compiled asset path.
    file_path = compiled_image_path(&file_path);

    let data = std::fs::read(&file_path).map_err(|_| crate::rhapsody_contract::TextureError::Io)?;

    if data.len() < 8 || data.len() % 8 != 0 {
        panic!("image file length not aligned: {}", file_path.display());
    }
    let magic = u32::from_le_bytes(data[0..4].try_into().unwrap());
    let version = u32::from_le_bytes(data[4..8].try_into().unwrap());
    if magic != COMPILED_IMAGE_MAGIC {
        panic!("Image has an incorrect magic id: {:08x}", magic);
    }
    if version != COMPILED_IMAGE_HEADER_VERSION {
        panic!("Image has an incorrect magic id (version): {}", version);
    }

    // O: tail fields — mip count at len-8 (u16, < 0xff); width/height + format
    // at len-0x14 / len-0xc (bytewise; exact meaning U).
    let mips = u16::from_le_bytes(data[data.len() - 8..data.len() - 6].try_into().unwrap());
    if mips > 0xff {
        panic!("too many mip levels: {}", mips);
    }
    let tail_a = u32::from_le_bytes(
        data[data.len() - 0x14..data.len() - 0x10]
            .try_into()
            .unwrap(),
    );
    let tail_b = u32::from_le_bytes(data[data.len() - 0xc..data.len() - 0x8].try_into().unwrap());
    let _ = (tail_a, tail_b); // U: format/extent fields

    let desc = TextureDesc::new()
        .sampled()
        .mip_levels(mips as u32)
        .array_elements(1);

    device.create_texture(desc, &data, crate::ustr_contract::intern(&file_path))
}

/// O: the game derives the compiled-image path from the source path
/// (`Path::_join` + `_with_extension`); exact extension U.
fn compiled_image_path(path: &Path) -> PathBuf {
    let mut out = path.to_path_buf();
    out.set_extension("bin");
    out
}
