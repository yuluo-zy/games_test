
void FUN_1418c3300(longlong param_1,ulonglong param_2)

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
  ulonglong uVar10;
  longlong unaff_RBX;
  longlong lVar11;
  ulonglong uVar12;
  undefined *puVar13;
  undefined *puVar14;
  undefined *puVar15;
  undefined1 uVar16;
  
  uVar10 = param_2;
  cVar6 = func_0x000140c16a00();
  if (cVar6 == '\0') {
code_r0x0001418c354d:
    FUN_140bf0b90(param_1 + 0x10,param_2);
    lVar8 = *(longlong *)(param_1 + 200);
    lVar11 = *(longlong *)(param_1 + 0xd0) * 0xb0 + lVar8;
    for (; lVar8 != lVar11; lVar8 = lVar8 + 0xb0) {
      FUN_140bf0b90();
    }
    if (cVar6 == '\0') {
      return;
    }
    plVar9 = (longlong *)func_0x000140c18130();
    if (*plVar9 == 1) {
      plVar9 = plVar9 + 1;
    }
    else if ((*plVar9 != 0) ||
            (plVar9 = (longlong *)FUN_1429090d0(plVar9,0), plVar9 == (longlong *)0x0))
    goto code_r0x0001418c3615;
    if (*plVar9 == 0) {
      *plVar9 = -1;
      FUN_140c170f0(plVar9 + 1,unaff_RBX);
      *plVar9 = *plVar9 + 1;
      return;
    }
  }
  else {
    unaff_RBX = 0x49;
    lVar8 = FUN_1418c2ed0(0x49);
    puVar14 = &UNK_142d065d0;
    if (lVar8 == 1) {
      if (uVar10 != 0) {
        if (uVar10 < 0x49) {
          if ((char)(&UNK_142d065d0)[uVar10] < -0x40) {
code_r0x0001418c3369:
            FUN_1428d9940(&UNK_142d065d0,0x49,0,uVar10,&UNK_142d066a0);
            goto code_r0x0001418c354d;
          }
        }
        else if (uVar10 != 0x49) goto code_r0x0001418c3369;
      }
      lVar8 = FUN_1418c2ed0(uVar10);
      if (lVar8 == 1) {
        uVar12 = uVar10 + 2;
        if (uVar12 != 0) {
          uVar16 = uVar12 == 0x49;
          if (0x48 < uVar12) goto code_r0x0001418c33e0;
          uVar16 = (&UNK_142d065d2)[uVar10] == -0x41;
          if ((char)(&UNK_142d065d2)[uVar10] < -0x40) {
            do {
              uVar10 = 0x49;
              FUN_1428d9940(&UNK_142d065d0,0x49);
code_r0x0001418c33e0:
            } while (!(bool)uVar16);
          }
        }
        unaff_RBX = 0x47 - uVar10;
        puVar14 = &UNK_142d065d0 + uVar12;
      }
    }
    puVar13 = &UNK_142d06709;
    puVar15 = &UNK_142d066e8;
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
        if (uVar7 != 0x5c) goto code_r0x0001418c3484;
code_r0x0001418c3496:
        puVar15 = puVar13 + -0x142d066e7;
        if (puVar15 != (undefined *)0x0) {
          uVar16 = puVar15 == (undefined *)0x21;
          if ((undefined *)0x20 < puVar15) goto code_r0x0001418c34d5;
          uVar16 = puVar13[1] == -0x41;
          if ((char)puVar13[1] < -0x40) {
            do {
              FUN_1428d9940(&UNK_142d066e8,0x21);
code_r0x0001418c34d5:
            } while (!(bool)uVar16);
          }
        }
        lVar8 = 0x21 - (longlong)puVar15;
        puVar15 = &UNK_142d066e8 + (longlong)puVar15;
        goto code_r0x0001418c34e3;
      }
      puVar13 = puVar13 + -1;
      if (uVar7 == 0x5c) goto code_r0x0001418c3496;
code_r0x0001418c3484:
      if (uVar7 == 0x2f) goto code_r0x0001418c3496;
    } while (puVar13 != &UNK_142d066e8);
    lVar8 = 0x21;
code_r0x0001418c34e3:
    plVar9 = (longlong *)func_0x000140c18130();
    if (*plVar9 == 1) {
      plVar9 = plVar9 + 1;
code_r0x0001418c34fc:
      if (*plVar9 != 0) goto code_r0x0001418c3621;
      *plVar9 = -1;
      unaff_RBX = FUN_140c16d40(plVar9 + 1,puVar14,unaff_RBX,puVar15,lVar8,1,0);
      *plVar9 = *plVar9 + 1;
      goto code_r0x0001418c354d;
    }
    if ((*plVar9 == 0) && (plVar9 = (longlong *)FUN_1429090d0(plVar9,0), plVar9 != (longlong *)0x0))
    goto code_r0x0001418c34fc;
code_r0x0001418c3615:
    FUN_1428d80d0(&UNK_142d06498);
  }
code_r0x0001418c3621:
  FUN_1428d9250(&UNK_142d06508);
  pcVar5 = (code *)swi(3);
  (*pcVar5)();
  return;
}

