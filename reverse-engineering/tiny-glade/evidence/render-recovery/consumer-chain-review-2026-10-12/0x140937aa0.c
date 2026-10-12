
undefined8 FUN_140937aa0(longlong param_1,undefined8 param_2,longlong param_3)

{
  ulonglong *puVar1;
  char *pcVar2;
  longlong lVar3;
  ulonglong uVar4;
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
  ushort uVar27;
  int iVar28;
  ulonglong uVar29;
  ulonglong uVar30;
  undefined8 uVar31;
  uint uVar32;
  longlong lVar33;
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
  
  puVar1 = (ulonglong *)(param_1 + 0x80);
  uVar34 = *(ulonglong *)(param_1 + 0x80);
  if (uVar34 < 0xfffffffffffffff0 && (char)(((uint)uVar34 & 8) >> 3) == '\0') {
    LOCK();
    uVar4 = *puVar1;
    if (uVar34 == uVar4) {
      *puVar1 = uVar34 + 0x10;
    }
    UNLOCK();
    if (uVar34 == uVar4) goto code_r0x000140937b0a;
  }
  FUN_1428cc4b0(puVar1,0,param_3,1000000000);
code_r0x000140937b0a:
  FUN_140c81c70(puVar1);
  uVar34 = (ulonglong)puVar1 | 1;
  FUN_140c81c70(uVar34);
  if (*(longlong *)(param_1 + 0xa0) != 0) {
    uVar29 = FUN_1405b1a70(param_1 + 0xa8,param_2);
    lVar3 = *(longlong *)(param_1 + 0x88);
    uVar4 = *(ulonglong *)(param_1 + 0x90);
    bVar35 = (byte)(uVar29 >> 0x39);
    auVar36 = ZEXT216(CONCAT11(bVar35,bVar35));
    auVar36 = pshuflw(auVar36,auVar36,0);
    cVar37 = auVar36[0];
    cVar38 = auVar36[1];
    cVar39 = auVar36[2];
    cVar40 = auVar36[3];
    lVar33 = 0;
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
      uVar29 = uVar4 & uVar29;
      pcVar2 = (char *)(lVar3 + uVar29);
      cVar11 = *pcVar2;
      cVar12 = pcVar2[1];
      cVar13 = pcVar2[2];
      cVar14 = pcVar2[3];
      cVar15 = pcVar2[4];
      cVar16 = pcVar2[5];
      cVar17 = pcVar2[6];
      cVar18 = pcVar2[7];
      cVar19 = pcVar2[8];
      cVar20 = pcVar2[9];
      cVar21 = pcVar2[10];
      cVar22 = pcVar2[0xb];
      cVar23 = pcVar2[0xc];
      cVar24 = pcVar2[0xd];
      cVar25 = pcVar2[0xe];
      cVar26 = pcVar2[0xf];
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
      uVar27 = (ushort)(SUB161(auVar36 >> 7,0) & 1) | (ushort)(SUB161(auVar36 >> 0xf,0) & 1) << 1 |
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
      uVar32 = (uint)uVar27;
      while (uVar27 != 0) {
        uVar5 = 0;
        for (uVar6 = uVar32; (uVar6 & 1) == 0; uVar6 = uVar6 >> 1 | 0x80000000) {
          uVar5 = uVar5 + 1;
        }
        uVar30 = uVar5 + uVar29 & uVar4;
        if ((*(longlong *)(lVar3 + -0x10 + uVar30 * -0x18) == param_3) &&
           (iVar28 = func_0x000142923bf0(param_2,*(undefined8 *)(lVar3 + -0x18 + uVar30 * -0x18),
                                         param_3), iVar28 == 0)) {
          FUN_140c81e10(puVar1);
          FUN_140c81e10(uVar34);
          LOCK();
          uVar34 = *puVar1;
          *puVar1 = *puVar1 - 0x10;
          UNLOCK();
          uVar31 = 1;
          goto joined_r0x000140937c9b;
        }
        uVar27 = (ushort)(uVar32 - 1) & (ushort)uVar32;
        uVar32 = CONCAT22((short)(uVar32 - 1 >> 0x10),uVar27);
      }
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
      uVar29 = uVar29 + lVar33 + 0x10;
      lVar33 = lVar33 + 0x10;
    }
  }
  FUN_140c81e10(puVar1);
  FUN_140c81e10(uVar34);
  LOCK();
  uVar34 = *puVar1;
  *puVar1 = *puVar1 - 0x10;
  UNLOCK();
  uVar31 = 0;
joined_r0x000140937c9b:
  if ((uVar34 & 0xfffffffffffffff2) == 0x12) {
    FUN_1428ccf50(puVar1);
  }
  return uVar31;
}

