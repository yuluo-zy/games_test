country_core::systems::wall::set_wall_style::set_wall_style :: systems/wall/set_wall_style.rs
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */
void FUN_1409a2290(undefined8 *param_1,undefined8 *param_2,undefined8 *param_3,undefined8 *param_4,
                  longlong *param_5,undefined8 *param_6,longlong *param_7)
{
  undefined8 uVar1;
  undefined1 auVar2 [16];
  undefined1 uVar3;
  undefined1 uVar4;
  undefined8 uVar5;
  ulonglong *puVar6;
  code *pcVar7;
  uint uVar8;
  uint uVar9;
  int iVar10;
  int iVar11;
  int iVar12;
  int iVar13;
  float fVar14;
  undefined4 *puVar15;
  undefined4 uVar16;
  uint uVar17;
  longlong lVar18;
  ulonglong uVar19;
  longlong lVar20;
  longlong lVar21;
  ulonglong uVar22;
  ulonglong uVar23;
  longlong lVar24;
  longlong lVar25;
  ulonglong uVar26;
  ushort uVar27;
  longlong lVar28;
  byte bVar29;
  char cVar30;
  char cVar34;
  float extraout_XMM0_Da;
  char cVar32;
  char cVar33;
  undefined1 auVar31 [16];
  undefined1 auVar35 [16];
  undefined1 auVar36 [16];
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
  float fVar37;
  float fVar38;
  float fVar39;
  longlong lStack_270;
  longlong lStack_268;
  undefined8 uStack_260;
  longlong lStack_258;
  undefined4 *puStack_178;
  longlong *plStack_170;
  longlong lStack_168;
  undefined8 *puStack_160;
  undefined8 *puStack_158;
  undefined8 uStack_150;
  undefined4 *puStack_148;
  undefined4 *puStack_140;
  undefined8 uStack_138;
  ulonglong *puStack_130;
  longlong lStack_128;
  longlong lStack_120;
  longlong lStack_118;
  longlong *plStack_110;
  longlong lStack_108;
  ulonglong *puStack_100;
  undefined8 uStack_f8;
  undefined8 uStack_f0;
  undefined8 uStack_e8;
  undefined8 uStack_d8;
  longlong lStack_d0;
  longlong lStack_c8;
  undefined4 uStack_bc;
  undefined4 uStack_b8;
  undefined1 uStack_b3;
  undefined1 uStack_b2;
  undefined1 uStack_b1;
  undefined8 uStack_b0;
  uStack_b0 = 0xfffffffffffffffe;
  puStack_100 = (ulonglong *)*param_1;
  lVar24 = param_1[1];
  uVar19 = *puStack_100;
  uVar23 = 0;
  uVar22 = uVar19 - *(ulonglong *)(lVar24 + 0x18);
  if (uVar19 < *(ulonglong *)(lVar24 + 0x18)) {
    uVar22 = uVar23;
  }
  uVar26 = uVar19 - *(ulonglong *)(lVar24 + 0x38);
  if (uVar19 < *(ulonglong *)(lVar24 + 0x38)) {
    uVar26 = uVar23;
  }
  lVar18 = uVar22 * 0x18 + *(longlong *)(lVar24 + 8);
  uVar19 = *(ulonglong *)(lVar24 + 0x10) - uVar22;
  if (*(ulonglong *)(lVar24 + 0x10) < uVar22) {
    lVar18 = 8;
    uVar19 = uVar23;
  }
  lVar21 = uVar26 * 0x18 + *(longlong *)(lVar24 + 0x28);
  uVar22 = *(ulonglong *)(lVar24 + 0x30) - uVar26;
  if (*(ulonglong *)(lVar24 + 0x30) < uVar26) {
    lVar21 = 8;
    uVar22 = uVar23;
  }
  *puStack_100 = (*(longlong *)(lVar24 + 0x40) - uVar22) - uVar19;
  fVar14 = _UNK_142925904;
  puStack_130 = (ulonglong *)(lVar18 + uVar19 * 0x18);
  lStack_128 = lVar21 + uVar22 * 0x18;
  uStack_bc = *(undefined4 *)((longlong)param_4 + 0x1c);
  uStack_150 = *param_4;
  puStack_148 = (undefined4 *)param_4[2];
  uStack_b8 = *(undefined4 *)((longlong)param_6 + 0x1c);
  uStack_138 = *param_6;
  puStack_140 = (undefined4 *)param_6[2];
  plStack_110 = (longlong *)*param_5;
  lStack_120 = CONCAT44(lStack_120._4_4_,*(undefined4 *)((longlong)param_7 + 0x1c));
  lStack_118 = *param_7;
  puStack_178 = (undefined4 *)param_7[2];
  iVar10 = -(uint)(CONCAT13(unaff_XMM7_Bd,
                            CONCAT12(unaff_XMM7_Bc,CONCAT11(unaff_XMM7_Bb,unaff_XMM7_Ba))) ==
                  CONCAT13(unaff_XMM7_Bd,
                           CONCAT12(unaff_XMM7_Bc,CONCAT11(unaff_XMM7_Bb,unaff_XMM7_Ba))));
  iVar11 = -(uint)(CONCAT13(unaff_XMM7_Bh,
                            CONCAT12(unaff_XMM7_Bg,CONCAT11(unaff_XMM7_Bf,unaff_XMM7_Be))) ==
                  CONCAT13(unaff_XMM7_Bh,
                           CONCAT12(unaff_XMM7_Bg,CONCAT11(unaff_XMM7_Bf,unaff_XMM7_Be))));
  iVar12 = -(uint)(CONCAT13(unaff_XMM7_Bl,
                            CONCAT12(unaff_XMM7_Bk,CONCAT11(unaff_XMM7_Bj,unaff_XMM7_Bi))) ==
                  CONCAT13(unaff_XMM7_Bl,
                           CONCAT12(unaff_XMM7_Bk,CONCAT11(unaff_XMM7_Bj,unaff_XMM7_Bi))));
  iVar13 = -(uint)(CONCAT13(unaff_XMM7_Bp,
                            CONCAT12(unaff_XMM7_Bo,CONCAT11(unaff_XMM7_Bn,unaff_XMM7_Bm))) ==
                  CONCAT13(unaff_XMM7_Bp,
                           CONCAT12(unaff_XMM7_Bo,CONCAT11(unaff_XMM7_Bn,unaff_XMM7_Bm))));
  plStack_170 = plStack_110 + 4;
  lStack_168 = lStack_118 + 0x20;
  puStack_160 = param_3;
  puStack_158 = param_2;
  while( true ) {
    do {
      if ((lVar18 == 0) || ((ulonglong *)lVar18 == puStack_130)) {
        if ((lVar21 == 0) || (lVar21 == lStack_128)) {
          puVar6 = (ulonglong *)*puStack_160;
          lVar24 = puStack_160[1];
          uVar19 = *puVar6;
          uVar23 = 0;
          uVar22 = uVar19 - *(ulonglong *)(lVar24 + 0x18);
          if (uVar19 < *(ulonglong *)(lVar24 + 0x18)) {
            uVar22 = uVar23;
          }
          uVar26 = uVar19 - *(ulonglong *)(lVar24 + 0x38);
          if (uVar19 < *(ulonglong *)(lVar24 + 0x38)) {
            uVar26 = uVar23;
          }
          lVar18 = uVar22 * 0x20 + *(longlong *)(lVar24 + 8);
          uVar19 = *(ulonglong *)(lVar24 + 0x10) - uVar22;
          if (*(ulonglong *)(lVar24 + 0x10) < uVar22) {
            lVar18 = 8;
            uVar19 = uVar23;
          }
          lVar21 = uVar26 * 0x20 + *(longlong *)(lVar24 + 0x28);
          uVar22 = *(ulonglong *)(lVar24 + 0x30) - uVar26;
          if (*(ulonglong *)(lVar24 + 0x30) < uVar26) {
            lVar21 = 8;
            uVar22 = uVar23;
          }
          *puVar6 = (*(longlong *)(lVar24 + 0x40) - uVar22) - uVar19;
          lStack_108 = uVar19 * 0x20 + lVar18;
          puStack_100 = (ulonglong *)(uVar22 * 0x20 + lVar21);
          do {
            puVar15 = puStack_148;
            uVar5 = uStack_150;
            if ((lVar18 == 0) || (lVar18 == lStack_108)) {
              if ((lVar21 == 0) || ((ulonglong *)lVar21 == puStack_100)) {
                puStack_130 = (ulonglong *)*puStack_158;
                lVar24 = puStack_158[1];
                uVar19 = *puStack_130;
                uVar23 = 0;
                uVar22 = uVar19 - *(ulonglong *)(lVar24 + 0x18);
                if (uVar19 < *(ulonglong *)(lVar24 + 0x18)) {
                  uVar22 = uVar23;
                }
                uVar26 = uVar19 - *(ulonglong *)(lVar24 + 0x38);
                if (uVar19 < *(ulonglong *)(lVar24 + 0x38)) {
                  uVar26 = uVar23;
                }
                lVar18 = uVar22 * 0x18 + *(longlong *)(lVar24 + 8);
                uVar19 = *(ulonglong *)(lVar24 + 0x10) - uVar22;
                if (*(ulonglong *)(lVar24 + 0x10) < uVar22) {
                  lVar18 = 8;
                  uVar19 = uVar23;
                }
                puStack_100 = (ulonglong *)(uVar26 * 0x18 + *(longlong *)(lVar24 + 0x28));
                uVar22 = *(ulonglong *)(lVar24 + 0x30) - uVar26;
                if (*(ulonglong *)(lVar24 + 0x30) < uVar26) {
                  puStack_100 = (ulonglong *)0x8;
                  uVar22 = uVar23;
                }
                *puStack_130 = (*(longlong *)(lVar24 + 0x40) - uVar22) - uVar19;
                lStack_128 = lVar18 + uVar19 * 0x18;
                lStack_120 = (longlong)puStack_100 + uVar22 * 0x18;
                do {
                  uVar16 = uStack_bc;
                  if ((lVar18 == 0) || (lVar18 == lStack_128)) {
                    if ((puStack_100 == (ulonglong *)0x0) ||
                       (puStack_100 == (ulonglong *)lStack_120)) {
                      return;
                    }
                    lVar18 = 0;
                    lVar24 = (longlong)puStack_100;
                    puStack_100 = (ulonglong *)((longlong)puStack_100 + 0x18);
                  }
                  else {
                    lVar24 = lVar18;
                    lVar18 = lVar18 + 0x18;
                  }
                  *puStack_130 = *puStack_130 + 1;
                  lStack_118 = CONCAT71(lStack_118._1_7_,*(undefined1 *)(lVar24 + 0x10));
                  uVar3 = *(undefined1 *)(lVar24 + 0x11);
                  uVar1 = *(undefined8 *)(lVar24 + 8);
                  lStack_108 = CONCAT71(lStack_108._1_7_,*(undefined1 *)(lVar24 + 0x12));
                  *puVar15 = uStack_bc;
                  plStack_110 = (longlong *)puStack_100;
                  lStack_270._0_1_ = uVar3;
                  FUN_140aa99e0(uVar5,uVar1,&lStack_270);
                  *puVar15 = uVar16;
                  lStack_270 = CONCAT71(lStack_270._1_7_,uVar3);
                  FUN_140aa9eb0(uVar5,uVar1,&lStack_270);
                  if ((char)lStack_108 == '\x01') {
                    uStack_d8 = 0;
                    lStack_d0 = 8;
                    lStack_c8 = 0;
                    uVar4 = (undefined1)lStack_118;
                    uStack_260._0_2_ = CONCAT11(uVar3,(undefined1)lStack_118);
                    lStack_270 = -0x7ffffffffffffff6;
                    uStack_b1 = 1;
                    lStack_268 = uVar1;
                    FUN_140a82b40(&uStack_d8,&lStack_270);
                    uStack_260._0_2_ = CONCAT11(uVar3,uVar4);
                    lStack_270 = -0x7ffffffffffffff9;
                    uStack_b1 = 1;
                    lStack_268 = uVar1;
                    FUN_140a82b40(&uStack_d8,&lStack_270);
                    *puStack_140 = uStack_b8;
                    uStack_f8 = 0;
                    uStack_f0 = 8;
                    uStack_e8 = 0;
                    lStack_258 = lStack_d0 + lStack_c8 * 0x18;
                    lStack_270 = lStack_d0;
                    lStack_268 = lStack_d0;
                    uStack_260 = uStack_d8;
                    lVar24 = lStack_d0;
                    lStack_268 = FUN_141696f50(&lStack_270);
                    lStack_270 = 7;
                    uStack_b1 = 0;
                    uStack_260 = lVar24;
                    FUN_140a84ec0(uStack_138,&lStack_270,&UNK_142abc910);
                    puStack_100 = (ulonglong *)plStack_110;
                  }
                } while( true );
              }
              lVar24 = lVar21 + 0x20;
              lVar18 = 0;
              lVar20 = lVar21;
            }
            else {
              lVar20 = lVar18;
              lVar24 = lVar21;
              lVar18 = lVar18 + 0x20;
            }
            *puVar6 = *puVar6 + 1;
            uVar5 = *(undefined8 *)(lVar20 + 0x10);
            uVar1 = *(undefined8 *)(lVar20 + 8);
            cVar30 = *(char *)(lVar20 + 0x18);
            *puStack_148 = uStack_bc;
            lStack_270 = CONCAT44(lStack_270._4_4_,(int)((ulonglong)uVar5 >> 0x20));
            FUN_140aaa530(uStack_150,uVar1,&lStack_270);
            lVar21 = lVar24;
            if (cVar30 != '\0') {
              uStack_d8 = 0;
              lStack_d0 = 8;
              lStack_c8 = 0;
              lStack_270 = -0x7ffffffffffffff7;
              uStack_b2 = 1;
              lStack_268 = uVar1;
              uStack_260 = uVar5;
              FUN_140a82b40(&uStack_d8,&lStack_270);
              *puStack_140 = uStack_b8;
              uStack_f8 = 0;
              uStack_f0 = 8;
              uStack_e8 = 0;
              lStack_258 = lStack_d0 + lStack_c8 * 0x18;
              lStack_270 = lStack_d0;
              lStack_268 = lStack_d0;
              uStack_260 = uStack_d8;
              lVar24 = lStack_d0;
              lStack_268 = FUN_141696f50(&lStack_270);
              lStack_270 = 7;
              uStack_b2 = 0;
              uStack_260 = lVar24;
              FUN_140a84ec0(uStack_138,&lStack_270,&UNK_142abc928);
            }
          } while( true );
        }
        lVar18 = 0;
        lVar24 = lVar21;
        lVar21 = lVar21 + 0x18;
      }
      else {
        lVar24 = lVar18;
        lVar18 = lVar18 + 0x18;
      }
      *puStack_100 = *puStack_100 + 1;
      uVar3 = *(undefined1 *)(lVar24 + 0x10);
      uVar4 = *(undefined1 *)(lVar24 + 0x11);
      lVar20 = *(longlong *)(lVar24 + 8);
      cVar30 = *(char *)(lVar24 + 0x13);
      cVar32 = *(char *)(lVar24 + 0x12);
      *puStack_148 = uStack_bc;
      lStack_108 = CONCAT71(lStack_108._1_7_,uVar4);
      lStack_270 = CONCAT71(lStack_270._1_7_,uVar4);
      FUN_140aaa350(uStack_150);
      if (cVar32 != '\0') {
        uStack_d8 = 0;
        lStack_d0 = 8;
        lStack_c8 = 0;
        uStack_260._0_2_ = CONCAT11((char)lStack_108,uVar3);
        lStack_270 = -0x7ffffffffffffff8;
        uStack_b3 = 1;
        lStack_268 = lVar20;
        FUN_140a82b40(&uStack_d8,&lStack_270);
        *puStack_140 = uStack_b8;
        uStack_f8 = 0;
        uStack_f0 = 8;
        uStack_e8 = 0;
        lStack_258 = lStack_d0 + lStack_c8 * 0x18;
        lStack_270 = lStack_d0;
        lStack_268 = lStack_d0;
        uStack_260 = uStack_d8;
        lVar24 = lStack_d0;
        lStack_268 = FUN_141696f50(&lStack_270);
        lStack_270 = 7;
        uStack_b3 = 0;
        uStack_260 = lVar24;
        FUN_140a84ec0(uStack_138);
      }
    } while (cVar30 == '\0');
    lStack_270 = lVar20;
    if (plStack_110[3] != 0) break;
code_r0x0001409a26d0:
    fVar37 = 0.0;
    fVar38 = 0.0;
    fVar39 = 0.0;
code_r0x0001409a277d:
    *puStack_178 = (undefined4)lStack_120;
    lVar24 = *(longlong *)(lStack_118 + 0x30);
    uVar5 = *(undefined8 *)(lStack_118 + 0x40);
    if (lVar24 == *(longlong *)(lStack_118 + 0x20)) {
      FUN_140562c30(lStack_168,&UNK_142c3a148);
    }
    lVar20 = *(longlong *)(lStack_118 + 0x28);
    lVar25 = lVar24 * 0x20;
    *(undefined8 *)(lVar20 + lVar25) = uVar5;
    *(undefined1 *)(lVar20 + 8 + lVar25) = 0x10;
    *(char *)(lVar20 + 9 + lVar25) = (char)lStack_108;
    *(ulonglong *)(lVar20 + 0x10 + lVar25) = CONCAT44(fVar38,fVar37);
    *(float *)(lVar20 + 0x18 + lVar25) = fVar39;
    *(longlong *)(lStack_118 + 0x30) = lVar24 + 1;
    *(longlong *)(lStack_118 + 0x40) = *(longlong *)(lStack_118 + 0x40) + 1;
  }
  uVar19 = FUN_1405b0d70(plStack_170,&lStack_270);
  lVar24 = *plStack_110;
  bVar29 = (byte)(uVar19 >> 0x39);
  auVar31 = ZEXT216(CONCAT11(bVar29,bVar29));
  auVar31 = pshuflw(auVar31,auVar31,0);
  lVar25 = 0;
  do {
    uVar19 = plStack_110[1] & uVar19;
    auVar2 = *(undefined1 (*) [16])(lVar24 + uVar19);
    cVar30 = auVar31[0];
    auVar36[0] = -(auVar2[0] == cVar30);
    cVar32 = auVar31[1];
    auVar36[1] = -(auVar2[1] == cVar32);
    cVar33 = auVar31[2];
    auVar36[2] = -(auVar2[2] == cVar33);
    cVar34 = auVar31[3];
    auVar36[3] = -(auVar2[3] == cVar34);
    auVar36[4] = -(auVar2[4] == cVar30);
    auVar36[5] = -(auVar2[5] == cVar32);
    auVar36[6] = -(auVar2[6] == cVar33);
    auVar36[7] = -(auVar2[7] == cVar34);
    auVar36[8] = -(auVar2[8] == cVar30);
    auVar36[9] = -(auVar2[9] == cVar32);
    auVar36[10] = -(auVar2[10] == cVar33);
    auVar36[0xb] = -(auVar2[0xb] == cVar34);
    auVar36[0xc] = -(auVar2[0xc] == cVar30);
    auVar36[0xd] = -(auVar2[0xd] == cVar32);
    auVar36[0xe] = -(auVar2[0xe] == cVar33);
    auVar36[0xf] = -(auVar2[0xf] == cVar34);
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
    uVar17 = (uint)uVar27;
    while (uVar27 != 0) {
      uVar8 = 0;
      for (uVar9 = uVar17; (uVar9 & 1) == 0; uVar9 = uVar9 >> 1 | 0x80000000) {
        uVar8 = uVar8 + 1;
      }
      lVar28 = (uVar8 + uVar19 & plStack_110[1]) * -0x2a8;
      if (*(longlong *)(lVar24 + -0x2a8 + lVar28) == lVar20) {
        if ((*(ulonglong *)(lVar24 + -0x290 + lVar28) < 2) ||
           (*(float *)(lVar24 + -0x270 + lVar28) <= 0.0)) {
          uVar19 = FUN_1428d9430(&UNK_142ada193,0x32,&UNK_142abc958);
        }
        else {
          lVar24 = lVar24 + lVar28;
          uVar17 = FUN_141467ce0(lVar24 + -0x268);
          uVar22 = (ulonglong)uVar17;
          if (*(ulonglong *)(lVar24 + -600) <= uVar22) goto code_r0x0001409a2cf4;
          uVar19 = uVar22 + 1;
          if (uVar19 < *(ulonglong *)(lVar24 + -600)) {
            lVar24 = *(longlong *)(lVar24 + -0x260);
            fVar39 = fVar14 - extraout_XMM0_Da;
            uVar5 = *(undefined8 *)(lVar24 + uVar22 * 0xc);
            uVar1 = *(undefined8 *)(lVar24 + uVar19 * 0xc);
            fVar37 = extraout_XMM0_Da * (float)uVar1 + (float)uVar5 * fVar39;
            fVar38 = extraout_XMM0_Da * (float)((ulonglong)uVar1 >> 0x20) +
                     (float)((ulonglong)uVar5 >> 0x20) * fVar39;
            fVar39 = *(float *)(lVar24 + 8 + uVar19 * 0xc) * extraout_XMM0_Da +
                     fVar39 * *(float *)(lVar24 + 8 + uVar22 * 0xc);
            goto code_r0x0001409a277d;
          }
        }
        FUN_1428d9518(uVar19);
code_r0x0001409a2cf4:
        FUN_1428d9518();
        pcVar7 = (code *)swi(3);
        (*pcVar7)();
        return;
      }
      uVar27 = (ushort)(uVar17 - 1) & (ushort)uVar17;
      uVar17 = CONCAT22((short)(uVar17 - 1 >> 0x10),uVar27);
    }
    auVar35[0] = -(auVar2[0] == (char)iVar10);
    auVar35[1] = -(auVar2[1] == (char)((uint)iVar10 >> 8));
    auVar35[2] = -(auVar2[2] == (char)((uint)iVar10 >> 0x10));
    auVar35[3] = -(auVar2[3] == (char)((uint)iVar10 >> 0x18));
    auVar35[4] = -(auVar2[4] == (char)iVar11);
    auVar35[5] = -(auVar2[5] == (char)((uint)iVar11 >> 8));
    auVar35[6] = -(auVar2[6] == (char)((uint)iVar11 >> 0x10));
    auVar35[7] = -(auVar2[7] == (char)((uint)iVar11 >> 0x18));
    auVar35[8] = -(auVar2[8] == (char)iVar12);
    auVar35[9] = -(auVar2[9] == (char)((uint)iVar12 >> 8));
    auVar35[10] = -(auVar2[10] == (char)((uint)iVar12 >> 0x10));
    auVar35[0xb] = -(auVar2[0xb] == (char)((uint)iVar12 >> 0x18));
    auVar35[0xc] = -(auVar2[0xc] == (char)iVar13);
    auVar35[0xd] = -(auVar2[0xd] == (char)((uint)iVar13 >> 8));
    auVar35[0xe] = -(auVar2[0xe] == (char)((uint)iVar13 >> 0x10));
    auVar35[0xf] = -(auVar2[0xf] == (char)((uint)iVar13 >> 0x18));
    if ((((((((((((((((SUB161(auVar35 >> 7,0) & 1) != 0 || (SUB161(auVar35 >> 0xf,0) & 1) != 0) ||
                    (SUB161(auVar35 >> 0x17,0) & 1) != 0) || (SUB161(auVar35 >> 0x1f,0) & 1) != 0)
                  || (SUB161(auVar35 >> 0x27,0) & 1) != 0) || (SUB161(auVar35 >> 0x2f,0) & 1) != 0)
                || (SUB161(auVar35 >> 0x37,0) & 1) != 0) || (SUB161(auVar35 >> 0x3f,0) & 1) != 0) ||
              (SUB161(auVar35 >> 0x47,0) & 1) != 0) || (SUB161(auVar35 >> 0x4f,0) & 1) != 0) ||
            (SUB161(auVar35 >> 0x57,0) & 1) != 0) || (SUB161(auVar35 >> 0x5f,0) & 1) != 0) ||
          (SUB161(auVar35 >> 0x67,0) & 1) != 0) || (SUB161(auVar35 >> 0x6f,0) & 1) != 0) ||
        (SUB161(auVar35 >> 0x77,0) & 1) != 0) || auVar35[0xf] < '\0') goto code_r0x0001409a26d0;
    uVar19 = uVar19 + lVar25 + 0x10;
    lVar25 = lVar25 + 0x10;
  } while( true );
}
