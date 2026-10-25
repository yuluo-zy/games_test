
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void FUN_140a51440(undefined8 param_1,longlong param_2)

{
  int *piVar1;
  undefined1 auVar2 [16];
  ulonglong uVar3;
  code *pcVar4;
  uint uVar5;
  uint uVar6;
  int iVar7;
  int iVar8;
  int iVar9;
  int iVar10;
  longlong lVar11;
  ulonglong uVar12;
  longlong lVar13;
  ulonglong uVar14;
  longlong lVar15;
  uint uVar16;
  ushort uVar17;
  ulonglong uVar18;
  bool bVar19;
  char in_XMM1_Ba;
  char in_XMM1_Bb;
  char in_XMM1_Bc;
  char in_XMM1_Bd;
  char in_XMM1_Be;
  char in_XMM1_Bf;
  char in_XMM1_Bg;
  char in_XMM1_Bh;
  char in_XMM1_Bi;
  char in_XMM1_Bj;
  char in_XMM1_Bk;
  char in_XMM1_Bl;
  char in_XMM1_Bm;
  char in_XMM1_Bn;
  char in_XMM1_Bo;
  char in_XMM1_Bp;
  undefined1 auVar20 [16];
  undefined1 auVar21 [16];
  undefined1 auVar22 [16];
  undefined1 auVar23 [16];
  undefined8 uStack_518;
  undefined8 uStack_510;
  undefined8 uStack_508;
  undefined8 uStack_500;
  undefined8 uStack_4f8;
  undefined8 uStack_4f0;
  undefined8 uStack_4e8;
  undefined8 uStack_4e0;
  undefined8 uStack_4d8;
  undefined8 uStack_4d0;
  undefined1 auStack_408 [38];
  char cStack_3e2;
  longlong lStack_3e0;
  undefined4 auStack_388 [2];
  undefined8 uStack_380;
  undefined8 uStack_378;
  undefined8 uStack_370;
  undefined8 uStack_368;
  undefined8 uStack_360;
  undefined8 uStack_358;
  undefined8 uStack_350;
  undefined8 uStack_348;
  undefined8 uStack_340;
  undefined8 uStack_338;
  undefined8 uStack_330;
  undefined8 uStack_328;
  undefined8 uStack_320;
  undefined8 uStack_318;
  undefined8 uStack_310;
  undefined8 uStack_308;
  undefined8 uStack_300;
  undefined8 uStack_2f8;
  undefined8 uStack_2f0;
  undefined8 uStack_2e8;
  undefined8 uStack_2e0;
  undefined8 uStack_2d8;
  undefined8 uStack_2d0;
  undefined1 uStack_2c8;
  undefined1 uStack_2c7;
  undefined1 uStack_2c6;
  undefined1 uStack_2c5;
  undefined1 uStack_2c4;
  undefined1 uStack_2c3;
  undefined1 uStack_2c2;
  undefined1 uStack_2c1;
  undefined1 uStack_2c0;
  undefined1 uStack_2bf;
  undefined1 uStack_2be;
  undefined1 uStack_2bd;
  undefined1 uStack_2bc;
  undefined1 uStack_2bb;
  undefined1 uStack_2ba;
  undefined1 uStack_2b9;
  undefined8 uStack_2b8;
  undefined8 uStack_2b0;
  undefined8 uStack_2a8;
  undefined8 uStack_2a0;
  undefined8 uStack_298;
  undefined8 uStack_290;
  undefined8 uStack_288;
  undefined8 uStack_280;
  undefined1 auStack_268 [22];
  char cStack_252;
  undefined8 uStack_250;
  undefined8 uStack_208;
  undefined4 uStack_1f8;
  undefined4 uStack_1f4;
  undefined4 uStack_1f0;
  undefined4 uStack_1ec;
  undefined4 uStack_1e8;
  undefined1 uStack_1e4;
  undefined1 uStack_1e3;
  undefined1 uStack_1e2;
  undefined1 uStack_1e1;
  undefined8 uStack_1e0;
  undefined8 uStack_1d8;
  undefined8 uStack_1d0;
  undefined8 uStack_1c8;
  undefined8 uStack_1c0;
  undefined8 uStack_1b8;
  undefined8 uStack_1b0;
  undefined8 uStack_1a8;
  undefined8 uStack_1a0;
  undefined4 uStack_198;
  undefined8 uStack_194;
  undefined4 uStack_18c;
  undefined8 uStack_180;
  undefined8 uStack_178;
  undefined1 uStack_170;
  undefined1 uStack_16f;
  undefined1 uStack_16e;
  undefined1 uStack_16d;
  undefined1 uStack_16c;
  undefined1 uStack_16b;
  undefined1 uStack_16a;
  undefined1 uStack_169;
  undefined1 uStack_168;
  undefined1 uStack_167;
  undefined1 uStack_166;
  undefined1 uStack_165;
  undefined1 uStack_164;
  undefined1 uStack_163;
  undefined1 uStack_162;
  undefined1 uStack_161;
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
  undefined1 auStack_68 [12];
  undefined4 uStack_5c;
  undefined1 uStack_58;
  undefined1 uStack_57;
  undefined1 uStack_56;
  undefined1 uStack_55;
  undefined1 uStack_54;
  undefined1 uStack_53;
  undefined1 uStack_52;
  undefined1 uStack_51;
  undefined1 uStack_50;
  undefined1 uStack_4f;
  undefined1 uStack_4e;
  undefined1 uStack_4d;
  undefined1 uStack_4c;
  undefined1 uStack_4b;
  undefined1 uStack_4a;
  undefined1 uStack_49;
  undefined1 uStack_48;
  undefined7 uStack_47;
  undefined4 uStack_40;
  undefined1 uStack_39;
  undefined8 uStack_38;
  
  uStack_38 = 0xfffffffffffffffe;
  uVar14 = 0xf5fd74e6faf4ed14;
  if (*(longlong *)(param_2 + 0x88) != 0) {
    lVar11 = *(longlong *)(param_2 + 0x70);
    uVar3 = *(ulonglong *)(param_2 + 0x78);
    uVar12 = uVar3 & 0xf5fd74e6faf4ed14;
    lVar15 = 0;
    iVar7 = -(uint)(CONCAT13(in_XMM1_Bd,CONCAT12(in_XMM1_Bc,CONCAT11(in_XMM1_Bb,in_XMM1_Ba))) ==
                   CONCAT13(in_XMM1_Bd,CONCAT12(in_XMM1_Bc,CONCAT11(in_XMM1_Bb,in_XMM1_Ba))));
    in_XMM1_Ba = (char)iVar7;
    in_XMM1_Bb = (char)((uint)iVar7 >> 8);
    in_XMM1_Bc = (char)((uint)iVar7 >> 0x10);
    in_XMM1_Bd = (char)((uint)iVar7 >> 0x18);
    iVar7 = -(uint)(CONCAT13(in_XMM1_Bh,CONCAT12(in_XMM1_Bg,CONCAT11(in_XMM1_Bf,in_XMM1_Be))) ==
                   CONCAT13(in_XMM1_Bh,CONCAT12(in_XMM1_Bg,CONCAT11(in_XMM1_Bf,in_XMM1_Be))));
    in_XMM1_Be = (char)iVar7;
    in_XMM1_Bf = (char)((uint)iVar7 >> 8);
    in_XMM1_Bg = (char)((uint)iVar7 >> 0x10);
    in_XMM1_Bh = (char)((uint)iVar7 >> 0x18);
    iVar7 = -(uint)(CONCAT13(in_XMM1_Bl,CONCAT12(in_XMM1_Bk,CONCAT11(in_XMM1_Bj,in_XMM1_Bi))) ==
                   CONCAT13(in_XMM1_Bl,CONCAT12(in_XMM1_Bk,CONCAT11(in_XMM1_Bj,in_XMM1_Bi))));
    in_XMM1_Bi = (char)iVar7;
    in_XMM1_Bj = (char)((uint)iVar7 >> 8);
    in_XMM1_Bk = (char)((uint)iVar7 >> 0x10);
    in_XMM1_Bl = (char)((uint)iVar7 >> 0x18);
    iVar7 = -(uint)(CONCAT13(in_XMM1_Bp,CONCAT12(in_XMM1_Bo,CONCAT11(in_XMM1_Bn,in_XMM1_Bm))) ==
                   CONCAT13(in_XMM1_Bp,CONCAT12(in_XMM1_Bo,CONCAT11(in_XMM1_Bn,in_XMM1_Bm))));
    in_XMM1_Bm = (char)iVar7;
    in_XMM1_Bn = (char)((uint)iVar7 >> 8);
    in_XMM1_Bo = (char)((uint)iVar7 >> 0x10);
    in_XMM1_Bp = (char)((uint)iVar7 >> 0x18);
    while( true ) {
      auVar2 = *(undefined1 (*) [16])(lVar11 + uVar12);
      auVar22[0] = -(auVar2[0] == UNK_142950520);
      auVar22[1] = -(auVar2[1] == UNK_142950520._1_1_);
      auVar22[2] = -(auVar2[2] == UNK_142950520._2_1_);
      auVar22[3] = -(auVar2[3] == UNK_142950520._3_1_);
      auVar22[4] = -(auVar2[4] == UNK_142950520._4_1_);
      auVar22[5] = -(auVar2[5] == UNK_142950520._5_1_);
      auVar22[6] = -(auVar2[6] == UNK_142950520._6_1_);
      auVar22[7] = -(auVar2[7] == UNK_142950520._7_1_);
      auVar22[8] = -(auVar2[8] == UNK_142950520._8_1_);
      auVar22[9] = -(auVar2[9] == UNK_142950520._9_1_);
      auVar22[10] = -(auVar2[10] == UNK_142950520._10_1_);
      auVar22[0xb] = -(auVar2[0xb] == UNK_142950520._11_1_);
      auVar22[0xc] = -(auVar2[0xc] == UNK_142950520._12_1_);
      auVar22[0xd] = -(auVar2[0xd] == UNK_142950520._13_1_);
      auVar22[0xe] = -(auVar2[0xe] == UNK_142950520._14_1_);
      auVar22[0xf] = -(auVar2[0xf] == UNK_142950520._15_1_);
      uVar17 = (ushort)(SUB161(auVar22 >> 7,0) & 1) | (ushort)(SUB161(auVar22 >> 0xf,0) & 1) << 1 |
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
      uVar16 = (uint)uVar17;
      while (uVar17 != 0) {
        uVar5 = 0;
        for (uVar6 = uVar16; (uVar6 & 1) == 0; uVar6 = uVar6 >> 1 | 0x80000000) {
          uVar5 = uVar5 + 1;
        }
        uVar18 = uVar5 + uVar12 & uVar3;
        if ((*(longlong *)(lVar11 + -0x18 + uVar18 * -0x18) == -0x597a9e4c6987b156) &&
           (*(longlong *)(lVar11 + -0x10 + uVar18 * -0x18) == -0xa028b19050b12ec)) {
          uVar14 = *(ulonglong *)(lVar11 + uVar18 * -0x18 + -8);
          if (uVar14 < *(ulonglong *)(param_2 + 0x218)) goto code_r0x000140a5162c;
          goto code_r0x000140a51aef;
        }
        uVar17 = (ushort)(uVar16 - 1) & (ushort)uVar16;
        uVar16 = CONCAT22((short)(uVar16 - 1 >> 0x10),uVar17);
      }
      auVar20[0] = -(auVar2[0] == in_XMM1_Ba);
      auVar20[1] = -(auVar2[1] == in_XMM1_Bb);
      auVar20[2] = -(auVar2[2] == in_XMM1_Bc);
      auVar20[3] = -(auVar2[3] == in_XMM1_Bd);
      auVar20[4] = -(auVar2[4] == in_XMM1_Be);
      auVar20[5] = -(auVar2[5] == in_XMM1_Bf);
      auVar20[6] = -(auVar2[6] == in_XMM1_Bg);
      auVar20[7] = -(auVar2[7] == in_XMM1_Bh);
      auVar20[8] = -(auVar2[8] == in_XMM1_Bi);
      auVar20[9] = -(auVar2[9] == in_XMM1_Bj);
      auVar20[10] = -(auVar2[10] == in_XMM1_Bk);
      auVar20[0xb] = -(auVar2[0xb] == in_XMM1_Bl);
      auVar20[0xc] = -(auVar2[0xc] == in_XMM1_Bm);
      auVar20[0xd] = -(auVar2[0xd] == in_XMM1_Bn);
      auVar20[0xe] = -(auVar2[0xe] == in_XMM1_Bo);
      auVar20[0xf] = -(auVar2[0xf] == in_XMM1_Bp);
      if ((((((((((((((((SUB161(auVar20 >> 7,0) & 1) != 0 || (SUB161(auVar20 >> 0xf,0) & 1) != 0) ||
                      (SUB161(auVar20 >> 0x17,0) & 1) != 0) || (SUB161(auVar20 >> 0x1f,0) & 1) != 0)
                    || (SUB161(auVar20 >> 0x27,0) & 1) != 0) || (SUB161(auVar20 >> 0x2f,0) & 1) != 0
                   ) || (SUB161(auVar20 >> 0x37,0) & 1) != 0) ||
                 (SUB161(auVar20 >> 0x3f,0) & 1) != 0) || (SUB161(auVar20 >> 0x47,0) & 1) != 0) ||
               (SUB161(auVar20 >> 0x4f,0) & 1) != 0) || (SUB161(auVar20 >> 0x57,0) & 1) != 0) ||
             (SUB161(auVar20 >> 0x5f,0) & 1) != 0) || (SUB161(auVar20 >> 0x67,0) & 1) != 0) ||
           (SUB161(auVar20 >> 0x6f,0) & 1) != 0) || (SUB161(auVar20 >> 0x77,0) & 1) != 0) ||
          auVar20[0xf] < '\0') break;
      lVar13 = uVar12 + lVar15;
      lVar15 = lVar15 + 0x10;
      uVar12 = lVar13 + 0x10U & uVar3;
    }
  }
  uVar16 = *(uint *)(param_2 + 0x90);
  if (uVar16 < 0x3ffffffe) {
    LOCK();
    bVar19 = uVar16 == *(uint *)(param_2 + 0x90);
    if (bVar19) {
      *(uint *)(param_2 + 0x90) = uVar16 + 1;
    }
    UNLOCK();
    if (bVar19) {
      lVar11 = *(longlong *)(param_2 + 0xf0);
      goto joined_r0x000140a51acf;
    }
  }
  FUN_1428d8a30(param_2 + 0x90);
  lVar11 = *(longlong *)(param_2 + 0xf0);
joined_r0x000140a51acf:
  if (lVar11 != 0) {
    lVar11 = *(longlong *)(param_2 + 0xd8);
    uVar3 = *(ulonglong *)(param_2 + 0xe0);
    uVar12 = uVar3 & 0xf5fd74e6faf4ed14;
    lVar15 = 0;
    iVar7 = -(uint)(CONCAT13(in_XMM1_Bd,CONCAT12(in_XMM1_Bc,CONCAT11(in_XMM1_Bb,in_XMM1_Ba))) ==
                   CONCAT13(in_XMM1_Bd,CONCAT12(in_XMM1_Bc,CONCAT11(in_XMM1_Bb,in_XMM1_Ba))));
    iVar8 = -(uint)(CONCAT13(in_XMM1_Bh,CONCAT12(in_XMM1_Bg,CONCAT11(in_XMM1_Bf,in_XMM1_Be))) ==
                   CONCAT13(in_XMM1_Bh,CONCAT12(in_XMM1_Bg,CONCAT11(in_XMM1_Bf,in_XMM1_Be))));
    iVar9 = -(uint)(CONCAT13(in_XMM1_Bl,CONCAT12(in_XMM1_Bk,CONCAT11(in_XMM1_Bj,in_XMM1_Bi))) ==
                   CONCAT13(in_XMM1_Bl,CONCAT12(in_XMM1_Bk,CONCAT11(in_XMM1_Bj,in_XMM1_Bi))));
    iVar10 = -(uint)(CONCAT13(in_XMM1_Bp,CONCAT12(in_XMM1_Bo,CONCAT11(in_XMM1_Bn,in_XMM1_Bm))) ==
                    CONCAT13(in_XMM1_Bp,CONCAT12(in_XMM1_Bo,CONCAT11(in_XMM1_Bn,in_XMM1_Bm))));
    while( true ) {
      auVar2 = *(undefined1 (*) [16])(lVar11 + uVar12);
      auVar23[0] = -(auVar2[0] == UNK_142950520);
      auVar23[1] = -(auVar2[1] == UNK_142950520._1_1_);
      auVar23[2] = -(auVar2[2] == UNK_142950520._2_1_);
      auVar23[3] = -(auVar2[3] == UNK_142950520._3_1_);
      auVar23[4] = -(auVar2[4] == UNK_142950520._4_1_);
      auVar23[5] = -(auVar2[5] == UNK_142950520._5_1_);
      auVar23[6] = -(auVar2[6] == UNK_142950520._6_1_);
      auVar23[7] = -(auVar2[7] == UNK_142950520._7_1_);
      auVar23[8] = -(auVar2[8] == UNK_142950520._8_1_);
      auVar23[9] = -(auVar2[9] == UNK_142950520._9_1_);
      auVar23[10] = -(auVar2[10] == UNK_142950520._10_1_);
      auVar23[0xb] = -(auVar2[0xb] == UNK_142950520._11_1_);
      auVar23[0xc] = -(auVar2[0xc] == UNK_142950520._12_1_);
      auVar23[0xd] = -(auVar2[0xd] == UNK_142950520._13_1_);
      auVar23[0xe] = -(auVar2[0xe] == UNK_142950520._14_1_);
      auVar23[0xf] = -(auVar2[0xf] == UNK_142950520._15_1_);
      uVar17 = (ushort)(SUB161(auVar23 >> 7,0) & 1) | (ushort)(SUB161(auVar23 >> 0xf,0) & 1) << 1 |
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
               (ushort)(SUB161(auVar23 >> 0x77,0) & 1) << 0xe | (ushort)(auVar23[0xf] >> 7) << 0xf;
      uVar16 = (uint)uVar17;
      while (uVar17 != 0) {
        uVar5 = 0;
        for (uVar6 = uVar16; (uVar6 & 1) == 0; uVar6 = uVar6 >> 1 | 0x80000000) {
          uVar5 = uVar5 + 1;
        }
        lVar13 = (uVar5 + uVar12 & uVar3) * -0x88;
        if ((*(longlong *)(lVar11 + -0x88 + lVar13) == -0x597a9e4c6987b156) &&
           (*(longlong *)(lVar11 + -0x80 + lVar13) == -0xa028b19050b12ec)) {
          uVar14 = *(ulonglong *)(lVar11 + lVar13 + -8);
          LOCK();
          piVar1 = (int *)(param_2 + 0x90);
          iVar7 = *piVar1;
          *piVar1 = *piVar1 + -1;
          UNLOCK();
          if (SBORROW4(0,iVar7 - 1U & 0xbfffffff)) goto code_r0x000140a51afb;
          if (uVar14 < *(ulonglong *)(param_2 + 0x218)) goto code_r0x000140a5162c;
          goto code_r0x000140a51aef;
        }
        uVar17 = (ushort)(uVar16 - 1) & (ushort)uVar16;
        uVar16 = CONCAT22((short)(uVar16 - 1 >> 0x10),uVar17);
      }
      auVar21[0] = -(auVar2[0] == (char)iVar7);
      auVar21[1] = -(auVar2[1] == (char)((uint)iVar7 >> 8));
      auVar21[2] = -(auVar2[2] == (char)((uint)iVar7 >> 0x10));
      auVar21[3] = -(auVar2[3] == (char)((uint)iVar7 >> 0x18));
      auVar21[4] = -(auVar2[4] == (char)iVar8);
      auVar21[5] = -(auVar2[5] == (char)((uint)iVar8 >> 8));
      auVar21[6] = -(auVar2[6] == (char)((uint)iVar8 >> 0x10));
      auVar21[7] = -(auVar2[7] == (char)((uint)iVar8 >> 0x18));
      auVar21[8] = -(auVar2[8] == (char)iVar9);
      auVar21[9] = -(auVar2[9] == (char)((uint)iVar9 >> 8));
      auVar21[10] = -(auVar2[10] == (char)((uint)iVar9 >> 0x10));
      auVar21[0xb] = -(auVar2[0xb] == (char)((uint)iVar9 >> 0x18));
      auVar21[0xc] = -(auVar2[0xc] == (char)iVar10);
      auVar21[0xd] = -(auVar2[0xd] == (char)((uint)iVar10 >> 8));
      auVar21[0xe] = -(auVar2[0xe] == (char)((uint)iVar10 >> 0x10));
      auVar21[0xf] = -(auVar2[0xf] == (char)((uint)iVar10 >> 0x18));
      if ((((((((((((((((SUB161(auVar21 >> 7,0) & 1) != 0 || (SUB161(auVar21 >> 0xf,0) & 1) != 0) ||
                      (SUB161(auVar21 >> 0x17,0) & 1) != 0) || (SUB161(auVar21 >> 0x1f,0) & 1) != 0)
                    || (SUB161(auVar21 >> 0x27,0) & 1) != 0) || (SUB161(auVar21 >> 0x2f,0) & 1) != 0
                   ) || (SUB161(auVar21 >> 0x37,0) & 1) != 0) ||
                 (SUB161(auVar21 >> 0x3f,0) & 1) != 0) || (SUB161(auVar21 >> 0x47,0) & 1) != 0) ||
               (SUB161(auVar21 >> 0x4f,0) & 1) != 0) || (SUB161(auVar21 >> 0x57,0) & 1) != 0) ||
             (SUB161(auVar21 >> 0x5f,0) & 1) != 0) || (SUB161(auVar21 >> 0x67,0) & 1) != 0) ||
           (SUB161(auVar21 >> 0x6f,0) & 1) != 0) || (SUB161(auVar21 >> 0x77,0) & 1) != 0) ||
          auVar21[0xf] < '\0') break;
      lVar13 = uVar12 + lVar15;
      lVar15 = lVar15 + 0x10;
      uVar12 = lVar13 + 0x10U & uVar3;
    }
  }
  LOCK();
  piVar1 = (int *)(param_2 + 0x90);
  iVar7 = *piVar1;
  *piVar1 = *piVar1 + -1;
  UNLOCK();
  if (SBORROW4(0,iVar7 - 1U & 0xbfffffff)) {
    FUN_1428d8d40(param_2 + 0x90);
    FUN_1428d9310(&UNK_142ad03a8);
  }
  else {
code_r0x000140a51aef:
    do {
      do {
        do {
          FUN_1428d9310(&UNK_142ad03a8);
code_r0x000140a51afb:
          FUN_1428d8d40(param_2 + 0x90);
        } while (*(ulonglong *)(param_2 + 0x218) <= uVar14);
code_r0x000140a5162c:
        uVar3 = *(ulonglong *)(*(longlong *)(param_2 + 0x210) + uVar14 * 8);
      } while (uVar3 == 0);
      lVar11 = ~uVar3 * 0x60;
    } while (*(longlong *)(*(longlong *)(param_2 + 0x1e0) + 0x20 + lVar11) == 0);
    FUN_14093cf30(&uStack_1f8,*(undefined8 *)(*(longlong *)(param_2 + 0x1e0) + lVar11 + 0x10),8);
    uStack_58 = uStack_1e4;
    uStack_57 = uStack_1e3;
    uStack_56 = uStack_1e2;
    uStack_55 = uStack_1e1;
    uStack_54 = (undefined1)uStack_1e0;
    uStack_53 = uStack_1e0._1_1_;
    uStack_52 = uStack_1e0._2_1_;
    uStack_51 = uStack_1e0._3_1_;
    uStack_50 = uStack_1e0._4_1_;
    uStack_4f = uStack_1e0._5_1_;
    uStack_4e = uStack_1e0._6_1_;
    uStack_4d = uStack_1e0._7_1_;
    uStack_4c = (undefined1)uStack_1d8;
    uStack_4b = uStack_1d8._1_1_;
    uStack_4a = uStack_1d8._2_1_;
    uStack_49 = uStack_1d8._3_1_;
    uStack_5c = uStack_1e8;
    FUN_1416032a0(&uStack_1f8,&UNK_142ad03c0,0x22,&UNK_142ad03e2,0x21,param_2,param_1);
    auStack_388[0] = CONCAT31(auStack_388[0]._1_3_,8);
    uStack_518 = CONCAT44(uStack_518._4_4_,2);
    FUN_140905260(auStack_388,&uStack_1f8,0xb981790202418bc8,0x32,&uStack_518);
    func_0x000142923c00(&uStack_518,&uStack_1f8,400);
    uStack_39 = 1;
    uVar14 = 0xbba451ee6a08a3ff;
    lVar11 = FUN_1400dd6f0(param_2 + 0x38,0xbba451ee6a08a3ff,0x8d008d15febcf29d);
    if (((lVar11 != 0) && (uVar14 < *(ulonglong *)(param_2 + 0x218))) &&
       (uVar14 = *(ulonglong *)(*(longlong *)(param_2 + 0x210) + uVar14 * 8), uVar14 != 0)) {
      lVar11 = ~uVar14 * 0x60;
      if (*(longlong *)(*(longlong *)(param_2 + 0x1e0) + 0x20 + lVar11) != 0) {
        uStack_39 = 1;
        func_0x000140c96d40(&uStack_48,
                            *(undefined8 *)(*(longlong *)(param_2 + 0x1e0) + lVar11 + 0x10));
        if (cStack_3e2 != '\b') {
          uStack_1f0 = uStack_40;
          uStack_1f8 = (undefined4)CONCAT71(uStack_47,uStack_48);
          uStack_1f4 = (undefined4)((uint7)uStack_47 >> 0x18);
          FUN_14096b660(auStack_408,lStack_3e0 + 0xe8,0x4bbdb30b873d3990,&uStack_1f8);
        }
        func_0x000142923c00(auStack_388,&uStack_518,400);
        uStack_518 = CONCAT71(uStack_518._1_7_,9);
        uStack_1f8 = 2;
        FUN_140905260(&uStack_518,auStack_388,0x61b08b5e0470c5f2,0x19,&uStack_1f8);
        func_0x000142923c00(&uStack_1f8,auStack_388,400);
        uStack_48 = 10;
        uStack_518 = CONCAT44(uStack_518._4_4_,2);
        FUN_140905260(&uStack_48,&uStack_1f8,0x297d1e4cdae8f657,0x19,&uStack_518);
        func_0x000142923c00(&uStack_518,&uStack_1f8,400);
        if (cStack_3e2 != '\b') {
          FUN_14096b850(auStack_408,lStack_3e0 + 0xe8,0x418a98441f59a54c,1);
        }
        func_0x000142923c00(&uStack_1f8,&uStack_518,400);
        uStack_48 = 0x11;
        auStack_388[0] = 2;
        FUN_140905260(&uStack_48,&uStack_1f8,0x11651f4dc9841aa4,0x19,auStack_388);
        func_0x000142923c00(auStack_388,&uStack_1f8,400);
        uStack_4d8 = uStack_2e8;
        uStack_4d0 = uStack_2e0;
        uStack_4e8 = uStack_2f8;
        uStack_4e0 = uStack_2f0;
        uStack_4f8 = uStack_308;
        uStack_4f0 = uStack_300;
        uStack_508 = uStack_318;
        uStack_500 = uStack_310;
        uStack_518 = uStack_328;
        uStack_510 = uStack_320;
        uStack_1e8 = (undefined4)uStack_378;
        uStack_1e4 = (undefined1)((ulonglong)uStack_378 >> 0x20);
        uStack_1e3 = (undefined1)((ulonglong)uStack_378 >> 0x28);
        uStack_1e2 = (undefined1)((ulonglong)uStack_378 >> 0x30);
        uStack_1e1 = (undefined1)((ulonglong)uStack_378 >> 0x38);
        uStack_1f8 = auStack_388[0];
        uStack_1f0 = (undefined4)uStack_380;
        uStack_1ec = (undefined4)((ulonglong)uStack_380 >> 0x20);
        uStack_1c8 = uStack_370;
        uStack_1c0 = uStack_368;
        uStack_1b8 = uStack_360;
        uStack_1e0 = uStack_358;
        uStack_1d8 = uStack_350;
        uStack_1d0 = uStack_348;
        uStack_180 = uStack_2d8;
        uStack_178 = uStack_2d0;
        uStack_170 = uStack_2c8;
        uStack_16f = uStack_2c7;
        uStack_16e = uStack_2c6;
        uStack_16d = uStack_2c5;
        uStack_16c = uStack_2c4;
        uStack_16b = uStack_2c3;
        uStack_16a = uStack_2c2;
        uStack_169 = uStack_2c1;
        uStack_168 = uStack_2c0;
        uStack_167 = uStack_2bf;
        uStack_166 = uStack_2be;
        uStack_165 = uStack_2bd;
        uStack_164 = uStack_2bc;
        uStack_163 = uStack_2bb;
        uStack_162 = uStack_2ba;
        uStack_161 = uStack_2b9;
        uStack_160 = uStack_2b8;
        uStack_158 = uStack_2b0;
        uStack_150 = uStack_2a8;
        uStack_148 = uStack_2a0;
        uStack_140 = uStack_298;
        uStack_138 = uStack_290;
        uStack_130 = uStack_288;
        uStack_128 = uStack_280;
        uStack_1a0 = uStack_330;
        uStack_1b0 = uStack_340;
        uStack_1a8 = uStack_338;
        uStack_118 = uStack_208;
        uStack_120 = uStack_250;
        uStack_198 = 1;
        uStack_194 = CONCAT17(uStack_55,
                              CONCAT16(uStack_56,CONCAT15(uStack_57,CONCAT14(uStack_58,uStack_5c))))
        ;
        uStack_18c = 1;
        FUN_1409dd4d0(&uStack_518,&uStack_1f8);
        if (cStack_252 != '\b') {
          FUN_140be24b0(auStack_268);
        }
        FUN_140a3a8f0(param_1,param_2,auStack_68,8,&UNK_142ad0420,7);
        return;
      }
    }
  }
  uStack_39 = 1;
  FUN_1428d9310(&UNK_142ad0408);
                    /* WARNING: Does not return */
  pcVar4 = (code *)invalidInstructionException();
  (*pcVar4)();
}

