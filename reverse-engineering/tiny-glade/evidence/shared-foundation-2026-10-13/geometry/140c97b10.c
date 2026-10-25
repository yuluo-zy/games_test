utils::geometry::circle::Circle2d::is_point_inside

bool tg_math_140c97b10(undefined8 *param_1,float param_2,float param_3)

{
  param_2 = param_2 - (float)*param_1;
  param_3 = param_3 - (float)((ulonglong)*param_1 >> 0x20);
  return SQRT(param_3 * param_3 + param_2 * param_2) < *(float *)(param_1 + 1);
}

