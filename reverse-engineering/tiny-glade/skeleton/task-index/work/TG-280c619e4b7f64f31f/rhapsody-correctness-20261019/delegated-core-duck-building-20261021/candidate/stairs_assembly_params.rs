//! B stairs assembly geometry leaves. The full assembly-request owner and its
//! graph-to-geometry producer are separate; no missing request fields default.
use glam::Vec2;

/// B 0x140959e80 returns the supplied packed Vec2 and angle unchanged.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct StairWallAnchor {
    pub position: Vec2,
    pub angle: f32,
}
impl StairWallAnchor {
    pub fn new(position: Vec2, angle: f32) -> Self {
        Self { position, angle }
    }
    /// B 0x140959e90 calls sinf then cosf and XORs both results with 0x80000000.
    /// The old chapter omitted those flips. These point into the wall.
    pub fn wall_normal(self) -> Vec2 {
        let x = f32::from_bits(self.angle.sin().to_bits() ^ 0x8000_0000);
        let y = f32::from_bits(self.angle.cos().to_bits() ^ 0x8000_0000);
        Vec2::new(x, y)
    }
}

/// Reads only the confirmed StairNodeAnchor::Wall payload. Retaining a slice
/// keeps the other variants and their still-unresolved payload layout intact.
pub struct ObservedStairNodeAnchor<'a>(&'a [u8]);
impl<'a> ObservedStairNodeAnchor<'a> {
    pub fn from_observed(bytes: &'a [u8]) -> Self {
        assert!(
            bytes.len() >= 4,
            "StairNodeAnchor requires its discriminant"
        );
        Self(bytes)
    }
    pub fn try_as_wall(&self) -> Option<StairWallAnchor> {
        // B 0x140959ef0 compares the four-byte discriminant to 2 and returns +8.
        if u32::from_le_bytes(self.0[..4].try_into().unwrap()) != 2 {
            return None;
        }
        assert!(
            self.0.len() >= 20,
            "StairNodeAnchor::Wall requires its payload"
        );
        Some(StairWallAnchor::new(
            Vec2::new(read(self.0, 8), read(self.0, 12)),
            read(self.0, 16),
        ))
    }
}

/// 原 B 的 AlignKind 值为20字节、对齐4；末尾不能混入外层候选的score。
/// 原字段：tag/flag/byte2，first@4，second@0xc；两个数学方法按原字段读取。
/// 完整证据：14080beb0三项候选stride24（kind20+score4），
/// 14080c360返回项stride24（DecoratorId4+kind20），见本轮类型合同。
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct AlignKind {
    /// +0x0 variant tag (0 / 1).
    pub tag: u8,
    /// +0x1 boolean payload used by the variant formulas.
    pub flag: u8,
    /// +0x2 second discriminator byte (variant 0 requires `flag == 0 || byte2 == 1`).
    pub byte_2: u8,
    /// +0x3 padding.
    pub pad_0: u8,
    /// +0x4 first Vec2 payload.
    pub first: Vec2,
    /// +0xc second Vec2 payload.
    pub second: Vec2,
}

impl AlignKind {
    /// 原值20字节；容器或外层tuple的padding不计入本值。
    pub const NATIVE_SIZE: usize = 0x14;

    /// B 0x140959f00. The f32 scale argument is in XMM1. Unsupported variants
    /// return the original zero.
    pub fn position_offset(&self, scale: f32) -> Vec2 {
        let mut out = Vec2::ZERO;
        match self.tag {
            0 => {
                if self.flag == 0 || self.byte_2 == 1 {
                    out = self.first * scale;
                }
            }
            1 => {
                let mut direction = self.first;
                if self.flag != 0 {
                    direction += self.second;
                }
                out = direction * (scale * 0.5);
            }
            _ => panic!("AlignKind::None has no defined position_offset"),
        }
        out
    }

    /// B 0x140959f50 returns both scalar result registers.
    pub fn outgoing_dir(&self) -> Vec2 {
        match self.tag {
            0 => self.first,
            1 => self.second,
            _ => panic!("AlignKind::None has no defined outgoing_dir"),
        }
    }
}

