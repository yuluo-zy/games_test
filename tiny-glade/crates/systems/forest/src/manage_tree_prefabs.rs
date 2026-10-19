//! B forest::manage_tree_prefabs141bf16c0/5342B. Canonical80B placement
//! records and24B animation components have distinct owners and lifetimes.
use bevy_ecs::prelude::*;
use glam::{Vec2,Vec3};
use country_core::{
    asset_libraries::ssbo_library::AssetSsboLibrary,
    components::TreeMarkedForDeletion,
    render::nani::instance_updates::pack_2x16f_uint,
    resources::{events::ConstructTimberframeCmd,
        terrain_height::{TerrainHeightsData,TerrainHeightSsbo},
        terrain_rock_mask::TerrainRockMaskSsbo,time::GameTime,
        tree_storage::{ForestTree,InteractableForestPlacements,InteractableForestTree},
        world_raster_defs::{GardenRaster,WaterRaster}},
    systems::{audio::tree_audio::TreeAmbience,paths::PathRastersResources,
        prefab::SpawnPrefabCmd,ui::signifiers::signifier_create::UiSignifierStream,
        wall::{valid_enclosure::ShapeParameters,wall_state::wall_spatial_hash::WallSpatialHash}},
};
use utils::geometry::Rectangle2d;

pub const WORLD_BOUND:f32=65.0;
pub const TREE_SHAPE_GAP:f32=7.0;
pub const TREE_SCALE_MIN:f32=0.3;
pub const TREE_SCALE_MAX:f32=0.75;
pub const HEIGHT_JITTER:f32=10.0;
pub const RASTER_SCALE:f32=130.0;
pub const DENSITY_MAX:f32=1.0;

#[derive(Default)]
pub struct MemCache {pub candidates:Vec<(Option<Entity>,bool)>}

pub fn is_too_close_to_shape(point:Vec2,center:Vec2,radius:f32)->bool {
    let d=center-point;
    radius+TREE_SHAPE_GAP > (d.y*d.y+d.x*d.x).sqrt()
}

/// Original Rectangle2d::closest_point_on_perimeter140c9ae20: use the
/// existing utility's native edge selection, then its observed projection and
/// weighted endpoint interpolation. Do not convert through arc-length u.
fn closest_rectangle_point(rect:Rectangle2d,point:Vec2)->Vec2 {
    let (a,b)=rect.get_edge_pts(rect.get_closest_edge_id(point));
    let d=b-a;let q=point-a;
    let length=d.y*d.y+d.x*d.x;
    let denominator=if length>1e-20 {length}else{1e-20};
    let raw=(q.y*d.y+q.x*d.x)/denominator;
    let low=if 0.0>raw {0.0}else{raw};
    let t=if 1.0<low {1.0}else{low};
    b*t+a*(1.0-t)
}

fn too_close_to_construct(point:Vec2,shape:&ShapeParameters)->bool {
    let b=shape.bytes();
    let f=|o:usize|f32::from_le_bytes(b[o..o+4].try_into().unwrap());
    if shape.tag()&1==0 {is_too_close_to_shape(point,Vec2::new(f(4),f(8)),f(12))}
    else {
        let rect=Rectangle2d{axis:Vec2::new(f(4),f(8)),center:Vec2::new(f(12),f(16)),size:Vec2::new(f(20),f(24))};
        let d=closest_rectangle_point(rect,point)-point;
        TREE_SHAPE_GAP>(d.y*d.y+d.x*d.x).sqrt()
    }
}

