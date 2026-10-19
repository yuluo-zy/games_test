//! 支撑梁发射（construct_lintels/place_support_beams.rs）。
//!
//! 原 B 源：`crates/systems/wall-constructor/src/construct_lintels/place_support_beams.rs`。
//! 机器码（building-structures findings，178 例/3104 条 64B 冻结；本文件与
//! country-core `systems/wall/construction/structures.rs` 的同一原函数
//! 0x141242530/0x141243740/0x141243c20 逐语句对齐）：
//!
//! - `place_support_beams`（0x141242530）：双向 greedy grower。0/end 两端向中
//!   propose，cost 小者 consume；len≥1.0 才发射、单侧总长 ≤3.1499999、
//!   join 判据 `total ∈ [1.0, 3.15]` 且合并后仍可 propose（长度上限）；
//!   未 join 时两段各 `len*0.85` 对角；join 且 `total ≥ bits3fb570a3`（1.4175）
//!   时双侧各 `total*0.5` 对角。
//! - `SupportBeamGrower::propose`（0x141fe09c0，463B）：flags[next]&0xf≠0 →
//!   stopped；方向 = 点差归一化；已有方向按 **grower 状态 flags@+0** 的
//!   bit0 选择投影（不是输入点 flags）；cost = `new_len*(‖Δdir‖+0.5)`
//!   （仅 len≥1）。提案布局：index@0 / xz_len@8 / dir@12 / curvature@24 /
//!   point@28 / tag@40；无提案时 tag=2 为无效哨兵。
//! - `add_diagonal_support`（0x141243740）：`extend_xz` 延展到 (index,fraction)，
//!   锚高 `sample_height−0.225`，方向 = 采样点差，轴侧向 0.2625。
//! - `emit_support_beams`（0x141243c20）：rdp_support_indices → 索引对 →
//!   ArchWalker 步长 bits40499999(≈3.15)、tail .1；plank 在 height−0.225，
//!   x 厚 .225、z 厚 bits3eaccccc；水平扩展 length+.1125+.1875。
use crate::arch_walker::ArchWalker;
use country_core::systems::wall::construction::structures::rdp_support_indices;
use glam::{Vec3, Vec4};

/// B `SupportBeamGrower::propose`（0x141fe09c0）的输出（40B 提议）。
/// 布局与原件一致：index 0..8、xz_length 8..12、direction 12..24、
/// curvature_cost 24..28、point 28..40、tag 40。
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SupportBeamProposal {
    pub index: usize,
    pub xz_length: f32,
    pub direction: Vec3,
    pub curvature_cost: f32,
    pub point: Vec3,
    /// 0/1；`flags[index]&0xf != 0` 时为 1。
    pub tag: u8,
}

impl SupportBeamProposal {
    /// 无效提案哨兵（原件 `uVar7` 初值 2，写入 out+0x28）。
    pub const INVALID_TAG: u8 = 2;

    /// 原件布局投影（40B + tag@40 = 41B；诊断/测试用）。
    pub fn observed_bytes(&self) -> [u8; 41] {
        let mut b = [0u8; 41];
        b[0..8].copy_from_slice(&(self.index as u64).to_le_bytes());
        b[8..12].copy_from_slice(&self.xz_length.to_le_bytes());
        b[12..24].copy_from_slice(&self.direction.to_array().map(f32::to_le_bytes).concat());
        b[24..28].copy_from_slice(&self.curvature_cost.to_le_bytes());
        b[28..40].copy_from_slice(&self.point.to_array().map(f32::to_le_bytes).concat());
        b[40] = self.tag;
        b
    }
}

/// B `SupportBeamGrower`（0x141242530 链）。原件为栈上 0x2cB 状态：
/// flags@0 / dir@4 / 末方向投影@0x0c / point@0x18 / length@0x24 / stopped@0x28。
#[derive(Clone, Copy, Debug)]
pub struct SupportBeamGrower {
    pub index: usize,
    pub point: Vec3,
    /// 已累计方向（None 等价原件 flags bit0==0）。
    pub direction: Option<Vec3>,
    pub length: f32,
    pub stopped: bool,
    /// 原件 +0x00 u32 flags；bit0 决定 cost 是否使用已存方向。
    pub flags: u32,
}

impl SupportBeamGrower {
    pub fn new(index: usize, point: Vec3) -> Self {
        Self {
            index,
            point,
            direction: None,
            length: 0.0,
            stopped: false,
            flags: 0,
        }
    }

