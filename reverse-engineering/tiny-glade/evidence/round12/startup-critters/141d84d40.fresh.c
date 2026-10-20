
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void FUN_141d84d40(longlong *param_1,undefined8 *param_2,undefined8 *param_3,longlong *param_4,
                  undefined8 *param_5,longlong *param_6,undefined8 *param_7,undefined8 *param_8)

{
  byte *pbVar1;
  byte bVar2;
  byte bVar3;
  byte bVar4;
  code *pcVar5;
  float fVar6;
  float fVar7;
  float fVar8;
  char cVar9;
  int iVar10;
  longlong lVar11;
  undefined8 *puVar12;
  longlong *plVar13;
  longlong lVar14;
  undefined8 *puVar15;
  uint *puVar16;
  ulonglong uVar17;
  undefined *puVar18;
  undefined8 *puVar19;
  longlong lVar20;
  ulonglong uVar21;
  undefined *puVar22;
  longlong lVar23;
  bool bVar24;
  undefined1 uVar25;
  bool bVar26;
  float fVar27;
  float fVar28;
  uint uVar29;
  float fVar30;
  float fVar31;
  float fVar32;
  float fVar33;
  float fVar34;
  undefined8 in_stack_fffffffffffffd58;
  undefined4 uVar35;
  longlong lStack_288;
  longlong lStack_280;
  longlong lStack_278;
  longlong lStack_270;
  uint *puStack_268;
  uint *puStack_260;
  undefined8 uStack_258;
  undefined8 uStack_250;
  longlong lStack_248;
  undefined8 uStack_240;
  longlong lStack_238;
  undefined8 uStack_230;
  longlong lStack_228;
  undefined8 uStack_220;
  undefined1 uStack_218;
  longlong lStack_1f8;
  longlong lStack_1f0;
  undefined8 uStack_1e8;
  ulonglong uStack_1e0;
  ulonglong uStack_1d8;
  byte bStack_1d0;
  undefined8 uStack_1c8;
  undefined8 uStack_1c0;
  uint *puStack_1a8;
  undefined8 uStack_1a0;
  undefined8 uStack_198;
  undefined *puStack_190;
  undefined8 uStack_188;
  undefined8 uStack_180;
  longlong lStack_178;
  undefined *puStack_170;
  longlong *plStack_168;
  longlong lStack_160;
  ulonglong uStack_158;
  longlong lStack_150;
  longlong lStack_148;
  longlong lStack_140;
  undefined8 uStack_138;
  longlong *plStack_130;
  longlong lStack_128;
  undefined *puStack_120;
  longlong lStack_118;
  longlong lStack_110;
  undefined8 uStack_108;
  longlong *plStack_100;
  byte bStack_f6;
  undefined1 uStack_f5;
  undefined4 uStack_f4;
  undefined8 uStack_f0;
  
  uVar35 = (undefined4)((ulonglong)in_stack_fffffffffffffd58 >> 0x20);
  uStack_f0 = 0xfffffffffffffffe;
  puVar12 = param_2;
  cVar9 = func_0x000140c16a00();
  if (cVar9 == '\0') {
LAB_141d84e2c:
    puVar22 = (undefined *)0x0;
LAB_141d85004:
    *(undefined4 *)param_2[2] = *(undefined4 *)((longlong)param_2 + 0x1c);
    uStack_138 = *param_2;
    fVar28 = *(float *)(*param_6 + 0x28);
    puStack_190 = puVar22;
    puStack_120 = puVar22;
    puVar12 = (undefined8 *)FUN_14230e780(*param_5,&UNK_14304a2e8);
    if (puVar12 == (undefined8 *)0x0) {
LAB_141d85ca8:
      puStack_120 = puVar22;
      FUN_1428d9310(&UNK_142f67f98);
      goto LAB_141d85cbb;
    }
    puStack_170 = &UNK_14304a3d0;
    plStack_168 = (longlong *)0x3f;
    puVar15 = puVar12 + 2;
    if ((puVar12[3] == 0x3f) &&
       (iVar10 = func_0x000142923bf0(&UNK_14304a3d0,*puVar15,0x3f), iVar10 == 0)) {
      puStack_120 = puVar22;
      puVar12 = (undefined8 *)(**(code **)(puVar12[1] + 0x18))(*puVar12);
      if (puVar12 == (undefined8 *)0x0) goto LAB_141d85ca8;
      uStack_1a0 = *puVar12;
      uStack_198 = puVar12[1];
      puStack_120 = puVar22;
      lVar11 = func_0x00014067dd80(*param_7);
      puVar12 = (undefined8 *)(lVar11 + 0x20);
      puVar19 = (undefined8 *)(lVar11 + 0x28);
      if (*(char *)(lVar11 + 0x298) == '\0') {
        puVar19 = (undefined8 *)(lVar11 + 0x10);
        puVar12 = (undefined8 *)(lVar11 + 8);
      }
      uStack_1c0 = *puVar19;
      uStack_1c8 = *puVar12;
      puStack_120 = puVar22;
      func_0x000140a12650(&lStack_288,*param_3);
      fVar33 = (float)lStack_280;
      fVar32 = (float)lStack_288;
      lStack_270 = *param_4;
      lStack_288 = param_4[1];
      puStack_268 = *(uint **)(lStack_270 + 0xf8);
      puStack_260 = puStack_268 + *(longlong *)(lStack_270 + 0x100);
      uStack_218 = *(undefined1 *)(lStack_270 + 0x108);
      uStack_230 = 0;
      lStack_228 = 0;
      uStack_240 = 0;
      lStack_238 = 0;
      uStack_220 = 0;
      lStack_278 = lStack_288 + 0xf8;
      lStack_280 = lStack_288 + 0x1a0;
      uStack_258 = 8;
      uStack_250 = 0;
      lStack_248 = 8;
      puStack_120 = puVar22;
      FUN_141dbfba0(&puStack_170,&lStack_288,&UNK_142f67e58);
      lStack_128 = *param_1;
      lStack_150 = param_1[1];
      func_0x00014230ea80(&puStack_268,lStack_150,lStack_128,(int)param_1[2],
                          CONCAT44(uVar35,*(undefined4 *)((longlong)param_1 + 0x14)));
      fVar30 = _DAT_1429cd078;
      fVar8 = _DAT_142934694;
      fVar7 = _DAT_142925904;
      fVar6 = _DAT_142925870;
      lStack_178 = lStack_248;
      lStack_148 = lStack_238;
      lStack_140 = lStack_228;
      uStack_f4 = uStack_220._4_4_;
      lStack_118 = lStack_1f8;
      lStack_110 = lStack_1f0;
      uStack_108 = uStack_1e8;
      uStack_180 = *param_8;
      fVar34 = _DAT_142925870 * fVar28;
      lVar11 = lStack_160 << 3;
      bStack_f6 = bStack_1d0;
      plStack_130 = plStack_168;
      puStack_1a8 = puStack_260;
      if ((bStack_1d0 & 1) == 0) {
        if (uStack_1d8 != uStack_1e0) {
LAB_141d8563c:
          uVar17 = (ulonglong)*(uint *)(lStack_178 + 8 + uStack_1d8 * 0x10);
          uStack_158 = uStack_1d8;
          goto LAB_141d85655;
        }
        if (puStack_268 != puStack_260) {
          puVar16 = puStack_268;
          do {
            if (*(longlong *)(*(longlong *)(lStack_150 + 0x100) + 0x58 + (ulonglong)*puVar16 * 0xa0)
                != 0) {
              lVar23 = *(longlong *)(lStack_150 + 0x100) + (ulonglong)*puVar16 * 0xa0;
              uVar17 = (ulonglong)*(uint *)(lVar23 + 0x94);
              lVar14 = *(longlong *)(lStack_150 + 0x1a8) + uVar17 * 0x48;
              uVar17 = *(ulonglong *)(*(longlong *)(lStack_150 + 0x1a8) + 0x40 + uVar17 * 0x48);
              if ((*(ulonglong *)(lStack_128 + 0x110) < uVar17) &&
                 (uVar21 = *(ulonglong *)
                            (*(longlong *)(lVar14 + 0x38) + *(ulonglong *)(lStack_128 + 0x110) * 8),
                 uVar21 != 0)) {
                lStack_118 = *(longlong *)(*(longlong *)(lVar14 + 0x18) + 0x10 + ~uVar21 * 0x30);
              }
              else {
                lStack_118 = 0;
              }
              lVar20 = *(longlong *)(lVar14 + 0x38);
              if (*(ulonglong *)(lStack_128 + 0x118) < uVar17) {
                uVar21 = *(ulonglong *)(lVar20 + *(ulonglong *)(lStack_128 + 0x118) * 8);
                lVar14 = *(longlong *)(lVar14 + 0x18);
                if (uVar21 == 0) goto LAB_141d855ab;
                lStack_110 = *(longlong *)(lVar14 + 0x10 + ~uVar21 * 0x30);
                uVar21 = *(ulonglong *)(lStack_128 + 0x130);
                if (uVar21 < uVar17) goto LAB_141d855c9;
LAB_141d855eb:
                uStack_108 = 0;
              }
              else {
                lVar14 = *(longlong *)(lVar14 + 0x18);
LAB_141d855ab:
                lStack_110 = 0;
                uVar21 = *(ulonglong *)(lStack_128 + 0x130);
                if (uVar17 <= uVar21) goto LAB_141d855eb;
LAB_141d855c9:
                uVar17 = *(ulonglong *)(lVar20 + uVar21 * 8);
                if (uVar17 == 0) goto LAB_141d855eb;
                uStack_108 = *(undefined8 *)(lVar14 + 0x10 + ~uVar17 * 0x30);
              }
              lVar20 = ~*(ulonglong *)(lVar20 + *(longlong *)(lStack_128 + 0x120) * 8) * 0x30;
              lStack_148 = *(longlong *)(lVar14 + 0x10 + lVar20);
              lStack_140 = *(longlong *)(lVar14 + 0x28 + lVar20);
              lStack_178 = *(longlong *)(lVar23 + 0x50);
              uStack_1d8 = 0;
              goto LAB_141d8563c;
            }
            puVar16 = puVar16 + 1;
          } while (puVar16 != puStack_260);
        }
      }
      else {
        if (uStack_1d8 != uStack_1e0) {
LAB_141d85599:
          uVar17 = uStack_1d8 & 0xffffffff;
          uStack_158 = uStack_1d8;
LAB_141d85655:
          lVar14 = lStack_110 + uVar17 * 0xc;
          puVar16 = (uint *)(lStack_148 + uVar17 * 0x18);
          lVar23 = uVar17 * 0x30 + lStack_118;
          plStack_100 = (longlong *)(lStack_140 + uVar17 * 4);
          cVar9 = func_0x000140911230();
          if (cVar9 == '\x03') {
            FUN_140911dd0(&lStack_288,uStack_180);
            fVar27 = *(float *)(lVar23 + 0x10);
            fVar31 = *(float *)(lVar23 + 0x18);
            fVar32 = SQRT(((float)lStack_280 - fVar31) * ((float)lStack_280 - fVar31) +
                          ((float)lStack_288 - fVar27) * ((float)lStack_288 - fVar27));
            bVar26 = fVar30 == fVar32;
            bVar24 = fVar30 < fVar32;
          }
          else {
            fVar27 = *(float *)(lVar23 + 0x10);
            fVar31 = *(float *)(lVar23 + 0x18);
            fVar32 = (fVar31 - fVar33) * (fVar31 - fVar33) + (fVar27 - fVar32) * (fVar27 - fVar32);
            bVar26 = fVar8 == fVar32;
            bVar24 = fVar8 < fVar32;
          }
          uStack_158 = uStack_158 + 1;
          lVar23 = 0;
          do {
            lVar20 = lVar23;
            if (lVar11 == lVar20) break;
            fVar32 = fVar27 - (float)*(undefined8 *)((longlong)plStack_130 + lVar20);
            fVar33 = fVar31 - (float)((ulonglong)*(undefined8 *)((longlong)plStack_130 + lVar20) >>
                                     0x20);
            lVar23 = lVar20 + 8;
          } while (fVar8 <= fVar33 * fVar33 + fVar32 * fVar32);
          fVar34 = (float)puVar16[4] - fVar34;
          if (fVar34 <= 0.0) {
            fVar34 = 0.0;
          }
          *(undefined4 *)plStack_100 = uStack_f4;
          puVar16[4] = (uint)fVar34;
          uStack_f5 = !bVar24 && !bVar26 || lVar11 != lVar20;
          if (!bVar24 && !bVar26 || lVar11 != lVar20) {
            *(undefined4 *)plStack_100 = uStack_f4;
            puVar16[4] = 0x3f800000;
            fVar34 = fVar7;
          }
          fVar32 = (float)puVar16[3];
          if ((fVar32 <= 0.0) || (0.0 < *(float *)(lVar14 + 4))) {
            if (fVar32 <= 0.0) {
              *(undefined4 *)plStack_100 = uStack_f4;
              fVar34 = (float)puVar16[3];
              puVar16[3] = (uint)(fVar34 + fVar28);
              if (fVar34 + fVar28 <= 0.0) {
                    /* WARNING: Could not recover jumptable at 0x000141d85a20. Too many branches */
                    /* WARNING: Treating indirect jump as call */
                (*(code *)(&UNK_142f68008 + *(int *)(&UNK_142f68008 + (ulonglong)*puVar16 * 4)))
                          (-(uint)(0.0 < (float)puVar16[4]) & (uint)fVar8);
                return;
              }
              fVar28 = (float)func_0x000141625ae0(uStack_138);
              fVar28 = fVar28 * _DAT_1429258d8 + _DAT_1429cd068;
              *(undefined4 *)plStack_100 = uStack_f4;
              puVar16[3] = (uint)fVar28;
                    /* WARNING: Could not recover jumptable at 0x000141d8594f. Too many branches */
                    /* WARNING: Treating indirect jump as call */
              (*(code *)(&UNK_142f68008 + *(int *)(&UNK_142f68008 + (ulonglong)*puVar16 * 4)))
                        (-(uint)(0.0 < (float)puVar16[4]) & (uint)fVar8);
              return;
            }
LAB_141d85866:
            if (0.0 < fVar32) goto LAB_141d85955;
            bVar26 = false;
            fVar28 = fVar30;
          }
          else {
            fVar33 = (float)((uint)(fVar6 - (float)puVar16[5]) & _DAT_14292e980);
            if ((fVar33 < _DAT_1429cd068) || (_DAT_142a7fca4 <= fVar33)) goto LAB_141d85866;
            *(undefined4 *)plStack_100 = uStack_f4;
            fVar28 = (float)puVar16[3] - fVar28;
            puVar16[3] = (uint)fVar28;
            bVar26 = true;
            if (0.0 < fVar28) {
                    /* WARNING: Could not recover jumptable at 0x000141d85850. Too many branches */
                    /* WARNING: Treating indirect jump as call */
              (*(code *)(&UNK_142f68008 + *(int *)(&UNK_142f68008 + (ulonglong)*puVar16 * 4)))
                        (-(uint)(0.0 < (float)puVar16[4]) & (uint)fVar8);
              return;
            }
          }
          fVar28 = (float)func_0x000141625ae0(fVar28);
          uVar29 = (uint)(fVar28 * _DAT_1429258d0 + fVar6) ^ _DAT_1429258f0;
          *(undefined4 *)plStack_100 = uStack_f4;
          puVar16[3] = uVar29;
          fVar34 = (float)puVar16[4];
          if (bVar26) {
            fVar30 = fVar7;
          }
LAB_141d85955:
                    /* WARNING: Could not recover jumptable at 0x000141d85981. Too many branches */
                    /* WARNING: Treating indirect jump as call */
          (*(code *)(&UNK_142f68008 + *(int *)(&UNK_142f68008 + (ulonglong)*puVar16 * 4)))
                    (fVar30,fVar34,-(uint)(0.0 < fVar34) & (uint)fVar8);
          return;
        }
        if (puStack_268 != puStack_260) {
          puVar16 = puStack_268;
          do {
            if (*(longlong *)(*(longlong *)(lStack_150 + 0x1a8) + 0x10 + (ulonglong)*puVar16 * 0x48)
                != 0) {
              lVar14 = *(longlong *)(lStack_150 + 0x1a8) + (ulonglong)*puVar16 * 0x48;
              uVar17 = *(ulonglong *)(lVar14 + 0x40);
              if ((*(ulonglong *)(lStack_128 + 0x110) < uVar17) &&
                 (uVar21 = *(ulonglong *)
                            (*(longlong *)(lVar14 + 0x38) + *(ulonglong *)(lStack_128 + 0x110) * 8),
                 uVar21 != 0)) {
                lStack_118 = *(longlong *)(*(longlong *)(lVar14 + 0x18) + 0x10 + ~uVar21 * 0x30);
              }
              else {
                lStack_118 = 0;
              }
              lVar23 = *(longlong *)(lVar14 + 0x38);
              if (*(ulonglong *)(lStack_128 + 0x118) < uVar17) {
                uVar21 = *(ulonglong *)(lVar23 + *(ulonglong *)(lStack_128 + 0x118) * 8);
                lVar14 = *(longlong *)(lVar14 + 0x18);
                if (uVar21 == 0) goto LAB_141d8551a;
                lStack_110 = *(longlong *)(lVar14 + 0x10 + ~uVar21 * 0x30);
              }
              else {
                lVar14 = *(longlong *)(lVar14 + 0x18);
LAB_141d8551a:
                lStack_110 = 0;
              }
              lVar20 = ~*(ulonglong *)(lVar23 + *(longlong *)(lStack_128 + 0x120) * 8) * 0x30;
              lStack_148 = *(longlong *)(lVar14 + 0x10 + lVar20);
              lStack_140 = *(longlong *)(lVar14 + 0x28 + lVar20);
              if ((*(ulonglong *)(lStack_128 + 0x130) < uVar17) &&
                 (uVar17 = *(ulonglong *)(lVar23 + *(ulonglong *)(lStack_128 + 0x130) * 8),
                 uVar17 != 0)) {
                uStack_108 = *(undefined8 *)(lVar14 + 0x10 + ~uVar17 * 0x30);
              }
              else {
                uStack_108 = 0;
              }
              uStack_1d8 = 0;
              goto LAB_141d85599;
            }
            puVar16 = puVar16 + 1;
          } while (puVar16 != puStack_260);
        }
      }
      if (puStack_170 != (undefined *)0x0) {
        func_0x000140613c20(plStack_168,(longlong)puStack_170 << 3,4);
      }
      if (puStack_190 == (undefined *)0x0) {
        return;
      }
      plVar13 = (longlong *)func_0x000140c18130();
      if (*plVar13 == 1) {
        plVar13 = plVar13 + 1;
      }
      else if ((*plVar13 != 0) ||
              (plVar13 = (longlong *)FUN_14291b030(plVar13,0), plVar13 == (longlong *)0x0))
      goto LAB_141d85c5e;
      if (*plVar13 == 0) {
        *plVar13 = -1;
        plStack_130 = plVar13;
        FUN_140c170f0(plVar13 + 1,uStack_188);
        *plStack_130 = *plStack_130 + 1;
        return;
      }
      goto LAB_141d85c6a;
    }
  }
  else {
    puVar15 = (undefined8 *)0x33;
    plStack_130 = param_4;
    lVar11 = FUN_141d84910(0x33);
    param_4 = (longlong *)&UNK_142f67e70;
    if (lVar11 == 1) {
      if (puVar12 != (undefined8 *)0x0) {
        if (puVar12 < (undefined8 *)0x33) {
          if (*(char *)(puVar12 + 0x285ecfce) < -0x40) {
LAB_141d84e0c:
            uVar35 = 1;
            FUN_1428d9940(&UNK_142f67e70,0x33,0,puVar12,&UNK_142f67f28);
            goto LAB_141d84e2c;
          }
        }
        else if (puVar12 != (undefined8 *)0x33) goto LAB_141d84e0c;
      }
      lVar11 = FUN_141d84910(puVar12);
      if (lVar11 == 1) {
        uVar17 = (longlong)puVar12 + 2;
        if (uVar17 != 0) {
          uVar25 = uVar17 == 0x33;
          if (0x32 < uVar17) goto LAB_141d84e84;
          uVar25 = *(char *)((longlong)puVar12 + 0x142f67e72) == -0x41;
          if (*(char *)((longlong)puVar12 + 0x142f67e72) < -0x40) {
            do {
              puVar12 = (undefined8 *)0x33;
              FUN_1428d9940(&UNK_142f67e70,0x33);
LAB_141d84e84:
            } while (!(bool)uVar25);
          }
        }
        puVar15 = (undefined8 *)(0x31 - (longlong)puVar12);
        param_4 = (longlong *)(&UNK_142f67e70 + uVar17);
      }
    }
    puVar18 = &UNK_142f67f98;
    puVar22 = &UNK_142f67f70;
    do {
      bVar2 = puVar18[-1];
      uVar29 = (uint)(char)bVar2;
      if ((char)bVar2 < '\0') {
        bVar3 = puVar18[-2];
        if ((char)bVar3 < -0x40) {
          bVar4 = puVar18[-3];
          if ((char)bVar4 < -0x40) {
            pbVar1 = puVar18 + -4;
            puVar18 = puVar18 + -4;
            uVar29 = bVar4 & 0x3f | (*pbVar1 & 7) << 6;
          }
          else {
            puVar18 = puVar18 + -3;
            uVar29 = bVar4 & 0xf;
          }
          uVar29 = bVar3 & 0x3f | uVar29 << 6;
        }
        else {
          puVar18 = puVar18 + -2;
          uVar29 = bVar3 & 0x1f;
        }
        uVar29 = (uint)(bVar2 & 0x3f) | uVar29 << 6;
        if (uVar29 != 0x5c) goto LAB_141d84f24;
LAB_141d84f39:
        puVar22 = puVar18 + -0x142f67f6f;
        if (puVar22 != (undefined *)0x0) {
          uVar25 = puVar22 == (undefined *)0x28;
          if ((undefined *)0x27 < puVar22) goto LAB_141d84f78;
          uVar25 = puVar18[1] == -0x41;
          if ((char)puVar18[1] < -0x40) {
            do {
              FUN_1428d9940(&UNK_142f67f70,0x28);
LAB_141d84f78:
            } while (!(bool)uVar25);
          }
        }
        lVar11 = 0x28 - (longlong)puVar22;
        puVar22 = &UNK_142f67f70 + (longlong)puVar22;
        goto LAB_141d84f89;
      }
      puVar18 = puVar18 + -1;
      if (uVar29 == 0x5c) goto LAB_141d84f39;
LAB_141d84f24:
      if (uVar29 == 0x2f) goto LAB_141d84f39;
    } while (puVar18 != &UNK_142f67f70);
    lVar11 = 0x28;
LAB_141d84f89:
    plVar13 = (longlong *)func_0x000140c18130();
    if (*plVar13 == 1) {
      plVar13 = plVar13 + 1;
LAB_141d84fa2:
      if (*plVar13 == 0) {
        *plVar13 = -1;
        plStack_100 = plVar13;
        uStack_188 = FUN_140c16d40(plVar13 + 1,param_4,puVar15,puVar22,lVar11,1,0);
        uVar35 = (undefined4)((ulonglong)lVar11 >> 0x20);
        *plStack_100 = *plStack_100 + 1;
        puVar22 = (undefined *)0x1;
        param_4 = plStack_130;
        goto LAB_141d85004;
      }
    }
    else {
      if ((*plVar13 == 0) &&
         (plVar13 = (longlong *)FUN_14291b030(plVar13,0), plVar13 != (longlong *)0x0))
      goto LAB_141d84fa2;
LAB_141d85c5e:
      FUN_1428d80d0(&UNK_14304a0d0);
    }
LAB_141d85c6a:
    FUN_1428d9250(&UNK_14304a140);
  }
  lStack_288 = 0;
  puStack_120 = puVar22;
  FUN_1428cab41(0,&puStack_170,puVar15,&lStack_288,&UNK_14304a2c0);
LAB_141d85cbb:
                    /* WARNING: Does not return */
  pcVar5 = (code *)invalidInstructionException();
  (*pcVar5)();
}

