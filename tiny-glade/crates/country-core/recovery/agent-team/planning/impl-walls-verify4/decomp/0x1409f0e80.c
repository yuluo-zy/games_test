
void __cdecl
country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::
get_cells_from_circle<country_core::systems::wall::wall_state::spatial_hash_debug::spatial_hash_debug::closure_env_2>
          (void)

{
  ulonglong uVar1;
  longlong lVar2;
  undefined4 extraout_EAX;
  int iVar3;
  longlong in_RCX;
  int iVar4;
  undefined8 *in_RDX;
  int iVar5;
  int iVar6;
  longlong *in_R8;
  int iVar7;
  int iVar8;
  uint uVar9;
  int iVar10;
  float fVar11;
  int iVar12;
  float fVar15;
  undefined1 auVar13 [16];
  undefined1 auVar14 [16];
  float fVar16;
  float fVar21;
  undefined1 auVar17 [16];
  undefined1 auVar18 [16];
  undefined1 auVar20 [16];
  undefined1 auVar22 [16];
  undefined1 auVar23 [16];
  undefined1 auVar24 [16];
  undefined1 auVar25 [16];
  undefined1 auVar26 [16];
  float fVar27;
  float fVar28;
  float fVar29;
  float fVar30;
  ulonglong local_d8;
  ulonglong local_d0;
  undefined1 auVar19 [16];
  
  uVar1 = *(ulonglong *)(in_RCX + 0x50);
  auVar26._8_8_ = 0;
  auVar26._0_8_ = uVar1;
  ::utils::geometry::circle::Circle2d::aabb();
  auVar23._8_8_ = 0;
  auVar23._0_8_ = local_d8;
  fVar29 = (float)uVar1;
  fVar16 = fVar29 * -0.5;
  fVar30 = (float)(uVar1 >> 0x20);
  fVar21 = fVar30 * -0.5;
  auVar22._4_4_ = fVar21;
  auVar22._0_4_ = fVar16;
  auVar22._8_8_ = 0;
  auVar23 = maxps(auVar22,auVar23);
  uVar9 = -(uint)NAN((float)(local_d8 >> 0x20));
  fVar11 = (float)(~-(uint)NAN((float)local_d8) & auVar23._0_4_ |
                  (uint)fVar16 & -(uint)NAN((float)local_d8));
  fVar15 = (float)(~uVar9 & auVar23._4_4_ | (uint)fVar21 & uVar9);
  iVar8 = -(uint)(fVar16 <= fVar11);
  iVar10 = -(uint)(fVar21 <= fVar15);
  auVar17._4_4_ = iVar10;
  auVar17._0_4_ = iVar8;
  auVar17._8_4_ = iVar10;
  auVar17._12_4_ = iVar10;
  auVar14._8_8_ = auVar17._8_8_;
  auVar14._4_4_ = iVar8;
  auVar14._0_4_ = iVar8;
  iVar8 = movmskpd(extraout_EAX,auVar14);
  if (iVar8 != 3) {
                    // WARNING: Subroutine does not return
    core::panicking::panic();
  }
  fVar27 = fVar29 * 0.5;
  fVar28 = fVar30 * 0.5;
  auVar24._8_8_ = 0;
  auVar24._0_8_ = local_d0;
  auVar18._4_4_ = fVar28;
  auVar18._0_4_ = fVar27;
  auVar18._8_8_ = 0;
  auVar23 = minps(auVar18,auVar24);
  uVar9 = -(uint)NAN((float)(local_d0 >> 0x20));
  fVar16 = (float)(~-(uint)NAN((float)local_d0) & auVar23._0_4_ |
                  (uint)fVar27 & -(uint)NAN((float)local_d0));
  fVar21 = (float)(~uVar9 & auVar23._4_4_ | (uint)fVar28 & uVar9);
  iVar8 = -(uint)(fVar16 <= fVar27);
  iVar10 = -(uint)(fVar21 <= fVar28);
  auVar20._4_4_ = iVar10;
  auVar20._0_4_ = iVar8;
  auVar20._8_4_ = iVar10;
  auVar20._12_4_ = iVar10;
  auVar19._8_8_ = auVar20._8_8_;
  auVar19._4_4_ = iVar8;
  auVar19._0_4_ = iVar8;
  iVar8 = movmskpd(3,auVar19);
  if (iVar8 == 3) {
    fVar29 = fVar29 / (float)*(int *)(in_RCX + 0x48);
    if (fVar29 <= 0.5) {
                    // WARNING: Subroutine does not return
      core::panicking::panic();
    }
    fVar30 = fVar30 / (float)*(int *)(in_RCX + 0x4c);
    fVar11 = floorf((fVar11 + fVar27) / fVar29);
    iVar8 = (int)fVar11;
    if (2.1474835e+09 < fVar11) {
      iVar8 = 0x7fffffff;
    }
    iVar10 = 0;
    if (NAN(fVar11)) {
      iVar8 = iVar10;
    }
    fVar11 = floorf((fVar15 + fVar28) / fVar30);
    iVar5 = (int)fVar11;
    if (2.1474835e+09 < fVar11) {
      iVar5 = 0x7fffffff;
    }
    if (NAN(fVar11)) {
      iVar5 = iVar10;
    }
    if (iVar8 < 1) {
      iVar8 = iVar10;
    }
    if (iVar5 < 1) {
      iVar5 = 0;
    }
    fVar11 = floorf((fVar16 + 0.0 + fVar27) / fVar29);
    iVar7 = (int)fVar11;
    if (2.1474835e+09 < fVar11) {
      iVar7 = 0x7fffffff;
    }
    if (NAN(fVar11)) {
      iVar7 = iVar10;
    }
    fVar11 = floorf((fVar21 + 0.0 + fVar28) / fVar30);
    iVar10 = (int)fVar11;
    if (2.1474835e+09 < fVar11) {
      iVar10 = 0x7fffffff;
    }
    if (NAN(fVar11)) {
      iVar10 = 0;
    }
    iVar12 = (int)*(undefined8 *)(in_RCX + 0x48);
    iVar6 = (int)((ulonglong)*(undefined8 *)(in_RCX + 0x48) >> 0x20);
    iVar4 = iVar6 + -1;
    if (iVar12 + -1 < iVar7) {
      iVar7 = iVar12 + -1;
    }
    if (iVar10 <= iVar4) {
      iVar4 = iVar10;
    }
    if (iVar5 <= iVar4) {
      if (iVar7 < iVar8) {
        do {
          if (iVar4 <= iVar5) {
            return;
          }
          iVar5 = iVar5 + (uint)(iVar5 < iVar4);
        } while (iVar5 <= iVar4);
      }
      else {
        if (iVar12 == 0) {
                    // WARNING: Subroutine does not return
          core::panicking::panic_const::panic_const_rem_by_zero();
        }
        auVar25._0_4_ = (float)iVar12;
        auVar25._4_4_ = (float)iVar6;
        auVar25._8_8_ = 0;
        auVar23 = divps(auVar26,auVar25);
        do {
          iVar10 = iVar8;
          do {
            iVar6 = iVar5 * iVar12 + iVar10;
            if ((iVar12 == -1) && (SBORROW4(0,iVar6))) {
                    // WARNING: Subroutine does not return
              core::panicking::panic_const::panic_const_rem_overflow();
            }
            fVar11 = floorf((float)iVar6 / auVar25._0_4_);
            iVar3 = (int)fVar11;
            if (2.1474835e+09 < fVar11) {
              iVar3 = 0x7fffffff;
            }
            if (NAN(fVar11)) {
              iVar3 = 0;
            }
            auVar13._0_4_ = (float)(iVar6 % iVar12);
            auVar13._4_4_ = (float)iVar3;
            auVar13._8_8_ = 0;
            auVar14 = divps(auVar13,auVar25);
            fVar11 = ((auVar14._0_4_ * fVar27 - (1.0 - auVar14._0_4_) * fVar27) +
                     auVar23._0_4_ * 0.5) - (float)*in_RDX;
            fVar15 = ((auVar14._4_4_ * fVar28 - (1.0 - auVar14._4_4_) * fVar28) +
                     auVar23._4_4_ * 0.5) - (float)((ulonglong)*in_RDX >> 0x20);
            if (SQRT(fVar15 * fVar15 + fVar11 * fVar11) <
                *(float *)(in_RDX + 1) + fVar29 * 0.5 * 1.4142135) {
              lVar2 = in_R8[2];
              if (lVar2 == *in_R8) {
                alloc::raw_vec::RawVec::grow_one<usize,alloc::alloc::Global>();
              }
              *(longlong *)(in_R8[1] + lVar2 * 8) = (longlong)iVar6;
              in_R8[2] = lVar2 + 1;
            }
          } while ((iVar10 < iVar7) && (iVar10 = iVar10 + (uint)(iVar10 < iVar7), iVar10 <= iVar7));
        } while ((iVar5 < iVar4) && (iVar5 = iVar5 + (uint)(iVar5 < iVar4), iVar5 <= iVar4));
      }
    }
    return;
  }
                    // WARNING: Subroutine does not return
  core::panicking::panic();
}

