
ulonglong *
FUN_140c90350(ulonglong *param_1,ulonglong param_2,undefined4 param_3,undefined8 param_4,
             undefined8 param_5)

{
  code *pcVar1;
  ulonglong *puVar2;
  undefined8 unaff_R12;
  ulonglong unaff_R15;
  ulonglong uStack_68;
  ulonglong uStack_60;
  ulonglong uStack_58;
  undefined8 uStack_50;
  
  uStack_50 = 0xfffffffffffffffe;
  if (param_2 < 2) {
    FUN_1428d9430(&UNK_142b2d880,0x1d,param_5);
  }
  else {
    unaff_R15 = param_2 * 4;
    unaff_R12 = 0;
    if ((param_2 >> 0x3e == 0) && (unaff_R15 < 0x7ffffffffffffffd)) {
      unaff_R12 = 4;
      uStack_60 = func_0x000140613c10(unaff_R15,4);
      if (uStack_60 != 0) {
        uStack_58 = 0;
        uStack_68 = param_2;
        FUN_140c90050(param_2,param_3,param_4,&uStack_68,param_5);
        param_1[2] = uStack_58;
        *param_1 = uStack_68;
        param_1[1] = uStack_60;
        return param_1;
      }
    }
  }
  FUN_1428d8fe3(unaff_R12,unaff_R15,param_5);
  pcVar1 = (code *)swi(3);
  puVar2 = (ulonglong *)(*pcVar1)();
  return puVar2;
}

