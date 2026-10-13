utils::geometry::circle::Circle2d::aabb

float * tg_math_140c98140(float *param_1,undefined8 *param_2)

{
  float fVar1;
  float fVar2;
  float fVar3;
  
  fVar1 = *(float *)(param_2 + 1);
  fVar2 = (float)*param_2;
  fVar3 = (float)((ulonglong)*param_2 >> 0x20);
  *param_1 = fVar2 - fVar1;
  param_1[1] = fVar3 - fVar1;
  param_1[2] = fVar1 + fVar2;
  param_1[3] = fVar1 + fVar3;
  return param_1;
}

