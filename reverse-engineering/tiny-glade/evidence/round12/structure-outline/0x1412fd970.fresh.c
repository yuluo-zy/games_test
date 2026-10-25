
ulonglong * FUN_1412fd970(ulonglong *param_1,undefined4 *param_2,undefined8 param_3)

{
  longlong lVar1;
  longlong lVar2;
  undefined1 auVar3 [16];
  ulonglong uVar4;
  ulonglong uVar5;
  undefined8 uVar6;
  undefined4 uStack_c8;
  undefined4 uStack_c4;
  undefined4 uStack_c0;
  undefined4 uStack_bc;
  undefined4 uStack_b8;
  undefined4 uStack_b4;
  undefined4 uStack_b0;
  undefined4 uStack_ac;
  longlong lStack_a8;
  longlong lStack_a0;
  undefined8 uStack_98;
  undefined8 uStack_90;
  ulonglong *puStack_80;
  undefined8 uStack_78;
  ulonglong uStack_70;
  undefined8 uStack_68;
  ulonglong uStack_60;
  ulonglong uStack_58;
  ulonglong auStack_50 [2];
  
  auStack_50[1] = 0xfffffffffffffffe;
  lVar1 = *(longlong *)(param_2 + 8);
  lVar2 = *(longlong *)(param_2 + 10);
  uVar5 = lVar2 - lVar1;
  uVar6 = 0;
  auVar3._8_8_ = 0;
  auVar3._0_8_ = uVar5;
  uVar4 = SUB168(auVar3 * ZEXT816(0xc),0);
  uStack_68 = param_3;
  if ((SUB168(auVar3 * ZEXT816(0xc),8) == 0) && (uVar4 < 0x7ffffffffffffffd)) {
    if (uVar4 != 0) {
      uVar6 = 4;
      uStack_70 = func_0x000140613c10(uVar4,4);
      if (uStack_70 != 0) goto LAB_1412fda07;
      goto LAB_1412fd9f1;
    }
  }
  else {
LAB_1412fd9f1:
    FUN_1428d8fe3(uVar6,uVar4,uStack_68);
  }
  uStack_70 = 4;
  uVar5 = 0;
LAB_1412fda07:
  puStack_80 = auStack_50;
  auStack_50[0] = 0;
  uStack_98 = *(undefined8 *)(param_2 + 0xc);
  uStack_90 = *(undefined8 *)(param_2 + 0xe);
  uStack_c8 = *param_2;
  uStack_c4 = param_2[1];
  uStack_c0 = param_2[2];
  uStack_bc = param_2[3];
  uStack_b8 = param_2[4];
  uStack_b4 = param_2[5];
  uStack_b0 = param_2[6];
  uStack_ac = param_2[7];
  uStack_78 = 0;
  lStack_a8 = lVar1;
  lStack_a0 = lVar2;
  uStack_60 = uVar5;
  uStack_58 = uStack_70;
  FUN_141475090(&uStack_c8,&puStack_80);
  param_1[2] = auStack_50[0];
  *param_1 = uStack_60;
  param_1[1] = uStack_58;
  return param_1;
}

