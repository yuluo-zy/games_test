//! 游戏启动与主题资源加载。调度分为一次性启动、会话请求、每帧更新。
//! 各资产加载器也可以由宿主独立调用，设备资源必须来自实际渲染上下文。

pub mod init_color_library;
pub mod init_photomode_state;
pub mod load_glade_audio;
pub mod load_glade_tod_and_weather;
pub mod load_mesh_atlases;
pub mod load_prefabs;
pub mod load_shaders;
pub mod load_textures;
pub mod run_game;
pub mod startup_blit;
pub mod startup_deferred_raster;
pub mod startup_garden_brush;
pub mod startup_ivy;
pub mod startup_nani;
pub mod startup_session;
pub mod startup_terrain;

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
