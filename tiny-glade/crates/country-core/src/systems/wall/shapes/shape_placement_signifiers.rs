//! 原 B 形状放置指示器：权威放置 → 完整曲线切片/高度映射 → 真实工厂。
//! 原 consumer 1408dec70/2628B，preview 1408dfc60/2670B；不把完整曲线
//! 降为一个锚点或数字 raw 请求。保留现有工厂、资源与 renderer 架构。

use super::shape_placement::ShapePlacement;
use crate::resources::{
    terrain_height::{TerrainHeightSsbo, TerrainHeightsData},
    walls::{public_wall_state::PublicWallState, public_walls::PublicWalls},
};
use crate::systems::{
    mode_manager::{AppMode, WallShape},
    ui::signifiers::{
        signifier_create::UiSignifierStream,
        signifier_factory::{SignifierEntityBuilder, SignifierFactory},
    },
};
use ::utils::curve::curve_slice::{circular_slice_cached, slice_cached_into};
use ::utils::curve::{Curve3, Curve3XZ, CurveCoordSemantic3D};
use bevy_ecs::prelude::*;
use glam::{Vec2, Vec3};
use std::sync::Arc;

const CIRCLE_EXPAND: f32 = 0.315;
const RECTANGLE_EXPAND: f32 = 0.63;
const PREVIEW_LINE_LEN: f32 = 4.5;

/// 原返回联合载荷包含锚点与完整映射点列；失败只影响曲线工厂，已发布球工厂保留。
#[derive(Debug)]
pub struct PreviewSnappingLine {
    pub anchor: Vec3,
    pub points: Vec<Vec3>,
}

fn wrapped_u(value: f32) -> f32 {
    assert!(!value.is_nan(), "原环绕参数不能是 NaN");
    let remainder = value % 1.0;
    if remainder < 0.0 {
        remainder + 1.0
    } else {
        remainder
    }
}

fn sliced_curve(state: &PublicWallState, position: Vec2, length: f32) -> Option<Curve3> {
    if state.is_premade_shape() {
        let curve: Curve3XZ = if let Some(mut circle) = state.is_circle() {
            circle.radius += CIRCLE_EXPAND;
            circle.as_curve3(0.5, 0.0)
        } else {
            let mut rectangle = state.is_rectangle().unwrap();
            rectangle.size += Vec2::splat(RECTANGLE_EXPAND);
            rectangle.as_curve3(0.0).crashy_resample_per_segment(0.5)
        };
        let u = curve.get_approx_u_from_pos(position);
        let half = ::utils::scalar::clamp01(length / curve.length) * 0.5;
        let start = wrapped_u(u - half);
        let end = wrapped_u(u + half);
        return curve
            .circular_slice(start, end)
            .ok()
            .map(Curve3XZ::into_curve3);
    }
    let geometry = &state.geometry;
    let u = ::utils::curve::approx_curve_u(&geometry.outline, &geometry.outline_u, position, |p| {
        Vec2::new(p[0], p[2])
    });
    let half = ::utils::scalar::clamp01(length / geometry.mapped_curve_length) * 0.5;
    let start = u - half;
    let end = u + half;
    if state.shape().tag() == 2 {
        // 原 boundless 参数先检查 finite，再夹到 [0,1]；不能先把 NaN 化为0。
        assert!(
            start.is_finite() && end.is_finite(),
            "原开放曲线参数必须 finite"
        );
        let mut points = Vec::new();
        slice_cached_into(
            &geometry.outline,
            &geometry.outline_u,
            ::utils::scalar::clamp01(start),
            ::utils::scalar::clamp01(end),
            |p| Vec3::from_array(*p),
            &mut points,
        );
        Curve3::try_new_from_points(points, false).ok()
    } else {
        circular_slice_cached::<_, _, CurveCoordSemantic3D>(
            &geometry.outline,
            &geometry.outline_u,
            wrapped_u(start),
            wrapped_u(end),
            |p| Vec3::from_array(*p),
        )
        .ok()
    }
}

