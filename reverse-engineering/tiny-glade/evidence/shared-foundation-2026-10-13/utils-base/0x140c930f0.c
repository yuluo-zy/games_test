/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */
undefined8 *
FUN_140c930f0(undefined8 *param_1,longlong param_2,undefined8 *param_3,ulonglong param_4)
{
  ulonglong uVar1;
  undefined4 uVar2;
  undefined4 uVar3;
  undefined4 uVar4;
  undefined4 uVar5;
  undefined4 uVar6;
  undefined4 uVar7;
  undefined4 uVar8;
  ulonglong *puVar9;
  code *pcVar10;
  undefined8 uVar11;
  undefined8 uVar12;
  undefined1 auVar13 [16];
  longlong lVar14;
  undefined4 *puVar15;
  longlong lVar16;
  ulonglong uVar17;
  longlong lVar18;
  undefined4 *puVar19;
  undefined8 *puVar20;
  ulonglong uVar21;
  float *pfVar22;
  ulonglong uVar23;
  longlong lVar24;
  undefined8 *puVar25;
  undefined8 *puVar26;
  undefined4 *puVar27;
  ulonglong uVar28;
  undefined4 *puVar29;
  ulonglong uVar30;
  ulonglong uVar31;
  undefined4 *puVar32;
  ulonglong uVar33;
  float *pfVar34;
  float fVar35;
  float fVar36;
  undefined1 auVar37 [16];
  undefined4 uStack_e8;
  undefined4 uStack_e4;
  undefined4 uStack_e0;
  undefined4 uStack_dc;
  undefined8 *puStack_a8;
  longlong lStack_a0;
  ulonglong uStack_98;
  ulonglong uStack_90;
  ulonglong uStack_80;
  longlong lStack_78;
  undefined8 *puStack_70;
  undefined4 *puStack_68;
  undefined8 uStack_60;
  uStack_60 = 0xfffffffffffffffe;
  if (param_4 < 3) {
LAB_140c93851:
    FUN_1428d9430(&UNK_142b2dc50,0x23,&UNK_142b2dc78);
LAB_140c93869:
    FUN_1428d9330(&UNK_142b2db40,0x22,&UNK_142b2dbc0);
  }
  else {
    uVar1 = param_4 * 4;
    puVar9 = *(ulonglong **)(param_2 + 0x10);
    uVar33 = puVar9[4] & 0xfffffffffffffffc;
    if ((uVar33 < *puVar9) || (uVar33 - *puVar9 <= uVar1 && uVar1 - (uVar33 - *puVar9) != 0)) {
LAB_140c937ef:
      pfVar34 = (float *)FUN_1428d7780(param_2,4,uVar1);
      if (pfVar34 != (float *)0x0) goto LAB_140c93169;
LAB_140c9384c:
      FUN_1428d79d0();
      goto LAB_140c93851;
    }
    pfVar34 = (float *)(uVar33 + param_4 * -4);
    puVar9[4] = (ulonglong)pfVar34;
    if (pfVar34 == (float *)0x0) goto LAB_140c937ef;
LAB_140c93169:
    puVar25 = param_3 + 2;
    fVar35 = (float)param_3[1] - (float)*param_3;
    fVar36 = (float)((ulonglong)param_3[1] >> 0x20) - (float)((ulonglong)*param_3 >> 0x20);
    *pfVar34 = SQRT(fVar36 * fVar36 + fVar35 * fVar35);
    uVar33 = 1;
    puVar20 = param_3 + 1;
    do {
      if (param_4 == uVar33) goto LAB_140c93869;
      puVar26 = puVar25;
      if (puVar25 == param_3 + param_4) {
        puVar26 = param_3;
      }
      puVar25 = puVar26 + 1;
      fVar35 = (float)*puVar26 - (float)*puVar20;
      fVar36 = (float)((ulonglong)*puVar26 >> 0x20) - (float)((ulonglong)*puVar20 >> 0x20);
      pfVar34[uVar33] = SQRT(fVar36 * fVar36 + fVar35 * fVar35);
      uVar33 = uVar33 + 1;
      puVar20 = puVar26;
    } while (param_4 != uVar33);
    uStack_80 = param_4 + 1;
    if (uStack_80 >> 0x3d != 0) goto LAB_140c9384c;
    uVar33 = param_4 * 4 + 4;
    puVar9 = *(ulonglong **)(param_2 + 0x10);
    uVar21 = puVar9[4] & 0xfffffffffffffffc;
    if ((uVar21 < *puVar9) || (uVar21 - *puVar9 < uVar33)) {
LAB_140c9380d:
      puStack_68 = (undefined4 *)FUN_1428d7780(param_2);
      if (puStack_68 == (undefined4 *)0x0) goto LAB_140c9384c;
    }
    else {
      puStack_68 = (undefined4 *)(uVar21 - uVar33);
      puVar9[4] = (ulonglong)puStack_68;
      if (puStack_68 == (undefined4 *)0x0) goto LAB_140c9380d;
    }
    lStack_78 = param_2;
    if (uStack_80 == 0) goto LAB_140c9384c;
    uVar33 = 0;
    func_0x000142923c20(puStack_68);
    uVar21 = (ulonglong)((uint)param_4 & 3);
    if (param_4 == 3) {
      fVar35 = 0.0;
    }
    else {
      fVar35 = 0.0;
      uVar30 = 0;
      do {
        fVar36 = pfVar34[uVar30];
        puStack_68[uVar30 + 1] = fVar35 + fVar36;
        fVar35 = fVar35 + fVar36 + pfVar34[uVar30 + 1];
        puStack_68[uVar30 + 2] = fVar35;
        fVar35 = fVar35 + pfVar34[uVar30 + 2];
        puStack_68[uVar30 + 3] = fVar35;
        fVar35 = fVar35 + pfVar34[uVar30 + 3];
        uVar33 = uVar30 + 4;
        puStack_68[uVar30 + 4] = fVar35;
        uVar30 = uVar33;
      } while ((param_4 & 0xfffffffffffffffc) != uVar33);
    }
    if (uVar21 != 0) {
      uVar30 = 0;
      do {
        fVar35 = fVar35 + pfVar34[uVar33 + uVar30];
        puStack_68[uVar33 + uVar30 + 1] = fVar35;
        uVar30 = uVar30 + 1;
      } while (uVar21 != uVar30);
    }
    if (param_4 >> 0x3d != 0) goto LAB_140c9384c;
    puVar9 = *(ulonglong **)(lStack_78 + 0x10);
    uVar33 = puVar9[4] & 0xfffffffffffffffc;
    if ((uVar33 < *puVar9) || (uVar33 - *puVar9 <= uVar1 && uVar1 - (uVar33 - *puVar9) != 0)) {
LAB_140c9382c:
      uVar33 = FUN_1428d7780(lStack_78,4,uVar1);
      if (uVar33 == 0) goto LAB_140c9384c;
    }
    else {
      uVar33 = uVar33 + param_4 * -4;
      puVar9[4] = uVar33;
      if (uVar33 == 0) goto LAB_140c9382c;
    }
    puVar15 = puStack_68;
    uVar1 = uStack_80;
    auVar13 = _DAT_142925930;
    uVar21 = param_4 - 1;
    if (param_4 < param_4 - 1) {
      uVar21 = param_4;
    }
    uVar30 = 0;
    pfVar22 = pfVar34;
    if ((3 < uVar21) && (0xf < uVar33 - (longlong)pfVar34)) {
      uVar21 = uVar21 + 1;
      uVar30 = 4;
      if ((uVar21 & 3) != 0) {
        uVar30 = (ulonglong)((uint)uVar21 & 3);
      }
      uVar30 = uVar21 - uVar30;
      pfVar22 = pfVar34 + uVar30;
      uVar21 = 0;
      do {
        auVar37 = divps(auVar13,*(undefined1 (*) [16])(pfVar34 + uVar21));
        *(undefined1 (*) [16])(uVar33 + uVar21 * 4) = auVar37;
        uVar21 = uVar21 + 4;
      } while (uVar30 != uVar21);
    }
    fVar35 = _DAT_142925904;
    do {
      if (pfVar22 == pfVar34 + param_4) goto LAB_140c93869;
      fVar36 = *pfVar22;
      pfVar22 = pfVar22 + 1;
      *(float *)(uVar33 + uVar30 * 4) = fVar35 / fVar36;
      uVar30 = uVar30 + 1;
    } while (param_4 != uVar30);
    puStack_a8 = (undefined8 *)0x4;
    lStack_a0 = lStack_78;
    uStack_98 = 0;
    uStack_90 = 0;
    FUN_141940de0(&puStack_a8,0,param_4,1);
    puVar25 = param_3;
    do {
      uVar2 = *(undefined4 *)puVar25;
      if (uStack_90 == uStack_98) {
        FUN_141940de0(&puStack_a8,uStack_90,1,1);
      }
      uVar21 = uStack_98;
      lVar14 = lStack_a0;
      puVar25 = puVar25 + 1;
      *(undefined4 *)((longlong)puStack_a8 + uStack_90 * 4) = uVar2;
      uVar30 = uStack_90 + 1;
      uStack_90 = uVar30;
    } while (puVar25 != param_3 + param_4);
    puStack_70 = puStack_a8;
    puVar27 = puStack_68;
    lVar16 = FUN_140c95200(lStack_78,puStack_68,uStack_80,pfVar34,param_4,uVar33,param_4,puStack_a8,
                           uVar30);
    uVar17 = uVar30;
    if (param_4 < uVar30) {
      uVar17 = param_4;
    }
    if (uVar30 != 0) {
      if ((uVar17 < 0x10) ||
         (puStack_70 < param_3 + uVar17 &&
          (undefined4 *)((longlong)param_3 + 4U) < (undefined4 *)((longlong)puStack_70 + uVar17 * 4)
         )) {
        uVar23 = 0;
      }
      else {
        uVar23 = uVar17 & 0x1ffffffffffffff8;
        uVar28 = 0;
        do {
          uVar2 = *(undefined4 *)((longlong)param_3 + uVar28 * 8 + 0xc);
          uVar8 = *(undefined4 *)((longlong)param_3 + uVar28 * 8 + 0x14);
          uVar3 = *(undefined4 *)((longlong)param_3 + uVar28 * 8 + 0x1c);
          uVar4 = *(undefined4 *)((longlong)param_3 + uVar28 * 8 + 0x3c);
          uVar5 = *(undefined4 *)((longlong)param_3 + uVar28 * 8 + 0x34);
          uVar6 = *(undefined4 *)((longlong)param_3 + uVar28 * 8 + 0x2c);
          uVar7 = *(undefined4 *)((longlong)param_3 + uVar28 * 8 + 0x24);
          puVar19 = (undefined4 *)((longlong)puStack_70 + uVar28 * 4);
          *puVar19 = *(undefined4 *)((longlong)param_3 + uVar28 * 8 + 4);
          puVar19[1] = uVar2;
          puVar19[2] = uVar8;
          puVar19[3] = uVar3;
          puVar19 = (undefined4 *)((longlong)puStack_70 + uVar28 * 4 + 0x10);
          *puVar19 = uVar7;
          puVar19[1] = uVar6;
          puVar19[2] = uVar5;
          puVar19[3] = uVar4;
          uVar28 = uVar28 + 8;
        } while (uVar23 != uVar28);
        if (uVar17 == uVar23) goto LAB_140c93647;
      }
      uVar28 = uVar23;
      for (uVar31 = uVar17 & 3; uVar31 != 0; uVar31 = uVar31 - 1) {
        *(undefined4 *)((longlong)puStack_70 + uVar28 * 4) =
             *(undefined4 *)((longlong)param_3 + uVar28 * 8 + 4);
        uVar28 = uVar28 + 1;
      }
      if (uVar23 - uVar17 < 0xfffffffffffffffd) {
        do {
          *(undefined4 *)((longlong)puStack_70 + uVar28 * 4) =
               *(undefined4 *)((longlong)param_3 + uVar28 * 8 + 4);
          *(undefined4 *)((longlong)puStack_70 + uVar28 * 4 + 4) =
               *(undefined4 *)((longlong)param_3 + uVar28 * 8 + 0xc);
          *(undefined4 *)((longlong)puStack_70 + uVar28 * 4 + 8) =
               *(undefined4 *)((longlong)param_3 + uVar28 * 8 + 0x14);
          *(undefined4 *)((longlong)puStack_70 + uVar28 * 4 + 0xc) =
               *(undefined4 *)((longlong)param_3 + uVar28 * 8 + 0x1c);
          uVar28 = uVar28 + 4;
        } while (uVar17 != uVar28);
      }
    }
LAB_140c93647:
    puVar19 = puStack_68;
    lVar18 = FUN_140c95200(lStack_78,puStack_68,uStack_80,pfVar34,param_4,uVar33,param_4,puStack_70,
                           uVar30);
    if (puVar19 < puVar27) {
      puVar27 = puVar19;
    }
    if (puVar27 < (undefined4 *)0x2e8ba2e8ba2e8bb) {
      puVar9 = *(ulonglong **)(lStack_78 + 0x10);
      uVar33 = puVar9[4] & 0xfffffffffffffffc;
      if ((uVar33 < *puVar9) ||
         (uVar33 - *puVar9 <= (ulonglong)((longlong)puVar27 * 0x2c) &&
          (longlong)puVar27 * 0x2c - (uVar33 - *puVar9) != 0)) {
LAB_140c936c8:
        puVar19 = (undefined4 *)FUN_1428d7780(lStack_78,4);
        if (puVar19 == (undefined4 *)0x0) {
          FUN_1428d79d0();
          goto LAB_140c938ad;
        }
      }
      else {
        puVar19 = (undefined4 *)(uVar33 + (longlong)puVar27 * -0x2c);
        puVar9[4] = (ulonglong)puVar19;
        if (puVar19 == (undefined4 *)0x0) goto LAB_140c936c8;
      }
      if (puVar27 != (undefined4 *)0x0) {
        lVar24 = 0;
        puVar29 = puVar27;
        puVar32 = puVar19;
        do {
          uVar2 = *(undefined4 *)(lVar16 + 0x10 + lVar24);
          uStack_98 = CONCAT44(uStack_98._4_4_,uVar2);
          puStack_a8 = *(undefined8 **)(lVar16 + lVar24);
          lStack_a0 = ((undefined8 *)(lVar16 + lVar24))[1];
          uVar8 = *(undefined4 *)(lVar18 + 0x10 + lVar24);
          uVar11 = *(undefined8 *)(lVar18 + lVar24);
          uVar12 = ((undefined8 *)(lVar18 + lVar24))[1];
          *puVar32 = *(undefined4 *)(lVar16 + 0x10 + lVar24);
          *(undefined8 **)(puVar32 + 1) = puStack_a8;
          *(longlong *)(puVar32 + 3) = lStack_a0;
          puVar32[5] = uVar2;
          uStack_e8 = (undefined4)uVar11;
          uStack_e4 = (undefined4)((ulonglong)uVar11 >> 0x20);
          uStack_e0 = (undefined4)uVar12;
          uStack_dc = (undefined4)((ulonglong)uVar12 >> 0x20);
          puVar32[6] = uStack_e8;
          puVar32[7] = uStack_e4;
          puVar32[8] = uStack_e0;
          puVar32[9] = uStack_dc;
          puVar32[10] = uVar8;
          lVar24 = lVar24 + 0x14;
          puVar32 = puVar32 + 0xb;
          puVar29 = (undefined4 *)((longlong)puVar29 + -1);
        } while (puVar29 != (undefined4 *)0x0);
      }
      if (uStack_80 == 0) {
        FUN_1428d9518(0,0,&UNK_142b2dc90);
      }
      else {
        if (puVar15 + (uVar1 - 1) != (undefined4 *)0x0) {
          uVar2 = *puStack_68;
          uVar8 = puVar15[uVar1 - 1];
          *param_1 = puVar19;
          param_1[1] = puVar27;
          *(undefined4 *)(param_1 + 2) = uVar2;
          *(undefined4 *)((longlong)param_1 + 0x14) = uVar8;
          *(undefined1 *)(param_1 + 3) = 1;
          if (uVar21 != 0) {
            puVar25 = *(undefined8 **)(*(longlong *)(lVar14 + 0x10) + 0x20);
            if (puVar25 == puStack_70) {
              *(undefined4 **)(*(longlong *)(lVar14 + 0x10) + 0x20) =
                   (undefined4 *)((longlong)puVar25 + uVar21 * 4);
            }
          }
          return param_1;
        }
        FUN_1428d9310(&UNK_142b2dca8);
      }
      goto LAB_140c938ad;
    }
  }
  FUN_1428d79d0();
LAB_140c938ad:
                    /* WARNING: Does not return */
  pcVar10 = (code *)invalidInstructionException();
  (*pcVar10)();
}
