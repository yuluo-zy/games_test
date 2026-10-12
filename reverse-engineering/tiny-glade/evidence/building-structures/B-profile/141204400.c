
void __cdecl
alloc::vec::spec_from_iter::impl$0::
from_iter<glam::f32::vec3::Vec3,core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<core::iter::adapters::map::Map<utils::loop_range::LoopRange,system_wall_constructor::construct_lintels::construct_lintels::closure_env_3>,core::slice::iter::Iter<f32>_>,system_wall_constructor::construct_lintels::construct_lintels::closure_env_4>_>
          (void)

{
  float fVar1;
  float fVar2;
  undefined8 uVar3;
  longlong lVar4;
  float *pfVar5;
  longlong lVar6;
  ulonglong extraout_RAX;
  undefined8 *extraout_RAX_00;
  ulonglong extraout_RAX_01;
  undefined8 *in_RCX;
  longlong *in_RDX;
  float fVar7;
  ulonglong local_108;
  float *pfStack_f0;
  byte local_c4;
  longlong local_a8;
  
  ::utils::loop_range::impl$2::next();
  if ((local_108 & 1) != 0) {
    lVar4 = *in_RDX;
    ::utils::loop_range::LoopIdx::index();
    if (*(ulonglong *)(lVar4 + 0x98) <= extraout_RAX) {
                    /* WARNING: Subroutine does not return */
      core::panicking::panic_bounds_check();
    }
    pfVar5 = (float *)in_RDX[3];
    if (pfVar5 != (float *)in_RDX[4]) {
      lVar4 = *(longlong *)(lVar4 + 0x90);
      in_RDX[3] = (longlong)(pfVar5 + 1);
      fVar1 = *pfVar5;
      uVar3 = *(undefined8 *)(lVar4 + extraout_RAX * 0xc);
      fVar2 = *(float *)(lVar4 + 8 + extraout_RAX * 0xc);
      std::alloc::__default_lib_allocator::__rust_alloc();
      if (extraout_RAX_00 == (undefined8 *)0x0) {
                    /* WARNING: Subroutine does not return */
        raw_vec::handle_error();
      }
      fVar7 = fVar1 * 0.0;
      *extraout_RAX_00 = CONCAT44(fVar1 + (float)((ulonglong)uVar3 >> 0x20),fVar7 + (float)uVar3);
      *(float *)(extraout_RAX_00 + 1) = fVar2 + fVar7;
      local_a8 = 1;
      lVar4 = *in_RDX;
      pfVar5 = (float *)in_RDX[4];
      pfStack_f0 = (float *)in_RDX[3];
      while (::utils::loop_range::impl$2::next(), (local_c4 & 1) != 0) {
        ::utils::loop_range::LoopIdx::index();
        if (*(ulonglong *)(lVar4 + 0x98) <= extraout_RAX_01) {
                    /* WARNING: Subroutine does not return */
          core::panicking::panic_bounds_check();
        }
        if (pfStack_f0 == pfVar5) break;
        lVar6 = *(longlong *)(lVar4 + 0x90);
        fVar1 = *pfStack_f0;
        uVar3 = *(undefined8 *)(lVar6 + extraout_RAX_01 * 0xc);
        fVar7 = fVar1 * 0.0;
        fVar2 = *(float *)(lVar6 + 8 + extraout_RAX_01 * 0xc);
        if (local_a8 == 4) {
          raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
        }
        *(ulonglong *)((longlong)extraout_RAX_00 + local_a8 * 0xc) =
             CONCAT44(fVar1 + (float)((ulonglong)uVar3 >> 0x20),fVar7 + (float)uVar3);
        *(float *)((longlong)extraout_RAX_00 + local_a8 * 0xc + 8) = fVar2 + fVar7;
        local_a8 = local_a8 + 1;
        pfStack_f0 = pfStack_f0 + 1;
      }
      in_RCX[2] = local_a8;
      *in_RCX = 4;
      in_RCX[1] = extraout_RAX_00;
      return;
    }
  }
  *in_RCX = 0;
  in_RCX[1] = 4;
  in_RCX[2] = 0;
  return;
}

