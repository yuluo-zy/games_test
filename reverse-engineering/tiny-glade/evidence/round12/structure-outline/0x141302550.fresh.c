
ulonglong * FUN_141302550(ulonglong *param_1,longlong *param_2,undefined8 param_3)

{
  longlong lVar1;
  longlong lVar2;
  undefined1 auVar3 [16];
  code *pcVar4;
  ulonglong uVar5;
  ulonglong *puVar6;
  ulonglong uVar7;
  undefined8 uVar8;
  ulonglong uVar9;
  undefined *puStack_88;
  undefined8 uStack_80;
  undefined8 uStack_78;
  undefined8 uStack_70;
  undefined8 uStack_68;
  ulonglong uStack_58;
  ulonglong uStack_50;
  undefined8 uStack_48;
  
  uStack_48 = 0xfffffffffffffffe;
  lVar1 = *param_2;
  lVar2 = param_2[1];
  uVar7 = (ulonglong)(lVar2 - lVar1) >> 3;
  uVar8 = 0;
  auVar3._8_8_ = 0;
  auVar3._0_8_ = uVar7;
  uVar5 = SUB168(auVar3 * ZEXT816(0xc),0);
  uStack_50 = param_3;
  if ((SUB168(auVar3 * ZEXT816(0xc),8) == 0) && (uVar5 < 0x7ffffffffffffffd)) {
    if (uVar5 == 0) {
      uStack_58 = 4;
      uStack_50 = 0;
      uVar9 = uStack_50;
    }
    else {
      uVar8 = 4;
      uStack_58 = func_0x000140613c10(uVar5,4);
      uVar9 = uVar7;
      if (uStack_58 == 0) goto LAB_1413026c2;
    }
    uStack_50 = uVar9;
    if (lVar1 == lVar2) {
      uVar5 = 0;
    }
    else {
      lVar2 = param_2[2];
      uVar5 = 0;
      uVar9 = 0;
      do {
        if ((*(byte *)(lVar2 + 0x108) & 1) == 0) {
          puStack_88 = &UNK_142aeaea0;
          uStack_80 = 1;
          uStack_78 = 8;
          uStack_70 = 0;
          uStack_68 = 0;
          FUN_1428d9390(&puStack_88,&UNK_142aeaeb0);
                    /* WARNING: Does not return */
          pcVar4 = (code *)invalidInstructionException();
          (*pcVar4)();
        }
        func_0x000140c9cde0(&puStack_88,uVar9 * 8 + lVar1,*(undefined4 *)(lVar2 + 0x10c));
        *(undefined4 *)(uStack_58 + 8 + uVar5 * 0xc) = (undefined4)uStack_80;
        *(undefined **)(uStack_58 + uVar5 * 0xc) = puStack_88;
        uVar5 = uVar5 + 1;
        uVar9 = uVar9 + 1;
      } while (uVar9 != uVar7);
    }
    *param_1 = uStack_50;
    param_1[1] = uStack_58;
    param_1[2] = uVar5;
    return param_1;
  }
LAB_1413026c2:
  FUN_1428d8fe3(uVar8,uVar5,uStack_50);
  pcVar4 = (code *)swi(3);
  puVar6 = (ulonglong *)(*pcVar4)();
  return puVar6;
}

