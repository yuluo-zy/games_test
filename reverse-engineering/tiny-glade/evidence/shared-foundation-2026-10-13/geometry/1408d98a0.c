utils::geometry::ray::Ray::xz

undefined4 * tg_math_1408d98a0(undefined4 *param_1,undefined4 *param_2)

{
  undefined4 uVar1;
  
  uVar1 = param_2[5];
  *param_1 = *param_2;
  *(undefined8 *)(param_1 + 1) = *(undefined8 *)(param_2 + 2);
  param_1[3] = uVar1;
  return param_1;
}

