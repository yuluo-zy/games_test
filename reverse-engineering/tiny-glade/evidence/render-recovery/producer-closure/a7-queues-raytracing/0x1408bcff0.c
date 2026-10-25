
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void FUN_1408bcff0(longlong ******param_1,longlong ******param_2,longlong ******param_3)

{
  byte *pbVar1;
  byte bVar2;
  byte bVar3;
  longlong *****ppppplVar4;
  longlong ******pppppplVar5;
  code *pcVar6;
  undefined8 uVar7;
  char cVar8;
  byte bVar9;
  int iVar10;
  longlong lVar11;
  undefined *puVar12;
  undefined *puVar13;
  undefined8 *puVar14;
  longlong *****ppppplVar15;
  longlong *plVar16;
  longlong ****pppplVar17;
  uint uVar18;
  ulonglong uVar19;
  ulonglong uVar20;
  undefined *puVar21;
  longlong lVar22;
  ulonglong *puVar23;
  undefined1 uVar24;
  bool bVar25;
  longlong *****ppppplStack_2d8;
  longlong ***ppplStack_2d0;
  code *pcStack_2c8;
  longlong ***ppplStack_2c0;
  code *pcStack_2b8;
  undefined8 uStack_2b0;
  undefined *puStack_2a8;
  undefined8 uStack_2a0;
  undefined8 uStack_298;
  longlong *****ppppplStack_288;
  longlong ***ppplStack_280;
  code *pcStack_278;
  longlong ***ppplStack_270;
  code *pcStack_268;
  undefined8 uStack_260;
  undefined *puStack_258;
  undefined8 uStack_250;
  undefined8 uStack_248;
  longlong *****ppppplStack_238;
  longlong ***ppplStack_230;
  code *pcStack_228;
  longlong ***ppplStack_220;
  code *pcStack_218;
  undefined8 uStack_210;
  undefined *puStack_208;
  undefined8 uStack_200;
  undefined8 uStack_1f8;
  undefined4 uStack_1e8;
  undefined4 uStack_1e4;
  undefined4 uStack_1e0;
  undefined4 uStack_1dc;
  undefined4 uStack_1d8;
  undefined4 uStack_1d4;
  undefined4 uStack_1d0;
  undefined4 uStack_1cc;
  undefined4 uStack_1c8;
  undefined4 uStack_1c4;
  undefined4 uStack_1c0;
  undefined4 uStack_1bc;
  undefined4 uStack_1b8;
  undefined4 uStack_1b4;
  undefined4 uStack_1b0;
  undefined4 uStack_1ac;
  longlong ***ppplStack_1a8;
  longlong ***ppplStack_140;
  longlong ***ppplStack_138;
  longlong ****pppplStack_130;
  longlong lStack_128;
  undefined8 uStack_120;
  undefined8 uStack_118;
  longlong *****ppppplStack_110;
  longlong *****ppppplStack_108;
  longlong *****ppppplStack_100;
  longlong ***ppplStack_f8;
  code *pcStack_f0;
  longlong ***ppplStack_e8;
  code *pcStack_e0;
  undefined8 uStack_d8;
  undefined *puStack_d0;
  undefined8 uStack_c8;
  undefined8 uStack_c0;
  undefined *puStack_b8;
  undefined8 uStack_b0;
  longlong *****ppppplStack_a8;
  undefined8 uStack_a0;
  undefined8 uStack_98;
  longlong *****ppppplStack_88;
  longlong ***ppplStack_80;
  longlong *****ppppplStack_78;
  undefined8 uStack_70;
  undefined8 uStack_68;
  longlong ***ppplStack_58;
  undefined1 uStack_49;
  undefined8 uStack_48;
  
  uStack_48 = 0xfffffffffffffffe;
  cVar8 = func_0x000140c16a00();
  if (cVar8 == '\0') {
code_r0x0001408bd088:
    lStack_128 = 0;
  }
  else {
    puVar23 = (ulonglong *)&UNK_142aa05ca;
    lVar22 = 0x2c;
    uVar19 = 0x2c;
    lVar11 = FUN_1408b9b00(&UNK_142aa05ca);
    if (lVar11 == 1) {
      if (uVar19 != 0) {
        if (uVar19 < 0x2c) {
          if ((char)(&UNK_142aa05ca)[uVar19] < -0x40) {
code_r0x0001408bd068:
            FUN_1428d9940(&UNK_142aa05ca,0x2c,0,uVar19,&UNK_142aa06f8);
            goto code_r0x0001408bd088;
          }
        }
        else if (uVar19 != 0x2c) goto code_r0x0001408bd068;
      }
      puVar23 = (ulonglong *)&UNK_142aa05ca;
      lVar11 = FUN_1408b9b00(&UNK_142aa05ca);
      if (lVar11 == 1) {
        uVar20 = uVar19 + 2;
        if (uVar20 != 0) {
          uVar24 = uVar20 == 0x2c;
          if (0x2b < uVar20) goto code_r0x0001408bd0e9;
          uVar24 = (&UNK_142aa05cc)[uVar19] == -0x41;
          if ((char)(&UNK_142aa05cc)[uVar19] < -0x40) {
            do {
              uVar19 = 0x2c;
              FUN_1428d9940(&UNK_142aa05ca,0x2c);
code_r0x0001408bd0e9:
            } while (!(bool)uVar24);
          }
        }
        lVar22 = 0x2a - uVar19;
        puVar23 = (ulonglong *)(&UNK_142aa05ca + uVar20);
      }
    }
    puVar12 = &UNK_142aa09c0;
    puVar21 = &UNK_142aa0995;
    do {
      bVar9 = puVar12[-1];
      uVar18 = (uint)(char)bVar9;
      if ((char)bVar9 < '\0') {
        bVar2 = puVar12[-2];
        if ((char)bVar2 < -0x40) {
          bVar3 = puVar12[-3];
          if ((char)bVar3 < -0x40) {
            pbVar1 = puVar12 + -4;
            puVar12 = puVar12 + -4;
            uVar18 = bVar3 & 0x3f | (*pbVar1 & 7) << 6;
          }
          else {
            puVar12 = puVar12 + -3;
            uVar18 = bVar3 & 0xf;
          }
          uVar18 = bVar2 & 0x3f | uVar18 << 6;
        }
        else {
          puVar12 = puVar12 + -2;
          uVar18 = bVar2 & 0x1f;
        }
        uVar18 = (uint)(bVar9 & 0x3f) | uVar18 << 6;
        if (uVar18 != 0x5c) goto code_r0x0001408bd189;
code_r0x0001408bd19e:
        puVar13 = puVar12 + -0x142aa0995;
        puVar21 = puVar12 + -0x142aa0994;
        if (puVar21 != (undefined *)0x0) {
          uVar24 = puVar21 == (undefined *)0x2b;
          if ((undefined *)0x2a < puVar21) goto code_r0x0001408bd1e0;
          uVar24 = puVar12[1] == -0x41;
          if ((char)puVar12[1] < -0x40) {
            do {
              puVar13 = (undefined *)FUN_1428d9940(&UNK_142aa0995,0x2b);
code_r0x0001408bd1e0:
            } while (!(bool)uVar24);
          }
        }
        ppppplVar15 = (longlong *****)(0x2a - (longlong)puVar13);
        puVar21 = &UNK_142aa0995 + (longlong)puVar21;
        goto code_r0x0001408bd1ed;
      }
      puVar12 = puVar12 + -1;
      if (uVar18 == 0x5c) goto code_r0x0001408bd19e;
code_r0x0001408bd189:
      if (uVar18 == 0x2f) goto code_r0x0001408bd19e;
    } while (puVar12 != &UNK_142aa0995);
    ppppplVar15 = (longlong *****)0x2b;
code_r0x0001408bd1ed:
    plVar16 = (longlong *)func_0x000140c18130();
    if (*plVar16 == 1) {
      pppplVar17 = (longlong ****)(plVar16 + 1);
    }
    else {
      if (*plVar16 != 0) goto code_r0x0001408bdb08;
      pppplVar17 = (longlong ****)FUN_1428f87d0(plVar16,0);
      if (pppplVar17 == (longlong ****)0x0) goto code_r0x0001408bdb08;
    }
    if (*pppplVar17 != (longlong ***)0x0) goto code_r0x0001408bdb28;
    *pppplVar17 = (longlong ***)0xffffffffffffffff;
    ppplStack_58 = (longlong ***)pppplVar17;
    uStack_120 = FUN_140c16d40(pppplVar17 + 1,puVar23,lVar22,puVar21,ppppplVar15,1,0);
    *ppplStack_58 = (longlong **)((longlong)*ppplStack_58 + 1);
    lStack_128 = 1;
  }
  ppppplVar15 = (longlong *****)**param_3;
  param_3 = (longlong ******)(ppppplVar15 + 2);
  LOCK();
  bVar25 = *(int *)(ppppplVar15 + 2) == 0;
  if (bVar25) {
    *(int *)(ppppplVar15 + 2) = 0x3fffffff;
  }
  UNLOCK();
  if (!bVar25) goto code_r0x0001408bdb34;
  do {
    puVar23 = puRam000000014362c420;
    if ((*puRam000000014362c420 & 0x7fffffffffffffff) == 0) {
      bVar9 = 0;
      cVar8 = *(char *)(ppppplVar15 + 3);
    }
    else {
      bVar9 = FUN_1428d86b0();
      bVar9 = bVar9 ^ 1;
      cVar8 = *(char *)(ppppplVar15 + 3);
    }
    if (cVar8 != '\0') {
      uVar19 = (ulonglong)ppppplStack_100 >> 8;
      ppppplStack_100 = (longlong *****)CONCAT71((int7)uVar19,bVar9);
      ppppplStack_108 = (longlong *****)param_3;
      FUN_1428d9760(&UNK_142aa03b8,0x2b,&ppppplStack_108,&UNK_142aa0488,&UNK_142aa0a58);
code_r0x0001408bdf86:
                    /* WARNING: Does not return */
      pcVar6 = (code *)invalidInstructionException();
      (*pcVar6)();
    }
    ppplStack_1a8 = (longlong ***)ppppplVar15[0xc];
    uStack_1e8 = *(undefined4 *)(ppppplVar15 + 4);
    uStack_1e4 = *(undefined4 *)((longlong)ppppplVar15 + 0x24);
    uStack_1e0 = *(undefined4 *)(ppppplVar15 + 5);
    uStack_1dc = *(undefined4 *)((longlong)ppppplVar15 + 0x2c);
    uStack_1d8 = *(undefined4 *)(ppppplVar15 + 6);
    uStack_1d4 = *(undefined4 *)((longlong)ppppplVar15 + 0x34);
    uStack_1d0 = *(undefined4 *)(ppppplVar15 + 7);
    uStack_1cc = *(undefined4 *)((longlong)ppppplVar15 + 0x3c);
    uStack_1c8 = *(undefined4 *)(ppppplVar15 + 8);
    uStack_1c4 = *(undefined4 *)((longlong)ppppplVar15 + 0x44);
    uStack_1c0 = *(undefined4 *)(ppppplVar15 + 9);
    uStack_1bc = *(undefined4 *)((longlong)ppppplVar15 + 0x4c);
    uStack_1b8 = *(undefined4 *)(ppppplVar15 + 10);
    uStack_1b4 = *(undefined4 *)((longlong)ppppplVar15 + 0x54);
    uStack_1b0 = *(undefined4 *)(ppppplVar15 + 0xb);
    uStack_1ac = *(undefined4 *)((longlong)ppppplVar15 + 0x5c);
    ppppplVar15[4] = (longlong ****)0x8000000000000000;
    if ((bVar9 == 0) && ((*puVar23 & 0x7fffffffffffffff) != 0)) {
      uStack_49 = 1;
      cVar8 = FUN_1428d86b0();
      if (cVar8 == '\0') {
        *(undefined1 *)(ppppplVar15 + 3) = 1;
      }
    }
    LOCK();
    iVar10 = *(int *)param_3;
    *(int *)param_3 = *(int *)param_3 + -0x3fffffff;
    UNLOCK();
    if (0x3fffffff < iVar10 + 0xc0000001U) goto code_r0x0001408bdb14;
code_r0x0001408bd317:
    if (!SBORROW8(0,CONCAT44(uStack_1e4,uStack_1e8))) {
      ppppplVar15 = (longlong *****)CONCAT44(uStack_1bc,uStack_1c0);
      ppplStack_58 = (longlong ***)CONCAT44(uStack_1d4,uStack_1d8);
      if (((ulonglong)ppplStack_1a8 | (ulonglong)ppppplVar15) < 0x40000 &&
          (longlong ****)CONCAT44(uStack_1d4,uStack_1d8) < (longlong ****)0x10000) {
        ppppplStack_110 = (longlong *****)CONCAT44(uStack_1ac,uStack_1b0);
        uStack_118 = CONCAT44(uStack_1dc,uStack_1e0);
        ppppplVar4 = *param_2;
        puVar14 = (undefined8 *)FUN_140969720(ppppplVar4,&UNK_142c2e3d8);
        if (puVar14 != (undefined8 *)0x0) {
          ppppplStack_88 = (longlong *****)&UNK_142a9dc30;
          ppplStack_80 = (longlong ***)0x2c;
          if ((puVar14[3] != 0x2c) ||
             (iVar10 = func_0x000142923bf0(&UNK_142a9dc30,puVar14[2],0x2c), iVar10 != 0)) {
            ppppplStack_108 = (longlong *****)0x0;
            FUN_1428cab41(0,&ppppplStack_88,puVar14 + 2,&ppppplStack_108,&UNK_142c2e180);
            goto code_r0x0001408bdf86;
          }
          puVar14 = (undefined8 *)(**(code **)(puVar14[1] + 0x18))(*puVar14);
          if (puVar14 != (undefined8 *)0x0) {
            pppppplVar5 = (longlong ******)*param_1;
            if (ppppplVar15 == (longlong *****)0x0) {
code_r0x0001408bd4cb:
              *puVar14 = ppppplVar15;
              puVar14 = (undefined8 *)FUN_140969720(ppppplVar4,&UNK_142c2e270);
              if (puVar14 != (undefined8 *)0x0) {
                ppppplStack_88 = (longlong *****)&UNK_142a9cf75;
                ppplStack_80 = (longlong ***)0x2f;
                puVar23 = puVar14 + 2;
                if ((puVar14[3] != 0x2f) ||
                   (iVar10 = func_0x000142923bf0(&UNK_142a9cf75,*puVar23,0x2f), iVar10 != 0)) {
                  ppppplStack_108 = (longlong *****)0x0;
                  FUN_1428cab41(0,&ppppplStack_88,puVar23,&ppppplStack_108,&UNK_142c2e180);
                  goto code_r0x0001408bdf86;
                }
                puVar14 = (undefined8 *)(**(code **)(puVar14[1] + 0x18))(*puVar14);
                if (puVar14 != (undefined8 *)0x0) {
                  if ((longlong ****)ppplStack_1a8 == (longlong ****)0x0) {
code_r0x0001408bd5c7:
                    *puVar14 = ppplStack_1a8;
                    puVar14 = (undefined8 *)FUN_140969720(ppppplVar4,&UNK_142c2e350);
                    if (puVar14 != (undefined8 *)0x0) {
                      ppppplStack_88 = (longlong *****)&UNK_142a9e726;
                      ppplStack_80 = (longlong ***)0x26;
                      param_2 = (longlong ******)(puVar14 + 2);
                      if ((puVar14[3] != 0x26) ||
                         (iVar10 = func_0x000142923bf0(&UNK_142a9e726,*param_2,0x26), iVar10 != 0))
                      {
                        ppppplStack_108 = (longlong *****)0x0;
                        FUN_1428cab41(0,&ppppplStack_88,param_2,&ppppplStack_108,&UNK_142c2e180);
                        goto code_r0x0001408bdf86;
                      }
                      ppppplVar15 = (longlong *****)(**(code **)(puVar14[1] + 0x18))(*puVar14);
                      if (ppppplVar15 != (longlong *****)0x0) {
                        if ((longlong ****)ppplStack_58 == (longlong ****)0x0) {
code_r0x0001408bd6d4:
                          *ppppplVar15 = (longlong ****)ppplStack_58;
                          param_3 = param_1;
                          param_1 = pppppplVar5;
                          goto code_r0x0001408bda07;
                        }
                        pppplVar17 = (longlong ****)((longlong)ppplStack_58 * 0x50);
                        if (ppppplVar15[4] < pppplVar17) {
                          ppppplStack_108 = (longlong *****)0xb;
                        }
                        else {
                          ppppplStack_88 = (longlong *****)0x0;
                          ppplStack_80 = (longlong ***)pppplVar17;
                          FUN_140beefa0(&ppppplStack_108,*pppppplVar5 + 2,ppppplVar15 + 1,uStack_118
                                        ,pppplVar17,&ppppplStack_88,1,0);
                          if ((longlong ******)ppppplStack_108 == (longlong ******)0xb)
                          goto code_r0x0001408bd6d4;
                          uStack_298 = uStack_c0;
                          puStack_2a8 = puStack_d0;
                          uStack_2a0 = uStack_c8;
                          pcStack_2b8 = pcStack_e0;
                          uStack_2b0 = uStack_d8;
                          pcStack_2c8 = pcStack_f0;
                          ppplStack_2c0 = ppplStack_e8;
                          ppppplStack_2d8 = ppppplStack_100;
                          ppplStack_2d0 = ppplStack_f8;
                        }
                        ppppplStack_100 = ppppplStack_2d8;
                        ppplStack_f8 = ppplStack_2d0;
                        pcStack_f0 = pcStack_2c8;
                        ppplStack_e8 = ppplStack_2c0;
                        pcStack_e0 = pcStack_2b8;
                        uStack_d8 = uStack_2b0;
                        puStack_d0 = puStack_2a8;
                        uStack_c8 = uStack_2a0;
                        uStack_c0 = uStack_298;
                        FUN_1428d9760(&UNK_142aa03b8,0x2b,&ppppplStack_108,&UNK_142aa0408,
                                      &UNK_142aa0be8);
                        goto code_r0x0001408bdf86;
                      }
                    }
                    FUN_1428d9310(&UNK_142aa0bd0);
                    goto code_r0x0001408bdf86;
                  }
                  pppplVar17 = (longlong ****)((longlong)ppplStack_1a8 * 4);
                  if (pppplVar17 < (longlong ****)puVar14[4] ||
                      (longlong)pppplVar17 - (longlong)puVar14[4] == 0) {
                    ppppplStack_88 = (longlong *****)0x0;
                    ppplStack_80 = (longlong ***)pppplVar17;
                    FUN_140beefa0(&ppppplStack_108,*pppppplVar5 + 2,puVar14 + 1,ppppplStack_110,
                                  pppplVar17,&ppppplStack_88,1,0);
                    if ((longlong ******)ppppplStack_108 == (longlong ******)0xb)
                    goto code_r0x0001408bd5c7;
                    uStack_248 = uStack_c0;
                    puStack_258 = puStack_d0;
                    uStack_250 = uStack_c8;
                    pcStack_268 = pcStack_e0;
                    uStack_260 = uStack_d8;
                    pcStack_278 = pcStack_f0;
                    ppplStack_270 = ppplStack_e8;
                    ppppplStack_288 = ppppplStack_100;
                    ppplStack_280 = ppplStack_f8;
                  }
                  else {
                    ppppplStack_108 = (longlong *****)0xb;
                  }
                  ppppplStack_100 = ppppplStack_288;
                  ppplStack_f8 = ppplStack_280;
                  pcStack_f0 = pcStack_278;
                  ppplStack_e8 = ppplStack_270;
                  pcStack_e0 = pcStack_268;
                  uStack_d8 = uStack_260;
                  puStack_d0 = puStack_258;
                  uStack_c8 = uStack_250;
                  uStack_c0 = uStack_248;
                  FUN_1428d9760(&UNK_142aa03b8,0x2b,&ppppplStack_108,&UNK_142aa0408,&UNK_142aa0bb8);
                  goto code_r0x0001408bdf86;
                }
              }
              FUN_1428d9310(&UNK_142aa0ba0);
              goto code_r0x0001408bdf86;
            }
            pppplVar17 = (longlong ****)((longlong)ppppplVar15 * 0x18);
            if ((longlong ****)puVar14[4] < pppplVar17) {
              ppppplStack_108 = (longlong *****)0xb;
            }
            else {
              ppppplStack_88 = (longlong *****)0x0;
              ppplStack_80 = (longlong ***)pppplVar17;
              FUN_140beefa0(&ppppplStack_108,*pppppplVar5 + 2,puVar14 + 1,
                            CONCAT44(uStack_1c4,uStack_1c8),pppplVar17,&ppppplStack_88,1,0);
              if ((longlong ******)ppppplStack_108 == (longlong ******)0xb)
              goto code_r0x0001408bd4cb;
              uStack_1f8 = uStack_c0;
              puStack_208 = puStack_d0;
              uStack_200 = uStack_c8;
              pcStack_218 = pcStack_e0;
              uStack_210 = uStack_d8;
              pcStack_228 = pcStack_f0;
              ppplStack_220 = ppplStack_e8;
              ppppplStack_238 = ppppplStack_100;
              ppplStack_230 = ppplStack_f8;
            }
            ppppplStack_100 = ppppplStack_238;
            ppplStack_f8 = ppplStack_230;
            pcStack_f0 = pcStack_228;
            ppplStack_e8 = ppplStack_220;
            pcStack_e0 = pcStack_218;
            uStack_d8 = uStack_210;
            puStack_d0 = puStack_208;
            uStack_c8 = uStack_200;
            uStack_c0 = uStack_1f8;
            FUN_1428d9760(&UNK_142aa03b8,0x2b,&ppppplStack_108,&UNK_142aa0408,&UNK_142aa0b88);
            goto code_r0x0001408bdf86;
          }
        }
        FUN_1428d9310(&UNK_142aa0b70);
        goto code_r0x0001408bdf86;
      }
      if (iRam0000000143634ca8 != 3) {
        ppppplStack_88 = (longlong *****)CONCAT71(ppppplStack_88._1_7_,1);
        ppppplStack_108 = (longlong *****)&ppppplStack_88;
        FUN_1428d8820(0x143634ca8,0,&ppppplStack_108,&UNK_142aa01a0,&UNK_142aa0a70);
      }
      FUN_140542890(&ppppplStack_108,uRam0000000143634ca0);
      if ((int)ppppplStack_108 == 1) {
        ppppplStack_88 = ppppplStack_100;
        ppplStack_80 = (longlong ***)CONCAT71(ppplStack_80._1_7_,ppplStack_f8._0_1_);
        FUN_1428d9760(&UNK_142aa0a88,0x13,&ppppplStack_88,&UNK_142aa0358,&UNK_142aa0a70);
        goto code_r0x0001408bdf86;
      }
      ppppplStack_110 = ppppplStack_100;
      uStack_118 = CONCAT71(uStack_118._1_7_,ppplStack_f8._0_1_);
      puVar14 = (undefined8 *)func_0x000140613c10(0xf,1);
      if (puVar14 == (undefined8 *)0x0) {
        FUN_1428d8fe3(1,0xf,&UNK_142aa0520);
        goto code_r0x0001408bdf86;
      }
      *(undefined8 *)((longlong)puVar14 + 7) = 0x2928202168746170;
      *puVar14 = 0x705f656c75646f6d;
      ppppplStack_108 = (longlong *****)0xf;
      ppplStack_f8 = (longlong ***)0xf;
      ppppplStack_100 = (longlong *****)puVar14;
      FUN_1428a9a60(&ppppplStack_108,0xf,0x18,1,1);
      uVar7 = _UNK_142aa0aa3;
      *(undefined8 *)((longlong)ppppplStack_100 + (longlong)ppplStack_f8) = _UNK_142aa0a9b;
      ((undefined8 *)((longlong)ppppplStack_100 + (longlong)ppplStack_f8))[1] = uVar7;
      *(undefined8 *)((longlong)ppppplStack_100 + (longlong)ppplStack_f8 + 0x10) =
           0x726f727245203a3a;
      ppppplStack_78 = (longlong *****)((longlong)ppplStack_f8 + 0x18);
      ppppplStack_88 = ppppplStack_108;
      ppplStack_80 = (longlong ***)ppppplStack_100;
      if ((ulonglong)((longlong)ppppplStack_108 - (longlong)ppppplStack_78) < 0x36) {
        FUN_1428a9a60(&ppppplStack_88,ppppplStack_78,0x36,1,1);
      }
      uVar7 = _UNK_142aa0adb;
      puVar14 = (undefined8 *)((longlong)ppplStack_80 + (longlong)ppppplStack_78 + 0x20);
      *puVar14 = _UNK_142aa0ad3;
      puVar14[1] = uVar7;
      uVar7 = _UNK_142aa0acb;
      puVar14 = (undefined8 *)((longlong)ppplStack_80 + (longlong)ppppplStack_78 + 0x10);
      *puVar14 = _UNK_142aa0ac3;
      puVar14[1] = uVar7;
      uVar7 = _UNK_142aa0abb;
      *(undefined8 *)((longlong)ppplStack_80 + (longlong)ppppplStack_78) = _UNK_142aa0ab3;
      ((undefined8 *)((longlong)ppplStack_80 + (longlong)ppppplStack_78))[1] = uVar7;
      *(undefined8 *)((longlong)ppplStack_80 + (longlong)ppppplStack_78 + 0x2e) = 0x617461642d676e69
      ;
      ppplStack_f8 = (longlong ***)((longlong)ppppplStack_78 + 0x36);
      ppppplStack_108 = ppppplStack_88;
      ppppplStack_100 = (longlong *****)ppplStack_80;
      param_2 = &ppppplStack_108;
      ppppplStack_78 = (longlong *****)ppplStack_f8;
      cVar8 = FUN_14143b8b0(ppppplStack_110 + 1,param_2);
      if ((cVar8 == '\0') && (*plRam000000014362c3a8 != 0)) {
        plVar16 = (longlong *)func_0x00014067dd80(&UNK_142aa0a70);
        ppplStack_e8 = (longlong ***)*plVar16;
        pcStack_e0 = (code *)plVar16[1];
        uStack_d8 = 1;
        puStack_d0 = &UNK_142aa0a20;
        uStack_c8 = 0x1b;
        puStack_b8 = &UNK_142aa0af0;
        uStack_b0 = 1;
        ppppplStack_a8 = (longlong *****)&ppppplStack_88;
        ppppplStack_108 = (longlong *****)0x0;
        uStack_a0 = 0;
        uStack_98 = 0;
        ppppplStack_100 = (longlong *****)&UNK_142aa0a20;
        ppplStack_f8 = (longlong ***)0x1b;
        pcStack_f0 = (code *)0x0;
        uStack_c0 = CONCAT44((int)plVar16[2],1);
        func_0x000140d4a330(ppppplStack_a8,&ppppplStack_108);
      }
      param_3 = (longlong ******)ppppplStack_110;
      if ((((char)uStack_118 == '\0') && ((*puVar23 & 0x7fffffffffffffff) != 0)) &&
         (cVar8 = FUN_1428d86b0(), cVar8 == '\0')) {
        *(char *)((longlong)param_3 + 1) = '\x01';
      }
      LOCK();
      cVar8 = *(char *)param_3;
      *(char *)param_3 = '\0';
      UNLOCK();
      if (cVar8 == '\x02') {
        FUN_1428d8810(param_3);
      }
      ppplStack_138 = ppplStack_1a8;
      ppplStack_140 = ppplStack_58;
      ppppplStack_108 = &pppplStack_130;
      ppppplStack_100 = (longlong *****)FUN_140d8d6c0;
      ppplStack_f8 = (longlong ***)&ppplStack_138;
      pcStack_f0 = FUN_140d8d6c0;
      ppplStack_e8 = (longlong ***)&ppplStack_140;
      pcStack_e0 = FUN_140d8d6c0;
      ppppplStack_88 = (longlong *****)&UNK_142aa0b30;
      ppplStack_80 = (longlong ***)0x4;
      uStack_68 = 0;
      uStack_70 = 3;
      pppplStack_130 = (longlong ****)ppppplVar15;
      ppppplStack_78 = (longlong *****)param_2;
      FUN_140d5caa0(&ppppplStack_88);
code_r0x0001408bda07:
      if (CONCAT44(uStack_1e4,uStack_1e8) != 0) {
        func_0x000140613c20(CONCAT44(uStack_1dc,uStack_1e0),CONCAT44(uStack_1e4,uStack_1e8) * 0x50,4
                           );
      }
      if (CONCAT44(uStack_1cc,uStack_1d0) != 0) {
        func_0x000140613c20(CONCAT44(uStack_1c4,uStack_1c8),CONCAT44(uStack_1cc,uStack_1d0) * 0x18,4
                           );
      }
      if (CONCAT44(uStack_1b4,uStack_1b8) != 0) {
        func_0x000140613c20(CONCAT44(uStack_1ac,uStack_1b0),CONCAT44(uStack_1b4,uStack_1b8) << 2,4);
      }
    }
    if (lStack_128 == 0) {
      return;
    }
    plVar16 = (longlong *)func_0x000140c18130();
    if (*plVar16 != 1) {
      if ((*plVar16 == 0) && (plVar16 = (longlong *)FUN_1428f87d0(), plVar16 != (longlong *)0x0))
      goto code_r0x0001408bda94;
code_r0x0001408bdb08:
      FUN_1428d80d0(&UNK_142c3aaa0);
code_r0x0001408bdb14:
      uStack_49 = 1;
      FUN_1428d8d40(param_3);
      goto code_r0x0001408bd317;
    }
    plVar16 = plVar16 + 1;
code_r0x0001408bda94:
    if (*plVar16 == 0) {
      *plVar16 = -1;
      ppplStack_58 = (longlong ***)plVar16;
      FUN_140c170f0(plVar16 + 1,uStack_120);
      *ppplStack_58 = (longlong **)((longlong)*ppplStack_58 + 1);
      return;
    }
code_r0x0001408bdb28:
    FUN_1428d9250(&UNK_142c3ab70);
code_r0x0001408bdb34:
    FUN_1428d8c10(param_3);
  } while( true );
}

