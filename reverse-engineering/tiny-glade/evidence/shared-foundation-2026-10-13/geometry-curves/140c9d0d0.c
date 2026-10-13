
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

bool FUN_140c9d0d0(float *param_1,float *param_2,float param_3)

{
  undefined1 auVar1 [16];
  undefined1 auVar2 [16];
  undefined1 auVar3 [16];
  undefined4 in_EAX;
  int iVar4;
  int iVar5;
  
  iVar4 = -(uint)(param_3 <
                 (float)((uint)((float)*(undefined8 *)(param_1 + 4) -
                               (float)*(undefined8 *)(param_2 + 4)) & _DAT_14292e980));
  iVar5 = -(uint)(param_3 <
                 (float)((uint)((float)((ulonglong)*(undefined8 *)(param_1 + 4) >> 0x20) -
                               (float)((ulonglong)*(undefined8 *)(param_2 + 4) >> 0x20)) &
                        _UNK_14292e984));
  auVar1._4_4_ = iVar4;
  auVar1._0_4_ = iVar4;
  auVar1._8_4_ = iVar5;
  auVar1._12_4_ = iVar5;
  iVar4 = movmskpd(in_EAX,auVar1);
  if (iVar4 != 0) {
    return false;
  }
  if (param_3 < (float)((uint)(param_1[6] - param_2[6]) & _DAT_14292e980)) {
    return false;
  }
  auVar2._4_4_ = -(uint)((float)((uint)(param_1[1] - param_2[1]) & _UNK_14292e984) <= param_3);
  auVar2._0_4_ = -(uint)((float)((uint)(*param_1 - *param_2) & _DAT_14292e980) <= param_3);
  auVar2._8_4_ = -(uint)((float)((uint)(param_1[2] - param_2[2]) & _UNK_14292e988) <= param_3);
  auVar2._12_4_ = -(uint)((float)((uint)(param_1[3] - param_2[3]) & _UNK_14292e98c) <= param_3);
  iVar4 = movmskps(0,auVar2);
  if (iVar4 != 0xf) {
    return false;
  }
  iVar4 = -(uint)(param_3 <
                 (float)((uint)((float)*(undefined8 *)(param_1 + 7) -
                               (float)*(undefined8 *)(param_2 + 7)) & _DAT_14292e980));
  iVar5 = -(uint)(param_3 <
                 (float)((uint)((float)((ulonglong)*(undefined8 *)(param_1 + 7) >> 0x20) -
                               (float)((ulonglong)*(undefined8 *)(param_2 + 7) >> 0x20)) &
                        _UNK_14292e984));
  auVar3._4_4_ = iVar4;
  auVar3._0_4_ = iVar4;
  auVar3._8_4_ = iVar5;
  auVar3._12_4_ = iVar5;
  iVar4 = movmskpd(0xf,auVar3);
  if (iVar4 != 0) {
    return false;
  }
  return (float)((uint)(param_1[9] - param_2[9]) & _DAT_14292e980) <= param_3;
}

