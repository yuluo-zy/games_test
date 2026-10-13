
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

undefined8 * FUN_1411f8ed0(undefined8 *param_1,longlong param_2,float param_3)

{
  longlong lVar1;
  float fVar2;
  float fVar3;
  undefined8 uVar4;
  ulonglong uVar5;
  code *pcVar6;
  undefined1 auVar7 [16];
  unkuint9 Var8;
  ulonglong uVar9;
  undefined8 *puVar10;
  longlong lVar11;
  float *pfVar12;
  ulonglong uVar13;
  undefined8 uVar14;
  float fVar15;
  float fVar16;
  float fVar17;
  undefined4 uVar18;
  undefined4 uVar19;
  float fVar20;
  float fVar21;
  float fVar22;
  float fVar23;
  float fVar24;
  float fVar25;
  float fVar26;
  float fVar27;
  float fVar28;
  float fVar29;
  undefined8 uStack_198;
  undefined4 uStack_188;
  undefined4 uStack_184;
  undefined4 uStack_180;
  undefined4 uStack_17c;
  ulonglong uStack_178;
  undefined8 *puStack_168;
  undefined8 uStack_160;
  undefined8 uStack_158;
  longlong lStack_150;
  ulonglong uStack_148;
  ulonglong uStack_140;
  ulonglong uStack_138;
  float *pfStack_130;
  undefined8 uStack_128;
  undefined8 uStack_120;
  float fStack_114;
  undefined8 uStack_110;
  undefined8 uStack_108;
  ulonglong uStack_100;
  undefined1 uStack_f1;
  undefined8 uStack_f0;
  
  uStack_f0 = 0xfffffffffffffffe;
  uVar5 = *(ulonglong *)(param_2 + 0x10);
  if (uVar5 < 2) {
    FUN_1428d9430(&UNK_142c18e90,0x28,&UNK_142c18eb8);
LAB_1411f95e9:
    FUN_1428d9430(&UNK_142c18ed0,0x27,&UNK_142c18ef8);
LAB_1411f9601:
    FUN_1428d9430(&UNK_142c18f10,0x24,&UNK_142c18f38);
  }
  else {
    if ((param_3 == 0.0) && (!NAN(param_3))) goto LAB_1411f95e9;
    fVar15 = *(float *)(param_2 + 0x30);
    if ((fVar15 == 0.0) && (!NAN(fVar15))) goto LAB_1411f9601;
    if (fVar15 <= _DAT_14295a0d8) {
      fVar15 = _DAT_14295a0d8;
    }
    fVar20 = param_3 / fVar15;
    if (_DAT_142925904 <= param_3 / fVar15) {
      fVar20 = _DAT_142925904;
    }
    fVar15 = (float)func_0x000142923fa0(_DAT_142925904 / fVar20);
    uVar13 = 0;
    if (0.0 <= fVar15) {
      uVar13 = (longlong)(fVar15 - _DAT_14295a0dc) & (longlong)fVar15 >> 0x3f | (longlong)fVar15;
    }
    uVar9 = 0xffffffffffffffff;
    if (fVar15 <= _DAT_14295a0e0) {
      uVar9 = uVar13;
    }
    uVar13 = uVar9 + 1 + (ulonglong)(uVar9 == 0);
    if (1 < uVar13) {
      Var8 = (unkuint9)uVar13;
      uVar14 = 0;
      auVar7._8_8_ = 0;
      auVar7._0_8_ = uVar13;
      uVar9 = SUB168(auVar7 * ZEXT816(0xc),0);
      if ((SUB168(auVar7 * ZEXT816(0xc),8) == 0) && (uVar9 < 0x7ffffffffffffffd)) {
        if (uVar9 != 0) {
          uVar14 = 4;
          uStack_108 = (undefined8 *)func_0x000140613c10(uVar9,4);
          if (uStack_108 != (undefined8 *)0x0) goto LAB_1411f9065;
          goto LAB_1411f904c;
        }
      }
      else {
LAB_1411f904c:
        FUN_1428d8fe3(uVar14,uVar9,&UNK_142c190a8);
      }
      uStack_108 = (undefined8 *)0x4;
      uVar13 = 0;
LAB_1411f9065:
      fVar27 = _DAT_142925904 / ((float)(unkint9)Var8 + _DAT_14295a0e4);
      puStack_168 = *(undefined8 **)(param_2 + 8);
      uStack_128 = *puStack_168;
      fVar15 = *(float *)(puStack_168 + 1);
      fVar20 = _DAT_14295a0e8 * fVar27 + _DAT_142925904;
      *uStack_108 = uStack_128;
      *(float *)(uStack_108 + 1) = fVar15;
      uStack_100 = 1;
      lVar11 = (longlong)puStack_168 + uVar5 * 0xc;
      uStack_158 = *(longlong *)(param_2 + 0x20);
      uStack_140 = *(ulonglong *)(param_2 + 0x28);
      lStack_150 = uStack_158 + uStack_140 * 4;
      if (uVar5 <= uStack_140) {
        uStack_140 = uVar5;
      }
      uStack_148 = 0;
      pfStack_130 = (float *)0x1;
      uVar9 = 1;
      fVar28 = 0.0;
      fVar26 = 0.0;
      fVar23 = 0.0;
      uStack_120 = 0;
      fVar22 = 0.0;
      uStack_198 = uStack_128;
      uStack_160 = lVar11;
      uStack_138 = uVar5;
      fStack_114 = fVar15;
      uStack_110 = uVar13;
      do {
        pfVar12 = pfStack_130;
        if (pfStack_130 == (float *)0x0) {
          if (uStack_148 < uStack_140) {
            uVar5 = uStack_148 + 1;
            puVar10 = (undefined8 *)(uStack_148 * 0xc + (longlong)puStack_168);
            pfVar12 = (float *)(uStack_148 * 4 + uStack_158);
            uStack_148 = uVar5;
            if (puVar10 != (undefined8 *)0x0) goto LAB_1411f91db;
          }
LAB_1411f93d6:
          lVar1 = (longlong)uStack_108 + uVar9 * 0xc;
          if (lVar1 == 0xc) {
            uStack_f1 = 1;
            FUN_1428d9310(&UNK_142c190d8);
            goto LAB_1411f9683;
          }
          if (lVar11 == 0xc) {
            uStack_f1 = 1;
            FUN_1428d9310(&UNK_142c190f0);
            goto LAB_1411f9683;
          }
          uVar14 = *(undefined8 *)(lVar11 + -0xc);
          fVar22 = (float)((ulonglong)uVar14 >> 0x20);
          fVar20 = *(float *)(lVar11 + -4);
          fVar27 = fVar20 - *(float *)(lVar1 + -4);
          fVar26 = (float)uVar14 - (float)*(undefined8 *)(lVar1 + -0xc);
          fVar23 = fVar22 - (float)((ulonglong)*(undefined8 *)(lVar1 + -0xc) >> 0x20);
          fVar27 = SQRT(fVar27 * fVar27 + fVar23 * fVar23 + fVar26 * fVar26);
          if (fVar27 <= 0.0) {
            if (fVar28 <= fVar27) goto LAB_1411f950b;
            if (uVar9 == uStack_110) {
              uStack_f1 = 1;
              FUN_140b9f180(&uStack_110,&UNK_142c19108);
            }
            pfVar12 = (float *)((longlong)uStack_108 + uVar9 * 0xc);
            *pfVar12 = (float)uStack_128;
            pfVar12[1] = uStack_128._4_4_;
            *(float *)((longlong)uStack_108 + uVar9 * 0xc + 8) = fVar15;
            uVar5 = uVar9 + 1;
            fVar27 = (float)uStack_128 - (float)uVar14;
            fVar22 = uStack_128._4_4_ - fVar22;
            uStack_100 = uVar5;
            if (SQRT((fVar15 - fVar20) * (fVar15 - fVar20) + fVar22 * fVar22 + fVar27 * fVar27) <=
                0.0) goto LAB_1411f9631;
            if (uVar5 == uStack_110) {
              uStack_f1 = 1;
              FUN_140b9f180(&uStack_110,&UNK_142c19180);
            }
            puVar10 = (undefined8 *)(uVar5 * 0xc + (longlong)uStack_108);
            lVar11 = 2;
          }
          else {
            if (uVar9 == uStack_110) {
              uStack_f1 = 1;
              FUN_140b9f180(&uStack_110,&UNK_142c19198);
            }
            puVar10 = (undefined8 *)((longlong)uStack_108 + uVar9 * 0xc);
            lVar11 = 1;
          }
          *puVar10 = uVar14;
          *(float *)(puVar10 + 1) = fVar20;
          uStack_100 = lVar11 + uVar9;
LAB_1411f950b:
          uStack_178 = uStack_100;
          uStack_188 = (undefined4)uStack_110;
          uStack_184 = uStack_110._4_4_;
          uStack_180 = (undefined4)uStack_108;
          uStack_17c = uStack_108._4_4_;
          uStack_f1 = 0;
          FUN_1411f9be0(&puStack_168,&uStack_188,0);
          if (SBORROW8(0,(longlong)puStack_168)) {
            uStack_188 = (undefined4)uStack_160;
            uStack_184 = uStack_160._4_4_;
            uStack_180 = (undefined4)uStack_158;
            uStack_17c = uStack_158._4_4_;
            FUN_1428d9760(&UNK_142c18bc8,0x2b,&uStack_188,&UNK_142c18ba8,&UNK_142c18f68);
            pcVar6 = (code *)swi(3);
            puVar10 = (undefined8 *)(*pcVar6)();
            return puVar10;
          }
          param_1[6] = uStack_138;
          param_1[4] = uStack_148;
          param_1[5] = uStack_140;
          param_1[2] = uStack_158;
          param_1[3] = lStack_150;
          *param_1 = puStack_168;
          param_1[1] = uStack_160;
          return param_1;
        }
        pfStack_130 = (float *)0x0;
        puVar10 = (undefined8 *)FUN_140152810(&puStack_168);
        if (puVar10 == (undefined8 *)0x0) goto LAB_1411f93d6;
LAB_1411f91db:
        uVar14 = *puVar10;
        fVar16 = (float)uVar14;
        fVar17 = (float)((ulonglong)uVar14 >> 0x20);
        fVar2 = *(float *)(puVar10 + 1);
        fVar3 = *pfVar12;
        fVar23 = fVar23 + (fVar3 - fVar22);
        fVar29 = fVar20;
        if (fVar23 <= fVar20) {
          fVar29 = fVar23;
        }
        fVar23 = (float)(-(uint)NAN(fVar23) & (uint)fVar20 | ~-(uint)NAN(fVar23) & (uint)fVar29);
        fVar29 = fVar28;
LAB_1411f9220:
        if (fVar27 <= fVar23 - fVar26) {
          fVar26 = fVar26 + fVar27;
          uStack_f1 = 1;
          fVar28 = (float)func_0x000140c904a0(fVar22,fVar3);
          uVar9 = uStack_100;
          if ((uStack_100 != 0) && ((longlong)uStack_108 + uStack_100 * 0xc != 0xc)) {
            fVar21 = fVar2 - *(float *)((longlong)uStack_108 + uStack_100 * 0xc + -4);
            uVar4 = *(undefined8 *)((longlong)uStack_108 + uStack_100 * 0xc + -0xc);
            fVar24 = fVar16 - (float)uVar4;
            fVar25 = fVar17 - (float)((ulonglong)uVar4 >> 0x20);
            if (SQRT(fVar21 * fVar21 + fVar25 * fVar25 + fVar24 * fVar24) <= fVar29)
            goto code_r0x0001411f92a6;
            fVar15 = 0.0;
            if (0.0 <= fVar28) {
              fVar15 = fVar28;
            }
            fVar28 = _DAT_142925904;
            if (fVar15 <= _DAT_142925904) {
              fVar28 = fVar15;
            }
            fVar29 = _DAT_142925904 - fVar28;
            fVar15 = fStack_114 * fVar29 + fVar2 * fVar28;
            uStack_128 = CONCAT44(fVar29 * uStack_198._4_4_ + fVar28 * fVar17,
                                  fVar29 * (float)uStack_198 + fVar28 * fVar16);
            uStack_120 = 0;
            if (uStack_100 == uStack_110) {
              uStack_f1 = 1;
              FUN_140b9f180(&uStack_110,&UNK_142c191f8);
            }
            goto LAB_1411f9329;
          }
          uStack_f1 = 1;
          FUN_1428d9310(&UNK_142c191c8);
          goto LAB_1411f9683;
        }
        if ((uVar9 == 0) || ((longlong)uStack_108 + uVar9 * 0xc == 0xc)) goto LAB_1411f964f;
        fVar28 = fVar2 - *(float *)((longlong)uStack_108 + uVar9 * 0xc + -4);
        uVar4 = *(undefined8 *)((longlong)uStack_108 + uVar9 * 0xc + -0xc);
        fVar22 = fVar16 - (float)uVar4;
        fVar21 = fVar17 - (float)((ulonglong)uVar4 >> 0x20);
        fVar28 = SQRT(fVar28 * fVar28 + fVar21 * fVar21 + fVar22 * fVar22);
        uVar18 = 0;
        uVar19 = 0;
        if (fVar28 <= fVar29) {
          fVar16 = (float)uStack_128;
          fVar17 = uStack_128._4_4_;
          uVar18 = (undefined4)uStack_120;
          uVar19 = uStack_120._4_4_;
        }
        fVar15 = (float)(~-(uint)(fVar29 < fVar28) & (uint)fVar15 |
                        -(uint)(fVar29 < fVar28) & (uint)fVar2);
        if (fVar28 <= fVar29) {
          fVar28 = fVar29;
        }
        uStack_128 = CONCAT44(fVar17,fVar16);
        uStack_120 = CONCAT44(uVar19,uVar18);
        fVar22 = fVar3;
        uStack_198 = uVar14;
        fStack_114 = fVar2;
      } while( true );
    }
  }
  FUN_1428d9430(&UNK_142c19070,0x1f,&UNK_142c19090);
LAB_1411f9631:
  uStack_f1 = 1;
  FUN_1428d9430(&UNK_142c19120,0x47,&UNK_142c19168);
LAB_1411f9683:
                    /* WARNING: Does not return */
  pcVar6 = (code *)invalidInstructionException();
  (*pcVar6)();
code_r0x0001411f92a6:
  if (0.0 < fVar29) {
    if (uStack_100 == uStack_110) {
      uStack_f1 = 1;
      FUN_140b9f180(&uStack_110,&UNK_142c191e0);
    }
LAB_1411f9329:
    *(undefined8 *)((longlong)uStack_108 + uVar9 * 0xc) = uStack_128;
    *(float *)((longlong)uStack_108 + uVar9 * 0xc + 8) = fVar15;
    uVar9 = uVar9 + 1;
    fVar29 = 0.0;
    uStack_100 = uVar9;
  }
  goto LAB_1411f9220;
LAB_1411f964f:
  uStack_f1 = 1;
  FUN_1428d9310(&UNK_142c191b0);
  goto LAB_1411f9683;
}

