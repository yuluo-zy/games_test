//! clutter_meshes.rs — 杂饰网格类型（WP21）。
//!
//! O：`ClutterMeshTy` 56 变体：`From<&str>`（0x1408b83d0）按资产路径匹配（
//! `clutter/plants/…`、`clutter/watering/…`、`clutter/windows_…`、`clutter/firewood/…`、
//! `clutter/crates_…`、`clutter/barrel_w_…`、`clutter/stall_veggies/…`、
//! `clutter/cart_w_pumpkins/…`、`clutter/basket_w_food/…` 等）；`clutter_mesh_ty_data`
//!（0x1408b8e90）变体→(名, kind) 数据表。`ClutterMesh`/`ClutterMeshes` 集合。

use bevy_ecs::component::Component;
use bevy_ecs::resource::Resource;

use crate::typed_gap::Gap;

/// `ClutterMeshTy`（O：56 变体枚举；From<&str> 资产路径匹配）。
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ClutterMeshTy {
    Plants,
    Watering,
    Windows,
    Firewood,
    Crates,
    Barrel,
    StallVeggies,
    CartPumpkins,
    BasketFood,
    Fireplace,
    Railings,
    Other(u8),
}

impl ClutterMeshTy {
    /// `clutter_mesh_ty_data`（O 0x1408b8e90）：变体 → (名称, kind id)。
    pub fn data(self) -> (&'static str, u8) {
        match self {
            ClutterMeshTy::Plants => ("clutter/plants", 0),
            ClutterMeshTy::Watering => ("clutter/watering", 1),
            ClutterMeshTy::Windows => ("clutter/windows", 2),
            ClutterMeshTy::Firewood => ("clutter/firewood", 3),
            ClutterMeshTy::Crates => ("clutter/crates", 4),
            ClutterMeshTy::Barrel => ("clutter/barrel", 5),
            ClutterMeshTy::StallVeggies => ("clutter/stall_veggies", 6),
            ClutterMeshTy::CartPumpkins => ("clutter/cart_w_pumpkins", 7),
            ClutterMeshTy::BasketFood => ("clutter/basket_w_food", 8),
            ClutterMeshTy::Fireplace => ("clutter/fireplace", 9),
            ClutterMeshTy::Railings => ("clutter/railings", 10),
            ClutterMeshTy::Other(_) => ("clutter/other", 0xff),
        }
    }
}

impl From<&str> for ClutterMeshTy {
    /// `From<&str>`（O 0x1408b83d0）：前缀匹配；未识别 panic。
    fn from(name: &str) -> Self {
        if name.starts_with("clutter/plants") {
            ClutterMeshTy::Plants
        } else if name.starts_with("clutter/watering") {
            ClutterMeshTy::Watering
        } else if name.starts_with("clutter/windows") {
            ClutterMeshTy::Windows
        } else if name.starts_with("clutter/firewood") {
            ClutterMeshTy::Firewood
        } else if name.starts_with("clutter/crates") {
            ClutterMeshTy::Crates
        } else if name.starts_with("clutter/barrel") {
            ClutterMeshTy::Barrel
        } else if name.starts_with("clutter/stall_veggies") {
            ClutterMeshTy::StallVeggies
        } else if name.starts_with("clutter/cart_w_pumpkins") {
            ClutterMeshTy::CartPumpkins
        } else if name.starts_with("clutter/basket_w_food") {
            ClutterMeshTy::BasketFood
        } else {
            panic!("clutter mesh type not found: {name}");
        }
    }
}

/// `ClutterMesh`（O 名；单个杂饰网格，U）。
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ClutterMesh(pub Gap);

/// `ClutterMeshes`（O 名；杂饰网格集合，U）。
#[derive(Resource, Clone, Debug, Default)]
pub struct ClutterMeshes(pub Gap);

/// `ClutterMarkedForDeletion`（O：UI 删除标记组件）。
#[derive(Component, Clone, Copy, Debug, Default, PartialEq)]
pub struct ClutterMarkedForDeletion;

/// `DecoratorAutoClutterChildren`（O：自动杂饰子组件）。
#[derive(Component, Clone, Debug, Default)]
pub struct DecoratorAutoClutterChildren(pub Gap);

/// `DisplayChildAutoClutterInfo`（O：UI 子杂饰信息）。
#[derive(Component, Clone, Debug, Default)]
pub struct DisplayChildAutoClutterInfo(pub Gap);
