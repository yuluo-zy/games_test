// name: construct_fences
// addr: 0x140765830
// size: 2000
// signature: void __cdecl construct_fences(void)


// WARNING: Removing unreachable block (ram,0x000140765e9c)
// WARNING: Removing unreachable block (ram,0x000140765eaf)

void __cdecl system_wall_constructor::construct_fences::construct_fences(void)

{
  byte *pbVar1;
  char *pcVar2;
  byte bVar3;
  byte bVar4;
  ulonglong *puVar5;
  longlong lVar6;
  uint uVar7;
  uint uVar8;
  int iVar9;
  int iVar10;
  int iVar11;
  char extraout_AL;
  uint uVar12;
  int iVar13;
  longlong extraout_RAX;
  longlong extraout_RAX_00;
  longlong *extraout_RAX_01;
  undefined8 *extraout_RAX_02;
  longlong lVar14;
  longlong lVar15;
  longlong lVar16;
  ulonglong extraout_RAX_03;
  longlong *extraout_RAX_04;
  longlong *extraout_RAX_05;
  longlong *extraout_RAX_06;
  undefined8 *in_RCX;
  ulonglong uVar17;
  longlong lVar18;
  longlong *plVar19;
  longlong *in_RDX;
  ulonglong uVar20;
  longlong lVar21;
  longlong lVar22;
  longlong lVar23;
  undefined *puVar24;
  ulonglong uVar25;
  longlong lVar26;
  ushort uVar27;
  ulonglong uVar28;
  bool bVar29;
  byte bVar30;
  char cVar31;
  char cVar33;
  char cVar34;
  char cVar35;
  undefined1 auVar32 [16];
  undefined1 auVar36 [16];
  undefined1 auVar37 [16];
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
  longlong local_150;
  
  plVar19 = in_RDX;
  puffin::are_scopes_on();
  if (extraout_AL != '\0') {
    core::str::impl$0::rfind<ref_<str_>_>();
    if (extraout_RAX == 1) {
      if (plVar19 != (longlong *)0x0) {
        if (&DAT_0000003a < plVar19) {
          if (plVar19 != (longlong *)0x3b) goto LAB_1407658b3;
        }
        else if ((char)plVar19[0x2854fb6e] < -0x40) {
LAB_1407658b3:
                    // WARNING: Subroutine does not return
          core::str::slice_error_fail();
        }
      }
      core::str::impl$0::rfind<ref_<str_>_>();
      if ((extraout_RAX_00 == 1) && (plVar19 != (longlong *)0xfffffffffffffffe)) {
        if ((undefined1 *)((longlong)plVar19 + 2) < (undefined1 *)0x3b) {
          if (*(char *)((longlong)plVar19 + 0x142a7db72) < -0x40) {
LAB_140765900:
                    // WARNING: Subroutine does not return
            core::str::slice_error_fail();
          }
        }
        else if ((undefined1 *)((longlong)plVar19 + 2) != (undefined1 *)0x3b) goto LAB_140765900;
      }
    }
    puVar24 = &DAT_142a7dcab;
    do {
      bVar30 = puVar24[-1];
      uVar12 = (uint)(char)bVar30;
      if ((char)bVar30 < '\0') {
        bVar3 = puVar24[-2];
        if ((char)bVar3 < -0x40) {
          bVar4 = puVar24[-3];
          if ((char)bVar4 < -0x40) {
            pbVar1 = puVar24 + -4;
            puVar24 = puVar24 + -4;
            uVar12 = bVar4 & 0x3f | (*pbVar1 & 7) << 6;
          }
          else {
            puVar24 = puVar24 + -3;
            uVar12 = bVar4 & 0xf;
          }
          uVar12 = bVar3 & 0x3f | uVar12 << 6;
        }
        else {
          puVar24 = puVar24 + -2;
          uVar12 = bVar3 & 0x1f;
        }
        uVar12 = (uint)(bVar30 & 0x3f) | uVar12 << 6;
        if (uVar12 != 0x5c) goto LAB_1407659c4;
LAB_1407659d5:
        if (puVar24 == &UNK_142a7dc6f) break;
        if (puVar24 + -0x142a7dc6f < (undefined *)0x3b) {
          if (-0x41 < (char)puVar24[1]) break;
        }
        else if (puVar24 + -0x142a7dc6f == (undefined *)0x3b) break;
                    // WARNING: Subroutine does not return
        core::str::slice_error_fail();
      }
      puVar24 = puVar24 + -1;
      if (uVar12 == 0x5c) goto LAB_1407659d5;
LAB_1407659c4:
      if (uVar12 == 0x2f) goto LAB_1407659d5;
    } while (puVar24 != &DAT_142a7dc70);
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_01 == 1) {
      plVar19 = extraout_RAX_01 + 1;
    }
    else if ((*extraout_RAX_01 != 0) ||
            (std::sys::thread_local::native::lazy::Storage::
             initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                       (), plVar19 = extraout_RAX_05, extraout_RAX_05 == (longlong *)0x0)) {
LAB_140765fd8:
                    // WARNING: Subroutine does not return
      std::thread::local::panic_access_error();
    }
    if (*plVar19 != 0) {
LAB_14076600b:
                    // WARNING: Subroutine does not return
      core::cell::panic_already_borrowed();
    }
    *plVar19 = -1;
    puffin::ThreadProfiler::begin_scope();
    *plVar19 = *plVar19 + 1;
  }
  frozen::FrozenMap::
  get<country_core::asset_libraries::ssbo_library::SsboId,alloc::boxed::Box<country_core::asset_libraries::ssbo_library::BufferState,alloc::alloc::Global>,core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<country_core::asset_libraries::ssbo_library::SsboId>_>,country_core::asset_libraries::ssbo_library::SsboId>
            ();
  if (extraout_RAX_02 != (undefined8 *)0x0) {
    if ((extraout_RAX_02[3] != 0x3d) ||
       (iVar13 = memcmp(&anon_11a7c4262c3a602e5b20874ec0ef09d1_0_llvm_11959870551130474958,
                        (void *)extraout_RAX_02[2],0x3d), iVar13 != 0)) {
                    // WARNING: Subroutine does not return
      core::panicking::assert_failed<ref_<str_>,ref_<str_>_>();
    }
    lVar14 = (**(code **)(extraout_RAX_02[1] + 0x18))(*extraout_RAX_02);
    if (lVar14 != 0) {
      puVar5 = (ulonglong *)*in_RCX;
      lVar21 = in_RCX[1];
      uVar20 = *puVar5;
      uVar28 = 0;
      uVar17 = uVar20 - *(ulonglong *)(lVar21 + 0x18);
      if (uVar20 < *(ulonglong *)(lVar21 + 0x18)) {
        uVar17 = uVar28;
      }
      uVar25 = uVar20 - *(ulonglong *)(lVar21 + 0x38);
      if (uVar20 < *(ulonglong *)(lVar21 + 0x38)) {
        uVar25 = uVar28;
      }
      lVar15 = uVar17 * 0x10 + *(longlong *)(lVar21 + 8);
      uVar20 = *(ulonglong *)(lVar21 + 0x10) - uVar17;
      if (*(ulonglong *)(lVar21 + 0x10) < uVar17) {
        lVar15 = 8;
        uVar20 = uVar28;
      }
      lVar18 = uVar25 * 0x10 + *(longlong *)(lVar21 + 0x28);
      uVar17 = *(ulonglong *)(lVar21 + 0x30) - uVar25;
      if (*(ulonglong *)(lVar21 + 0x30) < uVar25) {
        lVar18 = 8;
        uVar17 = uVar28;
      }
      *puVar5 = (*(longlong *)(lVar21 + 0x40) - uVar17) - uVar20;
      lVar22 = uVar20 * 0x10 + lVar15;
      lVar21 = uVar17 * 0x10 + lVar18;
      plVar19 = (longlong *)*in_RDX;
      iVar13 = -(uint)(CONCAT13(unaff_XMM6_Bd,
                                CONCAT12(unaff_XMM6_Bc,CONCAT11(unaff_XMM6_Bb,unaff_XMM6_Ba))) ==
                      CONCAT13(unaff_XMM6_Bd,
                               CONCAT12(unaff_XMM6_Bc,CONCAT11(unaff_XMM6_Bb,unaff_XMM6_Ba))));
      iVar9 = -(uint)(CONCAT13(unaff_XMM6_Bh,
                               CONCAT12(unaff_XMM6_Bg,CONCAT11(unaff_XMM6_Bf,unaff_XMM6_Be))) ==
                     CONCAT13(unaff_XMM6_Bh,
                              CONCAT12(unaff_XMM6_Bg,CONCAT11(unaff_XMM6_Bf,unaff_XMM6_Be))));
      iVar10 = -(uint)(CONCAT13(unaff_XMM6_Bl,
                                CONCAT12(unaff_XMM6_Bk,CONCAT11(unaff_XMM6_Bj,unaff_XMM6_Bi))) ==
                      CONCAT13(unaff_XMM6_Bl,
                               CONCAT12(unaff_XMM6_Bk,CONCAT11(unaff_XMM6_Bj,unaff_XMM6_Bi))));
      iVar11 = -(uint)(CONCAT13(unaff_XMM6_Bp,
                                CONCAT12(unaff_XMM6_Bo,CONCAT11(unaff_XMM6_Bn,unaff_XMM6_Bm))) ==
                      CONCAT13(unaff_XMM6_Bp,
                               CONCAT12(unaff_XMM6_Bo,CONCAT11(unaff_XMM6_Bn,unaff_XMM6_Bm))));
LAB_140765bb3:
      if ((lVar15 == 0) || (lVar15 == lVar22)) {
        if ((lVar18 == 0) || (lVar18 == lVar21)) {
          if (extraout_AL != '\0') {
            puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
            if (*extraout_RAX_04 == 1) {
              plVar19 = extraout_RAX_04 + 1;
            }
            else if ((*extraout_RAX_04 != 0) ||
                    (std::sys::thread_local::native::lazy::Storage::
                     initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                               (), plVar19 = extraout_RAX_06, extraout_RAX_06 == (longlong *)0x0))
            goto LAB_140765fd8;
            if (*plVar19 != 0) goto LAB_14076600b;
            *plVar19 = -1;
            puffin::ThreadProfiler::end_scope();
            *plVar19 = *plVar19 + 1;
          }
          return;
        }
        lVar15 = 0;
        lVar16 = lVar18;
        lVar18 = lVar18 + 0x10;
      }
      else {
        lVar16 = lVar15;
        lVar15 = lVar15 + 0x10;
      }
      *puVar5 = *puVar5 + 1;
      lVar16 = *(longlong *)(lVar16 + 8);
      if (plVar19[3] != 0) {
        core::hash::BuildHasher::
        hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>();
        lVar6 = *plVar19;
        bVar30 = (byte)(extraout_RAX_03 >> 0x39);
        auVar32 = ZEXT216(CONCAT11(bVar30,bVar30));
        auVar32 = pshuflw(auVar32,auVar32,0);
        lVar26 = 0;
        uVar20 = extraout_RAX_03;
        while( true ) {
          uVar20 = plVar19[1] & uVar20;
          pcVar2 = (char *)(lVar6 + uVar20);
          cVar31 = auVar32[0];
          auVar37[0] = -(*pcVar2 == cVar31);
          cVar33 = auVar32[1];
          auVar37[1] = -(pcVar2[1] == cVar33);
          cVar34 = auVar32[2];
          auVar37[2] = -(pcVar2[2] == cVar34);
          cVar35 = auVar32[3];
          auVar37[3] = -(pcVar2[3] == cVar35);
          auVar37[4] = -(pcVar2[4] == cVar31);
          auVar37[5] = -(pcVar2[5] == cVar33);
          auVar37[6] = -(pcVar2[6] == cVar34);
          auVar37[7] = -(pcVar2[7] == cVar35);
          auVar37[8] = -(pcVar2[8] == cVar31);
          auVar37[9] = -(pcVar2[9] == cVar33);
          auVar37[10] = -(pcVar2[10] == cVar34);
          auVar37[0xb] = -(pcVar2[0xb] == cVar35);
          auVar37[0xc] = -(pcVar2[0xc] == cVar31);
          auVar37[0xd] = -(pcVar2[0xd] == cVar33);
          auVar37[0xe] = -(pcVar2[0xe] == cVar34);
          auVar37[0xf] = -(pcVar2[0xf] == cVar35);
          uVar27 = (ushort)(SUB161(auVar37 >> 7,0) & 1) |
                   (ushort)(SUB161(auVar37 >> 0xf,0) & 1) << 1 |
                   (ushort)(SUB161(auVar37 >> 0x17,0) & 1) << 2 |
                   (ushort)(SUB161(auVar37 >> 0x1f,0) & 1) << 3 |
                   (ushort)(SUB161(auVar37 >> 0x27,0) & 1) << 4 |
                   (ushort)(SUB161(auVar37 >> 0x2f,0) & 1) << 5 |
                   (ushort)(SUB161(auVar37 >> 0x37,0) & 1) << 6 |
                   (ushort)(SUB161(auVar37 >> 0x3f,0) & 1) << 7 |
                   (ushort)(SUB161(auVar37 >> 0x47,0) & 1) << 8 |
                   (ushort)(SUB161(auVar37 >> 0x4f,0) & 1) << 9 |
                   (ushort)(SUB161(auVar37 >> 0x57,0) & 1) << 10 |
                   (ushort)(SUB161(auVar37 >> 0x5f,0) & 1) << 0xb |
                   (ushort)(SUB161(auVar37 >> 0x67,0) & 1) << 0xc |
                   (ushort)(SUB161(auVar37 >> 0x6f,0) & 1) << 0xd |
                   (ushort)(SUB161(auVar37 >> 0x77,0) & 1) << 0xe |
                   (ushort)(auVar37[0xf] >> 7) << 0xf;
          uVar12 = (uint)uVar27;
          while (uVar27 != 0) {
            uVar7 = 0;
            for (uVar8 = uVar12; (uVar8 & 1) == 0; uVar8 = uVar8 >> 1 | 0x80000000) {
              uVar7 = uVar7 + 1;
            }
            lVar23 = (uVar7 + uVar20 & plVar19[1]) * -0x2a8;
            if (lVar16 == *(longlong *)(lVar6 + -0x2a8 + lVar23)) {
              if (*(char *)(lVar6 + -10 + lVar23) == '\x01') {
                lVar16 = *(longlong *)(lVar6 + lVar23 + -0x170);
                if (lVar16 == 0) {
                    // WARNING: Subroutine does not return
                  core::option::unwrap_failed();
                }
                LOCK();
                bVar29 = *(char *)(lVar16 + 0x18) == '\0';
                if (bVar29) {
                  *(char *)(lVar16 + 0x18) = '\x01';
                }
                UNLOCK();
                if (!bVar29) {
                  parking_lot::raw_mutex::RawMutex::lock_slow();
                }
                parking_lot_core::parking_lot::deadlock_impl::acquire_resource();
                if ((*(longlong *)(lVar16 + 0x10) * 4 + *(longlong *)(lVar16 + 8) == 4 ||
                     *(longlong *)(lVar16 + 0x10) == 0) && (*(longlong *)(lVar14 + 0x28) == 0)) {
                    // WARNING: Subroutine does not return
                  core::option::unwrap_failed();
                }
                process_segment::
                emit_fences<system_wall_constructor::construct_fences::construct_fences::closure_env_0>
                          ();
                if (local_150 != 0) {
                  std::alloc::__default_lib_allocator::__rust_dealloc();
                }
                parking_lot_core::parking_lot::deadlock_impl::release_resource();
                LOCK();
                bVar29 = *(char *)(lVar16 + 0x18) == '\x01';
                if (bVar29) {
                  *(char *)(lVar16 + 0x18) = '\0';
                }
                UNLOCK();
                if (!bVar29) {
                  parking_lot::raw_mutex::RawMutex::unlock_slow();
                }
              }
              country_core::resources::walls::material_stats::MaterialStats::
              set_fences_segments_count();
              country_core::resources::walls::material_stats::MaterialStats::
              set_fences_segments_seed();
              goto LAB_140765bb3;
            }
            uVar27 = (ushort)(uVar12 - 1) & (ushort)uVar12;
            uVar12 = CONCAT22((short)(uVar12 - 1 >> 0x10),uVar27);
          }
          auVar36[0] = -(*pcVar2 == (char)iVar13);
          auVar36[1] = -(pcVar2[1] == (char)((uint)iVar13 >> 8));
          auVar36[2] = -(pcVar2[2] == (char)((uint)iVar13 >> 0x10));
          auVar36[3] = -(pcVar2[3] == (char)((uint)iVar13 >> 0x18));
          auVar36[4] = -(pcVar2[4] == (char)iVar9);
          auVar36[5] = -(pcVar2[5] == (char)((uint)iVar9 >> 8));
          auVar36[6] = -(pcVar2[6] == (char)((uint)iVar9 >> 0x10));
          auVar36[7] = -(pcVar2[7] == (char)((uint)iVar9 >> 0x18));
          auVar36[8] = -(pcVar2[8] == (char)iVar10);
          auVar36[9] = -(pcVar2[9] == (char)((uint)iVar10 >> 8));
          auVar36[10] = -(pcVar2[10] == (char)((uint)iVar10 >> 0x10));
          auVar36[0xb] = -(pcVar2[0xb] == (char)((uint)iVar10 >> 0x18));
          auVar36[0xc] = -(pcVar2[0xc] == (char)iVar11);
          auVar36[0xd] = -(pcVar2[0xd] == (char)((uint)iVar11 >> 8));
          auVar36[0xe] = -(pcVar2[0xe] == (char)((uint)iVar11 >> 0x10));
          auVar36[0xf] = -(pcVar2[0xf] == (char)((uint)iVar11 >> 0x18));
          if ((((((((((((((((SUB161(auVar36 >> 7,0) & 1) != 0 || (SUB161(auVar36 >> 0xf,0) & 1) != 0
                           ) || (SUB161(auVar36 >> 0x17,0) & 1) != 0) ||
                         (SUB161(auVar36 >> 0x1f,0) & 1) != 0) ||
                        (SUB161(auVar36 >> 0x27,0) & 1) != 0) ||
                       (SUB161(auVar36 >> 0x2f,0) & 1) != 0) || (SUB161(auVar36 >> 0x37,0) & 1) != 0
                      ) || (SUB161(auVar36 >> 0x3f,0) & 1) != 0) ||
                    (SUB161(auVar36 >> 0x47,0) & 1) != 0) || (SUB161(auVar36 >> 0x4f,0) & 1) != 0)
                  || (SUB161(auVar36 >> 0x57,0) & 1) != 0) || (SUB161(auVar36 >> 0x5f,0) & 1) != 0)
                || (SUB161(auVar36 >> 0x67,0) & 1) != 0) || (SUB161(auVar36 >> 0x6f,0) & 1) != 0) ||
              (SUB161(auVar36 >> 0x77,0) & 1) != 0) || auVar36[0xf] < '\0') break;
          uVar20 = uVar20 + lVar26 + 0x10;
          lVar26 = lVar26 + 0x10;
        }
      }
                    // WARNING: Subroutine does not return
      core::option::expect_failed();
    }
  }
                    // WARNING: Subroutine does not return
  core::option::unwrap_failed();
}

