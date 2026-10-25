utils::geometry::aabb::Aabb3::scaled

float * tg_math_141240490(float *param_1,float *param_2,float *param_3)

{
  float fVar1;
  undefined8 uVar2;
  float fVar3;
  float fVar4;
  float fVar5;
  float fVar6;
  float fVar7;
  
  fVar3 = param_2[1];
  fVar4 = param_2[2];
  fVar5 = param_2[3];
  fVar1 = *param_3;
  uVar2 = *(undefined8 *)(param_2 + 4);
  fVar6 = (float)*(undefined8 *)(param_3 + 1);
  fVar7 = (float)((ulonglong)*(undefined8 *)(param_3 + 1) >> 0x20);
  *param_1 = fVar1 * *param_2;
  param_1[1] = fVar6 * fVar3;
  param_1[2] = fVar7 * fVar4;
  param_1[3] = fVar1 * fVar5;
  *(ulonglong *)(param_1 + 4) =
       CONCAT44((float)((ulonglong)uVar2 >> 0x20) * fVar7,(float)uVar2 * fVar6);
  return param_1;
}