/// 保留其他既有 snapping 消费者的锚点调用面，使用原完整切片求中点。
/// 锚点来自映射前曲线；其Y不能替换为当前工具的地形/高度映射输出。
pub fn preview_snapping_line(state: &PublicWallState, position: Vec2) -> Option<Vec3> {
    sliced_curve(state, position, PREVIEW_LINE_LEN).map(|curve| curve.get_pos_at_u(0.5))
}

/// 原7参数 preview 的安全调用面：原始切片中点与映射后的完整点列同时保留。
pub fn preview_snapping_line_data(
    state: &PublicWallState,
    position: Vec3,
    terrain: &TerrainHeightsData<TerrainHeightSsbo>,
    length: f32,
    elevation: f32,
    placement_y: f32,
) -> Option<PreviewSnappingLine> {
    let curve = sliced_curve(state, Vec2::new(position.x, position.z), length)?;
    let anchor = curve.get_pos_at_u(0.5);
    let points = curve
        .points
        .into_iter()
        .map(|mut point| {
            if elevation != 0.0 {
                point.y = point.y.max(placement_y);
            } else if state.is_premade_shape() {
                point.y = terrain.sample_at_world_xz(point.x, point.z);
            }
            point
        })
        .collect();
    Some(PreviewSnappingLine { anchor, points })
}

fn sphere_factory(name_position: Vec3, position: Vec3) -> SignifierFactory {
    SignifierFactory::from_closure(
        &format!("snap_signifier_{name_position:?}"),
        Arc::new(move |meshes, shaders, commands| {
            vec![
                SignifierEntityBuilder::from_mesh_name("temp_sphere")
                    .disable_default_animation()
                    .animation_duration(0.0)
                    .position(position)
                    .scale(0.2)
                    .custom_shader("vertex_color_shader")
                    .build(meshes, shaders, commands),
            ]
        }),
    )
}

fn curve_factory(position: Vec3, points: Vec<Vec3>) -> SignifierFactory {
    SignifierFactory::from_closure(
        &format!("snap_signifier_curve_{position:?}"),
        Arc::new(move |meshes, shaders, commands| {
            let entity = SignifierEntityBuilder::from_points(points.clone())
                .disable_default_animation()
                .animation_duration(0.0)
                .custom_shader("vertex_color_shader")
                .build(meshes, shaders, commands);
            commands.entity(entity).insert(
                crate::render::pass_builder::draw_list::DrawTopology::Alt(2.2),
            );
            vec![entity]
        }),
    )
}

