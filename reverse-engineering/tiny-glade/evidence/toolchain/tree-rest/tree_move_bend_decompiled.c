
void __cdecl system_tree::tree_move_bend::tree_move_bend(void)

{
  uint *puVar1;
  float *pfVar2;
  undefined4 *puVar3;
  char cVar4;
  undefined4 uVar5;
  longlong lVar6;
  longlong lVar7;
  float fVar8;
  float fVar9;
  float fVar10;
  longlong lVar11;
  ulonglong uVar12;
  ulonglong uVar13;
  longlong *in_RCX;
  longlong lVar14;
  longlong lVar15;
  longlong lVar16;
  ulonglong uVar17;
  uint *puVar18;
  float extraout_XMM0_Da;
  float extraout_XMM0_Da_00;
  float fVar19;
  float fVar20;
  float fVar21;
  float fVar22;
  longlong local_f8;
  longlong local_f0;
  longlong local_e0;
  longlong local_d8;
  longlong local_c8;
  ulonglong local_b8;
  float local_a4;
  float local_9c;
  
  fVar10 = __real_40a00000;
  fVar9 = __real_3dcccccd;
  fVar8 = __real_3f800000;
  lVar6 = *in_RCX;
  lVar7 = in_RCX[1];
  uVar5 = *(undefined4 *)((longlong)in_RCX + 0x14);
  puVar18 = *(uint **)(lVar6 + 0xf8);
  puVar1 = puVar18 + *(longlong *)(lVar6 + 0x100);
  cVar4 = *(char *)(lVar6 + 0x108);
  local_c8 = 8;
  local_f8 = 0;
  local_f0 = 0;
  uVar17 = 0;
  local_b8 = 0;
  do {
    if (cVar4 == '\0') {
      if (local_b8 == uVar17) {
        if (puVar18 == puVar1) {
          return;
        }
        while( true ) {
          uVar17 = *(ulonglong *)(*(longlong *)(lVar7 + 0x100) + 0x58 + (ulonglong)*puVar18 * 0xa0);
          if (uVar17 != 0) break;
          puVar18 = puVar18 + 1;
          if (puVar18 == puVar1) {
            return;
          }
        }
        lVar14 = *(longlong *)(lVar7 + 0x100) + (ulonglong)*puVar18 * 0xa0;
        uVar12 = (ulonglong)*(uint *)(lVar14 + 0x94);
        lVar11 = *(longlong *)(*(longlong *)(lVar7 + 0x1a8) + 0x18 + uVar12 * 0x48);
        lVar15 = *(longlong *)(*(longlong *)(lVar7 + 0x1a8) + 0x38 + uVar12 * 0x48);
        lVar16 = ~*(ulonglong *)(lVar15 + *(longlong *)(lVar6 + 0x110) * 8) * 0x30;
        local_f0 = *(longlong *)(lVar11 + 0x10 + lVar16);
        local_d8 = *(longlong *)(lVar11 + 0x28 + lVar16);
        lVar15 = ~*(ulonglong *)(lVar15 + *(longlong *)(lVar6 + 0x118) * 8) * 0x30;
        local_f8 = *(longlong *)(lVar11 + 0x10 + lVar15);
        local_e0 = *(longlong *)(lVar11 + 0x28 + lVar15);
        local_c8 = *(longlong *)(lVar14 + 0x50);
        puVar18 = puVar18 + 1;
        local_b8 = 0;
      }
      uVar12 = (ulonglong)*(uint *)(local_c8 + 8 + local_b8 * 0x10);
    }
    else {
      if (local_b8 == uVar17) {
        if (puVar18 == puVar1) {
          return;
        }
        while( true ) {
          uVar17 = *(ulonglong *)(*(longlong *)(lVar7 + 0x1a8) + 0x10 + (ulonglong)*puVar18 * 0x48);
          if (uVar17 != 0) break;
          puVar18 = puVar18 + 1;
          if (puVar18 == puVar1) {
            return;
          }
        }
        lVar11 = *(longlong *)(lVar7 + 0x1a8) + (ulonglong)*puVar18 * 0x48;
        lVar15 = *(longlong *)(lVar11 + 0x18);
        lVar11 = *(longlong *)(lVar11 + 0x38);
        lVar14 = ~*(ulonglong *)(lVar11 + *(longlong *)(lVar6 + 0x110) * 8) * 0x30;
        local_f0 = *(longlong *)(lVar15 + 0x10 + lVar14);
        local_d8 = *(longlong *)(lVar15 + 0x28 + lVar14);
        lVar11 = ~*(ulonglong *)(lVar11 + *(longlong *)(lVar6 + 0x118) * 8) * 0x30;
        local_f8 = *(longlong *)(lVar15 + 0x10 + lVar11);
        local_e0 = *(longlong *)(lVar15 + 0x28 + lVar11);
        puVar18 = puVar18 + 1;
        local_b8 = 0;
      }
      uVar12 = local_b8 & 0xffffffff;
    }
    pfVar2 = (float *)(local_f0 + uVar12 * 8);
    puVar3 = (undefined4 *)(local_d8 + uVar12 * 4);
    lVar11 = uVar12 * 0x30 + local_f8;
    country_core::resources::placement_data::PlacementDataSsbo::position();
    ::utils::time_continuous_lerp();
    *puVar3 = uVar5;
    *pfVar2 = extraout_XMM0_Da;
    ::utils::time_continuous_lerp();
    *puVar3 = uVar5;
    pfVar2[1] = extraout_XMM0_Da_00;
    fVar21 = *pfVar2 - local_a4;
    fVar22 = extraout_XMM0_Da_00 - local_9c;
    fVar19 = SQRT(fVar22 * fVar22 + fVar21 * fVar21);
    uVar13 = 0;
    fVar20 = 0.0;
    if (fVar9 <= fVar19) {
      fVar20 = fVar10;
      if (fVar19 <= fVar10) {
        fVar20 = fVar19;
      }
      *puVar3 = uVar5;
      fVar21 = fVar20 * fVar21 * (fVar8 / fVar19);
      fVar20 = fVar20 * fVar22 * (fVar8 / fVar19);
      uVar13 = (ulonglong)(uint)fVar21;
      *(ulonglong *)pfVar2 = CONCAT44(local_9c + fVar20,local_a4 + fVar21);
    }
    local_b8 = local_b8 + 1;
    if ((((*(float *)(lVar11 + 0x24) != (float)uVar13) ||
         (NAN(*(float *)(lVar11 + 0x24)) || NAN((float)uVar13))) ||
        (*(float *)(lVar11 + 0x28) != fVar20)) || (NAN(*(float *)(lVar11 + 0x28)) || NAN(fVar20))) {
      *(undefined4 *)(local_e0 + uVar12 * 4) = uVar5;
      *(ulonglong *)(lVar11 + 0x24) = uVar13 | (ulonglong)(uint)fVar20 << 0x20;
    }
  } while( true );
}

