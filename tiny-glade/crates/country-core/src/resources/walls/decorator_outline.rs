//! 原装饰网格库使用的主描边、碰撞描边及拥有工厂。
//!
//! 原140b33b00与两条四表插入链分别证明String、Vec<Vec3>和半径载荷。
//! 主描边的原三工厂使用现有真实指示符消费者；碰撞描边保原476B变换与debug请求。

use glam::Vec3;

/// 原主描边的四种载荷；显式None与库中没有exact key不同。
#[derive(Clone, Debug, PartialEq)]
pub enum DecoratorOutline {
    Mesh(String),
    Lines(Vec<Vec3>),
    Sphere(f32),
    None,
}

impl DecoratorOutline {
    /// B140b33b00：主描边工厂。rank 参与原名字，forward 门和球动画门互相独立。
    /// 原输入是完整 Affine3A；mesh/lines 取其原矩阵转四元数，不先归一化缩放基。
    pub fn display(
        &self,
        transform: &glam::Affine3A,
        stream: &mut crate::systems::ui::signifiers::signifier_create::UiSignifierStream,
        id: super::decorator::DecoratorId,
        rank: super::decorator::DecoratorRank,
        offset_forward: bool,
        animate: bool,
    ) {
        use crate::systems::ui::signifiers::signifier_factory::{SignifierEntityBuilder, SignifierFactory};
        use crate::render::pass_builder::draw_list::DrawTopology;
        use std::sync::Arc;

        let (rotation, position) = decorator_outline_pose(transform, offset_forward);
        let name = format!("move_decorator_{}_{position:?}_{:?}", id.0, OriginalRankDebug(rank));
        let factory = match self {
            Self::Mesh(mesh) => {
                let mesh = mesh.clone();
                SignifierFactory::from_closure(&name, Arc::new(move |meshes, shaders, commands| {
                    let entity = SignifierEntityBuilder::from_mesh_name(mesh.clone())
                        .rotation(rotation).position(position).animation_duration(0.0)
                        .build(meshes, shaders, commands);
                    commands.entity(entity).insert(DrawTopology::Alt(3.0));
                    vec![entity]
                }))
            }
            Self::Lines(points) => {
                // 原 display 捕获一份 owned 点表；唤醒时再复制给 transient mesh。
                // 点保留局部空间，由实体 rotation/position 消费，不套第二次仿射变换。
                let points = points.clone();
                SignifierFactory::from_closure(&name, Arc::new(move |meshes, shaders, commands| {
                    let entity = SignifierEntityBuilder::from_points(points.clone())
                        .rotation(rotation).position(position).animation_duration(0.0)
                        .build(meshes, shaders, commands);
                    commands.entity(entity).insert(DrawTopology::Alt(3.0));
                    vec![entity]
                }))
            }
            Self::Sphere(radius) => {
                let radius = *radius;
                SignifierFactory::from_closure(&name, Arc::new(move |meshes, shaders, commands| {
                    vec![SignifierEntityBuilder::from_mesh_name("temp_sphere")
                        .position(position).scale(radius)
                        .animation_duration(if animate { 0.1 } else { 0.0 })
                        .build(meshes, shaders, commands)]
                }))
            }
            Self::None => return,
        };
        stream.write_factory(&factory);
    }
}

/// 原140b35930为DecoratorRank的debug_struct_field1_finish，字段名x。
/// 不修改重构库全局Debug格式，只保证该原工厂名字的身份格式。
struct OriginalRankDebug(super::decorator::DecoratorRank);
impl std::fmt::Debug for OriginalRankDebug {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DecoratorRank").field("x", &self.0.0).finish()
    }
}

