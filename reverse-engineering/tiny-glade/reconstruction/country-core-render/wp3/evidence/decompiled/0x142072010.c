// address 0x142072010
// name nani_update_solid_albedo_map
// signature void __cdecl nani_update_solid_albedo_map(void)

// WARNING: Type propagation algorithm not settling

void __cdecl country_core::render::nani::systems::nani_update_solid_albedo_map(void)

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
  longlong lVar13;
  undefined *puVar14;
  uint *puVar15;
  
  puffin::are_scopes_on();
  if (extraout_AL == '\0') {
    bVar8 = false;
  }
  else {
    uVar11 = 0x41;
    core::str::impl$0::rfind<ref_<str_>_>();
    if (extraout_RAX == 1) {
      if (uVar11 != 0) {
        if (uVar11 < 0x41) {
          if ((char)(&DAT_142fd30b8)[uVar11] < -0x40) {
LAB_1420720c4:
                    // WARNING: Subroutine does not return
            core::str::slice_error_fail();
          }
        }
        else if (uVar11 != 0x41) goto LAB_1420720c4;
      }
      core::str::impl$0::rfind<ref_<str_>_>();
      if ((extraout_RAX_00 == 1) && (uVar11 != 0xfffffffffffffffe)) {
        if (uVar11 + 2 < 0x41) {
          if ((char)(&UNK_142fd30ba)[uVar11] < -0x40) {
LAB_142072122:
                    // WARNING: Subroutine does not return
            core::str::slice_error_fail();
          }
        }
        else if (uVar11 + 2 != 0x41) goto LAB_142072122;
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
        if (uVar9 != 0x5c) goto LAB_1420721e4;
LAB_1420721f6:
        if (puVar14 == &UNK_142fd342f) break;
        if (puVar14 + -0x142fd342f < (undefined *)0x2e) {
          if (-0x41 < (char)puVar14[1]) break;
        }
        else if (puVar14 + -0x142fd342f == (undefined *)0x2e) break;
                    // WARNING: Subroutine does not return
        core::str::slice_error_fail();
      }
      puVar14 = puVar14 + -1;
      if (uVar9 == 0x5c) goto LAB_1420721f6;
LAB_1420721e4:
      if (uVar9 == 0x2f) goto LAB_1420721f6;
    } while (puVar14 != &DAT_142fd3430);
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_01 == 1) {
      plVar10 = extraout_RAX_01 + 1;
    }
    else if ((*extraout_RAX_01 != 0) ||
            (std::sys::thread_local::native::lazy::Storage::
             initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                       (), plVar10 = extraout_RAX_05, extraout_RAX_05 == (longlong *)0x0))
    goto LAB_142072801;
    if (*plVar10 != 0) goto LAB_14207280d;
    *plVar10 = -1;
    puffin::ThreadProfiler::begin_scope();
    *plVar10 = *plVar10 + 1;
    bVar8 = true;
  }
  gpu_world::GpuWorld::owned_subset_mut();
  lVar13 = *in_RCX;
  lVar7 = in_RCX[1];
  puVar15 = *(uint **)(lVar13 + 0xf8);
  puVar2 = puVar15 + *(longlong *)(lVar13 + 0x100);
  cVar6 = *(char *)(lVar13 + 0x108);
  lVar13 = 0;
  lVar12 = 0;
  if (cVar6 != '\0') goto LAB_142072450;
  do {
    if (lVar12 == lVar13) {
      do {
        if (puVar15 == puVar2) goto LAB_142072705;
        uVar9 = *puVar15;
        puVar15 = puVar15 + 1;
        lVar13 = *(longlong *)(*(longlong *)(lVar7 + 0x100) + 0x58 + (ulonglong)uVar9 * 0xa0);
      } while (lVar13 == 0);
      lVar12 = 0;
    }
    while( true ) {
      gpu_world::OwnedGpuWorldSubset::
      update_instance_data<country_core::render::nani::instance_data::SolidAlbedoMapInstanceData>();
      lVar12 = lVar12 + 1;
      if (cVar6 == '\0') break;
LAB_142072450:
      if (lVar12 == lVar13) {
        do {
          if (puVar15 == puVar2) goto LAB_142072705;
          uVar9 = *puVar15;
          puVar15 = puVar15 + 1;
          lVar13 = *(longlong *)(*(longlong *)(lVar7 + 0x1a8) + 0x10 + (ulonglong)uVar9 * 0x48);
        } while (lVar13 == 0);
        lVar12 = 0;
      }
    }
  } while( true );
LAB_142072705:
  parking_lot_core::parking_lot::deadlock_impl::release_resource();
  parking_lot_core::parking_lot::deadlock_impl::release_resource();
  LOCK();
  lVar13 = *extraout_RAX_02;
  if (lVar13 == 8) {
    *extraout_RAX_02 = 0;
  }
  UNLOCK();
  if (lVar13 != 8) {
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
LAB_142072801:
                    // WARNING: Subroutine does not return
      std::thread::local::panic_access_error();
    }
    if (*plVar10 != 0) {
LAB_14207280d:
                    // WARNING: Subroutine does not return
      core::cell::panic_already_borrowed();
    }
    *plVar10 = -1;
    puffin::ThreadProfiler::end_scope();
    *plVar10 = *plVar10 + 1;
  }
  return;
}

