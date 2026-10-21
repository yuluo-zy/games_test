//! Meadow 植物资源（B `startup_meadow::init_meadow` @0x1408639f0）。
//!
//! 证据（R10 契约 §2.8 + 发行资产实读）：
//! * batch2/OnEnter(Game)；输入 `Res<ActiveSession>` assets root、shader/mesh 库、Commands。
//! * 配置 `<assets_root>/assets/glade/<theme>/plants.json`，条目
//!   `{ shader, mesh, placements }`（发行资产实读）。
//! * shader `/clearing/meadow_plant.raster.hlsl`（name `meadow_plant_shader`）；
//!   `deserialize plants` 日志；1 个 spawn（bundle 0x88）。
//!
//! PlacementDataSsbo uses the canonical 48-byte record and allocation owner.
//! Each config entry emits one identity drawable plus a placement handle;
//! draw callback env5 consumes that same handle to select the instance count.
use super::register_mesh_file;
use crate::asset_libraries::{
    mesh_library::{AssetMeshLibrary, MeshAsset},
    meshwatch::MeshWatch,
    placement_data_library::AssetPlacementDataLibrary,
    shader_library::AssetShaderLibrary,
};
use crate::components::drawable::DrawableMeshBundle;
use crate::render::{shader::boundary::ShaderBackend, ssbo::rhapsody_boundary::GraphicsDevice};
use crate::resources::session::ActiveSession;
use crate::systems::render_loop::draw_passes::draw_callbacks_b::InstancedDrawFromPlacementData;
use crate::utils::components::transform::Transform;
use bevy_ecs::prelude::{Commands, Res};
use std::path::{Path, PathBuf};

pub const PLANTS_FILE_NAME: &str = "plants.json";
pub const MEADOW_PLANT_SHADER: (&str, &str) =
    ("/clearing/meadow_plant.raster.hlsl", "meadow_plant_shader");
/// O：init_meadow 的 spawn 记录数与 bundle 尺寸。
pub const MEADOW_SPAWN_BUNDLE: usize = 0x88;

#[derive(Clone, Debug, serde::Deserialize)]
pub struct MeadowPlant {
    pub shader: String,
    pub mesh: String,
    pub placements: String,
}

/// Session assets root already includes `assets/glade/<theme>`.
pub fn plants_json_path(assets_root: impl AsRef<Path>) -> PathBuf {
    assets_root.as_ref().join(PLANTS_FILE_NAME)
}

pub fn load_plants_from_str(text: &str) -> anyhow::Result<Vec<MeadowPlant>> {
    Ok(serde_json::from_str(text)?)
}

pub fn load_plants(path: impl AsRef<Path>) -> anyhow::Result<Vec<MeadowPlant>> {
    load_plants_from_str(&std::fs::read_to_string(path)?)
}

/// Original @1408639f0: compile common plant shader, parse each placement
/// config, upload its actual instance records, load/watch mesh, resolve the
/// named shader and spawn. Source-root parameter adapts native relative paths.
#[allow(clippy::too_many_arguments)]
pub fn init_meadow(
    root: &Path,
    session: &ActiveSession,
    device: &dyn GraphicsDevice,
    backend: &dyn ShaderBackend,
    meshes: &AssetMeshLibrary,
    watch: &MeshWatch,
    shaders: &AssetShaderLibrary,
    placements: &mut AssetPlacementDataLibrary,
    commands: &mut Commands,
) -> anyhow::Result<()> {
    use anyhow::Context;
    super::load_shaders::load_shader_into_library(
        shaders,
        backend,
        MEADOW_PLANT_SHADER.0,
        MEADOW_PLANT_SHADER.1,
    );
    let glade_root = root.join(session.assets_root());
    let plants = load_plants(plants_json_path(&glade_root)).context("deserialize plants")?;
    for plant in plants {
        let (_records, handle) = placements.load(device, glade_root.join(&plant.placements));
        let path = root
            .join("assets/meshes")
            .join(&plant.mesh)
            .with_extension("json");
        let mesh = crate::utils::load_json::try_load_json_as_mesh(
            path.to_str().context("meadow mesh path is not UTF-8")?,
        )?;
        watch.watch(&mesh);
        let mesh = meshes.add(MeshAsset::new(mesh, Some(plant.mesh.clone())));
        let shader = shaders
            .get_raster_handle_by_name(&plant.shader)
            .with_context(|| format!("meadow shader {} is not registered", plant.shader))?;
        commands.spawn((
            DrawableMeshBundle {
                mesh,
                shader,
                transform: Transform::identity(),
            },
            InstancedDrawFromPlacementData(handle.id().raw()),
        ));
    }
    Ok(())
}

