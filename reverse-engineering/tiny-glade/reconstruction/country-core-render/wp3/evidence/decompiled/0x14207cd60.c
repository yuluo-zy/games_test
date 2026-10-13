// address 0x14207cd60
// name nani_update_smoke
// signature void __cdecl nani_update_smoke(void)

// WARNING: Type propagation algorithm not settling

void __cdecl country_core::render::nani::systems::nani_update_smoke(void)

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
  longlong lVar14;
  uint *puVar15;
  
  puffin::are_scopes_on();
  if (extraout_AL == '\0') {
    bVar8 = false;
  }
  else {
    uVar11 = 0x36;
    core::str::impl$0::rfind<ref_<str_>_>();
    if (extraout_RAX == 1) {
      if (uVar11 != 0) {
        if (uVar11 < 0x36) {
          if ((char)(&DAT_142fd3135)[uVar11] < -0x40) {
LAB_14207cdf4:
                    // WARNING: Subroutine does not return
            core::str::slice_error_fail();
          }
        }
        else if (uVar11 != 0x36) goto LAB_14207cdf4;
      }
      core::str::impl$0::rfind<ref_<str_>_>();
      if ((extraout_RAX_00 == 1) && (uVar11 != 0xfffffffffffffffe)) {
        if (uVar11 + 2 < 0x36) {
          if ((char)(&UNK_142fd3137)[uVar11] < -0x40) {
LAB_14207ce52:
                    // WARNING: Subroutine does not return
            core::str::slice_error_fail();
          }
        }
        else if (uVar11 + 2 != 0x36) goto LAB_14207ce52;
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
        if (uVar9 != 0x5c) goto LAB_14207cf14;
LAB_14207cf26:
        if (puVar13 == &UNK_142fd342f) break;
        if (puVar13 + -0x142fd342f < (undefined *)0x2e) {
          if (-0x41 < (char)puVar13[1]) break;
        }
        else if (puVar13 + -0x142fd342f == (undefined *)0x2e) break;
                    // WARNING: Subroutine does not return
        core::str::slice_error_fail();
      }
      puVar13 = puVar13 + -1;
      if (uVar9 == 0x5c) goto LAB_14207cf26;
LAB_14207cf14:
      if (uVar9 == 0x2f) goto LAB_14207cf26;
    } while (puVar13 != &DAT_142fd3430);
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_01 == 1) {
      plVar10 = extraout_RAX_01 + 1;
    }
    else if ((*extraout_RAX_01 != 0) ||
            (std::sys::thread_local::native::lazy::Storage::
             initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                       (), plVar10 = extraout_RAX_05, extraout_RAX_05 == (longlong *)0x0))
    goto LAB_14207d51d;
    if (*plVar10 != 0) goto LAB_14207d529;
    *plVar10 = -1;
    puffin::ThreadProfiler::begin_scope();
    *plVar10 = *plVar10 + 1;
    bVar8 = true;
  }
  gpu_world::GpuWorld::owned_subset_mut();
  lVar12 = *in_RCX;
  lVar7 = in_RCX[1];
  puVar15 = *(uint **)(lVar12 + 0xf8);
  puVar2 = puVar15 + *(longlong *)(lVar12 + 0x100);
  cVar6 = *(char *)(lVar12 + 0x108);
  lVar12 = 0;
  lVar14 = 0;
  if (cVar6 != '\0') goto LAB_14207d190;
  do {
    if (lVar14 == lVar12) {
      do {
        if (puVar15 == puVar2) goto LAB_14207d449;
        uVar9 = *puVar15;
        puVar15 = puVar15 + 1;
        lVar12 = *(longlong *)(*(longlong *)(lVar7 + 0x100) + 0x58 + (ulonglong)uVar9 * 0xa0);
      } while (lVar12 == 0);
      lVar14 = 0;
    }
    while( true ) {
      gpu_world::OwnedGpuWorldSubset::
      update_instance_data<country_core::render::nani::instance_data::SmokeInstanceData>();
      lVar14 = lVar14 + 1;
      if (cVar6 == '\0') break;
LAB_14207d190:
      if (lVar14 == lVar12) {
        do {
          if (puVar15 == puVar2) goto LAB_14207d449;
          uVar9 = *puVar15;
          puVar15 = puVar15 + 1;
          lVar12 = *(longlong *)(*(longlong *)(lVar7 + 0x1a8) + 0x10 + (ulonglong)uVar9 * 0x48);
        } while (lVar12 == 0);
        lVar14 = 0;
      }
    }
  } while( true );
LAB_14207d449:
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
LAB_14207d51d:
                    // WARNING: Subroutine does not return
      std::thread::local::panic_access_error();
    }
    if (*plVar10 != 0) {
LAB_14207d529:
                    // WARNING: Subroutine does not return
      core::cell::panic_already_borrowed();
    }
    *plVar10 = -1;
    puffin::ThreadProfiler::end_scope();
    *plVar10 = *plVar10 + 1;
  }
  return;
}

