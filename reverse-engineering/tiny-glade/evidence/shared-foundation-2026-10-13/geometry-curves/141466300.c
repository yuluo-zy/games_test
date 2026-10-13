
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void FUN_141466300(longlong param_1,float param_2,float param_3,undefined **param_4)

{
  float fVar1;
  undefined8 uVar2;
  code *pcVar3;
  char cVar4;
  uint uVar5;
  uint uVar6;
  undefined **ppuVar7;
  undefined *puVar8;
  undefined **ppuVar9;
  float *unaff_RBX;
  longlong unaff_RBP;
  undefined **unaff_RSI;
  undefined **unaff_RDI;
  undefined **ppuVar10;
  undefined **in_R10;
  float *pfVar11;
  ulonglong uVar12;
  undefined **unaff_R14;
  undefined *unaff_R15;
  undefined *puVar13;
  float extraout_XMM0_Da;
  float fVar14;
  float fVar15;
  undefined8 extraout_XMM0_Qa;
  float fVar17;
  undefined8 uVar16;
  float fVar18;
  float fVar19;
  float fVar20;
  float fVar21;
  float unaff_XMM6_Da;
  float unaff_XMM7_Da;
  float unaff_XMM8_Da;
  float unaff_XMM8_Db;
  float unaff_XMM9_Da;
  float unaff_XMM9_Db;
  undefined **ppuStack_100;
  float fStack_f8;
  float fStack_f4;
  float *pfStack_f0;
  code *pcStack_e8;
  float *pfStack_e0;
  code *pcStack_d8;
  float *pfStack_d0;
  undefined *puStack_c8;
  undefined8 uStack_c0;
  float **ppfStack_b8;
  undefined8 uStack_b0;
  undefined8 uStack_a8;
  
  fStack_f8 = param_2;
  fStack_f4 = param_3;
  if (param_3 <= param_2) {
    pfStack_f0 = &fStack_f8;
    pcStack_e8 = FUN_14146c800;
    pfStack_e0 = &fStack_f4;
    pcStack_d8 = FUN_14146c800;
    puStack_c8 = &UNK_142c63418;
    uStack_c0 = 2;
    uStack_a8 = 0;
    ppfStack_b8 = &pfStack_f0;
    uStack_b0 = 2;
    FUN_1428d9390(&puStack_c8,&UNK_142c63438);
    ppuVar10 = param_4;
LAB_1414666b9:
    FUN_1428a9a60(in_R10,unaff_R15,unaff_RBX,4,0xc);
    puVar8 = *ppuStack_100;
    unaff_R15 = ppuStack_100[2];
    if (puVar8 == unaff_R15) {
LAB_1414666f1:
      FUN_1428a9a60(ppuStack_100,puVar8,1,4,0xc);
      unaff_R15 = ppuStack_100[2];
    }
LAB_1414664e9:
    puVar8 = ppuStack_100[1];
    *(ulonglong *)(puVar8 + (longlong)unaff_R15 * 0xc) = CONCAT44(unaff_XMM9_Db,unaff_XMM9_Da);
    *(float *)(puVar8 + (longlong)unaff_R15 * 0xc + 8) = unaff_XMM6_Da;
    puVar13 = unaff_R15 + 1;
    ppuStack_100[2] = puVar13;
    cVar4 = -1;
    if (unaff_RSI <= unaff_RDI) {
      cVar4 = unaff_RDI != unaff_RSI;
    }
    ppuVar7 = ppuStack_100;
    if (cVar4 == '\0') {
      if (unaff_RDI < unaff_R14) {
        uStack_c0 = CONCAT44(uStack_c0._4_4_,
                             *(undefined4 *)(unaff_RBP + 8 + (longlong)unaff_RDI * 0xc));
        puStack_c8 = *(undefined **)(unaff_RBP + (longlong)unaff_RDI * 0xc);
        if (*ppuStack_100 == puVar13) goto LAB_141466791;
        goto LAB_1414665bd;
      }
      goto LAB_141466849;
    }
    pfVar11 = (float *)0x2;
    if (cVar4 == '\x01') goto LAB_1414665e1;
    uVar12 = (longlong)ppuVar10 - (longlong)unaff_RDI;
    if (ppuVar10 < unaff_RDI) goto LAB_14146677f;
    if (unaff_RSI < unaff_R14) {
      if ((ulonglong)((longlong)*ppuStack_100 - (longlong)puVar13) < uVar12) goto LAB_1414667d5;
      if (ppuVar10 != unaff_RDI) goto LAB_141466554;
    }
    else {
      FUN_1428d9840(ppuVar10,unaff_R14,&UNK_142c63450);
LAB_1414667d5:
      FUN_1428a9a60(ppuVar7,puVar13,uVar12,4,0xc);
      puVar8 = ppuStack_100[1];
      puVar13 = ppuStack_100[2];
LAB_141466554:
      func_0x000142923c00(puVar8 + (longlong)puVar13 * 0xc,(longlong)unaff_RDI * 0xc + unaff_RBP,
                          uVar12 * 0xc);
      puVar13 = puVar13 + uVar12;
    }
    ppuStack_100[2] = puVar13;
    pfVar11 = (float *)(uVar12 + 2);
LAB_1414665e1:
    while( true ) {
      pfStack_f0 = pfVar11;
      if (*ppuStack_100 == puVar13) {
        FUN_1428a9a60(ppuStack_100,puVar13,1,4,0xc);
        puVar8 = ppuStack_100[1];
        puVar13 = ppuStack_100[2];
      }
      *(ulonglong *)(puVar8 + (longlong)puVar13 * 0xc) = CONCAT44(unaff_XMM8_Db,unaff_XMM8_Da);
      *(float *)(puVar8 + (longlong)puVar13 * 0xc + 8) = unaff_XMM7_Da;
      puVar13 = puVar13 + 1;
      ppuStack_100[2] = puVar13;
      if (pfVar11 == unaff_RBX) break;
      puStack_c8 = (undefined *)0x0;
      ppuVar10 = &puStack_c8;
      ppuVar7 = ppuStack_100;
      FUN_1428d9596(0,&pfStack_f0,&pfStack_d0,ppuVar10,&UNK_142c63480);
LAB_14146677f:
      FUN_1428d9850(unaff_RDI,ppuVar10,&UNK_142c63450);
LAB_141466791:
      FUN_1428a9a60(ppuVar7,puVar13,1,4,0xc);
      puVar8 = ppuStack_100[1];
      puVar13 = ppuStack_100[2];
LAB_1414665bd:
      *(undefined4 *)(puVar8 + (longlong)puVar13 * 0xc + 8) = (undefined4)uStack_c0;
      *(undefined **)(puVar8 + (longlong)puVar13 * 0xc) = puStack_c8;
      puVar13 = puVar13 + 1;
      ppuStack_100[2] = puVar13;
      pfVar11 = (float *)0x3;
    }
    return;
  }
  ppuVar10 = param_4;
  uVar5 = FUN_141467ce0();
  uVar6 = FUN_141467ce0(param_1,param_3);
  unaff_RDI = (undefined **)(ulonglong)uVar5;
  unaff_R14 = *(undefined ***)(param_1 + 0x10);
  if (unaff_RDI < unaff_R14) {
    ppuVar10 = (undefined **)((longlong)unaff_RDI + 1);
    if (unaff_R14 <= ppuVar10) goto LAB_141466816;
    ppuVar7 = (undefined **)(ulonglong)uVar6;
    uVar16 = extraout_XMM0_Qa;
    if (unaff_R14 <= ppuVar7) goto LAB_141466828;
    ppuVar9 = (undefined **)((longlong)ppuVar7 + 1);
    if (ppuVar9 < unaff_R14) {
      unaff_RBP = *(longlong *)(param_1 + 8);
      fVar1 = *(float *)(unaff_RBP + 8 + (longlong)ppuVar10 * 0xc);
      fVar19 = _DAT_142925904 - extraout_XMM0_Da;
      uVar16 = *(undefined8 *)(unaff_RBP + (longlong)unaff_RDI * 0xc);
      uVar2 = *(undefined8 *)(unaff_RBP + (longlong)ppuVar10 * 0xc);
      fVar20 = (float)uVar2;
      fVar21 = (float)((ulonglong)uVar2 >> 0x20);
      unaff_XMM9_Da = extraout_XMM0_Da * fVar20 + (float)uVar16 * fVar19;
      unaff_XMM9_Db = extraout_XMM0_Da * fVar21 + (float)((ulonglong)uVar16 >> 0x20) * fVar19;
      unaff_XMM6_Da =
           extraout_XMM0_Da * fVar1 + fVar19 * *(float *)(unaff_RBP + 8 + (longlong)unaff_RDI * 0xc)
      ;
      fVar19 = *(float *)(unaff_RBP + 8 + (longlong)ppuVar7 * 0xc);
      fVar14 = (float)extraout_XMM0_Qa;
      fVar18 = _DAT_142925904 - fVar14;
      uVar16 = *(undefined8 *)(unaff_RBP + (longlong)ppuVar9 * 0xc);
      uVar2 = *(undefined8 *)(unaff_RBP + (longlong)ppuVar7 * 0xc);
      fVar15 = (float)uVar2;
      fVar17 = (float)((ulonglong)uVar2 >> 0x20);
      unaff_RDI = (undefined **)((longlong)unaff_RDI + 2);
      if (unaff_XMM6_Da != fVar1) {
        unaff_RDI = ppuVar10;
      }
      unaff_XMM8_Da = (float)uVar16 * fVar14 + fVar18 * fVar15;
      unaff_XMM8_Db = (float)((ulonglong)uVar16 >> 0x20) * fVar14 + fVar18 * fVar17;
      if (NAN(unaff_XMM6_Da) || NAN(fVar1)) {
        unaff_RDI = ppuVar10;
      }
      if (unaff_XMM9_Db != fVar21) {
        unaff_RDI = ppuVar10;
      }
      if (unaff_XMM9_Da != fVar20) {
        unaff_RDI = ppuVar10;
      }
      if (NAN(unaff_XMM9_Da) || NAN(fVar20)) {
        unaff_RDI = ppuVar10;
      }
      unaff_XMM7_Da = *(float *)(unaff_RBP + 8 + (longlong)ppuVar9 * 0xc) * fVar14 + fVar18 * fVar19
      ;
      unaff_RSI = (undefined **)((longlong)ppuVar7 + -1);
      if (ppuVar7 == (undefined **)0x0) {
        unaff_RSI = (undefined **)0x0;
      }
      if (unaff_XMM7_Da != fVar19) {
        unaff_RSI = ppuVar7;
      }
      if (NAN(unaff_XMM7_Da) || NAN(fVar19)) {
        unaff_RSI = ppuVar7;
      }
      if (unaff_XMM8_Db != fVar17) {
        unaff_RSI = ppuVar7;
      }
      if (unaff_XMM8_Da != fVar15) {
        unaff_RSI = ppuVar7;
      }
      if (NAN(unaff_XMM8_Da) || NAN(fVar15)) {
        unaff_RSI = ppuVar7;
      }
      ppuVar10 = (undefined **)((longlong)unaff_RSI + 1);
      ppuVar7 = (undefined **)((longlong)ppuVar10 - (longlong)unaff_RDI);
      if (ppuVar10 < unaff_RDI) {
        ppuVar7 = (undefined **)0x0;
      }
      unaff_RBX = (float *)((longlong)ppuVar7 + 2);
      pfStack_f0 = (float *)0x0;
      puVar8 = *param_4;
      unaff_R15 = param_4[2];
      in_R10 = param_4;
      ppuStack_100 = param_4;
      pfStack_d0 = unaff_RBX;
      if (puVar8 + -(longlong)unaff_R15 < unaff_RBX) goto LAB_1414666b9;
      if (puVar8 == unaff_R15) goto LAB_1414666f1;
      goto LAB_1414664e9;
    }
  }
  else {
    FUN_1428d9518(unaff_RDI,unaff_R14,&UNK_142c63658);
LAB_141466816:
    uVar16 = FUN_1428d9518(ppuVar10,unaff_R14,&UNK_142c63670);
LAB_141466828:
    ppuVar9 = unaff_R14;
    FUN_1428d9518(uVar16,unaff_R14,&UNK_142c63658);
  }
  FUN_1428d9518(ppuVar9,unaff_R14,&UNK_142c63670);
LAB_141466849:
  FUN_1428d9518(unaff_RDI,unaff_R14,&UNK_142c63468);
  pcVar3 = (code *)swi(3);
  (*pcVar3)();
  return;
}

