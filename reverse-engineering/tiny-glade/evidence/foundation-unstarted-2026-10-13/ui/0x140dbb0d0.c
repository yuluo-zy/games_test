
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

code * FUN_140dbb0d0(code *param_1,code *param_2)

{
  code *pcVar1;
  code cVar2;
  longlong lVar3;
  longlong lVar4;
  undefined4 uVar5;
  undefined4 uVar6;
  undefined4 uVar7;
  undefined8 uVar8;
  code *pcVar9;
  code *pcVar10;
  ulonglong uVar11;
  undefined8 *puVar12;
  longlong lVar13;
  code *pcVar14;
  undefined4 *puVar15;
  undefined4 *puVar16;
  undefined7 uVar17;
  code *pcVar18;
  code *pcVar19;
  uint uVar20;
  code *pcVar21;
  undefined8 uVar22;
  code *pcVar23;
  code *unaff_RSI;
  uint uVar24;
  longlong lVar25;
  code *pcVar26;
  code *unaff_R14;
  code *pcVar27;
  code *unaff_R15;
  undefined *puVar28;
  bool bVar29;
  undefined4 uStack_270;
  undefined4 uStack_26c;
  undefined4 uStack_268;
  undefined4 uStack_264;
  code *pcStack_260;
  undefined4 uStack_258;
  undefined4 uStack_254;
  undefined4 uStack_250;
  undefined4 uStack_24c;
  code *pcStack_248;
  undefined4 uStack_240;
  undefined4 uStack_23c;
  undefined4 uStack_238;
  undefined4 uStack_234;
  code *pcStack_230;
  undefined4 uStack_228;
  undefined4 uStack_224;
  undefined4 uStack_220;
  undefined4 uStack_21c;
  code *pcStack_218;
  ulonglong uStack_210;
  code *pcStack_208;
  code *pcStack_200;
  undefined4 *puStack_1f8;
  undefined *puStack_1f0;
  ulonglong uStack_1e8;
  code *pcStack_1e0;
  code *pcStack_1d8;
  code *pcStack_1d0;
  code *pcStack_1c8;
  undefined8 *puStack_1c0;
  undefined *puStack_1b8;
  code *pcStack_1b0;
  undefined *puStack_1a8;
  code *pcStack_1a0;
  undefined8 uStack_198;
  undefined8 uStack_190;
  undefined8 uStack_188;
  undefined8 uStack_180;
  undefined8 uStack_178;
  undefined8 uStack_170;
  undefined8 uStack_168;
  undefined8 uStack_160;
  ulonglong uStack_158;
  undefined4 uStack_150;
  undefined4 uStack_14c;
  undefined8 uStack_148;
  undefined8 uStack_140;
  code *pcStack_138;
  undefined8 uStack_128;
  undefined8 uStack_120;
  longlong lStack_118;
  ulonglong uStack_110;
  code *pcStack_108;
  undefined8 uStack_100;
  undefined2 uStack_f8;
  undefined2 uStack_f6;
  undefined2 uStack_f4;
  undefined2 uStack_f2;
  undefined2 uStack_f0;
  undefined4 uStack_ee;
  undefined2 uStack_ea;
  undefined2 uStack_e8;
  undefined6 uStack_e6;
  code *pcStack_e0;
  undefined8 uStack_d8;
  undefined8 uStack_d0;
  undefined8 uStack_c8;
  undefined8 uStack_c0;
  undefined2 uStack_b8;
  undefined2 uStack_b6;
  undefined2 uStack_b4;
  undefined2 uStack_b2;
  undefined2 uStack_b0;
  undefined2 uStack_ae;
  undefined2 uStack_ac;
  undefined2 uStack_aa;
  undefined2 uStack_a8;
  undefined2 uStack_a6;
  undefined4 uStack_a4;
  undefined4 uStack_a0;
  undefined4 uStack_9c;
  ulonglong uStack_98;
  undefined2 uStack_90;
  undefined2 uStack_8e;
  undefined4 uStack_8c;
  undefined2 uStack_88;
  undefined2 uStack_86;
  undefined4 uStack_84;
  undefined4 uStack_80;
  undefined4 uStack_7c;
  code *pcStack_78;
  undefined2 uStack_6c;
  undefined1 uStack_6a;
  undefined1 uStack_61;
  code *pcStack_60;
  code *pcStack_58;
  undefined1 uStack_4e;
  undefined1 uStack_4d;
  undefined1 uStack_4c;
  undefined1 uStack_4b;
  undefined1 uStack_4a;
  undefined1 uStack_49;
  undefined8 uStack_48;
  
  uStack_48 = 0xfffffffffffffffe;
  uVar11 = *(ulonglong *)param_2;
  if (uVar11 == 0x8000000000000004) {
    unaff_RSI = (code *)0x22;
    uVar8 = FUN_140b9d480(&UNK_142b70322);
    if ((longlong)unaff_RSI < 0) {
      uVar22 = 0;
LAB_140dbd27d:
      FUN_1428d8fe3(uVar22,unaff_RSI,&UNK_142ea7758);
      goto LAB_140dbd28f;
    }
    if (unaff_RSI == (code *)0x0) {
      pcVar9 = (code *)0x1;
    }
    else {
      uVar22 = 1;
      pcVar9 = (code *)func_0x000140613c10(unaff_RSI,1);
      if (pcVar9 == (code *)0x0) goto LAB_140dbd27d;
    }
    func_0x000142923c00(pcVar9,uVar8,unaff_RSI);
    puVar12 = (undefined8 *)func_0x000140613c10(0x70,8);
    unaff_R14 = pcVar9;
    if (puVar12 != (undefined8 *)0x0) {
      *puVar12 = 0;
      puVar12[1] = &UNK_142ea7790;
      puVar12[2] = 9;
      *(undefined2 *)(puVar12 + 3) = 0x300;
      *(undefined1 *)((longlong)puVar12 + 0x1a) = 4;
      puVar12[4] = unaff_RSI;
      puVar12[5] = pcVar9;
      puVar12[6] = unaff_RSI;
      uVar8 = CONCAT44(_UNK_142b700ec,_UNK_142b700e8);
      puVar12[7] = CONCAT44(_UNK_142b700e4,_DAT_142b700e0);
      puVar12[8] = uVar8;
      *(undefined2 *)(puVar12 + 9) = 2;
      *(undefined2 *)(puVar12 + 10) = 2;
      *(undefined4 *)(puVar12 + 0xb) = 0;
      *(undefined4 *)((longlong)puVar12 + 0x6c) = 0x101;
      *(undefined8 **)param_1 = puVar12;
      *(undefined **)(param_1 + 8) = &UNK_142ea7a70;
      *(longlong *)(param_1 + 0x10) = 0;
      *(longlong *)(param_1 + 0x20) = 0;
      return param_1;
    }
  }
  else {
    pcStack_60 = param_2;
    pcStack_108 = (code *)func_0x000140613c10(0x33,1);
    lVar3 = _UNK_142b7036c;
    if (pcStack_108 != (code *)0x0) {
      *(longlong *)(pcStack_108 + 0x20) = _DAT_142b70364;
      *(longlong *)(pcStack_108 + 0x28) = lVar3;
      lVar3 = _UNK_142b7035c;
      *(longlong *)(pcStack_108 + 0x10) = _DAT_142b70354;
      *(longlong *)(pcStack_108 + 0x18) = lVar3;
      uVar7 = _UNK_142b70350;
      uVar6 = _UNK_142b7034c;
      uVar5 = _UNK_142b70348;
      *(undefined4 *)pcStack_108 = _DAT_142b70344;
      *(undefined4 *)(pcStack_108 + 4) = uVar5;
      *(undefined4 *)(pcStack_108 + 8) = uVar6;
      *(undefined4 *)(pcStack_108 + 0xc) = uVar7;
      *(undefined4 *)(pcStack_108 + 0x2f) = 0x6e6f6974;
      uStack_49 = 1;
      pcVar21 = (code *)0x27;
      pcVar9 = (code *)FUN_140b9d480(&UNK_142b70218);
      uStack_49 = 1;
      FUN_140d89940(&uStack_d8,pcVar9,pcVar21,&UNK_142b7023f,6);
      pcStack_1d8 = param_1;
      uStack_110 = uVar11;
      pcStack_e0 = pcVar9;
      pcStack_58 = pcVar21;
      if (((ulonglong)uStack_d8 & 1) == 0) {
        pcVar23 = param_1;
        if (uStack_c0._2_1_ == '\0') {
          lVar3 = CONCAT44(uStack_8c,CONCAT22(uStack_8e,uStack_90));
          unaff_R15 = (code *)CONCAT44(uStack_84,CONCAT22(uStack_86,uStack_88));
          uVar11 = (ulonglong)uStack_c0 & 0xff;
          pcVar23 = uStack_d0;
          do {
            if (pcVar23 != (code *)0x0) {
              if (pcVar23 < unaff_R15) {
                if ((char)pcVar23[lVar3] < -0x40) {
LAB_140dbd23a:
                  uStack_49 = 1;
                  FUN_1428d9940(lVar3,unaff_R15,pcVar23,unaff_R15,&UNK_142b70200);
                  goto LAB_140dbd5d1;
                }
              }
              else if (pcVar23 != unaff_R15) goto LAB_140dbd23a;
            }
            if (pcVar23 == unaff_R15) {
              pcVar23 = unaff_R15;
              pcVar10 = unaff_R15;
              if ((uVar11 & 1) == 0) break;
              goto LAB_140dbb694;
            }
            cVar2 = pcVar23[lVar3];
            uVar20 = (uint)(byte)cVar2;
            if ((char)cVar2 < '\0') {
              uVar24 = (byte)cVar2 & 0x1f;
              if ((byte)cVar2 < 0xe0) {
                uVar20 = uVar24 << 6 | (byte)pcVar23[lVar3 + 1] & 0x3f;
              }
              else {
                uVar20 = (byte)pcVar23[lVar3 + 2] & 0x3f | ((byte)pcVar23[lVar3 + 1] & 0x3f) << 6;
                if ((byte)cVar2 < 0xf0) {
                  uVar20 = uVar20 | uVar24 << 0xc;
                }
                else {
                  uVar20 = (byte)pcVar23[lVar3 + 3] & 0x3f | uVar20 << 6 | ((byte)cVar2 & 7) << 0x12
                  ;
                }
              }
            }
            pcVar10 = pcVar23;
            if ((uVar11 & 1) != 0) goto LAB_140dbb694;
            lVar25 = 1;
            if ((0x7f < uVar20) && (lVar25 = 2, 0x7ff < uVar20)) {
              lVar25 = 4 - (ulonglong)(uVar20 < 0x10000);
            }
            uVar11 = uVar11 ^ 1;
            pcVar23 = pcVar23 + lVar25;
          } while( true );
        }
      }
      else {
        lVar3 = CONCAT44(uStack_8c,CONCAT22(uStack_8e,uStack_90));
        pcStack_1a0 = pcStack_78 + -1;
        pcVar14 = (code *)CONCAT44(uStack_84,CONCAT22(uStack_86,uStack_88));
        lVar25 = CONCAT44(uStack_7c,uStack_80);
        pcVar23 = pcStack_78;
        if ((code *)CONCAT44(uStack_9c,uStack_a0) == (code *)0xffffffffffffffff) {
          unaff_R15 = (code *)CONCAT26(uStack_aa,CONCAT24(uStack_ac,CONCAT22(uStack_ae,uStack_b0)));
          pcVar27 = unaff_R15 + (longlong)pcStack_1a0;
          if (pcVar27 < pcVar14) {
            uVar11 = CONCAT26(uStack_b2,CONCAT24(uStack_b4,CONCAT22(uStack_b6,uStack_b8)));
            pcVar18 = uStack_d0 + -1;
            pcVar10 = unaff_R15;
            if (pcVar18 < pcStack_78) {
              pcStack_1c8 = uStack_c0;
              pcStack_1d0 = pcVar18 + lVar25;
LAB_140dbb598:
              if ((uVar11 >> ((ulonglong)(byte)pcVar27[lVar3] & 0x3f) & 1) == 0) {
                unaff_R15 = pcVar10 + (longlong)pcStack_78;
              }
              else {
                lVar13 = -(longlong)pcVar10;
                pcVar27 = uStack_d0;
                do {
                  if (pcStack_78 == pcVar27) {
                    pcVar27 = pcVar18 + (longlong)pcVar10;
                    pcVar19 = uStack_d0;
                    pcVar26 = pcStack_1d0;
                    goto LAB_140dbb5f0;
                  }
                  if (pcVar14 <= pcVar10 + (longlong)pcVar27) goto LAB_140dbd215;
                  pcVar19 = pcVar27 + lVar25;
                  lVar13 = lVar13 + -1;
                  pcVar26 = pcVar10 + lVar3 + (longlong)pcVar27;
                  pcVar27 = pcVar27 + 1;
                } while (*pcVar19 == *pcVar26);
                unaff_R15 = (code *)-lVar13;
              }
              goto LAB_140dbb58b;
            }
            do {
              if ((uVar11 >> ((ulonglong)(byte)pcVar27[lVar3] & 0x3f) & 1) == 0) {
                unaff_R15 = pcVar10 + (longlong)pcStack_78;
              }
              else {
                lVar13 = 0;
                pcVar27 = pcVar10;
                do {
                  if (pcStack_78 <= uStack_d0 + lVar13) {
                    if (uStack_d0 == (code *)0x0) goto LAB_140dbb691;
                    uStack_49 = 1;
                    FUN_1428d9518(pcVar18,pcStack_78,&UNK_142b70150);
                    goto LAB_140dbd5d1;
                  }
                  if (pcVar14 <= uStack_d0 + (longlong)pcVar27) goto LAB_140dbd215;
                  lVar4 = lVar13 + lVar25;
                  unaff_R15 = pcVar27 + 1;
                  lVar13 = lVar13 + 1;
                  pcVar19 = uStack_d0 + lVar3 + (longlong)pcVar27;
                  pcVar27 = unaff_R15;
                } while (uStack_d0[lVar4] == *pcVar19);
              }
              pcVar27 = unaff_R15 + (longlong)pcStack_1a0;
              pcVar10 = unaff_R15;
            } while (pcVar27 < pcVar14);
          }
        }
        else {
          unaff_R15 = (code *)CONCAT26(uStack_aa,CONCAT24(uStack_ac,CONCAT22(uStack_ae,uStack_b0)));
          pcVar10 = pcStack_1a0 + (longlong)unaff_R15;
          if (pcVar10 < pcVar14) {
            pcStack_1d0 = uStack_c0;
            pcStack_1c8 = pcStack_78 + -(longlong)uStack_c0;
            pcVar27 = (code *)CONCAT44(uStack_9c,uStack_a0);
LAB_140dbb480:
            do {
              if ((CONCAT26(uStack_b2,CONCAT24(uStack_b4,CONCAT22(uStack_b6,uStack_b8))) >>
                   ((ulonglong)(byte)pcVar10[lVar3] & 0x3f) & 1) == 0) {
                unaff_R15 = unaff_R15 + (longlong)pcStack_78;
              }
              else {
                pcVar10 = uStack_d0;
                if (uStack_d0 < pcVar27) {
                  pcVar10 = pcVar27;
                }
                pcVar18 = pcVar10;
                do {
                  pcVar19 = uStack_d0;
                  if (pcStack_78 <= pcVar18) goto LAB_140dbb4e0;
                  if (pcVar14 <= unaff_R15 + (longlong)pcVar18) {
                    pcVar9 = pcVar10 + (longlong)unaff_R15;
                    if (pcVar10 + (longlong)unaff_R15 < pcVar14) {
                      pcVar9 = pcVar14;
                    }
                    uStack_49 = 1;
                    FUN_1428d9518(pcVar9,pcVar14,&UNK_142b70180);
                    goto LAB_140dbd5d1;
                  }
                  pcVar19 = pcVar18 + 1;
                  pcVar26 = pcVar18 + lVar25;
                  pcVar1 = unaff_R15 + lVar3 + (longlong)pcVar18;
                  pcVar18 = pcVar19;
                } while (*pcVar26 == *pcVar1);
                unaff_R15 = pcVar19 + ((longlong)unaff_R15 - (longlong)uStack_d0);
              }
              pcVar10 = pcStack_1a0 + (longlong)unaff_R15;
              pcVar27 = (code *)0x0;
            } while (pcVar10 < pcVar14);
          }
        }
      }
LAB_140dbb573:
      pcVar14 = (code *)0x0;
      pcVar27 = pcVar9;
      pcVar10 = unaff_R15;
      goto LAB_140dbb6ae;
    }
LAB_140dbd28f:
    FUN_1428d8fe3(1,0x33,&UNK_142b701e8);
  }
  pcStack_108 = unaff_R14;
  pcStack_58 = unaff_RSI;
  FUN_1428d9000(8,0x70);
  goto LAB_140dbd5d1;
  while( true ) {
    if (pcVar14 <= pcVar27) {
      uStack_49 = 1;
      FUN_1428d9518(pcVar27,pcVar14,&UNK_142b70168);
      goto LAB_140dbd5d1;
    }
    cVar2 = *pcVar26;
    pcVar26 = pcVar26 + -1;
    pcVar1 = pcVar27 + lVar3;
    pcVar27 = pcVar27 + -1;
    if (cVar2 != *pcVar1) break;
LAB_140dbb5f0:
    bVar29 = pcVar19 == (code *)0x0;
    pcVar19 = pcVar19 + -1;
    if (bVar29) goto LAB_140dbb691;
  }
  unaff_R15 = pcVar10 + (longlong)uStack_c0;
LAB_140dbb58b:
  pcVar27 = unaff_R15 + (longlong)pcStack_1a0;
  pcVar10 = unaff_R15;
  if (pcVar14 <= pcVar27) goto LAB_140dbb573;
  goto LAB_140dbb598;
LAB_140dbd215:
  pcVar9 = pcVar10 + (longlong)uStack_d0;
  if (pcVar10 + (longlong)uStack_d0 < pcVar14) {
    pcVar9 = pcVar14;
  }
  uStack_49 = 1;
  FUN_1428d9518(pcVar9,pcVar14,&UNK_142b70180);
  goto LAB_140dbd5d1;
  while( true ) {
    pcVar19 = pcVar19 + -1;
    if (pcStack_78 <= pcVar19) {
      uStack_49 = 1;
      FUN_1428d9518(pcVar19,pcStack_78,&UNK_142b70150);
      goto LAB_140dbd5d1;
    }
    pcVar10 = pcVar19 + (longlong)unaff_R15;
    if (pcVar14 <= pcVar10) {
      uStack_49 = 1;
      FUN_1428d9518(pcVar10,pcVar14,&UNK_142b70168);
      goto LAB_140dbd5d1;
    }
    if (pcVar19[lVar25] != pcVar10[lVar3]) break;
LAB_140dbb4e0:
    pcVar10 = unaff_R15;
    if (pcVar19 <= pcVar27) goto LAB_140dbb691;
  }
  unaff_R15 = unaff_R15 + (longlong)uStack_c0;
  pcVar10 = pcStack_1a0 + (longlong)unaff_R15;
  pcVar27 = pcStack_1c8;
  if (pcVar14 <= pcVar10) goto LAB_140dbb573;
  goto LAB_140dbb480;
LAB_140dbb691:
  pcVar23 = pcStack_78 + (longlong)pcVar10;
LAB_140dbb694:
  pcVar27 = pcVar21 + -(longlong)pcVar23;
  pcVar23 = pcVar23 + (longlong)pcVar9;
  pcVar14 = pcVar9;
LAB_140dbb6ae:
  bVar29 = pcVar14 == (code *)0x0;
  if (bVar29) {
    pcVar10 = pcVar21;
    pcVar14 = pcVar9;
  }
  pcVar9 = (code *)0x1;
  if (bVar29) {
    pcVar23 = (code *)0x1;
    pcVar27 = (code *)0x0;
  }
  if ((longlong)pcVar10 < 0) {
    uVar8 = 0;
LAB_140dbd2cd:
    uStack_49 = 1;
    FUN_1428d8fe3(uVar8,pcVar10,&UNK_142ea7758);
    goto LAB_140dbd5d1;
  }
  if (pcVar10 != (code *)0x0) {
    uVar8 = 1;
    pcVar9 = (code *)func_0x000140613c10(pcVar10,1);
    if (pcVar9 == (code *)0x0) goto LAB_140dbd2cd;
  }
  func_0x000142923c00(pcVar9,pcVar14,pcVar10);
  pcVar21 = (code *)func_0x000140613c10(0x70,8);
  if (pcVar21 == (code *)0x0) {
    pcStack_60 = pcVar9;
    pcStack_58 = pcVar10;
    FUN_1428d9000(8,0x70);
    goto LAB_140dbd5d1;
  }
  *(longlong *)pcVar21 = 0;
  *(undefined **)(pcVar21 + 8) = &UNK_142ea7790;
  *(longlong *)(pcVar21 + 0x10) = 9;
  *(undefined2 *)(pcVar21 + 0x18) = 0x300;
  pcVar21[0x1a] = (code)0x4;
  *(code **)(pcVar21 + 0x20) = pcVar10;
  *(code **)(pcVar21 + 0x28) = pcVar9;
  *(code **)(pcVar21 + 0x30) = pcVar10;
  *(undefined4 *)(pcVar21 + 0x38) = 0;
  *(longlong *)(pcVar21 + 0x40) = 0x3fa6666600000000;
  *(undefined2 *)(pcVar21 + 0x48) = 2;
  *(undefined2 *)(pcVar21 + 0x50) = 2;
  *(undefined4 *)(pcVar21 + 0x58) = 0;
  *(undefined2 *)(pcVar21 + 0x6c) = 1;
  pcVar21[0x6e] = (code)0x0;
  puStack_1a8 = &UNK_142ea7a70;
  pcVar9 = (code *)0x24;
  pcStack_1b0 = pcVar21;
  uStack_198 = (code *)FUN_140b9d480(&UNK_142b70245);
  uStack_128 = &uStack_198;
  uStack_120 = &UNK_140109e90;
  uStack_d8 = (code *)&UNK_142b70270;
  uStack_d0 = (code *)0x1;
  uStack_c8 = (code *)&uStack_128;
  uStack_c0 = (code *)0x1;
  uStack_b8 = 0;
  uStack_b6 = 0;
  uStack_b4 = 0;
  uStack_b2 = 0;
  uStack_190 = pcVar9;
  FUN_140d79f90(&uStack_100,&uStack_d8);
  uStack_128 = (undefined8 *)
               CONCAT44(CONCAT22(uStack_100._6_2_,uStack_100._4_2_),(undefined4)uStack_100);
  uStack_120 = (undefined *)CONCAT44(CONCAT22(uStack_f2,uStack_f4),CONCAT22(uStack_f6,uStack_f8));
  lStack_118 = CONCAT26(uStack_ea,CONCAT42(uStack_ee,uStack_f0));
  FUN_140d7bd90(&uStack_198,&uStack_128);
  uStack_a8 = SUB82(uStack_188,0);
  uStack_a6 = (undefined2)((ulonglong)uStack_188 >> 0x10);
  uStack_a4 = (undefined4)((ulonglong)uStack_188 >> 0x20);
  uStack_b8 = SUB82(uStack_198,0);
  uStack_b6 = (undefined2)((ulonglong)uStack_198 >> 0x10);
  uStack_b4 = (undefined2)((ulonglong)uStack_198 >> 0x20);
  uStack_b2 = (undefined2)((ulonglong)uStack_198 >> 0x30);
  uStack_b0 = SUB82(uStack_190,0);
  uStack_ae = (undefined2)((ulonglong)uStack_190 >> 0x10);
  uStack_ac = (undefined2)((ulonglong)uStack_190 >> 0x20);
  uStack_aa = (undefined2)((ulonglong)uStack_190 >> 0x30);
  uStack_d8 = (code *)0x0;
  uStack_d0 = (code *)&UNK_142ea7790;
  uStack_c8 = (code *)0x9;
  uStack_c0 = (code *)CONCAT53(uStack_c0._3_5_,0x40300);
  uStack_a0 = 0;
  uStack_98 = 0x3fa6666600000000;
  uStack_90 = 2;
  uStack_88 = 2;
  uStack_80 = 0;
  uStack_6c = 1;
  uStack_6a = 0;
  if (uStack_128 != (undefined8 *)0x0) {
    func_0x000140613c20(uStack_120,uStack_128,1);
  }
  FUN_1419d8b90(&uStack_198,&uStack_d8);
  if ((-0x7ffffffffffffffe < (longlong)uStack_180) && (uStack_180 != (code *)0x0)) {
    func_0x000140613c20(uStack_178,uStack_180,1);
  }
  uStack_180 = (code *)0x8000000000000001;
  uStack_178 = (undefined8 *)&UNK_142b70280;
  uStack_170 = (undefined *)0x3e;
  uStack_d8 = uStack_198;
  uStack_d0 = uStack_190;
  uStack_a8 = (undefined2)uStack_168;
  uStack_a6 = (undefined2)((ulonglong)uStack_168 >> 0x10);
  uStack_a4 = uStack_168._4_4_;
  uStack_a0 = (undefined4)uStack_160;
  uStack_9c = uStack_160._4_4_;
  uStack_98 = uStack_158;
  uStack_90 = (undefined2)uStack_150;
  uStack_8e = (undefined2)((uint)uStack_150 >> 0x10);
  uStack_8c = uStack_14c;
  uStack_88 = (undefined2)uStack_148;
  uStack_86 = (undefined2)((ulonglong)uStack_148 >> 0x10);
  uStack_84 = uStack_148._4_4_;
  uStack_80 = (undefined4)uStack_140;
  uStack_7c = uStack_140._4_4_;
  pcStack_78 = pcStack_138;
  uStack_c8 = uStack_188;
  uStack_c0 = (code *)0x8000000000000001;
  uStack_b8 = 0x280;
  uStack_b6 = 0x42b7;
  uStack_b4 = 1;
  uStack_b2 = 0;
  uStack_b0 = 0x3e;
  uStack_ae = 0;
  uStack_ac = 0;
  uStack_aa = 0;
  if (4 < (uint)uStack_198) {
    pcStack_e0 = uStack_188;
    pcStack_58 = uStack_d0;
    if (*(code **)uStack_188 != (code *)0x0) {
      (**(code **)uStack_188)(uStack_190);
    }
    if (*(longlong *)(pcStack_e0 + 8) != 0) {
      func_0x000140613c20(pcStack_58,*(longlong *)(pcStack_e0 + 8),*(longlong *)(pcStack_e0 + 0x10))
      ;
    }
  }
  uStack_d8 = (code *)0x5;
  uStack_d0 = (code *)0x1;
  uStack_c8 = (code *)&UNK_142b702c0;
  uStack_178 = (undefined8 *)CONCAT26(uStack_b2,CONCAT24(uStack_b4,CONCAT22(uStack_b6,uStack_b8)));
  uStack_170 = (undefined *)CONCAT26(uStack_aa,CONCAT24(uStack_ac,CONCAT22(uStack_ae,uStack_b0)));
  uStack_168 = CONCAT44(uStack_a4,CONCAT22(uStack_a6,uStack_a8));
  uStack_160 = CONCAT44(uStack_9c,uStack_a0);
  uStack_150 = CONCAT22(uStack_8e,uStack_90);
  uStack_158 = uStack_98;
  uStack_14c = uStack_8c;
  uStack_198 = (code *)0x5;
  uStack_190 = (code *)0x1;
  uStack_188 = (code *)&UNK_142b702c0;
  uStack_180 = uStack_c0;
  uStack_148 = 0;
  uStack_140 = 0;
  pcStack_138 = pcStack_78;
  pcVar9 = (code *)func_0x000140613c10(0x68,8);
  uVar11 = uStack_110;
  if (pcVar9 == (code *)0x0) {
    FUN_1428d9000(8,0x68);
    goto LAB_140dbd5d1;
  }
  *(code **)(pcVar9 + 0x60) = pcStack_138;
  *(longlong *)(pcVar9 + 0x50) = uStack_148;
  *(longlong *)(pcVar9 + 0x58) = uStack_140;
  *(ulonglong *)(pcVar9 + 0x40) = uStack_158;
  *(longlong *)(pcVar9 + 0x48) = CONCAT44(uStack_14c,uStack_150);
  *(undefined4 *)(pcVar9 + 0x30) = (undefined4)uStack_168;
  *(undefined4 *)(pcVar9 + 0x34) = uStack_168._4_4_;
  *(undefined4 *)(pcVar9 + 0x38) = (undefined4)uStack_160;
  *(undefined4 *)(pcVar9 + 0x3c) = uStack_160._4_4_;
  *(undefined4 *)(pcVar9 + 0x20) = (undefined4)uStack_178;
  *(undefined4 *)(pcVar9 + 0x24) = uStack_178._4_4_;
  *(undefined4 *)(pcVar9 + 0x28) = (undefined4)uStack_170;
  *(undefined4 *)(pcVar9 + 0x2c) = uStack_170._4_4_;
  *(undefined4 *)(pcVar9 + 0x10) = (undefined4)uStack_188;
  *(undefined4 *)(pcVar9 + 0x14) = uStack_188._4_4_;
  *(undefined4 *)(pcVar9 + 0x18) = (undefined4)uStack_180;
  *(undefined4 *)(pcVar9 + 0x1c) = uStack_180._4_4_;
  *(undefined4 *)pcVar9 = (undefined4)uStack_198;
  *(undefined4 *)(pcVar9 + 4) = uStack_198._4_4_;
  *(undefined4 *)(pcVar9 + 8) = (undefined4)uStack_190;
  *(undefined4 *)(pcVar9 + 0xc) = uStack_190._4_4_;
  uStack_d0 = (code *)&UNK_142ea7908;
  uStack_d8 = pcVar9;
  if ((longlong)pcVar27 < 0) {
    uVar8 = 0;
LAB_140dbd327:
    FUN_1428d8fe3(uVar8,pcVar27,&UNK_142ea7758);
    goto LAB_140dbd5d1;
  }
  if (pcVar27 == (code *)0x0) {
    pcVar10 = (code *)0x1;
  }
  else {
    uVar8 = 1;
    pcVar10 = (code *)func_0x000140613c10(pcVar27,1);
    if (pcVar10 == (code *)0x0) goto LAB_140dbd327;
  }
  func_0x000142923c00(pcVar10,pcVar23,pcVar27);
  puVar12 = (undefined8 *)func_0x000140613c10(0x70,8);
  if (puVar12 == (undefined8 *)0x0) {
    pcStack_60 = pcVar10;
    pcStack_58 = pcVar27;
    FUN_1428d9000(8,0x70);
    goto LAB_140dbd5d1;
  }
  *puVar12 = 0;
  puVar12[1] = &UNK_142ea7790;
  puVar12[2] = 9;
  *(undefined2 *)(puVar12 + 3) = 0x300;
  *(undefined1 *)((longlong)puVar12 + 0x1a) = 4;
  puVar12[4] = pcVar27;
  puVar12[5] = pcVar10;
  puVar12[6] = pcVar27;
  *(undefined4 *)(puVar12 + 7) = 0;
  puVar12[8] = 0x3fa6666600000000;
  *(undefined2 *)(puVar12 + 9) = 2;
  *(undefined2 *)(puVar12 + 10) = 2;
  *(undefined4 *)(puVar12 + 0xb) = 0;
  *(undefined2 *)((longlong)puVar12 + 0x6c) = 1;
  *(undefined1 *)((longlong)puVar12 + 0x6e) = 0;
  uStack_190 = (code *)&UNK_142ea7a70;
  uStack_180 = (code *)&UNK_142ea7908;
  uStack_170 = &UNK_142ea7a70;
  uStack_49 = 1;
  uStack_198 = pcVar21;
  uStack_188 = pcVar9;
  uStack_178 = puVar12;
  FUN_1419da2b0(&uStack_d8,&uStack_198);
  puVar15 = (undefined4 *)func_0x000140613c10(0x40,8);
  pcVar9 = pcStack_60;
  if (puVar15 == (undefined4 *)0x0) {
    FUN_1428d9000(8,0x40);
    goto LAB_140dbd5d1;
  }
  puVar15[0xc] = CONCAT22(uStack_a6,uStack_a8);
  puVar15[0xd] = uStack_a4;
  puVar15[0xe] = uStack_a0;
  puVar15[0xf] = uStack_9c;
  puVar15[8] = CONCAT22(uStack_b6,uStack_b8);
  puVar15[9] = CONCAT22(uStack_b2,uStack_b4);
  puVar15[10] = CONCAT22(uStack_ae,uStack_b0);
  puVar15[0xb] = CONCAT22(uStack_aa,uStack_ac);
  puVar15[4] = (undefined4)uStack_c8;
  puVar15[5] = uStack_c8._4_4_;
  puVar15[6] = (undefined4)uStack_c0;
  puVar15[7] = uStack_c0._4_4_;
  *puVar15 = (undefined4)uStack_d8;
  puVar15[1] = uStack_d8._4_4_;
  puVar15[2] = (undefined4)uStack_d0;
  puVar15[3] = uStack_d0._4_4_;
  puStack_1f0 = &UNK_142ea7818;
  puStack_1f8 = puVar15;
  switch(uVar11 ^ 0x8000000000000000) {
  case 0:
    uStack_198 = (code *)&UNK_142b70390;
    uStack_190 = (code *)&UNK_140109e90;
    uStack_d8 = (code *)&UNK_142b703b8;
    uStack_d0 = (code *)0x1;
    uStack_c0 = (code *)0x1;
    uStack_b8 = 0;
    uStack_b6 = 0;
    uStack_b4 = 0;
    uStack_b2 = 0;
    uStack_c8 = (code *)&uStack_198;
    FUN_140d79f90(&uStack_240,&uStack_d8);
    uStack_198 = (code *)CONCAT44(uStack_23c,uStack_240);
    uStack_190 = (code *)CONCAT44(uStack_234,uStack_238);
    uStack_188 = pcStack_230;
    FUN_140d7bd90(&uStack_d8,&uStack_198);
    pcVar10 = uStack_c8;
    pcVar23 = uStack_d0;
    pcVar21 = uStack_d8;
    if (uStack_198 != (code *)0x0) {
      func_0x000140613c20(uStack_190,uStack_198,1);
    }
    pcVar9 = (code *)func_0x000140613c10(0x70,8);
    if (pcVar9 == (code *)0x0) {
      pcStack_60 = pcVar23;
      pcStack_58 = pcVar21;
      FUN_1428d9000(8,0x70);
      goto LAB_140dbd5d1;
    }
    break;
  case 1:
    pcVar9 = pcStack_60 + 8;
    uStack_100._0_4_ = SUB84(pcVar9,0);
    uStack_100._4_2_ = (undefined2)((ulonglong)pcVar9 >> 0x20);
    uStack_100._6_2_ = (undefined2)((ulonglong)pcVar9 >> 0x30);
    uStack_190 = (code *)&UNK_140053920;
    uStack_d8 = (code *)&UNK_142b703e8;
    uStack_d0 = (code *)0x1;
    uStack_c0 = (code *)0x1;
    uStack_b8 = 0;
    uStack_b6 = 0;
    uStack_b4 = 0;
    uStack_b2 = 0;
    uStack_198 = (code *)&uStack_100;
    uStack_c8 = (code *)&uStack_198;
    FUN_140d79f90(&uStack_228,&uStack_d8);
    uStack_198 = (code *)CONCAT44(uStack_224,uStack_228);
    uStack_190 = (code *)CONCAT44(uStack_21c,uStack_220);
    uStack_188 = pcStack_218;
    FUN_140d7bd90(&uStack_d8,&uStack_198);
    pcVar10 = uStack_c8;
    pcVar23 = uStack_d0;
    pcVar21 = uStack_d8;
    if (uStack_198 != (code *)0x0) {
      func_0x000140613c20(uStack_190,uStack_198,1);
    }
    pcVar9 = (code *)func_0x000140613c10(0x70,8);
    if (pcVar9 == (code *)0x0) {
      pcStack_60 = pcVar23;
      pcStack_58 = pcVar21;
      FUN_1428d9000(8,0x70);
      goto LAB_140dbd5d1;
    }
    break;
  case 2:
    pcVar9 = pcStack_60 + 8;
    uStack_100._0_4_ = SUB84(pcVar9,0);
    uStack_100._4_2_ = (undefined2)((ulonglong)pcVar9 >> 0x20);
    uStack_100._6_2_ = (undefined2)((ulonglong)pcVar9 >> 0x30);
    uStack_190 = (code *)&UNK_140053920;
    uStack_d8 = (code *)&UNK_142b70418;
    uStack_d0 = (code *)0x1;
    uStack_c0 = (code *)0x1;
    uStack_b8 = 0;
    uStack_b6 = 0;
    uStack_b4 = 0;
    uStack_b2 = 0;
    uStack_198 = (code *)&uStack_100;
    uStack_c8 = (code *)&uStack_198;
    FUN_140d79f90(&uStack_258,&uStack_d8);
    uStack_198 = (code *)CONCAT44(uStack_254,uStack_258);
    uStack_190 = (code *)CONCAT44(uStack_24c,uStack_250);
    uStack_188 = pcStack_248;
    FUN_140d7bd90(&uStack_d8,&uStack_198);
    pcVar10 = uStack_c8;
    pcVar23 = uStack_d0;
    pcVar21 = uStack_d8;
    if (uStack_198 != (code *)0x0) {
      func_0x000140613c20(uStack_190,uStack_198,1);
    }
    pcVar9 = (code *)func_0x000140613c10(0x70,8);
    if (pcVar9 == (code *)0x0) {
      pcStack_60 = pcVar23;
      pcStack_58 = pcVar21;
      FUN_1428d9000(8,0x70);
      goto LAB_140dbd5d1;
    }
    break;
  case 3:
    pcVar9 = pcStack_60 + 8;
    uStack_100._0_4_ = SUB84(pcVar9,0);
    uStack_100._4_2_ = (undefined2)((ulonglong)pcVar9 >> 0x20);
    uStack_100._6_2_ = (undefined2)((ulonglong)pcVar9 >> 0x30);
    uStack_190 = (code *)&UNK_140533950;
    uStack_188 = (code *)&UNK_142b70428;
    uStack_180 = FUN_140d8d560;
    uStack_d8 = (code *)&UNK_142b70460;
    uStack_d0 = (code *)0x2;
    uStack_c0 = (code *)0x2;
    uStack_b8 = 0;
    uStack_b6 = 0;
    uStack_b4 = 0;
    uStack_b2 = 0;
    uStack_198 = (code *)&uStack_100;
    uStack_c8 = (code *)&uStack_198;
    FUN_140d79f90(&uStack_270,&uStack_d8);
    uStack_198 = (code *)CONCAT44(uStack_26c,uStack_270);
    uStack_190 = (code *)CONCAT44(uStack_264,uStack_268);
    uStack_188 = pcStack_260;
    FUN_140d7bd90(&uStack_d8,&uStack_198);
    pcVar10 = uStack_c8;
    pcVar23 = uStack_d0;
    pcVar21 = uStack_d8;
    if (uStack_198 != (code *)0x0) {
      func_0x000140613c20(uStack_190,uStack_198,1);
    }
    pcVar9 = (code *)func_0x000140613c10(0x70,8);
    if (pcVar9 == (code *)0x0) {
      pcStack_60 = pcVar23;
      pcStack_58 = pcVar21;
      FUN_1428d9000(8,0x70);
      goto LAB_140dbd5d1;
    }
    break;
  default:
    uStack_4e = 1;
    pcVar21 = (code *)0x1f;
    uStack_198 = (code *)FUN_140b9d480(&UNK_142b70480);
    uStack_100._0_4_ = SUB84(&uStack_198,0);
    uStack_100._4_2_ = (undefined2)((ulonglong)&uStack_198 >> 0x20);
    uStack_100._6_2_ = (undefined2)((ulonglong)&uStack_198 >> 0x30);
    uStack_f8 = 0x9e90;
    uStack_f6 = 0x4010;
    uStack_f4 = 1;
    uStack_f2 = 0;
    uStack_d8 = (code *)&UNK_142b704a8;
    uStack_d0 = (code *)0x2;
    uStack_c0 = (code *)0x1;
    uStack_b8 = 0;
    uStack_b6 = 0;
    uStack_b4 = 0;
    uStack_b2 = 0;
    uStack_4e = 1;
    uStack_190 = pcVar21;
    uStack_c8 = (code *)&uStack_100;
    FUN_140d79f90(&uStack_210,&uStack_d8);
    uStack_110 = uStack_210;
    pcStack_e0 = pcStack_208;
    pcStack_58 = pcStack_200;
    uStack_128 = (undefined8 *)0x0;
    uStack_120 = (undefined *)0x8;
    lStack_118 = 0;
    if (*(longlong *)(pcVar9 + 0x10) != 0) {
      uStack_100._6_2_ = 0;
      uStack_f8 = 0;
      uStack_f6 = 0;
      uStack_f4 = 0;
      uStack_f2 = 0;
      uStack_f0 = 0;
      uStack_ee = 0;
      uStack_ea = 0;
      uStack_e8 = 0;
      uStack_d8 = (code *)0x0;
      uStack_d0 = (code *)0x8;
      uStack_c8 = (code *)0x0;
      uStack_c0 = (code *)CONCAT62(uStack_c0._2_6_,2);
      uStack_b8 = 2;
      uStack_b6 = (undefined2)(undefined4)uStack_100;
      uStack_b4 = (undefined2)((uint)(undefined4)uStack_100 >> 0x10);
      uStack_b2 = uStack_100._4_2_;
      uStack_b0 = 0;
      uStack_ae = 0;
      uStack_ac = 0;
      uStack_aa = 0;
      uStack_a8 = 0;
      uStack_a6 = 0;
      uStack_a4 = 0;
      uStack_a0 = 0;
      uStack_9c = 0x7f800000;
      uStack_98 = uStack_98 & 0xffffffffffff0000;
      uStack_4a = 1;
      FUN_1419dafe0(&uStack_198,*(longlong *)(pcVar9 + 8),
                    *(longlong *)(pcVar9 + 8) + *(longlong *)(pcVar9 + 0x10) * 0x18,&uStack_d8);
      uStack_4b = 1;
      pcVar9 = (code *)0x25;
      uVar8 = FUN_140b9d480(&UNK_142b704c8);
      if ((longlong)pcVar9 < 0) {
        uVar22 = 0;
code_r0x000140dbd414:
        uStack_4b = 1;
        FUN_1428d8fe3(uVar22,pcVar9,&UNK_142ea7758);
        goto LAB_140dbd5d1;
      }
      if (pcVar9 == (code *)0x0) {
        pcVar21 = (code *)0x1;
      }
      else {
        uVar22 = 1;
        pcVar21 = (code *)func_0x000140613c10(pcVar9,1);
        if (pcVar21 == (code *)0x0) goto code_r0x000140dbd414;
      }
      func_0x000142923c00(pcVar21,uVar8,pcVar9);
      puVar12 = (undefined8 *)func_0x000140613c10(0x70,8);
      if (puVar12 == (undefined8 *)0x0) {
        pcStack_60 = pcVar21;
        pcStack_58 = pcVar9;
        FUN_1428d9000(8,0x70);
        goto LAB_140dbd5d1;
      }
      *puVar12 = 0;
      puVar12[1] = &UNK_142ea7790;
      puVar12[2] = 9;
      *(undefined2 *)(puVar12 + 3) = 0x300;
      *(undefined1 *)((longlong)puVar12 + 0x1a) = 4;
      puVar12[4] = pcVar9;
      puVar12[5] = pcVar21;
      puVar12[6] = pcVar9;
      *(undefined4 *)(puVar12 + 7) = 0;
      puVar12[8] = 0x3fa6666600000000;
      *(undefined2 *)(puVar12 + 9) = 2;
      *(undefined2 *)(puVar12 + 10) = 2;
      *(undefined4 *)(puVar12 + 0xb) = 1;
      uVar7 = _UNK_1429677ec;
      uVar6 = _UNK_1429677e8;
      uVar5 = _UNK_1429677e4;
      *(undefined4 *)((longlong)puVar12 + 0x5c) = _UNK_1429677e0;
      *(undefined4 *)(puVar12 + 0xc) = uVar5;
      *(undefined4 *)((longlong)puVar12 + 100) = uVar6;
      *(undefined4 *)(puVar12 + 0xd) = uVar7;
      *(undefined2 *)((longlong)puVar12 + 0x6c) = 1;
      *(undefined1 *)((longlong)puVar12 + 0x6e) = 0;
      puStack_1b8 = &UNK_142ea7a70;
      puStack_1c0 = puVar12;
      pcVar9 = (code *)func_0x000140613c10(0x10,4);
      pcStack_1a0 = (code *)CONCAT71(pcStack_1a0._1_7_,pcVar9 == (code *)0x0);
      if (pcVar9 == (code *)0x0) {
        FUN_1428d9000(4,0x10);
        goto LAB_140dbd5d1;
      }
      *(longlong *)pcVar9 = 0x41a0000000000003;
      *(longlong *)(pcVar9 + 8) = 3;
      puStack_1a8 = &UNK_142ea7890;
      uStack_98 = uStack_158;
      uStack_a8 = (undefined2)uStack_168;
      uStack_a6 = (undefined2)((ulonglong)uStack_168 >> 0x10);
      uStack_a4 = uStack_168._4_4_;
      uStack_a0 = (undefined4)uStack_160;
      uStack_9c = uStack_160._4_4_;
      uStack_b8 = SUB82(uStack_178,0);
      uStack_b6 = (undefined2)((ulonglong)uStack_178 >> 0x10);
      uStack_b4 = (undefined2)((ulonglong)uStack_178 >> 0x20);
      uStack_b2 = (undefined2)((ulonglong)uStack_178 >> 0x30);
      uStack_b0 = SUB82(uStack_170,0);
      uStack_ae = (undefined2)((ulonglong)uStack_170 >> 0x10);
      uStack_ac = (undefined2)((ulonglong)uStack_170 >> 0x20);
      uStack_aa = (undefined2)((ulonglong)uStack_170 >> 0x30);
      uStack_c8 = uStack_188;
      uStack_c0 = uStack_180;
      uStack_d8 = uStack_198;
      uStack_d0 = uStack_190;
      pcStack_1b0 = pcVar9;
      puVar16 = (undefined4 *)func_0x000140613c10(0x48,8);
      if (puVar16 == (undefined4 *)0x0) {
        FUN_1428d9000(8,0x48);
        goto LAB_140dbd5d1;
      }
      *(ulonglong *)(puVar16 + 0x10) = uStack_98;
      puVar16[0xc] = CONCAT22(uStack_a6,uStack_a8);
      puVar16[0xd] = uStack_a4;
      puVar16[0xe] = uStack_a0;
      puVar16[0xf] = uStack_9c;
      puVar16[8] = CONCAT22(uStack_b6,uStack_b8);
      puVar16[9] = CONCAT22(uStack_b2,uStack_b4);
      puVar16[10] = CONCAT22(uStack_ae,uStack_b0);
      puVar16[0xb] = CONCAT22(uStack_aa,uStack_ac);
      puVar16[4] = (undefined4)uStack_c8;
      puVar16[5] = uStack_c8._4_4_;
      puVar16[6] = (undefined4)uStack_c0;
      puVar16[7] = uStack_c0._4_4_;
      *puVar16 = (undefined4)uStack_d8;
      puVar16[1] = uStack_d8._4_4_;
      puVar16[2] = (undefined4)uStack_d0;
      puVar16[3] = uStack_d0._4_4_;
      uStack_100._0_4_ = SUB84(pcVar9,0);
      uStack_100._4_2_ = (undefined2)((ulonglong)pcVar9 >> 0x20);
      uStack_100._6_2_ = (undefined2)((ulonglong)pcVar9 >> 0x30);
      uStack_f8 = 0x7890;
      uStack_f6 = 0x42ea;
      uStack_f4 = 1;
      uStack_f2 = 0;
      uStack_f0 = SUB82(puVar16,0);
      uStack_ee = (undefined4)((ulonglong)puVar16 >> 0x10);
      uStack_ea = (undefined2)((ulonglong)puVar16 >> 0x30);
      uStack_e8 = 0x7980;
      uStack_e6 = 0x142ea;
      FUN_1419da130(&uStack_d8,&uStack_100);
      puVar16 = (undefined4 *)func_0x000140613c10(0x40,8);
      if (puVar16 == (undefined4 *)0x0) {
        FUN_1428d9000(8,0x40);
        goto LAB_140dbd5d1;
      }
      puVar16[0xc] = CONCAT22(uStack_a6,uStack_a8);
      puVar16[0xd] = uStack_a4;
      puVar16[0xe] = uStack_a0;
      puVar16[0xf] = uStack_9c;
      puVar16[8] = CONCAT22(uStack_b6,uStack_b8);
      puVar16[9] = CONCAT22(uStack_b2,uStack_b4);
      puVar16[10] = CONCAT22(uStack_ae,uStack_b0);
      puVar16[0xb] = CONCAT22(uStack_aa,uStack_ac);
      puVar16[4] = (undefined4)uStack_c8;
      puVar16[5] = uStack_c8._4_4_;
      puVar16[6] = (undefined4)uStack_c0;
      puVar16[7] = uStack_c0._4_4_;
      *puVar16 = (undefined4)uStack_d8;
      puVar16[1] = uStack_d8._4_4_;
      puVar16[2] = (undefined4)uStack_d0;
      puVar16[3] = uStack_d0._4_4_;
      uStack_100._0_4_ = SUB84(puVar12,0);
      uStack_100._4_2_ = (undefined2)((ulonglong)puVar12 >> 0x20);
      uStack_100._6_2_ = (undefined2)((ulonglong)puVar12 >> 0x30);
      uStack_f8 = 0x7a70;
      uStack_f6 = 0x42ea;
      uStack_f4 = 1;
      uStack_f2 = 0;
      uStack_f0 = SUB82(puVar16,0);
      uStack_ee = (undefined4)((ulonglong)puVar16 >> 0x10);
      uStack_ea = (undefined2)((ulonglong)puVar16 >> 0x30);
      uStack_e8 = 0x7818;
      uStack_e6 = 0x142ea;
      uStack_4b = 0;
      FUN_1419dc6b0(&uStack_d8,&uStack_100);
      pcVar9 = (code *)func_0x000140613c10(0x48,8);
      lVar3 = lStack_118;
      if (pcVar9 == (code *)0x0) {
        FUN_1428d9000(8,0x48);
        goto LAB_140dbd5d1;
      }
      *(ulonglong *)(pcVar9 + 0x40) = uStack_98;
      *(uint *)(pcVar9 + 0x30) = CONCAT22(uStack_a6,uStack_a8);
      *(undefined4 *)(pcVar9 + 0x34) = uStack_a4;
      *(undefined4 *)(pcVar9 + 0x38) = uStack_a0;
      *(undefined4 *)(pcVar9 + 0x3c) = uStack_9c;
      *(uint *)(pcVar9 + 0x20) = CONCAT22(uStack_b6,uStack_b8);
      *(uint *)(pcVar9 + 0x24) = CONCAT22(uStack_b2,uStack_b4);
      *(uint *)(pcVar9 + 0x28) = CONCAT22(uStack_ae,uStack_b0);
      *(uint *)(pcVar9 + 0x2c) = CONCAT22(uStack_aa,uStack_ac);
      *(undefined4 *)(pcVar9 + 0x10) = (undefined4)uStack_c8;
      *(undefined4 *)(pcVar9 + 0x14) = uStack_c8._4_4_;
      *(undefined4 *)(pcVar9 + 0x18) = (undefined4)uStack_c0;
      *(undefined4 *)(pcVar9 + 0x1c) = uStack_c0._4_4_;
      *(undefined4 *)pcVar9 = (undefined4)uStack_d8;
      *(undefined4 *)(pcVar9 + 4) = uStack_d8._4_4_;
      *(undefined4 *)(pcVar9 + 8) = (undefined4)uStack_d0;
      *(undefined4 *)(pcVar9 + 0xc) = uStack_d0._4_4_;
      uStack_d0 = (code *)&UNK_142ea7980;
      uStack_d8 = pcVar9;
      if ((undefined8 *)lStack_118 == uStack_128) {
        FUN_140562630(&uStack_128,&UNK_142b704f0);
      }
      lVar25 = lVar3 * 0x10;
      *(code **)((longlong)uStack_120 + lVar25) = pcVar9;
      *(undefined **)((longlong)uStack_120 + 8 + lVar25) = &UNK_142ea7980;
      lStack_118 = lVar3 + 1;
      pcVar9 = pcStack_60;
    }
    if (*(longlong *)(pcVar9 + 0x28) != 0) {
      uStack_100._6_2_ = 0;
      uStack_f8 = 0;
      uStack_f6 = 0;
      uStack_f4 = 0;
      uStack_f2 = 0;
      uStack_f0 = 0;
      uStack_ee = 0;
      uStack_ea = 0;
      uStack_e8 = 0;
      uStack_d8 = (code *)0x0;
      uStack_d0 = (code *)0x8;
      uStack_c8 = (code *)0x0;
      uStack_c0 = (code *)CONCAT62(uStack_c0._2_6_,2);
      uStack_b8 = 2;
      uStack_b6 = (undefined2)(undefined4)uStack_100;
      uStack_b4 = (undefined2)((uint)(undefined4)uStack_100 >> 0x10);
      uStack_b2 = uStack_100._4_2_;
      uStack_b0 = 0;
      uStack_ae = 0;
      uStack_ac = 0;
      uStack_aa = 0;
      uStack_a8 = 0;
      uStack_a6 = 0;
      uStack_a4 = 0;
      uStack_a0 = 0;
      uStack_9c = 0x7f800000;
      uStack_98 = uStack_98 & 0xffffffffffff0000;
      uStack_4a = 1;
      FUN_1419dafe0(&uStack_198,*(longlong *)(pcVar9 + 0x20),
                    *(longlong *)(pcVar9 + 0x20) + *(longlong *)(pcVar9 + 0x28) * 0x18,&uStack_d8);
      uStack_4c = 1;
      pcVar9 = (code *)0x23;
      uVar8 = FUN_140b9d480(&UNK_142b70508);
      if ((longlong)pcVar9 < 0) {
        uVar22 = 0;
code_r0x000140dbd493:
        uStack_4c = 1;
        FUN_1428d8fe3(uVar22,pcVar9,&UNK_142ea7758);
        goto LAB_140dbd5d1;
      }
      if (pcVar9 == (code *)0x0) {
        pcVar21 = (code *)0x1;
      }
      else {
        uVar22 = 1;
        pcVar21 = (code *)func_0x000140613c10(pcVar9,1);
        if (pcVar21 == (code *)0x0) goto code_r0x000140dbd493;
      }
      func_0x000142923c00(pcVar21,uVar8,pcVar9);
      puVar12 = (undefined8 *)func_0x000140613c10(0x70,8);
      if (puVar12 == (undefined8 *)0x0) {
        pcStack_60 = pcVar21;
        pcStack_58 = pcVar9;
        FUN_1428d9000(8,0x70);
        goto LAB_140dbd5d1;
      }
      *puVar12 = 0;
      puVar12[1] = &UNK_142ea7790;
      puVar12[2] = 9;
      *(undefined2 *)(puVar12 + 3) = 0x300;
      *(undefined1 *)((longlong)puVar12 + 0x1a) = 4;
      puVar12[4] = pcVar9;
      puVar12[5] = pcVar21;
      puVar12[6] = pcVar9;
      *(undefined4 *)(puVar12 + 7) = 0;
      puVar12[8] = 0x3fa6666600000000;
      *(undefined2 *)(puVar12 + 9) = 2;
      *(undefined2 *)(puVar12 + 10) = 2;
      *(undefined4 *)(puVar12 + 0xb) = 1;
      uVar7 = _UNK_1429677ec;
      uVar6 = _UNK_1429677e8;
      uVar5 = _UNK_1429677e4;
      *(undefined4 *)((longlong)puVar12 + 0x5c) = _UNK_1429677e0;
      *(undefined4 *)(puVar12 + 0xc) = uVar5;
      *(undefined4 *)((longlong)puVar12 + 100) = uVar6;
      *(undefined4 *)(puVar12 + 0xd) = uVar7;
      *(undefined2 *)((longlong)puVar12 + 0x6c) = 1;
      *(undefined1 *)((longlong)puVar12 + 0x6e) = 0;
      puStack_1b8 = &UNK_142ea7a70;
      puStack_1c0 = puVar12;
      pcVar9 = (code *)func_0x000140613c10(0x10,4);
      pcStack_1a0 = (code *)CONCAT71(pcStack_1a0._1_7_,pcVar9 == (code *)0x0);
      if (pcVar9 == (code *)0x0) {
        FUN_1428d9000(4,0x10);
        goto LAB_140dbd5d1;
      }
      *(longlong *)pcVar9 = 0x41a0000000000003;
      *(longlong *)(pcVar9 + 8) = 3;
      puStack_1a8 = &UNK_142ea7890;
      uStack_98 = uStack_158;
      uStack_a8 = (undefined2)uStack_168;
      uStack_a6 = (undefined2)((ulonglong)uStack_168 >> 0x10);
      uStack_a4 = uStack_168._4_4_;
      uStack_a0 = (undefined4)uStack_160;
      uStack_9c = uStack_160._4_4_;
      uStack_b8 = SUB82(uStack_178,0);
      uStack_b6 = (undefined2)((ulonglong)uStack_178 >> 0x10);
      uStack_b4 = (undefined2)((ulonglong)uStack_178 >> 0x20);
      uStack_b2 = (undefined2)((ulonglong)uStack_178 >> 0x30);
      uStack_b0 = SUB82(uStack_170,0);
      uStack_ae = (undefined2)((ulonglong)uStack_170 >> 0x10);
      uStack_ac = (undefined2)((ulonglong)uStack_170 >> 0x20);
      uStack_aa = (undefined2)((ulonglong)uStack_170 >> 0x30);
      uStack_c8 = uStack_188;
      uStack_c0 = uStack_180;
      uStack_d8 = uStack_198;
      uStack_d0 = uStack_190;
      pcStack_1b0 = pcVar9;
      puVar16 = (undefined4 *)func_0x000140613c10(0x48,8);
      if (puVar16 == (undefined4 *)0x0) {
        FUN_1428d9000(8,0x48);
        goto LAB_140dbd5d1;
      }
      *(ulonglong *)(puVar16 + 0x10) = uStack_98;
      puVar16[0xc] = CONCAT22(uStack_a6,uStack_a8);
      puVar16[0xd] = uStack_a4;
      puVar16[0xe] = uStack_a0;
      puVar16[0xf] = uStack_9c;
      puVar16[8] = CONCAT22(uStack_b6,uStack_b8);
      puVar16[9] = CONCAT22(uStack_b2,uStack_b4);
      puVar16[10] = CONCAT22(uStack_ae,uStack_b0);
      puVar16[0xb] = CONCAT22(uStack_aa,uStack_ac);
      puVar16[4] = (undefined4)uStack_c8;
      puVar16[5] = uStack_c8._4_4_;
      puVar16[6] = (undefined4)uStack_c0;
      puVar16[7] = uStack_c0._4_4_;
      *puVar16 = (undefined4)uStack_d8;
      puVar16[1] = uStack_d8._4_4_;
      puVar16[2] = (undefined4)uStack_d0;
      puVar16[3] = uStack_d0._4_4_;
      uStack_100._0_4_ = SUB84(pcVar9,0);
      uStack_100._4_2_ = (undefined2)((ulonglong)pcVar9 >> 0x20);
      uStack_100._6_2_ = (undefined2)((ulonglong)pcVar9 >> 0x30);
      uStack_f8 = 0x7890;
      uStack_f6 = 0x42ea;
      uStack_f4 = 1;
      uStack_f2 = 0;
      uStack_f0 = SUB82(puVar16,0);
      uStack_ee = (undefined4)((ulonglong)puVar16 >> 0x10);
      uStack_ea = (undefined2)((ulonglong)puVar16 >> 0x30);
      uStack_e8 = 0x7980;
      uStack_e6 = 0x142ea;
      FUN_1419da130(&uStack_d8,&uStack_100);
      puVar16 = (undefined4 *)func_0x000140613c10(0x40,8);
      if (puVar16 == (undefined4 *)0x0) {
        FUN_1428d9000(8,0x40);
        goto LAB_140dbd5d1;
      }
      puVar16[0xc] = CONCAT22(uStack_a6,uStack_a8);
      puVar16[0xd] = uStack_a4;
      puVar16[0xe] = uStack_a0;
      puVar16[0xf] = uStack_9c;
      puVar16[8] = CONCAT22(uStack_b6,uStack_b8);
      puVar16[9] = CONCAT22(uStack_b2,uStack_b4);
      puVar16[10] = CONCAT22(uStack_ae,uStack_b0);
      puVar16[0xb] = CONCAT22(uStack_aa,uStack_ac);
      puVar16[4] = (undefined4)uStack_c8;
      puVar16[5] = uStack_c8._4_4_;
      puVar16[6] = (undefined4)uStack_c0;
      puVar16[7] = uStack_c0._4_4_;
      *puVar16 = (undefined4)uStack_d8;
      puVar16[1] = uStack_d8._4_4_;
      puVar16[2] = (undefined4)uStack_d0;
      puVar16[3] = uStack_d0._4_4_;
      uStack_100._0_4_ = SUB84(puVar12,0);
      uStack_100._4_2_ = (undefined2)((ulonglong)puVar12 >> 0x20);
      uStack_100._6_2_ = (undefined2)((ulonglong)puVar12 >> 0x30);
      uStack_f8 = 0x7a70;
      uStack_f6 = 0x42ea;
      uStack_f4 = 1;
      uStack_f2 = 0;
      uStack_f0 = SUB82(puVar16,0);
      uStack_ee = (undefined4)((ulonglong)puVar16 >> 0x10);
      uStack_ea = (undefined2)((ulonglong)puVar16 >> 0x30);
      uStack_e8 = 0x7818;
      uStack_e6 = 0x142ea;
      uStack_4c = 0;
      FUN_1419dc6b0(&uStack_d8,&uStack_100);
      pcVar9 = (code *)func_0x000140613c10(0x48,8);
      lVar3 = lStack_118;
      if (pcVar9 == (code *)0x0) {
        FUN_1428d9000(8,0x48);
        goto LAB_140dbd5d1;
      }
      *(ulonglong *)(pcVar9 + 0x40) = uStack_98;
      *(uint *)(pcVar9 + 0x30) = CONCAT22(uStack_a6,uStack_a8);
      *(undefined4 *)(pcVar9 + 0x34) = uStack_a4;
      *(undefined4 *)(pcVar9 + 0x38) = uStack_a0;
      *(undefined4 *)(pcVar9 + 0x3c) = uStack_9c;
      *(uint *)(pcVar9 + 0x20) = CONCAT22(uStack_b6,uStack_b8);
      *(uint *)(pcVar9 + 0x24) = CONCAT22(uStack_b2,uStack_b4);
      *(uint *)(pcVar9 + 0x28) = CONCAT22(uStack_ae,uStack_b0);
      *(uint *)(pcVar9 + 0x2c) = CONCAT22(uStack_aa,uStack_ac);
      *(undefined4 *)(pcVar9 + 0x10) = (undefined4)uStack_c8;
      *(undefined4 *)(pcVar9 + 0x14) = uStack_c8._4_4_;
      *(undefined4 *)(pcVar9 + 0x18) = (undefined4)uStack_c0;
      *(undefined4 *)(pcVar9 + 0x1c) = uStack_c0._4_4_;
      *(undefined4 *)pcVar9 = (undefined4)uStack_d8;
      *(undefined4 *)(pcVar9 + 4) = uStack_d8._4_4_;
      *(undefined4 *)(pcVar9 + 8) = (undefined4)uStack_d0;
      *(undefined4 *)(pcVar9 + 0xc) = uStack_d0._4_4_;
      uStack_d0 = (code *)&UNK_142ea7980;
      uStack_d8 = pcVar9;
      if ((undefined8 *)lStack_118 == uStack_128) {
        FUN_140562630(&uStack_128,&UNK_142b70530);
      }
      lVar25 = lVar3 * 0x10;
      *(code **)((longlong)uStack_120 + lVar25) = pcVar9;
      *(undefined **)((longlong)uStack_120 + 8 + lVar25) = &UNK_142ea7980;
      lStack_118 = lVar3 + 1;
      pcVar9 = pcStack_60;
    }
    if (*(longlong *)(pcVar9 + 0x40) != 0) {
      uStack_100._6_2_ = 0;
      uStack_f8 = 0;
      uStack_f6 = 0;
      uStack_f4 = 0;
      uStack_f2 = 0;
      uStack_f0 = 0;
      uStack_ee = 0;
      uStack_ea = 0;
      uStack_e8 = 0;
      uStack_d8 = (code *)0x0;
      uStack_d0 = (code *)0x8;
      uStack_c8 = (code *)0x0;
      uStack_c0 = (code *)CONCAT62(uStack_c0._2_6_,2);
      uStack_b8 = 2;
      uStack_b6 = (undefined2)(undefined4)uStack_100;
      uStack_b4 = (undefined2)((uint)(undefined4)uStack_100 >> 0x10);
      uStack_b2 = uStack_100._4_2_;
      uStack_b0 = 0;
      uStack_ae = 0;
      uStack_ac = 0;
      uStack_aa = 0;
      uStack_a8 = 0;
      uStack_a6 = 0;
      uStack_a4 = 0;
      uStack_a0 = 0;
      uStack_9c = 0x7f800000;
      uStack_98 = uStack_98 & 0xffffffffffff0000;
      uStack_4a = 1;
      FUN_1419dba60(&uStack_198,*(longlong *)(pcVar9 + 0x38),
                    *(longlong *)(pcVar9 + 0x38) + *(longlong *)(pcVar9 + 0x40) * 0x18,&uStack_d8);
      uStack_4d = 1;
      pcVar9 = (code *)0x2b;
      uVar8 = FUN_140b9d480(&UNK_142b70548);
      if ((longlong)pcVar9 < 0) {
        uVar22 = 0;
code_r0x000140dbd526:
        uStack_4d = 1;
        FUN_1428d8fe3(uVar22,pcVar9,&UNK_142ea7758);
        goto LAB_140dbd5d1;
      }
      if (pcVar9 == (code *)0x0) {
        pcVar21 = (code *)0x1;
      }
      else {
        uVar22 = 1;
        pcVar21 = (code *)func_0x000140613c10(pcVar9,1);
        if (pcVar21 == (code *)0x0) goto code_r0x000140dbd526;
      }
      func_0x000142923c00(pcVar21,uVar8,pcVar9);
      puVar12 = (undefined8 *)func_0x000140613c10(0x70,8);
      if (puVar12 == (undefined8 *)0x0) {
        pcStack_60 = pcVar21;
        pcStack_58 = pcVar9;
        FUN_1428d9000(8,0x70);
        goto LAB_140dbd5d1;
      }
      *puVar12 = 0;
      puVar12[1] = &UNK_142ea7790;
      puVar12[2] = 9;
      *(undefined2 *)(puVar12 + 3) = 0x300;
      *(undefined1 *)((longlong)puVar12 + 0x1a) = 4;
      puVar12[4] = pcVar9;
      puVar12[5] = pcVar21;
      puVar12[6] = pcVar9;
      *(undefined4 *)(puVar12 + 7) = 0;
      puVar12[8] = 0x3fa6666600000000;
      *(undefined2 *)(puVar12 + 9) = 2;
      *(undefined2 *)(puVar12 + 10) = 2;
      *(undefined4 *)(puVar12 + 0xb) = 1;
      uVar7 = _UNK_1429677ec;
      uVar6 = _UNK_1429677e8;
      uVar5 = _UNK_1429677e4;
      *(undefined4 *)((longlong)puVar12 + 0x5c) = _UNK_1429677e0;
      *(undefined4 *)(puVar12 + 0xc) = uVar5;
      *(undefined4 *)((longlong)puVar12 + 100) = uVar6;
      *(undefined4 *)(puVar12 + 0xd) = uVar7;
      *(undefined2 *)((longlong)puVar12 + 0x6c) = 1;
      *(undefined1 *)((longlong)puVar12 + 0x6e) = 0;
      puStack_1b8 = &UNK_142ea7a70;
      puStack_1c0 = puVar12;
      pcVar9 = (code *)func_0x000140613c10(0x10,4);
      pcStack_60 = (code *)CONCAT71(pcStack_60._1_7_,pcVar9 == (code *)0x0);
      if (pcVar9 == (code *)0x0) {
        FUN_1428d9000(4,0x10);
        goto LAB_140dbd5d1;
      }
      *(longlong *)pcVar9 = 0x41a0000000000003;
      *(longlong *)(pcVar9 + 8) = 3;
      puStack_1a8 = &UNK_142ea7890;
      uStack_98 = uStack_158;
      uStack_a8 = (undefined2)uStack_168;
      uStack_a6 = (undefined2)((ulonglong)uStack_168 >> 0x10);
      uStack_a4 = uStack_168._4_4_;
      uStack_a0 = (undefined4)uStack_160;
      uStack_9c = uStack_160._4_4_;
      uStack_b8 = SUB82(uStack_178,0);
      uStack_b6 = (undefined2)((ulonglong)uStack_178 >> 0x10);
      uStack_b4 = (undefined2)((ulonglong)uStack_178 >> 0x20);
      uStack_b2 = (undefined2)((ulonglong)uStack_178 >> 0x30);
      uStack_b0 = SUB82(uStack_170,0);
      uStack_ae = (undefined2)((ulonglong)uStack_170 >> 0x10);
      uStack_ac = (undefined2)((ulonglong)uStack_170 >> 0x20);
      uStack_aa = (undefined2)((ulonglong)uStack_170 >> 0x30);
      uStack_c8 = uStack_188;
      uStack_c0 = uStack_180;
      uStack_d8 = uStack_198;
      uStack_d0 = uStack_190;
      pcStack_1b0 = pcVar9;
      puVar16 = (undefined4 *)func_0x000140613c10(0x48,8);
      if (puVar16 == (undefined4 *)0x0) {
        FUN_1428d9000(8,0x48);
        goto LAB_140dbd5d1;
      }
      *(ulonglong *)(puVar16 + 0x10) = uStack_98;
      puVar16[0xc] = CONCAT22(uStack_a6,uStack_a8);
      puVar16[0xd] = uStack_a4;
      puVar16[0xe] = uStack_a0;
      puVar16[0xf] = uStack_9c;
      puVar16[8] = CONCAT22(uStack_b6,uStack_b8);
      puVar16[9] = CONCAT22(uStack_b2,uStack_b4);
      puVar16[10] = CONCAT22(uStack_ae,uStack_b0);
      puVar16[0xb] = CONCAT22(uStack_aa,uStack_ac);
      puVar16[4] = (undefined4)uStack_c8;
      puVar16[5] = uStack_c8._4_4_;
      puVar16[6] = (undefined4)uStack_c0;
      puVar16[7] = uStack_c0._4_4_;
      *puVar16 = (undefined4)uStack_d8;
      puVar16[1] = uStack_d8._4_4_;
      puVar16[2] = (undefined4)uStack_d0;
      puVar16[3] = uStack_d0._4_4_;
      uStack_100._0_4_ = SUB84(pcVar9,0);
      uStack_100._4_2_ = (undefined2)((ulonglong)pcVar9 >> 0x20);
      uStack_100._6_2_ = (undefined2)((ulonglong)pcVar9 >> 0x30);
      uStack_f8 = 0x7890;
      uStack_f6 = 0x42ea;
      uStack_f4 = 1;
      uStack_f2 = 0;
      uStack_f0 = SUB82(puVar16,0);
      uStack_ee = (undefined4)((ulonglong)puVar16 >> 0x10);
      uStack_ea = (undefined2)((ulonglong)puVar16 >> 0x30);
      uStack_e8 = 0x7980;
      uStack_e6 = 0x142ea;
      FUN_1419da130(&uStack_d8,&uStack_100);
      puVar16 = (undefined4 *)func_0x000140613c10(0x40,8);
      if (puVar16 == (undefined4 *)0x0) {
        FUN_1428d9000(8,0x40);
        goto LAB_140dbd5d1;
      }
      puVar16[0xc] = CONCAT22(uStack_a6,uStack_a8);
      puVar16[0xd] = uStack_a4;
      puVar16[0xe] = uStack_a0;
      puVar16[0xf] = uStack_9c;
      puVar16[8] = CONCAT22(uStack_b6,uStack_b8);
      puVar16[9] = CONCAT22(uStack_b2,uStack_b4);
      puVar16[10] = CONCAT22(uStack_ae,uStack_b0);
      puVar16[0xb] = CONCAT22(uStack_aa,uStack_ac);
      puVar16[4] = (undefined4)uStack_c8;
      puVar16[5] = uStack_c8._4_4_;
      puVar16[6] = (undefined4)uStack_c0;
      puVar16[7] = uStack_c0._4_4_;
      *puVar16 = (undefined4)uStack_d8;
      puVar16[1] = uStack_d8._4_4_;
      puVar16[2] = (undefined4)uStack_d0;
      puVar16[3] = uStack_d0._4_4_;
      uStack_100._0_4_ = SUB84(puVar12,0);
      uStack_100._4_2_ = (undefined2)((ulonglong)puVar12 >> 0x20);
      uStack_100._6_2_ = (undefined2)((ulonglong)puVar12 >> 0x30);
      uStack_f8 = 0x7a70;
      uStack_f6 = 0x42ea;
      uStack_f4 = 1;
      uStack_f2 = 0;
      uStack_f0 = SUB82(puVar16,0);
      uStack_ee = (undefined4)((ulonglong)puVar16 >> 0x10);
      uStack_ea = (undefined2)((ulonglong)puVar16 >> 0x30);
      uStack_e8 = 0x7818;
      uStack_e6 = 0x142ea;
      uStack_4d = 0;
      FUN_1419dc6b0(&uStack_d8,&uStack_100);
      pcVar9 = (code *)func_0x000140613c10(0x48,8);
      lVar3 = lStack_118;
      if (pcVar9 == (code *)0x0) {
        FUN_1428d9000(8,0x48);
        goto LAB_140dbd5d1;
      }
      *(ulonglong *)(pcVar9 + 0x40) = uStack_98;
      *(uint *)(pcVar9 + 0x30) = CONCAT22(uStack_a6,uStack_a8);
      *(undefined4 *)(pcVar9 + 0x34) = uStack_a4;
      *(undefined4 *)(pcVar9 + 0x38) = uStack_a0;
      *(undefined4 *)(pcVar9 + 0x3c) = uStack_9c;
      *(uint *)(pcVar9 + 0x20) = CONCAT22(uStack_b6,uStack_b8);
      *(uint *)(pcVar9 + 0x24) = CONCAT22(uStack_b2,uStack_b4);
      *(uint *)(pcVar9 + 0x28) = CONCAT22(uStack_ae,uStack_b0);
      *(uint *)(pcVar9 + 0x2c) = CONCAT22(uStack_aa,uStack_ac);
      *(undefined4 *)(pcVar9 + 0x10) = (undefined4)uStack_c8;
      *(undefined4 *)(pcVar9 + 0x14) = uStack_c8._4_4_;
      *(undefined4 *)(pcVar9 + 0x18) = (undefined4)uStack_c0;
      *(undefined4 *)(pcVar9 + 0x1c) = uStack_c0._4_4_;
      *(undefined4 *)pcVar9 = (undefined4)uStack_d8;
      *(undefined4 *)(pcVar9 + 4) = uStack_d8._4_4_;
      *(undefined4 *)(pcVar9 + 8) = (undefined4)uStack_d0;
      *(undefined4 *)(pcVar9 + 0xc) = uStack_d0._4_4_;
      uStack_d0 = (code *)&UNK_142ea7980;
      uStack_d8 = pcVar9;
      if ((undefined8 *)lStack_118 == uStack_128) {
        FUN_140562630(&uStack_128,&UNK_142b70578);
      }
      lVar25 = lVar3 * 0x10;
      *(code **)((longlong)uStack_120 + lVar25) = pcVar9;
      *(undefined **)((longlong)uStack_120 + 8 + lVar25) = &UNK_142ea7980;
      lStack_118 = lVar3 + 1;
    }
    uStack_188 = (code *)lStack_118;
    uStack_198 = (code *)uStack_128;
    uStack_190 = (code *)uStack_120;
    FUN_1419dd360(&uStack_d8,&uStack_198);
    uStack_a0 = 0x41a00000;
    uStack_198 = uStack_d8;
    uStack_190 = uStack_d0;
    uStack_188 = uStack_c8;
    uStack_180 = uStack_c0;
    uStack_178 = (undefined8 *)CONCAT44(CONCAT22(uStack_b2,uStack_b4),CONCAT22(uStack_b6,uStack_b8))
    ;
    uStack_170 = (undefined *)CONCAT44(CONCAT22(uStack_aa,uStack_ac),CONCAT22(uStack_ae,uStack_b0));
    uStack_158 = uStack_98;
    uStack_168 = CONCAT44(uStack_a4,CONCAT22(uStack_a6,uStack_a8));
    uStack_160 = CONCAT44(uStack_9c,0x41a00000);
    pcVar9 = (code *)func_0x000140613c10(0x48,8);
    if (pcVar9 == (code *)0x0) {
      FUN_1428d9000(8,0x48);
      goto LAB_140dbd5d1;
    }
    *(ulonglong *)(pcVar9 + 0x40) = uStack_158;
    *(longlong *)(pcVar9 + 0x30) = uStack_168;
    *(longlong *)(pcVar9 + 0x38) = uStack_160;
    *(undefined8 **)(pcVar9 + 0x20) = uStack_178;
    *(undefined **)(pcVar9 + 0x28) = uStack_170;
    *(code **)(pcVar9 + 0x10) = uStack_188;
    *(code **)(pcVar9 + 0x18) = uStack_180;
    *(undefined4 *)pcVar9 = (undefined4)uStack_198;
    *(undefined4 *)(pcVar9 + 4) = uStack_198._4_4_;
    *(undefined4 *)(pcVar9 + 8) = (undefined4)uStack_190;
    *(undefined4 *)(pcVar9 + 0xc) = uStack_190._4_4_;
    puVar28 = &UNK_142ea7980;
    uStack_d0 = (code *)&UNK_142ea7980;
    uVar17 = (undefined7)(uStack_158 >> 8);
    uStack_d8 = pcVar9;
    if ((longlong)pcStack_58 < 0) {
      pcStack_60 = (code *)CONCAT44(pcStack_60._4_4_,(int)CONCAT71(uVar17,1));
      uVar8 = 0;
      goto code_r0x000140dbd3d4;
    }
    if (pcStack_58 != (code *)0x0) {
      pcStack_60 = (code *)CONCAT44(pcStack_60._4_4_,(int)CONCAT71(uVar17,1));
      goto code_r0x000140dbc3f1;
    }
    pcStack_60 = (code *)CONCAT44(pcStack_60._4_4_,(int)CONCAT71(uVar17,1));
    pcVar21 = (code *)0x1;
    pcStack_58 = (code *)0x0;
    goto code_r0x000140dbc422;
  }
  *(longlong *)pcVar9 = 0;
  *(undefined **)(pcVar9 + 8) = &UNK_142ea7790;
  *(longlong *)(pcVar9 + 0x10) = 9;
  *(undefined2 *)(pcVar9 + 0x18) = 0x300;
  pcVar9[0x1a] = (code)0x4;
  *(code **)(pcVar9 + 0x20) = pcVar21;
  *(code **)(pcVar9 + 0x28) = pcVar23;
  *(code **)(pcVar9 + 0x30) = pcVar10;
  *(undefined4 *)(pcVar9 + 0x38) = 0;
  *(longlong *)(pcVar9 + 0x40) = 0x3fa6666600000000;
  *(undefined2 *)(pcVar9 + 0x48) = 2;
  *(undefined2 *)(pcVar9 + 0x50) = 2;
  *(undefined4 *)(pcVar9 + 0x58) = 0;
  *(undefined2 *)(pcVar9 + 0x6c) = 1;
  pcVar9[0x6e] = (code)0x0;
  puVar28 = &UNK_142ea7a70;
  uStack_d0 = (code *)&UNK_142ea7a70;
  uStack_110 = 0x33;
  pcStack_60 = (code *)((ulonglong)pcStack_60 & 0xffffffff00000000);
  pcStack_58 = (code *)0x33;
  pcStack_e0 = pcStack_108;
code_r0x000140dbc3f1:
  uVar8 = 1;
  uStack_d8 = pcVar9;
  pcVar21 = (code *)func_0x000140613c10(pcStack_58,1);
  if (pcVar21 == (code *)0x0) {
code_r0x000140dbd3d4:
    uStack_61 = SUB81(pcStack_60,0);
    pcStack_1e0 = pcStack_e0;
    uStack_1e8 = uStack_110;
    FUN_1428d8fe3(uVar8,pcStack_58,&UNK_142ea7758);
  }
  else {
code_r0x000140dbc422:
    pcVar14 = pcStack_58;
    pcVar10 = pcStack_e0;
    func_0x000142923c00(pcVar21,pcStack_e0,pcStack_58);
    puVar12 = (undefined8 *)func_0x000140613c10(0x70,8);
    pcVar23 = pcStack_1d8;
    if (puVar12 != (undefined8 *)0x0) {
      *puVar12 = 0;
      puVar12[1] = &UNK_142ea7790;
      puVar12[2] = 9;
      *(undefined2 *)(puVar12 + 3) = 0x300;
      *(undefined1 *)((longlong)puVar12 + 0x1a) = 4;
      puVar12[4] = pcVar14;
      puVar12[5] = pcVar21;
      puVar12[6] = pcVar14;
      uVar7 = _UNK_142b700ec;
      uVar6 = _UNK_142b700e8;
      uVar5 = _UNK_142b700e4;
      *(undefined4 *)(puVar12 + 7) = _DAT_142b700e0;
      *(undefined4 *)((longlong)puVar12 + 0x3c) = uVar5;
      *(undefined4 *)(puVar12 + 8) = uVar6;
      *(undefined4 *)((longlong)puVar12 + 0x44) = uVar7;
      *(undefined2 *)(puVar12 + 9) = 2;
      *(undefined2 *)(puVar12 + 10) = 2;
      *(undefined4 *)(puVar12 + 0xb) = 0;
      *(undefined4 *)((longlong)puVar12 + 0x6c) = 0x101;
      *(undefined8 **)pcStack_1d8 = puVar12;
      *(undefined **)(pcStack_1d8 + 8) = &UNK_142ea7a70;
      *(code **)(pcStack_1d8 + 0x10) = pcVar9;
      *(undefined **)(pcStack_1d8 + 0x18) = puVar28;
      *(undefined4 **)(pcStack_1d8 + 0x20) = puVar15;
      *(undefined **)(pcStack_1d8 + 0x28) = &UNK_142ea7818;
      if (uStack_110 != 0) {
        func_0x000140613c20(pcVar10,uStack_110,1);
      }
      if ((char)pcStack_60 != '\0') {
        func_0x000140613c20(pcStack_108,0x33,1);
      }
      return pcVar23;
    }
    pcStack_1d8 = pcVar21;
    pcStack_e0 = pcVar10;
    FUN_1428d9000(8,0x70);
  }
LAB_140dbd5d1:
                    /* WARNING: Does not return */
  pcVar9 = (code *)invalidInstructionException();
  (*pcVar9)();
}

