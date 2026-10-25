
ulonglong * FUN_140bed4c0(ulonglong *param_1,ulonglong param_2)

{
  longlong lVar1;
  code *pcVar2;
  undefined1 auVar3 [16];
  ulonglong uVar4;
  ulonglong uVar5;
  ulonglong uVar6;
  ulonglong *puVar7;
  longlong lVar8;
  undefined8 *puVar9;
  undefined8 uVar10;
  
  uVar10 = 0;
  auVar3._8_8_ = 0;
  auVar3._0_8_ = param_2;
  uVar5 = SUB168(auVar3 * ZEXT816(0x18),0);
  if ((SUB168(auVar3 * ZEXT816(0x18),8) == 0) && (uVar5 < 0x7ffffffffffffff9)) {
    if (uVar5 == 0) {
      uVar6 = 8;
      uVar4 = 0;
    }
    else {
      uVar10 = 8;
      uVar6 = func_0x000140613c10(uVar5,8);
      uVar4 = param_2;
      if (uVar6 == 0) goto code_r0x000140bed60b;
    }
    if (param_2 != 0) {
      if (param_2 < 4) {
        uVar5 = 0;
      }
      else {
        puVar9 = (undefined8 *)(uVar6 + 0x58);
        uVar5 = 0;
        do {
          puVar9[-0xb] = 0;
          puVar9[-10] = 8;
          puVar9[-9] = 0;
          puVar9[-8] = 0;
          puVar9[-7] = 8;
          puVar9[-6] = 0;
          puVar9[-5] = 0;
          puVar9[-4] = 8;
          puVar9[-3] = 0;
          puVar9[-2] = 0;
          uVar5 = uVar5 + 4;
          puVar9[-1] = 8;
          *puVar9 = 0;
          puVar9 = puVar9 + 0xc;
        } while ((param_2 & 0xfffffffffffffffc) != uVar5);
      }
      if ((param_2 & 3) != 0) {
        lVar1 = uVar6 + uVar5 * 0x18;
        lVar8 = 0;
        do {
          *(undefined8 *)(lVar1 + lVar8) = 0;
          *(undefined8 *)(lVar1 + 8 + lVar8) = 8;
          *(undefined8 *)(lVar1 + 0x10 + lVar8) = 0;
          lVar8 = lVar8 + 0x18;
        } while ((ulonglong)(((uint)param_2 & 3) << 3) * 3 != lVar8);
      }
    }
    *param_1 = uVar4;
    param_1[1] = uVar6;
    param_1[2] = param_2;
    return param_1;
  }
code_r0x000140bed60b:
  FUN_1428d8fe3(uVar10,uVar5,&UNK_142b14b18);
  pcVar2 = (code *)swi(3);
  puVar7 = (ulonglong *)(*pcVar2)();
  return puVar7;
}

