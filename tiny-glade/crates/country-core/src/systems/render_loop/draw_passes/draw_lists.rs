//! `draw_lists.rs` — CPU draw-list 容器、排序与构建（`build_draw_lists` 全量恢复，B_d_gap）。
//!
//! 地址（O）：`DrawLists::new` 0x1409ba340、`DrawLists::sort` 0x1409ba4b0（135 B）、`build_draw_lists` 0x1409ba540（14103 B）。
//!
//! # `DrawLists`
//! 21 个 `Vec<(DrawListItem, DrawListFn)>`（`new` 写 21×`{0, 0x10, 0}`；`sort` 对长度 ≥ 2 的列表做**稳定**排序，键 = i32 @ 元素 `+0x60`）。
//! 元素 = [`DrawListItem`]（0x70 B）+ 回调指针（+0x70）；`DrawListItem` 与 `EntityDrawParams` 的字段布局见
//! `render/pass_builder/draw_list.rs`（本文件 re-export 为 [`DrawItem`] / [`DrawFn`] / [`DrawEntry`]，不再保留第二份）。
//!
//! # `build_draw_lists`（O，逐实体）
//! 查询 `(Entity, &Handle<Vao>, &Handle<RasterShaderProgram>, &Transform, Option<&DrawTopology>, Option<&DesignatedRenderPass>,
//! Option<&UiPass>, Option<&WireframePass>, Option<&DontCastShadow>, Option<&OcclusionCulled>)`（PDB `QueryState::new` 实例的类型名，O）。
//! 每个实体：
//! 1. `Res<AssetShaderLibrary>::debug_raster_get_name(shader)`，`None` → `expect("Oops! This shader handle doesn't have a named shader")`；
//! 2. 按着色器名查 [`DISPATCH`]（原版是编译器展开的 `match`：长度分支 + `bcmp`/SIMD 比较；本表由机器码模拟器逐名核对）
//!    得 `(kind, variant, sort_key, func)`；未命中 → `panic!("{name} doesn't have a draw function")`（draw_lists.rs:1321:21）；
//! 3. 目标列表（见 [`select_lists`]）：`DesignatedRenderPass(n)` → `[n]`；否则 `UiPass` → `[19]`；否则 `WireframePass` → `[20]`；
//!    否则按 `kind`/`variant` 选主列表，`!DontCastShadow` 追加 `0`（kind 4 无阴影），`OcclusionCulled` 再追加 OC/2 列表；
//! 4. 模板参考值 `Option<u8>`（O：`0x1409bc81f` 跳转表）：kind 0/4 → `None`；kind 1/2 → `Some(variant)`；kind 3 → `Some(variant | 0x80)`；kind 5 → `Some(4)`；
//! 5. 对每个目标列表 `Vec::push((DrawListItem{..}, func))`（`cmp rax,0x14; ja panic_bounds_check` 限定下标 ≤ 20）。
//!
//! `variant` 就是 [`MaterialId`](super::material_id::MaterialId) 的数值（Solid 0 / Grass 1 / Plant 2 / Canopy 3 / Forward 4）。
//!
//! # 回调（`DrawFnId`）
//! 分发表引用 82 个不同的回调：25 个是 `draw_shaders::*` 的直接函数，其余 `closure_env$0..60` 是 `build_draw_lists` 内的闭包
//! （nani 绘制、纹理绑定、ECS 组件读取）。**本体已恢复**：见 [`DrawFnId::is_recovered`]；其余是 typed gap（调用时 panic，文案含原地址），
//! 每个 gap 在枚举文档里列出了机器码里的调用类别，留给闭包逐个恢复（依赖 nani `GpuWorld`/`World` 资源访问器，不在本范围）。

// Rust 1.88 enables newer style suggestions; retain original nested branches and modulo expressions.
#![allow(clippy::collapsible_if)]
use bevy_ecs::component::Component;
use bevy_ecs::entity::Entity;
use bevy_ecs::world::World;

use crate::asset_libraries::Handle;
use crate::render::pass_builder::draw_builder::{DrawBuilder, DrawTransform};
use crate::render::shader::RasterShaderProgram;
use crate::render::vao::Vao;
use crate::utils::components::transform::Transform;

pub use crate::render::pass_builder::draw_list::{
    DrawListEntry as DrawEntry, DrawListFn, DrawListFn as DrawFn, DrawListItem as DrawItem,
    DrawTopology, EntityDrawParams,
};

/// 列表数量（O：`0x1f8 / 0x18`）。
pub const DRAW_LIST_COUNT: usize = 21;

/// `DesignatedRenderPass(u32)`：直接指定目标列表下标（O：`mov esi,[rdi]` → 列表；`startup_*` 里 water = 0xf、stars/clouds = 0x10）。
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq)]
pub struct DesignatedRenderPass(pub u32);
/// 零大小标记：UI 列表（19）。
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct UiPass;
/// 零大小标记：线框列表（20）。
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct WireframePass;
/// 零大小标记：不进阴影列表（0）。
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct DontCastShadow;
/// 零大小标记：额外进 OC/2 列表。
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct OcclusionCulled;

/// 21 个列表的下标（O：由各 pass 读取的 `Vec` 偏移 `(ptr_off - 8) / 0x18` 得到；名称按使用它的 pass 命名，I）。
pub mod list {
    /// `shadow_pass`（+0x08）；`select_lists` 对非 `DontCastShadow` 追加。
    pub const SHADOW: usize = 0;
    /// main_pass `G-buffer`（ptr @+0x20）。
    pub const GBUFFER: usize = 1;
    /// main_pass `G-buffer emissive`（+0x38）。
    pub const GBUFFER_EMISSIVE: usize = 2;
    /// main_pass `G-buffer velocity`（+0x50）。
    pub const GBUFFER_VELOCITY: usize = 3;
    /// main_pass `Forward`（+0x68）。
    pub const FORWARD: usize = 4;
    /// main_pass `ForwardVelocity`（+0x80）。
    pub const FORWARD_VELOCITY: usize = 5;
    /// main_pass `Velocity`（+0x98）。
    pub const VELOCITY: usize = 6;
    /// main_pass_oc2 `G-buffer OC/2`（+0xb0）。
    pub const GBUFFER_OC2: usize = 7;
    /// main_pass_oc2 `G-buffer emissive OC/2`（+0xc8）。
    pub const GBUFFER_EMISSIVE_OC2: usize = 8;
    /// main_pass_oc2 `G-buffer velocity OC/2`（+0xe0）。
    pub const GBUFFER_VELOCITY_OC2: usize = 9;
    /// main_pass_oc2 `Forward OC/2`（+0xf8）。
    pub const FORWARD_OC2: usize = 10;
    /// main_pass_oc2 `Velocity OC/2`（+0x110）。
    pub const VELOCITY_OC2: usize = 11;
    /// `Late Deferred`（+0x128）。
    pub const LATE_DEFERRED: usize = 12;
    /// `Late Forward`（+0x140）。
    pub const LATE_FORWARD: usize = 13;
    /// `refraction_copy_pass`（+0x158）。
    pub const REFRACTION: usize = 14;
    /// 水面 `DesignatedRenderPass(0xf)` 的列表（+0x170；本范围无 pass 读取，U）。
    pub const WATER_SURFACE: usize = 15;
    /// `pre_taa_transparency_pass`（+0x188；stars/clouds 的 `DesignatedRenderPass(0x10)`）。
    pub const PRE_TAA: usize = 16;
    /// `post_taa_transparency_pass`（+0x1a0）。
    pub const POST_TAA: usize = 17;
    /// `loading_screen_pass`（+0x1b8）。
    pub const LOADING_SCREEN: usize = 18;
    /// `ui_pass`（+0x1d0）。
    pub const UI: usize = 19;
    /// `wireframe_pass`（+0x1e8）。
    pub const WIREFRAME: usize = 20;
}

/// CPU draw-list 容器（O：21 个 `Vec<DrawEntry>`）。
#[derive(Default)]
pub struct DrawLists {
    lists: [Vec<DrawEntry>; DRAW_LIST_COUNT],
}

impl DrawLists {
    /// `DrawLists::new`（0x1409ba340）：21 个空 `Vec`。
    pub fn new() -> DrawLists {
        DrawLists::default()
    }

    /// Empty every list but keep its capacity, so a long-lived `DrawLists`
    /// does not regrow 21 vectors of 0x78-byte entries every frame.
    pub fn clear(&mut self) {
        for list in &mut self.lists {
            list.clear();
        }
    }

    /// 第 `idx` 个列表（越界 panic，同原版数组边界）。
    pub fn list(&self, idx: usize) -> &[DrawEntry] {
        &self.lists[idx]
    }

    /// 可变访问（`build_draw_lists` 的 `Vec::push` 目标）。
    pub fn list_mut(&mut self, idx: usize) -> &mut Vec<DrawEntry> {
        &mut self.lists[idx]
    }

    /// 全部元素数。
    pub fn total_len(&self) -> usize {
        self.lists.iter().map(Vec::len).sum()
    }

    /// `DrawLists::sort`（0x1409ba4b0）：每个长度 ≥ 2 的列表按 `sort_key`（i32）稳定排序。
    pub fn sort(&mut self) {
        for list in &mut self.lists {
            if list.len() >= 2 {
                list.sort_by_key(|e| e.0.sort_key);
            }
        }
    }
}

/// 着色器名 → 主 pass 类别（O：分发叶子写入 `[rbp+0x380]` 的 dword；`0x1409bc9fd` / `0x1409bc7fd` 两张跳转表的下标）。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PassKind {
    /// kind 0：没有主 pass 列表（必须带 `DesignatedRenderPass`/`UiPass`/`WireframePass`，否则 panic
    /// "An object in the main pass must have a material ID"）；模板参考值 `None`。
    NoMainPass,
    /// kind 1：`variant < 4` → G-buffer（1），`== 4` → Forward（4）；OC/2：7 / 10；模板 `Some(variant)`。
    Opaque,
    /// kind 2：`variant 0` → G-buffer velocity（3），`4` → ForwardVelocity（5）；OC/2：9（仅 variant 0）；模板 `Some(variant)`。
    OpaqueVelocity,
    /// kind 3：`variant` 必须为 0 → G-buffer emissive（2）；OC/2：8；模板 `Some(variant | 0x80)`。
    Emissive,
    /// kind 4：Velocity（6），**无阴影**；OC/2：11；模板 `None`。
    VelocityOnly,
    /// kind 5：Late Forward（13）；无 OC/2；模板 `Some(4)`。
    LateForward,
}

