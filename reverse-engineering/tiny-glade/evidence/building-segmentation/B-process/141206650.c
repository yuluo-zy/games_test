
void __cdecl
alloc::vec::spec_from_iter::impl$0::
from_iter<glam::f32::vec2::Vec2,core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::iter::adapters::skip::Skip<core::iter::adapters::take::Take<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>_>_>_>,glam::f32::vec2::Vec2_(*)(glam::f32::vec3::Vec3)>_>
          (void)

{
  undefined4 uVar1;
  undefined4 uVar2;
  longlong lVar3;
  ulonglong uVar4;
  ulonglong extraout_RAX;
  undefined4 *extraout_RAX_00;
  ulonglong extraout_RAX_01;
  undefined4 *puVar5;
  undefined4 *extraout_RAX_02;
  undefined4 *extraout_RAX_03;
  undefined8 *in_RCX;
  longlong *in_RDX;
  int local_c8;
  longlong local_a8;
  longlong lStack_a0;
  int local_94;
  longlong local_78;
  
  if (in_RDX[5] == 0) {
    if (in_RDX[4] == 0) goto LAB_14120684d;
    in_RDX[4] = in_RDX[4] + -1;
    ::utils::loop_range::impl$2::next();
    if (local_c8 != 1) goto LAB_14120684d;
    lVar3 = *in_RDX;
    uVar4 = in_RDX[1];
    ::utils::loop_range::LoopIdx::index();
    if (uVar4 <= extraout_RAX) {
                    /* WARNING: Subroutine does not return */
      core::panicking::panic_bounds_check();
    }
    puVar5 = (undefined4 *)(lVar3 + extraout_RAX * 0xc);
  }
  else {
    in_RDX[5] = 0;
    core::iter::adapters::take::impl$1::nth<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>_>();
    puVar5 = extraout_RAX_03;
  }
  if (puVar5 != (undefined4 *)0x0) {
    uVar1 = *puVar5;
    uVar2 = puVar5[2];
    std::alloc::__default_lib_allocator::__rust_alloc();
    if (extraout_RAX_00 == (undefined4 *)0x0) {
                    /* WARNING: Subroutine does not return */
      raw_vec::handle_error();
    }
    *extraout_RAX_00 = uVar1;
    extraout_RAX_00[1] = uVar2;
    local_78 = 1;
    lVar3 = *in_RDX;
    uVar4 = in_RDX[1];
    local_a8 = in_RDX[4];
    lStack_a0 = in_RDX[5];
    do {
      if (lStack_a0 == 0) {
        if (local_a8 == 0) {
LAB_141206823:
          in_RCX[2] = local_78;
          *in_RCX = 4;
          in_RCX[1] = extraout_RAX_00;
          return;
        }
        local_a8 = local_a8 + -1;
        ::utils::loop_range::impl$2::next();
        if (local_94 == 1) {
          ::utils::loop_range::LoopIdx::index();
          if (uVar4 <= extraout_RAX_01) {
                    /* WARNING: Subroutine does not return */
            core::panicking::panic_bounds_check();
          }
          puVar5 = (undefined4 *)(lVar3 + extraout_RAX_01 * 0xc);
        }
        else {
          puVar5 = (undefined4 *)0x0;
        }
      }
      else {
        lStack_a0 = 0;
        core::iter::adapters::take::impl$1::
        nth<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>_>();
        puVar5 = extraout_RAX_02;
      }
      if (puVar5 == (undefined4 *)0x0) goto LAB_141206823;
      uVar1 = *puVar5;
      uVar2 = puVar5[2];
      if (local_78 == 4) {
        raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
      }
      extraout_RAX_00[local_78 * 2] = uVar1;
      extraout_RAX_00[local_78 * 2 + 1] = uVar2;
      local_78 = local_78 + 1;
    } while( true );
  }
LAB_14120684d:
  *in_RCX = 0;
  in_RCX[1] = 4;
  in_RCX[2] = 0;
  return;
}

