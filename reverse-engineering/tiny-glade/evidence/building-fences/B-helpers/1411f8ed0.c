
void __cdecl
utils::curve::Curve::crashy_resample<glam::f32::vec3::Vec3,utils::curve::CurveCoordSemantic3D>(void)

{
  ulonglong uVar1;
  longlong lVar2;
  undefined1 *puVar3;
  float fVar4;
  float fVar5;
  undefined8 uVar6;
  undefined8 uVar7;
  ulonglong uVar8;
  undefined8 *puVar9;
  longlong lVar10;
  undefined1 auVar11 [16];
  unkuint9 Var12;
  ulonglong uVar13;
  undefined8 *extraout_RAX;
  undefined8 *puVar14;
  undefined8 *extraout_RAX_00;
  undefined8 *puVar15;
  undefined8 *in_RCX;
  longlong in_RDX;
  ulonglong uVar16;
  float *pfVar17;
  ulonglong uVar18;
  ulonglong uVar19;
  float fVar20;
  float fVar21;
  float extraout_XMM0_Da;
  float fVar22;
  float fVar23;
  float in_XMM2_Da;
  float fVar24;
  float fVar25;
  float fVar26;
  float fVar27;
  float fVar28;
  float fVar29;
  float fVar30;
  undefined8 local_198;
  ulonglong local_148;
  float *local_130;
  undefined8 local_128;
  float local_114;
  ulonglong local_100;
  
  uVar8 = *(ulonglong *)(in_RDX + 0x10);
  if (uVar8 < 2) {
                    /* WARNING: Subroutine does not return */
    core::panicking::panic();
  }
  if ((in_XMM2_Da == 0.0) && (!NAN(in_XMM2_Da))) {
                    /* WARNING: Subroutine does not return */
    core::panicking::panic();
  }
  fVar20 = *(float *)(in_RDX + 0x30);
  if ((fVar20 == 0.0) && (!NAN(fVar20))) {
                    /* WARNING: Subroutine does not return */
    core::panicking::panic();
  }
  if (fVar20 <= __real_322bcc77) {
    fVar20 = __real_322bcc77;
  }
  fVar23 = in_XMM2_Da / fVar20;
  if (__real_3f800000 <= in_XMM2_Da / fVar20) {
    fVar23 = __real_3f800000;
  }
  fVar20 = (float)roundf(__real_3f800000 / fVar23);
  uVar18 = 0;
  if (0.0 <= fVar20) {
    uVar18 = (longlong)(fVar20 - __real_5f000000) & (longlong)fVar20 >> 0x3f | (longlong)fVar20;
  }
  uVar13 = 0xffffffffffffffff;
  if (fVar20 <= __real_5f7fffff) {
    uVar13 = uVar18;
  }
  uVar18 = uVar13 + 1 + (ulonglong)(uVar13 == 0);
  if (uVar18 < 2) {
                    /* WARNING: Subroutine does not return */
    core::panicking::panic();
  }
  Var12 = (unkuint9)uVar18;
  auVar11._8_8_ = 0;
  auVar11._0_8_ = uVar18;
  uVar13 = SUB168(auVar11 * ZEXT816(0xc),0);
  if ((SUB168(auVar11 * ZEXT816(0xc),8) != 0) || (0x7ffffffffffffffc < uVar13)) {
LAB_1411f904c:
                    /* WARNING: Subroutine does not return */
    alloc::raw_vec::handle_error();
  }
  if (uVar13 == 0) {
    puVar15 = (undefined8 *)&DAT_00000004;
    uVar18 = 0;
  }
  else {
    std::alloc::__default_lib_allocator::__rust_alloc();
    puVar15 = extraout_RAX;
    if (extraout_RAX == (undefined8 *)0x0) goto LAB_1411f904c;
  }
  fVar28 = __real_3f800000 / ((float)(unkint9)Var12 + __real_bf800000);
  puVar9 = *(undefined8 **)(in_RDX + 8);
  local_128 = *puVar9;
  fVar20 = *(float *)(puVar9 + 1);
  fVar23 = __real_bf000000 * fVar28 + __real_3f800000;
  *puVar15 = local_128;
  *(float *)(puVar15 + 1) = fVar20;
  local_100 = 1;
  lVar2 = (longlong)puVar9 + uVar8 * 0xc;
  lVar10 = *(longlong *)(in_RDX + 0x20);
  uVar13 = *(ulonglong *)(in_RDX + 0x28);
  uVar16 = uVar13;
  if (uVar8 <= uVar13) {
    uVar16 = uVar8;
  }
  local_148 = 0;
  local_130 = (float *)0x1;
  uVar19 = 1;
  fVar29 = 0.0;
  fVar27 = 0.0;
  fVar30 = 0.0;
  fVar24 = 0.0;
  local_198 = local_128;
  local_114 = fVar20;
  while (pfVar17 = local_130, local_130 != (float *)0x0) {
    local_130 = (float *)0x0;
    core::iter::adapters::zip::impl$1::
    nth<core::slice::iter::Iter<glam::f32::vec3::Vec3>,core::slice::iter::Iter<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic3D>_>_>
              ();
    puVar14 = extraout_RAX_00;
    if (extraout_RAX_00 == (undefined8 *)0x0) goto LAB_1411f93d6;
LAB_1411f91db:
    uVar7 = *puVar14;
    fVar21 = (float)uVar7;
    fVar22 = (float)((ulonglong)uVar7 >> 0x20);
    fVar4 = *(float *)(puVar14 + 1);
    fVar5 = *pfVar17;
    fVar30 = fVar30 + (fVar5 - fVar24);
    fVar24 = fVar23;
    if (fVar30 <= fVar23) {
      fVar24 = fVar30;
    }
    fVar30 = (float)(-(uint)NAN(fVar30) & (uint)fVar23 | ~-(uint)NAN(fVar30) & (uint)fVar24);
    fVar24 = fVar29;
LAB_1411f9220:
    if (fVar28 <= fVar30 - fVar27) {
      fVar27 = fVar27 + fVar28;
      inverse_lerp();
      if ((local_100 == 0) ||
         ((undefined1 *)((longlong)puVar15 + local_100 * 0xc) == (undefined1 *)0xc)) {
                    /* WARNING: Subroutine does not return */
        core::option::unwrap_failed();
      }
      fVar29 = fVar4 - *(float *)((longlong)puVar15 + local_100 * 0xc + -4);
      uVar6 = *(undefined8 *)((longlong)puVar15 + local_100 * 0xc + -0xc);
      fVar25 = fVar21 - (float)uVar6;
      fVar26 = fVar22 - (float)((ulonglong)uVar6 >> 0x20);
      if (SQRT(fVar29 * fVar29 + fVar26 * fVar26 + fVar25 * fVar25) <= fVar24)
      goto code_r0x0001411f92a6;
      fVar20 = 0.0;
      if (0.0 <= extraout_XMM0_Da) {
        fVar20 = extraout_XMM0_Da;
      }
      fVar29 = __real_3f800000;
      if (fVar20 <= __real_3f800000) {
        fVar29 = fVar20;
      }
      fVar24 = __real_3f800000 - fVar29;
      fVar20 = local_114 * fVar24 + fVar4 * fVar29;
      local_128 = CONCAT44(fVar24 * local_198._4_4_ + fVar29 * fVar22,
                           fVar24 * (float)local_198 + fVar29 * fVar21);
      if (local_100 == uVar18) {
        alloc::raw_vec::RawVec::grow_one<glam::f32::vec3::Vec3,alloc::alloc::Global>();
      }
      goto LAB_1411f9329;
    }
    if ((uVar19 == 0) || ((undefined1 *)((longlong)puVar15 + uVar19 * 0xc) == (undefined1 *)0xc)) {
                    /* WARNING: Subroutine does not return */
      core::option::unwrap_failed();
    }
    fVar29 = fVar4 - *(float *)((longlong)puVar15 + uVar19 * 0xc + -4);
    uVar6 = *(undefined8 *)((longlong)puVar15 + uVar19 * 0xc + -0xc);
    fVar25 = fVar21 - (float)uVar6;
    fVar26 = fVar22 - (float)((ulonglong)uVar6 >> 0x20);
    fVar29 = SQRT(fVar29 * fVar29 + fVar26 * fVar26 + fVar25 * fVar25);
    if (fVar29 <= fVar24) {
      fVar21 = (float)local_128;
      fVar22 = local_128._4_4_;
    }
    fVar20 = (float)(~-(uint)(fVar24 < fVar29) & (uint)fVar20 |
                    -(uint)(fVar24 < fVar29) & (uint)fVar4);
    if (fVar29 <= fVar24) {
      fVar29 = fVar24;
    }
    local_128 = CONCAT44(fVar22,fVar21);
    fVar24 = fVar5;
    local_198 = uVar7;
    local_114 = fVar4;
  }
  if (local_148 < uVar16) {
    uVar1 = local_148 + 1;
    puVar14 = (undefined8 *)(local_148 * 0xc + (longlong)puVar9);
    pfVar17 = (float *)(local_148 * 4 + lVar10);
    local_148 = uVar1;
    if (puVar14 != (undefined8 *)0x0) goto LAB_1411f91db;
  }
LAB_1411f93d6:
  puVar3 = (undefined1 *)((longlong)puVar15 + uVar19 * 0xc);
  if (puVar3 == (undefined1 *)0xc) {
                    /* WARNING: Subroutine does not return */
    core::option::unwrap_failed();
  }
  if (lVar2 == 0xc) {
                    /* WARNING: Subroutine does not return */
    core::option::unwrap_failed();
  }
  uVar7 = *(undefined8 *)(lVar2 + -0xc);
  fVar27 = (float)((ulonglong)uVar7 >> 0x20);
  fVar23 = *(float *)(lVar2 + -4);
  fVar28 = (float)uVar7 - (float)*(undefined8 *)(puVar3 + -0xc);
  fVar24 = fVar27 - (float)((ulonglong)*(undefined8 *)(puVar3 + -0xc) >> 0x20);
  fVar28 = SQRT((fVar23 - *(float *)(puVar3 + -4)) * (fVar23 - *(float *)(puVar3 + -4)) +
                fVar24 * fVar24 + fVar28 * fVar28);
  if (fVar28 <= 0.0) {
    if (fVar29 <= fVar28) goto LAB_1411f950b;
    if (uVar19 == uVar18) {
      alloc::raw_vec::RawVec::grow_one<glam::f32::vec3::Vec3,alloc::alloc::Global>();
    }
    pfVar17 = (float *)((longlong)puVar15 + uVar19 * 0xc);
    *pfVar17 = (float)local_128;
    pfVar17[1] = local_128._4_4_;
    *(float *)((longlong)puVar15 + uVar19 * 0xc + 8) = fVar20;
    uVar19 = uVar19 + 1;
    local_128._0_4_ = (float)local_128 - (float)uVar7;
    local_128._4_4_ = local_128._4_4_ - fVar27;
    if (SQRT((fVar20 - fVar23) * (fVar20 - fVar23) +
             local_128._4_4_ * local_128._4_4_ + (float)local_128 * (float)local_128) <= 0.0) {
                    /* WARNING: Subroutine does not return */
      core::panicking::panic();
    }
    if (uVar19 == uVar18) {
      alloc::raw_vec::RawVec::grow_one<glam::f32::vec3::Vec3,alloc::alloc::Global>();
    }
  }
  else if (uVar19 == uVar18) {
    alloc::raw_vec::RawVec::grow_one<glam::f32::vec3::Vec3,alloc::alloc::Global>();
  }
  puVar15 = (undefined8 *)((longlong)puVar15 + uVar19 * 0xc);
  *puVar15 = uVar7;
  *(float *)(puVar15 + 1) = fVar23;
LAB_1411f950b:
  try_new_from_points<glam::f32::vec3::Vec3,utils::curve::CurveCoordSemantic3D>();
  if (SBORROW8(0,(longlong)puVar9)) {
                    /* WARNING: Subroutine does not return */
    core::result::unwrap_failed();
  }
  in_RCX[6] = uVar8;
  in_RCX[4] = local_148;
  in_RCX[5] = uVar16;
  in_RCX[2] = lVar10;
  in_RCX[3] = lVar10 + uVar13 * 4;
  *in_RCX = puVar9;
  in_RCX[1] = lVar2;
  return;
code_r0x0001411f92a6:
  uVar19 = local_100;
  if (0.0 < fVar24) {
    if (local_100 == uVar18) {
      alloc::raw_vec::RawVec::grow_one<glam::f32::vec3::Vec3,alloc::alloc::Global>();
    }
LAB_1411f9329:
    *(undefined8 *)((longlong)puVar15 + local_100 * 0xc) = local_128;
    *(float *)((longlong)puVar15 + local_100 * 0xc + 8) = fVar20;
    uVar19 = local_100 + 1;
    fVar24 = 0.0;
    local_100 = uVar19;
  }
  goto LAB_1411f9220;
}