    /// 带 grower 状态 flags 的构造（原件 +0x00）。
    pub fn with_flags(index: usize, point: Vec3, flags: u32) -> Self {
        Self {
            flags,
            ..Self::new(index, point)
        }
    }

    /// 原 0x141fe09c0 的 cost 计算用方向：flags bit0 置位时取已存方向，
    /// 否则取新方向（此时 Δdir 恒为 0，cost = new_len*0.5）。
    pub fn propose(
        &self,
        index: usize,
        points: &[[f32; 3]],
        flags: &[u8],
    ) -> Option<SupportBeamProposal> {
        if self.stopped {
            return None;
        }
        let point = *points.get(index)?;
        let p = Vec3::from_array(point);
        let delta = p - self.point;
        let xz_length = (delta.x * delta.x + delta.z * delta.z).sqrt();
        let direction = delta * (1.0 / length_yxz(delta));
        let new_length = xz_length + self.length;
        // 单侧总长上限 bits40499999（3.1499999）。
        if new_length > f32::from_bits(0x40499999) {
            return None;
        }
        let curvature_cost = if self.length >= 1.0 {
            // grower flags bit0：0 → 忽略旧方向（Δ=0）；1 → 旧方向参与 Δ。
            let previous = if self.flags & 1 != 0 {
                self.direction.unwrap_or(direction)
            } else {
                direction
            };
            new_length * (length_yxz(previous - direction) + 0.5)
        } else {
            0.0
        };
        let tag = if flags[index] & 0x0f != 0 { 1 } else { 0 };
        Some(SupportBeamProposal {
            index,
            xz_length,
            direction,
            curvature_cost,
            point: p,
            tag,
        })
    }

    /// 原件 0x141fe09c0 的 sret 语义：先把输出 tag 置 2（无效哨兵），
    /// 仅在 propose 有效时覆盖。`out` 对应原件 in_RCX。
    pub fn propose_into(
        &self,
        index: usize,
        points: &[[f32; 3]],
        flags: &[u8],
        out: &mut SupportBeamProposal,
    ) {
        out.tag = SupportBeamProposal::INVALID_TAG;
        if let Some(proposal) = self.propose(index, points, flags) {
            *out = proposal;
        }
    }

    /// 原件 `SupportBeamGrower::accept/consume`：采用提案并累计长度。
    pub fn consume(&mut self, proposal: SupportBeamProposal) {
        self.index = proposal.index;
        self.point = proposal.point;
        self.direction = Some(proposal.direction);
        self.length += proposal.xz_length;
        self.stopped = proposal.tag != 0;
    }
}

/// `LengthedCurveCoord` 投影（point_index + fraction），对应原件
/// `ThresholdedCurveCoord` 在支撑梁链中的使用。
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SupportBeamCoord {
    pub point_index: usize,
    pub fraction: f32,
}

/// `place_support_beams` 的实际输入：materialized segment 的
/// heights / ground / raised 三数组 + flags + 端点坐标。
#[derive(Clone, Debug)]
pub struct SupportBeamSource {
    pub heights: Vec<f32>,
    pub ground_positions: Vec<[f32; 3]>,
    pub raised_positions: Vec<[f32; 3]>,
    pub flags: Vec<u8>,
    pub source_coords: Vec<SupportBeamCoord>,
}

fn length_yxz(v: Vec3) -> f32 {
    ((v.y * v.y + v.x * v.x) + v.z * v.z).sqrt()
}

fn affine_instance(
    x: Vec4,
    y: Vec4,
    z: Vec4,
    t: Vec4,
    wall_id: u64,
    ordinal: u32,
    source_index: u32,
) -> [u8; 64] {
    // 64B WoodenPlankSsbo：12×f32 affine (0..48)、segment ordinal@48、
    // source index@52、wall id@56。
    let mut b = [0u8; 64];
    for (i, v) in [x.x, x.y, x.z, y.x, y.y, y.z, z.x, z.y, z.z, t.x, t.y, t.z]
        .into_iter()
        .enumerate()
    {
        b[i * 4..i * 4 + 4].copy_from_slice(&v.to_le_bytes());
    }
    b[48..52].copy_from_slice(&ordinal.to_le_bytes());
    b[52..56].copy_from_slice(&source_index.to_le_bytes());
    b[56..60].copy_from_slice(&(wall_id as u32).to_le_bytes());
    b
}

