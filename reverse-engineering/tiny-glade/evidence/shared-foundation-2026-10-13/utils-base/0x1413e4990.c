longlong * FUN_1413e4990(longlong *param_1,undefined8 param_2)
{
  longlong lStack_38;
  longlong lStack_30;
  longlong lStack_28;
  longlong *plStack_20;
  undefined8 uStack_18;
  uStack_18 = 0xfffffffffffffffe;
  lStack_38 = -0x8000000000000000;
  plStack_20 = (longlong *)FUN_1414186d0(param_2,&UNK_142a9eaf1,0x4f,&UNK_142c410d8,1,&lStack_38);
  if (plStack_20 != (longlong *)0x0) {
    if (*plStack_20 == 1) {
      FUN_1405e7a00(plStack_20 + 1);
    }
    else if ((*plStack_20 == 0) && (plStack_20[2] != 0)) {
      func_0x000140613c20(plStack_20[1],plStack_20[2],1);
    }
    func_0x000140613c20(plStack_20,0x28,8);
  }
  if (SBORROW8(0,lStack_38)) {
    *param_1 = 0;
    param_1[1] = 8;
    param_1[2] = 0;
  }
  else {
    param_1[2] = lStack_28;
    *param_1 = lStack_38;
    param_1[1] = lStack_30;
  }
  return param_1;
}
