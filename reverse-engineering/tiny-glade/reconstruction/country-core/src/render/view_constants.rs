//! `ViewConstants` / `ViewportConstantBuilder` — per-view GPU constants (layer A1).
//!
//! 全部布局与数学来自原函数的指令级反编译，不再是推断：
//!  * `ViewportConstantBuilder::build`      0x140b18f60 (2339 B)  — builder(rdx) → ViewConstants(rcx, sret)
//!  * `ViewConstants::set_pixel_offset`     0x140b18af0 (1131 B)  — 就地重算抖动矩阵
//! 复核方法见 `evidence/country-core-audit/L0_NOTES.md` §view_constants。
//!
//! 标注：**O**＝反汇编/反编译中的字节偏移与运算；**I**＝字段命名（含义由数学关系推出，名称为我方所取）。
//!
//! 约定（O，glam 列主序）：矩阵 `m * v` 的第 k 列 = Σ_j v[k][j]·m.col_j。

use glam::{Mat4, UVec2, Vec2, Vec4};

/// 0x320 字节的逐视图常量块（O：`build` 写到 +0x31c，`size_of == 0x320`）。
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ViewConstants {
    /// +0x000 投影（未抖动）。O：`build` 自 builder+0x000 复制。
    pub clip_from_view: Mat4,
    /// +0x040 投影逆（未抖动）。O：自 builder+0x040 复制。
    pub view_from_clip: Mat4,
    /// +0x080 抖动后的投影：`T(-jitter_ndc) * clip_from_view`。O。
    pub clip_from_view_jittered: Mat4,
    /// +0x0c0 抖动后的投影逆：`view_from_clip * T(+jitter_ndc)`。O。
    pub view_from_clip_jittered: Mat4,
    /// +0x100 视图矩阵。O：自 builder+0x080 复制（I：命名）。
    pub view_from_world: Mat4,
    /// +0x140 视图逆。O：自 builder+0x0c0 复制。
    pub world_from_view: Mat4,
    /// +0x180 抖动后的 view-proj：`clip_from_view_jittered * view_from_world`。O。
    pub clip_from_world_jittered: Mat4,
    /// +0x1c0 重投影：`prev_clip_from_view * prev_view_from_world * world_from_view * view_from_clip`
    /// （当前裁剪空间 → 上一帧裁剪空间，**不含**抖动）。O（乘法链逐项对过）。
    pub prev_clip_from_clip: Mat4,
    /// +0x200 上一帧投影。O：自 builder+0x100 复制。
    pub prev_clip_from_view: Mat4,
    /// +0x240 上一帧投影逆。O：自 builder+0x140 复制。
    pub prev_view_from_clip: Mat4,
    /// +0x280 上一帧视图。O：自 builder+0x180 复制。
    pub prev_view_from_world: Mat4,
    /// +0x2c0 上一帧视图逆。O：自 builder+0x1c0 复制。
    pub prev_world_from_view: Mat4,
    /// +0x300 像素偏移（亚像素抖动，单位像素）。O。
    pub pixel_offset: Vec2,
    /// +0x308 NDC 抖动 = `pixel_offset * 2 / viewport`（**无** y 翻转）。O。
    pub jitter_ndc: Vec2,
    /// +0x310 上一帧像素偏移。O。
    pub prev_pixel_offset: Vec2,
    /// +0x318 上一帧 NDC 抖动。O。
    pub prev_jitter_ndc: Vec2,
}

const _: () = assert!(core::mem::size_of::<ViewConstants>() == 0x320);

/// `col + col.w * (-jx, -jy, 0, 0)`，即 `T(-j) * M` 的单列（O：build 与 set_pixel_offset 同形）。
#[inline]
fn jitter_column(c: Vec4, j: Vec2) -> Vec4 {
    Vec4::new(c.w * -j.x + c.x, c.w * -j.y + c.y, c.z, c.w)
}

