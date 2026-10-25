//! stairs_assembly_params.rs — 楼梯装配参数（WP21）。
//!
//! O：`PlayermadeStairsAssemblyParams::length_ws` 0x959ca0（points Vec<Vec2> @+0x8/
//! +0x10、heights Vec<f32> @+0x40/+0x48，zip xvy 收集 Vec3 求墙空间长）、
//! `StairWallAnchor::wall_normal` 0x959e90（(sin,cos)）、`AlignKind::outgoing_dir`
//! 0x959f50（判别 0/1 否则 panic）、`StairNodeAnchor::try_as_wall` 0x959ef0。

use glam::{Vec2, Vec3};

use crate::typed_gap::Gap;

/// `AlignKind`（O：对齐种类）。
///
/// Fresh B evidence（2026-10-17）：
/// - `grow_one<indexmap::Bucket<tuple<DecoratorId,DecoratorId>,AlignKind>>`
///   @0x140562ab0 以 stride **0x28=40B** 倍增；indexmap 2.7.1
///   `Bucket{hash:usize(8), key:(u32,u32)(8), value}` + 对齐 ⇒
///   `size_of::<AlignKind>() == 24`。
/// - `AlignKind::outgoing_dir` @0x140959f50（原字节）：
///   `movzx eax,[rcx]`；`tag==0 → (f32@+4,f32@+8)`，`tag==1 →
///   (f32@+0xc,f32@+0x10)`，其余 panic。
/// - `AlignKind::position_offset` @0x140959f00（原字节）：
///   `tag==0` 时 `!flag_a && flag_b` 才取 `Vec2@+4`，否则零；
///   `tag==1` 时 `Vec2@+4 (+ Vec2@+0xc if flag_a)` 按入参比例缩放。
///
/// 布局逐字节（O）：`tag u8@0, flag_a bool@1, flag_b bool@2, pad@3,
/// dir_a Vec2@4, dir_b Vec2@12, weight f32@20`，总 24B。
/// PDB 拼写为 `enum2$<...AlignKind>`，但变体字段名 U；这里按已证字节布局
/// 落为 `repr(C)` 载体，字段名只标注证据角色。
#[repr(C, align(4))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct AlignKind {
    /// 判别 @+0（0/1；outgoing_dir 其它值 panic）。
    pub tag: u8,
    /// @+1（position_offset 选择是否叠加 dir_b）。
    pub flag_a: bool,
    /// @+2（position_offset tag0 分支的附加门控）。
    pub flag_b: bool,
    /// @+4 方向 A（outgoing_dir tag0）。
    pub dir_a: Vec2,
    /// @+12 方向 B（outgoing_dir tag1）。
    pub dir_b: Vec2,
    /// @+20 权重（方法未读；布局证据为 Bucket 步长）。
    pub weight: f32,
}

impl AlignKind {
    /// `position_offset`（O 0x140959f00）：`scale` 由调用方经 xmm1 传入。
    pub fn position_offset(&self, scale: f32) -> Vec2 {
        match self.tag {
            0 => {
                if !self.flag_a && self.flag_b {
                    self.dir_a
                } else {
                    Vec2::ZERO
                }
            }
            1 => {
                let mut v = self.dir_a;
                if self.flag_a {
                    v += self.dir_b;
                }
                v * scale
            }
            _ => Vec2::ZERO,
        }
    }
    /// `outgoing_dir`（O 0x140959f50：判别 0/1 否则 panic）。
    pub fn outgoing_dir(&self) -> Vec2 {
        match self.tag {
            0 => self.dir_a,
            1 => self.dir_b,
            _ => panic!("invalid AlignKind discriminant"),
        }
    }
}

/// `system_decorator::decorator_visual::stairs::PreferredNodeAlignment`。
///
/// PDB 只有名称/泛型关系（`Vec<PreferredNodeAlignment>`、
/// `tuple<DecoratorId, Vec<PreferredNodeAlignment>>`），字段/大小 unknown。
/// Fresh B evidence（`find_alignment_preferences` @0x14080beb0）：
/// 3 个元素连续写入 72B；`find_best_neighbor_alignments` @0x14080c360 以
/// `plVar8 + 3`（`longlong*`，即 0x18=24B）步进读取 ⇒ **元素 24B**。
/// 读取形态：`tag u8@0`（0/1，其余 panic）；`tag==1` 读
/// `Vec2@4`、`Vec2@12`、`f32@20`；`tag==0` 只写 `Vec2@4`、`f32@20`；
/// `+2` 为 `bool`（两变体都写 `bVar2^1`）。
///
/// 该类型属 system-decorator 域；恢复工作区没有 decorator crate，按
/// stairs 资源相邻域落到本文件（`AlignKind` 同域消费者），不新建孤立 crate。
#[repr(C, align(4))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PreferredNodeAlignment {
    /// 判别 @+0（0/1）。
    pub tag: u8,
    /// @+1：证据未写入（恒 0）。
    pub pad1: u8,
    /// @+2：两变体都写入 `bool`。
    pub flag: bool,
    /// @+4。
    pub dir_a: Vec2,
    /// @+12（仅 tag==1 写入）。
    pub dir_b: Vec2,
    /// @+20。
    pub weight: f32,
}

impl PreferredNodeAlignment {
    /// 与 `AlignKind` 同形：tag0→+4，tag1→+12。
    pub fn outgoing_dir(&self) -> Vec2 {
        match self.tag {
            0 => self.dir_a,
            1 => self.dir_b,
            _ => panic!("invalid PreferredNodeAlignment discriminant"),
        }
    }
}

