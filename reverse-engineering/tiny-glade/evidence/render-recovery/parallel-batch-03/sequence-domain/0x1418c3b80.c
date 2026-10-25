
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void FUN_1418c3b80(ulonglong *param_1,ulonglong param_2,ulonglong param_3,ulonglong param_4)

{
  longlong *plVar1;
  ulonglong uVar2;
  longlong lVar3;
  longlong lVar4;
  longlong lVar5;
  longlong lVar6;
  ulonglong uVar7;
  ulonglong uVar8;
  ulonglong uVar9;
  ulonglong uVar10;
  ulonglong uVar11;
  ulonglong uVar12;
  ulonglong *puVar13;
  ulonglong uVar14;
  ulonglong uVar15;
  ulonglong unaff_R14;
  longlong lVar16;
  undefined4 uVar20;
  undefined1 auVar17 [16];
  longlong lVar21;
  undefined1 auVar18 [16];
  undefined1 auVar19 [16];
  longlong lVar22;
  
  uVar14 = 0;
  if (param_2 <= param_3) {
    uVar14 = param_3 - param_2;
  }
  uVar9 = param_1[3];
  if (CARRY8(uVar9,uVar14)) {
    param_1 = (ulonglong *)&UNK_142d069b0;
    param_2 = 0x11;
    FUN_1428d9330();
    param_3 = param_4;
code_r0x0001418c3c76:
    FUN_1428ca940();
    uVar15 = *param_1;
    uVar9 = param_1[3];
    uVar10 = param_1[2];
    if (unaff_R14 - uVar9 < uVar10) {
code_r0x0001418c3bd3:
      uVar11 = unaff_R14 - uVar10;
      uVar7 = uVar9 - uVar11;
      if (uVar15 - unaff_R14 < uVar7 || uVar11 <= uVar7) {
        uVar7 = uVar15 - uVar11;
        func_0x000142923c10(param_1[1] + uVar7 * 8,param_1[1] + uVar10 * 8,uVar11 * 8);
        param_1[2] = uVar7;
        uVar10 = uVar7;
      }
      else {
        func_0x000142923c00(param_1[1] + unaff_R14 * 8,param_1[1],uVar7 * 8);
      }
    }
  }
  else {
    uVar15 = *param_1;
    if (uVar15 < uVar9 + uVar14) {
      unaff_R14 = uVar15;
      if (uVar15 - uVar9 < uVar14) goto code_r0x0001418c3c76;
      uVar10 = param_1[2];
      if (uVar10 <= uVar15 - uVar9) goto code_r0x0001418c3cbf;
      goto code_r0x0001418c3bd3;
    }
    uVar10 = param_1[2];
  }
code_r0x0001418c3cbf:
  lVar6 = _UNK_1429ae328;
  lVar5 = _UNK_1429ae320;
  lVar4 = _UNK_1429ae318;
  lVar3 = _UNK_1429ae310;
  uVar7 = 0;
  if (uVar15 <= uVar10 + uVar9) {
    uVar7 = uVar15;
  }
  uVar11 = (uVar10 + uVar9) - uVar7;
  uVar12 = uVar15 - uVar11;
  uVar20 = (undefined4)(param_2 >> 0x20);
  if (uVar14 <= uVar12) {
    uVar14 = param_3 - param_2;
    if (param_3 < param_2 || uVar14 == 0) {
      uVar14 = 0;
    }
    else {
      uVar15 = param_1[1];
      if (uVar14 < 4) {
        uVar12 = 0;
        uVar10 = param_2;
      }
      else {
        uVar12 = uVar14 & 0xfffffffffffffffc;
        uVar10 = uVar12 + param_2;
        auVar18._8_4_ = (int)param_2;
        auVar18._0_8_ = param_2;
        auVar18._12_4_ = uVar20;
        lVar22 = param_2 + _UNK_1429ae300;
        lVar21 = auVar18._8_8_ + _UNK_1429ae308;
        lVar16 = uVar15 + uVar11 * 8;
        uVar7 = 0;
        do {
          plVar1 = (longlong *)(lVar16 + uVar7 * 8);
          *plVar1 = lVar22;
          plVar1[1] = lVar21;
          plVar1 = (longlong *)(lVar16 + 0x10 + uVar7 * 8);
          *plVar1 = lVar22 + lVar3;
          plVar1[1] = lVar21 + lVar4;
          uVar7 = uVar7 + 4;
          lVar22 = lVar22 + lVar5;
          lVar21 = lVar21 + lVar6;
        } while (uVar12 != uVar7);
        if (uVar14 == uVar12) goto code_r0x0001418c3f22;
      }
      puVar13 = (ulonglong *)(uVar15 + uVar11 * 8 + uVar12 * 8);
      do {
        *puVar13 = uVar10;
        uVar10 = uVar10 + 1;
        puVar13 = puVar13 + 1;
      } while (param_3 != uVar10);
    }
    goto code_r0x0001418c3f22;
  }
  uVar8 = param_2;
  if (uVar15 == uVar11) {
    uVar14 = 0;
    uVar10 = param_3 - param_2;
    uVar15 = uVar14;
    if (param_3 < param_2 || uVar10 == 0) goto code_r0x0001418c3f22;
  }
  else {
    uVar2 = param_1[1];
    uVar10 = (~uVar9 + uVar7 + uVar15) - uVar10;
    uVar15 = 0;
    uVar14 = param_3 - param_2;
    if (param_3 < param_2) {
      uVar14 = uVar15;
    }
    if (uVar10 < uVar14) {
      uVar14 = uVar10;
    }
    uVar14 = uVar14 + 1;
    if (4 < uVar14) {
      uVar15 = 4;
      if ((uVar14 & 3) != 0) {
        uVar15 = (ulonglong)((uint)uVar14 & 3);
      }
      uVar15 = uVar14 - uVar15;
      uVar8 = uVar15 + param_2;
      uVar12 = uVar12 - uVar15;
      auVar17._8_4_ = (int)param_2;
      auVar17._0_8_ = param_2;
      auVar17._12_4_ = uVar20;
      lVar22 = param_2 + _UNK_1429ae300;
      lVar21 = auVar17._8_8_ + _UNK_1429ae308;
      lVar16 = uVar2 + uVar11 * 8;
      uVar14 = 0;
      do {
        plVar1 = (longlong *)(lVar16 + uVar14 * 8);
        *plVar1 = lVar22;
        plVar1[1] = lVar21;
        plVar1 = (longlong *)(lVar16 + 0x10 + uVar14 * 8);
        *plVar1 = lVar22 + lVar3;
        plVar1[1] = lVar21 + lVar4;
        uVar14 = uVar14 + 4;
        lVar22 = lVar22 + lVar5;
        lVar21 = lVar21 + lVar6;
      } while (uVar15 != uVar14);
    }
    do {
      if (param_3 <= uVar8) {
        uVar10 = param_3 - uVar8;
        uVar14 = uVar15;
        if (param_3 < uVar8 || uVar10 == 0) goto code_r0x0001418c3f22;
        goto code_r0x0001418c3e02;
      }
      uVar7 = uVar8 + 1;
      uVar14 = uVar15 + 1;
      *(ulonglong *)(uVar2 + uVar11 * 8 + uVar15 * 8) = uVar8;
      uVar12 = uVar12 - 1;
      uVar15 = uVar14;
      uVar8 = uVar7;
    } while (uVar12 != 0);
    uVar10 = param_3 - uVar7;
    if (param_3 < uVar7 || uVar10 == 0) goto code_r0x0001418c3f22;
  }
code_r0x0001418c3e02:
  lVar6 = _UNK_1429ae328;
  lVar5 = _UNK_1429ae320;
  lVar4 = _UNK_1429ae318;
  lVar3 = _UNK_1429ae310;
  uVar14 = param_1[1];
  if (uVar10 < 4) {
    uVar12 = 0;
    uVar7 = uVar8;
code_r0x0001418c3f07:
    puVar13 = (ulonglong *)(uVar14 + uVar12 * 8);
    do {
      *puVar13 = uVar7;
      uVar7 = uVar7 + 1;
      puVar13 = puVar13 + 1;
    } while (param_3 != uVar7);
  }
  else {
    uVar12 = uVar10 & 0xfffffffffffffffc;
    auVar19._8_4_ = (int)uVar8;
    auVar19._0_8_ = uVar8;
    auVar19._12_4_ = (int)(uVar8 >> 0x20);
    lVar16 = uVar8 + _UNK_1429ae300;
    lVar22 = auVar19._8_8_ + _UNK_1429ae308;
    uVar7 = uVar8 + uVar12;
    uVar11 = 0;
    do {
      plVar1 = (longlong *)(uVar14 + uVar11 * 8);
      *plVar1 = lVar16;
      plVar1[1] = lVar22;
      plVar1 = (longlong *)(uVar14 + 0x10 + uVar11 * 8);
      *plVar1 = lVar16 + lVar3;
      plVar1[1] = lVar22 + lVar4;
      uVar11 = uVar11 + 4;
      lVar16 = lVar16 + lVar5;
      lVar22 = lVar22 + lVar6;
    } while (uVar12 != uVar11);
    if (uVar10 != uVar12) goto code_r0x0001418c3f07;
  }
  uVar14 = (uVar15 + param_3) - uVar8;
code_r0x0001418c3f22:
  param_1[3] = uVar9 + uVar14;
  return;
}

