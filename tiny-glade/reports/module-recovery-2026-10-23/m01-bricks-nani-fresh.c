/* fresh 0x142076230 Ghidra body=2209, PDB full-span see identity */

// WARNING: Type propagation algorithm not settling

void __cdecl country_core::render::nani::systems::nani_update_solid_vertex_color_bricks(void)

{
  byte *pbVar1;
  uint *puVar2;
  byte bVar3;
  byte bVar4;
  byte bVar5;
  char cVar6;
  longlong lVar7;
  bool bVar8;
  char extraout_AL;
  uint uVar9;
  longlong extraout_RAX;
  longlong extraout_RAX_00;
  longlong *extraout_RAX_01;
  longlong *extraout_RAX_02;
  longlong *extraout_RAX_03;
  longlong *extraout_RAX_04;
  longlong *extraout_RAX_05;
  longlong *in_RCX;
  longlong *plVar10;
  ulonglong uVar11;
  longlong lVar12;
  undefined *puVar13;
  uint *puVar14;
  longlong lVar15;
  
  puffin::are_scopes_on();
  if (extraout_AL == '\0') {
    bVar8 = false;
  }
  else {
    uVar11 = 0x4a;
    core::str::impl$0::rfind<ref_<str_>_>();
    if (extraout_RAX == 1) {
      if (uVar11 != 0) {
        if (uVar11 < 0x4a) {
          if ((char)(&DAT_142fd2ffb)[uVar11] < -0x40) {
LAB_1420762f2:
                    // WARNING: Subroutine does not return
            core::str::slice_error_fail();
          }
        }
        else if (uVar11 != 0x4a) goto LAB_1420762f2;
      }
      core::str::impl$0::rfind<ref_<str_>_>();
      if ((extraout_RAX_00 == 1) && (uVar11 != 0xfffffffffffffffe)) {
        if (uVar11 + 2 < 0x4a) {
          if ((char)(&UNK_142fd2ffd)[uVar11] < -0x40) {
LAB_142076350:
                    // WARNING: Subroutine does not return
            core::str::slice_error_fail();
          }
        }
        else if (uVar11 + 2 != 0x4a) goto LAB_142076350;
      }
    }
    puVar13 = &DAT_142fd345e;
    do {
      bVar3 = puVar13[-1];
      uVar9 = (uint)(char)bVar3;
      if ((char)bVar3 < '\0') {
        bVar4 = puVar13[-2];
        if ((char)bVar4 < -0x40) {
          bVar5 = puVar13[-3];
          if ((char)bVar5 < -0x40) {
            pbVar1 = puVar13 + -4;
            puVar13 = puVar13 + -4;
            uVar9 = bVar5 & 0x3f | (*pbVar1 & 7) << 6;
          }
          else {
            puVar13 = puVar13 + -3;
            uVar9 = bVar5 & 0xf;
          }
          uVar9 = bVar4 & 0x3f | uVar9 << 6;
        }
        else {
          puVar13 = puVar13 + -2;
          uVar9 = bVar4 & 0x1f;
        }
        uVar9 = (uint)(bVar3 & 0x3f) | uVar9 << 6;
        if (uVar9 != 0x5c) goto LAB_142076414;
LAB_142076426:
        if (puVar13 == &UNK_142fd342f) break;
        if (puVar13 + -0x142fd342f < (undefined *)0x2e) {
          if (-0x41 < (char)puVar13[1]) break;
        }
        else if (puVar13 + -0x142fd342f == (undefined *)0x2e) break;
                    // WARNING: Subroutine does not return
        core::str::slice_error_fail();
      }
      puVar13 = puVar13 + -1;
      if (uVar9 == 0x5c) goto LAB_142076426;
LAB_142076414:
      if (uVar9 == 0x2f) goto LAB_142076426;
    } while (puVar13 != &DAT_142fd3430);
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_01 == 1) {
      plVar10 = extraout_RAX_01 + 1;
    }
    else if ((*extraout_RAX_01 != 0) ||
            (std::sys::thread_local::native::lazy::Storage::
             initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                       (), plVar10 = extraout_RAX_05, extraout_RAX_05 == (longlong *)0x0))
    goto LAB_142076ab8;
    if (*plVar10 != 0) goto LAB_142076ac4;
    *plVar10 = -1;
    puffin::ThreadProfiler::begin_scope();
    *plVar10 = *plVar10 + 1;
    bVar8 = true;
  }
  gpu_world::GpuWorld::owned_subset_mut();
  lVar12 = *in_RCX;
  lVar7 = in_RCX[1];
  puVar14 = *(uint **)(lVar12 + 0xf8);
  puVar2 = puVar14 + *(longlong *)(lVar12 + 0x100);
  cVar6 = *(char *)(lVar12 + 0x108);
  lVar12 = 0;
  lVar15 = 0;
  if (cVar6 != '\0') goto LAB_1420766a0;
  do {
    if (lVar15 == lVar12) {
      do {
        if (puVar14 == puVar2) goto LAB_1420769b1;
        uVar9 = *puVar14;
        puVar14 = puVar14 + 1;
        lVar12 = *(longlong *)(*(longlong *)(lVar7 + 0x100) + 0x58 + (ulonglong)uVar9 * 0xa0);
      } while (lVar12 == 0);
      lVar15 = 0;
    }
    while( true ) {
      ::utils::pack_2x16f_uint();
      gpu_world::OwnedGpuWorldSubset::
      update_instance_data<country_core::render::nani::instance_data::SolidVertexColorBricksInstanceData>
                ();
      lVar15 = lVar15 + 1;
      if (cVar6 == '\0') break;
LAB_1420766a0:
      if (lVar15 == lVar12) {
        do {
          if (puVar14 == puVar2) goto LAB_1420769b1;
          uVar9 = *puVar14;
          puVar14 = puVar14 + 1;
          lVar12 = *(longlong *)(*(longlong *)(lVar7 + 0x1a8) + 0x10 + (ulonglong)uVar9 * 0x48);
        } while (lVar12 == 0);
        lVar15 = 0;
      }
    }
  } while( true );
