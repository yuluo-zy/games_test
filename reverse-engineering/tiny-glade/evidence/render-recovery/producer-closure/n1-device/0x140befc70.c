
undefined8 FUN_140befc70(undefined8 param_1,int param_2)

{
  undefined4 *puVar1;
  undefined1 uStack_89;
  undefined8 uStack_88;
  undefined *puStack_80;
  undefined8 uStack_78;
  undefined8 uStack_70;
  undefined4 uStack_68;
  undefined4 uStack_64;
  undefined4 uStack_60;
  undefined4 uStack_5c;
  undefined8 uStack_58;
  undefined *puStack_50;
  undefined8 uStack_48;
  undefined4 uStack_40;
  undefined4 uStack_3c;
  undefined *puStack_38;
  undefined8 uStack_30;
  undefined8 uStack_28;
  undefined8 uStack_20;
  undefined8 uStack_18;
  
  if ((param_2 != 2) && (param_2 != -4)) {
    return 0;
  }
  if (*plRam000000014362c3a8 != 0) {
    puVar1 = (undefined4 *)func_0x00014067dd80(&UNK_142b15d50);
    uStack_68 = *puVar1;
    uStack_64 = puVar1[1];
    uStack_60 = puVar1[2];
    uStack_5c = puVar1[3];
    uStack_3c = puVar1[4];
    uStack_58 = 1;
    puStack_50 = &UNK_142b15d68;
    uStack_48 = 0x14;
    puStack_38 = &UNK_142b15d20;
    uStack_30 = 1;
    uStack_28 = 8;
    uStack_88 = 0;
    uStack_20 = 0;
    uStack_18 = 0;
    puStack_80 = &UNK_142b15d68;
    uStack_78 = 0x14;
    uStack_70 = 0;
    uStack_40 = 1;
    func_0x000140d4a330(&uStack_89,&uStack_88);
  }
  FUN_140befd60(param_1);
  return 1;
}

