//! B hover_intersection@1408383a0 and its game closure in host140ac3f50.
//! Native containers/tasks are reused; all eight volume discriminants, closest
//! element selection, world depth rejection and stable priority are preserved.
use super::resources::{
    ui_state::{UiState, WidgetId},
    widget_params::{WidgetGeometry, WidgetParams},
};
use crate::{
    resources::{
        cursor_terrain_raycast::CursorTerrainRaycast, mouse_terrain_raycast::CursorPositionSS,
        read_mouse_depth_value::MouseDepthValue, window::WindowSize,
    },
    systems::camera_rig::MainCamera,
};
use bevy_ecs::prelude::*;
use bevy_tasks::{ComputeTaskPool, ParallelSlice};
use glam::{Mat4, Vec2, Vec3};
use std::sync::Arc;
use utils::geometry::intersect::{Sphere, ray_sphere_intersection, ray_triangle_intersection};
use utils::geometry::{Plane, Ray, Rectangle2d};

/// Native WidgetIntersection is56B. Unused enum payload/padding is initialized
/// here; normal tag0 does not expose those bytes as a normal.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WidgetIntersection {
    pub normal: Option<Vec3>,
    pub widget: WidgetId,
    pub element: usize,
    pub distance: f32,
    pub position: Vec3,
    pub occlude_by_world: bool,
    pub flags: u8,
}
impl WidgetIntersection {
    pub fn to_native(self) -> [u8; 56] {
        let mut b = [0; 56];
        b[..4].copy_from_slice(&u32::from(self.normal.is_some()).to_le_bytes());
        if let Some(n) = self.normal {
            for (i, v) in n.to_array().into_iter().enumerate() {
                b[4 + i * 4..8 + i * 4].copy_from_slice(&v.to_bits().to_le_bytes());
            }
        }
        b[16..24].copy_from_slice(&self.widget.0.to_le_bytes());
        b[24..32].copy_from_slice(&(self.element as u64).to_le_bytes());
        for (i, v) in [
            self.distance,
            self.position.x,
            self.position.y,
            self.position.z,
        ]
        .into_iter()
        .enumerate()
        {
            b[32 + i * 4..36 + i * 4].copy_from_slice(&v.to_bits().to_le_bytes());
        }
        b[48] = self.occlude_by_world as u8;
        b[49] = self.flags;
        b
    }
}
#[derive(Default)]
pub struct HoverIntersectionCache {
    widgets: Vec<(WidgetId, Arc<WidgetParams>)>,
}
#[derive(Clone, Copy)]
pub struct PickerInput {
    pub ray: Ray,
    pub screen: Vec2,
    pub terrain: Option<Vec3>,
}

