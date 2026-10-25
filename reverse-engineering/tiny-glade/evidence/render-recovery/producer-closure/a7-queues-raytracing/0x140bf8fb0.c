
ulonglong FUN_140bf8fb0(undefined8 *param_1,undefined8 param_2,undefined8 param_3)

{
  char *pcVar1;
  longlong lVar2;
  longlong *plVar3;
  code *pcVar4;
  uint uVar5;
  uint uVar6;
  int iVar7;
  int iVar8;
  int iVar9;
  int iVar10;
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
  char cVar25;
  char cVar26;
  longlong lVar27;
  longlong lVar28;
  ushort uVar29;
  int iVar30;
  ulonglong uVar31;
  ulonglong uVar32;
  undefined8 *puVar33;
  undefined *puVar34;
  ulonglong uVar35;
  longlong lVar36;
  uint uVar37;
  byte bVar38;
  undefined1 auVar39 [16];
  char cVar40;
  char cVar41;
  char cVar42;
  char cVar43;
  undefined1 unaff_XMM7_Ba;
  undefined1 unaff_XMM7_Bb;
  undefined1 unaff_XMM7_Bc;
  undefined1 unaff_XMM7_Bd;
  undefined1 unaff_XMM7_Be;
  undefined1 unaff_XMM7_Bf;
  undefined1 unaff_XMM7_Bg;
  undefined1 unaff_XMM7_Bh;
  undefined1 unaff_XMM7_Bi;
  undefined1 unaff_XMM7_Bj;
  undefined1 unaff_XMM7_Bk;
  undefined1 unaff_XMM7_Bl;
  undefined1 unaff_XMM7_Bm;
  undefined1 unaff_XMM7_Bn;
  undefined1 unaff_XMM7_Bo;
  undefined1 unaff_XMM7_Bp;
  undefined1 auVar44 [16];
  byte bStack_14a;
  undefined1 uStack_149;
  undefined8 **ppuStack_148;
  undefined *puStack_140;
  byte *pbStack_138;
  code *pcStack_130;
  longlong lStack_120;
  undefined8 *puStack_118;
  undefined8 *puStack_110;
  undefined8 uStack_108;
  undefined8 *puStack_100;
  longlong lStack_f8;
  undefined8 uStack_f0;
  undefined8 uStack_e8;
  undefined8 uStack_e0;
  undefined8 uStack_d8;
  undefined *puStack_d0;
  undefined8 uStack_c8;
  undefined4 uStack_c0;
  undefined4 uStack_bc;
  undefined *puStack_b8;
  undefined8 uStack_b0;
  undefined8 **ppuStack_a8;
  undefined8 uStack_a0;
  undefined8 uStack_98;
  undefined8 *puStack_88;
  longlong lStack_80;
  
  lVar36 = *(longlong *)*param_1;
  FUN_140d81100();
  lVar27 = lStack_f8;
  if ((int)uStack_108 == 1) {
    ppuStack_148 = (undefined8 **)puStack_100;
    puStack_140 = (undefined *)lStack_f8;
    puVar34 = &UNK_142b16898;
code_r0x000140bf92c7:
    FUN_1428d9760(&UNK_142b15188,0x2b,&ppuStack_148,&UNK_142b15218,puVar34);
    pcVar4 = (code *)swi(3);
    uVar35 = (*pcVar4)();
    return uVar35;
  }
  lVar28 = lStack_120;
  puStack_110 = param_1;
  if (*(longlong *)(lVar36 + 0xb8) != 0) {
    puStack_118 = puStack_100;
    uVar31 = FUN_1405b1a70(lVar36 + 0xc0,puStack_100,lStack_f8);
    lVar2 = *(longlong *)(lVar36 + 0xa0);
    uVar35 = *(ulonglong *)(lVar36 + 0xa8);
    bVar38 = (byte)(uVar31 >> 0x39);
    auVar39 = ZEXT216(CONCAT11(bVar38,bVar38));
    auVar39 = pshuflw(auVar39,auVar39,0);
    cVar40 = auVar39[0];
    cVar41 = auVar39[1];
    cVar42 = auVar39[2];
    cVar43 = auVar39[3];
    lVar36 = 0;
    iVar7 = -(uint)(CONCAT13(unaff_XMM7_Bd,
                             CONCAT12(unaff_XMM7_Bc,CONCAT11(unaff_XMM7_Bb,unaff_XMM7_Ba))) ==
                   CONCAT13(unaff_XMM7_Bd,
                            CONCAT12(unaff_XMM7_Bc,CONCAT11(unaff_XMM7_Bb,unaff_XMM7_Ba))));
    iVar8 = -(uint)(CONCAT13(unaff_XMM7_Bh,
                             CONCAT12(unaff_XMM7_Bg,CONCAT11(unaff_XMM7_Bf,unaff_XMM7_Be))) ==
                   CONCAT13(unaff_XMM7_Bh,
                            CONCAT12(unaff_XMM7_Bg,CONCAT11(unaff_XMM7_Bf,unaff_XMM7_Be))));
    iVar9 = -(uint)(CONCAT13(unaff_XMM7_Bl,
                             CONCAT12(unaff_XMM7_Bk,CONCAT11(unaff_XMM7_Bj,unaff_XMM7_Bi))) ==
                   CONCAT13(unaff_XMM7_Bl,
                            CONCAT12(unaff_XMM7_Bk,CONCAT11(unaff_XMM7_Bj,unaff_XMM7_Bi))));
    iVar10 = -(uint)(CONCAT13(unaff_XMM7_Bp,
                              CONCAT12(unaff_XMM7_Bo,CONCAT11(unaff_XMM7_Bn,unaff_XMM7_Bm))) ==
                    CONCAT13(unaff_XMM7_Bp,
                             CONCAT12(unaff_XMM7_Bo,CONCAT11(unaff_XMM7_Bn,unaff_XMM7_Bm))));
    while( true ) {
      uVar31 = uVar35 & uVar31;
      pcVar1 = (char *)(lVar2 + uVar31);
      cVar11 = *pcVar1;
      cVar12 = pcVar1[1];
      cVar13 = pcVar1[2];
      cVar14 = pcVar1[3];
      cVar15 = pcVar1[4];
      cVar16 = pcVar1[5];
      cVar17 = pcVar1[6];
      cVar18 = pcVar1[7];
      cVar19 = pcVar1[8];
      cVar20 = pcVar1[9];
      cVar21 = pcVar1[10];
      cVar22 = pcVar1[0xb];
      cVar23 = pcVar1[0xc];
      cVar24 = pcVar1[0xd];
      cVar25 = pcVar1[0xe];
      cVar26 = pcVar1[0xf];
      auVar39[0] = -(cVar11 == cVar40);
      auVar39[1] = -(cVar12 == cVar41);
      auVar39[2] = -(cVar13 == cVar42);
      auVar39[3] = -(cVar14 == cVar43);
      auVar39[4] = -(cVar15 == cVar40);
      auVar39[5] = -(cVar16 == cVar41);
      auVar39[6] = -(cVar17 == cVar42);
      auVar39[7] = -(cVar18 == cVar43);
      auVar39[8] = -(cVar19 == cVar40);
      auVar39[9] = -(cVar20 == cVar41);
      auVar39[10] = -(cVar21 == cVar42);
      auVar39[0xb] = -(cVar22 == cVar43);
      auVar39[0xc] = -(cVar23 == cVar40);
      auVar39[0xd] = -(cVar24 == cVar41);
      auVar39[0xe] = -(cVar25 == cVar42);
      auVar39[0xf] = -(cVar26 == cVar43);
      uVar29 = (ushort)(SUB161(auVar39 >> 7,0) & 1) | (ushort)(SUB161(auVar39 >> 0xf,0) & 1) << 1 |
               (ushort)(SUB161(auVar39 >> 0x17,0) & 1) << 2 |
               (ushort)(SUB161(auVar39 >> 0x1f,0) & 1) << 3 |
               (ushort)(SUB161(auVar39 >> 0x27,0) & 1) << 4 |
               (ushort)(SUB161(auVar39 >> 0x2f,0) & 1) << 5 |
               (ushort)(SUB161(auVar39 >> 0x37,0) & 1) << 6 |
               (ushort)(SUB161(auVar39 >> 0x3f,0) & 1) << 7 |
               (ushort)(SUB161(auVar39 >> 0x47,0) & 1) << 8 |
               (ushort)(SUB161(auVar39 >> 0x4f,0) & 1) << 9 |
               (ushort)(SUB161(auVar39 >> 0x57,0) & 1) << 10 |
               (ushort)(SUB161(auVar39 >> 0x5f,0) & 1) << 0xb |
               (ushort)(SUB161(auVar39 >> 0x67,0) & 1) << 0xc |
               (ushort)(SUB161(auVar39 >> 0x6f,0) & 1) << 0xd |
               (ushort)(SUB161(auVar39 >> 0x77,0) & 1) << 0xe | (ushort)(auVar39[0xf] >> 7) << 0xf;
      uVar37 = (uint)uVar29;
      lVar28 = lStack_120;
      while (lStack_120 = lVar36, uVar29 != 0) {
        uVar5 = 0;
        for (uVar6 = uVar37; (uVar6 & 1) == 0; uVar6 = uVar6 >> 1 | 0x80000000) {
          uVar5 = uVar5 + 1;
        }
        uVar32 = uVar5 + uVar31 & uVar35;
        if (*(longlong *)(lVar2 + -8 + uVar32 * -0x18) == lVar27) {
          iVar30 = func_0x000142923bf0(puStack_118,*(undefined8 *)(lVar2 + uVar32 * -0x18 + -0x10),
                                       lVar27);
          bStack_14a = 1;
          if (iVar30 == 0) goto code_r0x000140bf90f3;
        }
        uVar29 = (ushort)(uVar37 - 1) & (ushort)uVar37;
        uVar37 = CONCAT22((short)(uVar37 - 1 >> 0x10),uVar29);
        lVar36 = lStack_120;
        lVar28 = lStack_120;
      }
      auVar44[0] = -(cVar11 == (char)iVar7);
      auVar44[1] = -(cVar12 == (char)((uint)iVar7 >> 8));
      auVar44[2] = -(cVar13 == (char)((uint)iVar7 >> 0x10));
      auVar44[3] = -(cVar14 == (char)((uint)iVar7 >> 0x18));
      auVar44[4] = -(cVar15 == (char)iVar8);
      auVar44[5] = -(cVar16 == (char)((uint)iVar8 >> 8));
      auVar44[6] = -(cVar17 == (char)((uint)iVar8 >> 0x10));
      auVar44[7] = -(cVar18 == (char)((uint)iVar8 >> 0x18));
      auVar44[8] = -(cVar19 == (char)iVar9);
      auVar44[9] = -(cVar20 == (char)((uint)iVar9 >> 8));
      auVar44[10] = -(cVar21 == (char)((uint)iVar9 >> 0x10));
      auVar44[0xb] = -(cVar22 == (char)((uint)iVar9 >> 0x18));
      auVar44[0xc] = -(cVar23 == (char)iVar10);
      auVar44[0xd] = -(cVar24 == (char)((uint)iVar10 >> 8));
      auVar44[0xe] = -(cVar25 == (char)((uint)iVar10 >> 0x10));
      auVar44[0xf] = -(cVar26 == (char)((uint)iVar10 >> 0x18));
      if ((((((((((((((((SUB161(auVar44 >> 7,0) & 1) != 0 || (SUB161(auVar44 >> 0xf,0) & 1) != 0) ||
                      (SUB161(auVar44 >> 0x17,0) & 1) != 0) || (SUB161(auVar44 >> 0x1f,0) & 1) != 0)
                    || (SUB161(auVar44 >> 0x27,0) & 1) != 0) || (SUB161(auVar44 >> 0x2f,0) & 1) != 0
                   ) || (SUB161(auVar44 >> 0x37,0) & 1) != 0) ||
                 (SUB161(auVar44 >> 0x3f,0) & 1) != 0) || (SUB161(auVar44 >> 0x47,0) & 1) != 0) ||
               (SUB161(auVar44 >> 0x4f,0) & 1) != 0) || (SUB161(auVar44 >> 0x57,0) & 1) != 0) ||
             (SUB161(auVar44 >> 0x5f,0) & 1) != 0) || (SUB161(auVar44 >> 0x67,0) & 1) != 0) ||
           (SUB161(auVar44 >> 0x6f,0) & 1) != 0) || (SUB161(auVar44 >> 0x77,0) & 1) != 0) ||
          auVar44[0xf] < '\0') break;
      uVar31 = uVar31 + lStack_120 + 0x10;
      lVar36 = lStack_120 + 0x10;
      lStack_120 = lVar28;
    }
  }
  lStack_120 = lVar28;
  bStack_14a = 0;
code_r0x000140bf90f3:
  if (3 < *puRam000000014362c3a8) {
    FUN_140d81100(&uStack_108,param_2,param_3);
    if ((int)uStack_108 == 1) {
      ppuStack_148 = (undefined8 **)puStack_100;
      puStack_140 = (undefined *)lStack_f8;
      puVar34 = &UNK_142b168b0;
      goto code_r0x000140bf92c7;
    }
    puStack_88 = puStack_100;
    lStack_80 = lStack_f8;
    ppuStack_148 = &puStack_88;
    puStack_140 = &UNK_140109e90;
    pbStack_138 = &bStack_14a;
    pcStack_130 = FUN_140d88320;
    puVar33 = (undefined8 *)func_0x00014067dd80(&UNK_142b16900);
    uStack_e8 = *puVar33;
    uStack_e0 = puVar33[1];
    uStack_bc = *(undefined4 *)(puVar33 + 2);
    uStack_d8 = 4;
    puStack_d0 = &UNK_142b16030;
    uStack_c8 = 0x15;
    puStack_b8 = &UNK_142b168e0;
    uStack_b0 = 2;
    ppuStack_a8 = &ppuStack_148;
    uStack_a0 = 2;
    uStack_98 = 0;
    uStack_108 = 0;
    puStack_100 = (undefined8 *)&UNK_142b16030;
    lStack_f8 = 0x15;
    uStack_f0 = 0;
    uStack_c0 = 1;
    func_0x000140d4a330(&uStack_149,&uStack_108);
  }
  if ((bStack_14a & 1) == 0) {
    uVar35 = 0;
  }
  else {
    plVar3 = (longlong *)puStack_110[1];
    lVar36 = plVar3[2];
    uVar35 = CONCAT71((int7)((ulonglong)puStack_110 >> 8),1);
    if (lVar36 == *plVar3) {
      FUN_140562870(plVar3,&UNK_142b16918);
      uVar35 = (ulonglong)bStack_14a;
    }
    *(undefined8 *)(plVar3[1] + lVar36 * 8) = param_2;
    plVar3[2] = lVar36 + 1;
  }
  return uVar35 & 0xffffffffffffff01;
}

