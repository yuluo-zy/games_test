/* WARNING: Type propagation algorithm not settling */
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */
undefined8
FUN_1408a82f0(undefined8 param_1,longlong param_2,ulonglong *param_3,undefined1 param_4,
             undefined1 param_5)
{
  ulonglong *puVar1;
  float *pfVar2;
  code *pcVar3;
  byte bVar4;
  ulonglong *puVar5;
  undefined1 uVar6;
  longlong lVar7;
  undefined8 uVar8;
  undefined4 *puVar9;
  longlong lVar10;
  ulonglong uVar11;
  ulonglong *puVar12;
  float fVar13;
  float fVar14;
  undefined4 uVar15;
  undefined8 uVar16;
  undefined4 uVar17;
  undefined8 uStack_408;
  undefined *puStack_400;
  undefined8 uStack_3f8;
  undefined8 uStack_3f0;
  undefined *puStack_3e8;
  undefined8 uStack_3e0;
  undefined8 uStack_3d8;
  undefined8 uStack_3d0;
  undefined8 uStack_3c8;
  undefined4 uStack_3c0;
  undefined4 uStack_3bc;
  undefined4 uStack_3b8;
  undefined4 uStack_3b4;
  undefined8 uStack_3b0;
  undefined4 uStack_3a8;
  undefined4 uStack_3a4;
  undefined4 uStack_3a0;
  undefined4 uStack_39c;
  undefined8 uStack_390;
  undefined8 uStack_388;
  undefined4 uStack_380;
  undefined4 uStack_37c;
  undefined4 uStack_378;
  undefined4 uStack_374;
  undefined4 uStack_370;
  undefined4 uStack_36c;
  undefined4 uStack_368;
  undefined4 uStack_364;
  undefined4 uStack_360;
  undefined4 uStack_35c;
  undefined4 uStack_358;
  undefined4 uStack_354;
  undefined8 uStack_350;
  undefined8 uStack_348;
  undefined8 uStack_340;
  undefined8 uStack_338;
  undefined8 uStack_330;
  undefined8 uStack_328;
  ulonglong *puStack_278;
  undefined8 uStack_270;
  undefined8 uStack_268;
  undefined8 uStack_260;
  undefined8 uStack_258;
  undefined8 uStack_250;
  undefined8 uStack_248;
  undefined8 uStack_240;
  undefined8 uStack_238;
  undefined8 uStack_230;
  undefined4 uStack_228;
  undefined4 uStack_224;
  undefined4 uStack_220;
  undefined4 uStack_21c;
  undefined4 uStack_218;
  undefined4 uStack_214;
  undefined4 uStack_210;
  undefined4 uStack_20c;
  undefined4 uStack_208;
  undefined8 uStack_200;
  undefined8 uStack_1f8;
  undefined8 uStack_1f0;
  undefined8 uStack_1e8;
  undefined4 uStack_1e0;
  undefined4 uStack_1dc;
  undefined4 uStack_1d8;
  undefined4 uStack_1d4;
  undefined8 uStack_1d0;
  undefined *puStack_1c8;
  undefined8 uStack_1c0;
  undefined4 uStack_1b8;
  undefined4 uStack_1b4;
  undefined8 uStack_1b0;
  undefined8 uStack_1a8;
  ulonglong *puStack_1a0;
  undefined8 uStack_198;
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
  undefined1 auStack_e0 [22];
  char cStack_ca;
  undefined8 uStack_c8;
  undefined8 uStack_80;
  float *pfStack_70;
  uint uStack_64;
  ulonglong *puStack_60;
  ulonglong *puStack_58;
  undefined1 uStack_4a;
  byte bStack_49;
  undefined8 uStack_48;
  uStack_48 = 0xfffffffffffffffe;
  lVar10 = param_2 + 0x38;
  uVar11 = 0xbe2e7c0aff5cc813;
  lVar7 = FUN_1400dd6f0(lVar10,0xbe2e7c0aff5cc813,0x8f46594e22f01d76);
  if (((lVar7 == 0) || (*(ulonglong *)(param_2 + 0x218) <= uVar11)) ||
     (uVar11 = *(ulonglong *)(*(longlong *)(param_2 + 0x210) + uVar11 * 8), uVar11 == 0)) {
code_r0x0001408a8b75:
    FUN_1428d9310(&UNK_142a9bad8);
code_r0x0001408a8b81:
    FUN_1428d9310(&UNK_142a9baf0);
  }
  else {
    lVar7 = ~uVar11 * 0x60;
    if (*(longlong *)(*(longlong *)(param_2 + 0x1e0) + 0x20 + lVar7) == 0)
    goto code_r0x0001408a8b75;
    lVar7 = *(longlong *)(param_2 + 0x1e0) + lVar7;
    pfVar2 = *(float **)(lVar7 + 0x10);
    *(undefined4 *)(lVar7 + 0x5c) = *(undefined4 *)(param_2 + 0x5b0);
    uVar11 = 0xa284155e68bfeb3f;
    lVar7 = FUN_1400dd6f0(lVar10,0xa284155e68bfeb3f,0x84a2b3441bc64a3a);
    if (((lVar7 == 0) || (*(ulonglong *)(param_2 + 0x218) <= uVar11)) ||
       (uVar11 = *(ulonglong *)(*(longlong *)(param_2 + 0x210) + uVar11 * 8), uVar11 == 0))
    goto code_r0x0001408a8b81;
    lVar7 = ~uVar11 * 0x60;
    if (*(longlong *)(*(longlong *)(param_2 + 0x1e0) + 0x20 + lVar7) == 0)
    goto code_r0x0001408a8b81;
    lVar7 = *(longlong *)(param_2 + 0x1e0) + lVar7;
    puStack_60 = *(ulonglong **)(lVar7 + 0x10);
    *(undefined4 *)(lVar7 + 0x5c) = *(undefined4 *)(param_2 + 0x5b0);
    uStack_64 = (uint)puStack_60[9];
    if (uStack_64 == 2) {
      uVar11 = 2;
      if (*(char *)(pfVar2 + 1) == '\0') {
        bRam0000000143634c9c = 0;
      }
      else {
        *puStack_60 = 0;
        *(undefined1 *)((longlong)puStack_60 + 0x3e) = 7;
        puStack_60[8] = 0;
        *(undefined4 *)(puStack_60 + 9) = 0;
        *(undefined2 *)((longlong)puStack_60 + 0x54) = 0;
      }
code_r0x0001408a8b5b:
      return CONCAT71((int7)(uVar11 >> 8),(int)uVar11 != 2);
    }
    pfStack_70 = pfVar2;
    puStack_58 = param_3;
    uStack_4a = param_4;
    uVar8 = func_0x000140bd5790((longlong)param_3 + 4);
    bVar4 = bRam0000000143634c9c;
    bRam0000000143634c9c = 1;
    func_0x000140bd52a0(&uStack_200,uVar8,0x25);
    func_0x000140bd5390(&uStack_408,&uStack_200);
    func_0x000140bd5370(&puStack_278,&uStack_408);
    FUN_14096c890(&uStack_200,param_1,&UNK_142a9bb08,0xe,&puStack_278);
    bStack_49 = (byte)uStack_1dc;
    if ((byte)uStack_1dc == 2) goto code_r0x0001408a8b99;
    uStack_208 = uStack_1e0;
    uStack_218 = (undefined4)uStack_1f0;
    uStack_214 = uStack_1f0._4_4_;
    uStack_210 = (undefined4)uStack_1e8;
    uStack_20c = uStack_1e8._4_4_;
    uStack_228 = (undefined4)uStack_200;
    uStack_224 = uStack_200._4_4_;
    uStack_220 = (undefined4)uStack_1f8;
    uStack_21c = uStack_1f8._4_4_;
    if (4 < *puRam000000014362c3a8) {
      puVar9 = (undefined4 *)func_0x00014067dd80(&UNK_142a9bb58);
      uStack_1e0 = *puVar9;
      uStack_1dc = puVar9[1];
      uStack_1d8 = puVar9[2];
      uStack_1d4 = puVar9[3];
      uStack_1b4 = puVar9[4];
      uStack_1d0 = 5;
      puStack_1c8 = &UNK_142a9bb70;
      uStack_1c0 = 0x2b;
      uStack_1b0 = &UNK_142a9bb48;
      uStack_1a8 = 1;
      puStack_1a0 = (ulonglong *)0x8;
      uStack_200 = 0;
      uStack_198 = 0;
      uStack_190 = 0;
      uStack_1f8 = &UNK_142a9bb70;
      uStack_1f0 = 0x2b;
      uStack_1e8 = 0;
      uStack_1b8 = 1;
      func_0x000140d4a330(&uStack_408,&uStack_200);
    }
    lVar7 = param_2;
    uVar16 = param_1;
    FUN_1416032a0(&uStack_200,&UNK_142a9bb9b,0x16,&UNK_142a9bbb1,0x1f,param_2,param_1);
    uVar15 = (undefined4)((ulonglong)lVar7 >> 0x20);
    uVar17 = (undefined4)((ulonglong)uVar16 >> 0x20);
    puStack_278 = puStack_58;
    uStack_408 = CONCAT44(uStack_408._4_4_,2);
    FUN_1408ee240(&puStack_278,&uStack_200,0xd9978e541ce9a95e,0x19,&uStack_408);
    func_0x000142923c00(&uStack_408,&uStack_200,400);
    puStack_278 = (ulonglong *)&uStack_228;
    uStack_200 = CONCAT44(uStack_200._4_4_,2);
    FUN_1408edd00(&puStack_278,&uStack_408,0xed29cd643db266ee,0x32,&uStack_200);
    func_0x000142923c00(&uStack_200,&uStack_408,400);
    uStack_238 = uStack_160;
    uStack_230 = uStack_158;
    uStack_248 = uStack_170;
    uStack_240 = uStack_168;
    uStack_258 = uStack_180;
    uStack_250 = uStack_178;
    uStack_268 = uStack_190;
    uStack_260 = uStack_188;
    puStack_278 = puStack_1a0;
    uStack_270 = uStack_198;
    uStack_3f8 = uStack_1f0;
    uStack_408 = uStack_200;
    puStack_400 = uStack_1f8;
    uStack_3d0 = CONCAT44(uStack_1dc,uStack_1e0);
    uStack_3d8 = uStack_1e8;
    uStack_3c8 = CONCAT44(uStack_1d4,uStack_1d8);
    uStack_3f0 = uStack_1d0;
    puStack_3e8 = puStack_1c8;
    uStack_3e0 = uStack_1c0;
    uStack_390 = uStack_150;
    uStack_388 = uStack_148;
    uStack_380 = uStack_140;
    uStack_37c = uStack_13c;
    uStack_378 = uStack_138;
    uStack_374 = uStack_134;
    uStack_370 = uStack_130;
    uStack_36c = uStack_12c;
    uStack_368 = uStack_128;
    uStack_364 = uStack_124;
    uStack_360 = uStack_120;
    uStack_35c = uStack_11c;
    uStack_358 = uStack_118;
    uStack_354 = uStack_114;
    uStack_350 = uStack_110;
    uStack_348 = uStack_108;
    uStack_340 = uStack_100;
    uStack_338 = uStack_f8;
    uStack_3b0 = uStack_1a8;
    uStack_3c0 = uStack_1b8;
    uStack_3bc = uStack_1b4;
    uStack_3b8 = (undefined4)uStack_1b0;
    uStack_3b4 = uStack_1b0._4_4_;
    uStack_328 = uStack_80;
    uStack_330 = uStack_c8;
    uStack_3a8 = 1;
    uStack_3a4 = (undefined4)uVar8;
    uStack_3a0 = (undefined4)((ulonglong)uVar8 >> 0x20);
    uStack_39c = 1;
    FUN_1409dd4d0(&puStack_278,&uStack_408);
    uVar6 = uStack_4a;
    puVar5 = puStack_60;
    pfVar2 = pfStack_70;
    if (cStack_ca != '\b') {
      FUN_140be24b0(auStack_e0);
    }
    uVar8 = func_0x000140bd5790(&uStack_224);
    *(undefined4 *)(puVar5 + 9) = 1;
    *(undefined8 *)((longlong)puVar5 + 0x4c) = uVar8;
    uVar11 = 0x109ca79d5cccc2d1;
    lVar7 = FUN_1400dd6f0(lVar10,0x109ca79d5cccc2d1,0x24eabc1830c9840c);
    if (((lVar7 != 0) && (uVar11 < *(ulonglong *)(param_2 + 0x218))) &&
       (uVar11 = *(ulonglong *)(*(longlong *)(param_2 + 0x210) + uVar11 * 8), uVar11 != 0)) {
      lVar7 = ~uVar11 * 0x60;
      if (*(longlong *)(*(longlong *)(param_2 + 0x1e0) + 0x20 + lVar7) != 0) {
        fVar13 = *(float *)(*(longlong *)(*(longlong *)(param_2 + 0x1e0) + lVar7 + 0x10) + 0x58);
        if (_UNK_14295d580 <= fVar13) {
          fVar13 = _UNK_14295d580;
        }
        if (*(char *)((longlong)pfVar2 + 5) == '\0') {
          uVar11 = *puVar5;
          if ((uVar11 < 2 | *(byte *)((longlong)puVar5 + 0x54)) == 1) {
            fVar14 = *(float *)(puVar5 + 8);
          }
          else {
            fVar14 = fVar13 / _UNK_1429cd078 + *(float *)(puVar5 + 8);
            *(float *)(puVar5 + 8) = fVar14;
          }
        }
        else {
          fVar14 = *pfVar2;
          *(float *)(puVar5 + 8) = fVar14;
          uVar11 = *puVar5;
        }
        *puVar5 = uVar11 + 1;
        fVar13 = fVar13 + *(float *)((longlong)puVar5 + 0x44);
        *(float *)((longlong)puVar5 + 0x44) = fVar13;
        puVar1 = puVar5 + 1;
        puVar12 = (ulonglong *)0x0;
        if (*(char *)((longlong)puVar5 + 0x3e) != '\a') {
          puVar12 = puVar1;
        }
        FUN_1408a8d70(param_1,param_2,puVar12,&uStack_228,puStack_58,CONCAT44(uVar15,fVar13),
                      CONCAT44(uVar17,fVar14),uVar6,param_5,bVar4 & bStack_49 & 1);
        if (((_UNK_142925904 <= *(float *)(puVar5 + 8)) &&
            (*(char *)((longlong)pfVar2 + 5) != '\x01')) &&
           (*(char *)((longlong)puVar5 + 0x54) == '\0')) {
          if (((int)puVar5[9] != 2) && (*(char *)((longlong)puStack_60 + 0x3e) != '\a')) {
            puStack_58 = puVar1;
            func_0x00014003bb80(puVar1);
            FUN_140be24b0(puStack_58);
          }
          *(undefined4 *)(puStack_60 + 9) = 2;
          uVar11 = 0xd8f93d064ea9d61c;
          lVar10 = FUN_1400dd6f0(lVar10,0xd8f93d064ea9d61c,0x6447617066d9832f);
          if (((lVar10 == 0) || (*(ulonglong *)(param_2 + 0x218) <= uVar11)) ||
             (uVar11 = *(ulonglong *)(*(longlong *)(param_2 + 0x210) + uVar11 * 8), uVar11 == 0)) {
code_r0x0001408a8bce:
            FUN_1428d9310(&UNK_142a9bbe8);
            pcVar3 = (code *)swi(3);
            uVar8 = (*pcVar3)();
            return uVar8;
          }
          lVar10 = ~uVar11 * 0x60;
          if (*(longlong *)(*(longlong *)(param_2 + 0x1e0) + 0x20 + lVar10) == 0)
          goto code_r0x0001408a8bce;
          lVar10 = *(longlong *)(param_2 + 0x1e0) + lVar10;
          lVar7 = *(longlong *)(lVar10 + 0x10);
          *(undefined4 *)(lVar10 + 0x5c) = *(undefined4 *)(param_2 + 0x5b0);
          lVar10 = *(longlong *)(lVar7 + 0x30);
          uVar8 = *(undefined8 *)(lVar7 + 0x40);
          if (lVar10 == *(longlong *)(lVar7 + 0x20)) {
            FUN_140562870(lVar7 + 0x20,&UNK_142c89078);
          }
          *(undefined8 *)(*(longlong *)(lVar7 + 0x28) + lVar10 * 8) = uVar8;
          *(longlong *)(lVar7 + 0x30) = lVar10 + 1;
          *(longlong *)(lVar7 + 0x40) = *(longlong *)(lVar7 + 0x40) + 1;
          if (2 < *puRam000000014362c3a8) {
            puVar9 = (undefined4 *)func_0x00014067dd80(&UNK_142a9bc28);
            uStack_1e0 = *puVar9;
            uStack_1dc = puVar9[1];
            uStack_1d8 = puVar9[2];
            uStack_1d4 = puVar9[3];
            uStack_1b4 = puVar9[4];
            uStack_1d0 = 3;
            puStack_1c8 = &UNK_142a9bb70;
            uStack_1c0 = 0x2b;
            uStack_1b0 = &UNK_142a9bc18;
            uStack_1a8 = 1;
            puStack_1a0 = (ulonglong *)0x8;
            uStack_200 = 0;
            uStack_198 = 0;
            uStack_190 = 0;
            uStack_1f8 = &UNK_142a9bb70;
            uStack_1f0 = 0x2b;
            uStack_1e8 = 0;
            uStack_1b8 = 1;
            func_0x000140d4a330(&uStack_408,&uStack_200);
          }
        }
        uVar11 = (ulonglong)uStack_64;
        goto code_r0x0001408a8b5b;
      }
    }
  }
  FUN_1428d9310(&UNK_142a9bbd0);
code_r0x0001408a8b99:
  uStack_408 = uStack_200;
  FUN_1428d9760(&UNK_142a9b0c0,0x2b,&uStack_408,&UNK_142a9b0a0,&UNK_142a9bb18);
                    /* WARNING: Does not return */
  pcVar3 = (code *)invalidInstructionException();
  (*pcVar3)();
}
