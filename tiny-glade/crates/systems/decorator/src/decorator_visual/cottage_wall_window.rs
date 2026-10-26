//! 原 B1407ebfd0 农舍墙窗：只向唯一蓝图 owner 发布完整视觉与业务输出。
//! 设计与原证见本任务 window-producer-root-continuation-v1；不复制原生 ABI。
use std::hash::Hash;

use bevy_ecs::prelude::*;
use country_core::{
    asset_libraries::mesh_library::AssetMeshLibrary,
    resources::{color_id::ColorId, walls::{
        WallId,
        cached_decorator_transforms::{CachedDecoratorTransform, CachedDecoratorTransforms},
        decorator::{DecoratorId, DecoratorInfo},
        decorator_storage::{DecoratorStorage, DecoratorStorageAddr},
        decorator_subtype::{DecoratorLibKey, DerivedDecoratorInfoResources, DerivedDecoratorInfoView},
        dormers::DormerHoleSsbo,
        public_walls::{PublicWalls, PublicWallState},
        wall_holes::RawWallHole,
    }},
    systems::{collision::{WindowAutoClutterCandidate, WindowAutoClutterKind},
        ui::signifiers::signifier_create::UiSignifierStream},
    utils::components::transform::Transform,
};
use glam::{Vec2, Vec3};
use utils::{comparative_space::ComparativeSpace, wall_space::{WallSpace, WallSpaceCoord}};
use crate::{blueprint::{DecoratorBlueprint, DecoratorBlueprints},
    decorator_visual_state::ClothInfo,
    utils::{subdivide_wall_decorator, visual_entity_builder::WindowVisualEntityBuilder}};

const FRAME_NAMES: [&str; 3] = ["decorators/window_cottage_1x1",
    "decorators/window_cottage_2x1", "decorators/window_cottage_3x1"];
const GLASS_NAMES: [&str; 3] = ["decorators/window_cottage_1x1_glass",
    "decorators/window_cottage_2x1_glass", "decorators/window_cottage_3x1_glass"];
const HALF_DORMER_NAMES: [&str; 3] = ["decorators/window_cottage_1x1_halfdormer_frame",
    "decorators/window_cottage_2x1_halfdormer_frame", "decorators/window_cottage_3x1_halfdormer_frame"];
const BROKEN_GLASS_NAMES: [&str; 3] = ["decorators/window_cottage_1x1_glass_broken_0",
    "decorators/window_cottage_1x1_glass_broken_1", "decorators/window_cottage_1x1_glass_broken_2"];
const FLOWERBED_NAMES: [&str; 3] = ["decorators/window_cottage_1x1_flowerbed_locations",
    "decorators/window_cottage_2x1_flowerbed_locations", "decorators/window_cottage_3x1_flowerbed_locations"];
const CORNER_FLOWERBED_NAMES: [&str; 3] = ["decorators/window_cottage_corner_1x1_flowerbed_locations",
    "decorators/window_cottage_corner_2x1_flowerbed_locations", "decorators/window_cottage_corner_3x1_flowerbed_locations"];

pub const LINTEL_HEIGHT_OFFSET: f32 = f32::from_bits(0x3fb7_0a3f);
pub const LINTEL_HALF: f32 = 0.5;
pub const LINTEL_MESH_HEIGHT_EXTRA: f32 = f32::from_bits(0x3eb8_51eb);
pub const LINTEL_ROTATION_OFFSET: f32 = f32::from_bits(0x3da5_e353);
pub const LINTEL_SCALE_END: f32 = f32::from_bits(0x3f51_eb85);
pub const LINTEL_SCALE_MIDDLE: f32 = f32::from_bits(0x3f23_d70a);
pub const LINTEL_HEIGHT_DIVISOR: f32 = f32::from_bits(0x3f66_6666);
pub const LINTEL_RECORD_CONST: f32 = f32::from_bits(0x3e4c_cccd);
pub const LINTEL_MESH: &str = "decorators/setdressing_window_lintel";
pub const LINTEL_SHADER: &str = "nani_solid_vertex_color_window";

fn rank_index(rank: u8) -> usize {
    let index = usize::from(rank.wrapping_sub(1));
    assert!(index < 3, "农舍窗 rank 必须为1至3");
    index
}

