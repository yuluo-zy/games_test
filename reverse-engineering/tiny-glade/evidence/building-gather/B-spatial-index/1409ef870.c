
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void __cdecl
country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::
get_cells_from_segment<country_core::systems::wall::wall_state::wall_spatial_hash::impl_4::add_segment::closure_env_0>
          (void)

{
  ulonglong uVar1;
  longlong lVar2;
  undefined1 auVar3 [16];
  char extraout_AL;
  char extraout_AL_00;
  char extraout_AL_01;
  int iVar4;
  undefined4 extraout_EAX;
  int iVar5;
  uint uVar6;
  longlong in_RCX;
  int iVar7;
  int iVar8;
  int iVar9;
  uint uVar10;
  int iVar11;
  float fVar12;
  float fVar13;
  float fVar14;
  float fVar15;
  int iVar16;
  undefined1 auVar17 [16];
  undefined1 auVar18 [16];
  undefined1 auVar20 [16];
  undefined1 auVar21 [16];
  undefined1 auVar23 [16];
  undefined1 auVar24 [16];
  float in_XMM1_Da;
  float in_XMM1_Db;
  undefined1 auVar25 [16];
  int iVar30;
  undefined1 auVar26 [16];
  undefined1 auVar27 [16];
  undefined1 auVar28 [16];
  undefined1 auVar29 [16];
  float in_XMM2_Da;
  float in_XMM2_Db;
  float fVar31;
  undefined8 in_XMM3_Qa;
  undefined1 auVar32 [16];
  undefined1 auVar33 [16];
  float fVar34;
  float fVar35;
  undefined1 auVar36 [16];
  float fVar37;
  float fVar38;
  float in_stack_00000028;
  float in_stack_00000030;
  longlong *in_stack_00000038;
  undefined1 auVar19 [16];
  undefined1 auVar22 [16];
  
  fVar12 = (float)in_XMM3_Qa;
  fVar13 = (float)((ulonglong)in_XMM3_Qa >> 0x20);
  ::utils::geometry::aabb::is_inside_bbx();
  if ((extraout_AL != '\0') && (::utils::geometry::aabb::is_inside_bbx(), extraout_AL_00 != '\0')) {
    auVar18._0_4_ = in_stack_00000028 - in_XMM2_Da;
    auVar18._4_4_ = fVar12 - in_XMM1_Da;
    auVar18._8_4_ = 0.0 - in_XMM2_Db;
    auVar18._12_4_ = fVar13 - in_XMM1_Db;
    fVar31 = SQRT(auVar18._4_4_ * auVar18._4_4_ + auVar18._0_4_ * auVar18._0_4_);
    if ((fVar31 != 0.0) || (NAN(fVar31))) {
      auVar27._4_4_ = in_stack_00000028;
      auVar27._0_4_ = fVar12;
      auVar27._8_4_ = fVar13;
      auVar27._12_4_ = 0;
      auVar28._4_4_ = in_XMM2_Da;
      auVar28._0_4_ = in_XMM1_Da;
      auVar28._8_4_ = in_XMM1_Db;
      auVar28._12_4_ = in_XMM2_Db;
      auVar28 = minps(auVar27,auVar28);
      uVar6 = -(uint)NAN(in_XMM1_Da);
      uVar10 = -(uint)NAN(in_XMM2_Da);
      fVar34 = (float)(~uVar6 & auVar28._0_4_ | (uint)fVar12 & uVar6);
      fVar15 = (float)(~uVar10 & auVar28._4_4_ | (uint)in_stack_00000028 & uVar10);
      auVar29._4_4_ = fVar31;
      auVar29._0_4_ = fVar31;
      auVar29._8_8_ = 0;
      divps(auVar18,auVar29);
      ::utils::geometry::aabb::Aabb2::from_center_dims();
      uVar1 = *(ulonglong *)(in_RCX + 0x50);
      auVar36._8_8_ = 0;
      auVar36._0_8_ = uVar1;
      fVar38 = (float)(uVar1 >> 0x20);
      fVar37 = (float)uVar1;
      fVar31 = fVar37 / (float)*(int *)(in_RCX + 0x48);
      fVar35 = fVar38 / (float)*(int *)(in_RCX + 0x4c);
      ::utils::geometry::rectangle::Rectangle2d::new();
      iVar4 = -(uint)(___xmm_0000000000000000bf000000bf000000 * fVar37 <= fVar34);
      iVar11 = -(uint)(_UNK_142a171f4 * fVar38 <= fVar15);
      auVar20._4_4_ = iVar11;
      auVar20._0_4_ = iVar4;
      auVar20._8_4_ = iVar11;
      auVar20._12_4_ = iVar11;
      auVar19._8_8_ = auVar20._8_8_;
      auVar19._4_4_ = iVar4;
      auVar19._0_4_ = iVar4;
      iVar4 = movmskpd(extraout_EAX,auVar19);
      if (iVar4 != 3) {
                    /* WARNING: Subroutine does not return */
        core::panicking::panic();
      }
      auVar21._4_4_ = in_stack_00000028;
      auVar21._0_4_ = fVar12;
      auVar21._8_4_ = fVar13;
      auVar21._12_4_ = 0;
      auVar3._4_4_ = in_XMM2_Da;
      auVar3._0_4_ = in_XMM1_Da;
      auVar3._8_4_ = in_XMM1_Db;
      auVar3._12_4_ = in_XMM2_Db;
      auVar18 = maxps(auVar21,auVar3);
      fVar12 = (float)((uint)fVar12 & uVar6 | ~uVar6 & auVar18._0_4_);
      fVar13 = (float)((uint)in_stack_00000028 & uVar10 | ~uVar10 & auVar18._4_4_);
      fVar37 = fVar37 * (float)__xmm_00000000000000003f0000003f000000;
      fVar38 = fVar38 * __xmm_00000000000000003f0000003f000000._4_4_;
      iVar4 = -(uint)(fVar12 <= fVar37);
      iVar11 = -(uint)(fVar13 <= fVar38);
      auVar23._4_4_ = iVar11;
      auVar23._0_4_ = iVar4;
      auVar23._8_4_ = iVar11;
      auVar23._12_4_ = iVar11;
      auVar22._8_8_ = auVar23._8_8_;
      auVar22._4_4_ = iVar4;
      auVar22._0_4_ = iVar4;
      iVar4 = movmskpd(3,auVar22);
      if (iVar4 != 3) {
                    /* WARNING: Subroutine does not return */
        core::panicking::panic();
      }
      if (fVar31 <= __real_3f000000) {
                    /* WARNING: Subroutine does not return */
        core::panicking::panic();
      }
      fVar14 = floorf(((fVar34 - in_stack_00000030) + fVar37) / fVar31);
      fVar34 = __real_4effffff;
      iVar4 = (int)fVar14;
      if (__real_4effffff < fVar14) {
        iVar4 = 0x7fffffff;
      }
      if (NAN(fVar14)) {
        iVar4 = 0;
      }
      fVar15 = floorf(((fVar15 - in_stack_00000030) + fVar38) / fVar35);
      iVar11 = (int)fVar15;
      if (fVar34 < fVar15) {
        iVar11 = 0x7fffffff;
      }
      if (NAN(fVar15)) {
        iVar11 = 0;
      }
      if (iVar4 < 1) {
        iVar4 = 0;
      }
      if (iVar11 < 1) {
        iVar11 = 0;
      }
      fVar12 = floorf((fVar12 + in_stack_00000030 + fVar37) / fVar31);
      iVar30 = (int)fVar12;
      if (fVar34 < fVar12) {
        iVar30 = 0x7fffffff;
      }
      if (NAN(fVar12)) {
        iVar30 = 0;
      }
      fVar12 = floorf((fVar13 + in_stack_00000030 + fVar38) / fVar35);
      iVar8 = (int)fVar12;
      if (fVar34 < fVar12) {
        iVar8 = 0x7fffffff;
      }
      if (NAN(fVar12)) {
        iVar8 = 0;
      }
      iVar16 = (int)*(undefined8 *)(in_RCX + 0x48);
      iVar9 = (int)((ulonglong)*(undefined8 *)(in_RCX + 0x48) >> 0x20);
      iVar7 = iVar9 + -1;
      if (iVar16 + -1 < iVar30) {
        iVar30 = iVar16 + -1;
      }
      if (iVar8 <= iVar7) {
        iVar7 = iVar8;
      }
      if (iVar11 <= iVar7) {
        if (iVar30 < iVar4) {
          do {
            if (iVar7 <= iVar11) {
              return;
            }
            iVar11 = iVar11 + (uint)(iVar11 < iVar7);
          } while (iVar11 <= iVar7);
        }
        else {
          if (iVar16 == 0) {
                    /* WARNING: Subroutine does not return */
            core::panicking::panic_const::panic_const_rem_by_zero();
          }
          auVar33._0_4_ = (float)iVar16;
          auVar33._4_4_ = (float)iVar9;
          auVar33._8_8_ = 0;
          divps(auVar36,auVar33);
          do {
            iVar8 = iVar4;
            do {
              iVar9 = iVar11 * iVar16 + iVar8;
              if ((iVar16 == -1) && (SBORROW4(0,iVar9))) {
                    /* WARNING: Subroutine does not return */
                core::panicking::panic_const::panic_const_rem_overflow();
              }
              fVar12 = floorf((float)iVar9 / auVar33._0_4_);
              iVar5 = 0x7fffffff;
              if (fVar12 <= fVar34) {
                iVar5 = (int)fVar12;
              }
              if (NAN(fVar12)) {
                iVar5 = 0;
              }
              auVar24._0_4_ = (float)(iVar9 % iVar16);
              auVar24._4_4_ = (float)iVar5;
              auVar24._8_8_ = 0;
              divps(auVar24,auVar33);
              ::utils::geometry::rectangle::Rectangle2d::aabb();
              ::utils::geometry::aabb::Aabb2::intersects();
              if (extraout_AL_01 != '\0') {
                lVar2 = in_stack_00000038[2];
                if (lVar2 == *in_stack_00000038) {
                  alloc::raw_vec::RawVec::grow_one<usize,alloc::alloc::Global>();
                }
                *(longlong *)(in_stack_00000038[1] + lVar2 * 8) = (longlong)iVar9;
                in_stack_00000038[2] = lVar2 + 1;
              }
            } while ((iVar8 < iVar30) && (iVar8 = iVar8 + (uint)(iVar8 < iVar30), iVar8 <= iVar30));
          } while ((iVar11 < iVar7) && (iVar11 = iVar11 + (uint)(iVar11 < iVar7), iVar11 <= iVar7));
        }
      }
    }
    else {
      uVar1 = *(ulonglong *)(in_RCX + 0x50);
      auVar17._8_8_ = 0;
      auVar17._0_8_ = uVar1;
      auVar32._0_4_ = (float)__xmm_00000000000000003f0000003f000000 * (float)uVar1 + in_XMM1_Da;
      auVar32._4_4_ =
           __xmm_00000000000000003f0000003f000000._4_4_ * (float)(uVar1 >> 0x20) + in_XMM2_Da;
      auVar32._8_4_ = _UNK_1429258e8 * 0.0 + in_XMM1_Db;
      auVar32._12_4_ = _UNK_1429258ec * 0.0 + in_XMM2_Db;
      iVar11 = (int)*(undefined8 *)(in_RCX + 0x48);
      iVar30 = (int)((ulonglong)*(undefined8 *)(in_RCX + 0x48) >> 0x20);
      auVar25._0_4_ = (float)iVar11;
      auVar25._4_4_ = (float)iVar30;
      auVar25._8_8_ = 0;
      auVar18 = divps(auVar17,auVar25);
      auVar18 = divps(auVar32,auVar18);
      fVar12 = floorf(auVar18._4_4_);
      fVar13 = floorf(auVar18._0_4_);
      iVar4 = (int)fVar13;
      if (__real_4effffff < fVar13) {
        iVar4 = 0x7fffffff;
      }
      if (NAN(fVar13)) {
        iVar4 = 0;
      }
      iVar8 = (int)fVar12;
      if (__real_4effffff < fVar12) {
        iVar8 = 0x7fffffff;
      }
      if (NAN(fVar12)) {
        iVar8 = 0;
      }
      if ((((-1 < iVar4) &&
           (auVar26._0_4_ = -(uint)(iVar4 < iVar11), iVar30 = -(uint)(iVar8 < iVar30),
           auVar26._4_4_ = auVar26._0_4_, auVar26._8_4_ = iVar30, auVar26._12_4_ = iVar30,
           uVar6 = movmskpd(iVar8,auVar26), (uVar6 & 2) != 0)) && ((uVar6 & 1) != 0)) &&
         (-1 < iVar8)) {
        lVar2 = in_stack_00000038[2];
        if (lVar2 == *in_stack_00000038) {
          alloc::raw_vec::RawVec::grow_one<usize,alloc::alloc::Global>();
        }
        *(longlong *)(in_stack_00000038[1] + lVar2 * 8) = (longlong)(iVar8 * iVar11 + iVar4);
        in_stack_00000038[2] = lVar2 + 1;
      }
    }
  }
  return;
}

