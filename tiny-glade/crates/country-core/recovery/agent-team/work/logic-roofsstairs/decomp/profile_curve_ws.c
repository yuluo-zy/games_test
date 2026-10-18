
void __cdecl country_core::resources::roofs::roof_shape::profile_curve_ws(void)

{
  float *pfVar1;
  float *pfVar2;
  float fVar3;
  float fVar4;
  ulonglong uVar5;
  ulonglong uVar6;
  float fVar7;
  float fVar8;
  float fVar9;
  float fVar10;
  float fVar11;
  ulonglong extraout_RAX;
  longlong *in_RCX;
  ulonglong uVar12;
  longlong in_RDX;
  ulonglong uVar13;
  float in_XMM2_Da;
  float in_XMM3_Da;
  float in_stack_00000028;
  longlong local_a0;
  longlong lStack_98;
  longlong local_90;
  longlong lStack_88;
  longlong local_80;
  longlong lStack_78;
  longlong local_70;
  
  uVar5 = *(ulonglong *)(in_RDX + 0x10);
  if (uVar5 == 0) goto LAB_1408e3e90;
  uVar6 = *(ulonglong *)(in_RDX + 8);
  std::alloc::__default_lib_allocator::__rust_alloc();
  if (extraout_RAX == 0) {
                    // WARNING: Subroutine does not return
    alloc::raw_vec::handle_error();
  }
  if (uVar5 < 0xc) {
LAB_1408e3dc0:
    uVar12 = 0;
  }
  else {
    uVar12 = 0;
    if (((extraout_RAX <= (extraout_RAX + uVar5 * 8) - 8) &&
        (extraout_RAX + 4 <= (extraout_RAX + uVar5 * 8) - 4)) && (uVar5 - 1 >> 0x3d == 0)) {
      if (extraout_RAX < uVar6 + uVar5 * 8 && uVar6 < extraout_RAX + uVar5 * 8) goto LAB_1408e3dc0;
      uVar12 = uVar5 & 0x3ffffffffffffffc;
      uVar13 = 0;
      do {
        pfVar1 = (float *)(uVar6 + uVar13 * 8);
        fVar3 = *pfVar1;
        fVar4 = pfVar1[1];
        fVar7 = pfVar1[2];
        fVar8 = pfVar1[3];
        pfVar1 = (float *)(uVar6 + 0x10 + uVar13 * 8);
        fVar9 = pfVar1[1];
        fVar10 = pfVar1[2];
        fVar11 = pfVar1[3];
        pfVar2 = (float *)(extraout_RAX + 0x10 + uVar13 * 8);
        *pfVar2 = *pfVar1 * in_XMM3_Da + (1.0 - *pfVar1) * in_XMM2_Da;
        pfVar2[1] = fVar9 * in_stack_00000028;
        pfVar2[2] = fVar10 * in_XMM3_Da + (1.0 - fVar10) * in_XMM2_Da;
        pfVar2[3] = fVar11 * in_stack_00000028;
        pfVar1 = (float *)(extraout_RAX + uVar13 * 8);
        *pfVar1 = fVar3 * in_XMM3_Da + (1.0 - fVar3) * in_XMM2_Da;
        pfVar1[1] = fVar4 * in_stack_00000028;
        pfVar1[2] = fVar7 * in_XMM3_Da + (1.0 - fVar7) * in_XMM2_Da;
        pfVar1[3] = fVar8 * in_stack_00000028;
        uVar13 = uVar13 + 4;
      } while (uVar12 != uVar13);
      if (uVar5 == uVar12) goto LAB_1408e3e90;
    }
  }
  uVar13 = uVar12 | 1;
  if ((uVar5 & 1) != 0) {
    fVar3 = *(float *)(uVar6 + uVar12 * 8);
    fVar4 = *(float *)(uVar6 + 4 + uVar12 * 8);
    *(float *)(extraout_RAX + uVar12 * 8) = fVar3 * in_XMM3_Da + (1.0 - fVar3) * in_XMM2_Da;
    *(float *)(extraout_RAX + 4 + uVar12 * 8) = fVar4 * in_stack_00000028;
    uVar12 = uVar13;
  }
  while (uVar5 != uVar13) {
    fVar3 = *(float *)(uVar6 + uVar12 * 8);
    fVar4 = *(float *)(uVar6 + 4 + uVar12 * 8);
    *(float *)(extraout_RAX + uVar12 * 8) = fVar3 * in_XMM3_Da + (1.0 - fVar3) * in_XMM2_Da;
    *(float *)(extraout_RAX + 4 + uVar12 * 8) = fVar4 * in_stack_00000028;
    fVar3 = *(float *)(uVar6 + 8 + uVar12 * 8);
    fVar4 = *(float *)(uVar6 + 0xc + uVar12 * 8);
    *(float *)(extraout_RAX + 8 + uVar12 * 8) = fVar3 * in_XMM3_Da + (1.0 - fVar3) * in_XMM2_Da;
    *(float *)(extraout_RAX + 0xc + uVar12 * 8) = fVar4 * in_stack_00000028;
    uVar13 = uVar12 + 2;
    uVar12 = uVar13;
  }
LAB_1408e3e90:
  ::utils::curve::Curve::
  try_new_from_points<glam::f32::vec2::Vec2,utils::curve::CurveCoordSemantic2D>();
  if (SBORROW8(0,local_a0)) {
                    // WARNING: Subroutine does not return
    core::result::unwrap_failed();
  }
  in_RCX[6] = local_70;
  in_RCX[4] = local_80;
  in_RCX[5] = lStack_78;
  in_RCX[2] = local_90;
  in_RCX[3] = lStack_88;
  *in_RCX = local_a0;
  in_RCX[1] = lStack_98;
  return;
}

