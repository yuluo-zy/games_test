//! UNVERIFIED parallel integration of the recovered decorator cancellation owner.
//! Info72 is opaque. Projection requires an explicit observed Info→request binding.
use std::{any::Any,collections::{HashMap,HashSet},sync::Arc};
use bevy_ecs::prelude::*;
use crate::{BuildingReport,decorator_placement::PlaceWallOpening,openings::{OpeningId,RemoveOpening}};
use tiny_glade_reconstruction::building_wall_decorator_archivist::{
    DecoratorArchivist,DecoratorStorage,DecoratorStorageAddress,ObservedDecoratorInfo,
    restore_archived_decorators,
};
/// The real caller retains collision resource handles, not fabricated collider
/// geometry. Clearing these maps releases those dependency-owned references.
pub type ObservedCollisionHandle=Arc<dyn Any+Send+Sync>;
pub type ObservedDecoratorArchive=DecoratorArchivist<ObservedCollisionHandle>;
#[derive(Event,Clone)]
pub struct ObserveDecoratorStorage(pub Arc<DecoratorStorage>);
#[derive(Event,Clone)]
pub struct ObserveDecoratorArchive(pub Arc<ObservedDecoratorArchive>);
#[derive(Event,Clone,Copy)]
pub struct ObserveDecoratorOpeningBinding {
    pub decorator_id:u32,
    pub observed_info:[u8;72],
    /// Supplied by the actual opening request owner. This is never decoded
    /// by assuming positions or OpeningId are hidden at guessed Info offsets.
    pub opening:PlaceWallOpening,
}
#[derive(Resource,Default)]
pub struct WallDecoratorArchiveState {
    pub storage:Option<DecoratorStorage>,
    pub archive:Option<ObservedDecoratorArchive>,
    pub restored_this_frame:bool,
    pub restored_decorators_this_frame:Vec<u32>,
    pub pending_projection:HashMap<u32,String>,
    bindings:HashMap<u32,Vec<ObserveDecoratorOpeningBinding>>,
    published_openings:HashMap<u32,PlaceWallOpening>,
    projection_dirty:bool,
}
#[derive(Clone,Debug)]
pub enum CancelRestorePending {MissingStorage,MissingArchive,Restore(String)}
pub fn register(app:&mut bevy_app::App){
    app.init_resource::<WallDecoratorArchiveState>()
        .add_event::<ObserveDecoratorStorage>()
        .add_event::<ObserveDecoratorArchive>()
        .add_event::<ObserveDecoratorOpeningBinding>();
}
fn clone_archive(a:&ObservedDecoratorArchive)->ObservedDecoratorArchive{
    DecoratorArchivist{walls:a.walls.clone(),special1:a.special1.clone(),special2:a.special2.clone(),collision_a:a.collision_a.clone(),collision_b:a.collision_b.clone()}
}
pub(crate) fn consume_observed(
    mut storage:EventReader<ObserveDecoratorStorage>,
    mut archive:EventReader<ObserveDecoratorArchive>,
    mut bindings:EventReader<ObserveDecoratorOpeningBinding>,
    mut state:ResMut<WallDecoratorArchiveState>,
){
    state.restored_this_frame=false;state.restored_decorators_this_frame.clear();
    for event in storage.read(){state.storage=Some((*event.0).clone());state.projection_dirty=true;}
    for event in archive.read(){state.archive=Some(clone_archive(&event.0));}
    for event in bindings.read(){
        let versions=state.bindings.entry(event.decorator_id).or_default();
        // Raw bytes identify a supplied adapter binding, not game snapshot Eq.
        if let Some(old)=versions.iter_mut().find(|b|b.observed_info==event.observed_info&&b.opening.wall==event.opening.wall){*old=*event;}
        else{versions.push(*event);}
        state.projection_dirty=true;
    }
}
/// This mutates all three tracking containers and the live-ID index using the
/// recovered core. Missing original owner state leaves the finish pending.
pub(crate) fn restore_for_cancel(state:&mut WallDecoratorArchiveState)->Result<(),CancelRestorePending>{
    // Retain the known pre-cancel surface links even when the observed native
    // storage and the finish request arrive in the same adapter frame.
    let known_before:HashMap<_,_>=state.storage.as_ref().ok_or(CancelRestorePending::MissingStorage)?
        .walls.iter().flat_map(|(wall,d)|d.items.iter().filter_map(|(id,info)|{
            state.bindings.get(id).and_then(|v|v.iter().find(|b|b.observed_info==info.0&&b.opening.wall==*wall))
                .map(|b|(*id,b.opening))
        })).collect();
    let storage=state.storage.as_mut().ok_or(CancelRestorePending::MissingStorage)?;
    let archive=state.archive.as_mut().ok_or(CancelRestorePending::MissingArchive)?;
    let restored:Vec<_>=archive.walls.values().chain(archive.special1.iter()).chain(archive.special2.iter())
        .flat_map(|d|d.items.keys().copied()).collect();
    restore_archived_decorators(archive,storage).map_err(|e|CancelRestorePending::Restore(format!("{e:?}")))?;
    state.published_openings.extend(known_before);
    state.restored_this_frame=true;state.restored_decorators_this_frame=restored;state.projection_dirty=true;Ok(())
}
fn live_records(storage:&DecoratorStorage)->Vec<(u32,DecoratorStorageAddress,&ObservedDecoratorInfo)>{
    storage.walls.iter().flat_map(|(wall,d)|d.items.iter().map(move|(id,info)|(*id,DecoratorStorageAddress::Wall(*wall),info)))
        .chain(storage.special1.iter().flat_map(|d|d.items.iter().map(|(id,info)|(*id,DecoratorStorageAddress::ObservedSpecial1,info))))
        .chain(storage.special2.iter().flat_map(|d|d.items.iter().map(|(id,info)|(*id,DecoratorStorageAddress::ObservedSpecial2,info)))).collect()
}
/// Publish actual retained placement requests and clear stale known openings.
/// Unknown Info fields remain gaps while CPU storage restoration still commits.
pub(crate) fn publish_projection(
    mut state:ResMut<WallDecoratorArchiveState>,
    mut placements:EventWriter<PlaceWallOpening>,
    mut removals:EventWriter<RemoveOpening>,
    mut report:ResMut<BuildingReport>,
){
    if !state.projection_dirty{return;}
    let Some(storage)=state.storage.as_ref()else{return;};
    let mut desired=HashMap::new();let mut gaps=HashMap::new();
    for(id,address,info)in live_records(storage){
        let DecoratorStorageAddress::Wall(wall)=address else{
            gaps.insert(id,"Special storage Info72 projection owner is unresolved".to_string());continue;
        };
        if storage.id_to_address.get(&id)!=Some(&address){gaps.insert(id,"Live decorator index does not match the actual storage owner".to_string());continue;}
        let binding=state.bindings.get(&id).and_then(|v|v.iter().find(|b|b.observed_info==info.0&&b.opening.wall==wall));
        if let Some(binding)=binding{desired.insert(id,binding.opening);}
        else{gaps.insert(id,"Info72 has no observed wall-opening binding".to_string());}
    }
    // Openings consume upserts before removals. Preserve IDs also requested
    // by the restored version to avoid removing a same-frame replacement.
    let desired_ids:HashSet<OpeningId>=desired.values().map(|p|p.id).collect();
    let removed_ids:HashSet<_>=state.published_openings.values().map(|p|p.id).filter(|id|!desired_ids.contains(id)).collect();
    for id in removed_ids{removals.write(RemoveOpening(id));}
    for opening in desired.values(){placements.write(*opening);}
    for(id,gap)in &gaps{report.faults.push(format!("Decorator {id} restoration projection pending: {gap}"));}
    state.pending_projection=gaps;state.published_openings=desired;state.projection_dirty=false;
}
