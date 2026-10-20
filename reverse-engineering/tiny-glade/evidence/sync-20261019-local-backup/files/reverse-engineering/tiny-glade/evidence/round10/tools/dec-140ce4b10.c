
ulonglong * FUN_140ce4b10(ulonglong *param_1,ulonglong param_2,ulonglong param_3,undefined8 param_4)

{
  code *pcVar1;
  ulonglong uVar2;
  longlong *plVar3;
  ulonglong uVar4;
  undefined8 uVar5;
  ulonglong uVar6;
  ulonglong uVar7;
  ulonglong uVar8;
  longlong lStack_78;
  undefined8 uStack_70;
  ulonglong uStack_68;
  longlong lStack_60;
  longlong lStack_58;
  undefined8 uStack_50;
  undefined8 uStack_48;
  
  uStack_48 = 0xfffffffffffffffe;
  uVar7 = param_2;
  uVar8 = param_3;
  (**(code **)(param_3 + 0xa0))(&lStack_78);
  uVar4 = uStack_68 << 4;
  uVar5 = 0;
  if ((uStack_68 >> 0x3c == 0) && (uVar4 < 0x7ffffffffffffff9)) {
    uStack_50 = uStack_70;
    if (uVar4 == 0) {
      uVar2 = 8;
      uVar6 = 0;
    }
    else {
      uVar5 = 8;
      uVar2 = func_0x000140613c10(uVar4,8);
      uVar6 = uStack_68;
      if (uVar2 == 0) goto code_r0x000140ce4c6b;
    }
    func_0x000142923c00(uVar2,uStack_50,uVar4);
    if (lStack_78 == 0) {
      plVar3 = &lStack_58;
      lStack_78 = 0;
    }
    else {
      lStack_78 = lStack_78 << 4;
      lStack_58 = 8;
      plVar3 = &lStack_60;
    }
    *plVar3 = lStack_78;
    if ((lStack_58 != 0) && (lStack_60 != 0)) {
      func_0x000140613c20(uStack_50);
    }
    param_1[0xc] = param_2;
    param_1[0xd] = param_3;
    *param_1 = uVar6;
    param_1[1] = uVar2;
    param_1[2] = uStack_68;
    param_1[3] = 0;
    param_1[4] = 8;
    param_1[5] = 0;
    param_1[6] = 0x8000000000000000;
    param_1[9] = 0;
    param_1[10] = 8;
    param_1[0xb] = 0;
    return param_1;
  }
code_r0x000140ce4c6b:
  FUN_1428d8fe3(uVar5,uVar4,&UNK_142e7d780,param_4,uVar7,uVar8);
                    /* WARNING: Does not return */
  pcVar1 = (code *)invalidInstructionException();
  (*pcVar1)();
}

