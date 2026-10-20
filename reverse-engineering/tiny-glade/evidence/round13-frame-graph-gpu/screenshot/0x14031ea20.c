
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

longlong * FUN_14031ea20(undefined8 param_1)

{
  byte *pbVar1;
  byte bVar2;
  byte bVar3;
  byte bVar4;
  char cVar5;
  uint uVar6;
  longlong lVar7;
  longlong *plVar8;
  ulonglong uVar9;
  undefined *puVar10;
  longlong lVar11;
  ulonglong uVar12;
  undefined *puVar13;
  undefined *puVar14;
  longlong lVar15;
  undefined1 uVar16;
  bool bVar17;
  longlong lStack_50;
  undefined8 uStack_48;
  
  cVar5 = func_0x000140c16a00();
  if (cVar5 == '\0') {
LAB_14031eaa4:
    lStack_50 = 0;
  }
  else {
    puVar10 = &UNK_1429bc60e;
    lVar11 = 0x31;
    uVar9 = 0x31;
    lVar7 = FUN_14031aa90(&UNK_1429bc60e);
    if (lVar7 == 1) {
      if (uVar9 != 0) {
        if (uVar9 < 0x31) {
          if ((char)(&UNK_1429bc60e)[uVar9] < -0x40) {
LAB_14031ea84:
            FUN_1428d9940(&UNK_1429bc60e,0x31,0,uVar9,&UNK_1429bc6f8);
            goto LAB_14031eaa4;
          }
        }
        else if (uVar9 != 0x31) goto LAB_14031ea84;
      }
      puVar10 = &UNK_1429bc60e;
      lVar7 = FUN_14031aa90(&UNK_1429bc60e);
      if (lVar7 == 1) {
        uVar12 = uVar9 + 2;
        if (uVar12 != 0) {
          uVar16 = uVar12 == 0x31;
          if (0x30 < uVar12) goto LAB_14031eb05;
          uVar16 = (&UNK_1429bc610)[uVar9] == -0x41;
          if ((char)(&UNK_1429bc610)[uVar9] < -0x40) {
            do {
              uVar9 = 0x31;
              FUN_1428d9940(&UNK_1429bc60e,0x31);
LAB_14031eb05:
            } while (!(bool)uVar16);
          }
        }
        lVar11 = 0x2f - uVar9;
        puVar10 = &UNK_1429bc60e + uVar12;
      }
    }
    puVar13 = &UNK_1429bc763;
    puVar14 = &UNK_1429bc740;
    do {
      bVar2 = puVar13[-1];
      uVar6 = (uint)(char)bVar2;
      if ((char)bVar2 < '\0') {
        bVar3 = puVar13[-2];
        if ((char)bVar3 < -0x40) {
          bVar4 = puVar13[-3];
          if ((char)bVar4 < -0x40) {
            pbVar1 = puVar13 + -4;
            puVar13 = puVar13 + -4;
            uVar6 = bVar4 & 0x3f | (*pbVar1 & 7) << 6;
          }
          else {
            puVar13 = puVar13 + -3;
            uVar6 = bVar4 & 0xf;
          }
          uVar6 = bVar3 & 0x3f | uVar6 << 6;
        }
        else {
          puVar13 = puVar13 + -2;
          uVar6 = bVar3 & 0x1f;
        }
        uVar6 = (uint)(bVar2 & 0x3f) | uVar6 << 6;
        if (uVar6 != 0x5c) goto LAB_14031eba4;
LAB_14031ebb6:
        puVar14 = puVar13 + -0x1429bc73f;
        if (puVar14 != (undefined *)0x0) {
          uVar16 = puVar14 == (undefined *)0x23;
          if ((undefined *)0x22 < puVar14) goto LAB_14031ebf5;
          uVar16 = puVar13[1] == -0x41;
          if ((char)puVar13[1] < -0x40) {
            do {
              FUN_1428d9940(&UNK_1429bc740,0x23);
LAB_14031ebf5:
            } while (!(bool)uVar16);
          }
        }
        lVar7 = 0x23 - (longlong)puVar14;
        puVar14 = &UNK_1429bc740 + (longlong)puVar14;
        goto LAB_14031ec03;
      }
      puVar13 = puVar13 + -1;
      if (uVar6 == 0x5c) goto LAB_14031ebb6;
LAB_14031eba4:
      if (uVar6 == 0x2f) goto LAB_14031ebb6;
    } while (puVar13 != &UNK_1429bc740);
    lVar7 = 0x23;
LAB_14031ec03:
    plVar8 = (longlong *)func_0x000140c18130();
    if (*plVar8 == 1) {
      plVar8 = plVar8 + 1;
    }
    else {
      if (*plVar8 != 0) goto LAB_14031eda9;
      plVar8 = (longlong *)FUN_1428a1640(plVar8,0);
      if (plVar8 == (longlong *)0x0) goto LAB_14031eda9;
    }
    if (*plVar8 != 0) goto LAB_14031edb5;
    *plVar8 = -1;
    uStack_48 = FUN_140c16d40(plVar8 + 1,puVar10,lVar11,puVar14,lVar7,1,0);
    *plVar8 = *plVar8 + 1;
    lStack_50 = 1;
  }
  LOCK();
  bVar17 = DAT_14362b050 == 0;
  if (bVar17) {
    DAT_14362b050 = 1;
  }
  UNLOCK();
  if (!bVar17) goto LAB_14031edc1;
  do {
    FUN_140c81c70(&DAT_14362b050);
    lVar11 = _DAT_14362b068 * 0x98 + _DAT_14362b060;
    lVar7 = _DAT_14362b060;
    while (lVar15 = lVar7, lVar15 != lVar11) {
      lVar7 = lVar15 + 0x98;
      if ((*(char *)(lVar15 + 0x96) != '\x06') && (*(int *)(lVar15 + 0x50) != 2)) {
        if (*(int *)(lVar15 + 0x50) == 1) {
          FUN_140bf8980(param_1);
          FUN_14031f2e0(lVar15);
          *(undefined4 *)(lVar15 + 0x50) = 2;
        }
        else if (*(int *)(lVar15 + 0x54) == 0) {
          FUN_14031f2e0(lVar15);
          *(undefined4 *)(lVar15 + 0x50) = 2;
        }
        else {
          *(int *)(lVar15 + 0x54) = *(int *)(lVar15 + 0x54) + -1;
        }
      }
    }
    FUN_1402b5c00(0x14362b058);
    FUN_140c81e10();
    plVar8 = (longlong *)0x1;
    LOCK();
    bVar17 = DAT_14362b050 == 1;
    if (bVar17) {
      DAT_14362b050 = 0;
    }
    else {
      plVar8 = (longlong *)(ulonglong)DAT_14362b050;
    }
    UNLOCK();
    if (!bVar17) {
      plVar8 = (longlong *)FUN_1428cdb90(&DAT_14362b050,0);
    }
    if (lStack_50 == 0) {
      return plVar8;
    }
    plVar8 = (longlong *)func_0x000140c18130();
    if (*plVar8 == 1) {
      plVar8 = plVar8 + 1;
LAB_14031ed49:
      if (*plVar8 == 0) {
        *plVar8 = -1;
        FUN_140c170f0(plVar8 + 1,uStack_48);
        *plVar8 = *plVar8 + 1;
        return plVar8;
      }
    }
    else {
      if ((*plVar8 == 0) && (plVar8 = (longlong *)FUN_1428a1640(), plVar8 != (longlong *)0x0))
      goto LAB_14031ed49;
LAB_14031eda9:
      FUN_1428d80d0(&UNK_142932a10);
    }
LAB_14031edb5:
    FUN_1428d9250(&UNK_142932ab8);
LAB_14031edc1:
    FUN_1428cd210(&DAT_14362b050);
  } while( true );
}

