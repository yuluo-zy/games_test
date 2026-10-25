
undefined8 * FUN_140bf0860(undefined8 *param_1,undefined8 param_2,int *param_3)

{
  undefined8 uVar1;
  undefined8 uVar2;
  undefined8 uVar3;
  undefined8 uVar4;
  undefined8 uVar5;
  undefined8 uVar6;
  undefined8 uVar7;
  undefined4 *puVar8;
  undefined8 uStack_b0;
  undefined *puStack_a8;
  undefined8 uStack_a0;
  undefined8 uStack_98;
  undefined4 uStack_90;
  undefined4 uStack_8c;
  undefined4 uStack_88;
  undefined4 uStack_84;
  undefined8 uStack_80;
  undefined *puStack_78;
  undefined8 uStack_70;
  undefined4 uStack_68;
  undefined4 uStack_64;
  undefined *puStack_60;
  undefined8 uStack_58;
  undefined8 uStack_50;
  undefined8 uStack_48;
  undefined8 uStack_40;
  int *piStack_30;
  undefined1 uStack_21;
  undefined8 uStack_20;
  
  uStack_20 = 0xfffffffffffffffe;
  piStack_30 = param_3;
  if ((*param_3 == 9) && ((param_3[8] == 2 || (param_3[8] == -4)))) {
    if (*plRam000000014362c3a8 != 0) {
      puVar8 = (undefined4 *)func_0x00014067dd80(&UNK_142b15d50);
      uStack_90 = *puVar8;
      uStack_8c = puVar8[1];
      uStack_88 = puVar8[2];
      uStack_84 = puVar8[3];
      uStack_64 = puVar8[4];
      uStack_80 = 1;
      puStack_78 = &UNK_142b15d68;
      uStack_70 = 0x14;
      puStack_60 = &UNK_142b15d20;
      uStack_58 = 1;
      uStack_50 = 8;
      uStack_b0 = 0;
      uStack_48 = 0;
      uStack_40 = 0;
      puStack_a8 = &UNK_142b15d68;
      uStack_a0 = 0x14;
      uStack_98 = 0;
      uStack_68 = 1;
      func_0x000140d4a330(&uStack_21,&uStack_b0);
    }
    FUN_140befd60(param_2);
  }
  uVar1 = *(undefined8 *)(piStack_30 + 0x12);
  param_1[8] = *(undefined8 *)(piStack_30 + 0x10);
  param_1[9] = uVar1;
  uVar1 = *(undefined8 *)piStack_30;
  uVar2 = *(undefined8 *)(piStack_30 + 2);
  uVar3 = *(undefined8 *)(piStack_30 + 4);
  uVar4 = *(undefined8 *)(piStack_30 + 6);
  uVar5 = *(undefined8 *)(piStack_30 + 8);
  uVar6 = *(undefined8 *)(piStack_30 + 10);
  uVar7 = *(undefined8 *)(piStack_30 + 0xe);
  param_1[6] = *(undefined8 *)(piStack_30 + 0xc);
  param_1[7] = uVar7;
  param_1[4] = uVar5;
  param_1[5] = uVar6;
  param_1[2] = uVar3;
  param_1[3] = uVar4;
  *param_1 = uVar1;
  param_1[1] = uVar2;
  return param_1;
}

