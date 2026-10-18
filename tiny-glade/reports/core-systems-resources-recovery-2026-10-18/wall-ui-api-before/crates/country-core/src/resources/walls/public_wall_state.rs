//! Owned CPU fields of B PublicWallState. Container headers and GPU handles are
//! not reconstructed. Publication, path preparation and height consumers share
//! this record; its projection is only the previously identified byte fields.

use glam::Vec2;
use utils::geometry::{Circle2d, Rectangle2d};

use super::WallState;
use crate::resources::walls::decorator::anchors::DecoratorAnchorContext;
use crate::resources::walls::decorator::anchors::DecoratorAnchorError;
use crate::resources::walls::decorator::anchors::wall_attachment_height;
use crate::resources::walls::height_cache::RegularWallHeightCache;
use crate::resources::walls::height_cache::regular_wall_height_cache;
use crate::resources::walls::publication_source::CollectedWallPublication;
use crate::resources::walls::publication_source::CollectionError;
use crate::resources::walls::publication_source::TerrainHeightGrid512;
use crate::resources::walls::publication_source::collect_raw_wall_publication;
use crate::resources::walls::freeform_height_cache::FreeformWallHeightCache;
use crate::resources::walls::freeform_height_cache::freeform_wall_height_cache;
use crate::resources::walls::gable_heights::GableArchHeights;
use crate::resources::walls::gable_heights::GableError;
use crate::resources::walls::gable_heights::apply_gable_arch_heights;
use crate::systems::wall::wall_state::path_preparation::PathPreparationError;
use crate::systems::wall::wall_state::path_preparation::PathRasters512;
use crate::systems::wall::wall_state::path_preparation::PreparedWallPath;
use crate::systems::wall::wall_state::path_preparation::prepare_wall_path;
use crate::systems::wall::wall_state::publication_policy::WallPublicationCore;
use crate::resources::walls::publication_fields::PublicWallBuildingState;
use crate::resources::walls::publication_fields::WallShapeSnapshot;
use crate::resources::walls::publication_fields::validate_public_wall_read;

const MISSING_HEIGHT: &str = "wall heights have not been calculated yet. Perhaps you are trying to access it before `init_wall_height_and_arch_segment_height` system had a chance to run?";
const MISSING_GABLE_HEIGHT: &str = "`max_wall_y_gable` has not been calculated yet. Perhaps you are trying to access it before `init_wall_height_and_arch_segment_height` system had a chance to run?";

#[derive(Clone, Debug)]
pub struct PublicWallState {
    pub authority: WallState,
    pub geometry: CollectedWallPublication,
    pub regular_heights: Option<RegularWallHeightCache>,
    pub prepared_path: Option<PreparedWallPath>,
    pub freeform_heights: Option<FreeformWallHeightCache>,
    /// Original independent third array +f0/+f8/+100, initialized with the
    /// prepared segment-owner count (+d0), not the source point count.
    pub segment_heights: Vec<f32>,
    pub gable_heights: Option<GableArchHeights>,
}

impl PublicWallState {
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
            authority: authority.clone(),
            geometry,
            regular_heights,
            prepared_path: None,
            freeform_heights: None,
            segment_heights: Vec::new(),
            gable_heights: None,
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
        self.gable_heights
            .as_ref()
            .expect(MISSING_GABLE_HEIGHT)
            .maximum_y
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

    pub fn update_gable_heights(&mut self, roof: Option<&[u8; 88]>) -> Result<(), GableError> {
        self.gable_heights = None;
        if let (Some(base), Some(roof)) = (&self.regular_heights, roof) {
            self.gable_heights = Some(apply_gable_arch_heights(
                self.height_outline(),
                &base.arch_segment_heights,
                base.maximum_y,
                self.wall_space_length(),
                self.authority.style() >= 2,
                roof,
            )?);
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
        if let Some(base) = self.calculated_max_y() {
            bytes[0x108..0x10c].copy_from_slice(&1u32.to_le_bytes());
            bytes[0x10c..0x110].copy_from_slice(&base.to_le_bytes());
            bytes[0x110..0x114].copy_from_slice(&1u32.to_le_bytes());
            let gable = self
                .gable_heights
                .as_ref()
                .map(|h| h.maximum_y)
                .unwrap_or(base);
            bytes[0x114..0x118].copy_from_slice(&gable.to_le_bytes());
        }
        PublicWallBuildingState::from_observed(bytes)
    }
}
