
undefined8 * FUN_140904990(undefined8 *param_1,undefined4 *param_2,longlong param_3)

{
  undefined8 uVar1;
  undefined8 auStack_40 [3];
  undefined4 *puStack_28;
  undefined8 uStack_20;
  
  uStack_20 = 0xfffffffffffffffe;
  puStack_28 = param_2;
  uVar1 = FUN_140be79d0(*(undefined8 *)(param_3 + 0x10),*param_2);
  auStack_40[0] = 0;
  FUN_1416f7c30(param_3,uVar1,auStack_40,0);
  *param_1 = 0xb;
  func_0x000140613c20(puStack_28,4,4);
  return param_1;
}

