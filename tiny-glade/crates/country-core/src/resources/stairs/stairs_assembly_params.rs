//! 楼梯请求的权威值、历史表与几何基础；完整 producer 位于现有 decorator 模块。
use glam::Vec2;

use crate::resources::walls::WallId;
use super::stairs_state::StairSegment;

/// B 0x140959e80 原样传递墙身份与角度；首 qword 是 WallId，不是 Vec2。
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct StairWallAnchor {
    pub wall: WallId,
    pub angle: f32,
}
impl StairWallAnchor {
    pub fn new(wall: WallId, angle: f32) -> Self {
        Self { wall, angle }
    }
    /// B 0x140959e90 calls sinf then cosf and XORs both results with 0x80000000.
    /// The old chapter omitted those flips. These point into the wall.
    pub fn wall_normal(self) -> Vec2 {
        let x = f32::from_bits(self.angle.sin().to_bits() ^ 0x8000_0000);
        let y = f32::from_bits(self.angle.cos().to_bits() ^ 0x8000_0000);
        Vec2::new(x, y)
    }
}

/// 兼容既有原字节读取入口，只把已证 Wall payload 转为 canonical WallId 锚点。
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
            WallId(u64::from_le_bytes(self.0[8..16].try_into().unwrap())),
            read(self.0, 16),
        ))
    }
}

/// 原 derive 输出的三种节点锚点；非活动 payload 不制造默认业务值。
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum StairNodeAnchor {
    Terrain { height: f32 } = 0,
    Roof { height: f32 } = 1,
    Wall(StairWallAnchor) = 2,
}

impl StairNodeAnchor {
    /// B 0x140959ef0 只有 Wall 变体返回其 payload。
    pub fn try_as_wall(&self) -> Option<&StairWallAnchor> {
        match self {
            Self::Wall(anchor) => Some(anchor),
            Self::Terrain { .. } | Self::Roof { .. } => None,
        }
    }
}

/// B 0x140959e70：规范化 segment 的第一个节点在高 32 位。
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct StairGraphEdgeIdentity(u64);

impl From<StairSegment> for StairGraphEdgeIdentity {
    fn from(segment: StairSegment) -> Self {
        let (a, b) = segment.nodes();
        Self((u64::from(a.0) << 32) | u64::from(b.0))
    }
}

impl StairGraphEdgeIdentity {
    pub fn packed(self) -> u64 {
        self.0
    }
}

/// 原段记录 stride4；flag 是实际 producer 写入的业务位，不是 padding。
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StairSubSegment {
    pub kind: u8,
    pub flag: u8,
    pub count: u16,
}

pub type StairSubSegments = smallvec::SmallVec<[StairSubSegment; 1]>;

/// 原请求 owner +0x198 的旧段表；由 producer 统一维护，消费者只借用。
pub type StairSubSegmentMap = hashbrown::HashMap<
    StairGraphEdgeIdentity,
    StairSubSegments,
    std::hash::RandomState,
>;

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
    /// +0x2 原 tag0 在 `flag != 0 || byte2 == 1` 时产生位移。
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
                if self.flag != 0 || self.byte_2 == 1 {
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
            _ => {}
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

/// 原请求中实际发布的端点位置和出射方向；这是有向边的值。
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AdjustedStairEdge {
    pub position: Vec2,
    pub outgoing_dir: Vec2,
}

/// 原采样历史的位级元数据；f32 风格字段按原 u32 比较，保留 NaN payload。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StairSamplesMetadata {
    pub info_28: u32,
    pub extra_08: u32,
    pub extra_0c: u32,
    pub removed_support: bool,
    pub extra_10: u8,
}

/// 原历史 value 为 0x70；桶的 0x78 stride 还包含一个独立的八字节 key。
#[derive(Clone, Debug)]
pub struct SampledStairsPoints {
    pub points: Vec<glam::Vec3>,
    pub tangents: Vec<Vec2>,
    pub widths: Vec<f32>,
    /// 第四 Vec 目前只锁定四字节元素；原 interpolate 仅保留或释放，不猜其数值类型。
    pub auxiliary_words: Vec<u32>,
    pub metadata: StairSamplesMetadata,
}

/// 原完整装配器实际写入两种值；专名还未锁定，保留观测 variant。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StairArchWallSupportType {
    Kind0,
    Kind1,
}

/// 原 prev_requests 返回的真实子对象，统一拥有三个前帧表。
#[derive(Clone, Debug, Default)]
pub struct PreviousStairsAssemblyRequests {
    pub sampled_points: hashbrown::HashMap<
        StairGraphEdgeIdentity, SampledStairsPoints, std::hash::RandomState,
    >,
    pub arch_wall_supports: hashbrown::HashMap<
        StairGraphEdgeIdentity, StairArchWallSupportType, std::hash::RandomState,
    >,
    pub sub_segments: StairSubSegmentMap,
}