/// `T(-j) * M`：每一列的 xy 减去 `w * j`（O）。
fn jitter_clip_from_view(m: &Mat4, j: Vec2) -> Mat4 {
    Mat4::from_cols(
        jitter_column(m.x_axis, j),
        jitter_column(m.y_axis, j),
        jitter_column(m.z_axis, j),
        jitter_column(m.w_axis, j),
    )
}

/// `M * T(+j)`：只有第 3 列变化，`col3 += jx*col0 + jy*col1`（O）。
fn jitter_view_from_clip(m: &Mat4, j: Vec2) -> Mat4 {
    let col3 = m.y_axis * j.y + m.x_axis * j.x + m.w_axis;
    Mat4::from_cols(m.x_axis, m.y_axis, m.z_axis, col3)
}

/// `2 * offset / viewport`（O：`addps` 后 `divps`，viewport 先 `u32→f64→f32`）。
#[inline]
fn offset_to_ndc(offset: Vec2, viewport: UVec2) -> Vec2 {
    let vp = Vec2::new(viewport.x as f64 as f32, viewport.y as f64 as f32);
    (offset + offset) / vp
}

impl ViewConstants {
    /// `set_pixel_offset` (O: 0x140b18af0)。就地更新抖动相关字段：
    /// 参数（Win64 ABI 逐项对过）= `(self=rcx, offset.x=xmm1, offset.y=xmm2, prev.x=xmm3,
    /// prev.y=[rsp+0x150], vw=[rsp+0x158] u32, vh=[rsp+0x160] u32)`。
    ///
    /// 写入：+0x300 offset、+0x308 jitter_ndc、+0x310 prev_offset、+0x318 prev_jitter_ndc、
    /// +0x080 / +0x0c0 / +0x180。未改写：未抖动矩阵与 +0x1c0 重投影矩阵（O：函数里没有对应存储）。
    pub fn set_pixel_offset(&mut self, offset: Vec2, prev_offset: Vec2, viewport: UVec2) {
        let j = offset_to_ndc(offset, viewport);
        self.pixel_offset = offset;
        self.jitter_ndc = j;
        self.clip_from_view_jittered = jitter_clip_from_view(&self.clip_from_view, j);
        self.view_from_clip_jittered = jitter_view_from_clip(&self.view_from_clip, j);
        self.clip_from_world_jittered = self.clip_from_view_jittered * self.view_from_world;
        self.prev_pixel_offset = prev_offset;
        self.prev_jitter_ndc = offset_to_ndc(prev_offset, viewport);
    }
}

/// 构造 `ViewConstants` 所需的原始矩阵（O：`build` 读 builder 的 +0x000…+0x217）。
/// 布局（O）：8 个矩阵各 0x40 字节，随后 `viewport: UVec2 @0x200`、`pixel_offset @0x208`、
/// `prev_pixel_offset @0x210`；`size_of == 0x220`（16 对齐）。
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ViewportConstantBuilder {
    /// +0x000
    pub clip_from_view: Mat4,
    /// +0x040
    pub view_from_clip: Mat4,
    /// +0x080（I：命名为视图矩阵，依据 build 里 `clip_from_world_jittered = Aj * 此）
    pub view_from_world: Mat4,
    /// +0x0c0
    pub world_from_view: Mat4,
    /// +0x100
    pub prev_clip_from_view: Mat4,
    /// +0x140
    pub prev_view_from_clip: Mat4,
    /// +0x180
    pub prev_view_from_world: Mat4,
    /// +0x1c0
    pub prev_world_from_view: Mat4,
    /// +0x200
    pub viewport: UVec2,
    /// +0x208
    pub pixel_offset: Vec2,
    /// +0x210
    pub prev_pixel_offset: Vec2,
}

const _: () = assert!(core::mem::size_of::<ViewportConstantBuilder>() == 0x220);

