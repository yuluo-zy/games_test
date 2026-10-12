
char FUN_140befd60(longlong param_1)

{
  undefined1 auVar1 [16];
  code *pcVar2;
  uint uVar3;
  uint uVar4;
  int iVar5;
  int iVar6;
  int iVar7;
  int iVar8;
  ulonglong *puVar9;
  char *pcVar10;
  char cVar11;
  ushort uVar12;
  uint *puVar13;
  undefined8 *puVar14;
  ulonglong uVar15;
  ulonglong uVar16;
  longlong lVar17;
  uint uVar18;
  uint uVar19;
  longlong lVar20;
  uint uVar21;
  ulonglong uVar22;
  ulonglong *puVar23;
  bool bVar24;
  byte bVar25;
  char cVar27;
  char cVar28;
  char cVar29;
  undefined1 auVar26 [16];
  undefined1 auVar30 [16];
  undefined1 auVar31 [16];
  undefined1 unaff_XMM6_Ba;
  undefined1 unaff_XMM6_Bb;
  undefined1 unaff_XMM6_Bc;
  undefined1 unaff_XMM6_Bd;
  undefined1 unaff_XMM6_Be;
  undefined1 unaff_XMM6_Bf;
  undefined1 unaff_XMM6_Bg;
  undefined1 unaff_XMM6_Bh;
  undefined1 unaff_XMM6_Bi;
  undefined1 unaff_XMM6_Bj;
  undefined1 unaff_XMM6_Bk;
  undefined1 unaff_XMM6_Bl;
  undefined1 unaff_XMM6_Bm;
  undefined1 unaff_XMM6_Bn;
  undefined1 unaff_XMM6_Bo;
  undefined1 unaff_XMM6_Bp;
  undefined *puStack_228;
  undefined8 *puStack_220;
  undefined8 uStack_218;
  undefined *puStack_210;
  uint *puStack_208;
  undefined **ppuStack_200;
  undefined *puStack_1f8;
  uint *puStack_1f0;
  code *pcStack_1e8;
  undefined **ppuStack_1e0;
  undefined *puStack_1d8;
  undefined **ppuStack_1d0;
  undefined *puStack_1c8;
  undefined *puStack_1c0;
  undefined8 uStack_1b8;
  undefined ***pppuStack_1b0;
  undefined8 uStack_1a8;
  undefined8 *puStack_1a0;
  undefined8 uStack_198;
  longlong lStack_190;
  undefined *puStack_188;
  undefined8 uStack_180;
  undefined8 uStack_178;
  undefined *puStack_170;
  undefined8 uStack_168;
  undefined8 uStack_160;
  undefined8 uStack_158;
  undefined8 uStack_150;
  undefined8 uStack_148;
  undefined *puStack_140;
  undefined8 uStack_138;
  undefined8 uStack_130;
  undefined *puStack_128;
  undefined8 uStack_120;
  undefined ***pppuStack_118;
  undefined1 auStack_110 [16];
  undefined8 uStack_f8;
  undefined8 uStack_f0;
  undefined8 uStack_e8;
  undefined8 uStack_e0;
  undefined1 uStack_d8;
  undefined8 uStack_d0;
  undefined8 uStack_c0;
  undefined8 uStack_b8;
  undefined8 uStack_b0;
  undefined8 uStack_a8;
  undefined1 uStack_a0;
  uint uStack_94;
  char *pcStack_90;
  undefined *puStack_88;
  undefined8 *puStack_80;
  undefined8 uStack_78;
  uint uStack_70;
  uint uStack_6c;
  ulonglong *puStack_68;
  undefined8 uStack_60;
  
  uStack_60 = 0xfffffffffffffffe;
  if (*(char *)(*(longlong *)(param_1 + 0x48) + 0x6c) == '\x02') {
    FUN_1428d9310(&UNK_142d02388);
code_r0x000140bf05df:
    FUN_1428d9310(&UNK_142b15d80);
  }
  else {
    puVar13 = (uint *)func_0x000140c113b0(*(longlong *)(param_1 + 0x48) + 0x28);
    if (puVar13 == (uint *)0x0) goto code_r0x000140bf05df;
    uStack_6c = *puVar13;
    pcStack_90 = (char *)(param_1 + 0x810);
    LOCK();
    bVar24 = *(char *)(param_1 + 0x810) == '\0';
    if (bVar24) {
      *(char *)(param_1 + 0x810) = '\x01';
    }
    UNLOCK();
    if (!bVar24) {
      FUN_1428cd210(pcStack_90,1,1000000000);
    }
    FUN_140c81c70(pcStack_90);
    puVar14 = (undefined8 *)func_0x000140613c10(0xc);
    if (puVar14 != (undefined8 *)0x0) {
      *puVar14 = 0x6b72616d20555047;
      *(undefined4 *)(puVar14 + 1) = 0x3a737265;
      puStack_88 = (undefined *)0xc;
      uStack_78 = 0xc;
      uStack_94 = uStack_6c - 3;
      if (uStack_6c < 3) {
        uStack_94 = 0;
      }
      lStack_190 = param_1 + 0x838;
      iVar5 = -(uint)(CONCAT13(unaff_XMM6_Bd,
                               CONCAT12(unaff_XMM6_Bc,CONCAT11(unaff_XMM6_Bb,unaff_XMM6_Ba))) ==
                     CONCAT13(unaff_XMM6_Bd,
                              CONCAT12(unaff_XMM6_Bc,CONCAT11(unaff_XMM6_Bb,unaff_XMM6_Ba))));
      iVar6 = -(uint)(CONCAT13(unaff_XMM6_Bh,
                               CONCAT12(unaff_XMM6_Bg,CONCAT11(unaff_XMM6_Bf,unaff_XMM6_Be))) ==
                     CONCAT13(unaff_XMM6_Bh,
                              CONCAT12(unaff_XMM6_Bg,CONCAT11(unaff_XMM6_Bf,unaff_XMM6_Be))));
      iVar7 = -(uint)(CONCAT13(unaff_XMM6_Bl,
                               CONCAT12(unaff_XMM6_Bk,CONCAT11(unaff_XMM6_Bj,unaff_XMM6_Bi))) ==
                     CONCAT13(unaff_XMM6_Bl,
                              CONCAT12(unaff_XMM6_Bk,CONCAT11(unaff_XMM6_Bj,unaff_XMM6_Bi))));
      iVar8 = -(uint)(CONCAT13(unaff_XMM6_Bp,
                               CONCAT12(unaff_XMM6_Bo,CONCAT11(unaff_XMM6_Bn,unaff_XMM6_Bm))) ==
                     CONCAT13(unaff_XMM6_Bp,
                              CONCAT12(unaff_XMM6_Bo,CONCAT11(unaff_XMM6_Bn,unaff_XMM6_Bm))));
      puVar23 = (ulonglong *)0x0;
      uVar21 = 0;
      puStack_80 = puVar14;
      while ((puVar9 = puRam000000014362c3a8, puVar23 == (ulonglong *)0x0 && ((int)uVar21 < 7))) {
        uVar19 = uVar21 + 1;
        puVar23 = (ulonglong *)(ulonglong)(5 < (int)uVar21);
        if (5 < (int)uVar21) {
          uVar19 = 6;
        }
        uVar18 = uVar21 + uStack_94;
        puVar13 = (uint *)0xffffffff;
        if (CARRY4(uVar21,uStack_94)) {
          uVar18 = 0xffffffff;
        }
        uStack_178 = CONCAT44(uStack_178._4_4_,uVar18) & 0xffffffff00000fff;
        uStack_70 = uVar18;
        if (*(longlong *)(param_1 + 0x830) == 0) {
          puStack_210 = (undefined *)0x0;
        }
        else {
          uVar15 = FUN_1405b0ed0(lStack_190,&uStack_178);
          lVar17 = *(longlong *)(param_1 + 0x818);
          bVar25 = (byte)(uVar15 >> 0x39);
          auVar26 = ZEXT216(CONCAT11(bVar25,bVar25));
          auVar26 = pshuflw(auVar26,auVar26,0);
          lVar20 = 0;
          while( true ) {
            uVar15 = *(ulonglong *)(param_1 + 0x820) & uVar15;
            auVar1 = *(undefined1 (*) [16])(lVar17 + uVar15);
            cVar11 = auVar26[0];
            auVar31[0] = -(auVar1[0] == cVar11);
            cVar27 = auVar26[1];
            auVar31[1] = -(auVar1[1] == cVar27);
            cVar28 = auVar26[2];
            auVar31[2] = -(auVar1[2] == cVar28);
            cVar29 = auVar26[3];
            auVar31[3] = -(auVar1[3] == cVar29);
            auVar31[4] = -(auVar1[4] == cVar11);
            auVar31[5] = -(auVar1[5] == cVar27);
            auVar31[6] = -(auVar1[6] == cVar28);
            auVar31[7] = -(auVar1[7] == cVar29);
            auVar31[8] = -(auVar1[8] == cVar11);
            auVar31[9] = -(auVar1[9] == cVar27);
            auVar31[10] = -(auVar1[10] == cVar28);
            auVar31[0xb] = -(auVar1[0xb] == cVar29);
            auVar31[0xc] = -(auVar1[0xc] == cVar11);
            auVar31[0xd] = -(auVar1[0xd] == cVar27);
            auVar31[0xe] = -(auVar1[0xe] == cVar28);
            auVar31[0xf] = -(auVar1[0xf] == cVar29);
            uVar12 = (ushort)(SUB161(auVar31 >> 7,0) & 1) |
                     (ushort)(SUB161(auVar31 >> 0xf,0) & 1) << 1 |
                     (ushort)(SUB161(auVar31 >> 0x17,0) & 1) << 2 |
                     (ushort)(SUB161(auVar31 >> 0x1f,0) & 1) << 3 |
                     (ushort)(SUB161(auVar31 >> 0x27,0) & 1) << 4 |
                     (ushort)(SUB161(auVar31 >> 0x2f,0) & 1) << 5 |
                     (ushort)(SUB161(auVar31 >> 0x37,0) & 1) << 6 |
                     (ushort)(SUB161(auVar31 >> 0x3f,0) & 1) << 7 |
                     (ushort)(SUB161(auVar31 >> 0x47,0) & 1) << 8 |
                     (ushort)(SUB161(auVar31 >> 0x4f,0) & 1) << 9 |
                     (ushort)(SUB161(auVar31 >> 0x57,0) & 1) << 10 |
                     (ushort)(SUB161(auVar31 >> 0x5f,0) & 1) << 0xb |
                     (ushort)(SUB161(auVar31 >> 0x67,0) & 1) << 0xc |
                     (ushort)(SUB161(auVar31 >> 0x6f,0) & 1) << 0xd |
                     (ushort)(SUB161(auVar31 >> 0x77,0) & 1) << 0xe |
                     (ushort)(auVar31[0xf] >> 7) << 0xf;
            uVar21 = (uint)uVar12;
            while (puStack_68 = puVar23, uVar12 != 0) {
              uVar3 = 0;
              for (uVar4 = uVar21; (uVar4 & 1) == 0; uVar4 = uVar4 >> 1 | 0x80000000) {
                uVar3 = uVar3 + 1;
              }
              uVar22 = uVar3 + uVar15 & *(ulonglong *)(param_1 + 0x820);
              puVar13 = (uint *)(lVar17 + -0x20 + uVar22 * -0x20);
              if ((uVar18 & 0xfff) == *puVar13) {
                lVar20 = uVar22 * -0x20;
                if (*(uint *)(lVar17 + -0x18 + lVar20) == uVar18) {
                  lVar17 = lVar17 + lVar20;
                  puStack_210 = *(undefined **)(lVar17 + -0x10);
                  puVar13 = *(uint **)(lVar17 + -8);
                }
                else {
                  puStack_210 = (undefined *)0x0;
                }
                goto code_r0x000140beffd4;
              }
              uVar12 = (ushort)(uVar21 - 1) & (ushort)uVar21;
              uVar21 = CONCAT22((short)(uVar21 - 1 >> 0x10),uVar12);
            }
            auVar30[0] = -(auVar1[0] == (char)iVar5);
            auVar30[1] = -(auVar1[1] == (char)((uint)iVar5 >> 8));
            auVar30[2] = -(auVar1[2] == (char)((uint)iVar5 >> 0x10));
            auVar30[3] = -(auVar1[3] == (char)((uint)iVar5 >> 0x18));
            auVar30[4] = -(auVar1[4] == (char)iVar6);
            auVar30[5] = -(auVar1[5] == (char)((uint)iVar6 >> 8));
            auVar30[6] = -(auVar1[6] == (char)((uint)iVar6 >> 0x10));
            auVar30[7] = -(auVar1[7] == (char)((uint)iVar6 >> 0x18));
            auVar30[8] = -(auVar1[8] == (char)iVar7);
            auVar30[9] = -(auVar1[9] == (char)((uint)iVar7 >> 8));
            auVar30[10] = -(auVar1[10] == (char)((uint)iVar7 >> 0x10));
            auVar30[0xb] = -(auVar1[0xb] == (char)((uint)iVar7 >> 0x18));
            auVar30[0xc] = -(auVar1[0xc] == (char)iVar8);
            auVar30[0xd] = -(auVar1[0xd] == (char)((uint)iVar8 >> 8));
            auVar30[0xe] = -(auVar1[0xe] == (char)((uint)iVar8 >> 0x10));
            auVar30[0xf] = -(auVar1[0xf] == (char)((uint)iVar8 >> 0x18));
            uVar12 = (ushort)(SUB161(auVar30 >> 7,0) & 1) |
                     (ushort)(SUB161(auVar30 >> 0xf,0) & 1) << 1 |
                     (ushort)(SUB161(auVar30 >> 0x17,0) & 1) << 2 |
                     (ushort)(SUB161(auVar30 >> 0x1f,0) & 1) << 3 |
                     (ushort)(SUB161(auVar30 >> 0x27,0) & 1) << 4 |
                     (ushort)(SUB161(auVar30 >> 0x2f,0) & 1) << 5 |
                     (ushort)(SUB161(auVar30 >> 0x37,0) & 1) << 6 |
                     (ushort)(SUB161(auVar30 >> 0x3f,0) & 1) << 7 |
                     (ushort)(SUB161(auVar30 >> 0x47,0) & 1) << 8 |
                     (ushort)(SUB161(auVar30 >> 0x4f,0) & 1) << 9 |
                     (ushort)(SUB161(auVar30 >> 0x57,0) & 1) << 10 |
                     (ushort)(SUB161(auVar30 >> 0x5f,0) & 1) << 0xb |
                     (ushort)(SUB161(auVar30 >> 0x67,0) & 1) << 0xc |
                     (ushort)(SUB161(auVar30 >> 0x6f,0) & 1) << 0xd |
                     (ushort)(SUB161(auVar30 >> 0x77,0) & 1) << 0xe |
                     (ushort)(auVar30[0xf] >> 7) << 0xf;
            puVar13 = (uint *)(ulonglong)uVar12;
            if (uVar12 != 0) break;
            uVar15 = uVar15 + lVar20 + 0x10;
            lVar20 = lVar20 + 0x10;
          }
          puStack_210 = (undefined *)0x0;
        }
code_r0x000140beffd4:
        puStack_1a0 = &uStack_178;
        if (puStack_210 == (undefined *)0x0) {
          puStack_210 = &UNK_142b15e68;
          puVar13 = (uint *)0x3;
        }
        puStack_188 = (undefined *)0x1;
        if (uStack_70 == uStack_6c) {
          puStack_188 = &UNK_142b15e6b;
        }
        uStack_180 = 0;
        if (uStack_70 == uStack_6c) {
          uStack_180 = 0x14;
        }
        ppuStack_200 = &puStack_88;
        puStack_1f8 = &UNK_14003bad0;
        puStack_1f0 = &uStack_70;
        pcStack_1e8 = FUN_140d8d560;
        ppuStack_1e0 = &puStack_210;
        puStack_1d8 = &UNK_140109e90;
        ppuStack_1d0 = &puStack_188;
        puStack_1c8 = &UNK_140109e90;
        uStack_178 = 2;
        uStack_168 = 2;
        uStack_160 = 8;
        uStack_158 = 0;
        uStack_150 = 0x20;
        uStack_148 = CONCAT71(uStack_148._1_7_,3);
        puStack_140 = (undefined *)0x2;
        uStack_130 = 0;
        puStack_128 = (undefined *)0x8;
        uStack_120 = 1;
        pppuStack_118 = (undefined ***)0x20;
        auStack_110[0] = 3;
        auStack_110._8_8_ = 2;
        uStack_f8 = 2;
        uStack_f0 = 8;
        uStack_e8 = 2;
        uStack_e0 = 0x20;
        uStack_d8 = 3;
        uStack_d0 = 2;
        uStack_c0 = 2;
        uStack_b8 = 8;
        uStack_b0 = 3;
        uStack_a8 = 0x20;
        uStack_a0 = 3;
        puStack_1c0 = &UNK_142b15e88;
        uStack_1b8 = 4;
        uStack_1a8 = 4;
        uStack_198 = 4;
        puStack_208 = puVar13;
        pppuStack_1b0 = &ppuStack_200;
        FUN_140d79f90(&puStack_228,&puStack_1c0);
        if (puStack_88 != (undefined *)0x0) {
          func_0x000140613c20(puStack_80,puStack_88,1);
        }
        uStack_78 = uStack_218;
        puStack_88 = puStack_228;
        puStack_80 = puStack_220;
        uVar21 = uVar19;
      }
      if (2 < *puRam000000014362c3a8) {
        ppuStack_200 = &puStack_88;
        puStack_1f8 = &UNK_14003bad0;
        puVar14 = (undefined8 *)func_0x00014067dd80(&UNK_142b15da8);
        uStack_158 = *puVar14;
        uStack_150 = puVar14[1];
        uStack_148 = 3;
        puStack_140 = &UNK_142b15d68;
        uStack_138 = 0x14;
        puStack_128 = &UNK_142b15d98;
        uStack_120 = 1;
        auStack_110 = ZEXT816(1);
        uStack_178 = 0;
        puStack_170 = &UNK_142b15d68;
        uStack_168 = 0x14;
        uStack_160 = 0;
        uStack_130 = CONCAT44(*(undefined4 *)(puVar14 + 2),1);
        pppuStack_118 = &ppuStack_200;
        func_0x000140d4a330(&puStack_1c0,&uStack_178);
      }
      puVar23 = (ulonglong *)(param_1 + 0xa88);
      uVar15 = *(ulonglong *)(param_1 + 0xa88);
      if (uVar15 < 0xfffffffffffffff0 && (char)(((uint)uVar15 & 8) >> 3) == '\0') {
        LOCK();
        uVar22 = *puVar23;
        if (uVar15 == uVar22) {
          *puVar23 = uVar15 + 0x10;
        }
        UNLOCK();
        if (uVar15 != uVar22) goto code_r0x000140bf02ae;
      }
      else {
code_r0x000140bf02ae:
        FUN_1428cc4b0(puVar23,0);
      }
      FUN_140c81c70(puVar23);
      uVar15 = (ulonglong)puVar23 | 1;
      FUN_140c81c70(uVar15);
      lVar17 = *(longlong *)(param_1 + 0xa90);
      if (lVar17 == 0) {
        FUN_140c81e10(puVar23);
        FUN_140c81e10(uVar15);
        LOCK();
        uVar15 = *puVar23;
        *puVar23 = *puVar23 - 0x10;
        UNLOCK();
        if ((uVar15 & 0xfffffffffffffff2) == 0x12) {
          FUN_1428ccf50(puVar23);
        }
code_r0x000140bf039c:
        if (*(char *)(param_1 + 0x1b) != '\0') {
          cVar11 = FUN_140c104e0(5,0);
          puStack_188 = (undefined *)CONCAT71(puStack_188._1_7_,cVar11);
          if (cVar11 == '\x04') {
            if (2 < *puVar9) {
              puVar14 = (undefined8 *)func_0x00014067dd80(&UNK_142b15e50);
              uStack_158 = *puVar14;
              uStack_150 = puVar14[1];
              uStack_148 = 3;
              puStack_140 = &UNK_142b15d68;
              uStack_138 = 0x14;
              puStack_128 = &UNK_142b15e40;
              uStack_120 = 1;
              pppuStack_118 = (undefined ***)0x8;
              uStack_178 = 0;
              auStack_110 = (undefined1  [16])0x0;
              puStack_170 = &UNK_142b15d68;
              uStack_168 = 0x14;
              uStack_160 = 0;
              uStack_130 = CONCAT44(*(undefined4 *)(puVar14 + 2),1);
              func_0x000140d4a330(&puStack_1c0,&uStack_178);
            }
          }
          else if (*puVar9 != 0) {
            ppuStack_200 = &puStack_188;
            puStack_1f8 = &UNK_140bedc60;
            puVar14 = (undefined8 *)func_0x00014067dd80(&UNK_142b15e10);
            uStack_158 = *puVar14;
            uStack_150 = puVar14[1];
            uStack_148 = 1;
            puStack_140 = &UNK_142b15d68;
            uStack_138 = 0x14;
            puStack_128 = &UNK_142b15e00;
            uStack_120 = 1;
            auStack_110 = ZEXT816(1);
            uStack_178 = 0;
            puStack_170 = &UNK_142b15d68;
            uStack_168 = 0x14;
            uStack_160 = 0;
            uStack_130 = CONCAT44(*(undefined4 *)(puVar14 + 2),1);
            pppuStack_118 = &ppuStack_200;
            func_0x000140d4a330(&puStack_1c0,&uStack_178);
          }
        }
        if (puStack_88 != (undefined *)0x0) {
          func_0x000140613c20(puStack_80,puStack_88,1);
        }
        pcVar10 = pcStack_90;
        FUN_140c81e10(pcStack_90);
        LOCK();
        cVar11 = *pcVar10;
        bVar24 = cVar11 == '\x01';
        if (bVar24) {
          *pcVar10 = '\0';
          cVar11 = '\x01';
        }
        UNLOCK();
        if (!bVar24) {
          cVar11 = FUN_1428cdb90(pcVar10,0);
        }
        return cVar11;
      }
      uVar22 = *(ulonglong *)(param_1 + 0xa98);
      puStack_68 = puVar23;
      if (1 < uVar22) {
        if ((ulonglong)*(uint *)(lVar17 + 4) != 0) {
          uVar16 = (ulonglong)*(uint *)(lVar17 + 4) * 4 + 4;
          if (uVar22 < uVar16) {
            uVar16 = uVar22;
          }
          if (uVar22 < 4) {
            FUN_1428d9850(4,uVar16,&UNK_142b15dd8);
            goto code_r0x000140bf0614;
          }
          FUN_1418c46c0(lVar17 + 0x10,uVar16 - 4);
        }
        puVar23 = puStack_68;
        FUN_140c81e10(puStack_68);
        FUN_140c81e10(uVar15);
        LOCK();
        uVar15 = *puVar23;
        *puVar23 = *puVar23 - 0x10;
        UNLOCK();
        if ((uVar15 & 0xfffffffffffffff2) == 0x12) {
          FUN_1428ccf50(puVar23);
        }
        goto code_r0x000140bf039c;
      }
      FUN_1428d9518(1,uVar22,&UNK_142b15dc0);
      goto code_r0x000140bf0614;
    }
  }
  FUN_1428d8fe3(1,0xc,&UNK_142b15288);
code_r0x000140bf0614:
                    /* WARNING: Does not return */
  pcVar2 = (code *)invalidInstructionException();
  (*pcVar2)();
}

