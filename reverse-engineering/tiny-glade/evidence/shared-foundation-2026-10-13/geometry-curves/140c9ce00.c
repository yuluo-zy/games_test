
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void FUN_140c9ce00(float *param_1,undefined8 *param_2,undefined8 *param_3)

{
  float fVar1;
  float fVar2;
  float fVar3;
  float fVar4;
  float fVar5;
  float fVar6;
  float fVar7;
  float fVar8;
  float fVar9;
  float fVar10;
  float fVar11;
  
  fVar3 = param_1[6] - *(float *)(param_2 + 1);
  fVar6 = (float)*(undefined8 *)(param_1 + 4) - (float)*param_2;
  fVar7 = (float)((ulonglong)*(undefined8 *)(param_1 + 4) >> 0x20) -
          (float)((ulonglong)*param_2 >> 0x20);
  fVar5 = _DAT_142925904 / SQRT(fVar7 * fVar7 + fVar6 * fVar6 + fVar3 * fVar3);
  fVar6 = fVar5 * fVar6;
  fVar7 = fVar5 * fVar7;
  fVar5 = fVar5 * fVar3;
  fVar3 = (float)*param_3;
  fVar4 = *(float *)(param_3 + 1) * fVar6 - fVar3 * fVar5;
  fVar3 = fVar7 * fVar3 - (float)*(undefined8 *)((longlong)param_3 + 4) * fVar6;
  fVar8 = fVar5 * (float)((ulonglong)*param_3 >> 0x20) -
          (float)((ulonglong)*(undefined8 *)((longlong)param_3 + 4) >> 0x20) * fVar7;
  fVar9 = _DAT_142925904 / SQRT(fVar8 * fVar8 + fVar4 * fVar4 + fVar3 * fVar3);
  fVar8 = fVar8 * fVar9;
  fVar4 = fVar4 * fVar9;
  fVar11 = fVar6 * fVar4 - fVar7 * fVar8;
  fVar9 = fVar9 * fVar3;
  fVar3 = fVar7 * fVar9 - fVar5 * fVar4;
  fVar10 = fVar5 * fVar8 - fVar9 * fVar6;
  if (fVar5 <= 0.0) {
    fVar10 = fVar10 - fVar8;
    if (fVar10 <= 0.0) {
      fVar10 = (_DAT_142925904 - fVar5) - fVar10;
      fVar5 = _DAT_142925870 / SQRT(fVar10);
      fVar1 = fVar5 * fVar10;
      fVar10 = fVar5 * (fVar3 + fVar4);
      fVar2 = fVar5 * (fVar9 + fVar6);
      fVar5 = fVar5 * (fVar11 - fVar7);
    }
    else {
      fVar10 = (_DAT_142925904 - fVar5) + fVar10;
      fVar5 = _DAT_142925870 / SQRT(fVar10);
      fVar1 = fVar5 * (fVar4 + fVar3);
      fVar10 = fVar5 * fVar10;
      fVar2 = fVar5 * (fVar7 + fVar11);
      fVar5 = fVar5 * (fVar6 - fVar9);
    }
  }
  else {
    fVar8 = fVar8 + fVar10;
    if (fVar8 <= 0.0) {
      fVar8 = (fVar5 + _DAT_142925904) - fVar8;
      fVar5 = _DAT_142925870 / SQRT(fVar8);
      fVar1 = fVar5 * (fVar9 + fVar6);
      fVar10 = fVar5 * (fVar7 + fVar11);
      fVar2 = fVar5 * fVar8;
      fVar5 = fVar5 * (fVar4 - fVar3);
    }
    else {
      fVar8 = fVar8 + fVar5 + _DAT_142925904;
      fVar5 = _DAT_142925870 / SQRT(fVar8);
      fVar1 = fVar5 * (fVar11 - fVar7);
      fVar10 = fVar5 * (fVar6 - fVar9);
      fVar2 = fVar5 * (fVar4 - fVar3);
      fVar5 = fVar5 * fVar8;
    }
  }
  *param_1 = fVar1;
  param_1[1] = fVar10;
  param_1[2] = fVar2;
  param_1[3] = fVar5;
  return;
}

