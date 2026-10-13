
longlong * FUN_140153640(longlong *param_1,longlong *param_2,ulonglong param_3)

{
  longlong lVar1;
  float fVar2;
  undefined8 uVar3;
  ulonglong uVar4;
  float *pfVar5;
  code *pcVar6;
  longlong lVar7;
  float *pfVar8;
  ulonglong uVar9;
  longlong lVar10;
  longlong lVar11;
  float fVar12;
  float fVar13;
  float fVar14;
  undefined8 uVar15;
  float fVar16;
  
  uVar4 = param_2[2];
  if (uVar4 < 2) {
    *(undefined1 *)(param_1 + 1) = 0;
    *(char *)((longlong)param_1 + 9) = (char)uVar4;
    *param_1 = -0x8000000000000000;
    if (*param_2 == 0) {
      return param_1;
    }
    func_0x000140613c20(param_2[1],*param_2 * 0xc,4);
    return param_1;
  }
  lVar1 = uVar4 * 4;
  lVar7 = func_0x000140613c10(lVar1);
  if (lVar7 == 0) {
    FUN_1428d8fe3(4,lVar1,&UNK_14295a470);
                    /* WARNING: Does not return */
    pcVar6 = (code *)invalidInstructionException();
    (*pcVar6)();
  }
  pfVar5 = (float *)param_2[1];
  pfVar8 = pfVar5 + 2;
  fVar12 = 0.0;
  uVar9 = 0;
  fVar13 = pfVar5[2];
  uVar15 = *(undefined8 *)pfVar5;
  do {
    fVar2 = *pfVar8;
    fVar13 = fVar13 - fVar2;
    uVar3 = *(undefined8 *)(pfVar8 + -2);
    fVar14 = (float)uVar15 - (float)uVar3;
    fVar16 = (float)((ulonglong)uVar15 >> 0x20) - (float)((ulonglong)uVar3 >> 0x20);
    fVar12 = fVar12 + SQRT(fVar13 * fVar13 + fVar16 * fVar16 + fVar14 * fVar14);
    *(float *)(lVar7 + uVar9 * 4) = fVar12;
    uVar9 = uVar9 + 1;
    pfVar8 = pfVar8 + 3;
    fVar13 = fVar2;
    uVar15 = uVar3;
  } while (uVar4 != uVar9);
  if ((fVar12 != 0.0) || (NAN(fVar12))) {
    lVar10 = 0;
    do {
      if (lVar1 - lVar10 == 0) {
        if ((param_3 & 1) == 0) goto LAB_1401537fc;
        fVar13 = pfVar5[3];
        if ((*pfVar5 == fVar13) && (!NAN(*pfVar5) && !NAN(fVar13))) {
          if ((pfVar5[1] == pfVar5[4]) && (!NAN(pfVar5[1]) && !NAN(pfVar5[4]))) {
            if ((pfVar5[2] == pfVar5[5]) && (!NAN(pfVar5[2]) && !NAN(pfVar5[5]))) {
              lVar10 = 0;
              goto LAB_14015387f;
            }
          }
        }
        if (uVar4 == 2) goto LAB_1401537fc;
        lVar10 = 1;
        lVar11 = 0;
        goto LAB_14015384c;
      }
      fVar13 = *(float *)(lVar7 + lVar10) / fVar12;
      *(float *)(lVar7 + lVar10) = fVar13;
      lVar10 = lVar10 + 4;
    } while ((uint)ABS(fVar13) < 0x7f800000);
  }
  *(undefined1 *)(param_1 + 1) = 1;
LAB_140153780:
  *param_1 = -0x8000000000000000;
  func_0x000140613c20(lVar7,lVar1,4);
  if (*param_2 != 0) {
    func_0x000140613c20(pfVar5,*param_2 * 0xc,4);
  }
  return param_1;
LAB_14015384c:
  fVar2 = *(float *)((longlong)pfVar5 + lVar11 + 0x18);
  if ((fVar13 == fVar2) && (!NAN(fVar13) && !NAN(fVar2))) {
    fVar13 = *(float *)((longlong)pfVar5 + lVar11 + 0x10);
    pfVar8 = (float *)((longlong)pfVar5 + lVar11 + 0x1c);
    if ((fVar13 == *pfVar8) && (!NAN(fVar13) && !NAN(*pfVar8))) {
      fVar13 = *(float *)((longlong)pfVar5 + lVar11 + 0x14);
      pfVar8 = (float *)((longlong)pfVar5 + lVar11 + 0x20);
      if ((fVar13 == *pfVar8) && (!NAN(fVar13) && !NAN(*pfVar8))) goto LAB_14015387f;
    }
  }
  lVar11 = lVar11 + 0xc;
  lVar10 = lVar10 + 1;
  fVar13 = fVar2;
  if (uVar4 * 0xc + -0x18 == lVar11) {
LAB_1401537fc:
    param_1[2] = param_2[2];
    lVar1 = param_2[1];
    *param_1 = *param_2;
    param_1[1] = lVar1;
    param_1[3] = uVar4;
    param_1[4] = lVar7;
    param_1[5] = uVar4;
    *(float *)(param_1 + 6) = fVar12;
    return param_1;
  }
  goto LAB_14015384c;
LAB_14015387f:
  *(undefined1 *)(param_1 + 1) = 2;
  param_1[2] = lVar10;
  goto LAB_140153780;
}

