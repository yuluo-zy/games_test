
/* WARNING: Removing unreachable block (ram,0x00014146ac06) */
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

longlong * FUN_14146a830(longlong *param_1,longlong param_2,ulonglong **param_3)

{
  ulonglong **ppuVar1;
  float fVar2;
  undefined8 uVar3;
  undefined8 *puVar4;
  code *pcVar5;
  undefined8 *puVar6;
  longlong lVar7;
  longlong *plVar8;
  longlong lVar9;
  longlong unaff_RSI;
  longlong *unaff_RDI;
  ulonglong uVar10;
  undefined8 ***pppuVar11;
  ulonglong unaff_R13;
  ulonglong uVar12;
  undefined8 uVar13;
  float fVar14;
  float fVar15;
  uint uVar16;
  float fVar17;
  uint uVar18;
  float fVar19;
  float fVar20;
  float unaff_XMM6_Da;
  float fVar21;
  float fVar22;
  float fVar23;
  float fVar24;
  float fVar25;
  float fVar26;
  float fVar27;
  float fVar28;
  float fVar29;
  longlong lStack_1d0;
  undefined4 uStack_1c8;
  undefined4 uStack_1c4;
  undefined4 uStack_1c0;
  undefined4 uStack_1bc;
  longlong lStack_1b8;
  longlong lStack_1b0;
  longlong lStack_1a8;
  longlong lStack_1a0;
  undefined4 uStack_198;
  undefined4 uStack_194;
  undefined4 uStack_190;
  undefined4 uStack_18c;
  undefined1 auStack_188 [16];
  ulonglong **ppuStack_178;
  code *pcStack_170;
  undefined8 **ppuStack_168;
  code *pcStack_160;
  longlong lStack_158;
  code *pcStack_150;
  undefined8 **ppuStack_140;
  undefined8 *puStack_138;
  longlong lStack_130;
  undefined8 **ppuStack_128;
  undefined8 *puStack_120;
  undefined8 ***pppuStack_118;
  undefined8 uStack_110;
  undefined8 uStack_108;
  undefined1 uStack_f1;
  undefined8 uStack_f0;
  
  uStack_f0 = 0xfffffffffffffffe;
  ppuStack_140 = param_3;
  if ((ulonglong **)0x1 < param_3) {
    unaff_XMM6_Da = (float)param_3;
    uVar10 = (longlong)param_3 * 8;
    uVar13 = 0;
    if (((ulonglong)param_3 >> 0x3d == 0) && (uVar10 < 0x7ffffffffffffffd)) {
      uVar13 = 4;
      puVar6 = (undefined8 *)func_0x000140613c10(uVar10,4);
      if (puVar6 != (undefined8 *)0x0) {
        pppuStack_118 = (undefined8 ***)0x0;
        unaff_R13 = *(ulonglong *)(param_2 + 0x10);
        unaff_RSI = param_2;
        unaff_RDI = param_1;
        ppuStack_128 = param_3;
        puStack_120 = puVar6;
        if (unaff_R13 == 0) {
          uStack_f1 = 1;
          FUN_1428d9518(0,0,&UNK_142c63930);
          goto LAB_14146ae77;
        }
        goto LAB_14146a97f;
      }
    }
    FUN_1428d8fe3(uVar13,uVar10,&UNK_142c63918);
LAB_14146ae04:
    uStack_f1 = 1;
    FUN_1428d9310(&UNK_142c63a38);
LAB_14146ae77:
                    /* WARNING: Does not return */
    pcVar5 = (code *)invalidInstructionException();
    (*pcVar5)();
  }
  puVar6 = (undefined8 *)FUN_1428d9430(&UNK_142c638e0,0x1f);
LAB_14146a97f:
  fVar23 = _DAT_142925904 / (unaff_XMM6_Da + _DAT_14295a0e4);
  puVar4 = *(undefined8 **)(unaff_RSI + 8);
  auStack_188 = ZEXT416((uint)(_DAT_14295a0e8 * fVar23 + _DAT_142925904));
  uVar13 = *puVar4;
  fVar24 = (float)uVar13;
  fVar25 = (float)((ulonglong)uVar13 >> 0x20);
  *puVar6 = uVar13;
  pppuStack_118 = (undefined8 ***)0x1;
  puStack_138 = puVar4 + unaff_R13;
  if (*(ulonglong *)(unaff_RSI + 0x28) < unaff_R13) {
    unaff_R13 = *(ulonglong *)(unaff_RSI + 0x28);
  }
  lVar7 = *(longlong *)(unaff_RSI + 0x20);
  lStack_130 = -unaff_R13;
  fVar27 = 0.0;
  fVar22 = 0.0;
  fVar15 = 0.0;
  fVar21 = 0.0;
  pppuVar11 = (undefined8 ***)0x1;
  uVar10 = (ulonglong)(unaff_R13 != 0);
  if (uVar10 < unaff_R13) {
    lVar9 = 0;
    do {
      if (uVar10 - 1 == lVar9) {
        uVar10 = uVar10 - lVar9;
        uVar12 = uVar10 + 1;
        fVar17 = fVar24;
        fVar26 = fVar25;
        goto LAB_14146aa2a;
      }
      lVar9 = lVar9 + -1;
    } while (lStack_130 + uVar10 != lVar9);
  }
LAB_14146ac3a:
  if (puVar6 + ((longlong)pppuVar11 - 1) == (undefined8 *)0x0) {
    uStack_f1 = 1;
    FUN_1428d9310(&UNK_142c63948);
    goto LAB_14146ae77;
  }
  if (puStack_138 + -1 == (undefined8 *)0x0) {
    uStack_f1 = 1;
    FUN_1428d9310(&UNK_142c63960);
    goto LAB_14146ae77;
  }
  uVar13 = puStack_138[-1];
  fVar21 = (float)((ulonglong)uVar13 >> 0x20);
  uVar3 = puVar6[(longlong)pppuVar11 - 1];
  fVar23 = (float)uVar13 - (float)uVar3;
  fVar15 = fVar21 - (float)((ulonglong)uVar3 >> 0x20);
  fVar23 = SQRT(fVar15 * fVar15 + fVar23 * fVar23);
  if (fVar23 <= 0.0) {
    if (fVar22 <= fVar23) goto LAB_14146ad39;
    if (pppuVar11 == (undefined8 ***)ppuStack_128) {
      uStack_f1 = 1;
      FUN_140b81f90(&ppuStack_128,&UNK_142c63978);
      puVar6 = puStack_120;
    }
    puVar6[(longlong)pppuVar11] = CONCAT44(fVar25,fVar24);
    ppuVar1 = (ulonglong **)((longlong)pppuVar11 + 1);
    fVar24 = fVar24 - (float)uVar13;
    fVar25 = fVar25 - fVar21;
    pppuStack_118 = (undefined8 ***)ppuVar1;
    if (SQRT(fVar25 * fVar25 + fVar24 * fVar24) <= 0.0) {
      uStack_f1 = 1;
      FUN_1428d9430(&UNK_142c63990,0x47,&UNK_142c639d8);
      goto LAB_14146ae77;
    }
    if (ppuVar1 == ppuStack_128) {
      uStack_f1 = 1;
      FUN_140b81f90(&ppuStack_128,&UNK_142c639f0);
    }
    puVar6 = puStack_120 + (longlong)ppuVar1;
    lVar7 = 2;
  }
  else {
    if (pppuVar11 == (undefined8 ***)ppuStack_128) {
      uStack_f1 = 1;
      FUN_140b81f90(&ppuStack_128,&UNK_142c63a08);
      puVar6 = puStack_120;
    }
    puVar6 = puVar6 + (longlong)pppuVar11;
    lVar7 = 1;
  }
  *puVar6 = uVar13;
  pppuStack_118 = (undefined8 ***)(lVar7 + (longlong)pppuVar11);
LAB_14146ad39:
  ppuStack_168 = pppuStack_118;
  ppuStack_178 = ppuStack_128;
  pcStack_170 = (code *)puStack_120;
  uStack_f1 = 0;
  FUN_141469430(&lStack_1d0,&ppuStack_178,0);
  if (!SBORROW8(0,lStack_1d0)) {
    unaff_RDI[6] = lStack_1a0;
    unaff_RDI[4] = lStack_1b0;
    unaff_RDI[5] = lStack_1a8;
    unaff_RDI[2] = CONCAT44(uStack_1bc,uStack_1c0);
    unaff_RDI[3] = lStack_1b8;
    *unaff_RDI = lStack_1d0;
    unaff_RDI[1] = CONCAT44(uStack_1c4,uStack_1c8);
    return unaff_RDI;
  }
  uStack_198 = uStack_1c8;
  uStack_194 = uStack_1c4;
  uStack_190 = uStack_1c0;
  uStack_18c = uStack_1bc;
  ppuStack_178 = (ulonglong **)&ppuStack_140;
  pcStack_170 = FUN_140d8d6c0;
  ppuStack_168 = (undefined8 **)&uStack_198;
  pcStack_160 = FUN_14146c6d0;
  pcStack_150 = FUN_1412f74c0;
  ppuStack_128 = (undefined8 **)&UNK_142c63b18;
  puStack_120 = (undefined8 *)0x3;
  uStack_108 = 0;
  uStack_110 = 3;
  lStack_158 = unaff_RSI;
  pppuStack_118 = &ppuStack_178;
  FUN_1428d9390(&ppuStack_128,&UNK_142c63b48);
  pcVar5 = (code *)swi(3);
  plVar8 = (longlong *)(*pcVar5)();
  return plVar8;
LAB_14146aa2a:
  fVar28 = (float)puVar4[uVar10];
  fVar29 = (float)((ulonglong)puVar4[uVar10] >> 0x20);
  fVar2 = *(float *)(lVar7 + uVar10 * 4);
  fVar27 = fVar27 + (fVar2 - fVar15);
  fVar14 = (float)auStack_188._0_4_;
  if (fVar27 <= (float)auStack_188._0_4_) {
    fVar14 = fVar27;
  }
  fVar27 = (float)(-(uint)NAN(fVar27) & auStack_188._0_4_ | ~-(uint)NAN(fVar27) & (uint)fVar14);
LAB_14146aa70:
  if (fVar23 <= fVar27 - fVar21) {
    fVar21 = fVar21 + fVar23;
    uStack_f1 = 1;
    fVar14 = (float)func_0x000140c904a0(fVar15);
    pppuVar11 = pppuStack_118;
    if ((pppuStack_118 != (undefined8 ***)0x0) &&
       (puStack_120 + ((longlong)pppuStack_118 - 1) != (undefined8 *)0x0)) {
      uVar13 = puStack_120[(longlong)pppuStack_118 - 1];
      fVar19 = fVar28 - (float)uVar13;
      fVar20 = fVar29 - (float)((ulonglong)uVar13 >> 0x20);
      if (SQRT(fVar20 * fVar20 + fVar19 * fVar19) <= fVar22) goto code_r0x00014146aadf;
      fVar24 = 0.0;
      if (0.0 <= fVar14) {
        fVar24 = fVar14;
      }
      fVar25 = _DAT_142925904;
      if (fVar24 <= _DAT_142925904) {
        fVar25 = fVar24;
      }
      fVar24 = (_DAT_142925904 - fVar25) * fVar17 + fVar25 * fVar28;
      fVar25 = (_DAT_142925904 - fVar25) * fVar26 + fVar25 * fVar29;
      if (pppuStack_118 == (undefined8 ***)ppuStack_128) {
        uStack_f1 = 1;
        FUN_140b81f90(&ppuStack_128);
      }
      goto LAB_14146ab54;
    }
    goto LAB_14146ae04;
  }
  if ((pppuVar11 == (undefined8 ***)0x0) ||
     (puStack_120 + ((longlong)pppuVar11 - 1) == (undefined8 *)0x0)) {
    uStack_f1 = 1;
    FUN_1428d9310(&UNK_142c63a20);
    goto LAB_14146ae77;
  }
  uVar13 = puStack_120[(longlong)pppuVar11 - 1];
  fVar15 = fVar28 - (float)uVar13;
  fVar17 = fVar29 - (float)((ulonglong)uVar13 >> 0x20);
  fVar17 = SQRT(fVar17 * fVar17 + fVar15 * fVar15);
  fVar15 = fVar17;
  if (fVar17 <= fVar22) {
    fVar15 = fVar22;
  }
  uVar16 = (int)((uint)(fVar22 < fVar17) << 0x1f) >> 0x1f;
  uVar18 = (int)((uint)(fVar22 < fVar17) << 0x1f) >> 0x1f;
  fVar24 = (float)(~uVar16 & (uint)fVar24 | (uint)fVar28 & uVar16);
  fVar25 = (float)(~uVar18 & (uint)fVar25 | (uint)fVar29 & uVar18);
  puVar6 = puStack_120;
  fVar22 = fVar15;
  if (unaff_R13 <= uVar12) goto LAB_14146ac3a;
  uVar10 = uVar12;
  uVar12 = (uVar12 < unaff_R13) + uVar12;
  fVar15 = fVar2;
  fVar17 = fVar28;
  fVar26 = fVar29;
  goto LAB_14146aa2a;
code_r0x00014146aadf:
  if (0.0 < fVar22) {
    if (pppuStack_118 == (undefined8 ***)ppuStack_128) {
      uStack_f1 = 1;
      FUN_140b81f90(&ppuStack_128);
    }
LAB_14146ab54:
    puStack_120[(longlong)pppuVar11] = CONCAT44(fVar25,fVar24);
    pppuVar11 = (undefined8 ***)((longlong)pppuVar11 + 1);
    fVar22 = 0.0;
    pppuStack_118 = pppuVar11;
  }
  goto LAB_14146aa70;
}

