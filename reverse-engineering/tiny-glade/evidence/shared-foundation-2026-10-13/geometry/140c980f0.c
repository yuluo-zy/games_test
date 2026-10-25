utils::geometry::circle::Circle2d::contains_point

bool tg_math_140c980f0(undefined8 *param_1,float param_2,float param_3)

{
  param_2 = param_2 - (float)*param_1;
  param_3 = param_3 - (float)((ulonglong)*param_1 >> 0x20);
  return param_3 * param_3 + param_2 * param_2 < *(float *)(param_1 + 1) * *(float *)(param_1 + 1);
}

