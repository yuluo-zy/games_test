
void FUN_140be9420(longlong param_1)

{
  bool bVar1;
  undefined8 uStack_28;
  
  uStack_28 = *(longlong *)(param_1 + 0x10);
  *(undefined8 *)(param_1 + 0x10) = 0;
  while (bVar1 = uStack_28 != 0, uStack_28 = uStack_28 + -1, bVar1) {
    func_0x000140bdfc90();
  }
  *(undefined8 *)(param_1 + 0x28) = 0;
  *(undefined8 *)(param_1 + 0x40) = 0;
  return;
}

