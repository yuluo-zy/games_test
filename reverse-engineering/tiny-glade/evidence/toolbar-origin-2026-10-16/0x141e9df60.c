
void FUN_141e9df60(undefined8 *param_1,longlong *param_2)

{
  uint *puVar1;
  undefined4 uVar2;
  undefined4 uVar3;
  longlong lVar4;
  longlong lVar5;
  undefined8 uVar6;
  undefined4 *puVar7;
  undefined8 uVar8;
  undefined8 uVar9;
  ulonglong uVar10;
  ulonglong uVar11;
  longlong lVar12;
  longlong lVar13;
  longlong unaff_RBX;
  longlong unaff_RBP;
  longlong lVar14;
  ulonglong uVar15;
  longlong lVar16;
  longlong lVar17;
  longlong lVar18;
  uint *puVar19;
  
  lVar4 = *param_2;
  lVar5 = param_2[1];
  uVar2 = *(undefined4 *)((longlong)param_2 + 0x14);
  puVar19 = *(uint **)(lVar4 + 0xf8);
  puVar1 = puVar19 + *(longlong *)(lVar4 + 0x100);
  uVar3 = *(undefined4 *)((longlong)param_1 + 0x1c);
  uVar6 = *param_1;
  puVar7 = (undefined4 *)param_1[2];
  if (*(char *)(lVar4 + 0x108) == '\0') {
    lVar18 = 8;
    lVar17 = 0;
    lVar14 = 0;
    lVar16 = 0;
    lVar13 = 0;
    do {
      if (lVar13 == lVar16) {
        if (puVar19 == puVar1) {
          return;
        }
        while( true ) {
          lVar16 = *(longlong *)(*(longlong *)(lVar5 + 0x100) + 0x58 + (ulonglong)*puVar19 * 0xa0);
          if (lVar16 != 0) break;
          puVar19 = puVar19 + 1;
          if (puVar19 == puVar1) {
            return;
          }
        }
        lVar12 = *(longlong *)(lVar5 + 0x100) + (ulonglong)*puVar19 * 0xa0;
        uVar11 = (ulonglong)*(uint *)(lVar12 + 0x94);
        lVar14 = *(longlong *)(lVar5 + 0x1a8);
        lVar18 = *(longlong *)(lVar14 + 0x18 + uVar11 * 0x48);
        lVar13 = *(longlong *)(lVar14 + 0x38 + uVar11 * 0x48);
        if ((*(ulonglong *)(lVar4 + 0x118) < *(ulonglong *)(lVar14 + 0x40 + uVar11 * 0x48)) &&
           (uVar11 = *(ulonglong *)(lVar13 + *(ulonglong *)(lVar4 + 0x118) * 8), uVar11 != 0)) {
          lVar17 = *(longlong *)(lVar18 + 0x10 + ~uVar11 * 0x30);
        }
        else {
          lVar17 = 0;
        }
        lVar13 = ~*(ulonglong *)(lVar13 + *(longlong *)(lVar4 + 0x110) * 8) * 0x30;
        lVar14 = *(longlong *)(lVar18 + 0x10 + lVar13);
        unaff_RBP = *(longlong *)(lVar18 + 0x28 + lVar13);
        puVar19 = puVar19 + 1;
        lVar18 = *(longlong *)(lVar12 + 0x50);
        lVar13 = 0;
      }
      uVar11 = (ulonglong)*(uint *)(lVar18 + 8 + lVar13 * 0x10);
      lVar13 = lVar13 + 1;
      *(undefined4 *)(unaff_RBP + uVar11 * 4) = uVar2;
      uVar8 = *(undefined8 *)(lVar17 + uVar11 * 0x10);
      uVar9 = *(undefined8 *)(lVar17 + 8 + uVar11 * 0x10);
      *puVar7 = uVar3;
      FUN_140b91120(uVar11 * 600 + lVar14,uVar8,uVar9,0xffffffff,uVar6);
    } while( true );
  }
  lVar18 = 0;
  lVar14 = 0;
  uVar11 = 0;
  uVar15 = 0;
  do {
    if (uVar15 == uVar11) {
      if (puVar19 == puVar1) {
        return;
      }
      while( true ) {
        uVar11 = *(ulonglong *)(*(longlong *)(lVar5 + 0x1a8) + 0x10 + (ulonglong)*puVar19 * 0x48);
        if (uVar11 != 0) break;
        puVar19 = puVar19 + 1;
        if (puVar19 == puVar1) {
          return;
        }
      }
      lVar18 = *(longlong *)(lVar5 + 0x1a8) + (ulonglong)*puVar19 * 0x48;
      lVar16 = *(longlong *)(lVar18 + 0x18);
      lVar17 = ~*(ulonglong *)(*(longlong *)(lVar18 + 0x38) + *(longlong *)(lVar4 + 0x110) * 8) *
               0x30;
      lVar14 = *(longlong *)(lVar16 + 0x10 + lVar17);
      unaff_RBX = *(longlong *)(lVar16 + 0x28 + lVar17);
      puVar19 = puVar19 + 1;
      if ((*(ulonglong *)(lVar4 + 0x118) < *(ulonglong *)(lVar18 + 0x40)) &&
         (uVar15 = *(ulonglong *)(*(longlong *)(lVar18 + 0x38) + *(ulonglong *)(lVar4 + 0x118) * 8),
         uVar15 != 0)) {
        lVar18 = *(longlong *)(lVar16 + 0x10 + ~uVar15 * 0x30);
      }
      else {
        lVar18 = 0;
      }
      uVar15 = 0;
    }
    uVar10 = uVar15 & 0xffffffff;
    uVar15 = uVar15 + 1;
    *(undefined4 *)(unaff_RBX + uVar10 * 4) = uVar2;
    uVar8 = *(undefined8 *)(lVar18 + uVar10 * 0x10);
    uVar9 = *(undefined8 *)(lVar18 + 8 + uVar10 * 0x10);
    *puVar7 = uVar3;
    FUN_140b91120(uVar10 * 600 + lVar14,uVar8,uVar9,0xffffffff,uVar6);
  } while( true );
}

