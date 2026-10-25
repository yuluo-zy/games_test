//! Game event payloads (A §8 layout table). Offsets are slot-relative as documented.
use bevy_ecs::prelude::Event;
use glam::Vec3;

/// PlayWallAudio, slot stride 0x28, payload from +0x8.
#[derive(Event, Clone, Copy, Debug, PartialEq, Default)]
pub struct PlayWallAudio {
    /// kind u8 @+0x8 (jump table 0x142ab4998, 0..=23).
    pub kind: u8,
    /// byte @+0x9 (kind 10: HasBricks != 0; B §1).
    pub has_bricks: u8,
    /// f32 @+0xc (parameter source for kinds 1/2/7).
    pub field_c: f32,
    /// Vec3 @+0x10.
    pub position: Vec3,
    /// f32 @+0x1c (set_volume).
    pub volume: f32,
    /// byte @+0x20 (passed as r8d, unread by request_play).
    pub field_20: u8,
}

/// PlayBuildingAudio, stride 0x20, payload from +0 (no bevy bookmark).
#[derive(Event, Clone, Copy, Debug, PartialEq, Default)]
pub struct PlayBuildingAudio {
    /// id u64 @+0 (hash key of the 0x2a8 building entry).
    pub id: u64,
    /// Vec3 @+8.
    pub position: Vec3,
    /// f32 @+0x14.
    pub editing_speed: f32,
    /// kind u8 @+0x18.
    pub kind: u8,
}

/// PlayStairsAudioCmd, stride 0x38, payload from +0x8.
#[derive(Event, Clone, Copy, Debug, PartialEq, Default)]
pub struct PlayStairsAudioCmd {
    /// kind u8 @+0x8 (jump table 0x142bad9f0, 0..=18).
    pub kind: u8,
    /// Vec3 @+0x1c/+0x24.
    pub position: Vec3,
    // UNKNOWN(evidence: A §5b/§9): bytes @+9..+b, f32 @+c/+10/+14/+18/+28/+2c
    // (stair_width and friends) have no documented field mapping.
}

/// PlayMenuUiSound, stride 0x10, kind u8 @+0x8.
#[derive(Event, Clone, Copy, Debug, PartialEq, Default)]
pub struct PlayMenuUiSound {
    pub kind: u8,
}

/// PlaySheepReactSound, stride 0x20, kind u8 @+0x18 (B §3e).
#[derive(Event, Clone, Copy, Debug, PartialEq, Default)]
pub struct PlaySheepReactSound {
    pub kind: u8,
}

/// PlayClutterSoundCmd, stride 0x18, position @+0/+8, kind u8 @+0x14 (B §3f).
#[derive(Event, Clone, Copy, Debug, PartialEq, Default)]
pub struct PlayClutterSoundCmd {
    pub position: Vec3,
    pub kind: u8,
}

/// PlayTreeSoundCmd (kind insert=0/move=1/delete=2, in_water, flag, height, position).
/// The tree_audio system mapping is UNKNOWN(evidence: A §0/§9) in these docs; event type only.
#[derive(Event, Clone, Copy, Debug, PartialEq, Default)]
pub struct PlayTreeSoundCmd {
    pub kind: u8,
    pub in_water: bool,
    pub flag: bool,
    pub height: f32,
    pub position: Vec3,
}

macro_rules! opaque_events {
    ($($(#[$m:meta])* $name:ident),* $(,)?) => {$(
        $(#[$m])*
        #[derive(Event, Clone, Copy, Debug, PartialEq, Eq, Default)]
        pub struct $name;
    )*};
}

opaque_events! {
    /// UNKNOWN(evidence: A §1a/§9): payload layout and consumer (terrain_audio) not documented.
    PlayTerrainSound,
    /// UNKNOWN(evidence: A §1a/§9): payload and consumer (path_audio) not documented.
    SuppressPathSound,
    /// UNKNOWN(evidence: A §1a/§9): payload and consumer (replay_audio) not documented.
    PlayReplayAudio,
    /// UNKNOWN(evidence: A §1a/§9): payload and consumer (decorator_audio) not documented.
    PlayDecoratorSoundCmd,
    /// UNKNOWN(evidence: A §1a/§9): payload and consumer (wall_audio_ui) not documented.
    PlayWallUiSound,
    /// UNKNOWN(evidence: A §1a/§9): payload and consumer (wall_audio_orchestrator) not documented.
    RequestWallMaterialAudio,
    /// UNKNOWN(evidence: A §1a/§9): payload and consumer not documented.
    WallBendAudioParamReset,
    /// UNKNOWN(evidence: A §1a/§9): payload and consumer (timberframe_audio) not documented.
    PlayTimberframeSound,
    /// UNKNOWN(evidence: A §1a/§9): payload and consumer (elevation_supports_audio) not documented.
    PlayElevationSupportSoundCmd,
}