fn decorator_outline_pose(transform: &glam::Affine3A, offset_forward: bool) -> (glam::Quat, Vec3) {
    let x = transform.matrix3.x_axis;
    let y = transform.matrix3.y_axis;
    let z = transform.matrix3.z_axis;
    // 原140b33b6f..d46的四项矩阵分支；保有序比较、加法次序和非单位结果。
    let rotation = if z.z <= 0.0 {
        let difference = y.y - x.x;
        if difference <= 0.0 {
            let t = (1.0 - z.z) - difference;
            let scale = 0.5 / t.sqrt();
            glam::Quat::from_xyzw(t * scale, (x.y + y.x) * scale,
                (x.z + z.x) * scale, (y.z - z.y) * scale)
        } else {
            let t = difference + (1.0 - z.z);
            let scale = 0.5 / t.sqrt();
            glam::Quat::from_xyzw((x.y + y.x) * scale, t * scale,
                (y.z + z.y) * scale, (z.x - x.z) * scale)
        }
    } else {
        let sum = y.y + x.x;
        if sum <= 0.0 {
            let t = (1.0 + z.z) - sum;
            let scale = 0.5 / t.sqrt();
            glam::Quat::from_xyzw((x.z + z.x) * scale, (y.z + z.y) * scale,
                t * scale, (x.y - y.x) * scale)
        } else {
            let t = sum + (1.0 + z.z);
            let scale = 0.5 / t.sqrt();
            glam::Quat::from_xyzw((y.z - z.y) * scale, (z.x - x.z) * scale,
                (x.y - y.x) * scale, scale * t)
        }
    };
    let mut position: Vec3 = transform.translation.into();
    if offset_forward {
        // 原0x3e3851ec，不能从已归一化的四元数重建forward。
        position += Vec3::from(z) * f32::from_bits(0x3e38_51ec);
    }
    (rotation, position)
}

/// 原碰撞描边只保存线点、球半径或显式None。
#[derive(Clone, Debug, PartialEq)]
pub enum DecoratorCollisionOutline {
    Lines(Vec<Vec3>),
    Sphere(f32),
    None,
}

impl DecoratorCollisionOutline {
    /// B140b33920/476B：线点使用完整仿射变换，球仅使用平移且半径不缩放。
    /// 显式None没有显示副作用；空线点仍须提交原调试线请求。
    pub fn display(
        &self,
        transform: &glam::Affine3A,
        stream: &mut crate::systems::ui::signifiers::signifier_create::UiSignifierStream,
        dashed: bool,
    ) {
        use crate::utils::debug_viz::{DebugColor, DebugLine, DebugSphere};
        match self {
            Self::Lines(points) => {
                let points = collision_outline_points(points, transform);
                DebugLine { points, colors: None, color: DebugColor::White,
                    line_width: 2.0, ui_pass: true, dashed }.show(stream);
            }
            Self::Sphere(radius) => {
                DebugSphere { color: DebugColor::White, radius: *radius,
                    center: transform.translation.into(), alpha: 1.0, solid: true }.show(stream);
            }
            Self::None => {}
        }
    }
}

fn collision_outline_points(points: &[Vec3], transform: &glam::Affine3A) -> Vec<Vec3> {
    let x = transform.matrix3.x_axis;
    let y = transform.matrix3.y_axis;
    let z = transform.matrix3.z_axis;
    points.iter().map(|point| {
        // 原b33a4f..5f先y+x，再z加该和，最后平移；不借机滤除非有限输入。
        (z * point.z + (y * point.y + x * point.x) + transform.translation).into()
    }).collect()
}

#[cfg(test)]
mod collision_display_tests {
    use super::*;
    use crate::systems::ui::signifiers::signifier_create::UiSignifierStream;
    use glam::{Affine3A, Vec3A};

    #[test]
    fn full_affine_basis_preserves_point_order_and_nonuniform_scale() {
        let transform = Affine3A::from_cols(Vec3A::new(2.0, 1.0, 0.0),
            Vec3A::new(0.0, 3.0, 1.0), Vec3A::new(1.0, 0.0, 4.0), Vec3A::new(5.0, 6.0, 7.0));
        assert_eq!(collision_outline_points(&[Vec3::new(1.0, 2.0, 3.0), Vec3::ZERO], &transform),
            [Vec3::new(10.0, 13.0, 21.0), Vec3::new(5.0, 6.0, 7.0)]);
    }