LAB_1420769b1:
  parking_lot_core::parking_lot::deadlock_impl::release_resource();
  parking_lot_core::parking_lot::deadlock_impl::release_resource();
  LOCK();
  lVar12 = *extraout_RAX_02;
  if (lVar12 == 8) {
    *extraout_RAX_02 = 0;
  }
  UNLOCK();
  if (lVar12 != 8) {
    parking_lot::raw_rwlock::RawRwLock::unlock_exclusive_slow();
  }
  if (bVar8) {
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_03 == 1) {
      plVar10 = extraout_RAX_03 + 1;
    }
    else if ((*extraout_RAX_03 != 0) ||
            (std::sys::thread_local::native::lazy::Storage::
             initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                       (), plVar10 = extraout_RAX_04, extraout_RAX_04 == (longlong *)0x0)) {
LAB_142076ab8:
                    // WARNING: Subroutine does not return
      std::thread::local::panic_access_error();
    }
    if (*plVar10 != 0) {
LAB_142076ac4:
                    // WARNING: Subroutine does not return
      core::cell::panic_already_borrowed();
    }
    *plVar10 = -1;
    puffin::ThreadProfiler::end_scope();
    *plVar10 = *plVar10 + 1;
  }
  return;
}



/* fresh 0x142076e40 Ghidra body=2880, PDB full-span see identity */

// WARNING: Type propagation algorithm not settling

void __cdecl country_core::render::nani::systems::nani_update_gothic_window_bricks(void)

