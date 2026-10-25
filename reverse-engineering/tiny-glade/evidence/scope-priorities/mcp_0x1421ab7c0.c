
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void FUN_1421ab7c0(undefined8 *param_1,longlong *param_2,undefined8 *param_3,longlong *param_4,
                  longlong *param_5,longlong *param_6,longlong *param_7)

{
  undefined4 *puVar1;
  undefined4 uVar2;
  undefined4 uVar3;
  undefined4 uVar4;
  longlong lVar5;
  longlong lVar6;
  longlong lVar7;
  code *pcVar8;
  float fVar9;
  float fVar10;
  undefined1 uVar11;
  byte bVar12;
  byte bVar13;
  undefined1 uVar14;
  ulonglong uVar15;
  int *piVar16;
  ulonglong uVar17;
  longlong lVar18;
  longlong lVar19;
  undefined1 uVar20;
  int *piVar21;
  byte bVar22;
  ulonglong uVar23;
  ulonglong uVar24;
  int *piVar25;
  undefined8 uVar26;
  int *piVar27;
  longlong lVar28;
  undefined8 *puVar29;
  undefined8 uVar30;
  float fVar31;
  float fVar32;
  float fVar33;
  float fVar34;
  uint uVar35;
  uint uVar37;
  undefined1 auVar36 [16];
  float fVar38;
  undefined1 auVar39 [16];
  undefined1 auVar40 [16];
  undefined1 auVar41 [16];
  float fVar42;
  float fVar43;
  float fVar44;
  float fVar45;
  float fVar46;
  undefined8 uStack_1e8;
  undefined8 uStack_1e0;
  undefined8 uStack_1d8;
  undefined8 uStack_1d0;
  undefined8 uStack_1c8;
  longlong lStack_1b8;
  undefined4 *puStack_1b0;
  longlong lStack_1a8;
  ulonglong uStack_1a0;
  undefined4 uStack_194;
  float fStack_190;
  float fStack_18c;
  float fStack_188;
  undefined4 uStack_184;
  undefined4 *puStack_180;
  longlong lStack_178;
  longlong lStack_170;
  longlong lStack_168;
  int *piStack_160;
  int *piStack_158;
  longlong lStack_150;
  longlong lStack_148;
  longlong lStack_140;
  undefined4 *puStack_138;
  int *piStack_130;
  int *piStack_128;
  ulonglong uStack_120;
  undefined4 *puStack_118;
  undefined8 uStack_110;
  ulonglong *puStack_108;
  int *piStack_100;
  int *piStack_f8;
  code *pcStack_f0;
  
  puStack_108 = (ulonglong *)*param_3;
  lVar5 = param_3[1];
  uVar17 = *puStack_108;
  uVar23 = 0;
  uVar15 = uVar17 - *(ulonglong *)(lVar5 + 0x18);
  if (uVar17 < *(ulonglong *)(lVar5 + 0x18)) {
    uVar15 = uVar23;
  }
  uVar24 = uVar17 - *(ulonglong *)(lVar5 + 0x38);
  if (uVar17 < *(ulonglong *)(lVar5 + 0x38)) {
    uVar24 = uVar23;
  }
  piVar27 = (int *)(uVar15 * 0x40 + *(longlong *)(lVar5 + 8));
  uVar17 = *(ulonglong *)(lVar5 + 0x10) - uVar15;
  if (*(ulonglong *)(lVar5 + 0x10) < uVar15) {
    piVar27 = (int *)0x8;
    uVar17 = uVar23;
  }
  piVar16 = (int *)(uVar24 * 0x40 + *(longlong *)(lVar5 + 0x28));
  uVar15 = *(ulonglong *)(lVar5 + 0x30) - uVar24;
  if (*(ulonglong *)(lVar5 + 0x30) < uVar24) {
    piVar16 = (int *)0x8;
    uVar15 = uVar23;
  }
  *puStack_108 = (*(longlong *)(lVar5 + 0x40) - uVar15) - uVar17;
  fVar10 = _UNK_142925984;
  fVar9 = _UNK_142925904;
  piStack_128 = piVar27 + uVar17 * 0x10;
  piStack_130 = piVar16 + uVar15 * 0x10;
  uStack_110 = *param_1;
  lVar5 = *param_2;
  lVar28 = param_2[1];
  uVar2 = *(undefined4 *)((longlong)param_2 + 0x14);
  uStack_194 = *(undefined4 *)((longlong)param_6 + 0x1c);
  lStack_178 = *param_6;
  puStack_138 = (undefined4 *)param_6[2];
  uVar3 = *(undefined4 *)((longlong)param_7 + 0x1c);
  lVar6 = *param_7;
  puStack_1b0 = (undefined4 *)param_7[2];
  uStack_184 = *(undefined4 *)((longlong)param_5 + 0x1c);
  lStack_168 = *param_5;
  puStack_118 = (undefined4 *)param_5[2];
  uVar4 = *(undefined4 *)((longlong)param_4 + 0x1c);
  lStack_1a8 = *param_4;
  puStack_180 = (undefined4 *)param_4[2];
  lStack_150 = lStack_178 + 0x20;
  lStack_1b8 = lVar6 + 0x20;
  lStack_140 = lStack_168 + 0x20;
  lStack_148 = lStack_1a8 + 0x20;
  lStack_170 = lVar28;
code_r0x0001421ab9fc:
  if ((piVar27 == (int *)0x0) || (piVar27 == piStack_128)) {
    if ((piVar16 == (int *)0x0) || (piVar16 == piStack_130)) {
      return;
    }
    piVar27 = piVar16 + 0x10;
    piVar25 = (int *)0x0;
    piVar21 = piVar16;
  }
  else {
    piVar25 = piVar27 + 0x10;
    piVar21 = piVar27;
    piVar27 = piVar16;
  }
  *puStack_108 = *puStack_108 + 1;
  uVar17 = FUN_140a922b0(uStack_110,*(undefined8 *)(piVar21 + 8));
  if (uVar17 >> 0x20 != 0) {
    uVar15 = uVar17 & 0xffffffff;
    if ((uVar15 < *(ulonglong *)(lVar28 + 0x10)) &&
       (*(int *)(*(longlong *)(lVar28 + 8) + uVar15 * 0x14) == (int)(uVar17 >> 0x20))) {
      lVar18 = *(longlong *)(lVar28 + 8) + uVar15 * 0x14;
      uVar15 = (ulonglong)*(uint *)(lVar18 + 4);
      if (uVar15 == 0xffffffff) goto code_r0x0001421ac36c;
      piStack_160 = piVar25;
      piStack_158 = piVar27;
      if ((uVar15 < *(ulonglong *)(lVar5 + 0x38)) &&
         ((*(ulonglong *)(*(longlong *)(lVar5 + 0x28) + (ulonglong)(*(uint *)(lVar18 + 4) >> 6) * 8)
           >> (uVar15 & 0x3f) & 1) != 0)) {
        uVar15 = (ulonglong)*(uint *)(lVar18 + 0x10);
        lVar7 = *(longlong *)
                 (*(longlong *)(lVar28 + 0x1a8) + 0x18 + (ulonglong)*(uint *)(lVar18 + 0xc) * 0x48);
        lVar18 = ~*(ulonglong *)
                   (*(longlong *)
                     (*(longlong *)(lVar28 + 0x1a8) + 0x38 +
                     (ulonglong)*(uint *)(lVar18 + 0xc) * 0x48) + *(longlong *)(lVar5 + 0x110) * 8)
                 * 0x30;
        puVar29 = (undefined8 *)(uVar15 * 0x58 + *(longlong *)(lVar7 + 0x10 + lVar18));
        lVar28 = *(longlong *)(lVar7 + 0x28 + lVar18);
        uStack_120 = uVar15 * 4 + *(longlong *)(lVar7 + 0x20 + lVar18);
        fVar42 = 0.0;
        fVar43 = 0.0;
        if (*piVar21 != 0) {
          if (*piVar21 == 1) {
            fVar43 = (float)piVar21[1];
            fVar31 = 0.0;
            if (0.0 <= fVar43) {
              fVar31 = fVar43;
            }
            fVar32 = fVar9;
            if (fVar31 <= fVar9) {
              fVar32 = fVar31;
            }
            fVar43 = fVar43 - *(float *)((longlong)puVar29 + 0x2c);
            *(undefined4 *)(lVar28 + uVar15 * 4) = uVar2;
            *(float *)((longlong)puVar29 + 0x2c) = fVar32;
          }
          else {
            fVar43 = (float)piVar21[1];
            *(undefined4 *)(lVar28 + uVar15 * 4) = uVar2;
            fVar32 = *(float *)((longlong)puVar29 + 0x2c) + fVar43;
            fVar31 = 0.0;
            if (0.0 <= fVar32) {
              fVar31 = fVar32;
            }
            fVar32 = fVar9;
            if (fVar31 <= fVar9) {
              fVar32 = fVar31;
            }
            *(float *)((longlong)puVar29 + 0x2c) = fVar32;
          }
        }
        fVar31 = *(float *)(puVar29 + 6);
        if (piVar21[2] == 0) {
          bVar22 = 0;
          fVar32 = fVar31;
        }
        else {
          fVar32 = fVar9;
          if (piVar21[2] == 2) {
            fVar42 = (float)piVar21[3];
            *(undefined4 *)(lVar28 + uVar15 * 4) = uVar2;
            fVar44 = fVar10;
            if (fVar10 <= *(float *)(puVar29 + 6) + fVar42) {
              fVar44 = *(float *)(puVar29 + 6) + fVar42;
            }
            if (fVar44 <= fVar9) {
              fVar32 = fVar44;
            }
            *(float *)(puVar29 + 6) = fVar32;
          }
          else {
            fVar42 = (float)piVar21[3];
            fVar44 = 0.0;
            if (0.0 <= fVar42) {
              fVar44 = fVar42;
            }
            if (fVar44 <= fVar9) {
              fVar32 = fVar44;
            }
            *(undefined4 *)(lVar28 + uVar15 * 4) = uVar2;
            *(float *)(puVar29 + 6) = fVar32;
            fVar42 = fVar42 - fVar31;
          }
          bVar22 = 1;
        }
        fVar44 = 0.0;
        fVar46 = 0.0;
        if (piVar21[10] != 0) {
          if (piVar21[10] == 2) {
            fVar46 = (float)*(undefined8 *)(piVar21 + 0xb);
            fVar45 = (float)((ulonglong)*(undefined8 *)(piVar21 + 0xb) >> 0x20);
            *(undefined4 *)(lVar28 + uVar15 * 4) = uVar2;
            fVar33 = (float)*(undefined8 *)((longlong)puVar29 + 0x24) + fVar46;
            fVar34 = (float)((ulonglong)*(undefined8 *)((longlong)puVar29 + 0x24) >> 0x20) + fVar45;
            fVar38 = fVar34 * fVar34 + fVar33 * fVar33;
            uVar37 = (uint)(fVar9 < fVar38);
            auVar36._0_4_ = SQRT(fVar38);
            auVar36._4_4_ = auVar36._0_4_;
            auVar36._8_8_ = 0;
            auVar39._4_4_ = fVar34;
            auVar39._0_4_ = fVar33;
            auVar39._8_8_ = 0;
            auVar40 = divps(auVar39,auVar36);
            uVar35 = (int)(uVar37 << 0x1f) >> 0x1f;
            uVar37 = (int)(uVar37 << 0x1f) >> 0x1f;
            *(ulonglong *)((longlong)puVar29 + 0x24) =
                 CONCAT44(~uVar37 & (uint)fVar34,~uVar35 & (uint)fVar33) |
                 CONCAT44(auVar40._4_4_ & uVar37,auVar40._0_4_ & uVar35);
          }
          else {
            uVar26 = *(undefined8 *)(piVar21 + 0xb);
            uVar23 = *(ulonglong *)((longlong)puVar29 + 0x24);
            fVar46 = (float)uVar23;
            fVar45 = (float)(uVar23 >> 0x20);
            fVar33 = fVar45 * fVar45 + fVar46 * fVar46;
            uVar37 = (uint)(fVar9 < fVar33);
            auVar40._0_4_ = SQRT(fVar33);
            auVar40._4_4_ = auVar40._0_4_;
            auVar40._8_8_ = 0;
            auVar41._8_8_ = 0;
            auVar41._0_8_ = uVar23;
            auVar40 = divps(auVar41,auVar40);
            uVar35 = (int)(uVar37 << 0x1f) >> 0x1f;
            uVar37 = (int)(uVar37 << 0x1f) >> 0x1f;
            uVar23 = CONCAT44(~uVar37 & (uint)fVar45,~uVar35 & (uint)fVar46) |
                     CONCAT44(auVar40._4_4_ & uVar37,auVar40._0_4_ & uVar35);
            *(undefined4 *)(lVar28 + uVar15 * 4) = uVar2;
            *(undefined8 *)((longlong)puVar29 + 0x24) = uVar26;
            fVar46 = (float)uVar26 - (float)uVar23;
            fVar45 = (float)((ulonglong)uVar26 >> 0x20) - (float)(uVar23 >> 0x20);
          }
          fVar46 = SQRT(fVar45 * fVar45 + fVar46 * fVar46);
        }
        if (piVar21[6] != 0) {
          if (piVar21[6] == 2) {
            fVar44 = (float)piVar21[7];
            *(undefined4 *)(lVar28 + uVar15 * 4) = uVar2;
            fVar45 = 0.0;
            if (0.0 <= *(float *)(puVar29 + 7) + fVar44) {
              fVar45 = *(float *)(puVar29 + 7) + fVar44;
            }
            fVar33 = fVar9;
            if (fVar45 <= fVar9) {
              fVar33 = fVar45;
            }
            *(float *)(puVar29 + 7) = fVar33;
          }
          else {
            fVar44 = (float)piVar21[7];
            fVar45 = 0.0;
            if (0.0 <= *(float *)(puVar29 + 7)) {
              fVar45 = *(float *)(puVar29 + 7);
            }
            fVar33 = fVar9;
            if (fVar45 <= fVar9) {
              fVar33 = fVar45;
            }
            *(undefined4 *)(lVar28 + uVar15 * 4) = uVar2;
            *(float *)(puVar29 + 7) = fVar44;
            fVar44 = fVar44 - fVar33;
          }
        }
        uStack_1a0 = uVar17;
        piStack_100 = piVar21 + 8;
        bVar12 = func_0x0001408e33b0(puVar29);
        if (piVar21[4] == 0) {
          fVar45 = 0.0;
          bVar13 = bVar12;
        }
        else if (piVar21[4] == 2) {
          fVar45 = (float)piVar21[5];
          *(undefined4 *)(lVar28 + uVar15 * 4) = uVar2;
          fVar34 = *(float *)((longlong)puVar29 + 0x34) + fVar45;
          fVar33 = 0.0;
          if (0.0 <= fVar34) {
            fVar33 = fVar34;
          }
          fVar34 = fVar9;
          if (fVar33 <= fVar9) {
            fVar34 = fVar33;
          }
          *(float *)((longlong)puVar29 + 0x34) = fVar34;
          bVar13 = func_0x0001408e33b0(puVar29);
        }
        else {
          fVar45 = (float)piVar21[5];
          fVar33 = 0.0;
          if (0.0 <= *(float *)((longlong)puVar29 + 0x34)) {
            fVar33 = *(float *)((longlong)puVar29 + 0x34);
          }
          fVar34 = fVar9;
          if (fVar33 <= fVar9) {
            fVar34 = fVar33;
          }
          *(undefined4 *)(lVar28 + uVar15 * 4) = uVar2;
          *(float *)((longlong)puVar29 + 0x34) = fVar45;
          bVar13 = func_0x0001408e33b0(puVar29);
          fVar45 = fVar45 - fVar34;
        }
        if (((bVar12 == bVar13) && (((fVar43 != 0.0 | bVar22) & bVar13) == 0)) &&
           ((fVar44 != 0.0 & bVar13) == 0)) {
          if (bVar12 != 0) goto code_r0x0001421abe71;
code_r0x0001421abf1e:
          lVar28 = lStack_170;
          uVar17 = uStack_1a0;
          if (bVar13 != 0) {
            func_0x0001408e3220(&uStack_1d8,puVar29);
            uStack_1e8 = (undefined *)CONCAT71(uStack_1e8._1_7_,10);
            goto code_r0x0001421abf48;
          }
        }
        else {
          uVar26 = *puVar29;
          *puStack_138 = uStack_194;
          lVar28 = *(longlong *)(lStack_178 + 0x30);
          uVar30 = *(undefined8 *)(lStack_178 + 0x40);
          if (lVar28 == *(longlong *)(lStack_178 + 0x20)) {
            FUN_140562630(lStack_150,&UNK_142b72d70);
          }
          lVar18 = *(longlong *)(lStack_178 + 0x28);
          *(undefined8 *)(lVar18 + lVar28 * 0x10) = uVar30;
          *(undefined8 *)(lVar18 + 8 + lVar28 * 0x10) = uVar26;
          *(longlong *)(lStack_178 + 0x30) = lVar28 + 1;
          *(longlong *)(lStack_178 + 0x40) = *(longlong *)(lStack_178 + 0x40) + 1;
          if (bVar12 == 0) goto code_r0x0001421abf1e;
code_r0x0001421abe71:
          lVar28 = lStack_170;
          uVar17 = uStack_1a0;
          if (bVar13 == 0) {
            func_0x0001408e3220(&uStack_1d8,puVar29);
            uStack_1e8 = (undefined *)CONCAT71(uStack_1e8._1_7_,0xb);
code_r0x0001421abf48:
            *puStack_1b0 = uVar3;
            lVar18 = *(longlong *)(lVar6 + 0x30);
            uVar26 = *(undefined8 *)(lVar6 + 0x40);
            if (lVar18 == *(longlong *)(lVar6 + 0x20)) {
              FUN_140562ab0(lStack_1b8,&UNK_142b72d70);
            }
            lVar7 = *(longlong *)(lVar6 + 0x28);
            *(undefined8 *)(lVar7 + lVar18 * 0x28) = uVar26;
            puVar1 = (undefined4 *)(lVar7 + 0x14 + lVar18 * 0x28);
            *puVar1 = uStack_1e0._4_4_;
            puVar1[1] = (undefined4)uStack_1d8;
            puVar1[2] = uStack_1d8._4_4_;
            puVar1[3] = (undefined4)uStack_1d0;
            puVar1 = (undefined4 *)(lVar7 + 8 + lVar18 * 0x28);
            *puVar1 = (undefined4)uStack_1e8;
            puVar1[1] = uStack_1e8._4_4_;
            puVar1[2] = (undefined4)uStack_1e0;
            puVar1[3] = uStack_1e0._4_4_;
            *(longlong *)(lVar6 + 0x30) = lVar18 + 1;
            *(longlong *)(lVar6 + 0x40) = *(longlong *)(lVar6 + 0x40) + 1;
          }
        }
        uVar37 = _UNK_14292e980;
        if (((fVar44 != 0.0) || (fVar43 != 0.0)) || ((fVar42 != 0.0 || (fVar45 != 0.0)))) {
          uVar35 = (uint)fVar42 & _UNK_14292e980;
          fStack_190 = fVar46;
          fStack_18c = fVar32;
          fStack_188 = fVar31;
          fVar42 = (float)func_0x000142923e60(uVar35);
          fVar31 = (float)func_0x000142923e60((uint)fVar43 & uVar37);
          fVar32 = (float)func_0x000142923e60((uint)fVar45 & uVar37);
          fVar46 = (float)func_0x000142923e60((uint)fVar44 & uVar37);
          func_0x0001408e3220(&uStack_1d8,puVar29);
          uStack_1e8._0_2_ = CONCAT11(bVar13,2);
          uStack_1e8 = (undefined *)CONCAT44(fVar42,(undefined4)uStack_1e8);
          uStack_1e0 = CONCAT44(~-(uint)(0.0 < fVar42) &
                                (uint)((float)((uint)fVar45 & uVar37) +
                                       (float)((uint)fVar43 & uVar37) +
                                      (float)((uint)fVar44 & uVar37)) |
                                uVar35 & -(uint)(0.0 < fVar42),fVar46 + fVar32 + fVar31);
          *puStack_1b0 = uVar3;
          lVar18 = *(longlong *)(lVar6 + 0x30);
          uVar26 = *(undefined8 *)(lVar6 + 0x40);
          if (lVar18 == *(longlong *)(lVar6 + 0x20)) {
            FUN_140562ab0(lStack_1b8,&UNK_142b72d70);
          }
          lVar7 = *(longlong *)(lVar6 + 0x28);
          *(undefined8 *)(lVar7 + lVar18 * 0x28) = uVar26;
          puVar1 = (undefined4 *)(lVar7 + 0x14 + lVar18 * 0x28);
          *puVar1 = uStack_1e0._4_4_;
          puVar1[1] = (undefined4)uStack_1d8;
          puVar1[2] = uStack_1d8._4_4_;
          puVar1[3] = (undefined4)uStack_1d0;
          puVar1 = (undefined4 *)(lVar7 + 8 + lVar18 * 0x28);
          *puVar1 = (undefined4)uStack_1e8;
          puVar1[1] = uStack_1e8._4_4_;
          puVar1[2] = (undefined4)uStack_1e0;
          puVar1[3] = uStack_1e0._4_4_;
          *(longlong *)(lVar6 + 0x30) = lVar18 + 1;
          *(longlong *)(lVar6 + 0x40) = *(longlong *)(lVar6 + 0x40) + 1;
          fVar46 = fStack_190;
          fVar32 = fStack_18c;
          fVar31 = fStack_188;
        }
        if (0.0 < fVar46) {
          fVar43 = *(float *)((longlong)puVar29 + 0x24);
          fVar42 = *(float *)(puVar29 + 5);
          func_0x0001408e3220(&uStack_1d8,puVar29);
          uStack_1e8 = (undefined *)CONCAT71(uStack_1e8._1_7_,3);
          uStack_1e8 = (undefined *)
                       CONCAT44(SQRT(fVar42 * fVar42 + fVar43 * fVar43),(undefined4)uStack_1e8);
          uStack_1e0 = CONCAT44(uStack_1e0._4_4_,fVar46);
          *puStack_1b0 = uVar3;
          lVar18 = *(longlong *)(lVar6 + 0x30);
          uVar26 = *(undefined8 *)(lVar6 + 0x40);
          if (lVar18 == *(longlong *)(lVar6 + 0x20)) {
            FUN_140562ab0(lStack_1b8,&UNK_142b72d70);
          }
          lVar7 = *(longlong *)(lVar6 + 0x28);
          *(undefined8 *)(lVar7 + lVar18 * 0x28) = uVar26;
          puVar1 = (undefined4 *)(lVar7 + 0x14 + lVar18 * 0x28);
          *puVar1 = uStack_1e0._4_4_;
          puVar1[1] = (undefined4)uStack_1d8;
          puVar1[2] = uStack_1d8._4_4_;
          puVar1[3] = (undefined4)uStack_1d0;
          puVar1 = (undefined4 *)(lVar7 + 8 + lVar18 * 0x28);
          *puVar1 = (undefined4)uStack_1e8;
          puVar1[1] = uStack_1e8._4_4_;
          puVar1[2] = (undefined4)uStack_1e0;
          puVar1[3] = uStack_1e0._4_4_;
          *(longlong *)(lVar6 + 0x30) = lVar18 + 1;
          *(longlong *)(lVar6 + 0x40) = *(longlong *)(lVar6 + 0x40) + 1;
        }
        uVar26 = *(undefined8 *)piStack_100;
        *puStack_118 = uStack_184;
        lVar18 = *(longlong *)(lStack_168 + 0x30);
        uVar30 = *(undefined8 *)(lStack_168 + 0x40);
        if (lVar18 == *(longlong *)(lStack_168 + 0x20)) {
          FUN_140562c30(lStack_140,&UNK_142b72d70);
        }
        lVar7 = *(longlong *)(lStack_168 + 0x28);
        lVar19 = lVar18 * 0x20;
        *(undefined8 *)(lVar7 + lVar19) = uVar30;
        *(ulonglong *)(lVar7 + 8 + lVar19) = uVar17;
        *(undefined8 *)(lVar7 + 0x10 + lVar19) = uVar26;
        *(undefined1 *)(lVar7 + 0x18 + lVar19) = 1;
        *(longlong *)(lStack_168 + 0x30) = lVar18 + 1;
        *(longlong *)(lStack_168 + 0x40) = *(longlong *)(lStack_168 + 0x40) + 1;
        if ((fVar31 <= fVar10) || (fVar10 < fVar32)) goto code_r0x0001421ac280;
        uVar26 = *(undefined8 *)(piVar21 + 8);
        uVar20 = (undefined1)piVar21[0xd];
        *puStack_180 = uVar4;
        lVar18 = *(longlong *)(lStack_1a8 + 0x30);
        uVar30 = *(undefined8 *)(lStack_1a8 + 0x40);
        uVar14 = 0;
        uVar11 = 0;
        if (lVar18 == *(longlong *)(lStack_1a8 + 0x20)) goto code_r0x0001421ac2e5;
        goto code_r0x0001421ab9b0;
      }
      uStack_1e8 = (undefined *)(uVar15 << 0x20);
    }
    else {
code_r0x0001421ac36c:
      uStack_1e8 = (undefined *)((uStack_120 & 0xffffffff00000000) + 1);
    }
    uStack_1e0 = uVar17;
    FUN_1428d9760(&UNK_142ff74f0,0x2b,&uStack_1e8,&UNK_142ff74d0,&UNK_142ff7630);
  }
  pcStack_f0 = FUN_1421ab6c0;
  uStack_1e8 = &UNK_142ff7668;
  uStack_1e0 = 1;
  uStack_1c8 = 0;
  uStack_1d8 = &piStack_f8;
  uStack_1d0 = 1;
  piStack_f8 = piVar21 + 8;
  FUN_1428d9390(&uStack_1e8,&UNK_142ff7678);
  pcVar8 = (code *)swi(3);
  (*pcVar8)();
  return;
code_r0x0001421ac280:
  piVar16 = piStack_158;
  piVar27 = piStack_160;
  if ((fVar31 <= fVar10) && (fVar10 < fVar32)) {
    uVar26 = *(undefined8 *)(piVar21 + 8);
    uVar20 = (undefined1)piVar21[0xd];
    *puStack_180 = uVar4;
    lVar18 = *(longlong *)(lStack_1a8 + 0x30);
    uVar30 = *(undefined8 *)(lStack_1a8 + 0x40);
    uVar14 = 1;
    uVar11 = 1;
    if (lVar18 == *(longlong *)(lStack_1a8 + 0x20)) {
code_r0x0001421ac2e5:
      uVar14 = uVar11;
      FUN_140562c30(lStack_148,&UNK_142b72d70);
    }
code_r0x0001421ab9b0:
    lVar7 = *(longlong *)(lStack_1a8 + 0x28);
    lVar19 = lVar18 * 0x20;
    *(undefined8 *)(lVar7 + lVar19) = uVar30;
    *(undefined8 *)(lVar7 + 8 + lVar19) = uVar26;
    *(ulonglong *)(lVar7 + 0x10 + lVar19) = uStack_1a0;
    *(undefined1 *)(lVar7 + 0x18 + lVar19) = uVar20;
    *(undefined1 *)(lVar7 + 0x19 + lVar19) = uVar14;
    *(longlong *)(lStack_1a8 + 0x30) = lVar18 + 1;
    *(longlong *)(lStack_1a8 + 0x40) = *(longlong *)(lStack_1a8 + 0x40) + 1;
    piVar16 = piStack_158;
    piVar27 = piStack_160;
  }
  goto code_r0x0001421ab9fc;
}

