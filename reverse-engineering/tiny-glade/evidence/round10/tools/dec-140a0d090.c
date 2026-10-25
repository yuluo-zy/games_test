
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

undefined8 * FUN_140a0d090(undefined8 *param_1,longlong param_2,undefined4 param_3)

{
  undefined8 *puVar1;
  undefined8 *puVar2;
  undefined4 uStack_88;
  undefined4 uStack_84;
  undefined4 uStack_80;
  undefined4 uStack_7c;
  undefined4 uStack_78;
  undefined4 uStack_74;
  undefined4 uStack_70;
  undefined4 uStack_64;
  undefined4 uStack_60;
  undefined4 uStack_5c;
  undefined4 uStack_58;
  undefined4 uStack_54;
  undefined4 uStack_50;
  undefined4 uStack_4c;
  
  if (*(int *)(param_2 + 0x38) != *(int *)(param_2 + 0x3c)) {
    *(int *)(param_2 + 0x38) = *(int *)(param_2 + 0x3c);
    param_3 = _UNK_14295b630;
  }
  uStack_78 = _UNK_142aece50;
  uStack_74 = (undefined4)_UNK_142aece54;
  uStack_70 = (undefined4)((ulonglong)_UNK_142aece54 >> 0x20);
  uStack_88 = _UNK_142aece40;
  uStack_84 = _UNK_142aece44;
  uStack_80 = _UNK_142aece48;
  uStack_7c = _UNK_142aece4c;
  if (*(longlong *)(param_2 + 0x10) == 0) {
    uStack_54 = 0;
    uStack_50 = 0;
    uStack_4c = 0;
    uStack_64 = _UNK_14293e0c0;
    uStack_60 = _UNK_14293e0c4;
    uStack_5c = _UNK_14293e0c8;
    uStack_58 = _UNK_14293e0cc;
  }
  else {
    puVar1 = *(undefined8 **)(param_2 + 8);
    puVar2 = puVar1 + *(longlong *)(param_2 + 0x10) * 2;
    do {
      (**(code **)(puVar1[1] + 0x30))(&uStack_64,*puVar1,&uStack_88,param_3);
      uStack_88 = uStack_64;
      uStack_84 = uStack_60;
      uStack_80 = uStack_5c;
      uStack_7c = uStack_58;
      uStack_78 = uStack_54;
      uStack_74 = uStack_50;
      uStack_70 = uStack_4c;
      puVar1 = puVar1 + 2;
    } while (puVar1 != puVar2);
  }
  *(ulonglong *)(param_2 + 0x24) = CONCAT44(uStack_78,uStack_7c);
  *(ulonglong *)(param_2 + 0x2c) = CONCAT44(uStack_70,uStack_74);
  *(ulonglong *)(param_2 + 0x18) = CONCAT44(uStack_84,uStack_88);
  *(ulonglong *)(param_2 + 0x20) = CONCAT44(uStack_7c,uStack_80);
  *param_1 = CONCAT44(uStack_50,uStack_54);
  *(undefined4 *)(param_1 + 1) = uStack_4c;
  *(undefined4 *)(param_1 + 2) = uStack_64;
  *(undefined4 *)((longlong)param_1 + 0x14) = uStack_60;
  *(undefined4 *)(param_1 + 3) = uStack_5c;
  *(undefined4 *)((longlong)param_1 + 0x1c) = uStack_58;
  return param_1;
}

