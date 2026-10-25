//! 擦除操作中的子墙区间。保留父曲线参数的方向，切片后替换为新的子墙 ID。

/// 子墙：父墙标识加一段参数区间。
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ChildWall {
    /// 当前子墙标识。
    pub id: u64,
    /// 起点在父曲线上的归一化参数。
    pub a: f32,
    /// 终点在父曲线上的归一化参数，允许反向区间。
    pub b: f32,
}

/// 检查归一化曲线参数，拒绝 NaN 和区间外数值。
fn checked_curve_parameter(parameter: f32) -> f32 {
    assert!(
        (0.0..=1.0).contains(&parameter),
        "子墙参数必须位于 [0,1] 且为有限数值：{parameter}"
    );
    parameter
}

impl ChildWall {
    pub fn new(child_id: u64, start: f32, end: f32) -> Self {
        Self {
            id: child_id,
            a: start,
            b: end,
        }
    }

    /// 在当前参数区间内插值；校验的是结果，允许合法的局部外推。
    pub fn slice(&self, child_id: u64, start_fraction: f32, end_fraction: f32) -> ChildWall {
        let start =
            checked_curve_parameter(::utils::float_grid::lerp(self.a, self.b, start_fraction));
        let end = checked_curve_parameter(::utils::float_grid::lerp(self.a, self.b, end_fraction));
        ChildWall {
            id: child_id,
            a: start,
            b: end,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn slice_lerps_both_ends_inside_the_interval() {
        let wall = ChildWall::new(1, 0.25, 0.75);
        let slice = wall.slice(9, 0.0, 1.0);
        assert_eq!(slice, ChildWall::new(9, 0.25, 0.75));
        assert_eq!(wall.slice(2, 0.5, 0.5), ChildWall::new(2, 0.5, 0.5));
    }

    #[test]
    #[should_panic(expected = "子墙参数必须位于 [0,1] 且为有限数值：1.25")]
    fn slice_panics_when_the_result_leaves_unit_range() {
        ChildWall::new(1, 0.5, 1.0).slice(0, 0.0, 1.5);
    }

    #[test]
    #[should_panic(expected = "子墙参数必须位于 [0,1] 且为有限数值：NaN")]
    fn nan_takes_the_same_panic() {
        ChildWall::new(1, 0.0, 1.0).slice(0, f32::NAN, 0.0);
    }
}
