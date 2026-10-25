
ulonglong FUN_140bfc750(longlong param_1,byte param_2)

{
  code *pcVar1;
  longlong lVar2;
  ulonglong uVar3;
  
  lVar2 = 0xad4;
  if (((param_2 & 1) != 0) && (lVar2 = 0x3c, (*(byte *)(param_1 + 0x20) & 1) == 0)) {
    FUN_1428d9310(&UNK_142b16c60);
    pcVar1 = (code *)swi(3);
    uVar3 = (*pcVar1)();
    return uVar3;
  }
  return (ulonglong)*(uint *)(param_1 + lVar2);
}

