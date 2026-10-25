
void FUN_140bf92f0(longlong param_1,longlong *param_2)

{
  byte *pbVar1;
  byte *pbVar2;
  byte bVar3;
  byte bVar4;
  byte bVar5;
  code *pcVar6;
  char cVar7;
  longlong lVar8;
  undefined *puVar9;
  longlong *plVar10;
  uint uVar11;
  ulonglong uVar12;
  longlong *unaff_RBX;
  undefined *puVar13;
  longlong *plVar14;
  ulonglong uVar15;
  undefined *puVar16;
  undefined *puVar17;
  longlong lVar18;
  undefined1 uVar19;
  bool bVar20;
  longlong lStack_158;
  undefined8 uStack_150;
  undefined8 uStack_138;
  undefined8 uStack_130;
  undefined8 uStack_128;
  undefined8 uStack_120;
  longlong lStack_118;
  undefined4 uStack_110;
  undefined4 uStack_10c;
  longlong lStack_108;
  undefined *puStack_100;
  undefined8 uStack_f8;
  undefined4 uStack_f0;
  undefined4 uStack_ec;
  undefined *puStack_e8;
  undefined8 uStack_e0;
  undefined8 uStack_d8;
  undefined8 uStack_d0;
  undefined8 uStack_c8;
  int iStack_b0;
  undefined4 uStack_ac;
  undefined4 uStack_a8;
  undefined4 uStack_a4;
  undefined4 uStack_a0;
  undefined4 uStack_9c;
  undefined4 uStack_98;
  undefined4 uStack_94;
  longlong *plStack_90;
  longlong *plStack_88;
  longlong *plStack_80;
  longlong lStack_78;
  undefined8 uStack_70;
  undefined1 uStack_61;
  undefined8 uStack_60;
  undefined8 uStack_58;
  char cStack_49;
  longlong *plStack_48;
  undefined8 uStack_40;
  
  uStack_40 = 0xfffffffffffffffe;
  cVar7 = func_0x000140c16a00();
  if (cVar7 == '\0') {
code_r0x000140bf9382:
    lStack_158 = 0;
    lVar8 = param_2[5];
  }
  else {
    puVar16 = &UNK_142b1549e;
    unaff_RBX = (longlong *)0x5d;
    uVar12 = 0x5d;
    lVar8 = FUN_140bed840(&UNK_142b1549e);
    if (lVar8 == 1) {
      if (uVar12 != 0) {
        if (uVar12 < 0x5d) {
          if ((char)(&UNK_142b1549e)[uVar12] < -0x40) {
code_r0x000140bf9362:
            FUN_1428d9940(&UNK_142b1549e,0x5d,0,uVar12,&UNK_142b155f8);
            goto code_r0x000140bf9382;
          }
        }
        else if (uVar12 != 0x5d) goto code_r0x000140bf9362;
      }
      puVar16 = &UNK_142b1549e;
      lVar8 = FUN_140bed840(&UNK_142b1549e);
      if (lVar8 == 1) {
        uVar15 = uVar12 + 2;
        if (uVar15 != 0) {
          uVar19 = uVar15 == 0x5d;
          if (0x5c < uVar15) goto code_r0x000140bf93f2;
          uVar19 = (&UNK_142b154a0)[uVar12] == -0x41;
          if ((char)(&UNK_142b154a0)[uVar12] < -0x40) {
            do {
              uVar12 = 0x5d;
              FUN_1428d9940(&UNK_142b1549e);
code_r0x000140bf93f2:
            } while (!(bool)uVar19);
          }
        }
        unaff_RBX = (longlong *)(0x5b - uVar12);
        puVar16 = &UNK_142b1549e + uVar15;
      }
    }
    puVar13 = &UNK_142b16953;
    puVar17 = &UNK_142b16930;
    do {
      bVar4 = puVar13[-1];
      uVar11 = (uint)(char)bVar4;
      if ((char)bVar4 < '\0') {
        bVar5 = puVar13[-2];
        if ((char)bVar5 < -0x40) {
          bVar3 = puVar13[-3];
          if ((char)bVar3 < -0x40) {
            pbVar1 = puVar13 + -4;
            puVar13 = puVar13 + -4;
            uVar11 = bVar3 & 0x3f | (*pbVar1 & 7) << 6;
          }
          else {
            puVar13 = puVar13 + -3;
            uVar11 = bVar3 & 0xf;
          }
          uVar11 = bVar5 & 0x3f | uVar11 << 6;
        }
        else {
          puVar13 = puVar13 + -2;
          uVar11 = bVar5 & 0x1f;
        }
        uVar11 = (uint)(bVar4 & 0x3f) | uVar11 << 6;
        if (uVar11 != 0x5c) goto code_r0x000140bf9499;
code_r0x000140bf94af:
        puVar9 = puVar13 + -0x142b16930;
        puVar17 = puVar13 + -0x142b1692f;
        if (puVar17 != (undefined *)0x0) {
          uVar19 = puVar17 == (undefined *)0x23;
          if ((undefined *)0x22 < puVar17) goto code_r0x000140bf94f1;
          uVar19 = puVar13[1] == -0x41;
          if ((char)puVar13[1] < -0x40) {
            do {
              puVar9 = (undefined *)FUN_1428d9940(&UNK_142b16930,0x23);
code_r0x000140bf94f1:
            } while (!(bool)uVar19);
          }
        }
        lVar8 = 0x22 - (longlong)puVar9;
        puVar17 = &UNK_142b16930 + (longlong)puVar17;
        goto code_r0x000140bf94ff;
      }
      puVar13 = puVar13 + -1;
      if (uVar11 == 0x5c) goto code_r0x000140bf94af;
code_r0x000140bf9499:
      if (uVar11 == 0x2f) goto code_r0x000140bf94af;
    } while (puVar13 != &UNK_142b16930);
    lVar8 = 0x23;
code_r0x000140bf94ff:
    plVar10 = (longlong *)func_0x000140c18130();
    if (*plVar10 == 1) {
      plVar10 = plVar10 + 1;
    }
    else {
      if (*plVar10 != 0) goto code_r0x000140bfb155;
      plVar10 = (longlong *)FUN_1429090d0(plVar10,0);
      if (plVar10 == (longlong *)0x0) goto code_r0x000140bfb155;
    }
    plVar14 = unaff_RBX;
    if (*plVar10 != 0) goto code_r0x000140bfb161;
    *plVar10 = -1;
    plStack_48 = plVar10;
    uStack_150 = FUN_140c16d40(plVar10 + 1,puVar16,unaff_RBX,puVar17,lVar8,1,0);
    *plStack_48 = *plStack_48 + 1;
    lStack_158 = 1;
    lVar8 = param_2[5];
  }
  if (lVar8 != 0) {
    cVar7 = func_0x000140c16a00();
    lVar8 = 0;
    if (cVar7 == '\0') {
      uStack_138 = 0;
    }
    else {
      puVar16 = &UNK_142b16953;
      puVar13 = &UNK_142b16930;
      do {
        pbVar1 = puVar16 + -1;
        uVar11 = (uint)(char)*pbVar1;
        if ((int)uVar11 < 0) {
          bVar4 = puVar16[-2];
          if ((char)bVar4 < -0x40) {
            bVar5 = puVar16[-3];
            if ((char)bVar5 < -0x40) {
              pbVar2 = puVar16 + -4;
              puVar16 = puVar16 + -4;
              uVar11 = bVar5 & 0x3f | (*pbVar2 & 7) << 6;
            }
            else {
              puVar16 = puVar16 + -3;
              uVar11 = bVar5 & 0xf;
            }
            uVar11 = bVar4 & 0x3f | uVar11 << 6;
          }
          else {
            puVar16 = puVar16 + -2;
            uVar11 = bVar4 & 0x1f;
          }
          uVar11 = (uint)(*pbVar1 & 0x3f) | uVar11 << 6;
          if (uVar11 != 0x5c) goto code_r0x000140bf9619;
code_r0x000140bf9636:
          puVar13 = puVar16 + -0x142b1692f;
          if (puVar13 != (undefined *)0x0) {
            if (puVar13 < (undefined *)0x23) {
              if ((char)puVar16[1] < -0x40) {
code_r0x000140bf9655:
                FUN_1428d9940(&UNK_142b16930,0x23,puVar13,0x23,&UNK_142b155d8);
                goto code_r0x000140bfb454;
              }
            }
            else if (puVar13 != (undefined *)0x23) goto code_r0x000140bf9655;
          }
          lVar18 = 0x22 - (longlong)(puVar16 + -0x142b16930);
          puVar13 = puVar16 + 1;
          goto code_r0x000140bf968b;
        }
        puVar16 = puVar16 + -1;
        if (uVar11 == 0x5c) goto code_r0x000140bf9636;
code_r0x000140bf9619:
        if (uVar11 == 0x2f) goto code_r0x000140bf9636;
      } while (puVar16 != &UNK_142b16930);
      lVar18 = 0x23;
code_r0x000140bf968b:
      plVar10 = (longlong *)func_0x000140c18130();
      if (*plVar10 == 0) {
        plVar10 = (longlong *)FUN_1429090d0(plVar10,0);
        if (plVar10 == (longlong *)0x0) goto code_r0x000140bfb331;
      }
      else {
        if ((int)*plVar10 != 1) {
code_r0x000140bfb331:
          FUN_1428d80d0(&UNK_142d06498);
          goto code_r0x000140bfb454;
        }
        plVar10 = plVar10 + 1;
      }
      if (*plVar10 != 0) {
        FUN_1428d9250(&UNK_142d06508);
        goto code_r0x000140bfb454;
      }
      *plVar10 = -1;
      plStack_48 = plVar10;
      uStack_130 = (longlong *)FUN_140c16d40(plVar10 + 1,&UNK_142b16953,0x13,puVar13,lVar18,1,0);
      *plStack_48 = *plStack_48 + 1;
      uStack_138 = 1;
    }
    unaff_RBX = (longlong *)param_2[4];
    lVar18 = param_2[5];
    param_2[5] = 0;
    if (lVar18 != 0) {
      lVar18 = lVar18 << 3;
      lVar8 = 0;
      do {
        (**(code **)(param_1 + 0x1c0))(*(undefined8 *)(param_1 + 0x640),unaff_RBX[lVar8],0);
        lVar8 = lVar8 + 1;
        lVar18 = lVar18 + -8;
      } while (lVar18 != 0);
    }
    plStack_48 = (longlong *)CONCAT71(plStack_48._1_7_,cVar7);
    uStack_60._0_5_ = CONCAT14(6,(int)lVar8);
    FUN_1418c1380(0x143634e40,&uStack_60,5);
    if ((char)plStack_48 != '\0') {
      plVar10 = (longlong *)func_0x000140c18130();
      if (*plVar10 == 0) {
        plVar10 = (longlong *)FUN_1429090d0(plVar10,0);
        if (plVar10 == (longlong *)0x0) goto code_r0x000140bfb342;
      }
      else {
        if ((int)*plVar10 != 1) {
code_r0x000140bfb342:
          FUN_1428d80d0(&UNK_142d06498);
          goto code_r0x000140bfb454;
        }
        plVar10 = plVar10 + 1;
      }
      if (*plVar10 != 0) {
        FUN_1428d9250(&UNK_142d06508);
        goto code_r0x000140bfb454;
      }
      *plVar10 = -1;
      plStack_48 = plVar10;
      FUN_140c170f0(plVar10 + 1,uStack_130);
      *plStack_48 = *plStack_48 + 1;
    }
  }
  if (param_2[2] != 0) {
    cVar7 = func_0x000140c16a00();
    lVar8 = 0;
    if (cVar7 == '\0') {
      uStack_138 = 0;
    }
    else {
      puVar16 = &UNK_142b16953;
      puVar13 = &UNK_142b16930;
      do {
        pbVar1 = puVar16 + -1;
        uVar11 = (uint)(char)*pbVar1;
        if ((int)uVar11 < 0) {
          bVar4 = puVar16[-2];
          if ((char)bVar4 < -0x40) {
            bVar5 = puVar16[-3];
            if ((char)bVar5 < -0x40) {
              pbVar2 = puVar16 + -4;
              puVar16 = puVar16 + -4;
              uVar11 = bVar5 & 0x3f | (*pbVar2 & 7) << 6;
            }
            else {
              puVar16 = puVar16 + -3;
              uVar11 = bVar5 & 0xf;
            }
            uVar11 = bVar4 & 0x3f | uVar11 << 6;
          }
          else {
            puVar16 = puVar16 + -2;
            uVar11 = bVar4 & 0x1f;
          }
          uVar11 = (uint)(*pbVar1 & 0x3f) | uVar11 << 6;
          if (uVar11 != 0x5c) goto code_r0x000140bf98a9;
code_r0x000140bf98c6:
          puVar13 = puVar16 + -0x142b1692f;
          if (puVar13 != (undefined *)0x0) {
            if (puVar13 < (undefined *)0x23) {
              if ((char)puVar16[1] < -0x40) {
code_r0x000140bf98e5:
                FUN_1428d9940(&UNK_142b16930,0x23,puVar13,0x23,&UNK_142b155d8);
                goto code_r0x000140bfb454;
              }
            }
            else if (puVar13 != (undefined *)0x23) goto code_r0x000140bf98e5;
          }
          lVar18 = 0x22 - (longlong)(puVar16 + -0x142b16930);
          puVar13 = puVar16 + 1;
          goto code_r0x000140bf991b;
        }
        puVar16 = puVar16 + -1;
        if (uVar11 == 0x5c) goto code_r0x000140bf98c6;
code_r0x000140bf98a9:
        if (uVar11 == 0x2f) goto code_r0x000140bf98c6;
      } while (puVar16 != &UNK_142b16930);
      lVar18 = 0x23;
code_r0x000140bf991b:
      plVar10 = (longlong *)func_0x000140c18130();
      if (*plVar10 == 0) {
        plVar10 = (longlong *)FUN_1429090d0(plVar10,0);
        if (plVar10 == (longlong *)0x0) goto code_r0x000140bfb353;
      }
      else {
        if ((int)*plVar10 != 1) {
code_r0x000140bfb353:
          FUN_1428d80d0(&UNK_142d06498);
          goto code_r0x000140bfb454;
        }
        plVar10 = plVar10 + 1;
      }
      if (*plVar10 != 0) {
        FUN_1428d9250(&UNK_142d06508);
        goto code_r0x000140bfb454;
      }
      *plVar10 = -1;
      plStack_48 = plVar10;
      uStack_130 = (longlong *)FUN_140c16d40(plVar10 + 1,&UNK_142b16966,0xe,puVar13,lVar18,1,0);
      *plStack_48 = *plStack_48 + 1;
      uStack_138 = 1;
    }
    unaff_RBX = (longlong *)param_2[1];
    lVar18 = param_2[2];
    param_2[2] = 0;
    if (lVar18 != 0) {
      lVar18 = lVar18 << 3;
      lVar8 = 0;
      do {
        (**(code **)(param_1 + 0x1a8))(*(undefined8 *)(param_1 + 0x640),unaff_RBX[lVar8],0);
        lVar8 = lVar8 + 1;
        lVar18 = lVar18 + -8;
      } while (lVar18 != 0);
    }
    plStack_48 = (longlong *)CONCAT71(plStack_48._1_7_,cVar7);
    uStack_60._0_5_ = CONCAT14(7,(int)lVar8);
    FUN_1418c1380(0x143634e40,&uStack_60,5);
    if ((char)plStack_48 != '\0') {
      plVar10 = (longlong *)func_0x000140c18130();
      if (*plVar10 == 0) {
        plVar10 = (longlong *)FUN_1429090d0(plVar10,0);
        if (plVar10 == (longlong *)0x0) goto code_r0x000140bfb364;
      }
      else {
        if ((int)*plVar10 != 1) {
code_r0x000140bfb364:
          FUN_1428d80d0(&UNK_142d06498);
          goto code_r0x000140bfb454;
        }
        plVar10 = plVar10 + 1;
      }
      if (*plVar10 != 0) {
        FUN_1428d9250(&UNK_142d06508);
        goto code_r0x000140bfb454;
      }
      *plVar10 = -1;
      plStack_48 = plVar10;
      FUN_140c170f0(plVar10 + 1,uStack_130);
      *plStack_48 = *plStack_48 + 1;
    }
  }
  if (param_2[8] != 0) {
    cVar7 = func_0x000140c16a00();
    lVar8 = 0;
    if (cVar7 == '\0') {
      uStack_138 = 0;
    }
    else {
      puVar16 = &UNK_142b16953;
      puVar13 = &UNK_142b16930;
      do {
        pbVar1 = puVar16 + -1;
        uVar11 = (uint)(char)*pbVar1;
        if ((int)uVar11 < 0) {
          bVar4 = puVar16[-2];
          if ((char)bVar4 < -0x40) {
            bVar5 = puVar16[-3];
            if ((char)bVar5 < -0x40) {
              pbVar2 = puVar16 + -4;
              puVar16 = puVar16 + -4;
              uVar11 = bVar5 & 0x3f | (*pbVar2 & 7) << 6;
            }
            else {
              puVar16 = puVar16 + -3;
              uVar11 = bVar5 & 0xf;
            }
            uVar11 = bVar4 & 0x3f | uVar11 << 6;
          }
          else {
            puVar16 = puVar16 + -2;
            uVar11 = bVar4 & 0x1f;
          }
          uVar11 = (uint)(*pbVar1 & 0x3f) | uVar11 << 6;
          if (uVar11 != 0x5c) goto code_r0x000140bf9b39;
code_r0x000140bf9b56:
          puVar13 = puVar16 + -0x142b1692f;
          if (puVar13 != (undefined *)0x0) {
            if (puVar13 < (undefined *)0x23) {
              if ((char)puVar16[1] < -0x40) {
code_r0x000140bf9b75:
                FUN_1428d9940(&UNK_142b16930,0x23,puVar13,0x23,&UNK_142b155d8);
                goto code_r0x000140bfb454;
              }
            }
            else if (puVar13 != (undefined *)0x23) goto code_r0x000140bf9b75;
          }
          lVar18 = 0x22 - (longlong)(puVar16 + -0x142b16930);
          puVar13 = puVar16 + 1;
          goto code_r0x000140bf9bab;
        }
        puVar16 = puVar16 + -1;
        if (uVar11 == 0x5c) goto code_r0x000140bf9b56;
code_r0x000140bf9b39:
        if (uVar11 == 0x2f) goto code_r0x000140bf9b56;
      } while (puVar16 != &UNK_142b16930);
      lVar18 = 0x23;
code_r0x000140bf9bab:
      plVar10 = (longlong *)func_0x000140c18130();
      if (*plVar10 == 0) {
        plVar10 = (longlong *)FUN_1429090d0(plVar10,0);
        if (plVar10 == (longlong *)0x0) goto code_r0x000140bfb375;
      }
      else {
        if ((int)*plVar10 != 1) {
code_r0x000140bfb375:
          FUN_1428d80d0(&UNK_142d06498);
          goto code_r0x000140bfb454;
        }
        plVar10 = plVar10 + 1;
      }
      if (*plVar10 != 0) {
        FUN_1428d9250(&UNK_142d06508);
        goto code_r0x000140bfb454;
      }
      *plVar10 = -1;
      plStack_48 = plVar10;
      uStack_130 = (longlong *)FUN_140c16d40(plVar10 + 1,&UNK_142b16974,0xf,puVar13,lVar18,1,0);
      *plStack_48 = *plStack_48 + 1;
      uStack_138 = 1;
    }
    unaff_RBX = (longlong *)param_2[7];
    lVar18 = param_2[8];
    param_2[8] = 0;
    if (lVar18 != 0) {
      lVar18 = lVar18 << 3;
      lVar8 = 0;
      do {
        (**(code **)(param_1 + 0x188))(*(undefined8 *)(param_1 + 0x640),unaff_RBX[lVar8],0);
        lVar8 = lVar8 + 1;
        lVar18 = lVar18 + -8;
      } while (lVar18 != 0);
    }
    plStack_48 = (longlong *)CONCAT71(plStack_48._1_7_,cVar7);
    uStack_60._0_5_ = CONCAT14(8,(int)lVar8);
    FUN_1418c1380(0x143634e40,&uStack_60,5);
    if ((char)plStack_48 != '\0') {
      plVar10 = (longlong *)func_0x000140c18130();
      if (*plVar10 == 0) {
        plVar10 = (longlong *)FUN_1429090d0(plVar10,0);
        if (plVar10 == (longlong *)0x0) goto code_r0x000140bfb386;
      }
      else {
        if ((int)*plVar10 != 1) {
code_r0x000140bfb386:
          FUN_1428d80d0(&UNK_142d06498);
          goto code_r0x000140bfb454;
        }
        plVar10 = plVar10 + 1;
      }
      if (*plVar10 != 0) {
        FUN_1428d9250(&UNK_142d06508);
        goto code_r0x000140bfb454;
      }
      *plVar10 = -1;
      plStack_48 = plVar10;
      FUN_140c170f0(plVar10 + 1,uStack_130);
      *plStack_48 = *plStack_48 + 1;
    }
  }
  if (param_2[0xb] != 0) {
    if (3 < *puRam000000014362c3a8) {
      plVar10 = (longlong *)func_0x00014067dd80(&UNK_142b169d0);
      lStack_118 = *plVar10;
      uStack_110 = (undefined4)plVar10[1];
      uStack_10c = *(undefined4 *)((longlong)plVar10 + 0xc);
      uStack_ec = (undefined4)plVar10[2];
      lStack_108 = 4;
      puStack_100 = &UNK_142b169e8;
      uStack_f8 = 0x17;
      puStack_e8 = &UNK_142b169c0;
      uStack_e0 = 1;
      uStack_d8 = 8;
      uStack_138 = 0;
      uStack_d0 = 0;
      uStack_c8 = 0;
      uStack_130 = (longlong *)&UNK_142b169e8;
      uStack_128 = (longlong *)0x17;
      uStack_120 = 0;
      uStack_f0 = 1;
      func_0x000140d4a330(&plStack_90,&uStack_138);
    }
    FUN_140bf8980(param_1);
    if (param_2[0xb] != 0) {
      cStack_49 = func_0x000140c16a00();
      if (cStack_49 == '\0') {
        plStack_90 = (longlong *)0x0;
      }
      else {
        puVar16 = &UNK_142b16953;
        unaff_RBX = (longlong *)&UNK_142b16930;
        do {
          pbVar1 = puVar16 + -1;
          uVar11 = (uint)(char)*pbVar1;
          if ((int)uVar11 < 0) {
            bVar4 = puVar16[-2];
            if ((char)bVar4 < -0x40) {
              bVar5 = puVar16[-3];
              if ((char)bVar5 < -0x40) {
                pbVar2 = puVar16 + -4;
                puVar16 = puVar16 + -4;
                uVar11 = bVar5 & 0x3f | (*pbVar2 & 7) << 6;
              }
              else {
                puVar16 = puVar16 + -3;
                uVar11 = bVar5 & 0xf;
              }
              uVar11 = bVar4 & 0x3f | uVar11 << 6;
            }
            else {
              puVar16 = puVar16 + -2;
              uVar11 = bVar4 & 0x1f;
            }
            uVar11 = (uint)(*pbVar1 & 0x3f) | uVar11 << 6;
            if (uVar11 != 0x5c) goto code_r0x000140bf9e79;
code_r0x000140bf9e96:
            puVar13 = puVar16 + -0x142b1692f;
            if (puVar13 != (undefined *)0x0) {
              if (puVar13 < (undefined *)0x23) {
                if ((char)puVar16[1] < -0x40) {
code_r0x000140bf9eb5:
                  FUN_1428d9940(&UNK_142b16930,0x23,puVar13,0x23,&UNK_142b155d8);
                  goto code_r0x000140bfb454;
                }
              }
              else if (puVar13 != (undefined *)0x23) goto code_r0x000140bf9eb5;
            }
            lVar8 = 0x22 - (longlong)(puVar16 + -0x142b16930);
            unaff_RBX = (longlong *)(puVar16 + 1);
            goto code_r0x000140bf9eeb;
          }
          puVar16 = puVar16 + -1;
          if (uVar11 == 0x5c) goto code_r0x000140bf9e96;
code_r0x000140bf9e79:
          if (uVar11 == 0x2f) goto code_r0x000140bf9e96;
        } while (puVar16 != &UNK_142b16930);
        lVar8 = 0x23;
code_r0x000140bf9eeb:
        plVar10 = (longlong *)func_0x000140c18130();
        if (*plVar10 == 0) {
          plVar10 = (longlong *)FUN_1429090d0(plVar10,0);
          if (plVar10 == (longlong *)0x0) goto code_r0x000140bfb43a;
        }
        else {
          if ((int)*plVar10 != 1) {
code_r0x000140bfb43a:
            FUN_1428d80d0(&UNK_142d06498);
            goto code_r0x000140bfb454;
          }
          plVar10 = plVar10 + 1;
        }
        if (*plVar10 != 0) {
          FUN_1428d9250(&UNK_142d06508);
          goto code_r0x000140bfb454;
        }
        *plVar10 = -1;
        plStack_48 = plVar10;
        plStack_88 = (longlong *)FUN_140c16d40(plVar10 + 1,&UNK_142b169ff,0x12,unaff_RBX,lVar8,1,0);
        *plStack_48 = *plStack_48 + 1;
        plStack_90 = (longlong *)0x1;
      }
      uStack_128 = param_2 + 9;
      uStack_120 = param_2[0xb];
      param_2[0xb] = 0;
      uStack_130 = (longlong *)param_2[10] + uStack_120;
      lStack_118 = 0;
      if (uStack_120 != 0) {
        lVar8 = uStack_120 << 3;
        unaff_RBX = (longlong *)0x143634e40;
        plVar10 = (longlong *)param_2[10];
        do {
          plStack_48 = plVar10 + 1;
          (**(code **)(param_1 + 0x6b8))(*(undefined8 *)(param_1 + 0x6f0),*plVar10,0);
          uStack_61 = 0xe;
          FUN_1418c1380(0x143634e40,&uStack_61);
          lVar8 = lVar8 + -8;
          plVar10 = plStack_48;
        } while (lVar8 != 0);
      }
      if (cStack_49 != '\0') {
        plVar10 = (longlong *)func_0x000140c18130();
        if (*plVar10 == 0) {
          plVar10 = (longlong *)FUN_1429090d0(plVar10,0);
          if (plVar10 == (longlong *)0x0) goto code_r0x000140bfb448;
        }
        else {
          if ((int)*plVar10 != 1) {
code_r0x000140bfb448:
            FUN_1428d80d0(&UNK_142d06498);
            goto code_r0x000140bfb454;
          }
          plVar10 = plVar10 + 1;
        }
        if (*plVar10 != 0) {
          FUN_1428d9250(&UNK_142d06508);
          goto code_r0x000140bfb454;
        }
        *plVar10 = -1;
        plStack_48 = plVar10;
        FUN_140c170f0(plVar10 + 1,plStack_88);
        *plStack_48 = *plStack_48 + 1;
      }
    }
  }
  if (param_2[0xe] != 0) {
    cVar7 = func_0x000140c16a00();
    lVar8 = 0;
    if (cVar7 == '\0') {
      uStack_138 = 0;
    }
    else {
      puVar16 = &UNK_142b16953;
      puVar13 = &UNK_142b16930;
      do {
        pbVar1 = puVar16 + -1;
        uVar11 = (uint)(char)*pbVar1;
        if ((int)uVar11 < 0) {
          bVar4 = puVar16[-2];
          if ((char)bVar4 < -0x40) {
            bVar5 = puVar16[-3];
            if ((char)bVar5 < -0x40) {
              pbVar2 = puVar16 + -4;
              puVar16 = puVar16 + -4;
              uVar11 = bVar5 & 0x3f | (*pbVar2 & 7) << 6;
            }
            else {
              puVar16 = puVar16 + -3;
              uVar11 = bVar5 & 0xf;
            }
            uVar11 = bVar4 & 0x3f | uVar11 << 6;
          }
          else {
            puVar16 = puVar16 + -2;
            uVar11 = bVar4 & 0x1f;
          }
          uVar11 = (uint)(*pbVar1 & 0x3f) | uVar11 << 6;
          if (uVar11 != 0x5c) goto code_r0x000140bfa129;
code_r0x000140bfa146:
          puVar13 = puVar16 + -0x142b1692f;
          if (puVar13 != (undefined *)0x0) {
            if (puVar13 < (undefined *)0x23) {
              if ((char)puVar16[1] < -0x40) {
code_r0x000140bfa165:
                FUN_1428d9940(&UNK_142b16930,0x23,puVar13,0x23,&UNK_142b155d8);
                goto code_r0x000140bfb454;
              }
            }
            else if (puVar13 != (undefined *)0x23) goto code_r0x000140bfa165;
          }
          lVar18 = 0x22 - (longlong)(puVar16 + -0x142b16930);
          puVar13 = puVar16 + 1;
          goto code_r0x000140bfa19b;
        }
        puVar16 = puVar16 + -1;
        if (uVar11 == 0x5c) goto code_r0x000140bfa146;
code_r0x000140bfa129:
        if (uVar11 == 0x2f) goto code_r0x000140bfa146;
      } while (puVar16 != &UNK_142b16930);
      lVar18 = 0x23;
code_r0x000140bfa19b:
      plVar10 = (longlong *)func_0x000140c18130();
      if (*plVar10 == 0) {
        plVar10 = (longlong *)FUN_1429090d0(plVar10,0);
        if (plVar10 == (longlong *)0x0) goto code_r0x000140bfb397;
      }
      else {
        if ((int)*plVar10 != 1) {
code_r0x000140bfb397:
          FUN_1428d80d0(&UNK_142d06498);
          goto code_r0x000140bfb454;
        }
        plVar10 = plVar10 + 1;
      }
      if (*plVar10 != 0) {
        FUN_1428d9250(&UNK_142d06508);
        goto code_r0x000140bfb454;
      }
      *plVar10 = -1;
      plStack_48 = plVar10;
      uStack_130 = (longlong *)FUN_140c16d40(plVar10 + 1,&UNK_142b16a11,0x12,puVar13,lVar18,1,0);
      *plStack_48 = *plStack_48 + 1;
      uStack_138 = 1;
    }
    unaff_RBX = (longlong *)param_2[0xd];
    lVar18 = param_2[0xe];
    param_2[0xe] = 0;
    if (lVar18 != 0) {
      lVar18 = lVar18 << 3;
      lVar8 = 0;
      do {
        (**(code **)(param_1 + 0x138))(*(undefined8 *)(param_1 + 0x640),unaff_RBX[lVar8],0);
        lVar8 = lVar8 + 1;
        lVar18 = lVar18 + -8;
      } while (lVar18 != 0);
    }
    plStack_48 = (longlong *)CONCAT71(plStack_48._1_7_,cVar7);
    uStack_60._0_5_ = CONCAT14(9,(int)lVar8);
    FUN_1418c1380(0x143634e40,&uStack_60,5);
    if ((char)plStack_48 != '\0') {
      plVar10 = (longlong *)func_0x000140c18130();
      if (*plVar10 == 0) {
        plVar10 = (longlong *)FUN_1429090d0(plVar10,0);
        if (plVar10 == (longlong *)0x0) goto code_r0x000140bfb3a8;
      }
      else {
        if ((int)*plVar10 != 1) {
code_r0x000140bfb3a8:
          FUN_1428d80d0(&UNK_142d06498);
          goto code_r0x000140bfb454;
        }
        plVar10 = plVar10 + 1;
      }
      if (*plVar10 != 0) {
        FUN_1428d9250(&UNK_142d06508);
        goto code_r0x000140bfb454;
      }
      *plVar10 = -1;
      plStack_48 = plVar10;
      FUN_140c170f0(plVar10 + 1,uStack_130);
      *plStack_48 = *plStack_48 + 1;
    }
  }
  if (param_2[0x11] != 0) {
    cVar7 = func_0x000140c16a00();
    lVar8 = 0;
    if (cVar7 == '\0') {
      uStack_138 = 0;
    }
    else {
      puVar16 = &UNK_142b16953;
      puVar13 = &UNK_142b16930;
      do {
        pbVar1 = puVar16 + -1;
        uVar11 = (uint)(char)*pbVar1;
        if ((int)uVar11 < 0) {
          bVar4 = puVar16[-2];
          if ((char)bVar4 < -0x40) {
            bVar5 = puVar16[-3];
            if ((char)bVar5 < -0x40) {
              pbVar2 = puVar16 + -4;
              puVar16 = puVar16 + -4;
              uVar11 = bVar5 & 0x3f | (*pbVar2 & 7) << 6;
            }
            else {
              puVar16 = puVar16 + -3;
              uVar11 = bVar5 & 0xf;
            }
            uVar11 = bVar4 & 0x3f | uVar11 << 6;
          }
          else {
            puVar16 = puVar16 + -2;
            uVar11 = bVar4 & 0x1f;
          }
          uVar11 = (uint)(*pbVar1 & 0x3f) | uVar11 << 6;
          if (uVar11 != 0x5c) goto code_r0x000140bfa3b9;
code_r0x000140bfa3d6:
          puVar13 = puVar16 + -0x142b1692f;
          if (puVar13 != (undefined *)0x0) {
            if (puVar13 < (undefined *)0x23) {
              if ((char)puVar16[1] < -0x40) {
code_r0x000140bfa3f5:
                FUN_1428d9940(&UNK_142b16930,0x23,puVar13,0x23,&UNK_142b155d8);
                goto code_r0x000140bfb454;
              }
            }
            else if (puVar13 != (undefined *)0x23) goto code_r0x000140bfa3f5;
          }
          lVar18 = 0x22 - (longlong)(puVar16 + -0x142b16930);
          puVar13 = puVar16 + 1;
          goto code_r0x000140bfa42b;
        }
        puVar16 = puVar16 + -1;
        if (uVar11 == 0x5c) goto code_r0x000140bfa3d6;
code_r0x000140bfa3b9:
        if (uVar11 == 0x2f) goto code_r0x000140bfa3d6;
      } while (puVar16 != &UNK_142b16930);
      lVar18 = 0x23;
code_r0x000140bfa42b:
      plVar10 = (longlong *)func_0x000140c18130();
      if (*plVar10 == 0) {
        plVar10 = (longlong *)FUN_1429090d0(plVar10,0);
        if (plVar10 == (longlong *)0x0) goto code_r0x000140bfb3b9;
      }
      else {
        if ((int)*plVar10 != 1) {
code_r0x000140bfb3b9:
          FUN_1428d80d0(&UNK_142d06498);
          goto code_r0x000140bfb454;
        }
        plVar10 = plVar10 + 1;
      }
      if (*plVar10 != 0) {
        FUN_1428d9250(&UNK_142d06508);
        goto code_r0x000140bfb454;
      }
      *plVar10 = -1;
      plStack_48 = plVar10;
      uStack_130 = (longlong *)FUN_140c16d40(plVar10 + 1,&UNK_142b16a23,0x14,puVar13,lVar18,1,0);
      *plStack_48 = *plStack_48 + 1;
      uStack_138 = 1;
    }
    unaff_RBX = (longlong *)param_2[0x10];
    lVar18 = param_2[0x11];
    param_2[0x11] = 0;
    if (lVar18 != 0) {
      lVar18 = lVar18 << 3;
      lVar8 = 0;
      do {
        (**(code **)(param_1 + 0x278))(*(undefined8 *)(param_1 + 0x640),unaff_RBX[lVar8],0);
        lVar8 = lVar8 + 1;
        lVar18 = lVar18 + -8;
      } while (lVar18 != 0);
    }
    plStack_48 = (longlong *)CONCAT71(plStack_48._1_7_,cVar7);
    uStack_60._0_5_ = CONCAT14(10,(int)lVar8);
    FUN_1418c1380(0x143634e40,&uStack_60,5);
    if ((char)plStack_48 != '\0') {
      plVar10 = (longlong *)func_0x000140c18130();
      if (*plVar10 == 0) {
        plVar10 = (longlong *)FUN_1429090d0(plVar10,0);
        if (plVar10 == (longlong *)0x0) goto code_r0x000140bfb3ca;
      }
      else {
        if ((int)*plVar10 != 1) {
code_r0x000140bfb3ca:
          FUN_1428d80d0(&UNK_142d06498);
          goto code_r0x000140bfb454;
        }
        plVar10 = plVar10 + 1;
      }
      if (*plVar10 != 0) {
        FUN_1428d9250(&UNK_142d06508);
        goto code_r0x000140bfb454;
      }
      *plVar10 = -1;
      plStack_48 = plVar10;
      FUN_140c170f0(plVar10 + 1,uStack_130);
      *plStack_48 = *plStack_48 + 1;
    }
  }
  if (param_2[0x14] != 0) {
    cVar7 = func_0x000140c16a00();
    lVar8 = 0;
    if (cVar7 == '\0') {
      uStack_138 = 0;
    }
    else {
      puVar16 = &UNK_142b16953;
      puVar13 = &UNK_142b16930;
      do {
        pbVar1 = puVar16 + -1;
        uVar11 = (uint)(char)*pbVar1;
        if ((int)uVar11 < 0) {
          bVar4 = puVar16[-2];
          if ((char)bVar4 < -0x40) {
            bVar5 = puVar16[-3];
            if ((char)bVar5 < -0x40) {
              pbVar2 = puVar16 + -4;
              puVar16 = puVar16 + -4;
              uVar11 = bVar5 & 0x3f | (*pbVar2 & 7) << 6;
            }
            else {
              puVar16 = puVar16 + -3;
              uVar11 = bVar5 & 0xf;
            }
            uVar11 = bVar4 & 0x3f | uVar11 << 6;
          }
          else {
            puVar16 = puVar16 + -2;
            uVar11 = bVar4 & 0x1f;
          }
          uVar11 = (uint)(*pbVar1 & 0x3f) | uVar11 << 6;
          if (uVar11 != 0x5c) goto code_r0x000140bfa659;
code_r0x000140bfa676:
          puVar13 = puVar16 + -0x142b1692f;
          if (puVar13 != (undefined *)0x0) {
            if (puVar13 < (undefined *)0x23) {
              if ((char)puVar16[1] < -0x40) {
code_r0x000140bfa695:
                FUN_1428d9940(&UNK_142b16930,0x23,puVar13,0x23,&UNK_142b155d8);
                goto code_r0x000140bfb454;
              }
            }
            else if (puVar13 != (undefined *)0x23) goto code_r0x000140bfa695;
          }
          lVar18 = 0x22 - (longlong)(puVar16 + -0x142b16930);
          puVar13 = puVar16 + 1;
          goto code_r0x000140bfa6cb;
        }
        puVar16 = puVar16 + -1;
        if (uVar11 == 0x5c) goto code_r0x000140bfa676;
code_r0x000140bfa659:
        if (uVar11 == 0x2f) goto code_r0x000140bfa676;
      } while (puVar16 != &UNK_142b16930);
      lVar18 = 0x23;
code_r0x000140bfa6cb:
      plVar10 = (longlong *)func_0x000140c18130();
      if (*plVar10 == 0) {
        plVar10 = (longlong *)FUN_1429090d0(plVar10,0);
        if (plVar10 == (longlong *)0x0) goto code_r0x000140bfb3d8;
      }
      else {
        if ((int)*plVar10 != 1) {
code_r0x000140bfb3d8:
          FUN_1428d80d0(&UNK_142d06498);
          goto code_r0x000140bfb454;
        }
        plVar10 = plVar10 + 1;
      }
      if (*plVar10 != 0) {
        FUN_1428d9250(&UNK_142d06508);
        goto code_r0x000140bfb454;
      }
      *plVar10 = -1;
      plStack_48 = plVar10;
      uStack_130 = (longlong *)FUN_140c16d40(plVar10 + 1,&UNK_142b16a37,0x15,puVar13,lVar18,1,0);
      *plStack_48 = *plStack_48 + 1;
      uStack_138 = 1;
    }
    unaff_RBX = (longlong *)param_2[0x13];
    lVar18 = param_2[0x14];
    param_2[0x14] = 0;
    if (lVar18 != 0) {
      lVar18 = lVar18 << 3;
      lVar8 = 0;
      do {
        (**(code **)(param_1 + 0x288))(*(undefined8 *)(param_1 + 0x640),unaff_RBX[lVar8],0);
        lVar8 = lVar8 + 1;
        lVar18 = lVar18 + -8;
      } while (lVar18 != 0);
    }
    plStack_48 = (longlong *)CONCAT71(plStack_48._1_7_,cVar7);
    uStack_60._0_5_ = CONCAT14(0xb,(int)lVar8);
    FUN_1418c1380(0x143634e40,&uStack_60,5);
    if ((char)plStack_48 != '\0') {
      plVar10 = (longlong *)func_0x000140c18130();
      if (*plVar10 == 0) {
        plVar10 = (longlong *)FUN_1429090d0(plVar10,0);
        if (plVar10 == (longlong *)0x0) goto code_r0x000140bfb3e6;
      }
      else {
        if ((int)*plVar10 != 1) {
code_r0x000140bfb3e6:
          FUN_1428d80d0(&UNK_142d06498);
          goto code_r0x000140bfb454;
        }
        plVar10 = plVar10 + 1;
      }
      if (*plVar10 != 0) {
        FUN_1428d9250(&UNK_142d06508);
        goto code_r0x000140bfb454;
      }
      *plVar10 = -1;
      plStack_48 = plVar10;
      FUN_140c170f0(plVar10 + 1,uStack_130);
      *plStack_48 = *plStack_48 + 1;
    }
  }
  if (param_2[0x17] != 0) {
    cVar7 = func_0x000140c16a00();
    lVar8 = 0;
    if (cVar7 == '\0') {
      uStack_138 = 0;
    }
    else {
      puVar16 = &UNK_142b16953;
      puVar13 = &UNK_142b16930;
      do {
        pbVar1 = puVar16 + -1;
        uVar11 = (uint)(char)*pbVar1;
        if ((int)uVar11 < 0) {
          bVar4 = puVar16[-2];
          if ((char)bVar4 < -0x40) {
            bVar5 = puVar16[-3];
            if ((char)bVar5 < -0x40) {
              pbVar2 = puVar16 + -4;
              puVar16 = puVar16 + -4;
              uVar11 = bVar5 & 0x3f | (*pbVar2 & 7) << 6;
            }
            else {
              puVar16 = puVar16 + -3;
              uVar11 = bVar5 & 0xf;
            }
            uVar11 = bVar4 & 0x3f | uVar11 << 6;
          }
          else {
            puVar16 = puVar16 + -2;
            uVar11 = bVar4 & 0x1f;
          }
          uVar11 = (uint)(*pbVar1 & 0x3f) | uVar11 << 6;
          if (uVar11 != 0x5c) goto code_r0x000140bfa8f9;
code_r0x000140bfa916:
          puVar13 = puVar16 + -0x142b1692f;
          if (puVar13 != (undefined *)0x0) {
            if (puVar13 < (undefined *)0x23) {
              if ((char)puVar16[1] < -0x40) {
code_r0x000140bfa935:
                FUN_1428d9940(&UNK_142b16930,0x23,puVar13,0x23,&UNK_142b155d8);
                goto code_r0x000140bfb454;
              }
            }
            else if (puVar13 != (undefined *)0x23) goto code_r0x000140bfa935;
          }
          lVar18 = 0x22 - (longlong)(puVar16 + -0x142b16930);
          puVar13 = puVar16 + 1;
          goto code_r0x000140bfa96b;
        }
        puVar16 = puVar16 + -1;
        if (uVar11 == 0x5c) goto code_r0x000140bfa916;
code_r0x000140bfa8f9:
        if (uVar11 == 0x2f) goto code_r0x000140bfa916;
      } while (puVar16 != &UNK_142b16930);
      lVar18 = 0x23;
code_r0x000140bfa96b:
      plVar10 = (longlong *)func_0x000140c18130();
      if (*plVar10 == 0) {
        plVar10 = (longlong *)FUN_1429090d0(plVar10,0);
        if (plVar10 == (longlong *)0x0) goto code_r0x000140bfb3f4;
      }
      else {
        if ((int)*plVar10 != 1) {
code_r0x000140bfb3f4:
          FUN_1428d80d0(&UNK_142d06498);
          goto code_r0x000140bfb454;
        }
        plVar10 = plVar10 + 1;
      }
      if (*plVar10 != 0) {
        FUN_1428d9250(&UNK_142d06508);
        goto code_r0x000140bfb454;
      }
      *plVar10 = -1;
      plStack_48 = plVar10;
      uStack_130 = (longlong *)FUN_140c16d40(plVar10 + 1,&UNK_142b16a4c,0x10,puVar13,lVar18,1,0);
      *plStack_48 = *plStack_48 + 1;
      uStack_138 = 1;
    }
    unaff_RBX = (longlong *)param_2[0x16];
    lVar18 = param_2[0x17];
    param_2[0x17] = 0;
    if (lVar18 != 0) {
      lVar18 = lVar18 << 3;
      lVar8 = 0;
      do {
        (**(code **)(param_1 + 0x228))(*(undefined8 *)(param_1 + 0x640),unaff_RBX[lVar8],0);
        lVar8 = lVar8 + 1;
        lVar18 = lVar18 + -8;
      } while (lVar18 != 0);
    }
    plStack_48 = (longlong *)CONCAT71(plStack_48._1_7_,cVar7);
    uStack_60._0_5_ = CONCAT14(0xc,(int)lVar8);
    FUN_1418c1380(0x143634e40,&uStack_60,5);
    if ((char)plStack_48 != '\0') {
      plVar10 = (longlong *)func_0x000140c18130();
      if (*plVar10 == 0) {
        plVar10 = (longlong *)FUN_1429090d0(plVar10,0);
        if (plVar10 == (longlong *)0x0) goto code_r0x000140bfb402;
      }
      else {
        if ((int)*plVar10 != 1) {
code_r0x000140bfb402:
          FUN_1428d80d0(&UNK_142d06498);
          goto code_r0x000140bfb454;
        }
        plVar10 = plVar10 + 1;
      }
      if (*plVar10 != 0) {
        FUN_1428d9250(&UNK_142d06508);
        goto code_r0x000140bfb454;
      }
      *plVar10 = -1;
      plStack_48 = plVar10;
      FUN_140c170f0(plVar10 + 1,uStack_130);
      *plStack_48 = *plStack_48 + 1;
    }
  }
  if (param_2[0x1a] != 0) {
    cVar7 = func_0x000140c16a00();
    lVar8 = 0;
    if (cVar7 == '\0') {
      uStack_138 = 0;
    }
    else {
      puVar16 = &UNK_142b16953;
      puVar13 = &UNK_142b16930;
      do {
        pbVar1 = puVar16 + -1;
        uVar11 = (uint)(char)*pbVar1;
        if ((int)uVar11 < 0) {
          bVar4 = puVar16[-2];
          if ((char)bVar4 < -0x40) {
            bVar5 = puVar16[-3];
            if ((char)bVar5 < -0x40) {
              pbVar2 = puVar16 + -4;
              puVar16 = puVar16 + -4;
              uVar11 = bVar5 & 0x3f | (*pbVar2 & 7) << 6;
            }
            else {
              puVar16 = puVar16 + -3;
              uVar11 = bVar5 & 0xf;
            }
            uVar11 = bVar4 & 0x3f | uVar11 << 6;
          }
          else {
            puVar16 = puVar16 + -2;
            uVar11 = bVar4 & 0x1f;
          }
          uVar11 = (uint)(*pbVar1 & 0x3f) | uVar11 << 6;
          if (uVar11 != 0x5c) goto code_r0x000140bfab99;
code_r0x000140bfabb6:
          puVar13 = puVar16 + -0x142b1692f;
          if (puVar13 != (undefined *)0x0) {
            if (puVar13 < (undefined *)0x23) {
              if ((char)puVar16[1] < -0x40) {
code_r0x000140bfabd5:
                FUN_1428d9940(&UNK_142b16930,0x23,puVar13,0x23,&UNK_142b155d8);
                goto code_r0x000140bfb454;
              }
            }
            else if (puVar13 != (undefined *)0x23) goto code_r0x000140bfabd5;
          }
          lVar18 = 0x22 - (longlong)(puVar16 + -0x142b16930);
          puVar13 = puVar16 + 1;
          goto code_r0x000140bfac0b;
        }
        puVar16 = puVar16 + -1;
        if (uVar11 == 0x5c) goto code_r0x000140bfabb6;
code_r0x000140bfab99:
        if (uVar11 == 0x2f) goto code_r0x000140bfabb6;
      } while (puVar16 != &UNK_142b16930);
      lVar18 = 0x23;
code_r0x000140bfac0b:
      plVar10 = (longlong *)func_0x000140c18130();
      if (*plVar10 == 0) {
        plVar10 = (longlong *)FUN_1429090d0(plVar10,0);
        if (plVar10 == (longlong *)0x0) goto code_r0x000140bfb410;
      }
      else {
        if ((int)*plVar10 != 1) {
code_r0x000140bfb410:
          FUN_1428d80d0(&UNK_142d06498);
          goto code_r0x000140bfb454;
        }
        plVar10 = plVar10 + 1;
      }
      if (*plVar10 != 0) {
        FUN_1428d9250(&UNK_142d06508);
        goto code_r0x000140bfb454;
      }
      *plVar10 = -1;
      plStack_48 = plVar10;
      uStack_130 = (longlong *)FUN_140c16d40(plVar10 + 1,&UNK_142b16a5c,0x18,puVar13,lVar18,1,0);
      *plStack_48 = *plStack_48 + 1;
      uStack_138 = 1;
    }
    unaff_RBX = (longlong *)param_2[0x19];
    lVar18 = param_2[0x1a];
    param_2[0x1a] = 0;
    if (lVar18 != 0) {
      lVar18 = lVar18 << 3;
      lVar8 = 0;
      do {
        (**(code **)(param_1 + 0x248))(*(undefined8 *)(param_1 + 0x640),unaff_RBX[lVar8],0);
        lVar8 = lVar8 + 1;
        lVar18 = lVar18 + -8;
      } while (lVar18 != 0);
    }
    plStack_48 = (longlong *)CONCAT71(plStack_48._1_7_,cVar7);
    uStack_60._0_5_ = CONCAT14(0xd,(int)lVar8);
    FUN_1418c1380(0x143634e40,&uStack_60,5);
    if ((char)plStack_48 != '\0') {
      plVar10 = (longlong *)func_0x000140c18130();
      if (*plVar10 == 0) {
        plVar10 = (longlong *)FUN_1429090d0(plVar10,0);
        if (plVar10 == (longlong *)0x0) goto code_r0x000140bfb41e;
      }
      else {
        if ((int)*plVar10 != 1) {
code_r0x000140bfb41e:
          FUN_1428d80d0(&UNK_142d06498);
          goto code_r0x000140bfb454;
        }
        plVar10 = plVar10 + 1;
      }
      if (*plVar10 != 0) {
        FUN_1428d9250(&UNK_142d06508);
        goto code_r0x000140bfb454;
      }
      *plVar10 = -1;
      plStack_48 = plVar10;
      FUN_140c170f0(plVar10 + 1,uStack_130);
      *plStack_48 = *plStack_48 + 1;
    }
  }
  if (param_2[0x1d] == 0) goto code_r0x000140bfb0ce;
  cVar7 = func_0x000140c16a00();
  if (cVar7 == '\0') {
    uStack_60 = 0;
    goto code_r0x000140bfaf2b;
  }
  puVar16 = &UNK_142b16953;
  puVar13 = &UNK_142b16930;
  do {
    pbVar1 = puVar16 + -1;
    uVar11 = (uint)(char)*pbVar1;
    if ((int)uVar11 < 0) {
      bVar4 = puVar16[-2];
      if ((char)bVar4 < -0x40) {
        bVar5 = puVar16[-3];
        if ((char)bVar5 < -0x40) {
          pbVar2 = puVar16 + -4;
          puVar16 = puVar16 + -4;
          uVar11 = bVar5 & 0x3f | (*pbVar2 & 7) << 6;
        }
        else {
          puVar16 = puVar16 + -3;
          uVar11 = bVar5 & 0xf;
        }
        uVar11 = bVar4 & 0x3f | uVar11 << 6;
      }
      else {
        puVar16 = puVar16 + -2;
        uVar11 = bVar4 & 0x1f;
      }
      uVar11 = (uint)(*pbVar1 & 0x3f) | uVar11 << 6;
      if (uVar11 != 0x5c) goto code_r0x000140bfae29;
code_r0x000140bfae46:
      puVar13 = puVar16 + -0x142b1692f;
      if (puVar13 != (undefined *)0x0) {
        if (puVar13 < (undefined *)0x23) {
          if ((char)puVar16[1] < -0x40) {
code_r0x000140bfae65:
            FUN_1428d9940(&UNK_142b16930,0x23,puVar13,0x23,&UNK_142b155d8);
            goto code_r0x000140bfb454;
          }
        }
        else if (puVar13 != (undefined *)0x23) goto code_r0x000140bfae65;
      }
      lVar8 = 0x22 - (longlong)(puVar16 + -0x142b16930);
      puVar13 = puVar16 + 1;
      goto code_r0x000140bfae9b;
    }
    puVar16 = puVar16 + -1;
    if (uVar11 == 0x5c) goto code_r0x000140bfae46;
code_r0x000140bfae29:
    if (uVar11 == 0x2f) goto code_r0x000140bfae46;
  } while (puVar16 != &UNK_142b16930);
  lVar8 = 0x23;
code_r0x000140bfae9b:
  plVar10 = (longlong *)func_0x000140c18130();
  if (*plVar10 == 0) {
    plVar10 = (longlong *)FUN_1429090d0(plVar10,0);
    if (plVar10 == (longlong *)0x0) goto code_r0x000140bfb42c;
  }
  else {
    if ((int)*plVar10 != 1) {
code_r0x000140bfb42c:
      FUN_1428d80d0(&UNK_142d06498);
      goto code_r0x000140bfb454;
    }
    plVar10 = plVar10 + 1;
  }
  if (*plVar10 == 0) {
    *plVar10 = -1;
    plStack_48 = plVar10;
    uStack_58 = FUN_140c16d40(plVar10 + 1,&UNK_142b16a74,0x10,puVar13,lVar8,1,0);
    *plStack_48 = *plStack_48 + 1;
    uStack_60 = 1;
code_r0x000140bfaf2b:
    param_1 = *(longlong *)(param_1 + 0x678);
    plVar14 = (longlong *)(param_1 + 0x10);
    LOCK();
    bVar20 = *(char *)(param_1 + 0x10) == '\0';
    if (bVar20) {
      *(char *)(param_1 + 0x10) = '\x01';
    }
    UNLOCK();
    if (!bVar20) goto code_r0x000140bfb16d;
    do {
      FUN_140c81c70(plVar14);
      plStack_80 = param_2 + 0x1b;
      plStack_90 = (longlong *)param_2[0x1c];
      lStack_78 = param_2[0x1d];
      param_2[0x1d] = 0;
      plStack_88 = plStack_90 + lStack_78 * 9;
      uStack_70 = 0;
      param_1 = param_1 + 0x18;
      unaff_RBX = &uStack_138;
      plVar10 = plStack_90;
      plStack_48 = plVar14;
      if (plStack_90 != plStack_88) {
        do {
          plVar10 = plStack_90 + 9;
          if (*(char *)((longlong)plStack_90 + 0x44) == '\x02') break;
          uStack_138 = *plStack_90;
          uStack_130 = (longlong *)plStack_90[1];
          uStack_128 = (longlong *)plStack_90[2];
          uStack_120 = plStack_90[3];
          lStack_118 = plStack_90[4];
          uStack_110 = (undefined4)plStack_90[5];
          uStack_10c = *(undefined4 *)((longlong)plStack_90 + 0x2c);
          lStack_108 = plStack_90[6];
          puStack_100 = (undefined *)plStack_90[7];
          uStack_f8 = CONCAT35(*(undefined3 *)((longlong)plStack_90 + 0x45),
                               CONCAT14(*(char *)((longlong)plStack_90 + 0x44),(int)plStack_90[8]));
          plStack_90 = plStack_90 + 9;
          FUN_140c0e150(&iStack_b0,param_1,unaff_RBX);
          if (iStack_b0 != 9) {
            uStack_128 = (longlong *)CONCAT44(uStack_9c,uStack_a0);
            uStack_120 = CONCAT44(uStack_94,uStack_98);
            uStack_138 = CONCAT44(uStack_ac,iStack_b0);
            uStack_130 = (longlong *)CONCAT44(uStack_a4,uStack_a8);
            FUN_1428d9760(&UNK_142b15188,0x2b,&uStack_138,&UNK_142b15148,&UNK_142b16a88);
            goto code_r0x000140bfb454;
          }
          plVar10 = plStack_90;
        } while (plStack_90 != plStack_88);
      }
      plStack_90 = plVar10;
      FUN_140be1cf0(&plStack_90);
      param_2 = plStack_48;
      FUN_140c81e10();
      LOCK();
      bVar20 = (char)*param_2 == '\x01';
      if (bVar20) {
        *(char *)param_2 = '\0';
      }
      UNLOCK();
      if (!bVar20) {
        FUN_1428cdb90(param_2,0);
      }
      if (uStack_60 != 0) {
        plVar10 = (longlong *)func_0x000140c18130();
        if (*plVar10 == 0) {
          plVar10 = (longlong *)FUN_1429090d0(plVar10,0);
          if (plVar10 == (longlong *)0x0) goto code_r0x000140bfb320;
        }
        else {
          if ((int)*plVar10 != 1) {
code_r0x000140bfb320:
            FUN_1428d80d0(&UNK_142d06498);
            goto code_r0x000140bfb454;
          }
          plVar10 = plVar10 + 1;
        }
        if (*plVar10 != 0) {
          FUN_1428d9250(&UNK_142d06508);
          goto code_r0x000140bfb454;
        }
        *plVar10 = -1;
        plStack_48 = plVar10;
        FUN_140c170f0(plVar10 + 1,uStack_58);
        *plStack_48 = *plStack_48 + 1;
      }
code_r0x000140bfb0ce:
      if (lStack_158 == 0) {
        return;
      }
      plVar10 = (longlong *)func_0x000140c18130();
      if (*plVar10 == 1) {
        plVar10 = plVar10 + 1;
code_r0x000140bfb0ea:
        plVar14 = unaff_RBX;
        if (*plVar10 == 0) {
          *plVar10 = -1;
          plStack_48 = plVar10;
          FUN_140c170f0(plVar10 + 1,uStack_150);
          *plStack_48 = *plStack_48 + 1;
          return;
        }
      }
      else {
        if ((*plVar10 == 0) &&
           (plVar10 = (longlong *)FUN_1429090d0(plVar10,0), plVar10 != (longlong *)0x0))
        goto code_r0x000140bfb0ea;
code_r0x000140bfb155:
        FUN_1428d80d0(&UNK_142d06498);
        plVar14 = unaff_RBX;
      }
code_r0x000140bfb161:
      FUN_1428d9250(&UNK_142d06508);
code_r0x000140bfb16d:
      FUN_1428cd210(plVar14);
    } while( true );
  }
  FUN_1428d9250(&UNK_142d06508);
code_r0x000140bfb454:
                    /* WARNING: Does not return */
  pcVar6 = (code *)invalidInstructionException();
  (*pcVar6)();
}

