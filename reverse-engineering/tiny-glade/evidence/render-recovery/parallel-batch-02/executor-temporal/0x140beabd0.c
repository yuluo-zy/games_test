
undefined8 *
FUN_140beabd0(undefined8 *param_1,ulonglong *param_2,longlong param_3,undefined8 param_4,
             undefined1 param_5,undefined1 param_6,undefined8 *param_7)

{
  byte *pbVar1;
  byte bVar2;
  byte bVar3;
  byte bVar4;
  code *pcVar5;
  undefined4 uVar6;
  undefined4 uVar7;
  undefined4 uVar8;
  undefined4 uVar9;
  undefined4 uVar10;
  undefined4 uVar11;
  undefined4 uVar12;
  undefined4 uVar13;
  undefined4 uVar14;
  undefined8 uVar15;
  undefined8 *puVar16;
  char cVar17;
  uint uVar18;
  undefined4 uVar19;
  int iVar20;
  longlong lVar21;
  longlong *plVar22;
  undefined4 *puVar23;
  ulonglong uVar24;
  ulonglong uVar25;
  undefined8 uVar26;
  ulonglong uVar27;
  undefined *puVar28;
  undefined *puVar29;
  undefined *puVar30;
  longlong lVar31;
  longlong in_stack_fffffffffffffbe8;
  undefined4 uVar32;
  longlong *plStack_400;
  code *pcStack_3f8;
  undefined8 uStack_3f0;
  undefined4 uStack_3e8;
  undefined4 uStack_3e4;
  undefined4 uStack_3e0;
  undefined4 uStack_3dc;
  undefined4 uStack_3d8;
  undefined4 uStack_3d4;
  undefined4 uStack_3d0;
  undefined4 uStack_3cc;
  undefined4 uStack_3c8;
  undefined4 uStack_3c4;
  undefined4 uStack_3c0;
  undefined4 uStack_3bc;
  undefined8 uStack_3b8;
  undefined8 uStack_3b0;
  undefined8 uStack_3a8;
  undefined8 uStack_3a0;
  undefined4 uStack_398;
  undefined4 uStack_394;
  undefined4 uStack_390;
  undefined4 uStack_38c;
  undefined4 *puStack_388;
  undefined4 uStack_380;
  undefined4 uStack_37c;
  undefined4 uStack_378;
  undefined4 uStack_374;
  undefined4 uStack_2f8;
  undefined4 uStack_2f4;
  undefined4 uStack_2f0;
  undefined4 uStack_2ec;
  undefined8 uStack_2e8;
  undefined8 uStack_2e0;
  undefined8 uStack_2d8;
  undefined4 uStack_2d0;
  undefined4 uStack_2cc;
  undefined4 uStack_2c8;
  undefined4 uStack_2c4;
  undefined4 uStack_2c0;
  undefined4 uStack_2bc;
  undefined4 uStack_2b8;
  undefined4 uStack_2b4;
  undefined4 uStack_2b0;
  undefined4 uStack_2ac;
  undefined4 uStack_2a8;
  undefined4 uStack_2a4;
  undefined8 uStack_2a0;
  undefined8 uStack_298;
  undefined8 uStack_290;
  undefined8 uStack_288;
  undefined4 uStack_280;
  undefined4 uStack_27c;
  undefined4 uStack_278;
  undefined4 uStack_274;
  undefined4 *puStack_270;
  longlong lStack_258;
  undefined4 uStack_250;
  undefined4 uStack_24c;
  undefined8 uStack_248;
  undefined8 uStack_240;
  undefined4 uStack_238;
  undefined4 uStack_234;
  undefined4 uStack_230;
  undefined4 uStack_22c;
  undefined4 uStack_228;
  undefined4 uStack_224;
  undefined4 uStack_220;
  undefined4 uStack_21c;
  undefined4 uStack_218;
  undefined4 uStack_214;
  undefined4 uStack_210;
  undefined4 uStack_20c;
  undefined8 uStack_208;
  undefined8 uStack_200;
  undefined8 uStack_1f8;
  undefined8 uStack_1f0;
  undefined8 uStack_1e8;
  undefined8 uStack_1e0;
  undefined8 uStack_1d8;
  undefined8 uStack_1d0;
  undefined8 uStack_1c8;
  undefined8 auStack_1c0 [5];
  undefined8 uStack_198;
  undefined8 uStack_190;
  undefined4 *apuStack_188 [17];
  uint uStack_100;
  undefined8 *puStack_f0;
  undefined8 uStack_e8;
  undefined8 uStack_e0;
  undefined8 uStack_d8;
  undefined8 uStack_d0;
  undefined8 uStack_c8;
  undefined8 uStack_c0;
  undefined4 uStack_b8;
  undefined4 uStack_b4;
  undefined4 uStack_b0;
  undefined4 uStack_ac;
  undefined4 uStack_a8;
  undefined4 uStack_a4;
  undefined4 uStack_a0;
  undefined4 uStack_9c;
  undefined4 uStack_98;
  undefined4 uStack_94;
  undefined4 uStack_90;
  undefined4 uStack_8c;
  undefined8 uStack_88;
  longlong *plStack_70;
  undefined8 uStack_68;
  undefined8 uStack_60;
  longlong *plStack_58;
  undefined1 uStack_4c;
  undefined1 uStack_4b;
  undefined1 uStack_4a;
  undefined1 uStack_49;
  undefined8 uStack_48;
  
  uStack_48 = 0xfffffffffffffffe;
  puStack_f0 = param_1;
  uStack_e8 = param_4;
  cVar17 = func_0x000140c16a00();
  uStack_60 = CONCAT71(uStack_60._1_7_,cVar17);
  if (cVar17 == '\0') {
    uStack_d8 = 0;
  }
  else {
    puVar29 = &UNK_142b14380;
    lVar31 = 0x38;
    uVar25 = 0x38;
    lVar21 = FUN_140be9720(&UNK_142b14380);
    if (lVar21 == 1) {
      if (uVar25 != 0) {
        if (uVar25 < 0x38) {
          if ((char)(&UNK_142b14380)[uVar25] < -0x40) {
code_r0x000140beac59:
            FUN_1428d9940(&UNK_142b14380,0x38,0,uVar25,&UNK_142b14478);
            goto code_r0x000140beb73f;
          }
        }
        else if (uVar25 != 0x38) goto code_r0x000140beac59;
      }
      puVar29 = &UNK_142b14380;
      lVar21 = FUN_140be9720(&UNK_142b14380);
      if (lVar21 == 1) {
        uVar27 = uVar25 + 2;
        if (uVar27 != 0) {
          if (uVar27 < 0x38) {
            if ((char)(&UNK_142b14382)[uVar25] < -0x40) {
code_r0x000140beacbc:
              FUN_1428d9940(&UNK_142b14380,0x38,uVar27,0x38,&UNK_142b14490);
              goto code_r0x000140beb73f;
            }
          }
          else if (uVar27 != 0x38) goto code_r0x000140beacbc;
        }
        lVar31 = 0x36 - uVar25;
        puVar29 = &UNK_142b14382 + uVar25;
      }
    }
    puVar28 = &UNK_142b14612;
    puVar30 = &UNK_142b145f0;
    do {
      bVar2 = puVar28[-1];
      uVar18 = (uint)(char)bVar2;
      if ((char)bVar2 < '\0') {
        bVar3 = puVar28[-2];
        if ((char)bVar3 < -0x40) {
          bVar4 = puVar28[-3];
          if ((char)bVar4 < -0x40) {
            pbVar1 = puVar28 + -4;
            puVar28 = puVar28 + -4;
            uVar18 = bVar4 & 0x3f | (*pbVar1 & 7) << 6;
          }
          else {
            puVar28 = puVar28 + -3;
            uVar18 = bVar4 & 0xf;
          }
          uVar18 = bVar3 & 0x3f | uVar18 << 6;
        }
        else {
          puVar28 = puVar28 + -2;
          uVar18 = bVar3 & 0x1f;
        }
        uVar18 = (uint)(bVar2 & 0x3f) | uVar18 << 6;
        if (uVar18 != 0x5c) goto code_r0x000140bead84;
code_r0x000140bead95:
        puVar30 = puVar28 + -0x142b145ef;
        if (puVar30 != (undefined *)0x0) {
          if (puVar30 < (undefined *)0x22) {
            if ((char)puVar28[1] < -0x40) {
code_r0x000140beadb1:
              FUN_1428d9940(&UNK_142b145f0,0x22,puVar30,0x22,&UNK_142b14458);
              goto code_r0x000140beb73f;
            }
          }
          else if (puVar30 != (undefined *)0x22) goto code_r0x000140beadb1;
        }
        in_stack_fffffffffffffbe8 = 0x22 - (longlong)puVar30;
        puVar30 = puVar28 + 1;
        goto code_r0x000140beade6;
      }
      puVar28 = puVar28 + -1;
      if (uVar18 == 0x5c) goto code_r0x000140bead95;
code_r0x000140bead84:
      if (uVar18 == 0x2f) goto code_r0x000140bead95;
    } while (puVar28 != &UNK_142b145f0);
    in_stack_fffffffffffffbe8 = 0x22;
code_r0x000140beade6:
    plVar22 = (longlong *)func_0x000140c18130();
    if (*plVar22 == 0) {
      plVar22 = (longlong *)FUN_1429090d0(plVar22,0);
      if (plVar22 == (longlong *)0x0) goto code_r0x000140beb725;
    }
    else {
      if ((int)*plVar22 != 1) {
code_r0x000140beb725:
        FUN_1428d80d0(&UNK_142d06498);
        goto code_r0x000140beb73f;
      }
      plVar22 = plVar22 + 1;
    }
    if (*plVar22 != 0) {
      FUN_1428d9250(&UNK_142d06508);
      goto code_r0x000140beb73f;
    }
    *plVar22 = -1;
    plStack_58 = plVar22;
    uStack_e0 = FUN_140c16d40(plVar22 + 1,puVar29,lVar31,puVar30,in_stack_fffffffffffffbe8,
                              &UNK_142b146c7,0x19);
    *plStack_58 = *plStack_58 + 1;
    uStack_d8 = 1;
    uStack_d0 = uStack_e0;
  }
  uStack_200 = (longlong *)CONCAT71(uStack_200._1_7_,0x1b);
  FUN_1418c1380(0x143634e40,&uStack_200,1);
  uVar32 = (undefined4)((ulonglong)in_stack_fffffffffffffbe8 >> 0x20);
  uStack_100 = 0;
  uVar25 = param_2[3];
  uVar18 = uStack_100;
  while (uStack_100 = uVar18, 1 < uVar25) {
    uVar27 = param_2[2];
    uVar24 = 0;
    if (*param_2 <= uVar27 + 1) {
      uVar24 = *param_2;
    }
    param_2[2] = (uVar27 - uVar24) + 1;
    uVar25 = uVar25 - 1;
    param_2[3] = uVar25;
    plVar22 = *(longlong **)(param_2[1] + uVar27 * 0x90);
    func_0x000142923c00(&uStack_2e8,param_2[1] + uVar27 * 0x90 + 8,0x88);
    if (plVar22 == (longlong *)0x8000000000000001) {
      FUN_1428d9310(&UNK_142b146e0);
      goto code_r0x000140beb73f;
    }
    plStack_400 = plVar22;
    plStack_58 = plVar22;
    func_0x000142923c00(&pcStack_3f8,&uStack_2e8,0x88);
    uStack_2e8 = (longlong *)CONCAT44(uStack_37c,uStack_380);
    uStack_2e0 = (code *)CONCAT44(uStack_374,uStack_378);
    uStack_4c = 1;
    FUN_140bfc780(param_3,&uStack_2e8);
    uVar32 = (undefined4)((ulonglong)in_stack_fffffffffffffbe8 >> 0x20);
    if (1 < uVar18) {
      uStack_2e8 = plStack_58;
      uStack_2e0 = pcStack_3f8;
      uStack_2d8 = uStack_3f0;
      uStack_2d0 = uStack_3e8;
      uStack_2cc = uStack_3e4;
      uStack_2c8 = uStack_3e0;
      uStack_2c4 = uStack_3dc;
      uStack_2c0 = uStack_3d8;
      uStack_2bc = uStack_3d4;
      uStack_2b8 = uStack_3d0;
      uStack_2b4 = uStack_3cc;
      uStack_2b0 = uStack_3c8;
      uStack_2ac = uStack_3c4;
      uStack_2a8 = uStack_3c0;
      uStack_2a4 = uStack_3bc;
      uStack_2a0 = uStack_3b8;
      uStack_298 = uStack_3b0;
      uStack_290 = uStack_3a8;
      uStack_288 = uStack_3a0;
      uStack_280 = uStack_398;
      uStack_27c = uStack_394;
      uStack_278 = uStack_390;
      uStack_274 = uStack_38c;
      puStack_270 = puStack_388;
      FUN_1428d9760(&UNK_142d06850,0x2b,&uStack_2e8,&UNK_142d068a0,&UNK_142b146f8);
      goto code_r0x000140beb73f;
    }
    uVar27 = (ulonglong)uVar18;
    (&uStack_200)[uVar27 * 0x10] = plStack_58;
    (&uStack_1f8)[uVar27 * 0x10] = pcStack_3f8;
    (&uStack_1f0)[uVar27 * 0x10] = uStack_3f0;
    (&uStack_1e8)[uVar27 * 0x10] = CONCAT44(uStack_3e4,uStack_3e8);
    (&uStack_1e0)[uVar27 * 0x10] = CONCAT44(uStack_3dc,uStack_3e0);
    (&uStack_1d8)[uVar27 * 0x10] = CONCAT44(uStack_3d4,uStack_3d8);
    (&uStack_1d0)[uVar27 * 0x10] = CONCAT44(uStack_3cc,uStack_3d0);
    (&uStack_1c8)[uVar27 * 0x10] = CONCAT44(uStack_3c4,uStack_3c8);
    auStack_1c0[uVar27 * 0x10] = CONCAT44(uStack_3bc,uStack_3c0);
    auStack_1c0[uVar27 * 0x10 + 1] = uStack_3b8;
    auStack_1c0[uVar27 * 0x10 + 2] = uStack_3b0;
    auStack_1c0[uVar27 * 0x10 + 3] = uStack_3a8;
    auStack_1c0[uVar27 * 0x10 + 4] = uStack_3a0;
    *(undefined4 *)(&uStack_198 + uVar27 * 0x10) = uStack_398;
    *(undefined4 *)((longlong)&uStack_198 + uVar27 * 0x80 + 4) = uStack_394;
    *(undefined4 *)(apuStack_188 + uVar27 * 0x10 + -1) = uStack_390;
    *(undefined4 *)((longlong)apuStack_188 + uVar27 * 0x80 + -4) = uStack_38c;
    apuStack_188[uVar27 * 0x10] = puStack_388;
    uVar18 = uVar18 + 1;
  }
  func_0x000142923c00(&plStack_400,&uStack_200,0x108);
  if ((char)uStack_60 != '\0') {
    plVar22 = (longlong *)func_0x000140c18130();
    if (*plVar22 == 0) {
      plVar22 = (longlong *)FUN_1429090d0(plVar22,0);
      if (plVar22 == (longlong *)0x0) goto code_r0x000140beb733;
    }
    else {
      if ((int)*plVar22 != 1) {
code_r0x000140beb733:
        FUN_1428d80d0(&UNK_142d06498);
        goto code_r0x000140beb73f;
      }
      plVar22 = plVar22 + 1;
    }
    if (*plVar22 != 0) {
      FUN_1428d9250(&UNK_142d06508);
      goto code_r0x000140beb73f;
    }
    *plVar22 = -1;
    plStack_58 = plVar22;
    FUN_140c170f0(plVar22 + 1,uStack_e0);
    *plStack_58 = *plStack_58 + 1;
  }
  uStack_4b = 0;
  uVar26 = CONCAT44(uVar32,*(undefined4 *)(param_3 + 0xad4));
  FUN_140bef4a0(&lStack_258,param_3,&UNK_142b14780,0xe,uVar26);
  uVar32 = (undefined4)((ulonglong)uVar26 >> 0x20);
  uStack_49 = 1;
  uStack_4a = 0;
  FUN_140bed230(&lStack_258,param_3,0);
  uVar25 = param_2[0x19];
  uVar26 = CONCAT44(uStack_20c,uStack_210);
  (**(code **)(param_3 + 0x3f8))(uVar26,uVar25,0,2);
  (**(code **)(param_3 + 0x400))(uVar26,1,uVar25,0);
  if (*(char *)((longlong)param_7 + 0x4c) == '\x02') {
    plVar22 = (longlong *)0x8000000000000000;
  }
  else {
    uStack_49 = 1;
    uStack_4a = 0;
    FUN_140c11090(&uStack_200,param_3 + 0x6f8,param_3 + 0x78,CONCAT44(uStack_20c,uStack_210));
    uVar19 = FUN_140c110f0(&uStack_200,&UNK_142b1478e,0xb);
    FUN_140c11150(&uStack_200,param_3 + 0x78,CONCAT44(uStack_20c,uStack_210),uVar19);
    uStack_2f8 = (undefined4)uStack_1f0;
    uStack_2f4 = uStack_1f0._4_4_;
    uStack_2f0 = (undefined4)uStack_1e8;
    uStack_2ec = uStack_1e8._4_4_;
    plVar22 = uStack_200;
    uVar26 = uStack_1f8;
  }
  uStack_200 = (longlong *)CONCAT71(uStack_200._1_7_,0x18);
  uStack_49 = 1;
  uStack_4a = 1;
  plStack_70 = plVar22;
  uStack_68 = uVar26;
  FUN_1418c1380(0x143634e40,&uStack_200,1);
  if ((int)lStack_258 == 1) {
    uStack_60 = uVar26;
    plStack_58 = plVar22;
    iVar20 = (**(code **)(param_3 + 0x2c8))(CONCAT44(uStack_20c,uStack_210));
    if (iVar20 == 0) {
      lStack_258 = 2;
      uStack_200 = (longlong *)0x0;
      uStack_1f8 = 8;
      uStack_1f0 = (undefined8 *)0x0;
      uStack_1e8 = 0;
      uStack_1e0 = 4;
      uStack_1d8 = 0;
      uStack_1d0 = 0;
      uStack_1c8 = 8;
      auStack_1c0[0] = 0;
      auStack_1c0[1] = 0;
      auStack_1c0[2] = 8;
      auStack_1c0[3] = 0;
      auStack_1c0[4] = 0;
      uStack_198 = 8;
      uStack_190 = 0;
      apuStack_188[0] = (undefined4 *)0x0;
      apuStack_188[1] = (undefined4 *)0x8;
      apuStack_188[2] = (undefined4 *)0x0;
      uStack_88 = uStack_208;
      uStack_98 = uStack_218;
      uStack_94 = uStack_214;
      uStack_90 = uStack_210;
      uStack_8c = uStack_20c;
      uStack_a8 = uStack_228;
      uStack_a4 = uStack_224;
      uStack_a0 = uStack_220;
      uStack_9c = uStack_21c;
      uStack_b8 = uStack_238;
      uStack_b4 = uStack_234;
      uStack_b0 = uStack_230;
      uStack_ac = uStack_22c;
      uStack_c8 = uStack_248;
      uStack_c0 = uStack_240;
      uStack_d8._0_4_ = 2;
      uStack_d8._4_4_ = 0;
      uStack_d0._0_4_ = uStack_250;
      uStack_d0._4_4_ = uStack_24c;
      FUN_140562f30(apuStack_188,&UNK_142d06710);
      *(undefined8 *)(apuStack_188[1] + 0x14) = uStack_88;
      *(ulonglong *)(apuStack_188[1] + 0x10) = CONCAT44(uStack_94,uStack_98);
      *(ulonglong *)(apuStack_188[1] + 0x12) = CONCAT44(uStack_8c,uStack_90);
      apuStack_188[1][0xc] = uStack_a8;
      apuStack_188[1][0xd] = uStack_a4;
      apuStack_188[1][0xe] = uStack_a0;
      apuStack_188[1][0xf] = uStack_9c;
      apuStack_188[1][8] = uStack_b8;
      apuStack_188[1][9] = uStack_b4;
      apuStack_188[1][10] = uStack_b0;
      apuStack_188[1][0xb] = uStack_ac;
      apuStack_188[1][4] = (undefined4)uStack_c8;
      apuStack_188[1][5] = uStack_c8._4_4_;
      apuStack_188[1][6] = (undefined4)uStack_c0;
      apuStack_188[1][7] = uStack_c0._4_4_;
      *apuStack_188[1] = (undefined4)uStack_d8;
      apuStack_188[1][1] = uStack_d8._4_4_;
      apuStack_188[1][2] = (undefined4)uStack_d0;
      apuStack_188[1][3] = uStack_d0._4_4_;
      apuStack_188[2] = (undefined4 *)0x1;
      func_0x000142923c00(&uStack_2e8,&uStack_200,0x90);
      FUN_140bef4a0(&uStack_200,param_3,&UNK_142b14799,0xe,
                    CONCAT44(uVar32,*(undefined4 *)(param_3 + 0xad4)));
      FUN_140bed230(&uStack_200,param_3,0);
      uStack_d8 = 0;
      uStack_d0 = 8;
      uStack_c8 = 0;
      puVar23 = (undefined4 *)func_0x000140613c10(0x58,8);
      puVar16 = puStack_f0;
      if (puVar23 != (undefined4 *)0x0) {
        *(undefined8 *)(puVar23 + 0x14) = auStack_1c0[2];
        *(undefined8 *)(puVar23 + 0x10) = auStack_1c0[0];
        *(undefined8 *)(puVar23 + 0x12) = auStack_1c0[1];
        puVar23[0xc] = (undefined4)uStack_1d0;
        puVar23[0xd] = uStack_1d0._4_4_;
        puVar23[0xe] = (undefined4)uStack_1c8;
        puVar23[0xf] = uStack_1c8._4_4_;
        puVar23[8] = (undefined4)uStack_1e0;
        puVar23[9] = uStack_1e0._4_4_;
        puVar23[10] = (undefined4)uStack_1d8;
        puVar23[0xb] = uStack_1d8._4_4_;
        puVar23[4] = (undefined4)uStack_1f0;
        puVar23[5] = uStack_1f0._4_4_;
        puVar23[6] = (undefined4)uStack_1e8;
        puVar23[7] = uStack_1e8._4_4_;
        *puVar23 = (undefined4)uStack_200;
        puVar23[1] = uStack_200._4_4_;
        puVar23[2] = (undefined4)uStack_1f8;
        puVar23[3] = uStack_1f8._4_4_;
        func_0x000142923c00(puStack_f0 + 0xb,&uStack_2e8,0x90);
        puVar16[0x1d] = plStack_58;
        puVar16[0x1e] = uStack_60;
        puVar16[0x1f] = CONCAT44(uStack_2f4,uStack_2f8);
        puVar16[0x20] = CONCAT44(uStack_2ec,uStack_2f0);
        puVar16[0x2d] = param_7[10];
        uVar26 = param_7[9];
        puVar16[0x2b] = param_7[8];
        puVar16[0x2c] = uVar26;
        uVar26 = *param_7;
        uVar15 = param_7[1];
        uVar32 = *(undefined4 *)(param_7 + 2);
        uVar19 = *(undefined4 *)((longlong)param_7 + 0x14);
        uVar6 = *(undefined4 *)(param_7 + 3);
        uVar7 = *(undefined4 *)((longlong)param_7 + 0x1c);
        uVar8 = *(undefined4 *)(param_7 + 4);
        uVar9 = *(undefined4 *)((longlong)param_7 + 0x24);
        uVar10 = *(undefined4 *)(param_7 + 5);
        uVar11 = *(undefined4 *)((longlong)param_7 + 0x2c);
        uVar12 = *(undefined4 *)((longlong)param_7 + 0x34);
        uVar13 = *(undefined4 *)(param_7 + 7);
        uVar14 = *(undefined4 *)((longlong)param_7 + 0x3c);
        *(undefined4 *)(puVar16 + 0x29) = *(undefined4 *)(param_7 + 6);
        *(undefined4 *)((longlong)puVar16 + 0x14c) = uVar12;
        *(undefined4 *)(puVar16 + 0x2a) = uVar13;
        *(undefined4 *)((longlong)puVar16 + 0x154) = uVar14;
        *(undefined4 *)(puVar16 + 0x27) = uVar8;
        *(undefined4 *)((longlong)puVar16 + 0x13c) = uVar9;
        *(undefined4 *)(puVar16 + 0x28) = uVar10;
        *(undefined4 *)((longlong)puVar16 + 0x144) = uVar11;
        *(undefined4 *)(puVar16 + 0x25) = uVar32;
        *(undefined4 *)((longlong)puVar16 + 300) = uVar19;
        *(undefined4 *)(puVar16 + 0x26) = uVar6;
        *(undefined4 *)((longlong)puVar16 + 0x134) = uVar7;
        puVar16[0x23] = uVar26;
        puVar16[0x24] = uVar15;
        *(undefined4 *)(puVar16 + 5) = (undefined4)uStack_d8;
        *(undefined4 *)((longlong)puVar16 + 0x2c) = uStack_d8._4_4_;
        *(undefined4 *)(puVar16 + 6) = (undefined4)uStack_d0;
        *(undefined4 *)((longlong)puVar16 + 0x34) = uStack_d0._4_4_;
        puVar16[7] = uStack_c8;
        puVar16[8] = 1;
        puVar16[9] = puVar23;
        puVar16[10] = 1;
        func_0x000142923c00(puVar16 + 0x32,&plStack_400,0x108);
        *puVar16 = 0;
        puVar16[3] = 0;
        *(undefined4 *)(puVar16 + 0x21) = 2;
        puVar16[0x2e] = param_2;
        puVar16[0x2f] = param_3;
        puVar16[0x30] = uStack_e8;
        *(undefined1 *)(puVar16 + 0x31) = param_5;
        *(undefined1 *)((longlong)puVar16 + 0x189) = param_6;
        return puVar16;
      }
      FUN_1428d9000(8,0x58);
    }
    else {
      uStack_200 = (longlong *)CONCAT44(uStack_200._4_4_,iVar20);
      uStack_68 = uStack_60;
      plStack_70 = plStack_58;
      uStack_49 = 1;
      uStack_4a = 1;
      FUN_1428d9760(&UNK_142b14d20,0x12,&uStack_200,&UNK_142b14b30,&UNK_142b14d38);
    }
  }
  else {
    uStack_2e0 = FUN_140bed620;
    uStack_200 = (longlong *)&UNK_142b14d78;
    uStack_1f8 = 1;
    uStack_1e0 = 0;
    uStack_1e8 = 1;
    uStack_49 = 1;
    uStack_4a = 1;
    uStack_2e8 = &lStack_258;
    uStack_1f0 = &uStack_2e8;
    plStack_70 = plVar22;
    uStack_68 = uVar26;
    FUN_1428d9390(&uStack_200,&UNK_142b14d88);
  }
code_r0x000140beb73f:
                    /* WARNING: Does not return */
  pcVar5 = (code *)invalidInstructionException();
  (*pcVar5)();
}

