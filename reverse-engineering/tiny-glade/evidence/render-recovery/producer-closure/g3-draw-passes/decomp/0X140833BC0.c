// FUN_140833bc0  0x140833bc0

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void FUN_140833bc0(undefined8 param_1,longlong *param_2,longlong param_3,longlong param_4,
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
  undefined1 auStack_fb8 [248];
  undefined1 auStack_ec0 [169];
  byte bStack_e17;
  undefined1 auStack_dc8 [169];
  byte bStack_d1f;
  undefined1 auStack_cd0 [169];
  byte bStack_c27;
  undefined1 auStack_bd8 [169];
  byte bStack_b2f;
  undefined1 auStack_ae0 [169];
  byte bStack_a37;
  undefined1 auStack_9e8 [1000];
  undefined4 auStack_600 [2];
  longlong lStack_5f8;
  undefined4 uStack_5e0;
  undefined4 uStack_5dc;
  undefined4 uStack_5d8;
  undefined4 uStack_5d4;
  undefined4 uStack_5d0;
  longlong lStack_5c8;
  undefined1 uStack_54f;
  undefined8 uStack_218;
  undefined8 uStack_210;
  undefined8 uStack_1f8;
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
  float fStack_1a8;
  undefined1 uStack_1a4;
  undefined8 uStack_1a0;
  longlong lStack_198;
  undefined8 uStack_190;
  undefined8 uStack_188;
  undefined8 uStack_180;
  undefined8 uStack_178;
  undefined8 uStack_170;
  undefined8 uStack_168;
  undefined8 uStack_160;
  float fStack_158;
  uint uStack_154;
  undefined4 uStack_150;
  undefined4 uStack_14c;
  undefined4 uStack_148;
  undefined4 uStack_144;
  undefined4 uStack_140;
  undefined4 uStack_13c;
  undefined4 uStack_138;
  undefined4 uStack_134;
  undefined4 uStack_130;
  undefined4 uStack_12c;
  undefined8 uStack_128;
  undefined8 uStack_120;
  undefined8 uStack_118;
  undefined4 uStack_110;
  undefined4 uStack_10c;
  undefined4 uStack_108;
  undefined4 uStack_104;
  undefined8 uStack_100;
  uint uStack_f8;
  undefined4 uStack_f4;
  undefined2 uStack_f0;
  longlong lStack_a8;
  longlong lStack_a0;
  longlong lStack_98;
  undefined8 uStack_90;
  longlong *plStack_88;
  longlong lStack_80;
  char cStack_71;
  longlong *plStack_70;
  undefined1 uStack_66;
  undefined1 uStack_65;
  undefined1 uStack_64;
  undefined1 uStack_63;
  undefined1 uStack_62;
  undefined1 uStack_61;
  undefined8 uStack_60;
  
  uStack_60 = 0xfffffffffffffffe;
  lStack_a8 = param_4;
  uStack_90 = param_1;
  lStack_80 = param_3;
  cStack_71 = func_0x000140c16a00();
  if (cStack_71 == '\0') {
code_r0x000140833c73:
    uStack_218 = 0;
code_r0x000140833e54:
    lStack_a0 = lStack_80 + 0x28;
    uVar12 = func_0x000140bd5790();
    lStack_198 = lStack_80 + 0x24;
    auStack_600[0] = 2;
    lStack_5f8 = lStack_80;
    uStack_1a0 = 0;
    uStack_190 = 1;
    lStack_98 = lStack_198;
    FUN_140867970(auStack_9e8,&UNK_142a929fc,8,auStack_600,1,&uStack_1a0,uVar12,param_2,uStack_90);
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
        lVar9 = *(longlong *)(param_5 + 0x20);
        lVar15 = *(longlong *)(param_5 + 0x28) * 0x80 + lVar9;
        for (; lVar9 != lVar15; lVar9 = lVar9 + 0x80) {
          lVar6 = *(longlong *)(lVar9 + 0x50);
          uVar12 = *(undefined8 *)(lVar9 + 0x58);
          uStack_61 = 1;
          uVar8 = func_0x000140be6750();
          uStack_61 = 1;
          plVar13 = (longlong *)FUN_1400d9b20(0);
          if (plVar13 == (longlong *)0x0) goto code_r0x00014083538e;
          uStack_f0 = 0;
          uStack_f4 = 0x3f800000;
          uStack_1a0 = 1;
          uStack_190 = 0;
          uStack_188 = 4;
          uStack_180 = 0;
          uStack_110 = (undefined4)*plVar13;
          uStack_10c = *(undefined4 *)((longlong)plVar13 + 4);
          uStack_108 = (undefined4)plVar13[1];
          uStack_104 = *(undefined4 *)((longlong)plVar13 + 0xc);
          *plVar13 = *plVar13 + 1;
          uStack_120 = _UNK_142a96730;
          uStack_118 = _UNK_142a96738;
          uStack_130 = _UNK_142a96720;
          uStack_12c = _UNK_142a96724;
          uStack_128 = CONCAT44(_UNK_142a9672c,_UNK_142a96728);
          uStack_160 = 0x100000003;
          uStack_178 = 0;
          uStack_170 = 1;
          uStack_168 = 0;
          uStack_61 = 1;
          lStack_198 = lVar6;
          uStack_100 = uVar12;
          uStack_f8 = uVar8;
          FUN_140905be0(auStack_600,&uStack_1a0,auStack_9e8,0);
          if (*(char *)(lVar9 + 100) == '\0') {
            FUN_1428d9310(&UNK_142a92ac0);
            goto code_r0x0001408354fd;
          }
          uStack_54f = *(undefined1 *)(lVar9 + 0x65);
          func_0x000142923c00(&uStack_1a0,auStack_600,0xf8);
          uStack_61 = 1;
          FUN_14090b460(auStack_ae0,&uStack_1a0,&fStack_1f0);
          pcVar7 = *(code **)(lVar9 + 0x70);
          bStack_a37 = bStack_a37 & 0x8e | 0x21;
          func_0x000142923c00(auStack_600,auStack_ae0,0xf8);
          uStack_61 = 1;
          (*pcVar7)(auStack_600,lVar9,plStack_70);
        }
        func_0x000142923c00(auStack_600,auStack_9e8,1000);
        uStack_61 = 0;
        FUN_140868370(auStack_600);
        uVar12 = func_0x000140bd5790(lStack_a0);
        auStack_600[0] = 0;
        lStack_5f8 = lStack_80;
        uStack_5e0 = 1;
        uStack_5dc = 0;
        uStack_5d8 = 0;
        uStack_5d4 = 0;
        uStack_5d0 = 0;
        lStack_5c8 = lStack_a8;
        uStack_1a0 = 0;
        lStack_198 = lStack_98;
        uStack_190 = 0;
        FUN_140867970(auStack_9e8,&UNK_142a92a04,0x11,auStack_600,2,&uStack_1a0,uVar12,plStack_70,
                      uStack_90);
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
            lVar9 = *(longlong *)(param_5 + 0x38);
            lVar15 = *(longlong *)(param_5 + 0x40) * 0x80 + lVar9;
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
                goto code_r0x0001408354fd;
              }
              uStack_f0 = 0;
              uStack_f4 = 0x3f800000;
              uStack_1a0 = 1;
              uStack_190 = 0;
              uStack_188 = 4;
              uStack_180 = 0;
              uStack_110 = (undefined4)*plVar13;
              uStack_10c = *(undefined4 *)((longlong)plVar13 + 4);
              uStack_108 = (undefined4)plVar13[1];
              uStack_104 = *(undefined4 *)((longlong)plVar13 + 0xc);
              *plVar13 = *plVar13 + 1;
              uStack_120 = _UNK_142a96730;
              uStack_118 = _UNK_142a96738;
              uStack_130 = _UNK_142a96720;
              uStack_12c = _UNK_142a96724;
              uStack_128 = CONCAT44(_UNK_142a9672c,_UNK_142a96728);
              uStack_160 = 0x100000003;
              uStack_178 = 0;
              uStack_170 = 1;
              uStack_168 = 0;
              uStack_62 = 1;
              lStack_198 = lVar6;
              uStack_100 = uVar12;
              uStack_f8 = uVar8;
              FUN_140905be0(auStack_600,&uStack_1a0,auStack_9e8,0);
              if (*(char *)(lVar9 + 100) == '\0') {
                FUN_1428d9310(&UNK_142a92aa8);
                goto code_r0x0001408354fd;
              }
              uStack_54f = *(undefined1 *)(lVar9 + 0x65);
              func_0x000142923c00(&uStack_1a0,auStack_600,0xf8);
              uStack_62 = 1;
              FUN_14090b460(auStack_bd8,&uStack_1a0,&fStack_1f0);
              pcVar7 = *(code **)(lVar9 + 0x70);
              bStack_b2f = bStack_b2f & 0x8e | 0x21;
              func_0x000142923c00(auStack_600,auStack_bd8,0xf8);
              uStack_62 = 1;
              (*pcVar7)(auStack_600,lVar9,plStack_70);
            }
            func_0x000142923c00(auStack_600,auStack_9e8,1000);
            uStack_62 = 0;
            FUN_140868370(auStack_600);
            uVar12 = func_0x000140bd5790(lStack_a0);
            auStack_600[0] = 0;
            lStack_5f8 = lStack_80;
            lStack_5c8 = lStack_80 + 0x48;
            uStack_5e0 = 1;
            uStack_5dc = 0;
            uStack_5d8 = 0;
            uStack_5d4 = 0;
            uStack_5d0 = 0;
            uStack_1a0 = 0;
            lStack_198 = lStack_98;
            uStack_190 = 0;
            lStack_80 = lStack_5c8;
            FUN_140867970(auStack_9e8,&UNK_142a92a15,0x11,auStack_600,2,&uStack_1a0,uVar12,
                          plStack_70,uStack_90);
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
                lVar9 = *(longlong *)(param_5 + 0x50);
                lVar15 = *(longlong *)(param_5 + 0x58) * 0x80 + lVar9;
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
                    goto code_r0x0001408354fd;
                  }
                  uStack_f0 = 0;
                  uStack_f4 = 0x3f800000;
                  uStack_1a0 = 1;
                  uStack_190 = 0;
                  uStack_188 = 4;
                  uStack_180 = 0;
                  uStack_110 = (undefined4)*plVar13;
                  uStack_10c = *(undefined4 *)((longlong)plVar13 + 4);
                  uStack_108 = (undefined4)plVar13[1];
                  uStack_104 = *(undefined4 *)((longlong)plVar13 + 0xc);
                  *plVar13 = *plVar13 + 1;
                  uStack_120 = _UNK_142a96730;
                  uStack_118 = _UNK_142a96738;
                  uStack_130 = _UNK_142a96720;
                  uStack_12c = _UNK_142a96724;
                  uStack_128 = CONCAT44(_UNK_142a9672c,_UNK_142a96728);
                  uStack_160 = 0x100000003;
                  uStack_178 = 0;
                  uStack_170 = 1;
                  uStack_168 = 0;
                  uStack_63 = 1;
                  lStack_198 = lVar6;
                  uStack_100 = uVar12;
                  uStack_f8 = uVar8;
                  FUN_140905be0(auStack_600,&uStack_1a0,auStack_9e8,0);
                  if (*(char *)(lVar9 + 100) == '\0') {
                    FUN_1428d9310(&UNK_142a92a90);
                    goto code_r0x0001408354fd;
                  }
                  uStack_54f = *(undefined1 *)(lVar9 + 0x65);
                  func_0x000142923c00(&uStack_1a0,auStack_600,0xf8);
                  uStack_63 = 1;
                  FUN_14090b460(auStack_ec0,&uStack_1a0,&fStack_1f0);
                  pcVar7 = *(code **)(lVar9 + 0x70);
                  bStack_e17 = bStack_e17 & 0x8e | 0x21;
                  func_0x000142923c00(auStack_600,auStack_ec0,0xf8);
                  uStack_63 = 1;
                  (*pcVar7)(auStack_600,lVar9,plStack_70);
                }
                func_0x000142923c00(auStack_600,auStack_9e8,1000);
                uStack_63 = 0;
                FUN_140868370(auStack_600);
                uVar14 = 0xc95ed87f413070ab;
                lVar9 = FUN_1400dd6f0(plStack_88,0xc95ed87f413070ab,0xe67d7e3df983f8ce);
                plVar13 = plStack_70;
                if (((lVar9 != 0) && (uVar14 < (ulonglong)plStack_70[0x43])) &&
                   (uVar14 = *(ulonglong *)(plStack_70[0x42] + uVar14 * 8), uVar14 != 0)) {
                  lVar9 = ~uVar14 * 0x60;
                  if (*(longlong *)(plStack_70[0x3c] + 0x20 + lVar9) != 0) {
                    if (**(int **)(plStack_70[0x3c] + lVar9 + 0x10) == 0) {
                      uVar12 = func_0x000140bd5790(lStack_a0);
                      auStack_600[0] = 0;
                      lStack_5f8 = lStack_a8;
                      uStack_1a0 = 0;
                      lStack_198 = lStack_98;
                      uStack_190 = 0;
                      FUN_140867970(auStack_9e8,&UNK_142a92a40,7,auStack_600,1,&uStack_1a0,uVar12,
                                    plVar13,uStack_90);
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
                          lVar9 = *(longlong *)(param_5 + 0x68);
                          lVar15 = *(longlong *)(param_5 + 0x70) * 0x80 + lVar9;
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
                              goto code_r0x0001408354fd;
                            }
                            uStack_f0 = 0;
                            uStack_f4 = 0x3f800000;
                            uStack_1a0 = 1;
                            uStack_190 = 0;
                            uStack_188 = 4;
                            uStack_180 = 0;
                            uStack_110 = (undefined4)*plVar13;
                            uStack_10c = *(undefined4 *)((longlong)plVar13 + 4);
                            uStack_108 = (undefined4)plVar13[1];
                            uStack_104 = *(undefined4 *)((longlong)plVar13 + 0xc);
                            *plVar13 = *plVar13 + 1;
                            uStack_120 = _UNK_142a96730;
                            uStack_118 = _UNK_142a96738;
                            uStack_130 = _UNK_142a96720;
                            uStack_12c = _UNK_142a96724;
                            uStack_128 = CONCAT44(_UNK_142a9672c,_UNK_142a96728);
                            uStack_160 = 0x100000003;
                            uStack_178 = 0;
                            uStack_170 = 1;
                            uStack_168 = 0;
                            uStack_64 = 1;
                            lStack_198 = lVar6;
                            uStack_100 = uVar12;
                            uStack_f8 = uVar8;
                            FUN_140905be0(auStack_600,&uStack_1a0,auStack_9e8,0);
                            if (*(char *)(lVar9 + 100) == '\0') {
                              FUN_1428d9310(&UNK_142a92a78);
                              goto code_r0x0001408354fd;
                            }
                            uStack_54f = *(undefined1 *)(lVar9 + 0x65);
                            func_0x000142923c00(&uStack_1a0,auStack_600,0xf8);
                            uStack_64 = 1;
                            FUN_14090b460(auStack_cd0,&uStack_1a0,&fStack_1f0);
                            pcVar7 = *(code **)(lVar9 + 0x70);
                            bStack_c27 = bStack_c27 & 0x8e | 0x21;
                            func_0x000142923c00(auStack_600,auStack_cd0,0xf8);
                            uStack_64 = 1;
                            (*pcVar7)(auStack_600,lVar9,plStack_70);
                          }
                          func_0x000142923c00(auStack_600,auStack_9e8,1000);
                          uStack_64 = 0;
                          FUN_140868370(auStack_600);
                          uVar12 = func_0x000140bd5790(lStack_a0);
                          auStack_600[0] = 0;
                          lStack_5f8 = lStack_a8;
                          uStack_5e0 = 0;
                          uStack_5d8 = (undefined4)lStack_80;
                          uStack_5d4 = (undefined4)((ulonglong)lStack_80 >> 0x20);
                          uStack_1a0 = 0;
                          lStack_198 = lStack_98;
                          uStack_190 = 0;
                          FUN_140867970(auStack_9e8,&UNK_142a92a47,0xf,auStack_600,2,&uStack_1a0,
                                        uVar12,plStack_70,uStack_90);
                          uStack_65 = 1;
                          uVar14 = 0x4bc362195b80f729;
                          lVar9 = FUN_1400dd6f0(plStack_88,0x4bc362195b80f729,0x982798ce80cbcefb);
                          if (((lVar9 != 0) && (uVar14 < (ulonglong)plStack_70[0x43])) &&
                             (uVar14 = *(ulonglong *)(plStack_70[0x42] + uVar14 * 8), uVar14 != 0))
                          {
                            lVar9 = ~uVar14 * 0x60;
                            if (*(longlong *)(plStack_70[0x3c] + 0x20 + lVar9) != 0) {
                              puVar5 = *(ulonglong **)(plStack_70[0x3c] + lVar9 + 0x10);
                              fStack_1f0 = (float)(int)puVar5[1] / _UNK_1429a7640 + (float)*puVar5;
                              uStack_1ec = uStack_1ec & 0xffffff00;
                              lVar9 = *(longlong *)(param_5 + 0x80);
                              lVar15 = *(longlong *)(param_5 + 0x88) * 0x80 + lVar9;
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
                                  goto code_r0x0001408354fd;
                                }
                                uStack_f0 = 0;
                                uStack_f4 = 0x3f800000;
                                uStack_1a0 = 1;
                                uStack_190 = 0;
                                uStack_188 = 4;
                                uStack_180 = 0;
                                uStack_110 = (undefined4)*plVar13;
                                uStack_10c = *(undefined4 *)((longlong)plVar13 + 4);
                                uStack_108 = (undefined4)plVar13[1];
                                uStack_104 = *(undefined4 *)((longlong)plVar13 + 0xc);
                                *plVar13 = *plVar13 + 1;
                                uStack_120 = _UNK_142a96730;
                                uStack_118 = _UNK_142a96738;
                                uStack_130 = _UNK_142a96720;
                                uStack_12c = _UNK_142a96724;
                                uStack_128 = CONCAT44(_UNK_142a9672c,_UNK_142a96728);
                                uStack_160 = 0x100000003;
                                uStack_178 = 0;
                                uStack_170 = 1;
                                uStack_168 = 0;
                                uStack_65 = 1;
                                lStack_198 = lVar6;
                                uStack_100 = uVar12;
                                uStack_f8 = uVar8;
                                FUN_140905be0(auStack_600,&uStack_1a0,auStack_9e8,0);
                                if (*(char *)(lVar9 + 100) == '\0') {
                                  FUN_1428d9310(&UNK_142a92a60);
                                  goto code_r0x0001408354fd;
                                }
                                uStack_54f = *(undefined1 *)(lVar9 + 0x65);
                                func_0x000142923c00(&uStack_1a0,auStack_600,0xf8);
                                uStack_65 = 1;
                                FUN_14090b460(auStack_dc8,&uStack_1a0,&fStack_1f0);
                                pcVar7 = *(code **)(lVar9 + 0x70);
                                bStack_d1f = bStack_d1f & 0x8e | 0x21;
                                func_0x000142923c00(auStack_600,auStack_dc8,0xf8);
                                uStack_65 = 1;
                                (*pcVar7)(auStack_600,lVar9,plStack_70);
                              }
                              func_0x000142923c00(auStack_600,auStack_9e8,1000);
                              uStack_65 = 0;
                              FUN_140868370(auStack_600);
                              goto code_r0x000140834fab;
                            }
                          }
                          uStack_65 = 1;
                          FUN_1428d9310(&UNK_142aa5bd8);
                          goto code_r0x0001408354fd;
                        }
                      }
                      uStack_64 = 1;
                      FUN_1428d9310(&UNK_142aa5bd8);
                      goto code_r0x0001408354fd;
                    }
