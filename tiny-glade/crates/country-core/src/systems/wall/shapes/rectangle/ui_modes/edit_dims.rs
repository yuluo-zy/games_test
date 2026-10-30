//! B14155fc70/14155eba0：矩形尺寸编辑的真实拾取体及拥有型指示符。
use super::super::ui_edit_rectangle_dims::UiEditRectangleActiveWidgetResource;
use crate::{
    resources::walls::{
        WallId,
        public_wall_state::PublicWallState,
        wall_trigger_volume::{WallTriggerVolume, WallTriggerVolumeParams},
    },
    systems::ui::{
        resources::{
            ui_state::{UiState, UiWidgetContext, WidgetId},
            widget_params::WidgetParams,
        },
        signifiers::{SignifierEntityBuilder, SignifierFactory, UiSignifierStream},
    },
    utils::components::transform::Transform,
};
use bevy_ecs::prelude::*;
use glam::{Vec2, Vec3};
use std::sync::Arc;
use utils::{curve::Curve2, geometry::Rectangle2d};

#[derive(Clone, Debug)]
pub struct UiRectangleDimsEdit {
    pub wall: WallId,
    pub rectangle: Rectangle2d,
    pub edge: usize,
    pub selected: bool,
    pub transform: Option<Transform>,
}
impl UiRectangleDimsEdit {
    pub fn new(wall: WallId, rectangle: Rectangle2d, edge: usize, selected: bool) -> Self {
        assert!(edge < 4);
        Self {
            wall,
            rectangle,
            edge,
            selected,
            transform: None,
        }
    }
    pub fn widget_id(&self) -> WidgetId {
        WidgetId(utils::calculate_hash(&(self.wall.0, self.edge)) ^ 0x448d3066df154a46)
    }
    pub fn widget<'a>(
        &mut self,
        ui: &'a UiState,
        wall: &PublicWallState,
        trigger: Option<&WallTriggerVolume>,
        camera_moved: bool,
    ) -> UiWidgetContext<'a> {
        let triangles = if !self.selected {
            let trigger = trigger.expect("未聚焦矩形墙缺少WallTriggerVolume");
            if trigger.is_valid() {
                let u = self.rectangle.corner_us();
                trigger.get_tris(WallTriggerVolumeParams {
                    u_range: Some([u[self.edge], u[self.edge + 1]]),
                    ..WallTriggerVolumeParams::edit_shape_default()
                })
            } else {
                Vec::new()
            }
        } else {
            let path = wall
                .prepared_path
                .as_ref()
                .expect("矩形编辑前公开墙路径未准备");
            let closest = |point| {
                utils::curve::approx_curve_coord(&path.positions, point, |p| Vec2::new(p[0], p[2]))
            };
            let first = closest(self.rectangle.get_point(self.edge));
            let last = closest(self.rectangle.get_point((self.edge + 1) % 4));
            let height = |coordinate: utils::curve::CurveInternalCoord, values: &[f32]| {
                (values[coordinate.segment + 1] - values[coordinate.segment]) * coordinate.t
                    + values[coordinate.segment]
            };
            let base_height = |coordinate: utils::curve::CurveInternalCoord| {
                path.positions[coordinate.segment + 1][1] * coordinate.t
                    + (1.0 - coordinate.t) * path.positions[coordinate.segment][1]
            };
            let base_first = base_height(first);
            let base_last = base_height(last);
            let base = base_first.max(base_last);
            let heights = wall
                .arch_segment_heights()
                .expect("矩形编辑前拱高缓存未准备");
            let top = height(first, heights).max(height(last, heights));
            let y = (top - base) * 0.5 + base;
            let (a, b) = self.rectangle.get_edge_pts(self.edge);
            let normal = self.rectangle.get_edge_normal(self.edge);
            let mut transform = Transform::from_translation(Vec3::new(
                b.x * 0.5 + a.x * 0.5 + normal.x * 0.63 * 0.5,
                y,
                b.y * 0.5 + a.y * 0.5 + normal.y * 0.63 * 0.5,
            ));
            transform.rotation = utils::rotation_from_normal(Vec3::new(normal.x, 0.0, normal.y));
            self.transform = Some(transform);
            // 原盒沿本地+Z再移一个单位，原尺寸1×0.5×2。
            transform.translation += transform.rotation * Vec3::Z;
            super::super::ui_edit_rectangle_orientation::box_trigger_volume(
                1.0, 0.5, 2.0, &transform,
            )
        };
        let enabled = u32::from(!camera_moved);
        let mut params = if triangles.is_empty() {
            WidgetParams::default()
                .geometry(crate::systems::ui::resources::widget_params::WidgetGeometry::Empty)
        } else {
            WidgetParams::default().mesh_volume(triangles)
        }
        .occlude_by_world(!self.selected);
        for (slot, tag) in params.sounds.slots.iter_mut().zip([1u32, 2, 3, 3]) {
            slot[..4].copy_from_slice(&(if camera_moved { 0 } else { tag }).to_le_bytes());
        }
        params.fields[3] = enabled;
        params.fields[5] = enabled * 2;
        params.fields[6] = enabled * 3;
        params.fields[7] = enabled * 3;
        ui.add_with_id(self.widget_id(), params)
    }
    pub fn signifier(
        &self,
        context: &UiWidgetContext,
        stream: &mut UiSignifierStream,
        active: &mut UiEditRectangleActiveWidgetResource,
    ) {
        if !self.selected {
            let hit = context.intersecting();
            if u32::from_le_bytes(hit[..4].try_into().unwrap()) == 2 {
                return;
            }
            let y = f32::from_le_bytes(hit[40..44].try_into().unwrap());
            let (a, b) = self.rectangle.get_edge_pts(self.edge);
            let center = Vec2::new(b.x * 0.5 + a.x * 0.5, b.y * 0.5 + a.y * 0.5);
            let position = Vec3::new(center.x, y, center.y);
            let name = format!("rectangle_{:?}_edit_dims_arrow_{position:?}", self.wall);
            let normal = self.rectangle.get_edge_normal(self.edge);
            stream.write_factory(&SignifierFactory::from_closure(
                &name,
                Arc::new(move |m, s, c| {
                    vec![
                        SignifierEntityBuilder::from_mesh_name("flat_arrow")
                            .position(position)
                            .rotation(utils::rotation_from_normal(Vec3::new(
                                normal.x, 0.0, normal.y,
                            )))
                            .scale(2.0)
                            .animation_duration(0.0)
                            .build(m, s, c),
                    ]
                }),
            ));
            let curve = Curve2::crashy_new_from_points(vec![a - center, b - center], false);
            active.0 = Some((curve, context.id, position));
        } else if let Some(transform) = self.transform {
            let scale = if context.hovered() != 0 || context.dragged() != 2 {
                1.2
            } else {
                1.0
            };
            for (mesh, outline) in [("flat_arrow", false), ("flat_arrow_outline", true)] {
                stream.write_factory(&SignifierFactory::from_closure(
                    &format!(
                        "rectangle_{:?}_edit_dims_arrow_{:?}_{mesh}_{scale:?}",
                        self.wall, transform.translation
                    ),
                    Arc::new(move |m, s, c| {
                        vec![
                            SignifierEntityBuilder::from_mesh_name(mesh)
                                .position(transform.translation)
                                .rotation(transform.rotation)
                                .scale(scale * 1.5)
                                .animation_duration(0.0)
                                .set_ui_pass(u8::from(outline))
                                .build(m, s, c),
                        ]
                    }),
                ));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        asset_libraries::{
            mesh_library::{AssetMeshLibrary, MeshAsset},
            shader_library::AssetShaderLibrary,
        },
        render::{mesh::Mesh, shader::RasterShaderProgram, test_device::TestDevice},
        systems::{
            render_loop::draw_passes::draw_lists::UiPass,
            ui::{
                resources::ui_state::UiInteraction,
                signifiers::{SignifierAnimation, UiSignifierEntry},
            },
        },
    };
    use bevy_ecs::world::CommandQueue;

    #[test]
    fn original_dimension_factories_keep_zero_duration_and_separate_draw_passes() {
        let meshes = AssetMeshLibrary::new();
        for name in ["flat_arrow", "flat_arrow_outline"] {
            meshes.add(MeshAsset::new(
                Mesh::new("resolved named test asset"),
                Some(name.to_owned()),
            ));
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
        let rectangle = Rectangle2d::new_axis_aligned(Vec2::ZERO, Vec2::new(4., 6.));
        for selected in [false, true] {
            let mut widget = UiRectangleDimsEdit::new(WallId(7), rectangle, 0, selected);
            widget.transform =
                selected.then_some(Transform::from_translation(Vec3::new(1., 2., 3.)));
            let id = widget.widget_id();
            let mut ui = UiState::new(UiInteraction {
                native_tag: 2,
                native_phase: 4,
                widget: id,
            });
            ui.set_intersection_tag(0);
            ui.set_intersection_point(Vec3::new(-2., 3., 0.));
            let mut stream = UiSignifierStream::default();
            let mut active = UiEditRectangleActiveWidgetResource::default();
            widget.signifier(&ui.widget_context(id), &mut stream, &mut active);
            let factories = stream.take_entries();
            assert_eq!(factories.len(), if selected { 2 } else { 1 });
            if !selected {
                assert_eq!(active.0.as_ref().unwrap().1, id);
                assert_eq!(active.0.as_ref().unwrap().2.y, 3.);
            }
            let mut world = World::new();
            let mut queue = CommandQueue::default();
            let entities: Vec<_> = factories
                .iter()
                .flat_map(|entry| match entry {
                    UiSignifierEntry::Factory(factory) => {
                        factory.wake(&meshes, &shaders, &mut Commands::new(&mut queue, &world))
                    }
                    _ => panic!("矩形尺寸箭头必须拥有工厂"),
                })
                .collect();
            queue.apply(&mut world);
            for (index, entity) in entities.into_iter().enumerate() {
                let animation = world.get::<SignifierAnimation>(entity).unwrap();
                assert_eq!(animation.duration.to_bits(), 0);
                assert_eq!(
                    animation.value.to_bits(),
                    if selected {
                        0x3fe6_6667
                    } else {
                        2.0f32.to_bits()
                    }
                );
                assert_eq!(
                    world.get::<UiPass>(entity).is_some(),
                    !selected || index == 1,
                    "原flat位于world pass，outline位于UI pass；未聚焦单箭头为UI pass"
                );
            }
        }
    }
}
