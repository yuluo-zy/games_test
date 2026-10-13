utils::geometry::line_relation::LineInterval::line_segment

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

undefined8 * tg_math_14261f450(undefined8 *param_1,undefined8 *param_2)

{
  undefined8 uVar1;
  
  uVar1 = param_2[1];
  *param_1 = *param_2;
  param_1[1] = uVar1;
  param_1[2] = _DAT_1429345b0;
  return param_1;
}

