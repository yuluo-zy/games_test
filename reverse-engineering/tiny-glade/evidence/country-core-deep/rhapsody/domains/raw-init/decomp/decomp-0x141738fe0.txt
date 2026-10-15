
ulonglong FUN_141738fe0(int param_1)

{
  code *pcVar1;
  ulonglong uVar2;
  int iStack_44;
  int *piStack_40;
  undefined *puStack_38;
  undefined *puStack_30;
  undefined8 uStack_28;
  undefined4 **ppuStack_20;
  undefined8 uStack_18;
  undefined8 uStack_10;
  
  if (param_1 - 1U < 0xb8) {
    return (ulonglong)*(uint *)(&UNK_142ccd890 + (ulonglong)(param_1 - 1U) * 4);
  }
  piStack_40 = &iStack_44;
  puStack_38 = &UNK_140c87d80;
  puStack_30 = &UNK_142ccd7b8;
  uStack_28 = 1;
  uStack_10 = 0;
  ppuStack_20 = &piStack_40;
  uStack_18 = 1;
  iStack_44 = param_1;
  FUN_1428d9390(&puStack_30,&UNK_142ccd7f0);
  pcVar1 = (code *)swi(3);
  uVar2 = (*pcVar1)();
  return uVar2;
}

