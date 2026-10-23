//! Shared critter movement policies, original B1423179a0/142318c10/142318fa0.
//! Native Query bookkeeping writes change ticks, not velocity multipliers.
//! Canonical mapped mask, Water/Garden rasters and RaycastWorld are mandatory.
use bevy_ecs::prelude::*;
use country_core::{
    asset_libraries::ssbo_library::AssetSsboLibrary,
    resources::{terrain_rock_mask::TerrainRockMaskSsbo, time::GameTime,
        walls::public_walls::PublicWalls, world_raster::RasterBufferRef,
        world_raster_defs::{WaterRaster,GardenRaster}},
    systems::{collision::world::RaycastWorld, paths::PathRastersResources,
        ui::signifiers::signifier_create::UiSignifierStream,
        wall::wall_state::wall_spatial_hash::WallSpatialHash},
};
use glam::{Mat3,Quat,Vec2,Vec3};
use utils::geometry::line_relation::{LineInterval,LineRelation};
use crate::components::{BirdSpots,CritterGoal,CritterLocomotion,CritterVelocityMultiplier,
    CritterVelocityTarget,Duck,Sheep,Transform,Velocity};

/// Original typed Query is exactly (Target, mut Velocity), with no multiplier
/// filter. Its changed-tick column must not be reinterpreted as business data.
pub fn smooth_critter_velocities(time:Res<GameTime>,mut query:Query<(&CritterVelocityTarget,&mut Velocity)>) {
    for (target,mut velocity) in &mut query { smooth_velocity(&mut velocity.0,*target,time.delta); }
}
pub fn smooth_velocity(velocity:&mut Vec3,target:CritterVelocityTarget,dt:f32) {
    let retention=target.retention_per_second.powf(dt);let weight=1.0-retention;
    *velocity=Vec3::new(velocity.x*retention+target.direction.x*weight,
        velocity.y*retention+target.direction.y*weight,
        retention*velocity.z+weight*target.direction.z);
}

/// External world reads are separated from the per-entity game policy so the
/// exact same policy can consume recorded native boundary inputs in tests.
/// Implementations return actual samples/hits; they do not choose movement.
pub trait CritterEnvironment {
    fn water(&self,position:Vec2)->f32;
    fn garden(&self,position:Vec2)->f32;
    fn rock(&self,position:Vec2)->u32;
    fn paths(&self,position:Vec2)->[f32;4];
    fn wall_intersection(&self,position:Vec3,end:Vec2,locomotion:u32,require_public_for_flying:bool)->bool;
    fn spherecast(&self,origin:Vec3,direction:Vec3,radius:f32)->Option<Vec3>;
}
struct LiveEnvironment<'a> {
    water:RasterBufferRef<'a,512>,garden:Option<RasterBufferRef<'a,512>>,
    paths:Option<[RasterBufferRef<'a,512>;4]>,rock:&'a [TerrainRockMaskSsbo],
    walls:&'a PublicWalls,hash:&'a WallSpatialHash,raycast:&'a RaycastWorld,
}
impl CritterEnvironment for LiveEnvironment<'_> {
    fn water(&self,p:Vec2)->f32 {self.water.bilinear_sample_at_world_xz(p.x,p.y,130.0,130.0)}
    fn garden(&self,p:Vec2)->f32 {self.garden.expect("move_critters requires GardenRaster").bilinear_sample_at_world_xz(p.x,p.y,130.0,130.0)}
    fn rock(&self,p:Vec2)->u32 {TerrainRockMaskSsbo::sample_mapped_at_world_xz(self.rock,p.x,p.y)}
    fn paths(&self,p:Vec2)->[f32;4] {self.paths.as_ref().expect("avoid requires four PathRasters").map(|grid|grid.bilinear_sample_at_world_xz(p.x,p.y,130.0,130.0))}
    fn wall_intersection(&self,p:Vec3,end:Vec2,loc:u32,require:bool)->bool {wall_intersection(self.hash,self.walls,p,end,loc,require)}
    fn spherecast(&self,origin:Vec3,direction:Vec3,radius:f32)->Option<Vec3> {
        let hit=self.raycast.legacy_spherecast(origin,direction,radius);
        // Native legacy +0x30 discriminant2 means miss, never "rock slot2".
        hit.is_hit().then_some(hit.position)
    }
}

