//! Original B collision DirtyTiles rectangle policy. The type is the one
//! original `country_core::utils::dirty_tiles::DirtyTiles`; the collision code
//! keeps its historical `DirtyTiles64` name for the same 64x64 bit grid.

pub use crate::utils::dirty_tiles::{DirtyTileRect, DirtyTiles as DirtyTiles64, world_rect_cells};
