
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

longlong * FUN_140154490(longlong *param_1,longlong *param_2,longlong param_3)

{
  float fVar1;
  float fVar2;
  float fVar3;
  undefined8 uVar4;
  undefined8 *puVar5;
  ulonglong uVar6;
  undefined1 auVar7 [16];
  longlong lVar8;
  longlong lVar9;
  ulonglong uVar10;
  longlong lVar11;
  float *pfVar12;
  undefined1 uVar13;
  float *pfVar14;
  ulonglong uVar15;
  longlong lVar16;
  ulonglong uVar17;
  longlong lVar18;
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
  float fVar43;
  float fVar44;
  float fVar45;
  
  puVar5 = (undefined8 *)param_2[1];
  uVar6 = param_2[2];
  if (2 < uVar6 && param_3 != 0) {
    fVar20 = (float)*puVar5;
    fVar21 = (float)((ulonglong)*puVar5 >> 0x20);
    fVar24 = *(float *)(puVar5 + 1);
    pfVar12 = (float *)((longlong)puVar5 + 0x14);
    uVar15 = uVar6 - 2;
    auVar7._8_8_ = 0;
    auVar7._0_8_ = uVar6 - 3;
    lVar9 = SUB168(auVar7 * ZEXT816(0xc),0);
    uVar10 = uVar15 & 0xfffffffffffffffc;
    lVar11 = 0;
    fVar25 = _DAT_142925870;
    fVar26 = _DAT_1429258e0;
    fVar19 = _UNK_1429258e4;
    do {
      uVar4 = *(undefined8 *)((longlong)puVar5 + 0xcU);
      fVar27 = (float)uVar4;
      fVar23 = (float)((ulonglong)uVar4 >> 0x20);
      fVar22 = *pfVar12;
      lVar16 = 1;
      fVar28 = fVar23;
      fVar30 = fVar20;
      fVar31 = fVar21;
      fVar32 = fVar24;
      if ((uVar15 < 0xc) ||
         (uVar17 = uVar10, pfVar14 = (float *)((longlong)puVar5 + 0x44), fVar29 = fVar22,
         fVar33 = fVar21, fVar37 = fVar24, fVar41 = fVar27, fVar44 = fVar20,
         SUB168(auVar7 * ZEXT816(0xc),8) != 0 ||
         ((((undefined8 *)((longlong)puVar5 + lVar9 + 0xc) < (undefined8 *)((longlong)puVar5 + 0xcU)
           || (undefined8 *)((longlong)puVar5 + lVar9 + 0x10) < puVar5 + 2) ||
          ((float *)((longlong)puVar5 + lVar9 + 0x14) < pfVar12 ||
          (undefined8 *)((longlong)puVar5 + lVar9 + 0x20) < puVar5 + 4)) ||
         ((undefined8 *)((longlong)puVar5 + lVar9 + 0x18) < puVar5 + 3 ||
         (ulonglong)((longlong)puVar5 + lVar9 + 0x1c) < (longlong)puVar5 + 0x1cU)))) {
LAB_140154663:
        lVar18 = (uVar6 - 1) - lVar16;
        pfVar14 = pfVar12 + lVar16 * 3;
        do {
          fVar33 = fVar28;
          fVar29 = fVar27;
          fVar23 = fVar22;
          fVar22 = *pfVar14;
          fVar27 = (float)*(undefined8 *)(pfVar14 + -2);
          fVar28 = (float)((ulonglong)*(undefined8 *)(pfVar14 + -2) >> 0x20);
          *(ulonglong *)(pfVar14 + -5) =
               CONCAT44(((fVar31 + fVar28) * fVar19 - fVar33) * fVar19 + fVar33,
                        ((fVar30 + fVar27) * fVar26 - fVar29) * fVar26 + fVar29);
          pfVar14[-3] = ((fVar32 + fVar22) * fVar25 - fVar23) * fVar25 + fVar23;
          pfVar14 = pfVar14 + 3;
          lVar18 = lVar18 + -1;
          fVar30 = fVar29;
          fVar31 = fVar33;
          fVar32 = fVar23;
        } while (lVar18 != 0);
      }
      else {
        do {
          fVar25 = pfVar14[-0xb];
          fVar26 = pfVar14[-10];
          fVar19 = pfVar14[-9];
          fVar1 = pfVar14[-8];
          fVar30 = (float)*(undefined8 *)(pfVar14 + -5);
          fVar31 = (float)((ulonglong)*(undefined8 *)(pfVar14 + -5) >> 0x20);
          fVar27 = (float)*(undefined8 *)(pfVar14 + -2);
          fVar28 = (float)((ulonglong)*(undefined8 *)(pfVar14 + -2) >> 0x20);
          fVar2 = pfVar14[-7];
          fVar32 = pfVar14[-3];
          fVar22 = *pfVar14;
          fVar3 = pfVar14[-6];
          fVar42 = ((fVar41 + fVar1) * _UNK_14293ac64 - fVar25) * _UNK_14293ac64;
          fVar43 = ((fVar25 + fVar30) * _UNK_14293ac68 - fVar1) * _UNK_14293ac68;
          fVar45 = ((fVar1 + fVar27) * _UNK_14293ac6c - fVar30) * _UNK_14293ac6c;
          fVar33 = ((fVar33 + fVar26) * _DAT_14293ac60 - fVar23) * _DAT_14293ac60;
          fVar34 = ((fVar23 + fVar2) * _UNK_14293ac64 - fVar26) * _UNK_14293ac64;
          fVar35 = ((fVar26 + fVar31) * _UNK_14293ac68 - fVar2) * _UNK_14293ac68;
          fVar36 = ((fVar2 + fVar28) * _UNK_14293ac6c - fVar31) * _UNK_14293ac6c;
          fVar37 = ((fVar37 + fVar19) * _DAT_14293ac60 - fVar29) * _DAT_14293ac60;
          fVar38 = ((fVar29 + fVar3) * _UNK_14293ac64 - fVar19) * _UNK_14293ac64;
          fVar39 = ((fVar19 + fVar32) * _UNK_14293ac68 - fVar3) * _UNK_14293ac68;
          fVar40 = ((fVar3 + fVar22) * _UNK_14293ac6c - fVar32) * _UNK_14293ac6c;
          pfVar14[-0xe] = ((fVar44 + fVar25) * _DAT_14293ac60 - fVar41) * _DAT_14293ac60 + fVar41;
          pfVar14[-0xd] = fVar33 + fVar23;
          pfVar14[-0xc] = fVar37 + fVar29;
          pfVar14[-0xb] = fVar42 + fVar25;
          pfVar14[-10] = fVar34 + fVar26;
          pfVar14[-9] = fVar38 + fVar19;
          pfVar14[-8] = fVar43 + fVar1;
          pfVar14[-7] = fVar35 + fVar2;
          pfVar14[-6] = fVar39 + fVar3;
          pfVar14[-5] = fVar45 + fVar30;
          pfVar14[-4] = fVar36 + fVar31;
          pfVar14[-3] = fVar40 + fVar32;
          uVar17 = uVar17 - 4;
          pfVar14 = pfVar14 + 0xc;
          fVar23 = fVar28;
          fVar29 = fVar22;
          fVar33 = fVar31;
          fVar37 = fVar32;
          fVar41 = fVar27;
          fVar44 = fVar30;
        } while (uVar17 != 0);
        lVar16 = uVar10 + 1;
        fVar25 = _DAT_142925870;
        fVar26 = _DAT_1429258e0;
        fVar19 = _UNK_1429258e4;
        if (uVar15 != uVar10) goto LAB_140154663;
      }
      lVar11 = lVar11 + 1;
    } while (lVar11 != param_3);
  }
  param_2[5] = 0;
  if (uVar6 < 2) {
    uVar13 = 0;
    uVar15 = uVar6;
  }
  else {
    uVar4 = *puVar5;
    fVar24 = *(float *)(puVar5 + 1);
    if ((ulonglong)param_2[3] < uVar6) {
      FUN_1428a29e0(param_2 + 3,0,uVar6,4,4);
      lVar9 = param_2[5];
    }
    else {
      lVar9 = 0;
    }
    pfVar12 = (float *)param_2[4];
    lVar11 = lVar9 * 4;
    uVar15 = -uVar6;
    pfVar14 = (float *)(puVar5 + 1);
    fVar19 = 0.0;
    lVar16 = 0;
    fVar25 = (float)uVar4;
    fVar26 = (float)((ulonglong)uVar4 >> 0x20);
    do {
      fVar20 = *pfVar14;
      fVar24 = fVar24 - fVar20;
      fVar21 = (float)*(undefined8 *)(pfVar14 + -2);
      fVar22 = (float)((ulonglong)*(undefined8 *)(pfVar14 + -2) >> 0x20);
      fVar25 = fVar25 - fVar21;
      fVar26 = fVar26 - fVar22;
      fVar19 = fVar19 + SQRT(fVar24 * fVar24 + fVar26 * fVar26 + fVar25 * fVar25);
      *(float *)((longlong)pfVar12 + lVar11) = fVar19;
      lVar11 = lVar11 + 4;
      lVar16 = lVar16 + -1;
      pfVar14 = pfVar14 + 3;
      fVar24 = fVar20;
      fVar25 = fVar21;
      fVar26 = fVar22;
    } while (-lVar16 != uVar6);
    param_2[5] = lVar9 - lVar16;
    uVar13 = 1;
    if ((fVar19 != 0.0) || (NAN(fVar19))) {
      do {
        if (lVar11 == 0) {
          *(float *)(param_2 + 6) = fVar19;
          lVar9 = *param_2;
          lVar11 = param_2[1];
          lVar16 = param_2[2];
          lVar18 = param_2[3];
          lVar8 = param_2[5];
          param_1[4] = param_2[4];
          param_1[5] = lVar8;
          param_1[2] = lVar16;
          param_1[3] = lVar18;
          *param_1 = lVar9;
          param_1[1] = lVar11;
          param_1[6] = param_2[6];
          return param_1;
        }
        fVar24 = *pfVar12;
        *pfVar12 = fVar24 / fVar19;
        pfVar12 = pfVar12 + 1;
        lVar11 = lVar11 + -4;
      } while ((uint)ABS(fVar24 / fVar19) < 0x7f800000);
    }
  }
  *(undefined1 *)(param_1 + 1) = uVar13;
  *(char *)((longlong)param_1 + 9) = (char)uVar15;
  *param_1 = -0x8000000000000000;
  if (*param_2 != 0) {
    func_0x000140613c20(param_2[1],*param_2 * 0xc,4);
  }
  if (param_2[3] != 0) {
    func_0x000140613c20(param_2[4],param_2[3] << 2,4);
  }
  return param_1;
}

