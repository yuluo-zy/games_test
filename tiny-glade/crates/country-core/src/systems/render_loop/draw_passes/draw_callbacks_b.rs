//! `draw_callbacks_b.rs` — `build_draw_lists` 回调（`closure_env$N` 与 `draw_shaders::*` 直接函数）的**后一半**本体（d_cb_b）。
//!
//! 回调签名（O）：`fn(DrawBuilder, &EntityDrawParams, &mut World)`（`UnsafeWorldCell` 即 `&mut World`）。机器码里闭包把
//! `EntityDrawParams.entity` 与 `World` 存进 builder（`[rbp+0xa8]/[rbp+0xb0]`），再逐步 `bind_*` / `write_uniform` / 终结绘制。
//!
//! # 取值路径（O）
//! * 资源：`Components::get_resource_id(TypeId)` → `[world+0x210]` 稀疏表 → `[world+0x1e0]` 资源槽，缺失 → `Option::unwrap` panic
//!   （文案 `called `Option::unwrap()` on a `None` value`）。= `World::get_resource::<T>().unwrap()`。
//! * 组件：`world.entities` 取位置（失败 → `Result::unwrap` panic），`Components::get_id` + `Table::get_component`
//!   （失败 → `Option::unwrap` panic）= `world.get_entity(e).unwrap().get::<T>().unwrap()`；
//!   `get_id` + 原型稀疏集查询（`cmp [+8*id], 0; setne`）= `EntityRef::contains::<T>()`（Env18）。
//! * **类型名不是猜的**：TypeId 的两个 u64 常量在 EXE 里同时出现在 `init_state<T>` / `register_info<T>` /
//!   `component_ids<T>` / `register_resource_with<..T..>` 等泛型实例体内，据此由 `procs.tsv` 反查出 T
//!   （脚本：`/tmp/dcb/tid.py` 同法，报告 `B_d_cb_b.md` §2 列出全部 TypeId → 类型名）。
//!
//! # 真实类型 vs 视图类型
//! 真实类型已在 crate 里且字段可读的（`SemiTransparent`、`FlatLighting`、`ColorId`、`WallHighlight`、
//! `CursorTerrainRaycast`、`GladeSettings`、`GpuWorld`、`AssetSsboLibrary`）直接使用。字段布局仍是 typed gap
//! （`MainCamera.raw: Gap`、`ActiveSession`、`Weather`、`EditingHighlight`、`DebugGui`、`LogoAnimation`、`RoofFlagCloth`、
//! `PrevTransform`…）、没有 `Resource` 派生或在 crate 里根本不存在的类型，只在本文件里放一个**只含被读字段**的 `*View`
//! 视图类型（字段偏移 = 机器码观测），名字后缀 `View`；`Duck`/`DuckState`/`CritterControl`/`EraserSizeMultiplier`/
//! `RoofHighlight`/`InstancedDrawFromPlacementData` 在 crate 里不存在，直接按 PDB 名放 stub。真实类型落地后把读取点换过去即可
//! （每个视图的文档写明真实类型与偏移）。
//!
//! # nani_draw
//! 原版 `nani_draw` 经 builder 里的 `World` 取 `GpuWorld`、`AssetSsboLibrary`（均为 `Resource`）。[`nani_draw_world`] 做同样的事；
//! `RenderGraph` 在 crate 里是占位类型，没有 `Resource` 身份，这里用一次性 `RenderGraph::default()`（I：`get_rg_buffer`
//! 命中已导入句柄时不碰它）。

// Preserve the original instruction-level expression order/NaN policy, API call shapes, and evidence/test transcription. See README.md.
#![allow(clippy::field_reassign_with_default, clippy::redundant_closure)]
use alloc::vec::Vec;
use core::sync::atomic::{AtomicBool, Ordering};

use bevy_ecs::component::Component;
use bevy_ecs::entity::Entity;
use bevy_ecs::resource::Resource;
use bevy_ecs::world::World;
use glam::{Quat, Vec3};

use crate::asset_libraries::ssbo_library::{AssetSsboLibrary, SsboId};
use crate::components::{FlatLighting, SemiTransparent};
use crate::name_hash::NameHash;
use crate::render::nani::draw_ext::NaniDrawExt;
use crate::render::nani::gpu_world::{GpuWorld, GpuWorldSubsetKind};
use crate::render::pass_builder::bind_resources::{
    RtBvhSsbo, RtTriangleMetaSsbo, RtTrianglesSsbo, WallColorIdSsbo,
};
use crate::render::pass_builder::draw_builder::{BrickEraseHighlightInputs, DrawBuilder};
use crate::render::pass_builder::draw_list::EntityDrawParams;
use crate::render::pass_builder::pass_resource_access::SsboType;
use crate::resources::color_id::ColorId;
use crate::resources::cursor_terrain_raycast::CursorTerrainRaycast;
use crate::resources::glade_settings::GladeSettings;
use crate::resources::walls::WallHighlight;
use crate::rhapsody_contract::ManagedHandle;
use crate::rhapsody_types::RenderGraph;
use crate::systems::render_loop::draw_passes::d_main_support::UNWRAP_NONE;
use crate::systems::render_loop::draw_passes::draw_shaders::clearing_shaders::{
    TreeCanopyInputs, TreeTrunkInputs, tree_canopy, tree_trunk,
};
use crate::systems::render_loop::draw_passes::draw_shaders::common_includes::{
    bind_disk, bind_preset,
};
use crate::systems::render_loop::draw_passes::draw_shaders::instanced_critters::{
    DuckInputs, draw_duck_shader, draw_instanced_firefly_glow_shader,
    draw_instanced_firefly_shader, draw_wren_shader,
};
use crate::systems::render_loop::draw_passes::draw_shaders::instanced_roof::{
    RoofNaniParam, draw_instanced_roof_shader_nani, draw_instanced_roof_spire_shader,
    draw_instanced_roof_support_beam_shader,
};
use crate::systems::render_loop::draw_passes::draw_shaders::leaf_particle_draw::leaf_particle_draw;
use crate::systems::render_loop::draw_passes::draw_shaders::rocky_terrain_draw::rocky_terrain_shader;
use crate::systems::render_loop::draw_passes::draw_shaders::ssbo_tags::{
    DrawIndexedIndirectTag, DrawSsbo, InstanceBufferView, ReadSsboExt, RoofColorIdTag,
    SsboInstanceView,
};
use crate::systems::render_loop::draw_passes::draw_shaders::water_planes_draw::{
    WaterPlanesInputs, all_nonzero, water_planes_draw,
};

// =====================================================================================================================
// SSBO 标签（槽名 / SsboId 由 `dsh_ssbo_info.py` 从 EXE 抽取；测试用 xxh3 复算）
// =====================================================================================================================

