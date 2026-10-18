// ADDRESS 0x1422689c0 SIZE 1004 SIGNATURE void __cdecl tree_move_bend(void)

void __cdecl system_tree::tree_move_bend::tree_move_bend(void)

{
  uint *puVar1;
  float *pfVar2;
  undefined4 *puVar3;
  char cVar4;
  undefined4 uVar5;
  longlong lVar6;
  longlong lVar7;
  longlong lVar8;
  ulonglong uVar9;
  ulonglong uVar10;
  longlong *in_RCX;
  longlong lVar11;
  longlong lVar12;
  longlong lVar13;
  ulonglong uVar14;
  uint *puVar15;
  float extraout_XMM0_Da;
  float extraout_XMM0_Da_00;
  float fVar16;
  float fVar17;
  float fVar18;
  float fVar19;
  longlong local_f8;
  longlong local_f0;
  longlong local_e0;
  longlong local_d8;
  longlong local_c8;
  ulonglong local_b8;
  float local_a4;
  float local_9c;
  
  lVar6 = *in_RCX;
  lVar7 = in_RCX[1];
  uVar5 = *(undefined4 *)((longlong)in_RCX + 0x14);
  puVar15 = *(uint **)(lVar6 + 0xf8);
  puVar1 = puVar15 + *(longlong *)(lVar6 + 0x100);
  cVar4 = *(char *)(lVar6 + 0x108);
  local_c8 = 8;
  local_f8 = 0;
  local_f0 = 0;
  uVar14 = 0;
  local_b8 = 0;
  do {
    if (cVar4 == '\0') {
      if (local_b8 == uVar14) {
        if (puVar15 == puVar1) {
          return;
        }
        while( true ) {
          uVar14 = *(ulonglong *)(*(longlong *)(lVar7 + 0x100) + 0x58 + (ulonglong)*puVar15 * 0xa0);
          if (uVar14 != 0) break;
          puVar15 = puVar15 + 1;
          if (puVar15 == puVar1) {
            return;
          }
        }
        lVar11 = *(longlong *)(lVar7 + 0x100) + (ulonglong)*puVar15 * 0xa0;
        uVar9 = (ulonglong)*(uint *)(lVar11 + 0x94);
        lVar8 = *(longlong *)(*(longlong *)(lVar7 + 0x1a8) + 0x18 + uVar9 * 0x48);
        lVar12 = *(longlong *)(*(longlong *)(lVar7 + 0x1a8) + 0x38 + uVar9 * 0x48);
        lVar13 = ~*(ulonglong *)(lVar12 + *(longlong *)(lVar6 + 0x110) * 8) * 0x30;
        local_f0 = *(longlong *)(lVar8 + 0x10 + lVar13);
        local_d8 = *(longlong *)(lVar8 + 0x28 + lVar13);
        lVar12 = ~*(ulonglong *)(lVar12 + *(longlong *)(lVar6 + 0x118) * 8) * 0x30;
        local_f8 = *(longlong *)(lVar8 + 0x10 + lVar12);
        local_e0 = *(longlong *)(lVar8 + 0x28 + lVar12);
        local_c8 = *(longlong *)(lVar11 + 0x50);
        puVar15 = puVar15 + 1;
        local_b8 = 0;
      }
      uVar9 = (ulonglong)*(uint *)(local_c8 + 8 + local_b8 * 0x10);
    }
    else {
      if (local_b8 == uVar14) {
        if (puVar15 == puVar1) {
          return;
        }
        while( true ) {
          uVar14 = *(ulonglong *)(*(longlong *)(lVar7 + 0x1a8) + 0x10 + (ulonglong)*puVar15 * 0x48);
          if (uVar14 != 0) break;
          puVar15 = puVar15 + 1;
          if (puVar15 == puVar1) {
            return;
          }
        }
        lVar8 = *(longlong *)(lVar7 + 0x1a8) + (ulonglong)*puVar15 * 0x48;
        lVar12 = *(longlong *)(lVar8 + 0x18);
        lVar8 = *(longlong *)(lVar8 + 0x38);
        lVar11 = ~*(ulonglong *)(lVar8 + *(longlong *)(lVar6 + 0x110) * 8) * 0x30;
        local_f0 = *(longlong *)(lVar12 + 0x10 + lVar11);
        local_d8 = *(longlong *)(lVar12 + 0x28 + lVar11);
        lVar8 = ~*(ulonglong *)(lVar8 + *(longlong *)(lVar6 + 0x118) * 8) * 0x30;
        local_f8 = *(longlong *)(lVar12 + 0x10 + lVar8);
        local_e0 = *(longlong *)(lVar12 + 0x28 + lVar8);
        puVar15 = puVar15 + 1;
        local_b8 = 0;
      }
      uVar9 = local_b8 & 0xffffffff;
    }
    pfVar2 = (float *)(local_f0 + uVar9 * 8);
    puVar3 = (undefined4 *)(local_d8 + uVar9 * 4);
    lVar8 = uVar9 * 0x30 + local_f8;
    country_core::resources::placement_data::PlacementDataSsbo::position();
    ::utils::time_continuous_lerp();
    *puVar3 = uVar5;
    *pfVar2 = extraout_XMM0_Da;
    ::utils::time_continuous_lerp();
    *puVar3 = uVar5;
    pfVar2[1] = extraout_XMM0_Da_00;
    fVar18 = *pfVar2 - local_a4;
    fVar19 = extraout_XMM0_Da_00 - local_9c;
    fVar16 = SQRT(fVar19 * fVar19 + fVar18 * fVar18);
    uVar10 = 0;
    fVar17 = 0.0;
    if (0.1 <= fVar16) {
      fVar17 = 5.0;
      if (fVar16 <= 5.0) {
        fVar17 = fVar16;
      }
      *puVar3 = uVar5;
      fVar18 = fVar17 * fVar18 * (1.0 / fVar16);
      fVar17 = fVar17 * fVar19 * (1.0 / fVar16);
      uVar10 = (ulonglong)(uint)fVar18;
      *(ulonglong *)pfVar2 = CONCAT44(local_9c + fVar17,local_a4 + fVar18);
    }
    local_b8 = local_b8 + 1;
    if ((((*(float *)(lVar8 + 0x24) != (float)uVar10) ||
         (NAN(*(float *)(lVar8 + 0x24)) || NAN((float)uVar10))) ||
        (*(float *)(lVar8 + 0x28) != fVar17)) || (NAN(*(float *)(lVar8 + 0x28)) || NAN(fVar17))) {
      *(undefined4 *)(local_e0 + uVar9 * 4) = uVar5;
      *(ulonglong *)(lVar8 + 0x24) = uVar10 | (ulonglong)(uint)fVar17 << 0x20;
    }
  } while( true );
}


