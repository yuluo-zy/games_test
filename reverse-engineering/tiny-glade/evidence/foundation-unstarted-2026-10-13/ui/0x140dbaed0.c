
/* WARNING: Possible PIC construction at 0x000140dbaf4a: Changing call to branch */

void FUN_140dbaed0(longlong *param_1,undefined8 param_2,undefined8 param_3)

{
  undefined8 uVar1;
  longlong lVar2;
  longlong *plVar3;
  undefined1 *puVar4;
  undefined1 *unaff_RBP;
  longlong lVar5;
  longlong *unaff_RSI;
  ulonglong uVar6;
  undefined8 auStack_70 [5];
  undefined8 uStack_48;
  undefined8 uStack_40;
  longlong *plStack_38;
  longlong *plStack_30;
  longlong *plStack_28;
  undefined8 uStack_20;
  
  puVar4 = &stack0xffffffffffffffe8;
  uStack_20 = 0xfffffffffffffffe;
  auStack_70[0] = 0x140dbaefa;
  uStack_48 = param_2;
  uStack_40 = param_3;
  lVar2 = FUN_140d69590(param_2,param_3);
  if (lVar2 == 0) {
    if (-0x7ffffffffffffffc < *param_1) {
      auStack_70[0] = 0x140dbaf89;
      FUN_1419d3370(param_1 + 6,&uStack_48);
      if ((((-0x7ffffffffffffffc < *param_1) && (param_1[2] == 0)) && (param_1[5] == 0)) &&
         (param_1[8] == 0)) {
        plStack_38 = param_1 + 3;
        auStack_70[0] = 0x140dbafc5;
        plStack_30 = param_1;
        plStack_28 = param_1 + 6;
        FUN_1405f8f70(param_1);
        auStack_70[0] = 0x140dbafce;
        FUN_1405f8f70(plStack_38);
        auStack_70[0] = 0x140dbafd7;
        FUN_1405f8f70(plStack_28);
        *plStack_30 = -0x7ffffffffffffffc;
      }
    }
  }
  else if (((uint)lVar2 & 3) == 1) {
    plStack_38 = (longlong *)(lVar2 + -1);
    plStack_30 = *(longlong **)(lVar2 + -1);
    plStack_28 = *(longlong **)(lVar2 + 7);
    if ((code *)*plStack_28 != (code *)0x0) {
      auStack_70[0] = 0x140dbaf35;
      (*(code *)*plStack_28)(plStack_30);
    }
    lVar2 = plStack_28[1];
    if (lVar2 == 0) {
      lVar2 = 0x18;
      uVar6 = 8;
      plVar3 = plStack_38;
    }
    else {
      uVar6 = plStack_28[2];
      register0x00000020 = (BADSPACEBASE *)auStack_70;
      auStack_70[0] = 0x140dbaf4f;
      plVar3 = plStack_30;
      unaff_RBP = puVar4;
      unaff_RSI = param_1;
    }
    *(undefined1 **)((longlong)register0x00000020 + -8) = unaff_RBP;
    *(longlong **)((longlong)register0x00000020 + -0x10) = unaff_RSI;
    lVar5 = (longlong)plVar3;
    if (0x10 < uVar6) {
      lVar5 = *(longlong *)((longlong)plVar3 + -8);
    }
    *(undefined **)((longlong)register0x00000020 + -0x40) = &UNK_140d6582e;
    uVar1 = GetProcessHeap(plVar3,lVar2);
                    /* WARNING: Could not recover jumptable at 0x000140d6583c. Too many branches */
                    /* WARNING: Treating indirect jump as call */
    HeapFree(uVar1,0,lVar5);
    return;
  }
  return;
}

