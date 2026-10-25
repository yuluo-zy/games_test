
undefined8 FUN_140c0e150(undefined8 param_1,ulonglong param_2,undefined4 *param_3)

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
  undefined8 unaff_RBX;
  longlong lVar11;
  ulonglong uVar12;
  undefined *puVar13;
  undefined *puVar14;
  undefined *puVar15;
  undefined4 uStack_d8;
  undefined4 uStack_d4;
  undefined4 uStack_d0;
  undefined4 uStack_cc;
  undefined4 uStack_c8;
  undefined4 uStack_c4;
  undefined4 uStack_c0;
  undefined4 uStack_bc;
  undefined4 uStack_b8;
  undefined4 uStack_b4;
  undefined4 uStack_b0;
  undefined4 uStack_ac;
  undefined4 uStack_a8;
  undefined4 uStack_a4;
  undefined4 uStack_a0;
  undefined4 uStack_9c;
  undefined8 uStack_98;
  undefined8 uStack_88;
  undefined8 uStack_80;
  undefined4 *puStack_68;
  longlong *plStack_60;
  longlong *plStack_58;
  undefined1 uStack_49;
  undefined8 uStack_48;
  
  uStack_48 = 0xfffffffffffffffe;
  uStack_49 = 1;
  uVar10 = param_2;
  puStack_68 = param_3;
  cVar6 = func_0x000140c16a00();
  plStack_58 = (longlong *)CONCAT71(plStack_58._1_7_,cVar6);
  if (cVar6 == '\0') {
    uStack_88 = 0;
  }
  else {
    uStack_49 = 1;
    lVar11 = 0x2c;
    lVar8 = FUN_140c0d550(0x2c);
    puVar14 = &UNK_142b19938;
    if (lVar8 == 1) {
      if (uVar10 != 0) {
        if (uVar10 < 0x2c) {
          if ((char)(&UNK_142b19938)[uVar10] < -0x40) {
code_r0x000140c0e1cd:
            uStack_49 = 1;
            FUN_1428d9940(&UNK_142b19938,0x2c,0,uVar10,&UNK_142b199e8);
            goto code_r0x000140c0e4da;
          }
        }
        else if (uVar10 != 0x2c) goto code_r0x000140c0e1cd;
      }
      uStack_49 = 1;
      lVar8 = FUN_140c0d550(uVar10);
      if (lVar8 == 1) {
        uVar12 = uVar10 + 2;
        if (uVar12 != 0) {
          if (uVar12 < 0x2c) {
            if ((char)(&UNK_142b1993a)[uVar10] < -0x40) {
code_r0x000140c0e22e:
              uStack_49 = 1;
              FUN_1428d9940(&UNK_142b19938,0x2c,uVar12,0x2c,&UNK_142b19a00);
              goto code_r0x000140c0e4da;
            }
          }
          else if (uVar12 != 0x2c) goto code_r0x000140c0e22e;
        }
        lVar11 = 0x2a - uVar10;
        puVar14 = &UNK_142b1993a + uVar10;
      }
    }
    puVar13 = &UNK_142b19a54;
    puVar15 = &UNK_142b19a30;
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
        if (uVar7 != 0x5c) goto code_r0x000140c0e2f4;
code_r0x000140c0e306:
        puVar15 = puVar13 + -0x142b19a2f;
        if (puVar15 != (undefined *)0x0) {
          if (puVar15 < (undefined *)0x24) {
            if ((char)puVar13[1] < -0x40) {
code_r0x000140c0e322:
              uStack_49 = 1;
              FUN_1428d9940(&UNK_142b19a30,0x24,puVar15,0x24,&UNK_142b199c8);
              goto code_r0x000140c0e4da;
            }
          }
          else if (puVar15 != (undefined *)0x24) goto code_r0x000140c0e322;
        }
        lVar8 = 0x24 - (longlong)puVar15;
        puVar15 = puVar13 + 1;
        goto code_r0x000140c0e35c;
      }
      puVar13 = puVar13 + -1;
      if (uVar7 == 0x5c) goto code_r0x000140c0e306;
code_r0x000140c0e2f4:
      if (uVar7 == 0x2f) goto code_r0x000140c0e306;
    } while (puVar13 != &UNK_142b19a30);
    lVar8 = 0x24;
code_r0x000140c0e35c:
    plVar9 = (longlong *)func_0x000140c18130();
    if (*plVar9 == 0) {
      uStack_49 = 1;
      plVar9 = (longlong *)FUN_1429090d0(plVar9,0);
      if (plVar9 == (longlong *)0x0) goto code_r0x000140c0e4b8;
    }
    else {
      if ((int)*plVar9 != 1) {
code_r0x000140c0e4b8:
        uStack_49 = 1;
        FUN_1428d80d0(&UNK_142d06498);
        goto code_r0x000140c0e4da;
      }
      plVar9 = plVar9 + 1;
    }
    if (*plVar9 != 0) {
      uStack_49 = 1;
      FUN_1428d9250(&UNK_142d06508);
      goto code_r0x000140c0e4da;
    }
    *plVar9 = -1;
    plStack_60 = plVar9;
    unaff_RBX = FUN_140c16d40(plVar9 + 1,puVar14,lVar11,puVar15,lVar8,1,0);
    *plStack_60 = *plStack_60 + 1;
    uStack_88 = 1;
    uStack_80 = unaff_RBX;
  }
  uStack_98 = *(undefined8 *)(puStack_68 + 0x10);
  uStack_d8 = *puStack_68;
  uStack_d4 = puStack_68[1];
  uStack_d0 = puStack_68[2];
  uStack_cc = puStack_68[3];
  uStack_c8 = puStack_68[4];
  uStack_c4 = puStack_68[5];
  uStack_c0 = puStack_68[6];
  uStack_bc = puStack_68[7];
  uStack_b8 = puStack_68[8];
  uStack_b4 = puStack_68[9];
  uStack_b0 = puStack_68[10];
  uStack_ac = puStack_68[0xb];
  uStack_a8 = puStack_68[0xc];
  uStack_a4 = puStack_68[0xd];
  uStack_a0 = puStack_68[0xe];
  uStack_9c = puStack_68[0xf];
  FUN_140c14680(param_1,param_2,&uStack_d8);
  if ((char)plStack_58 == '\0') {
    return param_1;
  }
  plVar9 = (longlong *)func_0x000140c18130();
  if (*plVar9 == 0) {
    uStack_49 = 0;
    plVar9 = (longlong *)FUN_1429090d0(plVar9,0);
    if (plVar9 == (longlong *)0x0) goto code_r0x000140c0e4ca;
  }
  else {
    if ((int)*plVar9 != 1) {
code_r0x000140c0e4ca:
      uStack_49 = 0;
      FUN_1428d80d0(&UNK_142d06498);
      goto code_r0x000140c0e4da;
    }
    plVar9 = plVar9 + 1;
  }
  if (*plVar9 == 0) {
    *plVar9 = -1;
    plStack_58 = plVar9;
    FUN_140c170f0(plVar9 + 1,unaff_RBX);
    *plStack_58 = *plStack_58 + 1;
    return param_1;
  }
  uStack_49 = 0;
  FUN_1428d9250(&UNK_142d06508);
code_r0x000140c0e4da:
                    /* WARNING: Does not return */
  pcVar5 = (code *)invalidInstructionException();
  (*pcVar5)();
}

