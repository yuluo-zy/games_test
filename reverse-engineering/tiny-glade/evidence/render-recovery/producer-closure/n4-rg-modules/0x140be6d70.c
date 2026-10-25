
longlong FUN_140be6d70(longlong *param_1,undefined8 *param_2)

{
  char *pcVar1;
  int *piVar2;
  undefined8 *puVar3;
  char cVar4;
  longlong lVar5;
  code *pcVar6;
  uint uVar7;
  uint uVar8;
  undefined8 uVar9;
  undefined8 uVar10;
  undefined8 uVar11;
  undefined8 uVar12;
  undefined8 uVar13;
  ulonglong uVar14;
  longlong lVar15;
  longlong lVar16;
  uint uVar17;
  ushort uVar18;
  longlong lVar19;
  longlong lVar20;
  byte bVar21;
  char cVar22;
  char cVar24;
  char cVar25;
  char cVar26;
  undefined1 auVar23 [16];
  undefined1 auVar27 [16];
  undefined1 auVar28 [16];
  undefined8 uStack_e8;
  undefined8 *puStack_e0;
  undefined8 uStack_d8;
  longlong lStack_d0;
  longlong lStack_c8;
  longlong lStack_c0;
  undefined8 uStack_b8;
  undefined8 uStack_b0;
  undefined8 uStack_a8;
  undefined8 uStack_a0;
  undefined8 uStack_98;
  undefined8 uStack_90;
  undefined8 uStack_88;
  longlong lStack_80;
  ulonglong uStack_78;
  undefined8 *puStack_70;
  longlong lStack_68;
  longlong lStack_60;
  longlong lStack_58;
  char cStack_4b;
  char cStack_4a;
  undefined1 uStack_49;
  undefined8 uStack_48;
  
  uStack_48 = 0xfffffffffffffffe;
  puStack_70 = param_2;
  if (param_1[3] != 0) {
    uStack_49 = 1;
    uVar14 = FUN_1418bcc30(param_1 + 4,param_2 + 3);
    lVar15 = *param_1;
    bVar21 = (byte)(uVar14 >> 0x39);
    auVar23 = ZEXT216(CONCAT11(bVar21,bVar21));
    auVar23 = pshuflw(auVar23,auVar23,0);
    cVar4 = *(char *)((longlong)puStack_70 + 0x36);
    lStack_68 = *(longlong *)((longlong)puStack_70 + 0x24);
    lStack_60 = CONCAT71(lStack_60._1_7_,*(char *)((longlong)puStack_70 + 0x34));
    lStack_58 = CONCAT44(lStack_58._4_4_,*(int *)(puStack_70 + 6));
    cStack_4b = *(char *)((longlong)puStack_70 + 0x35);
    lVar19 = 0;
    uStack_78 = (ulonglong)*(uint *)((longlong)puStack_70 + 0x2c);
    while( true ) {
      uVar14 = param_1[1] & uVar14;
      pcVar1 = (char *)(lVar15 + uVar14);
      cVar22 = auVar23[0];
      auVar28[0] = -(*pcVar1 == cVar22);
      cVar24 = auVar23[1];
      auVar28[1] = -(pcVar1[1] == cVar24);
      cVar25 = auVar23[2];
      auVar28[2] = -(pcVar1[2] == cVar25);
      cVar26 = auVar23[3];
      auVar28[3] = -(pcVar1[3] == cVar26);
      auVar28[4] = -(pcVar1[4] == cVar22);
      auVar28[5] = -(pcVar1[5] == cVar24);
      auVar28[6] = -(pcVar1[6] == cVar25);
      auVar28[7] = -(pcVar1[7] == cVar26);
      auVar28[8] = -(pcVar1[8] == cVar22);
      auVar28[9] = -(pcVar1[9] == cVar24);
      auVar28[10] = -(pcVar1[10] == cVar25);
      auVar28[0xb] = -(pcVar1[0xb] == cVar26);
      auVar28[0xc] = -(pcVar1[0xc] == cVar22);
      auVar28[0xd] = -(pcVar1[0xd] == cVar24);
      auVar28[0xe] = -(pcVar1[0xe] == cVar25);
      auVar28[0xf] = -(pcVar1[0xf] == cVar26);
      uVar18 = (ushort)(SUB161(auVar28 >> 7,0) & 1) | (ushort)(SUB161(auVar28 >> 0xf,0) & 1) << 1 |
               (ushort)(SUB161(auVar28 >> 0x17,0) & 1) << 2 |
               (ushort)(SUB161(auVar28 >> 0x1f,0) & 1) << 3 |
               (ushort)(SUB161(auVar28 >> 0x27,0) & 1) << 4 |
               (ushort)(SUB161(auVar28 >> 0x2f,0) & 1) << 5 |
               (ushort)(SUB161(auVar28 >> 0x37,0) & 1) << 6 |
               (ushort)(SUB161(auVar28 >> 0x3f,0) & 1) << 7 |
               (ushort)(SUB161(auVar28 >> 0x47,0) & 1) << 8 |
               (ushort)(SUB161(auVar28 >> 0x4f,0) & 1) << 9 |
               (ushort)(SUB161(auVar28 >> 0x57,0) & 1) << 10 |
               (ushort)(SUB161(auVar28 >> 0x5f,0) & 1) << 0xb |
               (ushort)(SUB161(auVar28 >> 0x67,0) & 1) << 0xc |
               (ushort)(SUB161(auVar28 >> 0x6f,0) & 1) << 0xd |
               (ushort)(SUB161(auVar28 >> 0x77,0) & 1) << 0xe | (ushort)(auVar28[0xf] >> 7) << 0xf;
      uVar17 = (uint)uVar18;
      while (uVar18 != 0) {
        uVar7 = 0;
        for (uVar8 = uVar17; (uVar8 & 1) == 0; uVar8 = uVar8 >> 1 | 0x80000000) {
          uVar7 = uVar7 + 1;
        }
        lVar20 = (uVar7 + uVar14 & param_1[1]) * -0x38;
        lStack_80 = lVar19;
        cStack_4a = cVar4;
        if (((((cVar4 == *(char *)(lVar15 + -0x1a + lVar20)) &&
              (piVar2 = (int *)(lVar15 + -0x38 + lVar20), *(int *)(puStack_70 + 3) == *piVar2)) &&
             (*(int *)((longlong)puStack_70 + 0x1c) == piVar2[1])) &&
            ((*(int *)(puStack_70 + 4) == piVar2[2] &&
             (*(longlong *)(piVar2 + 3) == lStack_68 && (uint)piVar2[5] == uStack_78)))) &&
           ((*(char *)((longlong)puStack_70 + 0x34) == (char)piVar2[7] &&
            ((*(int *)(puStack_70 + 6) == piVar2[6] &&
             (cStack_4b == *(char *)((longlong)piVar2 + 0x1d))))))) {
          uStack_88 = puStack_70[6];
          uStack_b8 = *puStack_70;
          uStack_b0 = puStack_70[1];
          uStack_a8 = puStack_70[2];
          uStack_a0 = puStack_70[3];
          uStack_98 = puStack_70[4];
          uStack_90 = puStack_70[5];
          lVar19 = *(longlong *)(lVar15 + -8 + lVar20);
          if (lVar19 == *(longlong *)(lVar15 + -0x18 + lVar20)) {
            FUN_1406319d0(lVar15 + lVar20 + -0x18,&UNK_142b13a50);
          }
          lVar5 = *(longlong *)(lVar15 + lVar20 + -0x10);
          lVar16 = lVar19 * 0x38;
          *(undefined8 *)(lVar5 + 0x30 + lVar16) = uStack_88;
          puVar3 = (undefined8 *)(lVar5 + 0x20 + lVar16);
          *puVar3 = uStack_98;
          puVar3[1] = uStack_90;
          puVar3 = (undefined8 *)(lVar5 + 0x10 + lVar16);
          *puVar3 = uStack_a8;
          puVar3[1] = uStack_a0;
          *(undefined8 *)(lVar5 + lVar16) = uStack_b8;
          ((undefined8 *)(lVar5 + lVar16))[1] = uStack_b0;
          *(longlong *)(lVar15 + lVar20 + -8) = lVar19 + 1;
          return lVar5;
        }
        uVar18 = (ushort)(uVar17 - 1) & (ushort)uVar17;
        uVar17 = CONCAT22((short)(uVar17 - 1 >> 0x10),uVar18);
      }
      auVar27[0] = -(*pcVar1 == -1);
      auVar27[1] = -(pcVar1[1] == -1);
      auVar27[2] = -(pcVar1[2] == -1);
      auVar27[3] = -(pcVar1[3] == -1);
      auVar27[4] = -(pcVar1[4] == -1);
      auVar27[5] = -(pcVar1[5] == -1);
      auVar27[6] = -(pcVar1[6] == -1);
      auVar27[7] = -(pcVar1[7] == -1);
      auVar27[8] = -(pcVar1[8] == -1);
      auVar27[9] = -(pcVar1[9] == -1);
      auVar27[10] = -(pcVar1[10] == -1);
      auVar27[0xb] = -(pcVar1[0xb] == -1);
      auVar27[0xc] = -(pcVar1[0xc] == -1);
      auVar27[0xd] = -(pcVar1[0xd] == -1);
      auVar27[0xe] = -(pcVar1[0xe] == -1);
      auVar27[0xf] = -(pcVar1[0xf] == -1);
      if ((((((((((((((((SUB161(auVar27 >> 7,0) & 1) != 0 || (SUB161(auVar27 >> 0xf,0) & 1) != 0) ||
                      (SUB161(auVar27 >> 0x17,0) & 1) != 0) || (SUB161(auVar27 >> 0x1f,0) & 1) != 0)
                    || (SUB161(auVar27 >> 0x27,0) & 1) != 0) || (SUB161(auVar27 >> 0x2f,0) & 1) != 0
                   ) || (SUB161(auVar27 >> 0x37,0) & 1) != 0) ||
                 (SUB161(auVar27 >> 0x3f,0) & 1) != 0) || (SUB161(auVar27 >> 0x47,0) & 1) != 0) ||
               (SUB161(auVar27 >> 0x4f,0) & 1) != 0) || (SUB161(auVar27 >> 0x57,0) & 1) != 0) ||
             (SUB161(auVar27 >> 0x5f,0) & 1) != 0) || (SUB161(auVar27 >> 0x67,0) & 1) != 0) ||
           (SUB161(auVar27 >> 0x6f,0) & 1) != 0) || (SUB161(auVar27 >> 0x77,0) & 1) != 0) ||
          auVar27[0xf] < '\0') break;
      uVar14 = uVar14 + lVar19 + 0x10;
      lVar19 = lVar19 + 0x10;
    }
  }
  uStack_b8 = param_2[3];
  uStack_b0 = param_2[4];
  uStack_a8 = param_2[5];
  uStack_a0 = param_2[6];
  puStack_e0 = (undefined8 *)func_0x000140613c10(0x38,8);
  if (puStack_e0 != (undefined8 *)0x0) {
    puStack_e0[6] = puStack_70[6];
    uVar9 = *puStack_70;
    uVar10 = puStack_70[1];
    uVar11 = puStack_70[2];
    uVar12 = puStack_70[3];
    uVar13 = puStack_70[5];
    puStack_e0[4] = puStack_70[4];
    puStack_e0[5] = uVar13;
    puStack_e0[2] = uVar11;
    puStack_e0[3] = uVar12;
    *puStack_e0 = uVar9;
    puStack_e0[1] = uVar10;
    uStack_e8 = 1;
    uStack_d8 = 1;
    uStack_49 = 0;
    FUN_1418ad6b0(&lStack_d0,param_1,&uStack_b8,&uStack_e8);
    lVar15 = -lStack_d0;
    if (!SBORROW8(0,lStack_d0)) {
      lStack_60 = lStack_d0;
      uStack_78 = lStack_c0;
      lStack_68 = 0;
      lStack_58 = lStack_c8;
      lVar15 = lStack_c8;
      while (uStack_78 != lStack_68) {
        lStack_68 = lStack_68 + 1;
        FUN_140be24b0(lVar15);
        lVar15 = lVar15 + 0x38;
      }
      lVar15 = 0;
      if (lStack_60 != 0) {
        lVar15 = func_0x000140613c20(lStack_58,lStack_60 * 0x38,8);
      }
    }
    return lVar15;
  }
  uStack_49 = 1;
  FUN_1428d9000(8,0x38);
                    /* WARNING: Does not return */
  pcVar6 = (code *)invalidInstructionException();
  (*pcVar6)();
}

