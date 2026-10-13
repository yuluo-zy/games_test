//! `menu_background.rs` — menu background blit (boot/menu fullscreen).
//!
//! Business logic (contracts/menu-session.md §1):
//!   - `MenuBackgroundPlugin::build` 0x1408c99c0: insert `MenuBackgroundResources`
//!     with defaults {+0x00=1, +0x08=1, +0x10=0, +0x4e=7}.
//!   - `render_menu_bg` 0x1408c9a30: gated by `Res<AppMode>` (hash
//!     `0xd38dae617fb79c7a`) menu-mode set; locks `MenuBackgroundResources`
//!     (hash `0xfb32e4dcc8287bca`), copies the 7 texture handles, releases the
//!     main camera once (`+0x3e` marker), then blits the background texture
//!     fullscreen with the `blit_stretch_decode_srgb` raster shader (O string).

use crate::consume::pass_builder::{DrawBuilder, RasterPass};
use crate::consume::core::AssetShaderLibrary;
use crate::typed_gap::Gap;

/// 菜单背景资源（O 名称）。布局（O 偏移，语义 I）。
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct MenuBackgroundResources {
    /// +0x00 默认 1。
    pub _f0: u64,
    /// +0x08 默认 1。
    pub _f1: u64,
    /// +0x10 默认 0（标志）。
    pub flag: u8,
    /// +0x18..+0x40 背景/目标纹理句柄组（7 qword，O）。
    pub handles: [Gap; 7],
    /// +0x4e 完成标记（build 默认 7）。
    pub done: u8,
    /// 其余 typed gap。
    pub _gap: [u8; 0xa1],
}

/// MenuBackground bevy Plugin。
pub struct MenuBackgroundPlugin(pub Gap);

impl MenuBackgroundPlugin {
    /// O：0x1408c99c0（`impl$0::build`）。分配默认 `MenuBackgroundResources` 并
    /// `insert_resource`。
    pub fn build(&mut self) {
        // *alloc = {1,1,0, …, +0x4e=7}; insert_resource<MenuBackgroundResources>()
        unimplemented!()
    }
}

/// `impl$1::default`（inline @29，O）：默认 `{+0x00=1,+0x08=1,+0x10=0,+0x4e=7}`。
impl Default for MenuBackgroundResources {
    fn default() -> Self {
        Self {
            _f0: 1,
            _f1: 1,
            flag: 0,
            handles: [Gap; 7],
            done: 7,
            _gap: [0; 0xa1],
        }
    }
}

/// O：0x1408c9a30。菜单背景系统。
/// AppMode 模式位掩码选择（`0xaf>>k`/`0x50>>k`，k=state-2）→ 锁资源拷贝句柄 →
/// 释放主相机（+0x3e 标记）→ `blit_stretch_decode_srgb` 全屏 blit。
pub fn render_menu_bg(world: &mut Gap) {
    // O: AppMode skip-set (state==2 | (state==10 && byte@+0x08==1) | mask0)
    //     MenuBackgroundResources: Mutex lock, copy 7 handles @+0x18..+0x48,
    //     +0x4e=7 marker; +0x3e!=7 → finish<MainCameraPlugin> + drop
    //     get_raster_handle_by_name("blit_stretch_decode_srgb") → RasterPass +
    //     DrawBuilder: bind_texture<&Texture> + draw_no_vao → RasterPass::render
    let _ = world;
    unimplemented!()
}
