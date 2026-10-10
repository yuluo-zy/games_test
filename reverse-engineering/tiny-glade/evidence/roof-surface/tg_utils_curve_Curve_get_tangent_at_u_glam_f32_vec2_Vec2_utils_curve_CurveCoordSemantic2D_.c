
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

undefined8 FUN_141468f90(longlong param_1)

{
  undefined8 uVar1;
  code *pcVar2;
  uint uVar3;
  ulonglong uVar4;
  float fVar5;
  float fVar7;
  undefined8 uVar6;
  
  uVar3 = FUN_141467ce0();
  uVar4 = (ulonglong)uVar3;
  if (uVar4 + 1 < *(ulonglong *)(param_1 + 0x10)) {
    uVar6 = *(undefined8 *)(*(longlong *)(param_1 + 8) + uVar4 * 8);
    uVar1 = *(undefined8 *)(*(longlong *)(param_1 + 8) + 8 + uVar4 * 8);
    fVar5 = (float)uVar1 - (float)uVar6;
    fVar7 = (float)((ulonglong)uVar1 >> 0x20) - (float)((ulonglong)uVar6 >> 0x20);
    return CONCAT44(fVar7,fVar5 * (_DAT_142925904 / SQRT(fVar7 * fVar7 + fVar5 * fVar5)));
  }
  FUN_1428d9518(uVar4 + 1,*(ulonglong *)(param_1 + 0x10),&UNK_142c63688);
  pcVar2 = (code *)swi(3);
  uVar6 = (*pcVar2)();
  return uVar6;
}

