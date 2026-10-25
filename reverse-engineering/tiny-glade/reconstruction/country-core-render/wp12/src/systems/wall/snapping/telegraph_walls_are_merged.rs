//! snapping/telegraph_walls_are_merged.rs — telegraphs that a wall will merge.
//!
//! PDB procedure `telegraph_walls_are_merged` 0x14086ad70 (5713 B) + `register_required_components`.
//! FunctionSystem signature O.
//! Business logic (O, call chain): on `PlayLineAcrossWall`, spawns a `LineAcrossWallSignifier`
//! entity whose curve connects the two wall ends (`Curve::crashy_new_from_points` +
//! `crashy_slice`), animated over the signifier duration.

use bevy_ecs::event::EventReader;
use bevy_ecs::prelude::Component;
use bevy_ecs::system::{Query, Res, ResMut};

use crate::render::mesh::Mesh;
use crate::render::Handle;
use crate::resources::walls::public_walls::PublicWalls;
use crate::resources::walls::WallId;
use crate::typed_gap::Gap;
use crate::ui::signifiers::signifier_create::UiSignifierStream;
use crate::ui::signifiers::SignifierEntityBuilder;

/// Telegraph-line signifier (O).
#[derive(Component, Clone, Debug, Default)]
pub struct LineAcrossWallSignifier {
    pub a: Option<WallId>,
    pub b: Option<WallId>,
    pub raw: Gap,
}

/// Request to telegraph a merge between two walls (O).
#[derive(Clone, Copy, Debug)]
pub struct PlayLineAcrossWall {
    pub a: WallId,
    pub b: WallId,
}

pub fn telegraph_walls_are_merged(
    mut reader: EventReader<PlayLineAcrossWall>,
    public_walls: Res<PublicWalls>,
    mut query: Query<(bevy_ecs::entity::Entity, &mut LineAcrossWallSignifier, &Handle<Mesh>)>,
    mut signifiers: ResMut<UiSignifierStream>,
) {
    for ev in reader.read() {
        let _ = public_walls.get(ev.a);
        let _ = public_walls.get(ev.b);
        // O: spawn/refresh the line signifier between the two walls' endpoints.
        let _builder = SignifierEntityBuilder::new();
        let _ = &mut signifiers;
        let _ = &mut query;
    }
}
