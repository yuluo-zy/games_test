//! `session_selection/mod.rs` — session-selection preview thumbnails.
//!
//! Business logic (contracts/menu-session.md §2):
//!   - `SessionThumbnailsPlugin::build` 0x1408f68c0: insert `ThumbnailTextures`;
//!     PreStartup `thumbnails_startup`.
//!   - `thumbnails_startup` 0x1408f6900: load 4 session shaders; create a 6-slot
//!     thumbnail texture pool (stride 0x68).
//!   - `render_session_selection_previews` 0x1408f6c00 (11245B; Ghidra/ReVa
//!     decompile time out): gated by AppMode; per saved session refresh the
//!     screenshot PNG from disk, assign pool indices, and draw textured quads
//!     with the session-selection shaders.
//!
//! Shader names (O, read from .data): `session_selection_background`,
//! `session_selection_new_glade_thumbnail`, `session_selection_thumbnail`,
//! `session_selection_thumbnail_painterly`.

use crate::consume::pass_builder::{DrawBuilder, RasterPass};
use crate::consume::core::{AssetShaderLibrary, SavedSession, Texture};
use crate::typed_gap::Gap;

/// 会话缩略图纹理池（O 名称；布局 U）。内部 Vec 元素 stride 0x68（O）。
pub struct ThumbnailTextures(pub Gap);

/// 池中单块缩略图（O 名称；0x68B，字段 U）。
pub struct PooledThumbnail(pub Gap);

/// SessionSelection bevy Plugin。
pub struct SessionThumbnailsPlugin(pub Gap);

impl SessionThumbnailsPlugin {
    /// O：0x1408f68c0（`impl$0::build`）。
    /// `insert_resource<ThumbnailTextures>()` + `add_systems<PreStartup,
    /// thumbnails_startup>`（签名 `(Res<GraphicsDeviceResource>, Res<AssetShaderLibrary>,
    /// Res<ShaderWatch>, ResMut<ThumbnailTextures>)`）。
    pub fn build(&mut self) {
        unimplemented!()
    }
}

/// O：0x1408f6900（PreStartup 系统）。
/// `load_shader_into_library` ×4（I：4 个 session_selection shader）+ 6 槽纹理池
/// （`TextureDesc::new_2d + sampled + mip_levels`；closure 产出 0x68B 记录）。
pub fn thumbnails_startup(
    _device: &Gap,
    _shaders: &AssetShaderLibrary,
    _watch: &Gap,
    _thumbnails: &mut ThumbnailTextures,
) {
    // O: load_shader_into_library ×4; TextureDesc::new_2d+sampled+mip_levels;
    //     loop 6 × thumbnails_startup::closure$0 → Vec<PooledThumbnail> (stride 0x68)
    unimplemented!()
}

/// O：0x1408f6c00。会话选择预览系统（Ghidra/ReVa 反编译超时——以 callee 清单 +
/// PDB inline 宿主 + g7-post-ui 遗留反编译复述，contracts/menu-session.md §2）。
pub fn render_session_selection_previews(world: &mut Gap) {
    // O: Res<AppMode>(hash 0xd38dae617fb79c7a) state-2 <9 && !=4 gate
    //     get_raster_handle_by_name ×4 (O strings, 28/37/27/37)
    //     lock ThumbnailTextures; extract_thumbnails_to_display @72 /
    //     find_backed_thumbnail_range @127 / assign_pool_indices @149 /
    //     refresh_screenshot @235 (inline hosts, O)
    //     image open/decode/to_rgba8; path eq_by<Rev<Path::Components>>
    //     RasterPass + per-thumb DrawBuilder + optional_uniform<[u32;2]> +
    //     bind_texture + draw_no_vao
    let _ = world;
    unimplemented!()
}

/// `ThumbnailTextures::extract_thumbnails_to_display`（inline @72，O）。
pub fn extract_thumbnails_to_display(_sessions: &[SavedSession]) -> Vec<Option<&Texture>> {
    unimplemented!()
}

/// `ThumbnailTextures::find_backed_thumbnail_range`（inline @127，O）。
pub fn find_backed_thumbnail_range() -> (usize, usize) {
    unimplemented!()
}

/// `ThumbnailTextures::assign_pool_indices`（inline @149，O）。
pub fn assign_pool_indices(_displayed: &[Option<&Texture>]) -> Vec<usize> {
    unimplemented!()
}

/// `refresh_screenshot`（inline @235，O）：从磁盘刷新单张缩略图。
pub fn refresh_screenshot(_session: &SavedSession) {
    unimplemented!()
}
