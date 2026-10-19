//! B `cottage_dormers/util_assemble_gothic_dormer.rs` — 哥特老虎窗瓦装配工具（D1）。
//!
//! 证据：fresh RE-MCP 反编译 0x141290b30（assemble_gothic_dormer_tiles
//! closure_env<6>，28KB）+ 骨架 callees（O）：哥特剖面曲线消费、
//! LazyTile::from_index、halver to_f16（0x140c8ffb0）、swizzlers、瓦片变换装配。
//!
//! 业务（O 结构）：按哥特老虎窗剖面（gothic_dormer_profile_curve）装配屋顶瓦
//! （瓦坐标 f16 打包 + 索引）。
//!
//! 本文件落地可独立复核的部分：**f32 → f16 位打包**（`halver to_f16`
//! 0x140c8ffb0 的语义），以及把剖面点转为 f16 瓦坐标的记录类型。
//! 瓦列步进/索引顺序的数学主体仍 U。
//!
//! U 登记：① 瓦片装配算法（步进、重叠、索引） ② 参数面。

use glam::Vec2;
use half::f16;

/// 单个瓦坐标记录（O：2×f16 打包）。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct GothicDormerTile {
    /// 打包后的 `[u, v]` f16 位。
    pub packed: [u16; 2],
}

/// `halver to_f16`（0x140c8ffb0，O）：f32 → f16 位模式。
#[inline]
pub fn to_f16_bits(v: f32) -> u16 {
    f16::from_f32(v).to_bits()
}

/// f16 位 → f32（往返验证/调试用）。
#[inline]
pub fn from_f16_bits(bits: u16) -> f32 {
    f16::from_bits(bits).to_f32()
}

/// 把二维点打包为瓦坐标记录（O 打包格式；用途 U）。
pub fn pack_tile_point(p: Vec2) -> GothicDormerTile {
    GothicDormerTile {
        packed: [to_f16_bits(p.x), to_f16_bits(p.y)],
    }
}

/// 0x141290b30（O 结构）：装配哥特老虎窗瓦片（closure_env<6>）。
///
/// 数学主体 U → 返回空（不以伪造瓦列冒充）。剖面点打包接口已落地，
/// 供取证完成后直接消费。
pub fn assemble_gothic_dormer_tiles(
    _profile: &[Vec2],
    _roof_height: f32,
) -> Vec<GothicDormerTile> {
    Vec::new() // U：瓦列步进/索引待取证。
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn f16_conversion_roundtrips_documented_values() {
        assert_eq!(to_f16_bits(0.0), 0x0000);
        assert_eq!(to_f16_bits(1.0), 0x3c00);
        assert_eq!(to_f16_bits(0.5), 0x3800);
        assert!((from_f16_bits(to_f16_bits(0.35)) - 0.35).abs() < 1e-3);
    }

    #[test]
    fn pack_tile_point_uses_two_f16() {
        let t = pack_tile_point(Vec2::new(0.76, 0.18));
        assert_eq!(t.packed[0], to_f16_bits(0.76));
        assert_eq!(t.packed[1], to_f16_bits(0.18));
    }
}
