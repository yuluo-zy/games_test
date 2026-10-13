
undefined8 * FUN_140dbe760(undefined8 *param_1,undefined8 param_2,ulonglong *param_3)

{
  ulonglong uVar1;
  undefined8 uVar2;
  undefined8 *puVar3;
  ulonglong uVar4;
  longlong lVar5;
  
  uVar1 = *param_3;
  uVar4 = 2;
  if ((uVar1 ^ 0x8000000000000000) < 2) {
    uVar4 = uVar1 ^ 0x8000000000000000;
  }
  if (uVar4 != 0) {
    if (uVar4 != 1) {
      uVar4 = param_3[1];
      FUN_140dbaed0(param_2,uVar4,param_3[2]);
      *param_1 = 0x800000000000001e;
      if (uVar1 == 0) {
        return param_1;
      }
      func_0x000140613c20(uVar4,uVar1,1);
      return param_1;
    }
    lVar5 = func_0x000140b7f2d0(param_3[1],param_3[2]);
    if (lVar5 != 0) {
      if (((uint)lVar5 & 3) == 1) {
        uVar2 = *(undefined8 *)(lVar5 + -1);
        puVar3 = *(undefined8 **)(lVar5 + 7);
        if ((code *)*puVar3 != (code *)0x0) {
          (*(code *)*puVar3)(uVar2);
        }
        if (puVar3[1] != 0) {
          func_0x000140613c20(uVar2,puVar3[1],puVar3[2]);
        }
        func_0x000140613c20(lVar5 + -1,0x18,8);
      }
      *param_1 = 0x800000000000001e;
      return param_1;
    }
  }
  *param_1 = 0x8000000000000001;
  param_1[1] = 0;
  return param_1;
}