/// 原 `emit_plank`（walk_lintel 的每砖发射；height−0.225，x 厚 .225、
/// z 厚 bits3eaccccc）。
fn emit_plank(
    start: Vec3,
    end: Vec3,
    wall_id: u64,
    ordinal: u32,
    base_index: usize,
    out: &mut Vec<[u8; 64]>,
) {
    let a = Vec3::new(start.x, start.y + f32::from_bits(0xbe666667), start.z);
    let b = Vec3::new(end.x, end.y + f32::from_bits(0xbe666667), end.z);
    let delta = b - a;
    let length = length_yxz(delta);
    let d = delta * (1.0 / length);
    let side = Vec3::Y.cross(d);
    let dir = d.extend(d.z);
    let side = side.extend(side.z);
    let zero = Vec4::ZERO;
    let tail = Vec4::new(0.0, 0.0, 0.0, f32::from_bits(0x3e666667));
    let x = side * tail + dir * tail + Vec4::new(0.0, f32::from_bits(0x3e666667), 0.0, 0.0);
    let y = dir * Vec4::new(length, length, length, 0.0) + zero + side * zero;
    let z = dir * zero
        + zero
        + side
            * Vec4::new(
                f32::from_bits(0x3eaccccc),
                f32::from_bits(0x3eaccccc),
                f32::from_bits(0x3eaccccc),
                0.0,
            );
    let t = dir * zero + zero + side * zero + a.extend(a.z);
    let index = ((start.x * 1000.0) as i32)
        .wrapping_add((start.z * 1000.0) as i32)
        .wrapping_add(base_index as i32) as u32;
    out.push(affine_instance(x, y, z, t, wall_id, ordinal, index));
}

/// 原 `walk_lintel`（0x141243c20 内联链）：沿 (ground+height) 三维折线按
/// `step=3.15` 步进发射木板；尾砖 `step*0.1 < residual`。
fn walk_lintel(
    heights: &[f32],
    ground: &[[f32; 3]],
    wall_id: u64,
    ordinal: u32,
    base_index: usize,
    out: &mut Vec<[u8; 64]>,
) {
    let points: Vec<Vec3> = ground
        .iter()
        .zip(heights)
        .map(|(p, h)| Vec3::new(p[0], p[1] + *h, p[2]))
        .collect();
    let mut total = 0.0;
    for pair in points.windows(2) {
        total += length_yxz(pair[1] - pair[0]);
    }
    let count = (total / f32::from_bits(0x40499999) + 0.5).floor().max(1.0) as usize;
    let step = total / count as f32;
    let mut previous = points[0];
    let mut residual = 0.0f32;
    for pair in points.windows(2) {
        let length = length_yxz(pair[1] - pair[0]);
        let mut remaining = residual + length;
        if step < remaining {
            let mut traversed = 0.0f32;
            loop {
                let distance = (step - residual).max(0.0);
                traversed += distance;
                let fraction = traversed / length;
                let point = pair[1] * fraction + pair[0] * (1.0 - fraction);
                emit_plank(previous, point, wall_id, ordinal, base_index, out);
                residual = (residual - step).max(0.0);
                remaining = (length - traversed) + residual;
                previous = point;
                if !(step < remaining) {
                    break;
                }
            }
        }
        residual = remaining;
    }
    if step * 0.1 < residual {
        emit_plank(
            previous,
            *points.last().expect("non-empty lintel walk"),
            wall_id,
            ordinal,
            base_index,
            out,
        );
    }
}

/// 原 `extend_xz`：把 (index,fraction) 沿折线延展 `distance`。
fn extend_xz(
    points: &[[f32; 3]],
    mut index: usize,
    mut fraction: f32,
    mut distance: f32,
    right: bool,
) -> (usize, f32) {
    loop {
        let a = points[index];
        let b = points[index + 1];
        let dx = b[0] - a[0];
        let dz = b[2] - a[2];
        let length = (dx * dx + dz * dz).sqrt();
        if right {
            let remainder = (1.0 - fraction) * length - distance;
            if remainder >= 0.0 {
                return (index, 1.0 - remainder / length);
            }
            if index + 2 == points.len() {
                return (index, 1.0);
            }
            distance = -remainder;
            index += 1;
            fraction = 0.0;
        } else {
            let remainder = ((fraction + (-1.0)) + 1.0) * length - distance;
            if remainder >= 0.0 {
                return (index, remainder / length);
            }
            if index == 0 {
                return (0, 0.0);
            }
            distance = -remainder;
            index -= 1;
            fraction = 1.0;
        }
    }
}

