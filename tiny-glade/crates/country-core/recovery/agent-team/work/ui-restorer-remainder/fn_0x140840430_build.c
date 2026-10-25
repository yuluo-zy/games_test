void __cdecl build(void)


void __cdecl country_core::systems::prefab::impl$0::build(void)

{
  longlong extraout_RAX;
  longlong extraout_RAX_00;
  longlong *extraout_RAX_01;
  longlong in_RDX;
  ulonglong uVar1;
  
  uVar1 = 0xa73802c6af7eaf33;
  bevy_ecs::component::Components::get_resource_id();
  if ((((extraout_RAX == 0) || (*(ulonglong *)(in_RDX + 0x218) <= uVar1)) ||
      (uVar1 = *(ulonglong *)(*(longlong *)(in_RDX + 0x210) + uVar1 * 8), uVar1 == 0)) ||
     (*(longlong *)(*(longlong *)(in_RDX + 0x1e0) + 0x20 + ~uVar1 * 0x60) == 0)) {
    bevy_ecs::event::registry::EventRegistry::
    register_event<country_core::systems::prefab::SpawnPrefabCmd>();
  }
  uVar1 = 0x98f655b4397350da;
  bevy_ecs::component::Components::get_resource_id();
  if (((extraout_RAX_00 == 0) || (*(ulonglong *)(in_RDX + 0x218) <= uVar1)) ||
     ((uVar1 = *(ulonglong *)(*(longlong *)(in_RDX + 0x210) + uVar1 * 8), uVar1 == 0 ||
      (*(longlong *)(*(longlong *)(in_RDX + 0x1e0) + 0x20 + ~uVar1 * 0x60) == 0)))) {
    bevy_ecs::event::registry::EventRegistry::
    register_event<country_core::systems::prefab::DespawnPrefabCmd>();
  }
  _ZN8bevy_ecs8schedule6config19IntoScheduleConfigs21chain_ignore_deferred17hc3ffe9b5147141d6E();
  bevy_ecs::intern::Interner::intern<dyn_<bevy_ecs::schedule::set::SystemSet>_>();
  bevy_ecs::schedule::config::ScheduleConfigs::
  in_set_inner<bevy_ecs::intern::Interned<dyn_<bevy_ecs::schedule::set::SystemSet>_>_>();
  bevy_app::sub_app::SubApp::
  add_systems<tuple_<>,bevy_app::main_schedule::Update,enum2_<bevy_ecs::schedule::config::ScheduleConfigs<alloc::boxed::Box<dyn_<bevy_ecs::system::system::System<assoc_<In,tuple_<>_>,assoc_<Out,enum2_<core::result::Result<tuple_<>,bevy_ecs::error::bevy_error::BevyError>_>_>_>_>,alloc::alloc::Global>_>_>_>
            ();
  core::ops::function::FnOnce::
  call_once<std::hash::random::impl_0::new::KEYS::constant_0::closure_env_1,tuple_<enum2_<core::option::Option<ref_mut_<enum2_<core::option::Option<core::cell::Cell<tuple_<u64,u64>_>_>_>_>_>_>_>_>
            ();
  if (extraout_RAX_01 != (longlong *)0x0) {
    *extraout_RAX_01 = *extraout_RAX_01 + 1;
    bevy_app::app::App::insert_resource<country_core::systems::prefab::PrefabsToDelete>();
    return;
  }
                    // WARNING: Subroutine does not return
  std::thread::local::panic_access_error();
}

