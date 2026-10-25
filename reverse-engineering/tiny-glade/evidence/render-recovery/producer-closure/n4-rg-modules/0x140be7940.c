
longlong FUN_140be7940(longlong param_1,ulonglong param_2)

{
  ushort uVar1;
  ulonglong uVar2;
  code *pcVar3;
  longlong lVar4;
  
  if ((param_2 & 0xffffffff) < *(ulonglong *)(param_1 + 0x10)) {
    uVar1 = *(ushort *)(*(longlong *)(param_1 + 8) + (param_2 & 0xffffffff) * 4);
    if ((ulonglong)uVar1 < *(ulonglong *)(param_1 + 0x40)) {
      lVar4 = (ulonglong)uVar1 * 0x40;
      uVar2 = *(ulonglong *)(*(longlong *)(param_1 + 0x38) + lVar4);
      if (uVar2 < 2) {
        return *(longlong *)(param_1 + 0x38) + lVar4 + 8;
      }
      if (2 < uVar2 - 2) goto code_r0x000140be79a8;
      FUN_1428caa40();
      goto code_r0x000140be7990;
    }
  }
  else {
code_r0x000140be7990:
    FUN_1428d9518();
  }
  FUN_1428d9518();
code_r0x000140be79a8:
  FUN_1428d9430(&UNK_142b101c8,0x28,&UNK_142b13cd8);
  pcVar3 = (code *)swi(3);
  lVar4 = (*pcVar3)();
  return lVar4;
}

