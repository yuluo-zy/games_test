
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void FUN_141465260(undefined8 *param_1,ulonglong param_2,longlong param_3,char param_4)

{
  ulonglong uVar1;
  undefined1 auVar2 [16];
  undefined1 auVar3 [16];
  longlong lVar4;
  ulonglong uVar5;
  float *pfVar6;
  ulonglong uVar7;
  longlong lVar8;
  longlong lVar9;
  float *pfVar10;
  ulonglong uVar11;
  float fVar12;
  float fVar13;
  float fVar14;
  float fVar15;
  float fVar16;
  undefined8 uVar17;
  float fVar18;
  float fVar19;
  float fVar20;
  float fVar21;
  float fVar22;
  float fVar23;
  float fVar24;
  float fVar25;
  float fVar26;
  float fVar27;
  float fVar28;
  float fVar29;
  float fVar30;
  float fVar31;
  float fVar32;
  float fVar33;
  float fVar34;
  float fVar35;
  float fVar36;
  float fVar37;
  float fVar38;
  float fVar39;
  float fVar40;
  float fVar41;
  float fVar42;
  
  if (2 < param_2 && param_3 != 0) {
    uVar1 = param_2 - 1;
    if (param_4 == '\0') {
      fVar13 = *(float *)(param_1 + 1);
      fVar12 = (float)*param_1;
      fVar15 = (float)((ulonglong)*param_1 >> 0x20);
      pfVar6 = (float *)((longlong)param_1 + 0x14);
      uVar7 = param_2 - 2;
      auVar3._8_8_ = 0;
      auVar3._0_8_ = param_2 - 3;
      lVar4 = SUB168(auVar3 * ZEXT816(0xc),0);
      uVar5 = uVar7 & 0xfffffffffffffffc;
      lVar8 = 0;
      fVar21 = _DAT_1429258e0;
      fVar19 = _UNK_1429258e4;
      fVar20 = _DAT_142925870;
      do {
        uVar17 = *(undefined8 *)((longlong)param_1 + 0xcU);
        fVar33 = (float)uVar17;
        fVar14 = (float)((ulonglong)uVar17 >> 0x20);
        fVar16 = *pfVar6;
        lVar9 = 1;
        fVar23 = fVar14;
        fVar27 = fVar12;
        fVar28 = fVar15;
        fVar29 = fVar13;
        if ((uVar7 < 0xc) ||
           (uVar11 = uVar5, pfVar10 = (float *)((longlong)param_1 + 0x44), fVar26 = fVar16,
           fVar34 = fVar15, fVar38 = fVar13, fVar18 = fVar33, fVar22 = fVar12,
           SUB168(auVar3 * ZEXT816(0xc),8) != 0 ||
           ((((undefined8 *)((longlong)param_1 + lVar4 + 0xc) <
              (undefined8 *)((longlong)param_1 + 0xcU) ||
             (undefined8 *)((longlong)param_1 + lVar4 + 0x10) < param_1 + 2) ||
            ((float *)((longlong)param_1 + lVar4 + 0x14) < pfVar6 ||
            (undefined8 *)((longlong)param_1 + lVar4 + 0x20) < param_1 + 4)) ||
           ((undefined8 *)((longlong)param_1 + lVar4 + 0x18) < param_1 + 3 ||
           (ulonglong)((longlong)param_1 + lVar4 + 0x1c) < (longlong)param_1 + 0x1cU)))) {
LAB_14146579f:
          pfVar10 = pfVar6 + lVar9 * 3;
          lVar9 = uVar1 - lVar9;
          do {
            fVar34 = fVar23;
            fVar26 = fVar33;
            fVar14 = fVar16;
            fVar16 = *pfVar10;
            fVar33 = (float)*(undefined8 *)(pfVar10 + -2);
            fVar23 = (float)((ulonglong)*(undefined8 *)(pfVar10 + -2) >> 0x20);
            *(ulonglong *)(pfVar10 + -5) =
                 CONCAT44(((fVar28 + fVar23) * fVar19 - fVar34) * fVar19 + fVar34,
                          ((fVar27 + fVar33) * fVar21 - fVar26) * fVar21 + fVar26);
            pfVar10[-3] = ((fVar29 + fVar16) * fVar20 - fVar14) * fVar20 + fVar14;
            pfVar10 = pfVar10 + 3;
            lVar9 = lVar9 + -1;
            fVar27 = fVar26;
            fVar28 = fVar34;
            fVar29 = fVar14;
          } while (lVar9 != 0);
        }
        else {
          do {
            fVar21 = pfVar10[-0xb];
            fVar19 = pfVar10[-10];
            fVar20 = pfVar10[-9];
            fVar24 = pfVar10[-8];
            fVar27 = (float)*(undefined8 *)(pfVar10 + -5);
            fVar28 = (float)((ulonglong)*(undefined8 *)(pfVar10 + -5) >> 0x20);
            fVar33 = (float)*(undefined8 *)(pfVar10 + -2);
            fVar23 = (float)((ulonglong)*(undefined8 *)(pfVar10 + -2) >> 0x20);
            fVar25 = pfVar10[-7];
            fVar29 = pfVar10[-3];
            fVar16 = *pfVar10;
            fVar30 = pfVar10[-6];
            fVar40 = ((fVar18 + fVar24) * _UNK_14293ac64 - fVar21) * _UNK_14293ac64;
            fVar41 = ((fVar21 + fVar27) * _UNK_14293ac68 - fVar24) * _UNK_14293ac68;
            fVar42 = ((fVar24 + fVar33) * _UNK_14293ac6c - fVar27) * _UNK_14293ac6c;
            fVar34 = ((fVar34 + fVar19) * _DAT_14293ac60 - fVar14) * _DAT_14293ac60;
            fVar31 = ((fVar14 + fVar25) * _UNK_14293ac64 - fVar19) * _UNK_14293ac64;
            fVar32 = ((fVar19 + fVar28) * _UNK_14293ac68 - fVar25) * _UNK_14293ac68;
            fVar35 = ((fVar25 + fVar23) * _UNK_14293ac6c - fVar28) * _UNK_14293ac6c;
            fVar38 = ((fVar38 + fVar20) * _DAT_14293ac60 - fVar26) * _DAT_14293ac60;
            fVar36 = ((fVar26 + fVar30) * _UNK_14293ac64 - fVar20) * _UNK_14293ac64;
            fVar37 = ((fVar20 + fVar29) * _UNK_14293ac68 - fVar30) * _UNK_14293ac68;
            fVar39 = ((fVar30 + fVar16) * _UNK_14293ac6c - fVar29) * _UNK_14293ac6c;
            pfVar10[-0xe] = ((fVar22 + fVar21) * _DAT_14293ac60 - fVar18) * _DAT_14293ac60 + fVar18;
            pfVar10[-0xd] = fVar34 + fVar14;
            pfVar10[-0xc] = fVar38 + fVar26;
            pfVar10[-0xb] = fVar40 + fVar21;
            pfVar10[-10] = fVar31 + fVar19;
            pfVar10[-9] = fVar36 + fVar20;
            pfVar10[-8] = fVar41 + fVar24;
            pfVar10[-7] = fVar32 + fVar25;
            pfVar10[-6] = fVar37 + fVar30;
            pfVar10[-5] = fVar42 + fVar27;
            pfVar10[-4] = fVar35 + fVar28;
            pfVar10[-3] = fVar39 + fVar29;
            uVar11 = uVar11 - 4;
            pfVar10 = pfVar10 + 0xc;
            fVar14 = fVar23;
            fVar26 = fVar16;
            fVar34 = fVar28;
            fVar38 = fVar29;
            fVar18 = fVar33;
            fVar22 = fVar27;
          } while (uVar11 != 0);
          lVar9 = uVar5 + 1;
          fVar21 = _DAT_1429258e0;
          fVar19 = _UNK_1429258e4;
          fVar20 = _DAT_142925870;
          if (uVar7 != uVar5) goto LAB_14146579f;
        }
        lVar8 = lVar8 + 1;
      } while (lVar8 != param_3);
    }
    else {
      auVar2._8_8_ = 0;
      auVar2._0_8_ = param_2 - 2;
      lVar4 = SUB168(auVar2 * ZEXT816(0xc),0);
      uVar5 = uVar1 & 0xfffffffffffffffc;
      lVar8 = 0;
      fVar13 = _DAT_142925870;
      fVar21 = _DAT_1429258e0;
      fVar19 = _UNK_1429258e4;
      do {
        fVar20 = *(float *)((longlong)param_1 + param_2 * 0xc + -0x10);
        uVar17 = *(undefined8 *)((longlong)param_1 + param_2 * 0xc + -0x18);
        fVar15 = (float)uVar17;
        fVar16 = (float)((ulonglong)uVar17 >> 0x20);
        uVar17 = *param_1;
        fVar12 = *(float *)(param_1 + 1);
        if ((uVar1 < 0xc) ||
           (SUB168(auVar2 * ZEXT816(0xc),8) != 0 ||
            ((((undefined8 *)((longlong)param_1 + lVar4) < param_1 ||
              (ulonglong)((longlong)param_1 + lVar4 + 4) < (longlong)param_1 + 4U) ||
             ((undefined8 *)((longlong)param_1 + lVar4 + 8) < param_1 + 1 ||
             (ulonglong)((longlong)param_1 + lVar4 + 0x14) < (longlong)param_1 + 0x14U)) ||
            ((ulonglong)((longlong)param_1 + lVar4 + 0xc) < (longlong)param_1 + 0xcU ||
            (undefined8 *)((longlong)param_1 + lVar4 + 0x10) < param_1 + 2)))) {
          uVar7 = 0;
LAB_14146544d:
          pfVar6 = (float *)((longlong)param_1 + 0x14U + uVar7 * 0xc);
          lVar9 = uVar1 - uVar7;
          do {
            fVar29 = fVar12;
            fVar14 = (float)uVar17;
            fVar33 = (float)((ulonglong)uVar17 >> 0x20);
            fVar12 = *pfVar6;
            uVar17 = *(undefined8 *)(pfVar6 + -2);
            *(ulonglong *)(pfVar6 + -5) =
                 CONCAT44(((fVar16 + (float)((ulonglong)uVar17 >> 0x20)) * fVar19 - fVar33) * fVar19
                          + fVar33,((fVar15 + (float)uVar17) * fVar21 - fVar14) * fVar21 + fVar14);
            pfVar6[-3] = ((fVar20 + fVar12) * fVar13 - fVar29) * fVar13 + fVar29;
            pfVar6 = pfVar6 + 3;
            lVar9 = lVar9 + -1;
            fVar15 = fVar14;
            fVar16 = fVar33;
            fVar20 = fVar29;
          } while (lVar9 != 0);
        }
        else {
          uVar7 = uVar5;
          pfVar6 = (float *)(param_1 + 7);
          fVar13 = (float)((ulonglong)uVar17 >> 0x20);
          fVar21 = fVar16;
          fVar19 = fVar12;
          fVar29 = (float)uVar17;
          fVar14 = fVar15;
          fVar33 = fVar20;
          do {
            fVar23 = pfVar6[-0xb];
            fVar27 = pfVar6[-10];
            fVar28 = pfVar6[-9];
            fVar26 = pfVar6[-8];
            fVar15 = (float)*(undefined8 *)(pfVar6 + -5);
            fVar16 = (float)((ulonglong)*(undefined8 *)(pfVar6 + -5) >> 0x20);
            uVar17 = *(undefined8 *)(pfVar6 + -2);
            fVar18 = (float)((ulonglong)uVar17 >> 0x20);
            fVar34 = pfVar6[-7];
            fVar20 = pfVar6[-3];
            fVar12 = *pfVar6;
            fVar38 = pfVar6[-6];
            fVar30 = ((fVar29 + fVar26) * _UNK_14293ac64 - fVar23) * _UNK_14293ac64;
            fVar31 = ((fVar23 + fVar15) * _UNK_14293ac68 - fVar26) * _UNK_14293ac68;
            fVar32 = ((fVar26 + (float)uVar17) * _UNK_14293ac6c - fVar15) * _UNK_14293ac6c;
            fVar21 = ((fVar21 + fVar27) * _DAT_14293ac60 - fVar13) * _DAT_14293ac60;
            fVar22 = ((fVar13 + fVar34) * _UNK_14293ac64 - fVar27) * _UNK_14293ac64;
            fVar24 = ((fVar27 + fVar16) * _UNK_14293ac68 - fVar34) * _UNK_14293ac68;
            fVar25 = ((fVar34 + fVar18) * _UNK_14293ac6c - fVar16) * _UNK_14293ac6c;
            fVar33 = ((fVar33 + fVar28) * _DAT_14293ac60 - fVar19) * _DAT_14293ac60;
            fVar35 = ((fVar19 + fVar38) * _UNK_14293ac64 - fVar28) * _UNK_14293ac64;
            fVar36 = ((fVar28 + fVar20) * _UNK_14293ac68 - fVar38) * _UNK_14293ac68;
            fVar37 = ((fVar38 + fVar12) * _UNK_14293ac6c - fVar20) * _UNK_14293ac6c;
            pfVar6[-0xe] = ((fVar14 + fVar23) * _DAT_14293ac60 - fVar29) * _DAT_14293ac60 + fVar29;
            pfVar6[-0xd] = fVar21 + fVar13;
            pfVar6[-0xc] = fVar33 + fVar19;
            pfVar6[-0xb] = fVar30 + fVar23;
            pfVar6[-10] = fVar22 + fVar27;
            pfVar6[-9] = fVar35 + fVar28;
            pfVar6[-8] = fVar31 + fVar26;
            pfVar6[-7] = fVar24 + fVar34;
            pfVar6[-6] = fVar36 + fVar38;
            pfVar6[-5] = fVar32 + fVar15;
            pfVar6[-4] = fVar25 + fVar16;
            pfVar6[-3] = fVar37 + fVar20;
            pfVar6 = pfVar6 + 0xc;
            uVar7 = uVar7 - 4;
            fVar13 = fVar18;
            fVar21 = fVar16;
            fVar19 = fVar12;
            fVar29 = (float)uVar17;
            fVar14 = fVar15;
            fVar33 = fVar20;
          } while (uVar7 != 0);
          uVar7 = uVar5;
          fVar13 = _DAT_142925870;
          fVar21 = _DAT_1429258e0;
          fVar19 = _UNK_1429258e4;
          if (uVar1 != uVar5) goto LAB_14146544d;
        }
        lVar8 = lVar8 + 1;
        uVar17 = *param_1;
        *(undefined4 *)((longlong)param_1 + param_2 * 0xc + -4) = *(undefined4 *)(param_1 + 1);
        *(undefined8 *)((longlong)param_1 + param_2 * 0xc + -0xc) = uVar17;
      } while (lVar8 != param_3);
    }
  }
  return;
}

