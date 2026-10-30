//! B14083d3f0/14083da10：矩形角点旋转拾取与箭头，替换旧空壳。
//! selected的FlatMap由1413047b0→1412f1b70取证，是两组完整旋转盒。
use crate::{
    resources::walls::{WallId, public_wall_state::PublicWallState},
    systems::{
        ui::{
            resources::{
                ui_state::{UiState, UiWidgetContext, WidgetId},
                widget_params::WidgetParams,
            },
            signifiers::{SignifierEntityBuilder, SignifierFactory, UiSignifierStream},
        },
        wall::shapes::rectangle::ui_edit_rectangle_orientation::box_trigger_volume,
    },
    utils::components::transform::Transform,
};
use ::utils::{
    curve::approx_curve_coord,
    geometry::{Rectangle2d, intersect::Triangle},
};
use glam::{Quat, Vec2, Vec3};
use std::sync::Arc;

pub struct UiRectangleOrientationEdit {
    wall: WallId,
    rectangle: Rectangle2d,
    corner: usize,
    selected: bool,
    height: Option<f32>,
}
impl UiRectangleOrientationEdit {
    pub fn new(wall: WallId, rectangle: Rectangle2d, corner: usize, selected: bool) -> Self {
        assert!(corner < 4, "原矩形旋转角序必须小于4");
        Self {
            wall,
            rectangle,
            corner,
            selected,
            height: None,
        }
    }
    /// 原tuple Hash与固定异或盐，不使用字符串WidgetId代理。
    pub fn widget_id(&self) -> WidgetId {
        WidgetId(::utils::calculate_hash(&(self.wall.0, self.corner)) ^ 0xa8f6_4de7_10aa_4d92)
    }
    pub fn widget<'a>(
        &mut self,
        ui: &'a UiState,
        wall: &PublicWallState,
        camera_moved: bool,
    ) -> UiWidgetContext<'a> {
        let point = self.rectangle.get_point(self.corner);
        let path = wall
            .prepared_path
            .as_ref()
            .expect("矩形旋转拾取前必须发布完整墙路径");
        let coord = approx_curve_coord(&path.positions, point, |point| {
            Vec2::new(point[0], point[2])
        });
        let first = path.positions[coord.segment][1];
        let second = path.positions[coord.segment + 1][1];
        // 原两项乘加，不能改用a+(b-a)*t。
        let height = second * coord.t + (1.0 - coord.t) * first + 1.0;
        self.height = Some(height);
        let volume = if self.selected {
            selected_volume(self.rectangle, self.corner, point, height)
        } else {
            let heights = wall
                .arch_segment_heights()
                .expect("矩形旋转拾取前必须初始化墙顶高度");
            let top = heights[coord.segment]
                + (heights[coord.segment + 1] - heights[coord.segment]) * coord.t;
            let half_height = (top - height) * 0.5;
            let normal = self.rectangle.get_corner_normal(self.corner);
            let xz = point - normal * 0.3;
            box_trigger_volume(
                1.5,
                top - height,
                1.5,
                &Transform {
                    translation: Vec3::new(xz.x, half_height + height, xz.y),
                    rotation: Quat::from_mat3(&self.rectangle.orientation_mat3()),
                    scale: Vec3::ONE,
                },
            )
        };
        let mut params = WidgetParams::default()
            .mesh_volume(volume)
            .occlude_by_world(!self.selected);
        for (slot, tag) in params.sounds.slots.iter_mut().zip([1u32, 2, 3, 3]) {
            slot[..4].copy_from_slice(&(if camera_moved { 0 } else { tag }).to_le_bytes());
        }
        ui.add_with_id(self.widget_id(), params)
    }
    pub fn signifier(&self, context: &UiWidgetContext<'_>, stream: &mut UiSignifierStream) {
        if !self.selected && !context.is_active() {
            return;
        }
        let point = self.rectangle.get_point(self.corner);
        let height = if self.selected {
            self.height.expect("旋转箭头必须先创建拾取体") + 0.1
        } else {
            // UiWidgetContext返回的原56B交点载荷，Y位于+40。
            let intersection = context.intersecting();
            f32::from_le_bytes(intersection[40..44].try_into().unwrap())
        };
        let position = Vec3::new(point.x, height, point.y);
        let normal = self.rectangle.get_corner_normal(self.corner);
        let rotation = ::utils::rotation_from_normal(Vec3::new(normal.x, 0.0, normal.y));
        let enlarge = context.hovered() != 0 || context.dragged() != 2;
        let scale = if !self.selected {
            0.6
        } else if enlarge {
            f32::from_bits(0x3f13_74bd)
        } else {
            f32::from_bits(0x3ef5_c290)
        };
        for (pass, mesh) in [(0, "corner_arrow"), (1, "corner_arrow_outline")] {
            // 原两个pass共用名字；signifier_create按帧前实体快照去重，允许同帧两次唤醒。
            let name = format!("rectangle_{:?}_rotate_{position:?}{scale:?}", self.wall);
            stream.write_factory(&SignifierFactory::from_closure(
                &name,
                Arc::new(move |meshes, shaders, commands| {
                    vec![
                        SignifierEntityBuilder::from_mesh_name(mesh)
                            .rotation(rotation)
                            .position(position)
                            .scale(scale)
                            .animation_duration(0.0)
                            .set_ui_pass(pass)
                            .build(meshes, shaders, commands),
                    ]
                }),
            ));
        }
    }
}
/// 原1412f1c0f..cb1：两边各自的方向/外法线，完整生成24个三角形。
fn selected_volume(
    rectangle: Rectangle2d,
    corner: usize,
    point: Vec2,
    height: f32,
) -> Vec<Triangle> {
    let [previous, next] = rectangle.get_edge_dirs_from_corner_id(corner);
    let directions = [
        (previous, Vec2::new(previous.y, -previous.x)),
        (next, Vec2::new(-next.y, next.x)),
    ];
    let mut result = Vec::with_capacity(24);
    for (direction, outward) in directions {
        result.extend(box_trigger_volume(
            0.7,
            0.5,
            f32::from_bits(0x3ff3_3334),
            &selected_box_transform(point, direction, outward, height),
        ));
    }
    result
}

