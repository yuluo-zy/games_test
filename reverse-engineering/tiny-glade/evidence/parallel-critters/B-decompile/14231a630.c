
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
  float fVar9;
  char extraout_AL;
  ulonglong uVar10;
  int iVar11;
  longlong *in_RCX;
  longlong lVar12;
  longlong *in_RDX;
  longlong lVar13;
  longlong lVar14;
  ulonglong uVar15;
  longlong lVar16;
  ulonglong uVar17;
  bool bVar18;
  uint *puVar19;
  float fVar20;
  undefined8 uVar21;
  float fVar23;
  undefined8 uVar24;
  float fVar25;
  float fVar26;
  longlong local_108;
  longlong local_f0;
  longlong local_e0;
  float fVar22;
  
  fVar9 = __real_3e4ccccd;
  lVar6 = *in_RCX;
  lVar7 = in_RCX[1];
  uVar4 = *(undefined4 *)((longlong)in_RCX + 0x14);
  puVar19 = *(uint **)(lVar6 + 0xf8);
  puVar1 = puVar19 + *(longlong *)(lVar6 + 0x100);
  cVar3 = *(char *)(lVar6 + 0x108);
  lVar8 = *in_RDX;
  local_e0 = 8;
  lVar13 = 0;
  local_108 = 0;
  bVar18 = false;
  uVar15 = 0;
  uVar17 = 0;
  do {
    if (cVar3 == '\0') {
      if (uVar17 == uVar15) {
        if (puVar19 == puVar1) {
          return;
        }
        while( true ) {
          uVar15 = *(ulonglong *)(*(longlong *)(lVar7 + 0x100) + 0x58 + (ulonglong)*puVar19 * 0xa0);
          if (uVar15 != 0) break;
          puVar19 = puVar19 + 1;
          if (puVar19 == puVar1) {
            return;
          }
        }
        lVar16 = *(longlong *)(lVar7 + 0x100) + (ulonglong)*puVar19 * 0xa0;
        lVar13 = (ulonglong)*(uint *)(lVar16 + 0x94) * 0x48 + *(longlong *)(lVar7 + 0x1a8);
        uVar17 = *(ulonglong *)(lVar6 + 0x110);
        if ((uVar17 < *(ulonglong *)(lVar16 + 0x88)) &&
           (*(longlong *)(*(longlong *)(lVar16 + 0x80) + uVar17 * 8) != 0)) {
          lVar12 = *(longlong *)(lVar13 + 0x18);
          lVar13 = *(longlong *)(lVar13 + 0x38);
          local_108 = *(longlong *)(lVar12 + 0x10 + ~*(ulonglong *)(lVar13 + uVar17 * 8) * 0x30);
          bVar18 = true;
        }
        else {
          lVar12 = *(longlong *)(lVar13 + 0x18);
          lVar13 = *(longlong *)(lVar13 + 0x38);
          bVar18 = false;
        }
        puVar19 = puVar19 + 1;
        lVar14 = ~*(ulonglong *)(lVar13 + *(longlong *)(lVar6 + 0x118) * 8) * 0x30;
        lVar13 = *(longlong *)(lVar12 + 0x10 + lVar14);
        local_f0 = *(longlong *)(lVar12 + 0x28 + lVar14);
        local_e0 = *(longlong *)(lVar16 + 0x50);
        uVar17 = 0;
      }
      uVar10 = (ulonglong)*(uint *)(local_e0 + 8 + uVar17 * 0x10);
    }
    else {
      if (uVar17 == uVar15) {
        if (puVar19 == puVar1) {
          return;
        }
        while( true ) {
          uVar15 = *(ulonglong *)(*(longlong *)(lVar7 + 0x1a8) + 0x10 + (ulonglong)*puVar19 * 0x48);
          if (uVar15 != 0) break;
          puVar19 = puVar19 + 1;
          if (puVar19 == puVar1) {
            return;
          }
        }
        lVar16 = *(longlong *)(lVar7 + 0x1a8) + (ulonglong)*puVar19 * 0x48;
        if ((*(ulonglong *)(lVar6 + 0x110) < *(ulonglong *)(lVar16 + 0x40)) &&
           (uVar17 = *(ulonglong *)
                      (*(longlong *)(lVar16 + 0x38) + *(ulonglong *)(lVar6 + 0x110) * 8),
           uVar17 != 0)) {
          local_108 = *(longlong *)(*(longlong *)(lVar16 + 0x18) + 0x10 + ~uVar17 * 0x30);
          bVar18 = true;
        }
        else {
          bVar18 = false;
        }
        puVar19 = puVar19 + 1;
        lVar12 = ~*(ulonglong *)(*(longlong *)(lVar16 + 0x38) + *(longlong *)(lVar6 + 0x118) * 8) *
                 0x30;
        lVar13 = *(longlong *)(*(longlong *)(lVar16 + 0x18) + 0x10 + lVar12);
        local_f0 = *(longlong *)(*(longlong *)(lVar16 + 0x18) + 0x28 + lVar12);
        uVar17 = 0;
      }
      uVar10 = uVar17 & 0xffffffff;
    }
    lVar16 = uVar10 * 0x68 + lVar13;
    puVar2 = (undefined4 *)(local_f0 + uVar10 * 4);
    if (bVar18) {
      lVar12 = uVar10 * 0x30 + local_108;
      uVar24 = *(undefined8 *)(lVar16 + 0x4c);
      fVar20 = *(float *)(lVar16 + 0x54);
      uVar21 = uVar24;
      fVar23 = fVar20;
      if (lVar12 != 0) {
        uVar21 = *(undefined8 *)(lVar12 + 0x10);
        fVar23 = *(float *)(lVar12 + 0x18);
      }
    }
    else {
      uVar24 = *(undefined8 *)(lVar16 + 0x4c);
      fVar20 = *(float *)(lVar16 + 0x54);
      uVar21 = uVar24;
      fVar23 = fVar20;
    }
    fVar22 = (float)((ulonglong)uVar21 >> 0x20);
    fVar25 = (float)uVar21 - (float)uVar24;
    fVar26 = fVar22 - (float)((ulonglong)uVar24 >> 0x20);
    *(float *)(lVar16 + 0x48) =
         SQRT((fVar23 - fVar20) * (fVar23 - fVar20) + fVar26 * fVar26 + fVar25 * fVar25) /
         *(float *)(lVar8 + 0x28);
    *puVar2 = uVar4;
    *(float *)(lVar16 + 0x4c) = (float)uVar21;
    *(float *)(lVar16 + 0x50) = fVar22;
    *(float *)(lVar16 + 0x54) = fVar23;
    iVar5 = *(int *)(lVar16 + 0x10);
    fVar20 = *(float *)(lVar16 + 0x48);
    if (fVar20 <= fVar9) {
      if (iVar5 != 1) goto LAB_14231a9f0;
      iVar11 = 2;
      fVar20 = 0.0;
    }
    else {
      iVar11 = 1;
      if (iVar5 == 1) {
LAB_14231a9f0:
        fVar20 = *(float *)(lVar16 + 0x14);
        iVar11 = iVar5;
      }
    }
    uVar17 = uVar17 + 1;
    *puVar2 = uVar4;
    *(int *)(lVar16 + 0x10) = iVar11;
    *(float *)(lVar16 + 0x14) = fVar20;
    country_core::resources::critters::sheep::Sheep::being_petted();
    if (extraout_AL == '\0') {
                    /* WARNING: Could not recover jumptable at 0x00014231aa27. Too many branches */
                    /* WARNING: Treating indirect jump as call */
      (*(code *)(&DAT_14304b0f8 + *(int *)(&DAT_14304b0f8 + (ulonglong)*(uint *)(lVar16 + 0x10) * 4)
                ))();
      return;
    }
  } while( true );
}

