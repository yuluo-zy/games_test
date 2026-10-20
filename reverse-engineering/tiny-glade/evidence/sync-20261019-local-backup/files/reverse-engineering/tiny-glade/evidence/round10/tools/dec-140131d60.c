
longlong FUN_140131d60(longlong param_1,longlong *param_2)

{
  char *pcVar1;
  undefined4 uVar2;
  short sVar3;
  uint uVar4;
  uint uVar5;
  longlong lVar6;
  longlong lVar7;
  ulonglong uVar8;
  ushort uVar9;
  undefined1 uVar10;
  uint uVar11;
  ulonglong uVar12;
  longlong lVar13;
  longlong lVar14;
  ushort uVar15;
  ulonglong uVar16;
  undefined1 auVar17 [16];
  undefined1 auVar18 [16];
  undefined1 auVar19 [16];
  undefined1 auVar20 [16];
  undefined1 auVar21 [16];
  undefined1 auVar22 [16];
  longlong lStack_278;
  ulonglong *puStack_270;
  longlong lStack_268;
  longlong lStack_1a8;
  longlong lStack_1a0;
  longlong lStack_198;
  longlong lStack_190;
  longlong lStack_188;
  longlong lStack_180;
  longlong lStack_178;
  undefined8 uStack_170;
  undefined8 uStack_168;
  undefined8 uStack_160;
  undefined8 uStack_158;
  undefined8 uStack_150;
  undefined8 uStack_148;
  undefined8 uStack_140;
  undefined8 uStack_138;
  ulonglong auStack_d0 [5];
  undefined8 uStack_a8;
  undefined8 uStack_a0;
  undefined8 uStack_98;
  undefined *puStack_90;
  undefined8 uStack_88;
  code *pcStack_80;
  undefined2 uStack_78;
  undefined1 uStack_76;
  longlong lStack_70;
  ulonglong *puStack_68;
  longlong *plStack_60;
  longlong lStack_58;
  undefined1 uStack_4a;
  undefined1 uStack_49;
  undefined8 uStack_48;
  
  uStack_48 = 0xfffffffffffffffe;
  lVar6 = param_1 + 0x38;
  puStack_68 = (ulonglong *)(param_1 + 0x5a8);
  lStack_70 = lVar6;
  plStack_60 = param_2;
  if (*(longlong *)(param_1 + 0x88) != 0) {
    lVar7 = *(longlong *)(param_1 + 0x70);
    uVar8 = *(ulonglong *)(param_1 + 0x78);
    uVar12 = uVar8 & 0x309482ece30741d0;
    lVar14 = 0;
    while( true ) {
      pcVar1 = (char *)(lVar7 + uVar12);
      auVar21[0] = -(*pcVar1 == UNK_1429506a0);
      auVar21[1] = -(pcVar1[1] == UNK_1429506a1);
      auVar21[2] = -(pcVar1[2] == UNK_1429506a2);
      auVar21[3] = -(pcVar1[3] == UNK_1429506a3);
      auVar21[4] = -(pcVar1[4] == UNK_1429506a4);
      auVar21[5] = -(pcVar1[5] == UNK_1429506a5);
      auVar21[6] = -(pcVar1[6] == UNK_1429506a6);
      auVar21[7] = -(pcVar1[7] == UNK_1429506a7);
      auVar21[8] = -(pcVar1[8] == UNK_1429506a8);
      auVar21[9] = -(pcVar1[9] == UNK_1429506a9);
      auVar21[10] = -(pcVar1[10] == UNK_1429506aa);
      auVar21[0xb] = -(pcVar1[0xb] == UNK_1429506ab);
      auVar21[0xc] = -(pcVar1[0xc] == UNK_1429506ac);
      auVar21[0xd] = -(pcVar1[0xd] == UNK_1429506ad);
      auVar21[0xe] = -(pcVar1[0xe] == UNK_1429506ae);
      auVar21[0xf] = -(pcVar1[0xf] == UNK_1429506af);
      uVar9 = (ushort)(SUB161(auVar21 >> 7,0) & 1) | (ushort)(SUB161(auVar21 >> 0xf,0) & 1) << 1 |
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
      uVar11 = (uint)uVar9;
      while (uVar9 != 0) {
        uVar4 = 0;
        for (uVar5 = uVar11; (uVar5 & 1) == 0; uVar5 = uVar5 >> 1 | 0x80000000) {
          uVar4 = uVar4 + 1;
        }
        uVar16 = uVar4 + uVar12 & uVar8;
        if ((*(longlong *)(lVar7 + -0x18 + uVar16 * -0x18) == 0x631e74b3fd7d0c4e) &&
           (*(longlong *)(lVar7 + -0x10 + uVar16 * -0x18) == 0x309482ece30741d0)) {
          uVar8 = *(ulonglong *)(lVar7 + uVar16 * -0x18 + -8);
          goto code_r0x00014013215d;
        }
        uVar9 = (ushort)(uVar11 - 1) & (ushort)uVar11;
        uVar11 = CONCAT22((short)(uVar11 - 1 >> 0x10),uVar9);
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
                    || (SUB161(auVar19 >> 0x27,0) & 1) != 0) || (SUB161(auVar19 >> 0x2f,0) & 1) != 0
                   ) || (SUB161(auVar19 >> 0x37,0) & 1) != 0) ||
                 (SUB161(auVar19 >> 0x3f,0) & 1) != 0) || (SUB161(auVar19 >> 0x47,0) & 1) != 0) ||
               (SUB161(auVar19 >> 0x4f,0) & 1) != 0) || (SUB161(auVar19 >> 0x57,0) & 1) != 0) ||
             (SUB161(auVar19 >> 0x5f,0) & 1) != 0) || (SUB161(auVar19 >> 0x67,0) & 1) != 0) ||
           (SUB161(auVar19 >> 0x6f,0) & 1) != 0) || (SUB161(auVar19 >> 0x77,0) & 1) != 0) ||
          auVar19[0xf] < '\0') break;
      lVar13 = uVar12 + lVar14;
      lVar14 = lVar14 + 0x10;
      uVar12 = lVar13 + 0x10U & uVar8;
    }
  }
  lVar7 = *(longlong *)(param_1 + 0xd8);
  uVar8 = *(ulonglong *)(param_1 + 0xe0);
  uVar12 = uVar8 & 0x309482ece30741d0;
  lVar14 = 0;
  while( true ) {
    pcVar1 = (char *)(lVar7 + uVar12);
    auVar22[0] = -(*pcVar1 == UNK_1429506a0);
    auVar22[1] = -(pcVar1[1] == UNK_1429506a1);
    auVar22[2] = -(pcVar1[2] == UNK_1429506a2);
    auVar22[3] = -(pcVar1[3] == UNK_1429506a3);
    auVar22[4] = -(pcVar1[4] == UNK_1429506a4);
    auVar22[5] = -(pcVar1[5] == UNK_1429506a5);
    auVar22[6] = -(pcVar1[6] == UNK_1429506a6);
    auVar22[7] = -(pcVar1[7] == UNK_1429506a7);
    auVar22[8] = -(pcVar1[8] == UNK_1429506a8);
    auVar22[9] = -(pcVar1[9] == UNK_1429506a9);
    auVar22[10] = -(pcVar1[10] == UNK_1429506aa);
    auVar22[0xb] = -(pcVar1[0xb] == UNK_1429506ab);
    auVar22[0xc] = -(pcVar1[0xc] == UNK_1429506ac);
    auVar22[0xd] = -(pcVar1[0xd] == UNK_1429506ad);
    auVar22[0xe] = -(pcVar1[0xe] == UNK_1429506ae);
    auVar22[0xf] = -(pcVar1[0xf] == UNK_1429506af);
    uVar9 = (ushort)(SUB161(auVar22 >> 7,0) & 1) | (ushort)(SUB161(auVar22 >> 0xf,0) & 1) << 1 |
            (ushort)(SUB161(auVar22 >> 0x17,0) & 1) << 2 |
            (ushort)(SUB161(auVar22 >> 0x1f,0) & 1) << 3 |
            (ushort)(SUB161(auVar22 >> 0x27,0) & 1) << 4 |
            (ushort)(SUB161(auVar22 >> 0x2f,0) & 1) << 5 |
            (ushort)(SUB161(auVar22 >> 0x37,0) & 1) << 6 |
            (ushort)(SUB161(auVar22 >> 0x3f,0) & 1) << 7 |
            (ushort)(SUB161(auVar22 >> 0x47,0) & 1) << 8 |
            (ushort)(SUB161(auVar22 >> 0x4f,0) & 1) << 9 |
            (ushort)(SUB161(auVar22 >> 0x57,0) & 1) << 10 |
            (ushort)(SUB161(auVar22 >> 0x5f,0) & 1) << 0xb |
            (ushort)(SUB161(auVar22 >> 0x67,0) & 1) << 0xc |
            (ushort)(SUB161(auVar22 >> 0x6f,0) & 1) << 0xd |
            (ushort)(SUB161(auVar22 >> 0x77,0) & 1) << 0xe | (ushort)(auVar22[0xf] >> 7) << 0xf;
    uVar11 = (uint)uVar9;
    if (uVar9 != 0) break;
code_r0x000140131e9c:
    auVar20[0] = -(*pcVar1 == -1);
    auVar20[1] = -(pcVar1[1] == -1);
    auVar20[2] = -(pcVar1[2] == -1);
    auVar20[3] = -(pcVar1[3] == -1);
    auVar20[4] = -(pcVar1[4] == -1);
    auVar20[5] = -(pcVar1[5] == -1);
    auVar20[6] = -(pcVar1[6] == -1);
    auVar20[7] = -(pcVar1[7] == -1);
    auVar20[8] = -(pcVar1[8] == -1);
    auVar20[9] = -(pcVar1[9] == -1);
    auVar20[10] = -(pcVar1[10] == -1);
    auVar20[0xb] = -(pcVar1[0xb] == -1);
    auVar20[0xc] = -(pcVar1[0xc] == -1);
    auVar20[0xd] = -(pcVar1[0xd] == -1);
    auVar20[0xe] = -(pcVar1[0xe] == -1);
    auVar20[0xf] = -(pcVar1[0xf] == -1);
    if ((((((((((((((((SUB161(auVar20 >> 7,0) & 1) != 0 || (SUB161(auVar20 >> 0xf,0) & 1) != 0) ||
                    (SUB161(auVar20 >> 0x17,0) & 1) != 0) || (SUB161(auVar20 >> 0x1f,0) & 1) != 0)
                  || (SUB161(auVar20 >> 0x27,0) & 1) != 0) || (SUB161(auVar20 >> 0x2f,0) & 1) != 0)
                || (SUB161(auVar20 >> 0x37,0) & 1) != 0) || (SUB161(auVar20 >> 0x3f,0) & 1) != 0) ||
              (SUB161(auVar20 >> 0x47,0) & 1) != 0) || (SUB161(auVar20 >> 0x4f,0) & 1) != 0) ||
            (SUB161(auVar20 >> 0x57,0) & 1) != 0) || (SUB161(auVar20 >> 0x5f,0) & 1) != 0) ||
          (SUB161(auVar20 >> 0x67,0) & 1) != 0) || (SUB161(auVar20 >> 0x6f,0) & 1) != 0) ||
        (SUB161(auVar20 >> 0x77,0) & 1) != 0) || auVar20[0xf] < '\0') {
code_r0x000140131f9d:
      uVar8 = *puStack_68;
      uVar12 = uVar8 + 1;
      *puStack_68 = uVar12;
      uStack_98 = 0x8000000000000000;
      puStack_90 = &UNK_142956c05;
      uStack_88 = 0x37;
      uStack_78 = 0x100;
      auStack_d0[0] = 1;
      auStack_d0[1] = 0x631e74b3fd7d0c4e;
      auStack_d0[2] = 0x309482ece30741d0;
      uStack_a8 = 8;
      uStack_a0 = 0x38;
      pcStack_80 = FUN_14014b920;
      uStack_76 = 1;
      auStack_d0[3] = 0;
      uStack_49 = 1;
      func_0x000140d3f870(&lStack_278,uVar8,auStack_d0);
      if (*(ulonglong *)(param_1 + 0x48) < uVar12) {
        uStack_4a = 1;
        FUN_1402b5690(lVar6,uVar12,&UNK_142950880);
      }
      lVar7 = *(longlong *)(param_1 + 0x40);
      lStack_58 = lVar7 + uVar8 * 0xd0;
      func_0x000142923c00(&lStack_1a8,&lStack_278,0xd0);
      if (*(int *)(lVar7 + uVar8 * 0xd0) != 2) {
        FUN_1405f0b50(lStack_58);
      }
      func_0x000142923c00(lStack_58,&lStack_1a8,0xd0);
      uStack_49 = 1;
      FUN_140082c20(param_1 + 0x70,0x631e74b3fd7d0c4e,0x309482ece30741d0,uVar8);
code_r0x00014013215d:
      lStack_178 = plStack_60[6];
      lStack_1a8 = *plStack_60;
      lStack_1a0 = plStack_60[1];
      lStack_198 = plStack_60[2];
      lStack_190 = plStack_60[3];
      lStack_188 = plStack_60[4];
      lStack_180 = plStack_60[5];
      uVar2 = *(undefined4 *)(param_1 + 0x5b0);
      uStack_49 = 0;
      FUN_140cf8250(param_1);
      lStack_268 = param_1 + 0xf8;
      puStack_270 = auStack_d0;
      uStack_49 = 0;
      lStack_278 = lVar6;
      auStack_d0[0] = uVar8;
      lVar6 = FUN_140509180(param_1 + 0x1d8,uVar8,&lStack_278);
      if (*(longlong *)(lVar6 + 0x20) == 0) {
        if (*(longlong *)(lVar6 + 0x18) == 0) {
          uStack_49 = 0;
          FUN_1428cec40(lVar6,1);
          lVar7 = *(longlong *)(lVar6 + 0x20);
        }
        else {
          lVar7 = 0;
        }
        *(longlong *)(lVar6 + 0x20) = lVar7 + 1;
        func_0x000142923c00(lVar7 * *(longlong *)(lVar6 + 8) + *(longlong *)(lVar6 + 0x10),
                            &lStack_1a8);
        *(undefined4 *)(lVar6 + 0x58) = uVar2;
      }
      else {
        uStack_49 = 0;
        FUN_140cff670(lVar6,0,&lStack_1a8);
      }
      *(undefined4 *)(lVar6 + 0x5c) = uVar2;
      return param_1;
    }
    lVar13 = uVar12 + lVar14;
    lVar14 = lVar14 + 0x10;
    uVar12 = lVar13 + 0x10U & uVar8;
  }
