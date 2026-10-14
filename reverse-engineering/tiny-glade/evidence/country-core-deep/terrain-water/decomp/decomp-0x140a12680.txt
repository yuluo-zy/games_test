
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void FUN_140a12680(undefined8 param_1,longlong param_2)

{
  longlong lVar1;
  code *pcVar2;
  undefined8 uVar3;
  undefined8 uVar4;
  longlong lVar5;
  ulonglong uVar6;
  undefined4 uStack_3ec;
  undefined4 uStack_3e8;
  undefined4 uStack_3e4;
  undefined4 uStack_3e0;
  undefined4 uStack_3dc;
  undefined4 uStack_3d8;
  undefined4 uStack_3d4;
  undefined4 uStack_3d0;
  undefined4 uStack_3cc;
  undefined *puStack_3c8;
  undefined8 uStack_3c0;
  undefined8 uStack_3b8;
  undefined8 uStack_3b0;
  undefined8 uStack_3a8;
  undefined8 uStack_3a0;
  undefined8 uStack_398;
  undefined8 uStack_390;
  undefined8 uStack_388;
  undefined4 uStack_380;
  undefined4 uStack_37c;
  undefined4 uStack_378;
  undefined4 uStack_374;
  undefined8 uStack_370;
  undefined8 uStack_368;
  undefined8 uStack_360;
  undefined8 uStack_350;
  undefined8 uStack_348;
  undefined4 uStack_340;
  undefined4 uStack_33c;
  undefined4 uStack_338;
  undefined4 uStack_334;
  undefined4 uStack_330;
  undefined4 uStack_32c;
  undefined4 uStack_328;
  undefined4 uStack_324;
  undefined4 uStack_320;
  undefined4 uStack_31c;
  undefined4 uStack_318;
  undefined4 uStack_314;
  undefined8 uStack_310;
  undefined8 uStack_308;
  undefined8 uStack_300;
  undefined8 uStack_2f8;
  longlong lStack_2f0;
  undefined8 uStack_2e8;
  undefined1 auStack_2b8 [38];
  char cStack_292;
  longlong lStack_290;
  undefined4 uStack_238;
  undefined4 uStack_234;
  undefined4 uStack_230;
  undefined4 uStack_22c;
  undefined4 uStack_228;
  undefined4 uStack_224;
  undefined4 uStack_220;
  undefined4 uStack_21c;
  undefined8 uStack_218;
  undefined8 uStack_210;
  undefined8 uStack_208;
  undefined8 uStack_200;
  undefined8 uStack_1f8;
  undefined4 uStack_1f0;
  undefined4 uStack_1ec;
  undefined4 uStack_1e8;
  undefined4 uStack_1e4;
  undefined8 uStack_1e0;
  undefined8 uStack_1d8;
  undefined8 uStack_1d0;
  undefined8 uStack_1c8;
  undefined8 uStack_1c0;
  undefined8 uStack_1b8;
  undefined8 uStack_1b0;
  undefined8 uStack_1a8;
  undefined8 uStack_1a0;
  undefined8 uStack_198;
  undefined8 uStack_190;
  undefined8 uStack_188;
  undefined8 uStack_180;
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
  undefined4 uStack_150;
  undefined4 uStack_14c;
  undefined8 uStack_148;
  undefined8 uStack_140;
  undefined8 uStack_138;
  undefined8 uStack_130;
  undefined1 auStack_128 [16];
  undefined1 auStack_118 [22];
  char cStack_102;
  longlong lStack_100;
  undefined8 uStack_b8;
  undefined8 uStack_a8;
  undefined8 uStack_a0;
  undefined8 uStack_98;
  undefined8 uStack_90;
  undefined8 uStack_88;
  undefined8 uStack_80;
  undefined8 uStack_78;
  undefined8 uStack_70;
  undefined8 uStack_68;
  undefined8 uStack_60;
  undefined8 uStack_50;
  
  uStack_50 = 0xfffffffffffffffe;
  uVar6 = 0x1ad182936286a0ee;
  lVar5 = FUN_1400dd6f0(param_2 + 0x38,0x1ad182936286a0ee,0xac04e3b6f9127e6f);
  if (((lVar5 != 0) && (uVar6 < *(ulonglong *)(param_2 + 0x218))) &&
     (uVar6 = *(ulonglong *)(*(longlong *)(param_2 + 0x210) + uVar6 * 8), uVar6 != 0)) {
    lVar5 = ~uVar6 * 0x60;
    if (*(longlong *)(*(longlong *)(param_2 + 0x1e0) + 0x20 + lVar5) != 0) {
      lVar5 = *(longlong *)(param_2 + 0x1e0) + lVar5;
      lVar1 = *(longlong *)(lVar5 + 0x10);
      if (*(char *)(lVar1 + 0x29b) != '\0') {
        *(undefined4 *)(lVar5 + 0x5c) = *(undefined4 *)(param_2 + 0x5b0);
        *(undefined1 *)(lVar1 + 0x29b) = 0;
        func_0x000142923c20(lVar1 + 0x98,0,0x200);
        func_0x000140bd52a0(&uStack_a8,0x20000000200,0x61);
        uStack_228 = (undefined4)uStack_98;
        uStack_224 = uStack_98._4_4_;
        uStack_220 = (undefined4)uStack_90;
        uStack_21c = uStack_90._4_4_;
        uStack_238 = (undefined4)uStack_a8;
        uStack_234 = uStack_a8._4_4_;
        uStack_230 = (undefined4)uStack_a0;
        uStack_22c = uStack_a0._4_4_;
        func_0x000140bd6ce0(&uStack_3b8,&uStack_238);
        puStack_3c8 = &UNK_142ac7cc8;
        uStack_3c0 = 0xf;
        uStack_3ec = FUN_140be94d0(param_1,&puStack_3c8);
        uStack_3e8 = (undefined4)uStack_a8;
        uStack_3e4 = uStack_a8._4_4_;
        uStack_3e0 = (undefined4)uStack_a0;
        uStack_3dc = uStack_a0._4_4_;
        uStack_3d8 = (undefined4)uStack_98;
        uStack_3d4 = uStack_98._4_4_;
        uStack_3d0 = (undefined4)uStack_90;
        uStack_3cc = uStack_90._4_4_;
        FUN_1416032a0(&puStack_3c8,&UNK_142ac7cd7,0xf,&UNK_142ac7ce6,0x18,param_2,param_1);
        if (cStack_292 != '\b') {
          FUN_14096b850(auStack_2b8,lStack_290 + 0xe8,0xd91ff8372094bc22,1);
        }
        func_0x000142923c00(&uStack_238,&puStack_3c8,400);
        uStack_a8 = (undefined4 *)CONCAT71(uStack_a8._1_7_,0x13);
        puStack_3c8 = (undefined *)CONCAT44(puStack_3c8._4_4_,2);
        FUN_140905260(&uStack_a8,&uStack_238,0x6ad63ccd18c8f041,0x19,&puStack_3c8);
        func_0x000142923c00(&puStack_3c8,&uStack_238,400);
        uStack_238 = 2;
        uStack_a8 = &uStack_3ec;
        FUN_1408edd00(&uStack_a8,&puStack_3c8,0xe8f7c3a7b57d0768,0x32,&uStack_238);
        func_0x000142923c00(&uStack_238,&puStack_3c8,400);
        if (cStack_102 != '\b') {
          FUN_14096ba30(auStack_128,lStack_100 + 0xe8,0xdc022c4f3092a794,_UNK_1429345a8);
        }
        func_0x000142923c00(&puStack_3c8,&uStack_238,400);
        if (cStack_292 != '\b') {
          FUN_14096b570(auStack_2b8,lStack_290 + 0xe8,0x4ad76c5b9d4fae28,1);
        }
        func_0x000142923c00(&uStack_238,&puStack_3c8,400);
        uVar4 = _UNK_142aa2508;
        uVar3 = _UNK_142aa2500;
        uStack_68 = uStack_198;
        uStack_60 = uStack_190;
        uStack_78 = uStack_1a8;
        uStack_70 = uStack_1a0;
        uStack_88 = uStack_1b8;
        uStack_80 = uStack_1b0;
        uStack_98 = uStack_1c8;
        uStack_90 = uStack_1c0;
        uStack_a8 = (undefined4 *)uStack_1d8;
        uStack_a0 = uStack_1d0;
        uStack_3b8 = CONCAT44(uStack_224,uStack_228);
        puStack_3c8 = (undefined *)CONCAT44(uStack_234,uStack_238);
        uStack_3c0 = CONCAT44(uStack_22c,uStack_230);
        uStack_398 = CONCAT44(uStack_21c,uStack_220);
        uStack_390 = uStack_218;
        uStack_388 = uStack_210;
        uStack_3b0 = uStack_208;
        uStack_3a8 = uStack_200;
        uStack_3a0 = uStack_1f8;
        uStack_350 = uStack_188;
        uStack_348 = uStack_180;
        uStack_340 = uStack_178;
        uStack_33c = uStack_174;
        uStack_338 = uStack_170;
        uStack_334 = uStack_16c;
        uStack_330 = uStack_168;
        uStack_32c = uStack_164;
        uStack_328 = uStack_160;
        uStack_324 = uStack_15c;
        uStack_320 = uStack_158;
        uStack_31c = uStack_154;
        uStack_318 = uStack_150;
        uStack_314 = uStack_14c;
        uStack_310 = uStack_148;
        uStack_308 = uStack_140;
        uStack_300 = uStack_138;
        uStack_2f8 = uStack_130;
        uStack_370 = uStack_1e0;
        uStack_380 = uStack_1f0;
        uStack_37c = uStack_1ec;
        uStack_378 = uStack_1e8;
        uStack_374 = uStack_1e4;
        uStack_2e8 = uStack_b8;
        lStack_2f0 = lStack_100;
        uStack_368 = _UNK_142aa2500;
        uStack_360 = _UNK_142aa2508;
        FUN_1409dd4d0(&uStack_a8,&puStack_3c8);
        if (cStack_102 != '\b') {
          FUN_140be24b0(auStack_118);
        }
        FUN_1416032a0(&puStack_3c8,&UNK_142ac7cd7,0xf,&UNK_142ac7ce6,0x18,param_2,param_1);
        if (cStack_292 != '\b') {
          FUN_14096b850(auStack_2b8,lStack_290 + 0xe8,0xd91ff8372094bc22,0);
        }
        func_0x000142923c00(&uStack_238,&puStack_3c8,400);
        puStack_3c8 = (undefined *)CONCAT44(puStack_3c8._4_4_,2);
        uStack_a8 = &uStack_3ec;
        FUN_1408ee240(&uStack_a8,&uStack_238,0x6ad63ccd18c8f041,0x19,&puStack_3c8);
        func_0x000142923c00(&puStack_3c8,&uStack_238,400);
        uStack_a8 = (undefined4 *)CONCAT71(uStack_a8._1_7_,0x13);
        uStack_238 = 2;
        FUN_140905260(&uStack_a8,&puStack_3c8,0xe8f7c3a7b57d0768,0x32,&uStack_238);
        func_0x000142923c00(&uStack_238,&puStack_3c8,400);
        if (cStack_102 != '\b') {
          FUN_14096ba30(auStack_128,lStack_100 + 0xe8,0xdc022c4f3092a794,_UNK_1429345a8);
        }
        func_0x000142923c00(&puStack_3c8,&uStack_238,400);
        if (cStack_292 != '\b') {
          FUN_14096b570(auStack_2b8,lStack_290 + 0xe8,0x4ad76c5b9d4fae28,1);
        }
        func_0x000142923c00(&uStack_238,&puStack_3c8,400);
        uStack_68 = uStack_198;
        uStack_60 = uStack_190;
        uStack_78 = uStack_1a8;
        uStack_70 = uStack_1a0;
        uStack_88 = uStack_1b8;
        uStack_80 = uStack_1b0;
        uStack_98 = uStack_1c8;
        uStack_90 = uStack_1c0;
        uStack_a8 = (undefined4 *)uStack_1d8;
        uStack_a0 = uStack_1d0;
        uStack_3b8 = CONCAT44(uStack_224,uStack_228);
        puStack_3c8 = (undefined *)CONCAT44(uStack_234,uStack_238);
        uStack_3c0 = CONCAT44(uStack_22c,uStack_230);
        uStack_398 = CONCAT44(uStack_21c,uStack_220);
        uStack_390 = uStack_218;
        uStack_388 = uStack_210;
        uStack_3b0 = uStack_208;
        uStack_3a8 = uStack_200;
        uStack_3a0 = uStack_1f8;
        uStack_350 = uStack_188;
        uStack_348 = uStack_180;
        uStack_340 = uStack_178;
        uStack_33c = uStack_174;
        uStack_338 = uStack_170;
        uStack_334 = uStack_16c;
        uStack_330 = uStack_168;
        uStack_32c = uStack_164;
        uStack_328 = uStack_160;
        uStack_324 = uStack_15c;
        uStack_320 = uStack_158;
        uStack_31c = uStack_154;
        uStack_318 = uStack_150;
        uStack_314 = uStack_14c;
        uStack_310 = uStack_148;
        uStack_308 = uStack_140;
        uStack_300 = uStack_138;
        uStack_2f8 = uStack_130;
        uStack_370 = uStack_1e0;
        uStack_380 = uStack_1f0;
        uStack_37c = uStack_1ec;
        uStack_378 = uStack_1e8;
        uStack_374 = uStack_1e4;
        uStack_2e8 = uStack_b8;
        lStack_2f0 = lStack_100;
        uStack_368 = uVar3;
        uStack_360 = uVar4;
        FUN_1409dd4d0(&uStack_a8,&puStack_3c8);
        if (cStack_102 != '\b') {
          FUN_140be24b0(auStack_118);
        }
        FUN_1416032a0(&puStack_3c8,&UNK_142ac7cd7,0xf,&UNK_142ac7ce6,0x18,param_2,param_1);
        if (cStack_292 != '\b') {
          FUN_14096b850(auStack_2b8,lStack_290 + 0xe8,0xd91ff8372094bc22,1);
        }
        func_0x000142923c00(&uStack_238,&puStack_3c8,400);
        uStack_a8 = (undefined4 *)CONCAT71(uStack_a8._1_7_,0x13);
        puStack_3c8 = (undefined *)CONCAT44(puStack_3c8._4_4_,2);
        FUN_140905260(&uStack_a8,&uStack_238,0x6ad63ccd18c8f041,0x19,&puStack_3c8);
        func_0x000142923c00(&puStack_3c8,&uStack_238,400);
        uStack_238 = 2;
        uStack_a8 = &uStack_3ec;
        FUN_1408edd00(&uStack_a8,&puStack_3c8,0xe8f7c3a7b57d0768,0x32,&uStack_238);
        func_0x000142923c00(&uStack_238,&puStack_3c8,400);
        if (cStack_102 != '\b') {
          FUN_14096ba30(auStack_128,lStack_100 + 0xe8,0xdc022c4f3092a794,_UNK_1429cd034);
        }
        func_0x000142923c00(&puStack_3c8,&uStack_238,400);
        if (cStack_292 != '\b') {
          FUN_14096b570(auStack_2b8,lStack_290 + 0xe8,0x4ad76c5b9d4fae28,2);
        }
        func_0x000142923c00(&uStack_238,&puStack_3c8,400);
        uStack_68 = uStack_198;
        uStack_60 = uStack_190;
        uStack_78 = uStack_1a8;
        uStack_70 = uStack_1a0;
        uStack_88 = uStack_1b8;
        uStack_80 = uStack_1b0;
        uStack_98 = uStack_1c8;
        uStack_90 = uStack_1c0;
        uStack_a8 = (undefined4 *)uStack_1d8;
        uStack_a0 = uStack_1d0;
        uStack_3b8 = CONCAT44(uStack_224,uStack_228);
        puStack_3c8 = (undefined *)CONCAT44(uStack_234,uStack_238);
        uStack_3c0 = CONCAT44(uStack_22c,uStack_230);
        uStack_398 = CONCAT44(uStack_21c,uStack_220);
        uStack_390 = uStack_218;
        uStack_388 = uStack_210;
        uStack_3b0 = uStack_208;
        uStack_3a8 = uStack_200;
        uStack_3a0 = uStack_1f8;
        uStack_350 = uStack_188;
        uStack_348 = uStack_180;
        uStack_340 = uStack_178;
        uStack_33c = uStack_174;
        uStack_338 = uStack_170;
        uStack_334 = uStack_16c;
        uStack_330 = uStack_168;
        uStack_32c = uStack_164;
        uStack_328 = uStack_160;
        uStack_324 = uStack_15c;
        uStack_320 = uStack_158;
        uStack_31c = uStack_154;
        uStack_318 = uStack_150;
        uStack_314 = uStack_14c;
        uStack_310 = uStack_148;
        uStack_308 = uStack_140;
        uStack_300 = uStack_138;
        uStack_2f8 = uStack_130;
        uStack_370 = uStack_1e0;
        uStack_380 = uStack_1f0;
        uStack_37c = uStack_1ec;
        uStack_378 = uStack_1e8;
        uStack_374 = uStack_1e4;
        uStack_2e8 = uStack_b8;
        lStack_2f0 = lStack_100;
        uStack_368 = uVar3;
        uStack_360 = uVar4;
        FUN_1409dd4d0(&uStack_a8,&puStack_3c8);
        if (cStack_102 != '\b') {
          FUN_140be24b0(auStack_118);
        }
        FUN_1416032a0(&puStack_3c8,&UNK_142ac7cd7,0xf,&UNK_142ac7ce6,0x18,param_2,param_1);
        if (cStack_292 != '\b') {
          FUN_14096b850(auStack_2b8,lStack_290 + 0xe8,0xd91ff8372094bc22,0);
        }
        func_0x000142923c00(&uStack_238,&puStack_3c8,400);
        puStack_3c8 = (undefined *)CONCAT44(puStack_3c8._4_4_,2);
        uStack_a8 = &uStack_3ec;
        FUN_1408ee240(&uStack_a8,&uStack_238,0x6ad63ccd18c8f041,0x19,&puStack_3c8);
        func_0x000142923c00(&puStack_3c8,&uStack_238,400);
        uStack_a8 = (undefined4 *)CONCAT71(uStack_a8._1_7_,0x13);
        uStack_238 = 2;
        FUN_140905260(&uStack_a8,&puStack_3c8,0xe8f7c3a7b57d0768,0x32,&uStack_238);
        func_0x000142923c00(&uStack_238,&puStack_3c8,400);
        if (cStack_102 != '\b') {
          FUN_14096ba30(auStack_128,lStack_100 + 0xe8,0xdc022c4f3092a794,_UNK_1429cd034);
        }
        func_0x000142923c00(&puStack_3c8,&uStack_238,400);
        if (cStack_292 != '\b') {
          FUN_14096b570(auStack_2b8,lStack_290 + 0xe8,0x4ad76c5b9d4fae28,2);
        }
        func_0x000142923c00(&uStack_238,&puStack_3c8,400);
        uStack_68 = uStack_198;
        uStack_60 = uStack_190;
        uStack_78 = uStack_1a8;
        uStack_70 = uStack_1a0;
        uStack_88 = uStack_1b8;
        uStack_80 = uStack_1b0;
        uStack_98 = uStack_1c8;
        uStack_90 = uStack_1c0;
        uStack_a8 = (undefined4 *)uStack_1d8;
        uStack_a0 = uStack_1d0;
        uStack_3b8 = CONCAT44(uStack_224,uStack_228);
        puStack_3c8 = (undefined *)CONCAT44(uStack_234,uStack_238);
        uStack_3c0 = CONCAT44(uStack_22c,uStack_230);
        uStack_398 = CONCAT44(uStack_21c,uStack_220);
        uStack_390 = uStack_218;
        uStack_388 = uStack_210;
        uStack_3b0 = uStack_208;
        uStack_3a8 = uStack_200;
        uStack_3a0 = uStack_1f8;
        uStack_350 = uStack_188;
        uStack_348 = uStack_180;
        uStack_340 = uStack_178;
        uStack_33c = uStack_174;
        uStack_338 = uStack_170;
        uStack_334 = uStack_16c;
        uStack_330 = uStack_168;
        uStack_32c = uStack_164;
        uStack_328 = uStack_160;
        uStack_324 = uStack_15c;
        uStack_320 = uStack_158;
        uStack_31c = uStack_154;
        uStack_318 = uStack_150;
        uStack_314 = uStack_14c;
        uStack_310 = uStack_148;
        uStack_308 = uStack_140;
        uStack_300 = uStack_138;
        uStack_2f8 = uStack_130;
        uStack_370 = uStack_1e0;
        uStack_380 = uStack_1f0;
        uStack_37c = uStack_1ec;
        uStack_378 = uStack_1e8;
        uStack_374 = uStack_1e4;
        uStack_2e8 = uStack_b8;
        lStack_2f0 = lStack_100;
        uStack_368 = uVar3;
        uStack_360 = uVar4;
        FUN_1409dd4d0(&uStack_a8,&puStack_3c8);
        if (cStack_102 != '\b') {
          FUN_140be24b0(auStack_118);
        }
      }
      return;
    }
  }
  FUN_1428d9310(&UNK_142ac7cb0);
  pcVar2 = (code *)swi(3);
  (*pcVar2)();
  return;
}

