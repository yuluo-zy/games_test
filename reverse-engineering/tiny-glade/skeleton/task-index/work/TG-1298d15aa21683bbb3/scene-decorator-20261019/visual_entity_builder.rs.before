//! B `utils/visual_entity_builder.rs` — 装饰视觉实体构建器。
//!
//! 本轮（restorer-consume-nani-decorator，2026-10-17）在 `NaniBundle<T>` /
//! `DrawableMeshBundle` 落库（`country_core::components::drawable`）后，
//! fresh 复核并重建两个 `build` 实例：
//!
//! * `build<DecoratorTransientVisualState>` 0x1412ab270（3281 B）
//! * `build<DecoratorPersistentVisualState>` 0x1412a9b30（3281 B）
//!
//! 两实例函数体逐字节同构（仅尾部 impl$4/impl$5 与标签地址不同；`diff` 证据）。
//! 证据：`reports/unlock/nani/job1-0x1412ab270.c` /
//! `job1-0x1412a9b30.c`（ghidra fresh 全量反编译）+ 本轮 r2 独立反汇编
//! `work/restorer-consume-nani-decorator/disasm_1412ab270.txt`（ghidra）与
//! job6/job7（radare2 RNE 指令 oracle；跳表 case 体 Ghidra 流分析缺失，由 r2 补齐）。
//!
//! ## 构建器字段（O 偏移；字段名 U）
//!
//! 构建器总大小 **0x158**（`instantiate_blueprints` 0x1408289e2 `memcpy 0x158`
//! + `clone<WindowVisualEntityBuilder>` 0x14125f250）。build 读取：
//!
//! | 偏移 | 内容 | 用途 |
//! |---|---|---|
//! | +0x00 bit0 | `NormalMap` 条件位 | 链 10 |
//! | +0x08 | `NormalMap` 载荷 u64 | 链 10 |
//! | +0x10 bit0 | `WindowLight` 条件位 | 链 6 |
//! | +0x18 | `WindowLight` 载荷 u64 | 链 6 |
//! | +0x20 ==1 | `AlbedoMap` 条件 | 链 9 |
//! | +0x28 | `AlbedoMap` 载荷 u64 | 链 9 |
//! | +0x30 ==1 | scale 覆盖 tag | transform |
//! | +0x34..0x40 | 覆盖 scale（3×f32） | transform |
//! | +0x40 ==1 | `GothicWindowBrickDeformation` 条件 | 链 2 |
//! | +0x44..0x80 | 其载荷 0x3c B | 链 2 |
//! | +0x80 ==1 | `ClothlineCloth` 条件 | 链 4 |
//! | +0x84 / +0x8c | 其载荷 u64 / u32 | 链 4 |
//! | +0x90 ==1 | `SittableForBirds` 条件 | 链 14 |
//! | +0x94 / +0x9c | 其载荷 u64 / u32 | 链 14 |
//! | +0xa0 / +0xa8 | **mesh 名** `&str`（ptr/len） | NameHash + `get_handle_by_name` |
//! | +0xb0 / +0xb8 | **nani 名** `&str`（ptr/len） | 12 类分派 + `get_raster_handle_by_name` |
//! | +0xc0 | rotation `Quat`（16 B） | transform |
//! | +0xd0 | translation `Vec3`（12 B） | transform（+ 法线偏移） |
//! | +0xdc | scale `Vec3`（12 B） | transform |
//! | +0xe8 | Transform 尾部 8 B padding（被整体拷贝） | — |
//! | +0xf0 bit0 | `ColorId` 条件位 | 链 1 |
//! | +0xf4 | `ColorId` 载荷 u32 | 链 1 |
//! | +0xf8 ==1 | `RoofFlagCloth` 条件 | 链 11 |
//! | +0xfc | 其载荷 u32 | 链 11 |
//! | +0x100..0x118 | `BrickColor` 载荷 0x18 B（tag 字节在 +0x114） | 链 3 |
//! | +0x114 !=2 | `BrickColor` 插入条件（**不是 ==2**；旧报告极性写反） | 链 3 |
//! | +0x118 ==1 | `LanternLightParams` 条件 | 链 13 |
//! | +0x11c..0x12c | 方向 4×f32 | 链 13 |
//! | +0x130..0x140 | `WallFlagCloth` 载荷 0x10 B（tag 字节在 +0x13c） | 链 12 |
//! | +0x13c !=2 | `WallFlagCloth` 插入条件 | 链 12 |
//! | +0x140..0x14c | **法线偏移** Vec3（rotation 旋转后加到 translation） | transform |
//! | +0x14c !=2 | `Smoke` 插入条件；+0x14c / +0x14d 为两 u8 载荷 | 链 8 |
//! | +0x14e !=0 | `SemiTransparent` 条件 | 链 5 |
//! | +0x14f !=0 | `BalconyDoor` 条件 | 链 7 |
//! | +0x150 | `Option<NaniSolidTexture>`（1 B；**None 判别 = 9**） | 两个 albedo 分支 |
//!
//! transform 的构造（fresh 逐语句）：
//! 1. `translation += rotation * normal_offset`（仅当 `|normal_offset| > 0`；
//!    机器码 `UCOMISS; JBE`，NaN 亦跳过）；
//! 2. 若 `+0x30 == 1`，scale 取 `+0x34..0x40`，否则取 `+0xdc..0xe8`；
//! 3. `Transform { rotation, translation, scale }`。
//!
//! ## mesh 名分派（12 类 + 默认；r2 跳表 0x142c2b058 / 字节比较逐一复核）
//!
//! | nani 名（+0xb0，len） | 实例数据 | subset kind | selector |
//! |---|---|---|---|
//! | `nani_uber` (9) | `UberInstanceData` | 23 | Named(NameHash(mesh)) |
//! | `nani_cloth` (10) | `ClothInstanceData` | 20 | Named |
//! | `nani_smoke` (10) | `SmokeInstanceData` | 21 | **AtlasPart(0)**（`DecoratorRank::new(0)`→`impl$1::from`） |
//! | `nani_balcony_door` (17) | `BalconyDoorInstanceData` | 22 | Named |
//! | `nani_lantern_emissive` (21) | `LanternEmissiveInstanceData` | 18 | Named |
//! | `nani_solid_vertex_color` (23) | `SolidVertexColorInstanceData` | 3 | Named |
//! | `nani_gothic_window_bricks` (25) | `GothicWindowBricksInstanceData` | 7 | Named |
//! | `nani_clutter_SolidAlbedoMap` (27) | `SolidAlbedoMapInstanceData` | 15 | Named + `NaniSolidTexture` |
//! | `nani_solid_vertex_color_window` (30) | `SolidVertexColorWindowInstanceData` | 4 | Named |
//! | `nani_solid_vertex_color_bricks` (30 后的第二支) | `SolidVertexColorBricksInstanceData` | 6 | Named |
//! | `nani_clutter_SolidAlbedoMapEmissive` (35) | `SolidAlbedoMapEmissiveInstanceData` | 16 | Named + `NaniSolidTexture` |
//! | `nani_solid_vertex_color_window_glass` (36) | `SolidVertexColorWindowGlassInstanceData` | 5 | Named |
//! | 默认 | `DrawableMeshBundle`（`get_handle_by_name` / `get_raster_handle_by_name(...).expect(nani 名)`） | — | — |
//!
//! 两个 albedo 分支对 `+0x150` 做 `Option::unwrap`（判别 9 = None → `unwrap_failed`）。
//!
//! ## 条件组件链（14 位；O 顺序与载荷偏移）
//!
//! 1 `ColorId`（+0xf0 bit0，载荷 +0xf4）→ 2 `GothicWindowBrickDeformation`
//! （+0x40==1，0x3c B @+0x44）→ 3 `BrickColor`（+0x114!=2，0x18 B @+0x100）→
//! 4 `ClothlineCloth`（+0x80==1，u64@+0x84+u32@+0x8c）→ 5 `SemiTransparent`
//! （+0x14e!=0，ZST）→ 6 `WindowLight`（+0x10 bit0，u64@+0x18 + id）→
//! 7 `BalconyDoor`（+0x14f!=0，translation + `rotation*Vec3::Z`）→ 8 `Smoke`
//! （+0x14c!=2，`u64(id)` + u32 0 + u8(+0x14c) + u8(+0x14d)）→ 9 `AlbedoMap`
//! （+0x20==1，u64@+0x28）→ 10 `NormalMap`（+0x00 bit0，u64@+0x08）→
//! 11 `RoofFlagCloth`（+0xf8==1，u32@+0xfc）→ 12 `WallFlagCloth`（+0x13c!=2，
//! u64@+0x130 + u64@+0x134 + u32@+0x13c）→ 13 `LanternLightParams`（+0x118==1，
//! `rotation*direction` 3×f32 + 原始 4×f32 + id + 0）→ 14 `SittableForBirds`
//! （+0x90==1，id + u64@+0x94 + u32@+0x9c）。
//!
//! 主链之后：`build<Persistent>` → `impl$4::add_decorator_visual_entity`
//! （0x1408119e0，`HashMap<u32, PersistentVisualEntities>`，seal 位 → panic）；
//! `build<Transient>` → `impl$5::add_decorator_visual_entity`（0x140811c30）。
//!
//! ## impl$5 目标更正（fresh 机器码证据，2026-10-17）
//!
//! wave3 报告据 Ghidra 类型串称 impl$5 目标是
//! `HashMap<u32, Vec<DisplayChildAutoClutterInfo>>`。本轮 r2/ghidra 独立复核
//! **否证**该结论：调用点 0x1412ac662 为 `MOV EDX,ESI; MOV R8,RBX; CALL
//! 0x140811c30`，`RBX` = 入参 Entity；impl$5 尾部把该 **8 B Entity** 写入
//! `Vec`（步长 `len*8`）。`DisplayChildAutoClutterInfo` 的
//! `grow_one<DisplayChildAutoClutterInfo>` 在 0x14099ca60 处步长是 **0x60**，
//! 不可能由 `*8` 的同一容器承担。因此 impl$5 是
//! `DecoratorTransientVisualState::add_decorator_visual_entity(id, entity)`，
//! 目标即瞬态实体表 `HashMap<u32, Vec<Entity>>`（与 `clear_out_visual_entities`
//! 的 `Drain<DecoratorId, Vec<Entity>>` 一致）；Ghidra 的类型串是错误类型应用。
//! 旧 wave3 "目标分歧" 结论据此更正——D1 原 `build_transient` 落瞬态表是对的。
//!
//! ## typed-gap（不编造）
//!
//! * `NaniSolidTexture`：1 B enum，`Option` 以 9 为 None ⇒ 变体值域 0..=8；
//!   **变体名无证据**。这里以 `pending_core::NaniSolidTexture(pub u8)` 承载
//!   原始判别字节，不命名变体。
//! * 12 个条件组件类型在 country-core 尚未落库（仅 registry 名字/注册地址）。
//!   本文件 `pending_core` 子模块按 **原 PDB 名 + 机器码载荷字节数** 建 stand-in
//!   （core 落库后应删除并改用 core 类型；届时组件 TypeId 才与原版一致）。
//! * 构建器 `mesh_name` / `nani_name` 字段名 U；偏移/用途 O。
//! * `mark_as_preview` / `mark_as_window_glass` / `custom_shader` 的 build 外消费
//!   面 U（build 本体不读取对应状态；其设置方法保留）。

