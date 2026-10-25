utils::geometry::circle::Circle2d::with_expanded_diameter

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

undefined8 * tg_math_140c980c0(undefined8 *param_1,undefined8 *param_2,float param_3)

{
  float fVar1;
  undefined4 uVar2;
  
  param_3 = param_3 * _DAT_142925870;
  fVar1 = *(float *)(param_2 + 1);
  uVar2 = *(undefined4 *)((longlong)param_2 + 0xc);
  *param_1 = *param_2;
  *(float *)(param_1 + 1) = param_3 + fVar1;
  *(undefined4 *)((longlong)param_1 + 0xc) = uVar2;
  return param_1;
}

