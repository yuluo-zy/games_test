
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void FUN_1409014e0(undefined8 param_1,longlong param_2)

{
  longlong lVar1;
  undefined4 uVar2;
  longlong lVar3;
  longlong lVar4;
  code *pcVar5;
  undefined8 uVar6;
  undefined8 uVar7;
  undefined8 uVar8;
  longlong lVar9;
  longlong lVar10;
  longlong lVar11;
  longlong lVar12;
  ulonglong uVar13;
  undefined1 auStack_5d8 [36];
  undefined1 auStack_5b4 [36];
  undefined4 uStack_590;
  undefined4 uStack_58c;
  undefined4 uStack_588;
  undefined4 uStack_584;
  undefined4 uStack_580;
  undefined4 uStack_57c;
  undefined4 uStack_578;
  undefined4 uStack_574;
  undefined4 uStack_570;
  undefined4 uStack_56c;
  undefined4 uStack_568;
  undefined4 uStack_564;
  undefined4 uStack_560;
  undefined4 uStack_55c;
  undefined4 uStack_558;
  undefined4 uStack_554;
  undefined4 uStack_550;
  undefined4 uStack_54c;
  undefined8 uStack_548;
  undefined8 uStack_540;
  undefined8 uStack_538;
  undefined8 uStack_530;
  undefined8 uStack_528;
  undefined8 uStack_520;
  undefined8 uStack_518;
  undefined8 uStack_510;
  undefined8 uStack_508;
  undefined8 uStack_500;
  longlong lStack_3b8;
  longlong lStack_3b0;
  longlong lStack_3a8;
  undefined8 uStack_3a0;
  undefined *puStack_398;
  undefined8 uStack_390;
  undefined8 uStack_388;
  undefined8 uStack_380;
  undefined8 uStack_378;
  undefined8 uStack_370;
  undefined8 uStack_368;
  undefined8 uStack_360;
  undefined8 uStack_358;
  undefined4 uStack_350;
  undefined4 uStack_34c;
  undefined4 uStack_348;
  undefined4 uStack_344;
  undefined8 uStack_340;
  undefined *puStack_338;
  undefined8 uStack_330;
  undefined8 uStack_328;
  undefined8 uStack_320;
  undefined8 uStack_318;
  undefined4 uStack_310;
  undefined4 uStack_30c;
  undefined4 uStack_308;
  undefined4 uStack_304;
  undefined4 uStack_300;
  undefined4 uStack_2fc;
  undefined4 uStack_2f8;
  undefined4 uStack_2f4;
  undefined4 uStack_2f0;
  undefined4 uStack_2ec;
  undefined4 uStack_2e8;
  undefined4 uStack_2e4;
  undefined8 uStack_2e0;
  undefined8 uStack_2d8;
  undefined8 uStack_2d0;
  undefined8 uStack_2c8;
  undefined8 uStack_2c0;
  undefined8 uStack_2b8;
  undefined4 uStack_2b0;
  undefined4 uStack_2ac;
  undefined8 uStack_2a8;
  undefined8 uStack_2a0;
  undefined8 uStack_298;
  undefined8 uStack_290;
  undefined1 auStack_288 [16];
  undefined1 auStack_278 [22];
  char cStack_262;
  longlong lStack_260;
  undefined8 uStack_218;
  undefined *puStack_208;
  undefined8 uStack_200;
  undefined8 uStack_1f8;
  undefined8 uStack_1f0;
  undefined8 uStack_1e8;
  undefined8 uStack_1e0;
  undefined8 uStack_1d8;
  undefined8 uStack_1d0;
  undefined8 uStack_1c8;
  undefined4 uStack_1c0;
  undefined4 uStack_1bc;
  undefined4 uStack_1b8;
  undefined4 uStack_1b4;
  undefined8 uStack_1b0;
  undefined8 uStack_1a8;
  undefined8 uStack_1a0;
  undefined8 uStack_198;
  undefined8 uStack_190;
  undefined8 uStack_188;
  undefined4 uStack_180;
  undefined4 uStack_17c;
  undefined4 uStack_178;
  undefined4 uStack_174;
  undefined4 uStack_170;
  undefined4 uStack_16c;
  undefined4 uStack_168;
  undefined4 uStack_164;
  undefined4 uStack_160;
  undefined4 uStack_15c;
  undefined4 uStack_158;
  undefined4 uStack_154;
  undefined8 uStack_150;
  undefined8 uStack_148;
  undefined8 uStack_140;
  undefined8 uStack_138;
  undefined8 uStack_130;
  undefined8 uStack_128;
  undefined4 uStack_120;
  undefined4 uStack_11c;
  undefined8 uStack_118;
  undefined8 uStack_110;
  undefined8 uStack_108;
  undefined8 uStack_100;
  undefined1 auStack_f8 [16];
  undefined1 auStack_e8 [22];
  char cStack_d2;
  longlong lStack_d0;
  undefined8 uStack_88;
  undefined4 uStack_74;
  undefined4 uStack_70;
  undefined4 uStack_6c;
  undefined4 uStack_68;
  undefined1 uStack_61;
  undefined8 uStack_60;
  
  uStack_60 = 0xfffffffffffffffe;
  lVar1 = param_2 + 0x38;
  uVar13 = 0x85c987d23d58d2de;
  uStack_3a0 = param_1;
  lVar9 = FUN_1400dd6f0(lVar1,0x85c987d23d58d2de,0x2ec6ab231f77203b);
  if (((lVar9 == 0) || (*(ulonglong *)(param_2 + 0x218) <= uVar13)) ||
     (uVar13 = *(ulonglong *)(*(longlong *)(param_2 + 0x210) + uVar13 * 8), uVar13 == 0)) {
code_r0x000140902928:
    FUN_1428d9310(&UNK_142aa9ed0);
code_r0x000140902934:
    FUN_1428d9310(&UNK_142aa9ee8);
code_r0x000140902940:
    FUN_1428d9310(&UNK_142aa9f00);
  }
  else {
    lVar9 = ~uVar13 * 0x60;
    if (*(longlong *)(*(longlong *)(param_2 + 0x1e0) + 0x20 + lVar9) == 0)
    goto code_r0x000140902928;
    lVar9 = *(longlong *)(param_2 + 0x1e0) + lVar9;
    lStack_3b8 = *(longlong *)(lVar9 + 0x10);
    uVar2 = *(undefined4 *)(param_2 + 0x5b0);
    uVar13 = 0xbb8761bcc7907109;
    lVar10 = FUN_1400dd6f0(lVar1,0xbb8761bcc7907109,0xdc4ef498cea8673a);
    if (((lVar10 == 0) || (*(ulonglong *)(param_2 + 0x218) <= uVar13)) ||
       (uVar13 = *(ulonglong *)(*(longlong *)(param_2 + 0x210) + uVar13 * 8), uVar13 == 0))
    goto code_r0x000140902934;
    lVar10 = ~uVar13 * 0x60;
    if (*(longlong *)(*(longlong *)(param_2 + 0x1e0) + 0x20 + lVar10) == 0)
    goto code_r0x000140902934;
    lVar10 = *(longlong *)(param_2 + 0x1e0) + lVar10;
    lStack_3b0 = *(longlong *)(lVar10 + 0x10);
    uStack_74 = *(undefined4 *)(param_2 + 0x5b0);
    uVar13 = 0x86486f3e82eb6bbe;
    lVar11 = FUN_1400dd6f0(lVar1,0x86486f3e82eb6bbe,0x4d7091c2f9bfa189);
    if (((lVar11 == 0) || (*(ulonglong *)(param_2 + 0x218) <= uVar13)) ||
       (uVar13 = *(ulonglong *)(*(longlong *)(param_2 + 0x210) + uVar13 * 8), uVar13 == 0))
    goto code_r0x000140902940;
    lVar11 = ~uVar13 * 0x60;
    if (*(longlong *)(*(longlong *)(param_2 + 0x1e0) + 0x20 + lVar11) == 0)
    goto code_r0x000140902940;
    lVar11 = *(longlong *)(param_2 + 0x1e0) + lVar11;
    lVar3 = *(longlong *)(lVar11 + 0x10);
    uStack_6c = *(undefined4 *)(param_2 + 0x5b0);
    uVar13 = 0x6dc90dcfd23786fa;
    lStack_3a8 = lVar9;
    uStack_70 = uVar2;
    lVar9 = FUN_1400dd6f0(lVar1,0x6dc90dcfd23786fa,0xed952bd348f66c04);
    if (((lVar9 != 0) && (uVar13 < *(ulonglong *)(param_2 + 0x218))) &&
       (uVar13 = *(ulonglong *)(*(longlong *)(param_2 + 0x210) + uVar13 * 8), uVar13 != 0)) {
      lVar9 = ~uVar13 * 0x60;
      if (*(longlong *)(*(longlong *)(param_2 + 0x1e0) + 0x20 + lVar9) != 0) {
        lVar9 = *(longlong *)(param_2 + 0x1e0) + lVar9;
        lVar4 = *(longlong *)(lVar9 + 0x10);
        uStack_68 = *(undefined4 *)(param_2 + 0x5b0);
        uVar13 = 0xcda6af2f6afb130e;
        lVar12 = FUN_1400dd6f0(lVar1,0xcda6af2f6afb130e,0x36e88a927bd70aa);
        lVar1 = lStack_3b0;
        if (((lVar12 != 0) && (uVar13 < *(ulonglong *)(param_2 + 0x218))) &&
           (uVar13 = *(ulonglong *)(*(longlong *)(param_2 + 0x210) + uVar13 * 8), uVar13 != 0)) {
          lVar12 = ~uVar13 * 0x60;
          if (*(longlong *)(*(longlong *)(param_2 + 0x1e0) + 0x20 + lVar12) != 0) {
            if (((*(char *)(lStack_3b8 + 0x29b) != '\0') ||
                (*(char *)(lStack_3b0 + 0x29b) == '\x01')) ||
               ((*(char *)(lVar3 + 0x29b) != '\0' ||
                ((lVar12 = *(longlong *)(*(longlong *)(param_2 + 0x1e0) + lVar12 + 0x10),
                 *(longlong *)(lVar12 + 0x30) != 0 || *(longlong *)(lVar12 + 0x10) != 0 ||
                 (*(char *)(lVar4 + 0x29b) != '\0')))))) {
              *(undefined4 *)(lStack_3a8 + 0x5c) = uStack_70;
              *(undefined1 *)(lStack_3b8 + 0x29b) = 0;
              func_0x000142923c20(lStack_3b8 + 0x98,0,0x200);
              *(undefined4 *)(lVar10 + 0x5c) = uStack_74;
              *(undefined1 *)(lVar1 + 0x29b) = 0;
              func_0x000142923c20(lVar1 + 0x98,0,0x200);
              *(undefined4 *)(lVar11 + 0x5c) = uStack_6c;
              *(undefined1 *)(lVar3 + 0x29b) = 0;
              func_0x000142923c20(lVar3 + 0x98,0,0x200);
              *(undefined4 *)(lVar9 + 0x5c) = uStack_68;
              *(undefined1 *)(lVar4 + 0x29b) = 0;
              func_0x000142923c20(lVar4 + 0x98,0,0x200);
              uVar8 = uStack_3a0;
              FUN_1416032a0(&puStack_208,&UNK_142aa9f48,0x1b,&UNK_142aa9f63,0x26,param_2,uStack_3a0)
              ;
              uStack_548._0_1_ = 1;
              puStack_398._0_4_ = 2;
              FUN_140905260(&uStack_548,&puStack_208,0xed502fff6dc4021f,0x19,&puStack_398);
              func_0x000142923c00(&puStack_398,&puStack_208,400);
              uStack_548._0_1_ = 2;
              puStack_208._0_4_ = 2;
              FUN_140905260(&uStack_548,&puStack_398,0x921351e406bfb529,0x19,&puStack_208);
              func_0x000142923c00(&puStack_208,&puStack_398,400);
              uStack_548._0_1_ = 3;
              puStack_398._0_4_ = 2;
              FUN_140905260(&uStack_548,&puStack_208,0x898e75329fb493b2,0x19,&puStack_398);
              func_0x000142923c00(&puStack_398,&puStack_208,400);
              uStack_548._0_1_ = 4;
              puStack_208 = (undefined *)CONCAT44(puStack_208._4_4_,2);
              FUN_140905260(&uStack_548,&puStack_398,0x60ec324b9742b267,0x19,&puStack_208);
              func_0x000142923c00(&puStack_208,&puStack_398,400);
              uStack_548 = (undefined4 *)CONCAT71(uStack_548._1_7_,5);
              puStack_398 = (undefined *)CONCAT44(puStack_398._4_4_,2);
              FUN_140905260(&uStack_548,&puStack_208,0xb5e1aef5c3a339a4,0x32,&puStack_398);
              func_0x000142923c00(&puStack_398,&puStack_208,400);
              uVar7 = _UNK_142aa2508;
              uVar6 = _UNK_142aa2500;
              uStack_528 = uStack_318;
              uStack_538 = uStack_328;
              uStack_530 = uStack_320;
              uStack_548 = (undefined4 *)puStack_338;
              uStack_540 = uStack_330;
              uStack_1f8 = uStack_388;
              puStack_208 = puStack_398;
              uStack_200 = uStack_390;
              uStack_1d8 = uStack_380;
              uStack_1d0 = uStack_378;
              uStack_1c8 = uStack_370;
              uStack_1f0 = uStack_368;
              uStack_1e8 = uStack_360;
              uStack_1e0 = uStack_358;
              uStack_190 = CONCAT44(uStack_2e4,uStack_2e8);
              uStack_188 = uStack_2e0;
              uStack_180 = (undefined4)uStack_2d8;
              uStack_17c = uStack_2d8._4_4_;
              uStack_178 = (undefined4)uStack_2d0;
              uStack_174 = uStack_2d0._4_4_;
              uStack_170 = (undefined4)uStack_2c8;
              uStack_16c = uStack_2c8._4_4_;
              uStack_168 = (undefined4)uStack_2c0;
              uStack_164 = uStack_2c0._4_4_;
              uStack_160 = (undefined4)uStack_2b8;
              uStack_15c = uStack_2b8._4_4_;
              uStack_158 = uStack_2b0;
              uStack_154 = uStack_2ac;
              uStack_150 = uStack_2a8;
              uStack_148 = uStack_2a0;
              uStack_140 = uStack_298;
              uStack_138 = uStack_290;
              uStack_1b0 = uStack_340;
              uStack_1c0 = uStack_350;
              uStack_1bc = uStack_34c;
              uStack_1b8 = uStack_348;
              uStack_1b4 = uStack_344;
              uStack_128 = uStack_218;
              uStack_130 = lStack_260;
              uStack_1a8 = _UNK_142aa2500;
              uStack_1a0 = _UNK_142aa2508;
              FUN_1409dd4d0(&uStack_548,&puStack_208);
              if (cStack_262 != '\b') {
                FUN_140be24b0(auStack_278);
              }
              func_0x000140bd52a0(&uStack_548,0x20000000200,0x4c);
              uStack_388 = uStack_538;
              uStack_380 = uStack_530;
              puStack_398 = (undefined *)uStack_548;
              uStack_390 = uStack_540;
              func_0x000140bd6ce0(&uStack_1f8,&puStack_398);
              puStack_208 = &UNK_142aa9fb9;
              uStack_200 = 0xb;
              uStack_56c = FUN_140be94d0(uVar8,&puStack_208);
              uStack_568 = (undefined4)uStack_548;
              uStack_564 = uStack_548._4_4_;
              uStack_560 = (undefined4)uStack_540;
              uStack_55c = uStack_540._4_4_;
              uStack_558 = (undefined4)uStack_538;
              uStack_554 = uStack_538._4_4_;
              uStack_550 = (undefined4)uStack_530;
              uStack_54c = uStack_530._4_4_;
              func_0x000140bd52a0(&uStack_548,0x20000000200,0x4c);
              uStack_388 = uStack_538;
              uStack_380 = uStack_530;
              puStack_398 = (undefined *)uStack_548;
              uStack_390 = uStack_540;
              func_0x000140bd6ce0(&uStack_1f8,&puStack_398);
              puStack_208 = &UNK_142aa9fc4;
              uStack_200 = 0xb;
              uStack_590 = FUN_140be94d0(uVar8,&puStack_208);
              uStack_58c = (undefined4)uStack_548;
              uStack_588 = uStack_548._4_4_;
              uStack_584 = (undefined4)uStack_540;
              uStack_580 = uStack_540._4_4_;
              uStack_57c = (undefined4)uStack_538;
              uStack_578 = uStack_538._4_4_;
              uStack_574 = (undefined4)uStack_530;
              uStack_570 = uStack_530._4_4_;
              FUN_1416032a0(&puStack_398,&UNK_142aa9ff7,0x17,&UNK_142aaa00e,0x23,param_2,uVar8);
              uStack_548 = (undefined4 *)CONCAT71(uStack_548._1_7_,9);
              puStack_208._0_4_ = 2;
              FUN_140905260(&uStack_548,&puStack_398,0x61b08b5e0470c5f2,0x19,&puStack_208);
              func_0x000142923c00(&puStack_208,&puStack_398,400);
              auStack_5b4[0] = 10;
              uStack_548 = (undefined4 *)CONCAT44(uStack_548._4_4_,2);
              FUN_140905260(auStack_5b4,&puStack_208,0x297d1e4cdae8f657,0x19,&uStack_548);
              func_0x000142923c00(&uStack_548,&puStack_208,400);
              puStack_398 = (undefined *)CONCAT71(puStack_398._1_7_,0xd);
              puStack_208._0_4_ = 2;
              FUN_140905260(&puStack_398,&uStack_548,0xab24287344aba8c1,0x19,&puStack_208);
              func_0x000142923c00(&puStack_208,&uStack_548,400);
              uStack_548 = (undefined4 *)CONCAT71(uStack_548._1_7_,5);
              puStack_398 = (undefined *)CONCAT44(puStack_398._4_4_,2);
              FUN_140905260(&uStack_548,&puStack_208,0xed502fff6dc4021f,0x19,&puStack_398);
              func_0x000142923c00(&puStack_398,&puStack_208,400);
              puStack_208 = (undefined *)CONCAT44(puStack_208._4_4_,2);
              uStack_548 = &uStack_590;
              FUN_1408edd00(&uStack_548,&puStack_398,0x1ca51a73373208db,0x32,&puStack_208);
              func_0x000142923c00(&puStack_208,&puStack_398,400);
              uStack_508 = CONCAT44(uStack_164,uStack_168);
              uStack_500 = CONCAT44(uStack_15c,uStack_160);
              uStack_518 = CONCAT44(uStack_174,uStack_178);
              uStack_510 = CONCAT44(uStack_16c,uStack_170);
              uStack_520 = CONCAT44(uStack_17c,uStack_180);
              uStack_528 = uStack_188;
              uStack_538 = uStack_198;
              uStack_530 = uStack_190;
              uStack_548 = (undefined4 *)uStack_1a8;
              uStack_540 = uStack_1a0;
              uStack_388 = uStack_1f8;
              puStack_398 = puStack_208;
              uStack_390 = uStack_200;
              uStack_368 = uStack_1f0;
              uStack_360 = uStack_1e8;
              uStack_358 = uStack_1e0;
              uStack_380 = uStack_1d8;
              uStack_378 = uStack_1d0;
              uStack_370 = uStack_1c8;
              uStack_320 = CONCAT44(uStack_154,uStack_158);
              uStack_318 = uStack_150;
              uStack_310 = (undefined4)uStack_148;
              uStack_30c = uStack_148._4_4_;
              uStack_308 = (undefined4)uStack_140;
              uStack_304 = uStack_140._4_4_;
              uStack_300 = (undefined4)uStack_138;
              uStack_2fc = uStack_138._4_4_;
              uStack_2f8 = (undefined4)uStack_130;
              uStack_2f4 = uStack_130._4_4_;
              uStack_2f0 = (undefined4)uStack_128;
              uStack_2ec = uStack_128._4_4_;
              uStack_2e8 = uStack_120;
              uStack_2e4 = uStack_11c;
              uStack_2e0 = uStack_118;
              uStack_2d8 = uStack_110;
              uStack_2d0 = uStack_108;
              uStack_2c8 = uStack_100;
              uStack_340 = uStack_1b0;
              uStack_350 = uStack_1c0;
              uStack_34c = uStack_1bc;
              uStack_348 = uStack_1b8;
              uStack_344 = uStack_1b4;
              uStack_2b8 = uStack_88;
              uStack_2c0 = lStack_d0;
              puStack_338 = (undefined *)uVar6;
              uStack_330 = uVar7;
              FUN_1409dd4d0(&uStack_548,&puStack_398);
              if (cStack_d2 != '\b') {
                FUN_140be24b0(auStack_e8);
              }
              FUN_1416032a0(&puStack_208,&UNK_142aa9fcf,0x10,&UNK_142aa9fdf,0x18,param_2,uVar8);
              if (cStack_d2 != '\b') {
                FUN_14096b850(auStack_f8,lStack_d0 + 0xe8,0xd91ff8372094bc22,1);
              }
              func_0x000142923c00(&puStack_398,&puStack_208,400);
              if (cStack_262 != '\b') {
                FUN_14096ba30(auStack_288,lStack_260 + 0xe8,0xdc022c4f3092a794,_UNK_14292e6f0);
              }
              func_0x000142923c00(&puStack_208,&puStack_398,400);
              puStack_398 = (undefined *)CONCAT44(puStack_398._4_4_,2);
              uStack_548 = &uStack_590;
              FUN_1408ee240(&uStack_548,&puStack_208,0xd82d7a0adb9164c2,0x19,&puStack_398);
              func_0x000142923c00(&puStack_398,&puStack_208,400);
              puStack_208 = (undefined *)CONCAT44(puStack_208._4_4_,2);
              uStack_548 = &uStack_56c;
              FUN_1408edd00(&uStack_548,&puStack_398,0x1ca51a73373208db,0x32,&puStack_208);
              func_0x000142923c00(&puStack_208,&puStack_398,400);
              uStack_508 = CONCAT44(uStack_164,uStack_168);
              uStack_500 = CONCAT44(uStack_15c,uStack_160);
              uStack_518 = CONCAT44(uStack_174,uStack_178);
              uStack_510 = CONCAT44(uStack_16c,uStack_170);
              uStack_520 = CONCAT44(uStack_17c,uStack_180);
              uStack_528 = uStack_188;
              uStack_538 = uStack_198;
              uStack_530 = uStack_190;
              uStack_548 = (undefined4 *)uStack_1a8;
              uStack_540 = uStack_1a0;
              uStack_388 = uStack_1f8;
              puStack_398 = puStack_208;
              uStack_390 = uStack_200;
              uStack_368 = uStack_1f0;
              uStack_360 = uStack_1e8;
              uStack_358 = uStack_1e0;
              uStack_380 = uStack_1d8;
              uStack_378 = uStack_1d0;
              uStack_370 = uStack_1c8;
              uStack_320 = CONCAT44(uStack_154,uStack_158);
              uStack_318 = uStack_150;
              uStack_310 = (undefined4)uStack_148;
              uStack_30c = uStack_148._4_4_;
              uStack_308 = (undefined4)uStack_140;
              uStack_304 = uStack_140._4_4_;
              uStack_300 = (undefined4)uStack_138;
              uStack_2fc = uStack_138._4_4_;
              uStack_2f8 = (undefined4)uStack_130;
              uStack_2f4 = uStack_130._4_4_;
              uStack_2f0 = (undefined4)uStack_128;
              uStack_2ec = uStack_128._4_4_;
              uStack_2e8 = uStack_120;
              uStack_2e4 = uStack_11c;
              uStack_2e0 = uStack_118;
              uStack_2d8 = uStack_110;
              uStack_2d0 = uStack_108;
              uStack_2c8 = uStack_100;
              uStack_340 = uStack_1b0;
              uStack_350 = uStack_1c0;
              uStack_34c = uStack_1bc;
              uStack_348 = uStack_1b8;
              uStack_344 = uStack_1b4;
              uStack_2b8 = uStack_88;
              uStack_2c0 = lStack_d0;
              puStack_338 = (undefined *)uVar6;
              uStack_330 = uVar7;
              FUN_1409dd4d0(&uStack_548,&puStack_398);
              if (cStack_d2 != '\b') {
                FUN_140be24b0(auStack_e8);
              }
              FUN_1416032a0(&puStack_208,&UNK_142aa9fcf,0x10,&UNK_142aa9fdf,0x18,param_2,uVar8);
              if (cStack_d2 != '\b') {
                FUN_14096b850(auStack_f8,lStack_d0 + 0xe8,0xd91ff8372094bc22,0);
              }
              func_0x000142923c00(&puStack_398,&puStack_208,400);
              if (cStack_262 != '\b') {
                FUN_14096ba30(auStack_288,lStack_260 + 0xe8,0xdc022c4f3092a794,_UNK_14292e6f0);
              }
              func_0x000142923c00(&puStack_208,&puStack_398,400);
              puStack_398 = (undefined *)CONCAT44(puStack_398._4_4_,2);
              uStack_548 = &uStack_56c;
              FUN_1408ee240(&uStack_548,&puStack_208,0xd82d7a0adb9164c2,0x19,&puStack_398);
              func_0x000142923c00(&puStack_398,&puStack_208,400);
              uStack_548 = (undefined4 *)CONCAT71(uStack_548._1_7_,5);
              puStack_208 = (undefined *)CONCAT44(puStack_208._4_4_,2);
              FUN_140905260(&uStack_548,&puStack_398,0x1ca51a73373208db,0x32,&puStack_208);
              func_0x000142923c00(&puStack_208,&puStack_398,400);
              uStack_508 = CONCAT44(uStack_164,uStack_168);
              uStack_500 = CONCAT44(uStack_15c,uStack_160);
              uStack_518 = CONCAT44(uStack_174,uStack_178);
              uStack_510 = CONCAT44(uStack_16c,uStack_170);
              uStack_520 = CONCAT44(uStack_17c,uStack_180);
              uStack_528 = uStack_188;
              uStack_538 = uStack_198;
              uStack_530 = uStack_190;
              uStack_548 = (undefined4 *)uStack_1a8;
              uStack_540 = uStack_1a0;
              uStack_388 = uStack_1f8;
              puStack_398 = puStack_208;
              uStack_390 = uStack_200;
              uStack_368 = uStack_1f0;
              uStack_360 = uStack_1e8;
              uStack_358 = uStack_1e0;
              uStack_380 = uStack_1d8;
              uStack_378 = uStack_1d0;
              uStack_370 = uStack_1c8;
              uStack_320 = CONCAT44(uStack_154,uStack_158);
              uStack_318 = uStack_150;
              uStack_310 = (undefined4)uStack_148;
              uStack_30c = uStack_148._4_4_;
              uStack_308 = (undefined4)uStack_140;
              uStack_304 = uStack_140._4_4_;
              uStack_300 = (undefined4)uStack_138;
              uStack_2fc = uStack_138._4_4_;
              uStack_2f8 = (undefined4)uStack_130;
              uStack_2f4 = uStack_130._4_4_;
              uStack_2f0 = (undefined4)uStack_128;
              uStack_2ec = uStack_128._4_4_;
              uStack_2e8 = uStack_120;
              uStack_2e4 = uStack_11c;
              uStack_2e0 = uStack_118;
              uStack_2d8 = uStack_110;
              uStack_2d0 = uStack_108;
              uStack_2c8 = uStack_100;
              uStack_340 = uStack_1b0;
              uStack_350 = uStack_1c0;
              uStack_34c = uStack_1bc;
              uStack_348 = uStack_1b8;
              uStack_344 = uStack_1b4;
              uStack_2b8 = uStack_88;
              uStack_2c0 = lStack_d0;
              puStack_338 = (undefined *)uVar6;
              uStack_330 = uVar7;
              FUN_1409dd4d0(&uStack_548,&puStack_398);
              if (cStack_d2 != '\b') {
                FUN_140be24b0(auStack_e8);
              }
              uVar8 = uStack_3a0;
              FUN_140903350(auStack_5d8,uStack_3a0,param_2,1);
              FUN_140903350(auStack_5b4,uVar8,param_2,2);
              FUN_140903350(&uStack_56c,uVar8,param_2,3);
              FUN_140903350(&uStack_590,uVar8,param_2,4);
              FUN_1416032a0(&puStack_208,&UNK_142aa9f89,0x11,&UNK_142aa9f9a,0x1f,param_2,uVar8);
              uStack_548 = (undefined4 *)CONCAT71(uStack_548._1_7_,0x13);
              puStack_398._0_4_ = 2;
              FUN_140905260(&uStack_548,&puStack_208,0x6ad63ccd18c8f041,0x19,&puStack_398);
              func_0x000142923c00(&puStack_398,&puStack_208,400);
              puStack_208._0_4_ = 2;
              uStack_548 = (undefined4 *)auStack_5d8;
              FUN_1408ee240(&uStack_548,&puStack_398,0x8d1f9969c90d5348,0x19,&puStack_208);
              func_0x000142923c00(&puStack_208,&puStack_398,400);
              puStack_398._0_4_ = 2;
              uStack_548 = (undefined4 *)auStack_5b4;
              FUN_1408ee240(&uStack_548,&puStack_208,0xf92d7b12a65bc8e2,0x19,&puStack_398);
              func_0x000142923c00(&puStack_398,&puStack_208,400);
              puStack_208._0_4_ = 2;
              uStack_548 = &uStack_56c;
              FUN_1408ee240(&uStack_548,&puStack_398,0xa88a3e9b7a2f3451,0x19,&puStack_208);
              func_0x000142923c00(&puStack_208,&puStack_398,400);
              puStack_398 = (undefined *)CONCAT44(puStack_398._4_4_,2);
              uStack_548 = &uStack_590;
              FUN_1408ee240(&uStack_548,&puStack_208,0x39b7b6f8247cf130,0x19,&puStack_398);
              func_0x000142923c00(&puStack_398,&puStack_208,400);
              uStack_548 = (undefined4 *)CONCAT71(uStack_548._1_7_,9);
              puStack_208._0_4_ = 2;
              FUN_140905260(&uStack_548,&puStack_398,0x61b08b5e0470c5f2,0x19,&puStack_208);
              func_0x000142923c00(&puStack_208,&puStack_398,400);
              uStack_61 = 10;
              uStack_548 = (undefined4 *)CONCAT44(uStack_548._4_4_,2);
              FUN_140905260(&uStack_61,&puStack_208,0x297d1e4cdae8f657,0x19,&uStack_548);
              func_0x000142923c00(&uStack_548,&puStack_208,400);
              puStack_398 = (undefined *)CONCAT71(puStack_398._1_7_,6);
              puStack_208 = (undefined *)CONCAT44(puStack_208._4_4_,2);
              FUN_140905260(&puStack_398,&uStack_548,0xd7f481303b104abc,0x32,&puStack_208);
              func_0x000142923c00(&puStack_208,&uStack_548,400);
              uStack_548 = (undefined4 *)CONCAT71(uStack_548._1_7_,7);
              puStack_398 = (undefined *)CONCAT44(puStack_398._4_4_,2);
              FUN_140905260(&uStack_548,&puStack_208,0x3ec9710e6411d8d1,0x32,&puStack_398);
              func_0x000142923c00(&puStack_398,&puStack_208,400);
              uStack_508 = CONCAT44(uStack_2f4,uStack_2f8);
              uStack_500 = CONCAT44(uStack_2ec,uStack_2f0);
              uStack_518 = CONCAT44(uStack_304,uStack_308);
              uStack_510 = CONCAT44(uStack_2fc,uStack_300);
              uStack_520 = CONCAT44(uStack_30c,uStack_310);
              uStack_528 = uStack_318;
              uStack_538 = uStack_328;
              uStack_530 = uStack_320;
              uStack_548 = (undefined4 *)puStack_338;
              uStack_540 = uStack_330;
              uStack_1f8 = uStack_388;
              puStack_208 = puStack_398;
              uStack_200 = uStack_390;
              uStack_1d8 = uStack_380;
              uStack_1d0 = uStack_378;
              uStack_1c8 = uStack_370;
              uStack_1f0 = uStack_368;
              uStack_1e8 = uStack_360;
              uStack_1e0 = uStack_358;
              uStack_190 = CONCAT44(uStack_2e4,uStack_2e8);
              uStack_188 = uStack_2e0;
              uStack_180 = (undefined4)uStack_2d8;
              uStack_17c = uStack_2d8._4_4_;
              uStack_178 = (undefined4)uStack_2d0;
              uStack_174 = uStack_2d0._4_4_;
              uStack_170 = (undefined4)uStack_2c8;
              uStack_16c = uStack_2c8._4_4_;
              uStack_168 = (undefined4)uStack_2c0;
              uStack_164 = uStack_2c0._4_4_;
              uStack_160 = (undefined4)uStack_2b8;
              uStack_15c = uStack_2b8._4_4_;
              uStack_158 = uStack_2b0;
              uStack_154 = uStack_2ac;
              uStack_150 = uStack_2a8;
              uStack_148 = uStack_2a0;
              uStack_140 = uStack_298;
              uStack_138 = uStack_290;
              uStack_1b0 = uStack_340;
              uStack_1c0 = uStack_350;
              uStack_1bc = uStack_34c;
              uStack_1b8 = uStack_348;
              uStack_1b4 = uStack_344;
              uStack_128 = uStack_218;
              uStack_130 = lStack_260;
              uStack_1a8 = uVar6;
              uStack_1a0 = uVar7;
              FUN_1409dd4d0(&uStack_548,&puStack_208);
              if (cStack_262 != '\b') {
                FUN_140be24b0(auStack_278);
              }
            }
            return;
          }
        }
        goto code_r0x000140902958;
      }
    }
  }
  FUN_1428d9310(&UNK_142aa9f18);
code_r0x000140902958:
  FUN_1428d9310(&UNK_142aa9f30);
  pcVar5 = (code *)swi(3);
  (*pcVar5)();
  return;
}

