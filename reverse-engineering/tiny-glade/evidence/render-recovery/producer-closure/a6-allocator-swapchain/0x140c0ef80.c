
void FUN_140c0ef80(longlong param_1,longlong param_2)

{
  code *pcVar1;
  byte bVar2;
  undefined8 auStack_38 [6];
  
  if (*(short *)(param_1 + 0x14) == *(short *)(*(longlong *)(param_2 + 0x648) + 0xa50)) {
    if (*(char *)(param_1 + 0x16) == '\x02') {
      if (*(int *)(*(longlong *)(param_1 + 8) + 0xc) == *(int *)(param_1 + 0x10)) {
        bVar2 = FUN_1418cc1b0(*(longlong *)(param_1 + 8) + 0x10);
        *(byte *)(param_1 + 0x1c) = *(byte *)(param_1 + 0x1c) | bVar2;
        return;
      }
      goto code_r0x000140c0f028;
    }
  }
  else {
    auStack_38[0] = 0;
    param_1 = param_1 + 0x14;
    FUN_1428ca816(0,param_1,*(longlong *)(param_2 + 0x648) + 0xa50,auStack_38,&UNK_142d022a0);
  }
  auStack_38[0] = 0;
  FUN_1428ca902(0,param_1 + 0x16,&UNK_142d022d8,auStack_38,&UNK_142d022c0);
code_r0x000140c0f028:
  FUN_1428d9310(&UNK_142b19c88);
  pcVar1 = (code *)swi(3);
  (*pcVar1)();
  return;
}