    #[test]
    fn explicit_none_has_no_factory_and_nonfinite_points_are_not_dropped() {
        let mut stream = UiSignifierStream::default();
        DecoratorCollisionOutline::None.display(&Affine3A::IDENTITY, &mut stream, true);
        assert!(stream.take_entries().is_empty());
        let points = collision_outline_points(&[Vec3::new(f32::NAN, 0.0, 0.0)], &Affine3A::IDENTITY);
        assert_eq!(points.len(), 1);
        assert!(points[0].is_nan());
    }

    #[test]
    fn empty_lines_still_submit_original_factory_request() {
        let mut stream = UiSignifierStream::default();
        DecoratorCollisionOutline::Lines(Vec::new()).display(&Affine3A::IDENTITY, &mut stream, false);
        // 空点仍先提交原工厂；实际网格唤醒时沿严格资源/几何合同处理。
        assert_eq!(stream.take_entries().len(), 1);
    }
}

#[cfg(test)]
mod main_outline_display_tests {
    use super::*;
    use crate::{
        asset_libraries::{mesh_library::{AssetMeshLibrary, MeshAsset},
            shader_library::{AssetShaderLibrary, SharedAssetShaderLibrary},
            transient_mesh::{TransientMesh, delete_dropped_transient_meshes},
            vao_library::AssetVAOLibrary},
        render::{mesh::Mesh, shader::RasterShaderProgram, test_device::TestDevice,
            pass_builder::draw_list::DrawTopology},
        resources::{time::GameTime, walls::decorator::{DecoratorId, DecoratorRank}},
        systems::ui::{resources::{ui_state::{UiState, UiInteraction, WidgetId}, widget_params::WidgetParams},
            signifiers::{signifier_create::{UiSignifierStream, UiSignifierEntry, signifier_create, SignifierId},
                signifier_mark_for_delete::{signfier_mark_for_delete, despawn_marked},
                animation::{SignifierAnimation, signifier_animation_t, signifier_animation}}},
        utils::components::transform::Transform,
    };
    use bevy_ecs::{prelude::*, system::RunSystemOnce, world::CommandQueue};
    use glam::{Affine3A, Vec3A};
    use std::sync::Arc;

    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../reports/module-recovery-2026-10-23/m01-outline-pose-native-vectors.rs"));

    fn libraries() -> (AssetMeshLibrary, AssetShaderLibrary) {
        let meshes = AssetMeshLibrary::new();
        for name in ["outline", "temp_sphere"] {
            meshes.add(MeshAsset::new(Mesh::new(name), Some(name.to_owned())));
        }
        let shaders = AssetShaderLibrary::new();
        shaders.add_raster(RasterShaderProgram::new_draw("vertex_color_shader", Default::default(), &TestDevice(0)), false);
        (meshes, shaders)
    }

    fn wake(outline: &DecoratorOutline, transform: &Affine3A, forward: bool, animate: bool,
        meshes: &AssetMeshLibrary, shaders: &AssetShaderLibrary, world: &mut World) -> Entity {
        let mut stream = UiSignifierStream::default();
        outline.display(transform, &mut stream, DecoratorId(41), DecoratorRank(2), forward, animate);
        let UiSignifierEntry::Factory(factory) = stream.take_entries().pop().unwrap() else { panic!("必须是真实owned工厂"); };
        let mut queue = CommandQueue::default();
        let entities = factory.wake(meshes, shaders, &mut Commands::new(&mut queue, world));
        queue.apply(world);
        assert_eq!(entities.len(), 1);
        entities[0]
    }

    #[test]
    fn outline_pose_matches_128_original_matrix_branch_and_forward_vectors() {
        for &(matrix, forward, expected_rotation, expected_position) in OUTLINE_POSE_NATIVE {
            let matrix = matrix.map(f32::from_bits);
            let affine = Affine3A::from_cols(Vec3A::from_slice(&matrix[..3]), Vec3A::from_slice(&matrix[4..7]),
                Vec3A::from_slice(&matrix[8..11]), Vec3A::from_slice(&matrix[12..15]));
            let (rotation, position) = decorator_outline_pose(&affine, forward);
            assert_eq!(rotation.to_array().map(f32::to_bits), expected_rotation, "matrix={matrix:?}");
            assert_eq!(position.to_array().map(f32::to_bits), expected_position);
        }
    }

