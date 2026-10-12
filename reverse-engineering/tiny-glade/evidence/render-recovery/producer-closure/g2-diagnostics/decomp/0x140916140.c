
void FUN_140916140(longlong *param_1,undefined8 param_2)

{
  longlong lVar1;
  longlong *plStack_8;
  
  lVar1 = 0;
  if (*param_1 - 9U < 2) {
    lVar1 = *param_1 + -8;
  }
  if (lVar1 != 0) {
    if (lVar1 == 1) {
      plStack_8 = param_1 + 1;
      FUN_140d86ca0(param_2,&UNK_142aac908,6,&UNK_142aac90e,3,param_1 + 4,&UNK_142aac8c8,
                    &UNK_142aac911,5,&plStack_8,&UNK_142aac8e8);
      return;
    }
    plStack_8 = param_1 + 1;
    FUN_140d86bf0(param_2,&UNK_142aac916,0xe,&UNK_142aac924,4,&plStack_8,&UNK_142aac0a8);
    return;
  }
  plStack_8 = param_1 + 7;
  FUN_140d86d90(param_2,&UNK_142aac8a8,10,&UNK_142aac8b2,5,param_1,&UNK_142aac868,&UNK_142aac8b7,4,
                param_1 + 4,&UNK_142aac430,&UNK_142aac8bb,0xc,&plStack_8,&UNK_142aac888);
  return;
}

