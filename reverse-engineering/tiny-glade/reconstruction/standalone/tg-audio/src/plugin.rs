//! `AudioPlugin`: `add_audio_events` (A §1a) + the audio systems.
//!
//! System-set order inside each schedule is UNKNOWN(evidence: A §1b): the doc only shows which
//! systems exist, so the sets are chained in the documented module order.
use crate::building::BuildingAudioPlugin;
use crate::clutter::ClutterAudioPlugin;
use crate::menu::MenuAudioPlugin;
use crate::music::MusicAudioPlugin;
use crate::registry::add_audio_events;
use crate::sheep::SheepAudioPlugin;
use crate::stairs::StairsAudioPlugin;
use crate::update::AudioUpdatePlugin;
use crate::wall_system::WallAudioPlugin;
use bevy_app::{App, Plugin, Startup, Update};
use bevy_ecs::prelude::*;

/// Startup systems.
#[derive(SystemSet, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum AudioInit {
    BootUpdate,
    Wall,
    Building,
    Stairs,
    Menu,
    Music,
}

/// Per-frame systems.
#[derive(SystemSet, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum AudioFrame {
    Wall,
    Building,
    Stairs,
    Menu,
    Sheep,
    Clutter,
    Music,
    Update,
}

/// Registers the 16 audio events (documented order) and every audio system. The host must
/// insert an [`AudioBackendRes`](crate::backend::AudioBackendRes); `AudioSettings`,
/// `ListenerPose` and `MusicInputs` are host inputs whose systems stay idle until present.
pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        add_audio_events(app);
        app.configure_sets(
            Startup,
            (
                AudioInit::BootUpdate,
                AudioInit::Wall,
                AudioInit::Building,
                AudioInit::Stairs,
                AudioInit::Menu,
                AudioInit::Music,
            )
                .chain(),
        )
        .configure_sets(
            Update,
            (
                AudioFrame::Wall,
                AudioFrame::Building,
                AudioFrame::Stairs,
                AudioFrame::Menu,
                AudioFrame::Sheep,
                AudioFrame::Clutter,
                AudioFrame::Music,
                AudioFrame::Update,
            )
                .chain(),
        )
        .add_plugins((
            WallAudioPlugin,
            BuildingAudioPlugin,
            StairsAudioPlugin,
            MenuAudioPlugin,
            SheepAudioPlugin,
            ClutterAudioPlugin,
            MusicAudioPlugin,
            AudioUpdatePlugin,
        ));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::backend::Call;
    use crate::building::{BuildingEntry, BuildingTable};
    use crate::events::*;
    use crate::test_util::{app_with, build, frame};
    use glam::Vec3;

    #[test]
    fn new_registers_and_inits() {
        let (mut app, rec) = build(AudioPlugin);
        app.update();
        let loops = rec.calls().iter().filter(|c| matches!(c, Call::InstantiateLoop { .. })).count();
        assert_eq!(loops, 10 + 7 + 6);
        let names: Vec<_> = rec
            .calls()
            .iter()
            .filter_map(|c| if let Call::InstantiateAudio { name, .. } = c { Some(name.clone()) } else { None })
            .collect();
        assert_eq!(names, ["snapshot:/PauseMenu", "event:/Music/mus_GladePreview", "event:/Amb/GladePreview"]);
    }

    #[test]
    fn update_routes_and_drains() {
        let (mut app, rec) = app_with(AudioPlugin);
        app.world_mut().resource_mut::<BuildingTable>().0.insert(7, BuildingEntry { size_param: 0.25 });
        let w = app.world_mut();
        w.send_event(PlayWallAudio { kind: 21, volume: 1.0, ..Default::default() });
        w.send_event(PlaySheepReactSound { kind: 0 });
        w.send_event(PlayMenuUiSound { kind: 0 });
        w.send_event(PlayStairsAudioCmd { kind: 3, position: Vec3::ZERO });
        w.send_event(PlayBuildingAudio { kind: 6, ..Default::default() });
        w.send_event(PlayTreeSoundCmd::default());
        app.update();
        let names: Vec<_> = rec
            .take()
            .iter()
            .filter_map(|c| if let Call::InstantiateAudio { name, .. } = c { Some(name.clone()) } else { None })
            .collect();
        assert_eq!(
            names,
            ["event:/SFX/WallSplit", "event:/SFX/Shapes/DetachFromGround", "event:/SFX/Stairs/StairPlace",
             "event:/UI/Menu/Hover", "event:/SFX/Sheep/Aww"]
        );
        // drained: a second frame plays nothing new
        let calls = frame(&mut app, &rec, std::iter::empty::<PlayTreeSoundCmd>());
        assert!(!calls.iter().any(|c| matches!(c, Call::InstantiateAudio { .. })));
    }

    #[test]
    fn tree_event_has_no_consumer() {
        let (mut app, rec) = app_with(AudioPlugin);
        let with = frame(&mut app, &rec, [PlayTreeSoundCmd::default()]);
        let without = frame(&mut app, &rec, std::iter::empty::<PlayTreeSoundCmd>());
        assert_eq!(with, without);
    }

    #[test]
    fn plugin_registers_unconsumed_events() {
        let (mut app, _rec) = app_with(AudioPlugin);
        // the 9 events without a recovered consumer can still be sent
        let w = app.world_mut();
        assert!(w.send_event(PlayTerrainSound).is_some());
        assert!(w.send_event(PlayElevationSupportSoundCmd).is_some());
        app.update();
    }
}
