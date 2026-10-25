//! Owned CPU fields of B PublicWallState. Container headers and GPU handles are
//! not reconstructed. Publication, path preparation and height consumers share
//! this record; its projection is only the previously identified byte fields.

use glam::Vec2;
use utils::geometry::{Circle2d, Rectangle2d};

use super::WallState;
use crate::resources::walls::decorator::anchors::DecoratorAnchorContext;
use crate::resources::walls::decorator::anchors::DecoratorAnchorError;
use crate::resources::walls::decorator::anchors::wall_attachment_height;
use crate::resources::walls::freeform_height_cache::FreeformWallHeightCache;
use crate::resources::walls::freeform_height_cache::freeform_wall_height_cache;
use crate::resources::walls::gable_heights::GableArchHeights;
use crate::resources::walls::gable_heights::GableError;
use crate::resources::walls::gable_heights::apply_gable_arch_heights;
use crate::resources::walls::height_cache::RegularWallHeightCache;
use crate::resources::walls::height_cache::regular_wall_height_cache;
use crate::resources::walls::publication_fields::PublicWallBuildingState;
use crate::resources::walls::publication_fields::WallShapeSnapshot;
use crate::resources::walls::publication_fields::validate_public_wall_read;
use crate::resources::walls::publication_source::CollectedWallPublication;
use crate::resources::walls::publication_source::CollectionError;
use crate::resources::walls::publication_source::TerrainHeightGrid512;
use crate::resources::walls::publication_source::collect_raw_wall_publication;
use crate::systems::wall::wall_state::path_preparation::PathPreparationError;
use crate::systems::wall::wall_state::path_preparation::PathRasters512;
use crate::systems::wall::wall_state::path_preparation::PreparedWallPath;
use crate::systems::wall::wall_state::path_preparation::prepare_wall_path;
use crate::systems::wall::wall_state::publication_policy::WallPublicationCore;

const MISSING_HEIGHT: &str = "wall heights have not been calculated yet. Perhaps you are trying to access it before `init_wall_height_and_arch_segment_height` system had a chance to run?";
const MISSING_GABLE_HEIGHT: &str = "`max_wall_y_gable` has not been calculated yet. Perhaps you are trying to access it before `init_wall_height_and_arch_segment_height` system had a chance to run?";

#[derive(Clone, Debug)]
pub struct PublicWallState {
    pub material_stats: super::material_stats::MaterialStats,
    /// Original +110/+114 readiness and scalar. Distinct from optional profile
    /// geometry: a correctly initialized no-roof wall still owns a base value.
    gable_maximum_y: Option<f32>,
    pub authority: WallState,
    pub geometry: CollectedWallPublication,
    pub regular_heights: Option<RegularWallHeightCache>,
    pub prepared_path: Option<PreparedWallPath>,
    pub freeform_heights: Option<FreeformWallHeightCache>,
    /// Original independent third array +f0/+f8/+100, initialized with the
    /// prepared segment-owner count (+d0), not the source point count.
    pub segment_heights: Vec<f32>,
    pub gable_heights: Option<GableArchHeights>,
    /// Derived at publication, from the actual bulldoze animation queue.
    being_bulldozed: bool,
}

impl PublicWallState {
    pub fn is_being_bulldozed(&self) -> bool {
        self.being_bulldozed
    }

    /// The publication owner captures this value when rebuilding the row.
    /// UI consumers must not substitute the later queue's current membership.
    pub(crate) fn set_being_bulldozed(&mut self, value: bool) {
        self.being_bulldozed = value;
    }

    pub fn collect(
        authority: &WallState,
        terrain: &TerrainHeightGrid512,
    ) -> Result<Self, CollectionError> {
        let geometry = collect_raw_wall_publication(authority.source(), terrain)?;
        let regular_heights = regular_wall_height_cache(
            &geometry.outline,
            &authority.source().shape,
            geometry.metrics.constrained_extra,
        )
        .ok();
        Ok(Self {
            material_stats: super::material_stats::MaterialStats::default(),
            gable_maximum_y: None,
            authority: authority.clone(),
            geometry,
            regular_heights,
            prepared_path: None,
            freeform_heights: None,
            segment_heights: Vec::new(),
            gable_heights: None,
            being_bulldozed: false,
        })
    }

