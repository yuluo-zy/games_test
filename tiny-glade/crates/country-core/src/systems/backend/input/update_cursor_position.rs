//! 将窗口位移应用到游标；捕获保持锚点，另将原始输入位移交给工具消费者。
use bevy_ecs::prelude::*;
use glam::Vec2;

use super::translate_input_to_cursor_moved::clamp_window_position;
use crate::{
    input::action_set::ActionSetStates,
    resources::{
        mouse_capture::CursorCapturedPosition,
        mouse_terrain_raycast::CursorPositionSS,
        window::{CursorMovedScreenSpace, WindowSize},
    },
};

/// 录制和回放使用的整数像素位置。
#[derive(Resource, Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct CursorPosition {
    pub x: u32,
    pub y: u32,
}
impl CursorPosition {
    pub fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }
    pub fn as_vec2(self) -> Vec2 {
        Vec2::new(self.x as f32, self.y as f32)
    }
}

/// 在游戏消费者之前生成本帧屏幕坐标和工具位移事件。
pub fn update_cursor_position(
    states: Res<ActionSetStates>,
    captured: Res<CursorCapturedPosition>,
    window: Res<WindowSize>,
    mut cursor: ResMut<CursorPositionSS>,
    mut events: EventWriter<CursorMovedScreenSpace>,
) {
    let next = captured.0.unwrap_or(cursor.0 + states.analog[1]);
    let next = clamp_window_position(next, window.as_vec2());
    cursor.0 = next;
    // Position and tool motion use different existing action slots. Native
    // +0x148 updates the window cursor, while +0x140 is the event payload;
    // capture retains tool motion and zero motion produces no event.
    let movement = states.analog[0];
    if movement != Vec2::ZERO {
        events.write(CursorMovedScreenSpace(movement));
    }
}

#[cfg(test)]
mod native_event_tests {
    use super::*;
    use crate::{
        resources::terrain_editing::StrokeId,
        systems::terrain::ui_edit_terrain::edit_command_for_drag,
    };
    use glam::Vec3;
    use tg_terrain::edit::{Adjustment, EditCommand};

    fn world(input: [u32; 2], hardware: [u32; 2], captured: bool) -> World {
        let mut world = World::new();
        let mut states = ActionSetStates::default();
        states.analog[0] = Vec2::from_array(input.map(f32::from_bits));
        states.analog[1] = Vec2::from_array(hardware.map(f32::from_bits));
        world.insert_resource(states);
        world.insert_resource(CursorPositionSS(Vec2::new(50.0, 20.0)));
        world.insert_resource(CursorCapturedPosition(
            captured.then_some(Vec2::new(50.0, 20.0)),
        ));
        world.insert_resource(WindowSize::new(100, 100));
        world.init_resource::<Events<CursorMovedScreenSpace>>();
        world
    }

    #[test]
    fn original_event_payload_and_nonzero_gate() {
        // Executed original B inline fixtures. Position assertions are limited
        // to inputs compatible with the retained current clamp/anchor policy.
        for (input, hardware, captured, expected_position, expected_payload) in [
            (
                [0x0, 0x0],
                [0x0, 0x0],
                false,
                [0x42480000, 0x41a00000],
                None,
            ), // idle
            (
                [0x40400000, 0x41a00000],
                [0x3f800000, 0xc0000000],
                false,
                [0x424c0000, 0x41900000],
                Some([0x40400000, 0x41a00000]),
            ), // raw_differs_from_hardware
            (
                [0x40400000, 0x41a00000],
                [0x42c60000, 0x42c60000],
                true,
                [0x42480000, 0x41a00000],
                Some([0x40400000, 0x41a00000]),
            ), // captured_still_emits_raw
            (
                [0x0, 0x0],
                [0x3f800000, 0xc0000000],
                false,
                [0x424c0000, 0x41900000],
                None,
            ), // hardware_without_raw
            (
                [0x80000000, 0x80000000],
                [0x0, 0x0],
                false,
                [0x42480000, 0x41a00000],
                None,
            ), // negative_zero_is_zero
            (
                [0x7fc01234, 0x3f800000],
                [0x0, 0x0],
                false,
                [0x42480000, 0x41a00000],
                Some([0x7fc01234, 0x3f800000]),
            ), // nan_x_is_nonzero
            (
                [0x7f800000, 0xff800000],
                [0x0, 0x0],
                false,
                [0x42480000, 0x41a00000],
                Some([0x7f800000, 0xff800000]),
            ), // infinities_are_nonzero
            (
                [0x1, 0x80000001],
                [0x0, 0x0],
                false,
                [0x42480000, 0x41a00000],
                Some([0x1, 0x80000001]),
            ), // subnormals_are_nonzero
            (
                [0x0, 0x7fc00000],
                [0x0, 0x0],
                false,
                [0x42480000, 0x41a00000],
                Some([0x0, 0x7fc00000]),
            ), // nan_y_is_nonzero
            (
                [0xc0400000, 0xc1a00000],
                [0xbf800000, 0x40000000],
                false,
                [0x42440000, 0x41b00000],
                Some([0xc0400000, 0xc1a00000]),
            ), // negative_raw
        ] {
            let mut world = world(input, hardware, captured);
            let mut schedule = Schedule::default();
            schedule.add_systems(update_cursor_position);
            schedule.run(&mut world);
            assert_eq!(
                world
                    .resource::<CursorPositionSS>()
                    .0
                    .to_array()
                    .map(f32::to_bits),
                expected_position
            );
            let payloads: Vec<_> = world
                .resource_mut::<Events<CursorMovedScreenSpace>>()
                .drain()
                .map(|event| event.0.to_array().map(f32::to_bits))
                .collect();
            assert_eq!(payloads, expected_payload.into_iter().collect::<Vec<_>>());
        }
    }

    #[derive(Resource, Default)]
    struct DragCommands(Vec<EditCommand>);

    fn consume_drag(
        mut events: EventReader<CursorMovedScreenSpace>,
        mut commands: ResMut<DragCommands>,
    ) {
        for event in events.read() {
            if let Some(command) = edit_command_for_drag(StrokeId(7), 5.0, Vec3::ZERO, event.0) {
                commands.0.push(command);
            }
        }
    }

    #[test]
    fn captured_tool_motion_reaches_existing_terrain_drag_consumer_once() {
        let mut world = world([1.0_f32.to_bits(), 0], [0, 0], true);
        world.init_resource::<DragCommands>();
        let mut schedule = Schedule::default();
        schedule.add_systems((update_cursor_position, consume_drag).chain());
        schedule.run(&mut world);
        let commands = &world.resource::<DragCommands>().0;
        assert_eq!(commands.len(), 1);
        assert_eq!(commands[0].id, StrokeId(7));
        assert_eq!(commands[0].falloff, Adjustment::Delta(0.002));
        assert_eq!(commands[0].height, Adjustment::Keep);
        assert_eq!(
            world.resource::<CursorPositionSS>().0,
            Vec2::new(50.0, 20.0)
        );

        world.resource_mut::<ActionSetStates>().analog[0] = Vec2::ZERO;
        schedule.run(&mut world);
        assert_eq!(world.resource::<DragCommands>().0.len(), 1);
    }
}
