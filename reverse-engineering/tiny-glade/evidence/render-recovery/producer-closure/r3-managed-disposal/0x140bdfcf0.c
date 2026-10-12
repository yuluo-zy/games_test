
/* WARNING: Possible PIC construction at 0x000140bdfd0e: Changing call to branch */
/* WARNING: Possible PIC construction at 0x000140bdfd46: Changing call to branch */
/* WARNING: Possible PIC construction at 0x000140bdfd7e: Changing call to branch */
/* WARNING: Possible PIC construction at 0x000140bdfdbf: Changing call to branch */
/* WARNING: Possible PIC construction at 0x000140bdfe03: Changing call to branch */
/* WARNING: Removing unreachable block (ram,0x000140d65824) */

void FUN_140bdfcf0(longlong *param_1)

{
  longlong *plVar1;
  longlong lVar2;
  undefined8 uVar3;
  longlong lVar4;
  longlong lVar5;
  longlong lVar6;
  undefined8 unaff_RBP;
  longlong *unaff_RSI;
  longlong lVar7;
  longlong *plStack_e8;
  longlong *plStack_e0;
  longlong *plStack_c0;
  longlong *plStack_b8;
  undefined8 **ppuStack_b0;
  longlong lStack_88;
  longlong lStack_80;
  longlong lStack_78;
  longlong lStack_70;
  longlong lStack_68;
  longlong *plStack_60;
  longlong lStack_58;
  longlong lStack_50;
  undefined8 uStack_48;
  longlong *plStack_30;
  
  if (*param_1 == 0) {
    if (param_1[3] != 0) {
      plStack_30 = (longlong *)&UNK_140bdfd2f;
      func_0x000140613c20(param_1[4],param_1[3] << 3,8);
    }
    if (param_1[6] == 0) {
      if (param_1[9] != 0) {
        plStack_30 = (longlong *)&UNK_140bdfd67;
        func_0x000140613c20(param_1[10],param_1[9] << 3,8);
      }
      if (param_1[0xc] == 0) {
        if (param_1[0xf] != 0) {
          plStack_30 = (longlong *)&UNK_140bdfda2;
          func_0x000140613c20(param_1[0x10],param_1[0xf] << 3,8);
        }
        if (param_1[0x12] == 0) {
          if (param_1[0x15] != 0) {
            plStack_30 = (longlong *)&UNK_140bdfde6;
            func_0x000140613c20(param_1[0x16],param_1[0x15] << 3,8);
          }
          if (param_1[0x18] == 0) {
            plStack_60 = param_1 + 0x1b;
            uStack_48 = 0xfffffffffffffffe;
            lStack_50 = param_1[0x1c];
            lVar6 = param_1[0x1d];
            lStack_58 = lVar6;
            plStack_30 = unaff_RSI;
            if (lVar6 != 0) {
              lVar4 = lStack_50 + 0x38;
              lVar5 = 1;
              lVar7 = 0;
              do {
                lStack_78 = lStack_50 + lVar7 + 0x38;
                lVar2 = *(longlong *)(lStack_50 + 0x38 + lVar7);
                lStack_70 = lStack_50 + 0x40 + lVar7;
                lStack_88 = lVar4 + lVar7;
                plStack_e8 = &lStack_70;
                plStack_e0 = &lStack_78;
                plStack_c0 = &lStack_80;
                plStack_b8 = &lStack_88;
                ppuStack_b0 = &plStack_e8;
                lStack_80 = lVar2;
                lStack_68 = lVar5;
                FUN_1418bc7d0(&plStack_c0);
                if (lVar2 != 0) {
                  plStack_c0 = (longlong *)(lVar2 + -0x10);
                  LOCK();
                  plVar1 = (longlong *)(lVar2 + -0x10);
                  *plVar1 = *plVar1 + -1;
                  UNLOCK();
                  if (*plVar1 == 0) {
                    FUN_140c15f40(&plStack_c0);
                  }
                }
                lVar5 = lStack_68 + 1;
                lVar7 = lVar7 + 0x48;
              } while (lStack_68 != lVar6);
            }
            if (*plStack_60 == 0) {
              return;
            }
            lVar6 = *plStack_60 * 0x48;
            lVar4 = lStack_50;
            param_1 = plStack_30;
          }
          else {
            lVar6 = param_1[0x18] << 3;
            register0x00000020 = (BADSPACEBASE *)&plStack_30;
            plStack_30 = (longlong *)&UNK_140bdfe08;
            lVar4 = param_1[0x19];
          }
        }
        else {
          lVar6 = param_1[0x12] << 3;
          register0x00000020 = (BADSPACEBASE *)&plStack_30;
          plStack_30 = (longlong *)&UNK_140bdfdc4;
          lVar4 = param_1[0x13];
        }
      }
      else {
        lVar6 = param_1[0xc] << 3;
        register0x00000020 = (BADSPACEBASE *)&plStack_30;
        plStack_30 = (longlong *)&UNK_140bdfd83;
        lVar4 = param_1[0xd];
      }
    }
    else {
      lVar6 = param_1[6] << 3;
      register0x00000020 = (BADSPACEBASE *)&plStack_30;
      plStack_30 = (longlong *)&UNK_140bdfd4b;
      lVar4 = param_1[7];
    }
  }
  else {
    lVar6 = *param_1 << 3;
    register0x00000020 = (BADSPACEBASE *)&plStack_30;
    plStack_30 = (longlong *)&UNK_140bdfd13;
    lVar4 = param_1[1];
  }
  *(undefined8 *)((longlong)register0x00000020 + -8) = unaff_RBP;
  *(longlong **)((longlong)register0x00000020 + -0x10) = param_1;
  *(undefined **)((longlong)register0x00000020 + -0x40) = &UNK_140d6582e;
  uVar3 = GetProcessHeap(lVar4,lVar6);
                    /* WARNING: Could not recover jumptable at 0x000140d6583c. Too many branches */
                    /* WARNING: Treating indirect jump as call */
  HeapFree(uVar3,0,lVar4);
  return;
}

