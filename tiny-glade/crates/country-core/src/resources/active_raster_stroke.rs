//! active_raster_stroke.rs — 世界栅格工具（路径/鹅卵石/水/花园）的进行中笔画。
//!
//! 记录总长 0x470 B（`index_mut` 0x140868cf0：`idx * 0x470 + base`），共 6 个槽位
//! （`drop_in_place<ActiveRasterStrokesStorage>` 0x1408981c0 依次释放偏移 k·0x470 的三个 Vec，k = 0..5）。
//! 布局（`start_new` 0x140868d00 / `add_point` 0x140868fb0 / `resample_and_smooth` 0x140868e40 / `draw_raster` 0x142260ae0）：
//!
//! | 偏移 | 字段 | 备注 |
//! |---|---|---|
//! | +0x00 | `fluid_started`（u64，只判最低字节） | `start_new` 清 0，`add_point` 置 1 |
//! | +0x08 | `fluid_cursor` | `start_new` 不重置 |
//! | +0x10 | `solid`：`Vec<Vec3>` | 已固化曲线 |
//! | +0x28 | `input`：`Vec<Vec3>` | 原始输入点 |
//! | +0x40 | `length`：f32 | 自上次阈值以来的累计弧长 |
//! | +0x48 | `resampled`：`Vec<Vec3>` | `resample_and_smooth` 输出，笔画光栅化消费 |
//! | +0x60 | `brush_size`：f32 | |
//! | +0x64 | `mode`：u8 | |
//! | +0x68 | `param_a`：u32 | `start_new` 第 5 个参数，用途未恢复 |
//! | +0x6c | `param_b`：f32 | `start_new` 第 6 个参数（`movss`），用途未恢复 |
//! | +0x70 | `dirty_previous`：`DirtyTiles` | |
//! | +0x270 | `dirty_current`：`DirtyTiles` | `draw_raster` 清零后重新光栅化，再与 +0x70 互换 |
//!
//! `InProgressRasterStroke` 的三个方法（`last`/`add_point`/`solidify_fluid_into`）与
//! `ActiveRasterStroke::start_new` 作用在同一个地址（`start_new` 直接 `call add_point`），
//! 所以它是记录最前面 0x48 B 的子结构。

// Preserve the original instruction-level expression order/NaN policy, API call shapes, and evidence/test transcription. See README.md.
#![allow(clippy::neg_cmp_op_on_partial_ord)]

use std::ops::{Index, IndexMut};

use glam::Vec3;
use utils::curve::{smooth_in_place, Curve3};

use crate::utils::dirty_tiles::DirtyTiles;

/// 槽位数。O：0x1408981c0 的释放序列最大偏移 0x1680（k = 5）。
pub const STROKE_COUNT: usize = 6;
/// 记录大小。O：0x140868cf3 `imul rax, rax, 0x470`。
pub const STROKE_RECORD_SIZE: usize = 0x470;
/// 累计弧长超过它就触发固化。O：0x142925904 = 1.0。
pub const FLUID_LENGTH_THRESHOLD: f32 = 1.0;
/// `solidify_fluid_into` 的重采样间距。O：`crashy_resample` 的第二个参数来自 0x142925870 的低 4 字节 = 0.5。
pub const SOLIDIFY_SPACING: f32 = 0.5;
/// `resample_and_smooth` 的平滑遍数。O：0x140868f55 `mov r8d, 0xa`。
pub const SMOOTH_ITERATIONS: usize = 10;

/// 记录最前面 0x48 B。
#[derive(Clone, Debug, Default)]
pub struct InProgressRasterStroke {
    fluid_started: bool,
    fluid_cursor: usize,
    solid: Vec<Vec3>,
    input: Vec<Vec3>,
    length: f32,
}

impl InProgressRasterStroke {
    pub fn solid(&self) -> &[Vec3] {
        &self.solid
    }

    pub fn input(&self) -> &[Vec3] {
        &self.input
    }

    pub fn length(&self) -> f32 {
        self.length
    }

    pub fn fluid_started(&self) -> bool {
        self.fluid_started
    }

    pub fn fluid_cursor(&self) -> usize {
        self.fluid_cursor
    }