#[allow(clippy::too_many_arguments)]
pub fn manage_tree_prefabs<C:Send+Sync+'static>(
    _signifiers:Res<UiSignifierStream>,ssbos:Res<AssetSsboLibrary>,
    mut placements:ResMut<InteractableForestPlacements>,mut commands:Commands,
    mut spawns:EventWriter<SpawnPrefabCmd>,marked:Query<&TreeMarkedForDeletion>,
    mut constructs:EventReader<ConstructTimberframeCmd>,walls:Res<WallSpatialHash>,
    time:Res<GameTime>,heights:Res<TerrainHeightsData<TerrainHeightSsbo>>,
    paths:PathRastersResources,water:Res<WaterRaster>,garden:Res<GardenRaster>,
    mut cache:Local<MemCache>,
) {
    cache.candidates.clear();
    let grids=paths.as_float_grid();
    let water=(**water).as_ref();let garden=(**garden).as_ref();
    // Native gets/downcasts this same mandatory mapped owner before the loop.
    let rock=ssbos.get::<TerrainRockMaskSsbo>().expect("B forest TerrainRockMask mapped SSBO");
    let shape=constructs.read().last().map(|cmd|ShapeParameters::from_observed(cmd.shape));
    for record in &placements.records {
        let p=Vec2::new(record.placement.position.x,record.placement.position.z);
        let keep=if p.x.abs()>WORLD_BOUND || p.y.abs()>WORLD_BOUND {true} else {
            let a=grids[0].bilinear_sample_at_world_xz(p.x,p.y,RASTER_SCALE,RASTER_SCALE);
            let b=grids[1].bilinear_sample_at_world_xz(p.x,p.y,RASTER_SCALE,RASTER_SCALE);
            let c=grids[2].bilinear_sample_at_world_xz(p.x,p.y,RASTER_SCALE,RASTER_SCALE);
            let d=grids[3].bilinear_sample_at_world_xz(p.x,p.y,RASTER_SCALE,RASTER_SCALE);
            let w=water.bilinear_sample_at_world_xz(p.x,p.y,RASTER_SCALE,RASTER_SCALE);
            let g=garden.bilinear_sample_at_world_xz(p.x,p.y,RASTER_SCALE,RASTER_SCALE);
            if w>0.0 || g>0.0 {false} else {
                let rock=TerrainRockMaskSsbo::sample_mapped_at_world_xz(rock.mapped,p.x,p.y)!=0;
                let sum=((a+b)+c)+d;
                // MAXSS 0,sum then MINSS 1,result preserves an unordered source.
                let low=if 0.0>sum {0.0}else{sum};
                let density=if 1.0<low {1.0}else{low};
                if density>0.0 || rock {false}
                else if shape.as_ref().is_some_and(|shape|too_close_to_construct(p,shape)) {false}
                else {!walls.has_occupied_cells_in_circle(p,TREE_SHAPE_GAP)}
            }
        };
        cache.candidates.push((record.entity,keep));
    }
    apply_candidates(&mut placements,&cache.candidates,time.frame_count,&marked,
        &mut commands,&mut spawns,|x,z|heights.sample_at_world_xz(x,z));
}

/// Phase2 is shared with the real ECS regression so its deferred commands,
/// full prefab payload and canonical entity writeback are exercised directly.
fn apply_candidates(
    placements:&mut InteractableForestPlacements,candidates:&[(Option<Entity>,bool)],
    frame:u32,marked:&Query<&TreeMarkedForDeletion>,commands:&mut Commands,
    spawns:&mut EventWriter<SpawnPrefabCmd>,mut height:impl FnMut(f32,f32)->f32,
) {
    let mut rng=fastrand::Rng::with_seed(utils::rng::init_rng(frame));
    for (index,(entity,keep)) in candidates.iter().copied().enumerate() {
        // Native advances its local RNG for every candidate after processed,
        // including rejected/existing entities, before selecting any branch.
        let (initial_scale,progress,delay)=if placements.processed {
            let r=rng.f32();(0.0,0.0,r*TREE_SCALE_MAX+(1.0-r)*TREE_SCALE_MIN)
        } else {(placements.records[index].placement.scale,1.0,0.0)};
        if keep {
            if let Some(entity)=entity {
                if marked.contains(entity) {commands.entity(entity).remove::<TreeMarkedForDeletion>();}
            } else {
                let record=&mut placements.records[index];
                let entity=commands.spawn_empty().insert(InteractableForestTree {
                    placement_index:index,amplitude:record.placement.scale,progress,delay,
                }).id();
                let mut position=record.placement.position;
                if -WORLD_BOUND<=position.x && position.x<WORLD_BOUND
                    && -WORLD_BOUND<=position.z && position.z<WORLD_BOUND {
                    position.y=height(position.x,position.z);
                    // Preserve native zero multiplications and their NaN/-0 behavior.
                    let xz_jitter=initial_scale*0.0;
                    commands.entity(entity).insert(TreeAmbience {position:Vec3::new(
                        position.x+xz_jitter,position.y+initial_scale*HEIGHT_JITTER,
                        xz_jitter+position.z)});
                }
                spawns.write(prefab_command(record,entity,position,initial_scale));
                record.entity=Some(entity);
            }
        } else if let Some(entity)=entity {
            if !marked.contains(entity) {commands.entity(entity).insert(TreeMarkedForDeletion);}
        }
    }
    placements.processed=true;
}

