/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */
undefined8 *
FUN_140c926b0(undefined8 *param_1,longlong param_2,float *param_3,ulonglong param_4,
             undefined4 *param_5,undefined4 *param_6)
{
  ulonglong uVar1;
  float *pfVar2;
  float *pfVar3;
  float fVar4;
  float fVar5;
  undefined4 uVar6;
  undefined4 uVar7;
  ulonglong *puVar8;
  code *pcVar9;
  undefined8 uVar10;
  undefined8 uVar11;
  float *pfVar12;
  float *pfVar13;
  ulonglong uVar14;
  undefined4 *puVar15;
  ulonglong uVar16;
  longlong lVar17;
  ulonglong uVar18;
  longlong lVar19;
  undefined4 *puVar20;
  float *pfVar21;
  ulonglong uVar22;
  longlong lVar23;
  ulonglong uVar24;
  undefined4 *puVar25;
  ulonglong uVar26;
  undefined4 *puVar27;
  ulonglong uVar28;
  float *pfVar29;
  longlong lVar30;
  undefined4 *puVar31;
  undefined1 auVar32 [16];
  float fVar33;
  float fVar34;
  float fVar36;
  float fVar37;
  undefined1 auVar35 [16];
  float *pfStack_b8;
  longlong lStack_b0;
  ulonglong uStack_a8;
  ulonglong uStack_a0;
  longlong lStack_90;
  ulonglong uStack_88;
  float *pfStack_80;
  undefined4 *puStack_78;
  ulonglong uStack_70;
  ulonglong uStack_68;
  undefined8 uStack_60;
  uStack_60 = 0xfffffffffffffffe;
  if (param_4 < 2) {
LAB_140c92fcb:
    FUN_1428d9430(&UNK_142b2dbd8,0x23,&UNK_142b2dc20);
LAB_140c92fe3:
    FUN_1428d9330(&UNK_142b2db40,0x22,&UNK_142b2dbc0);
  }
  else {
    uVar1 = param_4 * 4 - 4;
    puVar8 = *(ulonglong **)(param_2 + 0x10);
    uVar28 = puVar8[4] & 0xfffffffffffffffc;
    uStack_70 = param_4;
    if ((uVar28 < *puVar8) || (uVar28 - *puVar8 < uVar1)) {
LAB_140c92f62:
      pfVar29 = (float *)FUN_1428d7780(param_2,4,uVar1);
      if (pfVar29 != (float *)0x0) goto LAB_140c9272d;
LAB_140c92fc6:
      FUN_1428d79d0();
      goto LAB_140c92fcb;
    }
    pfVar29 = (float *)(uVar28 - uVar1);
    puVar8[4] = (ulonglong)pfVar29;
    if (pfVar29 == (float *)0x0) goto LAB_140c92f62;
LAB_140c9272d:
    lVar17 = uStack_70 * 2;
    uStack_68 = uStack_70 - 1;
    fVar33 = (float)*(undefined8 *)(param_3 + 2) - (float)*(undefined8 *)param_3;
    fVar34 = (float)((ulonglong)*(undefined8 *)(param_3 + 2) >> 0x20) -
             (float)((ulonglong)*(undefined8 *)param_3 >> 0x20);
    *pfVar29 = SQRT(fVar34 * fVar34 + fVar33 * fVar33);
    if (uStack_68 == 1) {
      uStack_88 = 2;
      uVar28 = 8;
    }
    else {
      pfVar21 = param_3 + 2;
      pfVar13 = param_3 + 4;
      uVar24 = uStack_70 * 8 - 0x10 >> 3;
      uVar28 = uStack_70 - 3;
      uVar14 = uVar28;
      if (uVar24 < uVar28) {
        uVar14 = uVar24;
      }
      uVar16 = 1;
      pfVar12 = pfVar13;
      if (3 < uVar14) {
        if (uVar24 < uVar28) {
          uVar28 = uVar24;
        }
        if ((param_3 + uVar28 * 2 + 6 <= pfVar29 + 1) || (pfVar29 + uVar28 + 2 <= pfVar21)) {
          uVar14 = uVar14 + 1;
          uVar28 = 4;
          if ((uVar14 & 3) != 0) {
            uVar28 = (ulonglong)((uint)uVar14 & 3);
          }
          lVar19 = uVar14 - uVar28;
          pfVar21 = pfVar21 + lVar19 * 2;
          uVar16 = (uVar14 - uVar28) + 1;
          lVar30 = 0;
          do {
            pfVar2 = param_3 + lVar30 * 2 + 2;
            pfVar12 = pfVar13 + lVar30 * 2;
            auVar32 = *(undefined1 (*) [16])(param_3 + lVar30 * 2 + 6);
            pfVar3 = param_3 + lVar30 * 2 + 8;
            fVar33 = *pfVar3 - auVar32._0_4_;
            fVar34 = pfVar3[1] - auVar32._4_4_;
            fVar36 = pfVar3[2] - auVar32._8_4_;
            fVar37 = pfVar3[3] - auVar32._12_4_;
            auVar35._4_4_ =
                 (pfVar12[2] - pfVar2[2]) * (pfVar12[2] - pfVar2[2]) +
                 (pfVar12[3] - pfVar2[3]) * (pfVar12[3] - pfVar2[3]);
            auVar35._0_4_ =
                 (*pfVar12 - *pfVar2) * (*pfVar12 - *pfVar2) +
                 (pfVar12[1] - pfVar2[1]) * (pfVar12[1] - pfVar2[1]);
            auVar35._8_4_ = fVar33 * fVar33 + fVar34 * fVar34;
            auVar35._12_4_ = fVar36 * fVar36 + fVar37 * fVar37;
            auVar32 = sqrtps(auVar32,auVar35);
            *(undefined1 (*) [16])(pfVar29 + lVar30 + 1) = auVar32;
            lVar30 = lVar30 + 4;
            pfVar12 = pfVar13 + lVar19 * 2;
          } while (lVar19 != lVar30);
        }
      }
      do {
        pfVar13 = pfVar12;
        if (pfVar13 == param_3 + lVar17) goto LAB_140c92fe3;
        fVar33 = (float)*(undefined8 *)pfVar13 - (float)*(undefined8 *)pfVar21;
        fVar34 = (float)((ulonglong)*(undefined8 *)pfVar13 >> 0x20) -
                 (float)((ulonglong)*(undefined8 *)pfVar21 >> 0x20);
        pfVar29[uVar16] = SQRT(fVar34 * fVar34 + fVar33 * fVar33);
        uVar16 = uVar16 + 1;
        pfVar12 = pfVar13 + 2;
        pfVar21 = pfVar13;
      } while (uStack_68 != uVar16);
      if (uStack_68 == 0x1fffffffffffffff) goto LAB_140c92fc6;
      uVar28 = uStack_70 * 4;
      uStack_88 = uStack_70;
    }
    puVar8 = *(ulonglong **)(param_2 + 0x10);
    uVar14 = puVar8[4] & 0xfffffffffffffffc;
    if ((uVar14 < *puVar8) || (uVar14 - *puVar8 < uVar28)) {
LAB_140c92f87:
      puVar15 = (undefined4 *)FUN_1428d7780(param_2);
      if (puVar15 == (undefined4 *)0x0) goto LAB_140c92fc6;
    }
    else {
      puVar15 = (undefined4 *)(uVar14 - uVar28);
      puVar8[4] = (ulonglong)puVar15;
      if (puVar15 == (undefined4 *)0x0) goto LAB_140c92f87;
    }
    puStack_78 = puVar15;
    func_0x000142923c20(puVar15,0,param_4 * 4);
    uVar28 = uStack_68;
    uVar14 = 0;
    if ((uStack_88 & 0x3fffffffffffffff) != 0) {
      uVar14 = (uStack_88 & 0x3fffffffffffffff) - 1;
    }
    if (uStack_68 < uVar14) {
      uVar14 = uStack_68;
    }
    if (uVar14 != 0) {
      uVar24 = (ulonglong)((uint)uVar14 & 3);
      if (uVar14 < 4) {
        fVar33 = 0.0;
        uVar18 = 0;
      }
      else {
        fVar33 = 0.0;
        uVar16 = 0;
        do {
          fVar34 = pfVar29[uVar16];
          puStack_78[uVar16 + 1] = fVar33 + fVar34;
          fVar33 = fVar33 + fVar34 + pfVar29[uVar16 + 1];
          puStack_78[uVar16 + 2] = fVar33;
          fVar33 = fVar33 + pfVar29[uVar16 + 2];
          puStack_78[uVar16 + 3] = fVar33;
          fVar33 = fVar33 + pfVar29[uVar16 + 3];
          uVar18 = uVar16 + 4;
          puStack_78[uVar16 + 4] = fVar33;
          uVar16 = uVar18;
        } while ((uVar14 & 0x3ffffffffffffffc) != uVar18);
      }
      if (uVar24 != 0) {
        uVar14 = 0;
        do {
          fVar33 = fVar33 + pfVar29[uVar18 + uVar14];
          puStack_78[uVar18 + uVar14 + 1] = fVar33;
          uVar14 = uVar14 + 1;
        } while (uVar24 != uVar14);
      }
    }
    if (uStack_68 >> 0x3d != 0) goto LAB_140c92fc6;
    puVar8 = *(ulonglong **)(param_2 + 0x10);
    uVar14 = puVar8[4] & 0xfffffffffffffffc;
    lStack_90 = param_2;
    if ((uVar14 < *puVar8) || (uVar14 - *puVar8 < uVar1)) {
LAB_140c92f9f:
      uVar14 = FUN_1428d7780(param_2,4,uVar1);
      if (uVar14 == 0) goto LAB_140c92fc6;
    }
    else {
      uVar14 = uVar14 - uVar1;
      puVar8[4] = uVar14;
      if (uVar14 == 0) goto LAB_140c92f9f;
    }
    puVar15 = puStack_78;
    uVar1 = uStack_88;
    auVar32 = _DAT_142925930;
    uVar24 = uStack_70 - 2;
    if (uVar28 < uStack_70 - 2) {
      uVar24 = uVar28;
    }
    uVar16 = 0;
    pfVar21 = pfVar29;
    if ((3 < uVar24) && (0xf < uVar14 - (longlong)pfVar29)) {
      uVar24 = uVar24 + 1;
      uVar16 = 4;
      if ((uVar24 & 3) != 0) {
        uVar16 = (ulonglong)((uint)uVar24 & 3);
      }
      uVar16 = uVar24 - uVar16;
      pfVar21 = pfVar29 + uVar16;
      uVar24 = 0;
      do {
        auVar35 = divps(auVar32,*(undefined1 (*) [16])(pfVar29 + uVar24));
        *(undefined1 (*) [16])(uVar14 + uVar24 * 4) = auVar35;
        uVar24 = uVar24 + 4;
      } while (uVar16 != uVar24);
    }
    fVar33 = _DAT_142925904;
    do {
      if (pfVar21 == pfVar29 + uVar28) goto LAB_140c92fe3;
      fVar34 = *pfVar21;
      pfVar21 = pfVar21 + 1;
      *(float *)(uVar14 + uVar16 * 4) = fVar33 / fVar34;
      uVar16 = uVar16 + 1;
    } while (uVar28 != uVar16);
    pfStack_b8 = (float *)0x4;
    lStack_b0 = lStack_90;
    uStack_a8 = 0;
    uStack_a0 = 0;
    FUN_141940de0(&pfStack_b8,0,uStack_70,1);
    pfVar21 = param_3;
    do {
      fVar33 = *pfVar21;
      if (uStack_a0 == uStack_a8) {
        FUN_141940de0(&pfStack_b8,uStack_a0,1,1);
      }
      uVar28 = uStack_a8;
      lVar30 = lStack_b0;
      pfVar21 = pfVar21 + 2;
      pfStack_b8[uStack_a0] = fVar33;
      uVar24 = uStack_a0 + 1;
      uStack_a0 = uVar24;
    } while (pfVar21 != param_3 + lVar17);
    uVar6 = *param_5;
    uVar7 = *param_6;
    pfStack_80 = pfStack_b8;
    puVar25 = puStack_78;
    lVar17 = FUN_140c94a10(lStack_90,puStack_78,uStack_88,pfVar29,uStack_68,uVar14,uStack_68,
                           pfStack_b8,uVar24,uVar6,param_5[1],uVar7,param_6[1]);
    uVar16 = uStack_70;
    if (uVar24 <= uStack_70) {
      uVar16 = uVar24;
    }
    if (uVar24 != 0) {
      if (uVar16 < 0x10) {
        uVar18 = 0;
      }
      else if (pfStack_80 < param_3 + uVar16 * 2 && param_3 + 1 < pfStack_80 + uVar16) {
        uVar18 = 0;
      }
      else {
        uVar18 = uVar16 & 0x3ffffffffffffff8;
        uVar22 = 0;
        do {
          fVar33 = param_3[uVar22 * 2 + 5];
          fVar34 = param_3[uVar22 * 2 + 7];
          fVar36 = param_3[uVar22 * 2 + 0xf];
          fVar37 = param_3[uVar22 * 2 + 0xd];
          fVar4 = param_3[uVar22 * 2 + 0xb];
          fVar5 = param_3[uVar22 * 2 + 9];
          *(ulonglong *)(pfStack_80 + uVar22) =
               CONCAT44(param_3[uVar22 * 2 + 3],param_3[uVar22 * 2 + 1]);
          *(ulonglong *)(pfStack_80 + uVar22 + 2) = CONCAT44(fVar34,fVar33);
          pfVar21 = pfStack_80 + uVar22 + 4;
          *pfVar21 = fVar5;
          pfVar21[1] = fVar4;
          pfVar21[2] = fVar37;
          pfVar21[3] = fVar36;
          uVar22 = uVar22 + 8;
        } while (uVar18 != uVar22);
        if (uVar16 == uVar18) goto LAB_140c92d8f;
      }
      uVar22 = uVar18;
      for (uVar26 = uVar16 & 3; uVar26 != 0; uVar26 = uVar26 - 1) {
        pfStack_80[uVar22] = param_3[uVar22 * 2 + 1];
        uVar22 = uVar22 + 1;
      }
      if (uVar18 - uVar16 < 0xfffffffffffffffd) {
        do {
          pfStack_80[uVar22] = param_3[uVar22 * 2 + 1];
          pfStack_80[uVar22 + 1] = param_3[uVar22 * 2 + 3];
          pfStack_80[uVar22 + 2] = param_3[uVar22 * 2 + 5];
          pfStack_80[uVar22 + 3] = param_3[uVar22 * 2 + 7];
          uVar22 = uVar22 + 4;
        } while (uVar16 != uVar22);
      }
    }
LAB_140c92d8f:
    puVar20 = puStack_78;
    lVar19 = FUN_140c94a10(lStack_90,puStack_78,uStack_88,pfVar29,uStack_68,uVar14,uStack_68,
                           pfStack_80,uVar24,uVar6,param_5[2],uVar7,param_6[2]);
    if (puVar20 < puVar25) {
      puVar25 = puVar20;
    }
    if (puVar25 < (undefined4 *)0x2e8ba2e8ba2e8bb) {
      puVar8 = *(ulonglong **)(lStack_90 + 0x10);
      uVar14 = puVar8[4] & 0xfffffffffffffffc;
      if ((uVar14 < *puVar8) ||
         (uVar14 - *puVar8 <= (ulonglong)((longlong)puVar25 * 0x2c) &&
          (longlong)puVar25 * 0x2c - (uVar14 - *puVar8) != 0)) {
LAB_140c92e4a:
        puVar20 = (undefined4 *)FUN_1428d7780(lStack_90,4);
        if (puVar20 == (undefined4 *)0x0) {
          FUN_1428d79d0();
          goto LAB_140c93015;
        }
      }
      else {
        puVar20 = (undefined4 *)(uVar14 + (longlong)puVar25 * -0x2c);
        puVar8[4] = (ulonglong)puVar20;
        if (puVar20 == (undefined4 *)0x0) goto LAB_140c92e4a;
      }
      if (puVar25 != (undefined4 *)0x0) {
        lVar23 = 0;
        puVar27 = puVar25;
        puVar31 = puVar20;
        do {
          uVar6 = *(undefined4 *)(lVar17 + 0x10 + lVar23);
          uStack_a8 = CONCAT44(uStack_a8._4_4_,uVar6);
          pfStack_b8 = *(float **)(lVar17 + lVar23);
          lStack_b0 = ((undefined8 *)(lVar17 + lVar23))[1];
          uVar7 = *(undefined4 *)(lVar19 + 0x10 + lVar23);
          uVar10 = *(undefined8 *)(lVar19 + lVar23);
          uVar11 = ((undefined8 *)(lVar19 + lVar23))[1];
          *puVar31 = *(undefined4 *)(lVar17 + 0x10 + lVar23);
          *(float **)(puVar31 + 1) = pfStack_b8;
          *(longlong *)(puVar31 + 3) = lStack_b0;
          puVar31[5] = uVar6;
          *(undefined8 *)(puVar31 + 6) = uVar10;
          *(undefined8 *)(puVar31 + 8) = uVar11;
          puVar31[10] = uVar7;
          lVar23 = lVar23 + 0x14;
          puVar31 = puVar31 + 0xb;
          puVar27 = (undefined4 *)((longlong)puVar27 + -1);
        } while (puVar27 != (undefined4 *)0x0);
      }
      if (puVar15 + (uVar1 - 1) != (undefined4 *)0x0) {
        uVar6 = *puStack_78;
        uVar7 = puVar15[uVar1 - 1];
        *param_1 = puVar20;
        param_1[1] = puVar25;
        *(undefined4 *)(param_1 + 2) = uVar6;
        *(undefined4 *)((longlong)param_1 + 0x14) = uVar7;
        *(undefined1 *)(param_1 + 3) = 0;
        if (uVar28 != 0) {
          pfVar29 = *(float **)(*(longlong *)(lVar30 + 0x10) + 0x20);
          if (pfVar29 == pfStack_80) {
            *(float **)(*(longlong *)(lVar30 + 0x10) + 0x20) = pfVar29 + uVar28;
          }
        }
        return param_1;
      }
      FUN_1428d9310(&UNK_142b2dc38);
      goto LAB_140c93015;
    }
  }
  FUN_1428d79d0();
LAB_140c93015:
                    /* WARNING: Does not return */
  pcVar9 = (code *)invalidInstructionException();
  (*pcVar9)();
}
