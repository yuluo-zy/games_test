
ulonglong * FUN_140dce930(ulonglong *param_1,undefined4 *param_2,undefined8 param_3)

{
  int iVar1;
  int iVar2;
  undefined1 auVar3 [16];
  ulonglong uVar4;
  ulonglong uVar5;
  ulonglong uVar6;
  undefined4 uStack_c8;
  undefined4 uStack_c4;
  undefined4 uStack_c0;
  undefined4 uStack_bc;
  undefined4 uStack_b8;
  undefined4 uStack_b4;
  undefined4 uStack_b0;
  undefined4 uStack_ac;
  undefined4 uStack_a8;
  undefined4 uStack_a4;
  undefined4 uStack_a0;
  undefined4 uStack_9c;
  undefined8 uStack_98;
  int iStack_90;
  int iStack_8c;
  ulonglong *puStack_80;
  undefined8 uStack_78;
  ulonglong uStack_70;
  undefined8 uStack_68;
  ulonglong uStack_60;
  ulonglong uStack_58;
  ulonglong auStack_50 [2];
  
  auStack_50[1] = 0xfffffffffffffffe;
  iVar1 = param_2[0xe];
  iVar2 = param_2[0xf];
  uVar6 = 0;
  uVar5 = (longlong)iVar2 - (longlong)iVar1;
  if (iVar2 <= iVar1) {
    uVar5 = uVar6;
  }
  auVar3._8_8_ = 0;
  auVar3._0_8_ = uVar5;
  uVar4 = SUB168(auVar3 * ZEXT816(0x18),0);
  uStack_68 = param_3;
  if ((SUB168(auVar3 * ZEXT816(0x18),8) == 0) && (uVar4 < 0x7ffffffffffffffd)) {
    if (uVar4 != 0) {
      uVar6 = 4;
      uStack_70 = func_0x000140613c10(uVar4,4);
      if (uStack_70 != 0) goto LAB_140dce9cf;
      goto LAB_140dce9b8;
    }
  }
  else {
LAB_140dce9b8:
    FUN_1428d8fe3(uVar6,uVar4,uStack_68);
  }
  uStack_70 = 4;
  uVar5 = 0;
LAB_140dce9cf:
  puStack_80 = auStack_50;
  auStack_50[0] = 0;
  uStack_98 = *(undefined8 *)(param_2 + 0xc);
  uStack_c8 = *param_2;
  uStack_c4 = param_2[1];
  uStack_c0 = param_2[2];
  uStack_bc = param_2[3];
  uStack_b8 = param_2[4];
  uStack_b4 = param_2[5];
  uStack_b0 = param_2[6];
  uStack_ac = param_2[7];
  uStack_a8 = param_2[8];
  uStack_a4 = param_2[9];
  uStack_a0 = param_2[10];
  uStack_9c = param_2[0xb];
  uStack_78 = 0;
  iStack_90 = iVar1;
  iStack_8c = iVar2;
  uStack_60 = uVar5;
  uStack_58 = uStack_70;
  FUN_1419f0790(&uStack_c8,&puStack_80);
  param_1[2] = auStack_50[0];
  *param_1 = uStack_60;
  param_1[1] = uStack_58;
  return param_1;
}

