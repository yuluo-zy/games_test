
/* WARNING: Possible PIC construction at 0x0001415fc00e: Changing call to branch */
/* WARNING: Removing unreachable block (ram,0x000140d65824) */

void FUN_1415fbff0(longlong *param_1)

{
  longlong lVar1;
  undefined8 uVar2;
  longlong lVar3;
  undefined8 unaff_RBP;
  longlong *unaff_RSI;
  undefined *apuStack_30 [5];
  
  lVar1 = *param_1;
  if (lVar1 == 0) {
    lVar1 = param_1[3];
    if (lVar1 == 0) {
      return;
    }
    lVar3 = param_1[4];
  }
  else {
    lVar3 = param_1[1];
    register0x00000020 = (BADSPACEBASE *)apuStack_30;
    apuStack_30[0] = &UNK_1415fc013;
    unaff_RSI = param_1;
  }
  *(undefined8 *)((longlong)register0x00000020 + -8) = unaff_RBP;
  *(longlong **)((longlong)register0x00000020 + -0x10) = unaff_RSI;
  *(undefined **)((longlong)register0x00000020 + -0x40) = &UNK_140d6582e;
  uVar2 = GetProcessHeap(lVar3,lVar1 << 3);
                    /* WARNING: Could not recover jumptable at 0x000140d6583c. Too many branches */
                    /* WARNING: Treating indirect jump as call */
  HeapFree(uVar2,0,lVar3);
  return;
}

