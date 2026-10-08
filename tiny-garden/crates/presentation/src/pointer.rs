//! One viewport owner per gesture, shared by camera and world tools.
use bevy::{
    input::mouse::{AccumulatedMouseMotion, AccumulatedMouseScroll, MouseScrollUnit},
    prelude::*,
    ui::RelativeCursorPosition,
    window::PrimaryWindow,
};

#[derive(Component)]
pub struct BlocksWorldInput;

#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DesktopInputSet {
    Route,
    Camera,
    Tools,
}

#[derive(Default, Clone, Copy, PartialEq, Eq)]
enum Owner {
    #[default]
    Idle,
    Ui,
    Camera,
    World,
}

#[derive(Resource, Default)]
struct Router {
    owner: Owner,
}

#[derive(Resource, Default, Clone, Copy)]
pub struct PointerFrame {
    pub cursor: Option<Vec2>,
    pub world_begin: bool,
    pub world_held: bool,
    pub world_finish: bool,
    pub cancel_world: bool,
    pub camera_begin: bool,
    pub camera_held: bool,
    pub scroll: f32,
    pub motion: Vec2,
}

#[derive(Default)]
struct Sample {
    available: bool,
    over_ui: bool,
    cursor: Option<Vec2>,
    left_begin: bool,
    left_held: bool,
    left_end: bool,
    right_begin: bool,
    right_held: bool,
    scroll: f32,
    motion: Vec2,
}
impl Router {
    fn frame(&mut self, s: Sample) -> PointerFrame {
        let mut f = PointerFrame {
            cursor: s.cursor,
            motion: s.motion,
            ..default()
        };
        if !s.available {
            f.cancel_world = self.owner == Owner::World;
            self.owner = Owner::Idle;
            return f;
        }
        match self.owner {
            Owner::World => {
                if s.right_begin {
                    f.cancel_world = true;
                    self.owner = Owner::Idle;
                } else if s.left_end {
                    f.world_finish = !s.over_ui;
                    f.cancel_world = s.over_ui;
                    self.owner = Owner::Idle;
                } else if !s.left_held {
                    f.cancel_world = true;
                    self.owner = Owner::Idle;
                } else {
                    f.world_held = true;
                }
            }
            Owner::Camera => {
                if !s.right_held || s.left_held {
                    self.owner = Owner::Idle;
                } else {
                    f.camera_held = true;
                }
            }
            Owner::Ui => {
                if !s.left_held && !s.right_held {
                    self.owner = Owner::Idle;
                }
            }
            Owner::Idle => {
                if s.left_begin || s.right_begin {
                    if s.over_ui {
                        self.owner = Owner::Ui;
                    } else if s.left_begin && !s.right_held {
                        self.owner = Owner::World;
                        f.world_begin = true;
                        f.world_held = s.left_held;
                        if s.left_end {
                            f.world_finish = true;
                            self.owner = Owner::Idle;
                        }
                    } else if s.right_begin && !s.left_held {
                        self.owner = Owner::Camera;
                        f.camera_begin = true;
                        f.camera_held = s.right_held;
                    }
                }
            }
        }
        if !s.over_ui
            && !s.left_held
            && matches!(self.owner, Owner::Idle | Owner::Camera)
            && !f.world_finish
            && !f.cancel_world
        {
            f.scroll = s.scroll;
        }
        f
    }
}

