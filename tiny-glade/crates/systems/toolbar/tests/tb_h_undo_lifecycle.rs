//! Real Bevy owner systems compared with full original-system fixed outputs.
use bevy_ecs::prelude::*;
use bevy_time::Time;
use country_core::{
    game_config::GameConfig,
    input::action_set::{ActionSetStates, DigitalActionPhase},
    resources::{
        gui::EscMenuState, mouse_terrain_raycast::CursorPositionSS, ui_settings::ToolbarAlignment,
        window::WindowSize,
    },
    systems::ui::resources::widget_params::PlayUiSound,
};
use glam::Vec2;
use toolbar::{
    resouces_and_components::{
        taffy_layout::{AnchorSide, AnchoredNode, TaffyLayout},
        undo_redo_layout::UndoRedoLayout,
    },
    undo_redo_systems::{undo_redo_alignment, undo_redo_auto_hide},
};
fn n(b: &[u8], a: usize) -> u32 {
    u32::from_le_bytes(b[a..a + 4].try_into().unwrap())
}
fn f(b: &[u8], a: usize) -> f32 {
    f32::from_bits(n(b, a))
}
fn alignment(n: u8) -> ToolbarAlignment {
    match n {
        0 => ToolbarAlignment::Bottom,
        1 => ToolbarAlignment::Left,
        2 => ToolbarAlignment::Right,
        3 => ToolbarAlignment::Top,
        _ => panic!(),
    }
}
fn setup() -> World {
    let mut world = World::new();
    let mut taffy = TaffyLayout::default();
    let screen = taffy.taffy.new_leaf(Default::default()).unwrap();
    let buttons = taffy.taffy.new_leaf(Default::default()).unwrap();
    for node in [screen, screen, buttons] {
        taffy.anchored.push(AnchoredNode {
            order: 0,
            node,
            offset: Vec2::splat(17.),
            relative_to: node,
            gap: 0.,
            side: AnchorSide::Window,
        });
    }
    world.insert_resource(taffy);
    world.insert_resource(UndoRedoLayout {
        screen,
        buttons,
        hide_progress: 0.,
    });
    world.insert_resource(GameConfig::default());
    world.insert_resource(Time::<()>::default());
    world.insert_resource(WindowSize::new(1280, 720));
    world.insert_resource(CursorPositionSS(Vec2::ZERO));
    world.insert_resource(EscMenuState::Closed);
    world.init_resource::<ActionSetStates>();
    world.init_resource::<Events<PlayUiSound>>();
    world
}
#[test]
fn real_undo_auto_hide_matches_full_native_frames_and_only_first_matching_anchor() {
    let mut world = setup();
    let mut schedule = Schedule::default();
    schedule.add_systems(undo_redo_auto_hide);
    let fixture=include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"),"/../../../../reverse-engineering/tiny-glade/progress-20261020/phase2-undo-systems/native-auto-hide.bin"));
    let mut cases = 0;
    for (index, b) in fixture.chunks_exact(52).enumerate() {
        if f(b, 24) < 0. {
            continue;
        } // Time's public clock accepts nonnegative Duration.
        {
            let mut config = world.resource_mut::<GameConfig>();
            config.ui.toolbar_alignment = alignment(b[0]);
            config.ui.auto_hide_toolbar = b[1] != 0;
            config.ui.icon_size_mult = f(b, 16);
        }
        world
            .resource_mut::<Time>()
            .advance_by(std::time::Duration::from_secs_f32(f(b, 24)));
        *world.resource_mut::<WindowSize>() = WindowSize::new(n(b, 8), n(b, 12));
        world.resource_mut::<CursorPositionSS>().0 = Vec2::new(f(b, 28), f(b, 32));
        *world.resource_mut::<EscMenuState>() = match b[2] {
            2 => EscMenuState::Closed,
            3 => EscMenuState::Main,
            10 => EscMenuState::Photomode(b[3] != 0),
            _ => panic!(),
        };
        world.resource_mut::<ActionSetStates>().digital[0].phase = match n(b, 4) {
            0 => DigitalActionPhase::JustPressed,
            1 => DigitalActionPhase::Held,
            2 => DigitalActionPhase::Repeating,
            3 => DigitalActionPhase::JustReleased,
            4 => DigitalActionPhase::Idle,
            _ => panic!(),
        };
        world.resource_mut::<UndoRedoLayout>().hide_progress = f(b, 20);
        for anchor in &mut world.resource_mut::<TaffyLayout>().anchored {
            anchor.offset = Vec2::splat(17.);
        }
        schedule.run(&mut world);
        let actual = world.resource::<UndoRedoLayout>().hide_progress.to_bits();
        assert_eq!(actual, n(b, 36), "progress native{index}");
        let actual = world.resource::<TaffyLayout>().anchored[0].offset;
        assert_eq!(
            [actual.x.to_bits(), actual.y.to_bits()],
            [n(b, 40), n(b, 44)],
            "anchor native{index}"
        );
        assert!(world.resource::<TaffyLayout>().anchored[1..]
            .iter()
            .all(|a| a.offset == Vec2::splat(17.)));
        let sounds: Vec<_> = world
            .resource_mut::<Events<PlayUiSound>>()
            .drain()
            .map(|s| s.kind)
            .collect();
        assert_eq!(
            sounds,
            if n(b, 48) == u32::MAX {
                vec![]
            } else {
                vec![n(b, 48)]
            },
            "sound native{index}"
        );
        cases += 1;
    }
    assert_eq!(cases, 3600);
}
#[test]
fn real_undo_alignment_updates_both_nodes_for_all_original_edges() {
    use taffy::style::{AlignItems as A, FlexDirection as F, JustifyContent as J};
    let mut world = setup();
    let mut schedule = Schedule::default();
    schedule.add_systems(undo_redo_alignment);
    for (value, justify, align, flow) in [
        (0, J::Center, A::FlexStart, F::Row),
        (1, J::FlexEnd, A::Center, F::Column),
        (2, J::FlexStart, A::Center, F::Column),
        (3, J::Center, A::FlexEnd, F::Row),
    ] {
        world.resource_mut::<GameConfig>().ui.toolbar_alignment = alignment(value);
        schedule.run(&mut world);
        let undo = world.resource::<UndoRedoLayout>();
        let taffy = world.resource::<TaffyLayout>();
        let screen = taffy.taffy.style(undo.screen).unwrap();
        assert_eq!(screen.justify_content, Some(justify));
        assert_eq!(screen.align_items, Some(align));
        assert_eq!(
            taffy.taffy.style(undo.buttons).unwrap().flex_direction,
            flow
        );
        assert!(taffy.anchored.iter().all(|a| a.offset == Vec2::splat(17.)));
    }
}
