//! `ui_clutter/ui_autoclutter.rs` — convert auto-clutter into deco-clutter.
//!
//! PDB: `ui_convert_autoclutter_to_decoclutter @0x14099ca60` (8136 B) +
//! `closure$1 @0x14099ea30`. Named entities `EvRightClickedAutoClutter`,
//! `InstaCreateDeco`, `InstaCreateStairPoint`, `StartPlaceDecoClutterInteraction`.

#![allow(dead_code, unused_imports)]

use bevy_ecs::prelude::*;

use crate::Gap;

/// `EvRightClickedAutoClutter` (O named entity; event).
#[derive(bevy_ecs::event::Event, Clone, Copy, Debug, Default)]
pub struct EvRightClickedAutoClutter {
    pub _gap: Gap,
}

/// `InstaCreateDeco` (O named entity; event).
#[derive(bevy_ecs::event::Event, Clone, Copy, Debug, Default)]
pub struct InstaCreateDeco {
    pub _gap: Gap,
}

/// `InstaCreateStairPoint` (O named entity; event).
#[derive(bevy_ecs::event::Event, Clone, Copy, Debug, Default)]
pub struct InstaCreateStairPoint {
    pub _gap: Gap,
}

/// `StartPlaceDecoClutterInteraction` (O named entity; event).
#[derive(bevy_ecs::event::Event, Clone, Copy, Debug, Default)]
pub struct StartPlaceDecoClutterInteraction {
    pub _gap: Gap,
}

/// `ui_convert_autoclutter_to_decoclutter` @0x14099ca60 (O).
pub fn ui_convert_autoclutter_to_decoclutter(
    mut _right_clicked: EventReader<EvRightClickedAutoClutter>,
    mut _insta_deco: EventWriter<InstaCreateDeco>,
    mut _insta_stair: EventWriter<InstaCreateStairPoint>,
    _commands: Commands,
) {
    todo!("recover ui_convert_autoclutter_to_decoclutter (RMB → deco/stair conversion)")
}
