
/* WARNING: Possible PIC construction at 0x0001428cdc1e: Changing call to branch */
/* WARNING: Possible PIC construction at 0x0001428cde1a: Changing call to branch */
/* WARNING: Removing unreachable block (ram,0x0001428cdc23) */
/* WARNING: Removing unreachable block (ram,0x0001428cde1f) */
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void FUN_14031eed0(undefined8 *param_1)

{
  longlong *plVar1;
  ulonglong *puVar2;
  code *pcVar3;
  undefined *puVar4;
  longlong *plVar5;
  longlong lVar6;
  ulonglong uVar7;
  longlong lVar8;
  uint uVar9;
  ulonglong uVar10;
  longlong lVar11;
  longlong *plVar13;
  uint uVar14;
  ulonglong uVar15;
  ulonglong uVar16;
  ulonglong uVar17;
  ulonglong uVar18;
  bool bVar19;
  bool bVar20;
  char cVar21;
  longlong lStack_78;
  undefined4 uStack_70;
  undefined4 uStack_6c;
  undefined4 uStack_68;
  undefined4 uStack_64;
  longlong *plStack_60;
  longlong *plStack_58;
  undefined *puStack_50;
  longlong lStack_48;
  longlong lStack_40;
  longlong *plStack_38;
  undefined *puStack_30;
  undefined8 uStack_28;
  ulonglong *puVar12;
  
  uStack_28 = 0xfffffffffffffffe;
  puVar2 = (ulonglong *)*param_1;
  lVar6 = param_1[1];
  uVar16 = *puVar2;
  uVar7 = 0;
  uVar15 = uVar16 - *(ulonglong *)(lVar6 + 0x18);
  if (uVar16 < *(ulonglong *)(lVar6 + 0x18)) {
    uVar15 = uVar7;
  }
  uVar10 = uVar16 - *(ulonglong *)(lVar6 + 0x38);
  if (uVar16 < *(ulonglong *)(lVar6 + 0x38)) {
    uVar10 = uVar7;
  }
  uVar16 = *(ulonglong *)(lVar6 + 0x30);
  uVar17 = uVar16 - uVar10;
  if (uVar16 < uVar10) {
    uVar17 = uVar7;
  }
  bVar20 = uVar16 < uVar10 || uVar16 - uVar10 == 0;
  uVar16 = *(ulonglong *)(lVar6 + 0x10);
  uVar18 = uVar16 - uVar15;
  if (uVar16 < uVar15) {
    uVar18 = uVar7;
  }
  uVar7 = *(ulonglong *)(lVar6 + 0x40);
  *puVar2 = uVar7 - (uVar18 + uVar17);
  if ((uVar16 < uVar15 || uVar16 - uVar15 == 0) && bVar20) {
    return;
  }
  *puVar2 = uVar7;
  LOCK();
  bVar19 = DAT_14362b050 == '\0';
  if (bVar19) {
    DAT_14362b050 = '\x01';
  }
  UNLOCK();
  if (!bVar19) {
    FUN_1428cd210(&DAT_14362b050,CONCAT71((int7)(uVar10 >> 8),bVar20),1000000000);
  }
  FUN_140c81c70(&DAT_14362b050);
  lVar6 = _DAT_14362b060;
  if (_DAT_14362b068 != 0) {
    lVar8 = _DAT_14362b068 * 0x98;
    lVar11 = 0;
    do {
      *(undefined4 *)(lVar6 + 0x50 + lVar11) = 1;
      lVar11 = lVar11 + 0x98;
    } while (lVar8 - lVar11 != 0);
  }
  FUN_140c81e10();
  LOCK();
  bVar20 = DAT_14362b050 == '\x01';
  if (bVar20) {
    DAT_14362b050 = '\0';
  }
  UNLOCK();
  if (bVar20) {
    if (_DAT_14362b070 != 2) goto LAB_14031f17b;
LAB_14031efc3:
    LOCK();
    bVar20 = DAT_14362b080 == '\0';
    if (bVar20) {
      DAT_14362b080 = '\x01';
    }
    UNLOCK();
  }
  else {
    FUN_1428cdb90(&DAT_14362b050,0);
    if (_DAT_14362b070 == 2) goto LAB_14031efc3;
LAB_14031f17b:
    FUN_1428a45f0(&DAT_14362b070,&DAT_14362b070);
    LOCK();
    bVar20 = DAT_14362b080 == '\0';
    if (bVar20) {
      DAT_14362b080 = '\x01';
    }
    UNLOCK();
  }
  if (!bVar20) {
    FUN_1428cd210(&DAT_14362b080);
  }
  FUN_140c81c70(&DAT_14362b080);
  lStack_48 = _DAT_14362b098;
  _DAT_14362b098 = 0;
  plStack_58 = _DAT_14362b090 + lStack_48 * 3;
  puStack_50 = &DAT_14362b088;
  lStack_40 = 0;
  plVar13 = _DAT_14362b090;
  while (plVar5 = plVar13, plVar13 = plStack_58, lVar6 = lStack_40, puVar4 = puStack_50,
        plVar5 != plStack_58) {
    if (*plVar5 == 0) {
      plStack_60 = (longlong *)0x8;
      plStack_58 = (longlong *)0x8;
      if (plVar13 != plVar5 + 3) {
        puStack_30 = puStack_50;
        FUN_1405fc480();
        lVar6 = lStack_40;
        puVar4 = puStack_30;
      }
      break;
    }
    uStack_70 = (undefined4)plVar5[1];
    uStack_6c = *(undefined4 *)((longlong)plVar5 + 0xc);
    uStack_68 = (undefined4)plVar5[2];
    uStack_64 = *(undefined4 *)((longlong)plVar5 + 0x14);
    lStack_78 = *plVar5;
    plStack_60 = plVar5 + 3;
    puVar4 = (undefined *)FUN_140288020(&lStack_78);
    plVar13 = plStack_60;
    if (puVar4 != (undefined *)0x0) {
      plStack_38 = plVar5;
      puStack_30 = puVar4;
      if ((code *)*plVar5 != (code *)0x0) {
        (*(code *)*plVar5)(puVar4);
      }
      plVar13 = plStack_60;
      if (plStack_38[1] != 0) {
        func_0x000140613c20(puStack_30,plStack_38[1],plStack_38[2]);
        plVar13 = plStack_60;
      }
    }
  }
  lStack_40 = lVar6;
  if (lVar6 != 0) {
    lVar8 = *(longlong *)(puVar4 + 0x10);
    if (lStack_48 != lVar8) {
      func_0x000142923c10(*(longlong *)(puVar4 + 8) + lVar8 * 0x18,
                          *(longlong *)(puVar4 + 8) + lStack_48 * 0x18,lVar6 * 0x18);
    }
    *(longlong *)(puVar4 + 0x10) = lVar8 + lVar6;
  }
  FUN_140c81e10();
  plVar13 = plRam00000001436308f0;
  LOCK();
  bVar20 = DAT_14362b080 == '\x01';
  if (bVar20) {
    DAT_14362b080 = '\0';
  }
  UNLOCK();
  if (bVar20) {
    return;
  }
  do {
    plVar5 = (longlong *)*plVar13;
    if (plVar5 == (longlong *)0x0) {
      plStack_60 = (longlong *)&UNK_1428cdc34;
      plVar5 = (longlong *)FUN_14290b400();
    }
    uVar16 = 0xe40a1954d9967a80 >> (-(char)plVar5[3] & 0x3fU);
    if ((ulonglong)plVar5[1] <= uVar16) {
      plStack_60 = (longlong *)&UNK_1428cde4b;
      FUN_1428d9518(uVar16,plVar5[1],&UNK_142b1fac0);
      pcVar3 = (code *)swi(3);
      (*pcVar3)();
      return;
    }
    puVar2 = (ulonglong *)(*plVar5 + uVar16 * 0x40);
    plVar1 = (longlong *)(*plVar5 + uVar16 * 0x40);
    LOCK();
    bVar20 = *plVar1 == 0;
    if (bVar20) {
      *plVar1 = 1;
    }
    UNLOCK();
    if (!bVar20) {
      plStack_60 = (longlong *)&UNK_1428cdc2d;
      FUN_14290b740(puVar2);
    }
    if ((longlong *)*plVar13 == plVar5) {
      plVar13 = (longlong *)puVar2[1];
      if (plVar13 == (longlong *)0x0) goto code_r0x0001428cdca8;
      if ((undefined *)plVar13[2] != &DAT_14362b080) goto code_r0x0001428cdc60;
      puVar12 = puVar2 + 1;
      plVar5 = (longlong *)0x0;
      goto code_r0x0001428cdc79;
    }
    LOCK();
    uVar16 = *puVar2;
    *puVar2 = *puVar2 - 1;
    UNLOCK();
  } while (uVar16 < 4 || (char)(((uint)uVar16 & 2) >> 1) != '\0');
  plStack_60 = (longlong *)&UNK_1428cdc23;
  goto code_r0x00014290b8e0;
code_r0x0001428cdca8:
  DAT_14362b080 = '\0';
  LOCK();
  uVar16 = *puVar2;
  *puVar2 = *puVar2 - 1;
  UNLOCK();
  if (uVar16 < 4 || (char)(((uint)uVar16 & 2) >> 1) != '\0') {
    return;
  }
  goto code_r0x00014290b8e0;
  while ((undefined *)plVar13[2] != &DAT_14362b080) {
code_r0x0001428cdc60:
    plVar5 = plVar13;
    plVar13 = (longlong *)plVar5[3];
    if (plVar13 == (longlong *)0x0) goto code_r0x0001428cdca8;
  }
  puVar12 = (ulonglong *)(plVar5 + 3);
code_r0x0001428cdc79:
  uVar16 = plVar13[3];
  *puVar12 = uVar16;
  if ((longlong *)puVar2[2] == plVar13) {
    puVar2[2] = (ulonglong)plVar5;
code_r0x0001428cdce0:
    bVar20 = false;
  }
  else {
    if (uVar16 == 0) goto code_r0x0001428cdce0;
    do {
      bVar20 = *(undefined **)(uVar16 + 0x10) == &DAT_14362b080;
      if (bVar20) break;
      uVar16 = *(ulonglong *)(uVar16 + 0x18);
    } while (uVar16 != 0);
  }
  plStack_60 = (longlong *)&UNK_1428cdce8;
  uVar16 = FUN_140d635e0();
  uVar9 = (uint)puVar12;
  cVar21 = -1;
  if (puVar2[3] <= uVar16) {
    cVar21 = uVar16 != puVar2[3];
  }
  if (((cVar21 == '\0') && (cVar21 = uVar9 != (uint)puVar2[4], uVar9 < (uint)puVar2[4])) ||
     (cVar21 != '\x01')) {
    if (bVar20) {
      DAT_14362b080 = '\x02';
    }
    else {
      DAT_14362b080 = '\0';
    }
    lVar6 = 0;
  }
  else {
    uVar14 = (uint)puVar2[5] << 0xd ^ (uint)puVar2[5];
    uVar14 = uVar14 >> 0x11 ^ uVar14;
    *(uint *)(puVar2 + 5) = uVar14 << 5 ^ uVar14;
    plStack_60 = (longlong *)&UNK_1428cdd50;
    uVar16 = FUN_140d63a20();
    puVar2[3] = uVar16;
    *(uint *)(puVar2 + 4) = uVar9;
    lVar6 = 1;
    if (!bVar20) {
      DAT_14362b080 = '\x01';
    }
  }
  plVar13[4] = lVar6;
  plVar5 = (longlong *)*plVar13;
  lVar6 = *plVar5;
  if (lVar6 == 0) {
    plVar5 = plVar5 + 1;
    plVar13[1] = 0;
    plVar13 = plVar13 + 1;
  }
  else {
    LOCK();
    lVar8 = plVar13[1];
    plVar13[1] = 0;
    UNLOCK();
    plVar13 = plVar13 + 1;
    if (lVar8 != 1) {
      plVar13 = (longlong *)0x0;
    }
  }
  LOCK();
  uVar16 = *puVar2;
  *puVar2 = *puVar2 - 1;
  UNLOCK();
  if (uVar16 < 4 || (char)(((uint)uVar16 & 2) >> 1) != '\0') {
    if (lVar6 == 0) {
                    /* WARNING: Could not recover jumptable at 0x0001428cde38. Too many branches */
                    /* WARNING: Treating indirect jump as call */
      (*(code *)plVar5[1])(plVar13);
      return;
    }
    if (plVar13 == (longlong *)0x0) {
      return;
    }
                    /* WARNING: Could not recover jumptable at 0x0001428cddfb. Too many branches */
                    /* WARNING: Treating indirect jump as call */
    (*(code *)*plVar5)(plVar5[2],plVar13,0,0);
    return;
  }
  plStack_60 = (longlong *)&UNK_1428cde1f;
code_r0x00014290b8e0:
  uVar16 = *puVar2;
  do {
    if (uVar16 < 4) {
      return;
    }
    if ((uVar16 & 2) != 0) {
      return;
    }
    LOCK();
    uVar15 = *puVar2;
    bVar20 = uVar16 == uVar15;
    if (bVar20) {
      *puVar2 = uVar16 | 2;
      uVar15 = uVar16;
    }
    UNLOCK();
    uVar7 = uVar16;
    uVar16 = uVar15;
  } while (!bVar20);
  while( true ) {
    while( true ) {
      uVar15 = uVar7 & 0xfffffffffffffffc;
      plVar13 = *(longlong **)(uVar15 + 0x10);
      uVar16 = uVar15;
      while (plVar13 == (longlong *)0x0) {
        uVar10 = *(ulonglong *)(uVar16 + 0x20);
        *(ulonglong *)(uVar10 + 0x18) = uVar16;
        uVar16 = uVar10;
        plVar13 = *(longlong **)(uVar10 + 0x10);
      }
      *(longlong **)(uVar15 + 0x10) = plVar13;
      if ((uVar7 & 1) == 0) break;
      LOCK();
      uVar16 = *puVar2;
      bVar20 = uVar7 == uVar16;
      if (bVar20) {
        *puVar2 = uVar7 & 0xfffffffffffffffd;
        uVar16 = uVar7;
      }
      UNLOCK();
      uVar7 = uVar16;
      if (bVar20) {
        return;
      }
    }
    if (plVar13[3] != 0) break;
    do {
      LOCK();
      uVar16 = *puVar2;
      bVar20 = uVar7 == uVar16;
      if (bVar20) {
        *puVar2 = (ulonglong)((uint)uVar7 & 1);
        uVar16 = uVar7;
      }
      UNLOCK();
      if (bVar20) goto code_r0x00014290b999;
      uVar7 = uVar16;
    } while (uVar16 < 4);
  }
  *(longlong *)(uVar15 + 0x10) = plVar13[3];
  LOCK();
  *puVar2 = *puVar2 & 0xfffffffffffffffd;
  UNLOCK();
code_r0x00014290b999:
  plVar5 = (longlong *)*plVar13;
  plVar13 = plVar13 + 1;
  if (*plVar5 == 0) {
    *plVar13 = 0;
                    /* WARNING: Could not recover jumptable at 0x00014290b9cc. Too many branches */
                    /* WARNING: Treating indirect jump as call */
    (*(code *)plVar5[2])(plVar13);
    return;
  }
  LOCK();
  lVar6 = *plVar13;
  *plVar13 = 0;
  UNLOCK();
  if (lVar6 != 1) {
    return;
  }
                    /* WARNING: Could not recover jumptable at 0x00014290b9be. Too many branches */
                    /* WARNING: Treating indirect jump as call */
  (*(code *)*plVar5)(plVar5[2],plVar13,0,0);
  return;
}

