
/* WARNING: Removing unreachable block (ram,0x000141801c0d) */
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void FUN_141801980(undefined8 *param_1)

{
  byte *pbVar1;
  byte *pbVar2;
  byte *pbVar3;
  undefined8 *puVar4;
  float *pfVar5;
  float fVar6;
  undefined4 uVar7;
  float fVar8;
  byte bVar9;
  longlong *plVar10;
  char *pcVar11;
  char *pcVar12;
  undefined1 auVar13 [16];
  undefined1 auVar14 [14];
  undefined1 auVar15 [14];
  undefined1 auVar16 [14];
  undefined1 auVar17 [14];
  float fVar18;
  float fVar19;
  undefined8 uVar20;
  undefined8 uVar21;
  int iVar22;
  undefined2 uVar23;
  longlong *plVar24;
  char cVar25;
  char cVar26;
  undefined1 uVar27;
  longlong lVar28;
  uint uVar29;
  longlong lVar30;
  longlong lVar31;
  undefined *puVar32;
  uint uVar33;
  longlong lVar34;
  ulonglong uVar35;
  byte *pbVar36;
  int iVar37;
  char *pcVar38;
  int iVar39;
  int iVar40;
  ulonglong uVar41;
  uint uVar42;
  longlong lVar43;
  bool bVar44;
  byte bVar45;
  float fVar46;
  undefined8 extraout_XMM0_Qa;
  undefined8 extraout_XMM0_Qa_00;
  undefined8 extraout_XMM0_Qa_01;
  undefined8 extraout_XMM0_Qa_02;
  undefined8 extraout_XMM0_Qa_03;
  undefined8 extraout_XMM0_Qa_04;
  undefined8 extraout_XMM0_Qa_05;
  undefined8 extraout_XMM0_Qb;
  undefined8 extraout_XMM0_Qb_00;
  undefined8 extraout_XMM0_Qb_01;
  undefined8 extraout_XMM0_Qb_02;
  undefined8 extraout_XMM0_Qb_03;
  undefined8 extraout_XMM0_Qb_04;
  undefined8 extraout_XMM0_Qb_05;
  undefined1 auVar47 [16];
  undefined1 auVar48 [16];
  char *pcStack_288;
  ulonglong uStack_280;
  uint uStack_270;
  uint uStack_26c;
  undefined4 uStack_268;
  uint uStack_264;
  short sStack_260;
  short sStack_25e;
  longlong lStack_258;
  int iStack_250;
  int iStack_24c;
  undefined8 uStack_248;
  undefined *puStack_240;
  ulonglong uStack_238;
  undefined *puStack_230;
  ulonglong uStack_228;
  ulonglong uStack_220;
  undefined8 uStack_218;
  undefined1 uStack_210;
  undefined4 uStack_20c;
  char cStack_208;
  undefined8 uStack_200;
  undefined8 uStack_1f0;
  ulonglong uStack_1e0;
  ulonglong uStack_1d8;
  undefined8 uStack_1d0;
  undefined1 uStack_1c8;
  undefined4 uStack_1c4;
  undefined1 uStack_1c0;
  char *pcStack_1b8;
  ulonglong uStack_1b0;
  char *pcStack_1a8;
  char *pcStack_1a0;
  undefined *puStack_198;
  longlong lStack_190;
  longlong lStack_188;
  ulonglong uStack_180;
  undefined4 *puStack_178;
  char *pcStack_170;
  char *pcStack_168;
  ulonglong uStack_160;
  char *pcStack_158;
  ulonglong uStack_150;
  char *pcStack_148;
  char *pcStack_140;
  float *pfStack_138;
  ulonglong uStack_130;
  longlong lStack_128;
  longlong *plStack_120;
  longlong lStack_118;
  char *pcStack_110;
  longlong lStack_108;
  ulonglong uStack_100;
  longlong lStack_f8;
  float fStack_f0;
  float fStack_ec;
  char cStack_e4;
  undefined1 auStack_e0 [120];
  
  FUN_140e170c0(auStack_e0);
  pfStack_138 = (float *)*param_1;
  lStack_188 = param_1[1];
  pcStack_140 = (char *)param_1[2];
  plVar10 = (longlong *)param_1[3];
  puStack_178 = (undefined4 *)param_1[5];
  lVar43 = param_1[6];
  plStack_120 = (longlong *)param_1[4];
  FUN_140e14f50(&lStack_118,auStack_e0);
  uVar21 = _UNK_142cecb68;
  uVar20 = _DAT_142cecb60;
  fVar19 = _DAT_1429a49bc;
  lStack_128 = lVar43;
  lVar34 = lStack_108;
  uStack_180 = uStack_100;
  fVar18 = fStack_f0;
  fVar8 = fStack_ec;
joined_r0x000141801a3f:
  if (cStack_e4 == '\x02') {
    return;
  }
  fVar46 = *pfStack_138;
  fVar6 = fVar8;
  if (fVar8 <= fVar46) {
    fVar6 = fVar46;
  }
  *pfStack_138 = (float)(~-(uint)NAN(fVar46) & (uint)fVar6 | -(uint)NAN(fVar46) & (uint)fVar8);
  lStack_108 = lVar34;
  uStack_100 = uStack_180;
  fStack_f0 = fVar18;
  fStack_ec = fVar8;
  if ((*pcStack_140 != '\0') && (lStack_f8 == *(longlong *)(lStack_188 + 0x180))) {
    lStack_190 = lVar34 + uStack_180 * 0x48;
    auVar48 = ZEXT816(0);
    if (uStack_180 != 0) {
      pcVar11 = *(char **)(lStack_188 + 0x188);
      uVar41 = 0;
      uVar35 = 0;
      do {
        pcVar12 = *(char **)(lVar34 + 0x10 + uVar41);
        if (pcVar12 == pcVar11) goto LAB_1418025b5;
        if ((pcVar12 < pcVar11) && (pcVar38 = *(char **)(lVar34 + 0x18 + uVar41), pcVar11 < pcVar38)
           ) {
          if (pcVar12 != (char *)0x0) {
            if (pcVar12 < pcStack_110) {
              if (pcVar12[lStack_118] < -0x40) goto LAB_141802926;
            }
            else if (pcVar12 != pcStack_110) goto LAB_141802926;
          }
          if (pcVar38 < pcStack_110) {
            if (pcVar38[lStack_118] < -0x40) {
LAB_141802926:
              FUN_1428d9940(lStack_118,pcStack_110,pcVar12,pcVar38,&UNK_142cecdd8);
LAB_14180293a:
              FUN_1428d9940(pcStack_288,uStack_280,0,uVar41,&UNK_142cecc88);
              goto LAB_1418028dc;
            }
          }
          else if (pcVar38 != pcStack_110) goto LAB_141802926;
          uStack_220 = (longlong)pcVar38 - (longlong)pcVar12;
          pcStack_1b8 = pcVar12 + lStack_118;
          uStack_248 = 0;
          uStack_238 = 0;
          uStack_228 = 0;
          uStack_218 = 0;
          uStack_210 = 2;
          uStack_20c = 0xf020001;
          cStack_208 = '\x0f';
          uStack_200 = 0;
          uStack_1f0 = 0;
          uStack_1d0 = 0;
          uStack_1c8 = 2;
          uStack_1c4 = 0xf020001;
          uStack_1c0 = 0xf;
          iVar40 = 0;
          iVar39 = 0;
          lStack_258 = lVar34;
          uStack_1e0 = uStack_220;
          uStack_1d8 = uStack_220;
          uStack_1b0 = uStack_220;
          pcStack_1a8 = pcStack_1b8;
          uStack_160 = uVar41;
          pcStack_158 = pcVar11;
          uStack_150 = uVar35;
          goto LAB_141801dae;
        }
        uVar41 = uVar41 + 0x48;
        uVar35 = uVar35 + 1;
      } while (uStack_180 * 0x48 != uVar41);
      if (lStack_190 != 0x48) {
        if (*(char **)(lStack_190 + -0x30) != pcVar11) goto LAB_14180269a;
        goto LAB_141801c40;
      }
      uVar35 = 0;
      if (uStack_180 == 0) goto LAB_141801c40;
      goto LAB_1418025c3;
    }
    goto LAB_141801c40;
  }
  goto LAB_14180269a;
LAB_141801dae:
  uStack_280 = uStack_1b0;
  pcVar11 = pcStack_1b8;
  if (uStack_228 == uStack_220) goto LAB_1418028c9;
  if (uStack_228 != 0) {
    if (uStack_228 < uStack_1b0) {
      if (pcStack_1b8[uStack_228] < -0x40) goto LAB_1418028f1;
    }
    else if (uStack_228 != uStack_1b0) goto LAB_1418028f1;
  }
  iStack_250 = iVar39;
  iStack_24c = iVar40;
  if (uStack_228 == uStack_1b0) goto LAB_1418028bd;
  pbVar3 = (byte *)(pcStack_1b8 + uStack_228);
  bVar45 = *pbVar3;
  uVar42 = (uint)bVar45;
  uStack_130 = uStack_228;
  if ((char)bVar45 < '\0') {
    if (bVar45 < 0xe0) {
      pbVar36 = pbVar3 + 2;
      uVar42 = (uVar42 & 0x1f) << 6 | pbVar3[1] & 0x3f;
    }
    else {
      uVar29 = pbVar3[2] & 0x3f | (pbVar3[1] & 0x3f) << 6;
      if (bVar45 < 0xf0) {
        pbVar36 = pbVar3 + 3;
        uVar42 = uVar29 | (uVar42 & 0x1f) << 0xc;
      }
      else {
        uVar42 = pbVar3[3] & 0x3f | uVar29 << 6 | (uVar42 & 7) << 0x12;
        if (uVar42 == 0x110000) goto LAB_1418028bd;
        pbVar36 = pbVar3 + 4;
      }
    }
  }
  else {
    pbVar36 = pbVar3 + 1;
  }
  pbVar3 = (byte *)(pcStack_1b8 + uStack_1b0);
  pcStack_148 = pcStack_1b8 + -1;
  pcVar12 = pcStack_1b8 + -2;
  pcStack_168 = pcStack_1b8 + -3;
  pcStack_170 = pcStack_1b8 + -4;
LAB_141801eea:
  cVar25 = cStack_208;
  lVar28 = 1;
  if ((0x7f < uVar42) && (lVar28 = 2, 0x7ff < uVar42)) {
    lVar28 = 4 - (ulonglong)(uVar42 < 0x10000);
  }
  uStack_228 = uStack_228 + lVar28;
  cStack_208 = '\x0f';
  uStack_20c = CONCAT13(cVar25,(int3)uStack_20c) & 0xff00ffff;
  if (cVar25 == '\x0f') {
    cVar25 = FUN_140e8d440(&uStack_248,uVar42);
    auVar48._8_8_ = extraout_XMM0_Qb;
    auVar48._0_8_ = extraout_XMM0_Qa;
    uStack_20c = CONCAT13(cVar25,(undefined3)uStack_20c);
  }
  puVar32 = puStack_230 + 1;
  if (cVar25 != '\n') {
    uStack_238 = 1;
    puVar32 = (undefined *)0x0;
  }
  pcStack_288 = pcVar11;
  puStack_230 = puVar32;
  if (pbVar36 == pbVar3) {
    if (uStack_228 != uStack_220) goto LAB_141802914;
    goto LAB_1418024f6;
  }
  bVar45 = *pbVar36;
  uVar42 = (uint)bVar45;
  if ((char)bVar45 < '\0') {
    pbVar1 = pbVar36 + 1;
    if (bVar45 < 0xe0) {
      pbVar36 = pbVar36 + 2;
      uVar42 = (uVar42 & 0x1f) << 6 | *pbVar1 & 0x3f;
    }
    else {
      uVar29 = pbVar36[2] & 0x3f | (*pbVar1 & 0x3f) << 6;
      if (bVar45 < 0xf0) {
        pbVar36 = pbVar36 + 3;
        uVar42 = uVar29 | (uVar42 & 0x1f) << 0xc;
      }
      else {
        pbVar1 = pbVar36 + 3;
        pbVar36 = pbVar36 + 4;
        uVar42 = *pbVar1 & 0x3f | uVar29 << 6 | (uVar42 & 7) << 0x12;
      }
    }
  }
  else {
    pbVar36 = pbVar36 + 1;
  }
  cStack_208 = FUN_140e8d440(&uStack_248,uVar42);
  uVar29 = uStack_20c;
  uVar41 = uStack_228;
  auVar48._8_8_ = extraout_XMM0_Qb_00;
  auVar48._0_8_ = extraout_XMM0_Qa_00;
  cVar25 = uStack_20c._2_1_;
  uStack_20c._0_2_ = CONCAT11(1,(byte)uStack_20c);
  uVar33 = uStack_20c;
  if (cVar25 == '\x01') goto LAB_141801ed8;
  if (cVar25 == '\x02') goto LAB_1418024fe;
  if (uStack_280 < uStack_228) {
    uStack_270 = 3;
    puVar32 = puStack_198;
    goto LAB_14180287e;
  }
  if ((uStack_248 & 1) != 0) goto LAB_14180287c;
  uStack_20c._3_1_ = SUB41(uVar29,3);
  uStack_20c = uVar33;
  if (uStack_228 == 0) {
    if (cStack_208 == '\x04') {
      uVar27 = 4;
      goto LAB_141802861;
    }
    if (cStack_208 == '\n') goto LAB_14180285b;
    uVar29 = (uint)uStack_20c._3_1_;
    bVar44 = uStack_20c._3_1_ == 0xf;
    if (bVar44) goto LAB_141802868;
  }
  else {
    uVar29 = (uint)uStack_20c._3_1_;
    bVar44 = uStack_20c._3_1_ == 0xf;
    if (bVar44) {
      if ((uStack_228 < uStack_280) && (pcVar11[uStack_228] < -0x40)) goto LAB_1418028dc;
      if (pcVar11[uStack_228 - 1] < '\0') {
        bVar45 = pcVar11[uStack_228 - 2];
        if ((char)bVar45 < -0x40) {
          bVar9 = pcVar11[uStack_228 - 3];
          if ((char)bVar9 < -0x40) {
            uVar29 = bVar9 & 0x3f | ((byte)pcVar11[uStack_228 - 4] & 7) << 6;
          }
          else {
            uVar29 = bVar9 & 0xf;
          }
          uVar29 = bVar45 & 0x3f | uVar29 << 6;
        }
        else {
          uVar29 = bVar45 & 0x1f;
        }
        if (((uint)(pcVar11[uStack_228 - 1] & 0x3f) | uVar29 << 6) == 0x110000) goto LAB_141802908;
      }
      uVar29 = FUN_140e8d440(&uStack_248);
      auVar48._8_8_ = extraout_XMM0_Qb_01;
      auVar48._0_8_ = extraout_XMM0_Qa_01;
      uStack_20c = CONCAT13((char)uVar29,(undefined3)uStack_20c);
      if (cStack_208 == '\x0f') goto LAB_1418028b1;
    }
  }
  bVar45 = (byte)uVar29;
  uVar23 = (ushort)uStack_20c;
  if (bVar45 == 1) {
    cVar25 = cStack_208 != '\x06';
    if (!(bool)cVar25) goto LAB_141801ed0;
code_r0x00014180219d:
    if (cVar25 == '\x02') goto LAB_1418021a9;
    goto LAB_1418024f6;
  }
  uVar33 = uVar29 & 0xff;
  if ((uVar33 == 2) || (uVar33 == 6)) goto LAB_1418024f6;
  switch(cStack_208) {
  case '\x01':
  case '\x02':
  case '\x06':
    break;
  case '\x03':
  case '\x0e':
    goto LAB_141801ed0;
  default:
    if (bVar45 == 9) goto LAB_1418021a9;
    if (uVar33 != 10) {
      if ((uVar33 != 0xe) || (cStack_208 != '\x04')) break;
      pcVar38 = pcVar11 + uVar41;
      if ((uVar41 == 0) || (uStack_280 <= uVar41)) {
        if (uVar41 != 0) goto LAB_14180229a;
      }
      else {
        if (*pcVar38 < -0x40) goto LAB_14180293a;
LAB_14180229a:
        uVar29 = (uint)pcStack_148[uVar41];
        pcStack_1a0 = pcStack_148;
        if ((int)uVar29 < 0) {
          bVar45 = pcVar12[uVar41];
          if ((char)bVar45 < -0x40) {
            bVar9 = pcStack_168[uVar41];
            if ((char)bVar9 < -0x40) {
              uVar29 = bVar9 & 0x3f | ((byte)pcStack_170[uVar41] & 7) << 6;
              pcStack_1a0 = pcStack_170;
            }
            else {
              uVar29 = bVar9 & 0xf;
              pcStack_1a0 = pcStack_168;
            }
            uVar29 = bVar45 & 0x3f | uVar29 << 6;
          }
          else {
            uVar29 = bVar45 & 0x1f;
            pcStack_1a0 = pcVar12;
          }
          uVar29 = (uint)(pcStack_148[uVar41] & 0x3f) | uVar29 << 6;
        }
        pcStack_1a0 = pcStack_1a0 + uVar41;
        cVar26 = FUN_140e8d440(&uStack_248,uVar29);
        auVar48._8_8_ = extraout_XMM0_Qb_04;
        auVar48._0_8_ = extraout_XMM0_Qa_04;
        cVar25 = '\x02';
        pcVar38 = pcStack_1a0;
        if (cVar26 != '\x0e') goto LAB_1418024a7;
      }
      do {
        cVar25 = '\x02';
        if (pcVar38 == pcVar11) goto LAB_1418024a7;
        pbVar1 = (byte *)(pcVar38 + -1);
        uVar29 = (uint)(char)*pbVar1;
        if ((int)uVar29 < 0) {
          bVar45 = pcVar38[-2];
          if ((char)bVar45 < -0x40) {
            bVar9 = pcVar38[-3];
            if ((char)bVar9 < -0x40) {
              pbVar2 = (byte *)(pcVar38 + -4);
              pcVar38 = pcVar38 + -4;
              uVar29 = bVar9 & 0x3f | (*pbVar2 & 7) << 6;
            }
            else {
              pcVar38 = pcVar38 + -3;
              uVar29 = bVar9 & 0xf;
            }
            uVar29 = bVar45 & 0x3f | uVar29 << 6;
          }
          else {
            pcVar38 = pcVar38 + -2;
            uVar29 = bVar45 & 0x1f;
          }
          uVar29 = (uint)(*pbVar1 & 0x3f) | uVar29 << 6;
        }
        else {
          pcVar38 = pcVar38 + -1;
        }
        cVar26 = FUN_140e8d440(&uStack_248,uVar29);
        auVar48._8_8_ = extraout_XMM0_Qb_05;
        auVar48._0_8_ = extraout_XMM0_Qa_05;
      } while (cVar26 == '\x03');
      cVar25 = '\x02';
      if (cVar26 == '\x04') {
        cVar25 = '\x01';
      }
LAB_1418024a7:
      uStack_20c._0_3_ = CONCAT12(cVar25,(ushort)uStack_20c);
      bVar45 = cVar25 == '\x02';
      puStack_198 = (undefined *)((ulonglong)puStack_198 & 0xffffffffffffff00 | (ulonglong)bVar45);
      goto LAB_141801eda;
    }
    if (cStack_208 != '\n') break;
    if ((uStack_238 & 1) == 0) {
      if ((uVar41 == 0) || (uStack_280 <= uVar41)) {
        if (uVar41 != 0) goto LAB_14180230b;
        puVar32 = (undefined *)0x0;
      }
      else {
        if (pcVar11[uVar41] < -0x40) goto LAB_1418028dc;
LAB_14180230b:
        pcVar38 = pcVar11 + uVar41;
        puVar32 = (undefined *)0x0;
        do {
          pbVar1 = (byte *)(pcVar38 + -1);
          if ((char)*pbVar1 < '\0') {
            bVar45 = pcVar38[-2];
            if ((char)bVar45 < -0x40) {
              bVar9 = pcVar38[-3];
              if ((char)bVar9 < -0x40) {
                pbVar2 = (byte *)(pcVar38 + -4);
                pcVar38 = pcVar38 + -4;
                uVar29 = bVar9 & 0x3f | (*pbVar2 & 7) << 6;
              }
              else {
                pcVar38 = pcVar38 + -3;
                uVar29 = bVar9 & 0xf;
              }
              uVar29 = bVar45 & 0x3f | uVar29 << 6;
            }
            else {
              pcVar38 = pcVar38 + -2;
              uVar29 = bVar45 & 0x1f;
            }
            cVar25 = FUN_140e8d440(&uStack_248,(uint)(*pbVar1 & 0x3f) | uVar29 << 6);
            auVar48._8_8_ = extraout_XMM0_Qb_03;
            auVar48._0_8_ = extraout_XMM0_Qa_03;
          }
          else {
            pcVar38 = pcVar38 + -1;
            cVar25 = FUN_140e8d440(&uStack_248);
            auVar48._8_8_ = extraout_XMM0_Qb_02;
            auVar48._0_8_ = extraout_XMM0_Qa_02;
          }
        } while ((cVar25 == '\n') && (puVar32 = puVar32 + 1, pcVar38 != pcVar11));
      }
      uStack_238 = 1;
      cVar25 = '\x02' - ((byte)puVar32 & 1);
      lVar34 = lStack_258;
      puStack_230 = puVar32;
      goto LAB_1418024a7;
    }
    bVar45 = (byte)puStack_230 & 1;
    uStack_20c._0_3_ = CONCAT12('\x02' - bVar45,(ushort)uStack_20c);
    bVar45 = bVar45 ^ 1;
LAB_141801eda:
    uStack_20c._0_2_ = (ushort)(byte)uStack_20c;
    if (bVar45 != 0) goto LAB_141802506;
    goto LAB_141801eea;
  case '\x05':
  case '\a':
  case '\b':
    if ((bVar45 != 5) && (cVar25 = (bVar45 == 9) + '\x01', cVar25 != '\0'))
    goto code_r0x00014180219d;
LAB_141801ed0:
    uStack_20c._0_3_ = CONCAT12(1,(ushort)uStack_20c);
LAB_141801ed8:
    bVar45 = 0;
    goto LAB_141801eda;
  case '\v':
LAB_1418021a9:
    bVar45 = (byte)uStack_20c ^ 1;
    uStack_20c._0_3_ = CONCAT12('\x02' - (byte)uStack_20c,uVar23);
    goto LAB_141801eda;
  case '\f':
    if (bVar45 < 0xe) {
      if ((0x3180U >> (uVar29 & 0x1f) & 1) != 0) goto LAB_141801ed0;
      if (uVar33 == 9) goto LAB_1418021a9;
    }
    break;
  case '\r':
    switch(uVar33) {
    case 5:
    case 7:
    case 0xd:
      goto LAB_141801ed0;
    case 9:
      goto LAB_1418021a9;
    }
  }
LAB_1418024f6:
  uStack_20c._0_3_ = CONCAT12(2,(ushort)uStack_20c);
LAB_1418024fe:
  uStack_20c._0_2_ = (ushort)(byte)uStack_20c;
LAB_141802506:
  if (uStack_228 < uStack_130) goto LAB_141802847;
  if (uStack_130 != 0) {
    if (uStack_130 < uStack_1b0) {
      if (pcStack_1b8[uStack_130] < -0x40) goto LAB_141802847;
    }
    else if (uStack_130 != uStack_1b0) goto LAB_141802847;
  }
  if (uStack_228 != 0) {
    if (uStack_228 < uStack_1b0) {
      if (pcStack_1b8[uStack_228] < -0x40) {
LAB_141802847:
        FUN_1428d9940(pcStack_1b8,uStack_1b0,uStack_130,uStack_228,&UNK_142cecc10);
        uVar33 = uStack_20c;
LAB_14180285b:
        uStack_20c = uVar33;
        uVar27 = 3;
LAB_141802861:
        uStack_20c._0_3_ = CONCAT12(uVar27,(ushort)uStack_20c);
LAB_141802868:
        uStack_248 = 1;
        puStack_240 = (undefined *)0x0;
LAB_14180287c:
        uStack_270 = 0;
        puVar32 = puStack_240;
LAB_14180287e:
        do {
          uStack_26c = 0;
          uStack_268 = SUB84(puVar32,0);
          uStack_264 = (uint)((ulonglong)puVar32 >> 0x20);
          FUN_1428d9760(&UNK_142ceccf8,0x2b,&uStack_270,&UNK_142ceccd8,&UNK_142cecbe0);
LAB_1418028b1:
          FUN_1428d9310(&UNK_142cecc58);
LAB_1418028bd:
          FUN_1428d9310(&UNK_142ceccb8);
LAB_1418028c9:
          FUN_1428d9310(&UNK_142cecbf8);
LAB_1418028dc:
          FUN_1428d9940(pcStack_288);
LAB_1418028f1:
          FUN_1428d9940();
LAB_141802908:
          puVar32 = &UNK_142cecc40;
          FUN_1428d9310();
LAB_141802914:
          uStack_20c._0_2_ = CONCAT11(1,(byte)uStack_20c);
          uStack_270 = 2;
        } while( true );
      }
    }
    else if (uStack_228 != uStack_1b0) goto LAB_141802847;
  }
  iVar39 = iStack_250 +
           (uint)(pcStack_1b8 +
                  *(longlong *)(lVar34 + 0x10 + uStack_160) + (uStack_130 - (longlong)pcStack_1a8) <
                 pcStack_158);
  iVar40 = iStack_24c + 1;
  if (uStack_228 == uStack_1e0) goto LAB_141802598;
  goto LAB_141801dae;
LAB_141802598:
  auVar48._0_4_ = ((float)iVar39 * *(float *)(lVar34 + 0x34 + uStack_160)) / (float)iVar40;
  uVar35 = uStack_150;
LAB_1418025b5:
  if (uVar35 < uStack_180) {
LAB_1418025c3:
    lVar28 = lVar34 + uVar35 * 0x48;
    if ((*(byte *)(lVar28 + 0x46) & 1) == 0) {
      fVar46 = auVar48._0_4_ + *(float *)(lVar28 + 0x2c);
      goto LAB_1418025e4;
    }
    fVar46 = (*(float *)(lVar28 + 0x2c) + *(float *)(lVar28 + 0x34)) - auVar48._0_4_;
    iVar39 = (int)fVar46;
    if (fVar19 < fVar46) {
      iVar39 = 0x7fffffff;
    }
  }
  else {
LAB_141801c40:
    iVar39 = 0;
    uVar41 = uStack_180;
    if (lStack_190 == 0x48 || uStack_180 == 0) goto LAB_14180261b;
    fVar46 = *(float *)(lStack_190 + -0x1c);
    if ((*(byte *)(lStack_190 + -2) & 1) == 0) {
      fVar46 = fVar46 + *(float *)(lStack_190 + -0x14);
    }
LAB_1418025e4:
    iVar39 = (int)fVar46;
    if (fVar19 < fVar46) {
      iVar39 = 0x7fffffff;
    }
  }
  uVar41 = uStack_180;
  if (NAN(fVar46)) {
    iVar39 = 0;
  }
LAB_14180261b:
  uVar7 = *puStack_178;
  lVar28 = plVar10[2];
  if (lVar28 == *plVar10) {
    FUN_14082b5f0(plVar10,&UNK_142cecd90);
  }
  lVar31 = plVar10[1];
  lVar30 = lVar28 * 0x30;
  *(float *)(lVar31 + lVar30) = (float)iVar39;
  *(float *)(lVar31 + 4 + lVar30) = fVar8;
  *(undefined4 *)(lVar31 + 8 + lVar30) = 0x40800000;
  *(undefined4 *)(lVar31 + 0xc + lVar30) = uVar7;
  *(undefined8 *)(lVar31 + 0x10 + lVar30) = 0;
  puVar4 = (undefined8 *)(lVar31 + 0x18 + lVar30);
  *puVar4 = uVar20;
  puVar4[1] = uVar21;
  *(undefined8 *)(lVar31 + 0x28 + lVar30) = 0;
  plVar10[2] = lVar28 + 1;
  uStack_180 = uVar41;
LAB_14180269a:
  if (uStack_180 != 0) {
    lVar28 = uStack_180 * 0x48;
    do {
      FUN_140e1bc90(&uStack_248,lVar34);
      FUN_140b8fea0(&uStack_270,lVar43,&uStack_248);
      uVar29 = uStack_264;
      uVar7 = uStack_268;
      uVar42 = uStack_26c;
      if (((uStack_270 & 1) != 0) && (uStack_264 != 0)) {
        fVar8 = *(float *)(lVar43 + 0x4d0);
        fVar46 = fVar18 * fVar8;
        iVar39 = 0x7fffffff;
        if (fVar46 <= fVar19) {
          iVar39 = (int)fVar46;
        }
        iVar40 = (int)sStack_260;
        if (NAN(fVar46)) {
          iVar39 = 0;
        }
        iVar37 = sStack_25e + uStack_248._4_4_;
        iVar22 = (int)uStack_248;
        lVar43 = plVar10[2];
        lStack_258 = lVar34;
        if (lVar43 == *plVar10) {
          FUN_14082b5f0(plVar10,&UNK_142cecda8);
        }
        plVar24 = plStack_120;
        auVar47._0_4_ = (float)(iVar40 + iVar22);
        auVar47._4_4_ = (float)(iVar39 + iVar37);
        uVar41 = (ulonglong)(uVar29 >> 0x10) << 0x30;
        auVar14._8_4_ = 0;
        auVar14._0_8_ = uVar41;
        auVar14._12_2_ = (short)(uVar29 >> 0x10);
        auVar15._8_2_ = (short)uVar29;
        auVar15._0_8_ = uVar41;
        auVar15._10_4_ = auVar14._10_4_;
        auVar17._6_8_ = 0;
        auVar17._0_6_ = auVar15._8_6_;
        auVar16._4_2_ = (short)(uVar42 >> 0x10);
        auVar16._0_4_ = uVar42;
        auVar16._6_8_ = SUB148(auVar17 << 0x40,6);
        fVar46 = (float)(auVar14._10_4_ >> 0x10);
        auVar47._8_4_ = (float)auVar15._8_4_;
        auVar47._12_4_ = fVar46;
        auVar13._4_4_ = fVar8;
        auVar13._0_4_ = fVar8;
        auVar13._8_4_ = fVar8;
        auVar13._12_4_ = fVar8;
        auVar48 = divps(auVar47,auVar13);
        lVar34 = plVar10[1];
        lVar31 = lVar43 * 0x30;
        *(undefined1 (*) [16])(lVar34 + lVar31) = auVar48;
        pfVar5 = (float *)(lVar34 + 0x10 + lVar31);
        *pfVar5 = (float)(uVar42 & 0xffff);
        pfVar5[1] = (float)auVar16._4_4_;
        pfVar5[2] = (float)auVar15._8_4_;
        pfVar5[3] = fVar46;
        *(undefined8 *)(lVar34 + 0x20 + lVar31) = 0xffffffff;
        *(undefined8 *)(lVar34 + 0x28 + lVar31) = 0;
        plVar10[2] = lVar43 + 1;
        lVar43 = plStack_120[2];
        if (lVar43 == *plStack_120) {
          FUN_140b5f870(plStack_120,&UNK_142cecdc0);
        }
        *(undefined4 *)(plVar24[1] + lVar43 * 4) = uVar7;
        plVar24[2] = lVar43 + 1;
        lVar34 = lStack_258;
        lVar43 = lStack_128;
      }
      lVar34 = lVar34 + 0x48;
      lVar28 = lVar28 + -0x48;
    } while (lVar28 != 0);
  }
  FUN_140e14f50(&lStack_118,auStack_e0);
  lVar34 = lStack_108;
  uStack_180 = uStack_100;
  fVar18 = fStack_f0;
  fVar8 = fStack_ec;
  goto joined_r0x000141801a3f;
}