impl PassKind {
    /// 机器码里的 dword 编码。
    pub const fn from_code(code: u32) -> PassKind {
        match code {
            0 => PassKind::NoMainPass,
            1 => PassKind::Opaque,
            2 => PassKind::OpaqueVelocity,
            3 => PassKind::Emissive,
            4 => PassKind::VelocityOnly,
            _ => PassKind::LateForward,
        }
    }
    pub const fn code(self) -> u32 {
        match self {
            PassKind::NoMainPass => 0,
            PassKind::Opaque => 1,
            PassKind::OpaqueVelocity => 2,
            PassKind::Emissive => 3,
            PassKind::VelocityOnly => 4,
            PassKind::LateForward => 5,
        }
    }
}

/// 分发表一行。
#[derive(Clone, Copy, Debug)]
pub struct ShaderDispatch {
    pub name: &'static str,
    pub kind: PassKind,
    /// `[rbp+0x384]`：`MaterialId` 数值（kind 0 的 6 无意义）。
    pub variant: u32,
    /// `[rbp+0x390]` → 元素 `+0x60` 的排序键。
    pub sort_key: i32,
    pub func: DrawFnId,
}

/// `unreachable!()` 文案（O：`internal error: entered unreachable code`）。
pub const UNREACHABLE: &str = "internal error: entered unreachable code";
/// `kind == 0` 且无显式 pass 时的 panic 文案（O：`.rdata`）。
pub const NO_MATERIAL_ID: &str = "An object in the main pass must have a material ID";
/// 着色器句柄没有名字时的 `expect` 文案（O：`.rdata`，draw_lists.rs）。
pub const NO_SHADER_NAME: &str = "Oops! This shader handle doesn't have a named shader";

/// 按名字查分发表。
pub fn lookup(name: &str) -> Option<&'static ShaderDispatch> {
    DISPATCH.iter().find(|d| d.name == name)
}

/// 一个实体的 pass 标记（`Option<&C>` 查询项的存在性）。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct PassFlags {
    pub designated: Option<u32>,
    pub ui: bool,
    pub wireframe: bool,
    pub dont_cast_shadow: bool,
    pub occlusion_culled: bool,
}

/// 目标列表与模板参考值（O：`0x1409bc717..0x1409bcd14` + `0x1409bc7fd` 跳转表）。
pub fn select_lists(d: &ShaderDispatch, f: &PassFlags) -> (Vec<usize>, Option<u8>) {
    let v = d.variant;
    let stencil = match d.kind {
        PassKind::NoMainPass | PassKind::VelocityOnly => None,
        PassKind::Opaque | PassKind::OpaqueVelocity => Some(v as u8),
        PassKind::Emissive => Some((v as u8) | 0x80),
        PassKind::LateForward => Some(4),
    };
    if let Some(n) = f.designated {
        return (vec![n as usize], stencil);
    }
    if f.ui {
        return (vec![list::UI], stencil);
    }
    if f.wireframe {
        return (vec![list::WIREFRAME], stencil);
    }
    let mut lists = Vec::with_capacity(3);
    let main = match d.kind {
        PassKind::NoMainPass => panic!("{}", NO_MATERIAL_ID),
        PassKind::Opaque => match v {
            0..=3 => list::GBUFFER,
            4 => list::FORWARD,
            _ => unreachable!(),
        },
        PassKind::OpaqueVelocity => match v {
            0 => list::GBUFFER_VELOCITY,
            4 => list::FORWARD_VELOCITY,
            _ => unreachable!(),
        },
        PassKind::Emissive => match v {
            0 => list::GBUFFER_EMISSIVE,
            _ => unreachable!(),
        },
        PassKind::VelocityOnly => list::VELOCITY,
        PassKind::LateForward => list::LATE_FORWARD,
    };
    lists.push(main);
    if d.kind != PassKind::VelocityOnly && !f.dont_cast_shadow {
        lists.push(list::SHADOW);
    }
    if f.occlusion_culled {
        lists.push(match d.kind {
            PassKind::Opaque => match v {
                0..=3 => list::GBUFFER_OC2,
                4 => list::FORWARD_OC2,
                _ => unreachable!(),
            },
            PassKind::OpaqueVelocity => match v {
                0 => list::GBUFFER_VELOCITY_OC2,
                _ => unreachable!(),
            },
            PassKind::Emissive => match v {
                0 => list::GBUFFER_EMISSIVE_OC2,
                _ => unreachable!(),
            },
            PassKind::VelocityOnly => list::VELOCITY_OC2,
            _ => unreachable!(),
        });
    }
    (lists, stencil)
}

/// 查询行（`Query<(Entity, &Handle<Vao>, &Handle<RasterShaderProgram>, &Transform, Option<&DrawTopology>, …)>` 的一项）。
#[derive(Clone, Copy, Debug)]
pub struct DrawRow {
    pub entity: Entity,
    pub vao: u64,
    pub shader: u64,
    pub transform: DrawTransform,
    pub topology: Option<DrawTopology>,
    pub flags: PassFlags,
}

/// `AssetShaderLibrary::debug_raster_get_name`（0x140937cb0）的调用面。
pub trait ShaderNames {
    fn raster_name(&self, shader: Handle<RasterShaderProgram>) -> Option<&'static str>;
}

impl ShaderNames for crate::asset_libraries::shader_library::AssetShaderLibrary {
    fn raster_name(&self, shader: Handle<RasterShaderProgram>) -> Option<&'static str> {
        self.debug_raster_get_name(shader)
    }
}

/// `Transform` → 元素里的 0x30 B 变换（O：3×`movaps` 整块拷贝，布局即 `DrawTransform`）。
pub fn draw_transform(t: &Transform) -> DrawTransform {
    DrawTransform {
        rotation: t.rotation,
        translation: t.translation,
        scale: t.scale,
    }
}

/// 逐行入列（`0x1409bb2f5..0x1409bc9ea` 的循环体）。`name` = 该行着色器的 `debug_raster_get_name`。
pub fn push_row(lists: &mut DrawLists, row: &DrawRow, name: &'static str) {
    let d = lookup(name).unwrap_or_else(|| panic!("{name} doesn't have a draw function"));
    let (targets, stencil_ref) = select_lists(d, &row.flags);
    let item = DrawItem {
        params: EntityDrawParams {
            topology: row.topology,
            entity: row.entity,
            shader_name: name,
            transform: row.transform,
        },
        vao: row.vao,
        shader: row.shader,
        sort_key: d.sort_key,
        stencil_ref,
    };
    let func = d.func.func();
    for idx in targets {
        // O：`cmp rax, 0x14; ja panic_bounds_check`。
        lists.list_mut(idx).push((item, func));
    }
}

/// `build_draw_lists`（0x1409ba540）：遍历 `world` 里的可绘制实体，按着色器名入列。
///
/// 偏差（I）：原版经 `Res<AssetShaderLibrary>`（`get_resource_id`）取库；本 crate 的库不是 `Resource`，故由调用方给出
/// [`ShaderNames`]。puffin scope（`profile_function!`）省略。
pub fn build_draw_lists(world: &mut World, names: &dyn ShaderNames, lists: &mut DrawLists) {
    let mut q = world.query::<(
        Entity,
        &Handle<Vao>,
        &Handle<RasterShaderProgram>,
        &Transform,
        Option<&DrawTopology>,
        Option<&DesignatedRenderPass>,
        Option<&UiPass>,
        Option<&WireframePass>,
        Option<&DontCastShadow>,
        Option<&OcclusionCulled>,
    )>();
    for (entity, vao, shader, transform, topo, designated, ui, wire, no_shadow, oc) in q.iter(world)
    {
        let name = names.raster_name(*shader).expect(NO_SHADER_NAME);
        let row = DrawRow {
            entity,
            vao: vao.id().raw(),
            shader: shader.id().raw(),
            transform: draw_transform(transform),
            topology: topo.copied(),
            flags: PassFlags {
                designated: designated.map(|d| d.0),
                ui: ui.is_some(),
                wireframe: wire.is_some(),
                dont_cast_shadow: no_shadow.is_some(),
                occlusion_culled: oc.is_some(),
            },
        };
        push_row(lists, &row, name);
    }
}

/// 已恢复的回调本体。
pub mod bodies {
    use super::*;
    use crate::systems::render_loop::draw_passes::draw_shaders as ds;

    /// closure_env$4：`DrawBuilder::discard`（`instanced_background_tree_shader`）。
    pub fn discard(b: DrawBuilder<'_>, _p: &EntityDrawParams, _w: &mut World) {
        b.discard();
    }

    /// closure_env$42：`DrawBuilder::draw`（`sun_gizmo_shader`）。
    pub fn plain_draw(b: DrawBuilder<'_>, _p: &EntityDrawParams, _w: &mut World) {
        b.draw();
    }

    /// closure_env$35（0x1409b8340，`vertex_color_shader` / `sparkles_shader` / `ui_2d` 等）：
    /// `topology tag == 1` → `state.byte0 = (b & 0xfc) | 2` 且 `packet.+0xac`（f32）= 载荷；其它保持；然后 `draw`。
    pub fn topology_draw(mut b: DrawBuilder<'_>, p: &EntityDrawParams, _w: &mut World) {
        if let Some(DrawTopology::Alt(payload)) = p.topology {
            let pk = b.packet_mut();
            pk.state.0 = (pk.state.0 & !3) | 2;
            pk.blend = payload;
        }
        b.draw();
    }

    /// closure_env$58（draw_lists.rs:1294:25，`normal_debug_shader`）：`todo!("TODO(vulkan)")`。
    pub fn todo_vulkan_1294(_b: DrawBuilder<'_>, _p: &EntityDrawParams, _w: &mut World) {
        panic!("not yet implemented: TODO(vulkan)");
    }

    /// closure_env$59（draw_lists.rs:1304:25，`ivy_debug_lines_shader`）：`todo!("TODO(vulkan)")`。
    pub fn todo_vulkan_1304(_b: DrawBuilder<'_>, _p: &EntityDrawParams, _w: &mut World) {
        panic!("not yet implemented: TODO(vulkan)");
    }

