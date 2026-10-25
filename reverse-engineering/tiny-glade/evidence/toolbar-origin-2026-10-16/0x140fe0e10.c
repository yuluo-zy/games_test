
/* WARNING: Type propagation algorithm not settling */

uint FUN_140fe0e10(char *param_1,char *param_2,longlong *param_3)

{
  char *pcVar1;
  longlong *plVar2;
  longlong *plVar3;
  longlong lVar4;
  uint uVar5;
  uint uVar6;
  ulonglong uVar7;
  longlong lVar8;
  longlong lVar9;
  uint uVar10;
  ushort uVar11;
  longlong lVar12;
  longlong *plVar13;
  char cVar14;
  byte bVar15;
  char cVar17;
  char cVar18;
  char cVar19;
  undefined1 auVar16 [16];
  undefined1 auVar20 [16];
  undefined1 auVar21 [16];
  float fVar22;
  undefined1 auVar23 [16];
  undefined1 auVar24 [16];
  float fVar25;
  float fVar26;
  float fVar27;
  float fVar28;
  longlong alStack_90 [10];
  
  fVar26 = 0.0;
  if (param_1[0x20] == '\0') {
    fVar26 = *(float *)(param_1 + 0x24);
  }
  plVar13 = (longlong *)*param_3;
  plVar2 = (longlong *)param_3[1];
  uVar7 = (ulonglong)((longlong)plVar2 - (longlong)plVar13) >> 3;
  lVar8 = 0;
  if (uVar7 != 0) {
    lVar8 = uVar7 - 1;
  }
  fVar26 = (float)lVar8 * fVar26;
  plVar3 = (longlong *)param_3[2];
  if (param_1[0x44] == '\0') {
    if (plVar13 == plVar2) {
      cVar14 = *param_2;
      goto joined_r0x000140fe1215;
    }
    fVar27 = 0.0;
LAB_140fe1092:
    lVar8 = *plVar13;
    alStack_90[0] = lVar8;
    if (plVar3[3] != 0) {
      plVar13 = plVar13 + 1;
      uVar7 = FUN_1405b0d70(plVar3 + 4,alStack_90);
      lVar4 = *plVar3;
      bVar15 = (byte)(uVar7 >> 0x39);
      auVar16 = ZEXT216(CONCAT11(bVar15,bVar15));
      auVar16 = pshuflw(auVar16,auVar16,0);
      lVar9 = 0;
      while( true ) {
        uVar7 = plVar3[1] & uVar7;
        pcVar1 = (char *)(lVar4 + uVar7);
        cVar14 = auVar16[0];
        auVar24[0] = -(*pcVar1 == cVar14);
        cVar17 = auVar16[1];
        auVar24[1] = -(pcVar1[1] == cVar17);
        cVar18 = auVar16[2];
        auVar24[2] = -(pcVar1[2] == cVar18);
        cVar19 = auVar16[3];
        auVar24[3] = -(pcVar1[3] == cVar19);
        auVar24[4] = -(pcVar1[4] == cVar14);
        auVar24[5] = -(pcVar1[5] == cVar17);
        auVar24[6] = -(pcVar1[6] == cVar18);
        auVar24[7] = -(pcVar1[7] == cVar19);
        auVar24[8] = -(pcVar1[8] == cVar14);
        auVar24[9] = -(pcVar1[9] == cVar17);
        auVar24[10] = -(pcVar1[10] == cVar18);
        auVar24[0xb] = -(pcVar1[0xb] == cVar19);
        auVar24[0xc] = -(pcVar1[0xc] == cVar14);
        auVar24[0xd] = -(pcVar1[0xd] == cVar17);
        auVar24[0xe] = -(pcVar1[0xe] == cVar18);
        auVar24[0xf] = -(pcVar1[0xf] == cVar19);
        uVar11 = (ushort)(SUB161(auVar24 >> 7,0) & 1) | (ushort)(SUB161(auVar24 >> 0xf,0) & 1) << 1
                 | (ushort)(SUB161(auVar24 >> 0x17,0) & 1) << 2 |
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
                 (ushort)(SUB161(auVar24 >> 0x77,0) & 1) << 0xe | (ushort)(auVar24[0xf] >> 7) << 0xf
        ;
        uVar10 = (uint)uVar11;
        while (uVar11 != 0) {
          uVar5 = 0;
          for (uVar6 = uVar10; (uVar6 & 1) == 0; uVar6 = uVar6 >> 1 | 0x80000000) {
            uVar5 = uVar5 + 1;
          }
          lVar12 = (uVar5 + uVar7 & plVar3[1]) * -0xc0;
          if (*(longlong *)(lVar4 + -0xc0 + lVar12) == lVar8) {
            fVar22 = *(float *)(lVar4 + -0x44 + lVar12);
            fVar26 = fVar26 + *(float *)(lVar4 + -0x48 + lVar12);
            fVar25 = fVar22;
            if (fVar22 <= fVar27) {
              fVar25 = fVar27;
            }
            fVar27 = (float)(~-(uint)NAN(fVar27) & (uint)fVar25 | -(uint)NAN(fVar27) & (uint)fVar22)
            ;
            if (plVar13 == plVar2) goto LAB_140fe1101;
            goto LAB_140fe1092;
          }
          uVar11 = (ushort)(uVar10 - 1) & (ushort)uVar10;
          uVar10 = CONCAT22((short)(uVar10 - 1 >> 0x10),uVar11);
        }
        auVar21[0] = -(*pcVar1 == -1);
        auVar21[1] = -(pcVar1[1] == -1);
        auVar21[2] = -(pcVar1[2] == -1);
        auVar21[3] = -(pcVar1[3] == -1);
        auVar21[4] = -(pcVar1[4] == -1);
        auVar21[5] = -(pcVar1[5] == -1);
        auVar21[6] = -(pcVar1[6] == -1);
        auVar21[7] = -(pcVar1[7] == -1);
        auVar21[8] = -(pcVar1[8] == -1);
        auVar21[9] = -(pcVar1[9] == -1);
        auVar21[10] = -(pcVar1[10] == -1);
        auVar21[0xb] = -(pcVar1[0xb] == -1);
        auVar21[0xc] = -(pcVar1[0xc] == -1);
        auVar21[0xd] = -(pcVar1[0xd] == -1);
        auVar21[0xe] = -(pcVar1[0xe] == -1);
        auVar21[0xf] = -(pcVar1[0xf] == -1);
        if ((((((((((((((((SUB161(auVar21 >> 7,0) & 1) != 0 || (SUB161(auVar21 >> 0xf,0) & 1) != 0)
                        || (SUB161(auVar21 >> 0x17,0) & 1) != 0) ||
                       (SUB161(auVar21 >> 0x1f,0) & 1) != 0) || (SUB161(auVar21 >> 0x27,0) & 1) != 0
                      ) || (SUB161(auVar21 >> 0x2f,0) & 1) != 0) ||
                    (SUB161(auVar21 >> 0x37,0) & 1) != 0) || (SUB161(auVar21 >> 0x3f,0) & 1) != 0)
                  || (SUB161(auVar21 >> 0x47,0) & 1) != 0) || (SUB161(auVar21 >> 0x4f,0) & 1) != 0)
                || (SUB161(auVar21 >> 0x57,0) & 1) != 0) || (SUB161(auVar21 >> 0x5f,0) & 1) != 0) ||
              (SUB161(auVar21 >> 0x67,0) & 1) != 0) || (SUB161(auVar21 >> 0x6f,0) & 1) != 0) ||
            (SUB161(auVar21 >> 0x77,0) & 1) != 0) || auVar21[0xf] < '\0') break;
        uVar7 = uVar7 + lVar9 + 0x10;
        lVar9 = lVar9 + 0x10;
      }
    }
LAB_140fe10e3:
    FUN_1428d9330(&UNK_142bae90e,0x16,&UNK_142baefe0);
  }
  else if (plVar13 != plVar2) {
    fVar26 = 0.0;
LAB_140fe0f66:
    lVar8 = *plVar13;
    alStack_90[0] = lVar8;
    if (plVar3[3] != 0) {
      plVar13 = plVar13 + 1;
      uVar7 = FUN_1405b0d70(plVar3 + 4,alStack_90);
      lVar4 = *plVar3;
      bVar15 = (byte)(uVar7 >> 0x39);
      auVar16 = ZEXT216(CONCAT11(bVar15,bVar15));
      auVar16 = pshuflw(auVar16,auVar16,0);
      lVar9 = 0;
      while( true ) {
        uVar7 = plVar3[1] & uVar7;
        pcVar1 = (char *)(lVar4 + uVar7);
        cVar14 = auVar16[0];
        auVar23[0] = -(*pcVar1 == cVar14);
        cVar17 = auVar16[1];
        auVar23[1] = -(pcVar1[1] == cVar17);
        cVar18 = auVar16[2];
        auVar23[2] = -(pcVar1[2] == cVar18);
        cVar19 = auVar16[3];
        auVar23[3] = -(pcVar1[3] == cVar19);
        auVar23[4] = -(pcVar1[4] == cVar14);
        auVar23[5] = -(pcVar1[5] == cVar17);
        auVar23[6] = -(pcVar1[6] == cVar18);
        auVar23[7] = -(pcVar1[7] == cVar19);
        auVar23[8] = -(pcVar1[8] == cVar14);
        auVar23[9] = -(pcVar1[9] == cVar17);
        auVar23[10] = -(pcVar1[10] == cVar18);
        auVar23[0xb] = -(pcVar1[0xb] == cVar19);
        auVar23[0xc] = -(pcVar1[0xc] == cVar14);
        auVar23[0xd] = -(pcVar1[0xd] == cVar17);
        auVar23[0xe] = -(pcVar1[0xe] == cVar18);
        auVar23[0xf] = -(pcVar1[0xf] == cVar19);
        uVar11 = (ushort)(SUB161(auVar23 >> 7,0) & 1) | (ushort)(SUB161(auVar23 >> 0xf,0) & 1) << 1
                 | (ushort)(SUB161(auVar23 >> 0x17,0) & 1) << 2 |
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
                 (ushort)(SUB161(auVar23 >> 0x77,0) & 1) << 0xe | (ushort)(auVar23[0xf] >> 7) << 0xf
        ;
        uVar10 = (uint)uVar11;
        while (uVar11 != 0) {
          uVar5 = 0;
          for (uVar6 = uVar10; (uVar6 & 1) == 0; uVar6 = uVar6 >> 1 | 0x80000000) {
            uVar5 = uVar5 + 1;
          }
          lVar12 = (uVar5 + uVar7 & plVar3[1]) * -0xc0;
          if (*(longlong *)(lVar4 + -0xc0 + lVar12) == lVar8) {
            fVar27 = *(float *)(lVar4 + -0x48 + lVar12);
            fVar22 = fVar27;
            if (fVar27 <= fVar26) {
              fVar22 = fVar26;
            }
            fVar26 = (float)(~-(uint)NAN(fVar26) & (uint)fVar22 | -(uint)NAN(fVar26) & (uint)fVar27)
            ;
            if (plVar13 == plVar2) goto LAB_140fe1101;
            goto LAB_140fe0f66;
          }
          uVar11 = (ushort)(uVar10 - 1) & (ushort)uVar10;
          uVar10 = CONCAT22((short)(uVar10 - 1 >> 0x10),uVar11);
        }
        auVar20[0] = -(*pcVar1 == -1);
        auVar20[1] = -(pcVar1[1] == -1);
        auVar20[2] = -(pcVar1[2] == -1);
        auVar20[3] = -(pcVar1[3] == -1);
        auVar20[4] = -(pcVar1[4] == -1);
        auVar20[5] = -(pcVar1[5] == -1);
        auVar20[6] = -(pcVar1[6] == -1);
        auVar20[7] = -(pcVar1[7] == -1);
        auVar20[8] = -(pcVar1[8] == -1);
        auVar20[9] = -(pcVar1[9] == -1);
        auVar20[10] = -(pcVar1[10] == -1);
        auVar20[0xb] = -(pcVar1[0xb] == -1);
        auVar20[0xc] = -(pcVar1[0xc] == -1);
        auVar20[0xd] = -(pcVar1[0xd] == -1);
        auVar20[0xe] = -(pcVar1[0xe] == -1);
        auVar20[0xf] = -(pcVar1[0xf] == -1);
        if ((((((((((((((((SUB161(auVar20 >> 7,0) & 1) != 0 || (SUB161(auVar20 >> 0xf,0) & 1) != 0)
                        || (SUB161(auVar20 >> 0x17,0) & 1) != 0) ||
                       (SUB161(auVar20 >> 0x1f,0) & 1) != 0) || (SUB161(auVar20 >> 0x27,0) & 1) != 0
                      ) || (SUB161(auVar20 >> 0x2f,0) & 1) != 0) ||
                    (SUB161(auVar20 >> 0x37,0) & 1) != 0) || (SUB161(auVar20 >> 0x3f,0) & 1) != 0)
                  || (SUB161(auVar20 >> 0x47,0) & 1) != 0) || (SUB161(auVar20 >> 0x4f,0) & 1) != 0)
                || (SUB161(auVar20 >> 0x57,0) & 1) != 0) || (SUB161(auVar20 >> 0x5f,0) & 1) != 0) ||
              (SUB161(auVar20 >> 0x67,0) & 1) != 0) || (SUB161(auVar20 >> 0x6f,0) & 1) != 0) ||
            (SUB161(auVar20 >> 0x77,0) & 1) != 0) || auVar20[0xf] < '\0') break;
        uVar7 = uVar7 + lVar9 + 0x10;
        lVar9 = lVar9 + 0x10;
      }
    }
    goto LAB_140fe10e3;
  }
  fVar26 = 0.0;
LAB_140fe1101:
  cVar14 = *param_2;
joined_r0x000140fe1215:
  if (cVar14 == '\0') {
    fVar27 = *(float *)(param_2 + 4);
    cVar14 = param_2[8];
  }
  else {
    fVar27 = 0.0;
    cVar14 = param_2[8];
  }
  if (cVar14 == '\0') {
    cVar14 = param_1[0x28];
  }
  else {
    cVar14 = param_1[0x28];
  }
  if (cVar14 == '\0') {
    fVar22 = *(float *)(param_1 + 0x2c);
    cVar14 = param_1[0x30];
  }
  else {
    fVar22 = 0.0;
    cVar14 = param_1[0x30];
  }
  if (cVar14 == '\0') {
    cVar14 = *param_1;
  }
  else {
    cVar14 = *param_1;
  }
  if (cVar14 == '\0') {
    fVar25 = *(float *)(param_1 + 4);
    cVar14 = param_1[8];
  }
  else {
    fVar25 = 0.0;
    cVar14 = param_1[8];
  }
  if (cVar14 == '\0') {
    cVar14 = param_1[0x10];
  }
  else {
    cVar14 = param_1[0x10];
  }
  if (cVar14 == '\0') {
    fVar28 = *(float *)(param_1 + 0x14);
  }
  else {
    fVar28 = 0.0;
  }
  fVar26 = fVar26 + fVar25 + fVar28;
  fVar25 = fVar26;
  if (fVar26 <= fVar22) {
    fVar25 = fVar22;
  }
  fVar22 = (float)(~-(uint)NAN(fVar22) & (uint)fVar25 | -(uint)NAN(fVar22) & (uint)fVar26);
  fVar26 = fVar22;
  if (fVar22 <= fVar27) {
    fVar26 = fVar27;
  }
  return ~-(uint)NAN(fVar27) & (uint)fVar26 | -(uint)NAN(fVar27) & (uint)fVar22;
}

