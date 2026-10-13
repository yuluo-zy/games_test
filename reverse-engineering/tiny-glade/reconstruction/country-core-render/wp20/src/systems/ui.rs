//! `systems::ui` — INTEGRATION MARKER facade (wp15-owned real UI domain).
//! Real modules: `reconstruction/country-core/src/systems/ui/`. This gate
//! declares the typed-gap surfaces the WP20 UI systems reference.

#![allow(dead_code, unused_imports)]

use std::sync::Arc;

use crate::Gap;

pub mod resources {
    use std::sync::Arc;

    use crate::Gap;

    pub mod ui_state {
        use std::sync::Arc;

        use crate::Gap;

        /// `WidgetId` (wp15; u64 name-hash).
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
        pub struct WidgetId(pub u64);

        impl WidgetId {
            pub fn from_name(name: &str) -> WidgetId {
                WidgetId(crate::utils::calculate_hash(name))
            }
        }

        /// `HoverState` (wp15).
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
        pub enum HoverState {
            #[default]
            None,
            Hovering,
            Intersecting,
        }

        /// `DragState` (wp15).
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
        pub enum DragState {
            #[default]
            None,
            Started,
            Dragging,
            Finished,
        }

        /// `UiWidgetContext` (wp15; hover/click/active bytes + payload).
        #[derive(Clone, Copy, Debug, Default)]
        pub struct UiWidgetContext {
            pub hover: HoverState,
            pub click: u8,
            pub widget: WidgetId,
            pub intersection: Gap,
        }
        impl UiWidgetContext {
            pub fn clicked(&self) -> bool {
                false
            }
            pub fn hovered(&self) -> bool {
                false
            }
            pub fn intersecting(&self) -> Option<Gap> {
                None
            }
            pub fn id(&self) -> WidgetId {
                self.widget
            }
        }

        /// `UiState` (wp15 widget-registry resource).
        #[derive(bevy_ecs::resource::Resource, Clone, Debug, Default)]
        pub struct UiState {
            pub widgets: Arc<Gap>,
            pub context: UiWidgetContext,
        }
        impl UiState {
            pub fn add(&mut self, _name: &str, _params: Gap) -> WidgetId {
                WidgetId::from_name("")
            }
        }
    }

    pub mod widget_params {
        use crate::Gap;
        /// `WidgetParams` (wp15; 0xb0 B).
        #[derive(Clone, Debug, Default)]
        pub struct WidgetParams {
            pub _gap: Gap,
        }
    }
}

pub mod signifiers {
    use crate::Gap;

    /// `UiAppendSignifier` (O: pushed through
    /// `append_only_vec::AppendOnlyVec` by `ui_camera_mode_gizmos`; layout U).
    #[derive(Clone, Debug, Default)]
    pub struct UiAppendSignifier {
        pub _gap: Gap,
    }
}