fn prefab_command(record:&ForestTree,entity:Entity,position:Vec3,initial_scale:f32)->SpawnPrefabCmd {
    let p=record.placement;
    let mut flags=[0u8;16];
    flags[0..4].copy_from_slice(&p.custom1.to_le_bytes());
    flags[4..8].copy_from_slice(&pack_2x16f_uint(initial_scale,1.0).to_le_bytes());
    flags[8..12].copy_from_slice(&p.bend_vector.x.to_le_bytes());
    flags[12..16].copy_from_slice(&p.bend_vector.y.to_le_bytes());
    SpawnPrefabCmd {prefab_name:Some(record.prefab_name.clone()),entity:Some(entity),
        position,scale:initial_scale,rotation:p.rotation_angle,variant:p.id as u32,
        one:record.billboard_expansion_scale,flags,mode:p.pad0 as u32}
}

#[cfg(test)]
mod native_owner_tests {
    use super::*;
    use bevy_ecs::system::RunSystemOnce;
    use country_core::{resources::{placement_data::PlacementDataSsbo,
        render::mesh_atlas_library::PrefabName},systems::prefab::{DespawnPrefabCmd,PrefabInstanceData}};
    include!("native_forest_fixtures.rs");

    #[test]
    fn rectangle_projection_matches_84_original_executions() {
        for &(r,p,expected) in RECTANGLE_CASES {
            let f=|v:u32|f32::from_bits(v);
            let rect=Rectangle2d {axis:Vec2::new(f(r[0]),f(r[1])),center:Vec2::new(f(r[2]),f(r[3])),size:Vec2::new(f(r[4]),f(r[5]))};
            let actual=closest_rectangle_point(rect,Vec2::new(f(p[0]),f(p[1])));
            assert_eq!([actual.x.to_bits(),actual.y.to_bits()],expected,"rect={r:?}, point={p:?}");
        }
    }

