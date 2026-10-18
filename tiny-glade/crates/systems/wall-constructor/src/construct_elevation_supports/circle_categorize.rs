//! 圆形判定（construct_elevation_supports/circle_categorize.rs）。
//!
//! `circle_categorize`（0x14124af60，837B / 105 条指令，fresh 全量反汇编
//! `evidence/cc-dis.result.json`；常量逐字节 `evidence/cc-const.result.json`）。
//!
//! 已解完整控制流（与调用方 0x140788160 的 `extraout_AL` 用法一致，
//! 返回 bool：true=走 bottom_layer+pillars，false=走 turret）：
//!
//! ```text
//! radius = circle.radius
//! if 1.75 > radius:                // 0x142c20388 = 1.75
//!     return walls.is_empty()      // [R8+0x10] len == 0
//! t  = clamp((radius - 1.75) / 1.25, 0.0, 1.0)   // 0x142c2038c=-1.75, 0x142a8d600=1.25
//! s  = t*t*(3.0 - 2.0*t)                          // smoothstep (0x1429cd078=3.0)
//! threshold = s*0.6 + (1-s)*0.3                   // 0x142925908=0.6, 0x1429258d8=0.3
//! n  = ceil(2*radius) as usize                    // 0x142923e60=ceilf
//! step = (2*radius) / (n as f32)
//! inside = 0.0; in_wall = 0.0
//! for i in 0..=n:
//!     x = (circle.center.x - radius) + i as f32 * step
//!     for j in 0..=n:
//!         y = (circle.center.y - radius) + j as f32 * step
//!         if circle.is_point_inside((x,y)):       // 0x140c97b10
//!             inside += 1.0
//!             for wall in walls (stride 0x50, id @+0x38):
//!                 state = public_walls.get(WallId(id)).unwrap()   // 缺失 panic
//!                 shape = state.get_wall_shape()                   // 0x140b2a5d0
//!                 if shape_contains(shape, (x,y)):                 // 0x140ad2950
//!                     in_wall += 1.0
//!                     break
//! return in_wall / inside < threshold              // DIVSS + UCOMISS + SETC
//! ```
//!
//! `0x140ad2950`（`GladeBorder::is_pos_inside`）直接消费
//! `WallShapeSnapshot([u8;28])`：其字节即 `BorderShape` 的内存像
//! （tag u32@0；bit0==0 → `Circle2d{center@4,radius@0xc,rotation@0x10}`；
//! 否则 `Rectangle2d{axis@4,center@0xc,size@0x14}`），圆形走
//! `Circle2d::contains_point`，矩形走半轴投影测试。本实现用同一字节解释
//! 转换到 core 类型后调用 core 几何方法（未复制几何公式）。
//!
//! **登记（core-api-missing）**：建议 core 提供
//! `WallShapeSnapshot::contains_point(Vec2)`（或 `as_border_shape()`），
//! 落库后本文件的本地转换可删除。
use country_core::resources::walls::publication_fields::WallShapeSnapshot;
use country_core::resources::walls::public_walls::PublicWalls;
use country_core::resources::walls::WallId;
use glam::Vec2;
use utils::geometry::Circle2d;
use utils::geometry::Rectangle2d;

/// 墙条目（stride 0x50；机器只读 `+0x38` 的 wall id。字段其余语义 U）。
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct WallEntry50(pub [u8; 0x50]);

impl Default for WallEntry50 {
    fn default() -> Self {
        Self([0u8; 0x50])
    }
}

impl WallEntry50 {
    /// wall id @+0x38（机器 `MOV RDX,[R12+0x38]`）。
    pub fn wall_id(&self) -> WallId {
        WallId(u64::from_le_bytes(self.0[0x38..0x40].try_into().unwrap()))
    }
}

/// `WallShapeSnapshot` 字节 → core 几何查询（见模块注释；0x140ad2950 语义）。
fn shape_contains(shape: &WallShapeSnapshot, point: Vec2) -> bool {
    let b = shape.bytes();
    let f = |o: usize| f32::from_le_bytes(b[o..o + 4].try_into().unwrap());
    let tag = u32::from_le_bytes(b[0..4].try_into().unwrap());
    if tag & 1 == 0 {
        Circle2d {
            center: Vec2::new(f(4), f(8)),
            radius: f(0xc),
            rotation: f(0x10),
        }
        .contains_point(point)
    } else {
        Rectangle2d {
            axis: Vec2::new(f(4), f(8)),
            center: Vec2::new(f(0xc), f(0x10)),
            size: Vec2::new(f(0x14), f(0x18)),
        }
        .contains_point(point)
    }
}

/// x86 `MAXSS a, b`：`if a > b { a } else { b }`；unordered（NaN）返回 **b**。
#[inline]
fn maxss(a: f32, b: f32) -> f32 {
    if a > b {
        a
    } else {
        b
    }
}

