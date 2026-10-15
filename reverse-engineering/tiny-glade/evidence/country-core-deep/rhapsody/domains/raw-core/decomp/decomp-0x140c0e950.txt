
void FUN_140c0e950(longlong param_1,longlong param_2)

{
  ulonglong *puVar1;
  longlong *plVar2;
  ulonglong uVar3;
  code *pcVar4;
  longlong *plVar5;
  ulonglong uVar6;
  longlong ***ppplVar7;
  longlong ***ppplVar8;
  longlong ***ppplVar9;
  ulonglong uVar10;
  longlong ****pppplVar11;
  longlong lVar12;
  longlong ***ppplVar13;
  ulonglong uVar14;
  ulonglong *puVar15;
  longlong *plVar16;
  uint uVar17;
  longlong **pplVar18;
  longlong unaff_RBX;
  longlong **unaff_RSI;
  uint uVar19;
  longlong ***ppplVar20;
  longlong **pplVar21;
  longlong ***ppplVar22;
  longlong ****pppplVar23;
  bool bVar24;
  char cVar25;
  longlong ***ppplStack_288;
  ulonglong uStack_280;
  undefined1 auStack_278 [240];
  ulonglong uStack_188;
  ulonglong uStack_180;
  ulonglong uStack_178;
  longlong ***ppplStack_170;
  ulonglong uStack_168;
  undefined1 auStack_160 [224];
  undefined *puStack_80;
  ulonglong uStack_70;
  longlong *plStack_68;
  ulonglong uStack_60;
  undefined *puStack_58;
  char cStack_49;
  undefined8 uStack_48;
  
  if (*(short *)(param_1 + 0x14) == *(short *)(*(longlong *)(param_2 + 0x648) + 0xa50)) {
    if (*(char *)(param_1 + 0x16) != '\x02') goto code_r0x000140c0ea2d;
    unaff_RBX = *(longlong *)(param_1 + 8);
    if (*(int *)(unaff_RBX + 0xc) != *(int *)(param_1 + 0x10)) {
      puStack_80 = &UNK_140c0ea8c;
      FUN_1428d9310(&UNK_142b19c28);
      pcVar4 = (code *)swi(3);
      (*pcVar4)();
      return;
    }
    unaff_RSI = (longlong **)(unaff_RBX + 0x10);
    LOCK();
    plVar16 = *unaff_RSI;
    if (plVar16 == (longlong *)0x0) {
      *unaff_RSI = (longlong *)0x8;
    }
    UNLOCK();
    if (plVar16 == (longlong *)0x0) goto code_r0x000140c0e9a0;
  }
  else {
    uStack_48 = 0;
    lVar12 = param_1 + 0x14;
    puStack_58 = &UNK_142d022a0;
    param_1 = 0;
    puStack_80 = &UNK_140c0ea2d;
    FUN_1428ca816(0,lVar12,*(longlong *)(param_2 + 0x648) + 0xa50,&uStack_48);
code_r0x000140c0ea2d:
    uStack_48 = 0;
    puStack_58 = &UNK_142d022c0;
    puStack_80 = &UNK_140c0ea5c;
    FUN_1428ca902(0,param_1 + 0x16,&UNK_142d022d8,&uStack_48);
  }
  puStack_80 = &UNK_140c0ea6a;
  FUN_1428cb5f0(unaff_RSI);
code_r0x000140c0e9a0:
  puStack_80 = &UNK_140c0e9a8;
  FUN_140c81c70(unaff_RSI);
  puStack_80 = &UNK_140c0e9b7;
  FUN_140c81c70((ulonglong)unaff_RSI | 1);
  if (*(int *)(unaff_RBX + 0x40) == 2) {
    *(undefined8 *)(unaff_RBX + 0x60) = *(undefined8 *)(unaff_RBX + 0x38);
    *(undefined8 *)(unaff_RBX + 0x50) = *(undefined8 *)(unaff_RBX + 0x28);
    *(undefined8 *)(unaff_RBX + 0x58) = *(undefined8 *)(unaff_RBX + 0x30);
    *(undefined4 *)(unaff_RBX + 0x40) = *(undefined4 *)(unaff_RBX + 0x18);
    *(undefined4 *)(unaff_RBX + 0x44) = *(undefined4 *)(unaff_RBX + 0x1c);
    *(undefined4 *)(unaff_RBX + 0x48) = *(undefined4 *)(unaff_RBX + 0x20);
    *(undefined4 *)(unaff_RBX + 0x4c) = *(undefined4 *)(unaff_RBX + 0x24);
  }
  puStack_80 = &UNK_140c0e9e0;
  FUN_140c81e10(unaff_RSI);
  puStack_80 = &UNK_140c0e9e8;
  FUN_140c81e10();
  LOCK();
  plVar16 = *unaff_RSI;
  if (plVar16 == (longlong *)0x8) {
    *unaff_RSI = (longlong *)0x0;
  }
  plVar5 = plRam00000001436308f0;
  UNLOCK();
  if (plVar16 == (longlong *)0x8) {
    return;
  }
  uStack_48 = 0xfffffffffffffffe;
  while( true ) {
    plVar16 = (longlong *)*plVar5;
    if (plVar16 == (longlong *)0x0) {
      plVar16 = (longlong *)FUN_14290b400();
    }
    pplVar21 = (longlong **)
               ((ulonglong)((longlong)unaff_RSI * -0x61c8864680b583eb) >>
               (-(char)plVar16[3] & 0x3fU));
    pplVar18 = (longlong **)plVar16[1];
    if (pplVar18 <= pplVar21) {
      FUN_1428d9518(pplVar21,pplVar18,&UNK_142b1f990);
      pcVar4 = (code *)swi(3);
      (*pcVar4)();
      return;
    }
    puVar1 = (ulonglong *)(*plVar16 + (longlong)pplVar21 * 0x40);
    plVar2 = (longlong *)(*plVar16 + (longlong)pplVar21 * 0x40);
    LOCK();
    bVar24 = *plVar2 == 0;
    if (bVar24) {
      *plVar2 = 1;
    }
    UNLOCK();
    if (!bVar24) {
      FUN_14290b740(puVar1);
    }
    if ((longlong *)*plVar5 == plVar16) break;
    LOCK();
    uVar3 = *puVar1;
    *puVar1 = *puVar1 - 1;
    UNLOCK();
    if (3 < uVar3 && (char)(((uint)uVar3 & 2) >> 1) == '\0') {
      func_0x00014290b8e0();
    }
  }
  cStack_49 = '\0';
  ppplVar20 = (longlong ***)puVar1[1];
  uStack_70 = 0;
  uStack_60 = 0;
  puStack_58 = (undefined *)0x0;
  ppplVar8 = (longlong ***)(puVar1 + 1);
  ppplVar22 = (longlong ***)0x0;
code_r0x0001428cc05a:
  uVar3 = uStack_70;
  uVar14 = (ulonglong)puStack_58 & 0xffffffff;
  while( true ) {
    puStack_58 = (undefined *)uVar14;
    ppplVar7 = ppplVar22;
    do {
      ppplVar22 = ppplVar20;
      ppplVar13 = ppplVar7;
      ppplVar9 = ppplVar8;
      if (ppplVar22 == (longlong ***)0x0) goto code_r0x0001428cc168;
      pplVar18 = ppplVar22[2];
      ppplVar8 = ppplVar22 + 3;
      ppplVar20 = (longlong ***)ppplVar22[3];
      ppplVar7 = ppplVar22;
    } while (pplVar18 != unaff_RSI);
    if ((uStack_60 & 8) != 0) break;
    pplVar21 = ppplVar22[5];
    if ((uStack_60 & 4) == 0) goto code_r0x0001428cc0bc;
    uVar14 = CONCAT71((int7)(uStack_60 >> 8),1);
    pplVar18 = (longlong **)(ulonglong)((uint)pplVar21 & 0xc);
    if (((ulonglong)pplVar21 & 0xc) == 0) goto code_r0x0001428cc0bc;
  }
  puStack_58 = (undefined *)CONCAT71((int7)((ulonglong)ppplVar8 >> 8),1);
code_r0x0001428cc168:
  uVar17 = (uint)pplVar18;
  uVar14 = uStack_70;
  if (8 < uStack_70) {
    uVar14 = uStack_168;
  }
  if (uVar14 == 0) {
    *unaff_RSI = (longlong *)(ulonglong)((byte)puStack_58 & 1);
  }
  else {
    uVar10 = FUN_140d635e0();
    cVar25 = -1;
    if (puVar1[3] <= uVar10) {
      cVar25 = uVar10 != puVar1[3];
    }
    if (((cVar25 == '\0') && (cVar25 = uVar17 != (uint)puVar1[4], uVar17 < (uint)puVar1[4])) ||
       (cVar25 != '\x01')) {
      if (cStack_49 != '\0') goto code_r0x0001428cc387;
      plVar16 = (longlong *)((ulonglong)puStack_58 & 1);
      pplVar18 = (longlong **)0x0;
    }
    else {
      uVar19 = (uint)puVar1[5] << 0xd ^ (uint)puVar1[5];
      uVar19 = uVar19 >> 0x11 ^ uVar19;
      *(uint *)(puVar1 + 5) = uVar19 << 5 ^ uVar19;
      uVar10 = FUN_140d63a20(uVar10);
      puVar1[3] = uVar10;
      *(uint *)(puVar1 + 4) = uVar17;
code_r0x0001428cc387:
      plVar16 = (longlong *)(uStack_60 | (ulonglong)puStack_58 & 0xff);
      pplVar18 = (longlong **)0x1;
    }
    *unaff_RSI = plVar16;
    pppplVar11 = &ppplStack_170;
    if (8 < uVar3) {
      pppplVar11 = (longlong ****)ppplStack_170;
    }
    pppplVar23 = pppplVar11 + uVar14 * 4;
    do {
      (*pppplVar11)[4] = pplVar18;
      ppplVar8 = *pppplVar11;
      ppplVar20 = (longlong ***)*ppplVar8;
      if (*ppplVar20 == (longlong **)0x0) {
        ppplVar20 = ppplVar20 + 1;
        ppplVar8[1] = (longlong **)0x0;
        ppplVar8 = ppplVar8 + 1;
        ppplVar22 = (longlong ***)0x1;
      }
      else {
        LOCK();
        pplVar21 = ppplVar8[1];
        ppplVar8[1] = (longlong **)0x0;
        UNLOCK();
        ppplVar8 = ppplVar8 + 1;
        if (pplVar21 != (longlong **)0x1) {
          ppplVar8 = (longlong ***)0x0;
        }
        ppplVar22 = (longlong ***)0x0;
      }
      pppplVar11[1] = ppplVar22;
      pppplVar11[2] = ppplVar20;
      pppplVar11[3] = ppplVar8;
      pppplVar11 = pppplVar11 + 4;
    } while (pppplVar11 != pppplVar23);
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
  uVar10 = 0;
  bVar24 = 8 < uStack_70;
  uVar14 = uStack_70;
  if (bVar24) {
    uVar14 = uStack_168;
  }
  uStack_188 = 0;
  if (bVar24) {
    uStack_168 = uVar10;
    uStack_188 = uStack_70;
  }
  ppplStack_288 = ppplStack_170;
  uStack_280 = uStack_168;
  uStack_178 = uVar14;
  pppplVar11 = &ppplStack_288;
  if (bVar24) {
    pppplVar11 = (longlong ****)ppplStack_170;
  }
  if (uVar14 != 0) {
    pppplVar23 = pppplVar11 + 3;
    uVar6 = 1;
    do {
      uVar10 = uVar6;
      uStack_180 = uVar10;
      if (pppplVar23[-2] == (longlong ***)0x3) break;
      ppplVar8 = pppplVar23[-1];
      ppplVar20 = *pppplVar23;
      if (((ulonglong)pppplVar23[-2] & 1) == 0) {
        if (ppplVar20 != (longlong ***)0x0) {
          (*(code *)*ppplVar8)(ppplVar8[2],ppplVar20,0,0);
        }
      }
      else {
        (*(code *)ppplVar8[1])(ppplVar20);
      }
      pppplVar23 = pppplVar23 + 4;
      uVar6 = uVar10 + 1;
    } while (uVar10 != uVar14);
  }
  pppplVar11 = pppplVar11 + uVar10 * 4 + 1;
  do {
    uVar10 = uVar10 + 1;
    if (uVar10 - uVar14 == 1) break;
    uStack_180 = uVar10;
    ppplVar8 = *pppplVar11;
    pppplVar11 = pppplVar11 + 4;
  } while (ppplVar8 != (longlong ***)0x3);
  if (8 < uVar3) {
    func_0x000140613c20(ppplStack_170,uVar3 << 5,8);
  }
  return;
code_r0x0001428cc0bc:
  plStack_68 = (longlong *)pplVar21;
  *ppplVar9 = (longlong **)ppplVar20;
  if ((longlong ***)puVar1[2] == ppplVar22) {
    puVar1[2] = (ulonglong)ppplVar13;
  }
  puVar15 = &uStack_168;
  pppplVar11 = (longlong ****)ppplStack_170;
  if (uStack_70 < 9) {
    puVar15 = &uStack_70;
    uVar3 = 8;
    pppplVar11 = &ppplStack_170;
  }
  uVar14 = *puVar15;
  if (*puVar15 == uVar3) {
    FUN_14290b9d0(&ppplStack_170);
    puVar15 = &uStack_168;
    pppplVar11 = (longlong ****)ppplStack_170;
    uVar14 = uStack_168;
  }
  uStack_60 = uStack_60 + (longlong)plStack_68;
  pplVar18 = (longlong **)(uVar14 * 0x20);
  pppplVar11[uVar14 * 4] = ppplVar22;
  (pppplVar11 + 1)[uVar14 * 4] = (longlong ***)0x2;
  *puVar15 = *puVar15 + 1;
  ppplVar8 = ppplVar9;
  ppplVar22 = ppplVar13;
  goto code_r0x0001428cc05a;
}

