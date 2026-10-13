//! `grading_lut.rs` — color grading LUT resources (3D LUT textures).
//!
//! Business logic (contracts/lut.md §2):
//!   - `ColorGradingState`: current grading selection (name + 4 values);
//!     `flattened` 0x140b00b20 feeds `final_post` and `prepare_frame_constants`.
//!   - `GradingLut`: single 3D LUT. `load_unwrapped` 0x140b00bc0 (unwrapped PNG →
//!     RGBA8 → 3D texture); `load_cube` 0x140b01050 (full `.cube` parser: TITLE /
//!     LUT_3D_SIZE / DOMAIN_MIN|MAX / data lines → RGB9E5 pack → 3D texture);
//!     `parse_rgb` 0x140b01cc0 (3 f32 per line).
//!   - `GradingLuts::load` 0x140b01df0: readdir over the LUT directory, per-file
//!     closure load.
//!   - `LutGalleryState`: gallery state resource (inserted by lut_gallery::build,
//!     default layout O).

use crate::typed_gap::Gap;

/// 当前颜色分级选择（O 名称）。布局（O 偏移，语义 I）：
/// +0x00 String 名、+0x18..+0x24 4×f32、+0x28 byte 已命名标志。
#[derive(Clone, Copy, Debug, Default)]
#[repr(C)]
pub struct ColorGradingState {
    /// +0x00 LUT 名称（String {ptr,len,cap}，默认 `"default"`）。
    pub name: Gap,
    /// +0x18..+0x24 分级值（默认 {1.0,0.0,1.0,1.0}；语义 U：exposure/contrast/…）。
    pub values: [f32; 4],
    /// +0x28 已命名标志。
    pub named: u8,
}

impl ColorGradingState {
    /// O：0x140b00b20。扁平化输出（name + 4 值 + byte）；被 `final_post@50` 与
    /// `prepare_frame_constants@50` inline 消费。
    pub fn flattened(&self) -> FlattenedGrading {
        unimplemented!()
    }
}

/// `ColorGradingState::flattened` 输出（O：{String, 4×f32, byte}；布局 U）。
pub struct FlattenedGrading(pub Gap);

/// 单条 3D LUT（O 名称）。布局（O 偏移）：+0x00 String 名、+0x18 size、
/// +0x28..+0x58 3D 纹理句柄区。
#[derive(Clone, Copy, Debug, Default)]
#[repr(C)]
pub struct GradingLut {
    /// +0x00 LUT 名。
    pub name: Gap,
    /// +0x18..+0x20 尺寸（cube size）。
    pub size: u32,
    /// +0x28..+0x58 3D 纹理句柄（typed gap）。
    pub texture: Gap,
}

impl GradingLut {
    /// O：0x140b00bc0。展平 PNG → RGBA8 → 3D 纹理（`new_3d + sampled +
    /// create_texture<&[u8]>`）。返回 Result。
    pub fn load_unwrapped(_path: &Gap) -> Result<GradingLut, Gap> {
        unimplemented!()
    }

    /// O：0x140b01050。完整 `.cube` 解析器（TITLE/LUT_3D_SIZE/DOMAIN_*/数据行；
    /// RGB9E5 打包 → 3D 纹理）。
    pub fn load_cube(_path: &Gap) -> Result<GradingLut, Gap> {
        unimplemented!()
    }

    /// O（inline 宿主）：`lut_name_to_asset_name`（final_post@82/lut_gallery@82）。
    pub fn lut_name_to_asset_name(&self, _lut_name: &str) -> Gap {
        unimplemented!()
    }

    /// O（inline 宿主）：`asset_name`（lut_gallery@78）。
    pub fn asset_name(&self) -> &Gap {
        unimplemented!()
    }
}

/// `impl$2::load_cube::parse_rgb`（O，0x140b01cc0）：按空白解析 3 个 f32。
pub fn parse_rgb(_line: &str) -> Result<[f32; 3], Gap> {
    unimplemented!()
}

/// 分级 LUT 集合（O 名称）。布局（O 偏移）：+0x00 Vec<stride 0x28>、
/// +0x18 Vec<stride 0x38>。
#[derive(Clone, Copy, Debug, Default)]
#[repr(C)]
pub struct GradingLuts {
    /// +0x00 目录条目 Vec A（stride 0x28）。
    pub _list_a: Gap,
    /// +0x18 目录条目 Vec B（stride 0x38）。
    pub _list_b: Gap,
}

impl GradingLuts {
    /// O：0x140b01df0。`readdir` LUT 目录 + 每 `DirEntry` 调 `impl$3::load::closure_env$0`
    ///（load_cube/load_unwrapped，I）→ 两 Vec。
    pub fn load(_dir: &Gap) -> Result<GradingLuts, Gap> {
        unimplemented!()
    }
}

/// LUT 画廊状态：真实类型在 `systems::render_loop::lut_gallery`（O 16 B：状态 Arc + 预览槽）。
/// 旧的 0x40 B 全零占位已删除，这里只 re-export（`lib.rs` 的 `consume::core` 仍经此路径取用）。
pub use crate::systems::render_loop::lut_gallery::LutGalleryState;
