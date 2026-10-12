
undefined8 * FUN_141620a10(undefined8 *param_1,longlong param_2)

{
  undefined4 uVar1;
  undefined4 uVar2;
  undefined4 uVar3;
  undefined4 uVar4;
  undefined4 uVar5;
  undefined4 uVar6;
  undefined4 uVar7;
  undefined4 uVar8;
  undefined8 uStack_78;
  undefined8 uStack_70;
  undefined8 uStack_68;
  undefined4 uStack_60;
  undefined4 uStack_5c;
  undefined4 uStack_58;
  undefined4 uStack_54;
  undefined4 uStack_50;
  undefined4 uStack_4c;
  undefined4 uStack_48;
  undefined4 uStack_44;
  longlong lStack_40;
  longlong lStack_38;
  undefined8 uStack_30;
  undefined8 uStack_28;
  
  uStack_28 = 0xfffffffffffffffe;
  func_0x000140bd6b00(&uStack_78,param_2 + 0x40);
  uStack_30 = 0;
  lStack_40 = param_2;
  lStack_38 = param_2 + 0x40;
  FUN_140af5240(&uStack_60,&lStack_40);
  uVar8 = uStack_44;
  uVar7 = uStack_48;
  uVar6 = uStack_4c;
  uVar5 = uStack_50;
  uVar4 = uStack_54;
  uVar3 = uStack_58;
  uVar2 = uStack_5c;
  uVar1 = uStack_60;
  uStack_30 = 1;
  FUN_140af5240(&uStack_60,&lStack_40);
  param_1[6] = CONCAT44(uStack_4c,uStack_50);
  param_1[7] = CONCAT44(uStack_44,uStack_48);
  param_1[4] = CONCAT44(uStack_5c,uStack_60);
  param_1[5] = CONCAT44(uStack_54,uStack_58);
  param_1[2] = CONCAT44(uVar6,uVar5);
  param_1[3] = CONCAT44(uVar8,uVar7);
  *param_1 = CONCAT44(uVar2,uVar1);
  param_1[1] = CONCAT44(uVar4,uVar3);
  param_1[10] = uStack_68;
  param_1[8] = uStack_78;
  param_1[9] = uStack_70;
  *(undefined1 *)(param_1 + 0xb) = 0;
  return param_1;
}

