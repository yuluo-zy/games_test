//! draw_list.rs — `DrawListItem` / `EntityDrawParams`（CPU draw-list 项）。
//!
//! 本文件没有独立的 PDB 过程（`DrawListItem::to_draw_packet` 内联进各宿主 pass）。机器码证据：
//! - O 排序实例类型 `driftsort_main<tuple<DrawListItem, fn(DrawBuilder, &EntityDrawParams, &mut World)>>`：列表元素是
//!   `(DrawListItem, fn)` **元组**，0x80 B（`DrawListItem` 0x70 B + 回调指针 @+0x70）。
//! - O 回调的第二个实参 `&EntityDrawParams` 就是**元素指针本身**（`rdx = element`；`[rdx+8]` = 实体，见
//!   draw_bricks.rs / stairs_draw.rs），所以 `EntityDrawParams` 是 `DrawListItem` 的**首字段**（偏移 0，0x50 B）。
//! - O `build_draw_lists`（0x1409ba540 +0x2c9..）逐字段写入一个元素（`0x1409bc8f0..0x1409bc97c`）：
//!
//! | 偏移 | 内容 | 来源（O） |
//! |---|---|---|
//! | +0x00 | `Option<DrawTopology>`：tag u32（`None` = 2）| `Option<&DrawTopology>`，`None` → `mov [..],2` |
//! | +0x04 | 载荷 f32（`xmm15`，仅 `Some` 时有意义）| `movss xmm15,[r15+4]` |
//! | +0x08 | `Entity`（u64）| 查询项 0 |
//! | +0x10/+0x18 | 着色器名 `&'static str` | `AssetShaderLibrary::debug_raster_get_name` |
//! | +0x20 | 旋转 Quat（16 B）| `Transform` 的 0x30 B 副本（3×`movaps`）|
//! | +0x30 | 平移 Vec3 | 同上 |
//! | +0x3c | 缩放 Vec3 | 同上 |
//! | +0x50 | `Handle<Vao>` 的 u64（= `DrawPacket::head[1]`）| `*[vao_ref]` |
//! | +0x58 | `Handle<RasterShaderProgram>` 的 u64 | `*[shader_ref]` |
//! | +0x60 | 排序键 i32 | 着色器名分发表的"绘制序"常量（如 `floor_shader` = 31000）|
//! | +0x64 | `Option<u8>` tag | 模板参考值（`kind` 决定，见 `draw_lists.rs`）|
//! | +0x65 | 模板参考值 u8 | 同上 |
//! | +0x70 | 回调 fn（元组第二项）| 分发表 |
//!
//! `DrawTopology` 的变体名 U：机器码只有 tag（0/1）+ f32 载荷；`ui_2d` / vertex-color 回调（0x1409b8340）对
//! `tag == 1` 取载荷 f32 并把 `state.byte0 = (b & 0xfc) | 2`，其它（含 `None`）用 builder 自带的默认 f32。

use alloc::vec::Vec;
use bevy_ecs::entity::Entity;
use bevy_ecs::world::World;

use crate::render::pass_builder::draw_builder::{DrawBuilder, DrawTransform};
use crate::render::pass_builder::draw_stream::{DrawPacket, RasterState};
use crate::render::shader::RasterShaderProgram;
use crate::rhapsody_contract::{Handle, ManagedHandle};

/// `DrawTopology` 组件（O：tag u32 ∈ {0,1} + f32 载荷；变体名 U，下面按 `tag==1` 的用法起名，I）。
#[derive(bevy_ecs::component::Component, Clone, Copy, Debug, PartialEq)]
pub enum DrawTopology {
    /// tag 0（载荷 f32）。
    Default(f32),
    /// tag 1：回调里把 `state.byte0 = (b & 0xfc) | 2` 并改用此 f32。
    Alt(f32),
}

impl DrawTopology {
    /// 元素里的 tag（`+0x00` u32）。
    pub fn tag(&self) -> u32 {
        match self {
            DrawTopology::Default(_) => 0,
            DrawTopology::Alt(_) => 1,
        }
    }
    /// 元素里的载荷（`+0x04` f32）。
    pub fn payload(&self) -> f32 {
        match *self {
            DrawTopology::Default(v) | DrawTopology::Alt(v) => v,
        }
    }
}

