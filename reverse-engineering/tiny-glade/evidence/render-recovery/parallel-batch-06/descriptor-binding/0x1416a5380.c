
void FUN_1416a5380(longlong param_1,longlong param_2,longlong param_3,uint param_4,
                  undefined8 param_5,longlong *param_6)

{
  longlong lVar1;
  longlong *plVar2;
  longlong lVar3;
  undefined8 *puVar4;
  undefined1 auVar5 [16];
  longlong lVar6;
  ulonglong uVar7;
  undefined8 *puVar8;
  uint uVar9;
  uint uVar10;
  int iVar11;
  int iVar12;
  int iVar13;
  int iVar14;
  bool bVar15;
  undefined8 *puVar16;
  undefined8 uVar17;
  ulonglong uVar18;
  longlong lVar19;
  longlong lVar20;
  longlong lVar21;
  uint uVar22;
  ushort uVar23;
  undefined8 uVar24;
  char cVar25;
  byte bVar26;
  char cVar28;
  char cVar29;
  char cVar30;
  undefined1 auVar27 [16];
  undefined1 auVar31 [16];
  undefined1 auVar32 [16];
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
  undefined4 uVar33;
  undefined8 uStack_178;
  undefined8 uStack_170;
  undefined8 uStack_168;
  undefined8 uStack_160;
  undefined8 uStack_158;
  undefined8 uStack_150;
  undefined8 uStack_148;
  undefined8 uStack_140;
  undefined8 uStack_138;
  undefined1 auStack_130 [16];
  undefined8 *puStack_120;
  longlong lStack_118;
  longlong lStack_110;
  ulonglong uStack_108;
  longlong lStack_100;
  undefined8 *puStack_f8;
  undefined1 *puStack_f0;
  longlong *plStack_e8;
  undefined8 uStack_e0;
  longlong lStack_d8;
  undefined8 uStack_d0;
  longlong lStack_c8;
  longlong lStack_c0;
  longlong lStack_b8;
  longlong lStack_b0;
  undefined8 uStack_a8;
  uint uStack_9c;
  longlong lStack_98;
  longlong lStack_90;
  ulonglong uStack_88;
  ulonglong uStack_80;
  undefined8 uStack_78;
  ulonglong uStack_70;
  longlong lStack_68;
  undefined8 uStack_60;
  
  uStack_60 = 0xfffffffffffffffe;
  if ((ulonglong)param_4 < *(ulonglong *)(param_3 + 0x10)) {
    uStack_d0 = *(undefined8 *)(param_2 + 0x48);
    lVar20 = (ulonglong)param_4 * 0x78 + *(longlong *)(param_3 + 8);
    lStack_c8 = param_3;
    uStack_9c = param_4;
    func_0x000140d4a3b0(auStack_130);
    lStack_d8 = param_1;
    uVar17 = FUN_140be5a40(lVar20,param_1 + 0x78,param_2 + 0x18);
    lVar19 = *param_6;
    lVar6 = param_6[1];
    uVar7 = param_6[2];
    uStack_e0 = uVar17;
    FUN_1417c69c0(&lStack_c0,param_5,0,auStack_130);
    uStack_a8 = 0;
    uStack_78 = uVar17;
    FUN_1417c6af0(&lStack_98,param_5,0,auStack_130);
    uStack_80 = 0;
    puStack_f8 = &uStack_e0;
    uVar17 = uStack_78;
    lStack_110 = lVar6;
    uStack_108 = uVar7;
    lStack_100 = lVar20;
    puStack_f0 = auStack_130;
    plStack_e8 = &lStack_c0;
    if (lVar19 != lVar6) {
      iVar11 = -(uint)(CONCAT13(unaff_XMM6_Bd,
                                CONCAT12(unaff_XMM6_Bc,CONCAT11(unaff_XMM6_Bb,unaff_XMM6_Ba))) ==
                      CONCAT13(unaff_XMM6_Bd,
                               CONCAT12(unaff_XMM6_Bc,CONCAT11(unaff_XMM6_Bb,unaff_XMM6_Ba))));
      iVar12 = -(uint)(CONCAT13(unaff_XMM6_Bh,
                                CONCAT12(unaff_XMM6_Bg,CONCAT11(unaff_XMM6_Bf,unaff_XMM6_Be))) ==
                      CONCAT13(unaff_XMM6_Bh,
                               CONCAT12(unaff_XMM6_Bg,CONCAT11(unaff_XMM6_Bf,unaff_XMM6_Be))));
      iVar13 = -(uint)(CONCAT13(unaff_XMM6_Bl,
                                CONCAT12(unaff_XMM6_Bk,CONCAT11(unaff_XMM6_Bj,unaff_XMM6_Bi))) ==
                      CONCAT13(unaff_XMM6_Bl,
                               CONCAT12(unaff_XMM6_Bk,CONCAT11(unaff_XMM6_Bj,unaff_XMM6_Bi))));
      iVar14 = -(uint)(CONCAT13(unaff_XMM6_Bp,
                                CONCAT12(unaff_XMM6_Bo,CONCAT11(unaff_XMM6_Bn,unaff_XMM6_Bm))) ==
                      CONCAT13(unaff_XMM6_Bp,
                               CONCAT12(unaff_XMM6_Bo,CONCAT11(unaff_XMM6_Bn,unaff_XMM6_Bm))));
code_r0x0001416a54f9:
      lVar20 = lStack_100;
      lVar6 = lStack_110;
      if (*(longlong *)(lStack_100 + 0x30) == 0) {
        do {
          lVar19 = lVar19 + 0x20;
        } while (lVar19 != lStack_110);
      }
      else {
        lVar3 = lStack_100 + 0x38;
        uVar24 = uVar17;
        do {
          uVar7 = uStack_108;
          lVar1 = lVar19 + 0x20;
          uStack_70 = uStack_108 & 0xffffffff;
          lStack_118 = lVar1;
          lStack_68 = lVar19;
          if (*(longlong *)(lVar20 + 0x30) != 0) {
            uVar18 = FUN_1405b0ed0(lVar3,&uStack_70);
            uVar17 = uStack_78;
            bVar26 = (byte)(uVar18 >> 0x39);
            auVar27 = ZEXT216(CONCAT11(bVar26,bVar26));
            auVar27 = pshuflw(auVar27,auVar27,0);
            lVar21 = 0;
            while( true ) {
              uVar18 = *(ulonglong *)(lVar20 + 0x20) & uVar18;
              auVar5 = *(undefined1 (*) [16])(*(longlong *)(lVar20 + 0x18) + uVar18);
              cVar25 = auVar27[0];
              auVar32[0] = -(auVar5[0] == cVar25);
              cVar28 = auVar27[1];
              auVar32[1] = -(auVar5[1] == cVar28);
              cVar29 = auVar27[2];
              auVar32[2] = -(auVar5[2] == cVar29);
              cVar30 = auVar27[3];
              auVar32[3] = -(auVar5[3] == cVar30);
              auVar32[4] = -(auVar5[4] == cVar25);
              auVar32[5] = -(auVar5[5] == cVar28);
              auVar32[6] = -(auVar5[6] == cVar29);
              auVar32[7] = -(auVar5[7] == cVar30);
              auVar32[8] = -(auVar5[8] == cVar25);
              auVar32[9] = -(auVar5[9] == cVar28);
              auVar32[10] = -(auVar5[10] == cVar29);
              auVar32[0xb] = -(auVar5[0xb] == cVar30);
              auVar32[0xc] = -(auVar5[0xc] == cVar25);
              auVar32[0xd] = -(auVar5[0xd] == cVar28);
              auVar32[0xe] = -(auVar5[0xe] == cVar29);
              auVar32[0xf] = -(auVar5[0xf] == cVar30);
              uVar23 = (ushort)(SUB161(auVar32 >> 7,0) & 1) |
                       (ushort)(SUB161(auVar32 >> 0xf,0) & 1) << 1 |
                       (ushort)(SUB161(auVar32 >> 0x17,0) & 1) << 2 |
                       (ushort)(SUB161(auVar32 >> 0x1f,0) & 1) << 3 |
                       (ushort)(SUB161(auVar32 >> 0x27,0) & 1) << 4 |
                       (ushort)(SUB161(auVar32 >> 0x2f,0) & 1) << 5 |
                       (ushort)(SUB161(auVar32 >> 0x37,0) & 1) << 6 |
                       (ushort)(SUB161(auVar32 >> 0x3f,0) & 1) << 7 |
                       (ushort)(SUB161(auVar32 >> 0x47,0) & 1) << 8 |
                       (ushort)(SUB161(auVar32 >> 0x4f,0) & 1) << 9 |
                       (ushort)(SUB161(auVar32 >> 0x57,0) & 1) << 10 |
                       (ushort)(SUB161(auVar32 >> 0x5f,0) & 1) << 0xb |
                       (ushort)(SUB161(auVar32 >> 0x67,0) & 1) << 0xc |
                       (ushort)(SUB161(auVar32 >> 0x6f,0) & 1) << 0xd |
                       (ushort)(SUB161(auVar32 >> 0x77,0) & 1) << 0xe |
                       (ushort)(auVar32[0xf] >> 7) << 0xf;
              uVar22 = (uint)uVar23;
              while (uVar23 != 0) {
                uVar9 = 0;
                for (uVar10 = uVar22; (uVar10 & 1) == 0; uVar10 = uVar10 >> 1 | 0x80000000) {
                  uVar9 = uVar9 + 1;
                }
                if ((int)uStack_70 ==
                    *(int *)(*(longlong *)(lVar20 + 0x18) + -0x18 +
                            (uVar9 + uVar18 & *(ulonglong *)(lVar20 + 0x20)) * -0x18)) {
                  uStack_108 = uVar7 + 1;
                  if (lVar19 == 0) goto code_r0x0001416a56b9;
                  lStack_68 = lVar19;
                  FUN_140b06020(&uStack_170,&puStack_f8,&uStack_70);
                  if (uStack_80 == uStack_88) {
                    FUN_1417c6c10(&lStack_98,uStack_80,1,1);
                  }
                  lVar19 = uStack_80 * 0x40;
                  puVar4 = (undefined8 *)(lStack_98 + 0x30 + lVar19);
                  *puVar4 = uStack_140;
                  puVar4[1] = uStack_138;
                  puVar4 = (undefined8 *)(lStack_98 + 0x20 + lVar19);
                  *puVar4 = uStack_150;
                  puVar4[1] = uStack_148;
                  puVar4 = (undefined8 *)(lStack_98 + 0x10 + lVar19);
                  *puVar4 = uStack_160;
                  puVar4[1] = uStack_158;
                  *(undefined8 *)(lStack_98 + lVar19) = uStack_170;
                  ((undefined8 *)(lStack_98 + lVar19))[1] = uStack_168;
                  uStack_80 = uStack_80 + 1;
                  lVar19 = lStack_118;
                  if (lStack_118 != lStack_110) goto code_r0x0001416a54f9;
                  goto code_r0x0001416a56b9;
                }
                uVar23 = (ushort)(uVar22 - 1) & (ushort)uVar22;
                uVar22 = CONCAT22((short)(uVar22 - 1 >> 0x10),uVar23);
                uVar24 = uStack_78;
              }
              auVar31[0] = -(auVar5[0] == (char)iVar11);
              auVar31[1] = -(auVar5[1] == (char)((uint)iVar11 >> 8));
              auVar31[2] = -(auVar5[2] == (char)((uint)iVar11 >> 0x10));
              auVar31[3] = -(auVar5[3] == (char)((uint)iVar11 >> 0x18));
              auVar31[4] = -(auVar5[4] == (char)iVar12);
              auVar31[5] = -(auVar5[5] == (char)((uint)iVar12 >> 8));
              auVar31[6] = -(auVar5[6] == (char)((uint)iVar12 >> 0x10));
              auVar31[7] = -(auVar5[7] == (char)((uint)iVar12 >> 0x18));
              auVar31[8] = -(auVar5[8] == (char)iVar13);
              auVar31[9] = -(auVar5[9] == (char)((uint)iVar13 >> 8));
              auVar31[10] = -(auVar5[10] == (char)((uint)iVar13 >> 0x10));
              auVar31[0xb] = -(auVar5[0xb] == (char)((uint)iVar13 >> 0x18));
              auVar31[0xc] = -(auVar5[0xc] == (char)iVar14);
              auVar31[0xd] = -(auVar5[0xd] == (char)((uint)iVar14 >> 8));
              auVar31[0xe] = -(auVar5[0xe] == (char)((uint)iVar14 >> 0x10));
              auVar31[0xf] = -(auVar5[0xf] == (char)((uint)iVar14 >> 0x18));
              if ((((((((((((((((SUB161(auVar31 >> 7,0) & 1) != 0 ||
                               (SUB161(auVar31 >> 0xf,0) & 1) != 0) ||
                              (SUB161(auVar31 >> 0x17,0) & 1) != 0) ||
                             (SUB161(auVar31 >> 0x1f,0) & 1) != 0) ||
                            (SUB161(auVar31 >> 0x27,0) & 1) != 0) ||
                           (SUB161(auVar31 >> 0x2f,0) & 1) != 0) ||
                          (SUB161(auVar31 >> 0x37,0) & 1) != 0) ||
                         (SUB161(auVar31 >> 0x3f,0) & 1) != 0) ||
                        (SUB161(auVar31 >> 0x47,0) & 1) != 0) ||
                       (SUB161(auVar31 >> 0x4f,0) & 1) != 0) || (SUB161(auVar31 >> 0x57,0) & 1) != 0
                      ) || (SUB161(auVar31 >> 0x5f,0) & 1) != 0) ||
                    (SUB161(auVar31 >> 0x67,0) & 1) != 0) || (SUB161(auVar31 >> 0x6f,0) & 1) != 0)
                  || (SUB161(auVar31 >> 0x77,0) & 1) != 0) || auVar31[0xf] < '\0') break;
              uVar18 = uVar18 + lVar21 + 0x10;
              lVar21 = lVar21 + 0x10;
            }
          }
          uVar17 = uVar24;
          uStack_108 = uVar7 + 1;
          uVar24 = uVar17;
          lVar19 = lVar1;
        } while (lVar1 != lVar6);
      }
    }
code_r0x0001416a56b9:
    lVar19 = lStack_d8;
    uVar33 = 0;
    (**(code **)(lStack_d8 + 0x268))
              (*(undefined8 *)(lStack_d8 + 0x640),uStack_80 & 0xffffffff,lStack_98,0,8);
    LOCK();
    plVar2 = (longlong *)(lVar19 + 0xab0);
    *plVar2 = *plVar2 + 1;
    UNLOCK();
    if (uStack_88 != 0) {
      lVar6 = *(longlong *)(*(longlong *)(lStack_90 + 0x10) + 0x20);
      if (lVar6 == lStack_98) {
        *(ulonglong *)(*(longlong *)(lStack_90 + 0x10) + 0x20) = lVar6 + uStack_88 * 0x40;
      }
    }
    uStack_178 = uVar17;
    (**(code **)(lVar19 + 0x328))
              (uStack_d0,*(undefined4 *)(lStack_c8 + 0x30),*(undefined8 *)(lStack_c8 + 0x20),
               uStack_9c,CONCAT44(uVar33,1),&uStack_178,(int)uStack_a8,lStack_c0);
    puVar4 = puRam000000014362c3e0;
    if (lStack_b0 != 0) {
      lVar19 = *(longlong *)(*(longlong *)(lStack_b8 + 0x10) + 0x20);
      if (lVar19 == lStack_c0) {
        *(longlong *)(*(longlong *)(lStack_b8 + 0x10) + 0x20) = lVar19 + lStack_b0 * 4;
        puVar4 = puRam000000014362c3e0;
      }
    }
    while (puVar16 = puRam000000014362c3e0, bVar15 = puStack_120 != puRam000000014362c3e0,
          puRam000000014362c3e0 = puVar4, bVar15) {
      puVar8 = (undefined8 *)puStack_120[3];
      func_0x000140613c20(*puStack_120,puStack_120[2],puStack_120[1]);
      puVar4 = puRam000000014362c3e0;
      puStack_120 = puVar8;
      puRam000000014362c3e0 = puVar16;
    }
  }
  return;
}

