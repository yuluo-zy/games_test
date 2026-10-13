
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void FUN_14146bb70(longlong *param_1,longlong param_2,byte param_3,float param_4,undefined8 param_5,
                  longlong param_6)

{
  undefined *puVar1;
  code *pcVar2;
  float fVar3;
  float fVar4;
  float fVar5;
  float fVar6;
  bool bVar7;
  longlong *plVar8;
  ulonglong uVar9;
  byte bVar10;
  longlong *plVar11;
  longlong *plVar12;
  longlong *plVar13;
  longlong unaff_R14;
  undefined8 uVar14;
  ulonglong uVar15;
  uint uVar16;
  float fVar17;
  float fVar18;
  float fVar19;
  float fVar20;
  float fVar21;
  undefined1 auStack_1a8 [8];
  longlong lStack_1a0;
  longlong lStack_190;
  undefined4 uStack_188;
  undefined4 uStack_184;
  longlong lStack_180;
  longlong lStack_178;
  longlong lStack_170;
  longlong lStack_168;
  longlong lStack_160;
  longlong *plStack_158;
  longlong lStack_150;
  undefined *puStack_148;
  longlong *plStack_140;
  undefined **ppuStack_138;
  undefined8 uStack_130;
  undefined8 uStack_128;
  longlong *plStack_118;
  longlong *plStack_110;
  undefined *puStack_108;
  code *pcStack_100;
  undefined *puStack_f8;
  code *pcStack_f0;
  byte bStack_e2;
  undefined1 uStack_e1;
  undefined8 uStack_e0;
  
  uStack_e0 = 0xfffffffffffffffe;
  puVar1 = *(undefined **)(param_2 + 0x10);
  bStack_e2 = param_3;
  if (puVar1 < (undefined *)0x2) {
    FUN_1428d9430(&UNK_142c63700,0x28,&UNK_142c63820);
LAB_14146bff9:
    FUN_1428d9430(&UNK_142c63838,0x2b,&UNK_142c63868);
LAB_14146c011:
    FUN_1428d9430(&UNK_142c63780,0x24,&UNK_142c63880);
LAB_14146c029:
    uVar14 = 0;
  }
  else {
    if ((param_4 == 0.0) && (!NAN(param_4))) goto LAB_14146bff9;
    if ((*(float *)(param_2 + 0x30) == 0.0) && (!NAN(*(float *)(param_2 + 0x30))))
    goto LAB_14146c011;
    unaff_R14 = (longlong)puVar1 * 0xc;
    if ((undefined *)0xaaaaaaaaaaaaaaa < puVar1) goto LAB_14146c029;
    uVar14 = 4;
    plStack_158 = param_1;
    plVar8 = (longlong *)func_0x000140613c10(unaff_R14,4);
    if (plVar8 != (longlong *)0x0) {
      puStack_f8 = (undefined *)0x0;
      plVar12 = *(longlong **)(param_2 + 8);
      puStack_148 = (undefined *)*plVar12;
      uStack_e1 = 1;
      lStack_150 = param_2;
      puStack_108 = puVar1;
      pcStack_100 = (code *)plVar8;
      fVar17 = (float)FUN_1409e9640(param_5,(int)puStack_148,(int)((ulonglong)puStack_148 >> 0x20));
      uVar16 = -(uint)(0.0 < *(float *)(param_6 + 0x58));
      fVar3 = (float)(~uVar16 & _DAT_142c6328c | (uint)*(float *)(param_6 + 0x58) & uVar16);
      fVar20 = fVar3;
      if (fVar3 <= fVar17) {
        fVar20 = fVar17;
      }
      uStack_e1 = 1;
      plVar11 = &lStack_190;
      func_0x000140c9cde0(plVar11,&puStack_148,
                          ~-(uint)NAN(fVar17) & (uint)fVar20 | -(uint)NAN(fVar17) & (uint)fVar3);
      *(undefined4 *)(plVar8 + 1) = uStack_188;
      *plVar8 = lStack_190;
      fVar6 = _DAT_14295a0e4;
      fVar5 = _DAT_14295a0e0;
      fVar4 = _DAT_14295a0dc;
      fVar17 = _DAT_14295a0d8;
      fVar20 = _DAT_142925904;
      puStack_f8 = (undefined *)0x1;
      plStack_110 = plVar12 + (longlong)puVar1;
      bVar7 = true;
      while (plVar12 != plStack_110) {
        plVar8 = plVar12 + 1;
        if (bVar7) {
          if (plVar8 == plStack_110) break;
          plStack_118 = plVar12 + 2;
          plVar13 = plVar12;
          plVar11 = plVar8;
        }
        else {
          plStack_118 = plVar8;
          plVar13 = plVar11;
          plVar11 = plVar12;
        }
        fVar18 = (float)*plVar11 - (float)*plVar13;
        fVar19 = (float)((ulonglong)*plVar11 >> 0x20) - (float)((ulonglong)*plVar13 >> 0x20);
        fVar18 = SQRT(fVar19 * fVar19 + fVar18 * fVar18);
        if (fVar18 <= fVar17) {
          fVar18 = fVar17;
        }
        fVar19 = param_4 / fVar18;
        if (fVar20 <= param_4 / fVar18) {
          fVar19 = fVar20;
        }
        fVar18 = (float)func_0x000142923fa0(fVar20 / fVar19);
        uVar9 = (longlong)(fVar18 - fVar4) & (longlong)fVar18 >> 0x3f | (longlong)fVar18;
        if (fVar18 < 0.0) {
          uVar9 = 0;
        }
        if (fVar5 < fVar18) {
          uVar9 = 0xffffffffffffffff;
        }
        uVar9 = uVar9 + 1;
        if (uVar9 < 3) {
          uVar9 = 2;
        }
        bVar10 = (uVar9 & 1) == 0 & bStack_e2;
        uVar15 = bVar10 | uVar9;
        if ((longlong)uVar15 < 0) {
          fVar18 = (float)(uVar9 >> 1 | (ulonglong)((uint)bVar10 | (uint)uVar9 & 1));
          fVar18 = fVar18 + fVar18;
        }
        else {
          fVar18 = (float)(longlong)uVar15;
        }
        for (uVar9 = 1; uVar9 < uVar15; uVar9 = uVar9 + 1) {
          fVar19 = (float)uVar9 / (fVar18 + fVar6);
          fVar21 = fVar19 * (float)*plVar11 + (fVar20 - fVar19) * (float)*plVar13;
          fVar19 = fVar19 * (float)((ulonglong)*plVar11 >> 0x20) +
                   (fVar20 - fVar19) * (float)((ulonglong)*plVar13 >> 0x20);
          puStack_148 = (undefined *)CONCAT44(fVar19,fVar21);
          uStack_e1 = 1;
          fVar21 = (float)FUN_1409e9640(param_5,fVar21,fVar19);
          fVar19 = fVar3;
          if (fVar3 <= fVar21) {
            fVar19 = fVar21;
          }
          uStack_e1 = 1;
          func_0x000140c9cde0(&lStack_190,&puStack_148,
                              ~-(uint)NAN(fVar21) & (uint)fVar19 | -(uint)NAN(fVar21) & (uint)fVar3)
          ;
          puVar1 = puStack_f8;
          if (puStack_f8 == puStack_108) {
            uStack_e1 = 1;
            FUN_140b9f180(&puStack_108,&UNK_142c638b0);
          }
          *(undefined4 *)((longlong)pcStack_100 + (longlong)puVar1 * 0xc + 8) = uStack_188;
          *(longlong *)((longlong)pcStack_100 + (longlong)puVar1 * 0xc) = lStack_190;
          puStack_f8 = puVar1 + 1;
        }
        bVar7 = false;
        plVar12 = plStack_118;
      }
      ppuStack_138 = (undefined **)puStack_f8;
      puStack_148 = puStack_108;
      plStack_140 = (longlong *)pcStack_100;
      uStack_e1 = 0;
      FUN_141469230(&lStack_190,&puStack_148);
      if (!SBORROW8(0,lStack_190)) {
        plStack_158[6] = lStack_160;
        plStack_158[4] = lStack_170;
        plStack_158[5] = lStack_168;
        plStack_158[2] = lStack_180;
        plStack_158[3] = lStack_178;
        *plStack_158 = lStack_190;
        plStack_158[1] = CONCAT44(uStack_184,uStack_188);
        return;
      }
      goto LAB_14146c03e;
    }
  }
  FUN_1428d8fe3(uVar14,unaff_R14,&UNK_142c63898);
LAB_14146c03e:
  lStack_1a0 = lStack_180;
  puStack_108 = auStack_1a8;
  pcStack_100 = FUN_14146c6d0;
  puStack_f8 = (undefined *)lStack_150;
  pcStack_f0 = FUN_1412f74c0;
  puStack_148 = &UNK_142c63ab8;
  plStack_140 = (longlong *)0x2;
  uStack_128 = 0;
  ppuStack_138 = &puStack_108;
  uStack_130 = 2;
  FUN_1428d9390(&puStack_148,&UNK_142c63ad8);
  pcVar2 = (code *)swi(3);
  (*pcVar2)();
  return;
}

