// 0x142343a10 undefined FUN_142343a10(void) body_bytes=1675

/* WARNING: Instruction at (ram,0x000142344d22) overlaps instruction at (ram,0x000142344d21)
    */
/* WARNING: Control flow encountered bad instruction data */
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void FUN_142343a10(undefined8 *param_1,undefined8 *param_2,undefined8 *param_3,ulonglong *param_4,
                  undefined8 *param_5,undefined8 *param_6,undefined8 *param_7,undefined8 *param_8,
                  undefined8 *param_9,undefined8 *param_10,undefined8 *param_11,double *param_12,
                  longlong *param_13,ulonglong *param_14)

{
  uint *puVar1;
  float fVar2;
  byte bVar3;
  undefined8 *puVar4;
  ulonglong uVar5;
  double dVar6;
  double dVar7;
  longlong *plVar8;
  code *pcVar9;
  float fVar10;
  ulonglong uVar11;
  float fVar12;
  float fVar13;
  byte *pbVar14;
  char cVar15;
  longlong lVar16;
  ulonglong *puVar17;
  ulonglong uVar18;
  uint *puVar19;
  char *pcVar20;
  ulonglong *puVar21;
  undefined *puVar22;
  longlong lVar24;
  double *pdVar25;
  double *pdVar26;
  uint *puVar27;
  double *pdVar28;
  longlong lVar29;
  undefined2 uVar30;
  byte *pbVar31;
  ulonglong uVar32;
  double *pdVar33;
  undefined4 *puVar34;
  double *pdVar35;
  undefined8 uVar36;
  undefined8 *puVar37;
  double *pdVar38;
  undefined8 *puVar39;
  undefined8 *puVar40;
  uint *puVar41;
  undefined8 uVar42;
  double *pdVar43;
  undefined1 *puVar44;
  ulonglong *puVar45;
  uint uVar46;
  undefined4 uVar47;
  uint uVar48;
  longlong lVar49;
  ulonglong uVar50;
  ulonglong uVar51;
  longlong lVar52;
  ulonglong *puVar53;
  undefined8 uVar54;
  longlong unaff_GS_OFFSET;
  byte in_AF;
  bool bVar55;
  byte in_TF;
  byte in_IF;
  byte in_NT;
  byte in_AC;
  byte in_VIF;
  byte in_VIP;
  byte in_ID;
  float10 in_ST0;
  float fVar56;
  float fVar57;
  float fVar58;
  undefined8 extraout_XMM0_Qb;
  undefined1 auVar59 [16];
  undefined8 extraout_XMM0_Qb_00;
  undefined8 extraout_XMM0_Qb_01;
  undefined1 auVar60 [16];
  float fVar61;
  int iVar63;
  undefined1 in_XMM1 [16];
  undefined1 auVar62 [16];
  undefined1 in_XMM2 [16];
  undefined1 auVar64 [16];
  float fVar66;
  float fVar67;
  float fVar68;
  float fVar69;
  float fVar70;
  uint uVar71;
  uint uVar72;
  float fVar73;
  float fVar74;
  ulonglong auStack_208 [4];
  double adStack_1e8 [4];
  float fStack_1c8;
  uint uStack_1c4;
  undefined8 uStack_1c0;
  byte abStack_1b8 [8];
  longlong alStack_1b0 [2];
  uint auStack_1a0 [2];
  longlong lStack_198;
  double adStack_190 [2];
  uint uStack_17c;
  longlong alStack_178 [11];
  undefined1 auStack_120 [8];
  undefined8 uStack_118;
  undefined1 uStack_110;
  longlong alStack_108 [25];
  undefined7 uVar23;
  undefined8 extraout_XMM0_Qb_02;
  undefined4 uVar65;
  
  pdVar38 = adStack_1e8;
  uVar48 = *(uint *)*param_1;
  puVar41 = (uint *)(ulonglong)uVar48;
  *(undefined4 *)param_9[2] = *(undefined4 *)((longlong)param_9 + 0x1c);
  alStack_178[10] = *param_9;
  *(undefined4 *)param_10[2] = *(undefined4 *)((longlong)param_10 + 0x1c);
  fVar13 = _DAT_14304e2fc;
  fVar67 = _DAT_14295a0e8;
  fVar12 = _DAT_14295a0e4;
  fVar2 = _DAT_1429345a8;
  fVar69 = _DAT_1429258d8;
  fVar58 = _DAT_142925870;
  auStack_120[0] = 0;
  uStack_118 = *param_10;
  uStack_110 = 1;
  alStack_108[1] = 2;
  puVar4 = (undefined8 *)*param_4;
  alStack_178[9] = *param_14;
  alStack_178[7] = *param_7;
  alStack_1b0[1] = *param_6;
  puVar53 = (ulonglong *)param_6[1];
  adStack_190[0] = *param_12;
  adStack_190[1] = param_12[1];
  auStack_1a0[1] = *(undefined4 *)((longlong)param_12 + 0x14);
  alStack_178[5] = *param_2;
  alStack_178[6] = *param_3;
  alStack_178[2] = *param_5;
  alStack_178[4] = *param_8;
  puVar19 = (uint *)(ulonglong)*(uint *)((longlong)param_11 + 0x1c);
  pdVar43 = (double *)*param_11;
  alStack_178[8] = param_11[2];
  auStack_1a0[0] = *(undefined4 *)((longlong)param_13 + 0x1c);
  alStack_178[1] = param_13[2];
  fVar73 = (float)_DAT_14304e310;
  uVar18 = (ulonglong)_DAT_14304e310 >> 0x20;
  fVar74 = (float)_DAT_14304e300;
  uVar11 = (ulonglong)_DAT_14304e300 >> 0x20;
  puVar44 = auStack_120;
  lStack_198 = *param_13;
  alStack_178[0] = *param_13 + 0x20;
  lVar24 = 2;
  lVar16 = 0;
  fVar57 = _DAT_142a13848;
LAB_142343c18:
  pbVar14 = puVar44 + lVar16 * 0x10;
  do {
    pbVar31 = pbVar14;
    if (lVar24 == lVar16) {
      return;
    }
    lVar16 = lVar16 + 1;
    bVar3 = *pbVar31;
    pdVar35 = (double *)(ulonglong)bVar3;
    pbVar14 = pbVar31 + 0x10;
  } while ((uVar48 == 0x13) != (bool)bVar3);
  pdVar38[0x1c] = (double)lVar16;
  pdVar25 = *(double **)(pbVar31 + -8);
  uVar71 = 0;
  uVar72 = 0;
  fVar70 = *(float *)(puVar4 + 0x1d) * *(float *)pdVar38[0x17];
  uVar65 = SUB84(pdVar43,0);
  if (*(char *)((longlong)puVar4 + 0xf1) == '\0') {
    *(undefined4 *)((longlong)pdVar25 + 0x2c) = 0;
    goto LAB_142344410;
  }
  pdVar38[7] = (double)pdVar25;
  pdVar38[-1] = 2.67076420527417e-314;
  auVar59._0_8_ = func_0x000140949c10(pdVar38[0x15]);
  auVar59._8_8_ = extraout_XMM0_Qb;
  lVar16 = (longlong)pdVar38[8];
  pdVar25 = *(double **)(lVar16 + 0xf8);
  pdVar33 = (double *)((longlong)pdVar25 + *(longlong *)(lVar16 + 0x100) * 4);
  if (*(char *)(lVar16 + 0x108) != '\0') {
    param_4 = (ulonglong *)0x0;
    puVar45 = (ulonglong *)0x0;
    lVar16 = 0;
    bVar55 = false;
    pdVar38[6] = 0.0;
    param_12 = (double *)0x0;
    do {
      while (puVar45 == param_4) {
        do {
          if (pdVar25 == pdVar33) goto LAB_142343f2f;
          uVar32 = (ulonglong)*(uint *)pdVar25;
          pdVar25 = (double *)((longlong)pdVar25 + 4);
          puVar45 = (ulonglong *)(uVar32 * 9);
          param_4 = *(ulonglong **)(puVar53[0x35] + 0x10 + uVar32 * 0x48);
        } while (param_4 == (ulonglong *)0x0);
        lVar24 = puVar53[0x35] + uVar32 * 0x48;
        if ((*(ulonglong *)((longlong)pdVar38[8] + 0x118) < *(ulonglong *)(lVar24 + 0x40)) &&
           (uVar32 = *(ulonglong *)
                      (*(longlong *)(lVar24 + 0x38) +
                      *(ulonglong *)((longlong)pdVar38[8] + 0x118) * 8), uVar32 != 0)) {
          lVar16 = *(longlong *)(*(longlong *)(lVar24 + 0x18) + 0x10 + ~uVar32 * 0x30);
          uVar32 = *(ulonglong *)((longlong)pdVar38[8] + 0x120);
        }
        else {
          lVar16 = 0;
          uVar32 = *(ulonglong *)((longlong)pdVar38[8] + 0x120);
        }
        if (uVar32 < *(ulonglong *)(lVar24 + 0x40)) {
          bVar55 = *(longlong *)(*(longlong *)(lVar24 + 0x38) + uVar32 * 8) != 0;
          puVar17 = (ulonglong *)0x0;
          puVar21 = (ulonglong *)0x1;
          goto joined_r0x000142343d79;
        }
        puVar17 = (ulonglong *)0x0;
        bVar55 = false;
        puVar45 = (ulonglong *)0x1;
        if (bVar3 == 0) goto LAB_142343d7f;
      }
      puVar21 = (ulonglong *)((longlong)puVar45 + 1);
      puVar17 = puVar45;
joined_r0x000142343d79:
      puVar45 = puVar21;
      if (bVar55 == (bool)bVar3) {
LAB_142343d7f:
        param_14 = (ulonglong *)
                   CONCAT71((int7)((ulonglong)pdVar38[6] >> 8),
                            SUB81(pdVar38[6],0) |
                            *(byte *)(lVar16 + 1 + ((ulonglong)puVar17 & 0xffffffff) * 3));
        pdVar38[6] = (double)param_14;
        param_12 = (double *)(ulonglong)((int)param_12 + 1);
      }
    } while( true );
  }
  *(int *)((longlong)pdVar38 + 0x6c) = (int)puVar19;
  param_12 = (double *)0x0;
  pdVar38[6] = 0.0;
  param_4 = (ulonglong *)0x0;
  puVar45 = (ulonglong *)0x0;
  param_14 = (ulonglong *)0x0;
  while (pdVar25 != pdVar33) {
    uVar32 = (ulonglong)*(uint *)pdVar25;
    pdVar25 = (double *)((longlong)pdVar25 + 4);
    puVar45 = (ulonglong *)puVar53[0x20];
    param_4 = (ulonglong *)puVar45[uVar32 * 0x14 + 0xb];
    if (param_4 != (ulonglong *)0x0) {
      uVar50 = (ulonglong)*(uint *)((longlong)puVar45 + uVar32 * 0xa0 + 0x94);
      uVar51 = puVar53[0x35];
      if ((*(ulonglong *)((longlong)pdVar38[8] + 0x118) <
           *(ulonglong *)(uVar51 + 0x40 + uVar50 * 0x48)) &&
         (uVar5 = *(ulonglong *)
                   (*(longlong *)(uVar51 + 0x38 + uVar50 * 0x48) +
                   *(ulonglong *)((longlong)pdVar38[8] + 0x118) * 8), uVar5 != 0)) {
        lVar16 = *(longlong *)(*(longlong *)(uVar51 + uVar50 * 0x48 + 0x18) + 0x10 + ~uVar5 * 0x30);
        uVar51 = *(ulonglong *)((longlong)pdVar38[8] + 0x120);
        if (puVar45[uVar32 * 0x14 + 0x11] <= uVar51) goto LAB_142343ead;
LAB_142343ec9:
        bVar55 = *(longlong *)(puVar45[uVar32 * 0x14 + 0x10] + uVar51 * 8) != 0;
      }
      else {
        lVar16 = 0;
        uVar51 = *(ulonglong *)((longlong)pdVar38[8] + 0x120);
        if (uVar51 < puVar45[uVar32 * 0x14 + 0x11]) goto LAB_142343ec9;
LAB_142343ead:
        bVar55 = false;
      }
      uVar32 = puVar45[uVar32 * 0x14 + 10];
      puVar21 = (ulonglong *)0x1;
      puVar17 = (ulonglong *)0x0;
      while( true ) {
        puVar45 = puVar21;
        if (bVar55 == (bool)bVar3) {
          uVar46 = *(uint *)(uVar32 + 8 + (longlong)puVar17 * 0x10);
          pdVar38[0x11] = (double)param_12;
          pdVar38[6] = (double)CONCAT71((int7)((ulonglong)pdVar38[6] >> 8),
                                        SUB81(pdVar38[6],0) |
                                        *(byte *)(lVar16 + 1 + (ulonglong)uVar46 * 3));
          param_12 = (double *)(ulonglong)(SUB84(pdVar38[0x11],0) + 1);
        }
        param_14 = param_4;
        if (puVar45 == param_4) break;
        puVar21 = (ulonglong *)((longlong)puVar45 + 1);
        puVar17 = puVar45;
      }
    }
  }
  puVar19 = (uint *)(ulonglong)*(uint *)((longlong)pdVar38 + 0x6c);
LAB_142343f2f:
  fVar68 = ((float)(int)param_12 + fVar12) * fVar2 * fVar70 + fVar70 * fVar13 * (float)(int)param_12
  ;
  fVar61 = fVar70 * fVar13 * fVar69;
  auVar62 = ZEXT416((uint)fVar61);
  uVar32 = (ulonglong)*(byte *)(puVar4 + 0x1e);
  switch(uVar32) {
  case 1:
    fVar56 = fVar61 * fVar58;
    break;
  case 2:
    fVar56 = (float)auVar59._0_8_ + fVar61 * fVar67;
    break;
  default:
    pdVar26 = (double *)&UNK_14304e6b8;
    iVar63 = *(int *)(&UNK_14304e6b8 + uVar32 * 4);
    puVar17 = (ulonglong *)(&UNK_14304e6b8 + iVar63);
    cVar15 = (char)puVar17;
    pdVar28 = pdVar35;
    pdVar25 = pdVar35;
                    /* WARNING (jumptable): Sanity check requires truncation of jumptable */
                    /* WARNING: Could not find normalized switch variable to match jumptable */
    switch(uVar32) {
    case 3:
      in_XMM2 = ZEXT416((uint)(fVar61 * fVar58));
      fVar56 = (float)auVar59._0_8_ * fVar58;
      fVar66 = fVar68;
      fVar57 = fVar73;
      fVar10 = (float)uVar18;
      fVar68 = fVar61;
      goto code_r0x000142343fdc;
    case 4:
      pdVar26 = (double *)CONCAT62(0x14304,CONCAT11(-0x1a - cVar15,0xb8));
      break;
    case 5:
      goto code_r0x000142344493;
    case 6:
      pdVar33 = (double *)(ulonglong)*(uint *)((longlong)puVar17 + (longlong)pdVar33 * 4);
      goto code_r0x000142344493;
    case 7:
      puVar37 = pdVar38 + -1;
      pdVar38 = pdVar38 + -1;
      *puVar37 = pdVar43;
      puVar17 = (ulonglong *)(ulonglong)((int)puVar17 - 0x5e1445);
      break;
    case 8:
      *(char *)(pdVar35 + (longlong)puVar41) =
           *(char *)(pdVar35 + (longlong)puVar41) + (char)((ulonglong)puVar17 >> 8);
                    /* WARNING: Bad instruction - Truncating control flow here */
      halt_baddata();
    case 9:
      *(uint *)pdVar43 = uVar48;
      goto code_r0x000142344742;
    case 10:
      goto code_r0x000142344680;
    case 0xb:
      *(uint *)pdVar43 = uVar48;
      pdVar38[-1] = (double)&UNK_1423446ef;
      pdVar26 = param_12;
      FUN_142375ce0(param_12,(ulonglong)pdVar43 & 0xffffffff);
      goto code_r0x0001423447af;
    default:
      goto code_r0x000142344827;
    case 0xd:
    case 0xf:
      *(uint *)pdVar43 = uVar48;
      goto code_r0x0001423447a2;
    case 0x10:
      pdVar38[-1] = (double)param_12;
      pdVar38[-2] = (double)pdVar43;
      pdVar38[-3] = (double)puVar41;
      pdVar38[-4] = (double)pdVar35;
      pdVar38 = pdVar38 + -8;
      puVar19 = (uint *)*puVar45;
      param_14 = (ulonglong *)(ulonglong)*puVar19;
      puVar41 = (uint *)(ulonglong)puVar19[1];
      pdVar43 = (double *)(ulonglong)puVar19[2];
      goto code_r0x0001423448ba;
    case 0x11:
      goto code_r0x000142344963;
    case 0x12:
      if (puVar17 == (ulonglong *)0x0) goto code_r0x0001423448e9;
      *pdVar33 = (double)in_ST0;
      puVar17 = (ulonglong *)(ulonglong)(uint)((int)puVar17 + (int)*puVar17);
      pdVar26 = (double *)&UNK_14304e6b7;
    case 0x13:
      goto code_r0x000142344924;
    case 0x14:
      if (1 < (byte)((char)iVar63 + 0xb7U)) {
        pdVar26 = (double *)&UNK_14304e708;
        goto code_r0x000142344ab6;
      }
    case 0x15:
      goto code_r0x000142344adf;
    case 0x16:
      goto code_r0x000142344ab6;
    case 0x17:
      goto code_r0x000142344ac8;
    case 0x18:
      *(char *)puVar17 = (char)*puVar17 + cVar15;
      goto code_r0x000142344d17;
    case 0x19:
                    /* WARNING: Bad instruction - Truncating control flow here */
      halt_baddata();
    case 0x1a:
      puVar17 = (ulonglong *)((ulonglong)puVar17 & 0xffffffff);
      goto code_r0x000142344d1f;
    case 0x1b:
      puVar17 = (ulonglong *)
                CONCAT71((int7)((ulonglong)puVar17 >> 8),
                         cVar15 + -0x77 + (0xbcfb1947 < *(uint *)(&UNK_14304e645 + iVar63)));
      goto code_r0x000142344d1f;
    }
    goto code_r0x0001423444ab;
  }
  in_XMM2._0_4_ = in_XMM1._0_4_ * fVar58;
  in_XMM2._4_12_ = in_XMM1._4_12_;
  fVar66 = fVar61;
  fVar57 = fVar74;
  fVar10 = (float)uVar11;
code_r0x000142343fdc:
  lVar16 = (longlong)pdVar38[0xb];
  puVar27 = *(uint **)(lVar16 + 0xf8);
  puVar1 = puVar27 + *(longlong *)(lVar16 + 0x100);
  fVar66 = fVar66 + fVar70 * fVar57 + fVar70 * fVar57;
  fVar68 = fVar68 + fVar70 * fVar10 + fVar70 * fVar10;
  if (*(char *)(lVar16 + 0x108) != '\0') {
    param_4 = (ulonglong *)0x0;
    uVar32 = 0;
    uVar51 = 0;
    do {
      if (uVar51 == uVar32) {
        if (puVar27 == puVar1) goto code_r0x0001423441dd;
        while( true ) {
          uVar51 = (ulonglong)*puVar27;
          param_4 = (ulonglong *)(uVar51 * 9);
          uVar32 = *(ulonglong *)
                    (*(longlong *)((longlong)pdVar38[0xc] + 0x1a8) + 0x10 + uVar51 * 0x48);
          if (uVar32 != 0) break;
          puVar27 = puVar27 + 1;
          if (puVar27 == puVar1) goto code_r0x0001423441dd;
        }
        lVar16 = *(longlong *)((longlong)pdVar38[0xc] + 0x1a8) + uVar51 * 0x48;
        lVar24 = *(longlong *)(lVar16 + 0x18);
        lVar16 = ~*(ulonglong *)
                   (*(longlong *)(lVar16 + 0x38) + *(longlong *)((longlong)pdVar38[0xb] + 0x110) * 8
                   ) * 0x30;
        param_4 = *(ulonglong **)(lVar24 + 0x10 + lVar16);
        lVar16 = *(longlong *)(lVar24 + 0x28 + lVar16);
        puVar27 = puVar27 + 1;
        uVar51 = 0;
      }
      uVar50 = uVar51 & 0xffffffff;
      uVar51 = uVar51 + 1;
      *(float *)(param_4 + uVar50 * 2) = fVar56;
      *(int *)((longlong)param_4 + uVar50 * 0x10 + 4) = in_XMM2._0_4_;
      param_14 = (ulonglong *)(ulonglong)*(uint *)((longlong)pdVar38 + 0x4c);
      *(uint *)(lVar16 + uVar50 * 4) = *(uint *)((longlong)pdVar38 + 0x4c);
      param_4[uVar50 * 2 + 1] = CONCAT44(fVar68,fVar66);
    } while( true );
  }
  while (puVar27 != puVar1) {
    lVar16 = *(longlong *)((longlong)pdVar38[0xc] + 0x100);
    while( true ) {
      param_4 = (ulonglong *)((ulonglong)*puVar27 * 0xa0);
      lVar24 = *(longlong *)(lVar16 + 0x58 + (longlong)param_4);
      if (lVar24 != 0) break;
      puVar27 = puVar27 + 1;
      if (puVar27 == puVar1) goto code_r0x0001423441dd;
    }
    uVar32 = (ulonglong)*(uint *)((longlong)param_4 + lVar16 + 0x94);
    lVar52 = *(longlong *)(*(longlong *)((longlong)pdVar38[0xc] + 0x1a8) + 0x18 + uVar32 * 0x48);
    lVar49 = ~*(ulonglong *)
               (*(longlong *)(*(longlong *)((longlong)pdVar38[0xc] + 0x1a8) + 0x38 + uVar32 * 0x48)
               + *(longlong *)((longlong)pdVar38[0xb] + 0x110) * 8) * 0x30;
    lVar29 = *(longlong *)(lVar52 + 0x10 + lVar49);
    puVar45 = *(ulonglong **)(lVar52 + 0x28 + lVar49);
    lVar16 = *(longlong *)((longlong)param_4 + lVar16 + 0x50);
    puVar27 = puVar27 + 1;
    lVar52 = 0;
    do {
      uVar32 = (ulonglong)*(uint *)(lVar16 + 8 + lVar52 * 0x10);
      param_14 = (ulonglong *)(uVar32 * 0x10);
      lVar52 = lVar52 + 1;
      *(float *)(lVar29 + (longlong)param_14) = fVar56;
      *(int *)(lVar29 + 4 + (longlong)param_14) = in_XMM2._0_4_;
      *(int *)((longlong)puVar45 + uVar32 * 4) = *(int *)((longlong)pdVar38 + 0x4c);
      *(ulonglong *)(lVar29 + 8 + (longlong)param_14) = CONCAT44(fVar68,fVar66);
      param_12 = pdVar43;
      param_4 = puVar45;
    } while (lVar52 != lVar24);
  }
code_r0x0001423441dd:
  fVar61 = *(float *)((longlong)pdVar38[7] + 0x2c);
  if ((*(byte *)(pdVar38 + 6) & 1) == 0) {
    *(float *)(pdVar38 + 4) = fVar68;
    pdVar38[-1] = (double)&UNK_142344227;
    FUN_14123fff0(pdVar38 + 0x1e);
    in_XMM2 = ZEXT416(*(uint *)((longlong)pdVar38[0x13] + 4));
    pdVar38[-1] = (double)&UNK_142344240;
    cVar15 = func_0x000141240140(pdVar38 + 0x1e);
    if (cVar15 != '\0') goto code_r0x000142344244;
code_r0x000142344293:
    pdVar25 = (double *)pdVar38[7];
    fVar57 = *(float *)((longlong)pdVar38[0x10] + 0x58) / fVar69 +
             *(float *)((longlong)pdVar25 + 0x2c);
  }
  else {
code_r0x000142344244:
    pdVar38[-1] = (double)&UNK_142344251;
    cVar15 = func_0x00014093fbe0(pdVar38[0x14]);
    if (cVar15 == '\0') goto code_r0x000142344293;
    uVar47 = *(undefined4 *)pdVar38[0x12];
    pdVar38[-1] = (double)&UNK_142344269;
    cVar15 = func_0x00014093fa70(uVar47);
    pdVar25 = (double *)pdVar38[7];
    fVar57 = *(float *)((longlong)pdVar25 + 0x2c);
    if (cVar15 == '\0') {
      fVar57 = fVar57 + *(float *)((longlong)pdVar38[0x10] + 0x58) / _UNK_142925900;
    }
  }
  fVar68 = 0.0;
  if (0.0 <= fVar57) {
    fVar68 = fVar57;
  }
  fVar10 = _DAT_142925904;
  if (fVar68 <= _DAT_142925904) {
    fVar10 = fVar68;
  }
  *(float *)((longlong)pdVar25 + 0x2c) = fVar10;
  fVar57 = _DAT_142a13848;
  if ((fVar61 != 0.0) || (in_AF = 0, fVar10 <= 0.0)) {
    in_AF = 0;
    if ((fVar61 == _DAT_142925904) && (in_AF = 0, fVar10 < _DAT_142925904)) {
      *(undefined4 *)pdVar38[0xf] = *(undefined4 *)(pdVar38 + 9);
      lVar16 = (longlong)pdVar38[10];
      lVar24 = *(longlong *)(lVar16 + 0x30);
      param_14 = *(ulonglong **)(lVar16 + 0x40);
      if (lVar24 == *(longlong *)(lVar16 + 0x20)) {
        pdVar38[-1] = (double)&UNK_1423443d5;
        FUN_140562c30(pdVar38[0xe],&UNK_142bb5270);
        pdVar25 = (double *)pdVar38[7];
      }
      lVar16 = (longlong)pdVar38[10];
      lVar52 = *(longlong *)(lVar16 + 0x28);
      *(undefined4 *)(lVar52 + lVar24 * 0x20) = 0x11;
      *(ulonglong **)(lVar52 + 0x18 + lVar24 * 0x20) = param_14;
      *(longlong *)(lVar16 + 0x30) = lVar24 + 1;
      *(longlong *)(lVar16 + 0x40) = *(longlong *)(lVar16 + 0x40) + 1;
    }
  }
  else {
    *(undefined4 *)pdVar38[0xf] = *(undefined4 *)(pdVar38 + 9);
    lVar16 = (longlong)pdVar38[10];
    lVar24 = *(longlong *)(lVar16 + 0x30);
    param_14 = *(ulonglong **)(lVar16 + 0x40);
    if (lVar24 == *(longlong *)(lVar16 + 0x20)) {
      pdVar38[-1] = (double)&UNK_142344314;
      FUN_140562c30(pdVar38[0xe],&UNK_142bb5270);
      pdVar25 = (double *)pdVar38[7];
    }
    lVar16 = (longlong)pdVar38[10];
    lVar52 = *(longlong *)(lVar16 + 0x28);
    *(undefined4 *)(lVar52 + lVar24 * 0x20) = 0x10;
    *(ulonglong **)(lVar52 + 0x18 + lVar24 * 0x20) = param_14;
    *(longlong *)(lVar16 + 0x30) = lVar24 + 1;
    *(longlong *)(lVar16 + 0x40) = *(longlong *)(lVar16 + 0x40) + 1;
    fVar57 = _DAT_142a13848;
  }
LAB_142344410:
  auVar62 = ZEXT416(*(uint *)((longlong)pdVar25 + 0x2c));
  pdVar38[-1] = 2.67076516227932e-314;
  fVar68 = (float)FUN_140c95ef0(&UNK_14304e668);
  fVar70 = fVar70 * fVar57 * fVar68;
  *(int *)pdVar38[0x16] = (int)puVar19;
  puVar17 = (ulonglong *)pdVar43[0x14];
  pdVar26 = (double *)0x0;
  puVar45 = puVar17;
  do {
    if ((double *)((longlong)pdVar43[0x15] * 0x28) == pdVar26) goto LAB_1423444ca;
    puVar21 = puVar45 + 2;
    puVar45 = puVar45 + 5;
    auVar62._0_4_ = -(uint)((int)*puVar21 == SUB84(pdVar25[3],0));
    iVar63 = -(uint)((int)(*puVar21 >> 0x20) == (int)((ulonglong)pdVar25[3] >> 0x20));
    auVar62._4_4_ = auVar62._0_4_;
    auVar62._8_4_ = iVar63;
    auVar62._12_4_ = iVar63;
    uVar46 = movmskpd((int)param_4,auVar62);
    param_4 = (ulonglong *)(ulonglong)uVar46;
    pdVar26 = pdVar26 + 5;
  } while (uVar46 != 3);
  pdVar33 = (double *)(ulonglong)*(byte *)(puVar4 + 0x1e);
  auVar59 = (undefined1  [16])0x0;
  in_XMM1 = in_XMM2;
code_r0x000142344493:
  uVar47 = SUB84(param_4,0);
  uVar36 = in_XMM1._0_8_;
  puVar45 = (ulonglong *)&UNK_14304e6c8;
  uVar32 = (ulonglong)*(int *)(&UNK_14304e6c8 + (longlong)pdVar33 * 4);
  bVar55 = 0xfffffffebcfb1937 < uVar32;
  pdVar33 = (double *)(&UNK_14304e6c8 + uVar32);
  cVar15 = (char)puVar17;
  in_XMM2 = in_XMM1;
  pdVar28 = pdVar25;
                    /* WARNING (jumptable): Sanity check requires truncation of jumptable */
  switch(pdVar33) {
  case (double *)0x1423444a3:
    fVar70 = (float)((uint)fVar70 ^ _UNK_1429258f0);
    uVar71 = _UNK_1429258f8;
    uVar72 = _UNK_1429258fc;
  case (double *)0x1423444ab:
code_r0x0001423444ab:
    auVar59._4_4_ = 0;
    auVar59._0_4_ = fVar70;
    auVar59._8_4_ = uVar71;
    auVar59._12_4_ = uVar72;
    fVar70 = 0.0;
    in_XMM2 = in_XMM1;
    goto code_r0x0001423444bd;
  case (double *)0x1423444b5:
    fVar70 = (float)((uint)fVar70 ^ _UNK_1429258f0);
code_r0x0001423444bd:
    *(int *)((longlong)(puVar17 + -4) + (longlong)pdVar26) = auVar59._0_4_;
    *(float *)((longlong)((longlong)puVar17 + -0x1c) + (longlong)pdVar26) = fVar70;
LAB_1423444ca:
    lVar16 = *(longlong *)((longlong)pdVar38 + 0xe0);
    lVar24 = *(longlong *)((longlong)pdVar38 + 0xe8);
    puVar44 = (undefined1 *)((longlong)pdVar38 + 200);
    in_XMM1 = auVar62;
    goto LAB_142343c18;
  case (double *)0x1423444bd:
    goto code_r0x0001423444bd;
  case (double *)0x142344638:
    goto code_r0x000142344638;
  case (double *)0x142344690:
code_r0x000142344680:
    pdVar38[-1] = (double)&UNK_142344687;
    FUN_142375b40();
    *puVar19 = uVar48;
    pdVar38[-1] = (double)&UNK_142344697;
    pdVar26 = param_12;
    FUN_142375ce0(param_12,(ulonglong)pdVar43 & 0xffffffff);
    break;
  case (double *)0x1423446f0:
    pdVar35 = (double *)0x66000000;
    uVar65 = *(undefined4 *)(pdVar38 + 4);
    puVar34 = (undefined4 *)pdVar38[5];
    *puVar34 = uVar65;
    pdVar38[-1] = (double)&UNK_14234471a;
    FUN_142376050(param_12,(ulonglong)param_14 & 0xffffffff,(ulonglong)pdVar43 & 0xffffffff);
    *puVar34 = uVar65;
    pdVar38[-1] = (double)&UNK_14234472b;
    FUN_142376300(param_12,(ulonglong)param_14 & 0xffffffff,(ulonglong)pdVar43 & 0xffffffff);
    *puVar34 = uVar65;
    pdVar43 = (double *)(ulonglong)*(uint *)((longlong)pdVar38 + 0x24);
    pdVar38[-1] = (double)&UNK_14234473f;
    FUN_142375b40(param_12,pdVar43,(ulonglong)puVar53 & 0xffffffff);
    *puVar34 = uVar65;
code_r0x000142344742:
    pdVar38[-1] = (double)&UNK_14234474f;
    pdVar26 = param_12;
    FUN_142376300(param_12,(ulonglong)pdVar43 & 0xffffffff);
    pdVar25 = pdVar35;
    break;
  case (double *)0x142344750:
    param_12 = (double *)*pdVar38;
    uVar65 = *(undefined4 *)(pdVar38 + 5);
    puVar34 = (undefined4 *)pdVar38[6];
    *puVar34 = uVar65;
    *pdVar38 = (double)&UNK_14234477a;
    FUN_142375c10(param_12,(ulonglong)param_14 & 0xffffffff,(ulonglong)pdVar43 & 0xffffffff);
    *puVar34 = uVar65;
    *pdVar38 = (double)&UNK_14234478b;
    FUN_142375f80(param_12,(ulonglong)param_14 & 0xffffffff,(ulonglong)pdVar43 & 0xffffffff);
    *puVar34 = uVar65;
    pdVar43 = (double *)(ulonglong)*(uint *)((longlong)pdVar38 + 0x2c);
    *pdVar38 = (double)&UNK_14234479f;
    FUN_142375eb0(param_12,pdVar43,(ulonglong)puVar53 & 0xffffffff);
    *puVar34 = uVar65;
    pdVar38 = pdVar38 + 1;
code_r0x0001423447a2:
    *(undefined **)((longlong)pdVar38 + -8) = &UNK_1423447af;
    pdVar26 = param_12;
    FUN_142376300(param_12,(ulonglong)pdVar43 & 0xffffffff);
    break;
  case (double *)0x1423447b0:
    pdVar38 = (double *)(ulonglong)*(uint *)(pdVar25 + 1);
    break;
  case (double *)0x14234484d:
    *(char *)puVar17 = (char)*puVar17 + cVar15;
    goto code_r0x000142344850;
  case (double *)0x1423448b6:
    uVar23 = (undefined7)((ulonglong)puVar17 >> 8);
    cVar15 = cVar15 + -0x75;
    puVar19 = (uint *)CONCAT71(uVar23,cVar15);
    if (cVar15 < '\0') {
      puVar19 = (uint *)CONCAT71(uVar23,cVar15);
      *(char *)puVar19 = (char)*puVar19 + cVar15;
    }
code_r0x0001423448ba:
    param_12 = (double *)(ulonglong)puVar19[3];
    puVar17 = (ulonglong *)(ulonglong)*(byte *)((longlong)*pdVar26 + 0xf0);
    puVar19 = (uint *)(ulonglong)*(uint *)((longlong)pdVar33 + 0x1c);
    puVar53 = (ulonglong *)pdVar33[2];
    *(uint *)puVar53 = *(uint *)((longlong)pdVar33 + 0x1c);
    pdVar35 = (double *)*pdVar33;
    goto code_r0x0001423448d6;
  case (double *)0x1423448f5:
    *(undefined4 *)pdVar38 = uVar65;
    pdVar35 = pdVar26;
    goto code_r0x0001423448e9;
  case (double *)0x142344934:
    *(undefined4 *)pdVar38 = uVar65;
    pdVar35 = pdVar25;
    goto code_r0x000142344924;
  case (double *)0x142344973:
    *(undefined4 *)pdVar38 = uVar65;
    goto code_r0x000142344963;
  case (double *)0x142344ab3:
    pdVar38[-1] = (double)((ulonglong)(in_NT & 1) * 0x4000 |
                           (ulonglong)SCARRY8(uVar32,0x14304e6c8) * 0x800 |
                           (ulonglong)(in_IF & 1) * 0x200 | (ulonglong)(in_TF & 1) * 0x100 |
                           (ulonglong)((longlong)pdVar33 < 0) * 0x80 |
                           (ulonglong)(pdVar33 == (double *)0x0) * 0x40 |
                           (ulonglong)(in_AF & 1) * 0x10 |
                           (ulonglong)((POPCOUNT((ulonglong)pdVar33 & 0xff) & 1U) == 0) * 4 |
                           (ulonglong)bVar55 | (ulonglong)(in_ID & 1) * 0x200000 |
                           (ulonglong)(in_VIP & 1) * 0x100000 | (ulonglong)(in_VIF & 1) * 0x80000 |
                          (ulonglong)(in_AC & 1) * 0x40000);
    *(char *)puVar17 = (char)*puVar17 << 1 | (char)*puVar17 < '\0';
code_r0x000142344ab6:
                    /* WARNING: Could not recover jumptable at 0x000142344abd. Too many branches */
                    /* WARNING: Treating indirect jump as call */
    (*(code *)((longlong)*(int *)((longlong)pdVar26 + (longlong)puVar17 * 4) + (longlong)pdVar26))()
    ;
    return;
  case (double *)0x142344ac6:
    *(char *)((longlong)pdVar26 + 0x658b4d01) =
         *(char *)((longlong)pdVar26 + 0x658b4d01) + (char)pdVar26 + bVar55;
    *(char *)((longlong)pdVar26 + -0x39) = *(char *)((longlong)pdVar26 + -0x39) + cVar15;
code_r0x000142344ac8:
    *(int *)((longlong)puVar41 + -0x75) = *(int *)((longlong)puVar41 + -0x75) + (int)pdVar26;
    pcVar20 = (char *)((longlong)pdVar26 + unaff_GS_OFFSET + -0x39);
    *pcVar20 = *pcVar20 + (char)puVar17;
    pcVar20 = (char *)((ulonglong)puVar17 & 0xffffffffffffff10);
    *pcVar20 = *pcVar20 + (char)pcVar20;
    *pcVar20 = *pcVar20 + (char)pcVar20;
    goto LAB_142344cd5;
  case (double *)0x142344ad8:
    goto code_r0x000142344ad8;
  case (double *)0x142344aef:
    puVar17 = (ulonglong *)(ulonglong)(uint)((int)puVar17 + (int)pdVar26);
    goto code_r0x000142344ae3;
  case (double *)0x142344d23:
    *(char *)((longlong)param_12 + 0x66) = *(char *)((longlong)param_12 + 0x66) << 6;
    goto LAB_142344d30;
  case (double *)0x142344d2a:
    *(char *)puVar17 = (char)*puVar17 + cVar15;
    *(char *)puVar17 = (char)*puVar17 + cVar15;
    goto LAB_142344d30;
  case (double *)0x142344d32:
    return;
  case (double *)0x142344d39:
    pdVar38[-1] = (double)puVar17;
    *(char *)((longlong)pdVar26 + -0x7d) =
         *(char *)((longlong)pdVar26 + -0x7d) + (char)pdVar26 + bVar55;
    *(byte *)puVar17 = (byte)*puVar17 >> 6;
    goto code_r0x000142344d43;
  }
code_r0x0001423447af:
  puVar53 = (ulonglong *)pdVar25[1];
  puVar41 = (uint *)((longlong)pdVar25[2] << 5);
  puVar19 = (uint *)0x0;
  pdVar35 = pdVar26;
  pdVar26 = param_12;
  param_14 = puVar53;
code_r0x0001423447f0:
  if (puVar41 == puVar19) {
    if ((*(byte *)((longlong)pdVar38 + 0x30) & 1) != 0) {
      return;
    }
    *(ulonglong *)((longlong)pdVar38 + 0x30) = CONCAT71((int7)((ulonglong)pdVar35 >> 8),1);
    *(undefined4 *)((longlong)pdVar38 + 0x24) =
         *(undefined4 *)(*(longlong *)((longlong)pdVar38 + 0x38) + 0x20);
    **(undefined4 **)((longlong)pdVar38 + 0x28) = *(undefined4 *)((longlong)pdVar38 + 0x20);
    dVar6 = pdVar26[0x15];
    if (dVar6 != 0.0) {
      dVar7 = pdVar26[0x14];
      lVar16 = 0;
      do {
        if (*(char *)((longlong)dVar7 + 0x24 + lVar16) != '\x04') {
          *(char *)((longlong)dVar7 + 0x24 + lVar16) =
               (char)(0x1020300 >> ((*(byte *)(puVar4 + 0x1e) & 3) << 3));
        }
        lVar16 = lVar16 + 0x28;
      } while ((longlong)dVar6 * 0x28 != lVar16);
    }
    puVar17 = (ulonglong *)(ulonglong)*(byte *)(puVar4 + 0x1e);
code_r0x000142344638:
                    /* WARNING: Could not recover jumptable at 0x000142344646. Too many branches */
                    /* WARNING: Treating indirect jump as call */
    (*(code *)(&UNK_14304e6d8 + *(int *)(&UNK_14304e6d8 + (longlong)puVar17 * 4)))();
    return;
  }
  pdVar33 = (double *)(ulonglong)(uint)param_14[2];
  if ((uint)param_14[2] != 0) {
    uVar48 = *(uint *)((longlong)((longlong)puVar53 + 0x14) + (longlong)puVar19);
    pdVar25 = (double *)(ulonglong)uVar48;
    cVar15 = *(char *)(puVar4 + 0x1e);
    **(undefined4 **)((longlong)pdVar38 + 0x28) = *(undefined4 *)((longlong)pdVar38 + 0x20);
    puVar45 = (ulonglong *)(ulonglong)uVar48;
    pdVar35 = pdVar25;
    param_12 = pdVar26;
    pdVar43 = pdVar33;
    if ((byte)(cVar15 - 1U) < 2) {
code_r0x000142344827:
      *(undefined **)((longlong)pdVar38 + -8) = &UNK_14234482c;
      auVar59._0_8_ = FUN_142375eb0();
      uVar47 = SUB84(param_4,0);
      uVar36 = in_XMM1._0_8_;
      auVar59._8_8_ = extraout_XMM0_Qb_00;
      pdVar26 = (double *)&UNK_14304e6e8;
      uVar18 = (ulonglong)*(int *)(&UNK_14304e6e8 + (ulonglong)*(byte *)(puVar4 + 0x1e) * 4);
      puVar17 = (ulonglong *)(&UNK_14304e6e8 + uVar18);
      puVar21 = puVar17;
      pdVar25 = pdVar35;
                    /* WARNING (jumptable): Sanity check requires truncation of jumptable */
                    /* WARNING: Could not find normalized switch variable to match jumptable */
      switch(*(byte *)(puVar4 + 0x1e)) {
      case 1:
      case 3:
code_r0x0001423447d0:
        pdVar26 = param_12;
        **(undefined4 **)((longlong)pdVar38 + 0x28) = *(undefined4 *)((longlong)pdVar38 + 0x20);
        *(undefined **)((longlong)pdVar38 + -8) = &UNK_1423447e8;
        pdVar35 = pdVar26;
        FUN_142376300(pdVar26,(ulonglong)pdVar43 & 0xffffffff);
        goto code_r0x0001423447e8;
      case 4:
        goto code_r0x0001423448d6;
      case 5:
code_r0x000142344993:
        uVar65 = SUB84(pdVar33,0);
        pdVar28 = (double *)&UNK_14304e6e8;
        *(byte *)((longlong)pdVar25 + 0x5d) = *(byte *)((longlong)pdVar25 + 0x5d) & (byte)pdVar25;
        goto code_r0x000142344996;
      case 6:
code_r0x000142344915:
        uVar65 = SUB84(pdVar33,0);
        pdVar28 = (double *)&UNK_14304e6e8;
        *(byte *)((longlong)pdVar25 + 0x5d) = *(byte *)((longlong)pdVar25 + 0x5d) & (byte)pdVar25;
        goto code_r0x000142344918;
      case 7:
code_r0x000142344954:
        uVar65 = SUB84(pdVar33,0);
        pdVar35 = (double *)&UNK_14304e6e8;
        *(byte *)((longlong)pdVar25 + 0x5d) = *(byte *)((longlong)pdVar25 + 0x5d) & (byte)pdVar25;
        goto code_r0x000142344957;
      case 8:
code_r0x000142344ad3:
        *(char *)puVar17 = (char)*puVar17 + (char)puVar17;
code_r0x000142344ad8:
        *(int *)puVar17 = (int)*puVar17 + (int)puVar17;
        *(char *)((longlong)puVar41 + 0xf) = *(char *)((longlong)puVar41 + 0xf) + (char)puVar17;
code_r0x000142344adf:
        goto code_r0x000142344ae3;
      case 9:
code_r0x000142344b0f:
        bVar55 = 0xfffffffebcfb1917 < uVar18;
        pdVar35 = pdVar25;
        goto code_r0x000142344b08;
      case 10:
code_r0x000142344ae6:
                    /* WARNING: Bad instruction - Truncating control flow here */
        halt_baddata();
      case 0xb:
code_r0x000142344af8:
                    /* WARNING: Bad instruction - Truncating control flow here */
        halt_baddata();
      case 0xc:
        goto code_r0x000142344d43;
      case 0xd:
code_r0x000142344d59:
        pdVar26 = (double *)&UNK_14304e6e8;
        uVar30 = CONCAT11(0x96,(char)pdVar33);
        goto code_r0x000142344d5b;
      case 0xe:
code_r0x000142344d52:
        pdVar26 = (double *)&UNK_14304e6e8;
        uVar48 = 0x4304e6e8;
        goto code_r0x000142344d55;
      case 0xf:
code_r0x000142344d4a:
        uVar36 = in_XMM1._0_8_;
        uVar48 = movmskps(0x4304e6e8,in_XMM1);
        pdVar26 = (double *)(ulonglong)uVar48;
        goto code_r0x000142344d43;
      }
    }
    else {
code_r0x000142344850:
      *(undefined **)((longlong)pdVar38 + -8) = &UNK_142344855;
      auVar59._0_8_ = FUN_142375b40(pdVar26);
      uVar47 = SUB84(param_4,0);
      uVar36 = in_XMM1._0_8_;
      auVar59._8_8_ = extraout_XMM0_Qb_01;
      pdVar26 = (double *)&UNK_14304e6e8;
      uVar18 = (ulonglong)*(int *)(&UNK_14304e6e8 + (ulonglong)*(byte *)(puVar4 + 0x1e) * 4);
      puVar17 = (ulonglong *)(&UNK_14304e6e8 + uVar18);
      puVar21 = puVar17;
      pdVar35 = pdVar25;
                    /* WARNING (jumptable): Sanity check requires truncation of jumptable */
                    /* WARNING: Could not find normalized switch variable to match jumptable */
      switch(*(byte *)(puVar4 + 0x1e)) {
      case 1:
      case 3:
        goto code_r0x0001423447d0;
      case 4:
        goto code_r0x0001423448d6;
      case 5:
        goto code_r0x000142344993;
      case 6:
        goto code_r0x000142344915;
      case 7:
        goto code_r0x000142344954;
      case 8:
        goto code_r0x000142344ad3;
      case 9:
        goto code_r0x000142344b0f;
      case 10:
        goto code_r0x000142344ae6;
      case 0xb:
        goto code_r0x000142344af8;
      case 0xc:
        goto code_r0x000142344d43;
      case 0xd:
        goto code_r0x000142344d59;
      case 0xe:
        goto code_r0x000142344d52;
      case 0xf:
        goto code_r0x000142344d4a;
      }
    }
    pdVar26 = param_12;
    **(undefined4 **)((longlong)pdVar38 + 0x28) = *(undefined4 *)((longlong)pdVar38 + 0x20);
    *(undefined **)((longlong)pdVar38 + -8) = &UNK_142344885;
    pdVar35 = pdVar26;
    FUN_142375ce0(pdVar26,(ulonglong)pdVar43 & 0xffffffff);
  }
code_r0x0001423447e8:
  param_14 = param_14 + 4;
  puVar19 = puVar19 + 8;
  goto code_r0x0001423447f0;
code_r0x0001423448d6:
  uVar30 = SUB82(pdVar33,0);
  pdVar26 = (double *)&UNK_14304e6f8;
  bVar55 = 0xfffffffebcfb1907 <
           (ulonglong)(longlong)*(int *)(&UNK_14304e6f8 + (longlong)puVar17 * 4);
  puVar17 = (ulonglong *)(&UNK_14304e6f8 + *(int *)(&UNK_14304e6f8 + (longlong)puVar17 * 4));
  pdVar28 = pdVar35;
                    /* WARNING (jumptable): Sanity check requires truncation of jumptable */
  switch(puVar17) {
  case (ulonglong *)0x1423448e6:
code_r0x0001423448e9:
    *(undefined **)((longlong)pdVar38 + -8) = &UNK_1423448f4;
    FUN_142376050(pdVar35,(ulonglong)param_14 & 0xffffffff,(ulonglong)puVar41 & 0xffffffff);
    *(int *)puVar53 = (int)puVar19;
    *(undefined **)((longlong)pdVar38 + -8) = &UNK_142344906;
    FUN_142376300(pdVar28,(ulonglong)param_14 & 0xffffffff,(ulonglong)puVar41 & 0xffffffff);
    *(int *)puVar53 = (int)puVar19;
    uVar65 = SUB84(pdVar43,0);
    puVar45 = (ulonglong *)((ulonglong)param_12 & 0xffffffff);
    pdVar25 = *(double **)((longlong)pdVar38 + 0x20);
    pdVar38 = (double *)((longlong)pdVar38 + 0x30);
code_r0x000142344918:
    uVar36 = *pdVar38;
    uVar42 = pdVar38[1];
    uVar54 = pdVar38[3];
    puVar40 = pdVar38 + 5;
    goto code_r0x000142375b40;
  case (ulonglong *)0x142344925:
    pdVar33 = (double *)((ulonglong)param_14 & 0xffffffff);
    pdVar26 = pdVar35;
code_r0x000142344924:
    *(char *)((longlong)puVar17 + -0x77) = *(char *)((longlong)puVar17 + -0x77) + (char)pdVar26;
    *(undefined **)((longlong)pdVar38 + -8) = &UNK_142344933;
    FUN_142375c10(pdVar26,pdVar33,(ulonglong)puVar41 & 0xffffffff);
    *(int *)puVar53 = (int)puVar19;
    *(undefined **)((longlong)pdVar38 + -8) = &UNK_142344945;
    FUN_142375f80(pdVar35,(ulonglong)param_14 & 0xffffffff,(ulonglong)puVar41 & 0xffffffff);
    *(int *)puVar53 = (int)puVar19;
    uVar65 = SUB84(pdVar43,0);
    puVar45 = (ulonglong *)((ulonglong)param_12 & 0xffffffff);
    pdVar25 = *(double **)((longlong)pdVar38 + 0x20);
    pdVar38 = (double *)((longlong)pdVar38 + 0x30);
code_r0x000142344957:
    uVar36 = *pdVar38;
    uVar42 = pdVar38[1];
    uVar54 = pdVar38[3];
    puVar39 = pdVar38 + 5;
code_r0x000142375eb0:
    *(undefined8 *)((longlong)puVar39 + -8) = uVar54;
    *(undefined8 *)((longlong)puVar39 + -0x10) = uVar42;
    *(undefined8 *)((longlong)puVar39 + -0x18) = uVar36;
    *(double **)((longlong)puVar39 + -0x20) = pdVar25;
    *(undefined **)((longlong)puVar39 + -0x1f0) = &UNK_142375ed7;
    FUN_140fe7cb0((undefined1 *)((longlong)puVar39 + -0x1b8),pdVar35,uVar65,
                  (ulonglong)puVar45 & 0xffffffff);
    if (*(int *)((longlong)puVar39 + -0x1b8) == 4) {
      *(undefined **)((longlong)puVar39 + -0x1f0) = &UNK_142375ef0;
      FUN_142374fd0((undefined1 *)((longlong)puVar39 + -0x178),
                    *(undefined8 *)((longlong)puVar39 + -0x1b0));
      *(undefined1 *)((longlong)puVar39 + -0x2d) = 1;
      *(undefined1 **)((longlong)puVar39 + -0x1c8) = (undefined1 *)((longlong)puVar39 + -0x178);
      *(undefined **)((longlong)puVar39 + -0x1f0) = &UNK_142375f10;
      FUN_140fe7b10((undefined1 *)((longlong)puVar39 + -0x1b8),pdVar35,uVar65,
                    (ulonglong)puVar45 & 0xffffffff);
      if (*(int *)((longlong)puVar39 + -0x1b8) == 4) {
        return;
      }
      *(undefined8 *)((longlong)puVar39 + -0x188) = *(undefined8 *)((longlong)puVar39 + -0x1a8);
      *(undefined8 *)((longlong)puVar39 + -0x180) = *(undefined8 *)((longlong)puVar39 + -0x1a0);
      *(undefined8 *)((longlong)puVar39 + -0x198) = *(undefined8 *)((longlong)puVar39 + -0x1b8);
      *(undefined8 *)((longlong)puVar39 + -400) = *(undefined8 *)((longlong)puVar39 + -0x1b0);
      puVar22 = &UNK_143052e68;
    }
    else {
      *(undefined8 *)((longlong)puVar39 + -0x188) = *(undefined8 *)((longlong)puVar39 + -0x1a8);
      *(undefined8 *)((longlong)puVar39 + -0x180) = *(undefined8 *)((longlong)puVar39 + -0x1a0);
      *(undefined8 *)((longlong)puVar39 + -0x198) = *(undefined8 *)((longlong)puVar39 + -0x1b8);
      *(undefined8 *)((longlong)puVar39 + -400) = *(undefined8 *)((longlong)puVar39 + -0x1b0);
      puVar22 = &UNK_143052e50;
    }
    *(undefined **)((longlong)puVar39 + -0x1c8) = puVar22;
    *(undefined **)((longlong)puVar39 + -0x1f0) = &UNK_142375f7e;
    FUN_1428d9760(&UNK_143052c50,0x2b,(undefined1 *)((longlong)puVar39 + -0x198),&UNK_143052c30);
    pcVar9 = (code *)swi(3);
    (*pcVar9)();
    return;
  case (ulonglong *)0x142344964:
    pdVar33 = (double *)((ulonglong)param_14 & 0xffffffff);
    pdVar26 = pdVar35;
code_r0x000142344963:
    *(char *)((longlong)puVar17 + -0x77) = *(char *)((longlong)puVar17 + -0x77) + (char)pdVar26;
    *(undefined **)((longlong)pdVar38 + -8) = &UNK_142344972;
    FUN_142376050(pdVar26,pdVar33,(ulonglong)puVar41 & 0xffffffff);
    *(int *)puVar53 = (int)puVar19;
    *(undefined **)((longlong)pdVar38 + -8) = &UNK_142344984;
    FUN_142375ce0(pdVar28,(ulonglong)param_14 & 0xffffffff,(ulonglong)puVar41 & 0xffffffff);
    *(int *)puVar53 = (int)puVar19;
    uVar65 = SUB84(pdVar43,0);
    puVar45 = (ulonglong *)((ulonglong)param_12 & 0xffffffff);
    pdVar25 = *(double **)((longlong)pdVar38 + 0x20);
    pdVar38 = (double *)((longlong)pdVar38 + 0x30);
code_r0x000142344996:
    uVar36 = *pdVar38;
    uVar42 = pdVar38[1];
    uVar54 = pdVar38[3];
    puVar40 = pdVar38 + 5;
code_r0x000142375b40:
    *(undefined8 *)((longlong)puVar40 + -8) = uVar54;
    *(undefined8 *)((longlong)puVar40 + -0x10) = uVar42;
    *(undefined8 *)((longlong)puVar40 + -0x18) = uVar36;
    *(double **)((longlong)puVar40 + -0x20) = pdVar25;
    *(undefined **)((longlong)puVar40 + -0x1f0) = &UNK_142375b67;
    FUN_140fe7cb0((undefined1 *)((longlong)puVar40 + -0x1b8),pdVar28,uVar65,
                  (ulonglong)puVar45 & 0xffffffff);
    if (*(int *)((longlong)puVar40 + -0x1b8) == 4) {
      *(undefined **)((longlong)puVar40 + -0x1f0) = &UNK_142375b80;
      FUN_142374fd0((undefined1 *)((longlong)puVar40 + -0x178),
                    *(undefined8 *)((longlong)puVar40 + -0x1b0));
      *(undefined1 *)((longlong)puVar40 + -0x2d) = 0;
      *(undefined1 **)((longlong)puVar40 + -0x1c8) = (undefined1 *)((longlong)puVar40 + -0x178);
      *(undefined **)((longlong)puVar40 + -0x1f0) = &UNK_142375ba0;
      FUN_140fe7b10((undefined1 *)((longlong)puVar40 + -0x1b8),pdVar28,uVar65,
                    (ulonglong)puVar45 & 0xffffffff);
      if (*(int *)((longlong)puVar40 + -0x1b8) == 4) {
        return;
      }
      *(undefined8 *)((longlong)puVar40 + -0x188) = *(undefined8 *)((longlong)puVar40 + -0x1a8);
      *(undefined8 *)((longlong)puVar40 + -0x180) = *(undefined8 *)((longlong)puVar40 + -0x1a0);
      *(undefined8 *)((longlong)puVar40 + -0x198) = *(undefined8 *)((longlong)puVar40 + -0x1b8);
      *(undefined8 *)((longlong)puVar40 + -400) = *(undefined8 *)((longlong)puVar40 + -0x1b0);
      puVar22 = &UNK_143052e68;
    }
    else {
      *(undefined8 *)((longlong)puVar40 + -0x188) = *(undefined8 *)((longlong)puVar40 + -0x1a8);
      *(undefined8 *)((longlong)puVar40 + -0x180) = *(undefined8 *)((longlong)puVar40 + -0x1a0);
      *(undefined8 *)((longlong)puVar40 + -0x198) = *(undefined8 *)((longlong)puVar40 + -0x1b8);
      *(undefined8 *)((longlong)puVar40 + -400) = *(undefined8 *)((longlong)puVar40 + -0x1b0);
      puVar22 = &UNK_143052e50;
    }
    *(undefined **)((longlong)puVar40 + -0x1c8) = puVar22;
    *(undefined **)((longlong)puVar40 + -0x1f0) = &UNK_142375c0e;
    FUN_1428d9760(&UNK_143052c50,0x2b,(undefined1 *)((longlong)puVar40 + -0x198),&UNK_143052c30);
    pcVar9 = (code *)swi(3);
    (*pcVar9)();
    return;
  case (ulonglong *)0x1423449a3:
    *(undefined **)((longlong)pdVar38 + -8) = &UNK_1423449b1;
    FUN_142376230(pdVar35,(ulonglong)param_14 & 0xffffffff,(ulonglong)puVar41 & 0xffffffff);
    *(int *)puVar53 = (int)puVar19;
    *(undefined **)((longlong)pdVar38 + -8) = &UNK_1423449c3;
    FUN_142375f80(pdVar35,(ulonglong)param_14 & 0xffffffff,(ulonglong)puVar41 & 0xffffffff);
    *(int *)puVar53 = (int)puVar19;
    uVar65 = SUB84(pdVar43,0);
    puVar45 = (ulonglong *)((ulonglong)param_12 & 0xffffffff);
    pdVar25 = *(double **)((longlong)pdVar38 + 0x20);
    uVar36 = *(undefined8 *)((longlong)pdVar38 + 0x30);
    uVar42 = *(undefined8 *)((longlong)pdVar38 + 0x38);
    uVar54 = *(undefined8 *)((longlong)pdVar38 + 0x48);
    puVar39 = (undefined8 *)((longlong)pdVar38 + 0x58);
    goto code_r0x000142375eb0;
  case (ulonglong *)0x142344ae3:
code_r0x000142344ae3:
                    /* WARNING: Could not recover jumptable at 0x000142344af1. Too many branches */
                    /* WARNING: Treating indirect jump as call */
    (*(code *)(&UNK_14304e708 + *(int *)(&UNK_14304e708 + (longlong)puVar17 * 4)))();
    return;
  case (ulonglong *)0x142344af6:
    break;
  case (ulonglong *)0x142344b08:
code_r0x000142344b08:
    DAT_142925870 = DAT_142925870 + (char)puVar17 + bVar55;
    break;
  case (ulonglong *)0x142344b1f:
    pdVar35 = (double *)
              CONCAT71((int7)((ulonglong)pdVar35 >> 8),
                       (char)pdVar35 + (char)((ulonglong)pdVar33 >> 8));
    break;
  case (ulonglong *)0x142344d53:
    goto code_r0x000142344d57;
  case (ulonglong *)0x142344d5a:
    puVar21 = (ulonglong *)((ulonglong)param_12 & 0xffffffff);
    param_12 = (double *)((ulonglong)puVar17 & 0xffffffff);
code_r0x000142344d5b:
    LOCK();
    *(char *)puVar21 = (char)*puVar21 + (char)puVar21;
    UNLOCK();
    *(char *)((longlong)param_12 + 0xf) =
         *(char *)((longlong)param_12 + 0xf) + (char)((ulonglong)puVar21 >> 8);
    out(uVar30,(int)puVar21);
    puVar17 = puVar21;
  case (ulonglong *)0x142344d62:
    goto code_r0x000142344d57;
  case (ulonglong *)0x142344d69:
    bVar55 = 7 < UNK_14304e75b;
    UNK_14304e75b = UNK_14304e75b - 8;
    puVar17 = (ulonglong *)CONCAT71((int7)((ulonglong)puVar17 >> 8),(char)puVar17 + -0x70 + bVar55);
    goto code_r0x000142344d57;
  }
  puVar53 = (ulonglong *)*puVar4;
  fVar2 = *(float *)(puVar53 + 2);
  dVar6 = *pdVar35;
  *(undefined **)((longlong)pdVar38 + -8) = &UNK_142344b5a;
  cVar15 = func_0x00014093fbe0(dVar6);
  if (cVar15 == '\0') {
code_r0x000142344be2:
    fVar58 = *(float *)((longlong)*param_12 + 0x58) / _DAT_1429258d8;
code_r0x000142344bf2:
    uVar65 = *(undefined4 *)((longlong)puVar4 + 0x1c);
    puVar34 = (undefined4 *)puVar4[2];
    *puVar34 = uVar65;
    fVar58 = fVar58 + *(float *)(puVar53 + 2);
  }
  else {
    *(float *)((longlong)pdVar38 + 0x20) =
         _UNK_1429a0788 * fVar69 + _UNK_1429a0788 * fVar69 + fVar58;
    *(undefined **)((longlong)pdVar38 + -8) = &UNK_142344b9a;
    FUN_14123fff0((undefined1 *)((longlong)pdVar38 + 0x30));
    *(undefined **)((longlong)pdVar38 + -8) = &UNK_142344bae;
    cVar15 = func_0x000141240140((undefined1 *)((longlong)pdVar38 + 0x30));
    if (cVar15 == '\0') goto code_r0x000142344be2;
    uVar65 = *(undefined4 *)**(undefined8 **)((longlong)pdVar38 + 0x108);
    *(undefined **)((longlong)pdVar38 + -8) = &UNK_142344bc9;
    cVar15 = func_0x00014093fa70(uVar65);
    if (cVar15 == '\0') {
      fVar58 = *(float *)((longlong)*param_12 + 0x58) / _UNK_142925900;
      goto code_r0x000142344bf2;
    }
    fVar58 = *(float *)(puVar53 + 2);
    uVar65 = *(undefined4 *)((longlong)puVar4 + 0x1c);
    puVar34 = (undefined4 *)puVar4[2];
  }
  fVar12 = _DAT_142925904;
  plVar8 = *(longlong **)((longlong)pdVar38 + 0x120);
  fVar67 = 0.0;
  if (0.0 <= fVar58) {
    fVar67 = fVar58;
  }
  fVar58 = _DAT_142925904;
  if (fVar67 <= _DAT_142925904) {
    fVar58 = fVar67;
  }
  *puVar34 = uVar65;
  *(float *)(puVar53 + 2) = fVar58;
  if ((fVar2 != 0.0) || (fVar58 <= 0.0)) {
    if ((fVar2 == _DAT_142925904) && ((!NAN(fVar2) && !NAN(_DAT_142925904) && (fVar58 < fVar12)))) {
      *(undefined4 *)plVar8[2] = *(undefined4 *)((longlong)plVar8 + 0x1c);
      lVar16 = *plVar8;
      lVar24 = *(longlong *)(lVar16 + 0x30);
      uVar36 = *(undefined8 *)(lVar16 + 0x40);
      if (lVar24 == *(longlong *)(lVar16 + 0x20)) {
        *(undefined **)((longlong)pdVar38 + -8) = &UNK_142344cb3;
        FUN_140562c30(lVar16 + 0x20,&UNK_142bb5270);
      }
      lVar52 = *(longlong *)(lVar16 + 0x28);
      lVar29 = lVar24 * 0x20;
      *(undefined4 *)(lVar52 + lVar29) = 0x11;
      goto code_r0x000142344cc5;
    }
  }
  else {
    *(undefined4 *)plVar8[2] = *(undefined4 *)((longlong)plVar8 + 0x1c);
    lVar16 = *plVar8;
    lVar24 = *(longlong *)(lVar16 + 0x30);
    uVar36 = *(undefined8 *)(lVar16 + 0x40);
    if (lVar24 == *(longlong *)(lVar16 + 0x20)) {
      *(undefined **)((longlong)pdVar38 + -8) = &UNK_142344c64;
      FUN_140562c30(lVar16 + 0x20,&UNK_142bb5270);
    }
    lVar52 = *(longlong *)(lVar16 + 0x28);
    lVar29 = lVar24 * 0x20;
    *(undefined4 *)(lVar52 + lVar29) = 0x10;
code_r0x000142344cc5:
    *(undefined8 *)(lVar52 + 0x18 + lVar29) = uVar36;
    *(longlong *)(lVar16 + 0x30) = lVar24 + 1;
    *(longlong *)(lVar16 + 0x40) = *(longlong *)(lVar16 + 0x40) + 1;
  }
LAB_142344cd5:
  fVar69 = fVar69 * _DAT_14292c76c;
  *(undefined8 *)((longlong)pdVar38 + -8) = 0x142344cf1;
  auVar60._0_8_ = FUN_140c95ef0(&UNK_14304e668);
  auVar60._8_8_ = extraout_XMM0_Qb_02;
  auVar59._4_12_ = auVar60._4_12_;
  auVar59._0_4_ = (float)auVar60._0_8_ * fVar69;
  **(uint **)(puVar19 + 4) = puVar19[7];
  auVar62._8_8_ = 0;
  auVar62._0_8_ = *puVar53;
  puVar17 = *(ulonglong **)(*(longlong *)puVar19 + 0xa0);
  pdVar26 = *(double **)(*(longlong *)puVar19 + 0xa8);
code_r0x000142344d17:
  pdVar33 = (double *)((longlong)pdVar26 * 0x28);
code_r0x000142344d1f:
  pdVar26 = (double *)0x0;
  puVar45 = puVar17;
LAB_142344d30:
  do {
    uVar47 = SUB84(param_4,0);
    if (pdVar33 == pdVar26) {
      return;
    }
    puVar53 = puVar45 + 2;
    puVar45 = puVar45 + 5;
    uVar36 = CONCAT44(-(uint)((int)(*puVar53 >> 0x20) == auVar62._4_4_),
                      -(uint)((int)*puVar53 == auVar62._0_4_));
code_r0x000142344d43:
    auVar64._0_4_ = (undefined4)uVar36;
    uVar65 = (undefined4)((ulonglong)uVar36 >> 0x20);
    auVar64._4_4_ = auVar64._0_4_;
    auVar64._8_4_ = uVar65;
    auVar64._12_4_ = uVar65;
    uVar48 = movmskpd(uVar47,auVar64);
    param_4 = (ulonglong *)(ulonglong)uVar48;
    pdVar26 = pdVar26 + 5;
code_r0x000142344d55:
  } while (uVar48 != 3);
code_r0x000142344d57:
  uVar48 = auVar59._0_4_;
  uVar65 = 0;
  uVar71 = auVar59._8_4_;
  uVar72 = auVar59._12_4_;
                    /* WARNING (jumptable): Sanity check requires truncation of jumptable */
                    /* WARNING: Could not find normalized switch variable to match jumptable */
  switch((char)param_14[0x1e]) {
  case '\0':
    auVar59._0_4_ = (float)(uVar48 ^ _UNK_1429258f0);
    auVar59._4_4_ = 0;
    auVar59._8_4_ = uVar71 ^ _UNK_1429258f8;
    auVar59._12_4_ = uVar72 ^ _UNK_1429258fc;
  case '\x03':
    uVar65 = auVar59._0_4_;
    uVar48 = 0;
    break;
  case '\x02':
    uVar48 = uVar48 ^ _UNK_1429258f0;
  case '\x01':
  }
  *(uint *)((longlong)(puVar17 + -4) + (longlong)pdVar26) = uVar48;
  *(undefined4 *)((longlong)((longlong)puVar17 + -0x1c) + (longlong)pdVar26) = uVar65;
  return;
}

