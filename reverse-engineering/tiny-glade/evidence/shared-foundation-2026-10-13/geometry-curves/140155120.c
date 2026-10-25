
ulonglong * FUN_140155120(ulonglong *param_1,longlong param_2)

{
  undefined4 uVar1;
  ulonglong uVar2;
  ulonglong uVar3;
  undefined1 auVar4 [16];
  code *pcVar5;
  ulonglong uVar6;
  ulonglong uVar7;
  ulonglong uVar8;
  undefined8 uVar9;
  undefined8 *puVar10;
  ulonglong unaff_R14;
  undefined8 uStack_6c;
  undefined4 uStack_64;
  undefined8 uStack_60;
  undefined8 *puStack_58;
  ulonglong uStack_50;
  undefined8 uStack_48;
  
  uStack_48 = 0xfffffffffffffffe;
  uVar2 = *(ulonglong *)(param_2 + 0x10);
  uVar9 = 0;
  auVar4._8_8_ = 0;
  auVar4._0_8_ = uVar2;
  uVar6 = SUB168(auVar4 * ZEXT816(0xc),0);
  if ((SUB168(auVar4 * ZEXT816(0xc),8) == 0) && (uVar6 < 0x7ffffffffffffffd)) {
    unaff_R14 = *(ulonglong *)(param_2 + 8);
    if (uVar6 == 0) {
      puStack_58 = (undefined8 *)0x4;
      uStack_50 = 0;
      puVar10 = puStack_58;
      uVar3 = uStack_50;
    }
    else {
      uVar9 = 4;
      puStack_58 = (undefined8 *)func_0x000140613c10(uVar6,4);
      puVar10 = puStack_58;
      uVar3 = uVar2;
      if (puStack_58 == (undefined8 *)0x0) goto LAB_1401552b5;
    }
    uStack_50 = uVar3;
    puStack_58 = puVar10;
    if (uVar2 != 0) {
      uVar6 = uVar2;
      do {
        func_0x000140c9cdc0(&uStack_6c,unaff_R14);
        *(undefined4 *)(puVar10 + 1) = uStack_64;
        *puVar10 = uStack_6c;
        puVar10 = (undefined8 *)((longlong)puVar10 + 0xc);
        unaff_R14 = unaff_R14 + 8;
        uVar6 = uVar6 - 1;
      } while (uVar6 != 0);
    }
    uVar3 = *(ulonglong *)(param_2 + 0x28);
    unaff_R14 = uVar3 * 4;
    uVar6 = 0;
    if ((uVar3 >> 0x3e == 0) && (unaff_R14 < 0x7ffffffffffffffd)) {
      if (unaff_R14 == 0) {
        uVar7 = 4;
        uVar9 = *(undefined8 *)(param_2 + 0x20);
        uVar8 = 0;
      }
      else {
        uVar6 = 4;
        uStack_60 = *(undefined8 *)(param_2 + 0x20);
        uVar7 = func_0x000140613c10(unaff_R14,4);
        uVar9 = uStack_60;
        uVar8 = uVar3;
        if (uVar7 == 0) goto LAB_1401552c7;
      }
      func_0x000142923c00(uVar7,uVar9,unaff_R14);
      uVar1 = *(undefined4 *)(param_2 + 0x30);
      *param_1 = uStack_50;
      param_1[1] = (ulonglong)puStack_58;
      param_1[2] = uVar2;
      param_1[3] = uVar8;
      param_1[4] = uVar7;
      param_1[5] = uVar3;
      *(undefined4 *)(param_1 + 6) = uVar1;
      return param_1;
    }
  }
  else {
LAB_1401552b5:
    FUN_1428d8fe3(uVar9,uVar6,&UNK_14295a148);
  }
LAB_1401552c7:
  FUN_1428d8fe3(uVar6,unaff_R14,&UNK_1429ae438);
                    /* WARNING: Does not return */
  pcVar5 = (code *)invalidInstructionException();
  (*pcVar5)();
}

