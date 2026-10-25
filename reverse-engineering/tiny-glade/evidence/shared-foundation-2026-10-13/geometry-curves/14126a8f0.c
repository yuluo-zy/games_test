
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

undefined8 FUN_14126a8f0(undefined8 param_1,longlong param_2,float param_3,float param_4)

{
  ulonglong uVar1;
  undefined8 uVar2;
  undefined8 uVar3;
  longlong lVar4;
  code *pcVar5;
  longlong lVar6;
  char cVar7;
  uint uVar8;
  uint uVar9;
  ulonglong uVar10;
  float *pfVar11;
  ulonglong uVar12;
  ulonglong uVar13;
  float *pfVar14;
  ulonglong uVar15;
  longlong lVar16;
  float extraout_XMM0_Da;
  float extraout_XMM0_Da_00;
  float fVar17;
  float fVar18;
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
  undefined *puStack_138;
  undefined8 uStack_130;
  undefined4 **ppuStack_128;
  undefined8 uStack_120;
  undefined8 uStack_118;
  float *pfStack_108;
  float *pfStack_100;
  code *pcStack_f8;
  float *pfStack_f0;
  code *pcStack_e8;
  undefined8 uStack_e0;
  ulonglong uStack_d8;
  float fStack_d0;
  float fStack_cc;
  ulonglong uStack_c8;
  longlong lStack_c0;
  longlong lStack_b8;
  longlong lStack_b0;
  undefined1 uStack_a1;
  undefined8 uStack_a0;
  
  uStack_a0 = 0xfffffffffffffffe;
  lStack_c0 = 0;
  lStack_b8 = 4;
  lStack_b0 = 0;
  fStack_d0 = param_4;
  fStack_cc = param_3;
  if (param_4 <= param_3) {
    pfStack_100 = &fStack_cc;
    pcStack_f8 = FUN_14126b0e0;
    pfStack_f0 = &fStack_d0;
    pcStack_e8 = FUN_14126b0e0;
    puStack_138 = &UNK_142c223d0;
    uStack_130 = 2;
    uStack_118 = 0;
    ppuStack_128 = &pfStack_100;
    uStack_120 = 2;
    uStack_a1 = 1;
    FUN_1428d9390(&puStack_138,&UNK_142c22440);
    goto LAB_14126ae66;
  }
  uStack_a1 = 1;
  uVar8 = FUN_141269710(param_2,param_3);
  uStack_a1 = 1;
  uVar9 = FUN_141269710(param_2,param_4);
  uVar12 = (ulonglong)uVar8;
  uVar13 = *(ulonglong *)(param_2 + 0x10);
  if (uVar13 <= uVar12) {
    uStack_a1 = 1;
    FUN_1428d9518(uVar12,uVar13,&UNK_142c225e8);
    goto LAB_14126ae66;
  }
  uVar15 = uVar12 + 1;
  if (uVar13 <= uVar15) {
    uStack_a1 = 1;
    FUN_1428d9518(uVar15,uVar13,&UNK_142c22600);
    goto LAB_14126ae66;
  }
  uVar10 = (ulonglong)uVar9;
  if (uVar13 <= uVar10) {
    uStack_a1 = 1;
    FUN_1428d9518(uVar10,uVar13,&UNK_142c225e8);
    goto LAB_14126ae66;
  }
  uVar1 = uVar10 + 1;
  uStack_c8 = uVar13;
  if (uVar13 <= uVar1) {
    uStack_a1 = 1;
    FUN_1428d9518(uVar1,uVar13,&UNK_142c22600);
    goto LAB_14126ae66;
  }
  lVar4 = *(longlong *)(param_2 + 8);
  fVar24 = *(float *)(lVar4 + 8 + uVar15 * 0xc);
  fVar20 = _DAT_142925904 - extraout_XMM0_Da;
  uVar2 = *(undefined8 *)(lVar4 + uVar12 * 0xc);
  uVar3 = *(undefined8 *)(lVar4 + uVar15 * 0xc);
  fVar21 = (float)uVar3;
  fVar22 = (float)((ulonglong)uVar3 >> 0x20);
  fVar27 = extraout_XMM0_Da * fVar21 + (float)uVar2 * fVar20;
  fVar28 = extraout_XMM0_Da * fVar22 + (float)((ulonglong)uVar2 >> 0x20) * fVar20;
  fVar23 = extraout_XMM0_Da * fVar24 + fVar20 * *(float *)(lVar4 + 8 + uVar12 * 0xc);
  fVar20 = *(float *)(lVar4 + 8 + uVar10 * 0xc);
  fVar19 = _DAT_142925904 - extraout_XMM0_Da_00;
  uVar2 = *(undefined8 *)(lVar4 + uVar1 * 0xc);
  uVar3 = *(undefined8 *)(lVar4 + uVar10 * 0xc);
  fVar17 = (float)uVar3;
  fVar18 = (float)((ulonglong)uVar3 >> 0x20);
  uVar13 = uVar12 + 2;
  if (fVar23 != fVar24) {
    uVar13 = uVar15;
  }
  fVar25 = (float)uVar2 * extraout_XMM0_Da_00 + fVar19 * fVar17;
  fVar26 = (float)((ulonglong)uVar2 >> 0x20) * extraout_XMM0_Da_00 + fVar19 * fVar18;
  if (NAN(fVar23) || NAN(fVar24)) {
    uVar13 = uVar15;
  }
  if (fVar28 != fVar22) {
    uVar13 = uVar15;
  }
  if (fVar27 != fVar21) {
    uVar13 = uVar15;
  }
  if (NAN(fVar27) || NAN(fVar21)) {
    uVar13 = uVar15;
  }
  fVar24 = *(float *)(lVar4 + 8 + uVar1 * 0xc) * extraout_XMM0_Da_00 + fVar19 * fVar20;
  uVar12 = uVar10 - 1;
  if (uVar10 == 0) {
    uVar12 = 0;
  }
  if (fVar24 != fVar20) {
    uVar12 = uVar10;
  }
  if (NAN(fVar24) || NAN(fVar20)) {
    uVar12 = uVar10;
  }
  if (fVar26 != fVar18) {
    uVar12 = uVar10;
  }
  if (fVar25 != fVar17) {
    uVar12 = uVar10;
  }
  if (NAN(fVar25) || NAN(fVar17)) {
    uVar12 = uVar10;
  }
  uStack_d8 = uVar12 + 1;
  uVar15 = uStack_d8 - uVar13;
  if (uStack_d8 < uVar13) {
    uVar15 = 0;
  }
  pfVar11 = (float *)(uVar15 + 2);
  pfStack_100 = (float *)0x0;
  uStack_a1 = 1;
  pfStack_108 = pfVar11;
  FUN_1428a7f20(&lStack_c0,0,pfVar11,4,0xc);
  uStack_e0 = param_1;
  if (lStack_c0 == lStack_b0) {
    uStack_a1 = 1;
    FUN_1428a7f20(&lStack_c0,lStack_b0,1,4,0xc);
  }
  *(ulonglong *)(lStack_b8 + lStack_b0 * 0xc) = CONCAT44(fVar28,fVar27);
  *(float *)(lStack_b8 + 8 + lStack_b0 * 0xc) = fVar23;
  lStack_b0 = lStack_b0 + 1;
  cVar7 = -1;
  if (uVar12 <= uVar13) {
    cVar7 = uVar13 != uVar12;
  }
  if (cVar7 == '\0') {
    if (uStack_c8 <= uVar13) {
      uStack_a1 = 1;
      FUN_1428d9518(uVar13,uStack_c8,&UNK_142c22470);
      goto LAB_14126ae66;
    }
    uStack_130 = CONCAT44(uStack_130._4_4_,*(undefined4 *)(lVar4 + 8 + uVar13 * 0xc));
    puStack_138 = *(undefined **)(lVar4 + uVar13 * 0xc);
    if (lStack_c0 == lStack_b0) {
      uStack_a1 = 1;
      FUN_1428a7f20(&lStack_c0,lStack_b0,1,4,0xc);
    }
    *(undefined4 *)(lStack_b8 + 8 + lStack_b0 * 0xc) = (undefined4)uStack_130;
    *(undefined **)(lStack_b8 + lStack_b0 * 0xc) = puStack_138;
    lStack_b0 = lStack_b0 + 1;
    pfVar14 = (float *)0x3;
    lVar16 = lStack_b8;
  }
  else {
    pfVar14 = (float *)0x2;
    lVar16 = lStack_b8;
    if (cVar7 != '\x01') {
      uVar15 = uStack_d8 - uVar13;
      if (uStack_d8 < uVar13) {
        uStack_a1 = 1;
        FUN_1428d9850(uVar13,uStack_d8,&UNK_142c22458);
        goto LAB_14126ae66;
      }
      if (uStack_c8 <= uVar12) {
        uStack_a1 = 1;
        FUN_1428d9840(uStack_d8,uStack_c8,&UNK_142c22458);
        goto LAB_14126ae66;
      }
      if ((ulonglong)(lStack_c0 - lStack_b0) < uVar15) {
        uStack_a1 = 1;
        FUN_1428a7f20(&lStack_c0,lStack_b0,uVar15,4,0xc);
LAB_14126ab7f:
        lVar6 = lStack_b0;
        lVar16 = lStack_b8;
        func_0x000142923c00(lStack_b0 * 0xc + lStack_b8,lVar4 + uVar13 * 0xc,uVar15 * 0xc);
        lStack_b0 = lVar6 + uVar15;
      }
      else if (uStack_d8 != uVar13) goto LAB_14126ab7f;
      pfVar14 = (float *)(uVar15 + 2);
    }
  }
  pfStack_100 = pfVar14;
  if (lStack_c0 == lStack_b0) {
    uStack_a1 = 1;
    FUN_1428a7f20(&lStack_c0,lStack_b0,1,4,0xc);
    lVar16 = lStack_b8;
  }
  uVar2 = uStack_e0;
  *(ulonglong *)(lVar16 + lStack_b0 * 0xc) = CONCAT44(fVar26,fVar25);
  *(float *)(lVar16 + 8 + lStack_b0 * 0xc) = fVar24;
  lStack_b0 = lStack_b0 + 1;
  if (pfVar14 == pfVar11) {
    uStack_a1 = 0;
    FUN_141269ce0(uStack_e0,&lStack_c0,0);
    return uVar2;
  }
  puStack_138 = (undefined *)0x0;
  uStack_a1 = 1;
  FUN_1428d9596(0,&pfStack_100,&pfStack_108,&puStack_138,&UNK_142c22488);
LAB_14126ae66:
                    /* WARNING: Does not return */
  pcVar5 = (code *)invalidInstructionException();
  (*pcVar5)();
}

