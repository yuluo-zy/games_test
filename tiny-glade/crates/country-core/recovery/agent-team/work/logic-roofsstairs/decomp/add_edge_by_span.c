
void __cdecl
country_core::resources::stairs::structural_graph::StairsStructuralGraph::add_edge_by_span(void)

{
  char *pcVar1;
  undefined8 uVar2;
  longlong lVar3;
  uint uVar4;
  uint uVar5;
  byte extraout_AL;
  ulonglong extraout_RAX;
  ulonglong extraout_RAX_00;
  undefined1 *extraout_RAX_01;
  longlong in_RCX;
  undefined8 *in_RDX;
  ushort uVar6;
  uint unaff_EBP;
  undefined8 *in_R8;
  ulonglong uVar7;
  longlong lVar8;
  uint uVar9;
  ulonglong uVar10;
  byte bVar11;
  char cVar12;
  char cVar14;
  char cVar15;
  char cVar16;
  undefined1 auVar13 [16];
  undefined1 auVar17 [16];
  undefined1 auVar18 [16];
  int iVar23;
  undefined1 auVar19 [16];
  undefined1 auVar20 [16];
  undefined1 auVar21 [16];
  int iVar24;
  undefined1 auVar22 [16];
  
  if (*(longlong *)(in_RCX + 0xd8) != 0) {
    core::hash::BuildHasher::
    hash_one<std::hash::random::RandomState,ref_<tuple_<country_core::resources::stairs::stairs_state::StairGraphEdge,u32>_>_>
              ();
    lVar3 = *(longlong *)(in_RCX + 0xc0);
    bVar11 = (byte)(extraout_RAX >> 0x39);
    auVar13 = ZEXT216(CONCAT11(bVar11,bVar11));
    auVar13 = pshuflw(auVar13,auVar13,0);
    lVar8 = 0;
    uVar7 = extraout_RAX;
    while( true ) {
      uVar7 = *(ulonglong *)(in_RCX + 200) & uVar7;
      pcVar1 = (char *)(lVar3 + uVar7);
      cVar12 = auVar13[0];
      auVar19[0] = -(*pcVar1 == cVar12);
      cVar14 = auVar13[1];
      auVar19[1] = -(pcVar1[1] == cVar14);
      cVar15 = auVar13[2];
      auVar19[2] = -(pcVar1[2] == cVar15);
      cVar16 = auVar13[3];
      auVar19[3] = -(pcVar1[3] == cVar16);
      auVar19[4] = -(pcVar1[4] == cVar12);
      auVar19[5] = -(pcVar1[5] == cVar14);
      auVar19[6] = -(pcVar1[6] == cVar15);
      auVar19[7] = -(pcVar1[7] == cVar16);
      auVar19[8] = -(pcVar1[8] == cVar12);
      auVar19[9] = -(pcVar1[9] == cVar14);
      auVar19[10] = -(pcVar1[10] == cVar15);
      auVar19[0xb] = -(pcVar1[0xb] == cVar16);
      auVar19[0xc] = -(pcVar1[0xc] == cVar12);
      auVar19[0xd] = -(pcVar1[0xd] == cVar14);
      auVar19[0xe] = -(pcVar1[0xe] == cVar15);
      auVar19[0xf] = -(pcVar1[0xf] == cVar16);
      uVar6 = (ushort)(SUB161(auVar19 >> 7,0) & 1) | (ushort)(SUB161(auVar19 >> 0xf,0) & 1) << 1 |
              (ushort)(SUB161(auVar19 >> 0x17,0) & 1) << 2 |
              (ushort)(SUB161(auVar19 >> 0x1f,0) & 1) << 3 |
              (ushort)(SUB161(auVar19 >> 0x27,0) & 1) << 4 |
              (ushort)(SUB161(auVar19 >> 0x2f,0) & 1) << 5 |
              (ushort)(SUB161(auVar19 >> 0x37,0) & 1) << 6 |
              (ushort)(SUB161(auVar19 >> 0x3f,0) & 1) << 7 |
              (ushort)(SUB161(auVar19 >> 0x47,0) & 1) << 8 |
              (ushort)(SUB161(auVar19 >> 0x4f,0) & 1) << 9 |
              (ushort)(SUB161(auVar19 >> 0x57,0) & 1) << 10 |
              (ushort)(SUB161(auVar19 >> 0x5f,0) & 1) << 0xb |
              (ushort)(SUB161(auVar19 >> 0x67,0) & 1) << 0xc |
              (ushort)(SUB161(auVar19 >> 0x6f,0) & 1) << 0xd |
              (ushort)(SUB161(auVar19 >> 0x77,0) & 1) << 0xe | (ushort)(auVar19[0xf] >> 7) << 0xf;
      uVar9 = (uint)uVar6;
      while (uVar6 != 0) {
        uVar4 = 0;
        for (uVar5 = uVar9; (uVar5 & 1) == 0; uVar5 = uVar5 >> 1 | 0x80000000) {
          uVar4 = uVar4 + 1;
        }
        uVar10 = uVar4 + uVar7 & *(ulonglong *)(in_RCX + 200);
        uVar2 = *(undefined8 *)(lVar3 + -0x18 + uVar10 * -0x18);
        auVar20._0_4_ = -(uint)((int)uVar2 == (int)*in_RDX);
        iVar23 = -(uint)((int)((ulonglong)uVar2 >> 0x20) == (int)((ulonglong)*in_RDX >> 0x20));
        auVar20._4_4_ = auVar20._0_4_;
        auVar20._8_4_ = iVar23;
        auVar20._12_4_ = iVar23;
        iVar23 = movmskpd(unaff_EBP,auVar20);
        if ((iVar23 == 3) && (*(int *)(in_RDX + 1) == *(int *)(lVar3 + -0x18 + uVar10 * -0x18 + 8)))
        {
          if (*(char *)(lVar3 + -4 + uVar10 * -0x18) == '\x11') {
            return;
          }
          core::hash::BuildHasher::
          hash_one<std::hash::random::RandomState,ref_<tuple_<country_core::resources::stairs::stairs_state::StairGraphEdge,u32>_>_>
                    ();
          lVar3 = *(longlong *)(in_RCX + 0xc0);
          bVar11 = (byte)(extraout_RAX_00 >> 0x39);
          auVar13 = ZEXT216(CONCAT11(bVar11,bVar11));
          auVar13 = pshuflw(auVar13,auVar13,0);
          lVar8 = 0;
          iVar23 = 3;
          uVar7 = extraout_RAX_00;
          do {
            uVar7 = *(ulonglong *)(in_RCX + 200) & uVar7;
            pcVar1 = (char *)(lVar3 + uVar7);
            cVar12 = auVar13[0];
            auVar21[0] = -(*pcVar1 == cVar12);
            cVar14 = auVar13[1];
            auVar21[1] = -(pcVar1[1] == cVar14);
            cVar15 = auVar13[2];
            auVar21[2] = -(pcVar1[2] == cVar15);
            cVar16 = auVar13[3];
            auVar21[3] = -(pcVar1[3] == cVar16);
            auVar21[4] = -(pcVar1[4] == cVar12);
            auVar21[5] = -(pcVar1[5] == cVar14);
            auVar21[6] = -(pcVar1[6] == cVar15);
            auVar21[7] = -(pcVar1[7] == cVar16);
            auVar21[8] = -(pcVar1[8] == cVar12);
            auVar21[9] = -(pcVar1[9] == cVar14);
            auVar21[10] = -(pcVar1[10] == cVar15);
            auVar21[0xb] = -(pcVar1[0xb] == cVar16);
            auVar21[0xc] = -(pcVar1[0xc] == cVar12);
            auVar21[0xd] = -(pcVar1[0xd] == cVar14);
            auVar21[0xe] = -(pcVar1[0xe] == cVar15);
            auVar21[0xf] = -(pcVar1[0xf] == cVar16);
            uVar6 = (ushort)(SUB161(auVar21 >> 7,0) & 1) |
                    (ushort)(SUB161(auVar21 >> 0xf,0) & 1) << 1 |
                    (ushort)(SUB161(auVar21 >> 0x17,0) & 1) << 2 |
                    (ushort)(SUB161(auVar21 >> 0x1f,0) & 1) << 3 |
                    (ushort)(SUB161(auVar21 >> 0x27,0) & 1) << 4 |
                    (ushort)(SUB161(auVar21 >> 0x2f,0) & 1) << 5 |
                    (ushort)(SUB161(auVar21 >> 0x37,0) & 1) << 6 |
                    (ushort)(SUB161(auVar21 >> 0x3f,0) & 1) << 7 |
                    (ushort)(SUB161(auVar21 >> 0x47,0) & 1) << 8 |
                    (ushort)(SUB161(auVar21 >> 0x4f,0) & 1) << 9 |
                    (ushort)(SUB161(auVar21 >> 0x57,0) & 1) << 10 |
                    (ushort)(SUB161(auVar21 >> 0x5f,0) & 1) << 0xb |
                    (ushort)(SUB161(auVar21 >> 0x67,0) & 1) << 0xc |
                    (ushort)(SUB161(auVar21 >> 0x6f,0) & 1) << 0xd |
                    (ushort)(SUB161(auVar21 >> 0x77,0) & 1) << 0xe |
                    (ushort)(auVar21[0xf] >> 7) << 0xf;
            uVar9 = (uint)uVar6;
            while (uVar6 != 0) {
              uVar4 = 0;
              for (uVar5 = uVar9; (uVar5 & 1) == 0; uVar5 = uVar5 >> 1 | 0x80000000) {
                uVar4 = uVar4 + 1;
              }
              uVar10 = uVar4 + uVar7 & *(ulonglong *)(in_RCX + 200);
              uVar2 = *(undefined8 *)(lVar3 + -0x18 + uVar10 * -0x18);
              auVar22._0_4_ = -(uint)((int)uVar2 == (int)*in_R8);
              iVar24 = -(uint)((int)((ulonglong)uVar2 >> 0x20) == (int)((ulonglong)*in_R8 >> 0x20));
              auVar22._4_4_ = auVar22._0_4_;
              auVar22._8_4_ = iVar24;
              auVar22._12_4_ = iVar24;
              iVar23 = movmskpd(iVar23,auVar22);
              if ((iVar23 == 3) &&
                 (*(int *)(in_R8 + 1) == *(int *)(lVar3 + -0x18 + uVar10 * -0x18 + 8))) {
                if (*(char *)(lVar3 + -4 + uVar10 * -0x18) == '\x11') {
                  return;
                }
                petgraph::graphmap::GraphMap::
                add_edge<country_core::systems::collision::world::SubworldColliderHandle,country_core::resources::stairs::structural_graph::StructuralGraphConnection,enum2_<petgraph::Undirected>,std::hash::random::RandomState>
                          ();
                if ((extraout_AL != 2) == (bool)(extraout_AL & 1)) {
                  return;
                }
                petgraph::graphmap::GraphMap::
                edge_weight_mut<country_core::systems::collision::world::SubworldColliderHandle,country_core::resources::stairs::structural_graph::StructuralGraphConnection,enum2_<petgraph::Undirected>,std::hash::random::RandomState>
                          ();
                if (extraout_RAX_01 != (undefined1 *)0x0) {
                  *extraout_RAX_01 = 0;
                  return;
                }
                    // WARNING: Subroutine does not return
                core::option::unwrap_failed();
              }
              uVar6 = (ushort)(uVar9 - 1) & (ushort)uVar9;
              uVar9 = CONCAT22((short)(uVar9 - 1 >> 0x10),uVar6);
            }
            auVar18[0] = -(*pcVar1 == -1);
            auVar18[1] = -(pcVar1[1] == -1);
            auVar18[2] = -(pcVar1[2] == -1);
            auVar18[3] = -(pcVar1[3] == -1);
            auVar18[4] = -(pcVar1[4] == -1);
            auVar18[5] = -(pcVar1[5] == -1);
            auVar18[6] = -(pcVar1[6] == -1);
            auVar18[7] = -(pcVar1[7] == -1);
            auVar18[8] = -(pcVar1[8] == -1);
            auVar18[9] = -(pcVar1[9] == -1);
            auVar18[10] = -(pcVar1[10] == -1);
            auVar18[0xb] = -(pcVar1[0xb] == -1);
            auVar18[0xc] = -(pcVar1[0xc] == -1);
            auVar18[0xd] = -(pcVar1[0xd] == -1);
            auVar18[0xe] = -(pcVar1[0xe] == -1);
            auVar18[0xf] = -(pcVar1[0xf] == -1);
            if ((((((((((((((((SUB161(auVar18 >> 7,0) & 1) != 0 ||
                             (SUB161(auVar18 >> 0xf,0) & 1) != 0) ||
                            (SUB161(auVar18 >> 0x17,0) & 1) != 0) ||
                           (SUB161(auVar18 >> 0x1f,0) & 1) != 0) ||
                          (SUB161(auVar18 >> 0x27,0) & 1) != 0) ||
                         (SUB161(auVar18 >> 0x2f,0) & 1) != 0) ||
                        (SUB161(auVar18 >> 0x37,0) & 1) != 0) ||
                       (SUB161(auVar18 >> 0x3f,0) & 1) != 0) || (SUB161(auVar18 >> 0x47,0) & 1) != 0
                      ) || (SUB161(auVar18 >> 0x4f,0) & 1) != 0) ||
                    (SUB161(auVar18 >> 0x57,0) & 1) != 0) || (SUB161(auVar18 >> 0x5f,0) & 1) != 0)
                  || (SUB161(auVar18 >> 0x67,0) & 1) != 0) || (SUB161(auVar18 >> 0x6f,0) & 1) != 0)
                || (SUB161(auVar18 >> 0x77,0) & 1) != 0) || auVar18[0xf] < '\0') {
              return;
            }
            uVar7 = uVar7 + lVar8 + 0x10;
            lVar8 = lVar8 + 0x10;
          } while( true );
        }
        uVar6 = (ushort)(uVar9 - 1) & (ushort)uVar9;
        uVar9 = CONCAT22((short)(uVar9 - 1 >> 0x10),uVar6);
        unaff_EBP = uVar9;
      }
      auVar17[0] = -(*pcVar1 == -1);
      auVar17[1] = -(pcVar1[1] == -1);
      auVar17[2] = -(pcVar1[2] == -1);
      auVar17[3] = -(pcVar1[3] == -1);
      auVar17[4] = -(pcVar1[4] == -1);
      auVar17[5] = -(pcVar1[5] == -1);
      auVar17[6] = -(pcVar1[6] == -1);
      auVar17[7] = -(pcVar1[7] == -1);
      auVar17[8] = -(pcVar1[8] == -1);
      auVar17[9] = -(pcVar1[9] == -1);
      auVar17[10] = -(pcVar1[10] == -1);
      auVar17[0xb] = -(pcVar1[0xb] == -1);
      auVar17[0xc] = -(pcVar1[0xc] == -1);
      auVar17[0xd] = -(pcVar1[0xd] == -1);
      auVar17[0xe] = -(pcVar1[0xe] == -1);
      auVar17[0xf] = -(pcVar1[0xf] == -1);
      if ((((((((((((((((SUB161(auVar17 >> 7,0) & 1) != 0 || (SUB161(auVar17 >> 0xf,0) & 1) != 0) ||
                      (SUB161(auVar17 >> 0x17,0) & 1) != 0) || (SUB161(auVar17 >> 0x1f,0) & 1) != 0)
                    || (SUB161(auVar17 >> 0x27,0) & 1) != 0) || (SUB161(auVar17 >> 0x2f,0) & 1) != 0
                   ) || (SUB161(auVar17 >> 0x37,0) & 1) != 0) ||
                 (SUB161(auVar17 >> 0x3f,0) & 1) != 0) || (SUB161(auVar17 >> 0x47,0) & 1) != 0) ||
               (SUB161(auVar17 >> 0x4f,0) & 1) != 0) || (SUB161(auVar17 >> 0x57,0) & 1) != 0) ||
             (SUB161(auVar17 >> 0x5f,0) & 1) != 0) || (SUB161(auVar17 >> 0x67,0) & 1) != 0) ||
           (SUB161(auVar17 >> 0x6f,0) & 1) != 0) || (SUB161(auVar17 >> 0x77,0) & 1) != 0) ||
          auVar17[0xf] < '\0') break;
      uVar7 = uVar7 + lVar8 + 0x10;
      lVar8 = lVar8 + 0x10;
    }
  }
  return;
}

