
void __cdecl
utils::curve::Curve::
slice_into<glam::f32::vec2::Vec2,utils::curve::CurveCoordSemantic2D,alloc::vec::Vec<glam::f32::vec2::Vec2,alloc::alloc::Global>_>
          (void)

{
  undefined8 *puVar1;
  undefined8 *puVar2;
  undefined8 uVar3;
  undefined8 uVar4;
  longlong lVar5;
  undefined8 uVar6;
  uint extraout_EAX;
  uint extraout_EAX_00;
  longlong lVar7;
  char cVar8;
  longlong in_RCX;
  ulonglong uVar9;
  ulonglong uVar10;
  longlong lVar11;
  undefined1 *puVar12;
  ulonglong uVar13;
  ulonglong uVar14;
  longlong lVar15;
  longlong *in_R9;
  longlong lVar16;
  ulonglong uVar17;
  undefined1 *puVar18;
  ulonglong uVar19;
  float extraout_XMM0_Da;
  float extraout_XMM0_Da_00;
  float in_XMM1_Da;
  float fVar20;
  float fVar21;
  float in_XMM2_Da;
  float fVar22;
  float fVar23;
  float fVar24;
  float fVar25;
  float fVar26;
  undefined1 local_c8 [4];
  undefined1 local_c4 [4];
  undefined1 *local_c0;
  undefined1 *local_b8;
  code *local_b0;
  undefined1 *local_a8;
  code *local_a0;
  undefined **local_98;
  undefined8 local_90;
  undefined1 **local_88;
  undefined8 local_80;
  undefined8 local_78;
  
  if (in_XMM2_Da <= in_XMM1_Da) {
    local_b8 = local_c8;
    local_b0 = curve_u::impl$8::fmt<utils::curve::CurveCoordSemantic2D>;
    local_a8 = local_c4;
    local_a0 = curve_u::impl$8::fmt<utils::curve::CurveCoordSemantic2D>;
    local_98 = &PTR_s_u_start_must_be_smaller_than_u_e_142c18c80;
    local_90 = 2;
    local_78 = 0;
    local_88 = &local_b8;
    local_80 = 2;
                    /* WARNING: Subroutine does not return */
    core::panicking::panic_fmt();
  }
  get_coord_at_u<glam::f32::vec3::Vec3,utils::curve::CurveCoordSemantic3D,utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic3D>_>
            ();
  get_coord_at_u<glam::f32::vec3::Vec3,utils::curve::CurveCoordSemantic3D,utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic3D>_>
            ();
  uVar17 = *(ulonglong *)(in_RCX + 0x10);
  uVar14 = (ulonglong)extraout_EAX;
  if (uVar17 <= uVar14) {
                    /* WARNING: Subroutine does not return */
    core::panicking::panic_bounds_check();
  }
  uVar10 = uVar14 + 1;
  if (uVar17 <= uVar10) {
                    /* WARNING: Subroutine does not return */
    core::panicking::panic_bounds_check();
  }
  uVar9 = (ulonglong)extraout_EAX_00;
  if (uVar17 <= uVar9) {
                    /* WARNING: Subroutine does not return */
    core::panicking::panic_bounds_check();
  }
  if (uVar17 <= uVar9 + 1) {
                    /* WARNING: Subroutine does not return */
    core::panicking::panic_bounds_check();
  }
  lVar5 = *(longlong *)(in_RCX + 8);
  uVar3 = *(undefined8 *)(lVar5 + uVar14 * 8);
  uVar4 = *(undefined8 *)(lVar5 + 8 + uVar14 * 8);
  fVar22 = (float)uVar4;
  fVar23 = (float)((ulonglong)uVar4 >> 0x20);
  fVar25 = extraout_XMM0_Da * fVar22 + (__real_3f800000 - extraout_XMM0_Da) * (float)uVar3;
  fVar26 = extraout_XMM0_Da * fVar23 +
           (__real_3f800000 - extraout_XMM0_Da) * (float)((ulonglong)uVar3 >> 0x20);
  uVar3 = *(undefined8 *)(lVar5 + uVar9 * 8);
  fVar20 = (float)uVar3;
  fVar21 = (float)((ulonglong)uVar3 >> 0x20);
  uVar3 = *(undefined8 *)(lVar5 + 8 + uVar9 * 8);
  uVar14 = uVar14 + 2;
  if (fVar26 != fVar23) {
    uVar14 = uVar10;
  }
  if (fVar25 != fVar22) {
    uVar14 = uVar10;
  }
  fVar23 = extraout_XMM0_Da_00 * (float)uVar3 + (__real_3f800000 - extraout_XMM0_Da_00) * fVar20;
  fVar24 = extraout_XMM0_Da_00 * (float)((ulonglong)uVar3 >> 0x20) +
           (__real_3f800000 - extraout_XMM0_Da_00) * fVar21;
  if (NAN(fVar25) || NAN(fVar22)) {
    uVar14 = uVar10;
  }
  uVar10 = uVar9 - 1;
  if (uVar9 == 0) {
    uVar10 = 0;
  }
  if (fVar24 != fVar21) {
    uVar10 = uVar9;
  }
  if (fVar23 != fVar20) {
    uVar10 = uVar9;
  }
  if (NAN(fVar23) || NAN(fVar20)) {
    uVar10 = uVar9;
  }
  uVar9 = uVar10 + 1;
  uVar13 = uVar9 - uVar14;
  if (uVar9 < uVar14) {
    uVar13 = 0;
  }
  puVar12 = (undefined1 *)(uVar13 + 2);
  lVar11 = in_R9[2];
  local_c0 = puVar12;
  if ((undefined1 *)(*in_R9 - lVar11) < puVar12) {
    alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
    lVar11 = in_R9[2];
    if (*in_R9 == lVar11) goto LAB_1411f843a;
  }
  else if (*in_R9 == lVar11) {
LAB_1411f843a:
    alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
    lVar11 = in_R9[2];
  }
  lVar7 = in_R9[1];
  *(ulonglong *)(lVar7 + lVar11 * 8) = CONCAT44(fVar26,fVar25);
  lVar11 = lVar11 + 1;
  in_R9[2] = lVar11;
  uVar13 = uVar10 - uVar14;
  cVar8 = -1;
  if (uVar10 < uVar14 || uVar13 == 0) {
    cVar8 = uVar13 != 0;
  }
  if (cVar8 == '\0') {
    if (uVar17 <= uVar14) {
                    /* WARNING: Subroutine does not return */
      core::panicking::panic_bounds_check();
    }
    uVar3 = *(undefined8 *)(lVar5 + uVar14 * 8);
    if (*in_R9 == lVar11) {
      alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
      lVar7 = in_R9[1];
      lVar11 = in_R9[2];
    }
    *(undefined8 *)(lVar7 + lVar11 * 8) = uVar3;
    lVar11 = lVar11 + 1;
    in_R9[2] = lVar11;
    puVar18 = (undefined1 *)0x3;
    goto LAB_1411f835b;
  }
  puVar18 = (undefined1 *)0x2;
  if (cVar8 == '\x01') goto LAB_1411f835b;
  uVar19 = uVar9 - uVar14;
  if (uVar9 < uVar14) {
                    /* WARNING: Subroutine does not return */
    core::slice::index::slice_index_order_fail();
  }
  if (uVar17 <= uVar10) {
                    /* WARNING: Subroutine does not return */
    core::slice::index::slice_end_index_len_fail();
  }
  if ((ulonglong)(*in_R9 - lVar11) < uVar19) {
    alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
    lVar7 = in_R9[1];
    lVar11 = in_R9[2];
    if (uVar19 < 0x14) goto LAB_1411f8305;
LAB_1411f8546:
    uVar17 = lVar7 + lVar11 * 8;
    uVar10 = 0;
    if (((uVar17 <= uVar17 + uVar13 * 8) && (lVar7 + lVar11 * 8 + 4U <= uVar17 + uVar13 * 8 + 4)) &&
       (uVar13 >> 0x3d == 0)) {
      lVar16 = lVar11 * 8;
      uVar10 = uVar19 & 0x1fffffffc;
      lVar11 = lVar11 + uVar10;
      lVar15 = uVar14 * 8 + 0x10 + lVar5;
      lVar16 = lVar16 + lVar7;
      uVar17 = 0;
      do {
        puVar2 = (undefined8 *)(lVar15 + -0x10 + uVar17 * 8);
        uVar3 = puVar2[1];
        puVar1 = (undefined8 *)(lVar15 + uVar17 * 8);
        uVar4 = *puVar1;
        uVar6 = puVar1[1];
        puVar1 = (undefined8 *)(lVar16 + uVar17 * 8);
        *puVar1 = *puVar2;
        puVar1[1] = uVar3;
        puVar1 = (undefined8 *)(lVar16 + 0x10 + uVar17 * 8);
        *puVar1 = uVar4;
        puVar1[1] = uVar6;
        uVar17 = uVar17 + 4;
      } while (uVar10 != uVar17);
      if (uVar19 == uVar10) goto LAB_1411f8327;
    }
LAB_1411f8307:
    uVar14 = uVar14 + uVar10;
    do {
      *(undefined8 *)(lVar7 + lVar11 * 8) = *(undefined8 *)(lVar5 + uVar14 * 8);
      lVar11 = lVar11 + 1;
      uVar14 = uVar14 + 1;
    } while (uVar9 != uVar14);
  }
  else if (uVar9 != uVar14) {
    if (0x13 < uVar19) goto LAB_1411f8546;
LAB_1411f8305:
    uVar10 = 0;
    goto LAB_1411f8307;
  }
LAB_1411f8327:
  in_R9[2] = lVar11;
  puVar18 = (undefined1 *)(uVar19 + 2);
LAB_1411f835b:
  local_b8 = puVar18;
  if (*in_R9 == lVar11) {
    alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
    lVar7 = in_R9[1];
    lVar11 = in_R9[2];
  }
  *(ulonglong *)(lVar7 + lVar11 * 8) = CONCAT44(fVar24,fVar23);
  in_R9[2] = lVar11 + 1;
  if (puVar18 != puVar12) {
    local_98 = (undefined **)0x0;
                    /* WARNING: Subroutine does not return */
    core::panicking::assert_failed<usize,usize>();
  }
  return;
}