    #[test]
    fn exact_original_name_rank_and_explicit_none_keep_owned_factory_contract() {
        let mut stream = UiSignifierStream::default();
        for outline in [DecoratorOutline::Mesh("outline".into()), DecoratorOutline::Lines(vec![Vec3::ZERO]), DecoratorOutline::Sphere(2.0), DecoratorOutline::None] {
            outline.display(&Affine3A::IDENTITY, &mut stream, DecoratorId(41), DecoratorRank(2), false, true);
        }
        assert_eq!(stream.committed_len(), 3);
        for entry in stream.take_entries() {
            let UiSignifierEntry::Factory(factory) = entry else { panic!("不能降成raw qword"); };
            assert_eq!(factory.name(), Some("move_decorator_41_Vec3(0.0, 0.0, 0.0)_DecoratorRank { x: 2 }"));
            assert!(factory.has_closure());
        }
    }

    #[test]
    fn mesh_and_lines_use_local_geometry_original_rotation_and_line_width() {
        let (mut meshes, shaders) = libraries();
        let affine = Affine3A::from_cols(Vec3A::new(2.0, 1.0, 0.0), Vec3A::new(0.0, 3.0, 1.0),
            Vec3A::new(1.0, 0.0, 4.0), Vec3A::new(5.0, 6.0, 7.0));
        let expected = decorator_outline_pose(&affine, true);
        let points = vec![Vec3::new(1.0, 2.0, 3.0), Vec3::new(-4.0, 5.0, 6.0)];
        for (outline, transient) in [(DecoratorOutline::Mesh("outline".into()), false), (DecoratorOutline::Lines(points.clone()), true)] {
            let mut world = World::new();
            let entity = wake(&outline, &affine, true, true, &meshes, &shaders, &mut world);
            let pose = world.get::<Transform>(entity).unwrap();
            assert_eq!(pose.rotation, expected.0);
            assert_eq!(pose.translation, expected.1);
            assert_eq!(pose.scale, Vec3::ZERO);
            assert_eq!(*world.get::<DrawTopology>(entity).unwrap(), DrawTopology::Alt(3.0));
            assert_eq!(world.get::<SignifierAnimation>(entity).unwrap().duration, 0.0);
            assert_eq!(world.get::<TransientMesh>(entity).is_some(), transient);
            let handle = *world.get::<crate::asset_libraries::Handle<Mesh>>(entity).unwrap();
            if transient { assert_eq!(meshes.get(handle).unwrap().positions_without_indices(), points.as_slice()); }
            world.despawn(entity);
            delete_dropped_transient_meshes(&TestDevice(0), &mut meshes, &mut AssetVAOLibrary::new());
            assert_eq!(meshes.get(handle).is_none(), transient);
        }
    }

    #[test]
    fn sphere_uses_named_radius_and_independent_animation_gate_without_line_topology() {
        let (meshes, shaders) = libraries();
        for animate in [false, true] {
            let mut world = World::new();
            let affine = Affine3A::from_scale_rotation_translation(Vec3::new(2.0, 3.0, 4.0), glam::Quat::from_rotation_y(0.5), Vec3::new(1.0, 2.0, 3.0));
            let entity = wake(&DecoratorOutline::Sphere(-2.0), &affine, false, animate, &meshes, &shaders, &mut world);
            assert_eq!(world.get::<Transform>(entity).unwrap().rotation, glam::Quat::IDENTITY);
            assert_eq!(world.get::<Transform>(entity).unwrap().translation, Vec3::new(1.0, 2.0, 3.0));
            assert_eq!(*world.get::<crate::asset_libraries::Handle<Mesh>>(entity).unwrap(), meshes.get_handle_by_name("temp_sphere"));
            assert!(world.get::<DrawTopology>(entity).is_none());
            assert!(world.get::<TransientMesh>(entity).is_none());
            let animation = world.get::<SignifierAnimation>(entity).unwrap();
            assert_eq!(animation.value, -2.0);
            assert_eq!(animation.duration, if animate { 0.1 } else { 0.0 });
        }
    }

