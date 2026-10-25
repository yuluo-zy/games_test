
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

ulonglong FUN_141467270(longlong param_1)

{
  code *pcVar1;
  uint uVar2;
  ulonglong uVar3;
  ulonglong uVar4;
  float extraout_XMM0_Da;
  ulonglong uVar5;
  
  uVar2 = FUN_141467ce0();
  uVar4 = (ulonglong)uVar2;
  uVar5 = *(ulonglong *)(param_1 + 0x10);
  if (uVar4 < uVar5) {
    uVar3 = uVar4 + 1;
    if (uVar3 < uVar5) {
      return (ulonglong)
             (uint)(*(float *)(*(longlong *)(param_1 + 8) + 8 + uVar4 * 8) * extraout_XMM0_Da +
                   *(float *)(*(longlong *)(param_1 + 8) + uVar4 * 8) *
                   (_DAT_142925904 - extraout_XMM0_Da));
    }
  }
  else {
    uVar3 = FUN_1428d9518(uVar4,uVar5,&UNK_142c63658);
  }
  FUN_1428d9518(uVar3);
  pcVar1 = (code *)swi(3);
  uVar5 = (*pcVar1)();
  return uVar5;
}

