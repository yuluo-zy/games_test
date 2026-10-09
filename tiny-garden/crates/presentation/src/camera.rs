//! Desktop mouse adapter for an orbit view; never edits the scene document.
pub use crate::pointer::BlocksWorldInput;
use crate::pointer::{DesktopInputSet, PointerFrame, PointerRouterPlugin};
use bevy::prelude::*;

const MIN_DISTANCE: f32 = 8.0;
const MAX_DISTANCE: f32 = 160.0;
const MIN_PITCH: f32 = 0.15;
const MAX_PITCH: f32 = 1.40;

pub struct OrbitCameraPlugin;
impl Plugin for OrbitCameraPlugin {
    fn build(&self, app: &mut App) {
        if !app.is_plugin_added::<PointerRouterPlugin>() {
            app.add_plugins(PointerRouterPlugin);
        }
        app.init_resource::<PanMode>().add_systems(
            Update,
            (mouse_view, pan_view, button_view)
                .chain()
                .in_set(DesktopInputSet::Camera),
        );
    }
}

/// UI switches the view adapter without camera code depending on editor state.
#[derive(Resource, Default)]
pub struct PanMode(pub bool);

#[derive(Component, Clone, Copy)]
pub enum CameraAction {
    Left,
    Right,
    Up,
    Down,
    Near,
    Far,
    Home,
}

#[derive(Clone, Copy)]
struct Pose {
    target: Vec3,
    yaw: f32,
    pitch: f32,
    distance: f32,
}

#[derive(Component)]
pub struct OrbitCamera {
    pose: Pose,
    home: Pose,
    dragging: bool,
    pan_anchor: Option<Vec3>,
}
impl OrbitCamera {
    pub fn from_view(position: Vec3, target: Vec3) -> Option<Self> {
        let offset = position - target;
        let distance = offset.length();
        if !position.is_finite() || !target.is_finite() || !distance.is_finite() || distance < 0.01
        {
            return None;
        }
        let pose = Pose {
            target,
            yaw: offset.x.atan2(offset.z),
            pitch: (offset.y / distance)
                .clamp(-1.0, 1.0)
                .asin()
                .clamp(MIN_PITCH, MAX_PITCH),
            distance: distance.clamp(MIN_DISTANCE, MAX_DISTANCE),
        };
        Some(Self {
            pose,
            home: pose,
            dragging: false,
            pan_anchor: None,
        })
    }

    pub fn transform(&self) -> Transform {
        let p = self.pose;
        let offset = Vec3::new(
            p.yaw.sin() * p.pitch.cos(),
            p.pitch.sin(),
            p.yaw.cos() * p.pitch.cos(),
        );
        Transform::from_translation(p.target + offset * p.distance).looking_at(p.target, Vec3::Y)
    }

    fn rotate(&mut self, delta: Vec2) {
        if !delta.is_finite() {
            return;
        }
        self.pose.yaw = (self.pose.yaw - delta.x * 0.005).rem_euclid(std::f32::consts::TAU);
        self.pose.pitch = (self.pose.pitch + delta.y * 0.005).clamp(MIN_PITCH, MAX_PITCH);
    }
    fn zoom(&mut self, lines: f32) {
        if !lines.is_finite() {
            return;
        }
        self.pose.distance = (self.pose.distance * (-lines.clamp(-100.0, 100.0) * 0.12).exp())
            .clamp(MIN_DISTANCE, MAX_DISTANCE);
    }
    fn mouse(&mut self, frame: MouseFrame) {
        // Losing focus/leaving the window cancels capture. Re-entry while held
        // must not restart an old drag; a fresh button press is required.
        if !frame.available || frame.left_held || !frame.held {
            self.dragging = false;
        } else if frame.begin && !frame.over_ui {
            self.dragging = true;
        }
        if self.dragging {
            self.rotate(frame.motion);
        }
        if frame.available && !frame.over_ui && !frame.left_held {
            self.zoom(frame.scroll);
        }
    }
    fn action(&mut self, action: CameraAction) {
        self.dragging = false;
        self.pan_anchor = None;
        match action {
            CameraAction::Left => self.rotate(Vec2::new(-60.0, 0.0)),
            CameraAction::Right => self.rotate(Vec2::new(60.0, 0.0)),
            CameraAction::Up => self.rotate(Vec2::new(0.0, 30.0)),
            CameraAction::Down => self.rotate(Vec2::new(0.0, -30.0)),
            CameraAction::Near => self.zoom(1.0),
            CameraAction::Far => self.zoom(-1.0),
            CameraAction::Home => self.pose = self.home,
        }
    }
}
fn ground_point(camera: &Camera, transform: &Transform, cursor: Vec2) -> Option<Vec3> {
    let ray = camera
        .viewport_to_world(&GlobalTransform::from(*transform), cursor)
        .ok()?;
    if ray.direction.y.abs() < 1e-6 {
        return None;
    }
    let distance = -ray.origin.y / ray.direction.y;
    (distance.is_finite() && distance >= 0.0).then(|| ray.get_point(distance))
}
fn pan_view(
    frame: Res<PointerFrame>,
    mode: Res<PanMode>,
    mut cameras: Query<(&Camera, &mut OrbitCamera, &mut Transform)>,
) {
    for (camera, mut orbit, mut transform) in &mut cameras {
        if !mode.0 || frame.cancel_world {
            orbit.pan_anchor = None;
            continue;
        }
        if frame.world_begin {
            orbit.pan_anchor = frame
                .cursor
                .and_then(|cursor| ground_point(camera, &transform, cursor));
        }
        if (frame.world_held || frame.world_finish)
            && let Some(anchor) = orbit.pan_anchor
            && let Some(point) = frame
                .cursor
                .and_then(|cursor| ground_point(camera, &transform, cursor))
        {
            let delta = anchor - point;
            orbit.pose.target.x = (orbit.pose.target.x + delta.x).clamp(-100.0, 100.0);
            orbit.pose.target.z = (orbit.pose.target.z + delta.z).clamp(-100.0, 100.0);
            *transform = orbit.transform();
        }
        if frame.world_finish {
            orbit.pan_anchor = None;
        }
    }
}
#[derive(Default)]
struct MouseFrame {
    available: bool,
    over_ui: bool,
    begin: bool,
    held: bool,
    left_held: bool,
    motion: Vec2,
    scroll: f32,
}

