
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

undefined8 * FUN_141468ee0(undefined8 *param_1,longlong param_2,undefined4 param_3)

{
  ulonglong uVar1;
  undefined8 uVar2;
  undefined8 uVar3;
  longlong lVar4;
  code *pcVar5;
  uint uVar6;
  ulonglong uVar7;
  undefined8 *puVar8;
  float fVar9;
  float fVar10;
  float fVar11;
  float fVar12;
  
  uVar6 = FUN_141467ce0(param_2,param_3);
  uVar7 = (ulonglong)uVar6;
  uVar1 = uVar7 + 1;
  if (uVar1 < *(ulonglong *)(param_2 + 0x10)) {
    lVar4 = *(longlong *)(param_2 + 8);
    fVar9 = *(float *)(lVar4 + 8 + uVar1 * 0xc) - *(float *)(lVar4 + 8 + uVar7 * 0xc);
    uVar2 = *(undefined8 *)(lVar4 + uVar1 * 0xc);
    uVar3 = *(undefined8 *)(lVar4 + uVar7 * 0xc);
    fVar10 = (float)uVar2 - (float)uVar3;
    fVar11 = (float)((ulonglong)uVar2 >> 0x20) - (float)((ulonglong)uVar3 >> 0x20);
    fVar12 = _DAT_142925904 / SQRT(fVar11 * fVar11 + fVar10 * fVar10 + fVar9 * fVar9);
    *param_1 = CONCAT44(fVar12 * fVar11,fVar12 * fVar10);
    *(float *)(param_1 + 1) = fVar12 * fVar9;
    return param_1;
  }
  FUN_1428d9518(uVar1,*(ulonglong *)(param_2 + 0x10),&UNK_142c63688);
  pcVar5 = (code *)swi(3);
  puVar8 = (undefined8 *)(*pcVar5)();
  return puVar8;
}

