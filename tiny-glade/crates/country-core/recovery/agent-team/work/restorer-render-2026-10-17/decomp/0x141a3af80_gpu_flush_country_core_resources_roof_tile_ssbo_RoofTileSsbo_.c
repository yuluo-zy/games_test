// 0x141a3af80 gpu_flush<country_core::resources::roof_tile_ssbo::RoofTileSsbo> size=3208

void __cdecl
country_core::render::sparse_instance_buffer::SparseInstanceBuffer::
gpu_flush<country_core::resources::roof_tile_ssbo::RoofTileSsbo>(void)

{
  byte *pbVar1;
  ulonglong *puVar2;
  undefined4 *puVar3;
  byte bVar4;
  byte bVar5;
  longlong lVar6;
  byte bVar7;
  code *pcVar8;
  int iVar9;
  longlong *plVar10;
  undefined4 uVar11;
  undefined4 uVar12;
  undefined4 uVar13;
  undefined4 uVar14;
  undefined4 uVar15;
  undefined4 uVar16;
  undefined4 uVar17;
  longlong *plVar18;
  char extraout_AL;
  char extraout_AL_00;
  char extraout_AL_01;
  uint uVar19;
  longlong **pplVar20;
  longlong extraout_RAX;
  longlong extraout_RAX_00;
  longlong *extraout_RAX_01;
  longlong *extraout_RAX_02;
  longlong *extraout_RAX_03;
  longlong *extraout_RAX_04;
  longlong *extraout_RAX_05;
  longlong *extraout_RAX_06;
  longlong *extraout_RAX_07;
  longlong *extraout_RAX_08;
  longlong *extraout_RAX_09;
  longlong *in_RCX;
  longlong *plVar21;
  uint uVar22;
  longlong lVar23;
  longlong *plVar24;
  ulonglong uVar25;
  undefined4 *puVar26;
  longlong *plVar27;
  undefined *puVar28;
  longlong *plVar29;
  longlong lVar30;
  ulonglong uVar31;
  uint uVar32;
  bool bVar33;
  longlong local_160;
  longlong *local_158;
  longlong *local_d8;
  longlong *local_d0;
  longlong *local_c8;
  ulonglong local_c0;
  undefined8 local_b8;
  longlong *local_b0;
  longlong **local_a0;
  code *local_98;
  longlong *plStack_90;
  longlong local_88;
  longlong local_80;
  longlong *local_70;
  longlong *local_68;
  longlong *local_60;
  longlong *local_58;
  undefined1 local_49;
  undefined8 local_48;
  
  local_48 = 0xfffffffffffffffe;
  plVar21 = (longlong *)in_RCX[0xc];
  LOCK();
  lVar23 = *plVar21;
  *plVar21 = *plVar21 + 1;
  UNLOCK();
  if (*plVar21 == 0 || SCARRY8(lVar23,1) != *plVar21 < 0) {
                    // WARNING: Does not return
    pcVar8 = (code *)invalidInstructionException();
    (*pcVar8)();
  }
  LOCK();
  bVar33 = (char)plVar21[2] == '\0';
  if (bVar33) {
    *(char *)(plVar21 + 2) = '\x01';
  }
  UNLOCK();
  local_b0 = plVar21;
  if (!bVar33) {
    parking_lot::raw_mutex::RawMutex::lock_slow();
  }
  parking_lot_core::parking_lot::deadlock_impl::acquire_resource();
  plStack_90 = plVar21 + 3;
  local_88 = plVar21[5];
  plVar21[5] = 0;
  local_98 = (code *)((longlong *)plVar21[4] + local_88);
  local_80 = 0;
  plVar27 = (longlong *)((ulonglong)in_RCX | 1);
  plVar29 = in_RCX + 1;
  pplVar20 = (longlong **)plVar21[4];
  local_70 = plVar29;
  local_68 = plVar21 + 2;
  local_58 = in_RCX;
  while (plVar24 = local_58, lVar23 = local_80, plVar21 = plStack_90,
        pplVar20 != (longlong **)local_98) {
    local_a0 = pplVar20 + 1;
    plVar21 = *pplVar20;
    if (plVar21 == (longlong *)0x0) {
                    // WARNING: Subroutine does not return
      core::option::unwrap_failed();
    }
    lVar23 = plVar21[2];
    local_60 = plVar21;
    if (lVar23 != 0) {
      lVar6 = plVar21[1];
      lVar30 = 0;
      do {
        uVar19 = *(uint *)(lVar6 + lVar30);
        if ((ulonglong)local_58[5] <= (ulonglong)uVar19) {
                    // WARNING: Subroutine does not return
          core::panicking::panic_bounds_check();
        }
        *(undefined1 *)(local_58[4] + (ulonglong)uVar19) = 0;
        if ((ulonglong)local_58[7] <= (ulonglong)(uVar19 >> 6)) {
                    // WARNING: Subroutine does not return
          core::panicking::panic_bounds_check();
        }
        bVar7 = (byte)uVar19 & 0x3f;
        LOCK();
        puVar2 = (ulonglong *)(local_58[6] + (ulonglong)(uVar19 >> 6) * 8);
        *puVar2 = *puVar2 & (-2L << bVar7 | 0xfffffffffffffffeU >> 0x40 - bVar7);
        UNLOCK();
        lVar30 = lVar30 + 4;
      } while (lVar23 << 2 != lVar30);
    }
    LOCK();
    lVar23 = *local_58;
    if (lVar23 == 0) {
      *local_58 = 8;
    }
    UNLOCK();
    if (lVar23 != 0) {
      parking_lot::raw_rwlock::RawRwLock::lock_exclusive_slow();
    }
    parking_lot_core::parking_lot::deadlock_impl::acquire_resource();
    parking_lot_core::parking_lot::deadlock_impl::acquire_resource();
    plVar21 = local_58;
    local_d8 = (longlong *)local_60[1];
    local_c0 = local_60[2];
    local_60[2] = 0;
    local_d0 = (longlong *)((longlong)local_d8 + local_c0 * 4);
    local_c8 = local_60;
    local_b8 = 0;
    lVar23 = local_58[3];
    if ((ulonglong)(local_58[1] - lVar23) < local_c0) {
      local_49 = 1;
      alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
      lVar23 = plVar21[3];
    }
    lVar6 = local_58[2];
    if (local_d8 != local_d0) {
      uVar31 = (longlong)local_d0 + (-4 - (longlong)local_d8);
      plVar21 = local_d0;
      if ((0x3b < uVar31) &&
         ((plVar29 = (longlong *)(lVar6 + lVar23 * 4), local_d0 <= plVar29 ||
          (uVar25 = (longlong)local_d0 + (-4 - (longlong)local_d8) & 0xfffffffffffffffc,
          (longlong)plVar29 + uVar25 + 4 <= (longlong)local_d0 + (-4 - uVar25))))) {
        uVar31 = (uVar31 >> 2) + 1;
        uVar25 = uVar31 & 0xfffffffffffffff8;
        puVar26 = (undefined4 *)(lVar6 + lVar23 * 4 + 0x10);
        lVar23 = lVar23 + uVar25;
        in_RCX = (longlong *)0x0;
        do {
          puVar3 = (undefined4 *)((longlong)local_d0 + (longlong)in_RCX * 4 + -0x20);
          uVar11 = *puVar3;
          uVar12 = puVar3[1];
          uVar13 = puVar3[2];
          uVar14 = puVar3[3];
          puVar3 = (undefined4 *)((longlong)local_d0 + (longlong)in_RCX * 4 + -0x10);
          uVar15 = *puVar3;
          uVar16 = puVar3[1];
          uVar17 = puVar3[2];
          puVar26[-4] = puVar3[3];
          puVar26[-3] = uVar17;
          puVar26[-2] = uVar16;
          puVar26[-1] = uVar15;
          *puVar26 = uVar14;
          puVar26[1] = uVar13;
          puVar26[2] = uVar12;
          puVar26[3] = uVar11;
          puVar26 = puVar26 + 8;
          in_RCX = in_RCX + -1;
        } while (-(longlong)in_RCX != uVar25);
        plVar21 = (longlong *)((longlong)local_d0 + uVar25 * -4);
        if (uVar31 == uVar25) goto LAB_141a3b288;
      }
      do {
        plVar29 = (longlong *)((longlong)plVar21 + -4);
        *(undefined4 *)(lVar6 + lVar23 * 4) = *(undefined4 *)((longlong)plVar21 + -4);
        lVar23 = lVar23 + 1;
        plVar21 = plVar29;
      } while (plVar29 != local_d8);
    }
LAB_141a3b288:
    local_58[3] = lVar23;
    local_49 = 0;
    core::ptr::
    drop_in_place<alloc::vec::drain::Drain<rhapsody::rg::resource::ResourceHandle,alloc::alloc::Global>_>
              ();
    parking_lot_core::parking_lot::deadlock_impl::release_resource();
    parking_lot_core::parking_lot::deadlock_impl::release_resource();
    LOCK();
    lVar23 = *local_58;
    if (lVar23 == 8) {
      *local_58 = 0;
    }
    UNLOCK();
    if (lVar23 != 8) {
      parking_lot::raw_rwlock::RawRwLock::unlock_exclusive_slow();
    }
    if (*local_60 != 0) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
    plVar29 = (longlong *)&DAT_00000020;
    std::alloc::__default_lib_allocator::__rust_dealloc();
    pplVar20 = local_a0;
  }
  if (local_80 != 0) {
    lVar6 = plStack_90[2];
    if (local_88 != lVar6) {
      plVar29 = (longlong *)(plStack_90[1] + local_88 * 8);
      memmove((void *)(plStack_90[1] + lVar6 * 8),plVar29,local_80 * 8);
    }
    in_RCX = (longlong *)(lVar6 + lVar23);
    plVar21[2] = (longlong)in_RCX;
    plVar27 = plVar21;
  }
  plVar21 = local_68;
  parking_lot_core::parking_lot::deadlock_impl::release_resource();
  LOCK();
  bVar33 = (char)*plVar21 == '\x01';
  if (bVar33) {
    *(char *)plVar21 = '\0';
  }
  UNLOCK();
  if (!bVar33) {
    plVar29 = (longlong *)0x0;
    parking_lot::raw_mutex::RawMutex::unlock_slow();
  }
  LOCK();
  *local_b0 = *local_b0 + -1;
  UNLOCK();
  if (*local_b0 == 0) {
    alloc::sync::Arc::
    drop_slow<country_core::components::transient_mesh::TransientMeshRegistry,alloc::alloc::Global>
              ();
  }
  puffin::are_scopes_on();
  if (extraout_AL == '\0') {
    local_b0 = (longlong *)0x0;
  }
  else {
    core::str::impl$0::rfind<ref_<str_>_>();
    in_RCX = (longlong *)&DAT_142ebb758;
    if (extraout_RAX == 1) {
      if (plVar29 != (longlong *)0x0) {
        if (&DAT_0000004f < plVar29) {
          if (plVar29 != (longlong *)&DAT_00000050) goto LAB_141a3b40b;
        }
        else if ((char)plVar29[0x285d76eb] < -0x40) {
LAB_141a3b40b:
                    // WARNING: Subroutine does not return
          core::str::slice_error_fail();
        }
      }
      core::str::impl$0::rfind<ref_<str_>_>();
      if (extraout_RAX_00 == 1) {
        if (plVar29 != (longlong *)0xfffffffffffffffe) {
          if ((longlong)plVar29 + 2U < 0x50) {
            if (*(char *)((longlong)plVar29 + 0x142ebb75a) < -0x40) {
LAB_141a3b45f:
                    // WARNING: Subroutine does not return
              core::str::slice_error_fail();
            }
          }
          else if ((longlong)plVar29 + 2U != 0x50) goto LAB_141a3b45f;
        }
        in_RCX = (longlong *)((longlong)plVar29 + 0x142ebb75a);
      }
    }
    puVar28 = &DAT_142ebb498;
    do {
      bVar7 = puVar28[-1];
      uVar19 = (uint)(char)bVar7;
      if ((char)bVar7 < '\0') {
        bVar4 = puVar28[-2];
        if ((char)bVar4 < -0x40) {
          bVar5 = puVar28[-3];
          if ((char)bVar5 < -0x40) {
            pbVar1 = puVar28 + -4;
            puVar28 = puVar28 + -4;
            uVar19 = bVar5 & 0x3f | (*pbVar1 & 7) << 6;
          }
          else {
            puVar28 = puVar28 + -3;
            uVar19 = bVar5 & 0xf;
          }
          uVar19 = bVar4 & 0x3f | uVar19 << 6;
        }
        else {
          puVar28 = puVar28 + -2;
          uVar19 = bVar4 & 0x1f;
        }
        uVar19 = (uint)(bVar7 & 0x3f) | uVar19 << 6;
        if (uVar19 != 0x5c) goto LAB_141a3b524;
LAB_141a3b535:
        if (puVar28 == &UNK_142ebb45f) break;
        if (puVar28 + -0x142ebb45f < (undefined *)0x38) {
          if (-0x41 < (char)puVar28[1]) break;
        }
        else if (puVar28 + -0x142ebb45f == (undefined *)0x38) break;
                    // WARNING: Subroutine does not return
        core::str::slice_error_fail();
      }
      puVar28 = puVar28 + -1;
      if (uVar19 == 0x5c) goto LAB_141a3b535;
LAB_141a3b524:
      if (uVar19 == 0x2f) goto LAB_141a3b535;
    } while (puVar28 != &DAT_142ebb460);
    local_d8 = (longlong *)&anon_c4dc3b5cd97938b42123e838db8de09d_1_llvm_3856994597783109560;
    local_d0 = (longlong *)&DAT_00000035;
    local_a0 = &local_d8;
    local_98 = anyhow::context::impl$3::fmt<ref_<str_>,alloc::ffi::c_str::FromVecWithNulError>;
    alloc::fmt::format::format_inner();
    local_60 = local_158;
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    local_68 = (longlong *)local_160;
    if (*extraout_RAX_01 == 0) {
      std::sys::thread_local::native::lazy::Storage::
      initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                ();
      plVar21 = extraout_RAX_02;
      if (extraout_RAX_02 == (longlong *)0x0) goto LAB_141a3bc10;
    }
    else {
      if ((int)*extraout_RAX_01 != 1) {
LAB_141a3bc10:
                    // WARNING: Subroutine does not return
        std::thread::local::panic_access_error();
      }
      plVar21 = extraout_RAX_01 + 1;
    }
    if (*plVar21 != 0) {
                    // WARNING: Subroutine does not return
      core::cell::panic_already_borrowed();
    }
    *plVar21 = -1;
    local_70 = plVar21;
    puffin::ThreadProfiler::begin_scope();
    plVar24 = local_58;
    *local_70 = *local_70 + 1;
    if (local_68 != (longlong *)0x0) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
    local_b0 = (longlong *)0x1;
    plVar27 = extraout_RAX_03;
  }
  local_a0 = (longlong **)0x0;
  local_98 = (code *)0x8;
  plStack_90 = (longlong *)0x0;
  plVar29 = (longlong *)plVar24[10];
  plVar21 = plVar29 + plVar24[0xb];
  bVar33 = false;
  lVar23 = 0;
  uVar19 = 0;
  plVar24 = (longlong *)0x0;
  local_68 = plVar21;
  do {
    plVar18 = plStack_90;
    uVar22 = (uint)plVar24;
    if (!bVar33) goto LAB_141a3b790;
    while (plVar27 == (longlong *)0x0) {
LAB_141a3b790:
      if ((plVar29 == (longlong *)0x0) || (plVar29 == plVar21)) {
        if (uVar22 < uVar19) {
          if ((longlong **)plStack_90 == local_a0) {
            alloc::raw_vec::RawVec::
            grow_one<minidump_common::format::MINIDUMP_THREAD_NAME,alloc::alloc::Global>();
          }
          *(longlong *)((longlong)local_98 + (longlong)plVar18 * 0x10) = (longlong)plVar24 << 5;
          *(ulonglong *)((longlong)local_98 + 8 + (longlong)plVar18 * 0x10) = (ulonglong)uVar19 << 5
          ;
          plStack_90 = (longlong *)((longlong)plVar18 + 1);
        }
        puffin::are_scopes_on();
        local_60 = (longlong *)CONCAT71(local_60._1_7_,extraout_AL_00);
        if (extraout_AL_00 == '\0') goto LAB_141a3ba4e;
        puVar28 = &DAT_142ebb498;
        goto LAB_141a3b840;
      }
      plVar27 = (longlong *)*plVar29;
      *plVar29 = 0;
      plVar29 = plVar29 + 1;
      in_RCX = (longlong *)(lVar23 << 6);
      lVar23 = lVar23 + 1;
    }
    iVar9 = 0;
    for (plVar10 = plVar27; ((ulonglong)plVar10 & 1) == 0;
        plVar10 = (longlong *)((ulonglong)plVar10 >> 1 | 0x8000000000000000)) {
      iVar9 = iVar9 + 1;
    }
    uVar32 = iVar9 + (int)in_RCX;
    if ((uVar19 != uVar32) && (bVar33 = uVar22 < uVar19, uVar22 = uVar32, bVar33)) {
      local_60 = plVar24;
      if ((longlong **)plStack_90 == local_a0) {
        alloc::raw_vec::RawVec::
        grow_one<minidump_common::format::MINIDUMP_THREAD_NAME,alloc::alloc::Global>();
      }
      *(ulonglong *)((longlong)local_98 + (longlong)plVar18 * 0x10) =
           ((ulonglong)local_60 & 0xffffffff) << 5;
      *(ulonglong *)((longlong)local_98 + 8 + (longlong)plVar18 * 0x10) = (ulonglong)uVar19 << 5;
      plStack_90 = (longlong *)((longlong)plVar18 + 1);
      plVar21 = local_68;
    }
    plVar27 = (longlong *)((ulonglong)plVar27 & (longlong)plVar27 - 1U);
    uVar19 = uVar32 + 1;
    bVar33 = true;
    plVar24 = (longlong *)(ulonglong)uVar22;
  } while( true );
LAB_141a3b840:
  do {
    bVar7 = puVar28[-1];
    uVar19 = (uint)(char)bVar7;
    if ((char)bVar7 < '\0') {
      bVar4 = puVar28[-2];
      if ((char)bVar4 < -0x40) {
        bVar5 = puVar28[-3];
        if ((char)bVar5 < -0x40) {
          pbVar1 = puVar28 + -4;
          puVar28 = puVar28 + -4;
          uVar19 = bVar5 & 0x3f | (*pbVar1 & 7) << 6;
        }
        else {
          puVar28 = puVar28 + -3;
          uVar19 = bVar5 & 0xf;
        }
        uVar19 = bVar4 & 0x3f | uVar19 << 6;
      }
      else {
        puVar28 = puVar28 + -2;
        uVar19 = bVar4 & 0x1f;
      }
      uVar19 = (uint)(bVar7 & 0x3f) | uVar19 << 6;
      if (uVar19 != 0x5c) goto LAB_141a3b8b4;
LAB_141a3b8d2:
      if (puVar28 == &UNK_142ebb45f) break;
      if (puVar28 + -0x142ebb45f < (undefined *)0x38) {
        if (-0x41 < (char)puVar28[1]) break;
      }
      else if (puVar28 + -0x142ebb45f == (undefined *)0x38) break;
                    // WARNING: Subroutine does not return
      core::str::slice_error_fail();
    }
    puVar28 = puVar28 + -1;
    if (uVar19 == 0x5c) goto LAB_141a3b8d2;
LAB_141a3b8b4:
    if (uVar19 == 0x2f) goto LAB_141a3b8d2;
  } while (puVar28 != &DAT_142ebb460);
  local_d8 = (longlong *)0x0;
  local_d0 = (longlong *)0x1;
  local_c8 = (longlong *)0x0;
  core::fmt::num::imp::impl$9::fmt();
  plVar21 = local_d8;
  if (extraout_AL_01 != '\0') {
                    // WARNING: Subroutine does not return
    core::result::unwrap_failed();
  }
  local_68 = local_d0;
  puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
  local_70 = plVar21;
  if (*extraout_RAX_04 == 0) {
    std::sys::thread_local::native::lazy::Storage::
    initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
              ();
    plVar21 = extraout_RAX_05;
    if (extraout_RAX_05 == (longlong *)0x0) goto LAB_141a3bc4b;
  }
  else {
    if ((int)*extraout_RAX_04 != 1) {
LAB_141a3bc4b:
                    // WARNING: Subroutine does not return
      std::thread::local::panic_access_error();
    }
    plVar21 = extraout_RAX_04 + 1;
  }
  if (*plVar21 != 0) {
                    // WARNING: Subroutine does not return
    core::cell::panic_already_borrowed();
  }
  *plVar21 = -1;
  puffin::ThreadProfiler::begin_scope();
  *plVar21 = *plVar21 + 1;
  if (local_70 != (longlong *)0x0) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
LAB_141a3ba4e:
  ssbo::TypedStorageBuffer::
  gpu_write_ranges_nonoverlapping<country_core::resources::roof_tile_ssbo::RoofTileSsbo>();
  if ((char)local_60 != '\0') {
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_06 == 0) {
      std::sys::thread_local::native::lazy::Storage::
      initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                ();
      local_58 = extraout_RAX_07;
      if (extraout_RAX_07 == (longlong *)0x0) goto LAB_141a3bc59;
    }
    else {
      if ((int)*extraout_RAX_06 != 1) {
LAB_141a3bc59:
                    // WARNING: Subroutine does not return
        std::thread::local::panic_access_error();
      }
      local_58 = extraout_RAX_06 + 1;
    }
    if (*local_58 != 0) {
                    // WARNING: Subroutine does not return
      core::cell::panic_already_borrowed();
    }
    *local_58 = -1;
    puffin::ThreadProfiler::end_scope();
    *local_58 = *local_58 + 1;
  }
  if (local_a0 != (longlong **)0x0) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  if (local_b0 != (longlong *)0x0) {
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_08 == 1) {
      plVar21 = extraout_RAX_08 + 1;
    }
    else if ((*extraout_RAX_08 != 0) ||
            (std::sys::thread_local::native::lazy::Storage::
             initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                       (), plVar21 = extraout_RAX_09, extraout_RAX_09 == (longlong *)0x0)) {
                    // WARNING: Subroutine does not return
      std::thread::local::panic_access_error();
    }
    if (*plVar21 != 0) {
                    // WARNING: Subroutine does not return
      core::cell::panic_already_borrowed();
    }
    *plVar21 = -1;
    local_58 = plVar21;
    puffin::ThreadProfiler::end_scope();
    *local_58 = *local_58 + 1;
  }
  return;
}


