
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

undefined8 FUN_140154b60(undefined8 param_1,longlong param_2,float param_3,float param_4)

{
  undefined8 *puVar1;
  undefined4 *puVar2;
  undefined4 *puVar3;
  undefined8 uVar4;
  undefined8 uVar5;
  longlong lVar6;
  code *pcVar7;
  undefined4 uVar8;
  undefined4 uVar9;
  undefined4 uVar10;
  uint uVar11;
  uint uVar12;
  char cVar13;
  ulonglong uVar14;
  ulonglong uVar15;
  float *pfVar16;
  ulonglong uVar17;
  float *pfVar18;
  ulonglong uVar19;
  longlong lVar20;
  ulonglong uVar21;
  longlong lVar22;
  ulonglong uVar23;
  float extraout_XMM0_Da;
  float extraout_XMM0_Da_00;
  float fVar24;
  float fVar25;
  float fVar26;
  float fVar27;
  float fVar28;
  float fVar29;
  float fVar30;
  undefined *puStack_f8;
  undefined8 uStack_f0;
  undefined4 **ppuStack_e8;
  undefined8 uStack_e0;
  undefined8 uStack_d8;
  float *pfStack_c8;
  code *pcStack_c0;
  float *pfStack_b8;
  code *pcStack_b0;
  float *pfStack_a8;
  undefined8 uStack_a0;
  float fStack_98;
  float fStack_94;
  longlong lStack_90;
  longlong lStack_88;
  longlong lStack_80;
  undefined1 uStack_71;
  undefined8 uStack_70;
  
  uStack_70 = 0xfffffffffffffffe;
  lStack_90 = 0;
  lStack_88 = 4;
  lStack_80 = 0;
  uStack_a0 = param_1;
  fStack_98 = param_4;
  fStack_94 = param_3;
  if (param_4 <= param_3) {
    pfStack_c8 = &fStack_94;
    pcStack_c0 = FUN_140155380;
    pfStack_b8 = &fStack_98;
    pcStack_b0 = FUN_140155380;
    puStack_f8 = &UNK_14295a1e8;
    uStack_f0 = 2;
    uStack_d8 = 0;
    ppuStack_e8 = &pfStack_c8;
    uStack_e0 = 2;
    uStack_71 = 1;
    FUN_1428d9390(&puStack_f8,&UNK_14295a208);
    goto LAB_1401550a0;
  }
  uStack_71 = 1;
  uVar11 = FUN_140153380(param_2,param_3);
  uStack_71 = 1;
  uVar12 = FUN_140153380(param_2,param_4);
  uVar21 = *(ulonglong *)(param_2 + 0x10);
  uVar17 = (ulonglong)uVar11;
  if (uVar21 <= uVar17) {
    uStack_71 = 1;
    FUN_1428d9518(uVar17,uVar21,&UNK_14295a380);
    goto LAB_1401550a0;
  }
  uVar15 = uVar17 + 1;
  if (uVar21 <= uVar15) {
    uStack_71 = 1;
    FUN_1428d9518(uVar15,uVar21,&UNK_14295a398);
    goto LAB_1401550a0;
  }
  uVar14 = (ulonglong)uVar12;
  if (uVar21 <= uVar14) {
    uStack_71 = 1;
    FUN_1428d9518(uVar14,uVar21,&UNK_14295a380);
    goto LAB_1401550a0;
  }
  if (uVar21 <= uVar14 + 1) {
    uStack_71 = 1;
    FUN_1428d9518(uVar14 + 1,uVar21,&UNK_14295a398);
    goto LAB_1401550a0;
  }
  lVar6 = *(longlong *)(param_2 + 8);
  uVar4 = *(undefined8 *)(lVar6 + uVar17 * 8);
  uVar5 = *(undefined8 *)(lVar6 + 8 + uVar17 * 8);
  fVar26 = (float)uVar5;
  fVar27 = (float)((ulonglong)uVar5 >> 0x20);
  fVar29 = extraout_XMM0_Da * fVar26 + (_DAT_142925904 - extraout_XMM0_Da) * (float)uVar4;
  fVar30 = extraout_XMM0_Da * fVar27 +
           (_DAT_142925904 - extraout_XMM0_Da) * (float)((ulonglong)uVar4 >> 0x20);
  uVar4 = *(undefined8 *)(lVar6 + uVar14 * 8);
  fVar24 = (float)uVar4;
  fVar25 = (float)((ulonglong)uVar4 >> 0x20);
  uVar4 = *(undefined8 *)(lVar6 + 8 + uVar14 * 8);
  uVar17 = uVar17 + 2;
  if (fVar30 != fVar27) {
    uVar17 = uVar15;
  }
  if (fVar29 != fVar26) {
    uVar17 = uVar15;
  }
  fVar27 = extraout_XMM0_Da_00 * (float)uVar4 + (_DAT_142925904 - extraout_XMM0_Da_00) * fVar24;
  fVar28 = extraout_XMM0_Da_00 * (float)((ulonglong)uVar4 >> 0x20) +
           (_DAT_142925904 - extraout_XMM0_Da_00) * fVar25;
  if (NAN(fVar29) || NAN(fVar26)) {
    uVar17 = uVar15;
  }
  uVar15 = uVar14 - 1;
  if (uVar14 == 0) {
    uVar15 = 0;
  }
  if (fVar28 != fVar25) {
    uVar15 = uVar14;
  }
  if (fVar27 != fVar24) {
    uVar15 = uVar14;
  }
  if (NAN(fVar27) || NAN(fVar24)) {
    uVar15 = uVar14;
  }
  uVar14 = uVar15 + 1;
  uVar23 = uVar14 - uVar17;
  if (uVar14 < uVar17) {
    uVar23 = 0;
  }
  pfVar16 = (float *)(uVar23 + 2);
  uStack_71 = 1;
  pfStack_a8 = pfVar16;
  FUN_1428a29e0(&lStack_90,0,pfVar16,4,8);
  if (lStack_90 == lStack_80) {
    uStack_71 = 1;
    FUN_1428a29e0(&lStack_90,lStack_80,1,4,8);
  }
  *(ulonglong *)(lStack_88 + lStack_80 * 8) = CONCAT44(fVar30,fVar29);
  lStack_80 = lStack_80 + 1;
  uVar23 = uVar15 - uVar17;
  cVar13 = -1;
  if (uVar15 < uVar17 || uVar23 == 0) {
    cVar13 = uVar23 != 0;
  }
  if (cVar13 == '\0') {
    if (uVar21 <= uVar17) {
      uStack_71 = 1;
      FUN_1428d9518(uVar17,uVar21,&UNK_14295a238);
      goto LAB_1401550a0;
    }
    uVar4 = *(undefined8 *)(lVar6 + uVar17 * 8);
    if (lStack_90 == lStack_80) {
      uStack_71 = 1;
      FUN_1428a29e0(&lStack_90,lStack_80,1,4,8);
    }
    *(undefined8 *)(lStack_88 + lStack_80 * 8) = uVar4;
    lStack_80 = lStack_80 + 1;
    pfVar18 = (float *)0x3;
  }
  else {
    pfVar18 = (float *)0x2;
    if (cVar13 != '\x01') {
      uVar19 = uVar14 - uVar17;
      if (uVar14 < uVar17) {
        uStack_71 = 1;
        FUN_1428d9850(uVar17,uVar14,&UNK_14295a220);
        goto LAB_1401550a0;
      }
      if (uVar21 <= uVar15) {
        uStack_71 = 1;
        FUN_1428d9840(uVar14,uVar21,&UNK_14295a220);
        goto LAB_1401550a0;
      }
      if ((ulonglong)(lStack_90 - lStack_80) < uVar19) {
        uStack_71 = 1;
        FUN_1428a29e0(&lStack_90,lStack_80,uVar19,4,8);
        if (uVar19 < 0x14) goto LAB_140154d6b;
LAB_140154f9d:
        uVar21 = lStack_88 + lStack_80 * 8;
        uVar15 = 0;
        if (((uVar21 <= uVar21 + uVar23 * 8) &&
            (lStack_88 + lStack_80 * 8 + 4U <= uVar21 + uVar23 * 8 + 4)) && (uVar23 >> 0x3d == 0)) {
          lVar20 = lStack_80 * 8;
          uVar15 = uVar19 & 0x1fffffffc;
          lStack_80 = lStack_80 + uVar15;
          lVar22 = uVar17 * 8 + 0x10 + lVar6;
          lVar20 = lVar20 + lStack_88;
          uVar21 = 0;
          do {
            puVar2 = (undefined4 *)(lVar22 + -0x10 + uVar21 * 8);
            uVar8 = puVar2[1];
            uVar9 = puVar2[2];
            uVar10 = puVar2[3];
            puVar1 = (undefined8 *)(lVar22 + uVar21 * 8);
            uVar4 = *puVar1;
            uVar5 = puVar1[1];
            puVar3 = (undefined4 *)(lVar20 + uVar21 * 8);
            *puVar3 = *puVar2;
            puVar3[1] = uVar8;
            puVar3[2] = uVar9;
            puVar3[3] = uVar10;
            puVar1 = (undefined8 *)(lVar20 + 0x10 + uVar21 * 8);
            *puVar1 = uVar4;
            puVar1[1] = uVar5;
            uVar21 = uVar21 + 4;
          } while (uVar15 != uVar21);
          if (uVar19 == uVar15) goto LAB_140154d87;
        }
LAB_140154d6d:
        uVar17 = uVar17 + uVar15;
        do {
          *(undefined8 *)(lStack_88 + lStack_80 * 8) = *(undefined8 *)(lVar6 + uVar17 * 8);
          lStack_80 = lStack_80 + 1;
          uVar17 = uVar17 + 1;
        } while (uVar14 != uVar17);
      }
      else if (uVar14 != uVar17) {
        if (0x13 < uVar19) goto LAB_140154f9d;
LAB_140154d6b:
        uVar15 = 0;
        goto LAB_140154d6d;
      }
LAB_140154d87:
      pfVar18 = (float *)(uVar19 + 2);
    }
  }
  pfStack_c8 = pfVar18;
  if (lStack_90 == lStack_80) {
    uStack_71 = 1;
    FUN_1428a29e0(&lStack_90,lStack_80,1,4,8);
  }
  *(ulonglong *)(lStack_88 + lStack_80 * 8) = CONCAT44(fVar28,fVar27);
  lStack_80 = lStack_80 + 1;
  if (pfVar18 == pfVar16) {
    uStack_71 = 0;
    FUN_140153900(uStack_a0,&lStack_90,0);
    return uStack_a0;
  }
  puStack_f8 = (undefined *)0x0;
  uStack_71 = 1;
  FUN_1428d9596(0,&pfStack_c8,&pfStack_a8,&puStack_f8,&UNK_14295a250);
LAB_1401550a0:
                    /* WARNING: Does not return */
  pcVar7 = (code *)invalidInstructionException();
  (*pcVar7)();
}

