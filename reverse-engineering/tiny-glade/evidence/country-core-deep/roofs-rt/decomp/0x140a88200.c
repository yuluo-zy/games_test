
ulonglong FUN_140a88200(byte *param_1)

{
  code *pcVar1;
  ulonglong uVar2;
  
  if ((*param_1 & 1) == 0) {
    return (ulonglong)*(uint *)(param_1 + 4);
  }
  FUN_1428d9430(&UNK_142ad8d90,0x28,&UNK_142ad8df8);
  pcVar1 = (code *)swi(3);
  uVar2 = (*pcVar1)();
  return uVar2;
}

