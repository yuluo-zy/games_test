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
    /// B `propose`（0x141fe09c0）。**未实现（近似壳）**：真实逻辑为
    /// flags[next]&0xf≠0→stopped、方向=点差归一化、已有方向投影、
    /// cost=len*(‖Δdir‖+0.5)（len≥1）。现有实现仅装配 proposal 结构且
    /// curvature_cost=0，不是原二进制真实逻辑。
    pub fn propose(&mut self, _point: Vec3, _direction: Vec3, _index: usize) {
        todo!("未实现: SupportBeamGrower::propose 0x141fe09c0")
    }
}

/// B `place_support_beams`（0x141242530）。**未实现（零壳）**：真实双 Grower
/// 0/end 向中 propose、cost 小者 consume、len≥1.0 发射/≤3.15/join、分开各
/// len*0.85 对角、join 且 total≥1.4175 双侧 0.5。
pub fn place_support_beams(
    _grower: &mut SupportBeamGrower,
    _segments: &[[f32; 3]],
    _out: &mut Vec<[u8; 64]>,
) {
    todo!("未实现: place_support_beams 0x141242530")
}

/// B `add_diagonal_support`（0x141243740）。**未实现（零壳）**：真实 extend_xz
/// 延展到 (index,fraction)、锚高 sample_height−0.2、方向=采样点差(Δy−0.2)、
/// 轴 side×0.2/dir×(len−0.2)/third×0.2。
pub fn add_diagonal_support(
    _left: &[[f32; 3]],
    _right: &[[f32; 3]],
    _arch_heights: &[f32],
    _out: &mut Vec<[u8; 64]>,
) {
    todo!("未实现: add_diagonal_support 0x141243740")
}

/// B `SupportBeamBuilder::emit_support_beams`（0x141243c20）。**未实现（近似壳）**：
/// 真实 rdp_support_indices→索引对→walk_lintel。
pub fn emit_support_beams(
    _points: &[Vec3],
    _height: f32,
    _out: &mut Vec<[u8; 64]>,
) {
    todo!("未实现: emit_support_beams 0x141243c20")
}
