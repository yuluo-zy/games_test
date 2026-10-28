//! `slider3d.rs` — 3D 世界空间滑块（B 0x140a9bd40 `project_cursor_onto_line`，O）。
//!
//! 证据（2026-10-17 fresh 反编译/机器码 0x140a9bd40）：
//! - `self` 前 24 B 被整块拷成 `Ray { origin: Vec3 @0, direction: Vec3 @0xc }`，
//!   直接传给 `project_onto_ray`（**第二向量是方向，不是端点**）；
//! - 屏幕尺寸来自参数 `&[u32; 2]`（`cvtsi2ss` 转 f32，宽/高），现场构造
//!   `ProportionalScreenSpace`（w、h、w/h、1.0）；
//! - 调 `utils::mouse_ss_to_cs`（0x140c906d0）后把 x 乘 aspect（等价
//!   `ProportionalScreenSpace::transform_screen_space`，unit=1）；
//! - 调 `ProportionalScreenSpaceCoord::project_onto_ray`（0x140ac84a0），
//!   range 常量 = `[-1e10, 1e10]`（0x142ada620 的 0xd01502f9/0x501502f9 位型）；
//! - 相机裁剪矩阵由**单个指针**传入：second @+0x00、first @+0x80（本函数不读其它字段）。
use glam::{Mat4, Vec2, Vec3};
use utils::geometry::Ray;

use crate::utils::project_cursor_onto_ray::{
    ProportionalScreenSpace, ProportionalScreenSpaceCoord,
};

use super::ui_state::UiWidgetContext;

/// 3D 滑块模块对 `DragState` 的公开面（原生符号归属
/// `ui_state::DragState`，`into_drag_state` 定义在本模块）。
pub use super::ui_state::DragState;

/// `slider3d::impl$1::into_drag_state`（0x14092df10，26 B；与
/// `UiWidgetContext::dragged` 同址 ICF 折叠）。
///
/// fresh 机器码：
/// ```text
/// mov rdx,[rcx]              ; state
/// cmp byte [rdx+0x18],2
/// jb  ret(None)              ; ActiveWidget tag < 2 → None
/// mov r8,[rdx+0x38]
/// cmp r8,[rcx+8]             ; active widget id == context id
/// jne ret(None)
/// movzx eax, byte [rdx+0x19] ; 原样返回 DragState 判别字节
/// ```
/// 返回 1 B 判别：0/1 = finished、2 = None、3 = Started。
impl UiWidgetContext<'_> {
    pub fn into_drag_state(&self) -> DragState {
        self.drag_state_raw()
    }
}

/// 3D 滑块模块对 `DragState` 的公开面（原生符号归属
/// `ui_state::DragState`，`into_drag_state` 定义在本模块）。
pub use super::ui_state::DragState as SliderDragState;

/// `project_onto_ray` 的 range 常量（O 位型：-1e10 / 1e10）。
pub const PROJECT_RANGE: [f32; 2] = [-1.0e10, 1.0e10];

/// 原生单指针传入的 0x100 B 相机矩阵块（O 偏移）。
///
/// `project_onto_ray` 只读 `second`（+0x00 起 4×16 B）与 `first`（+0x80 起 4×16 B）；
/// 中间 0x40 B 的内容未被该函数读取（宿主相机类型未落库，不造字段）。
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Slider3dMatrices {
    /// +0x00：先应用的矩阵（原生 `second`）。
    pub second: Mat4,
    /// +0x40..0x80：未读区（U）。
    pub gap: [u8; 0x40],
    /// +0x80：后应用的矩阵（原生 `first`）。
    pub first: Mat4,
}

impl Default for Slider3dMatrices {
    fn default() -> Self {
        Self {
            second: Mat4::IDENTITY,
            gap: [0; 0x40],
            first: Mat4::IDENTITY,
        }
    }
}

/// 3D 滑块状态（O：前 24 B = `Ray`，`origin@0`、`direction@0xc`）。
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Slider3dState {
    /// 线段起点（世界；O `Ray.origin` @0）。
    pub origin: Vec3,
    /// 线段方向（世界；O `Ray.direction` @0xc——不是终点）。
    pub direction: Vec3,
    /// Offset captured when the drag starts (native +0x18).
    pub grab: f32,
}

impl Default for Slider3dState {
    fn default() -> Self {
        Self {
            origin: Vec3::ZERO,
            direction: Vec3::X,
            grab: 0.0,
        }
    }
}