/// B `add_diagonal_support`（0x141243740）：从锚点向一侧延展发射斜撑。
/// 返回实际锚点索引（原 closure 以 anchor 作为 source index）。
fn add_diagonal_support(
    source: &SupportBeamSource,
    anchor: usize,
    right: bool,
    budget: f32,
    wall_id: u64,
    ordinal: u32,
    out: &mut Vec<[u8; 64]>,
) {
    let coord = source.source_coords[anchor];
    let index = coord.point_index;
    let fraction = coord.fraction;
    let sample = |i: usize, t: f32| {
        let a = Vec3::from_array(source.ground_positions[i]);
        let b = Vec3::from_array(source.ground_positions[i + 1]);
        (b - a) * t + a
    };
    let sample_height =
        |i: usize, t: f32| (source.heights[i + 1] - source.heights[i]) * t + source.heights[i];
    let height = sample_height(index, fraction) + f32::from_bits(0xbe666667);
    let length = (budget + f32::from_bits(0xbe866666)).min(height * 0.4);
    let extension = (length + f32::from_bits(0x3de66667)) + f32::from_bits(0x3e400000);
    let (target_index, target_fraction) =
        extend_xz(&source.ground_positions, index, fraction, extension, right);
    let a = sample(index, fraction) + Vec3::Y * ((height - length) + f32::from_bits(0xbe400000));
    let b = sample(target_index, target_fraction)
        + Vec3::Y * (sample_height(target_index, target_fraction) + f32::from_bits(0xbde66667));
    let delta = b - a;
    let full_length = length_yxz(delta);
    let d = delta * (1.0 / full_length);
    let c = d.cross(Vec3::Y);
    let side = c / length_yxz(c);
    let third = side.cross(d);
    let side = side.extend(side.z);
    let dir = d.extend(d.z);
    let third = third.extend(third.z);
    let scale = f32::from_bits(0x3e866666);
    let tail = Vec4::new(0.0, 0.0, 0.0, scale);
    let zero = Vec4::ZERO;
    let x = side * Vec4::splat(scale) + dir * tail + third * tail;
    let y =
        dir * Vec4::new(
            full_length + (-0.2),
            full_length + (-0.2),
            full_length + (-0.2),
            0.0,
        ) + side * zero
            + third * zero;
    let z = dir * zero + side * zero + third * Vec4::new(scale, scale, scale, 0.0);
    let t = dir * zero + side * zero + third * zero + a.extend(a.z);
    out.push(affine_instance(x, y, z, t, wall_id, ordinal, anchor as u32));
}

/// 原 `emit_range`：对 [begin,end] 做 RDP 索引切分后逐段 walk_lintel。
fn emit_range(
    source: &SupportBeamSource,
    begin: usize,
    end: usize,
    wall_id: u64,
    ordinal: u32,
    out: &mut Vec<[u8; 64]>,
) {
    if begin == end {
        return;
    }
    let indices = rdp_support_indices(&source.raised_positions[begin..=end]);
    for pair in indices.windows(2) {
        let a = begin + pair[0];
        let b = begin + pair[1];
        walk_lintel(
            &source.heights[a..=b],
            &source.ground_positions[a..=b],
            wall_id,
            ordinal,
            begin,
            out,
        );
    }
}

