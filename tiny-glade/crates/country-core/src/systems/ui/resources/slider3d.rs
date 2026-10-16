//! `slider3d.rs` — 3D 世界空间滑块（B 0x140a9bd40 `project_cursor_onto_line`，O）。
//!
//! 证据：`chapters/systems/ui/resources/slider3d.rs.md`：
//! - `Slider3dState` 世界空间线段拖拽状态；`project_cursor_onto_line`（0x140a9bd40，
//!   wp15 b7 既有）游标→线段投影；
//! - `Slider3d` bevy SystemParam（SystemParam 胶水面，get_param/queue/init_state 1B no-op）。
use glam::Vec3;

/// 3D 滑块状态（O：世界空间线段拖拽状态；字段语义 I）。
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Slider3dState {
    /// 线段起点（世界）。
    pub start_ws: Vec3,
    /// 线段终点（世界）。
    pub end_ws: Vec3,
    /// 当前拖拽参数 t ∈ [0, 1]（I）。
    pub t: f32,
}

impl Slider3dState {
    /// `project_cursor_onto_line`（0x140a9bd40，O 结构）：游标投影到线段上，
    /// 返回参数化 t ∈ [0, 1] 与最近点。
    pub fn project_cursor_onto_line(&self, cursor_ws: Vec3) -> (f32, Vec3) {
        let axis = self.end_ws - self.start_ws;
        let len_sq = axis.length_squared();
        if len_sq <= f32::EPSILON {
            return (0.0, self.start_ws);
        }
        let t = ((cursor_ws - self.start_ws).dot(axis) / len_sq).clamp(0.0, 1.0);
        (t, self.start_ws + axis * t)
    }
}
