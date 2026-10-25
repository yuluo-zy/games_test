"""Prepare reviewable Rust candidates without touching any production source."""
from pathlib import Path
import difflib, hashlib, json, subprocess

ROOT=Path(__file__).resolve().parent
WORK=Path('D:/game')
OUT=ROOT/'lifecycle-candidate'
changes={}
def edit(relative, transform):
    original=(WORK/relative).read_text(encoding='utf8')
    updated=transform(original)
    assert updated!=original, relative
    changes[relative]=(original,updated)

edit('tiny-glade/crates/country-core/src/asset_libraries/placement_data_library.rs',
    lambda s:s.replace('    /// 加载并注册摆放数据，同一路径不能重复加载。', '''    /// B uninit_clearing@140a7a5fd..60c drops buffers before path keys.
    /// Retain table allocations, and let each typed buffer release its real lease.
    pub fn clear(&mut self) {
        self.buffers.clear();
        self.paths.clear();
    }

    /// 加载并注册摆放数据，同一路径不能重复加载。'''))

edit('tiny-glade/crates/country-core/src/resources/tree_storage.rs',
    lambda s:s.replace('    pub records: Vec<InteractableForestTree>,\n}', '''    pub records: Vec<InteractableForestTree>,
    /// Native resource +0x18: false for the first processing pass of a glade.
    /// This belongs to the placement owner, never a system-local cache.
    pub processed: bool,
}

impl InteractableForestPlacements {
    /// B uninit_clearing@140a7a718..774: drop all records, retain allocation,
    /// and clear the initial-processing flag. Record payload recovery remains
    /// separate; this restores the observed lifecycle of their existing owner.
    pub fn clear(&mut self) {
        self.records.clear();
        self.processed = false;
    }
}'''))

def forest(s):
    s=s.replace('    pub finished: u8,\n','')
    s=s.replace('let first_frame = cache.finished == 0;', 'let first_frame = !placements.processed;')
    s=s.replace('    cache.finished = 1;', '    placements.processed = true;')
    s=s.replace('以 `MemCache.finished`（Local）承载语义等价簿记（分歧 U，见模块头）。',
        '由 canonical `InteractableForestPlacements::processed` 承载，会话卸载复位。')
    s=s.replace('    /// 帧内完成标记（I：真实 init 标志在放置资源 `+0x18`，当前 country_core 无此字段——\n    /// 以 Local 承载等价簿记，分歧见模块头）。\n','')
    return s.replace('placements +0x18；此处 = cache.finished 代理', 'placements +0x18；canonical processed')
edit('tiny-glade/crates/systems/forest/src/manage_tree_prefabs.rs',forest)

