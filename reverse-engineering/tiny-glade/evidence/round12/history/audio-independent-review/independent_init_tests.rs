#[cfg(test)] mod independent_init {
use crate::resources::{audio_resources::*,audio_instance::PlaybackState};
use std::{sync::{Arc,Mutex},collections::HashMap,time::Duration};
#[derive(Default)] struct Trace { creates:usize, fail:Option<usize>, probes:Vec<usize>, releases:Vec<usize> }
struct Event { id:usize, trace:Arc<Mutex<Trace>> }
impl EventInstance for Event {
type Error=&'static str;
fn start(&mut self)->Result<(),Self::Error>{Ok(())}
fn stop(&mut self,_:i32)->Result<(),Self::Error>{Ok(())}
fn get_playback_state(&self)->Result<PlaybackState,Self::Error>{Ok(PlaybackState::Stopped)}
fn set_volume(&mut self,_:f32)->Result<(),Self::Error>{Ok(())}
fn set_parameter_by_name(&mut self,_:&str,_:f32)->Result<(),Self::Error>{Ok(())}
fn has_parameter(&self,n:&str)->Result<bool,Self::Error>{assert_eq!(n,"loop");self.trace.lock().unwrap().probes.push(self.id);Ok(self.id%4==0)}
fn set_3d_attributes(&mut self,_:[f32;3],_:[f32;3],_:[f32;3],_:[f32;3])->Result<(),Self::Error>{Ok(())}
fn get_timeline_position(&self)->Result<i32,Self::Error>{Ok(0)}
fn pause(&mut self,_:bool)->Result<(),Self::Error>{Ok(())}
fn install_beat_callback(&mut self)->Result<(),Self::Error>{Ok(())}
fn release(&mut self)->Result<(),Self::Error>{self.trace.lock().unwrap().releases.push(self.id);Ok(())}
}
struct StudioOwner(Arc<Mutex<Trace>>);struct Vca;
impl VcaHandle for Vca{type Error=&'static str;fn set_volume(&self,_:f32)->Result<(),Self::Error>{Ok(())}}
impl Studio for StudioOwner {
type Error=&'static str;type Event=Event;type Vca=Vca;
fn load_bank_file(&mut self,_:&str)->Result<(),Self::Error>{Ok(())}
fn load_sample_data(&mut self)->Result<(),Self::Error>{Ok(())}
fn get_vca(&self,_:&str)->Result<Vca,Self::Error>{Ok(Vca)}
fn set_listener_attributes(&mut self,_:ListenerAttributes)->Result<(),Self::Error>{Ok(())}
fn update(&mut self)->Result<(),Self::Error>{Ok(())}
fn set_parameter_by_name(&self,_:&str,_:f32)->Result<(),Self::Error>{Ok(())}
fn create_event_instance(&self,_:&str)->Result<Option<Event>,Self::Error>{let mut t=self.0.lock().unwrap();let id=t.creates;t.creates+=1;if t.fail==Some(id){return Err("injected")};Ok(Some(Event{id,trace:self.0.clone()}))}
}
fn setup(fail:Option<usize>)->(AudioCore<StudioOwner>,Arc<Mutex<Trace>>){let trace=Arc::new(Mutex::new(Trace{fail,..Default::default()}));(AudioCore{studio:Some(StudioOwner(trace.clone())),banks:HashMap::new()},trace)}
#[test] fn independent_every_partial_path_owner_releases_all_completed_rings_and_current_prefix(){
for fail in 0..20{let(core,t)=setup(Some(fail));let result=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||crate::systems::audio::path_audio::PathAudioState::new(&core)));assert!(result.is_err());let t=t.lock().unwrap();assert_eq!(t.creates,fail+1);let mut released=t.releases.clone();released.sort_unstable();assert_eq!(released,(0..fail).collect::<Vec<_>>());assert_eq!(t.probes,(0..fail/4).map(|i|i*4).collect::<Vec<_>>());}}
#[test] fn independent_custom_delay_and_count_survive_player_and_strict_stop_boundary(){
let(core,t)=setup(None);let mut desc=AudioLoopDesc::new("event:/independent");desc.count=7;desc.threshold_ms=137;let mut player=core.instantiate_loop_player(&desc);assert_eq!(player.audio.entries.len(),8);assert_eq!(t.lock().unwrap().probes,[0]);assert_eq!(player.audio.threshold_ms,137);player.audio.start_if_not_playing().unwrap();player.audio.stop(Duration::from_millis(20)).unwrap();player.audio.stop(Duration::from_millis(157)).unwrap();assert_eq!(player.audio.cursor,0);player.audio.stop(Duration::from_millis(158)).unwrap();assert_eq!(player.audio.cursor,1);drop(player);assert_eq!(t.lock().unwrap().releases,(0..8).collect::<Vec<_>>());}
#[test] fn independent_wrapping_count_max_preserves_original_empty_first_panic(){let(core,t)=setup(None);let mut d=AudioLoopDesc::new("event:/independent");d.count=usize::MAX;assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(||core.try_instantiate_loop(&d))).is_err());assert_eq!(t.lock().unwrap().creates,0);}
}
