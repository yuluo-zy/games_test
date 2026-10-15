
longlong * FUN_140be9c80(longlong *param_1,longlong param_2,undefined4 param_3)

{
  code *pcVar1;
  undefined8 uVar2;
  longlong *plVar3;
  undefined4 uStack_64;
  undefined1 auStack_60 [8];
  longlong lStack_58;
  longlong lStack_50;
  longlong lStack_48;
  longlong lStack_40;
  longlong lStack_38;
  longlong lStack_30;
  longlong lStack_28;
  longlong lStack_20;
  
  uStack_64 = param_3;
  uVar2 = FUN_1405b0ed0(param_2 + 0x20,&uStack_64);
  FUN_1417e7db0(auStack_60,param_2,uVar2,&uStack_64);
  if (lStack_58 != 6) {
    param_1[7] = lStack_20;
    param_1[5] = lStack_30;
    param_1[6] = lStack_28;
    param_1[3] = lStack_40;
    param_1[4] = lStack_38;
    param_1[1] = lStack_50;
    param_1[2] = lStack_48;
    *param_1 = lStack_58;
    return param_1;
  }
  FUN_1428d9310(&UNK_142b14618);
  pcVar1 = (code *)swi(3);
  plVar3 = (longlong *)(*pcVar1)();
  return plVar3;
}

