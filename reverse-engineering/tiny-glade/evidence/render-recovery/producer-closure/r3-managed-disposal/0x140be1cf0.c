
longlong * FUN_140be1cf0(longlong *param_1)

{
  undefined1 auVar1 [16];
  ulonglong uVar2;
  longlong *plVar3;
  ulonglong uVar4;
  longlong lVar5;
  longlong lVar6;
  longlong *plVar7;
  longlong lVar8;
  longlong lVar9;
  longlong *plStack_f0;
  longlong *plStack_e8;
  longlong *plStack_c8;
  longlong *plStack_c0;
  longlong **pplStack_b8;
  longlong lStack_90;
  longlong lStack_88;
  longlong lStack_80;
  longlong lStack_78;
  longlong lStack_70;
  ulonglong uStack_68;
  longlong lStack_60;
  ulonglong uStack_58;
  longlong *plStack_50;
  undefined8 uStack_48;
  
  uStack_48 = 0xfffffffffffffffe;
  lStack_60 = *param_1;
  lVar6 = param_1[1];
  uVar2 = lVar6 - lStack_60;
  auVar1._8_8_ = 0;
  auVar1._0_8_ = uVar2;
  *param_1 = 8;
  param_1[1] = 8;
  lVar5 = param_1[2];
  if (lVar6 == lStack_60) {
    lVar6 = param_1[4];
    if (lVar6 == 0) {
      return SUB168(auVar1 * ZEXT816(0xe38e38e38e38e38f),0);
    }
    plVar7 = *(longlong **)(lVar5 + 0x10);
    plVar3 = (longlong *)param_1[3];
    if (plVar3 == plVar7) goto code_r0x000140be1e6e;
  }
  else {
    uVar2 = uVar2 / 0x48;
    lVar8 = 0x40;
    uVar4 = 1;
    lVar9 = 0x38;
    lVar6 = 0x38;
    lStack_70 = lVar5;
    uStack_58 = uVar2;
    plStack_50 = param_1;
    do {
      lStack_80 = lVar9 + lStack_60;
      lVar5 = *(longlong *)(lVar9 + lStack_60);
      lStack_78 = lVar8 + lStack_60;
      lStack_90 = lStack_60 + lVar6;
      plStack_f0 = &lStack_78;
      plStack_e8 = &lStack_80;
      plStack_c8 = &lStack_88;
      plStack_c0 = &lStack_90;
      pplStack_b8 = &plStack_f0;
      lStack_88 = lVar5;
      uStack_68 = uVar4;
      FUN_1418bc7d0(&plStack_c8);
      if (lVar5 != 0) {
        plStack_c8 = (longlong *)(lVar5 + -0x10);
        LOCK();
        plVar7 = (longlong *)(lVar5 + -0x10);
        *plVar7 = *plVar7 + -1;
        UNLOCK();
        if (*plVar7 == 0) {
          FUN_140c15f40(&plStack_c8);
        }
      }
      uVar4 = uStack_68 + 1;
      lVar8 = lVar8 + 0x48;
      lVar6 = lVar6 + 0x48;
      lVar9 = lVar9 + 0x48;
    } while (uStack_68 != uVar2);
    lVar6 = plStack_50[4];
    if (lVar6 == 0) {
      return plStack_50;
    }
    plVar7 = *(longlong **)(lStack_70 + 0x10);
    plVar3 = (longlong *)plStack_50[3];
    lVar5 = lStack_70;
    if (plVar3 == plVar7) goto code_r0x000140be1e6e;
  }
  plVar3 = (longlong *)
           func_0x000142923c10(*(longlong *)(lVar5 + 8) + (longlong)plVar7 * 0x48,
                               *(longlong *)(lVar5 + 8) + (longlong)plVar3 * 0x48,lVar6 * 0x48);
code_r0x000140be1e6e:
  *(longlong *)(lVar5 + 0x10) = (longlong)plVar7 + lVar6;
  return plVar3;
}

