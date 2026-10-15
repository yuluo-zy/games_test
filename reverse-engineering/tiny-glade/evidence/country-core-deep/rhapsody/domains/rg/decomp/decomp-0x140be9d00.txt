
longlong FUN_140be9d00(longlong param_1,longlong param_2)

{
  longlong *plVar1;
  undefined8 uVar2;
  undefined8 uVar3;
  undefined8 uVar4;
  undefined8 uVar5;
  undefined8 uVar6;
  longlong lVar7;
  longlong *plVar8;
  longlong lVar9;
  longlong lStack_50;
  
  lVar7 = *(longlong *)(param_2 + 0x90);
  plVar1 = *(longlong **)(param_2 + 0x98);
  plVar8 = plVar1;
  if (plVar1 != plVar1 + *(longlong *)(param_2 + 0xa0) * 8) {
    plVar8 = plVar1 + 8;
    if (*plVar1 != 6) {
                    /* WARNING: Could not recover jumptable at 0x000140be9e12. Too many branches */
                    /* WARNING: Treating indirect jump as call */
      lVar7 = (*(code *)(&UNK_142b14a30 + *(int *)(&UNK_142b14a30 + *plVar1 * 4)))
                        (&UNK_142b14a30 + *(int *)(&UNK_142b14a30 + *plVar1 * 4));
      return lVar7;
    }
  }
  FUN_140be1bf0(plVar8,(ulonglong)
                       ((longlong)(plVar1 + *(longlong *)(param_2 + 0xa0) * 8) - (longlong)plVar8)
                       >> 6);
  if (lVar7 != 0) {
    func_0x000140613c20(plVar1,lVar7 << 6,8);
  }
  lVar7 = *(longlong *)(param_2 + 0x10);
  *(undefined8 *)(param_2 + 0x10) = 0;
  lStack_50 = 0;
  lVar9 = *(longlong *)(param_2 + 8);
  while (lVar7 != lStack_50) {
    lStack_50 = lStack_50 + 1;
    func_0x000140bdfc90(lVar9);
    lVar9 = lVar9 + 0x48;
  }
  *(undefined8 *)(param_2 + 0x28) = 0;
  *(undefined8 *)(param_2 + 0x40) = 0;
  func_0x000142923c00(param_1,param_2,0x90);
  uVar2 = *(undefined8 *)(param_2 + 0xb0);
  uVar3 = *(undefined8 *)(param_2 + 0xb8);
  uVar4 = *(undefined8 *)(param_2 + 0xc0);
  uVar5 = *(undefined8 *)(param_2 + 200);
  uVar6 = *(undefined8 *)(param_2 + 0xd0);
  *(undefined8 *)(param_1 + 0x90) = *(undefined8 *)(param_2 + 0xa8);
  *(undefined8 *)(param_1 + 0x98) = uVar2;
  *(undefined8 *)(param_1 + 0xa0) = uVar3;
  *(undefined8 *)(param_1 + 0xa8) = uVar4;
  *(undefined8 *)(param_1 + 0xb0) = uVar5;
  *(undefined8 *)(param_1 + 0xb8) = uVar6;
  return param_1;
}

