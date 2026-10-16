pub mod commands;
pub mod dormer_tiles;
pub mod geometry;
pub mod roof_animation;
pub mod roof_buffers;
pub mod roof_noise;
pub mod roof_ridge;
pub mod roof_shape;
pub mod roof_tile_sim;
pub mod roof_tiles;
pub mod roof_trigger_volume;
pub mod roof_visual_shape;

pub use dormer_tiles::DormerTiles;
pub use roof_animation::{RoofAnimation, RoofMarkedForDeletion};
pub use roof_buffers::RoofBuffers;
pub use roof_noise::{apply_roof_noise, noise, revert_roof_noise};
pub use roof_ridge::{
    calc_roof_tip_center_ls, inverse_ridge_01_param, RoofRidgeDims, RIDGE_MIN,
};
pub use roof_shape::{
    ridge_length_01_from_rectangle_ratio, roof_height_01_from_wall_shape,
    roof_height_ws_from_wall_shape, FlatRoofStyle, GableRoofStyle, Roof,
    RoofIntersectionSection, RoofShapeParams,
};
pub use roof_tile_sim::RoofTileSimulation;
pub use roof_tiles::{RoofTile, RoofTiles};
pub use roof_trigger_volume::RoofTriggerVolume;
pub use roof_visual_shape::{get_roof_expanded_top_and_bottom, RoofBoundsDims};

/// 屋顶事件注册（原 `systems/roof` 的注册面；事件类型属于 core，
/// 由 system-roof 的 `add_roof_events_and_resources`（0x1406305f0）调用）。
pub fn register_roof_events(world: &mut bevy_ecs::world::World) {
    use bevy_ecs::event::Events;
    world.init_resource::<Events<commands::CreateRoofCmd>>();
    world.init_resource::<Events<commands::DeleteRoofCmd>>();
    world.init_resource::<Events<commands::SwitchRoofDirectionCmd>>();
    world.init_resource::<Events<commands::SetFlatRoofStyleCmd>>();
    world.init_resource::<Events<commands::SetFlatRoofRimStyleCmd>>();
    world.init_resource::<Events<commands::EditRoofCmd>>();
    world.init_resource::<Events<commands::RoofIsInvalid>>();
    world.init_resource::<Events<commands::ResetRoofs>>();
    world.init_resource::<Events<commands::WallRoofPresenceChanged>>();
    world.init_resource::<Events<commands::WallGableChanged>>();
    world.init_resource::<Events<commands::RoofGeometryChanged>>();
    world.init_resource::<Events<commands::RecalculateRoofCmd>>();
    world.init_resource::<Events<commands::RecalculateWallHoles>>();
    world.init_resource::<Events<commands::RecalculateWallBricks>>();
    world.init_resource::<Events<commands::SwitchRoofTypeCmd>>();
    world.init_resource::<Events<commands::RoofSound>>();
    world.init_resource::<Events<commands::RecordRoofHistory>>();
    world.init_resource::<Events<commands::RoofEditFeedback>>();
    world.init_resource::<Events<commands::ReplayRoofEvent>>();
}