/// 原473B及其实际map闭包：读同一资产中的位置/法线，不将父scale乘到位置。
pub fn flowerbed_xforms(rank: u8, subtype: u8, parent: &Transform,
    mesh_library: &AssetMeshLibrary) -> Vec<Transform> {
    let names = match subtype {
        0 | 1 => &FLOWERBED_NAMES,
        3 => &CORNER_FLOWERBED_NAMES,
        value => panic!("flowerbed_xforms: unexpected subtype {value}"),
    };
    let mesh = mesh_library.get_by_name(names[rank_index(rank)])
        .expect("called `Option::unwrap()` on a `None` value");
    mesh.positions_without_indices().iter().zip(mesh.normals_without_indices()).map(|(p, n)| {
        Transform { rotation: parent.rotation * utils::rotation_from_normal(*n),
            translation: parent.rotation * *p + parent.translation, scale: parent.scale }
    }).collect()
}

/// 原2637B：真实墙洞与视觉builder按原顺序追加到同蓝图。
#[allow(clippy::too_many_arguments)]
pub fn add_lintels(id: DecoratorId, info: &DecoratorInfo, sources: &[Transform],
    semi_transparent: bool, color: ColorId, blueprint: &mut DecoratorBlueprint,
    derived: DerivedDecoratorInfoView<'_>, y_parameter: f32, flags: u32, height_parameter: f32) {
    let coord = info.dst_as_wall().wall_space_coord();
    let shift = (y_parameter + LINTEL_HEIGHT_OFFSET) * LINTEL_HALF;
    let center = coord.with_y_offset(shift);
    let key = DecoratorLibKey::new(derived.subtype(id).0, info.rank());
    let width = derived.mesh_library().collision_record(key).bounds.footprint.size.x;
    blueprint.wall_holes.push(RawWallHole { center: [center.u, center.y],
        dimensions: [width + LINTEL_MESH_HEIGHT_EXTRA, LINTEL_RECORD_CONST], kind: 0 });
    for (index, source) in sources.iter().enumerate() {
        let mut transform = *source;
        transform.translation.y = source.translation.y + shift;
        if info.rank() >= 2 {
            if index == 0 {
                transform.translation -= source.rotation * Vec3::new(LINTEL_ROTATION_OFFSET, 0.0, 0.0);
                transform.scale.x = LINTEL_SCALE_END;
            } else if index + 1 == sources.len() {
                transform.translation += source.rotation * Vec3::new(LINTEL_ROTATION_OFFSET, 0.0, 0.0);
                transform.scale.x = LINTEL_SCALE_END;
            } else { transform.scale.x = LINTEL_SCALE_MIDDLE; }
        }
        let height_ratio = height_parameter / LINTEL_HEIGHT_DIVISOR;
        if flags & 1 != 0 { transform.scale.x *= height_ratio; }
        blueprint.visual_entities.push(window_builder(id, transform, LINTEL_MESH,
            LINTEL_SHADER, Some(color), semi_transparent));
    }
}

/// 复用已有原构造器的None状态，只写该真实业务分支证明的字段。
pub(super) fn window_builder(id: DecoratorId, transform: Transform, mesh: &'static str,
    nani: &'static str, color: Option<ColorId>, semi: bool) -> WindowVisualEntityBuilder {
    let mut builder = WindowVisualEntityBuilder::new(transform.translation, transform.rotation, id.0);
    builder.scale = transform.scale; builder.mesh_name = mesh; builder.nani_name = nani;
    builder.color_id = color; builder.semi_transparent = semi; builder
}

pub(super) fn wall_space(wall: &PublicWallState) -> WallSpace {
    WallSpace { is_loop: wall.shape().tag() != 2, length: wall.wall_space_length() }
}

/// 原calculate_hash的Sip13零key、Hash顺序交给共享 `utils` crate。
pub(super) fn candidate_hash(value: impl Hash) -> u64 {
    ::utils::calculate_hash(&value)
}

fn wall_window_pose(info: &DecoratorInfo, width: f32, wall: &PublicWallState,
    cached: &CachedDecoratorTransforms, id: DecoratorId) -> Transform {
    if wall.shape().tag() == 1 {
        let transform = cached.transform(id.0);
        return Transform { rotation: transform.rotation, translation: transform.translation,
            scale: transform.scale };
    }
    let coord = info.dst_as_wall().wall_space_coord();
    let space = wall_space(wall);
    let first = WallSpaceCoord::new_unchecked(space.project_clamp(coord.u + width * -0.5), coord.y)
        .to_world_space(wall.simplified_curve_2d());
    let second = WallSpaceCoord::new_unchecked(space.project_clamp(coord.u + width * 0.5), coord.y)
        .to_world_space(wall.simplified_curve_2d());
    let dx = second.x - first.x; let dz = second.z - first.z;
    let reciprocal = 1.0 / (dx * dx + dz * dz).sqrt();
    Transform { rotation: utils::xz_rotation_bad(Vec2::new(-dz * reciprocal, dx * reciprocal)),
        translation: Vec3::new(second.x * 0.5 + first.x * 0.5, coord.y,
            second.z * 0.5 + first.z * 0.5), scale: Vec3::ONE }
}

