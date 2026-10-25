
void FUN_1418ccdd0(longlong **param_1,longlong param_2)

{
  ulonglong *puVar1;
  longlong *plVar2;
  ulonglong uVar3;
  longlong lVar4;
  code *pcVar5;
  longlong *plVar6;
  ulonglong *puVar7;
  ulonglong uVar8;
  longlong ***ppplVar9;
  longlong ***ppplVar10;
  longlong ***ppplVar11;
  ulonglong uVar12;
  longlong ****pppplVar13;
  longlong ***ppplVar14;
  ulonglong uVar15;
  ulonglong *puVar16;
  longlong *plVar17;
  uint uVar18;
  longlong **pplVar19;
  uint uVar20;
  longlong ***ppplVar21;
  longlong **pplVar22;
  longlong ***ppplVar23;
  longlong lVar24;
  longlong ****pppplVar25;
  longlong lVar26;
  bool bVar27;
  char cVar28;
  longlong ***ppplStack_288;
  ulonglong uStack_280;
  undefined1 auStack_278 [240];
  ulonglong uStack_188;
  ulonglong uStack_180;
  ulonglong uStack_178;
  longlong ***ppplStack_170;
  ulonglong uStack_168;
  undefined1 auStack_160 [176];
  undefined *puStack_b0;
  undefined *puStack_88;
  undefined8 *puStack_80;
  ulonglong *puStack_78;
  ulonglong uStack_70;
  longlong *plStack_68;
  ulonglong uStack_60;
  ulonglong *puStack_58;
  undefined8 uStack_50;
  undefined8 uStack_48;
  
  uStack_48 = 0xfffffffffffffffe;
  LOCK();
  plVar17 = *param_1;
  if (plVar17 == (longlong *)0x0) {
    *param_1 = (longlong *)0x8;
  }
  UNLOCK();
  if (plVar17 != (longlong *)0x0) {
    puStack_b0 = &UNK_1418ccfd8;
    FUN_1428cb5f0(param_1,param_2,1000000000);
  }
  puStack_b0 = &UNK_1418cce0d;
  FUN_140c81c70(param_1);
  puStack_b0 = &UNK_1418cce1c;
  FUN_140c81c70((ulonglong)param_1 | 1);
  puStack_58 = (ulonglong *)param_1;
  if (*(char *)(param_1 + 0x15) == '\x04') {
    puStack_88 = &UNK_142d08a18;
    puStack_80 = (undefined8 *)0x1;
    puStack_78 = (ulonglong *)0x8;
    uStack_70 = 0;
    plStack_68 = (longlong *)0x0;
    puStack_b0 = &UNK_1418cd029;
    FUN_1428d9390(&puStack_88,&UNK_142d08a28);
                    /* WARNING: Does not return */
    pcVar5 = (code *)invalidInstructionException();
    (*pcVar5)();
  }
  plVar17 = param_1[0x14];
  lVar24 = *(longlong *)(param_2 + 0x58);
  if (lVar24 == *(longlong *)(param_2 + 0x48)) {
    puStack_b0 = &UNK_1418cce4e;
    FUN_140562870(param_2 + 0x48,&UNK_142d05320);
  }
  *(longlong **)(*(longlong *)(param_2 + 0x50) + lVar24 * 8) = plVar17;
  *(longlong *)(param_2 + 0x58) = lVar24 + 1;
  *(undefined1 *)(puStack_58 + 0x15) = 4;
  puStack_78 = puStack_58 + 0xe;
  uStack_70 = puStack_58[0x10];
  puStack_58[0x10] = 0;
  puStack_80 = (longlong *)puStack_58[0xf] + uStack_70;
  plStack_68 = (longlong *)0x0;
  if ((longlong *)uStack_70 != (longlong *)0x0) {
    lVar24 = *(longlong *)(param_2 + 0x70);
    lVar26 = uStack_70 << 3;
    uStack_50 = (longlong *)puStack_58[0xf];
    do {
      plVar17 = uStack_50 + 1;
      lVar4 = *uStack_50;
      uStack_50 = plVar17;
      if (lVar24 == *(longlong *)(param_2 + 0x60)) {
        puStack_b0 = &UNK_1418ccef5;
        FUN_140562870((longlong *)(param_2 + 0x60),&UNK_142d05338);
      }
      *(longlong *)(*(longlong *)(param_2 + 0x68) + lVar24 * 8) = lVar4;
      lVar24 = lVar24 + 1;
      *(longlong *)(param_2 + 0x70) = lVar24;
      lVar26 = lVar26 + -8;
    } while (lVar26 != 0);
  }
  puStack_78 = puStack_58 + 0x11;
  uStack_70 = puStack_58[0x13];
  puStack_58[0x13] = 0;
  puStack_80 = (longlong *)puStack_58[0x12] + uStack_70;
  plStack_68 = (longlong *)0x0;
  if ((longlong *)uStack_70 != (longlong *)0x0) {
    lVar24 = *(longlong *)(param_2 + 0x70);
    lVar26 = uStack_70 << 3;
    plVar17 = (longlong *)puStack_58[0x12];
    do {
      uStack_50 = plVar17 + 1;
      lVar4 = *plVar17;
      if (lVar24 == *(longlong *)(param_2 + 0x60)) {
        puStack_b0 = &UNK_1418ccf95;
        FUN_140562870((longlong *)(param_2 + 0x60),&UNK_142d05338);
      }
      *(longlong *)(*(longlong *)(param_2 + 0x68) + lVar24 * 8) = lVar4;
      lVar24 = lVar24 + 1;
      *(longlong *)(param_2 + 0x70) = lVar24;
      lVar26 = lVar26 + -8;
      plVar17 = uStack_50;
    } while (lVar26 != 0);
  }
  puVar7 = puStack_58;
  puStack_b0 = &UNK_1418ccfa3;
  FUN_140c81e10(puStack_58);
  puStack_b0 = &UNK_1418ccfab;
  FUN_140c81e10();
  LOCK();
  plVar17 = (longlong *)*puVar7;
  if (plVar17 == (longlong *)0x8) {
    *puVar7 = 0;
  }
  plVar6 = plRam00000001436308f0;
  UNLOCK();
  if (plVar17 == (longlong *)0x8) {
    return;
  }
  uStack_48 = 0xfffffffffffffffe;
  while( true ) {
    plVar17 = (longlong *)*plVar6;
    if (plVar17 == (longlong *)0x0) {
      plVar17 = (longlong *)FUN_14290b400();
    }
    pplVar22 = (longlong **)
               ((ulonglong)((longlong)puVar7 * -0x61c8864680b583eb) >> (-(char)plVar17[3] & 0x3fU));
    pplVar19 = (longlong **)plVar17[1];
    if (pplVar19 <= pplVar22) {
      FUN_1428d9518(pplVar22,pplVar19,&UNK_142b1f990);
      pcVar5 = (code *)swi(3);
      (*pcVar5)();
      return;
    }
    puVar1 = (ulonglong *)(*plVar17 + (longlong)pplVar22 * 0x40);
    plVar2 = (longlong *)(*plVar17 + (longlong)pplVar22 * 0x40);
    LOCK();
    bVar27 = *plVar2 == 0;
    if (bVar27) {
      *plVar2 = 1;
    }
    UNLOCK();
    if (!bVar27) {
      FUN_14290b740(puVar1);
    }
    if ((longlong *)*plVar6 == plVar17) break;
    LOCK();
    uVar3 = *puVar1;
    *puVar1 = *puVar1 - 1;
    UNLOCK();
    if (3 < uVar3 && (char)(((uint)uVar3 & 2) >> 1) == '\0') {
      func_0x00014290b8e0();
    }
  }
  uStack_50 = (longlong *)((ulonglong)uStack_50 & 0xffffffffffffff);
  ppplVar21 = (longlong ***)puVar1[1];
  uStack_70 = 0;
  uStack_60 = 0;
  puStack_58 = (ulonglong *)0x0;
  ppplVar10 = (longlong ***)(puVar1 + 1);
  ppplVar23 = (longlong ***)0x0;
code_r0x0001428cc05a:
  uVar3 = uStack_70;
  uVar15 = (ulonglong)puStack_58 & 0xffffffff;
  while( true ) {
    puStack_58 = (ulonglong *)uVar15;
    ppplVar9 = ppplVar23;
    do {
      ppplVar23 = ppplVar21;
      ppplVar14 = ppplVar9;
      ppplVar11 = ppplVar10;
      if (ppplVar23 == (longlong ***)0x0) goto code_r0x0001428cc168;
      pplVar19 = ppplVar23[2];
      ppplVar10 = ppplVar23 + 3;
      ppplVar21 = (longlong ***)ppplVar23[3];
      ppplVar9 = ppplVar23;
    } while (pplVar19 != (longlong **)puVar7);
    if ((uStack_60 & 8) != 0) break;
    pplVar22 = ppplVar23[5];
    if ((uStack_60 & 4) == 0) goto code_r0x0001428cc0bc;
    uVar15 = CONCAT71((int7)(uStack_60 >> 8),1);
    pplVar19 = (longlong **)(ulonglong)((uint)pplVar22 & 0xc);
    if (((ulonglong)pplVar22 & 0xc) == 0) goto code_r0x0001428cc0bc;
  }
  puStack_58 = (ulonglong *)CONCAT71((int7)((ulonglong)ppplVar10 >> 8),1);
code_r0x0001428cc168:
  uVar18 = (uint)pplVar19;
  uVar15 = uStack_70;
  if (8 < uStack_70) {
    uVar15 = uStack_168;
  }
  if (uVar15 == 0) {
    *puVar7 = (ulonglong)(ulonglong)((byte)puStack_58 & 1);
  }
  else {
    uVar12 = FUN_140d635e0();
    cVar28 = -1;
    if (puVar1[3] <= uVar12) {
      cVar28 = uVar12 != puVar1[3];
    }
    if (((cVar28 == '\0') && (cVar28 = uVar18 != (uint)puVar1[4], uVar18 < (uint)puVar1[4])) ||
       (cVar28 != '\x01')) {
      if (uStack_50._7_1_ != '\0') goto code_r0x0001428cc387;
      plVar17 = (longlong *)((ulonglong)puStack_58 & 1);
      pplVar19 = (longlong **)0x0;
    }
    else {
      uVar20 = (uint)puVar1[5] << 0xd ^ (uint)puVar1[5];
      uVar20 = uVar20 >> 0x11 ^ uVar20;
      *(uint *)(puVar1 + 5) = uVar20 << 5 ^ uVar20;
      uVar12 = FUN_140d63a20(uVar12);
      puVar1[3] = uVar12;
      *(uint *)(puVar1 + 4) = uVar18;
code_r0x0001428cc387:
      plVar17 = (longlong *)(uStack_60 | (ulonglong)puStack_58 & 0xff);
      pplVar19 = (longlong **)0x1;
    }
    *puVar7 = (ulonglong)plVar17;
    pppplVar13 = &ppplStack_170;
    if (8 < uVar3) {
      pppplVar13 = (longlong ****)ppplStack_170;
    }
    pppplVar25 = pppplVar13 + uVar15 * 4;
    do {
      (*pppplVar13)[4] = pplVar19;
      ppplVar10 = *pppplVar13;
      ppplVar21 = (longlong ***)*ppplVar10;
      if (*ppplVar21 == (longlong **)0x0) {
        ppplVar21 = ppplVar21 + 1;
        ppplVar10[1] = (longlong **)0x0;
        ppplVar10 = ppplVar10 + 1;
        ppplVar23 = (longlong ***)0x1;
      }
      else {
        LOCK();
        pplVar22 = ppplVar10[1];
        ppplVar10[1] = (longlong **)0x0;
        UNLOCK();
        ppplVar10 = ppplVar10 + 1;
        if (pplVar22 != (longlong **)0x1) {
          ppplVar10 = (longlong ***)0x0;
        }
        ppplVar23 = (longlong ***)0x0;
      }
      pppplVar13[1] = ppplVar23;
      pppplVar13[2] = ppplVar21;
      pppplVar13[3] = ppplVar10;
      pppplVar13 = pppplVar13 + 4;
    } while (pppplVar13 != pppplVar25);
  }
  LOCK();
  uVar3 = *puVar1;
  *puVar1 = *puVar1 - 1;
  UNLOCK();
  if (3 < uVar3 && (char)(((uint)uVar3 & 2) >> 1) == '\0') {
    func_0x00014290b8e0(puVar1);
  }
  func_0x000142923c00(auStack_278,auStack_160,0xf0);
  uVar3 = uStack_70;
  uVar12 = 0;
  bVar27 = 8 < uStack_70;
  uVar15 = uStack_70;
  if (bVar27) {
    uVar15 = uStack_168;
  }
  uStack_188 = 0;
  if (bVar27) {
    uStack_168 = uVar12;
    uStack_188 = uStack_70;
  }
  ppplStack_288 = ppplStack_170;
  uStack_280 = uStack_168;
  uStack_178 = uVar15;
  pppplVar13 = &ppplStack_288;
  if (bVar27) {
    pppplVar13 = (longlong ****)ppplStack_170;
  }
  if (uVar15 != 0) {
    pppplVar25 = pppplVar13 + 3;
    uVar8 = 1;
    do {
      uVar12 = uVar8;
      uStack_180 = uVar12;
      if (pppplVar25[-2] == (longlong ***)0x3) break;
      ppplVar10 = pppplVar25[-1];
      ppplVar21 = *pppplVar25;
      if (((ulonglong)pppplVar25[-2] & 1) == 0) {
        if (ppplVar21 != (longlong ***)0x0) {
          (*(code *)*ppplVar10)(ppplVar10[2],ppplVar21,0,0);
        }
      }
      else {
        (*(code *)ppplVar10[1])(ppplVar21);
      }
      pppplVar25 = pppplVar25 + 4;
      uVar8 = uVar12 + 1;
    } while (uVar12 != uVar15);
  }
  pppplVar13 = pppplVar13 + uVar12 * 4 + 1;
  do {
    uVar12 = uVar12 + 1;
    if (uVar12 - uVar15 == 1) break;
    uStack_180 = uVar12;
    ppplVar10 = *pppplVar13;
    pppplVar13 = pppplVar13 + 4;
  } while (ppplVar10 != (longlong ***)0x3);
  if (8 < uVar3) {
    func_0x000140613c20(ppplStack_170,uVar3 << 5,8);
  }
  return;
code_r0x0001428cc0bc:
  plStack_68 = (longlong *)pplVar22;
  *ppplVar11 = (longlong **)ppplVar21;
  if ((longlong ***)puVar1[2] == ppplVar23) {
    puVar1[2] = (ulonglong)ppplVar14;
  }
  puVar16 = &uStack_168;
  pppplVar13 = (longlong ****)ppplStack_170;
  if (uStack_70 < 9) {
    puVar16 = &uStack_70;
    uVar3 = 8;
    pppplVar13 = &ppplStack_170;
  }
  uVar15 = *puVar16;
  if (*puVar16 == uVar3) {
    FUN_14290b9d0(&ppplStack_170);
    puVar16 = &uStack_168;
    pppplVar13 = (longlong ****)ppplStack_170;
    uVar15 = uStack_168;
  }
  uStack_60 = uStack_60 + (longlong)plStack_68;
  pplVar19 = (longlong **)(uVar15 * 0x20);
  pppplVar13[uVar15 * 4] = ppplVar23;
  (pppplVar13 + 1)[uVar15 * 4] = (longlong ***)0x2;
  *puVar16 = *puVar16 + 1;
  ppplVar10 = ppplVar11;
  ppplVar23 = ppplVar14;
  goto code_r0x0001428cc05a;
}

