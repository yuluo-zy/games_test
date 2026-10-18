
void __cdecl
bevy_app::sub_app::SubApp::
add_systems<tuple_<>,bevy_state::state::transitions::OnEnter<country_core::app_loading_state::AppLoadingState>,enum2_<bevy_ecs::schedule::config::ScheduleConfigs<alloc::boxed::Box<dyn_<bevy_ecs::system::system::System<assoc_<In,tuple_<>_>,assoc_<Out,enum2_<core::result::Result<tuple_<>,bevy_ecs::error::bevy_error::BevyError>_>_>_>_>,alloc::alloc::Global>_>_>_>
          (void)

{
  longlong extraout_RAX;
  longlong in_RCX;
  longlong lVar1;
  ulonglong uVar2;
  char *local_50;
  undefined8 local_48;
  char **local_40;
  code *local_38;
  undefined1 local_21;
  undefined8 local_20;
  
  local_20 = 0xfffffffffffffffe;
  local_21 = 1;
  uVar2 = 0x8e1bd251962ba784;
  bevy_ecs::component::Components::get_resource_id();
  if (((extraout_RAX != 0) && (uVar2 < *(ulonglong *)(in_RCX + 0x218))) &&
     (uVar2 = *(ulonglong *)(*(longlong *)(in_RCX + 0x210) + uVar2 * 8), uVar2 != 0)) {
    lVar1 = ~uVar2 * 0x60;
    if (*(longlong *)(*(longlong *)(in_RCX + 0x1e0) + 0x20 + lVar1) != 0) {
      *(undefined4 *)(*(longlong *)(in_RCX + 0x1e0) + lVar1 + 0x5c) =
           *(undefined4 *)(in_RCX + 0x5b0);
      local_21 = 0;
      bevy_ecs::schedule::schedule::Schedules::
      add_systems<tuple_<>,bevy_state::state::transitions::OnEnter<country_core::app_loading_state::AppLoadingState>,enum2_<bevy_ecs::schedule::config::ScheduleConfigs<alloc::boxed::Box<dyn_<bevy_ecs::system::system::System<assoc_<In,tuple_<>_>,assoc_<Out,enum2_<core::result::Result<tuple_<>,bevy_ecs::error::bevy_error::BevyError>_>_>_>_>,alloc::alloc::Global>_>_>_>
                ();
      return;
    }
  }
  local_50 = 
  "bevy_ecs::schedule::schedule::SchedulesRequested resource  does not exist in the `World`.\n                Did you forget to add it using `app.insert_resource` / `app.init_resource`?\n                Resources are also implicitly added via `app.add_event`,\n                and can be added by plugins."
  ;
  local_48 = 0x27;
  local_40 = &local_50;
  local_38 = anyhow::context::impl$3::fmt<ref_<str_>,alloc::ffi::c_str::FromVecWithNulError>;
  local_21 = 1;
                    // WARNING: Subroutine does not return
  core::panicking::panic_fmt();
}

