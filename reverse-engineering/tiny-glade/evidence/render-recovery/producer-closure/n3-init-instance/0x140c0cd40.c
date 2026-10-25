
/* WARNING: Type propagation algorithm not settling */

undefined8 FUN_140c0cd40(undefined8 param_1,ulonglong param_2,longlong param_3)

{
  char *pcVar1;
  char *pcVar2;
  byte bVar3;
  longlong lVar4;
  longlong lVar5;
  longlong lVar6;
  int iVar7;
  longlong lVar8;
  code *pcVar9;
  ulonglong uVar10;
  undefined4 *puVar11;
  undefined8 *******pppppppuVar12;
  undefined8 *******pppppppuVar13;
  undefined8 *******pppppppuVar14;
  uint uVar15;
  undefined *puVar16;
  uint uVar17;
  longlong lVar18;
  undefined8 *******pppppppuVar19;
  longlong lVar20;
  longlong lVar21;
  undefined8 *******pppppppuStack_148;
  code *pcStack_140;
  undefined8 *******pppppppuStack_138;
  code *pcStack_130;
  undefined8 *******pppppppuStack_128;
  undefined *puStack_120;
  undefined8 *******pppppppuStack_118;
  undefined *puStack_110;
  undefined8 *******pppppppuStack_108;
  undefined8 *******pppppppuStack_100;
  code *pcStack_f8;
  ulonglong uStack_f0;
  undefined8 *******pppppppuStack_e8;
  code *pcStack_e0;
  undefined8 uStack_d8;
  undefined4 uStack_d0;
  undefined4 uStack_cc;
  undefined4 uStack_c8;
  undefined4 uStack_c4;
  undefined8 uStack_c0;
  undefined8 *******pppppppuStack_b8;
  undefined8 uStack_b0;
  undefined4 uStack_a8;
  undefined4 uStack_a4;
  undefined8 *******pppppppuStack_a0;
  longlong lStack_98;
  undefined8 *******pppppppuStack_90;
  undefined8 uStack_88;
  undefined8 uStack_80;
  undefined4 uStack_6c;
  undefined *puStack_68;
  ulonglong uStack_60;
  undefined1 uStack_49;
  undefined8 uStack_48;
  
  uStack_48 = 0xfffffffffffffffe;
  if (param_3 != 0) {
    if ((*(int *)(param_3 + 0x20) != -0x37e52af2) && (*(int *)(param_3 + 0x20) != 0x609a13b)) {
      lVar4 = *(longlong *)(param_3 + 0x18);
      if (lVar4 == 0) {
        pcVar9 = (code *)0x14;
        pppppppuVar12 = (undefined8 *******)&UNK_142b19718;
      }
      else {
        lVar8 = func_0x000142924010(lVar4);
        FUN_140d81100(&uStack_f0,lVar4,lVar8 + 1);
        pcVar9 = pcStack_e0;
        pppppppuVar12 = pppppppuStack_e8;
        if ((int)uStack_f0 == 1) {
          pppppppuStack_138 = pppppppuStack_e8;
          pcStack_130 = pcStack_e0;
          FUN_1428d9760(&UNK_142b18f88,0x2b,&pppppppuStack_138,&UNK_142b18fb8,&UNK_142b197e8);
          goto code_r0x000140c0d448;
        }
      }
      lVar4 = *(longlong *)(param_3 + 0x28);
      pppppppuStack_148 = pppppppuVar12;
      pcStack_140 = pcVar9;
      if (lVar4 == 0) {
        pppppppuStack_100 = (undefined8 *******)&UNK_142b1972c;
        pcStack_f8 = (code *)0xc;
      }
      else {
        lVar8 = func_0x000142924010(lVar4);
        FUN_140d81100(&uStack_f0,lVar4,lVar8 + 1);
        if ((int)uStack_f0 == 1) {
          pppppppuStack_138 = pppppppuStack_e8;
          pcStack_130 = pcStack_e0;
          FUN_1428d9760(&UNK_142b18f88,0x2b,&pppppppuStack_138,&UNK_142b18fb8,&UNK_142b19800);
          goto code_r0x000140c0d448;
        }
        pppppppuStack_100 = pppppppuStack_e8;
        pcStack_f8 = pcStack_e0;
        if (pcStack_e0 < (code *)0x36) {
          if ((pcStack_e0 == (code *)0x35) &&
             (iVar7 = func_0x000142923bf0(&UNK_142b19738,pppppppuStack_e8,0x35), iVar7 == 0)) {
            return 0;
          }
        }
        else {
          FUN_140d89940(&uStack_f0,pppppppuStack_e8,pcStack_e0,&UNK_142b19738,0x35);
          if ((uStack_f0 & 1) == 0) {
            if (uStack_d8._2_1_ == '\0') {
              lVar4 = CONCAT44(uStack_a4,uStack_a8);
              uVar10 = uStack_d8 & 0xff;
              pppppppuVar12 = pppppppuStack_e8;
              do {
                if (pppppppuVar12 != (undefined8 *******)0x0) {
                  if (pppppppuVar12 < pppppppuStack_a0) {
                    if (*(char *)(lVar4 + (longlong)pppppppuVar12) < -0x40) {
code_r0x000140c0d3bb:
                      FUN_1428d9940(lVar4,pppppppuStack_a0,pppppppuVar12,pppppppuStack_a0,
                                    &UNK_142b19060);
                      goto code_r0x000140c0d448;
                    }
                  }
                  else if (pppppppuVar12 != pppppppuStack_a0) goto code_r0x000140c0d3bb;
                }
                if (pppppppuVar12 == pppppppuStack_a0) {
                  if ((uVar10 & 1) != 0) {
                    return 0;
                  }
                  break;
                }
                bVar3 = *(byte *)(lVar4 + (longlong)pppppppuVar12);
                uVar15 = (uint)bVar3;
                if ((char)bVar3 < '\0') {
                  uVar17 = bVar3 & 0x1f;
                  uVar15 = *(byte *)(lVar4 + 1 + (longlong)pppppppuVar12) & 0x3f;
                  if (bVar3 < 0xe0) {
                    uVar15 = uVar17 << 6 | uVar15;
                  }
                  else {
                    uVar15 = *(byte *)(lVar4 + 2 + (longlong)pppppppuVar12) & 0x3f | uVar15 << 6;
                    if (bVar3 < 0xf0) {
                      uVar15 = uVar15 | uVar17 << 0xc;
                    }
                    else {
                      uVar15 = *(byte *)(lVar4 + 3 + (longlong)pppppppuVar12) & 0x3f | uVar15 << 6 |
                               (bVar3 & 7) << 0x12;
                    }
                  }
                }
                if ((uVar10 & 1) != 0) {
                  return 0;
                }
                lVar8 = 1;
                if ((0x7f < uVar15) && (lVar8 = 2, 0x7ff < uVar15)) {
                  lVar8 = 4 - (ulonglong)(uVar15 < 0x10000);
                }
                uVar10 = uVar10 ^ 1;
                pppppppuVar12 = (undefined8 *******)((longlong)pppppppuVar12 + lVar8);
              } while( true );
            }
          }
          else {
            lVar4 = CONCAT44(uStack_a4,uStack_a8);
            puStack_68 = (undefined *)((longlong)pppppppuStack_90 + -1);
            if (pppppppuStack_b8 == (undefined8 *******)0xffffffffffffffff) {
              lVar8 = CONCAT44(uStack_c4,uStack_c8);
              pppppppuVar12 = (undefined8 *******)(puStack_68 + lVar8);
              if (pppppppuVar12 < pppppppuStack_a0) {
                if ((undefined8 *******)((longlong)pppppppuStack_e8 + -1) < pppppppuStack_90) {
                  uStack_60 = uStack_d8;
code_r0x000140c0d21f:
                  if ((CONCAT44(uStack_cc,uStack_d0) >>
                       ((ulonglong)*(byte *)(lVar4 + (longlong)pppppppuVar12) & 0x3f) & 1) == 0) {
                    lVar8 = lVar8 + (longlong)pppppppuStack_90;
                  }
                  else {
                    lVar21 = -lVar8;
                    pppppppuVar12 = pppppppuStack_e8;
                    do {
                      if (pppppppuStack_90 == pppppppuVar12) {
                        pppppppuVar12 = pppppppuStack_e8;
                        goto code_r0x000140c0d280;
                      }
                      if (pppppppuStack_a0 <= (undefined8 *******)(lVar8 + (longlong)pppppppuVar12))
                      goto code_r0x000140c0d39d;
                      pcVar1 = (char *)(lStack_98 + (longlong)pppppppuVar12);
                      lVar21 = lVar21 + -1;
                      pcVar2 = (char *)(lVar8 + lVar4 + (longlong)pppppppuVar12);
                      pppppppuVar12 = (undefined8 *******)((longlong)pppppppuVar12 + 1);
                    } while (*pcVar1 == *pcVar2);
                    lVar8 = -lVar21;
                  }
                  goto code_r0x000140c0d20b;
                }
                do {
                  if ((CONCAT44(uStack_cc,uStack_d0) >>
                       ((ulonglong)*(byte *)(lVar4 + (longlong)pppppppuVar12) & 0x3f) & 1) == 0) {
                    lVar21 = lVar8 + (longlong)pppppppuStack_90;
                  }
                  else {
                    lVar18 = 0;
                    lVar20 = lVar8;
                    do {
                      if (pppppppuStack_90 <=
                          (undefined8 *******)((longlong)pppppppuStack_e8 + lVar18)) {
                        if (pppppppuStack_e8 == (undefined8 *******)0x0) {
                          return 0;
                        }
                        FUN_1428d9518((undefined8 *******)((longlong)pppppppuStack_e8 + -1),
                                      pppppppuStack_90,&UNK_142b18eb0);
                        goto code_r0x000140c0d448;
                      }
                      if (pppppppuStack_a0 <=
                          (undefined8 *******)((longlong)pppppppuStack_e8 + lVar20))
                      goto code_r0x000140c0d39d;
                      lVar6 = lVar18 + lStack_98;
                      lVar21 = lVar20 + 1;
                      lVar18 = lVar18 + 1;
                      lVar5 = lVar20 + lVar4;
                      lVar20 = lVar21;
                    } while (*(char *)((longlong)pppppppuStack_e8 + lVar6) ==
                             *(char *)((longlong)pppppppuStack_e8 + lVar5));
                  }
                  pppppppuVar12 = (undefined8 *******)(puStack_68 + lVar21);
                  lVar8 = lVar21;
                } while (pppppppuVar12 < pppppppuStack_a0);
              }
            }
            else {
              puVar16 = (undefined *)CONCAT44(uStack_c4,uStack_c8);
              pppppppuVar12 = (undefined8 *******)(puStack_68 + (longlong)puVar16);
              if (pppppppuVar12 < pppppppuStack_a0) {
                uStack_60 = uStack_d8;
                pppppppuStack_108 = (undefined8 *******)((longlong)pppppppuStack_90 - uStack_d8);
                pppppppuVar19 = pppppppuStack_b8;
code_r0x000140c0d01d:
                if ((CONCAT44(uStack_cc,uStack_d0) >>
                     ((ulonglong)*(byte *)(lVar4 + (longlong)pppppppuVar12) & 0x3f) & 1) == 0) {
                  puVar16 = puVar16 + (longlong)pppppppuStack_90;
                  pppppppuVar19 = (undefined8 *******)0x0;
                }
                else {
                  pppppppuVar12 = pppppppuStack_e8;
                  if (pppppppuStack_e8 < pppppppuVar19) {
                    pppppppuVar12 = pppppppuVar19;
                  }
                  pppppppuVar13 = pppppppuVar12;
                  do {
                    pppppppuVar14 = pppppppuStack_e8;
                    if (pppppppuStack_90 <= pppppppuVar13) goto code_r0x000140c0d080;
                    if (pppppppuStack_a0 <= puVar16 + (longlong)pppppppuVar13) {
                      pppppppuVar19 =
                           (undefined8 *******)((longlong)pppppppuVar12 + (longlong)puVar16);
                      if ((undefined8 *******)((longlong)pppppppuVar12 + (longlong)puVar16) <
                          pppppppuStack_a0) {
                        pppppppuVar19 = pppppppuStack_a0;
                      }
                      FUN_1428d9518(pppppppuVar19,pppppppuStack_a0,&UNK_142b18ee0);
                      goto code_r0x000140c0d448;
                    }
                    pppppppuVar14 = (undefined8 *******)((longlong)pppppppuVar13 + 1);
                    pcVar1 = (char *)(lStack_98 + (longlong)pppppppuVar13);
                    pcVar2 = puVar16 + lVar4 + (longlong)pppppppuVar13;
                    pppppppuVar13 = pppppppuVar14;
                  } while (*pcVar1 == *pcVar2);
                  puVar16 = (undefined *)
                            (((longlong)puVar16 - (longlong)pppppppuStack_e8) +
                            (longlong)pppppppuVar14);
                  pppppppuVar19 = (undefined8 *******)0x0;
                }
                goto code_r0x000140c0d00a;
              }
            }
          }
        }
      }
code_r0x000140c0d105:
      if (*plRam000000014362c3a8 != 0) {
        uStack_6c = *(undefined4 *)(param_3 + 0x20);
        pppppppuStack_138 = (undefined8 *******)&uStack_6c;
        pcStack_130 = FUN_140d8cc50;
        pppppppuStack_128 = &pppppppuStack_148;
        puStack_120 = &UNK_140109e90;
        pppppppuStack_118 = &pppppppuStack_100;
        puStack_110 = &UNK_140109e90;
        puVar11 = (undefined4 *)func_0x00014067dd80(&UNK_142b197b8);
        uStack_d0 = *puVar11;
        uStack_cc = puVar11[1];
        uStack_c8 = puVar11[2];
        uStack_c4 = puVar11[3];
        uStack_a4 = puVar11[4];
        uStack_c0 = 1;
        pppppppuStack_b8 = (undefined8 *******)&UNK_142b191b0;
        uStack_b0 = 0x17;
        pppppppuStack_a0 = (undefined8 *******)&UNK_142b19778;
        lStack_98 = 4;
        pppppppuStack_90 = &pppppppuStack_138;
        uStack_88 = 3;
        uStack_80 = 0;
        uStack_f0 = 0;
        pppppppuStack_e8 = (undefined8 *******)&UNK_142b191b0;
        pcStack_e0 = (code *)0x17;
        uStack_d8 = 0;
        uStack_a8 = 1;
        func_0x000140d4a330(&uStack_49,&uStack_f0);
      }
      if ((param_2 & 2) != 0) {
        FUN_1428cab30();
        goto code_r0x000140c0d448;
      }
    }
    return 0;
  }
  FUN_1428d9310(&UNK_142b19700);
  goto code_r0x000140c0d448;
  while (pcVar2 = (char *)(lStack_98 + -1 + (longlong)pppppppuVar12),
        pcVar1 = (char *)(lVar8 + lVar4 + -1 + (longlong)pppppppuVar12),
        pppppppuVar12 = (undefined8 *******)((longlong)pppppppuVar12 + -1), *pcVar2 == *pcVar1) {
code_r0x000140c0d280:
    if (pppppppuVar12 == (undefined8 *******)0x0) {
      return 0;
    }
    if (pppppppuStack_a0 <= (undefined8 *******)(lVar8 + -1 + (longlong)pppppppuVar12)) {
      FUN_1428d9518((undefined *)((longlong)pppppppuVar12 + lVar8 + -1),pppppppuStack_a0,
                    &UNK_142b18ec8);
      goto code_r0x000140c0d448;
    }
  }
  lVar8 = lVar8 + uStack_d8;
code_r0x000140c0d20b:
  pppppppuVar12 = (undefined8 *******)(puStack_68 + lVar8);
  if (pppppppuStack_a0 <= pppppppuVar12) goto code_r0x000140c0d105;
  goto code_r0x000140c0d21f;
  while( true ) {
    pppppppuVar12 = (undefined8 *******)((longlong)pppppppuVar14 + (longlong)puVar16);
    if (pppppppuStack_a0 <= pppppppuVar12) {
      FUN_1428d9518(pppppppuVar12,pppppppuStack_a0,&UNK_142b18ec8);
      goto code_r0x000140c0d448;
    }
    if (*(char *)(lStack_98 + (longlong)pppppppuVar14) != *(char *)(lVar4 + (longlong)pppppppuVar12)
       ) break;
code_r0x000140c0d080:
    if (pppppppuVar14 <= pppppppuVar19) {
      return 0;
    }
    pppppppuVar14 = (undefined8 *******)((longlong)pppppppuVar14 + -1);
    if (pppppppuStack_90 <= pppppppuVar14) {
      FUN_1428d9518(pppppppuVar14,pppppppuStack_90,&UNK_142b18eb0);
      goto code_r0x000140c0d448;
    }
  }
  puVar16 = puVar16 + uStack_d8;
  pppppppuVar19 = pppppppuStack_108;
code_r0x000140c0d00a:
  pppppppuVar12 = (undefined8 *******)(puStack_68 + (longlong)puVar16);
  if (pppppppuStack_a0 <= pppppppuVar12) goto code_r0x000140c0d105;
  goto code_r0x000140c0d01d;
code_r0x000140c0d39d:
  pppppppuVar12 = (undefined8 *******)(lVar8 + (longlong)pppppppuStack_e8);
  if ((undefined8 *******)(lVar8 + (longlong)pppppppuStack_e8) < pppppppuStack_a0) {
    pppppppuVar12 = pppppppuStack_a0;
  }
  FUN_1428d9518(pppppppuVar12,pppppppuStack_a0,&UNK_142b18ee0);
code_r0x000140c0d448:
                    /* WARNING: Does not return */
  pcVar9 = (code *)invalidInstructionException();
  (*pcVar9)();
}

