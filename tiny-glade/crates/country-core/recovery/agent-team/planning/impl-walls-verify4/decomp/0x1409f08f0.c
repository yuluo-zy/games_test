
void __cdecl
country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::
get_cells_from_circle<country_core::systems::wall::wall_state::wall_spatial_hash::impl_4::query_with_circle::closure_env_0>
          (void)

{
  longlong *plVar1;
  ulonglong uVar2;
  longlong lVar3;
  longlong lVar4;
  ulonglong uVar5;
  longlong lVar6;
  undefined4 extraout_EAX;
  int iVar7;
  int iVar8;
  longlong lVar9;
  longlong lVar10;
  longlong in_RCX;
  int iVar11;
  undefined8 *in_RDX;
  ulonglong uVar12;
  ulonglong uVar13;
  int iVar14;
  longlong *in_R8;
  longlong in_R9;
  ulonglong uVar15;
  int iVar16;
  longlong lVar17;
  int iVar18;
  int iVar19;
  uint uVar20;
  int iVar21;
  float fVar22;
  int iVar23;
  float fVar27;
  undefined1 auVar24 [16];
  undefined1 auVar25 [16];
  undefined1 auVar26 [16];
  float fVar28;
  float fVar33;
  undefined1 auVar29 [16];
  undefined1 auVar30 [16];
  undefined1 auVar32 [16];
  undefined1 auVar34 [16];
  undefined1 auVar35 [16];
  undefined1 auVar36 [16];
  undefined1 auVar37 [16];
  undefined1 auVar38 [16];
  float fVar39;
  float fVar40;
  float fVar41;
  float fVar42;
  ulonglong local_d8;
  ulonglong local_d0;
  undefined1 auVar31 [16];
  
  uVar2 = *(ulonglong *)(in_RCX + 0x50);
  auVar38._8_8_ = 0;
  auVar38._0_8_ = uVar2;
  ::utils::geometry::circle::Circle2d::aabb();
  auVar35._8_8_ = 0;
  auVar35._0_8_ = local_d8;
  fVar41 = (float)uVar2;
  fVar28 = fVar41 * -0.5;
  fVar42 = (float)(uVar2 >> 0x20);
  fVar33 = fVar42 * -0.5;
  auVar34._4_4_ = fVar33;
  auVar34._0_4_ = fVar28;
  auVar34._8_8_ = 0;
  auVar35 = maxps(auVar34,auVar35);
  uVar20 = -(uint)NAN((float)(local_d8 >> 0x20));
  fVar22 = (float)(~-(uint)NAN((float)local_d8) & auVar35._0_4_ |
                  (uint)fVar28 & -(uint)NAN((float)local_d8));
  fVar27 = (float)(~uVar20 & auVar35._4_4_ | (uint)fVar33 & uVar20);
  iVar19 = -(uint)(fVar28 <= fVar22);
  iVar21 = -(uint)(fVar33 <= fVar27);
  auVar29._4_4_ = iVar21;
  auVar29._0_4_ = iVar19;
  auVar29._8_4_ = iVar21;
  auVar29._12_4_ = iVar21;
  auVar25._8_8_ = auVar29._8_8_;
  auVar25._4_4_ = iVar19;
  auVar25._0_4_ = iVar19;
  iVar19 = movmskpd(extraout_EAX,auVar25);
  if (iVar19 != 3) {
                    // WARNING: Subroutine does not return
    core::panicking::panic();
  }
  fVar39 = fVar41 * 0.5;
  fVar40 = fVar42 * 0.5;
  auVar36._8_8_ = 0;
  auVar36._0_8_ = local_d0;
  auVar30._4_4_ = fVar40;
  auVar30._0_4_ = fVar39;
  auVar30._8_8_ = 0;
  auVar35 = minps(auVar30,auVar36);
  uVar20 = -(uint)NAN((float)(local_d0 >> 0x20));
  fVar28 = (float)(~-(uint)NAN((float)local_d0) & auVar35._0_4_ |
                  (uint)fVar39 & -(uint)NAN((float)local_d0));
  fVar33 = (float)(~uVar20 & auVar35._4_4_ | (uint)fVar40 & uVar20);
  iVar19 = -(uint)(fVar28 <= fVar39);
  iVar21 = -(uint)(fVar33 <= fVar40);
  auVar32._4_4_ = iVar21;
  auVar32._0_4_ = iVar19;
  auVar32._8_4_ = iVar21;
  auVar32._12_4_ = iVar21;
  auVar31._8_8_ = auVar32._8_8_;
  auVar31._4_4_ = iVar19;
  auVar31._0_4_ = iVar19;
  iVar19 = movmskpd(3,auVar31);
  if (iVar19 != 3) {
                    // WARNING: Subroutine does not return
    core::panicking::panic();
  }
  fVar41 = fVar41 / (float)*(int *)(in_RCX + 0x48);
  if (fVar41 <= 0.5) {
                    // WARNING: Subroutine does not return
    core::panicking::panic();
  }
  fVar42 = fVar42 / (float)*(int *)(in_RCX + 0x4c);
  fVar22 = floorf((fVar22 + fVar39) / fVar41);
  iVar19 = (int)fVar22;
  if (2.1474835e+09 < fVar22) {
    iVar19 = 0x7fffffff;
  }
  if (NAN(fVar22)) {
    iVar19 = 0;
  }
  fVar22 = floorf((fVar27 + fVar40) / fVar42);
  iVar21 = (int)fVar22;
  if (2.1474835e+09 < fVar22) {
    iVar21 = 0x7fffffff;
  }
  if (NAN(fVar22)) {
    iVar21 = 0;
  }
  if (iVar19 < 1) {
    iVar19 = 0;
  }
  if (iVar21 < 1) {
    iVar21 = 0;
  }
  fVar22 = floorf((fVar28 + 0.0 + fVar39) / fVar41);
  iVar16 = (int)fVar22;
  if (2.1474835e+09 < fVar22) {
    iVar16 = 0x7fffffff;
  }
  if (NAN(fVar22)) {
    iVar16 = 0;
  }
  fVar22 = floorf((fVar33 + 0.0 + fVar40) / fVar42);
  iVar18 = (int)fVar22;
  if (2.1474835e+09 < fVar22) {
    iVar18 = 0x7fffffff;
  }
  if (NAN(fVar22)) {
    iVar18 = 0;
  }
  iVar23 = (int)*(undefined8 *)(in_RCX + 0x48);
  iVar7 = (int)((ulonglong)*(undefined8 *)(in_RCX + 0x48) >> 0x20);
  iVar11 = iVar7 + -1;
  if (iVar23 + -1 < iVar16) {
    iVar16 = iVar23 + -1;
  }
  if (iVar18 <= iVar11) {
    iVar11 = iVar18;
  }
  if (iVar21 <= iVar11) {
    if (iVar16 < iVar19) {
      do {
        if (iVar11 <= iVar21) {
          return;
        }
        iVar21 = iVar21 + (uint)(iVar21 < iVar11);
      } while (iVar21 <= iVar11);
    }
    else {
      if (iVar23 == 0) {
                    // WARNING: Subroutine does not return
        core::panicking::panic_const::panic_const_rem_by_zero();
      }
      lVar3 = *(longlong *)(in_R9 + 8);
      uVar2 = *(ulonglong *)(in_R9 + 0x10);
      auVar37._0_4_ = (float)iVar23;
      auVar37._4_4_ = (float)iVar7;
      auVar37._8_8_ = 0;
      auVar35 = divps(auVar38,auVar37);
      do {
        iVar7 = (uint)(iVar21 < iVar11) + iVar21;
        iVar18 = iVar19;
        do {
          iVar14 = iVar21 * iVar23 + iVar18;
          if ((iVar23 == -1) && (SBORROW4(0,iVar14))) {
                    // WARNING: Subroutine does not return
            core::panicking::panic_const::panic_const_rem_overflow();
          }
          fVar22 = floorf((float)iVar14 / auVar37._0_4_);
          iVar8 = (int)fVar22;
          if (2.1474835e+09 < fVar22) {
            iVar8 = 0x7fffffff;
          }
          if (NAN(fVar22)) {
            iVar8 = 0;
          }
          auVar24._0_4_ = (float)(iVar14 % iVar23);
          auVar24._4_4_ = (float)iVar8;
          auVar24._8_8_ = 0;
          auVar25 = divps(auVar24,auVar37);
          fVar22 = ((auVar25._0_4_ * fVar39 - (1.0 - auVar25._0_4_) * fVar39) + auVar35._0_4_ * 0.5)
                   - (float)*in_RDX;
          fVar27 = ((auVar25._4_4_ * fVar40 - (1.0 - auVar25._4_4_) * fVar40) + auVar35._4_4_ * 0.5)
                   - (float)((ulonglong)*in_RDX >> 0x20);
          if (SQRT(fVar27 * fVar27 + fVar22 * fVar22) <
              *(float *)(in_RDX + 1) + fVar41 * 0.5 * 1.4142135) {
            if (uVar2 <= (ulonglong)(longlong)iVar14) {
                    // WARNING: Subroutine does not return
              core::panicking::panic_bounds_check();
            }
            lVar9 = (longlong)iVar14 * 0x30;
            lVar4 = *(longlong *)(lVar3 + 8 + lVar9);
            uVar5 = *(ulonglong *)(lVar3 + 0x10 + lVar9);
            lVar9 = in_R8[2];
            if ((ulonglong)(*in_R8 - lVar9) < uVar5) {
              alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
              lVar9 = in_R8[2];
LAB_1409f0cf0:
              lVar6 = in_R8[1];
              uVar12 = uVar5 * 0x38 - 0x38;
              lVar10 = lVar4;
              if (0xa7 < uVar12) {
                uVar12 = uVar12 / 0x38 + 1;
                uVar15 = uVar12 & 0xfffffffffffffffc;
                lVar10 = lVar6 + lVar9 * 8;
                lVar9 = lVar9 + uVar15;
                uVar13 = 0;
                lVar17 = lVar4;
                do {
                  auVar26._8_4_ = (int)lVar17;
                  auVar26._0_8_ = lVar17;
                  auVar26._12_4_ = (int)((ulonglong)lVar17 >> 0x20);
                  plVar1 = (longlong *)(lVar10 + uVar13 * 8);
                  *plVar1 = lVar17;
                  plVar1[1] = auVar26._8_8_ + 0x38;
                  plVar1 = (longlong *)(lVar10 + 0x10 + uVar13 * 8);
                  *plVar1 = lVar17 + 0x70;
                  plVar1[1] = auVar26._8_8_ + 0xa8;
                  uVar13 = uVar13 + 4;
                  lVar17 = lVar17 + 0xe0;
                } while (uVar15 != uVar13);
                lVar10 = uVar15 * 0x38 + lVar4;
                if (uVar12 == uVar15) goto LAB_1409f0da0;
              }
              do {
                *(longlong *)(lVar6 + lVar9 * 8) = lVar10;
                lVar10 = lVar10 + 0x38;
                lVar9 = lVar9 + 1;
              } while (lVar10 != lVar4 + uVar5 * 0x38);
            }
            else if (uVar5 != 0) goto LAB_1409f0cf0;
LAB_1409f0da0:
            in_R8[2] = lVar9;
          }
        } while ((iVar18 < iVar16) && (iVar18 = iVar18 + (uint)(iVar18 < iVar16), iVar18 <= iVar16))
        ;
      } while ((iVar21 < iVar11) && (iVar21 = iVar7, iVar7 <= iVar11));
    }
  }
  return;
}

