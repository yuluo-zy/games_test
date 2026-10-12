
/* WARNING: Possible PIC construction at 0x0001428cdc1e: Changing call to branch */
/* WARNING: Possible PIC construction at 0x0001428cde1a: Changing call to branch */
/* WARNING: Removing unreachable block (ram,0x0001428cdc23) */
/* WARNING: Removing unreachable block (ram,0x0001428cde1f) */

void FUN_140bf8e60(longlong param_1)

{
  char *pcVar1;
  ulonglong *puVar2;
  longlong *plVar3;
  ulonglong uVar4;
  code *pcVar5;
  longlong *plVar6;
  ulonglong uVar7;
  uint uVar8;
  longlong lVar10;
  longlong lVar11;
  longlong *plVar12;
  uint uVar13;
  ulonglong uVar14;
  ulonglong uVar15;
  bool bVar16;
  char cVar17;
  ulonglong *puVar9;
  
  FUN_140bf8980();
  FUN_1418c0a70(*(undefined8 *)(param_1 + 0x648),param_1);
  pcVar1 = (char *)(param_1 + 0x718);
  LOCK();
  bVar16 = *(char *)(param_1 + 0x718) == '\0';
  if (bVar16) {
    *(char *)(param_1 + 0x718) = '\x01';
  }
  UNLOCK();
  if (!bVar16) {
    FUN_1428cd210(pcVar1);
  }
  FUN_140c81c70(pcVar1);
  FUN_1418c3300(param_1 + 0x720,param_1 + 0x78);
  FUN_140bf0cc0(param_1 + 0x730);
  if (*(longlong *)(param_1 + 0x7f0) != 0) {
    lVar11 = *(longlong *)(param_1 + 0x7e8);
    lVar10 = *(longlong *)(param_1 + 0x7f0) * 0xb0 + lVar11;
    do {
      FUN_140bf0cc0(lVar11);
      lVar11 = lVar11 + 0xb0;
    } while (lVar11 != lVar10);
  }
  FUN_140c81e10();
  LOCK();
  cVar17 = *pcVar1;
  if (cVar17 == '\x01') {
    *pcVar1 = '\0';
  }
  plVar12 = plRam00000001436308f0;
  UNLOCK();
  if (cVar17 == '\x01') {
    return;
  }
  do {
    plVar6 = (longlong *)*plVar12;
    if (plVar6 == (longlong *)0x0) {
      plVar6 = (longlong *)FUN_14290b400();
    }
    uVar15 = (ulonglong)((longlong)pcVar1 * -0x61c8864680b583eb) >> (-(char)plVar6[3] & 0x3fU);
    if ((ulonglong)plVar6[1] <= uVar15) {
      FUN_1428d9518(uVar15,plVar6[1],&UNK_142b1fac0);
      pcVar5 = (code *)swi(3);
      (*pcVar5)();
      return;
    }
    puVar2 = (ulonglong *)(*plVar6 + uVar15 * 0x40);
    plVar3 = (longlong *)(*plVar6 + uVar15 * 0x40);
    LOCK();
    bVar16 = *plVar3 == 0;
    if (bVar16) {
      *plVar3 = 1;
    }
    UNLOCK();
    if (!bVar16) {
      FUN_14290b740(puVar2);
    }
    if ((longlong *)*plVar12 == plVar6) {
      plVar12 = (longlong *)puVar2[1];
      if (plVar12 == (longlong *)0x0) goto code_r0x0001428cdca8;
      if ((char *)plVar12[2] != pcVar1) goto code_r0x0001428cdc60;
      puVar9 = puVar2 + 1;
      plVar6 = (longlong *)0x0;
      goto code_r0x0001428cdc79;
    }
    LOCK();
    uVar15 = *puVar2;
    *puVar2 = *puVar2 - 1;
    UNLOCK();
  } while (uVar15 < 4 || (char)(((uint)uVar15 & 2) >> 1) != '\0');
  goto code_r0x00014290b8e0;
code_r0x0001428cdca8:
  *pcVar1 = '\0';
  LOCK();
  uVar15 = *puVar2;
  *puVar2 = *puVar2 - 1;
  UNLOCK();
  if (uVar15 < 4 || (char)(((uint)uVar15 & 2) >> 1) != '\0') {
    return;
  }
  goto code_r0x00014290b8e0;
  while ((char *)plVar12[2] != pcVar1) {
code_r0x0001428cdc60:
    plVar6 = plVar12;
    plVar12 = (longlong *)plVar6[3];
    if (plVar12 == (longlong *)0x0) goto code_r0x0001428cdca8;
  }
  puVar9 = (ulonglong *)(plVar6 + 3);
code_r0x0001428cdc79:
  uVar15 = plVar12[3];
  *puVar9 = uVar15;
  if ((longlong *)puVar2[2] == plVar12) {
    puVar2[2] = (ulonglong)plVar6;
code_r0x0001428cdce0:
    bVar16 = false;
  }
  else {
    if (uVar15 == 0) goto code_r0x0001428cdce0;
    do {
      bVar16 = *(char **)(uVar15 + 0x10) == pcVar1;
      if (bVar16) break;
      uVar15 = *(ulonglong *)(uVar15 + 0x18);
    } while (uVar15 != 0);
  }
  uVar15 = FUN_140d635e0();
  uVar8 = (uint)puVar9;
  cVar17 = -1;
  if (puVar2[3] <= uVar15) {
    cVar17 = uVar15 != puVar2[3];
  }
  if (((cVar17 == '\0') && (cVar17 = uVar8 != (uint)puVar2[4], uVar8 < (uint)puVar2[4])) ||
     (cVar17 != '\x01')) {
    if (bVar16) {
      *pcVar1 = '\x02';
    }
    else {
      *pcVar1 = '\0';
    }
    lVar11 = 0;
  }
  else {
    uVar13 = (uint)puVar2[5] << 0xd ^ (uint)puVar2[5];
    uVar13 = uVar13 >> 0x11 ^ uVar13;
    *(uint *)(puVar2 + 5) = uVar13 << 5 ^ uVar13;
    uVar15 = FUN_140d63a20();
    puVar2[3] = uVar15;
    *(uint *)(puVar2 + 4) = uVar8;
    lVar11 = 1;
    if (!bVar16) {
      *pcVar1 = '\x01';
    }
  }
  plVar12[4] = lVar11;
  plVar6 = (longlong *)*plVar12;
  lVar11 = *plVar6;
  if (lVar11 == 0) {
    plVar6 = plVar6 + 1;
    plVar12[1] = 0;
    plVar12 = plVar12 + 1;
  }
  else {
    LOCK();
    lVar10 = plVar12[1];
    plVar12[1] = 0;
    UNLOCK();
    plVar12 = plVar12 + 1;
    if (lVar10 != 1) {
      plVar12 = (longlong *)0x0;
    }
  }
  LOCK();
  uVar15 = *puVar2;
  *puVar2 = *puVar2 - 1;
  UNLOCK();
  if (uVar15 < 4 || (char)(((uint)uVar15 & 2) >> 1) != '\0') {
    if (lVar11 == 0) {
                    /* WARNING: Could not recover jumptable at 0x0001428cde38. Too many branches */
                    /* WARNING: Treating indirect jump as call */
      (*(code *)plVar6[1])(plVar12);
      return;
    }
    if (plVar12 == (longlong *)0x0) {
      return;
    }
                    /* WARNING: Could not recover jumptable at 0x0001428cddfb. Too many branches */
                    /* WARNING: Treating indirect jump as call */
    (*(code *)*plVar6)(plVar6[2],plVar12,0,0);
    return;
  }
code_r0x00014290b8e0:
  uVar15 = *puVar2;
  do {
    if (uVar15 < 4) {
      return;
    }
    if ((uVar15 & 2) != 0) {
      return;
    }
    LOCK();
    uVar14 = *puVar2;
    bVar16 = uVar15 == uVar14;
    if (bVar16) {
      *puVar2 = uVar15 | 2;
      uVar14 = uVar15;
    }
    UNLOCK();
    uVar7 = uVar15;
    uVar15 = uVar14;
  } while (!bVar16);
  while( true ) {
    while( true ) {
      uVar14 = uVar7 & 0xfffffffffffffffc;
      plVar12 = *(longlong **)(uVar14 + 0x10);
      uVar15 = uVar14;
      while (plVar12 == (longlong *)0x0) {
        uVar4 = *(ulonglong *)(uVar15 + 0x20);
        *(ulonglong *)(uVar4 + 0x18) = uVar15;
        uVar15 = uVar4;
        plVar12 = *(longlong **)(uVar4 + 0x10);
      }
      *(longlong **)(uVar14 + 0x10) = plVar12;
      if ((uVar7 & 1) == 0) break;
      LOCK();
      uVar15 = *puVar2;
      bVar16 = uVar7 == uVar15;
      if (bVar16) {
        *puVar2 = uVar7 & 0xfffffffffffffffd;
        uVar15 = uVar7;
      }
      UNLOCK();
      uVar7 = uVar15;
      if (bVar16) {
        return;
      }
    }
    if (plVar12[3] != 0) break;
    do {
      LOCK();
      uVar15 = *puVar2;
      bVar16 = uVar7 == uVar15;
      if (bVar16) {
        *puVar2 = (ulonglong)((uint)uVar7 & 1);
        uVar15 = uVar7;
      }
      UNLOCK();
      if (bVar16) goto code_r0x00014290b999;
      uVar7 = uVar15;
    } while (uVar15 < 4);
  }
  *(longlong *)(uVar14 + 0x10) = plVar12[3];
  LOCK();
  *puVar2 = *puVar2 & 0xfffffffffffffffd;
  UNLOCK();
code_r0x00014290b999:
  plVar6 = (longlong *)*plVar12;
  plVar12 = plVar12 + 1;
  if (*plVar6 == 0) {
    *plVar12 = 0;
                    /* WARNING: Could not recover jumptable at 0x00014290b9cc. Too many branches */
                    /* WARNING: Treating indirect jump as call */
    (*(code *)plVar6[2])(plVar12);
    return;
  }
  LOCK();
  lVar11 = *plVar12;
  *plVar12 = 0;
  UNLOCK();
  if (lVar11 != 1) {
    return;
  }
                    /* WARNING: Could not recover jumptable at 0x00014290b9be. Too many branches */
                    /* WARNING: Treating indirect jump as call */
  (*(code *)*plVar6)(plVar6[2],plVar12,0,0);
  return;
}