def water_core(s):
    start=s.index('use crate::{')
    s='''//! Original water-audio owner: 10x10 fixed cell bases, 64-frame visibility
//! bits and wrapping i16 jitter sums. Native volume is based on popcount/64,
//! with RNE coefficient42.25; Active belongs to AudioInstance, not the cell.
''' + s[start:]
    s=s.replace('pub const WATER_GRID_EXTENT: usize = 8;', 'pub const WATER_GRID_EXTENT: usize = 10;')
    s=s.replace('pub const WATER_VOLUME_ATTENUATION: f32 = 42.5;', 'pub const WATER_VOLUME_ATTENUATION: f32 = 42.25;')
    start=s.index('/// A water-grid cell')
    end=s.index('/// WaterAmbienceAudio:',start)
    s=s[:start]+'''/// B native 24B cell: base x/z at0, 64-frame visibility bits at8,
/// two wrapping signed jitter sums at0x10; the last four bytes are padding.
#[repr(C)]
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct WaterGridCell {
    pub position: [f32; 2],
    pub bits: u64,
    pub sums: [i16; 2],
}

impl WaterGridCell {
    /// Native producer141baf4b3..50c: +/- the current jitter only on bit changes.
    pub fn set_sample(&mut self, bit: u64, visible: bool, delta: [i16; 2]) {
        let was_set = self.bits & bit != 0;
        if visible {
            self.bits |= bit;
            if !was_set {
                self.sums[0] = self.sums[0].wrapping_add(delta[0]);
                self.sums[1] = self.sums[1].wrapping_add(delta[1]);
            }
        } else {
            self.bits &= !bit;
            if was_set {
                self.sums[0] = self.sums[0].wrapping_sub(delta[0]);
                self.sums[1] = self.sums[1].wrapping_sub(delta[1]);
            }
        }
    }

    pub fn active_position(&self) -> [f32; 3] {
        let reciprocal_count = 1.0 / self.bits.count_ones() as f32;
        [self.position[0] + reciprocal_count * self.sums[0] as f32 * 0.8125,
            0.0,
            self.position[1] + reciprocal_count * self.sums[1] as f32 * 0.8125]
    }
}

#[derive(Resource, Debug, Default)]
pub struct WaterAudioGrid {
    pub cells: Vec<WaterGridCell>,
}

impl WaterAudioGrid {
    /// Original init1409ebf20: 10x10 cells centred in [-65,65].
    pub fn new() -> Self {
        let mut cells = Vec::with_capacity(100);
        for z in 0..WATER_GRID_EXTENT {
            let tz = (z as f32 + 0.5) / 10.0;
            for x in 0..WATER_GRID_EXTENT {
                let tx = (x as f32 + 0.5) / 10.0;
                cells.push(WaterGridCell {
                    position: [tx * 65.0 - (1.0-tx) * 65.0,
                        tz * 65.0 - (1.0-tz) * 65.0],
                    bits: 0, sums: [0; 2],
                });
            }
        }
        Self { cells }
    }

    /// B uninit_clearing140a7a7b0..7c8: retain base positions and allocation.
    pub fn clear_activity(&mut self) {
        for cell in &mut self.cells {
            cell.bits = 0;
            cell.sums = [0; 2];
        }
    }

    /// B1409ebe80 returns visibility density, not a reference to the cell.
    pub fn load_index_unchecked(&self, index: usize) -> f32 {
        self.cells[index].bits.count_ones() as f32 * WATER_VOLUME_BIT_WEIGHT
    }
}

''' + s[end:]
    s=s.replace('    let mut grid = WaterAudioGrid::default();\n    grid.cells = vec![WaterGridCell::default(); cells];', '    let grid = WaterAudioGrid::new();')
    s=s.replace('let cell = grid.load_index_unchecked(index);','let cell = &grid.cells[index];')
    start=s.index('fn water_ambience_volume(')
    end=s.index('/// `water_ambience_audio_system`',start)
    s=s[:start]+'''fn water_ambience_volume(pop: f32, dist_sq: f32) -> f32 {
    // Exact mul/div/add/reciprocal/mul/sqrt sequence1409ed2b8..36d.
    let fraction=pop*WATER_VOLUME_BIT_WEIGHT;
    let attenuation=fraction*WATER_VOLUME_ATTENUATION;
    (fraction*(WATER_VOLUME_ONE/(dist_sq/attenuation+WATER_VOLUME_ONE))).sqrt()
}

''' + s[end:]
    start=s.index('            // O: resolved cell world position')
    end=s.index('        } else {\n            let _ = loop_player.stop_now();',start)
    s=s[:start]+'''            let position = cell.active_position();
            let _ = current.set_position(position);
            let dx = position[0] - eye[0];
            let dy = position[1] - eye[1];
            let dz = position[2] - eye[2];
            let dist_sq = dx * dx + dy * dy + dz * dz;
            let volume = water_ambience_volume(cell.bits.count_ones() as f32, dist_sq);
            // Native ==1 guard is the selected AudioInstance::Active tag,
            // not a WaterGridCell state. AudioInstance::set_volume owns it.
            current.set_volume(volume).unwrap();
''' + s[end:]
    s=s.replace('let _ = loop_player.start_if_not_playing();', 'loop_player.start_if_not_playing().unwrap();')
    s=s.replace('let _ = loop_player.stop_now();', 'loop_player.stop_now().unwrap();')
    s=s.replace('let _ = current.set_position(position);', 'current.set_position(position).unwrap();')
    s=s.replace('(water_ambience_volume(1.0, 0.0) - 1.0)', '(water_ambience_volume(1.0, 0.0) - 0.125)')
    s=s.replace('water_ambience_volume(2.0, 0.0) - 2.0f32.sqrt()', 'water_ambience_volume(2.0, 0.0) - (2.0f32/64.0).sqrt()')
    s=s.replace('42.5','42.25').replace('Grid extent (native grid dimensions 待取证; the constant mirrors the\n/// documented 64-bit rolling-window cell accounting).','Native init1409ebf20 constructs10 rows with10 columns.')
    return s+'''

#[cfg(test)]
mod lifecycle_native_tests {
    use super::*;
    #[test]
    fn original_water_grid_owns_rollover_and_unload_preserves_positions() {
        let mut grid=WaterAudioGrid::new();
        assert_eq!(grid.cells.len(),100);
        assert_eq!(core::mem::size_of::<WaterGridCell>(),24);
        assert_eq!(core::mem::offset_of!(WaterGridCell,sums),0x10);
        assert_eq!(grid.cells[0].position,[-58.5,-58.5]);
        assert_eq!(grid.cells[99].position,[58.5,58.5]);
        let base=grid.cells.iter().map(|c|c.position).collect::<Vec<_>>();
        let address=grid.cells.as_ptr();
        grid.cells[0].set_sample(1<<11,true,[-1,-5]);
        grid.cells[0].set_sample(1<<11,true,[-1,-5]);
        assert_eq!(grid.cells[0].sums,[-1,-5]);
        assert_eq!(grid.cells[0].active_position(),[-59.3125,0.0,-62.5625]);
        assert_eq!(grid.load_index_unchecked(0),1.0/64.0);
        grid.clear_activity();
        assert_eq!(grid.cells.as_ptr(),address);
        assert_eq!(grid.cells.iter().map(|c|c.position).collect::<Vec<_>>(),base);
        assert!(grid.cells.iter().all(|c|c.bits==0 && c.sums==[0,0]));
        // The still-live producer's next frame uses the cleared canonical owner.
        grid.cells[0].set_sample(1<<22,true,[5,-3]);
        assert_eq!(grid.cells[0].bits,1<<22);
        assert_eq!(grid.cells[0].sums,[5,-3]);
        grid.cells[0].set_sample(1<<22,false,[5,-3]);
        assert_eq!(grid.cells[0].sums,[0,0]);
    }
    #[test]
    fn native_attenuation_constant_and_normalized_volume_are_retained() {
        assert_eq!(WATER_VOLUME_ATTENUATION.to_bits(),0x42290000);
        assert_eq!(water_ambience_volume(64.0,42.25).to_bits(),0.5f32.sqrt().to_bits());
    }
}
'''
edit('tiny-glade/crates/country-core/src/systems/audio/water_ambience_audio.rs',water_core)

