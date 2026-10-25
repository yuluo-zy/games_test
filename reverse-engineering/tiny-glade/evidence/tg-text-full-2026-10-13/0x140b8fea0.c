
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

undefined8 * FUN_140b8fea0(undefined8 *param_1,longlong param_2,longlong param_3)

{
  char *pcVar1;
  int *piVar2;
  undefined8 *puVar3;
  longlong lVar4;
  code *pcVar5;
  uint uVar6;
  undefined8 uVar7;
  ulonglong uVar8;
  longlong *plVar9;
  longlong lVar10;
  int iVar11;
  uint uVar12;
  uint uVar13;
  longlong *plVar14;
  ushort uVar15;
  ulonglong uVar16;
  longlong lVar17;
  undefined2 uVar18;
  int *piVar19;
  int *piVar20;
  undefined2 uVar21;
  byte bVar22;
  char cVar23;
  char cVar25;
  char cVar26;
  char cVar27;
  undefined1 auVar24 [16];
  undefined1 auVar28 [16];
  undefined1 auVar29 [16];
  longlong lStack_178;
  undefined8 uStack_170;
  undefined8 uStack_168;
  uint uStack_160;
  int iStack_15c;
  uint uStack_158;
  int iStack_154;
  undefined8 uStack_150;
  undefined8 uStack_148;
  undefined8 uStack_140;
  undefined8 uStack_138;
  undefined *puStack_130;
  undefined8 uStack_128;
  undefined8 uStack_120;
  longlong lStack_118;
  longlong lStack_110;
  undefined8 uStack_108;
  undefined *puStack_100;
  undefined8 uStack_f8;
  undefined4 uStack_f0;
  undefined4 uStack_ec;
  undefined *puStack_e8;
  undefined8 uStack_e0;
  undefined8 uStack_d8;
  undefined1 auStack_d0 [16];
  int iStack_b0;
  uint uStack_ac;
  undefined2 uStack_a8;
  undefined2 uStack_a4;
  uint uStack_98;
  int iStack_94;
  longlong *plStack_90;
  longlong lStack_88;
  longlong lStack_80;
  int iStack_78;
  uint uStack_74;
  undefined4 uStack_70;
  undefined8 uStack_6c;
  int iStack_64;
  undefined8 uStack_60;
  longlong lStack_58;
  undefined1 uStack_49;
  undefined8 uStack_48;
  
  uStack_48 = 0xfffffffffffffffe;
  plVar14 = (longlong *)(param_3 + 8);
  if (*(longlong *)(param_2 + 0x4b0) != 0) {
    plStack_90 = plVar14;
    uVar8 = FUN_141800460(param_2 + 0x4b8,plVar14);
    lVar4 = *(longlong *)(param_2 + 0x498);
    bVar22 = (byte)(uVar8 >> 0x39);
    auVar24 = ZEXT216(CONCAT11(bVar22,bVar22));
    auVar24 = pshuflw(auVar24,auVar24,0);
    lStack_88 = CONCAT44(lStack_88._4_4_,*(int *)(param_3 + 8));
    lStack_58 = CONCAT62(lStack_58._2_6_,*(short *)(param_3 + 0x18));
    lVar17 = 0;
    while( true ) {
      uVar8 = *(ulonglong *)(param_2 + 0x4a0) & uVar8;
      pcVar1 = (char *)(lVar4 + uVar8);
      cVar23 = auVar24[0];
      auVar29[0] = -(*pcVar1 == cVar23);
      cVar25 = auVar24[1];
      auVar29[1] = -(pcVar1[1] == cVar25);
      cVar26 = auVar24[2];
      auVar29[2] = -(pcVar1[2] == cVar26);
      cVar27 = auVar24[3];
      auVar29[3] = -(pcVar1[3] == cVar27);
      auVar29[4] = -(pcVar1[4] == cVar23);
      auVar29[5] = -(pcVar1[5] == cVar25);
      auVar29[6] = -(pcVar1[6] == cVar26);
      auVar29[7] = -(pcVar1[7] == cVar27);
      auVar29[8] = -(pcVar1[8] == cVar23);
      auVar29[9] = -(pcVar1[9] == cVar25);
      auVar29[10] = -(pcVar1[10] == cVar26);
      auVar29[0xb] = -(pcVar1[0xb] == cVar27);
      auVar29[0xc] = -(pcVar1[0xc] == cVar23);
      auVar29[0xd] = -(pcVar1[0xd] == cVar25);
      auVar29[0xe] = -(pcVar1[0xe] == cVar26);
      auVar29[0xf] = -(pcVar1[0xf] == cVar27);
      uVar15 = (ushort)(SUB161(auVar29 >> 7,0) & 1) | (ushort)(SUB161(auVar29 >> 0xf,0) & 1) << 1 |
               (ushort)(SUB161(auVar29 >> 0x17,0) & 1) << 2 |
               (ushort)(SUB161(auVar29 >> 0x1f,0) & 1) << 3 |
               (ushort)(SUB161(auVar29 >> 0x27,0) & 1) << 4 |
               (ushort)(SUB161(auVar29 >> 0x2f,0) & 1) << 5 |
               (ushort)(SUB161(auVar29 >> 0x37,0) & 1) << 6 |
               (ushort)(SUB161(auVar29 >> 0x3f,0) & 1) << 7 |
               (ushort)(SUB161(auVar29 >> 0x47,0) & 1) << 8 |
               (ushort)(SUB161(auVar29 >> 0x4f,0) & 1) << 9 |
               (ushort)(SUB161(auVar29 >> 0x57,0) & 1) << 10 |
               (ushort)(SUB161(auVar29 >> 0x5f,0) & 1) << 0xb |
               (ushort)(SUB161(auVar29 >> 0x67,0) & 1) << 0xc |
               (ushort)(SUB161(auVar29 >> 0x6f,0) & 1) << 0xd |
               (ushort)(SUB161(auVar29 >> 0x77,0) & 1) << 0xe | (ushort)(auVar29[0xf] >> 7) << 0xf;
      uVar12 = (uint)uVar15;
      if (uVar15 != 0) {
        uStack_60 = CONCAT44(uStack_60._4_4_,*(int *)(param_3 + 0xc));
        do {
          uVar13 = 0;
          for (uVar6 = uVar12; (uVar6 & 1) == 0; uVar6 = uVar6 >> 1 | 0x80000000) {
            uVar13 = uVar13 + 1;
          }
          uVar16 = uVar13 + uVar8 & *(ulonglong *)(param_2 + 0x4a0);
          if (((((*(int *)(param_3 + 0xc) == *(int *)(lVar4 + -0x20 + uVar16 * -0x24)) &&
                (piVar2 = (int *)(lVar4 + -0x24 + uVar16 * -0x24), *(int *)(param_3 + 8) == *piVar2)
                ) && (*(short *)(param_3 + 0x18) == (short)piVar2[4])) &&
              ((*(int *)(param_3 + 0x10) == piVar2[2] &&
               (*(char *)(param_3 + 0x1a) == *(char *)((longlong)piVar2 + 0x12))))) &&
             ((*(char *)(param_3 + 0x1b) == *(char *)((longlong)piVar2 + 0x13) &&
              (*(int *)(param_3 + 0x14) == piVar2[3])))) {
            puVar3 = (undefined8 *)(lVar4 + -0x10 + uVar16 * -0x24);
            uVar7 = puVar3[1];
            *(undefined8 *)((longlong)param_1 + 4) = *puVar3;
            *(undefined8 *)((longlong)param_1 + 0xc) = uVar7;
            *(undefined4 *)param_1 = 1;
            return param_1;
          }
          uVar15 = (ushort)(uVar12 - 1) & (ushort)uVar12;
          uVar12 = CONCAT22((short)(uVar12 - 1 >> 0x10),uVar15);
          lStack_80 = lVar17;
        } while (uVar15 != 0);
      }
      auVar28[0] = -(*pcVar1 == -1);
      auVar28[1] = -(pcVar1[1] == -1);
      auVar28[2] = -(pcVar1[2] == -1);
      auVar28[3] = -(pcVar1[3] == -1);
      auVar28[4] = -(pcVar1[4] == -1);
      auVar28[5] = -(pcVar1[5] == -1);
      auVar28[6] = -(pcVar1[6] == -1);
      auVar28[7] = -(pcVar1[7] == -1);
      auVar28[8] = -(pcVar1[8] == -1);
      auVar28[9] = -(pcVar1[9] == -1);
      auVar28[10] = -(pcVar1[10] == -1);
      auVar28[0xb] = -(pcVar1[0xb] == -1);
      auVar28[0xc] = -(pcVar1[0xc] == -1);
      auVar28[0xd] = -(pcVar1[0xd] == -1);
      auVar28[0xe] = -(pcVar1[0xe] == -1);
      auVar28[0xf] = -(pcVar1[0xf] == -1);
      plVar14 = plStack_90;
      if ((((((((((((((((SUB161(auVar28 >> 7,0) & 1) != 0 || (SUB161(auVar28 >> 0xf,0) & 1) != 0) ||
                      (SUB161(auVar28 >> 0x17,0) & 1) != 0) || (SUB161(auVar28 >> 0x1f,0) & 1) != 0)
                    || (SUB161(auVar28 >> 0x27,0) & 1) != 0) || (SUB161(auVar28 >> 0x2f,0) & 1) != 0
                   ) || (SUB161(auVar28 >> 0x37,0) & 1) != 0) ||
                 (SUB161(auVar28 >> 0x3f,0) & 1) != 0) || (SUB161(auVar28 >> 0x47,0) & 1) != 0) ||
               (SUB161(auVar28 >> 0x4f,0) & 1) != 0) || (SUB161(auVar28 >> 0x57,0) & 1) != 0) ||
             (SUB161(auVar28 >> 0x5f,0) & 1) != 0) || (SUB161(auVar28 >> 0x67,0) & 1) != 0) ||
           (SUB161(auVar28 >> 0x6f,0) & 1) != 0) || (SUB161(auVar28 >> 0x77,0) & 1) != 0) ||
          auVar28[0xf] < '\0') break;
      uVar8 = uVar8 + lVar17 + 0x10;
      lVar17 = lVar17 + 0x10;
    }
  }
  uStack_128 = CONCAT44(uStack_128._4_4_,(int)plVar14[2]);
  uStack_138 = *plVar14;
  puStack_130 = (undefined *)plVar14[1];
  FUN_140e12900(&lStack_178,param_2 + 0x110,param_2,&uStack_138);
  if (SBORROW8(0,lStack_178)) {
    *(undefined4 *)param_1 = 0;
    return param_1;
  }
  lVar4 = param_2 + 0x498;
  uStack_60 = uStack_170;
  lStack_58 = lStack_178;
  if (uStack_158 == 0) {
    uStack_128 = CONCAT44(uStack_128._4_4_,(int)plVar14[2]);
    uStack_138 = *plVar14;
    puStack_130 = (undefined *)plVar14[1];
    uStack_70 = 0;
    uStack_6c = 0xffffffff;
    iStack_64 = 0;
    uStack_49 = 0;
    FUN_141800670(&iStack_b0,lVar4,&uStack_138,&uStack_70);
  }
  else {
    if (iStack_154 != 0) {
      uStack_140 = uStack_168;
      iStack_94 = iStack_15c;
      uStack_98 = uStack_160;
      lStack_88 = param_2 + 0x3a8;
      uStack_49 = 0;
      uStack_74 = uStack_158;
      iStack_78 = iStack_154;
      lStack_80 = lVar4;
      FUN_1417f9c90(&uStack_138);
      if ((int)uStack_138 == 1) {
        uVar12 = uStack_138._4_4_;
        uVar21 = (short)puStack_130;
        uVar18 = (short)((ulonglong)puStack_130 >> 0x20);
LAB_140b903d5:
        uStack_49 = 0;
        FUN_141800040(param_2 + 0x3f8,uVar12 & 0xffff,*(undefined4 *)(param_2 + 0x4c8),
                      uVar12 >> 0x10);
        lVar4 = *(longlong *)(param_2 + 0x458);
        if (lVar4 == *(longlong *)(param_2 + 0x448)) {
          FUN_1405627b0(param_2 + 0x448,&UNK_142afc2c0);
        }
        lVar17 = *(longlong *)(param_2 + 0x450);
        lVar10 = lVar4 * 0x40;
        *(longlong *)(lVar17 + lVar10) = lStack_58;
        *(undefined8 *)(lVar17 + 8 + lVar10) = uStack_60;
        *(undefined8 *)(lVar17 + 0x10 + lVar10) = uStack_140;
        *(uint *)(lVar17 + 0x18 + lVar10) = uStack_98;
        *(int *)(lVar17 + 0x1c + lVar10) = iStack_94;
        *(uint *)(lVar17 + 0x20 + lVar10) = uStack_74;
        *(int *)(lVar17 + 0x24 + lVar10) = iStack_78;
        puVar3 = (undefined8 *)(lVar17 + 0x28 + lVar10);
        *puVar3 = uStack_150;
        puVar3[1] = uStack_148;
        *(undefined2 *)(lVar17 + 0x38 + lVar10) = uVar21;
        *(undefined2 *)(lVar17 + 0x3a + lVar10) = uVar18;
        *(short *)(lVar17 + 0x3c + lVar10) = (short)uStack_74;
        *(short *)(lVar17 + 0x3e + lVar10) = (short)iStack_78;
        *(longlong *)(param_2 + 0x458) = lVar4 + 1;
        uVar13 = iStack_78 << 0x10 | uStack_74 & 0xffff;
        iVar11 = (uStack_98 & 0xffff) + iStack_94 * -0x10000;
        uStack_128 = CONCAT44(uStack_128._4_4_,(int)plVar14[2]);
        uStack_138 = *plVar14;
        puStack_130 = (undefined *)plVar14[1];
        uStack_70 = CONCAT22(uVar18,uVar21);
        uStack_6c = CONCAT44(uVar13,uVar12);
        uStack_49 = 1;
        iStack_64 = iVar11;
        FUN_141800670(&iStack_b0,lStack_80,&uStack_138,&uStack_70);
        *(undefined2 *)((longlong)param_1 + 4) = uVar21;
        *(undefined2 *)((longlong)param_1 + 6) = uVar18;
        *(uint *)(param_1 + 1) = uVar12;
        *(uint *)((longlong)param_1 + 0xc) = uVar13;
        *(int *)(param_1 + 2) = iVar11;
        *(undefined4 *)param_1 = 1;
        return param_1;
      }
      if (3 < *_DAT_14362c3a8) {
        uStack_49 = 0;
        plVar9 = (longlong *)func_0x00014067dd80(&UNK_142afc310);
        lStack_118 = *plVar9;
        lStack_110 = plVar9[1];
        uStack_ec = (undefined4)plVar9[2];
        uStack_108 = 4;
        puStack_100 = &UNK_142afc2a0;
        uStack_f8 = 7;
        puStack_e8 = &UNK_142afc300;
        uStack_e0 = 1;
        uStack_d8 = 8;
        uStack_138 = 0;
        auStack_d0 = (undefined1  [16])0x0;
        puStack_130 = &UNK_142afc2a0;
        uStack_128 = 7;
        uStack_120 = 0;
        uStack_f0 = 1;
        uStack_49 = 0;
        func_0x000140d4a330(&iStack_b0,&uStack_138);
      }
      uStack_49 = 0;
      plVar9 = (longlong *)FUN_1400d9b20(0);
      plStack_90 = plVar14;
      if (plVar9 == (longlong *)0x0) {
        uStack_49 = 0;
        FUN_1428d80d0(&UNK_142cecb40);
                    /* WARNING: Does not return */
        pcVar5 = (code *)invalidInstructionException();
        (*pcVar5)();
      }
      lStack_118 = *plVar9;
      lStack_110 = plVar9[1];
      *plVar9 = *plVar9 + 1;
      uStack_128 = _DAT_142cecae0;
      uStack_120 = _UNK_142cecae8;
      uStack_138 = _DAT_142cecad0;
      puStack_130 = _UNK_142cecad8;
      piVar2 = *(int **)(param_2 + 0x400);
      lVar4 = *(longlong *)(param_2 + 0x408);
      piVar20 = piVar2;
      uVar12 = 0;
      while( true ) {
        uVar13 = uVar12;
        piVar19 = piVar20;
        if (piVar19 == piVar2 + lVar4 * 2) break;
        piVar20 = piVar19 + 2;
        uVar12 = uVar13 + 1;
        if ((*piVar19 != 0) && (*piVar19 + 1U < *(uint *)(param_2 + 0x4c8))) {
          uVar13 = piVar19[1] << 0x10 | uVar13 & 0xffff;
          FUN_1417fa4e0(lStack_88,uVar13);
          *piVar19 = 0;
          FUN_1402c9200(&uStack_138,uVar13);
        }
      }
      FUN_1418008d0(lStack_80,&uStack_138);
      FUN_1417f9c90(&iStack_b0,lStack_88,uStack_74,iStack_78);
      if (iStack_b0 == 1) {
        plVar14 = plStack_90;
        uVar12 = uStack_ac;
        uVar21 = uStack_a8;
        uVar18 = uStack_a4;
        if (puStack_130 != (undefined *)0x0) {
          uVar8 = (longlong)puStack_130 * 4 + 0x13U & 0xfffffffffffffff0;
          if (puStack_130 + uVar8 + 0x11 != (undefined *)0x0) {
            func_0x000140613c20(uStack_138 - uVar8,puStack_130 + uVar8 + 0x11,0x10);
            plVar14 = plStack_90;
          }
        }
        goto LAB_140b903d5;
      }
      *(undefined1 *)(param_2 + 0x4d4) = 1;
      if (puStack_130 != (undefined *)0x0) {
        uVar8 = (longlong)puStack_130 * 4 + 0x13U & 0xfffffffffffffff0;
        if (puStack_130 + uVar8 + 0x11 != (undefined *)0x0) {
          func_0x000140613c20(uStack_138 - uVar8,puStack_130 + uVar8 + 0x11,0x10);
        }
      }
      *(undefined4 *)param_1 = 0;
      goto joined_r0x000140b901d8;
    }
    uStack_128 = CONCAT44(uStack_128._4_4_,(int)plVar14[2]);
    uStack_138 = *plVar14;
    puStack_130 = (undefined *)plVar14[1];
    uStack_70 = 0;
    uStack_6c = 0xffffffff;
    iStack_64 = 0;
    uStack_49 = 0;
    FUN_141800670(&iStack_b0,lVar4,&uStack_138,&uStack_70);
  }
  param_1[1] = 0xffffffff;
  *(undefined4 *)(param_1 + 2) = 0;
  *param_1 = 1;
joined_r0x000140b901d8:
  if (lStack_58 != 0) {
    func_0x000140613c20(uStack_60,lStack_58,1);
  }
  return param_1;
}

