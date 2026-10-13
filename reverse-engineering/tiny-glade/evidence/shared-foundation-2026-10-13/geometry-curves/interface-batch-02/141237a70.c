
undefined4 * FUN_141237a70(undefined4 *param_1,ulonglong *param_2)

{
  uint uVar1;
  ulonglong uVar2;
  ulonglong uVar3;
  undefined4 uVar4;
  ulonglong uStack_78;
  undefined4 uStack_70;
  undefined4 uStack_6c;
  undefined4 uStack_68;
  undefined4 uStack_64;
  undefined4 uStack_60;
  undefined4 uStack_48;
  ulonglong uStack_44;
  undefined4 uStack_3c;
  ulonglong *puStack_38;
  ulonglong uStack_2c;
  undefined4 uStack_24;
  
  if (*param_2 == 2) {
LAB_141237ad2:
    uVar1 = (uint)param_2[5];
    if (uVar1 != 2) {
      uStack_78 = CONCAT44(uStack_78._4_4_,uVar1);
      uVar3 = (ulonglong)*(uint *)((longlong)param_2 + 0x2c);
      uVar4 = (undefined4)param_2[6];
      *(undefined4 *)(param_2 + 5) = 0;
      if ((uVar1 & 1) != 0) goto LAB_141237b13;
    }
LAB_141237c04:
    *param_1 = 0;
  }
  else {
    uVar1 = (uint)param_2[3];
    if (uVar1 == 2) {
LAB_141237ab0:
      if (((*param_2 & 1) == 0) || (uVar3 = param_2[1], (longlong)param_2[2] <= (longlong)uVar3)) {
LAB_141237acb:
        *param_2 = 2;
        goto LAB_141237ad2;
      }
      param_2[1] = uVar3 + 1;
      uVar4 = 0;
    }
    else {
      uVar3 = (ulonglong)*(uint *)((longlong)param_2 + 0x1c);
      uVar4 = (undefined4)param_2[4];
      *(uint *)(param_2 + 3) = (uint)(uVar1 == 0) * 2;
      if ((uVar1 & 1) == 0) goto LAB_141237ab0;
      if (uVar1 == 0) goto LAB_141237acb;
    }
LAB_141237b13:
    FUN_141206aa0(&uStack_2c,param_2 + 7,uVar3,uVar4);
    if ((param_2[8] & 1) == 0) {
      uStack_3c = uStack_24;
      uStack_44 = uStack_2c;
      uStack_48 = 1;
      puStack_38 = param_2;
      FUN_14123aa60(&uStack_78,&uStack_48);
      *(ulonglong *)((longlong)param_2 + 0x4c) = CONCAT44(uStack_68,uStack_6c);
      *(ulonglong *)((longlong)param_2 + 0x54) = CONCAT44(uStack_60,uStack_64);
      param_2[8] = uStack_78;
      param_2[9] = CONCAT44(uStack_6c,uStack_70);
      if ((param_2[8] & 1) == 0) goto LAB_141237c04;
      uStack_70 = *(undefined4 *)((longlong)param_2 + 0x4c);
      uStack_78 = *(ulonglong *)((longlong)param_2 + 0x44);
      uStack_6c = (undefined4)param_2[10];
      uStack_68 = (undefined4)(param_2[10] >> 0x20);
      uStack_24 = (undefined4)param_2[0xb];
    }
    else {
      uVar2 = param_2[0xb];
      uVar3 = param_2[10];
      param_2[10] = uStack_2c;
      *(undefined4 *)(param_2 + 0xb) = uStack_24;
      *(ulonglong *)((longlong)param_2 + 0x44) = uVar3;
      *(int *)((longlong)param_2 + 0x4c) = (int)uVar2;
      uStack_70 = *(undefined4 *)((longlong)param_2 + 0x4c);
      uStack_78 = *(ulonglong *)((longlong)param_2 + 0x44);
      uStack_6c = (undefined4)uStack_2c;
      uStack_68 = (undefined4)(uStack_2c >> 0x20);
    }
    *(ulonglong *)(param_1 + 1) = uStack_78;
    param_1[3] = uStack_70;
    param_1[4] = uStack_6c;
    *(ulonglong *)(param_1 + 5) = CONCAT44(uStack_24,uStack_68);
    *param_1 = 1;
  }
  return param_1;
}

