//! audio — typed-gap facades for the audio orchestration the wall systems drive.

use bevy_ecs::prelude::Resource;

use crate::typed_gap::Gap;

#[derive(Resource, Clone, Debug, Default)]
pub struct AudioCore {
    pub raw: Gap,
}

/// Wall material-audio request (O: 0x20-byte event row written by wall edits).
#[derive(Clone, Copy, Debug)]
pub struct RequestWallMaterialAudio {
    pub wall: crate::resources::walls::WallId,
    pub pos: glam::Vec3,
    pub raw: Gap,
}

/// Wall UI sound event (0x20-byte event row written by snapping/UI systems).
#[derive(Clone, Copy, Debug)]
pub struct PlayWallUiSound {
    pub raw: Gap,
}

#[derive(Clone, Copy, Debug)]
pub struct PlayBuildingAudio {
    pub raw: Gap,
}

#[derive(Clone, Copy, Debug)]
pub struct PlaySheepReactSound {
    pub raw: Gap,
}

#[derive(Clone, Copy, Debug)]
pub struct PlayStairsAudioCmd {
    pub raw: Gap,
}

#[derive(Clone, Copy, Debug)]
pub struct AudioInstance {
    pub raw: Gap,
}

pub fn get_building_size_01(_pos: glam::Vec3) -> f32 {
    0.0
}

// Audio subsystem states (typed-gap resources referenced by ui_* wall systems).
macro_rules! audio_state {
    ($name:ident) => {
        #[derive(Resource, Clone, Debug, Default)]
        pub struct $name {
            pub raw: Gap,
        }
    };
}
audio_state!(PathAudioState);
audio_state!(GardenAudioState);
audio_state!(TerrainAudioState);
audio_state!(SignifierAudioState);
audio_state!(ReplayAudioState);
audio_state!(TriggerAudioState);
audio_state!(RoofAudioState);
audio_state!(TerrainStairsAudioState);
audio_state!(StairsAudioState);
audio_state!(MenuAudioState);
audio_state!(GameLoadingAudioState);

pub mod wall_audio_orchestrator {
    pub use crate::audio::RequestWallMaterialAudio;
}
pub mod wall_audio_ui {
    pub use crate::audio::PlayWallUiSound;
}
pub mod building_audio {
    pub use crate::audio::{PlayBuildingAudio, get_building_size_01};
}
pub mod sheep_audio {
    pub use crate::audio::PlaySheepReactSound;
}
pub mod playermade_stairs_audio {
    pub use crate::audio::{PlayStairsAudioCmd, StairsAudioState};
}
pub mod audio_resources {
    pub use crate::audio::{AudioCore, AudioInstance};
}
