
ulonglong FUN_140be6850(int *param_1,undefined *param_2)

{
  code *pcVar1;
  int iVar2;
  ulonglong uVar3;
  int *piStack_60;
  undefined *puStack_58;
  int **ppiStack_50;
  undefined *puStack_48;
  undefined **ppuStack_40;
  code *pcStack_38;
  undefined *puStack_30;
  undefined8 uStack_28;
  int ***pppiStack_20;
  undefined8 uStack_18;
  undefined8 uStack_10;
  
  iVar2 = *param_1;
  piStack_60 = param_1;
  puStack_58 = param_2;
  if (iVar2 != 2) {
    if (iVar2 != 3) {
      if (iVar2 != 4) {
        ppiStack_50 = &piStack_60;
        puStack_48 = &UNK_1418c2cb0;
        ppuStack_40 = &puStack_58;
        pcStack_38 = FUN_140d8d6c0;
        puStack_30 = &UNK_142b13968;
        uStack_28 = 3;
        uStack_10 = 0;
        pppiStack_20 = &ppiStack_50;
        uStack_18 = 2;
        FUN_1428d9390(&puStack_30,&UNK_142b139e0);
        goto code_r0x000140be6a16;
      }
      uVar3 = (longlong)param_2 << 0x3e | (ulonglong)(param_2 + -4) >> 2;
      iVar2 = 100;
      if (uVar3 < 4) goto code_r0x000140be699b;
      ppiStack_50 = &piStack_60;
      puStack_48 = &UNK_1418c2cb0;
      ppuStack_40 = &puStack_58;
      pcStack_38 = FUN_140d8d6c0;
      puStack_30 = &UNK_142b13968;
      uStack_28 = 3;
      uStack_10 = 0;
      pppiStack_20 = &ppiStack_50;
      uStack_18 = 2;
      param_2 = &UNK_142b139c8;
      FUN_1428d9390(&puStack_30);
    }
    uVar3 = (longlong)param_2 << 0x3e | (ulonglong)(param_2 + -4) >> 2;
    iVar2 = 0x62;
    if (uVar3 < 4) goto code_r0x000140be699b;
    ppiStack_50 = &piStack_60;
    puStack_48 = &UNK_1418c2cb0;
    ppuStack_40 = &puStack_58;
    pcStack_38 = FUN_140d8d6c0;
    puStack_30 = &UNK_142b13968;
    uStack_28 = 3;
    uStack_10 = 0;
    pppiStack_20 = &ppiStack_50;
    uStack_18 = 2;
    param_2 = &UNK_142b139b0;
    FUN_1428d9390(&puStack_30);
  }
  uVar3 = (longlong)param_2 << 0x3e | (ulonglong)(param_2 + -4) >> 2;
  iVar2 = 99;
  if (3 < uVar3) {
code_r0x000140be6a16:
    ppiStack_50 = &piStack_60;
    puStack_48 = &UNK_1418c2cb0;
    ppuStack_40 = &puStack_58;
    pcStack_38 = FUN_140d8d6c0;
    puStack_30 = &UNK_142b13968;
    uStack_28 = 3;
    uStack_10 = 0;
    pppiStack_20 = &ppiStack_50;
    uStack_18 = 2;
    FUN_1428d9390(&puStack_30,&UNK_142b13998);
    pcVar1 = (code *)swi(3);
    uVar3 = (*pcVar1)();
    return uVar3;
  }
code_r0x000140be699b:
  return (ulonglong)(uint)((int)uVar3 * 3 + iVar2);
}

