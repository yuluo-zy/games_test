utils::geometry::rectangle::Rectangle2d::aabb

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

float * tg_math_140c9b680(float *param_1,uint *param_2)

{
  uint uVar1;
  float fVar2;
  float fVar3;
  float fVar4;
  float fVar5;
  float fVar6;
  float fVar7;
  float fVar8;
  float fVar9;
  float fVar10;
  
  fVar2 = (float)*(undefined8 *)(param_2 + 4) * _DAT_1429258e0;
  fVar3 = (float)((ulonglong)*(undefined8 *)(param_2 + 4) >> 0x20) * _UNK_1429258e4;
  uVar1 = *param_2;
  fVar6 = (float)(uVar1 & _UNK_14292e984);
  fVar7 = (float)(uVar1 & _UNK_14292e988);
  fVar9 = (float)(uVar1 & _UNK_14292e98c);
  fVar4 = (float)(param_2[1] & _DAT_14292e980) * fVar2;
  fVar5 = (float)(param_2[1] & _UNK_14292e984) * fVar3;
  fVar8 = (float)*(undefined8 *)(param_2 + 2);
  fVar10 = (float)((ulonglong)*(undefined8 *)(param_2 + 2) >> 0x20);
  *param_1 = fVar8 - (fVar5 + fVar2 * (float)(uVar1 & _DAT_14292e980));
  param_1[1] = fVar10 - (fVar4 + fVar3 * fVar6);
  param_1[2] = fVar8 + fVar5 + fVar2 * fVar7;
  param_1[3] = fVar10 + fVar4 + fVar3 * fVar9;
  return param_1;
}

