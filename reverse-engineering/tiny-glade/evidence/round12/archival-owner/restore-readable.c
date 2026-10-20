
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void FUN_140817a40(undefined8 *param_1,undefined8 *param_2,undefined8 *param_3,undefined8 *param_4,
                  longlong *param_5,undefined8 *param_6,undefined8 *param_7,undefined8 *param_8)

{
  char *pcVar1;
  undefined1 auVar2 [16];
  undefined4 uVar3;
  int iVar4;
  undefined4 *puVar5;
  undefined8 uVar6;
  code *pcVar7;
  uint uVar8;
  uint uVar9;
  int iVar10;
  int iVar11;
  int iVar12;
  int iVar13;
  undefined1 auVar14 [16];
  longlong lVar15;
  longlong lVar16;
  ulonglong uVar17;
  longlong *plVar18;
  longlong lVar19;
  ulonglong uVar20;
  ulonglong uVar21;
  undefined1 (*pauVar22) [16];
  undefined1 (*pauVar23) [16];
  longlong lVar24;
  ulonglong uVar25;
  ulonglong uVar26;
  ulonglong uVar27;
  ushort uVar28;
  longlong lVar29;
  undefined1 (*pauVar30) [16];
  uint uVar31;
  uint uVar32;
  longlong lVar33;
  undefined8 *puVar34;
  bool bVar35;
  bool bVar36;
  bool bVar37;
  bool bVar38;
  bool bVar39;
  char cVar40;
  byte bVar41;
  char cVar43;
  char cVar44;
  char cVar45;
  undefined1 auVar42 [16];
  undefined1 auVar46 [16];
  undefined1 auVar47 [16];
  undefined1 auVar48 [16];
  undefined1 auVar49 [16];
  undefined1 auVar50 [16];
  undefined1 auVar51 [16];
  undefined1 auVar52 [16];
  undefined1 auVar53 [16];
  undefined1 auVar54 [16];
  undefined1 unaff_XMM9_Ba;
  undefined1 unaff_XMM9_Bb;
  char cVar55;
  undefined1 unaff_XMM9_Bc;
  char cVar56;
  undefined1 unaff_XMM9_Bd;
  char cVar57;
  undefined1 unaff_XMM9_Be;
  undefined1 unaff_XMM9_Bf;
  char cVar58;
  undefined1 unaff_XMM9_Bg;
  char cVar59;
  undefined1 unaff_XMM9_Bh;
  char cVar60;
  undefined1 unaff_XMM9_Bi;
  undefined1 unaff_XMM9_Bj;
  char cVar61;
  undefined1 unaff_XMM9_Bk;
  char cVar62;
  undefined1 unaff_XMM9_Bl;
  char cVar63;
  undefined1 unaff_XMM9_Bm;
  undefined1 unaff_XMM9_Bn;
  char cVar64;
  undefined1 unaff_XMM9_Bo;
  char cVar65;
  undefined1 unaff_XMM9_Bp;
  char cVar66;
  undefined4 uStack_21c;
  longlong lStack_218;
  longlong lStack_210;
  undefined1 auStack_208 [16];
  undefined1 auStack_1f8 [16];
  undefined1 auStack_1e8 [16];
  longlong lStack_1d8;
  ulonglong uStack_1d0;
  undefined8 uStack_1c8;
  undefined8 uStack_1c0;
  longlong lStack_1b8;
  ulonglong uStack_1b0;
  undefined8 uStack_1a8;
  undefined4 uStack_19c;
  undefined8 *puStack_198;
  longlong lStack_190;
  longlong lStack_188;
  longlong lStack_180;
  longlong lStack_178;
  undefined8 uStack_170;
  undefined8 uStack_168;
  undefined8 uStack_160;
  undefined8 uStack_158;
  longlong *plStack_150;
  undefined8 uStack_148;
  undefined8 uStack_140;
  undefined8 uStack_138;
  undefined8 uStack_130;
  undefined4 uStack_128;
  undefined4 uStack_124;
  undefined8 *puStack_118;
  undefined8 uStack_110;
  longlong lStack_108;
  longlong lStack_100;
  longlong lStack_f8;
  longlong lStack_f0;
  undefined1 auStack_e8 [24];
  byte abStack_d0 [8];
  longlong lStack_c8;
  ulonglong uStack_c0;
  longlong lStack_b8;
  longlong lStack_b0;
  undefined1 (*pauStack_a8) [16];
  ushort uStack_98;
  longlong lStack_90;
  
  uVar3 = *(undefined4 *)((longlong)param_4 + 0x1c);
  puVar5 = (undefined4 *)param_4[2];
  *puVar5 = uVar3;
  uStack_1c0 = *param_4;
  func_0x000140a1c2b0();
  lVar19 = param_8[1];
  uVar17 = *(ulonglong *)*param_8;
  uVar20 = 0;
  uVar21 = uVar17 - *(ulonglong *)(lVar19 + 0x18);
  if (uVar17 < *(ulonglong *)(lVar19 + 0x18)) {
    uVar21 = uVar20;
  }
  uVar27 = uVar17 - *(ulonglong *)(lVar19 + 0x38);
  if (uVar17 < *(ulonglong *)(lVar19 + 0x38)) {
    uVar27 = uVar20;
  }
  bVar35 = *(ulonglong *)(lVar19 + 0x30) < uVar27;
  uVar27 = *(ulonglong *)(lVar19 + 0x30) - uVar27;
  bVar38 = uVar27 != 0;
  if (bVar35) {
    uVar27 = uVar20;
  }
  bVar36 = *(ulonglong *)(lVar19 + 0x10) < uVar21;
  uVar21 = *(ulonglong *)(lVar19 + 0x10) - uVar21;
  bVar39 = uVar21 != 0;
  if (bVar36) {
    uVar21 = uVar20;
  }
  uVar17 = uVar21 + uVar27;
  if (!bVar36 && bVar39 || !bVar35 && bVar38) {
    uVar17 = uVar20;
  }
  *(ulonglong *)*param_8 = *(longlong *)(lVar19 + 0x40) - uVar17;
  if (((!bVar36 && bVar39 || !bVar35 && bVar38) || (*(int *)*param_1 != 9)) ||
     ((char)((int *)*param_1)[1] != '\x01')) {
    *puVar5 = uVar3;
    uVar6 = *param_3;
    uStack_110 = *param_2;
    *(undefined4 *)param_5[2] = *(undefined4 *)((longlong)param_5 + 0x1c);
    lStack_188 = *param_5;
    *(undefined4 *)param_6[2] = *(undefined4 *)((longlong)param_6 + 0x1c);
    uStack_170 = *param_6;
    puStack_118 = param_3;
    func_0x00014003bb80(uStack_1c0);
    uStack_1a8 = uVar6;
    FUN_140b2bb80(auStack_208,uVar6);
    uVar6 = auStack_208._8_8_;
    lStack_210 = auStack_1f8._8_8_;
    uVar32 = (uint)(ushort)uStack_1d0;
    lVar19 = auStack_1f8._0_8_;
    pauVar22 = (undefined1 (*) [16])auStack_1e8._8_8_;
    lVar24 = auStack_208._0_8_;
    lVar29 = auStack_1e8._0_8_;
    lVar33 = uStack_1c8;
code_r0x000140817bc0:
    if (lVar19 == 5) {
code_r0x000140817c9a:
      if (lVar24 == 4) goto code_r0x000140817d15;
      auStack_208._0_8_ = 3;
      if (lVar24 == 3) goto code_r0x000140817d15;
      lVar15 = lVar24;
      lVar19 = 5;
      lVar16 = uVar6;
      lVar24 = 3;
    }
    else {
      if (lVar29 == 0) {
code_r0x000140817c79:
        if (lVar19 != 4) {
          auStack_1f8._0_8_ = 3;
          if (lVar19 != 3) {
            lVar29 = 0;
            lVar15 = lVar19;
            lVar19 = 3;
            lVar16 = lStack_210;
            goto code_r0x000140817cc0;
          }
        }
        auStack_1f8._0_8_ = 5;
        lVar29 = 0;
        goto code_r0x000140817c9a;
      }
      if (lVar33 == 0) {
        lVar33 = 0;
code_r0x000140817c70:
        auVar2._8_8_ = 0;
        auVar2._0_8_ = auStack_1e8._8_8_;
        auStack_1e8 = auVar2 << 0x40;
        goto code_r0x000140817c79;
      }
      if ((short)uVar32 == 0) {
        do {
          auVar2 = *pauVar22;
          uVar28 = (ushort)(SUB161(auVar2 >> 7,0) & 1) | (ushort)(SUB161(auVar2 >> 0xf,0) & 1) << 1
                   | (ushort)(SUB161(auVar2 >> 0x17,0) & 1) << 2 |
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
                   (ushort)(byte)(auVar2[0xf] >> 7) << 0xf;
          lVar29 = lVar29 + -0x980;
          pauVar22 = pauVar22 + 1;
        } while (uVar28 == 0xffff);
        auStack_1e8._8_8_ = pauVar22;
        auStack_1e8._0_8_ = lVar29;
        uVar31 = ~(uint)uVar28;
        uVar32 = uVar31 & -(uint)uVar28 - 2;
      }
      else {
        uVar31 = uVar32;
        uVar32 = uVar32 - 1 & uVar32;
      }
      uStack_1d0 = CONCAT62(uStack_1d0._2_6_,(short)uVar32);
      lVar33 = lVar33 + -1;
      uVar8 = 0;
      for (; (uVar31 & 1) == 0; uVar31 = uVar31 >> 1 | 0x80000000) {
        uVar8 = uVar8 + 1;
      }
      lVar16 = lVar19;
      uStack_1c8 = lVar33;
      lVar15 = func_0x0001408bc520(*(undefined8 *)(lVar29 + -0x98 + (ulonglong)uVar8 * -0x98));
      if (lVar15 == 3) goto code_r0x000140817c70;
    }
code_r0x000140817cc0:
    lVar16 = FUN_140a1cde0(uStack_1c0,lVar15,lVar16);
    if (lVar16 != 0) {
      FUN_141257900(lVar16,lVar15);
    }
    goto code_r0x000140817bc0;
  }
code_r0x000140818928:
  *(undefined4 *)param_7[2] = *(undefined4 *)((longlong)param_7 + 0x1c);
  FUN_1408ff210(*param_7);
  if (!bVar36 && bVar39 || !bVar35 && bVar38) {
    *(undefined4 *)param_3[2] = *(undefined4 *)((longlong)param_3 + 0x1c);
    FUN_140b2cb30(auStack_208,*param_3,uStack_1c0,1);
    FUN_140b743e0(auStack_208,auStack_1e8,0x70,0x10);
  }
  return;
code_r0x000140817d15:
  func_0x000140b2bb00(abStack_d0,uStack_1a8);
  auVar14 = _UNK_1429cd090;
  auVar2 = _UNK_1429cd080;
  lStack_190 = lStack_c8;
  uStack_1b0 = uStack_c0;
  lStack_218 = lStack_b0;
  lStack_178 = lStack_90;
  lStack_100 = lStack_188 + 0x30;
  iVar10 = -(uint)(CONCAT13(unaff_XMM9_Bd,
                            CONCAT12(unaff_XMM9_Bc,CONCAT11(unaff_XMM9_Bb,unaff_XMM9_Ba))) ==
                  CONCAT13(unaff_XMM9_Bd,
                           CONCAT12(unaff_XMM9_Bc,CONCAT11(unaff_XMM9_Bb,unaff_XMM9_Ba))));
  cVar55 = (char)((uint)iVar10 >> 8);
  cVar56 = (char)((uint)iVar10 >> 0x10);
  cVar57 = (char)((uint)iVar10 >> 0x18);
  iVar11 = -(uint)(CONCAT13(unaff_XMM9_Bh,
                            CONCAT12(unaff_XMM9_Bg,CONCAT11(unaff_XMM9_Bf,unaff_XMM9_Be))) ==
                  CONCAT13(unaff_XMM9_Bh,
                           CONCAT12(unaff_XMM9_Bg,CONCAT11(unaff_XMM9_Bf,unaff_XMM9_Be))));
  cVar58 = (char)((uint)iVar11 >> 8);
  cVar59 = (char)((uint)iVar11 >> 0x10);
  cVar60 = (char)((uint)iVar11 >> 0x18);
  iVar12 = -(uint)(CONCAT13(unaff_XMM9_Bl,
                            CONCAT12(unaff_XMM9_Bk,CONCAT11(unaff_XMM9_Bj,unaff_XMM9_Bi))) ==
                  CONCAT13(unaff_XMM9_Bl,
                           CONCAT12(unaff_XMM9_Bk,CONCAT11(unaff_XMM9_Bj,unaff_XMM9_Bi))));
  cVar61 = (char)((uint)iVar12 >> 8);
  cVar62 = (char)((uint)iVar12 >> 0x10);
  cVar63 = (char)((uint)iVar12 >> 0x18);
  iVar13 = -(uint)(CONCAT13(unaff_XMM9_Bp,
                            CONCAT12(unaff_XMM9_Bo,CONCAT11(unaff_XMM9_Bn,unaff_XMM9_Bm))) ==
                  CONCAT13(unaff_XMM9_Bp,
                           CONCAT12(unaff_XMM9_Bo,CONCAT11(unaff_XMM9_Bn,unaff_XMM9_Bm))));
  cVar64 = (char)((uint)iVar13 >> 8);
  cVar65 = (char)((uint)iVar13 >> 0x10);
  cVar66 = (char)((uint)iVar13 >> 0x18);
  pauVar22 = pauStack_a8;
  lVar29 = lStack_108;
  uVar32 = (uint)uStack_98;
  uVar6 = uStack_110;
  lVar19 = lStack_c8;
  lVar24 = lStack_b8;
code_r0x000140817de0:
  lVar33 = lVar24;
  lVar16 = lStack_218;
  if (uStack_1b0 == 2) goto code_r0x000140817f1c;
  if (lStack_218 != 0) {
    bVar37 = lStack_178 == 0;
    lStack_178 = lStack_178 + -1;
    if (bVar37) {
      lStack_178 = 0;
      goto joined_r0x000140817f14;
    }
    if ((short)uVar32 == 0) {
      do {
        auVar42 = *pauVar22;
        uVar28 = (ushort)(SUB161(auVar42 >> 7,0) & 1) | (ushort)(SUB161(auVar42 >> 0xf,0) & 1) << 1
                 | (ushort)(SUB161(auVar42 >> 0x17,0) & 1) << 2 |
                 (ushort)(SUB161(auVar42 >> 0x1f,0) & 1) << 3 |
                 (ushort)(SUB161(auVar42 >> 0x27,0) & 1) << 4 |
                 (ushort)(SUB161(auVar42 >> 0x2f,0) & 1) << 5 |
                 (ushort)(SUB161(auVar42 >> 0x37,0) & 1) << 6 |
                 (ushort)(SUB161(auVar42 >> 0x3f,0) & 1) << 7 |
                 (ushort)(SUB161(auVar42 >> 0x47,0) & 1) << 8 |
                 (ushort)(SUB161(auVar42 >> 0x4f,0) & 1) << 9 |
                 (ushort)(SUB161(auVar42 >> 0x57,0) & 1) << 10 |
                 (ushort)(SUB161(auVar42 >> 0x5f,0) & 1) << 0xb |
                 (ushort)(SUB161(auVar42 >> 0x67,0) & 1) << 0xc |
                 (ushort)(SUB161(auVar42 >> 0x6f,0) & 1) << 0xd |
                 (ushort)(SUB161(auVar42 >> 0x77,0) & 1) << 0xe |
                 (ushort)(byte)(auVar42[0xf] >> 7) << 0xf;
        lStack_218 = lStack_218 + -0x980;
        pauVar22 = pauVar22 + 1;
      } while (uVar28 == 0xffff);
      uVar32 = ~(uint)uVar28;
      uVar31 = uVar32 & -(uint)uVar28 - 2;
      lVar16 = lStack_218;
    }
    else {
      uVar31 = uVar32 - 1 & uVar32;
    }
    uVar8 = 0;
    for (; (uVar32 & 1) == 0; uVar32 = uVar32 >> 1 | 0x80000000) {
      uVar8 = uVar8 + 1;
    }
    lStack_218 = lVar16;
    lVar15 = func_0x000140a1c2a0(lVar16 + ((ulonglong)uVar8 * -0x13 + -0x13) * 8);
    uVar32 = uVar31;
    if (lVar15 == 0) goto code_r0x000140817e78;
    if (lVar15 == 3) goto joined_r0x000140817f14;
    goto code_r0x000140817ee0;
  }
joined_r0x000140817f14:
  if ((uStack_1b0 & 1) == 0) {
    lVar16 = 0;
code_r0x000140817f1c:
    param_3 = puStack_118;
    if ((lVar19 != 0 & abStack_d0[0]) != 1) goto code_r0x000140818928;
    lVar15 = 2;
    uStack_1b0 = 2;
    lStack_190 = 0;
    lStack_218 = lVar16;
  }
  else {
    lVar15 = 1;
    lVar33 = 0;
    lVar16 = 0;
    if (lVar24 == 0) goto code_r0x000140817f1c;
    lStack_218 = 0;
    lVar33 = 0;
    lVar29 = lStack_180;
    lStack_190 = lVar19;
    lVar19 = lVar24;
  }
  puVar34 = (undefined8 *)(lVar19 + 0x30);
  goto code_r0x000140817f5e;
code_r0x000140817e78:
  lVar15 = FUN_14085a560(uVar6,lVar16);
  lVar29 = lVar16;
  lStack_180 = lVar16;
  if (lVar15 != 0) {
    lVar15 = 0;
code_r0x000140817ee0:
    puVar34 = (undefined8 *)(lStack_218 + ((ulonglong)uVar8 * -0x13 + -0xc) * 8);
    lVar29 = lVar16;
    lStack_180 = lVar16;
code_r0x000140817f5e:
    lVar15 = FUN_140a1cf30(uStack_1c0,lVar15,lVar29);
    lStack_108 = lVar29;
    func_0x000140a1b430(auStack_e8);
    lVar16 = puVar34[3];
    lVar29 = lStack_108;
    uVar6 = uStack_110;
    lVar19 = lStack_190;
    lVar24 = lVar33;
    if (lVar16 != 0) {
      pauVar30 = (undefined1 (*) [16])*puVar34;
      pauVar23 = pauVar30 + 1;
      auVar42 = *pauVar30;
      uVar31 = ~(uint)(ushort)((ushort)(SUB161(auVar42 >> 7,0) & 1) |
                               (ushort)(SUB161(auVar42 >> 0xf,0) & 1) << 1 |
                               (ushort)(SUB161(auVar42 >> 0x17,0) & 1) << 2 |
                               (ushort)(SUB161(auVar42 >> 0x1f,0) & 1) << 3 |
                               (ushort)(SUB161(auVar42 >> 0x27,0) & 1) << 4 |
                               (ushort)(SUB161(auVar42 >> 0x2f,0) & 1) << 5 |
                               (ushort)(SUB161(auVar42 >> 0x37,0) & 1) << 6 |
                               (ushort)(SUB161(auVar42 >> 0x3f,0) & 1) << 7 |
                               (ushort)(SUB161(auVar42 >> 0x47,0) & 1) << 8 |
                               (ushort)(SUB161(auVar42 >> 0x4f,0) & 1) << 9 |
                               (ushort)(SUB161(auVar42 >> 0x57,0) & 1) << 10 |
                               (ushort)(SUB161(auVar42 >> 0x5f,0) & 1) << 0xb |
                               (ushort)(SUB161(auVar42 >> 0x67,0) & 1) << 0xc |
                               (ushort)(SUB161(auVar42 >> 0x6f,0) & 1) << 0xd |
                               (ushort)(SUB161(auVar42 >> 0x77,0) & 1) << 0xe |
                              (ushort)(byte)(auVar42[0xf] >> 7) << 0xf);
      lStack_f0 = lVar15 + 0x60;
      lStack_f8 = lVar15 + 0x30;
      lStack_210 = CONCAT44(lStack_210._4_4_,uVar32);
      lStack_1b8 = lVar33;
      puStack_198 = puVar34;
      do {
        if ((short)uVar31 == 0) {
          do {
            auVar42 = *pauVar23;
            uVar28 = (ushort)(SUB161(auVar42 >> 7,0) & 1) |
                     (ushort)(SUB161(auVar42 >> 0xf,0) & 1) << 1 |
                     (ushort)(SUB161(auVar42 >> 0x17,0) & 1) << 2 |
                     (ushort)(SUB161(auVar42 >> 0x1f,0) & 1) << 3 |
                     (ushort)(SUB161(auVar42 >> 0x27,0) & 1) << 4 |
                     (ushort)(SUB161(auVar42 >> 0x2f,0) & 1) << 5 |
                     (ushort)(SUB161(auVar42 >> 0x37,0) & 1) << 6 |
                     (ushort)(SUB161(auVar42 >> 0x3f,0) & 1) << 7 |
                     (ushort)(SUB161(auVar42 >> 0x47,0) & 1) << 8 |
                     (ushort)(SUB161(auVar42 >> 0x4f,0) & 1) << 9 |
                     (ushort)(SUB161(auVar42 >> 0x57,0) & 1) << 10 |
                     (ushort)(SUB161(auVar42 >> 0x5f,0) & 1) << 0xb |
                     (ushort)(SUB161(auVar42 >> 0x67,0) & 1) << 0xc |
                     (ushort)(SUB161(auVar42 >> 0x6f,0) & 1) << 0xd |
                     (ushort)(SUB161(auVar42 >> 0x77,0) & 1) << 0xe |
                     (ushort)(byte)(auVar42[0xf] >> 7) << 0xf;
            pauVar30 = pauVar30 + -0x50;
            pauVar23 = pauVar23 + 1;
          } while (uVar28 == 0xffff);
          uVar32 = ~(uint)uVar28;
          uVar31 = uVar32 & -(uint)uVar28 - 2;
        }
        else {
          uVar32 = uVar31;
          uVar31 = uVar31 - 1 & uVar31;
        }
        uVar8 = 0;
        for (; (uVar32 & 1) == 0; uVar32 = uVar32 >> 1 | 0x80000000) {
          uVar8 = uVar8 + 1;
        }
        uVar17 = (ulonglong)uVar8;
        uVar3 = *(undefined4 *)pauVar30[uVar17 * -5 + -5];
        uStack_140 = *(undefined8 *)pauVar30[uVar17 * -5 + -2];
        uStack_124 = *(undefined4 *)(pauVar30[uVar17 * -5 + -1] + 0xc);
        uStack_128 = *(undefined4 *)(pauVar30[uVar17 * -5 + -1] + 8);
        uStack_138 = *(undefined8 *)(pauVar30[uVar17 * -5 + -2] + 8);
        uStack_130 = *(undefined8 *)((longlong)(pauVar30[uVar17 * -5 + -2] + 8) + 8);
        uStack_168 = *(undefined8 *)(pauVar30[uVar17 * -5 + -5] + 8);
        uStack_160 = *(undefined8 *)((longlong)(pauVar30[uVar17 * -5 + -5] + 8) + 8);
        uStack_158 = *(undefined8 *)(pauVar30[uVar17 * -5 + -4] + 8);
        plStack_150 = *(longlong **)((longlong)(pauVar30[uVar17 * -5 + -4] + 8) + 8);
        uStack_148 = *(undefined8 *)(pauVar30[uVar17 * -5 + -3] + 8);
        auStack_1e8 = *(undefined1 (*) [16])(lVar15 + 0x20);
        auVar42._0_8_ = auStack_1e8._0_8_;
        auVar42._8_4_ = auStack_1e8._0_4_;
        auVar42._12_4_ = auStack_1e8._4_4_;
        auStack_208 = auVar42 ^ auVar2;
        auVar46._0_8_ = auStack_1e8._8_8_;
        auVar46._8_4_ = auStack_1e8._8_4_;
        auVar46._12_4_ = auStack_1e8._12_4_;
        auStack_1f8 = auVar46 ^ auVar14;
        lStack_1d8 = 0;
        uStack_1d0 = 0;
        uStack_1c8 = 0;
        uStack_21c = uVar3;
        uStack_19c = uVar3;
        FUN_140106160(auStack_208,&uStack_19c,4);
        uVar26 = lStack_1d8 << 0x38 | uStack_1d0;
        uVar17 = auStack_1f8._8_8_ ^ uVar26;
        uVar27 = auStack_208._0_8_ + auStack_1f8._0_8_;
        uVar20 = (auStack_1f8._0_8_ << 0xd | (ulonglong)auStack_1f8._0_8_ >> 0x33) ^ uVar27;
        uVar21 = (uVar17 << 0x10 | uVar17 >> 0x30) ^ auStack_208._8_8_ + uVar17;
        uVar25 = auStack_208._8_8_ + uVar17 + uVar20;
        uVar27 = (uVar27 << 0x20 | uVar27 >> 0x20) + uVar21;
        uVar20 = (uVar20 << 0x11 | uVar20 >> 0x2f) ^ uVar25;
        uVar17 = (uVar21 << 0x15 | uVar21 >> 0x2b) ^ uVar27;
        uVar27 = (uVar27 ^ uVar26) + uVar20;
        uVar25 = ((uVar25 << 0x20 | uVar25 >> 0x20) ^ 0xff) + uVar17;
        uVar21 = (uVar20 << 0xd | uVar20 >> 0x33) ^ uVar27;
        uVar17 = (uVar17 << 0x10 | uVar17 >> 0x30) ^ uVar25;
        uVar25 = uVar25 + uVar21;
        uVar20 = (uVar27 << 0x20 | uVar27 >> 0x20) + uVar17;
        uVar21 = (uVar21 << 0x11 | uVar21 >> 0x2f) ^ uVar25;
        uVar17 = (uVar17 << 0x15 | uVar17 >> 0x2b) ^ uVar20;
        uVar20 = uVar20 + uVar21;
        uVar27 = (uVar25 << 0x20 | uVar25 >> 0x20) + uVar17;
        uVar21 = (uVar21 << 0xd | uVar21 >> 0x33) ^ uVar20;
        uVar17 = (uVar17 << 0x10 | uVar17 >> 0x30) ^ uVar27;
        uVar27 = uVar27 + uVar21;
        uVar20 = (uVar20 << 0x20 | uVar20 >> 0x20) + uVar17;
        uVar21 = (uVar21 << 0x11 | uVar21 >> 0x2f) ^ uVar27;
        uVar17 = (uVar17 << 0x15 | uVar17 >> 0x2b) ^ uVar20;
        uVar27 = (uVar27 << 0x20 | uVar27 >> 0x20) + uVar17;
        uVar21 = (uVar21 << 0xd | uVar21 >> 0x33) ^ uVar20 + uVar21;
        uVar17 = (uVar17 << 0x10 | uVar17 >> 0x30) ^ uVar27;
        uVar27 = uVar27 + uVar21;
        FUN_140b776d0(lVar15,(uVar27 << 0x20 | uVar27 >> 0x20) ^ (uVar21 << 0x11 | uVar21 >> 0x2f) ^
                             (uVar17 << 0x15 | uVar17 >> 0x2b) ^ uVar27,&uStack_21c);
        FUN_1402c9200(lStack_f0,uVar3);
        FUN_1411fbfa0(auStack_208,lStack_f8,uStack_21c,&uStack_168);
        if (uStack_1c8._7_1_ == '\x02') {
          FUN_140a1b4e0();
        }
        else {
          FUN_140a1b5a0(auStack_e8,uStack_21c);
        }
        lVar16 = lVar16 + -1;
      } while (lVar16 != 0);
      lVar33 = puStack_198[3];
      lVar29 = lStack_108;
      uVar32 = (uint)lStack_210;
      uVar6 = uStack_110;
      lVar19 = lStack_190;
      lVar24 = lStack_1b8;
      if (lVar33 != 0) {
        pauVar30 = (undefined1 (*) [16])*puStack_198;
        auVar42 = *pauVar30;
        pauVar23 = pauVar30 + 1;
        uVar32 = ~(uint)(ushort)((ushort)(SUB161(auVar42 >> 7,0) & 1) |
                                 (ushort)(SUB161(auVar42 >> 0xf,0) & 1) << 1 |
                                 (ushort)(SUB161(auVar42 >> 0x17,0) & 1) << 2 |
                                 (ushort)(SUB161(auVar42 >> 0x1f,0) & 1) << 3 |
                                 (ushort)(SUB161(auVar42 >> 0x27,0) & 1) << 4 |
                                 (ushort)(SUB161(auVar42 >> 0x2f,0) & 1) << 5 |
                                 (ushort)(SUB161(auVar42 >> 0x37,0) & 1) << 6 |
                                 (ushort)(SUB161(auVar42 >> 0x3f,0) & 1) << 7 |
                                 (ushort)(SUB161(auVar42 >> 0x47,0) & 1) << 8 |
                                 (ushort)(SUB161(auVar42 >> 0x4f,0) & 1) << 9 |
                                 (ushort)(SUB161(auVar42 >> 0x57,0) & 1) << 10 |
                                 (ushort)(SUB161(auVar42 >> 0x5f,0) & 1) << 0xb |
                                 (ushort)(SUB161(auVar42 >> 0x67,0) & 1) << 0xc |
                                 (ushort)(SUB161(auVar42 >> 0x6f,0) & 1) << 0xd |
                                 (ushort)(SUB161(auVar42 >> 0x77,0) & 1) << 0xe |
                                (ushort)(byte)(auVar42[0xf] >> 7) << 0xf);
        do {
          if ((short)uVar32 == 0) {
            do {
              auVar42 = *pauVar23;
              uVar28 = (ushort)(SUB161(auVar42 >> 7,0) & 1) |
                       (ushort)(SUB161(auVar42 >> 0xf,0) & 1) << 1 |
                       (ushort)(SUB161(auVar42 >> 0x17,0) & 1) << 2 |
                       (ushort)(SUB161(auVar42 >> 0x1f,0) & 1) << 3 |
                       (ushort)(SUB161(auVar42 >> 0x27,0) & 1) << 4 |
                       (ushort)(SUB161(auVar42 >> 0x2f,0) & 1) << 5 |
                       (ushort)(SUB161(auVar42 >> 0x37,0) & 1) << 6 |
                       (ushort)(SUB161(auVar42 >> 0x3f,0) & 1) << 7 |
                       (ushort)(SUB161(auVar42 >> 0x47,0) & 1) << 8 |
                       (ushort)(SUB161(auVar42 >> 0x4f,0) & 1) << 9 |
                       (ushort)(SUB161(auVar42 >> 0x57,0) & 1) << 10 |
                       (ushort)(SUB161(auVar42 >> 0x5f,0) & 1) << 0xb |
                       (ushort)(SUB161(auVar42 >> 0x67,0) & 1) << 0xc |
                       (ushort)(SUB161(auVar42 >> 0x6f,0) & 1) << 0xd |
                       (ushort)(SUB161(auVar42 >> 0x77,0) & 1) << 0xe |
                       (ushort)(byte)(auVar42[0xf] >> 7) << 0xf;
              pauVar30 = pauVar30 + -0x50;
              pauVar23 = pauVar23 + 1;
            } while (uVar28 == 0xffff);
            uVar31 = ~(uint)uVar28;
            uVar32 = uVar31 & -(uint)uVar28 - 2;
          }
          else {
            uVar31 = uVar32;
            uVar32 = uVar32 - 1 & uVar32;
          }
          uVar8 = 0;
          for (; (uVar31 & 1) == 0; uVar31 = uVar31 >> 1 | 0x80000000) {
            uVar8 = uVar8 + 1;
          }
          iVar4 = *(int *)pauVar30[(ulonglong)uVar8 * -5 + -5];
          plVar18 = (longlong *)func_0x000140005c60(uStack_1a8);
          if (plVar18[3] != 0) {
            auStack_1e8 = *(undefined1 (*) [16])(plVar18 + 4);
            auVar48._0_8_ = auStack_1e8._0_8_;
            auVar48._8_4_ = auStack_1e8._0_4_;
            auVar48._12_4_ = auStack_1e8._4_4_;
            auStack_208 = auVar48 ^ auVar2;
            auVar49._0_8_ = auStack_1e8._8_8_;
            auVar49._8_4_ = auStack_1e8._8_4_;
            auVar49._12_4_ = auStack_1e8._12_4_;
            auStack_1f8 = auVar49 ^ auVar14;
            lStack_1d8 = 0;
            uStack_1d0 = 0;
            uStack_1c8 = 0;
            uStack_168 = CONCAT44(uStack_168._4_4_,iVar4);
            FUN_140106160(auStack_208,&uStack_168,4);
            lVar24 = lStack_1b8;
            uVar26 = lStack_1d8 << 0x38 | uStack_1d0;
            uVar17 = auStack_1f8._8_8_ ^ uVar26;
            uVar25 = auStack_208._0_8_ + auStack_1f8._0_8_;
            uVar20 = (auStack_1f8._0_8_ << 0xd | (ulonglong)auStack_1f8._0_8_ >> 0x33) ^ uVar25;
            uVar21 = (uVar17 << 0x10 | uVar17 >> 0x30) ^ auStack_208._8_8_ + uVar17;
            uVar27 = auStack_208._8_8_ + uVar17 + uVar20;
            uVar25 = (uVar25 << 0x20 | uVar25 >> 0x20) + uVar21;
            uVar20 = (uVar20 << 0x11 | uVar20 >> 0x2f) ^ uVar27;
            uVar17 = (uVar21 << 0x15 | uVar21 >> 0x2b) ^ uVar25;
            uVar25 = (uVar25 ^ uVar26) + uVar20;
            uVar27 = ((uVar27 << 0x20 | uVar27 >> 0x20) ^ 0xff) + uVar17;
            uVar21 = (uVar20 << 0xd | uVar20 >> 0x33) ^ uVar25;
            uVar17 = (uVar17 << 0x10 | uVar17 >> 0x30) ^ uVar27;
            uVar27 = uVar27 + uVar21;
            uVar25 = (uVar25 << 0x20 | uVar25 >> 0x20) + uVar17;
            uVar21 = (uVar21 << 0x11 | uVar21 >> 0x2f) ^ uVar27;
            uVar17 = (uVar17 << 0x15 | uVar17 >> 0x2b) ^ uVar25;
            uVar25 = uVar25 + uVar21;
            uVar20 = (uVar27 << 0x20 | uVar27 >> 0x20) + uVar17;
            uVar21 = (uVar21 << 0xd | uVar21 >> 0x33) ^ uVar25;
            uVar17 = (uVar17 << 0x10 | uVar17 >> 0x30) ^ uVar20;
            uVar20 = uVar20 + uVar21;
            uVar27 = (uVar25 << 0x20 | uVar25 >> 0x20) + uVar17;
            uVar21 = (uVar21 << 0x11 | uVar21 >> 0x2f) ^ uVar20;
            uVar17 = (uVar17 << 0x15 | uVar17 >> 0x2b) ^ uVar27;
            uVar20 = (uVar20 << 0x20 | uVar20 >> 0x20) + uVar17;
            uVar21 = (uVar21 << 0xd | uVar21 >> 0x33) ^ uVar27 + uVar21;
            uVar17 = (uVar17 << 0x10 | uVar17 >> 0x30) ^ uVar20;
            uVar20 = uVar20 + uVar21;
            uVar20 = (uVar20 << 0x20 | uVar20 >> 0x20) ^ (uVar21 << 0x11 | uVar21 >> 0x2f) ^
                     (uVar17 << 0x15 | uVar17 >> 0x2b) ^ uVar20;
            lVar19 = *plVar18;
            bVar41 = (byte)(uVar20 >> 0x39);
            auVar42 = ZEXT216(CONCAT11(bVar41,bVar41));
            auVar42 = pshuflw(auVar42,auVar42,0);
            lVar29 = 0;
            while( true ) {
              uVar20 = uVar20 & plVar18[1];
              pcVar1 = (char *)(lVar19 + uVar20);
              cVar40 = auVar42[0];
              auVar53[0] = -(*pcVar1 == cVar40);
              cVar43 = auVar42[1];
              auVar53[1] = -(pcVar1[1] == cVar43);
              cVar44 = auVar42[2];
              auVar53[2] = -(pcVar1[2] == cVar44);
              cVar45 = auVar42[3];
              auVar53[3] = -(pcVar1[3] == cVar45);
              auVar53[4] = -(pcVar1[4] == cVar40);
              auVar53[5] = -(pcVar1[5] == cVar43);
              auVar53[6] = -(pcVar1[6] == cVar44);
              auVar53[7] = -(pcVar1[7] == cVar45);
              auVar53[8] = -(pcVar1[8] == cVar40);
              auVar53[9] = -(pcVar1[9] == cVar43);
              auVar53[10] = -(pcVar1[10] == cVar44);
              auVar53[0xb] = -(pcVar1[0xb] == cVar45);
              auVar53[0xc] = -(pcVar1[0xc] == cVar40);
              auVar53[0xd] = -(pcVar1[0xd] == cVar43);
              auVar53[0xe] = -(pcVar1[0xe] == cVar44);
              auVar53[0xf] = -(pcVar1[0xf] == cVar45);
              uVar28 = (ushort)(SUB161(auVar53 >> 7,0) & 1) |
                       (ushort)(SUB161(auVar53 >> 0xf,0) & 1) << 1 |
                       (ushort)(SUB161(auVar53 >> 0x17,0) & 1) << 2 |
                       (ushort)(SUB161(auVar53 >> 0x1f,0) & 1) << 3 |
                       (ushort)(SUB161(auVar53 >> 0x27,0) & 1) << 4 |
                       (ushort)(SUB161(auVar53 >> 0x2f,0) & 1) << 5 |
                       (ushort)(SUB161(auVar53 >> 0x37,0) & 1) << 6 |
                       (ushort)(SUB161(auVar53 >> 0x3f,0) & 1) << 7 |
                       (ushort)(SUB161(auVar53 >> 0x47,0) & 1) << 8 |
                       (ushort)(SUB161(auVar53 >> 0x4f,0) & 1) << 9 |
                       (ushort)(SUB161(auVar53 >> 0x57,0) & 1) << 10 |
                       (ushort)(SUB161(auVar53 >> 0x5f,0) & 1) << 0xb |
                       (ushort)(SUB161(auVar53 >> 0x67,0) & 1) << 0xc |
                       (ushort)(SUB161(auVar53 >> 0x6f,0) & 1) << 0xd |
                       (ushort)(SUB161(auVar53 >> 0x77,0) & 1) << 0xe |
                       (ushort)(auVar53[0xf] >> 7) << 0xf;
              uVar31 = (uint)uVar28;
              while (uVar28 != 0) {
                uVar8 = 0;
                for (uVar9 = uVar31; (uVar9 & 1) == 0; uVar9 = uVar9 >> 1 | 0x80000000) {
                  uVar8 = uVar8 + 1;
                }
                lVar16 = (uVar8 + uVar20 & plVar18[1]) * -0x50;
                if (iVar4 == *(int *)(lVar19 + -0x50 + lVar16)) {
                  plVar18 = *(longlong **)(lVar19 + -8 + lVar16);
                  LOCK();
                  lVar29 = *plVar18;
                  *plVar18 = *plVar18 + 1;
                  UNLOCK();
                  if (*plVar18 == 0 || SCARRY8(lVar29,1) != *plVar18 < 0) goto code_r0x0001408189bd;
                  lVar19 = lVar19 + lVar16;
                  uStack_168 = *(undefined8 *)(lVar19 + -0x20);
                  uStack_160 = *(undefined8 *)(lVar19 + -0x18);
                  uStack_158 = *(undefined8 *)(lVar19 + -0x10);
                  auStack_208 = *(undefined1 (*) [16])(lVar19 + -0x40);
                  auStack_1f8 = *(undefined1 (*) [16])(lVar19 + -0x30);
                  plStack_150 = plVar18;
                  FUN_141298110(auStack_e8,uStack_170,lStack_188,iVar4,&uStack_168,auStack_208);
                  goto code_r0x0001408185c0;
                }
                uVar28 = (ushort)(uVar31 - 1) & (ushort)uVar31;
                uVar31 = CONCAT22((short)(uVar31 - 1 >> 0x10),uVar28);
              }
              auVar47[0] = -(*pcVar1 == (char)iVar10);
              auVar47[1] = -(pcVar1[1] == cVar55);
              auVar47[2] = -(pcVar1[2] == cVar56);
              auVar47[3] = -(pcVar1[3] == cVar57);
              auVar47[4] = -(pcVar1[4] == (char)iVar11);
              auVar47[5] = -(pcVar1[5] == cVar58);
              auVar47[6] = -(pcVar1[6] == cVar59);
              auVar47[7] = -(pcVar1[7] == cVar60);
              auVar47[8] = -(pcVar1[8] == (char)iVar12);
              auVar47[9] = -(pcVar1[9] == cVar61);
              auVar47[10] = -(pcVar1[10] == cVar62);
              auVar47[0xb] = -(pcVar1[0xb] == cVar63);
              auVar47[0xc] = -(pcVar1[0xc] == (char)iVar13);
              auVar47[0xd] = -(pcVar1[0xd] == cVar64);
              auVar47[0xe] = -(pcVar1[0xe] == cVar65);
              auVar47[0xf] = -(pcVar1[0xf] == cVar66);
              if ((((((((((((((((SUB161(auVar47 >> 7,0) & 1) != 0 ||
                               (SUB161(auVar47 >> 0xf,0) & 1) != 0) ||
                              (SUB161(auVar47 >> 0x17,0) & 1) != 0) ||
                             (SUB161(auVar47 >> 0x1f,0) & 1) != 0) ||
                            (SUB161(auVar47 >> 0x27,0) & 1) != 0) ||
                           (SUB161(auVar47 >> 0x2f,0) & 1) != 0) ||
                          (SUB161(auVar47 >> 0x37,0) & 1) != 0) ||
                         (SUB161(auVar47 >> 0x3f,0) & 1) != 0) ||
                        (SUB161(auVar47 >> 0x47,0) & 1) != 0) ||
                       (SUB161(auVar47 >> 0x4f,0) & 1) != 0) || (SUB161(auVar47 >> 0x57,0) & 1) != 0
                      ) || (SUB161(auVar47 >> 0x5f,0) & 1) != 0) ||
                    (SUB161(auVar47 >> 0x67,0) & 1) != 0) || (SUB161(auVar47 >> 0x6f,0) & 1) != 0)
                  || (SUB161(auVar47 >> 0x77,0) & 1) != 0) || auVar47[0xf] < '\0') break;
              uVar20 = lVar29 + uVar20 + 0x10;
              lVar29 = lVar29 + 0x10;
            }
          }
code_r0x0001408185c0:
          lVar33 = lVar33 + -1;
        } while (lVar33 != 0);
        lVar33 = puStack_198[3];
        lVar29 = lStack_108;
        uVar32 = (uint)lStack_210;
        uVar6 = uStack_110;
        lVar19 = lStack_190;
        if (lVar33 != 0) {
          pauVar30 = (undefined1 (*) [16])*puStack_198;
          auVar42 = *pauVar30;
          pauVar23 = pauVar30 + 1;
          uVar32 = ~(uint)(ushort)((ushort)(SUB161(auVar42 >> 7,0) & 1) |
                                   (ushort)(SUB161(auVar42 >> 0xf,0) & 1) << 1 |
                                   (ushort)(SUB161(auVar42 >> 0x17,0) & 1) << 2 |
                                   (ushort)(SUB161(auVar42 >> 0x1f,0) & 1) << 3 |
                                   (ushort)(SUB161(auVar42 >> 0x27,0) & 1) << 4 |
                                   (ushort)(SUB161(auVar42 >> 0x2f,0) & 1) << 5 |
                                   (ushort)(SUB161(auVar42 >> 0x37,0) & 1) << 6 |
                                   (ushort)(SUB161(auVar42 >> 0x3f,0) & 1) << 7 |
                                   (ushort)(SUB161(auVar42 >> 0x47,0) & 1) << 8 |
                                   (ushort)(SUB161(auVar42 >> 0x4f,0) & 1) << 9 |
                                   (ushort)(SUB161(auVar42 >> 0x57,0) & 1) << 10 |
                                   (ushort)(SUB161(auVar42 >> 0x5f,0) & 1) << 0xb |
                                   (ushort)(SUB161(auVar42 >> 0x67,0) & 1) << 0xc |
                                   (ushort)(SUB161(auVar42 >> 0x6f,0) & 1) << 0xd |
                                   (ushort)(SUB161(auVar42 >> 0x77,0) & 1) << 0xe |
                                  (ushort)(byte)(auVar42[0xf] >> 7) << 0xf);
          do {
            if ((short)uVar32 == 0) {
              do {
                auVar42 = *pauVar23;
                uVar28 = (ushort)(SUB161(auVar42 >> 7,0) & 1) |
                         (ushort)(SUB161(auVar42 >> 0xf,0) & 1) << 1 |
                         (ushort)(SUB161(auVar42 >> 0x17,0) & 1) << 2 |
                         (ushort)(SUB161(auVar42 >> 0x1f,0) & 1) << 3 |
                         (ushort)(SUB161(auVar42 >> 0x27,0) & 1) << 4 |
                         (ushort)(SUB161(auVar42 >> 0x2f,0) & 1) << 5 |
                         (ushort)(SUB161(auVar42 >> 0x37,0) & 1) << 6 |
                         (ushort)(SUB161(auVar42 >> 0x3f,0) & 1) << 7 |
                         (ushort)(SUB161(auVar42 >> 0x47,0) & 1) << 8 |
                         (ushort)(SUB161(auVar42 >> 0x4f,0) & 1) << 9 |
                         (ushort)(SUB161(auVar42 >> 0x57,0) & 1) << 10 |
                         (ushort)(SUB161(auVar42 >> 0x5f,0) & 1) << 0xb |
                         (ushort)(SUB161(auVar42 >> 0x67,0) & 1) << 0xc |
                         (ushort)(SUB161(auVar42 >> 0x6f,0) & 1) << 0xd |
                         (ushort)(SUB161(auVar42 >> 0x77,0) & 1) << 0xe |
                         (ushort)(byte)(auVar42[0xf] >> 7) << 0xf;
                pauVar30 = pauVar30 + -0x50;
                pauVar23 = pauVar23 + 1;
              } while (uVar28 == 0xffff);
              uVar31 = ~(uint)uVar28;
              uVar32 = uVar31 & -(uint)uVar28 - 2;
            }
            else {
              uVar31 = uVar32;
              uVar32 = uVar32 - 1 & uVar32;
            }
            uVar8 = 0;
            for (; (uVar31 & 1) == 0; uVar31 = uVar31 >> 1 | 0x80000000) {
              uVar8 = uVar8 + 1;
            }
            iVar4 = *(int *)pauVar30[(ulonglong)uVar8 * -5 + -5];
            plVar18 = (longlong *)func_0x0001403c03c0(uStack_1a8);
            if (plVar18[3] != 0) {
              auStack_1e8 = *(undefined1 (*) [16])(plVar18 + 4);
              auVar51._0_8_ = auStack_1e8._0_8_;
              auVar51._8_4_ = auStack_1e8._0_4_;
              auVar51._12_4_ = auStack_1e8._4_4_;
              auStack_208 = auVar51 ^ auVar2;
              auVar52._0_8_ = auStack_1e8._8_8_;
              auVar52._8_4_ = auStack_1e8._8_4_;
              auVar52._12_4_ = auStack_1e8._12_4_;
              auStack_1f8 = auVar52 ^ auVar14;
              lStack_1d8 = 0;
              uStack_1d0 = 0;
              uStack_1c8 = 0;
              uStack_168 = CONCAT44(uStack_168._4_4_,iVar4);
              FUN_140106160(auStack_208,&uStack_168,4);
              uVar26 = lStack_1d8 << 0x38 | uStack_1d0;
              uVar17 = auStack_1f8._8_8_ ^ uVar26;
              uVar25 = auStack_208._0_8_ + auStack_1f8._0_8_;
              uVar20 = (auStack_1f8._0_8_ << 0xd | (ulonglong)auStack_1f8._0_8_ >> 0x33) ^ uVar25;
              uVar21 = (uVar17 << 0x10 | uVar17 >> 0x30) ^ auStack_208._8_8_ + uVar17;
              uVar27 = auStack_208._8_8_ + uVar17 + uVar20;
              uVar25 = (uVar25 << 0x20 | uVar25 >> 0x20) + uVar21;
              uVar20 = (uVar20 << 0x11 | uVar20 >> 0x2f) ^ uVar27;
              uVar17 = (uVar21 << 0x15 | uVar21 >> 0x2b) ^ uVar25;
              uVar25 = (uVar25 ^ uVar26) + uVar20;
              uVar27 = ((uVar27 << 0x20 | uVar27 >> 0x20) ^ 0xff) + uVar17;
              uVar21 = (uVar20 << 0xd | uVar20 >> 0x33) ^ uVar25;
              uVar17 = (uVar17 << 0x10 | uVar17 >> 0x30) ^ uVar27;
              uVar27 = uVar27 + uVar21;
              uVar25 = (uVar25 << 0x20 | uVar25 >> 0x20) + uVar17;
              uVar21 = (uVar21 << 0x11 | uVar21 >> 0x2f) ^ uVar27;
              uVar17 = (uVar17 << 0x15 | uVar17 >> 0x2b) ^ uVar25;
              uVar25 = uVar25 + uVar21;
              uVar20 = (uVar27 << 0x20 | uVar27 >> 0x20) + uVar17;
              uVar21 = (uVar21 << 0xd | uVar21 >> 0x33) ^ uVar25;
              uVar17 = (uVar17 << 0x10 | uVar17 >> 0x30) ^ uVar20;
              uVar20 = uVar20 + uVar21;
              uVar27 = (uVar25 << 0x20 | uVar25 >> 0x20) + uVar17;
              uVar21 = (uVar21 << 0x11 | uVar21 >> 0x2f) ^ uVar20;
              uVar17 = (uVar17 << 0x15 | uVar17 >> 0x2b) ^ uVar27;
              uVar20 = (uVar20 << 0x20 | uVar20 >> 0x20) + uVar17;
              uVar21 = (uVar21 << 0xd | uVar21 >> 0x33) ^ uVar27 + uVar21;
              uVar17 = (uVar17 << 0x10 | uVar17 >> 0x30) ^ uVar20;
              uVar20 = uVar20 + uVar21;
              uVar20 = (uVar20 << 0x20 | uVar20 >> 0x20) ^ (uVar21 << 0x11 | uVar21 >> 0x2f) ^
                       (uVar17 << 0x15 | uVar17 >> 0x2b) ^ uVar20;
              lVar19 = *plVar18;
              bVar41 = (byte)(uVar20 >> 0x39);
              auVar42 = ZEXT216(CONCAT11(bVar41,bVar41));
              auVar42 = pshuflw(auVar42,auVar42,0);
              lVar24 = 0;
              do {
                uVar20 = uVar20 & plVar18[1];
                pcVar1 = (char *)(lVar19 + uVar20);
                cVar40 = auVar42[0];
                auVar54[0] = -(*pcVar1 == cVar40);
                cVar43 = auVar42[1];
                auVar54[1] = -(pcVar1[1] == cVar43);
                cVar44 = auVar42[2];
                auVar54[2] = -(pcVar1[2] == cVar44);
                cVar45 = auVar42[3];
                auVar54[3] = -(pcVar1[3] == cVar45);
                auVar54[4] = -(pcVar1[4] == cVar40);
                auVar54[5] = -(pcVar1[5] == cVar43);
                auVar54[6] = -(pcVar1[6] == cVar44);
                auVar54[7] = -(pcVar1[7] == cVar45);
                auVar54[8] = -(pcVar1[8] == cVar40);
                auVar54[9] = -(pcVar1[9] == cVar43);
                auVar54[10] = -(pcVar1[10] == cVar44);
                auVar54[0xb] = -(pcVar1[0xb] == cVar45);
                auVar54[0xc] = -(pcVar1[0xc] == cVar40);
                auVar54[0xd] = -(pcVar1[0xd] == cVar43);
                auVar54[0xe] = -(pcVar1[0xe] == cVar44);
                auVar54[0xf] = -(pcVar1[0xf] == cVar45);
                uVar28 = (ushort)(SUB161(auVar54 >> 7,0) & 1) |
                         (ushort)(SUB161(auVar54 >> 0xf,0) & 1) << 1 |
                         (ushort)(SUB161(auVar54 >> 0x17,0) & 1) << 2 |
                         (ushort)(SUB161(auVar54 >> 0x1f,0) & 1) << 3 |
                         (ushort)(SUB161(auVar54 >> 0x27,0) & 1) << 4 |
                         (ushort)(SUB161(auVar54 >> 0x2f,0) & 1) << 5 |
                         (ushort)(SUB161(auVar54 >> 0x37,0) & 1) << 6 |
                         (ushort)(SUB161(auVar54 >> 0x3f,0) & 1) << 7 |
                         (ushort)(SUB161(auVar54 >> 0x47,0) & 1) << 8 |
                         (ushort)(SUB161(auVar54 >> 0x4f,0) & 1) << 9 |
                         (ushort)(SUB161(auVar54 >> 0x57,0) & 1) << 10 |
                         (ushort)(SUB161(auVar54 >> 0x5f,0) & 1) << 0xb |
                         (ushort)(SUB161(auVar54 >> 0x67,0) & 1) << 0xc |
                         (ushort)(SUB161(auVar54 >> 0x6f,0) & 1) << 0xd |
                         (ushort)(SUB161(auVar54 >> 0x77,0) & 1) << 0xe |
                         (ushort)(auVar54[0xf] >> 7) << 0xf;
                uVar31 = (uint)uVar28;
                while (uVar28 != 0) {
                  uVar8 = 0;
                  for (uVar9 = uVar31; (uVar9 & 1) == 0; uVar9 = uVar9 >> 1 | 0x80000000) {
                    uVar8 = uVar8 + 1;
                  }
                  lVar29 = (uVar8 + uVar20 & plVar18[1]) * -0x50;
                  if (iVar4 == *(int *)(lVar19 + -0x50 + lVar29)) {
                    plVar18 = *(longlong **)(lVar19 + -8 + lVar29);
                    LOCK();
                    lVar24 = *plVar18;
                    *plVar18 = *plVar18 + 1;
                    UNLOCK();
                    if (*plVar18 == 0 || SCARRY8(lVar24,1) != *plVar18 < 0) {
code_r0x0001408189bd:
                    /* WARNING: Does not return */
                      pcVar7 = (code *)invalidInstructionException();
                      (*pcVar7)();
                    }
                    lVar19 = lVar19 + lVar29;
                    uStack_168 = *(undefined8 *)(lVar19 + -0x20);
                    uStack_160 = *(undefined8 *)(lVar19 + -0x18);
                    uStack_158 = *(undefined8 *)(lVar19 + -0x10);
                    auStack_208 = *(undefined1 (*) [16])(lVar19 + -0x40);
                    auStack_1f8 = *(undefined1 (*) [16])(lVar19 + -0x30);
                    plStack_150 = plVar18;
                    FUN_141298110(auStack_e8,uStack_170,lStack_100,iVar4,&uStack_168,auStack_208);
                    goto code_r0x000140818900;
                  }
                  uVar28 = (ushort)(uVar31 - 1) & (ushort)uVar31;
                  uVar31 = CONCAT22((short)(uVar31 - 1 >> 0x10),uVar28);
                }
                auVar50[0] = -(*pcVar1 == (char)iVar10);
                auVar50[1] = -(pcVar1[1] == cVar55);
                auVar50[2] = -(pcVar1[2] == cVar56);
                auVar50[3] = -(pcVar1[3] == cVar57);
                auVar50[4] = -(pcVar1[4] == (char)iVar11);
                auVar50[5] = -(pcVar1[5] == cVar58);
                auVar50[6] = -(pcVar1[6] == cVar59);
                auVar50[7] = -(pcVar1[7] == cVar60);
                auVar50[8] = -(pcVar1[8] == (char)iVar12);
                auVar50[9] = -(pcVar1[9] == cVar61);
                auVar50[10] = -(pcVar1[10] == cVar62);
                auVar50[0xb] = -(pcVar1[0xb] == cVar63);
                auVar50[0xc] = -(pcVar1[0xc] == (char)iVar13);
                auVar50[0xd] = -(pcVar1[0xd] == cVar64);
                auVar50[0xe] = -(pcVar1[0xe] == cVar65);
                auVar50[0xf] = -(pcVar1[0xf] == cVar66);
                if ((((((((((((((((SUB161(auVar50 >> 7,0) & 1) != 0 ||
                                 (SUB161(auVar50 >> 0xf,0) & 1) != 0) ||
                                (SUB161(auVar50 >> 0x17,0) & 1) != 0) ||
                               (SUB161(auVar50 >> 0x1f,0) & 1) != 0) ||
                              (SUB161(auVar50 >> 0x27,0) & 1) != 0) ||
                             (SUB161(auVar50 >> 0x2f,0) & 1) != 0) ||
                            (SUB161(auVar50 >> 0x37,0) & 1) != 0) ||
                           (SUB161(auVar50 >> 0x3f,0) & 1) != 0) ||
                          (SUB161(auVar50 >> 0x47,0) & 1) != 0) ||
                         (SUB161(auVar50 >> 0x4f,0) & 1) != 0) ||
                        (SUB161(auVar50 >> 0x57,0) & 1) != 0) ||
                       (SUB161(auVar50 >> 0x5f,0) & 1) != 0) || (SUB161(auVar50 >> 0x67,0) & 1) != 0
                      ) || (SUB161(auVar50 >> 0x6f,0) & 1) != 0) ||
                    (SUB161(auVar50 >> 0x77,0) & 1) != 0) || auVar50[0xf] < '\0') break;
                uVar20 = lVar24 + uVar20 + 0x10;
                lVar24 = lVar24 + 0x10;
              } while( true );
            }
code_r0x000140818900:
            lVar33 = lVar33 + -1;
          } while (lVar33 != 0);
          lVar29 = lStack_108;
          uVar32 = (uint)lStack_210;
          uVar6 = uStack_110;
          lVar19 = lStack_190;
          lVar24 = lStack_1b8;
        }
      }
    }
  }
  goto code_r0x000140817de0;
}