/// Native `system_decorator::decorator_visual::stairs::PreferredNodeAlignment`
/// element as pushed by the wall-edit producers (fresh 2026-10-17).
///
/// The `grow_one` symbol names the container
/// `Vec<(DecoratorId, Vec<PreferredNodeAlignment>)>`; the observed 0x20-byte push
/// is `{q0 @0, q1 @8, q2 @0x10, tag @0x18, byte @0x1c, byte @0x1d, pad @0x1e}`.
/// Both `edit_rectangle` (0x14097f620, bytes `04 00`) and
/// `adjust_wall_height_impl` (0x140ac1f20, bytes `00 <bool>`) write that record.
/// Field semantics beyond the offsets are **not** proven; they stay explicitly
/// numbered instead of being invented.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct PreferredNodeAlignment {
    /// +0x00 (qword).
    pub q0: u64,
    /// +0x08 (qword).
    pub q1: u64,
    /// +0x10 (qword).
    pub q2: u64,
    /// +0x18 (dword).
    pub tag: u32,
    /// +0x1c (byte; `4` at the `edit_rectangle` site, `0` at the
    /// `adjust_wall_height` site).
    pub byte_1c: u8,
    /// +0x1d (byte; `0` at `edit_rectangle`, a boolean at `adjust_wall_height`).
    pub byte_1d: u8,
    /// +0x1e terminal padding.
    pub pad_1e: [u8; 2],
}

impl PreferredNodeAlignment {
    /// Native element stride 0x20.
    pub const NATIVE_SIZE: usize = 0x20;
}

/// The confirmed 20 bytes read by AlignKind's two geometry methods. Variant
/// names are unproven, so the original tag and boolean bytes remain explicit.
/// 保留为20字节 [`AlignKind`] 的字节外观。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ObservedAlignKind([u8; 20]);
impl ObservedAlignKind {
    pub fn from_observed(bytes: [u8; 20]) -> Self {
        Self(bytes)
    }
    pub fn observed(&self) -> &[u8; 20] {
        &self.0
    }
    /// B 0x140959f00. The f32 scale argument is in XMM1; it was omitted by the
    /// old failed decompilation. Unsupported variants return the original zero.
    pub fn position_offset(&self, scale: f32) -> Vec2 {
        match self.0[0] {
            0 if self.0[1] != 0 || self.0[2] == 1 => self.vec2(4) * scale,
            1 => {
                let mut direction = self.vec2(4);
                if self.0[1] != 0 {
                    direction += self.vec2(12);
                }
                direction * (scale * 0.5)
            }
            _ => Vec2::ZERO,
        }
    }
    /// B 0x140959f50 returns both scalar result registers, not one u32.
    pub fn outgoing_dir(&self) -> Vec2 {
        match self.0[0] {
            0 => self.vec2(4),
            1 => self.vec2(12),
            _ => panic!("AlignKind::None has no defined outgoing_dir"),
        }
    }
    fn vec2(&self, offset: usize) -> Vec2 {
        Vec2::new(read(&self.0, offset), read(&self.0, offset + 4))
    }
}

/// B PlayermadeStairsAssemblyParams::length_ws (0x140959ca0). Its actual path
/// points and per-point heights are zipped; differing lengths stop at the
/// shorter source, and each neighboring pair contributes one 3D segment.
pub fn length_ws(points: &[Vec2], heights: &[f32]) -> f32 {
    let mut path = points.iter().zip(heights);
    let Some((first, &height)) = path.next() else {
        return 0.0;
    };
    let mut previous = [first.x, height, first.y];
    let mut total = 0.0;
    for (point, &height) in path {
        let next = [point.x, height, point.y];
        let x = previous[0] - next[0];
        let y = previous[1] - next[1];
        let z = previous[2] - next[2];
        total += (z * z + x * x + y * y).sqrt();
        previous = next;
    }
    total
}
fn read(bytes: &[u8], offset: usize) -> f32 {
    f32::from_le_bytes(bytes[offset..offset + 4].try_into().unwrap())
}

/// One 0x150-byte element of `PlayermadeStairsAssemblyRequests` (native
/// `PlayermadeStairsAssemblyParams`, the type `length_ws` 0x140959ca0 belongs to).
///
/// Only the fields consumed by `structure_outline` (0x140959fc0, element walk
/// 0x14095a387 and `FUN_14095fca0`) are modelled; the remaining bytes of the
/// native record (`AlignKind`s, anchors, ...) belong to the assembly producer
/// (`playermade_stairs_assemble`) and are **not recovered here**:
///
/// | native offset | field |
/// |---|---|
/// | `+0x08/+0x10` | `positions` (`Vec<Vec2>`, xz of the sampled stair path) |
/// | `+0x40/+0x48` | `heights` (`Vec<f32>`, same sample count) |
/// | `+0x58/+0x60` | `widths` (`Vec<f32>`, outline adds 0.3 and uses half) |
/// | `+0x70/+0x78` | `tangents` (`Vec<Vec2>`, normalized and rotated to a left normal) |
/// | `+0x128/+0x12c` | `segment` (two u32 endpoints compared with `OutlineStairsCmd::segment`) |
///
/// Field names for the four vectors are inferred from their arithmetic use.
#[derive(Clone, Debug, PartialEq)]
pub struct PlayermadeStairsAssemblyParams {
    pub positions: Vec<Vec2>,
    pub heights: Vec<f32>,
    pub widths: Vec<f32>,
    pub tangents: Vec<Vec2>,
    pub segment: super::stairs_state::StairSegment,
}

