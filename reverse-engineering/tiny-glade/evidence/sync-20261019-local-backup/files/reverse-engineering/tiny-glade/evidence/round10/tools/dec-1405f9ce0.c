
/* WARNING: Possible PIC construction at 0x0001405f9d1a: Changing call to branch */
/* WARNING: Possible PIC construction at 0x0001405f9d90: Changing call to branch */
/* WARNING: Possible PIC construction at 0x0001405cff20: Changing call to branch */
/* WARNING: Removing unreachable block (ram,0x0001405cff25) */

void FUN_1405f9ce0(longlong param_1)

{
  longlong *plVar1;
  undefined8 uVar2;
  longlong lVar3;
  longlong lVar4;
  longlong *plVar5;
  longlong *unaff_RBP;
  longlong unaff_RSI;
  longlong lVar6;
  ulonglong uVar7;
  undefined *apuStack_90 [6];
  longlong lStack_60;
  longlong lStack_58;
  longlong *plStack_50;
  longlong lStack_48;
  undefined8 *puStack_40;
  longlong lStack_38;
  undefined8 uStack_30;
  longlong lStack_18;
  
  plVar1 = &lStack_18;
  lStack_18 = -2;
  if (*(longlong *)(param_1 + 0x100) == 0) {
    if (*(longlong *)(param_1 + 0x1b8) != 0) {
      plStack_50 = (longlong *)&UNK_1405f9d48;
      func_0x000140613c20(*(undefined8 *)(param_1 + 0x1c0),*(longlong *)(param_1 + 0x1b8) * 0x1c,4);
    }
    plStack_50 = (longlong *)&UNK_1405f9d58;
    FUN_1405cfe90(param_1 + 0x270);
    plStack_50 = (longlong *)&UNK_1405f9d68;
    FUN_1405cfe90(param_1 + 0x2f0);
    if (*(longlong *)(param_1 + 0x390) == 0) {
      plStack_50 = (longlong *)(param_1 + 0x448);
      plVar5 = (longlong *)&stack0xffffffffffffffd8;
      uStack_30 = 0xfffffffffffffffe;
      lStack_38 = *(longlong *)(param_1 + 0x450);
      lStack_18 = unaff_RSI;
      if (*(longlong *)(param_1 + 0x458) != 0) {
        lVar3 = lStack_38 + 0x18;
        lVar4 = 1 - *(longlong *)(param_1 + 0x458);
        do {
          lStack_48 = *(longlong *)(lVar3 + -0x18);
          puStack_40 = *(undefined8 **)(lVar3 + -0x10);
          lStack_60 = lVar3;
          lStack_58 = lVar4;
          if ((code *)*puStack_40 != (code *)0x0) {
            apuStack_90[0] = &UNK_1405cff0b;
            (*(code *)*puStack_40)(lStack_48);
          }
          lVar3 = puStack_40[1];
          if (lVar3 != 0) {
            uVar7 = puStack_40[2];
            register0x00000020 = (BADSPACEBASE *)apuStack_90;
            apuStack_90[0] = &UNK_1405cff25;
            lVar4 = lStack_48;
            unaff_RBP = plVar5;
            goto code_r0x000140613c20;
          }
          lVar3 = lStack_60 + 0x10;
          lVar4 = lStack_58 + 1;
        } while (lVar4 != 1);
      }
      if (*plStack_50 == 0) {
        return;
      }
      lVar3 = *plStack_50 << 4;
      uVar7 = 8;
      lVar4 = lStack_38;
      unaff_RSI = lStack_18;
    }
    else {
      lVar3 = *(longlong *)(param_1 + 0x390) * 0x1c;
      uVar7 = 4;
      register0x00000020 = (BADSPACEBASE *)&plStack_50;
      plStack_50 = (longlong *)&UNK_1405f9d95;
      lVar4 = *(longlong *)(param_1 + 0x398);
      unaff_RBP = plVar1;
      unaff_RSI = param_1;
    }
  }
  else {
    lVar3 = *(longlong *)(param_1 + 0x100) * 0x1c;
    uVar7 = 4;
    register0x00000020 = (BADSPACEBASE *)&plStack_50;
    plStack_50 = (longlong *)&UNK_1405f9d1f;
    lVar4 = *(longlong *)(param_1 + 0x108);
    unaff_RBP = plVar1;
    unaff_RSI = param_1;
  }
code_r0x000140613c20:
  *(longlong **)((longlong)register0x00000020 + -8) = unaff_RBP;
  *(longlong *)((longlong)register0x00000020 + -0x10) = unaff_RSI;
  lVar6 = lVar4;
  if (0x10 < uVar7) {
    lVar6 = *(longlong *)(lVar4 + -8);
  }
  *(undefined **)((longlong)register0x00000020 + -0x40) = &UNK_140d6582e;
  uVar2 = GetProcessHeap(lVar4,lVar3);
                    /* WARNING: Could not recover jumptable at 0x000140d6583c. Too many branches */
                    /* WARNING: Treating indirect jump as call */
  HeapFree(uVar2,0,lVar6);
  return;
}

