
undefined8 FUN_140969720(uint *param_1,ulonglong *param_2)

{
  char *pcVar1;
  uint uVar2;
  longlong lVar3;
  ulonglong uVar4;
  code *pcVar5;
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
  char cVar28;
  ushort uVar29;
  undefined8 uVar30;
  ulonglong uVar31;
  longlong lVar32;
  uint uVar33;
  ulonglong uVar34;
  byte bVar35;
  undefined1 auVar36 [16];
  char cVar37;
  char cVar38;
  char cVar39;
  char cVar40;
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
  undefined1 auVar41 [16];
  uint *puStack_a0;
  uint *puStack_98;
  longlong lStack_90;
  uint *puStack_88;
  undefined8 uStack_80;
  
  uStack_80 = 0xfffffffffffffffe;
  uVar33 = *param_1;
  if (uVar33 < 0x3ffffffe) {
    LOCK();
    uVar2 = *param_1;
    if (uVar33 == uVar2) {
      *param_1 = uVar33 + 1;
    }
    UNLOCK();
    if (uVar33 == uVar2) {
      cVar37 = (char)param_1[2];
      goto joined_r0x0001409698c3;
    }
  }
  FUN_1428d8a30(param_1);
  cVar37 = (char)param_1[2];
joined_r0x0001409698c3:
  if (cVar37 != '\0') {
    puStack_a0 = param_1 + 4;
    puStack_98 = param_1;
    FUN_1428d9760(&UNK_142ab57c0,0x2b,&puStack_a0,&UNK_142ab57f0,&UNK_142ab5958);
                    /* WARNING: Does not return */
    pcVar5 = (code *)invalidInstructionException();
    (*pcVar5)();
  }
  if (*(longlong *)(param_1 + 10) == 0) {
    uVar30 = 0;
    lVar27 = lStack_90;
  }
  else {
    uVar31 = *param_2;
    lVar3 = *(longlong *)(param_1 + 4);
    uVar4 = *(ulonglong *)(param_1 + 6);
    bVar35 = (byte)(uVar31 >> 0x39);
    auVar36 = ZEXT216(CONCAT11(bVar35,bVar35));
    auVar36 = pshuflw(auVar36,auVar36,0);
    cVar37 = auVar36[0];
    cVar38 = auVar36[1];
    cVar39 = auVar36[2];
    cVar40 = auVar36[3];
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
    lVar32 = 0;
    while( true ) {
      uVar31 = uVar4 & uVar31;
      pcVar1 = (char *)(lVar3 + uVar31);
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
      auVar36[0] = -(cVar11 == cVar37);
      auVar36[1] = -(cVar12 == cVar38);
      auVar36[2] = -(cVar13 == cVar39);
      auVar36[3] = -(cVar14 == cVar40);
      auVar36[4] = -(cVar15 == cVar37);
      auVar36[5] = -(cVar16 == cVar38);
      auVar36[6] = -(cVar17 == cVar39);
      auVar36[7] = -(cVar18 == cVar40);
      auVar36[8] = -(cVar19 == cVar37);
      auVar36[9] = -(cVar20 == cVar38);
      auVar36[10] = -(cVar21 == cVar39);
      auVar36[0xb] = -(cVar22 == cVar40);
      auVar36[0xc] = -(cVar23 == cVar37);
      auVar36[0xd] = -(cVar24 == cVar38);
      auVar36[0xe] = -(cVar25 == cVar39);
      auVar36[0xf] = -(cVar26 == cVar40);
      uVar29 = (ushort)(SUB161(auVar36 >> 7,0) & 1) | (ushort)(SUB161(auVar36 >> 0xf,0) & 1) << 1 |
               (ushort)(SUB161(auVar36 >> 0x17,0) & 1) << 2 |
               (ushort)(SUB161(auVar36 >> 0x1f,0) & 1) << 3 |
               (ushort)(SUB161(auVar36 >> 0x27,0) & 1) << 4 |
               (ushort)(SUB161(auVar36 >> 0x2f,0) & 1) << 5 |
               (ushort)(SUB161(auVar36 >> 0x37,0) & 1) << 6 |
               (ushort)(SUB161(auVar36 >> 0x3f,0) & 1) << 7 |
               (ushort)(SUB161(auVar36 >> 0x47,0) & 1) << 8 |
               (ushort)(SUB161(auVar36 >> 0x4f,0) & 1) << 9 |
               (ushort)(SUB161(auVar36 >> 0x57,0) & 1) << 10 |
               (ushort)(SUB161(auVar36 >> 0x5f,0) & 1) << 0xb |
               (ushort)(SUB161(auVar36 >> 0x67,0) & 1) << 0xc |
               (ushort)(SUB161(auVar36 >> 0x6f,0) & 1) << 0xd |
               (ushort)(SUB161(auVar36 >> 0x77,0) & 1) << 0xe | (ushort)(auVar36[0xf] >> 7) << 0xf;
      uVar33 = (uint)uVar29;
      lVar27 = lStack_90;
      while (lStack_90 = lVar32, uVar29 != 0) {
        uVar2 = 0;
        for (uVar6 = uVar33; (uVar6 & 1) == 0; uVar6 = uVar6 >> 1 | 0x80000000) {
          uVar2 = uVar2 + 1;
        }
        uVar34 = uVar2 + uVar31 & uVar4;
        puStack_88 = param_1;
        cVar28 = func_0x000140c9cd70(param_2,lVar3 + -0x10 + uVar34 * -0x10);
        param_1 = puStack_88;
        lVar27 = lStack_90;
        if (cVar28 != '\0') {
          uVar30 = *(undefined8 *)(lVar3 + -8 + uVar34 * -0x10);
          goto code_r0x000140969875;
        }
        uVar29 = (ushort)(uVar33 - 1) & (ushort)uVar33;
        uVar33 = CONCAT22((short)(uVar33 - 1 >> 0x10),uVar29);
        lVar32 = lStack_90;
      }
      uVar30 = 0;
      auVar41[0] = -(cVar11 == (char)iVar7);
      auVar41[1] = -(cVar12 == (char)((uint)iVar7 >> 8));
      auVar41[2] = -(cVar13 == (char)((uint)iVar7 >> 0x10));
      auVar41[3] = -(cVar14 == (char)((uint)iVar7 >> 0x18));
      auVar41[4] = -(cVar15 == (char)iVar8);
      auVar41[5] = -(cVar16 == (char)((uint)iVar8 >> 8));
      auVar41[6] = -(cVar17 == (char)((uint)iVar8 >> 0x10));
      auVar41[7] = -(cVar18 == (char)((uint)iVar8 >> 0x18));
      auVar41[8] = -(cVar19 == (char)iVar9);
      auVar41[9] = -(cVar20 == (char)((uint)iVar9 >> 8));
      auVar41[10] = -(cVar21 == (char)((uint)iVar9 >> 0x10));
      auVar41[0xb] = -(cVar22 == (char)((uint)iVar9 >> 0x18));
      auVar41[0xc] = -(cVar23 == (char)iVar10);
      auVar41[0xd] = -(cVar24 == (char)((uint)iVar10 >> 8));
      auVar41[0xe] = -(cVar25 == (char)((uint)iVar10 >> 0x10));
      auVar41[0xf] = -(cVar26 == (char)((uint)iVar10 >> 0x18));
      if ((((((((((((((((SUB161(auVar41 >> 7,0) & 1) != 0 || (SUB161(auVar41 >> 0xf,0) & 1) != 0) ||
                      (SUB161(auVar41 >> 0x17,0) & 1) != 0) || (SUB161(auVar41 >> 0x1f,0) & 1) != 0)
                    || (SUB161(auVar41 >> 0x27,0) & 1) != 0) || (SUB161(auVar41 >> 0x2f,0) & 1) != 0
                   ) || (SUB161(auVar41 >> 0x37,0) & 1) != 0) ||
                 (SUB161(auVar41 >> 0x3f,0) & 1) != 0) || (SUB161(auVar41 >> 0x47,0) & 1) != 0) ||
               (SUB161(auVar41 >> 0x4f,0) & 1) != 0) || (SUB161(auVar41 >> 0x57,0) & 1) != 0) ||
             (SUB161(auVar41 >> 0x5f,0) & 1) != 0) || (SUB161(auVar41 >> 0x67,0) & 1) != 0) ||
           (SUB161(auVar41 >> 0x6f,0) & 1) != 0) || (SUB161(auVar41 >> 0x77,0) & 1) != 0) ||
          auVar41[0xf] < '\0') break;
      uVar31 = uVar31 + lStack_90 + 0x10;
      lVar32 = lStack_90 + 0x10;
      lStack_90 = lVar27;
    }
  }
code_r0x000140969875:
  lStack_90 = lVar27;
  LOCK();
  uVar33 = *param_1;
  *param_1 = *param_1 - 1;
  UNLOCK();
  if (SBORROW4(0,uVar33 - 1 & 0xbfffffff)) {
    FUN_1428d8d40(param_1);
  }
  return uVar30;
}