macro_rules! ssbo_tag {
    ($(#[$m:meta])* $tag:ident, $ty:literal, $slot:literal, $id:expr) => {
        $(#[$m])*
        #[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
        pub struct $tag;
        impl SsboType for $tag {
            fn rg_buffer_hash() -> u64 {
                $id
            }
        }
        impl DrawSsbo for $tag {
            const SLOT_NAME: &'static str = $slot;
            const TYPE_NAME: &'static str = $ty;
        }
    };
}

ssbo_tag!(
    /// `compute_passes::spawn_boats::BoatSsbo`（`read_storage_buffer` 实例 0x141631690）。
    BoatTag,
    "country_core::systems::render_loop::compute_passes::spawn_boats::BoatSsbo",
    "Boats",
    0x4f69c51e83e1d670
);
ssbo_tag!(
    /// `compute_passes::compute_garden::Flower03Ssbo`（0x14162b690）。
    Flower03Tag,
    "country_core::systems::render_loop::compute_passes::compute_garden::Flower03Ssbo",
    "Small lowpoly flowers",
    0xf7c8c9c2eb9e16fb
);
ssbo_tag!(
    /// `compute_passes::compute_garden::GardenStoneSsbo`（0x14162dd90）。
    GardenStoneTag,
    "country_core::systems::render_loop::compute_passes::compute_garden::GardenStoneSsbo",
    "Garden Stone",
    0x45a92b71a96e360f
);
ssbo_tag!(
    /// `resources::color_id::HalftimberColorIdSsbo`（0x141630190）。
    HalftimberColorIdTag,
    "country_core::resources::color_id::HalftimberColorIdSsbo",
    "HalftimberColorIdSsbo",
    0x0cf3b7d1045fdc98
);
ssbo_tag!(
    /// `resources::sparkle::SparkleSsbo`（0x141631990；类型名长 0x2d，Env36 里 `bcmp` 比对）。
    SparkleTag,
    "country_core::resources::sparkle::SparkleSsbo",
    "SparkleSsbo",
    0x2bf3b1a4ac675864
);

// =====================================================================================================================
// 视图 / stub 类型（只含被读字段；偏移为机器码观测）
// =====================================================================================================================

/// `MainCamera`（TypeId 0xbba451ee6a08a3ff/0x8d008d15febcf29d，`systems::camera_rig::MainCamera`，真实类型 `raw: Gap`）。
/// 读取：`CameraMatrices::eye_position`（0x140c96d40：`[+0xc0..+0xf0]` 与 0 相乘后加 `[+0xf0]` → `eye = [+0xf0].xyz`）、
/// 字节 `+0x4d0`（`MainCamera` 文档称模式字节；树 shader 里叫 season，I）、按该字节索引的子结构里 `f32 @+0x104`。
#[derive(Resource, Clone, Copy, Debug, Default, PartialEq)]
pub struct MainCameraView {
    /// `eye_position()`（O）。
    pub eye: Vec3,
    /// `byte @+0x4d0`。
    pub season: u8,
    /// 六个子结构各自的 `f32 @+0x104`（偏移表 `SEASON_OFFSETS`，O：0x142ac0cf0）。
    pub season_params: [f32; 6],
}

/// `Weather`（TypeId 0xd2f519263066413b/0x5192b15f962559e4，真实类型 `Weather(pub Gap)`）：`f32 @+4` = 风（O）。
#[derive(Resource, Clone, Copy, Debug, Default, PartialEq)]
pub struct WeatherView {
    pub wind: f32,
}

/// `TreeCanopyMaxCullDistance`（TypeId 0x1b4e385d376c1cc0/0x8c38d6ede2a437ec）：真实类型是 `(pub f32)` 且**没有** `Resource`
/// 派生；前一版文档称机器码读 `f32 @+4`（与单字段 `f32 @0` 不符，U）。视图保存该 f32。
#[derive(Resource, Clone, Copy, Debug, Default, PartialEq)]
pub struct TreeCanopyMaxCullDistanceView(pub f32);

/// `EditingHighlight`（TypeId 0x6efcda1c77e106b4/0x7c999c10e7fa637f，真实类型 `raw: Gap`）：`byte @+0xc`。
pub use crate::resources::editing_highlight::EditingHighlight as EditingHighlightView;

/// `DebugGui`（TypeId 0x48b321ef779dc301/0xbd65767194d9ddc8，真实类型 `raw: Gap`）：`byte @+0xfe`。
pub use crate::resources::gui::DebugGui as DebugGuiView;

/// `systems::wall::eraser::EraserSizeMultiplier`（TypeId 0x0ae77a86a2c6cefc/0xf5509a56b32e81d0）：首个 `f32`。crate 里不存在。
pub use crate::resources::walls::EraserSizeMultiplier;

/// `resources::roofs::roof_highlight::RoofHighlight`（TypeId 0x9311cf2688353878/0xfd54118f9a9ee3d4）：`u32 @+0`、`tag @+8`。
#[derive(Resource, Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct RoofHighlight {
    pub value: u32,
    pub tag: u8,
}

/// `ActiveSession`（TypeId 0x74430dfcc6d8719b/0xca5591f7e464509c，真实类型 `glade: Gap`）：`byte @+0x58`（变体）与
/// 字节序列（`ptr @+8`、`len @+0x10`）。
#[derive(Resource, Clone, Debug, Default, PartialEq, Eq)]
pub struct ActiveSessionView {
    pub variant: u8,
    pub glade_bytes: Vec<u8>,
}

/// `LogoAnimation`（TypeId 0x98df08dcfa7ea5ba/0x67b117a60e632bc3；真实类型没有 `Resource` 派生）：`f32 @+8`、`f32 @+0x10`。
#[derive(Resource, Clone, Copy, Debug, Default, PartialEq)]
pub struct LogoAnimationView {
    pub f8: f32,
    pub f10: f32,
}

/// `RoofFlagCloth`（组件，TypeId 0x983a5c6eded1c4e5/0x929a707bdcc6a79c，真实类型 `(pub Gap)`）：`u32 @+0`。
#[derive(Component, Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct RoofFlagClothView(pub u32);

/// `resources::critters::CritterControl`（TypeId 0xa2127765959c8b85/0xf078ccfbcec1f347，crate 里不存在）：`f32 @+0` 萤火虫强度。
#[derive(Resource, Clone, Copy, Debug, Default, PartialEq)]
pub struct CritterControl {
    pub intensity: f32,
}

/// `components::drawable::PrevTransform`（TypeId 0xa0da022fc84ca3d9/0xfc62e38c26a7c4c9，真实类型 `(pub Gap)`）：
/// 鸭 shader 读它的 `Transform` 布局（旋转 @0、平移 @0x10、缩放 @0x1c）。
#[derive(Component, Clone, Copy, Debug, PartialEq)]
pub struct PrevTransformView {
    pub rotation: Quat,
    pub translation: Vec3,
    pub scale: Vec3,
}

/// `resources::critters::Duck`（TypeId 0x272a1f294225ce7f/0xd79870fafff409bc，crate 里不存在）：`f32 @+0xf0/+0xf4/+0xf8`、`u8 @+0xfc`。
#[derive(Component, Clone, Copy, Debug, Default, PartialEq)]
pub struct Duck {
    pub f_f0: f32,
    pub f_f4: f32,
    pub f_f8: f32,
    pub b_fc: bool,
}

/// `resources::critters::DuckState`（TypeId 0x3a8f7d4d3ac26bd0/0xff6d570a464cbf09，枚举，首字节判别）。
#[derive(Component, Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct DuckState(pub u8);

/// `asset_libraries::placement_data_library::InstancedDrawFromPlacementData`（组件，TypeId 0xed76c1eeabfaeba0/0x282debdb03116a9c）：
/// `u64 @+0` = `GlobalId`（传给 `draw_instanced_from_placement_data`）。crate 里不存在。
#[derive(Component, Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct InstancedDrawFromPlacementData(pub u64);

/// `RockyTerrainMeshVertCount`（TypeId 0xfa44538b95ae76c6/0x12146c4ea814069b；真实类型 `(pub u32)` 无 `Resource` 派生）。
#[derive(Resource, Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct RockyTerrainMeshVertCountView(pub u32);

/// Env36 的稀疏实例缓冲：`SparseInstanceBuffer<SparkleSsbo>::to_draw_multi_instanced()` 的结果。`SparkleSsbo` 在 crate 里是
/// `(pub Gap)`，没有 `SparseSsbo` 实现，无法经 `AssetSsboLibrary::get` 取到真实缓冲，故由视图携带（U）。
#[derive(Resource, Clone, Debug, Default, PartialEq, Eq)]
pub struct SparkleDrawRangesView(pub Vec<[u32; 2]>);

/// `water_planes_draw` 的进程级布尔（`.data` 0x143634d90，O：为真 → `discard`）。
pub static WATER_PLANES_DISABLED: AtomicBool = AtomicBool::new(false);

// =====================================================================================================================
// World 取值助手
// =====================================================================================================================

fn res<T: Resource>(w: &World) -> &T {
    w.get_resource::<T>().expect(UNWRAP_NONE)
}

fn comp<T: Component>(w: &World, e: Entity) -> &T {
    w.get_entity(e).unwrap().get::<T>().expect(UNWRAP_NONE)
}

/// `nani_draw`（0x140904ae0）的 World 版：取 `GpuWorld` / `AssetSsboLibrary` 资源（缺失 → `Option::unwrap` panic）。
pub fn nani_draw_world(b: DrawBuilder<'_>, kind: GpuWorldSubsetKind, w: &World) {
    let gw = res::<GpuWorld>(w);
    let lib = res::<AssetSsboLibrary>(w);
    let mut rg = RenderGraph::default();
    b.nani_draw(kind, gw, lib, &mut rg);
}

/// `AssetSsboLibrary` → [`SsboInstanceView`]（`FrozenMap::get` + 名字 + 实例缓冲；`lookup(NameHash(id))`）。
pub struct LibView<'a>(pub &'a AssetSsboLibrary);

impl SsboInstanceView for LibView<'_> {
    fn lookup(&self, ssbo_id: u64) -> Option<(&str, Option<InstanceBufferView>)> {
        self.0
            .lookup(SsboId(NameHash(ssbo_id)))
            .map(|(name, info)| {
                (
                    name,
                    info.map(|i| InstanceBufferView {
                        instance_count: i.instance_count as u64,
                        handle: ManagedHandle(i.buffer.id),
                    }),
                )
            })
    }
}

/// `packet.state` 第 0 字节 `(b & 0xf3) | v`（O：`and al, -0xd; or al, v`）。
pub fn cull_state(state: u32, v: u8) -> u32 {
    (state & !0x0c) | v as u32
}

fn set_cull(b: &mut DrawBuilder<'_>, v: u8) {
    let p = b.packet_mut();
    p.state.0 = cull_state(p.state.0, v);
}

// =====================================================================================================================
// 只有 nani_draw 的闭包 / 少量绑定 + nani_draw
// =====================================================================================================================

