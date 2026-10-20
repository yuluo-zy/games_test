//! 游戏启动与主题资源加载。调度分为一次性启动、会话请求、每帧更新。
//! 各资产加载器也可以由宿主独立调用，设备资源必须来自实际渲染上下文。

pub mod clearing;
pub mod init_color_library;
pub mod init_photomode_state;
pub mod init_populate_minimal_startup_ssbo_library;
pub mod init_populate_ssbo_library;
pub mod load_audio;
pub mod load_glade_audio;
pub mod load_glade_grading;
pub mod load_glade_tod_and_weather;
pub mod load_mesh_atlases;
pub mod load_prefabs;
pub mod load_shaders;
pub mod load_textures;
pub mod run_game;
pub mod spawn_brush_previews;
pub mod spawn_human_ref;
pub mod spawn_preplaced_trees;
pub mod startup_blit;
pub mod startup_critters;
pub mod startup_deferred_raster;
pub mod startup_garden_brush;
pub mod startup_grass;
pub mod startup_ivy;
pub mod startup_meadow;
pub mod startup_nani;
pub mod startup_roof;
pub mod startup_session;
pub mod startup_sheep;
pub mod startup_sky_and_sun;
pub mod startup_terrain;
pub mod startup_time_of_day;
pub mod startup_walls;
pub mod startup_water;

#[cfg(test)]
mod tests;

use crate::render::shader::compile_shader_variants;
use crate::{
    asset_libraries::{Handle, shader_library::AssetShaderLibrary},
    render::shader::{RasterShaderProgram, boundary::ShaderBackend},
    rhapsody_types::RasterPipelineDesc,
};

/// 编译两个光栅变体后注册；编译失败返回错误，避免 Result 接口内部隐藏 panic。
pub fn load_shader_into_library(
    name: &'static str,
    desc: RasterPipelineDesc,
    device: &dyn ShaderBackend,
    library: &AssetShaderLibrary,
) -> anyhow::Result<Handle<RasterShaderProgram>> {
    let variants = compile_shader_variants(&desc, device)?;
    Ok(library.add_raster(
        RasterShaderProgram {
            name,
            desc,
            variants,
        },
        false,
    ))
}

/// 启动资产宿主：与原作 `load_mesh_into_library` / `load_shader_into_library` /
/// `Commands::spawn` 三件套一一对应的最小接口。
///
/// 设备与命令所有者属于宿主 App；这些模块只复现原作的资源表、调用顺序和实体
/// 组合，不合成 shader、不伪造句柄。测试使用记录器实现同一接口。
pub trait StartupAssetHost {
    /// 注册后的网格句柄类型。
    type Mesh: Copy;
    /// 注册后的着色器句柄类型。
    type Shader: Copy;

    /// 注册一个网格。`name` 为 `None` 时对应原作的匿名（仅路径）加载。
    fn load_mesh(&mut self, path: &'static str, name: Option<&'static str>) -> Self::Mesh;

    /// 编译并注册一个光栅着色器。
    fn load_shader(&mut self, path: &'static str, name: &'static str) -> Self::Shader;

    /// 按名查已导入的网格（mesh atlas/prefab 通道）。名式加载不得伪造文件路径；
    /// 查不到时返回 `None`，调用方保留 U。
    fn lookup_mesh(&mut self, _name: &str) -> Option<Self::Mesh> {
        None
    }

    /// 按名查已注册的着色器；查不到时返回 `None`。
    fn lookup_shader(&mut self, _name: &str) -> Option<Self::Shader> {
        None
    }

    /// 生成一个绘制实体。
    fn spawn_drawable(&mut self, mesh: Self::Mesh, shader: Self::Shader);
}

/// 读取并注册一个网格文件。启动资产表可能在无安装目录的测试宿主中运行：
/// 文件缺失时记录错误并跳过，不注册伪造资产；显式宿主调用可对 `None` 断言。
pub fn register_mesh_file(
    meshes: &crate::asset_libraries::mesh_library::AssetMeshLibrary,
    path: &str,
    name: Option<&str>,
) -> Option<crate::asset_libraries::Handle<crate::render::mesh::Mesh>> {
    use crate::asset_libraries::mesh_library::MeshAsset;
    if !std::path::Path::new(path).is_file() {
        log::error!(
            target: "country_core::startup",
            "mesh file {path} is absent; skipping (no fabricated asset)"
        );
        return None;
    }
    let mesh = crate::utils::load_json::load_json_as_mesh(path);
    Some(meshes.add(MeshAsset::new(mesh, name.map(ToOwned::to_owned))))
}

