// 0x141eaec50 undefined FUN_141eaec50(void) body_bytes=2840

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void FUN_141eaec50(longlong *param_1,longlong *param_2,undefined8 *param_3,undefined8 *param_4,
                  longlong *param_5,undefined8 *param_6)

{
  byte *pbVar1;
  undefined4 uVar2;
  byte bVar3;
  byte bVar4;
  byte bVar5;
  undefined4 uVar6;
  undefined8 uVar7;
  code *pcVar8;
  float fVar9;
  float fVar10;
  float fVar11;
  char cVar12;
  uint uVar13;
  longlong lVar14;
  ulonglong uVar15;
  longlong *plVar16;
  longlong lVar17;
  undefined8 *puVar18;
  undefined *puVar19;
  uint *puVar20;
  undefined4 *puVar21;
  ulonglong uVar22;
  undefined *puVar23;
  longlong lVar24;
  undefined4 *puVar25;
  undefined *puVar26;
  ulonglong uVar27;
  undefined4 *puVar28;
  undefined1 uVar29;
  float fVar30;
  int iStack_348;
  undefined4 uStack_344;
  undefined4 uStack_340;
  undefined4 uStack_33c;
  undefined4 uStack_338;
  undefined4 uStack_334;
  undefined4 uStack_330;
  undefined4 uStack_32c;
  undefined4 uStack_2a0;
  float fStack_29c;
  undefined4 uStack_298;
  float fStack_294;
  int aiStack_1f0 [2];
  longlong lStack_1e8;
  undefined4 *puStack_1e0;
  longlong lStack_1d8;
  int aiStack_1d0 [2];
  uint *puStack_1c8;
  undefined4 *puStack_1c0;
  longlong lStack_1b8;
  longlong lStack_1b0;
  undefined1 auStack_1a8 [8];
  undefined8 uStack_1a0;
  undefined1 uStack_198;
  longlong lStack_190;
  longlong lStack_188;
  int iStack_180;
  undefined4 uStack_17c;
  undefined4 uStack_178;
  undefined4 uStack_174;
  undefined4 uStack_170;
  undefined4 uStack_16c;
  undefined4 uStack_168;
  undefined4 uStack_164;
  longlong lStack_150;
  longlong lStack_148;
  float *pfStack_140;
  longlong lStack_138;
  undefined8 uStack_130;
  longlong lStack_128;
  undefined4 uStack_11c;
  undefined4 uStack_118;
  undefined4 uStack_114;
  longlong lStack_110;
  int iStack_108;
  undefined4 uStack_104;
  undefined4 uStack_100;
  undefined4 uStack_fc;
  undefined4 uStack_f8;
  undefined4 uStack_f4;
  undefined4 uStack_f0;
  undefined4 uStack_ec;
  undefined4 *puStack_e8;
  undefined4 *puStack_e0;
  float fStack_d4;
  undefined4 *puStack_d0;
  undefined4 *puStack_c8;
  longlong lStack_c0;
  longlong *plStack_b8;
  uint *puStack_b0;
  longlong lStack_a8;
  longlong *plStack_a0;
  undefined8 uStack_98;
  undefined8 uStack_90;
  
  uStack_90 = 0xfffffffffffffffe;
  puVar18 = param_2;
  cVar12 = func_0x000140c16a00();
  if (cVar12 == '\0') {
LAB_141eaed06:
    lStack_138 = 0;
  }
  else {
    lVar24 = 0x3f;
    lVar14 = FUN_141eae820(0x3f);
    puVar19 = &UNK_142f985a8;
    if (lVar14 == 1) {
      if (puVar18 != (undefined8 *)0x0) {
        if (puVar18 < (undefined8 *)0x3f) {
          if (*(char *)(puVar18 + 0x285f30b5) < -0x40) {
LAB_141eaece6:
            FUN_1428d9940(&UNK_142f985a8,0x3f,0,puVar18,&UNK_142f98670);
            goto LAB_141eaed06;
          }
        }
        else if (puVar18 != (undefined8 *)0x3f) goto LAB_141eaece6;
      }
      lVar14 = FUN_141eae820(puVar18);
      if (lVar14 == 1) {
        uVar22 = (longlong)puVar18 + 2;
        if (uVar22 != 0) {
          uVar29 = uVar22 == 0x3f;
          if (0x3e < uVar22) goto LAB_141eaed5d;
          uVar29 = *(char *)((longlong)puVar18 + 0x142f985aa) == -0x41;
          if (*(char *)((longlong)puVar18 + 0x142f985aa) < -0x40) {
            do {
              puVar18 = (undefined8 *)0x3f;
              FUN_1428d9940(&UNK_142f985a8,0x3f);
LAB_141eaed5d:
            } while (!(bool)uVar29);
          }
        }
        lVar24 = 0x3d - (longlong)puVar18;
        puVar19 = &UNK_142f985a8 + uVar22;
      }
    }
    puVar23 = &UNK_142f986e4;
    puVar26 = &UNK_142f986b8;
    do {
      bVar3 = puVar23[-1];
      uVar13 = (uint)(char)bVar3;
      plStack_a0 = param_2;
      if ((char)bVar3 < '\0') {
        bVar4 = puVar23[-2];
        if ((char)bVar4 < -0x40) {
          bVar5 = puVar23[-3];
          if ((char)bVar5 < -0x40) {
            pbVar1 = puVar23 + -4;
            puVar23 = puVar23 + -4;
            uVar13 = bVar5 & 0x3f | (*pbVar1 & 7) << 6;
          }
          else {
            puVar23 = puVar23 + -3;
            uVar13 = bVar5 & 0xf;
          }
          uVar13 = bVar4 & 0x3f | uVar13 << 6;
        }
        else {
          puVar23 = puVar23 + -2;
          uVar13 = bVar4 & 0x1f;
        }
        uVar13 = (uint)(bVar3 & 0x3f) | uVar13 << 6;
        if (uVar13 != 0x5c) goto LAB_141eaee04;
LAB_141eaee19:
        puVar26 = puVar23 + -0x142f986b7;
        if (puVar26 != (undefined *)0x0) {
          uVar29 = puVar26 == (undefined *)0x2c;
          if ((undefined *)0x2b < puVar26) goto LAB_141eaee58;
          uVar29 = puVar23[1] == -0x41;
          if ((char)puVar23[1] < -0x40) {
            do {
              FUN_1428d9940(&UNK_142f986b8,0x2c);
LAB_141eaee58:
            } while (!(bool)uVar29);
          }
        }
        lVar14 = 0x2c - (longlong)puVar26;
        puVar26 = &UNK_142f986b8 + (longlong)puVar26;
        goto LAB_141eaee69;
      }
      puVar23 = puVar23 + -1;
      if (uVar13 == 0x5c) goto LAB_141eaee19;
LAB_141eaee04:
      if (uVar13 == 0x2f) goto LAB_141eaee19;
    } while (puVar23 != &UNK_142f986b8);
    lVar14 = 0x2c;
LAB_141eaee69:
    plVar16 = (longlong *)func_0x000140c18130();
    if (*plVar16 == 1) {
      plVar16 = plVar16 + 1;
    }
    else if ((*plVar16 != 0) ||
            (plVar16 = (longlong *)FUN_1428e6250(plVar16,0), plVar16 == (longlong *)0x0)) {
LAB_141eaf713:
      FUN_1428d80d0(&UNK_142bb52d8);
LAB_141eaf71f:
      uStack_f8 = uStack_170;
      uStack_f4 = uStack_16c;
      uStack_f0 = uStack_168;
      uStack_ec = uStack_164;
      iStack_108 = iStack_180;
      uStack_104 = uStack_17c;
      uStack_100 = uStack_178;
      uStack_fc = uStack_174;
      FUN_1428d9760(&UNK_143052c50,0x2b,&iStack_108,&UNK_143052c30,&UNK_143052e68);
LAB_141eaf7a6:
                    /* WARNING: Does not return */
      pcVar8 = (code *)invalidInstructionException();
      (*pcVar8)();
    }
    if (*plVar16 != 0) {
LAB_141eaf7a8:
      FUN_1428d9250(&UNK_142bb5348);
      pcVar8 = (code *)swi(3);
      (*pcVar8)();
      return;
    }
    *plVar16 = -1;
    plStack_b8 = plVar16;
    uStack_130 = FUN_140c16d40(plVar16 + 1,puVar19,lVar24,puVar26,lVar14,1,0);
    *plStack_b8 = *plStack_b8 + 1;
    lStack_138 = 1;
    param_2 = plStack_a0;
  }
  fVar11 = _DAT_142a16cb8;
  fVar10 = _DAT_1429345a8;
  fVar9 = _DAT_142925904;
  lStack_1b0 = *param_2;
  auStack_1a8[0] = 0;
  uStack_1a0 = *param_3;
  uStack_198 = 1;
  lStack_188 = 2;
  lStack_148 = *param_5;
  pfStack_140 = (float *)*param_6;
  lStack_c0 = *param_1;
  lStack_110 = param_1[1];
  uStack_114 = *(undefined4 *)((longlong)param_1 + 0x14);
  plStack_a0 = (longlong *)CONCAT44(plStack_a0._4_4_,*(undefined4 *)((longlong)param_4 + 0x1c));
  uVar7 = *param_4;
  plStack_b8 = (longlong *)param_4[2];
  lVar14 = 0;
LAB_141eaefae:
  if (lStack_188 != lVar14) {
    lStack_190 = lVar14 + 1;
    lStack_150 = *(longlong *)(auStack_1a8 + lVar14 * 0x10 + -8);
    puStack_d0 = (undefined4 *)CONCAT71(puStack_d0._1_7_,auStack_1a8[lVar14 * 0x10]);
    uStack_11c = *(undefined4 *)(lStack_150 + 0x20);
    uStack_118 = *(undefined4 *)(lStack_150 + 0x24);
    uVar2 = *(undefined4 *)(lStack_150 + 0x28);
    fStack_d4 = *(float *)(lStack_148 + 0xe8) * fVar10 * *pfStack_140;
    puVar20 = *(uint **)(lStack_c0 + 0xf8);
    puStack_b0 = puVar20 + *(longlong *)(lStack_c0 + 0x100);
    puStack_e8 = (undefined4 *)CONCAT71(puStack_e8._1_7_,*(undefined1 *)(lStack_c0 + 0x108));
    lStack_128 = 8;
    uVar22 = 0;
    lStack_a8 = 0;
    uStack_98 = (undefined4 *)((ulonglong)uStack_98 & 0xffffffff00000000);
    uVar15 = 0;
LAB_141eaf073:
    uVar27 = uVar15;
    if ((char)puStack_e8 == '\0') {
      if (uVar27 == uVar22) {
        if (puVar20 == puStack_b0) goto LAB_141eaf300;
        while( true ) {
          uVar22 = *(ulonglong *)
                    (*(longlong *)(lStack_110 + 0x100) + 0x58 + (ulonglong)*puVar20 * 0xa0);
          if (uVar22 != 0) break;
          puVar20 = puVar20 + 1;
          if (puVar20 == puStack_b0) goto LAB_141eaf300;
        }
        lVar14 = *(longlong *)(lStack_110 + 0x100) + (ulonglong)*puVar20 * 0xa0;
        uVar15 = *(ulonglong *)(lStack_c0 + 0x118);
        if (uVar15 < *(ulonglong *)(lVar14 + 0x88)) {
          uStack_98 = (undefined4 *)
                      CONCAT44(uStack_98._4_4_,
                               (int)CONCAT71((int7)(uVar15 >> 8),
                                             *(longlong *)
                                              (*(longlong *)(lVar14 + 0x80) + uVar15 * 8) != 0));
        }
        else {
          uStack_98 = (undefined4 *)((ulonglong)uStack_98._4_4_ << 0x20);
        }
        lVar24 = *(longlong *)
                  (*(longlong *)(lStack_110 + 0x1a8) + 0x18 +
                  (ulonglong)*(uint *)(lVar14 + 0x94) * 0x48);
        lVar17 = ~*(ulonglong *)
                   (*(longlong *)
                     (*(longlong *)(lStack_110 + 0x1a8) + 0x38 +
                     (ulonglong)*(uint *)(lVar14 + 0x94) * 0x48) +
                   *(longlong *)(lStack_c0 + 0x110) * 8) * 0x30;
        lStack_a8 = *(longlong *)(lVar24 + 0x10 + lVar17);
        puStack_c8 = *(undefined4 **)(lVar24 + 0x28 + lVar17);
        puVar20 = puVar20 + 1;
        lStack_128 = *(longlong *)(lVar14 + 0x50);
        uVar27 = 0;
      }
      uVar15 = (ulonglong)*(uint *)(lStack_128 + 8 + uVar27 * 0x10);
    }
    else {
      if (uVar27 == uVar22) {
        if (puVar20 == puStack_b0) goto LAB_141eaf300;
        while( true ) {
          uVar22 = *(ulonglong *)
                    (*(longlong *)(lStack_110 + 0x1a8) + 0x10 + (ulonglong)*puVar20 * 0x48);
          if (uVar22 != 0) break;
          puVar20 = puVar20 + 1;
          if (puVar20 == puStack_b0) goto LAB_141eaf300;
        }
        lVar14 = *(longlong *)(lStack_110 + 0x1a8) + (ulonglong)*puVar20 * 0x48;
        uVar15 = *(ulonglong *)(lStack_c0 + 0x118);
        if (uVar15 < *(ulonglong *)(lVar14 + 0x40)) {
          uStack_98 = (undefined4 *)
                      CONCAT44(uStack_98._4_4_,
                               (int)CONCAT71((int7)(uVar15 >> 8),
                                             *(longlong *)
                                              (*(longlong *)(lVar14 + 0x38) + uVar15 * 8) != 0));
        }
        else {
          uStack_98 = (undefined4 *)((ulonglong)uStack_98._4_4_ << 0x20);
        }
        lVar24 = ~*(ulonglong *)
                   (*(longlong *)(lVar14 + 0x38) + *(longlong *)(lStack_c0 + 0x110) * 8) * 0x30;
        lStack_a8 = *(longlong *)(*(longlong *)(lVar14 + 0x18) + 0x10 + lVar24);
        puStack_c8 = *(undefined4 **)(*(longlong *)(lVar14 + 0x18) + 0x28 + lVar24);
        puVar20 = puVar20 + 1;
        uVar27 = 0;
      }
      uVar15 = uVar27 & 0xffffffff;
    }
    lVar14 = lStack_a8 + uVar15 * 0x14;
    puStack_e0 = puStack_c8 + uVar15;
    if (((ulonglong)uStack_98 & 1) == 0) goto LAB_141eaf2a0;
    uVar15 = uVar27 + 1;
    if ((char)puStack_d0 != '\0') goto LAB_141eaf2b0;
    goto LAB_141eaf073;
  }
  if (lStack_138 == 0) {
    return;
  }
  plVar16 = (longlong *)func_0x000140c18130();
  if (*plVar16 == 1) {
    plVar16 = plVar16 + 1;
  }
  else if ((*plVar16 != 0) ||
          (plVar16 = (longlong *)FUN_1428e6250(plVar16,0), plVar16 == (longlong *)0x0))
  goto LAB_141eaf713;
  if (*plVar16 == 0) {
    *plVar16 = -1;
    plStack_a0 = plVar16;
    FUN_140c170f0(plVar16 + 1,uStack_130);
    *plStack_a0 = *plStack_a0 + 1;
    return;
  }
  goto LAB_141eaf7a8;
LAB_141eaf300:
  *(undefined4 *)plStack_b8 = plStack_a0._0_4_;
  FUN_142375db0(uVar7,uStack_11c,uStack_118,&fStack_d4);
  fVar30 = fStack_d4;
  puStack_c8 = *(undefined4 **)(lStack_150 + 8) + *(longlong *)(lStack_150 + 0x10) * 8;
  puVar25 = *(undefined4 **)(lStack_150 + 8);
  puVar21 = puStack_c8;
  while (puVar28 = puVar25, lVar14 = lStack_190, puVar28 != puVar21) {
    puVar25 = puVar28 + 8;
    if ((puVar28[4] != 0) &&
       (FUN_140fe7950(aiStack_1f0,uVar7,puVar28[4],puVar28[5]), aiStack_1f0[0] == 4)) {
      puStack_e0 = puVar25;
      lStack_a8 = lStack_1e8;
      puStack_d0 = puStack_1e0;
      uStack_98 = puStack_1e0 + lStack_1d8 * 2;
      puVar25 = puStack_1e0;
      puVar21 = puStack_1e0 + lStack_1d8 * 2;
      while (puVar25 != puVar21) {
        uVar2 = *puVar25;
        uVar6 = puVar25[1];
        *(undefined4 *)plStack_b8 = plStack_a0._0_4_;
        FUN_140fe7cb0(&iStack_108,uVar7,uVar2,uVar6);
        if (iStack_108 != 4) {
          uStack_338 = uStack_f8;
          uStack_334 = uStack_f4;
          uStack_330 = uStack_f0;
          uStack_32c = uStack_ec;
          iStack_348 = iStack_108;
          uStack_344 = uStack_104;
          uStack_340 = uStack_100;
          uStack_33c = uStack_fc;
          FUN_1428d9760(&UNK_143052c50,0x2b,&iStack_348,&UNK_143052c30,&UNK_143052e50);
          goto LAB_141eaf7a6;
        }
        FUN_142374fd0(&iStack_348,CONCAT44(uStack_fc,uStack_100));
        uStack_2a0 = 0;
        fStack_29c = fVar30;
        uStack_298 = 0;
        fStack_294 = fVar30;
        FUN_140fe7b10(&iStack_180,uVar7,uVar2,uVar6,&iStack_348);
        if (iStack_180 != 4) goto LAB_141eaf71f;
        FUN_140fe7950(aiStack_1d0,uVar7,uVar2,uVar6);
        puVar25 = puVar25 + 2;
        if (aiStack_1d0[0] == 4) {
          puStack_b0 = puStack_1c8;
          puVar21 = puStack_1c0 + lStack_1b8 * 2;
          puStack_e8 = puStack_1c0;
          for (puVar28 = puStack_1c0; puVar28 != puVar21; puVar28 = puVar28 + 2) {
            uVar2 = *puVar28;
            uVar6 = puVar28[1];
            *(undefined4 *)plStack_b8 = plStack_a0._0_4_;
            FUN_140fe7cb0(&iStack_108,uVar7,uVar2,uVar6);
            if (iStack_108 != 4) {
              uStack_338 = uStack_f8;
              uStack_334 = uStack_f4;
              uStack_330 = uStack_f0;
              uStack_32c = uStack_ec;
              iStack_348 = iStack_108;
              uStack_344 = uStack_104;
              uStack_340 = uStack_100;
              uStack_33c = uStack_fc;
              FUN_1428d9760(&UNK_143052c50,0x2b,&iStack_348,&UNK_143052c30,&UNK_143052e50);
              goto LAB_141eaf7a6;
            }
            FUN_142374fd0(&iStack_348,CONCAT44(uStack_fc,uStack_100));
            uStack_2a0 = 0;
            fStack_29c = fVar30;
            uStack_298 = 0;
            fStack_294 = fVar30;
            FUN_140fe7b10(&iStack_180,uVar7,uVar2,uVar6,&iStack_348);
            if (iStack_180 != 4) {
              uStack_f8 = uStack_170;
              uStack_f4 = uStack_16c;
              uStack_f0 = uStack_168;
              uStack_ec = uStack_164;
              iStack_108 = iStack_180;
              uStack_104 = uStack_17c;
              uStack_100 = uStack_178;
              uStack_fc = uStack_174;
              FUN_1428d9760(&UNK_143052c50,0x2b,&iStack_108,&UNK_143052c30,&UNK_143052e68);
              goto LAB_141eaf7a6;
            }
          }
          puVar21 = uStack_98;
          if (puStack_b0 != (uint *)0x0) {
            func_0x000140613c20(puStack_e8,(longlong)puStack_b0 << 3,4);
            puVar21 = uStack_98;
          }
        }
      }
      puVar25 = puStack_e0;
      puVar21 = puStack_c8;
      if (lStack_a8 != 0) {
        func_0x000140613c20(puStack_d0,lStack_a8 << 3,4);
        puVar25 = puStack_e0;
        puVar21 = puStack_c8;
      }
    }
  }
  goto LAB_141eaefae;
LAB_141eaf2a0:
  uVar15 = uVar27 + 1;
  if ((char)puStack_d0 == '\0') {
LAB_141eaf2b0:
    fVar30 = (float)FUN_140c95ef0(&UNK_142f986e4,uVar2);
    *puStack_e0 = uStack_114;
    *(float *)(lVar14 + 0x10) = ((fVar9 - fVar30) * fVar11 + fVar30) * *(float *)(lVar14 + 0x10);
    uVar15 = uVar27 + 1;
  }
  goto LAB_141eaf073;
}

