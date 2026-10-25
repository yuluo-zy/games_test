
void FUN_1402c1d40(longlong param_1,ulonglong *param_2,undefined8 *****param_3,longlong param_4,
                  longlong param_5,longlong *param_6,undefined8 *****param_7,longlong param_8)

{
  undefined8 ******ppppppuVar1;
  undefined1 uVar2;
  undefined8 ****ppppuVar3;
  code *pcVar4;
  uint *****pppppuVar5;
  uint *****pppppuVar6;
  char cVar7;
  undefined4 uVar8;
  undefined4 uVar9;
  undefined8 ******ppppppuVar10;
  longlong lVar11;
  undefined8 ******ppppppuVar12;
  ulonglong uVar13;
  undefined8 uVar14;
  longlong *plVar15;
  uint *puVar16;
  undefined8 ****ppppuVar17;
  byte bVar18;
  undefined8 *****pppppuVar19;
  undefined *puVar20;
  ulonglong uVar21;
  undefined8 *****pppppuVar22;
  ulonglong uVar23;
  ulonglong uVar24;
  longlong lVar25;
  ulonglong unaff_RDI;
  undefined8 *puVar26;
  undefined8 *****pppppuVar27;
  longlong lVar28;
  byte *pbVar29;
  longlong lVar30;
  longlong lVar31;
  byte *pbVar32;
  bool bVar33;
  longlong in_stack_fffffffffffffdf8;
  undefined8 *****pppppuStack_1f8;
  uint ****ppppuStack_1f0;
  uint ****ppppuStack_1e8;
  longlong lStack_1e0;
  ulonglong uStack_1d8;
  uint ****ppppuStack_1d0;
  undefined8 uStack_1c8;
  ulonglong uStack_1c0;
  longlong *plStack_1b8;
  ulonglong *puStack_1b0;
  longlong lStack_1a8;
  ulonglong *puStack_1a0;
  longlong lStack_198;
  ulonglong uStack_190;
  uint ****ppppuStack_188;
  ulonglong uStack_180;
  undefined8 uStack_178;
  undefined8 uStack_170;
  uint *****pppppuStack_168;
  uint *****pppppuStack_160;
  uint ****ppppuStack_158;
  uint ****ppppuStack_150;
  uint ****ppppuStack_148;
  uint ****ppppuStack_140;
  uint ****ppppuStack_138;
  uint ****ppppuStack_130;
  uint ****ppppuStack_128;
  longlong lStack_120;
  uint *****pppppuStack_118;
  uint *****pppppuStack_110;
  ulonglong uStack_108;
  uint *****pppppuStack_100;
  uint *****pppppuStack_f8;
  uint *****pppppuStack_f0;
  uint *****pppppuStack_e8;
  uint ****ppppuStack_e0;
  uint ****ppppuStack_d8;
  uint ****ppppuStack_d0;
  uint ****ppppuStack_c8;
  uint ****ppppuStack_c0;
  uint ****ppppuStack_b8;
  uint ****ppppuStack_b0;
  longlong lStack_a8;
  uint ****ppppuStack_a0;
  longlong lStack_98;
  uint *****pppppuStack_90;
  uint *****pppppuStack_88;
  uint *****pppppuStack_80;
  ulonglong uStack_78;
  longlong lStack_70;
  longlong lStack_68;
  longlong lStack_60;
  uint ****ppppuStack_58;
  uint ****ppppuStack_50;
  undefined8 uStack_48;
  
  uStack_48 = 0xfffffffffffffffe;
  lVar30 = param_6[2];
  puStack_1a0 = param_2;
  ppppuStack_58 = (uint ****)param_3;
  if (lVar30 == 0) {
code_r0x0001402c2ef6:
    FUN_1428d9310(&UNK_1429aebd0);
  }
  else {
    lVar11 = param_6[1];
    ppppuStack_50 = (uint ****)(lVar30 * 0x58 + lVar11 + -0x58);
    if ((undefined8 *****)ppppuStack_50 == (undefined8 *****)0x0) goto code_r0x0001402c2ef6;
    ppppppuVar12 = *(undefined8 *******)(param_1 + 0x210);
    lStack_198 = lVar11;
    lStack_120 = lVar30;
    lStack_60 = param_4;
    if (ppppppuVar12 != (undefined8 ******)0x0) {
      pppppuStack_168 = (uint *****)(param_1 + 0x200);
      *(undefined8 *)(param_1 + 0x210) = 0;
      ppppppuVar1 = (undefined8 ******)
                    ((longlong)*(undefined8 *******)(param_1 + 0x208) + (longlong)ppppppuVar12 * 4);
      ppppuStack_158 = (uint ****)0x0;
      lStack_70 = param_1 + 0xa8;
      lStack_68 = param_1 + 0x1b0;
      ppppppuVar10 = *(undefined8 *******)(param_1 + 0x208);
      uStack_170 = ppppppuVar1;
      pppppuStack_160 = (uint *****)ppppppuVar12;
      while (ppppppuVar10 != ppppppuVar1) {
        ppppppuVar12 = (undefined8 ******)((longlong)ppppppuVar10 + 4);
        uVar24 = (ulonglong)*(uint *)ppppppuVar10;
        uStack_178 = ppppppuVar12;
        if (*(ulonglong *)(param_1 + 0x130) <= uVar24) {
          FUN_1428d9518(uVar24,*(ulonglong *)(param_1 + 0x130),&UNK_1429af010);
          goto code_r0x0001402c2f80;
        }
        uVar24 = (ulonglong)*(uint *)(*(longlong *)(param_1 + 0x128) + 0x10 + uVar24 * 0x18);
        ppppppuVar10 = ppppppuVar12;
        if (uVar24 != 0xffffffff) {
          if (*(ulonglong *)(param_1 + 0x100) <= uVar24) {
            FUN_1428d9518(uVar24,*(ulonglong *)(param_1 + 0x100),&UNK_1429af028);
            goto code_r0x0001402c2f80;
          }
          lVar30 = uVar24 * 0x14 + *(longlong *)(param_1 + 0xf8);
          FUN_140be78b0(&ppppuStack_1d0,lStack_70);
          if (uStack_1c8._1_1_ == '\0') {
            puVar20 = &UNK_1429af040;
            lVar11 = FUN_140bd6c80(ppppuStack_1d0);
            uVar9 = (undefined4)((ulonglong)in_stack_fffffffffffffdf8 >> 0x20);
            if (lVar11 == 0) {
              if (*(short *)(puVar20 + 0x14) != *(short *)(ppppuStack_58[0xc9] + 0x14a)) {
                pppppuStack_100 = (uint *****)0x0;
                FUN_1428ca816(0,puVar20 + 0x14,ppppuStack_58[0xc9] + 0x14a,&pppppuStack_100,
                              &UNK_1429f7cd8);
                goto code_r0x0001402c2f80;
              }
              if (puVar20[0x16] != '\0') {
                pppppuStack_100 = (uint *****)0x0;
                FUN_1428ca902(0,puVar20 + 0x16,&UNK_1429f7cf0,&pppppuStack_100,&UNK_1429f7cf8);
                goto code_r0x0001402c2f80;
              }
              lVar11 = *(longlong *)(puVar20 + 8);
              if (*(int *)(lVar11 + 0xc) != *(int *)(puVar20 + 0x10)) {
                FUN_1428d9310(&UNK_1429af058);
                goto code_r0x0001402c2f80;
              }
              FUN_140becfd0(lVar11,ppppuStack_50);
              uVar14 = *(undefined8 *)(lVar11 + 0x40);
              unaff_RDI = CONCAT71((int7)(unaff_RDI >> 8),(undefined1)uStack_1c8);
              uVar2 = *(undefined1 *)(lVar30 + 0x10);
              uVar8 = func_0x000140bd6630(*(undefined4 *)(lVar11 + 0x18));
              in_stack_fffffffffffffdf8 = CONCAT44(uVar9,uVar8);
              func_0x000140bd65e0(&pppppuStack_100,uVar14,unaff_RDI & 0xffffffff,
                                  CONCAT71((int7)((ulonglong)lVar30 >> 8),uVar2) & 0xffffffff,
                                  in_stack_fffffffffffffdf8);
              FUN_140bd66a0(lStack_68,&pppppuStack_100);
            }
            else {
              if (lVar11 != 1) {
                FUN_1428d9430(&UNK_1429aed78,0x13,&UNK_1429af088);
                goto code_r0x0001402c2f80;
              }
              if (*(short *)(puVar20 + 0x14) != *(short *)(ppppuStack_58[0xc9] + 0x14a)) {
                pppppuStack_100 = (uint *****)0x0;
                FUN_1428ca816(0,puVar20 + 0x14,ppppuStack_58[0xc9] + 0x14a,&pppppuStack_100,
                              &UNK_1429f7cd8);
                goto code_r0x0001402c2f80;
              }
              if (puVar20[0x16] != '\x01') {
                pppppuStack_100 = (uint *****)0x0;
                FUN_1428ca902(0,puVar20 + 0x16,&UNK_1429f7d10,&pppppuStack_100,&UNK_1429f7cf8);
                goto code_r0x0001402c2f80;
              }
              unaff_RDI = *(ulonglong *)(puVar20 + 8);
              if (*(int *)(unaff_RDI + 0xc) != *(int *)(puVar20 + 0x10)) {
                FUN_1428d9310(&UNK_1429af070);
                goto code_r0x0001402c2f80;
              }
              FUN_140becfd0(unaff_RDI,ppppuStack_50);
              func_0x000140bd6610(&pppppuStack_100,*(undefined8 *)(unaff_RDI + 0x30),
                                  (ulonglong)uStack_1c8 & 0xff,*(undefined1 *)(lVar30 + 0x10));
              FUN_140bd6810(lStack_68,&pppppuStack_100);
            }
          }
        }
      }
    }
    lVar30 = param_1 + 0x1b0;
    cVar7 = func_0x000140bd6980(lVar30);
    if (cVar7 == '\0') {
      FUN_140bee030(ppppuStack_58,ppppuStack_50,lVar30);
    }
    func_0x000140bd6950(lVar30);
    uVar24 = *puStack_1a0;
    uStack_1d8 = uVar24;
    lStack_70 = param_1;
    if (param_5 == 0) {
      ppppuStack_188 = (uint ****)0x4;
      uStack_180 = 0;
      uVar13 = puStack_1a0[1];
      goto code_r0x0001402c210e;
    }
    uVar13 = puStack_1a0[1];
    if (uVar13 < uVar24) {
      FUN_1428d9850(uVar24,uVar13,&UNK_1429aebe8);
code_r0x0001402c2df1:
      FUN_1428d9840(uVar13);
code_r0x0001402c2e00:
      FUN_1428d9430(&UNK_1429ae7c8,0x1c,&UNK_1429ae830);
      goto code_r0x0001402c2f80;
    }
    if (*(ulonglong *)(param_1 + 0xa0) < uVar13) goto code_r0x0001402c2df1;
    uVar23 = uVar13 * 0x40 + uVar24 * -0x40;
    if (uVar23 != 0) {
      lVar11 = *(longlong *)(param_1 + 0x98);
      uStack_78 = uVar23 >> 4;
      pppppuVar27 = (undefined8 *****)func_0x000140613c10(uStack_78,4);
      if (pppppuVar27 != (undefined8 *****)0x0) {
        puVar26 = (undefined8 *)(uVar24 * 0x40 + lVar11 + 0x20);
        uVar21 = 0;
        ppppuStack_50 = (uint ****)pppppuVar27;
        do {
          uVar9 = FUN_140c110f0(param_5,puVar26[-1],*puVar26);
          *(undefined4 *)((longlong)ppppuStack_50 + uVar21 * 4) = uVar9;
          uVar21 = uVar21 + 1;
          puVar26 = puVar26 + 8;
          ppppuStack_188 = ppppuStack_50;
          uStack_180 = uVar23 >> 6;
        } while (uVar23 >> 6 != uVar21);
        goto code_r0x0001402c210e;
      }
      FUN_1428d8fe3(4,uStack_78,&UNK_1429ae8a0);
code_r0x0001402c2f5f:
      FUN_1428d9518(0,0,&UNK_1429aec60);
      goto code_r0x0001402c2f80;
    }
    ppppuStack_188 = (uint ****)0x4;
    uStack_180 = 0;
code_r0x0001402c210e:
    lStack_1e0 = param_5;
    uStack_1c0 = lStack_70 + 0xa8;
    ppppuStack_1d0 = ppppuStack_58;
    uStack_1c8 = &uStack_1d8;
    plStack_1b8 = &lStack_1e0;
    puStack_1b0 = &uStack_190;
    lStack_1a8 = lStack_60;
    pppppuVar27 = (undefined8 *****)(uVar13 - uVar24);
    uStack_190 = uStack_180;
    uStack_78 = uStack_1c0;
    if (uVar13 < uVar24) {
      FUN_1428d9850(uVar24,uVar13,&UNK_1429aec00);
      goto code_r0x0001402c2f80;
    }
    if (*(ulonglong *)(lStack_70 + 0xa0) < uVar13) {
      FUN_1428d9840(uVar13,*(ulonglong *)(lStack_70 + 0xa0),&UNK_1429aec00);
      goto code_r0x0001402c2f80;
    }
    pppppuVar19 = (undefined8 *****)(uVar24 * 0x40 + *(longlong *)(lStack_70 + 0x98));
    bVar18 = ~*(byte *)((longlong)ppppuStack_58 + 0xabb);
    if (*pbRam000000014362c228 != 2) {
      bVar18 = *pbRam000000014362c228;
    }
    lStack_68 = lVar30;
    if ((bVar18 & 1) == 0) {
      FUN_1402c3240(&pppppuStack_100,&ppppuStack_1d0,pppppuVar19,pppppuVar27);
      ppppuStack_128 = ppppuStack_b0;
      ppppuStack_138 = ppppuStack_c0;
      ppppuStack_130 = ppppuStack_b8;
      ppppuStack_148 = ppppuStack_d0;
      ppppuStack_140 = ppppuStack_c8;
      ppppuStack_158 = ppppuStack_e0;
      ppppuStack_150 = ppppuStack_d8;
      pppppuStack_168 = pppppuStack_f0;
      pppppuStack_160 = pppppuStack_e8;
      uStack_178 = (undefined8 ******)pppppuStack_100;
      uStack_170 = (undefined8 ******)pppppuStack_f8;
      lStack_60 = lStack_a8;
      ppppuStack_50 = ppppuStack_a0;
      if (lStack_98 != 0) {
        pppppuVar27 = (undefined8 *****)ppppuStack_a0;
        do {
          ppppuVar3 = pppppuVar27[1];
          FUN_1402ca150(param_7,*pppppuVar27,ppppuVar3);
          param_7[6] = (undefined8 ****)((longlong)param_7[6] + (longlong)ppppuVar3);
          ppppuVar17 = param_7[7];
          if (param_7[7] < ppppuVar3) {
            ppppuVar17 = ppppuVar3;
          }
          param_7[7] = ppppuVar17;
          pppppuVar27 = pppppuVar27 + 2;
        } while (pppppuVar27 != (undefined8 *****)(ppppuStack_a0 + lStack_98 * 2));
      }
      if (lStack_60 != 0) {
        func_0x000140613c20(ppppuStack_50,lStack_60 << 4,8);
      }
      ppppppuVar12 = (undefined8 ******)func_0x000140613c10(0x58,8);
      if (ppppppuVar12 == (undefined8 ******)0x0) {
        FUN_1428d9000(8,0x58);
        goto code_r0x0001402c2f80;
      }
      ppppppuVar12[10] = (undefined8 *****)ppppuStack_128;
      ppppppuVar12[8] = (undefined8 *****)ppppuStack_138;
      ppppppuVar12[9] = (undefined8 *****)ppppuStack_130;
      ppppppuVar12[6] = (undefined8 *****)ppppuStack_148;
      ppppppuVar12[7] = (undefined8 *****)ppppuStack_140;
      ppppppuVar12[4] = (undefined8 *****)ppppuStack_158;
      ppppppuVar12[5] = (undefined8 *****)ppppuStack_150;
      ppppppuVar12[2] = (undefined8 *****)pppppuStack_168;
      ppppppuVar12[3] = (undefined8 *****)pppppuStack_160;
      *(uint *)ppppppuVar12 = (uint)uStack_178;
      *(uint *)((longlong)ppppppuVar12 + 4) = uStack_178._4_4_;
      *(uint *)(ppppppuVar12 + 1) = (uint)uStack_170;
      *(uint *)((longlong)ppppppuVar12 + 0xc) = uStack_170._4_4_;
      pppppuStack_118 = (uint *****)0x1;
      uStack_108 = 1;
      lVar30 = lStack_70;
      pppppuStack_110 = (uint *****)ppppppuVar12;
code_r0x0001402c25d9:
      uVar24 = uStack_108;
      pppppuVar5 = pppppuStack_110;
      lVar31 = uStack_108 * 0x58;
      pppppuStack_e8 = pppppuStack_110 + uStack_108 * 0xb;
      lVar11 = lStack_198;
      lVar28 = lStack_120;
      pppppuStack_100 = pppppuStack_110;
      pppppuStack_f8 = pppppuStack_110;
      pppppuStack_f0 = pppppuStack_118;
      if ((ulonglong)(*param_6 - lStack_120) < uStack_108) {
        in_stack_fffffffffffffdf8 = 0;
        FUN_1428a29e0(param_6,lStack_120,uStack_108,8,0x58);
        lVar11 = param_6[1];
        lVar28 = param_6[2];
      }
      func_0x000142923c00(lVar28 * 0x58 + lVar11,pppppuVar5,lVar31);
      lVar28 = uVar24 + lVar28;
      param_6[2] = lVar28;
      pppppuStack_e8 = pppppuVar5;
      FUN_1405d12d0(&pppppuStack_100);
      uVar24 = lStack_120 - 1;
      if (lVar28 - 1U < uVar24) {
        FUN_1428d9850(uVar24,lVar28 - 1U,&UNK_1429aec78);
      }
      else {
        if (lVar28 != 0) {
          lVar31 = uVar24 * 0x58 + lVar11;
          lVar25 = (lVar28 - lStack_120) * 0x58 + lVar31;
          for (; lVar31 != lVar25; lVar31 = lVar31 + 0x58) {
            FUN_140bed3d0(lVar31,ppppuStack_58);
            lVar30 = lStack_70;
          }
          if ((char)puStack_1a0[2] != '\0') {
            lVar11 = lVar11 + lVar28 * 0x58 + -0x58;
            if (lVar11 == 0) {
              FUN_1428d9310(&UNK_1429aec90);
              goto code_r0x0001402c2f80;
            }
            FUN_140bf09b0(ppppuStack_58,lVar11,&UNK_1429aeda8,9);
            if (*(longlong *)(lVar30 + 0x28) != 0) {
              pbVar32 = *(byte **)(lVar30 + 0x20);
              pbVar29 = pbVar32 + *(longlong *)(lVar30 + 0x28) * 0x30;
              do {
                uVar9 = (undefined4)((ulonglong)in_stack_fffffffffffffdf8 >> 0x20);
                if ((*pbVar32 & 1) == 0) {
                  FUN_140be78b0(&uStack_178,uStack_78,*(undefined4 *)(pbVar32 + 4));
                  puVar20 = &UNK_1429aedb8;
                  lVar28 = FUN_140bd6c80(uStack_178);
                  if (lVar28 != 0) {
                    FUN_1428d9430(&UNK_1429aee18,0x28,&UNK_1429aee40);
                    goto code_r0x0001402c2f80;
                  }
                  puVar16 = (uint *)(pbVar32 + 4);
                  if (*pbVar32 != 0) {
                    puVar16 = (uint *)(pbVar32 + 0x18);
                  }
                  uVar24 = (ulonglong)*puVar16;
                  if (*(ulonglong *)(lVar30 + 0x130) <= uVar24) {
                    FUN_1428d9518(uVar24,*(ulonglong *)(lVar30 + 0x130),&UNK_1429aedd0);
                    goto code_r0x0001402c2f80;
                  }
                  uVar24 = (ulonglong)
                           *(uint *)(*(longlong *)(lVar30 + 0x128) + 0x14 + uVar24 * 0x18);
                  if (uVar24 == 0xffffffff) {
                    uVar2 = (undefined1)uStack_170;
                  }
                  else {
                    if (*(ulonglong *)(lVar30 + 0x100) <= uVar24) {
                      FUN_1428d9518(uVar24,*(ulonglong *)(lVar30 + 0x100),&UNK_1429aede8);
                      goto code_r0x0001402c2f80;
                    }
                    uVar2 = *(undefined1 *)(*(longlong *)(lVar30 + 0xf8) + 0x10 + uVar24 * 0x14);
                  }
                  if (*(short *)(puVar20 + 0x14) != *(short *)(ppppuStack_58[0xc9] + 0x14a)) {
                    pppppuStack_100 = (uint *****)0x0;
                    FUN_1428ca816(0,puVar20 + 0x14,ppppuStack_58[0xc9] + 0x14a,&pppppuStack_100,
                                  &UNK_1429f7cd8);
                    goto code_r0x0001402c2f80;
                  }
                  if (puVar20[0x16] != '\0') {
                    pppppuStack_100 = (uint *****)0x0;
                    FUN_1428ca902(0,puVar20 + 0x16,&UNK_1429f7cf0,&pppppuStack_100,&UNK_1429f7cf8);
                    goto code_r0x0001402c2f80;
                  }
                  lVar28 = *(longlong *)(puVar20 + 8);
                  if (*(int *)(lVar28 + 0xc) != *(int *)(puVar20 + 0x10)) {
                    FUN_1428d9310(&UNK_1429aee00);
                    goto code_r0x0001402c2f80;
                  }
                  FUN_140becfd0(lVar28,lVar11);
                  uVar14 = *(undefined8 *)(lVar28 + 0x40);
                  bVar18 = pbVar32[0x28];
                  uVar8 = func_0x000140bd6630(*(undefined4 *)(pbVar32 + 0x10));
                  in_stack_fffffffffffffdf8 = CONCAT44(uVar9,uVar8);
                  func_0x000140bd65e0(&pppppuStack_100,uVar14,uVar2,
                                      CONCAT71((int7)((ulonglong)lVar28 >> 8),bVar18) & 0xffffffff,
                                      in_stack_fffffffffffffdf8);
                  FUN_140bd66a0(lStack_68,&pppppuStack_100);
                }
                else {
                  FUN_140be78b0(&uStack_178,uStack_78,*(undefined4 *)(pbVar32 + 0x18));
                  puVar20 = &UNK_1429aee58;
                  lVar28 = FUN_140bd6c80(uStack_178);
                  if (lVar28 != 1) {
                    FUN_1428d9430(&UNK_1429aee18,0x28,&UNK_1429aeeb8);
                    goto code_r0x0001402c2f80;
                  }
                  puVar16 = (uint *)(pbVar32 + 4);
                  if (*pbVar32 != 0) {
                    puVar16 = (uint *)(pbVar32 + 0x18);
                  }
                  uVar24 = (ulonglong)*puVar16;
                  if (*(ulonglong *)(lVar30 + 0x130) <= uVar24) {
                    FUN_1428d9518(uVar24,*(ulonglong *)(lVar30 + 0x130),&UNK_1429aee70);
                    goto code_r0x0001402c2f80;
                  }
                  uVar24 = (ulonglong)
                           *(uint *)(*(longlong *)(lVar30 + 0x128) + 0x14 + uVar24 * 0x18);
                  if (uVar24 == 0xffffffff) {
                    uVar2 = (undefined1)uStack_170;
                  }
                  else {
                    if (*(ulonglong *)(lVar30 + 0x100) <= uVar24) {
                      FUN_1428d9518(uVar24,*(ulonglong *)(lVar30 + 0x100),&UNK_1429aee88);
                      goto code_r0x0001402c2f80;
                    }
                    uVar2 = *(undefined1 *)(*(longlong *)(lVar30 + 0xf8) + 0x10 + uVar24 * 0x14);
                  }
                  if (*(short *)(puVar20 + 0x14) != *(short *)(ppppuStack_58[0xc9] + 0x14a)) {
                    pppppuStack_100 = (uint *****)0x0;
                    FUN_1428ca816(0,puVar20 + 0x14,ppppuStack_58[0xc9] + 0x14a,&pppppuStack_100,
                                  &UNK_1429f7cd8);
                    goto code_r0x0001402c2f80;
                  }
                  if (puVar20[0x16] != '\x01') {
                    pppppuStack_100 = (uint *****)0x0;
                    FUN_1428ca902(0,puVar20 + 0x16,&UNK_1429f7d10,&pppppuStack_100,&UNK_1429f7cf8);
                    goto code_r0x0001402c2f80;
                  }
                  lVar28 = *(longlong *)(puVar20 + 8);
                  if (*(int *)(lVar28 + 0xc) != *(int *)(puVar20 + 0x10)) {
                    FUN_1428d9310(&UNK_1429aeea0);
                    goto code_r0x0001402c2f80;
                  }
                  FUN_140becfd0(lVar28,lVar11);
                  func_0x000140bd6610(&pppppuStack_100,*(undefined8 *)(lVar28 + 0x30),uVar2,
                                      pbVar32[0x28]);
                  FUN_140bd6810(lStack_68,&pppppuStack_100);
                }
                pbVar32 = pbVar32 + 0x30;
              } while (pbVar32 != pbVar29);
            }
            cVar7 = func_0x000140bd6980(lStack_68);
            if (cVar7 == '\0') {
              FUN_140bee030(ppppuStack_58,lVar11,lStack_68);
            }
            func_0x000140bd6950(lStack_68);
            FUN_140bf0b40(ppppuStack_58,lVar11);
          }
          if (uStack_190 != 0) {
            func_0x000140613c20(ppppuStack_188,uStack_190 << 2,4);
          }
          return;
        }
        FUN_1428d9840(0xffffffffffffffff,0,&UNK_1429aec78);
      }
      goto code_r0x0001402c2f80;
    }
    ppppuStack_1e8 = (uint ****)pppppuVar19;
    ppppppuVar12 = (undefined8 ******)func_0x000140613c10(0x100,8);
    if (ppppppuVar12 != (undefined8 ******)0x0) {
      pppppuStack_90 = (uint *****)0x10;
      pppppuStack_80 = (uint *****)0x0;
      uVar13 = *(ulonglong *)(param_8 + 0x30) >> 4;
      uVar24 = *(ulonglong *)(param_8 + 0x30) >> 1;
      pppppuStack_88 = (uint *****)ppppppuVar12;
      if (uVar13 <= uVar24) {
        uVar23 = *(ulonglong *)(param_8 + 0x38);
        if (uVar23 < uVar24) {
          uVar24 = uVar23;
        }
        if (uVar23 < uVar13) {
          uVar24 = uVar13;
        }
        while (pppppuVar27 != (undefined8 *****)0x0) {
          pppppuVar22 = pppppuVar19 + (longlong)pppppuVar27 * 8;
          uVar13 = 0;
          lVar30 = 0;
          lVar11 = 0;
          ppppuStack_50 = (uint ****)pppppuVar19;
          do {
            lVar28 = lVar11;
            lVar11 = lVar30;
            if ((pppppuVar19 == pppppuVar22) || (pppppuVar19 == (undefined8 *****)0x0)) break;
            lVar30 = func_0x00014067dd80(pppppuVar19[3],pppppuVar19[4]);
            uVar23 = FUN_140bea1e0(param_8);
            if ((uVar23 & 1) == 0) {
              lVar30 = 0;
            }
            uVar13 = uVar13 + lVar30;
            pppppuVar19 = pppppuVar19 + 8;
            lVar30 = lVar11 + 1;
          } while (uVar13 < uVar24);
          pppppuVar5 = pppppuStack_80;
          pppppuVar22 = (undefined8 *****)(lVar28 + 1);
          bVar33 = pppppuVar27 < pppppuVar22;
          pppppuVar27 = (undefined8 *****)((longlong)pppppuVar27 - (longlong)pppppuVar22);
          if (bVar33) {
            pppppuStack_100 = (uint *****)&UNK_1429ae8c8;
            pppppuStack_f8 = (uint *****)0x1;
            pppppuStack_f0 = (uint *****)0x8;
            pppppuStack_e8 = (uint *****)0x0;
            ppppuStack_e0 = (uint ****)0x0;
            FUN_1428d9390(&pppppuStack_100,&UNK_1429aec30);
            goto code_r0x0001402c2f80;
          }
          if (pppppuStack_80 == pppppuStack_90) {
            FUN_140562630(&pppppuStack_90,&UNK_1429aec48);
          }
          pppppuVar19 = (undefined8 *****)(ppppuStack_50 + (longlong)pppppuVar22 * 8);
          pppppuStack_88[(longlong)pppppuVar5 * 2] = ppppuStack_50;
          pppppuStack_88[(longlong)pppppuVar5 * 2 + 1] = (uint ****)pppppuVar22;
          pppppuStack_80 = (uint *****)((longlong)pppppuVar5 + 1);
        }
        if ((undefined8 ******)0x1 < pppppuStack_80) {
          uVar14 = FUN_140d45730();
          pppppuVar6 = pppppuStack_80;
          pppppuVar5 = pppppuStack_88;
          ppppuStack_1f0 = (uint ****)0x1;
          pppppuStack_118 = &ppppuStack_1e8;
          pppppuStack_110 = &ppppuStack_1d0;
          pppppuStack_1f8 = &pppppuStack_118;
          plVar15 = (longlong *)FUN_1402b54b0(0);
          if (plVar15 != (longlong *)0x0) {
            pppppuStack_100 = pppppuVar5;
            pppppuStack_f8 = pppppuVar6;
            pppppuStack_f0 = &ppppuStack_1f0;
            pppppuStack_e8 = (uint *****)&pppppuStack_1f8;
            in_stack_fffffffffffffdf8 = *plVar15 + 0x10;
            FUN_1401058a0(&uStack_178,uVar14,1,in_stack_fffffffffffffdf8,in_stack_fffffffffffffdf8,
                          &pppppuStack_100);
            if (!SBORROW8(0,(longlong)uStack_178)) {
              pppppuStack_e8 = (uint *****)(uStack_170 + (longlong)pppppuStack_168 * 0xe);
              pppppuStack_100 = (uint *****)uStack_170;
              pppppuStack_f8 = (uint *****)uStack_170;
              pppppuStack_f0 = (uint *****)uStack_178;
              ppppuStack_e0 = (uint ****)param_7;
              FUN_140562480(&pppppuStack_118,&pppppuStack_100,&UNK_1429ae548);
              lVar30 = lStack_70;
              goto joined_r0x0001402c25a8;
            }
          }
          FUN_1428d80d0(&UNK_142932a10);
          goto code_r0x0001402c2f80;
        }
        if ((undefined8 ******)pppppuStack_80 == (undefined8 ******)0x0) goto code_r0x0001402c2f5f;
        FUN_1402c3240(&pppppuStack_100,&ppppuStack_1d0,*pppppuStack_88,pppppuStack_88[1]);
        ppppuStack_128 = ppppuStack_b0;
        ppppuStack_138 = ppppuStack_c0;
        ppppuStack_130 = ppppuStack_b8;
        ppppuStack_148 = ppppuStack_d0;
        ppppuStack_140 = ppppuStack_c8;
        ppppuStack_158 = ppppuStack_e0;
        ppppuStack_150 = ppppuStack_d8;
        pppppuStack_168 = pppppuStack_f0;
        pppppuStack_160 = pppppuStack_e8;
        uStack_178 = (undefined8 ******)pppppuStack_100;
        uStack_170 = (undefined8 ******)pppppuStack_f8;
        lStack_60 = lStack_a8;
        ppppuStack_50 = ppppuStack_a0;
        if (lStack_98 != 0) {
          pppppuVar27 = (undefined8 *****)ppppuStack_a0;
          do {
            ppppuVar3 = pppppuVar27[1];
            FUN_1402ca150(param_7,*pppppuVar27,ppppuVar3);
            param_7[6] = (undefined8 ****)((longlong)param_7[6] + (longlong)ppppuVar3);
            ppppuVar17 = param_7[7];
            if (param_7[7] < ppppuVar3) {
              ppppuVar17 = ppppuVar3;
            }
            param_7[7] = ppppuVar17;
            pppppuVar27 = pppppuVar27 + 2;
          } while (pppppuVar27 != (undefined8 *****)(ppppuStack_a0 + lStack_98 * 2));
        }
        if (lStack_60 != 0) {
          func_0x000140613c20(ppppuStack_50,lStack_60 << 4,8);
        }
        ppppppuVar12 = (undefined8 ******)func_0x000140613c10(0x58,8);
        if (ppppppuVar12 == (undefined8 ******)0x0) {
          FUN_1428d9000(8,0x58);
          goto code_r0x0001402c2f80;
        }
        ppppppuVar12[10] = (undefined8 *****)ppppuStack_128;
        ppppppuVar12[8] = (undefined8 *****)ppppuStack_138;
        ppppppuVar12[9] = (undefined8 *****)ppppuStack_130;
        ppppppuVar12[6] = (undefined8 *****)ppppuStack_148;
        ppppppuVar12[7] = (undefined8 *****)ppppuStack_140;
        ppppppuVar12[4] = (undefined8 *****)ppppuStack_158;
        ppppppuVar12[5] = (undefined8 *****)ppppuStack_150;
        ppppppuVar12[2] = (undefined8 *****)pppppuStack_168;
        ppppppuVar12[3] = (undefined8 *****)pppppuStack_160;
        *(uint *)ppppppuVar12 = (uint)uStack_178;
        *(uint *)((longlong)ppppppuVar12 + 4) = uStack_178._4_4_;
        *(uint *)(ppppppuVar12 + 1) = (uint)uStack_170;
        *(uint *)((longlong)ppppppuVar12 + 0xc) = uStack_170._4_4_;
        pppppuStack_118 = (uint *****)0x1;
        uStack_108 = 1;
        pppppuStack_110 = (uint *****)ppppppuVar12;
        lVar30 = lStack_70;
joined_r0x0001402c25a8:
        lStack_70 = lVar30;
        if ((undefined8 ******)pppppuStack_90 != (undefined8 ******)0x0) {
          func_0x000140613c20(pppppuStack_88,(longlong)pppppuStack_90 << 4,8);
        }
        goto code_r0x0001402c25d9;
      }
      goto code_r0x0001402c2e00;
    }
  }
  FUN_1428d8fe3(8,0x100,&UNK_1429aec18);
code_r0x0001402c2f80:
                    /* WARNING: Does not return */
  pcVar4 = (code *)invalidInstructionException();
  (*pcVar4)();
}