/// Native query140aa5730 returns only the current cell's original segment rows.
/// Move loc3 skips PublicWalls::get entirely; avoid still requires that owner,
/// and skips only its height test. Keep those two different failure contracts.
pub fn wall_intersection(hash:&WallSpatialHash,walls:&PublicWalls,p:Vec3,end:Vec2,loc:u32,require_public_for_flying:bool)->bool {
    let path=LineInterval::line_segment(Vec2::new(p.x,p.z),end);
    for segment in hash.segments_at_world_xz([p.x,p.z]) {
        if loc!=3 || require_public_for_flying {
            let state=walls.get(segment.wall).expect("critters require PublicWalls segment owner");
            if loc!=3 && state.max_y_gable()<p.y {continue;}
        }
        let wall=LineInterval::line_segment(Vec2::from_array(segment.endpoints[0]),Vec2::from_array(segment.endpoints[1]));
        if matches!(path.relate(wall),LineRelation::Intersection(_)) {return true;}
    }
    false
}
fn locomotion_ignores_spheres(loc:u32)->bool {loc<7 && (99u32>>loc)&1!=0}
fn distance_from_hit(origin:Vec3,hit:Vec3)->f32 {
    let delta=hit-origin;((delta.y*delta.y+delta.x*delta.x)+delta.z*delta.z).sqrt()
}
fn unchecked_direction(v:Vec3)->Vec3 {
    let inverse=1.0/(v.z*v.z+(v.y*v.y+v.x*v.x)).sqrt();v*inverse
}
fn clamp01_native(v:f32)->f32 {
    let lower=if 0.0>v {0.0}else{v};if 1.0<lower {1.0}else{lower}
}
fn path_sum(values:[f32;4])->f32 {clamp01_native(((values[0]+values[1])+values[2])+values[3])}

/// Full native per-entity movement, including blocked-path early returns.
#[allow(clippy::too_many_arguments)]
pub fn advance_critter(env:&impl CritterEnvironment,transform:&mut Transform,loc:u32,
    velocity:Vec3,multiplier:Option<f32>,duck:bool,sheep:bool,spots:Option<&mut BirdSpots>,dt:f32,rotation_retention:f32)->bool {
    // MAXSS y,0 selects positive zero for NaN and negative/negative-zero y.
    transform.translation.y=if transform.translation.y>0.0 {transform.translation.y}else{0.0};
    let mult=multiplier.unwrap_or(1.0);
    if mult==0.0 {return false;}
    // Preserve the original two multiplies (velocity*dt)*mult on every lane.
    let displacement=(velocity*dt)*mult;
    let position=transform.translation;
    let predicted=Vec2::new(position.x+displacement.x,position.z+displacement.z);
    let water=env.water(predicted);
    if loc==2 {
        let rock=env.rock(predicted);
        if rock!=0 || water>0.0 || (!(water>0.0) && duck) {return false;}
    } else if !(water>0.0) && duck {return false;}
    if duck || sheep {
        if !locomotion_ignores_spheres(loc) {
            let direction=unchecked_direction(displacement);
            if let Some(hit)=env.spherecast(position,direction,if sheep {0.8}else{0.5}) {
                if distance_from_hit(position,hit)<1.0 {return false;}
            }
            // Original stack+118 is GardenRaster, distinct from WaterRaster
            // at+E8. Sheep loc4 alone skips this two-position garden boundary.
            if sheep && loc!=4 {
                let current=env.garden(Vec2::new(position.x,position.z));
                let predicted=env.garden(predicted);
                if !(current>0.0) && predicted>0.0 {return false;}
            }
        }
    } else if env.wall_intersection(position,predicted,loc,false) {return false;}
    transform.translation+=displacement;
    let flat=Vec3::new(displacement.x,0.0,displacement.z);
    if let Some(direction)=flat.try_normalize() {
        let target=Quat::from_rotation_arc(-Vec3::Z,direction);
        let sign=target.dot(transform.rotation).to_bits()&0x8000_0000;
        let old=transform.rotation.to_array().map(|lane|f32::from_bits(lane.to_bits()^sign));
        let retention=rotation_retention;let weight=1.0-retention;let next=target.to_array();
        let blended=Quat::from_array(core::array::from_fn(|i|old[i]*retention+next[i]*weight));
        let length=blended.length();
        transform.rotation=Quat::from_array(blended.to_array().map(|lane|lane/length));
    }
    if let Some(spots)=spots {
        let position=bird_spot_position(transform);
        for spot in &mut spots.spots {spot.pos=position;}
    }
    true
}
pub fn bird_spot_position(transform:&Transform)->Vec3 {
    let basis=Mat3::from_quat(transform.rotation);
    // Native local offset is (0,1.567,-0.495), with all three scale lanes.
    // Retain the zero-x term: scale.x=Inf/NaN has observable IEEE behavior.
    let offset=(basis.z_axis*transform.scale.z*-0.495+basis.y_axis*transform.scale.y*1.567)
        +basis.x_axis*transform.scale.x*0.0;
    transform.translation+offset
}

