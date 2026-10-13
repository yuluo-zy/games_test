
/* WARNING: Removing unreachable block (ram,0x000140b9d7c1) */
/* WARNING: Removing unreachable block (ram,0x000140b9d7c7) */
/* WARNING: Removing unreachable block (ram,0x000140b9d687) */
/* WARNING: Removing unreachable block (ram,0x000140b9d68d) */
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

undefined8 FUN_140b9d480(undefined8 param_1,longlong *param_2)

{
  char *pcVar1;
  longlong *plVar2;
  longlong lVar3;
  uint uVar4;
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
  ushort uVar26;
  int iVar27;
  longlong *plVar28;
  ulonglong uVar29;
  ulonglong uVar30;
  undefined8 uVar31;
  longlong *plVar32;
  undefined8 *puVar33;
  uint uVar34;
  longlong lVar35;
  byte bVar36;
  undefined1 auVar37 [16];
  char cVar38;
  char cVar39;
  char cVar40;
  char cVar41;
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
  undefined1 auVar42 [16];
  longlong *plStack_138;
  longlong *plStack_130;
  undefined8 uStack_128;
  undefined8 uStack_120;
  undefined8 uStack_118;
  undefined8 uStack_110;
  undefined8 uStack_108;
  undefined *puStack_100;
  undefined8 uStack_f8;
  undefined4 uStack_f0;
  undefined4 uStack_ec;
  undefined *puStack_e8;
  undefined8 uStack_e0;
  undefined8 **ppuStack_d8;
  undefined8 uStack_d0;
  undefined8 uStack_c8;
  undefined8 *puStack_b8;
  undefined *puStack_b0;
  undefined8 uStack_a8;
  longlong *plStack_a0;
  longlong *plStack_98;
  longlong *plStack_90;
  undefined1 uStack_81;
  undefined8 uStack_80;
  
  uStack_80 = 0xfffffffffffffffe;
  plStack_138 = (longlong *)&DAT_143634da8;
  plVar32 = param_2;
  uStack_a8 = param_1;
  plStack_a0 = param_2;
  plVar28 = (longlong *)FUN_141849000(&plStack_138);
  if (plVar32 == (longlong *)0x0) {
    plStack_98 = (longlong *)0x0;
    plStack_90 = (longlong *)0x0;
    if (plVar28 != (longlong *)0x0) {
      LOCK();
      if (*plVar28 == 0) {
        *plVar28 = 3;
      }
      UNLOCK();
    }
    if (*_DAT_14362c3a8 == 0) {
      return uStack_a8;
    }
    puStack_b8 = &uStack_a8;
    puStack_b0 = &UNK_1405338a0;
    puVar33 = (undefined8 *)func_0x00014067dd80(&UNK_142afd858);
    uStack_118 = *puVar33;
    uStack_110 = puVar33[1];
    uStack_ec = *(undefined4 *)(puVar33 + 2);
    uStack_108 = 1;
    puStack_100 = &UNK_142afd730;
    uStack_f8 = 8;
    puStack_e8 = &UNK_142afd848;
    uStack_e0 = 1;
    ppuStack_d8 = &puStack_b8;
    uStack_d0 = 1;
    uStack_c8 = 0;
    plStack_138 = (longlong *)0x0;
    plStack_130 = (longlong *)&UNK_142afd730;
    uStack_128 = 8;
    uStack_120 = 0;
    uStack_f0 = 1;
    func_0x000140d4a330(&uStack_81,&plStack_138);
    return uStack_a8;
  }
  plStack_98 = plVar28;
  plStack_90 = plVar32;
  if (plVar32[7] != 0) {
    uVar29 = FUN_1405b1a70(plVar32 + 8,param_1);
    lVar3 = plVar32[4];
    plVar32 = (longlong *)plVar32[5];
    bVar36 = (byte)(uVar29 >> 0x39);
    auVar37 = ZEXT216(CONCAT11(bVar36,bVar36));
    auVar37 = pshuflw(auVar37,auVar37,0);
    cVar38 = auVar37[0];
    cVar39 = auVar37[1];
    cVar40 = auVar37[2];
    cVar41 = auVar37[3];
    lVar35 = 0;
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
      uVar29 = (ulonglong)plVar32 & uVar29;
      pcVar1 = (char *)(lVar3 + uVar29);
      cVar10 = *pcVar1;
      cVar11 = pcVar1[1];
      cVar12 = pcVar1[2];
      cVar13 = pcVar1[3];
      cVar14 = pcVar1[4];
      cVar15 = pcVar1[5];
      cVar16 = pcVar1[6];
      cVar17 = pcVar1[7];
      cVar18 = pcVar1[8];
      cVar19 = pcVar1[9];
      cVar20 = pcVar1[10];
      cVar21 = pcVar1[0xb];
      cVar22 = pcVar1[0xc];
      cVar23 = pcVar1[0xd];
      cVar24 = pcVar1[0xe];
      cVar25 = pcVar1[0xf];
      auVar37[0] = -(cVar10 == cVar38);
      auVar37[1] = -(cVar11 == cVar39);
      auVar37[2] = -(cVar12 == cVar40);
      auVar37[3] = -(cVar13 == cVar41);
      auVar37[4] = -(cVar14 == cVar38);
      auVar37[5] = -(cVar15 == cVar39);
      auVar37[6] = -(cVar16 == cVar40);
      auVar37[7] = -(cVar17 == cVar41);
      auVar37[8] = -(cVar18 == cVar38);
      auVar37[9] = -(cVar19 == cVar39);
      auVar37[10] = -(cVar20 == cVar40);
      auVar37[0xb] = -(cVar21 == cVar41);
      auVar37[0xc] = -(cVar22 == cVar38);
      auVar37[0xd] = -(cVar23 == cVar39);
      auVar37[0xe] = -(cVar24 == cVar40);
      auVar37[0xf] = -(cVar25 == cVar41);
      uVar26 = (ushort)(SUB161(auVar37 >> 7,0) & 1) | (ushort)(SUB161(auVar37 >> 0xf,0) & 1) << 1 |
               (ushort)(SUB161(auVar37 >> 0x17,0) & 1) << 2 |
               (ushort)(SUB161(auVar37 >> 0x1f,0) & 1) << 3 |
               (ushort)(SUB161(auVar37 >> 0x27,0) & 1) << 4 |
               (ushort)(SUB161(auVar37 >> 0x2f,0) & 1) << 5 |
               (ushort)(SUB161(auVar37 >> 0x37,0) & 1) << 6 |
               (ushort)(SUB161(auVar37 >> 0x3f,0) & 1) << 7 |
               (ushort)(SUB161(auVar37 >> 0x47,0) & 1) << 8 |
               (ushort)(SUB161(auVar37 >> 0x4f,0) & 1) << 9 |
               (ushort)(SUB161(auVar37 >> 0x57,0) & 1) << 10 |
               (ushort)(SUB161(auVar37 >> 0x5f,0) & 1) << 0xb |
               (ushort)(SUB161(auVar37 >> 0x67,0) & 1) << 0xc |
               (ushort)(SUB161(auVar37 >> 0x6f,0) & 1) << 0xd |
               (ushort)(SUB161(auVar37 >> 0x77,0) & 1) << 0xe | (ushort)(auVar37[0xf] >> 7) << 0xf;
      uVar34 = (uint)uVar26;
      while (uVar26 != 0) {
        uVar4 = 0;
        for (uVar5 = uVar34; (uVar5 & 1) == 0; uVar5 = uVar5 >> 1 | 0x80000000) {
          uVar4 = uVar4 + 1;
        }
        uVar30 = uVar4 + uVar29 & (ulonglong)plVar32;
        if ((*(longlong **)(lVar3 + -0x18 + uVar30 * -0x28) == param_2) &&
           (iVar27 = func_0x000142923bf0(param_1,*(undefined8 *)(lVar3 + uVar30 * -0x28 + -0x20),
                                         param_2), iVar27 == 0)) {
          uVar31 = *(undefined8 *)(lVar3 + -0x10 + uVar30 * -0x28);
          goto joined_r0x000140b9d899;
        }
        uVar26 = (ushort)(uVar34 - 1) & (ushort)uVar34;
        uVar34 = CONCAT22((short)(uVar34 - 1 >> 0x10),uVar26);
      }
      auVar42[0] = -(cVar10 == (char)iVar6);
      auVar42[1] = -(cVar11 == (char)((uint)iVar6 >> 8));
      auVar42[2] = -(cVar12 == (char)((uint)iVar6 >> 0x10));
      auVar42[3] = -(cVar13 == (char)((uint)iVar6 >> 0x18));
      auVar42[4] = -(cVar14 == (char)iVar7);
      auVar42[5] = -(cVar15 == (char)((uint)iVar7 >> 8));
      auVar42[6] = -(cVar16 == (char)((uint)iVar7 >> 0x10));
      auVar42[7] = -(cVar17 == (char)((uint)iVar7 >> 0x18));
      auVar42[8] = -(cVar18 == (char)iVar8);
      auVar42[9] = -(cVar19 == (char)((uint)iVar8 >> 8));
      auVar42[10] = -(cVar20 == (char)((uint)iVar8 >> 0x10));
      auVar42[0xb] = -(cVar21 == (char)((uint)iVar8 >> 0x18));
      auVar42[0xc] = -(cVar22 == (char)iVar9);
      auVar42[0xd] = -(cVar23 == (char)((uint)iVar9 >> 8));
      auVar42[0xe] = -(cVar24 == (char)((uint)iVar9 >> 0x10));
      auVar42[0xf] = -(cVar25 == (char)((uint)iVar9 >> 0x18));
      if ((((((((((((((((SUB161(auVar42 >> 7,0) & 1) != 0 || (SUB161(auVar42 >> 0xf,0) & 1) != 0) ||
                      (SUB161(auVar42 >> 0x17,0) & 1) != 0) || (SUB161(auVar42 >> 0x1f,0) & 1) != 0)
                    || (SUB161(auVar42 >> 0x27,0) & 1) != 0) || (SUB161(auVar42 >> 0x2f,0) & 1) != 0
                   ) || (SUB161(auVar42 >> 0x37,0) & 1) != 0) ||
                 (SUB161(auVar42 >> 0x3f,0) & 1) != 0) || (SUB161(auVar42 >> 0x47,0) & 1) != 0) ||
               (SUB161(auVar42 >> 0x4f,0) & 1) != 0) || (SUB161(auVar42 >> 0x57,0) & 1) != 0) ||
             (SUB161(auVar42 >> 0x5f,0) & 1) != 0) || (SUB161(auVar42 >> 0x67,0) & 1) != 0) ||
           (SUB161(auVar42 >> 0x6f,0) & 1) != 0) || (SUB161(auVar42 >> 0x77,0) & 1) != 0) ||
          auVar42[0xf] < '\0') break;
      uVar29 = uVar29 + lVar35 + 0x10;
      lVar35 = lVar35 + 0x10;
    }
  }
  puStack_b8 = (undefined8 *)&DAT_143634db0;
  plVar28 = (longlong *)FUN_141849000(&puStack_b8);
  plStack_130 = plVar32;
  if (plVar32 == (longlong *)0x0) {
    plStack_138 = (longlong *)0x0;
    if (plVar28 != (longlong *)0x0) {
      LOCK();
      if (*plVar28 == 0) {
        *plVar28 = 3;
      }
      UNLOCK();
    }
    uVar31 = uStack_a8;
    if (*_DAT_14362c3a8 != 0) {
      puStack_b8 = &uStack_a8;
      puStack_b0 = &UNK_1405338a0;
      puVar33 = (undefined8 *)func_0x00014067dd80(&UNK_142afd7f8);
      uStack_118 = *puVar33;
      uStack_110 = puVar33[1];
      uStack_ec = *(undefined4 *)(puVar33 + 2);
      uStack_108 = 1;
      puStack_100 = &UNK_142afd730;
      uStack_f8 = 8;
      puStack_e8 = &UNK_142afd7e8;
      uStack_e0 = 1;
      uStack_d0 = 1;
      uStack_c8 = 0;
      plStack_138 = (longlong *)0x0;
      plStack_130 = (longlong *)&UNK_142afd730;
      uStack_128 = 8;
      uStack_120 = 0;
      uStack_f0 = 1;
      ppuStack_d8 = &puStack_b8;
      func_0x000140d4a330(&uStack_81,&plStack_138);
      uVar31 = uStack_a8;
    }
  }
  else {
    plStack_138 = plVar28;
    uVar31 = FUN_141848140(plVar32 + 2,uStack_a8,plStack_a0);
    plVar32 = plStack_138;
    plStack_138 = (longlong *)0x0;
    if (plVar32 != (longlong *)0x0) {
      plVar28 = plStack_130 + 2;
      if (plStack_130 == (longlong *)0x0) {
        plVar28 = (longlong *)0x0;
      }
      LOCK();
      plVar2 = (longlong *)*plVar32;
      if (plVar28 == plVar2) {
        *plVar32 = 3;
      }
      UNLOCK();
      if (plVar28 == plVar2) goto joined_r0x000140b9d899;
    }
    if (plStack_130 != (longlong *)0x0) {
      LOCK();
      *plStack_130 = *plStack_130 + -1;
      UNLOCK();
      if (*plStack_130 == 0) {
        FUN_1418491f0(&plStack_130);
      }
    }
  }
joined_r0x000140b9d899:
  if (plStack_98 != (longlong *)0x0) {
    plVar32 = plStack_90 + 2;
    if (plStack_90 == (longlong *)0x0) {
      plVar32 = (longlong *)0x0;
    }
    LOCK();
    plVar28 = (longlong *)*plStack_98;
    if (plVar32 == plVar28) {
      *plStack_98 = 3;
    }
    UNLOCK();
    if (plVar32 == plVar28) {
      return uVar31;
    }
  }
  plStack_98 = (longlong *)0x0;
  if (plStack_90 != (longlong *)0x0) {
    LOCK();
    *plStack_90 = *plStack_90 + -1;
    UNLOCK();
    if (*plStack_90 == 0) {
      FUN_1418491f0(&plStack_90);
    }
  }
  return uVar31;
}

