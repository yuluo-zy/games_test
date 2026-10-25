
longlong * FUN_1411fa190(longlong *param_1,longlong *param_2,undefined8 param_3)

{
  float fVar1;
  longlong lVar2;
  longlong lVar3;
  longlong *plVar4;
  ulonglong uVar5;
  code *pcVar6;
  longlong *plVar7;
  longlong *plVar8;
  float *pfVar9;
  longlong lVar10;
  longlong *plVar11;
  ulonglong uVar12;
  ulonglong uVar13;
  longlong lVar14;
  float fVar15;
  float extraout_XMM0_Da;
  float fVar16;
  float fVar17;
  float fVar18;
  undefined1 uStack_58;
  undefined1 uStack_57;
  undefined4 uStack_56;
  undefined2 uStack_52;
  ulonglong uStack_50;
  longlong lStack_48;
  undefined8 uStack_40;
  
  uStack_40 = 0xfffffffffffffffe;
  lVar3 = *param_2;
  plVar4 = (longlong *)param_2[1];
  uVar5 = param_2[2];
  lVar14 = lVar3;
  if (uVar5 < 2) {
    uVar13 = uVar5 & 0xffffffff;
    plVar7 = (longlong *)0x0;
    plVar11 = param_1;
  }
  else {
    uVar13 = uVar5 * 4;
    plVar8 = (longlong *)func_0x000140613c10(uVar13);
    if (plVar8 == (longlong *)0x0) {
      lStack_48 = lVar3;
      FUN_1428d8fe3(4,uVar13,&UNK_142c18f50);
                    /* WARNING: Does not return */
      pcVar6 = (code *)invalidInstructionException();
      (*pcVar6)();
    }
    pfVar9 = (float *)(plVar4 + 1);
    fVar15 = 0.0;
    uVar12 = 0;
    fVar16 = *(float *)(plVar4 + 1);
    lVar10 = *plVar4;
    do {
      fVar1 = *pfVar9;
      fVar16 = fVar16 - fVar1;
      lVar2 = *(longlong *)(pfVar9 + -2);
      fVar17 = (float)lVar10 - (float)lVar2;
      fVar18 = (float)((ulonglong)lVar10 >> 0x20) - (float)((ulonglong)lVar2 >> 0x20);
      fVar15 = fVar15 + SQRT(fVar16 * fVar16 + fVar18 * fVar18 + fVar17 * fVar17);
      *(float *)((longlong)plVar8 + uVar12 * 4) = fVar15;
      uVar12 = uVar12 + 1;
      pfVar9 = pfVar9 + 3;
      fVar16 = fVar1;
      lVar10 = lVar2;
    } while (uVar5 != uVar12);
    if ((fVar15 != 0.0) || (NAN(fVar15))) {
      lVar10 = 0;
      do {
        if (uVar13 - lVar10 == 0) goto LAB_1411fa2e3;
        fVar16 = *(float *)((longlong)plVar8 + lVar10) / fVar15;
        *(float *)((longlong)plVar8 + lVar10) = fVar16;
        lVar10 = lVar10 + 4;
      } while ((uint)ABS(fVar16) < 0x7f800000);
    }
    func_0x000140613c20(plVar8,uVar13,4);
    plVar7 = (longlong *)CONCAT71((int7)((ulonglong)param_1 >> 8),1);
    plVar11 = plVar8;
  }
  param_1 = plVar7;
  if (lVar3 != 0) {
    lVar14 = lVar3 * 4;
    plVar11 = plVar4;
    func_0x000140613c20(plVar4,lVar3 * 0xc,4);
  }
  while( true ) {
    uStack_58 = SUB81(param_1,0);
    uStack_57 = (undefined1)uVar13;
    uStack_56 = SUB84(plVar11,0);
    uStack_52 = (undefined2)((ulonglong)plVar11 >> 0x20);
    uStack_50 = uVar5;
    plVar8 = (longlong *)FUN_1428d9760(&UNK_142c18bc8,0x2b,&uStack_58,&UNK_142c18ba8,param_3);
    fVar15 = extraout_XMM0_Da;
LAB_1411fa2e3:
    uVar13 = (ulonglong)plVar4 >> 8 & 0xffffff;
    plVar11 = (longlong *)((ulonglong)plVar4 >> 0x10);
    if (!SBORROW8(0,lVar14)) break;
    param_1 = (longlong *)((ulonglong)plVar4 & 0xffffffff);
  }
  *param_1 = lVar14;
  *(char *)(param_1 + 1) = (char)plVar4;
  *(char *)((longlong)param_1 + 9) = (char)uVar13;
  *(int *)((longlong)param_1 + 10) = (int)((ulonglong)plVar4 >> 0x10);
  *(short *)((longlong)param_1 + 0xe) = (short)((ulonglong)plVar4 >> 0x30);
  param_1[2] = uVar5;
  param_1[3] = uVar5;
  param_1[4] = (longlong)plVar8;
  param_1[5] = uVar5;
  *(float *)(param_1 + 6) = fVar15;
  return param_1;
}

