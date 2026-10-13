utils::geometry::circle::Circle2d::closest_point_on_perimeter

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

float tg_math_140c97b40(undefined8 *param_1,float param_2,float param_3)

{
  float fVar1;
  
  fVar1 = (float)*param_1;
  param_2 = param_2 - fVar1;
  param_3 = param_3 - (float)((ulonglong)*param_1 >> 0x20);
  return param_2 * (_DAT_142925904 / SQRT(param_3 * param_3 + param_2 * param_2)) *
         *(float *)(param_1 + 1) + fVar1;
}