// LOCALS:
[
 {
  "name": "fVar18",
  "type": "float",
  "storage": "XMM1_Da:4",
  "is_phantom": false,
  "in_decompiled_code": true
 },
 {
  "name": "fVar17",
  "type": "float",
  "storage": "XMM0_Db:4",
  "is_phantom": false,
  "in_decompiled_code": true
 },
 {
  "name": "fVar19",
  "type": "float",
  "storage": "XMM1_Db:4",
  "is_phantom": false,
  "in_decompiled_code": true
 },
 {
  "name": "lVar12",
  "type": "longlong",
  "storage": "RCX:8",
  "is_phantom": false,
  "in_decompiled_code": true
 },
 {
  "name": "lVar11",
  "type": "longlong",
  "storage": "RCX:8",
  "is_phantom": false,
  "in_decompiled_code": true
 },
 {
  "name": "uVar14",
  "type": "ulonglong",
  "storage": "R14:8",
  "is_phantom": false,
  "in_decompiled_code": true
 },
 {
  "name": "lVar13",
  "type": "longlong",
  "storage": "RDX:8",
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
  "name": "puVar15",
  "type": "uint *",
  "storage": "R15:8",
  "is_phantom": false,
  "in_decompiled_code": true
 },
 {
  "name": "fVar16",
  "type": "float",
  "storage": "XMM0_Da:4",
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
  "name": "local_f8",
  "type": "longlong",
  "storage": "Stack[-0xf8]:8",
  "is_phantom": false,
  "in_decompiled_code": true
 },
 {
  "name": "uVar5",
  "type": "undefined4",
  "is_phantom": false,
  "in_decompiled_code": true
 },
 {
  "name": "cVar4",
  "type": "char",
  "is_phantom": false,
  "in_decompiled_code": true
 },
 {
  "name": "local_f0",
  "type": "longlong",
  "storage": "Stack[-0xf0]:8",
  "is_phantom": false,
  "in_decompiled_code": true
 },
 {
  "name": "lVar7",
  "type": "longlong",
  "is_phantom": false,
  "in_decompiled_code": true
 },
 {
  "name": "lVar6",
  "type": "longlong",
  "is_phantom": false,
  "in_decompiled_code": true
 },
 {
  "name": "local_e0",
  "type": "longlong",
  "storage": "Stack[-0xe0]:8",
  "is_phantom": false,
  "in_decompiled_code": true
 },
 {
  "name": "local_d8",
  "type": "longlong",
  "storage": "Stack[-0xd8]:8",
  "is_phantom": false,
  "in_decompiled_code": true
 },
 {
  "name": "uVar9",
  "type": "ulonglong",
  "storage": "RAX:8",
  "is_phantom": false,
  "in_decompiled_code": true
 },
 {
  "name": "lVar8",
  "type": "longlong",
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
  "name": "local_c8",
  "type": "longlong",
  "storage": "Stack[-0xc8]:8",
  "is_phantom": false,
  "in_decompiled_code": true
 },
 {
  "name": "uVar10",
  "type": "ulonglong",
  "storage": "RAX:8",
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
  "type": "undefined4 *",
  "is_phantom": false,
  "in_decompiled_code": true
 },
 {
  "name": "pfVar2",
  "type": "float *",
  "is_phantom": false,
  "in_decompiled_code": true
 },
 {
  "name": "local_b8",
  "type": "ulonglong",
  "storage": "Stack[-0xb8]:8",
  "is_phantom": false,
  "in_decompiled_code": true
 },
 {
  "name": "local_a4",
  "type": "float",
  "storage": "Stack[-0xa4]:4",
  "is_phantom": false,
  "in_decompiled_code": true
 },
 {
  "name": "local_9c",
  "type": "float",
  "storage": "Stack[-0x9c]:4",
  "is_phantom": false,
  "in_decompiled_code": true
 }
]