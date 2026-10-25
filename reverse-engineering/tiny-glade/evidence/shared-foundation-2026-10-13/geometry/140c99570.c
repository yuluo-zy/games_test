utils::geometry::rectangle::Rectangle2d::orientation_mat3

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void tg_math_140c99570(undefined4 *param_1,undefined4 *param_2)

{
  undefined4 uVar1;
  uint uVar2;
  
  uVar1 = *param_2;
  uVar2 = param_2[1];
  *param_1 = uVar1;
  param_1[1] = 0;
  param_1[2] = uVar2;
  param_1[6] = uVar2 ^ _DAT_1429258f0;
  param_1[7] = 0;
  param_1[8] = uVar1;
  *(undefined8 *)(param_1 + 3) = 0x3f80000000000000;
  param_1[5] = 0;
  return;
}

