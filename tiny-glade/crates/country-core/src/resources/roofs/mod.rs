pub mod commands;
pub mod dormer_tiles;
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
pub use roof_tiles::RoofTile;
pub use roof_trigger_volume::RoofTriggerVolume;
pub use roof_visual_shape::{get_roof_expanded_top_and_bottom, RoofBoundsDims};
