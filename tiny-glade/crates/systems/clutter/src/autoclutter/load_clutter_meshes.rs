//! load_clutter_meshes（B 主基线；0x1422c0760，源码 19–；closure 资产加载）。
//!
//! 证据（skeleton PDB 依赖注解）：Commands::insert_resource（0x141cea830）、
//! load_clutter_meshes::closure（资产加载闭包）。语义：把自动杂物网格资产
//! （ClutterMeshes）载入网格库——ClutterMeshTy 56 变体按资产路径匹配
//! （chapters/resources/clutter_meshes.rs.md：`ClutterMeshTy::from(&str)` 0x1408b83d0，
//! anvil/door_bell/door_krans/stall_bread/rack_brooms/flowerbox/birdnest/firewood/
//! cart_w_pumpkins/cart_w_p_candles 等；未识别 panic）。
//!
//! 路径表（I：前缀 = 变体名 snake_case 的资产子路径；56 变体见 core
//! `systems::mode_manager::types::ClutterMeshTy`）。

use bevy_ecs::prelude::*;
use country_core::systems::mode_manager::ClutterMeshTy;

/// 删除标记组件（ClutterMarkedForDeletion，core 章节：UI 删除标记；manage 插入）。
#[derive(Component, Clone, Copy, Debug, Default, PartialEq)]
pub struct ClutterMarkedForDeletion;

/// 自动杂物网格集合（宿主资产库装载后填充；按 ClutterMeshTy 56 变体）。
#[derive(Resource, Clone, Debug, Default)]
pub struct ClutterMeshes {
    pub by_ty: Vec<(ClutterMeshTy, String)>,
}

/// 56 变体资产路径（O：`ClutterMeshTy::from(&str)` 0x1408b83d0 前缀匹配；
/// 举例路径 anvil/door_bell/door_krans/stall_bread/rack_brooms/flowerbox/birdnest/
/// firewood/cart_w_pumpkins/cart_w_p_candles 已证；其余按变体名推断 I）。
pub const CLUTTER_MESH_PATHS: [&str; 56] = [
    "clutter/plant_pot_v1", "clutter/ladder", "clutter/wheel", "clutter/pitchfork",
    "clutter/shovel", "clutter/box_w_stuff", "clutter/watering_can", "clutter/firewood",
    "clutter/bucket", "clutter/birdnest", "clutter/bench", "clutter/bench_w_basket",
    "clutter/bench_short", "clutter/barrel_w_plantpot", "clutter/haystack",
    "clutter/plant_pot_v2", "clutter/birdhouse", "clutter/basket",
    "clutter/basket_w_food", "clutter/barrel_w_food_basket", "clutter/potato_sack",
    "clutter/road_sign", "clutter/sunflower", "clutter/cart", "clutter/cart2",
    "clutter/crates_w_tools", "clutter/crates_w_flowers", "clutter/window_flowerpot",
    "clutter/window_shutter", "clutter/window_flowerbeds", "clutter/reed",
    "clutter/lilypad", "clutter/fallen_leaves", "clutter/fallen_petals",
    "clutter/olden_rock", "clutter/olden_tree_log", "clutter/olden_tree_stump",
    "clutter/pumpkin", "clutter/pumpkin2", "clutter/pumpkin_w_face",
    "clutter/cart_w_pumpkins", "clutter/crates_w_pumpkins",
    "clutter/olden_tree_stump_autumn", "clutter/barrel_w_candles", "clutter/anvil",
    "clutter/fabric_rolls", "clutter/stall_bread", "clutter/stall_veggies",
    "clutter/crate_fish", "clutter/rack_brooms", "clutter/flowerbox",
    "clutter/door_bell", "clutter/door_krans", "clutter/door_krans_autumn",
    "clutter/cart_w_pumpkins_regular", "clutter/crates_w_pumpkins_regular",
];

impl ClutterMeshes {
    /// 全 56 变体注册（O：load closure 语义；宿主资产库按路径装载）。
    pub fn register_all() -> Self {
        let by_ty = (0..56u8)
            .filter_map(|tag| ClutterMeshTy::from_native_tag(tag))
            .zip(CLUTTER_MESH_PATHS.iter().copied())
            .map(|(ty, path)| (ty, path.to_string()))
            .collect();
        Self { by_ty }
    }
}

/// load_clutter_meshes（0x1422c0760）：注册资产并 insert_resource。
pub fn load_clutter_meshes(mut commands: Commands) {
    commands.insert_resource(ClutterMeshes::register_all());
}