/// 生成 `<body>(b, nani)`（纯逻辑，便于测试）与 `<cb>(b, p, w)`（回调）。
macro_rules! nani_env {
    ($(#[$m:meta])* $cb:ident, $body:ident, $kind:ident, |$b:ident| $pre:block) => {
        $(#[$m])*
        pub fn $body<'p>(
            $b: DrawBuilder<'p>,
            nani: impl FnOnce(DrawBuilder<'p>, GpuWorldSubsetKind),
        ) {
            let b = $pre;
            nani(b, GpuWorldSubsetKind::$kind);
        }
        $(#[$m])*
        pub fn $cb(b: DrawBuilder<'_>, _p: &EntityDrawParams, w: &mut World) {
            $body(b, |b, k| nani_draw_world(b, k, w));
        }
    };
}

nani_env!(
    /// closure_env$25（0x1409b6100，`nani_draw(SolidVertexColor)`）。
    env25, env25_body, SolidVertexColor, |b| { b }
);
nani_env!(
    /// closure_env$49（0x1409b9030，`nani_draw(VelocityOnly = 0x19)`）。
    env49, env49_body, VelocityOnly, |b| { b }
);
nani_env!(
    /// closure_env$44（0x1409b7ec0）：预设 0xf → `nani_draw(Plant)`。
    env44, env44_body, Plant, |b| { bind_preset(b, 0xf, 0x035280b96dab13e6) }
);
nani_env!(
    /// closure_env$31（0x1409b9aa0）：两张磁盘纹理 → `nani_draw(SolidVertexColorWindow)`。
    env31, env31_body, SolidVertexColorWindow, |b| {
        let b = bind_disk(b, 0x5b9a9defbd41da38, 0x1aca8782e36dbd0e);
        bind_disk(b, 0xa041c59f6ca9394d, 0xa944a09a642557c2)
    }
);
nani_env!(
    /// closure_env$9（0x1409b9c10）：预设 9/0xa + 一张磁盘 → `nani_draw(SolidAlbedoMap)`。
    env9, env9_body, SolidAlbedoMap, |b| {
        let b = bind_preset(b, 9, 0x61b08b5e0470c5f2);
        let b = bind_preset(b, 0xa, 0x297d1e4cdae8f657);
        bind_disk(b, 0xc669ad39661fa107, 0xddedb57bb5d9df46)
    }
);
nani_env!(
    /// closure_env$29（0x1409b98e0）：`state.byte0 = (b & 0xf3) | 4`（`and al,-0xd; or al,4`），两张磁盘 →
    /// `nani_draw(SolidVertexColorWindowGlass)`。
    env29, env29_body, SolidVertexColorWindowGlass, |b| {
        let mut b = b;
        set_cull(&mut b, 4);
        let b = bind_disk(b, 0x1b3a0e97773fd725, 0x860759fcb9b13fc2);
        bind_disk(b, 0x944782cd1ef22b9a, 0xe69be10ca2ab87bf)
    }
);
nani_env!(
    /// closure_env$24（0x1409b9df0）：`state.byte0 = (b & 0xf3) | 4` → `nani_draw(Smoke = 0x15)`。
    env24, env24_body, Smoke, |b| {
        let mut b = b;
        set_cull(&mut b, 4);
        b
    }
);
nani_env!(
    /// closure_env$15（0x1409b7200）：`state.byte0 = (b & 0xf3) | 8`，五张磁盘 → `nani_draw(CurvedHalfTimber = 0x18)`。
    env15, env15_body, CurvedHalfTimber, |b| {
        let mut b = b;
        set_cull(&mut b, 8);
        let b = bind_disk(b, 0x4a2b687d0c1ecf58, 0xd0cc743d37cfaefb);
        let b = bind_disk(b, 0x5b9a9defbd41da38, 0x41748af61a225694);
        let b = bind_disk(b, 0xf1dbf278d4f6847b, 0xfe5687d6882edbe2);
        let b = bind_disk(b, 0x1b611d8a07dfd6ba, 0x7d2e85e59f64f385);
        bind_disk(b, 0x239bc9f60a193174, 0xf5ee046217439f64)
    }
);

/// closure_env$12/$13 共同的前半：`uniform<i32>(0x052e7aa4e36b047b, 0)`、Halftimber 颜色 SSBO、磁盘、预设 5/9/0xa、
/// 墙色定制、屋顶砖磁盘、屋顶颜色 SSBO。返回 builder（后续 `$13` 追加砖擦除高亮）。
fn halftimber_roof_common<'p>(b: DrawBuilder<'p>) -> DrawBuilder<'p> {
    let b = b.uniform(0x052e7aa4e36b047b, 0i32);
    let b = b.read_ssbo::<HalftimberColorIdTag>(0xecece750b6dd7eda);
    let b = bind_disk(b, 0x5b9a9defbd41da38, 0x41748af61a225694);
    let b = bind_preset(b, 5, 0xed502fff6dc4021f);
    let b = bind_preset(b, 9, 0x61b08b5e0470c5f2);
    let b = bind_preset(b, 0xa, 0x297d1e4cdae8f657);
    let b = b.bind_wall_color_customization::<WallColorIdSsbo>();
    let b = bind_disk(b, 0x7db9e7483fa24078, 0xaa8f0490dba0d03c);
    b.read_ssbo::<RoofColorIdTag>(0xd56729333fc94f92)
}

/// closure_env$12（0x1409b9550）→ `nani_draw(SolidVertexColorBricks = 6)`。
pub fn env12_body<'p>(b: DrawBuilder<'p>, nani: impl FnOnce(DrawBuilder<'p>, GpuWorldSubsetKind)) {
    nani(
        halftimber_roof_common(b),
        GpuWorldSubsetKind::SolidVertexColorBricks,
    );
}
pub fn env12(b: DrawBuilder<'_>, _p: &EntityDrawParams, w: &mut World) {
    env12_body(b, |b, k| nani_draw_world(b, k, w));
}

/// closure_env$13（0x1409b6540）：同 `$12`，再 `bind_brick_erase_highlight`（四个资源由 World 读出）→
/// `nani_draw(GothicWindowBricks = 7)`。
pub fn env13_body<'p>(
    b: DrawBuilder<'p>,
    brick: &BrickEraseHighlightInputs,
    nani: impl FnOnce(DrawBuilder<'p>, GpuWorldSubsetKind),
) {
    let b = halftimber_roof_common(b).bind_brick_erase_highlight(brick);
    nani(b, GpuWorldSubsetKind::GothicWindowBricks);
}
pub fn env13(b: DrawBuilder<'_>, _p: &EntityDrawParams, w: &mut World) {
    let brick = brick_erase_inputs(w);
    env13_body(b, &brick, |b, k| nani_draw_world(b, k, w));
}

/// `bind_brick_erase_highlight`（0x14090be10）的四个资源（O 读取序：EditingHighlight、CursorTerrainRaycast、
/// EraserSizeMultiplier、DebugGui；缺失 → `Option::unwrap` panic）。
pub fn brick_erase_inputs(w: &World) -> BrickEraseHighlightInputs {
    let enabled = res::<EditingHighlightView>(w).enabled;
    let cursor = res::<CursorTerrainRaycast>(w);
    let radius = res::<EraserSizeMultiplier>(w).0;
    let suppress = res::<DebugGuiView>(w).suppress;
    BrickEraseHighlightInputs {
        enabled,
        cursor_position: (cursor.tag & 1 != 0).then_some(cursor.pos),
        radius,
        suppress,
    }
}

/// 墙高亮 → `(value, tag)`（O：`tag = byte[+0x10]`；`edi = tag != 0 ? dword[+8] : 0`）。真实 `WallHighlight` 的
/// `ty`/`wall_id` 与该偏移的对应是 I。
pub fn wall_highlight_param(h: &WallHighlight) -> (i32, i32) {
    let tag = h.ty;
    let value = if tag != 0 {
        h.wall_id.map_or(0, |id| id.0 as u32)
    } else {
        0
    };
    (value as i32, tag as i32)
}

/// closure_env$34（0x1409b7560）的世界输入（读取序：WallHighlight、EditingHighlight、CursorTerrainRaycast、
/// EraserSizeMultiplier）。
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Env34Inputs {
    pub wall_value: i32,
    pub wall_tag: i32,
    pub editing_enabled: bool,
    pub cursor: Vec3,
    pub radius: f32,
}

/// closure_env$34：`state.byte0 = (b & 0xf3) | 8`；两张磁盘 → 墙色定制 → `bool(0x203f…)` / `i32(0x5add…)` / `i32(0x052e…)` /
/// `Vec3(0x06bd…)` / `f32(0xb585…)` → `nani_draw(Plaster = 0x13)`。
pub fn env34_body<'p>(
    b: DrawBuilder<'p>,
    i: &Env34Inputs,
    nani: impl FnOnce(DrawBuilder<'p>, GpuWorldSubsetKind),
) {
    let mut b = b;
    set_cull(&mut b, 8);
    let b = bind_disk(b, 0xe410c6c6901dbb51, 0xda3297075023f6d7);
    let b = bind_disk(b, 0x8d8a48802c6ae438, 0xc74d27dfe21f9ffc);
    let b = b.bind_wall_color_customization::<WallColorIdSsbo>();
    let b = b.uniform(0x203fd61c57064e54, i.editing_enabled);
    let b = b.uniform(0x5add70b0954f689d, i.wall_value);
    let b = b.uniform(0x052e7aa4e36b047b, i.wall_tag);
    let b = b.uniform(0x06bdce4199980bf7, i.cursor);
    let b = b.uniform(0xb585ce21b6c728c4, i.radius);
    nani(b, GpuWorldSubsetKind::Plaster);
}
pub fn env34(b: DrawBuilder<'_>, _p: &EntityDrawParams, w: &mut World) {
    let (wall_value, wall_tag) = wall_highlight_param(res::<WallHighlight>(w));
    let editing_enabled = res::<EditingHighlightView>(w).enabled;
    let cursor = res::<CursorTerrainRaycast>(w);
    let cursor = if cursor.tag & 1 != 0 {
        cursor.pos
    } else {
        Vec3::ZERO
    };
    let radius = res::<EraserSizeMultiplier>(w).0;
    let inputs = Env34Inputs {
        wall_value,
        wall_tag,
        editing_enabled,
        cursor,
        radius,
    };
    env34_body(b, &inputs, |b, k| nani_draw_world(b, k, w));
}

// =====================================================================================================================
// 绑定 + draw / multi_draw_indexed_indirect / draw_instanced_w_gl_draw_flag（不碰 World）
// =====================================================================================================================

/// closure_env$1（0x1409b5c70）：`BoatSsbo`（binding 0x6d9256d4ac7887ad）→ `multi_draw_indexed_indirect(0xc, 1, 0)`。
pub fn env1(b: DrawBuilder<'_>, _p: &EntityDrawParams, _w: &mut World) {
    let b = b.read_ssbo::<BoatTag>(0x6d9256d4ac7887ad);
    b.multi_draw_indexed_indirect::<DrawIndexedIndirectTag>(0xc, 1, 0);
}

/// closure_env$7（0x1409b6150）：磁盘 + 预设 0xf/5/0xe/0x13/9/0xa + `Flower03Ssbo` → `multi_draw_indexed_indirect(0x10, 1, 0)`。
pub fn env7(b: DrawBuilder<'_>, _p: &EntityDrawParams, _w: &mut World) {
    let b = bind_disk(b, 0x25df9e0f0168ac29, 0xfbdca30f95e604d1);
    let b = bind_preset(b, 0xf, 0x035280b96dab13e6);
    let b = bind_preset(b, 5, 0xed502fff6dc4021f);
    let b = bind_preset(b, 0xe, 0x4ee098b38612d2b9);
    let b = bind_preset(b, 0x13, 0x6ad63ccd18c8f041);
    let b = bind_preset(b, 9, 0x61b08b5e0470c5f2);
    let b = bind_preset(b, 0xa, 0x297d1e4cdae8f657);
    let b = b.read_ssbo::<Flower03Tag>(0x6d9256d4ac7887ad);
    b.multi_draw_indexed_indirect::<DrawIndexedIndirectTag>(0x10, 1, 0);
}

/// closure_env$26（0x1409b7d40）：预设 9/0xa + `GardenStoneSsbo` → `multi_draw_indexed_indirect(0xe, 1, 0)`。
pub fn env26(b: DrawBuilder<'_>, _p: &EntityDrawParams, _w: &mut World) {
    let b = bind_preset(b, 9, 0x61b08b5e0470c5f2);
    let b = bind_preset(b, 0xa, 0x297d1e4cdae8f657);
    let b = b.read_ssbo::<GardenStoneTag>(0x6d9256d4ac7887ad);
    b.multi_draw_indexed_indirect::<DrawIndexedIndirectTag>(0xe, 1, 0);
}

/// closure_env$54（0x1409b7110）：磁盘 → `draw_instanced_w_gl_draw_flag(0x1770)`。
pub fn env54(b: DrawBuilder<'_>, _p: &EntityDrawParams, _w: &mut World) {
    let b = bind_disk(b, 0xcf0ec9b45838117a, 0xe1347d0cd85b697d);
    b.draw_instanced_w_gl_draw_flag(0x1770);
}

/// closure_env$51（0x1409b8250）：磁盘 → `draw`。
pub fn env51(b: DrawBuilder<'_>, _p: &EntityDrawParams, _w: &mut World) {
    let b = bind_disk(b, 0x10b5ad13d86a52bb, 0xc6f60b64f3d08191);
    b.draw();
}

/// closure_env$60（0x1409b8430）：`state.byte0 = (b & 0xf3) | 8`；磁盘 → `draw`。
pub fn env60(b: DrawBuilder<'_>, _p: &EntityDrawParams, _w: &mut World) {
    let mut b = b;
    set_cull(&mut b, 8);
    let b = bind_disk(b, 0xc2074e99b3c51eee, 0xce8d8d6a107ec5a6);
    b.draw();
}

/// closure_env$43（0x1409b8620）：`rocky_terrain_shader(b, false)`（`r9d = 0`）；顶点数取自
/// `RockyTerrainMeshVertCount` 资源（缺失 → `Option::unwrap` panic）。
pub fn env43(b: DrawBuilder<'_>, _p: &EntityDrawParams, w: &mut World) {
    let count = res::<RockyTerrainMeshVertCountView>(w).0;
    rocky_terrain_shader(b, false, count);
}

// =====================================================================================================================
// 读 World 的闭包
// =====================================================================================================================

/// closure_env$18（0x1409b5cf0）：`entity.contains::<SemiTransparent>()` / `contains::<FlatLighting>()`（先于一切绑定求值）→
/// `bool(0x6183d59b2f20cb8a)` / `bool(0x5e4434780397435a)` → `draw`。
pub fn env18_bind(
    b: DrawBuilder<'_>,
    semi_transparent: bool,
    flat_lighting: bool,
) -> DrawBuilder<'_> {
    let b = b.uniform(0x6183d59b2f20cb8a, semi_transparent);
    b.uniform(0x5e4434780397435a, flat_lighting)
}
pub fn env18(b: DrawBuilder<'_>, p: &EntityDrawParams, w: &mut World) {
    let e = w.get_entity(p.entity).unwrap();
    let semi = e.contains::<SemiTransparent>();
    let flat = e.contains::<FlatLighting>();
    env18_bind(b, semi, flat).draw();
}