#[allow(clippy::too_many_arguments,clippy::type_complexity)]
pub fn move_critters(time:Res<GameTime>,water:Res<WaterRaster>,garden:Res<GardenRaster>,
    buffers:Res<AssetSsboLibrary>,raycast:Res<RaycastWorld>,walls_hash:Res<WallSpatialHash>,walls:Res<PublicWalls>,
    mut query:Query<(&mut Transform,&CritterLocomotion,&Velocity,Option<&CritterVelocityMultiplier>,Option<&Duck>,Option<&Sheep>,Option<&mut BirdSpots>)>,
    readbacks:Option<Res<country_core::render::gpu_readback::GpuReadbacks>>) {
    buffers.get::<TerrainRockMaskSsbo>().expect("move_critters requires canonical mapped TerrainRockMaskSsbo");
    // Render-thread publication; the live mapping belongs to the in-flight GPU frame.
    let rock=TerrainRockMaskSsbo::published(readbacks.as_deref());
    let env=LiveEnvironment {water:water.0.as_ref(),garden:Some(garden.0.as_ref()),paths:None,
        rock,walls:&walls,hash:&walls_hash,raycast:&raycast};
    let rotation_retention=0.01_f32.powf(time.delta);
    for (mut transform,loc,velocity,multiplier,duck,sheep,mut spots) in &mut query {
        advance_critter(&env,&mut transform,loc.0,velocity.0,multiplier.map(|value|value.0),duck.is_some(),sheep.is_some(),spots.as_deref_mut(),time.delta,rotation_retention);
    }
}