    /// closure_env$46：`draw_floor_shader(b, false)`（`r9d = 0`；`rdx/r8` 未用）。
    pub fn floor(b: DrawBuilder<'_>, _p: &EntityDrawParams, _w: &mut World) {
        ds::floor_shader::draw_floor_shader(b, false);
    }

    pub fn path_pebble(b: DrawBuilder<'_>, _p: &EntityDrawParams, _w: &mut World) {
        ds::path_pebble_shader::path_pebble_draw(b);
    }
    pub fn grass(b: DrawBuilder<'_>, _p: &EntityDrawParams, _w: &mut World) {
        ds::grass_shader::draw_grass_shader(b);
    }
    pub fn grass_lod1(b: DrawBuilder<'_>, _p: &EntityDrawParams, _w: &mut World) {
        ds::grass_shader::draw_grass_lod1_shader(b);
    }
    pub fn lilypad(b: DrawBuilder<'_>, _p: &EntityDrawParams, _w: &mut World) {
        ds::water_planes_draw::lilypad_draw(b);
    }
    pub fn lilypad_flower(b: DrawBuilder<'_>, _p: &EntityDrawParams, _w: &mut World) {
        ds::water_planes_draw::lilypad_flower_draw(b);
    }
    pub fn reed(b: DrawBuilder<'_>, _p: &EntityDrawParams, _w: &mut World) {
        ds::water_planes_draw::reed_draw(b);
    }
    pub fn butterfly(b: DrawBuilder<'_>, _p: &EntityDrawParams, _w: &mut World) {
        ds::instanced_critters::draw_instanced_butterfly_shader(b);
    }
}

/// 分发表里出现的 82 个回调（O：`lea rax,[fn]` 的目标）。`Env<N>` = `build_draw_lists::closure_env$N` 的 `FnOnce::call_once`，其余是 `draw_shaders::*` 直接函数。
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum DrawFnId {
    /// 0x14086a780（1507 B） `draw_shaders::stairs_draw::stairs_draw`
    StairsDraw,
    /// 0x1408a4ca0（1831 B） `draw_shaders::draw_bricks::indirect_draw_bricks<1>`
    IndirectDrawBricksLod1,
    /// 0x1408a53d0（1692 B） `draw_shaders::draw_bricks::indirect_draw_bricks<0>`
    IndirectDrawBricksLod0,
    /// 0x1408dbc50（1319 B） `draw_shaders::instanced_ivy::draw_instanced_ivy_branch_shader`
    DrawInstancedIvyBranchShader,
    /// 0x1408dc180（21 B） `draw_shaders::instanced_ivy::draw_instanced_ivy_leaf_shader`
    DrawInstancedIvyLeafShader,
    /// 0x1408dc1a0（21 B） `draw_shaders::instanced_ivy::draw_instanced_ivy_flower_shader`
    DrawInstancedIvyFlowerShader,
    /// 0x1408e5a10（227 B） `draw_shaders::path_pebble_shader::path_pebble_draw`；已恢复本体
    PathPebbleDraw,
    /// 0x1409ae900（2689 B） `closure_env$6`：get_id×3, table.get_component×3, bind_texture×10, write_uniform, draw
    Env6,
    /// 0x1409af390（2347 B） `closure_env$27`：get_id×5, table.get_component×3, bind_texture, write_uniform×4, draw
    Env27,
    /// 0x1409afcc0（2209 B） `closure_env$2`：get_id×2, table.get_component×2, write_uniform×4, read_storage_buffer, draw
    Env2,
    /// 0x1409b0570（634 B） `closure_env$48`：bind_texture×4, draw
    Env48,
    /// 0x1409b07f0（802 B） `closure_env$52`：get_resource_id, bind_texture×3, draw, discard
    Env52,
    /// 0x1409b0b20（1146 B） `closure_env$57`：get_resource_id, get_id×2, table.get_component×2, frozen::FrozenMap::get, write_uniform×2, draw
    Env57,
    /// 0x1409b0fa0（778 B） `closure_env$45`：bind_texture×5, draw
    Env45,
    /// 0x1409b12b0（2037 B） `closure_env$0`：get_resource_id×3, bind_texture×5, read_storage_buffer×2, write_uniform×3, nani_draw
    Env0,
    /// 0x1409b1ab0（74 B） `closure_env$42`：draw；已恢复本体
    Env42,
    /// 0x1409b1b00（903 B） `closure_env$37`：get_resource_id, bind_texture×2, write_uniform×2, draw
    Env37,
    /// 0x1409b1e90（76 B） `closure_env$39`：nani_draw
    Env39,
    /// 0x1409b1ee0（564 B） `closure_env$19`：get_id, table.get_component, write_uniform, draw
    Env19,
    /// 0x1409b2120（362 B） `closure_env$32`：bind_texture×2, nani_draw
    Env32,
    /// 0x1409b2290（741 B） `closure_env$41`：get_resource_id, bind_texture, write_uniform×2, draw
    Env41,
    /// 0x1409b2580（634 B） `closure_env$53`：bind_texture×4, draw
    Env53,
    /// 0x1409b2800（181 B） `closure_env$58`：；已恢复本体
    Env58,
    /// 0x1409b28c0（1651 B） `closure_env$23`：get_id×2, table.get_component×2, write_uniform×2, bind_texture×3, get_resource_id, draw
    Env23,
    /// 0x1409b2f40（730 B） `closure_env$8`：bind_texture×5, nani_draw
    Env8,
    /// 0x1409b3220（76 B） `closure_env$28`：nani_draw
    Env28,
    /// 0x1409b3270（1287 B） `closure_env$16`：get_resource_id, bind_texture×3, read_storage_buffer×4, bind_wall_color_customization, write_uniform×2, bind_stone_floor_textures, bind_wooden_floor_textures, nani_draw
    Env16,
    /// 0x1409b3780（1111 B） `closure_env$11`：get_resource_id, bind_texture×5, read_storage_buffer, nani_draw
    Env11,
    /// 0x1409b3be0（218 B） `closure_env$20`：bind_texture, draw
    Env20,
    /// 0x1409b3cc0（490 B） `closure_env$10`：bind_texture×3, read_storage_buffer, nani_draw
    Env10,
    /// 0x1409b3eb0（708 B） `closure_env$55`：get_id, table.get_component, write_uniform×2, draw
    Env55,
    /// 0x1409b4180（234 B） `closure_env$33`：bind_texture, nani_draw
    Env33,
    /// 0x1409b4270（714 B） `closure_env$17`：bind_texture×3, read_storage_buffer×4, bind_wall_color_customization, bind_stone_floor_textures, bind_wooden_floor_textures, nani_draw
    Env17,
    /// 0x1409b4540（181 B） `closure_env$59`：；已恢复本体
    Env59,
    /// 0x1409b4600（602 B） `closure_env$40`：bind_texture×4, draw
    Env40,
    /// 0x1409b4860（2259 B） `closure_env$22`：get_id×2, table.get_component×2, write_uniform×6, bind_texture×3, get_resource_id, draw
    Env22,
    /// 0x1409b5140（1169 B） `closure_env$3`：get_resource_id×3, write_uniform×3, read_storage_buffer, draw, discard
    Env3,
    /// 0x1409b55e0（474 B） `closure_env$30`：bind_texture×3, nani_draw
    Env30,
    /// 0x1409b57c0（1191 B） `closure_env$14`：get_id, table.get_component, bind_texture×3, write_uniform×2, read_storage_buffer, draw
    Env14,
    /// 0x1409b5c70（117 B） `closure_env$1`：read_storage_buffer, multi_draw_indexed_indirect
    Env1,
    /// 0x1409b5cf0（1025 B） `closure_env$18`：get_id×2, write_uniform×2, draw
    Env18,
    /// 0x1409b6100（76 B） `closure_env$25`：nani_draw
    Env25,
    /// 0x1409b6150（1002 B） `closure_env$7`：bind_texture×7, read_storage_buffer, multi_draw_indexed_indirect
    Env7,
    /// 0x1409b6540（926 B） `closure_env$13`：write_uniform, read_storage_buffer×2, bind_texture×5, bind_wall_color_customization, bind_brick_erase_highlight, nani_draw
    Env13,
    /// 0x1409b68e0（2082 B） `closure_env$5`：bind_texture×9, get_resource_id, utils::camera::CameraMatrices::eye_posit, write_uniform, get_id, table.get_component, draw
    Env5,
    /// 0x1409b7110（234 B） `closure_env$54`：bind_texture, draw
    Env54,
    /// 0x1409b7200（856 B） `closure_env$15`：bind_texture×5, nani_draw
    Env15,
    /// 0x1409b7560（2009 B） `closure_env$34`：get_resource_id×4, bind_texture×2, bind_wall_color_customization, write_uniform×5, nani_draw
    Env34,
    /// 0x1409b7d40（378 B） `closure_env$26`：bind_texture×2, read_storage_buffer, multi_draw_indexed_indirect
    Env26,
    /// 0x1409b7ec0（218 B） `closure_env$44`：bind_texture, nani_draw
    Env44,
    /// 0x1409b7fa0（674 B） `closure_env$36`：get_resource_id, frozen::FrozenMap::get, sparse_instance_buffer::impl$3::to_draw_, read_storage_buffer, draw
    Env36,
    /// 0x1409b8250（234 B） `closure_env$51`：bind_texture, draw
    Env51,
    /// 0x1409b8340（231 B） `closure_env$35`：draw；已恢复本体
    Env35,
    /// 0x1409b8430（312 B） `closure_env$60`：bind_texture, draw
    Env60,
    /// 0x1409b8570（83 B） `closure_env$46`：country_core::systems::render_loop::draw；已恢复本体
    Env46,
    /// 0x1409b85d0（74 B） `closure_env$4`：discard；已恢复本体
    Env4,
    /// 0x1409b8620（83 B） `closure_env$43`：country_core::systems::render_loop::draw
    Env43,
    /// 0x1409b8680（2467 B） `closure_env$21`：get_id×2, table.get_component×2, write_uniform×4, bind_texture×6, get_resource_id, draw
    Env21,
    /// 0x1409b9030（76 B） `closure_env$49`：nani_draw
    Env49,
    /// 0x1409b9080（1223 B） `closure_env$38`：get_resource_id, bind_texture×4, write_uniform×2, draw
    Env38,
    /// 0x1409b9550（906 B） `closure_env$12`：write_uniform, read_storage_buffer×2, bind_texture×5, bind_wall_color_customization, nani_draw
    Env12,
    /// 0x1409b98e0（437 B） `closure_env$29`：bind_texture×2, nani_draw
    Env29,
    /// 0x1409b9aa0（362 B） `closure_env$31`：bind_texture×2, nani_draw
    Env31,
    /// 0x1409b9c10（474 B） `closure_env$9`：bind_texture×3, nani_draw
    Env9,
    /// 0x1409b9df0（156 B） `closure_env$24`：nani_draw
    Env24,
    /// 0x1409c9830（2841 B） `draw_shaders::clearing_shaders::tree_trunk`
    TreeTrunk,
    /// 0x1409ca350（2400 B） `draw_shaders::clearing_shaders::tree_canopy`
    TreeCanopy,
    /// 0x1409d9070（1579 B） `draw_shaders::instanced_roof::draw_instanced_roof_shader_nani`
    DrawInstancedRoofShaderNani,
    /// 0x1409d96a0（498 B） `draw_shaders::instanced_roof::draw_instanced_roof_spire_shader`
    DrawInstancedRoofSpireShader,
    /// 0x1409d98a0（788 B） `draw_shaders::instanced_roof::draw_instanced_roof_support_beam_shader`
    DrawInstancedRoofSupportBeamShader,
    /// 0x1409d9bc0（2487 B） `draw_shaders::water_planes_draw::water_planes_draw`
    WaterPlanesDraw,
    /// 0x1409da580（474 B） `draw_shaders::water_planes_draw::lilypad_draw`；已恢复本体
    LilypadDraw,
    /// 0x1409da760（474 B） `draw_shaders::water_planes_draw::lilypad_flower_draw`；已恢复本体
    LilypadFlowerDraw,
    /// 0x1409da940（344 B） `draw_shaders::water_planes_draw::reed_draw`；已恢复本体
    ReedDraw,
    /// 0x140a8a090（628 B） `draw_shaders::leaf_particle_draw::leaf_particle_draw`
    LeafParticleDraw,
    /// 0x140b4af70（600 B） `draw_shaders::grass_shader::draw_grass_shader`；已恢复本体
    DrawGrassShader,
    /// 0x140b4b1d0（600 B） `draw_shaders::grass_shader::draw_grass_lod1_shader`；已恢复本体
    DrawGrassLod1Shader,
    /// 0x1415547e0（456 B） `draw_shaders::instanced_critters::draw_instanced_butterfly_shader`；已恢复本体
    DrawInstancedButterflyShader,
    /// 0x1415549b0（828 B） `draw_shaders::instanced_critters::draw_instanced_firefly_shader`
    DrawInstancedFireflyShader,
    /// 0x141554cf0（908 B） `draw_shaders::instanced_critters::draw_instanced_firefly_glow_shader`
    DrawInstancedFireflyGlowShader,
    /// 0x141555080（3637 B） `draw_shaders::instanced_critters::draw_duck_shader`
    DrawDuckShader,
    /// 0x141555ec0（437 B） `draw_shaders::instanced_critters::draw_wren_shader`
    DrawWrenShader,
}

