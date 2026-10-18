//! Original B startup_terrain @140926bf0/5382B and grid helpers
//! @140928100/580B / @140928350/579B. Startup eight-item tuple, fifth item.
//! Current typed Mesh/libraries/Commands are retained; original paths, entity
//! bundles, same-mesh relationships and resource publication are explicit.
use std::path::Path;

use crate::{
    asset_libraries::{
        Handle,
        mesh_library::{AssetMeshLibrary, MeshAsset},
        meshwatch::MeshWatch,
        shader_library::AssetShaderLibrary,
        ssbo_library::AssetSsboLibrary,
    },
    components::{
        KeepOnLevelUnload,
        drawable::DrawableMeshBundle,
        rendering::{
            BackgroundTerrain, DontReceiveContactShadows, FlatLighting, OverwriteRoughness,
            StairsStep,
        },
    },
    render::{
        mesh::Mesh,
        shader::{RasterShaderProgram, boundary::ShaderBackend},
        ssbo::rhapsody_boundary::GraphicsDevice,
    },
    resources::rocky_terrain::{RockyTerrainMesh, TerrainEntities},
    systems::render_loop::draw_passes::draw_lists::{DesignatedRenderPass, DontCastShadow},
    utils::{components::transform::Transform, load_json::try_load_json_as_mesh},
};
use bevy_ecs::system::Commands;

fn emit_block(
    indices: &mut Vec<u32>,
    column_start: u32,
    column_end: u32,
    rows: u32,
    cells_per_row: u32,
    counter_clockwise: bool,
) {
    let row_stride = cells_per_row.wrapping_add(1);
    for row in 0..rows {
        let top_row = row.wrapping_mul(row_stride);
        let bottom_row = row.wrapping_add(1).wrapping_mul(row_stride);
        for column in column_start..column_end {
            let top_left = top_row.wrapping_add(column);
            let top_right = top_left.wrapping_add(1);
            let bottom_left = bottom_row.wrapping_add(column);
            let bottom_right = bottom_left.wrapping_add(1);
            if counter_clockwise {
                for index in [
                    top_left,
                    top_right,
                    bottom_left,
                    bottom_left,
                    top_right,
                    bottom_right,
                ] {
                    indices.push(index);
                }
            } else {
                for index in [
                    top_left,
                    bottom_left,
                    top_right,
                    top_right,
                    bottom_left,
                    bottom_right,
                ] {
                    indices.push(index);
                }
            }
        }
    }
}

fn grid_gen(
    indices: &mut Vec<u32>,
    column_start: u32,
    column_end: u32,
    rows: u32,
    cells_per_row: u32,
    counter_clockwise: bool,
) {
    assert!(column_start <= column_end, "网格列区间不能反向");
    // B @14092812d/37d: (end+1-start)>=6 recurses through four-column
    // blocks. Keep that wrapping u32 predicate, not a guessed row-width or
    // overflow rejection. Each native index is pushed individually.
    let mut block_start = column_start;
    while column_end.wrapping_add(1).wrapping_sub(block_start) >= 6 {
        let block_end = block_start.wrapping_add(4).min(column_end);
        emit_block(
            indices,
            block_start,
            block_end,
            rows,
            cells_per_row,
            counter_clockwise,
        );
        block_start = block_end;
    }
    emit_block(
        indices,
        block_start,
        column_end,
        rows,
        cells_per_row,
        counter_clockwise,
    );
}

/// Original clockwise order, four-column blocks, wrapping u32 arithmetic.
pub fn grid_gen_cw(
    indices: &mut Vec<u32>,
    column_start: u32,
    column_end: u32,
    rows: u32,
    cells_per_row: u32,
) {
    grid_gen(
        indices,
        column_start,
        column_end,
        rows,
        cells_per_row,
        false,
    );
}

/// Original counter-clockwise order with the same block and index arithmetic.
pub fn grid_gen_ccw(
    indices: &mut Vec<u32>,
    column_start: u32,
    column_end: u32,
    rows: u32,
    cells_per_row: u32,
) {
    grid_gen(indices, column_start, column_end, rows, cells_per_row, true);
}

/// Same original common helper @140b1aca0: watch before library add. It does
/// not allocate a Vao here; the current canonical asset maintenance does that.
fn install_mesh(
    mesh: Mesh,
    name: &str,
    meshes: &AssetMeshLibrary,
    watch: &MeshWatch,
) -> Handle<Mesh> {
    watch.watch(&mesh);
    meshes.add(MeshAsset::new(mesh, Some(name.to_owned())))
}

