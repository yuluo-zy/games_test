
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void FUN_14126a110(longlong param_1,undefined8 param_2,undefined8 param_3,undefined8 param_4)

{
  undefined8 uVar1;
  code *pcVar2;
  uint uVar3;
  ulonglong uVar4;
  ulonglong uVar5;
  float extraout_XMM0_Da;
  float fVar6;
  
  if (*(ulonglong *)(param_1 + 0x10) < 2) {
    FUN_1428d9430(&UNK_142c22708,0x29,param_4);
LAB_14126a197:
    FUN_1428d9430(&UNK_142c236e0,0x1d,param_4);
LAB_14126a1ab:
    FUN_1428d9430(&UNK_142c23790,0x22,param_4);
  }
  else {
    uVar3 = FUN_14126a450();
    uVar5 = (ulonglong)uVar3;
    if (uVar5 < *(ulonglong *)(param_1 + 0x28)) {
      uVar4 = uVar5 + 1;
      if (*(ulonglong *)(param_1 + 0x28) <= uVar4) goto LAB_14126a1c7;
      uVar1 = *(undefined8 *)(*(longlong *)(param_1 + 0x20) + uVar5 * 4);
      fVar6 = extraout_XMM0_Da * (float)((ulonglong)uVar1 >> 0x20) +
              (_DAT_142925904 - extraout_XMM0_Da) * (float)uVar1;
      if (NAN(fVar6)) goto LAB_14126a197;
      if (ABS(fVar6) != INFINITY) {
        return;
      }
      goto LAB_14126a1ab;
    }
  }
  uVar4 = FUN_1428d9518();
LAB_14126a1c7:
  FUN_1428d9518(uVar4);
  pcVar2 = (code *)swi(3);
  (*pcVar2)();
  return;
}

