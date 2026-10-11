
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void __cdecl
country_core::utils::dirty_tiles::DirtyTiles::
iter_tiles<country_core::systems::wall::wall_state::publish_public_wall_state::gather_lazy_update_sources::closure_env_1>
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
  float fVar9;
  undefined1 auVar10 [16];
  undefined1 auVar11 [16];
  longlong in_RCX;
  longlong lVar12;
  ulonglong uVar13;
  longlong lVar14;
  ulonglong uVar15;
  longlong *in_RDX;
  ulonglong uVar16;
  longlong *plVar17;
  ulonglong uVar18;
  uint uVar19;
  longlong lVar20;
  ushort uVar21;
  longlong lVar22;
  ulonglong uVar23;
  byte bVar24;
  char cVar25;
  char cVar27;
  char cVar28;
  char cVar29;
  undefined1 auVar26 [16];
  undefined1 auVar30 [16];
  undefined1 auVar31 [16];
  undefined1 auVar32 [16];
  undefined1 auVar33 [16];
  undefined1 auVar34 [16];
  undefined1 auVar35 [16];
  undefined1 auVar36 [16];
  undefined1 auVar37 [16];
  undefined1 auVar38 [16];
  undefined1 auVar39 [16];
  undefined1 auVar40 [16];
  undefined1 auVar41 [16];
  undefined1 unaff_XMM9_Ba;
  char cVar42;
  undefined1 unaff_XMM9_Bb;
  char cVar43;
  undefined1 unaff_XMM9_Bc;
  char cVar44;
  undefined1 unaff_XMM9_Bd;
  char cVar45;
  undefined1 unaff_XMM9_Be;
  char cVar46;
  undefined1 unaff_XMM9_Bf;
  char cVar47;
  undefined1 unaff_XMM9_Bg;
  char cVar48;
  undefined1 unaff_XMM9_Bh;
  char cVar49;
  undefined1 unaff_XMM9_Bi;
  char cVar50;
  undefined1 unaff_XMM9_Bj;
  char cVar51;
  undefined1 unaff_XMM9_Bk;
  char cVar52;
  undefined1 unaff_XMM9_Bl;
  char cVar53;
  undefined1 unaff_XMM9_Bm;
  char cVar54;
  undefined1 unaff_XMM9_Bn;
  char cVar55;
  undefined1 unaff_XMM9_Bo;
  char cVar56;
  undefined1 unaff_XMM9_Bp;
  char cVar57;
  longlong local_128;
  longlong lStack_120;
  ulonglong local_118;
  ulonglong uStack_110;
  longlong local_d8;
  
  auVar11 = ___xmm_7465646279746573646f72616e646f6d;
  auVar10 = ___xmm_6c7967656e657261736f6d6570736575;
  fVar9 = __real_3f000000;
  lVar2 = *in_RDX;
  plVar3 = (longlong *)in_RDX[1];
  plVar4 = (longlong *)in_RDX[2];
  lVar12 = 0;
  iVar8 = -(uint)(CONCAT13(unaff_XMM9_Bd,
                           CONCAT12(unaff_XMM9_Bc,CONCAT11(unaff_XMM9_Bb,unaff_XMM9_Ba))) ==
                 CONCAT13(unaff_XMM9_Bd,
                          CONCAT12(unaff_XMM9_Bc,CONCAT11(unaff_XMM9_Bb,unaff_XMM9_Ba))));
  cVar42 = (char)iVar8;
  cVar43 = (char)((uint)iVar8 >> 8);
  cVar44 = (char)((uint)iVar8 >> 0x10);
  cVar45 = (char)((uint)iVar8 >> 0x18);
  iVar8 = -(uint)(CONCAT13(unaff_XMM9_Bh,
                           CONCAT12(unaff_XMM9_Bg,CONCAT11(unaff_XMM9_Bf,unaff_XMM9_Be))) ==
                 CONCAT13(unaff_XMM9_Bh,
                          CONCAT12(unaff_XMM9_Bg,CONCAT11(unaff_XMM9_Bf,unaff_XMM9_Be))));
  cVar46 = (char)iVar8;
  cVar47 = (char)((uint)iVar8 >> 8);
  cVar48 = (char)((uint)iVar8 >> 0x10);
  cVar49 = (char)((uint)iVar8 >> 0x18);
  iVar8 = -(uint)(CONCAT13(unaff_XMM9_Bl,
                           CONCAT12(unaff_XMM9_Bk,CONCAT11(unaff_XMM9_Bj,unaff_XMM9_Bi))) ==
                 CONCAT13(unaff_XMM9_Bl,
                          CONCAT12(unaff_XMM9_Bk,CONCAT11(unaff_XMM9_Bj,unaff_XMM9_Bi))));
  cVar50 = (char)iVar8;
  cVar51 = (char)((uint)iVar8 >> 8);
  cVar52 = (char)((uint)iVar8 >> 0x10);
  cVar53 = (char)((uint)iVar8 >> 0x18);
  iVar8 = -(uint)(CONCAT13(unaff_XMM9_Bp,
                           CONCAT12(unaff_XMM9_Bo,CONCAT11(unaff_XMM9_Bn,unaff_XMM9_Bm))) ==
                 CONCAT13(unaff_XMM9_Bp,
                          CONCAT12(unaff_XMM9_Bo,CONCAT11(unaff_XMM9_Bn,unaff_XMM9_Bm))));
  cVar54 = (char)iVar8;
  cVar55 = (char)((uint)iVar8 >> 8);
  cVar56 = (char)((uint)iVar8 >> 0x10);
  cVar57 = (char)((uint)iVar8 >> 0x18);
  local_d8 = 0;
  do {
    uVar23 = *(ulonglong *)(in_RCX + lVar12);
    if (uVar23 != 0) {
      do {
        uVar13 = 0;
        for (uVar15 = uVar23; (uVar15 & 1) == 0; uVar15 = uVar15 >> 1 | 0x8000000000000000) {
          uVar13 = uVar13 + 1;
        }
        uVar13 = uVar13 | local_d8 << 6;
        if (*(ulonglong *)(lVar2 + 0x10) <= uVar13) {
                    /* WARNING: Subroutine does not return */
          core::panicking::panic_bounds_check();
        }
        lVar5 = *(longlong *)(lVar2 + 8);
        lVar14 = uVar13 * 0x30;
        lVar20 = *(longlong *)(lVar5 + 0x28 + lVar14);
        if (lVar20 != 0) {
          if (plVar3[3] != 0) {
            plVar17 = *(longlong **)(lVar5 + 0x20 + lVar14);
            plVar1 = plVar17 + lVar20;
LAB_140adf620:
            lVar5 = *plVar17;
            auVar26 = *(undefined1 (*) [16])(plVar3 + 4);
            auVar30._0_8_ = auVar26._0_8_;
            auVar30._8_4_ = auVar26._0_4_;
            auVar30._12_4_ = auVar26._4_4_;
            auVar31._0_8_ = auVar26._8_8_;
            auVar31._8_4_ = auVar26._8_4_;
            auVar31._12_4_ = auVar26._12_4_;
            core::hash::sip::impl$5::write<core::hash::sip::Sip13Rounds>();
            local_118 = SUB168(auVar31 ^ auVar11,0);
            uStack_110 = SUB168(auVar31 ^ auVar11,8);
            local_128 = SUB168(auVar30 ^ auVar10,0);
            uVar18 = local_128 + local_118;
            lStack_120 = SUB168(auVar30 ^ auVar10,8);
            uVar15 = (local_118 << 0xd | local_118 >> 0x33) ^ uVar18;
            uVar13 = (uStack_110 << 0x10 | uStack_110 >> 0x30) ^ lStack_120 + uStack_110;
            uVar16 = lStack_120 + uStack_110 + uVar15;
            uVar18 = (uVar18 << 0x20 | uVar18 >> 0x20) + uVar13;
            uVar15 = (uVar15 << 0x11 | uVar15 >> 0x2f) ^ uVar16;
            uVar13 = (uVar13 << 0x15 | uVar13 >> 0x2b) ^ uVar18;
            uVar18 = uVar18 + uVar15;
            uVar16 = ((uVar16 << 0x20 | uVar16 >> 0x20) ^ 0xff) + uVar13;
            uVar15 = (uVar15 << 0xd | uVar15 >> 0x33) ^ uVar18;
            uVar13 = (uVar13 << 0x10 | uVar13 >> 0x30) ^ uVar16;
            uVar16 = uVar16 + uVar15;
            uVar18 = (uVar18 << 0x20 | uVar18 >> 0x20) + uVar13;
            uVar15 = (uVar15 << 0x11 | uVar15 >> 0x2f) ^ uVar16;
            uVar13 = (uVar13 << 0x15 | uVar13 >> 0x2b) ^ uVar18;
            uVar18 = uVar18 + uVar15;
            uVar16 = (uVar16 << 0x20 | uVar16 >> 0x20) + uVar13;
            uVar15 = (uVar15 << 0xd | uVar15 >> 0x33) ^ uVar18;
            uVar13 = (uVar13 << 0x10 | uVar13 >> 0x30) ^ uVar16;
            uVar16 = uVar16 + uVar15;
            uVar18 = (uVar18 << 0x20 | uVar18 >> 0x20) + uVar13;
            uVar15 = (uVar15 << 0x11 | uVar15 >> 0x2f) ^ uVar16;
            uVar13 = (uVar13 << 0x15 | uVar13 >> 0x2b) ^ uVar18;
            uVar16 = (uVar16 << 0x20 | uVar16 >> 0x20) + uVar13;
            uVar15 = (uVar15 << 0xd | uVar15 >> 0x33) ^ uVar18 + uVar15;
            uVar13 = (uVar13 << 0x10 | uVar13 >> 0x30) ^ uVar16;
            uVar16 = uVar16 + uVar15;
            uVar16 = (uVar16 << 0x20 | uVar16 >> 0x20) ^ (uVar15 << 0x11 | uVar15 >> 0x2f) ^
                     (uVar13 << 0x15 | uVar13 >> 0x2b) ^ uVar16;
            bVar24 = (byte)(uVar16 >> 0x39);
            auVar26 = ZEXT216(CONCAT11(bVar24,bVar24));
            auVar26 = pshuflw(auVar26,auVar26,0);
            lVar20 = 0;
            while( true ) {
              uVar16 = uVar16 & plVar3[1];
              auVar30 = *(undefined1 (*) [16])(*plVar3 + uVar16);
              cVar25 = auVar26[0];
              auVar39[0] = -(auVar30[0] == cVar25);
              cVar27 = auVar26[1];
              auVar39[1] = -(auVar30[1] == cVar27);
              cVar28 = auVar26[2];
              auVar39[2] = -(auVar30[2] == cVar28);
              cVar29 = auVar26[3];
              auVar39[3] = -(auVar30[3] == cVar29);
              auVar39[4] = -(auVar30[4] == cVar25);
              auVar39[5] = -(auVar30[5] == cVar27);
              auVar39[6] = -(auVar30[6] == cVar28);
              auVar39[7] = -(auVar30[7] == cVar29);
              auVar39[8] = -(auVar30[8] == cVar25);
              auVar39[9] = -(auVar30[9] == cVar27);
              auVar39[10] = -(auVar30[10] == cVar28);
              auVar39[0xb] = -(auVar30[0xb] == cVar29);
              auVar39[0xc] = -(auVar30[0xc] == cVar25);
              auVar39[0xd] = -(auVar30[0xd] == cVar27);
              auVar39[0xe] = -(auVar30[0xe] == cVar28);
              auVar39[0xf] = -(auVar30[0xf] == cVar29);
              uVar21 = (ushort)(SUB161(auVar39 >> 7,0) & 1) |
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
              uVar19 = (uint)uVar21;
              while (uVar21 != 0) {
                uVar6 = 0;
                for (uVar7 = uVar19; (uVar7 & 1) == 0; uVar7 = uVar7 >> 1 | 0x80000000) {
                  uVar6 = uVar6 + 1;
                }
                if (*(longlong *)(*plVar3 + -0x78 + (uVar6 + uVar16 & plVar3[1]) * -0x78) == lVar5)
                {
                  if (plVar4[3] == 0) goto LAB_140adfbef;
                  auVar26 = *(undefined1 (*) [16])(plVar4 + 4);
                  auVar33._0_8_ = auVar26._0_8_;
                  auVar33._8_4_ = auVar26._0_4_;
                  auVar33._12_4_ = auVar26._4_4_;
                  auVar34._0_8_ = auVar26._8_8_;
                  auVar34._8_4_ = auVar26._8_4_;
                  auVar34._12_4_ = auVar26._12_4_;
                  core::hash::sip::impl$5::write<core::hash::sip::Sip13Rounds>();
                  local_118 = SUB168(auVar34 ^ auVar11,0);
                  uStack_110 = SUB168(auVar34 ^ auVar11,8);
                  local_128 = SUB168(auVar33 ^ auVar10,0);
                  uVar18 = local_128 + local_118;
                  lStack_120 = SUB168(auVar33 ^ auVar10,8);
                  uVar15 = (local_118 << 0xd | local_118 >> 0x33) ^ uVar18;
                  uVar13 = (uStack_110 << 0x10 | uStack_110 >> 0x30) ^ lStack_120 + uStack_110;
                  uVar16 = lStack_120 + uStack_110 + uVar15;
                  uVar18 = (uVar18 << 0x20 | uVar18 >> 0x20) + uVar13;
                  uVar15 = (uVar15 << 0x11 | uVar15 >> 0x2f) ^ uVar16;
                  uVar13 = (uVar13 << 0x15 | uVar13 >> 0x2b) ^ uVar18;
                  uVar18 = uVar18 + uVar15;
                  uVar16 = ((uVar16 << 0x20 | uVar16 >> 0x20) ^ 0xff) + uVar13;
                  uVar15 = (uVar15 << 0xd | uVar15 >> 0x33) ^ uVar18;
                  uVar13 = (uVar13 << 0x10 | uVar13 >> 0x30) ^ uVar16;
                  uVar16 = uVar16 + uVar15;
                  uVar18 = (uVar18 << 0x20 | uVar18 >> 0x20) + uVar13;
                  uVar15 = (uVar15 << 0x11 | uVar15 >> 0x2f) ^ uVar16;
                  uVar13 = (uVar13 << 0x15 | uVar13 >> 0x2b) ^ uVar18;
                  uVar18 = uVar18 + uVar15;
                  uVar16 = (uVar16 << 0x20 | uVar16 >> 0x20) + uVar13;
                  uVar15 = (uVar15 << 0xd | uVar15 >> 0x33) ^ uVar18;
                  uVar13 = (uVar13 << 0x10 | uVar13 >> 0x30) ^ uVar16;
                  uVar16 = uVar16 + uVar15;
                  uVar18 = (uVar18 << 0x20 | uVar18 >> 0x20) + uVar13;
                  uVar15 = (uVar15 << 0x11 | uVar15 >> 0x2f) ^ uVar16;
                  uVar13 = (uVar13 << 0x15 | uVar13 >> 0x2b) ^ uVar18;
                  uVar16 = (uVar16 << 0x20 | uVar16 >> 0x20) + uVar13;
                  uVar15 = (uVar15 << 0xd | uVar15 >> 0x33) ^ uVar18 + uVar15;
                  uVar13 = (uVar13 << 0x10 | uVar13 >> 0x30) ^ uVar16;
                  uVar16 = uVar16 + uVar15;
                  uVar16 = (uVar16 << 0x20 | uVar16 >> 0x20) ^ (uVar15 << 0x11 | uVar15 >> 0x2f) ^
                           (uVar13 << 0x15 | uVar13 >> 0x2b) ^ uVar16;
                  lVar20 = *plVar4;
                  bVar24 = (byte)(uVar16 >> 0x39);
                  auVar26 = ZEXT216(CONCAT11(bVar24,bVar24));
                  auVar26 = pshuflw(auVar26,auVar26,0);
                  lVar14 = 0;
                  goto LAB_140adf967;
                }
                uVar21 = (ushort)(uVar19 - 1) & (ushort)uVar19;
                uVar19 = CONCAT22((short)(uVar19 - 1 >> 0x10),uVar21);
              }
              auVar32[0] = -(auVar30[0] == cVar42);
              auVar32[1] = -(auVar30[1] == cVar43);
              auVar32[2] = -(auVar30[2] == cVar44);
              auVar32[3] = -(auVar30[3] == cVar45);
              auVar32[4] = -(auVar30[4] == cVar46);
              auVar32[5] = -(auVar30[5] == cVar47);
              auVar32[6] = -(auVar30[6] == cVar48);
              auVar32[7] = -(auVar30[7] == cVar49);
              auVar32[8] = -(auVar30[8] == cVar50);
              auVar32[9] = -(auVar30[9] == cVar51);
              auVar32[10] = -(auVar30[10] == cVar52);
              auVar32[0xb] = -(auVar30[0xb] == cVar53);
              auVar32[0xc] = -(auVar30[0xc] == cVar54);
              auVar32[0xd] = -(auVar30[0xd] == cVar55);
              auVar32[0xe] = -(auVar30[0xe] == cVar56);
              auVar32[0xf] = -(auVar30[0xf] == cVar57);
              if ((((((((((((((((SUB161(auVar32 >> 7,0) & 1) != 0 ||
                               (SUB161(auVar32 >> 0xf,0) & 1) != 0) ||
                              (SUB161(auVar32 >> 0x17,0) & 1) != 0) ||
                             (SUB161(auVar32 >> 0x1f,0) & 1) != 0) ||
                            (SUB161(auVar32 >> 0x27,0) & 1) != 0) ||
                           (SUB161(auVar32 >> 0x2f,0) & 1) != 0) ||
                          (SUB161(auVar32 >> 0x37,0) & 1) != 0) ||
                         (SUB161(auVar32 >> 0x3f,0) & 1) != 0) ||
                        (SUB161(auVar32 >> 0x47,0) & 1) != 0) ||
                       (SUB161(auVar32 >> 0x4f,0) & 1) != 0) || (SUB161(auVar32 >> 0x57,0) & 1) != 0
                      ) || (SUB161(auVar32 >> 0x5f,0) & 1) != 0) ||
                    (SUB161(auVar32 >> 0x67,0) & 1) != 0) || (SUB161(auVar32 >> 0x6f,0) & 1) != 0)
                  || (SUB161(auVar32 >> 0x77,0) & 1) != 0) || auVar32[0xf] < '\0') break;
              uVar16 = uVar16 + lVar20 + 0x10;
              lVar20 = lVar20 + 0x10;
            }
            goto LAB_140adfbf7;
          }
          lVar20 = lVar20 << 3;
          do {
            lVar20 = lVar20 + -8;
          } while (lVar20 != 0);
        }
LAB_140adf5b0:
        uVar23 = uVar23 & uVar23 - 1;
      } while (uVar23 != 0);
    }
    lVar12 = lVar12 + 8;
    local_d8 = local_d8 + 1;
    if (lVar12 == 0x200) {
      return;
    }
  } while( true );
