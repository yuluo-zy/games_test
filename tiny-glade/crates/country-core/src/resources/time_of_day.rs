//! `time_of_day.rs` — 一天中的小时重映射（`remap_hour_to_fmod_hour`，B 0x140a74790）。
//!
//! 数据（O）：x 关键帧 @0x142ad5990 = `[0, 3.5, 9, 17, 20, 24]`，y 关键帧 @0x142ad59b0 = `[0, 3, 11, 17, 22, 24]`。
//! 冬季：x 在下标 2 插入 4.52、再在下标 3 插入 6.2；y 在下标 2 插入 5.0、再在下标 3 插入 9.0。
//! 两个数组最终长度都是 8（此前文档写成 x=7、y=9，与汇编 0x140a74885..0x140a74980 的四次插入不符）。
//!
//! 查找：对相邻关键帧对 `(x[i], x[i+1])`，`x[i] <= hour < x[i+1]` 时返回
//! `((hour - x0) / (x1 - x0)) * (y1 - y0) + y0`（O：0x140c918b0 的运算顺序）；没有命中（`hour < 0` 或 `>= 24`）
//! 原样返回 `hour`。
const X_SUMMER: [f32; 6] = [0.0, 3.5, 9.0, 17.0, 20.0, 24.0];
const Y_SUMMER: [f32; 6] = [0.0, 3.0, 11.0, 17.0, 22.0, 24.0];
const X_WINTER: [f32; 8] = [0.0, 3.5, 4.52, 6.2, 9.0, 17.0, 20.0, 24.0];
const Y_WINTER: [f32; 8] = [0.0, 3.0, 5.0, 9.0, 11.0, 17.0, 22.0, 24.0];

fn lerp(x: f32, x0: f32, x1: f32, y0: f32, y1: f32) -> f32 {
    ((x - x0) / (x1 - x0)) * (y1 - y0) + y0
}

fn remap(hour: f32, xs: &[f32], ys: &[f32]) -> f32 {
    let mut out = hour;
    for (x, y) in xs.windows(2).zip(ys.windows(2)) {
        if hour >= x[0] && hour < x[1] {
            out = lerp(hour, x[0], x[1], y[0], y[1]);
        }
    }
    out
}

/// 把游戏小时（0..24）重映射到日照曲线用的小时。
pub fn remap_hour_to_fmod_hour(hour: f32, winter: bool) -> f32 {
    if winter {
        remap(hour, &X_WINTER, &Y_WINTER)
    } else {
        remap(hour, &X_SUMMER, &Y_SUMMER)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keyframes_map_to_their_y_values() {
        for (x, y) in X_SUMMER.iter().zip(Y_SUMMER) {
            if *x < 24.0 {
                assert_eq!(remap_hour_to_fmod_hour(*x, false), y);
            }
        }
        for (x, y) in X_WINTER.iter().zip(Y_WINTER) {
            if *x < 24.0 {
                assert_eq!(remap_hour_to_fmod_hour(*x, true), y);
            }
        }
    }

    #[test]
    fn midpoints_interpolate_linearly() {
        assert_eq!(remap_hour_to_fmod_hour(6.25, false), 7.0);
        assert_eq!(remap_hour_to_fmod_hour(1.75, false), 1.5);
        assert!((remap_hour_to_fmod_hour(5.36, true) - 7.0).abs() < 1e-5);
    }

    #[test]
    fn out_of_range_hours_pass_through() {
        assert_eq!(remap_hour_to_fmod_hour(24.0, false), 24.0);
        assert_eq!(remap_hour_to_fmod_hour(-1.0, true), -1.0);
    }

    #[test]
    fn winter_differs_from_summer_only_inside_the_inserted_window() {
        assert_eq!(
            remap_hour_to_fmod_hour(2.0, true),
            remap_hour_to_fmod_hour(2.0, false)
        );
        assert_ne!(
            remap_hour_to_fmod_hour(5.0, true),
            remap_hour_to_fmod_hour(5.0, false)
        );
        assert_eq!(
            remap_hour_to_fmod_hour(12.0, true),
            remap_hour_to_fmod_hour(12.0, false)
        );
    }
}
