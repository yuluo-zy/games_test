
void FUN_140b906d0(longlong param_1,undefined8 param_2)

{
  longlong lVar1;
  
  FUN_1417f8980();
  func_0x000142923c00(param_1,param_2,0x1a0);
  *(undefined4 *)(param_1 + 0x24c) = 0;
  lVar1 = *(longlong *)(param_1 + 0x1e8);
  if ((!SBORROW8(0,lVar1)) && (lVar1 != 0)) {
    func_0x000140613c20(*(undefined8 *)(param_1 + 0x1f0),lVar1,1);
  }
  *(undefined8 *)(param_1 + 0x1e8) = 0x8000000000000000;
  return;
}

