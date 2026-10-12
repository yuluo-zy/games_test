
void __cdecl utils::wall_space::WallSpaceCoord::to_world_space(void)

{
  undefined8 uVar1;
  undefined8 uVar2;
  longlong lVar3;
  uint extraout_EAX;
  uint extraout_EAX_00;
  undefined8 *in_RCX;
  ulonglong uVar4;
  float *in_RDX;
  longlong in_R8;
  float extraout_XMM0_Da;
  float extraout_XMM0_Da_00;
  float fVar5;
  float fVar6;
  float fVar7;
  float fVar8;
  float fVar9;
  float fVar10;
  float fVar11;
  float fVar12;
  float fVar13;
  float local_ac;
  float *local_a8;
  code *local_a0;
  undefined **local_98;
  undefined8 local_90;
  float **local_88;
  undefined8 local_80;
  undefined8 local_78;
  
  fVar13 = __real_3f800000;
  fVar12 = *in_RDX;
  fVar6 = *(float *)(in_R8 + 0x30);
  fVar5 = fVar12 / fVar6;
  if ((fVar12 < 0.0) || (fVar6 < fVar12)) {
    if (NAN(fVar5)) {
                    /* WARNING: Subroutine does not return */
      core::panicking::panic();
    }
    if (ABS(fVar5) == INFINITY) {
                    /* WARNING: Subroutine does not return */
      core::panicking::panic();
    }
    curve::Curve::
    get_coord_at_u<glam::f32::vec2::Vec2,utils::curve::CurveCoordSemantic2D,utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>_>
              ();
    uVar4 = (ulonglong)extraout_EAX_00;
    if (*(ulonglong *)(in_R8 + 0x10) <= uVar4) goto LAB_140c9238d;
    if (*(ulonglong *)(in_R8 + 0x10) <= uVar4 + 1) goto LAB_140c92399;
    uVar1 = *(undefined8 *)(*(longlong *)(in_R8 + 8) + uVar4 * 8);
    uVar2 = *(undefined8 *)(*(longlong *)(in_R8 + 8) + 8 + uVar4 * 8);
    fVar9 = (float)((ulonglong)uVar1 >> 0x20);
    fVar8 = (float)uVar1;
    fVar10 = (float)uVar2;
    fVar11 = (float)((ulonglong)uVar2 >> 0x20);
    fVar7 = fVar11 * extraout_XMM0_Da_00 + fVar9 * (fVar13 - extraout_XMM0_Da_00);
    fVar5 = extraout_XMM0_Da_00 * fVar10 + (fVar13 - extraout_XMM0_Da_00) * fVar8;
    fVar10 = fVar10 - fVar8;
    fVar11 = fVar11 - fVar9;
    fVar8 = fVar13 / SQRT(fVar11 * fVar11 + fVar10 * fVar10);
    fVar10 = fVar8 * fVar10;
    fVar8 = fVar8 * fVar11;
    fVar13 = fVar13 / SQRT(fVar8 * fVar8 + fVar10 * fVar10);
    if (fVar12 <= fVar6) {
      fVar5 = fVar5 - fVar12 * fVar10 * fVar13;
      fVar6 = in_RDX[1] - fVar12 * fVar13 * 0.0;
      fVar7 = fVar7 - fVar8 * fVar13 * fVar12;
    }
    else {
      fVar12 = fVar12 - fVar6;
      fVar5 = fVar5 + fVar12 * fVar10 * fVar13;
      fVar6 = in_RDX[1] + fVar12 * fVar13 * 0.0;
      fVar7 = fVar7 + fVar12 * fVar8 * fVar13;
    }
  }
  else {
    local_ac = fVar5;
    if ((fVar5 < 0.0) || (__real_3f800000 < fVar5)) {
      local_a8 = &local_ac;
      local_a0 = core::fmt::float::impl$5::fmt;
      local_98 = &anon_ca83a639f917387dae152b8c26146856_1_llvm_10654403658398494596;
      local_90 = 1;
      local_78 = 0;
      local_88 = &local_a8;
      local_80 = 1;
                    /* WARNING: Subroutine does not return */
      core::panicking::panic_fmt();
    }
    if (NAN(fVar5)) {
                    /* WARNING: Subroutine does not return */
      core::panicking::panic();
    }
    curve::Curve::
    get_coord_at_u<glam::f32::vec2::Vec2,utils::curve::CurveCoordSemantic2D,utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>_>
              ();
    uVar4 = (ulonglong)extraout_EAX;
    if (*(ulonglong *)(in_R8 + 0x10) <= uVar4) {
LAB_140c9238d:
                    /* WARNING: Subroutine does not return */
      core::panicking::panic_bounds_check();
    }
    if (*(ulonglong *)(in_R8 + 0x10) <= uVar4 + 1) {
LAB_140c92399:
                    /* WARNING: Subroutine does not return */
      core::panicking::panic_bounds_check();
    }
    lVar3 = *(longlong *)(in_R8 + 8);
    fVar7 = *(float *)(lVar3 + 0xc + uVar4 * 8) * extraout_XMM0_Da +
            (fVar13 - extraout_XMM0_Da) * *(float *)(lVar3 + 4 + uVar4 * 8);
    fVar5 = *(float *)(lVar3 + 8 + uVar4 * 8) * extraout_XMM0_Da +
            *(float *)(lVar3 + uVar4 * 8) * (fVar13 - extraout_XMM0_Da);
    fVar6 = in_RDX[1];
  }
  *in_RCX = CONCAT44(fVar6,fVar5);
  *(float *)(in_RCX + 1) = fVar7;
  return;
}