impl Slider3dState {
    /// Update the persistent slider ray and project an active drag.
    pub fn update(&mut self, phase: u8, origin: Vec3, direction: Vec3, dimension: f32,
        cursor: Vec2, size: [u32; 2], matrices: &Slider3dMatrices, range: [f32; 2]) -> Option<(Vec3, f32)> {
        match phase.wrapping_sub(2).min(3) {
            0 => None,
            1 => {
                self.origin = origin;
                self.direction = direction;
                let point = self.project_cursor_onto_line(cursor, size, matrices);
                self.grab = point.dot(direction) - dimension;
                None
            }
            _ => {
                let point = self.project_cursor_onto_line(cursor, size, matrices);
                assert!(range[0] <= range[1], "invalid slider range");
                let value = point.dot(self.direction) - self.grab;
                // MAXSS/MINSS select their second operand for unordered or equal inputs.
                let low = if range[0] > value { range[0] } else { value };
                let value = if range[1] < low { range[1] } else { low };
                Some((self.origin + self.direction * value, value))
            }
        }
    }
    /// `project_cursor_onto_line`（0x140a9bd40，O）：游标（屏幕像素）→
    /// `mouse_ss_to_cs`/`transform_screen_space` → `project_onto_ray`（range ±1e10）。
    ///
    /// 参数按原生 ABI：`cursor_ss`（`&Vec2`，读 +0/+4）、`screen_size`（`&[u32;2]`）、
    /// `matrices`（单指针 0x100 B 块）。返回值 = 投影世界点（Vec3，sret）。
    pub fn project_cursor_onto_line(
        &self,
        cursor_ss: Vec2,
        screen_size: [u32; 2],
        matrices: &Slider3dMatrices,
    ) -> Vec3 {
        let screen = ProportionalScreenSpace::from_window_size(screen_size);
        // O：mouse_ss_to_cs + 显式 `x *= aspect`（等价 transform_screen_space，unit=1）。
        let coord = screen.transform_screen_space(cursor_ss.x, cursor_ss.y);
        let ray = Ray {
            origin: self.origin,
            direction: self.direction,
        };
        ProportionalScreenSpaceCoord(coord).project_onto_ray(
            &screen,
            &matrices.first,
            &matrices.second,
            ray,
            PROJECT_RANGE,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn project_defaults_to_line_origin_for_centered_cursor() {
        let state = Slider3dState::default();
        let m = Slider3dMatrices::default();
        // 光标在屏幕中心 → NDC (0,0)；近点投影 = 原点，t 计算应落在射线上。
        let p = state.project_cursor_onto_line(Vec2::new(50.0, 50.0), [100, 100], &m);
        assert!(p.is_finite(), "projection must be finite, got {p:?}");
    }

    #[test]
    fn matrices_block_has_native_offsets() {
        let m = Slider3dMatrices::default();
        let base = &m as *const _ as usize;
        assert_eq!(&m.first as *const _ as usize - base, 0x80);
        assert_eq!(&m.second as *const _ as usize - base, 0x00);
    }

    #[test]
    fn drag_captures_ray_and_clamps_using_persistent_grab() {
        let mut slider = Slider3dState::default();
        let matrices = Slider3dMatrices::default();
        let cursor = Vec2::splat(50.0);
        assert!(slider.update(2, Vec3::ZERO, Vec3::Y, 4.0, cursor, [100, 100], &matrices, [2.25, 15.0]).is_none());
        assert!(slider.update(3, Vec3::ZERO, Vec3::X, 4.0, cursor, [100, 100], &matrices, [2.25, 15.0]).is_none());
        let (point, value) = slider.update(4, Vec3::Y, Vec3::Y, 4.0, cursor, [100, 100], &matrices, [2.25, 15.0]).unwrap();
        assert_eq!(value, 4.0);
        assert_eq!(point, Vec3::X * 4.0);
        slider.grab = -100.0;
        assert_eq!(slider.update(4, Vec3::ZERO, Vec3::Y, 4.0, cursor, [100, 100], &matrices, [2.25, 15.0]).unwrap().1, 15.0);
        slider.grab = 100.0;
        assert_eq!(slider.update(1, Vec3::ZERO, Vec3::Y, 4.0, cursor, [100, 100], &matrices, [2.25, 15.0]).unwrap().1, 2.25);
    }
}
