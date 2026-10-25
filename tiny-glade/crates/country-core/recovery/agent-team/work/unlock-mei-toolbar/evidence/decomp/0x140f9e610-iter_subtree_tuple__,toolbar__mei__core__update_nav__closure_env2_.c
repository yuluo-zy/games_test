
void __cdecl
toolbar::mei::core::iter_subtree<tuple_<>,toolbar::mei::core::update_nav::closure_env_2>(void)

{
  char *pcVar1;
  undefined8 uVar2;
  undefined8 uVar3;
  longlong *plVar4;
  longlong lVar5;
  float *pfVar6;
  undefined8 *puVar7;
  uint uVar8;
  int iVar9;
  int iVar10;
  int iVar11;
  int iVar12;
  uint uVar13;
  ulonglong extraout_RAX;
  ulonglong extraout_RAX_00;
  uint uVar14;
  longlong in_RCX;
  longlong in_RDX;
  longlong *in_R8;
  ulonglong uVar15;
  longlong lVar16;
  ushort uVar17;
  longlong lVar18;
  byte bVar19;
  char cVar20;
  char cVar21;
  char cVar28;
  float fVar22;
  char cVar24;
  char cVar25;
  char cVar26;
  char cVar27;
  char cVar29;
  undefined1 auVar23 [16];
  undefined1 in_XMM1_Ba;
  char cVar30;
  undefined1 in_XMM1_Bb;
  char cVar31;
  char cVar32;
  undefined1 in_XMM1_Bc;
  char cVar33;
  char cVar34;
  undefined1 in_XMM1_Bd;
  char cVar35;
  char cVar36;
  undefined1 in_XMM1_Be;
  undefined1 in_XMM1_Bf;
  undefined1 in_XMM1_Bg;
  undefined1 in_XMM1_Bh;
  undefined1 in_XMM1_Bi;
  char cVar37;
  undefined1 in_XMM1_Bj;
  char cVar38;
  undefined1 in_XMM1_Bk;
  char cVar39;
  undefined1 in_XMM1_Bl;
  char cVar40;
  undefined1 in_XMM1_Bm;
  undefined1 in_XMM1_Bn;
  undefined1 in_XMM1_Bo;
  undefined1 in_XMM1_Bp;
  float fVar41;
  float fVar44;
  undefined1 auVar42 [16];
  undefined1 auVar43 [16];
  float fVar45;
  float fVar49;
  undefined1 auVar47 [16];
  float fVar46;
  float fVar50;
  undefined1 auVar48 [16];
  
  plVar4 = (longlong *)*in_R8;
  if (plVar4[3] != 0) {
    ::core::hash::BuildHasher::
    hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>();
    lVar5 = *plVar4;
    bVar19 = (byte)(extraout_RAX >> 0x39);
    auVar23 = ZEXT216(CONCAT11(bVar19,bVar19));
    auVar23 = pshuflw(auVar23,auVar23,0);
    lVar16 = 0;
    iVar9 = -(uint)(CONCAT13(in_XMM1_Bd,CONCAT12(in_XMM1_Bc,CONCAT11(in_XMM1_Bb,in_XMM1_Ba))) ==
                   CONCAT13(in_XMM1_Bd,CONCAT12(in_XMM1_Bc,CONCAT11(in_XMM1_Bb,in_XMM1_Ba))));
    cVar30 = (char)iVar9;
    cVar31 = (char)((uint)iVar9 >> 8);
    cVar33 = (char)((uint)iVar9 >> 0x10);
    cVar35 = (char)((uint)iVar9 >> 0x18);
    iVar9 = -(uint)(CONCAT13(in_XMM1_Bh,CONCAT12(in_XMM1_Bg,CONCAT11(in_XMM1_Bf,in_XMM1_Be))) ==
                   CONCAT13(in_XMM1_Bh,CONCAT12(in_XMM1_Bg,CONCAT11(in_XMM1_Bf,in_XMM1_Be))));
    cVar29 = (char)iVar9;
    cVar32 = (char)((uint)iVar9 >> 8);
    cVar34 = (char)((uint)iVar9 >> 0x10);
    cVar36 = (char)((uint)iVar9 >> 0x18);
    iVar9 = -(uint)(CONCAT13(in_XMM1_Bl,CONCAT12(in_XMM1_Bk,CONCAT11(in_XMM1_Bj,in_XMM1_Bi))) ==
                   CONCAT13(in_XMM1_Bl,CONCAT12(in_XMM1_Bk,CONCAT11(in_XMM1_Bj,in_XMM1_Bi))));
    cVar37 = (char)iVar9;
    cVar38 = (char)((uint)iVar9 >> 8);
    cVar39 = (char)((uint)iVar9 >> 0x10);
    cVar40 = (char)((uint)iVar9 >> 0x18);
    iVar9 = -(uint)(CONCAT13(in_XMM1_Bp,CONCAT12(in_XMM1_Bo,CONCAT11(in_XMM1_Bn,in_XMM1_Bm))) ==
                   CONCAT13(in_XMM1_Bp,CONCAT12(in_XMM1_Bo,CONCAT11(in_XMM1_Bn,in_XMM1_Bm))));
    cVar21 = (char)((uint)iVar9 >> 8);
    cVar25 = (char)((uint)iVar9 >> 0x10);
    cVar27 = (char)((uint)iVar9 >> 0x18);
    uVar15 = extraout_RAX;
    while( true ) {
      uVar15 = plVar4[1] & uVar15;
      pcVar1 = (char *)(lVar5 + uVar15);
      cVar20 = auVar23[0];
      auVar47[0] = -(*pcVar1 == cVar20);
      cVar24 = auVar23[1];
      auVar47[1] = -(pcVar1[1] == cVar24);
      cVar26 = auVar23[2];
      auVar47[2] = -(pcVar1[2] == cVar26);
      cVar28 = auVar23[3];
      auVar47[3] = -(pcVar1[3] == cVar28);
      auVar47[4] = -(pcVar1[4] == cVar20);
      auVar47[5] = -(pcVar1[5] == cVar24);
      auVar47[6] = -(pcVar1[6] == cVar26);
      auVar47[7] = -(pcVar1[7] == cVar28);
      auVar47[8] = -(pcVar1[8] == cVar20);
      auVar47[9] = -(pcVar1[9] == cVar24);
      auVar47[10] = -(pcVar1[10] == cVar26);
      auVar47[0xb] = -(pcVar1[0xb] == cVar28);
      auVar47[0xc] = -(pcVar1[0xc] == cVar20);
      auVar47[0xd] = -(pcVar1[0xd] == cVar24);
      auVar47[0xe] = -(pcVar1[0xe] == cVar26);
      auVar47[0xf] = -(pcVar1[0xf] == cVar28);
      uVar17 = (ushort)(SUB161(auVar47 >> 7,0) & 1) | (ushort)(SUB161(auVar47 >> 0xf,0) & 1) << 1 |
               (ushort)(SUB161(auVar47 >> 0x17,0) & 1) << 2 |
               (ushort)(SUB161(auVar47 >> 0x1f,0) & 1) << 3 |
               (ushort)(SUB161(auVar47 >> 0x27,0) & 1) << 4 |
               (ushort)(SUB161(auVar47 >> 0x2f,0) & 1) << 5 |
               (ushort)(SUB161(auVar47 >> 0x37,0) & 1) << 6 |
               (ushort)(SUB161(auVar47 >> 0x3f,0) & 1) << 7 |
               (ushort)(SUB161(auVar47 >> 0x47,0) & 1) << 8 |
               (ushort)(SUB161(auVar47 >> 0x4f,0) & 1) << 9 |
               (ushort)(SUB161(auVar47 >> 0x57,0) & 1) << 10 |
               (ushort)(SUB161(auVar47 >> 0x5f,0) & 1) << 0xb |
               (ushort)(SUB161(auVar47 >> 0x67,0) & 1) << 0xc |
               (ushort)(SUB161(auVar47 >> 0x6f,0) & 1) << 0xd |
               (ushort)(SUB161(auVar47 >> 0x77,0) & 1) << 0xe | (ushort)(auVar47[0xf] >> 7) << 0xf;
      uVar13 = (uint)uVar17;
      while (uVar17 != 0) {
        uVar14 = 0;
        for (uVar8 = uVar13; (uVar8 & 1) == 0; uVar8 = uVar8 >> 1 | 0x80000000) {
          uVar14 = uVar14 + 1;
        }
        lVar18 = (uVar14 + uVar15 & plVar4[1]) * -0xc0;
        if (*(longlong *)(lVar5 + -0xc0 + lVar18) == in_RDX) {
          cVar20 = (char)iVar9;
          if ((*(byte *)(lVar5 + -1 + lVar18) & 0x20) != 0) {
            uVar2 = *(undefined8 *)(lVar5 + lVar18 + -0x48);
            uVar3 = *(undefined8 *)(lVar5 + lVar18 + -0x40);
            fVar41 = ((float)uVar2 * 0.5 + (float)uVar3) - (float)*(undefined8 *)in_R8[1];
            fVar44 = ((float)((ulonglong)uVar2 >> 0x20) * 0.5 + (float)((ulonglong)uVar3 >> 0x20)) -
                     (float)((ulonglong)*(undefined8 *)in_R8[1] >> 0x20);
            fVar22 = SQRT(fVar44 * fVar44 + fVar41 * fVar41);
            fVar45 = 1.0 / fVar22;
            uVar14 = (uint)((int)fVar45 - 1U < 0x7fffff ||
                           (int)ABS(fVar45) - 0x800000U < 0x7f000000 && -1 < (int)fVar45);
            fVar46 = (float)*(undefined8 *)in_R8[2];
            fVar49 = (float)((ulonglong)*(undefined8 *)in_R8[2] >> 0x20);
            fVar50 = 1.0 / SQRT(fVar49 * fVar49 + fVar46 * fVar46);
            fVar49 = fVar50 * fVar49;
            uVar13 = (uint)((int)fVar50 - 1U < 0x7fffff ||
                           (int)ABS(fVar50) - 0x800000U < 0x7f000000 && -1 < (int)fVar50);
            uVar15 = CONCAT44((int)(uVar13 << 0x1f) >> 0x1f,(int)(uVar13 << 0x1f) >> 0x1f) &
                     CONCAT17((char)((uint)fVar49 >> 0x18),
                              CONCAT16((char)((uint)fVar49 >> 0x10),
                                       CONCAT15((char)((uint)fVar49 >> 8),
                                                CONCAT14(SUB41(fVar49,0),fVar50 * fVar46))));
            fVar50 = (float)(uVar15 >> 0x20) *
                     (float)((int)(uVar14 << 0x1f) >> 0x1f & (uint)(fVar45 * fVar44));
            cVar29 = SUB41(fVar50,0);
            cVar32 = (char)((uint)fVar50 >> 8);
            cVar34 = (char)((uint)fVar50 >> 0x10);
            cVar36 = (char)((uint)fVar50 >> 0x18);
            cVar37 = '\0';
            cVar38 = '\0';
            cVar39 = '\0';
            cVar40 = '\0';
            fVar50 = fVar50 + (float)uVar15 *
                              (float)((int)(uVar14 << 0x1f) >> 0x1f & (uint)(fVar45 * fVar41));
            cVar30 = SUB41(fVar50,0);
            cVar31 = (char)((uint)fVar50 >> 8);
            cVar33 = (char)((uint)fVar50 >> 0x10);
            cVar35 = (char)((uint)fVar50 >> 0x18);
            cVar20 = cVar37;
            cVar21 = cVar38;
            cVar25 = cVar39;
            cVar27 = cVar40;
            if (0.0 < fVar50) {
              pfVar6 = (float *)in_R8[3];
              fVar50 = *pfVar6;
              cVar30 = SUB41(fVar50,0);
              cVar31 = (char)((uint)fVar50 >> 8);
              cVar33 = (char)((uint)fVar50 >> 0x10);
              cVar35 = (char)((uint)fVar50 >> 0x18);
              cVar29 = '\0';
              cVar32 = '\0';
              cVar34 = '\0';
              cVar36 = '\0';
              cVar37 = '\0';
              cVar38 = '\0';
              cVar39 = '\0';
              cVar40 = '\0';
              cVar20 = '\0';
              cVar21 = '\0';
              cVar25 = '\0';
              cVar27 = '\0';
              if (fVar22 < fVar50) {
                puVar7 = (undefined8 *)in_R8[4];
                *puVar7 = 1;
                puVar7[1] = in_RDX;
                *pfVar6 = fVar22;
              }
            }
          }
          if (*(longlong *)(in_RCX + 0x1b8) != 0) {
            ::core::hash::BuildHasher::
            hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>();
            lVar5 = *(longlong *)(in_RCX + 0x1a0);
            bVar19 = (byte)(extraout_RAX_00 >> 0x39);
            auVar23 = ZEXT216(CONCAT11(bVar19,bVar19));
            auVar23 = pshuflw(auVar23,auVar23,0);
            lVar16 = 0;
            iVar9 = -(uint)(CONCAT13(cVar35,CONCAT12(cVar33,CONCAT11(cVar31,cVar30))) ==
                           CONCAT13(cVar35,CONCAT12(cVar33,CONCAT11(cVar31,cVar30))));
            iVar10 = -(uint)(CONCAT13(cVar36,CONCAT12(cVar34,CONCAT11(cVar32,cVar29))) ==
                            CONCAT13(cVar36,CONCAT12(cVar34,CONCAT11(cVar32,cVar29))));
            iVar11 = -(uint)(CONCAT13(cVar40,CONCAT12(cVar39,CONCAT11(cVar38,cVar37))) ==
                            CONCAT13(cVar40,CONCAT12(cVar39,CONCAT11(cVar38,cVar37))));
            iVar12 = -(uint)(CONCAT13(cVar27,CONCAT12(cVar25,CONCAT11(cVar21,cVar20))) ==
                            CONCAT13(cVar27,CONCAT12(cVar25,CONCAT11(cVar21,cVar20))));
            uVar15 = extraout_RAX_00;
            while( true ) {
              uVar15 = *(ulonglong *)(in_RCX + 0x1a8) & uVar15;
              pcVar1 = (char *)(lVar5 + uVar15);
              cVar21 = auVar23[0];
              auVar48[0] = -(*pcVar1 == cVar21);
              cVar25 = auVar23[1];
              auVar48[1] = -(pcVar1[1] == cVar25);
              cVar27 = auVar23[2];
              auVar48[2] = -(pcVar1[2] == cVar27);
              cVar29 = auVar23[3];
              auVar48[3] = -(pcVar1[3] == cVar29);
              auVar48[4] = -(pcVar1[4] == cVar21);
              auVar48[5] = -(pcVar1[5] == cVar25);
              auVar48[6] = -(pcVar1[6] == cVar27);
              auVar48[7] = -(pcVar1[7] == cVar29);
              auVar48[8] = -(pcVar1[8] == cVar21);
              auVar48[9] = -(pcVar1[9] == cVar25);
              auVar48[10] = -(pcVar1[10] == cVar27);
              auVar48[0xb] = -(pcVar1[0xb] == cVar29);
              auVar48[0xc] = -(pcVar1[0xc] == cVar21);
              auVar48[0xd] = -(pcVar1[0xd] == cVar25);
              auVar48[0xe] = -(pcVar1[0xe] == cVar27);
              auVar48[0xf] = -(pcVar1[0xf] == cVar29);
              uVar17 = (ushort)(SUB161(auVar48 >> 7,0) & 1) |
                       (ushort)(SUB161(auVar48 >> 0xf,0) & 1) << 1 |
                       (ushort)(SUB161(auVar48 >> 0x17,0) & 1) << 2 |
                       (ushort)(SUB161(auVar48 >> 0x1f,0) & 1) << 3 |
                       (ushort)(SUB161(auVar48 >> 0x27,0) & 1) << 4 |
                       (ushort)(SUB161(auVar48 >> 0x2f,0) & 1) << 5 |
                       (ushort)(SUB161(auVar48 >> 0x37,0) & 1) << 6 |
                       (ushort)(SUB161(auVar48 >> 0x3f,0) & 1) << 7 |
                       (ushort)(SUB161(auVar48 >> 0x47,0) & 1) << 8 |
                       (ushort)(SUB161(auVar48 >> 0x4f,0) & 1) << 9 |
                       (ushort)(SUB161(auVar48 >> 0x57,0) & 1) << 10 |
                       (ushort)(SUB161(auVar48 >> 0x5f,0) & 1) << 0xb |
                       (ushort)(SUB161(auVar48 >> 0x67,0) & 1) << 0xc |
                       (ushort)(SUB161(auVar48 >> 0x6f,0) & 1) << 0xd |
                       (ushort)(SUB161(auVar48 >> 0x77,0) & 1) << 0xe |
                       (ushort)(auVar48[0xf] >> 7) << 0xf;
              uVar13 = (uint)uVar17;
              while (uVar17 != 0) {
                uVar14 = 0;
                for (uVar8 = uVar13; (uVar8 & 1) == 0; uVar8 = uVar8 >> 1 | 0x80000000) {
                  uVar14 = uVar14 + 1;
                }
                lVar18 = (uVar14 + uVar15 & *(ulonglong *)(in_RCX + 0x1a8)) * -0xc0;
                if (*(longlong *)(lVar5 + -0xc0 + lVar18) == in_RDX) {
                  lVar5 = *(longlong *)(lVar5 + -0xa8 + lVar18);
                  if (lVar5 != 0) {
                    lVar16 = 0;
                    do {
                      iter_subtree<tuple_<>,toolbar::mei::core::update_nav::closure_env_2>();
                      lVar16 = lVar16 + 8;
                    } while (lVar5 << 3 != lVar16);
                  }
                  return;
                }
                uVar17 = (ushort)(uVar13 - 1) & (ushort)uVar13;
                uVar13 = CONCAT22((short)(uVar13 - 1 >> 0x10),uVar17);
              }
              auVar43[0] = -(*pcVar1 == (char)iVar9);
              auVar43[1] = -(pcVar1[1] == (char)((uint)iVar9 >> 8));
              auVar43[2] = -(pcVar1[2] == (char)((uint)iVar9 >> 0x10));
              auVar43[3] = -(pcVar1[3] == (char)((uint)iVar9 >> 0x18));
              auVar43[4] = -(pcVar1[4] == (char)iVar10);
              auVar43[5] = -(pcVar1[5] == (char)((uint)iVar10 >> 8));
              auVar43[6] = -(pcVar1[6] == (char)((uint)iVar10 >> 0x10));
              auVar43[7] = -(pcVar1[7] == (char)((uint)iVar10 >> 0x18));
              auVar43[8] = -(pcVar1[8] == (char)iVar11);
              auVar43[9] = -(pcVar1[9] == (char)((uint)iVar11 >> 8));
              auVar43[10] = -(pcVar1[10] == (char)((uint)iVar11 >> 0x10));
              auVar43[0xb] = -(pcVar1[0xb] == (char)((uint)iVar11 >> 0x18));
              auVar43[0xc] = -(pcVar1[0xc] == (char)iVar12);
              auVar43[0xd] = -(pcVar1[0xd] == (char)((uint)iVar12 >> 8));
              auVar43[0xe] = -(pcVar1[0xe] == (char)((uint)iVar12 >> 0x10));
              auVar43[0xf] = -(pcVar1[0xf] == (char)((uint)iVar12 >> 0x18));
              if ((((((((((((((((SUB161(auVar43 >> 7,0) & 1) != 0 ||
                               (SUB161(auVar43 >> 0xf,0) & 1) != 0) ||
                              (SUB161(auVar43 >> 0x17,0) & 1) != 0) ||
                             (SUB161(auVar43 >> 0x1f,0) & 1) != 0) ||
                            (SUB161(auVar43 >> 0x27,0) & 1) != 0) ||
                           (SUB161(auVar43 >> 0x2f,0) & 1) != 0) ||
                          (SUB161(auVar43 >> 0x37,0) & 1) != 0) ||
                         (SUB161(auVar43 >> 0x3f,0) & 1) != 0) ||
                        (SUB161(auVar43 >> 0x47,0) & 1) != 0) ||
                       (SUB161(auVar43 >> 0x4f,0) & 1) != 0) || (SUB161(auVar43 >> 0x57,0) & 1) != 0
                      ) || (SUB161(auVar43 >> 0x5f,0) & 1) != 0) ||
                    (SUB161(auVar43 >> 0x67,0) & 1) != 0) || (SUB161(auVar43 >> 0x6f,0) & 1) != 0)
                  || (SUB161(auVar43 >> 0x77,0) & 1) != 0) || auVar43[0xf] < '\0') break;
              uVar15 = uVar15 + lVar16 + 0x10;
              lVar16 = lVar16 + 0x10;
            }
          }
                    // WARNING: Subroutine does not return
          ::core::option::expect_failed();
        }
        uVar17 = (ushort)(uVar13 - 1) & (ushort)uVar13;
        uVar13 = CONCAT22((short)(uVar13 - 1 >> 0x10),uVar17);
      }
      auVar42[0] = -(*pcVar1 == cVar30);
      auVar42[1] = -(pcVar1[1] == cVar31);
      auVar42[2] = -(pcVar1[2] == cVar33);
      auVar42[3] = -(pcVar1[3] == cVar35);
      auVar42[4] = -(pcVar1[4] == cVar29);
      auVar42[5] = -(pcVar1[5] == cVar32);
      auVar42[6] = -(pcVar1[6] == cVar34);
      auVar42[7] = -(pcVar1[7] == cVar36);
      auVar42[8] = -(pcVar1[8] == cVar37);
      auVar42[9] = -(pcVar1[9] == cVar38);
      auVar42[10] = -(pcVar1[10] == cVar39);
      auVar42[0xb] = -(pcVar1[0xb] == cVar40);
      auVar42[0xc] = -(pcVar1[0xc] == (char)iVar9);
      auVar42[0xd] = -(pcVar1[0xd] == cVar21);
      auVar42[0xe] = -(pcVar1[0xe] == cVar25);
      auVar42[0xf] = -(pcVar1[0xf] == cVar27);
      if ((((((((((((((((SUB161(auVar42 >> 7,0) & 1) != 0 || (SUB161(auVar42 >> 0xf,0) & 1) != 0) ||
                      (SUB161(auVar42 >> 0x17,0) & 1) != 0) || (SUB161(auVar42 >> 0x1f,0) & 1) != 0)
                    || (SUB161(auVar42 >> 0x27,0) & 1) != 0) || (SUB161(auVar42 >> 0x2f,0) & 1) != 0
                   ) || (SUB161(auVar42 >> 0x37,0) & 1) != 0) ||
                 (SUB161(auVar42 >> 0x3f,0) & 1) != 0) || (SUB161(auVar42 >> 0x47,0) & 1) != 0) ||
               (SUB161(auVar42 >> 0x4f,0) & 1) != 0) || (SUB161(auVar42 >> 0x57,0) & 1) != 0) ||
             (SUB161(auVar42 >> 0x5f,0) & 1) != 0) || (SUB161(auVar42 >> 0x67,0) & 1) != 0) ||
           (SUB161(auVar42 >> 0x6f,0) & 1) != 0) || (SUB161(auVar42 >> 0x77,0) & 1) != 0) ||
          auVar42[0xf] < '\0') break;
      uVar15 = uVar15 + lVar16 + 0x10;
      lVar16 = lVar16 + 0x10;
    }
  }
                    // WARNING: Subroutine does not return
  ::core::option::expect_failed();
}