/// 生产用 [`StartupAssetHost`]：直接操作宿主世界中的真实资产库与 Commands
/// 语义（`World::spawn`）。设备仍由调用方以 `&dyn ShaderBackend` 提供。
pub struct WorldAssetHost<'w> {
    pub world: &'w mut bevy_ecs::world::World,
    pub backend: &'w dyn ShaderBackend,
}

impl StartupAssetHost for WorldAssetHost<'_> {
    type Mesh = crate::asset_libraries::Handle<crate::render::mesh::Mesh>;
    type Shader = crate::asset_libraries::Handle<RasterShaderProgram>;

    fn load_mesh(
        &mut self,
        path: &'static str,
        name: Option<&'static str>,
    ) -> Self::Mesh {
        let meshes = self
            .world
            .resource::<crate::asset_libraries::mesh_library::AssetMeshLibrary>();
        register_mesh_file(meshes, path, name)
            .unwrap_or_else(|| panic!("startup mesh file {path} is absent"))
    }

    fn load_shader(&mut self, path: &'static str, name: &'static str) -> Self::Shader {
        let shaders = self
            .world
            .resource::<crate::asset_libraries::shader_library::AssetShaderLibrary>();
        load_shaders::load_shader_into_library(shaders, self.backend, path, name)
    }

    fn lookup_mesh(&mut self, name: &str) -> Option<Self::Mesh> {
        self.world
            .resource::<crate::asset_libraries::mesh_library::AssetMeshLibrary>()
            .try_get_handle_by_name(name)
    }

    fn lookup_shader(&mut self, name: &str) -> Option<Self::Shader> {
        self.world
            .resource::<crate::asset_libraries::shader_library::AssetShaderLibrary>()
            .get_raster_handle_by_name(name)
    }

    fn spawn_drawable(&mut self, mesh: Self::Mesh, shader: Self::Shader) {
        use crate::components::drawable::DrawableMeshBundle;
        use crate::utils::components::transform::Transform;
        self.world.spawn(DrawableMeshBundle {
            mesh,
            shader,
            transform: Transform::identity(),
        });
    }
}

/// OnEnter(Game) 批的资产产者。契约 §1.5/§2：batch2 = OnEnter(Game)；
/// batch4 与 batch2 同 label（I）。`spawn_preplaced_trees` 由 save 注册，不在此表；
/// `load_audio`（batch1/PreStartup）与 `load_glade_grading` 需要宿主音频/设备容器，
/// 由宿主调用其宿主接口，不在这里伪造 ECS 资源。
fn add_startup_mesh_systems(session_schedule: &mut bevy_ecs::schedule::Schedule) {
    use bevy_ecs::schedule::{
        IntoScheduleConfigs,
        common_conditions::{resource_exists, run_once},
    };
    // 用例/无渲染宿主可能不装配资产库；缺库时整批跳过，不触发参数校验错误。
    let assets_present = resource_exists::<crate::asset_libraries::mesh_library::AssetMeshLibrary>;
    // 顺序：batch2 世界资产（walls→sky→critters→roof→clearing→meadow），
    // 其后 batch4（brush→human→tod）。
    session_schedule.add_systems(
        (
            startup_walls::startup_walls_meshes,
            startup_sky_and_sun::startup_sky_and_sun_meshes,
            startup_critters::startup_critters_meshes,
            startup_roof::startup_roof_meshes,
            clearing::init_clearing_meshes,
            startup_meadow::startup_meadow_asset_files,
            spawn_brush_previews::spawn_brush_previews_meshes,
        )
            .chain()
            .run_if(run_once)
            .run_if(assets_present),
    );
    // `startup_sheep` 在契约里同属 batch2，但它的网格由羊域动画注册链消费；
    // 这里仍写入真实网格库，紧随 batch2 世界资产之后。
    session_schedule.add_systems(
        startup_sheep::startup_sheep_meshes
            .run_if(run_once)
            .run_if(assets_present),
    );
}

/// 装配启动、会话、更新三个阶段；网格初始化只进入启动表。
pub fn add_game_startup_systems<B, S>(
    world: &mut bevy_ecs::world::World,
    startup_schedule: &mut bevy_ecs::schedule::Schedule,
    session_schedule: &mut bevy_ecs::schedule::Schedule,
    update_schedule: &mut bevy_ecs::schedule::Schedule,
) where
    B: crate::systems::audio::music::music_player::MusicBackend + bevy_ecs::resource::Resource,
    S: crate::resources::audio_resources::Studio + Send + Sync + 'static,
{
    use bevy_ecs::schedule::{IntoScheduleConfigs, common_conditions::run_once};
    run_game::register_core_startup::<B, S>(
        world,
        startup_schedule,
        session_schedule,
        update_schedule,
    );
    startup_schedule.add_systems(
        (
            startup_garden_brush::startup_garden_brush,
            startup_ivy::startup_ivy,
        )
            .run_if(run_once),
    );
}
