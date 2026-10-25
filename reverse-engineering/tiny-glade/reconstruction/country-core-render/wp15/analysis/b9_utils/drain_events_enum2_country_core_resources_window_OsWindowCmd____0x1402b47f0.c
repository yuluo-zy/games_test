// addr=0x1402b47f0 name=drain_events<enum2$<country_core::resources::window::OsWindowCmd>_>
// signature=void __cdecl drain_events<enum2$<country_core::resources::window::OsWindowCmd>_>(void)


void __cdecl
country_core::utils::drain_events<enum2_<country_core::resources::window::OsWindowCmd>_>(void)

{
  longlong lVar1;
  longlong extraout_RAX;
  longlong lVar2;
  longlong in_RDX;
  ulonglong uVar3;
  
  uVar3 = 0xdac2c590678dd6c0;
  bevy_ecs::component::Components::get_resource_id();
  if (((extraout_RAX != 0) && (uVar3 < *(ulonglong *)(in_RDX + 0x218))) &&
     (uVar3 = *(ulonglong *)(*(longlong *)(in_RDX + 0x210) + uVar3 * 8), uVar3 != 0)) {
    lVar2 = ~uVar3 * 0x60;
    if (*(longlong *)(*(longlong *)(in_RDX + 0x1e0) + 0x20 + lVar2) != 0) {
      lVar2 = *(longlong *)(in_RDX + 0x1e0) + lVar2;
      lVar1 = *(longlong *)(lVar2 + 0x10);
      *(undefined4 *)(lVar2 + 0x5c) = *(undefined4 *)(in_RDX + 0x5b0);
      *(undefined8 *)(lVar1 + 0x18) = *(undefined8 *)(lVar1 + 0x40);
      *(undefined8 *)(lVar1 + 0x38) = *(undefined8 *)(lVar1 + 0x40);
      *(undefined8 *)(lVar1 + 0x10) = 0;
      *(undefined8 *)(lVar1 + 0x30) = 0;
      alloc::vec::spec_from_iter::impl$0::
      from_iter<enum2_<country_core::resources::window::OsWindowCmd>,core::iter::adapters::map::Map<core::iter::adapters::chain::Chain<alloc::vec::drain::Drain<bevy_ecs::event::base::EventInstance<enum2_<country_core::resources::window::OsWindowCmd>_>,alloc::alloc::Global>,alloc::vec::drain::Drain<bevy_ecs::event::base::EventInstance<enum2_<country_core::resources::window::OsWindowCmd>_>,alloc::alloc::Global>_>,bevy_ecs::event::collections::impl_1::drain::closure_env_0<enum2_<country_core::resources::window::OsWindowCmd>_>_>_>
                ();
      return;
    }
  }
                    // WARNING: Subroutine does not return
  core::option::unwrap_failed();
}