    /// The source Curve2 is the field tested by PublicWalls::get, not the
    /// terrain-mapped Curve3 nor the later path-intersection curve.
    pub fn simplified_curve_2d(&self) -> &crate::resources::roofs::geometry::surface::Curve2 {
        &self.authority.source().curve
    }

    pub fn assert_valid(&self) {
        let curve = self.simplified_curve_2d();
        assert!(
            validate_public_wall_read(curve.points.len() as u64, curve.length).is_ok(),
            "assertion failed: w.simplified_curve_2d.is_valid()"
        );
    }

    /// Consume the four real current path buffers. A failed preparation leaves
    /// no freehand height cache from a previous revision.
    pub fn prepare_paths(&mut self, paths: &PathRasters512) -> Result<(), PathPreparationError> {
        self.prepared_path = None;
        self.freeform_heights = None;
        self.gable_heights = None;
        self.gable_maximum_y = None;
        self.segment_heights.clear();
        let prepared = prepare_wall_path(
            &self.geometry.outline,
            &self.geometry.relative_heights,
            self.authority.arch_height(),
            paths,
        )?;
        if self.shape().tag() >= 2 {
            self.freeform_heights = Some(
                freeform_wall_height_cache(
                    &prepared.positions,
                    self.shape(),
                    self.geometry.metrics.constrained_extra,
                    self.authority.freehand_ty(),
                    prepared.segment_values.len(),
                )
                .expect("freehand shape selected above"),
            );
        } else {
            self.regular_heights = Some(
                regular_wall_height_cache(
                    &prepared.positions,
                    self.shape(),
                    self.geometry.metrics.constrained_extra,
                )
                .expect("premade shape selected above"),
            );
        }
        self.segment_heights =
            crate::systems::wall::construction::parameters::initialize_segment_height_array(
                prepared.segment_values.len(),
            );
        self.prepared_path = Some(prepared);
        Ok(())
    }

    pub fn shape(&self) -> &WallShapeSnapshot {
        &self.authority.source().shape
    }
    pub fn is_premade_shape(&self) -> bool {
        self.shape().tag() < 2
    }
    pub fn try_get_wall_shape(&self) -> Option<&WallShapeSnapshot> {
        self.is_premade_shape().then(|| self.shape())
    }
    pub fn get_wall_shape(&self) -> &WallShapeSnapshot {
        self.try_get_wall_shape().unwrap()
    }

    /// B 0x140b2a740 / 0x140b2a770 use tag != 2, not tag < 2.
    pub fn wall_space(&self) -> Option<f32> {
        (self.shape().tag() != 2).then_some(self.wall_space_length())
    }
    pub fn wall_space_length(&self) -> f32 {
        self.geometry.original_curve_length
    }
    pub fn u_space(&self) -> Option<f32> {
        (self.shape().tag() != 2).then_some(1.0)
    }

    fn shape_float(&self, index: usize) -> f32 {
        let offset = 4 + index * 4;
        f32::from_le_bytes(self.shape().bytes()[offset..offset + 4].try_into().unwrap())
    }
    pub fn is_circle(&self) -> Option<Circle2d> {
        (self.shape().tag() == 0).then(|| Circle2d {
            center: Vec2::new(self.shape_float(0), self.shape_float(1)),
            radius: self.shape_float(2),
            rotation: self.shape_float(3),
        })
    }
    pub fn is_rectangle(&self) -> Option<Rectangle2d> {
        (self.shape().tag() == 1).then(|| Rectangle2d {
            axis: Vec2::new(self.shape_float(0), self.shape_float(1)),
            center: Vec2::new(self.shape_float(2), self.shape_float(3)),
            size: Vec2::new(self.shape_float(4), self.shape_float(5)),
        })
    }
    pub fn corner_pos_and_normal_xz(&self, corner: usize) -> Option<(Vec2, Vec2)> {
        self.is_rectangle().map(|rectangle| {
            (
                rectangle.get_point(corner),
                rectangle.get_corner_normal(corner),
            )
        })
    }

