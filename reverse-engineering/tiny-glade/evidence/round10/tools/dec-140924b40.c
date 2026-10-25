
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void FUN_140924b40(undefined8 *param_1,undefined8 *param_2,undefined8 *param_3,undefined8 *param_4,
                  undefined8 *param_5,undefined8 param_6)

{
  undefined4 *puVar1;
  undefined8 uVar2;
  undefined8 uVar3;
  longlong ***ppplVar4;
  longlong ***ppplVar5;
  longlong ***ppplVar6;
  code *pcVar7;
  undefined4 uVar8;
  undefined4 uVar9;
  undefined4 uVar10;
  undefined *puVar11;
  undefined8 uVar12;
  undefined8 uVar13;
  undefined8 uVar14;
  undefined8 uVar15;
  ulonglong uVar16;
  undefined8 uVar17;
  undefined *puVar18;
  longlong ****pppplVar19;
  longlong ****pppplVar20;
  longlong lVar21;
  undefined4 uStack_298;
  undefined4 uStack_294;
  undefined4 uStack_290;
  undefined4 uStack_28c;
  undefined8 uStack_288;
  undefined8 uStack_280;
  undefined8 uStack_278;
  undefined8 uStack_270;
  undefined8 uStack_268;
  undefined8 uStack_260;
  longlong ***ppplStack_258;
  undefined4 uStack_250;
  undefined4 uStack_24c;
  undefined4 uStack_248;
  undefined4 uStack_244;
  undefined4 uStack_240;
  undefined4 uStack_23c;
  undefined4 uStack_238;
  undefined4 uStack_234;
  undefined4 uStack_230;
  undefined4 uStack_22c;
  undefined4 uStack_228;
  undefined4 uStack_224;
  undefined4 uStack_220;
  undefined4 uStack_21c;
  undefined *puStack_218;
  undefined8 uStack_210;
  undefined *puStack_208;
  undefined8 uStack_200;
  undefined *puStack_1f8;
  undefined8 uStack_1f0;
  undefined *puStack_1e8;
  longlong ***ppplStack_1d8;
  undefined *puStack_1d0;
  undefined8 uStack_1c8;
  undefined8 uStack_1c0;
  undefined8 uStack_1b8;
  undefined8 uStack_1b0;
  undefined8 uStack_1a8;
  undefined8 uStack_1a0;
  undefined8 uStack_198;
  undefined8 uStack_190;
  longlong ***ppplStack_188;
  longlong ***ppplStack_180;
  undefined *puStack_178;
  undefined8 uStack_170;
  undefined8 uStack_168;
  undefined8 uStack_160;
  undefined *puStack_158;
  undefined8 uStack_150;
  undefined *puStack_148;
  undefined8 uStack_140;
  undefined *puStack_138;
  undefined8 uStack_130;
  undefined *puStack_128;
  undefined8 uStack_120;
  undefined *puStack_118;
  undefined8 uStack_110;
  undefined *puStack_108;
  undefined8 uStack_100;
  undefined8 uStack_f8;
  undefined8 uStack_f0;
  longlong ***ppplStack_e8;
  undefined8 uStack_e0;
  undefined8 uStack_d8;
  undefined8 uStack_d0;
  undefined8 uStack_c8;
  undefined4 uStack_c0;
  undefined4 uStack_bc;
  undefined4 uStack_b8;
  undefined4 uStack_b4;
  undefined4 uStack_b0;
  undefined4 uStack_ac;
  undefined *puStack_a8;
  undefined8 uStack_a0;
  undefined *puStack_98;
  undefined8 uStack_90;
  undefined *puStack_88;
  undefined8 uStack_80;
  undefined *puStack_78;
  undefined8 uStack_70;
  
  uStack_70 = 0xfffffffffffffffe;
  uVar2 = *param_4;
  uStack_1c8 = FUN_140b3fe40(*(longlong *)*param_1 + 0x10,&UNK_142aae9e8,0x20,&UNK_142aaea08,0x1a,
                             *param_5,uVar2);
  FUN_140b0eeb0(&uStack_198,&UNK_142aaea22,0x1e);
  pppplVar19 = _UNK_14292e6b0;
  if (SBORROW8(0,(longlong)uStack_198)) {
    ppplStack_e8 = (longlong ***)uStack_190;
    FUN_1428d9760(&UNK_142aae510,0x2b,&ppplStack_e8,&UNK_142aae4f0,&UNK_142aaea78);
  }
  else {
    puStack_78 = puStack_128;
    puStack_88 = puStack_138;
    uStack_80 = uStack_130;
    puStack_98 = puStack_148;
    uStack_90 = uStack_140;
    puStack_a8 = puStack_158;
    uStack_a0 = uStack_150;
    uStack_b8 = (undefined4)uStack_168;
    uStack_b4 = uStack_168._4_4_;
    uStack_b0 = (undefined4)uStack_160;
    uStack_ac = uStack_160._4_4_;
    uStack_c8 = puStack_178;
    uStack_c0 = (undefined4)uStack_170;
    uStack_bc = uStack_170._4_4_;
    uStack_d8 = (longlong ****)ppplStack_188;
    uStack_d0 = (longlong ****)ppplStack_180;
    uStack_e0 = uStack_190;
    ppplStack_258 = (longlong ***)_UNK_14292e6b0;
    uStack_250 = 0x3f800000;
    FUN_140a07680(&uStack_198,&ppplStack_e8,&ppplStack_258);
    uVar17 = *param_3;
    uVar3 = *param_2;
    uVar12 = FUN_140b1aca0(&uStack_198,&UNK_142aaea90,0xb,uVar17,uVar3);
    FUN_140b0eeb0(&uStack_198,&UNK_142aaea9b,0x1f);
    if (SBORROW8(0,(longlong)uStack_198)) {
      ppplStack_e8 = (longlong ***)uStack_190;
      FUN_1428d9760(&UNK_142aae510,0x2b,&ppplStack_e8,&UNK_142aae4f0,&UNK_142aaeac0);
    }
    else {
      puStack_78 = puStack_128;
      puStack_88 = puStack_138;
      uStack_80 = uStack_130;
      puStack_98 = puStack_148;
      uStack_90 = uStack_140;
      puStack_a8 = puStack_158;
      uStack_a0 = uStack_150;
      uStack_b8 = (undefined4)uStack_168;
      uStack_b4 = uStack_168._4_4_;
      uStack_b0 = (undefined4)uStack_160;
      uStack_ac = uStack_160._4_4_;
      uStack_c8 = puStack_178;
      uStack_c0 = (undefined4)uStack_170;
      uStack_bc = uStack_170._4_4_;
      uStack_d8 = (longlong ****)ppplStack_188;
      uStack_d0 = (longlong ****)ppplStack_180;
      uStack_e0 = uStack_190;
      ppplStack_258 = (longlong ***)pppplVar19;
      uStack_250 = 0x3f800000;
      FUN_140a07680(&uStack_198,&ppplStack_e8,&ppplStack_258);
      uStack_288 = FUN_140b1aca0(&uStack_198,&UNK_142aaead8,0xc,uVar17,uVar3);
      FUN_140b0eeb0(&uStack_198,&UNK_142aaeae4,0x25);
      if (SBORROW8(0,(longlong)uStack_198)) {
        ppplStack_e8 = (longlong ***)uStack_190;
        FUN_1428d9760(&UNK_142aae510,0x2b,&ppplStack_e8,&UNK_142aae4f0,&UNK_142aaeb10);
      }
      else {
        puStack_78 = puStack_128;
        puStack_88 = puStack_138;
        uStack_80 = uStack_130;
        puStack_98 = puStack_148;
        uStack_90 = uStack_140;
        puStack_a8 = puStack_158;
        uStack_a0 = uStack_150;
        uStack_b8 = (undefined4)uStack_168;
        uStack_b4 = uStack_168._4_4_;
        uStack_b0 = (undefined4)uStack_160;
        uStack_ac = uStack_160._4_4_;
        uStack_c8 = puStack_178;
        uStack_c0 = (undefined4)uStack_170;
        uStack_bc = uStack_170._4_4_;
        uStack_d8 = (longlong ****)ppplStack_188;
        uStack_d0 = (longlong ****)ppplStack_180;
        uStack_e0 = uStack_190;
        uVar13 = FUN_140b1aca0(&ppplStack_e8,&UNK_142aaeb28,0x12,uVar17,uVar3);
        FUN_140b0eeb0(&uStack_198,&UNK_142aaeb3a,0x28);
        if (SBORROW8(0,(longlong)uStack_198)) {
          ppplStack_e8 = (longlong ***)uStack_190;
          FUN_1428d9760(&UNK_142aae510,0x2b,&ppplStack_e8,&UNK_142aae4f0,&UNK_142aaeb68);
        }
        else {
          puStack_78 = puStack_128;
          puStack_88 = puStack_138;
          uStack_80 = uStack_130;
          puStack_98 = puStack_148;
          uStack_90 = uStack_140;
          puStack_a8 = puStack_158;
          uStack_a0 = uStack_150;
          uStack_b8 = (undefined4)uStack_168;
          uStack_b4 = uStack_168._4_4_;
          uStack_b0 = (undefined4)uStack_160;
          uStack_ac = uStack_160._4_4_;
          uStack_c8 = puStack_178;
          uStack_c0 = (undefined4)uStack_170;
          uStack_bc = uStack_170._4_4_;
          uStack_d8 = (longlong ****)ppplStack_188;
          uStack_d0 = (longlong ****)ppplStack_180;
          ppplStack_e8 = uStack_198;
          uStack_e0 = uStack_190;
          ppplStack_258 = (longlong ***)(ulonglong)_UNK_142925870;
          uStack_250 = 0;
          FUN_140a07680(&uStack_198,&ppplStack_e8,&ppplStack_258);
          uStack_1a0 = FUN_140b1aca0(&uStack_198,&UNK_142aaeb80,0x15,uVar17,uVar3);
          FUN_140b0eeb0(&uStack_198,&UNK_142aaeb95,0x23);
          if (SBORROW8(0,(longlong)uStack_198)) {
            ppplStack_e8 = (longlong ***)uStack_190;
            FUN_1428d9760(&UNK_142aae510,0x2b,&ppplStack_e8,&UNK_142aae4f0,&UNK_142aaebb8);
          }
          else {
            puStack_78 = puStack_128;
            puStack_88 = puStack_138;
            uStack_80 = uStack_130;
            puStack_98 = puStack_148;
            uStack_90 = uStack_140;
            puStack_a8 = puStack_158;
            uStack_a0 = uStack_150;
            uStack_b8 = (undefined4)uStack_168;
            uStack_b4 = uStack_168._4_4_;
            uStack_b0 = (undefined4)uStack_160;
            uStack_ac = uStack_160._4_4_;
            uStack_c8 = puStack_178;
            uStack_c0 = (undefined4)uStack_170;
            uStack_bc = uStack_170._4_4_;
            uStack_d8 = (longlong ****)ppplStack_188;
            uStack_d0 = (longlong ****)ppplStack_180;
            uStack_e0 = uStack_190;
            ppplStack_258 = (longlong ***)pppplVar19;
            uStack_250 = 0x3f800000;
            FUN_140a07680(&uStack_198,&ppplStack_e8,&ppplStack_258);
            uVar14 = FUN_140b1aca0(&uStack_198,&UNK_142aaebd0,0x10,uVar17,uVar3);
            FUN_140b0eeb0(&uStack_198,&UNK_142aaebe0,0x21);
            if (SBORROW8(0,(longlong)uStack_198)) {
              ppplStack_e8 = (longlong ***)uStack_190;
              FUN_1428d9760(&UNK_142aae510,0x2b,&ppplStack_e8,&UNK_142aae4f0,&UNK_142aaec08);
            }
            else {
              puStack_78 = puStack_128;
              puStack_88 = puStack_138;
              uStack_80 = uStack_130;
              puStack_98 = puStack_148;
              uStack_90 = uStack_140;
              puStack_a8 = puStack_158;
              uStack_a0 = uStack_150;
              uStack_b8 = (undefined4)uStack_168;
              uStack_b4 = uStack_168._4_4_;
              uStack_b0 = (undefined4)uStack_160;
              uStack_ac = uStack_160._4_4_;
              uStack_c8 = puStack_178;
              uStack_c0 = (undefined4)uStack_170;
              uStack_bc = uStack_170._4_4_;
              uStack_d8 = (longlong ****)ppplStack_188;
              uStack_d0 = (longlong ****)ppplStack_180;
              uStack_e0 = uStack_190;
              ppplStack_258 = (longlong ***)pppplVar19;
              uStack_250 = 0x3f800000;
              FUN_140a07680(&uStack_198,&ppplStack_e8,&ppplStack_258);
              uVar15 = FUN_140b1aca0(&uStack_198,&UNK_142aaec20,0x12,uVar17,uVar3);
              FUN_140b0eeb0(&uStack_198,&UNK_142aaec32,0x21);
              if (SBORROW8(0,(longlong)uStack_198)) {
                ppplStack_e8 = (longlong ***)uStack_190;
                FUN_1428d9760(&UNK_142aae510,0x2b,&ppplStack_e8,&UNK_142aae4f0,&UNK_142aaec58);
              }
              else {
                puStack_78 = puStack_128;
                puStack_88 = puStack_138;
                uStack_80 = uStack_130;
                puStack_98 = puStack_148;
                uStack_90 = uStack_140;
                puStack_a8 = puStack_158;
                uStack_a0 = uStack_150;
                uStack_b8 = (undefined4)uStack_168;
                uStack_b4 = uStack_168._4_4_;
                uStack_b0 = (undefined4)uStack_160;
                uStack_ac = uStack_160._4_4_;
                uStack_c8 = puStack_178;
                uStack_c0 = (undefined4)uStack_170;
                uStack_bc = uStack_170._4_4_;
                uStack_d8 = (longlong ****)ppplStack_188;
                uStack_d0 = (longlong ****)ppplStack_180;
                uStack_e0 = uStack_190;
                ppplStack_258 = (longlong ***)pppplVar19;
                uStack_250 = 0x3f800000;
                uStack_280 = uVar15;
                uStack_270 = uVar14;
                uStack_268 = uVar13;
                uStack_260 = uVar12;
                uStack_1b8 = uVar2;
                FUN_140a07680(&uStack_198,&ppplStack_e8,&ppplStack_258);
                uStack_278 = FUN_140b1aca0(&uStack_198,&UNK_142aaec70,0x14,uVar17,uVar3);
                FUN_140b0eeb0(&uStack_198,&UNK_142aaec84,0x21);
                if (SBORROW8(0,(longlong)uStack_198)) {
                  ppplStack_e8 = (longlong ***)uStack_190;
                  FUN_1428d9760(&UNK_142aae510,0x2b,&ppplStack_e8,&UNK_142aae4f0,&UNK_142aaeca8);
                }
                else {
                  puStack_78 = puStack_128;
                  puStack_88 = puStack_138;
                  uStack_80 = uStack_130;
                  puStack_98 = puStack_148;
                  uStack_90 = uStack_140;
                  puStack_a8 = puStack_158;
                  uStack_a0 = uStack_150;
                  uStack_b8 = (undefined4)uStack_168;
                  uStack_b4 = uStack_168._4_4_;
                  uStack_b0 = (undefined4)uStack_160;
                  uStack_ac = uStack_160._4_4_;
                  uStack_c0 = (undefined4)uStack_170;
                  uStack_bc = uStack_170._4_4_;
                  ppplStack_258 = (longlong ***)pppplVar19;
                  uStack_250 = 0x3f800000;
                  FUN_140a07680(&uStack_198,&ppplStack_e8,&ppplStack_258);
                  uStack_1b0 = uVar17;
                  uStack_1a8 = uVar3;
                  uStack_1c0 = FUN_140b1aca0(&uStack_198,&UNK_142aaecc0,0xc,uVar17,uVar3);
                  uStack_198 = (longlong ***)&UNK_142aaeccc;
                  uStack_190 = (longlong ****)0x1a;
                  ppplStack_188 = (longlong ***)&UNK_142aaece6;
                  ppplStack_180 = (longlong ***)0x1e;
                  puStack_178 = &UNK_142aaed04;
                  uStack_170 = 0x18;
                  uStack_168 = &UNK_142aaed1c;
                  uStack_160 = 10;
                  puStack_158 = &UNK_142aaed26;
                  uStack_150 = 0x12;
                  puStack_148 = &UNK_142aaed38;
                  uStack_140 = 0xc;
                  puStack_138 = &UNK_142aaed44;
                  uStack_130 = 0x14;
                  puStack_128 = &UNK_142aaed58;
                  uStack_120 = 0x1d;
                  puStack_118 = &UNK_142aaed75;
                  uStack_110 = 10;
                  puStack_108 = &UNK_142aaed7f;
                  uStack_100 = 0x12;
                  uStack_f8 = 0;
                  uStack_f0 = 10;
                  lVar21 = 8;
                  do {
                    uStack_298 = *(undefined4 *)((longlong)&uStack_1a0 + lVar21);
                    uStack_294 = *(undefined4 *)((longlong)&uStack_1a0 + lVar21 + 4);
                    uStack_290 = *(undefined4 *)((longlong)&uStack_198 + lVar21);
                    uStack_28c = *(undefined4 *)((longlong)&uStack_198 + lVar21 + 4);
                    ppplStack_1d8 = (longlong ***)&uStack_298;
                    puStack_1d0 = &UNK_140109e90;
                    ppplStack_e8 = (longlong ***)&UNK_142aaeef0;
                    uStack_e0 = (longlong ****)0x2;
                    uStack_d0 = (longlong ****)0x1;
                    uStack_c8 = (undefined *)0x0;
                    uStack_d8 = &ppplStack_1d8;
                    FUN_140d79f90(&ppplStack_258,&ppplStack_e8);
                    FUN_140b10ed0(&ppplStack_e8,&ppplStack_258);
                    if (SBORROW8(0,(longlong)ppplStack_e8)) {
                      ppplStack_258 = (longlong ***)uStack_e0;
                      FUN_1428d9760(&UNK_142aae510,0x2b,&ppplStack_258,&UNK_142aae4f0,&UNK_142aaef10
                                   );
                      goto code_r0x0001409267b9;
                    }
                    puStack_1e8 = puStack_78;
                    puStack_1f8 = puStack_88;
                    uStack_1f0 = uStack_80;
                    puStack_208 = puStack_98;
                    uStack_200 = uStack_90;
                    puStack_218 = puStack_a8;
                    uStack_210 = uStack_a0;
                    uStack_228 = uStack_b8;
                    uStack_224 = uStack_b4;
                    uStack_220 = uStack_b0;
                    uStack_21c = uStack_ac;
                    uStack_238 = (undefined4)uStack_c8;
                    uStack_234 = uStack_c8._4_4_;
                    uStack_230 = uStack_c0;
                    uStack_22c = uStack_bc;
                    uStack_248 = (undefined4)uStack_d8;
                    uStack_244 = uStack_d8._4_4_;
                    uStack_240 = (undefined4)uStack_d0;
                    uStack_23c = uStack_d0._4_4_;
                    ppplStack_258 = ppplStack_e8;
                    uStack_250 = (undefined4)uStack_e0;
                    uStack_24c = uStack_e0._4_4_;
                    ppplStack_1d8 = (longlong ***)pppplVar19;
                    puStack_1d0 = (undefined *)CONCAT44(puStack_1d0._4_4_,0x3f800000);
                    FUN_140a07680(&ppplStack_e8,&ppplStack_258,&ppplStack_1d8);
                    FUN_140b1aca0(&ppplStack_e8,CONCAT44(uStack_294,uStack_298),
                                  CONCAT44(uStack_28c,uStack_290),uStack_1b0,uStack_1a8);
                    lVar21 = lVar21 + 0x10;
                  } while (lVar21 != 0xa8);
                  puVar18 = &UNK_142aaed91;
                  uVar16 = FUN_140937aa0(uStack_1b8,&UNK_142aaed91,0x13);
                  if ((uVar16 & 1) == 0) {
                    FUN_1428d9310(&UNK_142aaeda8);
                  }
                  else {
                    FUN_140d146a0(&uStack_198,param_6);
                    puVar11 = puStack_178;
                    uVar17 = uStack_1a8;
                    uVar2 = uStack_288;
                    pppplVar20 = uStack_190;
                    if (uStack_190 == (longlong ****)0x0) {
                      pppplVar20 = (longlong ****)ppplStack_188;
                    }
                    ppplVar4 = pppplVar20[2];
                    if ((ulonglong)((longlong)*pppplVar20 - (longlong)ppplVar4) < 0x88) {
                      FUN_1428a9a60(pppplVar20,ppplVar4,0x88,1,1);
                    }
                    ppplVar5 = pppplVar20[1];
                    *(code **)((longlong)ppplVar5 + (longlong)ppplVar4) = FUN_1412d7ae0;
                    uVar10 = _UNK_14293e0cc;
                    uVar9 = _UNK_14293e0c8;
                    uVar8 = _UNK_14293e0c4;
                    puVar1 = (undefined4 *)((longlong)(ppplVar5 + 1) + (longlong)ppplVar4);
                    *puVar1 = _UNK_14293e0c0;
                    puVar1[1] = uVar8;
                    puVar1[2] = uVar9;
                    puVar1[3] = uVar10;
                    *(undefined8 *)((longlong)(ppplVar5 + 3) + (longlong)ppplVar4) = 0;
                    *(undefined4 *)((longlong)(ppplVar5 + 4) + (longlong)ppplVar4) = 0;
                    *(undefined8 *)((longlong)ppplVar5 + 0x24 + (longlong)ppplVar4) =
                         0x3f8000003f800000;
                    *(undefined4 *)((longlong)ppplVar5 + 0x2c + (longlong)ppplVar4) = 0x3f800000;
                    *(undefined8 *)((longlong)(ppplVar5 + 7) + (longlong)ppplVar4) = uStack_260;
                    *(undefined **)((longlong)(ppplVar5 + 8) + (longlong)ppplVar4) = puVar18;
                    *(undefined2 *)((longlong)(ppplVar5 + 9) + (longlong)ppplVar4) = 1;
                    *(undefined1 *)((longlong)(ppplVar5 + 0xb) + (longlong)ppplVar4) = 0;
                    *(undefined **)((longlong)(ppplVar5 + 0xd) + (longlong)ppplVar4) = puVar11;
                    *(code **)((longlong)(ppplVar5 + 0xf) + (longlong)ppplVar4) = FUN_141564d30;
                    pppplVar20[2] = ppplVar4 + 0x11;
                    FUN_140d146a0(&uStack_198,param_6);
                    puVar11 = puStack_178;
                    pppplVar20 = uStack_190;
                    if (uStack_190 == (longlong ****)0x0) {
                      pppplVar20 = (longlong ****)ppplStack_188;
                    }
                    ppplVar4 = pppplVar20[2];
                    if ((ulonglong)((longlong)*pppplVar20 - (longlong)ppplVar4) < 0x38) {
                      FUN_1428a9a60(pppplVar20,ppplVar4,0x38,1,1);
                    }
                    ppplVar5 = pppplVar20[1];
                    *(code **)((longlong)ppplVar5 + (longlong)ppplVar4) = FUN_1412d7090;
                    *(code **)((longlong)(ppplVar5 + 1) + (longlong)ppplVar4) = FUN_141564d30;
                    *(undefined **)((longlong)(ppplVar5 + 2) + (longlong)ppplVar4) = puVar11;
                    *(undefined8 *)((longlong)(ppplVar5 + 3) + (longlong)ppplVar4) = uVar2;
                    *(undefined **)((longlong)(ppplVar5 + 4) + (longlong)ppplVar4) = puVar18;
                    *(undefined2 *)((longlong)(ppplVar5 + 5) + (longlong)ppplVar4) = 0x300;
                    *(undefined1 *)((longlong)(ppplVar5 + 6) + (longlong)ppplVar4) = 0;
                    pppplVar20[2] = ppplVar4 + 7;
                    FUN_140d146a0(&uStack_198,param_6);
                    puVar11 = puStack_178;
                    pppplVar20 = uStack_190;
                    if (uStack_190 == (longlong ****)0x0) {
                      pppplVar20 = (longlong ****)ppplStack_188;
                    }
                    ppplVar4 = pppplVar20[2];
                    if ((ulonglong)((longlong)*pppplVar20 - (longlong)ppplVar4) < 0x38) {
                      FUN_1428a9a60(pppplVar20,ppplVar4,0x38,1,1);
                    }
                    ppplVar5 = pppplVar20[1];
                    *(code **)((longlong)ppplVar5 + (longlong)ppplVar4) = FUN_1412d6460;
                    *(code **)((longlong)(ppplVar5 + 1) + (longlong)ppplVar4) = FUN_141564d30;
                    *(undefined **)((longlong)(ppplVar5 + 2) + (longlong)ppplVar4) = puVar11;
                    *(undefined8 *)((longlong)(ppplVar5 + 3) + (longlong)ppplVar4) = uVar2;
                    *(undefined **)((longlong)(ppplVar5 + 4) + (longlong)ppplVar4) = puVar18;
                    *(undefined2 *)((longlong)(ppplVar5 + 5) + (longlong)ppplVar4) = 4;
                    *(undefined1 *)((longlong)(ppplVar5 + 6) + (longlong)ppplVar4) = 0;
                    pppplVar20[2] = ppplVar4 + 7;
                    FUN_140d146a0(&uStack_198,param_6);
                    puVar11 = puStack_178;
                    pppplVar20 = uStack_190;
                    if (uStack_190 == (longlong ****)0x0) {
                      pppplVar20 = (longlong ****)ppplStack_188;
                    }
                    ppplVar4 = pppplVar20[2];
                    if ((ulonglong)((longlong)*pppplVar20 - (longlong)ppplVar4) < 0x38) {
                      FUN_1428a9a60(pppplVar20,ppplVar4,0x38,1,1);
                    }
                    ppplVar5 = pppplVar20[1];
                    *(code **)((longlong)ppplVar5 + (longlong)ppplVar4) = FUN_1412d6460;
                    *(code **)((longlong)(ppplVar5 + 1) + (longlong)ppplVar4) = FUN_141564d30;
                    *(undefined **)((longlong)(ppplVar5 + 2) + (longlong)ppplVar4) = puVar11;
                    *(undefined8 *)((longlong)(ppplVar5 + 3) + (longlong)ppplVar4) = uVar2;
                    *(undefined **)((longlong)(ppplVar5 + 4) + (longlong)ppplVar4) = puVar18;
                    *(undefined2 *)((longlong)(ppplVar5 + 5) + (longlong)ppplVar4) = 5;
                    *(undefined1 *)((longlong)(ppplVar5 + 6) + (longlong)ppplVar4) = 0;
                    pppplVar20[2] = ppplVar4 + 7;
                    FUN_140d146a0(&uStack_198,param_6);
                    puVar11 = puStack_178;
                    pppplVar20 = uStack_190;
                    if (uStack_190 == (longlong ****)0x0) {
                      pppplVar20 = (longlong ****)ppplStack_188;
                    }
                    ppplVar4 = pppplVar20[2];
                    if ((ulonglong)((longlong)*pppplVar20 - (longlong)ppplVar4) < 0x38) {
                      FUN_1428a9a60(pppplVar20,ppplVar4,0x38,1,1);
                    }
                    ppplVar5 = pppplVar20[1];
                    *(code **)((longlong)ppplVar5 + (longlong)ppplVar4) = FUN_1412d76d0;
                    *(code **)((longlong)(ppplVar5 + 1) + (longlong)ppplVar4) = FUN_141564d30;
                    *(undefined **)((longlong)(ppplVar5 + 2) + (longlong)ppplVar4) = puVar11;
                    *(undefined8 *)((longlong)(ppplVar5 + 3) + (longlong)ppplVar4) = uStack_268;
                    *(undefined **)((longlong)(ppplVar5 + 4) + (longlong)ppplVar4) = puVar18;
                    *(undefined2 *)((longlong)(ppplVar5 + 5) + (longlong)ppplVar4) = 6;
                    *(undefined1 *)((longlong)(ppplVar5 + 6) + (longlong)ppplVar4) = 0;
                    pppplVar20[2] = ppplVar4 + 7;
                    FUN_140d146a0(&uStack_198,param_6);
                    puVar11 = puStack_178;
                    pppplVar20 = uStack_190;
                    if (uStack_190 == (longlong ****)0x0) {
                      pppplVar20 = (longlong ****)ppplStack_188;
                    }
                    ppplVar4 = pppplVar20[2];
                    if ((ulonglong)((longlong)*pppplVar20 - (longlong)ppplVar4) < 0x38) {
                      FUN_1428a9a60(pppplVar20,ppplVar4,0x38,1,1);
                    }
                    ppplVar5 = pppplVar20[1];
                    *(code **)((longlong)ppplVar5 + (longlong)ppplVar4) = FUN_1412d76d0;
                    *(code **)((longlong)(ppplVar5 + 1) + (longlong)ppplVar4) = FUN_141564d30;
                    *(undefined **)((longlong)(ppplVar5 + 2) + (longlong)ppplVar4) = puVar11;
                    *(undefined8 *)((longlong)(ppplVar5 + 3) + (longlong)ppplVar4) = uStack_1a0;
                    *(undefined **)((longlong)(ppplVar5 + 4) + (longlong)ppplVar4) = puVar18;
                    *(undefined2 *)((longlong)(ppplVar5 + 5) + (longlong)ppplVar4) = 7;
                    *(undefined1 *)((longlong)(ppplVar5 + 6) + (longlong)ppplVar4) = 0;
                    pppplVar20[2] = ppplVar4 + 7;
                    FUN_140d146a0(&uStack_198,param_6);
                    puVar11 = puStack_178;
                    pppplVar20 = uStack_190;
                    if (uStack_190 == (longlong ****)0x0) {
                      pppplVar20 = (longlong ****)ppplStack_188;
                    }
                    ppplVar4 = pppplVar20[2];
                    if ((ulonglong)((longlong)*pppplVar20 - (longlong)ppplVar4) < 0x38) {
                      FUN_1428a9a60(pppplVar20,ppplVar4,0x38,1,1);
                    }
                    ppplVar5 = pppplVar20[1];
                    *(code **)((longlong)ppplVar5 + (longlong)ppplVar4) = FUN_1412d76d0;
                    *(code **)((longlong)(ppplVar5 + 1) + (longlong)ppplVar4) = FUN_141564d30;
                    *(undefined **)((longlong)(ppplVar5 + 2) + (longlong)ppplVar4) = puVar11;
                    *(undefined8 *)((longlong)(ppplVar5 + 3) + (longlong)ppplVar4) = uStack_1a0;
                    *(undefined **)((longlong)(ppplVar5 + 4) + (longlong)ppplVar4) = puVar18;
                    *(undefined2 *)((longlong)(ppplVar5 + 5) + (longlong)ppplVar4) = 8;
                    *(undefined1 *)((longlong)(ppplVar5 + 6) + (longlong)ppplVar4) = 0;
                    pppplVar20[2] = ppplVar4 + 7;
                    FUN_140d146a0(&uStack_198,param_6);
                    puVar11 = puStack_178;
                    pppplVar20 = uStack_190;
                    if (uStack_190 == (longlong ****)0x0) {
                      pppplVar20 = (longlong ****)ppplStack_188;
                    }
                    ppplVar4 = pppplVar20[2];
                    if ((ulonglong)((longlong)*pppplVar20 - (longlong)ppplVar4) < 0x38) {
                      FUN_1428a9a60(pppplVar20,ppplVar4,0x38,1,1);
                    }
                    ppplVar5 = pppplVar20[1];
                    *(code **)((longlong)ppplVar5 + (longlong)ppplVar4) = FUN_1412d76d0;
                    *(code **)((longlong)(ppplVar5 + 1) + (longlong)ppplVar4) = FUN_141564d30;
                    *(undefined **)((longlong)(ppplVar5 + 2) + (longlong)ppplVar4) = puVar11;
                    *(undefined8 *)((longlong)(ppplVar5 + 3) + (longlong)ppplVar4) = uStack_1a0;
                    *(undefined **)((longlong)(ppplVar5 + 4) + (longlong)ppplVar4) = puVar18;
                    *(undefined2 *)((longlong)(ppplVar5 + 5) + (longlong)ppplVar4) = 9;
                    *(undefined1 *)((longlong)(ppplVar5 + 6) + (longlong)ppplVar4) = 0;
                    pppplVar20[2] = ppplVar4 + 7;
                    FUN_140d146a0(&uStack_198,param_6);
                    puVar11 = puStack_178;
                    pppplVar20 = uStack_190;
                    if (uStack_190 == (longlong ****)0x0) {
                      pppplVar20 = (longlong ****)ppplStack_188;
                    }
                    ppplVar4 = pppplVar20[2];
                    if ((ulonglong)((longlong)*pppplVar20 - (longlong)ppplVar4) < 0x38) {
                      FUN_1428a9a60(pppplVar20,ppplVar4,0x38,1,1);
                    }
                    ppplVar5 = pppplVar20[1];
                    *(code **)((longlong)ppplVar5 + (longlong)ppplVar4) = FUN_1412d76d0;
                    *(code **)((longlong)(ppplVar5 + 1) + (longlong)ppplVar4) = FUN_141564d30;
                    *(undefined **)((longlong)(ppplVar5 + 2) + (longlong)ppplVar4) = puVar11;
                    *(undefined8 *)((longlong)(ppplVar5 + 3) + (longlong)ppplVar4) = uStack_1a0;
                    *(undefined **)((longlong)(ppplVar5 + 4) + (longlong)ppplVar4) = puVar18;
                    *(undefined2 *)((longlong)(ppplVar5 + 5) + (longlong)ppplVar4) = 10;
                    *(undefined1 *)((longlong)(ppplVar5 + 6) + (longlong)ppplVar4) = 0;
                    pppplVar20[2] = ppplVar4 + 7;
                    FUN_140d146a0(&uStack_198,param_6);
                    puVar11 = puStack_178;
                    uVar2 = uStack_1b0;
                    pppplVar20 = uStack_190;
                    if (uStack_190 == (longlong ****)0x0) {
                      pppplVar20 = (longlong ****)ppplStack_188;
                    }
                    ppplVar4 = pppplVar20[2];
                    if ((ulonglong)((longlong)*pppplVar20 - (longlong)ppplVar4) < 0x38) {
                      FUN_1428a9a60(pppplVar20,ppplVar4,0x38,1,1);
                    }
                    ppplVar5 = pppplVar20[1];
                    *(code **)((longlong)ppplVar5 + (longlong)ppplVar4) = FUN_1412dbf80;
                    *(code **)((longlong)(ppplVar5 + 1) + (longlong)ppplVar4) = FUN_141564d30;
                    *(undefined **)((longlong)(ppplVar5 + 2) + (longlong)ppplVar4) = puVar11;
                    *(undefined8 *)((longlong)(ppplVar5 + 3) + (longlong)ppplVar4) = uStack_270;
                    *(undefined **)((longlong)(ppplVar5 + 4) + (longlong)ppplVar4) = puVar18;
                    *(undefined1 *)((longlong)(ppplVar5 + 5) + (longlong)ppplVar4) = 2;
                    *(undefined1 *)((longlong)(ppplVar5 + 6) + (longlong)ppplVar4) = 0;
                    pppplVar20[2] = ppplVar4 + 7;
                    FUN_140d146a0(&uStack_198,param_6);
                    puVar11 = puStack_178;
                    pppplVar20 = uStack_190;
                    if (uStack_190 == (longlong ****)0x0) {
                      pppplVar20 = (longlong ****)ppplStack_188;
                    }
                    ppplVar4 = pppplVar20[2];
                    if ((ulonglong)((longlong)*pppplVar20 - (longlong)ppplVar4) < 0x40) {
                      FUN_1428a9a60(pppplVar20,ppplVar4,0x40,1,1);
                    }
                    ppplVar5 = pppplVar20[1];
                    *(code **)((longlong)ppplVar5 + (longlong)ppplVar4) = FUN_1412d8030;
                    *(code **)((longlong)(ppplVar5 + 1) + (longlong)ppplVar4) = FUN_141564d30;
                    *(undefined **)((longlong)(ppplVar5 + 2) + (longlong)ppplVar4) = puVar11;
                    *(undefined8 *)((longlong)(ppplVar5 + 3) + (longlong)ppplVar4) = uStack_1c0;
                    *(undefined **)((longlong)(ppplVar5 + 4) + (longlong)ppplVar4) = puVar18;
                    *(undefined8 *)((longlong)(ppplVar5 + 5) + (longlong)ppplVar4) = 1;
                    *(undefined1 *)((longlong)(ppplVar5 + 6) + (longlong)ppplVar4) = 0xd;
                    *(undefined1 *)((longlong)(ppplVar5 + 7) + (longlong)ppplVar4) = 0;
                    pppplVar20[2] = ppplVar4 + 8;
                    FUN_140d146a0(&uStack_198,param_6);
                    puVar11 = puStack_178;
                    pppplVar20 = uStack_190;
                    if (uStack_190 == (longlong ****)0x0) {
                      pppplVar20 = (longlong ****)ppplStack_188;
                    }
                    ppplVar4 = pppplVar20[2];
                    if ((ulonglong)((longlong)*pppplVar20 - (longlong)ppplVar4) < 0x40) {
                      FUN_1428a9a60(pppplVar20,ppplVar4,0x40,1,1);
                    }
                    ppplVar5 = pppplVar20[1];
                    *(code **)((longlong)ppplVar5 + (longlong)ppplVar4) = FUN_1412d8030;
                    *(code **)((longlong)(ppplVar5 + 1) + (longlong)ppplVar4) = FUN_141564d30;
                    *(undefined **)((longlong)(ppplVar5 + 2) + (longlong)ppplVar4) = puVar11;
                    *(undefined8 *)((longlong)(ppplVar5 + 3) + (longlong)ppplVar4) = uStack_1c0;
                    *(undefined **)((longlong)(ppplVar5 + 4) + (longlong)ppplVar4) = puVar18;
                    *(undefined8 *)((longlong)(ppplVar5 + 5) + (longlong)ppplVar4) = 2;
                    *(undefined1 *)((longlong)(ppplVar5 + 6) + (longlong)ppplVar4) = 0xd;
                    *(undefined1 *)((longlong)(ppplVar5 + 7) + (longlong)ppplVar4) = 0;
                    pppplVar20[2] = ppplVar4 + 8;
                    FUN_140d146a0(&uStack_198,param_6);
                    puVar11 = puStack_178;
                    pppplVar20 = uStack_190;
                    if (uStack_190 == (longlong ****)0x0) {
                      pppplVar20 = (longlong ****)ppplStack_188;
                    }
                    ppplVar4 = pppplVar20[2];
                    if ((ulonglong)((longlong)*pppplVar20 - (longlong)ppplVar4) < 0x40) {
                      FUN_1428a9a60(pppplVar20,ppplVar4,0x40,1,1);
                    }
                    ppplVar5 = pppplVar20[1];
                    *(code **)((longlong)ppplVar5 + (longlong)ppplVar4) = FUN_1412dcf50;
                    *(code **)((longlong)(ppplVar5 + 1) + (longlong)ppplVar4) = FUN_141564d30;
                    *(undefined **)((longlong)(ppplVar5 + 2) + (longlong)ppplVar4) = puVar11;
                    *(undefined8 *)((longlong)(ppplVar5 + 3) + (longlong)ppplVar4) = uStack_280;
                    *(undefined8 *)((longlong)(ppplVar5 + 4) + (longlong)ppplVar4) = uStack_1c8;
                    *(undefined8 *)((longlong)(ppplVar5 + 5) + (longlong)ppplVar4) = 0;
                    *(undefined2 *)((longlong)(ppplVar5 + 6) + (longlong)ppplVar4) = 0xb01;
                    *(undefined1 *)((longlong)(ppplVar5 + 7) + (longlong)ppplVar4) = 0;
                    pppplVar20[2] = ppplVar4 + 8;
                    FUN_140d146a0(&uStack_198,param_6);
                    puVar11 = puStack_178;
                    pppplVar20 = uStack_190;
                    if (uStack_190 == (longlong ****)0x0) {
                      pppplVar20 = (longlong ****)ppplStack_188;
                    }
                    ppplVar4 = pppplVar20[2];
                    if ((ulonglong)((longlong)*pppplVar20 - (longlong)ppplVar4) < 0x40) {
                      FUN_1428a9a60(pppplVar20,ppplVar4,0x40,1,1);
                    }
                    ppplVar5 = pppplVar20[1];
                    *(code **)((longlong)ppplVar5 + (longlong)ppplVar4) = FUN_1412dcf50;
                    *(code **)((longlong)(ppplVar5 + 1) + (longlong)ppplVar4) = FUN_141564d30;
                    *(undefined **)((longlong)(ppplVar5 + 2) + (longlong)ppplVar4) = puVar11;
                    *(undefined8 *)((longlong)(ppplVar5 + 3) + (longlong)ppplVar4) = uStack_278;
                    *(undefined8 *)((longlong)(ppplVar5 + 4) + (longlong)ppplVar4) = uStack_1c8;
                    *(undefined8 *)((longlong)(ppplVar5 + 5) + (longlong)ppplVar4) = 0;
                    *(undefined2 *)((longlong)(ppplVar5 + 6) + (longlong)ppplVar4) = 0xc01;
                    *(undefined1 *)((longlong)(ppplVar5 + 7) + (longlong)ppplVar4) = 0;
                    pppplVar20[2] = ppplVar4 + 8;
                    FUN_140a06b00(&uStack_198,0,_UNK_142925904);
                    if (!SBORROW8(0,(longlong)uStack_198)) {
                      puStack_78 = puStack_128;
                      puStack_88 = puStack_138;
                      uStack_80 = uStack_130;
                      puStack_98 = puStack_148;
                      uStack_90 = uStack_140;
                      puStack_a8 = puStack_158;
                      uStack_a0 = uStack_150;
                      uStack_b8 = (undefined4)uStack_168;
                      uStack_b4 = uStack_168._4_4_;
                      uStack_b0 = (undefined4)uStack_160;
                      uStack_ac = uStack_160._4_4_;
                      uStack_c8 = puStack_178;
                      uStack_c0 = (undefined4)uStack_170;
                      uStack_bc = uStack_170._4_4_;
                      uStack_d8 = (longlong ****)ppplStack_188;
                      uStack_d0 = (longlong ****)ppplStack_180;
                      ppplStack_e8 = uStack_198;
                      uStack_e0 = uStack_190;
                      ppplStack_258 = (longlong ***)pppplVar19;
                      uStack_250 = 0x3f800000;
                      FUN_140a07680(&uStack_198,&ppplStack_e8,&ppplStack_258);
                      FUN_140b1aca0(&uStack_198,&UNK_142aaedd8,0xb,uVar2,uVar17);
                      FUN_140b0eeb0(&uStack_198,&UNK_142aaede3,0x17);
                      if (SBORROW8(0,(longlong)uStack_198)) {
                        ppplStack_e8 = (longlong ***)uStack_190;
                        FUN_1428d9760(&UNK_142aae510,0x2b,&ppplStack_e8,&UNK_142aae4f0,
                                      &UNK_142aaee00);
                      }
                      else {
                        puStack_78 = puStack_128;
                        puStack_88 = puStack_138;
                        uStack_80 = uStack_130;
                        puStack_98 = puStack_148;
                        uStack_90 = uStack_140;
                        puStack_a8 = puStack_158;
                        uStack_a0 = uStack_150;
                        uStack_b8 = (undefined4)uStack_168;
                        uStack_b4 = uStack_168._4_4_;
                        uStack_b0 = (undefined4)uStack_160;
                        uStack_ac = uStack_160._4_4_;
                        uStack_c8 = puStack_178;
                        uStack_c0 = (undefined4)uStack_170;
                        uStack_bc = uStack_170._4_4_;
                        uStack_d8 = (longlong ****)ppplStack_188;
                        uStack_d0 = (longlong ****)ppplStack_180;
                        ppplStack_e8 = uStack_198;
                        uStack_e0 = uStack_190;
                        uVar17 = FUN_140b1aca0(&ppplStack_e8,&UNK_142aaee18,8,uVar2,uVar17);
                        FUN_140d146a0(&uStack_198,param_6);
                        puVar11 = puStack_178;
                        pppplVar19 = uStack_190;
                        if (uStack_190 == (longlong ****)0x0) {
                          pppplVar19 = (longlong ****)ppplStack_188;
                        }
                        ppplVar4 = pppplVar19[2];
                        if ((ulonglong)((longlong)*pppplVar19 - (longlong)ppplVar4) < 0x38) {
                          FUN_1428a9a60(pppplVar19,ppplVar4,0x38,1,1);
                        }
                        ppplVar5 = pppplVar19[1];
                        *(code **)((longlong)ppplVar5 + (longlong)ppplVar4) = FUN_1412d8e20;
                        *(code **)((longlong)(ppplVar5 + 1) + (longlong)ppplVar4) = FUN_141564d30;
                        *(undefined **)((longlong)(ppplVar5 + 2) + (longlong)ppplVar4) = puVar11;
                        *(undefined8 *)((longlong)(ppplVar5 + 3) + (longlong)ppplVar4) = uVar17;
                        *(undefined **)((longlong)(ppplVar5 + 4) + (longlong)ppplVar4) = puVar18;
                        *(undefined2 *)((longlong)(ppplVar5 + 5) + (longlong)ppplVar4) = 0xe00;
                        *(undefined1 *)((longlong)(ppplVar5 + 6) + (longlong)ppplVar4) = 0;
                        pppplVar19[2] = ppplVar4 + 7;
                        FUN_140b0eeb0(&uStack_198,&UNK_142aaee20,0x20);
                        if (SBORROW8(0,(longlong)uStack_198)) {
                          ppplStack_e8 = (longlong ***)uStack_190;
                          FUN_1428d9760(&UNK_142aae510,0x2b,&ppplStack_e8,&UNK_142aae4f0,
                                        &UNK_142aaee40);
                        }
                        else {
                          puStack_78 = puStack_128;
                          puStack_88 = puStack_138;
                          uStack_80 = uStack_130;
                          puStack_98 = puStack_148;
                          uStack_90 = uStack_140;
                          puStack_a8 = puStack_158;
                          uStack_a0 = uStack_150;
                          uStack_b8 = (undefined4)uStack_168;
                          uStack_b4 = uStack_168._4_4_;
                          uStack_b0 = (undefined4)uStack_160;
                          uStack_ac = uStack_160._4_4_;
                          uStack_c8 = puStack_178;
                          uStack_c0 = (undefined4)uStack_170;
                          uStack_bc = uStack_170._4_4_;
                          uStack_d8 = (longlong ****)ppplStack_188;
                          uStack_d0 = (longlong ****)ppplStack_180;
                          ppplStack_e8 = uStack_198;
                          uStack_e0 = uStack_190;
                          uVar17 = FUN_140b1aca0(&ppplStack_e8,&UNK_142aaee58,6,uVar2,uStack_1a8);
                          uVar2 = uStack_1b8;
                          puVar18 = &UNK_142aaed91;
                          uVar16 = FUN_140937aa0(uStack_1b8,&UNK_142aaed91,0x13);
                          if ((uVar16 & 1) == 0) {
                            FUN_1428d9310(&UNK_142aaee60);
                          }
                          else {
                            func_0x000140c9d180(&uStack_198);
                            pppplVar19 = uStack_190;
                            ppplVar4 = uStack_198;
                            FUN_140d146a0(&uStack_198,param_6);
                            puVar11 = puStack_178;
                            pppplVar20 = uStack_190;
                            if (uStack_190 == (longlong ****)0x0) {
                              pppplVar20 = (longlong ****)ppplStack_188;
                            }
                            ppplVar5 = pppplVar20[2];
                            if ((ulonglong)((longlong)*pppplVar20 - (longlong)ppplVar5) < 0x78) {
                              FUN_1428a9a60(pppplVar20,ppplVar5,0x78,1,1);
                            }
                            ppplVar6 = pppplVar20[1];
                            *(code **)((longlong)ppplVar6 + (longlong)ppplVar5) = FUN_1412dd5d0;
                            *(undefined8 *)((longlong)(ppplVar6 + 1) + (longlong)ppplVar5) =
                                 ppplVar4;
                            ((undefined8 *)((longlong)(ppplVar6 + 1) + (longlong)ppplVar5))[1] =
                                 pppplVar19;
                            uVar3 = _UNK_142aa68b0;
                            *(undefined8 *)((longlong)(ppplVar6 + 3) + (longlong)ppplVar5) =
                                 _UNK_142aa68b0;
                            *(undefined4 *)((longlong)(ppplVar6 + 4) + (longlong)ppplVar5) =
                                 0x80000000;
                            *(undefined8 *)((longlong)ppplVar6 + 0x24 + (longlong)ppplVar5) = 0;
                            *(undefined4 *)((longlong)ppplVar6 + 0x2c + (longlong)ppplVar5) = 0;
                            *(undefined8 *)((longlong)(ppplVar6 + 7) + (longlong)ppplVar5) = uVar17;
                            *(undefined **)((longlong)(ppplVar6 + 8) + (longlong)ppplVar5) = puVar18
                            ;
                            *(undefined1 *)((longlong)(ppplVar6 + 9) + (longlong)ppplVar5) = 0;
                            *(undefined **)((longlong)(ppplVar6 + 0xb) + (longlong)ppplVar5) =
                                 puVar11;
                            *(code **)((longlong)(ppplVar6 + 0xd) + (longlong)ppplVar5) =
                                 FUN_141564d30;
                            pppplVar20[2] = ppplVar5 + 0xf;
                            FUN_140b0eeb0(&uStack_198,&UNK_142aaee78,0x1f);
                            if (!SBORROW8(0,(longlong)uStack_198)) {
                              puStack_78 = puStack_128;
                              puStack_88 = puStack_138;
                              uStack_80 = uStack_130;
                              puStack_98 = puStack_148;
                              uStack_90 = uStack_140;
                              puStack_a8 = puStack_158;
                              uStack_a0 = uStack_150;
                              uStack_b8 = (undefined4)uStack_168;
                              uStack_b4 = uStack_168._4_4_;
                              uStack_b0 = (undefined4)uStack_160;
                              uStack_ac = uStack_160._4_4_;
                              uStack_c8 = puStack_178;
                              uStack_c0 = (undefined4)uStack_170;
                              uStack_bc = uStack_170._4_4_;
                              uStack_d8 = (longlong ****)ppplStack_188;
                              uStack_d0 = (longlong ****)ppplStack_180;
                              ppplStack_e8 = uStack_198;
                              uStack_e0 = uStack_190;
                              uVar17 = FUN_140b1aca0(&ppplStack_e8,&UNK_142aaeeb0,0xd,uStack_1b0,
                                                     uStack_1a8);
                              puVar18 = &UNK_142aaed91;
                              uVar16 = FUN_140937aa0(uVar2,&UNK_142aaed91,0x13);
                              if ((uVar16 & 1) != 0) {
                                func_0x000140c9d180(&uStack_198);
                                pppplVar19 = uStack_190;
                                ppplVar4 = uStack_198;
                                FUN_140d146a0(&uStack_198,param_6);
                                puVar11 = puStack_178;
                                pppplVar20 = uStack_190;
                                if (uStack_190 == (longlong ****)0x0) {
                                  pppplVar20 = (longlong ****)ppplStack_188;
                                }
                                ppplVar5 = pppplVar20[2];
                                if ((ulonglong)((longlong)*pppplVar20 - (longlong)ppplVar5) < 0x78)
                                {
                                  FUN_1428a9a60(pppplVar20,ppplVar5,0x78,1,1);
                                }
                                ppplVar6 = pppplVar20[1];
                                *(code **)((longlong)ppplVar6 + (longlong)ppplVar5) = FUN_1412d71a0;
                                *(undefined8 *)((longlong)(ppplVar6 + 1) + (longlong)ppplVar5) =
                                     ppplVar4;
                                ((undefined8 *)((longlong)(ppplVar6 + 1) + (longlong)ppplVar5))[1] =
                                     pppplVar19;
                                *(undefined8 *)((longlong)(ppplVar6 + 3) + (longlong)ppplVar5) =
                                     uVar3;
                                *(undefined4 *)((longlong)(ppplVar6 + 4) + (longlong)ppplVar5) =
                                     0x80000000;
                                *(undefined8 *)((longlong)ppplVar6 + 0x24 + (longlong)ppplVar5) = 0;
                                *(undefined4 *)((longlong)ppplVar6 + 0x2c + (longlong)ppplVar5) = 0;
                                *(undefined8 *)((longlong)(ppplVar6 + 7) + (longlong)ppplVar5) =
                                     uVar17;
                                *(undefined **)((longlong)(ppplVar6 + 8) + (longlong)ppplVar5) =
                                     puVar18;
                                *(undefined1 *)((longlong)(ppplVar6 + 9) + (longlong)ppplVar5) = 0;
                                *(undefined **)((longlong)(ppplVar6 + 0xb) + (longlong)ppplVar5) =
                                     puVar11;
                                *(code **)((longlong)(ppplVar6 + 0xd) + (longlong)ppplVar5) =
                                     FUN_141564d30;
                                pppplVar20[2] = ppplVar5 + 0xf;
                                return;
                              }
                              FUN_1428d9310(&UNK_142aaeec0);
                              pcVar7 = (code *)swi(3);
                              (*pcVar7)();
                              return;
                            }
                          }
                          ppplStack_e8 = (longlong ***)uStack_190;
                          FUN_1428d9760(&UNK_142aae510,0x2b,&ppplStack_e8,&UNK_142aae4f0,
                                        &UNK_142aaee98);
                        }
                      }
                      goto code_r0x0001409267b9;
                    }
                  }
                  uStack_d8 = (longlong ****)ppplStack_180;
                  ppplStack_e8 = (longlong ***)uStack_190;
                  uStack_e0 = (longlong ****)ppplStack_188;
                  FUN_1428d9760(&UNK_142aae510,0x2b,&ppplStack_e8,&UNK_142aae540,&UNK_142aaedc0);
                }
              }
            }
          }
        }
      }
    }
  }
code_r0x0001409267b9:
                    /* WARNING: Does not return */
  pcVar7 = (code *)invalidInstructionException();
  (*pcVar7)();
}

