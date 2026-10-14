
void FUN_140a3ee80(undefined8 param_1,longlong param_2)

{
  code *pcVar1;
  longlong lVar2;
  undefined4 *puVar3;
  ulonglong uVar4;
  undefined4 uVar5;
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
  undefined4 uStack_358;
  undefined4 uStack_354;
  undefined8 uStack_350;
  undefined8 uStack_348;
  undefined8 uStack_340;
  undefined8 uStack_338;
  undefined8 uStack_330;
  undefined8 uStack_328;
  undefined8 uStack_320;
  undefined8 uStack_318;
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
  undefined1 auStack_238 [22];
  char cStack_222;
  undefined8 uStack_220;
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
  longlong lStack_180;
  undefined4 *puStack_178;
  undefined8 uStack_170;
  undefined4 uStack_168;
  undefined4 uStack_164;
  undefined8 uStack_160;
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
  undefined8 uStack_f0;
  undefined8 uStack_e8;
  undefined1 uStack_31;
  undefined8 uStack_30;
  
  uStack_30 = 0xfffffffffffffffe;
  uVar4 = 0xfa44538b95ae76c6;
  lVar2 = FUN_1400dd6f0(param_2 + 0x38,0xfa44538b95ae76c6,0x12146c4ea814069b);
  if (((lVar2 != 0) && (uVar4 < *(ulonglong *)(param_2 + 0x218))) &&
     (uVar4 = *(ulonglong *)(*(longlong *)(param_2 + 0x210) + uVar4 * 8), uVar4 != 0)) {
    lVar2 = ~uVar4 * 0x60;
    if (*(longlong *)(*(longlong *)(param_2 + 0x1e0) + 0x20 + lVar2) != 0) {
      uVar4 = **(ulonglong **)(*(longlong *)(param_2 + 0x1e0) + lVar2 + 0x10);
      FUN_1416032a0(&uStack_1c8,&UNK_142acdeb8,0x16,&UNK_142acdece,0x2d,param_2,param_1);
      FUN_141605c50(&uStack_358,&uStack_1c8,0x12ef39a9447f1f0e);
      FUN_141604c00(&uStack_4e8,&uStack_358,0x480a6146ae76b082);
      uStack_358 = CONCAT31(uStack_358._1_3_,9);
      uStack_1c8._0_4_ = 2;
      FUN_140905260(&uStack_358,&uStack_4e8,0x61b08b5e0470c5f2,0x19,&uStack_1c8);
      func_0x000142923c00(&uStack_1c8,&uStack_4e8,400);
      uStack_31 = 10;
      uStack_358 = 2;
      FUN_140905260(&uStack_31,&uStack_1c8,0x297d1e4cdae8f657,0x19,&uStack_358);
      func_0x000142923c00(&uStack_358,&uStack_1c8,400);
      uStack_4e8._0_1_ = 0xe;
      uStack_1c8._0_4_ = 2;
      FUN_140905260(&uStack_4e8,&uStack_358,0x4ee098b38612d2b9,0x19,&uStack_1c8);
      func_0x000142923c00(&uStack_1c8,&uStack_358,400);
      uStack_4e8._0_1_ = 5;
      uStack_358 = 2;
      FUN_140905260(&uStack_4e8,&uStack_1c8,0xed502fff6dc4021f,0x19,&uStack_358);
      func_0x000142923c00(&uStack_358,&uStack_1c8,400);
      uStack_4e8 = CONCAT71(uStack_4e8._1_7_,0x13);
      uStack_1c8 = CONCAT44(uStack_1c8._4_4_,2);
      FUN_140905260(&uStack_4e8,&uStack_358,0x6ad63ccd18c8f041,0x19,&uStack_1c8);
      func_0x000142923c00(&uStack_1c8,&uStack_358,400);
      puVar3 = (undefined4 *)func_0x000140613c10(4,1);
      if (puVar3 != (undefined4 *)0x0) {
        uVar5 = (undefined4)(uVar4 / 3);
        *puVar3 = uVar5;
        if (lStack_180 != 0) {
          func_0x000140613c20(puStack_178,lStack_180,1);
        }
        lStack_180 = 4;
        uStack_170 = 4;
        puStack_178 = puVar3;
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
        uStack_1c8 = CONCAT44(uStack_354,uStack_358);
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
        uStack_e8 = uStack_1d8;
        uStack_f0 = uStack_220;
        uStack_168 = 1;
        uStack_160 = 0x100000001;
        uStack_164 = uVar5;
        FUN_1409dd4d0(&uStack_4e8,&uStack_1c8);
        if (cStack_222 != '\b') {
          FUN_140be24b0(auStack_238);
        }
        return;
      }
      goto code_r0x000140a3f2ec;
    }
  }
  FUN_1428d9310(&UNK_142acdea0);
code_r0x000140a3f2ec:
  FUN_1428d8fe3(1,4,&UNK_142c7d8d0);
                    /* WARNING: Does not return */
  pcVar1 = (code *)invalidInstructionException();
  (*pcVar1)();
}

