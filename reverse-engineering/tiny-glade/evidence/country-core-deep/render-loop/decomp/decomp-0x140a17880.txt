
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void FUN_140a17880(undefined8 param_1,longlong param_2,undefined8 param_3)

{
  uint *puVar1;
  undefined4 uVar2;
  int iVar3;
  ulonglong *puVar4;
  code *pcVar5;
  uint uVar6;
  longlong lVar7;
  longlong lVar8;
  longlong lVar9;
  longlong lVar10;
  uint uVar11;
  ulonglong uVar12;
  bool bVar13;
  bool bVar14;
  undefined1 auVar15 [16];
  undefined8 uVar16;
  undefined1 auVar17 [16];
  undefined1 auVar18 [16];
  undefined4 uVar19;
  undefined8 uStack_3f8;
  undefined8 uStack_3f0;
  undefined8 uStack_3e8;
  undefined8 uStack_3e0;
  undefined8 uStack_3d8;
  undefined8 uStack_3d0;
  undefined8 uStack_3c8;
  undefined8 uStack_3c0;
  undefined8 uStack_3b8;
  undefined8 uStack_3b0;
  undefined8 uStack_3a8;
  undefined8 uStack_3a0;
  undefined8 uStack_398;
  undefined8 uStack_390;
  undefined8 uStack_388;
  undefined8 uStack_380;
  undefined8 uStack_378;
  undefined8 uStack_370;
  undefined8 uStack_368;
  undefined8 uStack_360;
  undefined8 uStack_358;
  undefined8 uStack_350;
  undefined8 uStack_348;
  undefined8 uStack_340;
  undefined8 uStack_330;
  undefined8 uStack_328;
  undefined4 uStack_320;
  undefined4 uStack_31c;
  undefined4 uStack_318;
  undefined4 uStack_314;
  undefined4 uStack_310;
  undefined4 uStack_30c;
  undefined4 uStack_308;
  undefined4 uStack_304;
  undefined4 uStack_300;
  undefined4 uStack_2fc;
  undefined4 uStack_2f8;
  undefined4 uStack_2f4;
  undefined8 uStack_2f0;
  undefined8 uStack_2e8;
  undefined8 uStack_2e0;
  undefined8 uStack_2d8;
  longlong lStack_2d0;
  undefined8 uStack_2c8;
  undefined1 auStack_298 [38];
  char cStack_272;
  longlong lStack_270;
  undefined4 uStack_218;
  undefined4 uStack_214;
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
  undefined8 uStack_168;
  undefined8 uStack_160;
  undefined4 uStack_158;
  undefined4 uStack_154;
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
  undefined8 uStack_128;
  undefined8 uStack_120;
  undefined8 uStack_118;
  undefined8 uStack_110;
  undefined1 auStack_108 [16];
  undefined1 auStack_f8 [22];
  char cStack_e2;
  longlong lStack_e0;
  undefined8 uStack_98;
  undefined8 uStack_88;
  longlong lStack_80;
  undefined4 uStack_78;
  undefined1 uStack_71;
  undefined8 uStack_70;
  
  uStack_70 = 0xfffffffffffffffe;
  lVar10 = param_2 + 0x38;
  uVar12 = 0xc2b8e941c91a6428;
  lVar7 = FUN_1400dd6f0(lVar10,0xc2b8e941c91a6428,0x59f3c6d1d3da43a0);
  if (((lVar7 == 0) || (*(ulonglong *)(param_2 + 0x218) <= uVar12)) ||
     (uVar12 = *(ulonglong *)(*(longlong *)(param_2 + 0x210) + uVar12 * 8), uVar12 == 0)) {
code_r0x000140a17ff8:
    FUN_1428d9310(&UNK_142ac87b8);
code_r0x000140a18004:
    FUN_1428d9310(&UNK_142ac87d0);
code_r0x000140a18010:
    FUN_1428d9310(&UNK_142ac87e8);
  }
  else {
    lVar7 = ~uVar12 * 0x60;
    if (*(longlong *)(*(longlong *)(param_2 + 0x1e0) + 0x20 + lVar7) == 0)
    goto code_r0x000140a17ff8;
    iVar3 = **(int **)(*(longlong *)(param_2 + 0x1e0) + lVar7 + 0x10);
    uVar12 = 0x864ddca98c2312a6;
    lVar7 = FUN_1400dd6f0(lVar10,0x864ddca98c2312a6,0x27b0e9369e9a484e);
    if (((lVar7 == 0) || (*(ulonglong *)(param_2 + 0x218) <= uVar12)) ||
       (uVar12 = *(ulonglong *)(*(longlong *)(param_2 + 0x210) + uVar12 * 8), uVar12 == 0))
    goto code_r0x000140a18004;
    lVar7 = ~uVar12 * 0x60;
    if (*(longlong *)(*(longlong *)(param_2 + 0x1e0) + 0x20 + lVar7) == 0)
    goto code_r0x000140a18004;
    puVar4 = *(ulonglong **)(*(longlong *)(param_2 + 0x1e0) + lVar7 + 0x10);
    uVar12 = 0xbba451ee6a08a3ff;
    uStack_88 = param_3;
    lVar7 = FUN_1400dd6f0(lVar10,0xbba451ee6a08a3ff,0x8d008d15febcf29d);
    if (((lVar7 == 0) || (*(ulonglong *)(param_2 + 0x218) <= uVar12)) ||
       (uVar12 = *(ulonglong *)(*(longlong *)(param_2 + 0x210) + uVar12 * 8), uVar12 == 0))
    goto code_r0x000140a18010;
    lVar7 = ~uVar12 * 0x60;
    if (*(longlong *)(*(longlong *)(param_2 + 0x1e0) + 0x20 + lVar7) == 0)
    goto code_r0x000140a18010;
    bVar14 = iVar3 == 0x13;
    lVar7 = *(longlong *)(*(longlong *)(param_2 + 0x1e0) + lVar7 + 0x10);
    lVar8 = (ulonglong)*(byte *)(lVar7 + 0x4d0) * 8;
    lVar9 = *(longlong *)(&UNK_142ac8948 + lVar8);
    lStack_80 = *(longlong *)(&UNK_142ac8978 + lVar8);
    puVar1 = (uint *)(lVar7 + lVar9);
    uVar2 = *(undefined4 *)(lVar7 + 0x18 + lVar9);
    uStack_71 = *(undefined1 *)(lVar7 + 0x128 + lStack_80);
    uVar6 = *(uint *)(lVar7 + lVar9);
    bVar13 = uVar6 < 2;
    if (!bVar13 || !bVar14) {
      uStack_78 = 0xffffffff;
code_r0x000140a17ad7:
      uVar11 = 2;
      if (uVar6 - 2 < 2) {
        uVar11 = uVar6 - 2;
      }
      if (uVar11 == 2) {
        lVar10 = 0x14;
        lVar9 = 0x10;
      }
      else {
        if (uVar11 != 1) {
          uVar12 = 0x4cab10763b3fdb80;
          lVar9 = FUN_1400dd6f0(lVar10,0x4cab10763b3fdb80,0x1a422ef7534b54d);
          if (((lVar9 == 0) || (*(ulonglong *)(param_2 + 0x218) <= uVar12)) ||
             (uVar12 = *(ulonglong *)(*(longlong *)(param_2 + 0x210) + uVar12 * 8), uVar12 == 0))
          goto code_r0x000140a18028;
          lVar9 = ~uVar12 * 0x60;
          if (*(longlong *)(*(longlong *)(param_2 + 0x1e0) + 0x20 + lVar9) == 0)
          goto code_r0x000140a18028;
          uVar16 = **(undefined8 **)(*(longlong *)(param_2 + 0x1e0) + lVar9 + 0x10);
          auVar17._0_4_ = (float)uVar16 + _UNK_1429258e0;
          auVar17._4_4_ = (float)((ulonglong)uVar16 >> 0x20) + _UNK_1429258e4;
          auVar17._8_4_ = _UNK_1429258e8 + 0.0;
          auVar17._12_4_ = _UNK_1429258ec + 0.0;
          auVar18._0_8_ = *puVar4 & 0xffffffff;
          auVar18._8_4_ = (int)(*puVar4 >> 0x20);
          auVar18._12_4_ = 0;
          auVar15._0_4_ = (float)(SUB168(auVar18 | _UNK_1429e95b0,0) - (double)UNK_1429e95b0);
          auVar15._4_4_ = (float)(SUB168(auVar18 | _UNK_1429e95b0,8) - UNK_1429e95b0._8_8_);
          auVar15._8_8_ = 0;
          auVar18 = divps(auVar17,auVar15);
          uVar16 = auVar18._0_8_;
          uVar12 = CONCAT71((int7)((ulonglong)lVar10 >> 8),1);
          goto code_r0x000140a17bfe;
        }
        lVar10 = 8;
        lVar9 = 4;
      }
      uVar16 = CONCAT44(_UNK_142a80ce4 - *(float *)((longlong)puVar1 + lVar10) * _UNK_1429258e4,
                        _UNK_142925870 +
                        (*(float *)((longlong)puVar1 + lVar9) /
                        ((float)(uint)*puVar4 / (float)*(uint *)((longlong)puVar4 + 4))) *
                        _UNK_1429258e0);
      uVar12 = 0;
code_r0x000140a17bfe:
      uVar19 = 0;
      FUN_1416032a0(&uStack_218,&UNK_142ac8830,0x19,&UNK_142ac8849,0x16,param_2,param_1);
      if (cStack_e2 != '\b') {
        FUN_14096b940(auStack_108,lStack_e0 + 0xe8,0xe6d7f61a7ebb9b21,(int)uVar16,
                      CONCAT44(uVar19,(int)((ulonglong)uVar16 >> 0x20)));
      }
      func_0x000142923c00(&uStack_3a8,&uStack_218,400);
      if (cStack_272 != '\b') {
        FUN_14096b850(auStack_298,lStack_270 + 0xe8,0xfd5b03d68ca8227c,uVar12 & 0xffffffff);
      }
      func_0x000142923c00(&uStack_218,&uStack_3a8,400);
      if (cStack_e2 != '\b') {
        FUN_14096ba30(auStack_108,lStack_e0 + 0xe8,0x2783b65eab1269bd,uVar2);
      }
      func_0x000142923c00(&uStack_3a8,&uStack_218,400);
      if (cStack_272 != '\b') {
        FUN_14096b570(auStack_298,lStack_270 + 0xe8,0x78dfbf1f882dcc45,uStack_78);
      }
      func_0x000142923c00(&uStack_218,&uStack_3a8,400);
      if (cStack_e2 != '\b') {
        FUN_14096b850(auStack_108,lStack_e0 + 0xe8,0x77e48da9bec3aa07,
                      CONCAT71((int7)((ulonglong)param_3 >> 8),bVar13 && bVar14) & 0xffffffff);
      }
      func_0x000142923c00(&uStack_3a8,&uStack_218,400);
      if (cStack_272 != '\b') {
        FUN_14096b850(auStack_298,lStack_270 + 0xe8,0xd7fa4852b47fdd5,uStack_71);
      }
      func_0x000142923c00(&uStack_218,&uStack_3a8,400);
      if (cStack_e2 != '\b') {
        FUN_14096ba30(auStack_108,lStack_e0 + 0xe8,0xb665deccfaab298b,
                      *(undefined4 *)(lVar7 + 0x118 + lStack_80));
      }
      func_0x000142923c00(&uStack_3a8,&uStack_218,400);
      FUN_141604dd0(&uStack_218,&uStack_3a8,0xc26554c6de4b08b0);
      FUN_1416061c0(&uStack_3a8,&uStack_218,0xd2ddf967422fe2e9);
      uStack_3f8 = uStack_88;
      uStack_218 = 2;
      FUN_1408ee240(&uStack_3f8,&uStack_3a8,0x4861579da049fbfc,0x19,&uStack_218);
      func_0x000142923c00(&uStack_218,&uStack_3a8,400);
      uStack_3b8 = uStack_178;
      uStack_3b0 = uStack_170;
      uStack_3c8 = uStack_188;
      uStack_3c0 = uStack_180;
      uStack_3d8 = uStack_198;
      uStack_3d0 = uStack_190;
      uStack_3e8 = uStack_1a8;
      uStack_3e0 = uStack_1a0;
      uStack_3f8 = uStack_1b8;
      uStack_3f0 = uStack_1b0;
      uStack_398 = uStack_208;
      uStack_3a8 = CONCAT44(uStack_214,uStack_218);
      uStack_3a0 = uStack_210;
      uStack_378 = uStack_200;
      uStack_370 = uStack_1f8;
      uStack_368 = uStack_1f0;
      uStack_390 = uStack_1e8;
      uStack_388 = uStack_1e0;
      uStack_380 = uStack_1d8;
      uStack_330 = uStack_168;
      uStack_328 = uStack_160;
      uStack_320 = uStack_158;
      uStack_31c = uStack_154;
      uStack_318 = uStack_150;
      uStack_314 = uStack_14c;
      uStack_310 = uStack_148;
      uStack_30c = uStack_144;
      uStack_308 = uStack_140;
      uStack_304 = uStack_13c;
      uStack_300 = uStack_138;
      uStack_2fc = uStack_134;
      uStack_2f8 = uStack_130;
      uStack_2f4 = uStack_12c;
      uStack_2f0 = uStack_128;
      uStack_2e8 = uStack_120;
      uStack_2e0 = uStack_118;
      uStack_2d8 = uStack_110;
      uStack_350 = uStack_1c0;
      uStack_360 = uStack_1d0;
      uStack_358 = uStack_1c8;
      uStack_2c8 = uStack_98;
      lStack_2d0 = lStack_e0;
      uStack_348 = _UNK_142ac7e70;
      uStack_340 = _UNK_142ac7e78;
      FUN_1409dd4d0(&uStack_3f8,&uStack_3a8);
      if (cStack_e2 != '\b') {
        FUN_140be24b0(auStack_f8);
      }
      return;
    }
    uVar12 = 0xfc2a14b9762daa97;
    lVar9 = FUN_1400dd6f0(lVar10,0xfc2a14b9762daa97,0x6658e89da1b9b629);
    if (((lVar9 != 0) && (uVar12 < *(ulonglong *)(param_2 + 0x218))) &&
       (uVar12 = *(ulonglong *)(*(longlong *)(param_2 + 0x210) + uVar12 * 8), uVar12 != 0)) {
      lVar9 = ~uVar12 * 0x60;
      if (*(longlong *)(*(longlong *)(param_2 + 0x1e0) + 0x20 + lVar9) != 0) {
        uStack_78 = *(undefined4 *)
                     (*(longlong *)(*(longlong *)(param_2 + 0x1e0) + lVar9 + 0x10) + 4);
        uVar6 = *puVar1;
        goto code_r0x000140a17ad7;
      }
    }
  }
  FUN_1428d9310(&UNK_142ac8800);
code_r0x000140a18028:
  FUN_1428d9310(&UNK_142ac8818);
  pcVar5 = (code *)swi(3);
  (*pcVar5)();
  return;
}

