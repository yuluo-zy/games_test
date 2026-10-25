//! `crate::ui` — WP12 wall-domain UI typed-gap boundary.
//!
//! The wall systems (`systems::wall`) were recovered against this UI facade
//! surface (AppMode / Modifiers / bool-predicate UiState / builder-signifier
//! surface / typed-gap `bevy_app::App`), which is a distinct layer from the
//! wp15-recovered widget state at `crate::systems::ui`. Kept as a genuine
//! typed-gap boundary (documented; no fabricated widget API).

use bevy_ecs::prelude::{Component, Resource};
use bevy_ecs::system::SystemParam;

use crate::typed_gap::Gap;

pub mod mode_manager {
    use super::*;
    /// Active app mode (enum resource). Modes relevant to wall systems:
    /// Draw wall / adjust wall height / bulldoze / shape-edit.
    #[derive(Resource, Clone, Copy, Debug, PartialEq, Eq)]
    pub enum AppMode {
        Idle,
        DrawWall,
        AdjustWallHeight,
        Bulldozer,
        EditShape,
        Other,
    }
}

pub mod ui_state {
    use super::*;
    use bevy_ecs::system::SystemParam;

    /// Widget interaction state (O: `UiState::intersecting/dragged/hovered/clicked`).
    #[derive(Resource, Clone, Debug, Default)]
    pub struct UiState {
        pub raw: Gap,
    }
    impl UiState {
        pub fn clicked(&self) -> bool {
            false
        }
        pub fn dragged(&self) -> bool {
            false
        }
        pub fn hovered(&self) -> bool {
            false
        }
        pub fn intersecting(&self, _name: u32) -> bool {
            false
        }
        pub fn is_active(&self) -> bool {
            false
        }
        pub fn is_widget_name_active(&self, _name: u32) -> bool {
            false
        }
        pub fn current_intersection(&self) -> Option<u32> {
            None
        }
        pub fn clearing_canvas(&self) -> bool {
            false
        }
        pub fn clearing_canvas_w_layer(&self) -> bool {
            false
        }
    }

    /// Widget interaction context passed alongside UiState (SystemParam-style).
    #[derive(SystemParam, Debug)]
    pub struct UiWidgetContext<'w, 's> {
        _marker: std::marker::PhantomData<(&'w (), &'s ())>,
    }
    impl UiWidgetContext<'_, '_> {
        pub fn clicked(&self) -> bool {
            false
        }
        pub fn dragged(&self) -> bool {
            false
        }
        pub fn hovered(&self) -> bool {
            false
        }
        pub fn intersecting(&self, _name: u32) -> bool {
            false
        }
        pub fn is_active(&self) -> bool {
            false
        }
    }
}

pub mod widget_params {
    use super::*;

    pub struct WidgetParams {
        pub raw: Gap,
    }
    impl WidgetParams {
        pub fn new() -> Self {
            Self { raw: Gap }
        }
        pub fn occlude_by_world(self, _b: bool) -> Self {
            self
        }
        pub fn sounds(self, _s: UiSounds) -> Self {
            self
        }
    }
    impl Default for WidgetParams {
        fn default() -> Self {
            Self::new()
        }
    }

    #[derive(Clone, Debug, Default)]
    pub struct UiSounds {
        pub raw: Gap,
    }
    impl UiSounds {
        pub fn empty() -> Self {
            Self { raw: Gap }
        }
    }
}

pub mod signifiers {
    use super::*;
    use bevy_ecs::prelude::Component;

    /// Animated signifier marker + t value.
    #[derive(Component, Clone, Copy, Debug, Default)]
    pub struct SignifierAnimation {
        pub raw: Gap,
    }
    impl SignifierAnimation {
        pub fn t(&self) -> f32 {
            0.0
        }
    }

    #[derive(Component, Clone, Copy, Debug, Default)]
    pub struct SignifierMarkedForDeletion {
        pub raw: Gap,
    }

    #[derive(Clone, Debug, Default)]
    pub struct SignifierEntityBuilder {
        pub raw: Gap,
    }
    impl SignifierEntityBuilder {
        pub fn new() -> Self {
            Self { raw: Gap }
        }
        pub fn position(mut self, _p: glam::Vec3) -> Self {
            self
        }
        pub fn rotation(mut self, _q: glam::Quat) -> Self {
            self
        }
        pub fn scale(mut self, _s: f32) -> Self {
            self
        }
        pub fn animation_duration(mut self, _d: f32) -> Self {
            self
        }
        pub fn disable_default_animation(mut self) -> Self {
            self
        }
        pub fn custom_shader(mut self, _uri: crate::render::pass_builder::shader_uri::ShaderUri) -> Self {
            self
        }
        pub fn parent_to_widget(mut self, _e: bevy_ecs::entity::Entity) -> Self {
            self
        }
        pub fn set_ui_pass(mut self) -> Self {
            self
        }
        pub fn spawn(&self, _commands: &mut bevy_ecs::system::Commands) -> bevy_ecs::entity::Entity {
            _commands.spawn_empty().id()
        }
    }

    #[derive(Clone, Debug, Default)]
    pub struct SignifierFactory {
        pub raw: Gap,
    }
    impl SignifierFactory {
        pub fn new() -> Self {
            Self { raw: Gap }
        }
    }

    /// Stream that materialises signifier entities from builders.
    pub mod signifier_create {
        use super::*;
        #[derive(Resource, Clone, Debug, Default)]
        pub struct UiSignifierStream {
            pub raw: Gap,
        }
        impl UiSignifierStream {
            pub fn write(&mut self, _builder: SignifierEntityBuilder) {}
        }
    }
}

pub mod mod_key {
    use super::*;
    #[derive(Resource, Clone, Copy, Debug, Default)]
    pub struct Modifiers {
        pub raw: Gap,
    }
}

pub mod action_set {
    use super::*;
    #[derive(Resource, Clone, Copy, Debug, Default)]
    pub struct ActionSetStates {
        pub raw: Gap,
    }
    #[derive(Resource, Clone, Copy, Debug, Default)]
    pub struct ActionSetValues {
        pub raw: Gap,
    }
}

pub mod app_loading_state {
    use super::*;
    #[derive(Resource, Clone, Copy, Debug, PartialEq, Eq)]
    pub enum AppLoadingState {
        Loading,
        Done,
    }
}

/// Typed-gap `bevy_app::App` facade (bevy_app 0.16 is absent from the offline registry
/// cache; `add_wall_shape_ux` only registers resources).
#[derive(Default, Debug)]
pub struct App {
    pub raw: Gap,
}
impl App {
    pub fn insert_resource<R: bevy_ecs::resource::Resource>(&mut self, _resource: R) -> &mut Self {
        self
    }
    pub fn init_resource<R: bevy_ecs::resource::Resource + Default>(&mut self) -> &mut Self {
        self
    }
}
