
void FUN_1408bc650(undefined8 *param_1,undefined8 *param_2,undefined8 *param_3,
                  undefined1 (**param_4) [16])

{
  byte *pbVar1;
  int iVar2;
  undefined1 auVar3 [16];
  byte bVar4;
  byte bVar5;
  ulonglong *puVar6;
  code *pcVar7;
  ushort uVar8;
  undefined4 uVar9;
  char cVar10;
  byte bVar11;
  longlong lVar12;
  undefined *puVar13;
  undefined *puVar14;
  undefined8 *puVar15;
  longlong lVar16;
  longlong lVar17;
  undefined8 uVar18;
  longlong *plVar19;
  undefined1 (*pauVar20) [16];
  longlong lVar21;
  ulonglong uVar22;
  undefined1 (*pauVar23) [16];
  undefined4 *puVar24;
  uint uVar25;
  uint uVar26;
  ulonglong uVar27;
  ulonglong uVar28;
  ulonglong uVar29;
  ulonglong uVar30;
  undefined *puVar31;
  undefined *puVar32;
  longlong lVar33;
  longlong unaff_R15;
  bool bVar34;
  bool bVar35;
  undefined1 uVar36;
  undefined1 (**ppauStack_138) [16];
  byte bStack_130;
  undefined7 uStack_12f;
  undefined8 uStack_128;
  undefined8 uStack_120;
  undefined8 uStack_118;
  undefined8 uStack_110;
  undefined8 uStack_108;
  undefined8 uStack_100;
  undefined8 uStack_f8;
  undefined4 uStack_f0;
  undefined4 uStack_ec;
  undefined *puStack_e8;
  undefined8 uStack_e0;
  undefined8 uStack_d8;
  undefined8 uStack_d0;
  undefined8 uStack_c8;
  undefined1 (*pauStack_b8) [16];
  undefined1 (*pauStack_b0) [16];
  undefined1 *puStack_a8;
  undefined8 uStack_a0;
  undefined8 uStack_98;
  undefined8 uStack_90;
  undefined4 uStack_88;
  undefined4 uStack_84;
  undefined4 uStack_80;
  undefined4 uStack_7c;
  undefined1 (**ppauStack_70) [16];
  byte bStack_68;
  longlong lStack_60;
  undefined8 uStack_58;
  longlong *plStack_50;
  undefined8 uStack_48;
  
  uStack_48 = 0xfffffffffffffffe;
  cVar10 = func_0x000140c16a00();
  if (cVar10 == '\0') {
code_r0x0001408bc6ec:
    lStack_60 = 0;
  }
  else {
    puVar31 = &UNK_142aa05f9;
    unaff_R15 = 0x2c;
    uVar22 = 0x2c;
    lVar12 = FUN_1408b9b00(&UNK_142aa05f9);
    if (lVar12 == 1) {
      if (uVar22 != 0) {
        if (uVar22 < 0x2c) {
          if ((char)(&UNK_142aa05f9)[uVar22] < -0x40) {
code_r0x0001408bc6cc:
            FUN_1428d9940(&UNK_142aa05f9,0x2c,0,uVar22,&UNK_142aa06f8);
            goto code_r0x0001408bc6ec;
          }
        }
        else if (uVar22 != 0x2c) goto code_r0x0001408bc6cc;
      }
      puVar31 = &UNK_142aa05f9;
      lVar12 = FUN_1408b9b00(&UNK_142aa05f9);
      if (lVar12 == 1) {
        uVar28 = uVar22 + 2;
        if (uVar28 != 0) {
          uVar36 = uVar28 == 0x2c;
          if (0x2b < uVar28) goto code_r0x0001408bc74d;
          uVar36 = (&UNK_142aa05fb)[uVar22] == -0x41;
          if ((char)(&UNK_142aa05fb)[uVar22] < -0x40) {
            do {
              uVar22 = 0x2c;
              FUN_1428d9940(&UNK_142aa05f9);
code_r0x0001408bc74d:
            } while (!(bool)uVar36);
          }
        }
        unaff_R15 = 0x2a - uVar22;
        puVar31 = &UNK_142aa05f9 + uVar28;
      }
    }
    puVar13 = &UNK_142aa09c0;
    puVar32 = &UNK_142aa0995;
    do {
      bVar11 = puVar13[-1];
      uVar25 = (uint)(char)bVar11;
      if ((char)bVar11 < '\0') {
        bVar4 = puVar13[-2];
        if ((char)bVar4 < -0x40) {
          bVar5 = puVar13[-3];
          if ((char)bVar5 < -0x40) {
            pbVar1 = puVar13 + -4;
            puVar13 = puVar13 + -4;
            uVar25 = bVar5 & 0x3f | (*pbVar1 & 7) << 6;
          }
          else {
            puVar13 = puVar13 + -3;
            uVar25 = bVar5 & 0xf;
          }
          uVar25 = bVar4 & 0x3f | uVar25 << 6;
        }
        else {
          puVar13 = puVar13 + -2;
          uVar25 = bVar4 & 0x1f;
        }
        uVar25 = (uint)(bVar11 & 0x3f) | uVar25 << 6;
        if (uVar25 != 0x5c) goto code_r0x0001408bc7e9;
code_r0x0001408bc7fe:
        puVar14 = puVar13 + -0x142aa0995;
        puVar32 = puVar13 + -0x142aa0994;
        if (puVar32 != (undefined *)0x0) {
          uVar36 = puVar32 == (undefined *)0x2b;
          if ((undefined *)0x2a < puVar32) goto code_r0x0001408bc840;
          uVar36 = puVar13[1] == -0x41;
          if ((char)puVar13[1] < -0x40) {
            do {
              puVar14 = (undefined *)FUN_1428d9940(&UNK_142aa0995,0x2b);
code_r0x0001408bc840:
            } while (!(bool)uVar36);
          }
        }
        puVar24 = (undefined4 *)(0x2a - (longlong)puVar14);
        puVar32 = &UNK_142aa0995 + (longlong)puVar32;
        goto code_r0x0001408bc84d;
      }
      puVar13 = puVar13 + -1;
      if (uVar25 == 0x5c) goto code_r0x0001408bc7fe;
code_r0x0001408bc7e9:
      if (uVar25 == 0x2f) goto code_r0x0001408bc7fe;
    } while (puVar13 != &UNK_142aa0995);
    puVar24 = (undefined4 *)0x2b;
code_r0x0001408bc84d:
    plVar19 = (longlong *)func_0x000140c18130();
    if (*plVar19 == 1) {
      plVar19 = plVar19 + 1;
    }
    else {
      if (*plVar19 != 0) goto code_r0x0001408bcdf2;
      plVar19 = (longlong *)FUN_1428f87d0(plVar19,0);
      if (plVar19 == (longlong *)0x0) goto code_r0x0001408bcdf2;
    }
    if (*plVar19 != 0) goto code_r0x0001408bce0b;
    *plVar19 = -1;
    plStack_50 = plVar19;
    uStack_58 = FUN_140c16d40(plVar19 + 1,puVar31,unaff_R15,puVar32,puVar24,1,0);
    *plStack_50 = *plStack_50 + 1;
    lStack_60 = 1;
  }
  pauVar23 = *param_4;
  pauVar20 = param_4[1];
  uVar22 = *(ulonglong *)*pauVar23;
  uVar27 = 0;
  uVar28 = uVar22 - *(ulonglong *)(pauVar20[1] + 8);
  if (uVar22 < *(ulonglong *)(pauVar20[1] + 8)) {
    uVar28 = uVar27;
  }
  uVar30 = uVar22 - *(ulonglong *)(pauVar20[3] + 8);
  if (uVar22 < *(ulonglong *)(pauVar20[3] + 8)) {
    uVar30 = uVar27;
  }
  bVar34 = uVar30 <= *(ulonglong *)pauVar20[3];
  uVar30 = *(ulonglong *)pauVar20[3] - uVar30;
  uVar22 = uVar30;
  if (!bVar34) {
    uVar22 = uVar27;
  }
  bVar35 = uVar28 <= *(ulonglong *)pauVar20[1];
  uVar28 = *(ulonglong *)pauVar20[1] - uVar28;
  uVar29 = uVar28;
  if (!bVar35) {
    uVar29 = uVar27;
  }
  lVar12 = *(longlong *)pauVar20[4];
  *(ulonglong *)*pauVar23 = lVar12 - (uVar29 + uVar22);
  if ((bVar35 && uVar28 != 0 || bVar34 && uVar30 != 0) &&
     (*(longlong *)*pauVar23 = lVar12, 4 < *puRam000000014362c3a8)) {
    puVar15 = (undefined8 *)func_0x00014067dd80(&UNK_142aa0a08);
    uStack_118 = *puVar15;
    uStack_110 = puVar15[1];
    uStack_ec = *(undefined4 *)(puVar15 + 2);
    uStack_108 = 5;
    uStack_100 = &UNK_142aa0a20;
    uStack_f8 = 0x1b;
    puStack_e8 = &UNK_142aa09f8;
    uStack_e0 = 1;
    uStack_d8 = 8;
    ppauStack_138 = (undefined1 (**) [16])0x0;
    uStack_d0 = 0;
    uStack_c8 = 0;
    bStack_130 = 0x20;
    uStack_12f = 0x142aa0a;
    uStack_128 = (undefined1 (**) [16])0x1b;
    uStack_120 = 0;
    uStack_f0 = 1;
    func_0x000140d4a330(&pauStack_b8,&ppauStack_138);
  }
  puVar24 = (undefined4 *)param_1[2];
  plStack_50 = (longlong *)CONCAT44(plStack_50._4_4_,*(undefined4 *)((longlong)param_1 + 0x1c));
  *puVar24 = *(undefined4 *)((longlong)param_1 + 0x1c);
  param_1 = (undefined8 *)*param_1;
  *(byte *)(param_1 + 7) = *(byte *)(param_1 + 7) | (bVar35 && uVar28 != 0 || bVar34 && uVar30 != 0)
  ;
  puVar6 = (ulonglong *)*param_3;
  lVar12 = param_3[1];
  uVar22 = *puVar6;
  uVar27 = 0;
  uVar28 = uVar22 - *(ulonglong *)(lVar12 + 0x18);
  if (uVar22 < *(ulonglong *)(lVar12 + 0x18)) {
    uVar28 = uVar27;
  }
  uVar30 = uVar22 - *(ulonglong *)(lVar12 + 0x38);
  if (uVar22 < *(ulonglong *)(lVar12 + 0x38)) {
    uVar30 = uVar27;
  }
  lVar16 = uVar28 * 0x10 + *(longlong *)(lVar12 + 8);
  uVar22 = *(ulonglong *)(lVar12 + 0x10) - uVar28;
  if (*(ulonglong *)(lVar12 + 0x10) < uVar28) {
    lVar16 = 8;
    uVar22 = uVar27;
  }
  lVar21 = uVar30 * 0x10 + *(longlong *)(lVar12 + 0x28);
  uVar28 = *(ulonglong *)(lVar12 + 0x30) - uVar30;
  if (*(ulonglong *)(lVar12 + 0x30) < uVar30) {
    lVar21 = 8;
    uVar28 = uVar27;
  }
  *puVar6 = (*(longlong *)(lVar12 + 0x40) - uVar28) - uVar22;
  lVar12 = uVar22 * 0x10 + lVar16;
  lVar33 = uVar28 * 0x10 + lVar21;
  param_4 = &pauStack_b8;
  do {
    if ((lVar16 == 0) || (lVar16 == lVar12)) {
      if ((lVar21 == 0) || (lVar21 == lVar33)) {
        if (*(char *)(param_1 + 7) == '\0') {
          pauVar20 = (undefined1 (*) [16])*param_1;
          lVar12 = param_1[3];
          auVar3 = *pauVar20;
          uVar25 = ~(uint)(ushort)((ushort)(SUB161(auVar3 >> 7,0) & 1) |
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
          pauVar23 = pauVar20 + 1;
          do {
            bVar34 = lVar12 == 0;
            lVar12 = lVar12 + -1;
            if (bVar34) goto code_r0x0001408bccc1;
            if ((short)uVar25 == 0) {
              do {
                auVar3 = *pauVar23;
                uVar8 = (ushort)(SUB161(auVar3 >> 7,0) & 1) |
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
                pauVar20 = pauVar20 + -0x20;
                pauVar23 = pauVar23 + 1;
              } while (uVar8 == 0xffff);
              uVar26 = ~(uint)uVar8;
              uVar25 = ~(uint)uVar8 & -(uint)uVar8 - 2;
            }
            else {
              uVar26 = uVar25;
              uVar25 = uVar25 - 1 & uVar25;
            }
            iVar2 = 0;
            for (; (uVar26 & 1) == 0; uVar26 = uVar26 >> 1 | 0x80000000) {
              iVar2 = iVar2 + 1;
            }
          } while (*(char *)((longlong)pauVar20 + (-8 - (ulonglong)(uint)(iVar2 << 5))) == '\0');
        }
        unaff_R15 = *(longlong *)*param_2;
        param_4 = (undefined1 (**) [16])(unaff_R15 + 0x10);
        LOCK();
        bVar34 = *(int *)(unaff_R15 + 0x10) == 0;
        if (bVar34) {
          *(int *)(unaff_R15 + 0x10) = 0x3fffffff;
        }
        UNLOCK();
        if (!bVar34) goto code_r0x0001408bce17;
        do {
          puVar6 = puRam000000014362c420;
          if ((*puRam000000014362c420 & 0x7fffffffffffffff) == 0) {
            bVar11 = 0;
            cVar10 = *(char *)(unaff_R15 + 0x18);
          }
          else {
            bVar11 = FUN_1428d86b0();
            bVar11 = bVar11 ^ 1;
            cVar10 = *(char *)(unaff_R15 + 0x18);
          }
          if (cVar10 != '\0') {
            ppauStack_138 = param_4;
            bStack_130 = bVar11;
            FUN_1428d9760(&UNK_142aa03b8,0x2b,&ppauStack_138,&UNK_142aa0488,&UNK_142aa0a40);
                    /* WARNING: Does not return */
            pcVar7 = (code *)invalidInstructionException();
            (*pcVar7)();
          }
          *puVar24 = plStack_50._0_4_;
          uVar36 = *(undefined1 *)(param_1 + 7);
          puVar24 = (undefined4 *)CONCAT71((int7)((ulonglong)puVar24 >> 8),uVar36);
          *(undefined1 *)(param_1 + 7) = 0;
          pauStack_b8 = (undefined1 (*) [16])*param_1;
          puStack_a8 = *pauStack_b8 + param_1[1] + 1;
          auVar3 = *pauStack_b8;
          uVar22 = (ulonglong)uStack_a0 >> 0x10;
          uStack_a0 = CONCAT62((int6)uVar22,
                               ~((ushort)(SUB161(auVar3 >> 7,0) & 1) |
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
                                (ushort)(byte)(auVar3[0xf] >> 7) << 0xf));
          pauStack_b0 = pauStack_b8 + 1;
          uStack_98 = param_1[3];
          ppauStack_70 = param_4;
          bStack_68 = bVar11;
          FUN_141449790(&ppauStack_138,&pauStack_b8);
          puVar31 = uStack_100;
          uStack_108 = param_1[6];
          uStack_100 = (undefined *)CONCAT71(uStack_100._1_7_,uVar36);
          puVar13 = uStack_100;
          uStack_80 = *(undefined4 *)(unaff_R15 + 0xa0);
          uStack_88 = *(undefined4 *)(unaff_R15 + 0x98);
          uStack_84 = *(undefined4 *)(unaff_R15 + 0x9c);
          uVar9 = uStack_80;
          uStack_7c = *(undefined4 *)(unaff_R15 + 0xa4);
          uStack_98 = *(undefined8 *)(unaff_R15 + 0x88);
          uStack_90 = *(undefined8 *)(unaff_R15 + 0x90);
          puStack_a8 = (undefined1 *)*(undefined8 *)(unaff_R15 + 0x78);
          uStack_a0 = *(undefined8 *)(unaff_R15 + 0x80);
          pauStack_b8 = *(undefined1 (**) [16])(unaff_R15 + 0x68);
          pauStack_b0 = (undefined1 (*) [16])*(undefined8 *)(unaff_R15 + 0x70);
          auVar3[8] = bStack_130;
          auVar3._0_8_ = ppauStack_138;
          auVar3._9_7_ = uStack_12f;
          *(undefined4 *)(unaff_R15 + 0x88) = (undefined4)uStack_118;
          *(undefined4 *)(unaff_R15 + 0x8c) = uStack_118._4_4_;
          *(undefined4 *)(unaff_R15 + 0x90) = (undefined4)uStack_110;
          *(undefined4 *)(unaff_R15 + 0x94) = uStack_110._4_4_;
          *(undefined8 *)(unaff_R15 + 0x98) = uStack_108;
          *(undefined1 *)(unaff_R15 + 0xa0) = uVar36;
          uStack_100._1_4_ = SUB84(puVar31,1);
          *(undefined4 *)(unaff_R15 + 0xa1) = uStack_100._1_4_;
          uStack_100._5_2_ = SUB82(puVar31,5);
          *(undefined2 *)(unaff_R15 + 0xa5) = uStack_100._5_2_;
          uStack_100._7_1_ = SUB81(puVar31,7);
          *(undefined1 *)(unaff_R15 + 0xa7) = uStack_100._7_1_;
          *(undefined1 (*) [16])(unaff_R15 + 0x68) = auVar3;
          *(undefined4 *)(unaff_R15 + 0x78) = (undefined4)uStack_128;
          *(undefined4 *)(unaff_R15 + 0x7c) = uStack_128._4_4_;
          *(undefined4 *)(unaff_R15 + 0x80) = (undefined4)uStack_120;
          *(undefined4 *)(unaff_R15 + 0x84) = uStack_120._4_4_;
          bVar34 = (char)uStack_80 != '\x02';
          uStack_100 = puVar13;
          uStack_80 = uVar9;
          if (bVar34) {
            FUN_1400524e0(&pauStack_b8,&uStack_98,0x20,0x10);
          }
          if (((bVar11 == 0) && ((*puVar6 & 0x7fffffffffffffff) != 0)) &&
             (cVar10 = FUN_1428d86b0(), cVar10 == '\0')) {
            *(undefined1 *)(unaff_R15 + 0x18) = 1;
          }
          LOCK();
          iVar2 = *(int *)param_4;
          *(int *)param_4 = *(int *)param_4 + -0x3fffffff;
          UNLOCK();
          if (0x3fffffff < iVar2 + 0xc0000001U) goto code_r0x0001408bcdfe;
code_r0x0001408bccc1:
          while( true ) {
            if (lStack_60 == 0) {
              return;
            }
            plVar19 = (longlong *)func_0x000140c18130();
            if (*plVar19 == 1) break;
            if ((*plVar19 == 0) &&
               (plVar19 = (longlong *)FUN_1428f87d0(plVar19,0), plVar19 != (longlong *)0x0))
            goto code_r0x0001408bcce4;
code_r0x0001408bcdf2:
            FUN_1428d80d0(&UNK_142c3aaa0);
code_r0x0001408bcdfe:
            FUN_1428d8d40(param_4);
          }
          plVar19 = plVar19 + 1;
code_r0x0001408bcce4:
          if (*plVar19 == 0) {
            *plVar19 = -1;
            plStack_50 = plVar19;
            FUN_140c170f0(plVar19 + 1,uStack_58);
            *plStack_50 = *plStack_50 + 1;
            return;
          }
code_r0x0001408bce0b:
          FUN_1428d9250(&UNK_142c3ab70);
code_r0x0001408bce17:
          FUN_1428d8c10(param_4);
        } while( true );
      }
      lVar16 = 0;
      lVar17 = lVar21;
      lVar21 = lVar21 + 0x10;
    }
    else {
      lVar17 = lVar16;
      lVar16 = lVar16 + 0x10;
    }
    *puVar6 = *puVar6 + 1;
    *puVar24 = plStack_50._0_4_;
    pauStack_b0 = *(undefined1 (**) [16])(lVar17 + 8);
    pauStack_b8 = (undefined1 (*) [16])0x0;
    uVar18 = FUN_1405b14b0(param_1 + 4,param_4);
    FUN_140b78550(&ppauStack_138,param_1,uVar18,param_4);
    bStack_68 = 2;
    if (ppauStack_138 != (undefined1 (**) [16])0x3) {
      bStack_68 = (byte)uStack_120;
    }
    ppauStack_70 = uStack_128;
    if (ppauStack_138 != (undefined1 (**) [16])0x3) {
      LOCK();
      *uStack_128 = (undefined1 (*) [16])((*uStack_128)[-1] + 0xf);
      UNLOCK();
      if (*uStack_128 == (undefined1 (*) [16])0x0) {
        FUN_140b5cad0(&ppauStack_70);
      }
      *(undefined1 *)(param_1 + 7) = 1;
    }
    *puVar24 = plStack_50._0_4_;
    *(undefined1 *)(param_1 + 7) = 1;
    unaff_R15 = lVar21;
  } while( true );
}

