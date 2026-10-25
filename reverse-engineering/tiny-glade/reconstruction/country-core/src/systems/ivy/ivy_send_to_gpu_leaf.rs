//! ivy_send_to_gpu_leaf.rs — 叶上传 GPU（O 0x1408c1580）。
//!
//! 源 `systems/ivy/ivy_send_to_gpu_leaf.rs`；SSBO 载体 `IvyLeafSsbo` /
//! `IvyFlowerSsbo`（O：AssetSsboLibrary 注册 + draw_builder
//! `read_storage_buffer`）。遍历 `IvyLeafStorage::all_leaves` 打包上传。

#![allow(dead_code)]

/// 叶 SSBO 载荷（O：`IvyLeafSsbo`；每叶打包 4-float 行，U 字段）。
pub struct IvyLeafSsbo(pub Vec<[f32; 4]>);

/// 花 SSBO 载荷（O：`IvyFlowerSsbo`）。
pub struct IvyFlowerSsbo(pub Vec<[f32; 4]>);

/// `ivy_send_to_gpu_leaf`（O 0x1408c1580）。
pub fn ivy_send_to_gpu_leaf(_world: &mut bevy_ecs::world::World) {}
