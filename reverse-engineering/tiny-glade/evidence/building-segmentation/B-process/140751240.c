
void __cdecl
system_wall_constructor::segmentation::calculate_wall_path_segmentation::add_splits_in_empty_range
          (void)

{
  float fVar1;
  float fVar2;
  undefined8 uVar3;
  uint extraout_EAX;
  undefined4 extraout_EAX_00;
  longlong in_RCX;
  undefined8 *in_R8;
  longlong in_R9;
  ulonglong uVar4;
  ulonglong uVar5;
  ulonglong uVar6;
  float fVar7;
  float fVar8;
  float extraout_XMM0_Da;
  undefined4 extraout_XMM0_Da_00;
  ulonglong in_stack_00000028;
  longlong in_stack_00000030;
  ulonglong in_stack_00000038;
  float in_stack_00000040;
  float local_160 [6];
  float *local_148;
  code *pcStack_140;
  undefined8 local_138;
  undefined8 uStack_130;
  longlong local_118;
  undefined8 uStack_110;
  undefined8 local_108;
  longlong lStack_100;
  undefined8 local_f8;
  undefined8 uStack_f0;
  undefined8 local_e8;
  undefined **local_d8;
  undefined8 uStack_d0;
  float **local_c8;
  longlong lStack_c0;
  undefined8 local_b8;
  undefined8 uStack_b0;
  undefined8 local_a8;
  undefined8 local_a0;
  
  local_a0 = 0xfffffffffffffffe;
  local_148 = (float *)*in_R8;
  pcStack_140 = (code *)in_R8[1];
  local_138 = in_R8[2];
  uStack_130 = in_R8[3];
  alloc::vec::spec_from_iter::impl$0::
  from_iter<glam::f32::vec2::Vec2,core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::iter::adapters::skip::Skip<core::iter::adapters::take::Take<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>_>_>_>,glam::f32::vec2::Vec2_(*)(glam::f32::vec3::Vec3)>_>
            ();
  ::utils::curve::Curve::
  try_new_from_points<glam::f32::vec2::Vec2,utils::curve::CurveCoordSemantic2D>();
  if (!SBORROW8(0,(longlong)local_d8)) {
    local_e8 = local_a8;
    uVar3 = local_e8;
    local_f8 = local_b8;
    uStack_f0 = uStack_b0;
    local_108 = local_c8;
    lStack_100 = lStack_c0;
    local_118 = (longlong)local_d8;
    uStack_110 = uStack_d0;
    local_e8._0_4_ = (float)local_a8;
    fVar2 = (float)local_e8;
    fVar7 = (float)local_e8 / in_stack_00000040;
    local_e8 = uVar3;
    fVar8 = floorf(fVar7);
    fVar1 = __real_3f800000;
    fVar7 = __real_3f000000;
    uVar5 = 0;
    if (0.0 <= fVar8) {
      uVar5 = (longlong)(fVar8 - __real_5f000000) & (longlong)fVar8 >> 0x3f | (longlong)fVar8;
    }
    uVar4 = 0xffffffffffffffff;
    if (fVar8 <= __real_5f7fffff) {
      uVar4 = uVar5;
    }
    if ((uVar4 == 1) && (uVar4 = 1, fVar2 < in_stack_00000040 * __real_3fc00000)) {
      uVar4 = 0;
    }
    uVar5 = 0;
    if (uVar4 != 0) {
      do {
        uVar5 = uVar5 + 1;
        local_160[0] = (float)uVar5 / (float)(uVar4 + 1);
        if ((local_160[0] < 0.0) || (fVar1 < local_160[0])) {
          local_148 = local_160;
          pcStack_140 = core::fmt::float::impl$5::fmt;
          local_d8 = &anon_95101f4d2e0328d894274322bc39dc37_3_llvm_14090887955549727255;
          uStack_d0 = 1;
          local_b8 = 0;
          local_c8 = &local_148;
          lStack_c0 = 1;
                    /* WARNING: Subroutine does not return */
          core::panicking::panic_fmt();
        }
        if (NAN(local_160[0])) {
                    /* WARNING: Subroutine does not return */
          core::panicking::panic();
        }
        ::utils::curve::Curve::
        get_coord_at_u<glam::f32::vec3::Vec3,utils::curve::CurveCoordSemantic3D,utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic3D>_>
                  ();
        ::utils::loop_range::LoopRange::len();
        ::utils::curve::CurveInternalCoord::
        new_for_point_count<utils::curve::CurveCoordSemanticThresholded3D>();
        uVar6 = (ulonglong)extraout_EAX + in_RCX + (ulonglong)(fVar7 < extraout_XMM0_Da);
        if (in_stack_00000028 <= uVar6) {
                    /* WARNING: Subroutine does not return */
          core::panicking::panic_bounds_check();
        }
        *(byte *)(in_R9 + uVar6) = *(byte *)(in_R9 + uVar6) | 8;
        if (in_stack_00000038 <= uVar6) {
                    /* WARNING: Subroutine does not return */
          core::panicking::panic_bounds_check();
        }
        *(undefined4 *)(in_stack_00000030 + uVar6 * 8) = extraout_EAX_00;
        *(undefined4 *)(in_stack_00000030 + 4 + uVar6 * 8) = extraout_XMM0_Da_00;
      } while (uVar5 < uVar4);
    }
    if (local_118 != 0) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
    if (lStack_100 != 0) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
  }
  return;
}

