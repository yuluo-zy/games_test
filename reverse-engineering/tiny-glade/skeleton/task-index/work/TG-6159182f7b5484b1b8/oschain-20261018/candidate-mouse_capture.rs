use bevy_ecs::prelude::*;
use glam::Vec2;

/// Native Option<Vec2>; the translator tests its discriminant, and keeps the
/// capture anchor owned by mouse capture rather than moving the window cursor.
#[derive(Resource, Clone, Copy, Debug, Default, PartialEq)]
pub struct CursorCapturedPosition(pub Option<Vec2>);

/// Native command byte: 0 captures; any nonzero byte releases (0x140b17830).
#[derive(Event, Clone, Copy, Debug, PartialEq, Eq)]
pub struct CursorCaptureCmd(pub bool);

/// Semantic names for native tags 0/1/2; names are not recovered source text.
#[derive(Event, Clone, Copy, Debug, PartialEq)]
pub enum OsCursorCaptureCmd {
    Capture(Vec2),
    Release,
    SetPosition(Vec2),
}

/// The OS combines suppression sources with bitwise OR/AND-NOT.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct OsCursorVisibilityCmdOrigin(pub u32);

impl OsCursorVisibilityCmdOrigin {
    /// 0x140b17830 emits packed 0x200000002/0x200000003.
    pub const MOUSE_CAPTURE: Self = Self(2);
}

/// Semantic projection of the four native modes handled at 0x14026f50e.
#[derive(Event, Clone, Copy, Debug, PartialEq, Eq)]
pub enum OsCursorVisibilityCmd {
    Standard,
    Custom(u32),
    Suppress(OsCursorVisibilityCmdOrigin),
    Unsuppress(OsCursorVisibilityCmdOrigin),
}

/// B 0x140b17830. Only the last unread request controls the current anchor.
pub fn mouse_capture(
    mut captured: ResMut<CursorCapturedPosition>,
    mut requests: EventReader<CursorCaptureCmd>,
    mut os_capture: EventWriter<OsCursorCaptureCmd>,
    mut os_visibility: EventWriter<OsCursorVisibilityCmd>,
    cursor: Res<super::mouse_terrain_raycast::CursorPositionSS>,
) {
    let Some(request) = requests.read().last() else {
        return;
    };
    if !request.0 {
        captured.0 = Some(cursor.0);
        os_capture.write(OsCursorCaptureCmd::Capture(cursor.0));
        os_visibility.write(OsCursorVisibilityCmd::Suppress(
            OsCursorVisibilityCmdOrigin::MOUSE_CAPTURE,
        ));
    } else {
        captured.0 = None;
        os_capture.write(OsCursorCaptureCmd::Release);
        os_visibility.write(OsCursorVisibilityCmd::Unsuppress(
            OsCursorVisibilityCmdOrigin::MOUSE_CAPTURE,
        ));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn last_request_controls_anchor_and_emits_matching_os_commands() {
        let mut world = World::new();
        world.init_resource::<CursorCapturedPosition>();
        world.insert_resource(super::super::mouse_terrain_raycast::CursorPositionSS(
            Vec2::new(17.0, 29.0),
        ));
        world.init_resource::<Events<CursorCaptureCmd>>();
        world.init_resource::<Events<OsCursorCaptureCmd>>();
        world.init_resource::<Events<OsCursorVisibilityCmd>>();
        let mut schedule = Schedule::default();
        schedule.add_systems(mouse_capture);
        world.send_event(CursorCaptureCmd(true));
        world.send_event(CursorCaptureCmd(false));
        schedule.run(&mut world);
        assert_eq!(
            world.resource::<CursorCapturedPosition>().0,
            Some(Vec2::new(17.0, 29.0))
        );
        let capture: Vec<_> = world
            .resource_mut::<Events<OsCursorCaptureCmd>>()
            .drain()
            .collect();
        let visibility: Vec<_> = world
            .resource_mut::<Events<OsCursorVisibilityCmd>>()
            .drain()
            .collect();
        assert_eq!(
            capture,
            [OsCursorCaptureCmd::Capture(Vec2::new(17.0, 29.0))]
        );
        assert_eq!(
            visibility,
            [OsCursorVisibilityCmd::Suppress(
                OsCursorVisibilityCmdOrigin(2)
            )]
        );
        world.send_event(CursorCaptureCmd(true));
        schedule.run(&mut world);
        assert_eq!(world.resource::<CursorCapturedPosition>().0, None);
        assert_eq!(
            world
                .resource_mut::<Events<OsCursorCaptureCmd>>()
                .drain()
                .collect::<Vec<_>>(),
            [OsCursorCaptureCmd::Release]
        );
        assert_eq!(
            world
                .resource_mut::<Events<OsCursorVisibilityCmd>>()
                .drain()
                .collect::<Vec<_>>(),
            [OsCursorVisibilityCmd::Unsuppress(
                OsCursorVisibilityCmdOrigin(2)
            )]
        );
    }
}
