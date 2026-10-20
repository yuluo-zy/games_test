
void FUN_14230edb0(undefined8 *param_1,undefined *param_2,undefined *param_3)

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
  undefined *puVar10;
  undefined *puVar11;
  undefined *puVar12;
  undefined *puVar13;
  longlong unaff_R14;
  undefined1 uVar14;
  
  puVar10 = param_2;
  cVar6 = func_0x000140c16a00();
  if (cVar6 != '\0') {
    unaff_R14 = 0x75;
    lVar8 = FUN_14230f610(0x75);
    puVar11 = &UNK_14304a670;
    if (lVar8 == 1) {
      if (puVar10 != (undefined *)0x0) {
        if (puVar10 < (undefined *)0x75) {
          if ((char)(&UNK_14304a670)[(longlong)puVar10] < -0x40) {
LAB_14230ee1f:
            FUN_1428d9940(&UNK_14304a670,0x75,0,puVar10,&UNK_14304a768);
            goto LAB_14230f01c;
          }
        }
        else if (puVar10 != (undefined *)0x75) goto LAB_14230ee1f;
      }
      lVar8 = FUN_14230f610(puVar10);
      if (lVar8 == 1) {
        puVar11 = puVar10 + 2;
        if (puVar11 != (undefined *)0x0) {
          uVar14 = puVar11 == (undefined *)0x75;
          if ((undefined *)0x74 < puVar11) goto LAB_14230ee96;
          uVar14 = (&UNK_14304a672)[(longlong)puVar10] == -0x41;
          if ((char)(&UNK_14304a672)[(longlong)puVar10] < -0x40) {
            do {
              puVar10 = (undefined *)0x75;
              FUN_1428d9940(&UNK_14304a670,0x75);
LAB_14230ee96:
            } while (!(bool)uVar14);
          }
        }
        unaff_R14 = 0x73 - (longlong)puVar10;
        puVar11 = &UNK_14304a670 + (longlong)puVar11;
      }
    }
    puVar10 = &UNK_14304a5a4;
    puVar13 = &UNK_14304a57e;
    do {
      bVar2 = puVar10[-1];
      uVar7 = (uint)(char)bVar2;
      if ((char)bVar2 < '\0') {
        bVar3 = puVar10[-2];
        if ((char)bVar3 < -0x40) {
          bVar4 = puVar10[-3];
          if ((char)bVar4 < -0x40) {
            pbVar1 = puVar10 + -4;
            puVar10 = puVar10 + -4;
            uVar7 = bVar4 & 0x3f | (*pbVar1 & 7) << 6;
          }
          else {
            puVar10 = puVar10 + -3;
            uVar7 = bVar4 & 0xf;
          }
          uVar7 = bVar3 & 0x3f | uVar7 << 6;
        }
        else {
          puVar10 = puVar10 + -2;
          uVar7 = bVar3 & 0x1f;
        }
        uVar7 = (uint)(bVar2 & 0x3f) | uVar7 << 6;
        if (uVar7 != 0x5c) goto LAB_14230ef34;
LAB_14230ef4f:
        puVar12 = puVar10 + -0x14304a57d;
        if (puVar12 != (undefined *)0x0) {
          uVar14 = puVar12 == (undefined *)0x26;
          if ((undefined *)0x25 < puVar12) goto LAB_14230ef8e;
          uVar14 = puVar10[1] == -0x41;
          if ((char)puVar10[1] < -0x40) {
            do {
              FUN_1428d9940(&UNK_14304a57e,0x26);
LAB_14230ef8e:
            } while (!(bool)uVar14);
          }
        }
        lVar8 = 0x26 - (longlong)puVar12;
        puVar13 = &UNK_14304a57e + (longlong)puVar12;
        goto LAB_14230efa5;
      }
      puVar10 = puVar10 + -1;
      if (uVar7 == 0x5c) goto LAB_14230ef4f;
LAB_14230ef34:
      if (uVar7 == 0x2f) goto LAB_14230ef4f;
    } while (puVar10 != &UNK_14304a57e);
    lVar8 = 0x26;
    puVar12 = &UNK_14304a57e;
LAB_14230efa5:
    plVar9 = (longlong *)func_0x000140c18130();
    if (*plVar9 == 1) {
      plVar9 = plVar9 + 1;
LAB_14230efbe:
      if (*plVar9 != 0) goto LAB_14230f0e8;
      *plVar9 = -1;
      unaff_R14 = FUN_140c16d40(plVar9 + 1,puVar11,unaff_R14,puVar13,lVar8,1,0);
      *plVar9 = *plVar9 + 1;
      goto LAB_14230f01c;
    }
    if ((*plVar9 == 0) && (plVar9 = (longlong *)FUN_14291b030(plVar9,0), plVar9 != (longlong *)0x0))
    goto LAB_14230efbe;
LAB_14230f0c1:
    FUN_1428d80d0(&UNK_14304a0d0);
    param_3 = param_2;
LAB_14230f0cd:
    FUN_1428d98e0(puVar12,param_3,&UNK_14304a5a8);
                    /* WARNING: Does not return */
    pcVar5 = (code *)invalidInstructionException();
    (*pcVar5)();
  }
LAB_14230f01c:
  puVar10 = (undefined *)param_1[1];
  puVar12 = puVar10;
  if (param_3 < puVar10) {
    puVar12 = param_3;
  }
  if (puVar10 < param_3) goto LAB_14230f0cd;
  puVar12 = (undefined *)((longlong)puVar12 << 5);
  func_0x000142923c00(*param_1,param_2,puVar12);
  if (cVar6 == '\0') {
    return;
  }
  plVar9 = (longlong *)func_0x000140c18130();
  if (*plVar9 == 1) {
    plVar9 = plVar9 + 1;
  }
  else {
    param_2 = param_3;
    if ((*plVar9 != 0) || (plVar9 = (longlong *)FUN_14291b030(plVar9,0), plVar9 == (longlong *)0x0))
    goto LAB_14230f0c1;
  }
  if (*plVar9 == 0) {
    *plVar9 = -1;
    FUN_140c170f0(plVar9 + 1,unaff_R14);
    *plVar9 = *plVar9 + 1;
    return;
  }
LAB_14230f0e8:
  FUN_1428d9250(&UNK_14304a140);
  pcVar5 = (code *)swi(3);
  (*pcVar5)();
  return;
}