fn append_half_dormer(blueprint: &mut DecoratorBlueprint, id: DecoratorId,
    wall: WallId, pose: Transform, bounds: utils::geometry::Rectangle3d, rank: usize,
    shutters: bool, color: ColorId, semi: bool) {
    let mut local_bounds = bounds;
    local_bounds.footprint.size.y = f32::from_bits(0x3f21_47ae);
    let extent = local_bounds.extent(); let forward = pose.rotation * Vec3::Z;
    let first = pose.translation + forward * f32::from_bits(0x3f8a_3d71) * 0.5;
    blueprint.dormer_holes.push(DormerHoleSsbo::cottage(wall.0 as u32, first,
        Vec3::new(extent.x, (extent.y + 0.0) + 0.2, extent.z + 0.5), [forward.x, forward.z]));
    if shutters {
        let second = pose.translation + forward * (extent.z + 0.5) * 0.5;
        blueprint.dormer_holes.push(DormerHoleSsbo::cottage(wall.0 as u32, second,
            Vec3::new(extent.x + 0.7, extent.y + -0.2, 0.5), [forward.x, forward.z]));
    }
    blueprint.visual_entities.push(window_builder(id, pose, HALF_DORMER_NAMES[rank],
        LINTEL_SHADER, Some(color), semi));
}

/// 原14303B主函数的七参面；真正实体由后续fullvisual与同World Nani发布。
pub fn generate_cottage_wall_windows(derived: DerivedDecoratorInfoResources,
    storage: Res<DecoratorStorage>, cached: Res<CachedDecoratorTransforms>, walls: Res<PublicWalls>,
    mut blueprints: ResMut<DecoratorBlueprints>, meshes: Res<AssetMeshLibrary>,
    _signifiers: Res<UiSignifierStream>) {
    let view = derived.as_view();
    for (wall_id, decorators) in storage.iter_walls() {
        for (modified, info) in decorators.iter_modified() {
            let id = DecoratorId(modified); let subtype = view.subtype(id).0;
            if subtype > 1 { continue; }
            let wall = walls.get(wall_id).expect("called `Option::unwrap()` on a `None` value");
            let key = DecoratorLibKey::new(subtype, info.rank());
            let bounds = view.mesh_library().collision_record(key).bounds;
            let blueprint = blueprints.insert_for(modified, DecoratorStorageAddr::Wall(wall_id), info.state());
            let index = rank_index(info.rank());
            let mut pose = wall_window_pose(info, bounds.footprint.size.x, wall, &cached, id);
            let premade = wall.shape().tag() < 2;
            if premade { pose.translation += pose.rotation * Vec3::new(0.0, 0.0, f32::from_bits(0x3e0f_5c29)); }
            let collision = view.mesh_library().collision_record(key).mesh.clone();
            let interaction = view.mesh_library().interaction_record(key).mesh.clone();
            let mut collision_position = pose.translation;
            if premade {
                let depth = bounds.extent().z;
                collision_position += pose.rotation * (Vec3::new(depth * 0.0, depth * 0.0, depth) * 0.5);
            }
            blueprint.set_positioned_colliders(collision, interaction, &collision_position, &pose.rotation);
            blueprint.set_cached_transform(CachedDecoratorTransform { rotation: pose.rotation,
                translation: pose.translation, scale: pose.scale });
            let color = ColorId(info.u28); let semi = info.state() != 0;
            blueprint.visual_entities.push(window_builder(id, pose, FRAME_NAMES[index],
                LINTEL_SHADER, Some(color), semi));
            if premade {
                let mut glass_pose = pose;
                glass_pose.translation -= pose.rotation * Vec3::new(0.0, 0.0, f32::from_bits(0x3d8f_5c29));
                let glass_color = ColorId(info.window_extra().0);
                let mut builder = window_builder(id, glass_pose, GLASS_NAMES[index],
                    "nani_solid_vertex_color_window_glass", Some(glass_color), semi);
                builder.window_light = Some(1000 * u64::from(info.rank()) + u64::from(modified));
                blueprint.visual_entities.push(builder);
            }
            let coord = info.dst_as_wall().wall_space_coord();
            let width = bounds.footprint.size.x;
            let hole_width = if wall.shape().tag() == 0 { width + f32::from_bits(
                [0xbd4c_cccd, 0xbdcc_cccd, 0xbe19_999a][index]) } else { width };
            let height = bounds.top - bounds.bottom;
            blueprint.wall_holes.push(RawWallHole { center: [coord.u, coord.y],
                dimensions: [hole_width, height], kind: 0 });
            let mut rng = fastrand::Rng::with_seed(utils::rng::init_rng(info.decorator_id));
            let shutters = info.rank() == 2 && rng.u32(..) >> 31 != 0;
            if subtype == 0 && info.rank() < 3 {
                let transforms: Vec<_> = subdivide_wall_decorator(wall_space(wall), coord, width,
                    usize::from(info.rank()), wall.simplified_curve_2d()).collect();
                // 此原调用的flags为0，height槽不参与任何输出；不拿它生成默认几何。
                add_lintels(id, info, &transforms, semi, color, blueprint, view, 0.0, 0, 0.0);
            } else if subtype == 1 && premade {
                append_half_dormer(blueprint, id, wall_id, pose, bounds, index, shutters, color, semi);
            }
            if !premade {
                for source in flowerbed_xforms(info.rank(), subtype, &pose, &meshes) {
                    let mesh = BROKEN_GLASS_NAMES[rng.usize(0..3)];
                    let mut broken = source; broken.translation.y += 0.8;
                    broken.translation += source.rotation * Vec3::new(-0.0, -0.1, -0.0);
                    blueprint.visual_entities.push(window_builder(id, broken, mesh,
                        "nani_solid_vertex_color", None, semi));
                }
                continue;
            }
            let mut rng = fastrand::Rng::with_seed(utils::rng::init_rng(info.decorator_id));
            if info.decorator_id.wrapping_mul(0xaaaa_aab6).wrapping_add(0x5556_884b)
                <= 0x5555_5555 && info.rank() < 3 {
                blueprint.cloth_info = Some(Box::new(ClothInfo { transform: pose, wall: wall_id,
                    dimensions: [hole_width, height], decorator_id: id }));
            } else if rng.u32(..) >> 30 == 1 {
                for (location, transform) in flowerbed_xforms(info.rank(), subtype, &pose, &meshes)
                    .into_iter().enumerate() {
                    let key = candidate_hash((id, "flower_bed", location));
                    let mesh_index = rng.usize(0..6);
                    blueprint.window_auto_clutter_candidates.push(WindowAutoClutterCandidate {
                        kind: WindowAutoClutterKind::WindowFlowerbed { mesh_index },
                        rotation: transform.rotation, translation: transform.translation,
                        scale: transform.scale, key, wall: wall_id });
                }
            }
            if shutters {
                for (side, factor, name) in [(false, 0.45, "shutter_right"), (true, -0.45, "shutter_left")] {
                    let key = candidate_hash((id, name));
                    let translation = pose.translation + pose.rotation * Vec3::new(width * factor, 0.0, 0.2);
                    blueprint.window_auto_clutter_candidates.push(WindowAutoClutterCandidate {
                        kind: WindowAutoClutterKind::WindowShutter { side, native_u32_04: info.u28 },
                        rotation: pose.rotation, translation, scale: pose.scale, key, wall: wall_id });
                }
            }
        }
    }
}

