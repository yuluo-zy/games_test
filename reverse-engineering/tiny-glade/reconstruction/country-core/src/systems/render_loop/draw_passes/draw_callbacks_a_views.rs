//! `draw_callbacks_a` 用到的**视图 / stub 类型**（只含被读字段；偏移为机器码观测）。见 `draw_callbacks_a.rs` 模块文档。

use alloc::vec::Vec;

use bevy_ecs::component::Component;
use bevy_ecs::resource::Resource;
use glam::{Quat, Vec3};

// =====================================================================================================================
// 视图 / stub 类型（只含被读字段；偏移为机器码观测）
// =====================================================================================================================

/// `components::drawable::AlbedoMap`（TypeId 0xed4361819ec3a729/0xc98cd7c5754ff3d7；真实类型 `(pub Gap)`）：`u64 @+0`
/// = `TextureFromDisk` 文件键。
#[derive(Component, Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct AlbedoMapView(pub u64);

/// `AlphaMap`（TypeId 0x028b8ca353f65821/0x1bedc6589e5297bc）：`u64 @+0` 文件键。**I**：类型名由「含该 TypeId 的 bundle
/// `component_ids<DrawableMeshBundle, AlbedoMap, AlphaMap, NormalMap>` 里剔除 AlbedoMap/NormalMap」推得。
#[derive(Component, Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct AlphaMapView(pub u64);

/// `components::NormalMap`（TypeId 0x83098c3ec284caf1/0x6530b452db618923；真实类型 `(pub Gap)`）：`u64 @+0` 文件键。
#[derive(Component, Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct NormalMapView(pub u64);

/// `components::WallFlagCloth`（TypeId 0x0e2c5c63fb76ee91/0xc662ff3d29b2c8d6；真实类型 `(pub Gap)`）：
/// `i32 @+0`、`f32 @+4/+8/+0xc`、`bool @+0x10`。
#[derive(Component, Clone, Copy, Debug, Default, PartialEq)]
pub struct WallFlagClothView {
    pub id: i32,
    pub f4: f32,
    pub f8: f32,
    pub fc: f32,
    pub flag: bool,
}

/// `components::RoofFlagCloth`（TypeId 0x983a5c6eded1c4e5/0x929a707bdcc6a79c）：`u32 @+0`。
#[derive(Component, Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct RoofFlagClothView(pub u32);

/// `components::WoodenGate`（TypeId 0x3b633fbe5a39b5b4/0xd54faa3a72d3ee09；真实类型 `(pub Gap)`）：
/// `qword @+0x18`（取低 32 位写 `i32` uniform）、`i32 @+0x30`。
#[derive(Component, Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct WoodenGateView {
    pub q18: u64,
    pub d30: i32,
}

/// `components::SplashParticle`（TypeId 0x73873257d5f05e11/0xdb3ade5ce1f0e7df）：`f32 @+8`。
#[derive(Component, Clone, Copy, Debug, Default, PartialEq)]
pub struct SplashParticleView {
    pub f8: f32,
}

/// `components::ArrowGizmo`（TypeId 0xf26b1f0dd0e37102/0x4ea1410c87a9f297）：`f32 @+0`、`f32 @+4`。
#[derive(Component, Clone, Copy, Debug, Default, PartialEq)]
pub struct ArrowGizmoView {
    pub f0: f32,
    pub f4: f32,
}

/// `components::DashedLineLength`（TypeId 0x4782fcee8a8c578b/0x8dfbde843e1dee9d）：`f32 @+0`。
#[derive(Component, Clone, Copy, Debug, Default, PartialEq)]
pub struct DashedLineLengthView(pub f32);

/// `resources::critters::sheep::Sheep`（TypeId 0x60556123d75da34f/0x847d370ca9cbbae9；crate 里不存在）：
/// `qword @+0x28`（取低 32 位写 `i32`）、`f32 @+0x58`、`u8 @+0x60`（写 `bool`）。
#[derive(Component, Clone, Copy, Debug, Default, PartialEq)]
pub struct SheepView {
    pub q28: u64,
    pub f58: f32,
    pub b60: u8,
}

/// `components::drawable::PrevTransform`（TypeId 0xa0da022fc84ca3d9/0xfc62e38c26a7c4c9）：`Transform` 布局
/// （旋转 @0、平移 @0x10、缩放 @0x1c）。
#[derive(Component, Clone, Copy, Debug, PartialEq)]
pub struct PrevTransformView {
    pub rotation: Quat,
    pub translation: Vec3,
    pub scale: Vec3,
}

/// `components::StairsStep`（TypeId 0x775c6361f3425a3a/0x3bad471c3be19d00；真实类型 `(pub Gap)`）：首字节 `flags`。
#[derive(Component, Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct StairsStepView {
    pub flags: u8,
}

/// `geometry::instanced_wall::WallBrickInstancedDrawProxy`（组件）：三个字节 `b0 @+0`、`b1 @+1`、`b2 @+2`。真实类型
/// `{ kind: u16, sub: u8 }` 的字节对应关系 U，故用视图。
#[derive(Component, Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct WallBrickInstancedDrawProxyView {
    pub b0: u8,
    pub b1: u8,
    pub b2: u8,
}

