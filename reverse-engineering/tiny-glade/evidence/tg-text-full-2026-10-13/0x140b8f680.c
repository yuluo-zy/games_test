
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void FUN_140b8f680(longlong param_1,undefined8 param_2)

{
  undefined8 *puVar1;
  code *pcVar2;
  bool bVar3;
  undefined4 uVar4;
  undefined4 uVar5;
  undefined4 uVar6;
  undefined8 uVar7;
  undefined8 *puVar8;
  undefined4 *puVar9;
  byte *pbVar10;
  ulonglong uVar11;
  longlong lVar12;
  longlong lVar13;
  undefined8 *puVar14;
  longlong lVar15;
  undefined8 uStack_358;
  undefined8 uStack_350;
  undefined8 uStack_348;
  undefined8 uStack_340;
  undefined4 uStack_338;
  undefined4 uStack_334;
  undefined4 uStack_330;
  undefined4 uStack_32c;
  undefined8 uStack_328;
  undefined8 uStack_320;
  undefined8 uStack_318;
  undefined8 uStack_310;
  undefined *puStack_308;
  undefined8 uStack_300;
  int **ppiStack_2f8;
  undefined8 uStack_2f0;
  undefined8 uStack_2e8;
  undefined1 auStack_c0 [16];
  undefined8 *puStack_b0;
  int *piStack_a8;
  undefined8 uStack_a0;
  undefined8 uStack_98;
  undefined8 uStack_90;
  undefined4 uStack_88;
  undefined4 uStack_84;
  undefined4 uStack_80;
  undefined4 uStack_7c;
  undefined4 uStack_78;
  undefined4 uStack_74;
  undefined4 uStack_70;
  undefined4 uStack_6c;
  undefined8 uStack_68;
  undefined8 uStack_60;
  longlong lStack_58;
  undefined8 uStack_50;
  undefined8 uStack_48;
  int iStack_3c;
  undefined4 *puStack_38;
  undefined8 uStack_30;
  
  uStack_30 = 0xfffffffffffffffe;
  if (*(char *)(param_1 + 0x4d4) == '\0') {
    if (*(longlong *)(param_1 + 0x458) != 0) {
      func_0x000140d4a3b0(auStack_c0);
      uStack_358 = *(int **)(param_1 + 0x450);
      uStack_350 = (code *)(*(longlong *)(param_1 + 0x458) * 0x40 + (longlong)uStack_358);
      uStack_348 = (int *)auStack_c0;
      FUN_141800310(&lStack_58,&uStack_358,&UNK_142afbe78);
      puStack_38 = (undefined4 *)uStack_50;
      FUN_140bfe9c0(&piStack_a8,param_2,param_1 + 0x460,uStack_50,uStack_48,0x22);
      if ((int)piStack_a8 != 0xb) {
        uStack_318 = uStack_68;
        uStack_310 = uStack_60;
        uStack_338 = uStack_88;
        uStack_334 = uStack_84;
        uStack_330 = uStack_80;
        uStack_32c = uStack_7c;
        uStack_348 = uStack_98;
        uStack_340 = uStack_90;
        uStack_358 = piStack_a8;
        uStack_350 = uStack_a0;
        FUN_1428d9760(&UNK_142afc218,0x11,&uStack_358,&UNK_142afbe90,&UNK_142afc230);
        goto LAB_140b8fcfb;
      }
      lVar12 = *(longlong *)(param_1 + 0x458);
      *(undefined8 *)(param_1 + 0x458) = 0;
      if (lVar12 != 0) {
        puVar14 = (undefined8 *)(*(longlong *)(param_1 + 0x450) + 8);
        do {
          if (puVar14[-1] != 0) {
            func_0x000140613c20(*puVar14,puVar14[-1],1);
          }
          puVar14 = puVar14 + 8;
          lVar12 = lVar12 + -1;
        } while (lVar12 != 0);
      }
      puVar14 = _DAT_14362c3e0;
      if (lStack_58 != 0) {
        func_0x000140613c20(puStack_38,lStack_58 * 0x30,8);
        puVar14 = _DAT_14362c3e0;
      }
      while (puVar8 = _DAT_14362c3e0, bVar3 = puStack_b0 != _DAT_14362c3e0, _DAT_14362c3e0 = puVar14
            , bVar3) {
        puVar1 = (undefined8 *)puStack_b0[3];
        func_0x000140613c20(*puStack_b0,puStack_b0[2],puStack_b0[1]);
        puVar14 = _DAT_14362c3e0;
        puStack_b0 = puVar1;
        _DAT_14362c3e0 = puVar8;
      }
    }
  }
  else {
    *(undefined1 *)(param_1 + 0x4d4) = 0;
    iStack_3c = *(int *)(param_1 + 0x484) * 2;
    if (2 < *_DAT_14362c3a8) {
      piStack_a8 = &iStack_3c;
      uStack_a0 = FUN_140d8d560;
      uStack_90 = FUN_140d8d560;
      uStack_98 = piStack_a8;
      puVar9 = (undefined4 *)func_0x00014067dd80(&UNK_142afc288);
      uStack_338 = *puVar9;
      uStack_334 = puVar9[1];
      uStack_330 = puVar9[2];
      uStack_32c = puVar9[3];
      uStack_328 = 3;
      uStack_320 = &UNK_142afc2a0;
      uStack_318 = 7;
      puStack_308 = &UNK_142afc268;
      uStack_300 = 2;
      ppiStack_2f8 = &piStack_a8;
      uStack_2f0 = 2;
      uStack_2e8 = 0;
      uStack_358 = (int *)0x0;
      uStack_350 = (code *)&UNK_142afc2a0;
      uStack_348 = (int *)0x7;
      uStack_340 = (code *)0x0;
      uStack_310 = CONCAT44(puVar9[4],1);
      func_0x000140d4a330(&lStack_58,&uStack_358);
    }
    func_0x000140bd52a0(&uStack_358,CONCAT44(iStack_3c,iStack_3c),0x2b);
    func_0x000140bd5370(&piStack_a8,&uStack_358);
    func_0x000140bd53b0(&uStack_358,&piStack_a8);
    lStack_58 = 1;
    FUN_1417faa80(&piStack_a8,param_2,&UNK_142afc168,0xb,&uStack_358,&lStack_58);
    if ((int)piStack_a8 != 0xb) {
      uStack_318 = uStack_68;
      uStack_310 = uStack_60;
      uStack_338 = uStack_88;
      uStack_334 = uStack_84;
      uStack_330 = uStack_80;
      uStack_32c = uStack_7c;
      uStack_348 = uStack_98;
      uStack_340 = uStack_90;
      uStack_358 = piStack_a8;
      uStack_350 = uStack_a0;
      FUN_1428d9760(&UNK_142afc173,0x1a,&uStack_358,&UNK_142afbe90,&UNK_142afc2a8);
LAB_140b8fcfb:
                    /* WARNING: Does not return */
      pcVar2 = (code *)invalidInstructionException();
      (*pcVar2)();
    }
    puStack_38 = (undefined4 *)(param_1 + 0x460);
    uStack_328 = CONCAT44(uStack_6c,uStack_70);
    uStack_338 = uStack_80;
    uStack_334 = uStack_7c;
    uStack_330 = uStack_78;
    uStack_32c = uStack_74;
    uStack_348._0_4_ = SUB84(uStack_90,0);
    uStack_348._4_4_ = uStack_90._4_4_;
    uStack_340._0_4_ = uStack_88;
    uStack_340._4_4_ = uStack_84;
    uStack_358._0_4_ = SUB84(uStack_a0,0);
    uStack_358._4_4_ = uStack_a0._4_4_;
    uStack_350._0_4_ = SUB84(uStack_98,0);
    uStack_350._4_4_ = uStack_98._4_4_;
    func_0x00014003bb80();
    FUN_140be24b0(puStack_38);
    *(undefined8 *)(puStack_38 + 0xc) = uStack_328;
    puStack_38[8] = uStack_338;
    puStack_38[9] = uStack_334;
    puStack_38[10] = uStack_330;
    puStack_38[0xb] = uStack_32c;
    puStack_38[4] = (undefined4)uStack_348;
    puStack_38[5] = uStack_348._4_4_;
    puStack_38[6] = (undefined4)uStack_340;
    puStack_38[7] = uStack_340._4_4_;
    *puStack_38 = (undefined4)uStack_358;
    puStack_38[1] = uStack_358._4_4_;
    puStack_38[2] = (undefined4)uStack_350;
    puStack_38[3] = uStack_350._4_4_;
    FUN_140e128c0(&uStack_358);
    puStack_38 = (undefined4 *)(param_1 + 0x110);
    FUN_1417f8890();
    func_0x000142923c00(puStack_38,&uStack_358,0x298);
    FUN_1417f99e0(&uStack_358,iStack_3c,iStack_3c);
    if (*(longlong *)(param_1 + 0x3a8) != 0) {
      func_0x000140613c20(*(undefined8 *)(param_1 + 0x3b0),*(longlong *)(param_1 + 0x3a8) * 0xe,2);
    }
    if (*(longlong *)(param_1 + 0x3c0) != 0) {
      func_0x000140613c20(*(undefined8 *)(param_1 + 0x3c8),*(longlong *)(param_1 + 0x3c0) * 0xe,2);
    }
    *(undefined8 *)(param_1 + 1000) = uStack_318;
    *(undefined8 *)(param_1 + 0x3f0) = uStack_310;
    *(undefined4 *)(param_1 + 0x3d8) = (undefined4)uStack_328;
    *(undefined4 *)(param_1 + 0x3dc) = uStack_328._4_4_;
    *(undefined4 *)(param_1 + 0x3e0) = (undefined4)uStack_320;
    *(undefined4 *)(param_1 + 0x3e4) = uStack_320._4_4_;
    *(undefined4 *)(param_1 + 0x3c8) = uStack_338;
    *(undefined4 *)(param_1 + 0x3cc) = uStack_334;
    *(undefined4 *)(param_1 + 0x3d0) = uStack_330;
    *(undefined4 *)(param_1 + 0x3d4) = uStack_32c;
    *(undefined4 *)(param_1 + 0x3b8) = (undefined4)uStack_348;
    *(undefined4 *)(param_1 + 0x3bc) = uStack_348._4_4_;
    *(undefined4 *)(param_1 + 0x3c0) = (undefined4)uStack_340;
    *(undefined4 *)(param_1 + 0x3c4) = uStack_340._4_4_;
    *(undefined4 *)(param_1 + 0x3a8) = (undefined4)uStack_358;
    *(undefined4 *)(param_1 + 0x3ac) = uStack_358._4_4_;
    *(undefined4 *)(param_1 + 0x3b0) = (undefined4)uStack_350;
    *(undefined4 *)(param_1 + 0x3b4) = uStack_350._4_4_;
    lVar12 = 0;
    if (*(longlong *)(param_1 + 0x3f8) != 0) {
      lVar12 = *(longlong *)(param_1 + 0x3f8) << 3;
      func_0x000140613c20(*(undefined8 *)(param_1 + 0x400),lVar12,4);
    }
    *(undefined8 *)(param_1 + 0x3f8) = 0;
    *(undefined8 *)(param_1 + 0x400) = 4;
    *(undefined8 *)(param_1 + 0x408) = 0;
    pbVar10 = (byte *)FUN_140d4d740();
    if ((*pbVar10 & 1) == 0) {
      lVar13 = FUN_140d64e00();
      pbVar10[0] = 1;
      pbVar10[1] = 0;
      pbVar10[2] = 0;
      pbVar10[3] = 0;
      pbVar10[4] = 0;
      pbVar10[5] = 0;
      pbVar10[6] = 0;
      pbVar10[7] = 0;
      *(longlong *)(pbVar10 + 8) = lVar13;
      *(longlong *)(pbVar10 + 0x10) = lVar12;
    }
    else {
      lVar13 = *(longlong *)(pbVar10 + 8);
      lVar12 = *(longlong *)(pbVar10 + 0x10);
    }
    *(longlong *)(pbVar10 + 8) = lVar13 + 1;
    lVar15 = *(longlong *)(param_1 + 0x4a0);
    if (lVar15 != 0) {
      uVar11 = (lVar15 + 1) * 0x24 + 0xfU & 0xfffffffffffffff0;
      lVar15 = lVar15 + uVar11 + 0x11;
      if (lVar15 != 0) {
        func_0x000140613c20(*(longlong *)(param_1 + 0x498) - uVar11,lVar15,0x10);
      }
    }
    uVar7 = _UNK_142afbe18;
    *(undefined8 *)(param_1 + 0x4a8) = _DAT_142afbe10;
    *(undefined8 *)(param_1 + 0x4b0) = uVar7;
    uVar6 = _UNK_142afbe0c;
    uVar5 = _UNK_142afbe08;
    uVar4 = _UNK_142afbe04;
    *(undefined4 *)(param_1 + 0x498) = _DAT_142afbe00;
    *(undefined4 *)(param_1 + 0x49c) = uVar4;
    *(undefined4 *)(param_1 + 0x4a0) = uVar5;
    *(undefined4 *)(param_1 + 0x4a4) = uVar6;
    *(longlong *)(param_1 + 0x4b8) = lVar13;
    *(longlong *)(param_1 + 0x4c0) = lVar12;
    lVar12 = *(longlong *)(param_1 + 0x450);
    lVar13 = *(longlong *)(param_1 + 0x458);
    if (lVar13 != 0) {
      puVar14 = (undefined8 *)(lVar12 + 8);
      do {
        if (puVar14[-1] != 0) {
          func_0x000140613c20(*puVar14,puVar14[-1],1);
        }
        puVar14 = puVar14 + 8;
        lVar13 = lVar13 + -1;
      } while (lVar13 != 0);
    }
    if (*(longlong *)(param_1 + 0x448) != 0) {
      func_0x000140613c20(lVar12,*(longlong *)(param_1 + 0x448) << 6,8);
    }
    *(undefined8 *)(param_1 + 0x448) = 0;
    *(undefined8 *)(param_1 + 0x450) = 8;
    *(undefined8 *)(param_1 + 0x458) = 0;
    *(undefined4 *)(param_1 + 0x4cc) = *(undefined4 *)(param_1 + 0x4c8);
  }
  return;
}

