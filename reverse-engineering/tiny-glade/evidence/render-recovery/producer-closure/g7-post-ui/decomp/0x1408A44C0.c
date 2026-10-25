
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void FUN_1408a44c0(undefined8 param_1,longlong param_2,longlong param_3,longlong param_4)

{
  char cVar1;
  code *pcVar2;
  longlong lVar3;
  undefined8 uVar4;
  ulonglong uVar5;
  undefined4 uVar6;
  undefined4 uVar7;
  longlong lStack_518;
  undefined8 uStack_510;
  undefined8 uStack_508;
  undefined8 uStack_500;
  undefined8 uStack_4f8;
  undefined8 uStack_4f0;
  undefined8 uStack_4e8;
  undefined8 uStack_4e0;
  undefined8 uStack_4d8;
  undefined8 uStack_4d0;
  undefined8 uStack_388;
  undefined8 uStack_380;
  undefined8 uStack_378;
  undefined8 uStack_370;
  undefined8 uStack_368;
  undefined8 uStack_360;
  undefined8 uStack_358;
  undefined8 uStack_350;
  undefined8 uStack_348;
  undefined4 uStack_340;
  undefined4 uStack_33c;
  undefined4 uStack_338;
  undefined4 uStack_334;
  undefined8 uStack_330;
  undefined4 uStack_328;
  undefined4 uStack_324;
  undefined4 uStack_320;
  undefined4 uStack_31c;
  undefined8 uStack_310;
  undefined8 uStack_308;
  undefined4 uStack_300;
  undefined4 uStack_2fc;
  undefined4 uStack_2f8;
  undefined4 uStack_2f4;
  undefined4 uStack_2f0;
  undefined4 uStack_2ec;
  undefined4 uStack_2e8;
  undefined4 uStack_2e4;
  undefined4 uStack_2e0;
  undefined4 uStack_2dc;
  undefined4 uStack_2d8;
  undefined4 uStack_2d4;
  undefined8 uStack_2d0;
  undefined8 uStack_2c8;
  undefined8 uStack_2c0;
  undefined8 uStack_2b8;
  longlong lStack_2b0;
  undefined8 uStack_2a8;
  undefined1 auStack_278 [38];
  char cStack_252;
  longlong lStack_250;
  undefined8 uStack_1f8;
  undefined8 uStack_1f0;
  undefined8 uStack_1e8;
  undefined8 uStack_1e0;
  undefined8 uStack_1d8;
  undefined8 uStack_1d0;
  undefined8 uStack_1c8;
  undefined8 uStack_1c0;
  undefined8 uStack_1b8;
  undefined4 uStack_1b0;
  undefined4 uStack_1ac;
  undefined4 uStack_1a8;
  undefined4 uStack_1a4;
  undefined8 uStack_1a0;
  longlong lStack_198;
  undefined8 uStack_190;
  undefined8 uStack_188;
  undefined8 uStack_180;
  undefined8 uStack_178;
  undefined8 uStack_170;
  undefined8 uStack_168;
  undefined8 uStack_160;
  undefined8 uStack_158;
  undefined8 uStack_150;
  undefined8 uStack_148;
  undefined8 uStack_140;
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
  undefined4 uStack_110;
  undefined4 uStack_10c;
  undefined8 uStack_108;
  undefined8 uStack_100;
  undefined8 uStack_f8;
  undefined8 uStack_f0;
  undefined1 auStack_e8 [16];
  undefined1 auStack_d8 [22];
  char cStack_c2;
  longlong lStack_c0;
  undefined8 uStack_78;
  undefined1 uStack_61;
  undefined8 uStack_60;
  
  uStack_60 = 0xfffffffffffffffe;
  uVar5 = 0xbba451ee6a08a3ff;
  lVar3 = FUN_1400dd6f0(param_2 + 0x38,0xbba451ee6a08a3ff,0x8d008d15febcf29d);
  if (((lVar3 != 0) && (uVar5 < *(ulonglong *)(param_2 + 0x218))) &&
     (uVar5 = *(ulonglong *)(*(longlong *)(param_2 + 0x210) + uVar5 * 8), uVar5 != 0)) {
    lVar3 = ~uVar5 * 0x60;
    if (*(longlong *)(*(longlong *)(param_2 + 0x1e0) + 0x20 + lVar3) != 0) {
      lVar3 = *(longlong *)(*(longlong *)(param_2 + 0x1e0) + lVar3 + 0x10);
      uVar4 = func_0x000140bd5790(param_3 + 4);
      cVar1 = *(char *)(lVar3 + 0x4d0);
      if ((cVar1 == '\x04') || (uVar6 = _UNK_142925904, uVar7 = _UNK_142925904, cVar1 == '\x01')) {
        uVar6 = _UNK_1429345a8;
        uVar7 = _UNK_142a9abd4;
      }
      FUN_1416032a0(&uStack_1f8,&UNK_142a9b388,0xb,&UNK_142a9b393,0x1a,param_2,param_1);
      if (cStack_c2 != '\b') {
        FUN_14096b850(auStack_e8,lStack_c0 + 0xe8,0x418a98441f59a54c,1);
      }
      func_0x000142923c00(&uStack_388,&uStack_1f8,400);
      uStack_61 = 0x11;
      lStack_518 = CONCAT44(lStack_518._4_4_,2);
      FUN_140905260(&uStack_61,&uStack_388,0x11651f4dc9841aa4,0x19,&lStack_518);
      func_0x000142923c00(&lStack_518,&uStack_388,400);
      uStack_1f8 = 0x579f39ab91a01865;
      uStack_388._0_4_ = 2;
      FUN_1409056d0(&uStack_1f8,&lStack_518,0x815bf5036966978c,0x19,&uStack_388);
      func_0x000142923c00(&uStack_388,&lStack_518,400);
      uStack_1f8 = CONCAT44(uStack_1f8._4_4_,2);
      lStack_518 = param_3;
      FUN_1408edd00(&lStack_518,&uStack_388,0xd9978e541ce9a95e,0x32,&uStack_1f8);
      func_0x000142923c00(&uStack_1f8,&uStack_388,400);
      uStack_388 = CONCAT44(uStack_388._4_4_,2);
      lStack_518 = param_4;
      FUN_1408ee240(&lStack_518,&uStack_1f8,0x4861579da049fbfc,0x19,&uStack_388);
      func_0x000142923c00(&uStack_388,&uStack_1f8,400);
      if (cStack_252 != '\b') {
        FUN_14096b570(auStack_278,lStack_250 + 0xe8,0xc5ab673fb08651f5,0);
      }
      func_0x000142923c00(&uStack_1f8,&uStack_388,400);
      if (cStack_c2 != '\b') {
        FUN_14096ba30(auStack_e8,lStack_c0 + 0xe8,0xc6e199844f1748b4,uVar7);
      }
      func_0x000142923c00(&uStack_388,&uStack_1f8,400);
      if (cStack_252 != '\b') {
        FUN_14096ba30(auStack_278,lStack_250 + 0xe8,0x47671f36b27a58e7,uVar6);
      }
      func_0x000142923c00(&uStack_1f8,&uStack_388,400);
      uStack_4d8 = uStack_158;
      uStack_4d0 = uStack_150;
      uStack_4e8 = uStack_168;
      uStack_4e0 = uStack_160;
      uStack_4f8 = uStack_178;
      uStack_4f0 = uStack_170;
      uStack_508 = uStack_188;
      uStack_500 = uStack_180;
      lStack_518 = lStack_198;
      uStack_510 = uStack_190;
      uStack_378 = uStack_1e8;
      uStack_388 = uStack_1f8;
      uStack_380 = uStack_1f0;
      uStack_358 = uStack_1e0;
      uStack_350 = uStack_1d8;
      uStack_348 = uStack_1d0;
      uStack_370 = uStack_1c8;
      uStack_368 = uStack_1c0;
      uStack_360 = uStack_1b8;
      uStack_310 = uStack_148;
      uStack_308 = uStack_140;
      uStack_300 = uStack_138;
      uStack_2fc = uStack_134;
      uStack_2f8 = uStack_130;
      uStack_2f4 = uStack_12c;
      uStack_2f0 = uStack_128;
      uStack_2ec = uStack_124;
      uStack_2e8 = uStack_120;
      uStack_2e4 = uStack_11c;
      uStack_2e0 = uStack_118;
      uStack_2dc = uStack_114;
      uStack_2d8 = uStack_110;
      uStack_2d4 = uStack_10c;
      uStack_2d0 = uStack_108;
      uStack_2c8 = uStack_100;
      uStack_2c0 = uStack_f8;
      uStack_2b8 = uStack_f0;
      uStack_330 = uStack_1a0;
      uStack_340 = uStack_1b0;
      uStack_33c = uStack_1ac;
      uStack_338 = uStack_1a8;
      uStack_334 = uStack_1a4;
      uStack_2a8 = uStack_78;
      lStack_2b0 = lStack_c0;
      uStack_328 = 1;
      uStack_324 = (undefined4)uVar4;
      uStack_320 = (undefined4)((ulonglong)uVar4 >> 0x20);
      uStack_31c = 1;
      FUN_1409dd4d0(&lStack_518,&uStack_388);
      if (cStack_c2 != '\b') {
        FUN_140be24b0(auStack_d8);
      }
      return;
    }
  }
  FUN_1428d9310(&UNK_142a9b370);
  pcVar2 = (code *)swi(3);
  (*pcVar2)();
  return;
}