#[cfg(test)]
mod layout_tests {
    use super::*;
    use core::mem::{offset_of, size_of};

    #[test]
    fn align_kind_layout_is_24b() {
        assert_eq!(size_of::<AlignKind>(), 24);
        assert_eq!(offset_of!(AlignKind, tag), 0);
        assert_eq!(offset_of!(AlignKind, flag_a), 1);
        assert_eq!(offset_of!(AlignKind, flag_b), 2);
        assert_eq!(offset_of!(AlignKind, dir_a), 4);
        assert_eq!(offset_of!(AlignKind, dir_b), 12);
        assert_eq!(offset_of!(AlignKind, weight), 20);
    }

    #[test]
    fn preferred_node_alignment_layout_is_24b() {
        assert_eq!(size_of::<PreferredNodeAlignment>(), 24);
        assert_eq!(offset_of!(PreferredNodeAlignment, tag), 0);
        assert_eq!(offset_of!(PreferredNodeAlignment, flag), 2);
        assert_eq!(offset_of!(PreferredNodeAlignment, dir_a), 4);
        assert_eq!(offset_of!(PreferredNodeAlignment, dir_b), 12);
        assert_eq!(offset_of!(PreferredNodeAlignment, weight), 20);
    }

    #[test]
    fn outgoing_dir_matches_evidence() {
        let a = AlignKind { tag: 0, dir_a: Vec2::new(1.0, 2.0), ..Default::default() };
        assert_eq!(a.outgoing_dir(), Vec2::new(1.0, 2.0));
        let b = AlignKind { tag: 1, dir_b: Vec2::new(3.0, 4.0), ..Default::default() };
        assert_eq!(b.outgoing_dir(), Vec2::new(3.0, 4.0));
    }
}

/// `StairWallAnchor`（O：墙锚点；angle @+0x8）。
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct StairWallAnchor {
    pub anchor: Vec2,
    /// 墙角（弧度，O：`wall_normal` 读 +8）。
    pub angle: f32,
}

impl StairWallAnchor {
    /// `new`（O 0x959e80）。
    pub fn new(anchor: Vec2, angle: f32) -> Self {
        Self { anchor, angle }
    }
    /// `wall_normal`（O 0x959e90）：`(sin(angle), cos(angle))`。
    pub fn wall_normal(&self) -> Vec2 {
        Vec2::new(self.angle.sin(), self.angle.cos())
    }
}

/// `StairNodeAnchor`（O：节点锚点）。
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum StairNodeAnchor {
    Wall(StairWallAnchor),
    Free,
}

impl StairNodeAnchor {
    /// `try_as_wall`（O 0x959ef0）。
    pub fn try_as_wall(&self) -> Option<&StairWallAnchor> {
        match self {
            StairNodeAnchor::Wall(w) => Some(w),
            StairNodeAnchor::Free => None,
        }
    }
}

/// `PlayermadeStairsAssemblyParams`（O：一段手工楼梯装配参数）。
#[derive(Clone, Debug, Default)]
pub struct PlayermadeStairsAssemblyParams {
    /// 墙空间点列（O：+0x8 ptr / +0x10 len，8B 步长 Vec2）。
    pub points: Vec<Vec2>,
    /// 高列表（O：+0x40/+0x48，4B 步长 f32）。
    pub heights: Vec<f32>,
    /// 未映射区（U）。
    pub _gap: Gap,
}

impl PlayermadeStairsAssemblyParams {
    /// `length_ws`（O 0x959ca0）：点/高 zip → Vec3 链求墙空间总长。
    pub fn length_ws(&self) -> f32 {
        let n = self.points.len().min(self.heights.len());
        let mut len = 0.0;
        let mut prev: Option<Vec3> = None;
        for i in 0..n {
            let p = Vec3::new(self.points[i].x, self.heights[i], self.points[i].y);
            if let Some(prev) = prev {
                len += (p - prev).length();
            }
            prev = Some(p);
        }
        len
    }
}

/// `PlayermadeStairsAssemblyRequests`（O：装配请求列表）。
#[derive(Clone, Debug, Default)]
pub struct PlayermadeStairsAssemblyRequests {
    pub params: Vec<PlayermadeStairsAssemblyParams>,
}

impl PlayermadeStairsAssemblyRequests {
    /// `prev_requests`（O 0x274480）。
    pub fn prev_requests(&self) -> &[PlayermadeStairsAssemblyParams] {
        &self.params
    }
}

/// `StairGraphEdgeIdentity`（O：32B 边标识；装配/支撑 map 键）。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct StairGraphEdgeIdentity(pub [u8; 32]);

/// `StairSubSegment`（O：smallvec<[StairSubSegment;1]> 元素；布局 U）。
#[derive(Clone, Debug, Default, PartialEq)]
pub struct StairSubSegment(pub Gap);

/// `StairArchWallSupportType`（O：拱墙支撑类型判别；map 值）。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct StairArchWallSupportType(pub u8);

/// `AdjustedStairEdge`（O：装配后边；IndexMap 值，U）。
#[derive(Clone, Debug, Default)]
pub struct AdjustedStairEdge(pub Gap);

/// `SampledStairsPoints`（O：采样楼梯点；U）。
#[derive(Clone, Debug, Default)]
pub struct SampledStairsPoints(pub Gap);
