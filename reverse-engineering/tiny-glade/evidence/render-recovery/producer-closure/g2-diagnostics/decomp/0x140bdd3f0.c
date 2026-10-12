
void FUN_140bdd3f0(longlong *param_1,longlong param_2)

{
  longlong lVar1;
  undefined8 uVar2;
  undefined8 uVar3;
  longlong *plStack_78;
  longlong *plStack_70;
  undefined *puStack_68;
  undefined8 uStack_60;
  undefined8 **ppuStack_58;
  undefined8 uStack_50;
  undefined8 uStack_48;
  longlong **pplStack_38;
  undefined *puStack_30;
  longlong **pplStack_28;
  code *pcStack_20;
  longlong **pplStack_18;
  code *pcStack_10;
  longlong *plStack_8;
  
  lVar1 = 0;
  if (*param_1 - 9U < 2) {
    lVar1 = *param_1 + -8;
  }
  if (lVar1 == 0) {
    plStack_70 = param_1 + 4;
    plStack_78 = param_1 + 7;
    pplStack_38 = &plStack_70;
    puStack_30 = &UNK_1400538e0;
    pplStack_28 = &plStack_78;
    pcStack_20 = FUN_1418bdf80;
    pplStack_18 = &plStack_8;
    pcStack_10 = FUN_1418be1c0;
    uVar2 = *(undefined8 *)(param_2 + 0x30);
    uVar3 = *(undefined8 *)(param_2 + 0x38);
    puStack_68 = &UNK_142b12998;
    uStack_60 = 3;
    uStack_50 = 3;
    plStack_8 = param_1;
  }
  else if (lVar1 == 1) {
    plStack_70 = param_1 + 4;
    plStack_78 = param_1 + 1;
    pplStack_38 = &plStack_70;
    puStack_30 = &UNK_14090dd80;
    pplStack_28 = &plStack_78;
    pcStack_20 = (code *)&UNK_1402e0960;
    uVar2 = *(undefined8 *)(param_2 + 0x30);
    uVar3 = *(undefined8 *)(param_2 + 0x38);
    puStack_68 = &UNK_142b129d8;
    uStack_60 = 2;
    uStack_50 = 2;
  }
  else {
    plStack_78 = param_1 + 1;
    pplStack_38 = &plStack_78;
    puStack_30 = &UNK_1400538e0;
    uVar2 = *(undefined8 *)(param_2 + 0x30);
    uVar3 = *(undefined8 *)(param_2 + 0x38);
    puStack_68 = &UNK_142b12a18;
    uStack_60 = 1;
    uStack_50 = 1;
  }
  ppuStack_58 = &pplStack_38;
  uStack_48 = 0;
  FUN_140d85ad0(uVar2,uVar3,&puStack_68);
  return;
}

