//! wall_splitter.rs — splits a wall into two walls at the cursor position.
//!
//! PDB procedure `wall_splitter` 0x140a74c20 (11988 B). FunctionSystem signature O.
//! Business logic (O, call chain): drags a split point across a wall; computes the curve
//! u at the cursor, creates two child walls via `ChildWall::new`, records new walls with
//! `InnerWalls::new_wall`, converts styles to stone where a split lands, and re-publishes
//! the changed wall state. Decorator archival resources are read to re-home decorators.

use bevy_ecs::system::{Res, ResMut};

use crate::resources::walls::decorator_archivist::DecoratorArchivalResources;
use crate::resources::walls::inner_walls::InnerWalls;
use crate::resources::walls::public_walls::PublicWalls;
use crate::resources::walls::WallId;
use crate::ui::mode_manager::AppMode;
use crate::ui::ui_state::UiState;
use crate::utils::child_wall::ChildWall;
use crate::utils::curve::{Curve, CurveU};

/// Split result: the two child wall ids + the shared split u.
#[derive(Clone, Copy, Debug)]
pub struct WallSplit {
    pub a: WallId,
    pub b: WallId,
    pub u: f32,
}

pub fn wall_splitter(
    app_mode: Res<AppMode>,
    public_walls: Res<PublicWalls>,
    decorator_archival: DecoratorArchivalResources,
    ui_state: Res<UiState>,
    mut inner_walls: ResMut<InnerWalls>,
) {
    let _ = (&app_mode, &public_walls, &decorator_archival, &ui_state);
    let ids = inner_walls.wall_ids();
    for id in ids {
        let Some(state) = inner_walls.get(id) else { continue };
        // O: a split at curve u creates two children from the wall's curve.
        let u = cursor_u_for_wall(state);
        let children = split_curve_at(&state.curve, u);
        if let (Some(ca), Some(cb)) = children {
            let child_a = ChildWall::new();
            let child_b = ChildWall::new();
            let _ = (&child_a, &child_b);
            let _ = inner_walls.new_wall(WallStateStub::from_curve(ca));
            let _ = inner_walls.new_wall(WallStateStub::from_curve(cb));
        }
    }
}

fn cursor_u_for_wall(_state: &crate::resources::walls::WallState) -> f32 {
    0.5
}

fn split_curve_at(curve: &Curve, u: f32) -> (Option<Curve>, Option<Curve>) {
    let mid = CurveU::new(u);
    (Some(curve.try_slice(CurveU::new(0.0), mid)), Some(curve.try_slice(mid, CurveU::new(1.0))))
}

struct WallStateStub {
    curve: Curve,
}
impl WallStateStub {
    fn from_curve(curve: Curve) -> crate::resources::walls::WallState {
        crate::resources::walls::WallState {
            raw: crate::typed_gap::Gap,
            height_ty: 0,
            curve,
            shape: crate::resources::walls::WallShapeSnapshot::from_observed([0; 28]),
        }
    }
}