    pub fn calculated_max_y(&self) -> Option<f32> {
        self.regular_heights
            .as_ref()
            .map(|h| h.maximum_y)
            .or_else(|| self.freeform_heights.as_ref().map(|h| h.maximum_y))
    }
    pub fn max_y(&self) -> f32 {
        self.calculated_max_y().expect(MISSING_HEIGHT)
    }
    /// 0x140b2a680（fresh disasm 2026-10-17）：`TEST byte[RCX+0x110],0x1;
    /// JZ panic_fmt` —— gable 高度未就绪时 **panic**（不回退 max_y）。
    pub fn max_y_gable(&self) -> f32 {
        self.gable_maximum_y.expect(MISSING_GABLE_HEIGHT)
    }
    pub fn flat_roof_y(&self) -> f32 {
        self.max_y() - 0.79
    }
    pub fn height_outline(&self) -> &[[f32; 3]] {
        self.prepared_path
            .as_ref()
            .map(|p| p.positions.as_slice())
            .unwrap_or(&self.geometry.outline)
    }
    pub fn arch_segment_heights(&self) -> Option<&[f32]> {
        self.gable_heights
            .as_ref()
            .map(|h| h.heights.as_slice())
            .or_else(|| {
                self.regular_heights
                    .as_ref()
                    .map(|h| h.arch_segment_heights.as_slice())
            })
            .or_else(|| {
                self.freeform_heights
                    .as_ref()
                    .map(|h| h.arch_segment_heights.as_slice())
            })
    }

    /// Execute the recovered arch algorithms against the authoritative prepared
    /// path, cached arch heights, and independent third height array.
    pub fn construct_arches(
        &mut self,
        wall_id: super::WallId,
        masks: &[crate::systems::wall::construction::segmentation::WallPathSegmentationMask],
        has_roof: bool,
    ) -> Result<
        crate::systems::wall::construction::arches::WallArchesOutput,
        crate::systems::wall::construction::arches::WallArchesError,
    > {
        use crate::systems::wall::construction::arches::*;
        use crate::systems::wall::construction::freehand_arches::*;
        let projection = self.building_projection();
        let path = self
            .prepared_path
            .as_ref()
            .ok_or(WallArchesError::InvalidInput)?;
        if projection.shape().tag() < 2 {
            let arch = if let Some(gable) = self.gable_heights.as_mut() {
                &mut gable.heights
            } else {
                &mut self
                    .regular_heights
                    .as_mut()
                    .ok_or(WallArchesError::InvalidInput)?
                    .arch_segment_heights
            };
            construct_regular_wall_arches(
                RegularWallArchesInput {
                    masks,
                    prepared_positions: &path.positions,
                    prepared_u: &path.points_u,
                    path_values: &path.segment_values,
                    shape_tag: projection.shape().tag(),
                    public_284: projection.float_0x284(),
                    observed_294: projection.observed_294(),
                    source_curve_length: self.authority.source().curve.length,
                    wall_id: crate::resources::walls::publication_fields::WallId(wall_id.0),
                    has_roof,
                },
                arch,
                &mut self.segment_heights,
            )
        } else {
            let arch = &mut self
                .freeform_heights
                .as_mut()
                .ok_or(WallArchesError::InvalidInput)?
                .arch_segment_heights;
            construct_freehand_wall_arches(
                FreehandWallArchesInput {
                    masks,
                    prepared_positions: &path.positions,
                    prepared_u: &path.points_u,
                    path_values: &path.segment_values,
                    public_284: projection.float_0x284(),
                    public_288: projection.float_0x288(),
                    observed_294: projection.observed_294(),
                    clutter_version: self.authority.snapshot().metadata.source_69,
                    source_curve_length: self.authority.source().curve.length,
                    wall_id: crate::resources::walls::publication_fields::WallId(wall_id.0),
                    has_roof,
                },
                arch,
                &mut self.segment_heights,
            )
        }
    }

