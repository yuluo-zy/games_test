
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

undefined8 * FUN_14123ada0(undefined8 *param_1,float *param_2)

{
  float fVar1;
  float fVar2;
  float fVar3;
  float fVar4;
  float fVar5;
  float fVar6;
  float fVar7;
  
  fVar2 = param_2[3] - *param_2;
  fVar3 = (float)*(undefined8 *)(param_2 + 4);
  fVar1 = param_2[7] - fVar3;
  fVar6 = (float)((ulonglong)*(undefined8 *)(param_2 + 4) >> 0x20);
  fVar3 = fVar3 - (float)*(undefined8 *)(param_2 + 1);
  fVar5 = fVar6 - (float)((ulonglong)*(undefined8 *)(param_2 + 1) >> 0x20);
  fVar6 = param_2[8] - fVar6;
  fVar7 = param_2[6] - param_2[3];
  fVar4 = fVar3 * fVar6 - fVar1 * fVar5;
  fVar6 = fVar5 * fVar7 - fVar6 * fVar2;
  fVar1 = fVar1 * fVar2 - fVar7 * fVar3;
  fVar2 = _DAT_142925904 / SQRT(fVar1 * fVar1 + fVar6 * fVar6 + fVar4 * fVar4);
  *param_1 = CONCAT44(fVar2 * fVar6,fVar2 * fVar4);
  *(float *)(param_1 + 1) = fVar2 * fVar1;
  return param_1;
}