/// TypeId 0x66476a13bbcc5e41/0xba435ad0d8465a44：`components` 模块里的 `impl$66`（`register_required_components`），
/// 只做 `contains`。**U**：类型名只有 TypeId；与它同属 bundle `[KeepOnLevelUnload, DrawableMeshBundle, BackgroundTerrain,
/// DontReceiveContactShadows, OverwriteRoughness, …]`，候选 `BackgroundTerrain`（I，未证实）。
#[derive(Component, Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct MarkerU66476a13;

/// TypeId 0xa6c0fc01ec67ef4c/0x7a8927565f75e3ef（`components::impl$25`）：取 `Option<&T>`，只看是否存在。
/// **U**；候选 `DontReceiveContactShadows`（I：同一 bundle、`impl$24 = DashedLineLength` 的字母序后继）。
#[derive(Component, Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct MarkerUa6c0fc01;

/// TypeId 0x192600af2434483b/0x2988839ec6eae656（`components::impl$29`）：`f32 @+0`，缺省 `0.8`。
/// **U**；候选 `OverwriteRoughness`（I：f32 缺省 0.8，字母序紧随 `NormalMap = impl$28`）。
#[derive(Component, Clone, Copy, Debug, Default, PartialEq)]
pub struct FloatU192600af(pub f32);

/// `LogoAnimation`（TypeId 0x98df08dcfa7ea5ba/0x67b117a60e632bc3；真实类型无 `Resource` 派生）：`f32 @+0`、`f32 @+0x10`。
#[derive(Resource, Clone, Copy, Debug, Default, PartialEq)]
pub struct LogoAnimationView {
    pub f0: f32,
    pub f10: f32,
}

/// `LoadingScreenProgress`（TypeId 0x5628183cc22697bc/0x67be6ff8af3d3090；真实类型 `(pub Gap)`）：`f32 @+0`、`f32 @+4`。
#[derive(Resource, Clone, Copy, Debug, Default, PartialEq)]
pub struct LoadingScreenProgressView {
    pub f0: f32,
    pub f4: f32,
}

/// `glade_border::ShowDemoBorderOverlay`（事件，TypeId 0x5eaaca67868e0174/0x67675b60a5763276 同属 `Events<T>`；crate 里不存在）。
#[derive(bevy_ecs::event::Event, Clone, Copy, Debug, Default)]
pub struct ShowDemoBorderOverlay;

/// `ActiveSession`（TypeId 0x74430dfcc6d8719b/0xca5591f7e464509c）：`byte @+0x58`（变体）与字节序列（`ptr @+8`、`len @+0x10`）。
#[derive(Resource, Clone, Debug, Default, PartialEq, Eq)]
pub struct ActiveSessionView {
    pub variant: u8,
    pub glade_bytes: Vec<u8>,
}

/// `Weather`（TypeId 0xd2f519263066413b/0x5192b15f962559e4；真实类型 `(pub Gap)`）：`f32 @+0xc`（强度）。
#[derive(Resource, Clone, Copy, Debug, Default, PartialEq)]
pub struct WeatherView {
    pub intensity: f32,
}

/// `AppMode`（TypeId 0xc2b8e941c91a6428/0x59f3c6d1d3da43a0）：只读首个 `u32`（判别）。真实 `AppMode` 的判别值域在 crate 里
/// 只有 5 个变体，与 `== 0x13` 的比较对不上，故用视图（U）。
#[derive(Resource, Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct AppModeView {
    pub tag: u32,
}

/// `MainCamera`（TypeId 0xbba451ee6a08a3ff/0x8d008d15febcf29d；真实类型 `raw: Gap`）：`eye_position()` 与 `byte @+0x4d0`。
#[derive(Resource, Clone, Copy, Debug, Default, PartialEq)]
pub struct MainCameraView {
    pub eye: Vec3,
    pub season: u8,
}

/// `EditingHighlight`（TypeId 0x6efcda1c77e106b4/0x7c999c10e7fa637f；真实类型 `raw: Gap`）：`byte @+0xc`。
#[derive(Resource, Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct EditingHighlightView {
    pub enabled: bool,
}

/// `DebugGui`（TypeId 0x48b321ef779dc301/0xbd65767194d9ddc8）：`byte @+0xfe`。
#[derive(Resource, Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct DebugGuiView {
    pub suppress: bool,
}

/// `systems::wall::eraser::EraserSizeMultiplier`（TypeId 0x0ae77a86a2c6cefc/0xf5509a56b32e81d0）：首个 `f32`。
#[derive(Resource, Clone, Copy, Debug, Default, PartialEq)]
pub struct EraserSizeMultiplier(pub f32);

/// `resources::roofs::roof_highlight::RoofHighlight`（TypeId 0x9311cf2688353878/0xfd54118f9a9ee3d4）：`u32 @+0`、`tag @+8`。
#[derive(Resource, Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct RoofHighlight {
    pub value: u32,
    pub tag: u8,
}
