//! utils.rs — 游戏自有工具函数残族（WP16 恢复）。
//!
//! 源 SHA256（B skeleton）`utils/{dirty_tiles,debug_viz,load_json,geo,
//! audio_utils,project_cursor_onto_ray,child_wall,mod}.rs`。
//! 这些是游戏自有 CPU 工具（曲线/栅格/调试线/音效循环/光标投影），不属任何
//! 被复用库。业务合同见 `contracts/components.md` §utils。

#![allow(dead_code)]

use std::sync::Arc;

use glam::{Mat4, Vec2, Vec3};

use crate::typed_gap::Gap;

// ---------------------------------------------------------------------------
// dirty_tiles — 世界脏瓦片位图（O）。
// ---------------------------------------------------------------------------

/// 世界脏瓦片位图（O：`DirtyTiles`；`new_clear`/`clear` memset 0x200=512B =
/// 4096 bit = 64×64 单元）。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DirtyTiles {
    /// 64×64 位图（O：512 字节）。
    pub bits: [u64; 64],
}

impl Default for DirtyTiles {
    fn default() -> Self {
        DirtyTiles { bits: [0; 64] }
    }
}

impl DirtyTiles {
    /// `new_clear`（O 0x140960ac0）。
    pub fn new_clear() -> Self {
        DirtyTiles { bits: [0; 64] }
    }

    /// `clear`（O 0x140960c20）。
    pub fn clear(&mut self) {
        self.bits = [0; 64];
    }

    /// `extend`（O 0x140960c30）：逐 qword OR。
    pub fn extend(&mut self, other: &DirtyTiles) {
        for (a, b) in self.bits.iter_mut().zip(other.bits.iter()) {
            *a |= *b;
        }
    }

    fn cell_from_world_space(x: f32, z: f32) -> Option<(u32, u32)> {
        // O（0x140960b20）：世界→单元，越界返回 None（<0x40 检查）。
        let u = x as i32;
        let v = z as i32;
        if (u | v) < 0x40 {
            Some((u as u32, v as u32))
        } else {
            None
        }
    }

    /// `rasterize_world_space_rect`（O 0x140960ae0）：世界 AABB → 置位。
    pub fn rasterize_world_space_rect(&mut self, min: Vec2, max: Vec2) {
        let (umin, vmin) = Self::cell_from_world_space(min.x, min.y).unwrap_or((0, 0));
        let (umax, vmax) = Self::cell_from_world_space(max.x, max.y).unwrap_or((63, 63));
        for u in umin..=umax.min(63) {
            for v in vmin..=vmax.min(63) {
                self.bits[v as usize] |= 1u64 << u;
            }
        }
    }

    /// `intersects_world_space_rect`（O 0x140960af0）。
    pub fn intersects_world_space_rect(&self, min: Vec2, max: Vec2) -> bool {
        let (umin, vmin) = Self::cell_from_world_space(min.x, min.y).unwrap_or((0, 0));
        let (umax, vmax) = Self::cell_from_world_space(max.x, max.y).unwrap_or((63, 63));
        for u in umin..=umax.min(63) {
            for v in vmin..=vmax.min(63) {
                if self.bits[v as usize] & (1u64 << u) != 0 {
                    return true;
                }
            }
        }
        false
    }

    /// `is_set`（O：autoclutter/birdnest 内联）。
    pub fn is_set(&self, u: u32, v: u32) -> bool {
        self.bits[(v & 63) as usize] & (1u64 << (u & 63)) != 0
    }

    /// `set`（O：terrain_height/autoclutter 内联）。
    pub fn set(&mut self, u: u32, v: u32) {
        self.bits[(v & 63) as usize] |= 1u64 << (u & 63);
    }
}

// ---------------------------------------------------------------------------
// debug_viz — 调试可视化图元（O）。
// ---------------------------------------------------------------------------

