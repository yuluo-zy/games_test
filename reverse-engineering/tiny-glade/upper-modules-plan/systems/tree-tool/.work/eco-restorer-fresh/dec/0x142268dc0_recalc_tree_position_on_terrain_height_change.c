// ADDRESS 0x142268dc0 SIZE 873 SIGNATURE void __cdecl recalc_tree_position_on_terrain_height_change(void)

void __cdecl
system_tree::recalc_tree_position_on_terrain_height_change::
recalc_tree_position_on_terrain_height_change(void)

{
  uint *puVar1;
  undefined4 uVar2;
  ulonglong *puVar3;
  longlong lVar4;
  longlong lVar5;
  ulonglong uVar6;
  longlong lVar7;
  ulonglong uVar8;
  longlong *in_RCX;
  longlong lVar9;
  longlong lVar10;
  float *pfVar11;
  float *pfVar12;
  ulonglong uVar13;
  longlong lVar14;
  ulonglong uVar15;
  ulonglong uVar16;
  longlong lVar17;
  undefined8 *in_R8;
  ulonglong uVar18;
  uint *puVar19;
  bool bVar20;
  float extraout_XMM0_Da;
  float fVar21;
  float extraout_XMM0_Da_00;
  
  puVar3 = (ulonglong *)*in_R8;
  lVar4 = in_R8[1];
  uVar8 = *puVar3;
  uVar6 = 0;
  uVar16 = uVar8 - *(ulonglong *)(lVar4 + 0x18);
  if (uVar8 < *(ulonglong *)(lVar4 + 0x18)) {
    uVar16 = uVar6;
  }
  uVar18 = uVar8 - *(ulonglong *)(lVar4 + 0x38);
  if (uVar8 < *(ulonglong *)(lVar4 + 0x38)) {
    uVar18 = uVar6;
  }
  bVar20 = *(ulonglong *)(lVar4 + 0x30) < uVar18;
  uVar13 = *(ulonglong *)(lVar4 + 0x30) - uVar18;
  uVar8 = uVar13;
  if (bVar20) {
    uVar8 = uVar6;
  }
  lVar7 = CONCAT71((int7)(uVar18 >> 8),bVar20 || uVar13 == 0);
  uVar18 = *(ulonglong *)(lVar4 + 0x10);
  uVar15 = uVar18 - uVar16;
  if (uVar18 < uVar16) {
    uVar15 = uVar6;
  }
  uVar6 = *(ulonglong *)(lVar4 + 0x40);
  *puVar3 = uVar6 - (uVar15 + uVar8);
  if ((uVar18 < uVar16 || uVar18 - uVar16 == 0) && (bVar20 || uVar13 == 0)) {
    return;
  }
  *puVar3 = uVar6;
  lVar4 = *in_RCX;
  lVar5 = in_RCX[1];
  uVar2 = *(undefined4 *)((longlong)in_RCX + 0x14);
  puVar19 = *(uint **)(lVar4 + 0xf8);
  puVar1 = puVar19 + *(longlong *)(lVar4 + 0x100);
  if (*(char *)(lVar4 + 0x108) == '\0') {
    lVar9 = 8;
    lVar14 = 0;
    lVar17 = 0;
    lVar10 = 0;
    do {
      if (lVar10 == lVar17) {
        if (puVar19 == puVar1) {
          return;
        }
        while( true ) {
          lVar17 = *(longlong *)(*(longlong *)(lVar5 + 0x100) + 0x58 + (ulonglong)*puVar19 * 0xa0);
          if (lVar17 != 0) break;
          puVar19 = puVar19 + 1;
          if (puVar19 == puVar1) {
            return;
          }
        }
        lVar9 = *(longlong *)(lVar5 + 0x100) + (ulonglong)*puVar19 * 0xa0;
        uVar8 = (ulonglong)*(uint *)(lVar9 + 0x94);
        lVar7 = *(longlong *)(*(longlong *)(lVar5 + 0x1a8) + 0x18 + uVar8 * 0x48);
        lVar10 = ~*(ulonglong *)
                   (*(longlong *)(*(longlong *)(lVar5 + 0x1a8) + 0x38 + uVar8 * 0x48) +
                   *(longlong *)(lVar4 + 0x110) * 8) * 0x30;
        lVar14 = *(longlong *)(lVar7 + 0x10 + lVar10);
        lVar7 = *(longlong *)(lVar7 + 0x28 + lVar10);
        lVar9 = *(longlong *)(lVar9 + 0x50);
        uVar8 = (ulonglong)*(uint *)(lVar9 + 8);
        pfVar12 = (float *)(uVar8 * 0x30 + lVar14);
        puVar19 = puVar19 + 1;
        lVar10 = 1;
      }
      else {
        if (lVar14 == 0) {
          return;
        }
        uVar8 = (ulonglong)*(uint *)(lVar9 + 8 + lVar10 * 0x10);
        pfVar12 = (float *)(uVar8 * 0x30 + lVar14);
        lVar10 = lVar10 + 1;
      }
      *(undefined4 *)(lVar7 + uVar8 * 4) = uVar2;
      if ((ABS(*pfVar12) <= 65.0) && (ABS(pfVar12[2]) <= 65.0)) {
        country_core::resources::placement_data::PlacementDataSsbo::position();
        country_core::resources::terrain_height::TerrainHeightsData::
        sample_at_world_xz<country_core::resources::terrain_height::TerrainHeightSsbo>();
        if ((pfVar12[1] != extraout_XMM0_Da_00) || (NAN(pfVar12[1]) || NAN(extraout_XMM0_Da_00))) {
          fVar21 = extraout_XMM0_Da_00;
          if (extraout_XMM0_Da_00 <= 0.0) {
            fVar21 = 0.0;
          }
          pfVar12[1] = fVar21;
        }
      }
    } while( true );
  }
  pfVar12 = (float *)0x0;
  uVar8 = 0;
  uVar16 = 0;
  do {
    if (uVar16 == uVar8) {
      if (puVar19 == puVar1) {
        return;
      }
      while( true ) {
        uVar8 = *(ulonglong *)(*(longlong *)(lVar5 + 0x1a8) + 0x10 + (ulonglong)*puVar19 * 0x48);
        if (uVar8 != 0) break;
        puVar19 = puVar19 + 1;
        if (puVar19 == puVar1) {
          return;
        }
      }
      lVar7 = *(longlong *)(lVar5 + 0x1a8) + (ulonglong)*puVar19 * 0x48;
      lVar17 = *(longlong *)(lVar7 + 0x18);
      lVar7 = ~*(ulonglong *)(*(longlong *)(lVar7 + 0x38) + *(longlong *)(lVar4 + 0x110) * 8) * 0x30
      ;
      pfVar11 = *(float **)(lVar17 + 0x10 + lVar7);
      in_RCX = *(longlong **)(lVar17 + 0x28 + lVar7);
      puVar19 = puVar19 + 1;
      uVar16 = 1;
      uVar6 = 0;
      pfVar12 = pfVar11;
    }
    else {
      if (pfVar12 == (float *)0x0) {
        return;
      }
      uVar6 = uVar16 & 0xffffffff;
      pfVar11 = pfVar12 + uVar6 * 0xc;
      uVar16 = uVar16 + 1;
    }
    *(undefined4 *)((longlong)in_RCX + uVar6 * 4) = uVar2;
    if ((ABS(*pfVar11) <= 65.0) && (ABS(pfVar11[2]) <= 65.0)) {
      country_core::resources::placement_data::PlacementDataSsbo::position();
      country_core::resources::terrain_height::TerrainHeightsData::
      sample_at_world_xz<country_core::resources::terrain_height::TerrainHeightSsbo>();
      if ((pfVar11[1] != extraout_XMM0_Da) || (NAN(pfVar11[1]) || NAN(extraout_XMM0_Da))) {
        fVar21 = extraout_XMM0_Da;
        if (extraout_XMM0_Da <= 0.0) {
          fVar21 = 0.0;
        }
        pfVar11[1] = fVar21;
      }
    }
  } while( true );
}


