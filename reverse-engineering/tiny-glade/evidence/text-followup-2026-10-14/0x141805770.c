
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void FUN_141805770(longlong *param_1,undefined8 *param_2)

{
  longlong lVar1;
  uint *puVar2;
  byte bVar3;
  ushort uVar4;
  undefined4 uVar5;
  longlong lVar6;
  byte *pbVar7;
  ulonglong *puVar8;
  code *pcVar9;
  undefined1 auVar10 [14];
  undefined1 auVar11 [14];
  undefined1 auVar12 [14];
  undefined8 uVar13;
  undefined8 uVar14;
  ulonglong uVar15;
  undefined1 auVar16 [14];
  uint uVar17;
  uint uVar18;
  uint uVar19;
  uint uVar20;
  ulonglong uVar21;
  byte *pbVar22;
  uint uVar23;
  ulonglong uVar24;
  byte *pbVar25;
  ulonglong uVar26;
  longlong lVar27;
  longlong lVar28;
  ulonglong uVar29;
  ulonglong uVar30;
  undefined1 uVar35;
  undefined1 auVar31 [16];
  undefined1 auVar37 [16];
  undefined8 auStack_e0 [6];
  ulonglong uStack_b0;
  longlong lStack_a8;
  longlong lStack_a0;
  ulonglong uStack_98;
  longlong *plStack_90;
  ulonglong uStack_88;
  ulonglong uStack_80;
  longlong lStack_78;
  undefined8 uStack_70;
  undefined1 auVar32 [16];
  undefined1 auVar33 [16];
  undefined1 auVar34 [16];
  undefined2 uVar36;
  undefined1 auVar38 [16];
  undefined1 auVar39 [16];
  undefined1 auVar40 [16];
  
  uVar20 = _UNK_142ced4ec;
  uVar19 = _UNK_142ced4e8;
  uVar18 = _UNK_142ced4e4;
  uVar17 = _DAT_142ced4e0;
  uStack_70 = 0xfffffffffffffffe;
  lVar28 = *param_1;
  plStack_90 = (longlong *)*param_2;
  lStack_78 = param_2[1];
  if (lVar28 != param_1[1]) {
    lStack_a8 = param_1[2];
    lVar6 = param_2[2];
    uVar29 = (ulonglong)(param_1[1] - lVar28) >> 6;
    uVar30 = 0;
    lStack_a0 = lVar28;
    uStack_98 = uVar29;
    do {
      lVar1 = lVar28 + uVar30 * 0x40;
      if (*(char *)(lVar28 + 0x34 + uVar30 * 0x40) == '\0') {
        uVar29 = *(ulonglong *)(lVar1 + 0x10);
        uVar24 = (ulonglong)*(ushort *)(lVar1 + 0x3c);
        uVar4 = *(ushort *)(lVar1 + 0x3e);
        uStack_88 = uVar4 * uVar24;
        uStack_80 = uVar29;
        if (uVar29 != uStack_88) {
          auStack_e0[0] = 0;
          FUN_1428d9596(0,&uStack_80,&uStack_88,auStack_e0,&UNK_142ced748);
          goto LAB_141805a92;
        }
        pbVar7 = *(byte **)(lVar1 + 8);
        uVar26 = uVar29 * 4;
        puVar8 = *(ulonglong **)(lStack_a8 + 0x10);
        uVar21 = puVar8[4] & 0xfffffffffffffffc;
        uStack_b0 = uVar24;
        if ((uVar21 < *puVar8) || (uVar21 - *puVar8 <= uVar26 && uVar26 - (uVar21 - *puVar8) != 0))
        {
LAB_141805900:
          pbVar22 = (byte *)FUN_1428d7780(lStack_a8,4,uVar26);
          if (pbVar22 == (byte *)0x0) {
            FUN_1428d79d0();
LAB_141805a92:
                    /* WARNING: Does not return */
            pcVar9 = (code *)invalidInstructionException();
            (*pcVar9)();
          }
        }
        else {
          pbVar22 = (byte *)(uVar21 + uVar29 * -4);
          puVar8[4] = (ulonglong)pbVar22;
          if (pbVar22 == (byte *)0x0) goto LAB_141805900;
        }
        if (uVar29 == 0) {
          uVar26 = 0;
        }
        else {
          uVar24 = uVar29 - 1;
          uVar21 = uVar24;
          if (uVar29 < uVar24) {
            uVar21 = uVar29;
          }
          if (uVar21 < 8) {
LAB_141805957:
            uVar21 = 0;
            pbVar25 = pbVar7;
          }
          else {
            if (uVar29 < uVar24) {
              uVar24 = uVar29;
            }
            if ((pbVar22 < pbVar7 + uVar24 + 1) && (pbVar7 < pbVar22 + uVar24 * 4 + 4))
            goto LAB_141805957;
            uVar21 = uVar21 + 1;
            uVar24 = (ulonglong)((uint)uVar21 & 7);
            if ((uVar21 & 7) == 0) {
              uVar24 = 8;
            }
            uVar21 = uVar21 - uVar24;
            pbVar25 = pbVar7 + uVar21;
            uVar24 = 0;
            do {
              uVar5 = *(undefined4 *)(pbVar7 + uVar24);
              uVar35 = (undefined1)((uint)uVar5 >> 0x18);
              uVar36 = CONCAT11(uVar35,uVar35);
              uVar35 = (undefined1)((uint)uVar5 >> 0x10);
              uVar13 = CONCAT35(CONCAT21(uVar36,uVar35),CONCAT14(uVar35,uVar5));
              uVar35 = (undefined1)((uint)uVar5 >> 8);
              uVar14 = CONCAT53(CONCAT41((int)((ulonglong)uVar13 >> 0x20),uVar35),
                                CONCAT12(uVar35,(short)uVar5));
              uVar15 = CONCAT62((int6)((ulonglong)uVar14 >> 0x10),(short)uVar5) & 0xffffffffffff00ff
              ;
              auVar34._8_4_ = 0;
              auVar34._0_8_ = uVar15;
              auVar34._12_2_ = uVar36;
              auVar34._14_2_ = uVar36;
              uVar36 = (undefined2)((ulonglong)uVar13 >> 0x20);
              auVar33._12_4_ = auVar34._12_4_;
              auVar33._8_2_ = 0;
              auVar33._0_8_ = uVar15;
              auVar33._10_2_ = uVar36;
              auVar32._10_6_ = auVar33._10_6_;
              auVar32._8_2_ = uVar36;
              auVar32._0_8_ = uVar15;
              uVar36 = (undefined2)((ulonglong)uVar14 >> 0x10);
              auVar31._8_8_ = auVar32._8_8_;
              auVar31._6_2_ = uVar36;
              auVar31._0_4_ = (int)uVar15;
              auVar31._4_2_ = uVar36;
              uVar5 = *(undefined4 *)(pbVar7 + uVar24 + 4);
              uVar35 = (undefined1)((uint)uVar5 >> 0x18);
              uVar36 = CONCAT11(uVar35,uVar35);
              uVar35 = (undefined1)((uint)uVar5 >> 0x10);
              uVar13 = CONCAT35(CONCAT21(uVar36,uVar35),CONCAT14(uVar35,uVar5));
              uVar35 = (undefined1)((uint)uVar5 >> 8);
              uVar14 = CONCAT53(CONCAT41((int)((ulonglong)uVar13 >> 0x20),uVar35),
                                CONCAT12(uVar35,(short)uVar5));
              uVar15 = CONCAT62((int6)((ulonglong)uVar14 >> 0x10),(short)uVar5) & 0xffffffffffff00ff
              ;
              auVar40._8_4_ = 0;
              auVar40._0_8_ = uVar15;
              auVar40._12_2_ = uVar36;
              auVar40._14_2_ = uVar36;
              uVar36 = (undefined2)((ulonglong)uVar13 >> 0x20);
              auVar39._12_4_ = auVar40._12_4_;
              auVar39._8_2_ = 0;
              auVar39._0_8_ = uVar15;
              auVar39._10_2_ = uVar36;
              auVar38._10_6_ = auVar39._10_6_;
              auVar38._8_2_ = uVar36;
              auVar38._0_8_ = uVar15;
              uVar36 = (undefined2)((ulonglong)uVar14 >> 0x10);
              auVar37._8_8_ = auVar38._8_8_;
              auVar37._6_2_ = uVar36;
              auVar37._0_4_ = (int)uVar15;
              auVar37._4_2_ = uVar36;
              puVar2 = (uint *)(pbVar22 + uVar24 * 4);
              *puVar2 = auVar31._0_4_ << 0x18 | uVar17;
              puVar2[1] = auVar31._4_4_ << 0x18 | uVar18;
              puVar2[2] = auVar32._8_4_ << 0x18 | uVar19;
              puVar2[3] = auVar33._12_4_ << 0x18 | uVar20;
              puVar2 = (uint *)(pbVar22 + uVar24 * 4 + 0x10);
              *puVar2 = auVar37._0_4_ << 0x18 | uVar17;
              puVar2[1] = auVar37._4_4_ << 0x18 | uVar18;
              puVar2[2] = auVar38._8_4_ << 0x18 | uVar19;
              puVar2[3] = auVar39._12_4_ << 0x18 | uVar20;
              uVar24 = uVar24 + 8;
            } while (uVar21 != uVar24);
          }
          do {
            if (pbVar25 == pbVar7 + uVar29) {
              FUN_1428d9330(&UNK_142ced5d8,0x22,&UNK_142ced658);
              goto LAB_141805a92;
            }
            bVar3 = *pbVar25;
            pbVar25 = pbVar25 + 1;
            *(uint *)(pbVar22 + uVar21 * 4) = (uint)bVar3 << 0x18 | 0xffffff;
            uVar21 = uVar21 + 1;
          } while (uVar29 != uVar21);
        }
        uVar23 = (uint)uStack_b0;
        lVar28 = lStack_a0;
        uVar29 = uStack_98;
      }
      else {
        uVar26 = *(ulonglong *)(lVar1 + 0x10);
        uVar23 = (uint)*(ushort *)(lVar1 + 0x3c);
        uVar4 = *(ushort *)(lVar1 + 0x3e);
        uStack_88 = (ulonglong)*(ushort *)(lVar1 + 0x3c) * (ulonglong)uVar4 * 4;
        if (uVar26 != uStack_88) {
          auStack_e0[0] = 0;
          uStack_80 = uVar26;
          FUN_1428d9596(0,&uStack_80,&uStack_88,auStack_e0,&UNK_142ced760);
          goto LAB_141805a92;
        }
        pbVar22 = *(byte **)(lVar1 + 8);
      }
      lVar27 = lStack_78 * 0x30;
      *(byte **)(lVar6 + lVar27) = pbVar22;
      *(ulonglong *)(lVar6 + 8 + lVar27) = uVar26;
      uVar24 = *(ulonglong *)(lVar1 + 0x38);
      auVar10._8_4_ = 0;
      auVar10._0_8_ = uVar24;
      auVar10._12_2_ = (short)(uVar24 >> 0x30);
      auVar11._8_2_ = (short)(uVar24 >> 0x20);
      auVar11._0_8_ = uVar24;
      auVar11._10_4_ = auVar10._10_4_;
      auVar16._6_8_ = 0;
      auVar16._0_6_ = auVar11._8_6_;
      auVar12._6_8_ = SUB148(auVar16 << 0x40,6);
      auVar12._4_2_ = (short)(uVar24 >> 0x10);
      auVar12._2_2_ = 0;
      auVar12._0_2_ = (ushort)uVar24;
      *(longlong *)(lVar6 + 0x10 + lVar27) = auVar12._0_8_;
      *(undefined4 *)(lVar6 + 0x18 + lVar27) = 0;
      *(uint *)(lVar6 + 0x1c + lVar27) = uVar23 & 0xffff;
      *(uint *)(lVar6 + 0x20 + lVar27) = (uint)uVar4;
      *(undefined8 *)(lVar6 + 0x24 + lVar27) = 1;
      *(undefined4 *)(lVar6 + 0x2c + lVar27) = 0;
      lStack_78 = lStack_78 + 1;
      uVar30 = uVar30 + 1;
    } while (uVar30 != uVar29);
  }
  *plStack_90 = lStack_78;
  return;
}