/// closure_env$5（0x1409b68e0）：磁盘 → `MainCamera` 的 `eye_position` → `Vec3(0x4bbd…)` → 预设
/// 0xf/5/0x13/0xe/0x15/0x10/9/0xa → 实体组件 `InstancedDrawFromPlacementData` → `draw_instanced_from_placement_data(id)`。
pub fn env5_bind(b: DrawBuilder<'_>, eye: Vec3) -> DrawBuilder<'_> {
    let b = bind_disk(b, 0x53684b663303712d, 0x1d40047216663b6a);
    let b = b.uniform(0x4bbdb30b873d3990, eye);
    let b = bind_preset(b, 0xf, 0x035280b96dab13e6);
    let b = bind_preset(b, 5, 0xed502fff6dc4021f);
    let b = bind_preset(b, 0x13, 0x6ad63ccd18c8f041);
    let b = bind_preset(b, 0xe, 0x4ee098b38612d2b9);
    let b = bind_preset(b, 0x15, 0x4a9aecb3eff0f013);
    let b = bind_preset(b, 0x10, 0x0d7eab55ca05e34c);
    let b = bind_preset(b, 9, 0x61b08b5e0470c5f2);
    bind_preset(b, 0xa, 0x297d1e4cdae8f657)
}
pub fn env5(b: DrawBuilder<'_>, p: &EntityDrawParams, w: &mut World) {
    let eye = res::<MainCameraView>(w).eye;
    let b = env5_bind(b, eye);
    let id = comp::<InstancedDrawFromPlacementData>(w, p.entity).0;
    b.draw_instanced_from_placement_data(id);
}

/// closure_env$21（0x1409b8680）的世界输入。
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Env21Inputs {
    /// `RoofFlagCloth` 的 `u32 @+0`。
    pub roof_flag_cloth: u32,
    /// `ColorId.0`。
    pub color_id: u32,
    /// `ActiveSession` 字节序列。
    pub session_bytes: Vec<u8>,
}

/// 末张磁盘纹理的文件键：会话字节里存在 `<= 1` 的字节 → `0x4c58d4d61ec450d2`，否则 `-0x1adc8a43cd569b5c`
/// （O 0x1409b8b60 循环：`cmp byte,1; ja continue`）。
pub fn env21_last_texture(bytes: &[u8]) -> u64 {
    if bytes.iter().any(|b| *b <= 1) {
        0x4c58d4d61ec450d2
    } else {
        0xe52375bc32a964a4
    }
}

/// closure_env$21：`bool(0x6183…)=false`、`bool(0x5e44…)=false` → 4 张磁盘 → `i32(0x2cfa…)=RoofFlagCloth` /
/// `i32(0xbf98…)=ColorId` → 磁盘 → 会话字节选择的磁盘 → `draw`。
pub fn env21_bind<'p>(b: DrawBuilder<'p>, i: &Env21Inputs) -> DrawBuilder<'p> {
    let b = b.uniform(0x6183d59b2f20cb8a, false);
    let b = b.uniform(0x5e4434780397435a, false);
    let b = bind_disk(b, 0x61c03c1f48298c31, 0x3d1911796f1e768f);
    let b = bind_disk(b, 0x1c9b640db8cc8190, 0xe1785f9c300d7c8c);
    let b = bind_disk(b, 0x93b4b6c9587d6b45, 0x3607cb2e2d647b2e);
    let b = bind_disk(b, 0x92f0e1606bfe7473, 0xa5453e8f34ef8850);
    let b = b.uniform(0x2cfad118d3ecc02c, i.roof_flag_cloth as i32);
    let b = b.uniform(0xbf98c47fe61d9aa0, i.color_id as i32);
    let b = bind_disk(b, 0xb72fba719985a58c, 0x83b632dfd4a3558c);
    bind_disk(b, env21_last_texture(&i.session_bytes), 0xce84646a28bf8813)
}
pub fn env21(b: DrawBuilder<'_>, p: &EntityDrawParams, w: &mut World) {
    let roof_flag_cloth = comp::<RoofFlagClothView>(w, p.entity).0;
    let color_id = comp::<ColorId>(w, p.entity).0;
    let session_bytes = res::<ActiveSessionView>(w).glade_bytes.clone();
    env21_bind(
        b,
        &Env21Inputs {
            roof_flag_cloth,
            color_id,
            session_bytes,
        },
    )
    .draw();
}

