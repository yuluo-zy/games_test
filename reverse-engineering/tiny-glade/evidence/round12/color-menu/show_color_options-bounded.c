
/* WARNING: Type propagation algorithm not settling */
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */
/* WARNING: Unknown calling convention -- yet parameter storage is locked */

void show_color_options(void *p0,void *p1,void *p2,void *p3,void *p4,void *p5,void *p6,void *p7,
                       void *p8,void *p9,void *p10,void *p11,void *p12,void *p13,void *p14)

{
  float appearance_divisor;
  undefined1 auVar1 [16];
  undefined1 auVar2 [16];
  int iVar3;
  undefined4 uVar4;
  code *pcVar5;
  ushort uVar6;
  undefined1 auVar7 [16];
  undefined1 auVar8 [16];
  undefined1 auVar9 [16];
  undefined4 uVar10;
  float fVar11;
  undefined8 uVar12;
  byte bVar13;
  char cVar14;
  uint uVar15;
  longlong lVar16;
  ulonglong uVar17;
  undefined1 (*pauVar18) [16];
  undefined1 (*pauVar19) [16];
  undefined1 (*pauVar20) [16];
  undefined1 (*pauVar21) [16];
  undefined1 (*pauVar22) [16];
  undefined1 (*pauVar23) [16];
  undefined1 (*pauVar24) [16];
  undefined8 *puVar25;
  undefined8 *puVar26;
  char *pcVar27;
  byte *pbVar28;
  undefined4 *puVar29;
  ulonglong uVar30;
  undefined8 uVar31;
  longlong lVar32;
  uint *puVar33;
  longlong lVar34;
  undefined1 uVar35;
  bool bVar36;
  undefined1 uVar37;
  undefined *puVar38;
  ulonglong uVar39;
  ulonglong uVar40;
  uint uVar41;
  uint uVar42;
  undefined1 (*pauVar43) [16];
  undefined1 (*unaff_R13) [16];
  undefined1 uVar44;
  longlong *plVar45;
  undefined4 uVar46;
  undefined4 uVar47;
  float radius;
  float fVar48;
  float fVar49;
  float fVar50;
  float fVar51;
  float icon_size;
  float fVar52;
  float spacing;
  undefined1 auStack_304 [12];
  longlong lStack_2f8;
  undefined1 (*pauStack_2f0) [16];
  longlong lStack_2e8;
  undefined4 *puStack_2e0;
  undefined1 (*pauStack_2d8) [16];
  undefined1 (*pauStack_2d0) [16];
  undefined7 uStack_2c8;
  undefined4 uStack_2c1;
  undefined8 uStack_2b8;
  ulonglong *puStack_2b0;
  undefined1 (*pauStack_2a8) [16];
  undefined8 uStack_2a0;
  undefined8 *puStack_298;
  uint uStack_28c;
  uint uStack_288;
  undefined4 uStack_284;
  undefined1 (*pauStack_280) [16];
  undefined1 auStack_278 [8];
  longlong lStack_270;
  undefined1 (*pauStack_268) [16];
  undefined8 uStack_260;
  undefined1 (*pauStack_258) [16];
  undefined8 *puStack_250;
  undefined8 uStack_248;
  undefined1 (*pauStack_240) [16];
  undefined1 (*pauStack_238) [16];
  undefined8 *puStack_230;
  undefined8 *puStack_228;
  undefined1 (*pauStack_220) [16];
  undefined1 (*pauStack_218) [16];
  undefined8 uStack_210;
  undefined4 uStack_208;
  undefined4 uStack_204;
  undefined8 uStack_200;
  undefined8 *puStack_1f8;
  longlong lStack_1f0;
  undefined1 (*pauStack_1e8) [16];
  undefined1 (*pauStack_1e0) [16];
  undefined1 (*pauStack_1d8) [16];
  longlong lStack_1d0;
  undefined1 (*pauStack_1c8) [16];
  undefined1 (*pauStack_1c0) [16];
  undefined1 (*pauStack_1b8) [16];
  undefined8 *puStack_1b0;
  undefined8 *puStack_1a8;
  undefined8 uStack_1a0;
  undefined1 (*pauStack_198) [16];
  undefined1 (*pauStack_190) [16];
  undefined1 uStack_181;
  undefined1 (*pauStack_180) [16];
  undefined8 uStack_178;
  undefined8 uStack_170;
  undefined8 uStack_168;
  undefined4 uStack_160;
  undefined1 uStack_15c;
  undefined1 uStack_15b;
  undefined1 uStack_15a;
  undefined1 uStack_159;
  undefined1 uStack_158;
  undefined1 uStack_157;
  undefined6 uStack_156;
  undefined1 uStack_14a;
  undefined1 uStack_149;
  undefined1 (*pauStack_148) [16];
  undefined1 uStack_139;
  undefined1 (*pauStack_138) [16];
  undefined1 (*pauStack_130) [16];
  undefined1 uStack_121;
  undefined1 (*pauStack_120) [16];
  undefined1 (*pauStack_118) [16];
  undefined8 uStack_110;
  undefined1 (*pauStack_108) [16];
  undefined1 (*pauStack_100) [16];
  undefined1 (*pauStack_f8) [16];
  undefined8 uStack_f0;
  
  uStack_f0 = 0xfffffffffffffffe;
  puStack_2b0 = *(ulonglong **)p0;
  lVar16 = *(longlong *)((longlong)p0 + 8);
  uVar30 = *puStack_2b0;
  uVar39 = 0;
  uVar17 = uVar30 - *(ulonglong *)(lVar16 + 0x18);
  if (uVar30 < *(ulonglong *)(lVar16 + 0x18)) {
    uVar17 = uVar39;
  }
  uVar40 = uVar30 - *(ulonglong *)(lVar16 + 0x38);
  if (uVar30 < *(ulonglong *)(lVar16 + 0x38)) {
    uVar40 = uVar39;
  }
  pauVar43 = (undefined1 (*) [16])(uVar17 * 0x20 + *(longlong *)(lVar16 + 8));
  uVar30 = *(ulonglong *)(lVar16 + 0x10) - uVar17;
  if (*(ulonglong *)(lVar16 + 0x10) < uVar17) {
    pauVar43 = (undefined1 (*) [16])0x8;
    uVar30 = uVar39;
  }
  pauStack_2a8 = (undefined1 (*) [16])(uVar40 * 0x20 + *(longlong *)(lVar16 + 0x28));
  uVar17 = *(ulonglong *)(lVar16 + 0x30) - uVar40;
  if (*(ulonglong *)(lVar16 + 0x30) < uVar40) {
    pauStack_2a8 = (undefined1 (*) [16])0x8;
    uVar17 = uVar39;
  }
  *puStack_2b0 = (*(longlong *)(lVar16 + 0x40) - uVar17) - uVar30;
  fVar11 = _DAT_1429d18cc;
  uVar10 = _DAT_142925904;
  pauVar20 = pauVar43 + uVar30 * 2;
  pauVar19 = pauStack_2a8 + uVar17 * 2;
  pauStack_f8 = *(undefined1 (**) [16])p5;
  pauStack_180 = *(undefined1 (**) [16])p1;
  pauStack_268 = *(undefined1 (**) [16])p3;
  lStack_1f0 = *(longlong *)p7;
  lStack_1d0 = *(longlong *)((longlong)p7 + 8);
  lStack_2f8 = *(longlong *)p8;
  pauStack_2d0 = *(undefined1 (**) [16])p2;
  lStack_270 = *(longlong *)p4;
  pauStack_148 = *(undefined1 (**) [16])((longlong)p4 + 8);
  pauStack_130 = *(undefined1 (**) [16])p9;
  uStack_2a0 = *(undefined8 *)p13;
  puStack_298 = *(undefined8 **)p12;
  uStack_284 = *(undefined4 *)((longlong)p10 + 0x1c);
  lStack_2e8 = *(longlong *)p10;
  puStack_2e0 = *(undefined4 **)((longlong)p10 + 0x10);
  pauStack_100 = *(undefined1 (**) [16])p6;
  pauVar24 = pauVar43;
  pauStack_2f0 = pauVar20;
  pauStack_2d8 = pauVar19;
  do {
    pauVar18 = pauStack_180;
    pauVar21 = (undefined1 (*) [16])0x73726f6c6f635f72;
    if ((pauVar24 == (undefined1 (*) [16])0x0) || (pauVar24 == pauStack_2f0)) {
      if (pauStack_2a8 == (undefined1 (*) [16])0x0) {
        return;
      }
      if (pauStack_2a8 == pauStack_2d8) {
        return;
      }
      *puStack_2b0 = *puStack_2b0 + 1;
      puVar38 = *pauStack_2a8;
      pauVar23 = pauStack_2a8 + 2;
      pauVar22 = (undefined1 (*) [16])0x0;
      pauVar24 = pauStack_2a8;
      pauStack_2a8 = pauVar23;
                    /* WARNING: Switch is manually overridden */
      switch(*puVar38) {
      case 0:
        goto override_jmp_14229aa20_case_0;
      case 1:
        goto override_jmp_14229aa20_case_1;
      case 2:
        goto override_jmp_14229aa20_case_2;
      case 3:
        goto override_jmp_14229aa20_case_3;
      default:
        goto override_jmp_14229aa20_case_4;
      case 5:
      case 8:
        goto override_jmp_14229aa20_case_5;
      case 6:
        goto override_jmp_14229aa20_case_6;
      case 9:
        goto override_jmp_14229aa20_case_9;
      case 10:
        goto override_jmp_14229aa20_case_10;
      case 0xb:
        goto override_jmp_14229aa20_case_11;
      }
    }
    *puStack_2b0 = *puStack_2b0 + 1;
    pauVar22 = pauVar24 + 2;
                    /* WARNING: Switch is manually overridden */
    switch((*pauVar24)[0]) {
    case 0:
      goto override_jmp_14229aa20_case_0;
    case 1:
override_jmp_14229aa20_case_1:
      pauStack_118 = pauVar22;
      if (*_DAT_14362c098 == '\0') {
        pauVar23 = (undefined1 (*) [16])func_0x000140613c10(0x1b8,8);
        if (pauVar23 == (undefined1 (*) [16])0x0) {
          FUN_1428d9000(8,0x1b8);
LAB_1422a2325:
          pauStack_f8 = pauVar18;
          FUN_1428d8fe3(1,0x15,&UNK_143136c68);
        }
        else {
          pauVar21 = (undefined1 (*) [16])func_0x000140613c10(0x15,1);
          uVar31 = _UNK_14303c2a9;
          pauVar18 = pauVar23;
          if (pauVar21 == (undefined1 (*) [16])0x0) goto LAB_1422a2325;
          *(undefined8 *)*pauVar21 = _DAT_14303c2a1;
          *(undefined8 *)(*pauVar21 + 8) = uVar31;
          *(undefined8 *)(*pauVar21 + 0xd) = 0x657268636f5f666f;
          pauVar20 = (undefined1 (*) [16])func_0x000140613c10(0x13,1);
          uVar31 = _UNK_14303c2be;
          if (pauVar20 == (undefined1 (*) [16])0x0) {
            pauStack_100 = pauVar21;
            pauStack_f8 = pauVar23;
            FUN_1428d8fe3(1,0x13,&UNK_143136c68);
          }
          else {
            *(undefined8 *)*pauVar20 = _DAT_14303c2b6;
            *(undefined8 *)(*pauVar20 + 8) = uVar31;
            *(undefined4 *)(*pauVar20 + 0xf) = 0x6465725f;
            pauVar19 = (undefined1 (*) [16])func_0x000140613c10(0x16,1);
            uVar31 = _UNK_14303c2d1;
            if (pauVar19 == (undefined1 (*) [16])0x0) {
              pauStack_118 = pauVar20;
              pauStack_100 = pauVar21;
              pauStack_f8 = pauVar23;
              FUN_1428d8fe3(1,0x16,&UNK_143136c68);
            }
            else {
              *(undefined8 *)*pauVar19 = _DAT_14303c2c9;
              *(undefined8 *)(*pauVar19 + 8) = uVar31;
              *(undefined8 *)(*pauVar19 + 0xe) = 0x656c707275705f66;
              pauVar43 = (undefined1 (*) [16])func_0x000140613c10(0x14,1);
              uVar31 = _UNK_14303c2e7;
              if (pauVar43 == (undefined1 (*) [16])0x0) {
                pauStack_130 = pauVar19;
                pauStack_118 = pauVar20;
                pauStack_100 = pauVar21;
                pauStack_f8 = pauVar23;
                FUN_1428d8fe3(1,0x14,&UNK_143136c68);
              }
              else {
                *(undefined8 *)*pauVar43 = _DAT_14303c2df;
                *(undefined8 *)(*pauVar43 + 8) = uVar31;
                *(undefined4 *)pauVar43[1] = 0x65756c62;
                unaff_R13 = (undefined1 (*) [16])func_0x000140613c10(0x1a,1);
                uVar31 = _UNK_14303c305;
                if (unaff_R13 == (undefined1 (*) [16])0x0) {
                  pauStack_148 = pauVar43;
                  pauStack_130 = pauVar19;
                  pauStack_118 = pauVar20;
                  pauStack_100 = pauVar21;
                  pauStack_f8 = pauVar23;
                  FUN_1428d8fe3(1,0x1a,&UNK_143136c68);
                }
                else {
                  *(ulonglong *)(*unaff_R13 + 10) = CONCAT26(_UNK_14303c303,_DAT_14303c2fd);
                  *(undefined8 *)(unaff_R13[1] + 2) = uVar31;
                  uVar31 = CONCAT62(_DAT_14303c2fd,_UNK_14303c2fb);
                  *(undefined8 *)*unaff_R13 = _DAT_14303c2f3;
                  *(undefined8 *)(*unaff_R13 + 8) = uVar31;
                  pauVar22 = (undefined1 (*) [16])func_0x000140613c10(0x19,1);
                  uVar31 = _UNK_14303c31e;
                  if (pauVar22 == (undefined1 (*) [16])0x0) {
                    pauStack_180 = unaff_R13;
                    pauStack_148 = pauVar43;
                    pauStack_130 = pauVar19;
                    pauStack_118 = pauVar20;
                    pauStack_100 = pauVar21;
                    pauStack_f8 = pauVar23;
                    FUN_1428d8fe3(1,0x19,&UNK_143136c68);
                  }
                  else {
                    *(ulonglong *)(*pauVar22 + 9) = CONCAT17(UNK_14303c31d,_DAT_14303c316);
                    *(undefined8 *)(pauVar22[1] + 1) = uVar31;
                    uVar31 = CONCAT71(_DAT_14303c316,UNK_14303c315);
                    *(undefined8 *)*pauVar22 = _DAT_14303c30d;
                    *(undefined8 *)(*pauVar22 + 8) = uVar31;
                    pauStack_108 = pauVar22;
                    pauVar22 = (undefined1 (*) [16])func_0x000140613c10(0x1b,1);
                    uVar31 = _UNK_14303c339;
                    if (pauVar22 == (undefined1 (*) [16])0x0) {
                      pauStack_180 = unaff_R13;
                      pauStack_148 = pauVar43;
                      pauStack_130 = pauVar19;
                      pauStack_118 = pauVar20;
                      pauStack_100 = pauVar21;
                      pauStack_f8 = pauVar23;
                      FUN_1428d8fe3(1,0x1b,&UNK_143136c68);
                    }
                    else {
                      *(ulonglong *)(*pauVar22 + 0xb) = CONCAT35(_UNK_14303c336,_DAT_14303c331);
                      *(undefined8 *)(pauVar22[1] + 3) = uVar31;
                      uVar31 = CONCAT53(_DAT_14303c331,_UNK_14303c32e);
                      *(undefined8 *)*pauVar22 = _DAT_14303c326;
                      *(undefined8 *)(*pauVar22 + 8) = uVar31;
                      uStack_110 = pauVar22;
                      pauVar22 = (undefined1 (*) [16])func_0x000140613c10(0x15,1);
                      uVar31 = _UNK_14303c349;
                      if (pauVar22 == (undefined1 (*) [16])0x0) {
LAB_1422a2418:
                        pauStack_180 = unaff_R13;
                        pauStack_148 = pauVar43;
                        pauStack_130 = pauVar19;
                        pauStack_118 = pauVar20;
                        pauStack_100 = pauVar21;
                        pauStack_f8 = pauVar18;
                        FUN_1428d8fe3(1,0x15,&UNK_143136c68);
                      }
                      else {
                        *(undefined8 *)*pauVar22 = _DAT_14303c341;
                        *(undefined8 *)(*pauVar22 + 8) = uVar31;
                        *(undefined8 *)(*pauVar22 + 0xd) = 0x6e656572675f666f;
                        pauStack_120 = pauVar22;
                        pauVar18 = (undefined1 (*) [16])func_0x000140613c10(0x19,1);
                        uVar31 = _UNK_14303c367;
                        if (pauVar18 == (undefined1 (*) [16])0x0) {
                          pauStack_180 = unaff_R13;
                          pauStack_148 = pauVar43;
                          pauStack_130 = pauVar19;
                          pauStack_118 = pauVar20;
                          pauStack_100 = pauVar21;
                          pauStack_f8 = pauVar23;
                          FUN_1428d8fe3(1,0x19,&UNK_143136c68);
                        }
                        else {
                          *(ulonglong *)(*pauVar18 + 9) = CONCAT17(UNK_14303c366,_DAT_14303c35f);
                          *(undefined8 *)(pauVar18[1] + 1) = uVar31;
                          uVar31 = CONCAT71(_DAT_14303c35f,UNK_14303c35e);
                          *(undefined8 *)*pauVar18 = _DAT_14303c356;
                          *(undefined8 *)(*pauVar18 + 8) = uVar31;
                          pauStack_138 = pauVar18;
                          puVar25 = (undefined8 *)func_0x000140613c10(0x14,1);
                          uVar31 = _UNK_14303c377;
                          if (puVar25 == (undefined8 *)0x0) {
                            pauStack_180 = unaff_R13;
                            pauStack_148 = pauVar43;
                            pauStack_130 = pauVar19;
                            pauStack_118 = pauVar20;
                            pauStack_100 = pauVar21;
                            pauStack_f8 = pauVar23;
                            FUN_1428d8fe3(1,0x14,&UNK_143136c68);
                          }
                          else {
                            *puVar25 = _DAT_14303c36f;
                            puVar25[1] = uVar31;
                            *(undefined4 *)(puVar25 + 2) = 0x6b726164;
                            puStack_1a8 = puVar25;
                            puVar25 = (undefined8 *)func_0x000140613c10(0x14,1);
                            uVar31 = _UNK_14303c38b;
                            if (puVar25 != (undefined8 *)0x0) {
                              *puVar25 = _DAT_14303c383;
                              puVar25[1] = uVar31;
                              *(undefined4 *)(puVar25 + 2) = 0x79657267;
                              *(undefined8 *)*pauVar23 = 0x15;
                              *(undefined1 (**) [16])(*pauVar23 + 8) = pauVar21;
                              *(undefined8 *)pauVar23[1] = 0x15;
                              pauVar23[1][8] = 3;
                              *(undefined4 *)(pauVar23[1] + 0xc) = 8;
                              *(undefined4 *)(pauVar23[2] + 4) = 0;
                              *(undefined8 *)(pauVar23[2] + 8) = 0x13;
                              *(undefined1 (**) [16])pauVar23[3] = pauVar20;
                              *(undefined8 *)(pauVar23[3] + 8) = 0x13;
                              pauVar23[4][0] = 3;
                              *(undefined4 *)(pauVar23[4] + 4) = 0;
                              *(undefined4 *)(pauVar23[4] + 0xc) = 0;
                              *(undefined8 *)pauVar23[5] = 0x16;
                              *(undefined1 (**) [16])(pauVar23[5] + 8) = pauVar19;
                              *(undefined8 *)pauVar23[6] = 0x16;
                              pauVar23[6][8] = 3;
                              *(undefined4 *)(pauVar23[6] + 0xc) = 10;
                              *(undefined4 *)(pauVar23[7] + 4) = 0;
                              *(undefined8 *)(pauVar23[7] + 8) = 0x14;
                              *(undefined1 (**) [16])pauVar23[8] = pauVar43;
                              *(undefined8 *)(pauVar23[8] + 8) = 0x14;
                              pauVar23[9][0] = 3;
                              *(undefined4 *)(pauVar23[9] + 4) = 2;
                              *(undefined4 *)(pauVar23[9] + 0xc) = 0;
                              *(undefined8 *)pauVar23[10] = 0x1a;
                              *(undefined1 (**) [16])(pauVar23[10] + 8) = unaff_R13;
                              *(undefined8 *)pauVar23[0xb] = 0x1a;
                              pauVar23[0xb][8] = 3;
                              *(undefined4 *)(pauVar23[0xb] + 0xc) = 9;
                              *(undefined4 *)(pauVar23[0xc] + 4) = 0;
                              *(undefined8 *)(pauVar23[0xc] + 8) = 0x19;
                              *(undefined1 (**) [16])pauVar23[0xd] = pauStack_108;
                              *(undefined8 *)(pauVar23[0xd] + 8) = 0x19;
                              pauVar23[0xe][0] = 3;
                              *(undefined4 *)(pauVar23[0xe] + 4) = 3;
                              *(undefined4 *)(pauVar23[0xe] + 0xc) = 0;
                              *(undefined8 *)pauVar23[0xf] = 0x1b;
                              *(undefined1 (**) [16])(pauVar23[0xf] + 8) = uStack_110;
                              *(undefined8 *)pauVar23[0x10] = 0x1b;
                              pauVar23[0x10][8] = 3;
                              *(undefined4 *)(pauVar23[0x10] + 0xc) = 6;
                              *(undefined4 *)(pauVar23[0x11] + 4) = 0;
                              *(undefined8 *)(pauVar23[0x11] + 8) = 0x15;
                              *(undefined1 (**) [16])pauVar23[0x12] = pauStack_120;
                              *(undefined8 *)(pauVar23[0x12] + 8) = 0x15;
                              pauVar23[0x13][0] = 3;
                              *(undefined4 *)(pauVar23[0x13] + 4) = 4;
                              *(undefined4 *)(pauVar23[0x13] + 0xc) = 0;
                              *(undefined8 *)pauVar23[0x14] = 0x19;
                              *(undefined1 (**) [16])(pauVar23[0x14] + 8) = pauStack_138;
                              *(undefined8 *)pauVar23[0x15] = 0x19;
                              pauVar23[0x15][8] = 3;
                              *(undefined4 *)(pauVar23[0x15] + 0xc) = 1;
                              *(undefined4 *)(pauVar23[0x16] + 4) = 0;
                              *(undefined8 *)(pauVar23[0x16] + 8) = 0x14;
                              *(undefined8 **)pauVar23[0x17] = puStack_1a8;
                              *(undefined8 *)(pauVar23[0x17] + 8) = 0x14;
                              pauVar23[0x18][0] = 3;
                              *(undefined4 *)(pauVar23[0x18] + 4) = 5;
                              *(undefined4 *)(pauVar23[0x18] + 0xc) = 0;
                              *(undefined8 *)pauVar23[0x19] = 0x14;
                              *(undefined8 **)(pauVar23[0x19] + 8) = puVar25;
                              *(undefined8 *)pauVar23[0x1a] = 0x14;
                              pauVar23[0x1a][8] = 3;
                              *(undefined4 *)(pauVar23[0x1a] + 0xc) = 7;
                              *(undefined4 *)(pauVar23[0x1b] + 4) = 0;
                              pauStack_1e8 = (undefined1 (*) [16])0xb;
                              pauStack_1d8 = (undefined1 (*) [16])0xb;
                              pauVar20 = pauStack_1e8;
                              pauVar19 = pauStack_1d8;
                              break;
                            }
                            pauStack_180 = unaff_R13;
                            pauStack_148 = pauVar43;
                            pauStack_130 = pauVar19;
                            pauStack_118 = pauVar20;
                            pauStack_100 = pauVar21;
                            pauStack_f8 = pauVar23;
                            FUN_1428d8fe3(1,0x14,&UNK_143136c68);
                          }
                        }
                      }
                    }
                  }
                }
              }
            }
          }
        }
      }
      else {
        pauVar18 = (undefined1 (*) [16])func_0x000140613c10(0x168,8);
        if (pauVar18 == (undefined1 (*) [16])0x0) goto LAB_1422a0fdc;
        pauVar20 = (undefined1 (*) [16])func_0x000140613c10(0x13,1);
        uVar31 = _UNK_14303c2be;
        if (pauVar20 == (undefined1 (*) [16])0x0) {
          pauStack_f8 = pauVar18;
          FUN_1428d8fe3(1,0x13,&UNK_143136c68);
        }
        else {
          *(undefined8 *)*pauVar20 = _DAT_14303c2b6;
          *(undefined8 *)(*pauVar20 + 8) = uVar31;
          *(undefined4 *)(*pauVar20 + 0xf) = 0x6465725f;
          pauVar21 = (undefined1 (*) [16])func_0x000140613c10(0x15,1);
          uVar31 = _UNK_14303c349;
          if (pauVar21 == (undefined1 (*) [16])0x0) {
            pauStack_100 = pauVar20;
            pauStack_f8 = pauVar18;
            FUN_1428d8fe3(1,0x15,&UNK_143136c68);
          }
          else {
            *(undefined8 *)*pauVar21 = _DAT_14303c341;
            *(undefined8 *)(*pauVar21 + 8) = uVar31;
            *(undefined8 *)(*pauVar21 + 0xd) = 0x6e656572675f666f;
            pauVar19 = (undefined1 (*) [16])func_0x000140613c10(0x11,1);
            uVar12 = _UNK_14303c298;
            uVar31 = _DAT_14303c290;
            if (pauVar19 == (undefined1 (*) [16])0x0) goto LAB_1422a2356;
            *(undefined8 *)*pauVar19 = _DAT_14303c290;
            *(undefined8 *)(*pauVar19 + 8) = uVar12;
            pauVar19[1][0] = 100;
            pauVar43 = (undefined1 (*) [16])func_0x000140613c10(0x11,1);
            if (pauVar43 == (undefined1 (*) [16])0x0) {
              pauStack_138 = pauVar19;
              pauStack_118 = pauVar21;
              pauStack_100 = pauVar20;
              pauStack_f8 = pauVar18;
              FUN_1428d8fe3(1,0x11,&UNK_143136c68);
            }
            else {
              *(undefined8 *)*pauVar43 = uVar31;
              *(undefined8 *)(*pauVar43 + 8) = uVar12;
              pauVar43[1][0] = 100;
              unaff_R13 = (undefined1 (*) [16])func_0x000140613c10(0x11,1);
              if (unaff_R13 == (undefined1 (*) [16])0x0) {
                pauStack_138 = pauVar19;
                pauStack_130 = pauVar43;
                pauStack_118 = pauVar21;
                pauStack_100 = pauVar20;
                pauStack_f8 = pauVar18;
                FUN_1428d8fe3(1,0x11,&UNK_143136c68);
              }
              else {
                *(undefined8 *)*unaff_R13 = uVar31;
                *(undefined8 *)(*unaff_R13 + 8) = uVar12;
                unaff_R13[1][0] = 100;
                pauVar22 = (undefined1 (*) [16])func_0x000140613c10(0x11,1);
                if (pauVar22 == (undefined1 (*) [16])0x0) {
                  pauStack_148 = unaff_R13;
                  pauStack_138 = pauVar19;
                  pauStack_130 = pauVar43;
                  pauStack_118 = pauVar21;
                  pauStack_100 = pauVar20;
                  pauStack_f8 = pauVar18;
                  FUN_1428d8fe3(1,0x11,&UNK_143136c68);
                }
                else {
                  *(undefined8 *)*pauVar22 = uVar31;
                  *(undefined8 *)(*pauVar22 + 8) = uVar12;
                  pauVar22[1][0] = 100;
                  pauStack_108 = pauVar22;
                  pauVar22 = (undefined1 (*) [16])func_0x000140613c10(0x11,1);
                  if (pauVar22 == (undefined1 (*) [16])0x0) {
                    pauStack_148 = unaff_R13;
                    pauStack_138 = pauVar19;
                    pauStack_130 = pauVar43;
                    pauStack_118 = pauVar21;
                    pauStack_100 = pauVar20;
                    pauStack_f8 = pauVar18;
                    FUN_1428d8fe3(1,0x11,&UNK_143136c68);
                  }
                  else {
                    *(undefined8 *)*pauVar22 = uVar31;
                    *(undefined8 *)(*pauVar22 + 8) = uVar12;
                    pauVar22[1][0] = 100;
                    uStack_110 = pauVar22;
                    pauVar22 = (undefined1 (*) [16])func_0x000140613c10(0x11,1);
                    if (pauVar22 == (undefined1 (*) [16])0x0) {
                      pauStack_148 = unaff_R13;
                      pauStack_138 = pauVar19;
                      pauStack_130 = pauVar43;
                      pauStack_118 = pauVar21;
                      pauStack_100 = pauVar20;
                      pauStack_f8 = pauVar18;
                      FUN_1428d8fe3(1,0x11,&UNK_143136c68);
                    }
                    else {
                      *(undefined8 *)*pauVar22 = uVar31;
                      *(undefined8 *)(*pauVar22 + 8) = uVar12;
                      pauVar22[1][0] = 100;
                      pauStack_120 = pauVar22;
                      puVar25 = (undefined8 *)func_0x000140613c10(0x11,1);
                      if (puVar25 != (undefined8 *)0x0) {
                        *puVar25 = uVar31;
                        puVar25[1] = uVar12;
                        *(undefined1 *)(puVar25 + 2) = 100;
                        *(undefined8 *)*pauVar18 = 0x13;
                        *(undefined1 (**) [16])(*pauVar18 + 8) = pauVar20;
                        *(undefined8 *)pauVar18[1] = 0x13;
                        pauVar18[1][8] = 3;
                        *(undefined4 *)(pauVar18[1] + 0xc) = 0;
                        *(undefined4 *)(pauVar18[2] + 4) = 0;
                        *(undefined8 *)(pauVar18[2] + 8) = 0x15;
                        *(undefined1 (**) [16])pauVar18[3] = pauVar21;
                        *(undefined8 *)(pauVar18[3] + 8) = 0x15;
                        pauVar18[4][0] = 3;
                        *(undefined4 *)(pauVar18[4] + 4) = 4;
                        *(undefined4 *)(pauVar18[4] + 0xc) = 0;
                        *(undefined8 *)pauVar18[5] = 0x11;
                        *(undefined1 (**) [16])(pauVar18[5] + 8) = pauVar19;
                        *(undefined8 *)pauVar18[6] = 0x11;
                        pauVar18[6][8] = 3;
                        *(undefined4 *)(pauVar18[6] + 0xc) = 0x309;
                        *(undefined4 *)(pauVar18[7] + 4) = 0;
                        *(undefined8 *)(pauVar18[7] + 8) = 0x11;
                        *(undefined1 (**) [16])pauVar18[8] = pauVar43;
                        *(undefined8 *)(pauVar18[8] + 8) = 0x11;
                        pauVar18[9][0] = 3;
                        *(undefined4 *)(pauVar18[9] + 4) = 0x309;
                        *(undefined4 *)(pauVar18[9] + 0xc) = 0;
                        *(undefined8 *)pauVar18[10] = 0x11;
                        *(undefined1 (**) [16])(pauVar18[10] + 8) = unaff_R13;
                        *(undefined8 *)pauVar18[0xb] = 0x11;
                        pauVar18[0xb][8] = 3;
                        *(undefined4 *)(pauVar18[0xb] + 0xc) = 0x309;
                        *(undefined4 *)(pauVar18[0xc] + 4) = 0;
                        *(undefined8 *)(pauVar18[0xc] + 8) = 0x11;
                        *(undefined1 (**) [16])pauVar18[0xd] = pauStack_108;
                        *(undefined8 *)(pauVar18[0xd] + 8) = 0x11;
                        pauVar18[0xe][0] = 3;
                        *(undefined4 *)(pauVar18[0xe] + 4) = 0x309;
                        *(undefined4 *)(pauVar18[0xe] + 0xc) = 0;
                        *(undefined8 *)pauVar18[0xf] = 0x11;
                        *(undefined1 (**) [16])(pauVar18[0xf] + 8) = uStack_110;
                        *(undefined8 *)pauVar18[0x10] = 0x11;
                        pauVar18[0x10][8] = 3;
                        *(undefined4 *)(pauVar18[0x10] + 0xc) = 0x309;
                        *(undefined4 *)(pauVar18[0x11] + 4) = 0;
                        *(undefined8 *)(pauVar18[0x11] + 8) = 0x11;
                        *(undefined1 (**) [16])pauVar18[0x12] = pauStack_120;
                        *(undefined8 *)(pauVar18[0x12] + 8) = 0x11;
                        pauVar18[0x13][0] = 3;
                        *(undefined4 *)(pauVar18[0x13] + 4) = 0x309;
                        *(undefined4 *)(pauVar18[0x13] + 0xc) = 0;
                        *(undefined8 *)pauVar18[0x14] = 0x11;
                        *(undefined8 **)(pauVar18[0x14] + 8) = puVar25;
                        *(undefined8 *)pauVar18[0x15] = 0x11;
                        pauVar18[0x15][8] = 3;
                        *(undefined4 *)(pauVar18[0x15] + 0xc) = 0x309;
                        *(undefined4 *)(pauVar18[0x16] + 4) = 0;
                        pauStack_1e8 = (undefined1 (*) [16])0x9;
                        pauStack_1d8 = (undefined1 (*) [16])0x9;
                        pauVar20 = pauStack_1e8;
                        pauVar23 = pauVar18;
                        pauVar19 = pauStack_1d8;
                        break;
                      }
LAB_1422a1f89:
                      pauStack_148 = unaff_R13;
                      pauStack_138 = pauVar19;
                      pauStack_130 = pauVar43;
                      pauStack_118 = pauVar21;
                      pauStack_100 = pauVar20;
                      pauStack_f8 = pauVar18;
                      FUN_1428d8fe3(1,0x11,&UNK_143136c68);
                    }
                  }
                }
              }
            }
          }
        }
      }
      goto LAB_1422a2ea6;
    case 2:
override_jmp_14229aa20_case_2:
      auStack_278 = *(undefined1 (*) [8])(*pauVar24 + 8);
      pauVar20 = (undefined1 (*) [16])auStack_278;
      pauStack_118 = pauVar22;
      uVar30 = func_0x0001400885d0(pauVar20);
      if (uVar30 < *(ulonglong *)pauVar18[1]) {
        uVar42 = *(uint *)(*(longlong *)(*pauVar18 + 8) + uVar30 * 4);
        pauVar21 = (undefined1 (*) [16])(ulonglong)uVar42;
        uStack_288 = uVar42;
        uVar30 = func_0x0001400885d0(pauVar20);
        if (*(ulonglong *)pauStack_268[1] <= uVar30) {
          FUN_1428d9518(uVar30,*(ulonglong *)pauStack_268[1],&UNK_14303c3b0);
          goto LAB_1422a13a9;
        }
        uVar15 = *(uint *)(*(longlong *)(*pauStack_268 + 8) + uVar30 * 4);
        uStack_28c = uVar15;
        uVar30 = FUN_140a922b0(pauStack_100,auStack_278);
        if (((uVar30 >> 0x20 == 0) ||
            (uVar17 = uVar30 & 0xffffffff, *(ulonglong *)(lStack_1d0 + 0x10) <= uVar17)) ||
           (*(int *)(*(longlong *)(lStack_1d0 + 8) + uVar17 * 0x14) != (int)(uVar30 >> 0x20))) {
LAB_14229c621:
          uVar47 = 0;
LAB_14229c623:
          pauVar20 = (undefined1 (*) [16])func_0x000140613c10(200,8);
          if (pauVar20 == (undefined1 (*) [16])0x0) {
            FUN_1428d9000(8,200);
          }
          else {
            uStack_110 = (undefined1 (*) [16])CONCAT44(uStack_110._4_4_,(int)pauVar21);
            pauVar43 = (undefined1 (*) [16])func_0x000140613c10(0x17,1);
            uVar31 = _UNK_14303c160;
            if (pauVar43 == (undefined1 (*) [16])0x0) {
              uStack_110 = pauVar20;
              FUN_1428d8fe3(1,0x17,&UNK_143136c68);
              goto LAB_1422a2ea6;
            }
            *(undefined8 *)*pauVar43 = _DAT_14303c158;
            *(undefined8 *)(*pauVar43 + 8) = uVar31;
            *(undefined8 *)(*pauVar43 + 0xf) = 0x65676965625f736b;
            pauVar21 = (undefined1 (*) [16])func_0x000140613c10(0x18,1);
            uVar31 = _UNK_14303c177;
            if (pauVar21 == (undefined1 (*) [16])0x0) {
              pauStack_120 = pauVar43;
              uStack_110 = pauVar20;
              FUN_1428d8fe3(1,0x18,&UNK_143136c68);
              goto LAB_1422a2ea6;
            }
            *(undefined8 *)*pauVar21 = _DAT_14303c16f;
            *(undefined8 *)(*pauVar21 + 8) = uVar31;
            *(undefined8 *)pauVar21[1] = 0x776f6c6c65795f73;
            pauVar18 = (undefined1 (*) [16])func_0x000140613c10(0x17,1);
            uVar31 = _UNK_14303c18f;
            pauVar19 = pauVar20;
            if (pauVar18 != (undefined1 (*) [16])0x0) {
              *(undefined8 *)*pauVar18 = _DAT_14303c187;
              *(undefined8 *)(*pauVar18 + 8) = uVar31;
              *(undefined8 *)(*pauVar18 + 0xf) = 0x746867696c5f736b;
              pauStack_108 = pauVar18;
              pauVar19 = (undefined1 (*) [16])func_0x000140613c10(0x16,1);
              uVar31 = _UNK_14303c1a6;
              if (pauVar19 == (undefined1 (*) [16])0x0) {
                pauStack_120 = pauVar43;
                uStack_110 = pauVar20;
                pauStack_f8 = pauVar21;
                FUN_1428d8fe3(1,0x16,&UNK_143136c68);
              }
              else {
                *(undefined8 *)*pauVar19 = _DAT_14303c19e;
                *(undefined8 *)(*pauVar19 + 8) = uVar31;
                *(undefined8 *)(*pauVar19 + 0xe) = 0x796572675f736b63;
                puVar25 = (undefined8 *)func_0x000140613c10(0x16,1);
                uVar31 = _UNK_14303c1bc;
                if (puVar25 != (undefined8 *)0x0) {
                  *puVar25 = _DAT_14303c1b4;
                  puVar25[1] = uVar31;
                  *(undefined8 *)((longlong)puVar25 + 0xe) = 0x6b7261645f736b63;
                  *(undefined8 *)*pauVar20 = 0x17;
                  *(undefined1 (**) [16])(*pauVar20 + 8) = pauVar43;
                  *(undefined8 *)pauVar20[1] = 0x17;
                  *(undefined2 *)(pauVar20[1] + 8) = 2;
                  *(undefined4 *)(pauVar20[1] + 0xc) = 0;
                  *(undefined4 *)pauVar20[2] = uVar47;
                  *(undefined4 *)(pauVar20[2] + 4) = 0;
                  *(undefined8 *)(pauVar20[2] + 8) = 0x18;
                  *(undefined1 (**) [16])pauVar20[3] = pauVar21;
                  *(undefined8 *)(pauVar20[3] + 8) = 0x18;
                  *(undefined2 *)pauVar20[4] = 2;
                  *(undefined4 *)(pauVar20[4] + 4) = 1;
                  *(undefined4 *)(pauVar20[4] + 8) = uVar47;
                  *(undefined4 *)(pauVar20[4] + 0xc) = 0;
                  *(undefined8 *)pauVar20[5] = 0x17;
                  *(undefined1 (**) [16])(pauVar20[5] + 8) = pauStack_108;
                  *(undefined8 *)pauVar20[6] = 0x17;
                  *(undefined2 *)(pauVar20[6] + 8) = 2;
                  *(undefined4 *)(pauVar20[6] + 0xc) = 3;
                  *(undefined4 *)pauVar20[7] = uVar47;
                  *(undefined4 *)(pauVar20[7] + 4) = 0;
                  *(undefined8 *)(pauVar20[7] + 8) = 0x16;
                  *(undefined1 (**) [16])pauVar20[8] = pauVar19;
                  *(undefined8 *)(pauVar20[8] + 8) = 0x16;
                  *(undefined2 *)pauVar20[9] = 2;
                  *(undefined4 *)(pauVar20[9] + 4) = 0xb;
                  *(undefined4 *)(pauVar20[9] + 8) = uVar47;
                  *(undefined4 *)(pauVar20[9] + 0xc) = 0;
                  *(undefined8 *)pauVar20[10] = 0x16;
                  *(undefined8 **)(pauVar20[10] + 8) = puVar25;
                  *(undefined8 *)pauVar20[0xb] = 0x16;
                  *(undefined2 *)(pauVar20[0xb] + 8) = 2;
                  *(undefined4 *)(pauVar20[0xb] + 0xc) = 2;
                  *(undefined4 *)pauVar20[0xc] = uVar47;
                  puVar29 = (undefined4 *)(pauVar20[0xc] + 4);
                  pauStack_220 = (undefined1 (*) [16])0x5;
                  goto LAB_14229c890;
                }
                pauStack_120 = pauVar43;
                uStack_110 = pauVar20;
                pauStack_100 = pauVar19;
                pauStack_f8 = pauVar21;
                FUN_1428d8fe3(1,0x16,&UNK_143136c68);
              }
              goto LAB_1422a2ea6;
            }
          }
          pauStack_120 = pauVar43;
          uStack_110 = pauVar19;
          pauStack_f8 = pauVar21;
          FUN_1428d8fe3(1,0x17,&UNK_143136c68);
          goto LAB_1422a2ea6;
        }
        lVar16 = *(longlong *)(lStack_1d0 + 8) + uVar17 * 0x14;
        uVar30 = (ulonglong)*(uint *)(lVar16 + 4);
        if (((uVar30 == 0xffffffff) || (*(ulonglong *)(lStack_1f0 + 0x38) <= uVar30)) ||
           ((*(ulonglong *)
              (*(longlong *)(lStack_1f0 + 0x28) + (ulonglong)(*(uint *)(lVar16 + 4) >> 6) * 8) >>
             (uVar30 & 0x3f) & 1) == 0)) goto LAB_14229c621;
        lVar34 = *(longlong *)
                  (*(longlong *)
                    (*(longlong *)(lStack_1d0 + 0x1a8) + 0x18 +
                    (ulonglong)*(uint *)(lVar16 + 0xc) * 0x48) + 0x10 +
                  ~*(ulonglong *)
                    (*(longlong *)
                      (*(longlong *)(lStack_1d0 + 0x1a8) + 0x38 +
                      (ulonglong)*(uint *)(lVar16 + 0xc) * 0x48) +
                    *(longlong *)(lStack_1f0 + 0x110) * 8) * 0x30);
        lVar16 = (ulonglong)*(uint *)(lVar16 + 0x10) * 0x58;
        cVar14 = *(char *)(lVar34 + 0x40 + lVar16);
        if (cVar14 != '\0') {
          if (cVar14 != '\x02') goto LAB_14229c621;
          lVar34 = lVar34 + lVar16;
          uVar47 = *(undefined4 *)(lVar34 + 0x48);
          uVar41 = *(uint *)(lVar34 + 0x44);
          if (*(char *)(lVar34 + 0x41) != '\0') goto LAB_14229b0ab;
LAB_1422a0b99:
          pauVar43 = (undefined1 (*) [16])(ulonglong)uVar41;
          pauVar21 = pauVar43;
          goto LAB_14229c623;
        }
        lVar16 = FUN_140a91ef0(pauStack_100,auStack_278,&UNK_14303c9b0);
        if (lVar16 == 0) {
          FUN_1428d9310(&UNK_14303c9c8);
LAB_1422a2e09:
          uStack_110 = pauVar19;
          pauStack_100 = pauVar21;
          pauStack_f8 = pauVar18;
          FUN_1428d8fe3(1,0x14,&UNK_143136c68);
          goto LAB_1422a2ea6;
        }
        cVar14 = func_0x0001408c5670(lVar16 + 0x299);
        puVar33 = &uStack_288;
        if (cVar14 != '\0') {
          puVar33 = &uStack_28c;
        }
        uVar47 = 0;
        uVar41 = *puVar33;
        if (cVar14 == '\0') goto LAB_1422a0b99;
LAB_14229b0ab:
        pauVar20 = (undefined1 (*) [16])func_0x000140613c10(400,8);
        if (pauVar20 == (undefined1 (*) [16])0x0) goto LAB_1422a0f87;
        uStack_110 = (undefined1 (*) [16])CONCAT44(uStack_110._4_4_,uVar42);
        pauVar18 = (undefined1 (*) [16])func_0x000140613c10(0x18,1);
        uVar31 = _UNK_14303c3d0;
        if (pauVar18 == (undefined1 (*) [16])0x0) {
          uStack_110 = pauVar20;
          FUN_1428d8fe3(1,0x18,&UNK_143136c68);
          goto LAB_1422a2ea6;
        }
        *(undefined8 *)*pauVar18 = _DAT_14303c3c8;
        *(undefined8 *)(*pauVar18 + 8) = uVar31;
        *(undefined8 *)pauVar18[1] = 0x65746968772f7377;
        pauVar21 = (undefined1 (*) [16])func_0x000140613c10(0x17,1);
        uVar31 = _UNK_14303c3e8;
        if (pauVar21 == (undefined1 (*) [16])0x0) {
          uStack_110 = pauVar20;
          pauStack_f8 = pauVar18;
          FUN_1428d8fe3(1,0x17,&UNK_143136c68);
          goto LAB_1422a2ea6;
        }
        *(undefined8 *)*pauVar21 = _DAT_14303c3e0;
        *(undefined8 *)(*pauVar21 + 8) = uVar31;
        *(undefined8 *)(*pauVar21 + 0xf) = 0x646f6f772f73776f;
        pauVar43 = (undefined1 (*) [16])func_0x000140613c10(0x1c,1);
        uVar31 = _UNK_14303c40b;
        if (pauVar43 == (undefined1 (*) [16])0x0) {
          uStack_110 = pauVar20;
          pauStack_100 = pauVar21;
          pauStack_f8 = pauVar18;
          FUN_1428d8fe3(1,0x1c,&UNK_143136c68);
          goto LAB_1422a2ea6;
        }
        *(ulonglong *)(*pauVar43 + 0xc) = CONCAT44(_UNK_14303c407,_DAT_14303c403);
        *(undefined8 *)(pauVar43[1] + 4) = uVar31;
        uVar31 = CONCAT44(_DAT_14303c403,_UNK_14303c3ff);
        *(undefined8 *)*pauVar43 = _DAT_14303c3f7;
        *(undefined8 *)(*pauVar43 + 8) = uVar31;
        pauStack_108 = pauVar43;
        pauVar43 = (undefined1 (*) [16])func_0x000140613c10(0x14,1);
        uVar31 = _UNK_14303c377;
        if (pauVar43 == (undefined1 (*) [16])0x0) {
          uStack_110 = pauVar20;
          pauStack_100 = pauVar21;
          pauStack_f8 = pauVar18;
          FUN_1428d8fe3(1,0x14,&UNK_143136c68);
          goto LAB_1422a2ea6;
        }
        *(undefined8 *)*pauVar43 = _DAT_14303c36f;
        *(undefined8 *)(*pauVar43 + 8) = uVar31;
        *(undefined4 *)pauVar43[1] = 0x6b726164;
        pauStack_120 = pauVar43;
        pauVar43 = (undefined1 (*) [16])func_0x000140613c10(0x13,1);
        uVar31 = _UNK_14303c2be;
        if (pauVar43 == (undefined1 (*) [16])0x0) {
          uStack_110 = pauVar20;
          pauStack_100 = pauVar21;
          pauStack_f8 = pauVar18;
          FUN_1428d8fe3(1,0x13,&UNK_143136c68);
          goto LAB_1422a2ea6;
        }
        *(undefined8 *)*pauVar43 = _DAT_14303c2b6;
        *(undefined8 *)(*pauVar43 + 8) = uVar31;
        *(undefined4 *)(*pauVar43 + 0xf) = 0x6465725f;
        pauStack_138 = pauVar43;
        puVar25 = (undefined8 *)func_0x000140613c10(0xf,1);
        if (puVar25 == (undefined8 *)0x0) {
          uStack_110 = pauVar20;
          pauStack_100 = pauVar21;
          pauStack_f8 = pauVar18;
          FUN_1428d8fe3(1,0xf,&UNK_143136c68);
          goto LAB_1422a2ea6;
        }
        *(undefined8 *)((longlong)puVar25 + 7) = 0x6b6e69702f6e6f63;
        *puVar25 = 0x63695f726f6c6f63;
        puStack_1a8 = puVar25;
        puVar25 = (undefined8 *)func_0x000140613c10(0x19,1);
        uVar31 = _UNK_14303c424;
        if (puVar25 == (undefined8 *)0x0) {
          uStack_110 = pauVar20;
          pauStack_100 = pauVar21;
          pauStack_f8 = pauVar18;
          FUN_1428d8fe3(1,0x19,&UNK_143136c68);
          goto LAB_1422a2ea6;
        }
        *(ulonglong *)((longlong)puVar25 + 9) = CONCAT17(UNK_14303c423,_DAT_14303c41c);
        *(undefined8 *)((longlong)puVar25 + 0x11) = uVar31;
        uVar31 = CONCAT71(_DAT_14303c41c,UNK_14303c41b);
        *puVar25 = _DAT_14303c413;
        puVar25[1] = uVar31;
        puStack_1b0 = puVar25;
        puVar25 = (undefined8 *)func_0x000140613c10(0x14,1);
        uVar31 = _UNK_14303c2e7;
        pauVar19 = pauVar20;
        if (puVar25 == (undefined8 *)0x0) goto LAB_1422a2e09;
        puStack_228 = (undefined8 *)CONCAT44(puStack_228._4_4_,uVar41);
        *puVar25 = _DAT_14303c2df;
        puVar25[1] = uVar31;
        *(undefined4 *)(puVar25 + 2) = 0x65756c62;
        puStack_1f8 = puVar25;
        pauVar43 = (undefined1 (*) [16])func_0x000140613c10(0x1b,1);
        uVar31 = _UNK_14303c339;
        if (pauVar43 == (undefined1 (*) [16])0x0) {
          uStack_110 = pauVar20;
          pauStack_100 = pauVar21;
          pauStack_f8 = pauVar18;
          FUN_1428d8fe3(1,0x1b,&UNK_143136c68);
          goto LAB_1422a2ea6;
        }
        *(ulonglong *)(*pauVar43 + 0xb) = CONCAT35(_UNK_14303c336,_DAT_14303c331);
        *(undefined8 *)(pauVar43[1] + 3) = uVar31;
        uVar31 = CONCAT53(_DAT_14303c331,_UNK_14303c32e);
        *(undefined8 *)*pauVar43 = _DAT_14303c326;
        *(undefined8 *)(*pauVar43 + 8) = uVar31;
        puVar25 = (undefined8 *)func_0x000140613c10(0x15,1);
        if (puVar25 == (undefined8 *)0x0) {
          pauStack_118 = pauVar43;
          uStack_110 = pauVar20;
          pauStack_100 = pauVar21;
          pauStack_f8 = pauVar18;
          FUN_1428d8fe3(1,0x15,&UNK_143136c68);
          goto LAB_1422a2ea6;
        }
        *(undefined8 *)*pauVar20 = 0x18;
        *(undefined1 (**) [16])(*pauVar20 + 8) = pauVar18;
        *(undefined8 *)pauVar20[1] = 0x18;
        *(undefined2 *)(pauVar20[1] + 8) = 0x102;
        *(undefined4 *)(pauVar20[1] + 0xc) = 4;
        *(undefined4 *)pauVar20[2] = uVar47;
        *(undefined4 *)(pauVar20[2] + 4) = 0;
        *(undefined8 *)(pauVar20[2] + 8) = 0x17;
        *(undefined1 (**) [16])pauVar20[3] = pauVar21;
        *(undefined8 *)(pauVar20[3] + 8) = 0x17;
        *(undefined2 *)pauVar20[4] = 0x102;
        *(undefined4 *)(pauVar20[4] + 4) = 0;
        *(undefined4 *)(pauVar20[4] + 8) = uVar47;
        *(undefined4 *)(pauVar20[4] + 0xc) = 0;
        *(undefined8 *)pauVar20[5] = 0x1c;
        *(undefined1 (**) [16])(pauVar20[5] + 8) = pauStack_108;
        *(undefined8 *)pauVar20[6] = 0x1c;
        *(undefined2 *)(pauVar20[6] + 8) = 0x102;
        *(undefined4 *)(pauVar20[6] + 0xc) = 1;
        *(undefined4 *)pauVar20[7] = uVar47;
        *(undefined4 *)(pauVar20[7] + 4) = 0;
        *(undefined8 *)(pauVar20[7] + 8) = 0x14;
        *(undefined1 (**) [16])pauVar20[8] = pauStack_120;
        *(undefined8 *)(pauVar20[8] + 8) = 0x14;
        *(undefined2 *)pauVar20[9] = 0x102;
        *(undefined4 *)(pauVar20[9] + 4) = 7;
        *(undefined4 *)(pauVar20[9] + 8) = uVar47;
        *(undefined4 *)(pauVar20[9] + 0xc) = 0;
        *(undefined8 *)pauVar20[10] = 0x13;
        *(undefined1 (**) [16])(pauVar20[10] + 8) = pauStack_138;
        *(undefined8 *)pauVar20[0xb] = 0x13;
        *(undefined2 *)(pauVar20[0xb] + 8) = 0x102;
        *(undefined4 *)(pauVar20[0xb] + 0xc) = 3;
        *(undefined4 *)pauVar20[0xc] = uVar47;
        *(undefined4 *)(pauVar20[0xc] + 4) = 0;
        *(undefined8 *)(pauVar20[0xc] + 8) = 0xf;
        *(undefined8 **)pauVar20[0xd] = puStack_1a8;
        *(undefined8 *)(pauVar20[0xd] + 8) = 0xf;
        *(undefined2 *)pauVar20[0xe] = 0x102;
        *(undefined4 *)(pauVar20[0xe] + 4) = 9;
        *(undefined4 *)(pauVar20[0xe] + 8) = uVar47;
        *(undefined4 *)(pauVar20[0xe] + 0xc) = 0;
        *(undefined8 *)pauVar20[0xf] = 0x19;
        *(undefined8 **)(pauVar20[0xf] + 8) = puStack_1b0;
        *(undefined8 *)pauVar20[0x10] = 0x19;
        *(undefined2 *)(pauVar20[0x10] + 8) = 0x102;
        *(undefined4 *)(pauVar20[0x10] + 0xc) = 8;
        *(undefined4 *)pauVar20[0x11] = uVar47;
        *(undefined4 *)(pauVar20[0x11] + 4) = 0;
        *(undefined8 *)(pauVar20[0x11] + 8) = 0x14;
        *(undefined8 **)pauVar20[0x12] = puStack_1f8;
        *(undefined8 *)(pauVar20[0x12] + 8) = 0x14;
        *(undefined2 *)pauVar20[0x13] = 0x102;
        *(undefined4 *)(pauVar20[0x13] + 4) = 2;
        *(undefined4 *)(pauVar20[0x13] + 8) = uVar47;
        *(undefined4 *)(pauVar20[0x13] + 0xc) = 0;
        *(undefined8 *)pauVar20[0x14] = 0x1b;
        *(undefined1 (**) [16])(pauVar20[0x14] + 8) = pauVar43;
        uVar31 = _UNK_14303c349;
        *puVar25 = _DAT_14303c341;
        puVar25[1] = uVar31;
        *(undefined8 *)((longlong)puVar25 + 0xd) = 0x6e656572675f666f;
        *(undefined8 *)pauVar20[0x15] = 0x1b;
        *(undefined2 *)(pauVar20[0x15] + 8) = 0x102;
        *(undefined4 *)(pauVar20[0x15] + 0xc) = 6;
        *(undefined4 *)pauVar20[0x16] = uVar47;
        *(undefined4 *)(pauVar20[0x16] + 4) = 0;
        *(undefined8 *)(pauVar20[0x16] + 8) = 0x15;
        *(undefined8 **)pauVar20[0x17] = puVar25;
        *(undefined8 *)(pauVar20[0x17] + 8) = 0x15;
        *(undefined2 *)pauVar20[0x18] = 0x102;
        *(undefined4 *)(pauVar20[0x18] + 4) = 5;
        *(undefined4 *)(pauVar20[0x18] + 8) = uVar47;
        puVar29 = (undefined4 *)(pauVar20[0x18] + 0xc);
        pauStack_220 = (undefined1 (*) [16])0xa;
        uVar15 = (uint)puStack_228;
LAB_14229c890:
        *puVar29 = 0;
        pauStack_218 = pauVar20;
        uStack_210 = pauStack_220;
        pauVar20 = (undefined1 (*) [16])func_0x000140613c10(0x118,8);
        if (pauVar20 == (undefined1 (*) [16])0x0) {
          FUN_1428d9000(8,0x118);
          goto LAB_1422a2ea6;
        }
        pauVar19 = (undefined1 (*) [16])func_0x000140613c10(5,1);
        if (pauVar19 == (undefined1 (*) [16])0x0) {
          pauStack_108 = pauVar20;
          FUN_1428d8fe3(1,5,&UNK_143136c68);
          goto LAB_1422a2ea6;
        }
        (*pauVar19)[4] = DAT_143136d57;
        *(undefined4 *)*pauVar19 = _DAT_143136d53;
        pauVar43 = (undefined1 (*) [16])func_0x000140613c10(0x10,1);
        uVar31 = _UNK_14303c434;
        if (pauVar43 == (undefined1 (*) [16])0x0) {
          uStack_110 = pauVar19;
          pauStack_108 = pauVar20;
          FUN_1428d8fe3(1,0x10,&UNK_143136c68);
          goto LAB_1422a2ea6;
        }
        *(undefined8 *)*pauVar43 = _DAT_14303c42c;
        *(undefined8 *)(*pauVar43 + 8) = uVar31;
        unaff_R13 = (undefined1 (*) [16])func_0x000140613c10(0x12,1);
        uVar31 = _UNK_14303c444;
        if (unaff_R13 == (undefined1 (*) [16])0x0) {
          uStack_110 = pauVar19;
          pauStack_108 = pauVar20;
          pauStack_f8 = pauVar43;
          FUN_1428d8fe3(1,0x12,&UNK_143136c68);
          goto LAB_1422a2ea6;
        }
        *(undefined8 *)*unaff_R13 = _DAT_14303c43c;
        *(undefined8 *)(*unaff_R13 + 8) = uVar31;
        *(undefined2 *)unaff_R13[1] = 0x6e6f;
        pauVar18 = (undefined1 (*) [16])func_0x000140613c10(0x12,1);
        uVar31 = _UNK_14303c456;
        if (pauVar18 == (undefined1 (*) [16])0x0) {
          uStack_110 = pauVar19;
          pauStack_108 = pauVar20;
          pauStack_100 = unaff_R13;
          pauStack_f8 = pauVar43;
          FUN_1428d8fe3(1,0x12,&UNK_143136c68);
          goto LAB_1422a2ea6;
        }
        *(undefined8 *)*pauVar18 = _DAT_14303c44e;
        *(undefined8 *)(*pauVar18 + 8) = uVar31;
        *(undefined2 *)pauVar18[1] = 0x6e6f;
        pauStack_120 = pauVar18;
        pauStack_108 = pauVar24;
        pauVar24 = (undefined1 (*) [16])func_0x000140613c10(0x11,1);
        uVar31 = _UNK_14303c468;
        if (pauVar24 == (undefined1 (*) [16])0x0) {
          uStack_110 = pauVar19;
          pauStack_108 = pauVar20;
          pauStack_100 = unaff_R13;
          pauStack_f8 = pauVar43;
          FUN_1428d8fe3(1,0x11,&UNK_143136c68);
          goto LAB_1422a2ea6;
        }
        uVar47 = (undefined4)uStack_110;
        *(undefined8 *)*pauVar24 = _DAT_14303c460;
        *(undefined8 *)(*pauVar24 + 8) = uVar31;
        pauVar24[1][0] = 0x6e;
        pauStack_138 = pauVar24;
        pauVar18 = (undefined1 (*) [16])func_0x000140613c10(0x13,1);
        uVar31 = _UNK_14303c479;
        if (pauVar18 == (undefined1 (*) [16])0x0) {
          uStack_110 = pauVar19;
          pauStack_108 = pauVar20;
          pauStack_100 = unaff_R13;
          pauStack_f8 = pauVar43;
          FUN_1428d8fe3(1,0x13,&UNK_143136c68);
          goto LAB_1422a2ea6;
        }
        *(undefined8 *)*pauVar18 = _DAT_14303c471;
        *(undefined8 *)(*pauVar18 + 8) = uVar31;
        *(undefined4 *)(*pauVar18 + 0xf) = 0x6e6f6369;
        puVar25 = (undefined8 *)func_0x000140613c10(0x13,1);
        uVar31 = _UNK_14303c48c;
        if (puVar25 != (undefined8 *)0x0) {
          *puVar25 = _DAT_14303c484;
          puVar25[1] = uVar31;
          *(undefined4 *)((longlong)puVar25 + 0xf) = 0x6e6f6369;
          *(undefined8 *)*pauVar20 = 5;
          *(undefined1 (**) [16])(*pauVar20 + 8) = pauVar19;
          *(undefined8 *)pauVar20[1] = 5;
          pauVar20[1][8] = 5;
          *(undefined4 *)(pauVar20[2] + 4) = 0;
          *(undefined8 *)(pauVar20[2] + 8) = 0x10;
          *(undefined1 (**) [16])pauVar20[3] = pauVar43;
          *(undefined8 *)(pauVar20[3] + 8) = 0x10;
          *(undefined2 *)pauVar20[4] = 2;
          *(undefined4 *)(pauVar20[4] + 4) = uVar47;
          *(undefined8 *)(pauVar20[4] + 8) = 0;
          *(undefined8 *)pauVar20[5] = 0x12;
          *(undefined1 (**) [16])(pauVar20[5] + 8) = unaff_R13;
          *(undefined8 *)pauVar20[6] = 0x12;
          *(undefined2 *)(pauVar20[6] + 8) = 2;
          *(undefined4 *)(pauVar20[6] + 0xc) = uVar47;
          *(undefined8 *)pauVar20[7] = 1;
          *(undefined8 *)(pauVar20[7] + 8) = 0x12;
          *(undefined1 (**) [16])pauVar20[8] = pauStack_120;
          *(undefined8 *)(pauVar20[8] + 8) = 0x12;
          *(undefined2 *)pauVar20[9] = 2;
          *(undefined4 *)(pauVar20[9] + 4) = uVar47;
          *(undefined8 *)(pauVar20[9] + 8) = 2;
          *(undefined8 *)pauVar20[10] = 0x11;
          *(undefined1 (**) [16])(pauVar20[10] + 8) = pauStack_138;
          *(undefined8 *)pauVar20[0xb] = 0x11;
          *(undefined2 *)(pauVar20[0xb] + 8) = 0x102;
          *(uint *)(pauVar20[0xb] + 0xc) = uVar15;
          *(undefined8 *)pauVar20[0xc] = 0;
          *(undefined8 *)(pauVar20[0xc] + 8) = 0x13;
          *(undefined1 (**) [16])pauVar20[0xd] = pauVar18;
          *(undefined8 *)(pauVar20[0xd] + 8) = 0x13;
          *(undefined2 *)pauVar20[0xe] = 0x102;
          *(uint *)(pauVar20[0xe] + 4) = uVar15;
          *(undefined8 *)(pauVar20[0xe] + 8) = 2;
          *(undefined8 *)pauVar20[0xf] = 0x13;
          *(undefined8 **)(pauVar20[0xf] + 8) = puVar25;
          *(undefined8 *)pauVar20[0x10] = 0x13;
          *(undefined2 *)(pauVar20[0x10] + 8) = 0x102;
          *(uint *)(pauVar20[0x10] + 0xc) = uVar15;
          *(undefined8 *)pauVar20[0x11] = 1;
          puVar38 = pauVar20[0x11] + 8;
          uStack_168._0_4_ = 7;
          uStack_168._4_4_ = 0;
          uStack_160 = SUB84(puVar38,0);
          uStack_15c = (undefined1)((ulonglong)puVar38 >> 0x20);
          uStack_15b = (undefined1)((ulonglong)puVar38 >> 0x28);
          uStack_15a = (undefined1)((ulonglong)puVar38 >> 0x30);
          uStack_159 = (undefined1)((ulonglong)puVar38 >> 0x38);
          uStack_178 = pauVar20;
          uStack_170 = pauVar20;
          if ((ulonglong)((longlong)pauStack_220 - (longlong)uStack_210) < 7) {
            FUN_1429107f0(&pauStack_220,uStack_210,7,8,0x28);
          }
          pauVar24 = pauStack_108;
          pauVar19 = uStack_210;
          func_0x000142923c00((undefined *)((longlong)uStack_210 * 0x28 + (longlong)pauStack_218),
                              pauVar20,0x118);
          uStack_210 = (undefined1 (*) [16])(*pauVar19 + 7);
          func_0x000140613c20(pauVar20,0x118,8);
          pauStack_1e8 = pauStack_220;
          pauStack_1e0 = pauStack_218;
          pauStack_1d8 = uStack_210;
          pauVar20 = pauStack_1e8;
          pauVar23 = pauStack_1e0;
          pauVar19 = pauStack_1d8;
          break;
        }
      }
      else {
        FUN_1428d9518(uVar30,*(ulonglong *)pauVar18[1],&UNK_14303c398);
      }
      pauStack_118 = pauVar18;
      uStack_110 = pauVar19;
      pauStack_108 = pauVar20;
      pauStack_100 = unaff_R13;
      pauStack_f8 = pauVar43;
      FUN_1428d8fe3(1,0x13,&UNK_143136c68);
      goto LAB_1422a2ea6;
    case 3:
override_jmp_14229aa20_case_3:
      uVar47 = *(undefined4 *)(*pauVar24 + 4);
      cVar14 = (*pauVar24)[1];
      pauStack_118 = pauVar22;
      FUN_1422acce0(&uStack_178);
      pauVar20 = uStack_178;
      pauVar23 = uStack_170;
      pauVar19 = uStack_168;
      if (cVar14 != '\0') {
        pauStack_220 = (undefined1 (*) [16])func_0x000140b26a70(p11);
        pauStack_218 = pauVar22;
        cVar14 = func_0x000140b26970(&pauStack_220,uVar47);
        pauVar20 = uStack_178;
        pauVar23 = uStack_170;
        pauVar19 = uStack_168;
        if (1 < (byte)(cVar14 - 9U)) {
          pauVar19 = (undefined1 (*) [16])func_0x000140613c10(0x1b,1);
          uVar31 = _UNK_14303c4aa;
          if (pauVar19 == (undefined1 (*) [16])0x0) {
            FUN_1428d8fe3(1,0x1b,&UNK_143136c68);
            goto LAB_1422a2ea6;
          }
          *(ulonglong *)(*pauVar19 + 0xb) = CONCAT35(_UNK_14303c4a7,_DAT_14303c4a2);
          *(undefined8 *)(pauVar19[1] + 3) = uVar31;
          uVar31 = CONCAT53(_DAT_14303c4a2,_UNK_14303c49f);
          *(undefined8 *)*pauVar19 = _DAT_14303c497;
          *(undefined8 *)(*pauVar19 + 8) = uVar31;
          pauVar20 = (undefined1 (*) [16])CONCAT44(uStack_168._4_4_,(uint)uStack_168);
          pauStack_108 = pauVar19;
          if (pauVar20 == uStack_178) {
            FUN_140562ab0(&uStack_178,&UNK_14303c4b8);
          }
          pauVar19 = uStack_170;
          if (pauVar20 != (undefined1 (*) [16])0x0) {
            func_0x000142923c10(uStack_170[2] + 8,uStack_170,(longlong)pauVar20 * 0x28);
          }
          *(undefined8 *)*pauVar19 = 0x1b;
          *(undefined1 (**) [16])(*pauVar19 + 8) = pauStack_108;
          *(undefined8 *)pauVar19[1] = 0x1b;
          pauVar19[1][8] = 3;
          *(undefined4 *)(pauVar19[1] + 0xc) = 999;
          *(undefined4 *)(pauVar19[2] + 4) = 0;
          uStack_168 = (undefined1 (*) [16])(*pauVar20 + 1);
          pauVar20 = uStack_178;
          pauVar23 = uStack_170;
          pauVar19 = uStack_168;
        }
      }
      break;
    default:
override_jmp_14229aa20_case_4:
      FUN_1428d9430(&UNK_14303c5f0,0x28,&UNK_14303c618);
      goto override_jmp_14229ea79_case_4;
    case 5:
    case 8:
override_jmp_14229aa20_case_5:
      pauStack_118 = pauVar22;
      FUN_1422acce0(&pauStack_1e8);
      pauVar20 = pauStack_1e8;
      pauVar23 = pauStack_1e0;
      pauVar19 = pauStack_1d8;
      break;
    case 6:
override_jmp_14229aa20_case_6:
      uVar47 = *(undefined4 *)(*pauVar24 + 4);
      pauStack_118 = pauVar22;
      lVar16 = FUN_140a1c930(pauStack_f8,uVar47);
      if (lVar16 == 0) {
        bVar36 = false;
      }
      else {
        bVar13 = *(char *)(lVar16 + 0x22) - 2;
        bVar36 = (bVar13 + 1 & 0x1fd) == 0 || 2 < bVar13;
      }
      pauVar43 = (undefined1 (*) [16])&uStack_178;
      pauVar19 = (undefined1 (*) [16])func_0x000140613c10(400,8);
      if (pauVar19 == (undefined1 (*) [16])0x0) {
LAB_1422a0f87:
        FUN_1428d9000(8,400);
        goto override_jmp_14229aa20_case_4;
      }
      pauVar20 = (undefined1 (*) [16])func_0x000140613c10(0x14,1);
      uVar31 = _UNK_14303c4d8;
      if (pauVar20 == (undefined1 (*) [16])0x0) {
        pauStack_f8 = pauVar19;
        FUN_1428d8fe3(1,0x14,&UNK_143136c68);
        goto LAB_1422a2ea6;
      }
      *(undefined8 *)*pauVar20 = _DAT_14303c4d0;
      *(undefined8 *)(*pauVar20 + 8) = uVar31;
      *(undefined4 *)pauVar20[1] = 0x6465722f;
      pauVar18 = (undefined1 (*) [16])func_0x000140613c10(0x15,1);
      uVar31 = _UNK_14303c4ec;
      if (pauVar18 == (undefined1 (*) [16])0x0) {
        pauStack_100 = pauVar20;
        pauStack_f8 = pauVar19;
        FUN_1428d8fe3(1,0x15,&UNK_143136c68);
        goto LAB_1422a2ea6;
      }
      *(undefined8 *)*pauVar18 = _DAT_14303c4e4;
      *(undefined8 *)(*pauVar18 + 8) = uVar31;
      *(undefined8 *)(*pauVar18 + 0xd) = 0x6b6e69702f736761;
      pauVar21 = (undefined1 (*) [16])func_0x000140613c10(0x17,1);
      uVar31 = _UNK_14303c501;
      if (pauVar21 == (undefined1 (*) [16])0x0) {
        pauStack_118 = pauVar18;
        pauStack_100 = pauVar20;
        pauStack_f8 = pauVar19;
        FUN_1428d8fe3(1,0x17,&UNK_143136c68);
        goto LAB_1422a2ea6;
      }
      *(undefined8 *)*pauVar21 = _DAT_14303c4f9;
      *(undefined8 *)(*pauVar21 + 8) = uVar31;
      *(undefined8 *)(*pauVar21 + 0xf) = 0x776f6c6c65792f73;
      pauStack_108 = pauVar21;
      pauVar21 = (undefined1 (*) [16])func_0x000140613c10(0x16,1);
      uVar31 = _UNK_14303c518;
      if (pauVar21 == (undefined1 (*) [16])0x0) {
        pauStack_118 = pauVar18;
        pauStack_100 = pauVar20;
        pauStack_f8 = pauVar19;
        FUN_1428d8fe3(1,0x16,&UNK_143136c68);
        goto LAB_1422a2ea6;
      }
      *(undefined8 *)*pauVar21 = _DAT_14303c510;
      *(undefined8 *)(*pauVar21 + 8) = uVar31;
      *(undefined8 *)(*pauVar21 + 0xe) = 0x6e656572672f7367;
      uStack_110 = pauVar21;
      pauVar21 = (undefined1 (*) [16])func_0x000140613c10(0x19,1);
      uVar31 = _UNK_14303c537;
      if (pauVar21 == (undefined1 (*) [16])0x0) {
        pauStack_118 = pauVar18;
        pauStack_100 = pauVar20;
        pauStack_f8 = pauVar19;
        FUN_1428d8fe3(1,0x19,&UNK_143136c68);
        goto LAB_1422a2ea6;
      }
      *(ulonglong *)(*pauVar21 + 9) = CONCAT17(UNK_14303c536,_DAT_14303c52f);
      *(undefined8 *)(pauVar21[1] + 1) = uVar31;
      uVar31 = CONCAT71(_DAT_14303c52f,UNK_14303c52e);
      *(undefined8 *)*pauVar21 = _DAT_14303c526;
      *(undefined8 *)(*pauVar21 + 8) = uVar31;
      pauStack_120 = pauVar21;
      pauVar21 = (undefined1 (*) [16])func_0x000140613c10(0x15,1);
      uVar31 = _UNK_14303c547;
      if (pauVar21 == (undefined1 (*) [16])0x0) {
        pauStack_118 = pauVar18;
        pauStack_100 = pauVar20;
        pauStack_f8 = pauVar19;
        FUN_1428d8fe3(1,0x15,&UNK_143136c68);
        goto LAB_1422a2ea6;
      }
      *(undefined8 *)*pauVar21 = _DAT_14303c53f;
      *(undefined8 *)(*pauVar21 + 8) = uVar31;
      *(undefined8 *)(*pauVar21 + 0xd) = 0x65756c622f736761;
      pauStack_138 = pauVar21;
      puVar25 = (undefined8 *)func_0x000140613c10(0x1d,1);
      uVar31 = _UNK_14303c569;
      if (puVar25 == (undefined8 *)0x0) {
        pauStack_118 = pauVar18;
        pauStack_100 = pauVar20;
        pauStack_f8 = pauVar19;
        FUN_1428d8fe3(1,0x1d,&UNK_143136c68);
        goto LAB_1422a2ea6;
      }
      puStack_1f8 = (undefined8 *)CONCAT44(puStack_1f8._4_4_,uVar47);
      *(ulonglong *)((longlong)puVar25 + 0xd) = CONCAT53(_UNK_14303c564,_DAT_14303c561);
      *(undefined8 *)((longlong)puVar25 + 0x15) = uVar31;
      uVar31 = CONCAT35(_DAT_14303c561,_UNK_14303c55c);
      *puVar25 = _DAT_14303c554;
      puVar25[1] = uVar31;
      puStack_1a8 = puVar25;
      puVar25 = (undefined8 *)func_0x000140613c10(0x17,1);
      uVar31 = _UNK_14303c579;
      if (puVar25 == (undefined8 *)0x0) {
        pauStack_118 = pauVar18;
        pauStack_100 = pauVar20;
        pauStack_f8 = pauVar19;
        FUN_1428d8fe3(1,0x17,&UNK_143136c68);
        goto LAB_1422a2ea6;
      }
      *puVar25 = _DAT_14303c571;
      puVar25[1] = uVar31;
      *(undefined8 *)((longlong)puVar25 + 0xf) = 0x656c707275702f73;
      puStack_1b0 = puVar25;
      pauVar21 = (undefined1 (*) [16])func_0x000140613c10(0x1d,1);
      uVar31 = _UNK_14303c59d;
      if (pauVar21 == (undefined1 (*) [16])0x0) {
        pauStack_118 = pauVar18;
        pauStack_100 = pauVar20;
        pauStack_f8 = pauVar19;
        FUN_1428d8fe3(1,0x1d,&UNK_143136c68);
        goto LAB_1422a2ea6;
      }
      *(ulonglong *)(*pauVar21 + 0xd) = CONCAT53(_UNK_14303c598,_DAT_14303c595);
      *(undefined8 *)(pauVar21[1] + 5) = uVar31;
      uVar31 = CONCAT35(_DAT_14303c595,_UNK_14303c590);
      *(undefined8 *)*pauVar21 = _DAT_14303c588;
      *(undefined8 *)(*pauVar21 + 8) = uVar31;
      puVar25 = (undefined8 *)func_0x000140613c10(0x15,1);
      uVar31 = _UNK_14303c5ad;
      if (puVar25 == (undefined8 *)0x0) {
        pauStack_130 = pauVar21;
        pauStack_118 = pauVar18;
        pauStack_100 = pauVar20;
        pauStack_f8 = pauVar19;
        FUN_1428d8fe3(1,0x15,&UNK_143136c68);
        goto LAB_1422a2ea6;
      }
      *puVar25 = _DAT_14303c5a5;
      puVar25[1] = uVar31;
      *(undefined8 *)((longlong)puVar25 + 0xd) = 0x6b7261642f736761;
      *(undefined8 *)*pauVar19 = 0x14;
      *(undefined1 (**) [16])(*pauVar19 + 8) = pauVar20;
      *(undefined8 *)pauVar19[1] = 0x14;
      pauVar19[1][8] = 3;
      *(undefined4 *)(pauVar19[1] + 0xc) = 1;
      *(undefined4 *)(pauVar19[2] + 4) = 0;
      *(undefined8 *)(pauVar19[2] + 8) = 0x15;
      *(undefined1 (**) [16])pauVar19[3] = pauVar18;
      *(undefined8 *)(pauVar19[3] + 8) = 0x15;
      pauVar19[4][0] = 3;
      *(undefined4 *)(pauVar19[4] + 4) = 7;
      *(undefined4 *)(pauVar19[4] + 0xc) = 0;
      *(undefined8 *)pauVar19[5] = 0x17;
      *(undefined1 (**) [16])(pauVar19[5] + 8) = pauStack_108;
      *(undefined8 *)pauVar19[6] = 0x17;
      pauVar19[6][8] = 3;
      *(undefined4 *)(pauVar19[6] + 0xc) = 3;
      *(undefined4 *)(pauVar19[7] + 4) = 0;
      *(undefined8 *)(pauVar19[7] + 8) = 0x16;
      *(undefined1 (**) [16])pauVar19[8] = uStack_110;
      *(undefined8 *)(pauVar19[8] + 8) = 0x16;
      pauVar19[9][0] = 3;
      *(undefined4 *)(pauVar19[9] + 4) = 2;
      *(undefined4 *)(pauVar19[9] + 0xc) = 0;
      *(undefined8 *)pauVar19[10] = 0x19;
      *(undefined1 (**) [16])(pauVar19[10] + 8) = pauStack_120;
      *(undefined8 *)pauVar19[0xb] = 0x19;
      pauVar19[0xb][8] = 3;
      *(undefined4 *)(pauVar19[0xb] + 0xc) = 9;
      *(undefined4 *)(pauVar19[0xc] + 4) = 0;
      *(undefined8 *)(pauVar19[0xc] + 8) = 0x15;
      *(undefined1 (**) [16])pauVar19[0xd] = pauStack_138;
      *(undefined8 *)(pauVar19[0xd] + 8) = 0x15;
      pauVar19[0xe][0] = 3;
      *(undefined4 *)(pauVar19[0xe] + 4) = 0;
      *(undefined4 *)(pauVar19[0xe] + 0xc) = 0;
      *(undefined8 *)pauVar19[0xf] = 0x1d;
      *(undefined8 **)(pauVar19[0xf] + 8) = puStack_1a8;
      *(undefined8 *)pauVar19[0x10] = 0x1d;
      pauVar19[0x10][8] = 3;
      *(undefined4 *)(pauVar19[0x10] + 0xc) = 6;
      *(undefined4 *)(pauVar19[0x11] + 4) = 0;
      *(undefined8 *)(pauVar19[0x11] + 8) = 0x17;
      *(undefined8 **)pauVar19[0x12] = puStack_1b0;
      *(undefined8 *)(pauVar19[0x12] + 8) = 0x17;
      pauVar19[0x13][0] = 3;
      *(undefined4 *)(pauVar19[0x13] + 4) = 4;
      *(undefined4 *)(pauVar19[0x13] + 0xc) = 0;
      *(undefined8 *)pauVar19[0x14] = 0x1d;
      *(undefined1 (**) [16])(pauVar19[0x14] + 8) = pauVar21;
      *(undefined8 *)pauVar19[0x15] = 0x1d;
      pauVar19[0x15][8] = 3;
      *(undefined4 *)(pauVar19[0x15] + 0xc) = 8;
      *(undefined4 *)(pauVar19[0x16] + 4) = 0;
      *(undefined8 *)(pauVar19[0x16] + 8) = 0x15;
      *(undefined8 **)pauVar19[0x17] = puVar25;
      *(undefined8 *)(pauVar19[0x17] + 8) = 0x15;
      pauVar19[0x18][0] = 3;
      *(undefined4 *)(pauVar19[0x18] + 4) = 5;
      *(undefined4 *)(pauVar19[0x18] + 0xc) = 0;
      uStack_178 = (undefined1 (*) [16])0xa;
      uStack_168._0_4_ = 10;
      uStack_168._4_4_ = 0;
      uStack_168 = (undefined1 (*) [16])0xa;
      uStack_170 = pauVar19;
      if (bVar36) {
        lVar16 = FUN_140a1c930(pauStack_f8,(ulonglong)puStack_1f8 & 0xffffffff);
        if (lVar16 == 0) {
          cVar14 = '\x01';
        }
        else {
          pcVar27 = (char *)FUN_140839340(lVar16 + 0x30,&UNK_14303c9e0);
          cVar14 = *pcVar27;
        }
        pauVar19 = (undefined1 (*) [16])func_0x000140613c10(5,1);
        if (pauVar19 == (undefined1 (*) [16])0x0) {
          FUN_1428d8fe3(1,5,&UNK_143136c68);
          goto LAB_1422a2ea6;
        }
        (*pauVar19)[4] = DAT_143136d57;
        *(undefined4 *)*pauVar19 = _DAT_143136d53;
        pauVar20 = (undefined1 (*) [16])CONCAT44(uStack_168._4_4_,(uint)uStack_168);
        pauStack_108 = pauVar19;
        if (pauVar20 == uStack_178) {
          FUN_140562ab0(pauVar43,&UNK_14303c5c0);
        }
        *(undefined8 *)((longlong)uStack_170 + (longlong)pauVar20 * 0x28) = 5;
        *(undefined1 (**) [16])((longlong)uStack_170 + (longlong)pauVar20 * 0x28 + 8) = pauStack_108
        ;
        *(undefined8 *)((longlong)uStack_170 + (longlong)pauVar20 * 0x28 + 0x10) = 5;
        *(undefined1 *)((longlong)uStack_170 + (longlong)pauVar20 * 0x28 + 0x18) = 5;
        *(undefined4 *)((longlong)uStack_170 + (longlong)pauVar20 * 0x28 + 0x24) = 0;
        uStack_168 = (undefined1 (*) [16])(*pauVar20 + 1);
        if (cVar14 == '\0') {
          pauVar19 = (undefined1 (*) [16])func_0x000140613c10(0x1d,1);
          uVar31 = _UNK_143136d6d;
          if (pauVar19 == (undefined1 (*) [16])0x0) {
            FUN_1428d8fe3(1,0x1d,&UNK_143136c68);
            goto LAB_1422a2ea6;
          }
          *(ulonglong *)(*pauVar19 + 0xd) = CONCAT53(_UNK_143136d68,_DAT_143136d65);
          *(undefined8 *)(pauVar19[1] + 5) = uVar31;
          uVar31 = CONCAT35(_DAT_143136d65,_UNK_143136d60);
          *(undefined8 *)*pauVar19 = _DAT_143136d58;
          *(undefined8 *)(*pauVar19 + 8) = uVar31;
          uStack_110 = (undefined1 (*) [16])0x1d;
          pauStack_108 = pauVar19;
          pauVar19 = uStack_168;
          if (uStack_168 == uStack_178) goto LAB_14229e7c0;
        }
        else {
          pauVar19 = (undefined1 (*) [16])func_0x000140613c10(0x1a,1);
          uVar31 = _UNK_143136d87;
          if (pauVar19 == (undefined1 (*) [16])0x0) {
            FUN_1428d8fe3(1,0x1a,&UNK_143136c68);
            goto LAB_1422a2ea6;
          }
          *(ulonglong *)(*pauVar19 + 10) = CONCAT26(_UNK_143136d85,_DAT_143136d7f);
          *(undefined8 *)(pauVar19[1] + 2) = uVar31;
          uVar31 = CONCAT62(_DAT_143136d7f,_UNK_143136d7d);
          *(undefined8 *)*pauVar19 = _DAT_143136d75;
          *(undefined8 *)(*pauVar19 + 8) = uVar31;
          uStack_110 = (undefined1 (*) [16])0x1a;
          pauStack_108 = pauVar19;
          pauVar19 = uStack_168;
          if (uStack_168 == uStack_178) {
LAB_14229e7c0:
            uStack_168 = pauVar19;
            FUN_140562ab0(pauVar43,&UNK_14303c5d8);
            uStack_168 = pauVar19;
          }
        }
        *(undefined1 (**) [16])((longlong)uStack_170 + (longlong)uStack_168 * 0x28) = uStack_110;
        *(undefined1 (**) [16])((longlong)uStack_170 + (longlong)uStack_168 * 0x28 + 8) =
             pauStack_108;
        *(undefined1 (**) [16])((longlong)uStack_170 + (longlong)uStack_168 * 0x28 + 0x10) =
             uStack_110;
        *(undefined1 *)((longlong)uStack_170 + (longlong)uStack_168 * 0x28 + 0x18) = 6;
        *(undefined4 *)((longlong)uStack_170 + (longlong)uStack_168 * 0x28 + 0x24) = 0;
        uStack_168 = (undefined1 (*) [16])(*uStack_168 + 1);
      }
      pauStack_1e8 = uStack_178;
      pauStack_1e0 = uStack_170;
      pauVar20 = pauStack_1e8;
      pauVar23 = pauStack_1e0;
      pauVar19 = uStack_168;
      break;
    case 9:
override_jmp_14229aa20_case_9:
      pauStack_118 = pauVar22;
      pauVar19 = (undefined1 (*) [16])func_0x000140613c10(0x168,8);
      if (pauVar19 == (undefined1 (*) [16])0x0) {
LAB_1422a0fdc:
        uVar30 = FUN_1428d9000(8,0x168);
        goto LAB_1422a0feb;
      }
      pauVar20 = (undefined1 (*) [16])func_0x000140613c10(0x16,1);
      uVar31 = _UNK_14303c638;
      if (pauVar20 == (undefined1 (*) [16])0x0) {
        pauStack_108 = pauVar19;
        FUN_1428d8fe3(1,0x16,&UNK_143136c68);
      }
      else {
        *(undefined8 *)*pauVar20 = _DAT_14303c630;
        *(undefined8 *)(*pauVar20 + 8) = uVar31;
        *(undefined8 *)(*pauVar20 + 0xe) = 0x776f6c6c65795f67;
        pauVar18 = (undefined1 (*) [16])func_0x000140613c10(0x16,1);
        uVar31 = _UNK_14303c64e;
        if (pauVar18 == (undefined1 (*) [16])0x0) {
          pauStack_108 = pauVar19;
          pauStack_f8 = pauVar20;
          FUN_1428d8fe3(1,0x16,&UNK_143136c68);
        }
        else {
          *(undefined8 *)*pauVar18 = _DAT_14303c646;
          *(undefined8 *)(*pauVar18 + 8) = uVar31;
          *(undefined8 *)(*pauVar18 + 0xe) = 0x6465722f73776f64;
          pauVar21 = (undefined1 (*) [16])func_0x000140613c10(0x19,1);
          uVar31 = _UNK_14303c424;
          if (pauVar21 == (undefined1 (*) [16])0x0) {
            pauStack_108 = pauVar19;
            pauStack_100 = pauVar18;
            pauStack_f8 = pauVar20;
            FUN_1428d8fe3(1,0x19,&UNK_143136c68);
          }
          else {
            *(ulonglong *)(*pauVar21 + 9) = CONCAT17(UNK_14303c423,_DAT_14303c41c);
            *(undefined8 *)(pauVar21[1] + 1) = uVar31;
            uVar31 = CONCAT71(_DAT_14303c41c,UNK_14303c41b);
            *(undefined8 *)*pauVar21 = _DAT_14303c413;
            *(undefined8 *)(*pauVar21 + 8) = uVar31;
            pauVar43 = (undefined1 (*) [16])func_0x000140613c10(0x17,1);
            uVar31 = _UNK_14303c664;
            if (pauVar43 == (undefined1 (*) [16])0x0) {
              pauStack_118 = pauVar21;
              pauStack_108 = pauVar19;
              pauStack_100 = pauVar18;
              pauStack_f8 = pauVar20;
              FUN_1428d8fe3(1,0x17,&UNK_143136c68);
            }
            else {
              *(undefined8 *)*pauVar43 = _DAT_14303c65c;
              *(undefined8 *)(*pauVar43 + 8) = uVar31;
              *(undefined8 *)(*pauVar43 + 0xf) = 0x65756c622f73776f;
              pauVar22 = (undefined1 (*) [16])func_0x000140613c10(0x1b,1);
              uVar31 = _UNK_14303c339;
              if (pauVar22 == (undefined1 (*) [16])0x0) {
                pauStack_138 = pauVar43;
                pauStack_118 = pauVar21;
                pauStack_108 = pauVar19;
                pauStack_100 = pauVar18;
                pauStack_f8 = pauVar20;
                FUN_1428d8fe3(1,0x1b,&UNK_143136c68);
              }
              else {
                *(ulonglong *)(*pauVar22 + 0xb) = CONCAT35(_UNK_14303c336,_DAT_14303c331);
                *(undefined8 *)(pauVar22[1] + 3) = uVar31;
                uVar31 = CONCAT53(_DAT_14303c331,_UNK_14303c32e);
                *(undefined8 *)*pauVar22 = _DAT_14303c326;
                *(undefined8 *)(*pauVar22 + 8) = uVar31;
                pauVar23 = (undefined1 (*) [16])func_0x000140613c10(5,1);
                if (pauVar23 == (undefined1 (*) [16])0x0) {
                  pauStack_138 = pauVar43;
                  pauStack_130 = pauVar22;
                  pauStack_118 = pauVar21;
                  pauStack_108 = pauVar19;
                  pauStack_100 = pauVar18;
                  pauStack_f8 = pauVar20;
                  FUN_1428d8fe3(1,5,&UNK_143136c68);
                }
                else {
                  (*pauVar23)[4] = DAT_143136d57;
                  *(undefined4 *)*pauVar23 = _DAT_143136d53;
                  uStack_110 = pauVar23;
                  pauVar23 = (undefined1 (*) [16])func_0x000140613c10(0x12,1);
                  uVar31 = _UNK_14303c67b;
                  if (pauVar23 == (undefined1 (*) [16])0x0) {
                    pauStack_138 = pauVar43;
                    pauStack_130 = pauVar22;
                    pauStack_118 = pauVar21;
                    pauStack_108 = pauVar19;
                    pauStack_100 = pauVar18;
                    pauStack_f8 = pauVar20;
                    FUN_1428d8fe3(1,0x12,&UNK_143136c68);
                  }
                  else {
                    *(undefined8 *)*pauVar23 = _DAT_14303c673;
                    *(undefined8 *)(*pauVar23 + 8) = uVar31;
                    *(undefined2 *)pauVar23[1] = 0x315f;
                    pauStack_120 = pauVar23;
                    pauStack_108 = pauVar24;
                    pauVar24 = (undefined1 (*) [16])func_0x000140613c10(0x12,1);
                    uVar31 = _UNK_14303c68d;
                    if (pauVar24 == (undefined1 (*) [16])0x0) {
                      pauStack_138 = pauVar43;
                      pauStack_130 = pauVar22;
                      pauStack_118 = pauVar21;
                      pauStack_108 = pauVar19;
                      pauStack_100 = pauVar18;
                      pauStack_f8 = pauVar20;
                      FUN_1428d8fe3(1,0x12,&UNK_143136c68);
                    }
                    else {
                      *(undefined8 *)*pauVar24 = _DAT_14303c685;
                      *(undefined8 *)(*pauVar24 + 8) = uVar31;
                      *(undefined2 *)pauVar24[1] = 0x325f;
                      puVar25 = (undefined8 *)func_0x000140613c10(0x12,1);
                      uVar31 = _UNK_14303c69f;
                      if (puVar25 != (undefined8 *)0x0) {
                        *puVar25 = _DAT_14303c697;
                        puVar25[1] = uVar31;
                        *(undefined2 *)(puVar25 + 2) = 0x335f;
                        *(undefined8 *)*pauVar19 = 0x16;
                        *(undefined1 (**) [16])(*pauVar19 + 8) = pauVar20;
                        *(undefined8 *)pauVar19[1] = 0x16;
                        pauVar19[1][8] = 3;
                        *(undefined4 *)(pauVar19[1] + 0xc) = 0;
                        *(undefined4 *)(pauVar19[2] + 4) = 0;
                        *(undefined8 *)(pauVar19[2] + 8) = 0x16;
                        *(undefined1 (**) [16])pauVar19[3] = pauVar18;
                        *(undefined8 *)(pauVar19[3] + 8) = 0x16;
                        pauVar19[4][0] = 3;
                        *(undefined4 *)(pauVar19[4] + 4) = 4;
                        *(undefined4 *)(pauVar19[4] + 0xc) = 0;
                        *(undefined8 *)pauVar19[5] = 0x19;
                        *(undefined1 (**) [16])(pauVar19[5] + 8) = pauVar21;
                        *(undefined8 *)pauVar19[6] = 0x19;
                        pauVar19[6][8] = 3;
                        *(undefined4 *)(pauVar19[6] + 0xc) = 3;
                        *(undefined4 *)(pauVar19[7] + 4) = 0;
                        *(undefined8 *)(pauVar19[7] + 8) = 0x17;
                        *(undefined1 (**) [16])pauVar19[8] = pauVar43;
                        *(undefined8 *)(pauVar19[8] + 8) = 0x17;
                        pauVar19[9][0] = 3;
                        *(undefined4 *)(pauVar19[9] + 4) = 1;
                        *(undefined4 *)(pauVar19[9] + 0xc) = 0;
                        *(undefined8 *)pauVar19[10] = 0x1b;
                        *(undefined1 (**) [16])(pauVar19[10] + 8) = pauVar22;
                        *(undefined8 *)pauVar19[0xb] = 0x1b;
                        pauVar19[0xb][8] = 3;
                        *(undefined4 *)(pauVar19[0xb] + 0xc) = 2;
                        *(undefined4 *)(pauVar19[0xc] + 4) = 0;
                        *(undefined8 *)(pauVar19[0xc] + 8) = 5;
                        *(undefined1 (**) [16])pauVar19[0xd] = uStack_110;
                        *(undefined8 *)(pauVar19[0xd] + 8) = 5;
                        pauVar19[0xe][0] = 5;
                        *(undefined4 *)(pauVar19[0xe] + 0xc) = 0;
                        *(undefined8 *)pauVar19[0xf] = 0x12;
                        *(undefined1 (**) [16])(pauVar19[0xf] + 8) = pauStack_120;
                        *(undefined8 *)pauVar19[0x10] = 0x12;
                        *(undefined2 *)(pauVar19[0x10] + 8) = 9;
                        *(undefined4 *)(pauVar19[0x11] + 4) = 0;
                        *(undefined8 *)(pauVar19[0x11] + 8) = 0x12;
                        *(undefined1 (**) [16])pauVar19[0x12] = pauVar24;
                        *(undefined8 *)(pauVar19[0x12] + 8) = 0x12;
                        *(undefined2 *)pauVar19[0x13] = 0x109;
                        *(undefined4 *)(pauVar19[0x13] + 0xc) = 0;
                        *(undefined8 *)pauVar19[0x14] = 0x12;
                        *(undefined8 **)(pauVar19[0x14] + 8) = puVar25;
                        *(undefined8 *)pauVar19[0x15] = 0x12;
                        *(undefined2 *)(pauVar19[0x15] + 8) = 0x209;
                        *(undefined4 *)(pauVar19[0x16] + 4) = 0;
                        pauStack_1e8 = (undefined1 (*) [16])0x9;
                        pauStack_1d8 = (undefined1 (*) [16])0x9;
                        pauStack_1e0 = pauVar19;
LAB_14229c615:
                        pauVar24 = pauStack_108;
                        pauVar20 = pauStack_1e8;
                        pauVar23 = pauStack_1e0;
                        pauVar19 = pauStack_1d8;
                        break;
                      }
                      pauStack_148 = pauVar24;
                      pauStack_138 = pauVar43;
                      pauStack_130 = pauVar22;
                      pauStack_118 = pauVar21;
                      pauStack_108 = pauVar19;
                      pauStack_100 = pauVar18;
                      pauStack_f8 = pauVar20;
                      FUN_1428d8fe3(1,0x12,&UNK_143136c68);
                    }
                  }
                }
              }
            }
          }
        }
      }
      goto LAB_1422a2ea6;
    case 10:
override_jmp_14229aa20_case_10:
      uVar42 = *(uint *)(*pauVar24 + 4);
      pauVar20 = (undefined1 (*) [16])(ulonglong)uVar42;
      pauStack_118 = pauVar22;
      uStack_178 = (undefined1 (*) [16])func_0x000140b26a70(p11);
      uStack_170 = pauVar22;
      uVar15 = func_0x000140b26970(&uStack_178,pauVar20);
      pauVar21 = (undefined1 (*) [16])(ulonglong)uVar15;
      lVar16 = FUN_140a1c930(pauStack_f8,uVar42);
      if (lVar16 == 0) {
        pauVar18 = (undefined1 (*) [16])CONCAT71((int7)((ulonglong)pauVar18 >> 8),1);
        if ((char)uVar15 != '\x14') goto LAB_14229dc5f;
LAB_14229aee1:
        pauVar23 = (undefined1 (*) [16])func_0x000140613c10(0x28,8);
        if (pauVar23 == (undefined1 (*) [16])0x0) {
          FUN_1428d9000(8,0x28);
          goto LAB_1422a1f89;
        }
        if ((char)pauVar18 == '\0') {
          puVar25 = (undefined8 *)func_0x000140613c10(0x21,1);
          if (puVar25 == (undefined8 *)0x0) {
            pauStack_120 = pauVar23;
            FUN_1428d8fe3(1,0x21,&UNK_143136c68);
            goto LAB_1422a2ea6;
          }
          *(undefined1 *)(puVar25 + 4) = DAT_143136daf;
          uVar31 = _UNK_143136da7;
          puVar25[2] = _DAT_143136d9f;
          puVar25[3] = uVar31;
          uVar31 = _UNK_143136d97;
          *puVar25 = _DAT_143136d8f;
          puVar25[1] = uVar31;
          uVar31 = 0x21;
        }
        else {
          puVar25 = (undefined8 *)func_0x000140613c10(0x1f,1);
          uVar31 = _UNK_143136dc7;
          if (puVar25 == (undefined8 *)0x0) {
            pauStack_120 = pauVar23;
            FUN_1428d8fe3(1,0x1f,&UNK_143136c68);
            goto LAB_1422a2ea6;
          }
          *(ulonglong *)((longlong)puVar25 + 0xf) = CONCAT71(_UNK_143136dc0,DAT_143136dbf);
          *(undefined8 *)((longlong)puVar25 + 0x17) = uVar31;
          uVar31 = CONCAT17(DAT_143136dbf,_UNK_143136db8);
          *puVar25 = _DAT_143136db0;
          puVar25[1] = uVar31;
          uVar31 = 0x1f;
        }
        *(undefined8 *)*pauVar23 = uVar31;
        *(undefined8 **)(*pauVar23 + 8) = puVar25;
        *(undefined8 *)pauVar23[1] = uVar31;
        pauVar23[1][8] = 10;
        pauVar19 = pauVar23 + 2;
        pauVar20 = (undefined1 (*) [16])0x1;
      }
      else {
        lVar16 = FUN_1408392d0(lVar16 + 0x30,&UNK_14303c9f8);
        pauVar18 = (undefined1 (*) [16])(ulonglong)*(byte *)(lVar16 + 4);
        if ((char)uVar15 == '\x14') goto LAB_14229aee1;
LAB_14229dc5f:
        pauVar23 = (undefined1 (*) [16])func_0x000140613c10(0x118,8);
        if (pauVar23 == (undefined1 (*) [16])0x0) {
          FUN_1428d9000(8,0x118);
LAB_1422a2356:
          pauStack_118 = pauVar21;
          pauStack_100 = pauVar20;
          pauStack_f8 = pauVar18;
          FUN_1428d8fe3(1,0x11,&UNK_143136c68);
          goto LAB_1422a2ea6;
        }
        pauVar19 = (undefined1 (*) [16])func_0x000140613c10(0x17,1);
        uVar31 = _UNK_14303c160;
        if (pauVar19 == (undefined1 (*) [16])0x0) {
          pauStack_120 = pauVar23;
          FUN_1428d8fe3(1,0x17,&UNK_143136c68);
          goto LAB_1422a2ea6;
        }
        *(undefined8 *)*pauVar19 = _DAT_14303c158;
        *(undefined8 *)(*pauVar19 + 8) = uVar31;
        *(undefined8 *)(*pauVar19 + 0xf) = 0x65676965625f736b;
        pauVar20 = (undefined1 (*) [16])func_0x000140613c10(0x18,1);
        uVar31 = _UNK_14303c177;
        if (pauVar20 == (undefined1 (*) [16])0x0) {
          pauStack_120 = pauVar23;
          pauStack_f8 = pauVar19;
          FUN_1428d8fe3(1,0x18,&UNK_143136c68);
          goto LAB_1422a2ea6;
        }
        *(undefined8 *)*pauVar20 = _DAT_14303c16f;
        *(undefined8 *)(*pauVar20 + 8) = uVar31;
        *(undefined8 *)pauVar20[1] = 0x776f6c6c65795f73;
        pauVar43 = (undefined1 (*) [16])func_0x000140613c10(0x17,1);
        uVar31 = _UNK_14303c18f;
        if (pauVar43 == (undefined1 (*) [16])0x0) {
          pauStack_120 = pauVar23;
          pauStack_100 = pauVar20;
          pauStack_f8 = pauVar19;
          FUN_1428d8fe3(1,0x17,&UNK_143136c68);
          goto LAB_1422a2ea6;
        }
        *(undefined8 *)*pauVar43 = _DAT_14303c187;
        *(undefined8 *)(*pauVar43 + 8) = uVar31;
        *(undefined8 *)(*pauVar43 + 0xf) = 0x746867696c5f736b;
        pauVar21 = (undefined1 (*) [16])func_0x000140613c10(0x16,1);
        uVar31 = _UNK_14303c1a6;
        if (pauVar21 == (undefined1 (*) [16])0x0) {
          pauStack_120 = pauVar23;
          pauStack_118 = pauVar43;
          pauStack_100 = pauVar20;
          pauStack_f8 = pauVar19;
          FUN_1428d8fe3(1,0x16,&UNK_143136c68);
          goto LAB_1422a2ea6;
        }
        *(undefined8 *)*pauVar21 = _DAT_14303c19e;
        *(undefined8 *)(*pauVar21 + 8) = uVar31;
        *(undefined8 *)(*pauVar21 + 0xe) = 0x796572675f736b63;
        pauVar22 = (undefined1 (*) [16])func_0x000140613c10(0x16,1);
        uVar31 = _UNK_14303c1bc;
        if (pauVar22 == (undefined1 (*) [16])0x0) {
          pauStack_138 = pauVar21;
          pauStack_120 = pauVar23;
          pauStack_118 = pauVar43;
          pauStack_100 = pauVar20;
          pauStack_f8 = pauVar19;
          FUN_1428d8fe3(1,0x16,&UNK_143136c68);
          goto LAB_1422a2ea6;
        }
        *(undefined8 *)*pauVar22 = _DAT_14303c1b4;
        *(undefined8 *)(*pauVar22 + 8) = uVar31;
        *(undefined8 *)(*pauVar22 + 0xe) = 0x6b7261645f736b63;
        pauStack_108 = pauVar22;
        pauVar22 = (undefined1 (*) [16])func_0x000140613c10(5,1);
        if (pauVar22 == (undefined1 (*) [16])0x0) {
          pauStack_138 = pauVar21;
          pauStack_120 = pauVar23;
          pauStack_118 = pauVar43;
          pauStack_100 = pauVar20;
          pauStack_f8 = pauVar19;
          FUN_1428d8fe3(1,5,&UNK_143136c68);
          goto LAB_1422a2ea6;
        }
        (*pauVar22)[4] = DAT_143136d57;
        *(undefined4 *)*pauVar22 = _DAT_143136d53;
        uStack_110 = pauVar22;
        if ((char)pauVar18 == '\0') {
          puVar25 = (undefined8 *)func_0x000140613c10(0x21,1);
          if (puVar25 == (undefined8 *)0x0) {
            pauStack_138 = pauVar21;
            pauStack_120 = pauVar23;
            pauStack_118 = pauVar43;
            pauStack_100 = pauVar20;
            pauStack_f8 = pauVar19;
            FUN_1428d8fe3(1,0x21,&UNK_143136c68);
            goto LAB_1422a2ea6;
          }
          *(undefined1 *)(puVar25 + 4) = DAT_143136daf;
          uVar31 = _UNK_143136da7;
          puVar25[2] = _DAT_143136d9f;
          puVar25[3] = uVar31;
          uVar31 = _UNK_143136d97;
          *puVar25 = _DAT_143136d8f;
          puVar25[1] = uVar31;
          uVar31 = 0x21;
        }
        else {
          puVar25 = (undefined8 *)func_0x000140613c10(0x1f,1);
          uVar31 = _UNK_143136dc7;
          if (puVar25 == (undefined8 *)0x0) {
            pauStack_138 = pauVar21;
            pauStack_120 = pauVar23;
            pauStack_118 = pauVar43;
            pauStack_100 = pauVar20;
            pauStack_f8 = pauVar19;
            FUN_1428d8fe3(1,0x1f,&UNK_143136c68);
            goto LAB_1422a2ea6;
          }
          *(ulonglong *)((longlong)puVar25 + 0xf) = CONCAT71(_UNK_143136dc0,DAT_143136dbf);
          *(undefined8 *)((longlong)puVar25 + 0x17) = uVar31;
          uVar31 = CONCAT17(DAT_143136dbf,_UNK_143136db8);
          *puVar25 = _DAT_143136db0;
          puVar25[1] = uVar31;
          uVar31 = 0x1f;
        }
        *(undefined8 *)*pauVar23 = 0x17;
        *(undefined1 (**) [16])(*pauVar23 + 8) = pauVar19;
        *(undefined8 *)pauVar23[1] = 0x17;
        pauVar23[1][8] = 3;
        *(undefined4 *)(pauVar23[1] + 0xc) = 100000;
        *(undefined4 *)(pauVar23[2] + 4) = 0;
        *(undefined8 *)(pauVar23[2] + 8) = 0x18;
        *(undefined1 (**) [16])pauVar23[3] = pauVar20;
        *(undefined8 *)(pauVar23[3] + 8) = 0x18;
        pauVar23[4][0] = 3;
        *(undefined4 *)(pauVar23[4] + 4) = 0x186a1;
        *(undefined4 *)(pauVar23[4] + 0xc) = 0;
        *(undefined8 *)pauVar23[5] = 0x17;
        *(undefined1 (**) [16])(pauVar23[5] + 8) = pauVar43;
        *(undefined8 *)pauVar23[6] = 0x17;
        pauVar23[6][8] = 3;
        *(undefined4 *)(pauVar23[6] + 0xc) = 0x186a3;
        *(undefined4 *)(pauVar23[7] + 4) = 0;
        *(undefined8 *)(pauVar23[7] + 8) = 0x16;
        *(undefined1 (**) [16])pauVar23[8] = pauVar21;
        *(undefined8 *)(pauVar23[8] + 8) = 0x16;
        pauVar23[9][0] = 3;
        *(undefined4 *)(pauVar23[9] + 4) = 0x186ab;
        *(undefined4 *)(pauVar23[9] + 0xc) = 0;
        *(undefined8 *)pauVar23[10] = 0x16;
        *(undefined1 (**) [16])(pauVar23[10] + 8) = pauStack_108;
        *(undefined8 *)pauVar23[0xb] = 0x16;
        pauVar23[0xb][8] = 3;
        *(undefined4 *)(pauVar23[0xb] + 0xc) = 0x186a2;
        *(undefined4 *)(pauVar23[0xc] + 4) = 0;
        *(undefined8 *)(pauVar23[0xc] + 8) = 5;
        *(undefined1 (**) [16])pauVar23[0xd] = uStack_110;
        *(undefined8 *)(pauVar23[0xd] + 8) = 5;
        pauVar23[0xe][0] = 5;
        *(undefined4 *)(pauVar23[0xe] + 0xc) = 0;
        *(undefined8 *)pauVar23[0xf] = uVar31;
        *(undefined8 **)(pauVar23[0xf] + 8) = puVar25;
        *(undefined8 *)pauVar23[0x10] = uVar31;
        pauVar23[0x10][8] = 10;
        pauVar19 = pauVar23 + 0x11;
        pauVar20 = (undefined1 (*) [16])0x7;
      }
      *(undefined4 *)(*pauVar19 + 4) = 0;
      pauVar19 = pauVar20;
      break;
    case 0xb:
override_jmp_14229aa20_case_11:
      pauStack_118 = pauVar22;
      pauVar18 = (undefined1 (*) [16])func_0x000140613c10(400,8);
      if (pauVar18 == (undefined1 (*) [16])0x0) goto LAB_1422a0f87;
      pauVar19 = (undefined1 (*) [16])func_0x000140613c10(0x17,1);
      uVar31 = _UNK_14303c160;
      if (pauVar19 == (undefined1 (*) [16])0x0) {
LAB_1422a13a9:
        pauStack_108 = pauVar18;
        FUN_1428d8fe3(1,0x17,&UNK_143136c68);
      }
      else {
        *(undefined8 *)*pauVar19 = _DAT_14303c158;
        *(undefined8 *)(*pauVar19 + 8) = uVar31;
        *(undefined8 *)(*pauVar19 + 0xf) = 0x65676965625f736b;
        pauVar20 = (undefined1 (*) [16])func_0x000140613c10(0x18,1);
        uVar31 = _UNK_14303c177;
        if (pauVar20 == (undefined1 (*) [16])0x0) {
          pauStack_108 = pauVar18;
          pauStack_f8 = pauVar19;
          FUN_1428d8fe3(1,0x18,&UNK_143136c68);
        }
        else {
          *(undefined8 *)*pauVar20 = _DAT_14303c16f;
          *(undefined8 *)(*pauVar20 + 8) = uVar31;
          *(undefined8 *)pauVar20[1] = 0x776f6c6c65795f73;
          pauVar21 = (undefined1 (*) [16])func_0x000140613c10(0x17,1);
          uVar31 = _UNK_14303c18f;
          if (pauVar21 == (undefined1 (*) [16])0x0) {
            pauStack_108 = pauVar18;
            pauStack_100 = pauVar20;
            pauStack_f8 = pauVar19;
            FUN_1428d8fe3(1,0x17,&UNK_143136c68);
          }
          else {
            *(undefined8 *)*pauVar21 = _DAT_14303c187;
            *(undefined8 *)(*pauVar21 + 8) = uVar31;
            *(undefined8 *)(*pauVar21 + 0xf) = 0x746867696c5f736b;
            pauVar43 = (undefined1 (*) [16])func_0x000140613c10(0x16,1);
            uVar31 = _UNK_14303c1a6;
            if (pauVar43 == (undefined1 (*) [16])0x0) {
              pauStack_118 = pauVar21;
              pauStack_108 = pauVar18;
              pauStack_100 = pauVar20;
              pauStack_f8 = pauVar19;
              FUN_1428d8fe3(1,0x16,&UNK_143136c68);
            }
            else {
              *(undefined8 *)*pauVar43 = _DAT_14303c19e;
              *(undefined8 *)(*pauVar43 + 8) = uVar31;
              *(undefined8 *)(*pauVar43 + 0xe) = 0x796572675f736b63;
              pauVar22 = (undefined1 (*) [16])func_0x000140613c10(0x16,1);
              uVar31 = _UNK_14303c1bc;
              if (pauVar22 == (undefined1 (*) [16])0x0) {
                pauStack_130 = pauVar43;
                pauStack_118 = pauVar21;
                pauStack_108 = pauVar18;
                pauStack_100 = pauVar20;
                pauStack_f8 = pauVar19;
                FUN_1428d8fe3(1,0x16,&UNK_143136c68);
              }
              else {
                *(undefined8 *)*pauVar22 = _DAT_14303c1b4;
                *(undefined8 *)(*pauVar22 + 8) = uVar31;
                *(undefined8 *)(*pauVar22 + 0xe) = 0x6b7261645f736b63;
                pauVar23 = (undefined1 (*) [16])func_0x000140613c10(5,1);
                if (pauVar23 == (undefined1 (*) [16])0x0) {
                  pauStack_148 = pauVar22;
                  pauStack_130 = pauVar43;
                  pauStack_118 = pauVar21;
                  pauStack_108 = pauVar18;
                  pauStack_100 = pauVar20;
                  pauStack_f8 = pauVar19;
                  FUN_1428d8fe3(1,5,&UNK_143136c68);
                }
                else {
                  (*pauVar23)[4] = DAT_143136d57;
                  *(undefined4 *)*pauVar23 = _DAT_143136d53;
                  uStack_110 = pauVar23;
                  pauVar23 = (undefined1 (*) [16])func_0x000140613c10(0x14,1);
                  if (pauVar23 == (undefined1 (*) [16])0x0) {
                    pauStack_148 = pauVar22;
                    pauStack_130 = pauVar43;
                    pauStack_118 = pauVar21;
                    pauStack_108 = pauVar18;
                    pauStack_100 = pauVar20;
                    pauStack_f8 = pauVar19;
                    FUN_1428d8fe3(1,0x14,&UNK_143136c68);
                  }
                  else {
                    *(undefined4 *)pauVar23[1] = _DAT_143136ddf;
                    uVar31 = _UNK_143136dd7;
                    *(undefined8 *)*pauVar23 = _DAT_143136dcf;
                    *(undefined8 *)(*pauVar23 + 8) = uVar31;
                    pauStack_120 = pauVar23;
                    pauVar23 = (undefined1 (*) [16])func_0x000140613c10(0x14,1);
                    if (pauVar23 == (undefined1 (*) [16])0x0) {
                      pauStack_148 = pauVar22;
                      pauStack_130 = pauVar43;
                      pauStack_118 = pauVar21;
                      pauStack_108 = pauVar18;
                      pauStack_100 = pauVar20;
                      pauStack_f8 = pauVar19;
                      FUN_1428d8fe3(1,0x14,&UNK_143136c68);
                    }
                    else {
                      *(undefined4 *)pauVar23[1] = _DAT_143136df3;
                      uVar31 = _UNK_143136deb;
                      *(undefined8 *)*pauVar23 = _DAT_143136de3;
                      *(undefined8 *)(*pauVar23 + 8) = uVar31;
                      pauStack_138 = pauVar23;
                      pauStack_108 = pauVar24;
                      pauVar24 = (undefined1 (*) [16])func_0x000140613c10(0x14,1);
                      if (pauVar24 == (undefined1 (*) [16])0x0) {
                        pauStack_148 = pauVar22;
                        pauStack_130 = pauVar43;
                        pauStack_118 = pauVar21;
                        pauStack_108 = pauVar18;
                        pauStack_100 = pauVar20;
                        pauStack_f8 = pauVar19;
                        FUN_1428d8fe3(1,0x14,&UNK_143136c68);
                      }
                      else {
                        *(undefined4 *)pauVar24[1] = _DAT_143136e07;
                        uVar31 = _UNK_143136dff;
                        *(undefined8 *)*pauVar24 = _DAT_143136df7;
                        *(undefined8 *)(*pauVar24 + 8) = uVar31;
                        puVar25 = (undefined8 *)func_0x000140613c10(0x14,1);
                        if (puVar25 != (undefined8 *)0x0) {
                          *(undefined4 *)(puVar25 + 2) = _DAT_143136e1b;
                          uVar31 = _UNK_143136e13;
                          *puVar25 = _DAT_143136e0b;
                          puVar25[1] = uVar31;
                          *(undefined8 *)*pauVar18 = 0x17;
                          *(undefined1 (**) [16])(*pauVar18 + 8) = pauVar19;
                          *(undefined8 *)pauVar18[1] = 0x17;
                          pauVar18[1][8] = 3;
                          *(undefined4 *)(pauVar18[1] + 0xc) = 100000;
                          *(undefined4 *)(pauVar18[2] + 4) = 0;
                          *(undefined8 *)(pauVar18[2] + 8) = 0x18;
                          *(undefined1 (**) [16])pauVar18[3] = pauVar20;
                          *(undefined8 *)(pauVar18[3] + 8) = 0x18;
                          pauVar18[4][0] = 3;
                          *(undefined4 *)(pauVar18[4] + 4) = 0x186a1;
                          *(undefined4 *)(pauVar18[4] + 0xc) = 0;
                          *(undefined8 *)pauVar18[5] = 0x17;
                          *(undefined1 (**) [16])(pauVar18[5] + 8) = pauVar21;
                          *(undefined8 *)pauVar18[6] = 0x17;
                          pauVar18[6][8] = 3;
                          *(undefined4 *)(pauVar18[6] + 0xc) = 0x186a3;
                          *(undefined4 *)(pauVar18[7] + 4) = 0;
                          *(undefined8 *)(pauVar18[7] + 8) = 0x16;
                          *(undefined1 (**) [16])pauVar18[8] = pauVar43;
                          *(undefined8 *)(pauVar18[8] + 8) = 0x16;
                          pauVar18[9][0] = 3;
                          *(undefined4 *)(pauVar18[9] + 4) = 0x186ab;
                          *(undefined4 *)(pauVar18[9] + 0xc) = 0;
                          *(undefined8 *)pauVar18[10] = 0x16;
                          *(undefined1 (**) [16])(pauVar18[10] + 8) = pauVar22;
                          *(undefined8 *)pauVar18[0xb] = 0x16;
                          pauVar18[0xb][8] = 3;
                          *(undefined4 *)(pauVar18[0xb] + 0xc) = 0x186a2;
                          *(undefined4 *)(pauVar18[0xc] + 4) = 0;
                          *(undefined8 *)(pauVar18[0xc] + 8) = 5;
                          *(undefined1 (**) [16])pauVar18[0xd] = uStack_110;
                          *(undefined8 *)(pauVar18[0xd] + 8) = 5;
                          pauVar18[0xe][0] = 5;
                          *(undefined4 *)(pauVar18[0xe] + 0xc) = 0;
                          *(undefined8 *)pauVar18[0xf] = 0x14;
                          *(undefined1 (**) [16])(pauVar18[0xf] + 8) = pauStack_120;
                          *(undefined8 *)pauVar18[0x10] = 0x14;
                          *(undefined2 *)(pauVar18[0x10] + 8) = 0xd;
                          *(undefined4 *)(pauVar18[0x11] + 4) = 0;
                          *(undefined8 *)(pauVar18[0x11] + 8) = 0x14;
                          *(undefined1 (**) [16])pauVar18[0x12] = pauStack_138;
                          *(undefined8 *)(pauVar18[0x12] + 8) = 0x14;
                          *(undefined2 *)pauVar18[0x13] = 0x10d;
                          *(undefined4 *)(pauVar18[0x13] + 0xc) = 0;
                          *(undefined8 *)pauVar18[0x14] = 0x14;
                          *(undefined1 (**) [16])(pauVar18[0x14] + 8) = pauVar24;
                          *(undefined8 *)pauVar18[0x15] = 0x14;
                          *(undefined2 *)(pauVar18[0x15] + 8) = 0x20d;
                          *(undefined4 *)(pauVar18[0x16] + 4) = 0;
                          *(undefined8 *)(pauVar18[0x16] + 8) = 0x14;
                          *(undefined8 **)pauVar18[0x17] = puVar25;
                          *(undefined8 *)(pauVar18[0x17] + 8) = 0x14;
                          *(undefined2 *)pauVar18[0x18] = 0x30d;
                          *(undefined4 *)(pauVar18[0x18] + 0xc) = 0;
                          pauStack_1e8 = (undefined1 (*) [16])0xa;
                          pauStack_1d8 = (undefined1 (*) [16])0xa;
                          pauStack_1e0 = pauVar18;
                          goto LAB_14229c615;
                        }
                        pauStack_180 = pauVar24;
                        pauStack_148 = pauVar22;
                        pauStack_130 = pauVar43;
                        pauStack_118 = pauVar21;
                        pauStack_108 = pauVar18;
                        pauStack_100 = pauVar20;
                        pauStack_f8 = pauVar19;
                        FUN_1428d8fe3(1,0x14,&UNK_143136c68);
                      }
                    }
                  }
                }
              }
            }
          }
        }
      }
      goto LAB_1422a2ea6;
    }
LAB_14229e9e0:
    pauStack_1d8 = pauVar19;
    pauStack_1e0 = pauVar23;
    pauStack_1e8 = pauVar20;
    lVar16 = (ulonglong)((*pauVar24)[0] == '\0') * 4;
    spacing = *(float *)(lStack_2f8 + 0xe8) * _DAT_1429cd2d4 * *(float *)(&UNK_14303bfe0 + lVar16);
    fVar51 = *(float *)(lStack_2f8 + 0xe8) * _DAT_142a16e40;
    radius = ((fVar51 + spacing) * (float)(longlong)pauStack_1d8) / _DAT_14303bfe8;
    if (radius <= fVar11) {
      radius = fVar11;
    }
    pauStack_1c8 = (undefined1 (*) [16])0x0;
    pauStack_1c0 = (undefined1 (*) [16])0x8;
    pauStack_1b8 = (undefined1 (*) [16])0x0;
                    /* WARNING: Switch is manually overridden */
    switch((*pauVar24)[0]) {
    case 0:
      uStack_178 = *(undefined1 (**) [16])(*pauVar24 + 8);
      uVar30 = func_0x0001400885d0(&uStack_178);
      pauVar19 = pauStack_180;
      if (*(ulonglong *)pauStack_180[1] <= uVar30) {
        FUN_1428d9518(uVar30,*(ulonglong *)pauStack_180[1],&UNK_14303c6b0);
        goto LAB_1422a2ea6;
      }
      break;
    case 1:
      pauVar19 = (undefined1 (*) [16])FUN_140a922b0(pauStack_100,*(undefined8 *)(*pauVar24 + 8));
      if ((ulonglong)pauVar19 >> 0x20 == 0) {
        FUN_1428d9310(&UNK_14303c6c8);
      }
      else {
        uVar30 = (ulonglong)pauVar19 & 0xffffffff;
        pauVar20 = pauStack_148;
        if ((uVar30 < *(ulonglong *)pauStack_148[1]) &&
           (pauVar20 = *(undefined1 (**) [16])(*pauStack_148 + 8),
           *(int *)((longlong)pauVar20 + uVar30 * 0x14) == (int)((ulonglong)pauVar19 >> 0x20))) {
          puVar38 = (undefined *)((longlong)pauVar20 + uVar30 * 0x14);
          uVar42 = *(uint *)(puVar38 + 4);
          pauVar20 = (undefined1 (*) [16])(ulonglong)uVar42;
          if (pauVar20 == (undefined1 (*) [16])0xffffffff) goto LAB_1422a0edf;
          uVar47 = 0;
          if ((*(undefined1 (**) [16])(lStack_270 + 0x38) <= pauVar20) ||
             ((*(ulonglong *)(*(longlong *)(lStack_270 + 0x28) + (ulonglong)(uVar42 >> 6) * 8) >>
               ((ulonglong)pauVar20 & 0x3f) & 1) == 0)) goto LAB_1422a0eef;
LAB_14229ec1d:
          pauStack_120 = (undefined1 (*) [16])
                         CONCAT44(pauStack_120._4_4_,
                                  *(undefined4 *)
                                   (*(longlong *)
                                     (*(longlong *)
                                       (*(longlong *)(pauStack_148[0x1a] + 8) + 0x18 +
                                       (ulonglong)*(uint *)(puVar38 + 0xc) * 0x48) + 0x10 +
                                     ~*(ulonglong *)
                                       (*(longlong *)
                                         (*(longlong *)(pauStack_148[0x1a] + 8) + 0x38 +
                                         (ulonglong)*(uint *)(puVar38 + 0xc) * 0x48) +
                                       *(longlong *)(lStack_270 + 0x110) * 8) * 0x30) + 0x50 +
                                   (ulonglong)*(uint *)(puVar38 + 0x10) * 0x58));
          cVar14 = (*pauVar24)[0];
          goto joined_r0x00014229ec76;
        }
LAB_1422a0edf:
        uVar42 = (uint)pauVar20;
        uVar47 = 1;
LAB_1422a0eef:
        uStack_178 = (undefined1 (*) [16])CONCAT44(uVar42,uVar47);
        uStack_170 = pauVar19;
        FUN_1428d9760(&UNK_14303c020,0x2b,&uStack_178,&UNK_14303c000,&UNK_14303c6e0);
      }
      goto LAB_1422a2ea6;
    case 2:
      pauVar19 = (undefined1 (*) [16])FUN_140a922b0(pauStack_100,*(undefined8 *)(*pauVar24 + 8));
      if ((ulonglong)pauVar19 >> 0x20 == 0) {
        FUN_1428d9310(&UNK_14303c6f8);
      }
      else {
        uVar30 = (ulonglong)pauVar19 & 0xffffffff;
        pauVar20 = pauStack_148;
        if ((uVar30 < *(ulonglong *)pauStack_148[1]) &&
           (pauVar20 = *(undefined1 (**) [16])(*pauStack_148 + 8),
           *(int *)((longlong)pauVar20 + uVar30 * 0x14) == (int)((ulonglong)pauVar19 >> 0x20))) {
          puVar38 = (undefined *)((longlong)pauVar20 + uVar30 * 0x14);
          uVar42 = *(uint *)(puVar38 + 4);
          pauVar20 = (undefined1 (*) [16])(ulonglong)uVar42;
          if (pauVar20 == (undefined1 (*) [16])0xffffffff) goto LAB_1422a0f33;
          uVar47 = 0;
          if ((pauVar20 < *(undefined1 (**) [16])(lStack_270 + 0x38)) &&
             ((*(ulonglong *)(*(longlong *)(lStack_270 + 0x28) + (ulonglong)(uVar42 >> 6) * 8) >>
               ((ulonglong)pauVar20 & 0x3f) & 1) != 0)) goto LAB_14229ec1d;
        }
        else {
LAB_1422a0f33:
          uVar42 = (uint)pauVar20;
          uVar47 = 1;
        }
        uStack_178 = (undefined1 (*) [16])CONCAT44(uVar42,uVar47);
        uStack_170 = pauVar19;
        FUN_1428d9760(&UNK_14303c020,0x2b,&uStack_178,&UNK_14303c000,&UNK_14303c710);
      }
      goto LAB_1422a2ea6;
    default:
      lVar34 = FUN_140a1c930(pauStack_f8,*(undefined4 *)(*pauVar24 + 4));
      if (lVar34 != 0) {
        uVar47 = *(undefined4 *)(lVar34 + 0x28);
        goto LAB_14229f058;
      }
      FUN_1428d9310(&UNK_14303c770);
      goto LAB_1422a2ea6;
    case 4:
    case 7:
override_jmp_14229ea79_case_4:
      FUN_1428d9430(&UNK_14303c5f0,0x28,&UNK_14303c740);
LAB_1422a2ea6:
                    /* WARNING: Does not return */
      pcVar5 = (code *)invalidInstructionException();
      (*pcVar5)();
    case 5:
      uStack_178 = *(undefined1 (**) [16])(*pauVar24 + 8);
      uVar30 = func_0x0001400885d0(&uStack_178);
      pauVar19 = pauStack_2d0;
      if (*(ulonglong *)pauStack_2d0[1] <= uVar30) {
        FUN_1428d9518(uVar30,*(ulonglong *)pauStack_2d0[1],&UNK_14303c728);
        goto LAB_1422a2ea6;
      }
      break;
    case 8:
      uStack_178 = *(undefined1 (**) [16])(*pauVar24 + 8);
      uVar30 = func_0x0001400885d0(&uStack_178);
      pauVar19 = pauStack_268;
      if (*(ulonglong *)pauStack_268[1] <= uVar30) {
        FUN_1428d9518(uVar30,*(ulonglong *)pauStack_268[1],&UNK_14303c758);
        goto LAB_1422a2ea6;
      }
      break;
    case 0xc:
      lVar34 = FUN_140a1c930(pauStack_f8,*(undefined4 *)(*pauVar24 + 4));
      if (lVar34 != 0) {
        lVar34 = FUN_1408394b0(lVar34 + 0x30,&UNK_14303c7a0);
        uVar47 = *(undefined4 *)(lVar34 + 8);
        goto LAB_14229f058;
      }
      FUN_1428d9310(&UNK_14303c788);
      goto LAB_1422a2ea6;
    }
    uVar47 = *(undefined4 *)(*(longlong *)(*pauVar19 + 8) + uVar30 * 4);
LAB_14229f058:
    pauStack_120 = (undefined1 (*) [16])CONCAT44(pauStack_120._4_4_,uVar47);
    cVar14 = (*pauVar24)[0];
joined_r0x00014229ec76:
    if (cVar14 == '\0') {
      uVar47 = 0;
      lVar34 = FUN_140a91ef0(pauStack_100,*(undefined8 *)(*pauVar24 + 8),&UNK_14303c7b8);
      if (lVar34 == 0) {
        FUN_1428d9310(&UNK_14303c7d0);
        goto LAB_1422a2ea6;
      }
      pauStack_138 = (undefined1 (*) [16])
                     CONCAT44(pauStack_138._4_4_,(uint)*(byte *)(lVar34 + 0x299));
      if ((*pauVar24)[0] == '\0') {
        lVar34 = FUN_140a91ef0(pauStack_100,*(undefined8 *)(*pauVar24 + 8),&UNK_14303c7e8);
        if (lVar34 != 0) {
          uVar47 = *(undefined4 *)(lVar34 + 0x290);
          cVar14 = (*pauVar24)[0];
          goto joined_r0x00014229f090;
        }
        FUN_1428d9310(&UNK_14303c800);
        goto LAB_1422a2ea6;
      }
      pauStack_108 = (undefined1 (*) [16])((ulonglong)pauStack_108 & 0xffffffff00000000);
      uStack_110 = (undefined1 (*) [16])((ulonglong)uStack_110 & 0xffffffff00000000);
      if ((*pauVar24)[0] != '\v') goto LAB_14229f096;
LAB_14229ecd8:
      lVar34 = FUN_140a1c930(pauStack_f8,*(undefined4 *)(*pauVar24 + 4));
      if (lVar34 == 0) {
LAB_14229ed20:
        uStack_110 = (undefined1 (*) [16])((ulonglong)uStack_110 & 0xffffffff00000000);
        cVar14 = (*pauVar24)[0];
      }
      else {
        lVar34 = func_0x000140839490(lVar34 + 0x30);
        if (lVar34 == 0) goto LAB_14229ed20;
        uStack_110 = (undefined1 (*) [16])CONCAT44(uStack_110._4_4_,(uint)*(byte *)(lVar34 + 0x10));
        cVar14 = (*pauVar24)[0];
      }
      if (cVar14 == '\v') goto LAB_14229f0a1;
LAB_14229ed40:
      if (cVar14 == '\x02') goto LAB_14229ed48;
LAB_14229f0e8:
      uVar35 = 0;
      if (cVar14 == '\x06') goto LAB_14229f0f2;
LAB_14229ee3d:
      pauVar43 = (undefined1 (*) [16])0x0;
joined_r0x00014229ee42:
      if (cVar14 == '\n') goto LAB_14229ee48;
LAB_14229f140:
      uVar44 = 0;
      cVar14 = (*pauVar24)[0];
joined_r0x00014229f148:
      if (cVar14 == '\0') goto LAB_14229ee82;
LAB_14229f14e:
      unaff_R13 = (undefined1 (*) [16])0x0;
      uVar37 = 0;
    }
    else {
      uVar47 = 0;
      pauStack_138 = (undefined1 (*) [16])((ulonglong)pauStack_138 & 0xffffffff00000000);
joined_r0x00014229f090:
      pauStack_108 = (undefined1 (*) [16])((ulonglong)pauStack_108 & 0xffffffff00000000);
      uStack_110 = (undefined1 (*) [16])((ulonglong)uStack_110 & 0xffffffff00000000);
      if (cVar14 == '\v') goto LAB_14229ecd8;
LAB_14229f096:
      cVar14 = (*pauVar24)[0];
      if (cVar14 != '\v') goto LAB_14229ed40;
LAB_14229f0a1:
      lVar34 = FUN_140a1c930(pauStack_f8,*(undefined4 *)(*pauVar24 + 4));
      pauStack_108 = (undefined1 (*) [16])((ulonglong)pauStack_108 & 0xffffffff00000000);
      if (lVar34 != 0) {
        lVar34 = func_0x000140839490(lVar34 + 0x30);
        if (lVar34 != 0) {
          pauStack_108 = (undefined1 (*) [16])
                         CONCAT44(pauStack_108._4_4_,*(undefined4 *)(lVar34 + 0xc));
        }
      }
      cVar14 = (*pauVar24)[0];
      if (cVar14 != '\x02') goto LAB_14229f0e8;
LAB_14229ed48:
      pauVar19 = (undefined1 (*) [16])FUN_140a922b0(pauStack_100,*(undefined8 *)(*pauVar24 + 8));
      if ((ulonglong)pauVar19 >> 0x20 == 0) {
        FUN_1428d9310(&UNK_14303c818);
        goto LAB_1422a2ea6;
      }
      uVar30 = (ulonglong)pauVar19 & 0xffffffff;
      pauVar20 = pauStack_148;
      if ((*(ulonglong *)pauStack_148[1] <= uVar30) ||
         (pauVar18 = *(undefined1 (**) [16])(*pauStack_148 + 8), pauVar20 = pauVar18,
         *(int *)((longlong)pauVar18 + uVar30 * 0x14) != (int)((ulonglong)pauVar19 >> 0x20))) {
LAB_1422a0e47:
        uVar42 = (uint)pauVar20;
        uVar46 = 1;
LAB_1422a0e57:
        uStack_178 = (undefined1 (*) [16])CONCAT44(uVar42,uVar46);
        uStack_170 = pauVar19;
        FUN_1428d9760(&UNK_14303c020,0x2b,&uStack_178,&UNK_14303c000,&UNK_14303c830);
        goto LAB_1422a2ea6;
      }
      uVar42 = *(uint *)((longlong)pauVar18 + uVar30 * 0x14 + 4);
      pauVar20 = (undefined1 (*) [16])(ulonglong)uVar42;
      if (pauVar20 == (undefined1 (*) [16])0xffffffff) goto LAB_1422a0e47;
      uVar46 = 0;
      if ((*(undefined1 (**) [16])(lStack_270 + 0x38) <= pauVar20) ||
         ((*(ulonglong *)(*(longlong *)(lStack_270 + 0x28) + (ulonglong)(uVar42 >> 6) * 8) >>
           ((ulonglong)pauVar20 & 0x3f) & 1) == 0)) goto LAB_1422a0e57;
      uVar17 = (ulonglong)*(uint *)((longlong)pauVar18 + uVar30 * 0x14 + 0xc);
      lVar34 = *(longlong *)
                (*(longlong *)(*(longlong *)(pauStack_148[0x1a] + 8) + 0x18 + uVar17 * 0x48) + 0x10
                + ~*(ulonglong *)
                    (*(longlong *)(*(longlong *)(pauStack_148[0x1a] + 8) + 0x38 + uVar17 * 0x48) +
                    *(longlong *)(lStack_270 + 0x110) * 8) * 0x30);
      lVar32 = (ulonglong)*(uint *)((longlong)pauVar18 + uVar30 * 0x14 + 0x10) * 0x58;
      uVar35 = *(undefined1 *)(lVar34 + 0x40 + lVar32);
      uVar31 = *(undefined8 *)(lVar34 + 0x41 + lVar32);
      uStack_2c8 = (undefined7)uVar31;
      uStack_2c1._1_3_ = (undefined3)((uint)*(undefined4 *)(lVar34 + 0x48 + lVar32) >> 8);
      uStack_2c1._0_1_ = (undefined1)((ulonglong)uVar31 >> 0x38);
      cVar14 = (*pauVar24)[0];
      if (cVar14 != '\x06') goto LAB_14229ee3d;
LAB_14229f0f2:
      lVar34 = FUN_140a1c930(pauStack_f8,*(undefined4 *)(*pauVar24 + 4));
      if (lVar34 == 0) {
        pauVar43 = (undefined1 (*) [16])CONCAT71((int7)((ulonglong)pauVar43 >> 8),1);
        cVar14 = (*pauVar24)[0];
        goto joined_r0x00014229ee42;
      }
      pbVar28 = (byte *)FUN_140839340(lVar34 + 0x30,&UNK_14303ca10);
      pauVar43 = (undefined1 (*) [16])(ulonglong)*pbVar28;
      if ((*pauVar24)[0] != '\n') goto LAB_14229f140;
LAB_14229ee48:
      lVar34 = FUN_140a1c930(pauStack_f8,*(undefined4 *)(*pauVar24 + 4));
      if (lVar34 != 0) {
        lVar34 = FUN_1408392d0(lVar34 + 0x30,&UNK_14303ca28);
        uVar44 = *(undefined1 *)(lVar34 + 4);
        cVar14 = (*pauVar24)[0];
        goto joined_r0x00014229f148;
      }
      uVar44 = 1;
      cVar14 = (*pauVar24)[0];
      if (cVar14 != '\0') goto LAB_14229f14e;
LAB_14229ee82:
      lVar34 = FUN_140a91ef0(pauStack_100,*(undefined8 *)(*pauVar24 + 8),&UNK_14303c848);
      if (lVar34 == 0) {
        FUN_1428d9310(&UNK_14303c860);
        goto LAB_1422a2ea6;
      }
      unaff_R13 = (undefined1 (*) [16])(ulonglong)*(byte *)(lVar34 + 0x295);
      cVar14 = (*pauVar24)[0];
      if (cVar14 != '\0') {
        uVar37 = 0;
        goto joined_r0x00014229efed;
      }
      uVar30 = FUN_140a922b0(pauStack_100,*(undefined8 *)(*pauVar24 + 8));
      if (((uVar30 >> 0x20 == 0) ||
          (uVar17 = uVar30 & 0xffffffff, *(ulonglong *)(lStack_1d0 + 0x10) <= uVar17)) ||
         (*(int *)(*(longlong *)(lStack_1d0 + 8) + uVar17 * 0x14) != (int)(uVar30 >> 0x20))) {
LAB_14229efe1:
        uVar37 = 0;
      }
      else {
        lVar34 = *(longlong *)(lStack_1d0 + 8) + uVar17 * 0x14;
        uVar30 = (ulonglong)*(uint *)(lVar34 + 4);
        if (((uVar30 == 0xffffffff) || (*(ulonglong *)(lStack_1f0 + 0x38) <= uVar30)) ||
           ((*(ulonglong *)
              (*(longlong *)(lStack_1f0 + 0x28) + (ulonglong)(*(uint *)(lVar34 + 4) >> 6) * 8) >>
             (uVar30 & 0x3f) & 1) == 0)) goto LAB_14229efe1;
        uVar37 = *(undefined1 *)
                  (*(longlong *)
                    (*(longlong *)
                      (*(longlong *)(lStack_1d0 + 0x1a8) + 0x18 +
                      (ulonglong)*(uint *)(lVar34 + 0xc) * 0x48) + 0x10 +
                    ~*(ulonglong *)
                      (*(longlong *)
                        (*(longlong *)(lStack_1d0 + 0x1a8) + 0x38 +
                        (ulonglong)*(uint *)(lVar34 + 0xc) * 0x48) +
                      *(longlong *)(lStack_1f0 + 0x110) * 8) * 0x30) + 0x3d +
                  (ulonglong)*(uint *)(lVar34 + 0x10) * 0x58);
      }
      cVar14 = (*pauVar24)[0];
    }
joined_r0x00014229efed:
    uVar46 = uVar10;
    if (cVar14 == '\t') {
      lVar34 = FUN_140a1c930(pauStack_f8,*(undefined4 *)(*pauVar24 + 4));
      if (lVar34 != 0) {
        puVar29 = (undefined4 *)FUN_140839420(lVar34 + 0x30,&UNK_14303ca40);
        uVar46 = *puVar29;
      }
    }
    fVar49 = *(float *)pauVar24[1];
    fVar48 = *(float *)(pauVar24[1] + 4);
    FUN_141c5a430(&uStack_1a0,&pauStack_1e8,&UNK_14303c878);
    uStack_248 = *(undefined1 (**) [16])*pauVar24;
    pauStack_240 = *(undefined1 (**) [16])(*pauVar24 + 8);
    uStack_168._4_4_ = (undefined4)((uint7)uStack_2c8 >> 0x18);
    uStack_160 = uStack_2c1;
    uStack_15b = SUB81(pauStack_138,0);
    uStack_15a = SUB81(pauVar43,0);
    uStack_159 = SUB81(unaff_R13,0);
    uStack_178 = (undefined1 (*) [16])CONCAT44(uVar46,(int)pauStack_120);
    uStack_168._0_4_ = CONCAT31((int3)uStack_2c8,uVar35);
    uStack_15c = SUB81(uStack_110,0);
    uStack_170 = (undefined1 (*) [16])CONCAT44(pauStack_108._0_4_,uVar47);
    uStack_181 = 1;
    pauVar20 = pauStack_130;
    uStack_158 = uVar37;
    uStack_157 = uVar44;
    FUN_140911dd0(&uStack_260);
    appearance_divisor = *(float *)(&UNK_14303bff0 + lVar16);
    uStack_181 = 0;
    color_create(&pauStack_220,fVar49,fVar48,radius,0.0,fVar51,spacing,appearance_divisor,
                 &uStack_1a0,&uStack_248,&uStack_178,&uStack_260,p14);
    pauVar19 = pauStack_1b8;
    if (pauStack_1b8 == pauStack_1c8) {
      pauVar20 = (undefined1 (*) [16])&UNK_14303c890;
      FUN_140562c30(&pauStack_1c8);
    }
    auVar1._8_8_ = pauStack_218;
    auVar1._0_8_ = pauStack_220;
    pauVar18 = pauStack_1c0 + (longlong)pauVar19 * 2 + 1;
    *(undefined4 *)*pauVar18 = (undefined4)uStack_210;
    *(undefined4 *)(*pauVar18 + 4) = uStack_210._4_4_;
    *(undefined4 *)(*pauVar18 + 8) = uStack_208;
    *(undefined4 *)(*pauVar18 + 0xc) = uStack_204;
    pauStack_1c0[(longlong)pauVar19 * 2] = auVar1;
    pauStack_1b8 = (undefined1 (*) [16])(*pauVar19 + 1);
    cVar14 = (*pauVar24)[0];
    if (cVar14 == '\0') {
      pauVar20 = *(undefined1 (**) [16])(*pauVar24 + 8);
      pauStack_280 = pauVar20;
      lVar16 = FUN_140a91ef0(pauStack_100,pauVar20,&UNK_14303c8a8);
      if ((lVar16 == 0) || (cVar14 = func_0x0001408c5670(), cVar14 == '\0')) {
        cVar14 = (*pauVar24)[0];
      }
      else {
        uVar30 = FUN_140a922b0(pauStack_100,pauStack_280);
        if (((uVar30 >> 0x20 == 0) ||
            (uVar17 = uVar30 & 0xffffffff, *(ulonglong *)(lStack_1d0 + 0x10) <= uVar17)) ||
           (*(int *)(*(longlong *)(lStack_1d0 + 8) + uVar17 * 0x14) != (int)(uVar30 >> 0x20))) {
LAB_14229f4fa:
          fVar48 = (float)*(undefined8 *)pauVar24[1];
          fVar50 = (float)((ulonglong)*(undefined8 *)pauVar24[1] >> 0x20);
          fVar49 = _DAT_14303bff8;
        }
        else {
          lVar16 = *(longlong *)(lStack_1d0 + 8) + uVar17 * 0x14;
          uVar30 = (ulonglong)*(uint *)(lVar16 + 4);
          if (((uVar30 == 0xffffffff) || (*(ulonglong *)(lStack_1f0 + 0x38) <= uVar30)) ||
             ((*(ulonglong *)
                (*(longlong *)(lStack_1f0 + 0x28) + (ulonglong)(*(uint *)(lVar16 + 4) >> 6) * 8) >>
               (uVar30 & 0x3f) & 1) == 0)) goto LAB_14229f4fa;
          cVar14 = func_0x0001408e3380((ulonglong)*(uint *)(lVar16 + 0x10) * 0x58 +
                                       *(longlong *)
                                        (*(longlong *)
                                          (*(longlong *)(lStack_1d0 + 0x1a8) + 0x18 +
                                          (ulonglong)*(uint *)(lVar16 + 0xc) * 0x48) + 0x10 +
                                        ~*(ulonglong *)
                                          (*(longlong *)
                                            (*(longlong *)(lStack_1d0 + 0x1a8) + 0x38 +
                                            (ulonglong)*(uint *)(lVar16 + 0xc) * 0x48) +
                                          *(longlong *)(lStack_1f0 + 0x110) * 8) * 0x30));
          fVar48 = (float)*(undefined8 *)pauVar24[1];
          fVar50 = (float)((ulonglong)*(undefined8 *)pauVar24[1] >> 0x20);
          fVar49 = _DAT_14303bff8;
          if (cVar14 != '\0') {
            fVar49 = _DAT_14303bffc;
          }
        }
        fVar52 = fVar51 * _DAT_142a89210;
        icon_size = fVar51 * _DAT_142a16cb8;
        FUN_1422acce0(&uStack_1a0);
        pauStack_240 = pauStack_280;
        uStack_248 = (undefined1 (*) [16])CONCAT71(uStack_248._1_7_,7);
        uStack_139 = 1;
        uVar30 = func_0x0001400885d0(&pauStack_280);
        if (*(ulonglong *)pauStack_268[1] <= uVar30) {
LAB_1422a0feb:
          uStack_139 = 1;
          FUN_1428d9518(uVar30);
          goto LAB_1422a2ea6;
        }
        uVar47 = *(undefined4 *)(*(longlong *)(*pauStack_268 + 8) + uVar30 * 4);
        uStack_139 = 1;
        uVar46 = func_0x000140b06780();
        uStack_15b = 0;
        uStack_15a = 0;
        uStack_159 = 0;
        uStack_158 = 0;
        uStack_178 = (undefined1 (*) [16])CONCAT44(uVar46,uVar47);
        uStack_157 = 0;
        uStack_15c = 0;
        uStack_168._0_4_ = (uint)uStack_168 & 0xffffff00;
        uStack_170 = (undefined1 (*) [16])0x0;
        uStack_139 = 1;
        pauVar20 = pauStack_130;
        FUN_140911dd0(&uStack_260);
        uStack_139 = 0;
        color_create(&pauStack_220,fVar48,fVar50,fVar52 + radius,fVar49,icon_size,spacing,
                     appearance_divisor,&uStack_1a0,&uStack_248,&uStack_178,&uStack_260,p14);
        pauVar19 = pauStack_1b8;
        if (pauStack_1b8 == pauStack_1c8) {
          pauVar20 = (undefined1 (*) [16])&UNK_14303c8d8;
          FUN_140562c30(&pauStack_1c8);
        }
        auVar2._8_8_ = pauStack_218;
        auVar2._0_8_ = pauStack_220;
        *(undefined1 (**) [16])pauStack_1c0[(longlong)pauVar19 * 2 + 1] = uStack_210;
        *(ulonglong *)(pauStack_1c0[(longlong)pauVar19 * 2 + 1] + 8) =
             CONCAT44(uStack_204,uStack_208);
        pauStack_1c0[(longlong)pauVar19 * 2] = auVar2;
        pauStack_1b8 = (undefined1 (*) [16])(*pauVar19 + 1);
        cVar14 = (*pauVar24)[0];
      }
    }
    pauVar19 = pauVar24;
    pauStack_108 = pauVar24;
    if (cVar14 == '\x03') {
      uVar42 = *(uint *)(*pauVar24 + 4);
      pauVar43 = (undefined1 (*) [16])(ulonglong)uVar42;
      uStack_260 = func_0x000140b26a70(p11);
      pauStack_258 = pauVar20;
      bVar13 = func_0x000140b26970(&uStack_260);
      pauVar24 = pauStack_108;
      if ((bVar13 < 0xb) && ((0x630U >> (bVar13 & 0x1f) & 1) != 0)) {
        cVar14 = (*pauStack_108)[0];
      }
      else {
        fVar49 = *(float *)pauStack_108[1];
        fVar48 = *(float *)(pauStack_108[1] + 4);
        fVar50 = fVar51 * _DAT_142a89210;
        fVar52 = fVar51 * _DAT_142a16cb8;
        pauVar20 = (undefined1 (*) [16])func_0x000140613c10(200,8);
        if (pauVar20 == (undefined1 (*) [16])0x0) {
          FUN_1428d9000(8,200);
          goto LAB_1422a2ea6;
        }
        pauVar19 = (undefined1 (*) [16])func_0x000140613c10(0x16,1);
        uVar31 = _UNK_14303c638;
        if (pauVar19 == (undefined1 (*) [16])0x0) {
          pauStack_108 = pauVar20;
          FUN_1428d8fe3(1,0x16,&UNK_143136c68);
          goto LAB_1422a2ea6;
        }
        *(undefined8 *)*pauVar19 = _DAT_14303c630;
        *(undefined8 *)(*pauVar19 + 8) = uVar31;
        *(undefined8 *)(*pauVar19 + 0xe) = 0x776f6c6c65795f67;
        pauVar18 = (undefined1 (*) [16])func_0x000140613c10(0x16,1);
        uVar31 = _UNK_14303c64e;
        if (pauVar18 == (undefined1 (*) [16])0x0) {
          uStack_110 = pauVar19;
          pauStack_108 = pauVar20;
          FUN_1428d8fe3(1,0x16,&UNK_143136c68);
          goto LAB_1422a2ea6;
        }
        *(undefined8 *)*pauVar18 = _DAT_14303c646;
        *(undefined8 *)(*pauVar18 + 8) = uVar31;
        *(undefined8 *)(*pauVar18 + 0xe) = 0x6465722f73776f64;
        unaff_R13 = (undefined1 (*) [16])func_0x000140613c10(0x19,1);
        uVar31 = _UNK_14303c424;
        if (unaff_R13 == (undefined1 (*) [16])0x0) {
          pauStack_120 = pauVar18;
          uStack_110 = pauVar19;
          pauStack_108 = pauVar20;
          FUN_1428d8fe3(1,0x19,&UNK_143136c68);
          goto LAB_1422a2ea6;
        }
        *(ulonglong *)(*unaff_R13 + 9) = CONCAT17(UNK_14303c423,_DAT_14303c41c);
        *(undefined8 *)(unaff_R13[1] + 1) = uVar31;
        uVar31 = CONCAT71(_DAT_14303c41c,UNK_14303c41b);
        *(undefined8 *)*unaff_R13 = _DAT_14303c413;
        *(undefined8 *)(*unaff_R13 + 8) = uVar31;
        pauVar21 = (undefined1 (*) [16])func_0x000140613c10(0x17,1);
        uVar31 = _UNK_14303c664;
        if (pauVar21 == (undefined1 (*) [16])0x0) {
          pauStack_120 = pauVar18;
          uStack_110 = pauVar19;
          pauStack_108 = pauVar20;
          pauStack_f8 = unaff_R13;
          FUN_1428d8fe3(1,0x17,&UNK_143136c68);
          goto LAB_1422a2ea6;
        }
        *(undefined8 *)*pauVar21 = _DAT_14303c65c;
        *(undefined8 *)(*pauVar21 + 8) = uVar31;
        *(undefined8 *)(*pauVar21 + 0xf) = 0x65756c622f73776f;
        puVar25 = (undefined8 *)func_0x000140613c10(0x1b,1);
        uVar31 = _UNK_14303c339;
        if (puVar25 == (undefined8 *)0x0) {
          pauStack_120 = pauVar18;
          uStack_110 = pauVar19;
          pauStack_108 = pauVar20;
          pauStack_100 = pauVar21;
          pauStack_f8 = unaff_R13;
          FUN_1428d8fe3(1,0x1b,&UNK_143136c68);
          goto LAB_1422a2ea6;
        }
        *(ulonglong *)((longlong)puVar25 + 0xb) = CONCAT35(_UNK_14303c336,_DAT_14303c331);
        *(undefined8 *)((longlong)puVar25 + 0x13) = uVar31;
        uVar31 = CONCAT53(_DAT_14303c331,_UNK_14303c32e);
        *puVar25 = _DAT_14303c326;
        puVar25[1] = uVar31;
        *(undefined8 *)*pauVar20 = 0x16;
        *(undefined1 (**) [16])(*pauVar20 + 8) = pauVar19;
        *(undefined8 *)pauVar20[1] = 0x16;
        pauVar20[1][8] = 3;
        *(undefined4 *)(pauVar20[1] + 0xc) = 0;
        *(undefined4 *)(pauVar20[2] + 4) = 0;
        *(undefined8 *)(pauVar20[2] + 8) = 0x16;
        *(undefined1 (**) [16])pauVar20[3] = pauVar18;
        *(undefined8 *)(pauVar20[3] + 8) = 0x16;
        pauVar20[4][0] = 3;
        *(undefined4 *)(pauVar20[4] + 4) = 4;
        *(undefined4 *)(pauVar20[4] + 0xc) = 0;
        *(undefined8 *)pauVar20[5] = 0x19;
        *(undefined1 (**) [16])(pauVar20[5] + 8) = unaff_R13;
        *(undefined8 *)pauVar20[6] = 0x19;
        pauVar20[6][8] = 3;
        *(undefined4 *)(pauVar20[6] + 0xc) = 3;
        *(undefined4 *)(pauVar20[7] + 4) = 0;
        *(undefined8 *)(pauVar20[7] + 8) = 0x17;
        *(undefined1 (**) [16])pauVar20[8] = pauVar21;
        *(undefined8 *)(pauVar20[8] + 8) = 0x17;
        pauVar20[9][0] = 3;
        *(undefined4 *)(pauVar20[9] + 4) = 1;
        *(undefined4 *)(pauVar20[9] + 0xc) = 0;
        *(undefined8 *)pauVar20[10] = 0x1b;
        *(undefined8 **)(pauVar20[10] + 8) = puVar25;
        *(undefined8 *)pauVar20[0xb] = 0x1b;
        pauVar20[0xb][8] = 3;
        *(undefined4 *)(pauVar20[0xb] + 0xc) = 2;
        *(undefined4 *)(pauVar20[0xc] + 4) = 0;
        uStack_1a0 = (undefined1 (*) [16])0x5;
        pauStack_190 = (undefined1 (*) [16])0x5;
        uStack_248 = (undefined1 (*) [16])CONCAT44(uVar42,(undefined4)uStack_248);
        uStack_248 = (undefined1 (*) [16])CONCAT71(uStack_248._1_7_,4);
        uStack_121 = 1;
        pauStack_198 = pauVar20;
        lVar16 = FUN_140a1c930(pauStack_f8,pauVar43);
        if (lVar16 == 0) {
          uStack_121 = 1;
          FUN_1428d9310(&UNK_14303c8f0);
          goto LAB_1422a2ea6;
        }
        uStack_121 = 1;
        puVar29 = (undefined4 *)FUN_1408393b0(lVar16 + 0x30,&UNK_14303c908);
        uVar47 = *puVar29;
        uStack_121 = 1;
        uVar46 = func_0x000140b06780();
        uStack_15b = 0;
        uStack_15a = 0;
        uStack_159 = 0;
        uStack_158 = 0;
        uStack_178 = (undefined1 (*) [16])CONCAT44(uVar46,uVar47);
        uStack_157 = 0;
        uStack_15c = 0;
        uStack_168._0_4_ = (uint)uStack_168 & 0xffffff00;
        uStack_170 = (undefined1 (*) [16])0x0;
        uStack_121 = 1;
        FUN_140911dd0(auStack_304);
        uStack_121 = 0;
        color_create(&pauStack_220,fVar49,fVar48,fVar50 + radius,0.38499996,fVar52,spacing,
                     appearance_divisor,&uStack_1a0,&uStack_248,&uStack_178,auStack_304,p14);
        pauVar20 = pauStack_1b8;
        if (pauStack_1b8 == pauStack_1c8) {
          FUN_140562c30(&pauStack_1c8);
        }
        auVar7._8_8_ = pauStack_218;
        auVar7._0_8_ = pauStack_220;
        *(undefined1 (**) [16])pauStack_1c0[(longlong)pauVar20 * 2 + 1] = uStack_210;
        *(ulonglong *)(pauStack_1c0[(longlong)pauVar20 * 2 + 1] + 8) =
             CONCAT44(uStack_204,uStack_208);
        pauStack_1c0[(longlong)pauVar20 * 2] = auVar7;
        pauStack_1b8 = (undefined1 (*) [16])(*pauVar20 + 1);
        cVar14 = (*pauVar24)[0];
      }
    }
    if (cVar14 == '\f') {
      uVar47 = *(undefined4 *)(*pauVar24 + 4);
      lVar16 = FUN_140a1c930();
      if ((lVar16 != 0) && (lVar34 = func_0x000140839490(), lVar34 != 0)) {
        puStack_1a8 = (undefined8 *)CONCAT44(puStack_1a8._4_4_,*(undefined4 *)(lVar16 + 0x28));
        uVar46 = *(undefined4 *)(lVar34 + 0xc);
        pauStack_138 = (undefined1 (*) [16])
                       CONCAT71(pauStack_138._1_7_,*(undefined1 *)(lVar34 + 0x10));
        FUN_1408fe8c0(&uStack_178,uStack_2a0,uVar47);
        lVar16 = CONCAT44(uStack_168._4_4_,(uint)uStack_168);
        pauStack_120 = (undefined1 (*) [16])CONCAT44(pauStack_120._4_4_,uVar47);
        puStack_1b0 = (undefined8 *)CONCAT44(puStack_1b0._4_4_,uVar46);
        if (lVar16 == 0) {
LAB_14229fc06:
          uStack_110 = (undefined1 (*) [16])((ulonglong)uStack_110._4_4_ << 0x20);
        }
        else {
          if ((undefined1 (*) [16])puStack_298[3] == (undefined1 (*) [16])0x0) {
            lVar16 = lVar16 << 2;
            do {
              lVar16 = lVar16 + -4;
            } while (lVar16 != 0);
            goto LAB_14229fc06;
          }
          pauVar24 = (undefined1 (*) [16])*puStack_298;
          auVar1 = *pauVar24;
          uStack_110 = (undefined1 (*) [16])((ulonglong)uStack_110._4_4_ << 0x20);
          pauVar20 = uStack_170;
          do {
            puVar38 = *pauVar20;
            pauVar20 = (undefined1 (*) [16])(*pauVar20 + 4);
            pauVar18 = pauVar24;
            pauVar21 = pauVar24 + 1;
            unaff_R13 = (undefined1 (*) [16])puStack_298[3];
            uVar42 = ~(uint)(ushort)((ushort)(SUB161(auVar1 >> 7,0) & 1) |
                                     (ushort)(SUB161(auVar1 >> 0xf,0) & 1) << 1 |
                                     (ushort)(SUB161(auVar1 >> 0x17,0) & 1) << 2 |
                                     (ushort)(SUB161(auVar1 >> 0x1f,0) & 1) << 3 |
                                     (ushort)(SUB161(auVar1 >> 0x27,0) & 1) << 4 |
                                     (ushort)(SUB161(auVar1 >> 0x2f,0) & 1) << 5 |
                                     (ushort)(SUB161(auVar1 >> 0x37,0) & 1) << 6 |
                                     (ushort)(SUB161(auVar1 >> 0x3f,0) & 1) << 7 |
                                     (ushort)(SUB161(auVar1 >> 0x47,0) & 1) << 8 |
                                     (ushort)(SUB161(auVar1 >> 0x4f,0) & 1) << 9 |
                                     (ushort)(SUB161(auVar1 >> 0x57,0) & 1) << 10 |
                                     (ushort)(SUB161(auVar1 >> 0x5f,0) & 1) << 0xb |
                                     (ushort)(SUB161(auVar1 >> 0x67,0) & 1) << 0xc |
                                     (ushort)(SUB161(auVar1 >> 0x6f,0) & 1) << 0xd |
                                     (ushort)(SUB161(auVar1 >> 0x77,0) & 1) << 0xe |
                                    (ushort)(byte)(auVar1[0xf] >> 7) << 0xf);
            do {
              if ((short)uVar42 == 0) {
                do {
                  auVar2 = *pauVar21;
                  uVar6 = (ushort)(SUB161(auVar2 >> 7,0) & 1) |
                          (ushort)(SUB161(auVar2 >> 0xf,0) & 1) << 1 |
                          (ushort)(SUB161(auVar2 >> 0x17,0) & 1) << 2 |
                          (ushort)(SUB161(auVar2 >> 0x1f,0) & 1) << 3 |
                          (ushort)(SUB161(auVar2 >> 0x27,0) & 1) << 4 |
                          (ushort)(SUB161(auVar2 >> 0x2f,0) & 1) << 5 |
                          (ushort)(SUB161(auVar2 >> 0x37,0) & 1) << 6 |
                          (ushort)(SUB161(auVar2 >> 0x3f,0) & 1) << 7 |
                          (ushort)(SUB161(auVar2 >> 0x47,0) & 1) << 8 |
                          (ushort)(SUB161(auVar2 >> 0x4f,0) & 1) << 9 |
                          (ushort)(SUB161(auVar2 >> 0x57,0) & 1) << 10 |
                          (ushort)(SUB161(auVar2 >> 0x5f,0) & 1) << 0xb |
                          (ushort)(SUB161(auVar2 >> 0x67,0) & 1) << 0xc |
                          (ushort)(SUB161(auVar2 >> 0x6f,0) & 1) << 0xd |
                          (ushort)(SUB161(auVar2 >> 0x77,0) & 1) << 0xe |
                          (ushort)(byte)(auVar2[0xf] >> 7) << 0xf;
                  pauVar18 = pauVar18 + -0x110;
                  pauVar21 = pauVar21 + 1;
                } while (uVar6 == 0xffff);
                uVar15 = ~(uint)uVar6;
                uVar42 = uVar15 & -(uint)uVar6 - 2;
              }
              else {
                uVar15 = uVar42;
                uVar42 = uVar42 - 1 & uVar42;
              }
              pauVar43 = (undefined1 (*) [16])(ulonglong)uVar42;
              uVar41 = 0;
              for (; (uVar15 & 1) == 0; uVar15 = uVar15 >> 1 | 0x80000000) {
                uVar41 = uVar41 + 1;
              }
              uVar30 = (ulonglong)uVar41;
              pauVar19 = (undefined1 (*) [16])(uVar30 * -0x110);
              if (((*(int *)(pauVar18 + -0x11)[uVar30 * -0x11] == *(int *)puVar38) ||
                  (*(int *)((pauVar18 + uVar30 * -0x11)[-0x11] + 4) == *(int *)puVar38)) &&
                 ((pauVar18 + uVar30 * -0x11)[-8][0] != '\0')) {
                uStack_110 = (undefined1 (*) [16])
                             CONCAT44(uStack_110._4_4_,
                                      (int)CONCAT71((int7)((ulonglong)pauVar18 >> 8),1));
                break;
              }
              unaff_R13 = (undefined1 (*) [16])(unaff_R13[-1] + 0xf);
            } while (unaff_R13 != (undefined1 (*) [16])0x0);
          } while (pauVar20 != (undefined1 (*) [16])(*uStack_170 + lVar16 * 4));
        }
        if (uStack_178 != (undefined1 (*) [16])0x0) {
          func_0x000140613c20(uStack_170,(longlong)uStack_178 << 2,4);
        }
        uVar47 = (uint)pauStack_120;
        if (((ulonglong)uStack_110 & 1) != 0) {
          fVar49 = *(float *)pauStack_108[1];
          fVar48 = *(float *)(pauStack_108[1] + 4);
          fVar50 = fVar51 * _DAT_142a89210;
          fVar52 = fVar51 * _DAT_142a16cb8;
          pauVar24 = (undefined1 (*) [16])func_0x000140613c10(0xf0,8);
          if (pauVar24 == (undefined1 (*) [16])0x0) {
            FUN_1428d9000(8,0xf0);
            goto LAB_1422a2ea6;
          }
          pauVar19 = (undefined1 (*) [16])func_0x000140613c10(0x1b,1);
          uVar31 = _UNK_14303c1fb;
          if (pauVar19 == (undefined1 (*) [16])0x0) {
            pauStack_108 = pauVar24;
            FUN_1428d8fe3(1,0x1b,&UNK_143136c68);
            goto LAB_1422a2ea6;
          }
          *(ulonglong *)(*pauVar19 + 0xb) = CONCAT35(_UNK_14303c1f8,_DAT_14303c1f3);
          *(undefined8 *)(pauVar19[1] + 3) = uVar31;
          uVar31 = CONCAT53(_DAT_14303c1f3,_UNK_14303c1f0);
          *(undefined8 *)*pauVar19 = _DAT_14303c1e8;
          *(undefined8 *)(*pauVar19 + 8) = uVar31;
          pauVar20 = (undefined1 (*) [16])func_0x000140613c10(0x17,1);
          uVar31 = _UNK_14303c664;
          if (pauVar20 == (undefined1 (*) [16])0x0) {
            uStack_110 = pauVar19;
            pauStack_108 = pauVar24;
            FUN_1428d8fe3(1,0x17,&UNK_143136c68);
            goto LAB_1422a2ea6;
          }
          *(undefined8 *)*pauVar20 = _DAT_14303c65c;
          *(undefined8 *)(*pauVar20 + 8) = uVar31;
          *(undefined8 *)(*pauVar20 + 0xf) = 0x65756c622f73776f;
          pauVar43 = (undefined1 (*) [16])func_0x000140613c10(0x19,1);
          uVar31 = _UNK_14303c424;
          if (pauVar43 == (undefined1 (*) [16])0x0) {
            pauStack_120 = pauVar20;
            uStack_110 = pauVar19;
            pauStack_108 = pauVar24;
            FUN_1428d8fe3(1,0x19,&UNK_143136c68);
            goto LAB_1422a2ea6;
          }
          *(ulonglong *)(*pauVar43 + 9) = CONCAT17(UNK_14303c423,_DAT_14303c41c);
          *(undefined8 *)(pauVar43[1] + 1) = uVar31;
          uVar31 = CONCAT71(_DAT_14303c41c,UNK_14303c41b);
          *(undefined8 *)*pauVar43 = _DAT_14303c413;
          *(undefined8 *)(*pauVar43 + 8) = uVar31;
          unaff_R13 = (undefined1 (*) [16])func_0x000140613c10(0x15,1);
          uVar31 = _UNK_14303c2a9;
          if (unaff_R13 == (undefined1 (*) [16])0x0) {
            pauStack_120 = pauVar20;
            uStack_110 = pauVar19;
            pauStack_108 = pauVar24;
            pauStack_f8 = pauVar43;
            FUN_1428d8fe3(1,0x15,&UNK_143136c68);
            goto LAB_1422a2ea6;
          }
          *(undefined8 *)*unaff_R13 = _DAT_14303c2a1;
          *(undefined8 *)(*unaff_R13 + 8) = uVar31;
          *(undefined8 *)(*unaff_R13 + 0xd) = 0x657268636f5f666f;
          pauVar18 = (undefined1 (*) [16])func_0x000140613c10(0x1b,1);
          uVar31 = _UNK_14303c339;
          if (pauVar18 == (undefined1 (*) [16])0x0) {
            pauStack_120 = pauVar20;
            uStack_110 = pauVar19;
            pauStack_108 = pauVar24;
            pauStack_100 = unaff_R13;
            pauStack_f8 = pauVar43;
            FUN_1428d8fe3(1,0x1b,&UNK_143136c68);
            goto LAB_1422a2ea6;
          }
          *(ulonglong *)(*pauVar18 + 0xb) = CONCAT35(_UNK_14303c336,_DAT_14303c331);
          *(undefined8 *)(pauVar18[1] + 3) = uVar31;
          uVar31 = CONCAT53(_DAT_14303c331,_UNK_14303c32e);
          *(undefined8 *)*pauVar18 = _DAT_14303c326;
          *(undefined8 *)(*pauVar18 + 8) = uVar31;
          puVar25 = (undefined8 *)func_0x000140613c10(0x14,1);
          uVar31 = _UNK_14303c377;
          if (puVar25 == (undefined8 *)0x0) {
            pauStack_120 = pauVar20;
            pauStack_118 = pauVar18;
            uStack_110 = pauVar19;
            pauStack_108 = pauVar24;
            pauStack_100 = unaff_R13;
            pauStack_f8 = pauVar43;
            FUN_1428d8fe3(1,0x14,&UNK_143136c68);
            goto LAB_1422a2ea6;
          }
          *puVar25 = _DAT_14303c36f;
          puVar25[1] = uVar31;
          *(undefined4 *)(puVar25 + 2) = 0x6b726164;
          *(undefined8 *)*pauVar24 = 0x1b;
          *(undefined1 (**) [16])(*pauVar24 + 8) = pauVar19;
          *(undefined8 *)pauVar24[1] = 0x1b;
          pauVar24[1][8] = 0xe;
          *(undefined4 *)(pauVar24[1] + 0xc) = 0;
          *(undefined4 *)(pauVar24[2] + 4) = 0;
          *(undefined8 *)(pauVar24[2] + 8) = 0x17;
          *(undefined1 (**) [16])pauVar24[3] = pauVar20;
          *(undefined8 *)(pauVar24[3] + 8) = 0x17;
          pauVar24[4][0] = 0xe;
          *(undefined4 *)(pauVar24[4] + 4) = 1;
          *(undefined4 *)(pauVar24[4] + 0xc) = 0;
          *(undefined8 *)pauVar24[5] = 0x19;
          *(undefined1 (**) [16])(pauVar24[5] + 8) = pauVar43;
          *(undefined8 *)pauVar24[6] = 0x19;
          pauVar24[6][8] = 0xe;
          *(undefined4 *)(pauVar24[6] + 0xc) = 2;
          *(undefined4 *)(pauVar24[7] + 4) = 0;
          *(undefined8 *)(pauVar24[7] + 8) = 0x15;
          *(undefined1 (**) [16])pauVar24[8] = unaff_R13;
          *(undefined8 *)(pauVar24[8] + 8) = 0x15;
          pauVar24[9][0] = 0xe;
          *(undefined4 *)(pauVar24[9] + 4) = 3;
          *(undefined4 *)(pauVar24[9] + 0xc) = 0;
          *(undefined8 *)pauVar24[10] = 0x1b;
          *(undefined1 (**) [16])(pauVar24[10] + 8) = pauVar18;
          *(undefined8 *)pauVar24[0xb] = 0x1b;
          pauVar24[0xb][8] = 0xe;
          *(undefined4 *)(pauVar24[0xb] + 0xc) = 4;
          *(undefined4 *)(pauVar24[0xc] + 4) = 0;
          *(undefined8 *)(pauVar24[0xc] + 8) = 0x14;
          *(undefined8 **)pauVar24[0xd] = puVar25;
          *(undefined8 *)(pauVar24[0xd] + 8) = 0x14;
          pauVar24[0xe][0] = 0xe;
          *(undefined4 *)(pauVar24[0xe] + 4) = 5;
          *(undefined4 *)(pauVar24[0xe] + 0xc) = 0;
          uStack_1a0 = (undefined1 (*) [16])0x6;
          pauStack_190 = (undefined1 (*) [16])0x6;
          uStack_248 = (undefined1 (*) [16])CONCAT44(uVar47,(undefined4)uStack_248);
          uStack_248 = (undefined1 (*) [16])CONCAT71(uStack_248._1_7_,0xb);
          uStack_149 = 1;
          pauStack_198 = pauVar24;
          uVar47 = func_0x000140b06780();
          uStack_15b = 0;
          uStack_15a = 0;
          uStack_159 = 0;
          uStack_158 = 0;
          uStack_178 = (undefined1 (*) [16])CONCAT44(uVar47,(int)puStack_1a8);
          uStack_157 = 0;
          uStack_168._0_4_ = (uint)uStack_168 & 0xffffff00;
          uStack_15c = (char)pauStack_138;
          uStack_170 = (undefined1 (*) [16])((longlong)puStack_1b0 << 0x20);
          uStack_149 = 1;
          FUN_140911dd0(&uStack_260);
          uStack_149 = 0;
          color_create(&pauStack_220,fVar49,fVar48,fVar50 + radius,0.38499996,fVar52,spacing,
                       appearance_divisor,&uStack_1a0,&uStack_248,&uStack_178,&uStack_260,p14);
          pauVar24 = pauStack_1b8;
          if (pauStack_1b8 == pauStack_1c8) {
            FUN_140562c30(&pauStack_1c8);
          }
          auVar8._8_8_ = pauStack_218;
          auVar8._0_8_ = pauStack_220;
          *(undefined1 (**) [16])pauStack_1c0[(longlong)pauVar24 * 2 + 1] = uStack_210;
          *(ulonglong *)(pauStack_1c0[(longlong)pauVar24 * 2 + 1] + 8) =
               CONCAT44(uStack_204,uStack_208);
          pauStack_1c0[(longlong)pauVar24 * 2] = auVar8;
          pauStack_1b8 = (undefined1 (*) [16])(*pauVar24 + 1);
        }
      }
      cVar14 = (*pauStack_108)[0];
      pauVar24 = pauStack_108;
    }
    if (cVar14 == '\v') {
      uVar47 = *(undefined4 *)(*pauVar24 + 4);
      lVar16 = FUN_140a1c930();
      if ((lVar16 != 0) && (lVar16 = func_0x000140839490(), lVar16 != 0)) {
        uVar46 = *(undefined4 *)(lVar16 + 8);
        uVar4 = *(undefined4 *)(lVar16 + 0xc);
        uVar35 = *(undefined1 *)(lVar16 + 0x10);
        FUN_1408fe8c0(&uStack_178,uStack_2a0,uVar47);
        lVar16 = CONCAT44(uStack_168._4_4_,(uint)uStack_168);
        pauStack_120 = (undefined1 (*) [16])CONCAT44(pauStack_120._4_4_,uVar47);
        puStack_1a8 = (undefined8 *)CONCAT44(puStack_1a8._4_4_,uVar4);
        puStack_1b0 = (undefined8 *)CONCAT44(puStack_1b0._4_4_,uVar46);
        pauStack_138 = (undefined1 (*) [16])CONCAT71(pauStack_138._1_7_,uVar35);
        if (lVar16 == 0) {
LAB_1422a01f6:
          uStack_110 = (undefined1 (*) [16])((ulonglong)uStack_110._4_4_ << 0x20);
        }
        else {
          if (puStack_298[3] == 0) {
            lVar16 = lVar16 << 2;
            do {
              lVar16 = lVar16 + -4;
            } while (lVar16 != 0);
            goto LAB_1422a01f6;
          }
          pauVar19 = (undefined1 (*) [16])*puStack_298;
          auVar1 = *pauVar19;
          uStack_110 = (undefined1 (*) [16])((ulonglong)uStack_110._4_4_ << 0x20);
          pauVar24 = uStack_170;
          do {
            puVar38 = *pauVar24;
            pauVar24 = (undefined1 (*) [16])(*pauVar24 + 4);
            unaff_R13 = pauVar19;
            lVar34 = puStack_298[3];
            pauVar20 = pauVar19 + 1;
            uVar42 = ~(uint)(ushort)((ushort)(SUB161(auVar1 >> 7,0) & 1) |
                                     (ushort)(SUB161(auVar1 >> 0xf,0) & 1) << 1 |
                                     (ushort)(SUB161(auVar1 >> 0x17,0) & 1) << 2 |
                                     (ushort)(SUB161(auVar1 >> 0x1f,0) & 1) << 3 |
                                     (ushort)(SUB161(auVar1 >> 0x27,0) & 1) << 4 |
                                     (ushort)(SUB161(auVar1 >> 0x2f,0) & 1) << 5 |
                                     (ushort)(SUB161(auVar1 >> 0x37,0) & 1) << 6 |
                                     (ushort)(SUB161(auVar1 >> 0x3f,0) & 1) << 7 |
                                     (ushort)(SUB161(auVar1 >> 0x47,0) & 1) << 8 |
                                     (ushort)(SUB161(auVar1 >> 0x4f,0) & 1) << 9 |
                                     (ushort)(SUB161(auVar1 >> 0x57,0) & 1) << 10 |
                                     (ushort)(SUB161(auVar1 >> 0x5f,0) & 1) << 0xb |
                                     (ushort)(SUB161(auVar1 >> 0x67,0) & 1) << 0xc |
                                     (ushort)(SUB161(auVar1 >> 0x6f,0) & 1) << 0xd |
                                     (ushort)(SUB161(auVar1 >> 0x77,0) & 1) << 0xe |
                                    (ushort)(byte)(auVar1[0xf] >> 7) << 0xf);
            do {
              if ((short)uVar42 == 0) {
                do {
                  auVar2 = *pauVar20;
                  uVar6 = (ushort)(SUB161(auVar2 >> 7,0) & 1) |
                          (ushort)(SUB161(auVar2 >> 0xf,0) & 1) << 1 |
                          (ushort)(SUB161(auVar2 >> 0x17,0) & 1) << 2 |
                          (ushort)(SUB161(auVar2 >> 0x1f,0) & 1) << 3 |
                          (ushort)(SUB161(auVar2 >> 0x27,0) & 1) << 4 |
                          (ushort)(SUB161(auVar2 >> 0x2f,0) & 1) << 5 |
                          (ushort)(SUB161(auVar2 >> 0x37,0) & 1) << 6 |
                          (ushort)(SUB161(auVar2 >> 0x3f,0) & 1) << 7 |
                          (ushort)(SUB161(auVar2 >> 0x47,0) & 1) << 8 |
                          (ushort)(SUB161(auVar2 >> 0x4f,0) & 1) << 9 |
                          (ushort)(SUB161(auVar2 >> 0x57,0) & 1) << 10 |
                          (ushort)(SUB161(auVar2 >> 0x5f,0) & 1) << 0xb |
                          (ushort)(SUB161(auVar2 >> 0x67,0) & 1) << 0xc |
                          (ushort)(SUB161(auVar2 >> 0x6f,0) & 1) << 0xd |
                          (ushort)(SUB161(auVar2 >> 0x77,0) & 1) << 0xe |
                          (ushort)(byte)(auVar2[0xf] >> 7) << 0xf;
                  unaff_R13 = unaff_R13 + -0x110;
                  pauVar20 = pauVar20 + 1;
                } while (uVar6 == 0xffff);
                uVar15 = ~(uint)uVar6;
                uVar42 = uVar15 & -(uint)uVar6 - 2;
              }
              else {
                uVar15 = uVar42;
                uVar42 = uVar42 - 1 & uVar42;
              }
              pauVar43 = (undefined1 (*) [16])(ulonglong)uVar42;
              uVar41 = 0;
              for (; (uVar15 & 1) == 0; uVar15 = uVar15 >> 1 | 0x80000000) {
                uVar41 = uVar41 + 1;
              }
              uVar30 = (ulonglong)uVar41;
              if (((*(int *)unaff_R13[uVar30 * -0x11 + -0x11] == *(int *)puVar38) ||
                  (*(int *)(unaff_R13[uVar30 * -0x11 + -0x11] + 4) == *(int *)puVar38)) &&
                 (unaff_R13[uVar30 * -0x11 + -8][0] != '\0')) {
                uStack_110 = (undefined1 (*) [16])
                             CONCAT44(uStack_110._4_4_,(int)CONCAT71((int7)(uVar30 * -0x110 >> 8),1)
                                     );
                break;
              }
              lVar34 = lVar34 + -1;
            } while (lVar34 != 0);
          } while (pauVar24 != (undefined1 (*) [16])(*uStack_170 + lVar16 * 4));
        }
        if (uStack_178 != (undefined1 (*) [16])0x0) {
          func_0x000140613c20(uStack_170,(longlong)uStack_178 << 2,4);
        }
        cVar14 = (char)pauStack_138;
        pauVar19 = (undefined1 (*) [16])((ulonglong)pauStack_120 & 0xffffffff);
        if (((char)pauStack_138 != '\x03' & ((byte)uStack_110 ^ 1)) == 0) {
          fVar49 = *(float *)pauStack_108[1];
          fVar48 = *(float *)(pauStack_108[1] + 4);
          fVar50 = fVar51 * _DAT_142a89210;
          fVar51 = fVar51 * _DAT_142a16cb8;
          uStack_1a0 = (undefined1 (*) [16])0x0;
          pauStack_198 = (undefined1 (*) [16])0x8;
          pauStack_190 = (undefined1 (*) [16])0x0;
          if (((ulonglong)uStack_110 & 1) == 0) {
            if ((char)pauStack_138 == '\x03') {
LAB_1422a092b:
              FUN_1422acce0(&pauStack_220);
              pauVar18 = uStack_210;
              pauVar20 = pauStack_218;
              pauVar24 = pauStack_220;
              lVar16 = (longlong)uStack_210 * 0x28;
              puVar38 = (undefined *)((longlong)pauStack_218 + lVar16);
              uStack_178 = pauStack_218;
              uStack_168._0_4_ = (uint)pauStack_220;
              uStack_168._4_4_ = (undefined4)((ulonglong)pauStack_220 >> 0x20);
              uStack_170 = pauStack_218;
              uStack_160 = SUB84(puVar38,0);
              uStack_15c = (undefined1)((ulonglong)puVar38 >> 0x20);
              uStack_15b = (undefined1)((ulonglong)puVar38 >> 0x28);
              uStack_15a = (undefined1)((ulonglong)puVar38 >> 0x30);
              uStack_159 = (undefined1)((ulonglong)puVar38 >> 0x38);
              if ((undefined1 (*) [16])((longlong)uStack_1a0 - (longlong)pauStack_190) < uStack_210)
              {
                FUN_1429107f0(&uStack_1a0,pauStack_190,uStack_210,8,0x28);
              }
              pauVar21 = pauStack_190;
              func_0x000142923c00((undefined *)
                                  ((longlong)pauStack_190 * 0x28 + (longlong)pauStack_198),pauVar20,
                                  lVar16);
              unaff_R13 = (undefined1 (*) [16])(*pauVar21 + (longlong)*pauVar18);
              pauStack_190 = unaff_R13;
              if (pauVar24 != (undefined1 (*) [16])0x0) {
                func_0x000140613c20(pauVar20,(longlong)pauVar24 * 0x28,8);
              }
            }
          }
          else {
            pauVar24 = (undefined1 (*) [16])func_0x000140613c10(0xf0,8);
            if (pauVar24 == (undefined1 (*) [16])0x0) {
              FUN_1428d9000(8,0xf0);
              goto LAB_1422a2ea6;
            }
            pauVar19 = (undefined1 (*) [16])func_0x000140613c10(0x1b,1);
            uVar31 = _UNK_14303c1fb;
            if (pauVar19 == (undefined1 (*) [16])0x0) {
              pauStack_108 = pauVar24;
              FUN_1428d8fe3(1,0x1b,&UNK_143136c68);
              goto LAB_1422a2ea6;
            }
            *(ulonglong *)(*pauVar19 + 0xb) = CONCAT35(_UNK_14303c1f8,_DAT_14303c1f3);
            *(undefined8 *)(pauVar19[1] + 3) = uVar31;
            uVar31 = CONCAT53(_DAT_14303c1f3,_UNK_14303c1f0);
            *(undefined8 *)*pauVar19 = _DAT_14303c1e8;
            *(undefined8 *)(*pauVar19 + 8) = uVar31;
            pauVar20 = (undefined1 (*) [16])func_0x000140613c10(0x17,1);
            uVar31 = _UNK_14303c664;
            if (pauVar20 == (undefined1 (*) [16])0x0) {
              uStack_110 = pauVar19;
              pauStack_108 = pauVar24;
              FUN_1428d8fe3(1,0x17,&UNK_143136c68);
              goto LAB_1422a2ea6;
            }
            *(undefined8 *)*pauVar20 = _DAT_14303c65c;
            *(undefined8 *)(*pauVar20 + 8) = uVar31;
            *(undefined8 *)(*pauVar20 + 0xf) = 0x65756c622f73776f;
            pauVar43 = (undefined1 (*) [16])func_0x000140613c10(0x19,1);
            uVar31 = _UNK_14303c424;
            if (pauVar43 == (undefined1 (*) [16])0x0) {
              pauStack_120 = pauVar20;
              uStack_110 = pauVar19;
              pauStack_108 = pauVar24;
              FUN_1428d8fe3(1,0x19,&UNK_143136c68);
              goto LAB_1422a2ea6;
            }
            *(ulonglong *)(*pauVar43 + 9) = CONCAT17(UNK_14303c423,_DAT_14303c41c);
            *(undefined8 *)(pauVar43[1] + 1) = uVar31;
            uVar31 = CONCAT71(_DAT_14303c41c,UNK_14303c41b);
            *(undefined8 *)*pauVar43 = _DAT_14303c413;
            *(undefined8 *)(*pauVar43 + 8) = uVar31;
            unaff_R13 = (undefined1 (*) [16])func_0x000140613c10(0x15,1);
            uVar31 = _UNK_14303c2a9;
            if (unaff_R13 == (undefined1 (*) [16])0x0) {
              pauStack_120 = pauVar20;
              uStack_110 = pauVar19;
              pauStack_108 = pauVar24;
              pauStack_f8 = pauVar43;
              FUN_1428d8fe3(1,0x15,&UNK_143136c68);
              goto LAB_1422a2ea6;
            }
            *(undefined8 *)*unaff_R13 = _DAT_14303c2a1;
            *(undefined8 *)(*unaff_R13 + 8) = uVar31;
            *(undefined8 *)(*unaff_R13 + 0xd) = 0x657268636f5f666f;
            pauVar18 = (undefined1 (*) [16])func_0x000140613c10(0x1b,1);
            uVar31 = _UNK_14303c339;
            if (pauVar18 == (undefined1 (*) [16])0x0) {
              pauStack_120 = pauVar20;
              uStack_110 = pauVar19;
              pauStack_108 = pauVar24;
              pauStack_100 = unaff_R13;
              pauStack_f8 = pauVar43;
              FUN_1428d8fe3(1,0x1b,&UNK_143136c68);
              goto LAB_1422a2ea6;
            }
            *(ulonglong *)(*pauVar18 + 0xb) = CONCAT35(_UNK_14303c336,_DAT_14303c331);
            *(undefined8 *)(pauVar18[1] + 3) = uVar31;
            uVar31 = CONCAT53(_DAT_14303c331,_UNK_14303c32e);
            *(undefined8 *)*pauVar18 = _DAT_14303c326;
            *(undefined8 *)(*pauVar18 + 8) = uVar31;
            puVar25 = (undefined8 *)func_0x000140613c10(0x14,1);
            uVar31 = _UNK_14303c377;
            if (puVar25 == (undefined8 *)0x0) {
              pauStack_120 = pauVar20;
              pauStack_118 = pauVar18;
              uStack_110 = pauVar19;
              pauStack_108 = pauVar24;
              pauStack_100 = unaff_R13;
              pauStack_f8 = pauVar43;
              FUN_1428d8fe3(1,0x14,&UNK_143136c68);
              goto LAB_1422a2ea6;
            }
            *puVar25 = _DAT_14303c36f;
            puVar25[1] = uVar31;
            *(undefined4 *)(puVar25 + 2) = 0x6b726164;
            *(undefined8 *)*pauVar24 = 0x1b;
            *(undefined1 (**) [16])(*pauVar24 + 8) = pauVar19;
            *(undefined8 *)pauVar24[1] = 0x1b;
            pauVar24[1][8] = 0xe;
            *(undefined4 *)(pauVar24[1] + 0xc) = 0;
            *(undefined4 *)(pauVar24[2] + 4) = 0;
            *(undefined8 *)(pauVar24[2] + 8) = 0x17;
            *(undefined1 (**) [16])pauVar24[3] = pauVar20;
            *(undefined8 *)(pauVar24[3] + 8) = 0x17;
            pauVar24[4][0] = 0xe;
            *(undefined4 *)(pauVar24[4] + 4) = 1;
            *(undefined4 *)(pauVar24[4] + 0xc) = 0;
            *(undefined8 *)pauVar24[5] = 0x19;
            *(undefined1 (**) [16])(pauVar24[5] + 8) = pauVar43;
            *(undefined8 *)pauVar24[6] = 0x19;
            pauVar24[6][8] = 0xe;
            *(undefined4 *)(pauVar24[6] + 0xc) = 2;
            *(undefined4 *)(pauVar24[7] + 4) = 0;
            *(undefined8 *)(pauVar24[7] + 8) = 0x15;
            *(undefined1 (**) [16])pauVar24[8] = unaff_R13;
            *(undefined8 *)(pauVar24[8] + 8) = 0x15;
            pauVar24[9][0] = 0xe;
            *(undefined4 *)(pauVar24[9] + 4) = 3;
            *(undefined4 *)(pauVar24[9] + 0xc) = 0;
            *(undefined8 *)pauVar24[10] = 0x1b;
            *(undefined1 (**) [16])(pauVar24[10] + 8) = pauVar18;
            *(undefined8 *)pauVar24[0xb] = 0x1b;
            pauVar24[0xb][8] = 0xe;
            *(undefined4 *)(pauVar24[0xb] + 0xc) = 4;
            *(undefined4 *)(pauVar24[0xc] + 4) = 0;
            *(undefined8 *)(pauVar24[0xc] + 8) = 0x14;
            *(undefined8 **)pauVar24[0xd] = puVar25;
            *(undefined8 *)(pauVar24[0xd] + 8) = 0x14;
            pauVar24[0xe][0] = 0xe;
            *(undefined4 *)(pauVar24[0xe] + 4) = 5;
            *(undefined4 *)(pauVar24[0xe] + 0xc) = 0;
            pauVar19 = pauVar24 + 0xf;
            uStack_168._0_4_ = 6;
            uStack_168._4_4_ = 0;
            uStack_160 = SUB84(pauVar19,0);
            uStack_15c = (undefined1)((ulonglong)pauVar19 >> 0x20);
            uStack_15b = (undefined1)((ulonglong)pauVar19 >> 0x28);
            uStack_15a = (undefined1)((ulonglong)pauVar19 >> 0x30);
            uStack_159 = (undefined1)((ulonglong)pauVar19 >> 0x38);
            uStack_178 = pauVar24;
            uStack_170 = pauVar24;
            if ((ulonglong)((longlong)uStack_1a0 - (longlong)pauStack_190) < 6) {
              FUN_1429107f0(&uStack_1a0,pauStack_190,6,8,0x28);
            }
            else {
            }
            pauVar20 = pauStack_190;
            pauVar19 = (undefined1 (*) [16])(ulonglong)(uint)pauStack_120;
            func_0x000142923c00((undefined *)
                                ((longlong)pauStack_190 * 0x28 + (longlong)pauStack_198),pauVar24,
                                0xf0);
            pauStack_190 = (undefined1 (*) [16])(*pauVar20 + 6);
            func_0x000140613c20(pauVar24,0xf0,8);
            if (cVar14 == '\x03') {
              pauVar20 = (undefined1 (*) [16])func_0x000140613c10(5,1);
              pauVar24 = pauStack_190;
              if (pauVar20 != (undefined1 (*) [16])0x0) {
                (*pauVar20)[4] = DAT_143136d57;
                *(undefined4 *)*pauVar20 = _DAT_143136d53;
                pauStack_108 = pauVar20;
                if (pauStack_190 == uStack_1a0) {
                  FUN_140562ab0(&uStack_1a0,&UNK_14303c950);
                }
                *(undefined8 *)((longlong)pauStack_198 + (longlong)pauVar24 * 0x28) = 5;
                *(undefined1 (**) [16])((longlong)pauStack_198 + (longlong)pauVar24 * 0x28 + 8) =
                     pauStack_108;
                *(undefined8 *)((longlong)pauStack_198 + (longlong)pauVar24 * 0x28 + 0x10) = 5;
                *(undefined1 *)((longlong)pauStack_198 + (longlong)pauVar24 * 0x28 + 0x18) = 5;
                *(undefined4 *)((longlong)pauStack_198 + (longlong)pauVar24 * 0x28 + 0x24) = 0;
                pauStack_190 = (undefined1 (*) [16])(*pauVar24 + 1);
                goto LAB_1422a092b;
              }
              FUN_1428d8fe3(1,5,&UNK_143136c68);
              goto LAB_1422a2ea6;
            }
          }
          pauStack_238 = pauStack_190;
          uStack_248 = uStack_1a0;
          pauStack_240 = pauStack_198;
          uStack_1a0 = (undefined1 (*) [16])CONCAT44((int)pauVar19,(undefined4)uStack_1a0);
          uStack_1a0 = (undefined1 (*) [16])CONCAT71(uStack_1a0._1_7_,0xc);
          uStack_14a = 1;
          uVar47 = func_0x000140b06780();
          uStack_15b = 0;
          uStack_15a = 0;
          uStack_159 = 0;
          uStack_158 = 0;
          uStack_178 = (undefined1 (*) [16])CONCAT44(uVar47,(int)puStack_1b0);
          uStack_157 = 0;
          uStack_168._0_4_ = (uint)uStack_168 & 0xffffff00;
          uStack_15c = 0;
          uStack_170 = (undefined1 (*) [16])((longlong)puStack_1a8 << 0x20);
          uStack_14a = 1;
          FUN_140911dd0(&uStack_260);
          uStack_14a = 0;
          color_create(&pauStack_220,fVar49,fVar48,fVar50 + radius,0.38499996,fVar51,spacing,
                       appearance_divisor,&uStack_248,&uStack_1a0,&uStack_178,&uStack_260,p14);
          pauVar24 = pauStack_1b8;
          if (pauStack_1b8 == pauStack_1c8) {
            FUN_140562c30();
          }
          auVar9._8_8_ = pauStack_218;
          auVar9._0_8_ = pauStack_220;
          *(undefined1 (**) [16])pauStack_1c0[(longlong)pauVar24 * 2 + 1] = uStack_210;
          *(ulonglong *)(pauStack_1c0[(longlong)pauVar24 * 2 + 1] + 8) =
               CONCAT44(uStack_204,uStack_208);
          pauStack_1c0[(longlong)pauVar24 * 2] = auVar9;
          pauStack_1b8 = (undefined1 (*) [16])(*pauVar24 + 1);
        }
      }
    }
    pauStack_108 = pauStack_1c0 + (longlong)pauStack_1b8 * 2;
    uStack_110 = (undefined1 (*) [16])(pauStack_1b8[-1] + 0xf);
    pauVar24 = (undefined1 (*) [16])0x0;
    pauVar20 = pauStack_1c0;
    pauStack_120 = pauStack_1b8;
    if (pauStack_1c0 != pauStack_108) {
      do {
        pauVar43 = (undefined1 (*) [16])(*pauVar24 + 1);
        uVar31 = *(undefined8 *)(pauVar20[1] + 8);
        if (((ulonglong)pauVar43 | (ulonglong)pauStack_120) >> 0x20 == 0) {
          pauVar19 = (undefined1 (*) [16])
                     (((ulonglong)pauVar43 & 0xffffffff) % ((ulonglong)pauStack_120 & 0xffffffff));
        }
        else {
          pauVar19 = (undefined1 (*) [16])((ulonglong)pauVar43 % (ulonglong)pauStack_120);
        }
        if (pauStack_1b8 <= pauVar19) {
          FUN_1428d9518(pauVar19,pauStack_1b8,&UNK_14303c980);
          goto LAB_1422a2ea6;
        }
        puVar38 = *uStack_110 + (longlong)*pauVar24;
        if (((ulonglong)puVar38 | (ulonglong)pauStack_120) >> 0x20 == 0) {
          pauVar24 = (undefined1 (*) [16])
                     (((ulonglong)puVar38 & 0xffffffff) % ((ulonglong)pauStack_120 & 0xffffffff));
        }
        else {
          pauVar24 = (undefined1 (*) [16])((ulonglong)puVar38 % (ulonglong)pauStack_120);
        }
        if (pauStack_1b8 <= pauVar24) {
          FUN_1428d9518(pauVar24,pauStack_1b8,&UNK_14303c998);
          goto LAB_1422a2ea6;
        }
        unaff_R13 = *(undefined1 (**) [16])(pauStack_1c0[(longlong)pauVar19 * 2 + 1] + 8);
        pauVar19 = *(undefined1 (**) [16])(pauStack_1c0[(longlong)pauVar24 * 2 + 1] + 8);
        plVar45 = *(longlong **)((longlong)p14 + 0x10);
        if (*(longlong **)((longlong)p14 + 8) != (longlong *)0x0) {
          plVar45 = *(longlong **)((longlong)p14 + 8);
        }
        lVar16 = plVar45[2];
        if ((ulonglong)(*plVar45 - lVar16) < 0x60) {
          FUN_1429107f0(plVar45,lVar16,0x60,1,1);
        }
        pauVar20 = pauVar20 + 2;
        lVar34 = plVar45[1];
        *(code **)(lVar34 + lVar16) = FUN_1422be250;
        *(undefined8 *)(lVar34 + 8 + lVar16) = 2;
        *(undefined8 *)(lVar34 + 0x18 + lVar16) = 2;
        *(undefined8 *)(lVar34 + 0x28 + lVar16) = 3;
        *(undefined1 (**) [16])(lVar34 + 0x30 + lVar16) = unaff_R13;
        *(undefined8 *)(lVar34 + 0x38 + lVar16) = 3;
        *(undefined1 (**) [16])(lVar34 + 0x40 + lVar16) = pauVar19;
        *(undefined1 *)(lVar34 + 0x48 + lVar16) = 0;
        *(undefined8 *)(lVar34 + 0x50 + lVar16) = uVar31;
        *(code **)(lVar34 + 0x58 + lVar16) = FUN_1422bf9d0;
        plVar45[2] = lVar16 + 0x60;
        pauVar24 = pauVar43;
      } while (pauVar20 != pauStack_108);
    }
    pauVar24 = pauStack_1c0;
    if (pauStack_1b8 != (undefined1 (*) [16])0x0) {
      uVar31 = *(undefined8 *)(pauStack_1c0[1] + 8);
      *puStack_2e0 = uStack_284;
      *(undefined8 *)(lStack_2e8 + 0x10) = 1;
      *(undefined8 *)(lStack_2e8 + 0x18) = uVar31;
      puVar25 = (undefined8 *)(*pauStack_1c0 + 8);
      pauVar18 = pauStack_1b8;
      do {
        if (puVar25[-1] != 0) {
          func_0x000140613c20(*puVar25,puVar25[-1] << 3,8);
        }
        puVar25 = puVar25 + 4;
        pauVar18 = (undefined1 (*) [16])(pauVar18[-1] + 0xf);
      } while (pauVar18 != (undefined1 (*) [16])0x0);
    }
    if (pauStack_1c8 != (undefined1 (*) [16])0x0) {
      func_0x000140613c20(pauVar24,(longlong)pauStack_1c8 << 5,8);
    }
    pauVar18 = pauStack_1e0;
    if (pauStack_1d8 != (undefined1 (*) [16])0x0) {
      puVar25 = (undefined8 *)(*pauStack_1e0 + 8);
      pauVar24 = pauStack_1d8;
      do {
        if (puVar25[-1] != 0) {
          func_0x000140613c20(*puVar25,puVar25[-1],1);
        }
        puVar25 = puVar25 + 5;
        pauVar24 = (undefined1 (*) [16])(pauVar24[-1] + 0xf);
      } while (pauVar24 != (undefined1 (*) [16])0x0);
    }
    pauVar24 = pauStack_118;
    if (pauStack_1e8 != (undefined1 (*) [16])0x0) {
      func_0x000140613c20(pauVar18,(longlong)pauStack_1e8 * 0x28,8);
      pauVar24 = pauStack_118;
    }
  } while( true );
override_jmp_14229aa20_case_0:
  uStack_2b8 = *(undefined8 *)(*pauVar24 + 8);
  pauStack_118 = pauVar22;
  if (*_DAT_14362c098 != '\0') {
    pauVar23 = (undefined1 (*) [16])func_0x000140613c10(0x280,8);
    if (pauVar23 == (undefined1 (*) [16])0x0) {
      FUN_1428d9000(8,0x280);
      goto LAB_1422a2418;
    }
    pauVar19 = (undefined1 (*) [16])func_0x000140613c10(0x16,1);
    uVar31 = _UNK_14303c1a6;
    if (pauVar19 == (undefined1 (*) [16])0x0) {
      pauStack_f8 = pauVar23;
      FUN_1428d8fe3(1,0x16,&UNK_143136c68);
    }
    else {
      *(undefined8 *)*pauVar19 = _DAT_14303c19e;
      *(undefined8 *)(*pauVar19 + 8) = uVar31;
      *(undefined8 *)(*pauVar19 + 0xe) = 0x796572675f736b63;
      pauVar43 = (undefined1 (*) [16])func_0x000140613c10(0xe,1);
      if (pauVar43 == (undefined1 (*) [16])0x0) {
        pauStack_100 = pauVar19;
        pauStack_f8 = pauVar23;
        FUN_1428d8fe3(1,0xe,&UNK_143136c68);
      }
      else {
        *(undefined8 *)(*pauVar43 + 6) = 0x73726f6c6f635f72;
        *(undefined8 *)*pauVar43 = 0x5f72657473616c70;
        pauVar20 = (undefined1 (*) [16])func_0x000140613c10(0xe,1);
        if (pauVar20 == (undefined1 (*) [16])0x0) {
          pauStack_118 = pauVar43;
          pauStack_100 = pauVar19;
          pauStack_f8 = pauVar23;
          FUN_1428d8fe3(1,0xe,&UNK_143136c68);
        }
        else {
          *(undefined8 *)(*pauVar20 + 6) = 0x73726f6c6f635f72;
          *(undefined8 *)*pauVar20 = 0x5f72657473616c70;
          pauVar18 = (undefined1 (*) [16])func_0x000140613c10(0x11,1);
          uVar12 = _UNK_14303c298;
          uVar31 = _DAT_14303c290;
          if (pauVar18 == (undefined1 (*) [16])0x0) {
            pauStack_130 = pauVar20;
            pauStack_118 = pauVar43;
            pauStack_100 = pauVar19;
            pauStack_f8 = pauVar23;
            FUN_1428d8fe3(1,0x11,&UNK_143136c68);
          }
          else {
            *(undefined8 *)*pauVar18 = _DAT_14303c290;
            *(undefined8 *)(*pauVar18 + 8) = uVar12;
            pauVar18[1][0] = 100;
            pauVar21 = (undefined1 (*) [16])func_0x000140613c10(0x11,1);
            if (pauVar21 == (undefined1 (*) [16])0x0) {
              pauStack_148 = pauVar18;
              pauStack_130 = pauVar20;
              pauStack_118 = pauVar43;
              pauStack_100 = pauVar19;
              pauStack_f8 = pauVar23;
              FUN_1428d8fe3(1,0x11,&UNK_143136c68);
            }
            else {
              *(undefined8 *)*pauVar21 = uVar31;
              *(undefined8 *)(*pauVar21 + 8) = uVar12;
              pauVar21[1][0] = 100;
              pauVar22 = (undefined1 (*) [16])func_0x000140613c10(0x11,1);
              if (pauVar22 == (undefined1 (*) [16])0x0) {
                pauStack_180 = pauVar21;
                pauStack_148 = pauVar18;
                pauStack_130 = pauVar20;
                pauStack_118 = pauVar43;
                pauStack_100 = pauVar19;
                pauStack_f8 = pauVar23;
                FUN_1428d8fe3(1,0x11,&UNK_143136c68);
              }
              else {
                *(undefined8 *)*pauVar22 = uVar31;
                *(undefined8 *)(*pauVar22 + 8) = uVar12;
                pauVar22[1][0] = 100;
                pauStack_108 = pauVar22;
                pauVar22 = (undefined1 (*) [16])func_0x000140613c10(0x11,1);
                if (pauVar22 == (undefined1 (*) [16])0x0) {
                  pauStack_180 = pauVar21;
                  pauStack_148 = pauVar18;
                  pauStack_130 = pauVar20;
                  pauStack_118 = pauVar43;
                  pauStack_100 = pauVar19;
                  pauStack_f8 = pauVar23;
                  FUN_1428d8fe3(1,0x11,&UNK_143136c68);
                }
                else {
                  *(undefined8 *)*pauVar22 = uVar31;
                  *(undefined8 *)(*pauVar22 + 8) = uVar12;
                  pauVar22[1][0] = 100;
                  uStack_110 = pauVar22;
                  pauVar22 = (undefined1 (*) [16])func_0x000140613c10(0x11,1);
                  if (pauVar22 == (undefined1 (*) [16])0x0) {
                    pauStack_180 = pauVar21;
                    pauStack_148 = pauVar18;
                    pauStack_130 = pauVar20;
                    pauStack_118 = pauVar43;
                    pauStack_100 = pauVar19;
                    pauStack_f8 = pauVar23;
                    FUN_1428d8fe3(1,0x11,&UNK_143136c68);
                  }
                  else {
                    *(undefined8 *)*pauVar22 = uVar31;
                    *(undefined8 *)(*pauVar22 + 8) = uVar12;
                    pauVar22[1][0] = 100;
                    pauStack_120 = pauVar22;
                    pauVar22 = (undefined1 (*) [16])func_0x000140613c10(0x11,1);
                    if (pauVar22 == (undefined1 (*) [16])0x0) {
                      pauStack_180 = pauVar21;
                      pauStack_148 = pauVar18;
                      pauStack_130 = pauVar20;
                      pauStack_118 = pauVar43;
                      pauStack_100 = pauVar19;
                      pauStack_f8 = pauVar23;
                      FUN_1428d8fe3(1,0x11,&UNK_143136c68);
                    }
                    else {
                      *(undefined8 *)*pauVar22 = uVar31;
                      *(undefined8 *)(*pauVar22 + 8) = uVar12;
                      pauVar22[1][0] = 100;
                      pauStack_138 = pauVar22;
                      puVar25 = (undefined8 *)func_0x000140613c10(0x11,1);
                      if (puVar25 == (undefined8 *)0x0) {
                        pauStack_180 = pauVar21;
                        pauStack_148 = pauVar18;
                        pauStack_130 = pauVar20;
                        pauStack_118 = pauVar43;
                        pauStack_100 = pauVar19;
                        pauStack_f8 = pauVar23;
                        FUN_1428d8fe3(1,0x11,&UNK_143136c68);
                      }
                      else {
                        *puVar25 = uVar31;
                        puVar25[1] = uVar12;
                        *(undefined1 *)(puVar25 + 2) = 100;
                        puStack_1a8 = puVar25;
                        puVar25 = (undefined8 *)func_0x000140613c10(0x11,1);
                        if (puVar25 == (undefined8 *)0x0) {
                          pauStack_180 = pauVar21;
                          pauStack_148 = pauVar18;
                          pauStack_130 = pauVar20;
                          pauStack_118 = pauVar43;
                          pauStack_100 = pauVar19;
                          pauStack_f8 = pauVar23;
                          FUN_1428d8fe3(1,0x11,&UNK_143136c68);
                        }
                        else {
                          *puVar25 = uVar31;
                          puVar25[1] = uVar12;
                          *(undefined1 *)(puVar25 + 2) = 100;
                          puStack_1b0 = puVar25;
                          puVar25 = (undefined8 *)func_0x000140613c10(0x11,1);
                          if (puVar25 == (undefined8 *)0x0) {
                            pauStack_180 = pauVar21;
                            pauStack_148 = pauVar18;
                            pauStack_130 = pauVar20;
                            pauStack_118 = pauVar43;
                            pauStack_100 = pauVar19;
                            pauStack_f8 = pauVar23;
                            FUN_1428d8fe3(1,0x11,&UNK_143136c68);
                          }
                          else {
                            *puVar25 = uVar31;
                            puVar25[1] = uVar12;
                            *(undefined1 *)(puVar25 + 2) = 100;
                            puStack_1f8 = puVar25;
                            puVar25 = (undefined8 *)func_0x000140613c10(5,1);
                            if (puVar25 == (undefined8 *)0x0) {
                              pauStack_180 = pauVar21;
                              pauStack_148 = pauVar18;
                              pauStack_130 = pauVar20;
                              pauStack_118 = pauVar43;
                              pauStack_100 = pauVar19;
                              pauStack_f8 = pauVar23;
                              FUN_1428d8fe3(1,5,&UNK_143136c68);
                            }
                            else {
                              *(undefined1 *)((longlong)puVar25 + 4) = DAT_143136d57;
                              *(undefined4 *)puVar25 = _DAT_143136d53;
                              puStack_228 = puVar25;
                              puVar25 = (undefined8 *)func_0x000140613c10(0x11,1);
                              if (puVar25 == (undefined8 *)0x0) {
                                pauStack_180 = pauVar21;
                                pauStack_148 = pauVar18;
                                pauStack_130 = pauVar20;
                                pauStack_118 = pauVar43;
                                pauStack_100 = pauVar19;
                                pauStack_f8 = pauVar23;
                                FUN_1428d8fe3(1,0x11,&UNK_143136c68);
                              }
                              else {
                                *puVar25 = uVar31;
                                puVar25[1] = uVar12;
                                *(undefined1 *)(puVar25 + 2) = 100;
                                puStack_230 = puVar25;
                                puVar25 = (undefined8 *)func_0x000140613c10(0x10,1);
                                uVar31 = _UNK_14303c245;
                                if (puVar25 == (undefined8 *)0x0) {
                                  pauStack_180 = pauVar21;
                                  pauStack_148 = pauVar18;
                                  pauStack_130 = pauVar20;
                                  pauStack_118 = pauVar43;
                                  pauStack_100 = pauVar19;
                                  pauStack_f8 = pauVar23;
                                  FUN_1428d8fe3(1,0x10,&UNK_143136c68);
                                }
                                else {
                                  *puVar25 = _DAT_14303c23d;
                                  puVar25[1] = uVar31;
                                  puStack_250 = puVar25;
                                  puVar25 = (undefined8 *)func_0x000140613c10(0x13,1);
                                  if (puVar25 != (undefined8 *)0x0) {
                                    *(undefined8 *)*pauVar23 = 0x16;
                                    *(undefined1 (**) [16])(*pauVar23 + 8) = pauVar19;
                                    *(undefined8 *)pauVar23[1] = 0x16;
                                    pauVar23[1][8] = 3;
                                    *(undefined4 *)(pauVar23[1] + 0xc) = 3;
                                    *(undefined4 *)(pauVar23[2] + 4) = 0;
                                    *(undefined8 *)(pauVar23[2] + 8) = 0xe;
                                    *(undefined1 (**) [16])pauVar23[3] = pauVar43;
                                    *(undefined8 *)(pauVar23[3] + 8) = 0xe;
                                    pauVar23[4][0] = 3;
                                    *(undefined4 *)(pauVar23[4] + 4) = 5;
                                    *(undefined4 *)(pauVar23[4] + 0xc) = 1;
                                    *(undefined8 *)pauVar23[5] = 0xe;
                                    *(undefined1 (**) [16])(pauVar23[5] + 8) = pauVar20;
                                    *(undefined8 *)pauVar23[6] = 0xe;
                                    pauVar23[6][8] = 3;
                                    *(undefined4 *)(pauVar23[6] + 0xc) = 8;
                                    *(undefined4 *)(pauVar23[7] + 4) = 4;
                                    *(undefined8 *)(pauVar23[7] + 8) = 0x11;
                                    *(undefined1 (**) [16])pauVar23[8] = pauVar18;
                                    *(undefined8 *)(pauVar23[8] + 8) = 0x11;
                                    pauVar23[9][0] = 3;
                                    *(undefined4 *)(pauVar23[9] + 4) = 0x309;
                                    *(undefined4 *)(pauVar23[9] + 0xc) = 0;
                                    *(undefined8 *)pauVar23[10] = 0x11;
                                    *(undefined1 (**) [16])(pauVar23[10] + 8) = pauVar21;
                                    *(undefined8 *)pauVar23[0xb] = 0x11;
                                    pauVar23[0xb][8] = 3;
                                    *(undefined4 *)(pauVar23[0xb] + 0xc) = 0x309;
                                    *(undefined4 *)(pauVar23[0xc] + 4) = 0;
                                    *(undefined8 *)(pauVar23[0xc] + 8) = 0x11;
                                    *(undefined1 (**) [16])pauVar23[0xd] = pauStack_108;
                                    *(undefined8 *)(pauVar23[0xd] + 8) = 0x11;
                                    pauVar23[0xe][0] = 3;
                                    *(undefined4 *)(pauVar23[0xe] + 4) = 0x309;
                                    *(undefined4 *)(pauVar23[0xe] + 0xc) = 0;
                                    *(undefined8 *)pauVar23[0xf] = 0x11;
                                    *(undefined1 (**) [16])(pauVar23[0xf] + 8) = uStack_110;
                                    *(undefined8 *)pauVar23[0x10] = 0x11;
                                    pauVar23[0x10][8] = 3;
                                    *(undefined4 *)(pauVar23[0x10] + 0xc) = 0x309;
                                    *(undefined4 *)(pauVar23[0x11] + 4) = 0;
                                    *(undefined8 *)(pauVar23[0x11] + 8) = 0x11;
                                    *(undefined1 (**) [16])pauVar23[0x12] = pauStack_120;
                                    *(undefined8 *)(pauVar23[0x12] + 8) = 0x11;
                                    pauVar23[0x13][0] = 3;
                                    *(undefined4 *)(pauVar23[0x13] + 4) = 0x309;
                                    *(undefined4 *)(pauVar23[0x13] + 0xc) = 0;
                                    *(undefined8 *)pauVar23[0x14] = 0x11;
                                    *(undefined1 (**) [16])(pauVar23[0x14] + 8) = pauStack_138;
                                    *(undefined8 *)pauVar23[0x15] = 0x11;
                                    pauVar23[0x15][8] = 3;
                                    *(undefined4 *)(pauVar23[0x15] + 0xc) = 0x309;
                                    *(undefined4 *)(pauVar23[0x16] + 4) = 0;
                                    *(undefined8 *)(pauVar23[0x16] + 8) = 0x11;
                                    *(undefined8 **)pauVar23[0x17] = puStack_1a8;
                                    *(undefined8 *)(pauVar23[0x17] + 8) = 0x11;
                                    pauVar23[0x18][0] = 3;
                                    *(undefined4 *)(pauVar23[0x18] + 4) = 0x309;
                                    *(undefined4 *)(pauVar23[0x18] + 0xc) = 0;
                                    *(undefined8 *)pauVar23[0x19] = 0x11;
                                    *(undefined8 **)(pauVar23[0x19] + 8) = puStack_1b0;
                                    *(undefined8 *)pauVar23[0x1a] = 0x11;
                                    pauVar23[0x1a][8] = 3;
                                    *(undefined4 *)(pauVar23[0x1a] + 0xc) = 0x309;
                                    *(undefined4 *)(pauVar23[0x1b] + 4) = 0;
                                    *(undefined8 *)(pauVar23[0x1b] + 8) = 0x11;
                                    *(undefined8 **)pauVar23[0x1c] = puStack_1f8;
                                    *(undefined8 *)(pauVar23[0x1c] + 8) = 0x11;
                                    pauVar23[0x1d][0] = 3;
                                    *(undefined4 *)(pauVar23[0x1d] + 4) = 0x309;
                                    *(undefined4 *)(pauVar23[0x1d] + 0xc) = 0;
                                    *(undefined8 *)pauVar23[0x1e] = 5;
                                    *(undefined8 **)(pauVar23[0x1e] + 8) = puStack_228;
                                    *(undefined8 *)pauVar23[0x1f] = 5;
                                    pauVar23[0x1f][8] = 5;
                                    *(undefined4 *)(pauVar23[0x20] + 4) = 0;
                                    *(undefined8 *)(pauVar23[0x20] + 8) = 0x11;
                                    *(undefined8 **)pauVar23[0x21] = puStack_230;
                                    *(undefined8 *)(pauVar23[0x21] + 8) = 0x11;
                                    pauVar23[0x22][0] = 3;
                                    *(undefined4 *)(pauVar23[0x22] + 4) = 0x309;
                                    *(undefined4 *)(pauVar23[0x22] + 0xc) = 0;
                                    *(undefined8 *)pauVar23[0x23] = 0x10;
                                    *(undefined8 **)(pauVar23[0x23] + 8) = puStack_250;
                                    uVar31 = _UNK_14303c255;
                                    *puVar25 = _DAT_14303c24d;
                                    puVar25[1] = uVar31;
                                    *(undefined4 *)((longlong)puVar25 + 0xf) = 0x7472696b;
                                    *(undefined8 *)pauVar23[0x24] = 0x10;
                                    *(undefined2 *)(pauVar23[0x24] + 8) = 0x104;
                                    *(undefined4 *)(pauVar23[0x25] + 4) = 0;
                                    *(undefined8 *)(pauVar23[0x25] + 8) = 0x13;
                                    *(undefined8 **)pauVar23[0x26] = puVar25;
                                    *(undefined8 *)(pauVar23[0x26] + 8) = 0x13;
                                    *(undefined2 *)pauVar23[0x27] = 4;
                                    *(undefined4 *)(pauVar23[0x27] + 0xc) = 0;
                                    pauStack_1e8 = (undefined1 (*) [16])0x10;
                                    pauStack_1d8 = (undefined1 (*) [16])0x10;
                                    pauVar20 = pauStack_1e8;
                                    pauVar19 = pauStack_1d8;
                                    goto LAB_14229e9e0;
                                  }
                                  pauStack_180 = pauVar21;
                                  pauStack_148 = pauVar18;
                                  pauStack_130 = pauVar20;
                                  pauStack_118 = pauVar43;
                                  pauStack_100 = pauVar19;
                                  pauStack_f8 = pauVar23;
                                  FUN_1428d8fe3(1,0x13,&UNK_143136c68);
                                }
                              }
                            }
                          }
                        }
                      }
                    }
                  }
                }
              }
            }
          }
        }
      }
    }
    goto LAB_1422a2ea6;
  }
  puVar25 = (undefined8 *)FUN_140a91ef0(pauStack_100,uStack_2b8,&UNK_14303c140);
  if (puVar25 == (undefined8 *)0x0) {
    return;
  }
  pauVar19 = (undefined1 (*) [16])func_0x000140613c10(0x208,8);
  if (pauVar19 == (undefined1 (*) [16])0x0) {
    FUN_1428d9000(8,0x208);
  }
  else {
    pauVar18 = (undefined1 (*) [16])func_0x000140613c10(0x17,1);
    uVar31 = _UNK_14303c160;
    pauVar20 = pauVar19;
    if (pauVar18 != (undefined1 (*) [16])0x0) {
      *(undefined8 *)*pauVar18 = _DAT_14303c158;
      *(undefined8 *)(*pauVar18 + 8) = uVar31;
      *(undefined8 *)(*pauVar18 + 0xf) = 0x65676965625f736b;
      pauVar20 = (undefined1 (*) [16])func_0x000140613c10(0x18,1);
      uVar31 = _UNK_14303c177;
      if (pauVar20 == (undefined1 (*) [16])0x0) {
        pauStack_100 = pauVar18;
        pauStack_f8 = pauVar19;
        FUN_1428d8fe3(1,0x18,&UNK_143136c68);
      }
      else {
        *(undefined8 *)*pauVar20 = _DAT_14303c16f;
        *(undefined8 *)(*pauVar20 + 8) = uVar31;
        *(undefined8 *)pauVar20[1] = 0x776f6c6c65795f73;
        pauVar43 = (undefined1 (*) [16])func_0x000140613c10(0x17,1);
        uVar31 = _UNK_14303c18f;
        if (pauVar43 == (undefined1 (*) [16])0x0) {
          pauStack_118 = pauVar20;
          pauStack_100 = pauVar18;
          pauStack_f8 = pauVar19;
          FUN_1428d8fe3(1,0x17,&UNK_143136c68);
        }
        else {
          *(undefined8 *)*pauVar43 = _DAT_14303c187;
          *(undefined8 *)(*pauVar43 + 8) = uVar31;
          *(undefined8 *)(*pauVar43 + 0xf) = 0x746867696c5f736b;
          pauVar21 = (undefined1 (*) [16])func_0x000140613c10(0x16,1);
          uVar31 = _UNK_14303c1a6;
          pauStack_108 = pauVar43;
          if (pauVar21 == (undefined1 (*) [16])0x0) {
            pauStack_118 = pauVar20;
            pauStack_100 = pauVar18;
            pauStack_f8 = pauVar19;
            FUN_1428d8fe3(1,0x16,&UNK_143136c68);
          }
          else {
            *(undefined8 *)*pauVar21 = _DAT_14303c19e;
            *(undefined8 *)(*pauVar21 + 8) = uVar31;
            *(undefined8 *)(*pauVar21 + 0xe) = 0x796572675f736b63;
            uStack_110 = pauVar21;
            pauVar43 = (undefined1 (*) [16])func_0x000140613c10(0x16,1);
            uVar31 = _UNK_14303c1bc;
            if (pauVar43 == (undefined1 (*) [16])0x0) {
              pauStack_118 = pauVar20;
              pauStack_100 = pauVar18;
              pauStack_f8 = pauVar19;
              FUN_1428d8fe3(1,0x16,&UNK_143136c68);
            }
            else {
              *(undefined8 *)*pauVar43 = _DAT_14303c1b4;
              *(undefined8 *)(*pauVar43 + 8) = uVar31;
              *(undefined8 *)(*pauVar43 + 0xe) = 0x6b7261645f736b63;
              pauStack_120 = pauVar43;
              pauVar43 = (undefined1 (*) [16])func_0x000140613c10(0xe,1);
              if (pauVar43 == (undefined1 (*) [16])0x0) {
                pauStack_118 = pauVar20;
                pauStack_100 = pauVar18;
                pauStack_f8 = pauVar19;
                FUN_1428d8fe3(1,0xe,&UNK_143136c68);
              }
              else {
                *(undefined8 *)(*pauVar43 + 6) = 0x73726f6c6f635f72;
                *(undefined8 *)*pauVar43 = 0x5f72657473616c70;
                pauStack_138 = pauVar43;
                puVar26 = (undefined8 *)func_0x000140613c10(0xe,1);
                if (puVar26 == (undefined8 *)0x0) {
                  pauStack_118 = pauVar20;
                  pauStack_100 = pauVar18;
                  pauStack_f8 = pauVar19;
                  FUN_1428d8fe3(1,0xe,&UNK_143136c68);
                }
                else {
                  *(undefined8 *)((longlong)puVar26 + 6) = 0x73726f6c6f635f72;
                  *puVar26 = 0x5f72657473616c70;
                  puStack_1a8 = puVar26;
                  puVar26 = (undefined8 *)func_0x000140613c10(0xe,1);
                  if (puVar26 == (undefined8 *)0x0) {
                    pauStack_118 = pauVar20;
                    pauStack_100 = pauVar18;
                    pauStack_f8 = pauVar19;
                    FUN_1428d8fe3(1,0xe,&UNK_143136c68);
                  }
                  else {
                    *(undefined8 *)((longlong)puVar26 + 6) = 0x73726f6c6f635f72;
                    *puVar26 = 0x5f72657473616c70;
                    puStack_1b0 = puVar26;
                    puVar26 = (undefined8 *)func_0x000140613c10(0xe,1);
                    if (puVar26 == (undefined8 *)0x0) {
                      pauStack_118 = pauVar20;
                      pauStack_100 = pauVar18;
                      pauStack_f8 = pauVar19;
                      FUN_1428d8fe3(1,0xe,&UNK_143136c68);
                    }
                    else {
                      *(undefined8 *)((longlong)puVar26 + 6) = 0x73726f6c6f635f72;
                      *puVar26 = 0x5f72657473616c70;
                      puStack_1f8 = puVar26;
                      puVar26 = (undefined8 *)func_0x000140613c10(0xe,1);
                      if (puVar26 != (undefined8 *)0x0) {
                        *(undefined8 *)((longlong)puVar26 + 6) = 0x73726f6c6f635f72;
                        *puVar26 = 0x5f72657473616c70;
                        puStack_228 = puVar26;
                        puVar26 = (undefined8 *)func_0x000140613c10(0xe,1);
                        if (puVar26 == (undefined8 *)0x0) {
                          pauStack_118 = pauVar20;
                          pauStack_100 = pauVar18;
                          pauStack_f8 = pauVar19;
                          FUN_1428d8fe3(1,0xe,&UNK_143136c68);
                          goto LAB_1422a2ea6;
                        }
                        *(undefined8 *)((longlong)puVar26 + 6) = 0x73726f6c6f635f72;
                        *puVar26 = 0x5f72657473616c70;
                        puStack_250 = puVar25;
                        puStack_230 = puVar26;
                        pauVar43 = (undefined1 (*) [16])func_0x000140613c10(0xe,1);
                        if (pauVar43 == (undefined1 (*) [16])0x0) {
                          pauStack_118 = pauVar20;
                          pauStack_100 = pauVar18;
                          pauStack_f8 = pauVar19;
                          FUN_1428d8fe3(1,0xe,&UNK_143136c68);
                          goto LAB_1422a2ea6;
                        }
                        *(undefined8 *)(*pauVar43 + 6) = 0x73726f6c6f635f72;
                        *(undefined8 *)*pauVar43 = 0x5f72657473616c70;
                        puVar25 = (undefined8 *)func_0x000140613c10(0xe,1);
                        if (puVar25 == (undefined8 *)0x0) {
                          pauStack_130 = pauVar43;
                          pauStack_118 = pauVar20;
                          pauStack_100 = pauVar18;
                          pauStack_f8 = pauVar19;
                          FUN_1428d8fe3(1,0xe,&UNK_143136c68);
                          goto LAB_1422a2ea6;
                        }
                        *(undefined8 *)*pauVar19 = 0x17;
                        *(undefined1 (**) [16])(*pauVar19 + 8) = pauVar18;
                        *(undefined8 *)pauVar19[1] = 0x17;
                        pauVar19[1][8] = 3;
                        *(undefined4 *)(pauVar19[1] + 0xc) = 0;
                        *(undefined4 *)(pauVar19[2] + 4) = 0;
                        *(undefined8 *)(pauVar19[2] + 8) = 0x18;
                        *(undefined1 (**) [16])pauVar19[3] = pauVar20;
                        *(undefined8 *)(pauVar19[3] + 8) = 0x18;
                        pauVar19[4][0] = 3;
                        *(undefined4 *)(pauVar19[4] + 4) = 1;
                        *(undefined4 *)(pauVar19[4] + 0xc) = 0;
                        *(undefined8 *)pauVar19[5] = 0x17;
                        *(undefined1 (**) [16])(pauVar19[5] + 8) = pauStack_108;
                        *(undefined8 *)pauVar19[6] = 0x17;
                        pauVar19[6][8] = 3;
                        *(undefined4 *)(pauVar19[6] + 0xc) = 3;
                        *(undefined4 *)(pauVar19[7] + 4) = 0;
                        *(undefined8 *)(pauVar19[7] + 8) = 0x16;
                        *(undefined1 (**) [16])pauVar19[8] = uStack_110;
                        *(undefined8 *)(pauVar19[8] + 8) = 0x16;
                        pauVar19[9][0] = 3;
                        *(undefined4 *)(pauVar19[9] + 4) = 0xb;
                        *(undefined4 *)(pauVar19[9] + 0xc) = 0;
                        *(undefined8 *)pauVar19[10] = 0x16;
                        *(undefined1 (**) [16])(pauVar19[10] + 8) = pauStack_120;
                        *(undefined8 *)pauVar19[0xb] = 0x16;
                        pauVar19[0xb][8] = 3;
                        *(undefined4 *)(pauVar19[0xb] + 0xc) = 2;
                        *(undefined4 *)(pauVar19[0xc] + 4) = 0;
                        *(undefined8 *)(pauVar19[0xc] + 8) = 0xe;
                        *(undefined1 (**) [16])pauVar19[0xd] = pauStack_138;
                        *(undefined8 *)(pauVar19[0xd] + 8) = 0xe;
                        pauVar19[0xe][0] = 3;
                        *(undefined4 *)(pauVar19[0xe] + 4) = 4;
                        *(undefined4 *)(pauVar19[0xe] + 0xc) = 0;
                        *(undefined8 *)pauVar19[0xf] = 0xe;
                        *(undefined8 **)(pauVar19[0xf] + 8) = puStack_1a8;
                        *(undefined8 *)pauVar19[0x10] = 0xe;
                        pauVar19[0x10][8] = 3;
                        *(undefined4 *)(pauVar19[0x10] + 0xc) = 0xc;
                        *(undefined4 *)(pauVar19[0x11] + 4) = 7;
                        *(undefined8 *)(pauVar19[0x11] + 8) = 0xe;
                        *(undefined8 **)pauVar19[0x12] = puStack_1b0;
                        *(undefined8 *)(pauVar19[0x12] + 8) = 0xe;
                        pauVar19[0x13][0] = 3;
                        *(undefined4 *)(pauVar19[0x13] + 4) = 5;
                        *(undefined4 *)(pauVar19[0x13] + 0xc) = 1;
                        *(undefined8 *)pauVar19[0x14] = 0xe;
                        *(undefined8 **)(pauVar19[0x14] + 8) = puStack_1f8;
                        *(undefined8 *)pauVar19[0x15] = 0xe;
                        pauVar19[0x15][8] = 3;
                        *(undefined4 *)(pauVar19[0x15] + 0xc) = 10;
                        *(undefined4 *)(pauVar19[0x16] + 4) = 6;
                        *(undefined8 *)(pauVar19[0x16] + 8) = 0xe;
                        *(undefined8 **)pauVar19[0x17] = puStack_228;
                        *(undefined8 *)(pauVar19[0x17] + 8) = 0xe;
                        pauVar19[0x18][0] = 3;
                        *(undefined4 *)(pauVar19[0x18] + 4) = 8;
                        *(undefined4 *)(pauVar19[0x18] + 0xc) = 4;
                        *(undefined8 *)pauVar19[0x19] = 0xe;
                        *(undefined8 **)(pauVar19[0x19] + 8) = puStack_230;
                        *(undefined8 *)pauVar19[0x1a] = 0xe;
                        pauVar19[0x1a][8] = 3;
                        *(undefined4 *)(pauVar19[0x1a] + 0xc) = 9;
                        *(undefined4 *)(pauVar19[0x1b] + 4) = 5;
                        *(undefined8 *)(pauVar19[0x1b] + 8) = 0xe;
                        *(undefined1 (**) [16])pauVar19[0x1c] = pauVar43;
                        *(undefined8 *)((longlong)puVar25 + 6) = 0x73726f6c6f635f72;
                        *puVar25 = 0x5f72657473616c70;
                        *(undefined8 *)(pauVar19[0x1c] + 8) = 0xe;
                        pauVar19[0x1d][0] = 3;
                        *(undefined4 *)(pauVar19[0x1d] + 4) = 6;
                        *(undefined4 *)(pauVar19[0x1d] + 0xc) = 2;
                        *(undefined8 *)pauVar19[0x1e] = 0xe;
                        *(undefined8 **)(pauVar19[0x1e] + 8) = puVar25;
                        *(undefined8 *)pauVar19[0x1f] = 0xe;
                        pauVar19[0x1f][8] = 3;
                        *(undefined4 *)(pauVar19[0x1f] + 0xc) = 7;
                        *(undefined4 *)(pauVar19[0x20] + 4) = 3;
                        uStack_1a0 = (undefined1 (*) [16])0xd;
                        pauStack_190 = (undefined1 (*) [16])0xd;
                        pauStack_198 = pauVar19;
                        uVar30 = func_0x0001400885d0(&uStack_2b8);
                        puVar25 = puStack_250;
                        if (*(ulonglong *)pauStack_180[1] <= uVar30) {
                          FUN_1428d9518(uVar30,*(ulonglong *)pauStack_180[1],&UNK_14303c1d0);
                          goto LAB_1422a2ea6;
                        }
                        iVar3 = *(int *)(*(longlong *)(*pauStack_180 + 8) + uVar30 * 4);
                        if ((iVar3 - 4U < 7) || (iVar3 == 0xc)) {
                          pauVar19 = (undefined1 (*) [16])func_0x000140613c10(0xa0,8);
                          if (pauVar19 == (undefined1 (*) [16])0x0) {
                            FUN_1428d9000(8,0xa0);
                            goto LAB_1422a2ea6;
                          }
                          pauVar20 = (undefined1 (*) [16])func_0x000140613c10(5,1);
                          if (pauVar20 == (undefined1 (*) [16])0x0) {
                            pauStack_108 = pauVar19;
                            FUN_1428d8fe3(1,5,&UNK_143136c68);
                            goto LAB_1422a2ea6;
                          }
                          (*pauVar20)[4] = DAT_143136d57;
                          *(undefined4 *)*pauVar20 = _DAT_143136d53;
                          pauVar18 = (undefined1 (*) [16])func_0x000140613c10(0x1b,1);
                          uVar31 = _UNK_14303c1fb;
                          if (pauVar18 == (undefined1 (*) [16])0x0) {
                            uStack_110 = pauVar20;
                            pauStack_108 = pauVar19;
                            FUN_1428d8fe3(1,0x1b,&UNK_143136c68);
                            goto LAB_1422a2ea6;
                          }
                          *(ulonglong *)(*pauVar18 + 0xb) = CONCAT35(_UNK_14303c1f8,_DAT_14303c1f3);
                          *(undefined8 *)(pauVar18[1] + 3) = uVar31;
                          uVar31 = CONCAT53(_DAT_14303c1f3,_UNK_14303c1f0);
                          *(undefined8 *)*pauVar18 = _DAT_14303c1e8;
                          *(undefined8 *)(*pauVar18 + 8) = uVar31;
                          pauVar43 = (undefined1 (*) [16])func_0x000140613c10(0x1e,1);
                          uVar31 = _UNK_14303c219;
                          if (pauVar43 == (undefined1 (*) [16])0x0) {
                            pauStack_120 = pauVar18;
                            uStack_110 = pauVar20;
                            pauStack_108 = pauVar19;
                            FUN_1428d8fe3(1,0x1e,&UNK_143136c68);
                            goto LAB_1422a2ea6;
                          }
                          *(ulonglong *)(*pauVar43 + 0xe) = CONCAT62(_UNK_14303c213,_DAT_14303c211);
                          *(undefined8 *)(pauVar43[1] + 6) = uVar31;
                          uVar31 = CONCAT26(_DAT_14303c211,_UNK_14303c20b);
                          *(undefined8 *)*pauVar43 = _DAT_14303c203;
                          *(undefined8 *)(*pauVar43 + 8) = uVar31;
                          puVar26 = (undefined8 *)func_0x000140613c10(0x1c,1);
                          uVar31 = _UNK_14303c235;
                          if (puVar26 == (undefined8 *)0x0) {
                            pauStack_120 = pauVar18;
                            uStack_110 = pauVar20;
                            pauStack_108 = pauVar19;
                            pauStack_f8 = pauVar43;
                            FUN_1428d8fe3(1,0x1c,&UNK_143136c68);
                            goto LAB_1422a2ea6;
                          }
                          *(ulonglong *)((longlong)puVar26 + 0xc) =
                               CONCAT44(_UNK_14303c231,_DAT_14303c22d);
                          *(undefined8 *)((longlong)puVar26 + 0x14) = uVar31;
                          uVar31 = CONCAT44(_DAT_14303c22d,_UNK_14303c229);
                          *puVar26 = _DAT_14303c221;
                          puVar26[1] = uVar31;
                          *(undefined8 *)*pauVar19 = 5;
                          *(undefined1 (**) [16])(*pauVar19 + 8) = pauVar20;
                          *(undefined8 *)pauVar19[1] = 5;
                          pauVar19[1][8] = 5;
                          *(undefined4 *)(pauVar19[2] + 4) = 0;
                          *(undefined8 *)(pauVar19[2] + 8) = 0x1b;
                          *(undefined1 (**) [16])pauVar19[3] = pauVar18;
                          *(undefined8 *)(pauVar19[3] + 8) = 0x1b;
                          pauVar19[4][0] = 0xc;
                          *(undefined4 *)(pauVar19[4] + 4) = 0;
                          *(undefined4 *)(pauVar19[4] + 0xc) = 0;
                          *(undefined8 *)pauVar19[5] = 0x1e;
                          *(undefined1 (**) [16])(pauVar19[5] + 8) = pauVar43;
                          *(undefined8 *)pauVar19[6] = 0x1e;
                          pauVar19[6][8] = 0xc;
                          *(undefined4 *)(pauVar19[6] + 0xc) = 0x3f000000;
                          *(undefined4 *)(pauVar19[7] + 4) = 0;
                          *(undefined8 *)(pauVar19[7] + 8) = 0x1c;
                          *(undefined8 **)pauVar19[8] = puVar26;
                          *(undefined8 *)(pauVar19[8] + 8) = 0x1c;
                          pauVar19[9][0] = 0xc;
                          *(undefined4 *)(pauVar19[9] + 4) = 0x3f800000;
                          *(undefined4 *)(pauVar19[9] + 0xc) = 0;
                          pauVar20 = pauVar19 + 10;
                          uStack_168._0_4_ = 4;
                          uStack_168._4_4_ = 0;
                          uStack_160 = SUB84(pauVar20,0);
                          uStack_15c = (undefined1)((ulonglong)pauVar20 >> 0x20);
                          uStack_15b = (undefined1)((ulonglong)pauVar20 >> 0x28);
                          uStack_15a = (undefined1)((ulonglong)pauVar20 >> 0x30);
                          uStack_159 = (undefined1)((ulonglong)pauVar20 >> 0x38);
                          uStack_178 = pauVar19;
                          uStack_170 = pauVar19;
                          if ((ulonglong)((longlong)uStack_1a0 - (longlong)pauStack_190) < 4) {
                            FUN_1429107f0(&uStack_1a0,pauStack_190,4,8,0x28);
                          }
                          pauVar20 = pauStack_190;
                          func_0x000142923c00((undefined *)
                                              ((longlong)pauStack_190 * 0x28 +
                                              (longlong)pauStack_198),pauVar19,0xa0);
                          pauStack_190 = (undefined1 (*) [16])(*pauVar20 + 4);
                          func_0x000140613c20(pauVar19,0xa0,8);
                        }
                        if (*(uint *)(puVar25 + 0x4b) < 2) {
                          pauVar19 = (undefined1 (*) [16])func_0x000140613c10(0xa0,8);
                          if (pauVar19 == (undefined1 (*) [16])0x0) {
                            FUN_1428d9000(8,0xa0);
                          }
                          else {
                            pauVar20 = (undefined1 (*) [16])func_0x000140613c10(5,1);
                            if (pauVar20 == (undefined1 (*) [16])0x0) {
                              pauStack_108 = pauVar19;
                              FUN_1428d8fe3(1,5,&UNK_143136c68);
                            }
                            else {
                              (*pauVar20)[4] = DAT_143136d57;
                              *(undefined4 *)*pauVar20 = _DAT_143136d53;
                              pauVar18 = (undefined1 (*) [16])func_0x000140613c10(0x15,1);
                              uVar31 = _UNK_14303c268;
                              if (pauVar18 == (undefined1 (*) [16])0x0) {
                                uStack_110 = pauVar20;
                                pauStack_108 = pauVar19;
                                FUN_1428d8fe3(1,0x15,&UNK_143136c68);
                              }
                              else {
                                *(undefined8 *)*pauVar18 = _DAT_14303c260;
                                *(undefined8 *)(*pauVar18 + 8) = uVar31;
                                *(undefined8 *)(*pauVar18 + 0xd) = 0x7265626d6974666c;
                                pauVar21 = (undefined1 (*) [16])func_0x000140613c10(0x10,1);
                                uVar31 = _UNK_14303c245;
                                if (pauVar21 == (undefined1 (*) [16])0x0) {
                                  pauStack_120 = pauVar18;
                                  uStack_110 = pauVar20;
                                  pauStack_108 = pauVar19;
                                  FUN_1428d8fe3(1,0x10,&UNK_143136c68);
                                }
                                else {
                                  *(undefined8 *)*pauVar21 = _DAT_14303c23d;
                                  *(undefined8 *)(*pauVar21 + 8) = uVar31;
                                  puVar25 = (undefined8 *)func_0x000140613c10(0x13,1);
                                  uVar31 = _UNK_14303c255;
                                  if (puVar25 != (undefined8 *)0x0) {
                                    *puVar25 = _DAT_14303c24d;
                                    puVar25[1] = uVar31;
                                    *(undefined4 *)((longlong)puVar25 + 0xf) = 0x7472696b;
                                    *(undefined8 *)*pauVar19 = 5;
                                    *(undefined1 (**) [16])(*pauVar19 + 8) = pauVar20;
                                    *(undefined8 *)pauVar19[1] = 5;
                                    pauVar19[1][8] = 5;
                                    *(undefined4 *)(pauVar19[2] + 4) = 0;
                                    *(undefined8 *)(pauVar19[2] + 8) = 0x15;
                                    *(undefined1 (**) [16])pauVar19[3] = pauVar18;
                                    *(undefined8 *)(pauVar19[3] + 8) = 0x15;
                                    *(undefined2 *)pauVar19[4] = 0x204;
                                    *(undefined4 *)(pauVar19[4] + 0xc) = 0;
                                    *(undefined8 *)pauVar19[5] = 0x10;
                                    *(undefined1 (**) [16])(pauVar19[5] + 8) = pauVar21;
                                    *(undefined8 *)pauVar19[6] = 0x10;
                                    *(undefined2 *)(pauVar19[6] + 8) = 0x104;
                                    *(undefined4 *)(pauVar19[7] + 4) = 0;
                                    *(undefined8 *)(pauVar19[7] + 8) = 0x13;
                                    *(undefined8 **)pauVar19[8] = puVar25;
                                    *(undefined8 *)(pauVar19[8] + 8) = 0x13;
                                    *(undefined2 *)pauVar19[9] = 4;
                                    *(undefined4 *)(pauVar19[9] + 0xc) = 0;
                                    pauVar20 = pauVar19 + 10;
                                    uStack_168._0_4_ = 4;
                                    uStack_168._4_4_ = 0;
                                    uStack_160 = SUB84(pauVar20,0);
                                    uStack_15c = (undefined1)((ulonglong)pauVar20 >> 0x20);
                                    uStack_15b = (undefined1)((ulonglong)pauVar20 >> 0x28);
                                    uStack_15a = (undefined1)((ulonglong)pauVar20 >> 0x30);
                                    uStack_159 = (undefined1)((ulonglong)pauVar20 >> 0x38);
                                    uStack_178 = pauVar19;
                                    uStack_170 = pauVar19;
                                    if ((ulonglong)((longlong)uStack_1a0 - (longlong)pauStack_190) <
                                        4) {
                                      FUN_1429107f0(&uStack_1a0,pauStack_190,4,8,0x28);
                                    }
                                    pauVar20 = pauStack_190;
                                    func_0x000142923c00((undefined *)
                                                        ((longlong)pauStack_190 * 0x28 +
                                                        (longlong)pauStack_198),pauVar19,0xa0);
                                    pauStack_190 = (undefined1 (*) [16])(*pauVar20 + 4);
                                    func_0x000140613c20(pauVar19,0xa0,8);
                                    uVar30 = FUN_140a922b0(pauStack_100,uStack_2b8);
                                    if (((uVar30 >> 0x20 != 0) &&
                                        (uVar17 = uVar30 & 0xffffffff,
                                        uVar17 < *(ulonglong *)(lStack_1d0 + 0x10))) &&
                                       (*(int *)(*(longlong *)(lStack_1d0 + 8) + uVar17 * 0x14) ==
                                        (int)(uVar30 >> 0x20))) {
                                      lVar16 = *(longlong *)(lStack_1d0 + 8) + uVar17 * 0x14;
                                      uVar30 = (ulonglong)*(uint *)(lVar16 + 4);
                                      if (((uVar30 != 0xffffffff) &&
                                          (uVar30 < *(ulonglong *)(lStack_1f0 + 0x38))) &&
                                         ((*(ulonglong *)
                                            (*(longlong *)(lStack_1f0 + 0x28) +
                                            (ulonglong)(*(uint *)(lVar16 + 4) >> 6) * 8) >>
                                           (uVar30 & 0x3f) & 1) != 0)) {
                                        if (*(ulonglong *)(lStack_1f0 + 0x110) <
                                            *(ulonglong *)
                                             (*(longlong *)(lStack_1d0 + 0x1a8) + 0x40 +
                                             (ulonglong)*(uint *)(lVar16 + 0xc) * 0x48)) {
                                          lVar34 = *(longlong *)(lStack_1d0 + 0x1a8) +
                                                   (ulonglong)*(uint *)(lVar16 + 0xc) * 0x48;
                                          uVar30 = *(ulonglong *)
                                                    (*(longlong *)(lVar34 + 0x38) +
                                                    *(ulonglong *)(lStack_1f0 + 0x110) * 8);
                                          if (uVar30 == 0) goto LAB_1422a0ba1;
                                          lVar34 = *(longlong *)
                                                    (*(longlong *)(lVar34 + 0x18) + 0x10 +
                                                    ~uVar30 * 0x30);
                                        }
                                        else {
LAB_1422a0ba1:
                                          lVar34 = 0;
                                        }
                                        lVar34 = (ulonglong)*(uint *)(lVar16 + 0x10) * 0x58 + lVar34
                                        ;
                                        cVar14 = func_0x0001408e3380(lVar34);
                                        if (cVar14 != '\0') {
                                          pauStack_108 = (undefined1 (*) [16])
                                                         func_0x000140613c10(0x50,8);
                                          if (pauStack_108 == (undefined1 (*) [16])0x0) {
                                            FUN_1428d9000(8,0x50);
                                            goto LAB_1422a2ea6;
                                          }
                                          FUN_1425f2680(&pauStack_220);
                                          FUN_1425f26f0(&uStack_178,*(undefined1 *)(lVar34 + 0x3d));
                                          *(undefined8 *)pauStack_108[2] = uStack_200;
                                          *(undefined1 (**) [16])pauStack_108[1] = uStack_210;
                                          *(ulonglong *)(pauStack_108[1] + 8) =
                                               CONCAT44(uStack_204,uStack_208);
                                          *(undefined1 (**) [16])*pauStack_108 = pauStack_220;
                                          *(undefined1 (**) [16])(*pauStack_108 + 8) = pauStack_218;
                                          *(undefined1 (**) [16])(pauStack_108[2] + 8) = uStack_178;
                                          *(undefined1 (**) [16])pauStack_108[3] = uStack_170;
                                          *(uint *)(pauStack_108[3] + 8) = (uint)uStack_168;
                                          *(undefined4 *)(pauStack_108[3] + 0xc) = uStack_168._4_4_;
                                          *(undefined4 *)pauStack_108[4] = uStack_160;
                                          *(uint *)(pauStack_108[4] + 4) =
                                               CONCAT13(uStack_159,
                                                        CONCAT12(uStack_15a,
                                                                 CONCAT11(uStack_15b,uStack_15c)));
                                          *(ulonglong *)(pauStack_108[4] + 8) =
                                               CONCAT62(uStack_156,CONCAT11(uStack_157,uStack_158));
                                          pauVar19 = pauStack_108 + 5;
                                          uStack_178 = pauStack_108;
                                          uStack_168._0_4_ = 2;
                                          uStack_168._4_4_ = 0;
                                          uStack_170 = pauStack_108;
                                          uStack_160 = SUB84(pauVar19,0);
                                          uStack_15c = (undefined1)((ulonglong)pauVar19 >> 0x20);
                                          uStack_15b = (undefined1)((ulonglong)pauVar19 >> 0x28);
                                          uStack_15a = (undefined1)((ulonglong)pauVar19 >> 0x30);
                                          uStack_159 = (undefined1)((ulonglong)pauVar19 >> 0x38);
                                          FUN_141c5a140(&uStack_1a0,&uStack_178,&UNK_14303c278);
                                        }
                                      }
                                    }
LAB_14229e9c1:
                                    pauStack_1d8 = pauStack_190;
                                    pauVar20 = uStack_1a0;
                                    pauVar23 = pauStack_198;
                                    pauVar19 = pauStack_1d8;
                                    goto LAB_14229e9e0;
                                  }
                                  pauStack_120 = pauVar18;
                                  uStack_110 = pauVar20;
                                  pauStack_108 = pauVar19;
                                  pauStack_f8 = pauVar21;
                                  FUN_1428d8fe3(1,0x13,&UNK_143136c68);
                                }
                              }
                            }
                          }
                          goto LAB_1422a2ea6;
                        }
                        pauVar19 = (undefined1 (*) [16])func_0x000140613c10(200,8);
                        if (pauVar19 == (undefined1 (*) [16])0x0) {
                          FUN_1428d9000(8,200);
                        }
                        else {
                          pauVar20 = (undefined1 (*) [16])func_0x000140613c10(5,1);
                          uVar35 = DAT_143136d57;
                          pauStack_108 = pauVar19;
                          if (pauVar20 == (undefined1 (*) [16])0x0) {
                            FUN_1428d8fe3(1,5,&UNK_143136c68);
                          }
                          else {
                            (*pauVar20)[4] = DAT_143136d57;
                            uVar47 = _DAT_143136d53;
                            *(undefined4 *)*pauVar20 = _DAT_143136d53;
                            uStack_110 = pauVar20;
                            pauVar19 = (undefined1 (*) [16])func_0x000140613c10(0x10,1);
                            uVar31 = _UNK_14303c245;
                            if (pauVar19 == (undefined1 (*) [16])0x0) {
                              FUN_1428d8fe3(1,0x10,&UNK_143136c68);
                            }
                            else {
                              *(undefined8 *)*pauVar19 = _DAT_14303c23d;
                              *(undefined8 *)(*pauVar19 + 8) = uVar31;
                              pauStack_120 = pauVar19;
                              pauVar19 = (undefined1 (*) [16])func_0x000140613c10(0x13,1);
                              uVar31 = _UNK_14303c255;
                              if (pauVar19 == (undefined1 (*) [16])0x0) {
                                FUN_1428d8fe3(1,0x13,&UNK_143136c68);
                              }
                              else {
                                *(undefined8 *)*pauVar19 = _DAT_14303c24d;
                                *(undefined8 *)(*pauVar19 + 8) = uVar31;
                                *(undefined4 *)(*pauVar19 + 0xf) = 0x7472696b;
                                pauStack_138 = pauVar19;
                                puVar26 = (undefined8 *)func_0x000140613c10(5,1);
                                if (puVar26 != (undefined8 *)0x0) {
                                  *(undefined1 *)((longlong)puVar26 + 4) = uVar35;
                                  *(undefined4 *)puVar26 = uVar47;
                                  uStack_178 = (undefined1 (*) [16])
                                               CONCAT71(uStack_178._1_7_,
                                                        *(undefined1 *)((longlong)puVar25 + 0x295));
                                  puStack_1a8 = puVar26;
                                  cVar14 = func_0x000140833230(&uStack_178);
                                  if (cVar14 == '\0') {
                                    puVar25 = (undefined8 *)func_0x000140613c10(0x1f,1);
                                    uVar31 = _UNK_143136e36;
                                    if (puVar25 == (undefined8 *)0x0) {
                                      FUN_1428d8fe3(1,0x1f,&UNK_143136c68);
                                      goto LAB_1422a2ea6;
                                    }
                                    *(ulonglong *)((longlong)puVar25 + 0xf) =
                                         CONCAT71(_UNK_143136e2f,DAT_143136e2e);
                                    *(undefined8 *)((longlong)puVar25 + 0x17) = uVar31;
                                    uVar31 = CONCAT17(DAT_143136e2e,_UNK_143136e27);
                                    *puVar25 = _DAT_143136e1f;
                                    puVar25[1] = uVar31;
                                    uVar31 = 0x1f;
                                  }
                                  else {
                                    puVar25 = (undefined8 *)func_0x000140613c10(0x1e,1);
                                    uVar31 = _UNK_143136e54;
                                    if (puVar25 == (undefined8 *)0x0) {
                                      FUN_1428d8fe3(1,0x1e,&UNK_143136c68);
                                      goto LAB_1422a2ea6;
                                    }
                                    *(ulonglong *)((longlong)puVar25 + 0xe) =
                                         CONCAT62(_UNK_143136e4e,_DAT_143136e4c);
                                    *(undefined8 *)((longlong)puVar25 + 0x16) = uVar31;
                                    uVar31 = CONCAT26(_DAT_143136e4c,_UNK_143136e46);
                                    *puVar25 = _DAT_143136e3e;
                                    puVar25[1] = uVar31;
                                    uVar31 = 0x1e;
                                  }
                                  pauVar19 = pauStack_108;
                                  *(undefined8 *)*pauStack_108 = 5;
                                  *(undefined1 (**) [16])(*pauStack_108 + 8) = uStack_110;
                                  *(undefined8 *)pauStack_108[1] = 5;
                                  pauStack_108[1][8] = 5;
                                  *(undefined4 *)(pauStack_108[2] + 4) = 0;
                                  *(undefined8 *)(pauStack_108[2] + 8) = 0x10;
                                  *(undefined1 (**) [16])pauStack_108[3] = pauStack_120;
                                  *(undefined8 *)(pauStack_108[3] + 8) = 0x10;
                                  *(undefined2 *)pauStack_108[4] = 0x104;
                                  *(undefined4 *)(pauStack_108[4] + 0xc) = 0;
                                  *(undefined8 *)pauStack_108[5] = 0x13;
                                  *(undefined1 (**) [16])(pauStack_108[5] + 8) = pauStack_138;
                                  *(undefined8 *)pauStack_108[6] = 0x13;
                                  *(undefined2 *)(pauStack_108[6] + 8) = 4;
                                  *(undefined4 *)(pauStack_108[7] + 4) = 0;
                                  *(undefined8 *)(pauStack_108[7] + 8) = 5;
                                  *(undefined8 **)pauStack_108[8] = puStack_1a8;
                                  *(undefined8 *)(pauStack_108[8] + 8) = 5;
                                  pauStack_108[9][0] = 5;
                                  *(undefined4 *)(pauStack_108[9] + 0xc) = 0;
                                  *(undefined8 *)pauStack_108[10] = uVar31;
                                  *(undefined8 **)(pauStack_108[10] + 8) = puVar25;
                                  *(undefined8 *)pauStack_108[0xb] = uVar31;
                                  pauStack_108[0xb][8] = 7;
                                  *(undefined4 *)(pauStack_108[0xc] + 4) = 0;
                                  puVar38 = pauStack_108[0xc] + 8;
                                  uStack_178 = pauStack_108;
                                  uStack_168._0_4_ = 5;
                                  uStack_168._4_4_ = 0;
                                  uStack_170 = pauStack_108;
                                  uStack_160 = SUB84(puVar38,0);
                                  uStack_15c = (undefined1)((ulonglong)puVar38 >> 0x20);
                                  uStack_15b = (undefined1)((ulonglong)puVar38 >> 0x28);
                                  uStack_15a = (undefined1)((ulonglong)puVar38 >> 0x30);
                                  uStack_159 = (undefined1)((ulonglong)puVar38 >> 0x38);
                                  if ((ulonglong)((longlong)uStack_1a0 - (longlong)pauStack_190) < 5
                                     ) {
                                    FUN_1429107f0(&uStack_1a0,pauStack_190,5,8,0x28);
                                  }
                                  pauVar20 = pauStack_190;
                                  func_0x000142923c00((undefined *)
                                                      ((longlong)pauStack_190 * 0x28 +
                                                      (longlong)pauStack_198),pauVar19,200);
                                  pauStack_190 = (undefined1 (*) [16])(*pauVar20 + 5);
                                  func_0x000140613c20(pauVar19,200,8);
                                  goto LAB_14229e9c1;
                                }
                                FUN_1428d8fe3(1,5,&UNK_143136c68);
                              }
                            }
                          }
                        }
                        goto LAB_1422a2ea6;
                      }
                      pauStack_118 = pauVar20;
                      pauStack_100 = pauVar18;
                      pauStack_f8 = pauVar19;
                      FUN_1428d8fe3(1,0xe,&UNK_143136c68);
                    }
                  }
                }
              }
            }
          }
        }
      }
      goto LAB_1422a2ea6;
    }
  }
  pauStack_f8 = pauVar20;
  FUN_1428d8fe3(1,0x17,&UNK_143136c68);
  goto LAB_1422a2ea6;
}