use bevy_ecs::entity::Entity;
use bevy_ecs::system::Commands;
use country_core::asset_libraries::mesh_library::AssetMeshLibrary;
use country_core::asset_libraries::shader_library::AssetShaderLibrary;
use country_core::components::drawable::{DrawableMeshBundle, NaniBundle, NaniInstanceDesc};
use country_core::name_hash::NameHash;
use country_core::render::nani::gpu_world::{GpuWorldSubsetKind, InstanceDataMarker};
use country_core::render::nani::instance_data::{
    BalconyDoorInstanceData, ClothInstanceData, GothicWindowBricksInstanceData,
    LanternEmissiveInstanceData, SmokeInstanceData, SolidAlbedoMapEmissiveInstanceData,
    SolidAlbedoMapInstanceData, SolidVertexColorBricksInstanceData, SolidVertexColorInstanceData,
    SolidVertexColorWindowGlassInstanceData, SolidVertexColorWindowInstanceData, UberInstanceData,
};
use country_core::render::nani::systems::NaniMeshSelector;
use country_core::resources::color_id::ColorId;
use country_core::resources::walls::decorator::DecoratorRank;
use country_core::utils::components::transform::Transform;
use glam::{Quat, Vec3};

use crate::decorator_visual_state::{
    DecoratorPersistentVisualState, DecoratorTransientVisualState,
};