impl ViewportConstantBuilder {
    /// `build` (O: 0x140b18f60)。逐字段对应见 `ViewConstants` 的注释。
    pub fn build(&self) -> ViewConstants {
        let jitter = offset_to_ndc(self.pixel_offset, self.viewport);
        let clip_from_view_jittered = jitter_clip_from_view(&self.clip_from_view, jitter);
        let view_from_clip_jittered = jitter_view_from_clip(&self.view_from_clip, jitter);
        // O：乘法链按 ((E*G)*D)*B 求值（反编译里 fVar67… → fVar49… → 输出 +0x1c0）。
        let prev_clip_from_clip =
            ((self.prev_clip_from_view * self.prev_view_from_world) * self.world_from_view)
                * self.view_from_clip;
        ViewConstants {
            clip_from_view: self.clip_from_view,
            view_from_clip: self.view_from_clip,
            clip_from_view_jittered,
            view_from_clip_jittered,
            view_from_world: self.view_from_world,
            world_from_view: self.world_from_view,
            clip_from_world_jittered: clip_from_view_jittered * self.view_from_world,
            prev_clip_from_clip,
            prev_clip_from_view: self.prev_clip_from_view,
            prev_view_from_clip: self.prev_view_from_clip,
            prev_view_from_world: self.prev_view_from_world,
            prev_world_from_view: self.prev_world_from_view,
            pixel_offset: self.pixel_offset,
            jitter_ndc: jitter,
            prev_pixel_offset: self.prev_pixel_offset,
            prev_jitter_ndc: offset_to_ndc(self.prev_pixel_offset, self.viewport),
        }
    }
}

