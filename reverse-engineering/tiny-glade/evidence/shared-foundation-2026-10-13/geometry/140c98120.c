utils::geometry::circle::Circle2d::signed_distance

undefined8 tg_circle_signed_distance(undefined8 *param_1,float param_2,float param_3)

{
  param_2 = param_2 - (float)*param_1;
  param_3 = param_3 - (float)((ulonglong)*param_1 >> 0x20);
  param_3 = param_3 * param_3;
  return CONCAT44(param_3,SQRT(param_3 + param_2 * param_2) - *(float *)(param_1 + 1));
}

