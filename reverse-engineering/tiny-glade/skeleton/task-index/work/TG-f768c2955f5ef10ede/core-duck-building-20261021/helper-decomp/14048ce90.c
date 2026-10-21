
void __cdecl
core::iter::adapters::map::impl$2::
fold<tuple_<country_core::resources::walls::decorator::DecoratorId,system_decorator::decorator_visual::stairs::StairPoint>,petgraph::graphmap::Nodes<country_core::resources::walls::decorator::DecoratorId>,system_decorator::decorator_visual::stairs::generate_playermade_stair_requests::closure_env_0<false>,tuple_<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env_0<tuple_<country_core::resources::walls::decorator::DecoratorId,system_decorator::decorator_visual::stairs::StairPoint>,hashbrown::map::impl_82::extend::closure_env_0<country_core::resources::walls::decorator::DecoratorId,system_decorator::decorator_visual::stairs::StairPoint,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<petgraph::graphmap::Nodes<country_core::resources::walls::decorator::DecoratorId>,system_decorator::decorator_visual::stairs::generate_playermade_stair_requests::closure_env_0<false>_>_>_>_>
          (void)

{
  char *pcVar1;
  int iVar2;
  longlong lVar3;
  longlong lVar4;
  uint uVar5;
  uint uVar6;
  int iVar7;
  int iVar8;
  int iVar9;
  int iVar10;
  longlong extraout_RAX;
  longlong extraout_RAX_00;
  ulonglong extraout_RAX_01;
  longlong extraout_RAX_02;
  longlong *in_RCX;
  ulonglong uVar11;
  longlong lVar12;
  uint uVar13;
  ushort uVar14;
  longlong lVar15;
  longlong lVar16;
  byte bVar17;
  char cVar18;
  char cVar20;
  char cVar21;
  char cVar22;
  undefined1 auVar19 [16];
  undefined1 auVar23 [16];
  undefined1 auVar24 [16];
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
  
  lVar16 = *in_RCX;
  lVar3 = in_RCX[1];
  if (lVar16 == lVar3) {
    return;
  }
  iVar7 = -(uint)(CONCAT13(unaff_XMM6_Bd,
                           CONCAT12(unaff_XMM6_Bc,CONCAT11(unaff_XMM6_Bb,unaff_XMM6_Ba))) ==
                 CONCAT13(unaff_XMM6_Bd,
                          CONCAT12(unaff_XMM6_Bc,CONCAT11(unaff_XMM6_Bb,unaff_XMM6_Ba))));
  iVar8 = -(uint)(CONCAT13(unaff_XMM6_Bh,
                           CONCAT12(unaff_XMM6_Bg,CONCAT11(unaff_XMM6_Bf,unaff_XMM6_Be))) ==
                 CONCAT13(unaff_XMM6_Bh,
                          CONCAT12(unaff_XMM6_Bg,CONCAT11(unaff_XMM6_Bf,unaff_XMM6_Be))));
  iVar9 = -(uint)(CONCAT13(unaff_XMM6_Bl,
                           CONCAT12(unaff_XMM6_Bk,CONCAT11(unaff_XMM6_Bj,unaff_XMM6_Bi))) ==
                 CONCAT13(unaff_XMM6_Bl,
                          CONCAT12(unaff_XMM6_Bk,CONCAT11(unaff_XMM6_Bj,unaff_XMM6_Bi))));
  iVar10 = -(uint)(CONCAT13(unaff_XMM6_Bp,
                            CONCAT12(unaff_XMM6_Bo,CONCAT11(unaff_XMM6_Bn,unaff_XMM6_Bm))) ==
                  CONCAT13(unaff_XMM6_Bp,
                           CONCAT12(unaff_XMM6_Bo,CONCAT11(unaff_XMM6_Bn,unaff_XMM6_Bm))));
LAB_14048cef5:
  iVar2 = *(int *)(lVar16 + 0x20);
  country_core::resources::walls::decorator_storage::DecoratorStorage::try_lookup_storage();
  if (extraout_RAX == 3) {
                    // WARNING: Subroutine does not return
    option::unwrap_failed();
  }
  country_core::resources::walls::decorator_storage::DecoratorStorage::get();
  if (extraout_RAX_00 == 0) {
                    // WARNING: Subroutine does not return
    option::unwrap_failed();
  }
  if (*(longlong *)(extraout_RAX_00 + 0x48) != 0) {
    lVar16 = lVar16 + 0x28;
    hash::BuildHasher::
    hash_one<std::hash::random::RandomState,ref_<shader_pipeline::reflection::BindingIndex>_>();
    lVar4 = *(longlong *)(extraout_RAX_00 + 0x30);
    bVar17 = (byte)(extraout_RAX_01 >> 0x39);
    auVar19 = ZEXT216(CONCAT11(bVar17,bVar17));
    auVar19 = pshuflw(auVar19,auVar19,0);
    lVar12 = 0;
    uVar11 = extraout_RAX_01;
    while( true ) {
      uVar11 = *(ulonglong *)(extraout_RAX_00 + 0x38) & uVar11;
      pcVar1 = (char *)(lVar4 + uVar11);
      cVar18 = auVar19[0];
      auVar24[0] = -(*pcVar1 == cVar18);
      cVar20 = auVar19[1];
      auVar24[1] = -(pcVar1[1] == cVar20);
      cVar21 = auVar19[2];
      auVar24[2] = -(pcVar1[2] == cVar21);
      cVar22 = auVar19[3];
      auVar24[3] = -(pcVar1[3] == cVar22);
      auVar24[4] = -(pcVar1[4] == cVar18);
      auVar24[5] = -(pcVar1[5] == cVar20);
      auVar24[6] = -(pcVar1[6] == cVar21);
      auVar24[7] = -(pcVar1[7] == cVar22);
      auVar24[8] = -(pcVar1[8] == cVar18);
      auVar24[9] = -(pcVar1[9] == cVar20);
      auVar24[10] = -(pcVar1[10] == cVar21);
      auVar24[0xb] = -(pcVar1[0xb] == cVar22);
      auVar24[0xc] = -(pcVar1[0xc] == cVar18);
      auVar24[0xd] = -(pcVar1[0xd] == cVar20);
      auVar24[0xe] = -(pcVar1[0xe] == cVar21);
      auVar24[0xf] = -(pcVar1[0xf] == cVar22);
      uVar14 = (ushort)(SUB161(auVar24 >> 7,0) & 1) | (ushort)(SUB161(auVar24 >> 0xf,0) & 1) << 1 |
               (ushort)(SUB161(auVar24 >> 0x17,0) & 1) << 2 |
               (ushort)(SUB161(auVar24 >> 0x1f,0) & 1) << 3 |
               (ushort)(SUB161(auVar24 >> 0x27,0) & 1) << 4 |
               (ushort)(SUB161(auVar24 >> 0x2f,0) & 1) << 5 |
               (ushort)(SUB161(auVar24 >> 0x37,0) & 1) << 6 |
               (ushort)(SUB161(auVar24 >> 0x3f,0) & 1) << 7 |
               (ushort)(SUB161(auVar24 >> 0x47,0) & 1) << 8 |
               (ushort)(SUB161(auVar24 >> 0x4f,0) & 1) << 9 |
               (ushort)(SUB161(auVar24 >> 0x57,0) & 1) << 10 |
               (ushort)(SUB161(auVar24 >> 0x5f,0) & 1) << 0xb |
               (ushort)(SUB161(auVar24 >> 0x67,0) & 1) << 0xc |
               (ushort)(SUB161(auVar24 >> 0x6f,0) & 1) << 0xd |
               (ushort)(SUB161(auVar24 >> 0x77,0) & 1) << 0xe | (ushort)(auVar24[0xf] >> 7) << 0xf;
      uVar13 = (uint)uVar14;
      while (uVar14 != 0) {
        uVar5 = 0;
        for (uVar6 = uVar13; (uVar6 & 1) == 0; uVar6 = uVar6 >> 1 | 0x80000000) {
          uVar5 = uVar5 + 1;
        }
        lVar15 = (uVar5 + uVar11 & *(ulonglong *)(extraout_RAX_00 + 0x38)) * -0x50;
        if (iVar2 == *(int *)(lVar4 + -0x50 + lVar15)) {
          country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms::
          transform();
          country_core::resources::walls::decorator::DecoratorExtraInfo::as_stair();
          if (*(byte *)(lVar4 + -0x26 + lVar15) < 2) {
            country_core::resources::walls::decorator_storage::DecoratorStorageAddr::as_wall();
            country_core::resources::walls::public_walls::PublicWalls::get();
            if (extraout_RAX_02 == 0) {
                    // WARNING: Subroutine does not return
              option::unwrap_failed();
            }
            ::utils::wall_space::WallSpaceCoord::get_tangent();
          }
          hashbrown::map::HashMap::
          insert<country_core::resources::walls::decorator::DecoratorId,system_decorator::decorator_visual::stairs::StairPoint,std::hash::random::RandomState,alloc::alloc::Global>
                    ();
          if (lVar16 == lVar3) {
            return;
          }
          goto LAB_14048cef5;
        }
        uVar14 = (ushort)(uVar13 - 1) & (ushort)uVar13;
        uVar13 = CONCAT22((short)(uVar13 - 1 >> 0x10),uVar14);
      }
      auVar23[0] = -(*pcVar1 == (char)iVar7);
      auVar23[1] = -(pcVar1[1] == (char)((uint)iVar7 >> 8));
      auVar23[2] = -(pcVar1[2] == (char)((uint)iVar7 >> 0x10));
      auVar23[3] = -(pcVar1[3] == (char)((uint)iVar7 >> 0x18));
      auVar23[4] = -(pcVar1[4] == (char)iVar8);
      auVar23[5] = -(pcVar1[5] == (char)((uint)iVar8 >> 8));
      auVar23[6] = -(pcVar1[6] == (char)((uint)iVar8 >> 0x10));
      auVar23[7] = -(pcVar1[7] == (char)((uint)iVar8 >> 0x18));
      auVar23[8] = -(pcVar1[8] == (char)iVar9);
      auVar23[9] = -(pcVar1[9] == (char)((uint)iVar9 >> 8));
      auVar23[10] = -(pcVar1[10] == (char)((uint)iVar9 >> 0x10));
      auVar23[0xb] = -(pcVar1[0xb] == (char)((uint)iVar9 >> 0x18));
      auVar23[0xc] = -(pcVar1[0xc] == (char)iVar10);
      auVar23[0xd] = -(pcVar1[0xd] == (char)((uint)iVar10 >> 8));
      auVar23[0xe] = -(pcVar1[0xe] == (char)((uint)iVar10 >> 0x10));
      auVar23[0xf] = -(pcVar1[0xf] == (char)((uint)iVar10 >> 0x18));
      if ((((((((((((((((SUB161(auVar23 >> 7,0) & 1) != 0 || (SUB161(auVar23 >> 0xf,0) & 1) != 0) ||
                      (SUB161(auVar23 >> 0x17,0) & 1) != 0) || (SUB161(auVar23 >> 0x1f,0) & 1) != 0)
                    || (SUB161(auVar23 >> 0x27,0) & 1) != 0) || (SUB161(auVar23 >> 0x2f,0) & 1) != 0
                   ) || (SUB161(auVar23 >> 0x37,0) & 1) != 0) ||
                 (SUB161(auVar23 >> 0x3f,0) & 1) != 0) || (SUB161(auVar23 >> 0x47,0) & 1) != 0) ||
               (SUB161(auVar23 >> 0x4f,0) & 1) != 0) || (SUB161(auVar23 >> 0x57,0) & 1) != 0) ||
             (SUB161(auVar23 >> 0x5f,0) & 1) != 0) || (SUB161(auVar23 >> 0x67,0) & 1) != 0) ||
           (SUB161(auVar23 >> 0x6f,0) & 1) != 0) || (SUB161(auVar23 >> 0x77,0) & 1) != 0) ||
          auVar23[0xf] < '\0') break;
      uVar11 = uVar11 + lVar12 + 0x10;
      lVar12 = lVar12 + 0x10;
    }
  }
                    // WARNING: Subroutine does not return
  option::unwrap_failed();
}

