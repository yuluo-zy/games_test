
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

longlong *
FUN_141468880(longlong *param_1,longlong *param_2,longlong param_3,longlong param_4,
             ulonglong param_5)

{
  ulonglong uVar1;
  undefined8 *puVar2;
  float *pfVar3;
  undefined8 uVar4;
  undefined8 *puVar5;
  longlong lVar6;
  code *pcVar7;
  undefined1 auVar8 [16];
  float fVar9;
  float fVar10;
  float fVar11;
  float fVar12;
  longlong lVar13;
  ulonglong uVar14;
  undefined1 uVar15;
  longlong lVar16;
  longlong lVar17;
  longlong lVar18;
  ulonglong uVar19;
  ulonglong uVar20;
  float fVar21;
  float fVar22;
  float fVar23;
  float fVar24;
  float fVar25;
  float fVar26;
  float fVar27;
  float fVar28;
  float fVar29;
  float fVar30;
  float fVar31;
  float fVar32;
  float fVar33;
  float fVar34;
  float fVar35;
  float fVar36;
  undefined1 auVar37 [16];
  undefined1 auVar38 [16];
  float fVar39;
  float fVar40;
  undefined1 auVar41 [16];
  undefined1 auVar42 [16];
  
  fVar12 = _UNK_14292593c;
  fVar11 = _UNK_142925938;
  fVar10 = _UNK_142925934;
  fVar9 = _DAT_142925930;
  fVar25 = _DAT_142925904;
  fVar24 = _UNK_1429258ec;
  fVar22 = _UNK_1429258e8;
  fVar31 = _UNK_1429258e4;
  fVar29 = _DAT_1429258e0;
  if (param_2[2] != param_5) {
    FUN_1428d9430(&UNK_142c63608,0x35,&UNK_142c63640);
LAB_141468dd5:
                    /* WARNING: Does not return */
    pcVar7 = (code *)invalidInstructionException();
    (*pcVar7)();
  }
  puVar5 = (undefined8 *)param_2[1];
  if (2 < param_5 && param_3 != 0) {
    fVar21 = (float)*puVar5;
    fVar23 = (float)((ulonglong)*puVar5 >> 0x20);
    uVar1 = param_5 - 1;
    uVar14 = param_5 - 3;
    if (uVar1 < param_5 - 3) {
      uVar14 = uVar1;
    }
    puVar2 = puVar5 + 1;
    lVar13 = 0;
    do {
      fVar35 = *(float *)(puVar5 + 1);
      fVar26 = *(float *)((longlong)puVar5 + 0xc);
      fVar32 = 0.0;
      fVar33 = 0.0;
      uVar19 = 1;
      fVar28 = fVar21;
      fVar30 = fVar23;
      if (4 < uVar14 + 1 &&
          (((puVar2 <= puVar5 + uVar14 + 1 && uVar14 >> 0x3d == 0) &&
           ((longlong)puVar5 + 0xcU <= (longlong)puVar5 + uVar14 * 8 + 0xc &&
           puVar5 + 2 <= puVar5 + uVar14 + 2)) &&
          (longlong)puVar5 + 0x14U <= (longlong)puVar5 + uVar14 * 8 + 0x14)) {
        uVar20 = 0;
        fVar39 = fVar23;
        fVar27 = fVar21;
        do {
          uVar19 = *(ulonglong *)(param_4 + 0x10 + uVar20 * 8);
          if ((longlong)uVar19 < 0) {
            fVar36 = (float)uVar19;
            uVar19 = *(ulonglong *)(param_4 + 8 + uVar20 * 8);
            if ((longlong)uVar19 < 0) goto LAB_141468b28;
LAB_141468a20:
            fVar40 = (float)(longlong)uVar19;
          }
          else {
            fVar36 = (float)(longlong)uVar19;
            uVar19 = *(ulonglong *)(param_4 + 8 + uVar20 * 8);
            if (-1 < (longlong)uVar19) goto LAB_141468a20;
LAB_141468b28:
            fVar40 = (float)uVar19;
          }
          pfVar3 = (float *)(puVar5 + 2 + uVar20);
          fVar28 = *pfVar3;
          fVar30 = pfVar3[1];
          fVar32 = pfVar3[2];
          fVar33 = pfVar3[3];
          auVar41._0_4_ = fVar40 * fVar35 + fVar27 * fVar29 + fVar28 * fVar29;
          auVar42._4_4_ = fVar36 * fVar28 + fVar35 * fVar31 + fVar32 * fVar31;
          auVar42._0_4_ = auVar41._0_4_;
          auVar42._8_4_ = auVar42._4_4_;
          auVar42._12_4_ = fVar30 * fVar36 + fVar26 * fVar31 + fVar33 * fVar31;
          auVar41._8_8_ = auVar42._8_8_;
          auVar41._4_4_ = fVar26 * fVar40 + fVar39 * fVar29 + fVar30 * fVar29;
          auVar38._4_4_ = fVar40 + fVar10;
          auVar38._0_4_ = fVar40 + fVar9;
          auVar38._8_4_ = fVar36 + fVar11;
          auVar38._12_4_ = fVar36 + fVar12;
          auVar38 = divps(auVar41,auVar38);
          *(undefined1 (*) [16])(puVar5 + uVar20 + 1) = auVar38;
          uVar20 = uVar20 + 2;
          uVar19 = uVar14 | 1;
          fVar35 = fVar32;
          fVar39 = fVar30;
          fVar26 = fVar33;
          fVar27 = fVar28;
        } while ((uVar14 & 0xfffffffffffffffe) != uVar20);
      }
      lVar13 = lVar13 + 1;
      fVar39 = fVar32;
      fVar27 = fVar33;
      do {
        if (param_5 == uVar19) {
          FUN_1428d9518(param_5,param_5,&UNK_142c63cc0);
          goto LAB_141468dd5;
        }
        fVar34 = (float)*(ulonglong *)(param_4 + uVar19 * 8);
        fVar36 = *(float *)(puVar2 + uVar19);
        fVar40 = *(float *)((longlong)(puVar2 + uVar19) + 4);
        auVar37._0_4_ = fVar34 * fVar35 + fVar28 * fVar29 + fVar36 * fVar29;
        auVar37._4_4_ = fVar34 * fVar26 + fVar30 * fVar31 + fVar40 * fVar31;
        auVar37._8_4_ = fVar39 * 0.0 + fVar32 * fVar22 + fVar22 * 0.0;
        auVar37._12_4_ = fVar27 * 0.0 + fVar33 * fVar24 + fVar24 * 0.0;
        auVar8._4_4_ = fVar34 + fVar25;
        auVar8._0_4_ = fVar34 + fVar25;
        auVar8._8_8_ = 0;
        auVar38 = divps(auVar37,auVar8);
        puVar5[uVar19] = auVar38._0_8_;
        uVar19 = uVar19 + 1;
        fVar28 = fVar35;
        fVar30 = fVar26;
        fVar32 = fVar39;
        fVar33 = fVar27;
        fVar39 = 0.0;
        fVar27 = 0.0;
        fVar35 = fVar36;
        fVar26 = fVar40;
      } while (uVar1 != uVar19);
    } while (lVar13 != param_3);
  }
  param_2[5] = 0;
  if (param_5 < 2) {
    uVar15 = 0;
  }
  else {
    fVar29 = (float)*puVar5;
    fVar31 = (float)((ulonglong)*puVar5 >> 0x20);
    if ((ulonglong)param_2[3] < param_5) {
      FUN_1428a9a60(param_2 + 3,0,param_5,4,4);
      lVar13 = param_2[5];
    }
    else {
      lVar13 = 0;
    }
    lVar6 = param_2[4];
    lVar17 = lVar6 + lVar13 * 4;
    fVar22 = 0.0;
    lVar16 = 0;
    lVar18 = 0;
    do {
      uVar4 = *(undefined8 *)((longlong)puVar5 + lVar18 * 2);
      fVar24 = (float)uVar4;
      fVar25 = (float)((ulonglong)uVar4 >> 0x20);
      fVar29 = fVar29 - fVar24;
      fVar31 = fVar31 - fVar25;
      fVar22 = SQRT(fVar31 * fVar31 + fVar29 * fVar29) + fVar22;
      *(float *)(lVar17 + lVar18) = fVar22;
      uVar4 = *(undefined8 *)((longlong)puVar5 + lVar18 * 2 + 8);
      fVar29 = (float)uVar4;
      fVar31 = (float)((ulonglong)uVar4 >> 0x20);
      fVar24 = fVar24 - fVar29;
      fVar25 = fVar25 - fVar31;
      fVar22 = SQRT(fVar25 * fVar25 + fVar24 * fVar24) + fVar22;
      *(float *)(lVar17 + 4 + lVar18) = fVar22;
      lVar18 = lVar18 + 8;
      lVar16 = lVar16 + -2;
    } while (-lVar16 != (param_5 & 0xfffffffffffffffe));
    if ((param_5 & 1) == 0) {
      lVar13 = lVar13 - lVar16;
    }
    else {
      uVar4 = *(undefined8 *)((longlong)puVar5 + lVar18 * 2);
      fVar29 = fVar29 - (float)uVar4;
      fVar31 = fVar31 - (float)((ulonglong)uVar4 >> 0x20);
      fVar22 = fVar22 + SQRT(fVar31 * fVar31 + fVar29 * fVar29);
      *(float *)(lVar18 + lVar17) = fVar22;
      lVar13 = (lVar13 - lVar16) + 1;
    }
    param_2[5] = lVar13;
    uVar15 = 1;
    if ((fVar22 != 0.0) || (NAN(fVar22))) {
      lVar17 = 0;
      do {
        if (lVar13 * 4 == lVar17) {
          *(float *)(param_2 + 6) = fVar22;
          lVar13 = *param_2;
          lVar17 = param_2[1];
          lVar6 = param_2[2];
          lVar16 = param_2[3];
          lVar18 = param_2[5];
          param_1[4] = param_2[4];
          param_1[5] = lVar18;
          param_1[2] = lVar6;
          param_1[3] = lVar16;
          *param_1 = lVar13;
          param_1[1] = lVar17;
          param_1[6] = param_2[6];
          return param_1;
        }
        fVar29 = *(float *)(lVar6 + lVar17) / fVar22;
        *(float *)(lVar6 + lVar17) = fVar29;
        lVar17 = lVar17 + 4;
      } while ((uint)ABS(fVar29) < 0x7f800000);
    }
  }
  *(undefined1 *)(param_1 + 1) = uVar15;
  *(char *)((longlong)param_1 + 9) = (char)param_5;
  *param_1 = -0x8000000000000000;
  if (*param_2 != 0) {
    func_0x000140613c20(param_2[1],*param_2 << 3,4);
  }
  if (param_2[3] != 0) {
    func_0x000140613c20(param_2[4],param_2[3] << 2,4);
  }
  return param_1;
}

