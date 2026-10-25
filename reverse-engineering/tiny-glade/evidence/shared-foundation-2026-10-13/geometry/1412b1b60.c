utils::geometry::rectangle::Rectangle2d::circular_slice_segment_points

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void FUN_1412b1b60(undefined8 param_1,float param_2,float param_3,undefined8 *param_4)

{
  undefined8 uVar1;
  byte *pbVar2;
  char *pcVar3;
  undefined8 uVar4;
  float *pfVar5;
  code *pcVar6;
  undefined4 *puVar7;
  longlong lVar8;
  longlong lVar9;
  float *pfVar10;
  undefined1 uVar11;
  undefined4 *puVar12;
  float *pfVar13;
  bool bVar14;
  undefined4 *puVar15;
  float *pfVar16;
  bool bVar17;
  uint uVar18;
  float fVar19;
  float fVar20;
  float fVar21;
  float fVar22;
  undefined4 uVar23;
  undefined4 uVar24;
  float fVar25;
  float fVar26;
  float fVar27;
  float fVar28;
  float fStack_2a4;
  float *pfStack_2a0;
  code *pcStack_298;
  uint uStack_28c;
  undefined8 uStack_288;
  undefined4 *puStack_280;
  undefined8 uStack_278;
  float **ppfStack_270;
  undefined8 uStack_268;
  undefined8 uStack_260;
  undefined4 uStack_250;
  float fStack_24c;
  undefined4 uStack_248;
  float fStack_244;
  undefined4 uStack_240;
  float fStack_23c;
  undefined4 uStack_238;
  float fStack_234;
  ulonglong uStack_230;
  longlong lStack_228;
  longlong lStack_220;
  undefined4 *apuStack_218 [5];
  undefined4 *puStack_1f0;
  undefined4 *puStack_1e8;
  longlong lStack_1e0;
  longlong lStack_1d8;
  float afStack_1d0 [4];
  undefined4 uStack_1c0;
  undefined8 uStack_1bc;
  float fStack_1b4;
  undefined4 uStack_1b0;
  undefined8 uStack_1a8;
  undefined8 uStack_1a0;
  undefined8 uStack_198;
  undefined4 *apuStack_190 [4];
  undefined4 *puStack_170;
  undefined4 *puStack_168;
  undefined4 *puStack_160;
  undefined4 *puStack_158;
  undefined4 *puStack_150;
  undefined4 *puStack_148;
  longlong lStack_140;
  longlong lStack_138;
  float *pfStack_130;
  float *pfStack_128;
  undefined8 uStack_120;
  undefined8 uStack_118;
  undefined8 uStack_110;
  undefined4 auStack_108 [50];
  
  fVar25 = _DAT_142925904;
  uVar18 = (uint)(param_3 + _DAT_142925904) & -(uint)(param_3 < param_2);
  fVar28 = (float)(~-(uint)(param_3 < param_2) & (uint)param_3 | uVar18);
  fVar22 = param_2;
  fVar19 = (float)FUN_140c996d0(uVar18,1);
  fVar20 = (float)FUN_140c996d0(param_1,2);
  fVar21 = (float)FUN_140c996d0(param_1,3);
  uStack_250 = FUN_140c9a5a0(param_1,0);
  fStack_24c = fVar22;
  uStack_248 = FUN_140c9a5a0(param_1,1);
  fStack_244 = fVar22;
  uStack_240 = FUN_140c9a5a0(param_1,2);
  fStack_23c = fVar22;
  uStack_238 = FUN_140c9a5a0(param_1,3);
  uStack_288 = param_1;
  fStack_234 = fVar22;
  FUN_140c98ab0(auStack_108,param_1);
  fVar22 = fVar28 - param_2;
  if (fVar28 - param_2 <= _DAT_142abaa00) {
    fVar22 = _DAT_142abaa00;
  }
  uStack_1a8 = 0;
  uStack_1a0 = 0;
  uStack_198 = 0;
  uStack_120 = 0;
  uStack_118 = 0;
  uStack_110 = 0;
  uStack_230 = 1;
  lStack_228 = 0;
  lStack_220 = 3;
  apuStack_218[0] = auStack_108;
  apuStack_218[1] = auStack_108;
  apuStack_218[2] = auStack_108;
  apuStack_218[3] = (undefined4 *)0x0;
  puStack_1f0 = (undefined4 *)0x0;
  lStack_1e0 = 0;
  lStack_1d8 = 9;
  afStack_1d0[0] = 0.0;
  afStack_1d0[1] = fVar19;
  afStack_1d0[2] = fVar20;
  afStack_1d0[3] = fVar21;
  uStack_1c0 = 0x3f800000;
  uStack_1bc = CONCAT44(fVar20 + _UNK_14292e6b4,fVar19 + _DAT_14292e6b0);
  fStack_1b4 = fVar21 + fVar25;
  uStack_1b0 = 0x40000000;
  apuStack_190[0] = &uStack_238;
  apuStack_190[1] = &uStack_250;
  apuStack_190[2] = &uStack_248;
  apuStack_190[3] = &uStack_240;
  puStack_170 = &uStack_238;
  puStack_168 = &uStack_250;
  puStack_160 = &uStack_248;
  puStack_158 = &uStack_240;
  puStack_150 = &uStack_238;
  puStack_148 = &uStack_250;
  lStack_140 = 0;
  lStack_138 = 10;
  pfStack_130 = (float *)0x0;
  pbVar2 = (byte *)*param_4;
  pcVar3 = (char *)param_4[1];
  uVar4 = param_4[2];
  pfVar5 = (float *)param_4[3];
  uStack_28c = -(uint)(0.0 <= param_2 && param_2 <= fVar25);
  bVar14 = false;
LAB_1412b1eca:
  do {
    pfVar10 = pfStack_128;
    lVar9 = lStack_140;
    lVar8 = lStack_1e0;
    puVar7 = puStack_1f0;
    puVar12 = apuStack_218[3];
    if (lStack_1d8 == lStack_1e0) {
      return;
    }
    lStack_1e0 = lStack_1e0 + 1;
    fVar19 = afStack_1d0[lVar8];
    if ((uStack_230 & 1) == 0) {
      if (apuStack_218[3] == (undefined4 *)0x0) goto LAB_1412b1fa0;
      bVar17 = apuStack_218[3] != apuStack_218[4];
      puVar15 = (undefined4 *)0x0;
      if (bVar17) {
        puVar15 = apuStack_218[3] + 2;
      }
      apuStack_218[3] = puVar15;
      if (!bVar17) goto LAB_1412b1fa0;
    }
    else {
      while( true ) {
        apuStack_218[3] = puVar12;
        if (puVar12 != (undefined4 *)0x0) {
          apuStack_218[3] = (undefined4 *)0x0;
          if (puVar12 != apuStack_218[4]) {
            apuStack_218[3] = puVar12 + 2;
          }
          if (puVar12 != apuStack_218[4]) goto LAB_1412b1fd5;
        }
        if (lStack_220 == lStack_228) break;
        apuStack_218[3] = apuStack_218[lStack_228];
        apuStack_218[4] = apuStack_218[3] + 8;
        lStack_228 = lStack_228 + 1;
        puVar12 = apuStack_218[3];
      }
LAB_1412b1fa0:
      if (puStack_1f0 == (undefined4 *)0x0) {
        return;
      }
      bVar17 = puStack_1f0 == puStack_1e8;
      puStack_1f0 = puStack_1f0 + 2;
      if (bVar17) {
        puStack_1f0 = (undefined4 *)0x0;
      }
      puVar12 = puVar7;
      if (bVar17) {
        return;
      }
    }
LAB_1412b1fd5:
    if (lStack_138 == lStack_140) {
      return;
    }
    lStack_140 = lStack_140 + 1;
    pfVar16 = (float *)apuStack_190[lVar9];
    if (pfStack_130 == (float *)0x0) {
      if (lStack_138 == lStack_140) {
        pfStack_130 = (float *)0x0;
        pfStack_128 = (float *)0x0;
        return;
      }
      lStack_140 = lVar9 + 2;
      pfVar13 = (float *)apuStack_190[lVar9 + 1];
      pfStack_130 = pfVar16;
      pfStack_128 = pfVar13;
    }
    else {
      pfStack_128 = pfVar16;
      pfStack_130 = pfVar10;
      pfVar13 = pfVar16;
      pfVar16 = pfVar10;
    }
    if (puVar12 == (undefined4 *)0x0) {
      return;
    }
    pfStack_2a0 = pfVar13;
    puStack_280 = puVar12;
    pfStack_130 = pfVar16;
    pfStack_128 = pfVar13;
    if (param_2 < fVar19) {
      if (!bVar14) {
        fStack_2a4 = param_2;
        if ((uStack_28c & 1) == 0) goto LAB_1412b24c9;
        if (NAN(param_2)) goto LAB_1412b2527;
        fVar20 = param_2;
        uVar23 = FUN_140c99800(uStack_288,param_2);
        uVar1 = *(undefined8 *)pfVar16;
        fVar21 = (float)uVar1;
        fVar26 = (float)((ulonglong)uVar1 >> 0x20);
        if (*pbVar2 == 0) {
          FUN_1412b2540(uVar4,uVar23,fVar20,fVar21,fVar26,0,5);
          *pbVar2 = 1;
          *pcVar3 = '\x01';
          *(undefined8 *)pfVar5 = uVar1;
        }
        else {
          uVar11 = 5;
          if (*pcVar3 == '\x01') {
            FUN_1412b2540(uVar4,uVar23,fVar20,*pfVar5 + fVar21,pfVar5[1] + fVar26,0,4);
            uVar11 = 4;
          }
          FUN_1412b2540(uVar4,uVar23,fVar20,fVar21,fVar26,0,uVar11);
          *pbVar2 = 0;
        }
      }
      if (fVar28 < fVar19) goto LAB_1412b2369;
      bVar14 = true;
      if (fVar28 <= fVar19) goto LAB_1412b1eca;
      fVar21 = fVar19 * (fVar25 / fVar22) - (fVar25 / fVar22) * param_2;
      uVar23 = *puVar12;
      uVar24 = puVar12[1];
      uVar1 = *(undefined8 *)pfVar16;
      fVar19 = (float)uVar1;
      fVar20 = (float)((ulonglong)uVar1 >> 0x20);
      if (*pbVar2 == 0) {
        FUN_1412b2540(uVar4,uVar23,uVar24,fVar19,fVar20,fVar21,5);
        *pbVar2 = 1;
        *pcVar3 = '\x01';
        *(undefined8 *)pfVar5 = uVar1;
        uVar23 = *puVar12;
        uVar24 = puVar12[1];
        fVar26 = (float)*(undefined8 *)pfVar13;
        fVar27 = (float)((ulonglong)*(undefined8 *)pfVar13 >> 0x20);
        if ((*pbVar2 & 1) != 0) {
          uVar11 = 5;
          if (*pcVar3 == '\x01') {
            FUN_1412b2540(uVar4,uVar23,uVar24,fVar19 + fVar26,fVar20 + fVar27,fVar21,4);
            uVar11 = 4;
          }
          FUN_1412b2540(uVar4,uVar23,uVar24,fVar26,fVar27,fVar21,uVar11);
          *pbVar2 = 0;
          goto LAB_1412b1eca;
        }
      }
      else {
        uVar11 = 5;
        if (*pcVar3 == '\x01') {
          FUN_1412b2540(uVar4,uVar23,uVar24,*pfVar5 + fVar19,fVar20 + pfVar5[1],fVar21,4);
          uVar11 = 4;
        }
        FUN_1412b2540(uVar4,uVar23,uVar24,fVar19,fVar20,fVar21,uVar11);
        *pbVar2 = 0;
        uVar23 = *puVar12;
        uVar24 = puVar12[1];
        fVar26 = (float)*(undefined8 *)pfVar13;
        fVar27 = (float)((ulonglong)*(undefined8 *)pfVar13 >> 0x20);
      }
      FUN_1412b2540(uVar4,uVar23,uVar24,fVar26,fVar27,fVar21,5);
      *pbVar2 = 1;
      *pcVar3 = '\x01';
      *(ulonglong *)pfVar5 = CONCAT44(fVar27,fVar26);
      goto LAB_1412b1eca;
    }
    if (fVar28 < fVar19) {
LAB_1412b2369:
      fStack_2a4 = param_3;
      if ((param_3 < 0.0) || (_DAT_142925904 < param_3)) {
LAB_1412b24c9:
        pfStack_2a0 = &fStack_2a4;
        pcStack_298 = FUN_140d8c910;
        puStack_280 = (undefined4 *)&UNK_142c236d0;
        uStack_278 = 1;
        uStack_260 = 0;
        ppfStack_270 = &pfStack_2a0;
        uStack_268 = 1;
        FUN_1428d9390(&puStack_280,&UNK_142c23690);
      }
      else if (!NAN(param_3)) {
        uVar23 = FUN_140c99800(uStack_288,param_3);
        uVar1 = *(undefined8 *)pfVar16;
        fVar25 = (float)uVar1;
        fVar22 = (float)((ulonglong)uVar1 >> 0x20);
        if (*pbVar2 == 0) {
          FUN_1412b2540(uVar4,uVar23,param_3,fVar25,fVar22,0x3f800000,5);
          *pbVar2 = 1;
          *pcVar3 = '\x01';
          *(undefined8 *)pfVar5 = uVar1;
        }
        else {
          uVar11 = 5;
          if (*pcVar3 == '\x01') {
            FUN_1412b2540(uVar4,uVar23,param_3,*pfVar5 + fVar25,fVar22 + pfVar5[1],0x3f800000,4);
            uVar11 = 4;
          }
          FUN_1412b2540(uVar4,uVar23,param_3,fVar25,fVar22,0x3f800000,uVar11);
          *pbVar2 = 0;
        }
        return;
      }
LAB_1412b2527:
      FUN_1428d9430(&UNK_142c236e0,0x1d,&UNK_142c23690);
      pcVar6 = (code *)swi(3);
      (*pcVar6)();
      return;
    }
  } while( true );
}

