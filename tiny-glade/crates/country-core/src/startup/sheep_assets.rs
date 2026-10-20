//! Complete original startup_sheep asset branch140a093e0. ECS sheep/audio
//! creation is owned by startup_sheep; all handles below are real library keys.
use crate::{
    asset_libraries::{
        Handle,
        mesh_library::{AssetMeshLibrary, MeshAsset},
        meshwatch::MeshWatch,
        shader_library::AssetShaderLibrary,
        ssbo_library::AssetSsboLibrary,
    },
    render::{
        mesh::{ATTR_POSITION, Mesh},
        shader::{RasterShaderProgram, boundary::ShaderBackend},
        ssbo::rhapsody_boundary::GraphicsDevice,
    },
    systems::render_loop::resources::critters::SheepAnimationSsbo,
};
use anyhow::Context;
use std::path::Path;
pub struct SheepStartupAssets {
    pub sheep_mesh: Handle<Mesh>,
    pub sheep_shader: Handle<RasterShaderProgram>,
    pub heart_mesh: Handle<Mesh>,
    pub umbrella_mesh: Handle<Mesh>,
    pub floatie_mesh: Handle<Mesh>,
    pub accessory_shader: Handle<RasterShaderProgram>,
}
fn load(root: &Path, path: &str) -> anyhow::Result<Mesh> {
    let path = root.join(path);
    crate::utils::load_json::try_load_json_as_mesh(
        path.to_str().context("sheep mesh path is not UTF-8")?,
    )
}
fn install(mesh: Mesh, name: &str, meshes: &AssetMeshLibrary, watch: &MeshWatch) -> Handle<Mesh> {
    watch.watch(&mesh);
    meshes.add(MeshAsset::new(mesh, Some(name.to_owned())))
}
/// Original vertex-major, then frame1..=30 packing. Do not enforce equal
/// lengths: original ignores extra frame vertices and bounds-checks each
/// requested position and normal only when its vertex is actually reached.
pub fn build_sheep_animation(
    base: &Mesh,
    frames: &[Mesh; 30],
    delighted: &Mesh,
) -> Vec<SheepAnimationSsbo> {
    build_sheep_animation_rows(sheep_vertex_count(base), frames, delighted)
}
fn sheep_vertex_count(base: &Mesh) -> usize {
    // Native uses the original array len at this point, without a Float3 tag
    // check. Per-frame and delighted access below does check Float3.
    base.attributes
        .get(ATTR_POSITION)
        .expect("called `Option::unwrap()` on a `None` value")
        .array_length()
}
fn build_sheep_animation_rows(
    vertex_count: usize,
    frames: &[Mesh; 30],
    delighted: &Mesh,
) -> Vec<SheepAnimationSsbo> {
    let mut rows = Vec::new();
    for vertex in 0..vertex_count {
        let mut positions = [[0.0; 4]; 30];
        let mut normals = [[0.0; 4]; 30];
        for (frame, mesh) in frames.iter().enumerate() {
            let p = mesh.positions_without_indices()[vertex];
            positions[frame] = [p.x, p.y, p.z, 0.0];
            let n = mesh.normals_without_indices()[vertex];
            normals[frame] = [n.x, n.y, n.z, 0.0];
        }
        let p = delighted.positions_without_indices()[vertex];
        let n = delighted.normals_without_indices()[vertex];
        rows.push(SheepAnimationSsbo {
            positions,
            normals,
            delighted_pos: [p.x, p.y, p.z, 0.0],
            delighted_normal: [n.x, n.y, n.z, 0.0],
        });
    }
    rows
}
#[allow(clippy::too_many_arguments)]
pub fn load_sheep_startup_assets(
    installed: &Path,
    meshes: &AssetMeshLibrary,
    shaders: &AssetShaderLibrary,
    backend: &dyn ShaderBackend,
    watch: &MeshWatch,
    ssbos: &AssetSsboLibrary,
    device: &dyn GraphicsDevice,
) -> anyhow::Result<SheepStartupAssets> {
    let sheep_mesh = if let Some(handle) = meshes.try_get_handle_by_name("sheep_mesh") {
        handle
    } else {
        let base = load(installed, "assets/meshes/sheep_animation/1.json")?;
        let delighted = load(installed, "assets/meshes/sheep_animation/delighted.json")?;
        let vertex_count = sheep_vertex_count(&base);
        // Original collect executes every source load, including frame1 again.
        let frames = (1..=30)
            .map(|frame| {
                load(
                    installed,
                    &format!("assets/meshes/sheep_animation/{frame}.json"),
                )
            })
            .collect::<anyhow::Result<Vec<_>>>()?;
        let frames: [Mesh; 30] = frames
            .try_into()
            .unwrap_or_else(|_| unreachable!("fixed thirty-frame source range"));
        let animation = build_sheep_animation_rows(vertex_count, &frames, &delighted);
        ssbos.add_new_with_data::<SheepAnimationSsbo>(device, &animation);
        install(base, "sheep_mesh", meshes, watch)
    };
    let sheep_shader = super::load_shaders::load_shader_into_library(
        shaders,
        backend,
        "/sheep.raster.hlsl",
        "sheep_shader",
    );
    let heart_mesh = install(
        load(installed, "assets/meshes/sheep_heart.json")?,
        "sheep_heart",
        meshes,
        watch,
    );
    let umbrella_mesh = install(
        load(installed, "assets/meshes/umbrella.json")?,
        "umbrella_mesh",
        meshes,
        watch,
    );
    let floatie_mesh = install(
        load(installed, "assets/meshes/floatie.json")?,
        "floatie_mesh",
        meshes,
        watch,
    );
    let accessory_shader = shaders
        .get_raster_handle_by_name("solid_vertex_color")
        .context("solid_vertex_color shader must already be registered")?;
    Ok(SheepStartupAssets {
        sheep_mesh,
        sheep_shader,
        heart_mesh,
        umbrella_mesh,
        floatie_mesh,
        accessory_shader,
    })
}
