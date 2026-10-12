
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void __cdecl
country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::
get_cells_from_segment<country_core::systems::wall::wall_state::wall_spatial_hash::impl_4::query_with_segment::closure_env_0>
          (void)

{
  longlong *plVar1;
  longlong lVar2;
  longlong lVar3;
  longlong lVar4;
  longlong lVar5;
  undefined1 auVar6 [16];
  longlong lVar7;
  char extraout_AL;
  char extraout_AL_00;
  char extraout_AL_01;
  int iVar8;
  undefined4 extraout_EAX;
  int iVar9;
  int iVar10;
  longlong lVar11;
  longlong lVar12;
  int iVar13;
  uint uVar14;
  longlong in_RCX;
  ulonglong uVar15;
  longlong lVar16;
  int iVar17;
  ulonglong uVar18;
  longlong lVar19;
  int iVar20;
  ulonglong uVar21;
  ulonglong uVar22;
  longlong lVar23;
  ulonglong uVar24;
  uint uVar25;
  int iVar26;
  float fVar27;
  float fVar28;
  float fVar29;
  int iVar30;
  undefined1 auVar31 [16];
  undefined1 auVar32 [16];
  undefined1 auVar34 [16];
  undefined1 auVar35 [16];
  undefined1 auVar37 [16];
  undefined1 auVar38 [16];
  undefined1 auVar39 [16];
  float in_XMM1_Da;
  float in_XMM1_Db;
  undefined1 auVar40 [16];
  int iVar45;
  undefined1 auVar41 [16];
  undefined1 auVar42 [16];
  undefined1 auVar43 [16];
  undefined1 auVar44 [16];
  float in_XMM2_Da;
  float in_XMM2_Db;
  undefined1 auVar46 [16];
  float in_XMM3_Da;
  float fVar47;
  float in_XMM3_Db;
  undefined1 auVar48 [16];
  undefined1 auVar49 [16];
  float fVar50;
  float fVar51;
  float fVar52;
  undefined1 auVar53 [16];
  float fVar54;
  float fVar55;
  float in_stack_00000028;
  float in_stack_00000030;
  longlong *in_stack_00000038;
  longlong in_stack_00000040;
  undefined1 auVar33 [16];
  undefined1 auVar36 [16];
  
  ::utils::geometry::aabb::is_inside_bbx();
  if (extraout_AL == '\0') {
    return;
  }
  ::utils::geometry::aabb::is_inside_bbx();
  if (extraout_AL_00 == '\0') {
    return;
  }
  auVar32._0_4_ = in_stack_00000028 - in_XMM2_Da;
  auVar32._4_4_ = in_XMM3_Da - in_XMM1_Da;
  auVar32._8_4_ = 0.0 - in_XMM2_Db;
  auVar32._12_4_ = in_XMM3_Db - in_XMM1_Db;
  fVar47 = SQRT(auVar32._4_4_ * auVar32._4_4_ + auVar32._0_4_ * auVar32._0_4_);
  if ((fVar47 != 0.0) || (NAN(fVar47))) {
    auVar42._4_4_ = in_stack_00000028;
    auVar42._0_4_ = in_XMM3_Da;
    auVar42._8_4_ = in_XMM3_Db;
    auVar42._12_4_ = 0;
    auVar43._4_4_ = in_XMM2_Da;
    auVar43._0_4_ = in_XMM1_Da;
    auVar43._8_4_ = in_XMM1_Db;
    auVar43._12_4_ = in_XMM2_Db;
    auVar43 = minps(auVar42,auVar43);
    uVar14 = -(uint)NAN(in_XMM1_Da);
    uVar25 = -(uint)NAN(in_XMM2_Da);
    fVar27 = (float)(~uVar14 & auVar43._0_4_ | (uint)in_XMM3_Da & uVar14);
    fVar29 = (float)(~uVar25 & auVar43._4_4_ | (uint)in_stack_00000028 & uVar25);
    auVar44._4_4_ = fVar47;
    auVar44._0_4_ = fVar47;
    auVar44._8_8_ = 0;
    divps(auVar32,auVar44);
    ::utils::geometry::aabb::Aabb2::from_center_dims();
    uVar15 = *(ulonglong *)(in_RCX + 0x50);
    auVar53._8_8_ = 0;
    auVar53._0_8_ = uVar15;
    fVar55 = (float)(uVar15 >> 0x20);
    fVar54 = (float)uVar15;
    fVar47 = fVar54 / (float)*(int *)(in_RCX + 0x48);
    fVar52 = fVar55 / (float)*(int *)(in_RCX + 0x4c);
    ::utils::geometry::rectangle::Rectangle2d::new();
    iVar8 = -(uint)(___xmm_0000000000000000bf000000bf000000 * fVar54 <= fVar27);
    iVar26 = -(uint)(_UNK_142a171f4 * fVar55 <= fVar29);
    auVar34._4_4_ = iVar26;
    auVar34._0_4_ = iVar8;
    auVar34._8_4_ = iVar26;
    auVar34._12_4_ = iVar26;
    auVar33._8_8_ = auVar34._8_8_;
    auVar33._4_4_ = iVar8;
    auVar33._0_4_ = iVar8;
    iVar8 = movmskpd(extraout_EAX,auVar33);
    if (iVar8 != 3) {
                    /* WARNING: Subroutine does not return */
      core::panicking::panic();
    }
    auVar35._4_4_ = in_stack_00000028;
    auVar35._0_4_ = in_XMM3_Da;
    auVar35._8_4_ = in_XMM3_Db;
    auVar35._12_4_ = 0;
    auVar6._4_4_ = in_XMM2_Da;
    auVar6._0_4_ = in_XMM1_Da;
    auVar6._8_4_ = in_XMM1_Db;
    auVar6._12_4_ = in_XMM2_Db;
    auVar32 = maxps(auVar35,auVar6);
    fVar50 = (float)((uint)in_XMM3_Da & uVar14 | ~uVar14 & auVar32._0_4_);
    fVar51 = (float)((uint)in_stack_00000028 & uVar25 | ~uVar25 & auVar32._4_4_);
    fVar54 = fVar54 * (float)__xmm_00000000000000003f0000003f000000;
    fVar55 = fVar55 * __xmm_00000000000000003f0000003f000000._4_4_;
    iVar8 = -(uint)(fVar50 <= fVar54);
    iVar26 = -(uint)(fVar51 <= fVar55);
    auVar37._4_4_ = iVar26;
    auVar37._0_4_ = iVar8;
    auVar37._8_4_ = iVar26;
    auVar37._12_4_ = iVar26;
    auVar36._8_8_ = auVar37._8_8_;
    auVar36._4_4_ = iVar8;
    auVar36._0_4_ = iVar8;
    iVar8 = movmskpd(3,auVar36);
    if (iVar8 != 3) {
                    /* WARNING: Subroutine does not return */
      core::panicking::panic();
    }
    if (fVar47 <= __real_3f000000) {
                    /* WARNING: Subroutine does not return */
      core::panicking::panic();
    }
    fVar28 = floorf(((fVar27 - in_stack_00000030) + fVar54) / fVar47);
    fVar27 = __real_4effffff;
    iVar8 = (int)fVar28;
    if (__real_4effffff < fVar28) {
      iVar8 = 0x7fffffff;
    }
    iVar26 = 0;
    if (NAN(fVar28)) {
      iVar8 = iVar26;
    }
    fVar29 = floorf(((fVar29 - in_stack_00000030) + fVar55) / fVar52);
    iVar45 = (int)fVar29;
    if (fVar27 < fVar29) {
      iVar45 = 0x7fffffff;
    }
    if (NAN(fVar29)) {
      iVar45 = iVar26;
    }
    if (iVar8 < 1) {
      iVar8 = iVar26;
    }
    if (iVar45 < 1) {
      iVar45 = 0;
    }
    fVar47 = floorf((fVar50 + in_stack_00000030 + fVar54) / fVar47);
    iVar13 = (int)fVar47;
    if (fVar27 < fVar47) {
      iVar13 = 0x7fffffff;
    }
    if (NAN(fVar47)) {
      iVar13 = iVar26;
    }
    fVar47 = floorf((fVar51 + in_stack_00000030 + fVar55) / fVar52);
    lVar7 = _UNK_142ac4d48;
    lVar3 = ___xmm_00000000000000a80000000000000070;
    lVar16 = _UNK_142ac4d38;
    lVar2 = ___xmm_00000000000000380000000000000000;
    iVar26 = (int)fVar47;
    if (fVar27 < fVar47) {
      iVar26 = 0x7fffffff;
    }
    if (NAN(fVar47)) {
      iVar26 = 0;
    }
    iVar30 = (int)*(undefined8 *)(in_RCX + 0x48);
    iVar9 = (int)((ulonglong)*(undefined8 *)(in_RCX + 0x48) >> 0x20);
    iVar17 = iVar9 + -1;
    if (iVar30 + -1 < iVar13) {
      iVar13 = iVar30 + -1;
    }
    if (iVar26 <= iVar17) {
      iVar17 = iVar26;
    }
    if (iVar17 < iVar45) {
      return;
    }
    if (iVar13 < iVar8) {
      do {
        if (iVar17 <= iVar45) {
          return;
        }
        iVar45 = iVar45 + (uint)(iVar45 < iVar17);
      } while (iVar45 <= iVar17);
      return;
    }
    if (iVar30 == 0) {
                    /* WARNING: Subroutine does not return */
      core::panicking::panic_const::panic_const_rem_by_zero();
    }
    lVar4 = *(longlong *)(in_stack_00000040 + 8);
    uVar15 = *(ulonglong *)(in_stack_00000040 + 0x10);
    auVar49._0_4_ = (float)iVar30;
    auVar49._4_4_ = (float)iVar9;
    auVar49._8_8_ = 0;
    divps(auVar53,auVar49);
    do {
      iVar9 = (uint)(iVar45 < iVar17) + iVar45;
      iVar26 = iVar8;
      do {
        iVar20 = iVar45 * iVar30 + iVar26;
        if ((iVar30 == -1) && (SBORROW4(0,iVar20))) {
                    /* WARNING: Subroutine does not return */
          core::panicking::panic_const::panic_const_rem_overflow();
        }
        fVar47 = floorf((float)iVar20 / auVar49._0_4_);
        iVar10 = (int)fVar47;
        if (fVar27 < fVar47) {
          iVar10 = 0x7fffffff;
        }
        if (NAN(fVar47)) {
          iVar10 = 0;
        }
        auVar38._0_4_ = (float)(iVar20 % iVar30);
        auVar38._4_4_ = (float)iVar10;
        auVar38._8_8_ = 0;
        divps(auVar38,auVar49);
        ::utils::geometry::rectangle::Rectangle2d::aabb();
        ::utils::geometry::aabb::Aabb2::intersects();
        if (extraout_AL_01 != '\0') {
          if (uVar15 <= (ulonglong)(longlong)iVar20) {
                    /* WARNING: Subroutine does not return */
            core::panicking::panic_bounds_check();
          }
          lVar11 = (longlong)iVar20 * 0x30;
          lVar5 = *(longlong *)(lVar4 + 8 + lVar11);
          uVar18 = *(ulonglong *)(lVar4 + 0x10 + lVar11);
          lVar11 = in_stack_00000038[2];
          if ((ulonglong)(*in_stack_00000038 - lVar11) < uVar18) {
            alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
            lVar11 = in_stack_00000038[2];
LAB_1409f0628:
            lVar12 = in_stack_00000038[1];
            uVar21 = uVar18 * 0x38 - 0x38;
            lVar19 = lVar5;
            if (0xa7 < uVar21) {
              uVar21 = uVar21 / 0x38 + 1;
              uVar22 = uVar21 & 0xfffffffffffffffc;
              lVar19 = lVar12 + lVar11 * 8;
              lVar11 = lVar11 + uVar22;
              uVar24 = 0;
              lVar23 = lVar5;
              do {
                auVar39._8_4_ = (int)lVar23;
                auVar39._0_8_ = lVar23;
                auVar39._12_4_ = (int)((ulonglong)lVar23 >> 0x20);
                plVar1 = (longlong *)(lVar19 + uVar24 * 8);
                *plVar1 = lVar23 + lVar2;
                plVar1[1] = auVar39._8_8_ + lVar16;
                plVar1 = (longlong *)(lVar19 + 0x10 + uVar24 * 8);
                *plVar1 = lVar23 + lVar3;
                plVar1[1] = auVar39._8_8_ + lVar7;
                uVar24 = uVar24 + 4;
                lVar23 = lVar23 + 0xe0;
              } while (uVar22 != uVar24);
              lVar19 = uVar22 * 0x38 + lVar5;
              if (uVar21 == uVar22) goto LAB_1409f06e0;
            }
            do {
              *(longlong *)(lVar12 + lVar11 * 8) = lVar19;
              lVar19 = lVar19 + 0x38;
              lVar11 = lVar11 + 1;
            } while (lVar19 != lVar5 + uVar18 * 0x38);
          }
          else if (uVar18 != 0) goto LAB_1409f0628;
LAB_1409f06e0:
          in_stack_00000038[2] = lVar11;
        }
      } while ((iVar26 < iVar13) && (iVar26 = iVar26 + (uint)(iVar26 < iVar13), iVar26 <= iVar13));
      if (iVar17 <= iVar45) {
        return;
      }
      iVar45 = iVar9;
      if (iVar17 < iVar9) {
        return;
      }
    } while( true );
  }
  uVar15 = *(ulonglong *)(in_RCX + 0x50);
  auVar31._8_8_ = 0;
  auVar31._0_8_ = uVar15;
  auVar48._0_4_ = (float)__xmm_00000000000000003f0000003f000000 * (float)uVar15 + in_XMM1_Da;
  auVar48._4_4_ =
       __xmm_00000000000000003f0000003f000000._4_4_ * (float)(uVar15 >> 0x20) + in_XMM2_Da;
  auVar48._8_4_ = _UNK_1429258e8 * 0.0 + in_XMM1_Db;
  auVar48._12_4_ = _UNK_1429258ec * 0.0 + in_XMM2_Db;
  iVar26 = (int)*(undefined8 *)(in_RCX + 0x48);
  iVar45 = (int)((ulonglong)*(undefined8 *)(in_RCX + 0x48) >> 0x20);
  auVar40._0_4_ = (float)iVar26;
  auVar40._4_4_ = (float)iVar45;
  auVar40._8_8_ = 0;
  auVar32 = divps(auVar31,auVar40);
  auVar32 = divps(auVar48,auVar32);
  fVar47 = floorf(auVar32._4_4_);
  fVar27 = floorf(auVar32._0_4_);
  iVar8 = (int)fVar27;
  if (__real_4effffff < fVar27) {
    iVar8 = 0x7fffffff;
  }
  if (NAN(fVar27)) {
    iVar8 = 0;
  }
  iVar13 = (int)fVar47;
  if (__real_4effffff < fVar47) {
    iVar13 = 0x7fffffff;
  }
  if (NAN(fVar47)) {
    iVar13 = 0;
  }
  if (iVar8 < 0) {
    return;
  }
  auVar41._0_4_ = -(uint)(iVar8 < iVar26);
  iVar45 = -(uint)(iVar13 < iVar45);
  auVar41._4_4_ = auVar41._0_4_;
  auVar41._8_4_ = iVar45;
  auVar41._12_4_ = iVar45;
  uVar14 = movmskpd(iVar13,auVar41);
  if ((uVar14 & 2) == 0) {
    return;
  }
  if ((uVar14 & 1) == 0) {
    return;
  }
  if (iVar13 < 0) {
    return;
  }
  uVar15 = (ulonglong)(iVar13 * iVar26 + iVar8);
  if (*(ulonglong *)(in_stack_00000040 + 0x10) <= uVar15) {
                    /* WARNING: Subroutine does not return */
    core::panicking::panic_bounds_check();
  }
  lVar16 = uVar15 * 0x30;
  lVar2 = *(longlong *)(*(longlong *)(in_stack_00000040 + 8) + 8 + lVar16);
  uVar15 = *(ulonglong *)(*(longlong *)(in_stack_00000040 + 8) + 0x10 + lVar16);
  lVar16 = in_stack_00000038[2];
  if ((ulonglong)(*in_stack_00000038 - lVar16) < uVar15) {
    alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
    lVar16 = in_stack_00000038[2];
  }
  else if (uVar15 == 0) goto LAB_1409f07d0;
  lVar11 = _UNK_142ac4d48;
  lVar5 = ___xmm_00000000000000a80000000000000070;
  lVar4 = _UNK_142ac4d38;
  lVar7 = ___xmm_00000000000000380000000000000000;
  lVar3 = in_stack_00000038[1];
  uVar18 = uVar15 * 0x38 - 0x38;
  lVar12 = lVar2;
  if (0xa7 < uVar18) {
    uVar18 = uVar18 / 0x38 + 1;
    uVar22 = uVar18 & 0xfffffffffffffffc;
    lVar12 = lVar3 + lVar16 * 8;
    lVar16 = lVar16 + uVar22;
    uVar21 = 0;
    lVar19 = lVar2;
    do {
      auVar46._8_4_ = (int)lVar19;
      auVar46._0_8_ = lVar19;
      auVar46._12_4_ = (int)((ulonglong)lVar19 >> 0x20);
      plVar1 = (longlong *)(lVar12 + uVar21 * 8);
      *plVar1 = lVar19 + lVar7;
      plVar1[1] = auVar46._8_8_ + lVar4;
      plVar1 = (longlong *)(lVar12 + 0x10 + uVar21 * 8);
      *plVar1 = lVar19 + lVar5;
      plVar1[1] = auVar46._8_8_ + lVar11;
      uVar21 = uVar21 + 4;
      lVar19 = lVar19 + 0xe0;
    } while (uVar22 != uVar21);
    lVar12 = uVar22 * 0x38 + lVar2;
    if (uVar18 == uVar22) goto LAB_1409f07d0;
  }
  do {
    *(longlong *)(lVar3 + lVar16 * 8) = lVar12;
    lVar12 = lVar12 + 0x38;
    lVar16 = lVar16 + 1;
  } while (lVar12 != lVar2 + uVar15 * 0x38);
LAB_1409f07d0:
  in_stack_00000038[2] = lVar16;
  return;
}

