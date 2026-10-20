
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

ulonglong FUN_140b22db0(longlong *param_1)

{
  char *pcVar1;
  longlong lVar2;
  longlong lVar3;
  short sVar4;
  uint uVar5;
  uint uVar6;
  ushort uVar7;
  ushort uVar8;
  undefined1 uVar9;
  uint uVar10;
  ulonglong uVar11;
  longlong lVar12;
  longlong lVar13;
  ulonglong uVar14;
  ulonglong uVar15;
  undefined1 auVar16 [16];
  undefined1 auVar17 [16];
  undefined1 auVar18 [16];
  undefined1 auVar19 [16];
  undefined1 auVar20 [16];
  undefined1 auVar21 [16];
  undefined8 uStack_2d8;
  undefined8 uStack_2d0;
  undefined8 uStack_2c8;
  undefined8 uStack_2c0;
  undefined8 uStack_208;
  undefined8 uStack_200;
  undefined8 uStack_1f8;
  undefined8 uStack_1f0;
  longlong lStack_138;
  undefined8 uStack_130;
  undefined8 uStack_128;
  undefined8 uStack_120;
  undefined8 uStack_118;
  undefined8 uStack_110;
  undefined8 uStack_108;
  undefined8 uStack_100;
  undefined8 uStack_f8;
  undefined8 uStack_f0;
  undefined8 uStack_e8;
  undefined8 uStack_e0;
  undefined8 uStack_d8;
  undefined8 uStack_d0;
  undefined8 uStack_c8;
  undefined8 uStack_c0;
  undefined8 uStack_b8;
  undefined8 uStack_b0;
  undefined8 uStack_a8;
  undefined8 uStack_98;
  undefined8 uStack_90;
  undefined8 uStack_88;
  undefined *puStack_80;
  undefined8 uStack_78;
  undefined8 uStack_70;
  undefined2 uStack_68;
  undefined1 uStack_66;
  longlong lStack_60;
  undefined8 uStack_58;
  undefined8 uStack_50;
  undefined8 *puStack_48;
  undefined1 uStack_3a;
  undefined1 uStack_39;
  undefined8 uStack_38;
  
  uStack_38 = 0xfffffffffffffffe;
  lStack_60 = 0;
  uStack_58 = 8;
  uStack_50 = 0;
  lVar2 = *param_1;
  if (*(longlong *)(lVar2 + 0x30) != 0) {
    lVar3 = *(longlong *)(lVar2 + 0x18);
    uVar14 = *(ulonglong *)(lVar2 + 0x20);
    uVar11 = uVar14 & 0xe079552911b8d79f;
    lVar13 = 0;
    while( true ) {
      pcVar1 = (char *)(lVar3 + uVar11);
      auVar20[0] = -(*pcVar1 == DAT_1429506f0);
      auVar20[1] = -(pcVar1[1] == UNK_1429506f1);
      auVar20[2] = -(pcVar1[2] == UNK_1429506f2);
      auVar20[3] = -(pcVar1[3] == UNK_1429506f3);
      auVar20[4] = -(pcVar1[4] == UNK_1429506f4);
      auVar20[5] = -(pcVar1[5] == UNK_1429506f5);
      auVar20[6] = -(pcVar1[6] == UNK_1429506f6);
      auVar20[7] = -(pcVar1[7] == UNK_1429506f7);
      auVar20[8] = -(pcVar1[8] == UNK_1429506f8);
      auVar20[9] = -(pcVar1[9] == UNK_1429506f9);
      auVar20[10] = -(pcVar1[10] == UNK_1429506fa);
      auVar20[0xb] = -(pcVar1[0xb] == UNK_1429506fb);
      auVar20[0xc] = -(pcVar1[0xc] == UNK_1429506fc);
      auVar20[0xd] = -(pcVar1[0xd] == UNK_1429506fd);
      auVar20[0xe] = -(pcVar1[0xe] == UNK_1429506fe);
      auVar20[0xf] = -(pcVar1[0xf] == UNK_1429506ff);
      uVar8 = (ushort)(SUB161(auVar20 >> 7,0) & 1) | (ushort)(SUB161(auVar20 >> 0xf,0) & 1) << 1 |
              (ushort)(SUB161(auVar20 >> 0x17,0) & 1) << 2 |
              (ushort)(SUB161(auVar20 >> 0x1f,0) & 1) << 3 |
              (ushort)(SUB161(auVar20 >> 0x27,0) & 1) << 4 |
              (ushort)(SUB161(auVar20 >> 0x2f,0) & 1) << 5 |
              (ushort)(SUB161(auVar20 >> 0x37,0) & 1) << 6 |
              (ushort)(SUB161(auVar20 >> 0x3f,0) & 1) << 7 |
              (ushort)(SUB161(auVar20 >> 0x47,0) & 1) << 8 |
              (ushort)(SUB161(auVar20 >> 0x4f,0) & 1) << 9 |
              (ushort)(SUB161(auVar20 >> 0x57,0) & 1) << 10 |
              (ushort)(SUB161(auVar20 >> 0x5f,0) & 1) << 0xb |
              (ushort)(SUB161(auVar20 >> 0x67,0) & 1) << 0xc |
              (ushort)(SUB161(auVar20 >> 0x6f,0) & 1) << 0xd |
              (ushort)(SUB161(auVar20 >> 0x77,0) & 1) << 0xe | (ushort)(auVar20[0xf] >> 7) << 0xf;
      uVar10 = (uint)uVar8;
      while (uVar8 != 0) {
        uVar5 = 0;
        for (uVar6 = uVar10; (uVar6 & 1) == 0; uVar6 = uVar6 >> 1 | 0x80000000) {
          uVar5 = uVar5 + 1;
        }
        uVar15 = uVar5 + uVar11 & uVar14;
        if ((*(longlong *)(lVar3 + -0x18 + uVar15 * -0x18) == 0x7ec782209740dd56) &&
           (*(longlong *)(lVar3 + -0x10 + uVar15 * -0x18) == -0x1f86aad6ee472861)) {
          return *(ulonglong *)(lVar3 + uVar15 * -0x18 + -8);
        }
        uVar8 = (ushort)(uVar10 - 1) & (ushort)uVar10;
        uVar10 = CONCAT22((short)(uVar10 - 1 >> 0x10),uVar8);
      }
      auVar18[0] = -(*pcVar1 == -1);
      auVar18[1] = -(pcVar1[1] == -1);
      auVar18[2] = -(pcVar1[2] == -1);
      auVar18[3] = -(pcVar1[3] == -1);
      auVar18[4] = -(pcVar1[4] == -1);
      auVar18[5] = -(pcVar1[5] == -1);
      auVar18[6] = -(pcVar1[6] == -1);
      auVar18[7] = -(pcVar1[7] == -1);
      auVar18[8] = -(pcVar1[8] == -1);
      auVar18[9] = -(pcVar1[9] == -1);
      auVar18[10] = -(pcVar1[10] == -1);
      auVar18[0xb] = -(pcVar1[0xb] == -1);
      auVar18[0xc] = -(pcVar1[0xc] == -1);
      auVar18[0xd] = -(pcVar1[0xd] == -1);
      auVar18[0xe] = -(pcVar1[0xe] == -1);
      auVar18[0xf] = -(pcVar1[0xf] == -1);
      if ((((((((((((((((SUB161(auVar18 >> 7,0) & 1) != 0 || (SUB161(auVar18 >> 0xf,0) & 1) != 0) ||
                      (SUB161(auVar18 >> 0x17,0) & 1) != 0) || (SUB161(auVar18 >> 0x1f,0) & 1) != 0)
                    || (SUB161(auVar18 >> 0x27,0) & 1) != 0) || (SUB161(auVar18 >> 0x2f,0) & 1) != 0
                   ) || (SUB161(auVar18 >> 0x37,0) & 1) != 0) ||
                 (SUB161(auVar18 >> 0x3f,0) & 1) != 0) || (SUB161(auVar18 >> 0x47,0) & 1) != 0) ||
               (SUB161(auVar18 >> 0x4f,0) & 1) != 0) || (SUB161(auVar18 >> 0x57,0) & 1) != 0) ||
             (SUB161(auVar18 >> 0x5f,0) & 1) != 0) || (SUB161(auVar18 >> 0x67,0) & 1) != 0) ||
           (SUB161(auVar18 >> 0x6f,0) & 1) != 0) || (SUB161(auVar18 >> 0x77,0) & 1) != 0) ||
          auVar18[0xf] < '\0') break;
      lVar12 = uVar11 + lVar13;
      lVar13 = lVar13 + 0x10;
      uVar11 = lVar12 + 0x10U & uVar14;
    }
  }
  lVar3 = *(longlong *)(lVar2 + 0x80);
  uVar14 = *(ulonglong *)(lVar2 + 0x88);
  uVar11 = uVar14 & 0xe079552911b8d79f;
  lVar13 = 0;
  do {
    pcVar1 = (char *)(lVar3 + uVar11);
    auVar21[0] = -(*pcVar1 == DAT_1429506f0);
    auVar21[1] = -(pcVar1[1] == UNK_1429506f1);
    auVar21[2] = -(pcVar1[2] == UNK_1429506f2);
    auVar21[3] = -(pcVar1[3] == UNK_1429506f3);
    auVar21[4] = -(pcVar1[4] == UNK_1429506f4);
    auVar21[5] = -(pcVar1[5] == UNK_1429506f5);
    auVar21[6] = -(pcVar1[6] == UNK_1429506f6);
    auVar21[7] = -(pcVar1[7] == UNK_1429506f7);
    auVar21[8] = -(pcVar1[8] == UNK_1429506f8);
    auVar21[9] = -(pcVar1[9] == UNK_1429506f9);
    auVar21[10] = -(pcVar1[10] == UNK_1429506fa);
    auVar21[0xb] = -(pcVar1[0xb] == UNK_1429506fb);
    auVar21[0xc] = -(pcVar1[0xc] == UNK_1429506fc);
    auVar21[0xd] = -(pcVar1[0xd] == UNK_1429506fd);
    auVar21[0xe] = -(pcVar1[0xe] == UNK_1429506fe);
    auVar21[0xf] = -(pcVar1[0xf] == UNK_1429506ff);
    uVar8 = (ushort)(SUB161(auVar21 >> 7,0) & 1) | (ushort)(SUB161(auVar21 >> 0xf,0) & 1) << 1 |
            (ushort)(SUB161(auVar21 >> 0x17,0) & 1) << 2 |
            (ushort)(SUB161(auVar21 >> 0x1f,0) & 1) << 3 |
            (ushort)(SUB161(auVar21 >> 0x27,0) & 1) << 4 |
            (ushort)(SUB161(auVar21 >> 0x2f,0) & 1) << 5 |
            (ushort)(SUB161(auVar21 >> 0x37,0) & 1) << 6 |
            (ushort)(SUB161(auVar21 >> 0x3f,0) & 1) << 7 |
            (ushort)(SUB161(auVar21 >> 0x47,0) & 1) << 8 |
            (ushort)(SUB161(auVar21 >> 0x4f,0) & 1) << 9 |
            (ushort)(SUB161(auVar21 >> 0x57,0) & 1) << 10 |
            (ushort)(SUB161(auVar21 >> 0x5f,0) & 1) << 0xb |
            (ushort)(SUB161(auVar21 >> 0x67,0) & 1) << 0xc |
            (ushort)(SUB161(auVar21 >> 0x6f,0) & 1) << 0xd |
            (ushort)(SUB161(auVar21 >> 0x77,0) & 1) << 0xe | (ushort)(auVar21[0xf] >> 7) << 0xf;
    uVar10 = (uint)uVar8;
    while (uVar8 != 0) {
      uVar5 = 0;
      for (uVar6 = uVar10; (uVar6 & 1) == 0; uVar6 = uVar6 >> 1 | 0x80000000) {
        uVar5 = uVar5 + 1;
      }
      uVar15 = uVar5 + uVar11 & uVar14;
      lVar12 = uVar15 * -0x88;
      if ((*(longlong *)(lVar3 + -0x88 + lVar12) == 0x7ec782209740dd56) &&
         (*(longlong *)(lVar3 + -0x80 + lVar12) == -0x1f86aad6ee472861)) {
        lVar13 = ((longlong)(uVar15 * 0x88) >> 3) * -0xf0f0f0f0f0f0f0f;
        uVar14 = lVar13 - 0x10U & uVar14;
        pcVar1 = (char *)(lVar3 + uVar14);
        auVar16[0] = -(*pcVar1 == -1);
        auVar16[1] = -(pcVar1[1] == -1);
        auVar16[2] = -(pcVar1[2] == -1);
        auVar16[3] = -(pcVar1[3] == -1);
        auVar16[4] = -(pcVar1[4] == -1);
        auVar16[5] = -(pcVar1[5] == -1);
        auVar16[6] = -(pcVar1[6] == -1);
        auVar16[7] = -(pcVar1[7] == -1);
        auVar16[8] = -(pcVar1[8] == -1);
        auVar16[9] = -(pcVar1[9] == -1);
        auVar16[10] = -(pcVar1[10] == -1);
        auVar16[0xb] = -(pcVar1[0xb] == -1);
        auVar16[0xc] = -(pcVar1[0xc] == -1);
        auVar16[0xd] = -(pcVar1[0xd] == -1);
        auVar16[0xe] = -(pcVar1[0xe] == -1);
        auVar16[0xf] = -(pcVar1[0xf] == -1);
        uVar8 = (ushort)(SUB161(auVar16 >> 7,0) & 1) | (ushort)(SUB161(auVar16 >> 0xf,0) & 1) << 1 |
                (ushort)(SUB161(auVar16 >> 0x17,0) & 1) << 2 |
                (ushort)(SUB161(auVar16 >> 0x1f,0) & 1) << 3 |
                (ushort)(SUB161(auVar16 >> 0x27,0) & 1) << 4 |
                (ushort)(SUB161(auVar16 >> 0x2f,0) & 1) << 5 |
                (ushort)(SUB161(auVar16 >> 0x37,0) & 1) << 6 |
                (ushort)(SUB161(auVar16 >> 0x3f,0) & 1) << 7 |
                (ushort)(SUB161(auVar16 >> 0x47,0) & 1) << 8 |
                (ushort)(SUB161(auVar16 >> 0x4f,0) & 1) << 9 |
                (ushort)(SUB161(auVar16 >> 0x57,0) & 1) << 10 |
                (ushort)(SUB161(auVar16 >> 0x5f,0) & 1) << 0xb |
                (ushort)(SUB161(auVar16 >> 0x67,0) & 1) << 0xc |
                (ushort)(SUB161(auVar16 >> 0x6f,0) & 1) << 0xd |
                (ushort)(SUB161(auVar16 >> 0x77,0) & 1) << 0xe | (ushort)(auVar16[0xf] >> 7) << 0xf;
        pcVar1 = (char *)(lVar3 + lVar13);
        auVar17[0] = -(*pcVar1 == -1);
        auVar17[1] = -(pcVar1[1] == -1);
        auVar17[2] = -(pcVar1[2] == -1);
        auVar17[3] = -(pcVar1[3] == -1);
        auVar17[4] = -(pcVar1[4] == -1);
        auVar17[5] = -(pcVar1[5] == -1);
        auVar17[6] = -(pcVar1[6] == -1);
        auVar17[7] = -(pcVar1[7] == -1);
        auVar17[8] = -(pcVar1[8] == -1);
        auVar17[9] = -(pcVar1[9] == -1);
        auVar17[10] = -(pcVar1[10] == -1);
        auVar17[0xb] = -(pcVar1[0xb] == -1);
        auVar17[0xc] = -(pcVar1[0xc] == -1);
        auVar17[0xd] = -(pcVar1[0xd] == -1);
        auVar17[0xe] = -(pcVar1[0xe] == -1);
        auVar17[0xf] = -(pcVar1[0xf] == -1);
        if (uVar8 == 0) {
          uVar7 = 0x10;
        }
        else {
          uVar7 = 0xf;
          if (uVar8 != 0) {
            for (; uVar8 >> uVar7 == 0; uVar7 = uVar7 - 1) {
            }
          }
          uVar7 = uVar7 ^ 0xf;
        }
        lVar12 = lVar12 + lVar3;
        sVar4 = 0;
        for (uVar10 = (ushort)((ushort)(SUB161(auVar17 >> 7,0) & 1) |
                               (ushort)(SUB161(auVar17 >> 0xf,0) & 1) << 1 |
                               (ushort)(SUB161(auVar17 >> 0x17,0) & 1) << 2 |
                               (ushort)(SUB161(auVar17 >> 0x1f,0) & 1) << 3 |
                               (ushort)(SUB161(auVar17 >> 0x27,0) & 1) << 4 |
                               (ushort)(SUB161(auVar17 >> 0x2f,0) & 1) << 5 |
                               (ushort)(SUB161(auVar17 >> 0x37,0) & 1) << 6 |
                               (ushort)(SUB161(auVar17 >> 0x3f,0) & 1) << 7 |
                               (ushort)(SUB161(auVar17 >> 0x47,0) & 1) << 8 |
                               (ushort)(SUB161(auVar17 >> 0x4f,0) & 1) << 9 |
                               (ushort)(SUB161(auVar17 >> 0x57,0) & 1) << 10 |
                               (ushort)(SUB161(auVar17 >> 0x5f,0) & 1) << 0xb |
                               (ushort)(SUB161(auVar17 >> 0x67,0) & 1) << 0xc |
                               (ushort)(SUB161(auVar17 >> 0x6f,0) & 1) << 0xd |
                               (ushort)(SUB161(auVar17 >> 0x77,0) & 1) << 0xe |
                              (ushort)(auVar17[0xf] >> 7) << 0xf) | 0x10000; (uVar10 & 1) == 0;
            uVar10 = uVar10 >> 1 | 0x80000000) {
          sVar4 = sVar4 + 1;
        }
        uVar9 = 0x80;
        if ((ushort)(uVar7 + sVar4) < 0x10) {
          *(longlong *)(lVar2 + 0x90) = *(longlong *)(lVar2 + 0x90) + 1;
          uVar9 = 0xff;
        }
        *(undefined1 *)(lVar3 + lVar13) = uVar9;
        *(undefined1 *)(lVar3 + 0x10 + uVar14) = uVar9;
        *(longlong *)(lVar2 + 0x98) = *(longlong *)(lVar2 + 0x98) + -1;
        lStack_138 = *(longlong *)(lVar12 + -0x78);
        if (lStack_138 != 2) {
          uStack_d0 = *(undefined8 *)(lVar12 + -0x10);
          uStack_c8 = *(undefined8 *)(lVar12 + -8);
          uStack_e0 = *(undefined8 *)(lVar12 + -0x20);
          uStack_d8 = *(undefined8 *)(lVar12 + -0x18);
          uStack_f0 = *(undefined8 *)(lVar12 + -0x30);
          uStack_e8 = *(undefined8 *)(lVar12 + -0x28);
          uStack_130 = *(undefined8 *)(lVar12 + -0x70);
          uStack_128 = *(undefined8 *)(lVar12 + -0x68);
          uStack_120 = *(undefined8 *)(lVar12 + -0x60);
          uStack_118 = *(undefined8 *)(lVar12 + -0x58);
          uStack_110 = *(undefined8 *)(lVar12 + -0x50);
          uStack_108 = *(undefined8 *)(lVar12 + -0x48);
          uStack_100 = *(undefined8 *)(lVar12 + -0x40);
          uStack_f8 = *(undefined8 *)(lVar12 + -0x38);
          uVar14 = FUN_140d3fa40(&lStack_138,param_1);
          return uVar14;
        }
        goto LAB_140b22fe7;
      }
      uVar8 = (ushort)(uVar10 - 1) & (ushort)uVar10;
      uVar10 = CONCAT22((short)(uVar10 - 1 >> 0x10),uVar8);
    }
    auVar19[0] = -(*pcVar1 == -1);
    auVar19[1] = -(pcVar1[1] == -1);
    auVar19[2] = -(pcVar1[2] == -1);
    auVar19[3] = -(pcVar1[3] == -1);
    auVar19[4] = -(pcVar1[4] == -1);
    auVar19[5] = -(pcVar1[5] == -1);
    auVar19[6] = -(pcVar1[6] == -1);
    auVar19[7] = -(pcVar1[7] == -1);
    auVar19[8] = -(pcVar1[8] == -1);
    auVar19[9] = -(pcVar1[9] == -1);
    auVar19[10] = -(pcVar1[10] == -1);
    auVar19[0xb] = -(pcVar1[0xb] == -1);
    auVar19[0xc] = -(pcVar1[0xc] == -1);
    auVar19[0xd] = -(pcVar1[0xd] == -1);
    auVar19[0xe] = -(pcVar1[0xe] == -1);
    auVar19[0xf] = -(pcVar1[0xf] == -1);
    if ((((((((((((((((SUB161(auVar19 >> 7,0) & 1) != 0 || (SUB161(auVar19 >> 0xf,0) & 1) != 0) ||
                    (SUB161(auVar19 >> 0x17,0) & 1) != 0) || (SUB161(auVar19 >> 0x1f,0) & 1) != 0)
                  || (SUB161(auVar19 >> 0x27,0) & 1) != 0) || (SUB161(auVar19 >> 0x2f,0) & 1) != 0)
                || (SUB161(auVar19 >> 0x37,0) & 1) != 0) || (SUB161(auVar19 >> 0x3f,0) & 1) != 0) ||
              (SUB161(auVar19 >> 0x47,0) & 1) != 0) || (SUB161(auVar19 >> 0x4f,0) & 1) != 0) ||
            (SUB161(auVar19 >> 0x57,0) & 1) != 0) || (SUB161(auVar19 >> 0x5f,0) & 1) != 0) ||
          (SUB161(auVar19 >> 0x67,0) & 1) != 0) || (SUB161(auVar19 >> 0x6f,0) & 1) != 0) ||
        (SUB161(auVar19 >> 0x77,0) & 1) != 0) || auVar19[0xf] < '\0') {
LAB_140b22fe7:
      uVar11 = *(ulonglong *)param_1[1];
      uVar14 = uVar11 + 1;
      *(ulonglong *)param_1[1] = uVar14;
      uStack_88 = 0x8000000000000000;
      puStack_80 = &UNK_142c368e4;
      uStack_78 = 0x2a;
      uStack_68 = 0x100;
      uStack_c0 = 1;
      uStack_b8 = 0x7ec782209740dd56;
      uStack_b0 = 0xe079552911b8d79f;
      uStack_98 = 1;
      uStack_90 = 0;
      uStack_70 = 0;
      uStack_66 = 1;
      uStack_a8 = 0;
      func_0x000140d3f870(&uStack_208,uVar11,&uStack_c0);
      if (*(ulonglong *)(lVar2 + 0x10) < uVar14) {
        uStack_39 = 1;
        FUN_1412f4340(lVar2,uVar14,&UNK_142ae9a28);
      }
      lVar3 = *(longlong *)(lVar2 + 8);
      lVar13 = uVar11 * 0xd0;
      puStack_48 = (undefined8 *)(lVar3 + lVar13);
      func_0x000142923c00(&uStack_2d8,&uStack_208,0xd0);
      if (*(int *)(lVar3 + lVar13) != 2) {
        FUN_1405f0b50(puStack_48);
      }
      func_0x000142923c00(puStack_48,&uStack_2d8,0xd0);
      FUN_140082c20(lVar2 + 0x18,0x7ec782209740dd56,0xe079552911b8d79f,uVar11);
      uStack_1f8 = _DAT_142ae9750;
      uStack_1f0 = _UNK_142ae9758;
      uStack_208 = _DAT_142ae9740;
      uStack_200 = _UNK_142ae9748;
      uStack_3a = 1;
      FUN_140f89370(uVar11,param_1,&uStack_208,0,&lStack_60);
      uStack_2c8 = uStack_1f8;
      uStack_2c0 = uStack_1f0;
      uStack_2d8 = uStack_208;
      uStack_2d0 = uStack_200;
      puStack_48 = (undefined8 *)(*(longlong *)(*param_1 + 8) + 0x60 + lVar13);
      FUN_140051340(puStack_48,*(longlong *)(*param_1 + 8) + 0x80 + lVar13,0x20,0x10);
      puStack_48[2] = uStack_2c8;
      puStack_48[3] = uStack_2c0;
      *puStack_48 = uStack_2d8;
      puStack_48[1] = uStack_2d0;
      if (lStack_60 != 0) {
        func_0x000140613c20(uStack_58,lStack_60 << 3,8);
      }
      return uVar11;
    }
    lVar12 = uVar11 + lVar13;
    lVar13 = lVar13 + 0x10;
    uVar11 = lVar12 + 0x10U & uVar14;
  } while( true );
}

