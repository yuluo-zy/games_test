
longlong FUN_140be7320(longlong param_1,undefined8 *param_2)

{
  longlong lVar1;
  undefined8 *puVar2;
  undefined1 auVar3 [16];
  code *pcVar4;
  uint uVar5;
  uint uVar6;
  undefined8 uVar7;
  undefined8 uVar8;
  undefined8 uVar9;
  ulonglong uVar10;
  longlong lVar11;
  ushort uVar12;
  ulonglong uVar13;
  longlong lVar14;
  uint uVar15;
  byte bVar16;
  char cVar17;
  char cVar19;
  char cVar20;
  char cVar21;
  undefined1 auVar18 [16];
  undefined1 auVar22 [16];
  undefined1 auVar23 [16];
  undefined8 uStack_d8;
  undefined8 uStack_d0;
  longlong lStack_c0;
  longlong lStack_b8;
  longlong lStack_b0;
  undefined8 uStack_a8;
  undefined8 *puStack_a0;
  undefined8 uStack_98;
  undefined8 uStack_90;
  undefined8 uStack_88;
  longlong lStack_78;
  longlong lStack_70;
  longlong lStack_68;
  longlong lStack_60;
  undefined8 *puStack_58;
  undefined1 uStack_49;
  undefined8 uStack_48;
  
  uStack_48 = 0xfffffffffffffffe;
  puStack_58 = param_2;
  if (*(longlong *)(param_1 + 0x48) != 0) {
    uStack_49 = 1;
    uVar10 = FUN_1418bce70(param_1 + 0x50,param_2 + 3);
    lVar11 = *(longlong *)(param_1 + 0x30);
    bVar16 = (byte)(uVar10 >> 0x39);
    auVar18 = ZEXT216(CONCAT11(bVar16,bVar16));
    auVar18 = pshuflw(auVar18,auVar18,0);
    lVar14 = 0;
    while( true ) {
      uVar10 = *(ulonglong *)(param_1 + 0x38) & uVar10;
      auVar3 = *(undefined1 (*) [16])(lVar11 + uVar10);
      cVar17 = auVar18[0];
      auVar23[0] = -(auVar3[0] == cVar17);
      cVar19 = auVar18[1];
      auVar23[1] = -(auVar3[1] == cVar19);
      cVar20 = auVar18[2];
      auVar23[2] = -(auVar3[2] == cVar20);
      cVar21 = auVar18[3];
      auVar23[3] = -(auVar3[3] == cVar21);
      auVar23[4] = -(auVar3[4] == cVar17);
      auVar23[5] = -(auVar3[5] == cVar19);
      auVar23[6] = -(auVar3[6] == cVar20);
      auVar23[7] = -(auVar3[7] == cVar21);
      auVar23[8] = -(auVar3[8] == cVar17);
      auVar23[9] = -(auVar3[9] == cVar19);
      auVar23[10] = -(auVar3[10] == cVar20);
      auVar23[0xb] = -(auVar3[0xb] == cVar21);
      auVar23[0xc] = -(auVar3[0xc] == cVar17);
      auVar23[0xd] = -(auVar3[0xd] == cVar19);
      auVar23[0xe] = -(auVar3[0xe] == cVar20);
      auVar23[0xf] = -(auVar3[0xf] == cVar21);
      uVar12 = (ushort)(SUB161(auVar23 >> 7,0) & 1) | (ushort)(SUB161(auVar23 >> 0xf,0) & 1) << 1 |
               (ushort)(SUB161(auVar23 >> 0x17,0) & 1) << 2 |
               (ushort)(SUB161(auVar23 >> 0x1f,0) & 1) << 3 |
               (ushort)(SUB161(auVar23 >> 0x27,0) & 1) << 4 |
               (ushort)(SUB161(auVar23 >> 0x2f,0) & 1) << 5 |
               (ushort)(SUB161(auVar23 >> 0x37,0) & 1) << 6 |
               (ushort)(SUB161(auVar23 >> 0x3f,0) & 1) << 7 |
               (ushort)(SUB161(auVar23 >> 0x47,0) & 1) << 8 |
               (ushort)(SUB161(auVar23 >> 0x4f,0) & 1) << 9 |
               (ushort)(SUB161(auVar23 >> 0x57,0) & 1) << 10 |
               (ushort)(SUB161(auVar23 >> 0x5f,0) & 1) << 0xb |
               (ushort)(SUB161(auVar23 >> 0x67,0) & 1) << 0xc |
               (ushort)(SUB161(auVar23 >> 0x6f,0) & 1) << 0xd |
               (ushort)(SUB161(auVar23 >> 0x77,0) & 1) << 0xe | (ushort)(auVar23[0xf] >> 7) << 0xf;
      uVar15 = (uint)uVar12;
      while (uVar12 != 0) {
        uVar5 = 0;
        for (uVar6 = uVar15; (uVar6 & 1) == 0; uVar6 = uVar6 >> 1 | 0x80000000) {
          uVar5 = uVar5 + 1;
        }
        uVar13 = uVar5 + uVar10 & *(ulonglong *)(param_1 + 0x38);
        if ((((puStack_58[3] == *(longlong *)(lVar11 + -0x28 + uVar13 * -0x28)) &&
             (lVar1 = lVar11 + -0x28 + uVar13 * -0x28,
             *(int *)(puStack_58 + 4) == *(int *)(lVar1 + 8))) &&
            (*(char *)((longlong)puStack_58 + 0x26) == *(char *)(lVar1 + 0xe))) &&
           (*(short *)((longlong)puStack_58 + 0x24) == *(short *)(lVar1 + 0xc))) {
          uStack_88 = puStack_58[4];
          uStack_a8 = *puStack_58;
          puStack_a0 = (undefined8 *)puStack_58[1];
          uStack_98 = puStack_58[2];
          uStack_90 = puStack_58[3];
          lVar14 = *(longlong *)(lVar11 + -8 + uVar13 * -0x28);
          if (lVar14 == *(longlong *)(lVar11 + -0x18 + uVar13 * -0x28)) {
            FUN_140562ab0(lVar11 + uVar13 * -0x28 + -0x18,&UNK_142b13a68);
          }
          lVar11 = lVar11 + uVar13 * -0x28;
          lVar1 = *(longlong *)(lVar11 + -0x10);
          *(undefined8 *)(lVar1 + 0x20 + lVar14 * 0x28) = uStack_88;
          puVar2 = (undefined8 *)(lVar1 + 0x10 + lVar14 * 0x28);
          *puVar2 = uStack_98;
          puVar2[1] = uStack_90;
          puVar2 = (undefined8 *)(lVar1 + lVar14 * 0x28);
          *puVar2 = uStack_a8;
          puVar2[1] = puStack_a0;
          *(longlong *)(lVar11 + -8) = lVar14 + 1;
          return lVar11;
        }
        uVar12 = (ushort)(uVar15 - 1) & (ushort)uVar15;
        uVar15 = CONCAT22((short)(uVar15 - 1 >> 0x10),uVar12);
      }
      auVar22[0] = -(auVar3[0] == -1);
      auVar22[1] = -(auVar3[1] == -1);
      auVar22[2] = -(auVar3[2] == -1);
      auVar22[3] = -(auVar3[3] == -1);
      auVar22[4] = -(auVar3[4] == -1);
      auVar22[5] = -(auVar3[5] == -1);
      auVar22[6] = -(auVar3[6] == -1);
      auVar22[7] = -(auVar3[7] == -1);
      auVar22[8] = -(auVar3[8] == -1);
      auVar22[9] = -(auVar3[9] == -1);
      auVar22[10] = -(auVar3[10] == -1);
      auVar22[0xb] = -(auVar3[0xb] == -1);
      auVar22[0xc] = -(auVar3[0xc] == -1);
      auVar22[0xd] = -(auVar3[0xd] == -1);
      auVar22[0xe] = -(auVar3[0xe] == -1);
      auVar22[0xf] = -(auVar3[0xf] == -1);
      if ((((((((((((((((SUB161(auVar22 >> 7,0) & 1) != 0 || (SUB161(auVar22 >> 0xf,0) & 1) != 0) ||
                      (SUB161(auVar22 >> 0x17,0) & 1) != 0) || (SUB161(auVar22 >> 0x1f,0) & 1) != 0)
                    || (SUB161(auVar22 >> 0x27,0) & 1) != 0) || (SUB161(auVar22 >> 0x2f,0) & 1) != 0
                   ) || (SUB161(auVar22 >> 0x37,0) & 1) != 0) ||
                 (SUB161(auVar22 >> 0x3f,0) & 1) != 0) || (SUB161(auVar22 >> 0x47,0) & 1) != 0) ||
               (SUB161(auVar22 >> 0x4f,0) & 1) != 0) || (SUB161(auVar22 >> 0x57,0) & 1) != 0) ||
             (SUB161(auVar22 >> 0x5f,0) & 1) != 0) || (SUB161(auVar22 >> 0x67,0) & 1) != 0) ||
           (SUB161(auVar22 >> 0x6f,0) & 1) != 0) || (SUB161(auVar22 >> 0x77,0) & 1) != 0) ||
          auVar22[0xf] < '\0') break;
      uVar10 = uVar10 + lVar14 + 0x10;
      lVar14 = lVar14 + 0x10;
    }
  }
  uStack_d8 = param_2[3];
  uStack_d0 = param_2[4];
  puStack_a0 = (undefined8 *)func_0x000140613c10(0x28,8);
  if (puStack_a0 != (undefined8 *)0x0) {
    puStack_a0[4] = puStack_58[4];
    uVar7 = *puStack_58;
    uVar8 = puStack_58[1];
    uVar9 = puStack_58[3];
    puStack_a0[2] = puStack_58[2];
    puStack_a0[3] = uVar9;
    *puStack_a0 = uVar7;
    puStack_a0[1] = uVar8;
    uStack_a8 = 1;
    uStack_98 = 1;
    uStack_49 = 0;
    FUN_1418ad9c0(&lStack_c0,param_1 + 0x30,&uStack_d8,&uStack_a8);
    lVar11 = -lStack_c0;
    if (!SBORROW8(0,lStack_c0)) {
      lStack_68 = lStack_c0;
      lStack_78 = lStack_b0;
      lStack_70 = 0;
      lStack_60 = lStack_b8;
      lVar11 = lStack_b8;
      while (lStack_78 != lStack_70) {
        lStack_70 = lStack_70 + 1;
        FUN_140be24b0(lVar11);
        lVar11 = lVar11 + 0x28;
      }
      lVar11 = 0;
      if (lStack_68 != 0) {
        lVar11 = func_0x000140613c20(lStack_60,lStack_68 * 0x28,8);
      }
    }
    return lVar11;
  }
  uStack_49 = 1;
  FUN_1428d9000(8,0x28);
                    /* WARNING: Does not return */
  pcVar4 = (code *)invalidInstructionException();
  (*pcVar4)();
}

