
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

undefined8 * FUN_1418cb520(undefined8 *param_1,longlong *param_2,longlong param_3)

{
  byte *pbVar1;
  longlong *plVar2;
  int *piVar3;
  byte bVar4;
  byte bVar5;
  byte bVar6;
  undefined1 uVar7;
  undefined2 uVar8;
  undefined4 uVar9;
  code *pcVar10;
  undefined4 uVar11;
  undefined4 uVar12;
  undefined4 uVar13;
  undefined8 uVar14;
  undefined8 uVar15;
  longlong lVar16;
  longlong *plVar17;
  char cVar18;
  uint uVar19;
  uint uVar20;
  longlong lVar21;
  longlong *plVar22;
  undefined8 *puVar23;
  undefined4 *puVar24;
  ulonglong uVar25;
  longlong lVar26;
  undefined8 uVar27;
  ulonglong uVar28;
  undefined *puVar29;
  undefined *puVar30;
  ulonglong uVar31;
  longlong lVar32;
  undefined *puVar33;
  undefined8 uStack_130;
  uint uStack_128;
  undefined4 uStack_124;
  undefined4 uStack_120;
  undefined4 uStack_11c;
  undefined1 uStack_118;
  undefined7 uStack_117;
  undefined1 uStack_110;
  undefined7 uStack_10f;
  undefined4 uStack_108;
  undefined4 uStack_104;
  undefined4 uStack_100;
  undefined4 uStack_fc;
  uint **ppuStack_f8;
  undefined *puStack_f0;
  undefined8 uStack_e8;
  undefined4 uStack_e0;
  undefined4 uStack_dc;
  undefined *puStack_d8;
  undefined8 uStack_d0;
  uint **ppuStack_c8;
  undefined8 uStack_c0;
  undefined8 uStack_b8;
  uint *puStack_a8;
  undefined *puStack_a0;
  uint *puStack_98;
  undefined *puStack_90;
  undefined *puStack_88;
  undefined8 uStack_80;
  undefined8 uStack_78;
  longlong *plStack_70;
  ulonglong uStack_68;
  longlong *plStack_60;
  uint uStack_54;
  longlong *plStack_50;
  undefined8 uStack_48;
  
  uStack_48 = 0xfffffffffffffffe;
  LOCK();
  lVar21 = *param_2;
  if (lVar21 == 0) {
    *param_2 = 8;
  }
  UNLOCK();
  if (lVar21 != 0) {
    FUN_1428cb5f0(param_2,param_2,1000000000);
  }
  FUN_140c81c70(param_2);
  uVar31 = (ulonglong)param_2 | 1;
  plStack_50 = param_2;
  FUN_140c81c70(uVar31);
  cVar18 = func_0x000140c16a00();
  plStack_60 = (longlong *)CONCAT71(plStack_60._1_7_,cVar18);
  uStack_68 = uVar31;
  if (cVar18 != '\0') {
    puVar33 = &UNK_142d081f0;
    lVar32 = 0x45;
    uVar31 = 0x45;
    lVar21 = FUN_1418c99c0(&UNK_142d081f0);
    if (lVar21 == 1) {
      if (uVar31 != 0) {
        if (uVar31 < 0x45) {
          if ((char)(&UNK_142d081f0)[uVar31] < -0x40) {
code_r0x0001418cb5d7:
            FUN_1428d9940(&UNK_142d081f0,0x45,0,uVar31,&UNK_142d08300);
            goto code_r0x0001418cbffb;
          }
        }
        else if (uVar31 != 0x45) goto code_r0x0001418cb5d7;
      }
      puVar33 = &UNK_142d081f0;
      lVar21 = FUN_1418c99c0(&UNK_142d081f0);
      if (lVar21 == 1) {
        uVar28 = uVar31 + 2;
        if (uVar28 != 0) {
          if (uVar28 < 0x45) {
            if ((char)(&UNK_142d081f2)[uVar31] < -0x40) {
code_r0x0001418cb63a:
              FUN_1428d9940(&UNK_142d081f0,0x45,uVar28,0x45,&UNK_142d08318);
              goto code_r0x0001418cbffb;
            }
          }
          else if (uVar28 != 0x45) goto code_r0x0001418cb63a;
        }
        lVar32 = 0x43 - uVar31;
        puVar33 = &UNK_142d081f2 + uVar31;
      }
    }
    puVar29 = &UNK_142d0842c;
    puVar30 = &UNK_142d08400;
    do {
      bVar4 = puVar29[-1];
      uVar19 = (uint)(char)bVar4;
      if ((char)bVar4 < '\0') {
        bVar5 = puVar29[-2];
        if ((char)bVar5 < -0x40) {
          bVar6 = puVar29[-3];
          if ((char)bVar6 < -0x40) {
            pbVar1 = puVar29 + -4;
            puVar29 = puVar29 + -4;
            uVar19 = bVar6 & 0x3f | (*pbVar1 & 7) << 6;
          }
          else {
            puVar29 = puVar29 + -3;
            uVar19 = bVar6 & 0xf;
          }
          uVar19 = bVar5 & 0x3f | uVar19 << 6;
        }
        else {
          puVar29 = puVar29 + -2;
          uVar19 = bVar5 & 0x1f;
        }
        uVar19 = (uint)(bVar4 & 0x3f) | uVar19 << 6;
        if (uVar19 != 0x5c) goto code_r0x0001418cb704;
code_r0x0001418cb715:
        puVar30 = puVar29 + -0x142d083ff;
        if (puVar30 != (undefined *)0x0) {
          if (puVar30 < (undefined *)0x2c) {
            if ((char)puVar29[1] < -0x40) {
code_r0x0001418cb731:
              FUN_1428d9940(&UNK_142d08400,0x2c,puVar30,0x2c,&UNK_142d082e0);
              goto code_r0x0001418cbffb;
            }
          }
          else if (puVar30 != (undefined *)0x2c) goto code_r0x0001418cb731;
        }
        lVar21 = 0x2c - (longlong)puVar30;
        puVar30 = puVar29 + 1;
        goto code_r0x0001418cb766;
      }
      puVar29 = puVar29 + -1;
      if (uVar19 == 0x5c) goto code_r0x0001418cb715;
code_r0x0001418cb704:
      if (uVar19 == 0x2f) goto code_r0x0001418cb715;
    } while (puVar29 != &UNK_142d08400);
    lVar21 = 0x2c;
code_r0x0001418cb766:
    plVar22 = (longlong *)func_0x000140c18130();
    if (*plVar22 == 0) {
      plVar22 = (longlong *)FUN_1429090d0(plVar22,0);
      if (plVar22 == (longlong *)0x0) goto code_r0x0001418cbf5e;
    }
    else {
      if ((int)*plVar22 != 1) {
code_r0x0001418cbf5e:
        FUN_1428d80d0(&UNK_142d06498);
        goto code_r0x0001418cbffb;
      }
      plVar22 = plVar22 + 1;
    }
    if (*plVar22 != 0) {
      FUN_1428d9250(&UNK_142d06508);
      goto code_r0x0001418cbffb;
    }
    *plVar22 = -1;
    plStack_70 = plVar22;
    uStack_130 = FUN_140c16d40(plVar22 + 1,puVar33,lVar32,puVar30,lVar21,1,0);
    *plStack_70 = *plStack_70 + 1;
  }
  uStack_128 = CONCAT31(uStack_128._1_3_,0x1f);
  FUN_1418c1380(0x143634e40,&uStack_128,1);
  plVar17 = plStack_50;
  plVar22 = plStack_50 + 1;
  if ((char)plStack_50[0x15] == '\x02') {
    plVar2 = plStack_50 + 6;
    if ((int)plStack_50[6] != 2) {
      uStack_128 = CONCAT31(uStack_128._1_3_,0x28);
      FUN_1418c1380(0x143634e40,&uStack_128,1);
      uVar27 = 0x10;
      puVar23 = (undefined8 *)func_0x000140613c10(0x10,1);
      uVar14 = _UNK_142d08738;
      if (puVar23 == (undefined8 *)0x0) {
        FUN_1428d8fe3(1,0x10,&UNK_142d081b8);
        goto code_r0x0001418cbffb;
      }
      *puVar23 = _UNK_142d08730;
      puVar23[1] = uVar14;
      uVar9 = *(undefined4 *)((longlong)plVar17 + 0x34);
      lVar21 = plVar17[7];
      uVar11 = *(undefined4 *)((longlong)plVar17 + 0x3c);
      lVar32 = plVar17[8];
      uVar12 = *(undefined4 *)((longlong)plVar17 + 0x44);
      lVar26 = plVar17[9];
      uVar13 = *(undefined4 *)((longlong)plVar17 + 0x4c);
      *(int *)(param_1 + 1) = (int)*plVar2;
      *(undefined4 *)((longlong)param_1 + 0xc) = uVar9;
      *(int *)(param_1 + 2) = (int)lVar21;
      *(undefined4 *)((longlong)param_1 + 0x14) = uVar11;
      *(int *)(param_1 + 3) = (int)lVar32;
      *(undefined4 *)((longlong)param_1 + 0x1c) = uVar12;
      *(int *)(param_1 + 4) = (int)lVar26;
      *(undefined4 *)((longlong)param_1 + 0x24) = uVar13;
      param_1[5] = plVar17[10];
      param_1[6] = 0x10;
      param_1[7] = puVar23;
      lVar21 = 0x40;
      goto code_r0x0001418cba59;
    }
    uVar31 = plStack_50[0x16];
    if ((ulonglong)plStack_50[0x10] <= uVar31) {
      FUN_1428d9518(uVar31,plStack_50[0x10],&UNK_142d08740);
      goto code_r0x0001418cbffb;
    }
    if ((ulonglong)plStack_50[0x13] <= uVar31) {
      FUN_1428d9518(uVar31,plStack_50[0x13],&UNK_142d08758);
      goto code_r0x0001418cbffb;
    }
    uVar27 = *(undefined8 *)(plStack_50[0xf] + uVar31 * 8);
    plStack_70 = *(longlong **)(plStack_50[0x12] + uVar31 * 8);
    uVar20 = (**(code **)(param_3 + 0x6c8))
                       (*(undefined8 *)(param_3 + 0x6f0),plStack_50[0x14],3000000000,uVar27,0,
                        &uStack_128);
    uVar19 = uStack_128;
    if ((uVar20 == 0x3b9acdeb) || (uVar20 == 0)) {
      uVar28 = plStack_50[0xd];
      if (uVar28 == 0) {
        FUN_1428d9a20(&UNK_142d08770);
        goto code_r0x0001418cbffb;
      }
      uVar25 = (ulonglong)uStack_128;
      uVar31 = uVar31 + 1;
      if ((uVar31 | uVar28) >> 0x20 == 0) {
        plStack_50[0x16] = (uVar31 & 0xffffffff) % (uVar28 & 0xffffffff);
      }
      else {
        plStack_50[0x16] = uVar31 % uVar28;
      }
      if (uVar28 <= uVar25) {
        FUN_1428d9518(uVar25,uVar28,&UNK_142d08788);
        goto code_r0x0001418cbffb;
      }
      lVar21 = plStack_50[0xc];
      lVar26 = uVar25 * 0x38;
      lVar32 = *(longlong *)(lVar21 + 8 + lVar26);
      LOCK();
      piVar3 = (int *)(lVar32 + 8);
      *piVar3 = *piVar3 + 1;
      UNLOCK();
      uVar9 = *(undefined4 *)(lVar21 + 0x10 + lVar26);
      uVar7 = *(undefined1 *)(lVar21 + 0x16 + lVar26);
      uVar8 = *(undefined2 *)(lVar21 + 0x14 + lVar26);
      puVar23 = (undefined8 *)(lVar21 + 0x28 + lVar26);
      uVar14 = *puVar23;
      uVar15 = puVar23[1];
      uStack_117 = (undefined7)uVar14;
      uStack_110 = (undefined1)((ulonglong)uVar14 >> 0x38);
      uStack_10f = (undefined7)uVar15;
      uStack_108._0_1_ = (undefined1)((ulonglong)uVar15 >> 0x38);
      puVar23 = (undefined8 *)(lVar21 + 0x18 + lVar26);
      uVar14 = *puVar23;
      uVar15 = puVar23[1];
      uStack_128._1_3_ = (undefined3)uVar14;
      uStack_124 = (undefined4)((ulonglong)uVar14 >> 0x18);
      uStack_120._0_1_ = (undefined1)((ulonglong)uVar14 >> 0x38);
      uStack_120._1_3_ = (undefined3)uVar15;
      uStack_11c = (undefined4)((ulonglong)uVar15 >> 0x18);
      uStack_118 = (undefined1)((ulonglong)uVar15 >> 0x38);
      param_1[1] = *(undefined8 *)(lVar21 + lVar26);
      param_1[2] = lVar32;
      *(undefined4 *)(param_1 + 3) = uVar9;
      *(undefined2 *)((longlong)param_1 + 0x1c) = uVar8;
      *(undefined1 *)((longlong)param_1 + 0x1e) = uVar7;
      *(uint *)((longlong)param_1 + 0x1f) = uStack_128;
      *(undefined4 *)((longlong)param_1 + 0x23) = uStack_124;
      *(undefined4 *)((longlong)param_1 + 0x27) = uStack_120;
      *(undefined4 *)((longlong)param_1 + 0x2b) = uStack_11c;
      *(ulonglong *)((longlong)param_1 + 0x2f) = CONCAT71(uStack_117,uStack_118);
      *(ulonglong *)((longlong)param_1 + 0x37) = CONCAT71(uStack_10f,uStack_110);
      *(undefined1 *)((longlong)param_1 + 0x3f) = (undefined1)uStack_108;
      *(uint *)(param_1 + 8) = uVar19;
      param_1[9] = uVar27;
      param_1[10] = plStack_70;
      *param_1 = 0;
    }
    else if (uVar20 - 1 < 2) {
      uStack_128 = CONCAT31(uStack_128._1_3_,0x2d);
      uStack_54 = uVar20;
      FUN_1418c1380(0x143634e40,&uStack_128,1);
      if (*puRam000000014362c3a8 != 0) {
        puStack_98 = &uStack_54;
        puStack_90 = &UNK_140c8f550;
        puVar24 = (undefined4 *)func_0x00014067dd80(&UNK_142d087e8);
        uStack_108 = *puVar24;
        uStack_104 = puVar24[1];
        uStack_100 = puVar24[2];
        uStack_fc = puVar24[3];
        uStack_dc = puVar24[4];
        ppuStack_f8 = (uint **)0x1;
        puStack_f0 = &UNK_142d08470;
        uStack_e8 = 0x21;
        puStack_d8 = &UNK_142d087d8;
        uStack_d0 = 1;
        ppuStack_c8 = &puStack_98;
        uStack_c0 = 1;
        uStack_b8 = 0;
        uStack_128 = 0;
        uStack_124 = 0;
        uStack_120 = 0x42d08470;
        uStack_11c = 1;
        uStack_118 = 0x21;
        uStack_117 = 0;
        uStack_110 = 0;
        uStack_10f = 0;
        uStack_e0 = 1;
        func_0x000140d4a330(&puStack_a8,&uStack_128);
      }
      uVar9 = *(undefined4 *)((longlong)plVar17 + 0xc);
      lVar21 = plVar17[2];
      uVar11 = *(undefined4 *)((longlong)plVar17 + 0x14);
      lVar32 = plVar17[3];
      uVar12 = *(undefined4 *)((longlong)plVar17 + 0x1c);
      lVar26 = plVar17[4];
      uVar13 = *(undefined4 *)((longlong)plVar17 + 0x24);
      *(int *)plVar2 = (int)*plVar22;
      *(undefined4 *)((longlong)plVar17 + 0x34) = uVar9;
      *(int *)(plVar17 + 7) = (int)lVar21;
      *(undefined4 *)((longlong)plVar17 + 0x3c) = uVar11;
      plVar17[10] = plVar17[5];
      *(int *)(plVar17 + 8) = (int)lVar32;
      *(undefined4 *)((longlong)plVar17 + 0x44) = uVar12;
      *(int *)(plVar17 + 9) = (int)lVar26;
      *(undefined4 *)((longlong)plVar17 + 0x4c) = uVar13;
      lVar21 = *plVar2;
      puStack_a8 = &uStack_54;
      puStack_a0 = &UNK_140c8f550;
      uStack_128 = 0x42d08820;
      uStack_124 = 1;
      uStack_120 = 1;
      uStack_11c = 0;
      uStack_118 = SUB81(&puStack_a8,0);
      uStack_117 = (undefined7)((ulonglong)&puStack_a8 >> 8);
      uStack_110 = 1;
      uStack_10f = 0;
      uStack_108 = 0;
      uStack_104 = 0;
      FUN_140d79f90(&puStack_98,&uStack_128);
      param_1[8] = puStack_88;
      param_1[6] = puStack_98;
      param_1[7] = puStack_90;
      lVar26 = plStack_50[2];
      uVar9 = *(undefined4 *)((longlong)plStack_50 + 0x14);
      lVar16 = plStack_50[3];
      uVar11 = *(undefined4 *)((longlong)plStack_50 + 0x1c);
      lVar32 = plStack_50[5];
      param_1[4] = plStack_50[4];
      param_1[5] = lVar32;
      *(int *)(param_1 + 2) = (int)lVar26;
      *(undefined4 *)((longlong)param_1 + 0x14) = uVar9;
      *(int *)(param_1 + 3) = (int)lVar16;
      *(undefined4 *)((longlong)param_1 + 0x1c) = uVar11;
      param_1[1] = lVar21;
      *param_1 = 1;
    }
    else if (uVar20 == 0xc4653214) {
      uStack_54 = 0xc4653214;
      uStack_128 = CONCAT31(uStack_128._1_3_,0x22);
      FUN_1418c1380(0x143634e40,&uStack_128,1);
      if (1 < *puRam000000014362c3a8) {
        puStack_a8 = &uStack_54;
        puStack_a0 = &UNK_140c8f550;
        uStack_128 = 0x42d08848;
        uStack_124 = 1;
        uStack_120 = 1;
        uStack_11c = 0;
        uStack_108 = 0;
        uStack_104 = 0;
        uStack_118 = SUB81(&puStack_a8,0);
        uStack_117 = (undefined7)((ulonglong)&puStack_a8 >> 8);
        uStack_110 = 1;
        uStack_10f = 0;
        uStack_78 = func_0x00014067dd80(&UNK_142d08858);
        puStack_98 = (uint *)&UNK_142d08470;
        puStack_90 = (undefined *)0x21;
        puStack_88 = &UNK_142d08470;
        uStack_80 = 0x21;
        FUN_1404b1430(&uStack_128,2,&puStack_98);
      }
      plVar17[10] = plVar17[5];
      lVar21 = *plVar22;
      uVar9 = *(undefined4 *)((longlong)plVar17 + 0xc);
      lVar32 = plVar17[2];
      uVar11 = *(undefined4 *)((longlong)plVar17 + 0x14);
      *(int *)(plVar17 + 8) = (int)plVar17[3];
      *(undefined4 *)((longlong)plVar17 + 0x44) = *(undefined4 *)((longlong)plVar17 + 0x1c);
      *(int *)(plVar17 + 9) = (int)plVar17[4];
      *(undefined4 *)((longlong)plVar17 + 0x4c) = *(undefined4 *)((longlong)plVar17 + 0x24);
      *(int *)plVar2 = (int)lVar21;
      *(undefined4 *)((longlong)plVar17 + 0x34) = uVar9;
      *(int *)(plVar17 + 7) = (int)lVar32;
      *(undefined4 *)((longlong)plVar17 + 0x3c) = uVar11;
      lVar21 = plStack_50[6];
      puStack_98 = (uint *)0x0;
      puStack_90 = (undefined *)0x1;
      puStack_88 = (undefined *)0x0;
      uStack_128 = 0;
      uStack_124 = 0;
      uStack_118 = 0;
      uStack_117 = 0;
      uStack_108 = 0x20;
      uStack_104 = 0;
      uStack_100 = CONCAT31(uStack_100._1_3_,3);
      ppuStack_f8 = &puStack_98;
      puStack_f0 = &UNK_142d07f10;
      cVar18 = func_0x000140c8f510(&uStack_54,&uStack_128);
      if (cVar18 != '\0') {
        FUN_1428d9760(&UNK_142d07f40,0x37,&puStack_a8,&UNK_142d08100,&UNK_142d07fc8);
        goto code_r0x0001418cbffb;
      }
      param_1[8] = puStack_88;
      param_1[6] = puStack_98;
      param_1[7] = puStack_90;
      lVar26 = plStack_50[2];
      uVar9 = *(undefined4 *)((longlong)plStack_50 + 0x14);
      lVar16 = plStack_50[3];
      uVar11 = *(undefined4 *)((longlong)plStack_50 + 0x1c);
      lVar32 = plStack_50[5];
      param_1[4] = plStack_50[4];
      param_1[5] = lVar32;
      *(int *)(param_1 + 2) = (int)lVar26;
      *(undefined4 *)((longlong)param_1 + 0x14) = uVar9;
      *(int *)(param_1 + 3) = (int)lVar16;
      *(undefined4 *)((longlong)param_1 + 0x1c) = uVar11;
      param_1[1] = lVar21;
      *param_1 = 1;
    }
    else {
      uStack_124 = CONCAT31(uStack_124._1_3_,0x25);
      uStack_128 = uVar20;
      FUN_1418c1380(0x143634e40,&uStack_128,5);
      param_1[1] = 5;
      param_1[2] = &UNK_142d087a0;
      param_1[3] = 0x15;
      *(uint *)(param_1 + 4) = uVar20;
      *param_1 = 1;
    }
    if ((char)plStack_60 == '\0') goto code_r0x0001418cbce2;
    plVar22 = (longlong *)func_0x000140c18130();
    if (*plVar22 == 0) {
      plVar22 = (longlong *)FUN_1429090d0(plVar22,0);
      if (plVar22 == (longlong *)0x0) goto code_r0x0001418cbfc5;
    }
    else {
      if ((int)*plVar22 != 1) {
code_r0x0001418cbfc5:
        FUN_1428d80d0(&UNK_142d06498);
        goto code_r0x0001418cbffb;
      }
      plVar22 = plVar22 + 1;
    }
    if (*plVar22 != 0) {
      FUN_1428d9250(&UNK_142d06508);
      goto code_r0x0001418cbffb;
    }
    *plVar22 = -1;
    plStack_60 = plVar22;
    FUN_140c170f0(plVar22 + 1,uStack_130);
  }
  else {
    param_1[6] = plStack_50[5];
    lVar21 = *plVar22;
    uVar9 = *(undefined4 *)((longlong)plStack_50 + 0xc);
    lVar32 = plStack_50[2];
    uVar11 = *(undefined4 *)((longlong)plStack_50 + 0x14);
    uVar12 = *(undefined4 *)((longlong)plStack_50 + 0x1c);
    lVar26 = plStack_50[4];
    uVar13 = *(undefined4 *)((longlong)plStack_50 + 0x24);
    *(int *)(param_1 + 4) = (int)plStack_50[3];
    *(undefined4 *)((longlong)param_1 + 0x24) = uVar12;
    *(int *)(param_1 + 5) = (int)lVar26;
    *(undefined4 *)((longlong)param_1 + 0x2c) = uVar13;
    *(int *)(param_1 + 2) = (int)lVar21;
    *(undefined4 *)((longlong)param_1 + 0x14) = uVar9;
    *(int *)(param_1 + 3) = (int)lVar32;
    *(undefined4 *)((longlong)param_1 + 0x1c) = uVar11;
    uVar27 = 4;
    lVar21 = 8;
code_r0x0001418cba59:
    *(undefined8 *)((longlong)param_1 + lVar21) = uVar27;
    *param_1 = 1;
    if ((char)plStack_60 == '\0') goto code_r0x0001418cbce2;
    plVar22 = (longlong *)func_0x000140c18130();
    if (*plVar22 == 0) {
      plVar22 = (longlong *)FUN_1429090d0(plVar22,0);
      if (plVar22 == (longlong *)0x0) goto code_r0x0001418cbf6f;
    }
    else {
      if ((int)*plVar22 != 1) {
code_r0x0001418cbf6f:
        FUN_1428d80d0(&UNK_142d06498);
        goto code_r0x0001418cbffb;
      }
      plVar22 = plVar22 + 1;
    }
    if (*plVar22 != 0) {
      FUN_1428d9250(&UNK_142d06508);
code_r0x0001418cbffb:
                    /* WARNING: Does not return */
      pcVar10 = (code *)invalidInstructionException();
      (*pcVar10)();
    }
    *plVar22 = -1;
    plStack_60 = plVar22;
    FUN_140c170f0(plVar22 + 1,uStack_130);
  }
  *plStack_60 = *plStack_60 + 1;
code_r0x0001418cbce2:
  plVar22 = plStack_50;
  FUN_140c81e10(plStack_50);
  FUN_140c81e10();
  LOCK();
  lVar21 = *plVar22;
  if (lVar21 == 8) {
    *plVar22 = 0;
  }
  UNLOCK();
  if (lVar21 != 8) {
    FUN_1428cbf60(plVar22,0);
  }
  return param_1;
}

