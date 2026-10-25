
void FUN_1408fca00(longlong *param_1,undefined8 param_2)

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
      FUN_140d86ca0(param_2,&UNK_142aa9480,6,&UNK_142aa9486,3,param_1 + 4,&UNK_142aa9440,
                    &UNK_142aa9489,5,&plStack_8,&UNK_142aa9460);
      return;
    }
    plStack_8 = param_1 + 1;
    FUN_140d86bf0(param_2,&UNK_142aa94b0,0xe,&UNK_142aa94be,4,&plStack_8,&UNK_142aa9490);
    return;
  }
  plStack_8 = param_1 + 7;
  FUN_140d86d90(param_2,&UNK_142aa9420,10,&UNK_142aa942a,5,param_1,&UNK_142aa93c0,&UNK_142aa942f,4,
                param_1 + 4,&UNK_142aa93e0,&UNK_142aa9433,0xc,&plStack_8,&UNK_142aa9400);
  return;
}

