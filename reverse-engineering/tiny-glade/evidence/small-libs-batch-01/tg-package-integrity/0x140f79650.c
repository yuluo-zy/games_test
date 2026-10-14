
/* WARNING: Removing unreachable block (ram,0x000140f7997e) */
/* WARNING: Removing unreachable block (ram,0x000140f7998f) */
/* WARNING: Removing unreachable block (ram,0x000140f799b8) */
/* WARNING: Removing unreachable block (ram,0x000140f799c1) */
/* WARNING: Removing unreachable block (ram,0x000140f799d8) */
/* WARNING: Removing unreachable block (ram,0x000140f799e1) */

void FUN_140f79650(undefined8 param_1,undefined8 param_2,longlong *param_3,longlong *param_4,
                  longlong *param_5)

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
  undefined *puVar25;
  char cVar26;
  ushort uVar27;
  int iVar28;
  ulonglong uVar29;
  ulonglong uVar30;
  longlong lVar31;
  longlong **pplVar32;
  uint uVar33;
  longlong lVar34;
  undefined8 uVar35;
  byte bVar36;
  undefined1 auVar37 [16];
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
  char cVar38;
  char cVar39;
  char cVar40;
  undefined1 auVar41 [16];
  longlong *aplStack_b00 [75];
  byte bStack_8a8;
  undefined7 uStack_8a7;
  longlong lStack_8a0;
  undefined1 auStack_898 [608];
  longlong lStack_638;
  longlong lStack_630;
  longlong lStack_3e0;
  longlong lStack_3d8;
  longlong *aplStack_3d0 [76];
  undefined8 uStack_170;
  longlong lStack_168;
  longlong lStack_160;
  longlong lStack_158;
  longlong lStack_140;
  longlong lStack_138;
  longlong *plStack_130;
  longlong lStack_128;
  longlong lStack_120;
  longlong *plStack_118;
  undefined *puStack_110;
  ulonglong uStack_108;
  undefined4 uStack_100;
  undefined4 uStack_fc;
  undefined4 uStack_f8;
  undefined4 uStack_f4;
  undefined8 *puStack_f0;
  undefined8 uStack_e8;
  longlong lStack_e0;
  ulonglong uStack_d8;
  longlong lStack_d0;
  undefined *puStack_c8;
  ulonglong uStack_c0;
  undefined4 uStack_b8;
  undefined4 uStack_b4;
  undefined4 uStack_b0;
  undefined4 uStack_ac;
  longlong *plStack_a8;
  longlong lStack_a0;
  undefined8 *puStack_98;
  undefined8 uStack_90;
  undefined1 uStack_83;
  undefined1 uStack_82;
  undefined1 uStack_81;
  undefined8 uStack_80;
  
  uStack_80 = 0xfffffffffffffffe;
  plStack_a8 = param_3;
  FUN_140d68f80(&bStack_8a8,param_1,param_2);
  lStack_3e0 = CONCAT71(uStack_8a7,bStack_8a8);
  if (lStack_3e0 == 2) {
    if (((uint)lStack_8a0 & 3) == 1) {
      lStack_a0 = lStack_8a0 + -1;
      uStack_90 = *(undefined8 *)(lStack_8a0 + -1);
      puStack_98 = *(undefined8 **)(lStack_8a0 + 7);
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
    func_0x000142923c00(aplStack_3d0,auStack_898,0x260);
    lStack_3d8 = lStack_8a0;
    lStack_128 = param_4[3];
    plStack_118 = param_4 + 4;
    lStack_120 = *param_4;
    uStack_d8 = param_4[1];
    lStack_d0 = lStack_120 + -0x18;
    lStack_140 = plStack_a8[1];
    lStack_138 = plStack_a8[2];
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
    plStack_130 = param_4;
    while (FUN_140d59e80(&bStack_8a8,&lStack_3e0), (bStack_8a8 & 1) != 0) {
      func_0x000142923c00(&lStack_638,&lStack_8a0,600);
      if (lStack_638 == 0) {
LAB_140f798e0:
        if (((uint)lStack_630 & 3) == 1) {
          lStack_a0 = lStack_630 + -1;
          uStack_90 = *(undefined8 *)(lStack_630 + -1);
          puStack_98 = *(undefined8 **)(lStack_630 + 7);
          if ((code *)*puStack_98 != (code *)0x0) {
            (*(code *)*puStack_98)(uStack_90);
          }
          goto LAB_140f79927;
        }
      }
      else {
        func_0x000142923c00(aplStack_b00,&lStack_8a0,600);
        FUN_140d5a110(&puStack_c8,aplStack_b00);
        uVar29 = uStack_c0;
        puVar25 = puStack_c8;
        if ((int)puStack_c8 == 0) {
          pplVar32 = aplStack_b00;
          FUN_140d5a130(&lStack_160,aplStack_b00);
          uVar35 = FUN_140d76ed0(&lStack_160);
          FUN_140d58e70(&puStack_c8,uVar35,pplVar32);
          uVar30 = uStack_c0;
          if (((ulonglong)puStack_c8 & 1) != 0) {
LAB_140f79b0d:
            if (lStack_160 != 0) {
              uVar35 = 1;
              lVar31 = lStack_158;
              lVar34 = lStack_160;
              goto LAB_140f79b26;
            }
            goto LAB_140f79b2b;
          }
          uVar35 = CONCAT44(uStack_b4,uStack_b8);
          FUN_140d59f10(&puStack_110,aplStack_b00);
          uStack_81 = 1;
          FUN_141d45aa0(&puStack_f0,lStack_140,lStack_138,uVar30,uVar35);
          lVar31 = lStack_e0;
          uStack_90 = uStack_e8;
          uStack_170 = uStack_e8;
          lStack_168 = lStack_e0;
          puStack_c8 = &UNK_142bab8a1;
          uStack_c0 = 3;
          uStack_82 = 1;
          uStack_83 = 1;
          cVar26 = FUN_140fcb800(&uStack_170,&puStack_c8);
          if (cVar26 != '\0') {
            if (puStack_f0 != (undefined8 *)0x0) {
              func_0x000140613c20(uStack_90,puStack_f0,1);
            }
            if (puStack_110 != (undefined *)0x0) {
              func_0x000140613c20(uStack_108,puStack_110,1);
            }
            goto LAB_140f79b0d;
          }
          if ((((ulonglong)puVar25 & 0x40000000000) == 0) || ((uVar29 & 0x20000000) == 0)) {
            if (((ulonglong)puVar25 & 0x1000000000) == 0) {
              if (lStack_128 != 0) {
                uVar29 = FUN_1405b1a70(plStack_118,uStack_90,lVar31);
                bVar36 = (byte)(uVar29 >> 0x39);
                auVar37 = ZEXT216(CONCAT11(bVar36,bVar36));
                auVar37 = pshuflw(auVar37,auVar37,0);
                cVar26 = auVar37[0];
                cVar38 = auVar37[1];
                cVar39 = auVar37[2];
                cVar40 = auVar37[3];
                lVar34 = 0;
                while( true ) {
                  uVar29 = uVar29 & uStack_d8;
                  pcVar1 = (char *)(lStack_120 + uVar29);
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
                  auVar37[0] = -(cVar9 == cVar26);
                  auVar37[1] = -(cVar10 == cVar38);
                  auVar37[2] = -(cVar11 == cVar39);
                  auVar37[3] = -(cVar12 == cVar40);
                  auVar37[4] = -(cVar13 == cVar26);
                  auVar37[5] = -(cVar14 == cVar38);
                  auVar37[6] = -(cVar15 == cVar39);
                  auVar37[7] = -(cVar16 == cVar40);
                  auVar37[8] = -(cVar17 == cVar26);
                  auVar37[9] = -(cVar18 == cVar38);
                  auVar37[10] = -(cVar19 == cVar39);
                  auVar37[0xb] = -(cVar20 == cVar40);
                  auVar37[0xc] = -(cVar21 == cVar26);
                  auVar37[0xd] = -(cVar22 == cVar38);
                  auVar37[0xe] = -(cVar23 == cVar39);
                  auVar37[0xf] = -(cVar24 == cVar40);
                  uVar27 = (ushort)(SUB161(auVar37 >> 7,0) & 1) |
                           (ushort)(SUB161(auVar37 >> 0xf,0) & 1) << 1 |
                           (ushort)(SUB161(auVar37 >> 0x17,0) & 1) << 2 |
                           (ushort)(SUB161(auVar37 >> 0x1f,0) & 1) << 3 |
                           (ushort)(SUB161(auVar37 >> 0x27,0) & 1) << 4 |
                           (ushort)(SUB161(auVar37 >> 0x2f,0) & 1) << 5 |
                           (ushort)(SUB161(auVar37 >> 0x37,0) & 1) << 6 |
                           (ushort)(SUB161(auVar37 >> 0x3f,0) & 1) << 7 |
                           (ushort)(SUB161(auVar37 >> 0x47,0) & 1) << 8 |
                           (ushort)(SUB161(auVar37 >> 0x4f,0) & 1) << 9 |
                           (ushort)(SUB161(auVar37 >> 0x57,0) & 1) << 10 |
                           (ushort)(SUB161(auVar37 >> 0x5f,0) & 1) << 0xb |
                           (ushort)(SUB161(auVar37 >> 0x67,0) & 1) << 0xc |
                           (ushort)(SUB161(auVar37 >> 0x6f,0) & 1) << 0xd |
                           (ushort)(SUB161(auVar37 >> 0x77,0) & 1) << 0xe |
                           (ushort)(auVar37[0xf] >> 7) << 0xf;
                  uVar33 = (uint)uVar27;
                  while (uVar27 != 0) {
                    uVar3 = 0;
                    for (uVar4 = uVar33; (uVar4 & 1) == 0; uVar4 = uVar4 >> 1 | 0x80000000) {
                      uVar3 = uVar3 + 1;
                    }
                    uVar30 = uVar3 + uVar29 & uStack_d8;
                    if ((*(longlong *)(lStack_d0 + 0x10 + uVar30 * -0x18) == lVar31) &&
                       (iVar28 = func_0x000142923bf0(uStack_90,
                                                     *(undefined8 *)(lStack_d0 + uVar30 * -0x18 + 8)
                                                     ,lVar31), iVar28 == 0)) goto LAB_140f79b56;
                    uVar27 = (ushort)(uVar33 - 1) & (ushort)uVar33;
                    uVar33 = CONCAT22((short)(uVar33 - 1 >> 0x10),uVar27);
                  }
                  auVar41[0] = -(cVar9 == (char)iVar5);
                  auVar41[1] = -(cVar10 == (char)((uint)iVar5 >> 8));
                  auVar41[2] = -(cVar11 == (char)((uint)iVar5 >> 0x10));
                  auVar41[3] = -(cVar12 == (char)((uint)iVar5 >> 0x18));
                  auVar41[4] = -(cVar13 == (char)iVar6);
                  auVar41[5] = -(cVar14 == (char)((uint)iVar6 >> 8));
                  auVar41[6] = -(cVar15 == (char)((uint)iVar6 >> 0x10));
                  auVar41[7] = -(cVar16 == (char)((uint)iVar6 >> 0x18));
                  auVar41[8] = -(cVar17 == (char)iVar7);
                  auVar41[9] = -(cVar18 == (char)((uint)iVar7 >> 8));
                  auVar41[10] = -(cVar19 == (char)((uint)iVar7 >> 0x10));
                  auVar41[0xb] = -(cVar20 == (char)((uint)iVar7 >> 0x18));
                  auVar41[0xc] = -(cVar21 == (char)iVar8);
                  auVar41[0xd] = -(cVar22 == (char)((uint)iVar8 >> 8));
                  auVar41[0xe] = -(cVar23 == (char)((uint)iVar8 >> 0x10));
                  auVar41[0xf] = -(cVar24 == (char)((uint)iVar8 >> 0x18));
                  if ((((((((((((((((SUB161(auVar41 >> 7,0) & 1) != 0 ||
                                   (SUB161(auVar41 >> 0xf,0) & 1) != 0) ||
                                  (SUB161(auVar41 >> 0x17,0) & 1) != 0) ||
                                 (SUB161(auVar41 >> 0x1f,0) & 1) != 0) ||
                                (SUB161(auVar41 >> 0x27,0) & 1) != 0) ||
                               (SUB161(auVar41 >> 0x2f,0) & 1) != 0) ||
                              (SUB161(auVar41 >> 0x37,0) & 1) != 0) ||
                             (SUB161(auVar41 >> 0x3f,0) & 1) != 0) ||
                            (SUB161(auVar41 >> 0x47,0) & 1) != 0) ||
                           (SUB161(auVar41 >> 0x4f,0) & 1) != 0) ||
                          (SUB161(auVar41 >> 0x57,0) & 1) != 0) ||
                         (SUB161(auVar41 >> 0x5f,0) & 1) != 0) ||
                        (SUB161(auVar41 >> 0x67,0) & 1) != 0) ||
                       (SUB161(auVar41 >> 0x6f,0) & 1) != 0) || (SUB161(auVar41 >> 0x77,0) & 1) != 0
                      ) || auVar41[0xf] < '\0') break;
                  uVar29 = uVar29 + lVar34 + 0x10;
                  lVar34 = lVar34 + 0x10;
                }
              }
              puStack_98 = puStack_f0;
              lVar34 = param_5[2];
              if (lVar34 == *param_5) {
                FUN_140562db0(param_5,&UNK_142bab8d0);
              }
              lVar2 = param_5[1];
              *(undefined8 **)(lVar2 + lVar34 * 0x18) = puStack_98;
              *(undefined8 *)(lVar2 + 8 + lVar34 * 0x18) = uStack_90;
              *(longlong *)(lVar2 + 0x10 + lVar34 * 0x18) = lVar31;
              param_5[2] = lVar34 + 1;
              goto joined_r0x000140f79dbb;
            }
            uStack_b8 = uStack_100;
            uStack_b4 = uStack_fc;
            uStack_b0 = uStack_f8;
            uStack_ac = uStack_f4;
            puStack_c8 = puStack_110;
            uStack_c0 = uStack_108;
            uStack_82 = 0;
            uStack_83 = 0;
            FUN_140f78630(&puStack_c8,&puStack_f0,plStack_130,param_5);
          }
          else {
LAB_140f79b56:
            if (puStack_f0 != (undefined8 *)0x0) {
              func_0x000140613c20(uStack_90,puStack_f0,1);
            }
joined_r0x000140f79dbb:
            if (puStack_110 != (undefined *)0x0) {
              func_0x000140613c20(uStack_108,puStack_110,1);
            }
          }
          if (lStack_160 != 0) {
            func_0x000140613c20(lStack_158,lStack_160,1);
          }
          LOCK();
          *aplStack_b00[0] = *aplStack_b00[0] + -1;
          UNLOCK();
          if (*aplStack_b00[0] == 0) {
            FUN_140d53bc0(aplStack_b00);
          }
          if ((lStack_638 == 0) && (((uint)lStack_630 & 3) == 1)) {
            lStack_a0 = lStack_630 + -1;
            uStack_90 = *(undefined8 *)(lStack_630 + -1);
            puStack_98 = *(undefined8 **)(lStack_630 + 7);
            if ((code *)*puStack_98 != (code *)0x0) {
              (*(code *)*puStack_98)(uStack_90);
            }
LAB_140f79927:
            if (puStack_98[1] != 0) {
              func_0x000140613c20(uStack_90,puStack_98[1],puStack_98[2]);
            }
            func_0x000140613c20(lStack_a0,0x18,8);
          }
        }
        else {
          if (((uint)uStack_c0 & 3) == 1) {
            lStack_a0 = uStack_c0 - 1;
            uStack_90 = *(undefined8 *)(uStack_c0 - 1);
            puStack_98 = *(undefined8 **)(uStack_c0 + 7);
            if ((code *)*puStack_98 != (code *)0x0) {
              (*(code *)*puStack_98)(uStack_90);
            }
            if (puStack_98[1] != 0) {
              func_0x000140613c20(uStack_90,puStack_98[1],puStack_98[2]);
            }
            uVar35 = 8;
            lVar31 = lStack_a0;
            lVar34 = 0x18;
LAB_140f79b26:
            func_0x000140613c20(lVar31,lVar34,uVar35);
          }
LAB_140f79b2b:
          LOCK();
          *aplStack_b00[0] = *aplStack_b00[0] + -1;
          UNLOCK();
          if (*aplStack_b00[0] == 0) {
            FUN_140d53bc0(aplStack_b00);
          }
          if (lStack_638 == 0) goto LAB_140f798e0;
        }
      }
    }
    if (lStack_3e0 != 0) {
      FUN_140d685b0(&lStack_3d8);
    }
    LOCK();
    *aplStack_3d0[0] = *aplStack_3d0[0] + -1;
    UNLOCK();
    if (*aplStack_3d0[0] == 0) {
      FUN_140d53bc0(aplStack_3d0);
    }
  }
  if (*plStack_a8 != 0) {
    func_0x000140613c20(plStack_a8[1],*plStack_a8,1);
  }
  return;
}

