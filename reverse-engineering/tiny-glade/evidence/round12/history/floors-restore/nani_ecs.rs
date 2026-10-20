//! Actual ECS owners for recovered Nani maintenance and StoneFloor update.
//! Other kind-specific consumers are not replaced by empty registrations.
use bevy_ecs::prelude::*;
use super::{
    active_instance::NaniActiveInstance,
    gpu_world::{GpuWorld,InstanceDataMarker},
    instance_data::StoneFloorInstanceData,
    systems::{self,NaniInstanceDesc,NaniActiveInstanceCollection,NewInstanceRow,ChangedInstanceRow,StoneFloorRow},
};
use crate::{
    resources::{render::mesh_atlas_library::MeshAtlasLibrary,roofs::{Roof,roof_animation::RoofAnimation},walls::Floor},
    utils::components::transform::Transform,
};

/// Original1420691c0 seven-parameter wrapper. The safe Rust key exposes its
/// immutable component access and interior cache hint are kept distinct from
/// the existing mutable plain-key GPU API.
pub fn maintain_instances(
    new:Query<(Entity,&NaniInstanceDesc,&Transform),Without<NaniActiveInstance>>,
    changed:Query<(&NaniInstanceDesc,&NaniActiveInstance),Changed<NaniInstanceDesc>>,
    mut removed:RemovedComponents<NaniActiveInstance>,
    atlas:Res<MeshAtlasLibrary>,
    mut gpu:ResMut<GpuWorld>,
    mut local:Local<NaniActiveInstanceCollection>,
    mut commands:Commands,
) {
    let new:Vec<_>=new.iter().map(|(entity,desc,transform)|NewInstanceRow{entity,desc,transform}).collect();
    let mut changed_items:Vec<_>=changed.iter().map(|(desc,active)|(desc,active.key_guard())).collect();
    let mut changed:Vec<_>=changed_items.iter_mut().map(|(desc,key)|ChangedInstanceRow{desc,active:key}).collect();
    let inserts=systems::nani_maintain_instances(removed.read(),&new,&mut changed,&atlas,&mut gpu,&mut local);
    for (entity,key) in inserts {commands.entity(entity).insert(NaniActiveInstance::from_key(key));}
}

/// Original142069810: no Changed filter; optional roof and floor share kind9.
/// The original unwrap when neither is present is retained by the real builder.
pub fn update_stone_floor(
    rows:Query<(&NaniActiveInstance,&Transform,Option<&Roof>,Option<&RoofAnimation>,Option<&Floor>),With<InstanceDataMarker<StoneFloorInstanceData>>>,
    gpu:Res<GpuWorld>,
) {
    let tangent=|words:&[u32]| {
        let f=|index|f32::from_bits(words[index]);
        utils::geometry::Circle2d {
            center:glam::Vec2::new(f(0),f(1)),radius:f(2),rotation:f(3),
        }.get_tangent_at_u(0.0)
    };
    for (active,transform,roof,animation,floor) in &rows {
        let mut key=active.key_guard();
        let floor_words=floor.map(Floor::words);
        systems::nani_update_stone_floor(&gpu,&tangent,&mut [StoneFloorRow {
            key:&mut key,transform,
            roof:roof.map(Roof::observed),roof_animation:animation.map(|animation|animation.values()[0]),
            floor:floor_words.as_ref(),
        }]);
    }
}

/// Original142080640 follows the real update tuple in backend set44.
pub fn flush_instance_updates(
    mut gpu:ResMut<GpuWorld>,
    device:Res<crate::render::device::GraphicsDeviceResource>,
) {
    systems::nani_flush_instance_updates(&mut gpu,device.device());
}
