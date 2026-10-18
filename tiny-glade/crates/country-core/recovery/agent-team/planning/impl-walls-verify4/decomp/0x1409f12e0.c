
void __cdecl
country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::
get_cells_from_aabb<country_core::systems::wall::wall_state::wall_spatial_hash::impl_4::query_with_aabb_clamped::closure_env_0>
          (void)

{
  longlong *plVar1;
  longlong lVar2;
  ulonglong uVar3;
  longlong lVar4;
  longlong lVar5;
  longlong lVar6;
  longlong lVar7;
  int iVar8;
  longlong in_RCX;
  ulonglong uVar10;
  int iVar11;
  ulonglong uVar12;
  longlong lVar13;
  int iVar14;
  ulonglong uVar15;
  ulonglong uVar16;
  int iVar17;
  int iVar18;
  int iVar19;
  float fVar20;
  undefined1 auVar21 [16];
  float in_XMM1_Da;
  float in_XMM2_Da;
  float in_XMM3_Da;
  float fVar22;
  float fVar23;
  float fVar24;
  float fVar25;
  float in_stack_00000028;
  float in_stack_00000030;
  longlong *in_stack_00000038;
  longlong in_stack_00000040;
  int iVar9;
  
  fVar22 = *(float *)(in_RCX + 0x50);
  if ((in_XMM1_Da < fVar22 * -0.5) ||
     (fVar23 = *(float *)(in_RCX + 0x54), in_XMM2_Da < fVar23 * -0.5)) {
                    // WARNING: Subroutine does not return
    core::panicking::panic();
  }
  fVar25 = fVar22 * 0.5;
  if ((fVar25 < in_XMM3_Da) || (fVar24 = fVar23 * 0.5, fVar24 < in_stack_00000028)) {
                    // WARNING: Subroutine does not return
    core::panicking::panic();
  }
  fVar22 = fVar22 / (float)*(int *)(in_RCX + 0x48);
  if (fVar22 <= 0.5) {
                    // WARNING: Subroutine does not return
    core::panicking::panic();
  }
  fVar23 = fVar23 / (float)*(int *)(in_RCX + 0x4c);
  fVar20 = floorf(((in_XMM1_Da - in_stack_00000030) + fVar25) / fVar22);
  iVar17 = (int)fVar20;
  if (2.1474835e+09 < fVar20) {
    iVar17 = 0x7fffffff;
  }
  if (NAN(fVar20)) {
    iVar17 = 0;
  }
  fVar20 = floorf(((in_XMM2_Da - in_stack_00000030) + fVar24) / fVar23);
  iVar14 = (int)fVar20;
  if (2.1474835e+09 < fVar20) {
    iVar14 = 0x7fffffff;
  }
  if (NAN(fVar20)) {
    iVar14 = 0;
  }
  if (iVar17 < 1) {
    iVar17 = 0;
  }
  if (iVar14 < 1) {
    iVar14 = 0;
  }
  fVar22 = floorf((in_XMM3_Da + in_stack_00000030 + fVar25) / fVar22);
  iVar19 = (int)fVar22;
  if (2.1474835e+09 < fVar22) {
    iVar19 = 0x7fffffff;
  }
  if (NAN(fVar22)) {
    iVar19 = 0;
  }
  fVar22 = floorf((in_stack_00000028 + in_stack_00000030 + fVar24) / fVar23);
  iVar18 = (int)fVar22;
  if (2.1474835e+09 < fVar22) {
    iVar18 = 0x7fffffff;
  }
  if (NAN(fVar22)) {
    iVar18 = 0;
  }
  iVar9 = (int)*(undefined8 *)(in_RCX + 0x48);
  iVar8 = iVar9 + -1;
  iVar11 = (int)((ulonglong)*(undefined8 *)(in_RCX + 0x48) >> 0x20) + -1;
  if (iVar8 < iVar19) {
    iVar19 = iVar8;
  }
  if (iVar18 <= iVar11) {
    iVar11 = iVar18;
  }
  if (iVar14 <= iVar11) {
    if (iVar19 < iVar17) {
      do {
        if (iVar11 <= iVar14) {
          return;
        }
        iVar14 = iVar14 + (uint)(iVar14 < iVar11);
      } while (iVar14 <= iVar11);
    }
    else {
      lVar2 = *(longlong *)(in_stack_00000040 + 8);
      uVar3 = *(ulonglong *)(in_stack_00000040 + 0x10);
      do {
        iVar8 = (uint)(iVar14 < iVar11) + iVar14;
        iVar18 = iVar17;
        do {
          uVar10 = (ulonglong)(iVar14 * iVar9 + iVar18);
          if (uVar3 <= uVar10) {
                    // WARNING: Subroutine does not return
            core::panicking::panic_bounds_check();
          }
          lVar6 = uVar10 * 0x30;
          lVar4 = *(longlong *)(lVar2 + 8 + lVar6);
          uVar10 = *(ulonglong *)(lVar2 + 0x10 + lVar6);
          lVar6 = in_stack_00000038[2];
          if ((ulonglong)(*in_stack_00000038 - lVar6) < uVar10) {
            alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
            lVar6 = in_stack_00000038[2];
LAB_1409f15fb:
            lVar5 = in_stack_00000038[1];
            uVar12 = uVar10 * 0x38 - 0x38;
            lVar7 = lVar4;
            if (0xa7 < uVar12) {
              uVar12 = uVar12 / 0x38 + 1;
              uVar16 = uVar12 & 0xfffffffffffffffc;
              lVar7 = lVar5 + lVar6 * 8;
              lVar6 = lVar6 + uVar16;
              uVar15 = 0;
              lVar13 = lVar4;
              do {
                auVar21._8_4_ = (int)lVar13;
                auVar21._0_8_ = lVar13;
                auVar21._12_4_ = (int)((ulonglong)lVar13 >> 0x20);
                plVar1 = (longlong *)(lVar7 + uVar15 * 8);
                *plVar1 = lVar13;
                plVar1[1] = auVar21._8_8_ + 0x38;
                plVar1 = (longlong *)(lVar7 + 0x10 + uVar15 * 8);
                *plVar1 = lVar13 + 0x70;
                plVar1[1] = auVar21._8_8_ + 0xa8;
                uVar15 = uVar15 + 4;
                lVar13 = lVar13 + 0xe0;
              } while (uVar16 != uVar15);
              lVar7 = uVar16 * 0x38 + lVar4;
              if (uVar12 == uVar16) goto LAB_1409f16c0;
            }
            do {
              *(longlong *)(lVar5 + lVar6 * 8) = lVar7;
              lVar7 = lVar7 + 0x38;
              lVar6 = lVar6 + 1;
            } while (lVar7 != lVar4 + uVar10 * 0x38);
          }
          else if (uVar10 != 0) goto LAB_1409f15fb;
LAB_1409f16c0:
          in_stack_00000038[2] = lVar6;
        } while ((iVar18 < iVar19) && (iVar18 = iVar18 + (uint)(iVar18 < iVar19), iVar18 <= iVar19))
        ;
      } while ((iVar14 < iVar11) && (iVar14 = iVar8, iVar8 <= iVar11));
    }
  }
  return;
}