def water_producer(s):
    s=s.replace('pub const SAMPLE_WINDOW: f32 = 260.0;', 'pub const SAMPLE_WINDOW: f32 = 130.0;')
    start=s.index('pub struct WaterAudioGridUpdateState {')
    end=s.index('/// `update_water_audio_grid`', start)
    s=s[:start]+'''#[derive(Default)]
pub struct WaterAudioGridUpdateState {
    /// Only this frame counter is Local in native141baefe0; rolling state is
    /// owned by WaterAudioGrid and reset on level unload.
    pub frame: u64,
}

''' + s[end:]
    s=s.replace('pub const WATER_GRID_EXTENT: usize = 8;', 'pub const WATER_GRID_EXTENT: usize = 10;')
    s=s.replace('for (index, cell) in grid.cells.iter_mut().enumerate() {','for cell in &mut grid.cells {')
    s=s.replace('        let coord = cell_coord(index);\n        let base = Vec2::new(coord.x * GRID_CELL_SPACING, coord.y * GRID_CELL_SPACING);',
        '        let base = Vec2::from_array(cell.position);')
    start=s.index('        let was_set = state.windows[index] & bit != 0;')
    end=s.index('\n    }\n}',start)
    s=s[:start]+'''        cell.set_sample(bit, visible, [delta_x, delta_z]);''' + s[end:]
    return s
edit('tiny-glade/crates/systems/water/src/water_audio_grid.rs',water_producer)

audio='''//! Original loading snapshot lifecycle: empty is Option::None, Disabled is
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
'''
audio+='''

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
'''
edit('tiny-glade/crates/country-core/src/systems/audio/game_loading_audio.rs',lambda _:audio)

