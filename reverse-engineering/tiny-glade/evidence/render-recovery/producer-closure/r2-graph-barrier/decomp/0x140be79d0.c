
longlong FUN_140be79d0(longlong param_1,ulonglong param_2)

{
  ushort uVar1;
  code *pcVar2;
  longlong lVar3;
  longlong lVar4;
  uint uVar5;
  
  if ((param_2 & 0xffffffff) < *(ulonglong *)(param_1 + 0x10)) {
    uVar1 = *(ushort *)(*(longlong *)(param_1 + 8) + (param_2 & 0xffffffff) * 4);
    if ((ulonglong)uVar1 < *(ulonglong *)(param_1 + 0x40)) {
      lVar4 = (ulonglong)uVar1 * 0x40;
      lVar3 = *(longlong *)(*(longlong *)(param_1 + 0x38) + lVar4);
      if (lVar3 - 2U < 2) {
        return *(longlong *)(param_1 + 0x38) + lVar4 + 8;
      }
      uVar5 = (uint)lVar3;
      if ((1 < uVar5) && (uVar5 != 4)) goto code_r0x000140be7a3c;
      FUN_1428caa60();
      goto code_r0x000140be7a24;
    }
  }
  else {
code_r0x000140be7a24:
    FUN_1428d9518();
  }
  FUN_1428d9518();
code_r0x000140be7a3c:
  FUN_1428d9430(&UNK_142b101c8,0x28,&UNK_142b13d38);
  pcVar2 = (code *)swi(3);
  lVar3 = (*pcVar2)();
  return lVar3;
}

