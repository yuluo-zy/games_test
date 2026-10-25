
ulonglong * FUN_1412fc900(ulonglong *param_1,longlong *param_2,undefined8 param_3)

{
  longlong lVar1;
  longlong lVar2;
  longlong lVar3;
  code *pcVar4;
  undefined1 auVar5 [16];
  ulonglong uVar6;
  ulonglong uVar7;
  ulonglong *puVar8;
  ulonglong uVar9;
  longlong lVar10;
  longlong lVar11;
  undefined8 uVar12;
  undefined8 uStack_7c;
  undefined4 uStack_74;
  ulonglong *puStack_70;
  ulonglong uStack_68;
  undefined4 uStack_60;
  undefined4 uStack_5c;
  ulonglong uStack_58;
  ulonglong uStack_50;
  undefined8 uStack_48;
  
  uStack_48 = 0xfffffffffffffffe;
  lVar11 = param_2[4];
  lVar2 = param_2[5];
  uVar9 = lVar2 - lVar11;
  uVar12 = 0;
  auVar5._8_8_ = 0;
  auVar5._0_8_ = uVar9;
  uVar7 = SUB168(auVar5 * ZEXT816(0xc),0);
  uStack_50 = param_3;
  if ((SUB168(auVar5 * ZEXT816(0xc),8) == 0) && (uVar7 < 0x7ffffffffffffffd)) {
    if (uVar7 == 0) {
      uStack_58 = 4;
      uVar6 = 0;
    }
    else {
      uVar12 = 4;
      uStack_58 = func_0x000140613c10(uVar7,4);
      uVar6 = uVar9;
      if (uStack_58 == 0) goto LAB_1412fca57;
    }
    uStack_50 = uVar6;
    if (lVar2 == lVar11) {
      uStack_68 = 0;
    }
    else {
      lVar3 = param_2[2];
      lVar1 = *param_2 + lVar11 * 0xc;
      lVar10 = 0;
      puStack_70 = param_1;
      uStack_68 = uVar9;
      do {
        uStack_60 = *(undefined4 *)(lVar1 + lVar10);
        uStack_5c = *(undefined4 *)(lVar1 + 8 + lVar10);
        func_0x000140c9cde0(&uStack_7c,&uStack_60,*(undefined4 *)(lVar3 + lVar11 * 4));
        *(undefined4 *)(uStack_58 + 8 + lVar10) = uStack_74;
        *(undefined8 *)(uStack_58 + lVar10) = uStack_7c;
        lVar11 = lVar11 + 1;
        lVar10 = lVar10 + 0xc;
        param_1 = puStack_70;
      } while (lVar2 != lVar11);
    }
    *param_1 = uStack_50;
    param_1[1] = uStack_58;
    param_1[2] = uStack_68;
    return param_1;
  }
LAB_1412fca57:
  FUN_1428d8fe3(uVar12,uVar7,uStack_50);
  pcVar4 = (code *)swi(3);
  puVar8 = (ulonglong *)(*pcVar4)();
  return puVar8;
}

