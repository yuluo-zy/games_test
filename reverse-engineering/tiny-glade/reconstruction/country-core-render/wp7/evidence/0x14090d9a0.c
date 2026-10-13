
void __cdecl country_core::systems::render_loop::rg_buffer_copy_from_slice<u32>(void)

{
  char cVar1;
  char *pcVar2;
  longlong extraout_RAX;
  longlong lVar3;
  longlong in_RDX;
  ulonglong uVar4;
  ulonglong _Size;
  ulonglong *in_R8;
  void *in_R9;
  longlong in_stack_00000028;
  void *local_108;
  ulonglong uStack_100;
  
  _Size = in_stack_00000028 << 2;
  if (*in_R8 < _Size) {
                    // WARNING: Subroutine does not return
    core::panicking::panic();
  }
  uVar4 = 0x47c94cb78283a72e;
  bevy_ecs::component::Components::get_resource_id();
  if (((extraout_RAX != 0) && (uVar4 < *(ulonglong *)(in_RDX + 0x218))) &&
     (uVar4 = *(ulonglong *)(*(longlong *)(in_RDX + 0x210) + uVar4 * 8), uVar4 != 0)) {
    lVar3 = ~uVar4 * 0x60;
    if (*(longlong *)(*(longlong *)(in_RDX + 0x1e0) + 0x20 + lVar3) != 0) {
      pcVar2 = *(char **)(*(longlong *)(in_RDX + 0x1e0) + lVar3 + 0x10);
      rhapsody::rg::resource_info::impl$13::from();
      bevy_app::plugin::Plugin::finish<country_core::systems::main_camera::MainCameraPlugin>();
      rhapsody::rg::pass_builder::PassBuilder::
      add_resource_access<country_core::render::pass_builder::pass_resources::PassResources>();
      LOCK();
      cVar1 = *pcVar2;
      if (cVar1 == '\0') {
        *pcVar2 = '\x01';
      }
      UNLOCK();
      if (cVar1 != '\0') {
        parking_lot::raw_mutex::RawMutex::lock_slow();
      }
      parking_lot_core::parking_lot::deadlock_impl::acquire_resource();
      rhapsody::raw::dynamic_constants::DynamicConstants::allocate();
      if (uStack_100 != _Size) {
                    // WARNING: Subroutine does not return
        core::slice::impl$0::copy_from_slice::len_mismatch_fail();
      }
      memcpy(local_108,in_R9,_Size);
      parking_lot_core::parking_lot::deadlock_impl::release_resource();
      LOCK();
      cVar1 = *pcVar2;
      if (cVar1 == '\x01') {
        *pcVar2 = '\0';
      }
      UNLOCK();
      if (cVar1 != '\x01') {
        parking_lot::raw_mutex::RawMutex::unlock_slow();
      }
      rhapsody::rg::pass_builder::PassBuilder::
      render<country_core::render::pass_builder::pass_resources::PassResources,country_core::systems::render_loop::rg_buffer_copy_from_slice::closure_env_0<u32>_>
                ();
      return;
    }
  }
                    // WARNING: Subroutine does not return
  core::option::unwrap_failed();
}

