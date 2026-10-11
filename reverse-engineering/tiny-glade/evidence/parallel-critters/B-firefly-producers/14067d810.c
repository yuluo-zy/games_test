
void __cdecl system_critters::sheep::impl$0::build(void)

{
  longlong extraout_RAX;
  longlong extraout_RAX_00;
  longlong extraout_RAX_01;
  longlong in_RDX;
  ulonglong uVar1;
  
  _ZN8bevy_ecs8schedule6config19IntoScheduleConfigs21chain_ignore_deferred17h9f74de232c6c9195E();
  bevy_ecs::schedule::config::impl$3::
  run_if<alloc::boxed::Box<dyn_<bevy_ecs::system::system::System<assoc_<In,tuple_<>_>,assoc_<Out,enum2_<core::result::Result<tuple_<>,bevy_ecs::error::bevy_error::BevyError>_>_>_>_>,alloc::alloc::Global>,tuple_<bevy_ecs::system::function_system::IsFunctionSystem,bool_(*)(bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<bevy_state::state::resources::State<country_core::app_loading_state::AppLoadingState>_>)>,bool_(*)(bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<bevy_state::state::resources::State<country_core::app_loading_state::AppLoadingState>_>)>
            ();
  bevy_app::sub_app::SubApp::
  add_systems<tuple_<>,bevy_app::main_schedule::Update,enum2_<bevy_ecs::schedule::config::ScheduleConfigs<alloc::boxed::Box<dyn_<bevy_ecs::system::system::System<assoc_<In,tuple_<>_>,assoc_<Out,enum2_<core::result::Result<tuple_<>,bevy_ecs::error::bevy_error::BevyError>_>_>_>_>,alloc::alloc::Global>_>_>_>
            ();
  uVar1 = 0xebbff8344555b7ea;
  bevy_ecs::component::Components::get_resource_id();
  if ((((extraout_RAX == 0) || (*(ulonglong *)(in_RDX + 0x218) <= uVar1)) ||
      (uVar1 = *(ulonglong *)(*(longlong *)(in_RDX + 0x210) + uVar1 * 8), uVar1 == 0)) ||
     (*(longlong *)(*(longlong *)(in_RDX + 0x1e0) + 0x20 + ~uVar1 * 0x60) == 0)) {
    bevy_ecs::event::registry::EventRegistry::
    register_event<system_critters::sheep::sheep_pet_animation::PlaySheepPetAnimation>();
  }
  uVar1 = 0x1173151c151d7a3b;
  bevy_ecs::component::Components::get_resource_id();
  if (((extraout_RAX_00 == 0) || (*(ulonglong *)(in_RDX + 0x218) <= uVar1)) ||
     ((uVar1 = *(ulonglong *)(*(longlong *)(in_RDX + 0x210) + uVar1 * 8), uVar1 == 0 ||
      (*(longlong *)(*(longlong *)(in_RDX + 0x1e0) + 0x20 + ~uVar1 * 0x60) == 0)))) {
    bevy_ecs::event::registry::EventRegistry::
    register_event<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>();
  }
  uVar1 = 0x1888c9e7607b4b94;
  bevy_ecs::component::Components::get_resource_id();
  if (((extraout_RAX_01 != 0) && (uVar1 < *(ulonglong *)(in_RDX + 0x218))) &&
     ((uVar1 = *(ulonglong *)(*(longlong *)(in_RDX + 0x210) + uVar1 * 8), uVar1 != 0 &&
      (*(longlong *)(*(longlong *)(in_RDX + 0x1e0) + 0x20 + ~uVar1 * 0x60) != 0)))) {
    return;
  }
  bevy_ecs::event::registry::EventRegistry::
  register_event<country_core::resources::critters::sheep::CmdUnfreezeSheep>();
  return;
}

