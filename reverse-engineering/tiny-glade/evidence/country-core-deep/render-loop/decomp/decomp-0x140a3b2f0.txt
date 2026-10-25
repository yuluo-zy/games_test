
undefined8 *
FUN_140a3b2f0(undefined8 *param_1,undefined8 param_2,longlong param_3,undefined8 *param_4,
             undefined8 *param_5,undefined8 *param_6,undefined8 *param_7,undefined8 *param_8,
             undefined4 param_9)

{
  undefined1 uVar1;
  code *pcVar2;
  undefined8 uVar3;
  undefined8 uVar4;
  longlong lVar5;
  undefined8 uVar6;
  undefined8 *puVar7;
  ulonglong uVar8;
  byte bVar9;
  undefined8 uStack_3b8;
  undefined8 uStack_3b0;
  undefined8 uStack_3a8;
  undefined8 uStack_3a0;
  undefined8 uStack_398;
  undefined8 uStack_390;
  undefined8 uStack_388;
  undefined8 uStack_380;
  undefined8 uStack_378;
  undefined4 uStack_370;
  undefined4 uStack_36c;
  undefined4 uStack_368;
  undefined4 uStack_364;
  undefined8 uStack_360;
  undefined4 uStack_358;
  undefined4 uStack_354;
  undefined4 uStack_350;
  undefined4 uStack_34c;
  undefined8 uStack_340;
  undefined8 uStack_338;
  undefined4 uStack_330;
  undefined4 uStack_32c;
  undefined4 uStack_328;
  undefined4 uStack_324;
  undefined4 uStack_320;
  undefined4 uStack_31c;
  undefined4 uStack_318;
  undefined4 uStack_314;
  undefined4 uStack_310;
  undefined4 uStack_30c;
  undefined4 uStack_308;
  undefined4 uStack_304;
  undefined8 uStack_300;
  undefined8 uStack_2f8;
  undefined8 uStack_2f0;
  undefined8 uStack_2e8;
  longlong lStack_2e0;
  undefined8 uStack_2d8;
  undefined1 auStack_2a8 [38];
  char cStack_282;
  longlong lStack_280;
  undefined4 uStack_228;
  undefined4 uStack_224;
  undefined8 uStack_220;
  undefined8 uStack_218;
  undefined8 uStack_210;
  undefined8 uStack_208;
  undefined8 uStack_200;
  undefined8 uStack_1f8;
  undefined8 uStack_1f0;
  undefined8 uStack_1e8;
  undefined4 uStack_1e0;
  undefined4 uStack_1dc;
  undefined4 uStack_1d8;
  undefined4 uStack_1d4;
  undefined8 uStack_1d0;
  undefined8 *puStack_1c8;
  undefined8 uStack_1c0;
  undefined8 uStack_1b8;
  undefined8 uStack_1b0;
  undefined8 uStack_1a8;
  undefined8 uStack_1a0;
  undefined8 uStack_198;
  undefined8 uStack_190;
  undefined8 uStack_188;
  undefined8 uStack_180;
  undefined8 uStack_178;
  undefined8 uStack_170;
  undefined4 uStack_168;
  undefined4 uStack_164;
  undefined4 uStack_160;
  undefined4 uStack_15c;
  undefined4 uStack_158;
  undefined4 uStack_154;
  undefined4 uStack_150;
  undefined4 uStack_14c;
  undefined4 uStack_148;
  undefined4 uStack_144;
  undefined4 uStack_140;
  undefined4 uStack_13c;
  undefined8 uStack_138;
  undefined8 uStack_130;
  undefined8 uStack_128;
  undefined8 uStack_120;
  undefined1 auStack_118 [16];
  undefined1 auStack_108 [22];
  char cStack_f2;
  longlong lStack_f0;
  undefined8 uStack_a8;
  undefined8 *puStack_98;
  undefined8 uStack_90;
  undefined8 uStack_88;
  undefined8 uStack_80;
  undefined8 uStack_78;
  undefined8 uStack_70;
  undefined8 uStack_68;
  undefined8 uStack_60;
  undefined8 uStack_58;
  undefined8 uStack_50;
  undefined1 uStack_41;
  undefined8 uStack_40;
  
  uStack_40 = 0xfffffffffffffffe;
  uVar8 = 0xa4c02661e67b048b;
  lVar5 = FUN_1400dd6f0(param_3 + 0x38,0xa4c02661e67b048b,0xeae761e6aa2a67ec);
  if (((lVar5 != 0) && (uVar8 < *(ulonglong *)(param_3 + 0x218))) &&
     (uVar8 = *(ulonglong *)(*(longlong *)(param_3 + 0x210) + uVar8 * 8), uVar8 != 0)) {
    lVar5 = ~uVar8 * 0x60;
    if (*(longlong *)(*(longlong *)(param_3 + 0x1e0) + 0x20 + lVar5) != 0) {
      uVar1 = *(undefined1 *)(*(longlong *)(*(longlong *)(param_3 + 0x1e0) + lVar5 + 0x10) + 0x20);
      FUN_1416032a0(&uStack_228,&UNK_142acd490,3,&UNK_142acd493,0xc,param_3,param_2);
      if (cStack_f2 != '\b') {
        bVar9 = 0;
        if (*(char *)((longlong)param_8 + 0x24) != '\0') {
          bVar9 = bRam0000000143634d50 ^ 1;
        }
        FUN_14096b570(auStack_118,lStack_f0 + 0xe8,0xa9a109b760eed902,bVar9);
      }
      func_0x000142923c00(&uStack_3b8,&uStack_228,400);
      uStack_228 = 2;
      puStack_98 = param_4;
      FUN_1408ee240(&puStack_98,&uStack_3b8,0xd9978e541ce9a95e,0x19,&uStack_228);
      func_0x000142923c00(&uStack_228,&uStack_3b8,400);
      puStack_98 = param_8;
      uStack_3b8._0_4_ = 2;
      FUN_14096f090(&puStack_98,&uStack_228,0x185a603c077dad19,0x19,&uStack_3b8);
      func_0x000142923c00(&uStack_3b8,&uStack_228,400);
      puStack_98 = param_6;
      uStack_228 = 2;
      FUN_1408ee240(&puStack_98,&uStack_3b8,0xb8a74ccd835a6be3,0x19,&uStack_228);
      func_0x000142923c00(&uStack_228,&uStack_3b8,400);
      puStack_98 = param_7;
      uStack_3b8 = CONCAT44(uStack_3b8._4_4_,2);
      FUN_1408edd00(&puStack_98,&uStack_228,0xaa75015a6ad61913,0x32,&uStack_3b8);
      func_0x000142923c00(&uStack_3b8,&uStack_228,400);
      puStack_98 = param_5;
      uStack_228 = 2;
      FUN_1408edd00(&puStack_98,&uStack_3b8,0x88219a3ae69892b2,0x32,&uStack_228);
      func_0x000142923c00(&uStack_228,&uStack_3b8,400);
      if (cStack_f2 != '\b') {
        FUN_14096b570(auStack_118,lStack_f0 + 0xe8,0xc4cf81f2f4a55f92,uVar1);
      }
      func_0x000142923c00(&uStack_3b8,&uStack_228,400);
      if (cStack_282 != '\b') {
        FUN_14096ba30(auStack_2a8,lStack_280 + 0xe8,0xa902d50b67521d49,param_9);
      }
      func_0x000142923c00(&uStack_228,&uStack_3b8,400);
      uStack_41 = 1;
      uVar6 = func_0x000140bd5790((longlong)param_7 + 4);
      uStack_58 = uStack_188;
      uStack_50 = uStack_180;
      uStack_68 = uStack_198;
      uStack_60 = uStack_190;
      uStack_78 = uStack_1a8;
      uStack_70 = uStack_1a0;
      uStack_88 = uStack_1b8;
      uStack_80 = uStack_1b0;
      puStack_98 = puStack_1c8;
      uStack_90 = uStack_1c0;
      uStack_3a8 = uStack_218;
      uStack_3b8 = CONCAT44(uStack_224,uStack_228);
      uStack_3b0 = uStack_220;
      uStack_388 = uStack_210;
      uStack_380 = uStack_208;
      uStack_378 = uStack_200;
      uStack_3a0 = uStack_1f8;
      uStack_398 = uStack_1f0;
      uStack_390 = uStack_1e8;
      uStack_340 = uStack_178;
      uStack_338 = uStack_170;
      uStack_330 = uStack_168;
      uStack_32c = uStack_164;
      uStack_328 = uStack_160;
      uStack_324 = uStack_15c;
      uStack_320 = uStack_158;
      uStack_31c = uStack_154;
      uStack_318 = uStack_150;
      uStack_314 = uStack_14c;
      uStack_310 = uStack_148;
      uStack_30c = uStack_144;
      uStack_308 = uStack_140;
      uStack_304 = uStack_13c;
      uStack_300 = uStack_138;
      uStack_2f8 = uStack_130;
      uStack_2f0 = uStack_128;
      uStack_2e8 = uStack_120;
      uStack_360 = uStack_1d0;
      uStack_370 = uStack_1e0;
      uStack_36c = uStack_1dc;
      uStack_368 = uStack_1d8;
      uStack_364 = uStack_1d4;
      uStack_2d8 = uStack_a8;
      lStack_2e0 = lStack_f0;
      uStack_358 = 1;
      uStack_354 = (undefined4)uVar6;
      uStack_350 = (undefined4)((ulonglong)uVar6 >> 0x20);
      uStack_34c = 1;
      FUN_1409dd4d0(&puStack_98,&uStack_3b8);
      if (cStack_f2 != '\b') {
        uStack_41 = 0;
        FUN_140be24b0(auStack_108);
      }
      *(undefined4 *)(param_1 + 4) = *(undefined4 *)(param_5 + 4);
      uVar6 = *param_5;
      uVar3 = param_5[1];
      uVar4 = param_5[3];
      param_1[2] = param_5[2];
      param_1[3] = uVar4;
      *param_1 = uVar6;
      param_1[1] = uVar3;
      return param_1;
    }
  }
  FUN_1428d9310(&UNK_142acd478);
  pcVar2 = (code *)swi(3);
  puVar7 = (undefined8 *)(*pcVar2)();
  return puVar7;
}

