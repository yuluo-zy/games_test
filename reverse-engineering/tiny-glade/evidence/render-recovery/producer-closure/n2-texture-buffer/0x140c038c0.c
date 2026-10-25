
longlong *
FUN_140c038c0(longlong *param_1,longlong param_2,undefined8 param_3,undefined8 param_4,
             undefined4 *param_5)

{
  longlong lVar1;
  longlong lStack_68;
  longlong lStack_60;
  longlong lStack_58;
  longlong lStack_50;
  longlong lStack_48;
  longlong lStack_40;
  longlong lStack_38;
  longlong lStack_30;
  longlong lStack_28;
  longlong lStack_20;
  undefined4 uStack_18;
  undefined4 uStack_14;
  undefined4 uStack_10;
  undefined4 uStack_c;
  
  lVar1 = param_2 + 0x870;
  if (*(longlong *)(param_2 + 0x870) == 0) {
    lVar1 = 0;
  }
  uStack_18 = *param_5;
  uStack_14 = param_5[1];
  uStack_10 = param_5[2];
  uStack_c = param_5[3];
  FUN_140bee200(&lStack_68,param_2 + 0x78,lVar1,*(undefined8 *)(param_2 + 0x648),
                *(longlong *)(param_2 + 0x678) + 0x10,&uStack_18,param_3,param_4);
  if (lStack_68 == 0xb) {
    param_1[5] = lStack_38;
    param_1[3] = lStack_48;
    param_1[4] = lStack_40;
    param_1[1] = lStack_58;
    param_1[2] = lStack_50;
  }
  else {
    param_1[5] = lStack_40;
    param_1[6] = lStack_38;
    param_1[3] = lStack_50;
    param_1[4] = lStack_48;
    param_1[1] = lStack_60;
    param_1[2] = lStack_58;
    param_1[7] = lStack_30;
    param_1[8] = lStack_28;
    param_1[9] = lStack_20;
  }
  *param_1 = lStack_68;
  return param_1;
}

