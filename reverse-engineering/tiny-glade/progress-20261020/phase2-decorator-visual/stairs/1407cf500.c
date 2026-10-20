
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void FUN_1407cf500(longlong *param_1,longlong *param_2,longlong *param_3,longlong *param_4,
                  undefined8 *param_5)

{
  char *pcVar1;
  float *pfVar2;
  undefined8 *puVar3;
  byte bVar4;
  byte bVar5;
  undefined4 *puVar6;
  undefined1 auVar7 [16];
  code *pcVar8;
  uint uVar9;
  uint uVar10;
  float fVar11;
  uint uVar12;
  float fVar13;
  float fVar14;
  uint uVar15;
  longlong lVar16;
  byte *pbVar17;
  undefined8 uVar18;
  longlong *plVar19;
  longlong *plVar20;
  longlong *plVar21;
  undefined *puVar22;
  ulonglong uVar23;
  char cVar24;
  longlong *plVar25;
  ushort uVar26;
  uint uVar27;
  longlong *in_R10;
  ulonglong uVar28;
  longlong lVar29;
  longlong *unaff_R12;
  longlong *plVar30;
  bool bVar31;
  longlong lVar32;
  undefined1 uVar33;
  byte bVar34;
  char cVar39;
  float fVar35;
  float extraout_XMM0_Da;
  char cVar37;
  char cVar38;
  undefined1 auVar36 [16];
  float fVar40;
  float fVar43;
  undefined1 auVar41 [16];
  undefined1 auVar42 [16];
  float fVar44;
  float fVar47;
  undefined1 auVar45 [16];
  undefined1 auVar46 [16];
  float fVar48;
  int iVar51;
  undefined1 auVar49 [16];
  float fVar52;
  int iVar53;
  undefined1 auVar50 [16];
  float fVar54;
  uint uVar55;
  float unaff_XMM6_Da;
  longlong lStack_3d0;
  undefined8 uStack_3c8;
  undefined8 uStack_3c0;
  longlong lStack_3b8;
  undefined1 auStack_3b0 [16];
  undefined8 uStack_3a0;
  undefined8 uStack_398;
  undefined8 uStack_390;
  longlong lStack_388;
  longlong lStack_380;
  longlong lStack_378;
  longlong lStack_370;
  longlong lStack_368;
  longlong lStack_360;
  longlong *plStack_348;
  longlong *plStack_340;
  longlong *plStack_338;
  undefined8 uStack_330;
  longlong lStack_328;
  undefined8 uStack_320;
  undefined8 uStack_318;
  longlong lStack_310;
  undefined1 auStack_308 [16];
  undefined8 uStack_2f8;
  longlong *plStack_2e8;
  longlong *plStack_2e0;
  longlong *plStack_2d8;
  longlong *plStack_2d0;
  longlong *plStack_2c8;
  longlong *plStack_2c0;
  longlong lStack_2b8;
  longlong *plStack_2b0;
  longlong *plStack_2a8;
  longlong *plStack_2a0;
  longlong *plStack_298;
  longlong *plStack_290;
  undefined8 uStack_288;
  undefined8 uStack_280;
  longlong *plStack_278;
  longlong *plStack_270;
  longlong *plStack_268;
  longlong *plStack_260;
  longlong *plStack_258;
  longlong lStack_250;
  longlong lStack_248;
  undefined8 uStack_240;
  longlong lStack_230;
  undefined8 uStack_228;
  longlong *plStack_208;
  longlong *plStack_200;
  longlong *plStack_1f8;
  undefined8 uStack_1f0;
  longlong *plStack_1e8;
  longlong *plStack_1e0;
  longlong *plStack_1d8;
  longlong *plStack_1d0;
  longlong *plStack_1c8;
  longlong *plStack_1c0;
  longlong *plStack_1b8;
  longlong *plStack_1b0;
  undefined1 auStack_1a8 [16];
  longlong *plStack_198;
  longlong *plStack_190;
  longlong *plStack_188;
  longlong *plStack_180;
  undefined8 uStack_178;
  undefined8 uStack_170;
  longlong *plStack_168;
  longlong *plStack_160;
  longlong *plStack_158;
  longlong *plStack_150;
  undefined8 uStack_148;
  longlong *plStack_140;
  longlong *plStack_138;
  int iStack_12c;
  int iStack_128;
  int iStack_124;
  longlong *plStack_120;
  undefined4 uStack_118;
  uint uStack_114;
  longlong *plStack_110;
  longlong *plStack_108;
  longlong *plStack_100;
  char cStack_f6;
  undefined1 uStack_f5;
  undefined1 uStack_f4;
  undefined1 uStack_f3;
  char cStack_f2;
  undefined1 uStack_f1;
  undefined8 uStack_f0;
  
  uStack_f0 = 0xfffffffffffffffe;
  cStack_f6 = func_0x000140c16a00();
  if (cStack_f6 == '\0') {
code_r0x0001407cf5ef:
    uStack_398 = 0;
  }
  else {
    unaff_R12 = (longlong *)&UNK_142a87a60;
    lVar32 = 0x5e;
    uVar23 = 0x5e;
    lVar16 = FUN_1407ceb10(&UNK_142a87a60);
    if (lVar16 == 1) {
      if (uVar23 != 0) {
        if (uVar23 < 0x5e) {
          if ((char)(&UNK_142a87a60)[uVar23] < -0x40) {
code_r0x0001407cf5cf:
            param_2 = (longlong *)0x5e;
            FUN_1428d9940(&UNK_142a87a60,0x5e,0,uVar23,&UNK_142a87c70);
            goto code_r0x0001407cf5ef;
          }
        }
        else if (uVar23 != 0x5e) goto code_r0x0001407cf5cf;
      }
      unaff_R12 = (longlong *)&UNK_142a87a60;
      lVar16 = FUN_1407ceb10(&UNK_142a87a60);
      if (lVar16 == 1) {
        uVar28 = uVar23 + 2;
        if (uVar28 != 0) {
          uVar33 = uVar28 == 0x5e;
          if (0x5d < uVar28) goto code_r0x0001407cf650;
          uVar33 = (&UNK_142a87a62)[uVar23] == -0x41;
          if ((char)(&UNK_142a87a62)[uVar23] < -0x40) {
            do {
              uVar23 = 0x5e;
              FUN_1428d9940(&UNK_142a87a60,0x5e);
code_r0x0001407cf650:
            } while (!(bool)uVar33);
          }
        }
        lVar32 = 0x5c - uVar23;
        unaff_R12 = (longlong *)(&UNK_142a87a60 + uVar28);
      }
    }
    puVar22 = &UNK_142a87f40;
    plVar30 = (longlong *)&UNK_142a87f00;
    do {
      bVar34 = puVar22[-1];
      uVar15 = (uint)(char)bVar34;
      plStack_100 = param_3;
      if ((char)bVar34 < '\0') {
        bVar4 = puVar22[-2];
        if ((char)bVar4 < -0x40) {
          bVar5 = puVar22[-3];
          if ((char)bVar5 < -0x40) {
            pbVar17 = puVar22 + -4;
            puVar22 = puVar22 + -4;
            uVar15 = bVar5 & 0x3f | (*pbVar17 & 7) << 6;
          }
          else {
            puVar22 = puVar22 + -3;
            uVar15 = bVar5 & 0xf;
          }
          uVar15 = bVar4 & 0x3f | uVar15 << 6;
        }
        else {
          puVar22 = puVar22 + -2;
          uVar15 = bVar4 & 0x1f;
        }
        uVar15 = (uint)(bVar34 & 0x3f) | uVar15 << 6;
        if (uVar15 != 0x5c) goto code_r0x0001407cf6f4;
code_r0x0001407cf705:
        plVar25 = (longlong *)(puVar22 + -0x142a87eff);
        if (plVar25 != (longlong *)0x0) {
          uVar33 = plVar25 == (longlong *)0x40;
          if ((longlong *)0x3f < plVar25) goto code_r0x0001407cf744;
          uVar33 = puVar22[1] == -0x41;
          if ((char)puVar22[1] < -0x40) {
            do {
              FUN_1428d9940(&UNK_142a87f00,0x40);
code_r0x0001407cf744:
            } while (!(bool)uVar33);
          }
        }
        plVar21 = (longlong *)(0x40 - (longlong)plVar25);
        plVar30 = plVar25 + 0x28550fe0;
        goto code_r0x0001407cf751;
      }
      puVar22 = puVar22 + -1;
      if (uVar15 == 0x5c) goto code_r0x0001407cf705;
code_r0x0001407cf6f4:
      if (uVar15 == 0x2f) goto code_r0x0001407cf705;
    } while (puVar22 != &UNK_142a87f00);
    plVar21 = (longlong *)0x40;
    plVar25 = (longlong *)&UNK_142a87f00;
code_r0x0001407cf751:
    plVar20 = (longlong *)func_0x000140c18130();
    if (*plVar20 == 1) {
      plVar20 = plVar20 + 1;
    }
    else {
      if (*plVar20 != 0) goto code_r0x0001407d0bc2;
      plVar20 = (longlong *)FUN_1428f2580(plVar20,0);
      if (plVar20 == (longlong *)0x0) goto code_r0x0001407d0bc2;
    }
    if (*plVar20 != 0) {
code_r0x0001407d0bf4:
      FUN_1428d9250(&UNK_142c21438);
code_r0x0001407d0c00:
      cStack_f2 = '\x01';
      uStack_f3 = 1;
      FUN_1428d9518(unaff_R12,in_R10,&UNK_142a87fa0);
code_r0x0001407d0daa:
                    /* WARNING: Does not return */
      pcVar8 = (code *)invalidInstructionException();
      (*pcVar8)();
    }
    *plVar20 = -1;
    param_2 = unaff_R12;
    plStack_110 = plVar20;
    uStack_390 = FUN_140c16d40(plVar20 + 1,unaff_R12,lVar32,plVar30,plVar21,1,0);
    *plStack_110 = *plStack_110 + 1;
    uStack_398 = 1;
    param_3 = plStack_100;
    uStack_330 = uStack_390;
  }
  puVar6 = (undefined4 *)param_1[2];
  *puVar6 = *(undefined4 *)((longlong)param_1 + 0x1c);
  param_1 = (longlong *)*param_1;
  unaff_XMM6_Da = (float)FUN_140c90540(puVar6,_UNK_142a7ba84);
  pbVar17 = (byte *)FUN_140d4d740();
  if ((*pbVar17 & 1) == 0) {
    lVar16 = FUN_140d64e00();
    pbVar17[0] = 1;
    pbVar17[1] = 0;
    pbVar17[2] = 0;
    pbVar17[3] = 0;
    pbVar17[4] = 0;
    pbVar17[5] = 0;
    pbVar17[6] = 0;
    pbVar17[7] = 0;
    *(longlong *)(pbVar17 + 8) = lVar16;
    *(longlong **)(pbVar17 + 0x10) = param_2;
  }
  else {
    lVar16 = *(longlong *)(pbVar17 + 8);
    param_2 = *(longlong **)(pbVar17 + 0x10);
  }
  plStack_1f8 = param_1 + 0x27;
  *(longlong *)(pbVar17 + 8) = lVar16 + 1;
  lVar32 = _UNK_142a87ee8;
  lStack_388 = *plStack_1f8;
  lStack_380 = param_1[0x28];
  lStack_378 = param_1[0x29];
  lStack_370 = param_1[0x2a];
  lStack_368 = param_1[0x2b];
  lStack_360 = param_1[0x2c];
  *plStack_1f8 = _UNK_142a87ee0;
  param_1[0x28] = lVar32;
  lVar32 = _UNK_142a87ef8;
  param_1[0x29] = _UNK_142a87ef0;
  param_1[0x2a] = lVar32;
  param_1[0x2b] = lVar16;
  param_1[0x2c] = (longlong)param_2;
  plVar25 = param_1 + 0x33;
  puVar22 = (undefined *)param_1[1];
  plVar21 = (longlong *)param_1[2];
  plVar20 = (longlong *)((ulonglong)((longlong)plVar21 + 1) >> 1);
  if (param_1[0x36] == 0) {
    plVar20 = plVar21;
  }
  plVar30 = param_3;
  if ((longlong *)param_1[0x35] < plVar20) goto code_r0x0001407d0bce;
code_r0x0001407cf8b4:
  FUN_141293420(puVar22,puVar22 + (longlong)plVar21 * 0x150,plVar25);
  fVar14 = _UNK_142a87794;
  fVar13 = _UNK_142a87790;
  uVar12 = _UNK_14292e980;
  fVar11 = _UNK_142925904;
  uVar10 = _UNK_1429258f4;
  uVar15 = _UNK_1429258f0;
  plStack_348 = (longlong *)param_1[1] + param_1[2] * 0x2a;
  uVar18 = *param_5;
  plStack_200 = (longlong *)*param_4;
  plStack_340 = plStack_200 + 4;
  plStack_1e0 = (longlong *)*plVar30;
  plStack_338 = plStack_1e0 + 4;
  plVar25 = (longlong *)param_1[1];
  uStack_1f0 = uVar18;
code_r0x0001407cf9a9:
  if (plVar25 == plStack_348) goto code_r0x0001407d0ac5;
  uStack_148 = func_0x000140959e70((int)plVar25[0x25],*(undefined4 *)((longlong)plVar25 + 300));
  plStack_1d8 = (longlong *)plVar25[1];
  plVar30 = (longlong *)plVar25[2];
  plStack_1c8 = (longlong *)plVar25[8];
  plStack_1b0 = (longlong *)plVar25[9];
  plStack_1c0 = (longlong *)((longlong)plStack_1c8 + (longlong)plStack_1b0 * 4);
  if (plVar30 <= plStack_1b0) {
    plStack_1b0 = plVar30;
  }
  in_R10 = plStack_1d8 + (longlong)plVar30;
  plStack_1b8 = (longlong *)0x0;
  auStack_1a8._0_8_ = plVar30;
  plStack_1d0 = in_R10;
  FUN_1402bd1e0(&plStack_160,&plStack_1d8,&UNK_142a879b8);
  plVar30 = plVar25 + 0x25;
  uStack_f1 = 1;
  plStack_100 = plVar25;
  lVar16 = FUN_140a1c930(uVar18,(int)*plVar30);
  if (lVar16 == 0) {
    iStack_124 = 0;
  }
  else {
    iStack_124 = *(int *)(lVar16 + 0x28);
  }
  uStack_f1 = 1;
  lVar16 = FUN_140a1c930(uVar18,(int)*plVar30);
  iVar53 = 0;
  if (lVar16 != 0) {
    uStack_f1 = 1;
    lVar16 = func_0x000140839490(lVar16 + 0x30);
    iVar53 = 0;
    if (lVar16 != 0) {
      iVar53 = *(int *)(lVar16 + 8);
    }
  }
  uStack_f1 = 1;
  lVar16 = FUN_140a1c930(uVar18,(int)*plVar30);
  if (lVar16 == 0) {
code_r0x0001407cfaf0:
    uStack_114 = 0;
  }
  else {
    uStack_f1 = 1;
    lVar16 = func_0x000140839490(lVar16 + 0x30);
    if (lVar16 == 0) goto code_r0x0001407cfaf0;
    uStack_114 = (uint)*(byte *)(lVar16 + 0x10);
  }
  uStack_f1 = 1;
  lVar16 = FUN_140a1c930(uVar18,(int)*plVar30);
  iStack_128 = 0;
  if (lVar16 != 0) {
    uStack_f1 = 1;
    lVar16 = func_0x000140839490(lVar16 + 0x30);
    if (lVar16 != 0) {
      iStack_128 = *(int *)(lVar16 + 0xc);
    }
  }
  if (plStack_200[3] != 0) {
    uStack_f1 = 1;
    uVar23 = FUN_1405b1be0(plStack_340,plVar30);
    bVar34 = (byte)(uVar23 >> 0x39);
    auVar36 = ZEXT216(CONCAT11(bVar34,bVar34));
    auVar36 = pshuflw(auVar36,auVar36,0);
    lVar16 = 0;
    while( true ) {
      uVar23 = plStack_200[1] & uVar23;
      pcVar1 = (char *)(*plStack_200 + uVar23);
      cVar24 = auVar36[0];
      auVar45[0] = -(*pcVar1 == cVar24);
      cVar37 = auVar36[1];
      auVar45[1] = -(pcVar1[1] == cVar37);
      cVar38 = auVar36[2];
      auVar45[2] = -(pcVar1[2] == cVar38);
      cVar39 = auVar36[3];
      auVar45[3] = -(pcVar1[3] == cVar39);
      auVar45[4] = -(pcVar1[4] == cVar24);
      auVar45[5] = -(pcVar1[5] == cVar37);
      auVar45[6] = -(pcVar1[6] == cVar38);
      auVar45[7] = -(pcVar1[7] == cVar39);
      auVar45[8] = -(pcVar1[8] == cVar24);
      auVar45[9] = -(pcVar1[9] == cVar37);
      auVar45[10] = -(pcVar1[10] == cVar38);
      auVar45[0xb] = -(pcVar1[0xb] == cVar39);
      auVar45[0xc] = -(pcVar1[0xc] == cVar24);
      auVar45[0xd] = -(pcVar1[0xd] == cVar37);
      auVar45[0xe] = -(pcVar1[0xe] == cVar38);
      auVar45[0xf] = -(pcVar1[0xf] == cVar39);
      uVar26 = (ushort)(SUB161(auVar45 >> 7,0) & 1) | (ushort)(SUB161(auVar45 >> 0xf,0) & 1) << 1 |
               (ushort)(SUB161(auVar45 >> 0x17,0) & 1) << 2 |
               (ushort)(SUB161(auVar45 >> 0x1f,0) & 1) << 3 |
               (ushort)(SUB161(auVar45 >> 0x27,0) & 1) << 4 |
               (ushort)(SUB161(auVar45 >> 0x2f,0) & 1) << 5 |
               (ushort)(SUB161(auVar45 >> 0x37,0) & 1) << 6 |
               (ushort)(SUB161(auVar45 >> 0x3f,0) & 1) << 7 |
               (ushort)(SUB161(auVar45 >> 0x47,0) & 1) << 8 |
               (ushort)(SUB161(auVar45 >> 0x4f,0) & 1) << 9 |
               (ushort)(SUB161(auVar45 >> 0x57,0) & 1) << 10 |
               (ushort)(SUB161(auVar45 >> 0x5f,0) & 1) << 0xb |
               (ushort)(SUB161(auVar45 >> 0x67,0) & 1) << 0xc |
               (ushort)(SUB161(auVar45 >> 0x6f,0) & 1) << 0xd |
               (ushort)(SUB161(auVar45 >> 0x77,0) & 1) << 0xe | (ushort)(auVar45[0xf] >> 7) << 0xf;
      uVar55 = (uint)uVar26;
      if (uVar26 != 0) {
        do {
          uVar27 = 0;
          for (uVar9 = uVar55; (uVar9 & 1) == 0; uVar9 = uVar9 >> 1 | 0x80000000) {
            uVar27 = uVar27 + 1;
          }
          uVar28 = uVar27 + uVar23 & plStack_200[1];
          lVar32 = *plStack_200 + uVar28 * -8;
          uVar18 = *(undefined8 *)(lVar32 + -8);
          auVar49._0_4_ = -(uint)((int)uVar18 == (int)*plVar30);
          iVar51 = -(uint)((int)((ulonglong)uVar18 >> 0x20) == (int)((ulonglong)*plVar30 >> 0x20));
          auVar49._4_4_ = auVar49._0_4_;
          auVar49._8_4_ = iVar51;
          auVar49._12_4_ = iVar51;
          uVar27 = movmskpd((int)uVar28 * 8,auVar49);
          in_R10 = (longlong *)(ulonglong)uVar27;
          uStack_118 = (undefined4)CONCAT71((int7)((ulonglong)lVar32 >> 8),1);
          if (uVar27 == 3) goto code_r0x0001407cfc2a;
          uVar26 = (ushort)(uVar55 - 1) & (ushort)uVar55;
          uVar55 = CONCAT22((short)(uVar55 - 1 >> 0x10),uVar26);
          in_R10 = (longlong *)(ulonglong)uVar55;
        } while (uVar26 != 0);
      }
      auVar41[0] = -(*pcVar1 == UNK_142925990);
      auVar41[1] = -(pcVar1[1] == UNK_142925991);
      auVar41[2] = -(pcVar1[2] == UNK_142925992);
      auVar41[3] = -(pcVar1[3] == UNK_142925993);
      auVar41[4] = -(pcVar1[4] == UNK_142925994);
      auVar41[5] = -(pcVar1[5] == UNK_142925995);
      auVar41[6] = -(pcVar1[6] == UNK_142925996);
      auVar41[7] = -(pcVar1[7] == UNK_142925997);
      auVar41[8] = -(pcVar1[8] == UNK_142925998);
      auVar41[9] = -(pcVar1[9] == UNK_142925999);
      auVar41[10] = -(pcVar1[10] == UNK_14292599a);
      auVar41[0xb] = -(pcVar1[0xb] == UNK_14292599b);
      auVar41[0xc] = -(pcVar1[0xc] == UNK_14292599c);
      auVar41[0xd] = -(pcVar1[0xd] == UNK_14292599d);
      auVar41[0xe] = -(pcVar1[0xe] == UNK_14292599e);
      auVar41[0xf] = -(pcVar1[0xf] == UNK_14292599f);
      if ((((((((((((((((SUB161(auVar41 >> 7,0) & 1) != 0 || (SUB161(auVar41 >> 0xf,0) & 1) != 0) ||
                      (SUB161(auVar41 >> 0x17,0) & 1) != 0) || (SUB161(auVar41 >> 0x1f,0) & 1) != 0)
                    || (SUB161(auVar41 >> 0x27,0) & 1) != 0) || (SUB161(auVar41 >> 0x2f,0) & 1) != 0
                   ) || (SUB161(auVar41 >> 0x37,0) & 1) != 0) ||
                 (SUB161(auVar41 >> 0x3f,0) & 1) != 0) || (SUB161(auVar41 >> 0x47,0) & 1) != 0) ||
               (SUB161(auVar41 >> 0x4f,0) & 1) != 0) || (SUB161(auVar41 >> 0x57,0) & 1) != 0) ||
             (SUB161(auVar41 >> 0x5f,0) & 1) != 0) || (SUB161(auVar41 >> 0x67,0) & 1) != 0) ||
           (SUB161(auVar41 >> 0x6f,0) & 1) != 0) || (SUB161(auVar41 >> 0x77,0) & 1) != 0) ||
          auVar41[0xf] < '\0') break;
      uVar23 = uVar23 + lVar16 + 0x10;
      lVar16 = lVar16 + 0x10;
    }
  }
  uStack_118 = 0;
code_r0x0001407cfc2a:
  uVar18 = FUN_1405b0d70(&lStack_368,&uStack_148);
  uStack_f1 = 1;
  FUN_140b76b50(&plStack_1d8,&lStack_388,uVar18,&uStack_148);
  plVar21 = plStack_1c0;
  plVar25 = plStack_1c8;
  plStack_208 = plStack_100 + 0x2a;
  iStack_12c = iVar53;
  if (SBORROW8(0,(longlong)plStack_1d0)) goto code_r0x0001407d0330;
  uStack_280 = plStack_168;
  plStack_290 = uStack_178;
  uStack_288 = uStack_170;
  plStack_2a0 = plStack_188;
  plStack_298 = plStack_180;
  plStack_2b0 = plStack_198;
  plStack_2a8 = plStack_190;
  plStack_2c0 = (longlong *)auStack_1a8._0_8_;
  lStack_2b8 = auStack_1a8._8_8_;
  plStack_2d0 = plStack_1b8;
  plStack_2c8 = plStack_1b0;
  plStack_2e0 = plStack_1c8;
  plStack_2d8 = plStack_1c0;
  plStack_140 = plStack_1d0;
  plStack_2e8 = plStack_1d0;
  auVar36._8_8_ = 0;
  auVar36._0_8_ = plStack_1c0;
  uVar23 = SUB168(auVar36 * ZEXT816(0xc),0);
  plStack_1e8 = plVar30;
  if (SUB168(auVar36 * ZEXT816(0xc),8) != 0) {
    plVar30 = (longlong *)0x0;
code_r0x0001407d0d66:
    uStack_f4 = 1;
    uStack_f5 = 1;
    FUN_1428d8fe3(plVar30,uVar23,&UNK_142c22010);
    goto code_r0x0001407d0daa;
  }
  plVar30 = (longlong *)0x0;
  if (0x7ffffffffffffffc < uVar23) goto code_r0x0001407d0d66;
  if (uVar23 == 0) {
    plVar19 = (longlong *)0x4;
    plVar20 = (longlong *)0x0;
  }
  else {
    plVar30 = (longlong *)0x4;
    plVar19 = (longlong *)func_0x000140613c10(uVar23,4);
    plVar20 = plVar21;
    if (plVar19 == (longlong *)0x0) goto code_r0x0001407d0d66;
  }
  func_0x000142923c00(plVar19,plVar25,uVar23);
  plStack_1c8 = plVar21;
  uStack_f4 = 1;
  uStack_f5 = 1;
  plStack_1d8 = plVar20;
  plStack_1d0 = plVar19;
  FUN_1412699d0(&lStack_3d0,&plStack_1d8,1);
  plVar20 = plStack_140;
  if (!SBORROW8(0,lStack_3d0)) {
    uStack_2f8 = uStack_3a0;
    auStack_308 = auStack_3b0;
    uStack_318 = uStack_3c0;
    lStack_310 = lStack_3b8;
    lStack_328 = lStack_3d0;
    uStack_320 = uStack_3c8;
    plStack_138 = (longlong *)plStack_100[4];
    uVar23 = plStack_100[2];
    if ((ulonglong)plStack_100[5] < (ulonglong)plStack_100[2]) {
      uVar23 = plStack_100[5];
    }
    unaff_R12 = (longlong *)0x0;
    bVar31 = false;
    plStack_108 = plStack_158;
    plStack_120 = plStack_150;
    plStack_110 = plVar21;
    plVar20 = plStack_158;
    plVar19 = plStack_100;
    in_R10 = plStack_150;
    for (uVar28 = 0; uVar28 < uVar23; uVar28 = uVar28 + 1) {
      if (in_R10 <= unaff_R12) goto code_r0x0001407d0c00;
      if ((longlong *)plVar19[0xf] <= unaff_R12) {
        cStack_f2 = '\x01';
        uStack_f3 = 1;
        FUN_1428d9518(unaff_R12,(longlong *)plVar19[0xf],&UNK_142a87fb8);
        goto code_r0x0001407d0daa;
      }
      plVar30 = (longlong *)((longlong)unaff_R12 * 3);
      lVar16 = plVar19[0xe];
      if (plVar21 == (longlong *)plVar19[2]) {
        if (plVar21 <= unaff_R12) {
          cStack_f2 = '\x01';
          uStack_f3 = 1;
          FUN_1428d9518(unaff_R12,plVar21,&UNK_142a88000);
          goto code_r0x0001407d0daa;
        }
        if (plStack_2c0 <= unaff_R12) {
          cStack_f2 = '\x01';
          uStack_f3 = 1;
          FUN_1428d9518(unaff_R12,plStack_2c0,&UNK_142a88018);
          goto code_r0x0001407d0daa;
        }
        fVar44 = *(float *)((longlong)plVar25 + (longlong)unaff_R12 * 0xc + 8);
        pfVar2 = (float *)((longlong)plVar25 + (longlong)unaff_R12 * 0xc);
        fVar47 = *pfVar2;
        fVar54 = pfVar2[1];
        fVar48 = *(float *)(plStack_2c8 + (longlong)unaff_R12);
        fVar52 = *(float *)((longlong)(plStack_2c8 + (longlong)unaff_R12) + 4);
      }
      else {
        cStack_f2 = '\x01';
        uStack_f3 = 1;
        uVar55 = FUN_141269710(&lStack_328,*(undefined4 *)((longlong)plStack_138 + uVar28 * 4));
        plVar21 = (longlong *)(ulonglong)uVar55;
        if (plStack_110 <= plVar21) {
          cStack_f2 = '\x01';
          uStack_f3 = 1;
          FUN_1428d9518(plVar21,plStack_110,&UNK_142a87fd0);
          goto code_r0x0001407d0daa;
        }
        plVar20 = (longlong *)((longlong)plVar21 + 1);
        if (plStack_110 <= plVar20) {
          cStack_f2 = '\x01';
          uStack_f3 = 1;
          FUN_1428d9518(plVar20,plStack_110,&UNK_142a87fd0);
          goto code_r0x0001407d0daa;
        }
        if (plStack_2c0 <= plVar21) {
          cStack_f2 = '\x01';
          uStack_f3 = 1;
          FUN_1428d9518(plVar21,plStack_2c0,&UNK_142a87fe8);
          goto code_r0x0001407d0daa;
        }
        if (plStack_2c0 <= plVar20) {
          cStack_f2 = '\x01';
          uStack_f3 = 1;
          FUN_1428d9518(plVar20,plStack_2c0,&UNK_142a87fe8);
          goto code_r0x0001407d0daa;
        }
        fVar44 = *(float *)((longlong)plVar25 + (longlong)plVar21 * 0xc + 8);
        uVar18 = *(undefined8 *)((longlong)plVar25 + (longlong)plVar21 * 0xc);
        fVar47 = (float)uVar18;
        fVar54 = (float)((ulonglong)uVar18 >> 0x20);
        uVar18 = *(undefined8 *)((longlong)plVar25 + (longlong)plVar20 * 0xc);
        fVar47 = ((float)uVar18 - fVar47) * extraout_XMM0_Da + fVar47;
        fVar54 = ((float)((ulonglong)uVar18 >> 0x20) - fVar54) * extraout_XMM0_Da + fVar54;
        fVar44 = (*(float *)((longlong)plVar25 + (longlong)plVar20 * 0xc + 8) - fVar44) *
                 extraout_XMM0_Da + fVar44;
        fVar48 = (float)plStack_2c8[(longlong)plVar21];
        fVar52 = (float)((ulonglong)plStack_2c8[(longlong)plVar21] >> 0x20);
        fVar48 = ((float)plStack_2c8[(longlong)((longlong)plVar21 + 1)] - fVar48) * extraout_XMM0_Da
                 + fVar48;
        fVar52 = ((float)((ulonglong)plStack_2c8[(longlong)((longlong)plVar21 + 1)] >> 0x20) -
                 fVar52) * extraout_XMM0_Da + fVar52;
        plVar21 = plStack_110;
        plVar20 = plStack_108;
        plVar19 = plStack_100;
        in_R10 = plStack_120;
      }
      puVar3 = (undefined8 *)((longlong)plVar20 + (longlong)unaff_R12 * 0xc);
      uVar18 = *(undefined8 *)(lVar16 + (longlong)unaff_R12 * 8);
      fVar35 = *(float *)(puVar3 + 1) - fVar44;
      fVar40 = (float)*puVar3 - fVar47;
      fVar43 = (float)((ulonglong)*puVar3 >> 0x20) - fVar54;
      *puVar3 = CONCAT44(unaff_XMM6_Da * fVar43 + fVar54,unaff_XMM6_Da * fVar40 + fVar47);
      *(float *)(puVar3 + 1) = unaff_XMM6_Da * fVar35 + fVar44;
      uVar55 = (uint)((float)((ulonglong)uVar18 >> 0x20) * fVar52 + (float)uVar18 * fVar48 < 0.0);
      uVar27 = (int)(uVar55 << 0x1f) >> 0x1f;
      uVar55 = (int)(uVar55 << 0x1f) >> 0x1f;
      fVar44 = (float)(~uVar27 & (uint)fVar48 | ((uint)fVar48 ^ uVar15) & uVar27);
      fVar47 = (float)(~uVar55 & (uint)fVar52 | ((uint)fVar52 ^ uVar10) & uVar55);
      uVar18 = *(undefined8 *)(lVar16 + (longlong)unaff_R12 * 8);
      fVar44 = ((float)uVar18 - fVar44) * unaff_XMM6_Da + fVar44;
      fVar47 = ((float)((ulonglong)uVar18 >> 0x20) - fVar47) * unaff_XMM6_Da + fVar47;
      fVar54 = fVar11 / SQRT(fVar47 * fVar47 + fVar44 * fVar44);
      uVar55 = (uint)((int)fVar54 - 1U < 0x7fffff ||
                     (int)ABS(fVar54) - 0x800000U < 0x7f000000 && -1 < (int)fVar54);
      uVar27 = (int)(uVar55 << 0x1f) >> 0x1f;
      *(ulonglong *)(lVar16 + (longlong)unaff_R12 * 8) =
           (ulonglong)(~uVar27 & (uint)fVar11) |
           CONCAT44((uint)(fVar54 * fVar47) & (int)(uVar55 << 0x1f) >> 0x1f,
                    (uint)(fVar54 * fVar44) & uVar27);
      unaff_R12 = (longlong *)((longlong)unaff_R12 + 1);
      bVar31 = (bool)(bVar31 | fVar14 < fVar35 * fVar35 + fVar43 * fVar43 + fVar40 * fVar40);
    }
    if ((!bVar31 &&
         (((((uStack_280._4_1_ ^ (byte)uStack_118) & 1) == 0 && (char)uStack_114 == uStack_280._5_1_
           ) && iStack_128 == (int)uStack_280) &&
         (uStack_288._4_4_ == iStack_12c && (int)uStack_288 == iStack_124))) &&
       (plVar21 = (longlong *)plVar19[0xc], plVar21 == plStack_2a8)) {
      if (plVar21 != (longlong *)0x0) {
        plVar20 = (longlong *)0x0;
        do {
          if (fVar13 < (float)((uint)(*(float *)(plVar19[0xb] + (longlong)plVar20 * 4) -
                                     *(float *)((longlong)plStack_2b0 + (longlong)plVar20 * 4)) &
                              uVar12)) goto code_r0x0001407d0270;
          plVar20 = (longlong *)((longlong)plVar20 + 1);
        } while (plVar21 != plVar20);
      }
      if ((*pcRam000000014362c018 == '\x01') && (plStack_1e0[3] != 0)) {
        cStack_f2 = '\x01';
        uStack_f3 = 1;
        uVar23 = FUN_1405b1be0(plStack_338,plStack_1e8);
        param_1 = plStack_1e8;
        lVar16 = *plStack_1e0;
        bVar34 = (byte)(uVar23 >> 0x39);
        auVar36 = ZEXT216(CONCAT11(bVar34,bVar34));
        auVar36 = pshuflw(auVar36,auVar36,0);
        lVar32 = 0;
        while( true ) {
          uVar23 = plStack_1e0[1] & uVar23;
          auVar45 = *(undefined1 (*) [16])(lVar16 + uVar23);
          cVar24 = auVar36[0];
          auVar46[0] = -(auVar45[0] == cVar24);
          cVar37 = auVar36[1];
          auVar46[1] = -(auVar45[1] == cVar37);
          cVar38 = auVar36[2];
          auVar46[2] = -(auVar45[2] == cVar38);
          cVar39 = auVar36[3];
          auVar46[3] = -(auVar45[3] == cVar39);
          auVar46[4] = -(auVar45[4] == cVar24);
          auVar46[5] = -(auVar45[5] == cVar37);
          auVar46[6] = -(auVar45[6] == cVar38);
          auVar46[7] = -(auVar45[7] == cVar39);
          auVar46[8] = -(auVar45[8] == cVar24);
          auVar46[9] = -(auVar45[9] == cVar37);
          auVar46[10] = -(auVar45[10] == cVar38);
          auVar46[0xb] = -(auVar45[0xb] == cVar39);
          auVar46[0xc] = -(auVar45[0xc] == cVar24);
          auVar46[0xd] = -(auVar45[0xd] == cVar37);
          auVar46[0xe] = -(auVar45[0xe] == cVar38);
          auVar46[0xf] = -(auVar45[0xf] == cVar39);
          uVar26 = (ushort)(SUB161(auVar46 >> 7,0) & 1) |
                   (ushort)(SUB161(auVar46 >> 0xf,0) & 1) << 1 |
                   (ushort)(SUB161(auVar46 >> 0x17,0) & 1) << 2 |
                   (ushort)(SUB161(auVar46 >> 0x1f,0) & 1) << 3 |
                   (ushort)(SUB161(auVar46 >> 0x27,0) & 1) << 4 |
                   (ushort)(SUB161(auVar46 >> 0x2f,0) & 1) << 5 |
                   (ushort)(SUB161(auVar46 >> 0x37,0) & 1) << 6 |
                   (ushort)(SUB161(auVar46 >> 0x3f,0) & 1) << 7 |
                   (ushort)(SUB161(auVar46 >> 0x47,0) & 1) << 8 |
                   (ushort)(SUB161(auVar46 >> 0x4f,0) & 1) << 9 |
                   (ushort)(SUB161(auVar46 >> 0x57,0) & 1) << 10 |
                   (ushort)(SUB161(auVar46 >> 0x5f,0) & 1) << 0xb |
                   (ushort)(SUB161(auVar46 >> 0x67,0) & 1) << 0xc |
                   (ushort)(SUB161(auVar46 >> 0x6f,0) & 1) << 0xd |
                   (ushort)(SUB161(auVar46 >> 0x77,0) & 1) << 0xe |
                   (ushort)(auVar46[0xf] >> 7) << 0xf;
          in_R10 = (longlong *)(ulonglong)uVar26;
          if (uVar26 != 0) {
            do {
              uVar55 = 0;
              for (uVar27 = (uint)in_R10; (uVar27 & 1) == 0; uVar27 = uVar27 >> 1 | 0x80000000) {
                uVar55 = uVar55 + 1;
              }
              lVar29 = (uVar55 + uVar23 & plStack_1e0[1]) * -0x110;
              uVar18 = *(undefined8 *)(lVar16 + -0x110 + lVar29);
              auVar50._0_4_ = -(uint)((int)uVar18 == (int)*plStack_1e8);
              iVar53 = -(uint)((int)((ulonglong)uVar18 >> 0x20) ==
                              (int)((ulonglong)*plStack_1e8 >> 0x20));
              auVar50._4_4_ = auVar50._0_4_;
              auVar50._8_4_ = iVar53;
              auVar50._12_4_ = iVar53;
              uVar55 = movmskpd((int)uVar28,auVar50);
              uVar28 = (ulonglong)uVar55;
              if (uVar55 == 3) {
                if (*(char *)(lVar16 + -8 + lVar29) != '\0') goto code_r0x0001407d0270;
                if (plStack_120 == plStack_110) {
                  lVar16 = (longlong)plStack_120 * 0xc;
                  if (plStack_120 == (longlong *)0x0) {
                    plVar20 = (longlong *)0x4;
                  }
                  else {
                    plVar20 = (longlong *)func_0x000140613c10(lVar16,4);
                    if (plVar20 == (longlong *)0x0) {
                      cStack_f2 = '\x01';
                      uStack_f3 = 1;
                      FUN_1428d8fe3(4,lVar16,&UNK_142c22010);
                      goto code_r0x0001407d0daa;
                    }
                  }
                  func_0x000142923c00(plVar20,plVar25,lVar16);
                  plStack_278 = plStack_120;
                  plStack_268 = plStack_120;
                  plVar21 = (longlong *)CONCAT71((int7)((ulonglong)plVar25 >> 8),1);
                  plStack_270 = plVar20;
                }
                else {
                  plStack_268 = plStack_150;
                  plStack_278 = plStack_160;
                  plStack_270 = plStack_158;
                  plVar21 = (longlong *)0x0;
                }
                cVar24 = (char)plVar21;
                uStack_f3 = 1;
                cStack_f2 = cVar24;
                FUN_141269ce0(&plStack_1d8,&plStack_278,1);
                param_4 = plStack_1d8;
                if (!SBORROW8(0,(longlong)plStack_1d8)) {
                  plStack_258 = plStack_1b0;
                  lStack_250 = auStack_1a8._0_8_;
                  plStack_268 = plStack_1c0;
                  plStack_260 = plStack_1b8;
                  plStack_278 = plStack_1d0;
                  plStack_270 = plStack_1c8;
                }
                lVar16 = plStack_100[0x13];
                if (!SBORROW8(0,lVar16)) {
                  if (lVar16 != 0) {
                    func_0x000140613c20(plStack_100[0x14],lVar16 * 0xc,4);
                  }
                  if (plStack_100[0x16] != 0) {
                    func_0x000140613c20(plStack_100[0x17],plStack_100[0x16] << 2,4);
                  }
                }
                plStack_100[0x13] = (longlong)param_4;
                plStack_100[0x18] = (longlong)plStack_258;
                plStack_100[0x19] = lStack_250;
                plStack_100[0x16] = (longlong)plStack_268;
                plStack_100[0x17] = (longlong)plStack_260;
                plStack_100[0x14] = (longlong)plStack_278;
                plStack_100[0x15] = (longlong)plStack_270;
                uStack_178 = uStack_288;
                uStack_170 = uStack_280;
                plStack_188 = plStack_298;
                plStack_180 = plStack_290;
                plStack_198 = plStack_2a8;
                plStack_190 = plStack_2a0;
                auStack_1a8._8_8_ = plStack_2b0;
                auStack_1a8._0_8_ = lStack_2b8;
                plStack_1b8 = plStack_2c8;
                plStack_1b0 = plStack_2c0;
                plStack_1c8 = plStack_2d8;
                plStack_1c0 = plStack_2d0;
                plStack_1d8 = plStack_2e8;
                plStack_1d0 = plStack_2e0;
                uStack_f3 = 0;
                cStack_f2 = cVar24;
                FUN_141266df0(&plStack_278,plStack_1f8,uStack_148,&plStack_1d8);
                uVar18 = uStack_1f0;
                if (!SBORROW8(0,(longlong)plStack_278)) {
                  if (plStack_278 != (longlong *)0x0) {
                    func_0x000140613c20(plStack_270,(longlong)plStack_278 * 0xc,4);
                  }
                  if (plStack_260 != (longlong *)0x0) {
                    func_0x000140613c20(plStack_258,(longlong)plStack_260 << 3,4);
                  }
                  if (lStack_248 != 0) {
                    func_0x000140613c20(uStack_240,lStack_248 << 2,4);
                  }
                  if (lStack_230 != 0) {
                    func_0x000140613c20(uStack_228,lStack_230 << 2,4);
                  }
                }
                if (lStack_328 != 0) {
                  func_0x000140613c20(uStack_320,lStack_328 * 0xc,4);
                }
                if (lStack_310 != 0) {
                  func_0x000140613c20(auStack_308._0_8_,lStack_310 << 2,4);
                }
                plVar25 = plStack_208;
                if ((cVar24 != '\0') && (plStack_160 != (longlong *)0x0)) {
                  func_0x000140613c20(plStack_108,(longlong)plStack_160 * 0xc,4);
                  plVar25 = plStack_208;
                }
                goto code_r0x0001407cf9a9;
              }
              iVar53 = (uint)in_R10 - 1;
              uVar26 = (ushort)iVar53 & (ushort)in_R10;
              in_R10 = (longlong *)(ulonglong)CONCAT22((short)((uint)iVar53 >> 0x10),uVar26);
            } while (uVar26 != 0);
          }
          auVar42[0] = -(auVar45[0] == UNK_142925990);
          auVar42[1] = -(auVar45[1] == UNK_142925991);
          auVar42[2] = -(auVar45[2] == UNK_142925992);
          auVar42[3] = -(auVar45[3] == UNK_142925993);
          auVar42[4] = -(auVar45[4] == UNK_142925994);
          auVar42[5] = -(auVar45[5] == UNK_142925995);
          auVar42[6] = -(auVar45[6] == UNK_142925996);
          auVar42[7] = -(auVar45[7] == UNK_142925997);
          auVar42[8] = -(auVar45[8] == UNK_142925998);
          auVar42[9] = -(auVar45[9] == UNK_142925999);
          auVar42[10] = -(auVar45[10] == UNK_14292599a);
          auVar42[0xb] = -(auVar45[0xb] == UNK_14292599b);
          auVar42[0xc] = -(auVar45[0xc] == UNK_14292599c);
          auVar42[0xd] = -(auVar45[0xd] == UNK_14292599d);
          auVar42[0xe] = -(auVar45[0xe] == UNK_14292599e);
          auVar42[0xf] = -(auVar45[0xf] == UNK_14292599f);
          uVar26 = (ushort)(SUB161(auVar42 >> 7,0) & 1) |
                   (ushort)(SUB161(auVar42 >> 0xf,0) & 1) << 1 |
                   (ushort)(SUB161(auVar42 >> 0x17,0) & 1) << 2 |
                   (ushort)(SUB161(auVar42 >> 0x1f,0) & 1) << 3 |
                   (ushort)(SUB161(auVar42 >> 0x27,0) & 1) << 4 |
                   (ushort)(SUB161(auVar42 >> 0x2f,0) & 1) << 5 |
                   (ushort)(SUB161(auVar42 >> 0x37,0) & 1) << 6 |
                   (ushort)(SUB161(auVar42 >> 0x3f,0) & 1) << 7 |
                   (ushort)(SUB161(auVar42 >> 0x47,0) & 1) << 8 |
                   (ushort)(SUB161(auVar42 >> 0x4f,0) & 1) << 9 |
                   (ushort)(SUB161(auVar42 >> 0x57,0) & 1) << 10 |
                   (ushort)(SUB161(auVar42 >> 0x5f,0) & 1) << 0xb |
                   (ushort)(SUB161(auVar42 >> 0x67,0) & 1) << 0xc |
                   (ushort)(SUB161(auVar42 >> 0x6f,0) & 1) << 0xd |
                   (ushort)(SUB161(auVar42 >> 0x77,0) & 1) << 0xe |
                   (ushort)(auVar42[0xf] >> 7) << 0xf;
          in_R10 = (longlong *)(ulonglong)uVar26;
          if (uVar26 != 0) break;
          uVar23 = uVar23 + lVar32 + 0x10;
          lVar32 = lVar32 + 0x10;
        }
      }
    }
code_r0x0001407d0270:
    plVar20 = plStack_140;
    if (lStack_328 != 0) {
      func_0x000140613c20(uStack_320,lStack_328 * 0xc,4);
    }
    if (lStack_310 != 0) {
      func_0x000140613c20(auStack_308._0_8_,lStack_310 << 2,4);
    }
  }
  if (plVar20 != (longlong *)0x0) {
    func_0x000140613c20(plVar25,(longlong)plVar20 * 0xc,4);
  }
  if (plStack_2d0 != (longlong *)0x0) {
    func_0x000140613c20(plStack_2c8,(longlong)plStack_2d0 << 3,4);
  }
  if (lStack_2b8 != 0) {
    func_0x000140613c20(plStack_2b0,lStack_2b8 << 2,4);
  }
  if (plStack_2a0 != (longlong *)0x0) {
    func_0x000140613c20(plStack_298,(longlong)plStack_2a0 << 2,4);
  }
code_r0x0001407d0330:
  plVar25 = plStack_150;
  plVar30 = plStack_158;
  auVar7._8_8_ = 0;
  auVar7._0_8_ = plStack_150;
  uVar23 = SUB168(auVar7 * ZEXT816(0xc),0);
  if (SUB168(auVar7 * ZEXT816(0xc),8) == 0) {
    uVar18 = 0;
    if (uVar23 < 0x7ffffffffffffffd) {
      if (uVar23 == 0) {
        plVar20 = (longlong *)0x4;
        plVar21 = (longlong *)0x0;
      }
      else {
        uVar18 = 4;
        plVar20 = (longlong *)func_0x000140613c10(uVar23,4);
        plVar21 = plVar25;
        if (plVar20 == (longlong *)0x0) goto code_r0x0001407d0d31;
      }
      func_0x000142923c00(plVar20,plVar30,uVar23);
      param_1 = (longlong *)plStack_100[0xf];
      uVar23 = (longlong)param_1 * 8;
      plStack_110 = plVar21;
      plStack_108 = plVar20;
      if ((ulonglong)param_1 >> 0x3d == 0) {
        uVar18 = 0;
        if (uVar23 < 0x7ffffffffffffffd) {
          lVar16 = plStack_100[0xe];
          if (uVar23 == 0) {
            plVar21 = (longlong *)0x4;
            plStack_120 = (longlong *)0x0;
            plVar30 = plStack_120;
          }
          else {
            uVar18 = 4;
            plVar21 = (longlong *)func_0x000140613c10(uVar23,4);
            plVar30 = param_1;
            if (plVar21 == (longlong *)0x0) goto code_r0x0001407d0d4f;
          }
          plStack_120 = plVar30;
          plStack_138 = plVar21;
          func_0x000142923c00(plVar21,lVar16,uVar23);
          plVar30 = (longlong *)plStack_100[0xc];
          param_4 = (longlong *)((longlong)plVar30 * 4);
          if ((ulonglong)plVar30 >> 0x3e == 0) {
            uVar18 = 0;
            if (param_4 < (longlong *)0x7ffffffffffffffd) {
              lVar16 = plStack_100[0xb];
              if (param_4 == (longlong *)0x0) {
                lVar32 = 4;
                unaff_R12 = (longlong *)0x0;
              }
              else {
                uVar18 = 4;
                lVar32 = func_0x000140613c10(param_4,4);
                unaff_R12 = plVar30;
                if (lVar32 == 0) goto code_r0x0001407d0d1a;
              }
              func_0x000142923c00(lVar32,lVar16,param_4);
              plStack_1d8 = plStack_110;
              plStack_1d0 = plStack_108;
              plStack_1c8 = plVar25;
              plStack_1c0 = plStack_120;
              plStack_1b8 = plStack_138;
              auStack_1a8._8_8_ = lVar32;
              auStack_1a8._0_8_ = unaff_R12;
              plStack_190 = (longlong *)0x0;
              plStack_188 = (longlong *)0x4;
              plStack_180 = (longlong *)0x0;
              uStack_178 = (longlong *)CONCAT44(iStack_12c,iStack_124);
              uStack_170 = (longlong *)
                           CONCAT44(CONCAT31(CONCAT21(uStack_170._6_2_,(char)uStack_114),
                                             (char)uStack_118),iStack_128);
              uStack_f1 = 1;
              plStack_1b0 = param_1;
              plStack_198 = plVar30;
              FUN_141266df0(&plStack_278,plStack_1f8,uStack_148,&plStack_1d8);
              uVar18 = uStack_1f0;
              if (!SBORROW8(0,(longlong)plStack_278)) {
                if (plStack_278 != (longlong *)0x0) {
                  func_0x000140613c20(plStack_270,(longlong)plStack_278 * 0xc,4);
                }
                if (plStack_260 != (longlong *)0x0) {
                  func_0x000140613c20(plStack_258,(longlong)plStack_260 << 3,4);
                }
                if (lStack_248 != 0) {
                  func_0x000140613c20(uStack_240,lStack_248 << 2,4);
                }
                if (lStack_230 != 0) {
                  func_0x000140613c20(uStack_228,lStack_230 << 2,4);
                }
              }
              plStack_268 = plStack_150;
              plStack_278 = plStack_160;
              plStack_270 = plStack_158;
              uStack_f1 = 0;
              FUN_141269ce0(&plStack_1d8,&plStack_278,1);
              plVar21 = plStack_1d8;
              if (!SBORROW8(0,(longlong)plStack_1d8)) {
                plStack_258 = plStack_1b0;
                lStack_250 = auStack_1a8._0_8_;
                plStack_268 = plStack_1c0;
                plStack_260 = plStack_1b8;
                plStack_278 = plStack_1d0;
                plStack_270 = plStack_1c8;
              }
              lVar16 = plStack_100[0x13];
              if (!SBORROW8(0,lVar16)) {
                if (lVar16 != 0) {
                  func_0x000140613c20(plStack_100[0x14],lVar16 * 0xc,4);
                }
                if (plStack_100[0x16] != 0) {
                  func_0x000140613c20(plStack_100[0x17],plStack_100[0x16] << 2,4);
                }
              }
              plStack_100[0x13] = (longlong)plVar21;
              plStack_100[0x14] = (longlong)plStack_278;
              plStack_100[0x15] = (longlong)plStack_270;
              plStack_100[0x16] = (longlong)plStack_268;
              plStack_100[0x17] = (longlong)plStack_260;
              plStack_100[0x18] = (longlong)plStack_258;
              plStack_100[0x19] = lStack_250;
              *(undefined1 *)(plStack_100 + 0x29) = 1;
              plVar25 = plStack_208;
              goto code_r0x0001407cf9a9;
            }
          }
          else {
            uVar18 = 0;
          }
code_r0x0001407d0d1a:
          FUN_1428d8fe3(uVar18,param_4,&UNK_142c22010);
          goto code_r0x0001407d0daa;
        }
      }
      else {
        uVar18 = 0;
      }
code_r0x0001407d0d4f:
      FUN_1428d8fe3(uVar18,uVar23,&UNK_142c22010);
      goto code_r0x0001407d0daa;
    }
  }
  else {
    uVar18 = 0;
  }
code_r0x0001407d0d31:
  uStack_f1 = 1;
  FUN_1428d8fe3(uVar18,uVar23,&UNK_142c22010);
  goto code_r0x0001407d0daa;
code_r0x0001407d0ac5:
  plVar25 = (longlong *)0x78;
  FUN_14004c440(&lStack_388,&lStack_368,0x78,0x10);
  if (cStack_f6 == '\0') {
    return;
  }
  plVar20 = (longlong *)func_0x000140c18130();
  if (*plVar20 != 1) {
    if ((*plVar20 == 0) &&
       (plVar20 = (longlong *)FUN_1428f2580(plVar20,0), plVar20 != (longlong *)0x0))
    goto code_r0x0001407d0aff;
code_r0x0001407d0bc2:
    puVar22 = &UNK_142c213c8;
    FUN_1428d80d0();
code_r0x0001407d0bce:
    FUN_1428a9ca0(plVar25);
    goto code_r0x0001407cf8b4;
  }
  plVar20 = plVar20 + 1;
code_r0x0001407d0aff:
  if (*plVar20 == 0) {
    *plVar20 = -1;
    plStack_100 = plVar20;
    FUN_140c170f0(plVar20 + 1,uStack_330);
    *plStack_100 = *plStack_100 + 1;
    return;
  }
  goto code_r0x0001407d0bf4;
}

