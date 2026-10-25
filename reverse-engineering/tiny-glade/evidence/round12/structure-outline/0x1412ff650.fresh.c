
ulonglong * FUN_1412ff650(ulonglong *param_1,undefined4 *param_2,undefined8 param_3)

{
  ulonglong uVar1;
  ulonglong uVar2;
  undefined1 auVar3 [16];
  ulonglong uVar4;
  ulonglong uVar5;
  ulonglong uVar6;
  undefined4 uStack_b8;
  undefined4 uStack_b4;
  undefined4 uStack_b0;
  undefined4 uStack_ac;
  undefined4 uStack_a8;
  undefined4 uStack_a4;
  undefined4 uStack_a0;
  undefined4 uStack_9c;
  ulonglong uStack_98;
  ulonglong uStack_90;
  ulonglong *puStack_80;
  undefined8 uStack_78;
  ulonglong uStack_70;
  undefined8 uStack_68;
  ulonglong uStack_60;
  ulonglong uStack_58;
  ulonglong auStack_50 [2];
  
  auStack_50[1] = 0xfffffffffffffffe;
  uVar1 = *(ulonglong *)(param_2 + 8);
  uVar2 = *(ulonglong *)(param_2 + 10);
  uVar6 = 0;
  uVar5 = uVar2 - uVar1;
  if (uVar2 < uVar1) {
    uVar5 = uVar6;
  }
  auVar3._8_8_ = 0;
  auVar3._0_8_ = uVar5;
  uVar4 = SUB168(auVar3 * ZEXT816(0xc),0);
  uStack_68 = param_3;
  if ((SUB168(auVar3 * ZEXT816(0xc),8) == 0) && (uVar4 < 0x7ffffffffffffffd)) {
    if (uVar4 != 0) {
      uVar6 = 4;
      uStack_70 = func_0x000140613c10(uVar4,4);
      if (uStack_70 != 0) goto LAB_1412ff6ec;
      goto LAB_1412ff6d5;
    }
  }
  else {
LAB_1412ff6d5:
    FUN_1428d8fe3(uVar6,uVar4,uStack_68);
  }
  uStack_70 = 4;
  uVar5 = 0;
LAB_1412ff6ec:
  puStack_80 = auStack_50;
  auStack_50[0] = 0;
  uStack_b8 = *param_2;
  uStack_b4 = param_2[1];
  uStack_b0 = param_2[2];
  uStack_ac = param_2[3];
  uStack_a8 = param_2[4];
  uStack_a4 = param_2[5];
  uStack_a0 = param_2[6];
  uStack_9c = param_2[7];
  uStack_78 = 0;
  uStack_98 = uVar1;
  uStack_90 = uVar2;
  uStack_60 = uVar5;
  uStack_58 = uStack_70;
  FUN_141476ba0(&uStack_b8,&puStack_80);
  param_1[2] = auStack_50[0];
  *param_1 = uStack_60;
  param_1[1] = uStack_58;
  return param_1;
}

