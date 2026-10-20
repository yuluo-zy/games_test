
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void FUN_140b2cfa0(longlong *param_1,longlong *param_2,undefined8 *param_3,longlong param_4,
                  undefined8 *param_5,undefined8 *param_6)

{
  char *pcVar1;
  byte bVar2;
  byte bVar3;
  undefined8 uVar4;
  longlong lVar5;
  code *pcVar6;
  uint uVar7;
  uint uVar8;
  int iVar9;
  longlong lVar10;
  char cVar11;
  uint uVar12;
  longlong lVar13;
  undefined *puVar14;
  byte *pbVar15;
  longlong *plVar16;
  longlong *plVar17;
  ulonglong uVar18;
  undefined *puVar19;
  undefined8 *puVar20;
  ushort uVar21;
  undefined *puVar22;
  ulonglong uVar23;
  longlong *plVar24;
  longlong lVar25;
  undefined1 uVar26;
  bool bVar27;
  byte bVar28;
  char cVar30;
  char cVar31;
  char cVar32;
  undefined1 auVar29 [16];
  undefined1 auVar33 [16];
  undefined1 auVar34 [16];
  char unaff_XMM6_Ba;
  char unaff_XMM6_Bb;
  char unaff_XMM6_Bc;
  char unaff_XMM6_Bd;
  char unaff_XMM6_Be;
  char unaff_XMM6_Bf;
  char unaff_XMM6_Bg;
  char unaff_XMM6_Bh;
  char unaff_XMM6_Bi;
  char unaff_XMM6_Bj;
  char unaff_XMM6_Bk;
  char unaff_XMM6_Bl;
  char unaff_XMM6_Bm;
  char unaff_XMM6_Bn;
  char unaff_XMM6_Bo;
  char unaff_XMM6_Bp;
  longlong *aplStack_2f8 [4];
  longlong lStack_2d8;
  longlong *plStack_2d0;
  undefined4 uStack_2c8;
  undefined4 uStack_2c4;
  undefined4 uStack_2c0;
  undefined4 uStack_2bc;
  undefined8 uStack_2b8;
  undefined8 uStack_2b0;
  undefined8 uStack_2a8;
  undefined8 uStack_2a0;
  undefined8 uStack_298;
  undefined8 uStack_290;
  ulonglong uStack_198;
  longlong *plStack_188;
  longlong *plStack_180;
  longlong *plStack_178;
  undefined8 *puStack_160;
  longlong lStack_158;
  undefined8 uStack_150;
  longlong *plStack_148;
  longlong *plStack_140;
  longlong *plStack_138;
  longlong *plStack_130;
  longlong lStack_128;
  longlong *plStack_120;
  undefined8 uStack_118;
  undefined8 uStack_110;
  undefined8 uStack_108;
  undefined1 auStack_100 [16];
  undefined1 auStack_f0 [16];
  undefined1 uStack_e0;
  undefined7 uStack_df;
  longlong *plStack_d8;
  undefined8 uStack_d0;
  longlong *plStack_c8;
  longlong *plStack_c0;
  longlong *plStack_b8;
  longlong *plStack_b0;
  longlong lStack_a8;
  longlong *plStack_a0;
  undefined8 *puStack_90;
  longlong *plStack_88;
  longlong *plStack_80;
  longlong *plStack_78;
  longlong *plStack_70;
  longlong *plStack_68;
  undefined8 uStack_60;
  
  uStack_60 = 0xfffffffffffffffe;
  puStack_90 = param_3;
  plStack_78 = param_2;
  plStack_70 = (longlong *)param_4;
  cVar11 = func_0x000140c16a00();
  plStack_80 = param_1;
  if (cVar11 == '\0') {
LAB_140b2d056:
    plVar17 = (longlong *)0x0;
  }
  else {
    param_2 = (longlong *)&UNK_142aeb288;
    lVar25 = 0x49;
    uVar18 = 0x49;
    lVar13 = FUN_140b2ae70(&UNK_142aeb288);
    if (lVar13 == 1) {
      if (uVar18 != 0) {
        if (uVar18 < 0x49) {
          if ((char)(&UNK_142aeb288)[uVar18] < -0x40) {
LAB_140b2d036:
            param_2 = (longlong *)0x49;
            FUN_1428d9940(&UNK_142aeb288,0x49,0,uVar18,&UNK_142aeb3e8);
            goto LAB_140b2d056;
          }
        }
        else if (uVar18 != 0x49) goto LAB_140b2d036;
      }
      param_2 = (longlong *)&UNK_142aeb288;
      lVar13 = FUN_140b2ae70(&UNK_142aeb288);
      if (lVar13 == 1) {
        uVar23 = uVar18 + 2;
        if (uVar23 != 0) {
          uVar26 = uVar23 == 0x49;
          if (0x48 < uVar23) goto LAB_140b2d0b8;
          uVar26 = (&UNK_142aeb28a)[uVar18] == -0x41;
          if ((char)(&UNK_142aeb28a)[uVar18] < -0x40) {
            do {
              uVar18 = 0x49;
              FUN_1428d9940(&UNK_142aeb288,0x49);
LAB_140b2d0b8:
            } while (!(bool)uVar26);
          }
        }
        lVar25 = 0x47 - uVar18;
        param_2 = (longlong *)(&UNK_142aeb288 + uVar23);
      }
    }
    puVar19 = &UNK_142aeb859;
    puVar22 = &UNK_142aeb818;
    do {
      bVar28 = puVar19[-1];
      uVar12 = (uint)(char)bVar28;
      if ((char)bVar28 < '\0') {
        bVar2 = puVar19[-2];
        if ((char)bVar2 < -0x40) {
          bVar3 = puVar19[-3];
          if ((char)bVar3 < -0x40) {
            pbVar15 = puVar19 + -4;
            puVar19 = puVar19 + -4;
            uVar12 = bVar3 & 0x3f | (*pbVar15 & 7) << 6;
          }
          else {
            puVar19 = puVar19 + -3;
            uVar12 = bVar3 & 0xf;
          }
          uVar12 = bVar2 & 0x3f | uVar12 << 6;
        }
        else {
          puVar19 = puVar19 + -2;
          uVar12 = bVar2 & 0x1f;
        }
        uVar12 = (uint)(bVar28 & 0x3f) | uVar12 << 6;
        if (uVar12 != 0x5c) goto LAB_140b2d159;
LAB_140b2d16e:
        puVar14 = puVar19 + -0x142aeb818;
        puVar22 = puVar19 + -0x142aeb817;
        if (puVar22 != (undefined *)0x0) {
          uVar26 = puVar22 == (undefined *)0x41;
          if ((undefined *)0x40 < puVar22) goto LAB_140b2d1b0;
          uVar26 = puVar19[1] == -0x41;
          if ((char)puVar19[1] < -0x40) {
            do {
              puVar14 = (undefined *)FUN_1428d9940(&UNK_142aeb818,0x41);
LAB_140b2d1b0:
            } while (!(bool)uVar26);
          }
        }
        lVar13 = 0x40 - (longlong)puVar14;
        puVar22 = &UNK_142aeb818 + (longlong)puVar22;
        goto LAB_140b2d1bd;
      }
      puVar19 = puVar19 + -1;
      if (uVar12 == 0x5c) goto LAB_140b2d16e;
LAB_140b2d159:
      if (uVar12 == 0x2f) goto LAB_140b2d16e;
    } while (puVar19 != &UNK_142aeb818);
    lVar13 = 0x41;
LAB_140b2d1bd:
    plVar17 = (longlong *)func_0x000140c18130();
    if (*plVar17 == 1) {
      plVar17 = plVar17 + 1;
    }
    else {
      if (*plVar17 != 0) goto LAB_140b2d922;
      plVar17 = (longlong *)FUN_1428f87d0(plVar17,0);
      if (plVar17 == (longlong *)0x0) goto LAB_140b2d922;
    }
    if (*plVar17 != 0) goto LAB_140b2d92e;
    *plVar17 = -1;
    plStack_68 = plVar17;
    uStack_d0 = FUN_140c16d40(plVar17 + 1,param_2,lVar25,puVar22,lVar13,1,0);
    *plStack_68 = *plStack_68 + 1;
    plVar17 = (longlong *)0x1;
  }
  plVar16 = (longlong *)*param_5;
  plVar24 = (longlong *)param_5[1];
  lVar13 = plVar16[0x1f];
  lVar25 = plVar16[0x20];
  lVar10 = plVar16[0x21];
  plStack_d8 = plVar17;
  pbVar15 = (byte *)FUN_140d4d740();
  plStack_68 = plVar17;
  if ((*pbVar15 & 1) == 0) {
    plStack_88 = plVar17;
    lStack_a8 = FUN_140d64e00();
    pbVar15[0] = 1;
    pbVar15[1] = 0;
    pbVar15[2] = 0;
    pbVar15[3] = 0;
    pbVar15[4] = 0;
    pbVar15[5] = 0;
    pbVar15[6] = 0;
    pbVar15[7] = 0;
    *(longlong *)(pbVar15 + 8) = lStack_a8;
    *(longlong **)(pbVar15 + 0x10) = param_2;
  }
  else {
    lStack_a8 = *(longlong *)(pbVar15 + 8);
    param_2 = *(longlong **)(pbVar15 + 0x10);
  }
  plStack_120 = (longlong *)(lVar13 + lVar25 * 4);
  plStack_138 = plVar24 + 0x1f;
  plStack_140 = plVar24 + 0x34;
  *(longlong *)(pbVar15 + 8) = lStack_a8 + 1;
  plStack_b8 = _DAT_142aeb100;
  plStack_b0 = _UNK_142aeb108;
  plStack_c8 = _DAT_142aeb0f0;
  plStack_c0 = _UNK_142aeb0f8;
  auStack_f0 = (undefined1  [16])0x0;
  auStack_100 = (undefined1  [16])0x0;
  uStack_118 = 8;
  uStack_110 = 0;
  uStack_108 = 8;
  plStack_148 = plVar24;
  plStack_130 = plVar16;
  lStack_128 = lVar13;
  uStack_e0 = (char)lVar10;
  plStack_a0 = param_2;
  FUN_141795a10(aplStack_2f8,&plStack_148);
  if (plStack_b8 < aplStack_2f8[0]) goto LAB_140b2d93a;
  do {
    uStack_290 = CONCAT71(uStack_df,uStack_e0);
    uStack_298 = auStack_f0._8_8_;
    uStack_2a8 = auStack_100._8_8_;
    uStack_2a0 = auStack_f0._0_8_;
    uStack_2b8 = uStack_108;
    uStack_2b0 = auStack_100._0_8_;
    uStack_2c8 = (undefined4)uStack_118;
    uStack_2c4 = uStack_118._4_4_;
    uStack_2c0 = (undefined4)uStack_110;
    uStack_2bc = uStack_110._4_4_;
    lStack_2d8 = lStack_128;
    plStack_2d0 = plStack_120;
    aplStack_2f8[2] = plStack_138;
    aplStack_2f8[3] = plStack_130;
    aplStack_2f8[0] = plStack_148;
    aplStack_2f8[1] = plStack_140;
    FUN_141475530(aplStack_2f8,&plStack_c8);
    plStack_148 = plStack_c8;
    plStack_140 = plStack_c0;
    plStack_138 = plStack_b8;
    plStack_130 = plStack_b0;
    lStack_128 = lStack_a8;
    plStack_120 = plStack_a0;
    *(undefined4 *)plStack_80[2] = *(undefined4 *)((longlong)plStack_80 + 0x1c);
    plStack_68 = (longlong *)*plStack_80;
    if ((char)plStack_68[3] != '\0') {
      *(undefined4 *)param_6[2] = *(undefined4 *)((longlong)param_6 + 0x1c);
      uVar4 = *param_6;
      func_0x000142923c00(aplStack_2f8,uVar4,0x170);
      lVar13 = 0;
      func_0x000142923c20(uVar4,0,0x170);
      uVar18 = uStack_198;
      for (uVar23 = 0; uVar23 < uVar18; uVar23 = uVar23 + 1) {
        uVar12 = func_0x000140c111b0(uVar23);
        if (0x2b < uVar12) {
          FUN_1428d9518((ulonglong)uVar12,0x2c,&UNK_142c63d78);
LAB_140b2d974:
                    /* WARNING: Does not return */
          pcVar6 = (code *)invalidInstructionException();
          (*pcVar6)();
        }
        plVar17 = (longlong *)plStack_68[4];
        plVar16 = aplStack_2f8[uVar12] + lVar13 * 5;
        LOCK();
        bVar27 = plVar17[1] == 1;
        if (bVar27) {
          plVar17[1] = -1;
        }
        UNLOCK();
        if ((!bVar27) || (plVar17[1] = 1, *plVar17 != 1)) {
          FUN_1428d9310(&UNK_142aeb860);
          goto LAB_140b2d974;
        }
        lVar25 = plStack_68[4];
        FUN_140d7bd90(&plStack_188,plVar16);
        plVar17 = (longlong *)plVar16[3];
        lVar10 = plVar16[4];
        LOCK();
        lVar13 = *plVar17;
        *plVar17 = *plVar17 + 1;
        UNLOCK();
        if (*plVar17 == 0 || SCARRY8(lVar13,1) != *plVar17 < 0) goto LAB_140b2d974;
        plStack_b8 = plStack_178;
        plStack_c8 = plStack_188;
        plStack_c0 = plStack_180;
        lVar13 = *(longlong *)(lVar25 + 0x20);
        plStack_b0 = plVar17;
        lStack_a8 = lVar10;
        if (lVar13 == *(longlong *)(lVar25 + 0x10)) {
          FUN_140562ab0(lVar25 + 0x10,&UNK_142aeb878);
        }
        lVar10 = *(longlong *)(lVar25 + 0x18);
        *(longlong *)(lVar10 + 0x20 + lVar13 * 0x28) = lStack_a8;
        puVar20 = (undefined8 *)(lVar10 + 0x10 + lVar13 * 0x28);
        *puVar20 = plStack_b8;
        puVar20[1] = plStack_b0;
        puVar20 = (undefined8 *)(lVar10 + lVar13 * 0x28);
        *puVar20 = plStack_c8;
        puVar20[1] = plStack_c0;
        *(longlong *)(lVar25 + 0x20) = lVar13 + 1;
        lVar13 = lStack_a8;
      }
      FUN_14087d740(aplStack_2f8);
      puVar20 = *(undefined8 **)(plStack_68[4] + 0x18);
      puStack_160 = puVar20 + *(longlong *)(plStack_68[4] + 0x20) * 5;
      lStack_158 = *plStack_78;
      uStack_150 = *puStack_90;
      iVar9 = -(uint)(CONCAT13(unaff_XMM6_Bd,
                               CONCAT12(unaff_XMM6_Bc,CONCAT11(unaff_XMM6_Bb,unaff_XMM6_Ba))) ==
                     CONCAT13(unaff_XMM6_Bd,
                              CONCAT12(unaff_XMM6_Bc,CONCAT11(unaff_XMM6_Bb,unaff_XMM6_Ba))));
      unaff_XMM6_Ba = (char)iVar9;
      unaff_XMM6_Bb = (char)((uint)iVar9 >> 8);
      unaff_XMM6_Bc = (char)((uint)iVar9 >> 0x10);
      unaff_XMM6_Bd = (char)((uint)iVar9 >> 0x18);
      iVar9 = -(uint)(CONCAT13(unaff_XMM6_Bh,
                               CONCAT12(unaff_XMM6_Bg,CONCAT11(unaff_XMM6_Bf,unaff_XMM6_Be))) ==
                     CONCAT13(unaff_XMM6_Bh,
                              CONCAT12(unaff_XMM6_Bg,CONCAT11(unaff_XMM6_Bf,unaff_XMM6_Be))));
      unaff_XMM6_Be = (char)iVar9;
      unaff_XMM6_Bf = (char)((uint)iVar9 >> 8);
      unaff_XMM6_Bg = (char)((uint)iVar9 >> 0x10);
      unaff_XMM6_Bh = (char)((uint)iVar9 >> 0x18);
      iVar9 = -(uint)(CONCAT13(unaff_XMM6_Bl,
                               CONCAT12(unaff_XMM6_Bk,CONCAT11(unaff_XMM6_Bj,unaff_XMM6_Bi))) ==
                     CONCAT13(unaff_XMM6_Bl,
                              CONCAT12(unaff_XMM6_Bk,CONCAT11(unaff_XMM6_Bj,unaff_XMM6_Bi))));
      unaff_XMM6_Bi = (char)iVar9;
      unaff_XMM6_Bj = (char)((uint)iVar9 >> 8);
      unaff_XMM6_Bk = (char)((uint)iVar9 >> 0x10);
      unaff_XMM6_Bl = (char)((uint)iVar9 >> 0x18);
      iVar9 = -(uint)(CONCAT13(unaff_XMM6_Bp,
                               CONCAT12(unaff_XMM6_Bo,CONCAT11(unaff_XMM6_Bn,unaff_XMM6_Bm))) ==
                     CONCAT13(unaff_XMM6_Bp,
                              CONCAT12(unaff_XMM6_Bo,CONCAT11(unaff_XMM6_Bn,unaff_XMM6_Bm))));
      unaff_XMM6_Bm = (char)iVar9;
      unaff_XMM6_Bn = (char)((uint)iVar9 >> 8);
      unaff_XMM6_Bo = (char)((uint)iVar9 >> 0x10);
      unaff_XMM6_Bp = (char)((uint)iVar9 >> 0x18);
      for (; puVar20 != puStack_160; puVar20 = puVar20 + 5) {
        plVar17 = (longlong *)plStack_68[7];
        aplStack_2f8[1] = (longlong *)puVar20[1];
        aplStack_2f8[2] = (longlong *)puVar20[2];
        aplStack_2f8[0] = plVar17;
        plVar16 = (longlong *)FUN_14145a710(aplStack_2f8);
        plStack_c8 = plVar16;
        if (plStack_130 != (longlong *)0x0) {
          uVar18 = FUN_1405b0d70(&lStack_128,&plStack_c8);
          bVar28 = (byte)(uVar18 >> 0x39);
          auVar29 = ZEXT216(CONCAT11(bVar28,bVar28));
          auVar29 = pshuflw(auVar29,auVar29,0);
          lVar13 = 0;
          while( true ) {
            uVar18 = (ulonglong)plStack_140 & uVar18;
            pcVar1 = (char *)((longlong)plStack_148 + uVar18);
            cVar11 = auVar29[0];
            auVar34[0] = -(*pcVar1 == cVar11);
            cVar30 = auVar29[1];
            auVar34[1] = -(pcVar1[1] == cVar30);
            cVar31 = auVar29[2];
            auVar34[2] = -(pcVar1[2] == cVar31);
            cVar32 = auVar29[3];
            auVar34[3] = -(pcVar1[3] == cVar32);
            auVar34[4] = -(pcVar1[4] == cVar11);
            auVar34[5] = -(pcVar1[5] == cVar30);
            auVar34[6] = -(pcVar1[6] == cVar31);
            auVar34[7] = -(pcVar1[7] == cVar32);
            auVar34[8] = -(pcVar1[8] == cVar11);
            auVar34[9] = -(pcVar1[9] == cVar30);
            auVar34[10] = -(pcVar1[10] == cVar31);
            auVar34[0xb] = -(pcVar1[0xb] == cVar32);
            auVar34[0xc] = -(pcVar1[0xc] == cVar11);
            auVar34[0xd] = -(pcVar1[0xd] == cVar30);
            auVar34[0xe] = -(pcVar1[0xe] == cVar31);
            auVar34[0xf] = -(pcVar1[0xf] == cVar32);
            uVar21 = (ushort)(SUB161(auVar34 >> 7,0) & 1) |
                     (ushort)(SUB161(auVar34 >> 0xf,0) & 1) << 1 |
                     (ushort)(SUB161(auVar34 >> 0x17,0) & 1) << 2 |
                     (ushort)(SUB161(auVar34 >> 0x1f,0) & 1) << 3 |
                     (ushort)(SUB161(auVar34 >> 0x27,0) & 1) << 4 |
                     (ushort)(SUB161(auVar34 >> 0x2f,0) & 1) << 5 |
                     (ushort)(SUB161(auVar34 >> 0x37,0) & 1) << 6 |
                     (ushort)(SUB161(auVar34 >> 0x3f,0) & 1) << 7 |
                     (ushort)(SUB161(auVar34 >> 0x47,0) & 1) << 8 |
                     (ushort)(SUB161(auVar34 >> 0x4f,0) & 1) << 9 |
                     (ushort)(SUB161(auVar34 >> 0x57,0) & 1) << 10 |
                     (ushort)(SUB161(auVar34 >> 0x5f,0) & 1) << 0xb |
                     (ushort)(SUB161(auVar34 >> 0x67,0) & 1) << 0xc |
                     (ushort)(SUB161(auVar34 >> 0x6f,0) & 1) << 0xd |
                     (ushort)(SUB161(auVar34 >> 0x77,0) & 1) << 0xe |
                     (ushort)(auVar34[0xf] >> 7) << 0xf;
            uVar12 = (uint)uVar21;
            while (uVar21 != 0) {
              uVar7 = 0;
              for (uVar8 = uVar12; (uVar8 & 1) == 0; uVar8 = uVar8 >> 1 | 0x80000000) {
                uVar7 = uVar7 + 1;
              }
              if (plVar16 == (longlong *)plStack_148[-1 - (uVar7 + uVar18 & (ulonglong)plStack_140)]
                 ) goto LAB_140b2d84d;
              uVar21 = (ushort)(uVar12 - 1) & (ushort)uVar12;
              uVar12 = CONCAT22((short)(uVar12 - 1 >> 0x10),uVar21);
            }
            auVar33[0] = -(*pcVar1 == unaff_XMM6_Ba);
            auVar33[1] = -(pcVar1[1] == unaff_XMM6_Bb);
            auVar33[2] = -(pcVar1[2] == unaff_XMM6_Bc);
            auVar33[3] = -(pcVar1[3] == unaff_XMM6_Bd);
            auVar33[4] = -(pcVar1[4] == unaff_XMM6_Be);
            auVar33[5] = -(pcVar1[5] == unaff_XMM6_Bf);
            auVar33[6] = -(pcVar1[6] == unaff_XMM6_Bg);
            auVar33[7] = -(pcVar1[7] == unaff_XMM6_Bh);
            auVar33[8] = -(pcVar1[8] == unaff_XMM6_Bi);
            auVar33[9] = -(pcVar1[9] == unaff_XMM6_Bj);
            auVar33[10] = -(pcVar1[10] == unaff_XMM6_Bk);
            auVar33[0xb] = -(pcVar1[0xb] == unaff_XMM6_Bl);
            auVar33[0xc] = -(pcVar1[0xc] == unaff_XMM6_Bm);
            auVar33[0xd] = -(pcVar1[0xd] == unaff_XMM6_Bn);
            auVar33[0xe] = -(pcVar1[0xe] == unaff_XMM6_Bo);
            auVar33[0xf] = -(pcVar1[0xf] == unaff_XMM6_Bp);
            if ((((((((((((((((SUB161(auVar33 >> 7,0) & 1) != 0 ||
                             (SUB161(auVar33 >> 0xf,0) & 1) != 0) ||
                            (SUB161(auVar33 >> 0x17,0) & 1) != 0) ||
                           (SUB161(auVar33 >> 0x1f,0) & 1) != 0) ||
                          (SUB161(auVar33 >> 0x27,0) & 1) != 0) ||
                         (SUB161(auVar33 >> 0x2f,0) & 1) != 0) ||
                        (SUB161(auVar33 >> 0x37,0) & 1) != 0) ||
                       (SUB161(auVar33 >> 0x3f,0) & 1) != 0) || (SUB161(auVar33 >> 0x47,0) & 1) != 0
                      ) || (SUB161(auVar33 >> 0x4f,0) & 1) != 0) ||
                    (SUB161(auVar33 >> 0x57,0) & 1) != 0) || (SUB161(auVar33 >> 0x5f,0) & 1) != 0)
                  || (SUB161(auVar33 >> 0x67,0) & 1) != 0) || (SUB161(auVar33 >> 0x6f,0) & 1) != 0)
                || (SUB161(auVar33 >> 0x77,0) & 1) != 0) || auVar33[0xf] < '\0') break;
            uVar18 = uVar18 + lVar13 + 0x10;
            lVar13 = lVar13 + 0x10;
          }
        }
        (**(code **)(puVar20[4] + 0x28))
                  (aplStack_2f8,
                   (*(longlong *)(puVar20[4] + 0x10) - 1U & 0xfffffffffffffff0) + puVar20[3] + 0x10,
                   lStack_158,uStack_150,plStack_70);
        plStack_78 = aplStack_2f8[0];
        plStack_80 = aplStack_2f8[1];
        puStack_90 = puVar20;
        if (aplStack_2f8[2] != (longlong *)0x0) {
          lVar25 = (longlong)aplStack_2f8[2] << 3;
          lVar13 = 0;
          do {
            uVar18 = *(ulonglong *)((longlong)plStack_80 + lVar13);
            if (uVar18 >> 0x20 == 0) break;
            plVar24 = *(longlong **)((longlong)plStack_70 + 8);
            if (plVar24 == (longlong *)0x0) {
              plVar24 = *(longlong **)((longlong)plStack_70 + 0x10);
            }
            lVar10 = plVar24[2];
            if ((ulonglong)(*plVar24 - lVar10) < 0x30) {
              FUN_1428a9a60(plVar24,lVar10,0x30,1,1);
            }
            lVar5 = plVar24[1];
            *(code **)(lVar5 + lVar10) = FUN_1412d9a10;
            *(code **)(lVar5 + 8 + lVar10) = FUN_141564d30;
            *(ulonglong *)(lVar5 + 0x10 + lVar10) = uVar18;
            *(longlong **)(lVar5 + 0x18 + lVar10) = plVar16;
            *(longlong **)(lVar5 + 0x20 + lVar10) = plVar17;
            *(undefined1 *)(lVar5 + 0x28 + lVar10) = 0;
            plVar24[2] = lVar10 + 0x30;
            lVar13 = lVar13 + 8;
          } while (lVar25 != lVar13);
        }
        puVar20 = puStack_90;
        if (plStack_78 != (longlong *)0x0) {
          func_0x000140613c20(plStack_80,(longlong)plStack_78 << 3,8);
        }
LAB_140b2d84d:
      }
    }
    if (plStack_140 != (longlong *)0x0) {
      uVar18 = (longlong)plStack_140 * 8 + 0x17U & 0xfffffffffffffff0;
      puVar19 = (undefined *)((longlong)plStack_140 + uVar18 + 0x11);
      if (puVar19 != (undefined *)0x0) {
        func_0x000140613c20((longlong)plStack_148 - uVar18,puVar19,0x10);
      }
    }
    if (plStack_d8 == (longlong *)0x0) {
      return;
    }
    plVar17 = (longlong *)func_0x000140c18130();
    if (*plVar17 == 1) {
      plVar17 = plVar17 + 1;
LAB_140b2d8ab:
      if (*plVar17 == 0) {
        *plVar17 = -1;
        plStack_70 = plVar17;
        FUN_140c170f0(plVar17 + 1,uStack_d0);
        *plStack_70 = *plStack_70 + 1;
        return;
      }
    }
    else {
      if ((*plVar17 == 0) &&
         (plVar17 = (longlong *)FUN_1428f87d0(plVar17,0), plVar17 != (longlong *)0x0))
      goto LAB_140b2d8ab;
LAB_140b2d922:
      FUN_1428d80d0(&UNK_142c3aaa0);
    }
LAB_140b2d92e:
    FUN_1428d9250(&UNK_142c3ab70);
LAB_140b2d93a:
    FUN_1428aa7a0(&plStack_c8);
  } while( true );
}

