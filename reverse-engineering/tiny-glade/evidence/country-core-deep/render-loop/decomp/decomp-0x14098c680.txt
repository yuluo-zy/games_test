
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void FUN_14098c680(undefined8 param_1,longlong param_2)

{
  undefined8 uVar1;
  code *pcVar2;
  undefined8 uVar3;
  undefined8 uVar4;
  undefined8 uVar5;
  undefined8 uVar6;
  undefined4 uVar7;
  longlong lVar8;
  undefined8 uVar9;
  longlong lVar10;
  undefined8 *puVar11;
  undefined8 *puVar12;
  undefined4 *puVar13;
  longlong *plVar14;
  ulonglong uVar15;
  ulonglong uVar16;
  undefined4 extraout_XMM0_Da;
  undefined4 extraout_XMM0_Da_00;
  undefined4 extraout_XMM0_Da_01;
  undefined4 extraout_XMM0_Da_02;
  undefined1 auStack_c48 [248];
  undefined1 auStack_b50 [248];
  ulonglong uStack_a58;
  undefined1 *puStack_a50;
  undefined8 uStack_a48;
  undefined8 uStack_a40;
  undefined8 uStack_a38;
  undefined8 uStack_a30;
  undefined8 uStack_a28;
  undefined8 uStack_a20;
  undefined8 uStack_a18;
  undefined4 uStack_a10;
  undefined4 uStack_a0c;
  undefined4 uStack_a08;
  undefined4 uStack_a04;
  undefined8 uStack_a00;
  undefined8 uStack_9f8;
  ulonglong uStack_9f0;
  undefined *puStack_9e8;
  undefined8 uStack_9e0;
  undefined8 uStack_9d8;
  undefined4 uStack_9d0;
  undefined4 uStack_9cc;
  undefined4 uStack_9c8;
  undefined4 uStack_9c4;
  undefined4 uStack_9c0;
  undefined4 uStack_9bc;
  undefined4 uStack_9b8;
  undefined4 uStack_9b4;
  undefined4 uStack_9b0;
  undefined4 uStack_9ac;
  undefined4 uStack_9a8;
  undefined4 uStack_9a4;
  undefined8 uStack_9a0;
  undefined8 uStack_998;
  undefined8 uStack_990;
  undefined8 uStack_988;
  undefined8 uStack_980;
  undefined8 uStack_978;
  undefined4 uStack_970;
  undefined4 uStack_96c;
  undefined8 uStack_968;
  undefined8 uStack_960;
  undefined8 uStack_958;
  undefined8 uStack_950;
  undefined1 auStack_938 [22];
  char cStack_922;
  undefined8 uStack_920;
  undefined8 uStack_8d8;
  undefined1 auStack_66c [4];
  undefined1 auStack_668 [32];
  ulonglong uStack_648;
  undefined1 *puStack_640;
  undefined8 uStack_638;
  undefined8 uStack_630;
  undefined8 uStack_628;
  undefined8 uStack_620;
  undefined8 uStack_618;
  undefined8 uStack_610;
  undefined8 uStack_608;
  undefined8 uStack_600;
  undefined8 uStack_5f8;
  undefined8 uStack_5f0;
  undefined8 uStack_5e8;
  undefined8 uStack_5e0;
  undefined *puStack_5d8;
  undefined8 uStack_5d0;
  undefined8 uStack_5c8;
  undefined4 uStack_5c0;
  undefined4 uStack_5bc;
  undefined4 uStack_5b8;
  undefined4 uStack_5b4;
  undefined4 uStack_5b0;
  undefined4 uStack_5ac;
  undefined4 uStack_5a8;
  undefined4 uStack_5a4;
  undefined4 uStack_5a0;
  undefined4 uStack_59c;
  undefined4 uStack_598;
  undefined4 uStack_594;
  undefined8 uStack_590;
  undefined8 uStack_588;
  undefined8 uStack_580;
  undefined8 uStack_578;
  undefined8 uStack_570;
  undefined8 uStack_568;
  undefined4 uStack_560;
  undefined4 uStack_55c;
  undefined8 uStack_558;
  undefined8 uStack_550;
  undefined8 uStack_548;
  undefined8 uStack_540;
  undefined1 auStack_528 [22];
  char cStack_512;
  undefined8 uStack_510;
  undefined8 uStack_4c8;
  undefined8 uStack_258;
  ulonglong uStack_250;
  undefined *puStack_248;
  undefined8 uStack_240;
  undefined8 uStack_238;
  undefined8 uStack_230;
  undefined8 uStack_228;
  undefined8 uStack_220;
  undefined8 uStack_218;
  undefined8 uStack_210;
  undefined8 uStack_208;
  undefined8 uStack_200;
  undefined4 uStack_1f8;
  undefined1 uStack_1f4;
  undefined8 uStack_1e8;
  undefined8 uStack_1e0;
  undefined8 uStack_1d8;
  undefined8 uStack_1d0;
  undefined4 uStack_1c8;
  undefined4 uStack_1c4;
  undefined4 uStack_1c0;
  undefined4 uStack_1bc;
  undefined8 uStack_1b8;
  undefined4 uStack_1b0;
  undefined4 uStack_1ac;
  undefined2 uStack_1a8;
  ulonglong *puStack_c0;
  undefined8 uStack_b8;
  undefined4 uStack_b0;
  undefined4 uStack_ac;
  ulonglong uStack_a8;
  undefined4 uStack_a0;
  undefined4 uStack_9c;
  ulonglong uStack_98;
  undefined8 *puStack_90;
  ulonglong uStack_88;
  longlong lStack_80;
  undefined1 uStack_71;
  undefined8 uStack_70;
  
  uStack_70 = 0xfffffffffffffffe;
  lStack_80 = param_2 + 0x38;
  uVar15 = 0xa68561b396784eaa;
  lVar8 = FUN_1400dd6f0(lStack_80,0xa68561b396784eaa,0xf5fd74e6faf4ed14);
  if (((lVar8 == 0) || (*(ulonglong *)(param_2 + 0x218) <= uVar15)) ||
     (uVar15 = *(ulonglong *)(*(longlong *)(param_2 + 0x210) + uVar15 * 8), uVar15 == 0)) {
code_r0x00014098d8d1:
    FUN_1428d9310(&UNK_142ab9180);
  }
  else {
    lVar8 = ~uVar15 * 0x60;
    if (*(longlong *)(*(longlong *)(param_2 + 0x1e0) + 0x20 + lVar8) == 0)
    goto code_r0x00014098d8d1;
    FUN_14093cf30(auStack_66c,*(undefined8 *)(*(longlong *)(param_2 + 0x1e0) + lVar8 + 0x10),10);
    uVar9 = func_0x000140bd5790(auStack_668);
    uStack_648 = 0;
    uStack_638 = 1;
    puStack_640 = auStack_66c;
    uStack_b8 = param_1;
    FUN_140867970(&uStack_a58,&UNK_142ab9198,0x15,&uStack_258,0,&uStack_648,uVar9,param_2,param_1);
    uStack_71 = 1;
    uVar15 = 0xd3bda3878f7a9d01;
    lVar8 = FUN_1400dd6f0(lStack_80,0xd3bda3878f7a9d01,0x5dda49fe35d11774);
    if (((lVar8 != 0) && (uVar15 < *(ulonglong *)(param_2 + 0x218))) &&
       (uVar15 = *(ulonglong *)(*(longlong *)(param_2 + 0x210) + uVar15 * 8), uVar15 != 0)) {
      lVar8 = ~uVar15 * 0x60;
      if (*(longlong *)(*(longlong *)(param_2 + 0x1e0) + 0x20 + lVar8) != 0) {
        puStack_c0 = *(ulonglong **)(*(longlong *)(param_2 + 0x1e0) + lVar8 + 0x10);
        uVar15 = *puStack_c0;
        uVar16 = uVar15 & 0xffffffff;
        if (((uVar16 < *(ulonglong *)(param_2 + 0x10)) &&
            (*(int *)(*(longlong *)(param_2 + 8) + uVar16 * 0x14) == (int)(uVar15 >> 0x20))) &&
           (lVar8 = *(longlong *)(param_2 + 8) + uVar16 * 0x14, *(int *)(lVar8 + 4) != -1)) {
          uVar16 = (ulonglong)*(uint *)(lVar8 + 0xc);
          uStack_71 = 1;
          uStack_88 = uVar15;
          lVar8 = FUN_1409871a0(lStack_80,0x93c2ea0b5dbce370,0xc3521cd69f6dc3c8);
          if ((lVar8 != 0) && (uVar16 < *(ulonglong *)(param_2 + 0x1b0))) {
            lVar8 = uVar16 * 0x48;
            uStack_71 = 1;
            puStack_90 = (undefined8 *)func_0x000140d00b50(*(longlong *)(param_2 + 0x1a8) + lVar8);
            if (puStack_90 != (undefined8 *)0x0) {
              uStack_71 = 1;
              lVar10 = FUN_1409871a0(lStack_80,0x6deafeaf70315369,0x7ccce21f8e4423ee);
              if ((lVar10 != 0) && (uVar16 < *(ulonglong *)(param_2 + 0x1b0))) {
                uStack_71 = 1;
                puVar11 = (undefined8 *)func_0x000140d00b50(*(longlong *)(param_2 + 0x1a8) + lVar8);
                if (puVar11 != (undefined8 *)0x0) {
                  uStack_71 = 1;
                  lVar10 = FUN_1409871a0(lStack_80,0x8672044888361604,0x9340ff6b57fed7e4);
                  if ((lVar10 != 0) && (uVar16 < *(ulonglong *)(param_2 + 0x1b0))) {
                    uStack_71 = 1;
                    puVar12 = (undefined8 *)
                              func_0x000140d00b50(*(longlong *)(param_2 + 0x1a8) + lVar8);
                    if (puVar12 != (undefined8 *)0x0) {
                      uStack_71 = 1;
                      lVar10 = FUN_1409871a0(lStack_80,0xb04edb2d6216372f,0x541721773055ec85);
                      uStack_a0 = 2;
                      uStack_9c = extraout_XMM0_Da;
                      if ((lVar10 != 0) && (uVar16 < *(ulonglong *)(param_2 + 0x1b0))) {
                        uStack_71 = 1;
                        puVar13 = (undefined4 *)
                                  func_0x000140d00b50(lVar8 + *(longlong *)(param_2 + 0x1a8));
                        uStack_9c = extraout_XMM0_Da_00;
                        if (puVar13 != (undefined4 *)0x0) {
                          uStack_a0 = *puVar13;
                          uStack_9c = puVar13[1];
                        }
                      }
                      uStack_98 = uStack_88;
                      uVar9 = *puVar11;
                      uVar1 = *puStack_90;
                      uStack_71 = 1;
                      uVar7 = func_0x000140be6750();
                      uStack_71 = 1;
                      plVar14 = (longlong *)FUN_1400d9b20(0);
                      if (plVar14 == (longlong *)0x0) {
                        uStack_71 = 1;
                        FUN_1428d80d0(&UNK_142c3aaa0);
                        goto code_r0x00014098da29;
                      }
                      uStack_1a8 = 0;
                      uStack_1ac = 0x3f800000;
                      uStack_258 = 1;
                      puStack_248 = (undefined *)0x0;
                      uStack_240 = 4;
                      uStack_238 = 0;
                      uStack_1c8 = (undefined4)*plVar14;
                      uStack_1c4 = *(undefined4 *)((longlong)plVar14 + 4);
                      uStack_1c0 = (undefined4)plVar14[1];
                      uStack_1bc = *(undefined4 *)((longlong)plVar14 + 0xc);
                      *plVar14 = *plVar14 + 1;
                      uVar6 = _UNK_142a96738;
                      uVar5 = _UNK_142a96730;
                      uVar4 = _UNK_142a96728;
                      uVar3 = _UNK_142a96720;
                      uStack_1d8 = _UNK_142a96730;
                      uStack_1d0 = _UNK_142a96738;
                      uStack_1e8 = _UNK_142a96720;
                      uStack_1e0 = _UNK_142a96728;
                      uStack_218 = 0x100000003;
                      uStack_230 = 0;
                      uStack_228 = 1;
                      uStack_220 = 0;
                      uStack_71 = 1;
                      uStack_250 = uVar1;
                      uStack_1b8 = uVar9;
                      uStack_1b0 = uVar7;
                      FUN_140905be0(&uStack_648,&uStack_258,&uStack_a58,1);
                      uStack_208 = *puStack_90;
                      uStack_200 = *puVar11;
                      uStack_238 = *puVar12;
                      uStack_230 = puVar12[1];
                      uStack_228 = puVar12[2];
                      uStack_220 = puVar12[3];
                      uStack_218 = puVar12[4];
                      uStack_210 = puVar12[5];
                      uStack_258 = CONCAT44(uStack_9c,uStack_a0);
                      uStack_250 = uStack_98;
                      uStack_1f4 = 0;
                      puStack_248 = &UNK_142ab9228;
                      uStack_240 = 0xe;
                      uStack_1f8 = 0;
                      uStack_71 = 1;
                      FUN_14090b460(auStack_b50,&uStack_648,&UNK_142ab9238,&uStack_258);
                      uStack_71 = 1;
                      FUN_1409cacb0(auStack_b50,&uStack_a0,param_2,1);
                      uVar15 = puStack_c0[1];
                      uVar16 = uVar15 & 0xffffffff;
                      if (((*(ulonglong *)(param_2 + 0x10) <= uVar16) ||
                          (*(int *)(*(longlong *)(param_2 + 8) + uVar16 * 0x14) !=
                           (int)(uVar15 >> 0x20))) ||
                         (lVar8 = *(longlong *)(param_2 + 8) + uVar16 * 0x14,
                         *(int *)(lVar8 + 4) == -1)) {
                        uStack_71 = 1;
                        uStack_648 = uVar15;
                        FUN_1428d9760(&UNK_142ab8978,0x2b,&uStack_648,&UNK_142ab8958,&UNK_142ab9240)
                        ;
                        goto code_r0x00014098da29;
                      }
                      uVar16 = (ulonglong)*(uint *)(lVar8 + 0xc);
                      uStack_71 = 1;
                      uStack_88 = uVar15;
                      lVar8 = FUN_1409871a0(lStack_80,0x93c2ea0b5dbce370,0xc3521cd69f6dc3c8);
                      if ((lVar8 != 0) && (uVar16 < *(ulonglong *)(param_2 + 0x1b0))) {
                        lVar8 = uVar16 * 0x48;
                        uStack_71 = 1;
                        puStack_90 = (undefined8 *)
                                     func_0x000140d00b50(*(longlong *)(param_2 + 0x1a8) + lVar8);
                        if (puStack_90 != (undefined8 *)0x0) {
                          uStack_71 = 1;
                          lVar10 = FUN_1409871a0(lStack_80,0x6deafeaf70315369,0x7ccce21f8e4423ee);
                          if ((lVar10 != 0) && (uVar16 < *(ulonglong *)(param_2 + 0x1b0))) {
                            uStack_71 = 1;
                            puVar11 = (undefined8 *)
                                      func_0x000140d00b50(*(longlong *)(param_2 + 0x1a8) + lVar8);
                            if (puVar11 != (undefined8 *)0x0) {
                              uStack_71 = 1;
                              lVar10 = FUN_1409871a0(lStack_80,0x8672044888361604,0x9340ff6b57fed7e4
                                                    );
                              if ((lVar10 != 0) && (uVar16 < *(ulonglong *)(param_2 + 0x1b0))) {
                                uStack_71 = 1;
                                puVar12 = (undefined8 *)
                                          func_0x000140d00b50(*(longlong *)(param_2 + 0x1a8) + lVar8
                                                             );
                                if (puVar12 != (undefined8 *)0x0) {
                                  uStack_71 = 1;
                                  lVar10 = FUN_1409871a0(lStack_80,0xb04edb2d6216372f,
                                                         0x541721773055ec85);
                                  uStack_b0 = 2;
                                  uStack_ac = extraout_XMM0_Da_01;
                                  if ((lVar10 != 0) && (uVar16 < *(ulonglong *)(param_2 + 0x1b0))) {
                                    uStack_71 = 1;
                                    puVar13 = (undefined4 *)
                                              func_0x000140d00b50(lVar8 + *(longlong *)
                                                                           (param_2 + 0x1a8));
                                    uStack_ac = extraout_XMM0_Da_02;
                                    if (puVar13 != (undefined4 *)0x0) {
                                      uStack_b0 = *puVar13;
                                      uStack_ac = puVar13[1];
                                    }
                                  }
                                  uStack_a8 = uStack_88;
                                  uVar9 = *puVar11;
                                  uVar1 = *puStack_90;
                                  uStack_71 = 1;
                                  uVar7 = func_0x000140be6750();
                                  uStack_71 = 1;
                                  plVar14 = (longlong *)FUN_1400d9b20(0);
                                  if (plVar14 == (longlong *)0x0) {
                                    uStack_71 = 1;
                                    FUN_1428d80d0(&UNK_142c3aaa0);
                                    goto code_r0x00014098da29;
                                  }
                                  uStack_1a8 = 0;
                                  uStack_1ac = 0x3f800000;
                                  uStack_258 = 1;
                                  puStack_248 = (undefined *)0x0;
                                  uStack_240 = 4;
                                  uStack_238 = 0;
                                  uStack_1c8 = (undefined4)*plVar14;
                                  uStack_1c4 = *(undefined4 *)((longlong)plVar14 + 4);
                                  uStack_1c0 = (undefined4)plVar14[1];
                                  uStack_1bc = *(undefined4 *)((longlong)plVar14 + 0xc);
                                  *plVar14 = *plVar14 + 1;
                                  uStack_1d8 = uVar5;
                                  uStack_1d0 = uVar6;
                                  uStack_1e8 = uVar3;
                                  uStack_1e0 = uVar4;
                                  uStack_218 = 0x100000003;
                                  uStack_230 = 0;
                                  uStack_228 = 1;
                                  uStack_220 = 0;
                                  uStack_71 = 1;
                                  uStack_250 = uVar1;
                                  uStack_1b8 = uVar9;
                                  uStack_1b0 = uVar7;
                                  FUN_140905be0(&uStack_648,&uStack_258,&uStack_a58,1);
                                  uStack_208 = *puStack_90;
                                  uStack_200 = *puVar11;
                                  uStack_238 = *puVar12;
                                  uStack_230 = puVar12[1];
                                  uStack_228 = puVar12[2];
                                  uStack_220 = puVar12[3];
                                  uStack_218 = puVar12[4];
                                  uStack_210 = puVar12[5];
                                  uStack_258 = CONCAT44(uStack_ac,uStack_b0);
                                  uStack_250 = uStack_a8;
                                  uStack_1f4 = 0;
                                  puStack_248 = &UNK_142ab92a0;
                                  uStack_240 = 0xd;
                                  uStack_1f8 = 0;
                                  uStack_71 = 1;
                                  FUN_14090b460(auStack_c48,&uStack_648,&UNK_142ab9238,&uStack_258);
                                  uStack_71 = 1;
                                  FUN_1409cb4e0(auStack_c48,&uStack_b0,param_2,1);
                                  func_0x000142923c00(&uStack_648,&uStack_a58,1000);
                                  uStack_71 = 0;
                                  FUN_140868370(&uStack_648);
                                  uVar9 = uStack_b8;
                                  FUN_1416032a0(&uStack_a58,&UNK_142ab92c8,0x13,&UNK_142ab92db,0x2c,
                                                param_2,uStack_b8);
                                  uStack_258 = CONCAT71(uStack_258._1_7_,9);
                                  uStack_648 = CONCAT44(uStack_648._4_4_,2);
                                  FUN_140905260(&uStack_258,&uStack_a58,0x61b08b5e0470c5f2,0x19,
                                                &uStack_648);
                                  func_0x000142923c00(&uStack_648,&uStack_a58,400);
                                  auStack_66c[0] = 10;
                                  uStack_258 = CONCAT44(uStack_258._4_4_,2);
                                  FUN_140905260(auStack_66c,&uStack_648,0x297d1e4cdae8f657,0x19,
                                                &uStack_258);
                                  func_0x000142923c00(&uStack_258,&uStack_648,400);
                                  FUN_141604690(&uStack_648,&uStack_258,0xb25729c74f87a459);
                                  FUN_141607950(&uStack_a58,&uStack_648,0x7703bd797f8b276e);
                                  uStack_238 = uStack_9d8;
                                  puStack_248 = puStack_9e8;
                                  uStack_240 = uStack_9e0;
                                  uStack_258 = uStack_9f8;
                                  uStack_250 = uStack_9f0;
                                  uStack_638 = uStack_a48;
                                  uStack_648 = uStack_a58;
                                  puStack_640 = puStack_a50;
                                  uStack_618 = uStack_a40;
                                  uStack_610 = uStack_a38;
                                  uStack_608 = uStack_a30;
                                  uStack_630 = uStack_a28;
                                  uStack_628 = uStack_a20;
                                  uStack_620 = uStack_a18;
                                  uStack_5d0 = CONCAT44(uStack_9a4,uStack_9a8);
                                  uStack_5c8 = uStack_9a0;
                                  uStack_5c0 = (undefined4)uStack_998;
                                  uStack_5bc = uStack_998._4_4_;
                                  uStack_5b8 = (undefined4)uStack_990;
                                  uStack_5b4 = uStack_990._4_4_;
                                  uStack_5b0 = (undefined4)uStack_988;
                                  uStack_5ac = uStack_988._4_4_;
                                  uStack_5a8 = (undefined4)uStack_980;
                                  uStack_5a4 = uStack_980._4_4_;
                                  uStack_5a0 = (undefined4)uStack_978;
                                  uStack_59c = uStack_978._4_4_;
                                  uStack_598 = uStack_970;
                                  uStack_594 = uStack_96c;
                                  uStack_590 = uStack_968;
                                  uStack_588 = uStack_960;
                                  uStack_580 = uStack_958;
                                  uStack_578 = uStack_950;
                                  uStack_5f0 = uStack_a00;
                                  uStack_600 = CONCAT44(uStack_a0c,uStack_a10);
                                  uStack_5f8 = CONCAT44(uStack_a04,uStack_a08);
                                  uStack_568 = uStack_8d8;
                                  uStack_570 = uStack_920;
                                  uStack_5e8 = CONCAT44(_UNK_142ab8414,_UNK_142ab8410);
                                  uStack_5e0 = CONCAT44(_UNK_142ab841c,_UNK_142ab8418);
                                  FUN_1409dd4d0(&uStack_258,&uStack_648);
                                  if (cStack_922 != '\b') {
                                    FUN_140be24b0(auStack_938);
                                  }
                                  FUN_1416032a0(&uStack_648,&UNK_142ab9307,0x16,&UNK_142ab931d,0x2f,
                                                param_2,uVar9);
                                  uStack_258 = CONCAT71(uStack_258._1_7_,0xe);
                                  uStack_a58 = CONCAT44(uStack_a58._4_4_,2);
                                  FUN_140905260(&uStack_258,&uStack_648,0x4ee098b38612d2b9,0x19,
                                                &uStack_a58);
                                  func_0x000142923c00(&uStack_a58,&uStack_648,400);
                                  FUN_141605510(&uStack_648,&uStack_a58,0x4ee7d41b19c88236);
                                  uVar15 = _UNK_142ab8428;
                                  uVar1 = _UNK_142ab8420;
                                  uStack_218 = CONCAT44(uStack_5a4,uStack_5a8);
                                  uStack_210 = CONCAT44(uStack_59c,uStack_5a0);
                                  uStack_228 = CONCAT44(uStack_5b4,uStack_5b8);
                                  uStack_220 = CONCAT44(uStack_5ac,uStack_5b0);
                                  uStack_230 = CONCAT44(uStack_5bc,uStack_5c0);
                                  uStack_238 = uStack_5c8;
                                  puStack_248 = puStack_5d8;
                                  uStack_240 = uStack_5d0;
                                  uStack_258 = uStack_5e8;
                                  uStack_250 = uStack_5e0;
                                  uStack_a48 = uStack_638;
                                  uStack_a58 = uStack_648;
                                  puStack_a50 = puStack_640;
                                  uStack_a28 = uStack_630;
                                  uStack_a20 = uStack_628;
                                  uStack_a18 = uStack_620;
                                  uStack_a40 = uStack_618;
                                  uStack_a38 = uStack_610;
                                  uStack_a30 = uStack_608;
                                  uStack_9e0 = CONCAT44(uStack_594,uStack_598);
                                  uStack_9d8 = uStack_590;
                                  uStack_9d0 = (undefined4)uStack_588;
                                  uStack_9cc = uStack_588._4_4_;
                                  uStack_9c8 = (undefined4)uStack_580;
                                  uStack_9c4 = uStack_580._4_4_;
                                  uStack_9c0 = (undefined4)uStack_578;
                                  uStack_9bc = uStack_578._4_4_;
                                  uStack_9b8 = (undefined4)uStack_570;
                                  uStack_9b4 = uStack_570._4_4_;
                                  uStack_9b0 = (undefined4)uStack_568;
                                  uStack_9ac = uStack_568._4_4_;
                                  uStack_9a8 = uStack_560;
                                  uStack_9a4 = uStack_55c;
                                  uStack_9a0 = uStack_558;
                                  uStack_998 = uStack_550;
                                  uStack_990 = uStack_548;
                                  uStack_988 = uStack_540;
                                  uStack_a00 = uStack_5f0;
                                  uStack_a10 = (undefined4)uStack_600;
                                  uStack_a0c = uStack_600._4_4_;
                                  uStack_a08 = (undefined4)uStack_5f8;
                                  uStack_a04 = uStack_5f8._4_4_;
                                  uStack_978 = uStack_4c8;
                                  uStack_980 = uStack_510;
                                  uStack_9f8 = _UNK_142ab8420;
                                  uStack_9f0 = _UNK_142ab8428;
                                  FUN_1409dd4d0(&uStack_258,&uStack_a58);
                                  if (cStack_512 != '\b') {
                                    FUN_140be24b0(auStack_528);
                                  }
                                  if (cRam0000000143634d90 == '\0') {
                                    FUN_1416032a0(&uStack_648,&UNK_142ab934c,10,&UNK_142ab9356,0x1e,
                                                  param_2,uVar9);
                                    uStack_258 = CONCAT71(uStack_258._1_7_,0x13);
                                    uStack_a58 = CONCAT44(uStack_a58._4_4_,2);
                                    FUN_140905260(&uStack_258,&uStack_648,0x6ad63ccd18c8f041,0x19,
                                                  &uStack_a58);
                                    func_0x000142923c00(&uStack_a58,&uStack_648,400);
                                    FUN_141607ec0(&uStack_648,&uStack_a58,0x4ee7d41b19c88236);
                                    FUN_1416044c0(&uStack_a58,&uStack_648,0x7864ea0bf1becbd8);
                                    uStack_218 = CONCAT44(uStack_9b4,uStack_9b8);
                                    uStack_210 = CONCAT44(uStack_9ac,uStack_9b0);
                                    uStack_228 = CONCAT44(uStack_9c4,uStack_9c8);
                                    uStack_220 = CONCAT44(uStack_9bc,uStack_9c0);
                                    uStack_230 = CONCAT44(uStack_9cc,uStack_9d0);
                                    uStack_238 = uStack_9d8;
                                    puStack_248 = puStack_9e8;
                                    uStack_240 = uStack_9e0;
                                    uStack_258 = uStack_9f8;
                                    uStack_250 = uStack_9f0;
                                    uStack_638 = uStack_a48;
                                    uStack_648 = uStack_a58;
                                    puStack_640 = puStack_a50;
                                    uStack_618 = uStack_a40;
                                    uStack_610 = uStack_a38;
                                    uStack_608 = uStack_a30;
                                    uStack_630 = uStack_a28;
                                    uStack_628 = uStack_a20;
                                    uStack_620 = uStack_a18;
                                    uStack_5d0 = CONCAT44(uStack_9a4,uStack_9a8);
                                    uStack_5c8 = uStack_9a0;
                                    uStack_5c0 = (undefined4)uStack_998;
                                    uStack_5bc = uStack_998._4_4_;
                                    uStack_5b8 = (undefined4)uStack_990;
                                    uStack_5b4 = uStack_990._4_4_;
                                    uStack_5b0 = (undefined4)uStack_988;
                                    uStack_5ac = uStack_988._4_4_;
                                    uStack_5a8 = (undefined4)uStack_980;
                                    uStack_5a4 = uStack_980._4_4_;
                                    uStack_5a0 = (undefined4)uStack_978;
                                    uStack_59c = uStack_978._4_4_;
                                    uStack_598 = uStack_970;
                                    uStack_594 = uStack_96c;
                                    uStack_590 = uStack_968;
                                    uStack_588 = uStack_960;
                                    uStack_580 = uStack_958;
                                    uStack_578 = uStack_950;
                                    uStack_5f0 = uStack_a00;
                                    uStack_600 = CONCAT44(uStack_a0c,uStack_a10);
                                    uStack_5f8 = CONCAT44(uStack_a04,uStack_a08);
                                    uStack_568 = uStack_8d8;
                                    uStack_570 = uStack_920;
                                    uStack_5e8 = uVar1;
                                    uStack_5e0 = uVar15;
                                    FUN_1409dd4d0(&uStack_258,&uStack_648);
                                    if (cStack_922 != '\b') {
                                      FUN_140be24b0(auStack_938);
                                    }
                                  }
                                  FUN_1416032a0(&uStack_648,&UNK_142ab9374,0xb,&UNK_142ab937f,0x20,
                                                param_2,uVar9);
                                  uStack_258 = CONCAT71(uStack_258._1_7_,0x14);
                                  uStack_a58 = CONCAT44(uStack_a58._4_4_,2);
                                  FUN_140905260(&uStack_258,&uStack_648,0x49f43a487883738d,0x19,
                                                &uStack_a58);
                                  func_0x000142923c00(&uStack_a58,&uStack_648,400);
                                  FUN_141605340(&uStack_648,&uStack_a58,0x4ee7d41b19c88236);
                                  uStack_218 = CONCAT44(uStack_5a4,uStack_5a8);
                                  uStack_210 = CONCAT44(uStack_59c,uStack_5a0);
                                  uStack_228 = CONCAT44(uStack_5b4,uStack_5b8);
                                  uStack_220 = CONCAT44(uStack_5ac,uStack_5b0);
                                  uStack_230 = CONCAT44(uStack_5bc,uStack_5c0);
                                  uStack_238 = uStack_5c8;
                                  puStack_248 = puStack_5d8;
                                  uStack_240 = uStack_5d0;
                                  uStack_258 = uStack_5e8;
                                  uStack_250 = uStack_5e0;
                                  uStack_a48 = uStack_638;
                                  uStack_a58 = uStack_648;
                                  puStack_a50 = puStack_640;
                                  uStack_a28 = uStack_630;
                                  uStack_a20 = uStack_628;
                                  uStack_a18 = uStack_620;
                                  uStack_a40 = uStack_618;
                                  uStack_a38 = uStack_610;
                                  uStack_a30 = uStack_608;
                                  uStack_9e0 = CONCAT44(uStack_594,uStack_598);
                                  uStack_9d8 = uStack_590;
                                  uStack_9d0 = (undefined4)uStack_588;
                                  uStack_9cc = uStack_588._4_4_;
                                  uStack_9c8 = (undefined4)uStack_580;
                                  uStack_9c4 = uStack_580._4_4_;
                                  uStack_9c0 = (undefined4)uStack_578;
                                  uStack_9bc = uStack_578._4_4_;
                                  uStack_9b8 = (undefined4)uStack_570;
                                  uStack_9b4 = uStack_570._4_4_;
                                  uStack_9b0 = (undefined4)uStack_568;
                                  uStack_9ac = uStack_568._4_4_;
                                  uStack_9a8 = uStack_560;
                                  uStack_9a4 = uStack_55c;
                                  uStack_9a0 = uStack_558;
                                  uStack_998 = uStack_550;
                                  uStack_990 = uStack_548;
                                  uStack_988 = uStack_540;
                                  uStack_a00 = uStack_5f0;
                                  uStack_a10 = (undefined4)uStack_600;
                                  uStack_a0c = uStack_600._4_4_;
                                  uStack_a08 = (undefined4)uStack_5f8;
                                  uStack_a04 = uStack_5f8._4_4_;
                                  uStack_978 = uStack_4c8;
                                  uStack_980 = uStack_510;
                                  uStack_9f8 = uVar1;
                                  uStack_9f0 = uVar15;
                                  FUN_1409dd4d0(&uStack_258,&uStack_a58);
                                  if (cStack_512 != '\b') {
                                    FUN_140be24b0(auStack_528);
                                  }
                                  uVar16 = 0xfc8691076ab4d826;
                                  uVar15 = 0x24ccb35ae3644351;
                                  lVar8 = FUN_1400dd6f0(lStack_80);
                                  if (((lVar8 != 0) && (uVar16 < *(ulonglong *)(param_2 + 0x218)))
                                     && (uVar16 = *(ulonglong *)
                                                   (*(longlong *)(param_2 + 0x210) + uVar16 * 8),
                                        uVar16 != 0)) {
                                    lVar8 = ~uVar16 * 0x60;
                                    if (*(longlong *)(*(longlong *)(param_2 + 0x1e0) + 0x20 + lVar8)
                                        != 0) {
                                      lVar8 = *(longlong *)(param_2 + 0x1e0) + lVar8;
                                      plVar14 = *(longlong **)(lVar8 + 0x10);
                                      *(undefined4 *)(lVar8 + 0x5c) =
                                           *(undefined4 *)(param_2 + 0x5b0);
                                      lVar8 = *plVar14;
                                      LOCK();
                                      if (lVar8 == *plVar14) {
                                        *plVar14 = 0;
                                      }
                                      UNLOCK();
                                      *(byte *)(plVar14 + 1) = *(byte *)(plVar14 + 1) | lVar8 != 0;
                                      return;
                                    }
                                  }
                                  FUN_1428d9310(&UNK_142ab92b0);
                                  goto code_r0x00014098d901;
                                }
                              }
                              uStack_71 = 1;
                              FUN_1428d9310(&UNK_142ab9288);
                              goto code_r0x00014098da29;
                            }
                          }
                          uStack_71 = 1;
                          FUN_1428d9310(&UNK_142ab9270);
                          goto code_r0x00014098da29;
                        }
                      }
                      uStack_71 = 1;
                      FUN_1428d9310(&UNK_142ab9258);
                      goto code_r0x00014098da29;
                    }
                  }
                  uStack_71 = 1;
                  FUN_1428d9310(&UNK_142ab9210);
                  goto code_r0x00014098da29;
                }
              }
              uStack_71 = 1;
              FUN_1428d9310(&UNK_142ab91f8);
              goto code_r0x00014098da29;
            }
          }
          uStack_71 = 1;
          FUN_1428d9310(&UNK_142ab91e0);
        }
        else {
code_r0x00014098d901:
          uStack_71 = 1;
          uStack_648 = uVar15;
          FUN_1428d9760(&UNK_142ab8978,0x2b,&uStack_648,&UNK_142ab8958,&UNK_142ab91c8);
        }
        goto code_r0x00014098da29;
      }
    }
  }
  uStack_71 = 1;
  FUN_1428d9310(&UNK_142ab91b0);
code_r0x00014098da29:
                    /* WARNING: Does not return */
  pcVar2 = (code *)invalidInstructionException();
  (*pcVar2)();
}