/// closure_env$38（0x1409b9080）：`LogoAnimation` 资源（`f32 @+8`、`f32 @+0x10`）→ 4 张磁盘 → `f32(0x3cc1…)` / `f32(0x1986…)` → `draw`。
pub fn env38_bind(b: DrawBuilder<'_>, f8: f32, f10: f32) -> DrawBuilder<'_> {
    let b = bind_disk(b, 0xd47acc582783e1a8, 0xff277bb82867f70e);
    let b = bind_disk(b, 0x394df4b2308658c8, 0xff2778b82867f1f5);
    let b = bind_disk(b, 0xa9ffa026a69075eb, 0xff276ab82867da2b);
    let b = bind_disk(b, 0x78624d43dedde98a, 0xff276cb82867dd91);
    let b = b.uniform(0x3cc185d80517a5cb, f8);
    b.uniform(0x1986361629359548, f10)
}
pub fn env38(b: DrawBuilder<'_>, _p: &EntityDrawParams, w: &mut World) {
    let l = *res::<LogoAnimationView>(w);
    env38_bind(b, l.f8, l.f10).draw();
}

/// closure_env$36（0x1409b7fa0）：`AssetSsboLibrary` → `SparkleSsbo` 状态（缺失 → `Option::unwrap` panic；名字不符 →
/// `assert_eq!` 失败）→ 稀疏缓冲 `to_draw_multi_instanced` → `read_storage_buffer<SparkleSsbo>(0xc3483cd70105de2f)` →
/// `draw_multi_instanced(ranges)`。
pub fn env36_body(b: DrawBuilder<'_>, ranges: Vec<[u32; 2]>) {
    let b = b.read_ssbo::<SparkleTag>(0xc3483cd70105de2f);
    b.draw_multi_instanced(ranges);
}
pub fn env36(b: DrawBuilder<'_>, _p: &EntityDrawParams, w: &mut World) {
    let lib = res::<AssetSsboLibrary>(w);
    let (name, _info) = lib
        .lookup(SsboId(NameHash(SparkleTag::rg_buffer_hash())))
        .expect(UNWRAP_NONE);
    assert_eq!(name, SparkleTag::TYPE_NAME);
    let ranges = res::<SparkleDrawRangesView>(w).0.clone();
    env36_body(b, ranges);
}

// =====================================================================================================================
// 直接函数的 World 适配器（`draw_shaders::*` 的 Rust 签名带数据参数，原版在函数内读 World）
// =====================================================================================================================

/// `tree_trunk` 适配器（0x1409c9830）：资源 `Weather`（风）、`MainCamera`（eye、季节字节）、`TreeCanopyMaxCullDistance`。
pub fn tree_trunk_cb(b: DrawBuilder<'_>, _p: &EntityDrawParams, w: &mut World) {
    let wind = res::<WeatherView>(w).wind;
    let cam = *res::<MainCameraView>(w);
    let blend = res::<TreeCanopyMaxCullDistanceView>(w).0;
    let inputs = TreeTrunkInputs {
        wind,
        eye: cam.eye,
        season_is_3: cam.season == 3,
        blend,
    };
    let w: &World = w;
    tree_trunk(b, &inputs, |b, k| nani_draw_world(b, k, w));
}

/// `tree_canopy` 适配器（0x1409ca350）：`season_param` = 季节子结构的 `f32 @+0x104`（`season >= 6` → 越界 panic，同原版的越界读）。
pub fn tree_canopy_cb(b: DrawBuilder<'_>, _p: &EntityDrawParams, w: &mut World) {
    let cam = *res::<MainCameraView>(w);
    let wind = res::<WeatherView>(w).wind;
    let blend = res::<TreeCanopyMaxCullDistanceView>(w).0;
    let inputs = TreeCanopyInputs {
        wind,
        season_param: cam.season_params[cam.season as usize],
        season_is_3: cam.season == 3,
        blend,
    };
    let w: &World = w;
    tree_canopy(b, &inputs, |b, k| nani_draw_world(b, k, w));
}

/// `draw_instanced_roof_shader_nani` 适配器（0x1409d9070）：`RoofHighlight` 资源。
pub fn roof_nani_cb(b: DrawBuilder<'_>, _p: &EntityDrawParams, w: &mut World) {
    let h = *res::<RoofHighlight>(w);
    let w: &World = w;
    draw_instanced_roof_shader_nani(
        b,
        RoofNaniParam {
            value: h.value,
            tag: h.tag,
        },
        |b, k| nani_draw_world(b, k, w),
    );
}

/// `draw_instanced_roof_spire_shader` 适配器（0x1409d96a0）：`AssetSsboLibrary` 资源。
pub fn roof_spire_cb(b: DrawBuilder<'_>, _p: &EntityDrawParams, w: &mut World) {
    draw_instanced_roof_spire_shader(b, &LibView(res::<AssetSsboLibrary>(w)));
}

/// `draw_instanced_roof_support_beam_shader` 适配器（0x1409d98a0）。
pub fn roof_support_beam_cb(b: DrawBuilder<'_>, _p: &EntityDrawParams, w: &mut World) {
    draw_instanced_roof_support_beam_shader(b, &LibView(res::<AssetSsboLibrary>(w)));
}

/// `water_planes_draw` 适配器（0x1409d9bc0）：进程布尔、`ActiveSession`（变体字节、字节序列，两次取资源）。
/// `bind_ray_tracing<DrawBuilder>` 用 A4 的真实实现（`RtTriangles/RtTriangleMeta/RtBvh/WallColorId`）。
pub fn water_planes_cb(b: DrawBuilder<'_>, _p: &EntityDrawParams, w: &mut World) {
    let skip = WATER_PLANES_DISABLED.load(Ordering::Relaxed);
    let s = res::<ActiveSessionView>(w);
    let inputs = WaterPlanesInputs {
        skip,
        variant: s.variant,
        all_set: all_nonzero(&s.glade_bytes),
    };
    water_planes_draw(b, inputs, |b| {
        b.bind_ray_tracing::<RtTrianglesSsbo, RtTriangleMetaSsbo, RtBvhSsbo, WallColorIdSsbo>()
    });
}

/// `leaf_particle_draw` (0x140a8a090) reads the actual GladeSettings u64 at +0.
pub fn leaf_particle_cb(b: DrawBuilder<'_>, _p: &EntityDrawParams, w: &mut World) {
    let count = res::<GladeSettings>(w).leaf_particle_count;
    leaf_particle_draw(b, count);
}

/// `draw_instanced_firefly_shader` 适配器（0x1415549b0）：`CritterControl.intensity`。
pub fn firefly_cb(b: DrawBuilder<'_>, _p: &EntityDrawParams, w: &mut World) {
    let i = res::<CritterControl>(w).intensity;
    draw_instanced_firefly_shader(b, i);
}

/// `draw_instanced_firefly_glow_shader` 适配器（0x141554cf0）。
pub fn firefly_glow_cb(b: DrawBuilder<'_>, _p: &EntityDrawParams, w: &mut World) {
    let i = res::<CritterControl>(w).intensity;
    draw_instanced_firefly_glow_shader(b, i);
}

/// `draw_duck_shader` 适配器（0x141555080）：实体上的 `PrevTransform`（Transform 布局）、`Duck`、`DuckState`（读取序同机器码）。
pub fn duck_cb(b: DrawBuilder<'_>, p: &EntityDrawParams, w: &mut World) {
    let t = *comp::<PrevTransformView>(w, p.entity);
    let d = *comp::<Duck>(w, p.entity);
    let s = comp::<DuckState>(w, p.entity).0;
    draw_duck_shader(
        b,
        &DuckInputs {
            rotation: t.rotation,
            translation: t.translation,
            scale: t.scale,
            f_f0: d.f_f0,
            f_f4: d.f_f4,
            f_f8: d.f_f8,
            b_fc: d.b_fc,
            state: s,
        },
    );
}

/// `draw_wren_shader` 适配器（0x141555ec0）：纯 `nani_draw(Wren)`。
pub fn wren_cb(b: DrawBuilder<'_>, _p: &EntityDrawParams, w: &mut World) {
    let w: &World = w;
    draw_wren_shader(b, |b, k| nani_draw_world(b, k, w));
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render::pass_builder::draw_builder::DrawTransform;
    use crate::render::pass_builder::draw_stream::DrawKind;
    use crate::systems::render_loop::draw_passes::draw_shaders::common_includes::{
        TERRAIN_HEIGHTMAP_NAME, TERRAIN_LIGHTING_NAME,
    };
    use crate::systems::render_loop::draw_passes::draw_shaders::test_support::{
        Call, Rig, as_f32, as_i32, rg, uniform_at,
    };
    use alloc::vec;
    use xxhash_rust::xxh3::xxh3_64;

    const H9: u64 = 0x61b08b5e0470c5f2;
    const H10: u64 = 0x297d1e4cdae8f657;

    fn params(e: Entity) -> EntityDrawParams {
        EntityDrawParams {
            topology: None,
            entity: e,
            shader_name: "t",
            transform: DrawTransform {
                rotation: Quat::IDENTITY,
                translation: Vec3::ZERO,
                scale: Vec3::ONE,
            },
        }
    }

    /// 以 `names` 作反射绑定名、`uniforms` 作常量块槽，在 rig 上运行 `f`，返回 (调用序列, 提交的包)。
    fn run(
        names: &[u64],
        uniforms: &[u64],
        f: impl FnOnce(DrawBuilder<'_>),
    ) -> (
        Vec<Call>,
        Vec<crate::render::pass_builder::draw_stream::DrawPacket>,
    ) {
        let mut rig = Rig::new(names, uniforms);
        f(rig.builder());
        (rig.calls(), rig.finish())
    }

    /// 运行带 `nani` 的 body：返回 (kind, 调用序列, 进入 nani 时的常量块, 提交的包)。
    fn run_nani<'a>(
        rig: &'a mut Rig,
        f: impl FnOnce(DrawBuilder<'a>, &mut dyn FnMut(DrawBuilder<'a>, GpuWorldSubsetKind)),
    ) -> (Option<GpuWorldSubsetKind>, Vec<u8>) {
        let mut kind = None;
        let mut data = Vec::new();
        let mut sink = |b: DrawBuilder<'a>, k: GpuWorldSubsetKind| {
            kind = Some(k);
            data = b.constants_data().map(|d| d.to_vec()).unwrap_or_default();
            b.draw();
        };
        f(rig.builder_unbound(), &mut sink);
        (kind, data)
    }

    trait BuilderUnbound {
        fn builder_unbound<'a>(&'a mut self) -> DrawBuilder<'a>;
    }
    impl BuilderUnbound for Rig {
        fn builder_unbound<'a>(&'a mut self) -> DrawBuilder<'a> {
            self.builder()
        }
    }

    macro_rules! nani_case {
        ($name:ident, $body:ident, $kind:ident, $kind_no:expr, [$($call:expr),*], $state:expr, $names:expr) => {
            #[test]
            fn $name() {
                let mut rig = Rig::new(&$names, &[]);
                let (kind, _) = run_nani(&mut rig, |b, sink| $body(b, |b, k| sink(b, k)));
                assert_eq!(kind, Some(GpuWorldSubsetKind::$kind));
                assert_eq!(GpuWorldSubsetKind::$kind as u8, $kind_no);
                assert_eq!(rig.calls(), vec![$($call),*]);
                let d = rig.finish();
                assert_eq!(d.len(), 1);
                if let Some(v) = $state {
                    assert_eq!(d[0].state.0 & 0x0c, v, "(b & 0xf3) | v");
                }
            }
        };
    }

    nani_case!(
        env25_is_a_bare_nani_draw,
        env25_body,
        SolidVertexColor,
        3,
        [],
        None::<u32>,
        []
    );
    nani_case!(
        env49_is_a_bare_nani_draw,
        env49_body,
        VelocityOnly,
        0x19,
        [],
        None::<u32>,
        []
    );
    nani_case!(
        env44_binds_preset_f,
        env44_body,
        Plant,
        0xa,
        [Call::Preset(0xf)],
        None::<u32>,
        [0x035280b96dab13e6u64]
    );
    nani_case!(
        env31_binds_two_disks,
        env31_body,
        SolidVertexColorWindow,
        4,
        [
            Call::Disk(0x5b9a9defbd41da38),
            Call::Disk(0xa041c59f6ca9394d)
        ],
        None::<u32>,
        [0x1aca8782e36dbd0eu64, 0xa944a09a642557c2]
    );
    nani_case!(
        env9_binds_9_a_then_disk,
        env9_body,
        SolidAlbedoMap,
        0xf,
        [
            Call::Preset(9),
            Call::Preset(10),
            Call::Disk(0xc669ad39661fa107)
        ],
        None::<u32>,
        [H9, H10, 0xddedb57bb5d9df46u64]
    );
    nani_case!(
        env29_cull_4_and_two_disks,
        env29_body,
        SolidVertexColorWindowGlass,
        5,
        [
            Call::Disk(0x1b3a0e97773fd725),
            Call::Disk(0x944782cd1ef22b9a)
        ],
        Some(4u32),
        [0x860759fcb9b13fc2u64, 0xe69be10ca2ab87bf]
    );
    nani_case!(
        env24_cull_4_smoke,
        env24_body,
        Smoke,
        0x15,
        [],
        Some(4u32),
        []
    );
    nani_case!(
        env15_cull_8_five_disks,
        env15_body,
        CurvedHalfTimber,
        0x18,
        [
            Call::Disk(0x4a2b687d0c1ecf58),
            Call::Disk(0x5b9a9defbd41da38),
            Call::Disk(0xf1dbf278d4f6847b),
            Call::Disk(0x1b611d8a07dfd6ba),
            Call::Disk(0x239bc9f60a193174)
        ],
        Some(8u32),
        [
            0xd0cc743d37cfaefbu64,
            0x41748af61a225694,
            0xfe5687d6882edbe2,
            0x7d2e85e59f64f385,
            0xf5ee046217439f64
        ]
    );

    #[test]
    fn cull_state_replaces_bits_2_and_3_only() {
        assert_eq!(cull_state(0xffff_ffff, 4), 0xffff_fff7);
        assert_eq!(cull_state(0x8000_008c, 8), 0x8000_0088);
        assert_eq!(cull_state(0, 4), 4);
    }

    #[test]
    fn tag_ids_are_xxh3_of_the_slot_names() {
        fn check<T: DrawSsbo>() {
            assert_eq!(
                T::rg_buffer_hash(),
                xxh3_64(T::SLOT_NAME.as_bytes()),
                "{}",
                T::TYPE_NAME
            );
            assert!(T::TYPE_NAME.starts_with("country_core::"));
        }
        check::<BoatTag>();
        check::<Flower03Tag>();
        check::<GardenStoneTag>();
        check::<HalftimberColorIdTag>();
        check::<SparkleTag>();
        assert_eq!(SparkleTag::TYPE_NAME.len(), 0x2d, "Env36 的 bcmp 长度 0x2d");
    }

    #[test]
    fn env1_env7_env26_end_in_multi_draw_indexed_indirect() {
        let mut w = World::new();
        let e = w.spawn_empty().id();
        let p = params(e);
        let mut rig = Rig::new(&[0x6d9256d4ac7887ad], &[]);
        env1(rig.builder(), &p, &mut w);
        assert_eq!(
            rig.calls(),
            vec![rg::<BoatTag>(), rg::<DrawIndexedIndirectTag>()]
        );
        let d = rig.finish();
        assert!(matches!(
            d[0].kind,
            DrawKind::IndexedIndirect {
                arg_50: 1,
                arg_58: 0,
                ..
            }
        ));

        let names = [0x6d9256d4ac7887adu64, H9, H10];
        let mut rig = Rig::new(&names, &[]);
        env7(rig.builder(), &p, &mut w);
        let calls = rig.calls();
        assert_eq!(calls[0], Call::Disk(0x25df9e0f0168ac29));
        assert_eq!(
            calls[1..7],
            [0xf, 5, 0xe, 0x13, 9, 0xa].map(Call::Preset),
            "预设顺序"
        );
        assert_eq!(calls[7], rg::<Flower03Tag>());
        rig.finish();

        let mut rig = Rig::new(&names, &[]);
        env26(rig.builder(), &p, &mut w);
        assert_eq!(
            rig.calls()[..3],
            [Call::Preset(9), Call::Preset(10), rg::<GardenStoneTag>()]
        );
        rig.finish();
    }

    #[test]
    fn env54_env51_env60_draw_kinds_and_cull() {
        let mut w = World::new();
        let p = params(w.spawn_empty().id());
        let (calls, d) = run(&[0xe1347d0cd85b697d], &[], |b| env54(b, &p, &mut w));
        assert_eq!(calls, vec![Call::Disk(0xcf0ec9b45838117a)]);
        assert_eq!(d[0].kind, DrawKind::InstancedGlDrawFlag { flag: 0x1770 });

        let (calls, d) = run(&[], &[], |b| env51(b, &p, &mut w));
        assert_eq!(calls, vec![Call::Disk(0x10b5ad13d86a52bb)]);
        assert_eq!(d.len(), 1);

        let (calls, d) = run(&[], &[], |b| env60(b, &p, &mut w));
        assert_eq!(calls, vec![Call::Disk(0xc2074e99b3c51eee)]);
        assert_eq!(d[0].state.0 & 0x0c, 8, "(b & 0xf3) | 8");
    }

    #[test]
    fn env43_passes_false_and_the_vertex_count_resource() {
        let mut w = World::new();
        w.insert_resource(RockyTerrainMeshVertCountView(0x1234));
        let p = params(w.spawn_empty().id());
        let (_, d) = run(&[], &[], |b| env43(b, &p, &mut w));
        assert_eq!(
            d[0].kind,
            DrawKind::NoIndexBuffer {
                vertex_count: 0x1234
            }
        );
    }

    #[test]
    #[should_panic(expected = "called `Option::unwrap()` on a `None` value")]
    fn env43_without_the_resource_panics() {
        let mut w = World::new();
        let p = params(w.spawn_empty().id());
        run(&[], &[], |b| env43(b, &p, &mut w));
    }

    #[test]
    fn env18_uniforms_follow_contains() {
        for (semi, flat) in [(false, false), (true, false), (false, true), (true, true)] {
            let mut w = World::new();
            let mut ent = w.spawn_empty();
            if semi {
                ent.insert(SemiTransparent);
            }
            if flat {
                ent.insert(FlatLighting);
            }
            let e = ent.id();
            let mut rig = Rig::new(&[], &[0x6183d59b2f20cb8a, 0x5e4434780397435a]);
            let er = w.get_entity(e).unwrap();
            let b = env18_bind(
                rig.builder(),
                er.contains::<SemiTransparent>(),
                er.contains::<FlatLighting>(),
            );
            let data = b.constants_data().unwrap().to_vec();
            assert_eq!(as_i32(&uniform_at(&data, 0, 4)) != 0, semi);
            assert_eq!(as_i32(&uniform_at(&data, 1, 4)) != 0, flat);
            b.draw();
            rig.finish();
            // 回调本体走同一路径
            let mut rig = Rig::new(&[], &[0x6183d59b2f20cb8a, 0x5e4434780397435a]);
            env18(rig.builder(), &params(e), &mut w);
            assert_eq!(rig.finish().len(), 1);
        }
    }

    #[test]
    #[should_panic(expected = "Err")]
    fn env18_missing_entity_panics_with_result_unwrap() {
        let mut w = World::new();
        let e = w.spawn_empty().id();
        w.despawn(e);
        run(&[], &[], |b| env18(b, &params(e), &mut w));
    }

    #[test]
    fn env12_and_env13_share_the_front_half() {
        let names = [
            0xecece750b6dd7edau64,
            0x41748af61a225694,
            0xed502fff6dc4021f,
            H9,
            H10,
            0xaa8f0490dba0d03c,
            0xd56729333fc94f92,
        ];
        let expect = |calls: Vec<Call>| {
            assert_eq!(calls[0], rg::<HalftimberColorIdTag>());
            assert_eq!(calls[1], Call::Disk(0x5b9a9defbd41da38));
            assert_eq!(
                calls[2..5],
                [Call::Preset(5), Call::Preset(9), Call::Preset(10)]
            );
            assert_eq!(calls[5], Call::Disk(0x4b15227408664b67), "墙色定制首张");
            assert_eq!(*calls.last().unwrap(), rg::<RoofColorIdTag>());
        };
        let mut rig = Rig::new(&names, &[0x052e7aa4e36b047b]);
        let (kind, data) = run_nani(&mut rig, |b, sink| env12_body(b, |b, k| sink(b, k)));
        assert_eq!(kind, Some(GpuWorldSubsetKind::SolidVertexColorBricks));
        assert_eq!(
            as_i32(&uniform_at(&data, 0, 4)),
            0,
            "uniform<i32>(0x052e…) = 0"
        );
        expect(rig.calls());
        rig.finish();

        let brick = BrickEraseHighlightInputs {
            enabled: true,
            cursor_position: Some(Vec3::new(1.0, 2.0, 3.0)),
            radius: 4.5,
            suppress: false,
        };
        let mut all = names.to_vec();
        all.extend([0x203fd61c57064e54, 0x06bdce4199980bf7, 0xb585ce21b6c728c4]);
        let uniforms = [
            0x052e7aa4e36b047b,
            0x203fd61c57064e54,
            0x06bdce4199980bf7,
            0xb585ce21b6c728c4,
        ];
        let mut rig = Rig::new(&names, &uniforms);
        let (kind, data) = run_nani(&mut rig, |b, sink| env13_body(b, &brick, |b, k| sink(b, k)));
        assert_eq!(kind, Some(GpuWorldSubsetKind::GothicWindowBricks));
        assert_eq!(as_i32(&uniform_at(&data, 1, 4)), 1, "enabled && !suppress");
        assert_eq!(as_f32(&uniform_at(&data, 3, 4)), 4.5);
        expect(rig.calls());
        rig.finish();
    }

    #[test]
    fn brick_erase_inputs_reads_the_four_resources() {
        let mut w = World::new();
        w.insert_resource(EditingHighlightView {
            enabled: true,
            ..Default::default()
        });
        w.insert_resource(CursorTerrainRaycast {
            tag: 1,
            pos: Vec3::new(1.0, 2.0, 3.0),
        });
        w.insert_resource(EraserSizeMultiplier(2.5));
        w.insert_resource(DebugGuiView { suppress: true, ..Default::default() });
        let i = brick_erase_inputs(&w);
        assert!(i.enabled && i.suppress);
        assert_eq!(i.cursor_position, Some(Vec3::new(1.0, 2.0, 3.0)));
        assert_eq!(i.radius, 2.5);
        w.insert_resource(CursorTerrainRaycast {
            tag: 0,
            pos: Vec3::ONE,
        });
        assert_eq!(
            brick_erase_inputs(&w).cursor_position,
            None,
            "tag bit0 = 0 → None"
        );
    }

    #[test]
    #[should_panic(expected = "called `Option::unwrap()` on a `None` value")]
    fn brick_erase_inputs_missing_resource_panics() {
        let mut w = World::new();
        w.insert_resource(EditingHighlightView {
            enabled: true,
            ..Default::default()
        });
        let _ = brick_erase_inputs(&w);
    }

    #[test]
    fn wall_highlight_param_zeroes_the_value_when_tag_is_zero() {
        let mut h = WallHighlight::default();
        h.ty = 0;
        h.wall_id = Some(crate::resources::walls::WallId(9));
        assert_eq!(wall_highlight_param(&h), (0, 0));
        h.ty = 2;
        assert_eq!(wall_highlight_param(&h), (9, 2));
    }

    #[test]
    fn env34_uniform_order_cull_and_kind() {
        let i = Env34Inputs {
            wall_value: 9,
            wall_tag: 2,
            editing_enabled: true,
            cursor: Vec3::new(1.0, 2.0, 3.0),
            radius: 0.75,
        };
        let uniforms = [
            0x203fd61c57064e54,
            0x5add70b0954f689d,
            0x052e7aa4e36b047b,
            0x06bdce4199980bf7,
            0xb585ce21b6c728c4,
        ];
        let mut rig = Rig::new(&[], &uniforms);
        let (kind, data) = run_nani(&mut rig, |b, sink| env34_body(b, &i, |b, k| sink(b, k)));
        assert_eq!(kind, Some(GpuWorldSubsetKind::Plaster));
        assert_eq!(GpuWorldSubsetKind::Plaster as u8, 0x13);
        assert_eq!(as_i32(&uniform_at(&data, 0, 4)), 1);
        assert_eq!(as_i32(&uniform_at(&data, 1, 4)), 9);
        assert_eq!(as_i32(&uniform_at(&data, 2, 4)), 2);
        assert_eq!(as_f32(&uniform_at(&data, 3, 4)), 1.0);
        assert_eq!(as_f32(&uniform_at(&data, 4, 4)), 0.75);
        assert_eq!(rig.calls()[0], Call::Disk(0xe410c6c6901dbb51));
        assert_eq!(rig.calls()[1], Call::Disk(0x8d8a48802c6ae438));
        assert_eq!(rig.finish()[0].state.0 & 0x0c, 8);
    }

    #[test]
    fn env5_binds_eye_then_eight_presets_then_placement_id() {
        let mut w = World::new();
        w.insert_resource(MainCameraView {
            eye: Vec3::new(1.0, 2.0, 3.0),
            ..Default::default()
        });
        let e = w.spawn(InstancedDrawFromPlacementData(77)).id();
        let mut rig = Rig::new(&[], &[0x4bbdb30b873d3990]);
        let b = env5_bind(rig.builder(), Vec3::new(1.0, 2.0, 3.0));
        let data = b.constants_data().unwrap().to_vec();
        assert_eq!(as_f32(&uniform_at(&data, 0, 4)), 1.0);
        b.draw();
        assert_eq!(
            rig.calls(),
            [
                vec![Call::Disk(0x53684b663303712d)],
                [0xf, 5, 0x13, 0xe, 0x15, 0x10, 9, 0xa]
                    .map(Call::Preset)
                    .to_vec()
            ]
            .concat()
        );
        rig.finish();
        // 回调：占位数据由 PlacementData 缓冲提供（Rig 宿主无该缓冲 → expect 失败文案）
        let r = std::panic::catch_unwind(core::panic::AssertUnwindSafe(|| {
            let mut rig = Rig::new(&[], &[]);
            env5(rig.builder(), &params(e), &mut w);
        }));
        let msg = format!(
            "{:?}",
            r.unwrap_err().downcast_ref::<alloc::string::String>()
        );
        assert!(msg.contains("PlacementDataSSBO"), "{msg}");
    }

    #[test]
    #[should_panic(expected = "called `Option::unwrap()` on a `None` value")]
    fn env5_without_the_camera_panics() {
        let mut w = World::new();
        let e = w.spawn(InstancedDrawFromPlacementData(1)).id();
        run(&[], &[], |b| env5(b, &params(e), &mut w));
    }

    #[test]
    fn env21_last_texture_depends_on_session_bytes() {
        assert_eq!(
            env21_last_texture(&[]),
            0xe52375bc32a964a4,
            "= -0x1adc8a43cd569b5c"
        );
        assert_eq!(env21_last_texture(&[2, 3, 9]), 0xe52375bc32a964a4);
        assert_eq!(env21_last_texture(&[2, 1, 9]), 0x4c58d4d61ec450d2);
        assert_eq!(env21_last_texture(&[0]), 0x4c58d4d61ec450d2);
        assert_eq!((-0x1adc8a43cd569b5ci64) as u64, 0xe52375bc32a964a4);
    }

    #[test]
    fn env21_order_and_uniforms() {
        let i = Env21Inputs {
            roof_flag_cloth: 5,
            color_id: 11,
            session_bytes: vec![7, 7],
        };
        let uniforms = [
            0x6183d59b2f20cb8a,
            0x5e4434780397435a,
            0x2cfad118d3ecc02c,
            0xbf98c47fe61d9aa0,
        ];
        let mut rig = Rig::new(&[], &uniforms);
        let b = env21_bind(rig.builder(), &i);
        let data = b.constants_data().unwrap().to_vec();
        assert_eq!(as_i32(&uniform_at(&data, 0, 4)), 0);
        assert_eq!(as_i32(&uniform_at(&data, 1, 4)), 0);
        assert_eq!(as_i32(&uniform_at(&data, 2, 4)), 5);
        assert_eq!(as_i32(&uniform_at(&data, 3, 4)), 11);
        b.draw();
        let c = rig.calls();
        assert_eq!(c.len(), 6);
        assert_eq!(c[0], Call::Disk(0x61c03c1f48298c31));
        assert_eq!(c[4], Call::Disk(0xb72fba719985a58c));
        assert_eq!(c[5], Call::Disk(0xe52375bc32a964a4));
        rig.finish();
    }

    #[test]
    fn env21_reads_two_components_and_the_session() {
        let mut w = World::new();
        w.insert_resource(ActiveSessionView {
            variant: 0,
            glade_bytes: vec![1],
        });
        let e = w.spawn((RoofFlagClothView(3), ColorId(4))).id();
        let mut rig = Rig::new(&[], &[]);
        env21(rig.builder(), &params(e), &mut w);
        assert_eq!(*rig.calls().last().unwrap(), Call::Disk(0x4c58d4d61ec450d2));
        assert_eq!(rig.finish().len(), 1);
    }

    #[test]
    #[should_panic(expected = "called `Option::unwrap()` on a `None` value")]
    fn env21_missing_component_panics() {
        let mut w = World::new();
        let e = w.spawn(ColorId(4)).id();
        run(&[], &[], |b| env21(b, &params(e), &mut w));
    }

    #[test]
    fn env38_logo_uniforms() {
        let mut rig = Rig::new(&[], &[0x3cc185d80517a5cb, 0x1986361629359548]);
        let b = env38_bind(rig.builder(), 0.25, 3.0);
        let data = b.constants_data().unwrap().to_vec();
        assert_eq!(as_f32(&uniform_at(&data, 0, 4)), 0.25);
        assert_eq!(as_f32(&uniform_at(&data, 1, 4)), 3.0);
        b.draw();
        assert_eq!(rig.calls().len(), 4);
        rig.finish();
        let mut w = World::new();
        w.insert_resource(LogoAnimationView { f8: 1.0, f10: 2.0 });
        let p = params(w.spawn_empty().id());
        let (_, d) = run(&[], &[], |b| env38(b, &p, &mut w));
        assert_eq!(d.len(), 1);
    }

    #[test]
    fn env36_draws_sparse_ranges_with_the_sparkle_buffer() {
        let (calls, d) = run(&[0xc3483cd70105de2f], &[], |b| {
            env36_body(b, vec![[0, 37], [64, 6]])
        });
        assert_eq!(calls, vec![rg::<SparkleTag>()]);
        assert_eq!(
            d[0].kind,
            DrawKind::MultiInstanced {
                ranges: vec![[0, 37], [64, 6]]
            }
        );
    }

    #[test]
    #[should_panic(expected = "called `Option::unwrap()` on a `None` value")]
    fn env36_without_the_library_panics() {
        let mut w = World::new();
        let p = params(w.spawn_empty().id());
        run(&[], &[], |b| env36(b, &p, &mut w));
    }

    #[test]
    #[should_panic(expected = "called `Option::unwrap()` on a `None` value")]
    fn env36_unregistered_sparkle_buffer_panics() {
        let mut w = World::new();
        w.insert_resource(AssetSsboLibrary::new());
        let p = params(w.spawn_empty().id());
        run(&[], &[], |b| env36(b, &p, &mut w));
    }

    #[test]
    fn spire_and_beam_adapters_draw_zero_when_unregistered() {
        let mut w = World::new();
        w.insert_resource(AssetSsboLibrary::new());
        let p = params(w.spawn_empty().id());
        let (_, d) = run(&[], &[], |b| roof_spire_cb(b, &p, &mut w));
        assert_eq!(d[0].kind, DrawKind::InstancedGlDrawFlag { flag: 0 });
        let (c, d) = run(&[H9, H10], &[], |b| roof_support_beam_cb(b, &p, &mut w));
        assert_eq!(d[0].kind, DrawKind::InstancedGlDrawFlag { flag: 0 });
        assert_eq!(c[..2], [Call::Preset(9), Call::Preset(10)]);
    }

    #[test]
    fn firefly_adapters_read_critter_control() {
        let mut w = World::new();
        let p = params(w.spawn_empty().id());
        w.insert_resource(CritterControl { intensity: 0.0 });
        let (_, d) = run(&[], &[], |b| firefly_cb(b, &p, &mut w));
        assert!(d.is_empty(), "intensity == 0 → discard");
        let (_, d) = run(&[], &[], |b| firefly_glow_cb(b, &p, &mut w));
        assert!(d.is_empty());
        w.insert_resource(CritterControl { intensity: 0.5 });
        let (c, d) = run(&[TERRAIN_HEIGHTMAP_NAME, TERRAIN_LIGHTING_NAME], &[], |b| {
            firefly_cb(b, &p, &mut w)
        });
        assert_eq!(c[..2], [Call::Preset(9), Call::Preset(10)]);
        assert_eq!(d[0].kind, DrawKind::InstancedGlDrawFlag { flag: 0x60 });
    }

    #[test]
    fn duck_adapter_reads_three_components() {
        let mut w = World::new();
        let e = w
            .spawn((
                PrevTransformView {
                    rotation: Quat::IDENTITY,
                    translation: Vec3::new(1.0, 2.0, 3.0),
                    scale: Vec3::ONE,
                },
                Duck {
                    f_f0: 1.0,
                    f_f4: 2.0,
                    f_f8: 3.0,
                    b_fc: true,
                },
                DuckState(3),
            ))
            .id();
        let (c, d) = run(&[], &[], |b| duck_cb(b, &params(e), &mut w));
        assert_eq!(c[..2], [Call::Preset(9), Call::Preset(10)]);
        assert_eq!(d.len(), 1);
    }

    #[test]
    #[should_panic(expected = "called `Option::unwrap()` on a `None` value")]
    fn duck_adapter_missing_state_panics() {
        let mut w = World::new();
        let e = w
            .spawn((
                PrevTransformView {
                    rotation: Quat::IDENTITY,
                    translation: Vec3::ZERO,
                    scale: Vec3::ONE,
                },
                Duck::default(),
            ))
            .id();
        run(&[], &[], |b| duck_cb(b, &params(e), &mut w));
    }

    /// 需要 `GpuWorld`/`AssetSsboLibrary` 的适配器：资源齐了才会走到 `nani_draw`；这里以缺 `GpuWorld` 的 panic 证明前面的
    /// 取值都成功（缺任何一个前置资源会先在它自己那里 panic，文案相同，故再用 `MainCameraView` 缺失的变体对照）。
    fn nani_adapter_panics(
        cb: for<'p> fn(DrawBuilder<'p>, &EntityDrawParams, &mut World),
        w: &mut World,
    ) {
        let p = params(w.spawn_empty().id());
        run(&[], &[], |b| cb(b, &p, w));
    }

    #[test]
    #[should_panic(expected = "called `Option::unwrap()` on a `None` value")]
    fn tree_trunk_adapter_reaches_nani_draw() {
        let mut w = World::new();
        w.insert_resource(WeatherView { wind: 1.0 });
        w.insert_resource(MainCameraView::default());
        w.insert_resource(TreeCanopyMaxCullDistanceView(0.5));
        nani_adapter_panics(tree_trunk_cb, &mut w);
    }

    #[test]
    #[should_panic(expected = "index out of bounds")]
    fn tree_canopy_adapter_indexes_the_season_table() {
        let mut w = World::new();
        w.insert_resource(WeatherView::default());
        w.insert_resource(MainCameraView {
            season: 6,
            ..Default::default()
        });
        w.insert_resource(TreeCanopyMaxCullDistanceView(0.5));
        nani_adapter_panics(tree_canopy_cb, &mut w);
    }

    #[test]
    #[should_panic(expected = "called `Option::unwrap()` on a `None` value")]
    fn roof_nani_and_wren_adapters_need_gpu_world() {
        let mut w = World::new();
        w.insert_resource(RoofHighlight { value: 1, tag: 1 });
        w.insert_resource(AssetSsboLibrary::new());
        nani_adapter_panics(roof_nani_cb, &mut w);
    }

    #[test]
    #[should_panic(expected = "called `Option::unwrap()` on a `None` value")]
    fn wren_adapter_needs_gpu_world() {
        let mut w = World::new();
        nani_adapter_panics(wren_cb, &mut w);
    }

    #[test]
    #[should_panic(expected = "called `Option::unwrap()` on a `None` value")]
    fn water_planes_adapter_needs_the_session() {
        let mut w = World::new();
        nani_adapter_panics(water_planes_cb, &mut w);
    }

    #[test]
    #[should_panic(expected = "called `Option::unwrap()` on a `None` value")]
    fn leaf_particle_adapter_needs_glade_settings() {
        let mut w = World::new();
        nani_adapter_panics(leaf_particle_cb, &mut w);
    }

    #[test]
    fn glade_settings_particle_count_reaches_the_draw_callback() {
        let mut world = World::new();
        world.insert_resource(GladeSettings {
            leaf_particle_count: 0x1_0000_0141,
            ..GladeSettings::default()
        });
        let p = params(world.spawn_empty().id());
        let (_, draws) = run(&[], &[], |builder| {
            leaf_particle_cb(builder, &p, &mut world);
        });
        assert_eq!(draws[0].kind, DrawKind::InstancedGlDrawFlag { flag: 321 });
    }

    #[test]
    fn world_helpers_report_the_original_failure_texts() {
        let w = World::new();
        let r = std::panic::catch_unwind(core::panic::AssertUnwindSafe(|| {
            let _ = res::<WeatherView>(&w);
        }));
        assert!(r.is_err());
    }
}