{
  byte *pbVar1;
  uint *puVar2;
  byte bVar3;
  byte bVar4;
  byte bVar5;
  char cVar6;
  longlong lVar7;
  bool bVar8;
  char extraout_AL;
  uint uVar9;
  longlong extraout_RAX;
  longlong extraout_RAX_00;
  longlong *extraout_RAX_01;
  longlong *extraout_RAX_02;
  longlong *extraout_RAX_03;
  longlong *extraout_RAX_04;
  longlong *extraout_RAX_05;
  longlong *in_RCX;
  longlong *plVar10;
  ulonglong uVar11;
  longlong lVar12;
  uint *puVar13;
  undefined *puVar14;
  longlong local_130;
  
  puffin::are_scopes_on();
  if (extraout_AL == '\0') {
    bVar8 = false;
  }
  else {
    uVar11 = 0x45;
    core::str::impl$0::rfind<ref_<str_>_>();
    if (extraout_RAX == 1) {
      if (uVar11 != 0) {
        if (uVar11 < 0x45) {
          if ((char)(&DAT_142fd330d)[uVar11] < -0x40) {
LAB_142076f02:
                    // WARNING: Subroutine does not return
            core::str::slice_error_fail();
          }
        }
        else if (uVar11 != 0x45) goto LAB_142076f02;
      }
      core::str::impl$0::rfind<ref_<str_>_>();
      if ((extraout_RAX_00 == 1) && (uVar11 != 0xfffffffffffffffe)) {
        if (uVar11 + 2 < 0x45) {
          if ((char)(&UNK_142fd330f)[uVar11] < -0x40) {
LAB_142076f60:
                    // WARNING: Subroutine does not return
            core::str::slice_error_fail();
          }
        }
        else if (uVar11 + 2 != 0x45) goto LAB_142076f60;
      }
    }
    puVar14 = &DAT_142fd345e;
    do {
      bVar3 = puVar14[-1];
      uVar9 = (uint)(char)bVar3;
      if ((char)bVar3 < '\0') {
        bVar4 = puVar14[-2];
        if ((char)bVar4 < -0x40) {
          bVar5 = puVar14[-3];
          if ((char)bVar5 < -0x40) {
            pbVar1 = puVar14 + -4;
            puVar14 = puVar14 + -4;
            uVar9 = bVar5 & 0x3f | (*pbVar1 & 7) << 6;
          }
          else {
            puVar14 = puVar14 + -3;
            uVar9 = bVar5 & 0xf;
          }
          uVar9 = bVar4 & 0x3f | uVar9 << 6;
        }
        else {
          puVar14 = puVar14 + -2;
          uVar9 = bVar4 & 0x1f;
        }
        uVar9 = (uint)(bVar3 & 0x3f) | uVar9 << 6;
        if (uVar9 != 0x5c) goto LAB_142077024;
LAB_142077036:
        if (puVar14 == &UNK_142fd342f) break;
        if (puVar14 + -0x142fd342f < (undefined *)0x2e) {
          if (-0x41 < (char)puVar14[1]) break;
        }
        else if (puVar14 + -0x142fd342f == (undefined *)0x2e) break;
                    // WARNING: Subroutine does not return
        core::str::slice_error_fail();
      }
      puVar14 = puVar14 + -1;
      if (uVar9 == 0x5c) goto LAB_142077036;
LAB_142077024:
      if (uVar9 == 0x2f) goto LAB_142077036;
    } while (puVar14 != &DAT_142fd3430);
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_01 == 1) {
      plVar10 = extraout_RAX_01 + 1;
    }
    else if ((*extraout_RAX_01 != 0) ||
            (std::sys::thread_local::native::lazy::Storage::
             initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                       (), plVar10 = extraout_RAX_05, extraout_RAX_05 == (longlong *)0x0))
    goto LAB_142077972;
    if (*plVar10 != 0) goto LAB_14207797e;
    *plVar10 = -1;
    puffin::ThreadProfiler::begin_scope();
    *plVar10 = *plVar10 + 1;
    bVar8 = true;
  }
  gpu_world::GpuWorld::owned_subset_mut();
  lVar12 = *in_RCX;
  lVar7 = in_RCX[1];
  puVar13 = *(uint **)(lVar12 + 0xf8);
  puVar2 = puVar13 + *(longlong *)(lVar12 + 0x100);
  cVar6 = *(char *)(lVar12 + 0x108);
  lVar12 = 0;
  local_130 = 0;
  if (cVar6 != '\0') goto LAB_142077390;
  do {
    if (local_130 == lVar12) {
      do {
        if (puVar13 == puVar2) goto LAB_14207785c;
        uVar9 = *puVar13;
        puVar13 = puVar13 + 1;
        lVar12 = *(longlong *)(*(longlong *)(lVar7 + 0x100) + 0x58 + (ulonglong)uVar9 * 0xa0);
      } while (lVar12 == 0);
      local_130 = 0;
    }
    while( true ) {
      ::utils::pack_2x16f_uint();
      ::utils::pack_2x16f_uint();
      ::utils::pack_2x16f_uint();
      ::utils::pack_2x16f_uint();
      gpu_world::OwnedGpuWorldSubset::
      update_instance_data<country_core::render::nani::instance_data::GothicWindowBricksInstanceData>
                ();
      local_130 = local_130 + 1;
      if (cVar6 == '\0') break;
LAB_142077390:
      if (local_130 == lVar12) {
        do {
          if (puVar13 == puVar2) goto LAB_14207785c;
          uVar9 = *puVar13;
          puVar13 = puVar13 + 1;
          lVar12 = *(longlong *)(*(longlong *)(lVar7 + 0x1a8) + 0x10 + (ulonglong)uVar9 * 0x48);
        } while (lVar12 == 0);
        local_130 = 0;
      }
    }
  } while( true );
LAB_14207785c:
  parking_lot_core::parking_lot::deadlock_impl::release_resource();
  parking_lot_core::parking_lot::deadlock_impl::release_resource();
  LOCK();
  lVar12 = *extraout_RAX_02;
  if (lVar12 == 8) {
    *extraout_RAX_02 = 0;
  }
  UNLOCK();
  if (lVar12 != 8) {
    parking_lot::raw_rwlock::RawRwLock::unlock_exclusive_slow();
  }
  if (bVar8) {
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_03 == 1) {
      plVar10 = extraout_RAX_03 + 1;
    }
    else if ((*extraout_RAX_03 != 0) ||
            (std::sys::thread_local::native::lazy::Storage::
             initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                       (), plVar10 = extraout_RAX_04, extraout_RAX_04 == (longlong *)0x0)) {
LAB_142077972:
                    // WARNING: Subroutine does not return
      std::thread::local::panic_access_error();
    }
    if (*plVar10 != 0) {
LAB_14207797e:
                    // WARNING: Subroutine does not return
      core::cell::panic_already_borrowed();
    }
    *plVar10 = -1;
    puffin::ThreadProfiler::end_scope();
    *plVar10 = *plVar10 + 1;
  }
  return;
}