    pub fn update_gable_heights(&mut self, roof: Option<&[u8; 88]>) -> Result<(), GableError> {
        self.gable_heights = None;
        // Original78bc70..bc96 writes both ready flags and the base scalar
        // before it even tests rectangle/roof. Never fallback in the getter.
        self.gable_maximum_y = Some(self.calculated_max_y().expect(MISSING_HEIGHT));
        if self.is_rectangle().is_some()
            && let (Some(base), Some(roof)) = (&self.regular_heights, roof)
        {
            let gable = apply_gable_arch_heights(
                self.height_outline(),
                &base.arch_segment_heights,
                base.maximum_y,
                self.wall_space_length(),
                self.authority.style() >= 2,
                roof,
            )?;
            self.gable_maximum_y = Some(gable.maximum_y);
            self.gable_heights = Some(gable);
        }
        Ok(())
    }

    pub fn get_wall_attachment_height(&self, u: f32) -> Result<f32, DecoratorAnchorError> {
        let public = self.building_projection();
        let heights = self
            .arch_segment_heights()
            .ok_or(DecoratorAnchorError::MissingHeightCache)?;
        wall_attachment_height(
            &DecoratorAnchorContext {
                raw_curve: self.simplified_curve_2d(),
                mapped_outline: self.height_outline(),
                arch_heights: heights,
                public: &public,
            },
            u,
        )
    }

    /// Compatibility with the independently recovered roof/math functions.
    /// Missing height caches retain absent flags; no successful value is invented.
    pub fn building_projection(&self) -> PublicWallBuildingState {
        let mut bytes = [0; 672];
        WallPublicationCore {
            source_point_count: self.simplified_curve_2d().points.len() as u64,
            source_curve_length: self.simplified_curve_2d().length,
            shape: self.shape(),
            style: self.authority.style(),
            metrics: self.geometry.metrics,
        }
        .write_projection(&mut bytes);
        bytes[0x295] = self.authority.freehand_ty();
        bytes[0x296] = self.authority.source_flag_65();
        bytes[0x297] = self.being_bulldozed as u8;
        if let Some(base) = self.calculated_max_y() {
            bytes[0x108..0x10c].copy_from_slice(&1u32.to_le_bytes());
            bytes[0x10c..0x110].copy_from_slice(&base.to_le_bytes());
        }
        if let Some(gable) = self.gable_maximum_y {
            bytes[0x110..0x114].copy_from_slice(&1u32.to_le_bytes());
            bytes[0x114..0x118].copy_from_slice(&gable.to_le_bytes());
        }
        PublicWallBuildingState::from_observed(bytes)
    }
}

#[cfg(test)]
mod initialization_tests {
    use super::*;
    #[test]
    fn no_roof_gable_maximum_has_explicit_ready_reset_lifecycle() {
        use crate::resources::roofs::geometry::surface::Curve2;
        use std::sync::Arc;
        let authority = WallState::new(
            Curve2::try_new(vec![[0., 0.], [8., 0.]], false).unwrap(),
            3.,
            0,
            2,
            false,
        );
        let terrain = TerrainHeightGrid512 {
            encoded: vec![0; 512 * 512].into(),
            minimum_y: 0.,
            maximum_y: 0.,
            sampling_fix: false,
        };
        let paths = PathRasters512::from_encoded(
            std::array::from_fn(|_| Arc::<[u8]>::from(vec![0; 512 * 512])),
            false,
        )
        .unwrap();
        let mut wall = PublicWallState::collect(&authority, &terrain).unwrap();
        wall.prepare_paths(&paths).unwrap();
        assert!(
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| wall.max_y_gable())).is_err()
        );
        assert!(wall.building_projection().max_y_gable().is_err());
        wall.update_gable_heights(None).unwrap();
        assert_eq!(wall.max_y_gable().to_bits(), wall.max_y().to_bits());
        assert_eq!(
            wall.building_projection().max_y_gable().unwrap().to_bits(),
            wall.max_y_gable().to_bits()
        );
        assert!(wall.gable_heights.is_none());
        wall.prepare_paths(&paths).unwrap();
        assert!(wall.building_projection().max_y_gable().is_err());
        assert!(
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| wall.max_y_gable())).is_err()
        );
        wall.update_gable_heights(None).unwrap();
        assert_eq!(wall.max_y_gable().to_bits(), wall.max_y().to_bits());
    }
}
