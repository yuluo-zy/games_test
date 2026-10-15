
undefined4 * FUN_1418cb100(undefined4 *param_1,ulonglong *param_2,undefined8 param_3)

{
  ulonglong uVar1;
  ulonglong uVar2;
  undefined4 uVar3;
  undefined4 uVar4;
  undefined4 uVar5;
  undefined4 uVar6;
  ulonglong uVar7;
  
  uVar2 = *param_2;
  if (uVar2 < 0xfffffffffffffff0 && (char)(((uint)uVar2 & 8) >> 3) == '\0') {
    LOCK();
    uVar1 = *param_2;
    if (uVar2 == uVar1) {
      *param_2 = uVar2 + 0x10;
    }
    UNLOCK();
    if (uVar2 == uVar1) goto code_r0x0001418cb140;
  }
  FUN_1428cc4b0(param_2,0,param_3,1000000000);
code_r0x0001418cb140:
  FUN_140c81c70(param_2);
  FUN_140c81c70((ulonglong)param_2 | 1);
  *(ulonglong *)(param_1 + 8) = param_2[5];
  uVar2 = param_2[1];
  uVar3 = *(undefined4 *)((longlong)param_2 + 0xc);
  uVar1 = param_2[2];
  uVar4 = *(undefined4 *)((longlong)param_2 + 0x14);
  uVar5 = *(undefined4 *)((longlong)param_2 + 0x1c);
  uVar7 = param_2[4];
  uVar6 = *(undefined4 *)((longlong)param_2 + 0x24);
  param_1[4] = (int)param_2[3];
  param_1[5] = uVar5;
  param_1[6] = (int)uVar7;
  param_1[7] = uVar6;
  *param_1 = (int)uVar2;
  param_1[1] = uVar3;
  param_1[2] = (int)uVar1;
  param_1[3] = uVar4;
  FUN_140c81e10(param_2);
  FUN_140c81e10((ulonglong)param_2 | 1);
  LOCK();
  uVar2 = *param_2;
  *param_2 = *param_2 - 0x10;
  UNLOCK();
  if ((uVar2 & 0xfffffffffffffff2) == 0x12) {
    FUN_1428ccf50(param_2);
  }
  return param_1;
}

