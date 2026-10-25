
void __cdecl
utils::curve::Curve::smooth_weighted<glam::f32::vec2::Vec2,utils::curve::CurveCoordSemantic2D>(void)

{
  ulonglong uVar1;
  undefined8 *puVar2;
  float *pfVar3;
  undefined8 uVar4;
  undefined8 *puVar5;
  longlong lVar6;
  undefined1 auVar7 [16];
  longlong lVar8;
  longlong *in_RCX;
  ulonglong uVar9;
  undefined1 uVar10;
  longlong *in_RDX;
  longlong in_R8;
  longlong lVar11;
  longlong lVar12;
  longlong in_R9;
  longlong lVar13;
  ulonglong uVar14;
  ulonglong uVar15;
  float fVar16;
  float fVar17;
  float fVar18;
  float fVar19;
  float fVar20;
  float fVar21;
  float fVar22;
  float fVar23;
  float fVar24;
  float fVar25;
  float fVar26;
  undefined1 auVar27 [16];
  undefined1 auVar28 [16];
  float fVar29;
  float fVar30;
  undefined1 auVar31 [16];
  ulonglong in_stack_00000028;
  undefined1 auVar32 [16];
  
  if (in_RDX[2] != in_stack_00000028) {
                    // WARNING: Subroutine does not return
    core::panicking::panic();
  }
  puVar5 = (undefined8 *)in_RDX[1];
  if (2 < in_stack_00000028 && in_R8 != 0) {
    fVar21 = (float)*puVar5;
    fVar22 = (float)((ulonglong)*puVar5 >> 0x20);
    uVar1 = in_stack_00000028 - 1;
    uVar9 = in_stack_00000028 - 3;
    if (uVar1 < in_stack_00000028 - 3) {
      uVar9 = uVar1;
    }
    puVar2 = puVar5 + 1;
    lVar8 = 0;
    do {
      fVar25 = *(float *)(puVar5 + 1);
      fVar19 = *(float *)((longlong)puVar5 + 0xc);
      fVar18 = 0.0;
      fVar23 = 0.0;
      uVar14 = 1;
      fVar16 = fVar21;
      fVar17 = fVar22;
      if (4 < uVar9 + 1 &&
          (((puVar2 <= puVar5 + uVar9 + 1 && uVar9 >> 0x3d == 0) &&
           ((longlong)puVar5 + 0xcU <= (longlong)puVar5 + uVar9 * 8 + 0xc &&
           puVar5 + 2 <= puVar5 + uVar9 + 2)) &&
          (longlong)puVar5 + 0x14U <= (longlong)puVar5 + uVar9 * 8 + 0x14)) {
        uVar15 = 0;
        fVar29 = fVar22;
        fVar20 = fVar21;
        do {
          uVar14 = *(ulonglong *)(in_R9 + 0x10 + uVar15 * 8);
          if ((longlong)uVar14 < 0) {
            fVar26 = (float)uVar14;
            uVar14 = *(ulonglong *)(in_R9 + 8 + uVar15 * 8);
            if ((longlong)uVar14 < 0) goto LAB_141468b28;
LAB_141468a20:
            fVar30 = (float)(longlong)uVar14;
          }
          else {
            fVar26 = (float)(longlong)uVar14;
            uVar14 = *(ulonglong *)(in_R9 + 8 + uVar15 * 8);
            if (-1 < (longlong)uVar14) goto LAB_141468a20;
LAB_141468b28:
            fVar30 = (float)uVar14;
          }
          pfVar3 = (float *)(puVar5 + 2 + uVar15);
          fVar16 = *pfVar3;
          fVar17 = pfVar3[1];
          fVar18 = pfVar3[2];
          fVar23 = pfVar3[3];
          auVar31._0_4_ = fVar30 * fVar25 + fVar20 * 0.5 + fVar16 * 0.5;
          auVar32._4_4_ = fVar26 * fVar16 + fVar25 * 0.5 + fVar18 * 0.5;
          auVar32._0_4_ = auVar31._0_4_;
          auVar32._8_4_ = auVar32._4_4_;
          auVar32._12_4_ = fVar17 * fVar26 + fVar19 * 0.5 + fVar23 * 0.5;
          auVar31._8_8_ = auVar32._8_8_;
          auVar31._4_4_ = fVar19 * fVar30 + fVar29 * 0.5 + fVar17 * 0.5;
          auVar28._4_4_ = fVar30 + 1.0;
          auVar28._0_4_ = fVar30 + 1.0;
          auVar28._8_4_ = fVar26 + 1.0;
          auVar28._12_4_ = fVar26 + 1.0;
          auVar28 = divps(auVar31,auVar28);
          *(undefined1 (*) [16])(puVar5 + uVar15 + 1) = auVar28;
          uVar15 = uVar15 + 2;
          uVar14 = uVar9 | 1;
          fVar25 = fVar18;
          fVar29 = fVar17;
          fVar19 = fVar23;
          fVar20 = fVar16;
        } while ((uVar9 & 0xfffffffffffffffe) != uVar15);
      }
      lVar8 = lVar8 + 1;
      fVar29 = fVar18;
      fVar20 = fVar23;
      do {
        if (in_stack_00000028 == uVar14) {
                    // WARNING: Subroutine does not return
          core::panicking::panic_bounds_check();
        }
        fVar24 = (float)*(ulonglong *)(in_R9 + uVar14 * 8);
        fVar26 = *(float *)(puVar2 + uVar14);
        fVar30 = *(float *)((longlong)(puVar2 + uVar14) + 4);
        auVar27._0_4_ = fVar24 * fVar25 + fVar16 * 0.5 + fVar26 * 0.5;
        auVar27._4_4_ = fVar24 * fVar19 + fVar17 * 0.5 + fVar30 * 0.5;
        auVar27._8_4_ = fVar29 * 0.0 + fVar18 * 0.0 + 0.0;
        auVar27._12_4_ = fVar20 * 0.0 + fVar23 * 0.0 + 0.0;
        auVar7._4_4_ = fVar24 + 1.0;
        auVar7._0_4_ = fVar24 + 1.0;
        auVar7._8_8_ = 0;
        auVar28 = divps(auVar27,auVar7);
        puVar5[uVar14] = auVar28._0_8_;
        uVar14 = uVar14 + 1;
        fVar16 = fVar25;
        fVar17 = fVar19;
        fVar18 = fVar29;
        fVar23 = fVar20;
        fVar29 = 0.0;
        fVar20 = 0.0;
        fVar25 = fVar26;
        fVar19 = fVar30;
      } while (uVar1 != uVar14);
    } while (lVar8 != in_R8);
  }
  in_RDX[5] = 0;
  if (in_stack_00000028 < 2) {
    uVar10 = 0;
  }
  else {
    fVar21 = (float)*puVar5;
    fVar22 = (float)((ulonglong)*puVar5 >> 0x20);
    if ((ulonglong)in_RDX[3] < in_stack_00000028) {
      alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
      lVar8 = in_RDX[5];
    }
    else {
      lVar8 = 0;
    }
    lVar6 = in_RDX[4];
    lVar12 = lVar6 + lVar8 * 4;
    fVar16 = 0.0;
    lVar11 = 0;
    lVar13 = 0;
    do {
      uVar4 = *(undefined8 *)((longlong)puVar5 + lVar13 * 2);
      fVar17 = (float)uVar4;
      fVar18 = (float)((ulonglong)uVar4 >> 0x20);
      fVar21 = fVar21 - fVar17;
      fVar22 = fVar22 - fVar18;
      fVar16 = SQRT(fVar22 * fVar22 + fVar21 * fVar21) + fVar16;
      *(float *)(lVar12 + lVar13) = fVar16;
      uVar4 = *(undefined8 *)((longlong)puVar5 + lVar13 * 2 + 8);
      fVar21 = (float)uVar4;
      fVar22 = (float)((ulonglong)uVar4 >> 0x20);
      fVar17 = fVar17 - fVar21;
      fVar18 = fVar18 - fVar22;
      fVar16 = SQRT(fVar18 * fVar18 + fVar17 * fVar17) + fVar16;
      *(float *)(lVar12 + 4 + lVar13) = fVar16;
      lVar13 = lVar13 + 8;
      lVar11 = lVar11 + -2;
    } while (-lVar11 != (in_stack_00000028 & 0xfffffffffffffffe));
    if ((in_stack_00000028 & 1) == 0) {
      lVar8 = lVar8 - lVar11;
    }
    else {
      uVar4 = *(undefined8 *)((longlong)puVar5 + lVar13 * 2);
      fVar21 = fVar21 - (float)uVar4;
      fVar22 = fVar22 - (float)((ulonglong)uVar4 >> 0x20);
      fVar16 = fVar16 + SQRT(fVar22 * fVar22 + fVar21 * fVar21);
      *(float *)(lVar13 + lVar12) = fVar16;
      lVar8 = (lVar8 - lVar11) + 1;
    }
    in_RDX[5] = lVar8;
    uVar10 = 1;
    if ((fVar16 != 0.0) || (NAN(fVar16))) {
      lVar12 = 0;
      do {
        if (lVar8 * 4 == lVar12) {
          *(float *)(in_RDX + 6) = fVar16;
          lVar8 = *in_RDX;
          lVar12 = in_RDX[1];
          lVar6 = in_RDX[2];
          lVar11 = in_RDX[3];
          lVar13 = in_RDX[5];
          in_RCX[4] = in_RDX[4];
          in_RCX[5] = lVar13;
          in_RCX[2] = lVar6;
          in_RCX[3] = lVar11;
          *in_RCX = lVar8;
          in_RCX[1] = lVar12;
          in_RCX[6] = in_RDX[6];
          return;
        }
        fVar21 = *(float *)(lVar6 + lVar12) / fVar16;
        *(float *)(lVar6 + lVar12) = fVar21;
        lVar12 = lVar12 + 4;
      } while ((uint)ABS(fVar21) < 0x7f800000);
    }
  }
  *(undefined1 *)(in_RCX + 1) = uVar10;
  *(char *)((longlong)in_RCX + 9) = (char)in_stack_00000028;
  *in_RCX = -0x8000000000000000;
  if (*in_RDX != 0) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  if (in_RDX[3] != 0) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  return;
}

