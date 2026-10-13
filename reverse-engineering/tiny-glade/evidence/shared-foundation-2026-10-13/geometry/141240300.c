utils::geometry::aabb::Aabb2::expanded

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

float * tg_math_141240300(float *param_1,float *param_2,float param_3,float param_4)

{
  float fVar1;
  float fVar2;
  float fVar3;
  
  param_3 = param_3 * _DAT_1429258e0;
  param_4 = param_4 * _UNK_1429258e4;
  fVar1 = param_2[1];
  fVar2 = param_2[2];
  fVar3 = param_2[3];
  *param_1 = *param_2 - param_3;
  param_1[1] = fVar1 - param_4;
  param_1[2] = param_3 + fVar2;
  param_1[3] = param_4 + fVar3;
  return param_1;
}

