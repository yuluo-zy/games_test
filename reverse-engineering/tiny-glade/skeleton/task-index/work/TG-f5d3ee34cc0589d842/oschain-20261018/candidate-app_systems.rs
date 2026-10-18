//! Original Tiny Glade host wall/session lifecycle. The complete unload body
//! @1402b1cb0 and Reload→Game PostUpdate consumer are recovered below.
//! Broader normal wall-system/event registration remains a separate unmounted
//! entry until every original consumer in that tuple is integrated.

/// `tiny_glade::app_systems::impl$0::add_wall_systems` `0x140493120` —— 未实现。
pub fn add_wall_systems() {
    todo!(
        "未实现: add_wall_systems 0x140493120 —— 注册顺序已解（见模块注释），\
         系统函数名/ResMut 面未逐个锚定"
    );
}

/// `tiny_glade::app_systems::impl$0::add_wall_events` `0x1404936b0` —— 未实现。
pub fn add_wall_events() {
    todo!("未实现: add_wall_events 0x1404936b0 —— 39KB 事件注册表未逐项解出")
}

/// Original B @1402b1cb0,5354B; registered OnExit(Game) by
/// add_wall_systems@14049356f..574. Container resets use their current owners;
/// three actual GPU color buffers are reset in original CPU-reset/upload order.
#[allow(clippy::too_many_arguments)]
pub fn walls_on_glade_unload(
    device: bevy_ecs::prelude::Res<country_core::render::device::GraphicsDeviceResource>,
    ssbos: bevy_ecs::prelude::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,
    mut public: bevy_ecs::prelude::ResMut<
        country_core::resources::walls::public_walls::PublicWalls,
    >,
    mut plasters: bevy_ecs::prelude::ResMut<
        country_core::resources::walls::public_walls::WallPlasters,
    >,
    mut entities: bevy_ecs::prelude::ResMut<
        country_core::resources::walls::public_walls::WallEntities,
    >,
    mut inner: bevy_ecs::prelude::ResMut<country_core::resources::walls::InnerWalls>,
    mut spatial: bevy_ecs::prelude::ResMut<
        country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash,
    >,
    mut wall_colors: bevy_ecs::prelude::ResMut<country_core::resources::color_id::WallColorIds>,
    mut gate_colors: bevy_ecs::prelude::ResMut<country_core::resources::color_id::GateColorIds>,
    mut timber_colors: bevy_ecs::prelude::ResMut<
        country_core::resources::color_id::HalftimberColorIds,
    >,
    mut highlight: bevy_ecs::prelude::ResMut<country_core::resources::walls::WallHighlight>,
    mut platforms: bevy_ecs::prelude::ResMut<
        system_wall_constructor::construct_platform_stairs::PlatformStairsStorage,
    >,
    mut gates: bevy_ecs::prelude::ResMut<country_core::resources::walls::wooden_gates::Gates>,
) {
    use country_core::resources::color_id::*;
    *public = Default::default();
    *entities = Default::default();
    *plasters = Default::default();
    // Original +10 byte only; retained id/entity payload is not zeroed.
    highlight.ty = 0;
    *platforms = Default::default();
    inner.remove_all();
    inner.reset_max_id();
    gates.clear();
    *spatial = Default::default();

    // Constructors allocate exactly0x5000 u32s and memset0 (native
    // 140b19890/140b19900/140b19970). GPU default_value returns Some(0):
    // low32=1 tag, upper32=0 value; it must not be misread as color1.
    *wall_colors = WallColorIds::new();
    reset_gpu_color_table::<WallColorIdSsbo>(&device, &ssbos);
    *gate_colors = GateColorIds::new();
    reset_gpu_color_table::<GateColorIdSsbo>(&device, &ssbos);
    *timber_colors = HalftimberColorIds::new();
    reset_gpu_color_table::<HalftimberColorIdSsbo>(&device, &ssbos);
}

fn reset_gpu_color_table<T: country_core::asset_libraries::ssbo_library::SsboType>(
    device: &country_core::render::device::GraphicsDeviceResource,
    ssbos: &country_core::asset_libraries::ssbo_library::AssetSsboLibrary,
) {
    use country_core::render::ssbo::rhapsody_boundary::GraphicsDevice;
    use country_core::resources::color_id::{ColorId, MAX_NUMBER_OF_WALLS};
    let buffer = ssbos
        .get_buffer::<T>()
        .expect("original wall color SSBO is required on unload");
    let defaults = vec![ColorId::default_value().0; MAX_NUMBER_OF_WALLS];
    let bytes = bytemuck::cast_slice(&defaults);
    assert!(
        buffer.byte_size >= bytes.len() as u64,
        "wall color SSBO is too short"
    );
    GraphicsDevice::update_buffer_bytes(device.device(), buffer, bytes, &[0..bytes.len()], 0)
        .expect("resetting original wall color SSBO on unload");
}

/// Original PostUpdate closure@1403bf630/483B: State2 requests State1.
/// It runs during the Reload frame; the following state transition enters
/// Game and invokes the canonical NextSession consumer.
pub fn advance_reload_state(
    state: bevy_ecs::prelude::Res<
        bevy_state::state::State<country_core::app_loading_state::AppLoadingState>,
    >,
    mut next: bevy_ecs::prelude::ResMut<
        bevy_state::state::NextState<country_core::app_loading_state::AppLoadingState>,
    >,
) {
    if *state.get() == country_core::app_loading_state::AppLoadingState::Reload {
        next.set(country_core::app_loading_state::AppLoadingState::Game);
    }
}
