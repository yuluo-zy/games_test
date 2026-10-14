//! `country_core::utils::child_wall` — 子墙（父墙上的一段 `[a, b]` 参数区间）。
//!
//! O 0x140a24000 `ChildWall::slice`：入参 `(out, &self, id: u64, t1: f32, t2: f32)`。
//! 机器码：`lerp(self.a, self.b, t)` 是 0x140c904d0（`a + (b - a) * t`）；对 `t1`、`t2` 各算一次，
//! 每个结果都要落在 `[0, 1]`（`ucomiss` 先比 0 再比 1.0，NaN 也走同一个分支），否则
//! panic `Tried to init CurveU from an invalid u: {u}`（字符串在 0x142aa5398，格式化走 0x140d8c910）；
//! 返回 `{id, lerp(t1), lerp(t2)}`，布局 id@0（8 B）、a@8、b@0xc。
//!
//! 这里没有复用 `utils::curve::CurveU::new`：它的 panic 文本与 EXE 不同（`CurveU outside [0,1]`），
//! 而 `utils` 是独立的共享 crate，不在这里改。`ChildWall::new` 无独立符号（内联，I）。

/// 子墙：父墙标识加一段参数区间。
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ChildWall {
    /// +0：被 `slice` 原样带到结果里的 8 字节（I：父墙 id）。
    pub id: u64,
    pub a: f32,
    pub b: f32,
}

fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

/// 与 EXE 一致的 `[0, 1]` 检查：NaN 不在区间内，也落到同一个 panic 分支。
fn checked_u(u: f32) -> f32 {
    if !(0.0..=1.0).contains(&u) {
        panic!("Tried to init CurveU from an invalid u: {u}");
    }
    u
}

impl ChildWall {
    pub fn new(id: u64, a: f32, b: f32) -> Self {
        Self { id, a, b }
    }

    /// O 0x140a24000。
    pub fn slice(&self, id: u64, t1: f32, t2: f32) -> ChildWall {
        let a = checked_u(lerp(self.a, self.b, t1));
        let b = checked_u(lerp(self.a, self.b, t2));
        ChildWall { id, a, b }
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
    #[should_panic(expected = "Tried to init CurveU from an invalid u: 1.25")]
    fn slice_panics_when_the_result_leaves_unit_range() {
        ChildWall::new(1, 0.5, 1.0).slice(0, 0.0, 1.5);
    }

    #[test]
    #[should_panic(expected = "Tried to init CurveU from an invalid u: NaN")]
    fn nan_takes_the_same_panic() {
        ChildWall::new(1, 0.0, 1.0).slice(0, f32::NAN, 0.0);
    }
}
