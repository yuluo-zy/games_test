// name: split_curve_based_on_angle
// addr: 0x141fdf9e0
// size: 810
// signature: void __cdecl split_curve_based_on_angle(void)


void __cdecl
system_wall_constructor::construct_fences::process_segment::split_curve_based_on_angle(void)

{
  ulonglong uVar1;
  float fVar2;
  undefined8 *extraout_RAX;
  undefined8 *puVar3;
  undefined8 *in_RCX;
  ulonglong uVar4;
  longlong in_RDX;
  ulonglong in_R8;
  bool bVar5;
  float fVar6;
  float in_XMM3_Da;
  float fVar7;
  float fVar8;
  undefined8 *local_138;
  undefined8 *puStack_130;
  undefined8 *local_128;
  undefined8 *local_c8;
  undefined8 *puStack_c0;
  undefined8 *local_b8;
  longlong local_b0;
  float local_a8;
  float fStack_a4;
  float local_98;
  float fStack_94;
  longlong local_78;
  float local_70;
  float fStack_6c;
  
  fVar6 = cosf(in_XMM3_Da);
  std::alloc::__default_lib_allocator::__rust_alloc();
  if (extraout_RAX == (undefined8 *)0x0) {
                    // WARNING: Subroutine does not return
    alloc::alloc::handle_alloc_error();
  }
  *extraout_RAX = 0;
  local_78 = 1;
  local_c8 = (undefined8 *)0x0;
  local_b0 = 0;
  uVar4 = 0;
  while (uVar4 < in_R8) {
    uVar1 = uVar4 + 1;
    if (in_R8 <= uVar4) {
                    // WARNING: Subroutine does not return
      core::panicking::panic_bounds_check();
    }
    puVar3 = (undefined8 *)(uVar4 * 0xc + in_RDX);
    bVar5 = local_c8 == (undefined8 *)0x0;
    local_c8 = puStack_c0;
    puStack_c0 = local_b8;
    if ((bVar5) &&
       (itertools::tuple_impl::impl$30::
        collect_from_iter_no_buf<ref_<glam::f32::vec3::Vec3>,core::iter::adapters::chain::Chain<core::iter::sources::once::Once<ref_<glam::f32::vec3::Vec3>_>,ref_mut_<core::iter::adapters::map::Map<core::ops::range::Range<usize>,system_wall_constructor::construct_fences::process_segment::split_curve_based_on_angle::closure_env_0>_>_>_>
                  (), puVar3 = local_128, local_c8 = local_138, puStack_c0 = puStack_130,
       local_138 == (undefined8 *)0x0)) break;
    local_b0 = local_b0 + 1;
    fVar2 = *(float *)(local_c8 + 1);
    fVar7 = *(float *)(puStack_c0 + 1);
    fVar8 = *(float *)(puVar3 + 1);
    local_98 = (float)*local_c8;
    local_70 = (float)*puStack_c0;
    fStack_6c = (float)((ulonglong)*puStack_c0 >> 0x20);
    if ((local_98 == local_70) && (!NAN(local_98) && !NAN(local_70))) {
      fStack_94 = (float)((ulonglong)*local_c8 >> 0x20);
      if ((fStack_94 == fStack_6c) && (!NAN(fStack_94) && !NAN(fStack_6c))) {
        if ((fVar2 == fVar7) && (!NAN(fVar2) && !NAN(fVar7))) {
                    // WARNING: Subroutine does not return
          core::panicking::assert_failed<glam::f32::vec3::Vec3,glam::f32::vec3::Vec3>();
        }
      }
    }
    local_a8 = (float)*puVar3;
    if ((local_70 == local_a8) && (!NAN(local_70) && !NAN(local_a8))) {
      fStack_a4 = (float)((ulonglong)*puVar3 >> 0x20);
      if ((fStack_6c == fStack_a4) && (!NAN(fStack_6c) && !NAN(fStack_a4))) {
        if ((fVar7 == fVar8) && (!NAN(fVar7) && !NAN(fVar8))) {
                    // WARNING: Subroutine does not return
          core::panicking::assert_failed<glam::f32::vec3::Vec3,glam::f32::vec3::Vec3>();
        }
      }
    }
    fVar8 = fVar8 - fVar7;
    fVar7 = fVar7 - fVar2;
    local_a8 = local_a8 - local_70;
    local_70 = local_70 - local_98;
    uVar4 = uVar1;
    local_b8 = puVar3;
    if ((fVar8 * fVar7 + local_70 * local_a8) /
        SQRT((fVar8 * fVar8 + local_a8 * local_a8) * (fVar7 * fVar7 + local_70 * local_70)) < fVar6)
    {
      if (local_78 == 1) {
        alloc::raw_vec::RawVec::grow_one<usize,alloc::alloc::Global>();
      }
      extraout_RAX[local_78] = local_b0;
      local_78 = local_78 + 1;
    }
  }
  if (local_78 == 1) {
    alloc::raw_vec::RawVec::grow_one<usize,alloc::alloc::Global>();
  }
  extraout_RAX[local_78] = in_R8 - 1;
  in_RCX[2] = local_78 + 1;
  *in_RCX = 1;
  in_RCX[1] = extraout_RAX;
  return;
}

