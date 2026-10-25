
ulonglong FUN_140bd4b80(ulonglong param_1)

{
  code *pcVar1;
  ulonglong uVar2;
  undefined1 uStack_1;
  
  if (param_1 >> 0x20 == 0) {
    return param_1 & 0xffffffff;
  }
  FUN_1428d9760(&UNK_142b0f5a8,0x2b,&uStack_1,&UNK_142b0f588,&UNK_142b0f750);
  pcVar1 = (code *)swi(3);
  uVar2 = (*pcVar1)();
  return uVar2;
}

