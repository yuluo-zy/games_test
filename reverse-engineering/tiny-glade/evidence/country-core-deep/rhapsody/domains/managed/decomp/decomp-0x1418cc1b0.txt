
ulonglong FUN_1418cc1b0(longlong *param_1,longlong param_2,undefined8 param_3,undefined4 param_4,
                       uint param_5,uint param_6)

{
  byte *pbVar1;
  byte bVar2;
  code *pcVar3;
  char cVar4;
  byte bVar5;
  byte bVar6;
  uint uVar7;
  longlong lVar8;
  longlong *plVar9;
  undefined8 *puVar10;
  ulonglong uVar11;
  longlong lVar12;
  ulonglong uVar13;
  undefined *puVar14;
  int iVar15;
  undefined *puVar16;
  undefined *puVar17;
  bool bVar18;
  undefined4 auStack_160 [2];
  undefined8 uStack_158;
  undefined4 uStack_150;
  undefined8 *puStack_148;
  undefined4 uStack_140;
  longlong *plStack_138;
  undefined4 *puStack_130;
  undefined8 uStack_128;
  int *piStack_110;
  undefined *puStack_108;
  longlong lStack_100;
  undefined8 uStack_f8;
  undefined4 auStack_f0 [2];
  undefined8 uStack_e8;
  undefined *puStack_e0;
  int **ppiStack_d8;
  undefined8 uStack_d0;
  undefined8 uStack_c8;
  undefined4 uStack_c0;
  undefined4 uStack_bc;
  undefined8 uStack_b8;
  undefined *puStack_b0;
  undefined8 uStack_a8;
  undefined4 uStack_a0;
  undefined4 uStack_9c;
  undefined *puStack_98;
  undefined8 uStack_90;
  undefined8 uStack_88;
  undefined8 uStack_80;
  undefined8 uStack_78;
  longlong lStack_68;
  undefined8 uStack_60;
  int iStack_54;
  longlong *plStack_50;
  longlong *plStack_48;
  undefined8 uStack_40;
  
  uStack_40 = 0xfffffffffffffffe;
  LOCK();
  lVar8 = *param_1;
  if (lVar8 == 0) {
    *param_1 = 8;
  }
  UNLOCK();
  if (lVar8 != 0) {
    FUN_1428cb5f0(param_1,param_2,1000000000);
  }
  FUN_140c81c70(param_1);
  plStack_50 = param_1;
  FUN_140c81c70((ulonglong)param_1 | 1);
  uStack_f8 = param_3;
  auStack_f0[0] = param_4;
  cVar4 = func_0x000140c16a00();
  if (cVar4 == '\0') {
    lStack_68 = 0;
  }
  else {
    puVar16 = &UNK_142d08238;
    lVar12 = 0x40;
    uVar11 = 0x40;
    lVar8 = FUN_1418c99c0(&UNK_142d08238);
    if (lVar8 == 1) {
      if (uVar11 != 0) {
        if (uVar11 < 0x40) {
          if ((char)(&UNK_142d08238)[uVar11] < -0x40) {
code_r0x0001418cc25c:
            FUN_1428d9940(&UNK_142d08238,0x40,0,uVar11,&UNK_142d08300);
            goto code_r0x0001418cc9ea;
          }
        }
        else if (uVar11 != 0x40) goto code_r0x0001418cc25c;
      }
      puVar16 = &UNK_142d08238;
      lVar8 = FUN_1418c99c0(&UNK_142d08238);
      if (lVar8 == 1) {
        uVar13 = uVar11 + 2;
        if (uVar13 != 0) {
          if (uVar13 < 0x40) {
            if ((char)(&UNK_142d0823a)[uVar11] < -0x40) {
code_r0x0001418cc2bf:
              FUN_1428d9940(&UNK_142d08238,0x40,uVar13,0x40,&UNK_142d08318);
              goto code_r0x0001418cc9ea;
            }
          }
          else if (uVar13 != 0x40) goto code_r0x0001418cc2bf;
        }
        lVar12 = 0x3e - uVar11;
        puVar16 = &UNK_142d0823a + uVar11;
      }
    }
    puVar14 = &UNK_142d0842c;
    puVar17 = &UNK_142d08400;
    do {
      bVar6 = puVar14[-1];
      uVar7 = (uint)(char)bVar6;
      if ((char)bVar6 < '\0') {
        bVar5 = puVar14[-2];
        if ((char)bVar5 < -0x40) {
          bVar2 = puVar14[-3];
          if ((char)bVar2 < -0x40) {
            pbVar1 = puVar14 + -4;
            puVar14 = puVar14 + -4;
            uVar7 = bVar2 & 0x3f | (*pbVar1 & 7) << 6;
          }
          else {
            puVar14 = puVar14 + -3;
            uVar7 = bVar2 & 0xf;
          }
          uVar7 = bVar5 & 0x3f | uVar7 << 6;
        }
        else {
          puVar14 = puVar14 + -2;
          uVar7 = bVar5 & 0x1f;
        }
        uVar7 = (uint)(bVar6 & 0x3f) | uVar7 << 6;
        if (uVar7 != 0x5c) goto code_r0x0001418cc384;
code_r0x0001418cc396:
        puVar17 = puVar14 + -0x142d083ff;
        if (puVar17 != (undefined *)0x0) {
          if (puVar17 < (undefined *)0x2c) {
            if ((char)puVar14[1] < -0x40) {
code_r0x0001418cc3b2:
              FUN_1428d9940(&UNK_142d08400,0x2c,puVar17,0x2c,&UNK_142d082e0);
              goto code_r0x0001418cc9ea;
            }
          }
          else if (puVar17 != (undefined *)0x2c) goto code_r0x0001418cc3b2;
        }
        lVar8 = 0x2c - (longlong)puVar17;
        puVar17 = puVar14 + 1;
        goto code_r0x0001418cc3e8;
      }
      puVar14 = puVar14 + -1;
      if (uVar7 == 0x5c) goto code_r0x0001418cc396;
code_r0x0001418cc384:
      if (uVar7 == 0x2f) goto code_r0x0001418cc396;
    } while (puVar14 != &UNK_142d08400);
    lVar8 = 0x2c;
code_r0x0001418cc3e8:
    plVar9 = (longlong *)func_0x000140c18130();
    if (*plVar9 == 0) {
      plVar9 = (longlong *)FUN_1429090d0(plVar9,0);
      if (plVar9 == (longlong *)0x0) goto code_r0x0001418cc904;
    }
    else {
      if ((int)*plVar9 != 1) {
code_r0x0001418cc904:
        FUN_1428d80d0(&UNK_142d06498);
        goto code_r0x0001418cc9ea;
      }
      plVar9 = plVar9 + 1;
    }
    if (*plVar9 != 0) {
      FUN_1428d9250(&UNK_142d06508);
      goto code_r0x0001418cc9ea;
    }
    *plVar9 = -1;
    plStack_48 = plVar9;
    uStack_60 = FUN_140c16d40(plVar9 + 1,puVar16,lVar12,puVar17,lVar8,1,0);
    *plStack_48 = *plStack_48 + 1;
    lStack_68 = 1;
  }
  uStack_e8 = (undefined *)CONCAT71(uStack_e8._1_7_,0x20);
  FUN_1418c1380(0x143634e40,&uStack_e8,1);
  bVar5 = (char)plStack_50[0x15] - 2;
  bVar6 = 1;
  if (bVar5 < 3) {
    bVar6 = bVar5;
  }
  if (bVar6 != 0) {
    if (bVar6 != 1) {
      uStack_e8 = &UNK_142d08930;
      puStack_e0 = (undefined *)0x1;
      ppiStack_d8 = (int **)0x8;
      uStack_d0 = 0;
      uStack_c8 = 0;
      FUN_1428d9390(&uStack_e8,&UNK_142d08940);
      goto code_r0x0001418cc9ea;
    }
    uStack_e8 = (undefined *)CONCAT71(uStack_e8._1_7_,0x26);
    FUN_1418c1380(0x143634e40,&uStack_e8,1);
    goto code_r0x0001418cc4d9;
  }
  puStack_130 = auStack_f0;
  lStack_100 = plStack_50[0x14];
  auStack_160[0] = 0x3b9acde9;
  uStack_158 = 0;
  uStack_150 = 1;
  puStack_148 = &uStack_f8;
  uStack_140 = 1;
  plStack_138 = &lStack_100;
  uStack_128 = 0;
  plStack_48 = (longlong *)(param_2 + 0x718);
  LOCK();
  bVar18 = *(char *)(param_2 + 0x718) == '\0';
  if (bVar18) {
    *(char *)(param_2 + 0x718) = '\x01';
  }
  UNLOCK();
  if (!bVar18) {
    FUN_1428cd210(plStack_48,plStack_50,1000000000);
  }
  FUN_140c81c70(plStack_48);
  if ((param_5 & 1) == 0) {
    lVar8 = param_2 + 0x730;
  }
  else {
    uVar11 = (ulonglong)param_6;
    if (*(ulonglong *)(param_2 + 0x7f0) <= uVar11) {
      FUN_1428d9518(uVar11,*(ulonglong *)(param_2 + 0x7f0),&UNK_142d06788);
      goto code_r0x0001418cc9ea;
    }
    lVar8 = uVar11 * 0xb0 + *(longlong *)(param_2 + 0x7e8);
  }
  uVar11 = FUN_140c03d70(lVar8,param_2 + 0x800,param_2,auStack_160);
  if ((uVar11 & 1) == 0) {
code_r0x0001418cc7b3:
    FUN_140c81e10();
    LOCK();
    bVar18 = (char)*plStack_48 == '\x01';
    if (bVar18) {
      *(char *)plStack_48 = '\0';
    }
    UNLOCK();
    if (!bVar18) {
      FUN_1428cdb90(plStack_48,0);
    }
    uVar11 = uVar11 ^ 1;
  }
  else {
    iVar15 = (int)(uVar11 >> 0x20);
    if (iVar15 == -0x3b9eae18) {
      uStack_e8 = (undefined *)CONCAT71(uStack_e8._1_7_,0x24);
      FUN_1418c1380(0x143634e40,&uStack_e8,1);
      if (*(char *)((longlong)plStack_50 + 0xb9) == '\0') goto code_r0x0001418cc7b3;
      if (3 < *puRam000000014362c3a8) {
        puVar10 = (undefined8 *)func_0x00014067dd80(&UNK_142d088e8);
        uStack_c8 = *puVar10;
        uStack_c0 = *(undefined4 *)(puVar10 + 1);
        uStack_bc = *(undefined4 *)((longlong)puVar10 + 0xc);
        uStack_9c = *(undefined4 *)(puVar10 + 2);
        uStack_b8 = 4;
        puStack_b0 = &UNK_142d08470;
        uStack_a8 = 0x21;
        puStack_98 = &UNK_142d088d8;
        uStack_90 = 1;
        uStack_88 = 8;
        uStack_e8 = (undefined *)0x0;
        uStack_80 = 0;
        uStack_78 = 0;
        puStack_e0 = &UNK_142d08470;
        ppiStack_d8 = (int **)0x21;
        uStack_d0 = 0;
        uStack_a0 = 1;
        func_0x000140d4a330(&piStack_110,&uStack_e8);
      }
      FUN_140c81e10();
      LOCK();
      bVar18 = (char)*plStack_48 == '\x01';
      if (bVar18) {
        *(char *)plStack_48 = '\0';
      }
      UNLOCK();
      if (!bVar18) {
        FUN_1428cdb90(plStack_48,0);
      }
code_r0x0001418cc4d9:
      uVar11 = 0;
    }
    else {
      if (iVar15 != 0x3b9acdeb) {
        if (iVar15 != -0x3b9acdec) {
          uStack_e8._0_5_ = CONCAT14(0x25,iVar15);
          iStack_54 = iVar15;
          FUN_1418c1380(0x143634e40,&uStack_e8,5);
          cVar4 = FUN_140befc70(param_2,uVar11 >> 0x20);
          if (cVar4 == '\0') {
            FUN_140befd60(param_2);
          }
          piStack_110 = &iStack_54;
          puStack_108 = &UNK_140c8f550;
          uStack_e8 = &UNK_142d08890;
          puStack_e0 = (undefined *)0x1;
          uStack_c8 = 0;
          ppiStack_d8 = &piStack_110;
          uStack_d0 = 1;
          FUN_1428d9390(&uStack_e8,&UNK_142d088a0);
          goto code_r0x0001418cc9ea;
        }
        uStack_e8 = (undefined *)CONCAT71(uStack_e8._1_7_,0x22);
        FUN_1418c1380(0x143634e40,&uStack_e8,1);
        FUN_140c81e10();
        LOCK();
        bVar18 = (char)*plStack_48 == '\x01';
        if (bVar18) {
          *(char *)plStack_48 = '\0';
        }
        UNLOCK();
        if (!bVar18) {
          FUN_1428cdb90(plStack_48,0);
        }
        goto code_r0x0001418cc4d9;
      }
      uStack_e8 = (undefined *)CONCAT71(uStack_e8._1_7_,0x23);
      FUN_1418c1380(0x143634e40,&uStack_e8,1);
      FUN_140c81e10();
      uVar11 = CONCAT71((int7)(uVar11 >> 8),1);
      LOCK();
      bVar18 = (char)*plStack_48 == '\x01';
      if (bVar18) {
        *(char *)plStack_48 = '\0';
      }
      UNLOCK();
      if (!bVar18) {
        FUN_1428cdb90(plStack_48,0);
      }
    }
    if ((int)plStack_50[6] == 2) {
      plStack_50[10] = plStack_50[5];
      plStack_50[8] = plStack_50[3];
      plStack_50[9] = plStack_50[4];
      *(int *)(plStack_50 + 6) = (int)plStack_50[1];
      *(undefined4 *)((longlong)plStack_50 + 0x34) = *(undefined4 *)((longlong)plStack_50 + 0xc);
      *(int *)(plStack_50 + 7) = (int)plStack_50[2];
      *(undefined4 *)((longlong)plStack_50 + 0x3c) = *(undefined4 *)((longlong)plStack_50 + 0x14);
    }
  }
  if (lStack_68 == 0) goto code_r0x0001418cc853;
  plVar9 = (longlong *)func_0x000140c18130();
  if (*plVar9 == 0) {
    plVar9 = (longlong *)FUN_1429090d0(plVar9,0);
    if (plVar9 == (longlong *)0x0) goto code_r0x0001418cc8f3;
  }
  else {
    if ((int)*plVar9 != 1) {
code_r0x0001418cc8f3:
      FUN_1428d80d0(&UNK_142d06498);
      goto code_r0x0001418cc9ea;
    }
    plVar9 = plVar9 + 1;
  }
  if (*plVar9 == 0) {
    *plVar9 = -1;
    plStack_48 = plVar9;
    FUN_140c170f0(plVar9 + 1,uStack_60);
    *plStack_48 = *plStack_48 + 1;
code_r0x0001418cc853:
    plVar9 = plStack_50;
    FUN_140c81e10(plStack_50);
    FUN_140c81e10();
    LOCK();
    lVar8 = *plVar9;
    if (lVar8 == 8) {
      *plVar9 = 0;
    }
    UNLOCK();
    if (lVar8 != 8) {
      FUN_1428cbf60(plVar9,0);
    }
    return uVar11 & 0xffffff01;
  }
  FUN_1428d9250(&UNK_142d06508);
code_r0x0001418cc9ea:
                    /* WARNING: Does not return */
  pcVar3 = (code *)invalidInstructionException();
  (*pcVar3)();
}