/// 回调看到的实体绘制参数（元素的前 0x50 B）。
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EntityDrawParams {
    /// `+0x00`：`Option<DrawTopology>`（`None` = tag 2）。
    pub topology: Option<DrawTopology>,
    /// `+0x08`：实体。
    pub entity: Entity,
    /// `+0x10`：着色器名（`debug_raster_get_name`）。
    pub shader_name: &'static str,
    /// `+0x20/+0x30/+0x3c`：`bind_common` 读取的变换。
    pub transform: DrawTransform,
}

/// 回调签名（O：`fn(DrawBuilder, &EntityDrawParams, &mut World)`）。
pub type DrawListFn = for<'p> fn(DrawBuilder<'p>, &EntityDrawParams, &mut World);

/// draw-list 项（0x70 B）。
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DrawListItem {
    /// `+0x00..0x50`。
    pub params: EntityDrawParams,
    /// `+0x50`：`Handle<Vao>`（原始 u64）；`DrawPacket::head = [1, vao]`。
    pub vao: u64,
    /// `+0x58`：`Handle<RasterShaderProgram>`（原始 u64）。
    pub shader: u64,
    /// `+0x60`：排序键（有符号，稳定排序）。
    pub sort_key: i32,
    /// `+0x64/+0x65`：`Option<u8>` 模板参考值（`None` → 读取方 `Option::unwrap` panic）。
    pub stencil_ref: Option<u8>,
}

impl DrawListItem {
    /// 只带着色器/排序键的元素（测试与构造用，变换为单位变换，实体 PLACEHOLDER）。
    pub fn new(shader: u64, sort_key: i32) -> DrawListItem {
        DrawListItem {
            params: EntityDrawParams {
                topology: None,
                entity: Entity::PLACEHOLDER,
                shader_name: "",
                transform: DrawTransform {
                    rotation: glam::Quat::IDENTITY,
                    translation: glam::Vec3::ZERO,
                    scale: glam::Vec3::ONE,
                },
            },
            vao: 0,
            shader,
            sort_key,
            stencil_ref: None,
        }
    }

    /// `DrawPacket::new(shader = +0x58, head = [1, +0x50], default_state)`（O：各 pass 的内联序列，`r8 = 1`、`r9 = +0x50`）。
    pub fn to_draw_packet(&self, default_state: RasterState) -> DrawPacket {
        DrawPacket::new(
            Handle(
                ManagedHandle(self.shader),
                core::marker::PhantomData::<fn() -> RasterShaderProgram>,
            ),
            [1, self.vao],
            default_state,
        )
    }
}

/// 列表元素（O：元组）。
pub type DrawListEntry = (DrawListItem, DrawListFn);

/// 稳定排序（O：`DrawLists::sort` = `sort_by_key(|e| e.0.sort_key)`；键 = i32 @ 元素 `+0x60`，有符号比较）。
pub fn sort_draw_list(list: &mut Vec<DrawListEntry>) {
    list.sort_by_key(|e| e.0.sort_key);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn noop(_b: DrawBuilder<'_>, _p: &EntityDrawParams, _w: &mut World) {}

    #[test]
    fn sort_is_stable_on_the_signed_i32_key() {
        let f: DrawListFn = noop;
        let mut v: Vec<DrawListEntry> = alloc::vec![
            (DrawListItem::new(1, 2), f),
            (DrawListItem::new(9, -1), f),
            (DrawListItem::new(7, 2), f),
        ];
        sort_draw_list(&mut v);
        assert_eq!(v.iter().map(|e| e.0.shader).collect::<Vec<_>>(), alloc::vec![9, 1, 7]);
    }

    #[test]
    fn to_draw_packet_uses_head_one_vao_and_the_item_shader() {
        let mut it = DrawListItem::new(5, 0);
        it.vao = 0x77;
        let p = it.to_draw_packet(RasterState(3));
        assert_eq!(p.shader.0.0, 5);
        assert_eq!(p.head, [1, 0x77]);
        assert_eq!(p.blend, 1.0);
        assert_eq!(p.kind.discriminant(), 3);
    }

    #[test]
    fn topology_tag_and_payload_match_the_element_layout() {
        assert_eq!(DrawTopology::Default(2.0).tag(), 0);
        assert_eq!(DrawTopology::Alt(3.5).tag(), 1);
        assert_eq!(DrawTopology::Alt(3.5).payload(), 3.5);
    }
}
