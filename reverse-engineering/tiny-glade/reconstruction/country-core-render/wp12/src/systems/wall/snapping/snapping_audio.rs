//! snapping/snapping_audio.rs — plays UI sounds when snapping engages.
//!
//! PDB procedure `snapping_audio` 0x140a6a480 (223 B). FunctionSystem signature O.
//! Business logic (O, decompiled 0x140a6a480): when `SnappingMode` is in mode 2 (ends) and
//! the sound flag differs, or mode 3 with snap-count <= 1, emits a `PlayWallUiSound` event.

use bevy_ecs::event::EventWriter;
use bevy_ecs::system::Res;

use crate::audio::PlayWallUiSound;
use crate::systems::wall::snapping::SnappingMode;

pub fn snapping_audio(snapping: Res<SnappingMode>, mut writer: EventWriter<PlayWallUiSound>) {
    if snapping.mode == 2 && !snapping.snapped {
        writer.write(PlayWallUiSound { raw: crate::typed_gap::Gap });
    } else if snapping.mode == 3 {
        writer.write(PlayWallUiSound { raw: crate::typed_gap::Gap });
    }
}
