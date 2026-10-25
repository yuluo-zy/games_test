
undefined8 FUN_140b91620(undefined8 param_1,undefined8 *param_2)

{
  longlong lVar1;
  undefined8 *puVar2;
  undefined4 *puVar3;
  uint uStack_178;
  undefined1 auStack_160 [16];
  undefined1 auStack_150 [320];
  
  lVar1 = 0;
  if ((longlong)param_2[7] < -0x7ffffffffffffffe) {
    lVar1 = param_2[7] + -0x7fffffffffffffff;
  }
  if (lVar1 == 0) {
    puVar2 = param_2 + 0x2a;
    puVar3 = (undefined4 *)((longlong)param_2 + 0x154);
  }
  else if (lVar1 == 1) {
    puVar2 = (undefined8 *)(param_2[8] + 0x118);
    puVar3 = (undefined4 *)(param_2[8] + 0x11c);
  }
  else {
    puVar2 = (undefined8 *)(param_2[8] + 0x128);
    puVar3 = (undefined4 *)(param_2[8] + 300);
  }
  FUN_140e15190(auStack_150,*(undefined4 *)puVar2,*puVar3);
  func_0x000142923c00(param_1,param_2,0x1a0);
  *param_2 = 0;
  param_2[4] = 0x8000000000000000;
  func_0x000142923c00(param_2 + 5,auStack_160,0x150);
  *(undefined4 *)(param_2 + 0x2f) = 0;
  *(ulonglong *)((longlong)param_2 + 0x17c) = (ulonglong)uStack_178;
  *(undefined8 *)((longlong)param_2 + 0x184) = 0;
  *(undefined8 *)((longlong)param_2 + 0x189) = 0;
  *(undefined4 *)(param_2 + 0x33) = 4;
  return param_1;
}

