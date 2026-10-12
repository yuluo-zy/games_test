
/* WARNING: Type propagation algorithm not settling */

undefined4 * FUN_140c08f10(undefined4 *param_1,longlong *param_2,undefined8 *param_3)

{
  longlong lVar1;
  char cVar2;
  undefined1 *puVar3;
  code *pcVar4;
  bool bVar5;
  bool bVar6;
  char cVar7;
  char cVar8;
  char cVar9;
  char cVar10;
  char cVar11;
  char cVar12;
  char cVar13;
  char cVar14;
  char cVar15;
  char cVar16;
  char cVar17;
  char cVar18;
  char cVar19;
  char cVar20;
  char cVar21;
  char cVar22;
  char cVar23;
  char cVar24;
  char cVar25;
  char cVar26;
  char cVar27;
  char cVar28;
  char cVar29;
  char cVar30;
  char cVar31;
  char cVar32;
  char cVar33;
  char cVar34;
  char cVar35;
  char cVar36;
  char cVar37;
  char cVar38;
  ulonglong *puVar39;
  uint uVar40;
  uint uVar41;
  uint uVar42;
  undefined4 uVar43;
  int iVar44;
  undefined8 *puVar45;
  char *pcVar46;
  longlong *plVar47;
  longlong *plVar48;
  longlong lVar49;
  ulonglong uVar50;
  undefined4 *puVar51;
  longlong lVar52;
  longlong lVar53;
  longlong lVar54;
  longlong *plVar55;
  ulonglong uVar56;
  ulonglong uVar57;
  undefined1 auVar58 [16];
  undefined1 auVar59 [16];
  undefined1 auStack_5e0 [196];
  undefined1 auStack_51c [196];
  uint uStack_458;
  uint uStack_454;
  uint uStack_450;
  uint uStack_44c;
  longlong lStack_448;
  undefined8 uStack_440;
  code *apcStack_388 [3];
  undefined8 uStack_370;
  undefined8 uStack_368;
  undefined8 uStack_360;
  undefined8 uStack_358;
  undefined4 uStack_350;
  undefined4 uStack_34c;
  undefined4 uStack_348;
  longlong *plStack_344;
  undefined *puStack_338;
  undefined *puStack_330;
  undefined8 uStack_328;
  undefined4 *puStack_320;
  undefined4 uStack_318;
  ulonglong uStack_314;
  undefined8 uStack_30c;
  undefined8 uStack_304;
  uint uStack_2fc;
  undefined8 uStack_2f8;
  undefined4 uStack_2f0;
  undefined4 uStack_2ec;
  int *piStack_2e8;
  undefined *puStack_2e0;
  longlong lStack_2d8;
  undefined8 uStack_2d0;
  undefined4 uStack_2c8;
  undefined **ppuStack_2c0;
  undefined4 uStack_2b8;
  undefined4 uStack_2b4;
  undefined8 uStack_2b0;
  undefined4 uStack_2a8;
  undefined8 uStack_2a4;
  code *pcStack_298;
  undefined8 uStack_290;
  uint uStack_288;
  undefined4 uStack_284;
  undefined4 uStack_280;
  undefined4 uStack_27c;
  undefined4 uStack_278;
  undefined4 uStack_274;
  undefined4 uStack_270;
  undefined4 uStack_26c;
  undefined4 uStack_268;
  undefined4 uStack_264;
  longlong *plStack_260;
  undefined4 uStack_258;
  undefined8 uStack_254;
  undefined4 uStack_24c;
  longlong lStack_248;
  longlong *plStack_240;
  ulonglong uStack_238;
  undefined4 *puStack_230;
  undefined8 uStack_228;
  longlong *plStack_220;
  undefined8 uStack_218;
  longlong lStack_210;
  longlong lStack_208;
  longlong *plStack_200;
  ulonglong uStack_1f8;
  int iStack_1e8;
  undefined4 uStack_1e4;
  longlong *plStack_1e0;
  uint uStack_1d8;
  uint uStack_1d4;
  undefined8 uStack_1d0;
  uint uStack_1c8;
  undefined4 uStack_1c4;
  undefined8 uStack_1c0;
  undefined4 uStack_1b8;
  undefined4 uStack_1b4;
  undefined8 uStack_1b0;
  undefined4 uStack_1a8;
  undefined4 uStack_1a4;
  longlong *plStack_1a0;
  longlong lStack_198;
  uint uStack_190;
  undefined4 uStack_18c;
  undefined *puStack_188;
  undefined8 uStack_180;
  int **ppiStack_178;
  undefined1 auStack_170 [16];
  char cStack_c8;
  longlong lStack_c0;
  longlong lStack_b8;
  longlong lStack_b0;
  undefined8 *puStack_a8;
  longlong lStack_a0;
  longlong *plStack_98;
  longlong lStack_90;
  char cStack_81;
  ulonglong uStack_80;
  undefined1 uStack_74;
  undefined1 uStack_73;
  undefined1 uStack_72;
  undefined1 uStack_71;
  undefined8 uStack_70;
  
  uStack_70 = 0xfffffffffffffffe;
  lStack_a0 = param_2[1];
  lVar49 = param_2[2];
  puStack_a8 = param_3;
  puVar45 = (undefined8 *)func_0x000140613c10(0x10,8);
  plStack_98 = param_2;
  if (puVar45 == (undefined8 *)0x0) {
    uStack_73 = 1;
    FUN_1428d9000(8,0x10);
    goto code_r0x000140c0a01c;
  }
  *puVar45 = &UNK_142b1909b;
  puVar45[1] = &UNK_142b190c2;
  uStack_1d8 = 2;
  uStack_1d4 = 0;
  uStack_1d0._0_4_ = (uint)puVar45;
  uStack_1d0._4_4_ = (uint)((ulonglong)puVar45 >> 0x20);
  uStack_1c8 = 2;
  uStack_1c4 = 0;
  cVar2 = *(char *)((longlong)param_2 + 0x19);
  if (cVar2 == '\0') {
    lVar52 = 2;
    uVar42 = 2;
    uVar41 = 0;
    if ((char)param_2[3] != '\0') goto code_r0x000140c08fdb;
  }
  else {
code_r0x000140c08fdb:
    FUN_140562870(&uStack_1d8,&UNK_142b190f8);
    puVar45 = (undefined8 *)CONCAT44(uStack_1d0._4_4_,(uint)uStack_1d0);
    puVar45[2] = &UNK_142b190e3;
    lVar52 = 3;
    uVar42 = uStack_1d8;
    uVar41 = uStack_1d4;
  }
  uStack_1c4 = uVar41;
  uStack_1c8 = uVar42;
  uStack_1b0 = lVar49 * 0x10 + lStack_a0;
  uStack_1c0 = puVar45 + lVar52;
  uStack_1d8 = (uint)puVar45;
  uStack_1d4 = (uint)((ulonglong)puVar45 >> 0x20);
  uStack_1b8 = (undefined4)lStack_a0;
  uStack_1b4 = (undefined4)((ulonglong)lStack_a0 >> 0x20);
  uStack_73 = 1;
  uStack_1d0._0_4_ = uStack_1d8;
  uStack_1d0._4_4_ = uStack_1d4;
  FUN_1418b6410(&lStack_c0,&uStack_1d8,&UNK_142b18f50);
  uStack_458 = 0;
  uStack_454 = 0;
  uStack_71 = 1;
  FUN_1418b05f0(&uStack_1d8,puStack_a8,&uStack_458);
  cVar38 = UNK_142b18dff;
  cVar37 = UNK_142b18dfe;
  cVar36 = UNK_142b18dfd;
  cVar35 = UNK_142b18dfc;
  cVar34 = UNK_142b18dfb;
  cVar33 = UNK_142b18dfa;
  cVar32 = UNK_142b18df9;
  cVar31 = UNK_142b18df8;
  cVar30 = UNK_142b18df7;
  cVar29 = UNK_142b18df6;
  cVar28 = UNK_142b18df5;
  cVar27 = UNK_142b18df4;
  cVar26 = UNK_142b18df3;
  cVar25 = UNK_142b18df2;
  cVar24 = UNK_142b18df1;
  cVar23 = UNK_142b18df0;
  cVar22 = UNK_142b18def;
  cVar21 = UNK_142b18dee;
  cVar20 = UNK_142b18ded;
  cVar19 = UNK_142b18dec;
  cVar18 = UNK_142b18deb;
  cVar17 = UNK_142b18dea;
  cVar16 = UNK_142b18de9;
  cVar15 = UNK_142b18de8;
  cVar14 = UNK_142b18de7;
  cVar13 = UNK_142b18de6;
  cVar12 = UNK_142b18de5;
  cVar11 = UNK_142b18de4;
  cVar10 = UNK_142b18de3;
  cVar9 = UNK_142b18de2;
  cVar8 = UNK_142b18de1;
  cVar7 = UNK_142b18de0;
  lVar49 = CONCAT44(uStack_1d4,uStack_1d8);
  if (SBORROW8(0,lVar49)) {
code_r0x000140c092b3:
    if (2 < *puRam000000014362c3a8) {
      uStack_71 = 1;
      puVar45 = (undefined8 *)func_0x00014067dd80(&UNK_142b19198);
      uStack_18c = *(undefined4 *)(puVar45 + 2);
      uStack_1a8 = 3;
      uStack_1a4 = 0;
      plStack_1a0 = (longlong *)&UNK_142b191b0;
      lStack_198 = 0x17;
      puStack_188 = &UNK_142b19188;
      uStack_180 = 1;
      ppiStack_178 = (int **)0x8;
      uStack_1d8 = 0;
      uStack_1d4 = 0;
      auStack_170 = (undefined1  [16])0x0;
      uStack_1d0._0_4_ = 0x42b191b0;
      uStack_1d0._4_4_ = 1;
      uStack_1c8 = 0x17;
      uStack_1c4 = 0;
      uStack_1c0._0_4_ = 0;
      uStack_1c0._4_4_ = 0;
      uStack_1b8 = (undefined4)*puVar45;
      uStack_1b4 = (undefined4)((ulonglong)*puVar45 >> 0x20);
      uStack_1b0._0_4_ = (undefined4)puVar45[1];
      uStack_1b0._4_4_ = (undefined4)((ulonglong)puVar45[1] >> 0x20);
      uStack_190 = 1;
      uStack_71 = 1;
      func_0x000140d4a330(&uStack_72,&uStack_1d8);
    }
    bVar5 = false;
  }
  else {
    uStack_80 = CONCAT44(uStack_1d0._4_4_,(uint)uStack_1d0);
    lVar52 = CONCAT44(uStack_1c4,uStack_1c8);
    if (lVar52 != 0) {
      lVar54 = 0;
      lStack_90 = lVar49;
      do {
        lVar53 = 0x100;
        pcVar46 = (char *)FUN_140d80ef0(uStack_80 + lVar54);
        lVar49 = lStack_90;
        if ((lVar53 == 0x16 && pcVar46 != (char *)0x0) &&
           (auVar58[0] = -(*pcVar46 == cVar23), auVar58[1] = -(pcVar46[1] == cVar24),
           auVar58[2] = -(pcVar46[2] == cVar25), auVar58[3] = -(pcVar46[3] == cVar26),
           auVar58[4] = -(pcVar46[4] == cVar27), auVar58[5] = -(pcVar46[5] == cVar28),
           auVar58[6] = -(pcVar46[6] == cVar29), auVar58[7] = -(pcVar46[7] == cVar30),
           auVar58[8] = -(pcVar46[8] == cVar31), auVar58[9] = -(pcVar46[9] == cVar32),
           auVar58[10] = -(pcVar46[10] == cVar33), auVar58[0xb] = -(pcVar46[0xb] == cVar34),
           auVar58[0xc] = -(pcVar46[0xc] == cVar35), auVar58[0xd] = -(pcVar46[0xd] == cVar36),
           auVar58[0xe] = -(pcVar46[0xe] == cVar37), auVar58[0xf] = -(pcVar46[0xf] == cVar38),
           auVar59[0] = -(pcVar46[6] == cVar7), auVar59[1] = -(pcVar46[7] == cVar8),
           auVar59[2] = -(pcVar46[8] == cVar9), auVar59[3] = -(pcVar46[9] == cVar10),
           auVar59[4] = -(pcVar46[10] == cVar11), auVar59[5] = -(pcVar46[0xb] == cVar12),
           auVar59[6] = -(pcVar46[0xc] == cVar13), auVar59[7] = -(pcVar46[0xd] == cVar14),
           auVar59[8] = -(pcVar46[0xe] == cVar15), auVar59[9] = -(pcVar46[0xf] == cVar16),
           auVar59[10] = -(pcVar46[0x10] == cVar17), auVar59[0xb] = -(pcVar46[0x11] == cVar18),
           auVar59[0xc] = -(pcVar46[0x12] == cVar19), auVar59[0xd] = -(pcVar46[0x13] == cVar20),
           auVar59[0xe] = -(pcVar46[0x14] == cVar21), auVar59[0xf] = -(pcVar46[0x15] == cVar22),
           auVar59 = auVar59 & auVar58,
           (ushort)((ushort)(SUB161(auVar59 >> 7,0) & 1) |
                    (ushort)(SUB161(auVar59 >> 0xf,0) & 1) << 1 |
                    (ushort)(SUB161(auVar59 >> 0x17,0) & 1) << 2 |
                    (ushort)(SUB161(auVar59 >> 0x1f,0) & 1) << 3 |
                    (ushort)(SUB161(auVar59 >> 0x27,0) & 1) << 4 |
                    (ushort)(SUB161(auVar59 >> 0x2f,0) & 1) << 5 |
                    (ushort)(SUB161(auVar59 >> 0x37,0) & 1) << 6 |
                    (ushort)(SUB161(auVar59 >> 0x3f,0) & 1) << 7 |
                    (ushort)(SUB161(auVar59 >> 0x47,0) & 1) << 8 |
                    (ushort)(SUB161(auVar59 >> 0x4f,0) & 1) << 9 |
                    (ushort)(SUB161(auVar59 >> 0x57,0) & 1) << 10 |
                    (ushort)(SUB161(auVar59 >> 0x5f,0) & 1) << 0xb |
                    (ushort)(SUB161(auVar59 >> 0x67,0) & 1) << 0xc |
                    (ushort)(SUB161(auVar59 >> 0x6f,0) & 1) << 0xd |
                    (ushort)(SUB161(auVar59 >> 0x77,0) & 1) << 0xe |
                   (ushort)(byte)(auVar59[0xf] >> 7) << 0xf) == 0xffff)) {
          bVar6 = true;
          bVar5 = true;
          goto joined_r0x000140c09152;
        }
        lVar54 = lVar54 + 0x104;
      } while (lVar52 * 0x104 != lVar54);
    }
    bVar6 = false;
    bVar5 = false;
joined_r0x000140c09152:
    if (lVar49 != 0) {
      func_0x000140613c20(uStack_80,lVar49 * 0x104,4);
      bVar5 = bVar6;
    }
    if (!bVar5) goto code_r0x000140c092b3;
    if (2 < *puRam000000014362c3a8) {
      uStack_71 = 1;
      puVar45 = (undefined8 *)func_0x00014067dd80(&UNK_142b191f8);
      uStack_18c = *(undefined4 *)(puVar45 + 2);
      uStack_1a8 = 3;
      uStack_1a4 = 0;
      plStack_1a0 = (longlong *)&UNK_142b191b0;
      lStack_198 = 0x17;
      puStack_188 = &UNK_142b191e8;
      uStack_180 = 1;
      ppiStack_178 = (int **)0x8;
      uStack_1d8 = 0;
      uStack_1d4 = 0;
      auStack_170 = (undefined1  [16])0x0;
      uStack_1d0._0_4_ = 0x42b191b0;
      uStack_1d0._4_4_ = 1;
      uStack_1c8 = 0x17;
      uStack_1c4 = 0;
      uStack_1c0._0_4_ = 0;
      uStack_1c0._4_4_ = 0;
      uStack_1b8 = (undefined4)*puVar45;
      uStack_1b4 = (undefined4)((ulonglong)*puVar45 >> 0x20);
      uStack_1b0._0_4_ = (undefined4)puVar45[1];
      uStack_1b0._4_4_ = (undefined4)((ulonglong)puVar45[1] >> 0x20);
      uStack_190 = 1;
      uStack_71 = 1;
      func_0x000140d4a330(&uStack_72,&uStack_1d8);
    }
    if (lStack_b0 == lStack_c0) {
      uStack_71 = 1;
      FUN_140562870(&lStack_c0,&UNK_142b19228);
    }
    *(undefined **)(lStack_b8 + lStack_b0 * 8) = &UNK_142b19210;
    lStack_b0 = lStack_b0 + 1;
    bVar5 = true;
  }
  lVar49 = plStack_98[3];
  lStack_248 = 0;
  plStack_240 = (longlong *)0x8;
  uStack_238 = 0;
  if ((char)lVar49 == '\x01') {
    FUN_140d797d0(&uStack_458,&UNK_142b19110,0x1b);
    if (!SBORROW8(0,CONCAT44(uStack_454,uStack_458))) {
      uStack_1c8 = (uint)lStack_448;
      uStack_1c4 = (undefined4)((ulonglong)lStack_448 >> 0x20);
      uStack_1c0._0_4_ = (undefined4)uStack_440;
      uStack_1c0._4_4_ = (undefined4)((ulonglong)uStack_440 >> 0x20);
      uStack_1d8 = uStack_458;
      uStack_1d4 = uStack_454;
      uStack_1d0._0_4_ = uStack_450;
      uStack_1d0._4_4_ = uStack_44c;
      FUN_1428d9760(&UNK_142b18f88,0x2b,&uStack_1d8,&UNK_142b18f68,&UNK_142b19130);
      goto code_r0x000140c0a01c;
    }
    lStack_90 = CONCAT44(uStack_44c,uStack_450);
    uStack_80 = lStack_448;
    FUN_140562630(&lStack_248,&UNK_142b19148);
    *plStack_240 = lStack_90;
    plStack_240[1] = uStack_80;
    uStack_238 = 1;
  }
  plVar55 = plStack_240;
  uStack_1f8 = uStack_238;
  lStack_208 = lStack_248;
  plStack_200 = plStack_240;
  lVar52 = uStack_238 * 8;
  uStack_80 = uStack_238;
  if (uStack_238 == 0) {
    plVar47 = (longlong *)0x8;
  }
  else {
    plVar47 = (longlong *)func_0x000140613c10(lVar52,8);
    if (plVar47 == (longlong *)0x0) {
      uStack_74 = 1;
      FUN_1428d8fe3(8,lVar52,&UNK_142b18f50);
      goto code_r0x000140c0a01c;
    }
    if (uStack_80 < 0x10) {
      uVar57 = 0;
    }
    else if (plVar47 < plVar55 + uStack_80 * 2 + -1 && plVar55 < plVar47 + uStack_80) {
      uVar57 = 0;
    }
    else {
      uVar57 = uStack_80 & 0xfffffffffffffffc;
      plVar48 = plVar55 + 6;
      uVar50 = 0;
      do {
        lVar54 = plVar48[-4];
        lVar53 = *plVar48;
        lVar1 = plVar48[-2];
        plVar47[uVar50] = plVar48[-6];
        (plVar47 + uVar50)[1] = lVar54;
        plVar47[uVar50 + 2] = lVar1;
        (plVar47 + uVar50 + 2)[1] = lVar53;
        uVar50 = uVar50 + 4;
        plVar48 = plVar48 + 8;
      } while (uVar57 != uVar50);
      if (uStack_80 == uVar57) goto code_r0x000140c095ff;
    }
    uVar50 = uVar57;
    if ((uStack_80 & 3) != 0) {
      uVar56 = 0;
      do {
        plVar47[uVar50] = *(longlong *)((longlong)plVar55 + uVar56 * 2 + uVar57 * 0x10);
        uVar50 = uVar50 + 1;
        uVar56 = uVar56 + 8;
      } while ((uint)((int)(uStack_80 & 3) << 3) != uVar56);
    }
    if (uVar57 - uStack_80 < 0xfffffffffffffffd) {
      plVar48 = plVar55 + uVar50 * 2 + 6;
      do {
        plVar47[uVar50] = plVar48[-6];
        plVar47[uVar50 + 1] = plVar48[-4];
        plVar47[uVar50 + 2] = plVar48[-2];
        plVar47[uVar50 + 3] = *plVar48;
        uVar50 = uVar50 + 4;
        plVar48 = plVar48 + 8;
      } while (uStack_80 != uVar50);
    }
  }
code_r0x000140c095ff:
  puVar39 = puRam000000014362c3a8;
  uStack_1d4 = 0;
  uVar42 = uStack_1d4;
  uStack_1c8 = 0;
  uStack_1c4 = 0;
  uStack_454 = 0;
  uStack_450 = 0;
  uStack_44c = 0;
  uStack_318 = 0;
  uStack_314 = (ulonglong)uStack_1d8;
  uStack_30c = 0;
  uStack_304 = 0;
  uStack_2fc = uStack_458;
  uStack_2f8 = 0;
  uStack_2f0 = 0;
  uStack_2ec = 0x402000;
  lStack_248 = CONCAT44(lStack_248._4_4_,1);
  plStack_240 = (longlong *)0x0;
  uStack_1d4 = (uint)(uStack_238 >> 0x20);
  uStack_238 = uStack_238 & 0xffffffff00000000;
  puStack_230 = &uStack_318;
  plStack_1e0 = plVar47;
  plStack_220 = plVar47;
  lStack_210 = lStack_b8;
  uStack_1e4 = 1;
  uStack_1d0 = (undefined4 *)0x0;
  if (bVar5) {
    puStack_338 = &UNK_142b19240;
    puStack_330 = &UNK_142b19249;
    uStack_328 = 0x100000000;
    puStack_320 = &uStack_1e4;
    lStack_2d8._0_4_ = 0x3ba25b80;
    uStack_2d0 = 0;
    uStack_2c8 = 1;
    ppuStack_2c0 = &puStack_338;
    uStack_1b8 = (undefined4)lStack_b0;
    uStack_1b4 = uStack_218._4_4_;
    uStack_1b0._0_4_ = (undefined4)lStack_b8;
    uStack_1b0._4_4_ = (undefined4)((ulonglong)lStack_b8 >> 0x20);
    uStack_1c8 = (int)uStack_80;
    uStack_1c4 = uStack_228._4_4_;
    uStack_1c0._0_4_ = SUB84(plVar47,0);
    uStack_1c0._4_4_ = (undefined4)((ulonglong)plVar47 >> 0x20);
    uStack_1d8 = 0;
    uStack_1d0._0_4_ = (uint)puStack_230;
    uStack_1d0._4_4_ = (uint)((ulonglong)puStack_230 >> 0x20);
    puVar51 = (undefined4 *)&lStack_2d8;
    do {
      puVar51 = *(undefined4 **)(puVar51 + 2);
    } while (puVar51 != (undefined4 *)0x0);
    uStack_238 = (ulonglong)uStack_1d4 << 0x20;
    plStack_240 = &lStack_2d8;
    uVar42 = uStack_1d4;
    uStack_1c0 = plVar47;
    uStack_1b0 = lStack_b8;
    uStack_1d0 = puStack_230;
  }
  uStack_1d4 = uVar42;
  uStack_218 = CONCAT44(uStack_218._4_4_,(undefined4)lStack_b0);
  uStack_228 = CONCAT44(uStack_228._4_4_,(int)uStack_80);
  plStack_260 = (longlong *)puStack_a8[5];
  uStack_268 = (undefined4)puStack_a8[4];
  uStack_264 = (undefined4)((ulonglong)puStack_a8[4] >> 0x20);
  uStack_278 = (undefined4)puStack_a8[2];
  uStack_274 = (undefined4)((ulonglong)puStack_a8[2] >> 0x20);
  uStack_270 = (undefined4)puStack_a8[3];
  uStack_26c = (undefined4)((ulonglong)puStack_a8[3] >> 0x20);
  uStack_288 = (uint)*puStack_a8;
  uStack_284 = (undefined4)((ulonglong)*puStack_a8 >> 0x20);
  uStack_280 = (undefined4)puStack_a8[1];
  uStack_27c = (undefined4)((ulonglong)puStack_a8[1] >> 0x20);
  lStack_90 = lVar52;
  if (3 < *puRam000000014362c3a8) {
    puVar45 = (undefined8 *)func_0x00014067dd80(&UNK_142b192a0);
    uStack_18c = *(undefined4 *)(puVar45 + 2);
    uStack_1a8 = 4;
    uStack_1a4 = 0;
    plStack_1a0 = (longlong *)&UNK_142b191b0;
    lStack_198 = 0x17;
    puStack_188 = &UNK_142b19290;
    uStack_180 = 1;
    ppiStack_178 = (int **)0x8;
    uStack_1d8 = 0;
    uStack_1d4 = 0;
    auStack_170 = (undefined1  [16])0x0;
    uStack_1d0._0_4_ = 0x42b191b0;
    uStack_1d0._4_4_ = 1;
    uStack_1c8 = 0x17;
    uStack_1c4 = 0;
    uStack_1c0._0_4_ = 0;
    uStack_1c0._4_4_ = 0;
    uStack_1b8 = (undefined4)*puVar45;
    uStack_1b4 = (undefined4)((ulonglong)*puVar45 >> 0x20);
    uStack_1b0._0_4_ = (undefined4)puVar45[1];
    uStack_1b0._4_4_ = (undefined4)((ulonglong)puVar45[1] >> 0x20);
    uStack_190 = 1;
    func_0x000140d4a330(&uStack_72,&uStack_1d8);
  }
  uVar42 = (*(code *)CONCAT44(uStack_27c,uStack_280))(&lStack_248,0,&uStack_2b8);
  if (uVar42 == 0) {
    FUN_1418ce390(&uStack_1d8,&uStack_288,CONCAT44(uStack_2b4,uStack_2b8));
    uVar40 = uStack_1d4;
    uVar41 = uStack_1d8;
    uVar42 = (uint)uStack_1d0;
    lVar52 = CONCAT44(uStack_1d4,uStack_1d8);
    if (lVar52 != 0) {
      cStack_81 = cVar2;
      func_0x000142923c00(auStack_5e0,(longlong)&uStack_1d0 + 4,0xc4);
      uStack_458 = uVar41;
      uStack_454 = uVar40;
      uStack_450 = uVar42;
      func_0x000142923c00(&uStack_44c,auStack_5e0,0xc4);
      if (3 < *puVar39) {
        puVar45 = (undefined8 *)func_0x00014067dd80(&UNK_142b192e0);
        uStack_18c = *(undefined4 *)(puVar45 + 2);
        uStack_1a8 = 4;
        uStack_1a4 = 0;
        plStack_1a0 = (longlong *)&UNK_142b191b0;
        lStack_198 = 0x17;
        puStack_188 = &UNK_142b192d0;
        uStack_180 = 1;
        ppiStack_178 = (int **)0x8;
        uStack_1d8 = 0;
        uStack_1d4 = 0;
        auStack_170 = (undefined1  [16])0x0;
        uStack_1d0._0_4_ = 0x42b191b0;
        uStack_1d0._4_4_ = 1;
        uStack_1c8 = 0x17;
        uStack_1c4 = 0;
        uStack_1c0._0_4_ = 0;
        uStack_1c0._4_4_ = 0;
        uStack_1b8 = (undefined4)*puVar45;
        uStack_1b4 = (undefined4)((ulonglong)*puVar45 >> 0x20);
        uStack_1b0._0_4_ = (undefined4)puVar45[1];
        uStack_1b0._4_4_ = (undefined4)((ulonglong)puVar45[1] >> 0x20);
        uStack_190 = 1;
        func_0x000140d4a330(&uStack_72,&uStack_1d8);
      }
      if ((char)lVar49 != '\0') {
        if (3 < *puVar39) {
          puVar45 = (undefined8 *)func_0x00014067dd80(&UNK_142b19330);
          uStack_18c = *(undefined4 *)(puVar45 + 2);
          uStack_1a8 = 4;
          uStack_1a4 = 0;
          plStack_1a0 = (longlong *)&UNK_142b191b0;
          lStack_198 = 0x17;
          puStack_188 = &UNK_142b19320;
          uStack_180 = 1;
          ppiStack_178 = (int **)0x8;
          uStack_1d8 = 0;
          uStack_1d4 = 0;
          auStack_170 = (undefined1  [16])0x0;
          uStack_1d0._0_4_ = 0x42b191b0;
          uStack_1d0._4_4_ = 1;
          uStack_1c8 = 0x17;
          uStack_1c4 = 0;
          uStack_1c0._0_4_ = 0;
          uStack_1c0._4_4_ = 0;
          uStack_1b8 = (undefined4)*puVar45;
          uStack_1b4 = (undefined4)((ulonglong)*puVar45 >> 0x20);
          uStack_1b0._0_4_ = (undefined4)puVar45[1];
          uStack_1b0._4_4_ = (undefined4)((ulonglong)puVar45[1] >> 0x20);
          uStack_190 = 1;
          func_0x000140d4a330(&uStack_72,&uStack_1d8);
        }
        uStack_254 = 0;
        uStack_24c = 0;
        uStack_2b8 = 0x3b9cbe04;
        uStack_2b4 = uStack_258;
        uStack_2b0 = 0;
        uStack_2a8 = 0;
        uStack_2a4 = 0x700001100;
        pcStack_298 = FUN_140c0cd40;
        uStack_290 = 0;
        FUN_1418c87c0(apcStack_388,&uStack_288,&uStack_458);
        iVar44 = (*apcStack_388[0])(uStack_370,&uStack_2b8,0,&uStack_1d8);
        if ((iVar44 != 0) && (iStack_1e8 = iVar44, 1 < *puVar39)) {
          piStack_2e8 = &iStack_1e8;
          puStack_2e0 = &UNK_140c8f550;
          puVar45 = (undefined8 *)func_0x00014067dd80(&UNK_142b193b8);
          uStack_18c = *(undefined4 *)(puVar45 + 2);
          uStack_1a8 = 2;
          uStack_1a4 = 0;
          plStack_1a0 = (longlong *)&UNK_142b191b0;
          lStack_198 = 0x17;
          puStack_188 = &UNK_142b19398;
          uStack_180 = 2;
          ppiStack_178 = &piStack_2e8;
          auStack_170 = ZEXT816(1);
          uStack_1d8 = 0;
          uStack_1d4 = 0;
          uStack_1d0._0_4_ = 0x42b191b0;
          uStack_1d0._4_4_ = 1;
          uStack_1c8 = 0x17;
          uStack_1c4 = 0;
          uStack_1c0._0_4_ = 0;
          uStack_1c0._4_4_ = 0;
          uStack_1b8 = (undefined4)*puVar45;
          uStack_1b4 = (undefined4)((ulonglong)*puVar45 >> 0x20);
          uStack_1b0._0_4_ = (undefined4)puVar45[1];
          uStack_1b0._4_4_ = (undefined4)((ulonglong)puVar45[1] >> 0x20);
          uStack_190 = 1;
          func_0x000140d4a330(&uStack_72,&uStack_1d8);
        }
      }
      uVar41 = uStack_288;
      uStack_368 = CONCAT44(uStack_280,uStack_284);
      uStack_360 = CONCAT44(uStack_278,uStack_27c);
      uStack_358 = CONCAT44(uStack_270,uStack_274);
      uStack_350 = uStack_26c;
      uStack_34c = uStack_268;
      uStack_348 = uStack_264;
      plStack_344 = plStack_260;
      func_0x000142923c00(auStack_51c,auStack_5e0,0xc4);
      if (uStack_80 != 0) {
        func_0x000140613c20(plStack_1e0,lStack_90,8);
        plVar55 = plStack_200;
        if (uStack_1f8 != 0) {
          plVar47 = plStack_200 + 1;
          uVar57 = uStack_1f8;
          do {
            puVar3 = (undefined1 *)plVar47[-1];
            lVar49 = *plVar47;
            *puVar3 = 0;
            if (lVar49 != 0) {
              func_0x000140613c20(puVar3,lVar49,1);
            }
            plVar47 = plVar47 + 2;
            uVar57 = uVar57 - 1;
          } while (uVar57 != 0);
        }
      }
      if (lStack_208 != 0) {
        func_0x000140613c20(plVar55,lStack_208 << 4,8);
      }
      if (lStack_c0 != 0) {
        func_0x000140613c20(lStack_b8,lStack_c0 << 3,8);
      }
      if (*plStack_98 != 0) {
        func_0x000140613c20(lStack_a0,*plStack_98 << 4,8);
      }
      uStack_1a4 = uStack_348;
      plStack_1a0 = plStack_344;
      uStack_1b4 = (undefined4)uStack_358;
      uStack_1b0._0_4_ = (undefined4)((ulonglong)uStack_358 >> 0x20);
      uStack_1b0._4_4_ = uStack_350;
      uStack_1a8 = uStack_34c;
      uStack_1c4 = (undefined4)uStack_368;
      uStack_1c0._0_4_ = (undefined4)((ulonglong)uStack_368 >> 0x20);
      uStack_1c0._4_4_ = (undefined4)uStack_360;
      uStack_1b8 = (undefined4)((ulonglong)uStack_360 >> 0x20);
      func_0x000142923c00(&uStack_18c,auStack_51c,0xc4);
      uStack_1d8 = 1;
      uStack_1d4 = 0;
      uStack_1d0._0_4_ = 1;
      uStack_1d0._4_4_ = 0;
      uStack_1c8 = uVar41;
      uStack_190 = uVar42;
      cStack_c8 = cStack_81;
      lStack_198 = lVar52;
      lVar49 = func_0x000140613c10(0x118,8);
      if (lVar49 == 0) {
        FUN_1428d9000(8,0x118);
code_r0x000140c0a01c:
                    /* WARNING: Does not return */
        pcVar4 = (code *)invalidInstructionException();
        (*pcVar4)();
      }
      func_0x000142923c00(lVar49,&uStack_1d8,0x118);
      *(longlong *)(param_1 + 2) = lVar49;
      uVar43 = 0;
      goto code_r0x000140c09f64;
    }
  }
  if (plStack_260 != (longlong *)0x0) {
    LOCK();
    *plStack_260 = *plStack_260 + -1;
    UNLOCK();
    if (*plStack_260 == 0) {
      FUN_140c8fd00(&plStack_260);
    }
  }
  if (uStack_80 != 0) {
    func_0x000140613c20(plStack_1e0,lStack_90,8);
  }
  plVar55 = plStack_200;
  if (uStack_1f8 != 0) {
    plVar47 = plStack_200 + 1;
    uVar57 = uStack_1f8;
    do {
      puVar3 = (undefined1 *)plVar47[-1];
      lVar49 = *plVar47;
      *puVar3 = 0;
      if (lVar49 != 0) {
        func_0x000140613c20(puVar3,lVar49,1);
      }
      plVar47 = plVar47 + 2;
      uVar57 = uVar57 - 1;
    } while (uVar57 != 0);
  }
  if (lStack_208 != 0) {
    func_0x000140613c20(plVar55,lStack_208 << 4,8);
  }
  if (lStack_c0 != 0) {
    func_0x000140613c20(lStack_b8,lStack_c0 << 3,8);
  }
  if (*plStack_98 != 0) {
    func_0x000140613c20(lStack_a0,*plStack_98 << 4,8);
  }
  param_1[1] = uVar42;
  uVar43 = 1;
code_r0x000140c09f64:
  *param_1 = uVar43;
  return param_1;
}

