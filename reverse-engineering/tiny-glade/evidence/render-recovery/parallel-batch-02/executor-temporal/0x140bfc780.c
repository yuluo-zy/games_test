
longlong * FUN_140bfc780(byte *param_1,byte *param_2)

{
  byte bVar1;
  byte bVar2;
  byte bVar3;
  code *pcVar4;
  char cVar5;
  int iVar6;
  longlong lVar7;
  undefined *puVar8;
  undefined *puVar9;
  longlong *plVar10;
  byte *pbVar11;
  uint uVar12;
  ulonglong uVar13;
  byte *pbVar14;
  ulonglong uVar15;
  undefined *puVar16;
  undefined *puVar17;
  undefined1 uVar18;
  bool bVar19;
  undefined *puStack_158;
  undefined8 uStack_150;
  undefined *puStack_148;
  undefined8 uStack_140;
  undefined8 uStack_138;
  undefined4 uStack_130;
  undefined4 uStack_12c;
  undefined4 uStack_128;
  undefined4 uStack_124;
  undefined4 uStack_120;
  undefined4 uStack_11c;
  byte *pbStack_118;
  byte *pbStack_110;
  undefined4 uStack_108;
  undefined4 uStack_104;
  undefined4 uStack_100;
  undefined4 uStack_fc;
  int iStack_e8;
  undefined4 uStack_e4;
  undefined8 uStack_c0;
  undefined *puStack_a8;
  undefined8 uStack_a0;
  undefined8 uStack_98;
  undefined8 uStack_90;
  undefined8 uStack_88;
  undefined8 uStack_68;
  longlong lStack_58;
  undefined8 uStack_50;
  byte *pbStack_48;
  undefined8 uStack_40;
  
  uStack_40 = 0xfffffffffffffffe;
  cVar5 = func_0x000140c16a00();
  if (cVar5 == '\0') {
code_r0x000140bfc812:
    lStack_58 = 0;
  }
  else {
    puVar16 = &UNK_142b153dc;
    pbVar14 = (byte *)0x55;
    uVar13 = 0x55;
    lVar7 = FUN_140bed840(&UNK_142b153dc);
    if (lVar7 == 1) {
      if (uVar13 != 0) {
        if (uVar13 < 0x55) {
          if ((char)(&UNK_142b153dc)[uVar13] < -0x40) {
code_r0x000140bfc7f2:
            FUN_1428d9940(&UNK_142b153dc,0x55,0,uVar13,&UNK_142b155f8);
            goto code_r0x000140bfc812;
          }
        }
        else if (uVar13 != 0x55) goto code_r0x000140bfc7f2;
      }
      puVar16 = &UNK_142b153dc;
      lVar7 = FUN_140bed840(&UNK_142b153dc);
      if (lVar7 == 1) {
        uVar15 = uVar13 + 2;
        if (uVar15 != 0) {
          uVar18 = uVar15 == 0x55;
          if (0x54 < uVar15) goto code_r0x000140bfc873;
          uVar18 = (&UNK_142b153de)[uVar13] == -0x41;
          if ((char)(&UNK_142b153de)[uVar13] < -0x40) {
            do {
              uVar13 = 0x55;
              FUN_1428d9940(&UNK_142b153dc);
code_r0x000140bfc873:
            } while (!(bool)uVar18);
          }
        }
        pbVar14 = (byte *)(0x53 - uVar13);
        puVar16 = &UNK_142b153dc + uVar15;
      }
    }
    puVar8 = &UNK_142b16c59;
    puVar17 = &UNK_142b16c38;
    do {
      bVar1 = puVar8[-1];
      uVar12 = (uint)(char)bVar1;
      if ((char)bVar1 < '\0') {
        bVar2 = puVar8[-2];
        if ((char)bVar2 < -0x40) {
          bVar3 = puVar8[-3];
          if ((char)bVar3 < -0x40) {
            pbVar11 = puVar8 + -4;
            puVar8 = puVar8 + -4;
            uVar12 = bVar3 & 0x3f | (*pbVar11 & 7) << 6;
          }
          else {
            puVar8 = puVar8 + -3;
            uVar12 = bVar3 & 0xf;
          }
          uVar12 = bVar2 & 0x3f | uVar12 << 6;
        }
        else {
          puVar8 = puVar8 + -2;
          uVar12 = bVar2 & 0x1f;
        }
        uVar12 = (uint)(bVar1 & 0x3f) | uVar12 << 6;
        if (uVar12 != 0x5c) goto code_r0x000140bfc919;
code_r0x000140bfc92f:
        puVar9 = puVar8 + -0x142b16c38;
        puVar17 = puVar8 + -0x142b16c37;
        if (puVar17 != (undefined *)0x0) {
          uVar18 = puVar17 == (undefined *)0x21;
          if ((undefined *)0x20 < puVar17) goto code_r0x000140bfc971;
          uVar18 = puVar8[1] == -0x41;
          if ((char)puVar8[1] < -0x40) {
            do {
              puVar9 = (undefined *)FUN_1428d9940(&UNK_142b16c38,0x21);
code_r0x000140bfc971:
            } while (!(bool)uVar18);
          }
        }
        lVar7 = 0x20 - (longlong)puVar9;
        puVar17 = &UNK_142b16c38 + (longlong)puVar17;
        goto code_r0x000140bfc97f;
      }
      puVar8 = puVar8 + -1;
      if (uVar12 == 0x5c) goto code_r0x000140bfc92f;
code_r0x000140bfc919:
      if (uVar12 == 0x2f) goto code_r0x000140bfc92f;
    } while (puVar8 != &UNK_142b16c38);
    lVar7 = 0x21;
code_r0x000140bfc97f:
    plVar10 = (longlong *)func_0x000140c18130();
    if (*plVar10 == 1) {
      pbVar11 = (byte *)(plVar10 + 1);
    }
    else {
      if (*plVar10 != 0) goto code_r0x000140bfcbf1;
      pbVar11 = (byte *)FUN_1429090d0(plVar10,0);
      if (pbVar11 == (byte *)0x0) goto code_r0x000140bfcbf1;
    }
    if (*(longlong *)pbVar11 != 0) goto code_r0x000140bfcbfd;
    pbVar11[0] = 0xff;
    pbVar11[1] = 0xff;
    pbVar11[2] = 0xff;
    pbVar11[3] = 0xff;
    pbVar11[4] = 0xff;
    pbVar11[5] = 0xff;
    pbVar11[6] = 0xff;
    pbVar11[7] = 0xff;
    pbStack_48 = pbVar11;
    uStack_50 = FUN_140c16d40(pbVar11 + 8,puVar16,pbVar14,puVar17,lVar7,1,0);
    *(longlong *)pbStack_48 = *(longlong *)pbStack_48 + 1;
    lStack_58 = 1;
  }
  uStack_108 = CONCAT31(uStack_108._1_3_,0x1c);
  FUN_1418c1380(0x143634e40,&uStack_108,1);
  pbVar14 = param_1 + 0x718;
  LOCK();
  bVar19 = param_1[0x718] == 0;
  if (bVar19) {
    param_1[0x718] = 1;
  }
  UNLOCK();
  if (!bVar19) goto code_r0x000140bfcc09;
  do {
    FUN_140c81c70(pbVar14);
    uStack_fc = 0;
    uStack_104 = 0;
    uStack_100 = 0;
    pbStack_48 = pbVar14;
    if ((*param_2 & 1) == 0) {
      pbStack_118 = param_1 + 0x730;
    }
    else {
      uVar13 = (ulonglong)*(uint *)(param_2 + 4);
      if (*(ulonglong *)(param_1 + 0x7f0) <= uVar13) {
        FUN_1428d9518(uVar13,*(ulonglong *)(param_1 + 0x7f0),&UNK_142d06788);
        goto code_r0x000140bfcd30;
      }
      pbStack_118 = (byte *)(uVar13 * 0xb0 + *(longlong *)(param_1 + 0x7e8));
    }
    pbStack_118 = pbStack_118 + 0x88;
    uStack_12c = uStack_108;
    uStack_128 = 0;
    uStack_124 = 0;
    uStack_120 = 0;
    param_2 = param_2 + 8;
    uStack_130 = 0x3b9df29c;
    uStack_11c = 1;
    pbStack_110 = param_2;
    iVar6 = (**(code **)(param_1 + 0x4f8))(*(undefined8 *)(param_1 + 0x640),&uStack_130,10000000000)
    ;
    if (iVar6 != 0) {
      FUN_140c84010(&uStack_100);
      uStack_108 = 9;
      uStack_104 = 0;
      iStack_e8 = iVar6;
      if ((iVar6 == -4) || (iVar6 == 2)) {
        if (*plRam000000014362c3a8 != 0) {
          puStack_a8 = &UNK_142b15d20;
          uStack_a0 = 1;
          uStack_98 = 8;
          uStack_90 = 0;
          uStack_88 = 0;
          uStack_138 = func_0x00014067dd80(&UNK_142b15d50);
          puStack_158 = &UNK_142b15d68;
          uStack_150 = 0x14;
          puStack_148 = &UNK_142b15d68;
          uStack_140 = 0x14;
          FUN_1404b1430(&puStack_a8,1,&puStack_158);
        }
        FUN_140befd60(param_1);
      }
      uStack_68 = uStack_c0;
      puStack_a8 = (undefined *)CONCAT44(uStack_fc,uStack_100);
      uStack_90 = CONCAT44(uStack_e4,iStack_e8);
      uStack_108 = 9;
      uStack_104 = 0;
      FUN_1428d9760(&UNK_142b15188,0x2b,&uStack_108,&UNK_142b151f8,&UNK_142b16c78);
code_r0x000140bfcd30:
                    /* WARNING: Does not return */
      pcVar4 = (code *)invalidInstructionException();
      (*pcVar4)();
    }
    FUN_140c81e10();
    LOCK();
    bVar1 = *pbVar14;
    if (bVar1 == 1) {
      *pbVar14 = 0;
    }
    UNLOCK();
    if (bVar1 != 1) {
      FUN_1428cdb90(pbVar14,0);
    }
    FUN_1418c0a70(*(undefined8 *)(param_1 + 0x648),param_1);
    LOCK();
    bVar1 = *pbVar14;
    if (bVar1 == 0) {
      *pbVar14 = 1;
    }
    UNLOCK();
    if (bVar1 != 0) {
      FUN_1428cd210(pbVar14);
    }
    FUN_140c81c70(pbVar14);
    FUN_140bf0cc0(param_1 + 0x730,param_1);
    if (*(longlong *)(param_1 + 0x7f0) != 0) {
      param_2 = *(byte **)(param_1 + 0x7e8);
      pbVar14 = param_2 + *(longlong *)(param_1 + 0x7f0) * 0xb0;
      do {
        FUN_140bf0cc0(param_2,param_1);
        param_2 = param_2 + 0xb0;
      } while (param_2 != pbVar14);
    }
    param_1 = pbStack_48;
    FUN_140c81e10();
    plVar10 = (longlong *)0x1;
    LOCK();
    bVar1 = *param_1;
    if (bVar1 == 1) {
      *param_1 = 0;
    }
    else {
      plVar10 = (longlong *)(ulonglong)bVar1;
    }
    UNLOCK();
    if (bVar1 != 1) {
      plVar10 = (longlong *)FUN_1428cdb90(param_1,0);
    }
    if (lStack_58 == 0) {
      return plVar10;
    }
    plVar10 = (longlong *)func_0x000140c18130();
    if (*plVar10 == 1) {
      plVar10 = plVar10 + 1;
code_r0x000140bfcb83:
      if (*plVar10 == 0) {
        *plVar10 = -1;
        pbStack_48 = (byte *)plVar10;
        FUN_140c170f0(plVar10 + 1,uStack_50);
        *(longlong *)pbStack_48 = *(longlong *)pbStack_48 + 1;
        return (longlong *)pbStack_48;
      }
    }
    else {
      if ((*plVar10 == 0) &&
         (plVar10 = (longlong *)FUN_1429090d0(plVar10,0), plVar10 != (longlong *)0x0))
      goto code_r0x000140bfcb83;
code_r0x000140bfcbf1:
      FUN_1428d80d0(&UNK_142d06498);
    }
code_r0x000140bfcbfd:
    FUN_1428d9250(&UNK_142d06508);
code_r0x000140bfcc09:
    FUN_1428cd210(pbVar14);
  } while( true );
}