/// The original parallel closure returns ordinary intersections separately from
/// layered canvases. A canvas is appended only after ordinary depth arbitration.
pub fn pick_chunk(
    widgets: &[(WidgetId, Arc<WidgetParams>)],
    input: PickerInput,
) -> (Vec<WidgetIntersection>, Vec<(u8, WidgetIntersection)>) {
    let mut ordinary = Vec::with_capacity(widgets.len());
    let mut canvases = Vec::new();
    for (id, params) in widgets {
        let mut hit = WidgetIntersection {
            normal: None,
            widget: *id,
            element: 0,
            distance: 0.0,
            position: Vec3::ZERO,
            occlude_by_world: params.occlude_by_world,
            flags: params.flags.0,
        };
        match params.resolved_geometry().as_ref() {
            WidgetGeometry::Sphere { center, radius } => {
                let Some(s) = ray_sphere_intersection(
                    input.ray,
                    Sphere {
                        center: *center,
                        radius: *radius,
                    },
                ) else {
                    continue;
                };
                hit.distance = s.closest_intersection_distance();
                hit.position = input.ray.origin + input.ray.direction * hit.distance;
                hit.normal = Some((hit.position - *center).normalize());
            }
            WidgetGeometry::SphereCloud { centers, radius } => {
                let mut best: Option<(usize, f32)> = None;
                for (i, center) in centers.iter().enumerate() {
                    let Some(s) = ray_sphere_intersection(
                        input.ray,
                        Sphere {
                            center: *center,
                            radius: *radius,
                        },
                    ) else {
                        continue;
                    };
                    let d = s.closest_intersection_distance();
                    if !d.is_finite() {
                        continue;
                    }
                    if best.is_none_or(|(_, b)| {
                        d.partial_cmp(&b)
                            .expect("original sphere distance comparison is unordered")
                            .is_lt()
                    }) {
                        best = Some((i, d));
                    }
                }
                let Some((element, distance)) = best else {
                    continue;
                };
                hit.element = element;
                hit.distance = distance;
                hit.position = input.ray.origin + input.ray.direction * distance;
            }
            WidgetGeometry::TriangleMesh(triangles) => {
                let mut best: Option<(usize, f32)> = None;
                for (i, triangle) in triangles.iter().enumerate() {
                    let Some(s) = ray_triangle_intersection(input.ray, *triangle) else {
                        continue;
                    };
                    if best.is_none_or(|(_, b)| {
                        s.distance
                            .partial_cmp(&b)
                            .expect("original triangle distance comparison is unordered")
                            .is_lt()
                    }) {
                        best = Some((i, s.distance));
                    }
                }
                let Some((element, distance)) = best else {
                    continue;
                };
                hit.element = element;
                hit.distance = distance;
                hit.position = input.ray.origin + input.ray.direction * distance;
                hit.normal = Some(triangles[element].normal());
            }
            WidgetGeometry::ScreenAabb { min, max, distance } => {
                if !utils::geometry::aabb::is_inside_bbx(input.screen, *min, *max) {
                    continue;
                }
                hit.distance = *distance;
                hit.position = input.screen.extend(*distance);
            }
            WidgetGeometry::WorldCanvas { extent, layer } => {
                let terrain = input.terrain.unwrap_or(Vec3::ZERO);
                let rectangle = Rectangle2d::new_centered_axis_aligned(*extent);
                if !rectangle.contains_point(Vec2::new(terrain.x, terrain.z)) {
                    continue;
                }
                hit.normal = Some(Vec3::Y);
                hit.distance = f32::MAX;
                hit.position = terrain;
                hit.occlude_by_world = false;
                canvases.push((*layer, hit));
                continue;
            }
            WidgetGeometry::TerrainCanvas { distance, .. } => {
                hit.distance = *distance;
                hit.position = input.terrain.unwrap_or(Vec3::ZERO);
                hit.occlude_by_world = false;
            }
            WidgetGeometry::PlaneDisc {
                point,
                normal,
                radius,
            } => {
                let Some((distance, position)) = (Plane {
                    point: *point,
                    normal: *normal,
                })
                .intersect_ray(input.ray, 0.0) else {
                    continue;
                };
                let p = position - *point;
                // Native evaluates y²+x²+z² and uses a strict comparison.
                if !(*radius * *radius > (p.y * p.y + p.x * p.x) + p.z * p.z) {
                    continue;
                }
                hit.distance = distance;
                hit.position = position;
                hit.normal = Some(*normal);
            }
            WidgetGeometry::Empty => continue,
        }
        ordinary.push(hit);
    }
    (ordinary, canvases)
}

/// Native retain closure1412f6680 computes negative view Z independently from
/// the stored position, then keeps world-occluded hits only strictly inside the
/// readback depth plus the original1.0 world-unit margin. NaN rejects a hit.
pub fn arbitrate(
    mut ordinary: Vec<WidgetIntersection>,
    mut canvases: Vec<(u8, WidgetIntersection)>,
    ray: Ray,
    view: Mat4,
    world_view_depth: f32,
) -> Vec<WidgetIntersection> {
    ordinary.retain(|hit| {
        if !hit.occlude_by_world {
            return true;
        }
        let p = ray.origin + ray.direction * hit.distance;
        let z = ((p.y * view.y_axis.z + p.x * view.x_axis.z) + p.z * view.z_axis.z) + view.w_axis.z;
        world_view_depth + 1.0 > -z
    });
    canvases.sort_by_key(|(layer, _)| *layer);
    ordinary.retain(|hit| hit.distance >= 0.0 || hit.flags & 4 != 0);
    ordinary.sort_by(|a, b| {
        a.distance
            .partial_cmp(&b.distance)
            .expect("original UI distance comparison is unordered")
    });
    ordinary.extend(canvases.into_iter().map(|(_, hit)| hit));
    ordinary
}

