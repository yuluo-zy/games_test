
// WARNING: Removing unreachable block (ram,0x0001408fed04)
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __cdecl
country_core::resources::stairs::stairs_state::StairsState::get_all_connected_nodes(void)

{
  char *pcVar1;
  uint uVar2;
  ulonglong uVar3;
  undefined1 auVar4 [16];
  uint uVar5;
  uint uVar6;
  int iVar7;
  int iVar8;
  int iVar9;
  int iVar10;
  undefined *puVar11;
  char extraout_AL;
  char extraout_AL_00;
  uint *extraout_RAX;
  ulonglong extraout_RAX_00;
  ulonglong extraout_RAX_01;
  longlong extraout_RAX_02;
  undefined8 *in_RCX;
  longlong in_RDX;
  ulonglong uVar12;
  longlong lVar13;
  uint in_R8D;
  longlong lVar14;
  ulonglong uVar15;
  uint uVar16;
  ushort uVar17;
  uint *puVar18;
  uint *puVar19;
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
  longlong lStack_190;
  longlong local_160;
  ulonglong uStack_158;
  longlong lStack_148;
  longlong local_e8;
  ulonglong local_68;
  
  std::alloc::__default_lib_allocator::__rust_alloc();
  if (extraout_RAX == (uint *)0x0) {
                    // WARNING: Subroutine does not return
    alloc::alloc::handle_alloc_error();
  }
  *extraout_RAX = in_R8D;
  local_68 = 1;
  foldhash::seed::fast::impl$0::default();
  hashbrown::raw::RawTableInner::
  fallible_with_capacity<allocator_api2::stable::alloc::global::Global>();
  foldhash::seed::fast::impl$0::default();
  hashbrown::raw::RawTableInner::
  fallible_with_capacity<allocator_api2::stable::alloc::global::Global>();
  alloc::raw_vec::RawVec::
  grow_one<country_core::render::sparse_instance_buffer::ChunkId,alloc::alloc::Global>();
  puVar11 = __imp__ZN8foldhash4seed6global19GLOBAL_SEED_STORAGE17h1d31b2cef4f74946E;
  local_e8 = 1;
  lVar14 = *(longlong *)(in_RDX + 0x30);
  uVar3 = *(ulonglong *)(in_RDX + 0x38);
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
  _DAT_00000004 = in_R8D;
  while (local_e8 != 0) {
    uVar2 = *(uint *)(local_e8 * 4);
    petgraph::visit::impl$5::
    visit<country_core::resources::walls::decorator::DecoratorId,foldhash::seed::fast::RandomState>
              ();
    if (extraout_AL == '\0') {
      if (local_e8 != 0) {
        local_e8 = local_e8 + -1;
      }
      petgraph::visit::impl$5::
      visit<country_core::resources::walls::decorator::DecoratorId,foldhash::seed::fast::RandomState>
                ();
      if (extraout_AL_00 != '\0') {
        if (local_68 == 1) {
          alloc::raw_vec::RawVec::
          grow_one<country_core::render::sparse_instance_buffer::ChunkId,alloc::alloc::Global>();
        }
        extraout_RAX[local_68] = uVar2;
        local_68 = local_68 + 1;
      }
    }
    else {
      puVar19 = (uint *)&DAT_00000004;
      if (uVar3 == 0) {
        puVar18 = (uint *)&DAT_00000004;
      }
      else if (uVar3 == 1) {
        puVar18 = (uint *)&DAT_00000004;
        if (*(uint *)(lVar14 + 0x20) == uVar2) {
          uVar12 = 0;
LAB_1408febb2:
          puVar18 = *(uint **)(lVar14 + 8 + uVar12 * 0x28);
          puVar19 = puVar18 + *(longlong *)(lVar14 + 0x10 + uVar12 * 0x28) * 2;
        }
      }
      else {
        indexmap::map::IndexMap::
        hash<country_core::resources::walls::decorator::DecoratorId,alloc::vec::Vec<tuple_<country_core::resources::walls::decorator::DecoratorId,petgraph::graphmap::CompactDirection>,alloc::alloc::Global>,std::hash::random::RandomState,country_core::resources::walls::decorator::DecoratorId>
                  ();
        uVar12 = extraout_RAX_01;
        indexmap::map::core::IndexMapCore::
        get_index_of<country_core::resources::walls::decorator::DecoratorId,alloc::vec::Vec<tuple_<country_core::resources::walls::decorator::DecoratorId,petgraph::graphmap::CompactDirection>,alloc::alloc::Global>,country_core::resources::walls::decorator::DecoratorId>
                  ();
        if (extraout_RAX_02 == 1) {
          if (uVar3 <= uVar12) {
                    // WARNING: Subroutine does not return
            core::panicking::panic_bounds_check();
          }
          puVar18 = (uint *)&DAT_00000004;
          if (lVar14 != 0) goto LAB_1408febb2;
        }
        else {
          puVar18 = (uint *)&DAT_00000004;
        }
      }
LAB_1408febef:
      if (puVar18 != puVar19) {
        uVar2 = *puVar18;
        puVar18 = puVar18 + 2;
        if (lStack_148 != 0) {
          auVar22._8_8_ = 0;
          auVar22._0_8_ = extraout_RAX_00 ^ uVar2;
          auVar4._8_8_ = 0;
          auVar4._0_8_ = *(ulonglong *)puVar11;
          uVar12 = SUB168(auVar22 * auVar4,8) ^ SUB168(auVar22 * auVar4,0);
          bVar20 = (byte)(uVar12 >> 0x39);
          auVar22 = ZEXT216(CONCAT11(bVar20,bVar20));
          auVar22 = pshuflw(auVar22,auVar22,0);
          lVar13 = 0;
          while( true ) {
            uVar12 = uStack_158 & uVar12;
            pcVar1 = (char *)(local_160 + uVar12);
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
            uVar17 = (ushort)(SUB161(auVar27 >> 7,0) & 1) |
                     (ushort)(SUB161(auVar27 >> 0xf,0) & 1) << 1 |
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
                     (ushort)(SUB161(auVar27 >> 0x77,0) & 1) << 0xe |
                     (ushort)(auVar27[0xf] >> 7) << 0xf;
            uVar16 = (uint)uVar17;
            while (uVar17 != 0) {
              uVar5 = 0;
              for (uVar6 = uVar16; (uVar6 & 1) == 0; uVar6 = uVar6 >> 1 | 0x80000000) {
                uVar5 = uVar5 + 1;
              }
              if (*(uint *)(local_160 + (uVar5 + uVar12 & uStack_158) * -4 + -4) == uVar2)
              goto LAB_1408febef;
              uVar17 = (ushort)(uVar16 - 1) & (ushort)uVar16;
              uVar16 = CONCAT22((short)(uVar16 - 1 >> 0x10),uVar17);
            }
            auVar26[0] = -(*pcVar1 == (char)iVar7);
            auVar26[1] = -(pcVar1[1] == (char)((uint)iVar7 >> 8));
            auVar26[2] = -(pcVar1[2] == (char)((uint)iVar7 >> 0x10));
            auVar26[3] = -(pcVar1[3] == (char)((uint)iVar7 >> 0x18));
            auVar26[4] = -(pcVar1[4] == (char)iVar8);
            auVar26[5] = -(pcVar1[5] == (char)((uint)iVar8 >> 8));
            auVar26[6] = -(pcVar1[6] == (char)((uint)iVar8 >> 0x10));
            auVar26[7] = -(pcVar1[7] == (char)((uint)iVar8 >> 0x18));
            auVar26[8] = -(pcVar1[8] == (char)iVar9);
            auVar26[9] = -(pcVar1[9] == (char)((uint)iVar9 >> 8));
            auVar26[10] = -(pcVar1[10] == (char)((uint)iVar9 >> 0x10));
            auVar26[0xb] = -(pcVar1[0xb] == (char)((uint)iVar9 >> 0x18));
            auVar26[0xc] = -(pcVar1[0xc] == (char)iVar10);
            auVar26[0xd] = -(pcVar1[0xd] == (char)((uint)iVar10 >> 8));
            auVar26[0xe] = -(pcVar1[0xe] == (char)((uint)iVar10 >> 0x10));
            auVar26[0xf] = -(pcVar1[0xf] == (char)((uint)iVar10 >> 0x18));
            if ((((((((((((((((SUB161(auVar26 >> 7,0) & 1) != 0 ||
                             (SUB161(auVar26 >> 0xf,0) & 1) != 0) ||
                            (SUB161(auVar26 >> 0x17,0) & 1) != 0) ||
                           (SUB161(auVar26 >> 0x1f,0) & 1) != 0) ||
                          (SUB161(auVar26 >> 0x27,0) & 1) != 0) ||
                         (SUB161(auVar26 >> 0x2f,0) & 1) != 0) ||
                        (SUB161(auVar26 >> 0x37,0) & 1) != 0) ||
                       (SUB161(auVar26 >> 0x3f,0) & 1) != 0) || (SUB161(auVar26 >> 0x47,0) & 1) != 0
                      ) || (SUB161(auVar26 >> 0x4f,0) & 1) != 0) ||
                    (SUB161(auVar26 >> 0x57,0) & 1) != 0) || (SUB161(auVar26 >> 0x5f,0) & 1) != 0)
                  || (SUB161(auVar26 >> 0x67,0) & 1) != 0) || (SUB161(auVar26 >> 0x6f,0) & 1) != 0)
                || (SUB161(auVar26 >> 0x77,0) & 1) != 0) || auVar26[0xf] < '\0') break;
            uVar12 = uVar12 + lVar13 + 0x10;
            lVar13 = lVar13 + 0x10;
          }
        }
        if (local_e8 == 0) {
          alloc::raw_vec::RawVec::
          grow_one<country_core::render::sparse_instance_buffer::ChunkId,alloc::alloc::Global>();
        }
        *(uint *)(local_e8 * 4 + 4) = uVar2;
        local_e8 = local_e8 + 1;
        goto LAB_1408febef;
      }
    }
  }
  uVar3 = local_68;
  if (1 < local_68) {
    if (local_68 < 0x15) {
      puVar19 = extraout_RAX + 1;
      lVar14 = 4;
      do {
        uVar16 = puVar19[-1];
        uVar2 = *puVar19;
        lVar13 = lVar14;
        if (uVar2 < uVar16) {
          do {
            *(uint *)((longlong)extraout_RAX + lVar13) = uVar16;
            puVar18 = extraout_RAX;
            if (lVar13 == 4) goto LAB_1408fedbf;
            uVar16 = *(uint *)((longlong)extraout_RAX + lVar13 + -8);
            lVar13 = lVar13 + -4;
          } while (uVar2 < uVar16);
          puVar18 = (uint *)(lVar13 + (longlong)extraout_RAX);
LAB_1408fedbf:
          *puVar18 = uVar2;
        }
        puVar19 = puVar19 + 1;
        lVar14 = lVar14 + 4;
      } while (puVar19 != extraout_RAX + local_68);
    }
    else {
      core::slice::sort::unstable::
      ipnsort<country_core::resources::walls::decorator::DecoratorId,bool_(*)(ref_<country_core::resources::walls::decorator::DecoratorId>,ref_<country_core::resources::walls::decorator::DecoratorId>)>
                ();
    }
    if (1 < local_68) {
      uVar12 = 1;
      do {
        uVar15 = uVar12;
        if (extraout_RAX[uVar12] == extraout_RAX[uVar12 - 1]) goto LAB_1408fee69;
        uVar12 = uVar12 + 1;
      } while (local_68 != uVar12);
    }
  }
LAB_1408fecdf:
  local_68 = uVar3;
  in_RCX[2] = local_68;
  *in_RCX = 1;
  in_RCX[1] = extraout_RAX;
  if ((uStack_158 != 0) && (uStack_158 + (uStack_158 * 4 + 0x13 & 0xfffffffffffffff0) != -0x11)) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  if ((lStack_190 != 0) && (lStack_190 + (lStack_190 * 4 + 0x13U & 0xfffffffffffffff0) != -0x11)) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  return;
LAB_1408fee69:
  while (uVar15 = uVar15 + 1, uVar3 = uVar12, uVar15 < local_68) {
    if (extraout_RAX[uVar15] != extraout_RAX[uVar12 - 1]) {
      extraout_RAX[uVar12] = extraout_RAX[uVar15];
      uVar12 = uVar12 + 1;
    }
  }
  goto LAB_1408fecdf;
}

