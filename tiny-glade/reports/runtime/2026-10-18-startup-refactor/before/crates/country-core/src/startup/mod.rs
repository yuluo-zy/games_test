//! `country_core::startup` — 启动期生产者。只放已经对着 EXE 核对过的纯函数部分；
//! 载入网格/着色器/实体生成的系统（`load_prefabs`、`startup_terrain` 本体等）依赖 ECS 与资产库，尚未做。
//! `startup_ivy` / `startup_garden_brush` 的网格注册链（路径/名字已对 B 图像 read_memory 核实）已实现；
//! shader 与 spawn 记录依赖渲染管线，登记待接线。
//!
//! `add_game_startup_systems` 0x140941110（O）：GameStartup 插件 build 注册 28-系统 tuple；
//! `load_shader_into_library` / `load_mesh_into_library`（O）。

pub mod init_photomode_state;
pub mod init_color_library;
pub mod load_glade_audio;
pub mod load_prefabs;
pub mod load_textures;
pub mod load_shaders;
pub mod startup_blit;
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
///
/// **fresh 反编译（2026-10-17，134 B）**：`country_core::startup::impl$0::
/// add_game_startup_systems` **无参数**（PDB `void __cdecl (void)`；由 `run_game`
/// 0x1405949c5 无条件调用）。函数体 = 5 次 `SubApp::add_systems`，调度分别为
/// `PreStartup` → `OnEnter<AppLoadingState>` → `OnExit<AppLoadingState>` → 再两次
/// `add_systems`（调度面由宿主装配）。**不是 `Startup` 单调度**。
///
/// 系统清单经 `IntoScheduleConfigs::into_configs` 组构建器（模板实参即系统）：
/// - **G1** 0x140a2bf40（10 系统）：load_textures / load_shader 变体×2 /
///   config-watch(ConfigWatch) / custom-cursor SSBO(CustomCursorSsboCreatedCmd) /
///   audio-samples promise / terrain-height / 纯 Commands×2 / load_prefabs(PrefabWatcher)。
/// - **G2** 0x140839b80（8 系统）：init_photomode_state(closure_env$0) /
///   mesh 载入族(AssetMeshLibrary) / ActiveSession+AssetTextureLibrary(create_preset_textures) /
///   TodSequence+Weather 等。
/// - **G3** 0x1415f4750（1）：PrefabLibrary 装配；**G4** 0x1409c8600（1）：
///   GraphicsDeviceResource shader 变体。
/// - **G5** 0x141736760（11）/ **G6** 0x140b6bd50（7）：mesh 载入族×7 + shader +
///   AudioCore + ImageFormationConfigs（G5）；AudioCore/MainCamera + AppMode 门 +
///   forest_audio / clutter_audio(PlayClutterSoundCmd) / elevation_supports_audio
///   （PlayElevationSupportSoundCmd）等（G6）。
///
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
    // 音频曲库接线（2026-10-17）：`music_audio_init_system<B>` 已建 MusicAudioState；
    // 宿主在 Glade 会话主题确定后调用
    // `load_glade_audio::load_glade_audio(&mut player, &backend, glade_dir, theme_byte)`
    // （0x1408be360 域内链：stop_immediately + HALLOWEEN_MUSIC_EVENTS load_tracks +
    // 主题音量门 + GladeTypeIsHalloween/IsHalloween 参数）。GladeAudio::load（audio.json）
    // 的 ambience/stinger 实例化与 FMOD 启动由音频域/host 接入，不在此注册空系统。
}

pub mod load_glade_tod_and_weather;