impl DrawFnId {
    /// 原版函数地址（O）。
    pub const fn addr(self) -> u64 {
        match self {
            DrawFnId::StairsDraw => 0x14086a780,
            DrawFnId::IndirectDrawBricksLod1 => 0x1408a4ca0,
            DrawFnId::IndirectDrawBricksLod0 => 0x1408a53d0,
            DrawFnId::DrawInstancedIvyBranchShader => 0x1408dbc50,
            DrawFnId::DrawInstancedIvyLeafShader => 0x1408dc180,
            DrawFnId::DrawInstancedIvyFlowerShader => 0x1408dc1a0,
            DrawFnId::PathPebbleDraw => 0x1408e5a10,
            DrawFnId::Env6 => 0x1409ae900,
            DrawFnId::Env27 => 0x1409af390,
            DrawFnId::Env2 => 0x1409afcc0,
            DrawFnId::Env48 => 0x1409b0570,
            DrawFnId::Env52 => 0x1409b07f0,
            DrawFnId::Env57 => 0x1409b0b20,
            DrawFnId::Env45 => 0x1409b0fa0,
            DrawFnId::Env0 => 0x1409b12b0,
            DrawFnId::Env42 => 0x1409b1ab0,
            DrawFnId::Env37 => 0x1409b1b00,
            DrawFnId::Env39 => 0x1409b1e90,
            DrawFnId::Env19 => 0x1409b1ee0,
            DrawFnId::Env32 => 0x1409b2120,
            DrawFnId::Env41 => 0x1409b2290,
            DrawFnId::Env53 => 0x1409b2580,
            DrawFnId::Env58 => 0x1409b2800,
            DrawFnId::Env23 => 0x1409b28c0,
            DrawFnId::Env8 => 0x1409b2f40,
            DrawFnId::Env28 => 0x1409b3220,
            DrawFnId::Env16 => 0x1409b3270,
            DrawFnId::Env11 => 0x1409b3780,
            DrawFnId::Env20 => 0x1409b3be0,
            DrawFnId::Env10 => 0x1409b3cc0,
            DrawFnId::Env55 => 0x1409b3eb0,
            DrawFnId::Env33 => 0x1409b4180,
            DrawFnId::Env17 => 0x1409b4270,
            DrawFnId::Env59 => 0x1409b4540,
            DrawFnId::Env40 => 0x1409b4600,
            DrawFnId::Env22 => 0x1409b4860,
            DrawFnId::Env3 => 0x1409b5140,
            DrawFnId::Env30 => 0x1409b55e0,
            DrawFnId::Env14 => 0x1409b57c0,
            DrawFnId::Env1 => 0x1409b5c70,
            DrawFnId::Env18 => 0x1409b5cf0,
            DrawFnId::Env25 => 0x1409b6100,
            DrawFnId::Env7 => 0x1409b6150,
            DrawFnId::Env13 => 0x1409b6540,
            DrawFnId::Env5 => 0x1409b68e0,
            DrawFnId::Env54 => 0x1409b7110,
            DrawFnId::Env15 => 0x1409b7200,
            DrawFnId::Env34 => 0x1409b7560,
            DrawFnId::Env26 => 0x1409b7d40,
            DrawFnId::Env44 => 0x1409b7ec0,
            DrawFnId::Env36 => 0x1409b7fa0,
            DrawFnId::Env51 => 0x1409b8250,
            DrawFnId::Env35 => 0x1409b8340,
            DrawFnId::Env60 => 0x1409b8430,
            DrawFnId::Env46 => 0x1409b8570,
            DrawFnId::Env4 => 0x1409b85d0,
            DrawFnId::Env43 => 0x1409b8620,
            DrawFnId::Env21 => 0x1409b8680,
            DrawFnId::Env49 => 0x1409b9030,
            DrawFnId::Env38 => 0x1409b9080,
            DrawFnId::Env12 => 0x1409b9550,
            DrawFnId::Env29 => 0x1409b98e0,
            DrawFnId::Env31 => 0x1409b9aa0,
            DrawFnId::Env9 => 0x1409b9c10,
            DrawFnId::Env24 => 0x1409b9df0,
            DrawFnId::TreeTrunk => 0x1409c9830,
            DrawFnId::TreeCanopy => 0x1409ca350,
            DrawFnId::DrawInstancedRoofShaderNani => 0x1409d9070,
            DrawFnId::DrawInstancedRoofSpireShader => 0x1409d96a0,
            DrawFnId::DrawInstancedRoofSupportBeamShader => 0x1409d98a0,
            DrawFnId::WaterPlanesDraw => 0x1409d9bc0,
            DrawFnId::LilypadDraw => 0x1409da580,
            DrawFnId::LilypadFlowerDraw => 0x1409da760,
            DrawFnId::ReedDraw => 0x1409da940,
            DrawFnId::LeafParticleDraw => 0x140a8a090,
            DrawFnId::DrawGrassShader => 0x140b4af70,
            DrawFnId::DrawGrassLod1Shader => 0x140b4b1d0,
            DrawFnId::DrawInstancedButterflyShader => 0x1415547e0,
            DrawFnId::DrawInstancedFireflyShader => 0x1415549b0,
            DrawFnId::DrawInstancedFireflyGlowShader => 0x141554cf0,
            DrawFnId::DrawDuckShader => 0x141555080,
            DrawFnId::DrawWrenShader => 0x141555ec0,
        }
    }

    /// 是否已恢复本体（否则调用时 panic，见 [`gap`]）。
    pub const fn is_recovered(self) -> bool {
        matches!(
            self,
            DrawFnId::PathPebbleDraw
                | DrawFnId::StairsDraw
                | DrawFnId::IndirectDrawBricksLod1
                | DrawFnId::IndirectDrawBricksLod0
                | DrawFnId::DrawInstancedIvyBranchShader
                | DrawFnId::DrawInstancedIvyLeafShader
                | DrawFnId::DrawInstancedIvyFlowerShader
                | DrawFnId::Env6
                | DrawFnId::Env27
                | DrawFnId::Env2
                | DrawFnId::Env48
                | DrawFnId::Env52
                | DrawFnId::Env57
                | DrawFnId::Env45
                | DrawFnId::Env0
                | DrawFnId::Env37
                | DrawFnId::Env39
                | DrawFnId::Env19
                | DrawFnId::Env32
                | DrawFnId::Env41
                | DrawFnId::Env53
                | DrawFnId::Env23
                | DrawFnId::Env8
                | DrawFnId::Env28
                | DrawFnId::Env16
                | DrawFnId::Env11
                | DrawFnId::Env20
                | DrawFnId::Env10
                | DrawFnId::Env55
                | DrawFnId::Env33
                | DrawFnId::Env17
                | DrawFnId::Env40
                | DrawFnId::Env22
                | DrawFnId::Env3
                | DrawFnId::Env30
                | DrawFnId::Env14
                | DrawFnId::Env42
                | DrawFnId::Env58
                | DrawFnId::Env59
                | DrawFnId::Env35
                | DrawFnId::Env46
                | DrawFnId::Env4
                | DrawFnId::LilypadDraw
                | DrawFnId::LilypadFlowerDraw
                | DrawFnId::ReedDraw
                | DrawFnId::DrawGrassShader
                | DrawFnId::DrawGrassLod1Shader
                | DrawFnId::DrawInstancedButterflyShader
                | DrawFnId::Env1
                | DrawFnId::Env18
                | DrawFnId::Env25
                | DrawFnId::Env7
                | DrawFnId::Env13
                | DrawFnId::Env5
                | DrawFnId::Env54
                | DrawFnId::Env15
                | DrawFnId::Env34
                | DrawFnId::Env26
                | DrawFnId::Env44
                | DrawFnId::Env36
                | DrawFnId::Env51
                | DrawFnId::Env60
                | DrawFnId::Env43
                | DrawFnId::Env21
                | DrawFnId::Env49
                | DrawFnId::Env38
                | DrawFnId::Env12
                | DrawFnId::Env29
                | DrawFnId::Env31
                | DrawFnId::Env9
                | DrawFnId::Env24
                | DrawFnId::TreeTrunk
                | DrawFnId::TreeCanopy
                | DrawFnId::DrawInstancedRoofShaderNani
                | DrawFnId::DrawInstancedRoofSpireShader
                | DrawFnId::DrawInstancedRoofSupportBeamShader
                | DrawFnId::WaterPlanesDraw
                | DrawFnId::LeafParticleDraw
                | DrawFnId::DrawInstancedFireflyShader
                | DrawFnId::DrawInstancedFireflyGlowShader
                | DrawFnId::DrawDuckShader
                | DrawFnId::DrawWrenShader
        )
    }