pub fn hover_intersection(
    mut ui: ResMut<UiState>,
    camera: Res<MainCamera>,
    terrain: Res<CursorTerrainRaycast>,
    cursor: Res<CursorPositionSS>,
    depth: Res<MouseDepthValue>,
    window: Res<WindowSize>,
    mut cache: Local<HoverIntersectionCache>,
) {
    // Original marks the UiState mutable access before ray construction.
    let ui = &mut *ui;
    // Original recomputes this ray here; set6 precedes camera_mouse_ray set10.
    let ray = Ray::from_mouse_ss(
        cursor.0,
        window.as_vec2(),
        camera.read_matrix(0x40),
        camera.read_matrix(0xc0),
    );
    cache.widgets.clear();
    cache.widgets.extend(ui.registry_snapshot());
    let input = PickerInput {
        ray,
        screen: cursor.0,
        terrain: ((terrain.tag & 1) != 0).then_some(terrain.pos),
    };
    let chunks = cache
        .widgets
        .par_splat_map(ComputeTaskPool::get(), None, |_, slice| {
            pick_chunk(slice, input)
        });
    cache.widgets.clear();
    let mut ordinary = Vec::new();
    let mut canvases = Vec::new();
    for (mut a, mut b) in chunks {
        ordinary.append(&mut a);
        canvases.append(&mut b);
    }
    let results = arbitrate(
        ordinary,
        canvases,
        ray,
        camera.read_matrix(0x80),
        depth.view_depth,
    );
    ui.set_hover_intersections(
        results
            .into_iter()
            .map(WidgetIntersection::to_native)
            .collect(),
    );
}