/// x86 `MINSS a, b`：`if a < b { a } else { b }`；unordered（NaN）返回 **b**。
#[inline]
fn minss(a: f32, b: f32) -> f32 {
    if a < b {
        a
    } else {
        b
    }
}

/// B `circle_categorize`（0x14124af60）：圆形墙的支柱覆盖占比判定。
///
/// 机器实参：`RCX = &Circle2d`、`R8 = &墙条目容器`（+8 指针 / +0x10 长度，
/// 条目 stride 0x50）、`R9 = &PublicWalls`。返回
/// `in_wall / inside < threshold`（低半径早退返回 `walls.is_empty()`）。
///
/// NaN/∞ 语义按机器逐字保留（FIX-G1）：
/// - `t = MINSS(1.0, MAXSS(0.0, (r−1.75)/1.25))`——NaN radius → t=NaN
///   （Rust `f32::max/min` 会吞掉 NaN，故用手写 `maxss/minss`）。
/// - 末比较 = `UCOMISS ratio, threshold` + `SETC`：CF=1 当
///   `ratio < threshold` **或 unordered**（NaN → 返回 true）。
pub fn circle_categorize(
    circle: &Circle2d,
    walls: &[WallEntry50],
    public_walls: &PublicWalls,
) -> bool {
    let radius = circle.radius;
    if 1.75 > radius {
        return walls.is_empty();
    }
    let t_raw = (radius - 1.75) / 1.25;
    // 0x14124afc7 段：MAXSS XMM1(0.0), XMM0(t_raw)；MINSS XMM0(1.0), XMM1。
    let t = minss(1.0, maxss(0.0, t_raw));
    let smooth = (t * t) * (3.0 - 2.0 * t);
    let threshold = smooth * 0.6 + (1.0 - smooth) * 0.3;
    let n = (radius + radius).ceil() as usize;
    let step = (radius + radius) / (n as f32);
    let mut inside = 0.0f32;
    let mut in_wall = 0.0f32;
    for i in 0..=n {
        let x = (circle.center.x - radius) + (i as f32) * step;
        for j in 0..=n {
            let y = (circle.center.y - radius) + (j as f32) * step;
            let point = Vec2::new(x, y);
            if !circle.is_point_inside(point) {
                continue;
            }
            inside += 1.0;
            for wall in walls {
                let state = public_walls
                    .get(wall.wall_id())
                    .expect("circle_categorize: PublicWalls::get 缺失（原版 unwrap）");
                let shape = state.get_wall_shape();
                if shape_contains(shape, point) {
                    in_wall += 1.0;
                    break;
                }
            }
        }
    }
    let ratio = in_wall / inside;
    // 0x14124b24f `DIVSS` + `UCOMISS XMM10,XMM12` + `SETC AL`：
    // CF=1 ⇔ ratio < threshold 或 unordered（NaN）。
    ratio < threshold || ratio.is_nan() || threshold.is_nan()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn small_circle_returns_walls_empty() {
        let circle = Circle2d::new(Vec2::ZERO, 1.0);
        assert!(circle_categorize(&circle, &[], &PublicWalls::default()));
        let walls = [WallEntry50([0u8; 0x50])];
        assert!(!circle_categorize(&circle, &walls, &PublicWalls::default()));
    }

    #[test]
    fn nan_radius_matches_ucomiss_unordered() {
        // NaN radius：MAXSS/MINSS 传播 NaN → threshold=NaN；网格为空
        // （NaN as usize == 0）→ ratio=0/0=NaN；UCOMISS unordered → SETC=1。
        let circle = Circle2d::new(Vec2::ZERO, f32::NAN);
        assert!(circle_categorize(&circle, &[], &PublicWalls::default()));
    }

    #[test]
    fn wall_shape_snapshot_circle_and_rect() {
        let mut circle_bytes = [0u8; 28];
        circle_bytes[4..8].copy_from_slice(&1.0f32.to_le_bytes()); // center.x
        circle_bytes[0xc..0x10].copy_from_slice(&2.0f32.to_le_bytes()); // radius
        let c = WallShapeSnapshot::from_observed(circle_bytes);
        assert!(shape_contains(&c, Vec2::new(1.5, 0.0)));
        assert!(!shape_contains(&c, Vec2::new(3.5, 0.0)));

        let mut rect_bytes = [0u8; 28];
        rect_bytes[0] = 1; // tag bit0 = 1
        rect_bytes[4..8].copy_from_slice(&1.0f32.to_le_bytes()); // axis.x
        rect_bytes[0x14..0x18].copy_from_slice(&2.0f32.to_le_bytes()); // size.x
        rect_bytes[0x18..0x1c].copy_from_slice(&2.0f32.to_le_bytes()); // size.y
        let r = WallShapeSnapshot::from_observed(rect_bytes);
        assert!(shape_contains(&r, Vec2::new(0.5, 0.5)));
        assert!(!shape_contains(&r, Vec2::new(2.0, 0.0)));
    }
}
