//! Original forest-growth pool: init140aa18f0/1543B, consumer140aa1f00/4295B.
//! Fifty independent canonical AudioLoops, free FIFO and placement-index map.
use std::{collections::VecDeque,hash::RandomState};
use hashbrown::HashMap;
use bevy_ecs::prelude::*;
use crate::{resources::{audio_resources::{AudioCore,AudioInstance,AudioLoopDesc,EventInstance,Studio},
    gui::EscMenuState,tree_storage::InteractableForestTree},
    systems::prefab::PrefabInstanceData,utils::audio_utils::AudioLoop};

pub const FOREST_GROW_EVENT:&str="event:/SFX/ForestGrow";
pub const FOREST_AUDIO_POOL_SIZE:usize=50;

#[derive(Resource)]
pub struct ForestAudioState<E:EventInstance> {
    pub players:Vec<AudioLoop<AudioInstance<E>>>,
    pub free:VecDeque<usize>,
    pub active:HashMap<usize,usize,RandomState>,
}
impl<E:EventInstance> ForestAudioState<E> {
    pub fn new<S:Studio<Event=E>>(core:&AudioCore<S>)->Self {
        let mut desc=AudioLoopDesc::new(FOREST_GROW_EVENT);
        desc.count=3;desc.threshold_ms=50;
        // The original first allocates50×0x38 loop records and constructs all
        // loops, then constructs its map and the explicit0..49 free FIFO.
        let players=(0..FOREST_AUDIO_POOL_SIZE).map(|_|core.instantiate_loop(&desc)).collect();
        let active=HashMap::with_hasher(RandomState::new());
        let free=(0..FOREST_AUDIO_POOL_SIZE).collect();
        Self {players,free,active}
    }
}

pub fn forest_audio_init_system<S:Studio+Send+Sync+'static>(core:Res<AudioCore<S>>,mut commands:Commands) {
    commands.insert_resource(ForestAudioState::<S::Event>::new(&core));
}

pub fn forest_audio_system<S:Studio+Send+Sync+'static>(
    mut state:ResMut<ForestAudioState<S::Event>>,
    trees:Query<(&InteractableForestTree,&PrefabInstanceData)>,menu:Res<EscMenuState>,
) {
    let menu=menu.view();
    if !(menu.tag==2 || (menu.tag==10 && menu.flag==0)) {
        // Original retains both ownership collections while requesting stop on
        // every loop. Returning to a playable menu resumes the same mappings.
        for player in &mut state.players {let _=player.stop_now();}
        return;
    }
    let mut seen=Vec::new();
    for (tree,placement) in &trees {
        let index=tree.placement_index;
        seen.push(index); // includes inactive trees and duplicate placement IDs
        if 0.0<tree.progress && tree.progress<1.0 {
            let slot=if let Some(&slot)=state.active.get(&index) {Some(slot)}
            else if let Some(slot)=state.free.pop_front() {
                state.active.insert(index,slot);Some(slot)
            } else {None}; // a full pool never grows or steals another tree's loop
            if let Some(slot)=slot {
                let player=&mut state.players[slot];
                let _=player.start_if_not_playing();
                let p=placement.0.position;
                let _=player.current_mut().set_position([p.x,p.y,p.z]);
            }
        } else if let Some(slot)=state.active.remove(&index) {
            state.free.push_back(slot);
        }
    }
    // Native clones the active keys before mutation; retain source iteration
    // and free-FIFO publication order rather than sorting keys or draining it.
    let keys:Vec<_>=state.active.keys().copied().collect();
    for index in keys {
        if !seen.contains(&index) {
            if let Some(slot)=state.active.remove(&index) {state.free.push_back(slot);}
        }
    }
    // Native clones the free deque before these stop calls. It does not clear
    // the queue and does not issue stop to a slot still assigned to a live tree.
    let free=state.free.clone();
    for slot in free {let _=state.players[slot].stop_now();}
}

