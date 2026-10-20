
void FUN_14067d440(undefined8 param_1,longlong param_2)

{
  char *pcVar1;
  undefined4 uVar2;
  code *pcVar3;
  short sVar4;
  uint uVar5;
  uint uVar6;
  longlong *plVar7;
  longlong lVar8;
  undefined8 uVar9;
  undefined8 uVar10;
  undefined8 *puVar11;
  longlong lVar12;
  longlong lVar13;
  ulonglong uVar14;
  ushort uVar15;
  ushort uVar16;
  undefined1 uVar17;
  uint uVar18;
  ulonglong uVar19;
  longlong lVar20;
  longlong lVar21;
  ulonglong uVar22;
  undefined1 auVar23 [16];
  undefined1 auVar24 [16];
  undefined1 auVar25 [16];
  undefined1 auVar26 [16];
  undefined1 auVar27 [16];
  undefined1 auVar28 [16];
  longlong lStack_258;
  ulonglong *puStack_250;
  longlong lStack_248;
  longlong *plStack_188;
  undefined8 uStack_180;
  undefined1 auStack_178 [16];
  undefined8 uStack_168;
  undefined8 uStack_160;
  undefined1 auStack_158 [16];
  undefined8 uStack_148;
  undefined8 uStack_140;
  undefined8 uStack_138;
  undefined8 uStack_130;
  undefined8 uStack_128;
  undefined8 uStack_120;
  undefined *puStack_118;
  undefined8 uStack_110;
  undefined ***pppuStack_108;
  undefined8 uStack_100;
  undefined8 uStack_f8;
  undefined8 uStack_f0;
  undefined8 uStack_e8;
  ulonglong auStack_b8 [2];
  undefined **ppuStack_a8;
  undefined *puStack_a0;
  undefined *puStack_98;
  undefined8 uStack_90;
  undefined8 uStack_88;
  undefined8 uStack_80;
  undefined *puStack_78;
  undefined8 uStack_70;
  code *pcStack_68;
  undefined2 uStack_60;
  undefined1 uStack_5e;
  longlong lStack_58;
  longlong lStack_50;
  longlong lStack_48;
  undefined1 uStack_39;
  undefined8 uStack_38;
  
  uStack_140 = 0x14067d462;
  FUN_140f7d430(&puStack_118);
  uStack_140 = 0x14067d475;
  FUN_140f7e4e0(&puStack_98,&puStack_118);
  uStack_140 = 0x14067d48f;
  FUN_140f801d0(&puStack_118,&puStack_98,0x24,&UNK_142a16ce0);
  uStack_140 = 0x14067d49a;
  FUN_140f80620(param_2,&puStack_118);
  uStack_140 = 0x14067d4b0;
  FUN_140f7d9e0(&puStack_118,0x2c,&UNK_142a16cf8);
  uStack_140 = 0x14067d4bb;
  FUN_140f80620(param_2,&puStack_118);
  uVar14 = 0x8e1bd251962ba784;
  uStack_140 = 0x14067d4d8;
  lVar8 = FUN_1400dd6f0(param_2 + 0x38,0x8e1bd251962ba784,0x6a782374c0e6529b);
  if (((lVar8 != 0) && (uVar14 < *(ulonglong *)(param_2 + 0x218))) &&
     (uVar14 = *(ulonglong *)(*(longlong *)(param_2 + 0x210) + uVar14 * 8), uVar14 != 0)) {
    lVar8 = ~uVar14 * 0x60;
    if (*(longlong *)(*(longlong *)(param_2 + 0x1e0) + 0x20 + lVar8) != 0) {
      lVar8 = *(longlong *)(param_2 + 0x1e0) + lVar8;
      uVar2 = *(undefined4 *)(param_2 + 0x5b0);
      *(undefined4 *)(lVar8 + 0x5c) = uVar2;
      uStack_140 = 0x14067d537;
      FUN_140f81fa0(*(undefined8 *)(lVar8 + 0x10),CONCAT71((uint7)(uint3)((uint)uVar2 >> 8),1));
      uStack_140 = 0x14067d547;
      uVar9 = FUN_140f84330(param_2,0xc0df,0);
      pppuStack_108 = (undefined ***)0x0;
      uStack_100 = 0;
      puStack_118 = (undefined *)0x0;
      uStack_110 = 4;
      uStack_f8 = 8;
      uStack_f0 = 0;
      uStack_e8 = 0x8000000000000000;
      uStack_140 = 0x14067d58f;
      uVar9 = FUN_140f85470(uVar9,&puStack_118);
      uStack_140 = 0x14067d597;
      uVar10 = func_0x000140f85b90();
      uStack_140 = 0x14067d5a2;
      uVar9 = FUN_140f837f0(uVar9,uVar10);
      uStack_100 = 8;
      uStack_f8 = 0;
      puStack_118 = (undefined *)0x66e610699;
      uStack_110 = 0;
      pppuStack_108 = (undefined ***)0x0;
      uStack_140 = 0x14067d5d5;
      uVar9 = FUN_140f82c00(uVar9,&puStack_118);
      uStack_140 = 0x14067d5e5;
      FUN_140f8ee90(&puStack_118);
      uStack_140 = 0x14067d5f0;
      uVar9 = FUN_140f831f0(uVar9,&puStack_118);
      uStack_140 = 0x14067d609;
      puVar11 = (undefined8 *)func_0x000140613c10(0x30,8);
      if (puVar11 != (undefined8 *)0x0) {
        *puVar11 = 1;
        puVar11[1] = 1;
        *(undefined1 *)(puVar11 + 2) = 0;
        puVar11[3] = 0;
        puVar11[4] = 8;
        puVar11[5] = 0;
        uStack_140 = 0x14067d64b;
        uVar9 = FUN_140f83d30(uVar9,puVar11,0);
        pppuStack_108 = (undefined ***)0x0;
        uStack_100 = 0;
        puStack_118 = (undefined *)0x0;
        uStack_110 = 4;
        uStack_f8 = 8;
        uStack_f0 = 0;
        uStack_e8 = 0;
        uStack_140 = 0x14067d67d;
        uVar9 = FUN_140f84e80(uVar9,&puStack_118);
        uStack_140 = 0x14067d68d;
        FUN_140f909f0(&puStack_118);
        uStack_140 = 0x14067d698;
        lVar8 = FUN_140f84880(uVar9,&puStack_118);
        uVar14 = 0xac0a08834819dfe0;
        uStack_140 = 0x14067d6be;
        lVar12 = FUN_1400dd6f0(lVar8 + 0x38,0xac0a08834819dfe0,0x62c72efa87a69547);
        if (((lVar12 == 0) || (*(ulonglong *)(lVar8 + 0x218) <= uVar14)) ||
           ((uVar14 = *(ulonglong *)(*(longlong *)(lVar8 + 0x210) + uVar14 * 8), uVar14 == 0 ||
            (*(longlong *)(*(longlong *)(lVar8 + 0x1e0) + 0x20 + ~uVar14 * 0x60) == 0)))) {
          uStack_140 = 0x14067d6fe;
          FUN_140f93070(lVar8);
        }
        uVar14 = 0x4564a93581af4abf;
        uStack_140 = 0x14067d71a;
        lVar12 = FUN_1400dd6f0(lVar8 + 0x38,0x4564a93581af4abf,0x6ff83a2e3e8570dc);
        if (((lVar12 != 0) && (uVar14 < *(ulonglong *)(lVar8 + 0x218))) &&
           ((uVar14 = *(ulonglong *)(*(longlong *)(lVar8 + 0x210) + uVar14 * 8), uVar14 != 0 &&
            (*(longlong *)(*(longlong *)(lVar8 + 0x1e0) + 0x20 + ~uVar14 * 0x60) != 0)))) {
          return;
        }
        uStack_38 = 0xfffffffffffffffe;
        lVar12 = lVar8 + 0x38;
        lStack_50 = lVar8 + 0x5a8;
        lStack_58 = lVar12;
        if (*(longlong *)(lVar8 + 0x88) != 0) {
          lVar13 = *(longlong *)(lVar8 + 0x70);
          uVar14 = *(ulonglong *)(lVar8 + 0x78);
          uVar19 = uVar14 & 0x6ff83a2e3e8570dc;
          lVar21 = 0;
          while( true ) {
            pcVar1 = (char *)(lVar13 + uVar19);
            auVar27[0] = -(*pcVar1 == UNK_142961700);
            auVar27[1] = -(pcVar1[1] == UNK_142961701);
            auVar27[2] = -(pcVar1[2] == UNK_142961702);
            auVar27[3] = -(pcVar1[3] == UNK_142961703);
            auVar27[4] = -(pcVar1[4] == UNK_142961704);
            auVar27[5] = -(pcVar1[5] == UNK_142961705);
            auVar27[6] = -(pcVar1[6] == UNK_142961706);
            auVar27[7] = -(pcVar1[7] == UNK_142961707);
            auVar27[8] = -(pcVar1[8] == UNK_142961708);
            auVar27[9] = -(pcVar1[9] == UNK_142961709);
            auVar27[10] = -(pcVar1[10] == UNK_14296170a);
            auVar27[0xb] = -(pcVar1[0xb] == UNK_14296170b);
            auVar27[0xc] = -(pcVar1[0xc] == UNK_14296170c);
            auVar27[0xd] = -(pcVar1[0xd] == UNK_14296170d);
            auVar27[0xe] = -(pcVar1[0xe] == UNK_14296170e);
            auVar27[0xf] = -(pcVar1[0xf] == UNK_14296170f);
            uVar16 = (ushort)(SUB161(auVar27 >> 7,0) & 1) |
                     (ushort)(SUB161(auVar27 >> 0xf,0) & 1) << 1 |
                     (ushort)(SUB161(auVar27 >> 0x17,0) & 1) << 2 |
                     (ushort)(SUB161(auVar27 >> 0x1f,0) & 1) << 3 |
                     (ushort)(SUB161(auVar27 >> 0x27,0) & 1) << 4 |
                     (ushort)(SUB161(auVar27 >> 0x2f,0) & 1) << 5 |
                     (ushort)(SUB161(auVar27 >> 0x37,0) & 1) << 6 |
                     (ushort)(SUB161(auVar27 >> 0x3f,0) & 1) << 7 |
                     (ushort)(SUB161(auVar27 >> 0x47,0) & 1) << 8 |
                     (ushort)(SUB161(auVar27 >> 0x4f,0) & 1) << 9 |
                     (ushort)(SUB161(auVar27 >> 0x57,0) & 1) << 10 |
                     (ushort)(SUB161(auVar27 >> 0x5f,0) & 1) << 0xb |
                     (ushort)(SUB161(auVar27 >> 0x67,0) & 1) << 0xc |
                     (ushort)(SUB161(auVar27 >> 0x6f,0) & 1) << 0xd |
                     (ushort)(SUB161(auVar27 >> 0x77,0) & 1) << 0xe |
                     (ushort)(auVar27[0xf] >> 7) << 0xf;
            uVar18 = (uint)uVar16;
            while (uVar16 != 0) {
              uVar5 = 0;
              for (uVar6 = uVar18; (uVar6 & 1) == 0; uVar6 = uVar6 >> 1 | 0x80000000) {
                uVar5 = uVar5 + 1;
              }
              uVar22 = uVar5 + uVar19 & uVar14;
              if ((*(longlong *)(lVar13 + -0x18 + uVar22 * -0x18) == 0x4564a93581af4abf) &&
                 (*(longlong *)(lVar13 + -0x10 + uVar22 * -0x18) == 0x6ff83a2e3e8570dc)) {
                uVar14 = *(ulonglong *)(lVar13 + uVar22 * -0x18 + -8);
                if (*(ulonglong *)(lVar8 + 0x218) <= uVar14) goto code_r0x000140f94734;
                goto code_r0x000140f94706;
              }
              uVar16 = (ushort)(uVar18 - 1) & (ushort)uVar18;
              uVar18 = CONCAT22((short)(uVar18 - 1 >> 0x10),uVar16);
            }
            auVar25[0] = -(*pcVar1 == -1);
            auVar25[1] = -(pcVar1[1] == -1);
            auVar25[2] = -(pcVar1[2] == -1);
            auVar25[3] = -(pcVar1[3] == -1);
            auVar25[4] = -(pcVar1[4] == -1);
            auVar25[5] = -(pcVar1[5] == -1);
            auVar25[6] = -(pcVar1[6] == -1);
            auVar25[7] = -(pcVar1[7] == -1);
            auVar25[8] = -(pcVar1[8] == -1);
            auVar25[9] = -(pcVar1[9] == -1);
            auVar25[10] = -(pcVar1[10] == -1);
            auVar25[0xb] = -(pcVar1[0xb] == -1);
            auVar25[0xc] = -(pcVar1[0xc] == -1);
            auVar25[0xd] = -(pcVar1[0xd] == -1);
            auVar25[0xe] = -(pcVar1[0xe] == -1);
            auVar25[0xf] = -(pcVar1[0xf] == -1);
            if ((((((((((((((((SUB161(auVar25 >> 7,0) & 1) != 0 ||
                             (SUB161(auVar25 >> 0xf,0) & 1) != 0) ||
                            (SUB161(auVar25 >> 0x17,0) & 1) != 0) ||
                           (SUB161(auVar25 >> 0x1f,0) & 1) != 0) ||
                          (SUB161(auVar25 >> 0x27,0) & 1) != 0) ||
                         (SUB161(auVar25 >> 0x2f,0) & 1) != 0) ||
                        (SUB161(auVar25 >> 0x37,0) & 1) != 0) ||
                       (SUB161(auVar25 >> 0x3f,0) & 1) != 0) || (SUB161(auVar25 >> 0x47,0) & 1) != 0
                      ) || (SUB161(auVar25 >> 0x4f,0) & 1) != 0) ||
                    (SUB161(auVar25 >> 0x57,0) & 1) != 0) || (SUB161(auVar25 >> 0x5f,0) & 1) != 0)
                  || (SUB161(auVar25 >> 0x67,0) & 1) != 0) || (SUB161(auVar25 >> 0x6f,0) & 1) != 0)
                || (SUB161(auVar25 >> 0x77,0) & 1) != 0) || auVar25[0xf] < '\0') break;
            lVar20 = uVar19 + lVar21;
            lVar21 = lVar21 + 0x10;
            uVar19 = lVar20 + 0x10U & uVar14;
          }
        }
        lVar13 = *(longlong *)(lVar8 + 0xd8);
        uVar14 = *(ulonglong *)(lVar8 + 0xe0);
        uVar19 = uVar14 & 0x6ff83a2e3e8570dc;
        lVar21 = 0;
        do {
          pcVar1 = (char *)(lVar13 + uVar19);
          auVar28[0] = -(*pcVar1 == UNK_142961700);
          auVar28[1] = -(pcVar1[1] == UNK_142961701);
          auVar28[2] = -(pcVar1[2] == UNK_142961702);
          auVar28[3] = -(pcVar1[3] == UNK_142961703);
          auVar28[4] = -(pcVar1[4] == UNK_142961704);
          auVar28[5] = -(pcVar1[5] == UNK_142961705);
          auVar28[6] = -(pcVar1[6] == UNK_142961706);
          auVar28[7] = -(pcVar1[7] == UNK_142961707);
          auVar28[8] = -(pcVar1[8] == UNK_142961708);
          auVar28[9] = -(pcVar1[9] == UNK_142961709);
          auVar28[10] = -(pcVar1[10] == UNK_14296170a);
          auVar28[0xb] = -(pcVar1[0xb] == UNK_14296170b);
          auVar28[0xc] = -(pcVar1[0xc] == UNK_14296170c);
          auVar28[0xd] = -(pcVar1[0xd] == UNK_14296170d);
          auVar28[0xe] = -(pcVar1[0xe] == UNK_14296170e);
          auVar28[0xf] = -(pcVar1[0xf] == UNK_14296170f);
          uVar16 = (ushort)(SUB161(auVar28 >> 7,0) & 1) |
                   (ushort)(SUB161(auVar28 >> 0xf,0) & 1) << 1 |
                   (ushort)(SUB161(auVar28 >> 0x17,0) & 1) << 2 |
                   (ushort)(SUB161(auVar28 >> 0x1f,0) & 1) << 3 |
                   (ushort)(SUB161(auVar28 >> 0x27,0) & 1) << 4 |
                   (ushort)(SUB161(auVar28 >> 0x2f,0) & 1) << 5 |
                   (ushort)(SUB161(auVar28 >> 0x37,0) & 1) << 6 |
                   (ushort)(SUB161(auVar28 >> 0x3f,0) & 1) << 7 |
                   (ushort)(SUB161(auVar28 >> 0x47,0) & 1) << 8 |
                   (ushort)(SUB161(auVar28 >> 0x4f,0) & 1) << 9 |
                   (ushort)(SUB161(auVar28 >> 0x57,0) & 1) << 10 |
                   (ushort)(SUB161(auVar28 >> 0x5f,0) & 1) << 0xb |
                   (ushort)(SUB161(auVar28 >> 0x67,0) & 1) << 0xc |
                   (ushort)(SUB161(auVar28 >> 0x6f,0) & 1) << 0xd |
                   (ushort)(SUB161(auVar28 >> 0x77,0) & 1) << 0xe |
                   (ushort)(auVar28[0xf] >> 7) << 0xf;
          uVar18 = (uint)uVar16;
          if (uVar16 != 0) {
code_r0x000140f9447d:
            uVar5 = 0;
            for (uVar6 = uVar18; (uVar6 & 1) == 0; uVar6 = uVar6 >> 1 | 0x80000000) {
              uVar5 = uVar5 + 1;
            }
            uVar22 = uVar5 + uVar19 & uVar14;
            lVar20 = uVar22 * -0x88;
            if ((*(longlong *)(lVar13 + -0x88 + lVar20) != 0x4564a93581af4abf) ||
               (*(longlong *)(lVar13 + -0x80 + lVar20) != 0x6ff83a2e3e8570dc))
            goto code_r0x000140f94470;
            lVar21 = ((longlong)(uVar22 * 0x88) >> 3) * -0xf0f0f0f0f0f0f0f;
            uVar14 = lVar21 - 0x10U & uVar14;
            pcVar1 = (char *)(lVar13 + uVar14);
            auVar23[0] = -(*pcVar1 == -1);
            auVar23[1] = -(pcVar1[1] == -1);
            auVar23[2] = -(pcVar1[2] == -1);
            auVar23[3] = -(pcVar1[3] == -1);
            auVar23[4] = -(pcVar1[4] == -1);
            auVar23[5] = -(pcVar1[5] == -1);
            auVar23[6] = -(pcVar1[6] == -1);
            auVar23[7] = -(pcVar1[7] == -1);
            auVar23[8] = -(pcVar1[8] == -1);
            auVar23[9] = -(pcVar1[9] == -1);
            auVar23[10] = -(pcVar1[10] == -1);
            auVar23[0xb] = -(pcVar1[0xb] == -1);
            auVar23[0xc] = -(pcVar1[0xc] == -1);
            auVar23[0xd] = -(pcVar1[0xd] == -1);
            auVar23[0xe] = -(pcVar1[0xe] == -1);
            auVar23[0xf] = -(pcVar1[0xf] == -1);
            uVar16 = (ushort)(SUB161(auVar23 >> 7,0) & 1) |
                     (ushort)(SUB161(auVar23 >> 0xf,0) & 1) << 1 |
                     (ushort)(SUB161(auVar23 >> 0x17,0) & 1) << 2 |
                     (ushort)(SUB161(auVar23 >> 0x1f,0) & 1) << 3 |
                     (ushort)(SUB161(auVar23 >> 0x27,0) & 1) << 4 |
                     (ushort)(SUB161(auVar23 >> 0x2f,0) & 1) << 5 |
                     (ushort)(SUB161(auVar23 >> 0x37,0) & 1) << 6 |
                     (ushort)(SUB161(auVar23 >> 0x3f,0) & 1) << 7 |
                     (ushort)(SUB161(auVar23 >> 0x47,0) & 1) << 8 |
                     (ushort)(SUB161(auVar23 >> 0x4f,0) & 1) << 9 |
                     (ushort)(SUB161(auVar23 >> 0x57,0) & 1) << 10 |
                     (ushort)(SUB161(auVar23 >> 0x5f,0) & 1) << 0xb |
                     (ushort)(SUB161(auVar23 >> 0x67,0) & 1) << 0xc |
                     (ushort)(SUB161(auVar23 >> 0x6f,0) & 1) << 0xd |
                     (ushort)(SUB161(auVar23 >> 0x77,0) & 1) << 0xe |
                     (ushort)(auVar23[0xf] >> 7) << 0xf;
            pcVar1 = (char *)(lVar13 + lVar21);
            auVar24[0] = -(*pcVar1 == -1);
            auVar24[1] = -(pcVar1[1] == -1);
            auVar24[2] = -(pcVar1[2] == -1);
            auVar24[3] = -(pcVar1[3] == -1);
            auVar24[4] = -(pcVar1[4] == -1);
            auVar24[5] = -(pcVar1[5] == -1);
            auVar24[6] = -(pcVar1[6] == -1);
            auVar24[7] = -(pcVar1[7] == -1);
            auVar24[8] = -(pcVar1[8] == -1);
            auVar24[9] = -(pcVar1[9] == -1);
            auVar24[10] = -(pcVar1[10] == -1);
            auVar24[0xb] = -(pcVar1[0xb] == -1);
            auVar24[0xc] = -(pcVar1[0xc] == -1);
            auVar24[0xd] = -(pcVar1[0xd] == -1);
            auVar24[0xe] = -(pcVar1[0xe] == -1);
            auVar24[0xf] = -(pcVar1[0xf] == -1);
            if (uVar16 == 0) {
              uVar15 = 0x10;
            }
            else {
              uVar15 = 0xf;
              if (uVar16 != 0) {
                for (; uVar16 >> uVar15 == 0; uVar15 = uVar15 - 1) {
                }
              }
              uVar15 = uVar15 ^ 0xf;
            }
            lVar20 = lVar20 + lVar13;
            sVar4 = 0;
            for (uVar18 = (ushort)((ushort)(SUB161(auVar24 >> 7,0) & 1) |
                                   (ushort)(SUB161(auVar24 >> 0xf,0) & 1) << 1 |
                                   (ushort)(SUB161(auVar24 >> 0x17,0) & 1) << 2 |
                                   (ushort)(SUB161(auVar24 >> 0x1f,0) & 1) << 3 |
                                   (ushort)(SUB161(auVar24 >> 0x27,0) & 1) << 4 |
                                   (ushort)(SUB161(auVar24 >> 0x2f,0) & 1) << 5 |
                                   (ushort)(SUB161(auVar24 >> 0x37,0) & 1) << 6 |
                                   (ushort)(SUB161(auVar24 >> 0x3f,0) & 1) << 7 |
                                   (ushort)(SUB161(auVar24 >> 0x47,0) & 1) << 8 |
                                   (ushort)(SUB161(auVar24 >> 0x4f,0) & 1) << 9 |
                                   (ushort)(SUB161(auVar24 >> 0x57,0) & 1) << 10 |
                                   (ushort)(SUB161(auVar24 >> 0x5f,0) & 1) << 0xb |
                                   (ushort)(SUB161(auVar24 >> 0x67,0) & 1) << 0xc |
                                   (ushort)(SUB161(auVar24 >> 0x6f,0) & 1) << 0xd |
                                   (ushort)(SUB161(auVar24 >> 0x77,0) & 1) << 0xe |
                                  (ushort)(auVar24[0xf] >> 7) << 0xf) | 0x10000; (uVar18 & 1) == 0;
                uVar18 = uVar18 >> 1 | 0x80000000) {
              sVar4 = sVar4 + 1;
            }
            uVar17 = 0x80;
            if ((ushort)(uVar15 + sVar4) < 0x10) {
              *(longlong *)(lVar8 + 0xe8) = *(longlong *)(lVar8 + 0xe8) + 1;
              uVar17 = 0xff;
            }
            *(undefined1 *)(lVar13 + lVar21) = uVar17;
            *(undefined1 *)(lVar13 + 0x10 + uVar14) = uVar17;
            *(longlong *)(lVar8 + 0xf0) = *(longlong *)(lVar8 + 0xf0) + -1;
            if (*(longlong **)(lVar20 + -0x78) == (longlong *)0x2) goto code_r0x000140f94547;
            uStack_120 = *(undefined8 *)(lVar20 + -0x10);
            puStack_118 = *(undefined **)(lVar20 + -8);
            uStack_130 = *(undefined8 *)(lVar20 + -0x20);
            uStack_128 = *(undefined8 *)(lVar20 + -0x18);
            uStack_140 = *(undefined8 *)(lVar20 + -0x30);
            uStack_138 = *(undefined8 *)(lVar20 + -0x28);
            uStack_180 = *(undefined8 *)(lVar20 + -0x70);
            auStack_178 = *(undefined1 (*) [16])(lVar20 + -0x68);
            uStack_168 = *(undefined8 *)(lVar20 + -0x58);
            uStack_160 = *(undefined8 *)(lVar20 + -0x50);
            auStack_158 = *(undefined1 (*) [16])(lVar20 + -0x48);
            uStack_148 = *(undefined8 *)(lVar20 + -0x38);
            plStack_188 = *(longlong **)(lVar20 + -0x78);
            uVar14 = FUN_140d3fa40(&plStack_188,&lStack_58);
            if (uVar14 < *(ulonglong *)(lVar8 + 0x218)) goto code_r0x000140f94706;
            goto code_r0x000140f94734;
          }
code_r0x000140f94447:
          auVar26[0] = -(*pcVar1 == -1);
          auVar26[1] = -(pcVar1[1] == -1);
          auVar26[2] = -(pcVar1[2] == -1);
          auVar26[3] = -(pcVar1[3] == -1);
          auVar26[4] = -(pcVar1[4] == -1);
          auVar26[5] = -(pcVar1[5] == -1);
          auVar26[6] = -(pcVar1[6] == -1);
          auVar26[7] = -(pcVar1[7] == -1);
          auVar26[8] = -(pcVar1[8] == -1);
          auVar26[9] = -(pcVar1[9] == -1);
          auVar26[10] = -(pcVar1[10] == -1);
          auVar26[0xb] = -(pcVar1[0xb] == -1);
          auVar26[0xc] = -(pcVar1[0xc] == -1);
          auVar26[0xd] = -(pcVar1[0xd] == -1);
          auVar26[0xe] = -(pcVar1[0xe] == -1);
          auVar26[0xf] = -(pcVar1[0xf] == -1);
          if ((((((((((((((((SUB161(auVar26 >> 7,0) & 1) != 0 || (SUB161(auVar26 >> 0xf,0) & 1) != 0
                           ) || (SUB161(auVar26 >> 0x17,0) & 1) != 0) ||
                         (SUB161(auVar26 >> 0x1f,0) & 1) != 0) ||
                        (SUB161(auVar26 >> 0x27,0) & 1) != 0) ||
                       (SUB161(auVar26 >> 0x2f,0) & 1) != 0) || (SUB161(auVar26 >> 0x37,0) & 1) != 0
                      ) || (SUB161(auVar26 >> 0x3f,0) & 1) != 0) ||
                    (SUB161(auVar26 >> 0x47,0) & 1) != 0) || (SUB161(auVar26 >> 0x4f,0) & 1) != 0)
                  || (SUB161(auVar26 >> 0x57,0) & 1) != 0) || (SUB161(auVar26 >> 0x5f,0) & 1) != 0)
                || (SUB161(auVar26 >> 0x67,0) & 1) != 0) || (SUB161(auVar26 >> 0x6f,0) & 1) != 0) ||
              (SUB161(auVar26 >> 0x77,0) & 1) != 0) || auVar26[0xf] < '\0')
          goto code_r0x000140f94547;
          lVar20 = uVar19 + lVar21;
          lVar21 = lVar21 + 0x10;
          uVar19 = lVar20 + 0x10U & uVar14;
        } while( true );
      }
      goto LAB_14067d7f6;
    }
  }
  puStack_98 = &UNK_142bacae0;
  uStack_90 = 0x27;
  puStack_a0 = &UNK_140109e90;
  puStack_118 = &UNK_142bacc10;
  uStack_110 = 2;
  uStack_f8 = 0;
  pppuStack_108 = &ppuStack_a8;
  uStack_100 = 1;
  uStack_140 = 0x14067d7f6;
  ppuStack_a8 = &puStack_98;
  FUN_1428d9390(&puStack_118,&UNK_142bacac8);
LAB_14067d7f6:
  uStack_140 = 0x14067d805;
  FUN_1428d9000(8,0x30);
  pcVar3 = (code *)swi(3);
  (*pcVar3)();
  return;
code_r0x000140f94470:
  uVar16 = (ushort)(uVar18 - 1) & (ushort)uVar18;
  uVar18 = CONCAT22((short)(uVar18 - 1 >> 0x10),uVar16);
  if (uVar16 == 0) goto code_r0x000140f94447;
  goto code_r0x000140f9447d;
code_r0x000140f94547:
  uVar14 = *(ulonglong *)(lVar8 + 0x5a8);
  uVar19 = uVar14 + 1;
  *(ulonglong *)(lVar8 + 0x5a8) = uVar19;
  uStack_80 = 0x8000000000000000;
  puStack_78 = &UNK_142f7b1c2;
  uStack_70 = 0x60;
  uStack_60 = 0x100;
  auStack_b8[0] = 1;
  auStack_b8[1] = 0x4564a93581af4abf;
  ppuStack_a8 = (undefined **)0x6ff83a2e3e8570dc;
  uStack_90 = 8;
  uStack_88 = 0x48;
  pcStack_68 = FUN_140149990;
  uStack_5e = 1;
  puStack_a0 = (undefined *)0x0;
  func_0x000140d3f870(&lStack_258,uVar14,auStack_b8);
  if (*(ulonglong *)(lVar8 + 0x48) < uVar19) {
    uStack_39 = 1;
    FUN_141dbd790(lVar12,uVar19,&UNK_142badea0);
  }
  lVar13 = *(longlong *)(lVar8 + 0x40);
  lStack_48 = lVar13 + uVar14 * 0xd0;
  func_0x000142923c00(&plStack_188,&lStack_258,0xd0);
  if (*(int *)(lVar13 + uVar14 * 0xd0) != 2) {
    FUN_1405f0b50(lStack_48);
  }
  func_0x000142923c00(lStack_48,&plStack_188,0xd0);
  FUN_140082c20(lVar8 + 0x70,0x4564a93581af4abf,0x6ff83a2e3e8570dc,uVar14);
  if (uVar14 < *(ulonglong *)(lVar8 + 0x218)) {
code_r0x000140f94706:
    uVar19 = *(ulonglong *)(*(longlong *)(lVar8 + 0x210) + uVar14 * 8);
    if ((uVar19 != 0) && (*(longlong *)(*(longlong *)(lVar8 + 0x1e0) + 0x20 + ~uVar19 * 0x60) != 0))
    goto code_r0x000140f9480b;
  }
code_r0x000140f94734:
  plStack_188 = (longlong *)0x0;
  uStack_180 = 8;
  auStack_178 = (undefined1  [16])0x0;
  uStack_168 = 0;
  uStack_160 = 8;
  auStack_158 = (undefined1  [16])0x0;
  uStack_148 = 0;
  uVar2 = *(undefined4 *)(lVar8 + 0x5b0);
  FUN_140cf8250(lVar8);
  lStack_248 = lVar8 + 0xf8;
  puStack_250 = auStack_b8;
  lStack_258 = lVar12;
  auStack_b8[0] = uVar14;
  lVar12 = FUN_141dbff60(lVar8 + 0x1d8,uVar14,&lStack_258);
  if (*(longlong *)(lVar12 + 0x20) == 0) {
    if (*(longlong *)(lVar12 + 0x18) == 0) {
      FUN_1428cec40(lVar12,1);
      lVar13 = *(longlong *)(lVar12 + 0x20);
    }
    else {
      lVar13 = 0;
    }
    *(longlong *)(lVar12 + 0x20) = lVar13 + 1;
    func_0x000142923c00(lVar13 * *(longlong *)(lVar12 + 8) + *(longlong *)(lVar12 + 0x10),
                        &plStack_188);
    *(undefined4 *)(lVar12 + 0x58) = uVar2;
  }
  else {
    FUN_140cff670(lVar12,0,&plStack_188);
  }
  *(undefined4 *)(lVar12 + 0x5c) = uVar2;
code_r0x000140f9480b:
  FUN_141dc4490(&plStack_188,lVar8,&UNK_142bade10);
  plVar7 = plStack_188;
  *(undefined4 *)auStack_178._0_8_ = auStack_178._12_4_;
  lVar8 = plStack_188[2];
  if (lVar8 == *plStack_188) {
    FUN_140672ed0(plStack_188,&UNK_142bade28);
  }
  lVar12 = plVar7[1];
  *(undefined **)(lVar12 + lVar8 * 0x18) = &UNK_140565930;
  *(ulonglong *)(lVar12 + 8 + lVar8 * 0x18) = uVar14;
  *(undefined1 *)(lVar12 + 0x10 + lVar8 * 0x18) = 0;
  plVar7[2] = lVar8 + 1;
  return;
}

