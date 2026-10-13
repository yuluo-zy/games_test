// address 0x14207d770
// name nani_update_balcony_door
// signature void __cdecl nani_update_balcony_door(void)

// WARNING: Type propagation algorithm not settling

void __cdecl country_core::render::nani::systems::nani_update_balcony_door(void)

{
  byte *pbVar1;
  uint *puVar2;
  byte bVar3;
  byte bVar4;
  byte bVar5;
  char cVar6;
  longlong lVar7;
  longlong lVar8;
  bool bVar9;
  char extraout_AL;
  uint uVar10;
  longlong extraout_RAX;
  longlong extraout_RAX_00;
  longlong *extraout_RAX_01;
  longlong *extraout_RAX_02;
  longlong *extraout_RAX_03;
  longlong *extraout_RAX_04;
  longlong *extraout_RAX_05;
  longlong *in_RCX;
  longlong *plVar11;
  ulonglong uVar12;
  ulonglong uVar13;
  longlong lVar14;
  longlong lVar15;
  undefined *puVar16;
  uint *puVar17;
  
  puffin::are_scopes_on();
  if (extraout_AL == '\0') {
    bVar9 = false;
  }
  else {
    uVar12 = 0x3d;
    core::str::impl$0::rfind<ref_<str_>_>();
    if (extraout_RAX == 1) {
      if (uVar12 != 0) {
        if (uVar12 < 0x3d) {
          if ((char)(&DAT_142fd32cd)[uVar12] < -0x40) {
LAB_14207d804:
                    // WARNING: Subroutine does not return
            core::str::slice_error_fail();
          }
        }
        else if (uVar12 != 0x3d) goto LAB_14207d804;
      }
      core::str::impl$0::rfind<ref_<str_>_>();
      if ((extraout_RAX_00 == 1) && (uVar12 != 0xfffffffffffffffe)) {
        if (uVar12 + 2 < 0x3d) {
          if ((char)(&UNK_142fd32cf)[uVar12] < -0x40) {
LAB_14207d862:
                    // WARNING: Subroutine does not return
            core::str::slice_error_fail();
          }
        }
        else if (uVar12 + 2 != 0x3d) goto LAB_14207d862;
      }
    }
    puVar16 = &DAT_142fd345e;
    do {
      bVar3 = puVar16[-1];
      uVar10 = (uint)(char)bVar3;
      if ((char)bVar3 < '\0') {
        bVar4 = puVar16[-2];
        if ((char)bVar4 < -0x40) {
          bVar5 = puVar16[-3];
          if ((char)bVar5 < -0x40) {
            pbVar1 = puVar16 + -4;
            puVar16 = puVar16 + -4;
            uVar10 = bVar5 & 0x3f | (*pbVar1 & 7) << 6;
          }
          else {
            puVar16 = puVar16 + -3;
            uVar10 = bVar5 & 0xf;
          }
          uVar10 = bVar4 & 0x3f | uVar10 << 6;
        }
        else {
          puVar16 = puVar16 + -2;
          uVar10 = bVar4 & 0x1f;
        }
        uVar10 = (uint)(bVar3 & 0x3f) | uVar10 << 6;
        if (uVar10 != 0x5c) goto LAB_14207d924;
LAB_14207d936:
        if (puVar16 == &UNK_142fd342f) break;
        if (puVar16 + -0x142fd342f < (undefined *)0x2e) {
          if (-0x41 < (char)puVar16[1]) break;
        }
        else if (puVar16 + -0x142fd342f == (undefined *)0x2e) break;
                    // WARNING: Subroutine does not return
        core::str::slice_error_fail();
      }
      puVar16 = puVar16 + -1;
      if (uVar10 == 0x5c) goto LAB_14207d936;
LAB_14207d924:
      if (uVar10 == 0x2f) goto LAB_14207d936;
    } while (puVar16 != &DAT_142fd3430);
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_01 == 1) {
      plVar11 = extraout_RAX_01 + 1;
    }
    else if ((*extraout_RAX_01 != 0) ||
            (std::sys::thread_local::native::lazy::Storage::
             initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                       (), plVar11 = extraout_RAX_05, extraout_RAX_05 == (longlong *)0x0))
    goto LAB_14207dfa8;
    if (*plVar11 != 0) goto LAB_14207dfb4;
    *plVar11 = -1;
    puffin::ThreadProfiler::begin_scope();
    *plVar11 = *plVar11 + 1;
    bVar9 = true;
  }
  gpu_world::GpuWorld::owned_subset_mut();
  lVar7 = *in_RCX;
  lVar8 = in_RCX[1];
  puVar17 = *(uint **)(lVar7 + 0xf8);
  puVar2 = puVar17 + *(longlong *)(lVar7 + 0x100);
  cVar6 = *(char *)(lVar7 + 0x108);
  lVar15 = 0;
  lVar14 = 0;
  if (cVar6 != '\0') goto LAB_14207dbe0;
  do {
    if (lVar14 == lVar15) {
      do {
        if (puVar17 == puVar2) goto LAB_14207ded4;
        uVar10 = *puVar17;
        puVar17 = puVar17 + 1;
        lVar15 = *(longlong *)(*(longlong *)(lVar8 + 0x100) + 0x58 + (ulonglong)uVar10 * 0xa0);
      } while (lVar15 == 0);
      lVar14 = 0;
    }
    while( true ) {
      gpu_world::OwnedGpuWorldSubset::
      update_instance_data<country_core::render::nani::instance_data::BalconyDoorInstanceData>();
      lVar14 = lVar14 + 1;
      if (cVar6 == '\0') break;
LAB_14207dbe0:
      if (lVar14 == lVar15) {
        do {
          if (puVar17 == puVar2) goto LAB_14207ded4;
          uVar10 = *puVar17;
          puVar17 = puVar17 + 1;
          lVar14 = *(longlong *)(lVar8 + 0x1a8);
          lVar15 = *(longlong *)(lVar14 + 0x10 + (ulonglong)uVar10 * 0x48);
        } while (lVar15 == 0);
        lVar14 = lVar14 + (ulonglong)uVar10 * 0x48;
        uVar12 = *(ulonglong *)(lVar14 + 0x40);
        if ((*(ulonglong *)(lVar7 + 0x110) < uVar12) &&
           (*(longlong *)(*(longlong *)(lVar14 + 0x38) + *(ulonglong *)(lVar7 + 0x110) * 8) != 0)) {
          uVar13 = *(ulonglong *)(lVar7 + 0x118);
          if (uVar12 <= uVar13) goto LAB_14207dca3;
LAB_14207dc70:
          if (*(longlong *)(*(longlong *)(lVar14 + 0x38) + uVar13 * 8) == 0) goto LAB_14207dca3;
          uVar13 = *(ulonglong *)(lVar7 + 0x120);
          if (uVar13 < uVar12) goto LAB_14207dcb7;
LAB_14207dcef:
          uVar13 = *(ulonglong *)(lVar7 + 0x128);
        }
        else {
          uVar13 = *(ulonglong *)(lVar7 + 0x118);
          if (uVar13 < uVar12) goto LAB_14207dc70;
LAB_14207dca3:
          uVar13 = *(ulonglong *)(lVar7 + 0x120);
          if (uVar12 <= uVar13) goto LAB_14207dcef;
LAB_14207dcb7:
          if (*(longlong *)(*(longlong *)(lVar14 + 0x38) + uVar13 * 8) == 0) goto LAB_14207dcef;
          uVar13 = *(ulonglong *)(lVar7 + 0x128);
        }
        if (uVar13 < uVar12) {
          lVar14 = 0;
        }
        else {
          lVar14 = 0;
        }
      }
    }
  } while( true );
LAB_14207ded4:
  parking_lot_core::parking_lot::deadlock_impl::release_resource();
  parking_lot_core::parking_lot::deadlock_impl::release_resource();
  LOCK();
  lVar7 = *extraout_RAX_02;
  if (lVar7 == 8) {
    *extraout_RAX_02 = 0;
  }
  UNLOCK();
  if (lVar7 != 8) {
    parking_lot::raw_rwlock::RawRwLock::unlock_exclusive_slow();
  }
  if (bVar9) {
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_03 == 1) {
      plVar11 = extraout_RAX_03 + 1;
    }
    else if ((*extraout_RAX_03 != 0) ||
            (std::sys::thread_local::native::lazy::Storage::
             initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                       (), plVar11 = extraout_RAX_04, extraout_RAX_04 == (longlong *)0x0)) {
LAB_14207dfa8:
                    // WARNING: Subroutine does not return
      std::thread::local::panic_access_error();
    }
    if (*plVar11 != 0) {
LAB_14207dfb4:
                    // WARNING: Subroutine does not return
      core::cell::panic_already_borrowed();
    }
    *plVar11 = -1;
    puffin::ThreadProfiler::end_scope();
    *plVar11 = *plVar11 + 1;
  }
  return;
}