/// B `place_support_beams`（0x141242530）：双向 greedy grower 全序。
///
/// 与 skeleton 的占位签名 `(&mut Grower, &[[f32;3]], &mut Vec<[u8;64]>)` 不同：
/// 原件需要 flags/heights/端点坐标才能产生真实 64B 记录，这里按实际算法面
/// 暴露 `SupportBeamSource`。
pub fn place_support_beams(
    source: &SupportBeamSource,
    wall_id: u64,
    ordinal: u32,
    out: &mut Vec<[u8; 64]>,
) {
    let end = match source.heights.len() {
        0 => return,
        n => n - 1,
    };
    if end == 0
        || source.ground_positions.len() < source.heights.len()
        || source.raised_positions.len() < source.heights.len()
        || source.flags.len() < source.heights.len()
    {
        return;
    }
    let mut left = SupportBeamGrower::new(0, Vec3::from_array(source.raised_positions[0]));
    let mut right = SupportBeamGrower::new(end, Vec3::from_array(source.raised_positions[end]));
    let mut next_left = 1usize;
    let mut next_right = end - 1;
    let mut lp = left.propose(next_left, &source.raised_positions, &source.flags);
    let mut rp = right.propose(next_right, &source.raised_positions, &source.flags);
    while left.index < right.index {
        match (lp, rp) {
            (None, None) => break,
            (Some(p), r) if r.is_none() || r.unwrap().curvature_cost > p.curvature_cost => {
                left.consume(p);
                next_left += 1;
                lp = if next_left <= end {
                    left.propose(next_left, &source.raised_positions, &source.flags)
                } else {
                    None
                };
            }
            (_, Some(p)) => {
                right.consume(p);
                rp = if next_right > 0 {
                    next_right -= 1;
                    right.propose(next_right, &source.raised_positions, &source.flags)
                } else {
                    None
                };
            }
            _ => unreachable!(),
        }
    }
    let total = left.length + right.length;
    let join = left.index == right.index
        && total >= 1.0
        && total <= f32::from_bits(0x40499999)
        && ((left.index != 0
            && left
                .propose(0, &source.raised_positions, &source.flags)
                .is_some())
            || (right.index != end
                && right
                    .propose(left.index, &source.raised_positions, &source.flags)
                    .is_some()));
    let (emit_left, emit_right) = if join {
        emit_range(source, 0, end, wall_id, ordinal, out);
        (true, true)
    } else {
        let a = left.length >= 1.0;
        let b = right.length >= 1.0;
        if a {
            emit_range(source, 0, left.index, wall_id, ordinal, out);
        }
        if b {
            emit_range(source, right.index, end, wall_id, ordinal, out);
        }
        (a, b)
    };
    if left.index != right.index {
        if emit_left {
            add_diagonal_support(source, 0, true, left.length * 0.85, wall_id, ordinal, out);
        }
        if emit_right {
            add_diagonal_support(
                source,
                end,
                false,
                right.length * 0.85,
                wall_id,
                ordinal,
                out,
            );
        }
    } else if (emit_left || emit_right) && total >= f32::from_bits(0x3fb570a3) {
        add_diagonal_support(source, 0, true, total * 0.5, wall_id, ordinal, out);
        add_diagonal_support(source, end, false, total * 0.5, wall_id, ordinal, out);
    }
}

