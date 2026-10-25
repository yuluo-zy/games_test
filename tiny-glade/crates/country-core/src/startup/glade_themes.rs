//! Original B startup_olden1415539d0/3588B and startup_flowery141553120/2224B.
//! Callers preserve theme branching and immediately insert/remove the returned
//! PreplacedAutoClutter owner. Asset/library/Commands owners are shared.
use crate::{
    asset_libraries::{
        Handle,
        mesh_library::{AssetMeshLibrary, MeshAsset},
        meshwatch::MeshWatch,
        placement_data_library::AssetPlacementDataLibrary,
        shader_library::AssetShaderLibrary,
    },
    components::{
        NormalMap,
        drawable::{AlbedoMap, AlphaMap, DrawableMeshBundle, NaniBundle, NaniInstanceMeshDesc},
    },
    name_hash::NameHash,
    render::{
        mesh::Mesh,
        nani::{
            gpu_world::{GpuWorldSubsetKind, InstanceDataMarker},
            instance_data::PreplacedFlowerInstanceData,
            systems::NaniInstanceDesc,
        },
        shader::RasterShaderProgram,
        ssbo::rhapsody_boundary::GraphicsDevice,
    },
    resources::{
        placement_data::PlacementDataSsbo,
        preplaced_autoclutter::{PreplacedAutoClutter, PreplacedAutoClutterEntry},
        session::ActiveSession,
    },
    systems::render_loop::draw_passes::draw_lists::DontCastShadow,
    utils::components::transform::Transform,
};
use anyhow::Context;
use bevy_ecs::system::Commands;
use glam::{Mat4, Quat, Vec2, Vec3, Vec4};
use std::path::Path;
use utils::geometry::Rectangle2d;

const OLDEN_PLACEMENTS: [(&str, u8, Vec2); 6] = [
    ("trees/rocks.json", 0, Vec2::new(1.3, 1.3)),
    ("trees/clearing_edge_rocks.json", 0, Vec2::new(1.3, 1.3)),
    ("trees/stumps.json", 2, Vec2::new(1.8, 1.9)),
    (
        "trees/clearing_edge_tree_stumps.json",
        2,
        Vec2::new(1.8, 1.9),
    ),
    ("trees/logs.json", 1, Vec2::new(1.8, 4.4)),
    ("trees/clearing_edge_tree_logs.json", 1, Vec2::new(1.8, 4.4)),
];
fn mesh(
    root: &Path,
    path: &str,
    name: &str,
    meshes: &AssetMeshLibrary,
    watch: &MeshWatch,
) -> anyhow::Result<Handle<Mesh>> {
    let path = root.join(path);
    let path = path.to_str().context("theme mesh path is not UTF-8")?;
    let mesh = crate::utils::load_json::try_load_json_as_mesh(path)?;
    watch.watch(&mesh);
    Ok(meshes.add(MeshAsset::new(mesh, Some(name.to_owned()))))
}
fn shader(shaders: &AssetShaderLibrary, name: &str) -> anyhow::Result<Handle<RasterShaderProgram>> {
    shaders
        .get_raster_handle_by_name(name)
        .with_context(|| format!("theme raster shader {name} is not registered"))
}
fn drawable(mesh: Handle<Mesh>, shader: Handle<RasterShaderProgram>) -> DrawableMeshBundle {
    DrawableMeshBundle {
        mesh,
        shader,
        transform: Transform::identity(),
    }
}
/// Original Quat axis-angle path multiplies sin(half-angle) by every Y-axis
/// component, retaining signed zero in X/Z; from_rotation_y would discard it.
fn placement_rotation(angle: f32) -> Quat {
    Quat::from_axis_angle(Vec3::Y, angle)
}

