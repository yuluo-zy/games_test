
undefined8 FUN_1417390e0(int param_1)

{
  code *pcVar1;
  undefined8 uVar2;
  uint uVar3;
  int iStack_44;
  int *piStack_40;
  undefined *puStack_38;
  undefined *puStack_30;
  undefined8 uStack_28;
  undefined4 **ppuStack_20;
  undefined8 uStack_18;
  undefined8 uStack_10;
  
  if (param_1 - 1U < 0x82) {
    return 1;
  }
  uVar3 = param_1 - 0x83;
  if (uVar3 < 0x10) {
    if ((0xccc0U >> (uVar3 & 0x1f) & 1) != 0) {
      return 0x10;
    }
    if (((3U >> (uVar3 & 0x1f) & 1) != 0) || (uVar3 == 8)) {
      return 8;
    }
  }
  piStack_40 = &iStack_44;
  puStack_38 = &UNK_140c87d80;
  puStack_30 = &UNK_142ccd868;
  uStack_28 = 1;
  uStack_10 = 0;
  ppuStack_20 = &piStack_40;
  uStack_18 = 1;
  iStack_44 = param_1;
  FUN_1428d9390(&puStack_30,&UNK_142ccd878);
  pcVar1 = (code *)swi(3);
  uVar2 = (*pcVar1)();
  return uVar2;
}

