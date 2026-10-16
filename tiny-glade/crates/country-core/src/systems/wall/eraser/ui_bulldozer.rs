//! B `systems/wall/eraser/ui_bulldozer.rs`: bulldozer UI entry.
//!
//! Evidence: `chapters/systems/wall/eraser/ui_bulldozer.rs.md` (O,
//! 0x1409de190 / condition 0x1409de170). The condition is
//! `AppMode::Eraser(EraserLayer::Bulldoze)` (native tag 9, sub-discriminant
//! 3). The ray pick / hit-type dispatch is render domain (待接线); the
//! command -> animation-buffer bridge and the dedup guard are implemented here.

use bevy_ecs::prelude::*;

use super::animate_wall_bulldoze::AnimateWallBulldozeBuffer;
use super::AnimateWallBulldozeCmd;
use crate::recovered_building::wall_bulldoze::WallBulldozeCommand;
use crate::resources::walls::InnerWalls;
use crate::systems::mode_manager::{AppMode, EraserLayer};

/// B 0x1409de170: `(*app).tag == 9 && (*app).byte@+4 == 3` = bulldozer layer.
pub fn is_bulldozer_layer(mode: &AppMode) -> bool {
    matches!(*mode, AppMode::Eraser(EraserLayer::Bulldoze))
}

/// B 0x1409de170 as a run condition.
pub fn wall_eraser_bulldoze_condition(mode: Res<AppMode>) -> bool {
    is_bulldozer_layer(&mode)
}

/// B 0x1409de190: enqueue the bulldoze command, skipping already-animating
/// walls (the original scans AnimateWallBulldozeBuffer by WallId).
pub fn wall_eraser_bulldoze(
    mut commands: EventReader<AnimateWallBulldozeCmd>,
    mut buffer: ResMut<AnimateWallBulldozeBuffer>,
    inner: Res<InnerWalls>,
) {
    for command in commands.read() {
        if buffer.contains(command.wall) {
            continue;
        }
        let Some(state) = inner.get(command.wall).cloned() else {
            continue;
        };
        buffer.push(WallBulldozeCommand {
            original: state.snapshot().clone(),
            wall: crate::recovered_building::wall_state::WallId(command.wall.0),
            low: command.low,
            high: command.high,
            anchor: command.anchor,
            phase: 0.0,
            finished: false,
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bulldoze_layer_condition() {
        assert!(is_bulldozer_layer(&AppMode::Eraser(EraserLayer::Bulldoze)));
        assert!(!is_bulldozer_layer(&AppMode::Eraser(EraserLayer::Wall)));
        assert!(!is_bulldozer_layer(&AppMode::Wall(
            crate::systems::mode_manager::WallShape::Freeform
        )));
    }
}

/// B 0x1409e35a0: hammer cursor icon system (UI layer, 4.4 KB, chapter I).
/// The cursor icon swap is render/UI host 待接线; the library exposes the
/// bulldozer-active predicate it is keyed on.
pub fn hammer_cursor_icon() -> bool {
    is_bulldozer_layer(&crate::systems::mode_manager::AppMode::Eraser(
        EraserLayer::Bulldoze,
    ))
}
