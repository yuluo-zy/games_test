utils::geometry::circle::Circle2d::intersect_circle

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void tg_math_140c97a40(undefined4 *param_1,undefined8 *param_2,undefined8 *param_3)

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
  
  fVar6 = (float)*param_2;
  fVar8 = (float)((ulonglong)*param_2 >> 0x20);
  fVar2 = (float)*param_3;
  fVar4 = fVar6 - fVar2;
  fVar3 = (float)((ulonglong)*param_3 >> 0x20);
  fVar7 = fVar8 - fVar3;
  fVar5 = SQRT(fVar7 * fVar7 + fVar4 * fVar4);
  fVar4 = *(float *)(param_2 + 1);
  fVar7 = *(float *)(param_3 + 1);
  uVar1 = 0;
  if ((fVar5 <= fVar4 + fVar7) && ((float)((uint)(fVar4 - fVar7) & _DAT_14292e980) <= fVar5)) {
    fVar9 = (fVar5 * fVar5 + (fVar4 * fVar4 - fVar7 * fVar7)) / (fVar5 + fVar5);
    fVar2 = fVar2 - fVar6;
    fVar3 = fVar3 - fVar8;
    fVar7 = SQRT(fVar4 * fVar4 - fVar9 * fVar9) / fVar5;
    fVar6 = (fVar9 / fVar5) * fVar2 + fVar6;
    fVar8 = (fVar9 / fVar5) * fVar3 + fVar8;
    fVar4 = (float)((uint)fVar3 ^ _DAT_1429258f0) * fVar7;
    fVar2 = fVar2 * fVar7;
    param_1[1] = fVar6 + fVar4;
    param_1[2] = fVar8 + fVar2;
    param_1[3] = fVar6 - fVar4;
    param_1[4] = fVar8 - fVar2;
    uVar1 = 1;
  }
  *param_1 = uVar1;
  return;
}