pub struct PointerRouterPlugin;
impl Plugin for PointerRouterPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Router>()
            .init_resource::<PointerFrame>()
            .init_resource::<ButtonInput<MouseButton>>()
            .init_resource::<AccumulatedMouseMotion>()
            .init_resource::<AccumulatedMouseScroll>()
            .configure_sets(
                Update,
                (
                    DesktopInputSet::Route,
                    DesktopInputSet::Camera,
                    DesktopInputSet::Tools,
                )
                    .chain(),
            )
            .add_systems(Update, route.in_set(DesktopInputSet::Route));
    }
}
fn route(
    windows: Query<&Window, With<PrimaryWindow>>,
    buttons: Res<ButtonInput<MouseButton>>,
    motion: Res<AccumulatedMouseMotion>,
    scroll: Res<AccumulatedMouseScroll>,
    panels: Query<&RelativeCursorPosition, With<BlocksWorldInput>>,
    mut router: ResMut<Router>,
    mut frame: ResMut<PointerFrame>,
) {
    let window = windows.iter().next();
    let cursor = window.and_then(Window::cursor_position);
    *frame = router.frame(Sample {
        available: window.is_some_and(|w| w.focused) && cursor.is_some(),
        cursor,
        over_ui: panels.iter().any(|p| p.cursor_over),
        left_begin: buttons.just_pressed(MouseButton::Left),
        left_held: buttons.pressed(MouseButton::Left),
        left_end: buttons.just_released(MouseButton::Left),
        right_begin: buttons.just_pressed(MouseButton::Right),
        right_held: buttons.pressed(MouseButton::Right),
        motion: motion.delta,
        scroll: scroll.delta.y
            / if scroll.unit == MouseScrollUnit::Pixel {
                40.0
            } else {
                1.0
            },
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn ui_started_drag_cannot_become_world_drag_when_pointer_leaves_ui() {
        let mut r = Router::default();
        assert!(
            !r.frame(Sample {
                available: true,
                over_ui: true,
                left_begin: true,
                left_held: true,
                ..default()
            })
            .world_begin
        );
        let f = r.frame(Sample {
            available: true,
            left_held: true,
            ..default()
        });
        assert!(!f.world_held && !f.world_begin);
        assert!(
            !r.frame(Sample {
                available: true,
                left_end: true,
                ..default()
            })
            .world_finish
        );
    }
    #[test]
    fn world_release_over_ui_cancels_and_same_frame_click_can_finish() {
        let mut r = Router::default();
        assert!(
            r.frame(Sample {
                available: true,
                left_begin: true,
                left_held: true,
                ..default()
            })
            .world_begin
        );
        assert!(
            r.frame(Sample {
                available: true,
                over_ui: true,
                left_end: true,
                ..default()
            })
            .cancel_world
        );
        let f = r.frame(Sample {
            available: true,
            left_begin: true,
            left_end: true,
            ..default()
        });
        assert!(f.world_begin && f.world_finish);
    }
    #[test]
    fn focus_loss_and_right_press_cancel_without_restarting_held_left() {
        let mut r = Router::default();
        r.frame(Sample {
            available: true,
            left_begin: true,
            left_held: true,
            ..default()
        });
        assert!(r.frame(Sample::default()).cancel_world);
        assert!(
            !r.frame(Sample {
                available: true,
                left_held: true,
                ..default()
            })
            .world_held
        );
        r.frame(Sample {
            available: true,
            left_begin: true,
            left_held: true,
            ..default()
        });
        let f = r.frame(Sample {
            available: true,
            left_held: true,
            right_begin: true,
            right_held: true,
            ..default()
        });
        assert!(f.cancel_world && !f.camera_held);
    }
    #[test]
    fn orbit_crosses_ui_but_world_drag_excludes_camera_and_zoom() {
        let mut r = Router::default();
        assert!(
            r.frame(Sample {
                available: true,
                right_begin: true,
                right_held: true,
                ..default()
            })
            .camera_begin
        );
        assert!(
            r.frame(Sample {
                available: true,
                over_ui: true,
                right_held: true,
                ..default()
            })
            .camera_held
        );
        r.frame(Sample {
            available: true,
            ..default()
        });
        let f = r.frame(Sample {
            available: true,
            left_begin: true,
            left_held: true,
            scroll: 3.0,
            ..default()
        });
        assert!(f.world_begin && !f.camera_held && f.scroll == 0.0);
    }
}
