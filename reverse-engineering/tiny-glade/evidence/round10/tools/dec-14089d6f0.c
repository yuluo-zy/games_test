
/* WARNING: Possible PIC construction at 0x00014089d784: Changing call to branch */

void FUN_14089d6f0(longlong *param_1)

{
  undefined8 *puVar1;
  code *pcVar2;
  undefined8 uVar3;
  longlong lVar4;
  longlong lVar5;
  undefined8 *unaff_RBP;
  longlong *unaff_RSI;
  longlong lVar6;
  ulonglong uVar7;
  undefined *apuStack_90 [5];
  undefined *puStack_68;
  longlong lStack_58;
  undefined4 uStack_50;
  undefined4 uStack_4c;
  undefined4 uStack_48;
  undefined4 uStack_44;
  longlong lStack_40;
  undefined1 auStack_38 [8];
  undefined4 uStack_30;
  undefined4 uStack_2c;
  longlong lStack_28;
  longlong lStack_20;
  undefined8 uStack_18;
  
  puVar1 = &uStack_18;
  uStack_18 = 0xfffffffffffffffe;
  apuStack_90[0] = &UNK_14089d71c;
  FUN_140d59140(&uStack_50,&UNK_142a99540,0x17);
  lStack_20 = CONCAT44(uStack_4c,uStack_50);
  if (SBORROW8(0,lStack_20)) {
    puStack_68 = &UNK_142a99558;
    apuStack_90[0] = &UNK_14089d7ee;
    FUN_1428d9760(&UNK_142a98e80,0x2b,auStack_38,&UNK_142a98eb0);
  }
  else {
    lStack_58 = CONCAT44(uStack_44,uStack_48);
    apuStack_90[0] = &UNK_14089d742;
    FUN_1414051e0(&uStack_50,lStack_58,lStack_40);
    if (!SBORROW8(0,CONCAT44(uStack_4c,uStack_50))) {
      lStack_28 = lStack_40;
      uStack_30 = uStack_48;
      uStack_2c = uStack_44;
      *(undefined4 *)param_1[2] = *(undefined4 *)((longlong)param_1 + 0x1c);
      param_1 = (longlong *)*param_1;
      if (*param_1 == 0) {
        param_1[2] = lStack_40;
        *(undefined4 *)param_1 = uStack_50;
        *(undefined4 *)((longlong)param_1 + 4) = uStack_4c;
        *(undefined4 *)(param_1 + 1) = uStack_48;
        *(undefined4 *)((longlong)param_1 + 0xc) = uStack_44;
        if (lStack_20 == 0) {
          return;
        }
        uVar7 = 1;
        lVar4 = lStack_58;
        lVar5 = lStack_20;
      }
      else {
        uVar7 = 8;
        register0x00000020 = (BADSPACEBASE *)apuStack_90;
        apuStack_90[0] = &UNK_14089d789;
        lVar4 = param_1[1];
        lVar5 = *param_1 * 0x50;
        unaff_RBP = puVar1;
        unaff_RSI = param_1;
      }
      *(undefined8 **)((longlong)register0x00000020 + -8) = unaff_RBP;
      *(longlong **)((longlong)register0x00000020 + -0x10) = unaff_RSI;
      lVar6 = lVar4;
      if (0x10 < uVar7) {
        lVar6 = *(longlong *)(lVar4 + -8);
      }
      *(undefined **)((longlong)register0x00000020 + -0x40) = &UNK_140d6582e;
      uVar3 = GetProcessHeap(lVar4,lVar5);
                    /* WARNING: Could not recover jumptable at 0x000140d6583c. Too many branches */
                    /* WARNING: Treating indirect jump as call */
      HeapFree(uVar3,0,lVar6);
      return;
    }
    puStack_68 = &UNK_142a99528;
    apuStack_90[0] = &UNK_14089d820;
    FUN_1428d9760(&UNK_142a98e80,0x2b,auStack_38,&UNK_142a98e60);
  }
                    /* WARNING: Does not return */
  pcVar2 = (code *)invalidInstructionException();
  (*pcVar2)();
}

