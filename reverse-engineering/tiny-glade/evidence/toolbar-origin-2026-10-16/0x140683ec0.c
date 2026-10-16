
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

undefined1 * FUN_140683ec0(longlong param_1,longlong param_2)

{
  char *pcVar1;
  undefined4 *puVar2;
  undefined1 (*pauVar3) [16];
  uint uVar4;
  ulonglong uVar5;
  undefined1 (*pauVar6) [16];
  undefined8 uVar7;
  uint uVar8;
  uint uVar9;
  byte bVar10;
  ushort uVar11;
  byte bVar12;
  ulonglong uVar13;
  longlong lVar14;
  longlong lVar15;
  undefined4 uVar16;
  longlong lVar17;
  byte bVar18;
  char cVar19;
  char cVar21;
  char cVar22;
  char cVar23;
  undefined1 auVar20 [16];
  undefined1 auVar24 [16];
  undefined1 auVar25 [16];
  longlong lStack_220;
  undefined8 uStack_218;
  undefined8 uStack_210;
  undefined8 uStack_208;
  undefined8 uStack_200;
  undefined8 uStack_1f8;
  undefined8 uStack_1f0;
  undefined8 uStack_1e8;
  undefined8 uStack_1e0;
  undefined4 uStack_1d8;
  undefined8 uStack_1c8;
  undefined8 uStack_1c0;
  undefined8 uStack_1b8;
  undefined8 uStack_1b0;
  undefined8 uStack_1a8;
  undefined8 uStack_1a0;
  longlong lStack_198;
  longlong lStack_190;
  undefined8 uStack_188;
  undefined8 uStack_180;
  undefined8 uStack_178;
  undefined8 uStack_170;
  undefined8 uStack_168;
  undefined8 uStack_160;
  undefined8 uStack_158;
  undefined8 uStack_150;
  undefined8 uStack_148;
  undefined8 uStack_140;
  undefined8 uStack_138;
  undefined8 uStack_130;
  undefined8 uStack_128;
  undefined8 uStack_120;
  undefined8 uStack_118;
  undefined4 uStack_110;
  undefined4 uStack_10c;
  undefined4 uStack_108;
  undefined4 uStack_104;
  undefined1 uStack_da;
  byte bStack_d9;
  undefined8 uStack_d8;
  undefined8 uStack_d0;
  undefined8 uStack_c8;
  undefined8 uStack_c0;
  undefined8 uStack_b8;
  undefined8 uStack_b0;
  undefined8 uStack_a8;
  undefined8 uStack_a0;
  undefined8 uStack_98;
  undefined8 uStack_90;
  undefined8 uStack_88;
  undefined8 uStack_80;
  undefined8 uStack_78;
  undefined8 uStack_70;
  
  lStack_220 = param_2;
  lStack_198 = param_2;
  uVar5 = FUN_1405b0d70(param_1 + 0x1c0,&lStack_198);
  lVar14 = *(longlong *)(param_1 + 0x1a0);
  bVar18 = (byte)(uVar5 >> 0x38);
  bVar10 = bVar18 >> 1;
  auVar20 = ZEXT216(CONCAT11(bVar10,bVar10));
  auVar20 = pshuflw(auVar20,auVar20,0);
  lVar15 = 0;
  uVar13 = uVar5;
  do {
    uVar13 = *(ulonglong *)(param_1 + 0x1a8) & uVar13;
    pcVar1 = (char *)(lVar14 + uVar13);
    cVar19 = auVar20[0];
    auVar25[0] = -(*pcVar1 == cVar19);
    cVar21 = auVar20[1];
    auVar25[1] = -(pcVar1[1] == cVar21);
    cVar22 = auVar20[2];
    auVar25[2] = -(pcVar1[2] == cVar22);
    cVar23 = auVar20[3];
    auVar25[3] = -(pcVar1[3] == cVar23);
    auVar25[4] = -(pcVar1[4] == cVar19);
    auVar25[5] = -(pcVar1[5] == cVar21);
    auVar25[6] = -(pcVar1[6] == cVar22);
    auVar25[7] = -(pcVar1[7] == cVar23);
    auVar25[8] = -(pcVar1[8] == cVar19);
    auVar25[9] = -(pcVar1[9] == cVar21);
    auVar25[10] = -(pcVar1[10] == cVar22);
    auVar25[0xb] = -(pcVar1[0xb] == cVar23);
    auVar25[0xc] = -(pcVar1[0xc] == cVar19);
    auVar25[0xd] = -(pcVar1[0xd] == cVar21);
    auVar25[0xe] = -(pcVar1[0xe] == cVar22);
    auVar25[0xf] = -(pcVar1[0xf] == cVar23);
    uVar11 = (ushort)(SUB161(auVar25 >> 7,0) & 1) | (ushort)(SUB161(auVar25 >> 0xf,0) & 1) << 1 |
             (ushort)(SUB161(auVar25 >> 0x17,0) & 1) << 2 |
             (ushort)(SUB161(auVar25 >> 0x1f,0) & 1) << 3 |
             (ushort)(SUB161(auVar25 >> 0x27,0) & 1) << 4 |
             (ushort)(SUB161(auVar25 >> 0x2f,0) & 1) << 5 |
             (ushort)(SUB161(auVar25 >> 0x37,0) & 1) << 6 |
             (ushort)(SUB161(auVar25 >> 0x3f,0) & 1) << 7 |
             (ushort)(SUB161(auVar25 >> 0x47,0) & 1) << 8 |
             (ushort)(SUB161(auVar25 >> 0x4f,0) & 1) << 9 |
             (ushort)(SUB161(auVar25 >> 0x57,0) & 1) << 10 |
             (ushort)(SUB161(auVar25 >> 0x5f,0) & 1) << 0xb |
             (ushort)(SUB161(auVar25 >> 0x67,0) & 1) << 0xc |
             (ushort)(SUB161(auVar25 >> 0x6f,0) & 1) << 0xd |
             (ushort)(SUB161(auVar25 >> 0x77,0) & 1) << 0xe | (ushort)(auVar25[0xf] >> 7) << 0xf;
    uVar8 = (uint)uVar11;
    while (uVar11 != 0) {
      uVar9 = 0;
      for (uVar4 = uVar8; (uVar4 & 1) == 0; uVar4 = uVar4 >> 1 | 0x80000000) {
        uVar9 = uVar9 + 1;
      }
      lVar17 = (uVar9 + uVar13 & *(ulonglong *)(param_1 + 0x1a8)) * -0xc0;
      if (*(longlong *)(lVar14 + -0xc0 + lVar17) == param_2) {
        pauVar6 = (undefined1 (*) [16])(lVar14 + lVar17);
        goto LAB_14068426a;
      }
      uVar11 = (ushort)(uVar8 - 1) & (ushort)uVar8;
      uVar8 = CONCAT22((short)(uVar8 - 1 >> 0x10),uVar11);
    }
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
    if ((((((((((((((((SUB161(auVar24 >> 7,0) & 1) != 0 || (SUB161(auVar24 >> 0xf,0) & 1) != 0) ||
                    (SUB161(auVar24 >> 0x17,0) & 1) != 0) || (SUB161(auVar24 >> 0x1f,0) & 1) != 0)
                  || (SUB161(auVar24 >> 0x27,0) & 1) != 0) || (SUB161(auVar24 >> 0x2f,0) & 1) != 0)
                || (SUB161(auVar24 >> 0x37,0) & 1) != 0) || (SUB161(auVar24 >> 0x3f,0) & 1) != 0) ||
              (SUB161(auVar24 >> 0x47,0) & 1) != 0) || (SUB161(auVar24 >> 0x4f,0) & 1) != 0) ||
            (SUB161(auVar24 >> 0x57,0) & 1) != 0) || (SUB161(auVar24 >> 0x5f,0) & 1) != 0) ||
          (SUB161(auVar24 >> 0x67,0) & 1) != 0) || (SUB161(auVar24 >> 0x6f,0) & 1) != 0) ||
        (SUB161(auVar24 >> 0x77,0) & 1) != 0) || auVar24[0xf] < '\0') {
      if (*(longlong *)(param_1 + 0x1b0) == 0) {
        FUN_1428e7ac0(param_1 + 0x1a0,1,param_1 + 0x1c0,
                      CONCAT71((int7)((ulonglong)(lVar14 + -0xc0) >> 8),1));
      }
      uVar7 = FUN_1405b0d70(param_1 + 0x220,&lStack_220);
      FUN_140fcc750(&lStack_198,param_1 + 0x200,uVar7,&lStack_220);
      if (SBORROW8(0,lStack_190)) {
        uStack_110 = 0;
        uStack_10c = 0;
        uStack_108 = 0;
        uStack_104 = 0;
        uStack_218 = 0;
        uStack_210 = 0;
        uStack_208 = 0;
        uStack_200 = 0;
        uStack_1f8 = 0;
        uStack_1f0 = 0;
        uStack_1e8 = 0;
        uStack_1e0 = 0;
        uStack_1d8 = 0;
        uStack_1c8 = 0;
        uStack_1c0 = 0;
        uStack_1b8 = 0;
        uStack_1b0 = 0;
        bVar10 = 0;
        uStack_da = 0;
        uVar16 = 0;
      }
      else {
        uStack_d8 = uStack_180;
        uStack_d0 = uStack_178;
        uStack_c8 = uStack_170;
        uStack_c0 = uStack_168;
        uStack_b8 = uStack_160;
        uStack_b0 = uStack_158;
        uStack_a8 = uStack_150;
        uStack_a0 = uStack_148;
        uStack_98 = uStack_140;
        uStack_90 = uStack_138;
        uStack_88 = uStack_130;
        uStack_80 = uStack_128;
        uStack_78 = uStack_120;
        uStack_70 = uStack_118;
        bVar10 = bStack_d9 & 0xf3;
        uStack_218 = 0;
        uStack_210 = 0;
        uStack_208 = 0;
        uStack_200 = 0;
        uStack_1f8 = 0;
        uStack_1f0 = 0;
        uStack_1e8 = 0;
        uStack_1e0 = 0;
        uStack_1d8 = 0;
        uStack_1c8 = 0;
        uStack_1c0 = 0;
        uStack_1b8 = 0;
        uStack_1b0 = 0;
        uStack_1a8 = uStack_120;
        uStack_1a0 = uStack_118;
        uVar16 = 1;
        if (lStack_190 != 0) {
          func_0x000140613c20(uStack_188,lStack_190 << 3,8);
        }
      }
      pauVar3 = *(undefined1 (**) [16])(param_1 + 0x1a0);
      uVar13 = *(ulonglong *)(param_1 + 0x1a8);
      uVar5 = uVar5 & uVar13;
      auVar20 = *(undefined1 (*) [16])(*pauVar3 + uVar5);
      uVar11 = (ushort)(SUB161(auVar20 >> 7,0) & 1) | (ushort)(SUB161(auVar20 >> 0xf,0) & 1) << 1 |
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
               (ushort)(SUB161(auVar20 >> 0x77,0) & 1) << 0xe |
               (ushort)(byte)(auVar20[0xf] >> 7) << 0xf;
      uVar8 = (uint)uVar11;
      if (uVar11 == 0) {
        lVar14 = 0x10;
        do {
          uVar5 = uVar5 + lVar14 & uVar13;
          auVar20 = *(undefined1 (*) [16])(*pauVar3 + uVar5);
          lVar14 = lVar14 + 0x10;
          uVar8 = (uint)(ushort)((ushort)(SUB161(auVar20 >> 7,0) & 1) |
                                 (ushort)(SUB161(auVar20 >> 0xf,0) & 1) << 1 |
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
                                 (ushort)(SUB161(auVar20 >> 0x77,0) & 1) << 0xe |
                                (ushort)(byte)(auVar20[0xf] >> 7) << 0xf);
        } while (uVar8 == 0);
      }
      uVar9 = 0;
      for (; (uVar8 & 1) == 0; uVar8 = uVar8 >> 1 | 0x80000000) {
        uVar9 = uVar9 + 1;
      }
      uVar5 = uVar9 + uVar5 & uVar13;
      bVar12 = (*pauVar3)[uVar5];
      if (-1 < (char)bVar12) {
        auVar20 = *pauVar3;
        uVar8 = 0;
        for (uVar9 = (uint)(ushort)((ushort)(SUB161(auVar20 >> 7,0) & 1) |
                                    (ushort)(SUB161(auVar20 >> 0xf,0) & 1) << 1 |
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
                                    (ushort)(SUB161(auVar20 >> 0x77,0) & 1) << 0xe |
                                   (ushort)(byte)(auVar20[0xf] >> 7) << 0xf); (uVar9 & 1) == 0;
            uVar9 = uVar9 >> 1 | 0x80000000) {
          uVar8 = uVar8 + 1;
        }
        uVar5 = (ulonglong)uVar8;
        bVar12 = (*pauVar3)[uVar5];
      }
      bVar18 = bVar18 >> 1;
      (*pauVar3)[uVar5] = bVar18;
      pauVar3[1][uVar5 - 0x10 & uVar13] = bVar18;
      pauVar6 = pauVar3 + uVar5 * -0xc;
      *(longlong *)(param_1 + 0x1b0) = *(longlong *)(param_1 + 0x1b0) - (ulonglong)(bVar12 & 1);
      *(longlong *)pauVar3[uVar5 * -0xc + -0xc] = param_2;
      *(undefined8 *)(pauVar3[uVar5 * -0xc + -0xc] + 8) = 0;
      *(undefined8 *)pauVar3[uVar5 * -0xc + -0xb] = 8;
      *(undefined8 *)(pauVar3[uVar5 * -0xc + -0xb] + 8) = 0;
      *(undefined8 *)pauVar3[uVar5 * -0xc + -10] = uStack_218;
      *(undefined8 *)(pauVar3[uVar5 * -0xc + -10] + 8) = uStack_210;
      *(undefined8 *)pauVar3[uVar5 * -0xc + -9] = uStack_208;
      *(undefined8 *)(pauVar3[uVar5 * -0xc + -9] + 8) = uStack_200;
      *(undefined8 *)pauVar3[uVar5 * -0xc + -8] = uStack_1f8;
      *(undefined8 *)(pauVar3[uVar5 * -0xc + -8] + 8) = uStack_1f0;
      *(undefined8 *)pauVar3[uVar5 * -0xc + -7] = uStack_1e8;
      *(undefined8 *)(pauVar3[uVar5 * -0xc + -7] + 8) = uStack_1e0;
      *(undefined4 *)pauVar3[uVar5 * -0xc + -6] = uStack_1d8;
      *(undefined2 *)(pauVar3[uVar5 * -0xc + -6] + 4) = 0;
      pauVar3[uVar5 * -0xc + -6][6] = 4;
      *(undefined8 *)(pauVar3[uVar5 * -0xc + -6] + 8) = uStack_1c8;
      *(undefined8 *)((longlong)(pauVar3[uVar5 * -0xc + -6] + 8) + 8) = uStack_1c0;
      *(undefined8 *)(pauVar3[uVar5 * -0xc + -5] + 8) = uStack_1b8;
      *(undefined8 *)((longlong)(pauVar3[uVar5 * -0xc + -5] + 8) + 8) = uStack_1b0;
      puVar2 = (undefined4 *)(pauVar3[uVar5 * -0xc + -4] + 8);
      *puVar2 = uStack_110;
      puVar2[1] = uStack_10c;
      puVar2[2] = uStack_108;
      puVar2[3] = uStack_104;
      *(undefined4 *)(pauVar3[uVar5 * -0xc + -3] + 8) = uVar16;
      *(undefined8 *)(pauVar3[uVar5 * -0xc + -3] + 0xc) = uStack_1a8;
      *(undefined8 *)((longlong)(pauVar3[uVar5 * -0xc + -3] + 0xc) + 8) = uStack_1a0;
      uVar7 = _UNK_1429677e8;
      *(undefined8 *)(pauVar3[uVar5 * -0xc + -2] + 0xc) = _DAT_1429677e0;
      *(undefined8 *)((longlong)(pauVar3[uVar5 * -0xc + -2] + 0xc) + 8) = uVar7;
      *(undefined2 *)(pauVar3[uVar5 * -0xc + -1] + 0xc) = 0xd0d;
      pauVar3[uVar5 * -0xc + -1][0xe] = uStack_da;
      pauVar3[uVar5 * -0xc + -1][0xf] = bVar10;
      *(longlong *)(param_1 + 0x1b8) = *(longlong *)(param_1 + 0x1b8) + 1;
LAB_14068426a:
      return pauVar6[-0xc] + 8;
    }
    uVar13 = uVar13 + lVar15 + 0x10;
    lVar15 = lVar15 + 0x10;
  } while( true );
}

