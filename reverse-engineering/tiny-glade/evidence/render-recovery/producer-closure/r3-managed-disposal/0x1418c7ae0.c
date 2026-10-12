
void FUN_1418c7ae0(ulonglong *param_1,ulonglong param_2,longlong param_3)

{
  byte *pbVar1;
  byte bVar2;
  byte bVar3;
  byte bVar4;
  code *pcVar5;
  char cVar6;
  uint uVar7;
  longlong lVar8;
  longlong *plVar9;
  longlong *plVar10;
  ulonglong uVar11;
  ulonglong uVar12;
  undefined *puVar13;
  ulonglong uVar14;
  undefined *puVar15;
  ulonglong uVar16;
  longlong lVar17;
  undefined *puVar18;
  undefined1 uVar19;
  ulonglong uStack_188;
  undefined1 uStack_180;
  undefined8 uStack_98;
  undefined8 uStack_90;
  ulonglong *puStack_78;
  ulonglong *puStack_70;
  ulonglong *puStack_68;
  ulonglong uStack_60;
  longlong *plStack_58;
  longlong *plStack_50;
  undefined8 uStack_48;
  
  uStack_48 = 0xfffffffffffffffe;
  uVar16 = param_2;
  cVar6 = func_0x000140c16a00();
  plStack_58 = (longlong *)CONCAT71(plStack_58._1_7_,cVar6);
  if (cVar6 == '\0') {
code_r0x0001418c7b7b:
    uStack_98 = 0;
code_r0x0001418c7d4a:
    uVar16 = param_1[4];
    if (param_2 <= uVar16) {
      FUN_1428d9430(&UNK_142d06fab,0x29,&UNK_142d06fd8);
      goto code_r0x0001418c8118;
    }
    uVar11 = ~uVar16 + param_2;
    if (uVar11 == 0) {
      puStack_78 = param_1 + 4;
      uVar16 = uVar16 + 1;
      *puStack_78 = uVar16;
      uStack_180 = 0x1e;
      uStack_188 = uVar16;
      FUN_1418c1380(0x143634e40,&uStack_188,9);
      FUN_1418c4100(&uStack_188,param_3 + 0xa50,uVar16);
      FUN_140bf92f0(param_3,&uStack_188);
      FUN_140bdfcf0(&uStack_188);
      if (param_1[3] == 0) {
        uVar16 = 0;
      }
      else {
        uVar16 = 0;
        if (*param_1 <= param_1[2] + 1) {
          uVar16 = *param_1;
        }
        param_1[2] = (param_1[2] - uVar16) + 1;
        uVar16 = param_1[3] - 1;
        param_1[3] = uVar16;
      }
      while (uVar16 != 0) {
        uVar11 = param_1[2];
        uVar12 = *param_1;
        uVar14 = uVar12;
        if (uVar11 < uVar12) {
          uVar14 = 0;
        }
        if (*(longlong *)(param_1[1] + (uVar11 - uVar14) * 8) == 0) break;
        uVar14 = 0;
        if (uVar12 <= uVar11 + 1) {
          uVar14 = uVar12;
        }
        param_1[2] = (uVar11 - uVar14) + 1;
        param_1[3] = uVar16 - 1;
        uVar16 = *(ulonglong *)(param_1[1] + uVar11 * 8);
        uVar11 = param_1[4];
        param_1[4] = uVar11 + 1;
        uStack_60 = uVar16;
        if (uVar11 + 1 != uVar16) goto code_r0x0001418c8072;
        uStack_180 = 0x1e;
        uStack_188 = uVar16;
        FUN_1418c1380(0x143634e40,&uStack_188,9);
        FUN_1418c4100(&uStack_188,param_3 + 0xa50,uVar16);
        FUN_140bf92f0(param_3,&uStack_188);
        FUN_140bdfcf0(&uStack_188);
        uVar16 = param_1[3];
      }
code_r0x0001418c7eec:
      if ((char)plStack_58 == '\0') {
        return;
      }
      plVar10 = (longlong *)func_0x000140c18130();
      plVar9 = plStack_50;
      if (*plVar10 == 1) {
        plVar10 = plVar10 + 1;
      }
      else if ((*plVar10 != 0) ||
              (plVar10 = (longlong *)FUN_1429090d0(plVar10,0), plVar10 == (longlong *)0x0))
      goto code_r0x0001418c8066;
      if (*plVar10 == 0) {
        *plVar10 = -1;
        plStack_58 = plVar10;
        FUN_140c170f0(plVar10 + 1,plVar9);
        *plStack_58 = *plStack_58 + 1;
        return;
      }
      goto code_r0x0001418c80bb;
    }
    uVar16 = param_1[3];
    cVar6 = -1;
    if (uVar16 <= uVar11) {
      cVar6 = uVar11 != uVar16;
    }
    if (cVar6 == '\0') {
      uVar11 = *param_1;
      if (uVar16 == uVar11) {
        FUN_140511380(param_1);
code_r0x0001418c7e74:
        uVar11 = *param_1;
        uVar16 = param_1[3];
      }
code_r0x0001418c7e7b:
      uVar12 = 0;
      if (uVar11 <= uVar16 + param_1[2]) {
        uVar12 = uVar11;
      }
      *(ulonglong *)(param_1[1] + ((uVar16 + param_1[2]) - uVar12) * 8) = param_2;
      param_1[3] = param_1[3] + 1;
      goto code_r0x0001418c7eec;
    }
    if (cVar6 == '\x01') {
      FUN_1418c3f40(param_1,uVar11,&UNK_142d07038);
      uVar11 = *param_1;
      uVar16 = param_1[3];
      if (uVar16 == uVar11) {
        FUN_140511380(param_1);
        goto code_r0x0001418c7e74;
      }
      goto code_r0x0001418c7e7b;
    }
    if (uVar16 <= uVar11) goto code_r0x0001418c80c7;
    uVar16 = 0;
    if (*param_1 <= uVar11 + param_1[2]) {
      uVar16 = *param_1;
    }
    lVar8 = (uVar11 + param_1[2]) - uVar16;
    puStack_68 = (ulonglong *)(param_1[1] + lVar8 * 8);
    uStack_60 = 0;
    puStack_70 = &uStack_60;
    if (*(longlong *)(param_1[1] + lVar8 * 8) == 0) {
      *puStack_68 = param_2;
      goto code_r0x0001418c7eec;
    }
  }
  else {
    lVar17 = 0x3d;
    lVar8 = FUN_1418c5180(0x3d);
    puVar18 = &UNK_142d06e80;
    if (lVar8 == 1) {
      if (uVar16 != 0) {
        if (uVar16 < 0x3d) {
          if ((char)(&UNK_142d06e80)[uVar16] < -0x40) {
code_r0x0001418c7b5b:
            FUN_1428d9940(&UNK_142d06e80,0x3d,0,uVar16,&UNK_142d06f40);
            goto code_r0x0001418c7b7b;
          }
        }
        else if (uVar16 != 0x3d) goto code_r0x0001418c7b5b;
      }
      lVar8 = FUN_1418c5180(uVar16);
      if (lVar8 == 1) {
        uVar11 = uVar16 + 2;
        if (uVar11 != 0) {
          uVar19 = uVar11 == 0x3d;
          if (0x3c < uVar11) goto code_r0x0001418c7bd2;
          uVar19 = (&UNK_142d06e82)[uVar16] == -0x41;
          if ((char)(&UNK_142d06e82)[uVar16] < -0x40) {
            do {
              uVar16 = 0x3d;
              FUN_1428d9940(&UNK_142d06e80);
code_r0x0001418c7bd2:
            } while (!(bool)uVar19);
          }
        }
        lVar17 = 0x3b - uVar16;
        puVar18 = &UNK_142d06e80 + uVar11;
      }
    }
    puVar13 = &UNK_142d06fab;
    puVar15 = &UNK_142d06f88;
    do {
      bVar2 = puVar13[-1];
      uVar7 = (uint)(char)bVar2;
      if ((char)bVar2 < '\0') {
        bVar3 = puVar13[-2];
        if ((char)bVar3 < -0x40) {
          bVar4 = puVar13[-3];
          if ((char)bVar4 < -0x40) {
            pbVar1 = puVar13 + -4;
            puVar13 = puVar13 + -4;
            uVar7 = bVar4 & 0x3f | (*pbVar1 & 7) << 6;
          }
          else {
            puVar13 = puVar13 + -3;
            uVar7 = bVar4 & 0xf;
          }
          uVar7 = bVar3 & 0x3f | uVar7 << 6;
        }
        else {
          puVar13 = puVar13 + -2;
          uVar7 = bVar3 & 0x1f;
        }
        uVar7 = (uint)(bVar2 & 0x3f) | uVar7 << 6;
        if (uVar7 != 0x5c) goto code_r0x0001418c7c74;
code_r0x0001418c7c86:
        puVar15 = puVar13 + -0x142d06f87;
        if (puVar15 != (undefined *)0x0) {
          uVar19 = puVar15 == (undefined *)0x23;
          if ((undefined *)0x22 < puVar15) goto code_r0x0001418c7cc5;
          uVar19 = puVar13[1] == -0x41;
          if ((char)puVar13[1] < -0x40) {
            do {
              FUN_1428d9940(&UNK_142d06f88,0x23);
code_r0x0001418c7cc5:
            } while (!(bool)uVar19);
          }
        }
        lVar8 = 0x23 - (longlong)puVar15;
        puVar15 = &UNK_142d06f88 + (longlong)puVar15;
        goto code_r0x0001418c7cd3;
      }
      puVar13 = puVar13 + -1;
      if (uVar7 == 0x5c) goto code_r0x0001418c7c86;
code_r0x0001418c7c74:
      if (uVar7 == 0x2f) goto code_r0x0001418c7c86;
    } while (puVar13 != &UNK_142d06f88);
    lVar8 = 0x23;
code_r0x0001418c7cd3:
    plVar9 = (longlong *)func_0x000140c18130();
    if (*plVar9 != 1) {
      if ((*plVar9 == 0) &&
         (plVar9 = (longlong *)FUN_1429090d0(plVar9,0), plVar9 != (longlong *)0x0))
      goto code_r0x0001418c7cec;
code_r0x0001418c8066:
      FUN_1428d80d0(&UNK_142d06498);
code_r0x0001418c8072:
      uStack_188 = 0;
      FUN_1428d95d4(0,puStack_78,&uStack_60,&uStack_188,&UNK_142d06ff0);
      goto code_r0x0001418c8118;
    }
    plVar9 = plVar9 + 1;
code_r0x0001418c7cec:
    if (*plVar9 == 0) {
      *plVar9 = -1;
      plStack_50 = plVar9;
      uStack_90 = FUN_140c16d40(plVar9 + 1,puVar18,lVar17,puVar15,lVar8,1,0);
      *plStack_50 = *plStack_50 + 1;
      uStack_98 = 1;
      plStack_50 = (longlong *)uStack_90;
      goto code_r0x0001418c7d4a;
    }
code_r0x0001418c80bb:
    FUN_1428d9250(&UNK_142d06508);
code_r0x0001418c80c7:
    puStack_68 = (ulonglong *)0x0;
  }
  puStack_70 = &uStack_60;
  uStack_60 = 0;
  uStack_188 = 0;
  FUN_1428ca851(0,&puStack_68,&puStack_70,&uStack_188,&UNK_142d07008);
code_r0x0001418c8118:
                    /* WARNING: Does not return */
  pcVar5 = (code *)invalidInstructionException();
  (*pcVar5)();
}

