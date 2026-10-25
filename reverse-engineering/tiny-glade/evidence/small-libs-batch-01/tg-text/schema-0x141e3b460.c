
/* WARNING: Type propagation algorithm not settling */
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

undefined8 * FUN_141e3b460(undefined8 *param_1,byte *param_2)

{
  int iVar1;
  undefined5 uVar2;
  byte *pbVar3;
  byte *pbVar4;
  undefined4 uVar5;
  char cVar6;
  longlong lVar7;
  byte bVar8;
  byte *pbVar9;
  byte *pbVar10;
  undefined1 uVar11;
  byte *pbVar12;
  undefined8 uVar13;
  ulonglong uVar14;
  byte *unaff_R14;
  byte *pbVar15;
  byte *pbVar16;
  byte *unaff_XMM8_Qa;
  byte *unaff_XMM8_Qb;
  byte *unaff_XMM9_Qa;
  byte *unaff_XMM9_Qb;
  byte *unaff_XMM10_Qa;
  byte *unaff_XMM10_Qb;
  byte *unaff_XMM11_Qa;
  byte *unaff_XMM11_Qb;
  byte *unaff_XMM12_Qa;
  byte *unaff_XMM12_Qb;
  char cStack_310;
  undefined1 uStack_30f;
  undefined4 uStack_30e;
  undefined2 uStack_30a;
  byte *pbStack_308;
  byte *pbStack_300;
  byte *pbStack_2f8;
  undefined1 uStack_2f0;
  byte *pbStack_2ef;
  undefined8 uStack_2e7;
  undefined4 uStack_2df;
  undefined4 uStack_2db;
  undefined4 uStack_2d7;
  undefined4 uStack_2d3;
  char cStack_2c8;
  undefined4 uStack_2c7;
  undefined3 uStack_2c3;
  byte *pbStack_2c0;
  byte *pbStack_2b8;
  byte *pbStack_2b0;
  undefined1 uStack_2a8;
  undefined4 uStack_2a7;
  undefined3 uStack_2a3;
  byte *pbStack_2a0;
  undefined8 uStack_298;
  undefined4 uStack_290;
  undefined4 uStack_28c;
  undefined4 uStack_288;
  undefined4 uStack_284;
  byte *pbStack_280;
  byte *pbStack_278;
  byte *pbStack_270;
  byte *pbStack_268;
  undefined8 *puStack_260;
  byte *pbStack_258;
  byte *pbStack_250;
  undefined8 uStack_248;
  undefined8 uStack_240;
  undefined8 uStack_238;
  undefined7 uStack_230;
  undefined1 uStack_229;
  undefined4 uStack_218;
  undefined3 uStack_214;
  undefined4 uStack_210;
  undefined3 uStack_20c;
  undefined8 uStack_208;
  undefined1 uStack_200;
  undefined8 uStack_1f8;
  undefined8 uStack_1f0;
  undefined8 uStack_1e8;
  undefined4 uStack_1e0;
  undefined4 uStack_1dc;
  byte *pbStack_1c8;
  undefined8 uStack_1c0;
  undefined8 uStack_1b8;
  undefined4 uStack_1b0;
  undefined4 uStack_1ac;
  undefined8 uStack_198;
  undefined8 uStack_190;
  undefined8 uStack_188;
  undefined4 uStack_180;
  undefined4 uStack_17c;
  byte *pbStack_170;
  undefined2 uStack_168;
  byte *pbStack_160;
  byte *pbStack_158;
  undefined4 uStack_150;
  undefined2 uStack_14c;
  undefined4 uStack_148;
  undefined2 uStack_144;
  undefined8 uStack_140;
  ulonglong uStack_138;
  byte bStack_12c;
  byte bStack_12b;
  byte bStack_12a;
  byte bStack_129;
  byte *pbStack_128;
  char cStack_120;
  byte bStack_11f;
  undefined2 uStack_11e;
  undefined1 uStack_11c;
  undefined1 uStack_11b;
  undefined2 uStack_11a;
  byte *pbStack_118;
  byte *pbStack_110;
  byte *pbStack_108;
  undefined4 uStack_100;
  undefined4 uStack_fc;
  undefined1 uStack_f8;
  undefined7 uStack_f7;
  undefined1 uStack_f0;
  undefined7 uStack_ef;
  undefined1 uStack_e8;
  undefined7 uStack_e7;
  undefined1 uStack_e0;
  undefined7 uStack_df;
  undefined4 uStack_d8;
  undefined2 uStack_d4;
  byte *pbStack_d0;
  byte bStack_c1;
  undefined8 uStack_c0;
  
  uStack_c0 = 0xfffffffffffffffe;
  uVar13 = *(undefined8 *)(param_2 + 0x40);
  uVar14 = *(ulonglong *)(param_2 + 0x48);
  pbVar12 = param_2 + 0x10;
  FUN_140bca140(&cStack_120,pbVar12);
  if (cStack_120 != '2') {
    param_1[8] = CONCAT71(uStack_df,uStack_e0);
    param_1[6] = CONCAT71(uStack_ef,uStack_f0);
    param_1[7] = CONCAT71(uStack_e7,uStack_e8);
    param_1[4] = CONCAT44(uStack_fc,uStack_100);
    param_1[5] = CONCAT71(uStack_f7,uStack_f8);
    param_1[2] = pbStack_110;
    param_1[3] = pbStack_108;
    *param_1 = CONCAT26(uStack_11a,
                        CONCAT15(uStack_11b,
                                 CONCAT14(uStack_11c,
                                          CONCAT22(uStack_11e,CONCAT11(bStack_11f,cStack_120)))));
    param_1[1] = pbStack_118;
    return param_1;
  }
  bVar8 = param_2[0x50];
  if (bVar8 == 0) {
    cVar6 = FUN_140bc41b0(pbVar12,0x28);
    if (cVar6 == '\0') {
      cStack_120 = '\x16';
      pbStack_110 = unaff_R14;
      goto LAB_141e3c0a6;
    }
    bVar8 = param_2[0x50];
  }
  param_2[0x50] = 0;
  if (*(int *)param_2 == 1) {
    if (*(longlong *)(param_2 + 8) != 0) {
      *(longlong *)(param_2 + 8) = *(longlong *)(param_2 + 8) + -1;
      goto LAB_141e3b532;
    }
    cStack_120 = '1';
    pbStack_110 = unaff_R14;
  }
  else {
LAB_141e3b532:
    pbVar4 = _UNK_1429ca3b8;
    pbVar3 = _DAT_1429ca3b0;
    pbVar16 = _UNK_1429ca368;
    pbVar15 = _DAT_1429ca360;
    uStack_168 = 0x103;
    bStack_c1 = 4;
    pbStack_d0 = (byte *)0x8000000000000002;
    pbStack_268 = pbVar12;
    puStack_260 = param_1;
    pbStack_170 = param_2;
    uStack_140 = uVar13;
    uStack_138 = uVar14;
    bStack_129 = bVar8;
LAB_141e3b590:
    do {
      uVar11 = SUB81(param_1,0);
      pbStack_128 = pbStack_d0;
      FUN_140bcd950(&cStack_120,&pbStack_170);
      pbVar9 = pbStack_170;
      pbVar10 = (byte *)(ulonglong)bStack_11f;
      if (cStack_120 != '2') {
        uStack_148 = CONCAT13(uStack_11b,CONCAT12(uStack_11c,uStack_11e));
        uStack_1f8 = (byte *)CONCAT17(uStack_f8,CONCAT43(uStack_fc,uStack_100._1_3_));
        uStack_190 = CONCAT17(uStack_f0,uStack_f7);
        uStack_188 = CONCAT17(uStack_e8,uStack_ef);
        uStack_1e0 = (undefined4)uStack_e7;
        uStack_1dc = CONCAT13(uStack_e0,(int3)((uint7)uStack_e7 >> 0x20));
        cVar6 = cStack_120;
        pbVar12 = pbStack_118;
        uVar11 = (byte)uStack_100;
        unaff_XMM12_Qa = pbStack_110;
        unaff_XMM12_Qb = pbStack_108;
        uStack_198 = uStack_1f8;
        uStack_180 = uStack_1e0;
        uStack_17c = uStack_1dc;
        pbVar9 = pbStack_158;
        uStack_144 = uStack_11a;
        uStack_d8 = uStack_148;
        uStack_d4 = uStack_11a;
        goto joined_r0x000141e3c606;
      }
      if ((bStack_11f & 1) == 0) {
LAB_141e3bd89:
        uVar14 = uStack_138;
        uVar13 = uStack_140;
        pbVar12 = pbStack_268;
        unaff_XMM12_Qb = _UNK_1429ca3b8;
        unaff_XMM12_Qa = _DAT_1429ca3b0;
        cVar6 = ',';
        if (pbStack_d0 == (byte *)0x8000000000000002) {
          pbVar12 = &UNK_142f8ec1a;
          cStack_310 = cVar6;
          uStack_2f0 = uVar11;
          unaff_XMM12_Qa = _DAT_1429ca360;
          unaff_XMM12_Qb = _UNK_1429ca368;
          goto LAB_141e3bf66;
        }
        if (bStack_c1 == 4) {
          pbVar12 = &UNK_142f8ec20;
          cStack_310 = cVar6;
          uStack_2f0 = uVar11;
          if ((-0x7fffffffffffffff < (longlong)pbStack_d0) && (pbStack_d0 != (byte *)0x0)) {
            func_0x000140613c20(pbStack_158,pbStack_d0,1);
          }
          goto LAB_141e3bf66;
        }
        if ((*param_2 & 1) != 0) {
          lVar7 = -1;
          if (*(longlong *)(param_2 + 8) + 1 != 0) {
            lVar7 = *(longlong *)(param_2 + 8) + 1;
          }
          *(longlong *)(param_2 + 8) = lVar7;
        }
        FUN_140bca140(&cStack_120,pbStack_268);
        param_1 = puStack_260;
        param_2 = (byte *)(ulonglong)bStack_c1;
        if (cStack_120 == '2') {
          if (((bStack_129 & 1) != 0) || (cVar6 = FUN_140bc41b0(pbVar12,0x29), cVar6 != '\0')) {
            uVar5 = (undefined4)uStack_198;
            uVar2 = CONCAT14(uStack_1f8._7_1_,uStack_198._3_4_);
            uStack_1f8 = (byte *)CONCAT44((int)((uint5)uVar2 >> 8),uVar5);
            pbVar12 = pbStack_d0;
            pbStack_110 = pbStack_158;
            cStack_120 = '2';
            goto LAB_141e3c073;
          }
          cVar6 = '\x18';
          cStack_120 = '\x18';
          pbVar12 = pbStack_d0;
          pbStack_110 = pbStack_158;
          if (((pbStack_d0 + 0x7ffffffffffffffe < (byte *)0x3 &&
                pbStack_d0 + 0x7ffffffffffffffe != (byte *)0x1) ||
              (pbStack_110 = pbStack_158, (longlong)pbStack_d0 < -0x7ffffffffffffffe)) ||
             (pbStack_110 = pbStack_158, pbVar15 = pbStack_158, pbStack_d0 == (byte *)0x0))
          goto LAB_141e3c073;
        }
        else {
          uVar2 = CONCAT14(uStack_248._7_1_,CONCAT22(uStack_11a,CONCAT11(uStack_11b,uStack_11c)));
          uStack_248 = (byte *)CONCAT44((int)((uint5)uVar2 >> 8),
                                        CONCAT13(uStack_11c,CONCAT21(uStack_11e,bStack_11f)));
          pbStack_160 = pbStack_108;
          param_2 = (byte *)(ulonglong)(byte)uStack_100;
          uStack_1f8._0_7_ = CONCAT43(uStack_fc,(int3)((uint)uStack_100 >> 8));
          pbStack_1c8 = (byte *)CONCAT71(uStack_f7,uStack_f8);
          uStack_1c0 = CONCAT71(uStack_ef,uStack_f0);
          uStack_1b8 = CONCAT71(uStack_e7,uStack_e8);
          uStack_1b0 = (undefined4)CONCAT71(uStack_df,uStack_e0);
          uStack_1ac = (undefined4)((uint7)uStack_df >> 0x18);
          pbVar12 = pbStack_118;
          pbVar15 = pbStack_110;
          cVar6 = cStack_120;
          if ((pbStack_d0 == (byte *)0x0 || (longlong)pbStack_d0 < -0x7ffffffffffffffe) ||
              pbStack_d0 + 0x7ffffffffffffffe != (byte *)0x1 &&
              pbStack_d0 + 0x7ffffffffffffffe < (byte *)0x3) goto LAB_141e3c073;
        }
        func_0x000140613c20(pbStack_158,pbStack_d0,1);
        pbStack_110 = pbVar15;
        cStack_120 = cVar6;
        goto LAB_141e3c073;
      }
      bVar8 = *pbStack_170;
      if ((char)uStack_168 != '\x01') {
        if ((char)uStack_168 != '\x03') {
          if ((bVar8 & 1) != 0) {
            if (*(longlong *)(pbStack_170 + 8) == 0) goto LAB_141e3bed5;
            *(longlong *)(pbStack_170 + 8) = *(longlong *)(pbStack_170 + 8) + -1;
          }
          pbStack_128 = pbStack_d0;
          FUN_140bc9b80(&cStack_120,pbStack_170 + 0x10);
          if (cStack_120 == '2') {
            *(byte **)(pbVar9 + 0x40) = pbStack_118;
            *(byte **)(pbVar9 + 0x48) = pbStack_110;
            pbVar12 = pbStack_270;
            unaff_XMM11_Qa = unaff_XMM8_Qa;
            unaff_XMM11_Qb = unaff_XMM8_Qb;
            pbVar10 = uStack_208;
            if (pbStack_110 == (byte *)0x7) {
              if (*(int *)(pbStack_118 + 3) != CONCAT31(_UNK_142bb123c,DAT_142bb123b) ||
                  *(int *)pbStack_118 != _DAT_142bb1238) goto LAB_141e3bb9e;
              uVar14 = 1;
              iVar1 = *(int *)pbStack_170;
            }
            else if ((pbStack_110 == (byte *)0x6) &&
                    (*(short *)(pbStack_118 + 4) == _DAT_142bb1236 &&
                     *(int *)pbStack_118 == _DAT_142bb1232)) {
              uVar14 = 0;
              iVar1 = *(int *)pbStack_170;
            }
            else {
LAB_141e3bb9e:
              uVar14 = 2;
              iVar1 = *(int *)pbStack_170;
            }
          }
          else {
            uVar14 = (ulonglong)bStack_11f;
            uStack_208._0_6_ =
                 CONCAT24(uStack_11a,CONCAT13(uStack_11b,CONCAT12(uStack_11c,uStack_11e)));
            pbStack_1c8 = (byte *)CONCAT17(uStack_f8,CONCAT43(uStack_fc,uStack_100._1_3_));
            uStack_1c0 = CONCAT17(uStack_f0,uStack_f7);
            uStack_1b8 = CONCAT17(uStack_e8,uStack_ef);
            uStack_1b0 = (undefined4)uStack_e7;
            uStack_1ac._0_3_ = (undefined3)((uint7)uStack_e7 >> 0x20);
            uStack_1ac = CONCAT13(uStack_e0,(undefined3)uStack_1ac);
            iVar1 = *(int *)pbStack_170;
            pbVar12 = pbStack_118;
            bStack_12a = (byte)uStack_100;
            pbVar10 = uStack_208;
            unaff_XMM11_Qa = pbStack_110;
            unaff_XMM11_Qb = pbStack_108;
          }
          if (iVar1 == 1) {
            lVar7 = *(longlong *)(pbStack_170 + 8) + 1;
            if (lVar7 == 0) {
              lVar7 = -1;
            }
            *(longlong *)(pbStack_170 + 8) = lVar7;
          }
          uStack_208._4_2_ = (undefined2)((ulonglong)pbVar10 >> 0x20);
          uStack_144 = uStack_208._4_2_;
          uStack_208._0_4_ = SUB84(pbVar10,0);
          uStack_148 = (undefined4)uStack_208;
          param_1 = (undefined8 *)(ulonglong)bStack_12a;
          unaff_XMM8_Qa = unaff_XMM11_Qa;
          unaff_XMM8_Qb = unaff_XMM11_Qb;
          pbStack_270 = pbVar12;
          uStack_208 = pbVar10;
          goto LAB_141e3b9e8;
        }
        if ((bVar8 & 1) == 0) {
LAB_141e3b60a:
          uStack_208 = pbStack_170;
          uStack_200 = 0;
          pbStack_128 = pbStack_d0;
          FUN_142374600(&cStack_120,&uStack_208);
          unaff_XMM11_Qa = unaff_XMM9_Qa;
          unaff_XMM11_Qb = unaff_XMM9_Qb;
          if (cStack_120 != '2') {
            uStack_14c = uStack_11a;
            uStack_150 = CONCAT13(uStack_11b,CONCAT12(uStack_11c,uStack_11e));
            pbStack_280 = pbStack_118;
            bStack_12c = (byte)uStack_100;
            pbStack_1c8 = (byte *)CONCAT17(uStack_f8,CONCAT43(uStack_fc,uStack_100._1_3_));
            uStack_1c0 = CONCAT17(uStack_f0,uStack_f7);
            uStack_1b8 = CONCAT17(uStack_e8,uStack_ef);
            uStack_1b0 = (undefined4)uStack_e7;
            uStack_1ac._0_3_ = (undefined3)((uint7)uStack_e7 >> 0x20);
            uStack_1ac = CONCAT13(uStack_e0,(undefined3)uStack_1ac);
            unaff_XMM11_Qa = pbStack_110;
            unaff_XMM11_Qb = pbStack_108;
          }
          if (*(int *)pbStack_170 == 1) {
            lVar7 = *(longlong *)(pbStack_170 + 8) + 1;
            if (lVar7 == 0) {
              lVar7 = -1;
            }
            *(longlong *)(pbStack_170 + 8) = lVar7;
          }
          uVar14 = (ulonglong)bStack_11f;
          uStack_144 = uStack_14c;
          uStack_148 = uStack_150;
          param_1 = (undefined8 *)(ulonglong)bStack_12c;
          pbVar12 = pbStack_280;
          unaff_XMM9_Qa = unaff_XMM11_Qa;
          unaff_XMM9_Qb = unaff_XMM11_Qb;
          goto LAB_141e3b9e8;
        }
        if (*(longlong *)(pbStack_170 + 8) != 0) {
          *(longlong *)(pbStack_170 + 8) = *(longlong *)(pbStack_170 + 8) + -1;
          goto LAB_141e3b60a;
        }
LAB_141e3bed5:
        cVar6 = '1';
LAB_141e3bed8:
        pbVar10 = (byte *)(uVar14 & 0xffffffff);
        unaff_XMM12_Qa = unaff_XMM11_Qa;
        unaff_XMM12_Qb = unaff_XMM11_Qb;
        uStack_198 = uStack_1f8;
        uStack_190 = uStack_1f0;
        uStack_188 = uStack_1e8;
        uStack_180 = uStack_1e0;
        uStack_17c = uStack_1dc;
        pbVar9 = pbStack_158;
        uStack_d8 = uStack_148;
        uStack_d4 = uStack_144;
        goto joined_r0x000141e3c606;
      }
      if ((bVar8 & 1) != 0) {
        if (*(longlong *)(pbStack_170 + 8) == 0) goto LAB_141e3bed5;
        *(longlong *)(pbStack_170 + 8) = *(longlong *)(pbStack_170 + 8) + -1;
      }
      uStack_208 = pbStack_170;
      uStack_200 = 1;
      pbStack_128 = pbStack_d0;
      FUN_142374600(&cStack_120,&uStack_208);
      unaff_XMM11_Qa = unaff_XMM10_Qa;
      unaff_XMM11_Qb = unaff_XMM10_Qb;
      if (cStack_120 != '2') {
        uStack_14c = uStack_11a;
        uStack_150 = CONCAT13(uStack_11b,CONCAT12(uStack_11c,uStack_11e));
        pbStack_278 = pbStack_118;
        bStack_12b = (byte)uStack_100;
        pbStack_1c8 = (byte *)CONCAT17(uStack_f8,CONCAT43(uStack_fc,uStack_100._1_3_));
        uStack_1c0 = CONCAT17(uStack_f0,uStack_f7);
        uStack_1b8 = CONCAT17(uStack_e8,uStack_ef);
        uStack_1b0 = (undefined4)uStack_e7;
        uStack_1ac._0_3_ = (undefined3)((uint7)uStack_e7 >> 0x20);
        uStack_1ac = CONCAT13(uStack_e0,(undefined3)uStack_1ac);
        unaff_XMM11_Qa = pbStack_110;
        unaff_XMM11_Qb = pbStack_108;
      }
      if (*(int *)pbStack_170 == 1) {
        lVar7 = *(longlong *)(pbStack_170 + 8) + 1;
        if (lVar7 == 0) {
          lVar7 = -1;
        }
        *(longlong *)(pbStack_170 + 8) = lVar7;
      }
      uVar14 = (ulonglong)bStack_11f;
      uStack_144 = uStack_14c;
      uStack_148 = uStack_150;
      param_1 = (undefined8 *)(ulonglong)bStack_12b;
      pbVar12 = pbStack_278;
      unaff_XMM10_Qa = unaff_XMM11_Qa;
      unaff_XMM10_Qb = unaff_XMM11_Qb;
LAB_141e3b9e8:
      pbVar10 = pbStack_170;
      uVar11 = SUB81(param_1,0);
      cVar6 = cStack_120;
      uStack_1f8 = pbStack_1c8;
      uStack_1f0 = uStack_1c0;
      uStack_1e8 = uStack_1b8;
      uStack_1e0 = uStack_1b0;
      uStack_1dc = uStack_1ac;
      if (cStack_120 != '2') goto LAB_141e3bed8;
      if ((char)uVar14 != '\x02') {
        pbVar9 = pbStack_158;
        if (uVar14 == 0) {
          if (pbStack_d0 != (byte *)0x8000000000000002) {
            cVar6 = '-';
            pbVar12 = &UNK_142f8ec1a;
            unaff_XMM12_Qa = pbVar15;
            unaff_XMM12_Qb = pbVar16;
            goto joined_r0x000141e3c606;
          }
          pbStack_160 = pbStack_170 + 0x10;
          pbStack_128 = (byte *)0x8000000000000002;
          FUN_140bca140(&cStack_120);
          if (cStack_120 == '2') {
            pbStack_128 = (byte *)0x8000000000000002;
            cVar6 = FUN_140bc41b0(pbStack_160,0x3a);
            if (cVar6 == '\0') {
              pbVar12 = pbStack_d0;
              uVar13 = uStack_140;
              uVar14 = uStack_138;
              cStack_310 = '\x13';
              uStack_2f0 = uVar11;
              goto LAB_141e3bf66;
            }
            pbStack_128 = (byte *)0x8000000000000002;
            FUN_140bca140(&cStack_120,pbStack_160);
            if (cStack_120 == '2') {
              if ((*pbVar10 & 1) != 0) {
                if (*(longlong *)(pbVar10 + 8) == 0) {
                  pbVar12 = pbStack_d0;
                  uVar13 = uStack_140;
                  uVar14 = uStack_138;
                  cStack_310 = '1';
                  uStack_2f0 = uVar11;
                  goto LAB_141e3bf66;
                }
                *(longlong *)(pbVar10 + 8) = *(longlong *)(pbVar10 + 8) + -1;
              }
              pbStack_128 = (byte *)0x8000000000000002;
              pbStack_1c8 = pbVar10;
              FUN_140fad7f0(&cStack_120,&pbStack_1c8);
              if (cStack_120 == '2') {
                pbStack_d0 = pbStack_118;
                pbStack_258 = pbStack_110;
                pbStack_250 = pbStack_108;
                if ((*pbVar10 & 1) != 0) {
                  lVar7 = *(longlong *)(pbVar10 + 8) + 1;
                  if (lVar7 == 0) {
                    lVar7 = -1;
                  }
                  *(longlong *)(pbVar10 + 8) = lVar7;
                }
                FUN_140bc8550(&cStack_120,pbStack_160);
                pbVar10 = (byte *)(ulonglong)bStack_11f;
                if (cStack_120 != '2') {
                  uStack_d4 = uStack_11a;
                  uStack_d8 = CONCAT13(uStack_11b,CONCAT12(uStack_11c,uStack_11e));
                  uStack_198 = (byte *)CONCAT17(uStack_f8,CONCAT43(uStack_fc,uStack_100._1_3_));
                  uStack_190 = CONCAT17(uStack_f0,uStack_f7);
                  uStack_188 = CONCAT17(uStack_e8,uStack_ef);
                  uStack_180 = (undefined4)uStack_e7;
                  uStack_17c._0_3_ = (undefined3)((uint7)uStack_e7 >> 0x20);
                  uStack_17c = CONCAT13(uStack_e0,(undefined3)uStack_17c);
                  pbVar12 = pbStack_118;
                  uVar13 = uStack_140;
                  uVar14 = uStack_138;
                  cStack_310 = cStack_120;
                  uStack_2f0 = (byte)uStack_100;
                  unaff_XMM12_Qa = pbStack_110;
                  unaff_XMM12_Qb = pbStack_108;
                  if (((longlong)pbStack_d0 < -0x7ffffffffffffffe) ||
                     (pbVar9 = pbStack_258, cVar6 = cStack_120, uVar11 = (byte)uStack_100,
                     pbStack_d0 == (byte *)0x0)) goto LAB_141e3bf66;
                  goto LAB_141e3bf5c;
                }
                uStack_168 = CONCAT11(bStack_11f,(char)uStack_168);
                pbStack_158 = pbStack_258;
                pbStack_160 = pbStack_250;
                unaff_XMM12_Qa = pbStack_258;
                unaff_XMM12_Qb = pbStack_250;
                goto LAB_141e3b590;
              }
            }
          }
          pbVar10 = (byte *)(ulonglong)bStack_11f;
          uStack_d8 = CONCAT13(uStack_11b,CONCAT12(uStack_11c,uStack_11e));
          uStack_d4 = uStack_11a;
          uStack_198 = (byte *)CONCAT17(uStack_f8,CONCAT43(uStack_fc,uStack_100._1_3_));
          uStack_190 = CONCAT17(uStack_f0,uStack_f7);
          uStack_188 = CONCAT17(uStack_e8,uStack_ef);
          uStack_180 = (undefined4)uStack_e7;
          uStack_17c._0_3_ = (undefined3)((uint7)uStack_e7 >> 0x20);
          uStack_17c = CONCAT13(uStack_e0,(undefined3)uStack_17c);
          pbVar12 = pbStack_118;
          uVar13 = uStack_140;
          uVar14 = uStack_138;
          cStack_310 = cStack_120;
          uStack_2f0 = (byte)uStack_100;
          unaff_XMM12_Qa = pbStack_110;
          unaff_XMM12_Qb = pbStack_108;
          goto LAB_141e3bf66;
        }
        if ((int)uVar14 != 1) goto LAB_141e3bd89;
        if (bStack_c1 != 4) {
          cVar6 = '-';
          pbVar12 = &UNK_142f8ec20;
          unaff_XMM12_Qa = pbVar3;
          unaff_XMM12_Qb = pbVar4;
          goto joined_r0x000141e3c606;
        }
        pbStack_258 = pbStack_170 + 0x10;
        pbStack_128 = pbStack_d0;
        FUN_140bca140(&cStack_120);
        if (cStack_120 == '2') {
          pbStack_128 = pbStack_d0;
          cVar6 = FUN_140bc41b0(pbStack_258,0x3a);
          if (cVar6 == '\0') {
            cVar6 = '\x13';
LAB_141e3c560:
            pbVar10 = (byte *)(ulonglong)bStack_c1;
          }
          else {
            pbStack_128 = pbStack_d0;
            FUN_140bca140(&cStack_120,pbStack_258);
            if (cStack_120 != '2') goto LAB_141e3c371;
            if ((*pbVar10 & 1) != 0) {
              if (*(longlong *)(pbVar10 + 8) == 0) {
                cVar6 = '1';
                goto LAB_141e3c560;
              }
              *(longlong *)(pbVar10 + 8) = *(longlong *)(pbVar10 + 8) + -1;
            }
            pbStack_128 = pbStack_d0;
            FUN_141e44170(&cStack_120,pbVar10,&UNK_142bb1328,0xc);
            bStack_c1 = bStack_11f;
            if (cStack_120 != '2') {
              uStack_d4 = uStack_11a;
              uStack_d8 = CONCAT13(uStack_11b,CONCAT12(uStack_11c,uStack_11e));
              uStack_248 = (byte *)CONCAT17(uStack_f8,CONCAT43(uStack_fc,uStack_100._1_3_));
              uStack_240 = CONCAT17(uStack_f0,uStack_f7);
              uStack_238 = CONCAT17(uStack_e8,uStack_ef);
              uStack_230 = uStack_e7;
              uStack_229 = uStack_e0;
              pbVar12 = pbStack_118;
              cVar6 = cStack_120;
              uVar11 = (byte)uStack_100;
              pbVar15 = pbStack_110;
              pbVar16 = pbStack_108;
              goto LAB_141e3c560;
            }
            if ((*pbVar10 & 1) != 0) {
              lVar7 = *(longlong *)(pbVar10 + 8) + 1;
              if (lVar7 == 0) {
                lVar7 = -1;
              }
              *(longlong *)(pbVar10 + 8) = lVar7;
            }
            pbStack_128 = pbStack_d0;
            FUN_140bc8550(&cStack_120,pbStack_258);
            if (cStack_120 == '2') {
              uStack_168 = CONCAT11(bStack_11f,(char)uStack_168);
              goto LAB_141e3b590;
            }
            uStack_d4 = uStack_11a;
            uStack_d8 = CONCAT13(uStack_11b,CONCAT12(uStack_11c,uStack_11e));
            uStack_248 = (byte *)CONCAT17(uStack_f8,CONCAT43(uStack_fc,uStack_100._1_3_));
            uStack_240 = CONCAT17(uStack_f0,uStack_f7);
            uStack_238 = CONCAT17(uStack_e8,uStack_ef);
            uStack_230 = uStack_e7;
            uStack_229 = uStack_e0;
            pbVar10 = (byte *)(ulonglong)bStack_11f;
            pbVar12 = pbStack_118;
            cVar6 = cStack_120;
            uVar11 = (byte)uStack_100;
            pbVar15 = pbStack_110;
            pbVar16 = pbStack_108;
          }
        }
        else {
LAB_141e3c371:
          pbVar10 = (byte *)(ulonglong)bStack_11f;
          uStack_d8 = CONCAT13(uStack_11b,CONCAT12(uStack_11c,uStack_11e));
          uStack_d4 = uStack_11a;
          uStack_248 = (byte *)CONCAT17(uStack_f8,CONCAT43(uStack_fc,uStack_100._1_3_));
          uStack_240 = CONCAT17(uStack_f0,uStack_f7);
          uStack_238 = CONCAT17(uStack_e8,uStack_ef);
          uStack_230 = uStack_e7;
          uStack_229 = uStack_e0;
          pbVar12 = pbStack_118;
          cVar6 = cStack_120;
          uVar11 = (byte)uStack_100;
          pbVar15 = pbStack_110;
          pbVar16 = pbStack_108;
        }
        uStack_180 = (undefined4)uStack_230;
        uStack_17c = (undefined4)(CONCAT17(uStack_229,uStack_230) >> 0x20);
        unaff_XMM12_Qa = pbVar15;
        unaff_XMM12_Qb = pbVar16;
        uStack_198 = uStack_248;
        uStack_190 = uStack_240;
        uStack_188 = uStack_238;
        pbVar9 = pbStack_158;
        goto joined_r0x000141e3c606;
      }
      pbStack_128 = pbStack_d0;
      FUN_141e3e370(&cStack_120,&pbStack_170);
    } while (cStack_120 == '2');
    uStack_d8 = CONCAT13(uStack_11b,CONCAT12(uStack_11c,uStack_11e));
    uStack_198 = (byte *)CONCAT17(uStack_f8,CONCAT43(uStack_fc,uStack_100._1_3_));
    uStack_190 = CONCAT17(uStack_f0,uStack_f7);
    uStack_188 = CONCAT17(uStack_e8,uStack_ef);
    uStack_180 = (undefined4)uStack_e7;
    uStack_17c._0_3_ = (undefined3)((uint7)uStack_e7 >> 0x20);
    uStack_17c = CONCAT13(uStack_e0,(undefined3)uStack_17c);
    cVar6 = cStack_120;
    pbVar12 = pbStack_118;
    uVar11 = (byte)uStack_100;
    unaff_XMM12_Qa = pbStack_110;
    unaff_XMM12_Qb = pbStack_108;
    pbVar10 = (byte *)(ulonglong)bStack_11f;
    pbVar9 = pbStack_158;
    uStack_d4 = uStack_11a;
joined_r0x000141e3c606:
    uVar13 = uStack_140;
    uVar14 = uStack_138;
    cStack_310 = cVar6;
    uStack_2f0 = uVar11;
    pbStack_158 = pbVar9;
    if ((-0x7ffffffffffffffe < (longlong)pbStack_d0) && (pbStack_d0 != (byte *)0x0)) {
LAB_141e3bf5c:
      uVar14 = uStack_138;
      uVar13 = uStack_140;
      func_0x000140613c20(pbVar9,pbStack_d0,1);
      cStack_310 = cVar6;
      uStack_2f0 = uVar11;
    }
LAB_141e3bf66:
    uStack_30f = SUB81(pbVar10,0);
    uStack_30e = uStack_d8;
    uStack_30a = uStack_d4;
    pbStack_2ef = uStack_198;
    uStack_2e7 = uStack_190;
    uStack_2df = (undefined4)uStack_188;
    uStack_2db = uStack_188._4_4_;
    uStack_2d7 = uStack_180;
    uStack_2d3 = uStack_17c;
    pbStack_308 = pbVar12;
    pbStack_300 = unaff_XMM12_Qa;
    pbStack_2f8 = unaff_XMM12_Qb;
    FUN_140bcd9f0(&cStack_120,&cStack_310,0,0);
    uStack_248._0_7_ =
         CONCAT43(CONCAT22(uStack_11a,CONCAT11(uStack_11b,uStack_11c)),
                  CONCAT21(uStack_11e,bStack_11f));
    pbStack_160 = pbStack_108;
    param_2 = (byte *)(ulonglong)(byte)uStack_100;
    uStack_218._0_3_ = (undefined3)((uint)uStack_100 >> 8);
    uStack_218._3_1_ = (undefined1)uStack_fc;
    uStack_214 = (undefined3)((uint)uStack_fc >> 8);
    pbStack_1c8 = (byte *)CONCAT71(uStack_f7,uStack_f8);
    uStack_1c0 = CONCAT71(uStack_ef,uStack_f0);
    uStack_1b8 = CONCAT71(uStack_e7,uStack_e8);
    uStack_1b0 = (undefined4)CONCAT71(uStack_df,uStack_e0);
    uStack_1ac = (undefined4)((uint7)uStack_df >> 0x18);
    uStack_20c = uStack_214;
    uStack_210 = uStack_218;
    uStack_1f8 = (byte *)CONCAT44(CONCAT13(uStack_1f8._7_1_,uStack_214),uStack_218);
    param_1 = puStack_260;
    pbVar12 = pbStack_118;
  }
LAB_141e3c073:
  if (cStack_120 == '2') {
    uVar5 = (undefined4)uStack_1f8;
    *(undefined4 *)((longlong)param_1 + 0x24) = uStack_1f8._3_4_;
    *(undefined4 *)((longlong)param_1 + 0x21) = uVar5;
    param_1[1] = pbVar12;
    param_1[2] = pbStack_110;
    param_1[3] = pbStack_160;
    *(char *)(param_1 + 4) = (char)param_2;
    *(undefined1 *)param_1 = 0x32;
    return param_1;
  }
LAB_141e3c0a6:
  uStack_2c3 = (undefined3)((ulonglong)uStack_248 >> 0x20);
  pbStack_2b0 = pbStack_160;
  uStack_2a8 = SUB81(param_2,0);
  uStack_2a3 = (undefined3)((ulonglong)uStack_1f8 >> 0x20);
  pbStack_2a0 = pbStack_1c8;
  uStack_298 = uStack_1c0;
  uStack_290 = (undefined4)uStack_1b8;
  uStack_28c = uStack_1b8._4_4_;
  uStack_288 = uStack_1b0;
  uStack_284 = uStack_1ac;
  cStack_2c8 = cStack_120;
  uStack_2c7 = (undefined4)uStack_248;
  pbStack_2c0 = pbVar12;
  pbStack_2b8 = pbStack_110;
  uStack_2a7 = (undefined4)uStack_1f8;
  FUN_140bcd9f0(param_1,&cStack_2c8,uVar13,uVar14);
  return param_1;
}

