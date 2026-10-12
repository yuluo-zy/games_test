
undefined8 FUN_140bff400(longlong param_1,longlong param_2)

{
  code *pcVar1;
  undefined8 uVar2;
  undefined8 auStack_30 [6];
  
  if (*(short *)(param_2 + 0x14) == *(short *)(*(longlong *)(param_1 + 0x648) + 0xa50)) {
    if (*(char *)(param_2 + 0x16) == '\x06') {
      if (*(int *)(*(longlong *)(param_2 + 8) + 0xc) == *(int *)(param_2 + 0x10)) {
        return 1;
      }
      return 0;
    }
  }
  else {
    auStack_30[0] = 0;
    param_2 = param_2 + 0x14;
    FUN_1428ca816(0,param_2,*(longlong *)(param_1 + 0x648) + 0xa50,auStack_30,&UNK_142d022a0);
  }
  auStack_30[0] = 0;
  FUN_1428ca902(0,param_2 + 0x16,&UNK_142d022dc,auStack_30,&UNK_142d022c0);
  pcVar1 = (code *)swi(3);
  uVar2 = (*pcVar1)();
  return uVar2;
}

