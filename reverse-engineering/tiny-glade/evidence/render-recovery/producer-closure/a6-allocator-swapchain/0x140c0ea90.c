
undefined8 FUN_140c0ea90(longlong param_1,longlong param_2)

{
  code *pcVar1;
  undefined8 uVar2;
  longlong lVar3;
  undefined8 auStack_30 [3];
  undefined8 uStack_18;
  
  if (*(short *)(param_1 + 0x14) == *(short *)(*(longlong *)(param_2 + 0x648) + 0xa50)) {
    if (*(char *)(param_1 + 0x16) == '\x02') {
      if (*(int *)(*(longlong *)(param_1 + 8) + 0xc) == *(int *)(param_1 + 0x10)) {
        FUN_1418cb100(auStack_30,*(longlong *)(param_1 + 8) + 0x10);
        return uStack_18;
      }
      goto code_r0x000140c0eb31;
    }
  }
  else {
    auStack_30[0] = 0;
    lVar3 = param_1 + 0x14;
    param_1 = 0;
    FUN_1428ca816(0,lVar3,*(longlong *)(param_2 + 0x648) + 0xa50,auStack_30,&UNK_142d022a0);
  }
  auStack_30[0] = 0;
  FUN_1428ca902(0,param_1 + 0x16,&UNK_142d022d8,auStack_30,&UNK_142d022c0);
code_r0x000140c0eb31:
  FUN_1428d9310(&UNK_142b19c40);
  pcVar1 = (code *)swi(3);
  uVar2 = (*pcVar1)();
  return uVar2;
}

