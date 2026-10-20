
ulonglong * FUN_1412fcdf0(ulonglong *param_1,longlong *param_2,undefined8 param_3)

{
  longlong lVar1;
  undefined4 *puVar2;
  code *pcVar3;
  undefined1 auVar4 [16];
  ulonglong uVar5;
  ulonglong uVar6;
  ulonglong *puVar7;
  longlong lVar8;
  ulonglong uVar9;
  undefined8 *puVar10;
  undefined8 uVar11;
  undefined8 uStack_64;
  undefined4 uStack_5c;
  undefined8 *puStack_58;
  ulonglong uStack_50;
  undefined8 uStack_48;
  
  uStack_48 = 0xfffffffffffffffe;
  lVar8 = *param_2;
  lVar1 = param_2[1];
  uVar9 = (ulonglong)(lVar1 - lVar8) >> 3;
  uVar11 = 0;
  auVar4._8_8_ = 0;
  auVar4._0_8_ = uVar9;
  uVar6 = SUB168(auVar4 * ZEXT816(0xc),0);
  uStack_50 = param_3;
  if ((SUB168(auVar4 * ZEXT816(0xc),8) == 0) && (uVar6 < 0x7ffffffffffffffd)) {
    if (uVar6 == 0) {
      puStack_58 = (undefined8 *)0x4;
      uVar5 = 0;
    }
    else {
      uVar11 = 4;
      puStack_58 = (undefined8 *)func_0x000140613c10(uVar6,4);
      uVar5 = uVar9;
      if (puStack_58 == (undefined8 *)0x0) goto LAB_1412fcf04;
    }
    uStack_50 = uVar5;
    if (lVar8 == lVar1) {
      uVar9 = 0;
    }
    else {
      puVar2 = (undefined4 *)param_2[2];
      puVar10 = puStack_58;
      uVar6 = uVar9;
      do {
        func_0x000140c9cde0(&uStack_64,lVar8,*puVar2);
        *(undefined4 *)(puVar10 + 1) = uStack_5c;
        *puVar10 = uStack_64;
        lVar8 = lVar8 + 8;
        puVar10 = (undefined8 *)((longlong)puVar10 + 0xc);
        uVar6 = uVar6 - 1;
      } while (uVar6 != 0);
    }
    *param_1 = uStack_50;
    param_1[1] = (ulonglong)puStack_58;
    param_1[2] = uVar9;
    return param_1;
  }
LAB_1412fcf04:
  FUN_1428d8fe3(uVar11,uVar6,uStack_50);
  pcVar3 = (code *)swi(3);
  puVar7 = (ulonglong *)(*pcVar3)();
  return puVar7;
}

