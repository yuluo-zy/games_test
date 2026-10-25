
longlong FUN_140bd4cd0(longlong param_1,longlong param_2)

{
  code *pcVar1;
  longlong lVar2;
  ulonglong uVar3;
  longlong lVar4;
  ulonglong uVar5;
  undefined8 auStack_48 [6];
  
  if (*(short *)(param_1 + 0x14) == *(short *)(*(longlong *)(param_2 + 0x648) + 0xa50)) {
    if (*(char *)(param_1 + 0x16) != '\x01') goto code_r0x000140bd4d70;
    lVar4 = *(longlong *)(param_1 + 8);
    if (*(int *)(lVar4 + 0xc) == *(int *)(param_1 + 0x10)) {
      if (*(char *)(lVar4 + 0x7c) == '\x02') goto code_r0x000140bd4dc9;
      lVar2 = func_0x000140c113b0(lVar4 + 0x38);
      if (lVar2 == 0) {
        return 0;
      }
      uVar3 = func_0x000140c02280(lVar4 + 0x38);
      uVar5 = *(ulonglong *)(param_1 + 0x18);
      if (uVar5 <= uVar3) {
        return lVar2;
      }
      goto code_r0x000140bd4d9f;
    }
  }
  else {
    auStack_48[0] = 0;
    param_1 = param_1 + 0x14;
    FUN_1428ca816(0,param_1,*(longlong *)(param_2 + 0x648) + 0xa50,auStack_48,&UNK_142d022a0);
code_r0x000140bd4d70:
    auStack_48[0] = 0;
    uVar5 = param_1 + 0x16;
    uVar3 = FUN_1428ca902(0,uVar5,&UNK_142d022d9,auStack_48,&UNK_142d022c0);
code_r0x000140bd4d9f:
    FUN_1428d9840(uVar5,uVar3,&UNK_142b0f818);
  }
  FUN_1428d9330(&UNK_142b0f7f0,0xe,&UNK_142b0f800);
code_r0x000140bd4dc9:
  FUN_1428d9310(&UNK_142d02388);
  pcVar1 = (code *)swi(3);
  lVar4 = (*pcVar1)();
  return lVar4;
}

