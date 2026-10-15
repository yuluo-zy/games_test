
undefined8 *
FUN_140bdc5d0(undefined8 *param_1,longlong param_2,longlong param_3,undefined8 *param_4)

{
  ulonglong *puVar1;
  char *pcVar2;
  longlong lVar3;
  undefined1 (*pauVar4) [16];
  uint uVar5;
  int iVar6;
  int iVar7;
  int iVar8;
  int iVar9;
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
  char cVar25;
  char cVar26;
  ushort uVar27;
  ulonglong uVar28;
  undefined8 *puVar29;
  ulonglong uVar30;
  longlong lVar31;
  ulonglong *unaff_RDI;
  byte bVar32;
  longlong lVar33;
  uint uVar34;
  uint uVar35;
  ulonglong unaff_R12;
  ulonglong uVar36;
  undefined8 *unaff_R14;
  ulonglong unaff_R15;
  byte bVar37;
  undefined1 auVar38 [16];
  char cVar39;
  char cVar40;
  char cVar41;
  char cVar42;
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
  undefined1 auVar43 [16];
  undefined8 uStack_148;
  undefined8 uStack_140;
  undefined4 uStack_138;
  undefined4 uStack_134;
  undefined4 uStack_130;
  undefined4 uStack_12c;
  undefined4 uStack_128;
  undefined4 uStack_124;
  undefined4 uStack_120;
  undefined4 uStack_11c;
  undefined4 uStack_118;
  undefined4 uStack_114;
  undefined4 uStack_110;
  undefined8 *puStack_100;
  ulonglong uStack_f8;
  int iStack_f0;
  undefined4 uStack_ec;
  undefined4 uStack_e8;
  undefined4 uStack_e4;
  undefined4 uStack_e0;
  undefined4 uStack_dc;
  undefined4 uStack_d8;
  undefined4 uStack_d4;
  undefined4 uStack_d0;
  undefined4 uStack_cc;
  undefined4 uStack_c8;
  undefined4 uStack_c4;
  undefined4 uStack_c0;
  undefined4 uStack_bc;
  undefined8 uStack_b8;
  longlong lStack_b0;
  ulonglong uStack_a8;
  longlong lStack_a0;
  ulonglong uStack_98;
  undefined8 *puStack_90;
  ulonglong *puStack_88;
  undefined8 uStack_80;
  
  uStack_80 = 0xfffffffffffffffe;
  if (param_4 == (undefined8 *)0x0) {
    if (*(longlong *)(param_2 + 0x70) != 0) {
      param_1[1] = *(longlong *)(param_2 + 0x70);
      *param_1 = 0xb;
      return param_1;
    }
    puStack_100 = (undefined8 *)&UNK_142b12638;
    uStack_f8 = 1;
    iStack_f0 = 8;
    uStack_ec = 0;
    uStack_e8 = 0;
    uStack_e4 = 0;
    uStack_e0 = 0;
    uStack_dc = 0;
    FUN_1428d9390(&puStack_100,&UNK_142b12648);
  }
  else {
    puVar1 = (ulonglong *)(param_2 + 0x38);
    uVar30 = *(ulonglong *)(param_2 + 0x38);
    puStack_90 = param_1;
    if (uVar30 < 0xfffffffffffffff0 && (char)(((uint)uVar30 & 8) >> 3) == '\0') {
      LOCK();
      uVar28 = *puVar1;
      if (uVar30 == uVar28) {
        *puVar1 = uVar30 + 0x10;
      }
      UNLOCK();
      if (uVar30 != uVar28) goto code_r0x000140bdc642;
    }
    else {
code_r0x000140bdc642:
      FUN_1428cc4b0(puVar1,0);
    }
    FUN_140c81c70(puVar1);
    uStack_98 = (ulonglong)puVar1 | 1;
    puStack_88 = puVar1;
    FUN_140c81c70(uStack_98);
    lVar31 = lStack_a0;
    if (*(longlong *)(param_2 + 0x58) != 0) {
      lStack_b0 = param_3;
      uVar28 = FUN_1418bd030(param_2 + 0x60,param_4);
      lVar3 = *(longlong *)(param_2 + 0x40);
      uVar30 = *(ulonglong *)(param_2 + 0x48);
      uVar36 = uVar30 & uVar28;
      bVar37 = (byte)(uVar28 >> 0x39);
      auVar38 = ZEXT216(CONCAT11(bVar37,bVar37));
      auVar38 = pshuflw(auVar38,auVar38,0);
      cVar39 = auVar38[0];
      cVar40 = auVar38[1];
      cVar41 = auVar38[2];
      cVar42 = auVar38[3];
      lVar33 = 0;
      iVar6 = -(uint)(CONCAT13(unaff_XMM7_Bd,
                               CONCAT12(unaff_XMM7_Bc,CONCAT11(unaff_XMM7_Bb,unaff_XMM7_Ba))) ==
                     CONCAT13(unaff_XMM7_Bd,
                              CONCAT12(unaff_XMM7_Bc,CONCAT11(unaff_XMM7_Bb,unaff_XMM7_Ba))));
      iVar7 = -(uint)(CONCAT13(unaff_XMM7_Bh,
                               CONCAT12(unaff_XMM7_Bg,CONCAT11(unaff_XMM7_Bf,unaff_XMM7_Be))) ==
                     CONCAT13(unaff_XMM7_Bh,
                              CONCAT12(unaff_XMM7_Bg,CONCAT11(unaff_XMM7_Bf,unaff_XMM7_Be))));
      iVar8 = -(uint)(CONCAT13(unaff_XMM7_Bl,
                               CONCAT12(unaff_XMM7_Bk,CONCAT11(unaff_XMM7_Bj,unaff_XMM7_Bi))) ==
                     CONCAT13(unaff_XMM7_Bl,
                              CONCAT12(unaff_XMM7_Bk,CONCAT11(unaff_XMM7_Bj,unaff_XMM7_Bi))));
      iVar9 = -(uint)(CONCAT13(unaff_XMM7_Bp,
                               CONCAT12(unaff_XMM7_Bo,CONCAT11(unaff_XMM7_Bn,unaff_XMM7_Bm))) ==
                     CONCAT13(unaff_XMM7_Bp,
                              CONCAT12(unaff_XMM7_Bo,CONCAT11(unaff_XMM7_Bn,unaff_XMM7_Bm))));
      while( true ) {
        pcVar2 = (char *)(lVar3 + uVar36);
        cVar10 = *pcVar2;
        cVar11 = pcVar2[1];
        cVar12 = pcVar2[2];
        cVar13 = pcVar2[3];
        cVar14 = pcVar2[4];
        cVar15 = pcVar2[5];
        cVar16 = pcVar2[6];
        cVar17 = pcVar2[7];
        cVar18 = pcVar2[8];
        cVar19 = pcVar2[9];
        cVar20 = pcVar2[10];
        cVar21 = pcVar2[0xb];
        cVar22 = pcVar2[0xc];
        cVar23 = pcVar2[0xd];
        cVar24 = pcVar2[0xe];
        cVar25 = pcVar2[0xf];
        auVar38[0] = -(cVar10 == cVar39);
        auVar38[1] = -(cVar11 == cVar40);
        auVar38[2] = -(cVar12 == cVar41);
        auVar38[3] = -(cVar13 == cVar42);
        auVar38[4] = -(cVar14 == cVar39);
        auVar38[5] = -(cVar15 == cVar40);
        auVar38[6] = -(cVar16 == cVar41);
        auVar38[7] = -(cVar17 == cVar42);
        auVar38[8] = -(cVar18 == cVar39);
        auVar38[9] = -(cVar19 == cVar40);
        auVar38[10] = -(cVar20 == cVar41);
        auVar38[0xb] = -(cVar21 == cVar42);
        auVar38[0xc] = -(cVar22 == cVar39);
        auVar38[0xd] = -(cVar23 == cVar40);
        auVar38[0xe] = -(cVar24 == cVar41);
        auVar38[0xf] = -(cVar25 == cVar42);
        uVar27 = (ushort)(SUB161(auVar38 >> 7,0) & 1) | (ushort)(SUB161(auVar38 >> 0xf,0) & 1) << 1
                 | (ushort)(SUB161(auVar38 >> 0x17,0) & 1) << 2 |
                 (ushort)(SUB161(auVar38 >> 0x1f,0) & 1) << 3 |
                 (ushort)(SUB161(auVar38 >> 0x27,0) & 1) << 4 |
                 (ushort)(SUB161(auVar38 >> 0x2f,0) & 1) << 5 |
                 (ushort)(SUB161(auVar38 >> 0x37,0) & 1) << 6 |
                 (ushort)(SUB161(auVar38 >> 0x3f,0) & 1) << 7 |
                 (ushort)(SUB161(auVar38 >> 0x47,0) & 1) << 8 |
                 (ushort)(SUB161(auVar38 >> 0x4f,0) & 1) << 9 |
                 (ushort)(SUB161(auVar38 >> 0x57,0) & 1) << 10 |
                 (ushort)(SUB161(auVar38 >> 0x5f,0) & 1) << 0xb |
                 (ushort)(SUB161(auVar38 >> 0x67,0) & 1) << 0xc |
                 (ushort)(SUB161(auVar38 >> 0x6f,0) & 1) << 0xd |
                 (ushort)(SUB161(auVar38 >> 0x77,0) & 1) << 0xe | (ushort)(auVar38[0xf] >> 7) << 0xf
        ;
        uVar35 = (uint)uVar27;
        param_3 = lStack_b0;
        lVar31 = lStack_a0;
        while (lStack_a0 = lVar33, lStack_b0 = param_3, uVar27 != 0) {
          uVar34 = 0;
          for (uVar5 = uVar35; (uVar5 & 1) == 0; uVar5 = uVar5 >> 1 | 0x80000000) {
            uVar34 = uVar34 + 1;
          }
          uVar28 = uVar34 + uVar36 & uVar30;
          uStack_a8 = uVar30;
          cVar26 = func_0x0001418bd600(param_4,lVar3 + -0x48 + uVar28 * -0x48);
          puVar1 = puStack_88;
          puVar29 = puStack_90;
          if (cVar26 != '\0') {
            puStack_90[1] = *(undefined8 *)(lVar3 + -8 + uVar28 * -0x48);
            *puStack_90 = 0xb;
            FUN_140c81e10(puStack_88);
            FUN_140c81e10(uStack_98);
            LOCK();
            uVar30 = *puVar1;
            *puVar1 = *puVar1 - 0x10;
            UNLOCK();
            if ((uVar30 & 0xfffffffffffffff2) != 0x12) {
              return puVar29;
            }
            FUN_1428ccf50(puVar1);
            return puVar29;
          }
          uVar27 = (ushort)(uVar35 - 1) & (ushort)uVar35;
          uVar35 = CONCAT22((short)(uVar35 - 1 >> 0x10),uVar27);
          uVar30 = uStack_a8;
          lVar33 = lStack_a0;
          param_3 = lStack_b0;
          lVar31 = lStack_a0;
        }
        auVar43[0] = -(cVar10 == (char)iVar6);
        auVar43[1] = -(cVar11 == (char)((uint)iVar6 >> 8));
        auVar43[2] = -(cVar12 == (char)((uint)iVar6 >> 0x10));
        auVar43[3] = -(cVar13 == (char)((uint)iVar6 >> 0x18));
        auVar43[4] = -(cVar14 == (char)iVar7);
        auVar43[5] = -(cVar15 == (char)((uint)iVar7 >> 8));
        auVar43[6] = -(cVar16 == (char)((uint)iVar7 >> 0x10));
        auVar43[7] = -(cVar17 == (char)((uint)iVar7 >> 0x18));
        auVar43[8] = -(cVar18 == (char)iVar8);
        auVar43[9] = -(cVar19 == (char)((uint)iVar8 >> 8));
        auVar43[10] = -(cVar20 == (char)((uint)iVar8 >> 0x10));
        auVar43[0xb] = -(cVar21 == (char)((uint)iVar8 >> 0x18));
        auVar43[0xc] = -(cVar22 == (char)iVar9);
        auVar43[0xd] = -(cVar23 == (char)((uint)iVar9 >> 8));
        auVar43[0xe] = -(cVar24 == (char)((uint)iVar9 >> 0x10));
        auVar43[0xf] = -(cVar25 == (char)((uint)iVar9 >> 0x18));
        if ((((((((((((((((SUB161(auVar43 >> 7,0) & 1) != 0 || (SUB161(auVar43 >> 0xf,0) & 1) != 0)
                        || (SUB161(auVar43 >> 0x17,0) & 1) != 0) ||
                       (SUB161(auVar43 >> 0x1f,0) & 1) != 0) || (SUB161(auVar43 >> 0x27,0) & 1) != 0
                      ) || (SUB161(auVar43 >> 0x2f,0) & 1) != 0) ||
                    (SUB161(auVar43 >> 0x37,0) & 1) != 0) || (SUB161(auVar43 >> 0x3f,0) & 1) != 0)
                  || (SUB161(auVar43 >> 0x47,0) & 1) != 0) || (SUB161(auVar43 >> 0x4f,0) & 1) != 0)
                || (SUB161(auVar43 >> 0x57,0) & 1) != 0) || (SUB161(auVar43 >> 0x5f,0) & 1) != 0) ||
              (SUB161(auVar43 >> 0x67,0) & 1) != 0) || (SUB161(auVar43 >> 0x6f,0) & 1) != 0) ||
            (SUB161(auVar43 >> 0x77,0) & 1) != 0) || auVar43[0xf] < '\0') break;
        lVar33 = lStack_a0 + 0x10;
        uVar36 = uVar36 + lStack_a0 + 0x10 & uVar30;
        lStack_a0 = lVar31;
      }
    }
    lStack_a0 = lVar31;
    unaff_RDI = puStack_88;
    FUN_140c81e10(puStack_88);
    unaff_R12 = uStack_98;
    FUN_140c81e10(uStack_98);
    LOCK();
    uVar30 = *unaff_RDI;
    *unaff_RDI = *unaff_RDI - 0x10;
    UNLOCK();
    if ((uVar30 & 0xfffffffffffffff2) == 0x12) {
      FUN_1428ccf50(unaff_RDI);
    }
    uStack_148 = *param_4;
    uStack_140 = param_4[1];
    uStack_138 = *(undefined4 *)(param_4 + 2);
    uStack_134 = *(undefined4 *)((longlong)param_4 + 0x14);
    uStack_130 = *(undefined4 *)(param_4 + 3);
    uStack_12c = *(undefined4 *)((longlong)param_4 + 0x1c);
    uStack_128 = *(undefined4 *)(param_4 + 4);
    uStack_124 = *(undefined4 *)((longlong)param_4 + 0x24);
    uStack_120 = *(undefined4 *)(param_4 + 5);
    uStack_11c = *(undefined4 *)((longlong)param_4 + 0x2c);
    uStack_118 = *(undefined4 *)(param_4 + 6);
    uStack_114 = *(undefined4 *)((longlong)param_4 + 0x34);
    uStack_110 = *(undefined4 *)(param_4 + 7);
    FUN_140bdcba0(&puStack_100,*(undefined8 *)(param_3 + 0x140),*(undefined8 *)(param_3 + 0x5c8),
                  &uStack_148,param_2,*(undefined8 *)(param_2 + 0x30));
    if (puStack_100 != (undefined8 *)0xb) {
      puStack_90[8] = CONCAT44(uStack_bc,uStack_c0);
      puStack_90[9] = uStack_b8;
      puStack_90[6] = CONCAT44(uStack_cc,uStack_d0);
      puStack_90[7] = CONCAT44(uStack_c4,uStack_c8);
      puStack_90[4] = CONCAT44(uStack_dc,uStack_e0);
      puStack_90[5] = CONCAT44(uStack_d4,uStack_d8);
      puStack_90[2] = CONCAT44(uStack_ec,iStack_f0);
      puStack_90[3] = CONCAT44(uStack_e4,uStack_e8);
      *puStack_90 = puStack_100;
      puStack_90[1] = uStack_f8;
      return puStack_90;
    }
    param_2 = param_2 + 0x40;
    LOCK();
    uVar30 = *unaff_RDI;
    if (uVar30 == 0) {
      *unaff_RDI = 8;
    }
    UNLOCK();
    param_1 = puStack_90;
    unaff_R14 = param_4;
    unaff_R15 = uStack_f8;
    if (uVar30 == 0) goto code_r0x000140bdc834;
  }
  FUN_1428cb5f0(unaff_RDI);
code_r0x000140bdc834:
  FUN_140c81c70(unaff_RDI);
  FUN_140c81c70(unaff_R12);
  uStack_148 = *unaff_R14;
  uStack_140 = unaff_R14[1];
  uStack_138 = *(undefined4 *)(unaff_R14 + 2);
  uStack_134 = *(undefined4 *)((longlong)unaff_R14 + 0x14);
  uStack_130 = *(undefined4 *)(unaff_R14 + 3);
  uStack_12c = *(undefined4 *)((longlong)unaff_R14 + 0x1c);
  uStack_128 = *(undefined4 *)(unaff_R14 + 4);
  uStack_124 = *(undefined4 *)((longlong)unaff_R14 + 0x24);
  uStack_120 = *(undefined4 *)(unaff_R14 + 5);
  uStack_11c = *(undefined4 *)((longlong)unaff_R14 + 0x2c);
  uStack_118 = *(undefined4 *)(unaff_R14 + 6);
  uStack_114 = *(undefined4 *)((longlong)unaff_R14 + 0x34);
  uStack_110 = *(undefined4 *)(unaff_R14 + 7);
  FUN_1418ad1d0(&puStack_100,param_2,&uStack_148);
  puVar1 = puStack_88;
  puVar29 = puStack_100;
  if (iStack_f0 != 2) {
    pauVar4 = (undefined1 (*) [16])*puStack_100;
    uVar30 = puStack_100[1];
    uVar28 = uVar30 & uStack_f8;
    auVar38 = *(undefined1 (*) [16])(*pauVar4 + uVar28);
    uVar27 = (ushort)(SUB161(auVar38 >> 7,0) & 1) | (ushort)(SUB161(auVar38 >> 0xf,0) & 1) << 1 |
             (ushort)(SUB161(auVar38 >> 0x17,0) & 1) << 2 |
             (ushort)(SUB161(auVar38 >> 0x1f,0) & 1) << 3 |
             (ushort)(SUB161(auVar38 >> 0x27,0) & 1) << 4 |
             (ushort)(SUB161(auVar38 >> 0x2f,0) & 1) << 5 |
             (ushort)(SUB161(auVar38 >> 0x37,0) & 1) << 6 |
             (ushort)(SUB161(auVar38 >> 0x3f,0) & 1) << 7 |
             (ushort)(SUB161(auVar38 >> 0x47,0) & 1) << 8 |
             (ushort)(SUB161(auVar38 >> 0x4f,0) & 1) << 9 |
             (ushort)(SUB161(auVar38 >> 0x57,0) & 1) << 10 |
             (ushort)(SUB161(auVar38 >> 0x5f,0) & 1) << 0xb |
             (ushort)(SUB161(auVar38 >> 0x67,0) & 1) << 0xc |
             (ushort)(SUB161(auVar38 >> 0x6f,0) & 1) << 0xd |
             (ushort)(SUB161(auVar38 >> 0x77,0) & 1) << 0xe |
             (ushort)(byte)(auVar38[0xf] >> 7) << 0xf;
    uVar35 = (uint)uVar27;
    if (uVar27 == 0) {
      lVar31 = 0x10;
      do {
        uVar28 = uVar28 + lVar31 & uVar30;
        auVar38 = *(undefined1 (*) [16])(*pauVar4 + uVar28);
        lVar31 = lVar31 + 0x10;
        uVar35 = (uint)(ushort)((ushort)(SUB161(auVar38 >> 7,0) & 1) |
                                (ushort)(SUB161(auVar38 >> 0xf,0) & 1) << 1 |
                                (ushort)(SUB161(auVar38 >> 0x17,0) & 1) << 2 |
                                (ushort)(SUB161(auVar38 >> 0x1f,0) & 1) << 3 |
                                (ushort)(SUB161(auVar38 >> 0x27,0) & 1) << 4 |
                                (ushort)(SUB161(auVar38 >> 0x2f,0) & 1) << 5 |
                                (ushort)(SUB161(auVar38 >> 0x37,0) & 1) << 6 |
                                (ushort)(SUB161(auVar38 >> 0x3f,0) & 1) << 7 |
                                (ushort)(SUB161(auVar38 >> 0x47,0) & 1) << 8 |
                                (ushort)(SUB161(auVar38 >> 0x4f,0) & 1) << 9 |
                                (ushort)(SUB161(auVar38 >> 0x57,0) & 1) << 10 |
                                (ushort)(SUB161(auVar38 >> 0x5f,0) & 1) << 0xb |
                                (ushort)(SUB161(auVar38 >> 0x67,0) & 1) << 0xc |
                                (ushort)(SUB161(auVar38 >> 0x6f,0) & 1) << 0xd |
                                (ushort)(SUB161(auVar38 >> 0x77,0) & 1) << 0xe |
                               (ushort)(byte)(auVar38[0xf] >> 7) << 0xf);
      } while (uVar35 == 0);
    }
    uVar34 = 0;
    for (; (uVar35 & 1) == 0; uVar35 = uVar35 >> 1 | 0x80000000) {
      uVar34 = uVar34 + 1;
    }
    uVar28 = uVar34 + uVar28 & uVar30;
    bVar37 = (*pauVar4)[uVar28];
    if (-1 < (char)bVar37) {
      auVar38 = *pauVar4;
      uVar35 = 0;
      for (uVar34 = (uint)(ushort)((ushort)(SUB161(auVar38 >> 7,0) & 1) |
                                   (ushort)(SUB161(auVar38 >> 0xf,0) & 1) << 1 |
                                   (ushort)(SUB161(auVar38 >> 0x17,0) & 1) << 2 |
                                   (ushort)(SUB161(auVar38 >> 0x1f,0) & 1) << 3 |
                                   (ushort)(SUB161(auVar38 >> 0x27,0) & 1) << 4 |
                                   (ushort)(SUB161(auVar38 >> 0x2f,0) & 1) << 5 |
                                   (ushort)(SUB161(auVar38 >> 0x37,0) & 1) << 6 |
                                   (ushort)(SUB161(auVar38 >> 0x3f,0) & 1) << 7 |
                                   (ushort)(SUB161(auVar38 >> 0x47,0) & 1) << 8 |
                                   (ushort)(SUB161(auVar38 >> 0x4f,0) & 1) << 9 |
                                   (ushort)(SUB161(auVar38 >> 0x57,0) & 1) << 10 |
                                   (ushort)(SUB161(auVar38 >> 0x5f,0) & 1) << 0xb |
                                   (ushort)(SUB161(auVar38 >> 0x67,0) & 1) << 0xc |
                                   (ushort)(SUB161(auVar38 >> 0x6f,0) & 1) << 0xd |
                                   (ushort)(SUB161(auVar38 >> 0x77,0) & 1) << 0xe |
                                  (ushort)(byte)(auVar38[0xf] >> 7) << 0xf); (uVar34 & 1) == 0;
          uVar34 = uVar34 >> 1 | 0x80000000) {
        uVar35 = uVar35 + 1;
      }
      uVar28 = (ulonglong)uVar35;
      bVar37 = (*pauVar4)[uVar28];
    }
    bVar32 = (byte)(uStack_f8 >> 0x39);
    (*pauVar4)[uVar28] = bVar32;
    pauVar4[1][uVar28 - 0x10 & uVar30] = bVar32;
    puStack_100[2] = puStack_100[2] - (ulonglong)(bVar37 & 1);
    *(int *)((longlong)pauVar4 + uVar28 * -0x48 + -0x48) = iStack_f0;
    puVar29 = (undefined8 *)((longlong)pauVar4 + uVar28 * -0x48 + -0x44);
    *puVar29 = CONCAT44(uStack_e8,uStack_ec);
    puVar29[1] = CONCAT44(uStack_e0,uStack_e4);
    puVar29 = (undefined8 *)((longlong)pauVar4 + uVar28 * -0x48 + -0x34);
    *puVar29 = CONCAT44(uStack_d8,uStack_dc);
    puVar29[1] = CONCAT44(uStack_d0,uStack_d4);
    puVar29 = (undefined8 *)((longlong)pauVar4 + uVar28 * -0x48 + -0x24);
    *puVar29 = CONCAT44(uStack_c8,uStack_cc);
    puVar29[1] = CONCAT44(uStack_c0,uStack_c4);
    *(ulonglong *)((longlong)pauVar4 + uVar28 * -0x48 + -0x14) =
         CONCAT44((undefined4)uStack_b8,uStack_bc);
    *(ulonglong *)((longlong)pauVar4 + uVar28 * -0x48 + -8) = unaff_R15;
    puStack_100[3] = puStack_100[3] + 1;
    puVar29 = (undefined8 *)((longlong)pauVar4 + uVar28 * -0x48);
  }
  param_1[1] = puVar29[-1];
  *param_1 = 0xb;
  FUN_140c81e10(puStack_88);
  FUN_140c81e10();
  LOCK();
  uVar30 = *puVar1;
  if (uVar30 == 8) {
    *puVar1 = 0;
  }
  UNLOCK();
  if (uVar30 != 8) {
    FUN_1428cbf60(puVar1,0);
  }
  return param_1;
}

