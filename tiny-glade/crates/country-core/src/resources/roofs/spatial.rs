//! B update_roof_spatial's game update after PublicWalls/ECS have resolved state.
//! Library entity tables, queries and event-buffer implementation are excluded.

// Preserve the original instruction-level expression order/NaN policy, API call shapes, and evidence/test transcription. See README.md.
#![allow(clippy::too_many_arguments)]
use crate::resources::roofs::roof_shape as roof_state;
use crate::resources::walls::height_rules::HeightCacheError;
use crate::resources::walls::publication_fields::PublicWallBuildingState;
use crate::resources::walls::publication_fields::WallId;
use crate::resources::walls::publication_fields::WallReadError;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SpatialUpdateError {
    WallRead(WallReadError),
    Height(HeightCacheError),
    InvalidRoofShape,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RoofGeometryChange {
    /// Opaque resolved Bevy entity value, not a handwritten entity implementation.
    pub roof_entity: u64,
    pub wall_id: WallId,
    pub observed_kind: u8,
}
#[derive(Clone, Debug)]
pub struct RoofSpatialOutcome {
    pub style_changed: bool,
    pub shape_changed: bool,
    /// A style and shape change produce two notifications. Do not deduplicate.
    pub geometry_changes: Vec<RoofGeometryChange>,
}

/// Preserves all unobserved bytes. Marks Roof and the related 48-byte component
/// for every resolved update, even when height/style/shape compare unchanged.
pub fn synchronize_resolved_roof(
    roof: &mut [u8; 88],
    related: &mut [u8; 48],
    wall: &PublicWallBuildingState,
    wall_id: WallId,
    roof_entity: u64,
    roof_tick: &mut u32,
    related_tick: &mut u32,
    current_tick: u32,
) -> Result<RoofSpatialOutcome, SpatialUpdateError> {
    wall.validate_read().map_err(SpatialUpdateError::WallRead)?;
    if u32::from_le_bytes(roof[8..12].try_into().unwrap()) > 1 {
        return Err(SpatialUpdateError::InvalidRoofShape);
    }
    let max_y = wall.max_y().map_err(SpatialUpdateError::Height)?;
    *roof_tick = current_tick;
    roof[0x4c..0x50].copy_from_slice(&max_y.to_le_bytes());
    let style = u8::from(wall.is_halftimbered());
    let style_changed = roof[0x54] != style;
    let mut geometry_changes = Vec::new();
    let notification = RoofGeometryChange {
        roof_entity,
        wall_id,
        observed_kind: 2,
    };
    if style_changed {
        roof[0x54] = style;
        geometry_changes.push(notification);
    }
    let shape_changed = wall.shape().synchronize_roof(roof);
    if shape_changed {
        geometry_changes.push(notification);
    }
    let height_01 = f32::from_le_bytes(roof[0x30..0x34].try_into().unwrap());
    let translation = if roof_state::ty_observed(height_01) {
        [0.0; 3]
    } else {
        let tag = u32::from_le_bytes(roof[8..12].try_into().unwrap()) as usize;
        let f = |offset: usize| f32::from_le_bytes(roof[offset..offset + 4].try_into().unwrap());
        [
            f(0x0c + tag * 8),
            max_y + f32::from_bits(0xbf30a3d7),
            f(0x10 + tag * 8),
        ]
    };
    *related_tick = current_tick;
    for (i, value) in translation.iter().enumerate() {
        related[16 + i * 4..20 + i * 4].copy_from_slice(&value.to_le_bytes());
    }
    Ok(RoofSpatialOutcome {
        style_changed,
        shape_changed,
        geometry_changes,
    })
}
