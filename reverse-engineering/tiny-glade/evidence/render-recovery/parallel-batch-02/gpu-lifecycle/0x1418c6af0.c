
void FUN_1418c6af0(ulonglong *param_1,longlong param_2,undefined8 param_3,byte *param_4,
                  longlong param_5,undefined8 param_6,undefined8 *param_7)

{
  byte *pbVar1;
  undefined1 auVar2 [16];
  code *pcVar3;
  uint uVar4;
  ulonglong *puVar5;
  ulonglong uVar6;
  ulonglong uVar7;
  uint uVar8;
  ulonglong uVar9;
  char *pcVar10;
  ushort uVar11;
  uint uVar12;
  longlong lVar13;
  byte *pbVar14;
  byte bVar15;
  char cVar16;
  undefined8 uVar17;
  char cVar19;
  char cVar20;
  char cVar21;
  undefined1 auVar18 [16];
  undefined1 auVar22 [16];
  undefined1 auVar23 [16];
  longlong lStack_90;
  undefined1 auStack_88 [16];
  undefined8 uStack_78;
  undefined8 uStack_70;
  longlong lStack_68;
  undefined1 uStack_60;
  ulonglong *puStack_58;
  char *pcStack_50;
  undefined8 uStack_48;
  
  uStack_48 = 0xfffffffffffffffe;
  uVar7 = *param_1;
  lStack_90 = param_2;
  pcStack_50 = (char *)param_3;
  if (uVar7 < 0xfffffffffffffff0 && (char)(((uint)uVar7 & 8) >> 3) == '\0') {
    LOCK();
    uVar6 = *param_1;
    if (uVar7 == uVar6) {
      *param_1 = uVar7 + 0x10;
    }
    UNLOCK();
    if (uVar7 == uVar6) goto code_r0x0001418c6b4e;
  }
  FUN_1428cc4b0(param_1,0);
code_r0x0001418c6b4e:
  FUN_140c81c70(param_1);
  FUN_140c81c70((ulonglong)param_1 | 1);
  if (param_1[4] != 0) {
    uVar6 = FUN_1405b0d70(param_1 + 5,&lStack_90);
    uVar7 = param_1[1];
    bVar15 = (byte)(uVar6 >> 0x39);
    auVar18 = ZEXT216(CONCAT11(bVar15,bVar15));
    auVar18 = pshuflw(auVar18,auVar18,0);
    lVar13 = 0;
    while( true ) {
      uVar6 = param_1[2] & uVar6;
      auVar2 = *(undefined1 (*) [16])(uVar7 + uVar6);
      cVar16 = auVar18[0];
      auVar23[0] = -(auVar2[0] == cVar16);
      cVar19 = auVar18[1];
      auVar23[1] = -(auVar2[1] == cVar19);
      cVar20 = auVar18[2];
      auVar23[2] = -(auVar2[2] == cVar20);
      cVar21 = auVar18[3];
      auVar23[3] = -(auVar2[3] == cVar21);
      auVar23[4] = -(auVar2[4] == cVar16);
      auVar23[5] = -(auVar2[5] == cVar19);
      auVar23[6] = -(auVar2[6] == cVar20);
      auVar23[7] = -(auVar2[7] == cVar21);
      auVar23[8] = -(auVar2[8] == cVar16);
      auVar23[9] = -(auVar2[9] == cVar19);
      auVar23[10] = -(auVar2[10] == cVar20);
      auVar23[0xb] = -(auVar2[0xb] == cVar21);
      auVar23[0xc] = -(auVar2[0xc] == cVar16);
      auVar23[0xd] = -(auVar2[0xd] == cVar19);
      auVar23[0xe] = -(auVar2[0xe] == cVar20);
      auVar23[0xf] = -(auVar2[0xf] == cVar21);
      uVar11 = (ushort)(SUB161(auVar23 >> 7,0) & 1) | (ushort)(SUB161(auVar23 >> 0xf,0) & 1) << 1 |
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
      uVar8 = (uint)uVar11;
      while (puStack_58 = param_1, uVar11 != 0) {
        uVar12 = 0;
        for (uVar4 = uVar8; (uVar4 & 1) == 0; uVar4 = uVar4 >> 1 | 0x80000000) {
          uVar12 = uVar12 + 1;
        }
        uVar9 = uVar12 + uVar6 & param_1[2];
        if (lStack_90 == *(longlong *)((uVar7 - 0x100) + uVar9 * -0x100)) {
          uVar17 = 0;
          auStack_88 = (undefined1  [16])0x0;
          if (param_5 == 0) goto code_r0x0001418c6e4c;
          pbVar14 = param_4 + param_5;
          uVar6 = 0;
          goto code_r0x0001418c6d89;
        }
        uVar11 = (ushort)(uVar8 - 1) & (ushort)uVar8;
        uVar8 = CONCAT22((short)(uVar8 - 1 >> 0x10),uVar11);
      }
      auVar22[0] = -(auVar2[0] == -1);
      auVar22[1] = -(auVar2[1] == -1);
      auVar22[2] = -(auVar2[2] == -1);
      auVar22[3] = -(auVar2[3] == -1);
      auVar22[4] = -(auVar2[4] == -1);
      auVar22[5] = -(auVar2[5] == -1);
      auVar22[6] = -(auVar2[6] == -1);
      auVar22[7] = -(auVar2[7] == -1);
      auVar22[8] = -(auVar2[8] == -1);
      auVar22[9] = -(auVar2[9] == -1);
      auVar22[10] = -(auVar2[10] == -1);
      auVar22[0xb] = -(auVar2[0xb] == -1);
      auVar22[0xc] = -(auVar2[0xc] == -1);
      auVar22[0xd] = -(auVar2[0xd] == -1);
      auVar22[0xe] = -(auVar2[0xe] == -1);
      auVar22[0xf] = -(auVar2[0xf] == -1);
      if ((((((((((((((((SUB161(auVar22 >> 7,0) & 1) != 0 || (SUB161(auVar22 >> 0xf,0) & 1) != 0) ||
                      (SUB161(auVar22 >> 0x17,0) & 1) != 0) || (SUB161(auVar22 >> 0x1f,0) & 1) != 0)
                    || (SUB161(auVar22 >> 0x27,0) & 1) != 0) || (SUB161(auVar22 >> 0x2f,0) & 1) != 0
                   ) || (SUB161(auVar22 >> 0x37,0) & 1) != 0) ||
                 (SUB161(auVar22 >> 0x3f,0) & 1) != 0) || (SUB161(auVar22 >> 0x47,0) & 1) != 0) ||
               (SUB161(auVar22 >> 0x4f,0) & 1) != 0) || (SUB161(auVar22 >> 0x57,0) & 1) != 0) ||
             (SUB161(auVar22 >> 0x5f,0) & 1) != 0) || (SUB161(auVar22 >> 0x67,0) & 1) != 0) ||
           (SUB161(auVar22 >> 0x6f,0) & 1) != 0) || (SUB161(auVar22 >> 0x77,0) & 1) != 0) ||
          auVar22[0xf] < '\0') break;
      uVar6 = uVar6 + lVar13 + 0x10;
      lVar13 = lVar13 + 0x10;
    }
  }
  FUN_140c81e10(param_1);
  FUN_140c81e10();
  LOCK();
  uVar7 = *param_1;
  *param_1 = *param_1 - 0x10;
  UNLOCK();
  if ((uVar7 & 0xfffffffffffffff2) == 0x12) {
    FUN_1428ccf50();
  }
  auStack_88 = (undefined1  [16])0x0;
  if (param_5 != 0) {
    auStack_88 = (undefined1  [16])0x0;
    pbVar14 = param_4 + param_5;
    uVar7 = 0;
    do {
      bVar15 = *param_4;
      if ((char)bVar15 < '\0') {
        uVar8 = bVar15 & 0x1f;
        pbVar1 = param_4 + 1;
        if (bVar15 < 0xe0) {
          param_4 = param_4 + 2;
          uVar12 = uVar8 << 6 | *pbVar1 & 0x3f;
        }
        else {
          uVar12 = param_4[2] & 0x3f | (*pbVar1 & 0x3f) << 6;
          if (bVar15 < 0xf0) {
            param_4 = param_4 + 3;
            uVar12 = uVar12 | uVar8 << 0xc;
          }
          else {
            uVar12 = param_4[3] & 0x3f | uVar12 << 6 | (bVar15 & 7) << 0x12;
            if (uVar12 == 0x110000) break;
            param_4 = param_4 + 4;
          }
        }
        bVar15 = (byte)uVar12;
        if (uVar12 < 0x80) goto code_r0x0001418c6d40;
      }
      else {
        param_4 = param_4 + 1;
code_r0x0001418c6d40:
        if (0xf < uVar7) {
          uVar17 = FUN_1428d9518(uVar7,0x10,&UNK_142b12f20);
code_r0x0001418c6fad:
          FUN_1428d9518(uVar17,0x10,&UNK_142b12f20);
                    /* WARNING: Does not return */
          pcVar3 = (code *)invalidInstructionException();
          (*pcVar3)();
        }
        auStack_88[uVar7] = bVar15;
        if (uVar7 == 0xf) break;
        uVar7 = uVar7 + 1;
      }
    } while (param_4 != pbVar14);
  }
  uStack_78 = auStack_88._0_8_;
  uStack_70 = auStack_88._8_8_;
  uVar7 = (ulonglong)lStack_68 >> 8;
  lStack_68 = CONCAT71((int7)uVar7,4);
  FUN_1418c1380(0x143634e40,&uStack_78,0x11);
  uStack_78 = *param_7;
  uStack_70 = param_7[1];
  FUN_1418af800(param_6,&uStack_78,pcStack_50);
  return;
code_r0x0001418c6d89:
  do {
    bVar15 = *param_4;
    if ((char)bVar15 < '\0') {
      uVar8 = bVar15 & 0x1f;
      pbVar1 = param_4 + 1;
      if (bVar15 < 0xe0) {
        param_4 = param_4 + 2;
        uVar12 = uVar8 << 6 | *pbVar1 & 0x3f;
      }
      else {
        uVar12 = param_4[2] & 0x3f | (*pbVar1 & 0x3f) << 6;
        if (bVar15 < 0xf0) {
          param_4 = param_4 + 3;
          uVar12 = uVar12 | uVar8 << 0xc;
        }
        else {
          uVar12 = param_4[3] & 0x3f | uVar12 << 6 | (bVar15 & 7) << 0x12;
          if (uVar12 == 0x110000) break;
          param_4 = param_4 + 4;
        }
      }
      bVar15 = (byte)uVar12;
      if (uVar12 < 0x80) goto code_r0x0001418c6e30;
    }
    else {
      param_4 = param_4 + 1;
code_r0x0001418c6e30:
      if (0xf < uVar6) goto code_r0x0001418c6fad;
      auStack_88[uVar6] = bVar15;
      if (uVar6 == 0xf) break;
      uVar6 = uVar6 + 1;
    }
  } while (param_4 != pbVar14);
code_r0x0001418c6e4c:
  uStack_78 = auStack_88._0_8_;
  uStack_70 = auStack_88._8_8_;
  lStack_68 = lStack_90;
  uStack_60 = 3;
  FUN_1418c1380(0x143634e40,&uStack_78,0x19);
  lVar13 = uVar9 * -0x100;
  pcVar10 = (char *)(uVar7 + lVar13 + -0xf8);
  LOCK();
  cVar16 = *pcVar10;
  if (cVar16 == '\0') {
    *pcVar10 = '\x01';
  }
  UNLOCK();
  if (cVar16 != '\0') {
    FUN_1428cd210(pcVar10);
  }
  FUN_140c81c70(pcVar10);
  uStack_78 = *param_7;
  uStack_70 = param_7[1];
  pcStack_50 = pcVar10;
  FUN_1418af800(param_6,&uStack_78,uVar7 + lVar13 + -0xf0);
  pcVar10 = pcStack_50;
  FUN_140c81e10();
  LOCK();
  cVar16 = *pcVar10;
  if (cVar16 == '\x01') {
    *pcVar10 = '\0';
  }
  UNLOCK();
  if (cVar16 != '\x01') {
    FUN_1428cdb90(pcVar10,0);
  }
  puVar5 = puStack_58;
  FUN_140c81e10(puStack_58);
  FUN_140c81e10((ulonglong)param_1 | 1);
  LOCK();
  uVar7 = *puVar5;
  *puVar5 = *puVar5 - 0x10;
  UNLOCK();
  if ((uVar7 & 0xfffffffffffffff2) != 0x12) {
    return;
  }
  FUN_1428ccf50(puVar5);
  return;
}

