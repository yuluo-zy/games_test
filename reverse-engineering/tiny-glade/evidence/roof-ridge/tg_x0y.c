
undefined4 * tg_x0y(undefined4 *param_1,undefined4 *param_2)

{
  undefined4 uVar1;
  
  uVar1 = param_2[1];
  *param_1 = *param_2;
  param_1[1] = 0;
  param_1[2] = uVar1;
  return param_1;
}

