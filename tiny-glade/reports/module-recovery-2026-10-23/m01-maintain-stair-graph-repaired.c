
/* WARNING: Removing unreachable block (ram,0x000140806751) */
/* WARNING: Removing unreachable block (ram,0x000140806c5d) */
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void __cdecl
system_decorator::decorator_visual::stairs::maintain_stair_graph
          (void *history,void *decorator_physics,void *storage,void *archivist,void *stairs,
          void *structural_graph,void *raycast_world,void *stair_colliders,void *deferred_events)

{
  byte *pbVar1;
  char *pcVar2;
  byte bVar3;
  byte bVar4;
  int iVar5;
  undefined4 uVar6;
  undefined4 uVar7;
  undefined4 *puVar8;
  undefined4 *puVar9;
  uint uVar10;
  int iVar11;
  int iVar12;
  int iVar13;
  int iVar14;
  bool bVar15;
  char extraout_AL;
  char extraout_AL_00;
  char extraout_AL_01;
  char extraout_AL_02;
  uint uVar16;
  longlong extraout_RAX;
  longlong extraout_RAX_00;
  longlong *extraout_RAX_01;
  longlong extraout_RAX_02;
  longlong extraout_RAX_03;
  longlong extraout_RAX_04;
  ulonglong extraout_RAX_05;
  longlong extraout_RAX_06;
  longlong extraout_RAX_07;
  longlong *extraout_RAX_08;
  longlong extraout_RAX_09;
  longlong *extraout_RAX_10;
  longlong *extraout_RAX_11;
  longlong *extraout_RAX_12;
  longlong *plVar17;
  ulonglong uVar18;
  int *piVar19;
  undefined1 (*pauVar20) [16];
  undefined *puVar21;
  longlong lVar22;
  ushort uVar23;
  longlong lVar24;
  uint uVar25;
  longlong lVar26;
  int *piVar27;
  byte bVar28;
  char cVar29;
  char cVar31;
  char cVar32;
  char cVar33;
  undefined1 auVar30 [16];
  undefined1 auVar34 [16];
  undefined1 auVar35 [16];
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
  int local_1d8;
  int local_1c8;
  char local_1b8;
  longlong *local_1a0;
  longlong local_100;
  longlong local_e0;
  longlong local_c8;
  
  if (*__imp__ZN12country_core26_DEV_OVERRIDE_IS_DEMO_MODE17h6f2a49c135369f5aE == '\0') {
    puffin::are_scopes_on();
    if (extraout_AL == '\0') {
      bVar15 = false;
    }
    else {
      uVar18 = 0x40;
      core::str::impl$0::rfind<ref_<str_>_>();
      if (extraout_RAX == 1) {
        if (uVar18 != 0) {
          if (uVar18 < 0x40) {
            if ((char)(&DAT_142a8dc3b)[uVar18] < -0x40) {
LAB_1408061a8:
                    /* WARNING: Subroutine does not return */
              core::str::slice_error_fail();
            }
          }
          else if (uVar18 != 0x40) goto LAB_1408061a8;
        }
        core::str::impl$0::rfind<ref_<str_>_>();
        if ((extraout_RAX_00 == 1) && (uVar18 != 0xfffffffffffffffe)) {
          if (uVar18 + 2 < 0x40) {
            if ((char)(&UNK_142a8dc3d)[uVar18] < -0x40) {
LAB_140806206:
                    /* WARNING: Subroutine does not return */
              core::str::slice_error_fail();
            }
          }
          else if (uVar18 + 2 != 0x40) goto LAB_140806206;
        }
      }
      puVar21 = &DAT_142a8dd7f;
      do {
        bVar28 = puVar21[-1];
        uVar16 = (uint)(char)bVar28;
        if ((char)bVar28 < '\0') {
          bVar3 = puVar21[-2];
          if ((char)bVar3 < -0x40) {
            bVar4 = puVar21[-3];
            if ((char)bVar4 < -0x40) {
              pbVar1 = puVar21 + -4;
              puVar21 = puVar21 + -4;
              uVar16 = bVar4 & 0x3f | (*pbVar1 & 7) << 6;
            }
            else {
              puVar21 = puVar21 + -3;
              uVar16 = bVar4 & 0xf;
            }
            uVar16 = bVar3 & 0x3f | uVar16 << 6;
          }
          else {
            puVar21 = puVar21 + -2;
            uVar16 = bVar3 & 0x1f;
          }
          uVar16 = (uint)(bVar28 & 0x3f) | uVar16 << 6;
          if (uVar16 != 0x5c) goto LAB_1408062c4;
LAB_1408062d6:
          if (puVar21 == &UNK_142a8dd47) break;
          if (puVar21 + -0x142a8dd47 < (undefined *)0x37) {
            if (-0x41 < (char)puVar21[1]) break;
          }
          else if (puVar21 + -0x142a8dd47 == (undefined *)0x37) break;
                    /* WARNING: Subroutine does not return */
          core::str::slice_error_fail();
        }
        puVar21 = puVar21 + -1;
        if (uVar16 == 0x5c) goto LAB_1408062d6;
LAB_1408062c4:
        if (uVar16 == 0x2f) goto LAB_1408062d6;
      } while (puVar21 != &anon_51762f656d63710c1898be1db88241c3_50_llvm_6858271601110728334);
      puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
      if (*extraout_RAX_01 == 1) {
        plVar17 = extraout_RAX_01 + 1;
      }
      else if ((*extraout_RAX_01 != 0) ||
              (std::sys::thread_local::native::lazy::Storage::
               initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                         (), plVar17 = extraout_RAX_12, extraout_RAX_12 == (longlong *)0x0))
      goto LAB_140806d03;
      if (*plVar17 != 0) goto LAB_140806d0f;
      *plVar17 = -1;
      puffin::ThreadProfiler::begin_scope();
      *plVar17 = *plVar17 + 1;
      bVar15 = true;
    }
    country_core::resources::history::History::is_recording();
    local_e0 = 0;
    local_c8 = 0;
    country_core::resources::stairs::stairs_state::StairsState::graph();
    lVar24 = *(longlong *)(extraout_RAX_02 + 8);
    lVar26 = *(longlong *)(extraout_RAX_02 + 0x10) * 0x28 + lVar24;
    iVar11 = -(uint)(CONCAT13(unaff_XMM6_Bd,
                              CONCAT12(unaff_XMM6_Bc,CONCAT11(unaff_XMM6_Bb,unaff_XMM6_Ba))) ==
                    CONCAT13(unaff_XMM6_Bd,
                             CONCAT12(unaff_XMM6_Bc,CONCAT11(unaff_XMM6_Bb,unaff_XMM6_Ba))));
    iVar12 = -(uint)(CONCAT13(unaff_XMM6_Bh,
                              CONCAT12(unaff_XMM6_Bg,CONCAT11(unaff_XMM6_Bf,unaff_XMM6_Be))) ==
                    CONCAT13(unaff_XMM6_Bh,
                             CONCAT12(unaff_XMM6_Bg,CONCAT11(unaff_XMM6_Bf,unaff_XMM6_Be))));
    iVar13 = -(uint)(CONCAT13(unaff_XMM6_Bl,
                              CONCAT12(unaff_XMM6_Bk,CONCAT11(unaff_XMM6_Bj,unaff_XMM6_Bi))) ==
                    CONCAT13(unaff_XMM6_Bl,
                             CONCAT12(unaff_XMM6_Bk,CONCAT11(unaff_XMM6_Bj,unaff_XMM6_Bi))));
    iVar14 = -(uint)(CONCAT13(unaff_XMM6_Bp,
                              CONCAT12(unaff_XMM6_Bo,CONCAT11(unaff_XMM6_Bn,unaff_XMM6_Bm))) ==
                    CONCAT13(unaff_XMM6_Bp,
                             CONCAT12(unaff_XMM6_Bo,CONCAT11(unaff_XMM6_Bn,unaff_XMM6_Bm))));
LAB_140806455:
    if (lVar24 != lVar26) {
      iVar5 = *(int *)(lVar24 + 0x20);
      country_core::resources::walls::decorator_storage::DecoratorStorage::try_lookup_storage();
      lVar24 = lVar24 + 0x28;
      if (extraout_RAX_03 != 3) {
        country_core::resources::walls::decorator_storage::DecoratorStorage::get();
        if (extraout_RAX_04 == 0) {
                    /* WARNING: Subroutine does not return */
          core::option::unwrap_failed();
        }
        local_1d8 = iVar5;
        if (*(longlong *)(extraout_RAX_04 + 0x48) != 0) {
          core::hash::BuildHasher::
          hash_one<std::hash::random::RandomState,ref_<shader_pipeline::reflection::BindingIndex>_>
                    ();
          bVar28 = (byte)(extraout_RAX_05 >> 0x39);
          auVar30 = ZEXT216(CONCAT11(bVar28,bVar28));
          auVar30 = pshuflw(auVar30,auVar30,0);
          lVar22 = 0;
          uVar18 = extraout_RAX_05;
          while( true ) {
            uVar18 = *(ulonglong *)(extraout_RAX_04 + 0x38) & uVar18;
            pcVar2 = (char *)(*(longlong *)(extraout_RAX_04 + 0x30) + uVar18);
            cVar29 = auVar30[0];
            auVar35[0] = -(*pcVar2 == cVar29);
            cVar31 = auVar30[1];
            auVar35[1] = -(pcVar2[1] == cVar31);
            cVar32 = auVar30[2];
            auVar35[2] = -(pcVar2[2] == cVar32);
            cVar33 = auVar30[3];
            auVar35[3] = -(pcVar2[3] == cVar33);
            auVar35[4] = -(pcVar2[4] == cVar29);
            auVar35[5] = -(pcVar2[5] == cVar31);
            auVar35[6] = -(pcVar2[6] == cVar32);
            auVar35[7] = -(pcVar2[7] == cVar33);
            auVar35[8] = -(pcVar2[8] == cVar29);
            auVar35[9] = -(pcVar2[9] == cVar31);
            auVar35[10] = -(pcVar2[10] == cVar32);
            auVar35[0xb] = -(pcVar2[0xb] == cVar33);
            auVar35[0xc] = -(pcVar2[0xc] == cVar29);
            auVar35[0xd] = -(pcVar2[0xd] == cVar31);
            auVar35[0xe] = -(pcVar2[0xe] == cVar32);
            auVar35[0xf] = -(pcVar2[0xf] == cVar33);
            uVar23 = (ushort)(SUB161(auVar35 >> 7,0) & 1) |
                     (ushort)(SUB161(auVar35 >> 0xf,0) & 1) << 1 |
                     (ushort)(SUB161(auVar35 >> 0x17,0) & 1) << 2 |
                     (ushort)(SUB161(auVar35 >> 0x1f,0) & 1) << 3 |
                     (ushort)(SUB161(auVar35 >> 0x27,0) & 1) << 4 |
                     (ushort)(SUB161(auVar35 >> 0x2f,0) & 1) << 5 |
                     (ushort)(SUB161(auVar35 >> 0x37,0) & 1) << 6 |
                     (ushort)(SUB161(auVar35 >> 0x3f,0) & 1) << 7 |
                     (ushort)(SUB161(auVar35 >> 0x47,0) & 1) << 8 |
                     (ushort)(SUB161(auVar35 >> 0x4f,0) & 1) << 9 |
                     (ushort)(SUB161(auVar35 >> 0x57,0) & 1) << 10 |
                     (ushort)(SUB161(auVar35 >> 0x5f,0) & 1) << 0xb |
                     (ushort)(SUB161(auVar35 >> 0x67,0) & 1) << 0xc |
                     (ushort)(SUB161(auVar35 >> 0x6f,0) & 1) << 0xd |
                     (ushort)(SUB161(auVar35 >> 0x77,0) & 1) << 0xe |
                     (ushort)(auVar35[0xf] >> 7) << 0xf;
            uVar16 = (uint)uVar23;
            while (uVar23 != 0) {
              uVar25 = 0;
              for (uVar10 = uVar16; (uVar10 & 1) == 0; uVar10 = uVar10 >> 1 | 0x80000000) {
                uVar25 = uVar25 + 1;
              }
              if (iVar5 == *(int *)(*(longlong *)(extraout_RAX_04 + 0x30) + -0x50 +
                                   (uVar25 + uVar18 & *(ulonglong *)(extraout_RAX_04 + 0x38)) *
                                   -0x50)) goto LAB_140806455;
              uVar23 = (ushort)(uVar16 - 1) & (ushort)uVar16;
              uVar16 = CONCAT22((short)(uVar16 - 1 >> 0x10),uVar23);
            }
            auVar34[0] = -(*pcVar2 == (char)iVar11);
            auVar34[1] = -(pcVar2[1] == (char)((uint)iVar11 >> 8));
            auVar34[2] = -(pcVar2[2] == (char)((uint)iVar11 >> 0x10));
            auVar34[3] = -(pcVar2[3] == (char)((uint)iVar11 >> 0x18));
            auVar34[4] = -(pcVar2[4] == (char)iVar12);
            auVar34[5] = -(pcVar2[5] == (char)((uint)iVar12 >> 8));
            auVar34[6] = -(pcVar2[6] == (char)((uint)iVar12 >> 0x10));
            auVar34[7] = -(pcVar2[7] == (char)((uint)iVar12 >> 0x18));
            auVar34[8] = -(pcVar2[8] == (char)iVar13);
            auVar34[9] = -(pcVar2[9] == (char)((uint)iVar13 >> 8));
            auVar34[10] = -(pcVar2[10] == (char)((uint)iVar13 >> 0x10));
            auVar34[0xb] = -(pcVar2[0xb] == (char)((uint)iVar13 >> 0x18));
            auVar34[0xc] = -(pcVar2[0xc] == (char)iVar14);
            auVar34[0xd] = -(pcVar2[0xd] == (char)((uint)iVar14 >> 8));
            auVar34[0xe] = -(pcVar2[0xe] == (char)((uint)iVar14 >> 0x10));
            auVar34[0xf] = -(pcVar2[0xf] == (char)((uint)iVar14 >> 0x18));
            if ((((((((((((((((SUB161(auVar34 >> 7,0) & 1) != 0 ||
                             (SUB161(auVar34 >> 0xf,0) & 1) != 0) ||
                            (SUB161(auVar34 >> 0x17,0) & 1) != 0) ||
                           (SUB161(auVar34 >> 0x1f,0) & 1) != 0) ||
                          (SUB161(auVar34 >> 0x27,0) & 1) != 0) ||
                         (SUB161(auVar34 >> 0x2f,0) & 1) != 0) ||
                        (SUB161(auVar34 >> 0x37,0) & 1) != 0) ||
                       (SUB161(auVar34 >> 0x3f,0) & 1) != 0) || (SUB161(auVar34 >> 0x47,0) & 1) != 0
                      ) || (SUB161(auVar34 >> 0x4f,0) & 1) != 0) ||
                    (SUB161(auVar34 >> 0x57,0) & 1) != 0) || (SUB161(auVar34 >> 0x5f,0) & 1) != 0)
                  || (SUB161(auVar34 >> 0x67,0) & 1) != 0) || (SUB161(auVar34 >> 0x6f,0) & 1) != 0)
                || (SUB161(auVar34 >> 0x77,0) & 1) != 0) || auVar34[0xf] < '\0') break;
            uVar18 = uVar18 + lVar22 + 0x10;
            lVar22 = lVar22 + 0x10;
          }
        }
      }
      if (local_c8 == 0) {
        alloc::raw_vec::RawVec::
        grow_one<country_core::render::sparse_instance_buffer::ChunkId,alloc::alloc::Global>();
      }
      *(int *)(local_c8 * 4 + 4) = iVar5;
      local_c8 = local_c8 + 1;
      goto LAB_140806455;
    }
    uVar6 = *(undefined4 *)((longlong)stairs + 0x1c);
    puVar8 = *(undefined4 **)((longlong)stairs + 0x10);
    while (local_c8 != 0) {
      lVar24 = local_c8 + -1;
      iVar11 = *(int *)(local_c8 * 4);
      local_c8 = lVar24;
      if (extraout_AL_00 != '\0') {
        country_core::resources::stairs::stairs_state::StairsState::graph();
        indexmap::map::IndexMap::
        get<country_core::resources::walls::decorator::DecoratorId,alloc::vec::Vec<tuple_<country_core::resources::walls::decorator::DecoratorId,petgraph::graphmap::CompactDirection>,alloc::alloc::Global>,std::hash::random::RandomState,country_core::resources::walls::decorator::DecoratorId>
                  ();
        local_1d8 = iVar11;
        if (extraout_RAX_06 == 0) {
          piVar19 = (int *)&DAT_00000004;
          piVar27 = (int *)&DAT_00000004;
        }
        else {
          piVar27 = *(int **)(extraout_RAX_06 + 8);
          piVar19 = piVar27 + *(longlong *)(extraout_RAX_06 + 0x10) * 2;
        }
        while (piVar27 != piVar19) {
          local_1d8 = *piVar27;
          country_core::resources::stairs::stairs_state::StairsState::graph();
          indexmap::map::IndexMap::
          get<country_core::resources::walls::decorator::DecoratorId,alloc::vec::Vec<tuple_<country_core::resources::walls::decorator::DecoratorId,petgraph::graphmap::CompactDirection>,alloc::alloc::Global>,std::hash::random::RandomState,country_core::resources::walls::decorator::DecoratorId>
                    ();
          piVar27 = piVar27 + 2;
          if ((((extraout_RAX_07 != 0) && (*(longlong *)(extraout_RAX_07 + 0x10) != 0)) &&
              (*(longlong *)(extraout_RAX_07 + 0x10) * 8 - 8U < 8)) &&
             (country_core::resources::walls::decorator_archivist::DecoratorArchivist::
              is_decorator_cull_allowed(), extraout_AL_01 != '\0')) {
            if (local_e0 == 0) {
              alloc::raw_vec::RawVec::
              grow_one<country_core::render::sparse_instance_buffer::ChunkId,alloc::alloc::Global>()
              ;
            }
            *(int *)(local_e0 * 4 + 4) = local_1d8;
            local_e0 = local_e0 + 1;
            if (local_c8 == 0) {
              alloc::raw_vec::RawVec::
              grow_one<country_core::render::sparse_instance_buffer::ChunkId,alloc::alloc::Global>()
              ;
            }
            *(int *)(local_c8 * 4 + 4) = local_1d8;
            local_c8 = local_c8 + 1;
          }
        }
      }
      *puVar8 = uVar6;
      country_core::resources::stairs::stairs_state::StairsState::remove_node();
      country_core::resources::stairs::stairs_state::StairsState::endpoint();
      if ((local_1d8 != 3) && (local_1c8 == iVar11)) {
        *puVar8 = uVar6;
        country_core::resources::stairs::stairs_state::StairsState::unset_endpoint();
      }
    }
    core::ops::function::FnOnce::
    call_once<std::hash::random::impl_0::new::KEYS::constant_0::closure_env_1,tuple_<enum2_<core::option::Option<ref_mut_<enum2_<core::option::Option<core::cell::Cell<tuple_<u64,u64>_>_>_>_>_>_>_>_>
              ();
    if (extraout_RAX_08 == (longlong *)0x0) {
                    /* WARNING: Subroutine does not return */
      std::thread::local::panic_access_error();
    }
    *extraout_RAX_08 = *extraout_RAX_08 + 1;
    lVar22 = _UNK_142c221f8;
    lVar26 = _UNK_142c221e8;
    puVar21 = anon_335873a3c4aec110b80141a54edc665a_2_llvm_11115642958688891036;
    pauVar20 = (undefined1 (*) [16])**(undefined8 **)stair_colliders;
    lVar24 = (*(undefined8 **)stair_colliders)[3];
    auVar30 = *pauVar20;
    pauVar20 = pauVar20 + 1;
    uVar16 = ~(uint)(ushort)((ushort)(SUB161(auVar30 >> 7,0) & 1) |
                             (ushort)(SUB161(auVar30 >> 0xf,0) & 1) << 1 |
                             (ushort)(SUB161(auVar30 >> 0x17,0) & 1) << 2 |
                             (ushort)(SUB161(auVar30 >> 0x1f,0) & 1) << 3 |
                             (ushort)(SUB161(auVar30 >> 0x27,0) & 1) << 4 |
                             (ushort)(SUB161(auVar30 >> 0x2f,0) & 1) << 5 |
                             (ushort)(SUB161(auVar30 >> 0x37,0) & 1) << 6 |
                             (ushort)(SUB161(auVar30 >> 0x3f,0) & 1) << 7 |
                             (ushort)(SUB161(auVar30 >> 0x47,0) & 1) << 8 |
                             (ushort)(SUB161(auVar30 >> 0x4f,0) & 1) << 9 |
                             (ushort)(SUB161(auVar30 >> 0x57,0) & 1) << 10 |
                             (ushort)(SUB161(auVar30 >> 0x5f,0) & 1) << 0xb |
                             (ushort)(SUB161(auVar30 >> 0x67,0) & 1) << 0xc |
                             (ushort)(SUB161(auVar30 >> 0x6f,0) & 1) << 0xd |
                             (ushort)(SUB161(auVar30 >> 0x77,0) & 1) << 0xe |
                            (ushort)(byte)(auVar30[0xf] >> 7) << 0xf);
    while (lVar24 != 0) {
      if ((short)uVar16 == 0) {
        do {
          auVar30 = *pauVar20;
          uVar23 = (ushort)(SUB161(auVar30 >> 7,0) & 1) |
                   (ushort)(SUB161(auVar30 >> 0xf,0) & 1) << 1 |
                   (ushort)(SUB161(auVar30 >> 0x17,0) & 1) << 2 |
                   (ushort)(SUB161(auVar30 >> 0x1f,0) & 1) << 3 |
                   (ushort)(SUB161(auVar30 >> 0x27,0) & 1) << 4 |
                   (ushort)(SUB161(auVar30 >> 0x2f,0) & 1) << 5 |
                   (ushort)(SUB161(auVar30 >> 0x37,0) & 1) << 6 |
                   (ushort)(SUB161(auVar30 >> 0x3f,0) & 1) << 7 |
                   (ushort)(SUB161(auVar30 >> 0x47,0) & 1) << 8 |
                   (ushort)(SUB161(auVar30 >> 0x4f,0) & 1) << 9 |
                   (ushort)(SUB161(auVar30 >> 0x57,0) & 1) << 10 |
                   (ushort)(SUB161(auVar30 >> 0x5f,0) & 1) << 0xb |
                   (ushort)(SUB161(auVar30 >> 0x67,0) & 1) << 0xc |
                   (ushort)(SUB161(auVar30 >> 0x6f,0) & 1) << 0xd |
                   (ushort)(SUB161(auVar30 >> 0x77,0) & 1) << 0xe |
                   (ushort)(byte)(auVar30[0xf] >> 7) << 0xf;
          pauVar20 = pauVar20 + 1;
        } while (uVar23 == 0xffff);
        uVar16 = ~(uint)uVar23;
        uVar25 = uVar16 & -(uint)uVar23 - 2;
      }
      else {
        uVar25 = uVar16 - 1 & uVar16;
      }
      for (; (uVar16 & 1) == 0; uVar16 = uVar16 >> 1 | 0x80000000) {
      }
      country_core::resources::stairs::stairs_state::StairsState::contains_edge_or_stub();
      lVar24 = lVar24 + -1;
      uVar16 = uVar25;
      if (extraout_AL_02 == '\0') {
        hashbrown::map::HashMap::
        insert<country_core::resources::stairs::stairs_state::StairGraphEdge,tuple_<>,std::hash::random::RandomState,alloc::alloc::Global>
                  ();
      }
    }
    **(undefined4 **)((longlong)stair_colliders + 0x10) =
         *(undefined4 *)((longlong)stair_colliders + 0x1c);
    uVar6 = *(undefined4 *)((longlong)raycast_world + 0x1c);
    puVar8 = *(undefined4 **)((longlong)raycast_world + 0x10);
    *puVar8 = uVar6;
    hashbrown::map::HashMap::
    retain<tuple_<country_core::resources::stairs::stairs_state::StairGraphEdge,u32>,country_core::systems::collision::world::ColliderMeshInstance,std::hash::random::RandomState,alloc::alloc::Global,country_core::systems::collision::world::impl_13::retain_colliders::closure_env_0<tuple_<country_core::resources::stairs::stairs_state::StairGraphEdge,u32>,system_decorator::decorator_visual::stairs::maintain_stair_graph::closure_env_2>_>
              ();
    if (lVar26 != 0) {
      local_100 = lVar26 + (lVar26 * 8 + 0x17U & 0xfffffffffffffff0) + 0x11;
    }
    if (lVar22 != 0) {
      auVar30 = *(undefined1 (*) [16])puVar21;
      uVar7 = *(undefined4 *)((longlong)structural_graph + 0x1c);
      puVar9 = *(undefined4 **)((longlong)structural_graph + 0x10);
      pauVar20 = (undefined1 (*) [16])(puVar21 + 0x10);
      uVar16 = ~(uint)(ushort)((ushort)(SUB161(auVar30 >> 7,0) & 1) |
                               (ushort)(SUB161(auVar30 >> 0xf,0) & 1) << 1 |
                               (ushort)(SUB161(auVar30 >> 0x17,0) & 1) << 2 |
                               (ushort)(SUB161(auVar30 >> 0x1f,0) & 1) << 3 |
                               (ushort)(SUB161(auVar30 >> 0x27,0) & 1) << 4 |
                               (ushort)(SUB161(auVar30 >> 0x2f,0) & 1) << 5 |
                               (ushort)(SUB161(auVar30 >> 0x37,0) & 1) << 6 |
                               (ushort)(SUB161(auVar30 >> 0x3f,0) & 1) << 7 |
                               (ushort)(SUB161(auVar30 >> 0x47,0) & 1) << 8 |
                               (ushort)(SUB161(auVar30 >> 0x4f,0) & 1) << 9 |
                               (ushort)(SUB161(auVar30 >> 0x57,0) & 1) << 10 |
                               (ushort)(SUB161(auVar30 >> 0x5f,0) & 1) << 0xb |
                               (ushort)(SUB161(auVar30 >> 0x67,0) & 1) << 0xc |
                               (ushort)(SUB161(auVar30 >> 0x6f,0) & 1) << 0xd |
                               (ushort)(SUB161(auVar30 >> 0x77,0) & 1) << 0xe |
                              (ushort)(byte)(auVar30[0xf] >> 7) << 0xf);
      do {
        if ((short)uVar16 == 0) {
          do {
            auVar30 = *pauVar20;
            uVar23 = (ushort)(SUB161(auVar30 >> 7,0) & 1) |
                     (ushort)(SUB161(auVar30 >> 0xf,0) & 1) << 1 |
                     (ushort)(SUB161(auVar30 >> 0x17,0) & 1) << 2 |
                     (ushort)(SUB161(auVar30 >> 0x1f,0) & 1) << 3 |
                     (ushort)(SUB161(auVar30 >> 0x27,0) & 1) << 4 |
                     (ushort)(SUB161(auVar30 >> 0x2f,0) & 1) << 5 |
                     (ushort)(SUB161(auVar30 >> 0x37,0) & 1) << 6 |
                     (ushort)(SUB161(auVar30 >> 0x3f,0) & 1) << 7 |
                     (ushort)(SUB161(auVar30 >> 0x47,0) & 1) << 8 |
                     (ushort)(SUB161(auVar30 >> 0x4f,0) & 1) << 9 |
                     (ushort)(SUB161(auVar30 >> 0x57,0) & 1) << 10 |
                     (ushort)(SUB161(auVar30 >> 0x5f,0) & 1) << 0xb |
                     (ushort)(SUB161(auVar30 >> 0x67,0) & 1) << 0xc |
                     (ushort)(SUB161(auVar30 >> 0x6f,0) & 1) << 0xd |
                     (ushort)(SUB161(auVar30 >> 0x77,0) & 1) << 0xe |
                     (ushort)(byte)(auVar30[0xf] >> 7) << 0xf;
            pauVar20 = pauVar20 + 1;
          } while (uVar23 == 0xffff);
          uVar16 = ~(uint)uVar23;
          uVar25 = uVar16 & -(uint)uVar23 - 2;
        }
        else {
          uVar25 = uVar16 - 1 & uVar16;
        }
        for (; (uVar16 & 1) == 0; uVar16 = uVar16 >> 1 | 0x80000000) {
        }
        *puVar8 = uVar6;
        country_core::systems::collision::world::RaycastWorld::
        remove_collider<country_core::resources::stairs::stairs_state::StairGraphEdge>();
        if (local_1b8 != '\x11') {
          LOCK();
          *local_1a0 = *local_1a0 + -1;
          UNLOCK();
          if (*local_1a0 == 0) {
            alloc::sync::Arc::
            drop_slow<country_core::systems::collision::world::RaycastMeshData,alloc::alloc::Global>
                      ();
          }
        }
        *puVar8 = uVar6;
        country_core::systems::collision::world::RaycastWorld::
        remove_collider<country_core::resources::stairs::stairs_state::StairGraphEdge>();
        if (local_1b8 != '\x11') {
          LOCK();
          *local_1a0 = *local_1a0 + -1;
          UNLOCK();
          if (*local_1a0 == 0) {
            alloc::sync::Arc::
            drop_slow<country_core::systems::collision::world::RaycastMeshData,alloc::alloc::Global>
                      ();
          }
        }
        *puVar8 = uVar6;
        country_core::systems::collision::world::RaycastWorld::
        remove_collider<country_core::resources::stairs::stairs_state::StairGraphEdge>();
        if (local_1b8 != '\x11') {
          LOCK();
          *local_1a0 = *local_1a0 + -1;
          UNLOCK();
          if (*local_1a0 == 0) {
            alloc::sync::Arc::
            drop_slow<country_core::systems::collision::world::RaycastMeshData,alloc::alloc::Global>
                      ();
          }
        }
        *puVar9 = uVar7;
        core::hash::BuildHasher::
        hash_one<std::hash::random::RandomState,ref_<country_core::resources::stairs::stairs_state::StairGraphEdge>_>
                  ();
        hashbrown::raw::RawTable::
        remove_entry<tuple_<country_core::resources::stairs::stairs_state::StairGraphEdge,u32>,alloc::alloc::Global,hashbrown::map::equivalent_key::closure_env_0<country_core::resources::stairs::stairs_state::StairGraphEdge,country_core::resources::stairs::stairs_state::StairGraphEdge,u32>_>
                  ();
        lVar22 = lVar22 + -1;
        uVar16 = uVar25;
      } while (lVar22 != 0);
    }
    if ((lVar26 != 0) && (local_100 != 0)) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
    if (local_e0 != 0) {
      uVar6 = *(undefined4 *)((longlong)storage + 0x1c);
      puVar8 = *(undefined4 **)((longlong)storage + 0x10);
      uVar7 = *(undefined4 *)((longlong)archivist + 0x1c);
      puVar9 = *(undefined4 **)((longlong)archivist + 0x10);
      lVar24 = 0;
      do {
        country_core::resources::walls::decorator_storage::DecoratorStorage::try_lookup_storage();
        if (extraout_RAX_09 != 3) {
          *puVar8 = uVar6;
          *puVar9 = uVar7;
          apply_decorator_op::apply_decorator_op();
        }
        lVar24 = lVar24 + 4;
      } while (local_e0 << 2 != lVar24);
    }
    if (bVar15) {
      puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
      if (*extraout_RAX_10 == 1) {
        plVar17 = extraout_RAX_10 + 1;
      }
      else if ((*extraout_RAX_10 != 0) ||
              (std::sys::thread_local::native::lazy::Storage::
               initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                         (), plVar17 = extraout_RAX_11, extraout_RAX_11 == (longlong *)0x0)) {
LAB_140806d03:
                    /* WARNING: Subroutine does not return */
        std::thread::local::panic_access_error();
      }
      if (*plVar17 != 0) {
LAB_140806d0f:
                    /* WARNING: Subroutine does not return */
        core::cell::panic_already_borrowed();
      }
      *plVar17 = -1;
      puffin::ThreadProfiler::end_scope();
      *plVar17 = *plVar17 + 1;
    }
  }
  return;
}