pub fn is_cottage_window(info: &DecoratorInfo) -> bool {
    info.decorator_type().0 == 0x38
}

#[cfg(test)]
pub(super) mod original_window_helper_tests {
    use super::*;
    use country_core::{asset_libraries::mesh_library::MeshAsset,
        render::mesh::{Mesh, VertexAttributeValues, ATTR_NORMAL, ATTR_POSITION}};

    fn location_mesh(positions: Vec<Vec3>, normals: VertexAttributeValues) -> Mesh {
        let mut mesh = Mesh::new("原窗花槽位置夹具");
        mesh.set_attribute(ATTR_POSITION, VertexAttributeValues::Float3(positions));
        mesh.set_attribute(ATTR_NORMAL, normals);
        mesh
    }

    #[test]
    fn actual_flowerbed_mesh_zip_and_parent_pose_preserve_rotation_and_unscaled_positions() {
        let meshes = AssetMeshLibrary::new();
        meshes.add(MeshAsset::new(location_mesh(vec![Vec3::X, Vec3::Z, Vec3::Y],
            VertexAttributeValues::Float3(vec![Vec3::Y, Vec3::X])),
            Some(FLOWERBED_NAMES[1].to_owned())));
        let parent = Transform { rotation: glam::Quat::from_rotation_y(std::f32::consts::FRAC_PI_2),
            translation: Vec3::new(10.0, 2.0, 3.0), scale: Vec3::new(2.0, 3.0, 4.0) };
        let output = flowerbed_xforms(2, 1, &parent, &meshes);
        assert_eq!(output.len(), 2);
        for (pose, expected_position, expected_normal) in [
            (&output[0], Vec3::new(10.0, 2.0, 2.0), Vec3::Y),
            (&output[1], Vec3::new(11.0, 2.0, 3.0), -Vec3::Z),
        ] {
            assert!((pose.translation - expected_position).abs().max_element() < 0.000001);
            assert!((pose.rotation * Vec3::Z - expected_normal).abs().max_element() < 0.000001);
            assert_eq!(pose.scale, parent.scale);
        }
    }

