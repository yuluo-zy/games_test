
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

ulonglong FUN_14146c3c0(longlong param_1,undefined8 param_2,undefined8 param_3)

{
  code *pcVar1;
  float fVar2;
  uint uVar3;
  ulonglong uVar4;
  ulonglong uVar5;
  float fVar6;
  float extraout_XMM0_Da;
  ulonglong uVar7;
  float fVar8;
  
  fVar6 = (float)FUN_141469a20(param_1,param_2,param_3,&UNK_142c63bc0);
  fVar2 = _DAT_142925904;
  fVar8 = 0.0;
  if (0.0 <= fVar6) {
    fVar8 = fVar6;
  }
  fVar6 = _DAT_142925904;
  if (fVar8 <= _DAT_142925904) {
    fVar6 = fVar8;
  }
  uVar3 = FUN_141467ce0(param_1,fVar6);
  uVar5 = (ulonglong)uVar3;
  uVar7 = *(ulonglong *)(param_1 + 0x10);
  if (uVar5 < uVar7) {
    uVar4 = uVar5 + 1;
    if (uVar4 < uVar7) {
      return (ulonglong)
             (uint)(*(float *)(*(longlong *)(param_1 + 8) + 8 + uVar5 * 8) * extraout_XMM0_Da +
                   *(float *)(*(longlong *)(param_1 + 8) + uVar5 * 8) * (fVar2 - extraout_XMM0_Da));
    }
  }
  else {
    uVar4 = FUN_1428d9518(uVar5,uVar7,&UNK_142c63658);
  }
  FUN_1428d9518(uVar4);
  pcVar1 = (code *)swi(3);
  uVar7 = (*pcVar1)();
  return uVar7;
}