/// B `country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests`
/// (a `Res<>` of `structure_outline`; element stride 0x150, vec ptr `+8`, len `+0x10`).
/// Its producer (`playermade_stairs_assemble`) is a separate, still-unrecovered owner.
#[derive(bevy_ecs::resource::Resource, Clone, Debug, Default, PartialEq)]
pub struct PlayermadeStairsAssemblyRequests {
    pub requests: Vec<PlayermadeStairsAssemblyParams>,
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 原两个候选容器都独立证明kind20字节、对齐4。
    #[test]
    fn align_kind_is_twenty_bytes_with_fresh_offsets() {
        assert_eq!(core::mem::size_of::<AlignKind>(), AlignKind::NATIVE_SIZE);
        assert_eq!(core::mem::size_of::<AlignKind>(), 20);
        assert_eq!(core::mem::align_of::<AlignKind>(), 4);
        assert_eq!(core::mem::offset_of!(AlignKind, tag), 0);
        assert_eq!(core::mem::offset_of!(AlignKind, flag), 1);
        assert_eq!(core::mem::offset_of!(AlignKind, byte_2), 2);
        assert_eq!(core::mem::offset_of!(AlignKind, first), 4);
        assert_eq!(core::mem::offset_of!(AlignKind, second), 0xc);
    }

    /// Upstream `PreferredNodeAlignment` push element (0x20 stride).
    #[test]
    fn preferred_node_alignment_is_thirty_two_bytes_with_fresh_offsets() {
        assert_eq!(
            core::mem::size_of::<PreferredNodeAlignment>(),
            PreferredNodeAlignment::NATIVE_SIZE
        );
        assert_eq!(core::mem::size_of::<PreferredNodeAlignment>(), 0x20);
        assert_eq!(core::mem::offset_of!(PreferredNodeAlignment, q0), 0);
        assert_eq!(core::mem::offset_of!(PreferredNodeAlignment, q1), 8);
        assert_eq!(core::mem::offset_of!(PreferredNodeAlignment, q2), 0x10);
        assert_eq!(core::mem::offset_of!(PreferredNodeAlignment, tag), 0x18);
        assert_eq!(core::mem::offset_of!(PreferredNodeAlignment, byte_1c), 0x1c);
        assert_eq!(core::mem::offset_of!(PreferredNodeAlignment, byte_1d), 0x1d);
    }

    /// 0x140959f00: variant 0 `(flag==0 || byte2==1)` scales `first`; variant 1
    /// adds `second` when `flag != 0` and halves the scale.
    #[test]
    fn align_kind_position_offset_matches_fresh_branches() {
        let base = AlignKind {
            tag: 0,
            flag: 0,
            byte_2: 0,
            pad_0: 0,
            first: Vec2::new(1.0, 2.0),
            second: Vec2::new(3.0, 4.0),
        };
        assert_eq!(base.position_offset(2.0), Vec2::new(2.0, 4.0));
        let suppressed = AlignKind {
            flag: 1,
            byte_2: 0,
            ..base
        };
        assert_eq!(suppressed.position_offset(2.0), Vec2::ZERO);
        let enabled = AlignKind {
            flag: 1,
            byte_2: 1,
            ..base
        };
        assert_eq!(enabled.position_offset(2.0), Vec2::new(2.0, 4.0));
        let variant_one = AlignKind {
            tag: 1,
            flag: 1,
            ..base
        };
        assert_eq!(variant_one.position_offset(2.0), Vec2::new(4.0, 6.0));
        let variant_one_no_flag = AlignKind {
            tag: 1,
            flag: 0,
            ..base
        };
        assert_eq!(variant_one_no_flag.position_offset(2.0), Vec2::new(1.0, 2.0));
    }

    /// 0x140959f50: variant 0 reads `+4/+8`, variant 1 reads `+0xc/+0x10`.
    #[test]
    fn align_kind_outgoing_dir_reads_the_confirmed_words() {
        let base = AlignKind {
            tag: 0,
            flag: 0,
            byte_2: 0,
            pad_0: 0,
            first: Vec2::new(5.0, 6.0),
            second: Vec2::new(7.0, 8.0),
        };
        assert_eq!(base.outgoing_dir(), Vec2::new(5.0, 6.0));
        let variant_one = AlignKind { tag: 1, ..base };
        assert_eq!(variant_one.outgoing_dir(), Vec2::new(7.0, 8.0));
    }

    #[test]
    #[should_panic(expected = "AlignKind::None has no defined outgoing_dir")]
    fn align_kind_outgoing_dir_panics_with_the_fresh_message() {
        let value = AlignKind {
            tag: 2,
            ..AlignKind::default()
        };
        let _ = value.outgoing_dir();
    }
}
