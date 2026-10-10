//! Transient control geometry and fixed gesture planes; no domain mutations.
use bevy::prelude::*;
use garden_application::tools::{GroundPoint, Tool};
use garden_domain::BlockId;
use garden_generation::BuildingLayout;

pub const HIT_RADIUS: f32 = 16.; // Logical viewport pixels, independent of DPI.

#[derive(Default, Reflect, GizmoConfigGroup)]
pub struct ControlGizmos;
pub fn setup_gizmos(mut configs: ResMut<GizmoConfigStore>) {
    configs.config_mut::<ControlGizmos>().0.depth_bias = -1.;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ControlKind {
    Width,
    Depth,
    Height,
    Rotate,
}
impl ControlKind {
    pub fn tool(self) -> Tool {
        match self {
            Self::Width | Self::Depth => Tool::Resize,
            Self::Height => Tool::Height,
            Self::Rotate => Tool::Rotate,
        }
    }
    pub fn color(self) -> Color {
        match self {
            Self::Width => Color::srgb_u8(244, 157, 74),
            Self::Depth => Color::srgb_u8(83, 177, 235),
            Self::Height => Color::srgb_u8(135, 220, 116),
            Self::Rotate => Color::srgb_u8(197, 132, 240),
        }
    }
}
#[derive(Debug, Clone, Copy)]
pub struct Control {
    pub kind: ControlKind,
    pub position: Vec3,
    pub anchor: Vec3,
}

pub fn controls(
    layout: &BuildingLayout,
    transform: &Transform,
    block: BlockId,
    tool: Tool,
) -> Vec<Control> {
    if matches!(
        tool,
        Tool::Build
            | Tool::Move
            | Tool::MoveBlock
            | Tool::Pan
            | Tool::Wall
            | Tool::Path
            | Tool::StrokeSelect
    ) {
        return Vec::new();
    }
    let Some(b) = layout.blocks.iter().find(|b| b.block == block) else {
        return Vec::new();
    };
    let r = b.footprint;
    let x = (r.x + r.width * 0.5) as f32;
    let z = (r.z + r.depth * 0.5) as f32;
    let middle = (b.base_elevation + b.height * 0.45) as f32;
    let top = (b.base_elevation + b.height) as f32;
    // Rotation is aggregate-scoped and always pivots around the base center.
    let root = layout
        .blocks
        .iter()
        .min_by(|a, b| a.base_elevation.total_cmp(&b.base_elevation))
        .unwrap();
    let root_r = root.footprint;
    let center = Vec3::new(
        (root_r.x + root_r.width * 0.5) as f32,
        root.base_elevation as f32 + 0.15,
        (root_r.z + root_r.depth * 0.5) as f32,
    );
    let radius = (root_r.width.max(root_r.depth) * 0.5) as f32 + 1.;
    let handles = [
        Control {
            kind: ControlKind::Width,
            position: Vec3::new(r.right() as f32 + 0.65, middle, z),
            anchor: Vec3::new(r.right() as f32, middle, z),
        },
        Control {
            kind: ControlKind::Depth,
            position: Vec3::new(x, middle, r.back() as f32 + 0.65),
            anchor: Vec3::new(x, middle, r.back() as f32),
        },
        Control {
            kind: ControlKind::Height,
            position: Vec3::new(r.right() as f32 + 0.65, top + 0.8, z),
            anchor: Vec3::new(r.right() as f32 + 0.65, top, z),
        },
        Control {
            kind: ControlKind::Rotate,
            position: center - Vec3::Z * radius,
            anchor: center,
        },
    ];
    let matrix = transform.compute_affine();
    handles
        .into_iter()
        .filter(|h| tool == Tool::Select || h.kind.tool() == tool)
        .map(|h| Control {
            position: matrix.transform_point3(h.position),
            anchor: matrix.transform_point3(h.anchor),
            ..h
        })
        .collect()
}

pub fn hit_control(
    camera: &Camera,
    view: &Transform,
    cursor: Vec2,
    controls: &[Control],
) -> Option<Control> {
    let view = GlobalTransform::from(*view);
    controls
        .iter()
        .filter_map(|h| {
            let p = camera.world_to_viewport(&view, h.position).ok()?;
            let distance = p.distance_squared(cursor);
            (distance <= HIT_RADIUS * HIT_RADIUS).then_some((distance, *h))
        })
        .min_by(|a, b| a.0.total_cmp(&b.0))
        .map(|(_, h)| h)
}

#[derive(Debug, Clone, Copy)]
pub struct DragPlane {
    origin: Vec3,
    normal: Vec3,
    anchor: Vec3,
}
impl DragPlane {
    pub fn begin(ray: Ray3d, origin: Vec3, normal: Vec3) -> Option<Self> {
        let mut plane = Self {
            origin,
            normal: normal.try_normalize()?,
            anchor: Vec3::ZERO,
        };
        plane.anchor = plane.intersection(ray)?;
        Some(plane)
    }
    pub fn intersection(self, ray: Ray3d) -> Option<Vec3> {
        let denominator = self.normal.dot(*ray.direction);
        if denominator.abs() < 1e-5 {
            return None;
        }
        let t = self.normal.dot(self.origin - ray.origin) / denominator;
        (t.is_finite() && t >= 0.)
            .then(|| ray.get_point(t))
            .filter(|p| p.is_finite())
    }
    pub fn height_delta(self, ray: Ray3d) -> Option<f64> {
        self.intersection(ray)
            .map(|p| f64::from(p.y - self.anchor.y))
    }
    pub fn ground(self, ray: Ray3d) -> Option<GroundPoint> {
        self.intersection(ray).map(|p| GroundPoint {
            x: f64::from(p.x),
            z: f64::from(p.z),
        })
    }
}

pub fn draw_controls(
    gizmos: &mut Gizmos<ControlGizmos>,
    camera: &Camera,
    view: &Transform,
    controls: &[Control],
) {
    let global = GlobalTransform::from(*view);
    for h in controls {
        let Ok(center) = camera.world_to_viewport(&global, h.position) else {
            continue;
        };
        let Ok(unit) = camera.world_to_viewport(&global, h.position + *view.right()) else {
            continue;
        };
        let radius = (10. / center.distance(unit).max(0.01)).clamp(0.03, 2.);
        gizmos.line(h.anchor, h.position, h.kind.color());
        gizmos.circle(
            Isometry3d::new(h.position, view.rotation),
            radius,
            h.kind.color(),
        );
        if h.kind == ControlKind::Rotate {
            gizmos.circle(
                Isometry3d::new(h.anchor, Quat::from_rotation_x(std::f32::consts::FRAC_PI_2)),
                h.anchor.distance(h.position),
                h.kind.color(),
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn handle_hit_radius_is_in_logical_pixels_at_both_dpi_scales() {
        let view = Transform::from_xyz(8., 12., 16.).looking_at(Vec3::ZERO, Vec3::Y);
        let handle = Control {
            kind: ControlKind::Width,
            position: Vec3::ZERO,
            anchor: Vec3::X,
        };
        let mut pixels = Vec::new();
        for scale in [1., 2.] {
            let mut camera = Camera::default();
            camera.computed.clip_from_view =
                Mat4::perspective_infinite_reverse_rh(std::f32::consts::PI / 3., 1.5, 0.1);
            camera.computed.target_info = Some(bevy::camera::RenderTargetInfo {
                physical_size: UVec2::new((1440. * scale) as u32, (960. * scale) as u32),
                scale_factor: scale,
            });
            let p = camera
                .world_to_viewport(&GlobalTransform::from(view), handle.position)
                .unwrap();
            pixels.push(p);
            assert_eq!(
                hit_control(&camera, &view, p + Vec2::X * 15., &[handle])
                    .unwrap()
                    .kind,
                ControlKind::Width
            );
            assert!(hit_control(&camera, &view, p + Vec2::X * 17., &[handle]).is_none());
        }
        assert!(pixels[0].distance(pixels[1]) < 0.01);
    }
    #[test]
    fn block_handles_follow_display_pose_and_unrelated_tools_have_none() {
        use garden_domain::{Building, BuildingId, sample_building};
        let building =
            Building::try_new(sample_building(BuildingId::new(1).unwrap(), 8., 3.)).unwrap();
        let layout = garden_generation::compile(&building);
        let block = building.blocks()[0].id;
        let pose = Transform::from_xyz(4., 2., -3.).with_rotation(Quat::from_rotation_y(1.));
        let local = controls(&layout, &Transform::IDENTITY, block, Tool::Select);
        let world = controls(&layout, &pose, block, Tool::Select);
        assert_eq!(world.len(), 4);
        for (a, b) in local.iter().zip(world.iter()) {
            assert!(pose.transform_point(a.position).distance(b.position) < 1e-5);
        }
        assert!(controls(&layout, &pose, block, Tool::Build).is_empty());
        assert_eq!(controls(&layout, &pose, block, Tool::Height).len(), 1);
    }
    #[test]
    fn fixed_plane_height_uses_world_meters_and_rejects_parallel_rays() {
        let start = Ray3d::new(Vec3::new(0., 3., 10.), Dir3::NEG_Z);
        let plane = DragPlane::begin(start, Vec3::ZERO, Vec3::Z).unwrap();
        let moved = Ray3d::new(Vec3::new(0., 5., 10.), Dir3::NEG_Z);
        assert_eq!(plane.height_delta(moved), Some(2.));
        assert!(
            plane
                .intersection(Ray3d::new(Vec3::ZERO, Dir3::X))
                .is_none()
        );
        assert!(plane.intersection(Ray3d::new(Vec3::Z, Dir3::Z)).is_none());
    }
}
