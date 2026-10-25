
longlong FUN_14090dda0(longlong *param_1,undefined8 param_2,undefined8 param_3)

{
  uint uVar1;
  code *pcVar2;
  longlong lVar3;
  longlong *plVar4;
  ulonglong uVar5;
  longlong lVar6;
  undefined8 auStack_c8 [2];
  undefined8 uStack_b8;
  undefined8 uStack_b0;
  undefined4 uStack_38;
  longlong *plStack_30;
  longlong lStack_28;
  undefined8 uStack_20;
  
  uStack_20 = 0xfffffffffffffffe;
  lVar3 = *param_1;
  uVar1 = *(uint *)(lVar3 + 0x90);
  if ((ulonglong)uVar1 < 8) {
    lVar6 = (ulonglong)uVar1 * 0x10;
    *(undefined8 *)(lVar3 + 0x10 + lVar6) = param_2;
    *(undefined8 *)(lVar3 + 0x18 + lVar6) = param_3;
    *(uint *)(lVar3 + 0x90) = uVar1 + 1;
    uVar5 = (ulonglong)*(uint *)(*param_1 + 0x90);
    if (uVar5 == 0) {
      FUN_1428d9518(0xffffffffffffffff,0,&UNK_142aab660);
code_r0x00014090deae:
      FUN_1428d9000(8,0x98);
                    /* WARNING: Does not return */
      pcVar2 = (code *)invalidInstructionException();
      (*pcVar2)();
    }
    lVar3 = *param_1 + uVar5 * 0x10;
  }
  else {
    uStack_38 = 1;
    auStack_c8[0] = 0;
    uStack_b8 = param_2;
    uStack_b0 = param_3;
    plVar4 = (longlong *)func_0x000140613c10(0x98,8);
    if (plVar4 == (longlong *)0x0) goto code_r0x00014090deae;
    func_0x000142923c00(plVar4,auStack_c8,0x98);
    lVar3 = *param_1;
    *param_1 = (longlong)plVar4;
    if (*plVar4 != 0) {
      plStack_30 = param_1;
      lStack_28 = lVar3;
      FUN_14087add0(plVar4 + 1);
      plVar4 = (longlong *)*plStack_30;
      lVar3 = lStack_28;
      param_1 = plStack_30;
    }
    *plVar4 = 1;
    plVar4[1] = lVar3;
    if (*(int *)(*param_1 + 0x90) == 0) {
      FUN_1428d9518(0,0,&UNK_142aab648);
      pcVar2 = (code *)swi(3);
      lVar3 = (*pcVar2)();
      return lVar3;
    }
    lVar3 = *param_1 + 0x10;
  }
  return lVar3;
}

