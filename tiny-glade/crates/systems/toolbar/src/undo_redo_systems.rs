//! Original undo/redo alignment1423448a0 and auto-hide1423449f0.
//! Native evidence: progress-20261020/phase2-undo-systems/original.asm.
use crate::resouces_and_components::{taffy_layout::TaffyLayout, undo_redo_layout::UndoRedoLayout};
use crate::toolbar_systems::{
    alignment_closures::*, auto_hide_progress, auto_hide_sound_kind, taffy_mut_style,
};
use bevy_ecs::prelude::*;
use bevy_time::Time;
use country_core::{
    game_config::GameConfig,
    input::action_set::ActionSetStates,
    resources::{gui::EscMenuState, mouse_terrain_raycast::CursorPositionSS, window::WindowSize},
    systems::ui::resources::widget_params::PlayUiSound,
};
use glam::Vec2;
use utils::{easing_functions::EasingFunction, geometry::aabb::Aabb2};

pub fn undo_redo_alignment(
    config: Res<GameConfig>,
    mut taffy: ResMut<TaffyLayout>,
    undo: Res<UndoRedoLayout>,
) {
    // Three native style calls, in order, on the actual two layout nodes.
    let (justify, align, flow): (
        fn(&mut taffy::style::Style),
        fn(&mut taffy::style::Style),
        fn(&mut taffy::style::Style),
    ) = match config.ui.toolbar_alignment as u8 {
        0 => (toolbar_env_12, toolbar_env_13, toolbar_env_10),
        1 => (undo_redo_env_0, undo_redo_env_1, toolbar_env_2),
        2 => (toolbar_env_0, undo_redo_env_1, toolbar_env_2),
        3 => (toolbar_env_12, toolbar_env_11, toolbar_env_10),
        _ => unreachable!("native ToolbarAlignment"),
    };
    taffy_mut_style(&mut taffy.taffy, undo.screen, justify);
    taffy_mut_style(&mut taffy.taffy, undo.screen, align);
    taffy_mut_style(&mut taffy.taffy, undo.buttons, flow);
}

/// Original activation geometry uses 4*110+30, even though the layout has two
/// history buttons. Keep the original dimensions, float order and both margins.
pub fn undo_activation_geometry(alignment: u8, window: Vec2, icon_scale: f32) -> (Vec2, Vec2) {
    let button = 110. * icon_scale;
    let total = 30. * icon_scale + 4. * button;
    let (center, dimensions) = match alignment {
        0 => (
            Vec2::new(window.x * 0.5, button * 0.5),
            Vec2::new(total, button),
        ),
        1 => (
            Vec2::new(window.x - button * 0.5, window.y * 0.5),
            Vec2::new(button, total),
        ),
        2 => (
            Vec2::new(button * 0.5, window.y * 0.5),
            Vec2::new(button, total),
        ),
        3 => (
            Vec2::new(window.x * 0.5, window.y - button * 0.5),
            Vec2::new(total, button),
        ),
        _ => unreachable!("native ToolbarAlignment"),
    };
    let margin = 40. * icon_scale;
    (center, dimensions + Vec2::splat(margin + margin))
}

pub fn undo_redo_offset(alignment: u8, icon_scale: f32, progress: f32) -> Vec2 {
    let offset = (icon_scale * 100.) * EasingFunction::CubicInOut.apply(progress);
    match alignment {
        0 => Vec2::new(0., -offset),
        1 => Vec2::new(offset, 0.),
        2 => Vec2::new(-offset, 0.),
        3 => Vec2::new(0., offset),
        _ => unreachable!("native ToolbarAlignment"),
    }
}

/// PDB's complete nine-parameter system; owns its independent hide progress.
/// No AppMode, ToolbarSizeMult or current toolbar category enters this path.
pub fn undo_redo_auto_hide(
    cursor: Res<CursorPositionSS>,
    menu: Res<EscMenuState>,
    config: Res<GameConfig>,
    time: Res<Time>,
    window: Res<WindowSize>,
    actions: Res<ActionSetStates>,
    mut undo: ResMut<UndoRedoLayout>,
    mut taffy: ResMut<TaffyLayout>,
    mut sounds: EventWriter<PlayUiSound>,
) {
    let alignment = config.ui.toolbar_alignment as u8;
    if config.ui.auto_hide_toolbar {
        let (center, dimensions) =
            undo_activation_geometry(alignment, window.as_vec2(), config.ui.icon_size_mult);
        let inside = Aabb2::from_center_dims(center, dimensions).contains(cursor.0);
        let old = undo.hide_progress;
        undo.hide_progress = auto_hide_progress(
            old,
            time.delta_secs(),
            inside,
            menu.is_closed(),
            actions.digital[0].pressed(),
        );
        if let Some(kind) = auto_hide_sound_kind(old, undo.hide_progress) {
            sounds.write(PlayUiSound {
                kind,
                _payload: [0; 0x10],
                id: 0,
            });
        }
    } else {
        undo.hide_progress = 0.;
    }
    let offset = undo_redo_offset(alignment, config.ui.icon_size_mult, undo.hide_progress);
    if let Some(anchor) = taffy
        .anchored
        .iter_mut()
        .find(|entry| entry.node == undo.screen)
    {
        anchor.offset = offset;
    }
}
