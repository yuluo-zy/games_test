
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void FUN_140ad1fe0(ulonglong *param_1,undefined8 *param_2,ulonglong param_3)

{
  ulonglong *puVar1;
  longlong lVar2;
  code *pcVar3;
  float fVar4;
  float fVar5;
  float fVar6;
  undefined8 *puVar7;
  undefined8 *puVar8;
  ulonglong uVar9;
  undefined8 *puVar10;
  ulonglong uVar11;
  undefined8 **ppuVar12;
  ulonglong uVar13;
  ulonglong uVar14;
  undefined8 *puVar15;
  ulonglong uVar16;
  undefined8 uVar17;
  undefined8 *unaff_R13;
  bool bVar18;
  float fVar19;
  float fVar20;
  float fVar21;
  float fVar22;
  float fVar23;
  float fVar24;
  float fVar25;
  float fVar26;
  float fVar27;
  float fVar28;
  float fVar29;
  float fVar30;
  undefined8 *puStack_f0;
  longlong lStack_e8;
  undefined8 *puStack_e0;
  undefined8 *puStack_d8;
  undefined8 *puStack_d0;
  undefined8 *puStack_c8;
  undefined8 *puStack_c0;
  ulonglong uStack_b8;
  undefined8 uStack_b0;
  
  uStack_b0 = 0xfffffffffffffffe;
  if (param_3 < 3) {
    uVar14 = param_3 * 8;
    if (param_3 == 0) {
      uVar11 = 4;
      puVar10 = (undefined8 *)0x8;
      uVar13 = 0;
LAB_140ad24a7:
      func_0x000142923c00(uVar11,param_2,uVar14);
      *param_1 = uVar13;
      param_1[1] = (ulonglong)puVar10;
      param_1[2] = uVar13;
      param_1[3] = param_3;
      param_1[4] = uVar11;
      param_1[5] = param_3;
      return;
    }
    puVar10 = (undefined8 *)func_0x000140613c10(uVar14,8);
    if (puVar10 == (undefined8 *)0x0) {
      FUN_1428d8fe3(8,uVar14,&UNK_142ae0e70);
    }
    else {
      *puVar10 = 0;
      puVar10[1] = 1;
      uVar11 = func_0x000140613c10(uVar14,4);
      unaff_R13 = puVar10;
      uVar13 = param_3;
      if (uVar11 != 0) goto LAB_140ad24a7;
    }
    puStack_d8 = unaff_R13;
    uStack_b8 = uVar14;
    FUN_1428d8fe3(4,uVar14,&UNK_142ae0f20);
    goto LAB_140ad26c5;
  }
  puStack_f0 = (undefined8 *)0x0;
  lStack_e8 = 8;
  puStack_e0 = (undefined8 *)0x0;
  puStack_d0 = (undefined8 *)0x0;
  puStack_c8 = (undefined8 *)0x8;
  puStack_c0 = (undefined8 *)0x0;
  FUN_140b81df0(&puStack_d0,&UNK_142ae0fe0);
  puVar10 = puStack_e0;
  *puStack_c8 = 0;
  puStack_c0 = (undefined8 *)0x1;
  if (puStack_e0 == puStack_f0) {
    FUN_140562db0(&puStack_f0,&UNK_142ae0ff8);
  }
  *(undefined8 *)(lStack_e8 + (longlong)puVar10 * 0x18) = 0;
  *(ulonglong *)(lStack_e8 + 8 + (longlong)puVar10 * 0x18) = param_3 - 1;
  *(undefined1 *)(lStack_e8 + 0x10 + (longlong)puVar10 * 0x18) = 0;
  fVar6 = _DAT_142a7baa0;
  fVar5 = _DAT_142925984;
  fVar4 = _DAT_142925904;
  puStack_e0 = (undefined8 *)((longlong)puVar10 + 1);
  puVar10 = param_2 + (param_3 - 1);
  if (puVar10 == (undefined8 *)0x0) {
    FUN_1428d9310(&UNK_142ae1010);
    goto LAB_140ad26c5;
  }
  fVar19 = (float)*param_2 - (float)*puVar10;
  fVar21 = (float)((ulonglong)*param_2 >> 0x20) - (float)((ulonglong)*puVar10 >> 0x20);
  fVar19 = SQRT(fVar21 * fVar21 + fVar19 * fVar19);
  uStack_b8 = CONCAT44(uStack_b8._4_4_,
                       (int)CONCAT71((int7)((ulonglong)puVar10 >> 8),fVar19 < _DAT_142ad4950));
  while (puVar10 = puStack_e0, puStack_e0 != (undefined8 *)0x0) {
    while( true ) {
      puVar7 = puStack_c0;
      puStack_e0 = (undefined8 *)((longlong)puVar10 + -1);
      if (*(char *)(lStack_e8 + 0x10 + (longlong)puStack_e0 * 0x18) == '\x02') goto LAB_140ad2434;
      puVar1 = (ulonglong *)(lStack_e8 + (longlong)puStack_e0 * 0x18);
      uVar14 = *(ulonglong *)(lStack_e8 + 8 + (longlong)puStack_e0 * 0x18);
      uVar13 = *puVar1;
      if (2 < (uVar14 - uVar13) + 1) break;
      if (puStack_c0 == puStack_d0) {
        FUN_140b81df0(&puStack_d0,&UNK_142ae10b8);
      }
      puStack_c8[(longlong)puVar7] = uVar14;
      puStack_c0 = (undefined8 *)((longlong)puVar7 + 1);
      puVar10 = puStack_e0;
      if (puStack_e0 == (undefined8 *)0x0) goto LAB_140ad2434;
    }
    if (param_3 <= uVar13) {
      FUN_1428d9518(uVar13,param_3,&UNK_142ae1028);
      goto LAB_140ad26c5;
    }
    if (param_3 <= uVar14) {
      FUN_1428d9518(uVar14,param_3,&UNK_142ae1040);
      goto LAB_140ad26c5;
    }
    if (uVar14 < uVar13) {
      FUN_1428d9850(uVar13,uVar14,&UNK_142ae1058);
      goto LAB_140ad26c5;
    }
    if (param_2 + uVar13 + 1 == param_2 + uVar14 || uVar14 * 8 + uVar13 * -8 == 0) {
      fVar27 = 0.0;
      uVar11 = 0;
    }
    else {
      fVar23 = (float)param_2[uVar14];
      fVar24 = (float)((ulonglong)param_2[uVar14] >> 0x20);
      fVar20 = (float)param_2[uVar13];
      fVar25 = fVar23 - fVar20;
      fVar22 = (float)((ulonglong)param_2[uVar13] >> 0x20);
      fVar26 = fVar24 - fVar22;
      fVar21 = fVar26 * fVar26 + fVar25 * fVar25;
      if (fVar21 <= fVar6) {
        fVar21 = fVar6;
      }
      fVar27 = 0.0;
      uVar11 = 0;
      uVar16 = 1;
      do {
        fVar28 = (float)param_2[uVar13 + uVar16];
        fVar29 = (float)((ulonglong)param_2[uVar13 + uVar16] >> 0x20);
        fVar30 = ((fVar29 - fVar22) * fVar26 + (fVar28 - fVar20) * fVar25) / fVar21;
        fVar28 = fVar28 - (fVar30 * fVar23 + (fVar4 - fVar30) * fVar20);
        fVar29 = fVar29 - (fVar30 * fVar24 + (fVar4 - fVar30) * fVar22);
        fVar28 = SQRT(fVar29 * fVar29 + fVar28 * fVar28);
        if (0x7f7fffff < (uint)ABS(fVar28)) {
          FUN_1428d9430(&UNK_142ae10d0,0x26,&UNK_142ae10f8);
          goto LAB_140ad26c5;
        }
        uVar9 = uVar16;
        if (fVar28 <= fVar27) {
          fVar28 = fVar27;
          uVar9 = uVar11;
        }
        uVar11 = uVar9;
        fVar27 = fVar28;
        bVar18 = uVar16 != (ulonglong)
                           ((longlong)(param_2 + uVar14) - (longlong)(param_2 + uVar13 + 1)) >> 3;
        uVar16 = uVar16 + 1;
      } while (bVar18);
    }
    if ((fVar5 < fVar27) || ((fVar19 < fVar27 & (byte)uStack_b8) != 0)) {
      uVar14 = uVar11 + uVar13;
      *puVar1 = uVar14;
      *(undefined1 *)(puVar1 + 2) = 0;
      puStack_e0 = puVar10;
      if (uVar11 < 3) {
        if (puStack_c0 == puStack_d0) {
          FUN_140b81df0(&puStack_d0,&UNK_142ae1088);
        }
        puStack_c8[(longlong)puVar7] = uVar14;
        uStack_b8 = uStack_b8 & 0xffffffff00000000;
        goto LAB_140ad241d;
      }
      if (puVar10 == puStack_f0) {
        FUN_140562db0(&puStack_f0,&UNK_142ae10a0);
      }
      *(ulonglong *)(lStack_e8 + (longlong)puVar10 * 0x18) = uVar13;
      *(ulonglong *)(lStack_e8 + 8 + (longlong)puVar10 * 0x18) = uVar14;
      *(undefined1 *)(lStack_e8 + 0x10 + (longlong)puVar10 * 0x18) = 0;
      uStack_b8 = uStack_b8 & 0xffffffff00000000;
      ppuVar12 = &puStack_e0;
    }
    else {
      if (puStack_c0 == puStack_d0) {
        FUN_140b81df0(&puStack_d0,&UNK_142ae1070);
      }
      puStack_c8[(longlong)puVar7] = uVar14;
LAB_140ad241d:
      ppuVar12 = &puStack_c0;
      puVar10 = puVar7;
    }
    *ppuVar12 = (undefined8 *)((longlong)puVar10 + 1);
  }
LAB_140ad2434:
  puVar7 = puStack_c0;
  puVar10 = puStack_c8;
  uVar14 = (longlong)puStack_c0 * 8;
  uVar17 = 0;
  if (((ulonglong)puStack_c0 >> 0x3d == 0) && (uVar14 < 0x7ffffffffffffff9)) {
    if (uVar14 == 0) {
      uVar13 = 8;
      puStack_d8 = (undefined8 *)0x0;
    }
    else {
      uVar17 = 8;
      uVar13 = func_0x000140613c10(uVar14,8);
      if (uVar13 == 0) goto LAB_140ad262d;
      puStack_d8 = puVar7;
    }
    func_0x000142923c00(uVar13,puVar10,uVar14);
    puVar8 = puStack_c0;
    puVar10 = puStack_c8;
    if (puStack_c0 == (undefined8 *)0x0) {
      uVar14 = 4;
    }
    else {
      lVar2 = (longlong)puStack_c0 * 8;
      uVar14 = func_0x000140613c10(lVar2,4);
      if (uVar14 == 0) {
        FUN_1428d8fe3(4,lVar2,&UNK_142ae0e70);
        goto LAB_140ad26c5;
      }
      puVar15 = (undefined8 *)0x0;
      do {
        uVar11 = puVar10[(longlong)puVar15];
        uStack_b8 = uVar14;
        if (param_3 <= uVar11) {
          FUN_1428d9518(uVar11,param_3,&UNK_142c64fd0);
          goto LAB_140ad26c5;
        }
        *(undefined8 *)(uVar14 + (longlong)puVar15 * 8) = param_2[uVar11];
        puVar15 = (undefined8 *)((longlong)puVar15 + 1);
      } while (puVar8 != puVar15);
    }
    *param_1 = (ulonglong)puStack_d8;
    param_1[1] = uVar13;
    param_1[2] = (ulonglong)puVar7;
    param_1[3] = (ulonglong)puVar8;
    param_1[4] = uVar14;
    param_1[5] = (ulonglong)puVar8;
    if (puStack_d0 != (undefined8 *)0x0) {
      func_0x000140613c20(puStack_c8,(longlong)puStack_d0 << 3,8);
    }
    if (puStack_f0 == (undefined8 *)0x0) {
      return;
    }
    func_0x000140613c20(lStack_e8,(longlong)puStack_f0 * 0x18,8);
    return;
  }
LAB_140ad262d:
  FUN_1428d8fe3(uVar17,uVar14,&UNK_142c38b78);
LAB_140ad26c5:
                    /* WARNING: Does not return */
  pcVar3 = (code *)invalidInstructionException();
  (*pcVar3)();
}

