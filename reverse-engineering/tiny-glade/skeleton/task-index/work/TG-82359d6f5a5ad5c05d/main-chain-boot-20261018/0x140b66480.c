
void __cdecl
bevy_app::sub_app::SubApp::
add_systems<tuple_<>,bevy_state::state::transitions::OnExit<country_core::app_loading_state::AppLoadingState>,enum2_<bevy_ecs::schedule::config::ScheduleConfigs<alloc::boxed::Box<dyn_<bevy_ecs::system::system::System<assoc_<In,tuple_<>_>,assoc_<Out,enum2_<core::result::Result<tuple_<>,bevy_ecs::error::bevy_error::BevyError>_>_>_>_>,alloc::alloc::Global>_>_>_>
          (void)

{
  longlong extraout_RAX;
  longlong in_RCX;
  longlong lVar1;
  ulonglong uVar2;
  undefined8 *in_R8;
  char *local_c8;
  undefined8 uStack_c0;
  undefined8 local_b8;
  undefined8 uStack_b0;
  undefined8 local_a8;
  undefined8 uStack_a0;
  undefined8 local_98;
  undefined8 uStack_90;
  undefined8 local_88;
  undefined8 uStack_80;
  undefined8 local_78;
  undefined8 uStack_70;
  undefined8 local_68;
  undefined8 uStack_60;
  char **local_50;
  code *local_48;
  undefined8 *local_30;
  undefined1 local_22;
  undefined1 local_21;
  undefined8 local_20;
  
  local_20 = 0xfffffffffffffffe;
  local_21 = 1;
  uVar2 = 0x8e1bd251962ba784;
  local_30 = in_R8;
  bevy_ecs::component::Components::get_resource_id();
  if (((extraout_RAX != 0) && (uVar2 < *(ulonglong *)(in_RCX + 0x218))) &&
     (uVar2 = *(ulonglong *)(*(longlong *)(in_RCX + 0x210) + uVar2 * 8), uVar2 != 0)) {
    lVar1 = ~uVar2 * 0x60;
    if (*(longlong *)(*(longlong *)(in_RCX + 0x1e0) + 0x20 + lVar1) != 0) {
      *(undefined4 *)(*(longlong *)(in_RCX + 0x1e0) + lVar1 + 0x5c) =
           *(undefined4 *)(in_RCX + 0x5b0);
      local_68 = local_30[0xc];
      uStack_60 = local_30[0xd];
      local_78 = local_30[10];
      uStack_70 = local_30[0xb];
      local_88 = local_30[8];
      uStack_80 = local_30[9];
      local_c8 = (char *)*local_30;
      uStack_c0 = local_30[1];
      local_b8 = local_30[2];
      uStack_b0 = local_30[3];
      local_a8 = local_30[4];
      uStack_a0 = local_30[5];
      local_98 = local_30[6];
      uStack_90 = local_30[7];
      local_22 = 1;
      bevy_ecs::schedule::schedule::Schedules::
      entry<bevy_state::state::transitions::OnExit<country_core::app_loading_state::AppLoadingState>_>
                ();
      local_22 = 0;
      bevy_ecs::schedule::schedule::ScheduleGraph::
      process_configs<alloc::boxed::Box<dyn_<bevy_ecs::system::system::System<assoc_<In,tuple_<>_>,assoc_<Out,enum2_<core::result::Result<tuple_<>,bevy_ecs::error::bevy_error::BevyError>_>_>_>_>,alloc::alloc::Global>_>
                ();
      if (local_50 != (char **)0x0) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
      return;
    }
  }
  local_c8 = 
  "bevy_ecs::schedule::schedule::SchedulesRequested resource  does not exist in the `World`.\n                Did you forget to add it using `app.insert_resource` / `app.init_resource`?\n                Resources are also implicitly added via `app.add_event`,\n                and can be added by plugins."
  ;
  uStack_c0 = 0x27;
  local_50 = &local_c8;
  local_48 = anyhow::context::impl$3::fmt<ref_<str_>,alloc::ffi::c_str::FromVecWithNulError>;
  local_21 = 1;
                    // WARNING: Subroutine does not return
  core::panicking::panic_fmt();
}

