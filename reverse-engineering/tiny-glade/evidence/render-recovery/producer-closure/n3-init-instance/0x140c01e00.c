
undefined8 * FUN_140c01e00(undefined8 *param_1,undefined8 param_2)

{
  longlong lStack_38;
  longlong lStack_30;
  longlong lStack_28;
  longlong lStack_20;
  undefined8 uStack_18;
  
  FUN_1418cd4d0(&lStack_38);
  if (SBORROW8(0,lStack_38)) {
    *(undefined4 *)(param_1 + 1) = (undefined4)lStack_30;
    *param_1 = 0x8000000000000000;
  }
  else {
    lStack_20 = lStack_30 + lStack_28 * 8;
    lStack_28 = lStack_38;
    uStack_18 = param_2;
    FUN_1418c9610(param_1,&lStack_38);
  }
  return param_1;
}