unload='''

/// First OnExit group, original closure140b3c930: enqueue false before closing
/// the actual save channel/file. Do not replace this with resource defaults.
pub fn begin_glade_unload(
    mut clear: bevy_ecs::prelude::EventWriter<crate::systems::mode_manager::ClearCanvasCmd>,
    mut autosave: bevy_ecs::prelude::ResMut<crate::systems::save::save_game::AutoSaveState>,
) {
    clear.write(crate::systems::mode_manager::ClearCanvasCmd(false));
    autosave.close_file();
}

/// Original eight-owner uninit140a7a5b0. AudioCore is read-only; every other
/// owner receives precisely its observed clear/flag policy.
#[allow(clippy::too_many_arguments)]
pub fn uninit_clearing<S: crate::resources::audio_resources::Studio+Send+Sync+'static>(
    mut placement: bevy_ecs::prelude::ResMut<crate::asset_libraries::placement_data_library::AssetPlacementDataLibrary>,
    mut triggers: bevy_ecs::prelude::ResMut<crate::asset_libraries::audio_trigger_library::AudioTriggerLibrary>,
    mut rt: bevy_ecs::prelude::ResMut<crate::resources::rt::RtWorld>,
    mut gpu: bevy_ecs::prelude::ResMut<crate::render::nani::gpu_world::GpuWorld>,
    mut water: Option<bevy_ecs::prelude::ResMut<crate::systems::audio::water_ambience_audio::WaterAudioGrid>>,
    mut forest: bevy_ecs::prelude::ResMut<crate::resources::tree_storage::InteractableForestPlacements>,
    audio: bevy_ecs::prelude::Res<crate::resources::audio_resources::AudioCore<S>>,
    mut loading: bevy_ecs::prelude::ResMut<crate::systems::audio::game_loading_audio::GameLoadingAudioState<S::Event>>,
) {
    placement.clear();
    triggers.clear();
    rt.clear_meshes();
    gpu.clear_owned_instances();
    forest.clear();
    if let Some(water)=water.as_deref_mut() {
        water.clear_activity();
    }
    loading.arm_after_unload(&audio);
}

/// Native1408b3510 query filter is established by140a04980/140b3f550,
/// rather than the standalone void() placeholder. Commands owns deferred work.
pub fn despawn_glade_entities(
    entities: bevy_ecs::prelude::Query<bevy_ecs::entity::Entity,
        bevy_ecs::prelude::Without<crate::components::KeepOnLevelUnload>>,
    mut commands: bevy_ecs::prelude::Commands,
) {
    for entity in &entities {
        commands.entity(entity).despawn();
    }
}
'''
edit('tiny-glade/crates/country-core/src/startup/clearing.rs',lambda s:s+unload)

def register(s):
    anchor='    app.add_systems(bevy_state::state::OnExit(country_core::app_loading_state::AppLoadingState::Game),crate::app_systems::walls_on_glade_unload);'
    assert anchor in s
    return s.replace(anchor,anchor+'''
    // Native140b3f550 outer chain: begin precedes the inner unordered pair.
    // Keep the independently registered full host wall-unload system above.
    app.add_systems(
        bevy_state::state::OnExit(country_core::app_loading_state::AppLoadingState::Game),
        (
            country_core::startup::clearing::begin_glade_unload,
            (
                country_core::startup::clearing::uninit_clearing::<country_core::resources::native_audio::NativeStudio>,
                country_core::startup::clearing::despawn_glade_entities,
            ),
        ).chain(),
    );''')
edit('tiny-glade/crates/tiny-glade/src/run_game.rs',register)

manifest={}
for relative,(before,after) in changes.items():
    path=OUT/relative
    path.parent.mkdir(parents=True,exist_ok=True)
    path.write_text(after,encoding='utf8')
    # Parse the complete candidate but keep unrelated source formatting intact.
    subprocess.run(['C:/Users/liyu/.rustup/toolchains/stable-x86_64-pc-windows-msvc/bin/rustfmt.exe','--edition','2024','--config','skip_children=true','--emit','stdout',str(path)],check=True,stdout=subprocess.DEVNULL)
    updated=after
    changes[relative]=(before,updated)
    manifest[relative]={'base_sha256':hashlib.sha256((WORK/relative).read_bytes()).hexdigest(),
        'candidate_sha256':hashlib.sha256(path.read_bytes()).hexdigest()}
patch=''.join(''.join(difflib.unified_diff(before.splitlines(True),after.splitlines(True),
    fromfile='a/'+relative,tofile='b/'+relative)) for relative,(before,after) in changes.items())
(ROOT/'startup-unload-owner-candidate.patch').write_text(patch,encoding='utf8')
(ROOT/'startup-unload-owner-candidate-manifest.json').write_text(json.dumps(manifest,indent=2),encoding='utf8')
print(json.dumps({'production_files_changed':0,'candidate_file_count':len(changes),'patch':'startup-unload-owner-candidate.patch'},indent=2))
