#[cfg(test)]
mod original_loop_initialization {
    use super::*;
    use std::sync::{Arc, Mutex};

    #[derive(Default)]
    struct Trace {
        creates: Vec<String>,
        probes: Vec<(usize, String)>,
        releases: Vec<usize>,
        fail_at: Option<usize>,
        probe: Option<Result<bool, &'static str>>,
    }
    struct RecordedEvent { index: usize, trace: Arc<Mutex<Trace>> }
    impl EventInstance for RecordedEvent {
        type Error = &'static str;
        fn start(&mut self) -> Result<(), Self::Error> { Ok(()) }
        fn stop(&mut self, _: i32) -> Result<(), Self::Error> { Ok(()) }
        fn get_playback_state(&self) -> Result<PlaybackState, Self::Error> { Ok(PlaybackState::Stopped) }
        fn set_volume(&mut self, _: f32) -> Result<(), Self::Error> { Ok(()) }
        fn set_parameter_by_name(&mut self, _: &str, _: f32) -> Result<(), Self::Error> { Ok(()) }
        fn has_parameter(&self, name: &str) -> Result<bool, Self::Error> {
            let mut trace = self.trace.lock().unwrap();
            trace.probes.push((self.index, name.to_owned()));
            trace.probe.unwrap_or(Ok(true))
        }
        fn set_3d_attributes(&mut self, _: [f32;3], _: [f32;3], _: [f32;3], _: [f32;3]) -> Result<(), Self::Error> { Ok(()) }
        fn get_timeline_position(&self) -> Result<i32, Self::Error> { Ok(0) }
        fn pause(&mut self, _: bool) -> Result<(), Self::Error> { Ok(()) }
        fn install_beat_callback(&mut self) -> Result<(), Self::Error> { Ok(()) }
        fn release(&mut self) -> Result<(), Self::Error> {
            self.trace.lock().unwrap().releases.push(self.index); Ok(())
        }
    }
    struct RecordedStudio(Arc<Mutex<Trace>>);
    struct RecordedVca;
    impl VcaHandle for RecordedVca {
        type Error = &'static str;
        fn set_volume(&self, _: f32) -> Result<(), Self::Error> { Ok(()) }
    }
    impl Studio for RecordedStudio {
        type Error = &'static str;
        type Event = RecordedEvent;
        type Vca = RecordedVca;
        fn load_bank_file(&mut self, _: &str) -> Result<(), Self::Error> { Ok(()) }
        fn load_sample_data(&mut self) -> Result<(), Self::Error> { Ok(()) }
        fn get_vca(&self, _: &str) -> Result<Self::Vca, Self::Error> { Ok(RecordedVca) }
        fn set_listener_attributes(&mut self, _: ListenerAttributes) -> Result<(), Self::Error> { Ok(()) }
        fn update(&mut self) -> Result<(), Self::Error> { Ok(()) }
        fn set_parameter_by_name(&self, _: &str, _: f32) -> Result<(), Self::Error> { Ok(()) }
        fn create_event_instance(&self, name: &str) -> Result<Option<Self::Event>, Self::Error> {
            let mut trace = self.0.lock().unwrap();
            let index = trace.creates.len();
            trace.creates.push(name.to_owned());
            if trace.fail_at == Some(index) { return Err("creation failure"); }
            Ok(Some(RecordedEvent { index, trace: self.0.clone() }))
        }
    }
    fn setup(trace: Trace) -> (AudioCore<RecordedStudio>, Arc<Mutex<Trace>>) {
        let trace = Arc::new(Mutex::new(trace));
        (AudioCore { studio: Some(RecordedStudio(trace.clone())), banks: HashMap::new() }, trace)
    }

    #[test]
    fn constructor_owns_four_events_probes_only_the_first_and_releases_every_event() {
        for probe in [Ok(true), Ok(false), Err("parameter lookup failure")] {
            let (core, trace) = setup(Trace { probe: Some(probe), ..Default::default() });
            let player = core.instantiate_loop_player(&AudioLoopDesc::new("event:/test"));
            assert_eq!(player.audio.entries.len(), 4);
            assert_eq!(player.audio.threshold_ms, 50);
            assert_eq!(player.audio.uses_loop_parameter, probe == Ok(true));
            assert!(!player.audio.stops_explicitly);
            assert_eq!(trace.lock().unwrap().probes, [(0, "loop".to_owned())]);
            drop(player);
            assert_eq!(trace.lock().unwrap().releases, [0,1,2,3]);
        }
    }
    #[test]
    fn every_creation_failure_releases_the_successful_prefix_without_probing() {
        for fail_at in 0..4 {
            let (core, trace) = setup(Trace { fail_at: Some(fail_at), ..Default::default() });
            assert!(core.try_instantiate_loop(&AudioLoopDesc::new("event:/test")).is_err());
            let trace = trace.lock().unwrap();
            assert_eq!(trace.creates.len(), fail_at + 1);
            assert_eq!(trace.releases, (0..fail_at).collect::<Vec<_>>());
            assert!(trace.probes.is_empty());
        }
    }
    #[test]
    fn real_startup_owners_preserve_original_event_names_ring_size_and_noloop() {
        use crate::systems::audio::{path_audio, sheep_audio, terrain_stairs_audio, decorator_audio};
        let (core, trace) = setup(Trace::default());
        let paths = path_audio::PathAudioState::new(&core);
        for ring in &paths.loops {
            assert_eq!(ring.entries.len(), 4);
            assert_eq!(ring.threshold_ms, 50);
            assert!(ring.uses_loop_parameter && ring.stops_explicitly);
        }
        let sheep = sheep_audio::SheepAudioState::new(&core);
        let stairs = terrain_stairs_audio::TerrainStairsAudioState::new(&core);
        let decorators = decorator_audio::DecoratorAudioState::new(&core);
        let trace = trace.lock().unwrap();
        assert_eq!(trace.creates.len(), (5+1+1+16)*4);
        assert_eq!(&trace.creates[20..24], ["event:/SFX/Sheep/Pet";4]);
        assert_eq!(&trace.creates[24..28], ["event:/SFX/StairDraw_LoopTrem";4]);
        assert_eq!(&trace.creates[88..92], ["event:/SFX/Cosmetics/MoveBuilding";4]);
        drop(trace);
        assert!(!sheep.loop4.stops_explicitly);
        assert!(!stairs.ring.stops_explicitly);
        assert!(!decorators.loops[15].stops_explicitly);
    }
}
