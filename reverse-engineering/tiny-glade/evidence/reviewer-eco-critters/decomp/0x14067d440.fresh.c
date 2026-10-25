// build 0x14067d440 sig=void __cdecl build(void)

void __cdecl system_critters::impl$0::build(void)

{
  longlong extraout_RAX;
  undefined8 *extraout_RAX_00;
  longlong extraout_RAX_01;
  longlong extraout_RAX_02;
  longlong extraout_RAX_03;
  longlong lVar1;
  longlong in_RDX;
  ulonglong uVar2;
  
  _ZN8bevy_ecs8schedule6config19IntoScheduleConfigs21chain_ignore_deferred17h5c87f57cc8014ddaE();
  _ZN8bevy_ecs8schedule6config19IntoScheduleConfigs21chain_ignore_deferred17h1e4e3f379758c6a2E();
  _ZN8bevy_ecs8schedule6config19IntoScheduleConfigs6in_set17h92026d8b900d1f8fE();
  bevy_app::sub_app::SubApp::
  add_systems<tuple_<>,bevy_app::main_schedule::Update,enum2_<bevy_ecs::schedule::config::ScheduleConfigs<alloc::boxed::Box<dyn_<bevy_ecs::system::system::System<assoc_<In,tuple_<>_>,assoc_<Out,enum2_<core::result::Result<tuple_<>,bevy_ecs::error::bevy_error::BevyError>_>_>_>_>,alloc::alloc::Global>_>_>_>
            ();
  bevy_ecs::schedule::config::IntoScheduleConfigs::
  in_set<tuple_<void_(*)(bevy_ecs::system::query::Query<tuple_<ref_<utils::components::transform::Transform>,ref_<country_core::resources::critters::Butterfly>_>,tuple_<>_>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>),void_(*)(bevy_ecs::system::query::Query<tuple_<ref_<utils::components::transform::Transform>,ref_<country_core::resources::critters::Firefly>_>,tuple_<>_>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)>,alloc::boxed::Box<dyn_<bevy_ecs::system::system::System<assoc_<In,tuple_<>_>,assoc_<Out,enum2_<core::result::Result<tuple_<>,bevy_ecs::error::bevy_error::BevyError>_>_>_>_>,alloc::alloc::Global>,tuple_<bevy_ecs::schedule::config::ScheduleConfigTupleMarker,tuple_<bevy_ecs::schedule::config::Infallible,tuple_<bevy_ecs::system::function_system::IsFunctionSystem,void_(*)(bevy_ecs::system::query::Query<tuple_<ref_<utils::components::transform::Transform>,ref_<country_core::resources::critters::Butterfly>_>,tuple_<>_>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)>_>,tuple_<bevy_ecs::schedule::config::Infallible,tuple_<bevy_ecs::system::function_system::IsFunctionSystem,void_(*)(bevy_ecs::system::query::Query<tuple_<ref_<utils::components::transform::Transform>,ref_<country_core::resources::critters::Firefly>_>,tuple_<>_>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)>_>_>,country_core::systems::GameSystemSet>
            ();
  bevy_app::sub_app::SubApp::
  add_systems<tuple_<>,bevy_app::main_schedule::Update,enum2_<bevy_ecs::schedule::config::ScheduleConfigs<alloc::boxed::Box<dyn_<bevy_ecs::system::system::System<assoc_<In,tuple_<>_>,assoc_<Out,enum2_<core::result::Result<tuple_<>,bevy_ecs::error::bevy_error::BevyError>_>_>_>_>,alloc::alloc::Global>_>_>_>
            ();
  uVar2 = 0x8e1bd251962ba784;
  bevy_ecs::component::Components::get_resource_id();
  if (((extraout_RAX != 0) && (uVar2 < *(ulonglong *)(in_RDX + 0x218))) &&
     (uVar2 = *(ulonglong *)(*(longlong *)(in_RDX + 0x210) + uVar2 * 8), uVar2 != 0)) {
    lVar1 = ~uVar2 * 0x60;
    if (*(longlong *)(*(longlong *)(in_RDX + 0x1e0) + 0x20 + lVar1) != 0) {
      *(undefined4 *)(*(longlong *)(in_RDX + 0x1e0) + lVar1 + 0x5c) =
           *(undefined4 *)(in_RDX + 0x5b0);
      bevy_ecs::schedule::schedule::Schedules::
      add_systems<tuple_<bevy_ecs::schedule::config::ScheduleConfigTupleMarker,tuple_<bevy_ecs::schedule::config::Infallible,tuple_<bevy_ecs::system::function_system::IsFunctionSystem,void_(*)(bevy_ecs::change_detection::ResMut<system_critters::wren_boids::WrenBoids>)>_>,tuple_<bevy_ecs::schedule::config::Infallible,tuple_<bevy_ecs::system::function_system::IsFunctionSystem,void_(*)(bevy_ecs::change_detection::ResMut<system_critters::fish::FishBoids>,bevy_ecs::change_detection::ResMut<system_critters::fish::FishSpawnLocations>)>_>_>,bevy_state::state::transitions::OnExit<country_core::app_loading_state::AppLoadingState>,tuple_<void_(*)(bevy_ecs::change_detection::ResMut<system_critters::wren_boids::WrenBoids>),void_(*)(bevy_ecs::change_detection::ResMut<system_critters::fish::FishBoids>,bevy_ecs::change_detection::ResMut<system_critters::fish::FishSpawnLocations>)>_>
                ();
      bevy_app::app::App::insert_resource<country_core::resources::critters::CritterRng>();
      bevy_app::app::App::insert_resource<system_critters::wren_boids::WrenBoids>();
      country_core::resources::critters::impl$0::default();
      bevy_app::app::App::insert_resource<country_core::resources::critters::CritterControl>();
      bevy_app::app::App::insert_resource<system_critters::duck::DuckTrajectoryPlanner>();
      spawn_ducks::DuckSpawnLocations::new();
      bevy_app::app::App::insert_resource<system_critters::spawn_ducks::DuckSpawnLocations>();
      std::alloc::__default_lib_allocator::__rust_alloc();
      if (extraout_RAX_00 == (undefined8 *)0x0) {
                    // WARNING: Subroutine does not return
        alloc::alloc::handle_alloc_error();
      }
      *extraout_RAX_00 = 1;
      extraout_RAX_00[1] = 1;
      *(undefined1 *)(extraout_RAX_00 + 2) = 0;
      extraout_RAX_00[3] = 0;
      extraout_RAX_00[4] = 8;
      extraout_RAX_00[5] = 0;
      bevy_app::app::App::insert_resource<country_core::resources::critters::BirdSpotManager>();
      bevy_app::app::App::insert_resource<system_critters::fish::FishBoids>();
      fish::FishSpawnLocations::new();
      bevy_app::app::App::insert_resource<system_critters::fish::FishSpawnLocations>();
      uVar2 = 0xac0a08834819dfe0;
      bevy_ecs::component::Components::get_resource_id();
      if (((extraout_RAX_02 == 0) || (*(ulonglong *)(extraout_RAX_01 + 0x218) <= uVar2)) ||
         ((uVar2 = *(ulonglong *)(*(longlong *)(extraout_RAX_01 + 0x210) + uVar2 * 8), uVar2 == 0 ||
          (*(longlong *)(*(longlong *)(extraout_RAX_01 + 0x1e0) + 0x20 + ~uVar2 * 0x60) == 0)))) {
        bevy_ecs::event::registry::EventRegistry::
        register_event<system_critters::spawn_ducks::DespawnDuckCmd>();
      }
      uVar2 = 0x4564a93581af4abf;
      bevy_ecs::component::Components::get_resource_id();
      if (((extraout_RAX_03 != 0) && (uVar2 < *(ulonglong *)(extraout_RAX_01 + 0x218))) &&
         ((uVar2 = *(ulonglong *)(*(longlong *)(extraout_RAX_01 + 0x210) + uVar2 * 8), uVar2 != 0 &&
          (*(longlong *)(*(longlong *)(extraout_RAX_01 + 0x1e0) + 0x20 + ~uVar2 * 0x60) != 0)))) {
        return;
      }
      bevy_ecs::event::registry::EventRegistry::
      register_event<system_critters::duck_splash_particle::SpawnSplashParticle>();
      return;
    }
  }
                    // WARNING: Subroutine does not return
  core::panicking::panic_fmt();
}

