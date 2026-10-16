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
/// 各域恢复后并入，不在这里注册空系统。B = 宿主 MusicBackend，S = 宿主 Studio
/// （audio 系统族通用；feature native-audio 提供 FmodMusicBackend / FMOD Studio 绑定）。
///
/// ⑤轮 A 块（① a-ecs-scheduler-integration 契约 §1）：G1-G4 asset 载入组并入。
/// 现 crate 可挂载的 ECS 系统为 G2 mesh 组（startup_garden_brush / startup_ivy，
/// `Res<AssetMeshLibrary>`；宿主提供该渲染资源）；G1 其余（load_textures/load_shader/
/// SSBO-cursor/load_prefabs/config-watch/terrain-height/audio-samples）与 G3/G4 为渲染
/// 域系统（GraphicsDevice 依赖，D组 rhapsody/host 装配面），登记待接线；28-tuple 槽位
/// 精确映射待①取证。
///
/// **⑥轮收官（① r6-28-system-names 契约 §1，宿主接线面）**：GameStartup 28-系统 =
/// G1→G4 顺序 add_systems(Startup)，宿主 GameLoop 驱动。系统名（into_configs 权威）：
/// - **G1**（FUN_140a2bf40，10 资产载入）：load_textures / load_shader 变体1 /
///   load_shader+FinalPost / custom-cursor SSBO / load_prefabs / config-watch /
///   纯 Commands（身份待①）/ terrain 高度+PathRaster / audio-samples promise。
/// - **G2**（0x140839b80，8 启动/渲染）：init_photomode_state / startup ToD·weather·glade /
///   mesh 载入族 ×3 / color-SSBO / load_textures 变体 / ScheduledTerrainReadback。
/// - **G3**（0x1415f4750，1）：mesh-atlas 装配；**G4**（0x1409c8600，1）：shader 变体。
/// - **G5/G6**（0x141736760 / 0x140b6bd50，6+6 音频/渲染）：见 round5 a 契约。
/// 实体由宿主/D组提供，本 crate 不重建（纯 Commands 身份待①）。
pub fn add_game_startup_systems<B, S>(
    world: &mut bevy_ecs::world::World,
    schedule: &mut bevy_ecs::schedule::Schedule,
)
where
    B: crate::systems::audio::music::music_player::MusicBackend + bevy_ecs::resource::Resource + 'static,
    S: crate::resources::audio_resources::Studio + Send + Sync + 'static,
{
    run_game::register_core_startup::<B, S>(world, schedule);
    // G2 mesh 载入族（0x140839b80，O）：startup_garden_brush / startup_ivy
    // （Res<AssetMeshLibrary>；宿主提供 AssetMeshLibrary 资源）。
    schedule.add_systems((
        startup_garden_brush::startup_garden_brush,
        startup_ivy::startup_ivy,
    ));
}
