
/* WARNING: Type propagation algorithm not settling */

undefined8 *
FUN_140bd7dd0(undefined8 *param_1,longlong *param_2,longlong *param_3,undefined2 *param_4)

{
  undefined *puVar1;
  undefined8 *puVar2;
  longlong lVar3;
  undefined **ppuVar4;
  longlong *plVar5;
  longlong lVar6;
  undefined4 uVar7;
  uint uVar8;
  longlong lVar9;
  undefined4 *puVar10;
  undefined *puVar11;
  ulonglong uVar12;
  undefined *puVar13;
  undefined8 *puVar14;
  undefined8 uVar15;
  code *pcVar16;
  undefined **ppuVar17;
  undefined *puStack_348;
  undefined *puStack_340;
  longlong lStack_338;
  undefined *puStack_330;
  longlong lStack_328;
  undefined *puStack_320;
  longlong lStack_318;
  longlong lStack_310;
  undefined4 uStack_308;
  undefined4 uStack_304;
  undefined4 uStack_300;
  undefined4 uStack_2fc;
  longlong lStack_2f8;
  undefined *puStack_2f0;
  undefined8 uStack_2e8;
  undefined4 uStack_2e0;
  undefined4 uStack_2dc;
  undefined *puStack_2d8;
  undefined8 uStack_2d0;
  undefined8 **ppuStack_2c8;
  undefined8 uStack_2c0;
  undefined8 uStack_2b8;
  undefined8 auStack_120 [2];
  undefined8 *puStack_110;
  undefined8 *puStack_108;
  undefined8 *puStack_100;
  undefined *puStack_f8;
  undefined8 uStack_f0;
  undefined4 uStack_e8;
  undefined4 uStack_e4;
  undefined4 uStack_d8;
  undefined4 uStack_d4;
  undefined4 uStack_d0;
  undefined4 uStack_cc;
  undefined4 uStack_c8;
  undefined4 uStack_c4;
  undefined8 *puStack_c0;
  undefined8 uStack_b8;
  undefined8 uStack_b0;
  undefined **ppuStack_a8;
  undefined8 uStack_a0;
  undefined4 uStack_98;
  undefined4 uStack_94;
  undefined4 uStack_90;
  undefined4 uStack_8c;
  code *pcStack_80;
  longlong *plStack_78;
  undefined **ppuStack_70;
  longlong lStack_68;
  undefined8 *puStack_60;
  undefined8 *puStack_58;
  undefined1 uStack_4c;
  undefined1 uStack_4b;
  undefined1 uStack_4a;
  undefined1 uStack_49;
  undefined8 uStack_48;
  
  uStack_48 = 0xfffffffffffffffe;
  lVar3 = *param_3;
  ppuVar4 = (undefined **)param_3[1];
  if (lVar3 == 0) {
    *param_1 = 0x12;
    param_1[1] = 0;
    param_1[2] = ppuVar4;
    plVar5 = (longlong *)param_2[5];
    if (plVar5 == (longlong *)0x0) {
      return param_1;
    }
    LOCK();
    *plVar5 = *plVar5 + -1;
    UNLOCK();
    if (*plVar5 != 0) {
      return param_1;
    }
    FUN_140c8fd00(param_2 + 5);
    return param_1;
  }
  uVar12 = (longlong)ppuVar4 << 4;
  puStack_c0 = param_1;
  plStack_78 = param_2;
  if (uVar12 < 0x7ffffffffffffff9) {
    if (uVar12 == 0) {
      pcStack_80 = (code *)0x8;
      ppuStack_70 = (undefined **)0x0;
      ppuVar17 = ppuStack_70;
    }
    else {
      uVar15 = 8;
      pcStack_80 = (code *)func_0x000140613c10(uVar12,8);
      ppuVar17 = ppuVar4;
      if (pcStack_80 == (code *)0x0) goto code_r0x000140bd877c;
    }
    ppuStack_70 = ppuVar17;
    uVar7 = (int)((ulonglong)uStack_b8 >> 0x20);
    if (ppuVar4 != (undefined **)0x0) {
      pcVar16 = pcStack_80 + 8;
      ppuVar17 = (undefined **)0x0;
      do {
        lVar6 = *(longlong *)(lVar3 + (longlong)ppuVar17 * 8);
        lVar9 = func_0x000142924010(lVar6);
        *(longlong *)(pcVar16 + -8) = lVar6;
        *(longlong *)pcVar16 = lVar9 + 1;
        ppuVar17 = (undefined **)((longlong)ppuVar17 + 1);
        pcVar16 = pcVar16 + 0x10;
        uVar7 = uStack_b8._4_4_;
      } while (ppuVar4 != ppuVar17);
    }
    uStack_b8._4_4_ = uVar7;
    if (3 < *puRam000000014362c3a8) {
      uStack_49 = 1;
      puVar10 = (undefined4 *)func_0x00014067dd80(&UNK_142b110a8);
      uStack_308 = *puVar10;
      uStack_304 = puVar10[1];
      uStack_300 = puVar10[2];
      uStack_2fc = puVar10[3];
      uStack_2dc = puVar10[4];
      lStack_2f8 = 4;
      puStack_2f0 = &UNK_142b110c0;
      uStack_2e8 = 0x13;
      puStack_2d8 = &UNK_142b11098;
      uStack_2d0 = 1;
      ppuStack_2c8 = (undefined8 **)0x8;
      lStack_328 = 0;
      uStack_2c0 = 0;
      uStack_2b8 = 0;
      puStack_320 = &UNK_142b110c0;
      lStack_318 = 0x13;
      lStack_310 = 0;
      uStack_2e0 = 1;
      uStack_49 = 1;
      func_0x000140d4a330(&uStack_4a,&lStack_328);
      param_2 = plStack_78;
    }
    uStack_49 = 1;
    FUN_1418b09f0(&lStack_328,param_2);
    puStack_60 = (undefined8 *)param_4;
    if (!SBORROW8(0,lStack_328)) {
      puVar13 = puStack_320 + lStack_318 * 0x208;
      puStack_348 = puStack_320;
      lStack_68 = lStack_328;
      lStack_338 = lStack_328;
      puStack_58 = (undefined8 *)puStack_320;
      puVar11 = puStack_320;
      puStack_330 = puVar13;
      while (puVar11 != puVar13) {
        puVar1 = puVar11 + 0x208;
        puStack_340 = puVar1;
        func_0x000142923c00(&lStack_328,puVar11,0x208);
        FUN_1418b0c80(&uStack_f0,&lStack_328,&lStack_328);
        if (SBORROW8(0,CONCAT44(uStack_f0._4_4_,(uint)uStack_f0))) break;
        uStack_98 = uStack_d0;
        uStack_94 = uStack_cc;
        uStack_90 = uStack_c8;
        uStack_8c = uStack_c4;
        uStack_a0 = (code *)CONCAT44(uStack_d4,uStack_d8);
        uStack_b8 = (undefined **)CONCAT44(uStack_f0._4_4_,(uint)uStack_f0);
        uStack_b0 = (code *)CONCAT44(uStack_e4,uStack_e8);
        if (3 < *puRam000000014362c3a8) {
          puStack_110 = &uStack_a0;
          puStack_108 = (undefined8 *)&UNK_14003bad0;
          puStack_100 = &uStack_b8;
          puStack_f8 = &UNK_14003bad0;
          puVar10 = (undefined4 *)func_0x00014067dd80(&UNK_142b11238);
          uStack_308 = *puVar10;
          uStack_304 = puVar10[1];
          uStack_300 = puVar10[2];
          uStack_2fc = puVar10[3];
          uStack_2dc = puVar10[4];
          lStack_2f8 = 4;
          puStack_2f0 = &UNK_142b110c0;
          uStack_2e8 = 0x13;
          puStack_2d8 = &UNK_142b11208;
          uStack_2d0 = 3;
          ppuStack_2c8 = &puStack_110;
          uStack_2c0 = 2;
          uStack_2b8 = 0;
          lStack_328 = 0;
          puStack_320 = &UNK_142b110c0;
          lStack_318 = 0x13;
          lStack_310 = 0;
          uStack_2e0 = 1;
          func_0x000140d4a330(&uStack_4a,&lStack_328);
        }
        if (uStack_b8 != (undefined **)0x0) {
          func_0x000140613c20(uStack_b0,uStack_b8,1);
        }
        puVar11 = puVar1;
        if (uStack_a0 != (code *)0x0) {
          func_0x000140613c20(CONCAT44(uStack_94,uStack_98),uStack_a0,1);
        }
      }
      if (lStack_68 != 0) {
        func_0x000140613c20(puStack_58,lStack_68 * 0x208,4);
      }
      uStack_b8 = ppuStack_70;
      uStack_b0 = pcStack_80;
      uStack_a0 = (code *)CONCAT62(uStack_a0._2_6_,*(undefined2 *)puStack_60);
      lStack_2f8 = plStack_78[6];
      lStack_328 = *plStack_78;
      puStack_320 = (undefined *)plStack_78[1];
      lStack_318 = plStack_78[2];
      lStack_310 = plStack_78[3];
      uStack_308 = (undefined4)plStack_78[4];
      uStack_304 = *(undefined4 *)((longlong)plStack_78 + 0x24);
      uStack_300 = (undefined4)plStack_78[5];
      uStack_2fc = *(undefined4 *)((longlong)plStack_78 + 0x2c);
      uStack_49 = 0;
      ppuStack_a8 = ppuVar4;
      FUN_140c08f10(&uStack_f0,&uStack_b8,&lStack_328);
      if (((uint)uStack_f0 & 1) == 0) {
        lStack_68 = CONCAT44(uStack_e4,uStack_e8);
        if (2 < *puRam000000014362c3a8) {
          puVar10 = (undefined4 *)func_0x00014067dd80(&UNK_142b110f8);
          uStack_308 = *puVar10;
          uStack_304 = puVar10[1];
          uStack_300 = puVar10[2];
          uStack_2fc = puVar10[3];
          uStack_2dc = puVar10[4];
          lStack_2f8 = 3;
          puStack_2f0 = &UNK_142b110c0;
          uStack_2e8 = 0x13;
          puStack_2d8 = &UNK_142b110e8;
          uStack_2d0 = 1;
          ppuStack_2c8 = (undefined8 **)0x8;
          lStack_328 = 0;
          uStack_2c0 = 0;
          uStack_2b8 = 0;
          puStack_320 = &UNK_142b110c0;
          lStack_318 = 0x13;
          lStack_310 = 0;
          uStack_2e0 = 1;
          func_0x000140d4a330(&uStack_4a,&lStack_328);
        }
        uVar8 = FUN_1418b1240();
        if (uVar8 == 1) {
          FUN_1418b1340(&puStack_110);
          puVar2 = puStack_100;
          if (SBORROW8(0,(longlong)puStack_110)) {
            uStack_f0._0_4_ = (uint)puStack_108;
            if (1 < *puRam000000014362c3a8) {
              uStack_b0 = (code *)&UNK_140bd7a40;
              uStack_b8 = (undefined **)&uStack_f0;
              puVar10 = (undefined4 *)func_0x00014067dd80(&UNK_142b111b0);
              uStack_308 = *puVar10;
              uStack_304 = puVar10[1];
              uStack_300 = puVar10[2];
              uStack_2fc = puVar10[3];
              uStack_2dc = puVar10[4];
              lStack_2f8 = 2;
              puStack_2f0 = &UNK_142b110c0;
              uStack_2e8 = 0x13;
              puStack_2d8 = &UNK_142b111a0;
              uStack_2d0 = 1;
              ppuStack_2c8 = (undefined8 **)&uStack_b8;
              uStack_2c0 = 1;
              uStack_2b8 = 0;
              lStack_328 = 0;
              puStack_320 = &UNK_142b110c0;
              lStack_318 = 0x13;
              lStack_310 = 0;
              uStack_2e0 = 1;
              func_0x000140d4a330(&uStack_4a,&lStack_328);
            }
          }
          else {
            puStack_58 = puStack_110;
            puStack_60 = puStack_108;
            if (2 < *puRam000000014362c3a8) {
              uStack_4c = 1;
              puVar10 = (undefined4 *)func_0x00014067dd80(&UNK_142b11130);
              uStack_308 = *puVar10;
              uStack_304 = puVar10[1];
              uStack_300 = puVar10[2];
              uStack_2fc = puVar10[3];
              uStack_2dc = puVar10[4];
              lStack_2f8 = 3;
              puStack_2f0 = &UNK_142b110c0;
              uStack_2e8 = 0x13;
              puStack_2d8 = &UNK_142b11120;
              uStack_2d0 = 1;
              ppuStack_2c8 = (undefined8 **)0x8;
              lStack_328 = 0;
              uStack_2c0 = 0;
              uStack_2b8 = 0;
              puStack_320 = &UNK_142b110c0;
              lStack_318 = 0x13;
              lStack_310 = 0;
              uStack_2e0 = 1;
              uStack_4c = 1;
              func_0x000140d4a330(&uStack_4a,&lStack_328);
            }
            puVar2 = puStack_60 + (longlong)puVar2;
            for (puVar14 = puStack_60; puVar14 != puVar2; puVar14 = puVar14 + 1) {
              auStack_120[0] = *puVar14;
              if (2 < *puRam000000014362c3a8) {
                FUN_1418b15d0(&uStack_f0,auStack_120);
                FUN_1418b17b0(&puStack_348,auStack_120);
                uStack_b0 = FUN_140bd7b50;
                uStack_a0 = FUN_140bd7ae0;
                uStack_b8 = (undefined **)&uStack_f0;
                ppuStack_a8 = &puStack_348;
                puVar10 = (undefined4 *)func_0x00014067dd80(&UNK_142b11170);
                uStack_308 = *puVar10;
                uStack_304 = puVar10[1];
                uStack_300 = puVar10[2];
                uStack_2fc = puVar10[3];
                uStack_2dc = puVar10[4];
                lStack_2f8 = 3;
                puStack_2f0 = &UNK_142b110c0;
                uStack_2e8 = 0x13;
                puStack_2d8 = &UNK_142b11150;
                uStack_2d0 = 2;
                ppuStack_2c8 = (undefined8 **)&uStack_b8;
                uStack_2c0 = 2;
                uStack_2b8 = 0;
                lStack_328 = 0;
                puStack_320 = &UNK_142b110c0;
                lStack_318 = 0x13;
                lStack_310 = 0;
                uStack_2e0 = 1;
                func_0x000140d4a330(&uStack_4a,&lStack_328);
                lVar3 = CONCAT44(uStack_f0._4_4_,(uint)uStack_f0);
                if ((!SBORROW8(0,lVar3)) && (lVar3 != 0)) {
                  func_0x000140613c20(CONCAT44(uStack_e4,uStack_e8),lVar3,1);
                }
              }
            }
            if (puStack_58 != (undefined8 *)0x0) {
              func_0x000140613c20(puStack_60,(longlong)puStack_58 << 3,8);
            }
          }
        }
        else if (1 < *puRam000000014362c3a8) {
          uStack_b0 = (code *)&UNK_140bd7a40;
          uStack_f0._0_4_ = uVar8;
          uStack_b8 = (undefined **)&uStack_f0;
          puVar10 = (undefined4 *)func_0x00014067dd80(&UNK_142b111e8);
          uStack_308 = *puVar10;
          uStack_304 = puVar10[1];
          uStack_300 = puVar10[2];
          uStack_2fc = puVar10[3];
          uStack_2dc = puVar10[4];
          lStack_2f8 = 2;
          puStack_2f0 = &UNK_142b110c0;
          uStack_2e8 = 0x13;
          puStack_2d8 = &UNK_142b111d8;
          uStack_2d0 = 1;
          ppuStack_2c8 = (undefined8 **)&uStack_b8;
          uStack_2c0 = 1;
          uStack_2b8 = 0;
          lStack_328 = 0;
          puStack_320 = &UNK_142b110c0;
          lStack_318 = 0x13;
          lStack_310 = 0;
          uStack_2e0 = 1;
          func_0x000140d4a330(&uStack_4a,&lStack_328);
        }
        puStack_c0[1] = lStack_68;
        *puStack_c0 = 0x18;
      }
      else {
        *puStack_c0 = 0x16;
        puStack_c0[1] = &UNK_142b11250;
        puStack_c0[2] = 0x1a;
        *(undefined4 *)(puStack_c0 + 3) = uStack_f0._4_4_;
      }
      return puStack_c0;
    }
    uStack_b8 = (undefined **)CONCAT44(uStack_b8._4_4_,puStack_320._0_4_);
    uStack_49 = 1;
    FUN_1428d9760(&UNK_142b1178e,0x2d,&uStack_b8,&UNK_142b10450,&UNK_142b117c0);
  }
  else {
    uVar15 = 0;
code_r0x000140bd877c:
    uStack_4b = 1;
    FUN_1428d8fe3(uVar15,uVar12,&UNK_142b10438);
  }
                    /* WARNING: Does not return */
  pcVar16 = (code *)invalidInstructionException();
  (*pcVar16)();
}

