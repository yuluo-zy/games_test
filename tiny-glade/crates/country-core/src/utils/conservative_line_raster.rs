//! 带半径线段的保守栅格覆盖；支持在枚举前限制输出矩形。
//!
//! 半径按轴对齐外扩计算，保证覆盖圆头笔画，允许少量多标记的边界格。
//! 有界入口的开销只与输出矩形及线段数量有关，不枚举矩形外的格子。

use std::ops::RangeInclusive;

/// 枚举整个 i32 坐标域内的覆盖格。输出量取决于实际线段跨度。
/// 绘制固定大小位图时应使用有界入口，避免无意义的大范围输出。
pub fn conservative_plot_line(
    start: (f32, f32),
    end: (f32, f32),
    radius: f32,
    visit: impl FnMut(i32, i32),
) {
    conservative_plot_line_in_bounds(
        start,
        end,
        radius,
        [i32::MIN..=i32::MAX, i32::MIN..=i32::MAX],
        visit,
    );
}

/// 枚举指定 X/Y 闭区间中的覆盖格；非有限坐标、负半径或空边界不产生输出。
///
/// 用 f64 进行范围和直线计算，避免有限 f32 端点相减时溢出。
/// 整数范围始终先相交后转换，既不饱和到错误位置，也不对整数上界加一。
pub fn conservative_plot_line_in_bounds(
    start: (f32, f32),
    end: (f32, f32),
    radius: f32,
    bounds: [RangeInclusive<i32>; 2],
    mut visit: impl FnMut(i32, i32),
) {
    if ![start.0, start.1, end.0, end.1, radius]
        .iter()
        .all(|value| value.is_finite())
        || radius < 0.0
        || bounds.iter().any(RangeInclusive::is_empty)
    {
        return;
    }
    let start = [f64::from(start.0), f64::from(start.1)];
    let end = [f64::from(end.0), f64::from(end.1)];
    let radius = f64::from(radius);
    let major_axis = usize::from((end[1] - start[1]).abs() >= (end[0] - start[0]).abs());
    let minor_axis = 1 - major_axis;
    let (start, end) = if start[major_axis] <= end[major_axis] {
        (start, end)
    } else {
        (end, start)
    };
    let Some(columns) = intersect_cells(
        start[major_axis] - radius,
        end[major_axis] + radius,
        &bounds[major_axis],
    ) else {
        return;
    };
    let major_length = end[major_axis] - start[major_axis];
    let slope = if major_length == 0.0 {
        0.0
    } else {
        (end[minor_axis] - start[minor_axis]) / major_length
    };
    // f32 端点乘积最多 48 位有效数，f64 能保留；先计算叉积再除长度，
    // 避免用远端位置减 slope*position 时把穿过原点的直线偏移到极远处。
    let intercept = if major_length == 0.0 {
        start[minor_axis]
    } else {
        (start[minor_axis] * end[major_axis] - end[minor_axis] * start[major_axis]) / major_length
    };
    for column in columns {
        let segment_start = (f64::from(column) - radius).max(start[major_axis]);
        let segment_end = (f64::from(column) + 1.0 + radius).min(end[major_axis]);
        let first_minor = slope.mul_add(segment_start, intercept);
        let last_minor = slope.mul_add(segment_end, intercept);
        let Some(rows) = intersect_cells(
            first_minor.min(last_minor) - radius,
            first_minor.max(last_minor) + radius,
            &bounds[minor_axis],
        ) else {
            continue;
        };
        for row in rows {
            if major_axis == 0 {
                visit(column, row);
            } else {
                visit(row, column);
            }
        }
    }
}

/// 连续坐标覆盖的整数格区间与输出边界求交。floor 保证负坐标也落在正确格中。
fn intersect_cells(
    minimum: f64,
    maximum: f64,
    bounds: &RangeInclusive<i32>,
) -> Option<RangeInclusive<i32>> {
    let minimum = minimum.floor().max(f64::from(*bounds.start()));
    let maximum = maximum.floor().min(f64::from(*bounds.end()));
    (minimum <= maximum).then_some(minimum as i32..=maximum as i32)
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

    #[test]
    fn extreme_points_do_not_wrap_into_the_grid() {
        assert!(cells((0.0, 3e9), (0.0, 3e9), 0.0).is_empty());
        assert!(cells((3e9, 0.0), (3e9, 0.0), 0.0).is_empty());
        assert_eq!(
            cells((0.0, i32::MIN as f32), (0.0, i32::MIN as f32), 0.0),
            BTreeSet::from([(0, i32::MIN)])
        );
    }

    #[test]
    fn huge_crossing_strokes_are_bounded_and_still_cover_the_grid() {
        let mut visited = BTreeSet::new();
        conservative_plot_line_in_bounds(
            (-f32::MAX, 32.5),
            (f32::MAX, 32.5),
            0.0,
            [0..=63, 0..=63],
            |x, y| {
                visited.insert((x, y));
            },
        );
        assert_eq!(visited, (0..64).map(|x| (x, 32)).collect::<BTreeSet<_>>());
        let mut count = 0;
        conservative_plot_line_in_bounds(
            (0.0, 0.0),
            (1.0, 1.0),
            f32::MAX,
            [0..=63, 0..=63],
            |_, _| count += 1,
        );
        assert_eq!(count, 4096);
        let mut diagonal = BTreeSet::new();
        conservative_plot_line_in_bounds(
            (-f32::MAX, -f32::MAX * 0.3),
            (f32::MAX, f32::MAX * 0.3),
            0.0,
            [0..=63, 0..=63],
            |column, row| {
                diagonal.insert((column, row));
            },
        );
        assert!(diagonal.contains(&(10, 3)) && diagonal.contains(&(50, 15)));
    }

    #[test]
    fn negative_coordinates_use_floor_and_invalid_input_is_ignored() {
        assert_eq!(
            cells((-0.25, -0.25), (-0.25, -0.25), 0.0),
            BTreeSet::from([(-1, -1)])
        );
        assert!(cells((0.0, 0.0), (1.0, 1.0), -1.0).is_empty());
        assert!(cells((f32::INFINITY, 0.0), (1.0, 1.0), 0.0).is_empty());
    }

    #[test]
    fn bounded_output_matches_filtering_for_each_axis_and_endpoint_order() {
        for (start, end) in [
            ((-10.2, 1.3), (100.5, 60.7)),
            ((5.2, -15.4), (60.8, 90.3)),
            ((6.3, 5.8), (-2.4, -8.1)),
        ] {
            for radius in [0.0, 0.2, 3.5] {
                let expected: BTreeSet<_> = cells(start, end, radius)
                    .into_iter()
                    .filter(|&(column, row)| (0..64).contains(&column) && (0..64).contains(&row))
                    .collect();
                for (start, end) in [(start, end), (end, start)] {
                    let mut actual = BTreeSet::new();
                    conservative_plot_line_in_bounds(
                        start,
                        end,
                        radius,
                        [0..=63, 0..=63],
                        |column, row| {
                            actual.insert((column, row));
                        },
                    );
                    assert_eq!(actual, expected);
                }
            }
        }
    }
}
