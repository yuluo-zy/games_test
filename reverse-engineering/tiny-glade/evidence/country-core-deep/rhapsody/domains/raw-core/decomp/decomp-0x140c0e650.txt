
undefined8 FUN_140c0e650(undefined8 param_1,longlong param_2)

{
  undefined8 uStack_30;
  undefined8 uStack_28;
  longlong lStack_20;
  undefined8 *puStack_18;
  undefined8 *puStack_10;
  
  uStack_30 = *(undefined8 *)(*(longlong *)(param_2 + 0x658) + 0xd0);
  uStack_28 = *(undefined8 *)(*(longlong *)(param_2 + 0x650) + 0x40);
  lStack_20 = *(longlong *)(param_2 + 0x650) + 0x10;
  puStack_18 = &uStack_30;
  puStack_10 = &uStack_28;
  FUN_1418b0490(param_1,&lStack_20);
  return param_1;
}

