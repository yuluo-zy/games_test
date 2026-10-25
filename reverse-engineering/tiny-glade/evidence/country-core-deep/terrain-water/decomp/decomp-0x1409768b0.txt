
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void FUN_1409768b0(undefined8 param_1,longlong param_2)

{
  byte *pbVar1;
  code *pcVar2;
  longlong lVar3;
  undefined4 *puVar4;
  ulonglong uVar5;
  undefined4 uVar6;
  undefined4 uVar7;
  undefined8 uStack_3b8;
  undefined8 uStack_3b0;
  undefined8 uStack_3a8;
  undefined8 uStack_3a0;
  undefined8 uStack_398;
  undefined8 uStack_390;
  undefined8 uStack_388;
  undefined8 uStack_380;
  undefined8 uStack_378;
  longlong lStack_370;
  undefined4 *puStack_368;
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
  undefined8 uStack_2e0;
  undefined8 uStack_2d8;
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
  undefined1 auStack_108 [22];
  char cStack_f2;
  undefined8 uStack_f0;
  undefined8 uStack_a8;
  undefined8 uStack_98;
  undefined8 uStack_90;
  undefined8 uStack_88;
  undefined8 uStack_80;
  undefined8 uStack_78;
  undefined8 uStack_70;
  undefined8 uStack_68;
  undefined8 uStack_60;
  undefined8 uStack_58;
  undefined8 uStack_50;
  undefined8 uStack_40;
  
  uStack_40 = 0xfffffffffffffffe;
  uVar5 = 0x62dfe8f62b6fcc5;
  lVar3 = FUN_1400dd6f0(param_2 + 0x38,0x62dfe8f62b6fcc5,0xf65008e548632ba7);
  if (((lVar3 != 0) && (uVar5 < *(ulonglong *)(param_2 + 0x218))) &&
     (uVar5 = *(ulonglong *)(*(longlong *)(param_2 + 0x210) + uVar5 * 8), uVar5 != 0)) {
    lVar3 = ~uVar5 * 0x60;
    if (*(longlong *)(*(longlong *)(param_2 + 0x1e0) + 0x20 + lVar3) != 0) {
      pbVar1 = *(byte **)(*(longlong *)(param_2 + 0x1e0) + lVar3 + 0x10);
      uVar6 = 0;
      uVar7 = 0;
      if ((*pbVar1 & 1) != 0) {
        uVar6 = *(undefined4 *)(pbVar1 + 4);
        uVar7 = *(undefined4 *)(pbVar1 + 0xc);
      }
      FUN_1416032a0(&uStack_3b8,&UNK_142ab68e0,0x13,&UNK_142ab68f3,0x1c,param_2,param_1);
      FUN_1416093d0(&uStack_228,&uStack_3b8,0xa868b69644bd36ba);
      uStack_98 = CONCAT71(uStack_98._1_7_,0xb);
      uStack_3b8 = CONCAT44(uStack_3b8._4_4_,2);
      FUN_140905260(&uStack_98,&uStack_228,0xe3659c0fbde7bf03,0x19,&uStack_3b8);
      func_0x000142923c00(&uStack_3b8,&uStack_228,400);
      puVar4 = (undefined4 *)func_0x000140613c10(8,1);
      if (puVar4 != (undefined4 *)0x0) {
        *puVar4 = uVar6;
        puVar4[1] = uVar7;
        if (lStack_370 != 0) {
          func_0x000140613c20(puStack_368,lStack_370,1);
        }
        lStack_370 = 8;
        uStack_360 = 8;
        puStack_368 = puVar4;
        func_0x000142923c00(&uStack_228,&uStack_3b8,400);
        uStack_58 = uStack_188;
        uStack_50 = uStack_180;
        uStack_68 = uStack_198;
        uStack_60 = uStack_190;
        uStack_78 = uStack_1a8;
        uStack_70 = uStack_1a0;
        uStack_88 = uStack_1b8;
        uStack_80 = uStack_1b0;
        uStack_98 = uStack_1c8;
        uStack_90 = uStack_1c0;
        uStack_3a8 = uStack_218;
        uStack_3b8 = uStack_228;
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
        lStack_370 = uStack_1e0;
        puStack_368 = (undefined4 *)uStack_1d8;
        uStack_2d8 = uStack_a8;
        uStack_2e0 = uStack_f0;
        uStack_358 = _UNK_142958fe0;
        uStack_354 = _UNK_142958fe4;
        uStack_350 = _UNK_142958fe8;
        uStack_34c = _UNK_142958fec;
        FUN_1409dd4d0(&uStack_98,&uStack_3b8);
        if (cStack_f2 != '\b') {
          FUN_140be24b0(auStack_108);
        }
        return;
      }
      goto code_r0x000140976bb7;
    }
  }
  FUN_1428d9310(&UNK_142ab68c8);
code_r0x000140976bb7:
  FUN_1428d8fe3(1,8,&UNK_142c7d8d0);
                    /* WARNING: Does not return */
  pcVar2 = (code *)invalidInstructionException();
  (*pcVar2)();
}

