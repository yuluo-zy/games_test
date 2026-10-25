/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */
undefined8 *
FUN_140c93970(undefined8 *param_1,longlong param_2,float *param_3,undefined4 *param_4,
             undefined4 *param_5,undefined4 *param_6)
{
  float fVar1;
  float fVar2;
  float fVar3;
  float fVar4;
  float fVar5;
  undefined4 uVar6;
  undefined4 uVar7;
  undefined4 uVar8;
  ulonglong *puVar9;
  code *pcVar10;
  undefined8 uVar11;
  undefined8 uVar12;
  undefined8 uVar13;
  undefined8 uVar14;
  undefined4 *puVar15;
  undefined4 *puVar16;
  float *pfVar17;
  float *pfVar18;
  longlong lVar19;
  undefined4 *puVar20;
  float *pfVar21;
  undefined4 *puVar22;
  undefined4 *puVar23;
  longlong lVar24;
  ulonglong uVar25;
  ulonglong uVar26;
  undefined4 *puVar27;
  undefined4 *puVar28;
  ulonglong uVar29;
  float *pfVar30;
  ulonglong uVar31;
  longlong lVar32;
  longlong lVar33;
  longlong lVar34;
  ulonglong uVar35;
  ulonglong uVar36;
  undefined4 *puVar37;
  undefined4 *puVar38;
  float fVar39;
  undefined1 auVar40 [16];
  undefined1 auVar41 [16];
  float *pfStack_b8;
  longlong lStack_b0;
  undefined4 *puStack_a8;
  undefined4 *puStack_a0;
  float *pfStack_98;
  undefined4 uStack_90;
  undefined4 uStack_8c;
  undefined4 *puStack_88;
  undefined4 *puStack_80;
  float *pfStack_78;
  undefined4 *puStack_70;
  longlong lStack_68;
  undefined8 uStack_60;
  uStack_60 = 0xfffffffffffffffe;
  if (param_4 < (undefined4 *)0x2) {
LAB_140c9456a:
    FUN_1428d9430(&UNK_142b2dbd8,0x23,&UNK_142b2dcc0);
LAB_140c94582:
    FUN_1428d9330(&UNK_142b2db40,0x22,&UNK_142b2dbc0);
  }
  else {
    uVar26 = (longlong)param_4 * 4 - 4;
    puVar9 = *(ulonglong **)(param_2 + 0x10);
    uVar29 = puVar9[4] & 0xfffffffffffffffc;
    puStack_70 = param_4;
    if ((uVar29 < *puVar9) || (uVar29 - *puVar9 < uVar26)) {
LAB_140c944f4:
      pfVar30 = (float *)FUN_1428d7780(param_2,4,uVar26);
      if (pfVar30 != (float *)0x0) goto LAB_140c939ee;
LAB_140c94565:
      FUN_1428d79d0();
      goto LAB_140c9456a;
    }
    pfVar30 = (float *)(uVar29 - uVar26);
    puVar9[4] = (ulonglong)pfVar30;
    if (pfVar30 == (float *)0x0) goto LAB_140c944f4;
LAB_140c939ee:
    lVar19 = (longlong)puStack_70 * 3;
    uVar29 = (longlong)puStack_70 - 1;
    *pfVar30 = SQRT((param_3[5] - param_3[2]) * (param_3[5] - param_3[2]) +
                    (param_3[4] - param_3[1]) * (param_3[4] - param_3[1]) +
                    (param_3[3] - *param_3) * (param_3[3] - *param_3));
    lStack_68 = param_2;
    if (uVar29 == 1) {
      puStack_80 = (undefined4 *)0x2;
      uVar31 = 8;
    }
    else {
      pfVar21 = param_3 + 3;
      pfVar18 = param_3 + 6;
      uVar35 = (longlong)puStack_70 + 0x3ffffffffffffffeU & 0x3fffffffffffffff;
      uVar31 = (longlong)puStack_70 - 3;
      uVar36 = uVar31;
      if (uVar35 < uVar31) {
        uVar36 = uVar35;
      }
      uVar25 = 1;
      pfVar17 = pfVar18;
      if (3 < uVar36) {
        if (uVar35 < uVar31) {
          uVar31 = uVar35;
        }
        if ((param_3 + uVar31 * 3 + 9 <= pfVar30 + 1) || (pfVar30 + uVar31 + 2 <= pfVar21)) {
          uVar36 = uVar36 + 1;
          uVar31 = 4;
          if ((uVar36 & 3) != 0) {
            uVar31 = (ulonglong)((uint)uVar36 & 3);
          }
          lVar32 = uVar36 - uVar31;
          pfVar17 = pfVar18 + lVar32 * 3;
          pfVar21 = pfVar21 + lVar32 * 3;
          uVar25 = (uVar36 - uVar31) + 1;
          lVar33 = 0;
          do {
            auVar40._0_4_ =
                 (pfVar18[2] - pfVar18[-1]) * (pfVar18[2] - pfVar18[-1]) +
                 (pfVar18[1] - pfVar18[-2]) * (pfVar18[1] - pfVar18[-2]) +
                 (*pfVar18 - pfVar18[-3]) * (*pfVar18 - pfVar18[-3]);
            auVar40._4_4_ =
                 (pfVar18[5] - pfVar18[2]) * (pfVar18[5] - pfVar18[2]) +
                 (pfVar18[4] - pfVar18[1]) * (pfVar18[4] - pfVar18[1]) +
                 (pfVar18[3] - *pfVar18) * (pfVar18[3] - *pfVar18);
            auVar40._8_4_ =
                 (pfVar18[8] - pfVar18[5]) * (pfVar18[8] - pfVar18[5]) +
                 (pfVar18[7] - pfVar18[4]) * (pfVar18[7] - pfVar18[4]) +
                 (pfVar18[6] - pfVar18[3]) * (pfVar18[6] - pfVar18[3]);
            auVar40._12_4_ =
                 (pfVar18[0xb] - pfVar18[8]) * (pfVar18[0xb] - pfVar18[8]) +
                 (pfVar18[10] - pfVar18[7]) * (pfVar18[10] - pfVar18[7]) +
                 (pfVar18[9] - pfVar18[6]) * (pfVar18[9] - pfVar18[6]);
            auVar40 = sqrtps(auVar40,auVar40);
            *(undefined1 (*) [16])(pfVar30 + lVar33 + 1) = auVar40;
            lVar33 = lVar33 + 4;
            pfVar18 = pfVar18 + 0xc;
          } while (lVar32 != lVar33);
        }
      }
      do {
        pfVar18 = pfVar17;
        if (pfVar18 == param_3 + lVar19) goto LAB_140c94582;
        pfVar30[uVar25] =
             SQRT((pfVar18[2] - pfVar21[2]) * (pfVar18[2] - pfVar21[2]) +
                  (pfVar18[1] - pfVar21[1]) * (pfVar18[1] - pfVar21[1]) +
                  (*pfVar18 - *pfVar21) * (*pfVar18 - *pfVar21));
        uVar25 = uVar25 + 1;
        pfVar17 = pfVar18 + 3;
        pfVar21 = pfVar18;
      } while (uVar29 != uVar25);
      if (uVar29 == 0x1fffffffffffffff) goto LAB_140c94565;
      uVar31 = (longlong)puStack_70 * 4;
      puStack_80 = puStack_70;
    }
    puVar9 = *(ulonglong **)(param_2 + 0x10);
    uVar36 = puVar9[4] & 0xfffffffffffffffc;
    pfStack_78 = param_3;
    if ((uVar36 < *puVar9) || (uVar36 - *puVar9 < uVar31)) {
LAB_140c9451f:
      puVar37 = (undefined4 *)FUN_1428d7780(param_2);
      if (puVar37 == (undefined4 *)0x0) goto LAB_140c94565;
    }
    else {
      puVar37 = (undefined4 *)(uVar36 - uVar31);
      puVar9[4] = (ulonglong)puVar37;
      if (puVar37 == (undefined4 *)0x0) goto LAB_140c9451f;
    }
    func_0x000142923c20(puVar37);
    uVar31 = 0;
    if (((ulonglong)puStack_80 & 0x3fffffffffffffff) != 0) {
      uVar31 = ((ulonglong)puStack_80 & 0x3fffffffffffffff) - 1;
    }
    if (uVar29 < uVar31) {
      uVar31 = uVar29;
    }
    if (uVar31 != 0) {
      uVar36 = (ulonglong)((uint)uVar31 & 3);
      if (uVar31 < 4) {
        fVar39 = 0.0;
        uVar25 = 0;
      }
      else {
        fVar39 = 0.0;
        uVar35 = 0;
        do {
          fVar1 = pfVar30[uVar35];
          puVar37[uVar35 + 1] = fVar39 + fVar1;
          fVar39 = fVar39 + fVar1 + pfVar30[uVar35 + 1];
          puVar37[uVar35 + 2] = fVar39;
          fVar39 = fVar39 + pfVar30[uVar35 + 2];
          puVar37[uVar35 + 3] = fVar39;
          fVar39 = fVar39 + pfVar30[uVar35 + 3];
          uVar25 = uVar35 + 4;
          puVar37[uVar35 + 4] = fVar39;
          uVar35 = uVar25;
        } while ((uVar31 & 0x3ffffffffffffffc) != uVar25);
      }
      if (uVar36 != 0) {
        uVar31 = 0;
        do {
          fVar39 = fVar39 + pfVar30[uVar25 + uVar31];
          puVar37[uVar25 + uVar31 + 1] = fVar39;
          uVar31 = uVar31 + 1;
        } while (uVar36 != uVar31);
      }
    }
    puStack_88 = puVar37;
    if (uVar29 >> 0x3d != 0) goto LAB_140c94565;
    puVar9 = *(ulonglong **)(lStack_68 + 0x10);
    uVar31 = puVar9[4] & 0xfffffffffffffffc;
    if ((uVar31 < *puVar9) || (uVar31 - *puVar9 < uVar26)) {
LAB_140c9453e:
      uVar31 = FUN_1428d7780(lStack_68,4,uVar26);
      if (uVar31 == 0) goto LAB_140c94565;
    }
    else {
      uVar31 = uVar31 - uVar26;
      puVar9[4] = uVar31;
      if (uVar31 == 0) goto LAB_140c9453e;
    }
    puVar16 = puStack_80;
    puVar37 = puStack_88;
    auVar40 = _DAT_142925930;
    uVar26 = (longlong)puStack_70 - 2U;
    if (uVar29 < (longlong)puStack_70 - 2U) {
      uVar26 = uVar29;
    }
    uVar36 = 0;
    pfVar21 = pfVar30;
    if ((3 < uVar26) && (0xf < uVar31 - (longlong)pfVar30)) {
      uVar26 = uVar26 + 1;
      uVar36 = 4;
      if ((uVar26 & 3) != 0) {
        uVar36 = (ulonglong)((uint)uVar26 & 3);
      }
      uVar36 = uVar26 - uVar36;
      pfVar21 = pfVar30 + uVar36;
      uVar26 = 0;
      do {
        auVar41 = divps(auVar40,*(undefined1 (*) [16])(pfVar30 + uVar26));
        *(undefined1 (*) [16])(uVar31 + uVar26 * 4) = auVar41;
        uVar26 = uVar26 + 4;
      } while (uVar36 != uVar26);
    }
    fVar39 = _DAT_142925904;
    do {
      if (pfVar21 == pfVar30 + uVar29) goto LAB_140c94582;
      fVar1 = *pfVar21;
      pfVar21 = pfVar21 + 1;
      *(float *)(uVar31 + uVar36 * 4) = fVar39 / fVar1;
      uVar36 = uVar36 + 1;
    } while (uVar29 != uVar36);
    pfStack_b8 = (float *)0x4;
    lStack_b0 = lStack_68;
    puStack_a8 = (undefined4 *)0x0;
    puStack_a0 = (undefined4 *)0x0;
    FUN_141940de0(&pfStack_b8,0,puStack_70,1);
    pfVar21 = pfStack_78;
    do {
      fVar39 = *pfVar21;
      if (puStack_a0 == puStack_a8) {
        FUN_141940de0(&pfStack_b8,puStack_a0,1,1);
      }
      puVar15 = puStack_a8;
      lVar32 = lStack_b0;
      pfVar21 = pfVar21 + 3;
      pfStack_b8[(longlong)puStack_a0] = fVar39;
      puVar38 = (undefined4 *)((longlong)puStack_a0 + 1);
      puStack_a0 = puVar38;
    } while (pfVar21 != param_3 + lVar19);
    uStack_90 = *param_5;
    uStack_8c = *param_6;
    pfStack_98 = pfStack_b8;
    puVar27 = puStack_88;
    lVar19 = FUN_140c94a10(lStack_68,puStack_88,puStack_80,pfVar30,uVar29,uVar31,uVar29,pfStack_b8,
                           puVar38,uStack_90,param_5[1],uStack_8c,param_6[1]);
    puVar28 = puStack_70;
    if (puVar38 <= puStack_70) {
      puVar28 = puVar38;
    }
    if (puVar38 != (undefined4 *)0x0) {
      if ((puVar28 < (undefined4 *)0x10) ||
         (pfStack_98 < pfStack_78 + (longlong)puVar28 * 3 + -1 &&
          pfStack_78 + 1 < pfStack_98 + (longlong)puVar28)) {
        puVar20 = (undefined4 *)0x0;
      }
      else {
        puVar20 = (undefined4 *)((ulonglong)puVar28 & 0x3ffffffffffffff8);
        pfVar21 = pfStack_78 + 0x16;
        puVar22 = (undefined4 *)0x0;
        do {
          fVar39 = pfVar21[-0xf];
          fVar1 = pfVar21[-0xc];
          fVar2 = *pfVar21;
          fVar3 = pfVar21[-3];
          fVar4 = pfVar21[-6];
          fVar5 = pfVar21[-9];
          *(ulonglong *)(pfStack_98 + (longlong)puVar22) = CONCAT44(pfVar21[-0x12],pfVar21[-0x15]);
          *(ulonglong *)(pfStack_98 + (longlong)puVar22 + 2) = CONCAT44(fVar1,fVar39);
          pfVar18 = pfStack_98 + (longlong)(puVar22 + 1);
          *pfVar18 = fVar5;
          pfVar18[1] = fVar4;
          pfVar18[2] = fVar3;
          pfVar18[3] = fVar2;
          puVar22 = puVar22 + 2;
          pfVar21 = pfVar21 + 0x18;
        } while (puVar20 != puVar22);
        if (puVar28 == puVar20) goto LAB_140c940ea;
      }
      puVar22 = puVar20;
      if (((ulonglong)puVar28 & 3) != 0) {
        lVar33 = 0;
        do {
          pfStack_98[(longlong)puVar22] = pfStack_78[(longlong)puVar20 * 3 + lVar33 + 1];
          puVar22 = (undefined4 *)((longlong)puVar22 + 1);
          lVar33 = lVar33 + 3;
        } while (((ulonglong)puVar28 & 3) * 3 != lVar33);
      }
      if ((ulonglong)((longlong)puVar20 - (longlong)puVar28) < 0xfffffffffffffffd) {
        pfVar21 = pfStack_78 + (longlong)puVar22 * 3 + 10;
        do {
          pfStack_98[(longlong)puVar22] = pfVar21[-9];
          pfStack_98[(longlong)puVar22 + 1] = pfVar21[-6];
          pfStack_98[(longlong)puVar22 + 2] = pfVar21[-3];
          pfStack_98[(longlong)puVar22 + 3] = *pfVar21;
          puVar22 = puVar22 + 1;
          pfVar21 = pfVar21 + 0xc;
        } while (puVar28 != puVar22);
      }
    }
LAB_140c940ea:
    puVar20 = puStack_88;
    lVar33 = FUN_140c94a10(lStack_68,puStack_88,puStack_80,pfVar30,uVar29,uVar31,uVar29,pfStack_98,
                           puVar38,uStack_90,param_5[2],uStack_8c,param_6[2]);
    if (puVar38 != (undefined4 *)0x0) {
      if ((puVar28 < (undefined4 *)0x10) ||
         (pfStack_98 < pfStack_78 + (longlong)puVar28 * 3 &&
          pfStack_78 + 2 < pfStack_98 + (longlong)puVar28)) {
        puVar22 = (undefined4 *)0x0;
      }
      else {
        puVar22 = (undefined4 *)((ulonglong)puVar28 & 0x3ffffffffffffff8);
        pfVar21 = pfStack_78 + 0x17;
        puVar23 = (undefined4 *)0x0;
        do {
          fVar39 = pfVar21[-0xf];
          fVar1 = pfVar21[-0xc];
          fVar2 = *pfVar21;
          fVar3 = pfVar21[-3];
          fVar4 = pfVar21[-6];
          fVar5 = pfVar21[-9];
          *(ulonglong *)(pfStack_98 + (longlong)puVar23) = CONCAT44(pfVar21[-0x12],pfVar21[-0x15]);
          *(ulonglong *)(pfStack_98 + (longlong)puVar23 + 2) = CONCAT44(fVar1,fVar39);
          pfVar18 = pfStack_98 + (longlong)(puVar23 + 1);
          *pfVar18 = fVar5;
          pfVar18[1] = fVar4;
          pfVar18[2] = fVar3;
          pfVar18[3] = fVar2;
          puVar23 = puVar23 + 2;
          pfVar21 = pfVar21 + 0x18;
        } while (puVar22 != puVar23);
        if (puVar28 == puVar22) goto LAB_140c942ba;
      }
      puVar23 = puVar22;
      if (((ulonglong)puVar28 & 3) != 0) {
        lVar34 = 0;
        do {
          pfStack_98[(longlong)puVar23] = pfStack_78[(longlong)puVar22 * 3 + lVar34 + 2];
          puVar23 = (undefined4 *)((longlong)puVar23 + 1);
          lVar34 = lVar34 + 3;
        } while (((ulonglong)puVar28 & 3) * 3 != lVar34);
      }
      if ((ulonglong)((longlong)puVar22 - (longlong)puVar28) < 0xfffffffffffffffd) {
        pfVar21 = pfStack_78 + (longlong)puVar23 * 3 + 0xb;
        do {
          pfStack_98[(longlong)puVar23] = pfVar21[-9];
          pfStack_98[(longlong)puVar23 + 1] = pfVar21[-6];
          pfStack_98[(longlong)puVar23 + 2] = pfVar21[-3];
          pfStack_98[(longlong)puVar23 + 3] = *pfVar21;
          puVar23 = puVar23 + 1;
          pfVar21 = pfVar21 + 0xc;
        } while (puVar28 != puVar23);
      }
    }
LAB_140c942ba:
    puVar28 = puStack_88;
    puStack_70 = puVar20;
    lVar34 = FUN_140c94a10(lStack_68,puStack_88,puStack_80,pfVar30,uVar29,uVar31,uVar29,pfStack_98,
                           puVar38,uStack_90,param_5[3],uStack_8c,param_6[3]);
    if (puStack_70 < puVar27) {
      puVar27 = puStack_70;
    }
    if (puVar28 < puVar27) {
      puVar27 = puVar28;
    }
    if ((ulonglong)puVar27 >> 0x39 == 0) {
      puVar9 = *(ulonglong **)(lStack_68 + 0x10);
      uVar26 = puVar9[4] & 0xfffffffffffffffc;
      if ((uVar26 < *puVar9) || (uVar26 - *puVar9 < (ulonglong)((longlong)puVar27 * 0x40))) {
LAB_140c94384:
        puVar38 = (undefined4 *)FUN_1428d7780(lStack_68,4);
        if (puVar38 == (undefined4 *)0x0) {
          FUN_1428d79d0();
          goto LAB_140c945b4;
        }
      }
      else {
        puVar38 = (undefined4 *)(uVar26 + (longlong)puVar27 * -0x40);
        puVar9[4] = (ulonglong)puVar38;
        if (puVar38 == (undefined4 *)0x0) goto LAB_140c94384;
      }
      if (puVar27 != (undefined4 *)0x0) {
        lVar24 = 0;
        puVar28 = puVar27;
        puVar20 = puVar38;
        do {
          uVar6 = *(undefined4 *)(lVar19 + 0x10 + lVar24);
          puStack_a8 = (undefined4 *)CONCAT44(puStack_a8._4_4_,uVar6);
          pfStack_b8 = *(float **)(lVar19 + lVar24);
          lStack_b0 = ((undefined8 *)(lVar19 + lVar24))[1];
          uVar11 = *(undefined8 *)(lVar33 + lVar24);
          uVar12 = ((undefined8 *)(lVar33 + lVar24))[1];
          uVar7 = *(undefined4 *)(lVar33 + 0x10 + lVar24);
          uVar8 = *(undefined4 *)(lVar34 + 0x10 + lVar24);
          uVar13 = *(undefined8 *)(lVar34 + lVar24);
          uVar14 = ((undefined8 *)(lVar34 + lVar24))[1];
          *puVar20 = *(undefined4 *)(lVar19 + 0x10 + lVar24);
          puVar20[5] = uVar6;
          *(float **)(puVar20 + 1) = pfStack_b8;
          *(longlong *)(puVar20 + 3) = lStack_b0;
          *(undefined8 *)(puVar20 + 6) = uVar11;
          *(undefined8 *)(puVar20 + 8) = uVar12;
          puVar20[10] = uVar7;
          *(undefined8 *)(puVar20 + 0xb) = uVar13;
          *(undefined8 *)(puVar20 + 0xd) = uVar14;
          puVar20[0xf] = uVar8;
          lVar24 = lVar24 + 0x14;
          puVar20 = puVar20 + 0x10;
          puVar28 = (undefined4 *)((longlong)puVar28 + -1);
        } while (puVar28 != (undefined4 *)0x0);
      }
      if (puVar37 + (longlong)puVar16 + -1 != (undefined4 *)0x0) {
        uVar6 = *puStack_88;
        uVar7 = puVar37[(longlong)puVar16 + -1];
        *param_1 = 0x8000000000000000;
        param_1[1] = puVar38;
        param_1[2] = puVar27;
        *(undefined4 *)(param_1 + 3) = uVar6;
        *(undefined4 *)((longlong)param_1 + 0x1c) = uVar7;
        if (puVar15 != (undefined4 *)0x0) {
          pfVar30 = *(float **)(*(longlong *)(lVar32 + 0x10) + 0x20);
          if (pfVar30 == pfStack_98) {
            *(float **)(*(longlong *)(lVar32 + 0x10) + 0x20) = pfVar30 + (longlong)puVar15;
          }
        }
        return param_1;
      }
      FUN_1428d9310(&UNK_142b2dcd8);
      goto LAB_140c945b4;
    }
  }
  FUN_1428d79d0();
LAB_140c945b4:
                    /* WARNING: Does not return */
  pcVar10 = (code *)invalidInstructionException();
  (*pcVar10)();
}
