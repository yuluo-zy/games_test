
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void __cdecl
country_core::utils::dirty_tiles::DirtyTiles::
iter_tiles<country_core::systems::wall::wall_state::publish_public_wall_state::gather_lazy_update_sources::closure_env_0>
          (void)

{
  longlong *plVar1;
  longlong lVar2;
  longlong *plVar3;
  longlong *plVar4;
  longlong lVar5;
  uint uVar6;
  uint uVar7;
  int iVar8;
  int iVar9;
  int iVar10;
  int iVar11;
  float fVar12;
  undefined1 auVar13 [16];
  undefined1 auVar14 [16];
  longlong in_RCX;
  longlong lVar15;
  ulonglong uVar16;
  longlong lVar17;
  ulonglong uVar18;
  longlong *in_RDX;
  ulonglong uVar19;
  ulonglong uVar20;
  ulonglong uVar21;
  longlong lVar22;
  uint uVar23;
  ushort uVar24;
  longlong lVar25;
  longlong *plVar26;
  byte bVar27;
  char cVar28;
  char cVar30;
  char cVar31;
  char cVar32;
  undefined1 auVar29 [16];
  undefined1 auVar33 [16];
  undefined1 auVar34 [16];
  undefined1 auVar35 [16];
  undefined1 auVar36 [16];
  undefined1 auVar37 [16];
  undefined1 auVar38 [16];
  undefined1 auVar39 [16];
  undefined1 auVar40 [16];
  undefined1 unaff_XMM9_Ba;
  undefined1 unaff_XMM9_Bb;
  char cVar41;
  undefined1 unaff_XMM9_Bc;
  char cVar42;
  undefined1 unaff_XMM9_Bd;
  char cVar43;
  undefined1 unaff_XMM9_Be;
  undefined1 unaff_XMM9_Bf;
  char cVar44;
  undefined1 unaff_XMM9_Bg;
  char cVar45;
  undefined1 unaff_XMM9_Bh;
  char cVar46;
  undefined1 unaff_XMM9_Bi;
  undefined1 unaff_XMM9_Bj;
  char cVar47;
  undefined1 unaff_XMM9_Bk;
  char cVar48;
  undefined1 unaff_XMM9_Bl;
  char cVar49;
  undefined1 unaff_XMM9_Bm;
  undefined1 unaff_XMM9_Bn;
  char cVar50;
  undefined1 unaff_XMM9_Bo;
  char cVar51;
  undefined1 unaff_XMM9_Bp;
  char cVar52;
  longlong local_128;
  longlong local_118;
  longlong lStack_110;
  ulonglong local_108;
  ulonglong uStack_100;
  
  auVar14 = ___xmm_7465646279746573646f72616e646f6d;
  auVar13 = ___xmm_6c7967656e657261736f6d6570736575;
  fVar12 = __real_3f000000;
  lVar2 = *in_RDX;
  plVar3 = (longlong *)in_RDX[1];
  plVar4 = (longlong *)in_RDX[2];
  lVar15 = 0;
  iVar8 = -(uint)(CONCAT13(unaff_XMM9_Bd,
                           CONCAT12(unaff_XMM9_Bc,CONCAT11(unaff_XMM9_Bb,unaff_XMM9_Ba))) ==
                 CONCAT13(unaff_XMM9_Bd,
                          CONCAT12(unaff_XMM9_Bc,CONCAT11(unaff_XMM9_Bb,unaff_XMM9_Ba))));
  cVar41 = (char)((uint)iVar8 >> 8);
  cVar42 = (char)((uint)iVar8 >> 0x10);
  cVar43 = (char)((uint)iVar8 >> 0x18);
  iVar9 = -(uint)(CONCAT13(unaff_XMM9_Bh,
                           CONCAT12(unaff_XMM9_Bg,CONCAT11(unaff_XMM9_Bf,unaff_XMM9_Be))) ==
                 CONCAT13(unaff_XMM9_Bh,
                          CONCAT12(unaff_XMM9_Bg,CONCAT11(unaff_XMM9_Bf,unaff_XMM9_Be))));
  cVar44 = (char)((uint)iVar9 >> 8);
  cVar45 = (char)((uint)iVar9 >> 0x10);
  cVar46 = (char)((uint)iVar9 >> 0x18);
  iVar10 = -(uint)(CONCAT13(unaff_XMM9_Bl,
                            CONCAT12(unaff_XMM9_Bk,CONCAT11(unaff_XMM9_Bj,unaff_XMM9_Bi))) ==
                  CONCAT13(unaff_XMM9_Bl,
                           CONCAT12(unaff_XMM9_Bk,CONCAT11(unaff_XMM9_Bj,unaff_XMM9_Bi))));
  cVar47 = (char)((uint)iVar10 >> 8);
  cVar48 = (char)((uint)iVar10 >> 0x10);
  cVar49 = (char)((uint)iVar10 >> 0x18);
  iVar11 = -(uint)(CONCAT13(unaff_XMM9_Bp,
                            CONCAT12(unaff_XMM9_Bo,CONCAT11(unaff_XMM9_Bn,unaff_XMM9_Bm))) ==
                  CONCAT13(unaff_XMM9_Bp,
                           CONCAT12(unaff_XMM9_Bo,CONCAT11(unaff_XMM9_Bn,unaff_XMM9_Bm))));
  cVar50 = (char)((uint)iVar11 >> 8);
  cVar51 = (char)((uint)iVar11 >> 0x10);
  cVar52 = (char)((uint)iVar11 >> 0x18);
  local_128 = 0;
  do {
    uVar20 = *(ulonglong *)(in_RCX + lVar15);
    if (uVar20 != 0) {
      do {
        uVar16 = 0;
        for (uVar18 = uVar20; (uVar18 & 1) == 0; uVar18 = uVar18 >> 1 | 0x8000000000000000) {
          uVar16 = uVar16 + 1;
        }
        uVar16 = uVar16 | local_128 << 6;
        if (*(ulonglong *)(lVar2 + 0x10) <= uVar16) {
                    /* WARNING: Subroutine does not return */
          core::panicking::panic_bounds_check();
        }
        lVar5 = *(longlong *)(lVar2 + 8);
        lVar17 = uVar16 * 0x30;
        lVar22 = *(longlong *)(lVar5 + 0x28 + lVar17);
        if (lVar22 != 0) {
          if (plVar3[3] != 0) {
            plVar26 = *(longlong **)(lVar5 + 0x20 + lVar17);
            plVar1 = plVar26 + lVar22;
            do {
              lVar5 = *plVar26;
              auVar29 = *(undefined1 (*) [16])(plVar3 + 4);
              auVar33._0_8_ = auVar29._0_8_;
              auVar33._8_4_ = auVar29._0_4_;
              auVar33._12_4_ = auVar29._4_4_;
              auVar33 = auVar33 ^ auVar13;
              auVar34._0_8_ = auVar29._8_8_;
              auVar34._8_4_ = auVar29._8_4_;
              auVar34._12_4_ = auVar29._12_4_;
              auVar34 = auVar34 ^ auVar14;
              core::hash::sip::impl$5::write<core::hash::sip::Sip13Rounds>();
              local_108 = auVar34._0_8_;
              uStack_100 = auVar34._8_8_;
              local_118 = auVar33._0_8_;
              uVar21 = local_118 + local_108;
              lStack_110 = auVar33._8_8_;
              uVar18 = (local_108 << 0xd | local_108 >> 0x33) ^ uVar21;
              uVar16 = (uStack_100 << 0x10 | uStack_100 >> 0x30) ^ lStack_110 + uStack_100;
              uVar19 = lStack_110 + uStack_100 + uVar18;
              uVar21 = (uVar21 << 0x20 | uVar21 >> 0x20) + uVar16;
              uVar18 = (uVar18 << 0x11 | uVar18 >> 0x2f) ^ uVar19;
              uVar16 = (uVar16 << 0x15 | uVar16 >> 0x2b) ^ uVar21;
              uVar21 = uVar21 + uVar18;
              uVar19 = ((uVar19 << 0x20 | uVar19 >> 0x20) ^ 0xff) + uVar16;
              uVar18 = (uVar18 << 0xd | uVar18 >> 0x33) ^ uVar21;
              uVar16 = (uVar16 << 0x10 | uVar16 >> 0x30) ^ uVar19;
              uVar19 = uVar19 + uVar18;
              uVar21 = (uVar21 << 0x20 | uVar21 >> 0x20) + uVar16;
              uVar18 = (uVar18 << 0x11 | uVar18 >> 0x2f) ^ uVar19;
              uVar16 = (uVar16 << 0x15 | uVar16 >> 0x2b) ^ uVar21;
              uVar21 = uVar21 + uVar18;
              uVar19 = (uVar19 << 0x20 | uVar19 >> 0x20) + uVar16;
              uVar18 = (uVar18 << 0xd | uVar18 >> 0x33) ^ uVar21;
              uVar16 = (uVar16 << 0x10 | uVar16 >> 0x30) ^ uVar19;
              uVar19 = uVar19 + uVar18;
              uVar21 = (uVar21 << 0x20 | uVar21 >> 0x20) + uVar16;
              uVar18 = (uVar18 << 0x11 | uVar18 >> 0x2f) ^ uVar19;
              uVar16 = (uVar16 << 0x15 | uVar16 >> 0x2b) ^ uVar21;
              uVar19 = (uVar19 << 0x20 | uVar19 >> 0x20) + uVar16;
              uVar18 = (uVar18 << 0xd | uVar18 >> 0x33) ^ uVar21 + uVar18;
              uVar16 = (uVar16 << 0x10 | uVar16 >> 0x30) ^ uVar19;
              uVar19 = uVar19 + uVar18;
              uVar19 = (uVar19 << 0x20 | uVar19 >> 0x20) ^ (uVar18 << 0x11 | uVar18 >> 0x2f) ^
                       (uVar16 << 0x15 | uVar16 >> 0x2b) ^ uVar19;
              bVar27 = (byte)(uVar19 >> 0x39);
              auVar29 = ZEXT216(CONCAT11(bVar27,bVar27));
              auVar29 = pshuflw(auVar29,auVar29,0);
              lVar22 = 0;
              while( true ) {
                uVar19 = uVar19 & plVar3[1];
                auVar33 = *(undefined1 (*) [16])(*plVar3 + uVar19);
                cVar28 = auVar29[0];
                auVar39[0] = -(auVar33[0] == cVar28);
                cVar30 = auVar29[1];
                auVar39[1] = -(auVar33[1] == cVar30);
                cVar31 = auVar29[2];
                auVar39[2] = -(auVar33[2] == cVar31);
                cVar32 = auVar29[3];
                auVar39[3] = -(auVar33[3] == cVar32);
                auVar39[4] = -(auVar33[4] == cVar28);
                auVar39[5] = -(auVar33[5] == cVar30);
                auVar39[6] = -(auVar33[6] == cVar31);
                auVar39[7] = -(auVar33[7] == cVar32);
                auVar39[8] = -(auVar33[8] == cVar28);
                auVar39[9] = -(auVar33[9] == cVar30);
                auVar39[10] = -(auVar33[10] == cVar31);
                auVar39[0xb] = -(auVar33[0xb] == cVar32);
                auVar39[0xc] = -(auVar33[0xc] == cVar28);
                auVar39[0xd] = -(auVar33[0xd] == cVar30);
                auVar39[0xe] = -(auVar33[0xe] == cVar31);
                auVar39[0xf] = -(auVar33[0xf] == cVar32);
                uVar24 = (ushort)(SUB161(auVar39 >> 7,0) & 1) |
                         (ushort)(SUB161(auVar39 >> 0xf,0) & 1) << 1 |
                         (ushort)(SUB161(auVar39 >> 0x17,0) & 1) << 2 |
                         (ushort)(SUB161(auVar39 >> 0x1f,0) & 1) << 3 |
                         (ushort)(SUB161(auVar39 >> 0x27,0) & 1) << 4 |
                         (ushort)(SUB161(auVar39 >> 0x2f,0) & 1) << 5 |
                         (ushort)(SUB161(auVar39 >> 0x37,0) & 1) << 6 |
                         (ushort)(SUB161(auVar39 >> 0x3f,0) & 1) << 7 |
                         (ushort)(SUB161(auVar39 >> 0x47,0) & 1) << 8 |
                         (ushort)(SUB161(auVar39 >> 0x4f,0) & 1) << 9 |
                         (ushort)(SUB161(auVar39 >> 0x57,0) & 1) << 10 |
                         (ushort)(SUB161(auVar39 >> 0x5f,0) & 1) << 0xb |
                         (ushort)(SUB161(auVar39 >> 0x67,0) & 1) << 0xc |
                         (ushort)(SUB161(auVar39 >> 0x6f,0) & 1) << 0xd |
                         (ushort)(SUB161(auVar39 >> 0x77,0) & 1) << 0xe |
                         (ushort)(auVar39[0xf] >> 7) << 0xf;
                uVar23 = (uint)uVar24;
                while (uVar24 != 0) {
                  uVar6 = 0;
                  for (uVar7 = uVar23; (uVar7 & 1) == 0; uVar7 = uVar7 >> 1 | 0x80000000) {
                    uVar6 = uVar6 + 1;
                  }
                  if (*(longlong *)(*plVar3 + -0x78 + (uVar6 + uVar19 & plVar3[1]) * -0x78) == lVar5
                     ) {
                    if (plVar4[3] == 0) goto LAB_140adf400;
                    auVar29 = *(undefined1 (*) [16])(plVar4 + 4);
                    auVar36._0_8_ = auVar29._0_8_;
                    auVar36._8_4_ = auVar29._0_4_;
                    auVar36._12_4_ = auVar29._4_4_;
                    auVar36 = auVar36 ^ auVar13;
                    auVar37._0_8_ = auVar29._8_8_;
                    auVar37._8_4_ = auVar29._8_4_;
                    auVar37._12_4_ = auVar29._12_4_;
                    auVar37 = auVar37 ^ auVar14;
                    core::hash::sip::impl$5::write<core::hash::sip::Sip13Rounds>();
                    local_108 = auVar37._0_8_;
                    uStack_100 = auVar37._8_8_;
                    local_118 = auVar36._0_8_;
                    uVar21 = local_118 + local_108;
                    lStack_110 = auVar36._8_8_;
                    uVar18 = (local_108 << 0xd | local_108 >> 0x33) ^ uVar21;
                    uVar16 = (uStack_100 << 0x10 | uStack_100 >> 0x30) ^ lStack_110 + uStack_100;
                    uVar19 = lStack_110 + uStack_100 + uVar18;
                    uVar21 = (uVar21 << 0x20 | uVar21 >> 0x20) + uVar16;
                    uVar18 = (uVar18 << 0x11 | uVar18 >> 0x2f) ^ uVar19;
                    uVar16 = (uVar16 << 0x15 | uVar16 >> 0x2b) ^ uVar21;
                    uVar21 = uVar21 + uVar18;
                    uVar19 = ((uVar19 << 0x20 | uVar19 >> 0x20) ^ 0xff) + uVar16;
                    uVar18 = (uVar18 << 0xd | uVar18 >> 0x33) ^ uVar21;
                    uVar16 = (uVar16 << 0x10 | uVar16 >> 0x30) ^ uVar19;
                    uVar19 = uVar19 + uVar18;
                    uVar21 = (uVar21 << 0x20 | uVar21 >> 0x20) + uVar16;
                    uVar18 = (uVar18 << 0x11 | uVar18 >> 0x2f) ^ uVar19;
                    uVar16 = (uVar16 << 0x15 | uVar16 >> 0x2b) ^ uVar21;
                    uVar21 = uVar21 + uVar18;
                    uVar19 = (uVar19 << 0x20 | uVar19 >> 0x20) + uVar16;
                    uVar18 = (uVar18 << 0xd | uVar18 >> 0x33) ^ uVar21;
                    uVar16 = (uVar16 << 0x10 | uVar16 >> 0x30) ^ uVar19;
                    uVar19 = uVar19 + uVar18;
                    uVar21 = (uVar21 << 0x20 | uVar21 >> 0x20) + uVar16;
                    uVar18 = (uVar18 << 0x11 | uVar18 >> 0x2f) ^ uVar19;
                    uVar16 = (uVar16 << 0x15 | uVar16 >> 0x2b) ^ uVar21;
                    uVar19 = (uVar19 << 0x20 | uVar19 >> 0x20) + uVar16;
                    uVar18 = (uVar18 << 0xd | uVar18 >> 0x33) ^ uVar21 + uVar18;
                    uVar16 = (uVar16 << 0x10 | uVar16 >> 0x30) ^ uVar19;
                    uVar19 = uVar19 + uVar18;
                    uVar19 = (uVar19 << 0x20 | uVar19 >> 0x20) ^ (uVar18 << 0x11 | uVar18 >> 0x2f) ^
                             (uVar16 << 0x15 | uVar16 >> 0x2b) ^ uVar19;
                    lVar22 = *plVar4;
                    bVar27 = (byte)(uVar19 >> 0x39);
                    auVar29 = ZEXT216(CONCAT11(bVar27,bVar27));
                    auVar29 = pshuflw(auVar29,auVar29,0);
                    lVar17 = 0;
                    goto LAB_140adf351;
                  }
                  uVar24 = (ushort)(uVar23 - 1) & (ushort)uVar23;
                  uVar23 = CONCAT22((short)(uVar23 - 1 >> 0x10),uVar24);
                }
                auVar35[0] = -(auVar33[0] == (char)iVar8);
                auVar35[1] = -(auVar33[1] == cVar41);
                auVar35[2] = -(auVar33[2] == cVar42);
                auVar35[3] = -(auVar33[3] == cVar43);
                auVar35[4] = -(auVar33[4] == (char)iVar9);
                auVar35[5] = -(auVar33[5] == cVar44);
                auVar35[6] = -(auVar33[6] == cVar45);
                auVar35[7] = -(auVar33[7] == cVar46);
                auVar35[8] = -(auVar33[8] == (char)iVar10);
                auVar35[9] = -(auVar33[9] == cVar47);
                auVar35[10] = -(auVar33[10] == cVar48);
                auVar35[0xb] = -(auVar33[0xb] == cVar49);
                auVar35[0xc] = -(auVar33[0xc] == (char)iVar11);
                auVar35[0xd] = -(auVar33[0xd] == cVar50);
                auVar35[0xe] = -(auVar33[0xe] == cVar51);
                auVar35[0xf] = -(auVar33[0xf] == cVar52);
                if ((((((((((((((((SUB161(auVar35 >> 7,0) & 1) != 0 ||
                                 (SUB161(auVar35 >> 0xf,0) & 1) != 0) ||
                                (SUB161(auVar35 >> 0x17,0) & 1) != 0) ||
                               (SUB161(auVar35 >> 0x1f,0) & 1) != 0) ||
                              (SUB161(auVar35 >> 0x27,0) & 1) != 0) ||
                             (SUB161(auVar35 >> 0x2f,0) & 1) != 0) ||
                            (SUB161(auVar35 >> 0x37,0) & 1) != 0) ||
                           (SUB161(auVar35 >> 0x3f,0) & 1) != 0) ||
                          (SUB161(auVar35 >> 0x47,0) & 1) != 0) ||
                         (SUB161(auVar35 >> 0x4f,0) & 1) != 0) ||
                        (SUB161(auVar35 >> 0x57,0) & 1) != 0) ||
                       (SUB161(auVar35 >> 0x5f,0) & 1) != 0) || (SUB161(auVar35 >> 0x67,0) & 1) != 0
                      ) || (SUB161(auVar35 >> 0x6f,0) & 1) != 0) ||
                    (SUB161(auVar35 >> 0x77,0) & 1) != 0) || auVar35[0xf] < '\0') break;
                uVar19 = uVar19 + lVar22 + 0x10;
                lVar22 = lVar22 + 0x10;
              }
LAB_140adf400:
              do {
                plVar26 = plVar26 + 1;
                if (plVar26 == plVar1) goto LAB_140adefa0;
              } while (plVar3[3] == 0);
            } while( true );
          }
          lVar22 = lVar22 << 3;
          do {
            lVar22 = lVar22 + -8;
          } while (lVar22 != 0);
        }
LAB_140adefa0:
        uVar20 = uVar20 & uVar20 - 1;
      } while (uVar20 != 0);
    }
    lVar15 = lVar15 + 8;
    local_128 = local_128 + 1;
    if (lVar15 == 0x200) {
      return;
    }
  } while( true );
LAB_140adf351:
  uVar19 = uVar19 & plVar4[1];
  auVar33 = *(undefined1 (*) [16])(lVar22 + uVar19);
  cVar28 = auVar29[0];
  auVar40[0] = -(auVar33[0] == cVar28);
  cVar30 = auVar29[1];
  auVar40[1] = -(auVar33[1] == cVar30);
  cVar31 = auVar29[2];
  auVar40[2] = -(auVar33[2] == cVar31);
  cVar32 = auVar29[3];
  auVar40[3] = -(auVar33[3] == cVar32);
  auVar40[4] = -(auVar33[4] == cVar28);
  auVar40[5] = -(auVar33[5] == cVar30);
  auVar40[6] = -(auVar33[6] == cVar31);
  auVar40[7] = -(auVar33[7] == cVar32);
  auVar40[8] = -(auVar33[8] == cVar28);
  auVar40[9] = -(auVar33[9] == cVar30);
  auVar40[10] = -(auVar33[10] == cVar31);
  auVar40[0xb] = -(auVar33[0xb] == cVar32);
  auVar40[0xc] = -(auVar33[0xc] == cVar28);
  auVar40[0xd] = -(auVar33[0xd] == cVar30);
  auVar40[0xe] = -(auVar33[0xe] == cVar31);
  auVar40[0xf] = -(auVar33[0xf] == cVar32);
  uVar24 = (ushort)(SUB161(auVar40 >> 7,0) & 1) | (ushort)(SUB161(auVar40 >> 0xf,0) & 1) << 1 |
           (ushort)(SUB161(auVar40 >> 0x17,0) & 1) << 2 |
           (ushort)(SUB161(auVar40 >> 0x1f,0) & 1) << 3 |
           (ushort)(SUB161(auVar40 >> 0x27,0) & 1) << 4 |
           (ushort)(SUB161(auVar40 >> 0x2f,0) & 1) << 5 |
           (ushort)(SUB161(auVar40 >> 0x37,0) & 1) << 6 |
           (ushort)(SUB161(auVar40 >> 0x3f,0) & 1) << 7 |
           (ushort)(SUB161(auVar40 >> 0x47,0) & 1) << 8 |
           (ushort)(SUB161(auVar40 >> 0x4f,0) & 1) << 9 |
           (ushort)(SUB161(auVar40 >> 0x57,0) & 1) << 10 |
           (ushort)(SUB161(auVar40 >> 0x5f,0) & 1) << 0xb |
           (ushort)(SUB161(auVar40 >> 0x67,0) & 1) << 0xc |
           (ushort)(SUB161(auVar40 >> 0x6f,0) & 1) << 0xd |
           (ushort)(SUB161(auVar40 >> 0x77,0) & 1) << 0xe | (ushort)(auVar40[0xf] >> 7) << 0xf;
  uVar23 = (uint)uVar24;
  while (uVar24 != 0) {
    uVar6 = 0;
    for (uVar7 = uVar23; (uVar7 & 1) == 0; uVar7 = uVar7 >> 1 | 0x80000000) {
      uVar6 = uVar6 + 1;
    }
    lVar25 = (uVar6 + uVar19 & plVar4[1]) * -0x2a8;
    if (*(longlong *)(lVar22 + -0x2a8 + lVar25) == lVar5) {
      if ((*(ulonglong *)(lVar22 + -0x290 + lVar25) < 2) ||
         (*(float *)(lVar22 + -0x270 + lVar25) <= 0.0)) {
                    /* WARNING: Subroutine does not return */
        core::panicking::panic();
      }
      if (*(float *)(lVar22 + lVar25 + -0x24) <= fVar12) {
        hashbrown::map::HashMap::
        insert<country_core::resources::walls::WallId,tuple_<>,std::hash::random::RandomState,alloc::alloc::Global>
                  ();
      }
      goto LAB_140adf400;
    }
    uVar24 = (ushort)(uVar23 - 1) & (ushort)uVar23;
    uVar23 = CONCAT22((short)(uVar23 - 1 >> 0x10),uVar24);
  }
  auVar38[0] = -(auVar33[0] == (char)iVar8);
  auVar38[1] = -(auVar33[1] == cVar41);
  auVar38[2] = -(auVar33[2] == cVar42);
  auVar38[3] = -(auVar33[3] == cVar43);
  auVar38[4] = -(auVar33[4] == (char)iVar9);
  auVar38[5] = -(auVar33[5] == cVar44);
  auVar38[6] = -(auVar33[6] == cVar45);
  auVar38[7] = -(auVar33[7] == cVar46);
  auVar38[8] = -(auVar33[8] == (char)iVar10);
  auVar38[9] = -(auVar33[9] == cVar47);
  auVar38[10] = -(auVar33[10] == cVar48);
  auVar38[0xb] = -(auVar33[0xb] == cVar49);
  auVar38[0xc] = -(auVar33[0xc] == (char)iVar11);
  auVar38[0xd] = -(auVar33[0xd] == cVar50);
  auVar38[0xe] = -(auVar33[0xe] == cVar51);
  auVar38[0xf] = -(auVar33[0xf] == cVar52);
  if ((((((((((((((((SUB161(auVar38 >> 7,0) & 1) != 0 || (SUB161(auVar38 >> 0xf,0) & 1) != 0) ||
                  (SUB161(auVar38 >> 0x17,0) & 1) != 0) || (SUB161(auVar38 >> 0x1f,0) & 1) != 0) ||
                (SUB161(auVar38 >> 0x27,0) & 1) != 0) || (SUB161(auVar38 >> 0x2f,0) & 1) != 0) ||
              (SUB161(auVar38 >> 0x37,0) & 1) != 0) || (SUB161(auVar38 >> 0x3f,0) & 1) != 0) ||
            (SUB161(auVar38 >> 0x47,0) & 1) != 0) || (SUB161(auVar38 >> 0x4f,0) & 1) != 0) ||
          (SUB161(auVar38 >> 0x57,0) & 1) != 0) || (SUB161(auVar38 >> 0x5f,0) & 1) != 0) ||
        (SUB161(auVar38 >> 0x67,0) & 1) != 0) || (SUB161(auVar38 >> 0x6f,0) & 1) != 0) ||
      (SUB161(auVar38 >> 0x77,0) & 1) != 0) || auVar38[0xf] < '\0') goto LAB_140adf400;
  uVar19 = uVar19 + lVar17 + 0x10;
  lVar17 = lVar17 + 0x10;
  goto LAB_140adf351;
}

