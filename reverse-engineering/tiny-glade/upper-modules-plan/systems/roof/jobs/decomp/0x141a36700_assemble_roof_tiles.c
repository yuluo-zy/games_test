// name=assemble_roof_tiles
// sig=void __cdecl assemble_roof_tiles(void)
// size=2459

void __cdecl system_roof::visual::assemble_roof_tiles::assemble_roof_tiles(void)

{
  byte *pbVar1;
  char *pcVar2;
  byte bVar3;
  byte bVar4;
  undefined4 uVar5;
  ulonglong *puVar6;
  longlong lVar7;
  longlong lVar8;
  longlong lVar9;
  uint uVar10;
  uint uVar11;
  int iVar12;
  int iVar13;
  int iVar14;
  int iVar15;
  bool bVar16;
  char extraout_AL;
  char extraout_AL_00;
  uint uVar17;
  int iVar18;
  longlong extraout_RAX;
  longlong extraout_RAX_00;
  longlong *extraout_RAX_01;
  ulonglong uVar19;
  undefined8 *extraout_RAX_02;
  longlong lVar20;
  longlong lVar21;
  undefined8 *extraout_RAX_03;
  longlong lVar22;
  ulonglong extraout_RAX_04;
  longlong *extraout_RAX_05;
  longlong *extraout_RAX_06;
  longlong *extraout_RAX_07;
  undefined8 *in_RCX;
  longlong *plVar23;
  longlong *in_RDX;
  ulonglong uVar24;
  ushort uVar25;
  undefined *puVar26;
  ulonglong uVar27;
  longlong *in_R9;
  ulonglong uVar28;
  longlong lVar29;
  longlong lVar30;
  longlong *plVar31;
  longlong lVar32;
  longlong lVar33;
  bool bVar34;
  byte bVar35;
  char cVar36;
  char cVar38;
  char cVar39;
  char cVar40;
  undefined1 auVar37 [16];
  undefined1 auVar41 [16];
  undefined1 auVar42 [16];
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
  longlong local_70;
  longlong local_68;
  
  plVar23 = in_RDX;
  puffin::are_scopes_on();
  if (extraout_AL == '\0') {
    bVar16 = false;
  }
  else {
    core::str::impl$0::rfind<ref_<str_>_>();
    if (extraout_RAX == 1) {
      if (plVar23 != (longlong *)0x0) {
        if (&DAT_0000003c < plVar23) {
          if (plVar23 != (longlong *)0x3d) goto LAB_141a3677f;
        }
        else if (*(char *)((longlong)plVar23 + 0x142eba60b) < -0x40) {
LAB_141a3677f:
                    // WARNING: Subroutine does not return
          core::str::slice_error_fail();
        }
      }
      core::str::impl$0::rfind<ref_<str_>_>();
      if ((extraout_RAX_00 == 1) && (plVar23 != (longlong *)0xfffffffffffffffe)) {
        if ((undefined1 *)((longlong)plVar23 + 2) < (undefined1 *)0x3d) {
          if (*(char *)((longlong)plVar23 + 0x142eba60d) < -0x40) {
LAB_141a367d3:
                    // WARNING: Subroutine does not return
            core::str::slice_error_fail();
          }
        }
        else if ((undefined1 *)((longlong)plVar23 + 2) != (undefined1 *)0x3d) goto LAB_141a367d3;
      }
    }
    puVar26 = &DAT_142eba815;
    do {
      bVar35 = puVar26[-1];
      uVar17 = (uint)(char)bVar35;
      if ((char)bVar35 < '\0') {
        bVar3 = puVar26[-2];
        if ((char)bVar3 < -0x40) {
          bVar4 = puVar26[-3];
          if ((char)bVar4 < -0x40) {
            pbVar1 = puVar26 + -4;
            puVar26 = puVar26 + -4;
            uVar17 = bVar4 & 0x3f | (*pbVar1 & 7) << 6;
          }
          else {
            puVar26 = puVar26 + -3;
            uVar17 = bVar4 & 0xf;
          }
          uVar17 = bVar3 & 0x3f | uVar17 << 6;
        }
        else {
          puVar26 = puVar26 + -2;
          uVar17 = bVar3 & 0x1f;
        }
        uVar17 = (uint)(bVar35 & 0x3f) | uVar17 << 6;
        if (uVar17 != 0x5c) goto LAB_141a36894;
LAB_141a368a6:
        if (puVar26 == &UNK_142eba7df) break;
        if (puVar26 + -0x142eba7df < (undefined *)0x35) {
          if (-0x41 < (char)puVar26[1]) break;
        }
        else if (puVar26 + -0x142eba7df == (undefined *)0x35) break;
                    // WARNING: Subroutine does not return
        core::str::slice_error_fail();
      }
      puVar26 = puVar26 + -1;
      if (uVar17 == 0x5c) goto LAB_141a368a6;
LAB_141a36894:
      if (uVar17 == 0x2f) goto LAB_141a368a6;
    } while (puVar26 != &DAT_142eba7e0);
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_01 == 1) {
      plVar23 = extraout_RAX_01 + 1;
    }
    else if ((*extraout_RAX_01 != 0) ||
            (std::sys::thread_local::native::lazy::Storage::
             initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                       (), plVar23 = extraout_RAX_06, extraout_RAX_06 == (longlong *)0x0)) {
LAB_141a36ffa:
                    // WARNING: Subroutine does not return
      std::thread::local::panic_access_error();
    }
    if (*plVar23 != 0) {
LAB_141a37030:
                    // WARNING: Subroutine does not return
      core::cell::panic_already_borrowed();
    }
    *plVar23 = -1;
    puffin::ThreadProfiler::begin_scope();
    *plVar23 = *plVar23 + 1;
    bVar16 = true;
  }
  puVar6 = (ulonglong *)*in_RCX;
  lVar7 = in_RCX[1];
  uVar27 = *puVar6;
  uVar28 = 0;
  uVar19 = uVar27 - *(ulonglong *)(lVar7 + 0x18);
  if (uVar27 < *(ulonglong *)(lVar7 + 0x18)) {
    uVar19 = uVar28;
  }
  uVar24 = uVar27 - *(ulonglong *)(lVar7 + 0x38);
  if (uVar27 < *(ulonglong *)(lVar7 + 0x38)) {
    uVar24 = uVar28;
  }
  local_70 = uVar19 * 0x20 + *(longlong *)(lVar7 + 8);
  uVar27 = *(ulonglong *)(lVar7 + 0x10) - uVar19;
  if (*(ulonglong *)(lVar7 + 0x10) < uVar19) {
    local_70 = 8;
    uVar27 = uVar28;
  }
  local_68 = uVar24 * 0x20 + *(longlong *)(lVar7 + 0x28);
  uVar19 = *(ulonglong *)(lVar7 + 0x30) - uVar24;
  if (*(ulonglong *)(lVar7 + 0x30) < uVar24) {
    local_68 = 8;
    uVar19 = uVar28;
  }
  *puVar6 = (*(longlong *)(lVar7 + 0x40) - uVar19) - uVar27;
  lVar29 = uVar27 * 0x20 + local_70;
  lVar32 = uVar19 * 0x20 + local_68;
  lVar7 = *in_RDX;
  lVar8 = in_RDX[1];
  uVar5 = *(undefined4 *)((longlong)in_RDX + 0x14);
  plVar23 = (longlong *)*in_R9;
  iVar12 = -(uint)(CONCAT13(unaff_XMM6_Bd,
                            CONCAT12(unaff_XMM6_Bc,CONCAT11(unaff_XMM6_Bb,unaff_XMM6_Ba))) ==
                  CONCAT13(unaff_XMM6_Bd,
                           CONCAT12(unaff_XMM6_Bc,CONCAT11(unaff_XMM6_Bb,unaff_XMM6_Ba))));
  iVar13 = -(uint)(CONCAT13(unaff_XMM6_Bh,
                            CONCAT12(unaff_XMM6_Bg,CONCAT11(unaff_XMM6_Bf,unaff_XMM6_Be))) ==
                  CONCAT13(unaff_XMM6_Bh,
                           CONCAT12(unaff_XMM6_Bg,CONCAT11(unaff_XMM6_Bf,unaff_XMM6_Be))));
  iVar14 = -(uint)(CONCAT13(unaff_XMM6_Bl,
                            CONCAT12(unaff_XMM6_Bk,CONCAT11(unaff_XMM6_Bj,unaff_XMM6_Bi))) ==
                  CONCAT13(unaff_XMM6_Bl,
                           CONCAT12(unaff_XMM6_Bk,CONCAT11(unaff_XMM6_Bj,unaff_XMM6_Bi))));
  iVar15 = -(uint)(CONCAT13(unaff_XMM6_Bp,
                            CONCAT12(unaff_XMM6_Bo,CONCAT11(unaff_XMM6_Bn,unaff_XMM6_Bm))) ==
                  CONCAT13(unaff_XMM6_Bp,
                           CONCAT12(unaff_XMM6_Bo,CONCAT11(unaff_XMM6_Bn,unaff_XMM6_Bm))));
  do {
    if ((local_70 == 0) || (local_70 == lVar29)) {
      if ((local_68 == 0) || (local_68 == lVar32)) {
        if (bVar16) {
          puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
          if (*extraout_RAX_05 == 1) {
            plVar23 = extraout_RAX_05 + 1;
          }
          else if ((*extraout_RAX_05 != 0) ||
                  (std::sys::thread_local::native::lazy::Storage::
                   initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                             (), plVar23 = extraout_RAX_07, extraout_RAX_07 == (longlong *)0x0))
          goto LAB_141a36ffa;
          if (*plVar23 != 0) goto LAB_141a37030;
          *plVar23 = -1;
          puffin::ThreadProfiler::end_scope();
          *plVar23 = *plVar23 + 1;
        }
        return;
      }
      local_70 = 0;
      lVar33 = local_68;
      local_68 = local_68 + 0x20;
    }
    else {
      lVar33 = local_70;
      local_70 = local_70 + 0x20;
    }
    *puVar6 = *puVar6 + 1;
    frozen::FrozenMap::
    get<country_core::asset_libraries::ssbo_library::SsboId,alloc::boxed::Box<country_core::asset_libraries::ssbo_library::BufferState,alloc::alloc::Global>,core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<country_core::asset_libraries::ssbo_library::SsboId>_>,country_core::asset_libraries::ssbo_library::SsboId>
              ();
    if (extraout_RAX_02 == (undefined8 *)0x0) {
LAB_141a3703c:
                    // WARNING: Subroutine does not return
      core::option::unwrap_failed();
    }
    if ((extraout_RAX_02[3] != 0x35) ||
       (iVar18 = memcmp(&anon_c4dc3b5cd97938b42123e838db8de09d_1_llvm_3856994597783109560,
                        (void *)extraout_RAX_02[2],0x35), iVar18 != 0)) {
                    // WARNING: Subroutine does not return
      core::panicking::assert_failed<ref_<str_>,ref_<str_>_>();
    }
    lVar20 = (**(code **)(extraout_RAX_02[1] + 0x18))(*extraout_RAX_02);
    if (lVar20 == 0) goto LAB_141a3703c;
    uVar27 = *(ulonglong *)(lVar33 + 8) & 0xffffffff;
    if ((*(ulonglong *)(lVar8 + 0x10) <= uVar27) ||
       (*(int *)(*(longlong *)(lVar8 + 8) + uVar27 * 0x14) !=
        (int)(*(ulonglong *)(lVar33 + 8) >> 0x20))) {
LAB_141a37077:
                    // WARNING: Subroutine does not return
      core::result::unwrap_failed();
    }
    lVar33 = *(longlong *)(lVar8 + 8) + uVar27 * 0x14;
    uVar27 = (ulonglong)*(uint *)(lVar33 + 4);
    if ((uVar27 == 0xffffffff) ||
       ((*(ulonglong *)(lVar7 + 0x38) <= uVar27 ||
        ((*(ulonglong *)(*(longlong *)(lVar7 + 0x28) + (ulonglong)(*(uint *)(lVar33 + 4) >> 6) * 8)
          >> (uVar27 & 0x3f) & 1) == 0)))) goto LAB_141a37077;
    uVar27 = (ulonglong)*(uint *)(lVar33 + 0xc);
    lVar30 = *(longlong *)(lVar8 + 0x1a8);
    lVar9 = *(longlong *)(lVar30 + 0x18 + uVar27 * 0x48);
    lVar21 = *(longlong *)(lVar30 + 0x38 + uVar27 * 0x48);
    if ((*(ulonglong *)(lVar7 + 0x118) < *(ulonglong *)(lVar30 + 0x40 + uVar27 * 0x48)) &&
       (uVar27 = *(ulonglong *)(lVar21 + *(ulonglong *)(lVar7 + 0x118) * 8), uVar27 != 0)) {
      lVar30 = *(longlong *)(lVar9 + 0x10 + ~uVar27 * 0x30);
    }
    else {
      lVar30 = 0;
    }
    uVar27 = (ulonglong)*(uint *)(lVar33 + 0x10);
    lVar21 = ~*(ulonglong *)(lVar21 + *(longlong *)(lVar7 + 0x110) * 8) * 0x30;
    lVar33 = *(longlong *)(lVar9 + 0x10 + lVar21);
    lVar9 = *(longlong *)(lVar9 + 0x28 + lVar21);
    plVar31 = (longlong *)(lVar33 + uVar27 * 0x10);
    if (*(longlong *)(lVar33 + uVar27 * 0x10) == 0) {
      std::alloc::__default_lib_allocator::__rust_alloc();
      if (extraout_RAX_03 == (undefined8 *)0x0) {
                    // WARNING: Subroutine does not return
        alloc::alloc::handle_alloc_error();
      }
      *extraout_RAX_03 = 0;
      extraout_RAX_03[1] = 4;
      extraout_RAX_03[2] = 0;
      *(undefined1 *)(extraout_RAX_03 + 3) = 0;
      lVar33 = *(longlong *)(lVar20 + 0x60);
      do {
        lVar21 = *(longlong *)(lVar33 + 8);
        while (lVar21 != -1) {
          if (lVar21 < 0) {
                    // WARNING: Subroutine does not return
            alloc::sync::impl$21::downgrade::panic_cold_display<ref_<str_>_>();
          }
          LOCK();
          lVar22 = *(longlong *)(lVar33 + 8);
          bVar34 = lVar21 == lVar22;
          if (bVar34) {
            *(longlong *)(lVar33 + 8) = lVar21 + 1;
            lVar22 = lVar21;
          }
          UNLOCK();
          lVar21 = lVar22;
          if (bVar34) {
            *(undefined4 *)(lVar9 + uVar27 * 4) = uVar5;
            if (*plVar31 != 0) {
              core::ptr::
              drop_in_place<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo>_>
                        ();
            }
            *plVar31 = lVar33;
            plVar31[1] = (longlong)extraout_RAX_03;
            goto LAB_141a36cc8;
          }
        }
      } while( true );
    }
LAB_141a36cc8:
    *(undefined4 *)(lVar9 + uVar27 * 4) = uVar5;
    if (*plVar31 == 0) {
                    // WARNING: Subroutine does not return
      core::option::unwrap_failed();
    }
    lVar33 = plVar31[1];
    if (lVar33 == 0) {
                    // WARNING: Subroutine does not return
      core::option::unwrap_failed();
    }
    LOCK();
    bVar34 = *(char *)(lVar33 + 0x18) == '\0';
    if (bVar34) {
      *(char *)(lVar33 + 0x18) = '\x01';
    }
    UNLOCK();
    if (!bVar34) {
      parking_lot::raw_mutex::RawMutex::lock_slow();
    }
    parking_lot_core::parking_lot::deadlock_impl::acquire_resource();
    if ((*(longlong *)(lVar33 + 0x10) * 4 + *(longlong *)(lVar33 + 8) == 4 ||
         *(longlong *)(lVar33 + 0x10) == 0) && (*(longlong *)(lVar20 + 0x28) == 0)) {
                    // WARNING: Subroutine does not return
      core::option::unwrap_failed();
    }
    country_core::render::sparse_instance_buffer::SparseInstanceBufferWriter::
    clear<country_core::resources::roof_tile_ssbo::RoofTileSsbo>();
    plVar31 = (longlong *)(lVar30 + uVar27 * 0x58);
    country_core::resources::roofs::roof_shape::Roof::is_flat();
    if (extraout_AL_00 == '\0') {
      if (plVar23[3] != 0) {
        core::hash::BuildHasher::
        hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>();
        bVar35 = (byte)(extraout_RAX_04 >> 0x39);
        auVar37 = ZEXT216(CONCAT11(bVar35,bVar35));
        auVar37 = pshuflw(auVar37,auVar37,0);
        lVar20 = 0;
        uVar27 = extraout_RAX_04;
        while( true ) {
          uVar27 = plVar23[1] & uVar27;
          pcVar2 = (char *)(*plVar23 + uVar27);
          cVar36 = auVar37[0];
          auVar42[0] = -(*pcVar2 == cVar36);
          cVar38 = auVar37[1];
          auVar42[1] = -(pcVar2[1] == cVar38);
          cVar39 = auVar37[2];
          auVar42[2] = -(pcVar2[2] == cVar39);
          cVar40 = auVar37[3];
          auVar42[3] = -(pcVar2[3] == cVar40);
          auVar42[4] = -(pcVar2[4] == cVar36);
          auVar42[5] = -(pcVar2[5] == cVar38);
          auVar42[6] = -(pcVar2[6] == cVar39);
          auVar42[7] = -(pcVar2[7] == cVar40);
          auVar42[8] = -(pcVar2[8] == cVar36);
          auVar42[9] = -(pcVar2[9] == cVar38);
          auVar42[10] = -(pcVar2[10] == cVar39);
          auVar42[0xb] = -(pcVar2[0xb] == cVar40);
          auVar42[0xc] = -(pcVar2[0xc] == cVar36);
          auVar42[0xd] = -(pcVar2[0xd] == cVar38);
          auVar42[0xe] = -(pcVar2[0xe] == cVar39);
          auVar42[0xf] = -(pcVar2[0xf] == cVar40);
          uVar25 = (ushort)(SUB161(auVar42 >> 7,0) & 1) |
                   (ushort)(SUB161(auVar42 >> 0xf,0) & 1) << 1 |
                   (ushort)(SUB161(auVar42 >> 0x17,0) & 1) << 2 |
                   (ushort)(SUB161(auVar42 >> 0x1f,0) & 1) << 3 |
                   (ushort)(SUB161(auVar42 >> 0x27,0) & 1) << 4 |
                   (ushort)(SUB161(auVar42 >> 0x2f,0) & 1) << 5 |
                   (ushort)(SUB161(auVar42 >> 0x37,0) & 1) << 6 |
                   (ushort)(SUB161(auVar42 >> 0x3f,0) & 1) << 7 |
                   (ushort)(SUB161(auVar42 >> 0x47,0) & 1) << 8 |
                   (ushort)(SUB161(auVar42 >> 0x4f,0) & 1) << 9 |
                   (ushort)(SUB161(auVar42 >> 0x57,0) & 1) << 10 |
                   (ushort)(SUB161(auVar42 >> 0x5f,0) & 1) << 0xb |
                   (ushort)(SUB161(auVar42 >> 0x67,0) & 1) << 0xc |
                   (ushort)(SUB161(auVar42 >> 0x6f,0) & 1) << 0xd |
                   (ushort)(SUB161(auVar42 >> 0x77,0) & 1) << 0xe |
                   (ushort)(auVar42[0xf] >> 7) << 0xf;
          uVar17 = (uint)uVar25;
          if (uVar25 != 0) {
            do {
              uVar10 = 0;
              for (uVar11 = uVar17; (uVar11 & 1) == 0; uVar11 = uVar11 >> 1 | 0x80000000) {
                uVar10 = uVar10 + 1;
              }
              if (*plVar31 ==
                  *(longlong *)(*plVar23 + -0x20 + (uVar10 + uVar27 & plVar23[1]) * -0x20)) {
                bVar35 = *(byte *)(plVar31 + 1);
                goto joined_r0x000141a36ee6;
              }
              uVar25 = (ushort)(uVar17 - 1) & (ushort)uVar17;
              uVar17 = CONCAT22((short)(uVar17 - 1 >> 0x10),uVar25);
            } while (uVar25 != 0);
          }
          auVar41[0] = -(*pcVar2 == (char)iVar12);
          auVar41[1] = -(pcVar2[1] == (char)((uint)iVar12 >> 8));
          auVar41[2] = -(pcVar2[2] == (char)((uint)iVar12 >> 0x10));
          auVar41[3] = -(pcVar2[3] == (char)((uint)iVar12 >> 0x18));
          auVar41[4] = -(pcVar2[4] == (char)iVar13);
          auVar41[5] = -(pcVar2[5] == (char)((uint)iVar13 >> 8));
          auVar41[6] = -(pcVar2[6] == (char)((uint)iVar13 >> 0x10));
          auVar41[7] = -(pcVar2[7] == (char)((uint)iVar13 >> 0x18));
          auVar41[8] = -(pcVar2[8] == (char)iVar14);
          auVar41[9] = -(pcVar2[9] == (char)((uint)iVar14 >> 8));
          auVar41[10] = -(pcVar2[10] == (char)((uint)iVar14 >> 0x10));
          auVar41[0xb] = -(pcVar2[0xb] == (char)((uint)iVar14 >> 0x18));
          auVar41[0xc] = -(pcVar2[0xc] == (char)iVar15);
          auVar41[0xd] = -(pcVar2[0xd] == (char)((uint)iVar15 >> 8));
          auVar41[0xe] = -(pcVar2[0xe] == (char)((uint)iVar15 >> 0x10));
          auVar41[0xf] = -(pcVar2[0xf] == (char)((uint)iVar15 >> 0x18));
          if ((((((((((((((((SUB161(auVar41 >> 7,0) & 1) != 0 || (SUB161(auVar41 >> 0xf,0) & 1) != 0
                           ) || (SUB161(auVar41 >> 0x17,0) & 1) != 0) ||
                         (SUB161(auVar41 >> 0x1f,0) & 1) != 0) ||
                        (SUB161(auVar41 >> 0x27,0) & 1) != 0) ||
                       (SUB161(auVar41 >> 0x2f,0) & 1) != 0) || (SUB161(auVar41 >> 0x37,0) & 1) != 0
                      ) || (SUB161(auVar41 >> 0x3f,0) & 1) != 0) ||
                    (SUB161(auVar41 >> 0x47,0) & 1) != 0) || (SUB161(auVar41 >> 0x4f,0) & 1) != 0)
                  || (SUB161(auVar41 >> 0x57,0) & 1) != 0) || (SUB161(auVar41 >> 0x5f,0) & 1) != 0)
                || (SUB161(auVar41 >> 0x67,0) & 1) != 0) || (SUB161(auVar41 >> 0x6f,0) & 1) != 0) ||
              (SUB161(auVar41 >> 0x77,0) & 1) != 0) || auVar41[0xf] < '\0') break;
          uVar27 = uVar27 + lVar20 + 0x10;
          lVar20 = lVar20 + 0x10;
        }
      }
      bVar35 = *(byte *)(plVar31 + 1);
joined_r0x000141a36ee6:
      if ((bVar35 & 1) == 0) {
        system_roof::utils::assemble_circular_roof::
        assemble_circular_roof<system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure_env_0>
                  ();
      }
      else {
        system_roof::utils::assemble_rectangular_roof::
        assemble_rectangular_roof<system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure_env_0>
                  ();
      }
      parking_lot_core::parking_lot::deadlock_impl::release_resource();
      LOCK();
      bVar34 = *(char *)(lVar33 + 0x18) == '\x01';
      if (bVar34) {
        *(char *)(lVar33 + 0x18) = '\0';
      }
      UNLOCK();
      if (!bVar34) {
        parking_lot::raw_mutex::RawMutex::unlock_slow();
      }
    }
    else {
      parking_lot_core::parking_lot::deadlock_impl::release_resource();
      LOCK();
      bVar34 = *(char *)(lVar33 + 0x18) == '\x01';
      if (bVar34) {
        *(char *)(lVar33 + 0x18) = '\0';
      }
      UNLOCK();
      if (!bVar34) {
        parking_lot::raw_mutex::RawMutex::unlock_slow();
      }
    }
  } while( true );
}

