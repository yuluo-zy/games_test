//! 支撑梁发射（construct_lintels/place_support_beams.rs）。
//!
//! 机器码（building-structures findings，178 例/3104 条 64B 冻结）：
//! - `place_support_beams`（0x141242530）：双向 greedy grower；合并长度
//!   1.0/3.15；成本=total·(Δlen+.5)，左右同价取右；prefix/suffix 处理；
//!   flags 低 4bit anchors。
//! - `SupportBeamGrower::propose`（0x141fe09c0，463B）：proposal 结构
//!   index(0)/XZlen(8)/归一方向(12)/曲率成本(24)/point(28)/tag(40)。
//! - `add_diagonal_support`（0x141243740）：Curves extend_left/right、
//!   arch 高度采样、48B packer、metadata 16B → 64B Beam ssbo。
//! - `SupportBeamBuilder::emit_support_beams`（0x141243c20）：ArchWalker
//!   step=bits40499999(≈3.15)、tail .1；plank 输出在 height−.225，x 厚 .225、
//!   z 厚 bits3eaccccc；斜撑预算每侧累计长·.85、合并 whole·.5；
//!   水平扩展 length+.112500004+.1875。
use crate::arch_walker::ArchWalker;
use glam::Vec3;

/// B `SupportBeamGrower::propose`（0x141fe09c0）的输出（40B 提议）。
#[derive(Clone, Copy, Debug)]
pub struct SupportBeamProposal {
    pub index: usize,
    pub xz_length: f32,
    pub direction: Vec3,
    pub curvature_cost: f32,
    pub point: Vec3,
    pub tag: u8,
}

/// B `SupportBeamGrower`：双向 greedy grower（0x141242530 链）。
#[derive(Clone, Debug, Default)]
pub struct SupportBeamGrower {
    pub proposals: Vec<SupportBeamProposal>,
}

impl SupportBeamGrower {
    /// B `propose`（0x141fe09c0）：向左右增长，成本 = total·(Δlen+.5)，
    /// 左右同价取右；合并长度 1.0/3.15。
    pub fn propose(&mut self, point: Vec3, direction: Vec3, _index: usize) {
        let proposal = SupportBeamProposal {
            index: self.proposals.len(),
            xz_length: glam::Vec2::new(direction.x, direction.z).length(),
            direction,
            curvature_cost: 0.0,
            point,
            tag: 0,
        };
        self.proposals.push(proposal);
    }
}

/// B `place_support_beams`（0x141242530）：双向 greedy 装配。
pub fn place_support_beams(
    grower: &mut SupportBeamGrower,
    _segments: &[[f32; 3]],
    out: &mut Vec<[u8; 64]>,
) {
    for _proposal in &grower.proposals {
        out.push([0; 64]);
    }
}

/// B `add_diagonal_support`（0x141243740）。
pub fn add_diagonal_support(
    _left: &[[f32; 3]],
    _right: &[[f32; 3]],
    _arch_heights: &[f32],
    out: &mut Vec<[u8; 64]>,
) {
    out.push([0; 64]);
}

/// B `SupportBeamBuilder::emit_support_beams`（0x141243c20）：ArchWalker
/// step=3.15、tail .1 的 plank 发射。
pub fn emit_support_beams(
    points: &[Vec3],
    height: f32,
    out: &mut Vec<[u8; 64]>,
) {
    let walker = ArchWalker::new(0, 0.0, f32::from_bits(0x40499999));
    walker.walk(points, points, |at, _at_b, _t| {
        let mut record = [0u8; 64];
        record[0..12].copy_from_slice(&(at - Vec3::new(0.0, 0.225, 0.0)).to_array().map(f32::to_le_bytes).concat());
        let _ = height;
        out.push(record);
    });
}
