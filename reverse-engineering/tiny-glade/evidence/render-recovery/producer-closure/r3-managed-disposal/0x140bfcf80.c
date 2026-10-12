
longlong * FUN_140bfcf80(longlong param_1,byte *param_2,ulonglong param_3,uint param_4)

{
  byte bVar1;
  byte bVar2;
  code *pcVar3;
  char cVar4;
  int iVar5;
  longlong lVar6;
  undefined *puVar7;
  undefined *puVar8;
  undefined8 uVar9;
  ulonglong uVar10;
  ulonglong uVar11;
  longlong *plVar12;
  byte *pbVar13;
  undefined4 uVar14;
  ulonglong uVar15;
  byte *pbVar16;
  ulonglong uVar17;
  ulonglong uVar18;
  undefined *puVar19;
  undefined *puVar20;
  uint uVar21;
  longlong lVar22;
  bool bVar23;
  undefined1 uVar24;
  byte bVar25;
  longlong lStack_80;
  undefined8 uStack_78;
  int iStack_6c;
  undefined4 uStack_68;
  int iStack_64;
  ulonglong uStack_60;
  byte *pbStack_58;
  longlong *plStack_50;
  undefined8 uStack_48;
  
  uStack_48 = 0xfffffffffffffffe;
  pbVar16 = (byte *)(ulonglong)param_4;
  pbVar13 = param_2;
  cVar4 = func_0x000140c16a00();
  uVar14 = SUB84(pbVar13,0);
  plStack_50 = (longlong *)param_3;
  if (cVar4 == '\0') {
code_r0x000140bfd01c:
    lStack_80 = 0;
    if (param_3 == 0 && param_4 < 1000000) {
      return (longlong *)(ulonglong)(param_3 == 0);
    }
code_r0x000140bfd263:
    FUN_140d635e0();
    plVar12 = plRam000000014362c250;
    uVar15 = *(ulonglong *)(param_2 + 8);
    pbVar16 = (byte *)(param_1 + 0x718);
    bVar2 = *param_2;
    uVar18 = (ulonglong)*(uint *)(param_2 + 4);
    iStack_64 = param_4 + 1000000000;
    uStack_68 = uVar14;
    pbStack_58 = pbVar16;
code_r0x000140bfd2c0:
    LOCK();
    bVar25 = *pbVar16;
    if (bVar25 == 0) {
      *pbVar16 = 1;
    }
    UNLOCK();
    if (bVar25 != 0) {
      FUN_1428cd210(pbVar16);
    }
    FUN_140c81c70(pbVar16);
    lVar6 = param_1 + 0x730;
    if ((bVar2 & 1) != 0) {
      if (*(ulonglong *)(param_1 + 0x7f0) <= uVar18) {
        FUN_1428d9518(uVar18,*(ulonglong *)(param_1 + 0x7f0),&UNK_142d06788);
        goto code_r0x000140bfd5b8;
      }
      lVar6 = *(longlong *)(param_1 + 0x7e8) + uVar18 * 0xb0;
    }
    uVar9 = *(undefined8 *)(lVar6 + 0x88);
    iVar5 = (**(code **)(param_1 + 0x4f0))(*(undefined8 *)(param_1 + 0x640),uVar9,&uStack_60);
    uVar21 = (uint)uVar9;
    if (iVar5 != 0) {
      iStack_6c = iVar5;
      FUN_1428d9760(&UNK_142b15188,0x2b,&iStack_6c,&UNK_142b15128,&UNK_142b16cb8);
      goto code_r0x000140bfd5b8;
    }
    if (uVar15 <= uStack_60) {
code_r0x000140bfd465:
      pbVar16 = pbStack_58;
      FUN_140c81e10();
      plVar12 = (longlong *)0x1;
      LOCK();
      bVar2 = *pbVar16;
      if (bVar2 == 1) {
        *pbVar16 = 0;
      }
      else {
        plVar12 = (longlong *)(ulonglong)bVar2;
      }
      UNLOCK();
      if (bVar2 != 1) goto code_r0x000140bfd552;
      goto code_r0x000140bfd47f;
    }
    uVar9 = FUN_140d635e0();
    uVar10 = FUN_140d63b00(uVar9);
    bVar25 = plStack_50 != (longlong *)uVar10;
    if (uVar10 < plStack_50) {
      bVar25 = 0xff;
    }
    bVar1 = -(uVar21 < param_4);
    if (bVar25 != 0) {
      bVar1 = bVar25;
    }
    if (bVar1 < 2) goto code_r0x000140bfd465;
    uVar17 = (longlong)plStack_50 - uVar10;
    if (plStack_50 < uVar10) {
code_r0x000140bfd561:
      FUN_1428d9330(&UNK_142b152eb,0x23,&UNK_142b15310);
      goto code_r0x000140bfd5b8;
    }
    if (param_4 < uVar21) {
      if (uVar17 == 0) goto code_r0x000140bfd561;
      uVar17 = uVar17 - 1;
      uVar21 = iStack_64 - uVar21;
      if (999999999 < uVar21) goto code_r0x000140bfd3bf;
code_r0x000140bfd399:
      lVar6 = *plVar12;
    }
    else {
      uVar21 = param_4 - uVar21;
      if (uVar21 < 1000000000) goto code_r0x000140bfd399;
code_r0x000140bfd3bf:
      uVar10 = (ulonglong)(uVar21 >> 9) * 0x44b83;
      uVar11 = uVar10 >> 0x27;
      bVar23 = CARRY8(uVar17,uVar11);
      uVar17 = uVar17 + uVar11;
      if (bVar23) goto code_r0x000140bfd52c;
      uVar21 = uVar21 + (uint)(uVar10 >> 0x27) * -1000000000;
      lVar6 = *plVar12;
    }
    if (lVar6 != 2) {
      FUN_142909350(plVar12,plVar12);
    }
    func_0x0001429234a0(*(undefined4 *)((longlong)plVar12 + 0xc));
    FUN_140d55980(uVar17,uVar21);
    if (*plVar12 != 2) {
      FUN_142909350(plVar12,plVar12);
    }
    func_0x0001429234b0(*(undefined4 *)((longlong)plVar12 + 0xc));
    pbVar16 = pbStack_58;
    FUN_140c81e10();
    LOCK();
    bVar25 = *pbVar16;
    if (bVar25 == 1) {
      *pbVar16 = 0;
    }
    UNLOCK();
    if (bVar25 != 1) {
      FUN_1428cdb90(pbVar16,0);
    }
    goto code_r0x000140bfd2c0;
  }
  puVar19 = &UNK_142b15434;
  lVar22 = 0x67;
  uVar15 = 0x67;
  lVar6 = FUN_140bed840(&UNK_142b15434);
  if (lVar6 == 1) {
    if (uVar15 != 0) {
      if (uVar15 < 0x67) {
        if ((char)(&UNK_142b15434)[uVar15] < -0x40) {
code_r0x000140bfcffc:
          uVar14 = 0x67;
          FUN_1428d9940(&UNK_142b15434,0x67,0,uVar15,&UNK_142b155f8);
          goto code_r0x000140bfd01c;
        }
      }
      else if (uVar15 != 0x67) goto code_r0x000140bfcffc;
    }
    puVar19 = &UNK_142b15434;
    lVar6 = FUN_140bed840(&UNK_142b15434);
    if (lVar6 == 1) {
      uVar18 = uVar15 + 2;
      if (uVar18 != 0) {
        uVar24 = uVar18 == 0x67;
        if (0x66 < uVar18) goto code_r0x000140bfd09a;
        uVar24 = (&UNK_142b15436)[uVar15] == -0x41;
        if ((char)(&UNK_142b15436)[uVar15] < -0x40) {
          do {
            uVar15 = 0x67;
            FUN_1428d9940(&UNK_142b15434,0x67);
code_r0x000140bfd09a:
          } while (!(bool)uVar24);
        }
      }
      lVar22 = 0x65 - uVar15;
      puVar19 = &UNK_142b15434 + uVar18;
    }
  }
  puVar7 = &UNK_142b16c59;
  puVar20 = &UNK_142b16c38;
  do {
    bVar2 = puVar7[-1];
    uVar21 = (uint)(char)bVar2;
    if ((char)bVar2 < '\0') {
      bVar25 = puVar7[-2];
      if ((char)bVar25 < -0x40) {
        bVar1 = puVar7[-3];
        if ((char)bVar1 < -0x40) {
          pbVar13 = puVar7 + -4;
          puVar7 = puVar7 + -4;
          uVar21 = bVar1 & 0x3f | (*pbVar13 & 7) << 6;
        }
        else {
          puVar7 = puVar7 + -3;
          uVar21 = bVar1 & 0xf;
        }
        uVar21 = bVar25 & 0x3f | uVar21 << 6;
      }
      else {
        puVar7 = puVar7 + -2;
        uVar21 = bVar25 & 0x1f;
      }
      uVar21 = (uint)(bVar2 & 0x3f) | uVar21 << 6;
      if (uVar21 != 0x5c) goto code_r0x000140bfd139;
code_r0x000140bfd14f:
      puVar8 = puVar7 + -0x142b16c38;
      puVar20 = puVar7 + -0x142b16c37;
      if (puVar20 != (undefined *)0x0) {
        uVar24 = puVar20 == (undefined *)0x21;
        if ((undefined *)0x20 < puVar20) goto code_r0x000140bfd191;
        uVar24 = puVar7[1] == -0x41;
        if ((char)puVar7[1] < -0x40) {
          do {
            puVar8 = (undefined *)FUN_1428d9940(&UNK_142b16c38,0x21);
code_r0x000140bfd191:
          } while (!(bool)uVar24);
        }
      }
      lVar6 = 0x20 - (longlong)puVar8;
      puVar20 = &UNK_142b16c38 + (longlong)puVar20;
      goto code_r0x000140bfd19f;
    }
    puVar7 = puVar7 + -1;
    if (uVar21 == 0x5c) goto code_r0x000140bfd14f;
code_r0x000140bfd139:
    if (uVar21 == 0x2f) goto code_r0x000140bfd14f;
  } while (puVar7 != &UNK_142b16c38);
  lVar6 = 0x21;
code_r0x000140bfd19f:
  plVar12 = (longlong *)func_0x000140c18130();
  if (*plVar12 == 1) {
    pbVar13 = (byte *)(plVar12 + 1);
  }
  else if ((*plVar12 != 0) || (pbVar13 = (byte *)FUN_1429090d0(plVar12,0), pbVar13 == (byte *)0x0))
  goto code_r0x000140bfd520;
  if (*(longlong *)pbVar13 == 0) {
    pbVar13[0] = 0xff;
    pbVar13[1] = 0xff;
    pbVar13[2] = 0xff;
    pbVar13[3] = 0xff;
    pbVar13[4] = 0xff;
    pbVar13[5] = 0xff;
    pbVar13[6] = 0xff;
    pbVar13[7] = 0xff;
    pbStack_58 = pbVar13;
    uStack_78 = FUN_140c16d40(pbVar13 + 8,puVar19,lVar22,puVar20,lVar6,&UNK_142b16c90,0x22);
    uVar14 = SUB84(puVar19,0);
    *(longlong *)pbStack_58 = *(longlong *)pbStack_58 + 1;
    lStack_80 = 1;
    if (plStack_50 != (longlong *)0x0 || 999999 < param_4) goto code_r0x000140bfd263;
    plVar12 = (longlong *)func_0x000140c18130();
    if (*plVar12 == 1) {
      plVar12 = plVar12 + 1;
    }
    else if ((*plVar12 != 0) ||
            (plVar12 = (longlong *)FUN_1429090d0(plVar12,0), plVar12 == (longlong *)0x0)) {
code_r0x000140bfd520:
      FUN_1428d80d0(&UNK_142d06498);
code_r0x000140bfd52c:
      FUN_1428d9330(&UNK_142b150a8,0x19,&UNK_142b15110);
code_r0x000140bfd5b8:
                    /* WARNING: Does not return */
      pcVar3 = (code *)invalidInstructionException();
      (*pcVar3)();
    }
    if (*plVar12 == 0) {
      *plVar12 = -1;
      plStack_50 = plVar12;
      FUN_140c170f0(plVar12 + 1,uStack_78);
      goto code_r0x000140bfd4bd;
    }
  }
  do {
    FUN_1428d9250(&UNK_142d06508);
code_r0x000140bfd552:
    plVar12 = (longlong *)FUN_1428cdb90(pbVar16);
code_r0x000140bfd47f:
    if (lStack_80 == 0) {
      return plVar12;
    }
    plVar12 = (longlong *)func_0x000140c18130();
    if (*plVar12 == 1) {
      plVar12 = plVar12 + 1;
    }
    else if ((*plVar12 != 0) ||
            (plVar12 = (longlong *)FUN_1429090d0(plVar12,0), plVar12 == (longlong *)0x0))
    goto code_r0x000140bfd520;
  } while (*plVar12 != 0);
  *plVar12 = -1;
  plStack_50 = plVar12;
  FUN_140c170f0(plVar12 + 1,uStack_78);
code_r0x000140bfd4bd:
  *plStack_50 = *plStack_50 + 1;
  return plStack_50;
}