    #[test]
    fn actual_corner_flowerbed_selects_ranked_corner_location_mesh() {
        let meshes = AssetMeshLibrary::new();
        meshes.add(MeshAsset::new(location_mesh(vec![Vec3::new(7.0, 2.0, -1.0)],
            VertexAttributeValues::Float3(vec![Vec3::Z])),
            Some(CORNER_FLOWERBED_NAMES[2].to_owned())));
        let pose = Transform { translation: Vec3::ZERO, rotation: glam::Quat::IDENTITY,
            scale: Vec3::ONE };
        let output = flowerbed_xforms(3, 3, &pose, &meshes);
        assert_eq!(output.len(), 1);
        assert_eq!(output[0].translation, Vec3::new(7.0, 2.0, -1.0));
    }

    #[test]
    #[should_panic(expected = "incorrect attribute type")]
    fn actual_flowerbed_requires_float3_normal_attribute() {
        let meshes = AssetMeshLibrary::new();
        meshes.add(MeshAsset::new(location_mesh(vec![Vec3::X],
            VertexAttributeValues::Float(vec![1.0])), Some(FLOWERBED_NAMES[0].to_owned())));
        flowerbed_xforms(1, 0, &Transform { translation: Vec3::ZERO,
            rotation: glam::Quat::IDENTITY, scale: Vec3::ONE }, &meshes);
    }

    #[test]
    #[should_panic(expected = "does not exist in the library")]
    fn actual_flowerbed_missing_required_mesh_is_a_failure() {
        flowerbed_xforms(1, 0, &Transform { translation: Vec3::ZERO,
            rotation: glam::Quat::IDENTITY, scale: Vec3::ONE }, &AssetMeshLibrary::new());
    }
    fn lintel_fixture(rank: u8) -> (DerivedDecoratorSubtypes, DecoratorMeshLibrary, DecoratorInfo) {
        use std::sync::Arc;
        use country_core::{resources::walls::decorator_subtype::{DecoratorCollision, DecoratorSubtype},
            systems::collision::world::RaycastMeshData};
        let mut subtypes = DerivedDecoratorSubtypes::default();
        subtypes.insert(7, DecoratorSubtype(0));
        let mut meshes = DecoratorMeshLibrary::default();
        meshes.collision.insert(DecoratorLibKey::new(0, rank), DecoratorCollision {
            triangles: Vec::new(), mesh: Arc::new(RaycastMeshData::new_cuboid_at(Vec3::ONE, Vec3::ZERO)),
            bounds: utils::geometry::Rectangle3d::new(Vec3::new(0.0, 1.0, 0.0),
                Vec3::new(4.0, 2.0, 1.0), Vec2::X),
        });
        let info = DecoratorInfo { dst_wall: u64::from(4.0_f32.to_bits()), rank,
            ..DecoratorInfo::default() };
        (subtypes, meshes, info)
    }

    use country_core::resources::walls::decorator_subtype::{DecoratorMeshLibrary, DerivedDecoratorSubtypes};

    #[test]
    fn actual_lintels_publish_typed_hole_and_three_builders_with_exact_end_scaling() {
        let (subtypes, meshes, info) = lintel_fixture(2);
        let view = DerivedDecoratorInfoView::new(&subtypes, &meshes);
        let source = |x| Transform { translation: Vec3::new(x, 0.0, 0.0),
            rotation: glam::Quat::IDENTITY, scale: Vec3::new(9.0, 2.0, 3.0) };
        let mut blueprint = DecoratorBlueprint::new();
        add_lintels(DecoratorId(7), &info, &[source(0.0), source(1.0), source(2.0)],
            true, ColorId(0x1234_abcd), &mut blueprint, view, 0.0, 1, 1.8);
        assert_eq!(blueprint.wall_holes.len(), 1);
        let hole = &blueprint.wall_holes[0];
        assert_eq!(hole.center, [4.0, f32::from_bits(0x3f37_0a3f)]);
        assert_eq!(hole.dimensions, [f32::from_bits(0x408b_851f), f32::from_bits(0x3e4c_cccd)]);
        assert_eq!(hole.kind, 0);
        assert_eq!(blueprint.visual_entities.len(), 3);
        for (builder, x, scale_x) in [
            (&blueprint.visual_entities[0], -f32::from_bits(0x3da5_e353), f32::from_bits(0x3fd1_eb85)),
            (&blueprint.visual_entities[1], 1.0, f32::from_bits(0x3fa3_d70a)),
            (&blueprint.visual_entities[2], 2.0 + f32::from_bits(0x3da5_e353), f32::from_bits(0x3fd1_eb85)),
        ] {
            assert_eq!(builder.translation, Vec3::new(x, f32::from_bits(0x3f37_0a3f), 0.0));
            assert_eq!(builder.scale, Vec3::new(scale_x, 2.0, 3.0));
            assert_eq!(builder.mesh_name, "decorators/setdressing_window_lintel");
            assert_eq!(builder.nani_name, "nani_solid_vertex_color_window");
            assert_eq!(builder.color_id, Some(ColorId(0x1234_abcd)));
            assert!(builder.semi_transparent);
            assert_eq!(builder.window_light, None);
        }
    }

