/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */
undefined8 *
FUN_140c95200(longlong param_1,float *param_2,undefined8 param_3,float *param_4,float *param_5,
             undefined8 param_6,undefined8 param_7,undefined8 param_8,float *param_9)
{
  uint *puVar1;
  float fVar2;
  float fVar3;
  float fVar4;
  float fVar5;
  ulonglong *puVar6;
  code *pcVar7;
  float fVar8;
  float fVar9;
  float fVar10;
  float fVar11;
  float fVar12;
  float fVar13;
  float fVar14;
  float fVar15;
  float fVar16;
  undefined8 uVar17;
  undefined8 uVar18;
  longlong lVar19;
  float *pfVar20;
  float *pfVar21;
  float *pfVar22;
  undefined8 *puVar23;
  float *pfVar24;
  float *pfVar25;
  float *pfVar26;
  ulonglong uVar27;
  float *pfVar28;
  float *unaff_RBX;
  ulonglong uVar29;
  float *pfVar30;
  ulonglong uVar31;
  longlong lVar32;
  longlong lVar33;
  ulonglong uVar34;
  longlong lVar35;
  float *unaff_R12;
  float *pfVar36;
  uint *puVar37;
  undefined8 *puVar38;
  undefined4 uVar39;
  float fVar40;
  undefined8 extraout_XMM0_Qa;
  undefined8 uVar41;
  undefined1 auVar42 [16];
  undefined1 auVar43 [16];
  float *pfVar44;
  float fVar45;
  float *pfVar46;
  float *pfVar47;
  float *pfStack_150;
  undefined8 uStack_148;
  float *pfStack_140;
  undefined8 uStack_138;
  undefined8 uStack_130;
  float *pfStack_128;
  float *pfStack_120;
  longlong lStack_118;
  float *pfStack_110;
  float *pfStack_108;
  undefined8 uStack_100;
  float *pfStack_f8;
  float *pfStack_f0;
  undefined8 uStack_cc;
  undefined8 uStack_c4;
  undefined4 uStack_bc;
  pfVar20 = param_9;
  pfVar22 = (float *)&param_9;
  uStack_148 = param_8;
  pfStack_140 = param_9;
  uStack_138 = param_6;
  uStack_130 = param_7;
  pfStack_120 = param_9;
  pfStack_150 = pfVar22;
  if ((ulonglong)param_9 >> 0x3d == 0) {
    uVar34 = (longlong)param_9 * 4;
    puVar6 = *(ulonglong **)(param_1 + 0x10);
    uVar31 = puVar6[4] & 0xfffffffffffffffc;
    if ((uVar31 < *puVar6) || (uVar31 - *puVar6 <= uVar34 && uVar34 - (uVar31 - *puVar6) != 0)) {
LAB_140c95ab0:
      pfVar26 = (float *)0x4;
      uVar31 = FUN_1428d7780(param_1,4,uVar34);
      if (uVar31 == 0) goto LAB_140c95ba9;
    }
    else {
      uVar31 = uVar31 + (longlong)param_9 * -4;
      puVar6[4] = uVar31;
      pfVar26 = param_2;
      if (uVar31 == 0) goto LAB_140c95ab0;
    }
    if (pfVar20 != (float *)0x0) {
      unaff_RBX = (float *)0x0;
      do {
        if (pfVar20 <= unaff_RBX) goto LAB_140c95bae;
        pfVar30 = (float *)((longlong)unaff_RBX + 1);
        pfVar26 = unaff_RBX;
        pfStack_128 = pfVar30;
        uVar39 = FUN_141941040(&pfStack_150);
        *(undefined4 *)(uVar31 + (longlong)unaff_RBX * 4) = uVar39;
        unaff_RBX = pfVar30;
      } while (pfVar30 != pfVar20);
    }
    puVar6 = *(ulonglong **)(param_1 + 0x10);
    uVar29 = puVar6[4] & 0xfffffffffffffffc;
    if ((uVar29 < *puVar6) || (uVar29 - *puVar6 < uVar34)) {
LAB_140c95ad1:
      pfVar26 = (float *)0x4;
      pfVar30 = (float *)FUN_1428d7780(param_1,4,uVar34);
      unaff_RBX = pfVar30;
      if (pfVar30 == (float *)0x0) goto LAB_140c95ba9;
    }
    else {
      pfVar30 = (float *)(uVar29 + (longlong)pfVar20 * -4);
      puVar6[4] = (ulonglong)pfVar30;
      if (pfVar30 == (float *)0x0) goto LAB_140c95ad1;
    }
    unaff_RBX = pfVar30;
    pfVar22 = param_5;
    if (pfVar20 != (float *)0x0) {
      lVar35 = -1;
      do {
        if (param_9 == (float *)0x0) goto LAB_140c95bfc;
        uVar29 = lVar35 + (longlong)param_9;
        if ((uVar29 | (ulonglong)param_9) >> 0x20 == 0) {
          pfVar26 = (float *)((uVar29 & 0xffffffff) % ((ulonglong)param_9 & 0xffffffff));
        }
        else {
          pfVar26 = (float *)(uVar29 % (ulonglong)param_9);
        }
        if (param_5 <= pfVar26) goto LAB_140c95c08;
        pfVar30[lVar35 + 1] = param_4[(longlong)pfVar26];
        lVar32 = lVar35 - (longlong)pfVar20;
        lVar35 = lVar35 + 1;
      } while (lVar32 != -2);
    }
    puVar6 = *(ulonglong **)(param_1 + 0x10);
    uVar29 = puVar6[4] & 0xfffffffffffffffc;
    if ((uVar29 < *puVar6) || (uVar29 - *puVar6 < uVar34)) {
LAB_140c95af2:
      pfVar26 = (float *)FUN_1428d7780(param_1,4,uVar34);
      if (pfVar26 == (float *)0x0) goto LAB_140c95ba9;
    }
    else {
      pfVar26 = (float *)(uVar29 + (longlong)pfVar20 * -4);
      puVar6[4] = (ulonglong)pfVar26;
      if (pfVar26 == (float *)0x0) goto LAB_140c95af2;
    }
    if (pfVar20 != (float *)0x0) {
      pfVar36 = (float *)((longlong)pfVar20 - 1U);
      if (param_5 < (float *)((longlong)pfVar20 - 1U)) {
        pfVar36 = param_5;
      }
      uVar29 = (longlong)pfVar36 + 1;
      if (uVar29 < 9) {
        pfVar36 = (float *)0x0;
      }
      else {
        pfVar36 = (float *)0x0;
        if ((0x1f < (ulonglong)((longlong)pfVar26 - (longlong)param_4)) &&
           (pfVar36 = (float *)0x0, 0x1f < (ulonglong)((longlong)pfVar26 - (longlong)pfVar30))) {
          uVar27 = 8;
          if ((uVar29 & 7) != 0) {
            uVar27 = (ulonglong)((uint)uVar29 & 7);
          }
          pfVar36 = (float *)(uVar29 - uVar27);
          pfVar24 = (float *)0x0;
          do {
            pfVar21 = param_4 + (longlong)pfVar24;
            pfVar44 = param_4 + (longlong)(pfVar24 + 1);
            fVar45 = *pfVar44;
            fVar2 = pfVar44[1];
            fVar40 = pfVar44[2];
            fVar3 = pfVar44[3];
            pfVar44 = pfVar30 + (longlong)pfVar24;
            pfVar25 = pfVar30 + (longlong)(pfVar24 + 1);
            fVar4 = *pfVar25;
            fVar5 = pfVar25[1];
            fVar8 = pfVar25[2];
            fVar9 = pfVar25[3];
            auVar42._0_4_ = *pfVar44 + *pfVar21 + *pfVar44 + *pfVar21;
            auVar42._4_4_ = pfVar44[1] + pfVar21[1] + pfVar44[1] + pfVar21[1];
            auVar42._8_4_ = pfVar44[2] + pfVar21[2] + pfVar44[2] + pfVar21[2];
            auVar42._12_4_ = pfVar44[3] + pfVar21[3] + pfVar44[3] + pfVar21[3];
            *(undefined1 (*) [16])(pfVar26 + (longlong)pfVar24) = auVar42;
            pfVar21 = pfVar26 + (longlong)(pfVar24 + 1);
            *pfVar21 = fVar4 + fVar45 + fVar4 + fVar45;
            pfVar21[1] = fVar5 + fVar2 + fVar5 + fVar2;
            pfVar21[2] = fVar8 + fVar40 + fVar8 + fVar40;
            pfVar21[3] = fVar9 + fVar3 + fVar9 + fVar3;
            pfVar24 = pfVar24 + 2;
          } while (pfVar36 != pfVar24);
        }
      }
      do {
        if (param_5 == pfVar36) goto LAB_140c95c1a;
        pfVar26[(longlong)pfVar36] =
             param_4[(longlong)pfVar36] + pfVar30[(longlong)pfVar36] +
             param_4[(longlong)pfVar36] + pfVar30[(longlong)pfVar36];
        pfVar36 = (float *)((longlong)pfVar36 + 1);
      } while (pfVar20 != pfVar36);
    }
    puVar6 = *(ulonglong **)(param_1 + 0x10);
    uVar29 = puVar6[4] & 0xfffffffffffffffc;
    if ((uVar29 < *puVar6) || (uVar29 - *puVar6 < uVar34)) {
LAB_140c95b1b:
      pfVar36 = (float *)FUN_1428d7780(param_1,4,uVar34);
      unaff_R12 = pfVar36;
      if (pfVar36 == (float *)0x0) goto LAB_140c95ba9;
    }
    else {
      pfVar36 = (float *)(uVar29 + (longlong)pfVar20 * -4);
      puVar6[4] = (ulonglong)pfVar36;
      if (pfVar36 == (float *)0x0) goto LAB_140c95b1b;
    }
    unaff_R12 = pfVar36;
    if (pfVar20 == (float *)0x0) goto LAB_140c95c2c;
    pfVar24 = (float *)((longlong)pfVar20 - 1);
    pfVar21 = pfVar24;
    if (param_5 < pfVar24) {
      pfVar21 = param_5;
    }
    uVar29 = (longlong)pfVar21 + 1;
    pfVar21 = (float *)0x0;
    if ((8 < uVar29) && (0x1f < (ulonglong)((longlong)pfVar36 - (longlong)param_4))) {
      uVar27 = 8;
      if ((uVar29 & 7) != 0) {
        uVar27 = (ulonglong)((uint)uVar29 & 7);
      }
      pfVar21 = (float *)(uVar29 - uVar27);
      pfVar44 = (float *)0x0;
      do {
        uVar41 = *(undefined8 *)(param_4 + (longlong)pfVar44 + 2);
        uVar17 = *(undefined8 *)(param_4 + (longlong)(pfVar44 + 1));
        uVar18 = *(undefined8 *)(param_4 + (longlong)(pfVar44 + 1) + 2);
        *(undefined8 *)(pfVar36 + (longlong)pfVar44) = *(undefined8 *)(param_4 + (longlong)pfVar44);
        *(undefined8 *)(pfVar36 + (longlong)pfVar44 + 2) = uVar41;
        *(undefined8 *)(pfVar36 + (longlong)(pfVar44 + 1)) = uVar17;
        *(undefined8 *)(pfVar36 + (longlong)(pfVar44 + 1) + 2) = uVar18;
        pfVar44 = pfVar44 + 2;
      } while (pfVar21 != pfVar44);
    }
    do {
      if (param_5 == pfVar21) goto LAB_140c95bc6;
      pfVar36[(longlong)pfVar21] = param_4[(longlong)pfVar21];
      pfVar21 = (float *)((longlong)pfVar21 + 1);
    } while (pfVar20 != pfVar21);
    fVar45 = pfVar36[(longlong)pfVar20 + -1];
    fVar2 = *pfVar26;
    fVar40 = *pfVar30;
    *pfVar26 = fVar2 + fVar2;
    pfVar26[(longlong)pfVar20 + -1] = (fVar40 * fVar45) / fVar2 + pfVar26[(longlong)pfVar20 + -1];
    puVar6 = *(ulonglong **)(param_1 + 0x10);
    uVar29 = puVar6[4] & 0xfffffffffffffffc;
    if ((uVar29 < *puVar6) || (uVar29 - *puVar6 < uVar34)) {
LAB_140c95b41:
      puVar37 = (uint *)FUN_1428d7780(param_1,4,uVar34);
      if (puVar37 == (uint *)0x0) goto LAB_140c95ba9;
    }
    else {
      puVar37 = (uint *)(uVar29 + (longlong)pfVar20 * -4);
      puVar6[4] = (ulonglong)puVar37;
      if (puVar37 == (uint *)0x0) goto LAB_140c95b41;
    }
    *puVar37 = _DAT_1429258f0 ^ (uint)fVar2;
    lVar19 = _UNK_142b2da68;
    lVar33 = _DAT_142b2da60;
    lVar32 = _UNK_1429ae328;
    lVar35 = _DAT_1429ae320;
    if (pfVar20 != (float *)0x1) {
      pfVar22 = (float *)0x1;
      if ((float *)0x7 < pfVar24) {
        pfVar25 = (float *)((ulonglong)pfVar24 & 0xfffffffffffffff8);
        pfVar22 = (float *)((longlong)pfVar25 + 1);
        pfVar28 = (float *)0x0;
        auVar43 = _DAT_142b2da50;
        pfVar21 = _DAT_142a7ba90;
        pfVar44 = _UNK_142a7ba98;
        do {
          pfVar46 = auVar43._0_8_;
          pfVar47 = auVar43._8_8_;
          puVar1 = puVar37 + (longlong)pfVar28 + 1;
          *puVar1 = -(uint)(pfVar24 == pfVar21) & (uint)fVar45;
          puVar1[1] = -(uint)(pfVar24 == pfVar44) & (uint)fVar45;
          puVar1[2] = -(uint)(pfVar24 == pfVar46) & (uint)fVar45;
          puVar1[3] = -(uint)(pfVar24 == pfVar47) & (uint)fVar45;
          puVar1 = puVar37 + (longlong)pfVar28 + 5;
          *puVar1 = -(uint)((float *)((longlong)pfVar21 + lVar35) == pfVar24) & (uint)fVar45;
          puVar1[1] = -(uint)((float *)((longlong)pfVar44 + lVar32) == pfVar24) & (uint)fVar45;
          puVar1[2] = -(uint)((float *)((longlong)pfVar46 + lVar35) == pfVar24) & (uint)fVar45;
          puVar1[3] = -(uint)((float *)((longlong)pfVar47 + lVar32) == pfVar24) & (uint)fVar45;
          pfVar28 = pfVar28 + 2;
          pfVar21 = (float *)((longlong)pfVar21 + lVar33);
          pfVar44 = (float *)((longlong)pfVar44 + lVar19);
          auVar43._0_8_ = (longlong)pfVar46 + lVar33;
          auVar43._8_8_ = (longlong)pfVar47 + lVar19;
        } while (pfVar25 != pfVar28);
        if (pfVar24 == pfVar25) goto LAB_140c95620;
      }
      do {
        fVar40 = fVar45;
        if (pfVar24 != pfVar22) {
          fVar40 = 0.0;
        }
        puVar37[(longlong)pfVar22] = (uint)fVar40;
        pfVar22 = (float *)((longlong)pfVar22 + 1);
      } while (pfVar20 != pfVar22);
    }
LAB_140c95620:
    unaff_R12 = pfVar30;
    pfVar21 = (float *)FUN_140c95c60(param_1,pfVar30,pfVar20,pfVar26,pfVar20,pfVar36,pfVar20,uVar31,
                                     pfVar20);
    pfVar26 = (float *)FUN_140c95c60(param_1,pfVar30,pfVar20,pfVar26,pfVar20,pfVar36,pfVar20,puVar37
                                     ,pfVar20);
    uVar41 = extraout_XMM0_Qa;
    if (pfVar24 < unaff_R12) {
      if (unaff_RBX <= pfVar24) goto LAB_140c95c4b;
      fVar40 = *pfVar21;
      fVar3 = *pfVar26;
      fVar4 = pfVar21[(longlong)pfVar20 + -1];
      fVar5 = pfVar26[(longlong)pfVar20 + -1];
      pfVar22 = *(float **)(param_1 + 0x10);
      uVar31 = *(ulonglong *)(pfVar22 + 8) & 0xfffffffffffffffc;
      if ((uVar31 < *(ulonglong *)pfVar22) || (uVar31 - *(longlong *)pfVar22 < uVar34)) {
LAB_140c95b67:
        lVar35 = FUN_1428d7780(param_1,4);
        if (lVar35 == 0) goto LAB_140c95ba9;
      }
      else {
        lVar35 = uVar31 + (longlong)pfVar20 * -4;
        *(longlong *)(pfVar22 + 8) = lVar35;
        if (lVar35 == 0) goto LAB_140c95b67;
      }
      if (unaff_RBX < pfVar24) {
        pfVar24 = unaff_RBX;
      }
      fVar45 = (fVar40 - (fVar4 * fVar45) / fVar2) /
               ((fVar3 + _DAT_142925904) - (fVar45 * fVar5) / fVar2);
      if (unaff_R12 <= pfVar24) {
        pfVar24 = unaff_R12;
      }
      if (pfVar20 <= pfVar24) {
        pfVar24 = pfVar20;
      }
      if (pfVar24 < (float *)0x8) {
        lVar32 = 0;
      }
      else {
        lVar32 = 0;
        if ((0x1f < (ulonglong)(lVar35 - (longlong)pfVar21)) &&
           (0x1f < (ulonglong)(lVar35 - (longlong)pfVar26))) {
          uVar31 = (longlong)pfVar24 + 1;
          uVar34 = 8;
          if ((uVar31 & 7) != 0) {
            uVar34 = (ulonglong)((uint)uVar31 & 7);
          }
          lVar32 = uVar31 - uVar34;
          lVar33 = 0;
          do {
            pfVar36 = pfVar21 + lVar33;
            fVar2 = pfVar36[1];
            fVar40 = pfVar36[2];
            fVar3 = pfVar36[3];
            pfVar22 = pfVar21 + lVar33 + 4;
            fVar4 = *pfVar22;
            fVar5 = pfVar22[1];
            fVar8 = pfVar22[2];
            fVar9 = pfVar22[3];
            pfVar22 = pfVar26 + lVar33;
            fVar10 = pfVar22[1];
            fVar11 = pfVar22[2];
            fVar12 = pfVar22[3];
            pfVar30 = pfVar26 + lVar33 + 4;
            fVar13 = *pfVar30;
            fVar14 = pfVar30[1];
            fVar15 = pfVar30[2];
            fVar16 = pfVar30[3];
            pfVar30 = (float *)(lVar35 + lVar33 * 4);
            *pfVar30 = *pfVar36 - *pfVar22 * fVar45;
            pfVar30[1] = fVar2 - fVar10 * fVar45;
            pfVar30[2] = fVar40 - fVar11 * fVar45;
            pfVar30[3] = fVar3 - fVar12 * fVar45;
            pfVar22 = (float *)(lVar35 + 0x10 + lVar33 * 4);
            *pfVar22 = fVar4 - fVar13 * fVar45;
            pfVar22[1] = fVar5 - fVar14 * fVar45;
            pfVar22[2] = fVar8 - fVar15 * fVar45;
            pfVar22[3] = fVar9 - fVar16 * fVar45;
            lVar33 = lVar33 + 8;
          } while (lVar32 != lVar33);
        }
      }
      pfVar22 = (float *)(lVar35 + lVar32 * 4);
      lVar33 = 0;
      do {
        if ((longlong)pfVar20 - lVar32 == lVar33) goto LAB_140c95bae;
        if ((longlong)unaff_R12 - lVar32 == lVar33) goto LAB_140c95bea;
        if ((longlong)unaff_RBX - lVar32 == lVar33) goto LAB_140c95bd8;
        pfVar22[lVar33] = pfVar21[lVar32 + lVar33] - pfVar26[lVar32 + lVar33] * fVar45;
        lVar33 = lVar33 + 1;
      } while ((longlong)pfVar20 - lVar32 != lVar33);
      pfStack_150 = (float *)&param_9;
      uStack_148 = param_8;
      pfStack_140 = pfVar20;
      uStack_138 = param_6;
      uStack_130 = param_7;
      pfStack_120 = param_5;
      pfStack_110 = pfVar20;
      pfStack_f0 = pfVar20;
      pfStack_128 = param_4;
      lStack_118 = lVar35;
      pfStack_108 = param_2;
      uStack_100 = param_3;
      if ((float *)0x666666666666666 < pfVar20) goto LAB_140c95ba9;
      puVar6 = *(ulonglong **)(param_1 + 0x10);
      uVar34 = puVar6[4] & 0xfffffffffffffffc;
      if ((uVar34 < *puVar6) || (uVar34 - *puVar6 < (ulonglong)((longlong)pfVar20 * 0x14))) {
LAB_140c95b90:
        puVar23 = (undefined8 *)FUN_1428d7780(param_1,4);
        if (puVar23 == (undefined8 *)0x0) goto LAB_140c95ba9;
      }
      else {
        puVar23 = (undefined8 *)(uVar34 + (longlong)pfVar20 * -0x14);
        puVar6[4] = (ulonglong)puVar23;
        if (puVar23 == (undefined8 *)0x0) goto LAB_140c95b90;
      }
      unaff_RBX = (float *)&uStack_cc;
      pfVar26 = (float *)0x0;
      puVar38 = puVar23;
      while (pfVar26 < pfVar20) {
        pfVar26 = (float *)((longlong)pfVar26 + 1);
        pfStack_f8 = pfVar26;
        FUN_141941340(unaff_RBX,&pfStack_150);
        *(undefined4 *)(puVar38 + 2) = uStack_bc;
        *puVar38 = uStack_cc;
        puVar38[1] = uStack_c4;
        puVar38 = (undefined8 *)((longlong)puVar38 + 0x14);
        unaff_R12 = pfVar26;
        if (pfVar26 == pfVar20) {
          return puVar23;
        }
      }
      goto LAB_140c95bae;
    }
  }
  else {
LAB_140c95ba9:
    FUN_1428d79d0();
LAB_140c95bae:
    FUN_1428d9330(&UNK_142b2db40,0x22,&UNK_142b2dbc0);
LAB_140c95bc6:
    FUN_1428d9518(pfVar22,pfVar22,&UNK_142e71b18);
LAB_140c95bd8:
    FUN_1428d9518(unaff_RBX,unaff_RBX,&UNK_142e71c08);
LAB_140c95bea:
    pfVar26 = unaff_R12;
    FUN_1428d9518(unaff_R12,unaff_R12,&UNK_142e71bf0);
LAB_140c95bfc:
    FUN_1428d9a20(&UNK_142e71ad0);
    pfVar30 = unaff_RBX;
    param_5 = pfVar22;
LAB_140c95c08:
    FUN_1428d9518(pfVar26,param_5,&UNK_142e71ae8);
LAB_140c95c1a:
    FUN_1428d9518(param_5,param_5,&UNK_142e71b00);
LAB_140c95c2c:
    uVar41 = FUN_1428d9518(0,0,&UNK_142b2df48);
  }
  unaff_RBX = pfVar30;
  uVar41 = FUN_1428d9518(uVar41,unaff_R12,&UNK_142b2df60);
LAB_140c95c4b:
  FUN_1428d9518(uVar41,unaff_RBX,&UNK_142b2df78);
  pcVar7 = (code *)swi(3);
  puVar23 = (undefined8 *)(*pcVar7)();
  return puVar23;
}