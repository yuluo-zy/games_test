
undefined4 * FUN_1406982e0(undefined4 *param_1,longlong param_2)

{
  undefined8 uVar1;
  undefined8 uVar2;
  
  uVar1 = *(undefined8 *)(param_2 + 8);
  uVar2 = GetWindowLongPtrW(uVar1,0xfffffffa);
  *param_1 = 8;
  *(undefined8 *)(param_1 + 2) = uVar1;
  *(undefined8 *)(param_1 + 4) = uVar2;
  return param_1;
}

