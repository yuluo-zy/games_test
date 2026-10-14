//! `conservative_line_raster::conservative_plot_line` — 带半径的保守线段栅格化。
//!
//! 原版是自有工作区 crate（`crates/conservative-line-raster/src/lib.rs`，PDB 的 `impl$0..$3`、
//! `conservative_plot_line_low`、两个 `quantize_range`、两个 `assert_delta`）；这里作为 `utils` 下的一个模块。
//! 核对对象：`DirtyTiles` 的实例 0x140b1e460（3501 B，回调内联为"置位图"）。另外两个实例
//! （0x1402582d0 / 0x142820fb0）回调不同，主体相同。
//!
//! 算法（O，逐段读自机器码）：
//! 1. `dx = x1 - x0`、`dy = y1 - y0`。`|dx| > |dy|`（严格；NaN 走另一支）为 **x 主轴**，否则 **y 主轴**
//!    （同一份代码把两轴互换）。下面以 x 主轴叙述，`major`=x、`minor`=y。
//! 2. `slope = dy / dx`，非有限则 0；`c = r * (1 - |slope|)`（斜率非有限时 `c = r`）；`h = hypotf(slope, 1) * r`
//!    （0x142923cf0 = `_hypotf`，导入表实读）。
//! 3. 按主轴排序端点，`xs = x_min - r`，`xe = x_max + r + 0`（`x_max + r`），`ys = y@x_min - slope * r`。
//! 4. 首列 `col = floor(xs)`：`y_end = (min(xe, col + 1) - xs) * slope + ys`；
//!    斜率为正（`minor_end > minor_start`，调用 `assert_delta` ≥ 0）：行 `[trunc(ys - c), trunc(y_end + h)]`；
//!    否则（`assert_delta` ≤ 0）：行 `[trunc(y_end - h), trunc(ys + c)]`。
//! 5. 之后每列（`col < last = floor(xe)`）：`y_new = y_prev + slope`，
//!    正：`[trunc(y_prev - h), trunc(y_new + h)]`；负：`[trunc(y_new - h), trunc(y_prev + h)]`。
//! 6. 最后一列（`col == last`）：`y_at = slope * (xe - last) + y_prev`，
//!    正：`[trunc(y_prev - h), trunc(y_at + c)]`；负：`[trunc(y_at - c), trunc(y_prev + h)]`。
//! 7. 每个 `trunc` 都是 `cvttss2si` + 饱和（> 2147483520 → `i32::MAX`，NaN → 0），即 Rust 的 `as i32`。
//!    行区间是闭区间，起点大于终点则该列为空；回调收到 `(x, y)`，x 主轴时 `(col, row)`，y 主轴时 `(row, col)`。
//!
//! `DirtyTiles` 实例的回调：`(x | y) as u32 <= 63` 才置 `rows[y] |= 1 << x`（见 [`DirtyTiles::set_cell`](super::dirty_tiles::DirtyTiles)）。
//! 其它实例有一个 `&mut bool` 提前停止标志（0x140ade620 的外层循环检查它），这个实例不碰它。
//!
//! 偏差/不确定：`assert_delta` 的 panic 文本在 0x142af6f30（Display 一个 f32，源码行 ≥ 93），只在斜率符号与分支
//! 矛盾时触发——上面的分支条件已经保证不会，所以这里不复刻（I）。

fn trunc_i32(value: f32) -> i32 {
    value as i32
}

/// `minss(a, b)` 再按 `cmpunord` 选择：`b` 是 NaN 时取 `a`。
fn min_or_a(a: f32, b: f32) -> f32 {
    let m = if a < b { a } else { b };
    if b.is_nan() { a } else { m }
}

struct Axes {
    start_major: f32,
    start_minor: f32,
    end_major: f32,
    end_minor: f32,
}

fn plot_major(axes: Axes, slope: f32, c: f32, h: f32, r: f32, mut cell: impl FnMut(i32, i32)) {
    let Axes {
        start_major,
        start_minor,
        end_major,
        end_minor,
    } = axes;
    let positive = end_minor > start_minor;
    let xs = start_major - r;
    let ys = start_minor - slope * r;
    let xe = end_major + r;
    let c = c * 1.0;

    let mut col = trunc_i32(xs.floor());
    let first_end = min_or_a(xe, 1.0 + xs.floor());
    let y_end = (first_end - xs) * slope + ys;
    let (lo, hi) = if positive {
        (trunc_i32(ys - c), trunc_i32(y_end + h))
    } else {
        (trunc_i32(y_end - h), trunc_i32(ys + c))
    };
    let mut emit = |col: i32, lo: i32, hi: i32| {
        let mut row = lo;
        while row < hi.wrapping_add(1) {
            cell(col, row);
            row += 1;
        }
    };
    emit(col, lo, hi);

    let last = trunc_i32(xe.floor());
    let mut y_prev = y_end;
    loop {
        col = col.wrapping_add(1);
        if col > last {
            return;
        }
        if col == last {
            let y_at = slope * (xe - xe.floor()) + y_prev;
            let (lo, hi) = if positive {
                (trunc_i32(y_prev - h), trunc_i32(y_at + c))
            } else {
                (trunc_i32(y_at - c), trunc_i32(y_prev + h))
            };
            emit(col, lo, hi);
            return;
        }
        let y_new = slope + y_prev;
        let (lo, hi) = if positive {
            (trunc_i32(y_prev - h), trunc_i32(y_new + h))
        } else {
            (trunc_i32(y_new - h), trunc_i32(y_prev + h))
        };
        emit(col, lo, hi);
        y_prev = y_new;
    }
}

