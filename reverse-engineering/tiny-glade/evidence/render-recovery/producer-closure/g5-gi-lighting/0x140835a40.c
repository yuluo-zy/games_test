
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void FUN_140835a40(undefined8 param_1,longlong *param_2,longlong param_3,longlong param_4,
                  longlong param_5)

{
  byte *pbVar1;
  byte bVar2;
  byte bVar3;
  byte bVar4;
  ulonglong *puVar5;
  longlong lVar6;
  code *pcVar7;
  uint uVar8;
  longlong lVar9;
  undefined *puVar10;
  undefined *puVar11;
  undefined8 uVar12;
  longlong *plVar13;
  ulonglong uVar14;
  longlong lVar15;
  ulonglong uVar16;
  undefined *puVar17;
  undefined *puVar18;
  undefined1 uVar19;
  undefined1 auStack_eb8 [248];
  undefined1 auStack_dc0 [169];
  byte bStack_d17;
  undefined1 auStack_cc8 [169];
  byte bStack_c1f;
  undefined1 auStack_bd0 [169];
  byte bStack_b27;
  undefined1 auStack_ad8 [169];
  byte bStack_a2f;
  undefined1 auStack_9e0 [1000];
  undefined4 auStack_5f8 [2];
  longlong lStack_5f0;
  undefined4 uStack_5d8;
  longlong lStack_5d0;
  undefined1 uStack_547;
  undefined8 uStack_210;
  undefined8 uStack_208;
  float fStack_1f0;
  uint uStack_1ec;
  undefined4 uStack_1e8;
  undefined4 uStack_1e4;
  undefined4 uStack_1e0;
  undefined4 uStack_1dc;
  undefined4 uStack_1d8;
  undefined4 uStack_1d4;
  undefined4 uStack_1d0;
  undefined4 uStack_1cc;
  undefined8 uStack_1c8;
  undefined8 uStack_1c0;
  undefined8 uStack_1b8;
  undefined8 uStack_1b0;
  undefined8 uStack_1a8;
  float fStack_1a0;
  undefined1 uStack_19c;
  longlong lStack_198;
  undefined8 uStack_190;
  longlong lStack_188;
  undefined8 uStack_180;
  undefined8 uStack_178;
  undefined8 uStack_170;
  undefined8 uStack_168;
  undefined8 uStack_160;
  undefined8 uStack_158;
  undefined8 uStack_150;
  float fStack_148;
  uint uStack_144;
  undefined4 uStack_140;
  undefined4 uStack_13c;
  undefined4 uStack_138;
  undefined4 uStack_134;
  undefined4 uStack_130;
  undefined4 uStack_12c;
  undefined4 uStack_128;
  undefined4 uStack_124;
  undefined4 uStack_120;
  undefined4 uStack_11c;
  undefined8 uStack_118;
  undefined8 uStack_110;
  undefined8 uStack_108;
  undefined4 uStack_100;
  undefined4 uStack_fc;
  undefined4 uStack_f8;
  undefined4 uStack_f4;
  undefined8 uStack_f0;
  uint uStack_e8;
  undefined4 uStack_e4;
  undefined2 uStack_e0;
  longlong lStack_98;
  longlong lStack_90;
  longlong *plStack_88;
  undefined8 uStack_80;
  longlong lStack_78;
  longlong *plStack_70;
  char cStack_66;
  undefined1 uStack_65;
  undefined1 uStack_64;
  undefined1 uStack_63;
  undefined1 uStack_62;
  undefined1 uStack_61;
  undefined8 uStack_60;
  
  uStack_60 = 0xfffffffffffffffe;
  lStack_198 = param_4;
  uStack_80 = param_1;
  lStack_78 = param_3;
  cStack_66 = func_0x000140c16a00();
  if (cStack_66 == '\0') {
code_r0x000140835af3:
    uStack_210 = 0;
code_r0x000140835cd4:
    lStack_98 = lStack_78 + 0x28;
    uVar12 = func_0x000140bd5790();
    lStack_188 = lStack_78 + 0x24;
    auStack_5f8[0] = 0;
    lStack_5f0 = lStack_78;
    uStack_190 = 0;
    uStack_180 = 0;
    lStack_90 = lStack_188;
    FUN_140867970(auStack_9e0,&UNK_142a92ad8,0xd,auStack_5f8,1,&uStack_190,uVar12,param_2,uStack_80)
    ;
    plStack_88 = param_2 + 7;
    uStack_61 = 1;
    uVar14 = 0x4bc362195b80f729;
    plStack_70 = param_2;
    lVar9 = FUN_1400dd6f0(plStack_88,0x4bc362195b80f729,0x982798ce80cbcefb);
    if (((lVar9 != 0) && (uVar14 < (ulonglong)plStack_70[0x43])) &&
       (uVar14 = *(ulonglong *)(plStack_70[0x42] + uVar14 * 8), uVar14 != 0)) {
      lVar9 = ~uVar14 * 0x60;
      if (*(longlong *)(plStack_70[0x3c] + 0x20 + lVar9) != 0) {
        puVar5 = *(ulonglong **)(plStack_70[0x3c] + lVar9 + 0x10);
        fStack_1f0 = (float)(int)puVar5[1] / _UNK_1429a7640 + (float)*puVar5;
        uStack_1ec = uStack_1ec & 0xffffff00;
        lVar9 = *(longlong *)(param_5 + 0xb0);
        lVar15 = *(longlong *)(param_5 + 0xb8) * 0x80 + lVar9;
        for (; lVar9 != lVar15; lVar9 = lVar9 + 0x80) {
          lVar6 = *(longlong *)(lVar9 + 0x50);
          uVar12 = *(undefined8 *)(lVar9 + 0x58);
          uStack_61 = 1;
          uVar8 = func_0x000140be6750();
          uStack_61 = 1;
          plVar13 = (longlong *)FUN_1400d9b20(0);
          if (plVar13 == (longlong *)0x0) goto code_r0x000140836e0e;
          uStack_e0 = 0;
          uStack_e4 = 0x3f800000;
          uStack_190 = 1;
          uStack_180 = 0;
          uStack_178 = 4;
          uStack_170 = 0;
          uStack_100 = (undefined4)*plVar13;
          uStack_fc = *(undefined4 *)((longlong)plVar13 + 4);
          uStack_f8 = (undefined4)plVar13[1];
          uStack_f4 = *(undefined4 *)((longlong)plVar13 + 0xc);
          *plVar13 = *plVar13 + 1;
          uStack_110 = _UNK_142a96730;
          uStack_108 = _UNK_142a96738;
          uStack_120 = _UNK_142a96720;
          uStack_11c = _UNK_142a96724;
          uStack_118 = CONCAT44(_UNK_142a9672c,_UNK_142a96728);
          uStack_150 = 0x100000003;
          uStack_168 = 0;
          uStack_160 = 1;
          uStack_158 = 0;
          uStack_61 = 1;
          lStack_188 = lVar6;
          uStack_f0 = uVar12;
          uStack_e8 = uVar8;
          FUN_140905be0(auStack_5f8,&uStack_190,auStack_9e0,0);
          if (*(char *)(lVar9 + 100) == '\0') {
            FUN_1428d9310(&UNK_142a92b78);
            goto code_r0x000140836f2b;
          }
          uStack_547 = *(undefined1 *)(lVar9 + 0x65);
          func_0x000142923c00(&uStack_190,auStack_5f8,0xf8);
          uStack_61 = 1;
          FUN_14090b460(auStack_ad8,&uStack_190,&fStack_1f0);
          pcVar7 = *(code **)(lVar9 + 0x70);
          bStack_a2f = bStack_a2f & 0x8e | 0x21;
          func_0x000142923c00(auStack_5f8,auStack_ad8,0xf8);
          uStack_61 = 1;
          (*pcVar7)(auStack_5f8,lVar9,plStack_70);
        }
        func_0x000142923c00(auStack_5f8,auStack_9e0,1000);
        uStack_61 = 0;
        FUN_140868370(auStack_5f8);
        uVar12 = func_0x000140bd5790(lStack_98);
        auStack_5f8[0] = 0;
        lStack_5f0 = lStack_78;
        uStack_5d8 = 0;
        lStack_5d0 = lStack_198;
        uStack_190 = 0;
        lStack_188 = lStack_90;
        uStack_180 = 0;
        FUN_140867970(auStack_9e0,&UNK_142a92ae5,0x16,auStack_5f8,2,&uStack_190,uVar12,plStack_70,
                      uStack_80);
        uStack_62 = 1;
        uVar14 = 0x4bc362195b80f729;
        lVar9 = FUN_1400dd6f0(plStack_88,0x4bc362195b80f729,0x982798ce80cbcefb);
        if (((lVar9 != 0) && (uVar14 < (ulonglong)plStack_70[0x43])) &&
           (uVar14 = *(ulonglong *)(plStack_70[0x42] + uVar14 * 8), uVar14 != 0)) {
          lVar9 = ~uVar14 * 0x60;
          if (*(longlong *)(plStack_70[0x3c] + 0x20 + lVar9) != 0) {
            puVar5 = *(ulonglong **)(plStack_70[0x3c] + lVar9 + 0x10);
            fStack_1f0 = (float)(int)puVar5[1] / _UNK_1429a7640 + (float)*puVar5;
            uStack_1ec = uStack_1ec & 0xffffff00;
            lVar9 = *(longlong *)(param_5 + 200);
            lVar15 = *(longlong *)(param_5 + 0xd0) * 0x80 + lVar9;
            for (; lVar9 != lVar15; lVar9 = lVar9 + 0x80) {
              lVar6 = *(longlong *)(lVar9 + 0x50);
              uVar12 = *(undefined8 *)(lVar9 + 0x58);
              uStack_62 = 1;
              uVar8 = func_0x000140be6750();
              uStack_62 = 1;
              plVar13 = (longlong *)FUN_1400d9b20(0);
              if (plVar13 == (longlong *)0x0) {
                uStack_62 = 1;
                FUN_1428d80d0(&UNK_142c3aaa0);
                goto code_r0x000140836f2b;
              }
              uStack_e0 = 0;
              uStack_e4 = 0x3f800000;
              uStack_190 = 1;
              uStack_180 = 0;
              uStack_178 = 4;
              uStack_170 = 0;
              uStack_100 = (undefined4)*plVar13;
              uStack_fc = *(undefined4 *)((longlong)plVar13 + 4);
              uStack_f8 = (undefined4)plVar13[1];
              uStack_f4 = *(undefined4 *)((longlong)plVar13 + 0xc);
              *plVar13 = *plVar13 + 1;
              uStack_110 = _UNK_142a96730;
              uStack_108 = _UNK_142a96738;
              uStack_120 = _UNK_142a96720;
              uStack_11c = _UNK_142a96724;
              uStack_118 = CONCAT44(_UNK_142a9672c,_UNK_142a96728);
              uStack_150 = 0x100000003;
              uStack_168 = 0;
              uStack_160 = 1;
              uStack_158 = 0;
              uStack_62 = 1;
              lStack_188 = lVar6;
              uStack_f0 = uVar12;
              uStack_e8 = uVar8;
              FUN_140905be0(auStack_5f8,&uStack_190,auStack_9e0,0);
              if (*(char *)(lVar9 + 100) == '\0') {
                FUN_1428d9310(&UNK_142a92b60);
                goto code_r0x000140836f2b;
              }
              uStack_547 = *(undefined1 *)(lVar9 + 0x65);
              func_0x000142923c00(&uStack_190,auStack_5f8,0xf8);
              uStack_62 = 1;
              FUN_14090b460(auStack_bd0,&uStack_190,&fStack_1f0);
              pcVar7 = *(code **)(lVar9 + 0x70);
              bStack_b27 = bStack_b27 & 0x8e | 0x21;
              func_0x000142923c00(auStack_5f8,auStack_bd0,0xf8);
              uStack_62 = 1;
              (*pcVar7)(auStack_5f8,lVar9,plStack_70);
            }
            func_0x000142923c00(auStack_5f8,auStack_9e0,1000);
            uStack_62 = 0;
            FUN_140868370(auStack_5f8);
            uVar12 = func_0x000140bd5790(lStack_98);
            auStack_5f8[0] = 0;
            lStack_5f0 = lStack_78;
            lStack_5d0 = lStack_78 + 0x48;
            uStack_5d8 = 0;
            uStack_190 = 0;
            lStack_188 = lStack_90;
            uStack_180 = 0;
            lStack_78 = lStack_5d0;
            FUN_140867970(auStack_9e0,&UNK_142a92afb,0x16,auStack_5f8,2,&uStack_190,uVar12,
                          plStack_70,uStack_80);
            uStack_63 = 1;
            uVar14 = 0x4bc362195b80f729;
            lVar9 = FUN_1400dd6f0(plStack_88,0x4bc362195b80f729,0x982798ce80cbcefb);
            if (((lVar9 != 0) && (uVar14 < (ulonglong)plStack_70[0x43])) &&
               (uVar14 = *(ulonglong *)(plStack_70[0x42] + uVar14 * 8), uVar14 != 0)) {
              lVar9 = ~uVar14 * 0x60;
              if (*(longlong *)(plStack_70[0x3c] + 0x20 + lVar9) != 0) {
                puVar5 = *(ulonglong **)(plStack_70[0x3c] + lVar9 + 0x10);
                fStack_1f0 = (float)(int)puVar5[1] / _UNK_1429a7640 + (float)*puVar5;
                uStack_1ec = uStack_1ec & 0xffffff00;
                lVar9 = *(longlong *)(param_5 + 0xe0);
                lVar15 = *(longlong *)(param_5 + 0xe8) * 0x80 + lVar9;
                for (; lVar9 != lVar15; lVar9 = lVar9 + 0x80) {
                  lVar6 = *(longlong *)(lVar9 + 0x50);
                  uVar12 = *(undefined8 *)(lVar9 + 0x58);
                  uStack_63 = 1;
                  uVar8 = func_0x000140be6750();
                  uStack_63 = 1;
                  plVar13 = (longlong *)FUN_1400d9b20(0);
                  if (plVar13 == (longlong *)0x0) {
                    uStack_63 = 1;
                    FUN_1428d80d0(&UNK_142c3aaa0);
                    goto code_r0x000140836f2b;
                  }
                  uStack_e0 = 0;
                  uStack_e4 = 0x3f800000;
                  uStack_190 = 1;
                  uStack_180 = 0;
                  uStack_178 = 4;
                  uStack_170 = 0;
                  uStack_100 = (undefined4)*plVar13;
                  uStack_fc = *(undefined4 *)((longlong)plVar13 + 4);
                  uStack_f8 = (undefined4)plVar13[1];
                  uStack_f4 = *(undefined4 *)((longlong)plVar13 + 0xc);
                  *plVar13 = *plVar13 + 1;
                  uStack_110 = _UNK_142a96730;
                  uStack_108 = _UNK_142a96738;
                  uStack_120 = _UNK_142a96720;
                  uStack_11c = _UNK_142a96724;
                  uStack_118 = CONCAT44(_UNK_142a9672c,_UNK_142a96728);
                  uStack_150 = 0x100000003;
                  uStack_168 = 0;
                  uStack_160 = 1;
                  uStack_158 = 0;
                  uStack_63 = 1;
                  lStack_188 = lVar6;
                  uStack_f0 = uVar12;
                  uStack_e8 = uVar8;
                  FUN_140905be0(auStack_5f8,&uStack_190,auStack_9e0,0);
                  if (*(char *)(lVar9 + 100) == '\0') {
                    FUN_1428d9310(&UNK_142a92b48);
                    goto code_r0x000140836f2b;
                  }
                  uStack_547 = *(undefined1 *)(lVar9 + 0x65);
                  func_0x000142923c00(&uStack_190,auStack_5f8,0xf8);
                  uStack_63 = 1;
                  FUN_14090b460(auStack_cc8,&uStack_190,&fStack_1f0);
                  pcVar7 = *(code **)(lVar9 + 0x70);
                  bStack_c1f = bStack_c1f & 0x8e | 0x21;
                  func_0x000142923c00(auStack_5f8,auStack_cc8,0xf8);
                  uStack_63 = 1;
                  (*pcVar7)(auStack_5f8,lVar9,plStack_70);
                }
                func_0x000142923c00(auStack_5f8,auStack_9e0,1000);
                uStack_63 = 0;
                FUN_140868370(auStack_5f8);
                uVar12 = func_0x000140bd5790(lStack_98);
                auStack_5f8[0] = 0;
                lStack_5f0 = lStack_198;
                uStack_190 = 0;
                lStack_188 = lStack_90;
                uStack_180 = 0;
                FUN_140867970(auStack_9e0,&UNK_142a92b11,0xc,auStack_5f8,1,&uStack_190,uVar12,
                              plStack_70,uStack_80);
                uStack_64 = 1;
                uVar14 = 0x4bc362195b80f729;
                lVar9 = FUN_1400dd6f0(plStack_88,0x4bc362195b80f729,0x982798ce80cbcefb);
                if (((lVar9 != 0) && (uVar14 < (ulonglong)plStack_70[0x43])) &&
                   (uVar14 = *(ulonglong *)(plStack_70[0x42] + uVar14 * 8), uVar14 != 0)) {
                  lVar9 = ~uVar14 * 0x60;
                  if (*(longlong *)(plStack_70[0x3c] + 0x20 + lVar9) != 0) {
                    puVar5 = *(ulonglong **)(plStack_70[0x3c] + lVar9 + 0x10);
                    fStack_1f0 = (float)(int)puVar5[1] / _UNK_1429a7640 + (float)*puVar5;
                    uStack_1ec = uStack_1ec & 0xffffff00;
                    lVar9 = *(longlong *)(param_5 + 0xf8);
                    lVar15 = *(longlong *)(param_5 + 0x100) * 0x80 + lVar9;
                    for (; lVar9 != lVar15; lVar9 = lVar9 + 0x80) {
                      lVar6 = *(longlong *)(lVar9 + 0x50);
                      uVar12 = *(undefined8 *)(lVar9 + 0x58);
                      uStack_64 = 1;
                      uVar8 = func_0x000140be6750();
                      uStack_64 = 1;
                      plVar13 = (longlong *)FUN_1400d9b20(0);
                      if (plVar13 == (longlong *)0x0) {
                        uStack_64 = 1;
                        FUN_1428d80d0(&UNK_142c3aaa0);
                        goto code_r0x000140836f2b;
                      }
                      uStack_e0 = 0;
                      uStack_e4 = 0x3f800000;
                      uStack_190 = 1;
                      uStack_180 = 0;
                      uStack_178 = 4;
                      uStack_170 = 0;
                      uStack_100 = (undefined4)*plVar13;
                      uStack_fc = *(undefined4 *)((longlong)plVar13 + 4);
                      uStack_f8 = (undefined4)plVar13[1];
                      uStack_f4 = *(undefined4 *)((longlong)plVar13 + 0xc);
                      *plVar13 = *plVar13 + 1;
                      uStack_110 = _UNK_142a96730;
                      uStack_108 = _UNK_142a96738;
                      uStack_120 = _UNK_142a96720;
                      uStack_11c = _UNK_142a96724;
                      uStack_118 = CONCAT44(_UNK_142a9672c,_UNK_142a96728);
                      uStack_150 = 0x100000003;
                      uStack_168 = 0;
                      uStack_160 = 1;
                      uStack_158 = 0;
                      uStack_64 = 1;
                      lStack_188 = lVar6;
                      uStack_f0 = uVar12;
                      uStack_e8 = uVar8;
                      FUN_140905be0(auStack_5f8,&uStack_190,auStack_9e0,0);
                      if (*(char *)(lVar9 + 100) == '\0') {
                        FUN_1428d9310(&UNK_142a92b30);
                        goto code_r0x000140836f2b;
                      }
                      uStack_547 = *(undefined1 *)(lVar9 + 0x65);
                      func_0x000142923c00(&uStack_190,auStack_5f8,0xf8);
                      uStack_64 = 1;
                      FUN_14090b460(auStack_dc0,&uStack_190,&fStack_1f0);
                      pcVar7 = *(code **)(lVar9 + 0x70);
                      bStack_d17 = bStack_d17 & 0x8e | 0x21;
                      func_0x000142923c00(auStack_5f8,auStack_dc0,0xf8);
                      uStack_64 = 1;
                      (*pcVar7)(auStack_5f8,lVar9,plStack_70);
                    }
                    func_0x000142923c00(auStack_5f8,auStack_9e0,1000);
                    uStack_64 = 0;
                    FUN_140868370(auStack_5f8);
                    uVar12 = func_0x000140bd5790(lStack_98);
                    auStack_5f8[0] = 0;
                    lStack_5f0 = lStack_78;
                    uStack_190 = 1;
                    lStack_188 = lStack_90;
                    uStack_180 = 0;
                    FUN_140867970(auStack_9e0,&UNK_142a92b1d,0xd,auStack_5f8,1,&uStack_190,uVar12,
                                  plStack_70,uStack_80);
                    uStack_65 = 1;
                    uVar14 = 0x4bc362195b80f729;
                    lVar9 = FUN_1400dd6f0(plStack_88,0x4bc362195b80f729,0x982798ce80cbcefb);
                    if (((lVar9 != 0) && (uVar14 < (ulonglong)plStack_70[0x43])) &&
                       (uVar14 = *(ulonglong *)(plStack_70[0x42] + uVar14 * 8), uVar14 != 0)) {
                      lVar9 = ~uVar14 * 0x60;
                      if (*(longlong *)(plStack_70[0x3c] + 0x20 + lVar9) != 0) {
                        puVar5 = *(ulonglong **)(plStack_70[0x3c] + lVar9 + 0x10);
                        fStack_1a0 = (float)(int)puVar5[1] / _UNK_1429a7640 + (float)*puVar5;
                        uStack_19c = 0;
                        lVar9 = *(longlong *)(param_5 + 0x110);
                        lVar15 = *(longlong *)(param_5 + 0x118) * 0x80 + lVar9;
                        for (; lVar9 != lVar15; lVar9 = lVar9 + 0x80) {
                          lVar6 = *(longlong *)(lVar9 + 0x50);
                          uVar12 = *(undefined8 *)(lVar9 + 0x58);
                          uStack_65 = 1;
                          uVar8 = func_0x000140be6750();
                          uStack_65 = 1;
                          plVar13 = (longlong *)FUN_1400d9b20(0);
                          if (plVar13 == (longlong *)0x0) {
                            uStack_65 = 1;
                            FUN_1428d80d0(&UNK_142c3aaa0);
                            goto code_r0x000140836f2b;
                          }
                          uStack_100 = (undefined4)*plVar13;
                          uStack_fc = *(undefined4 *)((longlong)plVar13 + 4);
                          uStack_f8 = (undefined4)plVar13[1];
                          uStack_f4 = *(undefined4 *)((longlong)plVar13 + 0xc);
                          *plVar13 = *plVar13 + 1;
                          uStack_1b8 = _UNK_142a96730;
                          uStack_1b0 = _UNK_142a96738;
                          uStack_1c8 = CONCAT44(_UNK_142a96724,_UNK_142a96720);
                          uStack_1c0 = CONCAT44(_UNK_142a9672c,_UNK_142a96728);
                          uStack_190 = 1;
                          uStack_180 = 0;
                          uStack_178 = 4;
                          uStack_170 = 0;
                          uStack_168 = 0;
                          uStack_160 = 1;
                          uStack_158 = 0;
                          uStack_150 = 0x100000003;
                          uStack_108 = _UNK_142a96738;
                          uStack_110 = _UNK_142a96730;
                          uStack_128 = uStack_1d0;
                          uStack_124 = uStack_1cc;
                          uStack_120 = _UNK_142a96720;
                          uStack_11c = _UNK_142a96724;
                          uStack_138 = uStack_1e0;
                          uStack_134 = uStack_1dc;
                          uStack_130 = uStack_1d8;
                          uStack_12c = uStack_1d4;
                          fStack_148 = fStack_1f0;
                          uStack_144 = uStack_1ec;
                          uStack_140 = uStack_1e8;
                          uStack_13c = uStack_1e4;
                          uStack_e8 = uVar8 & 0xffffff8f | 0x20;
                          uStack_e4 = 0x3f800000;
                          uStack_e0 = 0;
                          uStack_65 = 1;
                          lStack_188 = lVar6;
                          uStack_118 = uStack_1c0;
                          uStack_f0 = uVar12;
                          FUN_140905be0(auStack_5f8,&uStack_190,auStack_9e0,0);
                          uStack_65 = 1;
                          FUN_14090b460(auStack_eb8,auStack_5f8,&fStack_1a0);
                          uStack_65 = 1;
                          (**(code **)(lVar9 + 0x70))(auStack_eb8,lVar9,plStack_70);
                        }
                        func_0x000142923c00(auStack_5f8,auStack_9e0,1000);
                        uStack_65 = 0;
                        FUN_140868370(auStack_5f8);
                        if (cStack_66 == '\0') {
                          return;
                        }
                        plVar13 = (longlong *)func_0x000140c18130();
                        if (*plVar13 == 1) {
                          plVar13 = plVar13 + 1;
                        }
                        else if ((*plVar13 != 0) ||
                                (plVar13 = (longlong *)FUN_1428f87d0(plVar13,0),
                                plVar13 == (longlong *)0x0)) goto code_r0x000140836df6;
                        if (*plVar13 == 0) {
                          *plVar13 = -1;
                          plStack_70 = plVar13;
                          FUN_140c170f0(plVar13 + 1,uStack_1a8);
                          *plStack_70 = *plStack_70 + 1;
                          return;
                        }
                        goto code_r0x000140836e02;
                      }
                    }
                    uStack_65 = 1;
                    FUN_1428d9310(&UNK_142aa5bd8);
                    goto code_r0x000140836f2b;
                  }
                }
                uStack_64 = 1;
                FUN_1428d9310(&UNK_142aa5bd8);
                goto code_r0x000140836f2b;
              }
            }
            uStack_63 = 1;
            FUN_1428d9310(&UNK_142aa5bd8);
            goto code_r0x000140836f2b;
          }
        }
        uStack_62 = 1;
        FUN_1428d9310(&UNK_142aa5bd8);
        goto code_r0x000140836f2b;
      }
    }
    uStack_61 = 1;
    FUN_1428d9310(&UNK_142aa5bd8);
  }
  else {
    puVar17 = &UNK_142a9253c;
    lVar15 = 0x49;
    uVar14 = 0x49;
    lVar9 = FUN_140832920(&UNK_142a9253c);
    if (lVar9 == 1) {
      if (uVar14 != 0) {
        if (uVar14 < 0x49) {
          if ((char)(&UNK_142a9253c)[uVar14] < -0x40) {
code_r0x000140835ad3:
            FUN_1428d9940(&UNK_142a9253c,0x49,0,uVar14,&UNK_142a92608);
            goto code_r0x000140835af3;
          }
        }
        else if (uVar14 != 0x49) goto code_r0x000140835ad3;
      }
      puVar17 = &UNK_142a9253c;
      lVar9 = FUN_140832920(&UNK_142a9253c);
      if (lVar9 == 1) {
        uVar16 = uVar14 + 2;
        if (uVar16 != 0) {
          uVar19 = uVar16 == 0x49;
          if (0x48 < uVar16) goto code_r0x000140835b54;
          uVar19 = (&UNK_142a9253e)[uVar14] == -0x41;
          if ((char)(&UNK_142a9253e)[uVar14] < -0x40) {
            do {
              uVar14 = 0x49;
              FUN_1428d9940(&UNK_142a9253c,0x49);
code_r0x000140835b54:
            } while (!(bool)uVar19);
          }
        }
        lVar15 = 0x47 - uVar14;
        puVar17 = &UNK_142a9253c + uVar16;
      }
    }
    puVar10 = &UNK_142a929fc;
    puVar18 = &UNK_142a929b8;
    do {
      bVar2 = puVar10[-1];
      uVar8 = (uint)(char)bVar2;
      if ((char)bVar2 < '\0') {
        bVar3 = puVar10[-2];
        if ((char)bVar3 < -0x40) {
          bVar4 = puVar10[-3];
          if ((char)bVar4 < -0x40) {
            pbVar1 = puVar10 + -4;
            puVar10 = puVar10 + -4;
            uVar8 = bVar4 & 0x3f | (*pbVar1 & 7) << 6;
          }
          else {
            puVar10 = puVar10 + -3;
            uVar8 = bVar4 & 0xf;
          }
          uVar8 = bVar3 & 0x3f | uVar8 << 6;
        }
        else {
          puVar10 = puVar10 + -2;
          uVar8 = bVar3 & 0x1f;
        }
        uVar8 = (uint)(bVar2 & 0x3f) | uVar8 << 6;
        if (uVar8 != 0x5c) goto code_r0x000140835bf9;
code_r0x000140835c0e:
        puVar11 = puVar10 + -0x142a929b8;
        puVar18 = puVar10 + -0x142a929b7;
        if (puVar18 != (undefined *)0x0) {
          uVar19 = puVar18 == (undefined *)0x44;
          if ((undefined *)0x43 < puVar18) goto code_r0x000140835c50;
          uVar19 = puVar10[1] == -0x41;
          if ((char)puVar10[1] < -0x40) {
            do {
              puVar11 = (undefined *)FUN_1428d9940(&UNK_142a929b8,0x44);
code_r0x000140835c50:
            } while (!(bool)uVar19);
          }
        }
        lVar9 = 0x43 - (longlong)puVar11;
        puVar18 = &UNK_142a929b8 + (longlong)puVar18;
        goto code_r0x000140835c5d;
      }
      puVar10 = puVar10 + -1;
      if (uVar8 == 0x5c) goto code_r0x000140835c0e;
code_r0x000140835bf9:
      if (uVar8 == 0x2f) goto code_r0x000140835c0e;
    } while (puVar10 != &UNK_142a929b8);
    lVar9 = 0x44;
code_r0x000140835c5d:
    plVar13 = (longlong *)func_0x000140c18130();
    if (*plVar13 == 1) {
      plVar13 = plVar13 + 1;
code_r0x000140835c76:
      if (*plVar13 == 0) {
        *plVar13 = -1;
        plStack_70 = plVar13;
        uStack_208 = FUN_140c16d40(plVar13 + 1,puVar17,lVar15,puVar18,lVar9,1,0);
        *plStack_70 = *plStack_70 + 1;
        uStack_210 = 1;
        uStack_1a8 = uStack_208;
        goto code_r0x000140835cd4;
      }
    }
    else {
      if ((*plVar13 == 0) &&
         (plVar13 = (longlong *)FUN_1428f87d0(plVar13,0), plVar13 != (longlong *)0x0))
      goto code_r0x000140835c76;
code_r0x000140836df6:
      FUN_1428d80d0(&UNK_142c3aaa0);
    }
code_r0x000140836e02:
    FUN_1428d9250(&UNK_142c3ab70);
code_r0x000140836e0e:
    uStack_61 = 1;
    FUN_1428d80d0(&UNK_142c3aaa0);
  }
code_r0x000140836f2b:
                    /* WARNING: Does not return */
  pcVar7 = (code *)invalidInstructionException();
  (*pcVar7)();
}

