/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */
void FUN_1412a0820(longlong *param_1,undefined8 *param_2,undefined8 *param_3,longlong *param_4,
                  undefined8 *param_5,longlong *param_6)
{
  char *pcVar1;
  undefined1 auVar2 [16];
  longlong lVar3;
  undefined1 (*pauVar4) [16];
  undefined8 uVar5;
  code *pcVar6;
  int iVar7;
  uint uVar8;
  int iVar9;
  int iVar10;
  int iVar11;
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
  char cVar27;
  undefined1 auVar28 [16];
  char cVar29;
  uint uVar30;
  int iVar31;
  undefined4 uVar32;
  longlong *plVar33;
  longlong lVar34;
  undefined8 *puVar35;
  ulonglong uVar36;
  ulonglong uVar37;
  undefined1 (*pauVar38) [16];
  undefined1 (*pauVar39) [16];
  undefined8 *puVar40;
  ulonglong uVar41;
  longlong lVar42;
  undefined1 (*pauVar43) [16];
  ulonglong uVar44;
  uint uVar45;
  undefined8 *puVar46;
  ulonglong *puVar47;
  ulonglong *puVar48;
  undefined1 (*pauVar49) [16];
  ulonglong uVar50;
  ulonglong uVar51;
  ushort uVar52;
  undefined1 (*unaff_R12) [16];
  longlong lVar53;
  undefined8 *puVar54;
  bool bVar55;
  byte bVar56;
  char cVar57;
  char cVar60;
  char cVar61;
  char cVar62;
  undefined1 auVar58 [16];
  undefined1 auVar59 [16];
  undefined1 auVar63 [16];
  undefined1 auVar64 [16];
  undefined1 auVar65 [16];
  undefined1 auVar66 [16];
  undefined1 auVar67 [16];
  undefined1 unaff_XMM9_Ba;
  undefined1 unaff_XMM9_Bb;
  undefined1 unaff_XMM9_Bc;
  char cVar68;
  undefined1 unaff_XMM9_Bd;
  char cVar69;
  undefined1 unaff_XMM9_Be;
  undefined1 unaff_XMM9_Bf;
  char cVar70;
  undefined1 unaff_XMM9_Bg;
  char cVar71;
  undefined1 unaff_XMM9_Bh;
  char cVar72;
  undefined1 unaff_XMM9_Bi;
  undefined1 unaff_XMM9_Bj;
  char cVar73;
  undefined1 unaff_XMM9_Bk;
  char cVar74;
  undefined1 unaff_XMM9_Bl;
  char cVar75;
  undefined1 unaff_XMM9_Bm;
  undefined1 unaff_XMM9_Bn;
  char cVar76;
  undefined1 unaff_XMM9_Bo;
  char cVar77;
  undefined1 unaff_XMM9_Bp;
  char cVar78;
  undefined1 auVar79 [16];
  undefined1 (*pauStack_228) [16];
  code *pcStack_220;
  longlong lStack_218;
  longlong lStack_210;
  longlong lStack_208;
  longlong lStack_200;
  undefined1 (*pauStack_1f8) [16];
  code *pcStack_1f0;
  longlong lStack_1e8;
  longlong lStack_1e0;
  longlong lStack_1d8;
  longlong lStack_1d0;
  undefined1 (*pauStack_1c8) [16];
  code *pcStack_1c0;
  longlong lStack_1b8;
  longlong lStack_1b0;
  longlong lStack_1a8;
  longlong lStack_1a0;
  undefined4 uStack_18e;
  undefined2 uStack_18a;
  undefined8 *puStack_188;
  undefined1 (*pauStack_180) [16];
  undefined1 auStack_178 [9];
  undefined7 uStack_16f;
  undefined1 auStack_168 [10];
  undefined4 uStack_15e;
  undefined2 uStack_15a;
  undefined1 auStack_158 [16];
  longlong lStack_148;
  undefined1 auStack_140 [16];
  undefined8 uStack_130;
  undefined *puStack_128;
  undefined8 uStack_120;
  undefined1 (**ppauStack_118) [16];
  undefined8 uStack_110;
  undefined8 uStack_108;
  undefined1 (*pauStack_f0) [16];
  uint uStack_e4;
  undefined8 uStack_e0;
  undefined8 *puStack_d8;
  undefined8 *puStack_d0;
  undefined8 uStack_c8;
  undefined1 (*pauStack_c0) [16];
  undefined1 uStack_b1;
  undefined8 uStack_b0;
  uStack_b0 = 0xfffffffffffffffe;
  plVar33 = (longlong *)FUN_1400d9b20(0);
  if (plVar33 == (longlong *)0x0) {
    FUN_1428d80d0(&UNK_142c3aaa0);
  }
  else {
    lStack_1d8 = *plVar33;
    lStack_1d0 = plVar33[1];
    *plVar33 = *plVar33 + 1;
    lVar53 = _UNK_142c6bf18;
    lVar3 = _UNK_142c6bf10;
    pcVar6 = _UNK_142c6bf08;
    pauVar38 = _UNK_142c6bf00;
    pauStack_1f8 = _UNK_142c6bf00;
    pcStack_1f0 = _UNK_142c6bf08;
    lStack_1e8 = _UNK_142c6bf10;
    lStack_1e0 = _UNK_142c6bf18;
    plVar33 = (longlong *)FUN_1400d9b20(0);
    puStack_188 = param_2;
    if (plVar33 != (longlong *)0x0) {
      lStack_1a8 = *plVar33;
      lStack_1a0 = plVar33[1];
      *plVar33 = *plVar33 + 1;
      pauStack_1c8 = pauVar38;
      pcStack_1c0 = pcVar6;
      lStack_1b8 = lVar3;
      lStack_1b0 = lVar53;
      lVar3 = *param_4;
      uVar36 = *(ulonglong *)(lVar3 + 0x38);
      pauVar38 = (undefined1 (*) [16])(uVar36 + 0x10);
      LOCK();
      bVar55 = *(char *)(uVar36 + 0x10) == '\0';
      if (bVar55) {
        *(char *)(uVar36 + 0x10) = '\x01';
      }
      UNLOCK();
      if (!bVar55) {
        uStack_b1 = 1;
        FUN_1428d8750(pauVar38);
      }
      puVar48 = puRam000000014362c420;
      if ((*puRam000000014362c420 & 0x7fffffffffffffff) == 0) {
        uStack_c8 = (ulonglong *)((ulonglong)uStack_c8 & 0xffffffff00000000);
        cVar29 = *(char *)(uVar36 + 0x11);
        uVar5 = _UNK_142af4bb8;
      }
      else {
        uStack_b1 = 1;
        uVar32 = FUN_1428d86b0();
        uStack_c8 = (ulonglong *)(CONCAT44(uStack_c8._4_4_,uVar32) ^ 1);
        cVar29 = *(char *)(uVar36 + 0x11);
        uVar5 = _UNK_142af4bb8;
      }
      _UNK_142af4bb8 = uVar5;
      pauStack_c0 = pauVar38;
      if (cVar29 == '\0') {
        uStack_130 = uVar36 + 0x18;
        auStack_178._0_8_ = *(undefined8 *)(uVar36 + 0x18);
        lVar53 = *(longlong *)(uVar36 + 0x20);
        auVar2 = *(undefined1 (*) [16])auStack_178._0_8_;
        auStack_158._0_8_ = *(undefined8 *)(uVar36 + 0x30);
        auStack_158._8_8_ = *(undefined8 *)(uVar36 + 0x18);
        lStack_148 = *(longlong *)(uVar36 + 0x20);
        auStack_140 = *(undefined1 (*) [16])(uVar36 + 0x28);
        *(undefined8 *)(uVar36 + 0x18) = _UNK_142af4bb0;
        *(undefined8 *)(uVar36 + 0x20) = uVar5;
        uVar5 = _UNK_142af4bc8;
        *(undefined8 *)(uVar36 + 0x28) = _UNK_142af4bc0;
        *(undefined8 *)(uVar36 + 0x30) = uVar5;
        stack0xfffffffffffffe90 = (longlong)(auStack_178._0_8_ + 0x10);
        auStack_168._8_2_ =
             ~((ushort)(SUB161(auVar2 >> 7,0) & 1) | (ushort)(SUB161(auVar2 >> 0xf,0) & 1) << 1 |
               (ushort)(SUB161(auVar2 >> 0x17,0) & 1) << 2 |
               (ushort)(SUB161(auVar2 >> 0x1f,0) & 1) << 3 |
               (ushort)(SUB161(auVar2 >> 0x27,0) & 1) << 4 |
               (ushort)(SUB161(auVar2 >> 0x2f,0) & 1) << 5 |
               (ushort)(SUB161(auVar2 >> 0x37,0) & 1) << 6 |
               (ushort)(SUB161(auVar2 >> 0x3f,0) & 1) << 7 |
               (ushort)(SUB161(auVar2 >> 0x47,0) & 1) << 8 |
               (ushort)(SUB161(auVar2 >> 0x4f,0) & 1) << 9 |
               (ushort)(SUB161(auVar2 >> 0x57,0) & 1) << 10 |
               (ushort)(SUB161(auVar2 >> 0x5f,0) & 1) << 0xb |
               (ushort)(SUB161(auVar2 >> 0x67,0) & 1) << 0xc |
               (ushort)(SUB161(auVar2 >> 0x6f,0) & 1) << 0xd |
               (ushort)(SUB161(auVar2 >> 0x77,0) & 1) << 0xe |
              (ushort)(byte)(auVar2[0xf] >> 7) << 0xf);
        auStack_168._0_8_ = *(undefined1 (*) [16])auStack_178._0_8_ + lVar53 + 1;
        uStack_15e = uStack_18e;
        uStack_15a = uStack_18a;
        FUN_14130b3d0(&pauStack_228,auStack_178,&UNK_142c29fc0);
        pauVar38 = pauStack_c0;
        if (((char)uStack_c8 == '\0') && ((*puVar48 & 0x7fffffffffffffff) != 0)) {
          uStack_b1 = 1;
          cVar29 = FUN_1428d86b0();
          if (cVar29 == '\0') {
            *(undefined1 *)(uVar36 + 0x11) = 1;
          }
        }
        LOCK();
        cVar29 = (*pauVar38)[0];
        (*pauVar38)[0] = 0;
        UNLOCK();
        if (cVar29 == '\x02') {
          uStack_b1 = 1;
          FUN_1428d8810(pauVar38);
        }
        stack0xfffffffffffffe90 = (longlong)pcStack_220;
        auStack_178._0_8_ = pcStack_220;
        stack0xfffffffffffffea0 = (ulonglong)(pcStack_220 + lStack_218);
        auStack_168._0_8_ = pauStack_228;
        uStack_b1 = 1;
        FUN_1415232f0(&pauStack_1c8,auStack_178);
        uStack_c8 = (ulonglong *)*param_5;
        lVar53 = param_5[1];
        uVar37 = *uStack_c8;
        pauVar38 = (undefined1 (*) [16])0x0;
        pauVar43 = (undefined1 (*) [16])(uVar37 - *(ulonglong *)(lVar53 + 0x18));
        if (uVar37 < *(ulonglong *)(lVar53 + 0x18)) {
          pauVar43 = pauVar38;
        }
        pauVar39 = (undefined1 (*) [16])(uVar37 - *(ulonglong *)(lVar53 + 0x38));
        if (uVar37 < *(ulonglong *)(lVar53 + 0x38)) {
          pauVar39 = pauVar38;
        }
        puVar54 = (undefined8 *)((longlong)pauVar43 * 0x38 + *(longlong *)(lVar53 + 8));
        pauVar4 = *(undefined1 (**) [16])(lVar53 + 0x10);
        pauVar49 = (undefined1 (*) [16])((longlong)pauVar4 - (longlong)pauVar43);
        if (pauVar4 < pauVar43) {
          pauVar49 = pauVar38;
        }
        if (pauVar4 < pauVar43) {
          puVar54 = (undefined8 *)0x8;
        }
        puVar46 = (undefined8 *)((longlong)pauVar39 * 0x38 + *(longlong *)(lVar53 + 0x28));
        pauVar43 = (undefined1 (*) [16])
                   ((longlong)*(undefined1 (**) [16])(lVar53 + 0x30) - (longlong)pauVar39);
        if (*(undefined1 (**) [16])(lVar53 + 0x30) < pauVar39) {
          puVar46 = (undefined8 *)0x8;
          pauVar43 = pauVar38;
        }
        *uStack_c8 = (*(longlong *)(lVar53 + 0x40) - (longlong)pauVar43) - (longlong)pauVar49;
        puStack_d8 = puVar54 + (longlong)pauVar49 * 7;
        puStack_d0 = puVar46 + (longlong)pauVar43 * 7;
        pauStack_f0 = (undefined1 (*) [16])*param_1;
        while( true ) {
          uVar45 = (uint)uVar36;
          if (pauVar38 == (undefined1 (*) [16])0x0) goto code_r0x0001412a0be0;
          while (pauVar43 == (undefined1 (*) [16])0x0) {
code_r0x0001412a0be0:
            if ((puVar54 == (undefined8 *)0x0) ||
               (puVar35 = puVar54 + 7, puVar40 = puVar46, puVar54 == puStack_d8)) {
              if ((puVar46 == (undefined8 *)0x0) || (puVar46 == puStack_d0)) {
                if (lStack_1e0 == 0 && lStack_1b0 == 0) {
                  if (pcStack_1c0 != (code *)0x0) {
                    if (pcStack_1c0 + ((ulonglong)pcStack_1c0 & 0xfffffffffffffff0) + 0x21 !=
                        (code *)0x0) {
                      func_0x000140613c20((longlong)pauStack_1c8 +
                                          (-0x10 - ((ulonglong)pcStack_1c0 & 0xfffffffffffffff0)),
                                          pcStack_1c0 +
                                          ((ulonglong)pcStack_1c0 & 0xfffffffffffffff0) + 0x21,0x10)
                      ;
                    }
                  }
                  goto code_r0x0001412a15d4;
                }
                auVar2 = *pauStack_1c8;
                uVar45 = ~(uint)(ushort)((ushort)(SUB161(auVar2 >> 7,0) & 1) |
                                         (ushort)(SUB161(auVar2 >> 0xf,0) & 1) << 1 |
                                         (ushort)(SUB161(auVar2 >> 0x17,0) & 1) << 2 |
                                         (ushort)(SUB161(auVar2 >> 0x1f,0) & 1) << 3 |
                                         (ushort)(SUB161(auVar2 >> 0x27,0) & 1) << 4 |
                                         (ushort)(SUB161(auVar2 >> 0x2f,0) & 1) << 5 |
                                         (ushort)(SUB161(auVar2 >> 0x37,0) & 1) << 6 |
                                         (ushort)(SUB161(auVar2 >> 0x3f,0) & 1) << 7 |
                                         (ushort)(SUB161(auVar2 >> 0x47,0) & 1) << 8 |
                                         (ushort)(SUB161(auVar2 >> 0x4f,0) & 1) << 9 |
                                         (ushort)(SUB161(auVar2 >> 0x57,0) & 1) << 10 |
                                         (ushort)(SUB161(auVar2 >> 0x5f,0) & 1) << 0xb |
                                         (ushort)(SUB161(auVar2 >> 0x67,0) & 1) << 0xc |
                                         (ushort)(SUB161(auVar2 >> 0x6f,0) & 1) << 0xd |
                                         (ushort)(SUB161(auVar2 >> 0x77,0) & 1) << 0xe |
                                        (ushort)(byte)(auVar2[0xf] >> 7) << 0xf);
                pauVar38 = pauStack_1c8 + 1;
                puStack_d8 = (undefined8 *)
                             CONCAT44(puStack_d8._4_4_,*(undefined4 *)((longlong)puStack_188 + 0x1c)
                                     );
                uStack_e0 = *puStack_188;
                puStack_d0 = (undefined8 *)puStack_188[2];
                lVar53 = lStack_1b0;
                pauVar43 = pauStack_1c8;
                while (bVar55 = lVar53 != 0, lVar53 = lVar53 + -1, bVar55) {
                  if ((short)uVar45 == 0) {
                    do {
                      auVar2 = *pauVar38;
                      pauVar43 = pauVar43 + -1;
                      pauVar38 = pauVar38 + 1;
                      uVar45 = (uint)(ushort)((ushort)(SUB161(auVar2 >> 7,0) & 1) |
                                              (ushort)(SUB161(auVar2 >> 0xf,0) & 1) << 1 |
                                              (ushort)(SUB161(auVar2 >> 0x17,0) & 1) << 2 |
                                              (ushort)(SUB161(auVar2 >> 0x1f,0) & 1) << 3 |
                                              (ushort)(SUB161(auVar2 >> 0x27,0) & 1) << 4 |
                                              (ushort)(SUB161(auVar2 >> 0x2f,0) & 1) << 5 |
                                              (ushort)(SUB161(auVar2 >> 0x37,0) & 1) << 6 |
                                              (ushort)(SUB161(auVar2 >> 0x3f,0) & 1) << 7 |
                                              (ushort)(SUB161(auVar2 >> 0x47,0) & 1) << 8 |
                                              (ushort)(SUB161(auVar2 >> 0x4f,0) & 1) << 9 |
                                              (ushort)(SUB161(auVar2 >> 0x57,0) & 1) << 10 |
                                              (ushort)(SUB161(auVar2 >> 0x5f,0) & 1) << 0xb |
                                              (ushort)(SUB161(auVar2 >> 0x67,0) & 1) << 0xc |
                                              (ushort)(SUB161(auVar2 >> 0x6f,0) & 1) << 0xd |
                                              (ushort)(SUB161(auVar2 >> 0x77,0) & 1) << 0xe |
                                             (ushort)(byte)(auVar2[0xf] >> 7) << 0xf);
                    } while (uVar45 == 0xffff);
                    uVar45 = ~uVar45;
                  }
                  uVar30 = 0;
                  for (uVar8 = uVar45; (uVar8 & 1) == 0; uVar8 = uVar8 >> 1 | 0x80000000) {
                    uVar30 = uVar30 + 1;
                  }
                  uVar45 = uVar45 - 1 & uVar45;
                  *(undefined4 *)puStack_d0 = puStack_d8._0_4_;
                  uStack_b1 = 1;
                  FUN_140ae6880(uStack_e0,(*pauVar43)[~(ulonglong)uVar30]);
                }
                auVar2 = *pauStack_1f8;
                uVar45 = ~(uint)(ushort)((ushort)(SUB161(auVar2 >> 7,0) & 1) |
                                         (ushort)(SUB161(auVar2 >> 0xf,0) & 1) << 1 |
                                         (ushort)(SUB161(auVar2 >> 0x17,0) & 1) << 2 |
                                         (ushort)(SUB161(auVar2 >> 0x1f,0) & 1) << 3 |
                                         (ushort)(SUB161(auVar2 >> 0x27,0) & 1) << 4 |
                                         (ushort)(SUB161(auVar2 >> 0x2f,0) & 1) << 5 |
                                         (ushort)(SUB161(auVar2 >> 0x37,0) & 1) << 6 |
                                         (ushort)(SUB161(auVar2 >> 0x3f,0) & 1) << 7 |
                                         (ushort)(SUB161(auVar2 >> 0x47,0) & 1) << 8 |
                                         (ushort)(SUB161(auVar2 >> 0x4f,0) & 1) << 9 |
                                         (ushort)(SUB161(auVar2 >> 0x57,0) & 1) << 10 |
                                         (ushort)(SUB161(auVar2 >> 0x5f,0) & 1) << 0xb |
                                         (ushort)(SUB161(auVar2 >> 0x67,0) & 1) << 0xc |
                                         (ushort)(SUB161(auVar2 >> 0x6f,0) & 1) << 0xd |
                                         (ushort)(SUB161(auVar2 >> 0x77,0) & 1) << 0xe |
                                        (ushort)(byte)(auVar2[0xf] >> 7) << 0xf);
                pauVar38 = pauStack_1f8 + 1;
                pauVar43 = pauStack_1f8;
                lVar53 = lStack_1e0;
                while (auVar28 = _UNK_1429cd090, auVar2 = _UNK_1429cd080, bVar55 = lVar53 != 0,
                      lVar53 = lVar53 + -1, bVar55) {
                  if ((short)uVar45 == 0) {
                    do {
                      auVar2 = *pauVar38;
                      pauVar43 = pauVar43 + -0x10;
                      pauVar38 = pauVar38 + 1;
                      uVar45 = (uint)(ushort)((ushort)(SUB161(auVar2 >> 7,0) & 1) |
                                              (ushort)(SUB161(auVar2 >> 0xf,0) & 1) << 1 |
                                              (ushort)(SUB161(auVar2 >> 0x17,0) & 1) << 2 |
                                              (ushort)(SUB161(auVar2 >> 0x1f,0) & 1) << 3 |
                                              (ushort)(SUB161(auVar2 >> 0x27,0) & 1) << 4 |
                                              (ushort)(SUB161(auVar2 >> 0x2f,0) & 1) << 5 |
                                              (ushort)(SUB161(auVar2 >> 0x37,0) & 1) << 6 |
                                              (ushort)(SUB161(auVar2 >> 0x3f,0) & 1) << 7 |
                                              (ushort)(SUB161(auVar2 >> 0x47,0) & 1) << 8 |
                                              (ushort)(SUB161(auVar2 >> 0x4f,0) & 1) << 9 |
                                              (ushort)(SUB161(auVar2 >> 0x57,0) & 1) << 10 |
                                              (ushort)(SUB161(auVar2 >> 0x5f,0) & 1) << 0xb |
                                              (ushort)(SUB161(auVar2 >> 0x67,0) & 1) << 0xc |
                                              (ushort)(SUB161(auVar2 >> 0x6f,0) & 1) << 0xd |
                                              (ushort)(SUB161(auVar2 >> 0x77,0) & 1) << 0xe |
                                             (ushort)(byte)(auVar2[0xf] >> 7) << 0xf);
                    } while (uVar45 == 0xffff);
                    uVar45 = ~uVar45;
                  }
                  iVar7 = 0;
                  for (uVar30 = uVar45; (uVar30 & 1) == 0; uVar30 = uVar30 >> 1 | 0x80000000) {
                    iVar7 = iVar7 + 1;
                  }
                  uVar45 = uVar45 - 1 & uVar45;
                  *(undefined4 *)puStack_d0 = puStack_d8._0_4_;
                  uStack_b1 = 1;
                  FUN_140a37e50(uStack_e0,
                                (longlong)pauVar43 + (-0x10 - (ulonglong)(uint)(iVar7 << 4)));
                }
                pauVar38 = *(undefined1 (**) [16])*pauStack_f0;
                lVar53 = *(longlong *)(pauStack_f0[1] + 8);
                auVar59 = *pauVar38;
                pauStack_180 = pauVar38 + 1;
                iVar7 = -(uint)(CONCAT13(unaff_XMM9_Bd,
                                         CONCAT12(unaff_XMM9_Bc,
                                                  CONCAT11(unaff_XMM9_Bb,unaff_XMM9_Ba))) ==
                               CONCAT13(unaff_XMM9_Bd,
                                        CONCAT12(unaff_XMM9_Bc,CONCAT11(unaff_XMM9_Bb,unaff_XMM9_Ba)
                                                )));
                cVar29 = (char)((uint)iVar7 >> 8);
                cVar68 = (char)((uint)iVar7 >> 0x10);
                cVar69 = (char)((uint)iVar7 >> 0x18);
                iVar9 = -(uint)(CONCAT13(unaff_XMM9_Bh,
                                         CONCAT12(unaff_XMM9_Bg,
                                                  CONCAT11(unaff_XMM9_Bf,unaff_XMM9_Be))) ==
                               CONCAT13(unaff_XMM9_Bh,
                                        CONCAT12(unaff_XMM9_Bg,CONCAT11(unaff_XMM9_Bf,unaff_XMM9_Be)
                                                )));
                cVar70 = (char)((uint)iVar9 >> 8);
                cVar71 = (char)((uint)iVar9 >> 0x10);
                cVar72 = (char)((uint)iVar9 >> 0x18);
                iVar10 = -(uint)(CONCAT13(unaff_XMM9_Bl,
                                          CONCAT12(unaff_XMM9_Bk,
                                                   CONCAT11(unaff_XMM9_Bj,unaff_XMM9_Bi))) ==
                                CONCAT13(unaff_XMM9_Bl,
                                         CONCAT12(unaff_XMM9_Bk,
                                                  CONCAT11(unaff_XMM9_Bj,unaff_XMM9_Bi))));
                cVar73 = (char)((uint)iVar10 >> 8);
                cVar74 = (char)((uint)iVar10 >> 0x10);
                cVar75 = (char)((uint)iVar10 >> 0x18);
                iVar11 = -(uint)(CONCAT13(unaff_XMM9_Bp,
                                          CONCAT12(unaff_XMM9_Bo,
                                                   CONCAT11(unaff_XMM9_Bn,unaff_XMM9_Bm))) ==
                                CONCAT13(unaff_XMM9_Bp,
                                         CONCAT12(unaff_XMM9_Bo,
                                                  CONCAT11(unaff_XMM9_Bn,unaff_XMM9_Bm))));
                cVar76 = (char)((uint)iVar11 >> 8);
                cVar77 = (char)((uint)iVar11 >> 0x10);
                cVar78 = (char)((uint)iVar11 >> 0x18);
                uVar45 = ~(uint)(ushort)((ushort)(SUB161(auVar59 >> 7,0) & 1) |
                                         (ushort)(SUB161(auVar59 >> 0xf,0) & 1) << 1 |
                                         (ushort)(SUB161(auVar59 >> 0x17,0) & 1) << 2 |
                                         (ushort)(SUB161(auVar59 >> 0x1f,0) & 1) << 3 |
                                         (ushort)(SUB161(auVar59 >> 0x27,0) & 1) << 4 |
                                         (ushort)(SUB161(auVar59 >> 0x2f,0) & 1) << 5 |
                                         (ushort)(SUB161(auVar59 >> 0x37,0) & 1) << 6 |
                                         (ushort)(SUB161(auVar59 >> 0x3f,0) & 1) << 7 |
                                         (ushort)(SUB161(auVar59 >> 0x47,0) & 1) << 8 |
                                         (ushort)(SUB161(auVar59 >> 0x4f,0) & 1) << 9 |
                                         (ushort)(SUB161(auVar59 >> 0x57,0) & 1) << 10 |
                                         (ushort)(SUB161(auVar59 >> 0x5f,0) & 1) << 0xb |
                                         (ushort)(SUB161(auVar59 >> 0x67,0) & 1) << 0xc |
                                         (ushort)(SUB161(auVar59 >> 0x6f,0) & 1) << 0xd |
                                         (ushort)(SUB161(auVar59 >> 0x77,0) & 1) << 0xe |
                                        (ushort)(byte)(auVar59[0xf] >> 7) << 0xf);
                goto code_r0x0001412a0e22;
              }
              puVar35 = (undefined8 *)0x0;
              puVar40 = puVar46 + 7;
              puVar54 = puVar46;
            }
            *uStack_c8 = *uStack_c8 + 1;
            pauVar38 = (undefined1 (*) [16])*puVar54;
            auVar2 = *pauVar38;
            uVar45 = ~(uint)(ushort)((ushort)(SUB161(auVar2 >> 7,0) & 1) |
                                     (ushort)(SUB161(auVar2 >> 0xf,0) & 1) << 1 |
                                     (ushort)(SUB161(auVar2 >> 0x17,0) & 1) << 2 |
                                     (ushort)(SUB161(auVar2 >> 0x1f,0) & 1) << 3 |
                                     (ushort)(SUB161(auVar2 >> 0x27,0) & 1) << 4 |
                                     (ushort)(SUB161(auVar2 >> 0x2f,0) & 1) << 5 |
                                     (ushort)(SUB161(auVar2 >> 0x37,0) & 1) << 6 |
                                     (ushort)(SUB161(auVar2 >> 0x3f,0) & 1) << 7 |
                                     (ushort)(SUB161(auVar2 >> 0x47,0) & 1) << 8 |
                                     (ushort)(SUB161(auVar2 >> 0x4f,0) & 1) << 9 |
                                     (ushort)(SUB161(auVar2 >> 0x57,0) & 1) << 10 |
                                     (ushort)(SUB161(auVar2 >> 0x5f,0) & 1) << 0xb |
                                     (ushort)(SUB161(auVar2 >> 0x67,0) & 1) << 0xc |
                                     (ushort)(SUB161(auVar2 >> 0x6f,0) & 1) << 0xd |
                                     (ushort)(SUB161(auVar2 >> 0x77,0) & 1) << 0xe |
                                    (ushort)(byte)(auVar2[0xf] >> 7) << 0xf);
            unaff_R12 = pauVar38 + 1;
            pauVar43 = (undefined1 (*) [16])puVar54[3];
            puVar46 = puVar40;
            puVar54 = puVar35;
          }
          if ((short)uVar45 == 0) {
            do {
              auVar2 = *unaff_R12;
              pauVar38 = pauVar38 + -0x10;
              unaff_R12 = unaff_R12 + 1;
              uVar45 = (uint)(ushort)((ushort)(SUB161(auVar2 >> 7,0) & 1) |
                                      (ushort)(SUB161(auVar2 >> 0xf,0) & 1) << 1 |
                                      (ushort)(SUB161(auVar2 >> 0x17,0) & 1) << 2 |
                                      (ushort)(SUB161(auVar2 >> 0x1f,0) & 1) << 3 |
                                      (ushort)(SUB161(auVar2 >> 0x27,0) & 1) << 4 |
                                      (ushort)(SUB161(auVar2 >> 0x2f,0) & 1) << 5 |
                                      (ushort)(SUB161(auVar2 >> 0x37,0) & 1) << 6 |
                                      (ushort)(SUB161(auVar2 >> 0x3f,0) & 1) << 7 |
                                      (ushort)(SUB161(auVar2 >> 0x47,0) & 1) << 8 |
                                      (ushort)(SUB161(auVar2 >> 0x4f,0) & 1) << 9 |
                                      (ushort)(SUB161(auVar2 >> 0x57,0) & 1) << 10 |
                                      (ushort)(SUB161(auVar2 >> 0x5f,0) & 1) << 0xb |
                                      (ushort)(SUB161(auVar2 >> 0x67,0) & 1) << 0xc |
                                      (ushort)(SUB161(auVar2 >> 0x6f,0) & 1) << 0xd |
                                      (ushort)(SUB161(auVar2 >> 0x77,0) & 1) << 0xe |
                                     (ushort)(byte)(auVar2[0xf] >> 7) << 0xf);
            } while (uVar45 == 0xffff);
            uVar45 = ~uVar45;
          }
          iVar7 = 0;
          for (uVar30 = uVar45; (uVar30 & 1) == 0; uVar30 = uVar30 >> 1 | 0x80000000) {
            iVar7 = iVar7 + 1;
          }
          uVar30 = iVar7 << 4;
          plVar33 = *(longlong **)((longlong)pauVar38 + (-0x10 - (ulonglong)uVar30));
          uVar5 = *(undefined8 *)((longlong)pauVar38 + (-8 - (ulonglong)uVar30));
          LOCK();
          lVar53 = *plVar33;
          *plVar33 = *plVar33 + 1;
          UNLOCK();
          pauStack_c0 = pauVar43;
          if (*plVar33 == 0 || SCARRY8(lVar53,1) != *plVar33 < 0) break;
          uStack_b1 = 1;
          FUN_14152b320(&pauStack_1f8,plVar33,uVar5);
          uStack_b1 = 1;
          lVar34 = FUN_141552d70(pauStack_f0,(longlong)pauVar38 + (-0x10 - (ulonglong)uVar30));
          uVar36 = (ulonglong)(uVar45 - 1 & uVar45);
          pauStack_c0 = (undefined1 (*) [16])(pauStack_c0[-1] + 0xf);
          lVar53 = *(longlong *)(lVar34 + 8);
          lVar34 = *(longlong *)(lVar34 + 0x10) * 0x98 + lVar53;
          for (; pauVar43 = pauStack_c0, lVar53 != lVar34; lVar53 = lVar53 + 0x98) {
            uStack_b1 = 1;
            FUN_1415265f0(&pauStack_1c8,*(undefined1 *)(lVar53 + 0x90));
          }
        }
      }
      else {
        auStack_178[8] = (char)uStack_c8;
        auStack_178._0_8_ = pauVar38;
        FUN_1428d9760(&UNK_142c2a018,0x2b,auStack_178,&UNK_142c2a048,&UNK_142c2aae0);
      }
      goto code_r0x0001412a16f9;
    }
  }
  FUN_1428d80d0(&UNK_142c3aaa0);
code_r0x0001412a16f9:
                    /* WARNING: Does not return */
  pcVar6 = (code *)invalidInstructionException();
  (*pcVar6)();
code_r0x0001412a0e22:
  bVar55 = lVar53 == 0;
  lVar53 = lVar53 + -1;
  if (bVar55) {
    if (lStack_1b0 != 0) {
      if (2 < *puRam000000014362c3a8) {
        pauStack_228 = (undefined1 (*) [16])&pauStack_1c8;
        pcStack_220 = FUN_1408b2760;
        uStack_b1 = 1;
        pauVar38 = (undefined1 (*) [16])func_0x00014067dd80(&UNK_142c2a508);
        auStack_158 = *pauVar38;
        lStack_148 = 3;
        auStack_140._8_8_ = 0x28;
        auStack_140._0_8_ = &UNK_142c2a520;
        puStack_128 = &UNK_142c2a4f8;
        uStack_120 = 1;
        ppauStack_118 = &pauStack_228;
        uStack_110 = 1;
        uStack_108 = 0;
        _auStack_178 = ZEXT816(0x142c2a520) << 0x40;
        _auStack_168 = ZEXT816(0x28);
        uStack_130 = CONCAT44(*(undefined4 *)pauVar38[1],1);
        uStack_b1 = 1;
        func_0x000140d4a330(&uStack_18e,auStack_178);
      }
      *(undefined4 *)puStack_d0 = puStack_d8._0_4_;
      *(undefined4 *)param_4[2] = *(undefined4 *)((longlong)param_4 + 0x1c);
      uStack_b1 = 1;
      FUN_1415f5b60(uStack_e0,lVar3,&pauStack_1c8);
    }
    *(undefined4 *)puStack_d0 = puStack_d8._0_4_;
    *(undefined4 *)param_3[2] = *(undefined4 *)((longlong)param_3 + 0x1c);
    uStack_b1 = 1;
    FUN_140ae6b90(uStack_e0,*param_3);
    lStack_208 = lStack_1a8;
    lStack_200 = lStack_1a0;
    lStack_218 = lStack_1b8;
    lStack_210 = lStack_1b0;
    pauStack_228 = pauStack_1c8;
    pcStack_220 = pcStack_1c0;
    *(undefined4 *)param_6[2] = *(undefined4 *)((longlong)param_6 + 0x1c);
    lVar3 = *param_6;
    lStack_148 = *(longlong *)(lVar3 + 0x40);
    auStack_158._8_8_ = lStack_1a0;
    auStack_158._0_8_ = lStack_1a8;
    stack0xfffffffffffffea0 = lStack_1b0;
    auStack_168._0_8_ = lStack_1b8;
    stack0xfffffffffffffe90 = (longlong)pcStack_1c0;
    auStack_178._0_8_ = pauStack_1c8;
    lVar53 = *(longlong *)(lVar3 + 0x30);
    if (lVar53 == *(longlong *)(lVar3 + 0x20)) {
      FUN_1406319d0(lVar3 + 0x20,&UNK_142c3a148);
    }
    lVar34 = *(longlong *)(lVar3 + 0x28);
    lVar42 = lVar53 * 0x38;
    *(longlong *)(lVar34 + 0x30 + lVar42) = lStack_148;
    puVar54 = (undefined8 *)(lVar34 + 0x20 + lVar42);
    *puVar54 = auStack_158._0_8_;
    puVar54[1] = auStack_158._8_8_;
    puVar54 = (undefined8 *)(lVar34 + 0x10 + lVar42);
    *puVar54 = auStack_168._0_8_;
    puVar54[1] = stack0xfffffffffffffea0;
    *(undefined8 *)(lVar34 + lVar42) = auStack_178._0_8_;
    ((undefined8 *)(lVar34 + lVar42))[1] = stack0xfffffffffffffe90;
    *(longlong *)(lVar3 + 0x30) = lVar53 + 1;
    *(longlong *)(lVar3 + 0x40) = *(longlong *)(lVar3 + 0x40) + 1;
code_r0x0001412a15d4:
    FUN_140b73f00(&pauStack_1f8,&lStack_1d8,0x10,0x10);
    return;
  }
  if ((short)uVar45 == 0) {
    do {
      auVar59 = *pauStack_180;
      uVar52 = (ushort)(SUB161(auVar59 >> 7,0) & 1) | (ushort)(SUB161(auVar59 >> 0xf,0) & 1) << 1 |
               (ushort)(SUB161(auVar59 >> 0x17,0) & 1) << 2 |
               (ushort)(SUB161(auVar59 >> 0x1f,0) & 1) << 3 |
               (ushort)(SUB161(auVar59 >> 0x27,0) & 1) << 4 |
               (ushort)(SUB161(auVar59 >> 0x2f,0) & 1) << 5 |
               (ushort)(SUB161(auVar59 >> 0x37,0) & 1) << 6 |
               (ushort)(SUB161(auVar59 >> 0x3f,0) & 1) << 7 |
               (ushort)(SUB161(auVar59 >> 0x47,0) & 1) << 8 |
               (ushort)(SUB161(auVar59 >> 0x4f,0) & 1) << 9 |
               (ushort)(SUB161(auVar59 >> 0x57,0) & 1) << 10 |
               (ushort)(SUB161(auVar59 >> 0x5f,0) & 1) << 0xb |
               (ushort)(SUB161(auVar59 >> 0x67,0) & 1) << 0xc |
               (ushort)(SUB161(auVar59 >> 0x6f,0) & 1) << 0xd |
               (ushort)(SUB161(auVar59 >> 0x77,0) & 1) << 0xe |
               (ushort)(byte)(auVar59[0xf] >> 7) << 0xf;
      pauVar38 = pauVar38 + -0x28;
      pauStack_180 = pauStack_180 + 1;
    } while (uVar52 == 0xffff);
    uVar45 = ~(uint)uVar52;
    uStack_e4 = uVar45 & -(uint)uVar52 - 2;
  }
  else {
    uStack_e4 = uVar45 - 1 & uVar45;
  }
  uVar30 = 0;
  for (; (uVar45 & 1) == 0; uVar45 = uVar45 >> 1 | 0x80000000) {
    uVar30 = uVar30 + 1;
  }
  uVar36 = (ulonglong)uVar30;
  pauStack_c0 = (undefined1 (*) [16])((longlong)pauVar38 + uVar36 * -0x28);
  puVar48 = *(ulonglong **)((longlong)pauVar38 + uVar36 * -0x28 + -0x10);
  uStack_c8 = puVar48 + *(longlong *)((longlong)pauVar38 + uVar36 * -0x28 + -8) * 0x13;
code_r0x0001412a0f80:
  while (puVar47 = puVar48, uVar45 = uStack_e4, puVar47 != uStack_c8) {
    puVar48 = puVar47 + 0x13;
    if (lStack_1b0 != 0) {
      auStack_158._8_8_ = lStack_1a0;
      auStack_158._0_8_ = lStack_1a8;
      auVar59._8_4_ = (int)lStack_1a8;
      auVar59._0_8_ = lStack_1a8;
      auVar59._12_4_ = (int)((ulonglong)lStack_1a8 >> 0x20);
      _auStack_178 = auVar59 ^ auVar2;
      auVar63._8_4_ = (int)lStack_1a0;
      auVar63._0_8_ = lStack_1a0;
      auVar63._12_4_ = (int)((ulonglong)lStack_1a0 >> 0x20);
      _auStack_168 = auVar63 ^ auVar28;
      lStack_148 = 0;
      auStack_140 = ZEXT816(0);
      pauStack_228 = (undefined1 (*) [16])CONCAT71(pauStack_228._1_7_,(char)puVar47[0x12]);
      FUN_140106160(auStack_178,&pauStack_228,1);
      uVar51 = lStack_148 << 0x38 | auStack_140._0_8_;
      uVar36 = stack0xfffffffffffffea0 ^ uVar51;
      uVar50 = auStack_178._0_8_ + auStack_168._0_8_;
      uVar41 = (auStack_168._0_8_ << 0xd | (ulonglong)auStack_168._0_8_ >> 0x33) ^ uVar50;
      uVar37 = (uVar36 << 0x10 | uVar36 >> 0x30) ^ stack0xfffffffffffffe90 + uVar36;
      uVar44 = stack0xfffffffffffffe90 + uVar36 + uVar41;
      uVar50 = (uVar50 << 0x20 | uVar50 >> 0x20) + uVar37;
      uVar41 = (uVar41 << 0x11 | uVar41 >> 0x2f) ^ uVar44;
      uVar36 = (uVar37 << 0x15 | uVar37 >> 0x2b) ^ uVar50;
      uVar50 = (uVar50 ^ uVar51) + uVar41;
      uVar44 = ((uVar44 << 0x20 | uVar44 >> 0x20) ^ 0xff) + uVar36;
      uVar37 = (uVar41 << 0xd | uVar41 >> 0x33) ^ uVar50;
      uVar36 = (uVar36 << 0x10 | uVar36 >> 0x30) ^ uVar44;
      uVar44 = uVar44 + uVar37;
      uVar50 = (uVar50 << 0x20 | uVar50 >> 0x20) + uVar36;
      uVar37 = (uVar37 << 0x11 | uVar37 >> 0x2f) ^ uVar44;
      uVar36 = (uVar36 << 0x15 | uVar36 >> 0x2b) ^ uVar50;
      uVar50 = uVar50 + uVar37;
      uVar41 = (uVar44 << 0x20 | uVar44 >> 0x20) + uVar36;
      uVar37 = (uVar37 << 0xd | uVar37 >> 0x33) ^ uVar50;
      uVar36 = (uVar36 << 0x10 | uVar36 >> 0x30) ^ uVar41;
      uVar41 = uVar41 + uVar37;
      uVar44 = (uVar50 << 0x20 | uVar50 >> 0x20) + uVar36;
      uVar37 = (uVar37 << 0x11 | uVar37 >> 0x2f) ^ uVar41;
      uVar36 = (uVar36 << 0x15 | uVar36 >> 0x2b) ^ uVar44;
      uVar41 = (uVar41 << 0x20 | uVar41 >> 0x20) + uVar36;
      uVar37 = (uVar37 << 0xd | uVar37 >> 0x33) ^ uVar44 + uVar37;
      uVar36 = (uVar36 << 0x10 | uVar36 >> 0x30) ^ uVar41;
      uVar41 = uVar41 + uVar37;
      uVar41 = (uVar41 << 0x20 | uVar41 >> 0x20) ^ (uVar37 << 0x11 | uVar37 >> 0x2f) ^
               (uVar36 << 0x15 | uVar36 >> 0x2b) ^ uVar41;
      bVar56 = (byte)(uVar41 >> 0x39);
      auVar59 = ZEXT216(CONCAT11(bVar56,bVar56));
      auVar59 = pshuflw(auVar59,auVar59,0);
      lVar34 = 0;
      while( true ) {
        uVar41 = uVar41 & (ulonglong)pcStack_1c0;
        auVar63 = *(undefined1 (*) [16])(*pauStack_1c8 + uVar41);
        cVar57 = auVar59[0];
        auVar67[0] = -(auVar63[0] == cVar57);
        cVar60 = auVar59[1];
        auVar67[1] = -(auVar63[1] == cVar60);
        cVar61 = auVar59[2];
        auVar67[2] = -(auVar63[2] == cVar61);
        cVar62 = auVar59[3];
        auVar67[3] = -(auVar63[3] == cVar62);
        auVar67[4] = -(auVar63[4] == cVar57);
        auVar67[5] = -(auVar63[5] == cVar60);
        auVar67[6] = -(auVar63[6] == cVar61);
        auVar67[7] = -(auVar63[7] == cVar62);
        auVar67[8] = -(auVar63[8] == cVar57);
        auVar67[9] = -(auVar63[9] == cVar60);
        auVar67[10] = -(auVar63[10] == cVar61);
        auVar67[0xb] = -(auVar63[0xb] == cVar62);
        auVar67[0xc] = -(auVar63[0xc] == cVar57);
        auVar67[0xd] = -(auVar63[0xd] == cVar60);
        auVar67[0xe] = -(auVar63[0xe] == cVar61);
        auVar67[0xf] = -(auVar63[0xf] == cVar62);
        uVar52 = (ushort)(SUB161(auVar67 >> 7,0) & 1) | (ushort)(SUB161(auVar67 >> 0xf,0) & 1) << 1
                 | (ushort)(SUB161(auVar67 >> 0x17,0) & 1) << 2 |
                 (ushort)(SUB161(auVar67 >> 0x1f,0) & 1) << 3 |
                 (ushort)(SUB161(auVar67 >> 0x27,0) & 1) << 4 |
                 (ushort)(SUB161(auVar67 >> 0x2f,0) & 1) << 5 |
                 (ushort)(SUB161(auVar67 >> 0x37,0) & 1) << 6 |
                 (ushort)(SUB161(auVar67 >> 0x3f,0) & 1) << 7 |
                 (ushort)(SUB161(auVar67 >> 0x47,0) & 1) << 8 |
                 (ushort)(SUB161(auVar67 >> 0x4f,0) & 1) << 9 |
                 (ushort)(SUB161(auVar67 >> 0x57,0) & 1) << 10 |
                 (ushort)(SUB161(auVar67 >> 0x5f,0) & 1) << 0xb |
                 (ushort)(SUB161(auVar67 >> 0x67,0) & 1) << 0xc |
                 (ushort)(SUB161(auVar67 >> 0x6f,0) & 1) << 0xd |
                 (ushort)(SUB161(auVar67 >> 0x77,0) & 1) << 0xe | (ushort)(auVar67[0xf] >> 7) << 0xf
        ;
        uVar45 = (uint)uVar52;
        if (uVar52 != 0) {
          do {
            uVar30 = 0;
            for (uVar8 = uVar45; (uVar8 & 1) == 0; uVar8 = uVar8 >> 1 | 0x80000000) {
              uVar30 = uVar30 + 1;
            }
            if ((char)puVar47[0x12] == (*pauStack_1c8)[~(uVar30 + uVar41 & (ulonglong)pcStack_1c0)])
            goto code_r0x0001412a1190;
            uVar52 = (ushort)(uVar45 - 1) & (ushort)uVar45;
            uVar45 = CONCAT22((short)(uVar45 - 1 >> 0x10),uVar52);
          } while (uVar52 != 0);
        }
        auVar64[0] = -(auVar63[0] == (char)iVar7);
        auVar64[1] = -(auVar63[1] == cVar29);
        auVar64[2] = -(auVar63[2] == cVar68);
        auVar64[3] = -(auVar63[3] == cVar69);
        auVar64[4] = -(auVar63[4] == (char)iVar9);
        auVar64[5] = -(auVar63[5] == cVar70);
        auVar64[6] = -(auVar63[6] == cVar71);
        auVar64[7] = -(auVar63[7] == cVar72);
        auVar64[8] = -(auVar63[8] == (char)iVar10);
        auVar64[9] = -(auVar63[9] == cVar73);
        auVar64[10] = -(auVar63[10] == cVar74);
        auVar64[0xb] = -(auVar63[0xb] == cVar75);
        auVar64[0xc] = -(auVar63[0xc] == (char)iVar11);
        auVar64[0xd] = -(auVar63[0xd] == cVar76);
        auVar64[0xe] = -(auVar63[0xe] == cVar77);
        auVar64[0xf] = -(auVar63[0xf] == cVar78);
        if ((((((((((((((((SUB161(auVar64 >> 7,0) & 1) != 0 || (SUB161(auVar64 >> 0xf,0) & 1) != 0)
                        || (SUB161(auVar64 >> 0x17,0) & 1) != 0) ||
                       (SUB161(auVar64 >> 0x1f,0) & 1) != 0) || (SUB161(auVar64 >> 0x27,0) & 1) != 0
                      ) || (SUB161(auVar64 >> 0x2f,0) & 1) != 0) ||
                    (SUB161(auVar64 >> 0x37,0) & 1) != 0) || (SUB161(auVar64 >> 0x3f,0) & 1) != 0)
                  || (SUB161(auVar64 >> 0x47,0) & 1) != 0) || (SUB161(auVar64 >> 0x4f,0) & 1) != 0)
                || (SUB161(auVar64 >> 0x57,0) & 1) != 0) || (SUB161(auVar64 >> 0x5f,0) & 1) != 0) ||
              (SUB161(auVar64 >> 0x67,0) & 1) != 0) || (SUB161(auVar64 >> 0x6f,0) & 1) != 0) ||
            (SUB161(auVar64 >> 0x77,0) & 1) != 0) || auVar64[0xf] < '\0') break;
        uVar41 = uVar41 + lVar34 + 0x10;
        lVar34 = lVar34 + 0x10;
      }
    }
    if (lStack_1e0 != 0) {
      auStack_158._8_8_ = lStack_1d0;
      auStack_158._0_8_ = lStack_1d8;
      auVar65._8_4_ = (int)lStack_1d8;
      auVar65._0_8_ = lStack_1d8;
      auVar65._12_4_ = (int)((ulonglong)lStack_1d8 >> 0x20);
      _auStack_178 = auVar65 ^ auVar2;
      auVar66._8_4_ = (int)lStack_1d0;
      auVar66._0_8_ = lStack_1d0;
      auVar66._12_4_ = (int)((ulonglong)lStack_1d0 >> 0x20);
      _auStack_168 = auVar66 ^ auVar28;
      lStack_148 = 0;
      auStack_140 = ZEXT816(0);
      FUN_140106160(auStack_178,*(longlong *)(pauStack_c0[-3] + 8) + 0x10,
                    *(undefined8 *)pauStack_c0[-2]);
      pauStack_228 = (undefined1 (*) [16])CONCAT71(pauStack_228._1_7_,0xff);
      FUN_140106160(auStack_178,&pauStack_228,1);
      pcVar6 = pcStack_1f0;
      pauVar39 = pauStack_1f8;
      uVar51 = lStack_148 << 0x38 | auStack_140._0_8_;
      uVar36 = stack0xfffffffffffffea0 ^ uVar51;
      uVar50 = auStack_178._0_8_ + auStack_168._0_8_;
      uVar41 = (auStack_168._0_8_ << 0xd | (ulonglong)auStack_168._0_8_ >> 0x33) ^ uVar50;
      uVar37 = (uVar36 << 0x10 | uVar36 >> 0x30) ^ stack0xfffffffffffffe90 + uVar36;
      uVar44 = stack0xfffffffffffffe90 + uVar36 + uVar41;
      uVar50 = (uVar50 << 0x20 | uVar50 >> 0x20) + uVar37;
      uVar41 = (uVar41 << 0x11 | uVar41 >> 0x2f) ^ uVar44;
      uVar36 = (uVar37 << 0x15 | uVar37 >> 0x2b) ^ uVar50;
      uVar50 = (uVar50 ^ uVar51) + uVar41;
      uVar44 = ((uVar44 << 0x20 | uVar44 >> 0x20) ^ 0xff) + uVar36;
      uVar37 = (uVar41 << 0xd | uVar41 >> 0x33) ^ uVar50;
      uVar36 = (uVar36 << 0x10 | uVar36 >> 0x30) ^ uVar44;
      uVar44 = uVar44 + uVar37;
      uVar50 = (uVar50 << 0x20 | uVar50 >> 0x20) + uVar36;
      uVar37 = (uVar37 << 0x11 | uVar37 >> 0x2f) ^ uVar44;
      uVar36 = (uVar36 << 0x15 | uVar36 >> 0x2b) ^ uVar50;
      uVar50 = uVar50 + uVar37;
      uVar41 = (uVar44 << 0x20 | uVar44 >> 0x20) + uVar36;
      uVar37 = (uVar37 << 0xd | uVar37 >> 0x33) ^ uVar50;
      uVar36 = (uVar36 << 0x10 | uVar36 >> 0x30) ^ uVar41;
      uVar41 = uVar41 + uVar37;
      uVar44 = (uVar50 << 0x20 | uVar50 >> 0x20) + uVar36;
      uVar37 = (uVar37 << 0x11 | uVar37 >> 0x2f) ^ uVar41;
      uVar36 = (uVar36 << 0x15 | uVar36 >> 0x2b) ^ uVar44;
      uVar41 = (uVar41 << 0x20 | uVar41 >> 0x20) + uVar36;
      uVar37 = (uVar37 << 0xd | uVar37 >> 0x33) ^ uVar44 + uVar37;
      uVar36 = (uVar36 << 0x10 | uVar36 >> 0x30) ^ uVar41;
      uVar41 = uVar41 + uVar37;
      uVar41 = (uVar41 << 0x20 | uVar41 >> 0x20) ^ (uVar37 << 0x11 | uVar37 >> 0x2f) ^
               (uVar36 << 0x15 | uVar36 >> 0x2b) ^ uVar41;
      bVar56 = (byte)(uVar41 >> 0x39);
      auVar59 = ZEXT216(CONCAT11(bVar56,bVar56));
      auVar59 = pshuflw(auVar59,auVar59,0);
      cVar57 = auVar59[0];
      cVar60 = auVar59[1];
      cVar61 = auVar59[2];
      cVar62 = auVar59[3];
      pauVar43 = pauStack_1f8 + -1;
      puVar54 = (undefined8 *)0x0;
      while( true ) {
        uVar41 = uVar41 & (ulonglong)pcVar6;
        pcVar1 = *pauVar39 + uVar41;
        cVar12 = *pcVar1;
        cVar13 = pcVar1[1];
        cVar14 = pcVar1[2];
        cVar15 = pcVar1[3];
        cVar16 = pcVar1[4];
        cVar17 = pcVar1[5];
        cVar18 = pcVar1[6];
        cVar19 = pcVar1[7];
        cVar20 = pcVar1[8];
        cVar21 = pcVar1[9];
        cVar22 = pcVar1[10];
        cVar23 = pcVar1[0xb];
        cVar24 = pcVar1[0xc];
        cVar25 = pcVar1[0xd];
        cVar26 = pcVar1[0xe];
        cVar27 = pcVar1[0xf];
        auVar58[0] = -(cVar12 == cVar57);
        auVar58[1] = -(cVar13 == cVar60);
        auVar58[2] = -(cVar14 == cVar61);
        auVar58[3] = -(cVar15 == cVar62);
        auVar58[4] = -(cVar16 == cVar57);
        auVar58[5] = -(cVar17 == cVar60);
        auVar58[6] = -(cVar18 == cVar61);
        auVar58[7] = -(cVar19 == cVar62);
        auVar58[8] = -(cVar20 == cVar57);
        auVar58[9] = -(cVar21 == cVar60);
        auVar58[10] = -(cVar22 == cVar61);
        auVar58[0xb] = -(cVar23 == cVar62);
        auVar58[0xc] = -(cVar24 == cVar57);
        auVar58[0xd] = -(cVar25 == cVar60);
        auVar58[0xe] = -(cVar26 == cVar61);
        auVar58[0xf] = -(cVar27 == cVar62);
        uVar52 = (ushort)(SUB161(auVar58 >> 7,0) & 1) | (ushort)(SUB161(auVar58 >> 0xf,0) & 1) << 1
                 | (ushort)(SUB161(auVar58 >> 0x17,0) & 1) << 2 |
                 (ushort)(SUB161(auVar58 >> 0x1f,0) & 1) << 3 |
                 (ushort)(SUB161(auVar58 >> 0x27,0) & 1) << 4 |
                 (ushort)(SUB161(auVar58 >> 0x2f,0) & 1) << 5 |
                 (ushort)(SUB161(auVar58 >> 0x37,0) & 1) << 6 |
                 (ushort)(SUB161(auVar58 >> 0x3f,0) & 1) << 7 |
                 (ushort)(SUB161(auVar58 >> 0x47,0) & 1) << 8 |
                 (ushort)(SUB161(auVar58 >> 0x4f,0) & 1) << 9 |
                 (ushort)(SUB161(auVar58 >> 0x57,0) & 1) << 10 |
                 (ushort)(SUB161(auVar58 >> 0x5f,0) & 1) << 0xb |
                 (ushort)(SUB161(auVar58 >> 0x67,0) & 1) << 0xc |
                 (ushort)(SUB161(auVar58 >> 0x6f,0) & 1) << 0xd |
                 (ushort)(SUB161(auVar58 >> 0x77,0) & 1) << 0xe | (ushort)(auVar58[0xf] >> 7) << 0xf
        ;
        uVar45 = (uint)uVar52;
        if (uVar52 != 0) {
          lVar34 = *(longlong *)pauStack_c0[-2];
          puStack_188 = puVar54;
          pauStack_f0 = pauVar43;
          do {
            uVar30 = 0;
            for (uVar8 = uVar45; (uVar8 & 1) == 0; uVar8 = uVar8 >> 1 | 0x80000000) {
              uVar30 = uVar30 + 1;
            }
            if ((lVar34 == *(longlong *)(pauStack_f0[-(uVar30 + uVar41 & (ulonglong)pcVar6)] + 8))
               && (iVar31 = func_0x000142923bf0(*(longlong *)(pauStack_c0[-3] + 8) + 0x10,
                                                *(longlong *)
                                                 pauStack_f0[-(uVar30 + uVar41 & (ulonglong)pcVar6)]
                                                + 0x10,lVar34), iVar31 == 0))
            goto code_r0x0001412a1190;
            uVar52 = (ushort)(uVar45 - 1) & (ushort)uVar45;
            uVar45 = CONCAT22((short)(uVar45 - 1 >> 0x10),uVar52);
            puVar54 = puStack_188;
            pauVar43 = pauStack_f0;
          } while (uVar52 != 0);
        }
        auVar79[0] = -(cVar12 == (char)iVar7);
        auVar79[1] = -(cVar13 == cVar29);
        auVar79[2] = -(cVar14 == cVar68);
        auVar79[3] = -(cVar15 == cVar69);
        auVar79[4] = -(cVar16 == (char)iVar9);
        auVar79[5] = -(cVar17 == cVar70);
        auVar79[6] = -(cVar18 == cVar71);
        auVar79[7] = -(cVar19 == cVar72);
        auVar79[8] = -(cVar20 == (char)iVar10);
        auVar79[9] = -(cVar21 == cVar73);
        auVar79[10] = -(cVar22 == cVar74);
        auVar79[0xb] = -(cVar23 == cVar75);
        auVar79[0xc] = -(cVar24 == (char)iVar11);
        auVar79[0xd] = -(cVar25 == cVar76);
        auVar79[0xe] = -(cVar26 == cVar77);
        auVar79[0xf] = -(cVar27 == cVar78);
        if ((((((((((((((((SUB161(auVar79 >> 7,0) & 1) != 0 || (SUB161(auVar79 >> 0xf,0) & 1) != 0)
                        || (SUB161(auVar79 >> 0x17,0) & 1) != 0) ||
                       (SUB161(auVar79 >> 0x1f,0) & 1) != 0) || (SUB161(auVar79 >> 0x27,0) & 1) != 0
                      ) || (SUB161(auVar79 >> 0x2f,0) & 1) != 0) ||
                    (SUB161(auVar79 >> 0x37,0) & 1) != 0) || (SUB161(auVar79 >> 0x3f,0) & 1) != 0)
                  || (SUB161(auVar79 >> 0x47,0) & 1) != 0) || (SUB161(auVar79 >> 0x4f,0) & 1) != 0)
                || (SUB161(auVar79 >> 0x57,0) & 1) != 0) || (SUB161(auVar79 >> 0x5f,0) & 1) != 0) ||
              (SUB161(auVar79 >> 0x67,0) & 1) != 0) || (SUB161(auVar79 >> 0x6f,0) & 1) != 0) ||
            (SUB161(auVar79 >> 0x77,0) & 1) != 0) || auVar79[0xf] < '\0') break;
        uVar41 = (longlong)puVar54 + uVar41 + 0x10;
        puVar54 = puVar54 + 2;
      }
    }
  }
  goto code_r0x0001412a0e22;
code_r0x0001412a1190:
  if (*(longlong *)(puVar47[0x11] + 0x20) != 0) {
    *(undefined4 *)puStack_d0 = puStack_d8._0_4_;
    uVar36 = puVar47[0x12];
    plVar33 = *(longlong **)(pauStack_c0[-3] + 8);
    uVar5 = *(undefined8 *)pauStack_c0[-2];
    LOCK();
    lVar34 = *plVar33;
    *plVar33 = *plVar33 + 1;
    UNLOCK();
    if (*plVar33 == 0 || SCARRY8(lVar34,1) != *plVar33 < 0) goto code_r0x0001412a16f9;
    uStack_b1 = 1;
    FUN_140a37760(uStack_e0,(char)uVar36,plVar33,uVar5,puVar47[0x11] + 0x10);
  }
  goto code_r0x0001412a0f80;
}