LAB_140adf967:
  uVar16 = uVar16 & plVar4[1];
  auVar30 = *(undefined1 (*) [16])(lVar20 + uVar16);
  cVar25 = auVar26[0];
  auVar40[0] = -(auVar30[0] == cVar25);
  cVar27 = auVar26[1];
  auVar40[1] = -(auVar30[1] == cVar27);
  cVar28 = auVar26[2];
  auVar40[2] = -(auVar30[2] == cVar28);
  cVar29 = auVar26[3];
  auVar40[3] = -(auVar30[3] == cVar29);
  auVar40[4] = -(auVar30[4] == cVar25);
  auVar40[5] = -(auVar30[5] == cVar27);
  auVar40[6] = -(auVar30[6] == cVar28);
  auVar40[7] = -(auVar30[7] == cVar29);
  auVar40[8] = -(auVar30[8] == cVar25);
  auVar40[9] = -(auVar30[9] == cVar27);
  auVar40[10] = -(auVar30[10] == cVar28);
  auVar40[0xb] = -(auVar30[0xb] == cVar29);
  auVar40[0xc] = -(auVar30[0xc] == cVar25);
  auVar40[0xd] = -(auVar30[0xd] == cVar27);
  auVar40[0xe] = -(auVar30[0xe] == cVar28);
  auVar40[0xf] = -(auVar30[0xf] == cVar29);
  uVar21 = (ushort)(SUB161(auVar40 >> 7,0) & 1) | (ushort)(SUB161(auVar40 >> 0xf,0) & 1) << 1 |
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
  uVar19 = (uint)uVar21;
  while (uVar21 != 0) {
    uVar6 = 0;
    for (uVar7 = uVar19; (uVar7 & 1) == 0; uVar7 = uVar7 >> 1 | 0x80000000) {
      uVar6 = uVar6 + 1;
    }
    lVar22 = (uVar6 + uVar16 & plVar4[1]) * -0x2a8;
    if (*(longlong *)(lVar20 + -0x2a8 + lVar22) == lVar5) {
      if ((*(ulonglong *)(lVar20 + -0x290 + lVar22) < 2) ||
         (*(float *)(lVar20 + -0x270 + lVar22) <= 0.0)) {
                    /* WARNING: Subroutine does not return */
        core::panicking::panic();
      }
      if ((*(float *)(lVar20 + lVar22 + -0x24) <= fVar9) || (plVar3[0xf] == 0)) goto LAB_140adfbef;
      auVar26 = *(undefined1 (*) [16])(plVar3 + 0x10);
      auVar36._0_8_ = auVar26._0_8_;
      auVar36._8_4_ = auVar26._0_4_;
      auVar36._12_4_ = auVar26._4_4_;
      auVar37._0_8_ = auVar26._8_8_;
      auVar37._8_4_ = auVar26._8_4_;
      auVar37._12_4_ = auVar26._12_4_;
      core::hash::sip::impl$5::write<core::hash::sip::Sip13Rounds>();
      local_118 = SUB168(auVar37 ^ auVar11,0);
      uStack_110 = SUB168(auVar37 ^ auVar11,8);
      local_128 = SUB168(auVar36 ^ auVar10,0);
      uVar18 = local_128 + local_118;
      lStack_120 = SUB168(auVar36 ^ auVar10,8);
      uVar15 = (local_118 << 0xd | local_118 >> 0x33) ^ uVar18;
      uVar13 = (uStack_110 << 0x10 | uStack_110 >> 0x30) ^ lStack_120 + uStack_110;
      uVar16 = lStack_120 + uStack_110 + uVar15;
      uVar18 = (uVar18 << 0x20 | uVar18 >> 0x20) + uVar13;
      uVar15 = (uVar15 << 0x11 | uVar15 >> 0x2f) ^ uVar16;
      uVar13 = (uVar13 << 0x15 | uVar13 >> 0x2b) ^ uVar18;
      uVar18 = uVar18 + uVar15;
      uVar16 = ((uVar16 << 0x20 | uVar16 >> 0x20) ^ 0xff) + uVar13;
      uVar15 = (uVar15 << 0xd | uVar15 >> 0x33) ^ uVar18;
      uVar13 = (uVar13 << 0x10 | uVar13 >> 0x30) ^ uVar16;
      uVar16 = uVar16 + uVar15;
      uVar18 = (uVar18 << 0x20 | uVar18 >> 0x20) + uVar13;
      uVar15 = (uVar15 << 0x11 | uVar15 >> 0x2f) ^ uVar16;
      uVar13 = (uVar13 << 0x15 | uVar13 >> 0x2b) ^ uVar18;
      uVar18 = uVar18 + uVar15;
      uVar16 = (uVar16 << 0x20 | uVar16 >> 0x20) + uVar13;
      uVar15 = (uVar15 << 0xd | uVar15 >> 0x33) ^ uVar18;
      uVar13 = (uVar13 << 0x10 | uVar13 >> 0x30) ^ uVar16;
      uVar16 = uVar16 + uVar15;
      uVar18 = (uVar18 << 0x20 | uVar18 >> 0x20) + uVar13;
      uVar15 = (uVar15 << 0x11 | uVar15 >> 0x2f) ^ uVar16;
      uVar13 = (uVar13 << 0x15 | uVar13 >> 0x2b) ^ uVar18;
      uVar16 = (uVar16 << 0x20 | uVar16 >> 0x20) + uVar13;
      uVar15 = (uVar15 << 0xd | uVar15 >> 0x33) ^ uVar18 + uVar15;
      uVar13 = (uVar13 << 0x10 | uVar13 >> 0x30) ^ uVar16;
      uVar16 = uVar16 + uVar15;
      uVar16 = (uVar16 << 0x20 | uVar16 >> 0x20) ^ (uVar15 << 0x11 | uVar15 >> 0x2f) ^
               (uVar13 << 0x15 | uVar13 >> 0x2b) ^ uVar16;
      bVar24 = (byte)(uVar16 >> 0x39);
      auVar26 = ZEXT216(CONCAT11(bVar24,bVar24));
      auVar26 = pshuflw(auVar26,auVar26,0);
      lVar20 = 0;
      goto LAB_140adfb7f;
    }
    uVar21 = (ushort)(uVar19 - 1) & (ushort)uVar19;
    uVar19 = CONCAT22((short)(uVar19 - 1 >> 0x10),uVar21);
  }
  auVar35[0] = -(auVar30[0] == cVar42);
  auVar35[1] = -(auVar30[1] == cVar43);
  auVar35[2] = -(auVar30[2] == cVar44);
  auVar35[3] = -(auVar30[3] == cVar45);
  auVar35[4] = -(auVar30[4] == cVar46);
  auVar35[5] = -(auVar30[5] == cVar47);
  auVar35[6] = -(auVar30[6] == cVar48);
  auVar35[7] = -(auVar30[7] == cVar49);
  auVar35[8] = -(auVar30[8] == cVar50);
  auVar35[9] = -(auVar30[9] == cVar51);
  auVar35[10] = -(auVar30[10] == cVar52);
  auVar35[0xb] = -(auVar30[0xb] == cVar53);
  auVar35[0xc] = -(auVar30[0xc] == cVar54);
  auVar35[0xd] = -(auVar30[0xd] == cVar55);
  auVar35[0xe] = -(auVar30[0xe] == cVar56);
  auVar35[0xf] = -(auVar30[0xf] == cVar57);
  if ((((((((((((((((SUB161(auVar35 >> 7,0) & 1) != 0 || (SUB161(auVar35 >> 0xf,0) & 1) != 0) ||
                  (SUB161(auVar35 >> 0x17,0) & 1) != 0) || (SUB161(auVar35 >> 0x1f,0) & 1) != 0) ||
                (SUB161(auVar35 >> 0x27,0) & 1) != 0) || (SUB161(auVar35 >> 0x2f,0) & 1) != 0) ||
              (SUB161(auVar35 >> 0x37,0) & 1) != 0) || (SUB161(auVar35 >> 0x3f,0) & 1) != 0) ||
            (SUB161(auVar35 >> 0x47,0) & 1) != 0) || (SUB161(auVar35 >> 0x4f,0) & 1) != 0) ||
          (SUB161(auVar35 >> 0x57,0) & 1) != 0) || (SUB161(auVar35 >> 0x5f,0) & 1) != 0) ||
        (SUB161(auVar35 >> 0x67,0) & 1) != 0) || (SUB161(auVar35 >> 0x6f,0) & 1) != 0) ||
      (SUB161(auVar35 >> 0x77,0) & 1) != 0) || auVar35[0xf] < '\0') goto LAB_140adfbef;
  uVar16 = uVar16 + lVar14 + 0x10;
  lVar14 = lVar14 + 0x10;
  goto LAB_140adf967;