/// O 0x140b1e460 系：把半径 `r` 的线段 `(x0, y0) → (x1, y1)` 覆盖到的格子逐个交给 `cell(x, y)`。
pub fn conservative_plot_line(
    (x0, y0): (f32, f32),
    (x1, y1): (f32, f32),
    r: f32,
    mut cell: impl FnMut(i32, i32),
) {
    let dx = x1 - x0;
    let dy = y1 - y0;
    let finite = |v: f32| v.abs().to_bits() < 0x7f80_0000;
    if dx.abs() > dy.abs() {
        let raw = dy / dx;
        let slope = if finite(raw) { raw } else { 0.0 };
        let c = r * if finite(raw) { 1.0 - raw.abs() } else { 1.0 };
        let (first, second) = if x0 <= x1 {
            ((x0, y0), (x1, y1))
        } else {
            ((x1, y1), (x0, y0))
        };
        let axes = Axes {
            start_major: first.0,
            start_minor: first.1,
            end_major: second.0,
            end_minor: second.1,
        };
        let h = slope.hypot(1.0) * r;
        plot_major(axes, slope, c, h, r, &mut cell);
    } else {
        let raw = dx / dy;
        let slope = if finite(raw) { raw } else { 0.0 };
        let c = r * if finite(raw) { 1.0 - raw.abs() } else { 1.0 };
        let (first, second) = if y0 <= y1 {
            ((x0, y0), (x1, y1))
        } else {
            ((x1, y1), (x0, y0))
        };
        let axes = Axes {
            start_major: first.1,
            start_minor: first.0,
            end_major: second.1,
            end_minor: second.0,
        };
        let h = slope.hypot(1.0) * r;
        plot_major(axes, slope, c, h, r, |major, minor| cell(minor, major));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;

    fn cells(p0: (f32, f32), p1: (f32, f32), r: f32) -> BTreeSet<(i32, i32)> {
        let mut out = BTreeSet::new();
        conservative_plot_line(p0, p1, r, |x, y| {
            out.insert((x, y));
        });
        out
    }

    #[test]
    fn horizontal_line_covers_a_band_of_rows() {
        let got = cells((2.5, 5.5), (6.5, 5.5), 0.5);
        // 带宽 [5.0, 6.0] 的上沿正好压在第 6 行的边界上，`trunc(6.0) = 6`，所以保守地算进去。
        for x in 2..=7 {
            assert!(got.contains(&(x, 5)) && got.contains(&(x, 6)), "column {x}");
        }
        assert!(
            got.iter()
                .all(|&(x, y)| (2..=7).contains(&x) && (5..=6).contains(&y))
        );
    }

    #[test]
    fn vertical_line_uses_the_swapped_axes() {
        let got = cells((3.5, 1.5), (3.5, 6.5), 0.4);
        for y in 1..=6 {
            assert!(got.contains(&(3, y)), "row {y}");
        }
        assert!(got.iter().all(|&(x, _)| x == 3));
    }

    #[test]
    fn diagonal_is_conservative() {
        // 对角线：半径 0 时也必须覆盖线段穿过的每个格（相邻格共边或共角）。
        let got = cells((0.2, 0.2), (4.8, 4.8), 0.0);
        for i in 0..5 {
            assert!(got.contains(&(i, i)), "diagonal cell {i}");
        }
    }

    #[test]
    fn point_like_segment_marks_its_cell() {
        let got = cells((7.3, 9.1), (7.3, 9.1), 0.0);
        assert!(got.contains(&(7, 9)));
    }

    #[test]
    fn reversed_endpoints_give_the_same_cells() {
        let a = cells((1.2, 8.7), (9.4, 2.1), 0.7);
        let b = cells((9.4, 2.1), (1.2, 8.7), 0.7);
        assert_eq!(a, b);
    }

    #[test]
    fn negative_slope_band_follows_the_line() {
        let got = cells((0.5, 6.5), (6.5, 0.5), 0.0);
        for i in 0..=6 {
            assert!(got.contains(&(i, 6 - i)), "cell {i}");
        }
    }

    #[test]
    fn nan_input_does_not_loop_forever() {
        let mut n = 0;
        conservative_plot_line((f32::NAN, 0.0), (1.0, 1.0), 0.5, |_, _| n += 1);
        assert!(n < 1_000);
    }
}
