//! `country_core::startup` — 启动期生产者。只放已经对着 EXE 核对过的纯函数部分；
//! 载入网格/着色器/实体生成的系统（`load_prefabs`、`startup_terrain` 本体等）依赖 ECS 与资产库，尚未做。
//! `startup_ivy` / `startup_garden_brush` 的网格注册链（路径/名字已对 B 图像 read_memory 核实）已实现；
//! shader 与 spawn 记录依赖渲染管线，登记待接线。
//!
//! `add_game_startup_systems` 0x140941110（O）：GameStartup 插件 build 注册 28-系统 tuple；
//! `load_shader_into_library` / `load_mesh_into_library`（O）。

pub mod init_photomode_state;
pub mod load_glade_audio;
pub mod load_textures;
pub mod run_game;
pub mod startup_garden_brush;
pub mod startup_ivy;
pub mod startup_session;
pub mod startup_terrain;

use crate::{
    asset_libraries::{shader_library::AssetShaderLibrary, Handle},
    render::shader::{boundary::ShaderBackend, RasterShaderProgram},
    rhapsody_types::RasterPipelineDesc,
};

/// `load_shader_into_library`（O 0x140b3fe40<&str> / 0x140b40210<String>）：clone 名字 +
/// `compile_shader_variants` + `AssetShaderLibrary::add_raster`。失败按原版 `unwrap` panic。
pub fn load_shader_into_library(
    name: &'static str,
    desc: RasterPipelineDesc,
    device: &dyn ShaderBackend,
    library: &AssetShaderLibrary,
) -> anyhow::Result<Handle<RasterShaderProgram>> {
    let program = RasterShaderProgram::new_draw(name, desc, device);
    Ok(library.add_raster(program, false))
}

/// `add_game_startup_systems` 0x140941110：GameStartup 注册合同。
/// 原版注册 28-系统 tuple（含 AssetTextureLibrary::create_preset_textures + 4 子组构建器）
/// + audio startup（systems/audio/mod.rs:144）+ 排程 wiring。
/// 当前可挂载的注册见 `run_game::register_core_startup`；域系统（墙/屋顶/地形/水…）由
/// 各域恢复后并入，不在这里注册空系统。B = 宿主 MusicBackend。
pub fn add_game_startup_systems<B>(world: &mut bevy_ecs::world::World, schedule: &mut bevy_ecs::schedule::Schedule)
where
    B: crate::systems::audio::music::music_player::MusicBackend + bevy_ecs::resource::Resource + 'static,
{
    run_game::register_core_startup::<B>(world, schedule);
}
