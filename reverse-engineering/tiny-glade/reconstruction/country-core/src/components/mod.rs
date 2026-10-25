//! components/mod.rs — WP6 恢复的游戏组件集（systems.rs 消费的集合）。
//!
//! 源 SHA256（B skeleton）`components/mod.rs`；PDB 命名实体 45 个。
//! 本模块只恢复 nani systems 消费的组件；其余为 typed gap（绝不臆造字段）。

#![allow(dead_code, unused_imports)]

pub mod drawable;
// WP16: residual component families (geometry / transient_mesh / utils).
pub mod geometry;
pub mod transient_mesh;
pub mod utils;

use bevy_ecs::component::Component;

use crate::typed_gap::Gap;

/// 半透明标记组件（O：ECS `Option<&SemiTransparent>`；零尺寸 marker，I）。
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct SemiTransparent;

/// 平直光照标记组件（O：ECS `Option<&FlatLighting>`；零尺寸 marker，I）。
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct FlatLighting;

/// 砖色组件（O：ECS `&BrickColor`；nani_update_solid_vertex_color_bricks / gothic 读）。
/// 布局 U（typed gap；色索引/SSBO 句柄）。
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct BrickColor(pub Gap);

/// 哥特窗砖变形（O：ECS `&GothicWindowBrickDeformation`；nani_update_gothic_window_bricks 读）。
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct GothicWindowBrickDeformation(pub Gap);

/// 晾衣绳布料（O：ECS `&ClothlineCloth`；nani_update_cloth 读；布局 U）。
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct ClothlineCloth(pub Gap);

/// 法线贴图（O：ECS `&NormalMap`；nani_update_cloth 读）。
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct NormalMap(pub Gap);

/// 灯笼光照参数（O：ECS `&LanternLightParams`；nani_update_lantern_emissive 读）。
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct LanternLightParams(pub Gap);

/// 烟（O：ECS `&Smoke`；nani_update_smoke 读；布局 U）。
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct Smoke(pub Gap);

// ---------------------------------------------------------------------------
// WP16 residual markers（O：PDB 命名实体；`register_required_components` 注册的
// 空标记/参数组件）。
// ---------------------------------------------------------------------------

/// 箭头 gizmo（O：`ArrowGizmo`）。
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct ArrowGizmo(pub Gap);

/// 自动杂乱（O：`AutoClutter`；autoclutter 系统消费）。
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct AutoClutter(pub Gap);

/// 可转 de-clutter 的自动杂乱（O：`AutoClutterConvertableToDecoClutter`）。
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct AutoClutterConvertableToDecoClutter(pub Gap);

/// 背景地形（O：`BackgroundTerrain`）。
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct BackgroundTerrain(pub Gap);

/// 阳台门（O：`BalconyDoor`；balcony instance data 源）。
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct BalconyDoor(pub Gap);

/// 已转换预置树（O：`ConvertedPreplacedTree`）。
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct ConvertedPreplacedTree(pub Gap);

/// 虚线长度（O：`DashedLineLength`）。
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct DashedLineLength(pub Gap);

/// 不接收接触阴影（O：`DontReceiveContactShadows`）。
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct DontReceiveContactShadows(pub Gap);

/// 跟随鼠标（O：`FollowMouse`）。
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct FollowMouse(pub Gap);

/// 历史按钮（O：`HistoryButton`）。
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct HistoryButton(pub Gap);

/// 关卡卸载保留（O：`KeepOnLevelUnload`；DrawableMeshBundle 伴随）。
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct KeepOnLevelUnload(pub Gap);

/// 网格跟随地形（O：`MeshFollowTerrain`）。
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct MeshFollowTerrain(pub Gap);

/// 覆盖粗糙度（O：`OverwriteRoughness`）。
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct OverwriteRoughness(pub Gap);

/// 屋顶旗布（O：`RoofFlagCloth`）。
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct RoofFlagCloth(pub Gap);

/// 鸟可歇（O：`SittableForBirds`）。
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct SittableForBirds(pub Gap);

/// 端点吸附预览（O：`SnappingToEndPreview`）。
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct SnappingToEndPreview(pub Gap);

/// 段吸附预览（O：`SnappingToSegmentPreview`）。
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct SnappingToSegmentPreview(pub Gap);

/// 水花粒子（O：`SplashParticle`）。
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct SplashParticle(pub Gap);

/// 台阶步（O：`StairsStep`）。
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct StairsStep(pub Gap);

/// 测试立方体（O：`TestCube`）。
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct TestCube(pub Gap);

/// 树氛围（O：`TreeAmbience`）。
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct TreeAmbience(pub Gap);

/// 树动画（O：`TreeAnimation`）。
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct TreeAnimation(pub Gap);

/// 树组件（O：`TreeComponent`；tree 系统消费）。
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct TreeComponent(pub Gap);

/// 待删树（O：`TreeMarkedForDeletion`）。
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct TreeMarkedForDeletion(pub Gap);

/// 树移动弯折（O：`TreeMoveBend`）。
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct TreeMoveBend(pub Gap);

/// 树存储索引（O：`TreeStorageIndex`；`next`/hash/serde 证据，u64 索引）。
#[derive(Component, Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct TreeStorageIndex(pub u64);

/// 2D UI 跟随鼠标（O：`Ui2DFollowMouse`）。
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct Ui2DFollowMouse(pub Gap);

/// 2D UI 范围（O：`Ui2dExtent`）。
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct Ui2dExtent(pub Gap);

/// 2D UI 图像（O：`Ui2dImage`；`new` 内联于 UI/工具栏构造）。
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct Ui2dImage(pub Gap);

/// 2D UI 世界范围（O：`Ui2dWsExtent`）。
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct Ui2dWsExtent(pub Gap);

/// UI 绘制顺序（O：`UiDrawOrder`）。
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct UiDrawOrder(pub Gap);

/// UI 非激活（O：`UiInactive`）。
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct UiInactive(pub Gap);

/// UI 交互状态（O：`UiInteractionState`）。
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct UiInteractionState(pub Gap);

/// UI 鼠标可交互（O：`UiMouseInteractable`）。
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct UiMouseInteractable(pub Gap);

/// UI 导航（O：`UiNav`）。
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct UiNav(pub Gap);

/// 墙旗布（O：`WallFlagCloth`）。
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct WallFlagCloth(pub Gap);

/// 木门（O：`WoodenGate`；construct_wooden_gate_doors 消费）。
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct WoodenGate(pub Gap);
