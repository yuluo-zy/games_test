//! systems — non-wall systems/utilities the wall systems reference (typed-gap facades):
//! camera, collision world, mod keys, garden/terrain raster replay hooks.

use bevy_ecs::prelude::Resource;

use crate::typed_gap::Gap;

pub mod camera_rig {
    use super::*;
    #[derive(Resource, Clone, Copy, Debug, Default)]
    pub struct MainCamera {
        pub raw: Gap,
    }
    impl MainCamera {
        pub fn eye_position(&self) -> glam::Vec3 {
            glam::Vec3::ZERO
        }
    }
}

pub mod collision {
    pub mod world {
        use crate::typed_gap::Gap;

        #[derive(bevy_ecs::prelude::Resource, Clone, Debug, Default)]
        pub struct RaycastWorld {
            pub raw: Gap,
        }
        impl RaycastWorld {
            pub fn raycast_simple(&self, _from: glam::Vec3, _to: glam::Vec3) -> Option<f32> {
                None
            }
            pub fn legacy_raycast(&self, _from: glam::Vec3, _to: glam::Vec3) -> Option<f32> {
                None
            }
        }
        #[derive(Clone, Debug, Default)]
        pub struct Rapier {
            pub raw: Gap,
        }
        impl Rapier {
            pub fn legacy_raycast(&self, _from: glam::Vec3, _to: glam::Vec3) -> Option<f32> {
                None
            }
        }
    }
}

pub mod mod_key {
    pub use crate::ui::mod_key::*;
}

pub mod garden {
    use super::*;
    #[derive(Clone, Copy, Debug)]
    pub struct GardenDrawEvent;
    pub mod record_garden_history {
        use super::*;
        #[derive(Clone, Copy, Debug)]
        pub struct RecordGardenEditToHistoryCmd;
    }
}

pub mod world_raster {
    use super::*;
    #[derive(Clone, Debug, Default)]
    pub struct RasterHistoryRecordResources {
        pub raw: Gap,
    }
}

pub mod history {
    pub mod replay_raster {
        use crate::typed_gap::Gap;
        #[derive(Clone, Debug, Default)]
        pub struct RasterReplayResources {
            pub raw: Gap,
        }
    }
    pub use crate::resources::history::*;
}

pub mod ui {
    pub mod resources {
        pub mod ui_state {
            pub use crate::ui::ui_state::*;
        }
    }
    pub mod signifiers {
        pub use crate::ui::signifiers::*;
    }
}

pub mod audio {
    pub use crate::audio::*;
}

pub mod terrain {
    pub mod delayed_terrain_changed_msg {
        use bevy_ecs::prelude::Resource;
        use crate::typed_gap::Gap;
        #[derive(Resource, Clone, Debug, Default)]
        pub struct ScheduledTerrainReadback {
            pub raw: Gap,
        }
    }
    pub mod draw_terrain_stroke {
        use super::*;
        #[derive(Clone, Copy, Debug)]
        pub struct EvTerrainStrokeChanged;
    }
}

pub mod render_loop {
    pub mod compute_passes {
        pub mod roof_tiles_gravity {
            use crate::typed_gap::Gap;
            #[derive(Clone, Copy, Debug)]
            pub struct PushConstants {
                pub raw: Gap,
            }
        }
    }
}

pub mod mode_manager {
    pub use crate::ui::mode_manager::*;
}

pub mod input {
    pub mod action_set {
        pub use crate::ui::action_set::*;
    }
}

pub mod app_loading_state {
    pub use crate::ui::app_loading_state::*;
}

pub mod startup {
    pub mod init_photomode_state {
        use bevy_ecs::prelude::Resource;
        use crate::typed_gap::Gap;
        #[derive(Resource, Clone, Debug, Default)]
        pub struct PhotomodeState {
            pub raw: Gap,
        }
    }
}

pub mod ui_raise_shape {
    use super::*;
    #[derive(Clone, Copy, Debug)]
    pub struct HandleRaiseShapeUiInteraction;
}
