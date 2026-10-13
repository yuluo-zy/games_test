
longlong FUN_141208e10(longlong *param_1)

{
  ulonglong uVar1;
  code *pcVar2;
  ulonglong uVar3;
  longlong lVar4;
  int iStack_1c;
  undefined4 uStack_18;
  undefined4 uStack_14;
  
  func_0x0001412002f0(&iStack_1c,param_1 + 2);
  if (iStack_1c == 1) {
    lVar4 = *param_1;
    uVar1 = param_1[1];
    uVar3 = FUN_141200280(uStack_18,uStack_14);
    if (uVar1 <= uVar3) {
      FUN_1428d9518(uVar3,uVar1,&UNK_142c1a720);
      pcVar2 = (code *)swi(3);
      lVar4 = (*pcVar2)();
      return lVar4;
    }
    lVar4 = lVar4 + uVar3 * 0xc;
  }
  else {
    lVar4 = 0;
  }
  return lVar4;
}