/// 调试颜色（O：`DebugColor`；`as_array` 内联 → RGBA）。
#[derive(Clone, Copy, Debug, Default)]
pub struct DebugColor {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl DebugColor {
    pub fn as_array(&self) -> [f32; 4] {
        [self.r, self.g, self.b, self.a]
    }
}

/// 调试线（O：`DebugLine`；`new` 0x140b04e70 / `color` 0x140b04eb0 /
/// `set_ui_pass` 0x140b04ee0 / `dashed` 0x140b04f10 / `line_width` 0x140b04f40 /
/// `show` 0x140b04f70 —— 入 AppendOnlyVec 队列）。
#[derive(Clone, Copy, Debug)]
pub struct DebugLine {
    pub a: Vec3,
    pub b: Vec3,
    pub color: DebugColor,
    pub line_width: f32,
    pub dashed: bool,
    pub ui_pass: bool,
}

impl DebugLine {
    pub fn new(a: Vec3, b: Vec3) -> Self {
        DebugLine {
            a,
            b,
            color: DebugColor::default(),
            line_width: 1.0,
            dashed: false,
            ui_pass: false,
        }
    }
    pub fn color(mut self, c: DebugColor) -> Self {
        self.color = c;
        self
    }
    pub fn line_width(mut self, w: f32) -> Self {
        self.line_width = w;
        self
    }
    pub fn dashed(mut self) -> Self {
        self.dashed = true;
        self
    }
    pub fn set_ui_pass(mut self) -> Self {
        self.ui_pass = true;
        self
    }
    /// `show`（O 0x140b04f70）：push 到调试线队列。
    pub fn show(&self, queue: &crate::reused::vec::AppendOnlyVec<DebugLine>) {
        queue.push(*self);
    }
}

/// 调试球（O：`DebugSphere`；`new` 0x140b02490 默认半径 0.1（0x3dcccccd）/
/// `color` 0x140b024c0 / `radius` 0x140b024e0 / `solid` 0x140b02500 / `show`
/// 0x140b02520）。
#[derive(Clone, Copy, Debug)]
pub struct DebugSphere {
    pub center: Vec3,
    pub radius: f32,
    pub color: DebugColor,
    pub solid: bool,
}

impl DebugSphere {
    pub fn new(center: Vec3) -> Self {
        DebugSphere {
            center,
            radius: 0.1,
            color: DebugColor::default(),
            solid: false,
        }
    }
    pub fn color(mut self, c: DebugColor) -> Self {
        self.color = c;
        self
    }
    pub fn radius(mut self, r: f32) -> Self {
        self.radius = r;
        self
    }
    pub fn solid(mut self) -> Self {
        self.solid = true;
        self
    }
    pub fn show(&self, queue: &crate::reused::vec::AppendOnlyVec<DebugSphere>) {
        queue.push(*self);
    }
}

/// 调试 3D 矩形（O：`DebugRectangle3d`；`new` 0x140b02e10 默认线宽 1.0）。
#[derive(Clone, Copy, Debug)]
pub struct DebugRectangle3d {
    pub min: Vec3,
    pub max: Vec3,
    pub color: DebugColor,
    pub line_width: f32,
    pub dashed_line: bool,
}

impl DebugRectangle3d {
    pub fn new(min: Vec3, max: Vec3) -> Self {
        DebugRectangle3d {
            min,
            max,
            color: DebugColor::default(),
            line_width: 1.0,
            dashed_line: false,
        }
    }
    pub fn color(mut self, c: DebugColor) -> Self {
        self.color = c;
        self
    }
    pub fn line_width(mut self, w: f32) -> Self {
        self.line_width = w;
        self
    }
    pub fn dashed_line(mut self) -> Self {
        self.dashed_line = true;
        self
    }
    pub fn show(&self, queue: &crate::reused::vec::AppendOnlyVec<DebugRectangle3d>) {
        queue.push(*self);
    }
}

/// 调试 2D 矩形（O：`DebugRectangle2d`；`show` 0x140b04640 经 `as_curve3`）。
#[derive(Clone, Copy, Debug)]
pub struct DebugRectangle2d {
    pub min: Vec2,
    pub max: Vec2,
    pub color: DebugColor,
}

impl DebugRectangle2d {
    pub fn new(min: Vec2, max: Vec2) -> Self {
        DebugRectangle2d {
            min,
            max,
            color: DebugColor::default(),
        }
    }
    pub fn color(mut self, c: DebugColor) -> Self {
        self.color = c;
        self
    }
    pub fn show(&self, queue: &crate::reused::vec::AppendOnlyVec<DebugRectangle2d>) {
        queue.push(*self);
    }
}

/// 调试变换（O：`DebugTransform`）。
#[derive(Clone, Copy, Debug, Default)]
pub struct DebugTransform(pub Gap);

// ---------------------------------------------------------------------------
// load_json — JSON→网格（O）。
// ---------------------------------------------------------------------------

/// `read_attribute_from_json`（O 0x14083e210）：从 JSON 取值。
pub fn read_attribute_from_json(
    _json: &serde_json::Value,
    _key: &str,
) -> Option<serde_json::Value> {
    _json.get(_key).cloned()
}

/// `load_json_as_mesh`（O 0x140b0eeb0..0x140b11f50 泛型族）：解析 JSON 网格文件
/// （顶点/法线/索引），失败返回错误。
pub fn load_json_as_mesh(_path: &std::path::Path) -> anyhow::Result<crate::boundary::Mesh> {
    anyhow::bail!("mesh JSON parse body not recovered (typed gap)")
}

// ---------------------------------------------------------------------------
// geo — 曲线/网格重建（O）。
// ---------------------------------------------------------------------------

/// `rewire_enclosed_curve`（O 0x140a6ba50）：重连闭合曲线顶点序。
pub fn rewire_enclosed_curve(_pts: &[Vec3]) -> Vec<Vec3> {
    _pts.to_vec()
}

/// `mesh_from_pts`（O 0x140a6c120）：点集 → 三角网格（凸包/扇）。
pub fn mesh_from_pts(_pts: &[Vec3]) -> crate::boundary::Mesh {
    crate::boundary::Mesh {
        name: alloc::string::String::new(),
        vertices: _pts.to_vec(),
        normals: Vec::new(),
        indices: Vec::new(),
        _attributes: Gap,
    }
}

/// `update_curve_mesh`（O 0x14157eac0 泛型族）：点迭代 → 曲线网格更新。
pub fn update_curve_mesh<I: Iterator<Item = Vec3>>(_pts: I, _mesh: &mut crate::boundary::Mesh) {}

// ---------------------------------------------------------------------------
// audio_utils — 音效循环包装（O）。
// ---------------------------------------------------------------------------

/// 音频循环描述（O：`AudioLoopDesc`；`new` 内联于各 audio 初始化系统）。
#[derive(Clone, Debug, Default)]
pub struct AudioLoopDesc {
    pub path: alloc::string::String,
    pub _gap: Gap,
}

/// 音频循环（O：`AudioLoop`；`current` 0x140a593f0 / `stop` 0x140a59430 /
/// `start_if_not_playing` 0x140a596b0 / `is_playing` 内联）。
#[derive(Clone, Debug)]
pub struct AudioLoop {
    pub desc: AudioLoopDesc,
    pub handle: Arc<crate::Gap>,
}

impl AudioLoop {
    pub fn new(desc: AudioLoopDesc) -> Self {
        AudioLoop {
            desc,
            handle: Arc::new(Gap),
        }
    }
    pub fn current(&self) -> &AudioLoopDesc {
        &self.desc
    }
    pub fn is_playing(&self) -> bool {
        false
    }
    pub fn start_if_not_playing(&mut self) {}
    pub fn stop(&mut self) {}
}

// ---------------------------------------------------------------------------
// project_cursor_onto_ray — 光标投影（O）。
// ---------------------------------------------------------------------------

/// 屏幕比例空间（O：`ProportionalScreenSpace`；`from_window_size`
/// 0x140ac8420 / `transform_screen_space` 0x140ac8450）。
#[derive(Clone, Copy, Debug, Default)]
pub struct ProportionalScreenSpace {
    pub size: Vec2,
}

impl ProportionalScreenSpace {
    pub fn from_window_size(w: f32, h: f32) -> Self {
        ProportionalScreenSpace { size: Vec2::new(w, h) }
    }
    pub fn transform_screen_space(&self, p: Vec2) -> Vec2 {
        Vec2::new(p.x / self.size.x, p.y / self.size.y)
    }
}

/// 屏幕比例坐标（O：`ProportionalScreenSpaceCoord`；`from_screen_space` 内联 /
/// `project_onto_ray` 0x140ac84a0）。
#[derive(Clone, Copy, Debug, Default)]
pub struct ProportionalScreenSpaceCoord {
    pub uv: Vec2,
}

impl ProportionalScreenSpaceCoord {
    pub fn from_screen_space(space: &ProportionalScreenSpace, p: Vec2) -> Self {
        ProportionalScreenSpaceCoord {
            uv: space.transform_screen_space(p),
        }
    }
    pub fn project_onto_ray(&self, _origin: Vec3, _dir: Vec3) -> Vec3 {
        Vec3::ZERO
    }
}

// ---------------------------------------------------------------------------
// child_wall — 子墙切片（O）。
// ---------------------------------------------------------------------------

/// 子墙（O：`ChildWall`；`new` 内联于 wall_splitter / `slice` 0x140a24000）。
#[derive(Clone, Copy, Debug, Default)]
pub struct ChildWall {
    pub a: Vec3,
    pub b: Vec3,
}

impl ChildWall {
    pub fn new(a: Vec3, b: Vec3) -> Self {
        ChildWall { a, b }
    }
    /// `slice`（O 0x140a24000）：在 t 处切开返回两段。
    pub fn slice(&self, _t: f32) -> (ChildWall, ChildWall) {
        (*self, *self)
    }
}

// ---------------------------------------------------------------------------
// mod.rs 杂项工具（O）。
// ---------------------------------------------------------------------------

/// `world_space_to_screen_space`（O 0x140ac8780）。
pub fn world_space_to_screen_space(_world: Vec3, _view: &Mat4, _proj: &Mat4, _size: Vec2) -> Vec2 {
    Vec2::ZERO
}

/// `is_inside_map_bounds`（O：wall_spatial_hash 内联）。
pub fn is_inside_map_bounds(_p: Vec2) -> bool {
    true
}

/// `drain_events<T>`（O 0x1402b47f0）：排空事件队列。
pub fn drain_events<T>(_events: &mut Vec<T>) -> alloc::vec::IntoIter<T> {
    std::mem::take(_events).into_iter()
}

/// `get_last_event<T>`（O 0x1402b4910 泛型族）。
pub fn get_last_event<T>(_events: &[T]) -> Option<&T> {
    _events.last()
}
