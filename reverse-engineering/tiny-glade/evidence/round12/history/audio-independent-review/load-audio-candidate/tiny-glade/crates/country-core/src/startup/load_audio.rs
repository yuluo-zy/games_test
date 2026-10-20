//! Original load_audio140b0e0f0: wait for samples, then queue resource commands.
//! The12/8-byte queue entries are commands, not Schedule system builders.
use crate::{
    asset_libraries::audio_trigger_library::AudioTriggerLibrary,
    resources::audio_resources::{AudioSamplesLoadedPromise, EventInstance},
    systems::audio::{game_loading_audio::GameLoadingAudioState, terrain_stairs_audio::StairsCounter},
};
use bevy_ecs::prelude::Commands;
use std::time::{Duration, Instant};

pub const WAITED_LOG_PREFIX: &str = "Waited ";

/// The actual caller supplies its canonical event owner. No audio event is
/// created here: init_game_loading_audio is a separate Startup producer.
pub fn load_audio<E: EventInstance + Send + Sync + 'static>(
    loaded_samples: &AudioSamplesLoadedPromise<()>,
    commands: &mut Commands,
) {
    let start = Instant::now();
    let _ = loaded_samples.recv();
    log::info!(target:"country_core::startup::load_audio", "{}", waited_log(start.elapsed()));
    commands.insert_resource(AudioTriggerLibrary::default());
    commands.insert_resource(StairsCounter::default());
    commands.insert_resource(GameLoadingAudioState::<E>::empty_after_samples());
    commands.remove_resource::<AudioSamplesLoadedPromise<()>>();
}

pub fn waited_log(elapsed: Duration) -> String {
    format!("{WAITED_LOG_PREFIX}{:?}", elapsed)
}

#[cfg(test)] mod tests {
    use super::*;
    use crate::resources::audio_instance::PlaybackState;
    use bevy_ecs::{prelude::World, world::CommandQueue};
    struct NeverEvent;
    impl EventInstance for NeverEvent {
        type Error = &'static str;
        fn start(&mut self)->Result<(),Self::Error>{panic!("load_audio must not create/start events")}
        fn stop(&mut self,_:i32)->Result<(),Self::Error>{unreachable!()}
        fn get_playback_state(&self)->Result<PlaybackState,Self::Error>{unreachable!()}
        fn set_volume(&mut self,_:f32)->Result<(),Self::Error>{unreachable!()}
        fn set_parameter_by_name(&mut self,_:&str,_:f32)->Result<(),Self::Error>{unreachable!()}
        fn has_parameter(&self,_:&str)->Result<bool,Self::Error>{unreachable!()}
        fn set_3d_attributes(&mut self,_:[f32;3],_:[f32;3],_:[f32;3],_:[f32;3])->Result<(),Self::Error>{unreachable!()}
        fn get_timeline_position(&self)->Result<i32,Self::Error>{unreachable!()}
        fn pause(&mut self,_:bool)->Result<(),Self::Error>{unreachable!()}
        fn install_beat_callback(&mut self)->Result<(),Self::Error>{unreachable!()}
    }
    #[test] fn actual_wait_publishes_four_deferred_resource_commands_without_starting_audio() {
        let mut world=World::new();let(sender,promise)=AudioSamplesLoadedPromise::channel();sender.send(()).unwrap();
        world.insert_resource(promise);world.insert_resource(StairsCounter{count:99});
        let mut state=GameLoadingAudioState::<NeverEvent>::empty_after_samples();state.loading=true;state.timer=99.;world.insert_resource(state);
        let mut queue=CommandQueue::default();
        load_audio::<NeverEvent>(world.resource(),&mut Commands::new(&mut queue,&world));
        assert_eq!(world.resource::<StairsCounter>().count,99);
        assert!(world.contains_resource::<AudioSamplesLoadedPromise<()>>());
        queue.apply(&mut world);
        assert_eq!(world.resource::<StairsCounter>().count,0);
        assert!(world.contains_resource::<AudioTriggerLibrary>());
        let state=world.resource::<GameLoadingAudioState<NeverEvent>>();assert!(state.snapshot.is_none());assert!(!state.loading);assert_eq!(state.timer.to_bits(),0);
        assert!(!world.contains_resource::<AudioSamplesLoadedPromise<()>>());
    }
    #[test] fn disconnected_samples_panic_before_any_resource_command_is_published() {
        let mut world=World::new();let(sender,promise)=AudioSamplesLoadedPromise::<()>::channel();drop(sender);world.insert_resource(promise);world.insert_resource(StairsCounter{count:17});
        let mut queue=CommandQueue::default();
        assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(||load_audio::<NeverEvent>(world.resource(),&mut Commands::new(&mut queue,&world)))).is_err());
        queue.apply(&mut world);assert_eq!(world.resource::<StairsCounter>().count,17);assert!(!world.contains_resource::<GameLoadingAudioState<NeverEvent>>());assert!(!world.contains_resource::<AudioTriggerLibrary>());
    }
}
