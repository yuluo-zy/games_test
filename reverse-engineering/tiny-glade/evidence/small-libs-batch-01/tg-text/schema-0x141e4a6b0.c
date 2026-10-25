
/* WARNING: Type propagation algorithm not settling */
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

char * FUN_141e4a6b0(char *param_1,byte *param_2,undefined8 param_3,longlong param_4)

{
  byte *pbVar1;
  int iVar2;
  uint uVar3;
  int *piVar4;
  byte *pbVar5;
  undefined *puVar6;
  char cVar7;
  longlong lVar8;
  byte bVar9;
  undefined8 uVar10;
  byte bVar11;
  byte *pbVar12;
  undefined2 uVar13;
  undefined7 uVar15;
  byte *pbVar14;
  undefined4 uVar16;
  undefined4 unaff_XMM6_Da;
  undefined4 unaff_XMM6_Db;
  undefined4 unaff_XMM6_Dc;
  undefined4 unaff_XMM6_Dd;
  undefined4 unaff_XMM7_Da;
  undefined4 unaff_XMM7_Db;
  undefined4 unaff_XMM7_Dc;
  undefined4 unaff_XMM7_Dd;
  undefined4 unaff_XMM8_Da;
  undefined4 unaff_XMM8_Db;
  undefined4 unaff_XMM8_Dc;
  undefined4 unaff_XMM8_Dd;
  undefined4 unaff_XMM10_Da;
  undefined4 unaff_XMM11_Da;
  undefined4 unaff_XMM11_Db;
  undefined4 unaff_XMM11_Dc;
  undefined4 unaff_XMM11_Dd;
  undefined4 unaff_XMM12_Da;
  undefined4 unaff_XMM14_Da;
  undefined *puStack_278;
  undefined *puStack_270;
  undefined *puStack_268;
  byte *pbStack_258;
  longlong lStack_250;
  undefined8 uStack_248;
  undefined8 uStack_240;
  undefined8 uStack_238;
  undefined *puStack_230;
  undefined8 uStack_228;
  undefined8 uStack_220;
  undefined4 uStack_214;
  byte *pbStack_210;
  longlong lStack_208;
  undefined4 uStack_200;
  undefined4 uStack_1fc;
  undefined4 uStack_1f8;
  undefined4 uStack_1f4;
  undefined4 uStack_1ec;
  uint uStack_1e4;
  uint uStack_1e0;
  uint uStack_1dc;
  undefined8 uStack_1d8;
  undefined8 uStack_1d0;
  byte *pbStack_1c8;
  undefined2 uStack_1c0;
  byte *pbStack_1b8;
  longlong lStack_1b0;
  undefined8 uStack_1a8;
  undefined8 uStack_1a0;
  undefined *puStack_190;
  undefined2 uStack_182;
  undefined2 uStack_180;
  undefined2 uStack_17e;
  uint uStack_17c;
  undefined *puStack_178;
  undefined4 uStack_16c;
  longlong lStack_168;
  longlong lStack_160;
  uint uStack_154;
  undefined8 uStack_150;
  char cStack_148;
  byte bStack_147;
  undefined2 uStack_146;
  undefined4 uStack_144;
  undefined2 uStack_140;
  undefined6 uStack_13e;
  undefined2 uStack_138;
  undefined2 uStack_136;
  undefined4 uStack_134;
  undefined2 uStack_130;
  undefined2 uStack_12e;
  undefined4 uStack_12c;
  undefined2 uStack_128;
  undefined2 uStack_126;
  uint uStack_124;
  undefined2 uStack_120;
  undefined6 uStack_11e;
  undefined2 uStack_118;
  undefined6 uStack_116;
  undefined2 uStack_110;
  undefined6 uStack_10e;
  undefined2 uStack_108;
  undefined6 uStack_106;
  longlong lStack_100;
  undefined1 uStack_f2;
  byte bStack_f1;
  undefined8 uStack_f0;
  
  uStack_f0 = 0xfffffffffffffffe;
  if ((param_2[0x50] == 0) &&
     (FUN_140bc8c90(&cStack_148,param_2 + 0x10,param_3,param_4), cStack_148 != '2')) {
    *(ulonglong *)(param_1 + 0x40) = CONCAT62(uStack_106,uStack_108);
    *(ulonglong *)(param_1 + 0x32) = CONCAT26(uStack_110,uStack_116);
    *(ulonglong *)(param_1 + 0x3a) = CONCAT26(uStack_108,uStack_10e);
    *(ulonglong *)(param_1 + 0x22) = CONCAT26(uStack_120,CONCAT42(uStack_124,uStack_126));
    *(ulonglong *)(param_1 + 0x2a) = CONCAT26(uStack_118,uStack_11e);
    *(ulonglong *)(param_1 + 0x12) = CONCAT26(uStack_130,CONCAT42(uStack_134,uStack_136));
    *(ulonglong *)(param_1 + 0x1a) = CONCAT26(uStack_128,CONCAT42(uStack_12c,uStack_12e));
    *(ulonglong *)(param_1 + 2) = CONCAT26(uStack_140,CONCAT42(uStack_144,uStack_146));
    *(ulonglong *)(param_1 + 10) = CONCAT26(uStack_138,uStack_13e);
    *param_1 = cStack_148;
    param_1[1] = bStack_147;
  }
  else {
    pbVar1 = param_2 + 0x10;
    FUN_140bca140(&cStack_148,pbVar1);
    if (cStack_148 == '2') {
      bStack_f1 = param_2[0x50];
      if (bStack_f1 == 0) {
        cVar7 = FUN_140bc41b0(pbVar1,0x28);
        if (cVar7 == '\0') {
          if (param_4 != 0) {
            *param_1 = '\x17';
            *(undefined8 *)(param_1 + 8) = param_3;
            *(longlong *)(param_1 + 0x10) = param_4;
            return param_1;
          }
          *param_1 = '\x16';
          return param_1;
        }
        bStack_f1 = param_2[0x50];
      }
      param_2[0x50] = 0;
      if ((*param_2 & 1) != 0) {
        if (*(longlong *)(param_2 + 8) == 0) {
          *param_1 = '1';
          return param_1;
        }
        *(longlong *)(param_2 + 8) = *(longlong *)(param_2 + 8) + -1;
      }
      uStack_1c0 = 0x103;
      uStack_17c = 0;
      lStack_100 = -0x8000000000000000;
      pbVar14 = pbVar1;
      pbStack_1c8 = param_2;
      lStack_160 = param_4;
      uStack_150 = param_3;
LAB_141e4a810:
      do {
        uVar13 = SUB82(pbVar14,0);
        uStack_f2 = 1;
        lStack_168 = lStack_100;
        FUN_140bcd950(&cStack_148,&pbStack_1c8);
        bVar11 = bStack_147;
        pbVar5 = pbStack_1c8;
        pbVar12 = (byte *)(ulonglong)bStack_147;
        if (cStack_148 != '2') {
          puStack_230 = (undefined *)CONCAT62(uStack_13e,uStack_140);
          unaff_XMM6_Da = CONCAT22(uStack_136,uStack_138);
          unaff_XMM6_Dc = CONCAT22(uStack_12e,uStack_130);
          pbStack_258 = (byte *)CONCAT62(uStack_11e,uStack_120);
          lStack_250 = CONCAT62(uStack_116,uStack_118);
          uStack_248 = CONCAT62(uStack_10e,uStack_110);
          uStack_240 = CONCAT62(uStack_106,uStack_108);
          unaff_XMM10_Da = uStack_144;
          cVar7 = cStack_148;
          uVar13 = uStack_146;
          uVar3 = uStack_124;
          unaff_XMM6_Db = uStack_134;
          unaff_XMM6_Dd = uStack_12c;
LAB_141e4b088:
          uStack_1a8 = uStack_248;
          uStack_1a0 = uStack_240;
          pbStack_1b8 = pbStack_258;
          lStack_1b0 = lStack_250;
          goto LAB_141e4b09a;
        }
        if ((bStack_147 & 1) == 0) {
LAB_141e4aef5:
          bVar9 = bStack_f1;
          if (!SBORROW8(0,lStack_100)) {
            uVar16 = uStack_16c;
            if ((uStack_17c & 1) == 0) {
              uVar16 = FUN_140b06780();
            }
            if ((*param_2 & 1) != 0) {
              lVar8 = -1;
              if (*(longlong *)(param_2 + 8) + 1 != 0) {
                lVar8 = *(longlong *)(param_2 + 8) + 1;
              }
              *(longlong *)(param_2 + 8) = lVar8;
            }
            FUN_140bca140(&cStack_148,pbVar1);
            if (cStack_148 == '2') {
              if (((bVar9 & 1) != 0) || (cVar7 = FUN_140bc41b0(pbVar1,0x29), cVar7 != '\0')) {
                *(longlong *)(param_1 + 8) = lStack_100;
                *(undefined8 *)(param_1 + 0x10) = uStack_1d8;
                *(undefined8 *)(param_1 + 0x18) = uStack_1d0;
                *(undefined4 *)(param_1 + 0x20) = uVar16;
                *param_1 = '2';
                return param_1;
              }
              *param_1 = '\x18';
            }
            else {
              *(ulonglong *)(param_1 + 0x40) = CONCAT62(uStack_106,uStack_108);
              *(ulonglong *)(param_1 + 0x30) = CONCAT62(uStack_116,uStack_118);
              *(ulonglong *)(param_1 + 0x38) = CONCAT62(uStack_10e,uStack_110);
              *(ulonglong *)(param_1 + 0x20) = CONCAT44(uStack_124,CONCAT22(uStack_126,uStack_128));
              *(ulonglong *)(param_1 + 0x28) = CONCAT62(uStack_11e,uStack_120);
              *(ulonglong *)(param_1 + 0x10) = CONCAT44(uStack_134,CONCAT22(uStack_136,uStack_138));
              *(ulonglong *)(param_1 + 0x18) = CONCAT44(uStack_12c,CONCAT22(uStack_12e,uStack_130));
              *(ulonglong *)param_1 =
                   CONCAT44(uStack_144,CONCAT22(uStack_146,CONCAT11(bStack_147,cStack_148)));
              *(ulonglong *)(param_1 + 8) = CONCAT62(uStack_13e,uStack_140);
            }
            if (lStack_100 == 0) {
              return param_1;
            }
            func_0x000140613c20(uStack_1d8,lStack_100,1);
            return param_1;
          }
          puStack_230 = &UNK_142f8ec0f;
          lVar8 = lStack_160;
          cVar7 = ',';
          unaff_XMM6_Da = _DAT_1429ca360;
          unaff_XMM6_Db = _UNK_1429ca364;
          unaff_XMM6_Dc = _UNK_1429ca368;
          unaff_XMM6_Dd = _UNK_1429ca36c;
          puVar6 = puStack_178;
          goto LAB_141e4b0f9;
        }
        bVar9 = *pbStack_1c8;
        if ((char)uStack_1c0 != '\x01') {
          if ((char)uStack_1c0 != '\x03') {
            if ((bVar9 & 1) != 0) {
              if (*(longlong *)(pbStack_1c8 + 8) == 0) goto LAB_141e4b06c;
              *(longlong *)(pbStack_1c8 + 8) = *(longlong *)(pbStack_1c8 + 8) + -1;
            }
            lStack_168 = lStack_100;
            uStack_f2 = 1;
            FUN_140bc9b80(&cStack_148,pbStack_1c8 + 0x10);
            if (cStack_148 == '2') {
              piVar4 = (int *)CONCAT62(uStack_13e,uStack_140);
              lVar8 = CONCAT44(uStack_134,CONCAT22(uStack_136,uStack_138));
              *(int **)(pbVar5 + 0x40) = piVar4;
              *(longlong *)(pbVar5 + 0x48) = lVar8;
              uVar15 = (undefined7)((ulonglong)pbVar14 >> 8);
              unaff_XMM10_Da = unaff_XMM14_Da;
              if (lVar8 == 5) {
                if ((char)piVar4[1] != DAT_142f93082 || *piVar4 != _DAT_142f9307e)
                goto LAB_141e4abb5;
                pbVar14 = (byte *)CONCAT71(uVar15,1);
                iVar2 = *(int *)pbStack_1c8;
              }
              else if ((lVar8 == 6) &&
                      ((short)piVar4[1] == _DAT_142f9307c && *piVar4 == _DAT_142f93078)) {
                pbVar14 = (byte *)0x0;
                iVar2 = *(int *)pbStack_1c8;
              }
              else {
LAB_141e4abb5:
                pbVar14 = (byte *)CONCAT71(uVar15,2);
                iVar2 = *(int *)pbStack_1c8;
              }
            }
            else {
              pbVar14 = (byte *)(ulonglong)bStack_147;
              puStack_268 = (undefined *)CONCAT62(uStack_13e,uStack_140);
              unaff_XMM11_Da = CONCAT22(uStack_136,uStack_138);
              unaff_XMM11_Dc = CONCAT22(uStack_12e,uStack_130);
              uStack_238 = (byte *)CONCAT62(uStack_11e,uStack_120);
              puStack_230 = (undefined *)CONCAT62(uStack_116,uStack_118);
              uStack_228 = CONCAT62(uStack_10e,uStack_110);
              uStack_220 = CONCAT62(uStack_106,uStack_108);
              iVar2 = *(int *)pbStack_1c8;
              uStack_17e = uStack_146;
              uStack_1dc = uStack_124;
              unaff_XMM10_Da = uStack_144;
              unaff_XMM11_Db = uStack_134;
              unaff_XMM11_Dd = uStack_12c;
            }
            if (iVar2 == 1) {
              lVar8 = *(longlong *)(pbStack_1c8 + 8) + 1;
              if (lVar8 == 0) {
                lVar8 = -1;
              }
              *(longlong *)(pbStack_1c8 + 8) = lVar8;
            }
            puStack_178 = (undefined *)CONCAT62(puStack_178._2_6_,uStack_17e);
            unaff_XMM14_Da = unaff_XMM10_Da;
            unaff_XMM6_Da = unaff_XMM11_Da;
            unaff_XMM6_Db = unaff_XMM11_Db;
            unaff_XMM6_Dc = unaff_XMM11_Dc;
            unaff_XMM6_Dd = unaff_XMM11_Dd;
            puStack_190 = puStack_268;
            uStack_154 = uStack_1dc;
            goto LAB_141e4ac17;
          }
          if ((bVar9 & 1) == 0) {
LAB_141e4a882:
            pbStack_1b8 = pbStack_1c8;
            lStack_1b0 = (ulonglong)lStack_1b0._1_7_ << 8;
            lStack_168 = lStack_100;
            uStack_f2 = 1;
            FUN_1423740a0(&cStack_148,&pbStack_1b8);
            if (cStack_148 != '2') {
              uStack_182 = uStack_146;
              uStack_1ec = uStack_144;
              puStack_278 = (undefined *)CONCAT62(uStack_13e,uStack_140);
              unaff_XMM7_Da = CONCAT22(uStack_136,uStack_138);
              unaff_XMM7_Dc = CONCAT22(uStack_12e,uStack_130);
              uStack_1e4 = uStack_124;
              uStack_238 = (byte *)CONCAT62(uStack_11e,uStack_120);
              puStack_230 = (undefined *)CONCAT62(uStack_116,uStack_118);
              uStack_228 = CONCAT62(uStack_10e,uStack_110);
              uStack_220 = CONCAT62(uStack_106,uStack_108);
              unaff_XMM7_Db = uStack_134;
              unaff_XMM7_Dd = uStack_12c;
            }
            if (*(int *)pbStack_1c8 == 1) {
              lVar8 = *(longlong *)(pbStack_1c8 + 8) + 1;
              if (lVar8 == 0) {
                lVar8 = -1;
              }
              *(longlong *)(pbStack_1c8 + 8) = lVar8;
            }
            pbVar14 = (byte *)(ulonglong)bStack_147;
            puStack_190 = puStack_278;
            puStack_178 = (undefined *)CONCAT62(puStack_178._2_6_,uStack_182);
            uStack_154 = uStack_1e4;
            unaff_XMM10_Da = uStack_1ec;
            unaff_XMM6_Da = unaff_XMM7_Da;
            unaff_XMM6_Db = unaff_XMM7_Db;
            unaff_XMM6_Dc = unaff_XMM7_Dc;
            unaff_XMM6_Dd = unaff_XMM7_Dd;
            goto LAB_141e4ac17;
          }
          if (*(longlong *)(pbStack_1c8 + 8) != 0) {
            *(longlong *)(pbStack_1c8 + 8) = *(longlong *)(pbStack_1c8 + 8) + -1;
            goto LAB_141e4a882;
          }
LAB_141e4b06c:
          cVar7 = '1';
LAB_141e4b06f:
          puStack_230 = puStack_190;
          uVar13 = puStack_178._0_2_;
          uVar3 = uStack_154;
          bVar11 = (byte)pbVar14;
          goto LAB_141e4b088;
        }
        if ((bVar9 & 1) != 0) {
          if (*(longlong *)(pbStack_1c8 + 8) == 0) goto LAB_141e4b06c;
          *(longlong *)(pbStack_1c8 + 8) = *(longlong *)(pbStack_1c8 + 8) + -1;
        }
        pbStack_1b8 = pbStack_1c8;
        lStack_1b0 = CONCAT71(lStack_1b0._1_7_,1);
        lStack_168 = lStack_100;
        uStack_f2 = 1;
        FUN_1423740a0(&cStack_148,&pbStack_1b8);
        unaff_XMM10_Da = unaff_XMM12_Da;
        if (cStack_148 != '2') {
          uStack_180 = uStack_146;
          puStack_270 = (undefined *)CONCAT62(uStack_13e,uStack_140);
          unaff_XMM8_Da = CONCAT22(uStack_136,uStack_138);
          unaff_XMM8_Dc = CONCAT22(uStack_12e,uStack_130);
          uStack_1e0 = uStack_124;
          uStack_238 = (byte *)CONCAT62(uStack_11e,uStack_120);
          puStack_230 = (undefined *)CONCAT62(uStack_116,uStack_118);
          uStack_228 = CONCAT62(uStack_10e,uStack_110);
          uStack_220 = CONCAT62(uStack_106,uStack_108);
          unaff_XMM8_Db = uStack_134;
          unaff_XMM8_Dd = uStack_12c;
          unaff_XMM10_Da = uStack_144;
        }
        if (*(int *)pbStack_1c8 == 1) {
          lVar8 = *(longlong *)(pbStack_1c8 + 8) + 1;
          if (lVar8 == 0) {
            lVar8 = -1;
          }
          *(longlong *)(pbStack_1c8 + 8) = lVar8;
        }
        pbVar14 = (byte *)(ulonglong)bStack_147;
        puStack_190 = puStack_270;
        puStack_178 = (undefined *)CONCAT62(puStack_178._2_6_,uStack_180);
        uStack_154 = uStack_1e0;
        unaff_XMM12_Da = unaff_XMM10_Da;
        unaff_XMM6_Da = unaff_XMM8_Da;
        unaff_XMM6_Db = unaff_XMM8_Db;
        unaff_XMM6_Dc = unaff_XMM8_Dc;
        unaff_XMM6_Dd = unaff_XMM8_Dd;
LAB_141e4ac17:
        pbVar5 = pbStack_1c8;
        uStack_240 = uStack_220;
        uStack_248 = uStack_228;
        lStack_250 = (longlong)puStack_230;
        pbStack_258 = uStack_238;
        uVar13 = SUB82(pbVar14,0);
        cVar7 = cStack_148;
        if (cStack_148 != '2') goto LAB_141e4b06f;
        if ((char)pbVar14 != '\x02') {
          if (((ulonglong)pbVar14 & 0xff) != 0) {
            if (((uint)pbVar14 & 0xff) != 1) goto LAB_141e4aef5;
            if (uStack_17c == 0) {
              pbVar12 = pbStack_1c8 + 0x10;
              lStack_168 = lStack_100;
              uStack_f2 = 1;
              FUN_140bca140(&cStack_148,pbVar12);
              if (cStack_148 != '2') break;
              lStack_168 = lStack_100;
              uStack_f2 = 1;
              cVar7 = FUN_140bc41b0(pbVar12,0x3a);
              if (cVar7 != '\0') {
                lStack_168 = lStack_100;
                uStack_f2 = 1;
                FUN_140bca140(&cStack_148,pbVar12);
                if (cStack_148 != '2') break;
                if ((*pbVar5 & 1) != 0) {
                  if (*(longlong *)(pbVar5 + 8) == 0) {
                    cVar7 = '1';
                    goto LAB_141e4b2e2;
                  }
                  *(longlong *)(pbVar5 + 8) = *(longlong *)(pbVar5 + 8) + -1;
                }
                lStack_168 = lStack_100;
                uStack_f2 = 1;
                FUN_141e5b990(&cStack_148,pbVar12);
                if (cStack_148 != '2') break;
                if ((*pbVar5 & 1) != 0) {
                  lVar8 = *(longlong *)(pbVar5 + 8) + 1;
                  if (lVar8 == 0) {
                    lVar8 = -1;
                  }
                  *(longlong *)(pbVar5 + 8) = lVar8;
                }
                uStack_16c = uStack_144;
                lStack_168 = lStack_100;
                uStack_f2 = 1;
                FUN_140bc8550(&cStack_148,pbVar12);
                if (cStack_148 != '2') break;
                uStack_1c0 = CONCAT11(bStack_147,(char)uStack_1c0);
                uStack_17c = 1;
                goto LAB_141e4a810;
              }
              cVar7 = '\x13';
            }
            else {
              cVar7 = '-';
            }
LAB_141e4b2e2:
            bVar11 = (byte)pbVar12;
            puStack_230 = &UNK_142f8ec15;
            unaff_XMM6_Da = _DAT_1429ca3a0;
            unaff_XMM6_Db = _UNK_1429ca3a4;
            unaff_XMM6_Dc = _UNK_1429ca3a8;
            unaff_XMM6_Dd = _UNK_1429ca3ac;
            unaff_XMM10_Da = uStack_16c;
            goto joined_r0x000141e4b31b;
          }
          if (!SBORROW8(0,lStack_100)) {
            cVar7 = '-';
            puStack_230 = &UNK_142f8ec0f;
            unaff_XMM6_Da = _DAT_1429ca360;
            unaff_XMM6_Db = _UNK_1429ca364;
            unaff_XMM6_Dc = _UNK_1429ca368;
            unaff_XMM6_Dd = _UNK_1429ca36c;
            puVar6 = puStack_178;
            unaff_XMM10_Da = uStack_16c;
            lVar8 = lStack_160;
            bVar9 = bStack_f1;
            goto joined_r0x000141e4b35d;
          }
          lStack_168 = -0x8000000000000000;
          uStack_f2 = 1;
          FUN_141e3d970(&cStack_148,&pbStack_1c8);
          if (cStack_148 != '2') {
            puStack_230 = (undefined *)CONCAT62(uStack_13e,uStack_140);
            unaff_XMM6_Da = CONCAT22(uStack_136,uStack_138);
            unaff_XMM6_Dc = CONCAT22(uStack_12e,uStack_130);
            param_2 = (byte *)(ulonglong)uStack_124;
            pbStack_1b8 = (byte *)CONCAT62(uStack_11e,uStack_120);
            lStack_1b0 = CONCAT62(uStack_116,uStack_118);
            uStack_1a8 = CONCAT62(uStack_10e,uStack_110);
            uStack_1a0 = CONCAT62(uStack_106,uStack_108);
            lVar8 = lStack_160;
            unaff_XMM10_Da = uStack_144;
            cVar7 = cStack_148;
            bVar9 = bStack_f1;
            bVar11 = bStack_147;
            uVar13 = uStack_146;
            unaff_XMM6_Db = uStack_134;
            unaff_XMM6_Dd = uStack_12c;
            puVar6 = puStack_178;
            goto LAB_141e4b0f9;
          }
          lStack_100 = CONCAT62(uStack_13e,uStack_140);
          uStack_1d8 = CONCAT44(uStack_134,CONCAT22(uStack_136,uStack_138));
          uStack_1d0 = CONCAT44(uStack_12c,CONCAT22(uStack_12e,uStack_130));
          goto LAB_141e4a810;
        }
        lStack_168 = lStack_100;
        uStack_f2 = 1;
        FUN_141e3e370(&cStack_148,&pbStack_1c8);
      } while (cStack_148 == '2');
      puStack_230 = (undefined *)CONCAT62(uStack_13e,uStack_140);
      unaff_XMM6_Da = CONCAT22(uStack_136,uStack_138);
      unaff_XMM6_Dc = CONCAT22(uStack_12e,uStack_130);
      pbStack_1b8 = (byte *)CONCAT62(uStack_11e,uStack_120);
      lStack_1b0 = CONCAT62(uStack_116,uStack_118);
      uStack_1a8 = CONCAT62(uStack_10e,uStack_110);
      uStack_1a0 = CONCAT62(uStack_106,uStack_108);
      unaff_XMM10_Da = uStack_144;
      cVar7 = cStack_148;
      uVar13 = uStack_146;
      uVar3 = uStack_124;
      bVar11 = bStack_147;
      unaff_XMM6_Db = uStack_134;
      unaff_XMM6_Dd = uStack_12c;
LAB_141e4b09a:
      param_2 = (byte *)(ulonglong)uVar3;
joined_r0x000141e4b31b:
      lVar8 = lStack_160;
      bVar9 = bStack_f1;
      puVar6 = puStack_178;
      if (!SBORROW8(0,lStack_100)) {
joined_r0x000141e4b35d:
        puStack_178 = puStack_230;
        puStack_230 = puStack_178;
        lStack_160 = lVar8;
        bStack_f1 = bVar9;
        if (lStack_100 != 0) {
          func_0x000140613c20(uStack_1d8,lStack_100,1);
          puStack_230 = puStack_178;
          puVar6 = puStack_178;
        }
      }
LAB_141e4b0f9:
      puStack_178 = puVar6;
      uStack_238 = (byte *)CONCAT44(unaff_XMM10_Da,CONCAT22(uVar13,CONCAT11(bVar11,cVar7)));
      uStack_228 = CONCAT44(unaff_XMM6_Db,unaff_XMM6_Da);
      uStack_220 = CONCAT44(unaff_XMM6_Dd,unaff_XMM6_Dc);
      uStack_214 = SUB84(param_2,0);
      pbStack_210 = pbStack_1b8;
      lStack_208 = lStack_1b0;
      uVar10 = uStack_150;
      if ((bVar9 & 1) != 0) {
        uVar10 = 0;
      }
      uStack_200 = (undefined4)uStack_1a8;
      uStack_1fc = uStack_1a8._4_4_;
      uStack_1f8 = (undefined4)uStack_1a0;
      uStack_1f4 = uStack_1a0._4_4_;
      if (lVar8 == 0) {
        uVar10 = 0;
      }
      FUN_140bcd9f0(&cStack_148,&uStack_238,uVar10);
      *(undefined4 *)(param_1 + 4) = uStack_144;
      *(uint *)(param_1 + 1) = CONCAT13((undefined1)uStack_144,CONCAT21(uStack_146,bStack_147));
      *(ulonglong *)(param_1 + 0x38) = CONCAT62(uStack_10e,uStack_110);
      *(ulonglong *)(param_1 + 0x40) = CONCAT62(uStack_106,uStack_108);
      *(ulonglong *)(param_1 + 0x28) = CONCAT62(uStack_11e,uStack_120);
      *(ulonglong *)(param_1 + 0x30) = CONCAT62(uStack_116,uStack_118);
      *param_1 = cStack_148;
      *(ulonglong *)(param_1 + 8) = CONCAT62(uStack_13e,uStack_140);
      *(ulonglong *)(param_1 + 0x10) = CONCAT44(uStack_134,CONCAT22(uStack_136,uStack_138));
      *(ulonglong *)(param_1 + 0x18) = CONCAT44(uStack_12c,CONCAT22(uStack_12e,uStack_130));
      *(uint *)(param_1 + 0x20) = CONCAT22(uStack_126,uStack_128);
      *(uint *)(param_1 + 0x24) = uStack_124;
    }
    else {
      *(ulonglong *)(param_1 + 0x40) = CONCAT62(uStack_106,uStack_108);
      *(ulonglong *)(param_1 + 0x30) = CONCAT62(uStack_116,uStack_118);
      *(ulonglong *)(param_1 + 0x38) = CONCAT62(uStack_10e,uStack_110);
      *(ulonglong *)(param_1 + 0x20) = CONCAT44(uStack_124,CONCAT22(uStack_126,uStack_128));
      *(ulonglong *)(param_1 + 0x28) = CONCAT62(uStack_11e,uStack_120);
      *(ulonglong *)(param_1 + 0x10) = CONCAT44(uStack_134,CONCAT22(uStack_136,uStack_138));
      *(ulonglong *)(param_1 + 0x18) = CONCAT44(uStack_12c,CONCAT22(uStack_12e,uStack_130));
      *(ulonglong *)param_1 =
           CONCAT44(uStack_144,CONCAT22(uStack_146,CONCAT11(bStack_147,cStack_148)));
      *(ulonglong *)(param_1 + 8) = CONCAT62(uStack_13e,uStack_140);
    }
  }
  return param_1;
}

