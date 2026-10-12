// FUN_14098f9f0  0x14098f9f0

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void FUN_14098f9f0(undefined8 param_1,longlong param_2)

{
  int *piVar1;
  char *pcVar2;
  int iVar3;
  ulonglong uVar4;
  code *pcVar5;
  uint uVar6;
  uint uVar7;
  longlong lVar8;
  ulonglong uVar9;
  longlong lVar10;
  ulonglong uVar11;
  longlong lVar12;
  uint uVar13;
  ushort uVar14;
  ulonglong uVar15;
  bool bVar16;
  undefined1 auVar17 [16];
  undefined1 auVar18 [16];
  undefined1 auVar19 [16];
  undefined1 auVar20 [16];
  undefined8 auStack_3e8 [50];
  undefined8 uStack_258;
  undefined8 uStack_48;
  
  uStack_48 = 0xfffffffffffffffe;
  uVar11 = 0xe67d7e3df983f8ce;
  if (*(longlong *)(param_2 + 0x88) != 0) {
    lVar8 = *(longlong *)(param_2 + 0x70);
    uVar4 = *(ulonglong *)(param_2 + 0x78);
    uVar9 = uVar4 & 0xe67d7e3df983f8ce;
    lVar12 = 0;
    while( true ) {
      pcVar2 = (char *)(lVar8 + uVar9);
      auVar19[0] = -(*pcVar2 == UNK_1429616f0);
      auVar19[1] = -(pcVar2[1] == UNK_1429616f0._1_1_);
      auVar19[2] = -(pcVar2[2] == UNK_1429616f0._2_1_);
      auVar19[3] = -(pcVar2[3] == UNK_1429616f0._3_1_);
      auVar19[4] = -(pcVar2[4] == UNK_1429616f0._4_1_);
      auVar19[5] = -(pcVar2[5] == UNK_1429616f0._5_1_);
      auVar19[6] = -(pcVar2[6] == UNK_1429616f0._6_1_);
      auVar19[7] = -(pcVar2[7] == UNK_1429616f0._7_1_);
      auVar19[8] = -(pcVar2[8] == UNK_1429616f0._8_1_);
      auVar19[9] = -(pcVar2[9] == UNK_1429616f0._9_1_);
      auVar19[10] = -(pcVar2[10] == UNK_1429616f0._10_1_);
      auVar19[0xb] = -(pcVar2[0xb] == UNK_1429616f0._11_1_);
      auVar19[0xc] = -(pcVar2[0xc] == UNK_1429616f0._12_1_);
      auVar19[0xd] = -(pcVar2[0xd] == UNK_1429616f0._13_1_);
      auVar19[0xe] = -(pcVar2[0xe] == UNK_1429616f0._14_1_);
      auVar19[0xf] = -(pcVar2[0xf] == UNK_1429616f0._15_1_);
      uVar14 = (ushort)(SUB161(auVar19 >> 7,0) & 1) | (ushort)(SUB161(auVar19 >> 0xf,0) & 1) << 1 |
               (ushort)(SUB161(auVar19 >> 0x17,0) & 1) << 2 |
               (ushort)(SUB161(auVar19 >> 0x1f,0) & 1) << 3 |
               (ushort)(SUB161(auVar19 >> 0x27,0) & 1) << 4 |
               (ushort)(SUB161(auVar19 >> 0x2f,0) & 1) << 5 |
               (ushort)(SUB161(auVar19 >> 0x37,0) & 1) << 6 |
               (ushort)(SUB161(auVar19 >> 0x3f,0) & 1) << 7 |
               (ushort)(SUB161(auVar19 >> 0x47,0) & 1) << 8 |
               (ushort)(SUB161(auVar19 >> 0x4f,0) & 1) << 9 |
               (ushort)(SUB161(auVar19 >> 0x57,0) & 1) << 10 |
               (ushort)(SUB161(auVar19 >> 0x5f,0) & 1) << 0xb |
               (ushort)(SUB161(auVar19 >> 0x67,0) & 1) << 0xc |
               (ushort)(SUB161(auVar19 >> 0x6f,0) & 1) << 0xd |
               (ushort)(SUB161(auVar19 >> 0x77,0) & 1) << 0xe | (ushort)(auVar19[0xf] >> 7) << 0xf;
      uVar13 = (uint)uVar14;
      while (uVar14 != 0) {
        uVar6 = 0;
        for (uVar7 = uVar13; (uVar7 & 1) == 0; uVar7 = uVar7 >> 1 | 0x80000000) {
          uVar6 = uVar6 + 1;
        }
        uVar15 = uVar6 + uVar9 & uVar4;
        if ((*(longlong *)(lVar8 + -0x18 + uVar15 * -0x18) == -0x36a12780becf8f55) &&
           (*(longlong *)(lVar8 + -0x10 + uVar15 * -0x18) == -0x198281c2067c0732)) {
          uVar11 = *(ulonglong *)(lVar8 + uVar15 * -0x18 + -8);
          if (uVar11 < *(ulonglong *)(param_2 + 0x218)) goto code_r0x00014098fbec;
          goto code_r0x000140990689;
        }
        uVar14 = (ushort)(uVar13 - 1) & (ushort)uVar13;
        uVar13 = CONCAT22((short)(uVar13 - 1 >> 0x10),uVar14);
      }
      auVar17[0] = -(*pcVar2 == -1);
      auVar17[1] = -(pcVar2[1] == -1);
      auVar17[2] = -(pcVar2[2] == -1);
      auVar17[3] = -(pcVar2[3] == -1);
      auVar17[4] = -(pcVar2[4] == -1);
      auVar17[5] = -(pcVar2[5] == -1);
      auVar17[6] = -(pcVar2[6] == -1);
      auVar17[7] = -(pcVar2[7] == -1);
      auVar17[8] = -(pcVar2[8] == -1);
      auVar17[9] = -(pcVar2[9] == -1);
      auVar17[10] = -(pcVar2[10] == -1);
      auVar17[0xb] = -(pcVar2[0xb] == -1);
      auVar17[0xc] = -(pcVar2[0xc] == -1);
      auVar17[0xd] = -(pcVar2[0xd] == -1);
      auVar17[0xe] = -(pcVar2[0xe] == -1);
      auVar17[0xf] = -(pcVar2[0xf] == -1);
      if ((((((((((((((((SUB161(auVar17 >> 7,0) & 1) != 0 || (SUB161(auVar17 >> 0xf,0) & 1) != 0) ||
                      (SUB161(auVar17 >> 0x17,0) & 1) != 0) || (SUB161(auVar17 >> 0x1f,0) & 1) != 0)
                    || (SUB161(auVar17 >> 0x27,0) & 1) != 0) || (SUB161(auVar17 >> 0x2f,0) & 1) != 0
                   ) || (SUB161(auVar17 >> 0x37,0) & 1) != 0) ||
                 (SUB161(auVar17 >> 0x3f,0) & 1) != 0) || (SUB161(auVar17 >> 0x47,0) & 1) != 0) ||
               (SUB161(auVar17 >> 0x4f,0) & 1) != 0) || (SUB161(auVar17 >> 0x57,0) & 1) != 0) ||
             (SUB161(auVar17 >> 0x5f,0) & 1) != 0) || (SUB161(auVar17 >> 0x67,0) & 1) != 0) ||
           (SUB161(auVar17 >> 0x6f,0) & 1) != 0) || (SUB161(auVar17 >> 0x77,0) & 1) != 0) ||
          auVar17[0xf] < '\0') break;
      lVar10 = uVar9 + lVar12;
      lVar12 = lVar12 + 0x10;
      uVar9 = lVar10 + 0x10U & uVar4;
    }
  }
  uVar13 = *(uint *)(param_2 + 0x90);
  if (uVar13 < 0x3ffffffe) {
    LOCK();
    bVar16 = uVar13 == *(uint *)(param_2 + 0x90);
    if (bVar16) {
      *(uint *)(param_2 + 0x90) = uVar13 + 1;
    }
    UNLOCK();
    if (bVar16) {
      lVar8 = *(longlong *)(param_2 + 0xf0);
      goto joined_r0x000140990669;
    }
  }
  FUN_1428d8a30(param_2 + 0x90);
  lVar8 = *(longlong *)(param_2 + 0xf0);
joined_r0x000140990669:
  if (lVar8 != 0) {
    lVar8 = *(longlong *)(param_2 + 0xd8);
    uVar4 = *(ulonglong *)(param_2 + 0xe0);
    uVar9 = uVar4 & 0xe67d7e3df983f8ce;
    lVar12 = 0;
    while( true ) {
      pcVar2 = (char *)(lVar8 + uVar9);
      auVar20[0] = -(*pcVar2 == UNK_1429616f0);
      auVar20[1] = -(pcVar2[1] == UNK_1429616f0._1_1_);
      auVar20[2] = -(pcVar2[2] == UNK_1429616f0._2_1_);
      auVar20[3] = -(pcVar2[3] == UNK_1429616f0._3_1_);
      auVar20[4] = -(pcVar2[4] == UNK_1429616f0._4_1_);
      auVar20[5] = -(pcVar2[5] == UNK_1429616f0._5_1_);
      auVar20[6] = -(pcVar2[6] == UNK_1429616f0._6_1_);
      auVar20[7] = -(pcVar2[7] == UNK_1429616f0._7_1_);
      auVar20[8] = -(pcVar2[8] == UNK_1429616f0._8_1_);
      auVar20[9] = -(pcVar2[9] == UNK_1429616f0._9_1_);
      auVar20[10] = -(pcVar2[10] == UNK_1429616f0._10_1_);
      auVar20[0xb] = -(pcVar2[0xb] == UNK_1429616f0._11_1_);
      auVar20[0xc] = -(pcVar2[0xc] == UNK_1429616f0._12_1_);
      auVar20[0xd] = -(pcVar2[0xd] == UNK_1429616f0._13_1_);
      auVar20[0xe] = -(pcVar2[0xe] == UNK_1429616f0._14_1_);
      auVar20[0xf] = -(pcVar2[0xf] == UNK_1429616f0._15_1_);
      uVar14 = (ushort)(SUB161(auVar20 >> 7,0) & 1) | (ushort)(SUB161(auVar20 >> 0xf,0) & 1) << 1 |
               (ushort)(SUB161(auVar20 >> 0x17,0) & 1) << 2 |
               (ushort)(SUB161(auVar20 >> 0x1f,0) & 1) << 3 |
               (ushort)(SUB161(auVar20 >> 0x27,0) & 1) << 4 |
               (ushort)(SUB161(auVar20 >> 0x2f,0) & 1) << 5 |
               (ushort)(SUB161(auVar20 >> 0x37,0) & 1) << 6 |
               (ushort)(SUB161(auVar20 >> 0x3f,0) & 1) << 7 |
               (ushort)(SUB161(auVar20 >> 0x47,0) & 1) << 8 |
               (ushort)(SUB161(auVar20 >> 0x4f,0) & 1) << 9 |
               (ushort)(SUB161(auVar20 >> 0x57,0) & 1) << 10 |
               (ushort)(SUB161(auVar20 >> 0x5f,0) & 1) << 0xb |
               (ushort)(SUB161(auVar20 >> 0x67,0) & 1) << 0xc |
               (ushort)(SUB161(auVar20 >> 0x6f,0) & 1) << 0xd |
               (ushort)(SUB161(auVar20 >> 0x77,0) & 1) << 0xe | (ushort)(auVar20[0xf] >> 7) << 0xf;
      uVar13 = (uint)uVar14;
      while (uVar14 != 0) {
        uVar6 = 0;
        for (uVar7 = uVar13; (uVar7 & 1) == 0; uVar7 = uVar7 >> 1 | 0x80000000) {
          uVar6 = uVar6 + 1;
        }
        lVar10 = (uVar6 + uVar9 & uVar4) * -0x88;
        if ((*(longlong *)(lVar8 + -0x88 + lVar10) == -0x36a12780becf8f55) &&
           (*(longlong *)(lVar8 + -0x80 + lVar10) == -0x198281c2067c0732)) {
          uVar11 = *(ulonglong *)(lVar8 + lVar10 + -8);
          LOCK();
          piVar1 = (int *)(param_2 + 0x90);
          iVar3 = *piVar1;
          *piVar1 = *piVar1 + -1;
          UNLOCK();
          if (SBORROW4(0,iVar3 - 1U & 0xbfffffff)) goto code_r0x000140990695;
          if (uVar11 < *(ulonglong *)(param_2 + 0x218)) goto code_r0x00014098fbec;
          goto code_r0x000140990689;
        }
        uVar14 = (ushort)(uVar13 - 1) & (ushort)uVar13;
        uVar13 = CONCAT22((short)(uVar13 - 1 >> 0x10),uVar14);
      }
      auVar18[0] = -(*pcVar2 == -1);
      auVar18[1] = -(pcVar2[1] == -1);
      auVar18[2] = -(pcVar2[2] == -1);
      auVar18[3] = -(pcVar2[3] == -1);
      auVar18[4] = -(pcVar2[4] == -1);
      auVar18[5] = -(pcVar2[5] == -1);
      auVar18[6] = -(pcVar2[6] == -1);
      auVar18[7] = -(pcVar2[7] == -1);
      auVar18[8] = -(pcVar2[8] == -1);
      auVar18[9] = -(pcVar2[9] == -1);
      auVar18[10] = -(pcVar2[10] == -1);
      auVar18[0xb] = -(pcVar2[0xb] == -1);
      auVar18[0xc] = -(pcVar2[0xc] == -1);
      auVar18[0xd] = -(pcVar2[0xd] == -1);
      auVar18[0xe] = -(pcVar2[0xe] == -1);
      auVar18[0xf] = -(pcVar2[0xf] == -1);
      if ((((((((((((((((SUB161(auVar18 >> 7,0) & 1) != 0 || (SUB161(auVar18 >> 0xf,0) & 1) != 0) ||
                      (SUB161(auVar18 >> 0x17,0) & 1) != 0) || (SUB161(auVar18 >> 0x1f,0) & 1) != 0)
                    || (SUB161(auVar18 >> 0x27,0) & 1) != 0) || (SUB161(auVar18 >> 0x2f,0) & 1) != 0
                   ) || (SUB161(auVar18 >> 0x37,0) & 1) != 0) ||
                 (SUB161(auVar18 >> 0x3f,0) & 1) != 0) || (SUB161(auVar18 >> 0x47,0) & 1) != 0) ||
               (SUB161(auVar18 >> 0x4f,0) & 1) != 0) || (SUB161(auVar18 >> 0x57,0) & 1) != 0) ||
             (SUB161(auVar18 >> 0x5f,0) & 1) != 0) || (SUB161(auVar18 >> 0x67,0) & 1) != 0) ||
           (SUB161(auVar18 >> 0x6f,0) & 1) != 0) || (SUB161(auVar18 >> 0x77,0) & 1) != 0) ||
          auVar18[0xf] < '\0') break;
      lVar10 = uVar9 + lVar12;
      lVar12 = lVar12 + 0x10;
      uVar9 = lVar10 + 0x10U & uVar4;
    }
  }
  LOCK();
  piVar1 = (int *)(param_2 + 0x90);
  iVar3 = *piVar1;
  *piVar1 = *piVar1 + -1;
  UNLOCK();
  if (SBORROW4(0,iVar3 - 1U & 0xbfffffff)) {
    FUN_1428d8d40(param_2 + 0x90);
    FUN_1428d9310(&UNK_142ab9e58);
    auStack_3e8[0] = uStack_258;
    FUN_1428d9760(&UNK_142ab9ad8,0x2b,auStack_3e8,&UNK_142ab9ab8,&UNK_142ab9e90);
                    /* WARNING: Does not return */
    pcVar5 = (code *)invalidInstructionException();
    (*pcVar5)();
  }
code_r0x000140990689:
  do {
    do {
      do {
        FUN_1428d9310(&UNK_142ab9e58);
code_r0x000140990695:
        FUN_1428d8d40(param_2 + 0x90);
      } while (*(ulonglong *)(param_2 + 0x218) <= uVar11);
code_r0x00014098fbec:
      uVar4 = *(ulonglong *)(*(longlong *)(param_2 + 0x210) + uVar11 * 8);
    } while (uVar4 == 0);
    lVar8 = ~uVar4 * 0x60;
  } while (*(longlong *)(*(longlong *)(param_2 + 0x1e0) + 0x20 + lVar8) == 0);
                    /* WARNING: Could not recover jumptable at 0x00014098fc3c. Too many branches */
                    /* WARNING: Treating indirect jump as call */
  (*(code *)(&UNK_142ab9ff0 +
            *(int *)(&UNK_142ab9ff0 +
                    (ulonglong)**(uint **)(*(longlong *)(param_2 + 0x1e0) + lVar8 + 0x10) * 4)))
            (&UNK_142ab9ff0,*(undefined4 *)(param_2 + 0x5b0));
  return;
}