/// 核心组件未落库时的 stand-in（原 PDB 路径 = `country_core::components::*`）。
///
/// 每项字段都是**机器码载荷的字节数**（O），字段语义 U。core 落库后应删除本
/// 模块并把 `build` 的 insert 改为 core 类型（否则 TypeId 与原版不同）。
pub mod pending_core {
    use bevy_ecs::component::Component;

    macro_rules! raw_component {
        ($(#[$m:meta])* $name:ident, $n:expr) => {
            $(#[$m])*
            #[derive(Component, Clone, Copy, Debug)]
            #[repr(transparent)]
            pub struct $name(pub [u8; $n]);
        };
    }

    raw_component!(
        /// B `components::GothicWindowBrickDeformation`（builder +0x44 载荷，0x3c B，U）。
        GothicWindowBrickDeformation,
        0x3c
    );
    raw_component!(
        /// B `components::BrickColor`（builder +0x100 载荷，0x18 B，U）。
        BrickColor,
        0x18
    );
    raw_component!(
        /// B `components::ClothlineCloth`（u64 + u32，12 B，U）。
        ClothlineCloth,
        12
    );
    raw_component!(
        /// B `components::WindowLight`（u64 + u32(id)，12 B，U）。
        WindowLight,
        12
    );
    raw_component!(
        /// B `components::BalconyDoor`（translation + `rotation*Z`，0x18 B，O）。
        BalconyDoor,
        0x18
    );
    raw_component!(
        /// B `components::Smoke`（u64(id) + u32 0 + u8 + u8，14 B，U）。
        Smoke,
        14
    );
    raw_component!(
        /// B `components::drawable::AlbedoMap`（8 B 句柄，U）。
        AlbedoMap,
        8
    );
    raw_component!(
        /// B `components::drawable::NormalMap`（8 B 句柄，U）。
        NormalMap,
        8
    );
    raw_component!(
        /// B `components::RoofFlagCloth`（u32，U）。
        RoofFlagCloth,
        4
    );
    raw_component!(
        /// B `components::WallFlagCloth`（u64 + u64 + u32，0x14 B，U）。
        WallFlagCloth,
        0x14
    );
    raw_component!(
        /// B `components::LanternLightParams`（旋转方向 12 B + 原始 16 B + u32 + u8，0x24 B，U）。
        LanternLightParams,
        0x24
    );
    raw_component!(
        /// B `components::SittableForBirds`（u32(id) + u64 + u32，0x10 B，U）。
        SittableForBirds,
        0x10
    );

    /// B `components::drawable::NaniSolidTexture`：1 B enum。
    ///
    /// typed-gap：变体名无证据；`Option` 以判别 9 表示 None（build 的
    /// `cmp r12b,9; je unwrap_failed`），即变体值域 0..=8。以裸字节承载，
    /// 不发明变体名。
    #[derive(Component, Clone, Copy, Debug, Default, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct NaniSolidTexture(pub u8);
}

use pending_core::{
    AlbedoMap, BalconyDoor, BrickColor, ClothlineCloth, GothicWindowBrickDeformation,
    LanternLightParams, NaniSolidTexture, NormalMap, RoofFlagCloth, SittableForBirds, Smoke,
    WallFlagCloth, WindowLight,
};

/// 视觉实体状态落点（`impl$4` / `impl$5` 的统一抽象）。
pub trait DecoratorVisualStateSink {
    /// `add_decorator_visual_entity`：把本次生成的实体登记到装饰 id 下。
    fn add_decorator_visual_entity(&mut self, id: u32, entity: Entity);
}

impl DecoratorVisualStateSink for DecoratorPersistentVisualState {
    /// `impl$4::add_decorator_visual_entity` 0x1408119e0（O）：seal 位已置 →
    /// panic；否则 `entry(id).or_default().push(entity)`。
    fn add_decorator_visual_entity(&mut self, id: u32, entity: Entity) {
        DecoratorPersistentVisualState::add_decorator_visual_entity(self, id, entity);
    }
}

impl DecoratorVisualStateSink for DecoratorTransientVisualState {
    /// `impl$5::add_decorator_visual_entity` 0x140811c30（O，机器码复核）：
    /// `HashMap<u32, Vec<Entity>>::entry(id).or_default().push(entity)`。
    fn add_decorator_visual_entity(&mut self, id: u32, entity: Entity) {
        DecoratorTransientVisualState::add_decorator_visual_entity(self, id, entity);
    }
}

/// B `WindowVisualEntityBuilder`（0x158 B）。
///
/// 字段与偏移的对应见模块头表；`Option` 字段取 `Some` 即机器码中的插入分支
/// （原始 tag 比较值记在字段文档里）。字段名 U、条件/载荷 O。
#[derive(Clone, Debug)]
pub struct WindowVisualEntityBuilder {
    /// rotation（+0xc0，O）。
    pub rotation: Quat,
    /// translation（+0xd0，O）。
    pub translation: Vec3,
    /// scale（+0xdc，O）。
    pub scale: Vec3,
    /// 法线偏移（+0x140，O 数据流；语义名 U）：非零时 `rotation * offset`
    /// 加到 translation。
    pub normal_offset: Vec3,
    /// scale 覆盖（+0x30==1 → +0x34..0x40，O）。
    pub scale_override: Option<Vec3>,
    /// mesh 名（+0xa0/+0xa8，O；NameHash 与 `get_handle_by_name` 的输入）。
    pub mesh_name: &'static str,
    /// nani 名（+0xb0/+0xb8，O；12 类分派键 + `get_raster_handle_by_name`）。
    pub nani_name: &'static str,
    /// `ColorId`（+0xf0 bit0 / +0xf4，O）。
    pub color_id: Option<ColorId>,
    /// `GothicWindowBrickDeformation`（+0x40==1，载荷 0x3c B，O）。
    pub gothic_window_brick_deformation: Option<[u8; 0x3c]>,
    /// `BrickColor`（+0x114**!=2**，载荷 0x18 B，O）。
    pub brick_color: Option<[u8; 0x18]>,
    /// `ClothlineCloth`（+0x80==1，u64@+0x84 + u32@+0x8c，O）。
    pub clothline_cloth: Option<(u64, u32)>,
    /// `SemiTransparent`（+0x14e!=0，ZST，O）。
    pub semi_transparent: bool,
    /// `WindowLight`（+0x10 bit0，u64@+0x18，O）。
    pub window_light: Option<u64>,
    /// `BalconyDoor`（+0x14f!=0，O）。
    pub balcony_door: bool,
    /// `Smoke`（+0x14c!=2，载荷 u8(+0x14c)+u8(+0x14d)，O）。
    pub smoke: Option<(u8, u8)>,
    /// `AlbedoMap`（+0x20==1，u64@+0x28，O）。
    pub albedo_map: Option<u64>,
    /// `NormalMap`（+0x00 bit0，u64@+0x08，O）。
    pub normal_map: Option<u64>,
    /// `RoofFlagCloth`（+0xf8==1，u32@+0xfc，O）。
    pub roof_flag_cloth: Option<u32>,
    /// `WallFlagCloth`（+0x13c!=2，u64@+0x130 + u64@+0x134 + u32@+0x13c，O）。
    pub wall_flag_cloth: Option<(u64, u64, u32)>,
    /// `LanternLightParams`（+0x118==1，方向 4×f32 @+0x11c，O）。
    pub lantern_light_params: Option<[f32; 4]>,
    /// `SittableForBirds`（+0x90==1，u64@+0x94 + u32@+0x9c，O）。
    pub sittable_for_birds: Option<(u64, u32)>,
    /// `Option<NaniSolidTexture>`（+0x150；None 判别 = 9，O；变体名 typed-gap）。
    pub solid_texture: Option<u8>,
    /// 装饰 id（build 的 u32 参数在 D1 API 中随构建器携带；原版由调用点传入，O）。
    pub decorator_id: u32,
    /// `mark_as_preview` 状态（build 不读取；U：原消费面待取证）。
    pub preview: bool,
    /// `mark_as_window_glass` 状态（build 不读取；U：与 +0x14e 的映射无证据）。
    pub window_glass: bool,
    /// `custom_shader` 句柄（build 不读取；U：消费面待取证）。
    pub shader: Option<u32>,
}

impl Default for WindowVisualEntityBuilder {
    fn default() -> Self {
        Self {
            rotation: Quat::IDENTITY,
            translation: Vec3::ZERO,
            scale: Vec3::ONE,
            normal_offset: Vec3::ZERO,
            scale_override: None,
            mesh_name: "",
            nani_name: "",
            color_id: None,
            gothic_window_brick_deformation: None,
            brick_color: None,
            clothline_cloth: None,
            semi_transparent: false,
            window_light: None,
            balcony_door: false,
            smoke: None,
            albedo_map: None,
            normal_map: None,
            roof_flag_cloth: None,
            wall_flag_cloth: None,
            lantern_light_params: None,
            sittable_for_birds: None,
            solid_texture: None,
            decorator_id: 0,
            preview: false,
            window_glass: false,
            shader: None,
        }
    }
}

impl WindowVisualEntityBuilder {
    /// `new`（行 54，O 结构）：位置/旋转/id。scale = 1。
    pub fn new(pos: Vec3, rot: Quat, decorator_id: u32) -> Self {
        Self {
            translation: pos,
            rotation: rot,
            decorator_id,
            ..Self::default()
        }
    }

    /// `mark_as_preview`（行 84）。U：build 不消费。
    pub fn mark_as_preview(&mut self) {
        self.preview = true;
    }

    /// `has_birdspot`（行 89）：打开 `SittableForBirds`（+0x90==1）；载荷
    /// `(u64@+0x94, u32@+0x9c)` 由宿主另行填充（U，此处取 0）。
    pub fn has_birdspot(&mut self) {
        self.sittable_for_birds = Some((0, 0));
    }

    /// `color_id`（行 99）。
    pub fn color_id(&mut self, color: ColorId) {
        self.color_id = Some(color);
    }

    /// `lantern_light`（行 104）：打开 `LanternLightParams`（+0x118==1）；
    /// 方向 4×f32 载荷 U（此处 0）。
    pub fn lantern_light(&mut self) {
        self.lantern_light_params = Some([0.0; 4]);
    }

    /// `custom_shader`（行 126）。U：build 不消费。
    pub fn custom_shader(&mut self, shader: u32) {
        self.shader = Some(shader);
    }

    /// `albedo_map`（行 131）：打开 `AlbedoMap`（+0x20==1，载荷 +0x28）。
    pub fn albedo_map(&mut self, albedo: u64) {
        self.albedo_map = Some(albedo);
    }

    /// `clothline`（行 147）：打开 `ClothlineCloth`（+0x80==1）；载荷 U（0,0）。
    pub fn clothline(&mut self) {
        self.clothline_cloth = Some((0, 0));
    }

    /// `flag_roof_cloth`：打开 `RoofFlagCloth`（+0xf8==1）；载荷 U（0）。
    pub fn flag_roof_cloth(&mut self) {
        self.roof_flag_cloth = Some(0);
    }

    /// `flag_wall_cloth`：打开 `WallFlagCloth`（+0x13c!=2）；载荷 U（0,0,0）。
    pub fn flag_wall_cloth(&mut self) {
        self.wall_flag_cloth = Some((0, 0, 0));
    }

    /// `smoke`：打开 `Smoke`（+0x14c!=2）；载荷 u8(+0x14c)+u8(+0x14d) U（0,0）。
    pub fn smoke(&mut self) {
        self.smoke = Some((0, 0));
    }

    /// `mark_as_window_glass`（行 415 前）。U：build 本体无对应读取。
    pub fn mark_as_window_glass(&mut self) {
        self.window_glass = true;
    }

    /// 计算本实体 transform（fresh 逐语句）。
    fn compute_transform(&self) -> Transform {
        let mut translation = self.translation;
        // `|normal_offset| > 0` 才旋转叠加；NaN 走 JBE（跳过）。
        if self.normal_offset.length() > 0.0 {
            translation += self.rotation * self.normal_offset;
        }
        let scale = self.scale_override.unwrap_or(self.scale);
        Transform {
            rotation: self.rotation,
            translation,
            scale,
        }
    }

    /// 生成 `NaniBundle<T>`（12 类分支共用；每 case 原始代码都是
    /// `NameHash::compute(mesh 名)` → `Commands::spawn<NaniBundle<T>>`
    /// （helper 0x14126dfe0 等内部 `spawn_empty`），返回新实体）。
    // `InstanceDataMarker<T>` 的 `Default` 由 derive 要求 `T: Default`（12 类
    // 实例数据均 derive Default）；`NaniBundle<T>` 另要求 `Send + Sync`。
    fn spawn_nani<T: Send + Sync + Default + 'static>(
        &self,
        commands: &mut Commands,
        transform: Transform,
        subset_kind: GpuWorldSubsetKind,
        named: bool,
    ) -> Entity {
        let mesh = if named {
            // O：`NameHash::compute(+0xa0/+0xa8)`。
            NaniMeshSelector::Named(NameHash::compute(self.mesh_name))
        } else {
            // O：Smoke 分支 `DecoratorRank::new(0)`（0x140833250）→
            // `impl$1::from`（0x14082bd60，AtlasPart）。
            NaniMeshSelector::AtlasPart(u32::from(DecoratorRank::new(0).x()))
        };
        // O：`Commands::spawn<NaniBundle<T>>` = spawn_empty + insert。
        let entity = commands.spawn_empty().id();
        commands.entity(entity).insert(NaniBundle::<T> {
            desc: NaniInstanceDesc { mesh, subset_kind },
            transform,
            marker: InstanceDataMarker::<T>::default(),
        });
        entity
    }

    /// `build<State>`：mesh 分派 → 条件组件链 → 状态落点；返回新建实体。
    ///
    /// 参数面按原调用点（0x1407e29e7 / 0x140828a30）fresh 复核：
    /// `RCX=&builder, EDX=id, R8=&AssetMeshLibrary, R9=&AssetShaderLibrary,
    /// arg6([RSP+0x178])=&mut Commands, arg7([RSP+0x180])=&mut 状态表`；
    /// arg5 未被 build 读取。**实体由 build 内部的
    /// `Commands::spawn`/`spawn_empty` 创建**（不是入参）。
    pub fn build<S: DecoratorVisualStateSink>(
        &self,
        commands: &mut Commands,
        id: u32,
        mesh_lib: &AssetMeshLibrary,
        shader_lib: &AssetShaderLibrary,
        state: &mut S,
    ) -> Entity {
        let transform = self.compute_transform();

        // ---- mesh 名 12 类分派（O；跳表 0x142c2b058）----
        // 原版每个 case 都内部 `spawn`/`spawn_empty` 建实体，返回该 Entity。
        let entity = match self.nani_name {
            "nani_uber" => self.spawn_nani::<UberInstanceData>(
                commands,
                transform,
                GpuWorldSubsetKind::Uber,
                true,
            ),
            "nani_cloth" => self.spawn_nani::<ClothInstanceData>(
                commands,
                transform,
                GpuWorldSubsetKind::Cloth,
                true,
            ),
            "nani_smoke" => self.spawn_nani::<SmokeInstanceData>(
                commands,
                transform,
                GpuWorldSubsetKind::Smoke,
                false,
            ),
            "nani_balcony_door" => self.spawn_nani::<BalconyDoorInstanceData>(
                commands,
                transform,
                GpuWorldSubsetKind::BalconyDoor,
                true,
            ),
            "nani_lantern_emissive" => self.spawn_nani::<LanternEmissiveInstanceData>(
                commands,
                transform,
                GpuWorldSubsetKind::LanternEmissive,
                true,
            ),
            "nani_solid_vertex_color" => self.spawn_nani::<SolidVertexColorInstanceData>(
                commands,
                transform,
                GpuWorldSubsetKind::SolidVertexColor,
                true,
            ),
            "nani_gothic_window_bricks" => self.spawn_nani::<GothicWindowBricksInstanceData>(
                commands,
                transform,
                GpuWorldSubsetKind::GothicWindowBricks,
                true,
            ),
            "nani_clutter_SolidAlbedoMap" => {
                // O：`+0x150` Option::unwrap（None=9 → unwrap_failed，行 243）。
                let tex = NaniSolidTexture(self.solid_texture.unwrap());
                self.spawn_nani_tuple::<SolidAlbedoMapInstanceData>(
                    commands,
                    transform,
                    GpuWorldSubsetKind::SolidAlbedoMap,
                    tex,
                )
            }
            "nani_solid_vertex_color_window" => self
                .spawn_nani::<SolidVertexColorWindowInstanceData>(
                    commands,
                    transform,
                    GpuWorldSubsetKind::SolidVertexColorWindow,
                    true,
                ),
            "nani_solid_vertex_color_bricks" => self
                .spawn_nani::<SolidVertexColorBricksInstanceData>(
                    commands,
                    transform,
                    GpuWorldSubsetKind::SolidVertexColorBricks,
                    true,
                ),
            "nani_clutter_SolidAlbedoMapEmissive" => {
                let tex = NaniSolidTexture(self.solid_texture.unwrap());
                self.spawn_nani_tuple::<SolidAlbedoMapEmissiveInstanceData>(
                    commands,
                    transform,
                    GpuWorldSubsetKind::SolidAlbedoMapEmissive,
                    tex,
                )
            }
            "nani_solid_vertex_color_window_glass" => self
                .spawn_nani::<SolidVertexColorWindowGlassInstanceData>(
                    commands,
                    transform,
                    GpuWorldSubsetKind::SolidVertexColorWindowGlass,
                    true,
                ),
            _ => {
                // O 默认分支：AssetMeshLibrary::get_handle_by_name（内部 panic）
                // + AssetShaderLibrary::get_raster_handle_by_name(...).expect(nani 名)
                // （None → expect_failed，0x1412ac982）；两查表先于 spawn。
                let mesh = mesh_lib.get_handle_by_name(self.mesh_name);
                let shader = shader_lib
                    .get_raster_handle_by_name(self.nani_name)
                    .expect(self.nani_name);
                let entity = commands.spawn_empty().id();
                commands.entity(entity).insert(DrawableMeshBundle {
                    mesh,
                    shader,
                    transform,
                });
                entity
            }
        };

        // ---- 条件组件链（14 位，O 顺序）----

        // 1 ColorId
        if let Some(color) = self.color_id {
            commands.entity(entity).insert(color);
        }
        // 2 GothicWindowBrickDeformation
        if let Some(payload) = self.gothic_window_brick_deformation {
            commands
                .entity(entity)
                .insert(GothicWindowBrickDeformation(payload));
        }
        // 3 BrickColor（+0x114 != 2）
        if let Some(payload) = self.brick_color {
            commands.entity(entity).insert(BrickColor(payload));
        }
        // 4 ClothlineCloth
        if let Some((a, b)) = self.clothline_cloth {
            let mut payload = [0u8; 12];
            payload[0..8].copy_from_slice(&a.to_le_bytes());
            payload[8..12].copy_from_slice(&b.to_le_bytes());
            commands.entity(entity).insert(ClothlineCloth(payload));
        }
        // 5 SemiTransparent（ZST）
        if self.semi_transparent {
            commands
                .entity(entity)
                .insert(country_core::components::SemiTransparent);
        }
        // 6 WindowLight（u64 @+0x18 + id）
        if let Some(a) = self.window_light {
            let mut payload = [0u8; 12];
            payload[0..8].copy_from_slice(&a.to_le_bytes());
            payload[8..12].copy_from_slice(&id.to_le_bytes());
            commands.entity(entity).insert(WindowLight(payload));
        }
        // 7 BalconyDoor（translation + rotation*Vec3::Z）
        if self.balcony_door {
            let mut payload = [0u8; 0x18];
            payload[0..4].copy_from_slice(&transform.translation.x.to_le_bytes());
            payload[4..8].copy_from_slice(&transform.translation.y.to_le_bytes());
            payload[8..12].copy_from_slice(&transform.translation.z.to_le_bytes());
            let forward = transform.rotation * Vec3::Z;
            payload[12..16].copy_from_slice(&forward.x.to_le_bytes());
            payload[16..20].copy_from_slice(&forward.y.to_le_bytes());
            payload[20..24].copy_from_slice(&forward.z.to_le_bytes());
            commands.entity(entity).insert(BalconyDoor(payload));
        }
        // 8 Smoke（u64(id) + u32 0 + u8 + u8）
        if let Some((tag, sub)) = self.smoke {
            let mut payload = [0u8; 14];
            payload[0..8].copy_from_slice(&u64::from(id).to_le_bytes());
            payload[8..12].copy_from_slice(&0u32.to_le_bytes());
            payload[12] = tag;
            payload[13] = sub;
            commands.entity(entity).insert(Smoke(payload));
        }
        // 9 AlbedoMap
        if let Some(handle) = self.albedo_map {
            commands
                .entity(entity)
                .insert(AlbedoMap(handle.to_le_bytes()));
        }
        // 10 NormalMap
        if let Some(handle) = self.normal_map {
            commands
                .entity(entity)
                .insert(NormalMap(handle.to_le_bytes()));
        }
        // 11 RoofFlagCloth
        if let Some(value) = self.roof_flag_cloth {
            commands
                .entity(entity)
                .insert(RoofFlagCloth(value.to_le_bytes()));
        }
        // 12 WallFlagCloth（+0x13c != 2）
        if let Some((a, b, tag)) = self.wall_flag_cloth {
            let mut payload = [0u8; 0x14];
            payload[0..8].copy_from_slice(&a.to_le_bytes());
            payload[8..16].copy_from_slice(&b.to_le_bytes());
            payload[16..20].copy_from_slice(&tag.to_le_bytes());
            commands.entity(entity).insert(WallFlagCloth(payload));
        }
        // 13 LanternLightParams（旋转方向 3×f32 + 原始 4×f32 + id + u8 0）
        if let Some(dir) = self.lantern_light_params {
            let mut payload = [0u8; 0x24];
            let rotated = transform.rotation * Vec3::new(dir[0], dir[1], dir[2]);
            payload[0..4].copy_from_slice(&rotated.x.to_le_bytes());
            payload[4..8].copy_from_slice(&rotated.y.to_le_bytes());
            payload[8..12].copy_from_slice(&rotated.z.to_le_bytes());
            for (i, v) in dir.iter().enumerate() {
                payload[12 + i * 4..16 + i * 4].copy_from_slice(&v.to_le_bytes());
            }
            payload[28..32].copy_from_slice(&id.to_le_bytes());
            payload[32] = 0;
            commands.entity(entity).insert(LanternLightParams(payload));
        }
        // 14 SittableForBirds（id + u64 + u32）
        if let Some((a, b)) = self.sittable_for_birds {
            let mut payload = [0u8; 0x10];
            payload[0..4].copy_from_slice(&id.to_le_bytes());
            payload[4..12].copy_from_slice(&a.to_le_bytes());
            payload[12..16].copy_from_slice(&b.to_le_bytes());
            commands.entity(entity).insert(SittableForBirds(payload));
        }

        // ---- 状态落点（impl$4 / impl$5）----
        state.add_decorator_visual_entity(id, entity);
        entity
    }

    /// 两个 albedo 分支的 tuple `Commands::spawn<(NaniBundle<T>, NaniSolidTexture)>`
    /// （helper 0x140d146a0 内部 `spawn_empty`），返回新实体。
    fn spawn_nani_tuple<T: Send + Sync + Default + 'static>(
        &self,
        commands: &mut Commands,
        transform: Transform,
        subset_kind: GpuWorldSubsetKind,
        texture: NaniSolidTexture,
    ) -> Entity {
        let mesh = NaniMeshSelector::Named(NameHash::compute(self.mesh_name));
        let entity = commands.spawn_empty().id();
        commands.entity(entity).insert((
            NaniBundle::<T> {
                desc: NaniInstanceDesc { mesh, subset_kind },
                transform,
                marker: InstanceDataMarker::<T>::default(),
            },
            texture,
        ));
        entity
    }

