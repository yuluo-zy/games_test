// get_cells_from_segment<country_core::systems::wall::wall_state::wall_spatial_hash::impl$4::query_with_segment::closure_env$0>  size=2403
// void __cdecl get_cells_from_segment<country_core::systems::wall::wall_state::wall_spatial_hash::impl$4::query_with_segment::closure_env$0>(void)

void __cdecl
country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::
get_cells_from_segment<country_core::systems::wall::wall_state::wall_spatial_hash::impl_4::query_with_segment::closure_env_0>
          (void)

{
  longlong *plVar1;
  longlong lVar2;
  undefined1 auVar3 [16];
  char extraout_AL;
  char extraout_AL_00;
  char extraout_AL_01;
  int iVar4;
  undefined4 extraout_EAX;
  int iVar5;
  int iVar6;
  longlong lVar7;
  longlong lVar8;
  int iVar9;
  uint uVar10;
  longlong in_RCX;
  ulonglong uVar11;
  longlong lVar12;
  int iVar13;
  ulonglong uVar14;
  longlong lVar15;
  int iVar16;
  ulonglong uVar17;
  ulonglong uVar18;
  longlong lVar19;
  ulonglong uVar20;
  uint uVar21;
  int iVar22;
  float fVar23;
  int iVar24;
  undefined1 auVar25 [16];
  undefined1 auVar26 [16];
  undefined1 auVar28 [16];
  undefined1 auVar29 [16];
  undefined1 auVar31 [16];
  undefined1 auVar32 [16];
  undefined1 auVar33 [16];
  float in_XMM1_Da;
  float in_XMM1_Db;
  undefined1 auVar34 [16];
  int iVar39;
  undefined1 auVar35 [16];
  undefined1 auVar36 [16];
  undefined1 auVar37 [16];
  undefined1 auVar38 [16];
  float in_XMM2_Da;
  float in_XMM2_Db;
  undefined1 auVar40 [16];
  float in_XMM3_Da;
  float fVar41;
  float in_XMM3_Db;
  undefined1 auVar42 [16];
  undefined1 auVar43 [16];
  float fVar44;
  float fVar45;
  float fVar46;
  float fVar47;
  undefined1 auVar48 [16];
  float fVar49;
  float fVar50;
  float in_stack_00000028;
  float in_stack_00000030;
  longlong *in_stack_00000038;
  longlong in_stack_00000040;
  undefined1 auVar27 [16];
  undefined1 auVar30 [16];
  
  ::utils::geometry::aabb::is_inside_bbx();
  if (extraout_AL == '\0') {
    return;
  }
  ::utils::geometry::aabb::is_inside_bbx();
  if (extraout_AL_00 == '\0') {
    return;
  }
  auVar26._0_4_ = in_stack_00000028 - in_XMM2_Da;
  auVar26._4_4_ = in_XMM3_Da - in_XMM1_Da;
  auVar26._8_4_ = 0.0 - in_XMM2_Db;
  auVar26._12_4_ = in_XMM3_Db - in_XMM1_Db;
  fVar41 = SQRT(auVar26._4_4_ * auVar26._4_4_ + auVar26._0_4_ * auVar26._0_4_);
  if ((fVar41 != 0.0) || (NAN(fVar41))) {
    auVar36._4_4_ = in_stack_00000028;
    auVar36._0_4_ = in_XMM3_Da;
    auVar36._8_4_ = in_XMM3_Db;
    auVar36._12_4_ = 0;
    auVar37._4_4_ = in_XMM2_Da;
    auVar37._0_4_ = in_XMM1_Da;
    auVar37._8_4_ = in_XMM1_Db;
    auVar37._12_4_ = in_XMM2_Db;
    auVar37 = minps(auVar36,auVar37);
    uVar10 = -(uint)NAN(in_XMM1_Da);
    uVar21 = -(uint)NAN(in_XMM2_Da);
    fVar23 = (float)(~uVar10 & auVar37._0_4_ | (uint)in_XMM3_Da & uVar10);
    fVar46 = (float)(~uVar21 & auVar37._4_4_ | (uint)in_stack_00000028 & uVar21);
    auVar38._4_4_ = fVar41;
    auVar38._0_4_ = fVar41;
    auVar38._8_8_ = 0;
    divps(auVar26,auVar38);
    ::utils::geometry::aabb::Aabb2::from_center_dims();
    uVar11 = *(ulonglong *)(in_RCX + 0x50);
    auVar48._8_8_ = 0;
    auVar48._0_8_ = uVar11;
    fVar50 = (float)(uVar11 >> 0x20);
    fVar49 = (float)uVar11;
    fVar41 = fVar49 / (float)*(int *)(in_RCX + 0x48);
    fVar47 = fVar50 / (float)*(int *)(in_RCX + 0x4c);
    ::utils::geometry::rectangle::Rectangle2d::new();
    iVar4 = -(uint)(fVar49 * -0.5 <= fVar23);
    iVar22 = -(uint)(fVar50 * -0.5 <= fVar46);
    auVar28._4_4_ = iVar22;
    auVar28._0_4_ = iVar4;
    auVar28._8_4_ = iVar22;
    auVar28._12_4_ = iVar22;
    auVar27._8_8_ = auVar28._8_8_;
    auVar27._4_4_ = iVar4;
    auVar27._0_4_ = iVar4;
    iVar4 = movmskpd(extraout_EAX,auVar27);
    if (iVar4 != 3) {
                    // WARNING: Subroutine does not return
      core::panicking::panic();
    }
    auVar29._4_4_ = in_stack_00000028;
    auVar29._0_4_ = in_XMM3_Da;
    auVar29._8_4_ = in_XMM3_Db;
    auVar29._12_4_ = 0;
    auVar3._4_4_ = in_XMM2_Da;
    auVar3._0_4_ = in_XMM1_Da;
    auVar3._8_4_ = in_XMM1_Db;
    auVar3._12_4_ = in_XMM2_Db;
    auVar26 = maxps(auVar29,auVar3);
    fVar44 = (float)((uint)in_XMM3_Da & uVar10 | ~uVar10 & auVar26._0_4_);
    fVar45 = (float)((uint)in_stack_00000028 & uVar21 | ~uVar21 & auVar26._4_4_);
    fVar49 = fVar49 * 0.5;
    fVar50 = fVar50 * 0.5;
    iVar4 = -(uint)(fVar44 <= fVar49);
    iVar22 = -(uint)(fVar45 <= fVar50);
    auVar31._4_4_ = iVar22;
    auVar31._0_4_ = iVar4;
    auVar31._8_4_ = iVar22;
    auVar31._12_4_ = iVar22;
    auVar30._8_8_ = auVar31._8_8_;
    auVar30._4_4_ = iVar4;
    auVar30._0_4_ = iVar4;
    iVar4 = movmskpd(3,auVar30);
    if (iVar4 != 3) {
                    // WARNING: Subroutine does not return
      core::panicking::panic();
    }
    if (fVar41 <= 0.5) {
                    // WARNING: Subroutine does not return
      core::panicking::panic();
    }
    fVar23 = floorf(((fVar23 - in_stack_00000030) + fVar49) / fVar41);
    iVar4 = (int)fVar23;
    if (2.1474835e+09 < fVar23) {
      iVar4 = 0x7fffffff;
    }
    iVar22 = 0;
    if (NAN(fVar23)) {
      iVar4 = iVar22;
    }
    fVar23 = floorf(((fVar46 - in_stack_00000030) + fVar50) / fVar47);
    iVar39 = (int)fVar23;
    if (2.1474835e+09 < fVar23) {
      iVar39 = 0x7fffffff;
    }
    if (NAN(fVar23)) {
      iVar39 = iVar22;
    }
    if (iVar4 < 1) {
      iVar4 = iVar22;
    }
    if (iVar39 < 1) {
      iVar39 = 0;
    }
    fVar41 = floorf((fVar44 + in_stack_00000030 + fVar49) / fVar41);
    iVar9 = (int)fVar41;
    if (2.1474835e+09 < fVar41) {
      iVar9 = 0x7fffffff;
    }
    if (NAN(fVar41)) {
      iVar9 = iVar22;
    }
    fVar41 = floorf((fVar45 + in_stack_00000030 + fVar50) / fVar47);
    iVar22 = (int)fVar41;
    if (2.1474835e+09 < fVar41) {
      iVar22 = 0x7fffffff;
    }
    if (NAN(fVar41)) {
      iVar22 = 0;
    }
    iVar24 = (int)*(undefined8 *)(in_RCX + 0x48);
    iVar5 = (int)((ulonglong)*(undefined8 *)(in_RCX + 0x48) >> 0x20);
    iVar13 = iVar5 + -1;
    if (iVar24 + -1 < iVar9) {
      iVar9 = iVar24 + -1;
    }
    if (iVar22 <= iVar13) {
      iVar13 = iVar22;
    }
    if (iVar13 < iVar39) {
      return;
    }
    if (iVar9 < iVar4) {
      do {
        if (iVar13 <= iVar39) {
          return;
        }
        iVar39 = iVar39 + (uint)(iVar39 < iVar13);
      } while (iVar39 <= iVar13);
      return;
    }
    if (iVar24 == 0) {
                    // WARNING: Subroutine does not return
      core::panicking::panic_const::panic_const_rem_by_zero();
    }
    lVar2 = *(longlong *)(in_stack_00000040 + 8);
    uVar11 = *(ulonglong *)(in_stack_00000040 + 0x10);
    auVar43._0_4_ = (float)iVar24;
    auVar43._4_4_ = (float)iVar5;
    auVar43._8_8_ = 0;
    divps(auVar48,auVar43);
    do {
      iVar5 = (uint)(iVar39 < iVar13) + iVar39;
      iVar22 = iVar4;
      do {
        iVar16 = iVar39 * iVar24 + iVar22;
        if ((iVar24 == -1) && (SBORROW4(0,iVar16))) {
                    // WARNING: Subroutine does not return
          core::panicking::panic_const::panic_const_rem_overflow();
        }
        fVar41 = floorf((float)iVar16 / auVar43._0_4_);
        iVar6 = (int)fVar41;
        if (2.1474835e+09 < fVar41) {
          iVar6 = 0x7fffffff;
        }
        if (NAN(fVar41)) {
          iVar6 = 0;
        }
        auVar32._0_4_ = (float)(iVar16 % iVar24);
        auVar32._4_4_ = (float)iVar6;
        auVar32._8_8_ = 0;
        divps(auVar32,auVar43);
        ::utils::geometry::rectangle::Rectangle2d::aabb();
        ::utils::geometry::aabb::Aabb2::intersects();
        if (extraout_AL_01 != '\0') {
          if (uVar11 <= (ulonglong)(longlong)iVar16) {
                    // WARNING: Subroutine does not return
            core::panicking::panic_bounds_check();
          }
          lVar7 = (longlong)iVar16 * 0x30;
          lVar12 = *(longlong *)(lVar2 + 8 + lVar7);
          uVar14 = *(ulonglong *)(lVar2 + 0x10 + lVar7);
          lVar7 = in_stack_00000038[2];
          if ((ulonglong)(*in_stack_00000038 - lVar7) < uVar14) {
            alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
            lVar7 = in_stack_00000038[2];
LAB_1409f0628:
            lVar8 = in_stack_00000038[1];
            uVar17 = uVar14 * 0x38 - 0x38;
            lVar15 = lVar12;
            if (0xa7 < uVar17) {
              uVar17 = uVar17 / 0x38 + 1;
              uVar18 = uVar17 & 0xfffffffffffffffc;
              lVar15 = lVar8 + lVar7 * 8;
              lVar7 = lVar7 + uVar18;
              uVar20 = 0;
              lVar19 = lVar12;
              do {
                auVar33._8_4_ = (int)lVar19;
                auVar33._0_8_ = lVar19;
                auVar33._12_4_ = (int)((ulonglong)lVar19 >> 0x20);
                plVar1 = (longlong *)(lVar15 + uVar20 * 8);
                *plVar1 = lVar19;
                plVar1[1] = auVar33._8_8_ + 0x38;
                plVar1 = (longlong *)(lVar15 + 0x10 + uVar20 * 8);
                *plVar1 = lVar19 + 0x70;
                plVar1[1] = auVar33._8_8_ + 0xa8;
                uVar20 = uVar20 + 4;
                lVar19 = lVar19 + 0xe0;
              } while (uVar18 != uVar20);
              lVar15 = uVar18 * 0x38 + lVar12;
              if (uVar17 == uVar18) goto LAB_1409f06e0;
            }
            do {
              *(longlong *)(lVar8 + lVar7 * 8) = lVar15;
              lVar15 = lVar15 + 0x38;
              lVar7 = lVar7 + 1;
            } while (lVar15 != lVar12 + uVar14 * 0x38);
          }
          else if (uVar14 != 0) goto LAB_1409f0628;
LAB_1409f06e0:
          in_stack_00000038[2] = lVar7;
        }
      } while ((iVar22 < iVar9) && (iVar22 = iVar22 + (uint)(iVar22 < iVar9), iVar22 <= iVar9));
      if (iVar13 <= iVar39) {
        return;
      }
      iVar39 = iVar5;
      if (iVar13 < iVar5) {
        return;
      }
    } while( true );
  }
  uVar11 = *(ulonglong *)(in_RCX + 0x50);
  auVar25._8_8_ = 0;
  auVar25._0_8_ = uVar11;
  auVar42._0_4_ = (float)uVar11 * 0.5 + in_XMM1_Da;
  auVar42._4_4_ = (float)(uVar11 >> 0x20) * 0.5 + in_XMM2_Da;
  auVar42._8_4_ = in_XMM1_Db + 0.0;
  auVar42._12_4_ = in_XMM2_Db + 0.0;
  iVar22 = (int)*(undefined8 *)(in_RCX + 0x48);
  iVar39 = (int)((ulonglong)*(undefined8 *)(in_RCX + 0x48) >> 0x20);
  auVar34._0_4_ = (float)iVar22;
  auVar34._4_4_ = (float)iVar39;
  auVar34._8_8_ = 0;
  auVar26 = divps(auVar25,auVar34);
  auVar26 = divps(auVar42,auVar26);
  fVar41 = floorf(auVar26._4_4_);
  fVar23 = floorf(auVar26._0_4_);
  iVar4 = (int)fVar23;
  if (2.1474835e+09 < fVar23) {
    iVar4 = 0x7fffffff;
  }
  if (NAN(fVar23)) {
    iVar4 = 0;
  }
  iVar9 = (int)fVar41;
  if (2.1474835e+09 < fVar41) {
    iVar9 = 0x7fffffff;
  }
  if (NAN(fVar41)) {
    iVar9 = 0;
  }
  if (iVar4 < 0) {
    return;
  }
  auVar35._0_4_ = -(uint)(iVar4 < iVar22);
  iVar39 = -(uint)(iVar9 < iVar39);
  auVar35._4_4_ = auVar35._0_4_;
  auVar35._8_4_ = iVar39;
  auVar35._12_4_ = iVar39;
  uVar10 = movmskpd(iVar9,auVar35);
  if ((uVar10 & 2) == 0) {
    return;
  }
  if ((uVar10 & 1) == 0) {
    return;
  }
  if (iVar9 < 0) {
    return;
  }
  uVar11 = (ulonglong)(iVar9 * iVar22 + iVar4);
  if (*(ulonglong *)(in_stack_00000040 + 0x10) <= uVar11) {
                    // WARNING: Subroutine does not return
    core::panicking::panic_bounds_check();
  }
  lVar12 = uVar11 * 0x30;
  lVar2 = *(longlong *)(*(longlong *)(in_stack_00000040 + 8) + 8 + lVar12);
  uVar11 = *(ulonglong *)(*(longlong *)(in_stack_00000040 + 8) + 0x10 + lVar12);
  lVar12 = in_stack_00000038[2];
  if ((ulonglong)(*in_stack_00000038 - lVar12) < uVar11) {
    alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
    lVar12 = in_stack_00000038[2];
  }
  else if (uVar11 == 0) goto LAB_1409f07d0;
  lVar7 = in_stack_00000038[1];
  uVar14 = uVar11 * 0x38 - 0x38;
  lVar8 = lVar2;
  if (0xa7 < uVar14) {
    uVar14 = uVar14 / 0x38 + 1;
    uVar18 = uVar14 & 0xfffffffffffffffc;
    lVar8 = lVar7 + lVar12 * 8;
    lVar12 = lVar12 + uVar18;
    uVar17 = 0;
    lVar15 = lVar2;
    do {
      auVar40._8_4_ = (int)lVar15;
      auVar40._0_8_ = lVar15;
      auVar40._12_4_ = (int)((ulonglong)lVar15 >> 0x20);
      plVar1 = (longlong *)(lVar8 + uVar17 * 8);
      *plVar1 = lVar15;
      plVar1[1] = auVar40._8_8_ + 0x38;
      plVar1 = (longlong *)(lVar8 + 0x10 + uVar17 * 8);
      *plVar1 = lVar15 + 0x70;
      plVar1[1] = auVar40._8_8_ + 0xa8;
      uVar17 = uVar17 + 4;
      lVar15 = lVar15 + 0xe0;
    } while (uVar18 != uVar17);
    lVar8 = uVar18 * 0x38 + lVar2;
    if (uVar14 == uVar18) goto LAB_1409f07d0;
  }
  do {
    *(longlong *)(lVar7 + lVar12 * 8) = lVar8;
    lVar8 = lVar8 + 0x38;
    lVar12 = lVar12 + 1;
  } while (lVar8 != lVar2 + uVar11 * 0x38);
LAB_1409f07d0:
  in_stack_00000038[2] = lVar12;
  return;
}



// CALLEES: [{'name': 'aabb', 'address': '140c9b680'}, {'name': 'do_reserve_and_handle<alloc::alloc::Global>', 'address': '1428a9a60'}, {'name': 'floorf', 'address': '142923ee0'}, {'name': 'from_center_dims', 'address': '14123fff0'}, {'name': 'intersects', 'address': '141240170'}, {'name': 'is_inside_bbx', 'address': '14123ffc0'}, {'name': 'new', 'address': '140c98980'}, {'name': 'panic', 'address': '1428d9430'}, {'name': 'panic_bounds_check', 'address': '1428d9518'}, {'name': 'panic_const_rem_by_zero', 'address': '1428d9a20'}, {'name': 'panic_const_rem_overflow', 'address': '1428d99a0'}]