fn selected_box_transform(point: Vec2, direction: Vec2, outward: Vec2, height: f32) -> Transform {
    // 原常量是-0.09999996423721313，不能用四舍五入的-0.1。
    let xz = outward * 0.7 + (point + direction * f32::from_bits(0xbdcc_ccc8));
    Transform {
        translation: Vec3::new(xz.x, height + 0.1, xz.y),
        rotation: ::utils::xz_rotation_bad(direction),
        scale: Vec3::ONE,
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn actual_widget_and_owned_factories_use_published_heights() {
        use crate::{
            asset_libraries::{
                mesh_library::{AssetMeshLibrary, MeshAsset},
                shader_library::AssetShaderLibrary,
            },
            render::{mesh::Mesh, shader::RasterShaderProgram, test_device::TestDevice},
            resources::{
                roofs::geometry::surface::Curve2,
                walls::{WallState, publication_source::TerrainHeightGrid512},
            },
            systems::{
                ui::{
                    resources::ui_state::UiInteraction,
                    signifiers::{UiSignifierEntry, animation::SignifierAnimation},
                },
                wall::{
                    valid_enclosure::{Shape, ShapeParameters},
                    wall_state::path_preparation::PathRasters512,
                },
            },
        };
        use bevy_ecs::{prelude::*, world::CommandQueue};
        let rectangle = Rectangle2d::new_axis_aligned(Vec2::ZERO, Vec2::new(4., 6.));
        let shape = ShapeParameters::from_shape(Shape::Rectangle(rectangle));
        let mut authority = WallState::new(
            Curve2::try_new(
                shape
                    .as_curve2()
                    .points
                    .into_iter()
                    .map(|point| point.to_array())
                    .collect(),
                false,
            )
            .unwrap(),
            4.,
            0,
            0,
            false,
        );
        authority.snapshot.source.shape = shape.to_snapshot();
        let terrain = TerrainHeightGrid512 {
            encoded: vec![0u16; 512 * 512].into(),
            minimum_y: 2.,
            maximum_y: 2.,
            sampling_fix: false,
        };
        let paths = PathRasters512::from_encoded(
            std::array::from_fn(|_| vec![0u8; 512 * 512].into()),
            false,
        )
        .unwrap();
        let mut published = PublicWallState::collect(&authority, &terrain).unwrap();
        published.prepare_paths(&paths).unwrap();
        let meshes = AssetMeshLibrary::new();
        for name in ["corner_arrow", "corner_arrow_outline"] {
            meshes.add(MeshAsset::new(Mesh::new(name), Some(name.to_owned())));
        }
        let shaders = AssetShaderLibrary::new();
        shaders.add_raster(
            RasterShaderProgram::new_draw(
                "vertex_color_shader",
                Default::default(),
                &TestDevice(1),
            ),
            false,
        );
        for selected in [false, true] {
            let mut edit = UiRectangleOrientationEdit::new(WallId(8), rectangle, 0, selected);
            let mut ui = UiState::new(UiInteraction {
                native_tag: 2,
                native_phase: 4,
                widget: edit.widget_id(),
            });
            ui.set_intersection_point(Vec3::new(-2., 9., -3.));
            let context = edit.widget(&ui, &published, true);
            let params = ui.widget(context.id).unwrap();
            assert_eq!(params.occlude_by_world, !selected);
            let crate::systems::ui::resources::widget_params::WidgetGeometry::TriangleMesh(
                triangles,
            ) = params.geometry.as_ref().unwrap()
            else {
                panic!("必须使用原真实三角形拾取体")
            };
            assert_eq!(triangles.len(), if selected { 24 } else { 12 });
            assert!(
                params
                    .sounds
                    .slots
                    .iter()
                    .all(|slot| u32::from_le_bytes(slot[..4].try_into().unwrap()) == 0)
            );
            let mut stream = UiSignifierStream::default();
            edit.signifier(&context, &mut stream);
            let factories = stream.take_entries();
            assert_eq!(factories.len(), 2);
            let names: Vec<_> = factories
                .iter()
                .map(|entry| match entry {
                    UiSignifierEntry::Factory(f) => f.name().unwrap(),
                    _ => panic!("必须保留真实工厂"),
                })
                .collect();
            assert_eq!(names[0], names[1]);
            let mut world = World::new();
            let mut queue = CommandQueue::default();
            let entities: Vec<_> = factories
                .iter()
                .flat_map(|entry| match entry {
                    UiSignifierEntry::Factory(f) => {
                        f.wake(&meshes, &shaders, &mut Commands::new(&mut queue, &world))
                    }
                    _ => unreachable!(),
                })
                .collect();
            queue.apply(&mut world);
            assert_eq!(entities.len(), 2);
            for entity in entities {
                assert_eq!(
                    world.get::<Transform>(entity).unwrap().translation.y,
                    if selected { 3.1 } else { 9. }
                );
                let animation = world.get::<SignifierAnimation>(entity).unwrap();
                assert_eq!(animation.duration, 0.);
                assert_eq!(
                    animation.value.to_bits(),
                    if selected {
                        0x3f13_74bd
                    } else {
                        0.6f32.to_bits()
                    }
                );
            }
        }
    }
    #[test]
    fn native_flatmap_box_arguments_match_bits() {
        let cases: serde_json::Value = serde_json::from_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../reports/module-recovery-2026-10-23/m01-rectangle-orientation-native.json"
        )))
        .unwrap();
        let pair = |x: &serde_json::Value| {
            Vec2::new(
                f32::from_bits(x[0].as_u64().unwrap() as u32),
                f32::from_bits(x[1].as_u64().unwrap() as u32),
            )
        };
        for case in cases["cases"].as_array().unwrap() {
            let transform = selected_box_transform(
                pair(&case["point"]),
                pair(&case["direction"]),
                pair(&case["outward"]),
                f32::from_bits(case["height"].as_u64().unwrap() as u32),
            );
            let actual: Vec<u32> = transform
                .rotation
                .to_array()
                .into_iter()
                .chain(transform.translation.to_array())
                .chain(transform.scale.to_array())
                .map(f32::to_bits)
                .collect();
            let expected: Vec<u32> = case["transform"]
                .as_array()
                .unwrap()
                .iter()
                .map(|x| x.as_u64().unwrap() as u32)
                .collect();
            assert_eq!(actual, expected);
            assert_eq!(
                case["dimensions"],
                serde_json::json!([0.7f32.to_bits(), 0.5f32.to_bits(), 0x3ff3_3334u32])
            );
        }
    }
    #[test]
    fn selected_corner_preserves_two_independent_trigger_boxes() {
        let rectangle = Rectangle2d::new_axis_aligned(Vec2::ZERO, Vec2::new(4., 6.));
        let triangles = selected_volume(rectangle, 0, rectangle.get_point(0), 3.);
        assert_eq!(triangles.len(), 24);
        assert_ne!(triangles[0].points[0], triangles[12].points[0]);
    }
}
