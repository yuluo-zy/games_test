// ADDR 0x14081b920
// SIG void __cdecl closure$0(void)
// ORIG system::closure$0


void __cdecl
system_decorator::decorator_on_wall_height_changed::decorator_on_wall_height_changed::closure_0
          (void)

{
  longlong extraout_RAX;
  longlong extraout_RAX_00;
  longlong lVar1;
  longlong extraout_RAX_01;
  longlong extraout_RAX_02;
  longlong in_RCX;
  float extraout_XMM0_Da;
  undefined8 local_70;
  
  **(undefined4 **)(in_RCX + 0x10) = *(undefined4 *)(in_RCX + 0x1c);
  country_core::resources::walls::decorator_storage::DecoratorStorage::wall_decorators_mut();
  if (extraout_RAX != 0) {
    country_core::resources::walls::decorator_storage::impl$3::next();
    if (local_70 != 0) {
      do {
        minidump_unwind::instruction_seems_valid_by_symbols::async_fn$0::impl$0::get_instruction();
        lVar1 = (ulonglong)*(byte *)(extraout_RAX_00 + 0x22) - 1;
        if (2 < (byte)(*(byte *)(extraout_RAX_00 + 0x22) - 2)) {
          lVar1 = 0;
        }
        if (lVar1 == 0) {
          country_core::resources::walls::decorator_storage::DecoratorMaybeMut::get_mut();
          country_core::resources::walls::decorator::DecoratorInfo::dst_as_wall_mut();
          country_core::resources::walls::public_walls::PublicWalls::get();
          if (extraout_RAX_02 == 0) {
                    // WARNING: Subroutine does not return
            core::option::unwrap_failed();
          }
          country_core::resources::walls::public_wall_state::PublicWallState::
          get_wall_attachment_height();
          *(float *)(extraout_RAX_01 + 0x10) = *(float *)(extraout_RAX_01 + 4) - extraout_XMM0_Da;
        }
        else if (lVar1 != 1) {
                    // WARNING: Subroutine does not return
          core::panicking::panic();
        }
        country_core::resources::walls::decorator_storage::impl$3::next();
      } while (local_70 != 0);
    }
  }
  return;
}