    /// 回调函数指针。
    pub fn func(self) -> DrawListFn {
        match self {
            DrawFnId::StairsDraw => super::draw_callbacks_a::stairs_draw_cb,
            DrawFnId::IndirectDrawBricksLod1 => super::draw_callbacks_a::bricks_lod1_cb,
            DrawFnId::IndirectDrawBricksLod0 => super::draw_callbacks_a::bricks_lod0_cb,
            DrawFnId::DrawInstancedIvyBranchShader => super::draw_callbacks_a::ivy_branch_cb,
            DrawFnId::DrawInstancedIvyLeafShader => super::draw_callbacks_a::ivy_leaf_cb,
            DrawFnId::DrawInstancedIvyFlowerShader => super::draw_callbacks_a::ivy_flower_cb,
            DrawFnId::PathPebbleDraw => bodies::path_pebble,
            DrawFnId::Env6 => super::draw_callbacks_a::env6,
            DrawFnId::Env27 => super::draw_callbacks_a::env27,
            DrawFnId::Env2 => super::draw_callbacks_a::env2,
            DrawFnId::Env48 => super::draw_callbacks_a::env48,
            DrawFnId::Env52 => super::draw_callbacks_a::env52,
            DrawFnId::Env57 => super::draw_callbacks_a::env57,
            DrawFnId::Env45 => super::draw_callbacks_a::env45,
            DrawFnId::Env0 => super::draw_callbacks_a::env0,
            DrawFnId::Env42 => bodies::plain_draw,
            DrawFnId::Env37 => super::draw_callbacks_a::env37,
            DrawFnId::Env39 => super::draw_callbacks_a::env39,
            DrawFnId::Env19 => super::draw_callbacks_a::env19,
            DrawFnId::Env32 => super::draw_callbacks_a::env32,
            DrawFnId::Env41 => super::draw_callbacks_a::env41,
            DrawFnId::Env53 => super::draw_callbacks_a::env53,
            DrawFnId::Env58 => bodies::todo_vulkan_1294,
            DrawFnId::Env23 => super::draw_callbacks_a::env23,
            DrawFnId::Env8 => super::draw_callbacks_a::env8,
            DrawFnId::Env28 => super::draw_callbacks_a::env28,
            DrawFnId::Env16 => super::draw_callbacks_a::env16,
            DrawFnId::Env11 => super::draw_callbacks_a::env11,
            DrawFnId::Env20 => super::draw_callbacks_a::env20,
            DrawFnId::Env10 => super::draw_callbacks_a::env10,
            DrawFnId::Env55 => super::draw_callbacks_a::env55,
            DrawFnId::Env33 => super::draw_callbacks_a::env33,
            DrawFnId::Env17 => super::draw_callbacks_a::env17,
            DrawFnId::Env59 => bodies::todo_vulkan_1304,
            DrawFnId::Env40 => super::draw_callbacks_a::env40,
            DrawFnId::Env22 => super::draw_callbacks_a::env22,
            DrawFnId::Env3 => super::draw_callbacks_a::env3,
            DrawFnId::Env30 => super::draw_callbacks_a::env30,
            DrawFnId::Env14 => super::draw_callbacks_a::env14,
            DrawFnId::Env1 => super::draw_callbacks_b::env1,
            DrawFnId::Env18 => super::draw_callbacks_b::env18,
            DrawFnId::Env25 => super::draw_callbacks_b::env25,
            DrawFnId::Env7 => super::draw_callbacks_b::env7,
            DrawFnId::Env13 => super::draw_callbacks_b::env13,
            DrawFnId::Env5 => super::draw_callbacks_b::env5,
            DrawFnId::Env54 => super::draw_callbacks_b::env54,
            DrawFnId::Env15 => super::draw_callbacks_b::env15,
            DrawFnId::Env34 => super::draw_callbacks_b::env34,
            DrawFnId::Env26 => super::draw_callbacks_b::env26,
            DrawFnId::Env44 => super::draw_callbacks_b::env44,
            DrawFnId::Env36 => super::draw_callbacks_b::env36,
            DrawFnId::Env51 => super::draw_callbacks_b::env51,
            DrawFnId::Env35 => bodies::topology_draw,
            DrawFnId::Env60 => super::draw_callbacks_b::env60,
            DrawFnId::Env46 => bodies::floor,
            DrawFnId::Env4 => bodies::discard,
            DrawFnId::Env43 => super::draw_callbacks_b::env43,
            DrawFnId::Env21 => super::draw_callbacks_b::env21,
            DrawFnId::Env49 => super::draw_callbacks_b::env49,
            DrawFnId::Env38 => super::draw_callbacks_b::env38,
            DrawFnId::Env12 => super::draw_callbacks_b::env12,
            DrawFnId::Env29 => super::draw_callbacks_b::env29,
            DrawFnId::Env31 => super::draw_callbacks_b::env31,
            DrawFnId::Env9 => super::draw_callbacks_b::env9,
            DrawFnId::Env24 => super::draw_callbacks_b::env24,
            DrawFnId::TreeTrunk => super::draw_callbacks_b::tree_trunk_cb,
            DrawFnId::TreeCanopy => super::draw_callbacks_b::tree_canopy_cb,
            DrawFnId::DrawInstancedRoofShaderNani => super::draw_callbacks_b::roof_nani_cb,
            DrawFnId::DrawInstancedRoofSpireShader => super::draw_callbacks_b::roof_spire_cb,
            DrawFnId::DrawInstancedRoofSupportBeamShader => {
                super::draw_callbacks_b::roof_support_beam_cb
            }
            DrawFnId::WaterPlanesDraw => super::draw_callbacks_b::water_planes_cb,
            DrawFnId::LilypadDraw => bodies::lilypad,
            DrawFnId::LilypadFlowerDraw => bodies::lilypad_flower,
            DrawFnId::ReedDraw => bodies::reed,
            DrawFnId::LeafParticleDraw => super::draw_callbacks_b::leaf_particle_cb,
            DrawFnId::DrawGrassShader => bodies::grass,
            DrawFnId::DrawGrassLod1Shader => bodies::grass_lod1,
            DrawFnId::DrawInstancedButterflyShader => bodies::butterfly,
            DrawFnId::DrawInstancedFireflyShader => super::draw_callbacks_b::firefly_cb,
            DrawFnId::DrawInstancedFireflyGlowShader => super::draw_callbacks_b::firefly_glow_cb,
            DrawFnId::DrawDuckShader => super::draw_callbacks_b::duck_cb,
            DrawFnId::DrawWrenShader => super::draw_callbacks_b::wren_cb,
        }
    }
}

