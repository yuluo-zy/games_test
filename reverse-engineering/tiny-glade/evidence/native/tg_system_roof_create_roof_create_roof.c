
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void FUN_1421aaea0(undefined8 *param_1,undefined8 *param_2,longlong *param_3,longlong *param_4,
                  longlong *param_5,undefined8 param_6)

{
  undefined4 *puVar1;
  longlong lVar2;
  undefined8 uVar3;
  undefined8 uVar4;
  code *pcVar5;
  undefined8 uVar6;
  longlong lVar7;
  undefined1 uVar8;
  char cVar9;
  char cVar10;
  undefined4 uVar11;
  uint uVar12;
  ulonglong uVar13;
  longlong lVar14;
  longlong lVar15;
  longlong lVar16;
  ulonglong uVar17;
  ulonglong uVar18;
  ulonglong uVar19;
  undefined4 uVar20;
  undefined4 uVar21;
  undefined4 uVar22;
  undefined8 uStack_378;
  undefined8 uStack_370;
  undefined8 uStack_368;
  undefined8 uStack_360;
  undefined8 uStack_358;
  undefined8 uStack_350;
  undefined8 uStack_348;
  undefined8 uStack_340;
  undefined8 uStack_338;
  undefined8 uStack_330;
  undefined8 uStack_328;
  undefined8 uStack_320;
  undefined8 uStack_318;
  undefined8 uStack_310;
  undefined8 uStack_308;
  undefined8 uStack_300;
  undefined8 uStack_2f8;
  undefined8 uStack_2f0;
  undefined8 uStack_2e8;
  undefined8 uStack_2e0;
  undefined8 uStack_2d8;
  undefined8 uStack_2d0;
  undefined8 uStack_2c8;
  undefined4 uStack_2c0;
  undefined4 uStack_2bc;
  undefined8 uStack_2b8;
  longlong lStack_2b0;
  undefined4 uStack_2a8;
  undefined4 uStack_2a4;
  undefined4 uStack_2a0;
  undefined4 uStack_29c;
  undefined4 uStack_298;
  undefined4 uStack_294;
  undefined4 uStack_290;
  undefined4 uStack_28c;
  undefined4 uStack_288;
  undefined4 uStack_284;
  undefined4 uStack_280;
  undefined4 uStack_27c;
  undefined4 uStack_278;
  undefined4 uStack_274;
  undefined4 uStack_270;
  undefined4 uStack_26c;
  undefined8 uStack_268;
  undefined8 uStack_260;
  undefined8 uStack_250;
  undefined8 uStack_248;
  undefined8 uStack_240;
  undefined8 uStack_238;
  undefined8 uStack_230;
  undefined8 uStack_228;
  undefined8 uStack_220;
  undefined8 uStack_218;
  undefined8 uStack_210;
  undefined8 uStack_208;
  undefined8 uStack_200;
  undefined8 uStack_1f4;
  undefined8 uStack_1ec;
  undefined8 uStack_1e4;
  uint uStack_1dc;
  undefined4 uStack_1d8;
  undefined4 uStack_1d4;
  undefined4 uStack_1d0;
  undefined4 uStack_1cc;
  undefined8 uStack_1c8;
  undefined8 uStack_1c0;
  undefined4 uStack_1b8;
  undefined4 uStack_1b4;
  undefined8 *puStack_1b0;
  undefined8 uStack_1a8;
  undefined8 uStack_1a0;
  undefined8 uStack_198;
  undefined8 uStack_190;
  undefined8 uStack_188;
  undefined8 uStack_180;
  undefined8 uStack_178;
  longlong lStack_170;
  undefined8 uStack_168;
  undefined8 uStack_160;
  undefined8 uStack_158;
  undefined4 uStack_150;
  undefined4 uStack_14c;
  undefined4 uStack_148;
  undefined4 uStack_144;
  undefined4 uStack_140;
  undefined4 uStack_13c;
  undefined4 uStack_138;
  undefined4 uStack_134;
  undefined4 uStack_130;
  undefined4 uStack_12c;
  undefined1 uStack_128;
  undefined7 uStack_127;
  undefined8 uStack_120;
  ulonglong *puStack_118;
  undefined8 uStack_110;
  undefined4 *puStack_108;
  undefined4 *puStack_100;
  ulonglong uStack_f8;
  ulonglong uStack_f0;
  undefined4 *puStack_e8;
  longlong lStack_e0;
  longlong lStack_d8;
  longlong lStack_d0;
  longlong lStack_c8;
  ulonglong uStack_c0;
  ulonglong uStack_b8;
  undefined8 uStack_b0;
  longlong lStack_a8;
  undefined4 uStack_a0;
  undefined4 uStack_9c;
  undefined4 uStack_98;
  undefined1 uStack_91;
  undefined8 uStack_90;
  
  uStack_90 = 0xfffffffffffffffe;
  puStack_118 = (ulonglong *)*param_1;
  lVar2 = param_1[1];
  uVar19 = *puStack_118;
  uVar18 = 0;
  uVar13 = uVar19 - *(ulonglong *)(lVar2 + 0x18);
  if (uVar19 < *(ulonglong *)(lVar2 + 0x18)) {
    uVar13 = uVar18;
  }
  uVar17 = uVar19 - *(ulonglong *)(lVar2 + 0x38);
  if (uVar19 < *(ulonglong *)(lVar2 + 0x38)) {
    uVar17 = uVar18;
  }
  uStack_b8 = uVar13 * 0x40 + *(longlong *)(lVar2 + 8);
  uVar19 = *(ulonglong *)(lVar2 + 0x10) - uVar13;
  if (*(ulonglong *)(lVar2 + 0x10) < uVar13) {
    uStack_b8 = 8;
    uVar19 = uVar18;
  }
  uStack_c0 = uVar17 * 0x40 + *(longlong *)(lVar2 + 0x28);
  uVar13 = *(ulonglong *)(lVar2 + 0x30) - uVar17;
  if (*(ulonglong *)(lVar2 + 0x30) < uVar17) {
    uStack_c0 = 8;
    uVar13 = uVar18;
  }
  *puStack_118 = (*(longlong *)(lVar2 + 0x40) - uVar13) - uVar19;
  pcVar5 = _UNK_14293e0c8;
  lVar7 = _DAT_14293e0c0;
  uStack_f8 = uVar19 * 0x40 + uStack_b8;
  uVar19 = uVar13 * 0x40 + uStack_c0;
  uStack_110 = *param_2;
  uStack_98 = *(undefined4 *)((longlong)param_4 + 0x1c);
  lStack_a8 = *param_4;
  puStack_e8 = (undefined4 *)param_4[2];
  uStack_a0 = *(undefined4 *)((longlong)param_5 + 0x1c);
  lStack_c8 = *param_5;
  puStack_108 = (undefined4 *)param_5[2];
  uStack_9c = *(undefined4 *)((longlong)param_3 + 0x1c);
  lVar2 = *param_3;
  puStack_100 = (undefined4 *)param_3[2];
  lStack_d0 = lStack_a8 + 0x20;
  lStack_e0 = lStack_c8 + 0x20;
  lStack_d8 = lVar2 + 0x20;
  uStack_f0 = uVar19;
  while( true ) {
    uVar6 = uStack_110;
    if ((uStack_b8 == 0) || (uStack_b8 == uStack_f8)) {
      if ((uStack_c0 == 0) || (uStack_c0 == uStack_f0)) {
        return;
      }
      uStack_b8 = 0;
      uVar13 = uStack_c0;
      uStack_c0 = uStack_c0 + 0x40;
    }
    else {
      uVar13 = uStack_b8;
      uStack_b8 = uStack_b8 + 0x40;
    }
    *puStack_118 = *puStack_118 + 1;
    lVar14 = FUN_140a91ef0(uStack_110,*(undefined8 *)(uVar13 + 8),&UNK_142ff73e0);
    if (lVar14 == 0) goto LAB_1421ab5aa;
    lVar15 = FUN_140a91ef0(uVar6,*(undefined8 *)(uVar13 + 8),&UNK_142ff7410);
    if (lVar15 == 0) goto LAB_1421ab5b6;
    if (1 < *(uint *)(lVar15 + 600)) break;
    uStack_1c8 = *(undefined8 *)(lVar15 + 0x26c);
    uStack_1d8 = *(undefined4 *)(lVar15 + 0x25c);
    uStack_1d4 = *(undefined4 *)(lVar15 + 0x260);
    uStack_1d0 = *(undefined4 *)(lVar15 + 0x264);
    uStack_1cc = *(undefined4 *)(lVar15 + 0x268);
    uStack_91 = *(undefined1 *)(lVar15 + 0x299);
    uStack_1dc = *(uint *)(lVar15 + 600);
    uVar20 = FUN_140b2a620(lVar14);
    lVar14 = *(longlong *)(uVar13 + 8);
    uStack_168 = *(undefined8 *)(uVar13 + 0x20);
    uStack_160 = *(code **)(uVar13 + 0x28);
    uStack_158 = *(undefined8 *)(uVar13 + 0x30);
    uStack_150 = *(undefined4 *)(uVar13 + 0x38);
    uVar22 = *(undefined4 *)(uVar13 + 0x10);
    uVar8 = func_0x0001408c5670(&uStack_91);
    uStack_1b8 = *(undefined4 *)(uVar13 + 0x1c);
    uStack_1c0 = *(undefined8 *)(uVar13 + 0x14);
    uStack_368 = (undefined8 *)CONCAT44((undefined4)uStack_1c8,uStack_1cc);
    uStack_360 = CONCAT44(uStack_360._4_4_,uStack_1c8._4_4_);
    uStack_378 = (undefined *)CONCAT44(uStack_1d8,uStack_1dc);
    uStack_370 = CONCAT44(uStack_1d0,uStack_1d4);
    func_0x0001408e31c0(&uStack_250,lVar14,&uStack_168,&uStack_378,uVar20,uVar22,uVar8,&uStack_1c0);
    cVar9 = func_0x0001408e3250(&uStack_250);
    cVar10 = func_0x0001408e33b0(&uStack_250);
    uVar6 = uStack_250;
    uVar18 = uVar19;
    if (cVar10 != '\0') {
      *puStack_e8 = uStack_98;
      lVar14 = *(longlong *)(lStack_a8 + 0x30);
      uVar3 = *(undefined8 *)(lStack_a8 + 0x40);
      if (lVar14 == *(longlong *)(lStack_a8 + 0x20)) {
        FUN_140562630(lStack_d0,&UNK_142b72d70);
      }
      lVar15 = *(longlong *)(lStack_a8 + 0x28);
      *(undefined8 *)(lVar15 + lVar14 * 0x10) = uVar3;
      *(undefined8 *)(lVar15 + 8 + lVar14 * 0x10) = uVar6;
      *(longlong *)(lStack_a8 + 0x30) = lVar14 + 1;
      *(longlong *)(lStack_a8 + 0x40) = *(longlong *)(lStack_a8 + 0x40) + 1;
      uVar18 = uVar19 & 0xffffffff;
      lVar14 = lStack_a8;
    }
    uVar22 = uVar20;
    func_0x000140939390(&uStack_1f4);
    func_0x0001408e4520(&uStack_1c0);
    uVar21 = func_0x0001408dbc40();
    uStack_178 = func_0x000140002370();
    lStack_170 = lVar14;
    uStack_b0 = uStack_178;
    uVar11 = func_0x000140833250(0);
    func_0x00014082bd60(&uStack_138,uVar11);
    uStack_150 = 0;
    uStack_158 = 0;
    func_0x00014003bb80();
    uStack_128 = 9;
    uStack_168 = lVar7;
    uStack_160 = pcVar5;
    uStack_144 = _DAT_142ff76e4;
    uStack_14c = (undefined4)_DAT_142ff76dc;
    uStack_148 = (undefined4)((ulonglong)_DAT_142ff76dc >> 0x20);
    uStack_2e0 = uStack_200;
    uStack_2f0 = uStack_210;
    uStack_2e8 = uStack_208;
    uStack_300 = uStack_220;
    uStack_2f8 = uStack_218;
    uStack_310 = uStack_230;
    uStack_308 = uStack_228;
    uStack_320 = uStack_240;
    uStack_318 = uStack_238;
    uStack_330 = uStack_250;
    uStack_328 = uStack_248;
    uStack_2d8 = uStack_1f4;
    uStack_2d0 = uStack_1ec;
    uStack_2c8 = uStack_1e4;
    uStack_370 = CONCAT44(uStack_1b4,uStack_1b8);
    uStack_378 = (undefined *)uStack_1c0;
    uStack_368 = puStack_1b0;
    uStack_360 = uStack_1a8;
    uStack_358 = uStack_1a0;
    uStack_350 = uStack_198;
    uStack_348 = uStack_190;
    uStack_340 = uStack_188;
    uStack_338 = uStack_180;
    uStack_2b8 = uStack_b0;
    uStack_268 = CONCAT71(uStack_127,9);
    uStack_260 = uStack_120;
    uStack_168._0_4_ = (undefined4)lVar7;
    uStack_168._4_4_ = (undefined4)((ulonglong)lVar7 >> 0x20);
    uStack_160._0_4_ = SUB84(pcVar5,0);
    uStack_160._4_4_ = (undefined4)((ulonglong)pcVar5 >> 0x20);
    uStack_278 = uStack_138;
    uStack_274 = uStack_134;
    uStack_270 = uStack_130;
    uStack_26c = uStack_12c;
    uStack_288 = uStack_148;
    uStack_284 = _DAT_142ff76e4;
    uStack_280 = uStack_140;
    uStack_27c = uStack_13c;
    uStack_298 = (undefined4)uStack_158;
    uStack_294 = uStack_158._4_4_;
    uStack_290 = uStack_150;
    uStack_28c = uStack_14c;
    uStack_2a8 = (undefined4)uStack_168;
    uStack_2a4 = uStack_168._4_4_;
    uStack_2a0 = (undefined4)uStack_160;
    uStack_29c = uStack_160._4_4_;
    uVar11 = (undefined4)uStack_158;
    uStack_2c0 = uVar21;
    uStack_2bc = uVar22;
    lStack_2b0 = lVar14;
    uStack_168 = lVar7;
    uStack_160 = pcVar5;
    FUN_140dcf620(&uStack_168,param_6,&uStack_378,&UNK_142ff7440);
    uVar6 = CONCAT44(uStack_144,uStack_148);
    if (cVar9 == '\0') {
      uVar12 = func_0x0001408c5670(&uStack_91);
      uVar18 = (ulonglong)uVar12;
      uVar8 = 6;
    }
    else {
      uVar8 = 0;
    }
    uVar22 = func_0x0001411ff2a0(&uStack_1dc);
    uStack_1c0 = CONCAT44(uVar11,uVar22);
    func_0x000140c9cde0(&uStack_368,&uStack_1c0,uVar20);
    uVar19 = uVar18 & 0xffffffff;
    uStack_378._0_2_ = CONCAT11((char)uVar18,uVar8);
    *puStack_108 = uStack_a0;
    lVar14 = *(longlong *)(lStack_c8 + 0x30);
    uVar3 = *(undefined8 *)(lStack_c8 + 0x40);
    if (lVar14 == *(longlong *)(lStack_c8 + 0x20)) {
      FUN_140562ab0(lStack_e0,&UNK_142b72d70);
    }
    lVar15 = *(longlong *)(lStack_c8 + 0x28);
    *(undefined8 *)(lVar15 + lVar14 * 0x28) = uVar3;
    puVar1 = (undefined4 *)(lVar15 + 0x14 + lVar14 * 0x28);
    *puVar1 = uStack_370._4_4_;
    puVar1[1] = (undefined4)uStack_368;
    puVar1[2] = uStack_368._4_4_;
    puVar1[3] = (undefined4)uStack_360;
    puVar1 = (undefined4 *)(lVar15 + 8 + lVar14 * 0x28);
    *puVar1 = (undefined4)uStack_378;
    puVar1[1] = uStack_378._4_4_;
    puVar1[2] = (undefined4)uStack_370;
    puVar1[3] = uStack_370._4_4_;
    *(longlong *)(lStack_c8 + 0x30) = lVar14 + 1;
    *(longlong *)(lStack_c8 + 0x40) = *(longlong *)(lStack_c8 + 0x40) + 1;
    uVar3 = *(undefined8 *)(uVar13 + 8);
    *puStack_100 = uStack_9c;
    lVar14 = *(longlong *)(lVar2 + 0x30);
    uVar4 = *(undefined8 *)(lVar2 + 0x40);
    if (lVar14 == *(longlong *)(lVar2 + 0x20)) {
      FUN_140562c30(lStack_d8,&UNK_142b72d70);
    }
    lVar15 = *(longlong *)(lVar2 + 0x28);
    lVar16 = lVar14 * 0x20;
    *(undefined8 *)(lVar15 + lVar16) = uVar4;
    *(undefined8 *)(lVar15 + 8 + lVar16) = uVar6;
    *(undefined8 *)(lVar15 + 0x10 + lVar16) = uVar3;
    *(undefined1 *)(lVar15 + 0x18 + lVar16) = 0;
    *(longlong *)(lVar2 + 0x30) = lVar14 + 1;
    *(longlong *)(lVar2 + 0x40) = *(longlong *)(lVar2 + 0x40) + 1;
  }
  uStack_168 = lVar15 + 600;
  uStack_160 = FUN_1421aae00;
  uStack_378 = &UNK_142ff74a8;
  uStack_370 = 1;
  uStack_358 = 0;
  uStack_368 = &uStack_168;
  uStack_360 = 1;
  FUN_1428d9390(&uStack_378,&UNK_142ff74b8);
LAB_1421ab5aa:
  FUN_1428d9310(&UNK_142ff73f8);
LAB_1421ab5b6:
  FUN_1428d9310(&UNK_142ff7428);
  pcVar5 = (code *)swi(3);
  (*pcVar5)();
  return;
}

