
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

bool FUN_140be7ba0(ulonglong *param_1,ulonglong param_2,undefined8 param_3,undefined8 param_4)

{
  undefined8 *puVar1;
  undefined8 *puVar2;
  undefined1 auVar3 [16];
  char cVar4;
  undefined1 uVar5;
  ulonglong uVar6;
  undefined8 *****pppppuVar7;
  code *pcVar8;
  ushort uVar9;
  uint uVar10;
  undefined8 uVar11;
  undefined8 uVar12;
  undefined8 *****pppppuVar13;
  undefined8 *****pppppuVar14;
  uint uVar15;
  uint uVar16;
  ulonglong uVar17;
  byte *pbVar18;
  undefined8 uVar19;
  longlong lVar20;
  undefined8 *****pppppuVar21;
  undefined2 *puVar22;
  undefined2 uVar23;
  ulonglong uVar24;
  undefined1 *puVar25;
  longlong lVar26;
  char cVar27;
  undefined1 (*pauVar28) [16];
  uint *puVar29;
  ulonglong uVar30;
  int iVar31;
  undefined1 uVar32;
  ulonglong unaff_R14;
  ulonglong *unaff_R15;
  undefined1 (*pauVar33) [16];
  undefined4 uVar34;
  undefined4 uVar35;
  undefined4 uVar36;
  undefined4 uVar37;
  int iVar38;
  undefined4 uVar39;
  undefined8 uStack_328;
  undefined8 uStack_320;
  undefined1 (*pauStack_2f8) [16];
  longlong lStack_2f0;
  undefined8 uStack_2e8;
  longlong lStack_2e0;
  longlong lStack_2d8;
  ulonglong uStack_2d0;
  ulonglong *puStack_2c0;
  undefined8 ****ppppuStack_2b8;
  undefined8 uStack_2b0;
  undefined8 ****ppppuStack_2a8;
  undefined8 uStack_2a0;
  undefined8 uStack_298;
  undefined8 uStack_290;
  undefined8 ****ppppuStack_288;
  undefined8 ****ppppuStack_280;
  undefined1 (*pauStack_278) [16];
  longlong lStack_270;
  undefined8 uStack_268;
  longlong lStack_260;
  longlong lStack_258;
  ulonglong uStack_250;
  undefined8 ****ppppuStack_248;
  undefined8 ****ppppuStack_240;
  uint *puStack_230;
  ulonglong uStack_228;
  undefined8 uStack_220;
  undefined8 ****ppppuStack_218;
  undefined8 ****ppppuStack_210;
  undefined8 uStack_208;
  undefined8 uStack_200;
  undefined8 uStack_1f8;
  int iStack_1f0;
  undefined4 uStack_1ec;
  undefined8 uStack_1e8;
  undefined8 uStack_1d8;
  undefined8 ****ppppuStack_1d0;
  undefined8 ****ppppuStack_1c8;
  undefined8 uStack_1c0;
  undefined8 uStack_1b8;
  int iStack_1b0;
  undefined2 uStack_1ac;
  char cStack_1aa;
  undefined1 uStack_1a9;
  undefined8 uStack_1a8;
  undefined8 uStack_1a0;
  undefined8 uStack_198;
  undefined8 uStack_190;
  undefined8 ****ppppuStack_188;
  undefined8 ****ppppuStack_180;
  undefined8 ****ppppuStack_178;
  undefined8 ****ppppuStack_170;
  undefined8 uStack_168;
  undefined8 uStack_160;
  undefined8 uStack_158;
  int iStack_140;
  int iStack_13c;
  undefined8 ****ppppuStack_138;
  undefined8 uStack_130;
  ulonglong *puStack_128;
  undefined8 uStack_120;
  undefined8 uStack_118;
  undefined4 uStack_110;
  undefined4 uStack_10c;
  undefined4 uStack_108;
  int iStack_104;
  int iStack_100;
  undefined4 uStack_fc;
  undefined8 uStack_f8;
  undefined8 ****ppppuStack_f0;
  undefined8 uStack_e8;
  undefined8 uStack_e0;
  undefined8 uStack_d8;
  int iStack_d0;
  undefined2 uStack_cc;
  char cStack_ca;
  undefined1 uStack_c9;
  undefined8 uStack_c8;
  undefined8 uStack_c0;
  undefined8 uStack_b8;
  undefined8 uStack_b0;
  longlong lStack_a0;
  ulonglong uStack_98;
  undefined8 ****ppppuStack_90;
  undefined2 uStack_84;
  undefined2 uStack_82;
  uint *puStack_80;
  undefined1 uStack_75;
  char cStack_74;
  undefined1 uStack_73;
  undefined1 uStack_72;
  char cStack_71;
  undefined8 uStack_70;
  
  uStack_70 = 0xfffffffffffffffe;
  uStack_220 = param_3;
  uStack_120 = param_4;
  if (param_1[8] == 0) {
    unaff_R15 = param_1;
    if (param_1[2] != 0) goto code_r0x000140be8952;
    if (param_1[5] != 0) goto code_r0x000140be896a;
    unaff_R14 = *(ulonglong *)(param_2 + 0x10);
    uStack_228 = param_2;
    if (unaff_R14 == 0) {
      uVar24 = 0;
      uVar17 = 0;
      goto code_r0x000140be7cee;
    }
    if (*param_1 < unaff_R14) goto code_r0x000140be8982;
    uVar17 = 0;
    puVar22 = (undefined2 *)param_1[1];
  }
  else {
    FUN_1428d9430(&UNK_142b13de0,0x33,&UNK_142b13e18);
code_r0x000140be8952:
    FUN_1428d9430(&UNK_142b13e30,0x2b,&UNK_142b13e60);
code_r0x000140be896a:
    FUN_1428d9430(&UNK_142b13e78,0x39,&UNK_142b13eb8);
code_r0x000140be8982:
    param_2 = 0;
    FUN_1428ca940(unaff_R15,0,unaff_R14,2,4);
    uVar17 = unaff_R15[2];
    puVar22 = (undefined2 *)(uVar17 * 4 + unaff_R15[1]);
    param_1 = unaff_R15;
  }
  if (unaff_R14 != 1) {
    uVar24 = (ulonglong)((uint)(unaff_R14 - 1) & 7);
    if (6 < unaff_R14 - 2) {
      uVar30 = unaff_R14 - 1 & 0xfffffffffffffff8;
      do {
        *puVar22 = 0xffff;
        *(undefined1 *)(puVar22 + 1) = 0;
        puVar22[2] = 0xffff;
        *(undefined1 *)(puVar22 + 3) = 0;
        puVar22[4] = 0xffff;
        *(undefined1 *)(puVar22 + 5) = 0;
        puVar22[6] = 0xffff;
        *(undefined1 *)(puVar22 + 7) = 0;
        puVar22[8] = 0xffff;
        *(undefined1 *)(puVar22 + 9) = 0;
        puVar22[10] = 0xffff;
        *(undefined1 *)(puVar22 + 0xb) = 0;
        puVar22[0xc] = 0xffff;
        *(undefined1 *)(puVar22 + 0xd) = 0;
        puVar22[0xe] = 0xffff;
        *(undefined1 *)(puVar22 + 0xf) = 0;
        puVar22 = puVar22 + 0x10;
        uVar30 = uVar30 - 8;
      } while (uVar30 != 0);
    }
    for (; uVar24 != 0; uVar24 = uVar24 - 1) {
      *puVar22 = 0xffff;
      *(undefined1 *)(puVar22 + 1) = 0;
      puVar22 = puVar22 + 2;
    }
    uVar17 = (uVar17 + unaff_R14) - 1;
    param_2 = 0;
  }
  *puVar22 = 0xffff;
  *(undefined1 *)(puVar22 + 1) = 0;
  uVar17 = uVar17 + 1;
  uVar24 = param_1[5];
code_r0x000140be7cee:
  param_1[2] = uVar17;
  uVar30 = unaff_R14 - uVar24;
  uVar17 = unaff_R14;
  if (uVar24 <= unaff_R14 && uVar30 != 0) {
    if (param_1[3] - uVar24 < uVar30) {
      FUN_1428ca940(param_1 + 3,uVar24,uVar30,1,1);
      uVar6 = param_1[4];
      puVar25 = (undefined1 *)(uVar6 + param_1[5]);
      uVar17 = param_1[5];
      param_2 = uVar24;
    }
    else {
      uVar6 = param_1[4];
      puVar25 = (undefined1 *)(uVar6 + uVar24);
      uVar17 = uVar24;
    }
    if (1 < uVar30) {
      param_2 = 0;
      func_0x000142923c20(puVar25,0,uVar30 - 1);
      uVar17 = (uVar17 + uVar30) - 1;
      puVar25 = (undefined1 *)(uVar6 + uVar17);
    }
    *puVar25 = 0;
    uVar17 = uVar17 + 1;
  }
  param_1[5] = uVar17;
  pbVar18 = (byte *)FUN_140d4d740();
  if ((*pbVar18 & 1) == 0) {
    lStack_258 = FUN_140d64e00();
    pbVar18[0] = 1;
    pbVar18[1] = 0;
    pbVar18[2] = 0;
    pbVar18[3] = 0;
    pbVar18[4] = 0;
    pbVar18[5] = 0;
    pbVar18[6] = 0;
    pbVar18[7] = 0;
    *(longlong *)(pbVar18 + 8) = lStack_258;
    *(ulonglong *)(pbVar18 + 0x10) = param_2;
  }
  else {
    lStack_258 = *(longlong *)(pbVar18 + 8);
    param_2 = *(ulonglong *)(pbVar18 + 0x10);
  }
  *(longlong *)(pbVar18 + 8) = lStack_258 + 1;
  lVar20 = _UNK_142b13c28;
  uVar19 = _UNK_142b13c20;
  lVar26 = _UNK_142b13c18;
  pauVar33 = _UNK_142b13c10;
  uStack_268 = _UNK_142b13c20;
  lStack_260 = _UNK_142b13c28;
  pauStack_278 = _UNK_142b13c10;
  lStack_270 = _UNK_142b13c18;
  uStack_250 = param_2;
  pbVar18 = (byte *)FUN_140d4d740();
  if ((*pbVar18 & 1) == 0) {
    lStack_2d8 = FUN_140d64e00();
    pbVar18[0] = 1;
    pbVar18[1] = 0;
    pbVar18[2] = 0;
    pbVar18[3] = 0;
    pbVar18[4] = 0;
    pbVar18[5] = 0;
    pbVar18[6] = 0;
    pbVar18[7] = 0;
    *(longlong *)(pbVar18 + 8) = lStack_2d8;
    *(ulonglong *)(pbVar18 + 0x10) = param_2;
  }
  else {
    lStack_2d8 = *(longlong *)(pbVar18 + 8);
    param_2 = *(ulonglong *)(pbVar18 + 0x10);
  }
  puStack_2c0 = param_1 + 6;
  *(longlong *)(pbVar18 + 8) = lStack_2d8 + 1;
  uStack_2e8 = uVar19;
  lStack_2e0 = lVar20;
  pauStack_2f8 = pauVar33;
  lStack_2f0 = lVar26;
  puStack_230 = (uint *)param_1[0x19] + param_1[0x1a];
  lVar26 = *(longlong *)(uStack_228 + 8);
  puStack_80 = (uint *)param_1[0x19];
  uStack_2d0 = param_2;
  puStack_128 = param_1;
  lStack_a0 = lVar26;
  uStack_98 = unaff_R14;
code_r0x000140be7ea0:
  do {
    puVar29 = puStack_80;
    pppppuVar14 = uStack_130;
    if (puVar29 == puStack_230) {
      *(undefined8 *)(uStack_228 + 0x10) = 0;
      puStack_80 = (uint *)0x0;
      while ((uint *)uStack_98 != puStack_80) {
        puStack_80 = (uint *)((longlong)puStack_80 + 1);
        func_0x000140bdfc90(lVar26);
        lVar26 = lVar26 + 0x48;
      }
      if (lStack_2f0 != 0) {
        uVar17 = (lStack_2f0 + 1) * 0x18 + 0xfU & 0xfffffffffffffff0;
        lVar26 = lStack_2f0 + uVar17 + 0x11;
        if (lVar26 != 0) {
          func_0x000140613c20((longlong)pauStack_2f8 - uVar17,lVar26,0x10);
        }
      }
      if (lStack_270 != 0) {
        uVar17 = (lStack_270 + 1) * 0x24 + 0xfU & 0xfffffffffffffff0;
        lVar26 = lStack_270 + uVar17 + 0x11;
        if (lVar26 != 0) {
          func_0x000140613c20((longlong)pauStack_278 - uVar17,lVar26,0x10);
        }
      }
code_r0x000140be88ce:
      return puVar29 != puStack_230;
    }
    puStack_80 = puVar29 + 1;
    uVar17 = (ulonglong)(*puVar29 & 0xffff);
  } while ((*puVar29 & 0x10000) != 0);
  if (unaff_R14 <= uVar17) {
    FUN_1428d9518(uVar17,unaff_R14,&UNK_142b13ed0);
    goto code_r0x000140be8cb6;
  }
  lVar20 = *(longlong *)(lVar26 + uVar17 * 0x48);
  pppppuVar7 = *(undefined8 ******)(lVar26 + 8 + uVar17 * 0x48);
  pppppuVar21 = *(undefined8 ******)(lVar26 + 0x10 + uVar17 * 0x48);
  puVar1 = (undefined8 *)(lVar26 + 0x18 + uVar17 * 0x48);
  puVar2 = (undefined8 *)(lVar26 + 0x18 + uVar17 * 0x48);
  ppppuStack_288 = (undefined8 ****)*puVar2;
  ppppuStack_280 = (undefined8 ****)puVar2[1];
  uVar19 = *(undefined8 *)(lVar26 + 0x28 + uVar17 * 0x48);
  iVar38 = *(int *)(lVar26 + 0x30 + uVar17 * 0x48);
  uVar23 = *(undefined2 *)(lVar26 + 0x34 + uVar17 * 0x48);
  cVar4 = *(char *)(lVar26 + 0x36 + uVar17 * 0x48);
  uVar5 = *(undefined1 *)(lVar26 + 0x37 + uVar17 * 0x48);
  puVar2 = (undefined8 *)(lVar26 + 0x38 + uVar17 * 0x48);
  uVar11 = *puVar2;
  uVar12 = puVar2[1];
  *(undefined8 *)(lVar26 + uVar17 * 0x48) = 3;
  ppppuStack_90 = pppppuVar21;
  if (lVar20 == 0) {
    pppppuVar21 = (undefined8 *****)*puVar1;
    pppppuVar13 = (undefined8 *****)puVar1[1];
    iVar31 = (byte)(cVar4 - 7U) + 1;
    if (1 < (byte)(cVar4 - 7U)) {
      iVar31 = 0;
    }
    uStack_130._4_4_ = (undefined4)((ulonglong)pppppuVar13 >> 0x20);
    uStack_130 = pppppuVar14;
    if (iVar31 == 0) {
      uStack_110 = SUB84(pppppuVar13,0);
      uStack_108 = (undefined4)uVar19;
      iStack_104 = (int)((ulonglong)uVar19 >> 0x20);
      uStack_fc = CONCAT13(uVar5,CONCAT12(cVar4,uVar23));
      uStack_10c = uStack_130._4_4_;
      iStack_100 = iVar38;
      if (param_1[0x14] <= uVar17) {
        uStack_118 = pppppuVar21;
        FUN_1428d9518(uVar17,param_1[0x14],&UNK_142b13ee8);
        goto code_r0x000140be8cb6;
      }
      iVar38 = *(int *)(param_1[0x13] + uVar17 * 4);
      uStack_118 = (undefined8 *****)CONCAT44((int)((ulonglong)pppppuVar21 >> 0x20),iVar38);
      if (iVar38 == 0) {
        uStack_1d8 = (undefined8 *****)0x5;
        pppppuVar21 = (undefined8 *****)ppppuStack_2a8;
        uVar19 = uStack_2a0;
        iVar38 = iStack_13c;
        uVar23 = uStack_82;
        cStack_1aa = cStack_74;
        uStack_1a9 = uStack_73;
      }
      else {
        uVar19 = FUN_1418bcc30(&lStack_258,&uStack_118);
        FUN_1417e7f20(&uStack_f8,&pauStack_278,uVar19,&uStack_118);
        if (uStack_d8._2_1_ != '\x02') {
          if (puStack_128[2] <= uVar17) {
            FUN_1428d9518(uVar17,puStack_128[2],&UNK_142b13f00);
            goto code_r0x000140be8cb6;
          }
          uVar24 = puStack_128[1];
          *(undefined2 *)(uVar24 + uVar17 * 4) = (undefined2)uStack_d8;
          *(undefined1 *)(uVar24 + 2 + uVar17 * 4) = 1;
          if (puStack_128[5] <= uVar17) {
            FUN_1428d9518(uVar17,puStack_128[5],&UNK_142b13f18);
            goto code_r0x000140be8cb6;
          }
code_r0x000140be7e90:
          *(undefined1 *)(puStack_128[4] + uVar17) = 0;
          lVar26 = lStack_a0;
          unaff_R14 = uStack_98;
          param_1 = puStack_128;
          goto code_r0x000140be7ea0;
        }
        auVar3 = *pauStack_278;
        uVar15 = ~(uint)(ushort)((ushort)(SUB161(auVar3 >> 7,0) & 1) |
                                 (ushort)(SUB161(auVar3 >> 0xf,0) & 1) << 1 |
                                 (ushort)(SUB161(auVar3 >> 0x17,0) & 1) << 2 |
                                 (ushort)(SUB161(auVar3 >> 0x1f,0) & 1) << 3 |
                                 (ushort)(SUB161(auVar3 >> 0x27,0) & 1) << 4 |
                                 (ushort)(SUB161(auVar3 >> 0x2f,0) & 1) << 5 |
                                 (ushort)(SUB161(auVar3 >> 0x37,0) & 1) << 6 |
                                 (ushort)(SUB161(auVar3 >> 0x3f,0) & 1) << 7 |
                                 (ushort)(SUB161(auVar3 >> 0x47,0) & 1) << 8 |
                                 (ushort)(SUB161(auVar3 >> 0x4f,0) & 1) << 9 |
                                 (ushort)(SUB161(auVar3 >> 0x57,0) & 1) << 10 |
                                 (ushort)(SUB161(auVar3 >> 0x5f,0) & 1) << 0xb |
                                 (ushort)(SUB161(auVar3 >> 0x67,0) & 1) << 0xc |
                                 (ushort)(SUB161(auVar3 >> 0x6f,0) & 1) << 0xd |
                                 (ushort)(SUB161(auVar3 >> 0x77,0) & 1) << 0xe |
                                (ushort)(byte)(auVar3[0xf] >> 7) << 0xf);
        pauVar33 = pauStack_278 + 1;
        lVar26 = lStack_260;
        pauVar28 = pauStack_278;
        do {
          do {
            do {
              do {
                if (lVar26 == 0) goto code_r0x000140be839f;
                if ((short)uVar15 == 0) {
                  do {
                    auVar3 = *pauVar33;
                    uVar9 = (ushort)(SUB161(auVar3 >> 7,0) & 1) |
                            (ushort)(SUB161(auVar3 >> 0xf,0) & 1) << 1 |
                            (ushort)(SUB161(auVar3 >> 0x17,0) & 1) << 2 |
                            (ushort)(SUB161(auVar3 >> 0x1f,0) & 1) << 3 |
                            (ushort)(SUB161(auVar3 >> 0x27,0) & 1) << 4 |
                            (ushort)(SUB161(auVar3 >> 0x2f,0) & 1) << 5 |
                            (ushort)(SUB161(auVar3 >> 0x37,0) & 1) << 6 |
                            (ushort)(SUB161(auVar3 >> 0x3f,0) & 1) << 7 |
                            (ushort)(SUB161(auVar3 >> 0x47,0) & 1) << 8 |
                            (ushort)(SUB161(auVar3 >> 0x4f,0) & 1) << 9 |
                            (ushort)(SUB161(auVar3 >> 0x57,0) & 1) << 10 |
                            (ushort)(SUB161(auVar3 >> 0x5f,0) & 1) << 0xb |
                            (ushort)(SUB161(auVar3 >> 0x67,0) & 1) << 0xc |
                            (ushort)(SUB161(auVar3 >> 0x6f,0) & 1) << 0xd |
                            (ushort)(SUB161(auVar3 >> 0x77,0) & 1) << 0xe |
                            (ushort)(byte)(auVar3[0xf] >> 7) << 0xf;
                    pauVar28 = pauVar28 + -0x24;
                    pauVar33 = pauVar33 + 1;
                  } while (uVar9 == 0xffff);
                  uVar16 = ~(uint)uVar9;
                  uVar15 = uVar16 & -(uint)uVar9 - 2;
                }
                else {
                  uVar16 = uVar15;
                  uVar15 = uVar15 - 1 & uVar15;
                }
                lVar26 = lVar26 + -1;
                uVar10 = 0;
                for (; (uVar16 & 1) == 0; uVar16 = uVar16 >> 1 | 0x80000000) {
                  uVar10 = uVar10 + 1;
                }
                uVar24 = (ulonglong)uVar10;
              } while ((((*(longlong *)((longlong)pauVar28 + uVar24 * -0x24 + -0x18) !=
                          CONCAT44(uStack_108,uStack_10c) ||
                          iStack_104 != *(int *)((longlong)pauVar28 + uVar24 * -0x24 + -0x10)) ||
                        (*(char *)((longlong)pauVar28 + uVar24 * -0x24 + -8) != (char)uStack_fc)) ||
                       (*(int *)((longlong)pauVar28 + uVar24 * -0x24 + -0xc) != iStack_100)) ||
                      (*(char *)((longlong)pauVar28 + uVar24 * -0x24 + -7) != uStack_fc._1_1_));
              FUN_141739060(&uStack_1d8,*(undefined4 *)((longlong)pauVar28 + uVar24 * -0x24 + -0x1c)
                           );
              FUN_141739060(&uStack_f8,uStack_110);
            } while ((int)uStack_1d8 != (int)uStack_f8);
          } while (uStack_1d8._4_4_ != uStack_f8._4_4_);
        } while ((short)ppppuStack_1d0 != (short)ppppuStack_f0);
code_r0x000140be839f:
        FUN_140be6b90(&uStack_f8,uStack_220,&uStack_118);
        if (uStack_c8._6_1_ == '\a') {
          ppppuStack_f0 = (undefined8 ****)CONCAT44(uStack_10c,uStack_110);
          uStack_e8 = (undefined8 *****)CONCAT44(iStack_104,uStack_108);
          uStack_e0 = (undefined8 *****)CONCAT44(uStack_fc,iStack_100);
          uStack_f8 = uStack_118;
          ppppuStack_178 = (undefined8 *****)0x0;
          FUN_140c07950(&uStack_1d8,uStack_120,pppppuVar7,ppppuStack_90,&uStack_f8,&ppppuStack_178);
          if (uStack_1d8 != (undefined8 *****)0xb) {
            if (8 < (uint)uStack_1d8) {
              uStack_b8 = uStack_198;
              uStack_b0 = uStack_190;
              uStack_c8 = uStack_1a8;
              uStack_c0 = uStack_1a0;
              uStack_d8 = uStack_1b8;
              iStack_d0 = iStack_1b0;
              uStack_cc = uStack_1ac;
              cStack_ca = cStack_1aa;
              uStack_c9 = uStack_1a9;
              uStack_e8 = (undefined8 *****)ppppuStack_1c8;
              uStack_e0 = uStack_1c0;
              uStack_f8 = uStack_1d8;
              ppppuStack_f0 = ppppuStack_1d0;
              ppppuStack_248 = (undefined8 ****)&uStack_f8;
              ppppuStack_240 = (undefined8 ****)FUN_140be9270;
              ppppuStack_178 = (undefined8 ****)&UNK_142b13f58;
              ppppuStack_170 = (undefined8 *****)0x1;
              uStack_158 = 0;
              uStack_168 = &ppppuStack_248;
              uStack_160 = 1;
              FUN_1428d9390(&ppppuStack_178,&UNK_142b13f68);
              goto code_r0x000140be8cb6;
            }
            FUN_1407cea70(&uStack_1d8);
code_r0x000140be8926:
            func_0x00014014a360(&pauStack_2f8);
            func_0x000140bdebf0(&pauStack_278);
            goto code_r0x000140be88ce;
          }
          uStack_1e8 = uStack_1a0;
          uVar34 = (undefined4)uStack_1c0;
          uVar35 = uStack_1c0._4_4_;
          uVar36 = (undefined4)uStack_1b8;
          uVar37 = uStack_1b8._4_4_;
          iVar38 = iStack_1b0;
          uVar39 = CONCAT13(uStack_1a9,CONCAT12(cStack_1aa,uStack_1ac));
          iStack_1f0 = (int)uStack_1a8;
          uStack_1ec = uStack_1a8._4_4_;
          ppppuStack_218 = ppppuStack_1d0;
          ppppuStack_210 = ppppuStack_1c8;
        }
        else {
          uStack_1e8 = uStack_c8;
          uStack_1ec = CONCAT13(uStack_c9,CONCAT12(cStack_ca,uStack_cc));
          uVar34 = (undefined4)uStack_e8;
          uVar35 = uStack_e8._4_4_;
          uVar36 = (undefined4)uStack_e0;
          uVar37 = uStack_e0._4_4_;
          iVar38 = (int)uStack_d8;
          uVar39 = uStack_d8._4_4_;
          iStack_1f0 = iStack_d0;
          ppppuStack_218 = uStack_f8;
          ppppuStack_210 = ppppuStack_f0;
        }
        uStack_1f8 = CONCAT44(uVar39,iVar38);
        uStack_208 = (undefined8 *****)CONCAT44(uVar35,uVar34);
        uStack_200 = CONCAT44(uVar37,uVar36);
        FUN_140bfe650(uStack_120,&ppppuStack_218,pppppuVar7,ppppuStack_90);
        ppppuStack_138 = ppppuStack_210;
        uStack_130 = uStack_208;
        uStack_328 = CONCAT44(uStack_1ec,iStack_1f0);
        uStack_320 = uStack_1e8;
        uStack_1d8 = (undefined8 *****)0x0;
        pppppuVar21 = (undefined8 *****)ppppuStack_218;
        uVar19 = uStack_200;
        lVar26 = lStack_a0;
        unaff_R14 = uStack_98;
        param_1 = puStack_128;
        iVar38 = (int)uStack_1f8;
        uVar23 = uStack_1f8._4_2_;
        cStack_1aa = uStack_1f8._6_1_;
        uStack_1a9 = uStack_1f8._7_1_;
        pppppuVar14 = uStack_130;
      }
      uStack_130 = pppppuVar14;
      ppppuStack_188 = ppppuStack_138;
      ppppuStack_180 = uStack_130;
      uStack_298 = uStack_328;
      uStack_290 = uStack_320;
      cVar27 = cStack_71;
      ppppuStack_2a8 = pppppuVar21;
      uStack_2a0 = uVar19;
      iStack_13c = iVar38;
      uStack_82 = uVar23;
      cStack_74 = cStack_1aa;
      uStack_73 = uStack_1a9;
    }
    else {
      if (iVar31 != 1) {
        FUN_1428d9430(&UNK_142b14018,0xf,&UNK_142b14028);
        goto code_r0x000140be8cb6;
      }
      ppppuStack_138 = pppppuVar21;
      if (param_1[0x17] <= uVar17) {
        uStack_130 = pppppuVar13;
        FUN_1428d9518(uVar17,param_1[0x17],&UNK_142b13f80);
        goto code_r0x000140be8cb6;
      }
      iVar38 = *(int *)(param_1[0x16] + uVar17 * 4);
      uStack_130 = (undefined8 *****)CONCAT44(uStack_130._4_4_,iVar38);
      if (iVar38 == 0) {
        uStack_1d8 = (undefined8 *****)0x5;
        pppppuVar21 = (undefined8 *****)ppppuStack_2b8;
        uVar19 = uStack_2b0;
        iVar38 = iStack_140;
        cStack_1aa = cStack_71;
        uStack_1a9 = uStack_75;
        uVar23 = uStack_84;
      }
      else {
        uVar19 = FUN_1418bce70(&lStack_2d8,&ppppuStack_138);
        FUN_1417e7c30(&uStack_f8,&pauStack_2f8,uVar19,&ppppuStack_138);
        if (uStack_e8._2_1_ != '\x02') {
          if (puStack_128[2] <= uVar17) {
            FUN_1428d9518(uVar17,puStack_128[2],&UNK_142b13f98);
            goto code_r0x000140be8cb6;
          }
          uVar24 = puStack_128[1];
          *(undefined2 *)(uVar24 + uVar17 * 4) = (undefined2)uStack_e8;
          *(undefined1 *)(uVar24 + 2 + uVar17 * 4) = 1;
          if (puStack_128[5] <= uVar17) {
            FUN_1428d9518(uVar17,puStack_128[5],&UNK_142b13fb0);
            goto code_r0x000140be8cb6;
          }
          goto code_r0x000140be7e90;
        }
        FUN_140be71e0(&uStack_f8,uStack_220,&ppppuStack_138);
        if (uStack_d8._6_1_ == '\x05') {
          uStack_f8 = (undefined8 *****)ppppuStack_138;
          ppppuStack_f0 = uStack_130;
          FUN_140c03460(&uStack_1d8,uStack_120,pppppuVar7,ppppuStack_90,&uStack_f8,0);
          if (uStack_1d8 != (undefined8 *****)0xb) {
            if (8 < (uint)uStack_1d8) {
              uStack_b8 = uStack_198;
              uStack_b0 = uStack_190;
              uStack_c8 = uStack_1a8;
              uStack_c0 = uStack_1a0;
              uStack_d8 = uStack_1b8;
              iStack_d0 = iStack_1b0;
              uStack_cc = uStack_1ac;
              cStack_ca = cStack_1aa;
              uStack_c9 = uStack_1a9;
              uStack_e8 = (undefined8 *****)ppppuStack_1c8;
              uStack_e0 = uStack_1c0;
              uStack_f8 = uStack_1d8;
              ppppuStack_f0 = ppppuStack_1d0;
              uStack_118 = (undefined8 *****)&uStack_f8;
              uStack_110 = 0x40be9270;
              uStack_10c = 1;
              ppppuStack_218 = (undefined8 ****)&UNK_142b13ff0;
              ppppuStack_210 = (undefined8 *****)0x1;
              uStack_1f8 = 0;
              uStack_208 = (undefined8 *****)&uStack_118;
              uStack_200 = 1;
              FUN_1428d9390(&ppppuStack_218,&UNK_142b14000);
              goto code_r0x000140be8cb6;
            }
            FUN_1407cea70(&uStack_1d8);
            goto code_r0x000140be8926;
          }
          uStack_158 = CONCAT17(uStack_1a9,CONCAT16(cStack_1aa,CONCAT24(uStack_1ac,iStack_1b0)));
          uVar34 = (undefined4)uStack_1c0;
          uVar35 = uStack_1c0._4_4_;
          uVar36 = (undefined4)uStack_1b8;
          uVar37 = uStack_1b8._4_4_;
          ppppuStack_178 = ppppuStack_1d0;
          ppppuStack_170 = ppppuStack_1c8;
        }
        else {
          uStack_158 = uStack_d8;
          uVar34 = (undefined4)uStack_e8;
          uVar35 = uStack_e8._4_4_;
          uVar36 = (undefined4)uStack_e0;
          uVar37 = uStack_e0._4_4_;
          ppppuStack_178 = uStack_f8;
          ppppuStack_170 = ppppuStack_f0;
        }
        uStack_168 = (undefined8 *****)CONCAT44(uVar35,uVar34);
        uStack_160 = CONCAT44(uVar37,uVar36);
        FUN_140beec30(uStack_120,&ppppuStack_178,pppppuVar7,ppppuStack_90);
        ppppuStack_248 = ppppuStack_170;
        ppppuStack_240 = uStack_168;
        uStack_1d8 = (undefined8 *****)0x2;
        pppppuVar21 = (undefined8 *****)ppppuStack_178;
        uVar19 = uStack_160;
        lVar26 = lStack_a0;
        unaff_R14 = uStack_98;
        param_1 = puStack_128;
        iVar38 = (int)uStack_158;
        cStack_1aa = uStack_158._6_1_;
        uStack_1a9 = uStack_158._7_1_;
        uVar23 = uStack_158._4_2_;
      }
      ppppuStack_188 = ppppuStack_248;
      ppppuStack_180 = ppppuStack_240;
      cVar27 = cStack_1aa;
      ppppuStack_2b8 = pppppuVar21;
      uStack_2b0 = uVar19;
      iStack_140 = iVar38;
      uStack_84 = uVar23;
      uStack_75 = uStack_1a9;
    }
    uVar32 = 0;
  }
  else {
    uVar32 = (undefined1)((ulonglong)pppppuVar7 >> 8);
    cVar27 = cStack_71;
    if (lVar20 == 1) {
      if ((char)pppppuVar7 == '\0') {
        lVar20 = lVar26 + uVar17 * 0x48;
        ppppuStack_188 = (undefined8 ****)*puVar1;
        ppppuStack_180 = (undefined8 ****)puVar1[1];
        uStack_298 = *(undefined8 *)(lVar20 + 0x38);
        uStack_290 = *(undefined8 *)(lVar20 + 0x40);
        uStack_1d8 = (undefined8 *****)0x1;
        cStack_1aa = cVar4;
        uStack_1a9 = uVar5;
      }
      else if (((uint)pppppuVar7 & 0xff) == 1) {
        ppppuStack_188 = (undefined8 ****)*puVar1;
        ppppuStack_180 = (undefined8 ****)puVar1[1];
        uStack_1d8 = (undefined8 *****)0x3;
        cStack_1aa = cVar4;
        uStack_1a9 = uVar5;
      }
      else {
        uStack_1d8 = (undefined8 *****)0x4;
        cStack_1aa = cVar4;
        uStack_1a9 = uVar5;
      }
    }
    else {
      if (lVar20 != 2) {
        FUN_1428d9430(&UNK_142b140b8,0x28,&UNK_142b140e0);
        goto code_r0x000140be8cb6;
      }
      uStack_f8 = pppppuVar7;
      ppppuStack_f0 = pppppuVar21;
      uStack_e8 = (undefined8 *****)ppppuStack_288;
      uStack_e0 = (undefined8 *****)ppppuStack_280;
      uStack_d8 = uVar19;
      iStack_d0 = iVar38;
      uStack_cc = uVar23;
      cStack_ca = cVar4;
      uStack_c9 = uVar5;
      uStack_c8 = uVar11;
      uStack_c0 = uVar12;
      if ((char)pppppuVar7 == '\0') {
        uStack_1d8 = (undefined8 *****)0x0;
        uStack_298 = uVar11;
        uStack_290 = uVar12;
        ppppuStack_188 = ppppuStack_288;
        ppppuStack_180 = ppppuStack_280;
        cStack_1aa = cVar4;
        uStack_1a9 = uVar5;
      }
      else {
        if (((uint)pppppuVar7 & 0xff) != 1) {
          FUN_1428d9430(&UNK_142b14018,0xf,&UNK_142b140a0);
          goto code_r0x000140be8cb6;
        }
        uStack_1d8 = (undefined8 *****)0x2;
        ppppuStack_188 = ppppuStack_288;
        ppppuStack_180 = ppppuStack_280;
        cStack_1aa = cVar4;
        uStack_1a9 = uVar5;
      }
    }
  }
  ppppuStack_1c8 = ppppuStack_188;
  uStack_1c0 = (undefined8 *****)ppppuStack_180;
  uStack_1a8 = uStack_298;
  uStack_1a0 = uStack_290;
  ppppuStack_1d0 = pppppuVar21;
  uStack_1b8 = uVar19;
  iStack_1b0 = iVar38;
  uStack_1ac = uVar23;
  if (param_1[8] < 0x10000) {
    if (uVar17 < param_1[2]) {
      uVar24 = param_1[1];
      *(short *)(uVar24 + uVar17 * 4) = (short)param_1[8];
      *(undefined1 *)(uVar24 + 2 + uVar17 * 4) = 0;
      cStack_71 = cVar27;
      if (uVar17 < param_1[5]) {
        *(undefined1 *)(param_1[4] + uVar17) = uVar32;
        uStack_c8 = uStack_298;
        uStack_c0 = uStack_290;
        uStack_e8 = (undefined8 *****)ppppuStack_188;
        uStack_e0 = (undefined8 *****)ppppuStack_180;
        uVar17 = param_1[8];
        uStack_f8 = uStack_1d8;
        ppppuStack_f0 = pppppuVar21;
        uStack_d8 = uVar19;
        iStack_d0 = iVar38;
        uStack_cc = uVar23;
        cStack_ca = cStack_1aa;
        uStack_c9 = uStack_1a9;
        if (uVar17 == param_1[6]) {
          FUN_1405627b0(puStack_2c0,&UNK_142b14088);
        }
        uVar24 = param_1[7];
        lVar20 = uVar17 * 0x40;
        puVar1 = (undefined8 *)(uVar24 + 0x30 + lVar20);
        *puVar1 = uStack_c8;
        puVar1[1] = uStack_c0;
        puVar1 = (undefined8 *)(uVar24 + 0x20 + lVar20);
        *puVar1 = uStack_d8;
        puVar1[1] = CONCAT17(uStack_c9,CONCAT16(cStack_ca,CONCAT24(uStack_cc,iStack_d0)));
        puVar1 = (undefined8 *)(uVar24 + 0x10 + lVar20);
        *puVar1 = uStack_e8;
        puVar1[1] = uStack_e0;
        *(undefined8 *)(uVar24 + lVar20) = uStack_f8;
        ((undefined8 *)(uVar24 + lVar20))[1] = ppppuStack_f0;
        param_1[8] = uVar17 + 1;
        goto code_r0x000140be7ea0;
      }
      uStack_72 = 1;
      FUN_1428d9518(uVar17,param_1[5],&UNK_142b14070);
    }
    else {
      uStack_72 = 1;
      FUN_1428d9518(uVar17,param_1[2],&UNK_142b14058);
    }
  }
  else {
    uStack_72 = 1;
    FUN_1428d9760(&UNK_142b13aa0,0x2b,&uStack_f8,&UNK_142b13a80,&UNK_142b14040);
  }
code_r0x000140be8cb6:
                    /* WARNING: Does not return */
  pcVar8 = (code *)invalidInstructionException();
  (*pcVar8)();
}