/// Native uses a full homogeneous matrix/vector product, with w=0, then
/// normalizes the XZ projection. Keep the zero multiplications and the actual
/// stored angle; do not convert degrees, omit signed scale, or use only yaw.
fn clutter_rectangle(placement: &PlacementDataSsbo, dimensions: Vec2) -> Rectangle2d {
    let matrix = Mat4::from_scale_rotation_translation(
        Vec3::splat(placement.scale),
        placement_rotation(placement.rotation_angle),
        placement.position,
    );
    let direction = matrix * Vec4::X;
    let axis = Vec2::new(direction.x, direction.z).normalize();
    Rectangle2d::new(
        Vec2::new(placement.position.x, placement.position.z),
        dimensions * placement.scale,
        axis,
    )
}
#[allow(clippy::too_many_arguments)]
pub fn startup_olden(
    root: &Path,
    placements: &mut AssetPlacementDataLibrary,
    device: &dyn GraphicsDevice,
    session: &ActiveSession,
    commands: &mut Commands,
    watch: &MeshWatch,
    meshes: &AssetMeshLibrary,
    shaders: &AssetShaderLibrary,
) -> anyhow::Result<PreplacedAutoClutter> {
    let glade = root.join(session.assets_root());
    let mut clutter = Vec::new();
    for (file, kind, dimensions) in OLDEN_PLACEMENTS {
        let (records, _handle) = placements.load(device, glade.join(file));
        for record in records {
            clutter.push(PreplacedAutoClutterEntry {
                rectangle: clutter_rectangle(&record, dimensions),
                kind,
            });
        }
    }
    let clover = mesh(root, "assets/meshes/clover.json", "clover", meshes, watch)?;
    let flower_shader = shader(shaders, "meadow_flower")?;
    commands.spawn((
        drawable(clover, flower_shader),
        AlbedoMap(0x23ae4b5c6ce0d14d),
        AlphaMap(0x691e5a80737217f2),
        NormalMap(0xfbf3a82f352c1401),
    ));
    let flowers = mesh(
        root,
        "assets/meshes/clover_flowers.json",
        "clover_flowers",
        meshes,
        watch,
    )?;
    let flower_shader = shader(shaders, "meadow_flower")?;
    commands.spawn((
        drawable(flowers, flower_shader),
        AlbedoMap(0x2bb813728bcb5dd4),
    ));
    Ok(PreplacedAutoClutter(clutter))
}
#[allow(clippy::too_many_arguments)]
pub fn startup_flowery(
    root: &Path,
    placements: &mut AssetPlacementDataLibrary,
    device: &dyn GraphicsDevice,
    session: &ActiveSession,
    commands: &mut Commands,
    watch: &MeshWatch,
    meshes: &AssetMeshLibrary,
    shaders: &AssetShaderLibrary,
) -> anyhow::Result<()> {
    let billboards = mesh(
        root,
        "assets/meshes/billboard_plants_2.json",
        "billboard_plants_2",
        meshes,
        watch,
    )?;
    let billboard_shader = shader(shaders, "billboard_plant_shader")?;
    commands.spawn((drawable(billboards, billboard_shader), DontCastShadow));
    let (flowers, _handle) = placements.load(
        device,
        root.join(session.assets_root())
            .join("plants/lavender.json"),
    );
    for flower in flowers {
        commands.spawn(NaniBundle::<PreplacedFlowerInstanceData> {
            transform: Transform {
                rotation: placement_rotation(flower.rotation_angle),
                translation: flower.position,
                scale: Vec3::splat(flower.scale),
            },
            desc: NaniInstanceDesc {
                mesh: NaniInstanceMeshDesc::Named(NameHash(0x13fa53c58fdddee8)),
                subset_kind: GpuWorldSubsetKind::PreplacedFlower,
            },
            marker: InstanceDataMarker::default(),
        });
    }
    let meadow = mesh(
        root,
        "assets/meshes/meadow_lowpoly_flowers.json",
        "meadow_test",
        meshes,
        watch,
    )?;
    let flower_shader = shader(shaders, "meadow_flower")?;
    commands.spawn((
        drawable(meadow, flower_shader),
        AlbedoMap(NameHash::compute("meadow_flowers").0),
    ));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn original_complete_olden_loop_rectangle_goldens_preserve_angle_and_scale() {
        let p = PlacementDataSsbo {
            position: Vec3::new(1.0f32, 2.0f32, 3.0f32),
            scale: 0.75f32,
            rotation_angle: 0.0f32,
            ..Default::default()
        };
        let r = clutter_rectangle(&p, OLDEN_PLACEMENTS[0].2);
        assert_eq!(
            [
                r.axis.x.to_bits(),
                r.axis.y.to_bits(),
                r.center.x.to_bits(),
                r.center.y.to_bits(),
                r.size.x.to_bits(),
                r.size.y.to_bits()
            ],
            [
                0x3f800000, 0x0, 0x3f800000, 0x40400000, 0x3f799999, 0x3f799999
            ]
        );
        assert_eq!(OLDEN_PLACEMENTS[0].1, 0);
        let p = PlacementDataSsbo {
            position: Vec3::new(2.0f32, 2.0f32, 2.0f32),
            scale: 1.0f32,
            rotation_angle: 0.699999988079071f32,
            ..Default::default()
        };
        let r = clutter_rectangle(&p, OLDEN_PLACEMENTS[1].2);
        assert_eq!(
            [
                r.axis.x.to_bits(),
                r.axis.y.to_bits(),
                r.center.x.to_bits(),
                r.center.y.to_bits(),
                r.size.x.to_bits(),
                r.size.y.to_bits()
            ],
            [
                0x3f43ccb3, 0xbf24eb73, 0x40000000, 0x40000000, 0x3fa66666, 0x3fa66666
            ]
        );
        assert_eq!(OLDEN_PLACEMENTS[1].1, 0);
        let p = PlacementDataSsbo {
            position: Vec3::new(3.0f32, 2.0f32, 1.0f32),
            scale: 1.25f32,
            rotation_angle: -0.4000000059604645f32,
            ..Default::default()
        };
        let r = clutter_rectangle(&p, OLDEN_PLACEMENTS[2].2);
        assert_eq!(
            [
                r.axis.x.to_bits(),
                r.axis.y.to_bits(),
                r.center.x.to_bits(),
                r.center.y.to_bits(),
                r.size.x.to_bits(),
                r.size.y.to_bits()
            ],
            [
                0x3f6bcaa7, 0x3ec761d7, 0x40400000, 0x3f800000, 0x40100000, 0x40180000
            ]
        );
        assert_eq!(OLDEN_PLACEMENTS[2].1, 2);
        let p = PlacementDataSsbo {
            position: Vec3::new(4.0f32, 2.0f32, 0.0f32),
            scale: 1.5f32,
            rotation_angle: 90.0f32,
            ..Default::default()
        };
        let r = clutter_rectangle(&p, OLDEN_PLACEMENTS[3].2);
        assert_eq!(
            [
                r.axis.x.to_bits(),
                r.axis.y.to_bits(),
                r.center.x.to_bits(),
                r.center.y.to_bits(),
                r.size.x.to_bits(),
                r.size.y.to_bits()
            ],
            [
                0xbee569ea, 0xbf64dcf7, 0x40800000, 0x0, 0x402ccccc, 0x40366666
            ]
        );
        assert_eq!(OLDEN_PLACEMENTS[3].1, 2);
        let p = PlacementDataSsbo {
            position: Vec3::new(5.0f32, 2.0f32, -1.0f32),
            scale: 1.75f32,
            rotation_angle: 110.30000305175781f32,
            ..Default::default()
        };
        let r = clutter_rectangle(&p, OLDEN_PLACEMENTS[4].2);
        assert_eq!(
            [
                r.axis.x.to_bits(),
                r.axis.y.to_bits(),
                r.center.x.to_bits(),
                r.center.y.to_bits(),
                r.size.x.to_bits(),
                r.size.y.to_bits()
            ],
            [
                0xbf70fab4, 0x3eacccd8, 0x40a00000, 0xbf800000, 0x40499999, 0x40f66667
            ]
        );
        assert_eq!(OLDEN_PLACEMENTS[4].1, 1);
        let p = PlacementDataSsbo {
            position: Vec3::new(6.0f32, 2.0f32, -2.0f32),
            scale: 2.0f32,
            rotation_angle: -250.0f32,
            ..Default::default()
        };
        let r = clutter_rectangle(&p, OLDEN_PLACEMENTS[5].2);
        assert_eq!(
            [
                r.axis.x.to_bits(),
                r.axis.y.to_bits(),
                r.center.x.to_bits(),
                r.center.y.to_bits(),
                r.size.x.to_bits(),
                r.size.y.to_bits()
            ],
            [
                0x3e76c5a0, 0xbf787487, 0x40c00000, 0xc0000000, 0x40666666, 0x410ccccd
            ]
        );
        assert_eq!(OLDEN_PLACEMENTS[5].1, 1);
    }
    #[test]
    fn draw_callback_views_share_the_real_material_components() {
        use crate::systems::render_loop::draw_passes::draw_callbacks_a::{
            AlbedoMapView, AlphaMapView, NormalMapView,
        };
        use std::any::TypeId;
        assert_eq!(TypeId::of::<AlbedoMap>(), TypeId::of::<AlbedoMapView>());
        assert_eq!(TypeId::of::<AlphaMap>(), TypeId::of::<AlphaMapView>());
        assert_eq!(TypeId::of::<NormalMap>(), TypeId::of::<NormalMapView>());
        assert_eq!(std::mem::size_of::<PreplacedAutoClutterEntry>(), 28);
    }
    #[test]
    fn original_flowery_quaternions_preserve_signed_zero_channels() {
        assert_eq!(
            placement_rotation(0.0f32).to_array().map(f32::to_bits),
            [0x0, 0x0, 0x0, 0x3f800000]
        );
        assert_eq!(
            placement_rotation(0.699999988079071f32)
                .to_array()
                .map(f32::to_bits),
            [0x0, 0x3eaf904d, 0x0, 0x3f707abb]
        );
        assert_eq!(
            placement_rotation(-0.4000000059604645f32)
                .to_array()
                .map(f32::to_bits),
            [0x80000000, 0xbe4b6ff9, 0x80000000, 0x3f7ae5a5]
        );
        assert_eq!(
            placement_rotation(90.0f32).to_array().map(f32::to_bits),
            [0x0, 0x3f59d4d0, 0x0, 0x3f067b80]
        );
        assert_eq!(
            placement_rotation(110.30000305175781f32)
                .to_array()
                .map(f32::to_bits),
            [0x80000000, 0xbf7c3785, 0x80000000, 0x3e2f6461]
        );
        assert_eq!(
            placement_rotation(-250.0f32).to_array().map(f32::to_bits),
            [0x0, 0x3f1db4d4, 0x0, 0x3f49a7a9]
        );
    }
}
