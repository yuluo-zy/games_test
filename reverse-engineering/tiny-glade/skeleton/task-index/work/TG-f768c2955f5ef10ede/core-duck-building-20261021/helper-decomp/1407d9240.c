
void __cdecl
system_decorator::decorator_visual::stairs_assemble::calculate_adjusted_ladder_positions::closure_0
          (void)

{
  int *piVar1;
  char *pcVar2;
  uint uVar3;
  uint uVar4;
  int iVar5;
  int iVar6;
  int iVar7;
  int iVar8;
  int extraout_EAX;
  int extraout_EAX_00;
  longlong extraout_RAX;
  longlong extraout_RAX_00;
  longlong extraout_RAX_01;
  ulonglong extraout_RAX_02;
  int iVar9;
  int iVar10;
  longlong *in_RDX;
  ulonglong uVar11;
  int in_R9D;
  longlong lVar12;
  uint uVar13;
  ushort uVar14;
  int *piVar15;
  char cVar16;
  byte bVar17;
  char cVar19;
  char cVar20;
  char cVar21;
  undefined1 auVar18 [16];
  undefined1 auVar22 [16];
  undefined1 auVar23 [16];
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
  
  country_core::resources::stairs::stairs_state::StairsState::graph();
  indexmap::map::IndexMap::
  get<country_core::resources::walls::decorator::DecoratorId,alloc::vec::Vec<tuple_<country_core::resources::walls::decorator::DecoratorId,petgraph::graphmap::CompactDirection>,alloc::alloc::Global>,std::hash::random::RandomState,country_core::resources::walls::decorator::DecoratorId>
            ();
  if ((extraout_RAX != 0) && (*(longlong *)(extraout_RAX + 0x10) != 0)) {
    piVar15 = *(int **)(extraout_RAX + 8);
    piVar1 = piVar15 + *(longlong *)(extraout_RAX + 0x10) * 2;
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
    do {
      iVar9 = *piVar15;
      indexmap::map::IndexMap::
      get<tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId>,tuple_<>,std::hash::random::RandomState,tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId>_>
                ();
      if (extraout_RAX_00 == 0) {
                    // WARNING: Subroutine does not return
        core::panicking::panic();
      }
      country_core::resources::stairs::stairs_state::StairSegment::new();
      iVar10 = in_R9D;
      country_core::resources::stairs::stairs_state::StairSegment::new();
      if (iVar10 != iVar9 || extraout_EAX_00 != extraout_EAX) {
        country_core::resources::stairs::stairs_state::StairSegment::new();
        country_core::resources::stairs::stairs_assembly_params::impl$2::from();
        if (in_RDX[3] != 0) {
          core::hash::BuildHasher::
          hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>();
          bVar17 = (byte)(extraout_RAX_02 >> 0x39);
          auVar18 = ZEXT216(CONCAT11(bVar17,bVar17));
          auVar18 = pshuflw(auVar18,auVar18,0);
          lVar12 = 0;
          uVar11 = extraout_RAX_02;
          while( true ) {
            uVar11 = in_RDX[1] & uVar11;
            pcVar2 = (char *)(*in_RDX + uVar11);
            cVar16 = auVar18[0];
            auVar23[0] = -(*pcVar2 == cVar16);
            cVar19 = auVar18[1];
            auVar23[1] = -(pcVar2[1] == cVar19);
            cVar20 = auVar18[2];
            auVar23[2] = -(pcVar2[2] == cVar20);
            cVar21 = auVar18[3];
            auVar23[3] = -(pcVar2[3] == cVar21);
            auVar23[4] = -(pcVar2[4] == cVar16);
            auVar23[5] = -(pcVar2[5] == cVar19);
            auVar23[6] = -(pcVar2[6] == cVar20);
            auVar23[7] = -(pcVar2[7] == cVar21);
            auVar23[8] = -(pcVar2[8] == cVar16);
            auVar23[9] = -(pcVar2[9] == cVar19);
            auVar23[10] = -(pcVar2[10] == cVar20);
            auVar23[0xb] = -(pcVar2[0xb] == cVar21);
            auVar23[0xc] = -(pcVar2[0xc] == cVar16);
            auVar23[0xd] = -(pcVar2[0xd] == cVar19);
            auVar23[0xe] = -(pcVar2[0xe] == cVar20);
            auVar23[0xf] = -(pcVar2[0xf] == cVar21);
            uVar14 = (ushort)(SUB161(auVar23 >> 7,0) & 1) |
                     (ushort)(SUB161(auVar23 >> 0xf,0) & 1) << 1 |
                     (ushort)(SUB161(auVar23 >> 0x17,0) & 1) << 2 |
                     (ushort)(SUB161(auVar23 >> 0x1f,0) & 1) << 3 |
                     (ushort)(SUB161(auVar23 >> 0x27,0) & 1) << 4 |
                     (ushort)(SUB161(auVar23 >> 0x2f,0) & 1) << 5 |
                     (ushort)(SUB161(auVar23 >> 0x37,0) & 1) << 6 |
                     (ushort)(SUB161(auVar23 >> 0x3f,0) & 1) << 7 |
                     (ushort)(SUB161(auVar23 >> 0x47,0) & 1) << 8 |
                     (ushort)(SUB161(auVar23 >> 0x4f,0) & 1) << 9 |
                     (ushort)(SUB161(auVar23 >> 0x57,0) & 1) << 10 |
                     (ushort)(SUB161(auVar23 >> 0x5f,0) & 1) << 0xb |
                     (ushort)(SUB161(auVar23 >> 0x67,0) & 1) << 0xc |
                     (ushort)(SUB161(auVar23 >> 0x6f,0) & 1) << 0xd |
                     (ushort)(SUB161(auVar23 >> 0x77,0) & 1) << 0xe |
                     (ushort)(auVar23[0xf] >> 7) << 0xf;
            uVar13 = (uint)uVar14;
            while (uVar14 != 0) {
              uVar3 = 0;
              for (uVar4 = uVar13; (uVar4 & 1) == 0; uVar4 = uVar4 >> 1 | 0x80000000) {
                uVar3 = uVar3 + 1;
              }
              if (extraout_RAX_01 ==
                  *(longlong *)(*in_RDX + -0x20 + (uVar3 + uVar11 & in_RDX[1]) * -0x20)) {
                return;
              }
              uVar14 = (ushort)(uVar13 - 1) & (ushort)uVar13;
              uVar13 = CONCAT22((short)(uVar13 - 1 >> 0x10),uVar14);
            }
            auVar22[0] = -(*pcVar2 == (char)iVar5);
            auVar22[1] = -(pcVar2[1] == (char)((uint)iVar5 >> 8));
            auVar22[2] = -(pcVar2[2] == (char)((uint)iVar5 >> 0x10));
            auVar22[3] = -(pcVar2[3] == (char)((uint)iVar5 >> 0x18));
            auVar22[4] = -(pcVar2[4] == (char)iVar6);
            auVar22[5] = -(pcVar2[5] == (char)((uint)iVar6 >> 8));
            auVar22[6] = -(pcVar2[6] == (char)((uint)iVar6 >> 0x10));
            auVar22[7] = -(pcVar2[7] == (char)((uint)iVar6 >> 0x18));
            auVar22[8] = -(pcVar2[8] == (char)iVar7);
            auVar22[9] = -(pcVar2[9] == (char)((uint)iVar7 >> 8));
            auVar22[10] = -(pcVar2[10] == (char)((uint)iVar7 >> 0x10));
            auVar22[0xb] = -(pcVar2[0xb] == (char)((uint)iVar7 >> 0x18));
            auVar22[0xc] = -(pcVar2[0xc] == (char)iVar8);
            auVar22[0xd] = -(pcVar2[0xd] == (char)((uint)iVar8 >> 8));
            auVar22[0xe] = -(pcVar2[0xe] == (char)((uint)iVar8 >> 0x10));
            auVar22[0xf] = -(pcVar2[0xf] == (char)((uint)iVar8 >> 0x18));
            if ((((((((((((((((SUB161(auVar22 >> 7,0) & 1) != 0 ||
                             (SUB161(auVar22 >> 0xf,0) & 1) != 0) ||
                            (SUB161(auVar22 >> 0x17,0) & 1) != 0) ||
                           (SUB161(auVar22 >> 0x1f,0) & 1) != 0) ||
                          (SUB161(auVar22 >> 0x27,0) & 1) != 0) ||
                         (SUB161(auVar22 >> 0x2f,0) & 1) != 0) ||
                        (SUB161(auVar22 >> 0x37,0) & 1) != 0) ||
                       (SUB161(auVar22 >> 0x3f,0) & 1) != 0) || (SUB161(auVar22 >> 0x47,0) & 1) != 0
                      ) || (SUB161(auVar22 >> 0x4f,0) & 1) != 0) ||
                    (SUB161(auVar22 >> 0x57,0) & 1) != 0) || (SUB161(auVar22 >> 0x5f,0) & 1) != 0)
                  || (SUB161(auVar22 >> 0x67,0) & 1) != 0) || (SUB161(auVar22 >> 0x6f,0) & 1) != 0)
                || (SUB161(auVar22 >> 0x77,0) & 1) != 0) || auVar22[0xf] < '\0') break;
            uVar11 = uVar11 + lVar12 + 0x10;
            lVar12 = lVar12 + 0x10;
          }
        }
      }
      piVar15 = piVar15 + 2;
    } while (piVar15 != piVar1);
  }
  return;
}

