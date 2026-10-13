
void FUN_142879c20(longlong *param_1,ulonglong *param_2)

{
  ulonglong uVar1;
  ulonglong uVar2;
  longlong lVar3;
  ulonglong uVar4;
  longlong lStack_50;
  longlong lStack_48;
  longlong lStack_40;
  undefined8 uStack_38;
  undefined8 uStack_30;
  
  if (SBORROW8(0,*param_1)) {
    uVar1 = 2;
    if ((*param_2 ^ 0x8000000000000000) < 2) {
      uVar1 = *param_2 ^ 0x8000000000000000;
    }
    lVar3 = -0x8000000000000000;
    if (uVar1 != 0) {
      if (uVar1 == 1) {
        lVar3 = -0x7fffffffffffffff;
      }
      else {
        FUN_140d7bd90(&lStack_50);
        lVar3 = lStack_50;
      }
    }
    lStack_50 = lVar3;
    param_1[2] = lStack_40;
    *param_1 = lStack_50;
    param_1[1] = lStack_48;
    lVar3 = param_1[6];
  }
  else {
    lVar3 = param_1[6];
  }
  if (lVar3 == -0x7fffffffffffffff) {
    uVar1 = param_2[6];
    uVar2 = uVar1 + 0x7fffffffffffffff;
    uVar4 = 2;
    if (uVar2 < 2) {
      uVar4 = uVar2;
    }
    lVar3 = -0x7fffffffffffffff;
    if (uVar4 != 0) {
      if (uVar4 == 2) {
        lVar3 = -0x8000000000000000;
        if (!SBORROW8(0,uVar1)) {
          FUN_140d7bd90(&lStack_50,param_2 + 6);
          lVar3 = lStack_50;
        }
        uStack_38 = lStack_48;
        uStack_30 = lStack_40;
      }
      else {
        lVar3 = -0x7ffffffffffffffe;
      }
    }
    param_1[6] = lVar3;
    *(undefined4 *)(param_1 + 7) = (undefined4)uStack_38;
    *(undefined4 *)((longlong)param_1 + 0x3c) = uStack_38._4_4_;
    *(undefined4 *)(param_1 + 8) = (undefined4)uStack_30;
    *(undefined4 *)((longlong)param_1 + 0x44) = uStack_30._4_4_;
    lVar3 = param_1[3];
  }
  else {
    lVar3 = param_1[3];
  }
  if (SBORROW8(0,lVar3)) {
    uVar1 = 2;
    if ((param_2[3] ^ 0x8000000000000000) < 2) {
      uVar1 = param_2[3] ^ 0x8000000000000000;
    }
    if (uVar1 == 2) {
      FUN_140d7bd90(&lStack_50);
    }
    else if (uVar1 == 1) {
      lStack_50 = -0x7fffffffffffffff;
    }
    else {
      lStack_50 = -0x8000000000000000;
    }
    param_1[5] = lStack_40;
    param_1[3] = lStack_50;
    param_1[4] = lStack_48;
  }
  return;
}

