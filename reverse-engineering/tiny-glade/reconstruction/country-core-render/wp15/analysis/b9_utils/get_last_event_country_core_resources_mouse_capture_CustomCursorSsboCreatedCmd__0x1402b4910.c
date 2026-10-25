// addr=0x1402b4910 name=get_last_event<country_core::resources::mouse_capture::CustomCursorSsboCreatedCmd>
// signature=void __cdecl get_last_event<country_core::resources::mouse_capture::CustomCursorSsboCreatedCmd>(void)


void __cdecl
country_core::utils::
get_last_event<country_core::resources::mouse_capture::CustomCursorSsboCreatedCmd>(void)

{
  longlong lVar1;
  longlong extraout_RAX;
  longlong in_RCX;
  longlong lVar2;
  ulonglong uVar3;
  
  uVar3 = 0x6ecb4684a4c9e1f9;
  bevy_ecs::component::Components::get_resource_id();
  if (((extraout_RAX != 0) && (uVar3 < *(ulonglong *)(in_RCX + 0x218))) &&
     (uVar3 = *(ulonglong *)(*(longlong *)(in_RCX + 0x210) + uVar3 * 8), uVar3 != 0)) {
    lVar2 = ~uVar3 * 0x60;
    if (*(longlong *)(*(longlong *)(in_RCX + 0x1e0) + 0x20 + lVar2) != 0) {
      lVar2 = *(longlong *)(in_RCX + 0x1e0) + lVar2;
      lVar1 = *(longlong *)(lVar2 + 0x10);
      *(undefined4 *)(lVar2 + 0x5c) = *(undefined4 *)(in_RCX + 0x5b0);
      *(undefined8 *)(lVar1 + 0x18) = *(undefined8 *)(lVar1 + 0x40);
      *(undefined8 *)(lVar1 + 0x38) = *(undefined8 *)(lVar1 + 0x40);
      *(undefined8 *)(lVar1 + 0x10) = 0;
      *(undefined8 *)(lVar1 + 0x30) = 0;
      core::iter::adapters::chain::impl$1::
      fold<alloc::vec::drain::Drain<bevy_ecs::event::base::EventInstance<country_core::resources::mouse_capture::CustomCursorSsboCreatedCmd>,alloc::alloc::Global>,alloc::vec::drain::Drain<bevy_ecs::event::base::EventInstance<country_core::resources::mouse_capture::CustomCursorSsboCreatedCmd>,alloc::alloc::Global>,enum2_<core::option::Option<country_core::resources::mouse_capture::CustomCursorSsboCreatedCmd>_>,core::iter::adapters::map::map_fold::closure_env_0<bevy_ecs::event::base::EventInstance<country_core::resources::mouse_capture::CustomCursorSsboCreatedCmd>,country_core::resources::mouse_capture::CustomCursorSsboCreatedCmd,enum2_<core::option::Option<country_core::resources::mouse_capture::CustomCursorSsboCreatedCmd>_>,bevy_ecs::event::collections::impl_1::drain::closure_env_0<country_core::resources::mouse_capture::CustomCursorSsboCreatedCmd>,enum2_<core::option::Option<country_core::resources::mouse_capture::CustomCursorSsboCreatedCmd>_>_(*)(enum2_<core::option::Option<country_core::resources::mouse_capture::CustomCursorSsboCreatedCmd>_>,country_core::resources::mouse_capture::CustomCursorSsboCreatedCmd)>_>
                ();
      return;
    }
  }
                    // WARNING: Subroutine does not return
  core::panicking::panic_fmt();
}