#[cfg(test)]
mod original_pool_tests {
    use super::*;
    use std::sync::{Arc,Mutex,atomic::{AtomicUsize,Ordering}};
    use bevy_ecs::system::RunSystemOnce;
    use crate::resources::{audio_instance::PlaybackState,audio_resources::{ListenerAttributes,VcaHandle},
        placement_data::PlacementDataSsbo};
    use glam::Vec3;
    type Log=Arc<Mutex<Vec<String>>>;
    #[derive(Debug)] struct Event {id:usize,log:Log}
    impl EventInstance for Event {
        type Error=String;
        fn start(&mut self)->Result<(),String>{self.log.lock().unwrap().push(format!("start:{}",self.id));Ok(())}
        fn stop(&mut self,_:i32)->Result<(),String>{self.log.lock().unwrap().push(format!("stop:{}",self.id));Ok(())}
        fn get_playback_state(&self)->Result<PlaybackState,String>{Ok(PlaybackState::Stopped)}
        fn set_volume(&mut self,_:f32)->Result<(),String>{Ok(())}
        fn set_parameter_by_name(&mut self,_:&str,_:f32)->Result<(),String>{Ok(())}
        fn has_parameter(&self,_:&str)->Result<bool,String>{Ok(false)}
        fn set_3d_attributes(&mut self,p:[f32;3],_:[f32;3],_:[f32;3],_:[f32;3])->Result<(),String>{
            self.log.lock().unwrap().push(format!("position:{}:{p:?}",self.id));Ok(())}
        fn get_timeline_position(&self)->Result<i32,String>{Ok(0)}
        fn pause(&mut self,_:bool)->Result<(),String>{Ok(())}
        fn install_beat_callback(&mut self)->Result<(),String>{Ok(())}
    }
    struct Vca;
    impl VcaHandle for Vca {type Error=String;fn set_volume(&self,_:f32)->Result<(),String>{Ok(())}}
    struct RecordingStudio {log:Log,next:Arc<AtomicUsize>}
    impl Studio for RecordingStudio {
        type Error=String;type Event=Event;type Vca=Vca;
        fn load_bank_file(&mut self,_:&str)->Result<(),String>{Ok(())}
        fn load_sample_data(&mut self)->Result<(),String>{Ok(())}
        fn get_vca(&self,_:&str)->Result<Vca,String>{Ok(Vca)}
        fn set_listener_attributes(&mut self,_:ListenerAttributes)->Result<(),String>{Ok(())}
        fn update(&mut self)->Result<(),String>{Ok(())}
        fn set_parameter_by_name(&self,_:&str,_:f32)->Result<(),String>{Ok(())}
        fn create_event_instance(&self,name:&str)->Result<Option<Event>,String>{
            assert_eq!(name,FOREST_GROW_EVENT);let id=self.next.fetch_add(1,Ordering::Relaxed);
            Ok(Some(Event{id,log:self.log.clone()}))}
    }
    fn world()->(World,Log,Arc<AtomicUsize>) {
        let log=Log::default();let next=Arc::new(AtomicUsize::new(0));
        let core=AudioCore {studio:Some(RecordingStudio{log:log.clone(),next:next.clone()}),banks:Default::default()};
        let state=ForestAudioState::new(&core);let mut world=World::new();
        world.insert_resource(state);world.insert_resource(EscMenuState::Closed);
        (world,log,next)
    }
    fn tree(world:&mut World,index:usize,progress:f32,x:f32)->Entity {
        world.spawn((InteractableForestTree {placement_index:index,amplitude:1.0,progress,delay:0.0},
            PrefabInstanceData(PlacementDataSsbo{position:Vec3::new(x,2.0,3.0),..Default::default()}))).id()
    }
    fn tick(world:&mut World) {world.run_system_once(forest_audio_system::<RecordingStudio>).unwrap();}
    #[test]
    fn initializes_fifty_native_loops_and_two_hundred_real_instances() {
        let (world,_,next)=world();let s=world.resource::<ForestAudioState<Event>>();
        assert_eq!(s.players.len(),50);assert_eq!(next.load(Ordering::Relaxed),200);
        assert!(s.active.is_empty());assert_eq!(s.free.iter().copied().collect::<Vec<_>>(),(0..50).collect::<Vec<_>>());
        assert!(s.players.iter().all(|p|p.entries.len()==4 && p.threshold_ms==50));
    }
    #[test]
    fn canonical_query_allocates_reuses_releases_disappearing_trees_and_preserves_menu_maps() {
        let (mut world,log,_)=world();let a=tree(&mut world,17,0.25,1.0);let b=tree(&mut world,19,0.5,4.0);
        tick(&mut world);
        assert_eq!(world.resource::<ForestAudioState<Event>>().active.get(&17),Some(&0));
        assert_eq!(world.resource::<ForestAudioState<Event>>().active.get(&19),Some(&1));
        assert!(log.lock().unwrap().contains(&"position:0:[1.0, 2.0, 3.0]".to_string()));
        assert!(log.lock().unwrap().contains(&"position:4:[4.0, 2.0, 3.0]".to_string()));
        let active=world.resource::<ForestAudioState<Event>>().active.clone();
        let free=world.resource::<ForestAudioState<Event>>().free.clone();
        world.insert_resource(EscMenuState::Main);tick(&mut world);
        assert_eq!(world.resource::<ForestAudioState<Event>>().active,active);
        assert_eq!(world.resource::<ForestAudioState<Event>>().free,free);
        assert!(world.resource::<ForestAudioState<Event>>().players.iter().all(|p|p.start.is_some()));
        world.insert_resource(EscMenuState::Photomode(false));
        world.get_mut::<InteractableForestTree>(a).unwrap().progress=1.0;
        world.despawn(b);tick(&mut world);
        let s=world.resource::<ForestAudioState<Event>>();assert!(s.active.is_empty());
        assert_eq!(s.free.iter().copied().collect::<Vec<_>>(),(2..50).chain([0,1]).collect::<Vec<_>>());
        tree(&mut world,23,0.75,8.0);tick(&mut world);
        assert_eq!(world.resource::<ForestAudioState<Event>>().active.get(&23),Some(&2));
    }
    #[test]
    fn full_pool_does_not_allocate_more_slots_and_nan_progress_is_inactive() {
        let (mut world,_,next)=world();for i in 0..51 {tree(&mut world,i,0.5,i as f32);}
        tree(&mut world,99,f32::NAN,0.0);tick(&mut world);
        let s=world.resource::<ForestAudioState<Event>>();assert_eq!(s.active.len(),50);assert!(s.free.is_empty());
        assert!(!s.active.contains_key(&99));assert_eq!(next.load(Ordering::Relaxed),200);
    }
}
