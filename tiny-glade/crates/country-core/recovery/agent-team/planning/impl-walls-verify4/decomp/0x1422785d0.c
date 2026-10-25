
void __cdecl
country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::
query_with_circle_callback<forest::manage_tree_prefabs::manage_tree_prefabs::closure_env_0>(void)

{
  ulonglong uVar1;
  undefined1 auVar2 [16];
  undefined4 extraout_EAX;
  int extraout_EAX_00;
  int extraout_EAX_01;
  int iVar3;
  ulonglong extraout_RAX;
  longlong in_RCX;
  undefined8 *in_RDX;
  longlong lVar4;
  int iVar5;
  undefined1 *in_R8;
  int iVar6;
  int iVar7;
  int iVar8;
  uint uVar9;
  int iVar10;
  float extraout_XMM0_Da;
  float extraout_XMM0_Da_00;
  float extraout_XMM0_Da_01;
  float fVar11;
  float fVar16;
  undefined1 auVar12 [16];
  undefined1 auVar14 [16];
  undefined1 auVar15 [16];
  float fVar17;
  undefined1 auVar18 [16];
  undefined1 auVar19 [16];
  ulonglong local_e0;
  ulonglong local_d8;
  ulonglong local_a8;
  ulonglong local_a0;
  undefined1 auVar13 [16];
  
  bounds_ws();
  fVar17 = (float)(local_e0 >> 0x20);
  fVar11 = (float)(local_d8 >> 0x20);
  ::utils::geometry::circle::Circle2d::aabb();
  cell_size();
  auVar19._8_8_ = 0;
  auVar19._0_8_ = local_a8;
  auVar18._8_8_ = 0;
  auVar18._0_8_ = local_a0;
  auVar12._8_8_ = 0;
  auVar12._0_8_ = local_e0;
  auVar12 = maxps(auVar12,auVar19);
  uVar9 = -(uint)NAN((float)(local_a8 >> 0x20));
  lVar4 = in_RCX;
  bounds_ws();
  iVar6 = (int)lVar4;
  iVar8 = -(uint)((float)local_e0 <=
                 (float)(~-(uint)NAN((float)local_a8) & auVar12._0_4_ |
                        (uint)(float)local_e0 & -(uint)NAN((float)local_a8)));
  iVar10 = -(uint)(fVar17 <= (float)(~uVar9 & auVar12._4_4_ | (uint)fVar17 & uVar9));
  auVar14._4_4_ = iVar10;
  auVar14._0_4_ = iVar8;
  auVar14._8_4_ = iVar10;
  auVar14._12_4_ = iVar10;
  auVar13._8_8_ = auVar14._8_8_;
  auVar13._4_4_ = iVar8;
  auVar13._0_4_ = iVar8;
  iVar8 = movmskpd(extraout_EAX,auVar13);
  if (iVar8 != 3) {
                    // WARNING: Subroutine does not return
    core::panicking::panic();
  }
  auVar15._8_8_ = 0;
  auVar15._0_8_ = local_d8;
  auVar12 = minps(auVar15,auVar18);
  uVar9 = -(uint)NAN((float)(local_a0 >> 0x20));
  fVar17 = (float)(~-(uint)NAN((float)local_a0) & auVar12._0_4_ |
                  (uint)(float)local_d8 & -(uint)NAN((float)local_a0));
  iVar8 = -(uint)(fVar17 <= (float)local_d8);
  iVar10 = -(uint)((float)(~uVar9 & auVar12._4_4_ | (uint)fVar11 & uVar9) <= fVar11);
  auVar2._4_4_ = iVar8;
  auVar2._0_4_ = iVar8;
  auVar2._8_4_ = iVar10;
  auVar2._12_4_ = iVar10;
  iVar8 = movmskpd(3,auVar2);
  if (iVar8 != 3) {
                    // WARNING: Subroutine does not return
    core::panicking::panic();
  }
  cell_size();
  if (extraout_XMM0_Da_00 <= 0.5) {
                    // WARNING: Subroutine does not return
    core::panicking::panic();
  }
  world_space_to_grid_coord_unclamped();
  iVar8 = extraout_EAX_00;
  if (extraout_EAX_00 < 1) {
    iVar8 = 0;
  }
  iVar10 = 0;
  if (0 < iVar6) {
    iVar10 = iVar6;
  }
  fVar17 = fVar17 + 0.0;
  world_space_to_grid_coord_unclamped();
  iVar3 = (int)*(undefined8 *)(in_RCX + 0x48) + -1;
  iVar5 = (int)((ulonglong)*(undefined8 *)(in_RCX + 0x48) >> 0x20) + -1;
  iVar7 = extraout_EAX_01;
  if (iVar3 < extraout_EAX_01) {
    iVar7 = iVar3;
  }
  if (iVar6 <= iVar5) {
    iVar5 = iVar6;
  }
  if (iVar10 <= iVar5) {
    if (iVar7 < iVar8) {
      do {
        if (iVar5 <= iVar10) {
          return;
        }
        iVar10 = iVar10 + (uint)(iVar10 < iVar5);
      } while (iVar10 <= iVar5);
    }
    else {
      lVar4 = *(longlong *)(in_RCX + 8);
      uVar1 = *(ulonglong *)(in_RCX + 0x10);
      do {
        iVar3 = (uint)(iVar10 < iVar5) + iVar10;
        iVar6 = iVar8;
        do {
          grid_coord_to_cell_index();
          cell_center();
          fVar11 = extraout_XMM0_Da_01 - (float)*in_RDX;
          fVar16 = fVar17 - (float)((ulonglong)*in_RDX >> 0x20);
          fVar17 = *(float *)(in_RDX + 1) + extraout_XMM0_Da * 0.5 * 1.4142135;
          if (SQRT(fVar16 * fVar16 + fVar11 * fVar11) < fVar17) {
            if (uVar1 <= extraout_RAX) {
                    // WARNING: Subroutine does not return
              core::panicking::panic_bounds_check();
            }
            if (*(longlong *)(lVar4 + 0x10 + extraout_RAX * 0x30) != 0) {
              *in_R8 = 0;
              return;
            }
          }
        } while ((iVar6 < iVar7) && (iVar6 = iVar6 + (uint)(iVar6 < iVar7), iVar6 <= iVar7));
      } while ((iVar10 < iVar5) && (iVar10 = iVar3, iVar3 <= iVar5));
    }
  }
  return;
}