/// 原 0x150 请求的完整值；曲线是唯一采样路径，不维护 positions 镜像。
#[derive(Clone, Debug)]
pub struct PlayermadeStairsAssemblyParams {
    pub curve: utils::curve::Curve2,
    pub heights: Vec<f32>,
    pub widths: Vec<f32>,
    pub tangents: Vec<Vec2>,
    /// 原装配器以 MOVSS/UCOMISS 消费；具体业务专名仍待完整装配器复原。
    pub auxiliary_samples: Vec<f32>,
    pub interpolated_curve: Option<utils::curve::Curve3>,
    pub nodes: [(StairNodeAnchor, u8); 2],
    pub terrain_contacts: bitvec::vec::BitVec<usize, bitvec::order::Lsb0>,
    pub segment: StairSegment,
    pub sub_segments: StairSubSegments,
    pub dirty: bool,
}

impl PlayermadeStairsAssemblyParams {
    pub fn length_ws(&self) -> f32 {
        length_ws(&self.curve.points, &self.heights)
    }
}

/// 请求、双向业务图和历史表的唯一 owner；两个图各自保留有向边语义。
#[derive(bevy_ecs::resource::Resource, Clone, Debug, Default)]
pub struct PlayermadeStairsAssemblyRequests {
    pub requests: Vec<PlayermadeStairsAssemblyParams>,
    pub alignments: petgraph::graphmap::DiGraphMap<
        crate::resources::walls::decorator::DecoratorId, AlignKind, std::hash::RandomState,
    >,
    pub adjusted_edges: petgraph::graphmap::DiGraphMap<
        crate::resources::walls::decorator::DecoratorId, AdjustedStairEdge, std::hash::RandomState,
    >,
    pub previous: PreviousStairsAssemblyRequests,
}

impl PlayermadeStairsAssemblyRequests {
    /// 原 0x140274480 返回 +0x138 的历史子对象，不能返回 current Vec。
    pub fn prev_requests(&self) -> &PreviousStairsAssemblyRequests {
        &self.previous
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::resources::walls::decorator::DecoratorId;

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

    /// 原 tag0 的两个独立使能位，以及 tag1 的加向量与半宽顺序。
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
        assert_eq!(base.position_offset(2.0), Vec2::ZERO);
        let flag_enabled = AlignKind {
            flag: 1,
            byte_2: 0,
            ..base
        };
        assert_eq!(flag_enabled.position_offset(2.0), Vec2::new(2.0, 4.0));
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
    #[test]
    fn canonical_offset_and_observed_facade_agree_on_all_enable_branches() {
        for flag in [0, 1] {
            for byte_2 in [0, 1] {
                let value = AlignKind {
                    flag,
                    byte_2,
                    first: Vec2::new(2.0, -3.0),
                    ..AlignKind::default()
                };
                let mut bytes = [0; 20];
                bytes[1] = flag;
                bytes[2] = byte_2;
                bytes[4..8].copy_from_slice(&2.0f32.to_le_bytes());
                bytes[8..12].copy_from_slice(&(-3.0f32).to_le_bytes());
                assert_eq!(
                    value.position_offset(4.0),
                    ObservedAlignKind::from_observed(bytes).position_offset(4.0)
                );
            }
        }
        let none = AlignKind { tag: 2, ..AlignKind::default() };
        assert_eq!(none.position_offset(f32::NAN), Vec2::ZERO);
    }

    #[test]
    fn wall_anchor_preserves_full_wall_id_without_float_conversion() {
        let id = WallId(0xffee_ddcc_aabb_8899);
        let mut observed = [0; 24];
        observed[..4].copy_from_slice(&2u32.to_le_bytes());
        observed[8..16].copy_from_slice(&id.0.to_le_bytes());
        observed[16..20].copy_from_slice(&0.75f32.to_le_bytes());
        let anchor = ObservedStairNodeAnchor::from_observed(&observed)
            .try_as_wall().unwrap();
        assert_eq!(anchor, StairWallAnchor::new(id, 0.75));
        assert_eq!(StairNodeAnchor::Wall(anchor).try_as_wall(), Some(&anchor));
        assert!(StairNodeAnchor::Roof { height: 2.0 }.try_as_wall().is_none());
    }

    #[test]
    fn segment_identity_packs_sorted_ids_in_original_word_order() {
        let forward = StairSegment::new(DecoratorId(7), DecoratorId(29));
        let backward = StairSegment::new(DecoratorId(29), DecoratorId(7));
        let a = StairGraphEdgeIdentity::from(forward);
        assert_eq!(a, StairGraphEdgeIdentity::from(backward));
        assert_eq!(a.packed(), (7u64 << 32) | 29);
        assert_eq!(core::mem::size_of::<StairSubSegment>(), 4);
        assert_eq!(core::mem::offset_of!(StairSubSegment, flag), 1);
        assert_eq!(core::mem::offset_of!(StairSubSegment, count), 2);
    }

}
