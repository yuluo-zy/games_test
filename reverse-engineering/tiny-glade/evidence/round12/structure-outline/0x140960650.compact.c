/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */
void FUN_140960650(undefined8 *param_1,undefined8 *param_2,undefined8 param_3)
{
  char *pcVar1;
  float fVar2;
  float fVar3;
  int *piVar4;
  longlong *plVar5;
  longlong lVar6;
  undefined8 *puVar7;
  float *pfVar8;
  code *pcVar9;
  uint uVar10;
  uint uVar11;
  undefined8 uVar12;
  ulonglong uVar13;
  longlong lVar14;
  longlong lVar15;
  uint uVar16;
  ushort uVar17;
  longlong lVar18;
  byte bVar19;
  char cVar20;
  char cVar22;
  char cVar23;
  char cVar24;
  undefined1 auVar21 [16];
  undefined1 auVar25 [16];
  undefined1 auVar26 [16];
  float fVar27;
  float fVar28;
  float fVar29;
  float fVar30;
  float fVar31;
  undefined8 uStack_138;
  undefined4 uStack_130;
  undefined8 uStack_128;
  int iStack_120;
  int iStack_10c;
  undefined4 uStack_108;
  longlong lStack_100;
  longlong lStack_f8;
  longlong lStack_f0;
  longlong lStack_e8;
  undefined8 uStack_e0;
  piVar4 = (int *)*param_2;
  if (*piVar4 == 1) {
    param_1[2] = *(undefined8 *)(piVar4 + 5);
    uVar12 = *(undefined8 *)(piVar4 + 3);
    *param_1 = *(undefined8 *)(piVar4 + 1);
    param_1[1] = uVar12;
    return;
  }
  plVar5 = *(longlong **)param_2[1];
  lVar14 = *(longlong *)param_2[2];
  lStack_100 = lVar14;
  if (plVar5[3] != 0) {
    uVar13 = FUN_1405b0d70(plVar5 + 4,&lStack_100);
    lVar6 = *plVar5;
    bVar19 = (byte)(uVar13 >> 0x39);
    auVar21 = ZEXT216(CONCAT11(bVar19,bVar19));
    auVar21 = pshuflw(auVar21,auVar21,0);
    lVar15 = 0;
    while( true ) {
      uVar13 = plVar5[1] & uVar13;
      pcVar1 = (char *)(lVar6 + uVar13);
      cVar20 = auVar21[0];
      auVar26[0] = -(*pcVar1 == cVar20);
      cVar22 = auVar21[1];
      auVar26[1] = -(pcVar1[1] == cVar22);
      cVar23 = auVar21[2];
      auVar26[2] = -(pcVar1[2] == cVar23);
      cVar24 = auVar21[3];
      auVar26[3] = -(pcVar1[3] == cVar24);
      auVar26[4] = -(pcVar1[4] == cVar20);
      auVar26[5] = -(pcVar1[5] == cVar22);
      auVar26[6] = -(pcVar1[6] == cVar23);
      auVar26[7] = -(pcVar1[7] == cVar24);
      auVar26[8] = -(pcVar1[8] == cVar20);
      auVar26[9] = -(pcVar1[9] == cVar22);
      auVar26[10] = -(pcVar1[10] == cVar23);
      auVar26[0xb] = -(pcVar1[0xb] == cVar24);
      auVar26[0xc] = -(pcVar1[0xc] == cVar20);
      auVar26[0xd] = -(pcVar1[0xd] == cVar22);
      auVar26[0xe] = -(pcVar1[0xe] == cVar23);
      auVar26[0xf] = -(pcVar1[0xf] == cVar24);
      uVar17 = (ushort)(SUB161(auVar26 >> 7,0) & 1) | (ushort)(SUB161(auVar26 >> 0xf,0) & 1) << 1 |
               (ushort)(SUB161(auVar26 >> 0x17,0) & 1) << 2 |
               (ushort)(SUB161(auVar26 >> 0x1f,0) & 1) << 3 |
               (ushort)(SUB161(auVar26 >> 0x27,0) & 1) << 4 |
               (ushort)(SUB161(auVar26 >> 0x2f,0) & 1) << 5 |
               (ushort)(SUB161(auVar26 >> 0x37,0) & 1) << 6 |
               (ushort)(SUB161(auVar26 >> 0x3f,0) & 1) << 7 |
               (ushort)(SUB161(auVar26 >> 0x47,0) & 1) << 8 |
               (ushort)(SUB161(auVar26 >> 0x4f,0) & 1) << 9 |
               (ushort)(SUB161(auVar26 >> 0x57,0) & 1) << 10 |
               (ushort)(SUB161(auVar26 >> 0x5f,0) & 1) << 0xb |
               (ushort)(SUB161(auVar26 >> 0x67,0) & 1) << 0xc |
               (ushort)(SUB161(auVar26 >> 0x6f,0) & 1) << 0xd |
               (ushort)(SUB161(auVar26 >> 0x77,0) & 1) << 0xe | (ushort)(auVar26[0xf] >> 7) << 0xf;
      uVar16 = (uint)uVar17;
      while (uVar17 != 0) {
        uVar10 = 0;
        for (uVar11 = uVar16; (uVar11 & 1) == 0; uVar11 = uVar11 >> 1 | 0x80000000) {
          uVar10 = uVar10 + 1;
        }
        lVar18 = (uVar10 + uVar13 & plVar5[1]) * -0x2a8;
        if (*(longlong *)(lVar6 + -0x2a8 + lVar18) == lVar14) {
          if ((*(ulonglong *)(lVar6 + -0x290 + lVar18) < 2) ||
             (*(float *)(lVar6 + -0x270 + lVar18) <= 0.0)) goto LAB_140960a8c;
          if (*(longlong *)(lVar6 + lVar18 + -600) != 0) {
            puVar7 = *(undefined8 **)(lVar6 + lVar18 + -0x260);
            uStack_130 = *(undefined4 *)(puVar7 + 1);
            uStack_138 = *puVar7;
            iStack_120 = *(int *)(puVar7 + 1);
            uStack_128 = *puVar7;
            fVar27 = 0.0;
            FUN_140c97e10(&lStack_100,param_3,uVar13,0);
            if (lStack_f0 != 0) {
              pfVar8 = *(float **)param_2[4];
              lVar14 = 0;
              fVar28 = _DAT_142925954;
              do {
                puVar7 = (undefined8 *)(lStack_f8 + lVar14);
                fVar31 = *(float *)(lStack_f8 + lVar14);
                fVar2 = *(float *)(lStack_f8 + 4 + lVar14);
                fVar3 = *(float *)(lStack_f8 + 8 + lVar14);
                fVar29 = fVar3 * pfVar8[0x28] + fVar2 * pfVar8[0x24] + fVar31 * pfVar8[0x20] +
                         pfVar8[0x2c];
                fVar30 = fVar3 * pfVar8[0x29] + fVar2 * pfVar8[0x25] + fVar31 * pfVar8[0x21] +
                         pfVar8[0x2d];
                fVar31 = fVar3 * pfVar8[0x2a] + fVar2 * pfVar8[0x26] + fVar31 * pfVar8[0x22] +
                         pfVar8[0x2e];
                fVar31 = (((fVar31 * pfVar8[8] + fVar30 * pfVar8[4] + fVar29 * *pfVar8 + pfVar8[0xc]
                           ) / (fVar31 * pfVar8[0xb] + fVar30 * pfVar8[7] + fVar29 * pfVar8[3] +
                               pfVar8[0xf])) * _DAT_142925870 + _DAT_142925870) *
                         (float)**(uint **)param_2[3];
                if (fVar27 < fVar31) {
                  uStack_130 = *(undefined4 *)(puVar7 + 1);
                  uStack_138 = *puVar7;
                  fVar27 = fVar31;
                }
                if (fVar31 < fVar28) {
                  iStack_120 = *(int *)(puVar7 + 1);
                  uStack_128 = *puVar7;
                  fVar28 = fVar31;
                }
                lVar14 = lVar14 + 0xc;
              } while (lStack_f0 * 0xc != lVar14);
            }
            if (lStack_100 != 0) {
              func_0x000140613c20(lStack_f8,lStack_100 * 0xc,4);
            }
            if (lStack_e8 != 0) {
              func_0x000140613c20(uStack_e0,lStack_e8 << 2,4);
            }
            iStack_10c = (int)uStack_138;
            uStack_108 = (undefined4)((ulonglong)uStack_138 >> 0x20);
            *piVar4 = 1;
            *(ulonglong *)(piVar4 + 5) = CONCAT44(uStack_130,uStack_108);
            *(undefined8 *)(piVar4 + 1) = uStack_128;
            piVar4[3] = iStack_120;
            piVar4[4] = iStack_10c;
            *param_1 = uStack_128;
            *(int *)(param_1 + 1) = iStack_120;
            *(undefined8 *)((longlong)param_1 + 0xc) = uStack_138;
            *(undefined4 *)((longlong)param_1 + 0x14) = uStack_130;
            return;
          }
          goto LAB_140960aa4;
        }
        uVar17 = (ushort)(uVar16 - 1) & (ushort)uVar16;
        uVar16 = CONCAT22((short)(uVar16 - 1 >> 0x10),uVar17);
      }
      auVar25[0] = -(*pcVar1 == -1);
      auVar25[1] = -(pcVar1[1] == -1);
      auVar25[2] = -(pcVar1[2] == -1);
      auVar25[3] = -(pcVar1[3] == -1);
      auVar25[4] = -(pcVar1[4] == -1);
      auVar25[5] = -(pcVar1[5] == -1);
      auVar25[6] = -(pcVar1[6] == -1);
      auVar25[7] = -(pcVar1[7] == -1);
      auVar25[8] = -(pcVar1[8] == -1);
      auVar25[9] = -(pcVar1[9] == -1);
      auVar25[10] = -(pcVar1[10] == -1);
      auVar25[0xb] = -(pcVar1[0xb] == -1);
      auVar25[0xc] = -(pcVar1[0xc] == -1);
      auVar25[0xd] = -(pcVar1[0xd] == -1);
      auVar25[0xe] = -(pcVar1[0xe] == -1);
      auVar25[0xf] = -(pcVar1[0xf] == -1);
      if ((((((((((((((((SUB161(auVar25 >> 7,0) & 1) != 0 || (SUB161(auVar25 >> 0xf,0) & 1) != 0) ||
                      (SUB161(auVar25 >> 0x17,0) & 1) != 0) || (SUB161(auVar25 >> 0x1f,0) & 1) != 0)
                    || (SUB161(auVar25 >> 0x27,0) & 1) != 0) || (SUB161(auVar25 >> 0x2f,0) & 1) != 0
                   ) || (SUB161(auVar25 >> 0x37,0) & 1) != 0) ||
                 (SUB161(auVar25 >> 0x3f,0) & 1) != 0) || (SUB161(auVar25 >> 0x47,0) & 1) != 0) ||
               (SUB161(auVar25 >> 0x4f,0) & 1) != 0) || (SUB161(auVar25 >> 0x57,0) & 1) != 0) ||
             (SUB161(auVar25 >> 0x5f,0) & 1) != 0) || (SUB161(auVar25 >> 0x67,0) & 1) != 0) ||
           (SUB161(auVar25 >> 0x6f,0) & 1) != 0) || (SUB161(auVar25 >> 0x77,0) & 1) != 0) ||
          auVar25[0xf] < '\0') break;
      uVar13 = uVar13 + lVar15 + 0x10;
      lVar15 = lVar15 + 0x10;
    }
  }
  FUN_1428d9310(&UNK_142ab52e0);
LAB_140960a8c:
  FUN_1428d9430(&UNK_142ada193,0x32,&UNK_142ab52c8);
LAB_140960aa4:
  FUN_1428d9518(0,0,&UNK_142ab52f8);
  pcVar9 = (code *)swi(3);
  (*pcVar9)();
  return;
}
