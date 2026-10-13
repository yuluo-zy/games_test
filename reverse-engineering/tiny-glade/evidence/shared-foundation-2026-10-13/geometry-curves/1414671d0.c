
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

undefined8 * FUN_1414671d0(undefined8 *param_1,longlong param_2,undefined8 param_3)

{
  float fVar1;
  float fVar2;
  undefined8 uVar3;
  undefined8 uVar4;
  ulonglong uVar5;
  longlong lVar6;
  code *pcVar7;
  uint uVar8;
  ulonglong uVar9;
  undefined8 *puVar10;
  ulonglong uVar11;
  float extraout_XMM0_Da;
  float fVar12;
  
  uVar8 = FUN_141467ce0(param_2,param_3);
  uVar11 = (ulonglong)uVar8;
  uVar5 = *(ulonglong *)(param_2 + 0x10);
  if (uVar11 < uVar5) {
    uVar9 = uVar11 + 1;
    if (uVar9 < uVar5) {
      lVar6 = *(longlong *)(param_2 + 8);
      fVar12 = _DAT_142925904 - extraout_XMM0_Da;
      fVar1 = *(float *)(lVar6 + 8 + uVar11 * 0xc);
      fVar2 = *(float *)(lVar6 + 8 + uVar9 * 0xc);
      uVar3 = *(undefined8 *)(lVar6 + uVar11 * 0xc);
      uVar4 = *(undefined8 *)(lVar6 + uVar9 * 0xc);
      *param_1 = CONCAT44((float)((ulonglong)uVar4 >> 0x20) * extraout_XMM0_Da +
                          (float)((ulonglong)uVar3 >> 0x20) * fVar12,
                          (float)uVar4 * extraout_XMM0_Da + (float)uVar3 * fVar12);
      *(float *)(param_1 + 1) = fVar2 * extraout_XMM0_Da + fVar12 * fVar1;
      return param_1;
    }
  }
  else {
    uVar9 = FUN_1428d9518(uVar11,uVar5,&UNK_142c63658);
  }
  FUN_1428d9518(uVar9);
  pcVar7 = (code *)swi(3);
  puVar10 = (undefined8 *)(*pcVar7)();
  return puVar10;
}