/// 名字分发表（O：`build_draw_lists` 0x1409ba540 里按着色器名的 `match`，85 个叶子/87 个名字；顺序 = 叶子在机器码中的地址序）。
pub const DISPATCH: &[ShaderDispatch] = &[
    ShaderDispatch {
        name: "tree_canopy",
        kind: PassKind::from_code(1),
        variant: 3,
        sort_key: 1000,
        func: DrawFnId::TreeCanopy,
    }, // leaf 0x1409bb4ee fn 0x1409ca350
    ShaderDispatch {
        name: "support_beam_roof_shader",
        kind: PassKind::from_code(1),
        variant: 0,
        sort_key: 3500,
        func: DrawFnId::DrawInstancedRoofSupportBeamShader,
    }, // leaf 0x1409bb5a2 fn 0x1409d98a0
    ShaderDispatch {
        name: "nani_wooden_plank",
        kind: PassKind::from_code(1),
        variant: 0,
        sort_key: 5500,
        func: DrawFnId::Env0,
    }, // leaf 0x1409bb5b8 fn 0x1409b12b0
    ShaderDispatch {
        name: "meadow_plant_shader",
        kind: PassKind::from_code(1),
        variant: 2,
        sort_key: 8001,
        func: DrawFnId::Env5,
    }, // leaf 0x1409bb91f fn 0x1409b68e0
    ShaderDispatch {
        name: "tree_trunk",
        kind: PassKind::from_code(1),
        variant: 0,
        sort_key: 1001,
        func: DrawFnId::TreeTrunk,
    }, // leaf 0x1409bb935 fn 0x1409c9830
    ShaderDispatch {
        name: "instanced_roof_spire_shader",
        kind: PassKind::from_code(1),
        variant: 0,
        sort_key: 3100,
        func: DrawFnId::DrawInstancedRoofSpireShader,
    }, // leaf 0x1409bb94b fn 0x1409d96a0
    ShaderDispatch {
        name: "butterfly_shader",
        kind: PassKind::from_code(1),
        variant: 4,
        sort_key: 7000,
        func: DrawFnId::DrawInstancedButterflyShader,
    }, // leaf 0x1409bb961 fn 0x1415547e0
    ShaderDispatch {
        name: "snow_particles_shader",
        kind: PassKind::from_code(1),
        variant: 0,
        sort_key: 6000,
        func: DrawFnId::Env3,
    }, // leaf 0x1409bba5a fn 0x1409b5140
    ShaderDispatch {
        name: "sheep_shader",
        kind: PassKind::from_code(2),
        variant: 0,
        sort_key: 6000,
        func: DrawFnId::Env2,
    }, // leaf 0x1409bba70 fn 0x1409afcc0
    ShaderDispatch {
        name: "solid_vertex_color_boat",
        kind: PassKind::from_code(1),
        variant: 2,
        sort_key: 5750,
        func: DrawFnId::Env1,
    }, // leaf 0x1409bba86 fn 0x1409b5c70
    ShaderDispatch {
        name: "nani_instanced_roof",
        kind: PassKind::from_code(1),
        variant: 0,
        sort_key: 3300,
        func: DrawFnId::DrawInstancedRoofShaderNani,
    }, // leaf 0x1409bba9c fn 0x1409d9070
    ShaderDispatch {
        name: "garden_lowpoly_flower",
        kind: PassKind::from_code(1),
        variant: 2,
        sort_key: 8003,
        func: DrawFnId::Env7,
    }, // leaf 0x1409bbab2 fn 0x1409b6150
    ShaderDispatch {
        name: "duck_shader",
        kind: PassKind::from_code(2),
        variant: 0,
        sort_key: 7001,
        func: DrawFnId::DrawDuckShader,
    }, // leaf 0x1409bbb9c fn 0x141555080
    ShaderDispatch {
        name: "instanced_ivy_flower_shader",
        kind: PassKind::from_code(1),
        variant: 2,
        sort_key: 9001,
        func: DrawFnId::DrawInstancedIvyFlowerShader,
    }, // leaf 0x1409bbe32 fn 0x1408dc1a0
    ShaderDispatch {
        name: "solid_vertex_color_garden_stone",
        kind: PassKind::from_code(1),
        variant: 0,
        sort_key: 15012,
        func: DrawFnId::Env26,
    }, // leaf 0x1409bc016 fn 0x1409b7d40
    ShaderDispatch {
        name: "meadow_flower",
        kind: PassKind::from_code(1),
        variant: 2,
        sort_key: 8002,
        func: DrawFnId::Env6,
    }, // leaf 0x1409bc06e fn 0x1409ae900
    ShaderDispatch {
        name: "nani_flower",
        kind: PassKind::from_code(1),
        variant: 2,
        sort_key: 8006,
        func: DrawFnId::Env10,
    }, // leaf 0x1409bc084 fn 0x1409b3cc0
    ShaderDispatch {
        name: "instanced_ivy_branch_shader",
        kind: PassKind::from_code(1),
        variant: 2,
        sort_key: 10000,
        func: DrawFnId::DrawInstancedIvyBranchShader,
    }, // leaf 0x1409bc09a fn 0x1408dbc50
    ShaderDispatch {
        name: "nani_preplaced_flower",
        kind: PassKind::from_code(1),
        variant: 2,
        sort_key: 8004,
        func: DrawFnId::Env8,
    }, // leaf 0x1409bc0c5 fn 0x1409b2f40
    ShaderDispatch {
        name: "nani_bush",
        kind: PassKind::from_code(1),
        variant: 2,
        sort_key: 8007,
        func: DrawFnId::Env11,
    }, // leaf 0x1409bc0d8 fn 0x1409b3780
    ShaderDispatch {
        name: "nani_solid_vertex_color_bricks",
        kind: PassKind::from_code(1),
        variant: 0,
        sort_key: 15000,
        func: DrawFnId::Env12,
    }, // leaf 0x1409bc0eb fn 0x1409b9550
    ShaderDispatch {
        name: "nani_roof_floor",
        kind: PassKind::from_code(1),
        variant: 0,
        sort_key: 15003,
        func: DrawFnId::Env16,
    }, // leaf 0x1409bc11f fn 0x1409b3270
    ShaderDispatch {
        name: "instanced_ivy_leaf_shader",
        kind: PassKind::from_code(1),
        variant: 2,
        sort_key: 9000,
        func: DrawFnId::DrawInstancedIvyLeafShader,
    }, // leaf 0x1409bc135 fn 0x1408dc180
    ShaderDispatch {
        name: "nani_solid_albedo_map",
        kind: PassKind::from_code(1),
        variant: 0,
        sort_key: 8005,
        func: DrawFnId::Env9,
    }, // leaf 0x1409bc163 fn 0x1409b9c10
    ShaderDispatch {
        name: "wall_brick_lod0",
        kind: PassKind::from_code(1),
        variant: 0,
        sort_key: 4000,
        func: DrawFnId::IndirectDrawBricksLod0,
    }, // leaf 0x1409bc179 fn 0x1408a53d0
    ShaderDispatch {
        name: "nani_stairs_floor",
        kind: PassKind::from_code(1),
        variant: 0,
        sort_key: 15003,
        func: DrawFnId::Env17,
    }, // leaf 0x1409bc18f fn 0x1409b4270
    ShaderDispatch {
        name: "solid_vertex_color_wooden_gate",
        kind: PassKind::from_code(1),
        variant: 0,
        sort_key: 15002,
        func: DrawFnId::Env14,
    }, // leaf 0x1409bc1a5 fn 0x1409b57c0
    ShaderDispatch {
        name: "nani_curved_halftimber",
        kind: PassKind::from_code(1),
        variant: 0,
        sort_key: 15002,
        func: DrawFnId::Env15,
    }, // leaf 0x1409bc1bb fn 0x1409b7200
    ShaderDispatch {
        name: "nani_wren",
        kind: PassKind::from_code(2),
        variant: 0,
        sort_key: 7001,
        func: DrawFnId::DrawWrenShader,
    }, // leaf 0x1409bc1d1 fn 0x141555ec0
    ShaderDispatch {
        name: "instanced_background_tree_shader",
        kind: PassKind::from_code(1),
        variant: 4,
        sort_key: 8000,
        func: DrawFnId::Env4,
    }, // leaf 0x1409bc1fe fn 0x1409b85d0
    ShaderDispatch {
        name: "solid_vertex_color",
        kind: PassKind::from_code(1),
        variant: 0,
        sort_key: 15004,
        func: DrawFnId::Env18,
    }, // leaf 0x1409bc214 fn 0x1409b5cf0
    ShaderDispatch {
        name: "nani_gothic_window_bricks",
        kind: PassKind::from_code(1),
        variant: 0,
        sort_key: 15001,
        func: DrawFnId::Env13,
    }, // leaf 0x1409bc22a fn 0x1409b6540
    ShaderDispatch {
        name: "splash_particle",
        kind: PassKind::from_code(1),
        variant: 0,
        sort_key: 15005,
        func: DrawFnId::Env19,
    }, // leaf 0x1409bc240 fn 0x1409b1ee0
    ShaderDispatch {
        name: "wall_brick_lod1",
        kind: PassKind::from_code(1),
        variant: 0,
        sort_key: 4001,
        func: DrawFnId::IndirectDrawBricksLod1,
    }, // leaf 0x1409bc256 fn 0x1408a4ca0
    ShaderDispatch {
        name: "ice_cracks",
        kind: PassKind::from_code(1),
        variant: 0,
        sort_key: 15006,
        func: DrawFnId::Env20,
    }, // leaf 0x1409bc26c fn 0x1409b3be0
    ShaderDispatch {
        name: "flag_roof_cloth",
        kind: PassKind::from_code(1),
        variant: 0,
        sort_key: 15007,
        func: DrawFnId::Env21,
    }, // leaf 0x1409bc282 fn 0x1409b8680
    ShaderDispatch {
        name: "nani_solid_vertex_color_window_glass",
        kind: PassKind::from_code(3),
        variant: 0,
        sort_key: 15015,
        func: DrawFnId::Env29,
    }, // leaf 0x1409bc315 fn 0x1409b98e0
    ShaderDispatch {
        name: "flag_small",
        kind: PassKind::from_code(1),
        variant: 0,
        sort_key: 15009,
        func: DrawFnId::Env23,
    }, // leaf 0x1409bc32b fn 0x1409b28c0
    ShaderDispatch {
        name: "water_x_paths",
        kind: PassKind::from_code(1),
        variant: 0,
        sort_key: 15024,
        func: DrawFnId::Env40,
    }, // leaf 0x1409bc527 fn 0x1409b4600
    ShaderDispatch {
        name: "flag_wall_cloth",
        kind: PassKind::from_code(1),
        variant: 0,
        sort_key: 15008,
        func: DrawFnId::Env22,
    }, // leaf 0x1409bc53d fn 0x1409b4860
    ShaderDispatch {
        name: "nani_solid_vertex_color",
        kind: PassKind::from_code(1),
        variant: 0,
        sort_key: 15011,
        func: DrawFnId::Env25,
    }, // leaf 0x1409bc553 fn 0x1409b6100
    ShaderDispatch {
        name: "nani_smoke",
        kind: PassKind::from_code(2),
        variant: 0,
        sort_key: 15010,
        func: DrawFnId::Env24,
    }, // leaf 0x1409bc569 fn 0x1409b9df0
    ShaderDispatch {
        name: "nani_uber",
        kind: PassKind::from_code(1),
        variant: 0,
        sort_key: 15014,
        func: DrawFnId::Env28,
    }, // leaf 0x1409bc57f fn 0x1409b3220
    ShaderDispatch {
        name: "solid_vertex_color_albedo_map",
        kind: PassKind::from_code(1),
        variant: 0,
        sort_key: 15013,
        func: DrawFnId::Env27,
    }, // leaf 0x1409bc595 fn 0x1409af390
    ShaderDispatch {
        name: "nani_balcony_door",
        kind: PassKind::from_code(1),
        variant: 0,
        sort_key: 15018,
        func: DrawFnId::Env32,
    }, // leaf 0x1409bc5ab fn 0x1409b2120
    ShaderDispatch {
        name: "nani_cloth",
        kind: PassKind::from_code(1),
        variant: 0,
        sort_key: 15019,
        func: DrawFnId::Env33,
    }, // leaf 0x1409bc5c1 fn 0x1409b4180
    ShaderDispatch {
        name: "nani_solid_albedo_map_emissive",
        kind: PassKind::from_code(3),
        variant: 0,
        sort_key: 15016,
        func: DrawFnId::Env30,
    }, // leaf 0x1409bc5d7 fn 0x1409b55e0
    ShaderDispatch {
        name: "nani_plaster",
        kind: PassKind::from_code(1),
        variant: 0,
        sort_key: 15020,
        func: DrawFnId::Env34,
    }, // leaf 0x1409bc5ed fn 0x1409b7560
    ShaderDispatch {
        name: "vertex_color_shader",
        kind: PassKind::from_code(1),
        variant: 4,
        sort_key: 15020,
        func: DrawFnId::Env35,
    }, // leaf 0x1409bc603 fn 0x1409b8340
    ShaderDispatch {
        name: "stairs_step",
        kind: PassKind::from_code(1),
        variant: 0,
        sort_key: 19000,
        func: DrawFnId::StairsDraw,
    }, // leaf 0x1409bc6d4 fn 0x14086a780
    ShaderDispatch {
        name: "nani_solid_vertex_color_window",
        kind: PassKind::from_code(1),
        variant: 0,
        sort_key: 15017,
        func: DrawFnId::Env31,
    }, // leaf 0x1409bc6e7 fn 0x1409b9aa0
    ShaderDispatch {
        name: "sparkles_shader",
        kind: PassKind::from_code(2),
        variant: 4,
        sort_key: 15020,
        func: DrawFnId::Env36,
    }, // leaf 0x1409bcd19 fn 0x1409b7fa0
    ShaderDispatch {
        name: "logo_animation",
        kind: PassKind::from_code(1),
        variant: 4,
        sort_key: 15021,
        func: DrawFnId::Env37,
    }, // leaf 0x1409bcd4c fn 0x1409b1b00
    ShaderDispatch {
        name: "rocky_terrain",
        kind: PassKind::from_code(1),
        variant: 0,
        sort_key: 19000,
        func: DrawFnId::Env43,
    }, // leaf 0x1409bcdaa fn 0x1409b8620
    ShaderDispatch {
        name: "logo_branches_animation",
        kind: PassKind::from_code(1),
        variant: 4,
        sort_key: 15022,
        func: DrawFnId::Env38,
    }, // leaf 0x1409bcdc0 fn 0x1409b9080
    ShaderDispatch {
        name: "nani_lantern_emissive",
        kind: PassKind::from_code(3),
        variant: 0,
        sort_key: 15023,
        func: DrawFnId::Env39,
    }, // leaf 0x1409bcdd6 fn 0x1409b1e90
    ShaderDispatch {
        name: "demo_terrain_overlay",
        kind: PassKind::from_code(1),
        variant: 4,
        sort_key: 60000,
        func: DrawFnId::Env52,
    }, // leaf 0x1409bd02a fn 0x1409b07f0
    ShaderDispatch {
        name: "sun_gizmo_shader",
        kind: PassKind::from_code(1),
        variant: 4,
        sort_key: 16000,
        func: DrawFnId::Env42,
    }, // leaf 0x1409bd040 fn 0x1409b1ab0
    ShaderDispatch {
        name: "path_pebble",
        kind: PassKind::from_code(1),
        variant: 0,
        sort_key: 19000,
        func: DrawFnId::PathPebbleDraw,
    }, // leaf 0x1409bd056 fn 0x1408e5a10
    ShaderDispatch {
        name: "loading_screen",
        kind: PassKind::from_code(1),
        variant: 4,
        sort_key: 15500,
        func: DrawFnId::Env41,
    }, // leaf 0x1409bd06c fn 0x1409b2290
    ShaderDispatch {
        name: "leaf_particle",
        kind: PassKind::from_code(1),
        variant: 0,
        sort_key: 19000,
        func: DrawFnId::LeafParticleDraw,
    }, // leaf 0x1409bd082 fn 0x140a8a090
    ShaderDispatch {
        name: "lilypad_flower_shader",
        kind: PassKind::from_code(1),
        variant: 2,
        sort_key: 19000,
        func: DrawFnId::LilypadFlowerDraw,
    }, // leaf 0x1409bd098 fn 0x1409da760
    ShaderDispatch {
        name: "reed_shader",
        kind: PassKind::from_code(1),
        variant: 2,
        sort_key: 19000,
        func: DrawFnId::ReedDraw,
    }, // leaf 0x1409bd0ae fn 0x1409da940
    ShaderDispatch {
        name: "nani_plant",
        kind: PassKind::from_code(1),
        variant: 2,
        sort_key: 19000,
        func: DrawFnId::Env44,
    }, // leaf 0x1409bd0c4 fn 0x1409b7ec0
    ShaderDispatch {
        name: "grass_shader",
        kind: PassKind::from_code(1),
        variant: 1,
        sort_key: 20000,
        func: DrawFnId::DrawGrassShader,
    }, // leaf 0x1409bd0da fn 0x140b4af70
    ShaderDispatch {
        name: "lilypad_shader",
        kind: PassKind::from_code(1),
        variant: 2,
        sort_key: 19000,
        func: DrawFnId::LilypadDraw,
    }, // leaf 0x1409bd0ed fn 0x1409da580
    ShaderDispatch {
        name: "grass_lod1_shader",
        kind: PassKind::from_code(1),
        variant: 1,
        sort_key: 20001,
        func: DrawFnId::DrawGrassLod1Shader,
    }, // leaf 0x1409bd103 fn 0x140b4b1d0
    ShaderDispatch {
        name: "billboard_plant_shader",
        kind: PassKind::from_code(1),
        variant: 0,
        sort_key: 21000,
        func: DrawFnId::Env45,
    }, // leaf 0x1409bd12a fn 0x1409b0fa0
    ShaderDispatch {
        name: "floor_shader",
        kind: PassKind::from_code(1),
        variant: 0,
        sort_key: 31000,
        func: DrawFnId::Env46,
    }, // leaf 0x1409bd140 fn 0x1409b8570
    ShaderDispatch {
        name: "water_snow_plane",
        kind: PassKind::from_code(1),
        variant: 0,
        sort_key: 32000,
        func: DrawFnId::Env20,
    }, // leaf 0x1409bd156 fn 0x1409b3be0
    ShaderDispatch {
        name: "distant_billboard_shader",
        kind: PassKind::from_code(1),
        variant: 4,
        sort_key: 60000,
        func: DrawFnId::Env51,
    }, // leaf 0x1409bd165 fn 0x1409b8250
    ShaderDispatch {
        name: "tree_billboard_shader",
        kind: PassKind::from_code(1),
        variant: 4,
        sort_key: 60000,
        func: DrawFnId::Env51,
    }, // leaf 0x1409bd165 fn 0x1409b8250
    ShaderDispatch {
        name: "dashed_line",
        kind: PassKind::from_code(1),
        variant: 4,
        sort_key: 15000,
        func: DrawFnId::Env57,
    }, // leaf 0x1409bd2a6 fn 0x1409b0b20
    ShaderDispatch {
        name: "billboard_forest_shader",
        kind: PassKind::from_code(1),
        variant: 0,
        sort_key: 50000,
        func: DrawFnId::Env48,
    }, // leaf 0x1409bd2bc fn 0x1409b0570
    ShaderDispatch {
        name: "nani_velocity_only",
        kind: PassKind::from_code(4),
        variant: 0,
        sort_key: 55000,
        func: DrawFnId::Env49,
    }, // leaf 0x1409bd2d2 fn 0x1409b9030
    ShaderDispatch {
        name: "timed_terrain_gizmo_shader",
        kind: PassKind::from_code(0),
        variant: 6,
        sort_key: 100000,
        func: DrawFnId::Env55,
    }, // leaf 0x1409bd341 fn 0x1409b3eb0
    ShaderDispatch {
        name: "cloud_shader",
        kind: PassKind::from_code(1),
        variant: 4,
        sort_key: 9000000,
        func: DrawFnId::Env53,
    }, // leaf 0x1409bd354 fn 0x1409b2580
    ShaderDispatch {
        name: "ice_plane",
        kind: PassKind::from_code(1),
        variant: 4,
        sort_key: 95000,
        func: DrawFnId::WaterPlanesDraw,
    }, // leaf 0x1409bd36a fn 0x1409d9bc0
    ShaderDispatch {
        name: "water_plane",
        kind: PassKind::from_code(1),
        variant: 4,
        sort_key: 95000,
        func: DrawFnId::WaterPlanesDraw,
    }, // leaf 0x1409bd36a fn 0x1409d9bc0
    ShaderDispatch {
        name: "star_shader",
        kind: PassKind::from_code(1),
        variant: 4,
        sort_key: 9100000,
        func: DrawFnId::Env54,
    }, // leaf 0x1409bd380 fn 0x1409b7110
    ShaderDispatch {
        name: "firefly_shader",
        kind: PassKind::from_code(5),
        variant: 0,
        sort_key: 90000,
        func: DrawFnId::DrawInstancedFireflyShader,
    }, // leaf 0x1409bd396 fn 0x1415549b0
    ShaderDispatch {
        name: "water_plane_refraction_copy",
        kind: PassKind::from_code(0),
        variant: 6,
        sort_key: 91000,
        func: DrawFnId::WaterPlanesDraw,
    }, // leaf 0x1409bd3c2 fn 0x1409d9bc0
    ShaderDispatch {
        name: "skydome_shader",
        kind: PassKind::from_code(1),
        variant: 4,
        sort_key: 10000000,
        func: DrawFnId::Env60,
    }, // leaf 0x1409bd4a7 fn 0x1409b8430
    ShaderDispatch {
        name: "normal_debug_shader",
        kind: PassKind::from_code(1),
        variant: 4,
        sort_key: 102000,
        func: DrawFnId::Env58,
    }, // leaf 0x1409bd4bd fn 0x1409b2800
    ShaderDispatch {
        name: "ivy_debug_lines_shader",
        kind: PassKind::from_code(1),
        variant: 4,
        sort_key: 103000,
        func: DrawFnId::Env59,
    }, // leaf 0x1409bd4d3 fn 0x1409b4540
    ShaderDispatch {
        name: "firefly_glow_shader",
        kind: PassKind::from_code(0),
        variant: 6,
        sort_key: 1000001,
        func: DrawFnId::DrawInstancedFireflyGlowShader,
    }, // leaf 0x1409bd4e9 fn 0x141554cf0
    ShaderDispatch {
        name: "ui_2d",
        kind: PassKind::from_code(1),
        variant: 4,
        sort_key: 100000,
        func: DrawFnId::Env35,
    }, // leaf 0x1409bd4ff fn 0x1409b8340
];

