
void FUN_141a36700(undefined8 *param_1,longlong *param_2,undefined8 *param_3,longlong *param_4,
                  undefined8 *param_5)

{
  byte *pbVar1;
  byte bVar2;
  byte bVar3;
  longlong lVar4;
  longlong lVar5;
  code *pcVar6;
  uint uVar7;
  uint uVar8;
  int iVar9;
  int iVar10;
  int iVar11;
  int iVar12;
  char cVar13;
  uint uVar14;
  int iVar15;
  longlong lVar16;
  longlong *plVar17;
  undefined8 *puVar18;
  longlong lVar19;
  longlong lVar20;
  longlong lVar21;
  int *piVar22;
  ulonglong uVar23;
  ulonglong uVar24;
  char *pcVar25;
  ushort uVar26;
  ulonglong uVar27;
  ulonglong unaff_RDI;
  ulonglong uVar28;
  undefined *puVar29;
  undefined *puVar30;
  undefined8 uVar31;
  longlong lVar32;
  undefined *puVar33;
  longlong lVar34;
  longlong *plVar35;
  undefined1 uVar36;
  bool bVar37;
  byte bVar38;
  char cVar40;
  char cVar41;
  char cVar42;
  undefined1 auVar39 [16];
  undefined1 auVar43 [16];
  undefined1 auVar44 [16];
  undefined1 unaff_XMM6_Ba;
  undefined1 unaff_XMM6_Bb;
  undefined1 unaff_XMM6_Bc;
  undefined1 unaff_XMM6_Bd;
  undefined1 unaff_XMM6_Be;
  undefined1 unaff_XMM6_Bf;
  undefined1 unaff_XMM6_Bg;
  undefined1 unaff_XMM6_Bh;
  undefined1 unaff_XMM6_Bi;
  undefined1 unaff_XMM6_Bj;
  undefined1 unaff_XMM6_Bk;
  undefined1 unaff_XMM6_Bl;
  undefined1 unaff_XMM6_Bm;
  undefined1 unaff_XMM6_Bn;
  undefined1 unaff_XMM6_Bo;
  undefined1 unaff_XMM6_Bp;
  undefined8 uStack_188;
  undefined8 uStack_180;
  undefined8 uStack_178;
  undefined8 uStack_168;
  undefined8 uStack_160;
  undefined *puStack_158;
  undefined8 uStack_150;
  longlong lStack_148;
  ulonglong uStack_140;
  undefined8 uStack_138;
  undefined8 uStack_130;
  longlong lStack_128;
  undefined8 uStack_120;
  undefined8 uStack_118;
  longlong lStack_110;
  undefined8 uStack_108;
  undefined8 uStack_100;
  int iStack_f8;
  ulonglong *puStack_e0;
  undefined8 uStack_d8;
  longlong lStack_d0;
  longlong *plStack_c8;
  longlong *plStack_c0;
  longlong *plStack_b8;
  longlong *plStack_b0;
  undefined8 uStack_a8;
  longlong lStack_a0;
  undefined8 uStack_98;
  undefined8 *puStack_90;
  longlong lStack_88;
  longlong *plStack_80;
  undefined4 uStack_74;
  longlong *plStack_70;
  longlong *plStack_68;
  undefined8 uStack_60;
  
  uStack_60 = 0xfffffffffffffffe;
  plVar35 = param_2;
  cVar13 = func_0x000140c16a00();
  if (cVar13 == '\0') {
LAB_141a3679f:
    lStack_a0 = 0;
  }
  else {
    unaff_RDI = 0x3d;
    lVar16 = FUN_141a36210(0x3d);
    puVar33 = &UNK_142eba60b;
    if (lVar16 == 1) {
      if (plVar35 != (longlong *)0x0) {
        if (plVar35 < (longlong *)0x3d) {
          if (*(char *)((longlong)plVar35 + 0x142eba60b) < -0x40) {
LAB_141a3677f:
            FUN_1428d9940(&UNK_142eba60b,0x3d,0,plVar35,&UNK_142eba6d0);
            goto LAB_141a3679f;
          }
        }
        else if (plVar35 != (longlong *)0x3d) goto LAB_141a3677f;
      }
      lVar16 = FUN_141a36210(plVar35);
      if (lVar16 == 1) {
        uVar28 = (longlong)plVar35 + 2;
        if (uVar28 != 0) {
          uVar36 = uVar28 == 0x3d;
          if (0x3c < uVar28) goto LAB_141a367f6;
          uVar36 = *(char *)((longlong)plVar35 + 0x142eba60d) == -0x41;
          if (*(char *)((longlong)plVar35 + 0x142eba60d) < -0x40) {
            do {
              plVar35 = (longlong *)0x3d;
              FUN_1428d9940(&UNK_142eba60b);
LAB_141a367f6:
            } while (!(bool)uVar36);
          }
        }
        unaff_RDI = 0x3b - (longlong)plVar35;
        puVar33 = &UNK_142eba60b + uVar28;
      }
    }
    puVar29 = &UNK_142eba815;
    do {
      bVar38 = puVar29[-1];
      uVar14 = (uint)(char)bVar38;
      plStack_68 = param_4;
      if ((char)bVar38 < '\0') {
        bVar2 = puVar29[-2];
        if ((char)bVar2 < -0x40) {
          bVar3 = puVar29[-3];
          if ((char)bVar3 < -0x40) {
            pbVar1 = puVar29 + -4;
            puVar29 = puVar29 + -4;
            uVar14 = bVar3 & 0x3f | (*pbVar1 & 7) << 6;
          }
          else {
            puVar29 = puVar29 + -3;
            uVar14 = bVar3 & 0xf;
          }
          uVar14 = bVar2 & 0x3f | uVar14 << 6;
        }
        else {
          puVar29 = puVar29 + -2;
          uVar14 = bVar2 & 0x1f;
        }
        uVar14 = (uint)(bVar38 & 0x3f) | uVar14 << 6;
        if (uVar14 != 0x5c) goto LAB_141a36894;
LAB_141a368a6:
        puVar30 = puVar29 + -0x142eba7df;
        if (puVar30 != (undefined *)0x0) {
          uVar36 = puVar30 == (undefined *)0x35;
          if ((undefined *)0x34 < puVar30) goto LAB_141a368e5;
          uVar36 = puVar29[1] == -0x41;
          if ((char)puVar29[1] < -0x40) {
            do {
              FUN_1428d9940(&UNK_142eba7e0,0x35);
LAB_141a368e5:
            } while (!(bool)uVar36);
          }
        }
        lVar16 = 0x35 - (longlong)puVar30;
        param_4 = (longlong *)(&UNK_142eba7e0 + (longlong)puVar30);
        goto LAB_141a368f3;
      }
      puVar29 = puVar29 + -1;
      if (uVar14 == 0x5c) goto LAB_141a368a6;
LAB_141a36894:
      if (uVar14 == 0x2f) goto LAB_141a368a6;
    } while (puVar29 != &UNK_142eba7e0);
    lVar16 = 0x35;
    param_4 = (longlong *)&UNK_142eba7e0;
LAB_141a368f3:
    plVar35 = (longlong *)func_0x000140c18130();
    if (*plVar35 == 1) {
      plVar35 = plVar35 + 1;
    }
    else if ((*plVar35 != 0) ||
            (plVar35 = (longlong *)FUN_142918340(plVar35,0), plVar35 == (longlong *)0x0)) {
LAB_141a36ffa:
      FUN_1428d80d0(&UNK_142ff64e8);
LAB_141a37006:
      lStack_148 = 0;
      FUN_1428cab41(0,&puStack_158,param_4,&lStack_148,&UNK_142ebb348);
      goto LAB_141a370e1;
    }
    if (*plVar35 != 0) {
LAB_141a37030:
      FUN_1428d9250(&UNK_142ff6558);
LAB_141a3703c:
      FUN_1428d9310(&UNK_142eba818);
LAB_141a370e1:
                    /* WARNING: Does not return */
      pcVar6 = (code *)invalidInstructionException();
      (*pcVar6)();
    }
    *plVar35 = -1;
    plStack_70 = plVar35;
    uStack_98 = FUN_140c16d40(plVar35 + 1,puVar33,unaff_RDI,param_4,lVar16,1,0);
    *plStack_70 = *plStack_70 + 1;
    lStack_a0 = 1;
    param_4 = plStack_68;
  }
  puStack_e0 = (ulonglong *)*param_1;
  lVar16 = param_1[1];
  uVar28 = *puStack_e0;
  uVar27 = 0;
  uVar23 = uVar28 - *(ulonglong *)(lVar16 + 0x18);
  if (uVar28 < *(ulonglong *)(lVar16 + 0x18)) {
    uVar23 = uVar27;
  }
  uVar24 = uVar28 - *(ulonglong *)(lVar16 + 0x38);
  if (uVar28 < *(ulonglong *)(lVar16 + 0x38)) {
    uVar24 = uVar27;
  }
  plVar35 = (longlong *)(uVar23 * 0x20 + *(longlong *)(lVar16 + 8));
  uVar28 = *(ulonglong *)(lVar16 + 0x10) - uVar23;
  if (*(ulonglong *)(lVar16 + 0x10) < uVar23) {
    plVar35 = (longlong *)0x8;
    uVar28 = uVar27;
  }
  plVar17 = (longlong *)(uVar24 * 0x20 + *(longlong *)(lVar16 + 0x28));
  uVar23 = *(ulonglong *)(lVar16 + 0x30) - uVar24;
  if (*(ulonglong *)(lVar16 + 0x30) < uVar24) {
    plVar17 = (longlong *)0x8;
    uVar23 = uVar27;
  }
  *puStack_e0 = (*(longlong *)(lVar16 + 0x40) - uVar23) - uVar28;
  plStack_c8 = plVar35 + uVar28 * 4;
  plStack_c0 = plVar17 + uVar23 * 4;
  uStack_d8 = *param_3;
  lVar16 = *param_2;
  lStack_d0 = param_2[1];
  uStack_74 = *(undefined4 *)((longlong)param_2 + 0x14);
  plStack_b8 = (longlong *)*param_4;
  plStack_b0 = plStack_b8 + 4;
  uStack_a8 = *param_5;
  iVar9 = -(uint)(CONCAT13(unaff_XMM6_Bd,
                           CONCAT12(unaff_XMM6_Bc,CONCAT11(unaff_XMM6_Bb,unaff_XMM6_Ba))) ==
                 CONCAT13(unaff_XMM6_Bd,
                          CONCAT12(unaff_XMM6_Bc,CONCAT11(unaff_XMM6_Bb,unaff_XMM6_Ba))));
  iVar10 = -(uint)(CONCAT13(unaff_XMM6_Bh,
                            CONCAT12(unaff_XMM6_Bg,CONCAT11(unaff_XMM6_Bf,unaff_XMM6_Be))) ==
                  CONCAT13(unaff_XMM6_Bh,
                           CONCAT12(unaff_XMM6_Bg,CONCAT11(unaff_XMM6_Bf,unaff_XMM6_Be))));
  iVar11 = -(uint)(CONCAT13(unaff_XMM6_Bl,
                            CONCAT12(unaff_XMM6_Bk,CONCAT11(unaff_XMM6_Bj,unaff_XMM6_Bi))) ==
                  CONCAT13(unaff_XMM6_Bl,
                           CONCAT12(unaff_XMM6_Bk,CONCAT11(unaff_XMM6_Bj,unaff_XMM6_Bi))));
  iVar12 = -(uint)(CONCAT13(unaff_XMM6_Bp,
                            CONCAT12(unaff_XMM6_Bo,CONCAT11(unaff_XMM6_Bn,unaff_XMM6_Bm))) ==
                  CONCAT13(unaff_XMM6_Bp,
                           CONCAT12(unaff_XMM6_Bo,CONCAT11(unaff_XMM6_Bn,unaff_XMM6_Bm))));
LAB_141a36a4e:
  if ((plVar35 == (longlong *)0x0) || (plVar35 == plStack_c8)) {
    if ((plVar17 == (longlong *)0x0) || (plVar17 == plStack_c0)) {
      if (lStack_a0 != 0) {
        plVar35 = (longlong *)func_0x000140c18130();
        if (*plVar35 == 1) {
          plVar35 = plVar35 + 1;
        }
        else if ((*plVar35 != 0) ||
                (plVar35 = (longlong *)FUN_142918340(plVar35,0), plVar35 == (longlong *)0x0))
        goto LAB_141a36ffa;
        if (*plVar35 != 0) goto LAB_141a37030;
        *plVar35 = -1;
        plStack_68 = plVar35;
        FUN_140c170f0(plVar35 + 1,uStack_98);
        *plStack_68 = *plStack_68 + 1;
      }
      return;
    }
    plStack_68 = plVar17 + 4;
    plStack_70 = (longlong *)0x0;
    plVar35 = plVar17;
  }
  else {
    plStack_70 = plVar35 + 4;
    plStack_68 = plVar17;
  }
  *puStack_e0 = *puStack_e0 + 1;
  puVar18 = (undefined8 *)FUN_1421aeb40(uStack_d8,&UNK_142ebb2c0);
  if (puVar18 == (undefined8 *)0x0) goto LAB_141a3703c;
  puStack_158 = &UNK_142ff7e28;
  uStack_150 = 0x35;
  param_4 = puVar18 + 2;
  if ((puVar18[3] != 0x35) ||
     (iVar15 = func_0x000142923bf0(&UNK_142ff7e28,*param_4,0x35), iVar15 != 0)) goto LAB_141a37006;
  lVar19 = (**(code **)(puVar18[1] + 0x18))(*puVar18);
  if (lVar19 == 0) goto LAB_141a3703c;
  uVar28 = plVar35[1];
  uVar23 = uVar28 & 0xffffffff;
  if ((*(ulonglong *)(lStack_d0 + 0x10) <= uVar23) ||
     (*(int *)(*(longlong *)(lStack_d0 + 8) + uVar23 * 0x14) != (int)(uVar28 >> 0x20))) {
LAB_141a3705d:
    lStack_148 = (unaff_RDI & 0xffffffff00000000) + 1;
LAB_141a37077:
    uStack_140 = uVar28;
    FUN_1428d9760(&UNK_142eba5e0,0x2b,&lStack_148,&UNK_142eba5c0,&UNK_142eba830);
    goto LAB_141a370e1;
  }
  lVar32 = *(longlong *)(lStack_d0 + 8) + uVar23 * 0x14;
  uVar23 = (ulonglong)*(uint *)(lVar32 + 4);
  if (uVar23 == 0xffffffff) goto LAB_141a3705d;
  if ((*(ulonglong *)(lVar16 + 0x38) <= uVar23) ||
     ((*(ulonglong *)(*(longlong *)(lVar16 + 0x28) + (ulonglong)(*(uint *)(lVar32 + 4) >> 6) * 8) >>
       (uVar23 & 0x3f) & 1) == 0)) {
    lStack_148 = uVar23 << 0x20;
    goto LAB_141a37077;
  }
  uVar28 = (ulonglong)*(uint *)(lVar32 + 0xc);
  lVar34 = *(longlong *)(lStack_d0 + 0x1a8);
  lVar4 = *(longlong *)(lVar34 + 0x18 + uVar28 * 0x48);
  lVar5 = *(longlong *)(lVar34 + 0x38 + uVar28 * 0x48);
  if ((*(ulonglong *)(lVar16 + 0x118) < *(ulonglong *)(lVar34 + 0x40 + uVar28 * 0x48)) &&
     (uVar28 = *(ulonglong *)(lVar5 + *(ulonglong *)(lVar16 + 0x118) * 8), uVar28 != 0)) {
    lVar34 = *(longlong *)(lVar4 + 0x10 + ~uVar28 * 0x30);
  }
  else {
    lVar34 = 0;
  }
  uVar28 = (ulonglong)*(uint *)(lVar32 + 0x10);
  lVar20 = ~*(ulonglong *)(lVar5 + *(longlong *)(lVar16 + 0x110) * 8) * 0x30;
  lVar32 = *(longlong *)(lVar4 + 0x10 + lVar20);
  lVar5 = *(longlong *)(lVar4 + 0x20 + lVar20);
  lVar4 = *(longlong *)(lVar4 + 0x28 + lVar20);
  plVar35 = (longlong *)(lVar32 + uVar28 * 0x10);
  if (*(longlong *)(lVar32 + uVar28 * 0x10) == 0) {
    puVar18 = (undefined8 *)func_0x000140613c10(0x20,8);
    if (puVar18 != (undefined8 *)0x0) {
      *puVar18 = 0;
      puVar18[1] = 4;
      puVar18[2] = 0;
      *(undefined1 *)(puVar18 + 3) = 0;
      lVar32 = *(longlong *)(lVar19 + 0x60);
      do {
        lVar20 = *(longlong *)(lVar32 + 8);
        while (lVar20 != -1) {
          if (lVar20 < 0) {
            FUN_1428a9a00(&UNK_142eab990,&UNK_142eab9f0);
            goto LAB_141a370e1;
          }
          LOCK();
          lVar21 = *(longlong *)(lVar32 + 8);
          bVar37 = lVar20 == lVar21;
          if (bVar37) {
            *(longlong *)(lVar32 + 8) = lVar20 + 1;
            lVar21 = lVar20;
          }
          UNLOCK();
          lVar20 = lVar21;
          if (bVar37) {
            *(undefined4 *)(lVar4 + uVar28 * 4) = uStack_74;
            puStack_90 = puVar18;
            lStack_88 = lVar32;
            plStack_80 = plVar35;
            if (*plVar35 != 0) {
              FUN_140dcfcb0(plVar35);
            }
            *plStack_80 = lStack_88;
            plStack_80[1] = (longlong)puStack_90;
            plVar35 = plStack_80;
            goto LAB_141a36cc8;
          }
        }
      } while( true );
    }
    FUN_1428d9000(8,0x20);
    goto LAB_141a370e1;
  }
LAB_141a36cc8:
  *(undefined4 *)(lVar4 + uVar28 * 4) = uStack_74;
  if (*plVar35 == 0) {
    FUN_1428d9310(&UNK_142eba848);
    goto LAB_141a370e1;
  }
  lVar32 = plVar35[1];
  if (lVar32 == 0) {
    FUN_1428d9310(&UNK_142ebb400);
    goto LAB_141a370e1;
  }
  LOCK();
  bVar37 = *(char *)(lVar32 + 0x18) == '\0';
  if (bVar37) {
    *(char *)(lVar32 + 0x18) = '\x01';
  }
  UNLOCK();
  if (!bVar37) {
    FUN_1428cd210(lVar32 + 0x18);
  }
  FUN_140c81c70(lVar32 + 0x18);
  piVar22 = (int *)(*(longlong *)(lVar32 + 0x10) * 4 + *(longlong *)(lVar32 + 8) + -4);
  if (piVar22 == (int *)0x0 || *(longlong *)(lVar32 + 0x10) == 0) {
    uVar23 = *(ulonglong *)(lVar19 + 0x28);
    if (uVar23 == 0) goto LAB_141a370c4;
    iStack_f8 = (int)uVar23 + -1;
  }
  else {
    iStack_f8 = *piVar22;
    uVar23 = *(ulonglong *)(lVar19 + 0x28);
  }
  lStack_148 = *(longlong *)(lVar19 + 0x20);
  uStack_138 = *(undefined8 *)(lVar19 + 0x30);
  uStack_130 = *(undefined8 *)(lVar19 + 0x38);
  uStack_120 = *(undefined8 *)(lVar19 + 0x40);
  uStack_118 = *(undefined8 *)(lVar19 + 0x48);
  uStack_108 = *(undefined8 *)(lVar19 + 0x50);
  uStack_100 = *(undefined8 *)(lVar19 + 0x58);
  uStack_140 = uVar23;
  lStack_128 = lVar32;
  lStack_110 = lVar19;
  FUN_141a3c350(&lStack_148);
  param_4 = (longlong *)(lVar34 + uVar28 * 0x58);
  cVar13 = func_0x0001408e3380(param_4);
  plVar35 = plStack_b8;
  unaff_RDI = lVar5 + uVar28 * 4;
  if (cVar13 == '\0') {
    if (plStack_b8[3] != 0) {
      uVar23 = FUN_1405b0d70(plStack_b0,param_4);
      lVar19 = *plVar35;
      uVar28 = plVar35[1];
      bVar38 = (byte)(uVar23 >> 0x39);
      auVar39 = ZEXT216(CONCAT11(bVar38,bVar38));
      auVar39 = pshuflw(auVar39,auVar39,0);
      lVar32 = 0;
      while( true ) {
        uVar23 = uVar28 & uVar23;
        pcVar25 = (char *)(lVar19 + uVar23);
        cVar13 = auVar39[0];
        auVar44[0] = -(*pcVar25 == cVar13);
        cVar40 = auVar39[1];
        auVar44[1] = -(pcVar25[1] == cVar40);
        cVar41 = auVar39[2];
        auVar44[2] = -(pcVar25[2] == cVar41);
        cVar42 = auVar39[3];
        auVar44[3] = -(pcVar25[3] == cVar42);
        auVar44[4] = -(pcVar25[4] == cVar13);
        auVar44[5] = -(pcVar25[5] == cVar40);
        auVar44[6] = -(pcVar25[6] == cVar41);
        auVar44[7] = -(pcVar25[7] == cVar42);
        auVar44[8] = -(pcVar25[8] == cVar13);
        auVar44[9] = -(pcVar25[9] == cVar40);
        auVar44[10] = -(pcVar25[10] == cVar41);
        auVar44[0xb] = -(pcVar25[0xb] == cVar42);
        auVar44[0xc] = -(pcVar25[0xc] == cVar13);
        auVar44[0xd] = -(pcVar25[0xd] == cVar40);
        auVar44[0xe] = -(pcVar25[0xe] == cVar41);
        auVar44[0xf] = -(pcVar25[0xf] == cVar42);
        uVar26 = (ushort)(SUB161(auVar44 >> 7,0) & 1) | (ushort)(SUB161(auVar44 >> 0xf,0) & 1) << 1
                 | (ushort)(SUB161(auVar44 >> 0x17,0) & 1) << 2 |
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
                 (ushort)(SUB161(auVar44 >> 0x77,0) & 1) << 0xe | (ushort)(auVar44[0xf] >> 7) << 0xf
        ;
        uVar14 = (uint)uVar26;
        if (uVar26 != 0) {
          do {
            uVar7 = 0;
            for (uVar8 = uVar14; (uVar8 & 1) == 0; uVar8 = uVar8 >> 1 | 0x80000000) {
              uVar7 = uVar7 + 1;
            }
            uVar27 = uVar7 + uVar23 & uVar28;
            if (*param_4 == *(longlong *)(lVar19 + -0x20 + uVar27 * -0x20)) {
              uVar31 = CONCAT71((int7)(uVar23 >> 8),*(longlong *)(lVar19 + -8 + uVar27 * -0x20) != 0
                               );
              bVar38 = *(byte *)(param_4 + 1);
              goto joined_r0x000141a36ee6;
            }
            uVar26 = (ushort)(uVar14 - 1) & (ushort)uVar14;
            uVar14 = CONCAT22((short)(uVar14 - 1 >> 0x10),uVar26);
          } while (uVar26 != 0);
        }
        auVar43[0] = -(*pcVar25 == (char)iVar9);
        auVar43[1] = -(pcVar25[1] == (char)((uint)iVar9 >> 8));
        auVar43[2] = -(pcVar25[2] == (char)((uint)iVar9 >> 0x10));
        auVar43[3] = -(pcVar25[3] == (char)((uint)iVar9 >> 0x18));
        auVar43[4] = -(pcVar25[4] == (char)iVar10);
        auVar43[5] = -(pcVar25[5] == (char)((uint)iVar10 >> 8));
        auVar43[6] = -(pcVar25[6] == (char)((uint)iVar10 >> 0x10));
        auVar43[7] = -(pcVar25[7] == (char)((uint)iVar10 >> 0x18));
        auVar43[8] = -(pcVar25[8] == (char)iVar11);
        auVar43[9] = -(pcVar25[9] == (char)((uint)iVar11 >> 8));
        auVar43[10] = -(pcVar25[10] == (char)((uint)iVar11 >> 0x10));
        auVar43[0xb] = -(pcVar25[0xb] == (char)((uint)iVar11 >> 0x18));
        auVar43[0xc] = -(pcVar25[0xc] == (char)iVar12);
        auVar43[0xd] = -(pcVar25[0xd] == (char)((uint)iVar12 >> 8));
        auVar43[0xe] = -(pcVar25[0xe] == (char)((uint)iVar12 >> 0x10));
        auVar43[0xf] = -(pcVar25[0xf] == (char)((uint)iVar12 >> 0x18));
        if ((((((((((((((((SUB161(auVar43 >> 7,0) & 1) != 0 || (SUB161(auVar43 >> 0xf,0) & 1) != 0)
                        || (SUB161(auVar43 >> 0x17,0) & 1) != 0) ||
                       (SUB161(auVar43 >> 0x1f,0) & 1) != 0) || (SUB161(auVar43 >> 0x27,0) & 1) != 0
                      ) || (SUB161(auVar43 >> 0x2f,0) & 1) != 0) ||
                    (SUB161(auVar43 >> 0x37,0) & 1) != 0) || (SUB161(auVar43 >> 0x3f,0) & 1) != 0)
                  || (SUB161(auVar43 >> 0x47,0) & 1) != 0) || (SUB161(auVar43 >> 0x4f,0) & 1) != 0)
                || (SUB161(auVar43 >> 0x57,0) & 1) != 0) || (SUB161(auVar43 >> 0x5f,0) & 1) != 0) ||
              (SUB161(auVar43 >> 0x67,0) & 1) != 0) || (SUB161(auVar43 >> 0x6f,0) & 1) != 0) ||
            (SUB161(auVar43 >> 0x77,0) & 1) != 0) || auVar43[0xf] < '\0') break;
        uVar23 = uVar23 + lVar32 + 0x10;
        lVar32 = lVar32 + 0x10;
      }
    }
    uVar31 = 0;
    bVar38 = *(byte *)(param_4 + 1);
joined_r0x000141a36ee6:
    if ((bVar38 & 1) == 0) {
      uStack_168 = *(undefined8 *)((longlong)param_4 + 0xc);
      uStack_160 = *(undefined8 *)((longlong)param_4 + 0x14);
      FUN_1421b3120(&uStack_168,param_4,uVar31,&lStack_148);
    }
    else {
      uStack_178 = *(undefined8 *)((longlong)param_4 + 0x1c);
      uStack_188 = *(undefined8 *)((longlong)param_4 + 0xc);
      uStack_180 = *(undefined8 *)((longlong)param_4 + 0x14);
      FUN_1421aeef0(&uStack_188,param_4,uVar31,uStack_a8,&lStack_148);
    }
    pcVar25 = (char *)(lStack_128 + 0x18);
    FUN_140c81e10();
    LOCK();
    cVar13 = *pcVar25;
    if (cVar13 == '\x01') {
      *pcVar25 = '\0';
    }
    UNLOCK();
    plVar17 = plStack_68;
    plVar35 = plStack_70;
    if (cVar13 != '\x01') {
      FUN_1428cdb90(pcVar25);
      plVar17 = plStack_68;
      plVar35 = plStack_70;
    }
  }
  else {
    pcVar25 = (char *)(lStack_128 + 0x18);
    FUN_140c81e10();
    LOCK();
    cVar13 = *pcVar25;
    if (cVar13 == '\x01') {
      *pcVar25 = '\0';
    }
    UNLOCK();
    plVar17 = plStack_68;
    plVar35 = plStack_70;
    if (cVar13 != '\x01') {
      FUN_1428cdb90(pcVar25);
      plVar17 = plStack_68;
      plVar35 = plStack_70;
    }
  }
  goto LAB_141a36a4e;
LAB_141a370c4:
  FUN_1428d9310(&UNK_142ebb418);
  goto LAB_141a370e1;
}

