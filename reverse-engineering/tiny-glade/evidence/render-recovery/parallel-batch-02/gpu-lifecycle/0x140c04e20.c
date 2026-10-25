
undefined8 *
FUN_140c04e20(undefined8 *param_1,ulonglong param_2,uint param_3,uint param_4,undefined8 param_5)

{
  byte *pbVar1;
  int *piVar2;
  undefined8 *puVar3;
  code *pcVar4;
  byte bVar5;
  byte bVar6;
  byte bVar7;
  ulonglong *puVar8;
  undefined4 *puVar9;
  code **ppcVar10;
  code *pcVar11;
  uint uVar12;
  int iVar13;
  undefined4 uVar14;
  char cVar15;
  undefined4 uVar16;
  uint uVar17;
  int iVar18;
  longlong lVar19;
  longlong *plVar20;
  code **ppcVar21;
  code **ppcVar22;
  ulonglong uVar23;
  ulonglong *puVar24;
  ulonglong *puVar25;
  code **ppcVar26;
  ulonglong uVar27;
  longlong lVar28;
  undefined *puVar29;
  ulonglong uVar30;
  undefined *puVar31;
  ulonglong uVar32;
  ulonglong *puVar33;
  undefined *puVar34;
  bool bVar35;
  longlong lStack_4e0;
  undefined4 uStack_4d8;
  undefined4 uStack_4d4;
  undefined4 uStack_4d0;
  undefined4 uStack_4cc;
  undefined4 uStack_450;
  undefined4 uStack_44c;
  undefined4 uStack_448;
  undefined4 uStack_444;
  undefined4 uStack_440;
  undefined4 uStack_43c;
  undefined4 uStack_438;
  undefined4 uStack_434;
  undefined4 uStack_430;
  undefined4 uStack_42c;
  undefined4 uStack_428;
  undefined4 uStack_424;
  code *pcStack_420;
  code *pcStack_418;
  code **ppcStack_410;
  code *pcStack_408;
  undefined8 uStack_390;
  longlong lStack_388;
  undefined8 uStack_370;
  undefined8 uStack_368;
  code **ppcStack_360;
  code **ppcStack_358;
  code **ppcStack_350;
  undefined4 uStack_348;
  undefined4 uStack_344;
  undefined4 uStack_340;
  undefined4 uStack_33c;
  undefined4 uStack_338;
  undefined4 uStack_334;
  undefined4 uStack_330;
  undefined4 uStack_32c;
  code **ppcStack_328;
  code *pcStack_320;
  code **ppcStack_2e0;
  undefined4 *puStack_2d8;
  undefined4 *puStack_2d0;
  ulonglong *puStack_2c8;
  longlong lStack_2c0;
  longlong lStack_2b8;
  longlong lStack_2b0;
  undefined8 uStack_2a8;
  longlong lStack_2a0;
  code **ppcStack_298;
  undefined4 uStack_290;
  undefined4 uStack_28c;
  undefined4 uStack_288;
  undefined4 uStack_284;
  uint uStack_280;
  uint uStack_27c;
  longlong lStack_278;
  undefined8 uStack_270;
  code **ppcStack_268;
  undefined8 *puStack_260;
  code **ppcStack_258;
  code **ppcStack_250;
  code **ppcStack_248;
  code *pcStack_240;
  code **ppcStack_238;
  code **ppcStack_230;
  undefined8 uStack_228;
  undefined8 uStack_220;
  undefined8 uStack_218;
  undefined8 uStack_210;
  code **ppcStack_208;
  code *pcStack_200;
  longlong lStack_1f8;
  code *pcStack_1f0;
  longlong lStack_1e8;
  longlong lStack_1e0;
  longlong lStack_1d8;
  undefined8 uStack_1d0;
  undefined8 uStack_1c8;
  undefined8 uStack_1c0;
  int iStack_1b8;
  int iStack_1b4;
  undefined8 uStack_1b0;
  code **ppcStack_1a8;
  code **ppcStack_1a0;
  code **ppcStack_198;
  undefined8 uStack_190;
  undefined8 uStack_188;
  undefined8 uStack_180;
  longlong lStack_178;
  code **ppcStack_170;
  code *pcStack_168;
  undefined4 *puStack_160;
  undefined4 *puStack_158;
  undefined4 *puStack_150;
  undefined8 uStack_e8;
  code **ppcStack_e0;
  longlong lStack_d8;
  undefined8 uStack_d0;
  longlong lStack_c8;
  undefined4 *puStack_c0;
  longlong lStack_b8;
  char *pcStack_b0;
  code **ppcStack_a8;
  undefined8 uStack_a0;
  undefined8 uStack_98;
  code **ppcStack_90;
  uint uStack_88;
  uint uStack_84;
  ulonglong uStack_80;
  longlong lStack_78;
  undefined8 uStack_70;
  undefined1 uStack_65;
  undefined1 uStack_64;
  undefined1 uStack_63;
  undefined1 uStack_62;
  char cStack_61;
  code **ppcStack_60;
  undefined1 uStack_53;
  undefined1 uStack_52;
  undefined1 uStack_51;
  undefined1 uStack_50;
  undefined1 uStack_4f;
  undefined1 uStack_4e;
  undefined1 uStack_4d;
  undefined1 uStack_49;
  undefined8 uStack_48;
  
  uStack_48 = 0xfffffffffffffffe;
  uStack_49 = 1;
  puStack_260 = param_1;
  uStack_88 = param_3;
  uStack_84 = param_4;
  uStack_80 = param_2;
  uVar16 = func_0x000140c16a00();
  if ((char)uVar16 != '\0') {
    uStack_49 = 1;
    lVar28 = 0x51;
    lVar19 = FUN_140c02eb0(0x51);
    puVar34 = &UNK_142b183f0;
    if (lVar19 == 1) {
      if (param_2 != 0) {
        if (param_2 < 0x51) {
          if ((char)(&UNK_142b183f0)[param_2] < -0x40) {
code_r0x000140c04ebd:
            uStack_49 = 1;
            FUN_1428d9940(&UNK_142b183f0,0x51,0,param_2,&UNK_142b184c8);
            goto code_r0x000140c0687b;
          }
        }
        else if (param_2 != 0x51) goto code_r0x000140c04ebd;
      }
      uStack_49 = 1;
      lVar19 = FUN_140c02eb0(param_2);
      if (lVar19 == 1) {
        uVar30 = param_2 + 2;
        if (uVar30 != 0) {
          if (uVar30 < 0x51) {
            if ((char)(&UNK_142b183f2)[param_2] < -0x40) {
code_r0x000140c04f1d:
              uStack_49 = 1;
              FUN_1428d9940(&UNK_142b183f0,0x51,uVar30,0x51,&UNK_142b184e0);
              goto code_r0x000140c0687b;
            }
          }
          else if (uVar30 != 0x51) goto code_r0x000140c04f1d;
        }
        lVar28 = 0x4f - param_2;
        puVar34 = &UNK_142b183f2 + param_2;
      }
    }
    puVar31 = &UNK_142b187b1;
    puVar29 = &UNK_142b18790;
    do {
      bVar5 = puVar31[-1];
      uVar17 = (uint)(char)bVar5;
      if ((char)bVar5 < '\0') {
        bVar6 = puVar31[-2];
        if ((char)bVar6 < -0x40) {
          bVar7 = puVar31[-3];
          if ((char)bVar7 < -0x40) {
            pbVar1 = puVar31 + -4;
            puVar31 = puVar31 + -4;
            uVar17 = bVar7 & 0x3f | (*pbVar1 & 7) << 6;
          }
          else {
            puVar31 = puVar31 + -3;
            uVar17 = bVar7 & 0xf;
          }
          uVar17 = bVar6 & 0x3f | uVar17 << 6;
        }
        else {
          puVar31 = puVar31 + -2;
          uVar17 = bVar6 & 0x1f;
        }
        uVar17 = (uint)(bVar5 & 0x3f) | uVar17 << 6;
        if (uVar17 != 0x5c) goto code_r0x000140c04fe4;
code_r0x000140c04ff5:
        puVar29 = puVar31 + -0x142b1878f;
        if (puVar29 != (undefined *)0x0) {
          if (puVar29 < (undefined *)0x21) {
            if ((char)puVar31[1] < -0x40) {
code_r0x000140c05011:
              uStack_49 = 1;
              FUN_1428d9940(&UNK_142b18790,0x21,puVar29,0x21,&UNK_142b184a8);
              goto code_r0x000140c0687b;
            }
          }
          else if (puVar29 != (undefined *)0x21) goto code_r0x000140c05011;
        }
        lVar19 = 0x21 - (longlong)puVar29;
        puVar29 = puVar31 + 1;
        goto code_r0x000140c0504d;
      }
      puVar31 = puVar31 + -1;
      if (uVar17 == 0x5c) goto code_r0x000140c04ff5;
code_r0x000140c04fe4:
      if (uVar17 == 0x2f) goto code_r0x000140c04ff5;
    } while (puVar31 != &UNK_142b18790);
    lVar19 = 0x21;
code_r0x000140c0504d:
    plVar20 = (longlong *)func_0x000140c18130();
    if (*plVar20 == 0) {
      uStack_49 = 1;
      ppcVar22 = (code **)FUN_1429090d0(plVar20,0);
      if (ppcVar22 == (code **)0x0) goto code_r0x000140c06802;
    }
    else {
      if ((int)*plVar20 != 1) {
code_r0x000140c06802:
        uStack_49 = 1;
        FUN_1428d80d0(&UNK_142d06498);
        goto code_r0x000140c0687b;
      }
      ppcVar22 = (code **)(plVar20 + 1);
    }
    if (*ppcVar22 != (code *)0x0) {
      uStack_49 = 1;
      FUN_1428d9250(&UNK_142d06508);
      goto code_r0x000140c0687b;
    }
    *ppcVar22 = (code *)0xffffffffffffffff;
    ppcStack_60 = ppcVar22;
    uStack_d0 = FUN_140c16d40(ppcVar22 + 1,puVar34,lVar28,puVar29,lVar19,1,0);
    *ppcStack_60 = *ppcStack_60 + 1;
  }
  iStack_1b8 = CONCAT31(iStack_1b8._1_3_,0x19);
  uStack_4d = 1;
  cStack_61 = (char)uVar16;
  FUN_1418c1380(0x143634e40,&iStack_1b8,1);
  uStack_4d = 1;
  cVar15 = FUN_140bf0720(uStack_80);
  if (cVar15 != '\0') {
    uStack_4d = 1;
    FUN_140befd60(uStack_80);
    if (*plRam000000014362c3a8 != 0) {
      iStack_1b8 = 0x42b18968;
      iStack_1b4 = 1;
      uStack_1b0._0_4_ = 1;
      uStack_1b0._4_4_ = 0;
      ppcStack_1a8 = (code **)0x8;
      ppcStack_1a0 = (code **)0x0;
      ppcStack_198 = (code **)0x0;
      uStack_4d = 1;
      ppcStack_350 = (code **)func_0x00014067dd80(&UNK_142b18978);
      uStack_370 = (code *)&UNK_142b18990;
      uStack_368 = (code *)0x15;
      ppcStack_360 = (code **)&UNK_142b18990;
      ppcStack_358 = (code **)0x15;
      uStack_4d = 1;
      FUN_1404b1430(&iStack_1b8,1,&uStack_370);
    }
    uStack_4d = 1;
    FUN_140d63150(1);
    goto code_r0x000140c0687b;
  }
  pcStack_b0 = (char *)(uStack_80 + 0x718);
  LOCK();
  bVar35 = *(char *)(uStack_80 + 0x718) == '\0';
  if (bVar35) {
    *(char *)(uStack_80 + 0x718) = '\x01';
  }
  UNLOCK();
  if (!bVar35) {
    uStack_4d = 1;
    FUN_1428cd210(pcStack_b0,uStack_80,1000000000);
  }
  uStack_4d = 1;
  FUN_140c81c70(pcStack_b0);
  func_0x000142923c00(&iStack_1b8,param_5,0xd8);
  uStack_51 = 1;
  uStack_52 = 1;
  FUN_140bf84e0(&uStack_370,uStack_e8);
  lVar19 = lStack_178;
  ppcStack_a8 = (code **)0x0;
  uStack_a0 = 8;
  uStack_98 = (code **)0x0;
  uVar16 = (undefined4)CONCAT71((uint7)(uint3)((uint)uVar16 >> 8),1);
  bVar35 = !SBORROW8(0,lStack_178);
  if (bVar35) {
    func_0x000142923c00(&lStack_4e0,&lStack_178,0x90);
    FUN_140631a90(&ppcStack_a8,&UNK_142b18be0);
    lVar28 = uStack_a0;
    func_0x000142923c10(uStack_a0,&lStack_4e0,0x90);
    uStack_70 = (code **)((ulonglong)uStack_70._4_4_ << 0x20);
  }
  else {
    lVar28 = 8;
    uStack_70 = (code **)CONCAT44(uStack_70._4_4_,
                                  (int)CONCAT71((int7)((ulonglong)-lStack_178 >> 8),1));
  }
  uStack_98 = (code **)(ulonglong)bVar35;
  if (!SBORROW8(0,(longlong)uStack_370)) {
    func_0x000142923c00(&pcStack_420,&uStack_370,0x90);
    ppcStack_298 = ppcStack_2e0;
    if ((int)ppcStack_2e0 != 0) {
      ppcStack_60 = ppcStack_2e0;
      ppcStack_248 = (code **)0x0;
      FUN_1428ca88c(0,&ppcStack_298,&UNK_142b1818c,&ppcStack_248,&UNK_142b18bf8);
      goto code_r0x000140c0687b;
    }
    func_0x000142923c00(&ppcStack_248,&uStack_370,0x90);
    if ((code **)(ulonglong)bVar35 == ppcStack_a8) {
      ppcStack_60 = ppcStack_2e0;
      FUN_140631a90(&ppcStack_a8,&UNK_142b18c10);
      lVar28 = uStack_a0;
    }
    func_0x000142923c10((ulonglong)((uint)bVar35 * 0x90) + lVar28,&ppcStack_248,0x90);
    uStack_98 = (code **)((longlong)(ulonglong)bVar35 + 1);
    uVar16 = 0;
  }
  ppcStack_60 = (code **)CONCAT44(ppcStack_60._4_4_,uVar16);
  ppcStack_248 = (code **)0x0;
  pcStack_240 = (code *)0x8;
  ppcStack_238 = (code **)0x0;
  ppcStack_230 = (code **)0x0;
  uStack_228 = (code *)0x4;
  uStack_220 = (code **)0x0;
  uStack_218 = (code *)0x0;
  uStack_210 = (code *)0x8;
  ppcStack_208 = (code **)0x0;
  pcStack_200 = (code *)0x0;
  lStack_1f8 = 8;
  pcStack_1f0 = (code *)0x0;
  lStack_1e8 = 0;
  lStack_1e0 = 8;
  lStack_1d8 = 0;
  uStack_1d0 = 0;
  uStack_1c8 = 8;
  uStack_1c0 = 0;
  pcStack_408 = uStack_188 + uStack_180 * 0x58;
  pcStack_420 = uStack_188;
  ppcStack_410 = uStack_190;
  pcStack_418 = uStack_188;
  FUN_1418b4f50(&uStack_1d0,&pcStack_420,&UNK_142d059e0);
  func_0x000142923c00(&pcStack_420,&ppcStack_248,0x90);
  if (iStack_1b8 == 1) {
    func_0x000142923c00(&ppcStack_248,&pcStack_420,0x90);
    ppcVar22 = ppcStack_238;
    if (ppcStack_238 == ppcStack_248) {
      FUN_140562870(&ppcStack_248,&UNK_142d06728);
    }
    ppcVar26 = uStack_220;
    *(ulonglong *)(pcStack_240 + (longlong)ppcVar22 * 8) =
         CONCAT44(uStack_1b0._4_4_,(int)uStack_1b0);
    ppcStack_238 = (code **)((longlong)ppcVar22 + 1);
    if (uStack_220 == ppcStack_230) {
      FUN_140b5f870(&ppcStack_230,&UNK_142d06740);
    }
    *(undefined4 *)(uStack_228 + (longlong)ppcVar26 * 4) = ppcStack_1a8._0_4_;
    uStack_220 = (code **)((longlong)ppcVar26 + 1);
    func_0x000142923c00(&pcStack_420,&ppcStack_248,0x90);
  }
  ppcVar22 = ppcStack_198;
  if (((ulonglong)ppcStack_1a0 & 1) != 0) {
    func_0x000142923c00(&ppcStack_248,&pcStack_420,0x90);
    pcVar11 = pcStack_1f0;
    if (pcStack_1f0 == pcStack_200) {
      FUN_140562870(&pcStack_200,&UNK_142d06758);
    }
    lVar28 = lStack_1d8;
    *(code ***)(lStack_1f8 + (longlong)pcVar11 * 8) = ppcVar22;
    pcStack_1f0 = pcVar11 + 1;
    if (lStack_1d8 == lStack_1e8) {
      FUN_140562870(&lStack_1e8,&UNK_142d06770);
    }
    *(undefined8 *)(lStack_1e0 + lVar28 * 8) = 0;
    lStack_1d8 = lVar28 + 1;
    func_0x000142923c00(&pcStack_420,&ppcStack_248,0x90);
  }
  func_0x000142923c00(&ppcStack_248,&pcStack_420,0x90);
  ppcVar22 = uStack_98;
  if (uStack_98 == ppcStack_a8) {
    FUN_140631a90(&ppcStack_a8,&UNK_142b18c28);
  }
  ppcVar26 = ppcStack_a8;
  lStack_b8 = uStack_a0;
  func_0x000142923c10((longlong)ppcVar22 * 0x90 + uStack_a0,&ppcStack_248);
  if ((!SBORROW8(0,(longlong)uStack_370)) && ((char)ppcStack_60 != '\0')) {
    uStack_51 = 0;
    uStack_52 = SUB81(uStack_70,0);
    FUN_140bdf860(&uStack_370);
  }
  ppcStack_258 = ppcVar26;
  if ((!SBORROW8(0,lVar19)) && ((char)uStack_70 != '\0')) {
    FUN_140bdf860(&lStack_178);
  }
  ppcVar26 = (code **)((longlong)ppcVar22 + 1);
  uStack_53 = 1;
  ppcStack_90 = ppcVar26;
  FUN_1418c3300(uStack_80 + 0x720,uStack_80 + 0x78);
  iStack_1b8 = (int)ppcStack_258;
  iStack_1b4 = (int)((ulonglong)ppcStack_258 >> 0x20);
  uStack_1b0._0_4_ = (int)lStack_b8;
  uStack_1b0._4_4_ = (uint)((ulonglong)lStack_b8 >> 0x20);
  lStack_78 = lStack_b8;
  ppcStack_1a8 = ppcVar26;
  if (ppcVar26 < ppcStack_258) {
    if (ppcVar26 != (code **)0x0) {
      lStack_78 = func_0x000140613c30(lStack_b8,(longlong)ppcStack_258 * 0x90,8,
                                      (longlong)ppcVar26 * 0x90);
      if (lStack_78 == 0) {
        FUN_1428d8fe3(8,(longlong)ppcVar26 * 0x90,&UNK_142b187b8);
        goto code_r0x000140c0687b;
      }
      goto code_r0x000140c05693;
    }
    lStack_78 = 8;
    func_0x000140613c20(lStack_b8,(longlong)ppcStack_258 * 0x90);
  }
  else {
code_r0x000140c05693:
    if (ppcVar26 != (code **)0x0) {
      ppcStack_268 = ppcVar22;
      lVar19 = (longlong)ppcVar26 * 8;
      ppcVar21 = (code **)func_0x000140613c10(lVar19,8);
      uVar30 = uStack_80;
      if (ppcVar21 == (code **)0x0) {
        FUN_1428d8fe3(8,lVar19,&UNK_142b182a0);
        goto code_r0x000140c0687b;
      }
      uVar27 = (ulonglong)((uint)ppcVar26 & 3);
      if (ppcVar22 < (code **)0x3) {
        uVar23 = 0;
      }
      else {
        puVar24 = (ulonglong *)(lStack_78 + 0x238);
        uVar23 = 0;
        do {
          ppcVar21[uVar23] = (code *)puVar24[-0x36];
          ppcVar21[uVar23 + 1] = (code *)puVar24[-0x24];
          ppcVar21[uVar23 + 2] = (code *)puVar24[-0x12];
          ppcVar21[uVar23 + 3] = (code *)*puVar24;
          uVar23 = uVar23 + 4;
          puVar24 = puVar24 + 0x48;
        } while (((ulonglong)ppcVar26 & 0xfffffffffffffc) != uVar23);
      }
      lVar28 = (longlong)ppcVar26 * 0x90 + lStack_78;
      if (uVar27 != 0) {
        puVar24 = (ulonglong *)(uVar23 * 0x90 + lStack_78 + 0x88);
        uVar32 = 0;
        do {
          ppcVar21[uVar23 + uVar32] = (code *)*puVar24;
          uVar32 = uVar32 + 1;
          puVar24 = puVar24 + 0x12;
        } while (uVar27 != uVar32);
      }
      iStack_1b8 = (int)lStack_78;
      iStack_1b4 = (int)((ulonglong)lStack_78 >> 0x20);
      ppcStack_1a8 = (code **)0x0;
      ppcStack_198 = (code **)0x0;
      uStack_62 = 1;
      lStack_2b8 = lVar19;
      ppcStack_250 = ppcVar21;
      uStack_1b0 = lVar28;
      FUN_1418b6db0(&lStack_2b0,&iStack_1b8,&UNK_142b182a0);
      uStack_390 = uStack_2a8;
      lStack_388 = lStack_2a0;
      puStack_2d8 = *(undefined4 **)(uVar30 + 0x7f8);
      puStack_2d0 = (undefined4 *)(lStack_2a0 + (longlong)puStack_2d8);
      *(undefined4 **)(uVar30 + 0x7f8) = puStack_2d0;
      lStack_d8 = lVar28;
      puStack_c0 = puStack_2d8;
      if ((uStack_88 & 1) == 0) {
        lVar19 = uVar30 + 0x730;
      }
      else {
        uVar27 = (ulonglong)uStack_84;
        if (*(ulonglong *)(uVar30 + 0x7f0) <= uVar27) {
          uStack_4e = 1;
          FUN_1428d9518(uVar27,*(ulonglong *)(uVar30 + 0x7f0),&UNK_142d067a0);
          goto code_r0x000140c0687b;
        }
        lVar19 = uVar27 * 0xb0 + *(longlong *)(uVar30 + 0x7e8);
      }
      uStack_4e = 1;
      FUN_1418c3b80(lVar19,puStack_2d8,puStack_2d0,&UNK_142b187d0);
      puVar24 = (ulonglong *)(uStack_80 + 0x730);
      lVar19 = *(longlong *)(uStack_80 + 0x7f0) * 0xb0;
      puVar8 = *(ulonglong **)(uStack_80 + 0x7e8);
      puVar25 = puVar24;
      puVar33 = puVar8;
      if (*(longlong *)(uStack_80 + 0x748) == 0) {
        lVar28 = 0;
        do {
          puVar25 = puVar33;
          uVar30 = 0;
          if (lVar19 == 0) goto code_r0x000140c05955;
          puVar33 = puVar25 + 0x16;
          lVar19 = lVar19 + -0xb0;
        } while (puVar25[3] == 0);
      }
      uVar30 = 0;
      if (*puVar25 <= puVar25[2]) {
        uVar30 = *puVar25;
      }
      uVar30 = *(ulonglong *)(puVar25[1] + (puVar25[2] - uVar30) * 8);
      lVar28 = 1;
      if (puVar33 != puVar8 + *(longlong *)(uStack_80 + 0x7f0) * 0x16) {
        uVar27 = (ulonglong)
                 ((longlong)(puVar8 + *(longlong *)(uStack_80 + 0x7f0) * 0x16) - (longlong)puVar33)
                 / 0xb0;
        puVar33 = puVar33 + 3;
        do {
          if (*puVar33 != 0) {
            uVar23 = puVar33[-3];
            if (puVar33[-1] < puVar33[-3]) {
              uVar23 = 0;
            }
            uVar23 = *(ulonglong *)(puVar33[-2] + (puVar33[-1] - uVar23) * 8);
            if (uVar23 < uVar30) {
              uVar30 = uVar23;
            }
          }
          puVar33 = puVar33 + 0x16;
          uVar27 = uVar27 - 1;
        } while (uVar27 != 0);
      }
code_r0x000140c05955:
      *(longlong *)(uStack_80 + 0x720) = lVar28;
      *(ulonglong *)(uStack_80 + 0x728) = uVar30;
      if (lVar28 == 0) {
        uVar30 = 0;
      }
      iStack_1b4 = (int)uVar30;
      iStack_1b8 = *(int *)(uStack_80 + 0x7f8) - iStack_1b4;
      uStack_1b0._0_4_ = (int)(uVar30 >> 0x20);
      uStack_1b0._4_4_ = uStack_1b0._4_4_ & 0xffffff00;
      uStack_4e = 1;
      FUN_1418c1380(0x143634e40,&iStack_1b8,0xd);
      if (uStack_88 == 1) {
        uVar30 = (ulonglong)uStack_84;
        if (*(ulonglong *)(uStack_80 + 0x7f0) <= uVar30) {
          uStack_4e = 1;
          FUN_1428d9518(uVar30,*(ulonglong *)(uStack_80 + 0x7f0),&UNK_142d067a0);
          goto code_r0x000140c0687b;
        }
        puVar24 = (ulonglong *)(uVar30 * 0xb0 + *(longlong *)(uStack_80 + 0x7e8));
      }
      puStack_2c8 = puVar24 + 4;
      lVar19 = lStack_78;
      lVar28 = lStack_d8;
      while (lVar19 != lVar28) {
        lStack_2c0 = lVar19 + 0x90;
        lStack_c8 = *(longlong *)(lVar19 + 0x78);
        ppcStack_e0 = *(code ***)(lVar19 + 0x80);
        *(undefined8 *)(lVar19 + 0x78) = 0;
        *(undefined8 *)(lVar19 + 0x80) = 8;
        lVar28 = *(longlong *)(lVar19 + 0x88);
        *(undefined8 *)(lVar19 + 0x88) = 0;
        uStack_70 = ppcStack_e0 + lVar28 * 0xb;
        ppcVar22 = ppcStack_e0;
        while (ppcVar26 = uStack_70, ppcVar22 != uStack_70) {
          ppcVar26 = ppcVar22 + 0xb;
          pcVar11 = *ppcVar22;
          ppcStack_208 = (code **)ppcVar22[9];
          pcStack_200 = ppcVar22[10];
          uStack_218 = ppcVar22[7];
          uStack_210 = ppcVar22[8];
          uStack_228 = ppcVar22[5];
          uStack_220 = (code **)ppcVar22[6];
          ppcStack_238 = (code **)ppcVar22[3];
          ppcStack_230 = (code **)ppcVar22[4];
          pcVar4 = ppcVar22[1];
          ppcStack_248 = (code **)ppcVar22[1];
          ppcVar21 = ppcVar22 + 2;
          uVar17 = *(uint *)ppcVar21;
          uVar12 = *(uint *)((longlong)ppcVar22 + 0x14);
          ppcVar22 = (code **)*ppcVar21;
          pcStack_240 = *ppcVar21;
          if (pcVar11 == (code *)0x4) break;
          uStack_228._4_4_ = (undefined4)((ulonglong)uStack_228 >> 0x20);
          uStack_220._4_4_ = (undefined4)((ulonglong)uStack_220 >> 0x20);
          uStack_218._4_4_ = (undefined4)((ulonglong)uStack_218 >> 0x20);
          uStack_210._4_4_ = (undefined4)((ulonglong)uStack_210 >> 0x20);
          uStack_338 = (undefined4)uStack_218;
          uStack_334 = uStack_218._4_4_;
          uStack_330 = (undefined4)uStack_210;
          uStack_32c = uStack_210._4_4_;
          uStack_348 = (undefined4)uStack_228;
          uStack_344 = uStack_228._4_4_;
          uStack_340 = (undefined4)uStack_220;
          uStack_33c = uStack_220._4_4_;
          uStack_370 = pcVar11;
          uStack_368 = pcVar4;
          ppcStack_360 = ppcVar22;
          ppcStack_358 = ppcStack_238;
          ppcStack_350 = ppcStack_230;
          ppcStack_328 = ppcStack_208;
          pcStack_320 = pcStack_200;
          ppcStack_60 = ppcVar26;
          if (pcVar11 != (code *)0x2) {
            pcStack_420 = (code *)&uStack_370;
            pcStack_418 = FUN_140c08ac0;
            iStack_1b8 = 0x42b188e8;
            iStack_1b4 = 1;
            uStack_1b0._0_4_ = 1;
            uStack_1b0._4_4_ = 0;
            ppcStack_198 = (code **)0x0;
            ppcStack_1a8 = &pcStack_420;
            ppcStack_1a0 = (code **)0x1;
            uStack_50 = 1;
            FUN_1428d9390(&iStack_1b8,&UNK_142b188f8);
            goto code_r0x000140c0687b;
          }
          uStack_280 = uVar17;
          uStack_27c = uVar12;
          if ((uStack_88 != uVar17) || ((uVar17 != 0 && (uStack_84 != uVar12)))) {
            iStack_1b8 = 0;
            iStack_1b4 = 0;
            uStack_50 = 1;
            FUN_1428ca88c(0,&uStack_88,&uStack_280,&iStack_1b8,&UNK_142b18858);
            goto code_r0x000140c0687b;
          }
          uStack_370 = (code *)0x3;
          uVar30 = puVar24[0x11];
          lVar28 = *(longlong *)(lVar19 + 0x58);
          if (lVar28 == *(longlong *)(lVar19 + 0x48)) {
            uStack_50 = 1;
            FUN_140562870(lVar19 + 0x48,&UNK_142b18870);
          }
          puVar9 = puStack_c0;
          *(ulonglong *)(*(longlong *)(lVar19 + 0x50) + lVar28 * 8) = uVar30;
          *(longlong *)(lVar19 + 0x58) = lVar28 + 1;
          puStack_c0 = (undefined4 *)((longlong)puStack_c0 + 1);
          lVar28 = *(longlong *)(lVar19 + 0x70);
          if (lVar28 == *(longlong *)(lVar19 + 0x60)) {
            uStack_50 = 1;
            FUN_140562870(lVar19 + 0x60,&UNK_142b18888);
          }
          *(undefined4 **)(*(longlong *)(lVar19 + 0x68) + lVar28 * 8) = puVar9;
          *(longlong *)(lVar19 + 0x70) = lVar28 + 1;
          pcStack_168 = pcStack_320;
          lStack_178 = CONCAT44(uStack_32c,uStack_330);
          ppcStack_170 = ppcStack_328;
          iStack_1b8 = (int)uStack_370;
          iStack_1b4 = uStack_370._4_4_;
          uStack_1b0._0_4_ = (int)uStack_368;
          uStack_1b0._4_4_ = uStack_368._4_4_;
          uStack_188 = (code *)CONCAT44(uStack_33c,uStack_340);
          uStack_180 = CONCAT44(uStack_334,uStack_338);
          ppcStack_198 = ppcStack_350;
          uStack_190 = (code **)CONCAT44(uStack_344,uStack_348);
          ppcStack_1a8 = ppcStack_360;
          ppcStack_1a0 = ppcStack_358;
          puStack_160 = puVar9;
          uVar30 = puVar24[4];
          uVar27 = puVar24[7];
          if (uVar27 == uVar30) {
            FUN_140b5ad20(puStack_2c8,&UNK_142b188a0);
            uVar30 = puVar24[4];
            uVar27 = puVar24[7];
          }
          if (uVar27 + puVar24[6] < uVar30) {
            uVar30 = 0;
          }
          uVar23 = puVar24[5];
          lVar28 = ((uVar27 + puVar24[6]) - uVar30) * 0x60;
          puVar33 = (ulonglong *)(uVar23 + 0x50 + lVar28);
          *puVar33 = (ulonglong)pcStack_168;
          puVar33[1] = (ulonglong)puStack_160;
          plVar20 = (longlong *)(uVar23 + 0x40 + lVar28);
          *plVar20 = lStack_178;
          plVar20[1] = (longlong)ppcStack_170;
          puVar3 = (undefined8 *)(uVar23 + 0x30 + lVar28);
          *puVar3 = uStack_188;
          puVar3[1] = uStack_180;
          puVar3 = (undefined8 *)(uVar23 + 0x20 + lVar28);
          *puVar3 = ppcStack_198;
          puVar3[1] = uStack_190;
          puVar3 = (undefined8 *)(uVar23 + 0x10 + lVar28);
          *puVar3 = ppcStack_1a8;
          puVar3[1] = ppcStack_1a0;
          piVar2 = (int *)(uVar23 + lVar28);
          *piVar2 = iStack_1b8;
          piVar2[1] = iStack_1b4;
          piVar2[2] = (int)uStack_1b0;
          piVar2[3] = uStack_1b0._4_4_;
          puVar24[7] = puVar24[7] + 1;
          ppcVar22 = ppcStack_60;
        }
        FUN_140be14b0(ppcVar26,((ulonglong)((longlong)uStack_70 - (longlong)ppcVar26) >> 3) *
                               0x2e8ba2e8ba2e8ba3);
        lVar28 = lStack_d8;
        lVar19 = lStack_2c0;
        if (lStack_c8 != 0) {
          func_0x000140613c20(ppcStack_e0,lStack_c8 * 0x58,8);
        }
      }
      if (puStack_2d0 != puStack_c0) {
        iStack_1b8 = 0;
        iStack_1b4 = 0;
        uStack_4e = 1;
        FUN_1428d95d4(0,&puStack_2d0,&puStack_c0,&iStack_1b8,&UNK_142b187e8);
        goto code_r0x000140c0687b;
      }
      iStack_1b8 = (int)lStack_78;
      iStack_1b4 = (int)((ulonglong)lStack_78 >> 0x20);
      uStack_1b0._0_4_ = (int)lVar28;
      uStack_1b0._4_4_ = (uint)((ulonglong)lVar28 >> 0x20);
      ppcStack_1a8 = (code **)0x0;
      ppcStack_198 = (code **)0x0;
      uStack_4e = 1;
      FUN_1418b7b70(&ppcStack_298,&iStack_1b8,&UNK_142b182a0);
      uStack_430 = uStack_290;
      uStack_42c = uStack_28c;
      uStack_428 = uStack_288;
      uStack_424 = uStack_284;
      iStack_1b8 = (int)lStack_78;
      iVar18 = iStack_1b8;
      iStack_1b4 = (int)((ulonglong)lStack_78 >> 0x20);
      iVar13 = iStack_1b4;
      uStack_1b0._0_4_ = (int)lStack_d8;
      uVar16 = (int)uStack_1b0;
      uStack_1b0._4_4_ = (uint)((ulonglong)lStack_d8 >> 0x20);
      uVar14 = uStack_1b0._4_4_;
      ppcStack_1a8 = (code **)0x0;
      ppcStack_198 = (code **)0x0;
      uStack_63 = 1;
      FUN_1418b8760(&ppcStack_a8,&iStack_1b8,&UNK_142b182a0);
      uStack_440 = (undefined4)uStack_a0;
      uStack_43c = uStack_a0._4_4_;
      uStack_438 = (undefined4)uStack_98;
      uStack_434 = uStack_98._4_4_;
      ppcStack_1a8 = (code **)0x0;
      ppcStack_198 = (code **)0x0;
      uStack_64 = 1;
      iStack_1b8 = iVar18;
      iStack_1b4 = iVar13;
      uStack_1b0._0_4_ = uVar16;
      uStack_1b0._4_4_ = uVar14;
      FUN_1418b7e70(&lStack_4e0,&iStack_1b8,&UNK_142b182a0);
      uStack_450 = uStack_4d8;
      uStack_44c = uStack_4d4;
      uStack_448 = uStack_4d0;
      uStack_444 = uStack_4cc;
      uStack_65 = 1;
      func_0x000140d4a3b0(&pcStack_420);
      uStack_190 = ppcStack_250 + (longlong)ppcStack_90;
      uStack_188 = (code *)0x0;
      uStack_180 = 0;
      lStack_178 = 0;
      iStack_1b8 = (int)lStack_78;
      iStack_1b4 = (int)((ulonglong)lStack_78 >> 0x20);
      ppcStack_1a8 = ppcStack_250;
      ppcStack_1a0 = ppcStack_250;
      ppcStack_198 = ppcStack_90;
      pcStack_168 = (code *)&uStack_390;
      puStack_160 = &uStack_430;
      puStack_158 = &uStack_440;
      puStack_150 = &uStack_450;
      uStack_1b0._0_4_ = uVar16;
      uStack_1b0._4_4_ = uVar14;
      ppcStack_170 = &pcStack_420;
      FUN_1418b8f20(&ppcStack_248,&iStack_1b8,&UNK_142b182a0);
      uVar30 = uStack_80;
      cVar15 = func_0x000140c16a00();
      if (cVar15 == '\0') {
        lStack_278 = 0;
      }
      else {
        puVar29 = &UNK_142b187b1;
        puVar34 = &UNK_142b18790;
        do {
          bVar5 = puVar29[-1];
          uVar17 = (uint)(char)bVar5;
          if ((char)bVar5 < '\0') {
            bVar6 = puVar29[-2];
            if ((char)bVar6 < -0x40) {
              bVar7 = puVar29[-3];
              if ((char)bVar7 < -0x40) {
                pbVar1 = puVar29 + -4;
                puVar29 = puVar29 + -4;
                uVar17 = bVar7 & 0x3f | (*pbVar1 & 7) << 6;
              }
              else {
                puVar29 = puVar29 + -3;
                uVar17 = bVar7 & 0xf;
              }
              uVar17 = bVar6 & 0x3f | uVar17 << 6;
            }
            else {
              puVar29 = puVar29 + -2;
              uVar17 = bVar6 & 0x1f;
            }
            uVar17 = (uint)(bVar5 & 0x3f) | uVar17 << 6;
            if (uVar17 != 0x5c) goto code_r0x000140c05f94;
code_r0x000140c05faf:
            puVar34 = puVar29 + -0x142b1878f;
            if (puVar34 != (undefined *)0x0) {
              if (puVar34 < (undefined *)0x21) {
                if ((char)puVar29[1] < -0x40) {
code_r0x000140c05fd7:
                  FUN_1428d9940(&UNK_142b18790,0x21,puVar34,0x21,&UNK_142b184a8);
                  goto code_r0x000140c0687b;
                }
              }
              else if (puVar34 != (undefined *)0x21) goto code_r0x000140c05fd7;
            }
            lVar19 = 0x21 - (longlong)puVar34;
            puVar34 = puVar29 + 1;
            goto code_r0x000140c0609a;
          }
          puVar29 = puVar29 + -1;
          if (uVar17 == 0x5c) goto code_r0x000140c05faf;
code_r0x000140c05f94:
          if (uVar17 == 0x2f) goto code_r0x000140c05faf;
        } while (puVar29 != &UNK_142b18790);
        lVar19 = 0x21;
code_r0x000140c0609a:
        plVar20 = (longlong *)func_0x000140c18130();
        if (*plVar20 == 0) {
          ppcVar22 = (code **)FUN_1429090d0(plVar20,0);
          if (ppcVar22 == (code **)0x0) goto code_r0x000140c06844;
        }
        else {
          if ((int)*plVar20 != 1) {
code_r0x000140c06844:
            FUN_1428d80d0(&UNK_142d06498);
            goto code_r0x000140c0687b;
          }
          ppcVar22 = (code **)(plVar20 + 1);
        }
        if (*ppcVar22 != (code *)0x0) {
          FUN_1428d9250(&UNK_142d06508);
          goto code_r0x000140c0687b;
        }
        *ppcVar22 = (code *)0xffffffffffffffff;
        ppcStack_60 = ppcVar22;
        uStack_270 = FUN_140c16d40(ppcVar22 + 1,&UNK_142b18800,0xd,puVar34,lVar19,1,0);
        *ppcStack_60 = *ppcStack_60 + 1;
        lStack_278 = 1;
      }
      iStack_1b8 = CONCAT31(iStack_1b8._1_3_,0x1a);
      FUN_1418c1380(0x143634e40,&iStack_1b8,1);
      ppcVar22 = (code **)(uVar30 + 0x800);
      pcVar11 = *(code **)(uVar30 + 0x800);
      if (pcVar11 < (code *)0xfffffffffffffff0 && (char)(((uint)pcVar11 & 8) >> 3) == '\0') {
        LOCK();
        pcVar4 = *ppcVar22;
        if (pcVar11 == pcVar4) {
          *ppcVar22 = pcVar11 + 0x10;
        }
        UNLOCK();
        if (pcVar11 != pcVar4) goto code_r0x000140c06172;
      }
      else {
code_r0x000140c06172:
        FUN_1428cc4b0(ppcVar22,0);
      }
      FUN_140c81c70(ppcVar22);
      FUN_140c81c70((ulonglong)ppcVar22 | 1);
      puVar33 = puVar24 + 0xe;
      LOCK();
      bVar35 = (char)puVar24[0xe] == '\0';
      if (bVar35) {
        *(char *)(puVar24 + 0xe) = '\x01';
      }
      UNLOCK();
      ppcStack_60 = ppcVar22;
      if (!bVar35) {
        uStack_4f = 1;
        FUN_1428cd210(puVar33);
      }
      uStack_4f = 1;
      FUN_140c81c70(puVar33);
      iVar18 = (**(code **)(uStack_80 + 0x88))
                         (puVar24[0xf],(ulonglong)ppcStack_238 & 0xffffffff,pcStack_240,0);
      uStack_4f = 1;
      FUN_140c81e10();
      LOCK();
      bVar35 = (char)*puVar33 == '\x01';
      if (bVar35) {
        *(char *)puVar33 = '\0';
      }
      UNLOCK();
      if (!bVar35) {
        uStack_4f = 1;
        FUN_1428cdb90(puVar33,0);
      }
      uStack_4f = 0;
      FUN_140c81e10(ppcStack_60);
      uStack_4f = 0;
      FUN_140c81e10((ulonglong)ppcVar22 | 1);
      LOCK();
      pcVar11 = *ppcStack_60;
      *ppcStack_60 = *ppcStack_60 + -0x10;
      UNLOCK();
      if (((ulonglong)pcVar11 & 0xfffffffffffffff2) == 0x12) {
        uStack_4f = 0;
        FUN_1428ccf50(ppcStack_60);
      }
      uVar30 = uStack_80;
      if (iVar18 != 0) {
        FUN_140c84010(&uStack_1b0);
        ppcStack_198 = (code **)CONCAT44(ppcStack_198._4_4_,iVar18);
        iStack_1b8 = 9;
        iStack_1b4 = 0;
        FUN_140bf0860(&uStack_370,uVar30,&iStack_1b8);
        if (uStack_370 != (code *)0xb) {
          lStack_178 = CONCAT44(uStack_32c,uStack_330);
          ppcStack_170 = ppcStack_328;
          uStack_188 = (code *)CONCAT44(uStack_33c,uStack_340);
          uStack_180 = CONCAT44(uStack_334,uStack_338);
          ppcStack_198 = ppcStack_350;
          uStack_190 = (code **)CONCAT44(uStack_344,uStack_348);
          ppcStack_1a8 = ppcStack_360;
          ppcStack_1a0 = ppcStack_358;
          iStack_1b8 = (int)uStack_370;
          iStack_1b4 = uStack_370._4_4_;
          uStack_1b0._0_4_ = (int)uStack_368;
          uStack_1b0._4_4_ = uStack_368._4_4_;
          FUN_1428d9760(&UNK_142b1880d,0x18,&iStack_1b8,&UNK_142b182d8,&UNK_142b18828);
          goto code_r0x000140c0687b;
        }
      }
      if (lStack_278 != 0) {
        plVar20 = (longlong *)func_0x000140c18130();
        if (*plVar20 == 0) {
          ppcVar22 = (code **)FUN_1429090d0(plVar20,0);
          if (ppcVar22 == (code **)0x0) goto code_r0x000140c067f4;
        }
        else {
          if ((int)*plVar20 != 1) {
code_r0x000140c067f4:
            FUN_1428d80d0(&UNK_142d06498);
            goto code_r0x000140c0687b;
          }
          ppcVar22 = (code **)(plVar20 + 1);
        }
        if (*ppcVar22 != (code *)0x0) {
          FUN_1428d9250(&UNK_142d06508);
          goto code_r0x000140c0687b;
        }
        *ppcVar22 = (code *)0xffffffffffffffff;
        ppcStack_60 = ppcVar22;
        FUN_140c170f0(ppcVar22 + 1,uStack_270);
        *ppcStack_60 = *ppcStack_60 + 1;
      }
      FUN_1418c0a70(*(undefined8 *)(uVar30 + 0x648),uVar30);
      FUN_140bf0cc0(puVar24,uVar30);
      if (puStack_c0 == (undefined4 *)0x0) {
        FUN_1428d9310(&UNK_142b18840);
        goto code_r0x000140c0687b;
      }
      *puStack_260 = CONCAT44(uStack_84,uStack_88);
      puStack_260[1] = (longlong)puStack_c0 - 1;
      ppcVar22 = ppcRam000000014362c3e0;
      ppcVar26 = ppcStack_410;
      if (ppcStack_248 != (code **)0x0) {
        func_0x000140613c20(pcStack_240,(longlong)ppcStack_248 * 0x48,8);
        ppcVar22 = ppcRam000000014362c3e0;
        ppcVar26 = ppcStack_410;
      }
      while (ppcVar21 = ppcRam000000014362c3e0, ppcVar26 != ppcRam000000014362c3e0) {
        ppcVar10 = (code **)ppcVar26[3];
        ppcRam000000014362c3e0 = ppcVar22;
        func_0x000140613c20(*ppcVar26,ppcVar26[2],ppcVar26[1]);
        ppcVar22 = ppcRam000000014362c3e0;
        ppcVar26 = ppcVar10;
        ppcRam000000014362c3e0 = ppcVar21;
      }
      ppcRam000000014362c3e0 = ppcVar22;
      if (lStack_4e0 != 0) {
        func_0x000140613c20(CONCAT44(uStack_4d4,uStack_4d8),lStack_4e0 << 3,8);
      }
      lVar19 = lStack_78;
      if (ppcStack_a8 != (code **)0x0) {
        func_0x000140613c20(uStack_a0,(longlong)ppcStack_a8 << 2,4);
      }
      if (ppcStack_298 != (code **)0x0) {
        func_0x000140613c20(CONCAT44(uStack_28c,uStack_290),(longlong)ppcStack_298 << 3,8);
      }
      if (lStack_2b0 != 0) {
        func_0x000140613c20(uStack_2a8,lStack_2b0 << 3,8);
      }
      ppcStack_60 = (code **)0x0;
      while (ppcStack_90 != ppcStack_60) {
        ppcStack_60 = (code **)((longlong)ppcStack_60 + 1);
        FUN_140bdf860(lVar19);
        lVar19 = lVar19 + 0x90;
      }
      func_0x000140613c20(lStack_78,(longlong)ppcStack_90 * 0x90,8);
      uStack_4d = 0;
      FUN_140c81e10();
      LOCK();
      cVar15 = *pcStack_b0;
      if (cVar15 == '\x01') {
        *pcStack_b0 = '\0';
      }
      UNLOCK();
      if (cVar15 != '\x01') {
        uStack_4d = 0;
        FUN_1428cdb90(pcStack_b0,0);
      }
      if (cStack_61 == '\0') {
        return puStack_260;
      }
      plVar20 = (longlong *)func_0x000140c18130();
      if (*plVar20 == 0) {
        uStack_49 = 0;
        plVar20 = (longlong *)FUN_1429090d0(plVar20,0);
        if (plVar20 == (longlong *)0x0) goto code_r0x000140c06852;
      }
      else {
        if ((int)*plVar20 != 1) {
code_r0x000140c06852:
          uStack_49 = 0;
          FUN_1428d80d0(&UNK_142d06498,uStack_d0);
          goto code_r0x000140c0687b;
        }
        plVar20 = plVar20 + 1;
      }
      if (*plVar20 == 0) {
        *plVar20 = -1;
        ppcStack_60 = (code **)plVar20;
        FUN_140c170f0(plVar20 + 1);
        *ppcStack_60 = (code *)((longlong)*ppcStack_60 + 1);
        return puStack_260;
      }
      uStack_49 = 0;
      FUN_1428d9250(&UNK_142d06508);
      goto code_r0x000140c0687b;
    }
  }
  ppcStack_268 = ppcVar22;
  FUN_1428d9430(&UNK_142b18910,0x25,&UNK_142b18938);
code_r0x000140c0687b:
                    /* WARNING: Does not return */
  pcVar11 = (code *)invalidInstructionException();
  (*pcVar11)();
}

