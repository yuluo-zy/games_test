
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

undefined8 FUN_141469000(longlong param_1,float param_2,undefined8 param_3)

{
  ulonglong uVar1;
  undefined8 *puVar2;
  code *pcVar3;
  ulonglong uVar4;
  undefined8 uVar5;
  ulonglong uVar6;
  ulonglong uVar7;
  undefined1 auVar8 [16];
  undefined1 auVar9 [16];
  float fVar10;
  float fVar11;
  float fVar12;
  float fVar13;
  float fVar14;
  float fVar15;
  float fVar16;
  float fVar17;
  float fVar18;
  
  fVar10 = (float)param_3;
  puVar2 = *(undefined8 **)(param_1 + 8);
  uVar7 = *(ulonglong *)(param_1 + 0x10);
  if (uVar7 == 1) {
    uVar6 = 0;
LAB_141469151:
    uVar4 = uVar6 + 1;
    if (uVar4 < uVar7) {
      uVar5 = *(undefined8 *)((longlong)puVar2 + uVar6 * 8 + 4);
      fVar11 = (float)uVar5;
      param_2 = param_2 - *(float *)(puVar2 + uVar6);
      fVar16 = fVar11 - *(float *)((longlong)puVar2 + uVar6 * 8 + 0xc);
      fVar10 = fVar10 - fVar11;
      fVar11 = *(float *)(puVar2 + uVar6) - (float)((ulonglong)uVar5 >> 0x20);
      fVar15 = (float)((ulonglong)param_3 >> 0x20) - 0.0;
      auVar8._0_4_ = param_2 * param_2;
      auVar8._4_4_ = fVar16 * fVar16;
      auVar8._8_8_ = 0;
      auVar9._0_4_ = fVar10 * fVar10 + auVar8._0_4_;
      auVar9._4_4_ = fVar11 * fVar11 + auVar8._4_4_;
      auVar9._8_4_ = fVar15 * fVar15 + 0.0;
      auVar9._12_4_ = 0;
      auVar8 = sqrtps(auVar8,auVar9);
      auVar8 = divps(_DAT_14292e6b0,auVar8);
      return CONCAT71((int7)(uVar4 >> 8),
                      fVar16 * auVar8._4_4_ * param_2 * auVar8._0_4_ <
                      auVar8._4_4_ * fVar11 * auVar8._0_4_ * fVar10);
    }
LAB_1414691f6:
    FUN_1428d9518(uVar4);
  }
  else {
    uVar6 = (uVar7 + (uVar7 == 0)) - 1;
    uVar4 = uVar6;
    if (uVar7 < uVar6) {
      uVar4 = uVar7;
    }
    uVar1 = uVar7 - 2;
    if (uVar1 <= uVar4) {
      uVar4 = uVar1;
    }
    if (uVar7 != uVar4) {
      if (uVar6 <= uVar1) goto LAB_141469214;
      uVar6 = 0;
      uVar4 = 0;
      fVar11 = _DAT_14295b630;
      fVar15 = (float)*puVar2;
      fVar16 = (float)((ulonglong)*puVar2 >> 0x20);
      do {
        fVar12 = (float)puVar2[uVar4 + 1];
        fVar13 = (float)((ulonglong)puVar2[uVar4 + 1] >> 0x20);
        fVar17 = fVar12 - fVar15;
        fVar18 = fVar13 - fVar16;
        fVar14 = fVar18 * fVar18 + fVar17 * fVar17;
        if (fVar14 <= _DAT_142a7baa0) {
          fVar14 = _DAT_142a7baa0;
        }
        fVar14 = ((fVar10 - fVar16) * fVar18 + (param_2 - fVar15) * fVar17) / fVar14;
        fVar17 = 0.0;
        if (0.0 <= fVar14) {
          fVar17 = fVar14;
        }
        fVar14 = _DAT_142925904;
        if (fVar17 <= _DAT_142925904) {
          fVar14 = fVar17;
        }
        fVar15 = param_2 - (fVar14 * fVar12 + (_DAT_142925904 - fVar14) * fVar15);
        fVar16 = fVar10 - (fVar14 * fVar13 + (_DAT_142925904 - fVar14) * fVar16);
        fVar15 = SQRT(fVar16 * fVar16 + fVar15 * fVar15);
        uVar1 = uVar4;
        if (fVar11 <= fVar15) {
          fVar15 = fVar11;
          uVar1 = uVar6;
        }
        uVar6 = uVar1;
        fVar11 = fVar15;
        uVar4 = uVar4 + 1;
        fVar15 = fVar12;
        fVar16 = fVar13;
      } while (uVar7 - 1 != uVar4);
      if (uVar6 < uVar7) goto LAB_141469151;
      uVar4 = FUN_1428d9518(uVar6,uVar7,&UNK_142c636a0);
      goto LAB_1414691f6;
    }
  }
  FUN_1428d9518(uVar7);
LAB_141469214:
  FUN_1428d9518();
  pcVar3 = (code *)swi(3);
  uVar5 = (*pcVar3)();
  return uVar5;
}

