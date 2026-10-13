utils::geometry::aabb::Aabb3::expand

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void tg_math_1412403c0(undefined8 *param_1,float *param_2,float *param_3)

{
  undefined8 uVar1;
  float fVar2;
  float fVar3;
  float fVar4;
  
  fVar2 = *param_3 * _DAT_142925870;
  fVar3 = (float)*(undefined8 *)(param_3 + 1) * _DAT_1429258e0;
  fVar4 = (float)((ulonglong)*(undefined8 *)(param_3 + 1) >> 0x20) * _UNK_1429258e4;
  *param_2 = *param_2 - fVar2;
  param_2[1] = param_2[1] - fVar3;
  param_2[2] = param_2[2] - fVar4;
  param_2[3] = fVar2 + param_2[3];
  *(ulonglong *)(param_2 + 4) =
       CONCAT44((float)((ulonglong)*(undefined8 *)(param_2 + 4) >> 0x20) + fVar4,
                (float)*(undefined8 *)(param_2 + 4) + fVar3);
  param_1[2] = *(undefined8 *)(param_2 + 4);
  uVar1 = *(undefined8 *)(param_2 + 2);
  *param_1 = *(undefined8 *)param_2;
  param_1[1] = uVar1;
  return;
}

