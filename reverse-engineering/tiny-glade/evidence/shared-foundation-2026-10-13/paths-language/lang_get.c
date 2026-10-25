
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

undefined8 FUN_141848140(longlong param_1,undefined8 param_2,undefined8 **param_3)

{
  char *pcVar1;
  longlong lVar2;
  code *pcVar3;
  uint uVar4;
  uint uVar5;
  int iVar6;
  int iVar7;
  int iVar8;
  char cVar9;
  char cVar10;
  char cVar11;
  char cVar12;
  char cVar13;
  char cVar14;
  char cVar15;
  char cVar16;
  char cVar17;
  char cVar18;
  char cVar19;
  char cVar20;
  char cVar21;
  char cVar22;
  char cVar23;
  char cVar24;
  undefined8 **ppuVar25;
  char cVar26;
  ushort uVar27;
  int iVar28;
  int iVar29;
  uint uVar30;
  ulonglong uVar31;
  ulonglong uVar32;
  ulonglong uVar33;
  byte *pbVar34;
  undefined8 *puVar35;
  longlong lVar36;
  longlong lVar37;
  undefined8 uVar38;
  undefined8 uVar39;
  ulonglong *puVar40;
  bool bVar41;
  byte bVar42;
  undefined1 auVar43 [16];
  undefined1 auVar44 [16];
  char cVar45;
  char cVar46;
  char cVar47;
  char unaff_XMM7_Ba;
  char unaff_XMM7_Bb;
  char unaff_XMM7_Bc;
  char unaff_XMM7_Bd;
  char unaff_XMM7_Be;
  char unaff_XMM7_Bf;
  char unaff_XMM7_Bg;
  char unaff_XMM7_Bh;
  char unaff_XMM7_Bi;
  char unaff_XMM7_Bj;
  char unaff_XMM7_Bk;
  char unaff_XMM7_Bl;
  char unaff_XMM7_Bm;
  char unaff_XMM7_Bn;
  char unaff_XMM7_Bo;
  char unaff_XMM7_Bp;
  undefined1 auVar48 [16];
  undefined1 auVar49 [16];
  undefined1 auStack_158 [8];
  undefined8 uStack_150;
  undefined8 uStack_148;
  undefined8 **ppuStack_140;
  undefined *puStack_138;
  undefined8 **ppuStack_130;
  undefined8 uStack_128;
  undefined8 uStack_120;
  undefined8 uStack_118;
  undefined8 uStack_110;
  undefined *puStack_108;
  undefined8 uStack_100;
  undefined4 uStack_f8;
  undefined4 uStack_f4;
  undefined *puStack_f0;
  undefined8 uStack_e8;
  undefined8 **ppuStack_e0;
  undefined8 uStack_d8;
  undefined8 uStack_d0;
  longlong lStack_c0;
  undefined8 *puStack_b8;
  undefined *puStack_b0;
  undefined8 uStack_a8;
  undefined8 **ppuStack_a0;
  undefined8 uStack_98;
  undefined8 uStack_90;
  undefined1 uStack_81;
  undefined8 uStack_80;
  
  uStack_80 = 0xfffffffffffffffe;
  uStack_a8 = param_2;
  ppuStack_a0 = param_3;
  lVar36 = uStack_90;
  if (*(longlong *)(param_1 + 0x28) != 0) {
    uStack_98 = param_2;
    uVar31 = FUN_1405b1a70(param_1 + 0x30,param_2,param_3);
    lVar2 = *(longlong *)(param_1 + 0x10);
    uVar33 = *(ulonglong *)(param_1 + 0x18);
    bVar42 = (byte)(uVar31 >> 0x39);
    auVar43 = ZEXT216(CONCAT11(bVar42,bVar42));
    auVar43 = pshuflw(auVar43,auVar43,0);
    cVar26 = auVar43[0];
    cVar45 = auVar43[1];
    cVar46 = auVar43[2];
    cVar47 = auVar43[3];
    lVar37 = 0;
    iVar28 = -(uint)(CONCAT13(unaff_XMM7_Bd,
                              CONCAT12(unaff_XMM7_Bc,CONCAT11(unaff_XMM7_Bb,unaff_XMM7_Ba))) ==
                    CONCAT13(unaff_XMM7_Bd,
                             CONCAT12(unaff_XMM7_Bc,CONCAT11(unaff_XMM7_Bb,unaff_XMM7_Ba))));
    unaff_XMM7_Ba = (char)iVar28;
    unaff_XMM7_Bb = (char)((uint)iVar28 >> 8);
    unaff_XMM7_Bc = (char)((uint)iVar28 >> 0x10);
    unaff_XMM7_Bd = (char)((uint)iVar28 >> 0x18);
    iVar28 = -(uint)(CONCAT13(unaff_XMM7_Bh,
                              CONCAT12(unaff_XMM7_Bg,CONCAT11(unaff_XMM7_Bf,unaff_XMM7_Be))) ==
                    CONCAT13(unaff_XMM7_Bh,
                             CONCAT12(unaff_XMM7_Bg,CONCAT11(unaff_XMM7_Bf,unaff_XMM7_Be))));
    unaff_XMM7_Be = (char)iVar28;
    unaff_XMM7_Bf = (char)((uint)iVar28 >> 8);
    unaff_XMM7_Bg = (char)((uint)iVar28 >> 0x10);
    unaff_XMM7_Bh = (char)((uint)iVar28 >> 0x18);
    iVar28 = -(uint)(CONCAT13(unaff_XMM7_Bl,
                              CONCAT12(unaff_XMM7_Bk,CONCAT11(unaff_XMM7_Bj,unaff_XMM7_Bi))) ==
                    CONCAT13(unaff_XMM7_Bl,
                             CONCAT12(unaff_XMM7_Bk,CONCAT11(unaff_XMM7_Bj,unaff_XMM7_Bi))));
    unaff_XMM7_Bi = (char)iVar28;
    unaff_XMM7_Bj = (char)((uint)iVar28 >> 8);
    unaff_XMM7_Bk = (char)((uint)iVar28 >> 0x10);
    unaff_XMM7_Bl = (char)((uint)iVar28 >> 0x18);
    iVar28 = -(uint)(CONCAT13(unaff_XMM7_Bp,
                              CONCAT12(unaff_XMM7_Bo,CONCAT11(unaff_XMM7_Bn,unaff_XMM7_Bm))) ==
                    CONCAT13(unaff_XMM7_Bp,
                             CONCAT12(unaff_XMM7_Bo,CONCAT11(unaff_XMM7_Bn,unaff_XMM7_Bm))));
    unaff_XMM7_Bm = (char)iVar28;
    unaff_XMM7_Bn = (char)((uint)iVar28 >> 8);
    unaff_XMM7_Bo = (char)((uint)iVar28 >> 0x10);
    unaff_XMM7_Bp = (char)((uint)iVar28 >> 0x18);
    while( true ) {
      uVar31 = uVar33 & uVar31;
      pcVar1 = (char *)(lVar2 + uVar31);
      cVar9 = *pcVar1;
      cVar10 = pcVar1[1];
      cVar11 = pcVar1[2];
      cVar12 = pcVar1[3];
      cVar13 = pcVar1[4];
      cVar14 = pcVar1[5];
      cVar15 = pcVar1[6];
      cVar16 = pcVar1[7];
      cVar17 = pcVar1[8];
      cVar18 = pcVar1[9];
      cVar19 = pcVar1[10];
      cVar20 = pcVar1[0xb];
      cVar21 = pcVar1[0xc];
      cVar22 = pcVar1[0xd];
      cVar23 = pcVar1[0xe];
      cVar24 = pcVar1[0xf];
      auVar43[0] = -(cVar9 == cVar26);
      auVar43[1] = -(cVar10 == cVar45);
      auVar43[2] = -(cVar11 == cVar46);
      auVar43[3] = -(cVar12 == cVar47);
      auVar43[4] = -(cVar13 == cVar26);
      auVar43[5] = -(cVar14 == cVar45);
      auVar43[6] = -(cVar15 == cVar46);
      auVar43[7] = -(cVar16 == cVar47);
      auVar43[8] = -(cVar17 == cVar26);
      auVar43[9] = -(cVar18 == cVar45);
      auVar43[10] = -(cVar19 == cVar46);
      auVar43[0xb] = -(cVar20 == cVar47);
      auVar43[0xc] = -(cVar21 == cVar26);
      auVar43[0xd] = -(cVar22 == cVar45);
      auVar43[0xe] = -(cVar23 == cVar46);
      auVar43[0xf] = -(cVar24 == cVar47);
      uVar27 = (ushort)(SUB161(auVar43 >> 7,0) & 1) | (ushort)(SUB161(auVar43 >> 0xf,0) & 1) << 1 |
               (ushort)(SUB161(auVar43 >> 0x17,0) & 1) << 2 |
               (ushort)(SUB161(auVar43 >> 0x1f,0) & 1) << 3 |
               (ushort)(SUB161(auVar43 >> 0x27,0) & 1) << 4 |
               (ushort)(SUB161(auVar43 >> 0x2f,0) & 1) << 5 |
               (ushort)(SUB161(auVar43 >> 0x37,0) & 1) << 6 |
               (ushort)(SUB161(auVar43 >> 0x3f,0) & 1) << 7 |
               (ushort)(SUB161(auVar43 >> 0x47,0) & 1) << 8 |
               (ushort)(SUB161(auVar43 >> 0x4f,0) & 1) << 9 |
               (ushort)(SUB161(auVar43 >> 0x57,0) & 1) << 10 |
               (ushort)(SUB161(auVar43 >> 0x5f,0) & 1) << 0xb |
               (ushort)(SUB161(auVar43 >> 0x67,0) & 1) << 0xc |
               (ushort)(SUB161(auVar43 >> 0x6f,0) & 1) << 0xd |
               (ushort)(SUB161(auVar43 >> 0x77,0) & 1) << 0xe | (ushort)(auVar43[0xf] >> 7) << 0xf;
      uVar30 = (uint)uVar27;
      param_2 = uStack_98;
      lVar36 = uStack_90;
      while (uStack_90 = lVar37, uStack_98 = param_2, uVar27 != 0) {
        uVar4 = 0;
        for (uVar5 = uVar30; (uVar5 & 1) == 0; uVar5 = uVar5 >> 1 | 0x80000000) {
          uVar4 = uVar4 + 1;
        }
        uVar32 = uVar4 + uVar31 & uVar33;
        if ((*(undefined8 ***)(lVar2 + -0x18 + uVar32 * -0x28) == param_3) &&
           (iVar28 = func_0x000142923bf0(param_2,*(undefined8 *)(lVar2 + uVar32 * -0x28 + -0x20),
                                         param_3), iVar28 == 0)) {
          return *(undefined8 *)(lVar2 + -0x10 + uVar32 * -0x28);
        }
        uVar27 = (ushort)(uVar30 - 1) & (ushort)uVar30;
        uVar30 = CONCAT22((short)(uVar30 - 1 >> 0x10),uVar27);
        lVar37 = uStack_90;
        param_2 = uStack_98;
        lVar36 = uStack_90;
      }
      auVar48[0] = -(cVar9 == unaff_XMM7_Ba);
      auVar48[1] = -(cVar10 == unaff_XMM7_Bb);
      auVar48[2] = -(cVar11 == unaff_XMM7_Bc);
      auVar48[3] = -(cVar12 == unaff_XMM7_Bd);
      auVar48[4] = -(cVar13 == unaff_XMM7_Be);
      auVar48[5] = -(cVar14 == unaff_XMM7_Bf);
      auVar48[6] = -(cVar15 == unaff_XMM7_Bg);
      auVar48[7] = -(cVar16 == unaff_XMM7_Bh);
      auVar48[8] = -(cVar17 == unaff_XMM7_Bi);
      auVar48[9] = -(cVar18 == unaff_XMM7_Bj);
      auVar48[10] = -(cVar19 == unaff_XMM7_Bk);
      auVar48[0xb] = -(cVar20 == unaff_XMM7_Bl);
      auVar48[0xc] = -(cVar21 == unaff_XMM7_Bm);
      auVar48[0xd] = -(cVar22 == unaff_XMM7_Bn);
      auVar48[0xe] = -(cVar23 == unaff_XMM7_Bo);
      auVar48[0xf] = -(cVar24 == unaff_XMM7_Bp);
      if ((((((((((((((((SUB161(auVar48 >> 7,0) & 1) != 0 || (SUB161(auVar48 >> 0xf,0) & 1) != 0) ||
                      (SUB161(auVar48 >> 0x17,0) & 1) != 0) || (SUB161(auVar48 >> 0x1f,0) & 1) != 0)
                    || (SUB161(auVar48 >> 0x27,0) & 1) != 0) || (SUB161(auVar48 >> 0x2f,0) & 1) != 0
                   ) || (SUB161(auVar48 >> 0x37,0) & 1) != 0) ||
                 (SUB161(auVar48 >> 0x3f,0) & 1) != 0) || (SUB161(auVar48 >> 0x47,0) & 1) != 0) ||
               (SUB161(auVar48 >> 0x4f,0) & 1) != 0) || (SUB161(auVar48 >> 0x57,0) & 1) != 0) ||
             (SUB161(auVar48 >> 0x5f,0) & 1) != 0) || (SUB161(auVar48 >> 0x67,0) & 1) != 0) ||
           (SUB161(auVar48 >> 0x6f,0) & 1) != 0) || (SUB161(auVar48 >> 0x77,0) & 1) != 0) ||
          auVar48[0xf] < '\0') break;
      uVar31 = uVar31 + uStack_90 + 0x10;
      lVar37 = uStack_90 + 0x10;
      uStack_90 = lVar36;
    }
  }
  uStack_90 = lVar36;
  LOCK();
  bVar41 = DAT_143639608 == '\0';
  if (bVar41) {
    DAT_143639608 = '\x01';
  }
  UNLOCK();
  if (!bVar41) {
    FUN_1428d8750(&DAT_143639608);
  }
  puVar40 = _DAT_14362c420;
  if ((*_DAT_14362c420 & 0x7fffffffffffffff) == 0) {
    uVar30 = 0;
    lVar36 = _DAT_143639610;
  }
  else {
    uVar30 = FUN_1428d86b0();
    uVar30 = uVar30 ^ 1;
    lVar36 = _DAT_143639610;
  }
  _DAT_143639610 = lVar36;
  if (DAT_143639609 == '\0') {
    uVar33 = (ulonglong)uStack_90 >> 0x20;
    uStack_90 = CONCAT44((int)uVar33,uVar30);
    if (lVar36 == 0) {
      pbVar34 = (byte *)FUN_140d4d740();
      if ((*pbVar34 & 1) == 0) {
        _DAT_143639630 = FUN_140d64e00();
        pbVar34[0] = 1;
        pbVar34[1] = 0;
        pbVar34[2] = 0;
        pbVar34[3] = 0;
        pbVar34[4] = 0;
        pbVar34[5] = 0;
        pbVar34[6] = 0;
        pbVar34[7] = 0;
        *(longlong *)(pbVar34 + 8) = _DAT_143639630;
        *(undefined8 *)(pbVar34 + 0x10) = param_2;
      }
      else {
        _DAT_143639630 = *(longlong *)(pbVar34 + 8);
        param_2 = *(undefined8 *)(pbVar34 + 0x10);
      }
      *(longlong *)(pbVar34 + 8) = _DAT_143639630 + 1;
      _DAT_143639620 = _DAT_142cf1af0;
      _DAT_143639628 = _UNK_142cf1af8;
      _DAT_143639610 = _DAT_142cf1ae0;
      _DAT_143639618 = _UNK_142cf1ae8;
      _DAT_143639638 = param_2;
      lVar2 = lStack_c0;
    }
    else {
      lVar2 = lStack_c0;
      if (_DAT_143639628 != 0) {
        uStack_98 = param_2;
        uVar33 = FUN_1405b1a70(&DAT_143639630,param_2,param_3);
        uVar31 = _DAT_143639618;
        bVar42 = (byte)(uVar33 >> 0x39);
        auVar43 = ZEXT216(CONCAT11(bVar42,bVar42));
        auVar43 = pshuflw(auVar43,auVar43,0);
        cVar26 = auVar43[0];
        cVar45 = auVar43[1];
        cVar46 = auVar43[2];
        cVar47 = auVar43[3];
        lVar37 = 0;
        iVar28 = -(uint)(CONCAT13(unaff_XMM7_Bd,
                                  CONCAT12(unaff_XMM7_Bc,CONCAT11(unaff_XMM7_Bb,unaff_XMM7_Ba))) ==
                        CONCAT13(unaff_XMM7_Bd,
                                 CONCAT12(unaff_XMM7_Bc,CONCAT11(unaff_XMM7_Bb,unaff_XMM7_Ba))));
        iVar6 = -(uint)(CONCAT13(unaff_XMM7_Bh,
                                 CONCAT12(unaff_XMM7_Bg,CONCAT11(unaff_XMM7_Bf,unaff_XMM7_Be))) ==
                       CONCAT13(unaff_XMM7_Bh,
                                CONCAT12(unaff_XMM7_Bg,CONCAT11(unaff_XMM7_Bf,unaff_XMM7_Be))));
        iVar7 = -(uint)(CONCAT13(unaff_XMM7_Bl,
                                 CONCAT12(unaff_XMM7_Bk,CONCAT11(unaff_XMM7_Bj,unaff_XMM7_Bi))) ==
                       CONCAT13(unaff_XMM7_Bl,
                                CONCAT12(unaff_XMM7_Bk,CONCAT11(unaff_XMM7_Bj,unaff_XMM7_Bi))));
        iVar8 = -(uint)(CONCAT13(unaff_XMM7_Bp,
                                 CONCAT12(unaff_XMM7_Bo,CONCAT11(unaff_XMM7_Bn,unaff_XMM7_Bm))) ==
                       CONCAT13(unaff_XMM7_Bp,
                                CONCAT12(unaff_XMM7_Bo,CONCAT11(unaff_XMM7_Bn,unaff_XMM7_Bm))));
        while( true ) {
          uVar33 = uVar31 & uVar33;
          pcVar1 = (char *)(lVar36 + uVar33);
          cVar9 = *pcVar1;
          cVar10 = pcVar1[1];
          cVar11 = pcVar1[2];
          cVar12 = pcVar1[3];
          cVar13 = pcVar1[4];
          cVar14 = pcVar1[5];
          cVar15 = pcVar1[6];
          cVar16 = pcVar1[7];
          cVar17 = pcVar1[8];
          cVar18 = pcVar1[9];
          cVar19 = pcVar1[10];
          cVar20 = pcVar1[0xb];
          cVar21 = pcVar1[0xc];
          cVar22 = pcVar1[0xd];
          cVar23 = pcVar1[0xe];
          cVar24 = pcVar1[0xf];
          auVar44[0] = -(cVar9 == cVar26);
          auVar44[1] = -(cVar10 == cVar45);
          auVar44[2] = -(cVar11 == cVar46);
          auVar44[3] = -(cVar12 == cVar47);
          auVar44[4] = -(cVar13 == cVar26);
          auVar44[5] = -(cVar14 == cVar45);
          auVar44[6] = -(cVar15 == cVar46);
          auVar44[7] = -(cVar16 == cVar47);
          auVar44[8] = -(cVar17 == cVar26);
          auVar44[9] = -(cVar18 == cVar45);
          auVar44[10] = -(cVar19 == cVar46);
          auVar44[0xb] = -(cVar20 == cVar47);
          auVar44[0xc] = -(cVar21 == cVar26);
          auVar44[0xd] = -(cVar22 == cVar45);
          auVar44[0xe] = -(cVar23 == cVar46);
          auVar44[0xf] = -(cVar24 == cVar47);
          uVar27 = (ushort)(SUB161(auVar44 >> 7,0) & 1) |
                   (ushort)(SUB161(auVar44 >> 0xf,0) & 1) << 1 |
                   (ushort)(SUB161(auVar44 >> 0x17,0) & 1) << 2 |
                   (ushort)(SUB161(auVar44 >> 0x1f,0) & 1) << 3 |
                   (ushort)(SUB161(auVar44 >> 0x27,0) & 1) << 4 |
                   (ushort)(SUB161(auVar44 >> 0x2f,0) & 1) << 5 |
                   (ushort)(SUB161(auVar44 >> 0x37,0) & 1) << 6 |
                   (ushort)(SUB161(auVar44 >> 0x3f,0) & 1) << 7 |
                   (ushort)(SUB161(auVar44 >> 0x47,0) & 1) << 8 |
                   (ushort)(SUB161(auVar44 >> 0x4f,0) & 1) << 9 |
                   (ushort)(SUB161(auVar44 >> 0x57,0) & 1) << 10 |
                   (ushort)(SUB161(auVar44 >> 0x5f,0) & 1) << 0xb |
                   (ushort)(SUB161(auVar44 >> 0x67,0) & 1) << 0xc |
                   (ushort)(SUB161(auVar44 >> 0x6f,0) & 1) << 0xd |
                   (ushort)(SUB161(auVar44 >> 0x77,0) & 1) << 0xe |
                   (ushort)(auVar44[0xf] >> 7) << 0xf;
          uVar30 = (uint)uVar27;
          puVar40 = _DAT_14362c420;
          lVar2 = lStack_c0;
          while (lStack_c0 = lVar37, _DAT_14362c420 = puVar40, uVar27 != 0) {
            uVar4 = 0;
            for (uVar5 = uVar30; (uVar5 & 1) == 0; uVar5 = uVar5 >> 1 | 0x80000000) {
              uVar4 = uVar4 + 1;
            }
            uVar32 = uVar4 + uVar33 & uVar31;
            if ((*(undefined8 ***)(lVar36 + -0x18 + uVar32 * -0x28) == param_3) &&
               (iVar29 = func_0x000142923bf0(uStack_98,
                                             *(undefined8 *)(lVar36 + uVar32 * -0x28 + -0x20),
                                             param_3), iVar29 == 0)) {
              uVar38 = *(undefined8 *)(lVar36 + -0x10 + uVar32 * -0x28);
              puVar40 = _DAT_14362c420;
              goto LAB_1418485a1;
            }
            uVar27 = (ushort)(uVar30 - 1) & (ushort)uVar30;
            uVar30 = CONCAT22((short)(uVar30 - 1 >> 0x10),uVar27);
            lVar37 = lStack_c0;
            puVar40 = _DAT_14362c420;
            lVar2 = lStack_c0;
          }
          auVar49[0] = -(cVar9 == (char)iVar28);
          auVar49[1] = -(cVar10 == (char)((uint)iVar28 >> 8));
          auVar49[2] = -(cVar11 == (char)((uint)iVar28 >> 0x10));
          auVar49[3] = -(cVar12 == (char)((uint)iVar28 >> 0x18));
          auVar49[4] = -(cVar13 == (char)iVar6);
          auVar49[5] = -(cVar14 == (char)((uint)iVar6 >> 8));
          auVar49[6] = -(cVar15 == (char)((uint)iVar6 >> 0x10));
          auVar49[7] = -(cVar16 == (char)((uint)iVar6 >> 0x18));
          auVar49[8] = -(cVar17 == (char)iVar7);
          auVar49[9] = -(cVar18 == (char)((uint)iVar7 >> 8));
          auVar49[10] = -(cVar19 == (char)((uint)iVar7 >> 0x10));
          auVar49[0xb] = -(cVar20 == (char)((uint)iVar7 >> 0x18));
          auVar49[0xc] = -(cVar21 == (char)iVar8);
          auVar49[0xd] = -(cVar22 == (char)((uint)iVar8 >> 8));
          auVar49[0xe] = -(cVar23 == (char)((uint)iVar8 >> 0x10));
          auVar49[0xf] = -(cVar24 == (char)((uint)iVar8 >> 0x18));
          if ((((((((((((((((SUB161(auVar49 >> 7,0) & 1) != 0 || (SUB161(auVar49 >> 0xf,0) & 1) != 0
                           ) || (SUB161(auVar49 >> 0x17,0) & 1) != 0) ||
                         (SUB161(auVar49 >> 0x1f,0) & 1) != 0) ||
                        (SUB161(auVar49 >> 0x27,0) & 1) != 0) ||
                       (SUB161(auVar49 >> 0x2f,0) & 1) != 0) || (SUB161(auVar49 >> 0x37,0) & 1) != 0
                      ) || (SUB161(auVar49 >> 0x3f,0) & 1) != 0) ||
                    (SUB161(auVar49 >> 0x47,0) & 1) != 0) || (SUB161(auVar49 >> 0x4f,0) & 1) != 0)
                  || (SUB161(auVar49 >> 0x57,0) & 1) != 0) || (SUB161(auVar49 >> 0x5f,0) & 1) != 0)
                || (SUB161(auVar49 >> 0x67,0) & 1) != 0) || (SUB161(auVar49 >> 0x6f,0) & 1) != 0) ||
              (SUB161(auVar49 >> 0x77,0) & 1) != 0) || auVar49[0xf] < '\0') break;
          uVar33 = uVar33 + lStack_c0 + 0x10;
          lVar37 = lStack_c0 + 0x10;
          lStack_c0 = lVar2;
        }
      }
    }
    lStack_c0 = lVar2;
    if (*_DAT_14362c3a8 != 0) {
      puStack_b8 = &uStack_a8;
      puStack_b0 = &UNK_1405338a0;
      puVar35 = (undefined8 *)func_0x00014067dd80(&UNK_142cf1b70);
      uStack_120 = *puVar35;
      uStack_118 = puVar35[1];
      uStack_f4 = *(undefined4 *)(puVar35 + 2);
      uStack_110 = 1;
      puStack_108 = &UNK_142cf1b88;
      uStack_100 = 0x1a;
      puStack_f0 = &UNK_142cf1b60;
      uStack_e8 = 1;
      ppuStack_e0 = &puStack_b8;
      uStack_d8 = 1;
      uStack_d0 = 0;
      ppuStack_140 = (undefined8 **)0x0;
      puStack_138 = &UNK_142cf1b88;
      ppuStack_130 = (undefined8 **)0x1a;
      uStack_128 = 0;
      uStack_f8 = 1;
      func_0x000140d4a330(&uStack_81,&ppuStack_140);
    }
    puStack_b8 = &uStack_a8;
    puStack_b0 = &UNK_140109e90;
    ppuStack_140 = (undefined8 **)&UNK_142cf1ba8;
    puStack_138 = (undefined *)0x1;
    ppuStack_130 = &puStack_b8;
    uStack_128 = 1;
    uStack_120 = 0;
    FUN_140d79f90(auStack_158,&ppuStack_140);
    ppuVar25 = ppuStack_a0;
    uVar38 = uStack_a8;
    if ((longlong)ppuStack_a0 < 0) {
      uVar39 = 0;
    }
    else {
      if (ppuStack_a0 == (undefined8 **)0x0) {
        lVar36 = 1;
LAB_141848571:
        func_0x000142923c00(lVar36,uVar38,ppuVar25);
        ppuStack_140 = ppuVar25;
        ppuStack_130 = ppuVar25;
        puStack_138 = (undefined *)lVar36;
        FUN_14212cc80(&DAT_143639610,&ppuStack_140,uStack_150,uStack_148);
        uVar38 = uStack_150;
LAB_1418485a1:
        if ((((char)uStack_90 == '\0') && ((*puVar40 & 0x7fffffffffffffff) != 0)) &&
           (cVar26 = FUN_1428d86b0(), cVar26 == '\0')) {
          DAT_143639609 = '\x01';
        }
        cVar26 = DAT_143639608;
        LOCK();
        DAT_143639608 = 0;
        UNLOCK();
        if (cVar26 == '\x02') {
          FUN_1428d8810(&DAT_143639608);
        }
        return uVar38;
      }
      uVar39 = 1;
      lVar36 = func_0x000140613c10(ppuStack_a0,1);
      if (lVar36 != 0) goto LAB_141848571;
    }
    FUN_1428d8fe3(uVar39,ppuVar25,&UNK_142cf1ab8);
  }
  else {
    ppuStack_140 = (undefined8 **)&DAT_143639608;
    uVar33 = (ulonglong)puStack_138 >> 8;
    puStack_138 = (undefined *)CONCAT71((int7)uVar33,(char)uVar30);
    FUN_1428d9760(&UNK_142cf1a40,0x2b,&ppuStack_140,&UNK_142cf1a20,&UNK_142cf1b28);
  }
                    /* WARNING: Does not return */
  pcVar3 = (code *)invalidInstructionException();
  (*pcVar3)();
}

