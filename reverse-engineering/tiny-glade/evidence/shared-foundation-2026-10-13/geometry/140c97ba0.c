utils::geometry::circle::Circle2d::as_points2_ptcount

undefined8 * tg_math_140c97ba0(undefined8 *param_1,undefined8 param_2,undefined8 param_3)

{
  param_1[2] = 0;
  param_1[3] = param_3;
  *param_1 = param_2;
  param_1[1] = param_3;
  return param_1;
}

