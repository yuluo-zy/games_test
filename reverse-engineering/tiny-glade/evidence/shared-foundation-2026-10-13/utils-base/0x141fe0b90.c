/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */
ulonglong *
FUN_141fe0b90(ulonglong *param_1,float *param_2,ulonglong param_3,float param_4,float param_5)
{
  float fVar1;
  float fVar2;
  ulonglong uVar3;
  code *pcVar4;
  float fVar5;
  ulonglong uVar6;
  undefined8 *puVar7;
  ulonglong uVar8;
  ulonglong uVar9;
  ulonglong *puVar10;
  ulonglong uVar11;
  float *pfVar12;
  ulonglong uVar13;
  undefined8 uVar14;
  ulonglong uVar15;
  undefined8 *unaff_R13;
  longlong unaff_R14;
  ulonglong uVar16;
  bool bVar17;
  float fVar18;
  float fVar19;
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
  float fVar30;
  float fVar31;
  float fVar32;
  float fVar33;
  float fVar34;
  float fVar35;
  float fVar36;
  float fVar37;
  float fVar38;
  ulonglong uStack_178;
  ulonglong uStack_170;
  ulonglong uStack_168;
  undefined8 *puStack_160;
  undefined8 *puStack_158;
  float *pfStack_150;
  ulonglong uStack_148;
  float *pfStack_140;
  float fStack_138;
  float fStack_134;
  ulonglong uStack_130;
  longlong lStack_128;
  ulonglong uStack_120;
  ulonglong uStack_118;
  undefined8 *puStack_110;
  ulonglong uStack_108;
  ulonglong uStack_100;
  undefined8 *puStack_f8;
  undefined8 uStack_f0;
  uStack_f0 = 0xfffffffffffffffe;
  if (param_3 < 3) {
    if (param_3 == 0) {
      uVar8 = 4;
      puVar7 = (undefined8 *)0x8;
      unaff_R14 = 0;
      uVar15 = 0;
LAB_141fe1316:
      func_0x000142923c00(uVar8,param_2,unaff_R14);
      *param_1 = uVar15;
      param_1[1] = (ulonglong)puVar7;
      param_1[2] = uVar15;
      param_1[3] = param_3;
      param_1[4] = uVar8;
      param_1[5] = param_3;
      return param_1;
    }
    uVar16 = param_3 * 8;
    puVar7 = (undefined8 *)func_0x000140613c10(uVar16,8);
    if (puVar7 == (undefined8 *)0x0) {
      FUN_1428d8fe3(8,uVar16,&UNK_142fcb408);
    }
    else {
      *puVar7 = 0;
      puVar7[1] = 1;
      unaff_R14 = param_3 * 0xc;
      uVar8 = func_0x000140613c10(unaff_R14,4);
      uVar15 = param_3;
      unaff_R13 = puVar7;
      if (uVar8 != 0) goto LAB_141fe1316;
    }
    uStack_100 = uVar16;
    puStack_f8 = unaff_R13;
    FUN_1428d8fe3(4,unaff_R14,&UNK_142fcb470);
  }
  else {
    uStack_130 = 0;
    lStack_128 = 8;
    uStack_120 = 0;
    uStack_118 = 0;
    puStack_110 = (undefined8 *)0x8;
    uStack_108 = 0;
    FUN_140b81df0(&uStack_118,&UNK_142fcb4d0);
    uVar16 = uStack_120;
    *puStack_110 = 0;
    uStack_108 = 1;
    if (uStack_120 == uStack_130) {
      FUN_140562db0(&uStack_130,&UNK_142fcb4e8);
    }
    *(undefined8 *)(lStack_128 + uVar16 * 0x18) = 0;
    *(ulonglong *)(lStack_128 + 8 + uVar16 * 0x18) = param_3 - 1;
    *(undefined1 *)(lStack_128 + 0x10 + uVar16 * 0x18) = 0;
    uStack_120 = uVar16 + 1;
    pfVar12 = param_2 + param_3 * 3;
    if (pfVar12 == (float *)0xc) {
      FUN_1428d9310(&UNK_142fcb500);
    }
    else {
      fVar25 = SQRT((param_2[2] - pfVar12[-1]) * (param_2[2] - pfVar12[-1]) +
                    (param_2[1] - pfVar12[-2]) * (param_2[1] - pfVar12[-2]) +
                    (*param_2 - pfVar12[-3]) * (*param_2 - pfVar12[-3]));
      uStack_100 = CONCAT44(uStack_100._4_4_,
                            (int)CONCAT71((int7)((ulonglong)pfVar12 >> 8),fVar25 < _DAT_142ad4950));
      pfStack_140 = param_2 + 5;
      puStack_f8 = (undefined8 *)CONCAT44(puStack_f8._4_4_,param_5 * param_5);
      fVar33 = param_5 * param_5;
      fVar24 = _DAT_142a7baa0;
      while (uVar16 = uStack_120, uStack_120 != 0) {
        while( true ) {
          uVar15 = uStack_108;
          fVar30 = _DAT_142a7baa0;
          uStack_120 = uVar16 - 1;
          if (*(char *)(lStack_128 + 0x10 + uStack_120 * 0x18) == '\x02') goto LAB_141fe0df0;
          puVar10 = (ulonglong *)(lStack_128 + uStack_120 * 0x18);
          uVar8 = *(ulonglong *)(lStack_128 + 8 + uStack_120 * 0x18);
          uVar3 = *puVar10;
          if (2 < (uVar8 - uVar3) + 1) break;
          if (uStack_108 == uStack_118) {
            FUN_140b81df0(&uStack_118,&UNK_142fcb5a8);
          }
          puStack_110[uVar15] = uVar8;
          uStack_108 = uVar15 + 1;
          uVar16 = uStack_120;
          if (uStack_120 == 0) goto LAB_141fe0df0;
        }
        if (param_3 <= uVar3) {
          FUN_1428d9518(uVar3,param_3,&UNK_142fcb518);
          goto LAB_141fe14f1;
        }
        if (param_3 <= uVar8) {
          FUN_1428d9518(uVar8,param_3,&UNK_142fcb530);
          goto LAB_141fe14f1;
        }
        fStack_138 = fVar25;
        if (uVar8 < uVar3) {
          FUN_1428d9850(uVar3,uVar8,&UNK_142fcb548);
          goto LAB_141fe14f1;
        }
        fVar5 = puStack_f8._0_4_;
        fStack_134 = param_4;
        if (param_2 + uVar3 * 3 + 3 == param_2 + uVar8 * 3 || uVar8 == uVar3) {
          fVar33 = 0.0;
          uVar9 = 0;
        }
        else {
          fVar1 = param_2[uVar3 * 3 + 2];
          fVar31 = (float)*(undefined8 *)(param_2 + uVar8 * 3);
          fVar32 = (float)((ulonglong)*(undefined8 *)(param_2 + uVar8 * 3) >> 0x20);
          fVar2 = param_2[uVar8 * 3 + 2];
          uVar11 = (ulonglong)
                   ((longlong)(param_2 + uVar8 * 3) - (longlong)(param_2 + uVar3 * 3 + 3)) / 0xc;
          fVar18 = (float)*(undefined8 *)(param_2 + uVar3 * 3);
          fVar20 = fVar31 - fVar18;
          fVar19 = (float)((ulonglong)*(undefined8 *)(param_2 + uVar3 * 3) >> 0x20);
          fVar21 = fVar32 - fVar19;
          fVar22 = fVar2 - fVar1;
          fVar23 = fVar22 * fVar22 + fVar21 * fVar21 + fVar20 * fVar20;
          if (fVar23 <= fVar24) {
            fVar23 = fVar24;
          }
          if ((fVar33 != 0.0) || (NAN(fVar33))) {
            pfVar12 = pfStack_140 + uVar3 * 3;
            fVar33 = 0.0;
            uVar9 = 0;
            uVar13 = 1;
            do {
              fVar28 = (float)*(undefined8 *)(pfVar12 + -2);
              fVar29 = (float)((ulonglong)*(undefined8 *)(pfVar12 + -2) >> 0x20);
              fVar24 = *pfVar12;
              fVar36 = fVar28 - fVar18;
              fVar37 = fVar29 - fVar19;
              fVar38 = fVar24 - fVar1;
              fVar35 = (fVar22 * fVar38 + fVar21 * fVar37 + fVar20 * fVar36) / fVar23;
              fVar26 = _DAT_142925904 - fVar35;
              fVar34 = fVar24 - (fVar2 * fVar35 + fVar1 * fVar26);
              fVar27 = fVar28 - (fVar35 * fVar31 + fVar26 * fVar18);
              fVar35 = fVar29 - (fVar35 * fVar32 + fVar26 * fVar19);
              fVar35 = SQRT(fVar34 * fVar34 + fVar35 * fVar35 + fVar27 * fVar27);
              if (0x7f7fffff < (uint)ABS(fVar35)) goto LAB_141fe1446;
              if (((fVar33 < fVar35) &&
                  (puStack_f8._0_4_ <= fVar38 * fVar38 + fVar37 * fVar37 + fVar36 * fVar36)) &&
                 (puStack_f8._0_4_ <=
                  (fVar24 - fVar2) * (fVar24 - fVar2) +
                  (fVar29 - fVar32) * (fVar29 - fVar32) + (fVar28 - fVar31) * (fVar28 - fVar31))) {
                uVar9 = uVar13;
                fVar33 = fVar35;
              }
              pfVar12 = pfVar12 + 3;
              bVar17 = uVar13 != uVar11;
              uVar13 = uVar13 + 1;
            } while (bVar17);
          }
          else {
            pfVar12 = pfStack_140 + uVar3 * 3;
            fVar33 = 0.0;
            uVar9 = 0;
            uVar13 = 1;
            do {
              fVar35 = (float)*(undefined8 *)(pfVar12 + -2);
              fVar24 = (float)((ulonglong)*(undefined8 *)(pfVar12 + -2) >> 0x20);
              fVar27 = ((*pfVar12 - fVar1) * fVar22 +
                       (fVar24 - fVar19) * fVar21 + (fVar35 - fVar18) * fVar20) / fVar23;
              fVar26 = _DAT_142925904 - fVar27;
              fVar24 = fVar24 - (fVar32 * fVar27 + fVar19 * fVar26);
              fVar34 = *pfVar12 - (fVar27 * fVar2 + fVar26 * fVar1);
              fVar35 = fVar35 - (fVar27 * fVar31 + fVar26 * fVar18);
              fVar24 = SQRT(fVar35 * fVar35 + fVar24 * fVar24 + fVar34 * fVar34);
              if (0x7f7fffff < (uint)ABS(fVar24)) goto LAB_141fe1446;
              uVar6 = uVar13;
              if (fVar24 <= fVar33) {
                fVar24 = fVar33;
                uVar6 = uVar9;
              }
              uVar9 = uVar6;
              fVar33 = fVar24;
              pfVar12 = pfVar12 + 3;
              bVar17 = uVar13 != uVar11;
              uVar13 = uVar13 + 1;
            } while (bVar17);
          }
        }
        if ((param_4 < fVar33) || ((fVar25 < fVar33 & (byte)uStack_100) != 0)) {
          uVar8 = uVar9 + uVar3;
          *puVar10 = uVar8;
          *(undefined1 *)(puVar10 + 2) = 0;
          fVar30 = _DAT_142a7baa0;
          uStack_120 = uVar16;
          if (uVar9 < 3) {
            if (uStack_108 == uStack_118) {
              FUN_140b81df0(&uStack_118,&UNK_142fcb578);
            }
            puStack_110[uVar15] = uVar8;
            uStack_100 = uStack_100 & 0xffffffff00000000;
            goto LAB_141fe1193;
          }
          if (uVar16 == uStack_130) {
            FUN_140562db0(&uStack_130,&UNK_142fcb590);
          }
          *(ulonglong *)(lStack_128 + uVar16 * 0x18) = uVar3;
          *(ulonglong *)(lStack_128 + 8 + uVar16 * 0x18) = uVar8;
          *(undefined1 *)(lStack_128 + 0x10 + uVar16 * 0x18) = 0;
          uStack_100 = uStack_100 & 0xffffffff00000000;
          puVar10 = &uStack_120;
        }
        else {
          if (uStack_108 == uStack_118) {
            FUN_140b81df0(&uStack_118,&UNK_142fcb560);
          }
          puStack_110[uVar15] = uVar8;
LAB_141fe1193:
          puVar10 = &uStack_108;
          uVar16 = uVar15;
        }
        *puVar10 = uVar16 + 1;
        fVar33 = fVar5;
        fVar24 = fVar30;
      }
LAB_141fe0df0:
      uVar15 = uStack_108;
      puVar7 = puStack_110;
      uVar16 = uStack_108 * 8;
      uVar14 = 0;
      if ((uStack_108 >> 0x3d == 0) && (uVar16 < 0x7ffffffffffffff9)) {
        if (uVar16 == 0) {
          uVar8 = 8;
          puStack_f8 = (undefined8 *)0x0;
LAB_141fe134d:
          uStack_100 = uVar8;
          func_0x000142923c00(uVar8,puVar7,uVar16);
          puStack_158 = puStack_110 + uStack_108;
          puStack_160 = puStack_110;
          pfStack_150 = param_2;
          uStack_148 = param_3;
          FUN_141206920(&uStack_178,&puStack_160,&UNK_142fcb408);
          *param_1 = (ulonglong)puStack_f8;
          param_1[1] = uStack_100;
          param_1[2] = uVar15;
          param_1[5] = uStack_168;
          param_1[3] = uStack_178;
          param_1[4] = uStack_170;
          if (uStack_118 != 0) {
            func_0x000140613c20(puStack_110,uStack_118 << 3,8);
          }
          if (uStack_130 == 0) {
            return param_1;
          }
          func_0x000140613c20(lStack_128,uStack_130 * 0x18,8);
          return param_1;
        }
        uVar14 = 8;
        uVar8 = func_0x000140613c10(uVar16,8);
        if (uVar8 != 0) {
          puStack_f8 = (undefined8 *)uVar15;
          goto LAB_141fe134d;
        }
      }
      FUN_1428d8fe3(uVar14,uVar16,&UNK_142c19f70);
    }
  }
LAB_141fe14f1:
                    /* WARNING: Does not return */
  pcVar4 = (code *)invalidInstructionException();
  (*pcVar4)();
LAB_141fe1446:
  FUN_1428d9430(&UNK_142fcb5c0,0x26,&UNK_142fcb5e8);
  goto LAB_141fe14f1;
}
