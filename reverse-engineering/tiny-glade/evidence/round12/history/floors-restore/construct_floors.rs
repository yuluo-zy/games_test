//! Complete B construct_floors1407661e0/1888B, original five parameters.
use bevy_ecs::prelude::*;
use country_core::{
    components::drawable::{NaniBundle,NaniInstanceDesc,NaniInstanceMeshDesc},
    render::nani::{gpu_world::{GpuWorldSubsetKind,InstanceDataMarker},instance_data::StoneFloorInstanceData},
    resources::walls::{WallId,Floor,floor::floor_atlas_part,public_walls::{PublicWalls,OnWallChanged,OnWallDeleted}},
    systems::wall::valid_enclosure::ShapeParameters,
    utils::components::transform::Transform,
};

#[derive(Default,Debug)]
pub struct FloorStorage {pub entities:hashbrown::HashMap<WallId,Entity>}

fn remove_floor(storage:&mut FloorStorage,wall:WallId,commands:&mut Commands) {
    if let Some(entity)=storage.entities.remove(&wall) {
        // Original Entities::contains check precedes queuing despawn; stale
        // entries are removed from the map without issuing a failing command.
        if let Ok(mut entity)=commands.get_entity(entity) {entity.despawn();}
    }
}

pub fn construct_floors(
    walls:Res<PublicWalls>,
    mut changed:EventReader<OnWallChanged>,
    mut deleted:EventReader<OnWallDeleted>,
    mut storage:Local<FloorStorage>,
    mut commands:Commands,
) {
    for event in deleted.read() {remove_floor(&mut storage,event.0,&mut commands);}
    for event in changed.read() {
        remove_floor(&mut storage,event.0,&mut commands);
        // Original inline map probe unwraps missing IDs, without introducing
        // PublicWalls::get's separate source-curve validation.
        let wall=walls.get_unchecked(event.0).expect("original construct_floors public wall lookup");
        let published=wall.building_projection();
        let base_y=published.floor_base_y();
        let shape=wall.shape();
        if !(base_y>0.0)||shape.tag()>1 {continue;}
        let center=ShapeParameters::from_snapshot(shape).center();
        let part=floor_atlas_part(shape);
        let entity=commands.spawn((
            NaniBundle::<StoneFloorInstanceData>{
                desc:NaniInstanceDesc{mesh:NaniInstanceMeshDesc::AtlasPart(part),subset_kind:GpuWorldSubsetKind::RoofFloor},
                transform:Transform::from_translation(glam::Vec3::new(center.x,base_y,center.y)),
                marker:InstanceDataMarker::default(),
            },
            Floor{wall:event.0,shape:shape.clone()},
        )).id();
        storage.entities.insert(event.0,entity);
    }
}
