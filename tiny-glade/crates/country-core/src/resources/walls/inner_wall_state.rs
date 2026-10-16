//! Owned B wall authority. Field roles come from the InnerWalls update stores;
//! this representation does not reproduce native Vec headers or Rust ABI.

use crate::recovered_building::{
    wall_collection::RawWallPublicationSource,
    wall_history_edit::{WallAuthoritySnapshot, WallHistoryMetadata},
    wall_state::WallShapeSnapshot,
};
use crate::resources::roofs::geometry::surface::Curve2;

pub const DEFAULT_ARCH_HEIGHT: f32 = f32::from_bits(0x40333333);

/// B 0x1408c5690: ordered equality, including NaN and signed-zero behavior.
pub fn is_default_arch_height(value: f32) -> bool {
    value == DEFAULT_ARCH_HEIGHT
}

#[derive(Clone, Debug)]
pub struct WallState {
    pub(crate) snapshot: WallAuthoritySnapshot,
}

impl WallState {
    /// B 0x14085a370. The demo flag comes from the actual startup owner.
    pub fn new(curve: Curve2, height: f32, style: u8, clutter_version: u8, demo: bool) -> Self {
        let mut shape = [0; 28];
        shape[..4].copy_from_slice(&2u32.to_le_bytes());
        Self::from_snapshot(
            WallAuthoritySnapshot {
                source: RawWallPublicationSource {
                    curve,
                    curve_kind_tag: 2,
                    terrain_floor: 0.0,
                    shape: WallShapeSnapshot::from_observed(shape),
                    style,
                    extra: height,
                    special: false,
                    observed_path_scalar: DEFAULT_ARCH_HEIGHT,
                    clutter_version,
                    observed_source_flag_64: 0,
                },
                metadata: WallHistoryMetadata::for_new_wall(demo),
            },
            0,
        )
    }

    /// Import the real source +66, including older portable snapshots that
    /// omitted it. The metadata becomes its single authoritative storage.
    pub fn from_snapshot(mut snapshot: WallAuthoritySnapshot, freehand_ty: u8) -> Self {
        // The old portable snapshot has two representations of source +65.
        // Preserve its raw byte (historically named clutter_version), deriving
        // the collection predicate so the core never has two mutable owners.
        snapshot.source.special = snapshot.source.clutter_version != 0;
        snapshot.metadata.source_66 = Some(freehand_ty);
        Self { snapshot }
    }

    pub fn snapshot(&self) -> &WallAuthoritySnapshot {
        &self.snapshot
    }

    pub fn into_snapshot(self) -> (WallAuthoritySnapshot, u8) {
        let freehand_ty = self.freehand_ty();
        (self.snapshot, freehand_ty)
    }

    pub fn source(&self) -> &RawWallPublicationSource {
        &self.snapshot.source
    }

    /// Original source +65, copied verbatim to Public +296. The original game
    /// field name is unresolved; it is distinct from freehand type at +66.
    pub fn source_flag_65(&self) -> u8 {
        self.snapshot.source.clutter_version
    }

    pub fn is_special(&self) -> bool {
        self.source_flag_65() != 0
    }

    /// Source +0x54; this is independent of terrain elevation and public max Y.
    pub fn height(&self) -> f32 {
        self.snapshot.source.extra
    }

    pub fn elevation(&self) -> f32 {
        self.snapshot.source.terrain_floor
    }

    /// Source +0x5c, written by 0x140aaa710.
    pub fn arch_height(&self) -> f32 {
        self.snapshot.source.observed_path_scalar
    }

    /// Source +0x66, confirmed from the byte store at 0x140aa9ba7.
    pub fn freehand_ty(&self) -> u8 {
        self.snapshot
            .metadata
            .source_66
            .expect("core wall source +66")
    }

    /// Source +0x60, written as f32 by 0x140aaa6f8.
    pub fn plaster_peel_amount(&self) -> f32 {
        f32::from_bits(self.snapshot.metadata.source_60_bits)
    }

    pub fn arch_type(&self) -> u8 {
        self.snapshot.source.observed_source_flag_64
    }

    pub fn removed_supports(&self) -> u8 {
        self.snapshot.metadata.source_67
    }

    pub fn style(&self) -> u8 {
        self.snapshot.source.style
    }

    pub fn is_premade_shape(&self) -> bool {
        self.snapshot.source.shape.tag() < 2
    }

    /// B 0x1408c56b0 returns the enclosure only for tags below two.
    pub fn get_enclosure_params(&self) -> Option<&WallShapeSnapshot> {
        self.is_premade_shape()
            .then_some(&self.snapshot.source.shape)
    }
}