code_r0x000140834fab:
                    plVar13 = plStack_70;
                    uVar12 = func_0x000140bd5790(lStack_a0);
                    auStack_600[0] = 0;
                    lStack_5f8 = lStack_80;
                    uStack_1a0 = 1;
                    lStack_198 = lStack_98;
                    uStack_190 = 0;
                    FUN_140867970(auStack_9e8,&UNK_142a92a56,8,auStack_600,1,&uStack_1a0,uVar12,
                                  plVar13,uStack_90);
                    uStack_66 = 1;
                    uVar14 = 0x4bc362195b80f729;
                    lVar9 = FUN_1400dd6f0(plStack_88,0x4bc362195b80f729,0x982798ce80cbcefb);
                    if (((lVar9 != 0) && (uVar14 < (ulonglong)plStack_70[0x43])) &&
                       (uVar14 = *(ulonglong *)(plStack_70[0x42] + uVar14 * 8), uVar14 != 0)) {
                      lVar9 = ~uVar14 * 0x60;
                      if (*(longlong *)(plStack_70[0x3c] + 0x20 + lVar9) != 0) {
                        puVar5 = *(ulonglong **)(plStack_70[0x3c] + lVar9 + 0x10);
                        fStack_1a8 = (float)(int)puVar5[1] / _UNK_1429a7640 + (float)*puVar5;
                        uStack_1a4 = 0;
                        lVar9 = *(longlong *)(param_5 + 0x98);
                        lVar15 = *(longlong *)(param_5 + 0xa0) * 0x80 + lVar9;
                        for (; lVar9 != lVar15; lVar9 = lVar9 + 0x80) {
                          lVar6 = *(longlong *)(lVar9 + 0x50);
                          uVar12 = *(undefined8 *)(lVar9 + 0x58);
                          uStack_66 = 1;
                          uVar8 = func_0x000140be6750();
                          uStack_66 = 1;
                          plVar13 = (longlong *)FUN_1400d9b20(0);
                          if (plVar13 == (longlong *)0x0) {
                            uStack_66 = 1;
                            FUN_1428d80d0(&UNK_142c3aaa0);
                            goto code_r0x0001408354fd;
                          }
                          uStack_110 = (undefined4)*plVar13;
                          uStack_10c = *(undefined4 *)((longlong)plVar13 + 4);
                          uStack_108 = (undefined4)plVar13[1];
                          uStack_104 = *(undefined4 *)((longlong)plVar13 + 0xc);
                          *plVar13 = *plVar13 + 1;
                          uStack_1b8 = _UNK_142a96730;
                          uStack_1b0 = _UNK_142a96738;
                          uStack_1c8 = CONCAT44(_UNK_142a96724,_UNK_142a96720);
                          uStack_1c0 = CONCAT44(_UNK_142a9672c,_UNK_142a96728);
                          uStack_1a0 = 1;
                          uStack_190 = 0;
                          uStack_188 = 4;
                          uStack_180 = 0;
                          uStack_178 = 0;
                          uStack_170 = 1;
                          uStack_168 = 0;
                          uStack_160 = 0x100000003;
                          uStack_118 = _UNK_142a96738;
                          uStack_120 = _UNK_142a96730;
                          uStack_138 = uStack_1d0;
                          uStack_134 = uStack_1cc;
                          uStack_130 = _UNK_142a96720;
                          uStack_12c = _UNK_142a96724;
                          uStack_148 = uStack_1e0;
                          uStack_144 = uStack_1dc;
                          uStack_140 = uStack_1d8;
                          uStack_13c = uStack_1d4;
                          fStack_158 = fStack_1f0;
                          uStack_154 = uStack_1ec;
                          uStack_150 = uStack_1e8;
                          uStack_14c = uStack_1e4;
                          uStack_f8 = uVar8 & 0xffffff8f | 0x20;
                          uStack_f4 = 0x3f800000;
                          uStack_f0 = 0;
                          uStack_66 = 1;
                          lStack_198 = lVar6;
                          uStack_128 = uStack_1c0;
                          uStack_100 = uVar12;
                          FUN_140905be0(auStack_600,&uStack_1a0,auStack_9e8,0);
                          uStack_66 = 1;
                          FUN_14090b460(auStack_fb8,auStack_600,&fStack_1a8);
                          uStack_66 = 1;
                          (**(code **)(lVar9 + 0x70))(auStack_fb8,lVar9,plStack_70);
                        }
                        func_0x000142923c00(auStack_600,auStack_9e8,1000);
                        uStack_66 = 0;
                        FUN_140868370(auStack_600);
                        if (cStack_71 == '\0') {
                          return;
                        }
                        plVar13 = (longlong *)func_0x000140c18130();
                        if (*plVar13 == 1) {
                          plVar13 = plVar13 + 1;
                        }
                        else if ((*plVar13 != 0) ||
                                (plVar13 = (longlong *)FUN_1428f87d0(plVar13,0),
                                plVar13 == (longlong *)0x0)) goto code_r0x000140835376;
                        if (*plVar13 == 0) {
                          *plVar13 = -1;
                          plStack_70 = plVar13;
                          FUN_140c170f0(plVar13 + 1,uStack_1f8);
                          *plStack_70 = *plStack_70 + 1;
                          return;
                        }
                        goto code_r0x000140835382;
                      }
                    }
                    uStack_66 = 1;
                    FUN_1428d9310(&UNK_142aa5bd8);
                    goto code_r0x0001408354fd;
                  }
                }
                FUN_1428d9310(&UNK_142a92a28);
                goto code_r0x0001408354fd;
              }
            }
            uStack_63 = 1;
            FUN_1428d9310(&UNK_142aa5bd8);
            goto code_r0x0001408354fd;
          }
        }
        uStack_62 = 1;
        FUN_1428d9310(&UNK_142aa5bd8);
        goto code_r0x0001408354fd;
      }
    }
    uStack_61 = 1;
    FUN_1428d9310(&UNK_142aa5bd8);
  }
  else {
    puVar17 = &UNK_142a924f4;
    lVar15 = 0x45;
    uVar14 = 0x45;
    lVar9 = FUN_140832920(&UNK_142a924f4);
    if (lVar9 == 1) {
      if (uVar14 != 0) {
        if (uVar14 < 0x45) {
          if ((char)(&UNK_142a924f4)[uVar14] < -0x40) {
code_r0x000140833c53:
            FUN_1428d9940(&UNK_142a924f4,0x45,0,uVar14,&UNK_142a92608);
            goto code_r0x000140833c73;
          }
        }
        else if (uVar14 != 0x45) goto code_r0x000140833c53;
      }
      puVar17 = &UNK_142a924f4;
      lVar9 = FUN_140832920(&UNK_142a924f4);
      if (lVar9 == 1) {
        uVar16 = uVar14 + 2;
        if (uVar16 != 0) {
          uVar19 = uVar16 == 0x45;
          if (0x44 < uVar16) goto code_r0x000140833cd4;
          uVar19 = (&UNK_142a924f6)[uVar14] == -0x41;
          if ((char)(&UNK_142a924f6)[uVar14] < -0x40) {
            do {
              uVar14 = 0x45;
              FUN_1428d9940(&UNK_142a924f4,0x45);
code_r0x000140833cd4:
            } while (!(bool)uVar19);
          }
        }
        lVar15 = 0x43 - uVar14;
        puVar17 = &UNK_142a924f4 + uVar16;
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
        if (uVar8 != 0x5c) goto code_r0x000140833d79;
code_r0x000140833d8e:
        puVar11 = puVar10 + -0x142a929b8;
        puVar18 = puVar10 + -0x142a929b7;
        if (puVar18 != (undefined *)0x0) {
          uVar19 = puVar18 == (undefined *)0x44;
          if ((undefined *)0x43 < puVar18) goto code_r0x000140833dd0;
          uVar19 = puVar10[1] == -0x41;
          if ((char)puVar10[1] < -0x40) {
            do {
              puVar11 = (undefined *)FUN_1428d9940(&UNK_142a929b8,0x44);
code_r0x000140833dd0:
            } while (!(bool)uVar19);
          }
        }
        lVar9 = 0x43 - (longlong)puVar11;
        puVar18 = &UNK_142a929b8 + (longlong)puVar18;
        goto code_r0x000140833ddd;
      }
      puVar10 = puVar10 + -1;
      if (uVar8 == 0x5c) goto code_r0x000140833d8e;
code_r0x000140833d79:
      if (uVar8 == 0x2f) goto code_r0x000140833d8e;
    } while (puVar10 != &UNK_142a929b8);
    lVar9 = 0x44;
code_r0x000140833ddd:
    plVar13 = (longlong *)func_0x000140c18130();
    if (*plVar13 == 1) {
      plVar13 = plVar13 + 1;
code_r0x000140833df6:
      if (*plVar13 == 0) {
        *plVar13 = -1;
        plStack_70 = plVar13;
        uStack_210 = FUN_140c16d40(plVar13 + 1,puVar17,lVar15,puVar18,lVar9,1,0);
        *plStack_70 = *plStack_70 + 1;
        uStack_218 = 1;
        uStack_1f8 = uStack_210;
        goto code_r0x000140833e54;
      }
    }
    else {
      if ((*plVar13 == 0) &&
         (plVar13 = (longlong *)FUN_1428f87d0(plVar13,0), plVar13 != (longlong *)0x0))
      goto code_r0x000140833df6;
code_r0x000140835376:
      FUN_1428d80d0(&UNK_142c3aaa0);
    }
code_r0x000140835382:
    FUN_1428d9250(&UNK_142c3ab70);
code_r0x00014083538e:
    uStack_61 = 1;
    FUN_1428d80d0(&UNK_142c3aaa0);
  }
code_r0x0001408354fd:
                    /* WARNING: Does not return */
  pcVar7 = (code *)invalidInstructionException();
  (*pcVar7)();
}

