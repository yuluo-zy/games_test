
void __cdecl
alloc::vec::spec_from_iter::impl$0::
from_iter<f32,core::iter::adapters::map::Map<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,system_wall_constructor::segmentation::split_wall_path_segment_into_platform_stairs::split_wall_path_segment_into_platform_stairs::closure_env_5>_>
          (void)

{
  longlong lVar1;
  ulonglong uVar2;
  ulonglong extraout_RAX;
  float *extraout_RAX_00;
  ulonglong extraout_RAX_01;
  undefined8 *in_RCX;
  longlong *in_RDX;
  float extraout_XMM0_Da;
  float extraout_XMM0_Da_00;
  float fVar3;
  float fVar4;
  int local_d8;
  int local_a4;
  longlong local_88;
  
  ::utils::loop_range::impl$2::next();
  if (local_d8 == 1) {
    lVar1 = *in_RDX;
    uVar2 = in_RDX[1];
    ::utils::loop_range::LoopIdx::index();
    if (uVar2 <= extraout_RAX) {
                    /* WARNING: Subroutine does not return */
      core::panicking::panic_bounds_check();
    }
    fVar3 = *(float *)(lVar1 + 4 + extraout_RAX * 0xc);
    country_core::resources::walls::public_wall_state::PublicWallState::max_y();
    std::alloc::__default_lib_allocator::__rust_alloc();
    if (extraout_RAX_00 == (float *)0x0) {
                    /* WARNING: Subroutine does not return */
      raw_vec::handle_error();
    }
    if (fVar3 <= __real_3f0ccccd) {
      fVar3 = __real_3f0ccccd;
    }
    *extraout_RAX_00 = extraout_XMM0_Da - fVar3;
    fVar3 = __real_3f0ccccd;
    local_88 = 1;
    lVar1 = *in_RDX;
    uVar2 = in_RDX[1];
    while( true ) {
      ::utils::loop_range::impl$2::next();
      if (local_a4 != 1) break;
      ::utils::loop_range::LoopIdx::index();
      if (uVar2 <= extraout_RAX_01) {
                    /* WARNING: Subroutine does not return */
        core::panicking::panic_bounds_check();
      }
      fVar4 = *(float *)(lVar1 + 4 + extraout_RAX_01 * 0xc);
      country_core::resources::walls::public_wall_state::PublicWallState::max_y();
      if (fVar4 <= fVar3) {
        fVar4 = fVar3;
      }
      if (local_88 == 4) {
        raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
      }
      extraout_RAX_00[local_88] = extraout_XMM0_Da_00 - fVar4;
      local_88 = local_88 + 1;
    }
    in_RCX[2] = local_88;
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

