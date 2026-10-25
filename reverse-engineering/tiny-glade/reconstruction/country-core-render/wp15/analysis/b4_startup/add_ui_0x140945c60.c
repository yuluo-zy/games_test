// addr=0x140945c60 name=add_ui
// signature=void __cdecl add_ui(void)


void __cdecl country_core::systems::ui::impl$0::add_ui(void)

{
  longlong *extraout_RAX;
  longlong extraout_RAX_00;
  longlong extraout_RAX_01;
  longlong extraout_RAX_02;
  ulonglong uVar1;
  undefined8 local_208;
  undefined8 uStack_200;
  undefined8 local_1f8;
  undefined1 uStack_1f0;
  undefined8 local_1e8;
  undefined4 uStack_1e0;
  undefined1 local_1d8;
  undefined8 uStack_1d0;
  undefined8 local_1c8;
  undefined8 uStack_1c0;
  undefined8 local_1b8;
  undefined8 uStack_1b0;
  undefined8 local_1a8;
  undefined *puStack_1a0;
  undefined8 uStack_198;
  undefined8 local_190;
  undefined8 uStack_188;
  longlong local_180;
  longlong lStack_178;
  undefined8 local_98;
  undefined8 uStack_90;
  undefined8 local_88;
  undefined8 local_78;
  undefined8 uStack_60;
  undefined8 local_58;
  undefined8 uStack_50;
  undefined8 local_48;
  undefined8 uStack_40;
  undefined8 local_38;
  undefined8 uStack_30;
  undefined1 local_21;
  undefined8 local_20;
  
  local_20 = 0xfffffffffffffffe;
  _ZN8bevy_ecs8schedule6config19IntoScheduleConfigs21chain_ignore_deferred17hc4c3f3097884ae7cE();
  local_21 = 6;
  bevy_ecs::intern::Interner::intern<dyn_<bevy_ecs::schedule::set::SystemSet>_>();
  bevy_ecs::schedule::config::ScheduleConfigs::
  in_set_inner<bevy_ecs::intern::Interned<dyn_<bevy_ecs::schedule::set::SystemSet>_>_>();
  local_38 = local_1a8;
  uStack_30 = puStack_1a0;
  local_48 = local_1b8;
  uStack_40 = uStack_1b0;
  local_58 = local_1c8;
  uStack_50 = uStack_1c0;
  uStack_60 = uStack_1d0;
  local_78 = local_1e8;
  local_88 = local_1f8;
  local_98 = local_208;
  uStack_90 = uStack_200;
  bevy_app::sub_app::SubApp::
  add_systems<tuple_<>,bevy_app::main_schedule::Update,enum2_<bevy_ecs::schedule::config::ScheduleConfigs<alloc::boxed::Box<dyn_<bevy_ecs::system::system::System<assoc_<In,tuple_<>_>,assoc_<Out,enum2_<core::result::Result<tuple_<>,bevy_ecs::error::bevy_error::BevyError>_>_>_>_>,alloc::alloc::Global>_>_>_>
            ();
  core::ops::function::FnOnce::
  call_once<std::hash::random::impl_0::new::KEYS::constant_0::closure_env_1,tuple_<enum2_<core::option::Option<ref_mut_<enum2_<core::option::Option<core::cell::Cell<tuple_<u64,u64>_>_>_>_>_>_>_>_>
            ();
  if (extraout_RAX != (longlong *)0x0) {
    local_1a8 = 0;
    local_180 = *extraout_RAX;
    lStack_178 = extraout_RAX[1];
    *extraout_RAX = *extraout_RAX + 1;
    local_190 = 0;
    uStack_188 = 0;
    puStack_1a0 = &anon_03061c64e98a3a4ea7442d952ee9ce0d_23_llvm_2292516692454265093;
    uStack_198 = 0;
    local_208 = 0;
    uStack_200 = 8;
    local_1f8 = 0;
    uStack_1f0 = 0;
    bevy_app::app::App::insert_resource<country_core::systems::ui::resources::ui_state::UiState>();
    local_208 = 0;
    local_1f8 = 0;
    uStack_1e0 = 1000000000;
    local_1d8 = 0;
    bevy_app::app::App::
    insert_resource<country_core::systems::ui::resources::widget_params::UiNavState>();
    uVar1 = 0x3917598efcc5579e;
    bevy_ecs::component::Components::get_resource_id();
    if ((((extraout_RAX_01 == 0) || (*(ulonglong *)(extraout_RAX_00 + 0x218) <= uVar1)) ||
        (uVar1 = *(ulonglong *)(*(longlong *)(extraout_RAX_00 + 0x210) + uVar1 * 8), uVar1 == 0)) ||
       (*(longlong *)(*(longlong *)(extraout_RAX_00 + 0x1e0) + 0x20 + ~uVar1 * 0x60) == 0)) {
      bevy_ecs::event::registry::EventRegistry::
      register_event<enum2_<country_core::systems::ui::resources::widget_params::PlayUiSound>_>();
    }
    uVar1 = 0x2a88611d41c83a99;
    bevy_ecs::component::Components::get_resource_id();
    if (((extraout_RAX_02 == 0) || (*(ulonglong *)(extraout_RAX_00 + 0x218) <= uVar1)) ||
       ((uVar1 = *(ulonglong *)(*(longlong *)(extraout_RAX_00 + 0x210) + uVar1 * 8), uVar1 == 0 ||
        (*(longlong *)(*(longlong *)(extraout_RAX_00 + 0x1e0) + 0x20 + ~uVar1 * 0x60) == 0)))) {
      bevy_ecs::event::registry::EventRegistry::
      register_event<country_core::systems::ui::resources::widget_params::PlayPhotomodeSound>();
    }
    memset(&local_208,0,0x170);
    bevy_app::app::App::
    insert_resource<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>();
    return;
  }
                    // WARNING: Subroutine does not return
  std::thread::local::panic_access_error();
}


