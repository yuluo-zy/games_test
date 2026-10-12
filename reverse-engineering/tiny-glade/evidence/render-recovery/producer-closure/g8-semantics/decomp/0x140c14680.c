
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

undefined8 * FUN_140c14680(undefined8 *param_1,longlong *param_2,longlong *param_3)

{
  undefined8 ****ppppuVar1;
  longlong ***ppplVar2;
  uint uVar3;
  ulonglong uVar4;
  ulonglong uVar5;
  code *pcVar6;
  undefined4 uVar7;
  longlong lVar8;
  undefined4 uVar9;
  undefined4 uVar10;
  undefined8 uVar11;
  ulonglong *puVar12;
  longlong ****pppplVar13;
  char cVar14;
  undefined4 *puVar15;
  longlong ***ppplVar16;
  longlong *plVar17;
  undefined8 *****pppppuVar18;
  undefined *puVar19;
  undefined8 *****pppppuVar20;
  longlong *plVar21;
  longlong *plVar22;
  undefined8 ****ppppuStack_150;
  undefined8 **ppuStack_148;
  longlong *plStack_140;
  undefined8 ****ppppuStack_138;
  longlong *****ppppplStack_130;
  undefined8 *****pppppuStack_128;
  ulonglong uStack_120;
  undefined4 uStack_118;
  undefined4 uStack_114;
  undefined4 uStack_110;
  undefined4 uStack_10c;
  longlong lStack_108;
  longlong ***ppplStack_100;
  undefined8 *puStack_f8;
  undefined4 uStack_f0;
  undefined4 uStack_ec;
  undefined *puStack_e8;
  undefined8 uStack_e0;
  longlong *****ppppplStack_d8;
  undefined8 uStack_d0;
  undefined8 uStack_c8;
  undefined8 *****pppppuStack_b8;
  undefined8 *****pppppuStack_b0;
  undefined8 ****ppppuStack_a8;
  longlong ***ppplStack_a0;
  longlong ****pppplStack_98;
  longlong ****pppplStack_90;
  undefined8 *****pppppuStack_88;
  longlong *plStack_80;
  longlong *plStack_78;
  longlong lStack_70;
  char cStack_68;
  undefined4 uStack_67;
  undefined3 uStack_63;
  longlong ***ppplStack_60;
  longlong ****pppplStack_58;
  longlong *plStack_50;
  undefined1 uStack_41;
  undefined8 uStack_40;
  
  uStack_40 = 0xfffffffffffffffe;
  plStack_140 = param_3;
  if (*(char *)((longlong)param_2 + 0x61c) == '\x01') {
    ppppuStack_138 = (undefined8 ****)(param_3 + 7);
    uStack_41 = 1;
    plVar17 = param_2;
    pppplStack_58 = (longlong ****)FUN_1418d45a0(&ppppuStack_138);
    puVar12 = puRam000000014362c3a8;
    if (plVar17 == (longlong *)0x0) {
      ppplStack_a0 = (longlong ***)&UNK_142b1b4b8;
      pppplStack_98 = (longlong ****)0x6;
    }
    else {
      ppplStack_a0 = (longlong ***)plVar17[3];
      pppplStack_98 = (longlong ****)plVar17[4];
    }
    plStack_50 = plVar17;
    if (3 < *puRam000000014362c3a8) {
      pppplStack_90 = &ppplStack_a0;
      pppppuStack_88 = (undefined8 *****)&UNK_140109e90;
      puVar15 = (undefined4 *)func_0x00014067dd80(&UNK_142b1b4f0);
      uStack_118 = *puVar15;
      uStack_114 = puVar15[1];
      uStack_110 = puVar15[2];
      uStack_10c = puVar15[3];
      uStack_ec = puVar15[4];
      lStack_108 = 4;
      ppplStack_100 = (longlong ***)&UNK_142b1b078;
      puStack_f8 = (undefined8 *)0x15;
      puStack_e8 = &UNK_142b1b4d0;
      uStack_e0 = 2;
      ppppplStack_d8 = &pppplStack_90;
      uStack_d0 = 1;
      uStack_c8 = 0;
      ppppuStack_138 = (undefined8 ****)0x0;
      ppppplStack_130 = (longlong *****)&UNK_142b1b078;
      pppppuStack_128 = (undefined8 *****)0x15;
      uStack_120 = 0;
      uStack_f0 = 1;
      func_0x000140d4a330(&ppplStack_60,&ppppuStack_138);
    }
    if (*(char *)((longlong)param_2 + 0x61d) != '\0') {
      FUN_140d56a30(&pppplStack_90);
      if (3 < *puVar12) {
        pppppuStack_b0 = (undefined8 *****)FUN_140d56e50;
        pppppuStack_b8 = &pppplStack_90;
        puVar15 = (undefined4 *)func_0x00014067dd80(&UNK_142b1b530);
        uStack_118 = *puVar15;
        uStack_114 = puVar15[1];
        uStack_110 = puVar15[2];
        uStack_10c = puVar15[3];
        uStack_ec = puVar15[4];
        lStack_108 = 4;
        ppplStack_100 = (longlong ***)&UNK_142b1b078;
        puStack_f8 = (undefined8 *)0x15;
        puStack_e8 = &UNK_142b1b520;
        uStack_e0 = 1;
        ppppplStack_d8 = (longlong *****)&pppppuStack_b8;
        uStack_d0 = 1;
        uStack_c8 = 0;
        ppppuStack_138 = (undefined8 ****)0x0;
        ppppplStack_130 = (longlong *****)&UNK_142b1b078;
        pppppuStack_128 = (undefined8 *****)0x15;
        uStack_120 = 0;
        uStack_f0 = 1;
        func_0x000140d4a330(&ppplStack_60,&ppppuStack_138);
      }
      FUN_1418beca0(&pppplStack_90);
    }
    pppplVar13 = pppplStack_58;
    pppplStack_58 = (longlong ****)0x0;
    if (pppplVar13 != (longlong ****)0x0) {
      ppplVar16 = (longlong ***)(plStack_50 + 2);
      if (plStack_50 == (longlong *)0x0) {
        ppplVar16 = (longlong ***)0x0;
      }
      LOCK();
      ppplVar2 = *pppplVar13;
      if (ppplVar16 == ppplVar2) {
        *pppplVar13 = (longlong ***)0x3;
      }
      UNLOCK();
      if (ppplVar16 == ppplVar2) goto code_r0x000140c148eb;
    }
    if (plStack_50 != (longlong *)0x0) {
      LOCK();
      *plStack_50 = *plStack_50 + -1;
      UNLOCK();
      if (*plStack_50 == 0) {
        uStack_41 = 1;
        FUN_140c15f40(&plStack_50);
      }
    }
  }
code_r0x000140c148eb:
  if (*plStack_140 == 0) {
    *param_1 = 9;
    pppppuStack_b8 = (undefined8 *****)(plStack_140 + 7);
    ppplVar16 = (longlong ***)plStack_140[7];
    ppplStack_60 = (longlong ***)(plStack_140 + 8);
    pppplStack_90 = &ppplStack_60;
    pppppuStack_88 = &pppplStack_58;
    ppppuStack_138 = &ppplStack_a0;
    ppppplStack_130 = (longlong *****)&pppppuStack_b8;
    pppppuStack_128 = &pppplStack_90;
    ppplStack_a0 = ppplVar16;
    pppplStack_58 = (longlong ****)pppppuStack_b8;
    FUN_1418d4680(&ppppuStack_138);
    if (ppplVar16 == (longlong ***)0x0) {
      return param_1;
    }
    ppppuStack_138 = (undefined8 ****)(ppplVar16 + -2);
    LOCK();
    ppplVar16 = ppplVar16 + -2;
    *ppplVar16 = (longlong **)((longlong)*ppplVar16 + -1);
    UNLOCK();
    if (*ppplVar16 != (longlong **)0x0) {
      return param_1;
    }
    FUN_140c15f40(&ppppuStack_138);
    return param_1;
  }
  uVar4 = plStack_140[4];
  if ((ulonglong)param_2[2] <= uVar4) {
    uStack_41 = 1;
    FUN_1428d9518(uVar4,param_2[2],&UNK_142b1b548);
code_r0x000140c14cf9:
                    /* WARNING: Does not return */
    pcVar6 = (code *)invalidInstructionException();
    (*pcVar6)();
  }
  puStack_f8 = (undefined8 *)plStack_140[8];
  ppppuStack_138 = (undefined8 ****)*plStack_140;
  ppppplStack_130 = (longlong *****)plStack_140[1];
  pppppuStack_128 = (undefined8 *****)plStack_140[2];
  uStack_120 = plStack_140[3];
  uStack_118 = (undefined4)plStack_140[4];
  uStack_114 = *(undefined4 *)((longlong)plStack_140 + 0x24);
  uStack_110 = (undefined4)plStack_140[5];
  uStack_10c = *(undefined4 *)((longlong)plStack_140 + 0x2c);
  lStack_108 = plStack_140[6];
  ppplStack_100 = (longlong ***)plStack_140[7];
  uVar5 = *(ulonglong *)(param_2[1] + 0x10 + uVar4 * 0x38);
  if (uVar5 <= uStack_120) {
    FUN_1428d9518(uStack_120,uVar5,&UNK_142b1b1f8);
    goto code_r0x000140c14cf9;
  }
  puVar19 = (undefined *)(param_2[1] + uVar4 * 0x38);
  if (*(char *)(*(longlong *)(puVar19 + 8) + 0x28 + uStack_120 * 0x30) == '\x02') {
    plVar17 = (longlong *)func_0x000140613c10(0x1a,1);
    if (plVar17 == (longlong *)0x0) {
      FUN_1428d8fe3(1,0x1a,&UNK_142b1adc8);
      goto code_r0x000140c14cf9;
    }
  }
  else {
    plVar21 = (longlong *)(*(longlong *)(puVar19 + 8) + uStack_120 * 0x30);
    (**(code **)(plVar21[1] + 0x48))(&pppplStack_90,*plVar21,ppppuStack_138);
    pppppuVar18 = (undefined8 *****)pppplStack_90;
    plVar17 = plStack_80;
    pppppuVar20 = pppppuStack_88;
    plVar22 = plStack_78;
    if ((undefined8 *****)pppplStack_90 != (undefined8 *****)0x9) goto code_r0x000140c14c08;
    cVar14 = (**(code **)(plVar21[1] + 0x78))(*plVar21);
    pppppuVar18 = (undefined8 *****)0x9;
    plVar17 = param_2;
    pppppuVar20 = (undefined8 *****)puVar19;
    plVar22 = plVar21;
    if (cVar14 == '\0') goto code_r0x000140c14c08;
    cVar14 = (**(code **)(plVar21[1] + 0x68))(*plVar21);
    plVar17 = param_2 + 6;
    if (cVar14 == '\0') {
      lVar8 = plVar21[5];
      *(undefined1 *)(plVar21 + 5) = 2;
      if ((char)lVar8 != '\x02') {
        uStack_67 = *(undefined4 *)((longlong)plVar21 + 0x29);
        uVar3 = *(uint *)((longlong)plVar21 + 0x2c);
        uStack_63 = (undefined3)(uVar3 >> 8);
        pppplStack_90 = (longlong ****)*plVar21;
        pppppuStack_88 = (undefined8 *****)plVar21[1];
        plStack_80 = (longlong *)plVar21[2];
        plStack_78 = (longlong *)plVar21[3];
        lStack_70 = plVar21[4];
        cStack_68 = (char)lVar8;
        FUN_140c11ab0(&pppplStack_90,plVar17);
        pppppuVar20 = (undefined8 *****)(ulonglong)uVar3;
        goto code_r0x000140c14c08;
      }
      plVar17 = (longlong *)func_0x000140613c10(0x1a,1);
      if (plVar17 == (longlong *)0x0) {
        FUN_1428d8fe3(1,0x1a,&UNK_142b1adc8);
        goto code_r0x000140c14cf9;
      }
    }
    else {
      uVar4 = *(ulonglong *)(puVar19 + 0x28);
      if (uVar4 < 2) goto code_r0x000140c14c08;
      lVar8 = plVar21[5];
      *(undefined1 *)(plVar21 + 5) = 2;
      if ((char)lVar8 != '\x02') {
        uStack_67 = *(undefined4 *)((longlong)plVar21 + 0x29);
        uVar3 = *(uint *)((longlong)plVar21 + 0x2c);
        uStack_63 = (undefined3)(uVar3 >> 8);
        pppplStack_90 = (longlong ****)*plVar21;
        pppppuStack_88 = (undefined8 *****)plVar21[1];
        plStack_80 = (longlong *)plVar21[2];
        plStack_78 = (longlong *)plVar21[3];
        lStack_70 = plVar21[4];
        cStack_68 = (char)lVar8;
        FUN_140c11ab0(&pppplStack_90,plVar17);
        *(ulonglong *)(puVar19 + 0x28) = uVar4 - 1;
        plVar22 = (longlong *)(ulonglong)uVar3;
        goto code_r0x000140c14c08;
      }
      plVar17 = (longlong *)func_0x000140613c10(0x1a,1);
      if (plVar17 == (longlong *)0x0) {
        FUN_1428d8fe3(1,0x1a,&UNK_142b1adc8);
        goto code_r0x000140c14cf9;
      }
    }
  }
  uVar11 = _UNK_142b1b222;
  *(ulonglong *)((longlong)plVar17 + 10) =
       CONCAT26(_UNK_142b1b220,CONCAT42(_UNK_142b1b21c,_UNK_142b1b21a));
  *(undefined8 *)((longlong)plVar17 + 0x12) = uVar11;
  uVar10 = _UNK_142b1b21c;
  uVar9 = _UNK_142b1b214;
  uVar7 = CONCAT22(_UNK_142b1b21a,_UNK_142b1b218);
  *(undefined4 *)plVar17 = _UNK_142b1b210;
  *(undefined4 *)((longlong)plVar17 + 4) = uVar9;
  *(undefined4 *)(plVar17 + 1) = uVar7;
  *(undefined4 *)((longlong)plVar17 + 0xc) = uVar10;
  pppppuVar18 = (undefined8 *****)0x5;
  pppppuVar20 = (undefined8 *****)(undefined *)0x1a;
  plVar22 = (longlong *)0x1a;
code_r0x000140c14c08:
  ppplVar16 = ppplStack_100;
  ppppuStack_150 = &ppplStack_100;
  ppplStack_60 = ppplStack_100;
  ppplStack_a0 = &ppuStack_148;
  pppplStack_98 = (longlong ****)&ppppuStack_150;
  pppppuStack_b8 = (undefined8 *****)&ppplStack_60;
  pppppuStack_b0 = &pppplStack_58;
  ppppuStack_a8 = &ppplStack_a0;
  uStack_41 = 0;
  ppuStack_148 = &puStack_f8;
  pppplStack_58 = ppppuStack_150;
  FUN_1418d4680(&pppppuStack_b8);
  if ((undefined8 ****)ppplVar16 != (undefined8 ****)0x0) {
    pppppuStack_b8 = (undefined8 *****)(ppplVar16 + -2);
    LOCK();
    ppppuVar1 = (undefined8 ****)(ppplVar16 + -2);
    *ppppuVar1 = (undefined8 ***)((longlong)*ppppuVar1 + -1);
    UNLOCK();
    if (*ppppuVar1 == (undefined8 ***)0x0) {
      uStack_41 = 0;
      FUN_140c15f40(&pppppuStack_b8);
    }
  }
  if ((int)pppppuVar18 == 9) {
    *param_1 = 9;
  }
  else {
    *param_1 = pppppuVar18;
    param_1[1] = pppppuVar20;
    param_1[2] = plVar17;
    param_1[3] = plVar22;
  }
  return param_1;
}

