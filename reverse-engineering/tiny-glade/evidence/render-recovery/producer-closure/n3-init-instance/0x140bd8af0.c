
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

undefined8 * FUN_140bd8af0(undefined8 *param_1)

{
  code ******ppppppcVar1;
  undefined4 uVar2;
  undefined4 uVar3;
  undefined4 uVar4;
  char cVar5;
  uint uVar6;
  code ******ppppppcVar7;
  code *****pppppcVar8;
  longlong lVar9;
  undefined8 uVar10;
  longlong *plVar11;
  code *pcVar12;
  ulonglong uVar13;
  longlong lVar14;
  code ******ppppppcVar15;
  code ******ppppppcVar16;
  uint uVar17;
  ulonglong uVar18;
  code ******ppppppcVar19;
  bool bVar20;
  code *****pppppcStack_258;
  code *****pppppcStack_250;
  code *****pppppcStack_248;
  code *****pppppcStack_240;
  undefined8 uStack_238;
  undefined8 uStack_230;
  undefined *puStack_228;
  undefined8 uStack_220;
  undefined8 uStack_218;
  code *****pppppcStack_210;
  undefined4 uStack_208;
  undefined4 uStack_204;
  undefined4 uStack_200;
  undefined4 uStack_1fc;
  undefined4 uStack_1f8;
  undefined4 uStack_1f4;
  undefined4 uStack_1f0;
  undefined4 uStack_1ec;
  code *****pppppcStack_1e0;
  code *****pppppcStack_1d8;
  code *****pppppcStack_1d0;
  code *****pppppcStack_1c8;
  undefined8 uStack_1c0;
  undefined8 uStack_1b8;
  undefined4 uStack_1b0;
  undefined4 uStack_1ac;
  undefined4 uStack_1a8;
  undefined4 uStack_1a4;
  undefined4 uStack_1a0;
  undefined4 uStack_19c;
  code *****pppppcStack_190;
  code *****pppppcStack_188;
  undefined8 *puStack_180;
  uint uStack_174;
  code *****pppppcStack_170;
  undefined8 uStack_168;
  undefined8 uStack_160;
  undefined4 uStack_158;
  undefined4 uStack_154;
  undefined8 uStack_150;
  undefined4 uStack_148;
  undefined4 uStack_144;
  undefined4 uStack_140;
  undefined4 uStack_13c;
  code *****pppppcStack_130;
  code *****pppppcStack_128;
  code *****pppppcStack_120;
  undefined8 uStack_118;
  undefined8 uStack_110;
  undefined8 uStack_108;
  undefined8 uStack_100;
  undefined8 *puStack_f8;
  code *pcStack_f0;
  undefined1 auStack_e8 [8];
  code *****pppppcStack_e0;
  undefined8 uStack_d8;
  undefined8 uStack_d0;
  undefined8 uStack_c8;
  undefined8 uStack_c0;
  undefined1 *puStack_b8;
  code *pcStack_b0;
  undefined8 uStack_a8;
  undefined *puStack_a0;
  undefined8 uStack_98;
  undefined4 uStack_90;
  undefined4 uStack_8c;
  undefined *puStack_88;
  undefined8 uStack_80;
  code *****pppppcStack_78;
  undefined8 uStack_70;
  undefined8 uStack_68;
  undefined1 uStack_4b;
  undefined1 uStack_4a;
  undefined1 uStack_49;
  undefined8 uStack_48;
  
  uStack_48 = 0xfffffffffffffffe;
  puStack_180 = param_1;
  FUN_140d57650(&uStack_d8);
  if (SBORROW8(0,(longlong)uStack_d8)) {
    uStack_118 = (code ******)uStack_d0;
    FUN_1428d9760(&UNK_142b10490,0x2b,&uStack_118,&UNK_142b10470,&UNK_142b11270);
    goto code_r0x000140bda313;
  }
  uStack_1f8 = (undefined4)uStack_c8;
  uStack_1f4 = uStack_c8._4_4_;
  uStack_1f0 = (undefined4)uStack_c0;
  uStack_1ec = uStack_c0._4_4_;
  uStack_208 = (uint)uStack_d8;
  uStack_204 = uStack_d8._4_4_;
  uStack_200 = (undefined4)uStack_d0;
  uStack_1fc = uStack_d0._4_4_;
  ppppppcVar7 = (code ******)func_0x000140613c10(0x20,8);
  if (ppppppcVar7 == (code ******)0x0) {
    FUN_1428d9000(8,0x20);
    goto code_r0x000140bda313;
  }
  pppppcVar8 = (code *****)func_0x000140613c10(0xc,1);
  if (pppppcVar8 == (code *****)0x0) {
    pppppcStack_e0 = (code *****)ppppppcVar7;
    FUN_1428d8fe3(1,0xc,&UNK_142b10508);
    goto code_r0x000140bda313;
  }
  *pppppcVar8 = (code ****)0x312d6e616b6c7576;
  *(undefined4 *)(pppppcVar8 + 1) = 0x6c6c642e;
  *ppppppcVar7 = (code *****)0x8000000000000000;
  ppppppcVar7[1] = (code *****)0xc;
  ppppppcVar7[2] = pppppcVar8;
  ppppppcVar7[3] = (code *****)0xc;
  pppppcStack_130 = (code *****)0x1;
  pppppcStack_120 = (code *****)0x1;
  ppppppcVar16 = (code ******)0x82;
  pppppcStack_128 = (code *****)ppppppcVar7;
  FUN_141849e90(&pppppcStack_258,0x82);
  if (SBORROW8(0,(longlong)pppppcStack_258)) {
    FUN_140d57930(&uStack_d8,&UNK_142b11780,6);
    if (!SBORROW8(0,(longlong)uStack_d8)) {
      ppppppcVar7 = uStack_d8;
      ppppppcVar16 = (code ******)uStack_d0;
      if (uStack_d8 != (code ******)0x8000000000000001) goto joined_r0x000140bd8daf;
      if (!SBORROW8(0,(longlong)uStack_d0)) {
        pppppcStack_e0 = (code *****)uStack_d0;
        pppppcStack_170 = (code *****)uStack_c8;
        FUN_140d62810(&uStack_d8,uStack_c8,uStack_c0,&UNK_142b11786,8);
        ppppppcVar16 = (code ******)pppppcStack_e0;
        if ((code ******)pppppcStack_e0 != (code ******)0x0) {
          func_0x000140613c20(pppppcStack_170,pppppcStack_e0,1);
        }
        pppppcStack_1c8 = (code *****)uStack_d0;
        uStack_1c0 = uStack_c8;
        uStack_1b8 = (code ******)uStack_c0;
        pppppcStack_258 = (code *****)uStack_d8;
        if (!SBORROW8(0,(longlong)uStack_d8)) goto code_r0x000140bd8cde;
      }
    }
  }
  else {
    uStack_1b8 = (code ******)pppppcStack_240;
    pppppcStack_1c8 = pppppcStack_250;
    uStack_1c0 = (code ******)pppppcStack_248;
code_r0x000140bd8cde:
    uStack_160 = (code *)pppppcStack_1c8;
    uStack_158 = (undefined4)uStack_1c0;
    uStack_154 = uStack_1c0._4_4_;
    uStack_150 = uStack_1b8;
    uStack_168 = (code ******)pppppcStack_258;
    uVar10 = FUN_140d76ed0(&uStack_168);
    FUN_140d62810(&uStack_d8,uVar10,ppppppcVar16,&UNK_142b11288,0xc);
    pppppcVar8 = pppppcStack_120;
    uStack_108 = uStack_c8;
    uStack_100 = uStack_c0;
    uStack_118 = uStack_d8;
    uStack_110 = uStack_d0;
    if (pppppcStack_120 == pppppcStack_130) {
      FUN_140562c30(&pppppcStack_130,&UNK_142b11298);
    }
    pppppcStack_128[(longlong)pppppcVar8 * 4 + 2] = (code ****)uStack_108;
    (pppppcStack_128 + (longlong)pppppcVar8 * 4 + 2)[1] = (code ****)uStack_100;
    ppppppcVar7 = (code ******)(pppppcStack_128 + (longlong)pppppcVar8 * 4);
    *(undefined4 *)ppppppcVar7 = (undefined4)uStack_118;
    *(undefined4 *)((longlong)ppppppcVar7 + 4) = uStack_118._4_4_;
    *(undefined4 *)(ppppppcVar7 + 1) = (undefined4)uStack_110;
    *(undefined4 *)((longlong)ppppppcVar7 + 0xc) = uStack_110._4_4_;
    pppppcStack_120 = (code *****)((longlong)pppppcVar8 + 1);
    ppppppcVar7 = uStack_168;
    ppppppcVar16 = (code ******)uStack_160;
joined_r0x000140bd8daf:
    if (ppppppcVar7 != (code ******)0x0) {
      func_0x000140613c20(ppppppcVar16,ppppppcVar7,1);
    }
  }
  ppppppcVar7 = (code ******)func_0x000140613c10(0xc,1);
  pppppcVar8 = pppppcStack_120;
  if (ppppppcVar7 == (code ******)0x0) {
    FUN_1428d8fe3(1,0xc,&UNK_142b10508);
code_r0x000140bda313:
                    /* WARNING: Does not return */
    pcVar12 = (code *)invalidInstructionException();
    (*pcVar12)();
  }
  *ppppppcVar7 = (code *****)0x312d6e616b6c7576;
  *(undefined4 *)(ppppppcVar7 + 1) = 0x6c6c642e;
  uStack_d0 = (code *)0xc;
  uStack_c0 = (code *)0xc;
  uStack_d8 = (code ******)0x8000000000000001;
  uStack_c8 = ppppppcVar7;
  if (pppppcStack_120 == pppppcStack_130) {
    FUN_140562c30(&pppppcStack_130,&UNK_142b112b0);
  }
  ppppppcVar7 = (code ******)(pppppcStack_128 + (longlong)pppppcVar8 * 4 + 2);
  *(undefined4 *)ppppppcVar7 = (undefined4)uStack_c8;
  *(undefined4 *)((longlong)ppppppcVar7 + 4) = uStack_c8._4_4_;
  *(undefined4 *)(ppppppcVar7 + 1) = (undefined4)uStack_c0;
  *(undefined4 *)((longlong)ppppppcVar7 + 0xc) = uStack_c0._4_4_;
  ppppppcVar7 = (code ******)(pppppcStack_128 + (longlong)pppppcVar8 * 4);
  *(uint *)ppppppcVar7 = (uint)uStack_d8;
  *(undefined4 *)((longlong)ppppppcVar7 + 4) = uStack_d8._4_4_;
  *(undefined4 *)(ppppppcVar7 + 1) = (undefined4)uStack_d0;
  *(undefined4 *)((longlong)ppppppcVar7 + 0xc) = uStack_d0._4_4_;
  pppppcStack_120 = (code *****)((longlong)pppppcVar8 + 1);
  ppppppcVar7 = (code ******)func_0x000140613c10(0x15,1);
  pppppcVar8 = pppppcStack_120;
  uVar4 = _UNK_142b112d4;
  uVar3 = _UNK_142b112d0;
  uVar2 = _UNK_142b112cc;
  if (ppppppcVar7 == (code ******)0x0) {
    FUN_1428d8fe3(1,0x15,&UNK_142b10508);
    goto code_r0x000140bda313;
  }
  *(undefined4 *)ppppppcVar7 = _UNK_142b112c8;
  *(undefined4 *)((longlong)ppppppcVar7 + 4) = uVar2;
  *(undefined4 *)(ppppppcVar7 + 1) = uVar3;
  *(undefined4 *)((longlong)ppppppcVar7 + 0xc) = uVar4;
  *(undefined8 *)((longlong)ppppppcVar7 + 0xd) = 0x6c6c642e312d6e61;
  uStack_d0 = (code *)0x15;
  uStack_c0 = (code *)0x15;
  uStack_d8 = (code ******)0x8000000000000000;
  uStack_c8 = ppppppcVar7;
  if (pppppcStack_120 == pppppcStack_130) {
    FUN_140562c30(&pppppcStack_130,&UNK_142b112e0);
  }
  ppppppcVar7 = (code ******)(pppppcStack_128 + (longlong)pppppcVar8 * 4 + 2);
  *(undefined4 *)ppppppcVar7 = (undefined4)uStack_c8;
  *(undefined4 *)((longlong)ppppppcVar7 + 4) = uStack_c8._4_4_;
  *(undefined4 *)(ppppppcVar7 + 1) = (undefined4)uStack_c0;
  *(undefined4 *)((longlong)ppppppcVar7 + 0xc) = uStack_c0._4_4_;
  ppppppcVar7 = (code ******)(pppppcStack_128 + (longlong)pppppcVar8 * 4);
  *(uint *)ppppppcVar7 = (uint)uStack_d8;
  *(undefined4 *)((longlong)ppppppcVar7 + 4) = uStack_d8._4_4_;
  *(undefined4 *)(ppppppcVar7 + 1) = (undefined4)uStack_d0;
  *(undefined4 *)((longlong)ppppppcVar7 + 0xc) = uStack_d0._4_4_;
  pppppcStack_120 = (code *****)((longlong)pppppcVar8 + 1);
  ppppppcVar19 = (code ******)(pppppcStack_128 + (longlong)pppppcStack_120 * 4);
  pppppcStack_e0 = (code *****)0x0;
  ppppppcVar7 = (code ******)0x0;
  ppppppcVar16 = (code ******)pppppcStack_128;
  while (ppppppcVar16 != ppppppcVar19) {
    ppppppcVar15 = ppppppcVar16 + 4;
    ppppppcVar1 = (code ******)((longlong)ppppppcVar7 + 1);
    lVar9 = (longlong)*ppppppcVar16 + -0x7fffffffffffffff;
    if (-0x7fffffffffffffff < (longlong)*ppppppcVar16) {
      lVar9 = 0;
    }
    if (lVar9 == 0) {
      pppppcStack_188 = (code *****)ppppppcVar16;
      uVar10 = FUN_140d76ed0(ppppppcVar16);
      cVar5 = FUN_140d62cf0(uVar10);
      if (cVar5 == '\0') {
        ppppppcVar7 = ppppppcVar1;
        ppppppcVar16 = ppppppcVar15;
        if (3 < *puRam000000014362c3a8) {
          uStack_118 = &pppppcStack_188;
          uStack_110 = (code *)&UNK_14002e8f0;
          plVar11 = (longlong *)func_0x00014067dd80(&UNK_142b11568);
          puStack_b8 = (undefined1 *)*plVar11;
          pcStack_b0 = (code *)plVar11[1];
          uStack_8c = (undefined4)plVar11[2];
          uStack_a8 = 4;
          puStack_a0 = &UNK_142b110c0;
          uStack_98 = 0x13;
          puStack_88 = &UNK_142b11548;
          uStack_80 = 2;
          uStack_70 = 1;
          uStack_68 = 0;
          uStack_d8 = (code ******)0x0;
          uStack_d0 = (code *)&UNK_142b110c0;
          uStack_c8 = (code ******)0x13;
          uStack_c0 = (code *)0x0;
          uStack_90 = 1;
          pppppcStack_78 = (code *****)&uStack_118;
          func_0x000140d4a330(&uStack_49,&uStack_d8);
        }
      }
      else {
        if (3 < *puRam000000014362c3a8) {
          uStack_118 = &pppppcStack_188;
          uStack_110 = (code *)&UNK_14002e8f0;
          plVar11 = (longlong *)func_0x00014067dd80(&UNK_142b11580);
          puStack_b8 = (undefined1 *)*plVar11;
          pcStack_b0 = (code *)plVar11[1];
          uStack_8c = (undefined4)plVar11[2];
          uStack_a8 = 4;
          puStack_a0 = &UNK_142b110c0;
          uStack_98 = 0x13;
          puStack_88 = &UNK_142b11398;
          uStack_80 = 1;
          uStack_70 = 1;
          uStack_68 = 0;
          uStack_d8 = (code ******)0x0;
          uStack_d0 = (code *)&UNK_142b110c0;
          uStack_c8 = (code ******)0x13;
          uStack_c0 = (code *)0x0;
          uStack_90 = 1;
          pppppcStack_78 = (code *****)&uStack_118;
          func_0x000140d4a330(&uStack_49,&uStack_d8);
        }
        FUN_1418b2760(&pppppcStack_1c8,pppppcStack_188);
code_r0x000140bd934c:
        if ((code ******)pppppcStack_1c8 == (code ******)0x0) {
          uStack_108 = (code ******)CONCAT44(uStack_1ac,uStack_1b0);
          uStack_100 = (code *)CONCAT44(uStack_1a4,uStack_1a8);
          uStack_118 = uStack_1c0;
          uStack_110 = (code *)uStack_1b8;
          if (3 < *puRam000000014362c3a8) {
            uStack_160 = FUN_140bd7a70;
            uStack_168 = (code ******)&uStack_118;
            plVar11 = (longlong *)func_0x00014067dd80(&UNK_142b11768);
            puStack_b8 = (undefined1 *)*plVar11;
            pcStack_b0 = (code *)plVar11[1];
            uStack_8c = (undefined4)plVar11[2];
            uStack_a8 = 4;
            puStack_a0 = &UNK_142b110c0;
            uStack_98 = 0x13;
            puStack_88 = &UNK_142b11758;
            uStack_80 = 1;
            pppppcStack_78 = (code *****)&uStack_168;
            uStack_70 = 1;
            uStack_68 = 0;
            uStack_d8 = (code ******)0x0;
            uStack_d0 = (code *)&UNK_142b110c0;
            uStack_c8 = (code ******)0x13;
            uStack_c0 = (code *)0x0;
            uStack_90 = 1;
            func_0x000140d4a330(&uStack_49,&uStack_d8);
          }
          func_0x000140bd6e50(&uStack_118);
          ppppppcVar7 = ppppppcVar1;
          ppppppcVar16 = ppppppcVar15;
        }
        else {
          uStack_148 = uStack_1a8;
          uStack_144 = uStack_1a4;
          uStack_140 = uStack_1a0;
          uStack_13c = uStack_19c;
          uStack_158 = (undefined4)uStack_1b8;
          uStack_154 = uStack_1b8._4_4_;
          uStack_150 = (code ******)CONCAT44(uStack_1ac,uStack_1b0);
          uStack_168 = (code ******)pppppcStack_1c8;
          uStack_160 = (code *)uStack_1c0;
          pcVar12 = (code *)(*(code *)pppppcStack_1c8)(0,&UNK_142b10310);
          uVar13 = 0;
          if (pcVar12 != (code *)0x0) {
            uVar13 = (*pcVar12)(&uStack_d8);
            bVar20 = (int)uVar13 == 0;
            if (bVar20) {
              uVar13 = (ulonglong)uStack_d8 & 0xffffffff;
            }
            uVar13 = ((uVar13 << 0x20) - (ulonglong)bVar20) + 2;
          }
          uVar18 = uVar13 >> 0x20;
          if ((uVar13 & 1) == 0) {
            uVar18 = 0x400000;
          }
          if ((int)uVar13 == 2) {
            uVar18 = 0x400000;
          }
          uVar17 = (uint)uVar18;
          uStack_174 = (uint)(uVar18 >> 0x16) & 0x7f;
          auStack_e8._4_4_ = (uint)(uVar18 >> 0xc) & 0x3ff;
          auStack_e8._0_4_ = uVar17 & 0xfff;
          if (0x403105 < uVar17) {
            if (2 < *puRam000000014362c3a8) {
              uStack_d8 = (code ******)&uStack_174;
              uStack_d0 = FUN_140d8d560;
              uStack_c8 = (code ******)(auStack_e8 + 4);
              uStack_c0 = FUN_140d8d560;
              puStack_b8 = auStack_e8;
              pcStack_b0 = FUN_140d8d560;
              uStack_118 = (code ******)&UNK_142b11660;
              uStack_110 = (code *)0x4;
              puStack_f8 = (undefined8 *)0x0;
              uStack_108 = (code ******)&uStack_d8;
              uStack_100 = (code *)0x3;
              uStack_218 = func_0x00014067dd80(&UNK_142b116a0);
              uStack_238 = &UNK_142b110c0;
              uStack_230 = 0x13;
              puStack_228 = &UNK_142b110c0;
              uStack_220 = 0x13;
              FUN_1404b1430(&uStack_118,3,&uStack_238);
            }
            puStack_180[5] = CONCAT44(uStack_144,uStack_148);
            puStack_180[6] = CONCAT44(uStack_13c,uStack_140);
            puStack_180[3] = CONCAT44(uStack_154,uStack_158);
            puStack_180[4] = uStack_150;
            *(undefined4 *)(puStack_180 + 1) = (undefined4)uStack_168;
            *(undefined4 *)((longlong)puStack_180 + 0xc) = uStack_168._4_4_;
            *(undefined4 *)(puStack_180 + 2) = (undefined4)uStack_160;
            *(undefined4 *)((longlong)puStack_180 + 0x14) = uStack_160._4_4_;
            *(uint *)(puStack_180 + 7) = uVar17;
            goto code_r0x000140bd9f11;
          }
          if (1 < *puRam000000014362c3a8) {
            uStack_118 = (code ******)&uStack_174;
            uStack_110 = FUN_140d8d560;
            uStack_108 = (code ******)(auStack_e8 + 4);
            uStack_100 = FUN_140d8d560;
            puStack_f8 = (undefined8 *)auStack_e8;
            pcStack_f0 = FUN_140d8d560;
            plVar11 = (longlong *)func_0x00014067dd80(&UNK_142b11728);
            puStack_b8 = (undefined1 *)*plVar11;
            pcStack_b0 = (code *)plVar11[1];
            uStack_8c = (undefined4)plVar11[2];
            uStack_a8 = 2;
            puStack_a0 = &UNK_142b110c0;
            uStack_98 = 0x13;
            puStack_88 = &UNK_142b116e8;
            uStack_80 = 4;
            uStack_70 = 3;
            uStack_68 = 0;
            uStack_d8 = (code ******)0x0;
            uStack_d0 = (code *)&UNK_142b110c0;
            uStack_c8 = (code ******)0x13;
            uStack_c0 = (code *)0x0;
            uStack_90 = 1;
            pppppcStack_78 = (code *****)&uStack_118;
            func_0x000140d4a330(&uStack_49,&uStack_d8);
          }
          plVar11 = (longlong *)CONCAT44(uStack_13c,uStack_140);
          if (plVar11 != (longlong *)0x0) {
            LOCK();
            *plVar11 = *plVar11 + -1;
            UNLOCK();
            if (*plVar11 == 0) {
              FUN_140c8fd00(&uStack_140);
            }
          }
          if ((code ******)pppppcStack_e0 == (code ******)0x0) {
            pppppcStack_170 = (code *****)ppppppcVar7;
          }
          pppppcStack_e0 = (code *****)0x1;
          ppppppcVar7 = ppppppcVar1;
          ppppppcVar16 = ppppppcVar15;
        }
      }
    }
    else {
      ppppppcVar16 = ppppppcVar16 + 1;
      if (lVar9 != 1) {
        pppppcStack_1d0 = (code *****)ppppppcVar16;
        if (3 < *puRam000000014362c3a8) {
          uStack_118 = &pppppcStack_1d0;
          uStack_110 = (code *)&UNK_1400538e0;
          plVar11 = (longlong *)func_0x00014067dd80(&UNK_142b11628);
          puStack_b8 = (undefined1 *)*plVar11;
          pcStack_b0 = (code *)plVar11[1];
          uStack_8c = (undefined4)plVar11[2];
          uStack_a8 = 4;
          puStack_a0 = &UNK_142b110c0;
          uStack_98 = 0x13;
          puStack_88 = &UNK_142b11398;
          uStack_80 = 1;
          uStack_70 = 1;
          uStack_68 = 0;
          uStack_d8 = (code ******)0x0;
          uStack_d0 = (code *)&UNK_142b110c0;
          uStack_c8 = (code ******)0x13;
          uStack_c0 = (code *)0x0;
          uStack_90 = 1;
          pppppcStack_78 = (code *****)&uStack_118;
          func_0x000140d4a330(&uStack_49,&uStack_d8);
        }
        FUN_1418b22e0(&pppppcStack_1c8,pppppcStack_1d0);
        goto code_r0x000140bd934c;
      }
      pppppcStack_190 = (code *****)ppppppcVar16;
      uVar10 = FUN_140d76ed0(&uStack_208);
      FUN_140d62810(&uStack_118,uVar10,ppppppcVar16,pppppcStack_190[1],pppppcStack_190[2]);
      uStack_4a = 1;
      uVar10 = FUN_140d76ed0(&uStack_118);
      uStack_4a = 1;
      cVar5 = FUN_140d62cf0(uVar10);
      if (cVar5 != '\0') {
        if (3 < *puRam000000014362c3a8) {
          uStack_168 = &pppppcStack_190;
          uStack_160 = (code *)&UNK_140053920;
          uStack_4a = 1;
          plVar11 = (longlong *)func_0x00014067dd80(&UNK_142b11610);
          puStack_b8 = (undefined1 *)*plVar11;
          pcStack_b0 = (code *)plVar11[1];
          uStack_8c = (undefined4)plVar11[2];
          uStack_a8 = 4;
          puStack_a0 = &UNK_142b110c0;
          uStack_98 = 0x13;
          puStack_88 = &UNK_142b113e8;
          uStack_80 = 2;
          pppppcStack_78 = (code *****)&uStack_168;
          uStack_70 = 1;
          uStack_68 = 0;
          uStack_d8 = (code ******)0x0;
          uStack_d0 = (code *)&UNK_142b110c0;
          uStack_c8 = (code ******)0x13;
          uStack_c0 = (code *)0x0;
          uStack_90 = 1;
          uStack_4a = 1;
          func_0x000140d4a330(&uStack_49,&uStack_d8);
        }
        uStack_c8 = uStack_108;
        uStack_c0 = uStack_100;
        uStack_d8 = uStack_118;
        uStack_d0 = uStack_110;
        uStack_4a = 0;
        FUN_1418b2520(&pppppcStack_1c8,&uStack_d8);
        goto code_r0x000140bd934c;
      }
      if (3 < *puRam000000014362c3a8) {
        uStack_168 = &pppppcStack_190;
        uStack_160 = (code *)&UNK_140053920;
        uStack_4a = 1;
        plVar11 = (longlong *)func_0x00014067dd80(&UNK_142b115f8);
        puStack_b8 = (undefined1 *)*plVar11;
        pcStack_b0 = (code *)plVar11[1];
        uStack_8c = (undefined4)plVar11[2];
        uStack_a8 = 4;
        puStack_a0 = &UNK_142b110c0;
        uStack_98 = 0x13;
        puStack_88 = &UNK_142b115d8;
        uStack_80 = 2;
        pppppcStack_78 = (code *****)&uStack_168;
        uStack_70 = 1;
        uStack_68 = 0;
        uStack_d8 = (code ******)0x0;
        uStack_d0 = (code *)&UNK_142b110c0;
        uStack_c8 = (code ******)0x13;
        uStack_c0 = (code *)0x0;
        uStack_90 = 1;
        uStack_4a = 1;
        func_0x000140d4a330(&uStack_49,&uStack_d8);
      }
      ppppppcVar7 = ppppppcVar1;
      ppppppcVar16 = ppppppcVar15;
      if (uStack_118 != (code ******)0x0) {
        func_0x000140613c20(uStack_110,uStack_118,1);
      }
    }
  }
  if (((ulonglong)pppppcStack_e0 & 1) != 0) {
    if (3 < *puRam000000014362c3a8) {
      plVar11 = (longlong *)func_0x00014067dd80(&UNK_142b11348);
      puStack_b8 = (undefined1 *)*plVar11;
      pcStack_b0 = (code *)plVar11[1];
      uStack_8c = (undefined4)plVar11[2];
      uStack_a8 = 4;
      puStack_a0 = &UNK_142b110c0;
      uStack_98 = 0x13;
      puStack_88 = &UNK_142b11338;
      uStack_80 = 1;
      pppppcStack_78 = (code *****)0x8;
      uStack_d8 = (code ******)0x0;
      uStack_70 = 0;
      uStack_68 = 0;
      uStack_d0 = (code *)&UNK_142b110c0;
      uStack_c8 = (code ******)0x13;
      uStack_c0 = (code *)0x0;
      uStack_90 = 1;
      func_0x000140d4a330(&uStack_49,&uStack_d8);
    }
    if (pppppcStack_120 <= pppppcStack_170) {
      FUN_1428d9518(pppppcStack_170,pppppcStack_120,&UNK_142b11360);
      goto code_r0x000140bda313;
    }
    ppppppcVar7 = (code ******)(pppppcStack_128 + (longlong)pppppcStack_170 * 4);
    lVar9 = (longlong)pppppcStack_128[(longlong)pppppcStack_170 * 4] + -0x7fffffffffffffff;
    lVar14 = 0;
    if ((longlong)pppppcStack_128[(longlong)pppppcStack_170 * 4] < -0x7ffffffffffffffe) {
      lVar14 = lVar9;
    }
    if (lVar14 == 0) {
      pppppcStack_1d8 = (code *****)ppppppcVar7;
      if (3 < *puRam000000014362c3a8) {
        uStack_118 = &pppppcStack_1d8;
        uStack_110 = (code *)&UNK_14002e8f0;
        plVar11 = (longlong *)func_0x00014067dd80(&UNK_142b113a8);
        puStack_b8 = (undefined1 *)*plVar11;
        pcStack_b0 = (code *)plVar11[1];
        uStack_8c = (undefined4)plVar11[2];
        uStack_a8 = 4;
        puStack_a0 = &UNK_142b110c0;
        uStack_98 = 0x13;
        puStack_88 = &UNK_142b11398;
        uStack_80 = 1;
        uStack_70 = 1;
        uStack_68 = 0;
        uStack_d8 = (code ******)0x0;
        uStack_d0 = (code *)&UNK_142b110c0;
        uStack_c8 = (code ******)0x13;
        uStack_c0 = (code *)0x0;
        uStack_90 = 1;
        pppppcStack_78 = (code *****)&uStack_118;
        func_0x000140d4a330(&uStack_49,&uStack_d8);
      }
      FUN_1418b2760(&pppppcStack_1c8,pppppcStack_1d8);
    }
    else if (lVar14 == 2) {
      pppppcStack_1e0 = (code *****)(ppppppcVar7 + 1);
      if (3 < *puRam000000014362c3a8) {
        uStack_118 = &pppppcStack_1e0;
        uStack_110 = (code *)&UNK_1400538e0;
        plVar11 = (longlong *)func_0x00014067dd80(&UNK_142b11420);
        puStack_b8 = (undefined1 *)*plVar11;
        pcStack_b0 = (code *)plVar11[1];
        uStack_8c = (undefined4)plVar11[2];
        uStack_a8 = 4;
        puStack_a0 = &UNK_142b110c0;
        uStack_98 = 0x13;
        puStack_88 = &UNK_142b11398;
        uStack_80 = 1;
        uStack_70 = 1;
        uStack_68 = 0;
        uStack_d8 = (code ******)0x0;
        uStack_d0 = (code *)&UNK_142b110c0;
        uStack_c8 = (code ******)0x13;
        uStack_c0 = (code *)0x0;
        uStack_90 = 1;
        pppppcStack_78 = (code *****)&uStack_118;
        func_0x000140d4a330(&uStack_49,&uStack_d8);
      }
      FUN_1418b22e0(&pppppcStack_1c8,pppppcStack_1e0);
    }
    else {
      pppppcStack_210 = (code *****)(ppppppcVar7 + 1);
      uVar10 = FUN_140d76ed0(&uStack_208);
      FUN_140d62810(&uStack_118,uVar10,lVar9,ppppppcVar7[2],ppppppcVar7[3]);
      if (3 < *puRam000000014362c3a8) {
        uStack_168 = &pppppcStack_210;
        uStack_160 = (code *)&UNK_140053920;
        uStack_4b = 1;
        plVar11 = (longlong *)func_0x00014067dd80(&UNK_142b11408);
        puStack_b8 = (undefined1 *)*plVar11;
        pcStack_b0 = (code *)plVar11[1];
        uStack_8c = (undefined4)plVar11[2];
        uStack_a8 = 4;
        puStack_a0 = &UNK_142b110c0;
        uStack_98 = 0x13;
        puStack_88 = &UNK_142b113e8;
        uStack_80 = 2;
        pppppcStack_78 = (code *****)&uStack_168;
        uStack_70 = 1;
        uStack_68 = 0;
        uStack_d8 = (code ******)0x0;
        uStack_d0 = (code *)&UNK_142b110c0;
        uStack_c8 = (code ******)0x13;
        uStack_c0 = (code *)0x0;
        uStack_90 = 1;
        uStack_4b = 1;
        func_0x000140d4a330(&uStack_49,&uStack_d8);
      }
      uStack_c8 = uStack_108;
      uStack_c0 = uStack_100;
      uStack_d8 = uStack_118;
      uStack_d0 = uStack_110;
      uStack_4b = 0;
      FUN_1418b2520(&pppppcStack_1c8,&uStack_d8);
    }
    if ((code ******)pppppcStack_1c8 != (code ******)0x0) {
      uStack_148 = uStack_1a8;
      uStack_144 = uStack_1a4;
      uStack_140 = uStack_1a0;
      uStack_13c = uStack_19c;
      uStack_158 = (undefined4)uStack_1b8;
      uStack_154 = uStack_1b8._4_4_;
      uStack_150 = (code ******)CONCAT44(uStack_1ac,uStack_1b0);
      uStack_168 = (code ******)pppppcStack_1c8;
      uStack_160 = (code *)uStack_1c0;
      uVar13 = 0;
      pcVar12 = (code *)(*(code *)pppppcStack_1c8)(0,&UNK_142b10310);
      if (pcVar12 != (code *)0x0) {
        uVar6 = (*pcVar12)(&uStack_d8);
        uVar17 = uVar6;
        if (uVar6 == 0) {
          uVar17 = (uint)uStack_d8;
        }
        uVar13 = (((ulonglong)uVar17 << 0x20) - (ulonglong)(uVar6 == 0)) + 2;
      }
      uVar18 = uVar13 >> 0x20;
      if ((uVar13 & 1) == 0) {
        uVar18 = 0x400000;
      }
      if ((int)uVar13 == 2) {
        uVar18 = 0x400000;
      }
      auStack_e8._4_4_ = (uint)(uVar18 >> 0x16) & 0x7f;
      auStack_e8._0_4_ = (uint)(uVar18 >> 0xc) & 0x3ff;
      uStack_238 = (undefined *)(CONCAT44(uStack_238._4_4_,(int)uVar18) & 0xffffffff00000fff);
      if (2 < *puRam000000014362c3a8) {
        uStack_118 = (code ******)(auStack_e8 + 4);
        uStack_110 = FUN_140d8d560;
        uStack_108 = (code ******)auStack_e8;
        uStack_100 = FUN_140d8d560;
        puStack_f8 = &uStack_238;
        pcStack_f0 = FUN_140d8d560;
        plVar11 = (longlong *)func_0x00014067dd80(&UNK_142b114a0);
        puStack_b8 = (undefined1 *)*plVar11;
        pcStack_b0 = (code *)plVar11[1];
        uStack_8c = (undefined4)plVar11[2];
        uStack_a8 = 3;
        puStack_a0 = &UNK_142b110c0;
        uStack_98 = 0x13;
        puStack_88 = &UNK_142b11460;
        uStack_80 = 4;
        uStack_70 = 3;
        uStack_68 = 0;
        uStack_d8 = (code ******)0x0;
        uStack_d0 = (code *)&UNK_142b110c0;
        uStack_c8 = (code ******)0x13;
        uStack_c0 = (code *)0x0;
        uStack_90 = 1;
        pppppcStack_78 = (code *****)&uStack_118;
        func_0x000140d4a330(&uStack_49,&uStack_d8);
      }
      puStack_180[5] = CONCAT44(uStack_144,uStack_148);
      puStack_180[6] = CONCAT44(uStack_13c,uStack_140);
      puStack_180[3] = CONCAT44(uStack_154,uStack_158);
      puStack_180[4] = uStack_150;
      *(undefined4 *)(puStack_180 + 1) = (undefined4)uStack_168;
      *(undefined4 *)((longlong)puStack_180 + 0xc) = uStack_168._4_4_;
      *(undefined4 *)(puStack_180 + 2) = (undefined4)uStack_160;
      *(undefined4 *)((longlong)puStack_180 + 0x14) = uStack_160._4_4_;
      *(int *)(puStack_180 + 7) = (int)uVar18;
code_r0x000140bd9f11:
      ppppppcVar7 = (code ******)pppppcStack_128;
      *puStack_180 = 0x18;
      if ((code ******)pppppcStack_120 != (code ******)0x0) {
        lVar9 = 0;
        ppppppcVar16 = (code ******)pppppcStack_120;
        ppppppcVar19 = (code ******)pppppcStack_128;
        do {
          pppppcVar8 = *ppppppcVar19;
          ppppppcVar15 = ppppppcVar19;
          if ((longlong)pppppcVar8 < -0x7ffffffffffffffe) {
            ppppppcVar15 = ppppppcVar7 + lVar9 * 4 + 1;
            pppppcVar8 = ppppppcVar19[1];
          }
          if (pppppcVar8 != (code *****)0x0) {
            func_0x000140613c20(ppppppcVar15[1],pppppcVar8,1);
          }
          ppppppcVar19 = ppppppcVar19 + 4;
          lVar9 = lVar9 + 1;
          ppppppcVar16 = (code ******)((longlong)ppppppcVar16 + -1);
        } while (ppppppcVar16 != (code ******)0x0);
      }
      goto code_r0x000140bda119;
    }
    uStack_108 = (code ******)CONCAT44(uStack_1ac,uStack_1b0);
    uStack_100 = (code *)CONCAT44(uStack_1a4,uStack_1a8);
    uStack_118 = uStack_1c0;
    uStack_110 = (code *)uStack_1b8;
    if (3 < *puRam000000014362c3a8) {
      uStack_160 = FUN_140bd7a70;
      uStack_168 = (code ******)&uStack_118;
      plVar11 = (longlong *)func_0x00014067dd80(&UNK_142b114f8);
      puStack_b8 = (undefined1 *)*plVar11;
      pcStack_b0 = (code *)plVar11[1];
      uStack_8c = (undefined4)plVar11[2];
      uStack_a8 = 4;
      puStack_a0 = &UNK_142b110c0;
      uStack_98 = 0x13;
      puStack_88 = &UNK_142b114e8;
      uStack_80 = 1;
      pppppcStack_78 = (code *****)&uStack_168;
      uStack_70 = 1;
      uStack_68 = 0;
      uStack_d8 = (code ******)0x0;
      uStack_d0 = (code *)&UNK_142b110c0;
      uStack_c8 = (code ******)0x13;
      uStack_c0 = (code *)0x0;
      uStack_90 = 1;
      func_0x000140d4a330(&uStack_49,&uStack_d8);
    }
    func_0x000140bd6e50(&uStack_118);
  }
  ppppppcVar7 = (code ******)pppppcStack_128;
  *puStack_180 = 0x10;
  if ((code ******)pppppcStack_120 != (code ******)0x0) {
    lVar9 = 0;
    ppppppcVar16 = (code ******)pppppcStack_120;
    ppppppcVar19 = (code ******)pppppcStack_128;
    do {
      pppppcVar8 = *ppppppcVar19;
      ppppppcVar15 = ppppppcVar19;
      if ((longlong)pppppcVar8 < -0x7ffffffffffffffe) {
        ppppppcVar15 = ppppppcVar7 + lVar9 * 4 + 1;
        pppppcVar8 = ppppppcVar19[1];
      }
      if (pppppcVar8 != (code *****)0x0) {
        func_0x000140613c20(ppppppcVar15[1],pppppcVar8,1);
      }
      ppppppcVar19 = ppppppcVar19 + 4;
      lVar9 = lVar9 + 1;
      ppppppcVar16 = (code ******)((longlong)ppppppcVar16 + -1);
    } while (ppppppcVar16 != (code ******)0x0);
  }
code_r0x000140bda119:
  if ((code ******)pppppcStack_130 != (code ******)0x0) {
    func_0x000140613c20(ppppppcVar7,(longlong)pppppcStack_130 << 5,8);
  }
  if (CONCAT44(uStack_204,uStack_208) != 0) {
    func_0x000140613c20(CONCAT44(uStack_1fc,uStack_200),CONCAT44(uStack_204,uStack_208),1);
  }
  return puStack_180;
}

