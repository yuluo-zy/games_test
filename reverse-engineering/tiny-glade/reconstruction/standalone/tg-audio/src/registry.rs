//! `add_audio_events` (0x140941450, A §1a) and `add_audio_systems` (0x1409411a0, A §1b).
use crate::events::*;
use bevy_app::App;

/// The 16 registered audio events, in `register_event` call order.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AudioEventType {
    PlayTerrainSound,
    SuppressPathSound,
    PlayReplayAudio,
    PlayDecoratorSoundCmd,
    PlayStairsAudioCmd,
    PlayWallUiSound,
    PlayWallAudio,
    RequestWallMaterialAudio,
    WallBendAudioParamReset,
    PlayTimberframeSound,
    PlaySheepReactSound,
    PlayMenuUiSound,
    PlayBuildingAudio,
    PlayElevationSupportSoundCmd,
    PlayClutterSoundCmd,
    PlayTreeSoundCmd,
}

/// (type, register_event VA) in documented order (A §1a table rows 1..16).
pub const AUDIO_EVENT_ORDER: [(AudioEventType, u64); 16] = [
    (AudioEventType::PlayTerrainSound, 0x141514c40),
    (AudioEventType::SuppressPathSound, 0x141512d50),
    (AudioEventType::PlayReplayAudio, 0x141508c70),
    (AudioEventType::PlayDecoratorSoundCmd, 0x14151c800),
    (AudioEventType::PlayStairsAudioCmd, 0x141521ea0),
    (AudioEventType::PlayWallUiSound, 0x141520c10),
    (AudioEventType::PlayWallAudio, 0x14150dce0),
    (AudioEventType::RequestWallMaterialAudio, 0x14151d460),
    (AudioEventType::WallBendAudioParamReset, 0x14151b570),
    (AudioEventType::PlayTimberframeSound, 0x141509f00),
    (AudioEventType::PlaySheepReactSound, 0x141515ed0),
    (AudioEventType::PlayMenuUiSound, 0x14150d6b0),
    (AudioEventType::PlayBuildingAudio, 0x141510830),
    (AudioEventType::PlayElevationSupportSoundCmd, 0x14150b7c0),
    (AudioEventType::PlayClutterSoundCmd, 0x141512720),
    (AudioEventType::PlayTreeSoundCmd, 0x1415205e0),
];

/// `add_audio_events(app)`: one `App::add_event` per row of [`AUDIO_EVENT_ORDER`], in order.
/// `add_event` is idempotent (it skips an already-present `Events<T>`), matching the
/// component-table check in each block of the original (A §1a).
pub fn add_audio_events(app: &mut App) {
    app.add_event::<PlayTerrainSound>()
        .add_event::<SuppressPathSound>()
        .add_event::<PlayReplayAudio>()
        .add_event::<PlayDecoratorSoundCmd>()
        .add_event::<PlayStairsAudioCmd>()
        .add_event::<PlayWallUiSound>()
        .add_event::<PlayWallAudio>()
        .add_event::<RequestWallMaterialAudio>()
        .add_event::<WallBendAudioParamReset>()
        .add_event::<PlayTimberframeSound>()
        .add_event::<PlaySheepReactSound>()
        .add_event::<PlayMenuUiSound>()
        .add_event::<PlayBuildingAudio>()
        .add_event::<PlayElevationSupportSoundCmd>()
        .add_event::<PlayClutterSoundCmd>()
        .add_event::<PlayTreeSoundCmd>();
}

/// Systems `add_audio_systems` adds (A §1b). Only these are evidenced; the other
/// audio systems' registration point is UNKNOWN(evidence: A §1b/§9).
pub const ADD_AUDIO_SYSTEMS: [&str; 3] =
    ["music_debug_gui", "water_audio_system", "water_ambience_audio_system"];

#[cfg(test)]
mod tests {
    use super::*;
    use bevy_ecs::event::Events;

    /// Component ids of `Events<T>` in documented row order; ids are allocated on first
    /// registration, so strictly ascending ids == registered in that order.
    fn ids(app: &App) -> Vec<usize> {
        macro_rules! id {
            ($($t:ty),*) => {vec![$(app.world().resource_id::<Events<$t>>().expect(stringify!($t)).index()),*]};
        }
        id!(
            PlayTerrainSound, SuppressPathSound, PlayReplayAudio, PlayDecoratorSoundCmd, PlayStairsAudioCmd,
            PlayWallUiSound, PlayWallAudio, RequestWallMaterialAudio, WallBendAudioParamReset,
            PlayTimberframeSound, PlaySheepReactSound, PlayMenuUiSound, PlayBuildingAudio,
            PlayElevationSupportSoundCmd, PlayClutterSoundCmd, PlayTreeSoundCmd
        )
    }

    #[test]
    fn sixteen_events_in_documented_order() {
        let mut app = App::new();
        add_audio_events(&mut app);
        let ids = ids(&app);
        assert_eq!(ids.len(), 16);
        assert_eq!(AUDIO_EVENT_ORDER.len(), 16);
        assert!(ids.windows(2).all(|w| w[0] < w[1]), "registration order differs: {ids:?}");
        assert_eq!(AUDIO_EVENT_ORDER[0].0, AudioEventType::PlayTerrainSound);
        assert_eq!(AUDIO_EVENT_ORDER[15].0, AudioEventType::PlayTreeSoundCmd);
    }

    #[test]
    fn registration_is_idempotent() {
        let mut app = App::new();
        add_audio_events(&mut app);
        let first = ids(&app);
        add_audio_events(&mut app);
        assert_eq!(ids(&app), first);
    }

    #[test]
    fn va_table_matches_doc_spot_rows() {
        assert_eq!(AUDIO_EVENT_ORDER[6], (AudioEventType::PlayWallAudio, 0x14150dce0));
        assert_eq!(AUDIO_EVENT_ORDER[12].1, 0x141510830);
    }

    #[test]
    fn systems_listed() {
        assert_eq!(ADD_AUDIO_SYSTEMS.len(), 3);
    }
}