    /// `build<DecoratorTransientVisualState>`（0x1412ab270）。
    ///
    /// 原调用点在系统中传 `(builder, id, &mut Commands, &mut 状态表, 两库)`；
    /// 实体由 `build` 内部分派分支创建。此处仅按 D1 API 固定 id 来源。
    pub fn build_transient(
        &self,
        commands: &mut Commands,
        state: &mut DecoratorTransientVisualState,
        mesh_lib: &AssetMeshLibrary,
        shader_lib: &AssetShaderLibrary,
    ) -> Entity {
        self.build(commands, self.decorator_id, mesh_lib, shader_lib, state)
    }

    /// `build<DecoratorPersistentVisualState>`（0x1412a9b30）。
    pub fn build_persistent(
        &self,
        commands: &mut Commands,
        state: &mut DecoratorPersistentVisualState,
        mesh_lib: &AssetMeshLibrary,
        shader_lib: &AssetShaderLibrary,
    ) -> Entity {
        self.build(commands, self.decorator_id, mesh_lib, shader_lib, state)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// O：无 normal_offset / 无 scale 覆盖时 transform 原样。
    #[test]
    fn transform_passthrough() {
        let mut b =
            WindowVisualEntityBuilder::new(Vec3::new(1.0, 2.0, 3.0), Quat::from_rotation_z(0.5), 7);
        b.scale = Vec3::new(2.0, 2.0, 2.0);
        let t = b.compute_transform();
        assert_eq!(t.translation, Vec3::new(1.0, 2.0, 3.0));
        assert_eq!(t.rotation, Quat::from_rotation_z(0.5));
        assert_eq!(t.scale, Vec3::new(2.0, 2.0, 2.0));
    }

    /// O（0x1412ab3aa）：+0x30==1 时 scale 取 +0x34..0x40。
    #[test]
    fn scale_override_wins() {
        let mut b = WindowVisualEntityBuilder::new(Vec3::ZERO, Quat::IDENTITY, 0);
        b.scale = Vec3::ONE;
        b.scale_override = Some(Vec3::splat(3.0));
        assert_eq!(b.compute_transform().scale, Vec3::splat(3.0));
    }

    /// O（0x1412ab2db..0x1412ab3a6）：法线偏移按 rotation 旋转后加到 translation。
    #[test]
    fn normal_offset_is_rotated_into_translation() {
        let mut b = VisualBuilderHelper::quarter_turn();
        b.normal_offset = Vec3::X;
        let t = b.compute_transform();
        assert!((t.translation - Vec3::Y).length() < 1e-6);
    }

    #[test]
    fn zero_normal_offset_is_skipped() {
        let mut b = WindowVisualEntityBuilder::new(Vec3::new(1.0, 0.0, 0.0), Quat::IDENTITY, 0);
        b.normal_offset = Vec3::ZERO;
        assert_eq!(b.compute_transform().translation, Vec3::new(1.0, 0.0, 0.0));
    }

    struct VisualBuilderHelper;

    impl VisualBuilderHelper {
        fn quarter_turn() -> WindowVisualEntityBuilder {
            WindowVisualEntityBuilder::new(
                Vec3::ZERO,
                Quat::from_rotation_z(std::f32::consts::FRAC_PI_2),
                0,
            )
        }
    }
}
