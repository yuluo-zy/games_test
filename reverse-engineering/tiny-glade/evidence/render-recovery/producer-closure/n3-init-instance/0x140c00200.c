
void FUN_140c00200(int *param_1,longlong param_2)

{
  undefined8 uVar1;
  undefined8 uVar2;
  int *piStack_c0;
  code *pcStack_b8;
  int **ppiStack_b0;
  code *pcStack_a8;
  undefined **ppuStack_a0;
  code *pcStack_98;
  undefined *puStack_90;
  undefined8 uStack_88;
  int **ppiStack_80;
  undefined8 uStack_78;
  undefined8 uStack_70;
  undefined8 uStack_68;
  undefined1 uStack_60;
  undefined8 uStack_58;
  undefined8 uStack_48;
  undefined8 uStack_40;
  undefined8 uStack_38;
  undefined8 uStack_30;
  undefined1 uStack_28;
  int *piStack_20;
  code *pcStack_18;
  int *piStack_10;
  code *pcStack_8;
  
  if (*param_1 - 2U < 5) {
    ppuStack_a0 = (undefined **)(param_1 + 3);
    pcStack_98 = FUN_140d8d560;
    uVar1 = *(undefined8 *)(param_2 + 0x30);
    uVar2 = *(undefined8 *)(param_2 + 0x38);
    puStack_90 = &UNK_142b179c8;
    uStack_88 = 3;
    uStack_78 = 3;
  }
  else {
    if (*param_1 == 0) {
      piStack_20 = param_1 + 1;
      piStack_10 = param_1 + 2;
      pcStack_18 = FUN_140d8d560;
      pcStack_8 = FUN_140d8d560;
      puStack_90 = (undefined *)0x2;
      ppiStack_80 = (int **)0x2;
      uStack_78 = 2;
      uStack_70 = 0;
      uStack_68 = 0x20;
      uStack_60 = 3;
      uStack_58 = 2;
      uStack_48 = 0;
      uStack_40 = 2;
      uStack_38 = 1;
      uStack_30 = 0x800000020;
      uStack_28 = 3;
      piStack_c0 = (int *)&UNK_142b179a8;
      pcStack_b8 = (code *)0x2;
      ppiStack_b0 = &piStack_20;
      pcStack_a8 = (code *)0x2;
      ppuStack_a0 = &puStack_90;
      pcStack_98 = (code *)0x2;
      FUN_140d85ad0(*(undefined8 *)(param_2 + 0x30),*(undefined8 *)(param_2 + 0x38),&piStack_c0);
      return;
    }
    uVar1 = *(undefined8 *)(param_2 + 0x30);
    uVar2 = *(undefined8 *)(param_2 + 0x38);
    puStack_90 = &UNK_142b179a8;
    uStack_88 = 2;
    uStack_78 = 2;
  }
  ppiStack_80 = &piStack_c0;
  ppiStack_b0 = (int **)(param_1 + 2);
  piStack_c0 = param_1 + 1;
  pcStack_a8 = FUN_140d8d560;
  pcStack_b8 = FUN_140d8d560;
  uStack_70 = 0;
  FUN_140d85ad0(uVar1,uVar2,&puStack_90);
  return;
}