    /// O：0x140868f70。输入为空时 `Option::unwrap` panic。
    pub fn last(&self) -> &Vec3 {
        self.input.last().unwrap()
    }

    /// O：0x140868fb0。
    ///
    /// 输入为空：直接入队。否则 `length = |last - p| + length` 后入队
    /// （距离平方按 `(dy² + dx²) + dz²` 相加）。`length > 1.0`（NaN 不触发）时：
    /// - `fluid_started && fluid_cursor >= 2`：把 `input[..cursor]` 固化进 `solid`，
    ///   `input.drain(..cursor - 1)`，`cursor = input.len()`；`length` 不清零。
    /// - 否则：同一个点再入队一次，`fluid_started = true`，`cursor = input.len()`，`length = 0`。
    pub fn add_point(&mut self, point: Vec3) {
        let Some(&last) = self.input.last() else {
            self.input.push(point);
            return;
        };
        let dy = last.y - point.y;
        let dx = last.x - point.x;
        let dz = last.z - point.z;
        let distance = ((dy * dy + dx * dx) + dz * dz).sqrt();
        // `addss` 对加数顺序不敏感（IEEE 加法可交换），写成 `+=` 位级等价。
        self.length += distance;
        self.input.push(point);
        if !(self.length > FLUID_LENGTH_THRESHOLD) {
            return;
        }
        let cursor = self.fluid_cursor;
        if self.fluid_started && cursor >= 2 {
            Self::solidify_fluid_into(&self.input[..cursor], &mut self.solid);
            self.input.drain(..cursor - 1);
            self.fluid_started = true;
            self.fluid_cursor = self.input.len();
        } else {
            self.input.push(point);
            self.fluid_started = true;
            self.fluid_cursor = self.input.len();
            self.length = 0.0;
        }
    }

