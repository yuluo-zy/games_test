//! Original loading snapshot lifecycle: empty is Option::None, Disabled is
//! a retained AudioInstance. Native audio failures retain their unwrap policy.
use crate::{
    resources::{audio_resources::{AudioCore, AudioInstance, EventInstance, Studio}, time::GameTime},
    systems::save::load_game::{StartupRevealStarted, StartupRevealFinished},
};
use bevy_ecs::prelude::*;

pub const LOADING_SNAPSHOT: &str = "snapshot:/Loading";
pub const LOADING_EVENT: &str = "event:/UI/Loading";

#[derive(Resource)]
pub struct GameLoadingAudioState<E: EventInstance> {
    pub snapshot: Option<AudioInstance<E>>,
    pub loading: bool,
    pub timer: f32,
}

fn instantiate_started<S: Studio>(core: &AudioCore<S>, name: &str) -> AudioInstance<S::Event> {
    let mut instance = core.instantiate_audio(name);
    if let AudioInstance::Err(message) = &instance {
        panic!("original loading audio instantiation failed: {message}");
    }
    instance.start().unwrap();
    instance
}

impl<E: EventInstance> GameLoadingAudioState<E> {
    pub fn new<S: Studio<Event=E>>(core: &AudioCore<S>) -> Self {
        Self { snapshot: Some(instantiate_started(core,LOADING_SNAPSHOT)), loading:false,timer:0.0 }
    }

    /// uninit140a7a7ca..9b7 creates a snapshot only when empty, then writes
    /// loading=true and timer=0.0 as one qword. Existing instances are retained.
    pub fn arm_after_unload<S: Studio<Event=E>>(&mut self, core: &AudioCore<S>) {
        if self.snapshot.is_none() {
            self.snapshot=Some(instantiate_started(core,LOADING_SNAPSHOT));
        }
        self.loading=true;
        self.timer=0.0;
    }

    /// Native take sets the empty tag before stop/drop; failures unwind with
    /// the previous owner still on the stack. Release occurs via AudioInstance.
    pub fn stop_snapshot(&mut self) {
        if let Some(mut instance)=self.snapshot.take() {
            instance.stop().unwrap();
        }
    }
}

/// init140b1ae70 only replaces the snapshot; it preserves timer/loading.
pub fn init_game_loading_audio<S: Studio+Send+Sync+'static>(
    core:Res<AudioCore<S>>, mut state:Option<ResMut<GameLoadingAudioState<S::Event>>>,mut commands:Commands,
) {
    if let Some(state)=state.as_deref_mut() {
        state.snapshot=Some(instantiate_started(&core,LOADING_SNAPSHOT));
    } else {
        commands.insert_resource(GameLoadingAudioState::<S::Event>::new(&core));
    }
}

/// Exact original five-parameter consumer140b1b1b0, including drain-all/one
/// one-shot per frame, immediate finish, and strictly-greater-than-one timer.
/// Actual reveal writers remain independently required; this emits none.
pub fn loading_audio<S: Studio+Send+Sync+'static>(
    core:Res<AudioCore<S>>,mut state:ResMut<GameLoadingAudioState<S::Event>>,
    mut started:EventReader<StartupRevealStarted>,mut finished:EventReader<StartupRevealFinished>,
    time:Res<GameTime>,
) {
    if started.read().last().is_some() {
        let _event=instantiate_started(&core,LOADING_EVENT);
    }
    if finished.read().last().is_some() {
        state.stop_snapshot();
    }
    if state.loading {
        state.timer+=time.delta;
        if state.timer>1.0 {
            state.stop_snapshot();
            state.loading=false;
            // Native140b1b5a8 clears the flag only; timer is retained.
        }
    }
}


