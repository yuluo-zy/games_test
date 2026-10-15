  switch(uVar20) {
  case 0:
    uStack_63 = 0;
    FUN_1428d9430(&UNK_142c84625,0x28,&UNK_142c84650);
    goto code_r0x0001416718b9;
  case 1:
    if (cVar9 != '\x03') {
      uStack_548 = (longlong ******)CONCAT44(uStack_ac4,uStack_ac8);
      uStack_540 = (longlong ******)CONCAT44(uStack_abc,uStack_ac0);
      goto joined_r0x00014166e968;
    }
code_r0x00014166ead6:
    ppppplStack_490 = (longlong *****)0x0;
    ppppplStack_488 = (longlong *****)0x1;
    ppppplStack_480 = (longlong *****)0x0;
    ppppplStack_478 = (longlong *****)0x8000000000000000;
    uStack_438 = (longlong ******)((ulonglong)uStack_438 & 0xffffffffffffff00);
    ppppplStack_498 = (longlong *****)0x8000000000000001;
    uStack_430 = (longlong ******)0x8000000000000000;
    break;
  case 2:
    uStack_548 = (longlong ******)CONCAT44(uStack_b54,uStack_b58);
    uStack_540 = (longlong ******)CONCAT44(iStack_b4c,uStack_b50);
joined_r0x00014166e968:
    if ((longlong ******)ppppplStack_b60 == (longlong ******)0x8000000000000001) {
      uStack_63 = 0;
      cVar9 = FUN_140a9fa80(&uStack_548,*param_4);
      if (cVar9 != '\0') {
        if (2 < *puRam000000014362c3a8) {
          uStack_830 = FUN_141673430;
          uStack_63 = 0;
          uStack_838 = (longlong ******)&uStack_548;
          puVar11 = (undefined8 *)func_0x00014067dd80(&UNK_142c846d8);
          uStack_3a8 = (longlong ******)*puVar11;
          uStack_3a0 = (longlong ******)puVar11[1];
          uStack_398 = (longlong ******)0x3;
          uStack_390 = (longlong ******)&UNK_142c846f0;
          ppppplStack_388 = (longlong *****)0x26;
          uStack_378 = (longlong ******)&UNK_142c846b8;
          uStack_370 = (longlong ******)0x2;
          ppppplStack_368 = (longlong *****)&uStack_838;
          ppppplStack_360 = (longlong *****)0x1;
          puStack_358 = (undefined8 *)0x0;
          uStack_3c8 = (longlong ******)0x0;
          uStack_3c0 = (longlong ******)&UNK_142c846f0;
          uStack_3b8 = (longlong ******)0x26;
          uStack_3b0 = (longlong ******)0x0;
          uStack_380 = (longlong ******)CONCAT44(*(undefined4 *)(puVar11 + 2),1);
          uStack_63 = 0;
          func_0x000140d4a330(&uStack_68,&uStack_3c8);
        }
        *(undefined4 *)param_12[2] = *(undefined4 *)((longlong)param_12 + 0x1c);
        lVar13 = *param_12;
        lVar25 = *(longlong *)(lVar13 + 0x30);
        uVar19 = *(undefined8 *)(lVar13 + 0x40);
        if (lVar25 == *(longlong *)(lVar13 + 0x20)) {
          uStack_63 = 0;
          FUN_140562870(lVar13 + 0x20,&UNK_142c3a148);
        }
        *(undefined8 *)(*(longlong *)(lVar13 + 0x28) + lVar25 * 8) = uVar19;
        *(longlong *)(lVar13 + 0x30) = lVar25 + 1;
        *(longlong *)(lVar13 + 0x40) = *(longlong *)(lVar13 + 0x40) + 1;
        goto code_r0x00014166ead6;
      }
    }
    if (2 < *puRam000000014362c3a8) {
      uStack_838 = (longlong ******)&uStack_548;
      uStack_830 = FUN_141673430;
      uStack_63 = 0;
      puVar11 = (undefined8 *)func_0x00014067dd80(&UNK_142c84740);
      uStack_3a8 = (longlong ******)*puVar11;
      uStack_3a0 = (longlong ******)puVar11[1];
      uStack_398 = (longlong ******)0x3;
      uStack_390 = (longlong ******)&UNK_142c846f0;
      ppppplStack_388 = (longlong *****)0x26;
      uStack_378 = (longlong ******)&UNK_142c84730;
      uStack_370 = (longlong ******)0x1;
      ppppplStack_368 = (longlong *****)&uStack_838;
      ppppplStack_360 = (longlong *****)0x1;
      puStack_358 = (undefined8 *)0x0;
      uStack_3c8 = (longlong ******)0x0;
      uStack_3c0 = (longlong ******)&UNK_142c846f0;
      uStack_3b8 = (longlong ******)0x26;
      uStack_3b0 = (longlong ******)0x0;
      uStack_380 = (longlong ******)CONCAT44(*(undefined4 *)(puVar11 + 2),1);
      uStack_63 = 0;
      func_0x000140d4a330(&uStack_68,&uStack_3c8);
    }
    *(undefined4 *)param_11[2] = *(undefined4 *)((longlong)param_11 + 0x1c);
    lVar13 = *param_11;
    lVar25 = *(longlong *)(lVar13 + 0x30);
    uVar19 = *(undefined8 *)(lVar13 + 0x40);
    if (lVar25 == *(longlong *)(lVar13 + 0x20)) {
      uStack_63 = 0;
      FUN_140562870(lVar13 + 0x20,&UNK_142c89078);
    }
    *(undefined8 *)(*(longlong *)(lVar13 + 0x28) + lVar25 * 8) = uVar19;
    *(longlong *)(lVar13 + 0x30) = lVar25 + 1;
    *(longlong *)(lVar13 + 0x40) = *(longlong *)(lVar13 + 0x40) + 1;
    uStack_eb8 = (undefined4)uStack_548;
    uStack_eb4 = uStack_548._4_4_;
    uStack_eb0 = (uint)uStack_540;
    uStack_eac = uStack_540._4_4_;
    uStack_63 = 0;
    puVar16 = &uStack_eb8;
    FUN_140b46000(&uStack_aa8,puVar16);
    uVar19 = FUN_140d76ed0(&uStack_aa8);
    FUN_140d62810(&uStack_838,uVar19,puVar16,&UNK_142c845e7,0xc);
    uStack_dc8 = uStack_828;
    uStack_dc0 = uStack_820;
    uStack_dd8 = uStack_838;
    uStack_dd0 = (longlong ******)uStack_830;
    uStack_3b0._0_4_ = uStack_eb8;
    uStack_3b0._4_4_ = uStack_eb4;
    uStack_3a8 = (longlong ******)CONCAT44(uStack_eac,uStack_eb0);
    if (uStack_aa8 != (longlong ******)0x0) {
      func_0x000140613c20(uStack_aa0,uStack_aa8,1);
    }
    ppppplStack_488 = (longlong *****)uStack_dd8;
    ppppplStack_480 = (longlong *****)uStack_dd0;
    ppppplStack_478 = (longlong *****)uStack_dc8;
    ppppplStack_470 = (longlong *****)uStack_dc0;
    ppppplStack_400 = (longlong *****)uStack_3c8;
    ppppplStack_3f8 = (longlong *****)uStack_3c0;
    ppppplStack_3f0 = (longlong *****)uStack_3b8;
    uStack_3e8 = (longlong ******)CONCAT44(uStack_3b0._4_4_,(undefined4)uStack_3b0);
    ppppplStack_3e0 = (longlong *****)uStack_3a8;
    ppppplStack_498 = (longlong *****)0x8000000000000000;
    ppppplStack_490 = (longlong *****)0x0;
    ppppplStack_468 = (longlong *****)0x8000000000000000;
    ppppplStack_408 = (longlong *****)0x8000000000000000;
    uStack_3d8 = 0;
    uStack_3c0 = (longlong ******)0x0;
    uStack_3b8 = (longlong ******)0x1;
    uStack_3b0 = (longlong ******)0x0;
    uStack_3a8 = (longlong ******)0x8000000000000000;
    ppppplStack_368 = (longlong *****)((ulonglong)ppppplStack_368 & 0xffffffffffffff00);
    ppppplStack_360 = (longlong *****)0x8000000000000000;
    uStack_3c8 = (longlong ******)0x8000000000000001;
    if (uStack_120 == uStack_138) {
      FUN_140b5ac80(&uStack_138,&UNK_142c84758);
    }
    uVar14 = uStack_120;
    uVar20 = 0;
    if (uStack_138 <= lStack_128 + uStack_120) {
      uVar20 = uStack_138;
    }
    func_0x000142923c10(((lStack_128 + uStack_120) - uVar20) * 200 + lStack_130,&uStack_3c8,200);
    uStack_120 = uVar14 + 1;
    break;
  case 3:
    ppppplStack_568 = ppppplStack_b48;
    ppppplStack_560 = ppppplStack_b40;
    uStack_5b8 = uStack_b58;
    uStack_5b4 = uStack_b54;
    uStack_5b0 = uStack_b50;
    iStack_5ac = iStack_b4c;
    if (2 < *puRam000000014362c3a8) {
      uStack_81 = 1;
      puVar11 = (undefined8 *)func_0x00014067dd80(&UNK_142c847a8);
      uStack_3a8 = (longlong ******)*puVar11;
      uStack_3a0 = (longlong ******)puVar11[1];
      uStack_398 = (longlong ******)0x3;
      uStack_390 = (longlong ******)&UNK_142c846f0;
      ppppplStack_388 = (longlong *****)0x26;
      uStack_378 = (longlong ******)&UNK_142c84798;
      uStack_370 = (longlong ******)0x1;
      ppppplStack_368 = (longlong *****)0x8;
      uStack_3c8 = (longlong ******)0x0;
      ppppplStack_360 = (longlong *****)0x0;
      puStack_358 = (undefined8 *)0x0;
      uStack_3c0 = (longlong ******)&UNK_142c846f0;
      uStack_3b8 = (longlong ******)0x26;
      uStack_3b0 = (longlong ******)0x0;
      uStack_380 = (longlong ******)CONCAT44(*(undefined4 *)(puVar11 + 2),1);
      uStack_81 = 1;
      func_0x000140d4a330(&uStack_68,&uStack_3c8);
    }
    puVar16 = &uStack_5b8;
    FUN_140b46000(&uStack_3c8,puVar16);
    uVar19 = FUN_140d76ed0(&uStack_3c8);
    FUN_140d62810(&uStack_838,uVar19,puVar16,&UNK_142aef4ee,9);
    if (uStack_3c8 != (longlong ******)0x0) {
      func_0x000140613c20(uStack_3c0,uStack_3c8,1);
    }
    ppppplStack_498 = (longlong *****)uStack_838;
    ppppplStack_490 = (longlong *****)uStack_830;
    ppppplStack_488 = (longlong *****)uStack_828;
    ppppplStack_480 = (longlong *****)uStack_820;
    ppppplStack_478 = ppppplStack_568;
    ppppplStack_470 = ppppplStack_560;
    uStack_3e8 = (longlong ******)CONCAT44(uStack_5b4,uStack_5b8);
    ppppplStack_3e0 = (longlong *****)CONCAT44(iStack_5ac,uStack_5b0);
    ppppplStack_468 = (longlong *****)0x8000000000000000;
    ppppplStack_408 = (longlong *****)0x8000000000000000;
    uStack_3d8 = 0;
    break;
  default:
    uStack_838 = (longlong ******)ppppplStack_b60;
    pppppplVar12 = (longlong ******)CONCAT44(uStack_b54,uStack_b58);
    pppppplVar32 = (longlong ******)CONCAT44(iStack_b4c,uStack_b50);
    uStack_820 = (longlong ******)ppppplStack_b48;
    uStack_818 = (longlong ******)ppppplStack_b40;
    appppplStack_7e0[0] = ppppplStack_b08;
    ppppplVar18 = appppplStack_7e0[0];
    uStack_7f0 = (longlong ******)CONCAT44(uStack_b14,uStack_b18);
    uStack_7e8 = (longlong ******)CONCAT44(uStack_b0c,uStack_b10);
    uStack_800 = uStack_b28;
    uStack_7fc = uStack_b24;
    uStack_7f8 = (longlong ******)CONCAT44(uStack_b1c,uStack_b20);
    uStack_810 = uStack_b38;
    uStack_80c = uStack_b34;
    uStack_808 = uStack_b30;
    uStack_804 = uStack_b2c;
    ppppplStack_78 = ppppplStack_b00;
    ppppplStack_80 = ppppplStack_af8;
    uStack_830 = (code *)pppppplVar12;
    uStack_828 = pppppplVar32;
    if (iStack_b4c < 0) {
      uVar19 = 0;
code_r0x000141671890:
      FUN_1428d8fe3(uVar19,pppppplVar32,&UNK_142c38b48);
      goto code_r0x0001416718b9;
    }
    appppplStack_7e0[0]._0_1_ = (char)ppppplStack_b08;
    if (pppppplVar32 == (longlong ******)0x0) {
      pppppplVar10 = (longlong ******)0x1;
      pppppplVar31 = (longlong ******)ppppplStack_b08;
      uVar7 = (char)appppplStack_7e0[0];
      pppppplVar24 = (longlong ******)ppppplStack_a0;
    }
    else {
      ppppplStack_90 = (longlong *****)CONCAT71(ppppplStack_90._1_7_,(char)appppplStack_7e0[0]);
      uVar19 = 1;
      appppplStack_7e0[0] = ppppplVar18;
      pppppplVar10 = (longlong ******)func_0x000140613c10(pppppplVar32,1);
      if (pppppplVar10 == (longlong ******)0x0) goto code_r0x000141671890;
      pppppplVar24 = (longlong ******)((longlong)pppppplVar32 + -1);
      if (pppppplVar32 < (longlong ******)((longlong)pppppplVar32 + -1)) {
        pppppplVar24 = pppppplVar32;
      }
      if ((ulonglong)((longlong)pppppplVar10 - (longlong)pppppplVar12) < 0x20 ||
          pppppplVar24 < (longlong ******)0x20) {
        lVar25 = 0;
        pppppplVar17 = pppppplVar12;
        pppppplVar26 = pppppplVar32;
        uStack_b38 = extraout_XMM0_Da;
      }
      else {
        uVar20 = (longlong)pppppplVar24 + 1;
        uVar14 = 0x20;
        if ((uVar20 & 0x1f) != 0) {
          uVar14 = (ulonglong)((uint)uVar20 & 0x1f);
        }
        lVar25 = uVar20 - uVar14;
        pppppplVar26 = (longlong ******)((longlong)pppppplVar32 - lVar25);
        pppppplVar17 = (longlong ******)(lVar25 + (longlong)pppppplVar12);
        lVar13 = 0;
        do {
          puVar16 = (undefined4 *)((longlong)pppppplVar12 + lVar13);
          uStack_b38 = *puVar16;
          uVar34 = puVar16[1];
          uVar35 = puVar16[2];
          uVar36 = puVar16[3];
          puVar16 = (undefined4 *)((longlong)pppppplVar12 + lVar13 + 0x10);
          uStack_b28 = *puVar16;
          uStack_b24 = puVar16[1];
          uVar37 = puVar16[2];
          uVar38 = puVar16[3];
          puVar16 = (undefined4 *)((longlong)pppppplVar10 + lVar13);
          *puVar16 = uStack_b38;
          puVar16[1] = uVar34;
          puVar16[2] = uVar35;
          puVar16[3] = uVar36;
          puVar16 = (undefined4 *)((longlong)pppppplVar10 + lVar13 + 0x10);
          *puVar16 = uStack_b28;
          puVar16[1] = uStack_b24;
          puVar16[2] = uVar37;
          puVar16[3] = uVar38;
          lVar13 = lVar13 + 0x20;
        } while (lVar25 != lVar13);
      }
      puVar21 = (undefined *)((longlong)pppppplVar10 + lVar25);
      pppppplVar27 = (longlong ******)0x0;
      do {
        pbVar23 = (byte *)((longlong)pppppplVar17 + (longlong)pppppplVar27);
        pppppplVar31 = (longlong ******)appppplStack_7e0[0];
        uVar7 = ppppplStack_90._0_1_;
        pppppplVar24 = (longlong ******)ppppplStack_a0;
        if (pbVar23 == (byte *)((longlong)pppppplVar12 + (longlong)pppppplVar32)) break;
        bVar2 = *pbVar23;
        pbVar23 = (byte *)(ulonglong)bVar2;
        puVar21[(longlong)pppppplVar27] = bVar2;
        pppppplVar27 = (longlong ******)((longlong)pppppplVar27 + 1);
      } while (pppppplVar26 != pppppplVar27);
    }
    appppplStack_7e0[0] = (longlong *****)pppppplVar31;
    ppppplStack_a0 = (longlong *****)pppppplVar10;
    uStack_378._6_2_ = (undefined2)((ulonglong)uStack_378 >> 0x30);
    uVar22 = SUB81(pbVar23,0);
    uVar28 = SUB81(puVar21,0);
    uStack_3c0 = (longlong ******)ppppplStack_a0;
    uStack_3c8 = pppppplVar32;
    uStack_3b0 = (longlong ******)0x8000000000000000;
    if (!SBORROW8(0,(longlong)uStack_820)) {
      uVar28 = (undefined1)uStack_7f8;
      ppppplStack_90 = (longlong *****)pppppplVar32;
      FUN_140d7bd90(&uStack_aa8,&uStack_820);
      uStack_a80 = (longlong ******)CONCAT71(uStack_a80._1_7_,uVar28);
      uStack_dc8 = (longlong ******)CONCAT44(uStack_804,uStack_808);
      uStack_dc0 = (longlong ******)CONCAT44(uStack_7fc,uStack_800);
      uStack_b28 = SUB84(uStack_7f0,0);
      uStack_b24 = (undefined4)((ulonglong)uStack_7f0 >> 0x20);
      uVar28 = (undefined1)((ulonglong)uStack_7e8 >> 0x20);
      uVar22 = (undefined1)((ulonglong)uStack_7e8 >> 0x28);
      uStack_dd8 = uStack_aa0;
      uStack_dd0 = uStack_a98;
      uStack_db8 = uStack_a80;
      uStack_3c0 = (longlong ******)ppppplStack_a0;
      uStack_3c8 = (longlong ******)ppppplStack_90;
      uStack_3b0 = uStack_aa8;
      uStack_b38 = (undefined4)uStack_7e8;
      uStack_a90 = uStack_dc8;
      uStack_a88 = uStack_dc0;
      pppppplVar24 = (longlong ******)ppppplStack_a0;
    }
    ppppplStack_a0 = (longlong *****)pppppplVar24;
    ppppplVar18 = ppppplStack_78;
    pppppplVar32 = uStack_370;
    uStack_370 = (longlong ******)CONCAT71(uStack_370._1_7_,uVar7);
    pppppplVar12 = uStack_370;
    uStack_3a8 = uStack_dd8;
    uStack_3a0 = uStack_dd0;
    uStack_398 = uStack_dc8;
    uStack_390 = uStack_dc0;
    ppppplStack_388 = (longlong *****)uStack_db8;
    uStack_380 = (longlong ******)CONCAT44(uStack_b24,uStack_b28);
    uStack_378._0_5_ = CONCAT14(uVar28,uStack_b38);
    uStack_378._0_6_ = CONCAT15(uVar22,(undefined5)uStack_378);
    uStack_3b8 = uStack_3c8;
    if (SBORROW8(0,(longlong)ppppplStack_78)) {
      uStack_440 = (longlong ******)
                   CONCAT26(uStack_378._6_2_,CONCAT15(uVar22,(undefined5)uStack_378));
      uStack_370._1_4_ = SUB84(pppppplVar32,1);
      uStack_370._5_2_ = SUB82(pppppplVar32,5);
      uStack_370._7_1_ = SUB81(pppppplVar32,7);
      uStack_438 = (longlong ******)
                   CONCAT17(uStack_370._7_1_,
                            CONCAT25(uStack_370._5_2_,CONCAT41(uStack_370._1_4_,uVar7)));
      uStack_450 = uStack_db8;
      uStack_420 = (longlong ******)ppppplStack_af0;
      uStack_418 = (longlong ******)CONCAT71(uStack_418._1_7_,uStack_ae8);
      ppppplStack_498 = (longlong *****)0x8000000000000001;
      uStack_430 = (longlong ******)0x8000000000000000;
      ppppplStack_490 = (longlong *****)uStack_3c8;
      ppppplStack_488 = (longlong *****)uStack_3c0;
      ppppplStack_480 = (longlong *****)uStack_3c8;
      ppppplStack_478 = (longlong *****)uStack_3b0;
      ppppplStack_470 = (longlong *****)uStack_3a8;
      ppppplStack_468 = (longlong *****)uStack_3a0;
      ppppplStack_460 = (longlong *****)uStack_398;
      uStack_458 = uStack_390;
      uStack_448 = uStack_380;
      uStack_370 = pppppplVar12;
    }
    else {
      if ((longlong)ppppplStack_af0 < 0) {
        uVar19 = 0;
code_r0x0001416718a7:
        FUN_1428d8fe3(uVar19,ppppplStack_af0,&UNK_142c38b78);
        goto code_r0x0001416718b9;
      }
      if ((longlong ******)ppppplStack_af0 == (longlong ******)0x0) {
        pppppplVar12 = (longlong ******)0x1;
      }
      else {
        uVar19 = 1;
        pppppplVar12 = (longlong ******)func_0x000140613c10(ppppplStack_af0,1);
        if (pppppplVar12 == (longlong ******)0x0) goto code_r0x0001416718a7;
      }
      ppppplVar29 = ppppplStack_80;
      func_0x000142923c00(pppppplVar12,ppppplStack_80,ppppplStack_af0);
      ppppplStack_490 = (longlong *****)uStack_3c8;
      ppppplStack_488 = (longlong *****)uStack_3c0;
      ppppplStack_480 = (longlong *****)uStack_3b8;
      ppppplStack_478 = (longlong *****)uStack_3b0;
      ppppplStack_470 = (longlong *****)uStack_3a8;
      ppppplStack_468 = (longlong *****)uStack_3a0;
      ppppplStack_460 = (longlong *****)uStack_398;
      uStack_458 = uStack_390;
      uStack_450 = (longlong ******)ppppplStack_388;
      uStack_448 = uStack_380;
      uStack_440 = uStack_378;
      uStack_438 = uStack_370;
      uStack_420 = (longlong ******)ppppplStack_af0;
      uStack_418 = (longlong ******)CONCAT71(uStack_418._1_7_,uStack_ae8);
      ppppplStack_498 = (longlong *****)0x8000000000000001;
      uStack_430 = (longlong ******)ppppplStack_af0;
      ppppplStack_428 = (longlong *****)pppppplVar12;
      if ((longlong ******)ppppplVar18 != (longlong ******)0x0) {
        func_0x000140613c20(ppppplVar29,ppppplVar18,1);
      }
    }
    if (uStack_838 != (longlong ******)0x0) {
      func_0x000140613c20(uStack_830,uStack_838,1);
    }
    if ((!SBORROW8(0,(longlong)uStack_820)) && (uStack_820 != (longlong ******)0x0)) {
      func_0x000140613c20(uStack_818,uStack_820,1);
    }
  }
  lStack_108 = lStack_b68;
  lStack_118 = lStack_b78;
  uStack_110 = (undefined4)uStack_b70;
  uStack_10c = uStack_b70._4_4_;
  lStack_b78 = 0;
  uStack_b70 = 8;
  lStack_b68 = 0;
  lStack_4e0 = *param_3;
  uStack_3cc = *(undefined4 *)((longlong)param_10 + 0x1c);
  puStack_b0 = (undefined8 *)*param_10;
  puStack_5a0 = (undefined4 *)param_10[2];
  puStack_598 = puStack_b0 + 4;
  while( true ) {
    func_0x000142923c00(&uStack_eb8,&ppppplStack_498,200);
    uVar28 = uStack_df7;
    uVar7 = uStack_df8;
    ppppplStack_498 = (longlong *****)0x8000000000000002;
    pppppplVar12 = (longlong ******)CONCAT44(uStack_eb4,uStack_eb8);
    if (pppppplVar12 == (longlong ******)0x8000000000000002) break;
    pppppplVar32 = (longlong ******)CONCAT44(uStack_eac,uStack_eb0);
    if (pppppplVar12 == (longlong ******)0x8000000000000001) {
      uStack_530 = (longlong ******)ppppplStack_478;
      uStack_528 = (longlong ******)ppppplStack_470;
      uStack_540 = (longlong ******)ppppplStack_488;
      uStack_538 = (longlong ******)ppppplStack_480;
      ppppplStack_4f0 = (longlong *****)uStack_438;
      uStack_500 = (undefined4)uStack_448;
      uStack_4fc = uStack_448._4_4_;
      uStack_4f8 = (undefined4)uStack_440;
      uStack_4f4 = uStack_440._4_4_;
      uStack_510 = (undefined4)uStack_458;
      uStack_50c = uStack_458._4_4_;
      uStack_508 = (undefined4)uStack_450;
      uStack_504 = uStack_450._4_4_;
      ppppplStack_520 = ppppplStack_468;
      ppppplStack_518 = ppppplStack_460;
      ppppplStack_b8 = ppppplStack_428;
      uStack_ad8 = (undefined4)uStack_420;
      uStack_ad4 = uStack_420._4_4_;
      uStack_ad0 = (undefined4)uStack_418;
      uStack_acc = uStack_418._4_4_;
      ppppplStack_78 = (longlong *****)uStack_430;
      uStack_548 = pppppplVar32;
      if (2 < *puRam000000014362c3a8) {
        ppppplStack_a8 = (longlong *****)uStack_430;
        uStack_61 = 1;
        uStack_62 = 1;
        puVar11 = (undefined8 *)func_0x00014067dd80(&UNK_142c84928);
        uStack_3a8 = (longlong ******)*puVar11;
        uStack_3a0 = (longlong ******)puVar11[1];
        uStack_398 = (longlong ******)0x3;
        uStack_390 = (longlong ******)&UNK_142c846f0;
        ppppplStack_388 = (longlong *****)0x26;
        uStack_378 = (longlong ******)&UNK_142c84918;
        uStack_370 = (longlong ******)0x1;
        ppppplStack_368 = (longlong *****)0x8;
        uStack_3c8 = (longlong ******)0x0;
        ppppplStack_360 = (longlong *****)0x0;
        puStack_358 = (undefined8 *)0x0;
        uStack_3c0 = (longlong ******)&UNK_142c846f0;
        uStack_3b8 = (longlong ******)0x26;
        uStack_3b0 = (longlong ******)0x0;
        uStack_380 = (longlong ******)CONCAT44(*(undefined4 *)(puVar11 + 2),1);
        ppppplStack_a8 = ppppplStack_78;
        uStack_61 = 1;
        uStack_62 = 1;
        func_0x000140d4a330(&uStack_68,&uStack_3c8);
      }
      uStack_61 = 1;
      uStack_62 = 1;
      ppppplStack_a8 = ppppplStack_78;
      FUN_140c102e0(&uStack_3c8);
      uStack_558 = (int)uStack_3c8;
      uStack_554 = uStack_3c8._4_4_;
      uStack_550 = (undefined4)uStack_3c0;
      uStack_54c = uStack_3c0._4_4_;
      ppppplStack_a8 = ppppplStack_78;
      uStack_61 = 1;
      uStack_62 = 1;
      puVar16 = &uStack_558;
      FUN_140b46000(&uStack_3c8,puVar16);
      uStack_838 = (longlong ******)CONCAT71(uStack_838._1_7_,1);
      ppppplVar18 = (longlong *****)&uStack_3c8;
      uVar19 = FUN_140d76ed0(ppppplVar18);
      pppppplVar12 = (longlong ******)FUN_140d5a150(&uStack_838,uVar19,puVar16);
      if (uStack_3c8 != (longlong ******)0x0) {
        func_0x000140613c20(uStack_3c0,uStack_3c8,1);
      }
      if (pppppplVar12 != (longlong ******)0x0) {
        uStack_3c8 = pppppplVar12;
        FUN_1428d9760(&UNK_142c84940,0x20,&uStack_3c8,&UNK_142c84590,&UNK_142c84960);
        goto code_r0x0001416718b9;
      }
      uStack_3c8 = (longlong ******)CONCAT44(uStack_554,uStack_558);
      uStack_3c0 = (longlong ******)CONCAT44(uStack_54c,uStack_550);
      ppppplStack_a8 = ppppplStack_78;
      uStack_61 = 1;
      uStack_62 = 1;
      pppppplVar12 = (longlong ******)FUN_140b463c0(&uStack_548,ppppplVar18);
      if (pppppplVar12 != (longlong ******)0x0) {
        uStack_3c8 = pppppplVar12;
        FUN_1428d9760(&UNK_142c84978,0x1d,&uStack_3c8,&UNK_142c84570,&UNK_142c84998);
        goto code_r0x0001416718b9;
      }
      *puStack_5a0 = uStack_3cc;
      lVar13 = puStack_b0[6];
      uVar19 = puStack_b0[8];
      if (lVar13 == puStack_b0[4]) {
        ppppplStack_a8 = ppppplStack_78;
        uStack_61 = 1;
        uStack_62 = 1;
        ppppplVar18 = (longlong *****)&UNK_142c89078;
        FUN_140562870(puStack_598,&UNK_142c89078);
      }
      *(undefined8 *)(puStack_b0[5] + lVar13 * 8) = uVar19;
      puStack_b0[6] = lVar13 + 1;
      puStack_b0[8] = puStack_b0[8] + 1;
      uStack_92 = SBORROW8(0,(longlong)ppppplStack_78);
      if ((bool)uStack_92) {
        ppppplVar18 = (longlong *****)&UNK_142c849b0;
        if (cRam0000000143634d90 != '\0') {
          ppppplVar18 = (longlong *****)&UNK_142c849cb;
        }
        uStack_5c0 = 0x1b;
        pppplStack_5c8 = (longlong ****)ppppplVar18;
        if (*(longlong *)(lStack_4e0 + 0x1d8) == 0) {
          ppppplStack_a8 = (longlong *****)0x8000000000000000;
          uStack_61 = 1;
          uStack_62 = 1;
          FUN_140d62810(&pppplStack_f8,ppppplVar18,0x1b,&UNK_142c849e6,5);
        }
        else {
          ppppplStack_a8 = (longlong *****)0x8000000000000000;
          uStack_61 = 1;
          uStack_62 = 1;
          FUN_140d68f80(&uStack_3c8,ppppplVar18,0x1b);
          if ((int)uStack_3c8 == 2) {
            uStack_830 = (code *)uStack_3c0;
            uStack_838 = (longlong ******)0x2;
          }
          else {
            func_0x000142923c00(&uStack_838,&uStack_3c8,0x270);
          }
          ppppplStack_a8 = (longlong *****)0x8000000000000000;
          uStack_61 = 1;
          uStack_62 = 1;
          FUN_1415a5b60(&uStack_aa8,&uStack_838,&pppplStack_5c8);
          if ((int)uStack_aa8 == 2) {
            uStack_3c8 = uStack_aa0;
            FUN_1428d9760(&UNK_142c845b0,0x2b,&uStack_3c8,&UNK_142c84570,&UNK_142c849f0);
            goto code_r0x0001416718b9;
          }
          func_0x000142923c00(&uStack_3c8,&uStack_aa8,0x270);
          ppppplStack_150 = (longlong *****)0x0;
          ppppplStack_148 = (longlong *****)0x8;
          ppppplStack_140 = (longlong *****)0x0;
code_r0x00014166fe27:
          while (FUN_140d59e80(&uStack_838,&uStack_3c8), ((ulonglong)uStack_838 & 1) != 0) {
            func_0x000142923c00(&uStack_dd8,&uStack_830,600);
            if (uStack_dd8 == (longlong ******)0x0) goto code_r0x000141670071;
            func_0x000142923c00(&uStack_aa8,&uStack_830,600);
            puVar11 = &uStack_aa8;
            FUN_140d59f10(&ppppplStack_d8,&uStack_aa8);
            uStack_6a = 1;
            uVar19 = FUN_140d76ed0(&ppppplStack_d8);
            uStack_6a = 1;
            cVar9 = FUN_140d62e10(uVar19);
            if (cVar9 == '\0') goto code_r0x000141670030;
            uStack_6a = 1;
            uVar19 = FUN_140d76ed0(&ppppplStack_d8);
            uStack_6a = 1;
            lVar13 = FUN_140d61e00(uVar19);
            if (lVar13 == 0) {
              uStack_6a = 1;
              FUN_1428d9310(&UNK_142c84a80);
              goto code_r0x0001416718b9;
            }
            uStack_6a = 1;
            FUN_140d58e70(&uStack_4b8,lVar13,puVar11);
            ppppplVar18 = ppppplStack_140;
            if (((((ulonglong)uStack_4b8 & 1) == 0) && (lStack_4a8 == 5)) &&
               ((char)((int *)CONCAT44(uStack_4ac,uStack_4b0))[1] == 'o' &&
                *(int *)CONCAT44(uStack_4ac,uStack_4b0) == 0x72746e69)) {
              if ((longlong ******)ppppplStack_d8 != (longlong ******)0x0) {
                func_0x000140613c20(ppppplStack_d0,ppppplStack_d8,1);
              }
            }
            else {
              ppppplStack_80 = ppppplStack_d8;
              ppppplStack_90 = ppppplStack_d0;