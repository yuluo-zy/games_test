
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void FUN_1409daaa0(longlong *param_1,longlong param_2)

{
  byte bVar1;
  byte bVar2;
  byte bVar3;
  float *pfVar4;
  uint *puVar5;
  byte *pbVar6;
  code *pcVar7;
  char cVar8;
  longlong lVar9;
  undefined *puVar10;
  undefined *puVar11;
  undefined8 uVar12;
  longlong *plVar13;
  uint uVar14;
  ulonglong uVar15;
  longlong lVar16;
  ulonglong uVar17;
  undefined *puVar18;
  undefined *puVar19;
  undefined1 uVar20;
  undefined4 uVar21;
  undefined4 uVar22;
  undefined4 uVar23;
  float fVar24;
  float fVar25;
  undefined4 uStack_9c8;
  undefined4 uStack_9c4;
  undefined4 uStack_9c0;
  undefined4 uStack_9bc;
  undefined4 uStack_9b8;
  undefined4 uStack_9b4;
  undefined4 uStack_9b0;
  float fStack_9ac;
  longlong lStack_9a8;
  longlong lStack_9a0;
  longlong lStack_998;
  undefined4 *puStack_990;
  longlong lStack_988;
  longlong lStack_980;
  undefined8 uStack_978;
  undefined8 uStack_970;
  undefined8 uStack_968;
  undefined8 uStack_960;
  undefined8 uStack_958;
  undefined8 uStack_950;
  undefined8 uStack_948;
  undefined8 uStack_940;
  undefined1 uStack_938;
  undefined3 uStack_933;
  undefined4 uStack_5e0;
  undefined4 uStack_5dc;
  undefined4 uStack_5d8;
  undefined4 uStack_5d4;
  undefined4 uStack_5d0;
  undefined4 uStack_5cc;
  undefined1 *puStack_5c8;
  undefined8 uStack_5c0;
  undefined8 uStack_5b8;
  undefined8 uStack_5b0;
  undefined8 uStack_5a8;
  undefined8 uStack_5a0;
  undefined4 uStack_570;
  undefined4 uStack_56c;
  undefined4 uStack_568;
  undefined4 uStack_564;
  undefined8 uStack_560;
  undefined8 uStack_558;
  undefined4 uStack_550;
  undefined4 uStack_54c;
  undefined4 uStack_548;
  undefined4 uStack_544;
  undefined *puStack_540;
  uint uStack_538;
  undefined8 uStack_534;
  undefined8 uStack_52c;
  undefined4 uStack_524;
  undefined4 uStack_520;
  undefined4 uStack_51c;
  undefined4 uStack_518;
  undefined4 uStack_514;
  undefined4 uStack_510;
  undefined4 uStack_50c;
  undefined4 uStack_508;
  undefined4 uStack_504;
  undefined4 uStack_500;
  undefined4 uStack_4fc;
  undefined4 uStack_4f8;
  undefined4 uStack_4f4;
  undefined4 uStack_4f0;
  undefined4 uStack_4ec;
  longlong lStack_4e8;
  longlong lStack_4e0;
  undefined1 uStack_4d8;
  undefined1 auStack_1f4 [4];
  undefined1 auStack_1f0 [32];
  undefined4 uStack_1d0;
  undefined4 uStack_1cc;
  undefined1 auStack_1c8 [160];
  undefined1 uStack_128;
  undefined2 uStack_127;
  undefined1 uStack_125;
  undefined8 uStack_124;
  undefined8 uStack_11c;
  undefined4 uStack_114;
  undefined4 uStack_110;
  undefined4 uStack_10c;
  undefined4 uStack_108;
  undefined4 uStack_104;
  undefined4 uStack_100;
  undefined4 uStack_fc;
  undefined4 uStack_f8;
  undefined4 uStack_f4;
  undefined4 uStack_f0;
  undefined4 uStack_ec;
  undefined4 uStack_e8;
  undefined4 uStack_e4;
  undefined4 uStack_e0;
  undefined4 uStack_dc;
  longlong lStack_d8;
  longlong lStack_d0;
  longlong lStack_b8;
  longlong *plStack_b0;
  longlong lStack_a8;
  undefined *puStack_a0;
  longlong lStack_98;
  undefined8 uStack_90;
  longlong lStack_88;
  longlong *plStack_80;
  longlong *plStack_78;
  longlong *plStack_70;
  undefined1 uStack_61;
  undefined8 uStack_60;
  
  uStack_60 = 0xfffffffffffffffe;
  plStack_70 = param_1;
  cVar8 = func_0x000140c16a00();
  if (cVar8 == '\0') {
code_r0x0001409dab3f:
    lStack_98 = 0;
code_r0x0001409dad1d:
    lVar9 = param_2 + 0x38;
    uVar15 = 0x8597ee0727b461b3;
    lVar16 = FUN_1400dd6f0(lVar9,0x8597ee0727b461b3,0x8b297ec5ae15f15c);
    if (((lVar16 != 0) && (uVar15 < *(ulonglong *)(param_2 + 0x218))) &&
       (uVar15 = *(ulonglong *)(*(longlong *)(param_2 + 0x210) + uVar15 * 8), uVar15 != 0)) {
      lVar16 = ~uVar15 * 0x60;
      if (*(longlong *)(*(longlong *)(param_2 + 0x1e0) + 0x20 + lVar16) != 0) {
        puVar18 = &UNK_142ac2b38;
        uVar15 = FUN_140937aa0(*(undefined8 *)(*(longlong *)(param_2 + 0x1e0) + lVar16 + 0x10),
                               &UNK_142ac2b38,0x11);
        puStack_a0 = puVar18;
        if ((uVar15 & 1) == 0) {
          FUN_1428d9310(&UNK_142ac2b50);
          goto code_r0x0001409db707;
        }
        uVar15 = 0xf9c1dd2a6b8903b0;
        lVar16 = FUN_1400dd6f0(lVar9,0xf9c1dd2a6b8903b0,0xf1d4572b4d023299);
        if (((lVar16 != 0) && (uVar15 < *(ulonglong *)(param_2 + 0x218))) &&
           (uVar15 = *(ulonglong *)(*(longlong *)(param_2 + 0x210) + uVar15 * 8), uVar15 != 0)) {
          lVar16 = ~uVar15 * 0x60;
          if (*(longlong *)(*(longlong *)(param_2 + 0x1e0) + 0x20 + lVar16) != 0) {
            pfVar4 = *(float **)(*(longlong *)(param_2 + 0x1e0) + lVar16 + 0x10);
            uVar15 = 0xa68561b396784eaa;
            lVar16 = FUN_1400dd6f0(lVar9,0xa68561b396784eaa,0xf5fd74e6faf4ed14);
            if (((lVar16 != 0) && (uVar15 < *(ulonglong *)(param_2 + 0x218))) &&
               (uVar15 = *(ulonglong *)(*(longlong *)(param_2 + 0x210) + uVar15 * 8), uVar15 != 0))
            {
              lVar16 = ~uVar15 * 0x60;
              if (*(longlong *)(*(longlong *)(param_2 + 0x1e0) + 0x20 + lVar16) != 0) {
                FUN_14093cf30(auStack_1f4,
                              *(undefined8 *)(*(longlong *)(param_2 + 0x1e0) + lVar16 + 0x10),0xf);
                uVar15 = 0xc2b8e941c91a6428;
                lVar16 = FUN_1400dd6f0(lVar9,0xc2b8e941c91a6428,0x59f3c6d1d3da43a0);
                if (((lVar16 != 0) && (uVar15 < *(ulonglong *)(param_2 + 0x218))) &&
                   (uVar15 = *(ulonglong *)(*(longlong *)(param_2 + 0x210) + uVar15 * 8),
                   uVar15 != 0)) {
                  lVar16 = ~uVar15 * 0x60;
                  if (*(longlong *)(*(longlong *)(param_2 + 0x1e0) + 0x20 + lVar16) != 0) {
                    puVar5 = *(uint **)(*(longlong *)(param_2 + 0x1e0) + lVar16 + 0x10);
                    lVar16 = 0;
                    if (*puVar5 - 6 < 0xf) {
                      lVar16 = (ulonglong)*puVar5 - 5;
                    }
                    fVar25 = 0.0;
                    switch(lVar16) {
                    case 1:
                      if ((char)puVar5[1] == '\0') goto code_r0x0001409daf73;
                      break;
                    case 2:
                    case 3:
                      fVar25 = *pfVar4 + _UNK_142934690;
                      break;
                    case 4:
                      if ((char)puVar5[1] != '\x03') goto code_r0x0001409daf5c;
                      break;
                    case 5:
code_r0x0001409daf5c:
                      fVar25 = _UNK_142ac1970;
                      break;
                    case 0xc:
                      fVar25 = pfVar4[2] * _UNK_1429edb90 + _UNK_142934690;
                      break;
                    case 0xd:
code_r0x0001409daf73:
                      fVar25 = _UNK_1429258d0;
                      break;
                    case 0xe:
                      uVar15 = 0xbba451ee6a08a3ff;
                      lVar16 = FUN_1400dd6f0(lVar9,0xbba451ee6a08a3ff,0x8d008d15febcf29d);
                      if (((lVar16 != 0) && (uVar15 < *(ulonglong *)(param_2 + 0x218))) &&
                         (uVar15 = *(ulonglong *)(*(longlong *)(param_2 + 0x210) + uVar15 * 8),
                         uVar15 != 0)) {
                        lVar16 = ~uVar15 * 0x60;
                        if (*(longlong *)(*(longlong *)(param_2 + 0x1e0) + 0x20 + lVar16) != 0) {
                          if (*(char *)(*(longlong *)
                                         (*(longlong *)(param_2 + 0x1e0) + lVar16 + 0x10) + 0x4d0)
                              == '\x03') {
                            fVar25 = _UNK_142a7ba84;
                          }
                          break;
                        }
                      }
                      FUN_1428d9310(&UNK_142ac2bb0);
                      goto code_r0x0001409db707;
                    }
                    FUN_14165a540(&uStack_5e0,param_2);
                    FUN_141657e90(&uStack_5e0,param_2);
                    if (CONCAT44(uStack_5dc,uStack_5e0) == *(longlong *)(param_2 + 0x5a0)) {
                      uStack_1d0 = uStack_9c8;
                      uStack_1cc = CONCAT13(uStack_1cc._3_1_,(undefined3)uStack_9c4);
                      uVar21 = 0;
                      uVar22 = 0;
                      uVar23 = 0;
                      fVar24 = 0.0;
                      if (0.0 < fVar25) {
                        uVar15 = 0xbba451ee6a08a3ff;
                        lVar16 = FUN_1400dd6f0(lVar9,0xbba451ee6a08a3ff,0x8d008d15febcf29d);
                        if (((lVar16 != 0) && (uVar15 < *(ulonglong *)(param_2 + 0x218))) &&
                           (uVar15 = *(ulonglong *)(*(longlong *)(param_2 + 0x210) + uVar15 * 8),
                           uVar15 != 0)) {
                          lVar16 = ~uVar15 * 0x60;
                          if (*(longlong *)(*(longlong *)(param_2 + 0x1e0) + 0x20 + lVar16) != 0) {
                            if (*(char *)(*(longlong *)
                                           (*(longlong *)(param_2 + 0x1e0) + lVar16 + 0x10) + 0x4d0)
                                != '\x03') {
                              uVar15 = 0x62dfe8f62b6fcc5;
                              lVar9 = FUN_1400dd6f0(lVar9,0x62dfe8f62b6fcc5,0xf65008e548632ba7);
                              if (((lVar9 != 0) && (uVar15 < *(ulonglong *)(param_2 + 0x218))) &&
                                 (uVar15 = *(ulonglong *)
                                            (*(longlong *)(param_2 + 0x210) + uVar15 * 8),
                                 uVar15 != 0)) {
                                lVar9 = ~uVar15 * 0x60;
                                if (*(longlong *)(*(longlong *)(param_2 + 0x1e0) + 0x20 + lVar9) !=
                                    0) {
                                  pbVar6 = *(byte **)(*(longlong *)(param_2 + 0x1e0) + lVar9 + 0x10)
                                  ;
                                  if ((*pbVar6 & 1) == 0) {
                                    uVar21 = 0;
                                    uVar22 = 0;
                                    uStack_9c0 = 0;
                                  }
                                  else {
                                    uVar21 = (undefined4)*(undefined8 *)(pbVar6 + 4);
                                    uVar22 = (undefined4)
                                             ((ulonglong)*(undefined8 *)(pbVar6 + 4) >> 0x20);
                                    uStack_9c0 = *(undefined4 *)(pbVar6 + 0xc);
                                  }
                                  goto code_r0x0001409db21a;
                                }
                              }
                              FUN_1428d9310(&UNK_142ac2c10);
                              goto code_r0x0001409db707;
                            }
                            uVar15 = 0xbba451ee6a08a3ff;
                            lVar9 = FUN_1400dd6f0(lVar9,0xbba451ee6a08a3ff,0x8d008d15febcf29d);
                            if (((lVar9 != 0) && (uVar15 < *(ulonglong *)(param_2 + 0x218))) &&
                               (uVar15 = *(ulonglong *)(*(longlong *)(param_2 + 0x210) + uVar15 * 8)
                               , uVar15 != 0)) {
                              lVar9 = ~uVar15 * 0x60;
                              if (*(longlong *)(*(longlong *)(param_2 + 0x1e0) + 0x20 + lVar9) != 0)
                              {
                                func_0x000140c96d40(&uStack_9c8,
                                                    *(undefined8 *)
                                                     (*(longlong *)(param_2 + 0x1e0) + lVar9 + 0x10)
                                                   );
                                uVar21 = uStack_9c8;
                                uVar22 = uStack_9c4;
code_r0x0001409db21a:
                                uStack_9c8 = 1;
                                uVar23 = uStack_9c0;
                                fVar24 = fVar25;
                                goto code_r0x0001409db21f;
                              }
                            }
                            FUN_1428d9310(&UNK_142ac2bf8);
                            goto code_r0x0001409db707;
                          }
                        }
                        FUN_1428d9310(&UNK_142ac2be0);
                        goto code_r0x0001409db707;
                      }
                      uStack_9c8 = 0;
code_r0x0001409db21f:
                      lStack_980 = lStack_4e0 * 4 + lStack_4e8;
                      lStack_998 = param_2 + 0xf8;
                      lStack_9a0 = param_2 + 0x1a0;
                      lStack_988 = lStack_4e8;
                      uStack_978 = 8;
                      uStack_970 = 0;
                      uStack_968 = 8;
                      uStack_960 = 0;
                      uStack_958 = 0;
                      uStack_950 = 0;
                      uStack_948 = 0;
                      uStack_940 = 0;
                      uStack_938 = uStack_4d8;
                      uStack_933 = (undefined3)uStack_1cc;
                      uStack_9c4 = 0;
                      uStack_9c0 = 0;
                      uStack_9bc = 0;
                      uStack_9b8 = uVar21;
                      uStack_9b4 = uVar22;
                      uStack_9b0 = uVar23;
                      fStack_9ac = fVar24;
                      lStack_9a8 = param_2;
                      puStack_990 = &uStack_5e0;
                      FUN_1412fde80(&lStack_b8,&uStack_9c8,&UNK_142ac1e40);
                      uStack_61 = 1;
                      FUN_1405d0be0(&uStack_5e0);
                      uStack_61 = 1;
                      uVar12 = func_0x000140bd5790(auStack_1f0);
                      uStack_5e0 = 1;
                      uStack_5dc = 0;
                      uStack_5d8 = 0;
                      uStack_5d4 = 0;
                      uStack_5d0 = 0;
                      uStack_5cc = 0;
                      uStack_1d0 = 2;
                      uStack_61 = 1;
                      puStack_5c8 = auStack_1f4;
                      FUN_140867970(&uStack_9c8,&UNK_142ac2bc8,0x16,&uStack_5e0,1,&uStack_1d0,uVar12
                                    ,param_2,plStack_70);
                      lStack_88 = lStack_b8;
                      plStack_78 = plStack_b0 + lStack_a8 * 2;
                      plStack_80 = plStack_b0;
                      plStack_70 = plStack_b0;
                      if (plStack_b0 != plStack_78) {
                        do {
                          lVar9 = *plStack_b0;
                          lVar16 = plStack_b0[1];
                          plStack_70 = plStack_b0;
                          uVar21 = func_0x000140be6750();
                          plVar13 = (longlong *)FUN_1400d9b20(0);
                          if (plVar13 == (longlong *)0x0) {
                            FUN_1428d80d0(&UNK_142c3aaa0);
                            goto code_r0x0001409db707;
                          }
                          uVar15 = (ulonglong)uStack_534 >> 0x20;
                          uStack_534 = CONCAT44((uint)uVar15 & 0xffff0000,0x3f800000);
                          uStack_5e0 = 0;
                          uStack_5dc = 0;
                          puStack_540 = puStack_a0;
                          uStack_5d0 = 0;
                          uStack_5cc = 0;
                          puStack_5c8 = (undefined1 *)0x4;
                          uStack_5c0 = 0;
                          uStack_550 = (undefined4)*plVar13;
                          uStack_54c = *(undefined4 *)((longlong)plVar13 + 4);
                          uStack_548 = (undefined4)plVar13[1];
                          uStack_544 = *(undefined4 *)((longlong)plVar13 + 0xc);
                          *plVar13 = *plVar13 + 1;
                          uStack_560 = _UNK_142a96730;
                          uStack_558 = _UNK_142a96738;
                          uStack_570 = _UNK_142a96720;
                          uStack_56c = _UNK_142a96724;
                          uStack_568 = _UNK_142a96728;
                          uStack_564 = _UNK_142a9672c;
                          uStack_5a0 = 0x100000003;
                          uStack_5b8 = 0;
                          uStack_5b0 = 1;
                          uStack_5a8 = 0;
                          uStack_538 = uVar21;
                          FUN_140905be0(&uStack_1d0,&uStack_5e0,&uStack_9c8,0);
                          if (uStack_f4._2_1_ != '\b') {
                            lStack_d8 = lVar9;
                            lStack_d0 = lVar16;
                            FUN_14096bb20((longlong)&uStack_11c + 4,
                                          CONCAT44(uStack_e4,uStack_e8) + 0xe8,0x337ad802c8b34f67);
                          }
                          uVar22 = uStack_1cc;
                          uVar21 = uStack_1d0;
                          func_0x000142923c00(&uStack_5d8,auStack_1c8,0xa0);
                          uStack_534 = uStack_124;
                          uStack_52c = uStack_11c;
                          uStack_524 = uStack_114;
                          uStack_520 = uStack_110;
                          uStack_51c = uStack_10c;
                          uStack_518 = uStack_108;
                          uStack_514 = uStack_104;
                          uStack_510 = uStack_100;
                          uStack_50c = uStack_fc;
                          uStack_508 = uStack_f8;
                          uStack_504 = uStack_f4;
                          uStack_500 = uStack_f0;
                          uStack_4fc = uStack_ec;
                          uStack_4f8 = uStack_e8;
                          uStack_4f4 = uStack_e4;
                          uStack_4f0 = uStack_e0;
                          uStack_4ec = uStack_dc;
                          uStack_5e0 = uVar21;
                          uStack_5dc = uVar22;
                          uStack_538 = CONCAT13(uStack_125,CONCAT21(uStack_127,uStack_128)) &
                                       0x9fffff7f | 0x20000000;
                          FUN_140906a90(&uStack_5e0,6);
                          plStack_b0 = plStack_70 + 2;
                          plStack_70 = plStack_b0;
                        } while (plStack_b0 != plStack_78);
                      }
                      if (lStack_88 != 0) {
                        func_0x000140613c20(plStack_80,lStack_88 << 4,0x10);
                      }
                      func_0x000142923c00(&uStack_5e0,&uStack_9c8,1000);
                      FUN_140868370(&uStack_5e0);
                      if (lStack_98 == 0) {
                        return;
                      }
                      plVar13 = (longlong *)func_0x000140c18130();
                      if (*plVar13 == 1) {
                        plVar13 = plVar13 + 1;
                      }
                      else if ((*plVar13 != 0) ||
                              (plVar13 = (longlong *)FUN_1428f87d0(plVar13,0),
                              plVar13 == (longlong *)0x0)) goto code_r0x0001409db654;
                      if (*plVar13 == 0) {
                        *plVar13 = -1;
                        plStack_70 = plVar13;
                        FUN_140c170f0(plVar13 + 1,uStack_90);
                        *plStack_70 = *plStack_70 + 1;
                        return;
                      }
                      goto code_r0x0001409db660;
                    }
                    goto code_r0x0001409db66c;
                  }
                }
                FUN_1428d9310(&UNK_142ac2b98);
                goto code_r0x0001409db707;
              }
            }
            FUN_1428d9310(&UNK_142ac2b80);
            goto code_r0x0001409db707;
          }
        }
        FUN_1428d9310(&UNK_142ac2b68);
        goto code_r0x0001409db707;
      }
    }
    FUN_1428d9310(&UNK_142ac2b20);
  }
  else {
    puVar18 = &UNK_142ac1f20;
    lVar16 = 0x5a;
    uVar15 = 0x5a;
    lVar9 = FUN_1409d42d0(&UNK_142ac1f20);
    if (lVar9 == 1) {
      if (uVar15 != 0) {
        if (uVar15 < 0x5a) {
          if ((char)(&UNK_142ac1f20)[uVar15] < -0x40) {
code_r0x0001409dab1f:
            FUN_1428d9940(&UNK_142ac1f20,0x5a,0,uVar15,&UNK_142ac2000);
            goto code_r0x0001409dab3f;
          }
        }
        else if (uVar15 != 0x5a) goto code_r0x0001409dab1f;
      }
      puVar18 = &UNK_142ac1f20;
      lVar9 = FUN_1409d42d0(&UNK_142ac1f20);
      if (lVar9 == 1) {
        uVar17 = uVar15 + 2;
        if (uVar17 != 0) {
          uVar20 = uVar17 == 0x5a;
          if (0x59 < uVar17) goto code_r0x0001409daba0;
          uVar20 = (&UNK_142ac1f22)[uVar15] == -0x41;
          if ((char)(&UNK_142ac1f22)[uVar15] < -0x40) {
            do {
              uVar15 = 0x5a;
              FUN_1428d9940(&UNK_142ac1f20,0x5a);
code_r0x0001409daba0:
            } while (!(bool)uVar20);
          }
        }
        lVar16 = 0x58 - uVar15;
        puVar18 = &UNK_142ac1f20 + uVar17;
      }
    }
    puVar10 = &UNK_142ac2b1c;
    puVar19 = &UNK_142ac2ad0;
    do {
      bVar1 = puVar10[-1];
      uVar14 = (uint)(char)bVar1;
      if ((char)bVar1 < '\0') {
        bVar2 = puVar10[-2];
        if ((char)bVar2 < -0x40) {
          bVar3 = puVar10[-3];
          if ((char)bVar3 < -0x40) {
            pbVar6 = puVar10 + -4;
            puVar10 = puVar10 + -4;
            uVar14 = bVar3 & 0x3f | (*pbVar6 & 7) << 6;
          }
          else {
            puVar10 = puVar10 + -3;
            uVar14 = bVar3 & 0xf;
          }
          uVar14 = bVar2 & 0x3f | uVar14 << 6;
        }
        else {
          puVar10 = puVar10 + -2;
          uVar14 = bVar2 & 0x1f;
        }
        uVar14 = (uint)(bVar1 & 0x3f) | uVar14 << 6;
        if (uVar14 != 0x5c) goto code_r0x0001409dac49;
code_r0x0001409dac5e:
        puVar11 = puVar10 + -0x142ac2ad0;
        puVar19 = puVar10 + -0x142ac2acf;
        if (puVar19 != (undefined *)0x0) {
          uVar20 = puVar19 == (undefined *)0x4c;
          if ((undefined *)0x4b < puVar19) goto code_r0x0001409daca0;
          uVar20 = puVar10[1] == -0x41;
          if ((char)puVar10[1] < -0x40) {
            do {
              puVar11 = (undefined *)FUN_1428d9940(&UNK_142ac2ad0,0x4c);
code_r0x0001409daca0:
            } while (!(bool)uVar20);
          }
        }
        lVar9 = 0x4b - (longlong)puVar11;
        puVar19 = &UNK_142ac2ad0 + (longlong)puVar19;
        goto code_r0x0001409dacad;
      }
      puVar10 = puVar10 + -1;
      if (uVar14 == 0x5c) goto code_r0x0001409dac5e;
code_r0x0001409dac49:
      if (uVar14 == 0x2f) goto code_r0x0001409dac5e;
    } while (puVar10 != &UNK_142ac2ad0);
    lVar9 = 0x4c;
code_r0x0001409dacad:
    plVar13 = (longlong *)func_0x000140c18130();
    if (*plVar13 == 1) {
      plVar13 = plVar13 + 1;
code_r0x0001409dacc6:
      if (*plVar13 == 0) {
        *plVar13 = -1;
        plStack_78 = plVar13;
        uStack_90 = FUN_140c16d40(plVar13 + 1,puVar18,lVar16,puVar19,lVar9,1,0);
        *plStack_78 = *plStack_78 + 1;
        lStack_98 = 1;
        goto code_r0x0001409dad1d;
      }
    }
    else {
      if ((*plVar13 == 0) &&
         (plVar13 = (longlong *)FUN_1428f87d0(plVar13,0), plVar13 != (longlong *)0x0))
      goto code_r0x0001409dacc6;
code_r0x0001409db654:
      FUN_1428d80d0(&UNK_142c3aaa0);
    }
code_r0x0001409db660:
    FUN_1428d9250(&UNK_142c3ab70);
code_r0x0001409db66c:
    FUN_1428cf200();
  }
code_r0x0001409db707:
                    /* WARNING: Does not return */
  pcVar7 = (code *)invalidInstructionException();
  (*pcVar7)();
}