// LOCALS:
[
 {
  "name": "lVar7",
  "type": "longlong",
  "storage": "RAX:8",
  "is_phantom": false,
  "in_decompiled_code": true
 },
 {
  "name": "uVar6",
  "type": "ulonglong",
  "storage": "RAX:8",
  "is_phantom": false,
  "in_decompiled_code": true
 },
 {
  "name": "in_RCX",
  "type": "longlong *",
  "storage": "RCX:8",
  "is_phantom": true,
  "in_decompiled_code": true
 },
 {
  "name": "uVar8",
  "type": "ulonglong",
  "storage": "RAX:8",
  "is_phantom": false,
  "in_decompiled_code": true
 },
 {
  "name": "lVar10",
  "type": "longlong",
  "storage": "RDX:8",
  "is_phantom": false,
  "in_decompiled_code": true
 },
 {
  "name": "lVar9",
  "type": "longlong",
  "storage": "RDX:8",
  "is_phantom": false,
  "in_decompiled_code": true
 },
 {
  "name": "pfVar12",
  "type": "float *",
  "storage": "RBX:8",
  "is_phantom": false,
  "in_decompiled_code": true
 },
 {
  "name": "pfVar11",
  "type": "float *",
  "storage": "RBX:8",
  "is_phantom": false,
  "in_decompiled_code": true
 },
 {
  "name": "puVar1",
  "type": "uint *",
  "is_phantom": false,
  "in_decompiled_code": true
 },
 {
  "name": "puVar3",
  "type": "ulonglong *",
  "is_phantom": false,
  "in_decompiled_code": true
 },
 {
  "name": "uVar2",
  "type": "undefined4",
  "is_phantom": false,
  "in_decompiled_code": true
 },
 {
  "name": "lVar5",
  "type": "longlong",
  "is_phantom": false,
  "in_decompiled_code": true
 },
 {
  "name": "lVar4",
  "type": "longlong",
  "is_phantom": false,
  "in_decompiled_code": true
 },
 {
  "name": "extraout_XMM0_Da",
  "type": "float",
  "storage": "XMM0_Da:4",
  "is_phantom": true,
  "in_decompiled_code": true
 },
 {
  "name": "bVar20",
  "type": "bool",
  "storage": "CF:1",
  "is_phantom": false,
  "in_decompiled_code": true
 },
 {
  "name": "extraout_XMM0_Da_00",
  "type": "float",
  "storage": "XMM0_Da:4",
  "is_phantom": true,
  "in_decompiled_code": true
 },
 {
  "name": "fVar21",
  "type": "float",
  "storage": "XMM0_Da:4",
  "is_phantom": false,
  "in_decompiled_code": true
 },
 {
  "name": "lVar14",
  "type": "longlong",
  "storage": "RSI:8",
  "is_phantom": false,
  "in_decompiled_code": true
 },
 {
  "name": "uVar13",
  "type": "ulonglong",
  "storage": "RSI:8",
  "is_phantom": false,
  "in_decompiled_code": true
 },
 {
  "name": "uVar16",
  "type": "ulonglong",
  "storage": "RDI:8",
  "is_phantom": false,
  "in_decompiled_code": true
 },
 {
  "name": "uVar15",
  "type": "ulonglong",
  "storage": "RDI:8",
  "is_phantom": false,
  "in_decompiled_code": true
 },
 {
  "name": "in_R8",
  "type": "undefined8 *",
  "storage": "R8:8",
  "is_phantom": true,
  "in_decompiled_code": true
 },
 {
  "name": "lVar17",
  "type": "longlong",
  "storage": "RDI:8",
  "is_phantom": false,
  "in_decompiled_code": true
 },
 {
  "name": "puVar19",
  "type": "uint *",
  "storage": "R12:8",
  "is_phantom": false,
  "in_decompiled_code": true
 },
 {
  "name": "uVar18",
  "type": "ulonglong",
  "storage": "R9:8",
  "is_phantom": false,
  "in_decompiled_code": true
 }
]