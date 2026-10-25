/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */
void FUN_14129f400(undefined8 *param_1,undefined8 *param_2,undefined8 *param_3,longlong *param_4)
{
  float fVar1;
  longlong lVar2;
  undefined1 auVar3 [16];
  undefined1 uVar4;
  undefined8 uVar5;
  code *pcVar6;
  ushort uVar7;
  uint uVar8;
  uint uVar9;
  float fVar10;
  float fVar11;
  float fVar12;
  undefined8 uVar13;
  undefined8 uVar14;
  undefined8 uVar15;
  uint **ppuVar16;
  uint uVar17;
  longlong *plVar18;
  ulonglong uVar19;
  longlong lVar20;
  uint **ppuVar21;
  longlong lVar22;
  undefined1 (*pauVar23) [16];
  undefined1 (*pauVar24) [16];
  uint uVar25;
  int iVar26;
  longlong lVar27;
  bool bVar28;
  uint uVar29;
  float fVar30;
  float fVar31;
  undefined4 ***in_stack_fffffffffffffd48;
  undefined4 uVar33;
  uint ***pppuVar32;
  undefined4 **ppuStack_2a8;
  undefined8 uStack_2a0;
  undefined1 auStack_298 [16];
  undefined8 uStack_288;
  undefined4 uStack_280;
  undefined4 uStack_27c;
  undefined4 uStack_278;
  undefined4 uStack_274;
  undefined4 uStack_270;
  undefined4 uStack_26c;
  undefined8 uStack_268;
  undefined8 uStack_260;
  undefined8 uStack_258;
  longlong lStack_250;
  int *piStack_248;
  undefined *puStack_240;
  undefined8 uStack_238;
  uint **ppuStack_228;
  code *pcStack_220;
  longlong lStack_218;
  longlong lStack_210;
  longlong *plStack_208;
  undefined8 uStack_200;
  uint **ppuStack_1f8;
  code *pcStack_1f0;
  uint **ppuStack_1e8;
  undefined8 uStack_1e0;
  undefined8 uStack_1d8;
  undefined4 uStack_1d0;
  undefined4 uStack_1cc;
  undefined8 uStack_1c8;
  undefined8 uStack_1c0;
  undefined8 uStack_1b8;
  undefined8 uStack_1b0;
  undefined8 uStack_1a8;
  longlong lStack_1a0;
  int *piStack_198;
  undefined *puStack_190;
  undefined8 uStack_188;
  int *piStack_118;
  undefined4 uStack_110;
  int iStack_10c;
  uint *puStack_100;
  code *pcStack_f8;
  undefined8 uStack_f0;
  undefined4 *puStack_e8;
  int iStack_e0;
  uint uStack_dc;
  longlong *plStack_d8;
  longlong lStack_d0;
  undefined4 uStack_c4;
  undefined8 uStack_c0;
  uStack_c0 = 0xfffffffffffffffe;
  lVar22 = ((undefined8 *)*param_1)[3];
  uStack_c4 = *(undefined4 *)((longlong)param_2 + 0x1c);
  uStack_f0 = *param_2;
  puStack_e8 = (undefined4 *)param_2[2];
  plStack_d8 = param_4;
  if (lVar22 != 0) {
    pauVar23 = *(undefined1 (**) [16])*param_1;
    pauVar24 = pauVar23 + 1;
    auVar3 = *pauVar23;
    uVar25 = ~(uint)(ushort)((ushort)(SUB161(auVar3 >> 7,0) & 1) |
                             (ushort)(SUB161(auVar3 >> 0xf,0) & 1) << 1 |
                             (ushort)(SUB161(auVar3 >> 0x17,0) & 1) << 2 |
                             (ushort)(SUB161(auVar3 >> 0x1f,0) & 1) << 3 |
                             (ushort)(SUB161(auVar3 >> 0x27,0) & 1) << 4 |
                             (ushort)(SUB161(auVar3 >> 0x2f,0) & 1) << 5 |
                             (ushort)(SUB161(auVar3 >> 0x37,0) & 1) << 6 |
                             (ushort)(SUB161(auVar3 >> 0x3f,0) & 1) << 7 |
                             (ushort)(SUB161(auVar3 >> 0x47,0) & 1) << 8 |
                             (ushort)(SUB161(auVar3 >> 0x4f,0) & 1) << 9 |
                             (ushort)(SUB161(auVar3 >> 0x57,0) & 1) << 10 |
                             (ushort)(SUB161(auVar3 >> 0x5f,0) & 1) << 0xb |
                             (ushort)(SUB161(auVar3 >> 0x67,0) & 1) << 0xc |
                             (ushort)(SUB161(auVar3 >> 0x6f,0) & 1) << 0xd |
                             (ushort)(SUB161(auVar3 >> 0x77,0) & 1) << 0xe |
                            (ushort)(byte)(auVar3[0xf] >> 7) << 0xf);
    do {
      if ((short)uVar25 == 0) {
        do {
          auVar3 = *pauVar24;
          uVar7 = (ushort)(SUB161(auVar3 >> 7,0) & 1) | (ushort)(SUB161(auVar3 >> 0xf,0) & 1) << 1 |
                  (ushort)(SUB161(auVar3 >> 0x17,0) & 1) << 2 |
                  (ushort)(SUB161(auVar3 >> 0x1f,0) & 1) << 3 |
                  (ushort)(SUB161(auVar3 >> 0x27,0) & 1) << 4 |
                  (ushort)(SUB161(auVar3 >> 0x2f,0) & 1) << 5 |
                  (ushort)(SUB161(auVar3 >> 0x37,0) & 1) << 6 |
                  (ushort)(SUB161(auVar3 >> 0x3f,0) & 1) << 7 |
                  (ushort)(SUB161(auVar3 >> 0x47,0) & 1) << 8 |
                  (ushort)(SUB161(auVar3 >> 0x4f,0) & 1) << 9 |
                  (ushort)(SUB161(auVar3 >> 0x57,0) & 1) << 10 |
                  (ushort)(SUB161(auVar3 >> 0x5f,0) & 1) << 0xb |
                  (ushort)(SUB161(auVar3 >> 0x67,0) & 1) << 0xc |
                  (ushort)(SUB161(auVar3 >> 0x6f,0) & 1) << 0xd |
                  (ushort)(SUB161(auVar3 >> 0x77,0) & 1) << 0xe |
                  (ushort)(byte)(auVar3[0xf] >> 7) << 0xf;
          pauVar23 = pauVar23 + -0x28;
          pauVar24 = pauVar24 + 1;
        } while (uVar7 == 0xffff);
        uVar25 = ~(uint)uVar7;
        uVar17 = uVar25 & -(uint)uVar7 - 2;
      }
      else {
        uVar17 = uVar25 - 1 & uVar25;
      }
      uVar8 = 0;
      for (; (uVar25 & 1) == 0; uVar25 = uVar25 >> 1 | 0x80000000) {
        uVar8 = uVar8 + 1;
      }
      uVar19 = (ulonglong)uVar8;
      lVar20 = *(longlong *)((longlong)pauVar23 + uVar19 * -0x28 + -8);
      lStack_d0 = lVar22;
      if (lVar20 != 0) {
        lVar22 = *(longlong *)((longlong)pauVar23 + uVar19 * -0x28 + -0x10);
        lVar27 = 0;
        do {
          if (*(longlong *)(*(longlong *)(lVar22 + 0x88 + lVar27) + 0x20) != 0) {
            *puStack_e8 = uStack_c4;
            uVar4 = *(undefined1 *)(lVar22 + 0x90 + lVar27);
            plVar18 = *(longlong **)((longlong)pauVar23 + uVar19 * -0x28 + -0x28);
            uVar5 = *(undefined8 *)((longlong)pauVar23 + uVar19 * -0x28 + -0x20);
            LOCK();
            lVar2 = *plVar18;
            *plVar18 = *plVar18 + 1;
            UNLOCK();
            if (*plVar18 == 0 || SCARRY8(lVar2,1) != *plVar18 < 0) {
                    /* WARNING: Does not return */
              pcVar6 = (code *)invalidInstructionException();
              (*pcVar6)();
            }
            in_stack_fffffffffffffd48 =
                 (undefined4 ***)(*(longlong *)(lVar22 + 0x88 + lVar27) + 0x10);
            FUN_140a37760(uStack_f0,uVar4,plVar18,uVar5,in_stack_fffffffffffffd48);
          }
          lVar27 = lVar27 + 0x98;
        } while (lVar20 * 0x98 - lVar27 != 0);
      }
      lVar22 = lStack_d0 + -1;
      uVar25 = uVar17;
    } while (lVar22 != 0);
  }
  *puStack_e8 = uStack_c4;
  if (*plStack_d8 != 0) {
    *(undefined4 *)plStack_d8[2] = *(undefined4 *)((longlong)plStack_d8 + 0x1c);
  }
  FUN_1415f7190(uStack_f0);
  *puStack_e8 = uStack_c4;
  uVar8 = _UNK_142a8d5c4;
  fVar12 = _UNK_1429edba4;
  uVar17 = _UNK_142925930;
  fVar11 = _UNK_142925904;
  uVar25 = _UNK_1429258f0;
  fVar10 = _UNK_142925870;
  bVar28 = false;
  uVar29 = 0;
  while( true ) {
    uVar33 = (undefined4)((ulonglong)in_stack_fffffffffffffd48 >> 0x20);
    if ((bVar28) || (5 < uVar29)) break;
    bVar28 = 4 < uVar29;
    uVar9 = uVar29 + 1;
    if (bVar28) {
      uVar9 = 5;
    }
    iVar26 = 1 << ((byte)uVar29 & 0x1f);
    iStack_e0 = iVar26;
    uStack_dc = uVar29;
    plVar18 = (longlong *)FUN_1400d9b20(0);
    if (plVar18 == (longlong *)0x0) goto code_r0x0001412a00fd;
    lStack_1a0 = *plVar18;
    piStack_198 = (int *)plVar18[1];
    *plVar18 = *plVar18 + 1;
    uStack_1b0 = _UNK_142ac64d0;
    uStack_1a8 = _UNK_142ac64d8;
    uStack_1c0 = _UNK_142ac64c0;
    uStack_1b8 = _UNK_142ac64c8;
    ppuStack_1f8 = (uint **)0x0;
    pcStack_1f0 = (code *)0x4;
    ppuStack_1e8 = (uint **)0x0;
    uStack_1e0 = 0x8000000000000000;
    puStack_190 = &UNK_142aaf2ea;
    uStack_188 = 7;
    FUN_140928100(&ppuStack_1f8,0,1,iVar26,CONCAT44(uVar33,1));
    uStack_238 = uStack_188;
    piStack_248 = piStack_198;
    puStack_240 = puStack_190;
    uStack_258 = uStack_1a8;
    lStack_250 = lStack_1a0;
    uStack_268 = uStack_1b8;
    uStack_260 = uStack_1b0;
    auStack_298._8_8_ = uStack_1e0;
    auStack_298._0_8_ = ppuStack_1e8;
    uStack_278 = (undefined4)uStack_1c8;
    uStack_274 = uStack_1c8._4_4_;
    uStack_270 = (undefined4)uStack_1c0;
    uStack_26c = uStack_1c0._4_4_;
    uStack_288 = uStack_1d8;
    uStack_280 = uStack_1d0;
    uStack_27c = uStack_1cc;
    ppuStack_2a8 = ppuStack_1f8;
    uStack_2a0 = pcStack_1f0;
    ppuStack_1f8 = (uint **)0x0;
    uStack_1c8 = 0;
    puStack_190 = (undefined *)((ulonglong)(iStack_e0 + 1) << 0x20);
    piStack_198 = &iStack_e0;
    FUN_141302d60(&ppuStack_228,&ppuStack_1f8,&UNK_142c29fc0);
    pcVar6 = pcStack_220;
    if ((uStack_dc != 0) && (lStack_218 != 0)) {
      lVar22 = lStack_218 * 0xc;
      lVar20 = 0;
      do {
        fVar1 = *(float *)(pcVar6 + lVar20);
        fVar30 = (float)func_0x000142923fd0(*(float *)(pcVar6 + lVar20 + 8) * fVar12);
        fVar30 = fVar30 * fVar10;
        *(float *)(pcVar6 + lVar20 + 8) = fVar30;
        uVar29 = -(uint)NAN(fVar1);
        fVar31 = fVar11 - (fVar30 + fVar30) * (fVar30 + fVar30);
        fVar30 = 0.0;
        if (0.0 <= fVar31) {
          fVar30 = fVar31;
        }
        fVar31 = fVar11;
        if (fVar30 <= fVar11) {
          fVar31 = fVar30;
        }
        *(float *)(pcVar6 + lVar20) =
             SQRT(fVar31) * (float)(uVar29 & uVar8 | ~uVar29 & ((uint)fVar1 & uVar25 | uVar17)) *
             fVar10;
        lVar20 = lVar20 + 0xc;
      } while (lVar22 != lVar20);
    }
    ppuStack_1e8 = (uint **)lStack_218;
    ppuStack_1f8 = ppuStack_228;
    pcStack_1f0 = pcStack_220;
    FUN_140a34c90(&ppuStack_2a8,&UNK_142c2a361,0xf,&ppuStack_1f8);
    puStack_100 = &uStack_dc;
    pcStack_f8 = FUN_140d8d560;
    ppuStack_1f8 = (uint **)&UNK_142c2a380;
    pcStack_1f0 = (code *)0x1;
    ppuStack_1e8 = &puStack_100;
    uStack_1e0 = 1;
    uStack_1d8 = 0;
    FUN_140d79f90(&lStack_210,&ppuStack_1f8);
    lStack_d0 = lStack_210;
    plStack_d8 = plStack_208;
    in_stack_fffffffffffffd48 = &ppuStack_2a8;
    FUN_140a371d0(uStack_f0,9,plStack_208,uStack_200,&ppuStack_2a8);
    if (lStack_d0 != 0) {
      func_0x000140613c20(plStack_d8,lStack_d0,1);
    }
    FUN_140888b30(&ppuStack_2a8);
    uVar29 = uVar9;
  }
  *puStack_e8 = uStack_c4;
  bVar28 = false;
  uVar25 = 0;
  while ((uVar33 = (undefined4)((ulonglong)in_stack_fffffffffffffd48 >> 0x20), !bVar28 &&
         (uVar25 < 6))) {
    bVar28 = 4 < uVar25;
    uVar17 = uVar25 + 1;
    if (bVar28) {
      uVar17 = 5;
    }
    iVar26 = 1 << ((byte)uVar25 & 0x1f);
    iStack_e0 = iVar26;
    uStack_dc = uVar25;
    plVar18 = (longlong *)FUN_1400d9b20(0);
    if (plVar18 == (longlong *)0x0) goto code_r0x0001412a00fd;
    lStack_1a0 = *plVar18;
    piStack_198 = (int *)plVar18[1];
    *plVar18 = *plVar18 + 1;
    uStack_1b0 = _UNK_142ac64d0;
    uStack_1a8 = _UNK_142ac64d8;
    uStack_1c0 = _UNK_142ac64c0;
    uStack_1b8 = _UNK_142ac64c8;
    ppuStack_1f8 = (uint **)0x0;
    pcStack_1f0 = (code *)0x4;
    ppuStack_1e8 = (uint **)0x0;
    uStack_1e0 = 0x8000000000000000;
    puStack_190 = &UNK_142aaf3a0;
    uStack_188 = 8;
    FUN_140928350(&ppuStack_1f8,0,6,iVar26,CONCAT44(uVar33,6));
    uStack_238 = uStack_188;
    piStack_248 = piStack_198;
    puStack_240 = puStack_190;
    uStack_258 = uStack_1a8;
    lStack_250 = lStack_1a0;
    uStack_268 = uStack_1b8;
    uStack_260 = uStack_1b0;
    auStack_298._8_8_ = uStack_1e0;
    auStack_298._0_8_ = ppuStack_1e8;
    uStack_278 = (undefined4)uStack_1c8;
    uStack_274 = uStack_1c8._4_4_;
    uStack_270 = (undefined4)uStack_1c0;
    uStack_26c = uStack_1c0._4_4_;
    uStack_288 = uStack_1d8;
    uStack_280 = uStack_1d0;
    uStack_27c = uStack_1cc;
    ppuStack_2a8 = ppuStack_1f8;
    uStack_2a0 = pcStack_1f0;
    iStack_10c = iStack_e0 + 1;
    ppuStack_1f8 = (uint **)0x0;
    uStack_188 = 0;
    uStack_110 = 0;
    piStack_118 = &iStack_e0;
    FUN_1413018b0(&ppuStack_228,&ppuStack_1f8,&UNK_142c29fc0);
    FUN_140a34c90(&ppuStack_2a8,&UNK_142c2a361,0xf,&ppuStack_228);
    pcStack_f8 = FUN_140d8d560;
    ppuStack_1f8 = (uint **)&UNK_142c2a398;
    pcStack_1f0 = (code *)0x1;
    ppuStack_1e8 = &puStack_100;
    uStack_1e0 = 1;
    uStack_1d8 = 0;
    puStack_100 = &uStack_dc;
    FUN_140d79f90(&lStack_210,&ppuStack_1f8);
    lStack_d0 = lStack_210;
    plStack_d8 = plStack_208;
    in_stack_fffffffffffffd48 = &ppuStack_2a8;
    FUN_140a371d0(uStack_f0,0x18,plStack_208,uStack_200,&ppuStack_2a8);
    if (lStack_d0 != 0) {
      func_0x000140613c20(plStack_d8,lStack_d0,1);
    }
    FUN_140888b30(&ppuStack_2a8);
    uVar25 = uVar17;
  }
  *puStack_e8 = uStack_c4;
  plVar18 = (longlong *)FUN_1400d9b20(0);
  if (plVar18 != (longlong *)0x0) {
    lStack_1a0 = *plVar18;
    piStack_198 = (int *)plVar18[1];
    *plVar18 = *plVar18 + 1;
    uVar15 = _UNK_142ac64d8;
    uVar14 = _UNK_142ac64d0;
    uVar13 = _UNK_142ac64c8;
    uVar5 = _UNK_142ac64c0;
    uStack_1b0 = _UNK_142ac64d0;
    uStack_1a8 = _UNK_142ac64d8;
    uStack_1c0 = _UNK_142ac64c0;
    uStack_1b8 = _UNK_142ac64c8;
    ppuStack_1f8 = (uint **)0x0;
    pcStack_1f0 = (code *)0x4;
    ppuStack_1e8 = (uint **)0x0;
    uStack_1e0 = 0x8000000000000000;
    puStack_190 = &UNK_142c2a3a8;
    uStack_188 = 7;
    uVar25 = 0;
    while (ppuVar16 = ppuStack_1e8, uVar25 < 0x20) {
      if (ppuStack_1e8 == ppuStack_1f8) {
        FUN_1416a83a0(&ppuStack_1f8,&UNK_142c2a3c0);
      }
      *(uint *)((longlong)pcStack_1f0 + (longlong)ppuVar16 * 4) = uVar25 | 0x10000;
      ppuVar21 = (uint **)((ulonglong)ppuVar16 | 1);
      if (ppuVar21 == ppuStack_1f8) {
        ppuStack_1e8 = ppuVar21;
        FUN_1416a83a0(&ppuStack_1f8,&UNK_142c2a3d8);
      }
      *(uint *)((longlong)pcStack_1f0 + (longlong)ppuVar21 * 4) = uVar25;
      ppuStack_1e8 = (uint **)((longlong)ppuVar16 + 2);
      if (ppuStack_1e8 == ppuStack_1f8) {
        FUN_1416a83a0(&ppuStack_1f8,&UNK_142c2a3f0);
      }
      uVar17 = uVar25 + 1;
      *(uint *)((longlong)pcStack_1f0 + 8 + (longlong)ppuVar16 * 4) = uVar17;
      ppuStack_1e8 = (uint **)((longlong)ppuVar16 + 3);
      if (ppuStack_1e8 == ppuStack_1f8) {
        FUN_1416a83a0(&ppuStack_1f8,&UNK_142c2a408);
      }
      *(uint *)((longlong)pcStack_1f0 + 0xc + (longlong)ppuVar16 * 4) = uVar17;
      ppuStack_1e8 = (uint **)((longlong)ppuVar16 + 4);
      if (ppuStack_1e8 == ppuStack_1f8) {
        FUN_1416a83a0(&ppuStack_1f8,&UNK_142c2a420);
      }
      *(uint *)((longlong)pcStack_1f0 + 0x10 + (longlong)ppuVar16 * 4) = uVar25 + 0x10001;
      ppuStack_1e8 = (uint **)((longlong)ppuVar16 + 5);
      if (ppuStack_1e8 == ppuStack_1f8) {
        FUN_1416a83a0(&ppuStack_1f8,&UNK_142c2a438);
      }
      *(uint *)((longlong)pcStack_1f0 + 0x14 + (longlong)ppuVar16 * 4) = uVar25 | 0x10000;
      ppuStack_1e8 = (uint **)((longlong)ppuVar16 + 6);
      uVar25 = uVar17;
    }
    pppuVar32 = &ppuStack_1f8;
    FUN_140a371d0(uStack_f0,0x13,&UNK_142c2a3af,0xd,&ppuStack_1f8);
    FUN_140888b30(&ppuStack_1f8);
    *puStack_e8 = uStack_c4;
    puStack_100 = (uint *)CONCAT44(puStack_100._4_4_,4);
    uVar25 = 4;
    while (uVar33 = (undefined4)((ulonglong)pppuVar32 >> 0x20), uVar25 < 0x41) {
      plVar18 = (longlong *)FUN_1400d9b20(0);
      if (plVar18 == (longlong *)0x0) goto code_r0x0001412a00fd;
      lStack_1a0 = *plVar18;
      piStack_198 = (int *)plVar18[1];
      *plVar18 = *plVar18 + 1;
      uStack_1b0 = uVar14;
      uStack_1a8 = uVar15;
      uStack_1c0 = uVar5;
      uStack_1b8 = uVar13;
      ppuStack_1f8 = (uint **)0x0;
      pcStack_1f0 = (code *)0x4;
      ppuStack_1e8 = (uint **)0x0;
      uStack_1e0 = 0x8000000000000000;
      puStack_190 = &UNK_142c2a450;
      uStack_188 = 0xc;
      uVar25 = (uint)puStack_100;
      for (uVar17 = 0; ppuVar16 = ppuStack_1e8, uVar17 < uVar25; uVar17 = uVar17 + 1) {
        if (ppuStack_1e8 == ppuStack_1f8) {
          FUN_1416a83a0(&ppuStack_1f8,&UNK_142c2a480);
        }
        iVar26 = uVar17 * 3;
        *(int *)((longlong)pcStack_1f0 + (longlong)ppuVar16 * 4) = iVar26;
        ppuStack_1e8 = (uint **)((longlong)ppuVar16 + 1);
        if (ppuStack_1e8 == ppuStack_1f8) {
          FUN_1416a83a0(&ppuStack_1f8,&UNK_142c2a498);
        }
        *(int *)((longlong)pcStack_1f0 + 4 + (longlong)ppuVar16 * 4) = iVar26 + 1;
        ppuStack_1e8 = (uint **)((longlong)ppuVar16 + 2);
        if (ppuStack_1e8 == ppuStack_1f8) {
          FUN_1416a83a0(&ppuStack_1f8,&UNK_142c2a4b0);
        }
        *(int *)((longlong)pcStack_1f0 + 8 + (longlong)ppuVar16 * 4) = iVar26 + 2;
        ppuStack_1e8 = (uint **)((longlong)ppuVar16 + 3);
      }
      ppuStack_228 = &puStack_100;
      pcStack_220 = FUN_140d8d560;
      ppuStack_2a8 = (undefined4 **)&UNK_142c2a470;
      uStack_2a0 = 1;
      auStack_298._8_8_ = 1;
      auStack_298._0_8_ = &ppuStack_228;
      uStack_288 = 0;
      FUN_140d79f90(&lStack_210,&ppuStack_2a8);
      lStack_d0 = lStack_210;
      plStack_d8 = plStack_208;
      pppuVar32 = &ppuStack_1f8;
      FUN_140a371d0(uStack_f0,0x1a,plStack_208,uStack_200,&ppuStack_1f8);
      if (lStack_d0 != 0) {
        func_0x000140613c20(plStack_d8,lStack_d0,1);
      }
      puStack_100 = (uint *)CONCAT44(puStack_100._4_4_,(uint)puStack_100 << 1);
      FUN_140888b30();
      uVar25 = (uint)puStack_100;
    }
    *puStack_e8 = uStack_c4;
    plVar18 = (longlong *)FUN_1400d9b20(0);
    if (plVar18 != (longlong *)0x0) {
      lStack_1a0 = *plVar18;
      piStack_198 = (int *)plVar18[1];
      *plVar18 = *plVar18 + 1;
      uStack_1b0 = uVar14;
      uStack_1a8 = uVar15;
      uStack_1c0 = uVar5;
      uStack_1b8 = uVar13;
      ppuStack_1f8 = (uint **)0x0;
      pcStack_1f0 = (code *)0x4;
      ppuStack_1e8 = (uint **)0x0;
      uStack_1e0 = 0x8000000000000000;
      puStack_190 = &UNK_142aaf2ea;
      uStack_188 = 7;
      FUN_140928100(&ppuStack_1f8,0,0xc,0x32,CONCAT44(uVar33,0xc));
      uVar5 = uStack_f0;
      uStack_238 = uStack_188;
      piStack_248 = piStack_198;
      puStack_240 = puStack_190;
      uStack_258 = uStack_1a8;
      lStack_250 = lStack_1a0;
      uStack_268 = uStack_1b8;
      uStack_260 = uStack_1b0;
      uStack_278 = (undefined4)uStack_1c8;
      uStack_274 = uStack_1c8._4_4_;
      uStack_270 = (undefined4)uStack_1c0;
      uStack_26c = uStack_1c0._4_4_;
      uStack_288 = uStack_1d8;
      uStack_280 = uStack_1d0;
      uStack_27c = uStack_1cc;
      auStack_298._8_8_ = uStack_1e0;
      auStack_298._0_8_ = ppuStack_1e8;
      ppuStack_2a8 = ppuStack_1f8;
      uStack_2a0 = pcStack_1f0;
      FUN_140a371d0(uStack_f0,0x15,&UNK_142c2a35c,5,&ppuStack_2a8);
      FUN_140888b30(&ppuStack_2a8);
      *puStack_e8 = uStack_c4;
      *(undefined4 *)param_3[2] = *(undefined4 *)((longlong)param_3 + 0x1c);
      FUN_140ae6b90(uVar5,*param_3);
      return;
    }
  }
code_r0x0001412a00fd:
  FUN_1428d80d0(&UNK_142c3aaa0);
  pcVar6 = (code *)swi(3);
  (*pcVar6)();
  return;
}
