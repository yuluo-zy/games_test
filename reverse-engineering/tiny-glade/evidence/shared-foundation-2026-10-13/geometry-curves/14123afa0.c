
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void FUN_14123afa0(undefined4 *param_1,undefined8 *param_2,float *param_3)

{
  undefined4 uVar1;
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
  float fVar12;
  float fVar13;
  float fVar14;
  float fVar15;
  float fVar16;
  float fVar17;
  
  fVar15 = *param_3;
  fVar3 = param_3[3] - fVar15;
  fVar16 = (float)*(undefined8 *)(param_3 + 1);
  fVar17 = (float)((ulonglong)*(undefined8 *)(param_3 + 1) >> 0x20);
  fVar4 = (float)*(undefined8 *)(param_3 + 4) - fVar16;
  fVar5 = (float)((ulonglong)*(undefined8 *)(param_3 + 4) >> 0x20) - fVar17;
  fVar2 = param_3[7] - fVar16;
  fVar6 = param_3[8] - fVar17;
  fVar7 = param_3[6] - fVar15;
  fVar9 = *(float *)((longlong)param_2 + 0xc);
  fVar10 = (float)param_2[2];
  fVar11 = (float)((ulonglong)param_2[2] >> 0x20);
  fVar12 = fVar6 * fVar10 - fVar2 * fVar11;
  fVar13 = fVar7 * fVar11 - fVar6 * fVar9;
  fVar8 = fVar2 * fVar9 - fVar7 * fVar10;
  fVar14 = fVar5 * fVar8 + fVar13 * fVar4 + fVar3 * fVar12;
  uVar1 = 0;
  if (_DAT_142950d70 <= (float)(_DAT_14292e980 & (uint)fVar14)) {
    fVar14 = _DAT_142925904 / fVar14;
    fVar15 = (float)*param_2 - fVar15;
    fVar16 = (float)((ulonglong)*param_2 >> 0x20) - fVar16;
    fVar17 = *(float *)(param_2 + 1) - fVar17;
    fVar8 = (fVar8 * fVar17 + fVar13 * fVar16 + fVar12 * fVar15) * fVar14;
    if ((0.0 <= fVar8) && (fVar8 <= _DAT_142925904)) {
      fVar12 = fVar4 * fVar15 - fVar16 * fVar3;
      fVar4 = fVar5 * fVar16 - fVar17 * fVar4;
      fVar15 = fVar3 * fVar17 - fVar5 * fVar15;
      fVar9 = (fVar9 * fVar4 + fVar10 * fVar15 + fVar11 * fVar12) * fVar14;
      if ((0.0 <= fVar9) &&
         ((fVar8 + fVar9 <= _DAT_142925904 &&
          (fVar14 = fVar14 * (fVar7 * fVar4 + fVar2 * fVar15 + fVar6 * fVar12),
          _DAT_142950d70 < fVar14)))) {
        param_1[1] = fVar14;
        param_1[2] = fVar8;
        param_1[3] = fVar9;
        uVar1 = 1;
      }
    }
  }
  *param_1 = uVar1;
  return;
}

