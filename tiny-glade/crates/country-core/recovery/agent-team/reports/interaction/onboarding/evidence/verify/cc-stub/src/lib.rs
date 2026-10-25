//! Minimal stub of country-core for system-onboarding compile/test verification.
//! Types mirror the workspace country-core surface consumed by this crate.
use bevy_ecs::prelude::*;
use glam::Vec2;

pub mod resources {
    pub mod time {
        #[derive(bevy_ecs::prelude::Resource, Clone, Copy, Debug, Default)]
        pub struct GameClock {
            pub delta: f32,
        }
        pub type GameTime = GameClock;
    }
    pub mod window {
        #[derive(bevy_ecs::prelude::Resource, Clone, Copy, Debug, Default)]
        pub struct WindowSize(pub glam::Vec2);
        impl WindowSize {
            pub fn as_vec2(self) -> glam::Vec2 {
                self.0
            }
        }
        #[derive(bevy_ecs::event::Event, Clone, Copy, Debug)]
        pub struct CursorMovedScreenSpace(pub glam::Vec2);
    }
    pub mod gui {
        #[derive(bevy_ecs::prelude::Resource, Clone, Copy, Debug, Default)]
        pub struct EscMenuState;
        impl EscMenuState {
            pub fn is_open(&self) -> bool {
                false
            }
        }
    }
    pub mod events {
        #[derive(bevy_ecs::event::Event, Clone, Copy, Debug, Default)]
        pub struct ModeJustChanged;
    }
    pub mod onboarding {
        #[derive(bevy_ecs::event::Event, Clone, Copy, Debug)]
        pub struct NotifyHintSystemToolWasJustUsed;
    }
    pub mod walls {
        pub mod public_walls {
            #[derive(bevy_ecs::prelude::Resource, Clone, Copy, Debug, Default)]
            pub struct PublicWalls;
        }
    }
    pub mod roofs {
        pub mod roof_shape {
            #[derive(bevy_ecs::component::Component, Clone, Copy, Debug, Default)]
            pub struct Roof;
            impl Roof {
                pub fn is_flat(&self) -> bool {
                    false
                }
            }
        }
    }
}

pub mod game_profile {
    #[derive(bevy_ecs::prelude::Resource, Clone, Debug, Default)]
    pub struct GameProfile {
        pub tutorial: bool,
    }
}

pub mod systems {
    pub mod mode_manager {
        #[derive(bevy_ecs::prelude::Resource, Clone, Copy, Debug, Default)]
        pub struct AppMode;
        impl AppMode {
            pub fn is_tool_selected(self) -> bool {
                false
            }
        }
    }
    pub mod collision {
        pub mod world {
            #[derive(bevy_ecs::prelude::Resource, Clone, Copy, Debug, Default)]
            pub struct RaycastWorld;
            pub struct RaycastWorldResult;
            impl RaycastWorld {
                pub fn raycast(&self, _o: glam::Vec3, _d: glam::Vec3, _m: u32) -> RaycastWorldResult {
                    RaycastWorldResult
                }
                pub fn legacy_raycast(&self, _o: glam::Vec3, _d: glam::Vec3) -> RaycastWorldResult {
                    RaycastWorldResult
                }
            }
        }
    }
    pub mod ui {
        pub mod resources {
            pub mod widget_params {
                #[derive(bevy_ecs::event::Event, Clone, Copy, Debug)]
                pub struct PlayUiSound;
            }
        }
    }
}