fn install_json_mesh(
    root: &Path,
    path: &str,
    name: &str,
    meshes: &AssetMeshLibrary,
    watch: &MeshWatch,
) -> anyhow::Result<Handle<Mesh>> {
    let path = root.join(path);
    let path = path
        .to_str()
        .ok_or_else(|| anyhow::anyhow!("terrain mesh path is not UTF-8: {}", path.display()))?;
    Ok(install_mesh(
        try_load_json_as_mesh(path)?,
        name,
        meshes,
        watch,
    ))
}

fn drawable(mesh: Handle<Mesh>, shader: Handle<RasterShaderProgram>) -> DrawableMeshBundle {
    DrawableMeshBundle {
        mesh,
        shader,
        transform: Transform::identity(),
    }
}

/// Actual gameplay Startup producer, not a no-argument schedule marker.
/// `root` and `demo` are explicit current-host equivalents of original CWD and
/// global143634d90. The device/backend/libraries all belong to this one App.
/// Original unwrap failures remain errors; no empty mesh/count/entity fallback.
#[allow(clippy::too_many_arguments)]
pub fn startup_terrain(
    root: &Path,
    demo: bool,
    device: &dyn GraphicsDevice,
    backend: &dyn ShaderBackend,
    meshes: &AssetMeshLibrary,
    watch: &MeshWatch,
    shaders: &AssetShaderLibrary,
    ssbos: &AssetSsboLibrary,
    commands: &mut Commands,
) -> anyhow::Result<()> {
    let mut floor = Mesh::new("grid cw");
    let mut indices = Vec::new();
    grid_gen_cw(&mut indices, 0, 90, 90, 90);
    floor.set_indices(indices);
    let floor_mesh = install_mesh(floor, "floor", meshes, watch);
    let shader = super::load_shaders::load_shader_into_library(
        shaders,
        backend,
        "/terrain_editing/floor.raster.hlsl",
        "floor_shader",
    );
    let floor = commands
        .spawn((KeepOnLevelUnload, drawable(floor_mesh, shader)))
        .id();

    // Original branch cmp143634d90 @140926e58: extra overlay only in demo.
    if demo {
        let mut mesh = Mesh::new("grid cw");
        let mut indices = Vec::new();
        grid_gen_cw(&mut indices, 0, 45, 45, 45);
        mesh.set_indices(indices);
        let mesh = install_mesh(mesh, "demo_text_floor", meshes, watch);
        let shader = super::load_shaders::load_shader_into_library(
            shaders,
            backend,
            "/demo_terrain_overlay.raster.hlsl",
            "demo_terrain_overlay",
        );
        commands.spawn((
            KeepOnLevelUnload,
            drawable(mesh, shader),
            DesignatedRenderPass(16),
        ));
    }

    let rocky =
        RockyTerrainMesh::load(&root.join("assets/data/rocky_terrain.json"), ssbos, device)?;
    commands.insert_resource(rocky.into_vertex_count());
    let shader = super::load_shaders::load_shader_into_library(
        shaders,
        backend,
        "/rocky_terrain/rocky_terrain.raster.hlsl",
        "rocky_terrain",
    );
    // Native reuses the floor mesh handle for the rocky indirect draw.
    let rocky = commands
        .spawn((KeepOnLevelUnload, drawable(floor_mesh, shader)))
        .id();
    commands.insert_resource(TerrainEntities { floor, rocky });

    let mesh = install_json_mesh(
        root,
        "assets/meshes/stairs_step.json",
        "stairs_step_mesh",
        meshes,
        watch,
    )?;
    let stairs_shader = super::load_shaders::load_shader_into_library(
        shaders,
        backend,
        "/rocky_terrain/stairs/stairs_indirect.raster.hlsl",
        "stairs_step",
    );
    commands.spawn((
        KeepOnLevelUnload,
        drawable(mesh, stairs_shader),
        StairsStep { flags: 0 },
    ));

    let mesh = install_json_mesh(
        root,
        "assets/meshes/path_pebble.json",
        "path_pebble_mesh",
        meshes,
        watch,
    )?;
    let shader = super::load_shaders::load_shader_into_library(
        shaders,
        backend,
        "/path_pebble.raster.hlsl",
        "path_pebble",
    );
    commands.spawn((KeepOnLevelUnload, drawable(mesh, shader)));

    let mesh = install_json_mesh(
        root,
        "assets/meshes/stairs_pebble.json",
        "stairs_pebble_mesh",
        meshes,
        watch,
    )?;
    // Native reuses stairs_shader and changes only StairsStep.flags to one.
    commands.spawn((
        KeepOnLevelUnload,
        drawable(mesh, stairs_shader),
        StairsStep { flags: 1 },
    ));

    let mesh = install_json_mesh(root, "assets/meshes/terrain.json", "terrain", meshes, watch)?;
    let shader = shaders
        .get_raster_handle_by_name("solid_vertex_color_albedo_map")
        .ok_or_else(|| {
            anyhow::anyhow!("startup_terrain missing common shader solid_vertex_color_albedo_map")
        })?;
    commands.spawn((
        KeepOnLevelUnload,
        drawable(mesh, shader),
        BackgroundTerrain,
        DontReceiveContactShadows,
        OverwriteRoughness(0.9),
    ));

    let mesh = install_json_mesh(
        root,
        "assets/meshes/terrain_rocks.json",
        "terrain_rocks",
        meshes,
        watch,
    )?;
    let shader = shaders
        .get_raster_handle_by_name("solid_vertex_color")
        .ok_or_else(|| {
            anyhow::anyhow!("startup_terrain missing common shader solid_vertex_color")
        })?;
    commands.spawn((KeepOnLevelUnload, drawable(mesh, shader), FlatLighting));

    let mesh = install_json_mesh(
        root,
        "assets/meshes/billboard_plants.json",
        "billboard_plants",
        meshes,
        watch,
    )?;
    let shader = super::load_shaders::load_shader_into_library(
        shaders,
        backend,
        "/billboard_plant.raster.hlsl",
        "billboard_plant_shader",
    );
    commands.spawn((KeepOnLevelUnload, drawable(mesh, shader), DontCastShadow));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;

    fn triangles(indices: &[u32]) -> BTreeSet<[u32; 3]> {
        indices
            .chunks_exact(3)
            .map(|t| [t[0], t[1], t[2]])
            .collect()
    }

    #[test]
    fn single_quad_has_the_two_exe_triangles() {
        let mut cw = Vec::new();
        grid_gen_cw(&mut cw, 0, 1, 1, 1);
        // S = 2：顶点 0 1 / 2 3。
        assert_eq!(cw, [0, 2, 1, 1, 2, 3]);
        let mut ccw = Vec::new();
        grid_gen_ccw(&mut ccw, 0, 1, 1, 1);
        assert_eq!(ccw, [0, 1, 2, 2, 1, 3]);
    }

    #[test]
    fn the_two_windings_are_mirror_images() {
        let (mut cw, mut ccw) = (Vec::new(), Vec::new());
        grid_gen_cw(&mut cw, 0, 7, 3, 7);
        grid_gen_ccw(&mut ccw, 0, 7, 3, 7);
        assert_eq!(cw.len(), 7 * 3 * 6);
        // 每个三角形反转两个顶点就是另一种缠绕。
        let flipped: BTreeSet<[u32; 3]> = triangles(&cw)
            .into_iter()
            .map(|[a, b, c]| [a, c, b])
            .collect();
        let ccw_set = triangles(&ccw);
        for t in &flipped {
            let mut rotations = [[t[0], t[1], t[2]], [t[1], t[2], t[0]], [t[2], t[0], t[1]]];
            rotations.sort();
            assert!(
                rotations.iter().any(|r| ccw_set.contains(r)),
                "triangle {t:?} has no CCW counterpart"
            );
        }
    }

    #[test]
    fn blocking_changes_the_order_but_not_the_triangle_set() {
        let mut blocked = Vec::new();
        grid_gen_cw(&mut blocked, 0, 11, 2, 11);
        let mut plain = Vec::new();
        for row in 0..2u32 {
            for col in 0..11u32 {
                let mut one = Vec::new();
                emit_block(&mut one, col, col + 1, 1, 11, false);
                // 单列单行的块在行 0；平移到第 `row` 行。
                plain.extend(one.iter().map(|i| i + row * 12));
            }
        }
        assert_eq!(triangles(&blocked), triangles(&plain));
        assert_ne!(
            blocked, plain,
            "11 columns split into 4 + 4 + 3, so the order differs"
        );
        // 前 4 列、两行都在第一块里：第二行的第一个三角形出现在第 5 个 quad 之前。
        assert_eq!(&blocked[..6], &[0, 12, 1, 1, 12, 13]);
        assert_eq!(&blocked[4 * 6..4 * 6 + 6], &[12, 24, 13, 13, 24, 25]);
    }

    #[test]
    fn native_wrapping_indices_and_columns_beyond_row_width_are_not_rejected() {
        let mut indices = Vec::new();
        grid_gen_cw(&mut indices, 2, 3, 1, 0);
        assert_eq!(indices, [2, 3, 3, 3, 3, 4]);
        let mut wrapped = Vec::new();
        grid_gen_cw(&mut wrapped, 0, 1, 1, u32::MAX);
        assert_eq!(wrapped, [0, 0, 1, 1, 0, 1]);
    }

    #[test]
    fn empty_ranges_and_zero_rows_emit_nothing() {
        let mut v = Vec::new();
        grid_gen_cw(&mut v, 3, 3, 4, 10);
        grid_gen_cw(&mut v, 0, 5, 0, 10);
        assert!(v.is_empty());
    }

    #[test]
    #[should_panic(expected = "网格列区间不能反向")]
    fn reversed_range_panics() {
        grid_gen_cw(&mut Vec::new(), 5, 4, 1, 8);
    }

    /// CPU integration of the actual shipped JSON loader, shared libraries and
    /// Commands. TestDevice verifies producer wiring only, not GPU execution.
    #[test]
    fn shipped_startup_publishes_actual_entities_mesh_sharing_and_stair_flags() {
        use crate::{
            render::test_device::TestDevice,
            resources::rocky_terrain::{
                RockyTerrainMeshVertCount, RockyTerrainTriangleSsbo, RockyTerrainVertexSsbo,
            },
        };
        use bevy_ecs::world::{CommandQueue, World};
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../../ljxsj_92385/Tiny Glade");
        for demo in [false, true] {
            let device = TestDevice::default();
            let meshes = AssetMeshLibrary::new();
            let watch = MeshWatch::new(false);
            let shaders = AssetShaderLibrary::new();
            let ssbos = AssetSsboLibrary::new();
            super::super::load_shaders::load_shaders(&shaders, &device, &mut None);
            let mut world = World::new();
            let mut queue = CommandQueue::default();
            startup_terrain(
                &root,
                demo,
                &device,
                &device,
                &meshes,
                &watch,
                &shaders,
                &ssbos,
                &mut Commands::new(&mut queue, &world),
            )
            .unwrap();
            queue.apply(&mut world);
            let entities = *world.resource::<TerrainEntities>();
            assert_ne!(entities.floor, entities.rocky);
            let floor_mesh = world.get::<Handle<Mesh>>(entities.floor).unwrap();
            assert_eq!(
                floor_mesh,
                world.get::<Handle<Mesh>>(entities.rocky).unwrap()
            );
            assert_eq!(*floor_mesh, meshes.get_handle_by_name("floor"));
            let floor_mesh = meshes.get(*floor_mesh).unwrap();
            assert_eq!(floor_mesh.indices.len(), 90 * 90 * 6);
            assert_eq!(&floor_mesh.indices[..6], &[0, 91, 1, 1, 91, 92]);
            assert_eq!(
                &floor_mesh.indices[4 * 6..5 * 6],
                &[91, 182, 92, 92, 182, 183]
            );
            assert!(floor_mesh.attributes.is_empty());
            assert!(floor_mesh.source_file.is_none());
            for (entity, name) in [
                (entities.floor, "floor_shader"),
                (entities.rocky, "rocky_terrain"),
            ] {
                assert!(world.get::<KeepOnLevelUnload>(entity).is_some());
                let handle = world.get::<Handle<RasterShaderProgram>>(entity).unwrap();
                assert_eq!(shaders.get_raster(*handle).unwrap().name, name);
            }
            assert!(world.resource::<RockyTerrainMeshVertCount>().0 > 0);
            assert!(ssbos.contains::<RockyTerrainVertexSsbo>());
            assert!(ssbos.contains::<RockyTerrainTriangleSsbo>());
            let mut drawables = world.query::<(&Handle<Mesh>, &KeepOnLevelUnload)>();
            assert_eq!(drawables.iter(&world).count(), if demo { 9 } else { 8 });
            let mut stairs =
                world.query::<(&Handle<Mesh>, &Handle<RasterShaderProgram>, &StairsStep)>();
            let mut flags = Vec::new();
            for (mesh, shader, step) in stairs.iter(&world) {
                assert_eq!(shaders.get_raster(*shader).unwrap().name, "stairs_step");
                let expected = if step.flags == 0 {
                    "stairs_step_mesh"
                } else {
                    "stairs_pebble_mesh"
                };
                assert_eq!(*mesh, meshes.get_handle_by_name(expected));
                flags.push(step.flags);
            }
            flags.sort_unstable();
            assert_eq!(flags, [0, 1]);
            let mut background = world.query::<(
                &BackgroundTerrain,
                &DontReceiveContactShadows,
                &OverwriteRoughness,
            )>();
            let (_, _, roughness) = background.single(&world).unwrap();
            assert_eq!(roughness.0.to_bits(), 0x3f66_6666);
            let mut overlay = world.query::<&DesignatedRenderPass>();
            let passes: Vec<_> = overlay.iter(&world).map(|p| p.0).collect();
            assert_eq!(passes, if demo { vec![16] } else { Vec::new() });
            assert_eq!(
                meshes.try_get_handle_by_name("demo_text_floor").is_some(),
                demo
            );
            if demo {
                let mesh = meshes.get_by_name("demo_text_floor").unwrap();
                assert_eq!(mesh.indices.len(), 45 * 45 * 6);
                assert!(mesh.attributes.is_empty());
            }
        }
    }
}