    /// O：0x1408691b0。
    ///
    /// 1. 去掉相邻的完全相同的点（闭包 0x141308760：保存上一个点，`!=` 才保留，
    ///    每个点都会更新“上一个点”；NaN 恒不等）。
    /// 2. 不足 2 个点：原样当结果。否则 `Curve::try_new_from_points(.., false).unwrap()`
    ///    后 `crashy_resample(0.5)` 取其点列。
    /// 3. `out` 为空：整段追加；否则跳过结果的第一个点再追加（`extend(Skip(1))`）。
    pub fn solidify_fluid_into(points: &[Vec3], out: &mut Vec<Vec3>) {
        let mut previous: Option<Vec3> = None;
        let filtered: Vec<Vec3> = points
            .iter()
            .copied()
            .filter(|point| {
                let keep = previous != Some(*point);
                previous = Some(*point);
                keep
            })
            .collect();
        let resampled = if filtered.len() < 2 {
            filtered
        } else {
            Curve3::try_new_from_points(filtered, false)
                .unwrap()
                .crashy_resample(SOLIDIFY_SPACING)
                .points
        };
        if out.is_empty() {
            out.extend_from_slice(&resampled);
        } else {
            out.extend(resampled.into_iter().skip(1));
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct ActiveRasterStroke {
    in_progress: InProgressRasterStroke,
    resampled: Vec<Vec3>,
    brush_size: f32,
    mode: u8,
    param_a: u32,
    param_b: f32,
    dirty_previous: DirtyTiles,
    dirty_current: DirtyTiles,
}

impl ActiveRasterStroke {
    pub fn in_progress(&self) -> &InProgressRasterStroke {
        &self.in_progress
    }

    pub fn resampled(&self) -> &[Vec3] {
        &self.resampled
    }

    pub fn brush_size(&self) -> f32 {
        self.brush_size
    }

    pub fn mode(&self) -> u8 {
        self.mode
    }

    pub fn param_a(&self) -> u32 {
        self.param_a
    }

    pub fn param_b(&self) -> f32 {
        self.param_b
    }

    pub fn dirty_previous(&self) -> &DirtyTiles {
        &self.dirty_previous
    }

    pub fn dirty_current(&self) -> &DirtyTiles {
        &self.dirty_current
    }

    pub fn dirty_current_mut(&mut self) -> &mut DirtyTiles {
        &mut self.dirty_current
    }

    pub fn dirty_previous_mut(&mut self) -> &mut DirtyTiles {
        &mut self.dirty_previous
    }

    /// B 0x140957990 detaches the smoothed curve and zeros radius/mode before
    /// constructing the Arc history record. Input, blend parameters and dirty
    /// maps survive. Vec pointer=4 is the empty Vec3 sentinel, not capacity=4.
    pub fn take_history_stroke(&mut self) -> std::sync::Arc<super::raster_stroke::RasterStroke> {
        let points = std::mem::take(&mut self.resampled);
        let radius = std::mem::take(&mut self.brush_size);
        let draw = std::mem::take(&mut self.mode) != 0;
        std::sync::Arc::new(super::raster_stroke::RasterStroke {
            flags: self.param_a,
            strength: self.param_b,
            points: points.into(),
            radius,
            draw,
        })
    }

    /// `draw_raster` 在光栅化之后对两份位图做的 512 B 互换（0x142260c2d 起的 `movups` 序列）。
    pub fn swap_dirty(&mut self) {
        std::mem::swap(&mut self.dirty_previous, &mut self.dirty_current);
    }

    pub fn add_point(&mut self, point: Vec3) {
        self.in_progress.add_point(point);
    }

    pub fn last(&self) -> &Vec3 {
        self.in_progress.last()
    }

    /// O：0x140868d00。清空 `solid`/`input`/`resampled`、`length`、`fluid_started`
    /// （`fluid_cursor` 与两份位图保持不动），写入笔刷与模式，`add_point(point)`，
    /// 把 `point` 作为 `resampled` 的第一个点，最后写 `param_a`/`param_b`。
    pub fn start_new(
        &mut self,
        point: Vec3,
        mode: u8,
        brush_size: f32,
        param_a: u32,
        param_b: f32,
    ) {
        let in_progress = &mut self.in_progress;
        in_progress.solid = Vec::new();
        in_progress.input = Vec::new();
        in_progress.length = 0.0;
        in_progress.fluid_started = false;
        self.resampled = Vec::new();
        self.brush_size = brush_size;
        self.mode = mode;
        self.in_progress.add_point(point);
        self.resampled.push(point);
        self.param_a = param_a;
        self.param_b = param_b;
    }

    /// O：0x140868e40。`resampled = solid.clone()`，再把全部 `input` 固化追加进去，
    /// 最后 `smooth_in_place(resampled, 10, false)`（0x141465260，`closed = 0`）。
    pub fn resample_and_smooth(&mut self) {
        self.resampled.clear();
        self.resampled.extend_from_slice(&self.in_progress.solid);
        InProgressRasterStroke::solidify_fluid_into(&self.in_progress.input, &mut self.resampled);
        smooth_in_place(&mut self.resampled, SMOOTH_ITERATIONS, false);
    }
}

/// 6 个笔画槽位（PathRaster、Cobblestone1/2/3、Water、Garden 各一）。
#[derive(bevy_ecs::resource::Resource, Clone, Debug, Default)]
pub struct ActiveRasterStrokesStorage {
    strokes: [ActiveRasterStroke; STROKE_COUNT],
}

impl ActiveRasterStrokesStorage {}

impl Index<u8> for ActiveRasterStrokesStorage {
    type Output = ActiveRasterStroke;
    fn index(&self, index: u8) -> &ActiveRasterStroke {
        &self.strokes[usize::from(index)]
    }
}

/// O：0x140868cf0，`movzx eax, dl` 零扩展索引，不做越界检查（这里越界 panic）。
impl IndexMut<u8> for ActiveRasterStrokesStorage {
    fn index_mut(&mut self, index: u8) -> &mut ActiveRasterStroke {
        &mut self.strokes[usize::from(index)]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn p(x: f32) -> Vec3 {
        Vec3::new(x, 0.0, 0.0)
    }

    fn started(x: f32) -> ActiveRasterStroke {
        let mut stroke = ActiveRasterStroke::default();
        stroke.start_new(p(x), 1, 2.0, 7, 0.5);
        stroke
    }

    #[test]
    fn constants_match_the_exe() {
        assert_eq!(STROKE_RECORD_SIZE, 0x470);
        assert_eq!(STROKE_COUNT, 6);
        assert_eq!(FLUID_LENGTH_THRESHOLD, 1.0);
        assert_eq!(SOLIDIFY_SPACING, 0.5);
        assert_eq!(SMOOTH_ITERATIONS, 10);
    }

    #[test]
    fn start_new_resets_and_seeds_both_lists() {
        let mut stroke = started(3.0);
        assert_eq!(stroke.in_progress().input(), [p(3.0)]);
        assert_eq!(stroke.resampled(), [p(3.0)]);
        assert!(stroke.in_progress().solid().is_empty());
        assert_eq!(stroke.in_progress().length(), 0.0);
        assert_eq!(stroke.brush_size(), 2.0);
        assert_eq!(stroke.mode(), 1);
        assert_eq!(stroke.param_a(), 7);
        assert_eq!(stroke.param_b(), 0.5);
        // 再次开始：旧数据被清掉。
        stroke.add_point(p(3.4));
        stroke.start_new(p(9.0), 0, 1.0, 1, 2.0);
        assert_eq!(stroke.in_progress().input(), [p(9.0)]);
        assert_eq!(stroke.resampled(), [p(9.0)]);
    }

    #[test]
    fn start_new_keeps_the_fluid_cursor_and_dirty_bitmaps() {
        let mut stroke = started(0.0);
        for i in 1..=4 {
            stroke.add_point(p(0.4 * i as f32));
        }
        assert!(stroke.in_progress().fluid_cursor() > 0);
        let cursor = stroke.in_progress().fluid_cursor();
        stroke.dirty_current_mut().set_all();
        stroke.start_new(p(1.0), 0, 1.0, 0, 0.0);
        assert_eq!(stroke.in_progress().fluid_cursor(), cursor);
        assert!(!stroke.in_progress().fluid_started());
        assert!(!stroke.dirty_current().is_empty());
    }

    #[test]
    fn crossing_the_threshold_duplicates_the_point_and_resets_length() {
        let mut stroke = started(0.0);
        stroke.add_point(p(0.4));
        stroke.add_point(p(0.8));
        assert!(!stroke.in_progress().fluid_started());
        stroke.add_point(p(1.2));
        let progress = stroke.in_progress();
        assert!(progress.fluid_started());
        assert_eq!(progress.input().len(), 5);
        assert_eq!(progress.input()[3], progress.input()[4]);
        assert_eq!(progress.fluid_cursor(), 5);
        assert_eq!(progress.length(), 0.0);
        assert!(progress.solid().is_empty());
    }

    #[test]
    fn second_crossing_solidifies_and_drains_all_but_one_point() {
        let mut stroke = started(0.0);
        for x in [0.4, 0.8, 1.2, 1.6, 2.0, 2.4] {
            stroke.add_point(p(x));
        }
        let progress = stroke.in_progress();
        // 第二次越过阈值时 input = [0,.4,.8,1.2,1.2,1.6,2.0,2.4]：固化前 5 个，
        // 去重后是 0..1.2，重采样间距 0.5 → 3 点（首、中、尾）。
        assert_eq!(progress.solid().len(), 3);
        assert_eq!(progress.solid()[0], p(0.0));
        assert_eq!(progress.solid()[2], p(1.2));
        assert!((progress.solid()[1].x - 0.6).abs() < 1e-6);
        // drain(..cursor-1) 之后保留从重复点起的 4 个点。
        assert_eq!(progress.input(), [p(1.2), p(1.6), p(2.0), p(2.4)]);
        assert_eq!(progress.fluid_cursor(), 4);
        // 固化分支不清零 length。
        assert!(progress.length() > 1.0);
    }

    #[test]
    fn nan_length_never_triggers_solidify() {
        let mut stroke = started(0.0);
        stroke.add_point(Vec3::new(f32::NAN, 0.0, 0.0));
        assert!(!stroke.in_progress().fluid_started());
        assert_eq!(stroke.in_progress().input().len(), 2);
    }

    #[test]
    fn solidify_dedupes_adjacent_duplicates_only() {
        let mut out = Vec::new();
        InProgressRasterStroke::solidify_fluid_into(&[p(0.0), p(0.0), p(1.0), p(1.0)], &mut out);
        // 去重后 [0, 1]，长度 1，间距 0.5 → 3 点。
        assert_eq!(out.len(), 3);
        assert_eq!(out[0], p(0.0));
        assert_eq!(out[2], p(1.0));
        // 单个点：原样返回。
        let mut single = Vec::new();
        InProgressRasterStroke::solidify_fluid_into(&[p(2.0), p(2.0)], &mut single);
        assert_eq!(single, [p(2.0)]);
    }

    #[test]
    fn solidify_into_non_empty_output_skips_the_first_resampled_point() {
        let mut out = vec![p(-1.0)];
        InProgressRasterStroke::solidify_fluid_into(&[p(0.0), p(1.0)], &mut out);
        assert_eq!(out[0], p(-1.0));
        assert_eq!(out.len(), 3);
        assert_eq!(out[2], p(1.0));
    }

    #[test]
    fn resample_and_smooth_keeps_endpoints_and_rebuilds_from_scratch() {
        let mut stroke = started(0.0);
        for x in [0.3, 0.6, 0.9] {
            stroke.add_point(p(x));
        }
        stroke.resample_and_smooth();
        let first = stroke.resampled().to_vec();
        assert_eq!(first.first(), Some(&p(0.0)));
        assert_eq!(first.last(), Some(&p(0.9)));
        // 重复调用结果相同：它总是从 solid + input 重建。
        stroke.resample_and_smooth();
        assert_eq!(stroke.resampled(), first);
    }

    #[test]
    fn smoothing_is_ten_passes_of_the_quarter_half_quarter_kernel() {
        let mut stroke = started(0.0);
        stroke.add_point(Vec3::new(0.5, 0.5, 0.0));
        stroke.add_point(Vec3::new(1.0, 0.0, 0.0));
        stroke.resample_and_smooth();
        // 不经 resample_and_smooth 的平滑：固化后的点列，再按 0x141465260 的汇编
        // `p' = ((prev + next) * 0.5 - cur) * 0.5 + cur`（prev 取平滑前的值）手算 10 遍。
        let mut expected = Vec::new();
        InProgressRasterStroke::solidify_fluid_into(stroke.in_progress().input(), &mut expected);
        for _ in 0..10 {
            let mut previous = expected[0];
            for i in 1..expected.len() - 1 {
                let current = expected[i];
                expected[i] = ((previous + expected[i + 1]) * 0.5 - current) * 0.5 + current;
                previous = current;
            }
        }
        assert_eq!(stroke.resampled(), expected);
        assert!(expected.len() > 3);
        // 拐点确实被拉平了，但端点不动。
        assert!(expected[1].y < 0.5);
        assert_eq!(expected[0], p(0.0));
        assert_eq!(*expected.last().unwrap(), p(1.0));
    }

    #[test]
    #[should_panic(expected = "called `Option::unwrap()` on a `None` value")]
    fn last_of_an_empty_stroke_panics() {
        let _ = ActiveRasterStroke::default().last();
    }

    #[test]
    fn storage_has_six_independent_slots() {
        let mut storage = ActiveRasterStrokesStorage::default();
        storage.index_mut(5).start_new(p(1.0), 1, 1.0, 0, 0.0);
        assert_eq!(storage[5].resampled(), [p(1.0)]);
        assert!(storage[0].resampled().is_empty());
    }

    #[test]
    #[should_panic]
    fn storage_index_past_the_sixth_slot_panics() {
        let mut storage = ActiveRasterStrokesStorage::default();
        let _ = storage.index_mut(6);
    }

    #[test]
    fn swap_dirty_exchanges_the_two_bitmaps() {
        let mut stroke = ActiveRasterStroke::default();
        stroke.dirty_current_mut().set_all();
        stroke.swap_dirty();
        assert!(stroke.dirty_current().is_empty());
        assert!(!stroke.dirty_previous().is_empty());
    }
}
