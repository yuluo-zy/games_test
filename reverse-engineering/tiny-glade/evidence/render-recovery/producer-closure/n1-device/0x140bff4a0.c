
longlong FUN_140bff4a0(longlong param_1,longlong param_2)

{
  undefined4 *puVar1;
  undefined1 uStack_91;
  longlong lStack_90;
  undefined *puStack_88;
  undefined8 uStack_80;
  undefined8 uStack_78;
  undefined4 uStack_70;
  undefined4 uStack_6c;
  undefined4 uStack_68;
  undefined4 uStack_64;
  undefined8 uStack_60;
  undefined *puStack_58;
  undefined8 uStack_50;
  undefined4 uStack_48;
  undefined4 uStack_44;
  undefined *puStack_40;
  undefined8 uStack_38;
  undefined8 uStack_30;
  undefined8 uStack_28;
  undefined8 uStack_20;
  
  if (3 < *puRam000000014362c3a8) {
    puVar1 = (undefined4 *)func_0x00014067dd80(&UNK_142b172b0);
    uStack_70 = *puVar1;
    uStack_6c = puVar1[1];
    uStack_68 = puVar1[2];
    uStack_64 = puVar1[3];
    uStack_44 = puVar1[4];
    uStack_60 = 4;
    puStack_58 = &UNK_142b172c8;
    uStack_50 = 0x17;
    puStack_40 = &UNK_142b17278;
    uStack_38 = 1;
    uStack_30 = 8;
    lStack_90 = 0;
    uStack_28 = 0;
    uStack_20 = 0;
    puStack_88 = &UNK_142b172c8;
    uStack_80 = 0x17;
    uStack_78 = 0;
    uStack_48 = 1;
    func_0x000140d4a330(&uStack_91,&lStack_90);
  }
  lStack_90 = param_2;
  FUN_140bff5a0(param_1,&lStack_90,0x2000);
  *(undefined4 *)(param_1 + 0x50) = *(undefined4 *)(*(longlong *)(param_2 + 0x658) + 0x3a8);
  return param_1;
}

