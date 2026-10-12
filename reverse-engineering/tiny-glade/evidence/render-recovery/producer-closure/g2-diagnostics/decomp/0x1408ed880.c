
void FUN_1408ed880(longlong *param_1,undefined8 param_2)

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
      FUN_140d86ca0(param_2,&UNK_142aa6ce8,6,&UNK_142aa6cee,3,param_1 + 4,&UNK_142aa6ca8,
                    &UNK_142aa6cf1,5,&plStack_8,&UNK_142aa6cc8);
      return;
    }
    plStack_8 = param_1 + 1;
    FUN_140d86bf0(param_2,&UNK_142aa6d18,0xe,&UNK_142aa6d26,4,&plStack_8,&UNK_142aa6cf8);
    return;
  }
  plStack_8 = param_1 + 7;
  FUN_140d86d90(param_2,&UNK_142aa6c88,10,&UNK_142aa6c92,5,param_1,&UNK_142aa6c48,&UNK_142aa6c97,4,
                param_1 + 4,&UNK_142aa6a80,&UNK_142aa6c9b,0xc,&plStack_8,&UNK_142aa6c68);
  return;
}

