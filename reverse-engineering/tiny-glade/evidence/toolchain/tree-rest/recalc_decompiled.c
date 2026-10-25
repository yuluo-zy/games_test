
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void __cdecl
system_tree::recalc_tree_position_on_terrain_height_change::
recalc_tree_position_on_terrain_height_change(void)

{
  uint *puVar1;
  undefined4 uVar2;
  ulonglong *puVar3;
  longlong lVar4;
  longlong lVar5;
  uint uVar6;
  float fVar7;
  ulonglong uVar8;
  longlong lVar9;
  ulonglong uVar10;
  longlong *in_RCX;
  longlong lVar11;
  longlong lVar12;
  uint *puVar13;
  uint *puVar14;
  ulonglong uVar15;
  longlong lVar16;
  ulonglong uVar17;
  ulonglong uVar18;
  longlong lVar19;
  undefined8 *in_R8;
  ulonglong uVar20;
  uint *puVar21;
  bool bVar22;
  float extraout_XMM0_Da;
  float fVar23;
  float extraout_XMM0_Da_00;
  
  puVar3 = (ulonglong *)*in_R8;
  lVar4 = in_R8[1];
  uVar10 = *puVar3;
  uVar8 = 0;
  uVar18 = uVar10 - *(ulonglong *)(lVar4 + 0x18);
  if (uVar10 < *(ulonglong *)(lVar4 + 0x18)) {
    uVar18 = uVar8;
  }
  uVar20 = uVar10 - *(ulonglong *)(lVar4 + 0x38);
  if (uVar10 < *(ulonglong *)(lVar4 + 0x38)) {
    uVar20 = uVar8;
  }
  bVar22 = *(ulonglong *)(lVar4 + 0x30) < uVar20;
  uVar15 = *(ulonglong *)(lVar4 + 0x30) - uVar20;
  uVar10 = uVar15;
  if (bVar22) {
    uVar10 = uVar8;
  }
  lVar9 = CONCAT71((int7)(uVar20 >> 8),bVar22 || uVar15 == 0);
  uVar20 = *(ulonglong *)(lVar4 + 0x10);
  uVar17 = uVar20 - uVar18;
  if (uVar20 < uVar18) {
    uVar17 = uVar8;
  }
  uVar8 = *(ulonglong *)(lVar4 + 0x40);
  *puVar3 = uVar8 - (uVar17 + uVar10);
  if ((uVar20 < uVar18 || uVar20 - uVar18 == 0) && (bVar22 || uVar15 == 0)) {
    return;
  }
  *puVar3 = uVar8;
  fVar7 = __real_42820000;
  uVar6 = ___xmm_7fffffff7fffffff7fffffff7fffffff;
  lVar4 = *in_RCX;
  lVar5 = in_RCX[1];
  uVar2 = *(undefined4 *)((longlong)in_RCX + 0x14);
  puVar21 = *(uint **)(lVar4 + 0xf8);
  puVar1 = puVar21 + *(longlong *)(lVar4 + 0x100);
  if (*(char *)(lVar4 + 0x108) == '\0') {
    lVar11 = 8;
    lVar16 = 0;
    lVar19 = 0;
    lVar12 = 0;
    do {
      if (lVar12 == lVar19) {
        if (puVar21 == puVar1) {
          return;
        }
        while( true ) {
          lVar19 = *(longlong *)(*(longlong *)(lVar5 + 0x100) + 0x58 + (ulonglong)*puVar21 * 0xa0);
          if (lVar19 != 0) break;
          puVar21 = puVar21 + 1;
          if (puVar21 == puVar1) {
            return;
          }
        }
        lVar11 = *(longlong *)(lVar5 + 0x100) + (ulonglong)*puVar21 * 0xa0;
        uVar10 = (ulonglong)*(uint *)(lVar11 + 0x94);
        lVar9 = *(longlong *)(*(longlong *)(lVar5 + 0x1a8) + 0x18 + uVar10 * 0x48);
        lVar12 = ~*(ulonglong *)
                   (*(longlong *)(*(longlong *)(lVar5 + 0x1a8) + 0x38 + uVar10 * 0x48) +
                   *(longlong *)(lVar4 + 0x110) * 8) * 0x30;
        lVar16 = *(longlong *)(lVar9 + 0x10 + lVar12);
        lVar9 = *(longlong *)(lVar9 + 0x28 + lVar12);
        lVar11 = *(longlong *)(lVar11 + 0x50);
        uVar10 = (ulonglong)*(uint *)(lVar11 + 8);
        puVar14 = (uint *)(uVar10 * 0x30 + lVar16);
        puVar21 = puVar21 + 1;
        lVar12 = 1;
      }
      else {
        if (lVar16 == 0) {
          return;
        }
        uVar10 = (ulonglong)*(uint *)(lVar11 + 8 + lVar12 * 0x10);
        puVar14 = (uint *)(uVar10 * 0x30 + lVar16);
        lVar12 = lVar12 + 1;
      }
      *(undefined4 *)(lVar9 + uVar10 * 4) = uVar2;
      if (((float)(*puVar14 & uVar6) <= fVar7) && ((float)(puVar14[2] & uVar6) <= fVar7)) {
        country_core::resources::placement_data::PlacementDataSsbo::position();
        country_core::resources::terrain_height::TerrainHeightsData::
        sample_at_world_xz<country_core::resources::terrain_height::TerrainHeightSsbo>();
        if (((float)puVar14[1] != extraout_XMM0_Da_00) ||
           (NAN((float)puVar14[1]) || NAN(extraout_XMM0_Da_00))) {
          fVar23 = extraout_XMM0_Da_00;
          if (extraout_XMM0_Da_00 <= 0.0) {
            fVar23 = 0.0;
          }
          puVar14[1] = (uint)fVar23;
        }
      }
    } while( true );
  }
  puVar14 = (uint *)0x0;
  uVar10 = 0;
  uVar18 = 0;
  do {
    if (uVar18 == uVar10) {
      if (puVar21 == puVar1) {
        return;
      }
      while( true ) {
        uVar10 = *(ulonglong *)(*(longlong *)(lVar5 + 0x1a8) + 0x10 + (ulonglong)*puVar21 * 0x48);
        if (uVar10 != 0) break;
        puVar21 = puVar21 + 1;
        if (puVar21 == puVar1) {
          return;
        }
      }
      lVar9 = *(longlong *)(lVar5 + 0x1a8) + (ulonglong)*puVar21 * 0x48;
      lVar19 = *(longlong *)(lVar9 + 0x18);
      lVar9 = ~*(ulonglong *)(*(longlong *)(lVar9 + 0x38) + *(longlong *)(lVar4 + 0x110) * 8) * 0x30
      ;
      puVar13 = *(uint **)(lVar19 + 0x10 + lVar9);
      in_RCX = *(longlong **)(lVar19 + 0x28 + lVar9);
      puVar21 = puVar21 + 1;
      uVar18 = 1;
      uVar8 = 0;
      puVar14 = puVar13;
    }
    else {
      if (puVar14 == (uint *)0x0) {
        return;
      }
      uVar8 = uVar18 & 0xffffffff;
      puVar13 = puVar14 + uVar8 * 0xc;
      uVar18 = uVar18 + 1;
    }
    *(undefined4 *)((longlong)in_RCX + uVar8 * 4) = uVar2;
    if (((float)(*puVar13 & uVar6) <= fVar7) && ((float)(puVar13[2] & uVar6) <= fVar7)) {
      country_core::resources::placement_data::PlacementDataSsbo::position();
      country_core::resources::terrain_height::TerrainHeightsData::
      sample_at_world_xz<country_core::resources::terrain_height::TerrainHeightSsbo>();
      if (((float)puVar13[1] != extraout_XMM0_Da) ||
         (NAN((float)puVar13[1]) || NAN(extraout_XMM0_Da))) {
        fVar23 = extraout_XMM0_Da;
        if (extraout_XMM0_Da <= 0.0) {
          fVar23 = 0.0;
        }
        puVar13[1] = (uint)fVar23;
      }
    }
  } while( true );
}

