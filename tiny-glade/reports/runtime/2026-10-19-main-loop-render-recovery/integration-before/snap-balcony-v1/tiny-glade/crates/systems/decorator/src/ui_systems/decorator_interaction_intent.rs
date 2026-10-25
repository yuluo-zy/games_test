//! B `ui_systems/decorator_interaction_intent.rs` — 装饰交互意图计算。
//!
//! - `calculate_decorator_interaction_intent` 0x14127cf70（O 结构，48KB）：按
//!   目的 tag 分派（switch @+0x04）——case 0 墙：`PublicWalls::get`
//!   （0x140a91ef0）+ `derive_decorator_subtype`（0x140813d00）+ 部件位置
//!   （0x14125cf90）+ 合并位置（0x14126a040/0x142001880）+ 四角吸附
//!   （0x140c996d0(0..3)，`should_offer_snap_to_corner` 0x1408331c0 门控）+
//!   阳台门吸附（`snap_balcony_door` 0x14127f0e0）；case 1 屋顶：
//!   `can_place_on_roof` 门控；输出 0x98B 意图记录（字段 I，见
//!   [`DecoratorInteractionIntent`]）。
//! - `snap_balcony_door` 0x14127f0e0（O 结构）：窗/门（type 0x1a）吸附到
//!   阳台门边缘的几何搜索（0x140c9a6a0 圆测试 0.8 半径 + 射线 0x1408e3380
//!   + 0x140c98120 角度）；精确几何 I。
use country_core::resources::walls::decorator::{DecoratorInfo, DecoratorType};
use country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo;
use glam::Vec3;

use crate::merge_proposal_wall_decorators::find_best_wall_decorator_merge_position;

/// 0x98B 意图记录（I：字段语义按 0x14127cf70 输出写面；@0 u64 判别、
/// @0x10 位置簇、@0x28 墙信息、@0x40+ 吸附/合并结果）。
#[derive(Clone, Copy, Debug, Default)]
pub struct DecoratorInteractionIntent {
    /// @0：目的（param_12 观察；I）。
    pub dst_tag: u8,
    /// @0x10：墙空间位置（u64 对打包；I）。
    pub wall_pos: (u32, u32),
    /// @0x18：高度（I）。
    pub height: f32,
    /// @0x28..0x38：墙信息（I）。
    pub wall_info: [u64; 2],
    /// @0x40：有效标志（=1；I）。
    pub valid: u64,
    /// @0x50：世界位置（I）。
    pub pos: Vec3,
    /// @0x60：结果/合并（I）。
    pub result: u64,
    /// @0x70：u8（I）。
    pub sub: u8,
    /// @0x39..0x3a：标志位（I）。
    pub flags: [u8; 2],
    /// @0x80：f32（I）。
    pub f80: f32,
    /// @0x88：u64（I）。
    pub u88: u64,
    /// @0x98：f32（I）。
    pub f98: f32,
    /// @0xbc：rank（I）。
    pub rank: u8,
}

/// B 0x14127cf70（O 结构）。命中 + 装饰信息 → 交互意图。
/// 精确几何（四角吸附/合并/阳台门）I；输出记录字段见上。
pub fn calculate_decorator_interaction_intent(
    _hit: &crate::ui_systems::calculate_decorator_grab_offset::RaycastIntersection,
    info: &DecoratorInfo,
    subtype: &DerivedDecoratorInfo,
    wall: Option<&country_core::resources::walls::public_wall_state::PublicWallState>,
    storage: &country_core::resources::walls::decorator_storage::DecoratorStorage,
    walls: &country_core::resources::walls::public_walls::PublicWalls,
    cursor_ws: Vec3,
) -> DecoratorInteractionIntent {
    let mut out = DecoratorInteractionIntent::default();
    out.dst_tag = info.dst_tag();
    out.pos = cursor_ws;
    match info.dst_tag() {
        // case 0：墙（O：PublicWalls::get + subtype + 部件位置 + 合并 + 四角吸附）。
        0 => {
            let Some(wall) = wall else {
                return out;
            };
            let _ = subtype;
            // 合并位置（0x14126a040/0x142001880 族；I；D3 签名
            // `(&[WallId], &DecoratorInfo, &DecoratorStorage, &PublicWalls)`）。
            let _merge = find_best_wall_decorator_merge_position(&[], info, storage, walls);
            // 四角吸附（O：0x140c996d0(0..3) + should_offer_snap_to_corner）。
            if info.decorator_type().should_offer_snap_to_corner() {
                for corner in 0..4u8 {
                    let _ = corner_snap_candidate(corner, wall, cursor_ws);
                }
            }
            out.valid = 1;
        }
        // case 1：屋顶（O：can_place_on_roof 门控）。
        1 => {
            if !info.decorator_type().can_place_on_roof() {
                return out;
            }
            out.valid = 1;
        }
        // case 2/3/4：地形/楼梯（O 结构；位置由宿主投影）。
        2 | 3 | 4 => {
            out.valid = 1;
        }
        _ => {}
    }
    out
}

/// 四角吸附候选（I：0x140c996d0(corner) + 圆/矩形包含测试；几何 U）。
#[allow(clippy::unnecessary_wraps)]
fn corner_snap_candidate(
    corner: u8,
    _wall: &country_core::resources::walls::public_wall_state::PublicWallState,
    _cursor_ws: Vec3,
) -> Option<u8> {
    let _ = corner;
    None
}

/// B 0x14127f0e0（O 结构）：阳台门吸附。门/窗类（type==0x1a，字符 `\x1a`
/// 判别观察）在墙上按部件位置搜索可吸附阳台门边；精确几何 I。
pub fn snap_balcony_door(
    _info: &DecoratorInfo,
    _wall: &country_core::resources::walls::public_wall_state::PublicWallState,
    _cursor_ws: Vec3,
) -> Option<u64> {
    let _ = DecoratorType::default();
    None
}