/// B `emit_support_beams`（0x141243c20）：rdp_support_indices → 索引对 →
/// ArchWalker(step≈3.15) 发射 64B 木板。
///
/// `points` 为已抬高曲线（world），`height` 为 plank 计算用高度
/// （原件 walk 在 `height−0.225` 处放置板底）。
pub fn emit_support_beams(
    points: &[Vec3],
    height: f32,
    wall_id: u64,
    ordinal: u32,
    out: &mut Vec<[u8; 64]>,
) {
    if points.len() < 2 {
        return;
    }
    let arrays: Vec<[f32; 3]> = points.iter().map(|p| p.to_array()).collect();
    let indices = rdp_support_indices(&arrays);
    for pair in indices.windows(2) {
        let a = pair[0];
        let b = pair[1];
        if b <= a {
            continue;
        }
        let heights: Vec<f32> = points[a..=b].iter().map(|_| height).collect();
        let mut us = Vec::with_capacity(b - a + 1);
        let mut accumulated = 0.0f32;
        us.push(0.0);
        for i in a + 1..=b {
            accumulated += (points[i] - points[i - 1]).length();
            us.push(accumulated);
        }
        let total = us.last().copied().unwrap_or(0.0);
        if total > 0.0 {
            for u in &mut us {
                *u /= total;
            }
        }
        let walker = ArchWalker::new(0, 0.0, f32::from_bits(0x40499999));
        let mut previous = points[a];
        walker.walk(&heights, &points[a..=b], &us, |sample, _| {
            emit_plank(previous, sample, wall_id, ordinal, a, out);
            previous = sample;
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn source() -> SupportBeamSource {
        SupportBeamSource {
            heights: vec![1.0, 1.0, 1.0, 1.0],
            ground_positions: vec![
                [0.0, 0.0, 0.0],
                [2.0, 0.0, 0.0],
                [4.0, 0.0, 0.0],
                [6.0, 0.0, 0.0],
            ],
            raised_positions: vec![
                [0.0, 1.0, 0.0],
                [2.0, 1.0, 0.0],
                [4.0, 1.0, 0.0],
                [6.0, 1.0, 0.0],
            ],
            flags: vec![0, 0, 0, 0],
            source_coords: vec![
                SupportBeamCoord {
                    point_index: 0,
                    fraction: 0.0,
                },
                SupportBeamCoord {
                    point_index: 1,
                    fraction: 0.0,
                },
                SupportBeamCoord {
                    point_index: 2,
                    fraction: 0.0,
                },
                SupportBeamCoord {
                    point_index: 2,
                    fraction: 1.0,
                },
            ],
        }
    }

    #[test]
    fn propose_normalizes_direction_and_costs_after_first_metre() {
        let grower = SupportBeamGrower::new(0, Vec3::ZERO);
        let points = [[0.0, 0.0, 0.0], [0.0, 0.0, 2.0]];
        let flags = [0u8, 0u8];
        let p = grower.propose(1, &points, &flags).unwrap();
        assert_eq!(p.index, 1);
        assert!((p.xz_length - 2.0).abs() < 1e-6);
        assert!((p.direction.z - 1.0).abs() < 1e-6);
        assert_eq!(p.curvature_cost, 0.0);
        assert_eq!(p.tag, 0);
    }

    #[test]
    fn propose_stops_on_low_flag_bits_and_rejects_over_315() {
        let grower = SupportBeamGrower::new(0, Vec3::ZERO);
        let points = [[0.0, 0.0, 0.0], [0.0, 0.0, 2.0], [0.0, 0.0, 4.0]];
        let flags = [0u8, 0x03u8, 0u8];
        let p = grower.propose(1, &points, &flags).unwrap();
        assert_eq!(p.tag, 1);
        let long = [[0.0, 0.0, 0.0], [0.0, 0.0, 4.0]];
        assert!(grower.propose(1, &long, &[0, 0]).is_none());
    }

    #[test]
    fn propose_cost_bit0_reads_grower_flags_not_point_flags() {
        let points = [[0.0, 0.0, 0.0], [0.0, 0.0, 2.0]];
        // grower 已有方向 X、累计长度 1；输入点 flags 的 bit0 不参与 cost。
        let mut base = SupportBeamGrower::new(0, Vec3::ZERO);
        base.direction = Some(Vec3::X);
        base.length = 1.0;
        let no_projection = base.propose(1, &points, &[0x01, 0x00]).unwrap();
        // grower flags=0 → Δdir=0 → cost = new_len(2+1)*(0+0.5)。
        assert!((no_projection.curvature_cost - 1.5).abs() < 1e-6);

        let mut with_projection = base;
        with_projection.flags = 1;
        let projected = with_projection.propose(1, &points, &[0x00, 0x00]).unwrap();
        // grower flags=1 → Δdir = |X−Z| = √2 → cost = 3*(√2+0.5)。
        let expected = 3.0 * (2.0f32.sqrt() + 0.5);
        assert!((projected.curvature_cost - expected).abs() < 1e-5);
    }

    #[test]
    fn propose_into_writes_tag_two_when_invalid() {
        let grower = SupportBeamGrower {
            stopped: true,
            ..SupportBeamGrower::new(0, Vec3::ZERO)
        };
        let points = [[0.0, 0.0, 0.0], [0.0, 0.0, 2.0]];
        let mut out = SupportBeamProposal {
            index: 0,
            xz_length: 0.0,
            direction: Vec3::ZERO,
            curvature_cost: 0.0,
            point: Vec3::ZERO,
            tag: 0,
        };
        grower.propose_into(1, &points, &[0, 0], &mut out);
        assert_eq!(out.tag, SupportBeamProposal::INVALID_TAG);
        assert_eq!(out.observed_bytes()[40], 2);
    }

    #[test]
    fn place_support_beams_emits_64b_records() {
        let src = source();
        let mut out = Vec::new();
        place_support_beams(&src, 7, 0, &mut out);
        assert!(!out.is_empty());
        // wall id 位于 56..60。
        for record in &out {
            assert_eq!(u32::from_le_bytes(record[56..60].try_into().unwrap()), 7);
        }
    }

    #[test]
    fn emit_support_beams_walks_with_315_step() {
        let points: Vec<Vec3> = (0..8).map(|i| Vec3::new(i as f32, 0.0, 0.0)).collect();
        let mut out = Vec::new();
        emit_support_beams(&points, 1.0, 3, 0, &mut out);
        assert_eq!(out.len(), 2);
    }
}