#[cfg(test)]
mod lifecycle_native_tests {
    use super::*;
    use crate::resources::{audio_instance::PlaybackState,audio_resources::{ListenerAttributes,VcaHandle}};
    use std::sync::{Arc,Mutex};
    type Log=Arc<Mutex<Vec<String>>>;
    #[derive(Debug)]
    struct Event { name:String,log:Log }
    impl EventInstance for Event {
        type Error=String;
        fn start(&mut self)->Result<(),String>{self.log.lock().unwrap().push(format!("start:{}",self.name));Ok(())}
        fn stop(&mut self,_:i32)->Result<(),String>{self.log.lock().unwrap().push(format!("stop:{}",self.name));Ok(())}
        fn release(&mut self)->Result<(),String>{self.log.lock().unwrap().push(format!("release:{}",self.name));Ok(())}
        fn get_playback_state(&self)->Result<PlaybackState,String>{Ok(PlaybackState::Playing)}
        fn set_volume(&mut self,_:f32)->Result<(),String>{Ok(())}
        fn set_parameter_by_name(&mut self,_:&str,_:f32)->Result<(),String>{Ok(())}
        fn has_parameter(&self,_:&str)->Result<bool,String>{Ok(false)}
        fn set_3d_attributes(&mut self,_:[f32;3],_:[f32;3],_:[f32;3],_:[f32;3])->Result<(),String>{Ok(())}
        fn get_timeline_position(&self)->Result<i32,String>{Ok(0)}
        fn pause(&mut self,_:bool)->Result<(),String>{Ok(())}
        fn install_beat_callback(&mut self)->Result<(),String>{Ok(())}
    }
    struct Vca;
    impl VcaHandle for Vca {type Error=String;fn set_volume(&self,_:f32)->Result<(),String>{Ok(())}}
    struct RecordingStudio(Log);
    impl Studio for RecordingStudio {
        type Error=String;type Event=Event;type Vca=Vca;
        fn load_bank_file(&mut self,_:&str)->Result<(),String>{Ok(())}
        fn load_sample_data(&mut self)->Result<(),String>{Ok(())}
        fn get_vca(&self,_:&str)->Result<Vca,String>{Ok(Vca)}
        fn set_listener_attributes(&mut self,_:ListenerAttributes)->Result<(),String>{Ok(())}
        fn update(&mut self)->Result<(),String>{Ok(())}
        fn set_parameter_by_name(&self,_:&str,_:f32)->Result<(),String>{Ok(())}
        fn create_event_instance(&self,name:&str)->Result<Option<Event>,String>{
            self.0.lock().unwrap().push(format!("create:{name}"));
            Ok(Some(Event{name:name.into(),log:self.0.clone()}))
        }
    }
    fn count(log:&Log,call:&str)->usize { log.lock().unwrap().iter().filter(|s|s.as_str()==call).count() }
    #[test]
    fn original_on_exit_closes_file_clears_nonempty_owners_and_keeps_persistent_entities() {
        use crate::{
            asset_libraries::{placement_data_library::AssetPlacementDataLibrary,
                audio_trigger_library::{AudioTriggerLibrary,AudioTriggerKind}},
            resources::{rt::{RtWorld,RtMeshHandle,RtUploadData},placement_data::PlacementDataSsbo,
                session::SessionId,tree_storage::{InteractableForestPlacements,InteractableForestTree}},
            render::{test_device::TestDevice,nani::gpu_world::{GpuWorld,GpuWorldSubset,
                GpuWorldSubsetKind,OwnedGpuWorldSubset,InstanceCullData}},
            startup::clearing::{begin_glade_unload,uninit_clearing,despawn_glade_entities},
            systems::{save::save_game::{AutoSaveState,AsyncSaveCmd},mode_manager::ClearCanvasCmd,
                audio::water_ambience_audio::WaterAudioGrid},
        };
        let device=TestDevice(19);
        let temp=tempfile::tempdir().unwrap();
        let file=temp.path().join("nonempty-meadow.json");
        std::fs::write(&file,r#"[{"position":[1,2,3],"scale":1,"rotation_angle":0,"id":9}]"#).unwrap();
        let mut placements=AssetPlacementDataLibrary::new();
        let (_,old_handle)=placements.load(&device,file.clone());
        let mut triggers=AudioTriggerLibrary::default();
        triggers.add_trigger(AudioTriggerKind::Primary,crate::utils_contract::Aabb3{
            min:glam::Vec3::ZERO,max:glam::Vec3::ONE});
        let mut rt=RtWorld::default();
        rt.get_mut(RtMeshHandle::from_handle(7)).positions.push(glam::Vec3::ONE);
        rt.next_prefab_handle=91;rt.pending_upload=Some(RtUploadData::default());
        let kind=GpuWorldSubsetKind::VelocityOnly;
        let mut gpu=GpuWorld::new(&device);
        let mut subset=OwnedGpuWorldSubset::new_with_mesh_parts::<PlacementDataSsbo>(&device,kind,7);
        let items=[InstanceCullData::new(glam::Vec3::ONE,2.0,0),InstanceCullData::new(glam::Vec3::splat(3.0),4.0,0)];
        subset.extend_no_data(kind,&items,|_|{});
        gpu.assign_subset(kind,GpuWorldSubset::Owned(subset));gpu.total_instance_count=29;
        let water_base;
        let mut water=WaterAudioGrid::new();
        water.cells[11].set_sample(1<<11,true,[-1,-5]);water_base=water.cells[11].position;
        let forest=InteractableForestPlacements{records:vec![InteractableForestTree::default()],processed:true};
        let (sender,receiver)=crossbeam_channel::unbounded();
        let mut autosave=AutoSaveState::default();let session=SessionId::new();
        autosave.session=session;autosave.attach_command_sender(sender);
        let log=Log::default();
        let core=AudioCore{studio:Some(RecordingStudio(log.clone())),banks:Default::default()};
        let loading=GameLoadingAudioState::new(&core);
        let mut world=World::new();
        world.insert_resource(placements);world.insert_resource(triggers);world.insert_resource(rt);
        world.insert_resource(gpu);world.insert_resource(water);world.insert_resource(forest);
        world.insert_resource(autosave);world.insert_resource(core);world.insert_resource(loading);
        world.init_resource::<Events<ClearCanvasCmd>>();
        let keep=world.spawn(crate::components::KeepOnLevelUnload).id();
        let old0=world.spawn_empty().id();let old1=world.spawn_empty().id();
        let mut schedule=Schedule::default();
        schedule.add_systems((begin_glade_unload,(uninit_clearing::<RecordingStudio>,despawn_glade_entities)).chain());
        schedule.run(&mut world);
        assert!(matches!(receiver.try_recv().unwrap(),AsyncSaveCmd::CloseFile{session:s} if s==session));
        let messages=world.resource::<Events<ClearCanvasCmd>>();
        assert_eq!(messages.iter_current_update_events().map(|e|e.0).collect::<Vec<_>>(),vec![false]);
        assert!(world.get_entity(keep).is_ok());assert!(world.get_entity(old0).is_err());assert!(world.get_entity(old1).is_err());
        let placements=world.resource::<AssetPlacementDataLibrary>();assert!(placements.buffers.is_empty());assert!(placements.paths.is_empty());
        assert_eq!(world.resource::<AudioTriggerLibrary>().get_triggers(AudioTriggerKind::Primary).boxes().count(),0);
        let rt=world.resource::<RtWorld>();assert!(rt.meshes.is_empty());assert_eq!(rt.next_prefab_handle,91);assert!(rt.pending_upload.is_some());assert!(rt.has_pending_changes());
        let gpu=world.resource::<GpuWorld>();assert_eq!(gpu.total_instance_count,29);
        {let mut subset=gpu.owned_subset_mut(kind);assert_eq!(subset.instance_count(),0);assert_eq!(subset.mesh_part_count(),7);
            assert_eq!(subset.positions()[0].position,glam::Vec3::ONE);assert_eq!(subset.positions_dirty().to_range(),Some(0..2));
            let mut keys=Vec::new();subset.extend_no_data(kind,&items[..1],|key|keys.push(key));assert_eq!(keys[0].id,2,"native next_id survives unload");}
        let water=world.resource::<WaterAudioGrid>();assert_eq!(water.cells.len(),100);assert_eq!(water.cells[11].position,water_base);assert_eq!(water.cells[11].bits,0);assert_eq!(water.cells[11].sums,[0,0]);
        let forest=world.resource::<InteractableForestPlacements>();assert!(forest.records.is_empty());assert!(!forest.processed);
        let loading=world.resource::<GameLoadingAudioState<Event>>();assert!(loading.loading);assert_eq!(loading.timer,0.0);
        assert_eq!(count(&log,"create:snapshot:/Loading"),1,"existing snapshot retained across unload");
        let (_,new_handle)=world.resource_mut::<AssetPlacementDataLibrary>().load(&device,file);
        assert_ne!(new_handle,old_handle,"second nonempty meadow load gets a new real owner");
    }
    #[test]
    fn native_started_finished_and_unload_timer_preserve_instance_lifetimes() {
        let log=Log::default();
        let core=AudioCore{studio:Some(RecordingStudio(log.clone())),banks:Default::default()};
        let mut state=GameLoadingAudioState::new(&core);
        state.arm_after_unload(&core);
        assert_eq!(count(&log,"create:snapshot:/Loading"),1,"retain existing snapshot");
        let mut world=World::new();
        world.insert_resource(core);world.insert_resource(state);world.init_resource::<GameTime>();
        world.init_resource::<Events<StartupRevealStarted>>();world.init_resource::<Events<StartupRevealFinished>>();
        for _ in 0..3 {world.resource_mut::<Events<StartupRevealStarted>>().write(StartupRevealStarted);}
        let mut schedule=Schedule::default();schedule.add_systems(loading_audio::<RecordingStudio>);
        schedule.run(&mut world);
        assert_eq!(count(&log,"create:event:/UI/Loading"),1);
        assert_eq!(count(&log,"start:event:/UI/Loading"),1);
        assert_eq!(count(&log,"release:event:/UI/Loading"),1);
        schedule.run(&mut world);
        assert_eq!(count(&log,"create:event:/UI/Loading"),1,"read cursor drains old starts");
        for _ in 0..2 {world.resource_mut::<Events<StartupRevealFinished>>().write(StartupRevealFinished);}
        schedule.run(&mut world);
        assert!(world.resource::<GameLoadingAudioState<Event>>().snapshot.is_none());
        assert_eq!(count(&log,"stop:snapshot:/Loading"),1);
        assert_eq!(count(&log,"release:snapshot:/Loading"),1);
        world.resource_scope(|world,mut state:Mut<GameLoadingAudioState<Event>>| {
            state.arm_after_unload(world.resource::<AudioCore<RecordingStudio>>());
        });
        assert_eq!(count(&log,"create:snapshot:/Loading"),2);
        world.resource_mut::<GameTime>().delta=1.0;
        schedule.run(&mut world);
        assert!(world.resource::<GameLoadingAudioState<Event>>().snapshot.is_some(),"strict >1 guard");
        world.resource_mut::<GameTime>().delta=0.125;
        schedule.run(&mut world);
        let state=world.resource::<GameLoadingAudioState<Event>>();
        assert!(state.snapshot.is_none());assert!(!state.loading);assert_eq!(state.timer,1.125);
        assert_eq!(count(&log,"stop:snapshot:/Loading"),2);
        assert_eq!(count(&log,"release:snapshot:/Loading"),2);
        assert_eq!(count(&log,"create:event:/UI/Loading"),1,"timer must never synthesize reveal events");
    }
}
