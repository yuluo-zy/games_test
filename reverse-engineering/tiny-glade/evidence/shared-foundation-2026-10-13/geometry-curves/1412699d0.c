
longlong * FUN_1412699d0(longlong *param_1,longlong *param_2,ulonglong param_3)

{
  ulonglong uVar1;
  float *pfVar2;
  ulonglong uVar3;
  float *pfVar4;
  code *pcVar5;
  longlong lVar6;
  ulonglong uVar7;
  float *pfVar8;
  longlong lVar9;
  longlong lVar10;
  float fVar11;
  float fVar12;
  float fVar13;
  float fVar14;
  float fVar15;
  
  uVar3 = param_2[2];
  if (uVar3 < 2) {
    *(undefined1 *)(param_1 + 1) = 0;
    *(char *)((longlong)param_1 + 9) = (char)uVar3;
    *param_1 = -0x8000000000000000;
    if (*param_2 == 0) {
      return param_1;
    }
    func_0x000140613c20(param_2[1],*param_2 * 0xc,4);
    return param_1;
  }
  uVar1 = uVar3 * 4;
  lVar6 = func_0x000140613c10(uVar1,4);
  if (lVar6 == 0) {
    FUN_1428d8fe3(4,uVar1,&UNK_142c226f0);
                    /* WARNING: Does not return */
    pcVar5 = (code *)invalidInstructionException();
    (*pcVar5)();
  }
  pfVar4 = (float *)param_2[1];
  fVar13 = *pfVar4;
  fVar12 = pfVar4[2];
  fVar11 = 0.0;
  uVar7 = 0;
  pfVar2 = pfVar4;
  do {
    pfVar8 = pfVar2;
    fVar14 = *pfVar8;
    fVar15 = pfVar8[2];
    fVar13 = fVar13 - fVar14;
    fVar12 = fVar12 - fVar15;
    fVar11 = SQRT(fVar12 * fVar12 + fVar13 * fVar13) + fVar11;
    *(float *)(lVar6 + uVar7) = fVar11;
    fVar13 = pfVar8[3];
    fVar12 = pfVar8[5];
    fVar14 = fVar14 - fVar13;
    fVar15 = fVar15 - fVar12;
    fVar11 = SQRT(fVar15 * fVar15 + fVar14 * fVar14) + fVar11;
    *(float *)(lVar6 + 4 + uVar7) = fVar11;
    uVar7 = uVar7 + 8;
    pfVar2 = pfVar8 + 6;
  } while ((uVar1 & 0xfffffffffffffff8) != uVar7);
  if ((uVar3 & 1) != 0) {
    fVar13 = fVar13 - pfVar8[6];
    fVar11 = fVar11 + SQRT((fVar12 - pfVar8[8]) * (fVar12 - pfVar8[8]) + fVar13 * fVar13);
    *(float *)(lVar6 + uVar7) = fVar11;
  }
  if ((fVar11 != 0.0) || (NAN(fVar11))) {
    lVar9 = 0;
    do {
      if (uVar1 - lVar9 == 0) {
        if ((param_3 & 1) == 0) goto LAB_141269bdc;
        fVar13 = pfVar4[3];
        if ((*pfVar4 == fVar13) && (!NAN(*pfVar4) && !NAN(fVar13))) {
          if ((pfVar4[1] == pfVar4[4]) && (!NAN(pfVar4[1]) && !NAN(pfVar4[4]))) {
            if ((pfVar4[2] == pfVar4[5]) && (!NAN(pfVar4[2]) && !NAN(pfVar4[5]))) {
              lVar9 = 0;
              goto LAB_141269c5f;
            }
          }
        }
        if (uVar3 == 2) goto LAB_141269bdc;
        lVar9 = 1;
        lVar10 = 0;
        goto LAB_141269c2c;
      }
      fVar13 = *(float *)(lVar6 + lVar9) / fVar11;
      *(float *)(lVar6 + lVar9) = fVar13;
      lVar9 = lVar9 + 4;
    } while ((uint)ABS(fVar13) < 0x7f800000);
  }
  *(undefined1 *)(param_1 + 1) = 1;
LAB_141269b60:
  *param_1 = -0x8000000000000000;
  func_0x000140613c20(lVar6,uVar1,4);
  if (*param_2 != 0) {
    func_0x000140613c20(pfVar4,*param_2 * 0xc,4);
  }
  return param_1;
LAB_141269c2c:
  fVar12 = *(float *)((longlong)pfVar4 + lVar10 + 0x18);
  if ((fVar13 == fVar12) && (!NAN(fVar13) && !NAN(fVar12))) {
    fVar13 = *(float *)((longlong)pfVar4 + lVar10 + 0x10);
    pfVar2 = (float *)((longlong)pfVar4 + lVar10 + 0x1c);
    if ((fVar13 == *pfVar2) && (!NAN(fVar13) && !NAN(*pfVar2))) {
      fVar13 = *(float *)((longlong)pfVar4 + lVar10 + 0x14);
      pfVar2 = (float *)((longlong)pfVar4 + lVar10 + 0x20);
      if ((fVar13 == *pfVar2) && (!NAN(fVar13) && !NAN(*pfVar2))) goto LAB_141269c5f;
    }
  }
  lVar10 = lVar10 + 0xc;
  lVar9 = lVar9 + 1;
  fVar13 = fVar12;
  if (uVar3 * 0xc + -0x18 == lVar10) {
LAB_141269bdc:
    param_1[2] = param_2[2];
    lVar9 = param_2[1];
    *param_1 = *param_2;
    param_1[1] = lVar9;
    param_1[3] = uVar3;
    param_1[4] = lVar6;
    param_1[5] = uVar3;
    *(float *)(param_1 + 6) = fVar11;
    return param_1;
  }
  goto LAB_141269c2c;
LAB_141269c5f:
  *(undefined1 *)(param_1 + 1) = 2;
  param_1[2] = lVar9;
  goto LAB_141269b60;
}

