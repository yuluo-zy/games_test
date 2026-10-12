
void FUN_140bea680(ulonglong *param_1,undefined8 param_2,longlong param_3,int param_4)

{
  byte *pbVar1;
  undefined4 *puVar2;
  byte bVar3;
  byte bVar4;
  byte bVar5;
  code *pcVar6;
  char cVar7;
  uint uVar8;
  longlong lVar9;
  longlong *plVar10;
  ulonglong uVar11;
  ulonglong uVar12;
  undefined *puVar13;
  undefined *puVar14;
  undefined *puVar15;
  longlong unaff_R15;
  undefined1 uVar16;
  undefined4 uStack_68;
  undefined4 uStack_64;
  undefined4 uStack_60;
  undefined4 uStack_5c;
  longlong *plStack_58;
  longlong *plStack_50;
  undefined8 uStack_48;
  
  uStack_48 = 0xfffffffffffffffe;
  cVar7 = func_0x000140c16a00();
  plStack_50 = (longlong *)CONCAT71(plStack_50._1_7_,cVar7);
  if (cVar7 == '\0') {
code_r0x000140bea8f2:
    uStack_68 = CONCAT31(uStack_68._1_3_,0x1d);
    FUN_1418c1380(0x143634e40,&uStack_68);
    if ((param_3 != 0 || param_4 != 0) && (1 < param_1[3])) {
      uVar11 = 0;
      if (*param_1 <= param_1[2]) {
        uVar11 = *param_1;
      }
      puVar2 = (undefined4 *)(param_1[1] + 0x80 + (param_1[2] - uVar11) * 0x90);
      uStack_68 = *puVar2;
      uStack_64 = puVar2[1];
      uStack_60 = puVar2[2];
      uStack_5c = puVar2[3];
      FUN_140bfcf80(param_2,&uStack_68,param_3,param_4);
    }
    if ((char)plStack_50 == '\0') {
      return;
    }
    plVar10 = (longlong *)func_0x000140c18130();
    if (*plVar10 == 1) {
      plVar10 = plVar10 + 1;
    }
    else if ((*plVar10 != 0) ||
            (plVar10 = (longlong *)FUN_1429090d0(plVar10,0), plVar10 == (longlong *)0x0))
    goto code_r0x000140bea9e5;
    if (*plVar10 == 0) {
      *plVar10 = -1;
      plStack_50 = plVar10;
      FUN_140c170f0(plVar10 + 1,unaff_R15);
      *plStack_50 = *plStack_50 + 1;
      return;
    }
  }
  else {
    puVar14 = &UNK_142b143bb;
    unaff_R15 = 0x3a;
    uVar11 = 0x3a;
    lVar9 = FUN_140be9720(&UNK_142b143bb);
    if (lVar9 == 1) {
      if (uVar11 != 0) {
        if (uVar11 < 0x3a) {
          if ((char)(&UNK_142b143bb)[uVar11] < -0x40) {
code_r0x000140bea6fb:
            FUN_1428d9940(&UNK_142b143bb,0x3a,0,uVar11,&UNK_142b14478);
            goto code_r0x000140bea8f2;
          }
        }
        else if (uVar11 != 0x3a) goto code_r0x000140bea6fb;
      }
      puVar14 = &UNK_142b143bb;
      lVar9 = FUN_140be9720(&UNK_142b143bb);
      if (lVar9 == 1) {
        uVar12 = uVar11 + 2;
        if (uVar12 != 0) {
          uVar16 = uVar12 == 0x3a;
          if (0x39 < uVar12) goto code_r0x000140bea77c;
          uVar16 = (&UNK_142b143bd)[uVar11] == -0x41;
          if ((char)(&UNK_142b143bd)[uVar11] < -0x40) {
            do {
              uVar11 = 0x3a;
              FUN_1428d9940(&UNK_142b143bb,0x3a);
code_r0x000140bea77c:
            } while (!(bool)uVar16);
          }
        }
        unaff_R15 = 0x38 - uVar11;
        puVar14 = &UNK_142b143bb + uVar12;
      }
    }
    puVar13 = &UNK_142b14612;
    puVar15 = &UNK_142b145f0;
    do {
      bVar3 = puVar13[-1];
      uVar8 = (uint)(char)bVar3;
      if ((char)bVar3 < '\0') {
        bVar4 = puVar13[-2];
        if ((char)bVar4 < -0x40) {
          bVar5 = puVar13[-3];
          if ((char)bVar5 < -0x40) {
            pbVar1 = puVar13 + -4;
            puVar13 = puVar13 + -4;
            uVar8 = bVar5 & 0x3f | (*pbVar1 & 7) << 6;
          }
          else {
            puVar13 = puVar13 + -3;
            uVar8 = bVar5 & 0xf;
          }
          uVar8 = bVar4 & 0x3f | uVar8 << 6;
        }
        else {
          puVar13 = puVar13 + -2;
          uVar8 = bVar4 & 0x1f;
        }
        uVar8 = (uint)(bVar3 & 0x3f) | uVar8 << 6;
        if (uVar8 != 0x5c) goto code_r0x000140bea824;
code_r0x000140bea835:
        puVar15 = puVar13 + -0x142b145ef;
        if (puVar15 != (undefined *)0x0) {
          uVar16 = puVar15 == (undefined *)0x22;
          if ((undefined *)0x21 < puVar15) goto code_r0x000140bea874;
          uVar16 = puVar13[1] == -0x41;
          if ((char)puVar13[1] < -0x40) {
            do {
              FUN_1428d9940(&UNK_142b145f0,0x22);
code_r0x000140bea874:
            } while (!(bool)uVar16);
          }
        }
        lVar9 = 0x22 - (longlong)puVar15;
        puVar15 = &UNK_142b145f0 + (longlong)puVar15;
        goto code_r0x000140bea881;
      }
      puVar13 = puVar13 + -1;
      if (uVar8 == 0x5c) goto code_r0x000140bea835;
code_r0x000140bea824:
      if (uVar8 == 0x2f) goto code_r0x000140bea835;
    } while (puVar13 != &UNK_142b145f0);
    lVar9 = 0x22;
code_r0x000140bea881:
    plVar10 = (longlong *)func_0x000140c18130();
    if (*plVar10 == 1) {
      plVar10 = plVar10 + 1;
code_r0x000140bea89a:
      if (*plVar10 != 0) goto code_r0x000140bea9f1;
      *plVar10 = -1;
      plStack_58 = plVar10;
      unaff_R15 = FUN_140c16d40(plVar10 + 1,puVar14,unaff_R15,puVar15,lVar9,&UNK_142b146b0,0x17);
      *plStack_58 = *plStack_58 + 1;
      goto code_r0x000140bea8f2;
    }
    if ((*plVar10 == 0) &&
       (plVar10 = (longlong *)FUN_1429090d0(plVar10,0), plVar10 != (longlong *)0x0))
    goto code_r0x000140bea89a;
code_r0x000140bea9e5:
    FUN_1428d80d0(&UNK_142d06498);
  }
code_r0x000140bea9f1:
  FUN_1428d9250(&UNK_142d06508);
  pcVar6 = (code *)swi(3);
  (*pcVar6)();
  return;
}

