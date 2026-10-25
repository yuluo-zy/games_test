
undefined8
FUN_140be7a60(undefined8 param_1,longlong param_2,longlong param_3,ulonglong param_4,
             undefined8 param_5)

{
  ushort uVar1;
  code *pcVar2;
  ulonglong uVar3;
  undefined8 uVar4;
  longlong lVar5;
  undefined8 auStack_38 [6];
  
  if ((param_4 & 0xffffffff) < *(ulonglong *)(param_2 + 0x10)) {
    uVar1 = *(ushort *)(*(longlong *)(param_2 + 8) + (param_4 & 0xffffffff) * 4);
    if (*(ulonglong *)(param_2 + 0x40) <= (ulonglong)uVar1) goto code_r0x000140be7b6c;
    lVar5 = (ulonglong)uVar1 * 0x40;
    uVar3 = *(ulonglong *)(*(longlong *)(param_2 + 0x38) + lVar5);
    if (1 < uVar3) {
code_r0x000140be7b4e:
      if (2 < uVar3 - 2) goto code_r0x000140be7b87;
      FUN_1428caa80();
      goto code_r0x000140be7b5d;
    }
    lVar5 = *(longlong *)(param_2 + 0x38) + lVar5;
    if (*(short *)(lVar5 + 0x1c) != *(short *)(*(longlong *)(param_3 + 0x648) + 0xa50)) {
      auStack_38[0] = 0;
      lVar5 = lVar5 + 0x1c;
      FUN_1428ca816(0,lVar5,*(longlong *)(param_3 + 0x648) + 0xa50,auStack_38,&UNK_142d022a0);
code_r0x000140be7b22:
      auStack_38[0] = 0;
      uVar3 = FUN_1428ca902(0,lVar5 + 0x1e,&UNK_142d022b8,auStack_38,&UNK_142d022c0);
      goto code_r0x000140be7b4e;
    }
    if (*(char *)(lVar5 + 0x1e) != '\0') goto code_r0x000140be7b22;
    if (*(int *)(*(longlong *)(lVar5 + 0x10) + 0xc) == *(int *)(lVar5 + 0x18)) {
      FUN_140bdc5d0(param_1,*(longlong *)(lVar5 + 0x10) + 0x10,param_3 + 0x78,param_5);
      return param_1;
    }
  }
  else {
code_r0x000140be7b5d:
    FUN_1428d9518();
code_r0x000140be7b6c:
    FUN_1428d9518();
  }
  FUN_1428d9310(&UNK_142b13db0);
code_r0x000140be7b87:
  FUN_1428d9430(&UNK_142b101c8,0x28,&UNK_142b13d98);
  pcVar2 = (code *)swi(3);
  uVar4 = (*pcVar2)();
  return uVar4;
}