    fn record(position:Vec3)->ForestTree {
        ForestTree {prefab_name:PrefabName::new("original/nonempty-tree"),entity:None,
            billboard_expansion_scale:1.625,placement:PlacementDataSsbo {
                position,scale:2.0,rotation_angle:-23.5,id:-7,custom0:0x12345678,
                custom1:0x89abcdef,anim_t_packed:0x01020304,
                bend_vector:Vec2::new(0.125,-0.75),pad0:-19,
            }}
    }
    fn world(records:Vec<ForestTree>)->World {
        let mut world=World::new();
        world.insert_resource(InteractableForestPlacements {records,processed:false});
        world.insert_resource(Events::<SpawnPrefabCmd>::default());
        world.insert_resource(Events::<DespawnPrefabCmd>::default());
        world.insert_resource(GameTime{delta:1.0,..Default::default()});
        world
    }
    fn phase(world:&mut World,keep:bool,frame:u32) {
        world.run_system_once(move |mut owner:ResMut<InteractableForestPlacements>,
            marked:Query<&TreeMarkedForDeletion>,mut commands:Commands,
            mut spawns:EventWriter<SpawnPrefabCmd>| {
            let candidates:Vec<_>=owner.records.iter().map(|r|(r.entity,keep)).collect();
            apply_candidates(&mut owner,&candidates,frame,&marked,&mut commands,&mut spawns,
                |x,z| {assert_eq!([x,z],[3.0,4.0]);7.0});
        }).unwrap();
    }
    #[test]
    fn actual_deferred_entity_payload_delete_and_regrow_share_one_record() {
        let mut world=world(vec![record(Vec3::new(3.0,99.0,4.0))]);
        phase(&mut world,true,7);
        let entity=world.resource::<InteractableForestPlacements>().records[0].entity.unwrap();
        let tree=*world.get::<InteractableForestTree>(entity).unwrap();
        assert_eq!((tree.placement_index,tree.amplitude,tree.progress,tree.delay),(0,2.0,1.0,0.0));
        assert_eq!(world.get::<TreeAmbience>(entity).unwrap().position,Vec3::new(3.0,27.0,4.0));
        let mut cursor=bevy_ecs::event::EventCursor::<SpawnPrefabCmd>::default();
        let cmd=cursor.read(world.resource::<Events<SpawnPrefabCmd>>()).next().unwrap().clone();
        assert_eq!(cmd.entity,Some(entity));
        assert_eq!(cmd.prefab_name.as_ref().unwrap().0.as_ref(),"original/nonempty-tree");
        assert_eq!(cmd.position,Vec3::new(3.0,7.0,4.0));assert_eq!(cmd.scale,2.0);
        let p=cmd.placement();assert_eq!(p.rotation_angle,-23.5);assert_eq!(p.id,-7);
        assert_eq!(p.custom0,1.625_f32.to_bits());assert_eq!(p.custom1,0x89abcdef);
        assert_eq!(p.anim_t_packed,pack_2x16f_uint(2.0,1.0));
        assert_eq!(p.bend_vector,Vec2::new(0.125,-0.75));assert_eq!(p.pad0,-19);
        world.entity_mut(entity).insert(PrefabInstanceData(p));
        world.entity_mut(entity).insert(TreeMarkedForDeletion);
        phase(&mut world,true,8);
        assert!(world.get::<TreeMarkedForDeletion>(entity).is_none(),"keep reverses pending deletion");
        phase(&mut world,false,9);
        assert!(world.get::<TreeMarkedForDeletion>(entity).is_some());
        assert_eq!(world.resource::<Events<SpawnPrefabCmd>>().len(),1,"existing entity must never spawn twice");
        world.run_system_once(crate::tree_animation::forest_tree_animation).unwrap();
        assert_eq!(world.resource::<Events<DespawnPrefabCmd>>().len(),1);
        assert!(world.resource::<InteractableForestPlacements>().records[0].entity.is_none());
        phase(&mut world,true,10);
        let replacement=world.resource::<InteractableForestPlacements>().records[0].entity.unwrap();
        assert_ne!(replacement,entity);
        let tree=world.get::<InteractableForestTree>(replacement).unwrap();
        let mut rng=fastrand::Rng::with_seed(utils::rng::init_rng(10));let r=rng.f32();
        assert_eq!(tree.amplitude,2.0);assert_eq!(tree.progress,0.0);
        assert_eq!(tree.delay.to_bits(),(r*0.75+(1.0-r)*0.3).to_bits());
        let cmd=cursor.read(world.resource::<Events<SpawnPrefabCmd>>()).next().unwrap();
        assert_eq!(cmd.scale,0.0);assert_eq!(cmd.entity,Some(replacement));
    }
    #[test]
    fn outside_forest_placement_retains_saved_height_and_has_no_ambience_component() {
        let mut world=world(vec![record(Vec3::new(65.0,99.0,-65.0))]);
        phase(&mut world,true,0);
        let entity=world.resource::<InteractableForestPlacements>().records[0].entity.unwrap();
        assert!(world.get::<TreeAmbience>(entity).is_none());
        let mut cursor=bevy_ecs::event::EventCursor::<SpawnPrefabCmd>::default();
        let cmd=cursor.read(world.resource::<Events<SpawnPrefabCmd>>()).next().unwrap();
        assert_eq!(cmd.position,Vec3::new(65.0,99.0,-65.0));
    }
    #[test]
    fn actual_forest_animation_progress_matches_280_native_sse_cases() {
        for &(input,expected) in PROGRESS_CASES {
            let mut world=world(vec![record(Vec3::new(65.0,99.0,-65.0))]);
            world.resource_mut::<GameTime>().delta=f32::from_bits(input[1]);
            let entity=world.spawn((InteractableForestTree{placement_index:0,amplitude:2.0,
                progress:f32::from_bits(input[0]),delay:f32::from_bits(input[2])},
                PrefabInstanceData(world.resource::<InteractableForestPlacements>().records[0].placement))).id();
            if input[3]!=0 {world.entity_mut(entity).insert(TreeMarkedForDeletion);}
            world.resource_mut::<InteractableForestPlacements>().records[0].entity=Some(entity);
            world.run_system_once(crate::tree_animation::forest_tree_animation).unwrap();
            let tree=world.get::<InteractableForestTree>(entity).unwrap();
            assert_eq!([tree.progress.to_bits(),tree.delay.to_bits()],expected,"native inputs {input:?}");
        }
    }
}
