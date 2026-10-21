
void __cdecl
system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble::closure_24<false>
          (void)

{
  longlong *plVar1;
  longlong lVar2;
  longlong *plVar3;
  longlong lVar4;
  uint uVar5;
  uint uVar6;
  int iVar7;
  int iVar8;
  int iVar9;
  int iVar10;
  ushort uVar11;
  longlong extraout_RAX;
  ulonglong extraout_RAX_00;
  longlong extraout_RAX_01;
  ulonglong extraout_RAX_02;
  char *pcVar12;
  ulonglong extraout_RAX_03;
  longlong extraout_RAX_04;
  longlong extraout_RAX_05;
  ulonglong extraout_RAX_06;
  longlong in_RCX;
  int in_EDX;
  int in_R8D;
  ulonglong uVar13;
  char in_R9B;
  longlong lVar14;
  uint uVar15;
  ulonglong uVar16;
  longlong lVar17;
  char cVar18;
  byte bVar19;
  char cVar21;
  char cVar22;
  char cVar23;
  undefined1 auVar20 [16];
  undefined1 auVar24 [16];
  undefined1 auVar25 [16];
  undefined1 auVar26 [16];
  undefined1 auVar27 [16];
  undefined1 unaff_XMM6_Ba;
  undefined1 unaff_XMM6_Bb;
  char cVar28;
  undefined1 unaff_XMM6_Bc;
  char cVar29;
  undefined1 unaff_XMM6_Bd;
  char cVar30;
  undefined1 unaff_XMM6_Be;
  undefined1 unaff_XMM6_Bf;
  char cVar31;
  undefined1 unaff_XMM6_Bg;
  char cVar32;
  undefined1 unaff_XMM6_Bh;
  char cVar33;
  undefined1 unaff_XMM6_Bi;
  undefined1 unaff_XMM6_Bj;
  char cVar34;
  undefined1 unaff_XMM6_Bk;
  char cVar35;
  undefined1 unaff_XMM6_Bl;
  char cVar36;
  undefined1 unaff_XMM6_Bm;
  undefined1 unaff_XMM6_Bn;
  char cVar37;
  undefined1 unaff_XMM6_Bo;
  char cVar38;
  undefined1 unaff_XMM6_Bp;
  char cVar39;
  int local_b4;
  longlong local_b0;
  
  country_core::resources::stairs::stairs_state::StairsState::graph();
  uVar13 = *(ulonglong *)(extraout_RAX + 0x10);
  if ((uVar13 != 0) && (uVar13 != 1)) {
    indexmap::map::IndexMap::
    hash<country_core::resources::walls::decorator::DecoratorId,alloc::vec::Vec<tuple_<country_core::resources::walls::decorator::DecoratorId,petgraph::graphmap::CompactDirection>,alloc::alloc::Global>,std::hash::random::RandomState,country_core::resources::walls::decorator::DecoratorId>
              ();
    uVar16 = extraout_RAX_00;
    indexmap::map::core::IndexMapCore::
    get_index_of<country_core::resources::walls::decorator::DecoratorId,alloc::vec::Vec<tuple_<country_core::resources::walls::decorator::DecoratorId,petgraph::graphmap::CompactDirection>,alloc::alloc::Global>,country_core::resources::walls::decorator::DecoratorId>
              ();
    if ((extraout_RAX_01 == 1) && (uVar13 <= uVar16)) {
                    // WARNING: Subroutine does not return
      core::panicking::panic_bounds_check();
    }
  }
  plVar1 = *(longlong **)(in_RCX + 8);
  lVar2 = *(longlong *)(in_RCX + 0x10);
  plVar3 = *(longlong **)(in_RCX + 0x18);
  petgraph::graphmap::impl$11::
  next<country_core::resources::walls::decorator::DecoratorId,tuple_<>,enum2_<petgraph::Undirected>,std::hash::random::RandomState>
            ();
  if (local_b0 == 0) {
    return;
  }
  iVar7 = -(uint)(CONCAT13(unaff_XMM6_Bd,
                           CONCAT12(unaff_XMM6_Bc,CONCAT11(unaff_XMM6_Bb,unaff_XMM6_Ba))) ==
                 CONCAT13(unaff_XMM6_Bd,
                          CONCAT12(unaff_XMM6_Bc,CONCAT11(unaff_XMM6_Bb,unaff_XMM6_Ba))));
  cVar28 = (char)((uint)iVar7 >> 8);
  cVar29 = (char)((uint)iVar7 >> 0x10);
  cVar30 = (char)((uint)iVar7 >> 0x18);
  iVar8 = -(uint)(CONCAT13(unaff_XMM6_Bh,
                           CONCAT12(unaff_XMM6_Bg,CONCAT11(unaff_XMM6_Bf,unaff_XMM6_Be))) ==
                 CONCAT13(unaff_XMM6_Bh,
                          CONCAT12(unaff_XMM6_Bg,CONCAT11(unaff_XMM6_Bf,unaff_XMM6_Be))));
  cVar31 = (char)((uint)iVar8 >> 8);
  cVar32 = (char)((uint)iVar8 >> 0x10);
  cVar33 = (char)((uint)iVar8 >> 0x18);
  iVar9 = -(uint)(CONCAT13(unaff_XMM6_Bl,
                           CONCAT12(unaff_XMM6_Bk,CONCAT11(unaff_XMM6_Bj,unaff_XMM6_Bi))) ==
                 CONCAT13(unaff_XMM6_Bl,
                          CONCAT12(unaff_XMM6_Bk,CONCAT11(unaff_XMM6_Bj,unaff_XMM6_Bi))));
  cVar34 = (char)((uint)iVar9 >> 8);
  cVar35 = (char)((uint)iVar9 >> 0x10);
  cVar36 = (char)((uint)iVar9 >> 0x18);
  iVar10 = -(uint)(CONCAT13(unaff_XMM6_Bp,
                            CONCAT12(unaff_XMM6_Bo,CONCAT11(unaff_XMM6_Bn,unaff_XMM6_Bm))) ==
                  CONCAT13(unaff_XMM6_Bp,
                           CONCAT12(unaff_XMM6_Bo,CONCAT11(unaff_XMM6_Bn,unaff_XMM6_Bm))));
  cVar37 = (char)((uint)iVar10 >> 8);
  cVar38 = (char)((uint)iVar10 >> 0x10);
  cVar39 = (char)((uint)iVar10 >> 0x18);
LAB_140028a80:
  if (local_b4 != in_R8D) {
    country_core::resources::stairs::stairs_state::StairSegment::new();
    country_core::resources::stairs::stairs_assembly_params::impl$2::from();
    if (plVar1[3] != 0) {
      core::hash::BuildHasher::
      hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>();
      lVar4 = *plVar1;
      bVar19 = (byte)(extraout_RAX_06 >> 0x39);
      auVar20 = ZEXT216(CONCAT11(bVar19,bVar19));
      auVar20 = pshuflw(auVar20,auVar20,0);
      lVar14 = 0;
      uVar13 = extraout_RAX_06;
      while( true ) {
        uVar13 = plVar1[1] & uVar13;
        pcVar12 = (char *)(lVar4 + uVar13);
        cVar18 = auVar20[0];
        auVar26[0] = -(*pcVar12 == cVar18);
        cVar21 = auVar20[1];
        auVar26[1] = -(pcVar12[1] == cVar21);
        cVar22 = auVar20[2];
        auVar26[2] = -(pcVar12[2] == cVar22);
        cVar23 = auVar20[3];
        auVar26[3] = -(pcVar12[3] == cVar23);
        auVar26[4] = -(pcVar12[4] == cVar18);
        auVar26[5] = -(pcVar12[5] == cVar21);
        auVar26[6] = -(pcVar12[6] == cVar22);
        auVar26[7] = -(pcVar12[7] == cVar23);
        auVar26[8] = -(pcVar12[8] == cVar18);
        auVar26[9] = -(pcVar12[9] == cVar21);
        auVar26[10] = -(pcVar12[10] == cVar22);
        auVar26[0xb] = -(pcVar12[0xb] == cVar23);
        auVar26[0xc] = -(pcVar12[0xc] == cVar18);
        auVar26[0xd] = -(pcVar12[0xd] == cVar21);
        auVar26[0xe] = -(pcVar12[0xe] == cVar22);
        auVar26[0xf] = -(pcVar12[0xf] == cVar23);
        uVar11 = (ushort)(SUB161(auVar26 >> 7,0) & 1) | (ushort)(SUB161(auVar26 >> 0xf,0) & 1) << 1
                 | (ushort)(SUB161(auVar26 >> 0x17,0) & 1) << 2 |
                 (ushort)(SUB161(auVar26 >> 0x1f,0) & 1) << 3 |
                 (ushort)(SUB161(auVar26 >> 0x27,0) & 1) << 4 |
                 (ushort)(SUB161(auVar26 >> 0x2f,0) & 1) << 5 |
                 (ushort)(SUB161(auVar26 >> 0x37,0) & 1) << 6 |
                 (ushort)(SUB161(auVar26 >> 0x3f,0) & 1) << 7 |
                 (ushort)(SUB161(auVar26 >> 0x47,0) & 1) << 8 |
                 (ushort)(SUB161(auVar26 >> 0x4f,0) & 1) << 9 |
                 (ushort)(SUB161(auVar26 >> 0x57,0) & 1) << 10 |
                 (ushort)(SUB161(auVar26 >> 0x5f,0) & 1) << 0xb |
                 (ushort)(SUB161(auVar26 >> 0x67,0) & 1) << 0xc |
                 (ushort)(SUB161(auVar26 >> 0x6f,0) & 1) << 0xd |
                 (ushort)(SUB161(auVar26 >> 0x77,0) & 1) << 0xe | (ushort)(auVar26[0xf] >> 7) << 0xf
        ;
        uVar15 = (uint)uVar11;
        while (uVar11 != 0) {
          uVar5 = 0;
          for (uVar6 = uVar15; (uVar6 & 1) == 0; uVar6 = uVar6 >> 1 | 0x80000000) {
            uVar5 = uVar5 + 1;
          }
          uVar16 = uVar5 + uVar13 & plVar1[1];
          if (extraout_RAX_05 == *(longlong *)(lVar4 + -0x10 + uVar16 * -0x10)) {
            if (*(char *)(lVar4 + -8 + uVar16 * -0x10) != '\x01') goto LAB_140028b0e;
            if (plVar3[3] == 0) goto LAB_1400289c9;
            core::hash::BuildHasher::
            hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>();
            lVar4 = *plVar3;
            bVar19 = (byte)(extraout_RAX_02 >> 0x39);
            auVar20 = ZEXT216(CONCAT11(bVar19,bVar19));
            auVar20 = pshuflw(auVar20,auVar20,0);
            lVar14 = 0;
            uVar13 = extraout_RAX_02;
            goto LAB_1400288f4;
          }
          uVar11 = (ushort)(uVar15 - 1) & (ushort)uVar15;
          uVar15 = CONCAT22((short)(uVar15 - 1 >> 0x10),uVar11);
        }
        auVar24[0] = -(*pcVar12 == (char)iVar7);
        auVar24[1] = -(pcVar12[1] == cVar28);
        auVar24[2] = -(pcVar12[2] == cVar29);
        auVar24[3] = -(pcVar12[3] == cVar30);
        auVar24[4] = -(pcVar12[4] == (char)iVar8);
        auVar24[5] = -(pcVar12[5] == cVar31);
        auVar24[6] = -(pcVar12[6] == cVar32);
        auVar24[7] = -(pcVar12[7] == cVar33);
        auVar24[8] = -(pcVar12[8] == (char)iVar9);
        auVar24[9] = -(pcVar12[9] == cVar34);
        auVar24[10] = -(pcVar12[10] == cVar35);
        auVar24[0xb] = -(pcVar12[0xb] == cVar36);
        auVar24[0xc] = -(pcVar12[0xc] == (char)iVar10);
        auVar24[0xd] = -(pcVar12[0xd] == cVar37);
        auVar24[0xe] = -(pcVar12[0xe] == cVar38);
        auVar24[0xf] = -(pcVar12[0xf] == cVar39);
        if ((((((((((((((((SUB161(auVar24 >> 7,0) & 1) != 0 || (SUB161(auVar24 >> 0xf,0) & 1) != 0)
                        || (SUB161(auVar24 >> 0x17,0) & 1) != 0) ||
                       (SUB161(auVar24 >> 0x1f,0) & 1) != 0) || (SUB161(auVar24 >> 0x27,0) & 1) != 0
                      ) || (SUB161(auVar24 >> 0x2f,0) & 1) != 0) ||
                    (SUB161(auVar24 >> 0x37,0) & 1) != 0) || (SUB161(auVar24 >> 0x3f,0) & 1) != 0)
                  || (SUB161(auVar24 >> 0x47,0) & 1) != 0) || (SUB161(auVar24 >> 0x4f,0) & 1) != 0)
                || (SUB161(auVar24 >> 0x57,0) & 1) != 0) || (SUB161(auVar24 >> 0x5f,0) & 1) != 0) ||
              (SUB161(auVar24 >> 0x67,0) & 1) != 0) || (SUB161(auVar24 >> 0x6f,0) & 1) != 0) ||
            (SUB161(auVar24 >> 0x77,0) & 1) != 0) || auVar24[0xf] < '\0') break;
        uVar13 = uVar13 + lVar14 + 0x10;
        lVar14 = lVar14 + 0x10;
      }
    }
  }
  goto LAB_140028b0e;
LAB_1400288f4:
  uVar13 = plVar3[1] & uVar13;
  pcVar12 = (char *)(lVar4 + uVar13);
  cVar18 = auVar20[0];
  auVar27[0] = -(*pcVar12 == cVar18);
  cVar21 = auVar20[1];
  auVar27[1] = -(pcVar12[1] == cVar21);
  cVar22 = auVar20[2];
  auVar27[2] = -(pcVar12[2] == cVar22);
  cVar23 = auVar20[3];
  auVar27[3] = -(pcVar12[3] == cVar23);
  auVar27[4] = -(pcVar12[4] == cVar18);
  auVar27[5] = -(pcVar12[5] == cVar21);
  auVar27[6] = -(pcVar12[6] == cVar22);
  auVar27[7] = -(pcVar12[7] == cVar23);
  auVar27[8] = -(pcVar12[8] == cVar18);
  auVar27[9] = -(pcVar12[9] == cVar21);
  auVar27[10] = -(pcVar12[10] == cVar22);
  auVar27[0xb] = -(pcVar12[0xb] == cVar23);
  auVar27[0xc] = -(pcVar12[0xc] == cVar18);
  auVar27[0xd] = -(pcVar12[0xd] == cVar21);
  auVar27[0xe] = -(pcVar12[0xe] == cVar22);
  auVar27[0xf] = -(pcVar12[0xf] == cVar23);
  uVar11 = (ushort)(SUB161(auVar27 >> 7,0) & 1) | (ushort)(SUB161(auVar27 >> 0xf,0) & 1) << 1 |
           (ushort)(SUB161(auVar27 >> 0x17,0) & 1) << 2 |
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
           (ushort)(SUB161(auVar27 >> 0x77,0) & 1) << 0xe | (ushort)(auVar27[0xf] >> 7) << 0xf;
  uVar15 = (uint)uVar11;
  while (uVar11 != 0) {
    uVar5 = 0;
    for (uVar6 = uVar15; (uVar6 & 1) == 0; uVar6 = uVar6 >> 1 | 0x80000000) {
      uVar5 = uVar5 + 1;
    }
    uVar16 = uVar5 + uVar13 & plVar3[1];
    if (extraout_RAX_05 == *(longlong *)(lVar4 + -0x20 + uVar16 * -0x20)) {
      lVar17 = uVar16 * -0x20;
      lVar14 = lVar4 + lVar17;
      pcVar12 = (char *)(lVar4 + lVar17 + -0x18);
      uVar13 = *(ulonglong *)(lVar4 + -8 + lVar17);
      if (in_R9B == '\0') {
        uVar16 = uVar13;
        if (1 < uVar13) {
          uVar16 = *(ulonglong *)(lVar14 + -0x10);
        }
        if (uVar16 == 0) goto LAB_1400289c9;
        if (1 < uVar13) {
          pcVar12 = *(char **)pcVar12;
        }
      }
      else {
        if (1 < uVar13) {
          pcVar12 = *(char **)(lVar14 + -0x18);
          uVar13 = *(ulonglong *)(lVar14 + -0x10);
        }
        if ((uVar13 == 0) || (pcVar12 = pcVar12 + uVar13 * 4 + -4, pcVar12 == (char *)0x0))
        goto LAB_1400289c9;
      }
      if (*pcVar12 == '\x03') {
        return;
      }
      goto LAB_1400289c9;
    }
    uVar11 = (ushort)(uVar15 - 1) & (ushort)uVar15;
    uVar15 = CONCAT22((short)(uVar15 - 1 >> 0x10),uVar11);
  }
  auVar25[0] = -(*pcVar12 == (char)iVar7);
  auVar25[1] = -(pcVar12[1] == cVar28);
  auVar25[2] = -(pcVar12[2] == cVar29);
  auVar25[3] = -(pcVar12[3] == cVar30);
  auVar25[4] = -(pcVar12[4] == (char)iVar8);
  auVar25[5] = -(pcVar12[5] == cVar31);
  auVar25[6] = -(pcVar12[6] == cVar32);
  auVar25[7] = -(pcVar12[7] == cVar33);
  auVar25[8] = -(pcVar12[8] == (char)iVar9);
  auVar25[9] = -(pcVar12[9] == cVar34);
  auVar25[10] = -(pcVar12[10] == cVar35);
  auVar25[0xb] = -(pcVar12[0xb] == cVar36);
  auVar25[0xc] = -(pcVar12[0xc] == (char)iVar10);
  auVar25[0xd] = -(pcVar12[0xd] == cVar37);
  auVar25[0xe] = -(pcVar12[0xe] == cVar38);
  auVar25[0xf] = -(pcVar12[0xf] == cVar39);
  if ((((((((((((((((SUB161(auVar25 >> 7,0) & 1) != 0 || (SUB161(auVar25 >> 0xf,0) & 1) != 0) ||
                  (SUB161(auVar25 >> 0x17,0) & 1) != 0) || (SUB161(auVar25 >> 0x1f,0) & 1) != 0) ||
                (SUB161(auVar25 >> 0x27,0) & 1) != 0) || (SUB161(auVar25 >> 0x2f,0) & 1) != 0) ||
              (SUB161(auVar25 >> 0x37,0) & 1) != 0) || (SUB161(auVar25 >> 0x3f,0) & 1) != 0) ||
            (SUB161(auVar25 >> 0x47,0) & 1) != 0) || (SUB161(auVar25 >> 0x4f,0) & 1) != 0) ||
          (SUB161(auVar25 >> 0x57,0) & 1) != 0) || (SUB161(auVar25 >> 0x5f,0) & 1) != 0) ||
        (SUB161(auVar25 >> 0x67,0) & 1) != 0) || (SUB161(auVar25 >> 0x6f,0) & 1) != 0) ||
      (SUB161(auVar25 >> 0x77,0) & 1) != 0) || auVar25[0xf] < '\0') goto LAB_1400289c9;
  uVar13 = uVar13 + lVar14 + 0x10;
  lVar14 = lVar14 + 0x10;
  goto LAB_1400288f4;
LAB_1400289c9:
  uVar13 = *(ulonglong *)(lVar2 + 0x58);
  if (uVar13 == 1) {
    lVar4 = *(longlong *)(lVar2 + 0x50);
    if (*(int *)(lVar4 + 8) != in_EDX) {
      return;
    }
    if (*(int *)(lVar4 + 0xc) != local_b4) {
      return;
    }
    cVar18 = *(char *)(lVar4 + 0x10);
  }
  else {
    if (uVar13 == 0) {
      return;
    }
    indexmap::map::IndexMap::
    hash<tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId>,country_core::resources::stairs::stairs_assembly_params::AdjustedStairEdge,std::hash::random::RandomState,tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId>_>
              ();
    uVar16 = extraout_RAX_03;
    indexmap::map::core::IndexMapCore::
    get_index_of<tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId>,enum2_<country_core::resources::stairs::stairs_assembly_params::AlignKind>,tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId>_>
              ();
    if (extraout_RAX_04 != 1) {
      return;
    }
    if (uVar13 <= uVar16) {
                    // WARNING: Subroutine does not return
      core::panicking::panic_bounds_check();
    }
    cVar18 = *(char *)(*(longlong *)(lVar2 + 0x50) + 0x10 + uVar16 * 0x28);
  }
  if (cVar18 != '\0') {
    return;
  }
LAB_140028b0e:
  petgraph::graphmap::impl$11::
  next<country_core::resources::walls::decorator::DecoratorId,tuple_<>,enum2_<petgraph::Undirected>,std::hash::random::RandomState>
            ();
  if (local_b0 == 0) {
    return;
  }
  goto LAB_140028a80;
}

