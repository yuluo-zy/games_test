// FUN_1409d20b0  0x1409d20b0

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void FUN_1409d20b0(undefined8 param_1,longlong param_2,longlong param_3,longlong param_4,
                  longlong param_5,byte param_6,longlong param_7,longlong param_8,longlong param_9,
                  longlong param_10,longlong param_11,longlong param_12,longlong param_13)

{
  ulonglong *puVar1;
  code *pcVar2;
  ulonglong uVar3;
  int iVar4;
  undefined4 uVar5;
  longlong lVar6;
  undefined8 uVar7;
  longlong lVar8;
  undefined8 *puVar9;
  longlong *plVar10;
  ulonglong uVar11;
  ulonglong uVar12;
  uint uVar13;
  float fVar14;
  undefined1 auStack_e00 [1000];
  undefined1 auStack_a18 [184];
  undefined1 auStack_960 [38];
  char cStack_93a;
  longlong lStack_930;
  undefined1 auStack_920 [184];
  undefined1 auStack_868 [38];
  char cStack_842;
  longlong lStack_838;
  ulonglong uStack_828;
  undefined1 auStack_820 [160];
  byte bStack_780;
  byte bStack_77f;
  byte bStack_77e;
  byte bStack_77d;
  undefined4 uStack_77c;
  undefined4 uStack_778;
  undefined4 uStack_774;
  undefined4 uStack_770;
  undefined4 uStack_76c;
  undefined4 uStack_768;
  undefined4 uStack_764;
  undefined4 uStack_760;
  undefined4 uStack_75c;
  undefined4 uStack_758;
  undefined4 uStack_754;
  undefined4 uStack_750;
  undefined4 uStack_74c;
  undefined4 uStack_748;
  undefined4 uStack_744;
  undefined4 uStack_740;
  undefined4 uStack_73c;
  undefined8 uStack_738;
  ulonglong uStack_730;
  longlong alStack_728 [4];
  undefined8 uStack_708;
  undefined8 uStack_700;
  undefined8 uStack_6f8;
  undefined8 uStack_6f0;
  byte bStack_688;
  byte bStack_687;
  byte bStack_686;
  byte bStack_685;
  undefined4 uStack_684;
  undefined4 uStack_680;
  undefined4 uStack_67c;
  undefined4 uStack_678;
  undefined4 uStack_674;
  undefined4 uStack_670;
  undefined4 uStack_66c;
  undefined4 uStack_668;
  undefined4 uStack_664;
  undefined4 uStack_660;
  undefined4 uStack_65c;
  undefined4 uStack_658;
  undefined4 uStack_654;
  undefined4 uStack_650;
  undefined4 uStack_64c;
  undefined8 uStack_648;
  undefined8 uStack_640;
  longlong alStack_348 [23];
  undefined1 auStack_290 [38];
  char cStack_26a;
  longlong lStack_260;
  undefined *puStack_250;
  longlong lStack_248;
  undefined8 uStack_240;
  undefined8 uStack_238;
  undefined8 uStack_230;
  undefined8 uStack_228;
  undefined8 uStack_220;
  undefined8 uStack_218;
  undefined8 uStack_210;
  undefined4 uStack_208;
  undefined4 uStack_204;
  undefined4 uStack_200;
  undefined4 uStack_1fc;
  undefined4 uStack_1f8;
  undefined4 uStack_1f4;
  undefined4 uStack_1f0;
  undefined4 uStack_1ec;
  undefined4 uStack_1e8;
  undefined4 uStack_1e4;
  undefined4 uStack_1e0;
  undefined4 uStack_1dc;
  undefined8 uStack_1d8;
  undefined8 uStack_1d0;
  undefined8 uStack_1c8;
  undefined4 uStack_1c0;
  undefined4 uStack_1bc;
  undefined4 uStack_1b8;
  undefined4 uStack_1b4;
  undefined8 uStack_1b0;
  byte bStack_1a8;
  undefined2 uStack_1a7;
  byte bStack_1a5;
  undefined4 uStack_1a4;
  undefined1 uStack_1a0;
  undefined1 uStack_19f;
  undefined8 uStack_158;
  undefined8 uStack_150;
  undefined4 uStack_148;
  undefined4 uStack_144;
  undefined4 uStack_140;
  undefined4 uStack_13c;
  undefined4 uStack_138;
  undefined4 uStack_134;
  undefined8 uStack_130;
  undefined8 uStack_128;
  undefined8 uStack_120;
  undefined8 uStack_118;
  longlong lStack_108;
  undefined8 uStack_100;
  uint uStack_f8;
  char acStack_f4 [36];
  longlong lStack_d0;
  longlong lStack_c8;
  longlong lStack_c0;
  longlong lStack_b8;
  undefined8 uStack_b0;
  longlong lStack_a8;
  longlong lStack_a0;
  longlong lStack_98;
  uint uStack_8c;
  longlong lStack_88;
  longlong lStack_80;
  undefined1 uStack_72;
  undefined1 uStack_71;
  undefined8 uStack_70;
  
  uStack_70 = 0xfffffffffffffffe;
  lVar8 = param_2 + 0x38;
  uVar11 = 0x78cac557b9afa382;
  lStack_a8 = param_3;
  lStack_88 = param_4;
  lVar6 = FUN_1400dd6f0(lVar8,0x78cac557b9afa382,0x4199801d7a83a055);
  if (((lVar6 == 0) || (*(ulonglong *)(param_2 + 0x218) <= uVar11)) ||
     (uVar11 = *(ulonglong *)(*(longlong *)(param_2 + 0x210) + uVar11 * 8), uVar11 == 0)) {
code_r0x0001409d2f8a:
    FUN_1428d9310(&UNK_142ac1860);
code_r0x0001409d2f96:
    FUN_1428d9310(&UNK_142ac1878);
  }
  else {
    lVar6 = ~uVar11 * 0x60;
    if (*(longlong *)(*(longlong *)(param_2 + 0x1e0) + 0x20 + lVar6) == 0)
    goto code_r0x0001409d2f8a;
    lStack_98 = *(longlong *)(*(longlong *)(param_2 + 0x1e0) + lVar6 + 0x10);
    uVar11 = 0x4bc362195b80f729;
    lVar6 = FUN_1400dd6f0(lVar8,0x4bc362195b80f729,0x982798ce80cbcefb);
    if (((lVar6 == 0) || (*(ulonglong *)(param_2 + 0x218) <= uVar11)) ||
       (uVar11 = *(ulonglong *)(*(longlong *)(param_2 + 0x210) + uVar11 * 8), uVar11 == 0))
    goto code_r0x0001409d2f96;
    lVar6 = ~uVar11 * 0x60;
    if (*(longlong *)(*(longlong *)(param_2 + 0x1e0) + 0x20 + lVar6) == 0)
    goto code_r0x0001409d2f96;
    puVar1 = *(ulonglong **)(*(longlong *)(param_2 + 0x1e0) + lVar6 + 0x10);
    uVar11 = *puVar1;
    uVar3 = puVar1[1];
    lVar6 = lStack_a8 + 0x24;
    uVar7 = func_0x000140bd5790(lStack_a8 + 0x28);
    uStack_730 = uStack_730 & 0xffffffff00000000;
    alStack_728[0] = lStack_88;
    puStack_250 = (undefined *)0x1;
    uStack_240 = 0;
    lStack_248 = lVar6;
    lStack_d0 = lVar6;
    lStack_a0 = param_2;
    FUN_140867970(auStack_e00,&UNK_142ac1890,0x17,&uStack_730,1,&puStack_250,uVar7,param_2,param_1);
    uStack_71 = 1;
    uVar12 = 0x8226cc786e679355;
    lVar8 = FUN_1400dd6f0(lVar8,0x8226cc786e679355,0x65cfda6a55a841ac);
    if (((lVar8 != 0) && (uVar12 < *(ulonglong *)(lStack_a0 + 0x218))) &&
       (uVar12 = *(ulonglong *)(*(longlong *)(lStack_a0 + 0x210) + uVar12 * 8), uVar12 != 0)) {
      lVar8 = ~uVar12 * 0x60;
      if (*(longlong *)(*(longlong *)(lStack_a0 + 0x1e0) + 0x20 + lVar8) != 0) {
        uStack_71 = 1;
        puVar9 = (undefined8 *)
                 FUN_140969720(*(undefined8 *)(*(longlong *)(lStack_a0 + 0x1e0) + lVar8 + 0x10),
                               &UNK_142c2e230);
        if (puVar9 != (undefined8 *)0x0) {
          puStack_250 = &UNK_142a9d2fe;
          lStack_248 = 0x35;
          if ((puVar9[3] != 0x35) ||
             (iVar4 = func_0x000142923bf0(&UNK_142a9d2fe,puVar9[2],0x35), iVar4 != 0)) {
            uStack_730 = 0;
            uStack_71 = 1;
            FUN_1428cab41(0,&puStack_250,puVar9 + 2,&uStack_730,&UNK_142c2e180);
            goto code_r0x0001409d2fca;
          }
          uStack_71 = 1;
          puVar9 = (undefined8 *)(**(code **)(puVar9[1] + 0x18))(*puVar9);
          if (puVar9 != (undefined8 *)0x0) {
            fVar14 = (float)(int)uVar3 / _UNK_1429a7640;
            uStack_b0 = *puVar9;
            uStack_100 = 5;
            uStack_f8 = 1;
            acStack_f4[0] = '\0';
            acStack_f4[4] = '\x03';
            acStack_f4[5] = '\0';
            acStack_f4[6] = '\0';
            acStack_f4[7] = '\0';
            acStack_f4[8] = 0;
            acStack_f4[0xc] = '\0';
            acStack_f4[0xd] = '\0';
            acStack_f4[0xe] = '\0';
            acStack_f4[0xf] = '\0';
            acStack_f4[0x10] = 0;
            acStack_f4[0x14] = '\0';
            acStack_f4[0x15] = '\0';
            acStack_f4[0x16] = '\0';
            acStack_f4[0x17] = '\0';
            acStack_f4[0x18] = 1;
            acStack_f4[0x1c] = '\x02';
            acStack_f4[0x1d] = '\0';
            acStack_f4[0x1e] = '\0';
            acStack_f4[0x1f] = '\0';
            acStack_f4[0x20] = 0;
            uStack_8c = (uint)param_6;
            lStack_88 = lStack_88 + 4;
            lStack_c8 = param_9 + 0x24;
            lStack_c0 = param_9 + 0x48;
            lStack_b8 = param_13 + 0x18;
            lVar8 = 0;
            while( true ) {
              lVar6 = lVar8 + 1;
              uVar13 = *(uint *)(acStack_f4 + lVar8 * 8 + -4);
              lStack_108 = lVar6;
              if (4 < uVar13) {
                uStack_71 = 1;
                FUN_1428d9518(5,5,&UNK_142ac18d8);
                goto code_r0x0001409d2fca;
              }
              if ((*(byte *)(lStack_98 + (ulonglong)(uVar13 << 4)) & 1) == 0) break;
              uVar7 = *(undefined8 *)(lStack_98 + 8 + (ulonglong)(uVar13 << 4));
              if (acStack_f4[lVar8 * 8] == '\0') {
                uStack_71 = 1;
                uVar5 = func_0x000140be6750();
                uStack_71 = 1;
                plVar10 = (longlong *)FUN_1400d9b20(0);
                if (plVar10 == (longlong *)0x0) {
                  uStack_71 = 1;
                  FUN_1428d80d0(&UNK_142c3aaa0);
                  goto code_r0x0001409d2fca;
                }
                uStack_1c0 = (undefined4)*plVar10;
                uStack_1bc = *(undefined4 *)((longlong)plVar10 + 4);
                uStack_1b8 = (undefined4)plVar10[1];
                uStack_1b4 = *(undefined4 *)((longlong)plVar10 + 0xc);
                *plVar10 = *plVar10 + 1;
                bStack_1a5 = (byte)((uint)uVar5 >> 0x18);
              }
              else {
                uStack_71 = 1;
                uVar5 = func_0x000140be6750();
                uStack_71 = 1;
                plVar10 = (longlong *)FUN_1400d9b20(0);
                if (plVar10 == (longlong *)0x0) {
                  uStack_71 = 1;
                  FUN_1428d80d0(&UNK_142c3aaa0);
                  goto code_r0x0001409d2fca;
                }
                uStack_1c0 = (undefined4)*plVar10;
                uStack_1bc = *(undefined4 *)((longlong)plVar10 + 4);
                uStack_1b8 = (undefined4)plVar10[1];
                uStack_1b4 = *(undefined4 *)((longlong)plVar10 + 0xc);
                *plVar10 = *plVar10 + 1;
                uVar13 = (uint)(byte)((byte)uVar13 | 0x80);
                bStack_1a5 = (byte)((uint)uVar5 >> 0x18) & 0x9f | 0x20;
              }
              bStack_1a8 = (byte)uVar5 & 0xbf;
              puStack_250 = (undefined *)0x0;
              uStack_240 = 0;
              uStack_238 = 4;
              uStack_230 = 0;
              uStack_228 = 0;
              uStack_220 = 1;
              uStack_218 = 0;
              uStack_210 = 0x100000003;
              uStack_1c8 = _UNK_142a96738;
              uStack_158._0_4_ = (undefined4)uStack_730;
              uStack_158._4_4_ = (undefined4)(uStack_730 >> 0x20);
              uStack_150._0_4_ = (undefined4)alStack_728[0];
              uStack_150._4_4_ = (undefined4)((ulonglong)alStack_728[0] >> 0x20);
              uStack_148 = (undefined4)alStack_728[1];
              uStack_144 = (undefined4)((ulonglong)alStack_728[1] >> 0x20);
              uStack_140 = (undefined4)alStack_728[2];
              uStack_13c = (undefined4)((ulonglong)alStack_728[2] >> 0x20);
              uStack_138 = (undefined4)alStack_728[3];
              uStack_134 = (undefined4)((ulonglong)alStack_728[3] >> 0x20);
              uStack_130._0_4_ = (undefined4)_UNK_142a96720;
              uStack_130._4_4_ = (undefined4)((ulonglong)_UNK_142a96720 >> 0x20);
              uStack_1d8 = _UNK_142a96728;
              uStack_1d0 = _UNK_142a96730;
              uStack_1e8 = uStack_138;
              uStack_1e4 = uStack_134;
              uStack_1e0 = (undefined4)uStack_130;
              uStack_1dc = uStack_130._4_4_;
              uStack_1f8 = uStack_148;
              uStack_1f4 = uStack_144;
              uStack_1f0 = uStack_140;
              uStack_1ec = uStack_13c;
              uStack_208 = (undefined4)uStack_158;
              uStack_204 = uStack_158._4_4_;
              uStack_200 = (undefined4)uStack_150;
              uStack_1fc = uStack_150._4_4_;
              uStack_1a7 = (undefined2)((uint)uVar5 >> 8);
              uStack_1a4 = 0x3f800000;
              uStack_1a0 = 0;
              uStack_19f = (undefined1)uVar13;
              uStack_71 = 1;
              uStack_708 = _UNK_142a96720;
              uStack_700 = _UNK_142a96728;
              uStack_6f8 = _UNK_142a96730;
              uStack_6f0 = _UNK_142a96738;
              uStack_1b0 = uVar7;
              uStack_158 = uStack_730;
              uStack_150 = alStack_728[0];
              uStack_130 = _UNK_142a96720;
              uStack_128 = _UNK_142a96728;
              uStack_120 = _UNK_142a96730;
              uStack_118 = _UNK_142a96738;
              FUN_140905be0(&uStack_730,&puStack_250,auStack_e00,0);
              if (uStack_654._2_1_ != '\b') {
                FUN_14096b570(&uStack_678,uStack_648 + 0xe8,0x578883943452a027,uStack_8c);
              }
              func_0x000142923c00(&puStack_250,&uStack_730,0xf8);
              lStack_80 = lStack_a8;
              uStack_730._0_4_ = 2;
              FUN_1408ee800(&lStack_80,&puStack_250,0x41aa4a0d2dea0b36,0x1d,&uStack_730);
              func_0x000142923c00(&uStack_730,&puStack_250,0xf8);
              lStack_80 = lStack_d0;
              puStack_250._0_4_ = 2;
              FUN_1408ee800(&lStack_80,&uStack_730,0x4861579da049fbfc,0x1d,&puStack_250);
              func_0x000142923c00(&puStack_250,&uStack_730,0xf8);
              lStack_80 = param_8;
              uStack_730._0_4_ = 2;
              FUN_1408ee800(&lStack_80,&puStack_250,0x2a333c3a28028230,0x1d,&uStack_730);
              func_0x000142923c00(&uStack_730,&puStack_250,0xf8);
              lStack_80 = param_5;
              puStack_250._0_4_ = 2;
              FUN_1408ee800(&lStack_80,&uStack_730,0x4175185826c09d0,0x1d,&puStack_250);
              func_0x000142923c00(&puStack_250,&uStack_730,0xf8);
              lStack_80 = 0x579f39ab91a01865;
              uStack_730._0_4_ = 2;
              FUN_140908610(&lStack_80,&puStack_250,0x815bf5036966978c,0x1d,&uStack_730);
              func_0x000142923c00(&uStack_730,&puStack_250,0xf8);
              lStack_80 = param_7;
              puStack_250._0_4_ = 2;
              FUN_1408ee800(&lStack_80,&uStack_730,0xe003e2bb64399af9,0x1d,&puStack_250);
              func_0x000142923c00(&puStack_250,&uStack_730,0xf8);
              lStack_80._0_1_ = 8;
              uStack_730 = CONCAT44(uStack_730._4_4_,2);
              FUN_140908c50(&lStack_80,&puStack_250,0x77160113ac8dca3f,0x1d,&uStack_730);
              func_0x000142923c00(&uStack_730,&puStack_250,0xf8);
              lStack_80 = CONCAT71(lStack_80._1_7_,0xc);
              puStack_250 = (undefined *)CONCAT44(puStack_250._4_4_,2);
              FUN_140908c50(&lStack_80,&uStack_730,0x8f870a5c7060b0f4,0x1d,&puStack_250);
              func_0x000142923c00(alStack_348,&uStack_730,0xf8);
              if (cStack_26a != '\b') {
                FUN_14096b850(auStack_290,lStack_260 + 0xe8,0x418a98441f59a54c,1);
              }
              func_0x000142923c00(&uStack_730,alStack_348,0xf8);
              lStack_80 = CONCAT71(lStack_80._1_7_,0x11);
              puStack_250 = (undefined *)CONCAT44(puStack_250._4_4_,2);
              FUN_140908c50(&lStack_80,&uStack_730,0x11651f4dc9841aa4,0x1d,&puStack_250);
              func_0x000142923c00(&puStack_250,&uStack_730,0xf8);
              alStack_348[0] = param_11;
              uStack_730 = CONCAT44(uStack_730._4_4_,2);
              FUN_1408ee800(alStack_348,&puStack_250,0x9941437e56d66d25,0x1d,&uStack_730);
              func_0x000142923c00(&uStack_730,&puStack_250,0xf8);
              uStack_71 = 1;
              FUN_141629a20(auStack_920,&uStack_730,0x1e5485ef2e9f9384,fVar14 + (float)uVar11);
              if (cStack_842 != '\b') {
                FUN_14096b570(auStack_868,lStack_838 + 0xe8,0xc5ab673fb08651f5,0);
              }
              func_0x000142923c00(&uStack_730,auStack_920,0xf8);
              uStack_72 = 1;
              uVar7 = func_0x000140bd5790(lStack_88);
              uStack_72 = 0;
              FUN_141629d20(auStack_a18,&uStack_730,0x531bb9318eea4971,uVar7);
              if (cStack_93a != '\b') {
                FUN_14096b570(auStack_960,lStack_930 + 0xe8,0x9b88329e408ff69f,uStack_b0);
              }
              func_0x000142923c00(&uStack_730,auStack_a18,0xf8);
              alStack_348[0] = param_10;
              puStack_250._0_4_ = 2;
              FUN_1408ee800(alStack_348,&uStack_730,0xba3e6caf025774d7,0x1d,&puStack_250);
              func_0x000142923c00(&puStack_250,&uStack_730,0xf8);
              alStack_348[0] = param_9;
              uStack_730._0_4_ = 2;
              FUN_1408ee800(alStack_348,&puStack_250,0xfe8da59f97749daa,0x1d,&uStack_730);
              func_0x000142923c00(&uStack_730,&puStack_250,0xf8);
              alStack_348[0] = lStack_c8;
              puStack_250._0_4_ = 2;
              FUN_1408ee800(alStack_348,&uStack_730,0xd36dd992ba29df1,0x1d,&puStack_250);
              func_0x000142923c00(&puStack_250,&uStack_730,0xf8);
              alStack_348[0] = lStack_c0;
              uStack_730._0_4_ = 2;
              FUN_1408ee800(alStack_348,&puStack_250,0x4b1b8bd236f14df0,0x1d,&uStack_730);
              func_0x000142923c00(&uStack_730,&puStack_250,0xf8);
              uStack_71 = 1;
              FUN_141630a90(&puStack_250,&uStack_730,0xbc5ad80c0ba915c3);
              uStack_71 = 1;
              FUN_14162a790(&uStack_730,&puStack_250,0xc6b33eb20992aad);
              alStack_348[0] = param_12;
              puStack_250 = (undefined *)CONCAT44(puStack_250._4_4_,2);
              FUN_1408ee800(alStack_348,&uStack_730,0x5a67347026116ffe,0x1d,&puStack_250);
              func_0x000142923c00(&puStack_250,&uStack_730,0xf8);
              uStack_71 = 1;
              FUN_140906150(&uStack_730,&puStack_250,0x2bd0ac289087c8e1,param_13);
              uStack_71 = 1;
              FUN_140906150(&puStack_250,&uStack_730,0x129f01ce8d5406a0,lStack_b8);
              uStack_71 = 1;
              FUN_1416286f0(&uStack_730,&puStack_250);
              uStack_71 = 1;
              FUN_1416289d0(&puStack_250,&uStack_730,lStack_a0);
              alStack_348[0] = CONCAT71(alStack_348[0]._1_7_,9);
              uStack_730._0_4_ = 2;
              FUN_140908c50(alStack_348,&puStack_250,0x61b08b5e0470c5f2,0x1d,&uStack_730);
              func_0x000142923c00(&uStack_730,&puStack_250,0xf8);
              auStack_920[0] = 10;
              alStack_348[0] = CONCAT44(alStack_348[0]._4_4_,2);
              FUN_140908c50(auStack_920,&uStack_730,0x297d1e4cdae8f657,0x1d,alStack_348);
              func_0x000142923c00(&uStack_828,&uStack_730,0xf8);
              puStack_250 = (undefined *)((ulonglong)puStack_250 & 0xffffffffffffff00);
              uStack_730 = CONCAT44(uStack_730._4_4_,2);
              FUN_140908c50(&puStack_250,&uStack_828,0xb79d1a8a6a254145,0x1d,&uStack_730);
              uVar3 = uStack_828;
              func_0x000142923c00(alStack_728,auStack_820,0xa0);
              uStack_640 = uStack_738;
              uStack_654 = uStack_74c;
              uStack_650 = uStack_748;
              uStack_64c = uStack_744;
              uStack_648 = CONCAT44(uStack_73c,uStack_740);
              uStack_664 = uStack_75c;
              uStack_660 = uStack_758;
              uStack_65c = uStack_754;
              uStack_658 = uStack_750;
              uStack_674 = uStack_76c;
              uStack_670 = uStack_768;
              uStack_66c = uStack_764;
              uStack_668 = uStack_760;
              uStack_684 = uStack_77c;
              uStack_680 = uStack_778;
              uStack_67c = uStack_774;
              uStack_678 = uStack_770;
              bStack_688 = bStack_780 & 0x7f;
              bStack_687 = bStack_77f | 1;
              bStack_686 = bStack_77e & 3 | 0xe8;
              bStack_685 = bStack_77d & 0xe0 | 0x10;
              uStack_730 = uVar3;
              uStack_71 = 1;
              FUN_140906a90(&uStack_730,3);
              lVar8 = lVar6;
              if (lVar6 == 5) {
                func_0x000142923c00(&uStack_730,auStack_e00,1000);
                uStack_71 = 0;
                FUN_140868370(&uStack_730);
                return;
              }
            }
            uStack_71 = 1;
            FUN_1428d9310(&UNK_142ac18f0);
            goto code_r0x0001409d2fca;
          }
        }
        uStack_71 = 1;
        FUN_1428d9310(&UNK_142ac18c0);
        goto code_r0x0001409d2fca;
      }
    }
  }
  uStack_71 = 1;
  FUN_1428d9310(&UNK_142ac18a8);
code_r0x0001409d2fca:
                    /* WARNING: Does not return */
  pcVar2 = (code *)invalidInstructionException();
  (*pcVar2)();
}

