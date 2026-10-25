
void __cdecl
system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble::closure_9<true>
          (void)

{
  char *pcVar1;
  uint uVar2;
  uint uVar3;
  int iVar4;
  int iVar5;
  int iVar6;
  int iVar7;
  int extraout_EAX;
  int extraout_EAX_00;
  longlong extraout_RAX;
  ulonglong extraout_RAX_00;
  longlong extraout_RAX_01;
  longlong extraout_RAX_02;
  ulonglong extraout_RAX_03;
  int iVar8;
  int iVar9;
  longlong *in_RDX;
  ulonglong uVar10;
  ulonglong uVar11;
  int in_R9D;
  longlong lVar12;
  uint uVar13;
  ushort uVar14;
  char cVar15;
  byte bVar16;
  char cVar18;
  char cVar19;
  char cVar20;
  undefined1 auVar17 [16];
  undefined1 auVar21 [16];
  undefined1 auVar22 [16];
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
  int local_8c;
  longlong local_88;
  
  country_core::resources::stairs::stairs_state::StairsState::graph();
  uVar11 = *(ulonglong *)(extraout_RAX + 0x10);
  if ((uVar11 != 0) && (uVar11 != 1)) {
    indexmap::map::IndexMap::
    hash<country_core::resources::walls::decorator::DecoratorId,alloc::vec::Vec<tuple_<country_core::resources::walls::decorator::DecoratorId,petgraph::graphmap::CompactDirection>,alloc::alloc::Global>,std::hash::random::RandomState,country_core::resources::walls::decorator::DecoratorId>
              ();
    uVar10 = extraout_RAX_00;
    indexmap::map::core::IndexMapCore::
    get_index_of<country_core::resources::walls::decorator::DecoratorId,alloc::vec::Vec<tuple_<country_core::resources::walls::decorator::DecoratorId,petgraph::graphmap::CompactDirection>,alloc::alloc::Global>,country_core::resources::walls::decorator::DecoratorId>
              ();
    if ((extraout_RAX_01 == 1) && (uVar11 <= uVar10)) {
                    // WARNING: Subroutine does not return
      core::panicking::panic_bounds_check();
    }
  }
  petgraph::graphmap::impl$11::
  next<country_core::resources::walls::decorator::DecoratorId,tuple_<>,enum2_<petgraph::Undirected>,std::hash::random::RandomState>
            ();
  if (local_88 != 0) {
    iVar4 = -(uint)(CONCAT13(unaff_XMM6_Bd,
                             CONCAT12(unaff_XMM6_Bc,CONCAT11(unaff_XMM6_Bb,unaff_XMM6_Ba))) ==
                   CONCAT13(unaff_XMM6_Bd,
                            CONCAT12(unaff_XMM6_Bc,CONCAT11(unaff_XMM6_Bb,unaff_XMM6_Ba))));
    iVar5 = -(uint)(CONCAT13(unaff_XMM6_Bh,
                             CONCAT12(unaff_XMM6_Bg,CONCAT11(unaff_XMM6_Bf,unaff_XMM6_Be))) ==
                   CONCAT13(unaff_XMM6_Bh,
                            CONCAT12(unaff_XMM6_Bg,CONCAT11(unaff_XMM6_Bf,unaff_XMM6_Be))));
    iVar6 = -(uint)(CONCAT13(unaff_XMM6_Bl,
                             CONCAT12(unaff_XMM6_Bk,CONCAT11(unaff_XMM6_Bj,unaff_XMM6_Bi))) ==
                   CONCAT13(unaff_XMM6_Bl,
                            CONCAT12(unaff_XMM6_Bk,CONCAT11(unaff_XMM6_Bj,unaff_XMM6_Bi))));
    iVar7 = -(uint)(CONCAT13(unaff_XMM6_Bp,
                             CONCAT12(unaff_XMM6_Bo,CONCAT11(unaff_XMM6_Bn,unaff_XMM6_Bm))) ==
                   CONCAT13(unaff_XMM6_Bp,
                            CONCAT12(unaff_XMM6_Bo,CONCAT11(unaff_XMM6_Bn,unaff_XMM6_Bm))));
    do {
      iVar8 = local_8c;
      country_core::resources::stairs::stairs_state::StairSegment::new();
      iVar9 = in_R9D;
      country_core::resources::stairs::stairs_state::StairSegment::new();
      if (iVar9 != iVar8 || extraout_EAX_00 != extraout_EAX) {
        country_core::resources::stairs::stairs_state::StairSegment::new();
        country_core::resources::stairs::stairs_assembly_params::impl$2::from();
        if (in_RDX[3] != 0) {
          core::hash::BuildHasher::
          hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>();
          bVar16 = (byte)(extraout_RAX_03 >> 0x39);
          auVar17 = ZEXT216(CONCAT11(bVar16,bVar16));
          auVar17 = pshuflw(auVar17,auVar17,0);
          lVar12 = 0;
          uVar11 = extraout_RAX_03;
          while( true ) {
            uVar11 = in_RDX[1] & uVar11;
            pcVar1 = (char *)(*in_RDX + uVar11);
            cVar15 = auVar17[0];
            auVar22[0] = -(*pcVar1 == cVar15);
            cVar18 = auVar17[1];
            auVar22[1] = -(pcVar1[1] == cVar18);
            cVar19 = auVar17[2];
            auVar22[2] = -(pcVar1[2] == cVar19);
            cVar20 = auVar17[3];
            auVar22[3] = -(pcVar1[3] == cVar20);
            auVar22[4] = -(pcVar1[4] == cVar15);
            auVar22[5] = -(pcVar1[5] == cVar18);
            auVar22[6] = -(pcVar1[6] == cVar19);
            auVar22[7] = -(pcVar1[7] == cVar20);
            auVar22[8] = -(pcVar1[8] == cVar15);
            auVar22[9] = -(pcVar1[9] == cVar18);
            auVar22[10] = -(pcVar1[10] == cVar19);
            auVar22[0xb] = -(pcVar1[0xb] == cVar20);
            auVar22[0xc] = -(pcVar1[0xc] == cVar15);
            auVar22[0xd] = -(pcVar1[0xd] == cVar18);
            auVar22[0xe] = -(pcVar1[0xe] == cVar19);
            auVar22[0xf] = -(pcVar1[0xf] == cVar20);
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
                     (ushort)(auVar22[0xf] >> 7) << 0xf;
            uVar13 = (uint)uVar14;
            while (uVar14 != 0) {
              uVar2 = 0;
              for (uVar3 = uVar13; (uVar3 & 1) == 0; uVar3 = uVar3 >> 1 | 0x80000000) {
                uVar2 = uVar2 + 1;
              }
              if (extraout_RAX_02 ==
                  *(longlong *)(*in_RDX + -0x20 + (uVar2 + uVar11 & in_RDX[1]) * -0x20)) {
                return;
              }
              uVar14 = (ushort)(uVar13 - 1) & (ushort)uVar13;
              uVar13 = CONCAT22((short)(uVar13 - 1 >> 0x10),uVar14);
            }
            auVar21[0] = -(*pcVar1 == (char)iVar4);
            auVar21[1] = -(pcVar1[1] == (char)((uint)iVar4 >> 8));
            auVar21[2] = -(pcVar1[2] == (char)((uint)iVar4 >> 0x10));
            auVar21[3] = -(pcVar1[3] == (char)((uint)iVar4 >> 0x18));
            auVar21[4] = -(pcVar1[4] == (char)iVar5);
            auVar21[5] = -(pcVar1[5] == (char)((uint)iVar5 >> 8));
            auVar21[6] = -(pcVar1[6] == (char)((uint)iVar5 >> 0x10));
            auVar21[7] = -(pcVar1[7] == (char)((uint)iVar5 >> 0x18));
            auVar21[8] = -(pcVar1[8] == (char)iVar6);
            auVar21[9] = -(pcVar1[9] == (char)((uint)iVar6 >> 8));
            auVar21[10] = -(pcVar1[10] == (char)((uint)iVar6 >> 0x10));
            auVar21[0xb] = -(pcVar1[0xb] == (char)((uint)iVar6 >> 0x18));
            auVar21[0xc] = -(pcVar1[0xc] == (char)iVar7);
            auVar21[0xd] = -(pcVar1[0xd] == (char)((uint)iVar7 >> 8));
            auVar21[0xe] = -(pcVar1[0xe] == (char)((uint)iVar7 >> 0x10));
            auVar21[0xf] = -(pcVar1[0xf] == (char)((uint)iVar7 >> 0x18));
            if ((((((((((((((((SUB161(auVar21 >> 7,0) & 1) != 0 ||
                             (SUB161(auVar21 >> 0xf,0) & 1) != 0) ||
                            (SUB161(auVar21 >> 0x17,0) & 1) != 0) ||
                           (SUB161(auVar21 >> 0x1f,0) & 1) != 0) ||
                          (SUB161(auVar21 >> 0x27,0) & 1) != 0) ||
                         (SUB161(auVar21 >> 0x2f,0) & 1) != 0) ||
                        (SUB161(auVar21 >> 0x37,0) & 1) != 0) ||
                       (SUB161(auVar21 >> 0x3f,0) & 1) != 0) || (SUB161(auVar21 >> 0x47,0) & 1) != 0
                      ) || (SUB161(auVar21 >> 0x4f,0) & 1) != 0) ||
                    (SUB161(auVar21 >> 0x57,0) & 1) != 0) || (SUB161(auVar21 >> 0x5f,0) & 1) != 0)
                  || (SUB161(auVar21 >> 0x67,0) & 1) != 0) || (SUB161(auVar21 >> 0x6f,0) & 1) != 0)
                || (SUB161(auVar21 >> 0x77,0) & 1) != 0) || auVar21[0xf] < '\0') break;
            uVar11 = uVar11 + lVar12 + 0x10;
            lVar12 = lVar12 + 0x10;
          }
        }
      }
      petgraph::graphmap::impl$11::
      next<country_core::resources::walls::decorator::DecoratorId,tuple_<>,enum2_<petgraph::Undirected>,std::hash::random::RandomState>
                ();
    } while (local_88 != 0);
  }
  return;
}

