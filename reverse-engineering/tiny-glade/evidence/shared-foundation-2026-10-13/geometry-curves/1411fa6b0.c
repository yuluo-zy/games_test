
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void FUN_1411fa6b0(longlong *param_1,longlong param_2,float param_3)

{
  undefined4 *puVar1;
  code *pcVar2;
  float fVar3;
  float fVar4;
  float fVar5;
  float fVar6;
  undefined4 **ppuVar7;
  bool bVar8;
  undefined4 **ppuVar9;
  undefined4 **unaff_RBX;
  undefined4 **ppuVar10;
  undefined4 **ppuVar11;
  ulonglong uVar12;
  undefined4 *puVar13;
  ulonglong uVar14;
  float fVar15;
  float fVar16;
  float fVar17;
  float fVar18;
  float fVar19;
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
  undefined4 **ppuStack_128;
  undefined4 **ppuStack_120;
  undefined4 *puStack_118;
  code *pcStack_110;
  undefined4 *puStack_108;
  code *pcStack_100;
  undefined1 uStack_f1;
  undefined8 uStack_f0;
  
  uStack_f0 = 0xfffffffffffffffe;
  puVar1 = *(undefined4 **)(param_2 + 0x10);
  if (puVar1 < (undefined4 *)0x2) {
    FUN_1428d9430(&UNK_142c18e90,0x28,&UNK_142c18fb0);
LAB_1411faa5e:
    FUN_1428d9430(&UNK_142c18fc8,0x2b,&UNK_142c18ff8);
LAB_1411faa76:
    FUN_1428d9430(&UNK_142c18f10,0x24,&UNK_142c19010);
  }
  else {
    if ((param_3 == 0.0) && (!NAN(param_3))) goto LAB_1411faa5e;
    if ((*(float *)(param_2 + 0x30) == 0.0) && (!NAN(*(float *)(param_2 + 0x30))))
    goto LAB_1411faa76;
    unaff_RBX = (undefined4 **)((longlong)puVar1 * 8);
    ppuVar9 = unaff_RBX;
    plStack_138 = param_1;
    pcStack_110 = (code *)func_0x000140613c10(unaff_RBX,4);
    if (pcStack_110 != (code *)0x0) {
      ppuVar10 = *(undefined4 ***)(param_2 + 8);
      *(undefined4 **)pcStack_110 = *ppuVar10;
      fVar6 = _DAT_14295a0e4;
      fVar5 = _DAT_14295a0e0;
      fVar4 = _DAT_14295a0dc;
      fVar3 = _DAT_142925904;
      puStack_108 = (undefined4 *)0x1;
      ppuStack_120 = ppuVar10 + (longlong)puVar1;
      bVar8 = true;
      puVar13 = (undefined4 *)0x1;
      unaff_RBX = &puStack_118;
      lStack_130 = param_2;
      puStack_118 = puVar1;
      while (ppuVar10 != ppuStack_120) {
        ppuVar7 = ppuVar10 + 1;
        ppuVar11 = ppuVar9;
        ppuVar9 = ppuVar10;
        if (bVar8) {
          if (ppuVar7 == ppuStack_120) break;
          ppuVar11 = ppuVar10;
          ppuVar9 = ppuVar7;
          ppuVar7 = ppuVar10 + 2;
        }
        ppuStack_128 = ppuVar7;
        fVar15 = *(float *)((longlong)ppuVar9 + 4) - *(float *)((longlong)ppuVar11 + 4);
        fVar15 = SQRT(fVar15 * fVar15 +
                      (*(float *)ppuVar9 - *(float *)ppuVar11) *
                      (*(float *)ppuVar9 - *(float *)ppuVar11));
        if (fVar15 <= _DAT_14295a0d8) {
          fVar15 = _DAT_14295a0d8;
        }
        fVar16 = param_3 / fVar15;
        if (fVar3 <= param_3 / fVar15) {
          fVar16 = fVar3;
        }
        fVar15 = (float)func_0x000142923fa0(fVar3 / fVar16);
        uVar14 = (longlong)(fVar15 - fVar4) & (longlong)fVar15 >> 0x3f | (longlong)fVar15;
        if (fVar15 < 0.0) {
          uVar14 = 0;
        }
        if (fVar5 < fVar15) {
          uVar14 = 0xffffffffffffffff;
        }
        uVar14 = uVar14 + 1;
        if (uVar14 < 3) {
          uVar14 = 2;
        }
        for (uVar12 = 1; uVar12 < uVar14; uVar12 = uVar12 + 1) {
          if ((longlong)uVar12 < 0) {
            fVar15 = SUB84(*ppuVar11,0);
            fVar16 = (float)((ulonglong)*ppuVar11 >> 0x20);
            fVar18 = SUB84(*ppuVar9,0);
            fVar19 = (float)((ulonglong)*ppuVar9 >> 0x20);
          }
          else {
            fVar15 = SUB84(*ppuVar11,0);
            fVar16 = (float)((ulonglong)*ppuVar11 >> 0x20);
            fVar18 = SUB84(*ppuVar9,0);
            fVar19 = (float)((ulonglong)*ppuVar9 >> 0x20);
          }
          if (puVar13 == puStack_118) {
            uStack_f1 = 1;
            FUN_140b81f90(unaff_RBX,&UNK_142c19040);
          }
          fVar17 = (float)uVar12 / ((float)uVar14 + fVar6);
          *(ulonglong *)((longlong)pcStack_110 + puVar13 * 8) =
               CONCAT44(fVar17 * fVar19 + (fVar3 - fVar17) * fVar16,
                        fVar17 * fVar18 + (fVar3 - fVar17) * fVar15);
          puVar13 = (undefined4 *)((longlong)puVar13 + 1);
          puStack_108 = puVar13;
        }
        bVar8 = false;
        ppuVar10 = ppuStack_128;
      }
      ppuStack_158 = (undefined4 **)puStack_108;
      puStack_168 = puStack_118;
      puStack_160 = (undefined8 *)pcStack_110;
      uStack_f1 = 0;
      FUN_1411f9920(&lStack_1b0,&puStack_168,0);
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
      goto LAB_1411faaa2;
    }
  }
  FUN_1428d8fe3(4,unaff_RBX,&UNK_142c19028);
LAB_1411faaa2:
  uStack_178 = uStack_1a8;
  uStack_174 = uStack_1a4;
  uStack_170 = uStack_1a0;
  uStack_16c = uStack_19c;
  puStack_118 = &uStack_178;
  pcStack_110 = FUN_1411fb3e0;
  puStack_108 = (undefined4 *)lStack_130;
  pcStack_100 = FUN_141201570;
  puStack_168 = (undefined4 *)&UNK_142c19248;
  puStack_160 = (undefined8 *)0x2;
  uStack_148 = 0;
  uStack_150 = 2;
  ppuStack_158 = unaff_RBX;
  FUN_1428d9390(&puStack_168,&UNK_142c19268);
  pcVar2 = (code *)swi(3);
  (*pcVar2)();
  return;
}