/// 原5参数系统；当前流通过 ResMut 显式借用，取代原 AppendOnlyVec 内部可变性。
pub fn shape_placement_signifiers(
    mode: Res<AppMode>,
    placement: Res<ShapePlacement>,
    mut stream: ResMut<UiSignifierStream>,
    public: Res<PublicWalls>,
    terrain: Res<TerrainHeightsData<TerrainHeightSsbo>>,
) {
    if !matches!(
        *mode,
        AppMode::Wall(WallShape::Circle | WallShape::Rectangle)
    ) || placement.flat_roof().is_none()
    {
        return;
    }
    if placement.surface() == 2 {
        // 原 roof surface 的球位置是 +0xc，工厂身份使用 +0x44。
        // 真实 producer 当前两者相同，安全接口仍保留原读取合同。
        let bytes = placement.observed();
        let at = |o| f32::from_le_bytes(bytes[o..o + 4].try_into().unwrap());
        let position = Vec3::new(at(0x0c), at(0x10), at(0x14));
        let base = placement.base_position();
        let factory = sphere_factory(base, position);
        stream.write_factory(&factory);
        return;
    }
    if placement.surface() != 1 {
        return;
    }
    let state = public
        .get(placement.wall_id().unwrap())
        .expect("原吸附墙必须仍有有效公共状态");
    let mut base = placement.base_position();
    if state.is_premade_shape() {
        let curve = state.simplified_curve_2d();
        let coord =
            ::utils::curve::approx_curve_coord(&curve.points, Vec2::new(base.x, base.z), |p| {
                Vec2::from_array(*p)
            });
        let a = curve.points[coord.segment];
        let b = curve.points[coord.segment + 1];
        let dx = b[0] - a[0];
        let dz = b[1] - a[1];
        let inv = 1.0 / (dz * dz + dx * dx).sqrt();
        base += Vec3::new(
            dz * inv * (-RECTANGLE_EXPAND) * 0.5,
            0.0,
            dx * inv * RECTANGLE_EXPAND * 0.5,
        );
    }
    stream.write_factory(&sphere_factory(base, base));
    if let Some(preview) = preview_snapping_line_data(
        state,
        base,
        &terrain,
        PREVIEW_LINE_LEN,
        placement.elevation(),
        placement.position().y,
    ) {
        stream.write_factory(&curve_factory(base, preview.points));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::resources::{
        roofs::geometry::surface::Curve2,
        walls::{
            WallState, publication_fields::WallShapeSnapshot,
            publication_source::TerrainHeightGrid512,
        },
    };
    fn state(input: &serde_json::Value) -> PublicWallState {
        let shape = input["shape"].as_u64().unwrap_or(2) as u32;
        let points: Vec<[f32; 3]> = input
            .get("points")
            .map(|v| serde_json::from_value(v.clone()).unwrap())
            .unwrap_or_else(|| vec![[0., 0., 0.], [4., 2., 0.], [8., 0., 0.]]);
        let us: Vec<f32> = input
            .get("us")
            .map(|v| serde_json::from_value(v.clone()).unwrap())
            .unwrap_or_else(|| vec![0., 0.5, 1.]);
        let mut authority = WallState::new(
            Curve2 {
                points: points.iter().map(|p| [p[0], p[2]]).collect(),
                points_u: us.clone(),
                length: 8.,
            },
            3.,
            0,
            0,
            false,
        )
        .snapshot()
        .clone();
        let mut bytes = [0; 28];
        bytes[..4].copy_from_slice(&shape.to_le_bytes());
        let fields: Vec<f32> = if shape == 1 {
            vec![1., 0., 4., 0., 8., 6.]
        } else {
            vec![4., 0., 4., 0.]
        };
        for (i, v) in fields.iter().enumerate() {
            bytes[4 + i * 4..8 + i * 4].copy_from_slice(&v.to_le_bytes());
        }
        authority.source.shape = WallShapeSnapshot::from_observed(bytes);
        let terrain = TerrainHeightGrid512 {
            encoded: vec![0; 512 * 512].into(),
            minimum_y: 0.,
            maximum_y: 0.,
            sampling_fix: false,
        };
        let mut state =
            PublicWallState::collect(&WallState::from_snapshot(authority, 0), &terrain).unwrap();
        state.geometry.outline = Arc::new(points);
        state.geometry.outline_u = us;
        state.geometry.mapped_curve_length =
            input["mapped_length"].as_f64().unwrap_or(80f64.sqrt()) as f32;
        state
    }
    fn terrain() -> TerrainHeightsData<TerrainHeightSsbo> {
        let mut terrain = TerrainHeightsData::default();
        for z in 0..512 {
            for x in 0..512 {
                terrain.heights_mut()[z * 512 + x] = ((x * 137 + z * 71) & 65535) as u16;
            }
        }
        terrain
    }
    #[test]
    fn preview_entire_point_list_and_anchor_match_original_2670_bytes() {
        let rows: Vec<serde_json::Value> = serde_json::from_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../tests/fixtures/native/toolbar-placement-signifier-preview.json"
        )))
        .unwrap();
        let terrain = terrain();
        for (index, row) in rows.into_iter().enumerate() {
            let input = &row["input"];
            let state = state(input);
            let position = input["position"]
                .as_array()
                .map(|v| {
                    Vec3::new(
                        v[0].as_f64().unwrap() as f32,
                        v[1].as_f64().unwrap() as f32,
                        v[2].as_f64().unwrap() as f32,
                    )
                })
                .unwrap_or(Vec3::new(2., 3., 1.));
            let at =
                |key: &str, default: f32| input[key].as_f64().map(|v| v as f32).unwrap_or(default);
            let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                preview_snapping_line_data(
                    &state,
                    position,
                    &terrain,
                    at("length", 4.5),
                    at("elevation", 0.),
                    at("placement_y", 3.),
                )
            }));
            if row.get("panic").is_some() {
                assert!(result.is_err(), "case {index}: {input}");
                continue;
            }
            let result = result.unwrap_or_else(|_| panic!("candidate panic {index}: {input}"));
            if row["none"].as_bool() == Some(true) {
                assert!(result.is_none(), "case {index}");
                continue;
            }
            let result = result.unwrap_or_else(|| panic!("candidate None {index}: {input}"));
            let expected_anchor: [u32; 3] =
                std::array::from_fn(|i| row["anchor"][i].as_u64().unwrap() as u32);
            assert_eq!(
                result.anchor.to_array().map(f32::to_bits),
                expected_anchor,
                "anchor {index}: {input}"
            );
            let expected_points: Vec<[u32; 3]> = row["points"]
                .as_array()
                .unwrap()
                .iter()
                .map(|p| std::array::from_fn(|i| p[i].as_u64().unwrap() as u32))
                .collect();
            assert_eq!(
                result
                    .points
                    .iter()
                    .map(|p| p.to_array().map(f32::to_bits))
                    .collect::<Vec<_>>(),
                expected_points,
                "points {index}: {input}"
            );
        }
    }
    fn libraries() -> (
        crate::asset_libraries::mesh_library::AssetMeshLibrary,
        crate::asset_libraries::shader_library::AssetShaderLibrary,
    ) {
        use crate::{
            asset_libraries::mesh_library::{AssetMeshLibrary, MeshAsset},
            asset_libraries::shader_library::AssetShaderLibrary,
            render::{mesh::Mesh, shader::RasterShaderProgram, test_device::TestDevice},
        };
        let meshes = AssetMeshLibrary::new();
        meshes.add(MeshAsset::new(
            Mesh::new("sphere fixture"),
            Some("temp_sphere".into()),
        ));
        let shaders = AssetShaderLibrary::new();
        shaders.add_raster(
            RasterShaderProgram::new_draw(
                "vertex_color_shader",
                Default::default(),
                &TestDevice(1),
            ),
            false,
        );
        (meshes, shaders)
    }
    fn public(state: PublicWallState) -> (crate::resources::walls::InnerWalls, PublicWalls) {
        use crate::resources::walls::{InnerWalls, WallId};
        let mut inner = InnerWalls::new();
        inner.new_wall_with_id(WallId(42), state.authority.clone());
        let terrain = TerrainHeightGrid512 {
            encoded: vec![0; 512 * 512].into(),
            minimum_y: 0.,
            maximum_y: 0.,
            sampling_fix: false,
        };
        let mut public = PublicWalls::default();
        public.consume_with_paths(&mut inner, &terrain, None, &crate::resources::walls::public_walls::WallPublicationDirty::default(), &crate::resources::walls::public_walls::WallSpatialHash::default());
        *public.get_mut(WallId(42)).unwrap() = state;
        (inner, public)
    }
    fn placement(input: &serde_json::Value) -> ShapePlacement {
        let integer = |key: &str, default: u64| input[key].as_u64().unwrap_or(default);
        let vector = |key: &str, default: [f32; 3]| {
            input
                .get(key)
                .map(|v| serde_json::from_value::<[f32; 3]>(v.clone()).unwrap())
                .unwrap_or(default)
        };
        let mut bytes = [0; 80];
        bytes[8..12].copy_from_slice(&(integer("surface", 1) as u32).to_le_bytes());
        bytes[44] = integer("flat", 0) as u8;
        bytes[32..40].copy_from_slice(&42u64.to_le_bytes());
        for (o, p) in [
            (12, vector("roof_point", [2., 3., 1.])),
            (48, vector("position", [2., 3., 1.])),
            (68, vector("base", [2., 3., 1.])),
        ] {
            for (i, x) in p.into_iter().enumerate() {
                bytes[o + i * 4..o + i * 4 + 4].copy_from_slice(&x.to_le_bytes());
            }
        }
        bytes[60..64]
            .copy_from_slice(&(input["elevation"].as_f64().unwrap_or(0.) as f32).to_le_bytes());
        ShapePlacement::from_observed(bytes)
    }
    #[test]
    fn original_consumer_and_both_woken_factories_match_names_geometry_and_visual_contract() {
        use crate::{
            asset_libraries::{
                Handle,
                transient_mesh::{TransientMesh, delete_dropped_transient_meshes},
                vao_library::AssetVAOLibrary,
            },
            render::{mesh::Mesh, pass_builder::draw_list::DrawTopology, test_device::TestDevice},
            systems::{
                render_loop::draw_passes::draw_lists::{DontCastShadow, UiPass},
                ui::signifiers::{
                    animation::SignifierAnimation, signifier_create::UiSignifierEntry,
                },
            },
            utils::components::transform::Transform,
        };
        use bevy_ecs::{system::RunSystemOnce, world::CommandQueue};
        let rows: Vec<serde_json::Value> = serde_json::from_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../tests/fixtures/native/toolbar-placement-signifier-factories.json"
        )))
        .unwrap();
        for (index, row) in rows.into_iter().enumerate() {
            let input = &row["input"];
            let mut world = World::new();
            let (_, mut public) = public(state(input));
            if input["public"].as_bool() == Some(false) {
                public = PublicWalls::default();
            }
            let mode = if input["mode"].as_u64() == Some(5) {
                AppMode::Path
            } else {
                AppMode::Wall(match input["tool"].as_u64() {
                    Some(0) => WallShape::Freeform,
                    Some(2) => WallShape::Rectangle,
                    _ => WallShape::Circle,
                })
            };
            world.insert_resource(mode);
            world.insert_resource(placement(input));
            world.insert_resource(public);
            world.insert_resource(terrain());
            world.init_resource::<UiSignifierStream>();
            let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                world.run_system_once(shape_placement_signifiers).unwrap()
            }));
            if !row["panic"].is_null() {
                assert!(result.is_err(), "original panic {index}");
                continue;
            }
            assert!(result.is_ok(), "candidate panic {index}");
            let factories = world.resource_mut::<UiSignifierStream>().take_entries();
            let expected = row["factories"].as_array().unwrap();
            assert_eq!(factories.len(), expected.len(), "factory count {index}");
            let (mut meshes, shaders) = libraries();
            let mut vaos = AssetVAOLibrary::new();
            for (i, entry) in factories.into_iter().enumerate() {
                let UiSignifierEntry::Factory(factory) = entry else {
                    panic!("不允许 raw 数字请求");
                };
                assert_eq!(
                    factory.name(),
                    expected[i]["name"].as_str(),
                    "identity {index}/{i}"
                );
                let mut queue = CommandQueue::default();
                let entities =
                    factory.wake(&meshes, &shaders, &mut Commands::new(&mut queue, &world));
                queue.apply(&mut world);
                assert_eq!(entities.len(), 1);
                let entity = entities[0];
                let native = &row["builders"][i];
                let expected_pos: [u32; 3] =
                    std::array::from_fn(|j| native["position"][j].as_u64().unwrap() as u32);
                let transform = world.get::<Transform>(entity).unwrap();
                assert_eq!(
                    transform.translation.to_array().map(f32::to_bits),
                    expected_pos
                );
                assert_eq!(transform.rotation, glam::Quat::IDENTITY);
                let scale = if native["scale_tag"].as_u64() == Some(1) {
                    f32::from_bits(native["scale_bits"].as_u64().unwrap() as u32)
                } else {
                    1.
                };
                assert_eq!(transform.scale, Vec3::splat(scale));
                let animation = world.get::<SignifierAnimation>(entity).unwrap();
                let flags = native["animation"][0].as_u64().unwrap();
                assert_eq!(animation.flag_8, (flags & 255) as u8);
                assert_eq!(animation.flag_9, ((flags >> 8) & 255) as u8);
                assert_eq!(
                    animation.value.to_bits(),
                    native["animation"][1].as_u64().unwrap() as u32
                );
                assert_eq!(
                    animation.duration.to_bits(),
                    native["duration_bits"].as_u64().unwrap() as u32
                );
                assert!(world.get::<UiPass>(entity).is_some());
                assert!(world.get::<DontCastShadow>(entity).is_some());
                let handle = *world.get::<Handle<Mesh>>(entity).unwrap();
                if expected[i]["kind"].as_str() == Some("sphere") {
                    assert_eq!(handle, meshes.get_handle_by_name("temp_sphere"));
                    assert!(world.get::<TransientMesh>(entity).is_none());
                    assert!(world.get::<DrawTopology>(entity).is_none());
                } else {
                    let expected_points: Vec<[u32; 3]> = native["mesh"]["points"]
                        .as_array()
                        .unwrap()
                        .iter()
                        .map(|p| std::array::from_fn(|j| p[j].as_u64().unwrap() as u32))
                        .collect();
                    assert_eq!(
                        meshes
                            .get(handle)
                            .unwrap()
                            .positions_without_indices()
                            .iter()
                            .map(|p| p.to_array().map(f32::to_bits))
                            .collect::<Vec<_>>(),
                        expected_points,
                        "mesh {index}"
                    );
                    assert!(world.get::<TransientMesh>(entity).is_some());
                    let Some(DrawTopology::Alt(width)) = world.get::<DrawTopology>(entity) else {
                        panic!("原曲线线宽缺失")
                    };
                    assert_eq!(
                        width.to_bits(),
                        native["draw_topology"][1].as_u64().unwrap() as u32
                    );
                }
                world.despawn(entity);
                delete_dropped_transient_meshes(&TestDevice(0), &mut meshes, &mut vaos);
                assert_eq!(meshes.len(), 1, "瞬态曲线回收，命名球网格仍由库拥有");
            }
        }
    }
    #[test]
    fn real_placement_factory_capture_repeat_and_deleted_wall_reclaim_meshes() {
        use crate::{
            asset_libraries::{
                mesh_library::AssetMeshLibrary,
                shader_library::SharedAssetShaderLibrary,
                transient_mesh::{TransientMesh, delete_dropped_transient_meshes},
                vao_library::AssetVAOLibrary,
            },
            render::test_device::TestDevice,
            resources::{
                cursor_terrain_raycast::CursorTerrainRaycast,
                mouse_terrain_raycast::CameraCursorRay, onboarding::NotifyHintSystemSnapping,
                walls::WallId,
            },
            systems::{
                audio::PlayWallUiSound,
                backend::input::mod_key::Modifiers,
                ui::{
                    resources::{
                        ui_state::{UiInteraction, UiState, WidgetId},
                        widget_params::WidgetParams,
                    },
                    signifiers::{
                        signifier_create::{SignifierId, signifier_create},
                        signifier_mark_for_delete::{despawn_marked, signfier_mark_for_delete},
                    },
                },
                wall::{
                    shapes::shape_placement::{
                        DontRecalculateShapePlacement, SnapMode, shape_placement_params,
                    },
                    wall_state::wall_spatial_hash::{WallSpatialHash, WallSpatialSegment},
                },
            },
        };
        fn capture(world: &mut World) {
            world.resource_mut::<UiState>().replace_interaction_full(
                UiInteraction {
                    native_tag: 1,
                    native_phase: 1,
                    widget: WidgetId(91),
                },
                Some(Arc::new(WidgetParams::default())),
                [0; 56],
            );
        }
        let (mut inner, public) = public(state(&serde_json::json!({"shape":1})));
        let terrain_grid = TerrainHeightGrid512 {
            encoded: vec![0; 512 * 512].into(),
            minimum_y: 0.,
            maximum_y: 0.,
            sampling_fix: false,
        };
        let mut spatial = WallSpatialHash::default();
        for endpoints in [[[0., 0.], [4., 0.]], [[4., 0.], [4., 4.]]] {
            spatial.add_segment(WallSpatialSegment {
                endpoints,
                source_u: [0., 1.],
                relative_heights: [0.; 2],
                terrain_heights: [0.; 2],
                wall: WallId(42),
            });
        }
        let mut world = World::new();
        world.insert_resource(AppMode::Wall(WallShape::Circle));
        world.insert_resource(public);
        world.insert_resource(spatial);
        world.insert_resource(terrain());
        world.insert_resource(CursorTerrainRaycast {
            tag: 1,
            pos: Vec3::new(2., 0., 1.),
            ..Default::default()
        });
        world.init_resource::<ShapePlacement>();
        world.init_resource::<SnapMode>();
        world.init_resource::<Modifiers>();
        world.init_resource::<CameraCursorRay>();
        world.init_resource::<UiSignifierStream>();
        world.init_resource::<Events<PlayWallUiSound>>();
        world.init_resource::<Events<NotifyHintSystemSnapping>>();
        world.init_resource::<Events<DontRecalculateShapePlacement>>();
        world.insert_resource(UiState::at_startup());
        let (meshes, shaders) = libraries();
        world.insert_resource(meshes);
        world.insert_resource(SharedAssetShaderLibrary(Arc::new(shaders)));
        let mut schedule = Schedule::default();
        schedule.add_systems(
            (
                shape_placement_params,
                shape_placement_signifiers,
                signifier_create,
                signfier_mark_for_delete,
                despawn_marked,
            )
                .chain_ignore_deferred(),
        );
        capture(&mut world);
        schedule.run(&mut world);
        assert_eq!(world.query::<&SignifierId>().iter(&world).count(), 2);
        assert_eq!(world.query::<&TransientMesh>().iter(&world).count(), 1);
        assert_eq!(
            world.resource::<AssetMeshLibrary>().len(),
            2,
            "一个命名球网格+一个实际曲线网格"
        );
        let receipt = world
            .query::<&TransientMesh>()
            .iter(&world)
            .next()
            .unwrap()
            .mesh_handle();
        let weak_factory = {
            let params = world.resource::<UiState>().active_params().unwrap();
            Arc::downgrade(params.signifiers[0].closure.as_ref().unwrap())
        };
        capture(&mut world);
        schedule.run(&mut world);
        assert_eq!(world.query::<&SignifierId>().iter(&world).count(), 2);
        assert_eq!(world.resource::<AssetMeshLibrary>().len(), 2);
        assert_eq!(
            world
                .query::<&TransientMesh>()
                .iter(&world)
                .next()
                .unwrap()
                .mesh_handle(),
            receipt,
            "重复帧真实消费者复用实体和网格"
        );
        assert!(
            weak_factory.upgrade().is_none(),
            "上一帧捕获factory应卸载，不能由常驻流持有"
        );
        inner.remove(WallId(42));
        world
            .resource_mut::<PublicWalls>()
            .consume_with_paths(&mut inner, &terrain_grid, None, &crate::resources::walls::public_walls::WallPublicationDirty::default(), &crate::resources::walls::public_walls::WallSpatialHash::default());
        world
            .resource_mut::<WallSpatialHash>()
            .remove_wall_id(WallId(42));
        for _ in 0..2 {
            capture(&mut world);
            schedule.run(&mut world);
        }
        assert_eq!(world.resource::<ShapePlacement>().surface(), 0);
        assert_eq!(world.resource::<SnapMode>().0, 0);
        assert_eq!(world.query::<&SignifierId>().iter(&world).count(), 0);
        assert_eq!(world.query::<&TransientMesh>().iter(&world).count(), 0);
        assert!(
            world
                .resource::<UiState>()
                .active_params()
                .unwrap()
                .signifiers
                .is_empty()
        );
        assert_eq!(world.resource::<UiSignifierStream>().committed_len(), 0);
        let mut meshes = world.remove_resource::<AssetMeshLibrary>().unwrap();
        assert_eq!(
            meshes.transient_meshes().lock().unwrap().as_slice(),
            &[receipt]
        );
        delete_dropped_transient_meshes(&TestDevice(0), &mut meshes, &mut AssetVAOLibrary::new());
        assert!(meshes.get(receipt).is_none());
        assert_eq!(meshes.len(), 1);
        assert!(meshes.get_by_name("temp_sphere").is_some());
    }
}
