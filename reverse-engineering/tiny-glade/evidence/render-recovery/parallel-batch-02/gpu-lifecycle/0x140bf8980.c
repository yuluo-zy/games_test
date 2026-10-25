
longlong * FUN_140bf8980(longlong param_1)

{
  byte *pbVar1;
  byte bVar2;
  byte bVar3;
  byte bVar4;
  char cVar5;
  longlong lVar6;
  undefined *puVar7;
  undefined *puVar8;
  undefined4 *puVar9;
  longlong *plVar10;
  uint uVar11;
  ulonglong uVar12;
  undefined *puVar13;
  longlong *plVar14;
  ulonglong uVar15;
  undefined *puVar16;
  undefined1 uVar17;
  bool bVar18;
  undefined8 uStack_e8;
  undefined *puStack_e0;
  undefined8 uStack_d8;
  undefined8 uStack_d0;
  undefined4 uStack_c8;
  undefined4 uStack_c4;
  undefined4 uStack_c0;
  undefined4 uStack_bc;
  undefined8 uStack_b8;
  undefined *puStack_b0;
  undefined8 uStack_a8;
  undefined4 uStack_a0;
  undefined4 uStack_9c;
  undefined *puStack_98;
  undefined8 uStack_90;
  undefined8 uStack_88;
  undefined8 uStack_80;
  undefined8 uStack_78;
  longlong lStack_58;
  undefined8 uStack_50;
  longlong *plStack_48;
  undefined1 uStack_39;
  undefined8 uStack_38;
  
  uStack_38 = 0xfffffffffffffffe;
  cVar5 = func_0x000140c16a00();
  if (cVar5 == '\0') {
code_r0x000140bf8a0a:
    lStack_58 = 0;
  }
  else {
    puVar13 = &UNK_142b154fe;
    plVar14 = (longlong *)0x30;
    uVar12 = 0x30;
    lVar6 = FUN_140bed840(&UNK_142b154fe);
    if (lVar6 == 1) {
      if (uVar12 != 0) {
        if (uVar12 < 0x30) {
          if ((char)(&UNK_142b154fe)[uVar12] < -0x40) {
code_r0x000140bf89ea:
            FUN_1428d9940(&UNK_142b154fe,0x30,0,uVar12,&UNK_142b155f8);
            goto code_r0x000140bf8a0a;
          }
        }
        else if (uVar12 != 0x30) goto code_r0x000140bf89ea;
      }
      puVar13 = &UNK_142b154fe;
      lVar6 = FUN_140bed840(&UNK_142b154fe);
      if (lVar6 == 1) {
        uVar15 = uVar12 + 2;
        if (uVar15 != 0) {
          uVar17 = uVar15 == 0x30;
          if (0x2f < uVar15) goto code_r0x000140bf8a6b;
          uVar17 = (&UNK_142b15500)[uVar12] == -0x41;
          if ((char)(&UNK_142b15500)[uVar12] < -0x40) {
            do {
              uVar12 = 0x30;
              FUN_1428d9940(&UNK_142b154fe);
code_r0x000140bf8a6b:
            } while (!(bool)uVar17);
          }
        }
        plVar14 = (longlong *)(0x2e - uVar12);
        puVar13 = &UNK_142b154fe + uVar15;
      }
    }
    puVar7 = &UNK_142b15f39;
    puVar16 = &UNK_142b15f18;
    do {
      bVar2 = puVar7[-1];
      uVar11 = (uint)(char)bVar2;
      if ((char)bVar2 < '\0') {
        bVar3 = puVar7[-2];
        if ((char)bVar3 < -0x40) {
          bVar4 = puVar7[-3];
          if ((char)bVar4 < -0x40) {
            pbVar1 = puVar7 + -4;
            puVar7 = puVar7 + -4;
            uVar11 = bVar4 & 0x3f | (*pbVar1 & 7) << 6;
          }
          else {
            puVar7 = puVar7 + -3;
            uVar11 = bVar4 & 0xf;
          }
          uVar11 = bVar3 & 0x3f | uVar11 << 6;
        }
        else {
          puVar7 = puVar7 + -2;
          uVar11 = bVar3 & 0x1f;
        }
        uVar11 = (uint)(bVar2 & 0x3f) | uVar11 << 6;
        if (uVar11 != 0x5c) goto code_r0x000140bf8b09;
code_r0x000140bf8b1f:
        puVar8 = puVar7 + -0x142b15f18;
        puVar16 = puVar7 + -0x142b15f17;
        if (puVar16 != (undefined *)0x0) {
          uVar17 = puVar16 == (undefined *)0x21;
          if ((undefined *)0x20 < puVar16) goto code_r0x000140bf8b61;
          uVar17 = puVar7[1] == -0x41;
          if ((char)puVar7[1] < -0x40) {
            do {
              puVar8 = (undefined *)FUN_1428d9940(&UNK_142b15f18,0x21);
code_r0x000140bf8b61:
            } while (!(bool)uVar17);
          }
        }
        lVar6 = 0x20 - (longlong)puVar8;
        puVar16 = &UNK_142b15f18 + (longlong)puVar16;
        goto code_r0x000140bf8b6f;
      }
      puVar7 = puVar7 + -1;
      if (uVar11 == 0x5c) goto code_r0x000140bf8b1f;
code_r0x000140bf8b09:
      if (uVar11 == 0x2f) goto code_r0x000140bf8b1f;
    } while (puVar7 != &UNK_142b15f18);
    lVar6 = 0x21;
code_r0x000140bf8b6f:
    plVar10 = (longlong *)func_0x000140c18130();
    if (*plVar10 == 1) {
      plVar10 = plVar10 + 1;
    }
    else {
      if (*plVar10 != 0) goto code_r0x000140bf8d70;
      plVar10 = (longlong *)FUN_1429090d0(plVar10,0);
      if (plVar10 == (longlong *)0x0) goto code_r0x000140bf8d70;
    }
    if (*plVar10 != 0) goto code_r0x000140bf8d7c;
    *plVar10 = -1;
    plStack_48 = plVar10;
    uStack_50 = FUN_140c16d40(plVar10 + 1,puVar13,plVar14,puVar16,lVar6,1,0);
    *plStack_48 = *plStack_48 + 1;
    lStack_58 = 1;
  }
  uStack_e8 = CONCAT71(uStack_e8._1_7_,5);
  FUN_1418c1380(0x143634e40,&uStack_e8,1);
  if (4 < *puRam000000014362c3a8) {
    puVar9 = (undefined4 *)func_0x00014067dd80(&UNK_142b163b0);
    uStack_c8 = *puVar9;
    uStack_c4 = puVar9[1];
    uStack_c0 = puVar9[2];
    uStack_bc = puVar9[3];
    uStack_9c = puVar9[4];
    uStack_b8 = 5;
    puStack_b0 = &UNK_142b16030;
    uStack_a8 = 0x15;
    puStack_98 = &UNK_142b163a0;
    uStack_90 = 1;
    uStack_88 = 8;
    uStack_e8 = 0;
    uStack_80 = 0;
    uStack_78 = 0;
    puStack_e0 = &UNK_142b16030;
    uStack_d8 = 0x15;
    uStack_d0 = 0;
    uStack_a0 = 1;
    func_0x000140d4a330(&uStack_39,&uStack_e8);
  }
  plVar14 = (longlong *)(param_1 + 0x800);
  LOCK();
  bVar18 = *(longlong *)(param_1 + 0x800) == 0;
  if (bVar18) {
    *(longlong *)(param_1 + 0x800) = 8;
  }
  UNLOCK();
  if (!bVar18) goto code_r0x000140bf8d88;
  do {
    FUN_140c81c70(plVar14);
    FUN_140c81c70((ulonglong)plVar14 | 1);
    (**(code **)(param_1 + 0x98))(*(undefined8 *)(param_1 + 0x640));
    FUN_140c81e10(plVar14);
    FUN_140c81e10();
    LOCK();
    plVar10 = (longlong *)*plVar14;
    bVar18 = plVar10 == (longlong *)0x8;
    if (bVar18) {
      *plVar14 = 0;
      plVar10 = (longlong *)0x8;
    }
    UNLOCK();
    if (!bVar18) {
      plVar10 = (longlong *)FUN_1428cbf60(plVar14,0);
    }
    if (lStack_58 == 0) {
      return plVar10;
    }
    plVar10 = (longlong *)func_0x000140c18130();
    if (*plVar10 == 1) {
      plVar10 = plVar10 + 1;
code_r0x000140bf8d0d:
      if (*plVar10 == 0) {
        *plVar10 = -1;
        plStack_48 = plVar10;
        FUN_140c170f0(plVar10 + 1,uStack_50);
        *plStack_48 = *plStack_48 + 1;
        return plStack_48;
      }
    }
    else {
      if ((*plVar10 == 0) && (plVar10 = (longlong *)FUN_1429090d0(), plVar10 != (longlong *)0x0))
      goto code_r0x000140bf8d0d;
code_r0x000140bf8d70:
      FUN_1428d80d0(&UNK_142d06498);
    }
code_r0x000140bf8d7c:
    FUN_1428d9250(&UNK_142d06508);
code_r0x000140bf8d88:
    FUN_1428cb5f0(plVar14);
  } while( true );
}