// callees: [{"name": "VAL", "address": "140c18130"}, {"name": "__rust_dealloc", "address": "140613c20"}, {"name": "acquire_resource", "address": "140c81c70"}, {"name": "are_scopes_on", "address": "140c16a00"}, {"name": "begin_scope", "address": "140c16d40"}, {"name": "do_reserve_and_handle<alloc::alloc::Global>", "address": "1428dacd0"}, {"name": "drop_in_place<alloc::vec::drain::Drain<rhapsody::rg::resource::ResourceHandle,alloc::alloc::Global>_>", "address": "140609cb0"}, {"name": "drop_slow<country_core::components::transient_mesh::TransientMeshRegistry,alloc::alloc::Global>", "address": "140b5c6b0"}, {"name": "end_scope", "address": "140c170f0"}, {"name": "fmt", "address": "140d8d6c0"}, {"name": "format_inner", "address": "140d79f90"}, {"name": "gpu_write_ranges_nonoverlapping<country_core::resources::roof_tile_ssbo::RoofTileSsbo>", "address": "1421b4de0"}, {"name": "grow_one<minidump_common::format::MINIDUMP_THREAD_NAME,alloc::alloc::Global>", "address": "1407404d0"}, {"name": "initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple$<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>", "address": "142918340"}, {"name": "lock_exclusive_slow", "address": "1428cb5f0"}, {"name": "lock_slow", "address": "1428cd210"}, {"name": "memmove", "address": "142923c10"}, {"name": "panic_access_error", "address": "1428d80d0"}, {"name": "panic_already_borrowed", "address": "1428d9250"}, {"name": "panic_bounds_check", "address": "1428d9518"}, {"name": "release_resource", "address": "140c81e10"}, {"name": "rfind<ref$<str$>_>", "address": "141a3c680"}, {"name": "slice_error_fail", "address": "1428d9940"}, {"name": "unlock_exclusive_slow", "address": "1428cbf60"}, {"name": "unlock_slow", "address": "1428cdb90"}, {"name": "unwrap_failed", "address": "1428d9760"}, {"name": "unwrap_failed", "address": "1428d9310"}]
