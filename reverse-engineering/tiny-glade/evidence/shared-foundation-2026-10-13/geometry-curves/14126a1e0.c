
longlong * FUN_14126a1e0(longlong *param_1,longlong *param_2,undefined8 param_3)

{
  undefined8 uVar1;
  longlong lVar2;
  longlong *plVar3;
  ulonglong uVar4;
  code *pcVar5;
  longlong *plVar6;
  longlong *plVar7;
  ulonglong uVar8;
  longlong lVar9;
  longlong *plVar10;
  ulonglong uVar11;
  float fVar12;
  float extraout_XMM0_Da;
  float fVar13;
  float fVar14;
  float fVar15;
  float fVar16;
  undefined1 uStack_58;
  undefined1 uStack_57;
  undefined4 uStack_56;
  undefined2 uStack_52;
  ulonglong uStack_50;
  longlong lStack_48;
  undefined8 uStack_40;
  
  uStack_40 = 0xfffffffffffffffe;
  lVar2 = *param_2;
  plVar3 = (longlong *)param_2[1];
  uVar4 = param_2[2];
  if (uVar4 < 2) {
    uVar11 = uVar4 & 0xffffffff;
    plVar6 = (longlong *)0x0;
    plVar10 = param_1;
  }
  else {
    uVar11 = uVar4 * 4;
    plVar7 = (longlong *)func_0x000140613c10(uVar11,4);
    if (plVar7 == (longlong *)0x0) {
      lStack_48 = lVar2;
      FUN_1428d8fe3(4,uVar11,&UNK_142c226f0);
                    /* WARNING: Does not return */
      pcVar5 = (code *)invalidInstructionException();
      (*pcVar5)();
    }
    fVar13 = (float)*plVar3;
    fVar14 = (float)((ulonglong)*plVar3 >> 0x20);
    fVar12 = 0.0;
    uVar8 = 0;
    do {
      uVar1 = *(undefined8 *)((longlong)plVar3 + uVar8 * 2);
      fVar15 = (float)uVar1;
      fVar13 = fVar13 - fVar15;
      fVar16 = (float)((ulonglong)uVar1 >> 0x20);
      fVar14 = fVar14 - fVar16;
      fVar12 = SQRT(fVar14 * fVar14 + fVar13 * fVar13) + fVar12;
      *(float *)((longlong)plVar7 + uVar8) = fVar12;
      uVar1 = *(undefined8 *)((longlong)plVar3 + uVar8 * 2 + 8);
      fVar13 = (float)uVar1;
      fVar14 = (float)((ulonglong)uVar1 >> 0x20);
      fVar15 = fVar15 - fVar13;
      fVar16 = fVar16 - fVar14;
      fVar12 = SQRT(fVar16 * fVar16 + fVar15 * fVar15) + fVar12;
      *(float *)((longlong)plVar7 + uVar8 + 4) = fVar12;
      uVar8 = uVar8 + 8;
    } while ((uVar11 & 0xfffffffffffffff8) != uVar8);
    if ((uVar4 & 1) != 0) {
      uVar1 = *(undefined8 *)((longlong)plVar3 + uVar8 * 2);
      fVar13 = fVar13 - (float)uVar1;
      fVar14 = fVar14 - (float)((ulonglong)uVar1 >> 0x20);
      fVar12 = fVar12 + SQRT(fVar14 * fVar14 + fVar13 * fVar13);
      *(float *)((longlong)plVar7 + uVar8) = fVar12;
    }
    if ((fVar12 != 0.0) || (NAN(fVar12))) {
      lVar9 = 0;
      do {
        if (uVar11 - lVar9 == 0) goto LAB_14126a372;
        fVar13 = *(float *)((longlong)plVar7 + lVar9) / fVar12;
        *(float *)((longlong)plVar7 + lVar9) = fVar13;
        lVar9 = lVar9 + 4;
      } while ((uint)ABS(fVar13) < 0x7f800000);
    }
    func_0x000140613c20(plVar7,uVar11,4);
    plVar6 = (longlong *)CONCAT71((int7)((ulonglong)param_1 >> 8),1);
    plVar10 = plVar7;
  }
  param_1 = plVar6;
  if (lVar2 != 0) {
    plVar10 = plVar3;
    func_0x000140613c20(plVar3,lVar2 << 3,4);
  }
  while( true ) {
    uStack_58 = SUB81(param_1,0);
    uStack_57 = (undefined1)uVar11;
    uStack_56 = SUB84(plVar10,0);
    uStack_52 = (undefined2)((ulonglong)plVar10 >> 0x20);
    uStack_50 = uVar4;
    plVar7 = (longlong *)FUN_1428d9760(&UNK_142c22368,0x2b,&uStack_58,&UNK_142c22348,param_3);
    fVar12 = extraout_XMM0_Da;
LAB_14126a372:
    uVar11 = (ulonglong)plVar3 >> 8 & 0xffffff;
    plVar10 = (longlong *)((ulonglong)plVar3 >> 0x10);
    if (!SBORROW8(0,lVar2)) break;
    param_1 = (longlong *)((ulonglong)plVar3 & 0xffffffff);
  }
  *param_1 = lVar2;
  *(char *)(param_1 + 1) = (char)plVar3;
  *(char *)((longlong)param_1 + 9) = (char)uVar11;
  *(int *)((longlong)param_1 + 10) = (int)((ulonglong)plVar3 >> 0x10);
  *(short *)((longlong)param_1 + 0xe) = (short)((ulonglong)plVar3 >> 0x30);
  param_1[2] = uVar4;
  param_1[3] = uVar4;
  param_1[4] = (longlong)plVar7;
  param_1[5] = uVar4;
  *(float *)(param_1 + 6) = fVar12;
  return param_1;
}

