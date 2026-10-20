//! Meadow 植物资源（B `startup_meadow::init_meadow` @0x1408639f0）。
//!
//! 证据（R10 契约 §2.8 + 发行资产实读）：
//! * batch2/OnEnter(Game)；输入 `Res<ActiveSession>` assets root、shader/mesh 库、Commands。
//! * 配置 `<assets_root>/assets/glade/<theme>/plants.json`，条目
//!   `{ shader, mesh, placements }`（发行资产实读）。
//! * shader `/clearing/meadow_plant.raster.hlsl`（name `meadow_plant_shader`）；
//!   `deserialize plants` 日志；1 个 spawn（bundle 0x88）。
//!
//! U：placements 文件的实例化消费链与 1×0x88 bundle 字段；plants 条目的
//! `mesh` 名对应 `assets/meshes/<mesh>.json`（plant_thistle/plant_leafy 等实存）。
use crate::asset_libraries::mesh_library::AssetMeshLibrary;
use crate::resources::session::ActiveSession;
use super::{StartupAssetHost, register_mesh_file};
use bevy_ecs::prelude::Res;
use std::path::{Path, PathBuf};

pub const PLANTS_FILE_NAME: &str = "plants.json";
pub const MEADOW_PLANT_SHADER: (&str, &str) = (
    "/clearing/meadow_plant.raster.hlsl",
    "meadow_plant_shader",
);
/// O：init_meadow 的 spawn 记录数与 bundle 尺寸。
pub const MEADOW_SPAWN_BUNDLE: usize = 0x88;

#[derive(Clone, Debug, serde::Deserialize)]
pub struct MeadowPlant {
    pub shader: String,
    pub mesh: String,
    pub placements: String,
}

/// O：`<assets_root>/assets/glade/<theme>/plants.json`。
pub fn plants_json_path(assets_root: impl AsRef<Path>, theme: &str) -> PathBuf {
    assets_root
        .as_ref()
        .join("assets")
        .join("glade")
        .join(theme)
        .join(PLANTS_FILE_NAME)
}

pub fn load_plants_from_str(text: &str) -> anyhow::Result<Vec<MeadowPlant>> {
    Ok(serde_json::from_str(text)?)
}

pub fn load_plants(path: impl AsRef<Path>) -> anyhow::Result<Vec<MeadowPlant>> {
    load_plants_from_str(&std::fs::read_to_string(path)?)
}

/// 校验 plants 条目：shader 名必须已在库内；网格注册由
/// [`startup_meadow_asset_files`] 用动态路径完成（host trait 的静态路径 API
/// 不承载运行时配置名）。
pub fn init_meadow<H: StartupAssetHost>(host: &mut H, plants: &[MeadowPlant]) {
    for plant in plants {
        if host.lookup_shader(&plant.shader).is_none()
            && plant.shader != MEADOW_PLANT_SHADER.1
        {
            log::warn!(
                target: "country_core::startup::startup_meadow",
                "U: shader {} not registered",
                plant.shader
            );
        }
    }
    // U: 1×0x88 spawn（placements 实例化的 bundle 字段未恢复）。
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
    let theme = session.meta.glade.assets_folder();
    let path = plants_json_path(&assets_root, theme);
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
    // U: 1×0x88 spawn 依赖 placements 的实例化 schema。
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
            plants_json_path("root", "summer"),
            PathBuf::from("root/assets/glade/summer/plants.json")
        );
        assert_eq!(
            MEADOW_PLANT_SHADER,
            ("/clearing/meadow_plant.raster.hlsl", "meadow_plant_shader")
        );
        assert_eq!(MEADOW_SPAWN_BUNDLE, 0x88);
    }
}
