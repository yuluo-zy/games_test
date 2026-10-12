
undefined8 FUN_1400dd6f0(longlong param_1,longlong param_2,ulonglong param_3)

{
  int *piVar1;
  char *pcVar2;
  int iVar3;
  longlong lVar4;
  uint uVar5;
  uint uVar6;
  longlong lVar7;
  undefined8 uVar8;
  ushort uVar9;
  ulonglong uVar10;
  uint uVar11;
  ulonglong uVar12;
  longlong lVar13;
  bool bVar14;
  byte bVar15;
  char cVar16;
  byte bVar17;
  char cVar19;
  char cVar20;
  char cVar21;
  undefined1 auVar18 [16];
  undefined1 auVar22 [16];
  undefined1 auVar23 [16];
  undefined1 auVar24 [16];
  undefined1 auVar25 [16];
  
  bVar17 = (byte)(param_3 >> 0x38);
  if (*(longlong *)(param_1 + 0x50) != 0) {
    lVar4 = *(longlong *)(param_1 + 0x38);
    bVar15 = bVar17 >> 1;
    auVar18 = ZEXT216(CONCAT11(bVar15,bVar15));
    auVar18 = pshuflw(auVar18,auVar18,0);
    lVar13 = 0;
    uVar12 = param_3;
    while( true ) {
      uVar12 = *(ulonglong *)(param_1 + 0x40) & uVar12;
      pcVar2 = (char *)(lVar4 + uVar12);
      cVar16 = auVar18[0];
      auVar24[0] = -(*pcVar2 == cVar16);
      cVar19 = auVar18[1];
      auVar24[1] = -(pcVar2[1] == cVar19);
      cVar20 = auVar18[2];
      auVar24[2] = -(pcVar2[2] == cVar20);
      cVar21 = auVar18[3];
      auVar24[3] = -(pcVar2[3] == cVar21);
      auVar24[4] = -(pcVar2[4] == cVar16);
      auVar24[5] = -(pcVar2[5] == cVar19);
      auVar24[6] = -(pcVar2[6] == cVar20);
      auVar24[7] = -(pcVar2[7] == cVar21);
      auVar24[8] = -(pcVar2[8] == cVar16);
      auVar24[9] = -(pcVar2[9] == cVar19);
      auVar24[10] = -(pcVar2[10] == cVar20);
      auVar24[0xb] = -(pcVar2[0xb] == cVar21);
      auVar24[0xc] = -(pcVar2[0xc] == cVar16);
      auVar24[0xd] = -(pcVar2[0xd] == cVar19);
      auVar24[0xe] = -(pcVar2[0xe] == cVar20);
      auVar24[0xf] = -(pcVar2[0xf] == cVar21);
      uVar9 = (ushort)(SUB161(auVar24 >> 7,0) & 1) | (ushort)(SUB161(auVar24 >> 0xf,0) & 1) << 1 |
              (ushort)(SUB161(auVar24 >> 0x17,0) & 1) << 2 |
              (ushort)(SUB161(auVar24 >> 0x1f,0) & 1) << 3 |
              (ushort)(SUB161(auVar24 >> 0x27,0) & 1) << 4 |
              (ushort)(SUB161(auVar24 >> 0x2f,0) & 1) << 5 |
              (ushort)(SUB161(auVar24 >> 0x37,0) & 1) << 6 |
              (ushort)(SUB161(auVar24 >> 0x3f,0) & 1) << 7 |
              (ushort)(SUB161(auVar24 >> 0x47,0) & 1) << 8 |
              (ushort)(SUB161(auVar24 >> 0x4f,0) & 1) << 9 |
              (ushort)(SUB161(auVar24 >> 0x57,0) & 1) << 10 |
              (ushort)(SUB161(auVar24 >> 0x5f,0) & 1) << 0xb |
              (ushort)(SUB161(auVar24 >> 0x67,0) & 1) << 0xc |
              (ushort)(SUB161(auVar24 >> 0x6f,0) & 1) << 0xd |
              (ushort)(SUB161(auVar24 >> 0x77,0) & 1) << 0xe | (ushort)(auVar24[0xf] >> 7) << 0xf;
      uVar11 = (uint)uVar9;
      while (uVar9 != 0) {
        uVar5 = 0;
        for (uVar6 = uVar11; (uVar6 & 1) == 0; uVar6 = uVar6 >> 1 | 0x80000000) {
          uVar5 = uVar5 + 1;
        }
        uVar10 = uVar5 + uVar12 & *(ulonglong *)(param_1 + 0x40);
        if ((*(longlong *)(lVar4 + -0x18 + uVar10 * -0x18) == param_2) &&
           (*(ulonglong *)(lVar4 + -0x10 + uVar10 * -0x18) == param_3)) {
          return 1;
        }
        uVar9 = (ushort)(uVar11 - 1) & (ushort)uVar11;
        uVar11 = CONCAT22((short)(uVar11 - 1 >> 0x10),uVar9);
      }
      auVar22[0] = -(*pcVar2 == -1);
      auVar22[1] = -(pcVar2[1] == -1);
      auVar22[2] = -(pcVar2[2] == -1);
      auVar22[3] = -(pcVar2[3] == -1);
      auVar22[4] = -(pcVar2[4] == -1);
      auVar22[5] = -(pcVar2[5] == -1);
      auVar22[6] = -(pcVar2[6] == -1);
      auVar22[7] = -(pcVar2[7] == -1);
      auVar22[8] = -(pcVar2[8] == -1);
      auVar22[9] = -(pcVar2[9] == -1);
      auVar22[10] = -(pcVar2[10] == -1);
      auVar22[0xb] = -(pcVar2[0xb] == -1);
      auVar22[0xc] = -(pcVar2[0xc] == -1);
      auVar22[0xd] = -(pcVar2[0xd] == -1);
      auVar22[0xe] = -(pcVar2[0xe] == -1);
      auVar22[0xf] = -(pcVar2[0xf] == -1);
      if ((((((((((((((((SUB161(auVar22 >> 7,0) & 1) != 0 || (SUB161(auVar22 >> 0xf,0) & 1) != 0) ||
                      (SUB161(auVar22 >> 0x17,0) & 1) != 0) || (SUB161(auVar22 >> 0x1f,0) & 1) != 0)
                    || (SUB161(auVar22 >> 0x27,0) & 1) != 0) || (SUB161(auVar22 >> 0x2f,0) & 1) != 0
                   ) || (SUB161(auVar22 >> 0x37,0) & 1) != 0) ||
                 (SUB161(auVar22 >> 0x3f,0) & 1) != 0) || (SUB161(auVar22 >> 0x47,0) & 1) != 0) ||
               (SUB161(auVar22 >> 0x4f,0) & 1) != 0) || (SUB161(auVar22 >> 0x57,0) & 1) != 0) ||
             (SUB161(auVar22 >> 0x5f,0) & 1) != 0) || (SUB161(auVar22 >> 0x67,0) & 1) != 0) ||
           (SUB161(auVar22 >> 0x6f,0) & 1) != 0) || (SUB161(auVar22 >> 0x77,0) & 1) != 0) ||
          auVar22[0xf] < '\0') break;
      uVar12 = uVar12 + lVar13 + 0x10;
      lVar13 = lVar13 + 0x10;
    }
  }
  uVar11 = *(uint *)(param_1 + 0x58);
  if (uVar11 < 0x3ffffffe) {
    LOCK();
    bVar14 = uVar11 == *(uint *)(param_1 + 0x58);
    if (bVar14) {
      *(uint *)(param_1 + 0x58) = uVar11 + 1;
    }
    UNLOCK();
    if (bVar14) {
      lVar4 = *(longlong *)(param_1 + 0xb8);
      goto joined_r0x0001400dd8bf;
    }
  }
  FUN_1428d8a30(param_1 + 0x58);
  lVar4 = *(longlong *)(param_1 + 0xb8);
joined_r0x0001400dd8bf:
  if (lVar4 == 0) {
    uVar8 = 0;
  }
  else {
    lVar4 = *(longlong *)(param_1 + 0xa0);
    bVar17 = bVar17 >> 1;
    auVar18 = ZEXT216(CONCAT11(bVar17,bVar17));
    auVar18 = pshuflw(auVar18,auVar18,0);
    uVar8 = 0;
    lVar13 = 0;
    uVar12 = param_3;
    while( true ) {
      uVar12 = *(ulonglong *)(param_1 + 0xa8) & uVar12;
      pcVar2 = (char *)(lVar4 + uVar12);
      cVar16 = auVar18[0];
      auVar25[0] = -(*pcVar2 == cVar16);
      cVar19 = auVar18[1];
      auVar25[1] = -(pcVar2[1] == cVar19);
      cVar20 = auVar18[2];
      auVar25[2] = -(pcVar2[2] == cVar20);
      cVar21 = auVar18[3];
      auVar25[3] = -(pcVar2[3] == cVar21);
      auVar25[4] = -(pcVar2[4] == cVar16);
      auVar25[5] = -(pcVar2[5] == cVar19);
      auVar25[6] = -(pcVar2[6] == cVar20);
      auVar25[7] = -(pcVar2[7] == cVar21);
      auVar25[8] = -(pcVar2[8] == cVar16);
      auVar25[9] = -(pcVar2[9] == cVar19);
      auVar25[10] = -(pcVar2[10] == cVar20);
      auVar25[0xb] = -(pcVar2[0xb] == cVar21);
      auVar25[0xc] = -(pcVar2[0xc] == cVar16);
      auVar25[0xd] = -(pcVar2[0xd] == cVar19);
      auVar25[0xe] = -(pcVar2[0xe] == cVar20);
      auVar25[0xf] = -(pcVar2[0xf] == cVar21);
      uVar9 = (ushort)(SUB161(auVar25 >> 7,0) & 1) | (ushort)(SUB161(auVar25 >> 0xf,0) & 1) << 1 |
              (ushort)(SUB161(auVar25 >> 0x17,0) & 1) << 2 |
              (ushort)(SUB161(auVar25 >> 0x1f,0) & 1) << 3 |
              (ushort)(SUB161(auVar25 >> 0x27,0) & 1) << 4 |
              (ushort)(SUB161(auVar25 >> 0x2f,0) & 1) << 5 |
              (ushort)(SUB161(auVar25 >> 0x37,0) & 1) << 6 |
              (ushort)(SUB161(auVar25 >> 0x3f,0) & 1) << 7 |
              (ushort)(SUB161(auVar25 >> 0x47,0) & 1) << 8 |
              (ushort)(SUB161(auVar25 >> 0x4f,0) & 1) << 9 |
              (ushort)(SUB161(auVar25 >> 0x57,0) & 1) << 10 |
              (ushort)(SUB161(auVar25 >> 0x5f,0) & 1) << 0xb |
              (ushort)(SUB161(auVar25 >> 0x67,0) & 1) << 0xc |
              (ushort)(SUB161(auVar25 >> 0x6f,0) & 1) << 0xd |
              (ushort)(SUB161(auVar25 >> 0x77,0) & 1) << 0xe | (ushort)(auVar25[0xf] >> 7) << 0xf;
      uVar11 = (uint)uVar9;
      while (uVar9 != 0) {
        uVar5 = 0;
        for (uVar6 = uVar11; (uVar6 & 1) == 0; uVar6 = uVar6 >> 1 | 0x80000000) {
          uVar5 = uVar5 + 1;
        }
        lVar7 = (uVar5 + uVar12 & *(ulonglong *)(param_1 + 0xa8)) * -0x88;
        if ((*(longlong *)(lVar4 + -0x88 + lVar7) == param_2) &&
           (*(ulonglong *)(lVar4 + -0x80 + lVar7) == param_3)) {
          uVar8 = 1;
          goto code_r0x0001400dd8c7;
        }
        uVar9 = (ushort)(uVar11 - 1) & (ushort)uVar11;
        uVar11 = CONCAT22((short)(uVar11 - 1 >> 0x10),uVar9);
      }
      auVar23[0] = -(*pcVar2 == -1);
      auVar23[1] = -(pcVar2[1] == -1);
      auVar23[2] = -(pcVar2[2] == -1);
      auVar23[3] = -(pcVar2[3] == -1);
      auVar23[4] = -(pcVar2[4] == -1);
      auVar23[5] = -(pcVar2[5] == -1);
      auVar23[6] = -(pcVar2[6] == -1);
      auVar23[7] = -(pcVar2[7] == -1);
      auVar23[8] = -(pcVar2[8] == -1);
      auVar23[9] = -(pcVar2[9] == -1);
      auVar23[10] = -(pcVar2[10] == -1);
      auVar23[0xb] = -(pcVar2[0xb] == -1);
      auVar23[0xc] = -(pcVar2[0xc] == -1);
      auVar23[0xd] = -(pcVar2[0xd] == -1);
      auVar23[0xe] = -(pcVar2[0xe] == -1);
      auVar23[0xf] = -(pcVar2[0xf] == -1);
      if ((((((((((((((((SUB161(auVar23 >> 7,0) & 1) != 0 || (SUB161(auVar23 >> 0xf,0) & 1) != 0) ||
                      (SUB161(auVar23 >> 0x17,0) & 1) != 0) || (SUB161(auVar23 >> 0x1f,0) & 1) != 0)
                    || (SUB161(auVar23 >> 0x27,0) & 1) != 0) || (SUB161(auVar23 >> 0x2f,0) & 1) != 0
                   ) || (SUB161(auVar23 >> 0x37,0) & 1) != 0) ||
                 (SUB161(auVar23 >> 0x3f,0) & 1) != 0) || (SUB161(auVar23 >> 0x47,0) & 1) != 0) ||
               (SUB161(auVar23 >> 0x4f,0) & 1) != 0) || (SUB161(auVar23 >> 0x57,0) & 1) != 0) ||
             (SUB161(auVar23 >> 0x5f,0) & 1) != 0) || (SUB161(auVar23 >> 0x67,0) & 1) != 0) ||
           (SUB161(auVar23 >> 0x6f,0) & 1) != 0) || (SUB161(auVar23 >> 0x77,0) & 1) != 0) ||
          auVar23[0xf] < '\0') break;
      uVar12 = uVar12 + lVar13 + 0x10;
      lVar13 = lVar13 + 0x10;
    }
  }
code_r0x0001400dd8c7:
  LOCK();
  piVar1 = (int *)(param_1 + 0x58);
  iVar3 = *piVar1;
  *piVar1 = *piVar1 + -1;
  UNLOCK();
  uVar11 = iVar3 - 1;
  if (SBORROW4(0,uVar11 & 0xbfffffff)) {
    FUN_1428d8d40(param_1 + 0x58,uVar11,uVar11,-(uVar11 & 0xbfffffff));
  }
  return uVar8;
}