code_r0x000140131ecd:
  uVar4 = 0;
  for (uVar5 = uVar11; (uVar5 & 1) == 0; uVar5 = uVar5 >> 1 | 0x80000000) {
    uVar4 = uVar4 + 1;
  }
  uVar16 = uVar4 + uVar12 & uVar8;
  lVar13 = uVar16 * -0x88;
  if ((*(longlong *)(lVar7 + -0x88 + lVar13) != 0x631e74b3fd7d0c4e) ||
     (*(longlong *)(lVar7 + -0x80 + lVar13) != 0x309482ece30741d0)) goto code_r0x000140131ec0;
  lVar14 = ((longlong)(uVar16 * 0x88) >> 3) * -0xf0f0f0f0f0f0f0f;
  uVar8 = lVar14 - 0x10U & uVar8;
  pcVar1 = (char *)(lVar7 + uVar8);
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
  uVar9 = (ushort)(SUB161(auVar17 >> 7,0) & 1) | (ushort)(SUB161(auVar17 >> 0xf,0) & 1) << 1 |
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
          (ushort)(SUB161(auVar17 >> 0x77,0) & 1) << 0xe | (ushort)(auVar17[0xf] >> 7) << 0xf;
  pcVar1 = (char *)(lVar7 + lVar14);
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
  if (uVar9 == 0) {
    uVar15 = 0x10;
  }
  else {
    uVar15 = 0xf;
    if (uVar9 != 0) {
      for (; uVar9 >> uVar15 == 0; uVar15 = uVar15 - 1) {
      }
    }
    uVar15 = uVar15 ^ 0xf;
  }
  lVar13 = lVar13 + lVar7;
  sVar3 = 0;
  for (uVar11 = (ushort)((ushort)(SUB161(auVar18 >> 7,0) & 1) |
                         (ushort)(SUB161(auVar18 >> 0xf,0) & 1) << 1 |
                         (ushort)(SUB161(auVar18 >> 0x17,0) & 1) << 2 |
                         (ushort)(SUB161(auVar18 >> 0x1f,0) & 1) << 3 |
                         (ushort)(SUB161(auVar18 >> 0x27,0) & 1) << 4 |
                         (ushort)(SUB161(auVar18 >> 0x2f,0) & 1) << 5 |
                         (ushort)(SUB161(auVar18 >> 0x37,0) & 1) << 6 |
                         (ushort)(SUB161(auVar18 >> 0x3f,0) & 1) << 7 |
                         (ushort)(SUB161(auVar18 >> 0x47,0) & 1) << 8 |
                         (ushort)(SUB161(auVar18 >> 0x4f,0) & 1) << 9 |
                         (ushort)(SUB161(auVar18 >> 0x57,0) & 1) << 10 |
                         (ushort)(SUB161(auVar18 >> 0x5f,0) & 1) << 0xb |
                         (ushort)(SUB161(auVar18 >> 0x67,0) & 1) << 0xc |
                         (ushort)(SUB161(auVar18 >> 0x6f,0) & 1) << 0xd |
                         (ushort)(SUB161(auVar18 >> 0x77,0) & 1) << 0xe |
                        (ushort)(auVar18[0xf] >> 7) << 0xf) | 0x10000; (uVar11 & 1) == 0;
      uVar11 = uVar11 >> 1 | 0x80000000) {
    sVar3 = sVar3 + 1;
  }
  uVar10 = 0x80;
  if ((ushort)(uVar15 + sVar3) < 0x10) {
    *(longlong *)(param_1 + 0xe8) = *(longlong *)(param_1 + 0xe8) + 1;
    uVar10 = 0xff;
  }
  *(undefined1 *)(lVar7 + lVar14) = uVar10;
  *(undefined1 *)(lVar7 + 0x10 + uVar8) = uVar10;
  *(longlong *)(param_1 + 0xf0) = *(longlong *)(param_1 + 0xf0) + -1;
  lStack_1a8 = *(longlong *)(lVar13 + -0x78);
  if (lStack_1a8 != 2) {
    uStack_140 = *(undefined8 *)(lVar13 + -0x10);
    uStack_138 = *(undefined8 *)(lVar13 + -8);
    uStack_150 = *(undefined8 *)(lVar13 + -0x20);
    uStack_148 = *(undefined8 *)(lVar13 + -0x18);
    uStack_160 = *(undefined8 *)(lVar13 + -0x30);
    uStack_158 = *(undefined8 *)(lVar13 + -0x28);
    lStack_1a0 = *(undefined8 *)(lVar13 + -0x70);
    lStack_198 = *(undefined8 *)(lVar13 + -0x68);
    lStack_190 = *(undefined8 *)(lVar13 + -0x60);
    lStack_188 = *(undefined8 *)(lVar13 + -0x58);
    lStack_180 = *(undefined8 *)(lVar13 + -0x50);
    lStack_178 = *(undefined8 *)(lVar13 + -0x48);
    uStack_170 = *(undefined8 *)(lVar13 + -0x40);
    uStack_168 = *(undefined8 *)(lVar13 + -0x38);
    uStack_49 = 1;
    uVar8 = FUN_140d3fa40(&lStack_1a8,&lStack_70);
    goto code_r0x00014013215d;
  }
  goto code_r0x000140131f9d;
code_r0x000140131ec0:
  uVar9 = (ushort)(uVar11 - 1) & (ushort)uVar11;
  uVar11 = CONCAT22((short)(uVar11 - 1 >> 0x10),uVar9);
  if (uVar9 == 0) goto code_r0x000140131e9c;
  goto code_r0x000140131ecd;
}