#[cfg(test)]
mod original_tests {
    use super::*;
    use serde_json::Value;
    fn unhex(s: &str) -> Vec<u8> {
        s.as_bytes()
            .chunks_exact(2)
            .map(|b| u8::from_str_radix(std::str::from_utf8(b).unwrap(), 16).unwrap())
            .collect()
    }
    fn f(b: &[u8], i: usize) -> f32 {
        f32::from_le_bytes(b[i..i + 4].try_into().unwrap())
    }
    fn v3(b: &[u8], i: usize) -> Vec3 {
        Vec3::new(f(b, i), f(b, i + 4), f(b, i + 8))
    }
    fn json3(v: &Value) -> Vec3 {
        Vec3::new(
            v[0].as_f64().unwrap() as f32,
            v[1].as_f64().unwrap() as f32,
            v[2].as_f64().unwrap() as f32,
        )
    }
    fn input(v: &Value) -> PickerInput {
        PickerInput {
            ray: Ray {
                origin: v.get("origin").map(json3).unwrap_or(Vec3::ZERO),
                direction: v.get("direction").map(json3).unwrap_or(Vec3::Z),
            },
            screen: v
                .get("screen")
                .map(|v| Vec2::new(v[0].as_f64().unwrap() as f32, v[1].as_f64().unwrap() as f32))
                .unwrap_or(Vec2::ZERO),
            terrain: v.get("terrain").filter(|v| !v.is_null()).map(json3),
        }
    }
    fn decode_widget(w: &Value) -> (WidgetId, Arc<WidgetParams>) {
        let b = unhex(w["bytes"].as_str().unwrap());
        let geometry = match b[0x88] {
            0 => WidgetGeometry::Sphere {
                center: v3(&b, 0x90),
                radius: f(&b, 0x8c),
            },
            1 => WidgetGeometry::SphereCloud {
                centers: w["extra"]
                    .as_array()
                    .map(|a| a.iter().map(json3).collect())
                    .unwrap_or_default(),
                radius: f(&b, 0x8c),
            },
            2 => WidgetGeometry::TriangleMesh(
                w["extra"]
                    .as_array()
                    .map(|a| {
                        a.iter()
                            .map(|x| {
                                let q = x.as_array().unwrap();
                                let v = |i| {
                                    Vec3::new(
                                        q[i].as_f64().unwrap() as f32,
                                        q[i + 1].as_f64().unwrap() as f32,
                                        q[i + 2].as_f64().unwrap() as f32,
                                    )
                                };
                                utils::geometry::intersect::Triangle::new(v(0), v(3), v(6))
                            })
                            .collect()
                    })
                    .unwrap_or_default(),
            ),
            3 => WidgetGeometry::ScreenAabb {
                min: Vec2::new(f(&b, 0x90), f(&b, 0x94)),
                max: Vec2::new(f(&b, 0x98), f(&b, 0x9c)),
                distance: f(&b, 0x8c),
            },
            4 => WidgetGeometry::WorldCanvas {
                extent: Vec2::new(f(&b, 0x8c), f(&b, 0x90)),
                layer: b[0x89],
            },
            5 => WidgetGeometry::TerrainCanvas {
                distance: f(&b, 0x8c),
                layer: b[0x89],
            },
            6 => WidgetGeometry::PlaneDisc {
                point: v3(&b, 0x8c),
                normal: v3(&b, 0x98),
                radius: f(&b, 0xa4),
            },
            7 => WidgetGeometry::Empty,
            _ => panic!("fixture has invalid volume tag"),
        };
        let mut p = WidgetParams::default().geometry(geometry);
        p.occlude_by_world = b[0xa8] != 0;
        p.flags.0 = b[0xa9];
        (WidgetId(w["id"].as_u64().unwrap()), Arc::new(p))
    }
    fn equal_semantics(actual: [u8; 56], expected: &[u8], label: &str) {
        assert_eq!(&actual[..4], &expected[..4], "{label}: normal tag");
        assert_eq!(
            &actual[16..32],
            &expected[16..32],
            "{label}: widget/element"
        );
        assert_eq!(&actual[48..50], &expected[48..50], "{label}: flags");
        let mut offsets = vec![32, 36, 40, 44];
        if expected[0] == 1 {
            offsets.extend([4, 8, 12]);
        }
        for i in offsets {
            let x = f(&actual, i);
            let y = f(expected, i);
            assert!(
                x.to_bits() == y.to_bits() || (x - y).abs() <= 1e-6 || (x.is_nan() && y.is_nan()),
                "{label}: offset{i} actual{x} original{y}"
            );
        }
    }
    #[test]
    fn unchanged_original_picker_fixtures() {
        let cases:Value=serde_json::from_str(include_str!(concat!(env!("CARGO_MANIFEST_DIR"),"/../../../reverse-engineering/tiny-glade/evidence/round12/ui-picker/native-picker-all-fixtures.json"))).unwrap();
        for case in cases.as_array().unwrap() {
            let widgets = case["widgets"]
                .as_array()
                .unwrap()
                .iter()
                .map(decode_widget)
                .collect::<Vec<_>>();
            let (a, b) = pick_chunk(&widgets, input(&case["inputs"]));
            let ordinary = a;
            let canvases = b.into_iter().map(|(_, h)| h).collect::<Vec<_>>();
            for (name, actual) in [("ordinary", ordinary), ("canvas", canvases)] {
                let expected = case["result"][name].as_array().unwrap();
                assert_eq!(actual.len(), expected.len(), "{} {name}", case["name"]);
                for (hit, e) in actual.into_iter().zip(expected) {
                    let bytes = unhex(e.as_str().unwrap());
                    equal_semantics(
                        hit.to_native(),
                        &bytes[if name == "canvas" { 8 } else { 0 }..],
                        case["name"].as_str().unwrap(),
                    );
                }
            }
        }
    }
    fn decode_hit(b: &[u8]) -> WidgetIntersection {
        WidgetIntersection {
            normal: (b[0] == 1).then(|| v3(b, 4)),
            widget: WidgetId(u64::from_le_bytes(b[16..24].try_into().unwrap())),
            element: u64::from_le_bytes(b[24..32].try_into().unwrap()) as usize,
            distance: f(b, 32),
            position: v3(b, 36),
            occlude_by_world: b[48] != 0,
            flags: b[49],
        }
    }
    #[test]
    fn unchanged_original_arbitration_fixtures() {
        let cases:Value=serde_json::from_str(include_str!(concat!(env!("CARGO_MANIFEST_DIR"),"/../../../reverse-engineering/tiny-glade/evidence/round12/ui-picker/native-arbitration-fixtures.json"))).unwrap();
        for case in cases.as_array().unwrap() {
            let decode = |x: &Value| decode_hit(&unhex(x.as_str().unwrap()));
            let ordinary = case["ordinary"]
                .as_array()
                .unwrap()
                .iter()
                .map(decode)
                .collect();
            let canvases = case["canvases"]
                .as_array()
                .unwrap()
                .iter()
                .map(|v| (v[0].as_u64().unwrap() as u8, decode(&v[1])))
                .collect();
            let view = case["inputs"]
                .get("view")
                .map(|v| {
                    Mat4::from_cols_array(&std::array::from_fn(|i| v[i].as_f64().unwrap() as f32))
                })
                .unwrap_or(Mat4::IDENTITY);
            let depth = case["inputs"]
                .get("depth")
                .map(|v| v.as_f64().unwrap() as f32)
                .unwrap_or(0.0);
            let actual = arbitrate(ordinary, canvases, input(&case["inputs"]).ray, view, depth);
            let expected = case["result"].as_array().unwrap();
            assert_eq!(actual.len(), expected.len(), "{}", case["name"]);
            for (hit, e) in actual.into_iter().zip(expected) {
                equal_semantics(
                    hit.to_native(),
                    &unhex(e.as_str().unwrap()),
                    case["name"].as_str().unwrap(),
                );
            }
        }
    }
    #[test]
    fn scheduled_picker_uses_current_mouse_position_and_real_registry() {
        ComputeTaskPool::get_or_init(|| bevy_tasks::TaskPoolBuilder::new().num_threads(2).build());
        let mut world = World::new();
        let ui = UiState::at_startup();
        ui.add_with_id(
            WidgetId(91),
            WidgetParams::default()
                .geometry(WidgetGeometry::ScreenAabb {
                    min: Vec2::splat(40.0),
                    max: Vec2::splat(60.0),
                    distance: 0.5,
                })
                .occlude_by_world(false),
        );
        world.insert_resource(ui);
        let mut camera = MainCamera::default();
        for offset in [0x40, 0x80, 0xc0] {
            camera.write_matrix(offset, &Mat4::IDENTITY);
        }
        world.insert_resource(camera);
        world.insert_resource(CursorTerrainRaycast::default());
        world.insert_resource(CursorPositionSS(Vec2::splat(50.0)));
        world.insert_resource(MouseDepthValue::default());
        world.insert_resource(WindowSize::new(100, 100));
        let mut schedule = bevy_ecs::schedule::Schedule::default();
        schedule.add_systems(hover_intersection);
        schedule.run(&mut world);
        let hits = world.resource::<UiState>().hover_intersections();
        assert_eq!(hits.len(), 1);
        assert_eq!(u64::from_le_bytes(hits[0][16..24].try_into().unwrap()), 91);
        world.resource_mut::<CursorPositionSS>().0 = Vec2::splat(90.0);
        schedule.run(&mut world);
        assert!(world.resource::<UiState>().hover_intersections().is_empty());
    }
}
