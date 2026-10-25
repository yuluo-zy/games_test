
/* WARNING: Control flow encountered bad instruction data */
/* WARNING: Exceeded maximum restarts with more pending */

void FUN_1418c0a70(char *param_1,ulonglong param_2)

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
  longlong lVar10;
  ulonglong uVar11;
  undefined *puVar12;
  undefined *puVar13;
  undefined *puVar14;
  undefined1 uVar15;
  undefined8 uStack_180;
  undefined8 uStack_178;
  undefined8 uStack_170;
  undefined8 uStack_168;
  undefined8 uStack_160;
  undefined8 uStack_158;
  undefined8 uStack_150;
  undefined8 uStack_148;
  undefined8 uStack_140;
  undefined8 uStack_138;
  undefined8 uStack_130;
  undefined8 uStack_128;
  undefined8 uStack_120;
  undefined8 uStack_118;
  undefined8 uStack_110;
  undefined8 uStack_108;
  undefined8 uStack_100;
  undefined8 uStack_f8;
  undefined8 uStack_f0;
  undefined8 uStack_e8;
  undefined8 uStack_e0;
  undefined8 uStack_d8;
  undefined8 uStack_d0;
  undefined8 uStack_c8;
  undefined8 uStack_c0;
  undefined8 uStack_b8;
  undefined8 uStack_b0;
  undefined8 uStack_a8;
  undefined8 uStack_a0;
  undefined8 uStack_98;
  longlong lStack_80;
  undefined8 uStack_78;
  longlong lStack_60;
  longlong *plStack_58;
  undefined1 uStack_49;
  undefined8 uStack_48;
  
  uStack_48 = 0xfffffffffffffffe;
  uVar9 = param_2;
  cVar5 = func_0x000140c16a00();
  if (cVar5 == '\0') {
code_r0x0001418c0b01:
    lStack_80 = 0;
  }
  else {
    lVar10 = 0x4b;
    lVar7 = FUN_1418c04b0(0x4b);
    puVar13 = &UNK_142d05f38;
    if (lVar7 == 1) {
      if (uVar9 != 0) {
        if (uVar9 < 0x4b) {
          if ((char)(&UNK_142d05f38)[uVar9] < -0x40) {
code_r0x0001418c0ae1:
            FUN_1428d9940(&UNK_142d05f38,0x4b,0,uVar9,&UNK_142d06008);
            goto code_r0x0001418c0b01;
          }
        }
        else if (uVar9 != 0x4b) goto code_r0x0001418c0ae1;
      }
      lVar7 = FUN_1418c04b0(uVar9);
      if (lVar7 == 1) {
        uVar11 = uVar9 + 2;
        if (uVar11 != 0) {
          uVar15 = uVar11 == 0x4b;
          if (0x4a < uVar11) goto code_r0x0001418c0b58;
          uVar15 = (&UNK_142d05f3a)[uVar9] == -0x41;
          if ((char)(&UNK_142d05f3a)[uVar9] < -0x40) {
            do {
              uVar9 = 0x4b;
              FUN_1428d9940(&UNK_142d05f38,0x4b);
code_r0x0001418c0b58:
            } while (!(bool)uVar15);
          }
        }
        lVar10 = 0x49 - uVar9;
        puVar13 = &UNK_142d05f38 + uVar11;
      }
    }
    puVar12 = &UNK_142d060c4;
    puVar14 = &UNK_142d06098;
    do {
      bVar2 = puVar12[-1];
      uVar6 = (uint)(char)bVar2;
      if ((char)bVar2 < '\0') {
        bVar3 = puVar12[-2];
        if ((char)bVar3 < -0x40) {
          bVar4 = puVar12[-3];
          if ((char)bVar4 < -0x40) {
            pbVar1 = puVar12 + -4;
            puVar12 = puVar12 + -4;
            uVar6 = bVar4 & 0x3f | (*pbVar1 & 7) << 6;
          }
          else {
            puVar12 = puVar12 + -3;
            uVar6 = bVar4 & 0xf;
          }
          uVar6 = bVar3 & 0x3f | uVar6 << 6;
        }
        else {
          puVar12 = puVar12 + -2;
          uVar6 = bVar3 & 0x1f;
        }
        uVar6 = (uint)(bVar2 & 0x3f) | uVar6 << 6;
        if (uVar6 != 0x5c) goto code_r0x0001418c0bf4;
code_r0x0001418c0c06:
        puVar14 = puVar12 + -0x142d06097;
        if (puVar14 != (undefined *)0x0) {
          uVar15 = puVar14 == (undefined *)0x2c;
          if ((undefined *)0x2b < puVar14) goto code_r0x0001418c0c45;
          uVar15 = puVar12[1] == -0x41;
          if ((char)puVar12[1] < -0x40) {
            do {
              FUN_1428d9940(&UNK_142d06098,0x2c);
code_r0x0001418c0c45:
            } while (!(bool)uVar15);
          }
        }
        lVar7 = 0x2c - (longlong)puVar14;
        puVar14 = &UNK_142d06098 + (longlong)puVar14;
        goto code_r0x0001418c0c53;
      }
      puVar12 = puVar12 + -1;
      if (uVar6 == 0x5c) goto code_r0x0001418c0c06;
code_r0x0001418c0bf4:
      if (uVar6 == 0x2f) goto code_r0x0001418c0c06;
    } while (puVar12 != &UNK_142d06098);
    lVar7 = 0x2c;
code_r0x0001418c0c53:
    plVar8 = (longlong *)func_0x000140c18130();
    if (*plVar8 == 1) {
      plVar8 = plVar8 + 1;
    }
    else {
      if (*plVar8 != 0) goto code_r0x0001418c10b1;
      plVar8 = (longlong *)FUN_1429090d0(plVar8,0);
      if (plVar8 == (longlong *)0x0) goto code_r0x0001418c10b1;
    }
    if (*plVar8 != 0) goto code_r0x0001418c10bd;
    *plVar8 = -1;
    plStack_58 = plVar8;
    uStack_78 = FUN_140c16d40(plVar8 + 1,puVar13,lVar10,puVar14,lVar7,1,0);
    *plStack_58 = *plStack_58 + 1;
    lStack_80 = 1;
  }
  uStack_180 = 0;
  uStack_178 = 8;
  uStack_170 = 0;
  uStack_168 = 0;
  uStack_160 = 8;
  uStack_158 = 0;
  uStack_150 = 0;
  uStack_148 = 8;
  uStack_140 = 0;
  uStack_138 = 0;
  uStack_130 = 8;
  uStack_128 = 0;
  uStack_120 = 0;
  uStack_118 = 8;
  uStack_110 = 0;
  uStack_108 = 0;
  uStack_100 = 8;
  uStack_f8 = 0;
  uStack_f0 = 0;
  uStack_e8 = 8;
  uStack_e0 = 0;
  uStack_d8 = 0;
  uStack_d0 = 8;
  uStack_c8 = 0;
  uStack_c0 = 0;
  uStack_b8 = 8;
  uStack_b0 = 0;
  uStack_a8 = 0;
  uStack_a0 = 8;
  uStack_98 = 0;
  param_1 = *(char **)param_1;
  LOCK();
  cVar5 = *param_1;
  if (cVar5 == '\0') {
    *param_1 = '\x01';
  }
  UNLOCK();
  if (cVar5 != '\0') goto code_r0x0001418c10c9;
  do {
    FUN_140c81c70(param_1);
    lStack_60 = *(longlong *)(param_1 + 8);
    plStack_58 = *(longlong **)(param_1 + 0x10);
    lVar7 = *(longlong *)(param_1 + 0x18);
    param_1[8] = '\0';
    param_1[9] = '\0';
    param_1[10] = '\0';
    param_1[0xb] = '\0';
    param_1[0xc] = '\0';
    param_1[0xd] = '\0';
    param_1[0xe] = '\0';
    param_1[0xf] = '\0';
    param_1[0x10] = '\b';
    param_1[0x11] = '\0';
    param_1[0x12] = '\0';
    param_1[0x13] = '\0';
    param_1[0x14] = '\0';
    param_1[0x15] = '\0';
    param_1[0x16] = '\0';
    param_1[0x17] = '\0';
    param_1[0x18] = '\0';
    param_1[0x19] = '\0';
    param_1[0x1a] = '\0';
    param_1[0x1b] = '\0';
    param_1[0x1c] = '\0';
    param_1[0x1d] = '\0';
    param_1[0x1e] = '\0';
    param_1[0x1f] = '\0';
    uStack_49 = 1;
    FUN_140c81e10();
    LOCK();
    cVar5 = *param_1;
    if (cVar5 == '\x01') {
      *param_1 = '\0';
    }
    UNLOCK();
    if (cVar5 != '\x01') {
      uStack_49 = 1;
      FUN_1428cdb90(param_1,0);
    }
    param_1 = (char *)(param_2 + 0xa50);
    if (plStack_58 != plStack_58 + lVar7 * 2) {
                    /* WARNING (jumptable): Sanity check requires truncation of jumptable */
      switch(*(undefined1 *)((longlong)plStack_58 + 0xe)) {
      case 9:
                    /* WARNING: Bad instruction - Truncating control flow here */
        halt_baddata();
      }
    }
    if (lStack_60 != 0) {
      func_0x000140613c20(plStack_58,lStack_60 << 4,8);
    }
    uStack_49 = 0;
    FUN_140bf92f0(param_2);
    FUN_140bdfcf0(&uStack_180);
    if (lStack_80 == 0) {
      return;
    }
    plVar8 = (longlong *)func_0x000140c18130();
    if (*plVar8 == 1) {
      plVar8 = plVar8 + 1;
code_r0x0001418c1041:
      if (*plVar8 == 0) {
        *plVar8 = -1;
        plStack_58 = plVar8;
        FUN_140c170f0(plVar8 + 1,uStack_78);
        *plStack_58 = *plStack_58 + 1;
        return;
      }
    }
    else {
      if ((*plVar8 == 0) && (plVar8 = (longlong *)FUN_1429090d0(), plVar8 != (longlong *)0x0))
      goto code_r0x0001418c1041;
code_r0x0001418c10b1:
      FUN_1428d80d0(&UNK_142d06498);
    }
code_r0x0001418c10bd:
    FUN_1428d9250(&UNK_142d06508);
code_r0x0001418c10c9:
    FUN_1428cd210(param_1);
  } while( true );
}

