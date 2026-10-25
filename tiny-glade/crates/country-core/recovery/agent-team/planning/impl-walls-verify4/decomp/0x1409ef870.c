
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
  int iVar15;
  undefined1 auVar16 [16];
  undefined1 auVar17 [16];
  undefined1 auVar19 [16];
  undefined1 auVar20 [16];
  undefined1 auVar22 [16];
  undefined1 auVar23 [16];
  float in_XMM1_Da;
  float in_XMM1_Db;
  undefined1 auVar24 [16];
  int iVar29;
  undefined1 auVar25 [16];
  undefined1 auVar26 [16];
  undefined1 auVar27 [16];
  undefined1 auVar28 [16];
  float in_XMM2_Da;
  float in_XMM2_Db;
  float fVar30;
  undefined8 in_XMM3_Qa;
  undefined1 auVar31 [16];
  undefined1 auVar32 [16];
  float fVar33;
  float fVar34;
  undefined1 auVar35 [16];
  float fVar36;
  float fVar37;
  float in_stack_00000028;
  float in_stack_00000030;
  longlong *in_stack_00000038;
  undefined1 auVar18 [16];
  undefined1 auVar21 [16];
  
  fVar12 = (float)in_XMM3_Qa;
  fVar13 = (float)((ulonglong)in_XMM3_Qa >> 0x20);
  ::utils::geometry::aabb::is_inside_bbx();
  if ((extraout_AL != '\0') && (::utils::geometry::aabb::is_inside_bbx(), extraout_AL_00 != '\0')) {
    auVar17._0_4_ = in_stack_00000028 - in_XMM2_Da;
    auVar17._4_4_ = fVar12 - in_XMM1_Da;
    auVar17._8_4_ = 0.0 - in_XMM2_Db;
    auVar17._12_4_ = fVar13 - in_XMM1_Db;
    fVar30 = SQRT(auVar17._4_4_ * auVar17._4_4_ + auVar17._0_4_ * auVar17._0_4_);
    if ((fVar30 != 0.0) || (NAN(fVar30))) {
      auVar26._4_4_ = in_stack_00000028;
      auVar26._0_4_ = fVar12;
      auVar26._8_4_ = fVar13;
      auVar26._12_4_ = 0;
      auVar27._4_4_ = in_XMM2_Da;
      auVar27._0_4_ = in_XMM1_Da;
      auVar27._8_4_ = in_XMM1_Db;
      auVar27._12_4_ = in_XMM2_Db;
      auVar27 = minps(auVar26,auVar27);
      uVar6 = -(uint)NAN(in_XMM1_Da);
      uVar10 = -(uint)NAN(in_XMM2_Da);
      fVar14 = (float)(~uVar6 & auVar27._0_4_ | (uint)fVar12 & uVar6);
      fVar33 = (float)(~uVar10 & auVar27._4_4_ | (uint)in_stack_00000028 & uVar10);
      auVar28._4_4_ = fVar30;
      auVar28._0_4_ = fVar30;
      auVar28._8_8_ = 0;
      divps(auVar17,auVar28);
      ::utils::geometry::aabb::Aabb2::from_center_dims();
      uVar1 = *(ulonglong *)(in_RCX + 0x50);
      auVar35._8_8_ = 0;
      auVar35._0_8_ = uVar1;
      fVar37 = (float)(uVar1 >> 0x20);
      fVar36 = (float)uVar1;
      fVar30 = fVar36 / (float)*(int *)(in_RCX + 0x48);
      fVar34 = fVar37 / (float)*(int *)(in_RCX + 0x4c);
      ::utils::geometry::rectangle::Rectangle2d::new();
      iVar4 = -(uint)(fVar36 * -0.5 <= fVar14);
      iVar11 = -(uint)(fVar37 * -0.5 <= fVar33);
      auVar19._4_4_ = iVar11;
      auVar19._0_4_ = iVar4;
      auVar19._8_4_ = iVar11;
      auVar19._12_4_ = iVar11;
      auVar18._8_8_ = auVar19._8_8_;
      auVar18._4_4_ = iVar4;
      auVar18._0_4_ = iVar4;
      iVar4 = movmskpd(extraout_EAX,auVar18);
      if (iVar4 != 3) {
                    // WARNING: Subroutine does not return
        core::panicking::panic();
      }
      auVar20._4_4_ = in_stack_00000028;
      auVar20._0_4_ = fVar12;
      auVar20._8_4_ = fVar13;
      auVar20._12_4_ = 0;
      auVar3._4_4_ = in_XMM2_Da;
      auVar3._0_4_ = in_XMM1_Da;
      auVar3._8_4_ = in_XMM1_Db;
      auVar3._12_4_ = in_XMM2_Db;
      auVar17 = maxps(auVar20,auVar3);
      fVar12 = (float)((uint)fVar12 & uVar6 | ~uVar6 & auVar17._0_4_);
      fVar13 = (float)((uint)in_stack_00000028 & uVar10 | ~uVar10 & auVar17._4_4_);
      fVar36 = fVar36 * 0.5;
      fVar37 = fVar37 * 0.5;
      iVar4 = -(uint)(fVar12 <= fVar36);
      iVar11 = -(uint)(fVar13 <= fVar37);
      auVar22._4_4_ = iVar11;
      auVar22._0_4_ = iVar4;
      auVar22._8_4_ = iVar11;
      auVar22._12_4_ = iVar11;
      auVar21._8_8_ = auVar22._8_8_;
      auVar21._4_4_ = iVar4;
      auVar21._0_4_ = iVar4;
      iVar4 = movmskpd(3,auVar21);
      if (iVar4 != 3) {
                    // WARNING: Subroutine does not return
        core::panicking::panic();
      }
      if (fVar30 <= 0.5) {
                    // WARNING: Subroutine does not return
        core::panicking::panic();
      }
      fVar14 = floorf(((fVar14 - in_stack_00000030) + fVar36) / fVar30);
      iVar4 = (int)fVar14;
      if (2.1474835e+09 < fVar14) {
        iVar4 = 0x7fffffff;
      }
      if (NAN(fVar14)) {
        iVar4 = 0;
      }
      fVar14 = floorf(((fVar33 - in_stack_00000030) + fVar37) / fVar34);
      iVar11 = (int)fVar14;
      if (2.1474835e+09 < fVar14) {
        iVar11 = 0x7fffffff;
      }
      if (NAN(fVar14)) {
        iVar11 = 0;
      }
      if (iVar4 < 1) {
        iVar4 = 0;
      }
      if (iVar11 < 1) {
        iVar11 = 0;
      }
      fVar12 = floorf((fVar12 + in_stack_00000030 + fVar36) / fVar30);
      iVar29 = (int)fVar12;
      if (2.1474835e+09 < fVar12) {
        iVar29 = 0x7fffffff;
      }
      if (NAN(fVar12)) {
        iVar29 = 0;
      }
      fVar12 = floorf((fVar13 + in_stack_00000030 + fVar37) / fVar34);
      iVar8 = (int)fVar12;
      if (2.1474835e+09 < fVar12) {
        iVar8 = 0x7fffffff;
      }
      if (NAN(fVar12)) {
        iVar8 = 0;
      }
      iVar15 = (int)*(undefined8 *)(in_RCX + 0x48);
      iVar9 = (int)((ulonglong)*(undefined8 *)(in_RCX + 0x48) >> 0x20);
      iVar7 = iVar9 + -1;
      if (iVar15 + -1 < iVar29) {
        iVar29 = iVar15 + -1;
      }
      if (iVar8 <= iVar7) {
        iVar7 = iVar8;
      }
      if (iVar11 <= iVar7) {
        if (iVar29 < iVar4) {
          do {
            if (iVar7 <= iVar11) {
              return;
            }
            iVar11 = iVar11 + (uint)(iVar11 < iVar7);
          } while (iVar11 <= iVar7);
        }
        else {
          if (iVar15 == 0) {
                    // WARNING: Subroutine does not return
            core::panicking::panic_const::panic_const_rem_by_zero();
          }
          auVar32._0_4_ = (float)iVar15;
          auVar32._4_4_ = (float)iVar9;
          auVar32._8_8_ = 0;
          divps(auVar35,auVar32);
          do {
            iVar8 = iVar4;
            do {
              iVar9 = iVar11 * iVar15 + iVar8;
              if ((iVar15 == -1) && (SBORROW4(0,iVar9))) {
                    // WARNING: Subroutine does not return
                core::panicking::panic_const::panic_const_rem_overflow();
              }
              fVar12 = floorf((float)iVar9 / auVar32._0_4_);
              iVar5 = 0x7fffffff;
              if (fVar12 <= 2.1474835e+09) {
                iVar5 = (int)fVar12;
              }
              if (NAN(fVar12)) {
                iVar5 = 0;
              }
              auVar23._0_4_ = (float)(iVar9 % iVar15);
              auVar23._4_4_ = (float)iVar5;
              auVar23._8_8_ = 0;
              divps(auVar23,auVar32);
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
            } while ((iVar8 < iVar29) && (iVar8 = iVar8 + (uint)(iVar8 < iVar29), iVar8 <= iVar29));
          } while ((iVar11 < iVar7) && (iVar11 = iVar11 + (uint)(iVar11 < iVar7), iVar11 <= iVar7));
        }
      }
    }
    else {
      uVar1 = *(ulonglong *)(in_RCX + 0x50);
      auVar16._8_8_ = 0;
      auVar16._0_8_ = uVar1;
      auVar31._0_4_ = (float)uVar1 * 0.5 + in_XMM1_Da;
      auVar31._4_4_ = (float)(uVar1 >> 0x20) * 0.5 + in_XMM2_Da;
      auVar31._8_4_ = in_XMM1_Db + 0.0;
      auVar31._12_4_ = in_XMM2_Db + 0.0;
      iVar11 = (int)*(undefined8 *)(in_RCX + 0x48);
      iVar29 = (int)((ulonglong)*(undefined8 *)(in_RCX + 0x48) >> 0x20);
      auVar24._0_4_ = (float)iVar11;
      auVar24._4_4_ = (float)iVar29;
      auVar24._8_8_ = 0;
      auVar17 = divps(auVar16,auVar24);
      auVar17 = divps(auVar31,auVar17);
      fVar12 = floorf(auVar17._4_4_);
      fVar13 = floorf(auVar17._0_4_);
      iVar4 = (int)fVar13;
      if (2.1474835e+09 < fVar13) {
        iVar4 = 0x7fffffff;
      }
      if (NAN(fVar13)) {
        iVar4 = 0;
      }
      iVar8 = (int)fVar12;
      if (2.1474835e+09 < fVar12) {
        iVar8 = 0x7fffffff;
      }
      if (NAN(fVar12)) {
        iVar8 = 0;
      }
      if ((((-1 < iVar4) &&
           (auVar25._0_4_ = -(uint)(iVar4 < iVar11), iVar29 = -(uint)(iVar8 < iVar29),
           auVar25._4_4_ = auVar25._0_4_, auVar25._8_4_ = iVar29, auVar25._12_4_ = iVar29,
           uVar6 = movmskpd(iVar8,auVar25), (uVar6 & 2) != 0)) && ((uVar6 & 1) != 0)) &&
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

