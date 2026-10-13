//! ivy_send_to_gpu_segments.rs — 枝段上传 GPU（O 0x140a9b4e0）。
//!
//! 源 `systems/ivy/ivy_send_to_gpu_segments.rs`；SSBO 载体 `IvySegmentSsbo`
//! （O）。遍历 `IvyStorage::iter`，把每株折线段打包上传（instanced_ivy 绘制）。

#![allow(dead_code)]

/// 枝段 SSBO 载荷（O：`IvySegmentSsbo`；每段打包，U 字段）。
pub struct IvySegmentSsbo(pub Vec<[f32; 4]>);

/// `ivy_send_to_gpu_segments`（O 0x140a9b4e0）。
pub fn ivy_send_to_gpu_segments(_world: &mut bevy_ecs::world::World) {}
