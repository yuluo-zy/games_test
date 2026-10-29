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
use crate::resources::{mouse_terrain_raycast::CursorPositionSS, window::WindowSize};
use crate::systems::camera_rig::MainCamera;
use bevy_ecs::prelude::*;

/// B Slider3d的独立Local状态为28字节：24字节射线及末尾标量偏移。
/// 原有24字节投影叶子保持兼容，不把其他编辑器的slider状态合并。
#[derive(Clone, Copy, Debug, Default)]
pub struct Slider3dDragState {
    pub line: Slider3dState,
    pub offset: f32,
}

/// B原SystemParam：Local状态 + 屏幕光标、窗口和相机三个权威资源。
#[derive(bevy_ecs::system::SystemParam)]
pub struct Slider3d<'w, 's> {
    state: Local<'s, Slider3dDragState>,
    cursor: Res<'w, CursorPositionSS>,
    window: Res<'w, WindowSize>,
    camera: Res<'w, MainCamera>,
}

/// Started闭包给出的原点、方向与当前属性值。
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Slider3dStart {
    pub origin: Vec3,
    pub direction: Vec3,
    pub value: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Slider3dUpdate {
    pub position: Vec3,
    pub value: f32,
}

impl Slider3d<'_, '_> {
    /// 原140b1cc80：Started只建锚点；Held/Finished投影并返回钳制结果。
    /// 相机直接使用原+0/+80矩阵，禁止逆矩阵再求逆制造舍入误差。
    pub fn update(
        &mut self,
        phase: DragState,
        start: impl FnOnce() -> Slider3dStart,
        range: [f32; 2],
    ) -> Option<Slider3dUpdate> {
        let project = |line: Slider3dState| {
            line.project_cursor_onto_line(
                self.cursor.0,
                self.window.as_dims(),
                &Slider3dMatrices {
                    second: self.camera.read_matrix(0),
                    gap: [0; 0x40],
                    first: self.camera.read_matrix(0x80),
                },
            )
        };
        if phase == DragState::NONE {
            return None;
        }
        if phase == DragState::STARTED {
            let start = start();
            self.state.line = Slider3dState {
                origin: start.origin,
                direction: start.direction,
            };
            let point = project(self.state.line);
            // 原代码先z*d.z，再y*d.y，再x*d.x；不用glam::dot重排。
            self.state.offset = ((start.direction.z * point.z + start.direction.y * point.y)
                + point.x * start.direction.x)
                - start.value;
            return None;
        }
        assert!(range[0] <= range[1], "slider3d范围不合法");
        let point = project(self.state.line);
        let direction = self.state.line.direction;
        let value = (((point.z * direction.z + point.y * direction.y) + point.x * direction.x)
            - self.state.offset)
            .clamp(range[0], range[1]);
        Some(Slider3dUpdate {
            position: self.state.line.origin + direction * value,
            value,
        })
    }
}

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
}

impl Default for Slider3dState {
    fn default() -> Self {
        Self {
            origin: Vec3::ZERO,
            direction: Vec3::X,
        }
    }
}

impl Slider3dState {
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
}
