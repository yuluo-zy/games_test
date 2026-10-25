//! B system-water 音频网格 CPU 更新 — 反汇编级证据草稿。
//!
//! 证据全文：`evidence/multi-baseline/water-extra-analysis.md`（§2）与
//! `evidence/toolchain/water-extra/`（reva dump + radare2 常量核对）。
//! 本模块不是 ECS/ABI 复刻，只恢复已证实的水域音频网格业务规则；所有未知项以 `// [unknown]`
//! 标注，不伪造。
//!
//! 签名（B 注册证据）：`(Local<WaterAudioGridUpdateState>, Res<WaterRaster>, Raycasting,
//! Res<MainCamera>, ResMut<WaterAudioGrid>)`，run_if `Option<Res<WaterAudioGrid>>`。
//! 函数 `system_water::water_audio_grid::update_water_audio_grid` @0x141baefe0
//! （PDB code_size 2477，源行 21–106）。

/// 每格 24B：位置(x,z) 2×f32 + u64 水位位掩码 + u32 双 u16 计数。
/// 证据：0x141baefe0 遍历 `Vec<Cell>`（stride 0x18）：
/// `MOVSD [R12]` 位置、`[R12+0x8]` 位掩码 OR/AND 置清除、`[R12+0x10]` PADDW/PSUBW 双计数。
#[derive(Clone, Debug)]
pub struct WaterAudioGridCell {
    pub xz: [f32; 2],   // +0x0, +0x4
    /// +0x8：64 位水位滚动窗；本帧位 = `bit_index & 63`。
    pub water_mask: u64,
    /// +0x10：2×u16 入水连续计数（与 mask 同增减）。[unknown] 双计数的精确语义。
    pub counters: u32,
}

/// 距离²阈值：80² = 6400.0（.rdata 0x142eed900，radare2 核对 0x45c80000）。
pub const AUDIO_RADIUS_SQ: f32 = 6400.0;
/// 入水阈值（归一样本）：样本 > 0.5 判水面（.rdata 0x142925870 = 0.5f）。
pub const WATER_PRESENT_THRESHOLD: f32 = 0.5;
/// 世界→栅格采样除数（bilinear_sample 的参数，.rdata 0x142a7b658 = 130.0f）。
pub const WORLD_SCALE: f32 = 130.0;
/// intersects_any 参数（XMM2=1.0 @0x142925904；R9D=0x32a3 标志位掩码）。[unknown] 语义。
pub const RAY_PARAM: f32 = 1.0;
pub const RAY_FLAGS: u32 = 0x32a3;

/// 每帧测试位 = (帧计数派生的 uVar12) & 63。
/// 证据：`uVar12 = lVar6 + lVar2*10 + 1`（lVar2 = lVar6+1，Local 为 u64 帧计数）。
pub fn frame_bit(frame_count: u64) -> u32 {
    (frame_count + (frame_count + 1) * 10 + 1) as u32 & 63
}

/// 单格水面判定 + 位窗更新（每帧一格测一位）。
/// 语义（0x141baf3f0 循环）：
/// - dist²≥6400 或 归一样本≤0.5 → 清位；若该位原置则递减计数。
/// - 否则 `Raycasting::intersects_any(相机→格心)`；无相交（水面可见）→ 置位；新置则递增计数。
/// 本函数只表达状态转换，不做相机/射线几何（依赖原实现语义）。
pub fn update_cell(cell: &mut WaterAudioGridCell, bit: u32, sample: f32, dist_sq: f32,
                   water_visible: bool) {
    let mask_bit: u64 = 1u64 << bit;
    let was_set = (cell.water_mask & mask_bit) != 0;
    let mark = dist_sq < AUDIO_RADIUS_SQ && sample > WATER_PRESENT_THRESHOLD && water_visible;
    if mark {
        if !was_set {
            // 新置位 → +0x10 双 u16 计数 +1（PADDW）。
            let (lo, hi) = (cell.counters as u16, (cell.counters >> 16) as u16);
            cell.counters = (hi.wrapping_add(1) as u32) << 16 | lo.wrapping_add(1) as u32;
        }
        cell.water_mask |= mask_bit;
    } else {
        if was_set {
            // 清位 → +0x10 双 u16 计数 -1（PSUBW）。
            let (lo, hi) = (cell.counters as u16, (cell.counters >> 16) as u16);
            cell.counters = (hi.wrapping_sub(1) as u32) << 16 | lo.wrapping_sub(1) as u32;
        }
        cell.water_mask &= !mask_bit;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bit_rotates_with_frame() {
        let b0 = frame_bit(0);
        let b1 = frame_bit(1);
        assert!(b0 < 64 && b1 < 64 && b0 != b1);
    }

    #[test]
    fn far_cell_clears_bit() {
        let mut c = WaterAudioGridCell { xz: [0.0; 2], water_mask: 1, counters: 0x0001_0001 };
        update_cell(&mut c, 0, 0.9, 9000.0, true);
        assert_eq!(c.water_mask, 0);
        assert_eq!(c.counters, 0);
    }

    #[test]
    fn visible_near_water_marks_cell() {
        let mut c = WaterAudioGridCell { xz: [0.0; 2], water_mask: 0, counters: 0 };
        update_cell(&mut c, 3, 0.9, 100.0, true);
        assert_eq!(c.water_mask, 1 << 3);
        assert_eq!(c.counters & 0xffff, 1);
    }
}
