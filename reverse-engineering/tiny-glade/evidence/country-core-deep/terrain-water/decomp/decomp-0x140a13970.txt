
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void FUN_140a13970(undefined8 param_1,longlong param_2)

{
  code *pcVar1;
  longlong lVar2;
  ulonglong uVar3;
  undefined8 uStack_4e8;
  undefined8 uStack_4e0;
  undefined8 uStack_4d8;
  undefined8 uStack_4d0;
  undefined8 uStack_4c8;
  undefined8 uStack_4c0;
  undefined8 uStack_4b8;
  undefined8 uStack_4b0;
  undefined8 uStack_4a8;
  undefined8 uStack_4a0;
  undefined8 uStack_498;
  undefined8 uStack_490;
  undefined8 uStack_488;
  undefined8 uStack_480;
  undefined8 uStack_478;
  undefined8 uStack_470;
  undefined8 uStack_468;
  undefined8 uStack_460;
  undefined8 uStack_458;
  undefined8 uStack_450;
  undefined8 uStack_448;
  undefined8 uStack_440;
  undefined8 uStack_438;
  undefined8 uStack_430;
  undefined4 uStack_428;
  undefined4 uStack_424;
  undefined4 uStack_420;
  undefined4 uStack_41c;
  undefined4 uStack_418;
  undefined4 uStack_414;
  undefined4 uStack_410;
  undefined4 uStack_40c;
  undefined4 uStack_408;
  undefined4 uStack_404;
  undefined4 uStack_400;
  undefined4 uStack_3fc;
  undefined8 uStack_3f8;
  undefined8 uStack_3f0;
  undefined8 uStack_3e8;
  undefined8 uStack_3e0;
  undefined1 auStack_3c8 [22];
  char cStack_3b2;
  undefined8 uStack_3b0;
  undefined8 uStack_368;
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
  undefined8 uStack_2c0;
  undefined8 uStack_2b8;
  undefined8 uStack_2b0;
  undefined8 uStack_2a8;
  undefined8 uStack_2a0;
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
  undefined8 uStack_258;
  undefined8 uStack_250;
  undefined1 auStack_248 [16];
  undefined1 auStack_238 [22];
  char cStack_222;
  longlong lStack_220;
  undefined8 uStack_1d8;
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
  undefined8 uStack_150;
  undefined8 uStack_148;
  undefined4 uStack_140;
  undefined4 uStack_13c;
  undefined4 uStack_138;
  undefined4 uStack_134;
  undefined4 uStack_130;
  undefined4 uStack_12c;
  undefined4 uStack_128;
  undefined4 uStack_124;
  undefined4 uStack_120;
  undefined4 uStack_11c;
  undefined4 uStack_118;
  undefined4 uStack_114;
  undefined8 uStack_110;
  undefined8 uStack_108;
  undefined8 uStack_100;
  undefined8 uStack_f8;
  longlong lStack_f0;
  undefined8 uStack_e8;
  undefined1 auStack_b8 [38];
  char cStack_92;
  longlong lStack_90;
  undefined1 uStack_32;
  undefined1 uStack_31;
  undefined8 uStack_30;
  
  uStack_30 = 0xfffffffffffffffe;
  if (cRam0000000143634d90 != '\0') {
    return;
  }
  FUN_1416032a0(&uStack_1c8,&UNK_142ac7cfe,0xe,&UNK_142ac7d0c,0x1d,param_2,param_1);
  FUN_141606560(&uStack_358,&uStack_1c8,0x1de1282bcdffb385);
  FUN_141605ff0(&uStack_1c8,&uStack_358,0xd0bb0d2aa1cf854b);
  uStack_32 = 1;
  uVar3 = 0x74430dfcc6d8719b;
  lVar2 = FUN_1400dd6f0(param_2 + 0x38,0x74430dfcc6d8719b,0xca5591f7e464509c);
  if (((lVar2 != 0) && (uVar3 < *(ulonglong *)(param_2 + 0x218))) &&
     (uVar3 = *(ulonglong *)(*(longlong *)(param_2 + 0x210) + uVar3 * 8), uVar3 != 0)) {
    lVar2 = ~uVar3 * 0x60;
    if (*(longlong *)(*(longlong *)(param_2 + 0x1e0) + 0x20 + lVar2) != 0) {
      if (cStack_92 != '\b') {
        FUN_14096b850(auStack_b8,lStack_90 + 0xe8,0xeaa3e0c70b923afc,
                      *(char *)(*(longlong *)(*(longlong *)(param_2 + 0x1e0) + lVar2 + 0x10) + 0x58)
                      == '\x03');
      }
      func_0x000142923c00(&uStack_358,&uStack_1c8,400);
      if (cStack_222 != '\b') {
        FUN_14096b760(auStack_248,lStack_220 + 0xe8,0xfb539f7243dbb832,0xe1000000e1);
      }
      func_0x000142923c00(&uStack_4e8,&uStack_358,400);
      uStack_358 = CONCAT71(uStack_358._1_7_,9);
      uStack_1c8._0_4_ = 2;
      FUN_140905260(&uStack_358,&uStack_4e8,0x61b08b5e0470c5f2,0x19,&uStack_1c8);
      func_0x000142923c00(&uStack_1c8,&uStack_4e8,400);
      uStack_31 = 10;
      uStack_358._0_4_ = 2;
      FUN_140905260(&uStack_31,&uStack_1c8,0x297d1e4cdae8f657,0x19,&uStack_358);
      func_0x000142923c00(&uStack_358,&uStack_1c8,400);
      uStack_4e8._0_1_ = 0x13;
      uStack_1c8._0_4_ = 2;
      FUN_140905260(&uStack_4e8,&uStack_358,0x6ad63ccd18c8f041,0x19,&uStack_1c8);
      func_0x000142923c00(&uStack_1c8,&uStack_358,400);
      uStack_4e8._0_1_ = 5;
      uStack_358._0_4_ = 2;
      FUN_140905260(&uStack_4e8,&uStack_1c8,0xed502fff6dc4021f,0x19,&uStack_358);
      func_0x000142923c00(&uStack_358,&uStack_1c8,400);
      uStack_4e8._0_1_ = 0x14;
      uStack_1c8 = CONCAT44(uStack_1c8._4_4_,2);
      FUN_140905260(&uStack_4e8,&uStack_358,0x49f43a487883738d,0x19,&uStack_1c8);
      func_0x000142923c00(&uStack_1c8,&uStack_358,400);
      uStack_4e8 = CONCAT71(uStack_4e8._1_7_,0x17);
      uStack_358 = CONCAT44(uStack_358._4_4_,2);
      FUN_140905260(&uStack_4e8,&uStack_1c8,0x9140004c9a5b7b9d,0x32,&uStack_358);
      func_0x000142923c00(&uStack_358,&uStack_1c8,400);
      FUN_141608c10(&uStack_1c8,&uStack_358,0xf1c07a2b8012765a,1);
      if (cStack_92 != '\b') {
        FUN_14096b570(auStack_b8,lStack_90 + 0xe8,0xe9511a616b75789,8);
      }
      func_0x000142923c00(&uStack_4e8,&uStack_1c8,400);
      uStack_318 = uStack_448;
      uStack_310 = uStack_440;
      uStack_328 = uStack_458;
      uStack_320 = uStack_450;
      uStack_338 = uStack_468;
      uStack_330 = uStack_460;
      uStack_348 = uStack_478;
      uStack_340 = uStack_470;
      uStack_358 = uStack_488;
      uStack_350 = uStack_480;
      uStack_1b8 = uStack_4d8;
      uStack_1c8 = uStack_4e8;
      uStack_1c0 = uStack_4e0;
      uStack_198 = uStack_4d0;
      uStack_190 = uStack_4c8;
      uStack_188 = uStack_4c0;
      uStack_1b0 = uStack_4b8;
      uStack_1a8 = uStack_4b0;
      uStack_1a0 = uStack_4a8;
      uStack_150 = uStack_438;
      uStack_148 = uStack_430;
      uStack_140 = uStack_428;
      uStack_13c = uStack_424;
      uStack_138 = uStack_420;
      uStack_134 = uStack_41c;
      uStack_130 = uStack_418;
      uStack_12c = uStack_414;
      uStack_128 = uStack_410;
      uStack_124 = uStack_40c;
      uStack_120 = uStack_408;
      uStack_11c = uStack_404;
      uStack_118 = uStack_400;
      uStack_114 = uStack_3fc;
      uStack_110 = uStack_3f8;
      uStack_108 = uStack_3f0;
      uStack_100 = uStack_3e8;
      uStack_f8 = uStack_3e0;
      uStack_170 = uStack_490;
      uStack_180 = uStack_4a0;
      uStack_178 = uStack_498;
      uStack_e8 = uStack_368;
      lStack_f0 = uStack_3b0;
      uStack_168 = _UNK_142ac70a0;
      uStack_164 = _UNK_142ac70a4;
      uStack_160 = _UNK_142ac70a8;
      uStack_15c = _UNK_142ac70ac;
      FUN_1409dd4d0(&uStack_358,&uStack_1c8);
      if (cStack_3b2 != '\b') {
        FUN_140be24b0(auStack_3c8);
      }
      FUN_1416032a0(&uStack_1c8,&UNK_142ac7d48,0xb,&UNK_142ac7d53,0x1a,param_2,param_1);
      FUN_141606ad0(&uStack_358,&uStack_1c8,0x6d9256d4ac7887ad);
      if (cStack_222 != '\b') {
        FUN_14096b760(auStack_248,lStack_220 + 0xe8,0xfb539f7243dbb832,0x9600000096);
      }
      func_0x000142923c00(&uStack_4e8,&uStack_358,400);
      uStack_358 = CONCAT71(uStack_358._1_7_,9);
      uStack_1c8._0_4_ = 2;
      FUN_140905260(&uStack_358,&uStack_4e8,0x61b08b5e0470c5f2,0x19,&uStack_1c8);
      func_0x000142923c00(&uStack_1c8,&uStack_4e8,400);
      uStack_31 = 10;
      uStack_358._0_4_ = 2;
      FUN_140905260(&uStack_31,&uStack_1c8,0x297d1e4cdae8f657,0x19,&uStack_358);
      func_0x000142923c00(&uStack_358,&uStack_1c8,400);
      uStack_4e8._0_1_ = 0x13;
      uStack_1c8._0_4_ = 2;
      FUN_140905260(&uStack_4e8,&uStack_358,0x6ad63ccd18c8f041,0x19,&uStack_1c8);
      func_0x000142923c00(&uStack_1c8,&uStack_358,400);
      uStack_4e8 = CONCAT71(uStack_4e8._1_7_,5);
      uStack_358 = CONCAT44(uStack_358._4_4_,2);
      FUN_140905260(&uStack_4e8,&uStack_1c8,0xed502fff6dc4021f,0x19,&uStack_358);
      func_0x000142923c00(&uStack_358,&uStack_1c8,400);
      uStack_31 = 0x16;
      uStack_1c8 = CONCAT44(uStack_1c8._4_4_,2);
      FUN_140905260(&uStack_31,&uStack_358,0x6570181649ac6420,0x19,&uStack_1c8);
      func_0x000142923c00(&uStack_4e8,&uStack_358,400);
      FUN_141608c10(&uStack_1c8,&uStack_4e8,0xf1c07a2b8012765a,1);
      if (cStack_92 != '\b') {
        FUN_14096b570(auStack_b8,lStack_90 + 0xe8,0xe9511a616b75789,9);
      }
      func_0x000142923c00(&uStack_358,&uStack_1c8,400);
      uStack_4a8 = uStack_2b8;
      uStack_4a0 = uStack_2b0;
      uStack_4b8 = uStack_2c8;
      uStack_4b0 = uStack_2c0;
      uStack_4c8 = uStack_2d8;
      uStack_4c0 = uStack_2d0;
      uStack_4d8 = uStack_2e8;
      uStack_4d0 = uStack_2e0;
      uStack_4e8 = uStack_2f8;
      uStack_4e0 = uStack_2f0;
      uStack_1b8 = uStack_348;
      uStack_1c8 = uStack_358;
      uStack_1c0 = uStack_350;
      uStack_198 = uStack_340;
      uStack_190 = uStack_338;
      uStack_188 = uStack_330;
      uStack_1b0 = uStack_328;
      uStack_1a8 = uStack_320;
      uStack_1a0 = uStack_318;
      uStack_150 = uStack_2a8;
      uStack_148 = uStack_2a0;
      uStack_140 = uStack_298;
      uStack_13c = uStack_294;
      uStack_138 = uStack_290;
      uStack_134 = uStack_28c;
      uStack_130 = uStack_288;
      uStack_12c = uStack_284;
      uStack_128 = uStack_280;
      uStack_124 = uStack_27c;
      uStack_120 = uStack_278;
      uStack_11c = uStack_274;
      uStack_118 = uStack_270;
      uStack_114 = uStack_26c;
      uStack_110 = uStack_268;
      uStack_108 = uStack_260;
      uStack_100 = uStack_258;
      uStack_f8 = uStack_250;
      uStack_170 = uStack_300;
      uStack_180 = uStack_310;
      uStack_178 = uStack_308;
      uStack_e8 = uStack_1d8;
      lStack_f0 = lStack_220;
      uStack_168 = _UNK_142aa5130;
      uStack_164 = _UNK_142aa5134;
      uStack_160 = _UNK_142aa5138;
      uStack_15c = _UNK_142aa513c;
      FUN_1409dd4d0(&uStack_4e8,&uStack_1c8);
      if (cStack_222 == '\b') {
        return;
      }
      FUN_140be24b0(auStack_238);
      return;
    }
  }
  uStack_32 = 1;
  FUN_1428d9310(&UNK_142ac7d30);
                    /* WARNING: Does not return */
  pcVar1 = (code *)invalidInstructionException();
  (*pcVar1)();
}

