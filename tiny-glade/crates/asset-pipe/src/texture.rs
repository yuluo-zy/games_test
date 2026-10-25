//! Texture decoding, encoding and persistent compilation.
mod array;
mod cache;
mod compile;
mod dds;
mod image;
mod layout;
mod output;

pub use array::{assemble_array, process_texture_asset};
pub use cache::{TextureAssetLibraryMeta, TextureAssetMeta, TextureSourceStamp, cache_is_current};
pub use compile::{compile_or_load_texture_assets, compile_texture_asset, compile_texture_assets};
pub use dds::{process_dds, process_dds_image};
pub use image::{
    bc7_settings, block_bytes, compress, mip_count, pad_edges, process_decoded_image,
    process_regular_image, srgb_eotf, srgb_oetf,
};
