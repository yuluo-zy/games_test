
void __cdecl
country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::
query_with_circle_callback<system_tree::hide_trees::hide_trees::closure_env_0>(void)

{
  undefined8 uVar1;
  ulonglong uVar2;
  longlong *plVar3;
  float *pfVar4;
  longlong lVar5;
  longlong lVar6;
  undefined1 auVar7 [16];
  undefined4 extraout_EAX;
  int extraout_EAX_00;
  int extraout_EAX_01;
  int iVar8;
  ulonglong extraout_RAX;
  longlong in_RCX;
  undefined8 *in_RDX;
  longlong lVar9;
  int iVar10;
  undefined8 *in_R8;
  longlong lVar11;
  int iVar12;
  int iVar13;
  int iVar14;
  uint uVar15;
  int iVar16;
  float extraout_XMM0_Da;
  float extraout_XMM0_Da_00;
  float extraout_XMM0_Da_01;
  float fVar17;
  float fVar22;
  undefined1 auVar18 [16];
  undefined1 auVar20 [16];
  undefined1 auVar21 [16];
  float fVar23;
  float fVar24;
  float fVar25;
  float fVar26;
  float fVar27;
  float fVar28;
  float fVar29;
  undefined1 auVar30 [16];
  undefined1 auVar31 [16];
  ulonglong local_d0;
  ulonglong local_c8;
  ulonglong local_a8;
  ulonglong local_a0;
  undefined1 auVar19 [16];
  
  bounds_ws();
  fVar29 = (float)(local_d0 >> 0x20);
  fVar17 = (float)(local_c8 >> 0x20);
  ::utils::geometry::circle::Circle2d::aabb();
  cell_size();
  auVar31._8_8_ = 0;
  auVar31._0_8_ = local_a8;
  auVar30._8_8_ = 0;
  auVar30._0_8_ = local_a0;
  auVar18._8_8_ = 0;
  auVar18._0_8_ = local_d0;
  auVar18 = maxps(auVar18,auVar31);
  uVar15 = -(uint)NAN((float)(local_a8 >> 0x20));
  lVar9 = in_RCX;
  bounds_ws();
  iVar12 = (int)lVar9;
  iVar14 = -(uint)((float)local_d0 <=
                  (float)(~-(uint)NAN((float)local_a8) & auVar18._0_4_ |
                         (uint)(float)local_d0 & -(uint)NAN((float)local_a8)));
  iVar16 = -(uint)(fVar29 <= (float)(~uVar15 & auVar18._4_4_ | (uint)fVar29 & uVar15));
  auVar20._4_4_ = iVar16;
  auVar20._0_4_ = iVar14;
  auVar20._8_4_ = iVar16;
  auVar20._12_4_ = iVar16;
  auVar19._8_8_ = auVar20._8_8_;
  auVar19._4_4_ = iVar14;
  auVar19._0_4_ = iVar14;
  iVar14 = movmskpd(extraout_EAX,auVar19);
  if (iVar14 != 3) {
                    // WARNING: Subroutine does not return
    core::panicking::panic();
  }
  auVar21._8_8_ = 0;
  auVar21._0_8_ = local_c8;
  auVar18 = minps(auVar21,auVar30);
  uVar15 = -(uint)NAN((float)(local_a0 >> 0x20));
  fVar29 = (float)(~-(uint)NAN((float)local_a0) & auVar18._0_4_ |
                  (uint)(float)local_c8 & -(uint)NAN((float)local_a0));
  iVar14 = -(uint)(fVar29 <= (float)local_c8);
  iVar16 = -(uint)((float)(~uVar15 & auVar18._4_4_ | (uint)fVar17 & uVar15) <= fVar17);
  auVar7._4_4_ = iVar14;
  auVar7._0_4_ = iVar14;
  auVar7._8_4_ = iVar16;
  auVar7._12_4_ = iVar16;
  iVar14 = movmskpd(3,auVar7);
  if (iVar14 == 3) {
    cell_size();
    if (extraout_XMM0_Da_00 <= 0.5) {
                    // WARNING: Subroutine does not return
      core::panicking::panic();
    }
    world_space_to_grid_coord_unclamped();
    iVar14 = extraout_EAX_00;
    if (extraout_EAX_00 < 1) {
      iVar14 = 0;
    }
    iVar16 = 0;
    if (0 < iVar12) {
      iVar16 = iVar12;
    }
    fVar29 = fVar29 + 0.0;
    world_space_to_grid_coord_unclamped();
    iVar8 = (int)*(undefined8 *)(in_RCX + 0x48) + -1;
    iVar10 = (int)((ulonglong)*(undefined8 *)(in_RCX + 0x48) >> 0x20) + -1;
    iVar13 = extraout_EAX_01;
    if (iVar8 < extraout_EAX_01) {
      iVar13 = iVar8;
    }
    if (iVar12 <= iVar10) {
      iVar10 = iVar12;
    }
    if (iVar16 <= iVar10) {
      if (iVar13 < iVar14) {
        do {
          if (iVar10 <= iVar16) {
            return;
          }
          iVar16 = iVar16 + (uint)(iVar16 < iVar10);
        } while (iVar16 <= iVar10);
      }
      else {
        lVar9 = *(longlong *)(in_RCX + 8);
        uVar2 = *(ulonglong *)(in_RCX + 0x10);
        plVar3 = (longlong *)*in_R8;
        pfVar4 = (float *)in_R8[1];
        do {
          iVar8 = (uint)(iVar16 < iVar10) + iVar16;
          iVar12 = iVar14;
          do {
            grid_coord_to_cell_index();
            cell_center();
            fVar17 = extraout_XMM0_Da_01 - (float)*in_RDX;
            fVar22 = fVar29 - (float)((ulonglong)*in_RDX >> 0x20);
            fVar29 = *(float *)(in_RDX + 1) + extraout_XMM0_Da * 0.5 * 1.4142135;
            if (SQRT(fVar22 * fVar22 + fVar17 * fVar17) < fVar29) {
              if (uVar2 <= extraout_RAX) {
                    // WARNING: Subroutine does not return
                core::panicking::panic_bounds_check();
              }
              lVar5 = *(longlong *)(lVar9 + 0x10 + extraout_RAX * 0x30);
              if (lVar5 != 0) {
                lVar6 = *(longlong *)(extraout_RAX * 0x30 + lVar9 + 8);
                lVar11 = 0;
                do {
                  if (*(longlong *)(lVar6 + 0x28 + lVar11) == *plVar3) {
                    uVar1 = *(undefined8 *)(lVar6 + 8 + lVar11);
                    fVar23 = (float)uVar1;
                    fVar24 = (float)((ulonglong)uVar1 >> 0x20);
                    fVar17 = *pfVar4;
                    fVar29 = pfVar4[2];
                    fVar25 = (float)*(undefined8 *)(lVar6 + lVar11);
                    fVar26 = (float)((ulonglong)*(undefined8 *)(lVar6 + lVar11) >> 0x20);
                    fVar27 = fVar23 - fVar25;
                    fVar28 = fVar24 - fVar26;
                    fVar22 = fVar28 * fVar28 + fVar27 * fVar27;
                    if (fVar22 <= 1e-20) {
                      fVar22 = 1e-20;
                    }
                    fVar22 = ((fVar29 - fVar26) * fVar28 + (fVar17 - fVar25) * fVar27) / fVar22;
                    fVar27 = 0.0;
                    if (0.0 <= fVar22) {
                      fVar27 = fVar22;
                    }
                    fVar22 = 1.0;
                    if (fVar27 <= 1.0) {
                      fVar22 = fVar27;
                    }
                    fVar17 = fVar17 - (fVar22 * fVar23 + (1.0 - fVar22) * fVar25);
                    fVar29 = fVar29 - (fVar22 * fVar24 + (1.0 - fVar22) * fVar26);
                    fVar29 = fVar29 * fVar29 + fVar17 * fVar17;
                    if (SQRT(fVar29) < 0.4) {
                      *(undefined1 *)in_R8[2] = 0;
                      return;
                    }
                  }
                  lVar11 = lVar11 + 0x38;
                } while (lVar5 * 0x38 - lVar11 != 0);
              }
            }
          } while ((iVar12 < iVar13) &&
                  (iVar12 = iVar12 + (uint)(iVar12 < iVar13), iVar12 <= iVar13));
        } while ((iVar16 < iVar10) && (iVar16 = iVar8, iVar8 <= iVar10));
      }
    }
    return;
  }
                    // WARNING: Subroutine does not return
  core::panicking::panic();
}

