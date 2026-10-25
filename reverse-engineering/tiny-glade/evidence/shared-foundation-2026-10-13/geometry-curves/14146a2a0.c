
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void FUN_14146a2a0(longlong *param_1,longlong param_2,float param_3)

{
  undefined8 uVar1;
  undefined8 uVar2;
  undefined4 *puVar3;
  code *pcVar4;
  float fVar5;
  float fVar6;
  float *pfVar7;
  bool bVar8;
  float *pfVar9;
  undefined4 **unaff_RBX;
  float *pfVar10;
  float *pfVar11;
  ulonglong uVar12;
  undefined4 *puVar13;
  ulonglong uVar14;
  float fVar15;
  float fVar16;
  float fVar17;
  float fVar18;
  longlong lStack_1b0;
  undefined4 uStack_1a8;
  undefined4 uStack_1a4;
  undefined4 uStack_1a0;
  undefined4 uStack_19c;
  longlong lStack_198;
  longlong lStack_190;
  longlong lStack_188;
  longlong lStack_180;
  undefined4 uStack_178;
  undefined4 uStack_174;
  undefined4 uStack_170;
  undefined4 uStack_16c;
  undefined4 *puStack_168;
  undefined8 *puStack_160;
  undefined4 **ppuStack_158;
  undefined8 uStack_150;
  undefined8 uStack_148;
  longlong *plStack_138;
  longlong lStack_130;
  float *pfStack_128;
  float *pfStack_120;
  undefined4 *puStack_118;
  code *pcStack_110;
  undefined4 *puStack_108;
  code *pcStack_100;
  undefined1 uStack_f1;
  undefined8 uStack_f0;
  
  uStack_f0 = 0xfffffffffffffffe;
  puVar3 = *(undefined4 **)(param_2 + 0x10);
  if (puVar3 < (undefined4 *)0x2) {
    FUN_1428d9430(&UNK_142c63700,0x28,&UNK_142c63820);
LAB_14146a691:
    FUN_1428d9430(&UNK_142c63838,0x2b,&UNK_142c63868);
LAB_14146a6a9:
    FUN_1428d9430(&UNK_142c63780,0x24,&UNK_142c63880);
  }
  else {
    if ((param_3 == 0.0) && (!NAN(param_3))) goto LAB_14146a691;
    if ((*(float *)(param_2 + 0x30) == 0.0) && (!NAN(*(float *)(param_2 + 0x30))))
    goto LAB_14146a6a9;
    unaff_RBX = (undefined4 **)((longlong)puVar3 * 0xc);
    plStack_138 = param_1;
    pcStack_110 = (code *)func_0x000140613c10(unaff_RBX,4);
    if (pcStack_110 != (code *)0x0) {
      pfVar10 = *(float **)(param_2 + 8);
      pfVar9 = *(float **)pfVar10;
      *(float *)((longlong)pcStack_110 + 8) = pfVar10[2];
      *(float **)pcStack_110 = pfVar9;
      fVar6 = _DAT_14295a0e4;
      fVar5 = _DAT_142925904;
      puStack_108 = (undefined4 *)0x1;
      pfStack_120 = pfVar10 + (longlong)puVar3 * 3;
      bVar8 = true;
      puVar13 = (undefined4 *)0x1;
      unaff_RBX = &puStack_118;
      lStack_130 = param_2;
      puStack_118 = puVar3;
      while (pfVar10 != pfStack_120) {
        pfVar7 = pfVar10 + 3;
        pfVar11 = pfVar9;
        pfVar9 = pfVar10;
        if (bVar8) {
          if (pfVar7 == pfStack_120) break;
          pfVar11 = pfVar10;
          pfVar9 = pfVar7;
          pfVar7 = pfVar10 + 6;
        }
        pfStack_128 = pfVar7;
        fVar15 = SQRT((pfVar9[2] - pfVar11[2]) * (pfVar9[2] - pfVar11[2]) +
                      (pfVar9[1] - pfVar11[1]) * (pfVar9[1] - pfVar11[1]) +
                      (*pfVar9 - *pfVar11) * (*pfVar9 - *pfVar11));
        if (fVar15 <= _DAT_14295a0d8) {
          fVar15 = _DAT_14295a0d8;
        }
        fVar16 = param_3 / fVar15;
        if (fVar5 <= param_3 / fVar15) {
          fVar16 = fVar5;
        }
        fVar15 = (float)func_0x000142923fa0(fVar5 / fVar16);
        uVar14 = (longlong)(fVar15 - _DAT_14295a0dc) & (longlong)fVar15 >> 0x3f | (longlong)fVar15;
        if (fVar15 < _DAT_142960690) {
          uVar14 = 0;
        }
        if (_DAT_14295a0e0 < fVar15) {
          uVar14 = 0xffffffffffffffff;
        }
        uVar14 = uVar14 + 1;
        if (uVar14 < 3) {
          uVar14 = 2;
        }
        for (uVar12 = 1; uVar12 < uVar14; uVar12 = uVar12 + 1) {
          fVar18 = (float)uVar12 / ((float)uVar14 + fVar6);
          fVar17 = fVar5 - fVar18;
          fVar15 = pfVar11[2];
          uVar1 = *(undefined8 *)pfVar11;
          uVar2 = *(undefined8 *)pfVar9;
          fVar16 = pfVar9[2];
          if (puVar13 == puStack_118) {
            uStack_f1 = 1;
            FUN_140b9f180(unaff_RBX,&UNK_142c638b0);
          }
          *(ulonglong *)((longlong)pcStack_110 + (longlong)puVar13 * 0xc) =
               CONCAT44(fVar18 * (float)((ulonglong)uVar2 >> 0x20) +
                        fVar17 * (float)((ulonglong)uVar1 >> 0x20),
                        fVar18 * (float)uVar2 + fVar17 * (float)uVar1);
          *(float *)((longlong)pcStack_110 + (longlong)puVar13 * 0xc + 8) =
               fVar15 * fVar17 + fVar16 * fVar18;
          puVar13 = (undefined4 *)((longlong)puVar13 + 1);
          puStack_108 = puVar13;
        }
        bVar8 = false;
        pfVar10 = pfStack_128;
      }
      ppuStack_158 = (undefined4 **)puStack_108;
      puStack_168 = puStack_118;
      puStack_160 = (undefined8 *)pcStack_110;
      uStack_f1 = 0;
      FUN_1414696f0(&lStack_1b0,&puStack_168);
      if (!SBORROW8(0,lStack_1b0)) {
        plStack_138[6] = lStack_180;
        plStack_138[4] = lStack_190;
        plStack_138[5] = lStack_188;
        plStack_138[2] = CONCAT44(uStack_19c,uStack_1a0);
        plStack_138[3] = lStack_198;
        *plStack_138 = lStack_1b0;
        plStack_138[1] = CONCAT44(uStack_1a4,uStack_1a8);
        return;
      }
      goto LAB_14146a6d5;
    }
  }
  FUN_1428d8fe3(4,unaff_RBX,&UNK_142c63898);
LAB_14146a6d5:
  uStack_178 = uStack_1a8;
  uStack_174 = uStack_1a4;
  uStack_170 = uStack_1a0;
  uStack_16c = uStack_19c;
  puStack_118 = &uStack_178;
  pcStack_110 = FUN_14146c6d0;
  puStack_108 = (undefined4 *)lStack_130;
  pcStack_100 = FUN_1412f7540;
  puStack_168 = (undefined4 *)&UNK_142c63ab8;
  puStack_160 = (undefined8 *)0x2;
  uStack_148 = 0;
  uStack_150 = 2;
  ppuStack_158 = unaff_RBX;
  FUN_1428d9390(&puStack_168,&UNK_142c63ad8);
  pcVar4 = (code *)swi(3);
  (*pcVar4)();
  return;
}

