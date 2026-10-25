
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void FUN_14146b5f0(longlong *param_1,longlong param_2,float param_3)

{
  float *pfVar1;
  undefined4 *puVar2;
  code *pcVar3;
  float fVar4;
  float fVar5;
  float fVar6;
  float fVar7;
  float fVar8;
  bool bVar9;
  longlong *plVar10;
  float *pfVar11;
  ulonglong uVar12;
  float *pfVar13;
  float *pfVar14;
  ulonglong uVar15;
  longlong unaff_R14;
  undefined8 uVar16;
  float fVar17;
  float fVar18;
  undefined4 uStack_198;
  undefined4 uStack_194;
  undefined4 uStack_190;
  undefined4 uStack_18c;
  longlong lStack_180;
  undefined4 uStack_178;
  undefined4 uStack_174;
  undefined4 uStack_170;
  undefined4 uStack_16c;
  longlong lStack_168;
  longlong lStack_160;
  longlong lStack_158;
  longlong lStack_150;
  longlong *plStack_148;
  longlong lStack_140;
  undefined4 *puStack_138;
  longlong *plStack_130;
  undefined4 **ppuStack_128;
  undefined8 uStack_120;
  undefined8 uStack_118;
  float *pfStack_108;
  float *pfStack_100;
  undefined4 *puStack_f8;
  code *pcStack_f0;
  undefined4 *puStack_e8;
  code *pcStack_e0;
  undefined1 uStack_d1;
  undefined8 uStack_d0;
  
  uStack_d0 = 0xfffffffffffffffe;
  puVar2 = *(undefined4 **)(param_2 + 0x10);
  if (puVar2 < (undefined4 *)0x2) {
    FUN_1428d9430(&UNK_142c63700,0x28,&UNK_142c63820);
LAB_14146b9e5:
    FUN_1428d9430(&UNK_142c63838,0x2b,&UNK_142c63868);
LAB_14146b9fd:
    FUN_1428d9430(&UNK_142c63780,0x24,&UNK_142c63880);
LAB_14146ba15:
    uVar16 = 0;
  }
  else {
    if ((param_3 == 0.0) && (!NAN(param_3))) goto LAB_14146b9e5;
    if ((*(float *)(param_2 + 0x30) == 0.0) && (!NAN(*(float *)(param_2 + 0x30))))
    goto LAB_14146b9fd;
    unaff_R14 = (longlong)puVar2 * 0xc;
    if ((undefined4 *)0xaaaaaaaaaaaaaaa < puVar2) goto LAB_14146ba15;
    uVar16 = 4;
    plStack_148 = param_1;
    plVar10 = (longlong *)func_0x000140613c10(unaff_R14,4);
    if (plVar10 != (longlong *)0x0) {
      puStack_e8 = (undefined4 *)0x0;
      pfVar13 = *(float **)(param_2 + 8);
      puStack_138 = *(undefined4 **)pfVar13;
      uStack_d1 = 1;
      pfVar11 = (float *)&lStack_180;
      lStack_140 = param_2;
      puStack_f8 = puVar2;
      pcStack_f0 = (code *)plVar10;
      func_0x000140c9cde0(pfVar11,&puStack_138,0);
      *(undefined4 *)(plVar10 + 1) = uStack_178;
      *plVar10 = lStack_180;
      fVar8 = _DAT_14295a0e4;
      fVar7 = _DAT_14295a0e0;
      fVar6 = _DAT_14295a0dc;
      fVar5 = _DAT_14295a0d8;
      fVar4 = _DAT_142925904;
      puStack_e8 = (undefined4 *)0x1;
      pfStack_100 = pfVar13 + (longlong)puVar2 * 2;
      bVar9 = true;
      while (pfVar13 != pfStack_100) {
        pfVar1 = pfVar13 + 2;
        if (bVar9) {
          if (pfVar1 == pfStack_100) break;
          pfStack_108 = pfVar13 + 4;
          pfVar14 = pfVar13;
          pfVar11 = pfVar1;
        }
        else {
          pfStack_108 = pfVar1;
          pfVar14 = pfVar11;
          pfVar11 = pfVar13;
        }
        fVar17 = SQRT((pfVar11[1] - pfVar14[1]) * (pfVar11[1] - pfVar14[1]) +
                      (*pfVar11 - *pfVar14) * (*pfVar11 - *pfVar14));
        if (fVar17 <= fVar5) {
          fVar17 = fVar5;
        }
        fVar18 = param_3 / fVar17;
        if (fVar4 <= param_3 / fVar17) {
          fVar18 = fVar4;
        }
        fVar17 = (float)func_0x000142923fa0(fVar4 / fVar18);
        uVar15 = (longlong)(fVar17 - fVar6) & (longlong)fVar17 >> 0x3f | (longlong)fVar17;
        if (fVar17 < 0.0) {
          uVar15 = 0;
        }
        if (fVar7 < fVar17) {
          uVar15 = 0xffffffffffffffff;
        }
        uVar15 = uVar15 + 1;
        if (uVar15 < 3) {
          uVar15 = 2;
        }
        for (uVar12 = 1; uVar12 < uVar15; uVar12 = uVar12 + 1) {
          fVar17 = (float)uVar12 / ((float)uVar15 + fVar8);
          puStack_138 = (undefined4 *)
                        CONCAT44(fVar17 * (float)((ulonglong)*(undefined8 *)pfVar11 >> 0x20) +
                                 (fVar4 - fVar17) *
                                 (float)((ulonglong)*(undefined8 *)pfVar14 >> 0x20),
                                 fVar17 * (float)*(undefined8 *)pfVar11 +
                                 (fVar4 - fVar17) * (float)*(undefined8 *)pfVar14);
          uStack_d1 = 1;
          func_0x000140c9cde0(&lStack_180,&puStack_138,0);
          puVar2 = puStack_e8;
          if (puStack_e8 == puStack_f8) {
            uStack_d1 = 1;
            FUN_140b9f180(&puStack_f8,&UNK_142c638b0);
          }
          *(undefined4 *)((longlong)pcStack_f0 + (longlong)puVar2 * 0xc + 8) = uStack_178;
          *(longlong *)((longlong)pcStack_f0 + (longlong)puVar2 * 0xc) = lStack_180;
          puStack_e8 = (undefined4 *)((longlong)puVar2 + 1);
        }
        bVar9 = false;
        pfVar13 = pfStack_108;
      }
      ppuStack_128 = (undefined4 **)puStack_e8;
      puStack_138 = puStack_f8;
      plStack_130 = (longlong *)pcStack_f0;
      uStack_d1 = 0;
      FUN_141469230(&lStack_180,&puStack_138);
      if (!SBORROW8(0,lStack_180)) {
        plStack_148[6] = lStack_150;
        plStack_148[4] = lStack_160;
        plStack_148[5] = lStack_158;
        plStack_148[2] = CONCAT44(uStack_16c,uStack_170);
        plStack_148[3] = lStack_168;
        *plStack_148 = lStack_180;
        plStack_148[1] = CONCAT44(uStack_174,uStack_178);
        return;
      }
      goto LAB_14146ba2a;
    }
  }
  FUN_1428d8fe3(uVar16,unaff_R14,&UNK_142c63898);
LAB_14146ba2a:
  uStack_198 = uStack_178;
  uStack_194 = uStack_174;
  uStack_190 = uStack_170;
  uStack_18c = uStack_16c;
  puStack_f8 = &uStack_198;
  pcStack_f0 = FUN_14146c6d0;
  puStack_e8 = (undefined4 *)lStack_140;
  pcStack_e0 = FUN_1412f74c0;
  puStack_138 = (undefined4 *)&UNK_142c63ab8;
  plStack_130 = (longlong *)0x2;
  uStack_118 = 0;
  ppuStack_128 = &puStack_f8;
  uStack_120 = 2;
  FUN_1428d9390(&puStack_138,&UNK_142c63ad8);
  pcVar3 = (code *)swi(3);
  (*pcVar3)();
  return;
}

