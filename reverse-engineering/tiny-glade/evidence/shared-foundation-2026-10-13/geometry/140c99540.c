utils::geometry::rectangle::Rectangle2d::orientation_mat2

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

undefined4 * tg_math_140c99540(undefined4 *param_1,undefined4 *param_2)

{
  undefined4 uVar1;
  uint uVar2;
  uint uVar3;
  
  uVar1 = *param_2;
  uVar2 = param_2[1];
  uVar3 = uVar2 ^ _UNK_1429258f8;
  *param_1 = uVar1;
  param_1[1] = uVar2;
  param_1[2] = uVar3;
  param_1[3] = uVar1;
  return param_1;
}