    #[test]
    fn actual_lintel_singleton_is_first_only_and_empty_slice_still_publishes_hole() {
        let (subtypes, meshes, info) = lintel_fixture(2);
        let view = DerivedDecoratorInfoView::new(&subtypes, &meshes);
        let mut blueprint = DecoratorBlueprint::new();
        let source = Transform { translation: Vec3::ZERO, rotation: glam::Quat::IDENTITY,
            scale: Vec3::new(9.0, 2.0, 3.0) };
        add_lintels(DecoratorId(7), &info, &[source], false, ColorId(13), &mut blueprint,
            view, 0.0, 0, f32::NAN);
        assert_eq!(blueprint.visual_entities.len(), 1);
        assert_eq!(blueprint.visual_entities[0].translation.x.to_bits(), 0xbda5_e353);
        assert_eq!(blueprint.visual_entities[0].scale.x.to_bits(), 0x3f51_eb85);
        add_lintels(DecoratorId(7), &info, &[], false, ColorId(13), &mut blueprint,
            view, 0.0, 0, f32::NAN);
        assert_eq!(blueprint.wall_holes.len(), 2);
        assert_eq!(blueprint.visual_entities.len(), 1);
    }

    #[test]
    fn actual_lintel_missing_collision_fails_before_any_hole_or_builder_write() {
        let (subtypes, mut meshes, info) = lintel_fixture(2);
        meshes.collision.clear();
        let view = DerivedDecoratorInfoView::new(&subtypes, &meshes);
        let mut blueprint = DecoratorBlueprint::new();
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            add_lintels(DecoratorId(7), &info, &[], false, ColorId(13), &mut blueprint,
                view, 0.0, 0, 0.0);
        }));
        assert!(result.is_err());
        assert!(blueprint.wall_holes.is_empty() && blueprint.visual_entities.is_empty());
    }

    pub(in crate::decorator_visual) fn producer_world(subtype: u8, rank: u8) -> World {
        producer_world_with_shape(subtype, rank, 1)
    }

    pub(in crate::decorator_visual) fn producer_world_with_shape(subtype: u8, rank: u8, shape: u32) -> World {
        use std::sync::Arc;
        use country_core::{resources::{roofs::geometry::surface::Curve2,
            walls::{InnerWalls, WallState, decorator::{DecoratorDst, WallDecoratorDst},
                decorator_subtype::{DecoratorCollision, DecoratorSubtype},
                publication_fields::WallShapeSnapshot, publication_source::TerrainHeightGrid512}},
            systems::collision::world::RaycastMeshData};
        let rectangle = utils::geometry::Rectangle2d::new_centered_axis_aligned(Vec2::splat(8.0));
        let points = rectangle.as_points2().map(|p| [p.x, p.y]);
        let curve = Curve2::try_new(points.to_vec(), true).unwrap();
        let mut snapshot = WallState::new(curve, 3.0, 0, 0, false).snapshot().clone();
        let mut bytes = [0; 28]; bytes[..4].copy_from_slice(&shape.to_le_bytes());
        for (index, value) in [1.0_f32, 0.0, 0.0, 0.0, 8.0, 8.0].into_iter().enumerate() {
            bytes[4 + index * 4..8 + index * 4].copy_from_slice(&value.to_le_bytes());
        }
        snapshot.source.shape = WallShapeSnapshot::from_observed(bytes);
        snapshot.source.curve_kind_tag = shape;
        let mut inner = InnerWalls::new();
        inner.new_wall_with_id(WallId(42), WallState::from_snapshot(snapshot, 0));
        let mut walls = PublicWalls::default();
        walls.consume_with_paths(&mut inner, &TerrainHeightGrid512 { encoded: vec![0; 512 * 512].into(),
            minimum_y: 0.0, maximum_y: 0.0, sampling_fix: false }, None, &country_core::resources::walls::public_walls::WallPublicationDirty::default(), &country_core::resources::walls::public_walls::WallSpatialHash::default());
        assert!(walls.get(WallId(42)).is_some());
        let mut info = DecoratorInfo { rank, decorator_id: 1, state: 2,
            u28: 0x5566_7788, u30: 0x1020_3040, u40: 6 << 8, ..DecoratorInfo::default() };
        info.set_destination(DecoratorDst::Wall(WallDecoratorDst {
            coord: WallSpaceCoord::new_unchecked(4.0, 2.0), platform: WallId(99),
            relative_y: 0.0, position: Vec3::new(-4.0, 2.0, -4.0),
            corner_attachment: 1, bottom_as_door: false }));
        let mut storage = DecoratorStorage::default();
        storage.get_or_insert_default(DecoratorStorageAddr::Wall(WallId(42)))
            .insert(DecoratorId(7), info);
        let mut subtypes = DerivedDecoratorSubtypes::default();
        subtypes.insert(7, DecoratorSubtype(subtype));
        let mut library = DecoratorMeshLibrary::default();
        let bounds = utils::geometry::Rectangle3d::new(Vec3::new(0.0, 1.0, 0.0),
            Vec3::new(4.0, 2.0, 1.0), Vec2::X);
        let key = DecoratorLibKey::new(subtype, rank);
        for (table, dimensions) in [(&mut library.collision, Vec3::ONE),
            (&mut library.interaction, Vec3::splat(2.0))] {
            table.insert(key, DecoratorCollision { triangles: Vec::new(),
                mesh: Arc::new(RaycastMeshData::new_cuboid_at(dimensions, Vec3::ZERO)), bounds });
        }
        let mut cached = CachedDecoratorTransforms::default();
        cached.insert(7, CachedDecoratorTransform { translation: Vec3::new(-4.0, 2.0, -4.0),
            rotation: glam::Quat::IDENTITY, scale: Vec3::new(2.0, 3.0, 4.0) });
        let meshes = AssetMeshLibrary::new();
        for name in FLOWERBED_NAMES.into_iter().chain(CORNER_FLOWERBED_NAMES) {
            meshes.add(MeshAsset::new(location_mesh(Vec::new(), VertexAttributeValues::Float3(Vec::new())),
                Some(name.to_owned())));
        }
        let mut world = World::new();
        world.insert_resource(walls); world.insert_resource(storage); world.insert_resource(subtypes);
        world.insert_resource(library); world.insert_resource(cached); world.insert_resource(meshes);
        world.insert_resource(DecoratorBlueprints::default());
        world.insert_resource(UiSignifierStream::default());
        world
    }

    #[test]
    fn actual_wall_producer_connects_outer_wall_modified_id_colors_glass_key_and_real_mesh_arcs() {
        use bevy_ecs::system::RunSystemOnce;
        use crate::blueprint::BlueprintMeshSlot;
        let mut world = producer_world(0, 1);
        world.run_system_once(generate_cottage_wall_windows).unwrap();
        let blueprints = world.resource::<DecoratorBlueprints>();
        let blueprint = blueprints.get(7).unwrap();
        assert_eq!(blueprint.storage_addr, Some(DecoratorStorageAddr::Wall(WallId(42))));
        assert_eq!(blueprint.interaction_source_flags, 2);
        assert_eq!(blueprint.visual_entities.len(), 3);
        let frame = &blueprint.visual_entities[0]; let glass = &blueprint.visual_entities[1];
        assert_eq!(frame.mesh_name, "decorators/window_cottage_1x1");
        assert_eq!(frame.color_id, Some(ColorId(0x5566_7788)));
        assert_eq!(glass.mesh_name, "decorators/window_cottage_1x1_glass");
        assert_eq!(glass.color_id, Some(ColorId(0x1020_3040)));
        assert_eq!(glass.window_light, Some(1007));
        assert!(frame.semi_transparent && glass.semi_transparent);
        assert_eq!(frame.scale, Vec3::new(2.0, 3.0, 4.0));
        assert!((frame.translation.z - -3.86).abs() < 0.000001);
        assert!((glass.translation.z - -3.93).abs() < 0.000001);
        let BlueprintMeshSlot::Mesh(collision) = &blueprint.collision_mesh else { panic!("缺真实碰撞") };
        let BlueprintMeshSlot::Mesh(interaction) = &blueprint.interaction_mesh else { panic!("缺真实交互") };
        let library = world.resource::<DecoratorMeshLibrary>();
        let key = DecoratorLibKey::new(0, 1);
        assert!(std::sync::Arc::ptr_eq(&collision.mesh, &library.collision_record(key).mesh));
        assert!(std::sync::Arc::ptr_eq(&interaction.mesh, &library.interaction_record(key).mesh));
        assert!((collision.position.z - -3.36).abs() < 0.000001);
        assert_eq!(blueprint.wall_holes.len(), 2);
        let cloth = blueprint.cloth_info.as_ref().unwrap();
        assert_eq!(cloth.wall, WallId(42));
        assert_eq!(cloth.decorator_id, DecoratorId(7));
        assert_eq!(cloth.dimensions, [4.0, 2.0]);
    }

    #[test]
    fn actual_wall_missing_interaction_preserves_prior_insert_and_pre_frame_failure() {
        use bevy_ecs::system::RunSystemOnce;
        let mut world = producer_world(0, 1);
        world.resource_mut::<DecoratorMeshLibrary>().interaction.clear();
        let failure = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            world.run_system_once(generate_cottage_wall_windows).unwrap();
        }));
        assert!(failure.is_err());
        let blueprints = world.resource::<DecoratorBlueprints>();
        let blueprint = blueprints.get(7).unwrap();
        assert_eq!(blueprint.storage_addr, Some(DecoratorStorageAddr::Wall(WallId(42))));
        assert!(blueprint.visual_entities.is_empty() && blueprint.transform.is_none());
        assert!(blueprint.wall_holes.is_empty());
    }

    #[test]
    fn actual_premade_wall_all_ranks_and_both_subtypes_publish_their_own_branches() {
        use bevy_ecs::system::RunSystemOnce;
        for subtype in 0..=1 {
            for rank in 1..=3 {
                let mut world = producer_world(subtype, rank);
                world.run_system_once(generate_cottage_wall_windows).unwrap();
                let blueprints = world.resource::<DecoratorBlueprints>();
                let blueprint = blueprints.get(7).unwrap();
                let count = if subtype == 1 { 3 } else if rank < 3 { 2 + usize::from(rank) } else { 2 };
                assert_eq!(blueprint.visual_entities.len(), count, "subtype={subtype}, rank={rank}");
                assert_eq!(blueprint.visual_entities[1].window_light, Some(1000 * u64::from(rank) + 7));
                assert_eq!(blueprint.cloth_info.is_some(), rank < 3);
                if subtype == 1 {
                    assert_eq!(blueprint.visual_entities[2].mesh_name, HALF_DORMER_NAMES[usize::from(rank - 1)]);
                    assert!((1..=2).contains(&blueprint.dormer_holes.len()));
                    if rank != 2 { assert_eq!(blueprint.dormer_holes.len(), 1); }
                } else { assert!(blueprint.dormer_holes.is_empty()); }
            }
        }
    }

    #[test]
    fn actual_nonpremade_wall_publishes_real_broken_glass_without_premade_business() {
        use bevy_ecs::system::RunSystemOnce;
        let mut world = producer_world_with_shape(0, 1, 2);
        world.resource::<AssetMeshLibrary>().add(MeshAsset::new(location_mesh(vec![Vec3::X],
            VertexAttributeValues::Float3(vec![Vec3::Y])), Some(FLOWERBED_NAMES[0].to_owned())));
        world.run_system_once(generate_cottage_wall_windows).unwrap();
        let blueprints = world.resource::<DecoratorBlueprints>();
        let blueprint = blueprints.get(7).unwrap();
        assert_eq!(blueprint.visual_entities.len(), 3);
        let broken = blueprint.visual_entities.last().unwrap();
        assert!(BROKEN_GLASS_NAMES.contains(&broken.mesh_name));
        assert_eq!(broken.nani_name, "nani_solid_vertex_color");
        assert_eq!(broken.color_id, None); assert_eq!(broken.window_light, None);
        assert!(blueprint.visual_entities.iter().all(|builder| builder.window_light.is_none()));
        assert!(blueprint.cloth_info.is_none() && blueprint.window_auto_clutter_candidates.is_empty());
        assert!(blueprint.dormer_holes.is_empty());
    }

    #[test]
    fn actual_wall_invalid_rank_fails_after_insert_when_the_required_key_exists() {
        use bevy_ecs::system::RunSystemOnce;
        let mut world = producer_world(0, 0);
        let failure = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            world.run_system_once(generate_cottage_wall_windows).unwrap();
        }));
        assert!(failure.is_err());
        let blueprints = world.resource::<DecoratorBlueprints>();
        let blueprint = blueprints.get(7).unwrap();
        assert!(blueprint.visual_entities.is_empty());
    }

}
