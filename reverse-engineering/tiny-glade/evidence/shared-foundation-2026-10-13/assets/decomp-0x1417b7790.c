
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

undefined1 (*) [16] FUN_1417b7790(undefined1 (*param_1) [16],char param_2,undefined1 param_3)

{
  undefined1 (*pauVar1) [16];
  undefined1 auVar2 [16];
  code *pcVar3;
  undefined1 (*pauVar4) [16];
  undefined1 (*pauVar5) [16];
  byte *pbVar6;
  undefined8 uVar7;
  undefined8 *puVar8;
  ulonglong uVar9;
  longlong lVar10;
  undefined *puVar11;
  undefined1 (*pauVar12) [16];
  undefined1 (*pauVar13) [16];
  undefined1 (*pauStack_388) [16];
  undefined1 (*pauStack_380) [16];
  undefined8 uStack_378;
  undefined8 uStack_370;
  undefined4 uStack_368;
  undefined4 uStack_364;
  undefined4 uStack_360;
  undefined4 uStack_35c;
  undefined4 uStack_358;
  undefined4 uStack_354;
  undefined4 uStack_350;
  undefined4 uStack_34c;
  undefined4 uStack_348;
  undefined4 uStack_344;
  undefined4 uStack_340;
  undefined4 uStack_33c;
  undefined1 (**ppauStack_338) [16];
  longlong lStack_330;
  undefined8 uStack_328;
  undefined8 uStack_318;
  undefined1 (*pauStack_310) [16];
  longlong lStack_308;
  undefined1 (*pauStack_300) [16];
  undefined1 (*pauStack_2f0) [16];
  undefined1 (*pauStack_2e8) [16];
  undefined1 (*pauStack_2e0) [16];
  undefined1 (*pauStack_2d8) [16];
  undefined4 uStack_2d0;
  undefined4 uStack_2cc;
  undefined4 uStack_2c8;
  undefined4 uStack_2c4;
  undefined8 uStack_2c0;
  undefined1 (*pauStack_2b8) [16];
  longlong lStack_2b0;
  longlong lStack_2a8;
  undefined *puStack_2a0;
  undefined1 (*pauStack_298) [16];
  longlong lStack_290;
  undefined8 uStack_288;
  longlong lStack_270;
  undefined8 uStack_268;
  longlong lStack_250;
  undefined1 (*pauStack_248) [16];
  undefined1 (*pauStack_240) [16];
  undefined1 (*pauStack_238) [16];
  undefined1 (*pauStack_230) [16];
  undefined8 uStack_228;
  undefined1 (*pauStack_220) [16];
  longlong lStack_218;
  undefined1 (*pauStack_210) [16];
  undefined1 (*pauStack_208) [16];
  undefined1 (*pauStack_200) [16];
  undefined4 uStack_1f8;
  undefined4 uStack_1f4;
  undefined4 uStack_1f0;
  undefined4 uStack_1ec;
  undefined4 uStack_1e8;
  undefined4 uStack_1e4;
  undefined4 uStack_1e0;
  undefined4 uStack_1dc;
  undefined4 uStack_1d8;
  undefined4 uStack_1d4;
  undefined4 uStack_1d0;
  undefined4 uStack_1cc;
  undefined8 uStack_1c8;
  undefined8 uStack_1c0;
  undefined1 (**ppauStack_1b8) [16];
  undefined1 (*pauStack_1a8) [16];
  undefined1 (*pauStack_1a0) [16];
  undefined8 uStack_198;
  undefined1 (*pauStack_190) [16];
  longlong lStack_188;
  undefined1 (*pauStack_180) [16];
  undefined1 (*pauStack_178) [16];
  undefined1 (*pauStack_170) [16];
  undefined8 uStack_168;
  undefined1 (*pauStack_160) [16];
  undefined1 (*pauStack_158) [16];
  undefined1 (*pauStack_150) [16];
  undefined1 (*pauStack_148) [16];
  undefined1 (*pauStack_140) [16];
  undefined1 (*pauStack_138) [16];
  undefined4 uStack_130;
  undefined4 uStack_12c;
  undefined4 uStack_128;
  undefined4 uStack_124;
  undefined4 uStack_120;
  undefined4 uStack_11c;
  undefined4 uStack_118;
  undefined4 uStack_114;
  undefined4 uStack_110;
  undefined4 uStack_10c;
  undefined1 (**ppauStack_108) [16];
  undefined8 uStack_100;
  undefined *puStack_f8;
  undefined1 (*pauStack_e8) [16];
  undefined1 (*pauStack_e0) [16];
  undefined1 (*pauStack_d8) [16];
  undefined8 uStack_d0;
  undefined8 uStack_c8;
  undefined8 uStack_c0;
  undefined8 uStack_b8;
  undefined8 uStack_b0;
  undefined8 uStack_a8;
  undefined8 uStack_a0;
  undefined *puStack_98;
  undefined8 uStack_90;
  undefined1 (**ppauStack_88) [16];
  undefined8 uStack_80;
  undefined8 uStack_78;
  undefined *puStack_68;
  undefined1 (*pauStack_60) [16];
  undefined1 (*pauStack_58) [16];
  undefined1 uStack_4d;
  undefined1 uStack_4c;
  undefined1 uStack_4b;
  undefined1 uStack_4a;
  undefined1 uStack_49;
  undefined8 uStack_48;
  
  uStack_48 = 0xfffffffffffffffe;
  pauStack_150 = (undefined1 (*) [16])CONCAT71(pauStack_150._1_7_,param_3);
  puVar11 = &UNK_142cdf5c4;
  FUN_140d61b00(&lStack_270,&UNK_142cdf5c4,0x15);
  pauVar5 = (undefined1 (*) [16])FUN_140d76ed0(&lStack_270);
  FUN_140d6a710(&pauStack_e8,pauVar5,puVar11);
  if ((int)pauStack_e8 == 2) {
    if (((uint)pauStack_e0 & 3) == 1) {
      puStack_68 = pauStack_e0[-1] + 0xf;
      pauStack_58 = *(undefined1 (**) [16])(pauStack_e0[-1] + 0xf);
      pauStack_60 = *(undefined1 (**) [16])(*pauStack_e0 + 7);
      if (*(code **)*pauStack_60 != (code *)0x0) {
        (**(code **)*pauStack_60)(pauStack_58);
      }
      if (*(longlong *)(*pauStack_60 + 8) != 0) {
        func_0x000140613c20(pauStack_58,*(longlong *)(*pauStack_60 + 8),
                            *(undefined8 *)pauStack_60[1]);
      }
      pauVar5 = (undefined1 (*) [16])0x18;
      func_0x000140613c20(puStack_68,0x18,8);
    }
    pbVar6 = (byte *)FUN_140d4d740();
    if ((*pbVar6 & 1) == 0) {
      lStack_218 = FUN_140d64e00();
      pbVar6[0] = 1;
      pbVar6[1] = 0;
      pbVar6[2] = 0;
      pbVar6[3] = 0;
      pbVar6[4] = 0;
      pbVar6[5] = 0;
      pbVar6[6] = 0;
      pbVar6[7] = 0;
      *(longlong *)(pbVar6 + 8) = lStack_218;
      *(undefined1 (**) [16])(pbVar6 + 0x10) = pauVar5;
    }
    else {
      lStack_218 = *(longlong *)(pbVar6 + 8);
      pauVar5 = *(undefined1 (**) [16])(pbVar6 + 0x10);
    }
    *(longlong *)(pbVar6 + 8) = lStack_218 + 1;
    uStack_228 = _DAT_142cdf560;
    pauStack_220 = _UNK_142cdf568;
    pauStack_238 = _DAT_142cdf550;
    pauStack_230 = _UNK_142cdf558;
    pauStack_210 = pauVar5;
  }
  else {
    uVar7 = FUN_140d76ed0(&lStack_270);
    FUN_140d59140((undefined1 (*) [16])&pauStack_208,uVar7,pauVar5);
    pauVar5 = pauStack_200;
    if (SBORROW8(0,(longlong)pauStack_208)) {
      pauStack_1a8 = pauStack_200;
      uStack_4b = 1;
      FUN_140d56700(&pauStack_e8);
      uStack_128 = (undefined4)uStack_c8;
      uStack_124 = uStack_c8._4_4_;
      uStack_120 = (undefined4)uStack_c0;
      uStack_11c = uStack_c0._4_4_;
      pauStack_138 = pauStack_d8;
      uStack_130 = (undefined4)uStack_d0;
      uStack_12c = uStack_d0._4_4_;
      pauStack_148 = pauStack_e8;
      pauStack_140 = pauStack_e0;
      pauStack_e8 = (undefined1 (*) [16])&UNK_142fdbea8;
      pauStack_e0 = (undefined1 (*) [16])0x15;
      pauStack_d8 = pauVar5;
      uStack_4b = 0;
      pauVar5 = (undefined1 (*) [16])&pauStack_148;
      pauStack_230 = (undefined1 (*) [16])FUN_1429155a0(&pauStack_e8);
    }
    else {
      pauStack_58 = pauStack_208;
      pauStack_d8 = (undefined1 (*) [16])0x0;
      pauStack_e8 = (undefined1 (*) [16])0x1;
      pauStack_e0 = (undefined1 (*) [16])0x80;
      pauStack_60 = pauStack_200;
      FUN_1420d7b80(&pauStack_148,&pauStack_e8,pauStack_200,CONCAT44(uStack_1f4,uStack_1f8));
      if ((char)pauStack_148 == '2') {
        lStack_308 = CONCAT44(uStack_11c,uStack_120);
        pauStack_300 = (undefined1 (*) [16])CONCAT44(uStack_114,uStack_118);
        uStack_318 = CONCAT44(uStack_12c,uStack_130);
        pauStack_310 = (undefined1 (*) [16])CONCAT44(uStack_124,uStack_128);
        pauStack_230 = pauStack_138;
        pauStack_238 = pauStack_140;
        pauVar5 = pauStack_58;
      }
      else {
        pauStack_208 = (undefined1 (*) [16])CONCAT44(uStack_12c,uStack_130);
        pauStack_200 = (undefined1 (*) [16])CONCAT44(uStack_124,uStack_128);
        uStack_1f8 = uStack_120;
        uStack_1f4 = uStack_11c;
        uStack_1f0 = uStack_118;
        uStack_1ec = uStack_114;
        uStack_b0 = (undefined1 (*) [16])CONCAT44(uStack_10c,uStack_110);
        uStack_a8 = ppauStack_108;
        uStack_a0 = uStack_100;
        puStack_98 = puStack_f8;
        pauStack_e0 = pauStack_140;
        pauStack_d8 = pauStack_138;
        uStack_c0 = (undefined *)CONCAT44(uStack_11c,uStack_120);
        uStack_b8 = CONCAT44(uStack_114,uStack_118);
        pauStack_e8 = pauStack_148;
        uStack_d0 = pauStack_208;
        uStack_c8 = pauStack_200;
        pauStack_230 = (undefined1 (*) [16])FUN_142915a00(&pauStack_e8);
        pauStack_238 = (undefined1 (*) [16])0x0;
        pauVar5 = pauStack_58;
      }
      pauStack_58 = pauVar5;
      if (pauVar5 != (undefined1 (*) [16])0x0) {
        func_0x000140613c20(pauStack_60,pauVar5,1);
      }
      if (pauStack_238 != (undefined1 (*) [16])0x0) {
        uStack_228 = uStack_318;
        pauStack_220 = pauStack_310;
        lStack_218 = lStack_308;
        pauStack_210 = pauStack_300;
        goto LAB_1417b7bf9;
      }
    }
    pauStack_208 = pauStack_230;
    if (1 < *_DAT_14362c3a8) {
      pauStack_140 = (undefined1 (*) [16])&UNK_140d4cc20;
      pauStack_148 = (undefined1 (*) [16])&pauStack_208;
      puVar8 = (undefined8 *)asset_recovery_14067dd80(&UNK_142cdf710);
      uStack_c8 = (undefined1 (*) [16])*puVar8;
      uStack_c0 = (undefined *)puVar8[1];
      uStack_b8 = 2;
      uStack_b0 = (undefined1 (*) [16])&UNK_142cdf658;
      uStack_a8 = (undefined1 (**) [16])0x13;
      puStack_98 = &UNK_142cdf700;
      uStack_90 = 1;
      ppauStack_88 = &pauStack_148;
      uStack_80 = 1;
      uStack_78 = 0;
      pauStack_e8 = (undefined1 (*) [16])0x0;
      pauStack_e0 = (undefined1 (*) [16])&UNK_142cdf658;
      pauStack_d8 = (undefined1 (*) [16])0x13;
      uStack_d0 = (undefined1 (*) [16])0x0;
      uStack_a0 = CONCAT44(*(undefined4 *)(puVar8 + 2),1);
      pauVar5 = (undefined1 (*) [16])&pauStack_e8;
      func_0x000140d4a330(&pauStack_178);
    }
    pbVar6 = (byte *)FUN_140d4d740();
    if ((*pbVar6 & 1) == 0) {
      lStack_218 = FUN_140d64e00();
      pbVar6[0] = 1;
      pbVar6[1] = 0;
      pbVar6[2] = 0;
      pbVar6[3] = 0;
      pbVar6[4] = 0;
      pbVar6[5] = 0;
      pbVar6[6] = 0;
      pbVar6[7] = 0;
      *(longlong *)(pbVar6 + 8) = lStack_218;
      *(undefined1 (**) [16])(pbVar6 + 0x10) = pauVar5;
    }
    else {
      lStack_218 = *(longlong *)(pbVar6 + 8);
      pauVar5 = *(undefined1 (**) [16])(pbVar6 + 0x10);
    }
    *(longlong *)(pbVar6 + 8) = lStack_218 + 1;
    uStack_228 = _DAT_142cdf560;
    pauStack_220 = _UNK_142cdf568;
    pauStack_238 = _DAT_142cdf550;
    pauStack_230 = _UNK_142cdf558;
    pauStack_210 = pauVar5;
    func_0x000140d4cc30(&pauStack_208);
  }
LAB_1417b7bf9:
  puVar11 = &UNK_142cdf570;
  FUN_140d61b00(&lStack_290,&UNK_142cdf570,0x13);
  uVar7 = FUN_140d76ed0(&lStack_290);
  FUN_1420d8ba0(&pauStack_e8,uVar7,puVar11);
  pauVar4 = uStack_d0;
  pauVar13 = pauStack_e0;
  pauVar5 = pauStack_e8;
  if (pauStack_e8 == (undefined1 (*) [16])0x0) {
    *(undefined1 (**) [16])(*param_1 + 8) = pauStack_e0;
    *(undefined8 *)*param_1 = 0;
    pauStack_e8 = (undefined1 (*) [16])0x0;
  }
  else {
    uStack_2d0 = (undefined4)uStack_c8;
    uStack_2cc = uStack_c8._4_4_;
    uStack_2c8 = (undefined4)uStack_c0;
    uStack_2c4 = uStack_c0._4_4_;
    pauStack_2e0 = pauStack_d8;
    pauStack_2d8 = uStack_d0;
    pauStack_2f0 = pauStack_e8;
    pauStack_2e8 = pauStack_e0;
    pauStack_d8 = (undefined1 (*) [16])(*pauStack_e8 + 1 + (longlong)pauStack_e0);
    auVar2 = *pauStack_e8;
    pauVar1 = pauStack_e8 + 1;
    uStack_d0 = (undefined1 (*) [16])
                CONCAT62(uStack_d0._2_6_,
                         ~((ushort)(SUB161(auVar2 >> 7,0) & 1) |
                           (ushort)(SUB161(auVar2 >> 0xf,0) & 1) << 1 |
                           (ushort)(SUB161(auVar2 >> 0x17,0) & 1) << 2 |
                           (ushort)(SUB161(auVar2 >> 0x1f,0) & 1) << 3 |
                           (ushort)(SUB161(auVar2 >> 0x27,0) & 1) << 4 |
                           (ushort)(SUB161(auVar2 >> 0x2f,0) & 1) << 5 |
                           (ushort)(SUB161(auVar2 >> 0x37,0) & 1) << 6 |
                           (ushort)(SUB161(auVar2 >> 0x3f,0) & 1) << 7 |
                           (ushort)(SUB161(auVar2 >> 0x47,0) & 1) << 8 |
                           (ushort)(SUB161(auVar2 >> 0x4f,0) & 1) << 9 |
                           (ushort)(SUB161(auVar2 >> 0x57,0) & 1) << 10 |
                           (ushort)(SUB161(auVar2 >> 0x5f,0) & 1) << 0xb |
                           (ushort)(SUB161(auVar2 >> 0x67,0) & 1) << 0xc |
                           (ushort)(SUB161(auVar2 >> 0x6f,0) & 1) << 0xd |
                           (ushort)(SUB161(auVar2 >> 0x77,0) & 1) << 0xe |
                          (ushort)(byte)(auVar2[0xf] >> 7) << 0xf));
    uStack_c8 = pauVar4;
    uStack_4c = 1;
    pauVar12 = (undefined1 (*) [16])&pauStack_e8;
    pauStack_e0 = pauVar1;
    pauStack_60 = param_1;
    FUN_1420dbb30(&lStack_2a8,pauVar12,&UNK_142cdda30);
    puStack_68 = puStack_2a0;
    if ((undefined1 (*) [16])0x1 < pauStack_298) {
      pauVar12 = pauStack_298;
      if (pauStack_298 < (undefined1 (*) [16])0x15) {
        uStack_49 = 1;
        FUN_1420deb50(puStack_2a0,pauStack_298,1,&pauStack_178);
      }
      else {
        uStack_49 = 1;
        FUN_1420dfa70(puStack_2a0,pauStack_298,&pauStack_178);
      }
    }
    if (param_2 == '\0') {
      pauStack_208 = pauVar4;
      pauStack_148 = (undefined1 (*) [16])&pauStack_208;
      pauStack_140 = (undefined1 (*) [16])FUN_140d8d6c0;
      pauStack_e8 = (undefined1 (*) [16])&UNK_142cdf5f0;
      pauStack_e0 = (undefined1 (*) [16])0x2;
      uStack_c8 = (undefined1 (*) [16])0x0;
      pauStack_d8 = (undefined1 (*) [16])&pauStack_148;
      uStack_d0 = (undefined1 (*) [16])0x1;
      uStack_49 = 1;
      FUN_140d5c990(&pauStack_e8);
    }
    else if (2 < *_DAT_14362c3a8) {
      pauStack_208 = pauVar4;
      pauStack_148 = (undefined1 (*) [16])&pauStack_208;
      pauStack_140 = (undefined1 (*) [16])FUN_140d8d6c0;
      uStack_49 = 1;
      puVar8 = (undefined8 *)asset_recovery_14067dd80(&UNK_142cdf640);
      uStack_c8 = (undefined1 (*) [16])*puVar8;
      uStack_c0 = (undefined *)puVar8[1];
      uStack_b8 = 3;
      uStack_b0 = (undefined1 (*) [16])&UNK_142cdf658;
      uStack_a8 = (undefined1 (**) [16])0x13;
      puStack_98 = &UNK_142cdf620;
      uStack_90 = 2;
      ppauStack_88 = &pauStack_148;
      uStack_80 = 1;
      uStack_78 = 0;
      pauStack_e8 = (undefined1 (*) [16])0x0;
      pauStack_e0 = (undefined1 (*) [16])&UNK_142cdf658;
      pauStack_d8 = (undefined1 (*) [16])0x13;
      uStack_d0 = (undefined1 (*) [16])0x0;
      uStack_a0 = CONCAT44(*(undefined4 *)(puVar8 + 2),1);
      uStack_49 = 1;
      pauVar12 = (undefined1 (*) [16])&pauStack_e8;
      func_0x000140d4a330(&pauStack_178);
    }
    pbVar6 = (byte *)FUN_140d4d740();
    if ((*pbVar6 & 1) == 0) {
      uStack_49 = 1;
      lStack_188 = FUN_140d64e00();
      pbVar6[0] = 1;
      pbVar6[1] = 0;
      pbVar6[2] = 0;
      pbVar6[3] = 0;
      pbVar6[4] = 0;
      pbVar6[5] = 0;
      pbVar6[6] = 0;
      pbVar6[7] = 0;
      *(longlong *)(pbVar6 + 8) = lStack_188;
      *(undefined1 (**) [16])(pbVar6 + 0x10) = pauVar12;
    }
    else {
      lStack_188 = *(longlong *)(pbVar6 + 8);
      pauVar12 = *(undefined1 (**) [16])(pbVar6 + 0x10);
    }
    pauStack_d8 = (undefined1 (*) [16])(lStack_188 + 1);
    *(undefined1 (**) [16])(pbVar6 + 8) = pauStack_d8;
    uStack_198 = _DAT_142cdf560;
    pauStack_190 = _UNK_142cdf568;
    pauStack_1a8 = _DAT_142cdf550;
    pauStack_1a0 = _UNK_142cdf558;
    auVar2 = *pauVar5;
    if (pauVar13 == (undefined1 (*) [16])0x0) {
      pauStack_e8 = (undefined1 (*) [16])0x0;
      pauStack_e0 = pauVar12;
    }
    else {
      uVar9 = (longlong)(*pauVar13 + 1) * 0x68 + 0xfU & 0xfffffffffffffff0;
      pauStack_d8 = (undefined1 (*) [16])((longlong)pauVar5 - uVar9);
      pauStack_e8 = (undefined1 (*) [16])0x10;
      pauStack_e0 = (undefined1 (*) [16])(pauVar13[1] + uVar9 + 1);
    }
    uStack_c0 = *pauVar13 + 1 + (longlong)pauVar5;
    uStack_d0 = pauVar5;
    uStack_b8 = CONCAT62(uStack_b8._2_6_,
                         ~((ushort)(SUB161(auVar2 >> 7,0) & 1) |
                           (ushort)(SUB161(auVar2 >> 0xf,0) & 1) << 1 |
                           (ushort)(SUB161(auVar2 >> 0x17,0) & 1) << 2 |
                           (ushort)(SUB161(auVar2 >> 0x1f,0) & 1) << 3 |
                           (ushort)(SUB161(auVar2 >> 0x27,0) & 1) << 4 |
                           (ushort)(SUB161(auVar2 >> 0x2f,0) & 1) << 5 |
                           (ushort)(SUB161(auVar2 >> 0x37,0) & 1) << 6 |
                           (ushort)(SUB161(auVar2 >> 0x3f,0) & 1) << 7 |
                           (ushort)(SUB161(auVar2 >> 0x47,0) & 1) << 8 |
                           (ushort)(SUB161(auVar2 >> 0x4f,0) & 1) << 9 |
                           (ushort)(SUB161(auVar2 >> 0x57,0) & 1) << 10 |
                           (ushort)(SUB161(auVar2 >> 0x5f,0) & 1) << 0xb |
                           (ushort)(SUB161(auVar2 >> 0x67,0) & 1) << 0xc |
                           (ushort)(SUB161(auVar2 >> 0x6f,0) & 1) << 0xd |
                           (ushort)(SUB161(auVar2 >> 0x77,0) & 1) << 0xe |
                          (ushort)(byte)(auVar2[0xf] >> 7) << 0xf));
    uStack_b0 = pauVar4;
    pauStack_148 = (undefined1 (*) [16])0x0;
    pauStack_140 = (undefined1 (*) [16])0x8;
    pauStack_138 = (undefined1 (*) [16])0x0;
    uStack_a8 = &pauStack_238;
    pauStack_180 = pauVar12;
    uStack_c8 = pauVar1;
    FUN_1420dd5e0(&pauStack_208,&pauStack_148,&pauStack_e8);
    FUN_1420dc1a0(&uStack_2c0,&pauStack_208);
    pauVar13 = pauStack_2b8 + lStack_2b0 * 7;
    pauStack_178 = pauStack_2b8;
    uStack_168 = uStack_2c0;
    pauVar5 = pauStack_2b8;
    pauStack_160 = pauVar13;
    if (lStack_2b0 != 0) {
      do {
        ppauStack_88 = *(undefined1 (***) [16])(pauStack_2b8[6] + 8);
        puStack_98 = *(undefined **)(pauStack_2b8[5] + 8);
        uStack_90 = *(undefined8 *)pauStack_2b8[6];
        uStack_a8 = *(undefined1 (***) [16])(pauStack_2b8[4] + 8);
        uStack_a0 = *(undefined8 *)pauStack_2b8[5];
        uStack_b8 = *(undefined8 *)(pauStack_2b8[3] + 8);
        uStack_b0 = *(undefined1 (**) [16])pauStack_2b8[4];
        uStack_c8 = *(undefined1 (**) [16])(pauStack_2b8[2] + 8);
        uStack_c0 = *(undefined **)pauStack_2b8[3];
        pauStack_d8 = *(undefined1 (**) [16])(pauStack_2b8[1] + 8);
        uStack_d0 = *(undefined1 (**) [16])pauStack_2b8[2];
        pauStack_e8 = *(undefined1 (**) [16])(*pauStack_2b8 + 8);
        pauStack_e0 = *(undefined1 (**) [16])pauStack_2b8[1];
        pauVar5 = pauStack_2b8 + 7;
        if (SBORROW8(0,*(longlong *)*pauStack_2b8)) break;
        uStack_c8._4_4_ = (undefined4)((ulonglong)uStack_c8 >> 0x20);
        uStack_c0._4_4_ = (undefined4)((ulonglong)uStack_c0 >> 0x20);
        uStack_b8._4_4_ = (undefined4)((ulonglong)uStack_b8 >> 0x20);
        uStack_b0._4_4_ = (undefined4)((ulonglong)uStack_b0 >> 0x20);
        uStack_a8._4_4_ = (undefined4)((ulonglong)uStack_a8 >> 0x20);
        uStack_a0._4_4_ = (undefined4)((ulonglong)uStack_a0 >> 0x20);
        uStack_1d8 = (undefined4)uStack_a8;
        uStack_1d4 = uStack_a8._4_4_;
        uStack_1d0 = (undefined4)uStack_a0;
        uStack_1cc = uStack_a0._4_4_;
        uStack_1e8 = (undefined4)uStack_b8;
        uStack_1e4 = uStack_b8._4_4_;
        uStack_1e0 = (undefined4)uStack_b0;
        uStack_1dc = uStack_b0._4_4_;
        uStack_1f8 = (undefined4)uStack_c8;
        uStack_1f4 = uStack_c8._4_4_;
        uStack_1f0 = (undefined4)uStack_c0;
        uStack_1ec = uStack_c0._4_4_;
        uStack_4a = 1;
        lStack_250 = *(longlong *)*pauStack_2b8;
        pauStack_248 = pauStack_e8;
        pauStack_240 = pauStack_e0;
        pauStack_208 = pauStack_d8;
        pauStack_200 = uStack_d0;
        uStack_1c8 = puStack_98;
        uStack_1c0 = uStack_90;
        ppauStack_1b8 = ppauStack_88;
        pauStack_58 = pauStack_2b8 + 7;
        FUN_140d7bd90(&lStack_330,&lStack_250);
        if (SBORROW8(0,(longlong)pauStack_208)) {
          pauStack_170 = pauStack_58;
          *(undefined1 (**) [16])(*pauStack_60 + 8) = pauStack_200;
          *(undefined8 *)*pauStack_60 = 0;
          if (lStack_330 != 0) {
            func_0x000140613c20(uStack_328,lStack_330,1);
          }
          if (lStack_250 != 0) {
            func_0x000140613c20(pauStack_248,lStack_250,1);
          }
          FUN_1420d3e20(&pauStack_178);
          goto LAB_1417b83f3;
        }
        ppauStack_338 = ppauStack_1b8;
        uStack_378 = CONCAT44(uStack_1f4,uStack_1f8);
        uStack_370 = CONCAT44(uStack_1ec,uStack_1f0);
        uStack_348 = (undefined4)uStack_1c8;
        uStack_344 = uStack_1c8._4_4_;
        uStack_340 = (undefined4)uStack_1c0;
        uStack_33c = uStack_1c0._4_4_;
        uStack_358 = uStack_1d8;
        uStack_354 = uStack_1d4;
        uStack_350 = uStack_1d0;
        uStack_34c = uStack_1cc;
        uStack_368 = uStack_1e8;
        uStack_364 = uStack_1e4;
        uStack_360 = uStack_1e0;
        uStack_35c = uStack_1dc;
        pauStack_388 = pauStack_208;
        pauStack_380 = pauStack_200;
        uStack_4a = 0;
        FUN_1420de0b0(&pauStack_148,&pauStack_1a8,&lStack_330,&pauStack_388);
        if (!SBORROW8(0,(longlong)pauStack_148)) {
          uStack_4a = 0;
          FUN_140889e60(&pauStack_148);
        }
        if (lStack_250 != 0) {
          func_0x000140613c20(pauStack_248,lStack_250,1);
        }
        pauVar5 = pauVar13;
        pauStack_2b8 = pauStack_58;
      } while (pauStack_58 != pauVar13);
    }
    pauStack_170 = pauVar5;
    FUN_1420d3e20(&pauStack_178);
    uVar7 = FUN_140d76ed0(&lStack_270);
    lVar10 = FUN_140d61cd0(uVar7);
    if (lVar10 == 0) {
      FUN_1428d9310(&UNK_142cdf670);
LAB_1417b8969:
                    /* WARNING: Does not return */
      pcVar3 = (code *)invalidInstructionException();
      (*pcVar3)();
    }
    pauStack_e8 = (undefined1 (*) [16])CONCAT71(pauStack_e8._1_7_,1);
    pauVar5 = (undefined1 (*) [16])FUN_140d5a150(&pauStack_e8,lVar10,pauVar5);
    if (pauVar5 == (undefined1 (*) [16])0x0) {
      pauStack_d8 = (undefined1 (*) [16])0x0;
      pauStack_e8 = (undefined1 (*) [16])0x1;
      pauStack_e0 = (undefined1 (*) [16])0x80;
      FUN_1420d8640(&pauStack_148,&pauStack_e8,&pauStack_1a8);
      if ((char)pauStack_148 != '2') {
        uStack_a8 = ppauStack_108;
        uStack_b8 = CONCAT44(uStack_114,uStack_118);
        uStack_b0 = (undefined1 (*) [16])CONCAT44(uStack_10c,uStack_110);
        uStack_c8 = (undefined1 (*) [16])CONCAT44(uStack_124,uStack_128);
        uStack_c0 = (undefined *)CONCAT44(uStack_11c,uStack_120);
        pauStack_d8 = pauStack_138;
        uStack_d0 = (undefined1 (*) [16])CONCAT44(uStack_12c,uStack_130);
        pauStack_e8 = pauStack_148;
        pauStack_e0 = pauStack_140;
        FUN_1428d9760(&UNK_142cdda68,0x2b,&pauStack_e8,&UNK_142cddab8,&UNK_142cdf6a8);
        goto LAB_1417b8969;
      }
      pauStack_158 = pauStack_140;
      pauStack_58 = pauStack_138;
      uVar7 = FUN_140d76ed0(&lStack_270);
      lVar10 = FUN_140d59430(uVar7);
      if (pauStack_158 != (undefined1 (*) [16])0x0) {
        func_0x000140613c20(pauStack_58,pauStack_158,1);
      }
      if (lVar10 != 0) {
        uVar7 = FUN_142915970(lVar10);
        goto LAB_1417b83e1;
      }
      if ((char)pauStack_150 == '\0') {
LAB_1417b8719:
        *(longlong *)pauStack_60[2] = lStack_188;
        *(undefined1 (**) [16])(pauStack_60[2] + 8) = pauStack_180;
        *(undefined8 *)pauStack_60[1] = uStack_198;
        *(undefined1 (**) [16])(pauStack_60[1] + 8) = pauStack_190;
        *(undefined1 (**) [16])*pauStack_60 = pauStack_1a8;
        *(undefined1 (**) [16])(*pauStack_60 + 8) = pauStack_1a0;
        if (pauStack_298 != (undefined1 (*) [16])0x0) {
          puVar8 = (undefined8 *)(puStack_68 + 8);
          pauVar5 = pauStack_298;
          do {
            if (puVar8[-1] != 0) {
              func_0x000140613c20(*puVar8,puVar8[-1],1);
            }
            puVar8 = puVar8 + 3;
            pauVar5 = (undefined1 (*) [16])(pauVar5[-1] + 0xf);
          } while (pauVar5 != (undefined1 (*) [16])0x0);
        }
        if (lStack_2a8 != 0) {
          func_0x000140613c20(puStack_68,lStack_2a8 * 0x18,8);
        }
        param_1 = pauStack_60;
        if (lStack_290 != 0) {
          func_0x000140613c20(uStack_288,lStack_290,1);
        }
        FUN_140b73a50(&pauStack_238,&lStack_218,0x70,0x10);
        goto LAB_1417b84bc;
      }
      pauStack_d8 = (undefined1 (*) [16])(*pauStack_1a8 + (longlong)(*pauStack_1a0 + 1));
      auVar2 = *pauStack_1a8;
      pauStack_e8 = pauStack_1a8;
      pauStack_e0 = pauStack_1a8 + 1;
      uStack_d0 = (undefined1 (*) [16])
                  CONCAT62(uStack_d0._2_6_,
                           ~((ushort)(SUB161(auVar2 >> 7,0) & 1) |
                             (ushort)(SUB161(auVar2 >> 0xf,0) & 1) << 1 |
                             (ushort)(SUB161(auVar2 >> 0x17,0) & 1) << 2 |
                             (ushort)(SUB161(auVar2 >> 0x1f,0) & 1) << 3 |
                             (ushort)(SUB161(auVar2 >> 0x27,0) & 1) << 4 |
                             (ushort)(SUB161(auVar2 >> 0x2f,0) & 1) << 5 |
                             (ushort)(SUB161(auVar2 >> 0x37,0) & 1) << 6 |
                             (ushort)(SUB161(auVar2 >> 0x3f,0) & 1) << 7 |
                             (ushort)(SUB161(auVar2 >> 0x47,0) & 1) << 8 |
                             (ushort)(SUB161(auVar2 >> 0x4f,0) & 1) << 9 |
                             (ushort)(SUB161(auVar2 >> 0x57,0) & 1) << 10 |
                             (ushort)(SUB161(auVar2 >> 0x5f,0) & 1) << 0xb |
                             (ushort)(SUB161(auVar2 >> 0x67,0) & 1) << 0xc |
                             (ushort)(SUB161(auVar2 >> 0x6f,0) & 1) << 0xd |
                             (ushort)(SUB161(auVar2 >> 0x77,0) & 1) << 0xe |
                            (ushort)(byte)(auVar2[0xf] >> 7) << 0xf));
      uStack_c8 = pauStack_190;
      FUN_1420dbef0(&pauStack_208,&pauStack_e8,&UNK_142cdda30);
      uVar9 = CONCAT44(uStack_1f4,uStack_1f8);
      if (1 < uVar9) {
        if (uVar9 < 0x15) {
          FUN_1420dec80(pauStack_200,uVar9,1,&pauStack_178);
        }
        else {
          FUN_1420df8b0(pauStack_200,uVar9,&pauStack_178);
        }
      }
      pauStack_d8 = (undefined1 (*) [16])0x0;
      pauStack_e8 = (undefined1 (*) [16])0x1;
      pauStack_e0 = (undefined1 (*) [16])0x80;
      FUN_1420d8310(&pauStack_148,&pauStack_e8,&pauStack_208);
      if ((char)pauStack_148 != '2') {
        uStack_a8 = ppauStack_108;
        uStack_b8 = CONCAT44(uStack_114,uStack_118);
        uStack_b0 = (undefined1 (*) [16])CONCAT44(uStack_10c,uStack_110);
        uStack_c8 = (undefined1 (*) [16])CONCAT44(uStack_124,uStack_128);
        uStack_c0 = (undefined *)CONCAT44(uStack_11c,uStack_120);
        pauStack_d8 = pauStack_138;
        uStack_d0 = (undefined1 (*) [16])CONCAT44(uStack_12c,uStack_130);
        pauStack_e8 = pauStack_148;
        pauStack_e0 = pauStack_140;
        FUN_1428d9760(&UNK_142cdda68,0x2b,&pauStack_e8,&UNK_142cddab8,&UNK_142cdf6c0);
        goto LAB_1417b8969;
      }
      pauStack_58 = pauStack_140;
      pauStack_150 = pauStack_138;
      lVar10 = FUN_140d59430(&UNK_142cdf5a3,0x21,pauStack_138,CONCAT44(uStack_12c,uStack_130));
      if (pauStack_58 != (undefined1 (*) [16])0x0) {
        func_0x000140613c20(pauStack_150,pauStack_58,1);
      }
      if (lVar10 == 0) {
        if (pauStack_208 != (undefined1 (*) [16])0x0) {
          func_0x000140613c20(pauStack_200,(longlong)pauStack_208 << 3,8);
        }
        goto LAB_1417b8719;
      }
      uVar7 = FUN_142915970(lVar10);
      *(undefined8 *)(*pauStack_60 + 8) = uVar7;
      *(undefined8 *)*pauStack_60 = 0;
      if (pauStack_208 != (undefined1 (*) [16])0x0) {
        func_0x000140613c20(pauStack_200,(longlong)pauStack_208 << 3,8);
      }
    }
    else {
      uStack_4d = 1;
      pauStack_208 = pauVar5;
      FUN_140d56700(&pauStack_e8);
      uStack_128 = (undefined4)uStack_c8;
      uStack_124 = uStack_c8._4_4_;
      uStack_120 = (undefined4)uStack_c0;
      uStack_11c = uStack_c0._4_4_;
      pauStack_138 = pauStack_d8;
      uStack_130 = (undefined4)uStack_d0;
      uStack_12c = uStack_d0._4_4_;
      pauStack_148 = pauStack_e8;
      pauStack_140 = pauStack_e0;
      pauStack_e8 = (undefined1 (*) [16])&UNK_142cdf688;
      pauStack_e0 = (undefined1 (*) [16])0x1b;
      uStack_4d = 0;
      pauStack_d8 = pauVar5;
      uVar7 = FUN_1429155a0(&pauStack_e8,&pauStack_148);
LAB_1417b83e1:
      *(undefined8 *)(*pauStack_60 + 8) = uVar7;
      *(undefined8 *)*pauStack_60 = 0;
    }
LAB_1417b83f3:
    uStack_49 = 0;
    FUN_140b73a50(&pauStack_1a8,&lStack_188,0x70,0x10);
    if (pauStack_298 != (undefined1 (*) [16])0x0) {
      puVar8 = (undefined8 *)(puStack_68 + 8);
      pauVar5 = pauStack_298;
      do {
        if (puVar8[-1] != 0) {
          func_0x000140613c20(*puVar8,puVar8[-1],1);
        }
        puVar8 = puVar8 + 3;
        pauVar5 = (undefined1 (*) [16])(pauVar5[-1] + 0xf);
      } while (pauVar5 != (undefined1 (*) [16])0x0);
    }
    param_1 = pauStack_60;
    if (lStack_2a8 != 0) {
      func_0x000140613c20(puStack_68,lStack_2a8 * 0x18,8);
      param_1 = pauStack_60;
    }
  }
  if (lStack_290 != 0) {
    func_0x000140613c20(uStack_288,lStack_290,1);
  }
  FUN_140b73a50(&pauStack_238,&lStack_218,0x70,0x10);
LAB_1417b84bc:
  if (lStack_270 != 0) {
    func_0x000140613c20(uStack_268,lStack_270,1);
  }
  return param_1;
}

