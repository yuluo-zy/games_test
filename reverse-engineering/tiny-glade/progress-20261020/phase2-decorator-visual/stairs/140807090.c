
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

longlong *
FUN_140807090(longlong *param_1,longlong *param_2,undefined8 param_3,longlong param_4,
             longlong param_5,char *param_6,char *param_7,float param_8,float param_9,float param_10
             ,float param_11,float param_12,float param_13,float param_14,float param_15,
             undefined8 param_16,ulonglong *param_17,ulonglong *param_18,ulonglong *param_19,
             byte *param_20,longlong *param_21,longlong *param_22)

{
  short *psVar1;
  float *pfVar2;
  char *pcVar3;
  float *pfVar4;
  float fVar5;
  float fVar6;
  float fVar7;
  float fVar8;
  float fVar9;
  undefined8 uVar10;
  byte bVar11;
  code *pcVar12;
  uint uVar13;
  uint uVar14;
  float fVar15;
  float fVar16;
  float fVar17;
  float fVar18;
  longlong lVar19;
  longlong lVar20;
  longlong lVar21;
  longlong lVar22;
  longlong lVar23;
  undefined4 uVar24;
  int ******ppppppiVar25;
  undefined4 *puVar26;
  byte bVar27;
  float *pfVar28;
  int *******pppppppiVar29;
  ulonglong *puVar30;
  ulonglong uVar31;
  int *******pppppppiVar32;
  uint uVar33;
  ulonglong uVar34;
  ulonglong uVar35;
  ulonglong *puVar36;
  undefined *puVar37;
  ulonglong uVar38;
  ulonglong *puVar39;
  longlong lVar40;
  byte *pbVar41;
  ulonglong uVar42;
  undefined1 uVar43;
  ushort uVar44;
  ulonglong uVar45;
  longlong lVar46;
  ulonglong uVar47;
  int *******pppppppiVar48;
  ulonglong *puVar49;
  ulonglong *puVar50;
  ulonglong uVar51;
  bool bVar52;
  char cVar53;
  char cVar56;
  char cVar57;
  float fVar54;
  char cVar58;
  undefined1 auVar55 [16];
  float fVar59;
  float fVar60;
  float fVar61;
  undefined1 auVar62 [16];
  undefined1 auVar63 [16];
  int ******ppppppiStack_318;
  code *pcStack_310;
  int ******ppppppiStack_308;
  longlong *plStack_2f8;
  longlong *plStack_2f0;
  int ******ppppppiStack_2e8;
  longlong *plStack_2e0;
  undefined4 uStack_2d8;
  float fStack_2d4;
  float fStack_2d0;
  undefined4 uStack_2cc;
  ulonglong uStack_2c8;
  float *pfStack_2c0;
  float *pfStack_2b8;
  undefined8 uStack_2b0;
  ulonglong uStack_2a8;
  ulonglong uStack_2a0;
  float *pfStack_298;
  float *pfStack_290;
  undefined8 uStack_288;
  ulonglong uStack_280;
  ulonglong uStack_278;
  undefined8 uStack_270;
  undefined8 uStack_268;
  undefined4 *puStack_260;
  undefined4 *puStack_258;
  int ******ppppppiStack_250;
  ulonglong uStack_248;
  ulonglong uStack_240;
  int ******ppppppiStack_238;
  code *pcStack_230;
  int ******ppppppiStack_228;
  code *pcStack_220;
  int iStack_218;
  int iStack_214;
  undefined4 uStack_210;
  undefined4 uStack_20c;
  int ******ppppppiStack_208;
  ulonglong auStack_200 [2];
  ulonglong *puStack_1f0;
  int ******ppppppiStack_1e8;
  code *pcStack_1e0;
  int ******ppppppiStack_1d8;
  ulonglong uStack_1c8;
  float *pfStack_1c0;
  float *pfStack_1b8;
  int ******ppppppiStack_1b0;
  code *pcStack_1a8;
  int ******ppppppiStack_1a0;
  int ******ppppppiStack_198;
  undefined4 *puStack_190;
  float *pfStack_188;
  undefined4 *puStack_180;
  int ******ppppppiStack_178;
  ulonglong auStack_170 [4];
  undefined4 *puStack_150;
  ulonglong uStack_148;
  ulonglong uStack_140;
  float *pfStack_138;
  ulonglong uStack_130;
  ulonglong uStack_128;
  float *pfStack_120;
  ulonglong uStack_118;
  undefined1 uStack_106;
  undefined1 uStack_105;
  undefined1 uStack_104;
  undefined1 uStack_103;
  undefined1 uStack_102;
  undefined1 uStack_101;
  undefined1 uStack_100;
  undefined1 uStack_ff;
  undefined1 uStack_fe;
  undefined1 uStack_fd;
  undefined1 uStack_fc;
  undefined1 uStack_fb;
  undefined1 uStack_fa;
  undefined1 uStack_f9;
  undefined1 uStack_f8;
  undefined1 uStack_f7;
  undefined1 uStack_f6;
  undefined1 uStack_f5;
  undefined1 uStack_f4;
  undefined1 uStack_f3;
  undefined1 uStack_f2;
  undefined1 uStack_f1;
  undefined8 uStack_f0;
  
  uStack_f0 = 0xfffffffffffffffe;
  uStack_20c = *(undefined4 *)(param_4 + 0x20);
  uStack_210 = *(undefined4 *)(param_5 + 0x20);
  fVar5 = *(float *)(param_4 + 0x2c);
  fVar6 = *(float *)(param_5 + 0x2c);
  if (SBORROW8(0,*param_17)) {
    ppppppiStack_2e8 = (int ******)param_22[2];
    plStack_2f8 = param_22;
    plStack_2f0 = (longlong *)0x0;
    uStack_fb = 1;
    uStack_fc = 1;
    uStack_fd = 1;
    uStack_fe = 1;
    uStack_ff = 0;
    FUN_1412618c0(&uStack_128,&plStack_2f8,&UNK_142a8d840);
  }
  else {
    uStack_118 = param_17[2];
    uStack_128 = *param_17;
    pfStack_120 = (float *)param_17[1];
  }
  if (SBORROW8(0,*param_18)) {
    uVar51 = param_22[5];
    if (uVar51 == 0) {
      pfVar28 = (float *)0x4;
      fVar54 = _UNK_142925904;
    }
    else {
      lVar40 = param_22[4];
      pfVar28 = (float *)func_0x000140613c10(uVar51 * 4,4);
      fVar60 = _UNK_14292593c;
      fVar7 = _UNK_142925938;
      fVar61 = _UNK_142925934;
      fVar54 = _UNK_142925930;
      if (pfVar28 == (float *)0x0) {
        uStack_100 = 1;
        uStack_101 = 1;
        uStack_102 = 1;
        uStack_103 = 1;
        FUN_1428d8fe3(4,uVar51 * 4,&UNK_142a8d840);
        goto code_r0x000140808b03;
      }
      if (uVar51 < 8 || (ulonglong)((longlong)pfVar28 - lVar40) < 0x20) {
        uVar42 = 0;
      }
      else {
        uVar42 = uVar51 & 0xfffffffffffffff8;
        uVar35 = 0;
        do {
          pfVar2 = (float *)(lVar40 + uVar35 * 4);
          fVar59 = pfVar2[1];
          fVar8 = pfVar2[2];
          fVar9 = pfVar2[3];
          pfVar4 = (float *)(lVar40 + 0x10 + uVar35 * 4);
          fVar15 = *pfVar4;
          fVar16 = pfVar4[1];
          fVar17 = pfVar4[2];
          fVar18 = pfVar4[3];
          pfVar4 = pfVar28 + uVar35;
          *pfVar4 = *pfVar2 * fVar6 + (fVar54 - *pfVar2) * fVar5;
          pfVar4[1] = fVar59 * fVar6 + (fVar61 - fVar59) * fVar5;
          pfVar4[2] = fVar8 * fVar6 + (fVar7 - fVar8) * fVar5;
          pfVar4[3] = fVar9 * fVar6 + (fVar60 - fVar9) * fVar5;
          pfVar2 = pfVar28 + uVar35 + 4;
          *pfVar2 = fVar15 * fVar6 + (fVar54 - fVar15) * fVar5;
          pfVar2[1] = fVar16 * fVar6 + (fVar61 - fVar16) * fVar5;
          pfVar2[2] = fVar17 * fVar6 + (fVar7 - fVar17) * fVar5;
          pfVar2[3] = fVar18 * fVar6 + (fVar60 - fVar18) * fVar5;
          uVar35 = uVar35 + 8;
        } while (uVar42 != uVar35);
        fVar54 = _UNK_142925904;
        if (uVar51 == uVar42) goto code_r0x000140807347;
      }
      uVar35 = uVar42 | 1;
      fVar54 = _UNK_142925904;
      if ((uVar51 & 1) != 0) {
        fVar54 = *(float *)(lVar40 + uVar42 * 4);
        pfVar28[uVar42] = fVar54 * fVar6 + (_UNK_142925904 - fVar54) * fVar5;
        uVar42 = uVar35;
        fVar54 = _UNK_142925904;
      }
      while (fVar61 = _UNK_142925904, uVar51 != uVar35) {
        fVar7 = *(float *)(lVar40 + uVar42 * 4);
        fVar60 = _UNK_142925904 - fVar7;
        _UNK_142925904 = fVar54;
        pfVar28[uVar42] = fVar7 * fVar6 + fVar60 * fVar5;
        fVar54 = *(float *)(lVar40 + 4 + uVar42 * 4);
        pfVar28[uVar42 + 1] = fVar54 * fVar6 + (fVar61 - fVar54) * fVar5;
        uVar35 = uVar42 + 2;
        uVar42 = uVar35;
        fVar54 = _UNK_142925904;
        _UNK_142925904 = fVar61;
      }
    }
code_r0x000140807347:
    _UNK_142925904 = fVar54;
    uStack_140 = uVar51;
    pfStack_138 = pfVar28;
    uStack_130 = uVar51;
  }
  else {
    uStack_140 = *param_18;
    pfStack_138 = (float *)param_18[1];
    uStack_130 = param_18[2];
  }
  pppppppiVar32 = (int *******)param_22[2];
  pppppppiVar48 =
       (int *******)
       ((((ulonglong)pppppppiVar32 >> 6) + 1) - (ulonglong)(((ulonglong)pppppppiVar32 & 0x3f) == 0))
  ;
  if (pppppppiVar48 == (int *******)0x0) {
    pppppppiVar29 = (int *******)0x8;
  }
  else {
    pppppppiVar29 = (int *******)func_0x000140613c10((longlong)pppppppiVar48 * 8);
    if (pppppppiVar29 == (int *******)0x0) {
      uStack_f7 = 1;
      uStack_f8 = 1;
      uStack_f9 = 1;
      uStack_fa = 1;
      FUN_1428d8fe3(8,(longlong)pppppppiVar48 * 8,&UNK_142a8d720);
      goto code_r0x000140808b03;
    }
  }
  ppppppiStack_1b0 = (int ******)pppppppiVar29;
  pcStack_1a8 = (code *)0x0;
  ppppppiStack_1a0 = (int ******)pppppppiVar48;
  ppppppiStack_208 = (int ******)pppppppiVar32;
  uVar51 = (ulonglong)((int)pppppppiVar29 * 8 & 0x38);
  ppppppiStack_318 = (int ******)(int *******)0x0;
  if (uVar51 <= (ulonglong)((longlong)pppppppiVar48 * 0x40)) {
    ppppppiStack_318 = (int ******)((longlong)pppppppiVar48 * 0x40 - uVar51);
  }
  if (ppppppiStack_318 < pppppppiVar32) {
    ppppppiStack_238 = (int ******)&ppppppiStack_208;
    pcStack_230 = FUN_140d8d6c0;
    ppppppiStack_228 = (int ******)&ppppppiStack_318;
    pcStack_220 = FUN_140d8d6c0;
    plStack_2f8 = (longlong *)&UNK_142a8d888;
    plStack_2f0 = (longlong *)0x2;
    uStack_2d8 = 0;
    fStack_2d4 = 0.0;
    ppppppiStack_2e8 = (int ******)&ppppppiStack_238;
    plStack_2e0 = (longlong *)0x2;
    FUN_1428d9390(&plStack_2f8,&UNK_142a8d990);
  }
  else {
    pcStack_1a8 = (code *)((longlong)pppppppiVar32 * 8);
    lVar40 = (ulonglong)((uVar51 + (longlong)pppppppiVar32 & 0x3f) != 0) +
             (uVar51 + (longlong)pppppppiVar32 >> 6);
    if (lVar40 != 0) {
      func_0x000142923c20((ulonglong)pppppppiVar29 & 0xfffffffffffffff8,0,lVar40 * 8);
    }
    ppppppiStack_1d8 = ppppppiStack_1a0;
    ppppppiStack_1e8 = ppppppiStack_1b0;
    pcStack_1e0 = pcStack_1a8;
    if (SBORROW8(0,*param_19)) {
      plStack_2f8 = (longlong *)param_22[1];
      uStack_2c8 = param_22[2];
      plStack_2f0 = plStack_2f8 + uStack_2c8;
      ppppppiStack_2e8 = (int ******)param_22[4];
      uStack_2a0 = param_22[5];
      plStack_2e0 = (longlong *)((longlong)ppppppiStack_2e8 + uStack_2a0 * 4);
      if (uStack_2c8 <= uStack_2a0) {
        uStack_2a0 = uStack_2c8;
      }
      uStack_2d8 = 0;
      fStack_2d4 = 0.0;
      fStack_2d0 = (float)uStack_2a0;
      uStack_2cc = (undefined4)(uStack_2a0 >> 0x20);
      pfStack_2c0 = pfStack_120;
      pfStack_2b8 = pfStack_120 + uStack_118 * 2;
      uStack_2a8 = uStack_118;
      if (uStack_2a0 <= uStack_118) {
        uStack_2a8 = uStack_2a0;
      }
      uStack_2b0 = 0;
      pfStack_298 = pfStack_138;
      pfStack_290 = pfStack_138 + uStack_130;
      uStack_280 = uStack_130;
      if (uStack_2a8 <= uStack_130) {
        uStack_280 = uStack_2a8;
      }
      uStack_288 = 0;
      uStack_270 = 0;
      puStack_260 = &uStack_20c;
      puStack_258 = &uStack_210;
      ppppppiStack_250 = (int ******)&ppppppiStack_1e8;
      uStack_104 = 1;
      uStack_105 = 1;
      uStack_106 = 1;
      uStack_278 = uStack_2a8;
      uStack_268 = param_3;
      FUN_141260990(auStack_170 + 3,&plStack_2f8,&UNK_142a8d840);
    }
    else {
      auStack_170[3] = *param_19;
      puStack_150 = (undefined4 *)param_19[1];
      uStack_148 = param_19[2];
    }
    uVar35 = uStack_118;
    uVar42 = uStack_130;
    uVar51 = uStack_148;
    pcVar12 = pcStack_1e0;
    ppppppiVar25 = ppppppiStack_1e8;
    fVar54 = _UNK_142925904;
    if (uStack_118 == 0) {
      uStack_f4 = 1;
      uStack_f5 = 1;
      uStack_f6 = 1;
      FUN_1428d9518(0,0,&UNK_142a8ded0);
    }
    else {
      fVar61 = _UNK_142925904 / SQRT(param_13 * param_13 + param_12 * param_12);
      uVar33 = (uint)((int)fVar61 - 1U < 0x7fffff ||
                     (int)ABS(fVar61) - 0x800000U < 0x7f000000 && -1 < (int)fVar61);
      *(ulonglong *)pfStack_120 =
           CONCAT44((int)(uVar33 << 0x1f) >> 0x1f,(int)(uVar33 << 0x1f) >> 0x1f) &
           CONCAT44(param_13 * fVar61,param_12 * fVar61);
      lVar40 = param_22[2];
      uVar45 = lVar40 - 1;
      if (uVar45 < uStack_118) {
        fVar54 = fVar54 / SQRT(param_15 * param_15 + param_14 * param_14);
        uVar33 = (uint)((int)fVar54 - 1U < 0x7fffff ||
                       (int)ABS(fVar54) - 0x800000U < 0x7f000000 && -1 < (int)fVar54);
        pfStack_120[lVar40 * 2 + -2] =
             (float)((int)(uVar33 << 0x1f) >> 0x1f & (uint)(param_14 * fVar54));
        (pfStack_120 + lVar40 * 2 + -2)[1] =
             (float)((int)(uVar33 << 0x1f) >> 0x1f & (uint)(param_15 * fVar54));
        if ((*param_20 & 1) == 0) {
          auStack_200[1] = 0;
          ppppppiStack_198 = (int ******)*param_22;
          pfStack_1c0 = (float *)param_22[1];
          uVar45 = param_22[2];
          *param_22 = 0;
          param_22[1] = 4;
          param_22[2] = 0;
          fVar7 = _UNK_1429cd064;
          uVar33 = _UNK_14292e980;
          fVar61 = _UNK_142925950;
          puStack_190 = (undefined4 *)uStack_128;
          pfStack_1b8 = pfStack_120;
          uStack_128 = 0;
          pfStack_120 = (float *)0x4;
          uStack_118 = 0;
          uStack_248 = uStack_140;
          pfStack_188 = pfStack_138;
          uStack_140 = 0;
          pfStack_138 = (float *)0x4;
          uStack_130 = 0;
          uStack_240 = auStack_170[3];
          puStack_180 = puStack_150;
          auStack_170[3] = 0;
          puStack_150 = (undefined4 *)0x4;
          uStack_148 = 0;
          ppppppiStack_308 = ppppppiStack_1d8;
          ppppppiStack_318 = ppppppiStack_1e8;
          pcStack_310 = pcStack_1e0;
          ppppppiStack_1e8 = (int ******)_UNK_142a8d9a8;
          pcStack_1e0 = _UNK_142a8d9b0;
          ppppppiStack_1d8 = (int ******)0x0;
          ppppppiStack_1b0 = (int ******)0x0;
          if (uVar51 < uVar45) {
            uVar45 = uVar51;
          }
          pcStack_1a8 = (code *)0x4;
          if (uVar42 < uVar45) {
            uVar45 = uVar42;
          }
          ppppppiStack_1a0 = (int ******)0x0;
          if (uVar35 < uVar45) {
            uVar45 = uVar35;
          }
          puVar36 = (ulonglong *)((ulonglong)ppppppiVar25 & 0xfffffffffffffff8);
          uVar51 = (ulonglong)((uint)pcVar12 & 7) + (ulonglong)((uint)ppppppiVar25 & 7) * 8;
          uVar31 = ((ulonglong)pcVar12 >> 3) + uVar51;
          bVar52 = false;
          puStack_1f0 = puVar36 + (uVar31 >> 6);
          uVar42 = 0;
          puVar30 = puVar36;
          uVar35 = uVar51;
          puVar50 = puVar36 + (uVar31 >> 6);
          fVar60 = 0.0;
          fVar59 = param_14 * fVar54;
          fVar54 = param_15 * fVar54;
          if (uVar45 != 0) {
            do {
              while( true ) {
                puVar39 = puStack_1f0;
                bVar27 = (byte)uVar35;
                if ((puVar30 == puVar50) && (bVar27 == ((byte)uVar31 & 0x3f)))
                goto code_r0x000140807920;
                puVar49 = (ulonglong *)
                          ((ulonglong)((uint)(uVar35 + 1 >> 3) & 0x1ffffff8) + (longlong)puVar30);
                if (puVar49 == (ulonglong *)0x0) {
                  uStack_f3 = 1;
                  FUN_1428d9310(&UNK_142a8d680);
                  goto code_r0x000140808b03;
                }
                uStack_1c8 = uVar42 + 1;
                uVar35 = (ulonglong)((byte)(uVar35 + 1) & 0x3f);
                uVar47 = 1L << bVar27 & *puVar30;
                pfVar28 = (float *)((longlong)pfStack_1c0 + uVar42 * 8);
                fVar15 = *pfVar28;
                fVar16 = pfVar28[1];
                fVar8 = (float)puStack_180[uVar42];
                fVar9 = pfStack_188[uVar42];
                uVar10 = *(undefined8 *)(pfStack_1b8 + uVar42 * 2);
                puVar30 = puVar49;
                if (bVar52) break;
code_r0x000140808239:
                puVar39 = puVar50;
                if (ppppppiStack_1a0 == ppppppiStack_1b0) {
code_r0x000140808249:
                  ppppppiVar25 = ppppppiStack_1a0;
                  uStack_f3 = 1;
                  FUN_140b81f90(&ppppppiStack_1b0,&UNK_142a8e050);
                  ppppppiStack_1a0 = ppppppiVar25;
                }
code_r0x000140808263:
                uVar42 = uStack_118;
                *(float *)(pcStack_1a8 + (longlong)ppppppiStack_1a0 * 8) = fVar15;
                *(float *)((longlong)(pcStack_1a8 + (longlong)ppppppiStack_1a0 * 8) + 4) = fVar16;
                ppppppiStack_1a0 = (int ******)((longlong)ppppppiStack_1a0 + 1);
                if (uStack_118 == uStack_128) {
                  uStack_f3 = 1;
                  FUN_140b81f90(&uStack_128,&UNK_142a8e068);
                }
                uVar34 = uStack_130;
                *(undefined8 *)(pfStack_120 + uVar42 * 2) = uVar10;
                uStack_118 = uVar42 + 1;
                if (uStack_130 == uStack_140) {
                  uStack_f3 = 1;
                  FUN_140650050(&uStack_140,&UNK_142a8e080);
                }
                uVar42 = uStack_148;
                pfStack_138[uVar34] = fVar9;
                uStack_130 = uVar34 + 1;
                if (uStack_148 == auStack_170[3]) {
                  uStack_f3 = 1;
                  FUN_140650050(auStack_170 + 3,&UNK_142a8e098);
                }
                puStack_150[uVar42] = fVar8;
                uStack_148 = uVar42 + 1;
                uStack_f3 = 1;
                bVar52 = true;
                FUN_1408037b0(&ppppppiStack_1e8,uVar47 != 0);
                uVar42 = uStack_1c8;
                puVar50 = puVar39;
                fVar60 = fVar8;
                fVar59 = fVar15;
                fVar54 = fVar16;
                if (uVar45 <= uStack_1c8) goto code_r0x000140807920;
              }
              fVar54 = (float)((uint)(fVar60 - fVar8) & uVar33) /
                       SQRT((fVar54 - fVar16) * (fVar54 - fVar16) +
                            (fVar59 - fVar15) * (fVar59 - fVar15));
              bVar27 = 3;
              if (fVar54 <= fVar7) {
                bVar27 = (fVar61 < fVar54) + 1;
              }
              pppppppiVar32 = &ppppppiStack_208;
              uVar42 = auStack_200[1];
              if (1 < auStack_200[1]) {
                pppppppiVar32 = (int *******)ppppppiStack_208;
                uVar42 = auStack_200[0];
              }
              pbVar41 = (byte *)((longlong)pppppppiVar32 + uVar42 * 4 + -4);
              if (pbVar41 == (byte *)0x0 || uVar42 == 0) {
code_r0x0001408081f0:
                puVar39 = auStack_200;
                uVar42 = auStack_200[1];
                if (auStack_200[1] < 2) {
                  uVar42 = 1;
                  puVar39 = auStack_200 + 1;
                }
                uVar34 = auStack_200[auStack_200[1] < 2];
                if (auStack_200[auStack_200[1] < 2] == uVar42) {
                  uStack_f3 = 1;
                  FUN_1428f8520(&ppppppiStack_208);
                  puVar39 = auStack_200;
                  pppppppiVar32 = (int *******)ppppppiStack_208;
                  uVar34 = auStack_200[0];
                }
                *(uint *)((longlong)pppppppiVar32 + uVar34 * 4) = bVar27 | 0x10100;
                *puVar39 = *puVar39 + 1;
                goto code_r0x000140808239;
              }
              bVar11 = *pbVar41;
              bVar52 = bVar11 < 2;
              if (1 < bVar27) {
                bVar52 = bVar11 == bVar27 && !bVar52;
              }
              puVar50 = puStack_1f0;
              if (!bVar52) goto code_r0x0001408081f0;
              if ((bVar11 == bVar27) && ((bVar11 | bVar27) < 2)) {
                if (*(char *)((longlong)pppppppiVar32 + uVar42 * 4 + -3) == '\0')
                goto code_r0x0001408081f0;
              }
              else if ((bVar11 | bVar27) < 2) goto code_r0x0001408081f0;
              if (fVar54 <= fVar7) {
                psVar1 = (short *)((longlong)pppppppiVar32 + uVar42 * 4 + -2);
                *psVar1 = *psVar1 + 1;
                if (ppppppiStack_1a0 == ppppppiStack_1b0) goto code_r0x000140808249;
                goto code_r0x000140808263;
              }
              if (((int *******)ppppppiStack_1a0 == (int *******)0x0) ||
                 ((float *)(pcStack_1a8 + (longlong)ppppppiStack_1a0 * 8 + -8) == (float *)0x0)) {
                uStack_f3 = 1;
                FUN_1428d9310(&UNK_142a8dfd8);
                goto code_r0x000140808b03;
              }
              *(float *)(pcStack_1a8 + (longlong)ppppppiStack_1a0 * 8 + -8) = fVar15;
              *(float *)(pcStack_1a8 + (longlong)ppppppiStack_1a0 * 8 + -4) = fVar16;
              if ((uStack_118 == 0) || (pfStack_120 + uStack_118 * 2 + -2 == (float *)0x0)) {
                uStack_f3 = 1;
                FUN_1428d9310(&UNK_142a8dff0);
                goto code_r0x000140808b03;
              }
              *(undefined8 *)(pfStack_120 + uStack_118 * 2 + -2) = uVar10;
              if ((uStack_130 == 0) || (pfStack_138 + (uStack_130 - 1) == (float *)0x0)) {
                uStack_f3 = 1;
                FUN_1428d9310(&UNK_142a8e008);
                goto code_r0x000140808b03;
              }
              pfStack_138[uStack_130 - 1] = fVar9;
              if ((uStack_148 == 0) || ((float *)(puStack_150 + (uStack_148 - 1)) == (float *)0x0))
              {
                uStack_f3 = 1;
                FUN_1428d9310(&UNK_142a8e020);
                goto code_r0x000140808b03;
              }
              puStack_150[uStack_148 - 1] = fVar8;
              if (pcStack_1e0 < (code *)0x8) {
                uStack_f3 = 1;
                FUN_1428d9310(&UNK_142a8e038);
                goto code_r0x000140808b03;
              }
              uVar34 = ((ulonglong)((uint)pcStack_1e0 & 7) +
                        (ulonglong)((uint)ppppppiStack_1e8 & 7) * 8 + ((ulonglong)pcStack_1e0 >> 3))
                       - 1;
              uVar38 = uVar34 >> 6;
              uVar42 = (ulonglong)ppppppiStack_1e8 & 0xfffffffffffffff8;
              uVar34 = 1L << ((byte)uVar34 & 0x3f);
              if (uVar47 == 0) {
                uVar34 = ~uVar34 & *(ulonglong *)(uVar42 + uVar38 * 8);
              }
              else {
                uVar34 = uVar34 | *(ulonglong *)(uVar42 + uVar38 * 8);
              }
              *(ulonglong *)(uVar42 + uVar38 * 8) = uVar34;
              bVar52 = true;
              uVar42 = uStack_1c8;
              fVar60 = fVar8;
              fVar59 = fVar15;
              fVar54 = fVar16;
            } while (uStack_1c8 < uVar45);
          }
code_r0x000140807920:
          ppppppiStack_228 = ppppppiStack_1a0;
          ppppppiStack_238 = ppppppiStack_1b0;
          pcStack_230 = pcStack_1a8;
          uStack_f3 = 0;
          FUN_14126a1e0(&plStack_2f8,&ppppppiStack_238,&UNK_142a8dfc0);
          if (*param_22 != 0) {
            func_0x000140613c20(param_22[1],*param_22 << 3,4);
          }
          if (param_22[3] != 0) {
            func_0x000140613c20(param_22[4],param_22[3] << 2,4);
          }
          param_22[6] = uStack_2c8;
          param_22[4] = CONCAT44(fStack_2d4,uStack_2d8);
          param_22[5] = CONCAT44(uStack_2cc,fStack_2d0);
          param_22[2] = (longlong)ppppppiStack_2e8;
          param_22[3] = (longlong)plStack_2e0;
          *param_22 = (longlong)plStack_2f8;
          param_22[1] = (longlong)plStack_2f0;
          ppppppiStack_178 = ppppppiStack_208;
          auStack_170[0] = auStack_200[0];
          auStack_170[1] = auStack_200[1];
          if (((((code *)0x7 < pcVar12) || (puVar36 != (ulonglong *)0x8)) || (uVar51 != 0)) &&
             ((int *******)ppppppiStack_308 != (int *******)0x0)) {
            func_0x000140613c20(puVar36,(longlong)ppppppiStack_308 << 3,8);
          }
          if (uStack_240 != 0) {
            func_0x000140613c20(puStack_180,uStack_240 << 2,4);
          }
          if (uStack_248 != 0) {
            func_0x000140613c20(pfStack_188,uStack_248 << 2,4);
          }
          if (puStack_190 != (undefined4 *)0x0) {
            func_0x000140613c20(pfStack_1b8,(longlong)puStack_190 << 3,4);
          }
          pfVar28 = (float *)0x0;
          if (ppppppiStack_198 != (int ******)0x0) {
            pfVar28 = (float *)((longlong)ppppppiStack_198 << 3);
            func_0x000140613c20(pfStack_1c0,pfVar28,4);
          }
        }
        else {
          auStack_170[1] = *(ulonglong *)(param_20 + 0x18);
          ppppppiStack_178 = (int ******)*(int ********)(param_20 + 8);
          auStack_170[0] = *(ulonglong *)(param_20 + 0x10);
          pfVar28 = pfStack_120;
        }
        iStack_214 = (int)pfVar28;
        if (*param_6 == '\0') {
          bVar27 = param_6[1];
          bVar11 = param_6[2];
          if ((bVar27 != 0) || ((bVar11 & 1) != 0)) {
            bVar52 = auStack_170[1] < 2;
            puVar30 = auStack_170 + 1;
            pppppppiVar32 = &ppppppiStack_178;
            if (!bVar52) {
              puVar30 = auStack_170;
              pppppppiVar32 = (int *******)ppppppiStack_178;
            }
            uVar51 = 1;
            if (!bVar52) {
              uVar51 = auStack_170[1];
            }
            uVar42 = auStack_170[bVar52];
            if (auStack_170[bVar52] == uVar51) {
              uStack_f1 = 1;
              uStack_f2 = 1;
              FUN_1428f8520(&ppppppiStack_178);
              pppppppiVar32 = (int *******)ppppppiStack_178;
              uVar42 = auStack_170[0];
              puVar30 = auStack_170;
            }
            if (uVar42 != 0) {
              func_0x000142923c10((int *)((longlong)pppppppiVar32 + 4),pppppppiVar32,uVar42 * 4);
            }
            *puVar30 = uVar42 + 1;
            *(uint *)pppppppiVar32 = (bVar27 ^ 1) + (bVar11 ^ 1) * 0x100 + 0x10000;
            ppppppiStack_2e8 = (int ******)*param_22;
            plStack_2f8 = (longlong *)param_22[1];
            plStack_2e0 = plStack_2f8 + param_22[2];
            uStack_2d8 = 1;
            fStack_2d4 = param_8 - fVar5 * (float)*(undefined8 *)(param_6 + 4);
            fStack_2d0 = param_9 - fVar5 * (float)((ulonglong)*(undefined8 *)(param_6 + 4) >> 0x20);
            uStack_f1 = 1;
            uStack_f2 = 0;
            plStack_2f0 = plStack_2f8;
            FUN_1412605c0(&ppppppiStack_238,&plStack_2f8,&UNK_142a8d840);
            uStack_f1 = 1;
            uStack_f2 = 0;
            FUN_14126a1e0(&plStack_2f8,&ppppppiStack_238,&UNK_142a8df00);
            if (param_22[3] != 0) {
              func_0x000140613c20(param_22[4],param_22[3] << 2,4);
            }
            uVar51 = uStack_148;
            uVar24 = uStack_20c;
            param_22[6] = uStack_2c8;
            param_22[4] = CONCAT44(fStack_2d4,uStack_2d8);
            param_22[5] = CONCAT44(uStack_2cc,fStack_2d0);
            param_22[2] = (longlong)ppppppiStack_2e8;
            param_22[3] = (longlong)plStack_2e0;
            *param_22 = (longlong)plStack_2f8;
            param_22[1] = (longlong)plStack_2f0;
            if (uStack_148 == auStack_170[3]) {
              uStack_f1 = 1;
              uStack_f2 = 1;
              FUN_140650050(auStack_170 + 3);
            }
            puVar26 = puStack_150;
            if (uVar51 != 0) {
              func_0x000142923c10(puStack_150 + 1,puStack_150,uVar51 * 4);
            }
            uVar42 = uStack_130;
            *puVar26 = uVar24;
            uStack_148 = uVar51 + 1;
            if (uStack_130 == uStack_140) {
              uStack_f1 = 1;
              uStack_f2 = 1;
              FUN_140650050(&uStack_140);
            }
            pfVar28 = pfStack_138;
            if (uVar42 != 0) {
              func_0x000142923c10(pfStack_138 + 1,pfStack_138,uVar42 * 4);
            }
            uVar51 = uStack_118;
            *pfVar28 = fVar5;
            uStack_130 = uVar42 + 1;
            if (uStack_118 == uStack_128) {
              uStack_f1 = 1;
              uStack_f2 = 1;
              FUN_140b81f90(&uStack_128);
            }
            pfVar28 = pfStack_120;
            if (uVar51 != 0) {
              func_0x000142923c10(pfStack_120 + 2,pfStack_120,uVar51 * 8);
            }
            *pfVar28 = param_12;
            pfVar28[1] = param_13;
            uStack_118 = uVar51 + 1;
            uStack_f1 = 1;
            uStack_f2 = 1;
            iStack_214 = 0;
            FUN_1408037b0(&ppppppiStack_1e8);
            pcVar12 = pcStack_1e0;
            ppppppiVar25 = ppppppiStack_1e8;
            if (pcStack_1e0 < (code *)0x8) {
              plStack_2f8 = (longlong *)&UNK_142a8db60;
              plStack_2f0 = (longlong *)0x1;
              ppppppiStack_2e8 = (int ******)0x8;
              plStack_2e0 = (longlong *)0x0;
              uStack_2d8 = 0;
              fStack_2d4 = 0.0;
              uStack_f1 = 1;
              uStack_f2 = 1;
              FUN_1428d9390(&plStack_2f8,&UNK_142a8dbd0);
              goto code_r0x000140808b03;
            }
            uVar51 = (ulonglong)pcStack_1e0 >> 3;
            if (uVar51 != 1) {
              plStack_2f8 = (longlong *)0x0;
              uVar35 = (ulonglong)((uint)pcStack_1e0 & 7);
              uVar42 = (uVar51 + uVar35 + (ulonglong)((uint)ppppppiStack_1e8 & 7) * 8) - 1;
              uStack_f1 = 1;
              uStack_f2 = 1;
              FUN_1408039a0(&plStack_2f8,8,
                            (ulonglong)((uint)(uVar42 >> 3) & 7) |
                            ((ulonglong)ppppppiStack_1e8 & 0xfffffffffffffff8) +
                            ((longlong)uVar42 >> 6) * 8,(uint)uVar42 & 7 | 8);
              uStack_f1 = 1;
              uStack_f2 = 1;
              FUN_1412b0ad0(ppppppiVar25,pcVar12,uVar51 - 1,1);
              uVar35 = uVar35 | 8;
              iStack_214 = (int)uVar35;
              uStack_f1 = 1;
              uStack_f2 = 1;
              FUN_1408039a0(ppppppiVar25,uVar35,&plStack_2f8,8);
            }
          }
        }
        if (*param_7 == '\0') {
          bVar27 = param_7[1];
          bVar11 = param_7[2];
          if ((bVar27 != 0) || ((bVar11 & 1) != 0)) {
            bVar52 = auStack_170[1] < 2;
            puVar30 = auStack_170 + 1;
            pppppppiVar32 = &ppppppiStack_178;
            if (!bVar52) {
              puVar30 = auStack_170;
              pppppppiVar32 = (int *******)ppppppiStack_178;
            }
            uVar51 = 1;
            if (!bVar52) {
              uVar51 = auStack_170[1];
            }
            uVar42 = auStack_170[bVar52];
            if (auStack_170[bVar52] == uVar51) {
              uStack_f1 = 1;
              uStack_f2 = 1;
              FUN_1428f8520(&ppppppiStack_178);
              puVar30 = auStack_170;
              pppppppiVar32 = (int *******)ppppppiStack_178;
              uVar42 = auStack_170[0];
            }
            *(uint *)((longlong)pppppppiVar32 + uVar42 * 4) =
                 (bVar11 ^ 1) + (bVar27 ^ 1) * 0x100 + 0x10000;
            *puVar30 = *puVar30 + 1;
            ppppppiStack_2e8 = (int ******)*param_22;
            plStack_2f8 = (longlong *)param_22[1];
            plStack_2e0 = plStack_2f8 + param_22[2];
            uStack_2d8 = 1;
            fStack_2d4 = param_10 - fVar6 * (float)*(undefined8 *)(param_7 + 4);
            fStack_2d0 = param_11 - fVar6 * (float)((ulonglong)*(undefined8 *)(param_7 + 4) >> 0x20)
            ;
            uStack_f1 = 1;
            uStack_f2 = 0;
            plStack_2f0 = plStack_2f8;
            FUN_141260220(&ppppppiStack_238,&plStack_2f8,&UNK_142a8d840);
            uStack_f1 = 1;
            uStack_f2 = 0;
            FUN_14126a1e0(&plStack_2f8,&ppppppiStack_238,&UNK_142a8df60);
            puVar37 = (undefined *)0x0;
            if (param_22[3] != 0) {
              puVar37 = (undefined *)(param_22[3] << 2);
              func_0x000140613c20(param_22[4],puVar37,4);
            }
            uVar51 = uStack_148;
            uVar24 = uStack_210;
            param_22[6] = uStack_2c8;
            param_22[4] = CONCAT44(fStack_2d4,uStack_2d8);
            param_22[5] = CONCAT44(uStack_2cc,fStack_2d0);
            param_22[2] = (longlong)ppppppiStack_2e8;
            param_22[3] = (longlong)plStack_2e0;
            *param_22 = (longlong)plStack_2f8;
            param_22[1] = (longlong)plStack_2f0;
            if (uStack_148 == auStack_170[3]) {
              uStack_f1 = 1;
              uStack_f2 = 1;
              puVar37 = &UNK_142a8df78;
              FUN_140650050(auStack_170 + 3,&UNK_142a8df78);
            }
            uVar42 = uStack_130;
            puStack_150[uVar51] = uVar24;
            uStack_148 = uVar51 + 1;
            if (uStack_130 == uStack_140) {
              uStack_f1 = 1;
              uStack_f2 = 1;
              puVar37 = &UNK_142a8df90;
              FUN_140650050(&uStack_140,&UNK_142a8df90);
            }
            uVar51 = uStack_118;
            pfStack_138[uVar42] = fVar6;
            uStack_130 = uVar42 + 1;
            if (uStack_118 == uStack_128) {
              uStack_f1 = 1;
              uStack_f2 = 1;
              puVar37 = &UNK_142a8dfa8;
              FUN_140b81f90(&uStack_128,&UNK_142a8dfa8);
            }
            pfStack_120[uVar51 * 2] = param_14;
            pfStack_120[uVar51 * 2 + 1] = param_15;
            uStack_118 = uVar51 + 1;
            uStack_f1 = 1;
            uStack_f2 = 1;
            iStack_214 = (int)CONCAT71((int7)((ulonglong)puVar37 >> 8),1);
            FUN_1408037b0(&ppppppiStack_1e8);
          }
        }
        uStack_f1 = 1;
        uStack_f2 = 1;
        iStack_218 = func_0x0001408fe5c0(param_16);
        uVar45 = uStack_118;
        uVar35 = uStack_130;
        uVar42 = uStack_148;
        uVar51 = auStack_170[0];
        uStack_2c8 = param_22[6];
        plStack_2f8 = (longlong *)*param_22;
        plStack_2f0 = (longlong *)param_22[1];
        ppppppiStack_2e8 = (int ******)param_22[2];
        plStack_2e0 = (longlong *)param_22[3];
        uStack_2d8 = (undefined4)param_22[4];
        fStack_2d4 = (float)((ulonglong)param_22[4] >> 0x20);
        fStack_2d0 = (float)param_22[5];
        uStack_2cc = (undefined4)((ulonglong)param_22[5] >> 0x20);
        pfStack_1c0 = pfStack_138;
        uStack_1c8 = uStack_128;
        pfStack_1b8 = pfStack_120;
        ppppppiStack_228 = ppppppiStack_1d8;
        ppppppiStack_238 = ppppppiStack_1e8;
        pcStack_230 = pcStack_1e0;
        if (param_21[3] == 0) {
          uVar43 = 0;
          puStack_190 = puStack_150;
          puStack_180 = (undefined4 *)auStack_170[1];
          ppppppiStack_198 = ppppppiStack_178;
          pfStack_188 = (float *)uStack_140;
          puStack_1f0 = (ulonglong *)auStack_170[3];
        }
        else {
          pfStack_188 = (float *)uStack_140;
          puStack_190 = puStack_150;
          puStack_1f0 = (ulonglong *)auStack_170[3];
          ppppppiStack_198 = ppppppiStack_178;
          puStack_180 = (undefined4 *)auStack_170[1];
          uVar31 = FUN_1405b1be0(param_21 + 4,&iStack_218);
          bVar27 = (byte)(uVar31 >> 0x39);
          auVar55 = ZEXT216(CONCAT11(bVar27,bVar27));
          auVar55 = pshuflw(auVar55,auVar55,0);
          lVar40 = 0;
          while( true ) {
            uVar31 = param_21[1] & uVar31;
            pcVar3 = (char *)(*param_21 + uVar31);
            cVar53 = auVar55[0];
            auVar63[0] = -(*pcVar3 == cVar53);
            cVar56 = auVar55[1];
            auVar63[1] = -(pcVar3[1] == cVar56);
            cVar57 = auVar55[2];
            auVar63[2] = -(pcVar3[2] == cVar57);
            cVar58 = auVar55[3];
            auVar63[3] = -(pcVar3[3] == cVar58);
            auVar63[4] = -(pcVar3[4] == cVar53);
            auVar63[5] = -(pcVar3[5] == cVar56);
            auVar63[6] = -(pcVar3[6] == cVar57);
            auVar63[7] = -(pcVar3[7] == cVar58);
            auVar63[8] = -(pcVar3[8] == cVar53);
            auVar63[9] = -(pcVar3[9] == cVar56);
            auVar63[10] = -(pcVar3[10] == cVar57);
            auVar63[0xb] = -(pcVar3[0xb] == cVar58);
            auVar63[0xc] = -(pcVar3[0xc] == cVar53);
            auVar63[0xd] = -(pcVar3[0xd] == cVar56);
            auVar63[0xe] = -(pcVar3[0xe] == cVar57);
            auVar63[0xf] = -(pcVar3[0xf] == cVar58);
            uVar44 = (ushort)(SUB161(auVar63 >> 7,0) & 1) |
                     (ushort)(SUB161(auVar63 >> 0xf,0) & 1) << 1 |
                     (ushort)(SUB161(auVar63 >> 0x17,0) & 1) << 2 |
                     (ushort)(SUB161(auVar63 >> 0x1f,0) & 1) << 3 |
                     (ushort)(SUB161(auVar63 >> 0x27,0) & 1) << 4 |
                     (ushort)(SUB161(auVar63 >> 0x2f,0) & 1) << 5 |
                     (ushort)(SUB161(auVar63 >> 0x37,0) & 1) << 6 |
                     (ushort)(SUB161(auVar63 >> 0x3f,0) & 1) << 7 |
                     (ushort)(SUB161(auVar63 >> 0x47,0) & 1) << 8 |
                     (ushort)(SUB161(auVar63 >> 0x4f,0) & 1) << 9 |
                     (ushort)(SUB161(auVar63 >> 0x57,0) & 1) << 10 |
                     (ushort)(SUB161(auVar63 >> 0x5f,0) & 1) << 0xb |
                     (ushort)(SUB161(auVar63 >> 0x67,0) & 1) << 0xc |
                     (ushort)(SUB161(auVar63 >> 0x6f,0) & 1) << 0xd |
                     (ushort)(SUB161(auVar63 >> 0x77,0) & 1) << 0xe |
                     (ushort)(auVar63[0xf] >> 7) << 0xf;
            uVar33 = (uint)uVar44;
            while (uVar44 != 0) {
              uVar13 = 0;
              for (uVar14 = uVar33; (uVar14 & 1) == 0; uVar14 = uVar14 >> 1 | 0x80000000) {
                uVar13 = uVar13 + 1;
              }
              lVar46 = *param_21 + (uVar13 + uVar31 & param_21[1]) * -8;
              if ((iStack_218 == *(int *)(lVar46 + -8)) &&
                 (uVar43 = 1, iStack_214 == *(int *)(lVar46 + -4))) goto code_r0x000140808786;
              uVar44 = (ushort)(uVar33 - 1) & (ushort)uVar33;
              uVar33 = CONCAT22((short)(uVar33 - 1 >> 0x10),uVar44);
            }
            auVar62[0] = -(*pcVar3 == -1);
            auVar62[1] = -(pcVar3[1] == -1);
            auVar62[2] = -(pcVar3[2] == -1);
            auVar62[3] = -(pcVar3[3] == -1);
            auVar62[4] = -(pcVar3[4] == -1);
            auVar62[5] = -(pcVar3[5] == -1);
            auVar62[6] = -(pcVar3[6] == -1);
            auVar62[7] = -(pcVar3[7] == -1);
            auVar62[8] = -(pcVar3[8] == -1);
            auVar62[9] = -(pcVar3[9] == -1);
            auVar62[10] = -(pcVar3[10] == -1);
            auVar62[0xb] = -(pcVar3[0xb] == -1);
            auVar62[0xc] = -(pcVar3[0xc] == -1);
            auVar62[0xd] = -(pcVar3[0xd] == -1);
            auVar62[0xe] = -(pcVar3[0xe] == -1);
            auVar62[0xf] = -(pcVar3[0xf] == -1);
            if ((((((((((((((((SUB161(auVar62 >> 7,0) & 1) != 0 ||
                             (SUB161(auVar62 >> 0xf,0) & 1) != 0) ||
                            (SUB161(auVar62 >> 0x17,0) & 1) != 0) ||
                           (SUB161(auVar62 >> 0x1f,0) & 1) != 0) ||
                          (SUB161(auVar62 >> 0x27,0) & 1) != 0) ||
                         (SUB161(auVar62 >> 0x2f,0) & 1) != 0) ||
                        (SUB161(auVar62 >> 0x37,0) & 1) != 0) ||
                       (SUB161(auVar62 >> 0x3f,0) & 1) != 0) || (SUB161(auVar62 >> 0x47,0) & 1) != 0
                      ) || (SUB161(auVar62 >> 0x4f,0) & 1) != 0) ||
                    (SUB161(auVar62 >> 0x57,0) & 1) != 0) || (SUB161(auVar62 >> 0x5f,0) & 1) != 0)
                  || (SUB161(auVar62 >> 0x67,0) & 1) != 0) || (SUB161(auVar62 >> 0x6f,0) & 1) != 0)
                || (SUB161(auVar62 >> 0x77,0) & 1) != 0) || auVar62[0xf] < '\0') break;
            uVar31 = uVar31 + lVar40 + 0x10;
            lVar40 = lVar40 + 0x10;
          }
          uVar43 = 0;
        }
code_r0x000140808786:
        lVar40 = *param_2;
        lVar46 = param_2[1];
        lVar19 = param_2[2];
        lVar20 = param_2[3];
        lVar21 = param_2[4];
        lVar22 = param_2[5];
        lVar23 = param_2[7];
        param_1[0x20] = param_2[6];
        param_1[0x21] = lVar23;
        param_1[0x1e] = lVar21;
        param_1[0x1f] = lVar22;
        param_1[0x1c] = lVar19;
        param_1[0x1d] = lVar20;
        param_1[0x1a] = lVar40;
        param_1[0x1b] = lVar46;
        *(int *)(param_1 + 0x25) = iStack_218;
        *(int *)((longlong)param_1 + 300) = iStack_214;
        param_1[6] = uStack_2c8;
        *param_1 = (longlong)plStack_2f8;
        param_1[1] = (longlong)plStack_2f0;
        param_1[2] = (longlong)ppppppiStack_2e8;
        param_1[3] = (longlong)plStack_2e0;
        param_1[4] = CONCAT44(fStack_2d4,uStack_2d8);
        param_1[5] = CONCAT44(uStack_2cc,fStack_2d0);
        param_1[7] = (longlong)puStack_1f0;
        param_1[8] = (longlong)puStack_190;
        param_1[9] = uVar42;
        param_1[10] = (longlong)pfStack_188;
        param_1[0xb] = (longlong)pfStack_1c0;
        param_1[0xc] = uVar35;
        param_1[0xd] = uStack_1c8;
        param_1[0xe] = (longlong)pfStack_1b8;
        param_1[0xf] = uVar45;
        param_1[0x24] = (longlong)ppppppiStack_228;
        param_1[0x22] = (longlong)ppppppiStack_238;
        param_1[0x23] = (longlong)pcStack_230;
        param_1[0x26] = (longlong)ppppppiStack_198;
        param_1[0x27] = uVar51;
        param_1[0x28] = (longlong)puStack_180;
        *(undefined1 *)(param_1 + 0x29) = uVar43;
        param_1[0x13] = -0x8000000000000000;
        param_1[0x10] = 0;
        param_1[0x11] = 4;
        param_1[0x12] = 0;
        return param_1;
      }
      uStack_f4 = 1;
      uStack_f5 = 1;
      uStack_f6 = 1;
      FUN_1428d9518(uVar45,uStack_118,&UNK_142a8dee8);
    }
  }
code_r0x000140808b03:
                    /* WARNING: Does not return */
  pcVar12 = (code *)invalidInstructionException();
  (*pcVar12)();
}