/// Returns whether the original Duck despawn event must be emitted.
pub fn avoid_critter_collision(env:&impl CritterEnvironment,transform:&Transform,goal:&mut CritterGoal,loc:u32,duck:bool,sheep:bool)->bool {
    if goal.goal_tag!=1 {return false;}
    let position=transform.translation;let to=goal.position-position;
    let direction=Vec2::new(to.x,to.z).normalize_or_zero();
    let probe=Vec2::new(position.x+direction.x*0.1,position.z+direction.y*0.1);
    let water=env.water(probe);
    if loc==2 && (env.rock(probe)!=0 || water>0.0) {goal.goal_tag=0;return false;}
    if duck {
        let current=path_sum(env.paths(Vec2::new(position.x,position.z)));
        let distance=if current>0.0 {goal.position.length()}else{0.3};
        let predicted=Vec2::new(position.x+direction.x*distance,position.z+direction.y*distance);
        if path_sum(env.paths(predicted))>0.0 {goal.goal_tag=0;return false;}
    } else if !sheep {
        // This endpoint is goal+unit-direction, not position+unit-direction.
        let end=Vec2::new(goal.position.x+direction.x,goal.position.z+direction.y);
        if env.wall_intersection(position,end,loc,true) {goal.goal_tag=0;}
        return false;
    }
    if locomotion_ignores_spheres(loc) {return false;}
    let direction=unchecked_direction(Vec3::new(direction.x,0.0,direction.y));
    if let Some(hit)=env.spherecast(position,direction,if sheep {0.8}else{0.5}) {
        let distance=distance_from_hit(position,hit);
        if distance<1.0 {
            goal.goal_tag=0;
            return duck && distance<0.4;
        }
    }
    false
}
#[allow(clippy::too_many_arguments,clippy::type_complexity)]
pub fn critters_avoid_collisions(water:Res<WaterRaster>,buffers:Res<AssetSsboLibrary>,walls_hash:Res<WallSpatialHash>,walls:Res<PublicWalls>,raycast:Res<RaycastWorld>,
    path_rasters:PathRastersResources,_signifiers:Res<UiSignifierStream>,mut ev_despawn:EventWriter<crate::spawn_ducks::DespawnDuckCmd>,
    mut query:Query<(Entity,&Transform,&mut CritterGoal,&CritterLocomotion,Option<&Sheep>,Option<&Duck>)>,
    readbacks:Option<Res<country_core::render::gpu_readback::GpuReadbacks>>) {
    buffers.get::<TerrainRockMaskSsbo>().expect("critters_avoid_collisions requires canonical mapped TerrainRockMaskSsbo");
    let rock=TerrainRockMaskSsbo::published(readbacks.as_deref());
    let env=LiveEnvironment {water:water.0.as_ref(),garden:None,paths:Some(path_rasters.as_float_grid()),
        rock,walls:&walls,hash:&walls_hash,raycast:&raycast};
    for (entity,transform,mut goal,loc,sheep,duck) in &mut query {
        if avoid_critter_collision(&env,transform,&mut goal,loc.0,duck.is_some(),sheep.is_some()) {
            ev_despawn.write(crate::spawn_ducks::DespawnDuckCmd {entity});
        }
    }
}

/// Compatibility name now has the same Resource TypeId as the real core world;
/// no private empty world or unimplemented slot_query remains.
pub use country_core::systems::collision::world::RaycastWorld as RaycastWorldHandle;
pub const LEGACY_SPHERECAST_SLOT_ORDER:[u32;10]=[0,1,3,7,6,4,11,2,12,13];
#[derive(Clone,Copy,Debug,Default)]
pub struct SpherecastResult {pub hit:bool,pub distance:f32,pub hitcode:u32}
pub fn legacy_spherecast(raycast:&RaycastWorldHandle,origin:Vec3,direction:Vec3,radius:f32)->SpherecastResult {
    let hit=raycast.legacy_spherecast(origin,direction,radius);
    if hit.is_hit() {SpherecastResult {hit:true,distance:distance_from_hit(origin,hit.position),hitcode:0}}
    else {SpherecastResult {hit:false,distance:f32::MAX,hitcode:2}}
}

/// Historical shadow retained only for still-unrestored species leaf APIs.
/// The recovered shared systems above never consume or initialize this owner.
#[derive(Resource,Clone,Debug,Default)]
pub struct TerrainRockMask {mask:Vec<u32>}
impl TerrainRockMask {pub fn mask(&self)->&[u32] {&self.mask} pub fn set_mask(&mut self,mask:Vec<u32>) {self.mask=mask;}}
