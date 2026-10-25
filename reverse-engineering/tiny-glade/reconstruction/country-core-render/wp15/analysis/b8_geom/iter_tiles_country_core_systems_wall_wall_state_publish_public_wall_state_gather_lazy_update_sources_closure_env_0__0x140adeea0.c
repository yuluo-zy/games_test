// addr=0x140adeea0 name=iter_tiles<country_core::systems::wall::wall_state::publish_public_wall_state::gather_lazy_update_sources::closure_env$0>
// signature=void __cdecl iter_tiles<country_core::systems::wall::wall_state::publish_public_wall_state::gather_lazy_update_sources::closure_env$0>(void)


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

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
  undefined1 auVar12 [16];
  undefined1 auVar13 [16];
  longlong in_RCX;
  longlong lVar14;
  ulonglong uVar15;
  longlong lVar16;
  ulonglong uVar17;
  longlong *in_RDX;
  ulonglong uVar18;
  ulonglong uVar19;
  ulonglong uVar20;
  longlong lVar21;
  uint uVar22;
  ushort uVar23;
  longlong lVar24;
  longlong *plVar25;
  byte bVar26;
  char cVar27;
  char cVar29;
  char cVar30;
  char cVar31;
  undefined1 auVar28 [16];
  undefined1 auVar32 [16];
  undefined1 auVar33 [16];
  undefined1 auVar34 [16];
  undefined1 auVar35 [16];
  undefined1 auVar36 [16];
  undefined1 auVar37 [16];
  undefined1 auVar38 [16];
  undefined1 auVar39 [16];
  undefined1 unaff_XMM9_Ba;
  undefined1 unaff_XMM9_Bb;
  char cVar40;
  undefined1 unaff_XMM9_Bc;
  char cVar41;
  undefined1 unaff_XMM9_Bd;
  char cVar42;
  undefined1 unaff_XMM9_Be;
  undefined1 unaff_XMM9_Bf;
  char cVar43;
  undefined1 unaff_XMM9_Bg;
  char cVar44;
  undefined1 unaff_XMM9_Bh;
  char cVar45;
  undefined1 unaff_XMM9_Bi;
  undefined1 unaff_XMM9_Bj;
  char cVar46;
  undefined1 unaff_XMM9_Bk;
  char cVar47;
  undefined1 unaff_XMM9_Bl;
  char cVar48;
  undefined1 unaff_XMM9_Bm;
  undefined1 unaff_XMM9_Bn;
  char cVar49;
  undefined1 unaff_XMM9_Bo;
  char cVar50;
  undefined1 unaff_XMM9_Bp;
  char cVar51;
  longlong local_128;
  longlong local_118;
  longlong lStack_110;
  ulonglong local_108;
  ulonglong uStack_100;
  
  auVar13 = ___xmm_7465646279746573646f72616e646f6d;
  auVar12 = ___xmm_6c7967656e657261736f6d6570736575;
  lVar2 = *in_RDX;
  plVar3 = (longlong *)in_RDX[1];
  plVar4 = (longlong *)in_RDX[2];
  lVar14 = 0;
  iVar8 = -(uint)(CONCAT13(unaff_XMM9_Bd,
                           CONCAT12(unaff_XMM9_Bc,CONCAT11(unaff_XMM9_Bb,unaff_XMM9_Ba))) ==
                 CONCAT13(unaff_XMM9_Bd,
                          CONCAT12(unaff_XMM9_Bc,CONCAT11(unaff_XMM9_Bb,unaff_XMM9_Ba))));
  cVar40 = (char)((uint)iVar8 >> 8);
  cVar41 = (char)((uint)iVar8 >> 0x10);
  cVar42 = (char)((uint)iVar8 >> 0x18);
  iVar9 = -(uint)(CONCAT13(unaff_XMM9_Bh,
                           CONCAT12(unaff_XMM9_Bg,CONCAT11(unaff_XMM9_Bf,unaff_XMM9_Be))) ==
                 CONCAT13(unaff_XMM9_Bh,
                          CONCAT12(unaff_XMM9_Bg,CONCAT11(unaff_XMM9_Bf,unaff_XMM9_Be))));
  cVar43 = (char)((uint)iVar9 >> 8);
  cVar44 = (char)((uint)iVar9 >> 0x10);
  cVar45 = (char)((uint)iVar9 >> 0x18);
  iVar10 = -(uint)(CONCAT13(unaff_XMM9_Bl,
                            CONCAT12(unaff_XMM9_Bk,CONCAT11(unaff_XMM9_Bj,unaff_XMM9_Bi))) ==
                  CONCAT13(unaff_XMM9_Bl,
                           CONCAT12(unaff_XMM9_Bk,CONCAT11(unaff_XMM9_Bj,unaff_XMM9_Bi))));
  cVar46 = (char)((uint)iVar10 >> 8);
  cVar47 = (char)((uint)iVar10 >> 0x10);
  cVar48 = (char)((uint)iVar10 >> 0x18);
  iVar11 = -(uint)(CONCAT13(unaff_XMM9_Bp,
                            CONCAT12(unaff_XMM9_Bo,CONCAT11(unaff_XMM9_Bn,unaff_XMM9_Bm))) ==
                  CONCAT13(unaff_XMM9_Bp,
                           CONCAT12(unaff_XMM9_Bo,CONCAT11(unaff_XMM9_Bn,unaff_XMM9_Bm))));
  cVar49 = (char)((uint)iVar11 >> 8);
  cVar50 = (char)((uint)iVar11 >> 0x10);
  cVar51 = (char)((uint)iVar11 >> 0x18);
  local_128 = 0;
  do {
    uVar19 = *(ulonglong *)(in_RCX + lVar14);
    if (uVar19 != 0) {
      do {
        uVar15 = 0;
        for (uVar17 = uVar19; (uVar17 & 1) == 0; uVar17 = uVar17 >> 1 | 0x8000000000000000) {
          uVar15 = uVar15 + 1;
        }
        uVar15 = uVar15 | local_128 << 6;
        if (*(ulonglong *)(lVar2 + 0x10) <= uVar15) {
                    // WARNING: Subroutine does not return
          core::panicking::panic_bounds_check();
        }
        lVar5 = *(longlong *)(lVar2 + 8);
        lVar16 = uVar15 * 0x30;
        lVar21 = *(longlong *)(lVar5 + 0x28 + lVar16);
        if (lVar21 != 0) {
          if (plVar3[3] != 0) {
            plVar25 = *(longlong **)(lVar5 + 0x20 + lVar16);
            plVar1 = plVar25 + lVar21;
            do {
              lVar5 = *plVar25;
              auVar28 = *(undefined1 (*) [16])(plVar3 + 4);
              auVar32._0_8_ = auVar28._0_8_;
              auVar32._8_4_ = auVar28._0_4_;
              auVar32._12_4_ = auVar28._4_4_;
              auVar32 = auVar32 ^ auVar12;
              auVar33._0_8_ = auVar28._8_8_;
              auVar33._8_4_ = auVar28._8_4_;
              auVar33._12_4_ = auVar28._12_4_;
              auVar33 = auVar33 ^ auVar13;
              core::hash::sip::impl$5::write<core::hash::sip::Sip13Rounds>();
              local_108 = auVar33._0_8_;
              uStack_100 = auVar33._8_8_;
              local_118 = auVar32._0_8_;
              uVar20 = local_118 + local_108;
              lStack_110 = auVar32._8_8_;
              uVar17 = (local_108 << 0xd | local_108 >> 0x33) ^ uVar20;
              uVar15 = (uStack_100 << 0x10 | uStack_100 >> 0x30) ^ lStack_110 + uStack_100;
              uVar18 = lStack_110 + uStack_100 + uVar17;
              uVar20 = (uVar20 << 0x20 | uVar20 >> 0x20) + uVar15;
              uVar17 = (uVar17 << 0x11 | uVar17 >> 0x2f) ^ uVar18;
              uVar15 = (uVar15 << 0x15 | uVar15 >> 0x2b) ^ uVar20;
              uVar20 = uVar20 + uVar17;
              uVar18 = ((uVar18 << 0x20 | uVar18 >> 0x20) ^ 0xff) + uVar15;
              uVar17 = (uVar17 << 0xd | uVar17 >> 0x33) ^ uVar20;
              uVar15 = (uVar15 << 0x10 | uVar15 >> 0x30) ^ uVar18;
              uVar18 = uVar18 + uVar17;
              uVar20 = (uVar20 << 0x20 | uVar20 >> 0x20) + uVar15;
              uVar17 = (uVar17 << 0x11 | uVar17 >> 0x2f) ^ uVar18;
              uVar15 = (uVar15 << 0x15 | uVar15 >> 0x2b) ^ uVar20;
              uVar20 = uVar20 + uVar17;
              uVar18 = (uVar18 << 0x20 | uVar18 >> 0x20) + uVar15;
              uVar17 = (uVar17 << 0xd | uVar17 >> 0x33) ^ uVar20;
              uVar15 = (uVar15 << 0x10 | uVar15 >> 0x30) ^ uVar18;
              uVar18 = uVar18 + uVar17;
              uVar20 = (uVar20 << 0x20 | uVar20 >> 0x20) + uVar15;
              uVar17 = (uVar17 << 0x11 | uVar17 >> 0x2f) ^ uVar18;
              uVar15 = (uVar15 << 0x15 | uVar15 >> 0x2b) ^ uVar20;
              uVar18 = (uVar18 << 0x20 | uVar18 >> 0x20) + uVar15;
              uVar17 = (uVar17 << 0xd | uVar17 >> 0x33) ^ uVar20 + uVar17;
              uVar15 = (uVar15 << 0x10 | uVar15 >> 0x30) ^ uVar18;
              uVar18 = uVar18 + uVar17;
              uVar18 = (uVar18 << 0x20 | uVar18 >> 0x20) ^ (uVar17 << 0x11 | uVar17 >> 0x2f) ^
                       (uVar15 << 0x15 | uVar15 >> 0x2b) ^ uVar18;
              bVar26 = (byte)(uVar18 >> 0x39);
              auVar28 = ZEXT216(CONCAT11(bVar26,bVar26));
              auVar28 = pshuflw(auVar28,auVar28,0);
              lVar21 = 0;
              while( true ) {
                uVar18 = uVar18 & plVar3[1];
                auVar32 = *(undefined1 (*) [16])(*plVar3 + uVar18);
                cVar27 = auVar28[0];
                auVar38[0] = -(auVar32[0] == cVar27);
                cVar29 = auVar28[1];
                auVar38[1] = -(auVar32[1] == cVar29);
                cVar30 = auVar28[2];
                auVar38[2] = -(auVar32[2] == cVar30);
                cVar31 = auVar28[3];
                auVar38[3] = -(auVar32[3] == cVar31);
                auVar38[4] = -(auVar32[4] == cVar27);
                auVar38[5] = -(auVar32[5] == cVar29);
                auVar38[6] = -(auVar32[6] == cVar30);
                auVar38[7] = -(auVar32[7] == cVar31);
                auVar38[8] = -(auVar32[8] == cVar27);
                auVar38[9] = -(auVar32[9] == cVar29);
                auVar38[10] = -(auVar32[10] == cVar30);
                auVar38[0xb] = -(auVar32[0xb] == cVar31);
                auVar38[0xc] = -(auVar32[0xc] == cVar27);
                auVar38[0xd] = -(auVar32[0xd] == cVar29);
                auVar38[0xe] = -(auVar32[0xe] == cVar30);
                auVar38[0xf] = -(auVar32[0xf] == cVar31);
                uVar23 = (ushort)(SUB161(auVar38 >> 7,0) & 1) |
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
                uVar22 = (uint)uVar23;
                while (uVar23 != 0) {
                  uVar6 = 0;
                  for (uVar7 = uVar22; (uVar7 & 1) == 0; uVar7 = uVar7 >> 1 | 0x80000000) {
                    uVar6 = uVar6 + 1;
                  }
                  if (*(longlong *)(*plVar3 + -0x78 + (uVar6 + uVar18 & plVar3[1]) * -0x78) == lVar5
                     ) {
                    if (plVar4[3] == 0) goto LAB_140adf400;
                    auVar28 = *(undefined1 (*) [16])(plVar4 + 4);
                    auVar35._0_8_ = auVar28._0_8_;
                    auVar35._8_4_ = auVar28._0_4_;
                    auVar35._12_4_ = auVar28._4_4_;
                    auVar35 = auVar35 ^ auVar12;
                    auVar36._0_8_ = auVar28._8_8_;
                    auVar36._8_4_ = auVar28._8_4_;
                    auVar36._12_4_ = auVar28._12_4_;
                    auVar36 = auVar36 ^ auVar13;
                    core::hash::sip::impl$5::write<core::hash::sip::Sip13Rounds>();
                    local_108 = auVar36._0_8_;
                    uStack_100 = auVar36._8_8_;
                    local_118 = auVar35._0_8_;
                    uVar20 = local_118 + local_108;
                    lStack_110 = auVar35._8_8_;
                    uVar17 = (local_108 << 0xd | local_108 >> 0x33) ^ uVar20;
                    uVar15 = (uStack_100 << 0x10 | uStack_100 >> 0x30) ^ lStack_110 + uStack_100;
                    uVar18 = lStack_110 + uStack_100 + uVar17;
                    uVar20 = (uVar20 << 0x20 | uVar20 >> 0x20) + uVar15;
                    uVar17 = (uVar17 << 0x11 | uVar17 >> 0x2f) ^ uVar18;
                    uVar15 = (uVar15 << 0x15 | uVar15 >> 0x2b) ^ uVar20;
                    uVar20 = uVar20 + uVar17;
                    uVar18 = ((uVar18 << 0x20 | uVar18 >> 0x20) ^ 0xff) + uVar15;
                    uVar17 = (uVar17 << 0xd | uVar17 >> 0x33) ^ uVar20;
                    uVar15 = (uVar15 << 0x10 | uVar15 >> 0x30) ^ uVar18;
                    uVar18 = uVar18 + uVar17;
                    uVar20 = (uVar20 << 0x20 | uVar20 >> 0x20) + uVar15;
                    uVar17 = (uVar17 << 0x11 | uVar17 >> 0x2f) ^ uVar18;
                    uVar15 = (uVar15 << 0x15 | uVar15 >> 0x2b) ^ uVar20;
                    uVar20 = uVar20 + uVar17;
                    uVar18 = (uVar18 << 0x20 | uVar18 >> 0x20) + uVar15;
                    uVar17 = (uVar17 << 0xd | uVar17 >> 0x33) ^ uVar20;
                    uVar15 = (uVar15 << 0x10 | uVar15 >> 0x30) ^ uVar18;
                    uVar18 = uVar18 + uVar17;
                    uVar20 = (uVar20 << 0x20 | uVar20 >> 0x20) + uVar15;
                    uVar17 = (uVar17 << 0x11 | uVar17 >> 0x2f) ^ uVar18;
                    uVar15 = (uVar15 << 0x15 | uVar15 >> 0x2b) ^ uVar20;
                    uVar18 = (uVar18 << 0x20 | uVar18 >> 0x20) + uVar15;
                    uVar17 = (uVar17 << 0xd | uVar17 >> 0x33) ^ uVar20 + uVar17;
                    uVar15 = (uVar15 << 0x10 | uVar15 >> 0x30) ^ uVar18;
                    uVar18 = uVar18 + uVar17;
                    uVar18 = (uVar18 << 0x20 | uVar18 >> 0x20) ^ (uVar17 << 0x11 | uVar17 >> 0x2f) ^
                             (uVar15 << 0x15 | uVar15 >> 0x2b) ^ uVar18;
                    lVar21 = *plVar4;
                    bVar26 = (byte)(uVar18 >> 0x39);
                    auVar28 = ZEXT216(CONCAT11(bVar26,bVar26));
                    auVar28 = pshuflw(auVar28,auVar28,0);
                    lVar16 = 0;
                    goto LAB_140adf351;
                  }
                  uVar23 = (ushort)(uVar22 - 1) & (ushort)uVar22;
                  uVar22 = CONCAT22((short)(uVar22 - 1 >> 0x10),uVar23);
                }
                auVar34[0] = -(auVar32[0] == (char)iVar8);
                auVar34[1] = -(auVar32[1] == cVar40);
                auVar34[2] = -(auVar32[2] == cVar41);
                auVar34[3] = -(auVar32[3] == cVar42);
                auVar34[4] = -(auVar32[4] == (char)iVar9);
                auVar34[5] = -(auVar32[5] == cVar43);
                auVar34[6] = -(auVar32[6] == cVar44);
                auVar34[7] = -(auVar32[7] == cVar45);
                auVar34[8] = -(auVar32[8] == (char)iVar10);
                auVar34[9] = -(auVar32[9] == cVar46);
                auVar34[10] = -(auVar32[10] == cVar47);
                auVar34[0xb] = -(auVar32[0xb] == cVar48);
                auVar34[0xc] = -(auVar32[0xc] == (char)iVar11);
                auVar34[0xd] = -(auVar32[0xd] == cVar49);
                auVar34[0xe] = -(auVar32[0xe] == cVar50);
                auVar34[0xf] = -(auVar32[0xf] == cVar51);
                if ((((((((((((((((SUB161(auVar34 >> 7,0) & 1) != 0 ||
                                 (SUB161(auVar34 >> 0xf,0) & 1) != 0) ||
                                (SUB161(auVar34 >> 0x17,0) & 1) != 0) ||
                               (SUB161(auVar34 >> 0x1f,0) & 1) != 0) ||
                              (SUB161(auVar34 >> 0x27,0) & 1) != 0) ||
                             (SUB161(auVar34 >> 0x2f,0) & 1) != 0) ||
                            (SUB161(auVar34 >> 0x37,0) & 1) != 0) ||
                           (SUB161(auVar34 >> 0x3f,0) & 1) != 0) ||
                          (SUB161(auVar34 >> 0x47,0) & 1) != 0) ||
                         (SUB161(auVar34 >> 0x4f,0) & 1) != 0) ||
                        (SUB161(auVar34 >> 0x57,0) & 1) != 0) ||
                       (SUB161(auVar34 >> 0x5f,0) & 1) != 0) || (SUB161(auVar34 >> 0x67,0) & 1) != 0
                      ) || (SUB161(auVar34 >> 0x6f,0) & 1) != 0) ||
                    (SUB161(auVar34 >> 0x77,0) & 1) != 0) || auVar34[0xf] < '\0') break;
                uVar18 = uVar18 + lVar21 + 0x10;
                lVar21 = lVar21 + 0x10;
              }
LAB_140adf400:
              do {
                plVar25 = plVar25 + 1;
                if (plVar25 == plVar1) goto LAB_140adefa0;
              } while (plVar3[3] == 0);
            } while( true );
          }
          lVar21 = lVar21 << 3;
          do {
            lVar21 = lVar21 + -8;
          } while (lVar21 != 0);
        }
LAB_140adefa0:
        uVar19 = uVar19 & uVar19 - 1;
      } while (uVar19 != 0);
    }
    lVar14 = lVar14 + 8;
    local_128 = local_128 + 1;
    if (lVar14 == 0x200) {
      return;
    }
  } while( true );
