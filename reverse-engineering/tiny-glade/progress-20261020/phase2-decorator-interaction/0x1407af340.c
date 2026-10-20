
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

undefined8 *
FUN_1407af340(undefined8 *param_1,undefined8 param_2,undefined8 *param_3,undefined8 *param_4,
             undefined8 *param_5,undefined8 *param_6,undefined8 param_7,undefined8 *param_8)

{
  undefined1 uVar1;
  undefined4 uVar2;
  undefined4 uVar3;
  undefined4 uVar4;
  undefined8 uVar5;
  undefined8 uVar6;
  undefined8 uVar7;
  ulonglong uVar8;
  float *pfVar9;
  undefined8 *puVar10;
  undefined8 uVar11;
  undefined8 uVar12;
  undefined8 uVar13;
  undefined8 uVar14;
  char cVar15;
  undefined8 *puVar16;
  ulonglong uVar17;
  ulonglong uVar18;
  undefined4 uVar19;
  undefined8 *puVar20;
  uint uVar21;
  undefined8 *puVar22;
  ulonglong uVar23;
  undefined8 *puVar24;
  ulonglong uVar25;
  ulonglong *puVar26;
  ulonglong uVar27;
  longlong lVar28;
  longlong lVar29;
  longlong lVar30;
  ulonglong uVar31;
  ulonglong uVar32;
  ulonglong uVar33;
  ulonglong uVar34;
  char *pcVar35;
  undefined8 *puVar36;
  bool bVar37;
  bool bVar38;
  bool bVar39;
  bool bVar40;
  undefined8 uVar41;
  undefined8 *in_stack_00000060;
  undefined4 uStack_4d0;
  undefined4 uStack_448;
  float fStack_444;
  undefined4 uStack_440;
  undefined4 uStack_43c;
  undefined4 uStack_438;
  undefined4 uStack_434;
  undefined4 uStack_430;
  undefined4 uStack_42c;
  undefined4 uStack_428;
  undefined4 uStack_424;
  undefined4 uStack_420;
  undefined4 uStack_41c;
  int iStack_418;
  int iStack_414;
  undefined4 uStack_410;
  undefined4 uStack_40c;
  ulonglong uStack_358;
  ulonglong uStack_350;
  undefined8 *puStack_348;
  ulonglong uStack_340;
  undefined4 uStack_338;
  undefined4 uStack_334;
  undefined4 uStack_330;
  undefined4 uStack_32c;
  undefined8 uStack_320;
  undefined8 uStack_318;
  undefined1 auStack_308 [9];
  undefined4 uStack_2ff;
  undefined2 uStack_2fb;
  undefined1 uStack_2f9;
  undefined4 uStack_2bc;
  undefined8 uStack_2b8;
  undefined4 *puStack_2b0;
  undefined8 uStack_298;
  undefined8 uStack_290;
  undefined8 uStack_288;
  undefined8 uStack_280;
  undefined4 uStack_278;
  undefined4 uStack_274;
  undefined4 uStack_270;
  undefined4 uStack_26c;
  int iStack_268;
  int iStack_264;
  undefined8 uStack_260;
  byte bStack_1ef;
  undefined8 uStack_1e8;
  undefined1 uStack_1d8;
  undefined7 uStack_1d7;
  undefined1 uStack_1d0;
  undefined4 uStack_1cf;
  undefined2 uStack_1cb;
  undefined1 uStack_1c9;
  ulonglong uStack_1c8;
  undefined8 *puStack_1c0;
  undefined8 uStack_1b8;
  undefined8 uStack_1b0;
  int iStack_1a8;
  int iStack_1a4;
  undefined8 uStack_1a0;
  undefined8 *puStack_168;
  ulonglong *puStack_150;
  ulonglong *puStack_148;
  undefined4 uStack_13c;
  longlong lStack_138;
  undefined8 uStack_130;
  ulonglong uStack_120;
  undefined8 uStack_118;
  uint *puStack_110;
  undefined1 uStack_103;
  undefined1 uStack_102;
  ulonglong *puStack_100;
  undefined8 uStack_f0;
  
  uStack_f0 = 0xfffffffffffffffe;
  uVar41 = *param_8;
  uVar5 = param_8[8];
  uVar6 = param_8[0x10];
  puStack_148 = (ulonglong *)in_stack_00000060[5];
  lStack_138 = in_stack_00000060[6];
  puStack_150 = (ulonglong *)in_stack_00000060[10];
  uStack_120 = in_stack_00000060[0xb];
  uStack_2b8 = in_stack_00000060[0xf];
  uStack_2bc = *(undefined4 *)((longlong)in_stack_00000060 + 0x94);
  puStack_168 = param_3;
  uStack_118 = param_2;
  puStack_100 = param_4;
  puVar16 = (undefined8 *)func_0x00014093fbc0();
  if ((char)puVar16 != '\0') {
    return puVar16;
  }
  puStack_110 = (uint *)*param_1;
  uVar21 = *puStack_110;
  lVar29 = 0;
  if (uVar21 - 6 < 0xf) {
    lVar29 = (ulonglong)uVar21 - 5;
  }
  if (lVar29 == 0) {
    if ((4 < uVar21) || ((0x19U >> (uVar21 & 0x1f) & 1) == 0)) goto LAB_1407af4a0;
  }
  else if ((lVar29 != 6) && (lVar29 != 0xf)) {
LAB_1407af4a0:
    *(undefined4 *)puStack_100[2] = *(undefined4 *)((longlong)puStack_100 + 0x1c);
    puVar16 = (undefined8 *)*puStack_100;
    puVar16[1] = 0;
    *puVar16 = 6;
    return puVar16;
  }
  uVar7 = *param_5;
  fStack_444 = 3.4028235e+38;
  uStack_448 = CONCAT31(uStack_448._1_3_,5);
  auStack_308[0] = 0;
  uStack_1d8 = 0;
  uStack_1d7 = (undefined7)auStack_308._1_8_;
  uStack_1d0 = SUB81(auStack_308._1_8_,7);
  uStack_1cf = uStack_2ff;
  uStack_1cb = uStack_2fb;
  uStack_1c9 = uStack_2f9;
  uStack_320 = uVar6;
  uStack_318 = uVar5;
  uStack_1e8 = uVar41;
  func_0x0001409a8e10(&uStack_298,&uStack_448,&uStack_1d8);
  uStack_102 = 1;
  func_0x0001409a8fb0(&uStack_1d8);
  uStack_102 = 0;
  FUN_1409a8e80(&uStack_448,&uStack_298,&uStack_1d8);
  uVar27 = 0;
  FUN_1409a8ee0(&uStack_298,&uStack_448,0);
  uStack_130 = uVar7;
  FUN_1409236d0(uVar7,&UNK_142a835e6,0x1f,&uStack_298);
  lVar29 = in_stack_00000060[1];
  uVar18 = *(ulonglong *)*in_stack_00000060;
  uVar32 = uVar18 - *(ulonglong *)(lVar29 + 0x18);
  if (uVar18 < *(ulonglong *)(lVar29 + 0x18)) {
    uVar32 = uVar27;
  }
  uVar17 = uVar18 - *(ulonglong *)(lVar29 + 0x38);
  if (uVar18 < *(ulonglong *)(lVar29 + 0x38)) {
    uVar17 = uVar27;
  }
  uVar18 = *(ulonglong *)(lVar29 + 0x10);
  uVar8 = *(ulonglong *)(lVar29 + 0x30);
  bVar37 = uVar8 < uVar17;
  uVar31 = uVar8 - uVar17;
  uVar23 = uVar31;
  if (bVar37) {
    uVar23 = uVar27;
  }
  bVar38 = uVar18 < uVar32;
  uVar33 = uVar18 - uVar32;
  uVar34 = uVar33;
  if (bVar38) {
    uVar34 = uVar27;
  }
  uVar25 = uVar23 + uVar34;
  if (!bVar38 && uVar33 != 0 || !bVar37 && uVar31 != 0) {
    uVar25 = uVar27;
  }
  lVar28 = *(longlong *)(lVar29 + 8);
  lVar30 = *(longlong *)(lVar29 + 0x28);
  *(ulonglong *)*in_stack_00000060 = *(longlong *)(lVar29 + 0x40) - uVar25;
  if (!bVar38 && uVar33 != 0 || !bVar37 && uVar31 != 0) {
    lVar30 = lVar30 + uVar17 * 0x70;
    lVar28 = lVar28 + uVar32 * 0x70;
    if (uVar18 < uVar32) {
      lVar28 = 0x10;
    }
    puVar16 = (undefined8 *)0x0;
    if (uVar32 < uVar18) {
      puVar16 = (undefined8 *)(lVar28 + -0x70 + uVar34 * 0x70);
    }
    if (uVar8 < uVar17) {
      lVar30 = 0x10;
    }
    puVar20 = (undefined8 *)(lVar30 + -0x70 + uVar23 * 0x70);
    if (uVar8 <= uVar17) {
      puVar20 = puVar16;
    }
    puVar16 = (undefined8 *)puVar20[2];
    uVar5 = puVar20[6];
    uVar14 = puVar20[7];
    uStack_4d0 = (undefined4)((ulonglong)*(undefined8 *)((longlong)puVar20 + 0x3c) >> 0x20);
    uVar19 = *(undefined4 *)((longlong)puVar20 + 0x44);
    uVar2 = *(undefined4 *)(puVar20 + 9);
    uVar7 = puVar20[3];
    uVar11 = puVar20[4];
    uVar6 = puVar20[5];
    uVar1 = *(undefined1 *)(puVar20 + 0xb);
    uVar12 = *puVar20;
    uVar13 = puVar20[1];
    puVar22 = (undefined8 *)*puStack_100;
    uVar41 = *(undefined8 *)((longlong)puVar20 + 0x4c);
    *(undefined4 *)puStack_100[2] = *(undefined4 *)((longlong)puStack_100 + 0x1c);
    puVar22[1] = 0;
    *puVar22 = 1;
    puVar22[2] = uVar12;
    puVar22[3] = uVar13;
    puVar22[4] = uVar7;
    puVar22[5] = uVar11;
    puVar22[6] = uVar6;
    puVar22[7] = uVar5;
    puVar22[8] = uVar14;
    puVar22[9] = CONCAT44(uVar19,uStack_4d0);
    *(undefined4 *)(puVar22 + 10) = uVar2;
    *(undefined8 *)((longlong)puVar22 + 0x54) = uVar41;
    puVar22[0xc] = puVar16;
    *(undefined1 *)(puVar22 + 0xd) = 1;
    *(undefined1 *)((longlong)puVar22 + 0x69) = uVar1;
    return puVar16;
  }
  uStack_13c = *(undefined4 *)((longlong)puStack_100 + 0x1c);
  puStack_2b0 = (undefined4 *)puStack_100[2];
  *puStack_2b0 = uStack_13c;
  puVar26 = (ulonglong *)*puStack_100;
  uVar18 = *puVar26;
  lVar29 = 0;
  if ((puVar26[1] - 1) + (ulonglong)(1 < uVar18) < (ulonglong)(uVar18 - 2 < 5)) {
    lVar29 = uVar18 - 1;
  }
                    /* WARNING: Switch is manually overridden */
  switch(lVar29) {
  case 0:
    lVar29 = 0x69;
    lVar30 = 0x20;
    lVar28 = 0x60;
    goto code_r0x0001407af7ce;
  default:
    lVar29 = 0x5c;
    break;
  case 2:
    lVar29 = 0x60;
    break;
  case 5:
    uVar21 = *puStack_110;
    goto code_r0x0001407af8f7;
  }
  lVar30 = 0x18;
  lVar28 = 0x10;
code_r0x0001407af7ce:
  uVar18 = *(ulonglong *)((longlong)puVar26 + lVar28);
  uStack_298 = (longlong *)func_0x00014067dd80(uStack_130);
  uStack_290 = uVar18;
  cVar15 = func_0x00014092df10(&uStack_298);
  if ((cVar15 == '\x02') && (uVar21 = *puStack_110, uVar21 != 0x14)) {
code_r0x0001407af8f7:
    bVar37 = true;
    if (uVar21 == 3) {
      uVar18 = *puStack_148;
      uVar27 = 0;
      uVar32 = uVar18 - *(ulonglong *)(lStack_138 + 0x18);
      if (uVar18 < *(ulonglong *)(lStack_138 + 0x18)) {
        uVar32 = uVar27;
      }
      uVar17 = uVar18 - *(ulonglong *)(lStack_138 + 0x38);
      if (uVar18 < *(ulonglong *)(lStack_138 + 0x38)) {
        uVar17 = uVar27;
      }
      lVar29 = *(longlong *)(lStack_138 + 8);
      puStack_110 = *(uint **)(lStack_138 + 0x30);
      uVar18 = (longlong)puStack_110 - uVar17;
      if (uVar17 > puStack_110) {
        uVar18 = uVar27;
      }
      uVar8 = *(ulonglong *)(lStack_138 + 0x10);
      uVar23 = uVar8 - uVar32;
      if (uVar32 > uVar8) {
        uVar23 = uVar27;
      }
      lVar28 = *(longlong *)(lStack_138 + 0x28);
      uVar27 = *(ulonglong *)(lStack_138 + 0x40);
      *puStack_148 = (uVar27 - uVar18) - uVar23;
      if (uVar32 <= uVar8 && uVar8 - uVar32 != 0 ||
          uVar17 <= puStack_110 && (longlong)puStack_110 - uVar17 != 0) {
        lVar28 = lVar28 + uVar17 * 0x58;
        lVar29 = lVar29 + uVar32 * 0x58;
        if (uVar8 < uVar32) {
          lVar29 = 8;
        }
        lVar29 = lVar29 + -0x58 + uVar23 * 0x58;
        if (uVar8 <= uVar32) {
          lVar29 = 0;
        }
        if (puStack_110 < uVar17) {
          lVar28 = 8;
        }
        lVar28 = lVar28 + -0x58 + uVar18 * 0x58;
        if (puStack_110 <= uVar17) {
          lVar28 = lVar29;
        }
        *puStack_148 = uVar27;
        uStack_288 = (ulonglong)*(uint *)(lVar28 + 0x50);
        uVar21 = (uint)*(byte *)(lVar28 + 0x54);
        uVar18 = *(ulonglong *)(lVar28 + 8);
        uStack_340 = *(ulonglong *)(lVar28 + 0x28);
        uStack_338 = *(undefined4 *)(lVar28 + 0x30);
        uStack_334 = (undefined4)*(undefined8 *)(lVar28 + 0x34);
        uStack_330 = (undefined4)((ulonglong)*(undefined8 *)(lVar28 + 0x34) >> 0x20);
        uStack_32c = *(undefined4 *)(lVar28 + 0x3c);
        uVar19 = *(undefined4 *)(lVar28 + 0x40);
        uVar41 = *(undefined8 *)(lVar28 + 0x44);
        puVar16 = *(undefined8 **)(lVar28 + 0x20);
        uStack_358 = *(ulonglong *)(lVar28 + 0x10);
        uStack_350 = *(ulonglong *)(lVar28 + 0x18);
        uVar32 = 2;
        puStack_348 = puVar16;
      }
      else {
        in_stack_00000060 = in_stack_00000060 + 10;
        puStack_100 = puVar26;
        uVar41 = FUN_141282260(&uStack_298,in_stack_00000060);
        puVar16 = (undefined8 *)CONCAT44(uStack_274,uStack_278);
        bVar37 = uStack_290 == 0;
        uVar19 = (undefined4)CONCAT71((int7)((ulonglong)lVar29 >> 8),bVar37);
        bVar38 = uStack_290 == uStack_288;
        uVar21 = (uint)CONCAT71((int7)((ulonglong)in_stack_00000060 >> 8),bVar38 || bVar37);
        bVar39 = uStack_280 == (undefined8 *)0x0;
        bVar40 = uStack_280 == puVar16;
        uVar18 = CONCAT71((int7)((ulonglong)uStack_280 >> 8),bVar40 || bVar39);
        puVar26 = puStack_100;
        if ((bVar40 || bVar39) && (bVar38 || bVar37)) {
          uVar32 = 6;
        }
        else {
          puVar20 = (undefined8 *)(uStack_288 - 0x60);
          if (bVar38 || bVar37) {
            puVar20 = (undefined8 *)0x0;
          }
          puVar22 = puVar16 + -0xc;
          if (bVar40 || bVar39) {
            puVar22 = puVar20;
          }
          *uStack_298 = *uStack_298 + CONCAT44(uStack_26c,uStack_270);
          uStack_288 = (ulonglong)*(uint *)(puVar22 + 10);
          uVar18 = puVar22[1];
          uStack_340 = puVar22[5];
          uStack_338 = *(undefined4 *)(puVar22 + 6);
          uStack_334 = (undefined4)*(undefined8 *)((longlong)puVar22 + 0x34);
          uStack_330 = (undefined4)((ulonglong)*(undefined8 *)((longlong)puVar22 + 0x34) >> 0x20);
          uStack_32c = *(undefined4 *)((longlong)puVar22 + 0x3c);
          uVar19 = *(undefined4 *)(puVar22 + 8);
          uVar41 = *(undefined8 *)((longlong)puVar22 + 0x44);
          uStack_358 = puVar22[2];
          uStack_350 = puVar22[3];
          uVar21 = *(uint *)((longlong)puVar22 + 0x54);
          puVar16 = (undefined8 *)(ulonglong)*(byte *)(puVar22 + 0xb);
          uVar32 = 3;
          puStack_348 = (undefined8 *)puVar22[4];
        }
      }
      *puStack_2b0 = uStack_13c;
      puVar26[1] = 0;
      *puVar26 = uVar32;
      puVar26[2] = uVar18;
      puVar26[3] = uStack_358;
      puVar26[4] = uStack_350;
      puVar26[5] = (ulonglong)puStack_348;
      puVar26[6] = uStack_340;
      puVar26[7] = CONCAT44(uStack_334,uStack_338);
      puVar26[8] = CONCAT44(uStack_32c,uStack_330);
      *(undefined4 *)(puVar26 + 9) = uVar19;
      *(undefined8 *)((longlong)puVar26 + 0x4c) = uVar41;
      *(int *)(puVar26 + 0xb) = (int)uStack_288;
      *(uint *)((longlong)puVar26 + 0x5c) = uVar21;
      *(char *)(puVar26 + 0xc) = (char)puVar16;
      return puStack_348;
    }
    goto code_r0x0001407afb4c;
  }
  pcVar35 = (char *)(lVar30 + (longlong)puVar26);
  pfVar9 = (float *)*puStack_168;
  uVar19 = 0x30ab;
  if (*(char *)((longlong)puVar26 + lVar29) == '?') {
    uVar19 = 0x23;
  }
  FUN_1408cc2e0(&uStack_448,uStack_118,pfVar9,_UNK_142925980,uVar19);
  if (iStack_414 == 2) {
code_r0x0001407af8b0:
    func_0x000140a12650(&uStack_280,*param_6);
    uStack_298 = (longlong *)CONCAT71(uStack_298._1_7_,0xb);
    uStack_274 = 0;
    uStack_270 = 0x3f800000;
    uStack_26c = 0;
    iStack_268 = 0;
  }
  else {
    iStack_268 = iStack_418;
    iStack_264 = iStack_414;
    uStack_260 = CONCAT44(uStack_40c,uStack_410);
    uStack_278 = uStack_428;
    uStack_274 = uStack_424;
    uStack_270 = uStack_420;
    uStack_26c = uStack_41c;
    uStack_288 = CONCAT44(uStack_434,uStack_438);
    uStack_280 = (undefined8 *)CONCAT44(uStack_42c,uStack_430);
    uStack_298 = (longlong *)CONCAT44(fStack_444,uStack_448);
    uStack_290 = CONCAT44(uStack_43c,uStack_440);
    func_0x0001408cc630(&uStack_1d8,&uStack_298);
    if (iStack_1a8 == 2) goto code_r0x0001407af8b0;
    uStack_298 = (longlong *)CONCAT71(uStack_1d7,uStack_1d8);
    uStack_290 = CONCAT17(uStack_1c9,CONCAT25(uStack_1cb,CONCAT41(uStack_1cf,uStack_1d0)));
    iStack_268 = iStack_1a8;
    iStack_264 = iStack_1a4;
    uStack_260 = uStack_1a0;
    uStack_278 = (undefined4)uStack_1b8;
    uStack_274 = (undefined4)((ulonglong)uStack_1b8 >> 0x20);
    uStack_270 = (undefined4)uStack_1b0;
    uStack_26c = (undefined4)((ulonglong)uStack_1b0 >> 0x20);
    uStack_288 = uStack_1c8;
    uStack_280 = puStack_1c0;
  }
  *(int *)(pcVar35 + 0x30) = iStack_268;
  *(int *)(pcVar35 + 0x34) = iStack_264;
  *(undefined4 *)(pcVar35 + 0x38) = (undefined4)uStack_260;
  *(undefined4 *)(pcVar35 + 0x3c) = uStack_260._4_4_;
  *(ulonglong *)(pcVar35 + 0x20) = CONCAT44(uStack_274,uStack_278);
  *(ulonglong *)(pcVar35 + 0x28) = CONCAT44(uStack_26c,uStack_270);
  *(ulonglong *)(pcVar35 + 0x10) = uStack_288;
  *(undefined8 **)(pcVar35 + 0x18) = uStack_280;
  *(longlong **)pcVar35 = uStack_298;
  *(ulonglong *)(pcVar35 + 8) = uStack_290;
  if (((*(char *)((longlong)puVar26 + lVar29) == '?') && (*pcVar35 == '\x02')) &&
     ((pcVar35[1] & 1U) == 0)) {
    *pcVar35 = '\v';
  }
  uVar41 = *(undefined8 *)((longlong)puVar26 + lVar28);
  fStack_444 = SQRT((pfVar9[2] - *(float *)(pcVar35 + 0x20)) *
                    (pfVar9[2] - *(float *)(pcVar35 + 0x20)) +
                    (pfVar9[1] - *(float *)(pcVar35 + 0x1c)) *
                    (pfVar9[1] - *(float *)(pcVar35 + 0x1c)) +
                    (*pfVar9 - *(float *)(pcVar35 + 0x18)) * (*pfVar9 - *(float *)(pcVar35 + 0x18)))
               + _UNK_1429ddce8;
  uStack_448 = CONCAT31(uStack_448._1_3_,5);
  func_0x0001409a8e10(&uStack_298,&uStack_448,auStack_308);
  bStack_1ef = bStack_1ef | 2;
  func_0x000142923c00(&uStack_448,&uStack_298,0xb0);
  uStack_103 = 1;
  func_0x0001409a8fb0(&uStack_1d8);
  uStack_103 = 0;
  FUN_1409a8e80(&uStack_298,&uStack_448,&uStack_1d8);
  FUN_14092e3e0(uStack_130,uVar41);
  uVar21 = *puStack_110;
  bVar37 = false;
code_r0x0001407afb4c:
  if (uVar21 < 6) {
    puVar16 = (undefined8 *)(ulonglong)CONCAT31((int3)(uVar21 >> 8),uVar21 == 4);
    if ((bool)(bVar37 & uVar21 == 4)) {
      uVar18 = *puStack_150;
      puVar16 = (undefined8 *)0x0;
      puVar20 = (undefined8 *)(uVar18 - *(ulonglong *)(uStack_120 + 0x18));
      if (uVar18 < *(ulonglong *)(uStack_120 + 0x18)) {
        puVar20 = puVar16;
      }
      puVar22 = (undefined8 *)(uVar18 - *(ulonglong *)(uStack_120 + 0x38));
      if (uVar18 < *(ulonglong *)(uStack_120 + 0x38)) {
        puVar22 = puVar16;
      }
      lVar29 = *(longlong *)(uStack_120 + 8);
      puVar10 = *(undefined8 **)(uStack_120 + 0x10);
      puStack_100 = *(ulonglong **)(uStack_120 + 0x30);
      puVar24 = (undefined8 *)((longlong)puStack_100 - (longlong)puVar22);
      if (puStack_100 < puVar22) {
        puVar24 = puVar16;
      }
      puVar36 = (undefined8 *)((longlong)puVar10 - (longlong)puVar20);
      if (puVar10 < puVar20) {
        puVar36 = puVar16;
      }
      lVar28 = *(longlong *)(uStack_120 + 0x28);
      uStack_120 = *(ulonglong *)(uStack_120 + 0x40);
      *puStack_150 = (uStack_120 - (longlong)puVar24) - (longlong)puVar36;
      puVar16 = (undefined8 *)0x1;
      if (puVar10 >= puVar20 &&
          (undefined8 *)((longlong)puVar10 - (longlong)puVar20) != (undefined8 *)0x0 ||
          puStack_100 >= puVar22 &&
          (undefined8 *)((longlong)puStack_100 - (longlong)puVar22) != (undefined8 *)0x0) {
        lVar28 = lVar28 + (longlong)puVar22 * 0x60;
        lVar29 = lVar29 + (longlong)puVar20 * 0x60;
        if (puVar10 < puVar20) {
          lVar29 = 8;
        }
        lVar30 = 0;
        if (puVar20 < puVar10) {
          lVar30 = lVar29 + -0x60 + (longlong)puVar36 * 0x60;
        }
        if (puStack_100 < puVar22) {
          lVar28 = 8;
        }
        lVar29 = lVar28 + -0x60 + (longlong)puVar24 * 0x60;
        if (puStack_100 <= puVar22) {
          lVar29 = lVar30;
        }
        *puStack_150 = uStack_120;
        uVar21 = *(uint *)(lVar29 + 0x50);
        uVar18 = *(ulonglong *)(lVar29 + 8);
        uVar32 = *(ulonglong *)(lVar29 + 0x28);
        uVar2 = *(undefined4 *)(lVar29 + 0x30);
        uVar5 = *(undefined8 *)(lVar29 + 0x34);
        uVar3 = *(undefined4 *)(lVar29 + 0x3c);
        uVar4 = *(undefined4 *)(lVar29 + 0x40);
        uVar17 = *(ulonglong *)(lVar29 + 0x10);
        uVar8 = *(ulonglong *)(lVar29 + 0x18);
        uVar27 = *(ulonglong *)(lVar29 + 0x20);
        uVar19 = *(undefined4 *)(lVar29 + 0x54);
        uVar1 = *(undefined1 *)(lVar29 + 0x58);
        uVar41 = *(undefined8 *)(lVar29 + 0x44);
        *puStack_2b0 = uStack_13c;
        puVar26[1] = 0;
        *puVar26 = 3;
        puVar26[2] = uVar18;
        puVar26[5] = uVar27;
        puVar26[6] = uVar32;
        *(undefined4 *)(puVar26 + 7) = uVar2;
        *(undefined8 *)((longlong)puVar26 + 0x3c) = uVar5;
        *(undefined4 *)((longlong)puVar26 + 0x44) = uVar3;
        puVar26[3] = uVar17;
        puVar26[4] = uVar8;
        *(undefined4 *)(puVar26 + 9) = uVar4;
        *(undefined8 *)((longlong)puVar26 + 0x4c) = uVar41;
        *(uint *)(puVar26 + 0xb) = uVar21;
        *(undefined4 *)((longlong)puVar26 + 0x5c) = uVar19;
        *(undefined1 *)(puVar26 + 0xc) = uVar1;
        return (undefined8 *)(ulonglong)uVar21;
      }
    }
  }
  else {
    puVar16 = (undefined8 *)0x0;
  }
  if (!bVar37) {
    return puVar16;
  }
  uVar18 = *puVar26;
  lVar29 = 0;
  if ((puVar26[1] - 1) + (ulonglong)(1 < uVar18) < (ulonglong)(uVar18 - 2 < 5)) {
    lVar29 = uVar18 - 1;
  }
                    /* WARNING: Could not recover jumptable at 0x0001407afd6e. Too many branches */
                    /* WARNING: Treating indirect jump as call */
  puVar16 = (undefined8 *)
            (*(code *)(&UNK_142a837a8 + *(int *)(&UNK_142a837a8 + lVar29 * 4)))
                      (&UNK_142a837a8,&UNK_142a837a8 + *(int *)(&UNK_142a837a8 + lVar29 * 4));
  return puVar16;
}