fn mouse_view(frame: Res<PointerFrame>, mut cameras: Query<(&mut OrbitCamera, &mut Transform)>) {
    for (mut camera, mut transform) in &mut cameras {
        camera.mouse(MouseFrame {
            available: true,
            over_ui: false,
            begin: frame.camera_begin,
            held: frame.camera_held,
            left_held: frame.world_held,
            motion: frame.motion,
            scroll: frame.scroll,
        });
        *transform = camera.transform();
    }
}
fn button_view(
    buttons: Query<(&Interaction, &CameraAction), Changed<Interaction>>,
    mut cameras: Query<(&mut OrbitCamera, &mut Transform)>,
) {
    for (interaction, action) in &buttons {
        if *interaction != Interaction::Pressed {
            continue;
        }
        for (mut camera, mut transform) in &mut cameras {
            camera.action(*action);
            *transform = camera.transform();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn camera() -> OrbitCamera {
        OrbitCamera::from_view(Vec3::new(40.0, 38.0, 52.0), Vec3::new(0.0, 3.0, 0.0)).unwrap()
    }
    #[test]
    fn initial_pose_matches_view_and_rejects_invalid_positions() {
        let c = camera();
        assert!(
            c.transform()
                .translation
                .distance(Vec3::new(40.0, 38.0, 52.0))
                < 0.0001
        );
        assert!(OrbitCamera::from_view(Vec3::ZERO, Vec3::ZERO).is_none());
        assert!(OrbitCamera::from_view(Vec3::NAN, Vec3::ZERO).is_none());
    }
    #[test]
    fn ground_pan_keeps_drag_anchor_under_cursor_and_home_restores_view() {
        let mut app = App::new();
        app.init_resource::<PointerFrame>()
            .insert_resource(PanMode(true))
            .add_systems(Update, pan_view);
        let orbit = camera();
        let home = orbit.transform();
        let mut view = Camera::default();
        view.computed.clip_from_view =
            Mat4::perspective_infinite_reverse_rh(std::f32::consts::PI / 3.0, 1.5, 0.1);
        view.computed.target_info = Some(bevy::camera::RenderTargetInfo {
            physical_size: UVec2::new(1440, 960),
            scale_factor: 1.0,
        });
        let start = view
            .world_to_viewport(&GlobalTransform::from(home), Vec3::ZERO)
            .unwrap();
        let end = start + Vec2::new(80.0, 30.0);
        let entity = app.world_mut().spawn((view, orbit, home)).id();
        *app.world_mut().resource_mut::<PointerFrame>() = PointerFrame {
            cursor: Some(start),
            world_begin: true,
            world_held: true,
            ..default()
        };
        app.update();
        *app.world_mut().resource_mut::<PointerFrame>() = PointerFrame {
            cursor: Some(end),
            world_held: true,
            ..default()
        };
        app.update();
        let camera = app.world().get::<Camera>(entity).unwrap();
        let transform = app.world().get::<Transform>(entity).unwrap();
        assert!(
            camera
                .world_to_viewport(&GlobalTransform::from(*transform), Vec3::ZERO)
                .unwrap()
                .distance(end)
                < 0.01
        );
        assert!((transform.translation.y - home.translation.y).abs() < 0.001);
        *app.world_mut().resource_mut::<PointerFrame>() = PointerFrame {
            cancel_world: true,
            ..default()
        };
        app.update();
        assert!(
            app.world()
                .get::<OrbitCamera>(entity)
                .unwrap()
                .pan_anchor
                .is_none()
        );
        app.world_mut()
            .get_mut::<OrbitCamera>(entity)
            .unwrap()
            .action(CameraAction::Home);
        assert_eq!(
            app.world().get::<OrbitCamera>(entity).unwrap().transform(),
            home
        );
    }
    #[test]
    fn pan_is_disabled_outside_camera_tool_and_mode_switch_releases_anchor() {
        let mut app = App::new();
        app.init_resource::<PointerFrame>()
            .init_resource::<PanMode>()
            .add_systems(Update, pan_view);
        let mut orbit = camera();
        orbit.pan_anchor = Some(Vec3::ZERO);
        let home = orbit.transform();
        let entity = app.world_mut().spawn((Camera::default(), orbit, home)).id();
        *app.world_mut().resource_mut::<PointerFrame>() = PointerFrame {
            cursor: Some(Vec2::ONE),
            world_held: true,
            ..default()
        };
        app.update();
        assert_eq!(*app.world().get::<Transform>(entity).unwrap(), home);
        assert!(
            app.world()
                .get::<OrbitCamera>(entity)
                .unwrap()
                .pan_anchor
                .is_none()
        );
    }
    #[test]
    fn zoom_and_pitch_stay_bounded_and_ignore_invalid_input() {
        let mut c = camera();
        c.zoom(10000.0);
        assert_eq!(c.pose.distance, MIN_DISTANCE);
        c.zoom(-10000.0);
        assert_eq!(c.pose.distance, MAX_DISTANCE);
        c.rotate(Vec2::new(1e8, 1e8));
        assert_eq!(c.pose.pitch, MAX_PITCH);
        let before = c.transform();
        c.zoom(f32::NAN);
        c.rotate(Vec2::NAN);
        assert_eq!(before, c.transform());
    }
    #[test]
    fn scene_drag_retains_ownership_when_crossing_ui() {
        let mut c = camera();
        c.mouse(MouseFrame {
            available: true,
            begin: true,
            held: true,
            ..default()
        });
        let before = c.transform();
        c.mouse(MouseFrame {
            available: true,
            held: true,
            over_ui: true,
            motion: Vec2::new(12.0, 3.0),
            ..default()
        });
        assert_ne!(before, c.transform());
        c.mouse(MouseFrame::default());
        assert!(!c.dragging);
    }
    #[test]
    fn ui_press_and_focus_loss_never_leak_or_restart_a_drag() {
        let mut c = camera();
        let before = c.transform();
        c.mouse(MouseFrame {
            available: true,
            begin: true,
            held: true,
            over_ui: true,
            motion: Vec2::ONE,
            scroll: 3.0,
            ..default()
        });
        assert_eq!(before, c.transform());
        c.mouse(MouseFrame {
            available: true,
            held: true,
            motion: Vec2::ONE,
            ..default()
        });
        assert_eq!(before, c.transform());
        c.mouse(MouseFrame {
            available: true,
            begin: true,
            held: true,
            ..default()
        });
        c.mouse(MouseFrame {
            held: true,
            motion: Vec2::ONE,
            ..default()
        });
        c.mouse(MouseFrame {
            available: true,
            held: true,
            motion: Vec2::ONE,
            ..default()
        });
        assert_eq!(before, c.transform());
        assert!(!c.dragging);
    }
    #[test]
    fn visible_buttons_drive_view_and_home_restores_it() {
        let mut app = App::new();
        app.add_plugins(OrbitCameraPlugin);
        let c = camera();
        let home = c.transform();
        let entity = app.world_mut().spawn((c, home)).id();
        let button = app
            .world_mut()
            .spawn((Interaction::Pressed, CameraAction::Near))
            .id();
        app.update();
        assert_ne!(*app.world().get::<Transform>(entity).unwrap(), home);
        app.world_mut()
            .entity_mut(button)
            .insert((Interaction::None, CameraAction::Home));
        app.update();
        app.world_mut()
            .entity_mut(button)
            .insert(Interaction::Pressed);
        app.update();
        assert_eq!(*app.world().get::<Transform>(entity).unwrap(), home);
    }
}
