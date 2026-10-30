//! B `elevation_supports_audio.rs` 0x140ad51c0: elevated-shape supports
//! (wooden brackets / brick / wooden columns) one-shot placement sounds.
//!
//! 原跳表142ae1998：0不播放，1木托架，2砖柱，3木柱；位置原样传递。
//! 原调用140ad5282使用位置专用helper，不改动事件的默认音量。

use crate::{
    resources::audio_resources::{AudioCore, Studio},
    systems::audio::roof_audio::instantiate_audio_w_position,
};
use crate::systems::audio::PlayElevationSupportSoundCmd;
use bevy_ecs::prelude::*;

/// The three support one-shot events (O: rne-read, contiguous .data).
pub const SUPPORT_EVENTS: [&str; 3] = [
    "event:/SFX/Shapes/WoodenBracketsAppear",
    "event:/SFX/Shapes/BrickColumnAppear",
    "event:/SFX/Shapes/WoodenColumnAppear",
];

fn support_event(kind: u8) -> Option<&'static str> {
    match kind {
        0 => None,
        1..=3 => Some(SUPPORT_EVENTS[usize::from(kind - 1)]),
        // 原编译枚举要求有效判别值；安全接口明确拒绝非法字节，不模拟表外跳转。
        _ => panic!("invalid elevation support sound kind: {kind}"),
    }
}

/// 原0x140ad51c0的四种有效分派；仅非零类别实例化一次性声音。
pub fn elevation_supports_audio<S: Studio + Send + Sync + 'static>(
    mut events: EventReader<PlayElevationSupportSoundCmd>,
    core: Res<AudioCore<S>>,
) {
    for event in events.read() {
        let Some(name) = support_event(event.kind) else { continue };
        let position = [event.position.x, event.position.y, event.position.z];
        // O: `roof_audio::instantiate_audio_w_position` (0x140aa3360) which
        // instantiates, positions and starts the one-shot.
        let _ = instantiate_audio_w_position(&core, name, position);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::resources::{audio_resources::{EventInstance, ListenerAttributes, VcaHandle},
        audio_instance::PlaybackState};
    use std::sync::{Arc, Mutex};

    #[derive(Debug, PartialEq, Eq)]
    enum Call { Create(String), Position([u32; 3]), State, Start, Stop, Volume, Release }
    struct RecordedEvent(Arc<Mutex<Vec<Call>>>);
    impl EventInstance for RecordedEvent {
        type Error = &'static str;
        fn start(&mut self) -> Result<(), Self::Error> { self.0.lock().unwrap().push(Call::Start); Ok(()) }
        fn stop(&mut self, _: i32) -> Result<(), Self::Error> { self.0.lock().unwrap().push(Call::Stop); Ok(()) }
        fn get_playback_state(&self) -> Result<PlaybackState, Self::Error> { self.0.lock().unwrap().push(Call::State); Ok(PlaybackState::Stopped) }
        fn set_volume(&mut self, _: f32) -> Result<(), Self::Error> { self.0.lock().unwrap().push(Call::Volume); Err("不应覆盖事件默认音量") }
        fn set_parameter_by_name(&mut self, _: &str, _: f32) -> Result<(), Self::Error> { Ok(()) }
        fn has_parameter(&self, _: &str) -> Result<bool, Self::Error> { Ok(false) }
        fn set_3d_attributes(&mut self, position: [f32; 3], _: [f32; 3], _: [f32; 3], _: [f32; 3]) -> Result<(), Self::Error> {
            self.0.lock().unwrap().push(Call::Position(position.map(f32::to_bits))); Ok(())
        }
        fn get_timeline_position(&self) -> Result<i32, Self::Error> { Ok(0) }
        fn pause(&mut self, _: bool) -> Result<(), Self::Error> { Ok(()) }
        fn install_beat_callback(&mut self) -> Result<(), Self::Error> { Ok(()) }
        fn release(&mut self) -> Result<(), Self::Error> { self.0.lock().unwrap().push(Call::Release); Ok(()) }
    }
    struct RecordedStudio(Arc<Mutex<Vec<Call>>>);
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
            self.0.lock().unwrap().push(Call::Create(name.to_owned())); Ok(Some(RecordedEvent(self.0.clone())))
        }
    }

    #[test]
    fn real_ecs_dispatch_uses_position_only_helper_and_releases_each_one_shot_once() {
        let trace = Arc::new(Mutex::new(Vec::new()));
        let mut world = World::new();
        world.insert_resource(AudioCore { studio: Some(RecordedStudio(trace.clone())), banks: Default::default() });
        world.init_resource::<Events<PlayElevationSupportSoundCmd>>();
        let position = glam::Vec3::new(-0.0, f32::from_bits(0x7fc01234), 3.5);
        for kind in 0..=3 { world.send_event(PlayElevationSupportSoundCmd { kind, position }); }
        let mut schedule = Schedule::default();
        schedule.add_systems(elevation_supports_audio::<RecordedStudio>);
        schedule.run(&mut world);
        let mut expected = Vec::new();
        for name in SUPPORT_EVENTS {
            expected.extend([Call::Create(name.into()), Call::Position(position.to_array().map(f32::to_bits)),
                Call::State, Call::Start, Call::Release]);
        }
        assert_eq!(*trace.lock().unwrap(), expected);
        schedule.run(&mut world);
        assert_eq!(*trace.lock().unwrap(), expected, "原事件游标不重复播放已消费请求");
    }

    #[test]
    fn support_event_names_match_the_original_string_table() {
        assert_eq!(SUPPORT_EVENTS[0], "event:/SFX/Shapes/WoodenBracketsAppear");
        assert_eq!(SUPPORT_EVENTS[1], "event:/SFX/Shapes/BrickColumnAppear");
        assert_eq!(SUPPORT_EVENTS[2], "event:/SFX/Shapes/WoodenColumnAppear");
    }

    #[test]
    fn original_four_way_dispatch_preserves_silent_kind_and_rejects_invalid_enum() {
        assert_eq!(support_event(0), None);
        for kind in 1..=3 { assert_eq!(support_event(kind), Some(SUPPORT_EVENTS[usize::from(kind - 1)])); }
        for kind in [4, 255] { assert!(std::panic::catch_unwind(|| support_event(kind)).is_err()); }
    }
}
