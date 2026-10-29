
void __cdecl core::iter::adapters::map::impl$2::railing_map(void *env,void *output)

{
  ulonglong uVar1;
  float fVar2;
  undefined8 uVar3;
  undefined8 uVar4;
  longlong lVar5;
  longlong lVar6;
  longlong *plVar7;
  ulonglong uVar8;
  longlong *plVar9;
  longlong lVar10;
  void *curve;
  longlong lVar11;
  longlong lVar12;
  float fVar13;
  float fVar14;
  uint extraout_EAX;
  uint uVar15;
  ulonglong uVar16;
  longlong lVar17;
  ulonglong uVar18;
  float fVar19;
  float extraout_XMM0_Da;
  float extraout_XMM0_Da_00;
  float fVar20;
  
  fVar14 = __real_5f000000;
  fVar13 = __real_3f800000;
  lVar5 = *(longlong *)env;
  lVar6 = *(longlong *)((longlong)env + 8);
  plVar7 = *(longlong **)output;
  lVar17 = *(longlong *)((longlong)output + 8);
  if (lVar5 != lVar6) {
    uVar8 = *(ulonglong *)((longlong)env + 0x20);
    plVar9 = *(longlong **)((longlong)env + 0x10);
    lVar10 = *(longlong *)((longlong)env + 0x18);
    curve = *(void **)((longlong)env + 0x28);
    lVar11 = *(longlong *)((longlong)output + 0x10);
    uVar18 = 0;
    do {
      fVar2 = *(float *)(lVar5 + uVar18 * 4);
      fVar19 = 0.0;
      if (0.0 <= fVar2) {
        fVar19 = fVar2;
      }
      fVar20 = fVar13;
      if (fVar19 <= fVar13) {
        fVar20 = fVar19;
      }
      fVar20 = fVar20 * (float)(uVar8 - 1);
      fVar19 = floorf(fVar20);
      truncf(fVar20,(longlong)(fVar19 - fVar14) & (longlong)fVar19 >> 0x3f | (longlong)fVar19);
      ::utils::curve::CurveInternalCoord::new_for_point_count<utils::curve::CurveCoordSemantic2D>();
      uVar16 = (ulonglong)extraout_EAX;
      if (uVar8 <= uVar16) {
                    /* WARNING: Subroutine does not return */
        panicking::panic_bounds_check();
      }
      if (uVar8 <= uVar16 + 1) {
                    /* WARNING: Subroutine does not return */
        panicking::panic_bounds_check();
      }
      fVar19 = *(float *)(lVar10 + uVar16 * 4);
      fVar20 = *(float *)(lVar10 + 4 + uVar16 * 4);
      lVar12 = plVar9[2];
      if (lVar12 == *plVar9) {
        ::alloc::raw_vec::RawVec::grow_one<f32,alloc::alloc::Global>();
      }
      *(float *)(plVar9[1] + lVar12 * 4) = (fVar20 - fVar19) * extraout_XMM0_Da + fVar19;
      plVar9[2] = lVar12 + 1;
      if ((fVar2 < 0.0) || (fVar13 < fVar2)) {
                    /* WARNING: Subroutine does not return */
        panicking::panic_fmt();
      }
      if (NAN(fVar2)) {
                    /* WARNING: Subroutine does not return */
        panicking::panic();
      }
      uVar15 = ::utils::curve::Curve::curve_coord(curve,fVar2);
      uVar16 = (ulonglong)uVar15;
      if (*(ulonglong *)((longlong)curve + 0x10) <= uVar16) {
                    /* WARNING: Subroutine does not return */
        panicking::panic_bounds_check();
      }
      uVar1 = uVar16 + 1;
      if (*(ulonglong *)((longlong)curve + 0x10) <= uVar1) {
                    /* WARNING: Subroutine does not return */
        panicking::panic_bounds_check();
      }
      lVar12 = *(longlong *)((longlong)curve + 8);
      fVar20 = fVar13 - extraout_XMM0_Da_00;
      fVar2 = *(float *)(lVar12 + 8 + uVar16 * 0xc);
      fVar19 = *(float *)(lVar12 + 8 + uVar1 * 0xc);
      uVar3 = *(undefined8 *)(lVar12 + uVar16 * 0xc);
      uVar4 = *(undefined8 *)(lVar12 + uVar1 * 0xc);
      *(ulonglong *)(lVar11 + lVar17 * 0xc) =
           CONCAT44((float)((ulonglong)uVar4 >> 0x20) * extraout_XMM0_Da_00 +
                    (float)((ulonglong)uVar3 >> 0x20) * fVar20,
                    (float)uVar4 * extraout_XMM0_Da_00 + (float)uVar3 * fVar20);
      *(float *)(lVar11 + 8 + lVar17 * 0xc) = fVar19 * extraout_XMM0_Da_00 + fVar20 * fVar2;
      lVar17 = lVar17 + 1;
      uVar18 = uVar18 + 1;
    } while (uVar18 != (ulonglong)(lVar6 - lVar5) >> 2);
  }
  *plVar7 = lVar17;
  return;
}