#[cfg(test)]
mod tests {
    use super::*;

    fn noop(_b: DrawBuilder<'_>, _i: &EntityDrawParams, _w: &mut World) {}

    fn e(shader: u64, key: i32) -> DrawEntry {
        (DrawItem::new(shader, key), noop)
    }

    fn row(flags: PassFlags) -> DrawRow {
        DrawRow {
            entity: Entity::PLACEHOLDER,
            vao: 0x55,
            shader: 0x66,
            transform: DrawItem::new(0, 0).params.transform,
            topology: None,
            flags,
        }
    }

    #[test]
    fn new_has_21_empty_lists() {
        let l = DrawLists::new();
        assert_eq!(DRAW_LIST_COUNT, 21);
        assert_eq!(l.total_len(), 0);
        for i in 0..DRAW_LIST_COUNT {
            assert!(l.list(i).is_empty());
        }
    }

    #[test]
    fn sort_is_stable_on_signed_i32_key() {
        let mut l = DrawLists::new();
        let v = l.list_mut(list::WIREFRAME);
        v.push(e(1, 5));
        v.push(e(2, -3));
        v.push(e(3, 5));
        v.push(e(4, -3));
        v.push(e(5, 0));
        l.sort();
        let order: Vec<u64> = l.list(list::WIREFRAME).iter().map(|x| x.0.shader).collect();
        assert_eq!(order, vec![2, 4, 5, 1, 3]);
    }

    #[test]
    fn sort_handles_lists_longer_than_the_insertion_threshold() {
        let mut l = DrawLists::new();
        for i in 0..40u64 {
            l.list_mut(list::UI).push(e(i, (i % 3) as i32));
        }
        l.sort();
        let keys: Vec<i32> = l.list(list::UI).iter().map(|x| x.0.sort_key).collect();
        let mut expect = keys.clone();
        expect.sort();
        assert_eq!(keys, expect);
        let mut last: Option<(i32, u64)> = None;
        for x in l.list(list::UI) {
            if let Some((k, s)) = last {
                if k == x.0.sort_key {
                    assert!(s < x.0.shader);
                }
            }
            last = Some((x.0.sort_key, x.0.shader));
        }
    }

    #[test]
    fn list_indices_match_the_observed_vec_offsets() {
        let ptr_off = |idx: usize| 0x18 * idx + 8;
        assert_eq!(ptr_off(list::SHADOW), 0x08);
        assert_eq!(ptr_off(list::GBUFFER), 0x20);
        assert_eq!(ptr_off(list::VELOCITY), 0x98);
        assert_eq!(ptr_off(list::FORWARD_OC2), 0xf8);
        assert_eq!(ptr_off(list::LATE_FORWARD), 0x140);
        assert_eq!(ptr_off(list::REFRACTION), 0x158);
        assert_eq!(ptr_off(list::PRE_TAA), 0x188);
        assert_eq!(ptr_off(list::POST_TAA), 0x1a0);
        assert_eq!(ptr_off(list::UI), 0x1d0);
        assert_eq!(ptr_off(list::WIREFRAME), 0x1e8);
    }

    #[test]
    fn dispatch_table_has_the_87_decoded_names_without_duplicates() {
        assert_eq!(DISPATCH.len(), 87);
        let mut names: Vec<&str> = DISPATCH.iter().map(|d| d.name).collect();
        names.sort();
        names.dedup();
        assert_eq!(names.len(), 87);
        // 85 个叶子：`distant_billboard_shader`/`tree_billboard_shader` 与 `ice_plane`/`water_plane` 各共用一个叶子。
        assert_eq!(DISPATCH.iter().filter(|d| d.sort_key == 60000).count(), 3);
    }

    #[test]
    fn dispatch_rows_match_machine_code_constants() {
        let d = lookup("floor_shader").unwrap();
        assert_eq!(
            (d.kind, d.variant, d.sort_key),
            (PassKind::Opaque, 0, 31000)
        );
        assert_eq!(d.func, DrawFnId::Env46);
        assert_eq!(d.func.addr(), 0x1409b8570);
        let g = lookup("grass_lod1_shader").unwrap();
        assert_eq!(
            (g.variant, g.sort_key, g.func.addr()),
            (1, 20001, 0x140b4b1d0)
        );
        let t = lookup("tree_canopy").unwrap();
        assert_eq!((t.variant, t.sort_key), (3, 1000));
        let w = lookup("nani_solid_vertex_color_window_glass").unwrap();
        assert_eq!((w.kind, w.sort_key), (PassKind::Emissive, 15015));
        let v = lookup("nani_velocity_only").unwrap();
        assert_eq!((v.kind, v.sort_key), (PassKind::VelocityOnly, 55000));
        let f = lookup("firefly_shader").unwrap();
        assert_eq!((f.kind, f.sort_key), (PassKind::LateForward, 90000));
        let s = lookup("skydome_shader").unwrap();
        assert_eq!((s.variant, s.sort_key), (4, 10000000));
        assert!(lookup("not_a_shader").is_none());
    }

    #[test]
    fn variants_are_material_ids() {
        use crate::systems::render_loop::draw_passes::material_id::MaterialId as M;
        assert_eq!(lookup("grass_shader").unwrap().variant, M::Grass as u32);
        assert_eq!(lookup("nani_plant").unwrap().variant, M::Plant as u32);
        assert_eq!(
            lookup("vertex_color_shader").unwrap().variant,
            M::Forward as u32
        );
    }

    #[test]
    fn opaque_goes_to_gbuffer_and_shadow_and_oc2() {
        let d = lookup("nani_uber").unwrap();
        let (l, s) = select_lists(d, &PassFlags::default());
        assert_eq!(l, vec![list::GBUFFER, list::SHADOW]);
        assert_eq!(s, Some(0));
        let (l, _) = select_lists(
            d,
            &PassFlags {
                dont_cast_shadow: true,
                occlusion_culled: true,
                ..Default::default()
            },
        );
        assert_eq!(l, vec![list::GBUFFER, list::GBUFFER_OC2]);
    }

    #[test]
    fn forward_material_goes_to_forward_lists() {
        let d = lookup("vertex_color_shader").unwrap();
        let (l, s) = select_lists(
            d,
            &PassFlags {
                occlusion_culled: true,
                ..Default::default()
            },
        );
        assert_eq!(l, vec![list::FORWARD, list::SHADOW, list::FORWARD_OC2]);
        assert_eq!(s, Some(4));
    }

    #[test]
    fn velocity_kinds_and_stencil_encodings() {
        let (l, s) = select_lists(lookup("sheep_shader").unwrap(), &PassFlags::default());
        assert_eq!(l, vec![list::GBUFFER_VELOCITY, list::SHADOW]);
        assert_eq!(s, Some(0));
        let (l, s) = select_lists(
            lookup("duck_shader").unwrap(),
            &PassFlags {
                occlusion_culled: true,
                ..Default::default()
            },
        );
        assert_eq!(
            l,
            vec![
                list::GBUFFER_VELOCITY,
                list::SHADOW,
                list::GBUFFER_VELOCITY_OC2
            ]
        );
        assert_eq!(s, Some(0));
        let (l, s) = select_lists(lookup("sparkles_shader").unwrap(), &PassFlags::default());
        assert_eq!(l, vec![list::FORWARD_VELOCITY, list::SHADOW]);
        assert_eq!(s, Some(4));
        // kind 3：模板 = variant | 0x80，列表 = emissive(+OC/2)
        let (l, s) = select_lists(
            lookup("nani_lantern_emissive").unwrap(),
            &PassFlags {
                occlusion_culled: true,
                ..Default::default()
            },
        );
        assert_eq!(
            l,
            vec![
                list::GBUFFER_EMISSIVE,
                list::SHADOW,
                list::GBUFFER_EMISSIVE_OC2
            ]
        );
        assert_eq!(s, Some(0x80));
        // kind 4：无阴影，模板 None
        let (l, s) = select_lists(
            lookup("nani_velocity_only").unwrap(),
            &PassFlags {
                occlusion_culled: true,
                ..Default::default()
            },
        );
        assert_eq!(l, vec![list::VELOCITY, list::VELOCITY_OC2]);
        assert_eq!(s, None);
        // kind 5
        let (l, s) = select_lists(lookup("firefly_shader").unwrap(), &PassFlags::default());
        assert_eq!(l, vec![list::LATE_FORWARD, list::SHADOW]);
        assert_eq!(s, Some(4));
    }

    #[test]
    fn explicit_pass_markers_override_everything_in_priority_order() {
        let d = lookup("nani_uber").unwrap();
        let f = PassFlags {
            designated: Some(16),
            ui: true,
            wireframe: true,
            occlusion_culled: true,
            ..Default::default()
        };
        assert_eq!(select_lists(d, &f).0, vec![16]);
        let f = PassFlags {
            ui: true,
            wireframe: true,
            ..Default::default()
        };
        assert_eq!(select_lists(d, &f).0, vec![list::UI]);
        let f = PassFlags {
            wireframe: true,
            ..Default::default()
        };
        assert_eq!(select_lists(d, &f).0, vec![list::WIREFRAME]);
        // kind 0 + Designated 通过（模板 None）
        let w = lookup("water_plane_refraction_copy").unwrap();
        assert_eq!(
            select_lists(
                w,
                &PassFlags {
                    designated: Some(14),
                    ..Default::default()
                }
            ),
            (vec![14], None)
        );
    }

    #[test]
    #[should_panic(expected = "An object in the main pass must have a material ID")]
    fn kind0_without_a_pass_marker_panics() {
        let _ = select_lists(
            lookup("firefly_glow_shader").unwrap(),
            &PassFlags::default(),
        );
    }

    #[test]
    #[should_panic(expected = "doesn't have a draw function")]
    fn unknown_shader_name_panics_with_the_original_message() {
        let mut l = DrawLists::new();
        push_row(&mut l, &row(PassFlags::default()), "no_such_shader");
    }

    #[test]
    fn push_row_fills_item_fields_like_the_machine_code() {
        let mut l = DrawLists::new();
        push_row(&mut l, &row(PassFlags::default()), "grass_shader");
        let it = &l.list(list::GBUFFER)[0].0;
        assert_eq!((it.vao, it.shader, it.sort_key), (0x55, 0x66, 20000));
        assert_eq!(it.stencil_ref, Some(1));
        assert_eq!(it.params.shader_name, "grass_shader");
        assert_eq!(l.list(list::SHADOW).len(), 1);
        assert_eq!(l.total_len(), 2);
    }

    #[test]
    fn recovered_bodies_flag() {
        assert!(DrawFnId::Env4.is_recovered());
        // Env6 已由 `draw_callbacks_a` 恢复（d_cb_a）。
        assert!(DrawFnId::Env6.is_recovered());
    }
}
