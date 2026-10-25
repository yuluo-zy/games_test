
/* WARNING: Removing unreachable block (ram,0x000140f7896e) */
/* WARNING: Removing unreachable block (ram,0x000140f7897f) */
/* WARNING: Removing unreachable block (ram,0x000140f789a8) */
/* WARNING: Removing unreachable block (ram,0x000140f789b1) */
/* WARNING: Removing unreachable block (ram,0x000140f789c8) */
/* WARNING: Removing unreachable block (ram,0x000140f789d1) */

void FUN_140f78630(longlong *param_1,longlong *param_2,longlong *param_3,longlong *param_4)

{
  char *pcVar1;
  longlong lVar2;
  uint uVar3;
  uint uVar4;
  int iVar5;
  int iVar6;
  int iVar7;
  int iVar8;
  char cVar9;
  char cVar10;
  char cVar11;
  char cVar12;
  char cVar13;
  char cVar14;
  char cVar15;
  char cVar16;
  char cVar17;
  char cVar18;
  char cVar19;
  char cVar20;
  char cVar21;
  char cVar22;
  char cVar23;
  char cVar24;
  longlong *plVar25;
  undefined *puVar26;
  char cVar27;
  ushort uVar28;
  int iVar29;
  undefined8 uVar30;
  ulonglong uVar31;
  ulonglong uVar32;
  longlong lVar33;
  longlong **pplVar34;
  uint uVar35;
  longlong lVar36;
  byte bVar37;
  undefined1 auVar38 [16];
  undefined1 unaff_XMM6_Ba;
  undefined1 unaff_XMM6_Bb;
  undefined1 unaff_XMM6_Bc;
  undefined1 unaff_XMM6_Bd;
  undefined1 unaff_XMM6_Be;
  undefined1 unaff_XMM6_Bf;
  undefined1 unaff_XMM6_Bg;
  undefined1 unaff_XMM6_Bh;
  undefined1 unaff_XMM6_Bi;
  undefined1 unaff_XMM6_Bj;
  undefined1 unaff_XMM6_Bk;
  undefined1 unaff_XMM6_Bl;
  undefined1 unaff_XMM6_Bm;
  undefined1 unaff_XMM6_Bn;
  undefined1 unaff_XMM6_Bo;
  undefined1 unaff_XMM6_Bp;
  char cVar39;
  char cVar40;
  char cVar41;
  undefined1 auVar42 [16];
  longlong *aplStack_b18 [75];
  byte bStack_8c0;
  undefined7 uStack_8bf;
  longlong lStack_8b8;
  undefined1 auStack_8b0 [608];
  longlong lStack_650;
  longlong lStack_648;
  longlong lStack_3f8;
  longlong lStack_3f0;
  longlong *aplStack_3e8 [76];
  undefined8 uStack_188;
  longlong lStack_180;
  longlong lStack_178;
  longlong lStack_170;
  longlong lStack_158;
  longlong lStack_150;
  longlong *plStack_148;
  longlong lStack_140;
  longlong lStack_138;
  longlong *plStack_130;
  undefined *puStack_128;
  ulonglong uStack_120;
  undefined4 uStack_118;
  undefined4 uStack_114;
  undefined4 uStack_110;
  undefined4 uStack_10c;
  undefined8 *puStack_108;
  undefined8 uStack_100;
  longlong lStack_f8;
  longlong *plStack_f0;
  ulonglong uStack_e8;
  longlong lStack_e0;
  undefined *puStack_d8;
  ulonglong uStack_d0;
  undefined4 uStack_c8;
  undefined4 uStack_c4;
  undefined4 uStack_c0;
  undefined4 uStack_bc;
  longlong *plStack_b0;
  longlong *plStack_a8;
  longlong lStack_a0;
  undefined8 *puStack_98;
  undefined8 uStack_90;
  undefined1 uStack_83;
  undefined1 uStack_82;
  undefined1 uStack_81;
  undefined8 uStack_80;
  
  uStack_80 = 0xfffffffffffffffe;
  plStack_b0 = param_1;
  plStack_a8 = param_2;
  uVar30 = FUN_140d76ed0();
  FUN_140d68f80(&bStack_8c0,uVar30,param_2);
  lStack_3f8 = CONCAT71(uStack_8bf,bStack_8c0);
  if (lStack_3f8 == 2) {
    if (((uint)lStack_8b8 & 3) == 1) {
      lStack_a0 = lStack_8b8 + -1;
      uStack_90 = *(undefined8 *)(lStack_8b8 + -1);
      puStack_98 = *(undefined8 **)(lStack_8b8 + 7);
      if ((code *)*puStack_98 != (code *)0x0) {
        (*(code *)*puStack_98)(uStack_90);
      }
      if (puStack_98[1] != 0) {
        func_0x000140613c20(uStack_90,puStack_98[1],puStack_98[2]);
      }
      func_0x000140613c20(lStack_a0,0x18,8);
    }
  }
  else {
    plStack_f0 = param_4;
    func_0x000142923c00(aplStack_3e8,auStack_8b0,0x260);
    lStack_3f0 = lStack_8b8;
    lStack_140 = param_3[3];
    plStack_130 = param_3 + 4;
    lStack_138 = *param_3;
    uStack_e8 = param_3[1];
    lStack_e0 = lStack_138 + -0x18;
    lStack_158 = plStack_a8[1];
    lStack_150 = plStack_a8[2];
    iVar5 = -(uint)(CONCAT13(unaff_XMM6_Bd,
                             CONCAT12(unaff_XMM6_Bc,CONCAT11(unaff_XMM6_Bb,unaff_XMM6_Ba))) ==
                   CONCAT13(unaff_XMM6_Bd,
                            CONCAT12(unaff_XMM6_Bc,CONCAT11(unaff_XMM6_Bb,unaff_XMM6_Ba))));
    iVar6 = -(uint)(CONCAT13(unaff_XMM6_Bh,
                             CONCAT12(unaff_XMM6_Bg,CONCAT11(unaff_XMM6_Bf,unaff_XMM6_Be))) ==
                   CONCAT13(unaff_XMM6_Bh,
                            CONCAT12(unaff_XMM6_Bg,CONCAT11(unaff_XMM6_Bf,unaff_XMM6_Be))));
    iVar7 = -(uint)(CONCAT13(unaff_XMM6_Bl,
                             CONCAT12(unaff_XMM6_Bk,CONCAT11(unaff_XMM6_Bj,unaff_XMM6_Bi))) ==
                   CONCAT13(unaff_XMM6_Bl,
                            CONCAT12(unaff_XMM6_Bk,CONCAT11(unaff_XMM6_Bj,unaff_XMM6_Bi))));
    iVar8 = -(uint)(CONCAT13(unaff_XMM6_Bp,
                             CONCAT12(unaff_XMM6_Bo,CONCAT11(unaff_XMM6_Bn,unaff_XMM6_Bm))) ==
                   CONCAT13(unaff_XMM6_Bp,
                            CONCAT12(unaff_XMM6_Bo,CONCAT11(unaff_XMM6_Bn,unaff_XMM6_Bm))));
    plStack_148 = param_3;
    while (FUN_140d59e80(&bStack_8c0,&lStack_3f8), (bStack_8c0 & 1) != 0) {
      func_0x000142923c00(&lStack_650,&lStack_8b8,600);
      if (lStack_650 == 0) {
LAB_140f788d0:
        if (((uint)lStack_648 & 3) == 1) {
          lStack_a0 = lStack_648 + -1;
          uStack_90 = *(undefined8 *)(lStack_648 + -1);
          puStack_98 = *(undefined8 **)(lStack_648 + 7);
          if ((code *)*puStack_98 != (code *)0x0) {
            (*(code *)*puStack_98)(uStack_90);
          }
          goto LAB_140f78917;
        }
      }
      else {
        func_0x000142923c00(aplStack_b18,&lStack_8b8,600);
        FUN_140d5a110(&puStack_d8,aplStack_b18);
        uVar31 = uStack_d0;
        puVar26 = puStack_d8;
        if ((int)puStack_d8 == 0) {
          pplVar34 = aplStack_b18;
          FUN_140d5a130(&lStack_178,aplStack_b18);
          uVar30 = FUN_140d76ed0(&lStack_178);
          FUN_140d58e70(&puStack_d8,uVar30,pplVar34);
          uVar32 = uStack_d0;
          if (((ulonglong)puStack_d8 & 1) != 0) {
LAB_140f78afd:
            if (lStack_178 != 0) {
              uVar30 = 1;
              lVar33 = lStack_170;
              lVar36 = lStack_178;
              goto LAB_140f78b16;
            }
            goto LAB_140f78b1b;
          }
          uVar30 = CONCAT44(uStack_c4,uStack_c8);
          FUN_140d59f10(&puStack_128,aplStack_b18);
          uStack_81 = 1;
          FUN_141d45aa0(&puStack_108,lStack_158,lStack_150,uVar32,uVar30);
          lVar33 = lStack_f8;
          uStack_90 = uStack_100;
          uStack_188 = uStack_100;
          lStack_180 = lStack_f8;
          puStack_d8 = &UNK_142bab8a1;
          uStack_d0 = 3;
          uStack_82 = 1;
          uStack_83 = 1;
          cVar27 = FUN_140fcb800(&uStack_188,&puStack_d8);
          if (cVar27 != '\0') {
            if (puStack_108 != (undefined8 *)0x0) {
              func_0x000140613c20(uStack_90,puStack_108,1);
            }
            if (puStack_128 != (undefined *)0x0) {
              func_0x000140613c20(uStack_120,puStack_128,1);
            }
            goto LAB_140f78afd;
          }
          if ((((ulonglong)puVar26 & 0x40000000000) == 0) || ((uVar31 & 0x20000000) == 0)) {
            if (((ulonglong)puVar26 & 0x1000000000) == 0) {
              if (lStack_140 != 0) {
                uVar31 = FUN_1405b1a70(plStack_130,uStack_90,lVar33);
                bVar37 = (byte)(uVar31 >> 0x39);
                auVar38 = ZEXT216(CONCAT11(bVar37,bVar37));
                auVar38 = pshuflw(auVar38,auVar38,0);
                cVar27 = auVar38[0];
                cVar39 = auVar38[1];
                cVar40 = auVar38[2];
                cVar41 = auVar38[3];
                lVar36 = 0;
                while( true ) {
                  uVar31 = uVar31 & uStack_e8;
                  pcVar1 = (char *)(lStack_138 + uVar31);
                  cVar9 = *pcVar1;
                  cVar10 = pcVar1[1];
                  cVar11 = pcVar1[2];
                  cVar12 = pcVar1[3];
                  cVar13 = pcVar1[4];
                  cVar14 = pcVar1[5];
                  cVar15 = pcVar1[6];
                  cVar16 = pcVar1[7];
                  cVar17 = pcVar1[8];
                  cVar18 = pcVar1[9];
                  cVar19 = pcVar1[10];
                  cVar20 = pcVar1[0xb];
                  cVar21 = pcVar1[0xc];
                  cVar22 = pcVar1[0xd];
                  cVar23 = pcVar1[0xe];
                  cVar24 = pcVar1[0xf];
                  auVar38[0] = -(cVar9 == cVar27);
                  auVar38[1] = -(cVar10 == cVar39);
                  auVar38[2] = -(cVar11 == cVar40);
                  auVar38[3] = -(cVar12 == cVar41);
                  auVar38[4] = -(cVar13 == cVar27);
                  auVar38[5] = -(cVar14 == cVar39);
                  auVar38[6] = -(cVar15 == cVar40);
                  auVar38[7] = -(cVar16 == cVar41);
                  auVar38[8] = -(cVar17 == cVar27);
                  auVar38[9] = -(cVar18 == cVar39);
                  auVar38[10] = -(cVar19 == cVar40);
                  auVar38[0xb] = -(cVar20 == cVar41);
                  auVar38[0xc] = -(cVar21 == cVar27);
                  auVar38[0xd] = -(cVar22 == cVar39);
                  auVar38[0xe] = -(cVar23 == cVar40);
                  auVar38[0xf] = -(cVar24 == cVar41);
                  uVar28 = (ushort)(SUB161(auVar38 >> 7,0) & 1) |
                           (ushort)(SUB161(auVar38 >> 0xf,0) & 1) << 1 |
                           (ushort)(SUB161(auVar38 >> 0x17,0) & 1) << 2 |
                           (ushort)(SUB161(auVar38 >> 0x1f,0) & 1) << 3 |
                           (ushort)(SUB161(auVar38 >> 0x27,0) & 1) << 4 |
                           (ushort)(SUB161(auVar38 >> 0x2f,0) & 1) << 5 |
                           (ushort)(SUB161(auVar38 >> 0x37,0) & 1) << 6 |
                           (ushort)(SUB161(auVar38 >> 0x3f,0) & 1) << 7 |
                           (ushort)(SUB161(auVar38 >> 0x47,0) & 1) << 8 |
                           (ushort)(SUB161(auVar38 >> 0x4f,0) & 1) << 9 |
                           (ushort)(SUB161(auVar38 >> 0x57,0) & 1) << 10 |
                           (ushort)(SUB161(auVar38 >> 0x5f,0) & 1) << 0xb |
                           (ushort)(SUB161(auVar38 >> 0x67,0) & 1) << 0xc |
                           (ushort)(SUB161(auVar38 >> 0x6f,0) & 1) << 0xd |
                           (ushort)(SUB161(auVar38 >> 0x77,0) & 1) << 0xe |
                           (ushort)(auVar38[0xf] >> 7) << 0xf;
                  uVar35 = (uint)uVar28;
                  while (uVar28 != 0) {
                    uVar3 = 0;
                    for (uVar4 = uVar35; (uVar4 & 1) == 0; uVar4 = uVar4 >> 1 | 0x80000000) {
                      uVar3 = uVar3 + 1;
                    }
                    uVar32 = uVar3 + uVar31 & uStack_e8;
                    if ((*(longlong *)(lStack_e0 + 0x10 + uVar32 * -0x18) == lVar33) &&
                       (iVar29 = func_0x000142923bf0(uStack_90,
                                                     *(undefined8 *)(lStack_e0 + uVar32 * -0x18 + 8)
                                                     ,lVar33), iVar29 == 0)) goto LAB_140f78b46;
                    uVar28 = (ushort)(uVar35 - 1) & (ushort)uVar35;
                    uVar35 = CONCAT22((short)(uVar35 - 1 >> 0x10),uVar28);
                  }
                  auVar42[0] = -(cVar9 == (char)iVar5);
                  auVar42[1] = -(cVar10 == (char)((uint)iVar5 >> 8));
                  auVar42[2] = -(cVar11 == (char)((uint)iVar5 >> 0x10));
                  auVar42[3] = -(cVar12 == (char)((uint)iVar5 >> 0x18));
                  auVar42[4] = -(cVar13 == (char)iVar6);
                  auVar42[5] = -(cVar14 == (char)((uint)iVar6 >> 8));
                  auVar42[6] = -(cVar15 == (char)((uint)iVar6 >> 0x10));
                  auVar42[7] = -(cVar16 == (char)((uint)iVar6 >> 0x18));
                  auVar42[8] = -(cVar17 == (char)iVar7);
                  auVar42[9] = -(cVar18 == (char)((uint)iVar7 >> 8));
                  auVar42[10] = -(cVar19 == (char)((uint)iVar7 >> 0x10));
                  auVar42[0xb] = -(cVar20 == (char)((uint)iVar7 >> 0x18));
                  auVar42[0xc] = -(cVar21 == (char)iVar8);
                  auVar42[0xd] = -(cVar22 == (char)((uint)iVar8 >> 8));
                  auVar42[0xe] = -(cVar23 == (char)((uint)iVar8 >> 0x10));
                  auVar42[0xf] = -(cVar24 == (char)((uint)iVar8 >> 0x18));
                  if ((((((((((((((((SUB161(auVar42 >> 7,0) & 1) != 0 ||
                                   (SUB161(auVar42 >> 0xf,0) & 1) != 0) ||
                                  (SUB161(auVar42 >> 0x17,0) & 1) != 0) ||
                                 (SUB161(auVar42 >> 0x1f,0) & 1) != 0) ||
                                (SUB161(auVar42 >> 0x27,0) & 1) != 0) ||
                               (SUB161(auVar42 >> 0x2f,0) & 1) != 0) ||
                              (SUB161(auVar42 >> 0x37,0) & 1) != 0) ||
                             (SUB161(auVar42 >> 0x3f,0) & 1) != 0) ||
                            (SUB161(auVar42 >> 0x47,0) & 1) != 0) ||
                           (SUB161(auVar42 >> 0x4f,0) & 1) != 0) ||
                          (SUB161(auVar42 >> 0x57,0) & 1) != 0) ||
                         (SUB161(auVar42 >> 0x5f,0) & 1) != 0) ||
                        (SUB161(auVar42 >> 0x67,0) & 1) != 0) ||
                       (SUB161(auVar42 >> 0x6f,0) & 1) != 0) || (SUB161(auVar42 >> 0x77,0) & 1) != 0
                      ) || auVar42[0xf] < '\0') break;
                  uVar31 = uVar31 + lVar36 + 0x10;
                  lVar36 = lVar36 + 0x10;
                }
              }
              plVar25 = plStack_f0;
              puStack_98 = puStack_108;
              lVar36 = plStack_f0[2];
              if (lVar36 == *plStack_f0) {
                FUN_140562db0(plStack_f0,&UNK_142bab8d0);
              }
              lVar2 = plVar25[1];
              *(undefined8 **)(lVar2 + lVar36 * 0x18) = puStack_98;
              *(undefined8 *)(lVar2 + 8 + lVar36 * 0x18) = uStack_90;
              *(longlong *)(lVar2 + 0x10 + lVar36 * 0x18) = lVar33;
              plVar25[2] = lVar36 + 1;
              goto joined_r0x000140f78da7;
            }
            uStack_c8 = uStack_118;
            uStack_c4 = uStack_114;
            uStack_c0 = uStack_110;
            uStack_bc = uStack_10c;
            puStack_d8 = puStack_128;
            uStack_d0 = uStack_120;
            uStack_82 = 0;
            uStack_83 = 0;
            FUN_140f78630(&puStack_d8,&puStack_108,plStack_148,plStack_f0);
          }
          else {
LAB_140f78b46:
            if (puStack_108 != (undefined8 *)0x0) {
              func_0x000140613c20(uStack_90,puStack_108,1);
            }
joined_r0x000140f78da7:
            if (puStack_128 != (undefined *)0x0) {
              func_0x000140613c20(uStack_120,puStack_128,1);
            }
          }
          if (lStack_178 != 0) {
            func_0x000140613c20(lStack_170,lStack_178,1);
          }
          LOCK();
          *aplStack_b18[0] = *aplStack_b18[0] + -1;
          UNLOCK();
          if (*aplStack_b18[0] == 0) {
            FUN_140d53bc0(aplStack_b18);
          }
          if ((lStack_650 == 0) && (((uint)lStack_648 & 3) == 1)) {
            lStack_a0 = lStack_648 + -1;
            uStack_90 = *(undefined8 *)(lStack_648 + -1);
            puStack_98 = *(undefined8 **)(lStack_648 + 7);
            if ((code *)*puStack_98 != (code *)0x0) {
              (*(code *)*puStack_98)(uStack_90);
            }
LAB_140f78917:
            if (puStack_98[1] != 0) {
              func_0x000140613c20(uStack_90,puStack_98[1],puStack_98[2]);
            }
            func_0x000140613c20(lStack_a0,0x18,8);
          }
        }
        else {
          if (((uint)uStack_d0 & 3) == 1) {
            lStack_a0 = uStack_d0 - 1;
            uStack_90 = *(undefined8 *)(uStack_d0 - 1);
            puStack_98 = *(undefined8 **)(uStack_d0 + 7);
            if ((code *)*puStack_98 != (code *)0x0) {
              (*(code *)*puStack_98)(uStack_90);
            }
            if (puStack_98[1] != 0) {
              func_0x000140613c20(uStack_90,puStack_98[1],puStack_98[2]);
            }
            uVar30 = 8;
            lVar33 = lStack_a0;
            lVar36 = 0x18;
LAB_140f78b16:
            func_0x000140613c20(lVar33,lVar36,uVar30);
          }
LAB_140f78b1b:
          LOCK();
          *aplStack_b18[0] = *aplStack_b18[0] + -1;
          UNLOCK();
          if (*aplStack_b18[0] == 0) {
            FUN_140d53bc0(aplStack_b18);
          }
          if (lStack_650 == 0) goto LAB_140f788d0;
        }
      }
    }
    if (lStack_3f8 != 0) {
      FUN_140d685b0(&lStack_3f0);
    }
    LOCK();
    *aplStack_3e8[0] = *aplStack_3e8[0] + -1;
    UNLOCK();
    if (*aplStack_3e8[0] == 0) {
      FUN_140d53bc0(aplStack_3e8);
    }
  }
  if (*plStack_a8 != 0) {
    func_0x000140613c20(plStack_a8[1],*plStack_a8,1);
  }
  if (*plStack_b0 != 0) {
    func_0x000140613c20(plStack_b0[1],*plStack_b0,1);
  }
  return;
}

