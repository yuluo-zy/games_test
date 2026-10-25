
void FUN_1408f4d00(undefined8 param_1,longlong param_2)

{
  code *pcVar1;
  longlong lVar2;
  ulonglong uVar3;
  ulonglong uVar4;
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
  undefined8 uStack_350;
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
  undefined8 uStack_228;
  undefined8 uStack_220;
  undefined8 uStack_218;
  undefined8 uStack_210;
  undefined8 uStack_208;
  undefined8 uStack_200;
  undefined8 uStack_1f8;
  undefined8 uStack_1f0;
  undefined8 uStack_1e8;
  undefined8 uStack_1e0;
  undefined8 uStack_1d0;
  undefined8 uStack_1c8;
  undefined8 uStack_1c0;
  undefined8 uStack_1b8;
  undefined8 uStack_1b0;
  undefined8 uStack_1a8;
  undefined8 uStack_1a0;
  undefined8 uStack_198;
  undefined8 uStack_190;
  undefined4 uStack_188;
  undefined4 uStack_184;
  undefined4 uStack_180;
  undefined4 uStack_17c;
  undefined8 uStack_178;
  undefined8 uStack_170;
  undefined8 uStack_168;
  undefined8 uStack_160;
  undefined8 uStack_158;
  undefined8 uStack_150;
  undefined8 uStack_148;
  undefined8 uStack_140;
  undefined8 uStack_138;
  undefined8 uStack_130;
  undefined8 uStack_128;
  undefined8 uStack_120;
  undefined8 uStack_118;
  undefined4 uStack_110;
  undefined4 uStack_10c;
  undefined4 uStack_108;
  undefined4 uStack_104;
  undefined4 uStack_100;
  undefined4 uStack_fc;
  undefined4 uStack_f8;
  undefined4 uStack_f4;
  undefined4 uStack_f0;
  undefined4 uStack_ec;
  undefined4 uStack_e8;
  undefined4 uStack_e4;
  undefined8 uStack_e0;
  undefined8 uStack_d8;
  undefined8 uStack_d0;
  undefined8 uStack_c8;
  undefined1 auStack_c0 [16];
  undefined1 auStack_b0 [22];
  char cStack_9a;
  longlong lStack_98;
  undefined8 uStack_50;
  undefined1 uStack_39;
  undefined8 uStack_38;
  
  uStack_38 = 0xfffffffffffffffe;
  uVar3 = 0x3dce36237fb50dff;
  lVar2 = FUN_1400dd6f0(param_2 + 0x38,0x3dce36237fb50dff,0x93a27cf4512d885c);
  if (((lVar2 != 0) && (uVar3 < *(ulonglong *)(param_2 + 0x218))) &&
     (uVar3 = *(ulonglong *)(*(longlong *)(param_2 + 0x210) + uVar3 * 8), uVar3 != 0)) {
    lVar2 = ~uVar3 * 0x60;
    if (*(longlong *)(*(longlong *)(param_2 + 0x1e0) + 0x20 + lVar2) != 0) {
      uVar3 = **(ulonglong **)(*(longlong *)(param_2 + 0x1e0) + lVar2 + 0x10);
      if (uVar3 == 0) {
        return;
      }
      uVar4 = 0x340;
      if (uVar3 < 0x340) {
        uVar4 = uVar3;
      }
      FUN_1416032a0(&uStack_3b8,&UNK_142aa82f8,0xd,&UNK_142aa8305,0x17,param_2,param_1);
      FUN_1416073e0(&uStack_1d0,&uStack_3b8,0x316dd88f93e7113);
      FUN_14160dbf0(&uStack_3b8,&uStack_1d0,0x1ad3d8a882713639);
      FUN_14160e330(&uStack_1d0,&uStack_3b8);
      if (cStack_9a != '\b') {
        FUN_14096b570(auStack_c0,lStack_98 + 0xe8,0xae67afb65e518cb4,uVar4 & 0xffffffff);
      }
      func_0x000142923c00(&uStack_3b8,&uStack_1d0,400);
      uStack_39 = 1;
      uVar3 = 0x4259a350202a056c;
      lVar2 = FUN_1400dd6f0(param_2 + 0x38,0x4259a350202a056c,0x1e9e3f559133d465);
      if (((lVar2 != 0) && (uVar3 < *(ulonglong *)(param_2 + 0x218))) &&
         (uVar3 = *(ulonglong *)(*(longlong *)(param_2 + 0x210) + uVar3 * 8), uVar3 != 0)) {
        lVar2 = ~uVar3 * 0x60;
        if (*(longlong *)(*(longlong *)(param_2 + 0x1e0) + 0x20 + lVar2) != 0) {
          if (cStack_282 != '\b') {
            FUN_14096b570(auStack_2a8,lStack_280 + 0xe8,0x6ae860ef74719a42,
                          **(undefined8 **)(*(longlong *)(param_2 + 0x1e0) + lVar2 + 0x10));
          }
          func_0x000142923c00(&uStack_1d0,&uStack_3b8,400);
          uStack_1e8 = uStack_130;
          uStack_1e0 = uStack_128;
          uStack_1f8 = uStack_140;
          uStack_1f0 = uStack_138;
          uStack_208 = uStack_150;
          uStack_200 = uStack_148;
          uStack_218 = uStack_160;
          uStack_210 = uStack_158;
          uStack_228 = uStack_170;
          uStack_220 = uStack_168;
          uStack_3a8 = uStack_1c0;
          uStack_3b8 = uStack_1d0;
          uStack_3b0 = uStack_1c8;
          uStack_388 = uStack_1b8;
          uStack_380 = uStack_1b0;
          uStack_378 = uStack_1a8;
          uStack_3a0 = uStack_1a0;
          uStack_398 = uStack_198;
          uStack_390 = uStack_190;
          uStack_340 = uStack_120;
          uStack_338 = uStack_118;
          uStack_330 = uStack_110;
          uStack_32c = uStack_10c;
          uStack_328 = uStack_108;
          uStack_324 = uStack_104;
          uStack_320 = uStack_100;
          uStack_31c = uStack_fc;
          uStack_318 = uStack_f8;
          uStack_314 = uStack_f4;
          uStack_310 = uStack_f0;
          uStack_30c = uStack_ec;
          uStack_308 = uStack_e8;
          uStack_304 = uStack_e4;
          uStack_300 = uStack_e0;
          uStack_2f8 = uStack_d8;
          uStack_2f0 = uStack_d0;
          uStack_2e8 = uStack_c8;
          uStack_360 = uStack_178;
          uStack_370 = uStack_188;
          uStack_36c = uStack_184;
          uStack_368 = uStack_180;
          uStack_364 = uStack_17c;
          uStack_2d8 = uStack_50;
          lStack_2e0 = lStack_98;
          uStack_358 = 1;
          uStack_350 = 0x100000001;
          uStack_354 = (int)uVar4;
          FUN_1409dd4d0(&uStack_228,&uStack_3b8);
          if (cStack_9a == '\b') {
            return;
          }
          FUN_140be24b0(auStack_b0);
          return;
        }
      }
      goto code_r0x0001408f5078;
    }
  }
  FUN_1428d9310(&UNK_142aa82e0);
code_r0x0001408f5078:
  uStack_39 = 1;
  FUN_1428d9310(&UNK_142aa8320);
                    /* WARNING: Does not return */
  pcVar1 = (code *)invalidInstructionException();
  (*pcVar1)();
}