    #[test]
    fn actual_stream_create_animation_retirement_reclaims_factory_entity_and_transient_mesh() {
        let (meshes, shaders) = libraries();
        let mut stream = UiSignifierStream::default();
        DecoratorOutline::Lines(vec![Vec3::ZERO, Vec3::X]).display(&Affine3A::IDENTITY, &mut stream,
            DecoratorId(41), DecoratorRank(2), false, true);
        let UiSignifierEntry::Factory(factory) = stream.take_entries().pop().unwrap() else { panic!("factory"); };
        let weak = Arc::downgrade(factory.closure.as_ref().unwrap());
        stream.write_factory(&factory);
        drop(factory);
        let mut ui = UiState::new(UiInteraction { native_tag: 0, native_phase: 0, widget: WidgetId(0) });
        ui.replace_interaction_full(UiInteraction { native_tag: 1, native_phase: 0, widget: WidgetId(7) }, Some(Arc::new(WidgetParams::default())), [0; 56]);
        let mut world = World::new();
        world.insert_resource(meshes); world.insert_resource(SharedAssetShaderLibrary(Arc::new(shaders)));
        world.insert_resource(ui); world.insert_resource(stream);
        world.insert_resource(GameTime { delta: 0.1, ..Default::default() });
        world.run_system_once(signifier_create).unwrap();
        let entity = world.query_filtered::<Entity, With<SignifierId>>().single(&world).unwrap();
        let handle = *world.get::<crate::asset_libraries::Handle<Mesh>>(entity).unwrap();
        assert_eq!(world.resource::<UiSignifierStream>().committed_len(), 0);
        assert!(weak.upgrade().is_some());
        world.run_system_once(signifier_animation_t).unwrap();
        world.run_system_once(signifier_animation).unwrap();
        assert_eq!(world.get::<Transform>(entity).unwrap().scale, Vec3::ONE);
        world.run_system_once(signifier_create).unwrap();
        assert_eq!(world.query_filtered::<Entity, With<SignifierId>>().iter(&world).count(), 1);
        world.resource_mut::<UiState>().replace_interaction_full(UiInteraction { native_tag: 0, native_phase: 0, widget: WidgetId(0) }, None, [0; 56]);
        assert!(weak.upgrade().is_none());
        world.run_system_once(signfier_mark_for_delete).unwrap();
        world.run_system_once(signifier_animation_t).unwrap();
        world.run_system_once(despawn_marked).unwrap();
        assert!(world.get_entity(entity).is_err());
        let mut meshes = world.remove_resource::<AssetMeshLibrary>().unwrap();
        delete_dropped_transient_meshes(&TestDevice(0), &mut meshes, &mut AssetVAOLibrary::new());
        assert!(meshes.get(handle).is_none());
    }

    #[test]
    fn required_mesh_shader_and_empty_line_fail_at_real_consumer_not_display() {
        let (meshes, shaders) = libraries();
        for (outline, missing_shader) in [(DecoratorOutline::Mesh("absent".into()), false),
            (DecoratorOutline::Lines(Vec::new()), false), (DecoratorOutline::Sphere(1.0), true)] {
            let mut stream = UiSignifierStream::default();
            outline.display(&Affine3A::IDENTITY, &mut stream, DecoratorId(41), DecoratorRank(0), false, false);
            assert_eq!(stream.committed_len(), 1);
            let UiSignifierEntry::Factory(factory) = stream.take_entries().pop().unwrap() else { panic!("factory"); };
            let empty_shaders = AssetShaderLibrary::new();
            let mut world = World::new(); let mut queue = CommandQueue::default();
            let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                factory.wake(&meshes, if missing_shader { &empty_shaders } else { &shaders }, &mut Commands::new(&mut queue, &world));
            }));
            assert!(result.is_err());
        }
    }
}
