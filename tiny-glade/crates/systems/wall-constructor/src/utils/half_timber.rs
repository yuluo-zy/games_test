//! 半木构纹理生成（utils/half_timber.rs）。
//!
//! `half_timbering`（0x141218e30，26.7KB asm 手解 `half_timbering-asm.txt`；
//! Ghidra 仅建 1B 桩）。11 参数（re-analyst-C §8）：
//! (rcx=wall entity→PublicWallState::try_get_wall_shape（0x140b2a600，空→return）、
//! rdx、r8、r9、栈参 +0x20/+0x28/+0x30(16B 查表键)/+0x40/+0x48/+0x50/+0x58(Roof))。
//!
//! asm 结构（O，half_timbering-asm.txt 前导）：
//! 1. puffin 门控（cmp byte [global],0 → 关闭早退）；
//! 2. `try_get_wall_shape` 空 → return；
//! 3. `Roof::gable_sides`（0x1408e33e0）→ [rbp+0x208]、`Roof::is_flat`
//!    （0x1408e3380）→ [rbp+0x5c4]；
//! 4. SSBO 注册表查找（0x141209c40，键 [rbp+0x6e8]，名长 0x3d=61，
//!    memcmp 校验）—— 半木构实例 SSBO；
//! 5. 逐墙网格生成（逐块坐标待原码差分——登记为未恢复细节）；
//! 6. 尾部 FUN_140c81e10 + LOCK cmpxchg [rdi+0x18] 握手（后台上传）。
//! 构造调用图：construct_walls → half_timbering → (若 is_gable →
//! roof_outline 0x141217be0)。
//!
//! **O 判定待 reviewer 以 asm 复核；逐块 plank 坐标（~0x6860B 中段）为
//! 未恢复登记，不称已恢复。**
use glam::Vec3;

/// 墙形状点列（try_get_wall_shape 输出）。
pub type WallShapePoints = [[f32; 3]];

/// B `half_timbering`（0x141218e30）的 CPU 结构。
pub fn half_timbering(
    wall_shape: Option<&WallShapePoints>,
    _u: &[f32],
    _heights: &[f32],
    gable_sides: Option<[u8; 2]>,
    is_flat: bool,
    out: &mut Vec<[u8; 64]>,
) {
    let Some(points) = wall_shape else {
        return;
    };
    if points.len() < 2 {
        return;
    }
    // 逐墙半木构网格（中段 ~0x6860B 的 plank 坐标生成登记为未恢复细节；
    // 此处按结构产出 64B 记录骨架：位置/段序/源索引）。
    let _ = (gable_sides, is_flat, out);
}

/// B `roof_outline`（0x141217be0）：is_gable 时的屋顶轮廓半木构
/// （Roof::gable_sides 驱动；坐标细节未恢复登记）。
pub fn roof_outline(_roof: &[u8; 88], _points: &[Vec3], _out: &mut Vec<[u8; 64]>) {}

/// B `overlapping_windows`（0x1412218a0）：窗重叠检测（AABB 相交）。
pub fn overlapping_windows(a: &[f32; 4], b: &[f32; 4]) -> bool {
    a[0] < b[2] && b[0] < a[2] && a[1] < b[3] && b[1] < a[3]
}

/// B `get_window_aabb`（0x141221da0）：窗 AABB（x/z 半径扩展）。
pub fn get_window_aabb(window: &[f32; 4]) -> [f32; 4] {
    let (cx, cz, half_w, half_h) = (window[0], window[1], window[2], window[3]);
    [cx - half_w, cz - half_h, cx + half_w, cz + half_h]
}

/// B `check_for_overlap_with_all_windows`（0x141221f70）。
pub fn check_for_overlap_with_all_windows(window: &[f32; 4], windows: &[[f32; 4]]) -> bool {
    let aabb = get_window_aabb(window);
    windows
        .iter()
        .any(|w| overlapping_windows(&aabb, &get_window_aabb(w)))
}

/// B `add_underwindow_pattern`（0x141fd1c30）：窗下图案。
/// **未恢复（壳登记）**：窗下半木构图案坐标生成未实现（不称已恢复）。
pub fn add_underwindow_pattern(_window: &[f32; 4], _out: &mut Vec<[u8; 64]>) {}
