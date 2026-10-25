
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void FUN_1421b3120(undefined8 *param_1,undefined4 param_2,byte param_3,undefined8 param_4)

{
  code *pcVar1;
  longlong lVar2;
  undefined2 uVar3;
  int iVar4;
  uint uVar5;
  ulonglong uVar6;
  undefined8 uVar7;
  ulonglong uVar8;
  undefined4 *in_RDX;
  int iVar9;
  int iVar10;
  uint uVar11;
  ulonglong uVar12;
  undefined4 uVar13;
  float fVar14;
  float extraout_XMM0_Da;
  float extraout_XMM0_Da_00;
  float fVar15;
  float fVar16;
  float fVar17;
  float fVar18;
  float fVar19;
  float fVar20;
  float fVar21;
  float fVar22;
  undefined8 uVar23;
  float fVar24;
  float fVar25;
  float fVar26;
  float fVar27;
  float fVar28;
  float fVar29;
  float fVar30;
  float fVar31;
  float fVar32;
  float fVar33;
  float fVar34;
  float fVar35;
  float fVar36;
  undefined8 in_stack_fffffffffffffcd8;
  undefined4 uVar38;
  undefined *puVar37;
  longlong lStack_320;
  undefined8 uStack_318;
  longlong lStack_308;
  undefined8 uStack_300;
  float fStack_2e8;
  float fStack_2e4;
  float fStack_2e0;
  float fStack_2dc;
  float fStack_2d8;
  float fStack_2d4;
  float fStack_2d0;
  float fStack_2cc;
  undefined8 uStack_2c8;
  undefined8 uStack_2c0;
  undefined1 auStack_2b8 [16];
  undefined8 uStack_2a8;
  undefined8 uStack_2a0;
  float fStack_298;
  float fStack_294;
  float fStack_290;
  float fStack_28c;
  float fStack_288;
  float fStack_284;
  float fStack_280;
  float fStack_27c;
  longlong lStack_270;
  longlong lStack_268;
  ulonglong uStack_260;
  longlong lStack_258;
  undefined8 uStack_250;
  float fStack_240;
  float afStack_234 [3];
  undefined1 auStack_228 [8];
  float fStack_220;
  undefined8 uStack_218;
  float fStack_210;
  ulonglong uStack_208;
  undefined4 *puStack_200;
  undefined1 auStack_1f8 [16];
  undefined1 auStack_1e8 [16];
  undefined8 uStack_1d8;
  undefined8 uStack_1d0;
  float fStack_1bc;
  float fStack_1b8;
  undefined4 uStack_1b4;
  undefined8 uStack_1b0;
  float **ppfStack_1a8;
  code *pcStack_1a0;
  ulonglong uStack_198;
  float fStack_190;
  uint uStack_18c;
  float fStack_188;
  int iStack_184;
  float fStack_180;
  undefined4 uStack_17c;
  int iStack_178;
  int iStack_174;
  undefined8 uStack_170;
  undefined8 **ppuStack_168;
  code *pcStack_160;
  undefined4 uStack_154;
  undefined8 *puStack_150;
  ulonglong uStack_148;
  float fStack_13c;
  undefined8 uStack_138;
  ulonglong uStack_130;
  undefined8 ***pppuStack_128;
  undefined8 uStack_120;
  undefined8 uStack_118;
  undefined4 uStack_110;
  undefined8 uStack_10c;
  int iStack_104;
  undefined8 uStack_100;
  byte bStack_f1;
  undefined8 uStack_f0;
  
  uVar38 = (undefined4)((ulonglong)in_stack_fffffffffffffcd8 >> 0x20);
  uStack_f0 = 0xfffffffffffffffe;
  uStack_170 = 0;
  uStack_1b0 = param_4;
  func_0x000140a880c0(&uStack_138);
  uVar23 = FUN_140a88200(&uStack_138);
  FUN_1408e3420(&lStack_320,in_RDX);
  uVar13 = FUN_1408e3260(in_RDX);
  puVar37 = (undefined *)CONCAT44(uVar38,uVar13);
  FUN_1408e3d40(&lStack_270,&lStack_320,uVar23,param_2,puVar37);
  fVar14 = (float)func_0x000142923e60(fStack_240 / _DAT_142ff7e9c);
  if (fVar14 <= _DAT_14292e6f0) {
    fVar14 = _DAT_14292e6f0;
  }
  iStack_178 = (int)fVar14;
  if (_DAT_1429a49bc < fVar14) {
    iStack_178 = 0x7fffffff;
  }
  if (NAN(fVar14)) {
    iStack_178 = 0;
  }
  fStack_180 = fVar14 + _DAT_14295a0e4;
  uStack_2a8 = *param_1;
  uStack_2a0 = 0;
  fStack_188 = (fStack_240 / fVar14) * _DAT_142ff7ea0;
  uStack_17c = *(undefined4 *)((longlong)param_1 + 0xc);
  uStack_154 = *in_RDX;
  uStack_18c = -iStack_178;
  iVar9 = 0;
  fVar14 = _DAT_142925904;
  iVar10 = 0;
  while( true ) {
    uVar13 = (undefined4)((ulonglong)puVar37 >> 0x20);
    iVar4 = iVar10;
    if (iVar10 < iStack_178) {
      iVar4 = iStack_178;
    }
    uVar11 = -iVar10;
    do {
      if (-iVar4 == uVar11) {
        if (lStack_270 != 0) {
          func_0x000140613c20(lStack_268,lStack_270 << 3,4);
        }
        if (lStack_258 != 0) {
          func_0x000140613c20(uStack_250,lStack_258 << 2,4);
        }
        if (lStack_320 != 0) {
          func_0x000140613c20(uStack_318,lStack_320 << 3,4);
        }
        if (lStack_308 != 0) {
          func_0x000140613c20(uStack_300,lStack_308 << 2,4);
        }
        return;
      }
      uVar11 = uVar11 - 1;
    } while (uStack_18c == uVar11);
    fVar20 = (float)(int)~uVar11 / fStack_180;
    ppuStack_168 = (undefined8 **)CONCAT44(ppuStack_168._4_4_,fVar20);
    if ((fVar20 < 0.0) || (fVar14 < fVar20)) {
      ppfStack_1a8 = (float **)&ppuStack_168;
      pcStack_1a0 = FUN_140d8c910;
      uStack_138 = &UNK_142ff4538;
      uStack_130 = 1;
      uStack_118 = 0;
      pppuStack_128 = (undefined8 ***)&ppfStack_1a8;
      uStack_120 = 1;
      FUN_1428d9390(&uStack_138,&UNK_142ff8500);
      goto LAB_1421b4275;
    }
    if (NAN(fVar20)) {
      FUN_1428d9430(&UNK_142ff4548,0x1d,&UNK_142ff8500);
      goto LAB_1421b4275;
    }
    auStack_1e8 = ZEXT416((uint)fVar20);
    uVar5 = FUN_142197610(&lStack_270);
    lVar2 = lStack_268;
    uVar8 = (ulonglong)uVar5;
    if (uStack_260 <= uVar8) {
      FUN_1428d9518(uVar8,uStack_260,&UNK_142ff4438);
      goto LAB_1421b4275;
    }
    uStack_148 = uStack_260;
    if (uStack_260 <= uVar8 + 1) {
      FUN_1428d9518(uVar8 + 1,uStack_260,&UNK_142ff4450);
      goto LAB_1421b4275;
    }
    fVar20 = *(float *)(lStack_268 + uVar8 * 8);
    fVar16 = *(float *)(lStack_268 + 8 + uVar8 * 8);
    fVar17 = (float)auStack_1e8._0_4_;
    uVar5 = FUN_142197610(&lStack_270);
    uVar8 = (ulonglong)uVar5;
    if (uStack_148 <= uVar8) {
      FUN_1428d9518(uVar8,uStack_148,&UNK_142ff4438);
      goto LAB_1421b4275;
    }
    if (uStack_148 <= uVar8 + 1) {
      FUN_1428d9518(uVar8 + 1,uStack_148,&UNK_142ff4450);
      goto LAB_1421b4275;
    }
    fVar18 = *(float *)(lVar2 + 4 + uVar8 * 8);
    fVar28 = *(float *)(lVar2 + 0xc + uVar8 * 8);
    fVar15 = (float)FUN_1408e3780(in_RDX);
    uStack_138 = (undefined *)
                 CONCAT44(fVar17 * (float)auStack_1e8._0_4_ + uStack_2a8._4_4_,
                          fVar15 * (float)auStack_1e8._0_4_ + (float)uStack_2a8);
    func_0x000140c9cde0(&uStack_218,&uStack_138,
                        fVar28 * extraout_XMM0_Da_00 + (fVar14 - extraout_XMM0_Da_00) * fVar18);
    fVar20 = (fVar14 - extraout_XMM0_Da) * fVar20 + fVar16 * extraout_XMM0_Da;
    func_0x000140c97560(auStack_1f8,0,0,fVar20,CONCAT44(uVar13,uStack_17c));
    fVar16 = (float)func_0x000140c904a0(_DAT_142a7c88c,_DAT_1429258d8,fVar20);
    fVar20 = 0.0;
    if (0.0 <= fVar16) {
      fVar20 = fVar16;
    }
    fVar16 = fVar14;
    if (fVar20 <= fVar14) {
      fVar16 = fVar20;
    }
    fVar28 = fVar16 * _DAT_142925870;
    fVar17 = (float)func_0x000140c97970(auStack_1f8);
    fVar18 = (float)func_0x000140caf380(&uStack_170);
    fVar28 = fVar28 + (fVar14 - fVar16);
    fVar20 = (float)func_0x000142923e60(fVar17 / (fVar28 * _DAT_142925870));
    if (fVar20 <= _DAT_142925988) {
      fVar20 = _DAT_142925988;
    }
    iVar10 = (int)fVar20;
    if (_DAT_1429a49bc < fVar20) {
      iVar10 = 0x7fffffff;
    }
    if (NAN(fVar20)) {
      iVar10 = 0;
    }
    fVar18 = fVar18 * _DAT_142934674;
    fVar20 = (float)func_0x000142923fd0(fVar18);
    fVar16 = (float)func_0x000142923e80(fVar18);
    puVar37 = &UNK_142ff8518;
    FUN_140c90350(&ppfStack_1a8,(longlong)iVar10 + 1,(fVar28 * 0.0) / fVar17,&uStack_170,
                  &UNK_142ff8518);
    uVar8 = uStack_198;
    puStack_200 = in_RDX;
    if (uStack_198 < 2) break;
    iStack_174 = -uVar11;
    fStack_280 = (float)((uint)fVar20 ^ _DAT_1429258f0);
    uStack_208 = uStack_198 - 1;
    iStack_184 = (uVar11 == 0xffffffff | param_3) + 1;
    uVar12 = 0;
    fStack_288 = fVar16;
    fStack_284 = fVar20;
    fStack_27c = fVar16;
    bStack_f1 = param_3;
    while (uVar12 < uStack_208) {
      if (uVar8 <= uVar12) {
        FUN_1428d9518(uVar12,uVar8,&UNK_142ff8588);
        goto LAB_1421b4275;
      }
      fVar20 = *(float *)(pcStack_1a0 + uVar12 * 4);
      puStack_150 = (undefined8 *)CONCAT44(puStack_150._4_4_,fVar20);
      if ((fVar20 < 0.0) || (fVar14 < fVar20)) {
        ppuStack_168 = &puStack_150;
        pcStack_160 = FUN_140d8c910;
        uStack_138 = &UNK_142ff4538;
        uStack_130 = 1;
        uStack_118 = 0;
        pppuStack_128 = &ppuStack_168;
        uStack_120 = 1;
        FUN_1428d9390(&uStack_138,&UNK_142ff85a0);
        goto LAB_1421b4275;
      }
      if (NAN(fVar20)) {
        FUN_1428d9430(&UNK_142ff4548,0x1d,&UNK_142ff85a0);
        goto LAB_1421b4275;
      }
      uVar12 = uVar12 + 1;
      if (uVar8 <= uVar12) {
        FUN_1428d9518(uVar12,uVar8,&UNK_142ff85b8);
        goto LAB_1421b4275;
      }
      fVar16 = *(float *)(pcStack_1a0 + uVar12 * 4);
      puStack_150 = (undefined8 *)CONCAT44(puStack_150._4_4_,fVar16);
      if ((fVar16 < 0.0) || (fVar14 < fVar16)) {
        ppuStack_168 = &puStack_150;
        pcStack_160 = FUN_140d8c910;
        uStack_138 = &UNK_142ff4538;
        uStack_130 = 1;
        uStack_118 = 0;
        pppuStack_128 = &ppuStack_168;
        uStack_120 = 1;
        FUN_1428d9390(&uStack_138,&UNK_142ff85d0);
        goto LAB_1421b4275;
      }
      if (NAN(fVar16)) {
        FUN_1428d9430(&UNK_142ff4548,0x1d,&UNK_142ff85d0);
        goto LAB_1421b4275;
      }
      fVar17 = fVar20;
      fVar28 = (float)FUN_140c975a0(auStack_1f8);
      fVar18 = fVar16;
      fVar15 = (float)FUN_140c975a0(auStack_1f8);
      fVar26 = fVar17 * fStack_280 + fVar28 * fStack_288;
      fVar27 = fVar17 * fStack_27c + fVar28 * fStack_284;
      fVar28 = fVar18 * fStack_280 + fVar15 * fStack_288;
      fVar18 = fVar18 * fStack_27c + fVar15 * fStack_284;
      puStack_150 = (undefined8 *)
                    CONCAT44((fVar27 + fVar18) * _UNK_1429258e4,(fVar26 + fVar28) * _DAT_1429258e0);
      func_0x000140c9cdc0(&uStack_138,&puStack_150);
      uVar23 = uStack_218;
      uStack_2c8 = uStack_138;
      uStack_2c0 = 0;
      fStack_13c = fStack_210 + (float)uStack_130;
      fVar17 = (float)func_0x000140caf380(&uStack_170);
      fVar15 = (fVar14 - fVar17) * _DAT_142a16cb8;
      fVar17 = fVar17 * _DAT_1429cd128;
      uVar5 = FUN_142197610(&lStack_270,auStack_1e8._0_4_);
      uVar6 = (ulonglong)uVar5;
      if (uStack_148 <= uVar6 + 1) {
        FUN_1428d9518(uVar6 + 1,uStack_148,&UNK_142ff4468);
        goto LAB_1421b4275;
      }
      fVar21 = *(float *)(lVar2 + 8 + uVar6 * 8) - *(float *)(lVar2 + uVar6 * 8);
      fVar22 = *(float *)(lVar2 + 0xc + uVar6 * 8) - *(float *)(lVar2 + 4 + uVar6 * 8);
      fVar19 = fVar14 / SQRT(fVar22 * fVar22 + fVar21 * fVar21);
      fVar19 = (float)func_0x000142923e30((fVar19 * fVar22) / (fVar21 * fVar19));
      fVar20 = (fVar20 + fVar16) * _DAT_142925870;
      fStack_190 = fVar20;
      if ((fVar20 < 0.0) || (fVar14 < fVar20)) {
        ppuStack_168 = (undefined8 **)&fStack_190;
        pcStack_160 = FUN_140d8c910;
        uStack_138 = &UNK_142ff4538;
        uStack_130 = 1;
        uStack_118 = 0;
        pppuStack_128 = &ppuStack_168;
        uStack_120 = 1;
        FUN_1428d9390(&uStack_138,&UNK_142ff8570);
        goto LAB_1421b4275;
      }
      if (NAN(fVar20)) {
        FUN_1428d9430(&UNK_142ff4548,0x1d,&UNK_142ff8570);
        goto LAB_1421b4275;
      }
      fVar14 = (float)FUN_140c978e0(auStack_1f8);
      uStack_138 = (undefined *)
                   CONCAT44(fVar20 * fStack_27c + fVar14 * fStack_284,
                            fVar20 * fStack_280 + fVar14 * fStack_288);
      func_0x000140c9cdc0(afStack_234,&uStack_138);
      fVar20 = afStack_234[0];
      uStack_1d8 = uVar23;
      uStack_1d0 = 0;
      fVar30 = (float)afStack_234._4_8_;
      fVar31 = SUB84(afStack_234._4_8_,4);
      fVar32 = afStack_234[0] * 0.0;
      fVar14 = (_DAT_142ff83a0 - fVar19) * _DAT_14295a0e8;
      fVar16 = (float)func_0x000142923fd0(fVar14);
      fVar19 = (float)func_0x000142923e80(fVar14);
      fVar14 = _DAT_142925904;
      fVar25 = fVar20 - fVar30 * _DAT_1429259a0;
      fVar29 = fVar30 * _DAT_1429259a0 - fVar31;
      fVar32 = fVar31 * _UNK_1429259a4 - fVar32;
      fStack_290 = _UNK_1429259ac * 0.0 - 0.0;
      fVar33 = fVar16 * fVar29;
      fVar35 = fVar16 * fVar32;
      fVar16 = fVar16 * fVar25;
      fVar21 = fVar19 * fVar19;
      fVar24 = fVar16 * fVar16 + fVar35 * fVar35 + fVar33 * fVar33;
      fVar22 = fVar31 * fVar16 + fVar30 * fVar35 + fVar20 * fVar33;
      fVar22 = fVar22 + fVar22;
      fVar19 = fVar19 + fVar19;
      fVar34 = fVar19 * (fVar35 * fVar31 - fVar30 * fVar16) +
               fVar22 * fVar33 + (fVar21 - fVar24) * fVar20;
      fVar36 = fVar19 * (fVar16 * fVar20 - fVar31 * fVar33) +
               fVar22 * fVar35 + (fVar21 - fVar24) * fVar30;
      fVar21 = fVar19 * (fVar33 * fVar30 - fVar20 * fVar35) +
               fVar22 * fVar16 + (fVar21 - fVar24) * fVar31;
      fVar20 = fVar32 * fVar21 - fVar25 * fVar36;
      fVar16 = fVar25 * fVar34 - fVar21 * fVar29;
      fVar19 = fVar36 * fVar29 - fVar32 * fVar34;
      auStack_2b8 = ZEXT416((uint)fVar25);
      if (fVar19 <= 0.0) {
        fStack_2d4 = fVar36 - fVar29;
        if (fStack_2d4 <= 0.0) {
          fStack_2d4 = (_DAT_142925904 - fVar19) - fStack_2d4;
          fStack_2cc = _DAT_142925870 / SQRT(fStack_2d4);
          fStack_2d8 = fStack_2d4 * fStack_2cc;
          fStack_2d4 = (fVar32 + fVar34) * fStack_2cc;
          fStack_2d0 = (fVar20 + fVar25) * fStack_2cc;
          fStack_2cc = (fVar21 - fVar16) * fStack_2cc;
        }
        else {
          fStack_2d4 = fStack_2d4 + (_DAT_142925904 - fVar19);
          fStack_2cc = _DAT_142925870 / SQRT(fStack_2d4);
          fStack_2d8 = (fVar32 + fVar34) * fStack_2cc;
          fStack_2d4 = fStack_2d4 * fStack_2cc;
          fStack_2d0 = (fVar16 + fVar21) * fStack_2cc;
          fStack_2cc = (fVar20 - fVar25) * fStack_2cc;
        }
      }
      else {
        fVar22 = fVar29 + fVar36;
        if (fVar22 <= 0.0) {
          fVar22 = (fVar19 + _DAT_142925904) - fVar22;
          fStack_2cc = _DAT_142925870 / SQRT(fVar22);
          fStack_2d8 = (fVar20 + fVar25) * fStack_2cc;
          fStack_2d4 = (fVar16 + fVar21) * fStack_2cc;
          fStack_2d0 = fVar22 * fStack_2cc;
          fStack_2cc = (fVar32 - fVar34) * fStack_2cc;
        }
        else {
          fVar22 = fVar22 + fVar19 + _DAT_142925904;
          fStack_2cc = _DAT_142925870 / SQRT(fVar22);
          fStack_2d8 = (fVar21 - fVar16) * fStack_2cc;
          fStack_2d4 = (fVar20 - fVar25) * fStack_2cc;
          fStack_2d0 = (fVar32 - fVar34) * fStack_2cc;
          fStack_2cc = fStack_2cc * fVar22;
        }
      }
      fVar20 = (float)uStack_1d8;
      fVar16 = uStack_1d8._4_4_;
      fStack_298 = fVar32;
      fStack_294 = fVar32;
      fStack_28c = fStack_290;
      uVar23 = FUN_1420071b0(&fStack_2d8);
      fVar26 = fVar26 - fVar28;
      fVar27 = fVar27 - fVar18;
      fVar28 = SQRT(fVar27 * fVar27 + fVar26 * fVar26);
      fVar15 = (fVar17 + fVar15) * fStack_188;
      uStack_138 = (undefined *)CONCAT44(fVar15,fVar28);
      uStack_130 = CONCAT44(uStack_130._4_4_,0x3dcccccd);
      uVar7 = FUN_140c8ffb0(&uStack_138);
      fVar17 = (float)func_0x000140caf380(&uStack_170);
      fVar18 = fVar17 * _DAT_1429cd068 + (float)auStack_1e8._0_4_;
      fVar17 = 0.0;
      if (0.0 <= fVar18) {
        fVar17 = fVar18;
      }
      fVar18 = fVar14;
      if (fVar17 <= fVar14) {
        fVar18 = fVar17;
      }
      uVar3 = FUN_1408b9f20(fVar18);
      fVar20 = fVar20 + (float)uStack_2c8;
      fVar16 = fVar16 + uStack_2c8._4_4_;
      uStack_138 = (undefined *)CONCAT44(fVar16,fVar20);
      uStack_130 = (ulonglong)(uint)fStack_13c;
      pppuStack_128 = (undefined8 ***)0x7bff00003c000000;
      uStack_120 = CONCAT26(uVar3,(int6)uVar7);
      uStack_110 = uStack_154;
      uStack_10c = 0;
      uStack_100 = CONCAT44(iStack_184,1);
      uStack_118 = uVar23;
      iStack_104 = iVar9;
      FUN_1421b4510(uStack_1b0,&uStack_138);
      if (uVar11 == 0xffffffff) {
        fVar18 = fVar21 * fVar15 * _DAT_142925870;
        fVar17 = fVar15 * fVar34 * _DAT_1429258e0;
        fVar27 = fVar20 + fVar17;
        fVar16 = fVar16 + fVar15 * fVar36 * _UNK_1429258e4;
        uStack_138 = (undefined *)CONCAT44(fStack_13c - fVar18,fVar20 - fVar17);
        func_0x000140c9cde0(auStack_228,&uStack_138,CONCAT44(fVar16,fVar16));
        fVar20 = fStack_220;
        fVar18 = fStack_13c + fVar18;
        fVar19 = auStack_228._0_4_;
        fVar21 = auStack_228._4_4_;
        fVar17 = fStack_220 - fVar18;
        fVar15 = fVar19 - fVar27;
        fVar26 = fVar21 - fVar16;
        fVar17 = SQRT(fVar17 * fVar17 + fVar26 * fVar26 + fVar15 * fVar15);
        if (_DAT_142925984 < fVar17) {
          fVar15 = fStack_298 * 0.0 - fVar29;
          uStack_1b4 = 0x3dcccccd;
          fVar26 = (float)auStack_2b8._0_4_ - fStack_298 * 0.0;
          fVar29 = (fVar29 * 0.0 - (float)auStack_2b8._0_4_ * 0.0) - fVar29;
          if (fVar29 <= 0.0) {
            fStack_2dc = _DAT_142925870 / SQRT(fVar14 - fVar29);
            fStack_2e8 = (fVar14 - fVar29) * fStack_2dc;
            fStack_2e4 = (fVar26 + fStack_298) * fStack_2dc;
            fStack_2e0 = ((float)auStack_2b8._0_4_ + _DAT_142a887c0) * fStack_2dc;
            fStack_2dc = (fVar15 + _UNK_142a887c4) * fStack_2dc;
          }
          else {
            fStack_2dc = _DAT_142925870 / SQRT(_UNK_1429345b4 + fVar29);
            fStack_2e8 = (fVar32 + fVar26) * fStack_2dc;
            fStack_2e4 = (_UNK_1429345b4 + fVar29) * fStack_2dc;
            fStack_2e0 = (fVar15 + fVar14) * fStack_2dc;
            fStack_2dc = (0.0 - (float)auStack_2b8._0_4_) * fStack_2dc;
          }
          fStack_1bc = fVar28;
          fStack_1b8 = fVar17;
          uVar23 = FUN_1420071b0(&fStack_2e8);
          uVar7 = FUN_140c8ffb0(&fStack_1bc);
          uVar3 = FUN_1408b9f20(0);
          uStack_138 = (undefined *)
                       CONCAT44(fVar16 * _UNK_1429258e4 + fVar21 * _UNK_1429258e4,
                                fVar27 * _DAT_1429258e0 + fVar19 * _DAT_1429258e0);
          uStack_130 = (ulonglong)(uint)(fVar18 * _DAT_142925870 + fVar20 * _DAT_142925870);
          pppuStack_128 = (undefined8 ***)0x7bff00003c000000;
          uStack_120 = CONCAT26(uVar3,(int6)uVar7);
          uStack_110 = uStack_154;
          uStack_10c = 0;
          uStack_100 = 0x200000001;
          uStack_118 = uVar23;
          iStack_104 = iVar9;
          FUN_1421b4510(uStack_1b0,&uStack_138);
        }
      }
      iVar9 = iVar9 + 1;
    }
    in_RDX = puStack_200;
    iVar10 = iStack_174;
    param_3 = bStack_f1;
    if ((undefined8 ***)ppfStack_1a8 != (undefined8 ***)0x0) {
      func_0x000140613c20(pcStack_1a0,(longlong)ppfStack_1a8 << 2,4);
      in_RDX = puStack_200;
      iVar10 = iStack_174;
      param_3 = bStack_f1;
    }
  }
  FUN_1428d9430(&UNK_142ff8530,0x23,&UNK_142ff8558);
LAB_1421b4275:
                    /* WARNING: Does not return */
  pcVar1 = (code *)invalidInstructionException();
  (*pcVar1)();
}

