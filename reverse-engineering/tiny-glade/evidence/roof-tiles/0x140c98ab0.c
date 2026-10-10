
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

float * FUN_140c98ab0(float *param_1,undefined8 *param_2)

{
  undefined8 uVar1;
  code *pcVar3;
  float *pfVar4;
  float fVar5;
  float fVar6;
  float fVar7;
  float fVar8;
  float fVar9;
  float fVar10;
  float fVar11;
  float fVar12;
  float fVar13;
  float fVar14;
  float fVar15;
  undefined8 uVar2;
  
  fVar5 = *(float *)(param_2 + 2);
  fVar6 = *(float *)((longlong)param_2 + 0x14);
  if (0.0 < fVar5 * fVar6) {
    fVar14 = (float)((ulonglong)*param_2 >> 0x20);
    fVar11 = (float)((uint)fVar14 ^ _DAT_1429258f0);
    fVar13 = (float)*param_2;
    fVar12 = fVar5 * _DAT_142925870;
    fVar15 = _DAT_142925870 * fVar6;
    fVar5 = fVar5 * _DAT_14295a0e8;
    fVar6 = fVar6 * _DAT_14295a0e8;
    uVar1 = param_2[1];
    uVar2 = param_2[1];
    fVar7 = (float)uVar1;
    fVar8 = (float)((ulonglong)uVar1 >> 0x20);
    fVar9 = (float)uVar2;
    fVar10 = (float)((ulonglong)uVar2 >> 0x20);
    *param_1 = fVar5 * fVar13 + fVar6 * fVar11 + fVar7;
    param_1[1] = fVar5 * fVar14 + fVar6 * fVar13 + fVar8;
    param_1[2] = fVar5 * fVar13 + fVar15 * fVar11 + fVar9;
    param_1[3] = fVar5 * fVar14 + fVar15 * fVar13 + fVar10;
    param_1[4] = fVar12 * fVar13 + fVar15 * fVar11 + fVar7;
    param_1[5] = fVar12 * fVar14 + fVar15 * fVar13 + fVar8;
    param_1[6] = fVar12 * fVar13 + fVar6 * fVar11 + fVar9;
    param_1[7] = fVar12 * fVar14 + fVar6 * fVar13 + fVar10;
    return param_1;
  }
  FUN_1428d9430(&UNK_142b2e66c,0x23,&UNK_142b2e6d0);
  pcVar3 = (code *)swi(3);
  pfVar4 = (float *)(*pcVar3)();
  return pfVar4;
}