impl Default for ViewConstants {
    /// U：原版默认值来自 `core::array::impl$56::default<ViewConstants>`（0x1400ef490，2159 B），
    /// 尚未逐项核对；暂取单位矩阵 / 0（占位，非 O）。
    fn default() -> Self {
        Self {
            clip_from_view: Mat4::IDENTITY,
            view_from_clip: Mat4::IDENTITY,
            clip_from_view_jittered: Mat4::IDENTITY,
            view_from_clip_jittered: Mat4::IDENTITY,
            view_from_world: Mat4::IDENTITY,
            world_from_view: Mat4::IDENTITY,
            clip_from_world_jittered: Mat4::IDENTITY,
            prev_clip_from_clip: Mat4::IDENTITY,
            prev_clip_from_view: Mat4::IDENTITY,
            prev_view_from_clip: Mat4::IDENTITY,
            prev_view_from_world: Mat4::IDENTITY,
            prev_world_from_view: Mat4::IDENTITY,
            pixel_offset: Vec2::ZERO,
            jitter_ndc: Vec2::ZERO,
            prev_pixel_offset: Vec2::ZERO,
            prev_jitter_ndc: Vec2::ZERO,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use glam::{Vec3, Vec4Swizzles};

    fn sample_builder(offset: Vec2, prev: Vec2) -> ViewportConstantBuilder {
        let proj = Mat4::perspective_rh(1.1, 16.0 / 9.0, 0.1, 500.0);
        let view = Mat4::look_at_rh(Vec3::new(3.0, 5.0, 8.0), Vec3::new(0.0, 1.0, 0.0), Vec3::Y);
        let pproj = Mat4::perspective_rh(1.1, 16.0 / 9.0, 0.1, 500.0);
        let pview = Mat4::look_at_rh(Vec3::new(2.5, 5.0, 8.2), Vec3::new(0.0, 1.0, 0.0), Vec3::Y);
        ViewportConstantBuilder {
            clip_from_view: proj,
            view_from_clip: proj.inverse(),
            view_from_world: view,
            world_from_view: view.inverse(),
            prev_clip_from_view: pproj,
            prev_view_from_clip: pproj.inverse(),
            prev_view_from_world: pview,
            prev_world_from_view: pview.inverse(),
            viewport: UVec2::new(1920, 1080),
            pixel_offset: offset,
            prev_pixel_offset: prev,
        }
    }

    fn close(a: &Mat4, b: &Mat4, eps: f32) -> bool {
        a.to_cols_array().iter().zip(b.to_cols_array().iter()).all(|(x, y)| (x - y).abs() <= eps)
    }

    #[test]
    fn layout_matches_original() {
        assert_eq!(core::mem::size_of::<ViewConstants>(), 0x320);
        assert_eq!(core::mem::offset_of!(ViewConstants, clip_from_view_jittered), 0x80);
        assert_eq!(core::mem::offset_of!(ViewConstants, view_from_clip_jittered), 0xc0);
        assert_eq!(core::mem::offset_of!(ViewConstants, clip_from_world_jittered), 0x180);
        assert_eq!(core::mem::offset_of!(ViewConstants, prev_clip_from_clip), 0x1c0);
        assert_eq!(core::mem::offset_of!(ViewConstants, pixel_offset), 0x300);
        assert_eq!(core::mem::offset_of!(ViewConstants, prev_jitter_ndc), 0x318);
        assert_eq!(core::mem::offset_of!(ViewportConstantBuilder, viewport), 0x200);
        assert_eq!(core::mem::offset_of!(ViewportConstantBuilder, prev_pixel_offset), 0x210);
    }

    /// 两个独立反编译出来的函数必须给出同一结果：build(offset) ≡ build(0) + set_pixel_offset(offset)。
    #[test]
    fn set_pixel_offset_agrees_with_build() {
        let off = Vec2::new(0.37, -0.21);
        let prev = Vec2::new(-0.11, 0.43);
        let direct = sample_builder(off, prev).build();
        let mut incr = sample_builder(Vec2::ZERO, Vec2::ZERO).build();
        incr.set_pixel_offset(off, prev, UVec2::new(1920, 1080));
        assert_eq!(direct, incr);
    }

    /// 抖动语义：任意点经抖动投影后的 NDC xy = 未抖动 NDC xy − jitter_ndc。
    #[test]
    fn jittered_projection_shifts_ndc_by_jitter() {
        let c = sample_builder(Vec2::new(0.5, -0.25), Vec2::ZERO).build();
        let p = Vec4::new(1.0, 2.0, -10.0, 1.0);
        let a = c.clip_from_view * p;
        let b = c.clip_from_view_jittered * p;
        let (na, nb) = (a.xy() / a.w, b.xy() / b.w);
        assert!((na - nb - c.jitter_ndc).abs().max_element() < 1e-5);
        assert!((c.jitter_ndc - Vec2::new(0.5 * 2.0 / 1920.0, -0.25 * 2.0 / 1080.0)).abs().max_element() < 1e-9);
    }

    /// view_from_clip_jittered 是 clip_from_view_jittered 的逆。
    #[test]
    fn jittered_inverse_is_inverse() {
        let c = sample_builder(Vec2::new(0.4, 0.3), Vec2::ZERO).build();
        assert!(close(&(c.clip_from_view_jittered * c.view_from_clip_jittered), &Mat4::IDENTITY, 1e-4));
    }

    /// 静止相机（当前=上一帧）时重投影矩阵应为单位阵。
    #[test]
    fn reprojection_identity_when_static() {
        let mut b = sample_builder(Vec2::ZERO, Vec2::ZERO);
        b.prev_clip_from_view = b.clip_from_view;
        b.prev_view_from_clip = b.view_from_clip;
        b.prev_view_from_world = b.view_from_world;
        b.prev_world_from_view = b.world_from_view;
        assert!(close(&b.build().prev_clip_from_clip, &Mat4::IDENTITY, 1e-4));
    }

    #[test]
    fn clip_from_world_jittered_is_product() {
        let c = sample_builder(Vec2::new(0.2, 0.1), Vec2::ZERO).build();
        assert_eq!(c.clip_from_world_jittered, c.clip_from_view_jittered * c.view_from_world);
    }
}
