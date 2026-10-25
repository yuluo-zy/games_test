
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

undefined8 FUN_140152b00(undefined8 param_1,longlong param_2,float param_3,undefined4 param_4)

{
  longlong lVar1;
  float fVar2;
  float fVar3;
  undefined8 uVar4;
  ulonglong uVar5;
  undefined1 auVar6 [16];
  code *pcVar7;
  unkuint9 Var8;
  ulonglong uVar9;
  longlong lVar10;
  float *pfVar11;
  undefined8 uVar12;
  undefined8 *puVar13;
  ulonglong uVar14;
  undefined8 *puVar15;
  float fVar16;
  float fVar17;
  float fVar18;
  undefined4 uVar19;
  undefined4 uVar20;
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
  undefined8 uStack_178;
  undefined8 *puStack_168;
  undefined8 *puStack_160;
  undefined8 *puStack_158;
  longlong lStack_150;
  ulonglong uStack_148;
  ulonglong uStack_140;
  ulonglong uStack_138;
  float *pfStack_130;
  undefined8 uStack_128;
  undefined8 uStack_120;
  undefined4 uStack_118;
  float fStack_114;
  undefined8 *puStack_110;
  undefined8 *puStack_108;
  undefined8 *puStack_100;
  undefined1 uStack_f1;
  undefined8 uStack_f0;
  
  uStack_f0 = 0xfffffffffffffffe;
  uVar5 = *(ulonglong *)(param_2 + 0x10);
  if (uVar5 < 2) {
    FUN_1428d9430(&UNK_14295a3b0,0x28,&UNK_14295a3d8);
LAB_1401531f4:
    FUN_1428d9430(&UNK_14295a3f0,0x27,&UNK_14295a418);
LAB_14015320c:
    FUN_1428d9430(&UNK_14295a430,0x24,&UNK_14295a458);
  }
  else {
    if ((param_3 == 0.0) && (!NAN(param_3))) goto LAB_1401531f4;
    fVar16 = *(float *)(param_2 + 0x30);
    if ((fVar16 == 0.0) && (!NAN(fVar16))) goto LAB_14015320c;
    if (fVar16 <= _DAT_14295a0d8) {
      fVar16 = _DAT_14295a0d8;
    }
    fVar21 = param_3 / fVar16;
    if (_DAT_142925904 <= param_3 / fVar16) {
      fVar21 = _DAT_142925904;
    }
    uStack_118 = param_4;
    fVar16 = (float)func_0x000142923fa0(_DAT_142925904 / fVar21);
    uVar9 = 0;
    if (0.0 <= fVar16) {
      uVar9 = (longlong)(fVar16 - _DAT_14295a0dc) & (longlong)fVar16 >> 0x3f | (longlong)fVar16;
    }
    uVar14 = 0xffffffffffffffff;
    if (fVar16 <= _DAT_14295a0e0) {
      uVar14 = uVar9;
    }
    puVar15 = (undefined8 *)(uVar14 + 1 + (ulonglong)(uVar14 == 0));
    if ((undefined8 *)0x1 < puVar15) {
      Var8 = ZEXT89(puVar15);
      uVar12 = 0;
      auVar6._8_8_ = 0;
      auVar6._0_8_ = puVar15;
      uVar9 = SUB168(auVar6 * ZEXT816(0xc),0);
      if ((SUB168(auVar6 * ZEXT816(0xc),8) == 0) && (uVar9 < 0x7ffffffffffffffd)) {
        if (uVar9 != 0) {
          uVar12 = 4;
          puStack_108 = (undefined8 *)func_0x000140613c10(uVar9,4);
          if (puStack_108 != (undefined8 *)0x0) goto LAB_140152c92;
          goto LAB_140152c78;
        }
      }
      else {
LAB_140152c78:
        FUN_1428d8fe3(uVar12,uVar9,&UNK_14295a4c0);
      }
      puStack_108 = (undefined8 *)0x4;
      puVar15 = (undefined8 *)0x0;
LAB_140152c92:
      fVar28 = _DAT_142925904 / ((float)(unkint9)Var8 + _DAT_14295a0e4);
      puStack_168 = *(undefined8 **)(param_2 + 8);
      uStack_128 = *puStack_168;
      fVar16 = *(float *)(puStack_168 + 1);
      fVar21 = _DAT_14295a0e8 * fVar28 + _DAT_142925904;
      *puStack_108 = uStack_128;
      *(float *)(puStack_108 + 1) = fVar16;
      puStack_100 = (undefined8 *)0x1;
      lVar10 = (longlong)puStack_168 + uVar5 * 0xc;
      puStack_158 = *(undefined8 **)(param_2 + 0x20);
      uStack_140 = *(ulonglong *)(param_2 + 0x28);
      lStack_150 = (longlong)puStack_158 + uStack_140 * 4;
      if (uVar5 <= uStack_140) {
        uStack_140 = uVar5;
      }
      uStack_148 = 0;
      pfStack_130 = (float *)0x1;
      puVar13 = (undefined8 *)0x1;
      fVar29 = 0.0;
      fVar27 = 0.0;
      fVar24 = 0.0;
      uStack_120 = 0;
      fVar23 = 0.0;
      uStack_178 = uStack_128;
      puStack_160 = (undefined8 *)lVar10;
      uStack_138 = uVar5;
      fStack_114 = fVar16;
      puStack_110 = puVar15;
      do {
        pfVar11 = pfStack_130;
        if (pfStack_130 == (float *)0x0) {
          if (uStack_148 < uStack_140) {
            uVar5 = uStack_148 + 1;
            puVar15 = (undefined8 *)(uStack_148 * 0xc + (longlong)puStack_168);
            pfVar11 = (float *)(uStack_148 * 4 + (longlong)puStack_158);
            uStack_148 = uVar5;
            if (puVar15 != (undefined8 *)0x0) goto LAB_140152e0b;
          }
LAB_14015300d:
          lVar1 = (longlong)puStack_108 + (longlong)puVar13 * 0xc;
          if (lVar1 == 0xc) {
            uStack_f1 = 1;
            FUN_1428d9310(&UNK_14295a4f0);
            goto LAB_14015328e;
          }
          if (lVar10 == 0xc) {
            uStack_f1 = 1;
            FUN_1428d9310(&UNK_14295a508);
            goto LAB_14015328e;
          }
          uVar12 = *(undefined8 *)(lVar10 + -0xc);
          fVar23 = (float)((ulonglong)uVar12 >> 0x20);
          fVar21 = *(float *)(lVar10 + -4);
          fVar28 = fVar21 - *(float *)(lVar1 + -4);
          fVar27 = (float)uVar12 - (float)*(undefined8 *)(lVar1 + -0xc);
          fVar24 = fVar23 - (float)((ulonglong)*(undefined8 *)(lVar1 + -0xc) >> 0x20);
          fVar28 = SQRT(fVar28 * fVar28 + fVar24 * fVar24 + fVar27 * fVar27);
          if (fVar28 <= 0.0) {
            if (fVar29 <= fVar28) goto LAB_14015314a;
            if (puVar13 == puStack_110) {
              uStack_f1 = 1;
              FUN_140b9f180(&puStack_110,&UNK_14295a520);
            }
            pfVar11 = (float *)((longlong)puStack_108 + (longlong)puVar13 * 0xc);
            *pfVar11 = (float)uStack_128;
            pfVar11[1] = uStack_128._4_4_;
            *(float *)((longlong)puStack_108 + (longlong)puVar13 * 0xc + 8) = fVar16;
            puVar15 = (undefined8 *)((longlong)puVar13 + 1);
            fVar28 = (float)uStack_128 - (float)uVar12;
            fVar23 = uStack_128._4_4_ - fVar23;
            puStack_100 = puVar15;
            if (SQRT((fVar16 - fVar21) * (fVar16 - fVar21) + fVar23 * fVar23 + fVar28 * fVar28) <=
                0.0) goto LAB_14015323c;
            if (puVar15 == puStack_110) {
              uStack_f1 = 1;
              FUN_140b9f180(&puStack_110,&UNK_14295a598);
            }
            puVar15 = (undefined8 *)((longlong)puVar15 * 0xc + (longlong)puStack_108);
            lVar10 = 2;
          }
          else {
            if (puVar13 == puStack_110) {
              uStack_f1 = 1;
              FUN_140b9f180(&puStack_110,&UNK_14295a5b0);
            }
            puVar15 = (undefined8 *)((longlong)puStack_108 + (longlong)puVar13 * 0xc);
            lVar10 = 1;
          }
          *puVar15 = uVar12;
          *(float *)(puVar15 + 1) = fVar21;
          puStack_100 = (undefined8 *)(lVar10 + (longlong)puVar13);
LAB_14015314a:
          puStack_158 = puStack_100;
          puStack_168 = puStack_110;
          puStack_160 = puStack_108;
          uStack_f1 = 0;
          FUN_140153640(param_1,&puStack_168,uStack_118);
          return param_1;
        }
        pfStack_130 = (float *)0x0;
        puVar15 = (undefined8 *)FUN_140152810(&puStack_168);
        if (puVar15 == (undefined8 *)0x0) goto LAB_14015300d;
LAB_140152e0b:
        uVar12 = *puVar15;
        fVar17 = (float)uVar12;
        fVar18 = (float)((ulonglong)uVar12 >> 0x20);
        fVar2 = *(float *)(puVar15 + 1);
        fVar3 = *pfVar11;
        fVar24 = fVar24 + (fVar3 - fVar23);
        fVar30 = fVar21;
        if (fVar24 <= fVar21) {
          fVar30 = fVar24;
        }
        fVar24 = (float)(-(uint)NAN(fVar24) & (uint)fVar21 | ~-(uint)NAN(fVar24) & (uint)fVar30);
        fVar30 = fVar29;
LAB_140152e50:
        if (fVar28 <= fVar24 - fVar27) {
          fVar27 = fVar27 + fVar28;
          uStack_f1 = 1;
          fVar29 = (float)func_0x000140c904a0(fVar23,fVar3);
          puVar13 = puStack_100;
          if ((puStack_100 != (undefined8 *)0x0) &&
             ((longlong)puStack_108 + (longlong)puStack_100 * 0xc != 0xc)) {
            fVar22 = fVar2 - *(float *)((longlong)puStack_108 + (longlong)puStack_100 * 0xc + -4);
            uVar4 = *(undefined8 *)((longlong)puStack_108 + (longlong)puStack_100 * 0xc + -0xc);
            fVar25 = fVar17 - (float)uVar4;
            fVar26 = fVar18 - (float)((ulonglong)uVar4 >> 0x20);
            if (SQRT(fVar22 * fVar22 + fVar26 * fVar26 + fVar25 * fVar25) <= fVar30)
            goto code_r0x000140152edd;
            fVar16 = 0.0;
            if (0.0 <= fVar29) {
              fVar16 = fVar29;
            }
            fVar29 = _DAT_142925904;
            if (fVar16 <= _DAT_142925904) {
              fVar29 = fVar16;
            }
            fVar30 = _DAT_142925904 - fVar29;
            fVar16 = fStack_114 * fVar30 + fVar2 * fVar29;
            uStack_128 = CONCAT44(fVar30 * uStack_178._4_4_ + fVar29 * fVar18,
                                  fVar30 * (float)uStack_178 + fVar29 * fVar17);
            uStack_120 = 0;
            if (puStack_100 == puStack_110) {
              uStack_f1 = 1;
              FUN_140b9f180(&puStack_110,&UNK_14295a610);
            }
            goto LAB_140152f60;
          }
          uStack_f1 = 1;
          FUN_1428d9310(&UNK_14295a5e0);
          goto LAB_14015328e;
        }
        if ((puVar13 == (undefined8 *)0x0) ||
           ((longlong)puStack_108 + (longlong)puVar13 * 0xc == 0xc)) goto LAB_14015325a;
        fVar29 = fVar2 - *(float *)((longlong)puStack_108 + (longlong)puVar13 * 0xc + -4);
        uVar4 = *(undefined8 *)((longlong)puStack_108 + (longlong)puVar13 * 0xc + -0xc);
        fVar23 = fVar17 - (float)uVar4;
        fVar22 = fVar18 - (float)((ulonglong)uVar4 >> 0x20);
        fVar29 = SQRT(fVar29 * fVar29 + fVar22 * fVar22 + fVar23 * fVar23);
        uVar19 = 0;
        uVar20 = 0;
        if (fVar29 <= fVar30) {
          fVar17 = (float)uStack_128;
          fVar18 = uStack_128._4_4_;
          uVar19 = (undefined4)uStack_120;
          uVar20 = uStack_120._4_4_;
        }
        fVar16 = (float)(~-(uint)(fVar30 < fVar29) & (uint)fVar16 |
                        -(uint)(fVar30 < fVar29) & (uint)fVar2);
        if (fVar29 <= fVar30) {
          fVar29 = fVar30;
        }
        uStack_128 = CONCAT44(fVar18,fVar17);
        uStack_120 = CONCAT44(uVar20,uVar19);
        fVar23 = fVar3;
        uStack_178 = uVar12;
        fStack_114 = fVar2;
      } while( true );
    }
  }
  FUN_1428d9430(&UNK_14295a488,0x1f,&UNK_14295a4a8);
LAB_14015323c:
  uStack_f1 = 1;
  FUN_1428d9430(&UNK_14295a538,0x47,&UNK_14295a580);
LAB_14015328e:
                    /* WARNING: Does not return */
  pcVar7 = (code *)invalidInstructionException();
  (*pcVar7)();
code_r0x000140152edd:
  if (0.0 < fVar30) {
    if (puStack_100 == puStack_110) {
      uStack_f1 = 1;
      FUN_140b9f180(&puStack_110,&UNK_14295a5f8);
    }
LAB_140152f60:
    *(undefined8 *)((longlong)puStack_108 + (longlong)puVar13 * 0xc) = uStack_128;
    *(float *)((longlong)puStack_108 + (longlong)puVar13 * 0xc + 8) = fVar16;
    puVar13 = (undefined8 *)((longlong)puVar13 + 1);
    fVar30 = 0.0;
    puStack_100 = puVar13;
  }
  goto LAB_140152e50;
LAB_14015325a:
  uStack_f1 = 1;
  FUN_1428d9310(&UNK_14295a5c8);
  goto LAB_14015328e;
}