LAB_140adf351:
  uVar18 = uVar18 & plVar4[1];
  auVar32 = *(undefined1 (*) [16])(lVar21 + uVar18);
  cVar27 = auVar28[0];
  auVar39[0] = -(auVar32[0] == cVar27);
  cVar29 = auVar28[1];
  auVar39[1] = -(auVar32[1] == cVar29);
  cVar30 = auVar28[2];
  auVar39[2] = -(auVar32[2] == cVar30);
  cVar31 = auVar28[3];
  auVar39[3] = -(auVar32[3] == cVar31);
  auVar39[4] = -(auVar32[4] == cVar27);
  auVar39[5] = -(auVar32[5] == cVar29);
  auVar39[6] = -(auVar32[6] == cVar30);
  auVar39[7] = -(auVar32[7] == cVar31);
  auVar39[8] = -(auVar32[8] == cVar27);
  auVar39[9] = -(auVar32[9] == cVar29);
  auVar39[10] = -(auVar32[10] == cVar30);
  auVar39[0xb] = -(auVar32[0xb] == cVar31);
  auVar39[0xc] = -(auVar32[0xc] == cVar27);
  auVar39[0xd] = -(auVar32[0xd] == cVar29);
  auVar39[0xe] = -(auVar32[0xe] == cVar30);
  auVar39[0xf] = -(auVar32[0xf] == cVar31);
  uVar23 = (ushort)(SUB161(auVar39 >> 7,0) & 1) | (ushort)(SUB161(auVar39 >> 0xf,0) & 1) << 1 |
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
           (ushort)(SUB161(auVar39 >> 0x77,0) & 1) << 0xe | (ushort)(auVar39[0xf] >> 7) << 0xf;
  uVar22 = (uint)uVar23;
  while (uVar23 != 0) {
    uVar6 = 0;
    for (uVar7 = uVar22; (uVar7 & 1) == 0; uVar7 = uVar7 >> 1 | 0x80000000) {
      uVar6 = uVar6 + 1;
    }
    lVar24 = (uVar6 + uVar18 & plVar4[1]) * -0x2a8;
    if (*(longlong *)(lVar21 + -0x2a8 + lVar24) == lVar5) {
      if ((*(ulonglong *)(lVar21 + -0x290 + lVar24) < 2) ||
         (*(float *)(lVar21 + -0x270 + lVar24) <= 0.0)) {
                    // WARNING: Subroutine does not return
        core::panicking::panic();
      }
      if (*(float *)(lVar21 + lVar24 + -0x24) <= 0.5) {
        hashbrown::map::HashMap::
        insert<country_core::resources::walls::WallId,tuple_<>,std::hash::random::RandomState,alloc::alloc::Global>
                  ();
      }
      goto LAB_140adf400;
    }
    uVar23 = (ushort)(uVar22 - 1) & (ushort)uVar22;
    uVar22 = CONCAT22((short)(uVar22 - 1 >> 0x10),uVar23);
  }
  auVar37[0] = -(auVar32[0] == (char)iVar8);
  auVar37[1] = -(auVar32[1] == cVar40);
  auVar37[2] = -(auVar32[2] == cVar41);
  auVar37[3] = -(auVar32[3] == cVar42);
  auVar37[4] = -(auVar32[4] == (char)iVar9);
  auVar37[5] = -(auVar32[5] == cVar43);
  auVar37[6] = -(auVar32[6] == cVar44);
  auVar37[7] = -(auVar32[7] == cVar45);
  auVar37[8] = -(auVar32[8] == (char)iVar10);
  auVar37[9] = -(auVar32[9] == cVar46);
  auVar37[10] = -(auVar32[10] == cVar47);
  auVar37[0xb] = -(auVar32[0xb] == cVar48);
  auVar37[0xc] = -(auVar32[0xc] == (char)iVar11);
  auVar37[0xd] = -(auVar32[0xd] == cVar49);
  auVar37[0xe] = -(auVar32[0xe] == cVar50);
  auVar37[0xf] = -(auVar32[0xf] == cVar51);
  if ((((((((((((((((SUB161(auVar37 >> 7,0) & 1) != 0 || (SUB161(auVar37 >> 0xf,0) & 1) != 0) ||
                  (SUB161(auVar37 >> 0x17,0) & 1) != 0) || (SUB161(auVar37 >> 0x1f,0) & 1) != 0) ||
                (SUB161(auVar37 >> 0x27,0) & 1) != 0) || (SUB161(auVar37 >> 0x2f,0) & 1) != 0) ||
              (SUB161(auVar37 >> 0x37,0) & 1) != 0) || (SUB161(auVar37 >> 0x3f,0) & 1) != 0) ||
            (SUB161(auVar37 >> 0x47,0) & 1) != 0) || (SUB161(auVar37 >> 0x4f,0) & 1) != 0) ||
          (SUB161(auVar37 >> 0x57,0) & 1) != 0) || (SUB161(auVar37 >> 0x5f,0) & 1) != 0) ||
        (SUB161(auVar37 >> 0x67,0) & 1) != 0) || (SUB161(auVar37 >> 0x6f,0) & 1) != 0) ||
      (SUB161(auVar37 >> 0x77,0) & 1) != 0) || auVar37[0xf] < '\0') goto LAB_140adf400;
  uVar18 = uVar18 + lVar16 + 0x10;
  lVar16 = lVar16 + 0x10;
  goto LAB_140adf351;
}


