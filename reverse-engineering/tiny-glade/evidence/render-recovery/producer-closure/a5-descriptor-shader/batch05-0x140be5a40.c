
undefined8 FUN_140be5a40(longlong param_1,longlong param_2,longlong param_3)

{
  longlong *plVar1;
  undefined8 *puVar2;
  char cVar3;
  code *pcVar4;
  code *pcVar5;
  undefined8 uVar6;
  code *pcVar7;
  longlong lVar8;
  longlong lVar9;
  undefined *puVar10;
  char *pcVar11;
  int iVar12;
  longlong *plVar13;
  undefined8 *puVar14;
  undefined8 uVar15;
  ulonglong uVar16;
  bool bVar17;
  undefined8 uStack_108;
  longlong *plStack_100;
  undefined8 *puStack_f8;
  undefined8 uStack_f0;
  longlong lStack_e8;
  undefined4 uStack_e0;
  undefined4 uStack_dc;
  undefined8 uStack_d8;
  undefined *puStack_d0;
  undefined8 uStack_c8;
  undefined4 uStack_c0;
  undefined4 uStack_bc;
  undefined *puStack_b8;
  undefined8 uStack_b0;
  undefined8 *puStack_a8;
  undefined8 uStack_a0;
  undefined8 uStack_98;
  undefined8 uStack_88;
  longlong lStack_80;
  char *pcStack_78;
  ulonglong uStack_70;
  longlong *plStack_68;
  undefined8 uStack_60;
  undefined4 uStack_58;
  undefined4 uStack_54;
  undefined1 uStack_49;
  undefined8 uStack_48;
  
  uStack_48 = 0xfffffffffffffffe;
  LOCK();
  bVar17 = *(char *)(param_1 + 0x58) == '\0';
  if (bVar17) {
    *(char *)(param_1 + 0x58) = '\x01';
  }
  UNLOCK();
  if (!bVar17) {
    FUN_1428cd210((char *)(param_1 + 0x58),1,1000000000);
  }
  pcStack_78 = (char *)(param_1 + 0x58);
  FUN_140c81c70();
  lStack_80 = param_1 + 0x60;
  uStack_88 = 0;
  pcVar4 = *(code **)(param_2 + 0x1c8);
  pcVar5 = *(code **)(param_2 + 0x1d8);
  uVar6 = *(undefined8 *)(param_2 + 0x5c8);
  pcVar7 = *(code **)(param_2 + 0x1e0);
  while( true ) {
    while( true ) {
      lVar8 = *(longlong *)(param_1 + 0x70);
      plVar13 = (longlong *)(lVar8 * 8 + *(longlong *)(param_1 + 0x68) + -8);
      if (plVar13 != (longlong *)0x0 && lVar8 != 0) break;
      if (lVar8 != 0) {
        FUN_1428d9430(&UNK_142b13748,0x22,&UNK_142b13770);
        goto code_r0x000140be5fb9;
      }
      lVar9 = *(longlong *)(param_1 + 0x50);
      plVar1 = (longlong *)(lVar9 + 0x10);
      LOCK();
      bVar17 = *(char *)(lVar9 + 0x10) == '\0';
      if (bVar17) {
        *(char *)(lVar9 + 0x10) = '\x01';
      }
      UNLOCK();
      if (!bVar17) {
        FUN_1428cd210(plVar1,plVar13 == (longlong *)0x0 || lVar8 == 0,1000000000);
      }
      FUN_140c81c70(plVar1);
      uVar16 = *(ulonglong *)(lVar9 + 0x28);
      plStack_68 = plVar1;
      if (uVar16 == 0) {
        uVar15 = *(undefined8 *)(lVar9 + 0x20);
        uVar16 = 0;
      }
      else {
        if (4 < *puRam000000014362c3a8) {
          uStack_60 = &uStack_70;
          uStack_58 = 0x40d8d6c0;
          uStack_54 = 1;
          uStack_70 = uVar16;
          plVar13 = (longlong *)func_0x00014067dd80(&UNK_142b137c8);
          lStack_e8 = *plVar13;
          uStack_e0 = (undefined4)plVar13[1];
          uStack_dc = *(undefined4 *)((longlong)plVar13 + 0xc);
          uStack_bc = (undefined4)plVar13[2];
          uStack_d8 = 5;
          puStack_d0 = &UNK_142b137e0;
          uStack_c8 = 0x15;
          puStack_b8 = &UNK_142b137a8;
          uStack_b0 = 2;
          puStack_a8 = &uStack_60;
          uStack_a0 = 1;
          uStack_98 = 0;
          uStack_108 = (undefined *)0x0;
          plStack_100 = (longlong *)&UNK_142b137e0;
          puStack_f8 = (undefined8 *)0x15;
          uStack_f0 = 0;
          uStack_c0 = 1;
          func_0x000140d4a330(&uStack_49,&uStack_108);
          uVar16 = *(ulonglong *)(lVar9 + 0x28);
        }
        uVar15 = *(undefined8 *)(lVar9 + 0x20);
        if ((ulonglong)(*(longlong *)(param_1 + 0x60) - *(longlong *)(param_1 + 0x70)) < uVar16) {
          FUN_1428ca940(lStack_80,*(longlong *)(param_1 + 0x70),uVar16,8,8);
        }
      }
      func_0x000142923c00(*(longlong *)(param_1 + 0x70) * 8 + *(longlong *)(param_1 + 0x68),uVar15,
                          uVar16 * 8);
      plVar13 = plStack_68;
      *(longlong *)(param_1 + 0x70) = *(longlong *)(param_1 + 0x70) + uVar16;
      *(undefined8 *)(lVar9 + 0x28) = 0;
      FUN_140c81e10();
      LOCK();
      bVar17 = (char)*plVar13 == '\x01';
      if (bVar17) {
        *(char *)plVar13 = '\0';
      }
      UNLOCK();
      if (!bVar17) {
        FUN_1428cdb90(plVar13,0);
      }
      if (*(longlong *)(param_1 + 0x70) == 0) {
        uStack_54 = 0;
        uStack_60._4_4_ = 0;
        uStack_58 = 0;
        plStack_100 = (longlong *)0x0;
        lStack_e8 = *(longlong *)(param_1 + 8);
        uStack_108 = (undefined *)CONCAT44((int)uStack_60,0x21);
        puStack_f8 = (undefined8 *)0x6400000000;
        uStack_f0 = CONCAT44(uStack_f0._4_4_,*(undefined4 *)(param_1 + 0x10));
        iVar12 = (*pcVar4)(uVar6,&uStack_108,0,&uStack_60);
        if (iVar12 != 0) {
          uStack_60._0_4_ = iVar12;
          FUN_1428d9760(&UNK_142b13640,0x2b,&uStack_60,&UNK_142b13620,&UNK_142b13810);
          goto code_r0x000140be5fb9;
        }
        uVar15 = CONCAT44(uStack_60._4_4_,(int)uStack_60);
        uStack_60._4_4_ = CONCAT31(uStack_60._5_3_,0xf);
        FUN_1418c1380(0x143634e40,&uStack_60,5);
        lVar8 = *(longlong *)(param_1 + 0x70);
        if (lVar8 == *(longlong *)(param_1 + 0x60)) {
          FUN_140562870(lStack_80,&UNK_142b13828);
        }
        *(undefined8 *)(*(longlong *)(param_1 + 0x68) + lVar8 * 8) = uVar15;
        *(longlong *)(param_1 + 0x70) = lVar8 + 1;
      }
      else {
        puVar14 = *(undefined8 **)(param_1 + 0x68);
        puVar2 = puVar14 + *(longlong *)(param_1 + 0x70);
        while (puVar14 != puVar2) {
          uStack_108._0_5_ = CONCAT14(0x10,*(undefined4 *)puVar14);
          FUN_1418c1380(0x143634e40,&uStack_108,5);
          uVar15 = *puVar14;
          puVar14 = puVar14 + 1;
          iVar12 = (*pcVar5)(uVar6,uVar15,0);
          if (iVar12 != 0) {
            uStack_108 = (undefined *)CONCAT44(uStack_108._4_4_,iVar12);
            FUN_1428d9760(&UNK_142b13640,0x2b,&uStack_108,&UNK_142b13620,&UNK_142b137f8);
            goto code_r0x000140be5fb9;
          }
        }
      }
    }
    puStack_f8 = (undefined8 *)*plVar13;
    uStack_108 = (undefined *)CONCAT44(uStack_108._4_4_,0x22);
    plStack_100 = (longlong *)0x0;
    uStack_f0 = CONCAT44(uStack_f0._4_4_,1);
    lStack_e8 = param_1 + 0x48;
    iVar12 = (*pcVar7)(uVar6,&uStack_108,&uStack_88);
    uStack_70 = CONCAT44(uStack_70._4_4_,iVar12);
    if ((iVar12 != -0x3b9bd788) && (iVar12 != -0xc)) break;
    lVar8 = *(longlong *)(param_1 + 0x70);
    if (lVar8 == 0) {
      FUN_1428d9310(&UNK_142b136c8);
      goto code_r0x000140be5fb9;
    }
    *(longlong *)(param_1 + 0x70) = lVar8 + -1;
    plVar13 = *(longlong **)(param_1 + 0x50);
    puVar10 = *(undefined **)(*(longlong *)(param_1 + 0x68) + -8 + lVar8 * 8);
    LOCK();
    lVar8 = *plVar13;
    *plVar13 = *plVar13 + 1;
    UNLOCK();
    if (*plVar13 == 0 || SCARRY8(lVar8,1) != *plVar13 < 0) goto code_r0x000140be5fb9;
    lVar8 = *(longlong *)(param_3 + 0x28);
    uStack_108 = puVar10;
    plStack_100 = plVar13;
    plStack_68 = plVar13;
    if (lVar8 == *(longlong *)(param_3 + 0x18)) {
      FUN_140562630(param_3 + 0x18,&UNK_142b136e0);
    }
    lVar9 = *(longlong *)(param_3 + 0x20);
    *(undefined **)(lVar9 + lVar8 * 0x10) = puVar10;
    *(longlong **)(lVar9 + 8 + lVar8 * 0x10) = plStack_68;
    *(longlong *)(param_3 + 0x28) = lVar8 + 1;
  }
  if (iVar12 == 0) {
    uStack_108._0_5_ = CONCAT14(0x11,*(undefined4 *)plVar13);
    FUN_1418c1380(0x143634e40,&uStack_108,5);
    pcVar11 = pcStack_78;
    uVar6 = uStack_88;
    FUN_140c81e10();
    LOCK();
    cVar3 = *pcVar11;
    if (cVar3 == '\x01') {
      *pcVar11 = '\0';
    }
    UNLOCK();
    if (cVar3 != '\x01') {
      FUN_1428cdb90(pcVar11,0);
    }
    return uVar6;
  }
  uStack_60 = &uStack_70;
  uStack_58 = 0x40c8f550;
  uStack_54 = 1;
  uStack_108 = &UNK_142b13720;
  plStack_100 = (longlong *)0x1;
  lStack_e8 = 0;
  puStack_f8 = &uStack_60;
  uStack_f0 = 1;
  FUN_1428d9390(&uStack_108,&UNK_142b13730);
code_r0x000140be5fb9:
                    /* WARNING: Does not return */
  pcVar4 = (code *)invalidInstructionException();
  (*pcVar4)();
}

