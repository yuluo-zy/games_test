
void __cdecl
alloc::vec::spec_from_iter::impl$0::
from_iter<f32,core::iter::adapters::map::Map<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,system_wall_constructor::construct_platform_stairs::process_loop_range_into_wallpath_segments::closure_env_0>_>
          (void)

{
  ulonglong uVar1;
  longlong lVar2;
  longlong lVar3;
  float fVar4;
  undefined4 uVar5;
  float fVar6;
  uint extraout_EAX;
  ulonglong extraout_RAX;
  undefined4 *extraout_RAX_00;
  ulonglong extraout_RAX_01;
  undefined8 *in_RCX;
  longlong *in_RDX;
  longlong lVar7;
  longlong lVar8;
  float extraout_XMM0_Da;
  float extraout_XMM0_Da_00;
  undefined4 uVar9;
  undefined4 uVar10;
  int local_108;
  longlong local_c8;
  ulonglong local_c0;
  
  ::utils::loop_range::impl$2::next();
  if (local_108 == 1) {
    uVar1 = in_RDX[1];
    ::utils::loop_range::LoopIdx::index();
    if (uVar1 <= extraout_RAX) {
                    /* WARNING: Subroutine does not return */
      core::panicking::panic_bounds_check();
    }
    lVar2 = in_RDX[5];
    ::utils::wall_space::WallSpaceCoord::from_world_space();
    fVar6 = __real_3e4ccccd;
    uVar5 = __real_3f800000;
    fVar4 = __real_3f000000;
    lVar3 = *(longlong *)(lVar2 + 0x10);
    uVar9 = uVar5;
    if (lVar3 != 0) {
      lVar7 = *(longlong *)(lVar2 + 8);
      lVar2 = lVar7 + lVar3 * 0xc;
      do {
        country_core::resources::walls::public_wall_state::PublicWallState::wall_space();
        ::utils::wall_space::impl$1::distance();
        uVar9 = __real_bf800000;
        if (extraout_XMM0_Da < *(float *)(lVar7 + 8) * fVar4 + fVar6) break;
        lVar7 = lVar7 + 0xc;
        uVar9 = uVar5;
      } while (lVar7 != lVar2);
    }
    std::alloc::__default_lib_allocator::__rust_alloc();
    if (extraout_RAX_00 == (undefined4 *)0x0) {
                    /* WARNING: Subroutine does not return */
      raw_vec::handle_error();
    }
    *extraout_RAX_00 = uVar9;
    fVar6 = __real_3e4ccccd;
    uVar9 = __real_bf800000;
    uVar5 = __real_3f800000;
    fVar4 = __real_3f000000;
    local_c8 = 1;
    lVar2 = *in_RDX;
    uVar1 = in_RDX[1];
    lVar3 = in_RDX[5];
    while( true ) {
      ::utils::loop_range::impl$2::next();
      if ((int)local_c0 != 1) break;
      ::utils::loop_range::LoopIdx::index();
      if (uVar1 <= extraout_RAX_01) {
                    /* WARNING: Subroutine does not return */
        core::panicking::panic_bounds_check();
      }
      local_c0 = *(ulonglong *)(lVar2 + extraout_RAX_01 * 0xc);
      ::utils::wall_space::WallSpaceCoord::from_world_space();
      lVar7 = *(longlong *)(lVar3 + 0x10);
      if (lVar7 != 0) {
        lVar8 = *(longlong *)(lVar3 + 8);
        lVar7 = lVar8 + lVar7 * 0xc;
        do {
          country_core::resources::walls::public_wall_state::PublicWallState::wall_space();
          local_c0 = (ulonglong)extraout_EAX;
          ::utils::wall_space::impl$1::distance();
          if (extraout_XMM0_Da_00 < *(float *)(lVar8 + 8) * fVar4 + fVar6) {
            uVar10 = uVar9;
            if (local_c8 != 4) goto LAB_141206244;
            goto LAB_141206240;
          }
          lVar8 = lVar8 + 0xc;
        } while (lVar8 != lVar7);
      }
      uVar10 = uVar5;
      if (local_c8 == 4) {
LAB_141206240:
        raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
      }
LAB_141206244:
      extraout_RAX_00[local_c8] = uVar10;
      local_c8 = local_c8 + 1;
    }
    in_RCX[2] = local_c8;
    *in_RCX = 4;
    in_RCX[1] = extraout_RAX_00;
  }
  else {
    *in_RCX = 0;
    in_RCX[1] = 4;
    in_RCX[2] = 0;
  }
  return;
}

