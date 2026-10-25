// debug_viz_threshold_curve  size=1105
// void __cdecl debug_viz_threshold_curve(void)

void __cdecl
country_core::systems::wall::wall_state::debug_viz_threshold_curve::debug_viz_threshold_curve(void)

{
  char *pcVar1;
  longlong *plVar2;
  longlong lVar3;
  uint uVar4;
  uint uVar5;
  int iVar6;
  int iVar7;
  int iVar8;
  int iVar9;
  uint uVar10;
  ulonglong extraout_RAX;
  longlong *in_RCX;
  longlong lVar11;
  undefined8 *in_RDX;
  ulonglong uVar12;
  longlong lVar13;
  ushort uVar14;
  longlong lVar15;
  undefined1 (*pauVar16) [16];
  undefined1 (*pauVar17) [16];
  longlong lVar18;
  ulonglong uVar19;
  byte bVar20;
  char cVar21;
  char cVar23;
  char cVar24;
  char cVar25;
  undefined1 auVar22 [16];
  undefined1 auVar26 [16];
  undefined1 auVar27 [16];
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
  uint local_e0;
  
  if (*(char *)(*in_RCX + 0xf3) == '\x01') {
    plVar2 = (longlong *)*in_RDX;
    lVar18 = plVar2[3];
    if (lVar18 != 0) {
      pauVar16 = (undefined1 (*) [16])*plVar2;
      pauVar17 = pauVar16 + 1;
      auVar22 = *pauVar16;
      local_e0 = ~(uint)(ushort)((ushort)(SUB161(auVar22 >> 7,0) & 1) |
                                 (ushort)(SUB161(auVar22 >> 0xf,0) & 1) << 1 |
                                 (ushort)(SUB161(auVar22 >> 0x17,0) & 1) << 2 |
                                 (ushort)(SUB161(auVar22 >> 0x1f,0) & 1) << 3 |
                                 (ushort)(SUB161(auVar22 >> 0x27,0) & 1) << 4 |
                                 (ushort)(SUB161(auVar22 >> 0x2f,0) & 1) << 5 |
                                 (ushort)(SUB161(auVar22 >> 0x37,0) & 1) << 6 |
                                 (ushort)(SUB161(auVar22 >> 0x3f,0) & 1) << 7 |
                                 (ushort)(SUB161(auVar22 >> 0x47,0) & 1) << 8 |
                                 (ushort)(SUB161(auVar22 >> 0x4f,0) & 1) << 9 |
                                 (ushort)(SUB161(auVar22 >> 0x57,0) & 1) << 10 |
                                 (ushort)(SUB161(auVar22 >> 0x5f,0) & 1) << 0xb |
                                 (ushort)(SUB161(auVar22 >> 0x67,0) & 1) << 0xc |
                                 (ushort)(SUB161(auVar22 >> 0x6f,0) & 1) << 0xd |
                                 (ushort)(SUB161(auVar22 >> 0x77,0) & 1) << 0xe |
                                (ushort)(byte)(auVar22[0xf] >> 7) << 0xf);
      iVar6 = -(uint)(CONCAT13(unaff_XMM6_Bd,
                               CONCAT12(unaff_XMM6_Bc,CONCAT11(unaff_XMM6_Bb,unaff_XMM6_Ba))) ==
                     CONCAT13(unaff_XMM6_Bd,
                              CONCAT12(unaff_XMM6_Bc,CONCAT11(unaff_XMM6_Bb,unaff_XMM6_Ba))));
      iVar7 = -(uint)(CONCAT13(unaff_XMM6_Bh,
                               CONCAT12(unaff_XMM6_Bg,CONCAT11(unaff_XMM6_Bf,unaff_XMM6_Be))) ==
                     CONCAT13(unaff_XMM6_Bh,
                              CONCAT12(unaff_XMM6_Bg,CONCAT11(unaff_XMM6_Bf,unaff_XMM6_Be))));
      iVar8 = -(uint)(CONCAT13(unaff_XMM6_Bl,
                               CONCAT12(unaff_XMM6_Bk,CONCAT11(unaff_XMM6_Bj,unaff_XMM6_Bi))) ==
                     CONCAT13(unaff_XMM6_Bl,
                              CONCAT12(unaff_XMM6_Bk,CONCAT11(unaff_XMM6_Bj,unaff_XMM6_Bi))));
      iVar9 = -(uint)(CONCAT13(unaff_XMM6_Bp,
                               CONCAT12(unaff_XMM6_Bo,CONCAT11(unaff_XMM6_Bn,unaff_XMM6_Bm))) ==
                     CONCAT13(unaff_XMM6_Bp,
                              CONCAT12(unaff_XMM6_Bo,CONCAT11(unaff_XMM6_Bn,unaff_XMM6_Bm))));
      do {
        if ((short)local_e0 == 0) {
          do {
            auVar22 = *pauVar17;
            uVar14 = (ushort)(SUB161(auVar22 >> 7,0) & 1) |
                     (ushort)(SUB161(auVar22 >> 0xf,0) & 1) << 1 |
                     (ushort)(SUB161(auVar22 >> 0x17,0) & 1) << 2 |
                     (ushort)(SUB161(auVar22 >> 0x1f,0) & 1) << 3 |
                     (ushort)(SUB161(auVar22 >> 0x27,0) & 1) << 4 |
                     (ushort)(SUB161(auVar22 >> 0x2f,0) & 1) << 5 |
                     (ushort)(SUB161(auVar22 >> 0x37,0) & 1) << 6 |
                     (ushort)(SUB161(auVar22 >> 0x3f,0) & 1) << 7 |
                     (ushort)(SUB161(auVar22 >> 0x47,0) & 1) << 8 |
                     (ushort)(SUB161(auVar22 >> 0x4f,0) & 1) << 9 |
                     (ushort)(SUB161(auVar22 >> 0x57,0) & 1) << 10 |
                     (ushort)(SUB161(auVar22 >> 0x5f,0) & 1) << 0xb |
                     (ushort)(SUB161(auVar22 >> 0x67,0) & 1) << 0xc |
                     (ushort)(SUB161(auVar22 >> 0x6f,0) & 1) << 0xd |
                     (ushort)(SUB161(auVar22 >> 0x77,0) & 1) << 0xe |
                     (ushort)(byte)(auVar22[0xf] >> 7) << 0xf;
            pauVar16 = pauVar16 + -0x2a8;
            pauVar17 = pauVar17 + 1;
          } while (uVar14 == 0xffff);
          uVar10 = ~(uint)uVar14;
          local_e0 = uVar10 & -(uint)uVar14 - 2;
        }
        else {
          uVar10 = local_e0;
          local_e0 = local_e0 - 1 & local_e0;
        }
        uVar4 = 0;
        for (; (uVar10 & 1) == 0; uVar10 = uVar10 >> 1 | 0x80000000) {
          uVar4 = uVar4 + 1;
        }
        lVar3 = *(longlong *)((longlong)pauVar16 + (ulonglong)uVar4 * -0x2a8 + -0x2a8);
        if (plVar2[3] == 0) {
LAB_140a8b504:
                    // WARNING: Subroutine does not return
          core::option::unwrap_failed();
        }
        lVar18 = lVar18 + -1;
        core::hash::BuildHasher::
        hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>();
        lVar11 = *plVar2;
        bVar20 = (byte)(extraout_RAX >> 0x39);
        auVar22 = ZEXT216(CONCAT11(bVar20,bVar20));
        auVar22 = pshuflw(auVar22,auVar22,0);
        lVar13 = 0;
        uVar12 = extraout_RAX;
LAB_140a8b1de:
        uVar12 = plVar2[1] & uVar12;
        pcVar1 = (char *)(lVar11 + uVar12);
        cVar21 = auVar22[0];
        auVar27[0] = -(*pcVar1 == cVar21);
        cVar23 = auVar22[1];
        auVar27[1] = -(pcVar1[1] == cVar23);
        cVar24 = auVar22[2];
        auVar27[2] = -(pcVar1[2] == cVar24);
        cVar25 = auVar22[3];
        auVar27[3] = -(pcVar1[3] == cVar25);
        auVar27[4] = -(pcVar1[4] == cVar21);
        auVar27[5] = -(pcVar1[5] == cVar23);
        auVar27[6] = -(pcVar1[6] == cVar24);
        auVar27[7] = -(pcVar1[7] == cVar25);
        auVar27[8] = -(pcVar1[8] == cVar21);
        auVar27[9] = -(pcVar1[9] == cVar23);
        auVar27[10] = -(pcVar1[10] == cVar24);
        auVar27[0xb] = -(pcVar1[0xb] == cVar25);
        auVar27[0xc] = -(pcVar1[0xc] == cVar21);
        auVar27[0xd] = -(pcVar1[0xd] == cVar23);
        auVar27[0xe] = -(pcVar1[0xe] == cVar24);
        auVar27[0xf] = -(pcVar1[0xf] == cVar25);
        uVar14 = (ushort)(SUB161(auVar27 >> 7,0) & 1) | (ushort)(SUB161(auVar27 >> 0xf,0) & 1) << 1
                 | (ushort)(SUB161(auVar27 >> 0x17,0) & 1) << 2 |
                 (ushort)(SUB161(auVar27 >> 0x1f,0) & 1) << 3 |
                 (ushort)(SUB161(auVar27 >> 0x27,0) & 1) << 4 |
                 (ushort)(SUB161(auVar27 >> 0x2f,0) & 1) << 5 |
                 (ushort)(SUB161(auVar27 >> 0x37,0) & 1) << 6 |
                 (ushort)(SUB161(auVar27 >> 0x3f,0) & 1) << 7 |
                 (ushort)(SUB161(auVar27 >> 0x47,0) & 1) << 8 |
                 (ushort)(SUB161(auVar27 >> 0x4f,0) & 1) << 9 |
                 (ushort)(SUB161(auVar27 >> 0x57,0) & 1) << 10 |
                 (ushort)(SUB161(auVar27 >> 0x5f,0) & 1) << 0xb |
                 (ushort)(SUB161(auVar27 >> 0x67,0) & 1) << 0xc |
                 (ushort)(SUB161(auVar27 >> 0x6f,0) & 1) << 0xd |
                 (ushort)(SUB161(auVar27 >> 0x77,0) & 1) << 0xe | (ushort)(auVar27[0xf] >> 7) << 0xf
        ;
        uVar10 = (uint)uVar14;
        while( true ) {
          if (uVar14 == 0) goto LAB_140a8b220;
          uVar4 = 0;
          for (uVar5 = uVar10; (uVar5 & 1) == 0; uVar5 = uVar5 >> 1 | 0x80000000) {
            uVar4 = uVar4 + 1;
          }
          lVar15 = (uVar4 + uVar12 & plVar2[1]) * -0x2a8;
          if (*(longlong *)(lVar11 + -0x2a8 + lVar15) == lVar3) break;
          uVar14 = (ushort)(uVar10 - 1) & (ushort)uVar10;
          uVar10 = CONCAT22((short)(uVar10 - 1 >> 0x10),uVar14);
        }
        if ((*(ulonglong *)(lVar11 + -0x290 + lVar15) < 2) ||
           (*(float *)(lVar11 + -0x270 + lVar15) <= 0.0)) {
                    // WARNING: Subroutine does not return
          core::panicking::panic();
        }
        lVar11 = lVar11 + lVar15;
        uVar12 = *(ulonglong *)(lVar11 + -0x208);
        if (*(ulonglong *)(lVar11 + -0x1d0) < *(ulonglong *)(lVar11 + -0x208)) {
          uVar12 = *(ulonglong *)(lVar11 + -0x1d0);
        }
        if (*(ulonglong *)(lVar11 + -0x1b8) < uVar12) {
          uVar12 = *(ulonglong *)(lVar11 + -0x1b8);
        }
        if (*(ulonglong *)(lVar11 + -0x1a0) < uVar12) {
          uVar12 = *(ulonglong *)(lVar11 + -0x1a0);
        }
        if (uVar12 != 0) {
          uVar19 = 0;
          do {
            uVar19 = uVar19 + 1;
            country_core::utils::debug_viz::DebugSphere::show();
            country_core::utils::debug_viz::DebugSphere::show();
            country_core::utils::debug_viz::DebugSphere::show();
            country_core::utils::debug_viz::DebugSphere::show();
          } while (uVar12 != uVar19);
        }
        if (lVar18 == 0) {
          return;
        }
      } while( true );
    }
  }
  return;
LAB_140a8b220:
  auVar26[0] = -(*pcVar1 == (char)iVar6);
  auVar26[1] = -(pcVar1[1] == (char)((uint)iVar6 >> 8));
  auVar26[2] = -(pcVar1[2] == (char)((uint)iVar6 >> 0x10));
  auVar26[3] = -(pcVar1[3] == (char)((uint)iVar6 >> 0x18));
  auVar26[4] = -(pcVar1[4] == (char)iVar7);
  auVar26[5] = -(pcVar1[5] == (char)((uint)iVar7 >> 8));
  auVar26[6] = -(pcVar1[6] == (char)((uint)iVar7 >> 0x10));
  auVar26[7] = -(pcVar1[7] == (char)((uint)iVar7 >> 0x18));
  auVar26[8] = -(pcVar1[8] == (char)iVar8);
  auVar26[9] = -(pcVar1[9] == (char)((uint)iVar8 >> 8));
  auVar26[10] = -(pcVar1[10] == (char)((uint)iVar8 >> 0x10));
  auVar26[0xb] = -(pcVar1[0xb] == (char)((uint)iVar8 >> 0x18));
  auVar26[0xc] = -(pcVar1[0xc] == (char)iVar9);
  auVar26[0xd] = -(pcVar1[0xd] == (char)((uint)iVar9 >> 8));
  auVar26[0xe] = -(pcVar1[0xe] == (char)((uint)iVar9 >> 0x10));
  auVar26[0xf] = -(pcVar1[0xf] == (char)((uint)iVar9 >> 0x18));
  if ((((((((((((((((SUB161(auVar26 >> 7,0) & 1) != 0 || (SUB161(auVar26 >> 0xf,0) & 1) != 0) ||
                  (SUB161(auVar26 >> 0x17,0) & 1) != 0) || (SUB161(auVar26 >> 0x1f,0) & 1) != 0) ||
                (SUB161(auVar26 >> 0x27,0) & 1) != 0) || (SUB161(auVar26 >> 0x2f,0) & 1) != 0) ||
              (SUB161(auVar26 >> 0x37,0) & 1) != 0) || (SUB161(auVar26 >> 0x3f,0) & 1) != 0) ||
            (SUB161(auVar26 >> 0x47,0) & 1) != 0) || (SUB161(auVar26 >> 0x4f,0) & 1) != 0) ||
          (SUB161(auVar26 >> 0x57,0) & 1) != 0) || (SUB161(auVar26 >> 0x5f,0) & 1) != 0) ||
        (SUB161(auVar26 >> 0x67,0) & 1) != 0) || (SUB161(auVar26 >> 0x6f,0) & 1) != 0) ||
      (SUB161(auVar26 >> 0x77,0) & 1) != 0) || auVar26[0xf] < '\0') goto LAB_140a8b504;
  uVar12 = uVar12 + lVar13 + 0x10;
  lVar13 = lVar13 + 0x10;
  goto LAB_140a8b1de;
}



// CALLEES: [{'name': 'hash_one<std::hash::random::RandomState,ref$<country_core::systems::ui::GlobalId>_>', 'address': '1405b0d70'}, {'name': 'panic', 'address': '1428d9430'}, {'name': 'show', 'address': '140b02520'}, {'name': 'unwrap_failed', 'address': '1428d9310'}]