/// ECS 产者：读取会话格调的 plants.json 并注册真实网格文件。
pub fn startup_meadow_asset_files(
    meshes: Res<AssetMeshLibrary>,
    session: Option<Res<ActiveSession>>,
) {
    let Some(session) = session else {
        return;
    };
    let assets_root = session.assets_root();
    let path = plants_json_path(&assets_root);
    let plants = match load_plants(&path) {
        Ok(plants) => plants,
        Err(error) => {
            log::error!(
                target: "country_core::startup::startup_meadow",
                "deserialize plants: {error:#}"
            );
            return;
        }
    };
    for plant in &plants {
        let mesh_path = format!("assets/meshes/{}.json", plant.mesh);
        if Path::new(&mesh_path).is_file() {
            register_mesh_file(&meshes, &mesh_path, Some(&plant.mesh));
        }
    }
    // CPU-only loader; the GPU owner calls init_meadow for complete entities.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shipped_plants_schema_and_paths_are_exact() {
        let plants = load_plants_from_str(
            r#"[{"shader":"meadow_plant_shader","mesh":"plant_thistle","placements":"plants/flowers.json"}]"#,
        )
        .unwrap();
        assert_eq!(plants.len(), 1);
        assert_eq!(plants[0].mesh, "plant_thistle");
        assert_eq!(
            plants_json_path("assets/glade/summer"),
            PathBuf::from("assets/glade/summer/plants.json")
        );
        assert_eq!(
            MEADOW_PLANT_SHADER,
            ("/clearing/meadow_plant.raster.hlsl", "meadow_plant_shader")
        );
        assert_eq!(MEADOW_SPAWN_BUNDLE, 0x88);
    }

    #[test]
    fn original_flowery_placements_reach_the_canonical_buffer_and_drawable() {
        use crate::asset_libraries::Handle;
        use crate::render::{mesh::Mesh, shader::RasterShaderProgram, test_device::TestDevice};
        use crate::resources::session::{GladeType, SessionId, SessionMeta};
        use bevy_ecs::world::{CommandQueue, World};
        let root = std::env::var_os("TG_INSTALL_ROOT")
            .map(std::path::PathBuf::from)
            .unwrap_or_else(|| Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../ljxsj_92385/Tiny Glade"));
        let meshes = AssetMeshLibrary::new();
        let shaders = AssetShaderLibrary::new();
        let watch = MeshWatch::new(false);
        let mut library = AssetPlacementDataLibrary::new();
        let session = ActiveSession::new(
            SessionId::new(),
            SessionMeta {
                glade: GladeType::Flowery,
                ..SessionMeta::default()
            },
        );
        let mut world = World::new();
        let mut queue = CommandQueue::default();
        init_meadow(
            &root,
            &session,
            &TestDevice::default(),
            &TestDevice::default(),
            &meshes,
            &watch,
            &shaders,
            &mut library,
            &mut Commands::new(&mut queue, &world),
        )
        .unwrap();
        queue.apply(&mut world);
        let path = root
            .join(session.assets_root())
            .join("plants/leafy_plants.json");
        let records: Vec<crate::resources::placement_data::PlacementDataSsbo> =
            serde_json::from_str(&std::fs::read_to_string(&path).unwrap()).unwrap();
        assert!(!records.is_empty());
        let handle = library.paths[&path];
        let (buffer, count) = &library.buffers[&handle];
        assert_eq!(*count, records.len());
        assert_eq!(buffer.buffer.byte_size, (records.len() * 48) as u64);
        let mut query = world.query::<(
            &Handle<Mesh>,
            &Handle<RasterShaderProgram>,
            &Transform,
            &InstancedDrawFromPlacementData,
        )>();
        let entries: Vec<_> = query.iter(&world).collect();
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].3.0, handle.id().raw());
        assert_eq!(entries[0].2.compute_matrix(), glam::Mat4::IDENTITY);
        assert_eq!(
            meshes.try_get_handle_by_name("plant_leafy"),
            Some(*entries[0].0)
        );
        assert_eq!(
            shaders.get_raster_handle_by_name(MEADOW_PLANT_SHADER.1),
            Some(*entries[0].1)
        );
    }
}
