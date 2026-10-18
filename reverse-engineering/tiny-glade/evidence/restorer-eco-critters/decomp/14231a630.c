// 0x14231a630 sheep_walk_animation
// SIG: void __cdecl sheep_walk_animation(void)

void __cdecl system_critters::sheep::sheep_walk_animation::sheep_walk_animation(void)

{
  uint *puVar1;
  undefined4 *puVar2;
  char cVar3;
  undefined4 uVar4;
  int iVar5;
  longlong lVar6;
  longlong lVar7;
  longlong lVar8;
  char extraout_AL;
  ulonglong uVar9;
  int iVar10;
  longlong *in_RCX;
  longlong lVar11;
  longlong *in_RDX;
  longlong lVar12;
  longlong lVar13;
  ulonglong uVar14;
  longlong lVar15;
  ulonglong uVar16;
  bool bVar17;
  uint *puVar18;
  float fVar19;
  undefined8 uVar20;
  float fVar22;
  undefined8 uVar23;
  float fVar24;
  float fVar25;
  longlong local_108;
  longlong local_f0;
  longlong local_e0;
  float fVar21;
  
  lVar6 = *in_RCX;
  lVar7 = in_RCX[1];
  uVar4 = *(undefined4 *)((longlong)in_RCX + 0x14);
  puVar18 = *(uint **)(lVar6 + 0xf8);
  puVar1 = puVar18 + *(longlong *)(lVar6 + 0x100);
  cVar3 = *(char *)(lVar6 + 0x108);
  lVar8 = *in_RDX;
  local_e0 = 8;
  lVar12 = 0;
  local_108 = 0;
  bVar17 = false;
  uVar14 = 0;
  uVar16 = 0;
  do {
    if (cVar3 == '\0') {
      if (uVar16 == uVar14) {
        if (puVar18 == puVar1) {
          return;
        }
        while( true ) {
          uVar14 = *(ulonglong *)(*(longlong *)(lVar7 + 0x100) + 0x58 + (ulonglong)*puVar18 * 0xa0);
          if (uVar14 != 0) break;
          puVar18 = puVar18 + 1;
          if (puVar18 == puVar1) {
            return;
          }
        }
        lVar15 = *(longlong *)(lVar7 + 0x100) + (ulonglong)*puVar18 * 0xa0;
        lVar12 = (ulonglong)*(uint *)(lVar15 + 0x94) * 0x48 + *(longlong *)(lVar7 + 0x1a8);
        uVar16 = *(ulonglong *)(lVar6 + 0x110);
        if ((uVar16 < *(ulonglong *)(lVar15 + 0x88)) &&
           (*(longlong *)(*(longlong *)(lVar15 + 0x80) + uVar16 * 8) != 0)) {
          lVar11 = *(longlong *)(lVar12 + 0x18);
          lVar12 = *(longlong *)(lVar12 + 0x38);
          local_108 = *(longlong *)(lVar11 + 0x10 + ~*(ulonglong *)(lVar12 + uVar16 * 8) * 0x30);
          bVar17 = true;
        }
        else {
          lVar11 = *(longlong *)(lVar12 + 0x18);
          lVar12 = *(longlong *)(lVar12 + 0x38);
          bVar17 = false;
        }
        puVar18 = puVar18 + 1;
        lVar13 = ~*(ulonglong *)(lVar12 + *(longlong *)(lVar6 + 0x118) * 8) * 0x30;
        lVar12 = *(longlong *)(lVar11 + 0x10 + lVar13);
        local_f0 = *(longlong *)(lVar11 + 0x28 + lVar13);
        local_e0 = *(longlong *)(lVar15 + 0x50);
        uVar16 = 0;
      }
      uVar9 = (ulonglong)*(uint *)(local_e0 + 8 + uVar16 * 0x10);
    }
    else {
      if (uVar16 == uVar14) {
        if (puVar18 == puVar1) {
          return;
        }
        while( true ) {
          uVar14 = *(ulonglong *)(*(longlong *)(lVar7 + 0x1a8) + 0x10 + (ulonglong)*puVar18 * 0x48);
          if (uVar14 != 0) break;
          puVar18 = puVar18 + 1;
          if (puVar18 == puVar1) {
            return;
          }
        }
        lVar15 = *(longlong *)(lVar7 + 0x1a8) + (ulonglong)*puVar18 * 0x48;
        if ((*(ulonglong *)(lVar6 + 0x110) < *(ulonglong *)(lVar15 + 0x40)) &&
           (uVar16 = *(ulonglong *)
                      (*(longlong *)(lVar15 + 0x38) + *(ulonglong *)(lVar6 + 0x110) * 8),
           uVar16 != 0)) {
          local_108 = *(longlong *)(*(longlong *)(lVar15 + 0x18) + 0x10 + ~uVar16 * 0x30);
          bVar17 = true;
        }
        else {
          bVar17 = false;
        }
        puVar18 = puVar18 + 1;
        lVar11 = ~*(ulonglong *)(*(longlong *)(lVar15 + 0x38) + *(longlong *)(lVar6 + 0x118) * 8) *
                 0x30;
        lVar12 = *(longlong *)(*(longlong *)(lVar15 + 0x18) + 0x10 + lVar11);
        local_f0 = *(longlong *)(*(longlong *)(lVar15 + 0x18) + 0x28 + lVar11);
        uVar16 = 0;
      }
      uVar9 = uVar16 & 0xffffffff;
    }
    lVar15 = uVar9 * 0x68 + lVar12;
    puVar2 = (undefined4 *)(local_f0 + uVar9 * 4);
    if (bVar17) {
      lVar11 = uVar9 * 0x30 + local_108;
      uVar23 = *(undefined8 *)(lVar15 + 0x4c);
      fVar19 = *(float *)(lVar15 + 0x54);
      uVar20 = uVar23;
      fVar22 = fVar19;
      if (lVar11 != 0) {
        uVar20 = *(undefined8 *)(lVar11 + 0x10);
        fVar22 = *(float *)(lVar11 + 0x18);
      }
    }
    else {
      uVar23 = *(undefined8 *)(lVar15 + 0x4c);
      fVar19 = *(float *)(lVar15 + 0x54);
      uVar20 = uVar23;
      fVar22 = fVar19;
    }
    fVar21 = (float)((ulonglong)uVar20 >> 0x20);
    fVar24 = (float)uVar20 - (float)uVar23;
    fVar25 = fVar21 - (float)((ulonglong)uVar23 >> 0x20);
    *(float *)(lVar15 + 0x48) =
         SQRT((fVar22 - fVar19) * (fVar22 - fVar19) + fVar25 * fVar25 + fVar24 * fVar24) /
         *(float *)(lVar8 + 0x28);
    *puVar2 = uVar4;
    *(float *)(lVar15 + 0x4c) = (float)uVar20;
    *(float *)(lVar15 + 0x50) = fVar21;
    *(float *)(lVar15 + 0x54) = fVar22;
    iVar5 = *(int *)(lVar15 + 0x10);
    fVar19 = *(float *)(lVar15 + 0x48);
    if (fVar19 <= 0.2) {
      if (iVar5 != 1) goto LAB_14231a9f0;
      iVar10 = 2;
      fVar19 = 0.0;
    }
    else {
      iVar10 = 1;
      if (iVar5 == 1) {
LAB_14231a9f0:
        fVar19 = *(float *)(lVar15 + 0x14);
        iVar10 = iVar5;
      }
    }
    uVar16 = uVar16 + 1;
    *puVar2 = uVar4;
    *(int *)(lVar15 + 0x10) = iVar10;
    *(float *)(lVar15 + 0x14) = fVar19;
    country_core::resources::critters::sheep::Sheep::being_petted();
    if (extraout_AL == '\0') {
                    // WARNING: Could not recover jumptable at 0x00014231aa27. Too many branches
                    // WARNING: Treating indirect jump as call
      (*(code *)(&DAT_14304b0f8 + *(int *)(&DAT_14304b0f8 + (ulonglong)*(uint *)(lVar15 + 0x10) * 4)
                ))();
      return;
    }
  } while( true );
}

