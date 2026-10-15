
/* WARNING: Type propagation algorithm not settling */
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

undefined8 *
FUN_1418ca030(undefined8 *param_1,uint ******param_2,uint ******param_3,uint *******param_4,
             byte param_5)

{
  code *pcVar1;
  code cVar2;
  uint uVar3;
  undefined8 uVar4;
  uint uVar5;
  uint uVar6;
  uint uVar7;
  uint uVar8;
  undefined8 *puVar9;
  uint ******ppppppuVar10;
  ulonglong uVar11;
  longlong lVar12;
  uint *******pppppppuVar13;
  uint uVar14;
  uint *****unaff_RDI;
  int iVar15;
  uint ****ppppuVar16;
  uint ******ppppppuVar17;
  uint *******pppppppuVar18;
  byte bVar19;
  uint uVar20;
  undefined4 uVar21;
  ulonglong uVar22;
  uint *****pppppuVar23;
  undefined8 uStack_320;
  undefined8 uStack_318;
  undefined8 uStack_310;
  undefined8 uStack_308;
  undefined4 auStack_278 [2];
  undefined4 *puStack_270;
  undefined4 uStack_268;
  undefined4 auStack_260 [2];
  undefined8 uStack_258;
  uint *****pppppuStack_250;
  uint ****ppppuStack_248;
  code *pcStack_240;
  undefined8 uStack_238;
  uint *******pppppppuStack_228;
  undefined8 uStack_220;
  undefined *puStack_218;
  uint *******pppppppuStack_210;
  code *pcStack_208;
  undefined4 uStack_200;
  uint uStack_1ec;
  uint uStack_1e8;
  uint uStack_1e4;
  undefined4 uStack_1e0;
  undefined8 uStack_1d8;
  undefined8 uStack_1d0;
  undefined8 uStack_1c8;
  uint *******pppppppuStack_1c0;
  uint ****ppppuStack_1a0;
  code *pcStack_198;
  undefined8 uStack_190;
  undefined4 uStack_184;
  uint *******pppppppuStack_180;
  undefined8 uStack_178;
  undefined *puStack_170;
  undefined8 uStack_168;
  undefined8 uStack_160;
  uint *******pppppppuStack_158;
  uint *******pppppppuStack_150;
  uint *******pppppppuStack_148;
  int iStack_13c;
  uint *****pppppuStack_138;
  uint ****ppppuStack_130;
  undefined8 uStack_128;
  undefined4 uStack_120;
  undefined4 uStack_11c;
  uint uStack_118;
  uint uStack_114;
  uint uStack_110;
  uint uStack_10c;
  undefined4 uStack_108;
  undefined4 uStack_104;
  undefined4 uStack_100;
  undefined4 uStack_fc;
  undefined4 uStack_f8;
  undefined4 uStack_f4;
  undefined8 uStack_f0;
  undefined8 uStack_e8;
  undefined8 uStack_e0;
  uint *******pppppppuStack_d8;
  undefined8 uStack_d0;
  undefined8 uStack_c8;
  uint ******ppppppuStack_b8;
  uint *******pppppppuStack_b0;
  uint ******ppppppuStack_a8;
  uint *******pppppppuStack_a0;
  undefined8 uStack_98;
  uint ******ppppppuStack_90;
  uint *******pppppppuStack_88;
  uint ******ppppppuStack_80;
  undefined8 uStack_78;
  uint ******ppppppuStack_70;
  undefined1 uStack_61;
  undefined8 uStack_60;
  
  uStack_60 = 0xfffffffffffffffe;
  uVar7 = (*(code *)param_2[0xca][4])(param_2[0xcb][0x1a],param_2[0xca][8]);
  uVar6 = uStack_10c;
  uVar5 = uStack_110;
  uVar3 = uStack_114;
  if (uVar7 == 0) {
    uStack_200 = (undefined4)uStack_128;
    iVar15 = (int)((ulonglong)ppppuStack_130 >> 0x20);
    uVar22 = (ulonglong)pppppuStack_138 >> 0x20;
    uStack_1ec = uStack_114;
    uStack_1e8 = uStack_110;
    uStack_1e4 = uStack_10c;
    uStack_1e0 = uStack_108;
    uVar14 = (uint)pppppuStack_138;
    uVar20 = (uint)((ulonglong)pppppuStack_138 >> 0x20);
    uVar8 = (uint)ppppuStack_130;
    pppppppuVar13 = (uint *******)(param_3 + 3);
    pcVar1 = (code *)((longlong)param_3 + 0x1c);
    if (uVar8 == 0xffffffff) {
      uVar8 = *(uint *)pppppppuVar13;
      iVar15 = *(int *)pcVar1;
      ppppuVar16 = (uint ****)(ulonglong)uVar8;
    }
    else {
      *(uint *)pppppppuVar13 = uVar8;
      *(int *)pcVar1 = iVar15;
      ppppuVar16 = ppppuStack_130;
    }
    uVar7 = (uint)ppppuVar16;
    unaff_RDI = pppppuStack_138;
    pppppppuStack_158 = param_4;
    if (uVar8 == 0) {
      pppppuVar23 = (uint *****)0x0;
      pppppppuVar18 = (uint *******)0x2;
      uVar11 = 0;
    }
    else {
      pppppuVar23 = (uint *****)0x0;
      pppppppuVar18 = (uint *******)0x2;
      uVar11 = 0;
      ppppppuStack_70 = param_3;
      uStack_128 = (uint *******)CONCAT44(uStack_128._4_4_,(undefined4)uStack_128);
      if (iVar15 != 0) {
        pppppppuStack_1c0 = pppppppuVar13;
        if (3 < *puRam000000014362c3a8) {
          ppppppuStack_a8 = (uint ******)FUN_140d8d560;
          uStack_98 = FUN_140d8d560;
          pppppppuStack_b0 = pppppppuVar13;
          pppppppuStack_a0 = (uint *******)pcVar1;
          puVar9 = (undefined8 *)func_0x00014067dd80(&UNK_142d084c8);
          uStack_108 = 4;
          uStack_104 = 0;
          uStack_100 = 0x42d08470;
          uStack_fc = 1;
          uStack_f8 = 0x21;
          uStack_f4 = 0;
          uStack_e8 = &UNK_142d084a8;
          uStack_e0 = 2;
          pppppppuStack_d8 = (uint *******)&pppppppuStack_b0;
          uStack_d0 = 2;
          uStack_c8 = 0;
          pppppuStack_138 = (uint *****)0x0;
          ppppuStack_130 = (uint ****)&UNK_142d08470;
          uStack_128._0_4_ = 0x21;
          uStack_128._4_4_ = 0;
          uStack_120 = 0;
          uStack_11c = 0;
          uStack_118 = (uint)*puVar9;
          uStack_114 = (uint)((ulonglong)*puVar9 >> 0x20);
          uStack_110 = (uint)puVar9[1];
          uStack_10c = (uint)((ulonglong)puVar9[1] >> 0x20);
          uStack_f0 = CONCAT44(*(undefined4 *)(puVar9 + 2),1);
          func_0x000140d4a330(&pppppppuStack_180,&pppppuStack_138);
        }
        cVar2 = *(code *)(ppppppuStack_70 + 4);
        if (cVar2 == (code)0x0) {
          ppppppuVar10 = (uint ******)func_0x000140613c10(8,4);
          if (ppppppuVar10 == (uint ******)0x0) goto code_r0x0001418caeae;
          *ppppppuVar10 = (uint *****)0x100000000;
          pppppppuStack_150 = (uint *******)0x2;
          ppppppuStack_b8 = ppppppuVar10;
        }
        else {
          ppppppuVar10 = (uint ******)func_0x000140613c10(4,4);
          if (ppppppuVar10 == (uint ******)0x0) {
            FUN_1428d9000(4,4);
code_r0x0001418caeae:
            FUN_1428d9000(4,8);
            goto code_r0x0001418caebd;
          }
          *(int *)ppppppuVar10 = 2;
          pppppppuStack_150 = (uint *******)0x1;
          ppppppuStack_b8 = ppppppuVar10;
        }
        ppppuStack_1a0 = param_2[0xcb][0x1a];
        pppppppuStack_180 = (uint *******)param_2[0xca][8];
        pppppuStack_138 = param_2[0xca] + 2;
        ppppuStack_130 = (uint ****)&ppppuStack_1a0;
        uStack_128 = (uint *******)&pppppppuStack_180;
        uStack_61 = 1;
        FUN_1418b0890(&pppppppuStack_b0);
        ppppppuVar10 = ppppppuStack_70;
        if (!SBORROW8(0,(longlong)pppppppuStack_b0)) {
          uVar7 = 2;
          if (2 < uVar14) {
            uVar7 = uVar14;
          }
          uVar8 = uVar7;
          if (uVar20 < uVar7) {
            uVar8 = uVar20;
          }
          if (uVar22 == 0) {
            uVar8 = uVar7;
          }
          iStack_13c = *(int *)ppppppuStack_b8;
          pppppppuStack_148 = pppppppuStack_b0;
          lVar12 = 0;
          do {
            if ((longlong)pppppppuStack_a0 << 2 == lVar12) {
              iStack_13c = 2;
              if (cVar2 == (code)0x0) {
                lVar12 = 0;
                goto code_r0x0001418cac20;
              }
              break;
            }
            pcVar1 = (code *)((longlong)ppppppuStack_a8 + lVar12);
            lVar12 = lVar12 + 4;
          } while (iStack_13c != *(int *)pcVar1);
          goto code_r0x0001418ca5e8;
        }
        param_3 = (uint ******)&UNK_142d08651;
        pppppuVar23 = (uint *****)0x29;
        ppppppuVar10 = ppppppuStack_b8;
        pppppppuVar13 = pppppppuStack_150;
        uVar7 = (uint)ppppppuStack_a8;
        goto code_r0x0001418caa6e;
      }
    }
  }
  else {
    param_3 = (uint ******)&UNK_142d08628;
    uVar11 = 0x100000000;
    pppppppuVar18 = (uint *******)0x5;
    pppppuVar23 = (uint *****)0x29;
    uStack_128 = (uint *******)CONCAT44(uStack_128._4_4_,(undefined4)uStack_128);
  }
  goto code_r0x0001418ca0b5;
  while (pcVar1 = (code *)((longlong)ppppppuStack_a8 + lVar12), lVar12 = lVar12 + 4,
        iStack_13c = *(int *)((longlong)ppppppuStack_b8 + 4),
        *(int *)((longlong)ppppppuStack_b8 + 4) != *(int *)pcVar1) {
code_r0x0001418cac20:
    iStack_13c = 2;
    if ((longlong)pppppppuStack_a0 << 2 == lVar12) break;
  }
code_r0x0001418ca5e8:
  ppppppuStack_70 = ppppppuStack_a8;
  func_0x000140613c20();
  if (3 < *puRam000000014362c3a8) {
    pppppppuStack_b0 = (uint *******)&iStack_13c;
    ppppppuStack_a8 = (uint ******)&UNK_140c89300;
    puVar9 = (undefined8 *)func_0x00014067dd80(&UNK_142d08508);
    uStack_108 = 4;
    uStack_104 = 0;
    uStack_100 = 0x42d08470;
    uStack_fc = 1;
    uStack_f8 = 0x21;
    uStack_f4 = 0;
    uStack_e8 = &UNK_142d084f8;
    uStack_e0 = 1;
    uStack_d0 = 1;
    uStack_c8 = 0;
    pppppuStack_138 = (uint *****)0x0;
    ppppuStack_130 = (uint ****)&UNK_142d08470;
    uStack_128._0_4_ = 0x21;
    uStack_128._4_4_ = 0;
    uStack_120 = 0;
    uStack_11c = 0;
    uStack_118 = (uint)*puVar9;
    uStack_114 = (uint)((ulonglong)*puVar9 >> 0x20);
    uStack_110 = (uint)puVar9[1];
    uStack_10c = (uint)((ulonglong)puVar9[1] >> 0x20);
    uStack_f0 = CONCAT44(*(undefined4 *)(puVar9 + 2),1);
    pppppppuStack_d8 = (uint *******)&pppppppuStack_b0;
    func_0x000140d4a330(&pppppppuStack_180);
  }
  pppppppuVar13 = uStack_128;
  if (3 < *puRam000000014362c3a8) {
    pppppppuStack_b0 = (uint *******)&uStack_1ec;
    ppppppuStack_a8 = (uint ******)&UNK_140c87bb0;
    puVar9 = (undefined8 *)func_0x00014067dd80(&UNK_142d08550);
    uStack_108 = 4;
    uStack_104 = 0;
    uStack_100 = 0x42d08470;
    uStack_fc = 1;
    uStack_f8 = 0x21;
    uStack_f4 = 0;
    uStack_e8 = &UNK_142d08540;
    uStack_e0 = 1;
    uStack_d0 = 1;
    uStack_c8 = 0;
    pppppuStack_138 = (uint *****)0x0;
    ppppuStack_130 = (uint ****)&UNK_142d08470;
    uStack_128._0_4_ = 0x21;
    uStack_128._4_4_ = 0;
    uStack_120 = 0;
    uStack_11c = 0;
    uStack_118 = (uint)*puVar9;
    uStack_114 = (uint)((ulonglong)*puVar9 >> 0x20);
    uStack_110 = (uint)puVar9[1];
    uStack_10c = (uint)((ulonglong)puVar9[1] >> 0x20);
    uStack_f0 = CONCAT44(*(undefined4 *)(puVar9 + 2),1);
    pppppppuStack_d8 = (uint *******)&pppppppuStack_b0;
    func_0x000140d4a330(&pppppppuStack_180);
    pppppppuVar13 = (uint *******)CONCAT44(uStack_128._4_4_,(undefined4)uStack_128);
  }
  uStack_128._4_4_ = (undefined4)((ulonglong)pppppppuVar13 >> 0x20);
  unaff_RDI = (uint *****)0x1;
  if (((uVar3 & 1) == 0) && (unaff_RDI = (uint *****)(ulonglong)uVar5, 3 < *puRam000000014362c3a8))
  {
    pppppppuStack_b0 = (uint *******)&uStack_1e8;
    ppppppuStack_a8 = (uint ******)&UNK_140c87bb0;
    uStack_128 = pppppppuVar13;
    puVar9 = (undefined8 *)func_0x00014067dd80(&UNK_142d085b8);
    uStack_108 = 4;
    uStack_104 = 0;
    uStack_100 = 0x42d08470;
    uStack_fc = 1;
    uStack_f8 = 0x21;
    uStack_f4 = 0;
    uStack_e8 = &UNK_142d085a8;
    uStack_e0 = 1;
    uStack_d0 = 1;
    uStack_c8 = 0;
    pppppuStack_138 = (uint *****)0x0;
    ppppuStack_130 = (uint ****)&UNK_142d08470;
    uStack_128._0_4_ = 0x21;
    uStack_128._4_4_ = 0;
    uStack_120 = 0;
    uStack_11c = 0;
    uStack_118 = (uint)*puVar9;
    uStack_114 = (uint)((ulonglong)*puVar9 >> 0x20);
    uStack_110 = (uint)puVar9[1];
    uStack_10c = (uint)((ulonglong)puVar9[1] >> 0x20);
    uStack_f0 = CONCAT44(*(undefined4 *)(puVar9 + 2),1);
    pppppppuStack_d8 = (uint *******)&pppppppuStack_b0;
    func_0x000140d4a330(&pppppppuStack_180);
  }
  uVar21 = 1;
  if ((((uVar6 & 1) == 0) && (uVar21 = 2, (uVar6 & 2) == 0)) && (uVar21 = 4, (uVar6 & 4) == 0)) {
    uVar21 = 8;
    if ((uVar6 & 8) == 0) {
      uVar21 = 1;
    }
  }
  pppppppuStack_d8 = pppppppuStack_158;
  if (param_5 == 2) {
    pppppppuStack_d8 = (uint *******)0x0;
  }
  auStack_260[0] = 0x3b9eae19;
  uStack_258 = 0;
  pppppuStack_250 = (uint *****)0x0;
  if (*ppppppuVar10 == (uint *****)0x1) {
    pppppuStack_250 = ppppppuVar10[1];
  }
  if ((*ppppppuVar10 == (uint *****)0x0) || (param_2[0x10a] == (uint *****)0x0)) {
    ppppuStack_130 = (uint ****)0x0;
  }
  else {
    auStack_278[0] = 0x3b9eae18;
    uStack_268 = 1;
    puStack_270 = auStack_260;
    ppppuStack_130 = (uint ****)auStack_278;
    ppppuVar16 = ppppuStack_130;
    do {
      ppppuVar16 = (uint ****)ppppuVar16[1];
    } while (ppppuVar16 != (uint ****)0x0);
  }
  pppppuStack_138 = (uint *****)CONCAT44(pppppuStack_138._4_4_,0x3b9acde8);
  uStack_128._0_4_ = 0;
  uStack_120 = SUB84(param_2[0xca][8],0);
  uStack_11c = (undefined4)((ulonglong)param_2[0xca][8] >> 0x20);
  uStack_114 = (uint)ppppppuVar10[2];
  uStack_110 = (uint)((ulonglong)ppppppuVar10[2] >> 0x20);
  uStack_10c = (uint)ppppppuVar10[3];
  uStack_108 = (undefined4)((ulonglong)ppppppuVar10[3] >> 0x20);
  uStack_104 = (undefined4)_UNK_142d07f00;
  uStack_100 = (undefined4)((ulonglong)_UNK_142d07f00 >> 0x20);
  uStack_fc = 0;
  uStack_f8 = 0;
  uStack_f0 = 0;
  uStack_e8 = (undefined *)CONCAT44(uVar21,(int)unaff_RDI);
  uStack_e0 = CONCAT44(1,iStack_13c);
  uStack_118 = uVar8;
  uVar7 = (*(code *)param_2[0xd6])(param_2[0xde],&pppppuStack_138,0,&pppppppuStack_b0);
  pppppppuVar13 = pppppppuStack_b0;
  if (uVar7 == 0) {
    pppppppuStack_180 = pppppppuStack_b0;
    FUN_1418b0740(&pppppppuStack_b0,param_2 + 0xd6,&pppppppuStack_180);
    pppppppuVar18 = pppppppuStack_b0;
    if (SBORROW8(0,(longlong)pppppppuStack_b0)) {
code_r0x0001418caebd:
      pppppppuStack_180 = (uint *******)CONCAT44(pppppppuStack_180._4_4_,(uint)ppppppuStack_a8);
      FUN_1428d9760(&UNK_142d08140,0x2b,&pppppppuStack_180,&UNK_142d08120,&UNK_142d085d0);
                    /* WARNING: Does not return */
      pcVar1 = (code *)invalidInstructionException();
      (*pcVar1)();
    }
    ppppppuStack_90 = ppppppuVar10 + 2;
    uStack_98 = (code *)(ppppppuStack_a8 + (longlong)pppppppuStack_a0);
    pppppppuStack_b0 = (uint *******)ppppppuStack_a8;
    pppppppuStack_a0 = pppppppuVar18;
    pppppppuStack_88 = pppppppuStack_1c0;
    ppppppuStack_80 = param_2;
    FUN_1418b5f70(&uStack_1d8,&pppppppuStack_b0,&UNK_142d080e8);
    uVar4 = uStack_1c8;
    if (4 < *puRam000000014362c3a8) {
      uStack_184 = (undefined4)uStack_1c8;
      ppppuStack_1a0 = (uint ****)&uStack_184;
      pcStack_198 = FUN_140d8d560;
      pppppppuStack_b0 = (uint *******)&UNK_142d08600;
      ppppppuStack_a8 = (uint ******)0x1;
      ppppppuStack_90 = (uint ******)0x0;
      pppppppuStack_a0 = (uint *******)&ppppuStack_1a0;
      uStack_98 = (code *)0x1;
      uStack_160 = func_0x00014067dd80(&UNK_142d08610);
      pppppppuStack_180 = (uint *******)&UNK_142d08470;
      uStack_178 = 0x21;
      puStack_170 = &UNK_142d08470;
      uStack_168 = 0x21;
      FUN_1404b1430(&pppppppuStack_b0,5,&pppppppuStack_180);
    }
    ppppppuStack_a8 = (uint ******)0x0;
    pppppppuStack_a0 = (uint *******)uVar4;
    pppppppuStack_b0 = (uint *******)param_2;
    FUN_1418b68f0(&ppppuStack_1a0,&pppppppuStack_b0,&UNK_142d080e8);
    pppppppuStack_158 = pppppppuVar13;
    ppppppuStack_a8 = (uint ******)0x0;
    pppppppuStack_a0 = (uint *******)uVar4;
    pppppppuStack_b0 = (uint *******)param_2;
    FUN_1418b6bf0(&pppppppuStack_180,&pppppppuStack_b0,&UNK_142d080e8);
    pppppppuVar18 = (uint *******)ppppppuVar10[1];
    param_3 = (uint ******)ppppppuVar10[2];
    pppppuVar23 = ppppppuVar10[3];
    uVar3 = *(uint *)(ppppppuVar10 + 4);
    iVar15 = *(int *)((longlong)ppppppuVar10 + 0x24);
    ppppuStack_248 = ppppuStack_1a0;
    pcStack_240 = pcStack_198;
    uStack_238 = uStack_190;
    pppppppuStack_228 = pppppppuStack_180;
    uStack_220 = uStack_178;
    puStack_218 = puStack_170;
    bVar19 = 1;
    if (param_5 != 2) {
      bVar19 = param_5 & 1;
    }
    ppppppuVar17 = (uint ******)*ppppppuVar10;
    if (pppppppuStack_148 != (uint *******)0x0) {
      ppppppuStack_b8 = (uint ******)*ppppppuVar10;
      func_0x000140613c20(ppppppuStack_70,(longlong)pppppppuStack_148 << 2,4);
      ppppppuVar17 = ppppppuStack_b8;
    }
    if (ppppppuVar17 != (uint ******)0x2) {
      param_1[0x13] = uStack_220;
      param_1[0x14] = puStack_218;
      param_1[0x11] = uStack_238;
      param_1[0x12] = pppppppuStack_228;
      param_1[0xf] = ppppuStack_248;
      param_1[0x10] = pcStack_240;
      param_1[0xd] = uStack_1d0;
      param_1[0xe] = uStack_1c8;
      param_1[0xb] = uStack_308;
      param_1[0xc] = uStack_1d8;
      param_1[10] = uStack_310;
      param_1[8] = uStack_320;
      param_1[9] = uStack_318;
      param_1[2] = ppppppuVar17;
      param_1[3] = pppppppuVar18;
      param_1[4] = param_3;
      param_1[5] = pppppuVar23;
      *(uint *)(param_1 + 6) = uVar3;
      *(int *)((longlong)param_1 + 0x34) = iVar15;
      param_1[7] = 2;
      param_1[0x15] = pppppppuStack_158;
      *(undefined1 *)(param_1 + 0x16) = 2;
      param_1[0x17] = 0;
      *(undefined1 *)(param_1 + 0x18) = 0;
      *(byte *)((longlong)param_1 + 0xc1) = bVar19;
      *param_1 = 0;
      param_1[1] = 0;
      return param_1;
    }
    uVar22 = (ulonglong)pppppuVar23 & 0xffffffff00000000;
    uVar11 = (ulonglong)param_3 & 0xffffffff00000000;
    uStack_98._0_4_ = uVar3;
    uStack_98._4_4_ = iVar15;
    goto code_r0x0001418ca0b8;
  }
  param_3 = (uint ******)&UNK_142d0867a;
  pppppuVar23 = (uint *****)0x14;
  ppppppuVar10 = ppppppuStack_70;
  pppppppuVar13 = pppppppuStack_148;
  if (pppppppuStack_148 != (uint *******)0x0) {
code_r0x0001418caa6e:
    unaff_RDI = (uint *****)(ulonglong)uVar7;
    func_0x000140613c20(ppppppuVar10,(longlong)pppppppuVar13 << 2);
  }
  uVar11 = (ulonglong)param_3 & 0xffffffff00000000;
  pppppppuVar18 = (uint *******)0x5;
code_r0x0001418ca0b5:
  uVar22 = 0;
  uStack_98._0_4_ = uVar7;
  uStack_98._4_4_ = (int)unaff_RDI;
code_r0x0001418ca0b8:
  ppppppuStack_a8 = (uint ******)((ulonglong)param_3 & 0xffffffff | uVar11);
  pppppppuStack_a0 = (uint *******)((ulonglong)pppppuVar23 & 0xffffffff | uVar22);
  ppppppuStack_90 = (uint ******)0x2;
  pppppppuStack_88 = (uint *******)uStack_320;
  ppppppuStack_80 = (uint ******)uStack_318;
  uStack_78 = uStack_310;
  if (*puRam000000014362c3a8 == 0) {
    ppppppuStack_90 = (uint ******)0x2;
  }
  else {
    pppppppuStack_210 = (uint *******)&pppppppuStack_b0;
    pcStack_208 = FUN_1418cd1d0;
    pppppppuStack_b0 = pppppppuVar18;
    puVar9 = (undefined8 *)func_0x00014067dd80(&UNK_142d08458);
    uStack_108 = 1;
    uStack_104 = 0;
    uStack_100 = 0x42d08470;
    uStack_fc = 1;
    uStack_f8 = 0x21;
    uStack_f4 = 0;
    uStack_e8 = &UNK_142d08448;
    uStack_e0 = 1;
    pppppppuStack_d8 = (uint *******)&pppppppuStack_210;
    uStack_d0 = 1;
    uStack_c8 = 0;
    pppppuStack_138 = (uint *****)0x0;
    ppppuStack_130 = (uint ****)&UNK_142d08470;
    uStack_128._0_4_ = 0x21;
    uStack_128._4_4_ = 0;
    uStack_120 = 0;
    uStack_11c = 0;
    uStack_118 = (uint)*puVar9;
    uStack_114 = (uint)((ulonglong)*puVar9 >> 0x20);
    uStack_110 = (uint)puVar9[1];
    uStack_10c = (uint)((ulonglong)puVar9[1] >> 0x20);
    uStack_f0 = CONCAT44(*(undefined4 *)(puVar9 + 2),1);
    func_0x000140d4a330(&pppppppuStack_180,&pppppuStack_138);
    pppppppuVar18 = pppppppuStack_b0;
  }
  param_1[8] = uStack_78;
  param_1[6] = pppppppuStack_88;
  param_1[7] = ppppppuStack_80;
  param_1[1] = pppppppuVar18;
  param_1[2] = ppppppuStack_a8;
  param_1[3] = pppppppuStack_a0;
  *(uint *)(param_1 + 4) = (uint)uStack_98;
  *(int *)((longlong)param_1 + 0x24) = uStack_98._4_4_;
  param_1[5] = ppppppuStack_90;
  *param_1 = 1;
  return param_1;
}

