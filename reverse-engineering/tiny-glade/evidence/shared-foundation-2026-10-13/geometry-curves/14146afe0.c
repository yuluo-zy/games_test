
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void FUN_14146afe0(longlong *param_1,longlong param_2,float param_3,undefined8 *param_4,
                  longlong *param_5)

{
  float fVar1;
  float fVar2;
  float fVar3;
  float fVar4;
  longlong *plVar5;
  float *pfVar6;
  longlong lVar7;
  code *pcVar8;
  bool bVar9;
  float *pfVar10;
  float *pfVar11;
  float *pfVar12;
  float *pfVar13;
  ulonglong uVar14;
  undefined8 uVar15;
  ulonglong uVar16;
  longlong unaff_R15;
  float fVar17;
  float fVar18;
  float fVar19;
  float fVar20;
  float fVar21;
  float fVar22;
  longlong lStack_1b0;
  longlong lStack_1a8;
  longlong lStack_1a0;
  longlong lStack_198;
  longlong lStack_190;
  longlong lStack_188;
  longlong lStack_180;
  longlong lStack_178;
  longlong lStack_170;
  longlong *plStack_168;
  float *pfStack_160;
  longlong **pplStack_158;
  undefined8 uStack_150;
  undefined8 uStack_148;
  longlong *plStack_138;
  longlong lStack_130;
  float *pfStack_128;
  float *pfStack_120;
  longlong *plStack_118;
  float *pfStack_110;
  longlong *plStack_108;
  code *pcStack_100;
  undefined1 uStack_f1;
  undefined8 uStack_f0;
  
  uStack_f0 = 0xfffffffffffffffe;
  plVar5 = *(longlong **)(param_2 + 0x10);
  if (plVar5 < (longlong *)0x2) {
    FUN_1428d9430(&UNK_142c63700,0x28,&UNK_142c63820);
LAB_14146b44b:
    FUN_1428d9430(&UNK_142c63838,0x2b,&UNK_142c63868);
LAB_14146b463:
    FUN_1428d9430(&UNK_142c63780,0x24,&UNK_142c63880);
LAB_14146b47b:
    uVar15 = 0;
  }
  else {
    if ((param_3 == 0.0) && (!NAN(param_3))) goto LAB_14146b44b;
    if ((*(float *)(param_2 + 0x30) == 0.0) && (!NAN(*(float *)(param_2 + 0x30))))
    goto LAB_14146b463;
    unaff_R15 = (longlong)plVar5 * 0xc;
    if ((longlong *)0xaaaaaaaaaaaaaaa < plVar5) goto LAB_14146b47b;
    pfVar6 = (float *)*param_4;
    lVar7 = *param_5;
    uVar15 = 4;
    plStack_138 = param_1;
    pfVar10 = (float *)func_0x000140613c10(unaff_R15,4);
    if (pfVar10 != (float *)0x0) {
      plStack_108 = (longlong *)0x0;
      pfVar12 = *(float **)(param_2 + 8);
      fVar1 = *pfVar12;
      fVar2 = pfVar12[1];
      uStack_f1 = 1;
      pfVar11 = pfVar6;
      lStack_130 = param_2;
      plStack_118 = plVar5;
      pfStack_110 = pfVar10;
      fVar17 = (float)FUN_1409e9640(pfVar6,fVar1);
      fVar3 = *(float *)(lVar7 + 0x3c);
      fVar18 = fVar3;
      if (fVar3 <= fVar17) {
        fVar18 = fVar17;
      }
      *pfVar10 = fVar1;
      pfVar10[1] = (float)(~-(uint)NAN(fVar17) & (uint)fVar18 | -(uint)NAN(fVar17) & (uint)fVar3);
      pfVar10[2] = fVar2;
      fVar3 = _DAT_14295a0e4;
      fVar2 = _DAT_14295a0e0;
      fVar1 = _DAT_142925904;
      plStack_108 = (longlong *)0x1;
      pfStack_120 = pfVar12 + (longlong)plVar5 * 2;
      bVar9 = true;
      while (pfVar12 != pfStack_120) {
        pfVar10 = pfVar12 + 2;
        pfVar13 = pfVar11;
        pfVar11 = pfVar12;
        if (bVar9) {
          if (pfVar10 == pfStack_120) break;
          pfVar13 = pfVar12;
          pfVar11 = pfVar10;
          pfVar10 = pfVar12 + 4;
        }
        pfStack_128 = pfVar10;
        fVar18 = SQRT((pfVar11[1] - pfVar13[1]) * (pfVar11[1] - pfVar13[1]) +
                      (*pfVar11 - *pfVar13) * (*pfVar11 - *pfVar13));
        if (fVar18 <= _DAT_14295a0d8) {
          fVar18 = _DAT_14295a0d8;
        }
        fVar17 = param_3 / fVar18;
        if (fVar1 <= param_3 / fVar18) {
          fVar17 = fVar1;
        }
        fVar18 = (float)func_0x000142923fa0(fVar1 / fVar17);
        uVar14 = (longlong)(fVar18 - _DAT_14295a0dc) & (longlong)fVar18 >> 0x3f | (longlong)fVar18;
        if (fVar18 < 0.0) {
          uVar14 = 0;
        }
        if (fVar2 < fVar18) {
          uVar14 = 0xffffffffffffffff;
        }
        uVar14 = uVar14 + 1;
        if (uVar14 < 3) {
          uVar14 = 2;
        }
        uVar16 = 1;
        while (uVar16 < uVar14) {
          fVar20 = (float)uVar16 / ((float)uVar14 + fVar3);
          fVar18 = pfVar13[1];
          fVar21 = *pfVar11 * fVar20 + *pfVar13 * (fVar1 - fVar20);
          fVar17 = pfVar11[1];
          uStack_f1 = 1;
          fVar19 = (float)FUN_1409e9640(pfVar6,fVar21);
          plVar5 = plStack_108;
          fVar4 = *(float *)(lVar7 + 0x3c);
          if (plStack_108 == plStack_118) {
            uStack_f1 = 1;
            FUN_140b9f180(&plStack_118,&UNK_142c638b0);
          }
          uVar16 = uVar16 + 1;
          fVar22 = fVar4;
          if (fVar4 <= fVar19) {
            fVar22 = fVar19;
          }
          pfStack_110[(longlong)plVar5 * 3] = fVar21;
          pfStack_110[(longlong)plVar5 * 3 + 1] =
               (float)(~-(uint)NAN(fVar19) & (uint)fVar22 | -(uint)NAN(fVar19) & (uint)fVar4);
          pfStack_110[(longlong)plVar5 * 3 + 2] = fVar20 * fVar17 + (fVar1 - fVar20) * fVar18;
          plStack_108 = (longlong *)((longlong)plVar5 + 1);
        }
        bVar9 = false;
        pfVar12 = pfStack_128;
      }
      pplStack_158 = (longlong **)plStack_108;
      plStack_168 = plStack_118;
      pfStack_160 = pfStack_110;
      uStack_f1 = 0;
      FUN_141469230(&lStack_1b0,&plStack_168);
      if (!SBORROW8(0,lStack_1b0)) {
        plStack_138[6] = lStack_180;
        plStack_138[4] = lStack_190;
        plStack_138[5] = lStack_188;
        plStack_138[2] = lStack_1a0;
        plStack_138[3] = lStack_198;
        *plStack_138 = lStack_1b0;
        plStack_138[1] = lStack_1a8;
        return;
      }
      goto LAB_14146b490;
    }
  }
  FUN_1428d8fe3(uVar15,unaff_R15,&UNK_142c63898);
LAB_14146b490:
  lStack_178 = lStack_1a8;
  lStack_170 = lStack_1a0;
  plStack_118 = &lStack_178;
  pfStack_110 = (float *)FUN_14146c6d0;
  plStack_108 = (longlong *)lStack_130;
  pcStack_100 = FUN_1412f74c0;
  plStack_168 = (longlong *)&UNK_142c63ab8;
  pfStack_160 = (float *)0x2;
  uStack_148 = 0;
  pplStack_158 = &plStack_118;
  uStack_150 = 2;
  FUN_1428d9390(&plStack_168,&UNK_142c63ad8);
  pcVar8 = (code *)swi(3);
  (*pcVar8)();
  return;
}

