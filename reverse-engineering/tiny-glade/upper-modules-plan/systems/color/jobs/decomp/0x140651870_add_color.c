// name=add_color
// sig=void __cdecl add_color(void)
// size=1916

void __cdecl system_color::impl$0::add_color(void)

{
  longlong extraout_RAX;
  longlong extraout_RAX_00;
  longlong extraout_RAX_01;
  longlong extraout_RAX_02;
  longlong extraout_RAX_03;
  longlong extraout_RAX_04;
  longlong extraout_RAX_05;
  longlong extraout_RAX_06;
  longlong extraout_RAX_07;
  longlong extraout_RAX_08;
  longlong extraout_RAX_09;
  longlong extraout_RAX_10;
  longlong extraout_RAX_11;
  longlong extraout_RAX_12;
  longlong extraout_RAX_13;
  longlong extraout_RAX_14;
  longlong in_RCX;
  longlong lVar1;
  ulonglong uVar2;
  
  uVar2 = 0x115d28f9163e5588;
  bevy_ecs::component::Components::get_resource_id();
  if ((((extraout_RAX == 0) || (*(ulonglong *)(in_RCX + 0x218) <= uVar2)) ||
      (uVar2 = *(ulonglong *)(*(longlong *)(in_RCX + 0x210) + uVar2 * 8), uVar2 == 0)) ||
     (*(longlong *)(*(longlong *)(in_RCX + 0x1e0) + 0x20 + ~uVar2 * 0x60) == 0)) {
    bevy_ecs::event::registry::EventRegistry::
    register_event<system_color::show_color_menu::ShowColorMenuCmd>();
  }
  uVar2 = 0xaf72b2d5d1f5be32;
  bevy_ecs::component::Components::get_resource_id();
  if (((extraout_RAX_00 == 0) || (*(ulonglong *)(in_RCX + 0x218) <= uVar2)) ||
     ((uVar2 = *(ulonglong *)(*(longlong *)(in_RCX + 0x210) + uVar2 * 8), uVar2 == 0 ||
      (*(longlong *)(*(longlong *)(in_RCX + 0x1e0) + 0x20 + ~uVar2 * 0x60) == 0)))) {
    bevy_ecs::event::registry::EventRegistry::
    register_event<system_color::show_color_menu::HideColorMenuCmd>();
  }
  uVar2 = 0x743c1ebedfb0d737;
  bevy_ecs::component::Components::get_resource_id();
  if (((extraout_RAX_01 == 0) || (*(ulonglong *)(in_RCX + 0x218) <= uVar2)) ||
     ((uVar2 = *(ulonglong *)(*(longlong *)(in_RCX + 0x210) + uVar2 * 8), uVar2 == 0 ||
      (*(longlong *)(*(longlong *)(in_RCX + 0x1e0) + 0x20 + ~uVar2 * 0x60) == 0)))) {
    bevy_ecs::event::registry::EventRegistry::
    register_event<country_core::resources::color_id::SetWallColorIdCmd>();
  }
  uVar2 = 0x9b20189b590a96e6;
  bevy_ecs::component::Components::get_resource_id();
  if ((((extraout_RAX_02 == 0) || (*(ulonglong *)(in_RCX + 0x218) <= uVar2)) ||
      (uVar2 = *(ulonglong *)(*(longlong *)(in_RCX + 0x210) + uVar2 * 8), uVar2 == 0)) ||
     (*(longlong *)(*(longlong *)(in_RCX + 0x1e0) + 0x20 + ~uVar2 * 0x60) == 0)) {
    bevy_ecs::event::registry::EventRegistry::
    register_event<system_color::set_roof_customization::SetRoofColorIdCmd>();
  }
  uVar2 = 0x2ffe421984b56322;
  bevy_ecs::component::Components::get_resource_id();
  if (((extraout_RAX_03 == 0) || (*(ulonglong *)(in_RCX + 0x218) <= uVar2)) ||
     ((uVar2 = *(ulonglong *)(*(longlong *)(in_RCX + 0x210) + uVar2 * 8), uVar2 == 0 ||
      (*(longlong *)(*(longlong *)(in_RCX + 0x1e0) + 0x20 + ~uVar2 * 0x60) == 0)))) {
    bevy_ecs::event::registry::EventRegistry::
    register_event<system_color::set_decorator_color_id::SetDecoratorColorIdCmd>();
  }
  uVar2 = 0x2938182f3f2c5857;
  bevy_ecs::component::Components::get_resource_id();
  if (((extraout_RAX_04 == 0) || (*(ulonglong *)(in_RCX + 0x218) <= uVar2)) ||
     ((uVar2 = *(ulonglong *)(*(longlong *)(in_RCX + 0x210) + uVar2 * 8), uVar2 == 0 ||
      (*(longlong *)(*(longlong *)(in_RCX + 0x1e0) + 0x20 + ~uVar2 * 0x60) == 0)))) {
    bevy_ecs::event::registry::EventRegistry::
    register_event<system_color::set_decorator_color_id::SetDecoratorBrightnessCmd>();
  }
  uVar2 = 0xe4cd1a9ede960cf0;
  bevy_ecs::component::Components::get_resource_id();
  if ((((extraout_RAX_05 == 0) || (*(ulonglong *)(in_RCX + 0x218) <= uVar2)) ||
      (uVar2 = *(ulonglong *)(*(longlong *)(in_RCX + 0x210) + uVar2 * 8), uVar2 == 0)) ||
     (*(longlong *)(*(longlong *)(in_RCX + 0x1e0) + 0x20 + ~uVar2 * 0x60) == 0)) {
    bevy_ecs::event::registry::EventRegistry::
    register_event<country_core::resources::color_id::SetGateColorIdCmd>();
  }
  uVar2 = 0x85c09f82c338e9a8;
  bevy_ecs::component::Components::get_resource_id();
  if (((extraout_RAX_06 == 0) || (*(ulonglong *)(in_RCX + 0x218) <= uVar2)) ||
     ((uVar2 = *(ulonglong *)(*(longlong *)(in_RCX + 0x210) + uVar2 * 8), uVar2 == 0 ||
      (*(longlong *)(*(longlong *)(in_RCX + 0x1e0) + 0x20 + ~uVar2 * 0x60) == 0)))) {
    bevy_ecs::event::registry::EventRegistry::
    register_event<country_core::resources::color_id::SetHalftimberColorIdCmd>();
  }
  uVar2 = 0xa49f066a12adf5b5;
  bevy_ecs::component::Components::get_resource_id();
  if (((extraout_RAX_07 == 0) || (*(ulonglong *)(in_RCX + 0x218) <= uVar2)) ||
     ((uVar2 = *(ulonglong *)(*(longlong *)(in_RCX + 0x210) + uVar2 * 8), uVar2 == 0 ||
      (*(longlong *)(*(longlong *)(in_RCX + 0x1e0) + 0x20 + ~uVar2 * 0x60) == 0)))) {
    bevy_ecs::event::registry::EventRegistry::
    register_event<country_core::resources::walls::decorator::SetFlagConnectionCmd>();
  }
  uVar2 = 0x6172b00179304b93;
  bevy_ecs::component::Components::get_resource_id();
  if ((((extraout_RAX_08 == 0) || (*(ulonglong *)(in_RCX + 0x218) <= uVar2)) ||
      (uVar2 = *(ulonglong *)(*(longlong *)(in_RCX + 0x210) + uVar2 * 8), uVar2 == 0)) ||
     (*(longlong *)(*(longlong *)(in_RCX + 0x1e0) + 0x20 + ~uVar2 * 0x60) == 0)) {
    bevy_ecs::event::registry::EventRegistry::
    register_event<system_color::set_window_light::SetWindowLightCmd>();
  }
  uVar2 = 0xaf3ec279349138db;
  bevy_ecs::component::Components::get_resource_id();
  if (((extraout_RAX_09 == 0) || (*(ulonglong *)(in_RCX + 0x218) <= uVar2)) ||
     ((uVar2 = *(ulonglong *)(*(longlong *)(in_RCX + 0x210) + uVar2 * 8), uVar2 == 0 ||
      (*(longlong *)(*(longlong *)(in_RCX + 0x1e0) + 0x20 + ~uVar2 * 0x60) == 0)))) {
    bevy_ecs::event::registry::EventRegistry::
    register_event<country_core::resources::walls::decorator::SetChimneySmokeCmd>();
  }
  uVar2 = 0x549bffb3f8662b2a;
  bevy_ecs::component::Components::get_resource_id();
  if (((extraout_RAX_10 == 0) || (*(ulonglong *)(in_RCX + 0x218) <= uVar2)) ||
     ((uVar2 = *(ulonglong *)(*(longlong *)(in_RCX + 0x210) + uVar2 * 8), uVar2 == 0 ||
      (*(longlong *)(*(longlong *)(in_RCX + 0x1e0) + 0x20 + ~uVar2 * 0x60) == 0)))) {
    bevy_ecs::event::registry::EventRegistry::
    register_event<system_color::set_roof_customization::SetFlatRoofStyleCmd>();
  }
  uVar2 = 0x9c864360197d8fc2;
  bevy_ecs::component::Components::get_resource_id();
  if ((((extraout_RAX_11 == 0) || (*(ulonglong *)(in_RCX + 0x218) <= uVar2)) ||
      (uVar2 = *(ulonglong *)(*(longlong *)(in_RCX + 0x210) + uVar2 * 8), uVar2 == 0)) ||
     (*(longlong *)(*(longlong *)(in_RCX + 0x1e0) + 0x20 + ~uVar2 * 0x60) == 0)) {
    bevy_ecs::event::registry::EventRegistry::
    register_event<system_color::set_decorator_color_id::SetLadderColorIdCmd>();
  }
  uVar2 = 0x583234cb55f86187;
  bevy_ecs::component::Components::get_resource_id();
  if (((extraout_RAX_12 == 0) || (*(ulonglong *)(in_RCX + 0x218) <= uVar2)) ||
     ((uVar2 = *(ulonglong *)(*(longlong *)(in_RCX + 0x210) + uVar2 * 8), uVar2 == 0 ||
      (*(longlong *)(*(longlong *)(in_RCX + 0x1e0) + 0x20 + ~uVar2 * 0x60) == 0)))) {
    bevy_ecs::event::registry::EventRegistry::
    register_event<system_color::set_decorator_color_id::SetStairRailingCmd>();
  }
  uVar2 = 0xa0edb8fa8a01ed67;
  bevy_ecs::component::Components::get_resource_id();
  if (((extraout_RAX_13 == 0) || (*(ulonglong *)(in_RCX + 0x218) <= uVar2)) ||
     ((uVar2 = *(ulonglong *)(*(longlong *)(in_RCX + 0x210) + uVar2 * 8), uVar2 == 0 ||
      (*(longlong *)(*(longlong *)(in_RCX + 0x1e0) + 0x20 + ~uVar2 * 0x60) == 0)))) {
    bevy_ecs::event::registry::EventRegistry::
    register_event<system_color::set_decorator_color_id::SetMagicColorIdCmd>();
  }
  uVar2 = 0x8e1bd251962ba784;
  bevy_ecs::component::Components::get_resource_id();
  if (((extraout_RAX_14 != 0) && (uVar2 < *(ulonglong *)(in_RCX + 0x218))) &&
     (uVar2 = *(ulonglong *)(*(longlong *)(in_RCX + 0x210) + uVar2 * 8), uVar2 != 0)) {
    lVar1 = ~uVar2 * 0x60;
    if (*(longlong *)(*(longlong *)(in_RCX + 0x1e0) + 0x20 + lVar1) != 0) {
      *(undefined4 *)(*(longlong *)(in_RCX + 0x1e0) + lVar1 + 0x5c) =
           *(undefined4 *)(in_RCX + 0x5b0);
      bevy_ecs::schedule::schedule::Schedules::
      add_systems<tuple_<bevy_ecs::schedule::config::Infallible,tuple_<bevy_ecs::system::function_system::IsFunctionSystem,void_(*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::system::commands::Commands)>_>,bevy_app::main_schedule::Startup,void_(*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::system::commands::Commands)>
                ();
      _ZN8bevy_ecs8schedule6config19IntoScheduleConfigs6in_set17h0129bcd9b9d486ffE();
      _ZN8bevy_ecs8schedule6config19IntoScheduleConfigs21chain_ignore_deferred17h9062cff01c0ba91eE()
      ;
      bevy_ecs::intern::Interner::intern<dyn_<bevy_ecs::schedule::set::SystemSet>_>();
      bevy_ecs::schedule::config::ScheduleConfigs::
      in_set_inner<alloc::boxed::Box<dyn_<bevy_ecs::system::system::System<assoc_<In,tuple_<>_>,assoc_<Out,enum2_<core::result::Result<tuple_<>,bevy_ecs::error::bevy_error::BevyError>_>_>_>_>,alloc::alloc::Global>_>
                ();
      bevy_app::sub_app::SubApp::
      add_systems<tuple_<bevy_ecs::schedule::config::ScheduleConfigTupleMarker,tuple_<>,tuple_<>_>,bevy_app::main_schedule::Update,tuple_<enum2_<bevy_ecs::schedule::config::ScheduleConfigs<alloc::boxed::Box<dyn_<bevy_ecs::system::system::System<assoc_<In,tuple_<>_>,assoc_<Out,enum2_<core::result::Result<tuple_<>,bevy_ecs::error::bevy_error::BevyError>_>_>_>_>,alloc::alloc::Global>_>_>,enum2_<bevy_ecs::schedule::config::ScheduleConfigs<alloc::boxed::Box<dyn_<bevy_ecs::system::system::System<assoc_<In,tuple_<>_>,assoc_<Out,enum2_<core::result::Result<tuple_<>,bevy_ecs::error::bevy_error::BevyError>_>_>_>_>,alloc::alloc::Global>_>_>_>_>
                ();
      return;
    }
  }
                    // WARNING: Subroutine does not return
  core::panicking::panic_fmt();
}