LAB_140adfb7f:
  uVar16 = uVar16 & plVar3[0xd];
  auVar30 = *(undefined1 (*) [16])(plVar3[0xc] + uVar16);
  cVar25 = auVar26[0];
  auVar41[0] = -(auVar30[0] == cVar25);
  cVar27 = auVar26[1];
  auVar41[1] = -(auVar30[1] == cVar27);
  cVar28 = auVar26[2];
  auVar41[2] = -(auVar30[2] == cVar28);
  cVar29 = auVar26[3];
  auVar41[3] = -(auVar30[3] == cVar29);
  auVar41[4] = -(auVar30[4] == cVar25);
  auVar41[5] = -(auVar30[5] == cVar27);
  auVar41[6] = -(auVar30[6] == cVar28);
  auVar41[7] = -(auVar30[7] == cVar29);
  auVar41[8] = -(auVar30[8] == cVar25);
  auVar41[9] = -(auVar30[9] == cVar27);
  auVar41[10] = -(auVar30[10] == cVar28);
  auVar41[0xb] = -(auVar30[0xb] == cVar29);
  auVar41[0xc] = -(auVar30[0xc] == cVar25);
  auVar41[0xd] = -(auVar30[0xd] == cVar27);
  auVar41[0xe] = -(auVar30[0xe] == cVar28);
  auVar41[0xf] = -(auVar30[0xf] == cVar29);
  uVar21 = (ushort)(SUB161(auVar41 >> 7,0) & 1) | (ushort)(SUB161(auVar41 >> 0xf,0) & 1) << 1 |
           (ushort)(SUB161(auVar41 >> 0x17,0) & 1) << 2 |
           (ushort)(SUB161(auVar41 >> 0x1f,0) & 1) << 3 |
           (ushort)(SUB161(auVar41 >> 0x27,0) & 1) << 4 |
           (ushort)(SUB161(auVar41 >> 0x2f,0) & 1) << 5 |
           (ushort)(SUB161(auVar41 >> 0x37,0) & 1) << 6 |
           (ushort)(SUB161(auVar41 >> 0x3f,0) & 1) << 7 |
           (ushort)(SUB161(auVar41 >> 0x47,0) & 1) << 8 |
           (ushort)(SUB161(auVar41 >> 0x4f,0) & 1) << 9 |
           (ushort)(SUB161(auVar41 >> 0x57,0) & 1) << 10 |
           (ushort)(SUB161(auVar41 >> 0x5f,0) & 1) << 0xb |
           (ushort)(SUB161(auVar41 >> 0x67,0) & 1) << 0xc |
           (ushort)(SUB161(auVar41 >> 0x6f,0) & 1) << 0xd |
           (ushort)(SUB161(auVar41 >> 0x77,0) & 1) << 0xe | (ushort)(auVar41[0xf] >> 7) << 0xf;
  uVar19 = (uint)uVar21;
  while (uVar21 != 0) {
    uVar6 = 0;
    for (uVar7 = uVar19; (uVar7 & 1) == 0; uVar7 = uVar7 >> 1 | 0x80000000) {
      uVar6 = uVar6 + 1;
    }
    if (lVar5 == *(longlong *)(plVar3[0xc] + (uVar6 + uVar16 & plVar3[0xd]) * -8 + -8))
    goto LAB_140adfbf7;
    uVar21 = (ushort)(uVar19 - 1) & (ushort)uVar19;
    uVar19 = CONCAT22((short)(uVar19 - 1 >> 0x10),uVar21);
  }
  auVar38[0] = -(auVar30[0] == cVar42);
  auVar38[1] = -(auVar30[1] == cVar43);
  auVar38[2] = -(auVar30[2] == cVar44);
  auVar38[3] = -(auVar30[3] == cVar45);
  auVar38[4] = -(auVar30[4] == cVar46);
  auVar38[5] = -(auVar30[5] == cVar47);
  auVar38[6] = -(auVar30[6] == cVar48);
  auVar38[7] = -(auVar30[7] == cVar49);
  auVar38[8] = -(auVar30[8] == cVar50);
  auVar38[9] = -(auVar30[9] == cVar51);
  auVar38[10] = -(auVar30[10] == cVar52);
  auVar38[0xb] = -(auVar30[0xb] == cVar53);
  auVar38[0xc] = -(auVar30[0xc] == cVar54);
  auVar38[0xd] = -(auVar30[0xd] == cVar55);
  auVar38[0xe] = -(auVar30[0xe] == cVar56);
  auVar38[0xf] = -(auVar30[0xf] == cVar57);
  if ((((((((((((((((SUB161(auVar38 >> 7,0) & 1) != 0 || (SUB161(auVar38 >> 0xf,0) & 1) != 0) ||
                  (SUB161(auVar38 >> 0x17,0) & 1) != 0) || (SUB161(auVar38 >> 0x1f,0) & 1) != 0) ||
                (SUB161(auVar38 >> 0x27,0) & 1) != 0) || (SUB161(auVar38 >> 0x2f,0) & 1) != 0) ||
              (SUB161(auVar38 >> 0x37,0) & 1) != 0) || (SUB161(auVar38 >> 0x3f,0) & 1) != 0) ||
            (SUB161(auVar38 >> 0x47,0) & 1) != 0) || (SUB161(auVar38 >> 0x4f,0) & 1) != 0) ||
          (SUB161(auVar38 >> 0x57,0) & 1) != 0) || (SUB161(auVar38 >> 0x5f,0) & 1) != 0) ||
        (SUB161(auVar38 >> 0x67,0) & 1) != 0) || (SUB161(auVar38 >> 0x6f,0) & 1) != 0) ||
      (SUB161(auVar38 >> 0x77,0) & 1) != 0) || auVar38[0xf] < '\0') goto LAB_140adfbef;
  uVar16 = uVar16 + lVar20 + 0x10;
  lVar20 = lVar20 + 0x10;
  goto LAB_140adfb7f;
LAB_140adfbef:
  hashbrown::map::HashMap::
  insert<country_core::resources::walls::WallId,tuple_<>,std::hash::random::RandomState,alloc::alloc::Global>
            ();
LAB_140adfbf7:
  do {
    plVar17 = plVar17 + 1;
    if (plVar17 == plVar1) goto LAB_140adf5b0;
  } while (plVar3[3] == 0);
  goto LAB_140adf620;
}

