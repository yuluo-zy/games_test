
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void FUN_1411f8130(longlong param_1,float param_2,float param_3,longlong *param_4)

{
  undefined8 *puVar1;
  undefined4 *puVar2;
  undefined4 *puVar3;
  undefined8 uVar4;
  undefined8 uVar5;
  code *pcVar6;
  undefined4 uVar7;
  undefined4 uVar8;
  undefined4 uVar9;
  uint uVar10;
  uint uVar11;
  longlong lVar12;
  ulonglong uVar13;
  char cVar14;
  ulonglong uVar15;
  longlong lVar16;
  float *unaff_RBX;
  ulonglong unaff_RBP;
  ulonglong uVar17;
  ulonglong unaff_RDI;
  longlong lVar18;
  longlong lVar19;
  longlong *in_R11;
  float *pfVar20;
  ulonglong uVar21;
  longlong unaff_R13;
  ulonglong unaff_R14;
  ulonglong unaff_R15;
  float extraout_XMM0_Da;
  float extraout_XMM0_Da_00;
  float fVar22;
  float fVar23;
  float fVar24;
  float fVar25;
  float unaff_XMM6_Da;
  float unaff_XMM6_Db;
  float unaff_XMM7_Da;
  float unaff_XMM7_Db;
  longlong *plStack_d0;
  float fStack_c8;
  float fStack_c4;
  float *pfStack_c0;
  float *pfStack_b8;
  code *pcStack_b0;
  float *pfStack_a8;
  code *pcStack_a0;
  undefined *puStack_98;
  undefined8 uStack_90;
  undefined4 **ppuStack_88;
  undefined8 uStack_80;
  undefined8 uStack_78;
  
  fStack_c8 = param_2;
  fStack_c4 = param_3;
  if (param_3 <= param_2) {
    pfStack_b8 = &fStack_c8;
    pcStack_b0 = FUN_1411fb490;
    pfStack_a8 = &fStack_c4;
    pcStack_a0 = FUN_1411fb490;
    puStack_98 = &UNK_142c18c80;
    uStack_90 = 2;
    uStack_78 = 0;
    ppuStack_88 = &pfStack_b8;
    uStack_80 = 2;
    FUN_1428d9390(&puStack_98,&UNK_142c18ca0);
LAB_1411f840b:
    FUN_1428a7de0(in_R11);
    lVar12 = *plStack_d0;
    lVar16 = plStack_d0[2];
    if (lVar12 == lVar16) {
LAB_1411f843a:
      FUN_1428a7de0(plStack_d0,lVar12,1,4,8);
      lVar16 = plStack_d0[2];
    }
LAB_1411f8297:
    lVar12 = plStack_d0[1];
    *(ulonglong *)(lVar12 + lVar16 * 8) = CONCAT44(unaff_XMM7_Db,unaff_XMM7_Da);
    lVar16 = lVar16 + 1;
    plStack_d0[2] = lVar16;
    uVar17 = unaff_RBP - unaff_RDI;
    cVar14 = -1;
    if (unaff_RBP < unaff_RDI || uVar17 == 0) {
      cVar14 = uVar17 != 0;
    }
    if (cVar14 == '\0') {
      if (unaff_RDI < unaff_R15) {
        uVar4 = *(undefined8 *)(unaff_R13 + unaff_RDI * 8);
        unaff_XMM7_Da = (float)uVar4;
        unaff_XMM7_Db = (float)((ulonglong)uVar4 >> 0x20);
        if (*plStack_d0 == lVar16) goto LAB_1411f84d4;
        goto LAB_1411f834a;
      }
      goto LAB_1411f8622;
    }
    pfVar20 = (float *)0x2;
    if (cVar14 == '\x01') goto LAB_1411f835b;
    uVar21 = unaff_R14 - unaff_RDI;
    if (unaff_R14 < unaff_RDI) goto LAB_1411f84c2;
    if (unaff_RBP < unaff_R15) {
      if ((ulonglong)(*plStack_d0 - lVar16) < uVar21) goto LAB_1411f8515;
      if (unaff_R14 != unaff_RDI) {
        if (0x13 < uVar21) goto LAB_1411f8546;
        goto LAB_1411f8305;
      }
    }
    else {
      FUN_1428d9840(unaff_R14,unaff_R15,&UNK_142c18cb8);
LAB_1411f8515:
      FUN_1428a7de0();
      lVar12 = plStack_d0[1];
      lVar16 = plStack_d0[2];
      if (uVar21 < 0x14) {
LAB_1411f8305:
        uVar15 = 0;
      }
      else {
LAB_1411f8546:
        uVar13 = lVar12 + lVar16 * 8;
        uVar15 = 0;
        if (((uVar13 <= uVar13 + uVar17 * 8) &&
            (lVar12 + lVar16 * 8 + 4U <= uVar13 + uVar17 * 8 + 4)) && (uVar17 >> 0x3d == 0)) {
          lVar19 = lVar16 * 8;
          uVar15 = uVar21 & 0x1fffffffc;
          lVar16 = lVar16 + uVar15;
          lVar18 = unaff_RDI * 8 + 0x10 + unaff_R13;
          lVar19 = lVar19 + lVar12;
          uVar17 = 0;
          do {
            puVar2 = (undefined4 *)(lVar18 + -0x10 + uVar17 * 8);
            uVar7 = puVar2[1];
            uVar8 = puVar2[2];
            uVar9 = puVar2[3];
            puVar1 = (undefined8 *)(lVar18 + uVar17 * 8);
            uVar4 = *puVar1;
            uVar5 = puVar1[1];
            puVar3 = (undefined4 *)(lVar19 + uVar17 * 8);
            *puVar3 = *puVar2;
            puVar3[1] = uVar7;
            puVar3[2] = uVar8;
            puVar3[3] = uVar9;
            puVar1 = (undefined8 *)(lVar19 + 0x10 + uVar17 * 8);
            *puVar1 = uVar4;
            puVar1[1] = uVar5;
            uVar17 = uVar17 + 4;
          } while (uVar15 != uVar17);
          if (uVar21 == uVar15) goto LAB_1411f8327;
        }
      }
      unaff_RDI = unaff_RDI + uVar15;
      do {
        *(undefined8 *)(lVar12 + lVar16 * 8) = *(undefined8 *)(unaff_R13 + unaff_RDI * 8);
        lVar16 = lVar16 + 1;
        unaff_RDI = unaff_RDI + 1;
      } while (unaff_R14 != unaff_RDI);
    }
LAB_1411f8327:
    plStack_d0[2] = lVar16;
    pfVar20 = (float *)(uVar21 + 2);
LAB_1411f835b:
    while( true ) {
      pfStack_b8 = pfVar20;
      if (*plStack_d0 == lVar16) {
        FUN_1428a7de0(plStack_d0,lVar16,1,4,8);
        lVar12 = plStack_d0[1];
        lVar16 = plStack_d0[2];
      }
      *(ulonglong *)(lVar12 + lVar16 * 8) = CONCAT44(unaff_XMM6_Db,unaff_XMM6_Da);
      plStack_d0[2] = lVar16 + 1;
      if (pfVar20 == unaff_RBX) break;
      puStack_98 = (undefined *)0x0;
      FUN_1428d9596(0,&pfStack_b8,&pfStack_c0,&puStack_98,&UNK_142c18ce8);
LAB_1411f84c2:
      FUN_1428d9850(unaff_RDI,unaff_R14,&UNK_142c18cb8);
LAB_1411f84d4:
      FUN_1428a7de0();
      lVar12 = plStack_d0[1];
      lVar16 = plStack_d0[2];
LAB_1411f834a:
      *(ulonglong *)(lVar12 + lVar16 * 8) = CONCAT44(unaff_XMM7_Db,unaff_XMM7_Da);
      lVar16 = lVar16 + 1;
      plStack_d0[2] = lVar16;
      pfVar20 = (float *)0x3;
    }
    return;
  }
  uVar10 = FUN_1411f8c10();
  uVar11 = FUN_1411f8c10(param_1,param_3);
  unaff_R15 = *(ulonglong *)(param_1 + 0x10);
  unaff_RDI = (ulonglong)uVar10;
  if (unaff_RDI < unaff_R15) {
    uVar17 = unaff_RDI + 1;
    if (unaff_R15 <= uVar17) goto LAB_1411f85ef;
    uVar21 = (ulonglong)uVar11;
    fVar22 = extraout_XMM0_Da_00;
    if (unaff_R15 <= uVar21) goto LAB_1411f8601;
    uVar13 = uVar21 + 1;
    if (uVar13 < unaff_R15) {
      unaff_R13 = *(longlong *)(param_1 + 8);
      uVar4 = *(undefined8 *)(unaff_R13 + unaff_RDI * 8);
      uVar5 = *(undefined8 *)(unaff_R13 + 8 + unaff_RDI * 8);
      fVar24 = (float)uVar5;
      fVar25 = (float)((ulonglong)uVar5 >> 0x20);
      unaff_XMM7_Da = extraout_XMM0_Da * fVar24 + (_DAT_142925904 - extraout_XMM0_Da) * (float)uVar4
      ;
      unaff_XMM7_Db =
           extraout_XMM0_Da * fVar25 +
           (_DAT_142925904 - extraout_XMM0_Da) * (float)((ulonglong)uVar4 >> 0x20);
      uVar4 = *(undefined8 *)(unaff_R13 + uVar21 * 8);
      fVar22 = (float)uVar4;
      fVar23 = (float)((ulonglong)uVar4 >> 0x20);
      uVar4 = *(undefined8 *)(unaff_R13 + 8 + uVar21 * 8);
      unaff_RDI = unaff_RDI + 2;
      if (unaff_XMM7_Db != fVar25) {
        unaff_RDI = uVar17;
      }
      if (unaff_XMM7_Da != fVar24) {
        unaff_RDI = uVar17;
      }
      unaff_XMM6_Da =
           extraout_XMM0_Da_00 * (float)uVar4 + (_DAT_142925904 - extraout_XMM0_Da_00) * fVar22;
      unaff_XMM6_Db =
           extraout_XMM0_Da_00 * (float)((ulonglong)uVar4 >> 0x20) +
           (_DAT_142925904 - extraout_XMM0_Da_00) * fVar23;
      if (NAN(unaff_XMM7_Da) || NAN(fVar24)) {
        unaff_RDI = uVar17;
      }
      unaff_RBP = uVar21 - 1;
      if (uVar21 == 0) {
        unaff_RBP = 0;
      }
      if (unaff_XMM6_Db != fVar23) {
        unaff_RBP = uVar21;
      }
      if (unaff_XMM6_Da != fVar22) {
        unaff_RBP = uVar21;
      }
      if (NAN(unaff_XMM6_Da) || NAN(fVar22)) {
        unaff_RBP = uVar21;
      }
      unaff_R14 = unaff_RBP + 1;
      uVar17 = unaff_R14 - unaff_RDI;
      if (unaff_R14 < unaff_RDI) {
        uVar17 = 0;
      }
      unaff_RBX = (float *)(uVar17 + 2);
      lVar12 = *param_4;
      lVar16 = param_4[2];
      in_R11 = param_4;
      plStack_d0 = param_4;
      pfStack_c0 = unaff_RBX;
      if ((float *)(lVar12 - lVar16) < unaff_RBX) goto LAB_1411f840b;
      if (lVar12 == lVar16) goto LAB_1411f843a;
      goto LAB_1411f8297;
    }
  }
  else {
    uVar17 = unaff_R15;
    FUN_1428d9518(unaff_RDI,unaff_R15,&UNK_142c18e48);
LAB_1411f85ef:
    fVar22 = (float)FUN_1428d9518(uVar17,unaff_R15,&UNK_142c18e60);
LAB_1411f8601:
    uVar13 = FUN_1428d9518(fVar22,unaff_R15,&UNK_142c18e48);
  }
  FUN_1428d9518(uVar13,unaff_R15,&UNK_142c18e60);
LAB_1411f8622:
  FUN_1428d9518(unaff_RDI,unaff_R15,&UNK_142c18cd0);
  pcVar6 = (code *)swi(3);
  (*pcVar6)();
  return;
}

