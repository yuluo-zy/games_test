
undefined8 FUN_140bd49d0(int param_1)

{
  code *pcVar1;
  undefined8 uVar2;
  int iStack_44;
  int *piStack_40;
  undefined *puStack_38;
  undefined *puStack_30;
  undefined8 uStack_28;
  undefined4 **ppuStack_20;
  undefined8 uStack_18;
  undefined8 uStack_10;
  
  if (param_1 < 0x220) {
    if (param_1 < 0x100) {
      if (param_1 == 0x20) {
        return 4;
      }
      if (param_1 == 0x40) {
        return 8;
      }
      if (param_1 == 0x80) {
        return 0x10;
      }
    }
    else {
      if ((param_1 == 0x100) || (param_1 == 0x180)) {
        return 0x10;
      }
      if (param_1 == 0x200) {
        return 0x20;
      }
    }
  }
  else if (param_1 < 0x800) {
    if ((param_1 == 0x400) || (param_1 == 0x600)) {
      return 0x20;
    }
    if (param_1 == 0x220) {
      return 0x24;
    }
  }
  else {
    if (param_1 == 0x800) {
      return 1;
    }
    if (param_1 == 0x1000) {
      return 2;
    }
    if (param_1 == 0x18000) {
      return 8;
    }
  }
  piStack_40 = &iStack_44;
  puStack_38 = &UNK_140c87a90;
  puStack_30 = &UNK_142b0f710;
  uStack_28 = 1;
  uStack_10 = 0;
  ppuStack_20 = &piStack_40;
  uStack_18 = 1;
  iStack_44 = param_1;
  FUN_1428d9390(&puStack_30,&UNK_142b0f720);
  pcVar1 = (code *)swi(3);
  uVar2 = (*pcVar1)();
  return uVar2;
}

