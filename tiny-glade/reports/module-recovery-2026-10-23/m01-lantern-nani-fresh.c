
// WARNING: Type propagation algorithm not settling

void __cdecl country_core::render::nani::systems::nani_update_lantern_emissive(void)

{
  byte *pbVar1;
  uint *puVar2;
  byte bVar3;
  byte bVar4;
  byte bVar5;
  char cVar6;
  longlong lVar7;
  longlong lVar8;
  longlong lVar9;
  bool bVar10;
  char extraout_AL;
  uint uVar11;
  longlong extraout_RAX;
  longlong extraout_RAX_00;
  longlong *extraout_RAX_01;
  longlong *extraout_RAX_02;
  longlong *extraout_RAX_03;
  longlong *extraout_RAX_04;
  longlong *extraout_RAX_05;
  longlong *in_RCX;
  longlong *plVar12;
  ulonglong uVar13;
  ulonglong uVar14;
  longlong lVar15;
  undefined *puVar16;
  uint *puVar17;
  longlong local_f0;
  
  puffin::are_scopes_on();
  if (extraout_AL == '\0') {
    bVar10 = false;
  }
  else {
    uVar13 = 0x41;
    core::str::impl$0::rfind<ref_<str_>_>();
    if (extraout_RAX == 1) {
      if (uVar13 != 0) {
        if (uVar13 < 0x41) {
          if ((char)(&DAT_142fd316e)[uVar13] < -0x40) {
LAB_14207a58a:
                    // WARNING: Subroutine does not return
            core::str::slice_error_fail();
          }
        }
        else if (uVar13 != 0x41) goto LAB_14207a58a;
      }
      core::str::impl$0::rfind<ref_<str_>_>();
      if ((extraout_RAX_00 == 1) && (uVar13 != 0xfffffffffffffffe)) {
        if (uVar13 + 2 < 0x41) {
          if ((char)(&UNK_142fd3170)[uVar13] < -0x40) {
LAB_14207a5e8:
                    // WARNING: Subroutine does not return
            core::str::slice_error_fail();
          }
        }
        else if (uVar13 + 2 != 0x41) goto LAB_14207a5e8;
      }
    }
    puVar16 = &DAT_142fd345e;
    do {
      bVar3 = puVar16[-1];
      uVar11 = (uint)(char)bVar3;
      if ((char)bVar3 < '\0') {
        bVar4 = puVar16[-2];
        if ((char)bVar4 < -0x40) {
          bVar5 = puVar16[-3];
          if ((char)bVar5 < -0x40) {
            pbVar1 = puVar16 + -4;
            puVar16 = puVar16 + -4;
            uVar11 = bVar5 & 0x3f | (*pbVar1 & 7) << 6;
          }
          else {
            puVar16 = puVar16 + -3;
            uVar11 = bVar5 & 0xf;
          }
          uVar11 = bVar4 & 0x3f | uVar11 << 6;
        }
        else {
          puVar16 = puVar16 + -2;
          uVar11 = bVar4 & 0x1f;
        }
        uVar11 = (uint)(bVar3 & 0x3f) | uVar11 << 6;
        if (uVar11 != 0x5c) goto LAB_14207a6a4;
LAB_14207a6b6:
        if (puVar16 == &UNK_142fd342f) break;
        if (puVar16 + -0x142fd342f < (undefined *)0x2e) {
          if (-0x41 < (char)puVar16[1]) break;
        }
        else if (puVar16 + -0x142fd342f == (undefined *)0x2e) break;
                    // WARNING: Subroutine does not return
        core::str::slice_error_fail();
      }
      puVar16 = puVar16 + -1;
      if (uVar11 == 0x5c) goto LAB_14207a6b6;
LAB_14207a6a4:
      if (uVar11 == 0x2f) goto LAB_14207a6b6;
    } while (puVar16 != &DAT_142fd3430);
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_01 == 1) {
      plVar12 = extraout_RAX_01 + 1;
    }
    else if ((*extraout_RAX_01 != 0) ||
            (std::sys::thread_local::native::lazy::Storage::
             initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                       (), plVar12 = extraout_RAX_05, extraout_RAX_05 == (longlong *)0x0))
    goto LAB_14207b044;
    if (*plVar12 != 0) goto LAB_14207b050;
    *plVar12 = -1;
    puffin::ThreadProfiler::begin_scope();
    *plVar12 = *plVar12 + 1;
    bVar10 = true;
  }
  gpu_world::GpuWorld::owned_subset_mut();
  lVar7 = *in_RCX;
  lVar8 = in_RCX[1];
  puVar17 = *(uint **)(lVar7 + 0xf8);
  puVar2 = puVar17 + *(longlong *)(lVar7 + 0x100);
  cVar6 = *(char *)(lVar7 + 0x108);
  lVar15 = 0;
  local_f0 = 0;
  if (cVar6 != '\0') goto LAB_14207aac0;
  do {
    if (local_f0 == lVar15) {
      do {
        if (puVar17 == puVar2) goto LAB_14207af67;
        uVar11 = *puVar17;
        puVar17 = puVar17 + 1;
        lVar15 = *(longlong *)(*(longlong *)(lVar8 + 0x100) + 0x58 + (ulonglong)uVar11 * 0xa0);
      } while (lVar15 == 0);
      local_f0 = 0;
    }
    while( true ) {
      ::utils::pack_2x16f_uint();
      ::utils::pack_2x16f_uint();
      gpu_world::OwnedGpuWorldSubset::
      update_instance_data<country_core::render::nani::instance_data::LanternEmissiveInstanceData>()
      ;
      local_f0 = local_f0 + 1;
      if (cVar6 == '\0') break;
LAB_14207aac0:
      if (local_f0 == lVar15) {
        do {
          if (puVar17 == puVar2) goto LAB_14207af67;
          uVar11 = *puVar17;
          puVar17 = puVar17 + 1;
          lVar9 = *(longlong *)(lVar8 + 0x1a8);
          lVar15 = *(longlong *)(lVar9 + 0x10 + (ulonglong)uVar11 * 0x48);
        } while (lVar15 == 0);
        lVar9 = lVar9 + (ulonglong)uVar11 * 0x48;
        uVar13 = *(ulonglong *)(lVar9 + 0x40);
        if ((*(ulonglong *)(lVar7 + 0x110) < uVar13) &&
           (*(longlong *)(*(longlong *)(lVar9 + 0x38) + *(ulonglong *)(lVar7 + 0x110) * 8) != 0)) {
          uVar14 = *(ulonglong *)(lVar7 + 0x118);
          if (uVar13 <= uVar14) goto LAB_14207ac20;
LAB_14207abea:
          if (*(longlong *)(*(longlong *)(lVar9 + 0x38) + uVar14 * 8) == 0) goto LAB_14207ac20;
          uVar14 = *(ulonglong *)(lVar7 + 0x120);
          if (uVar14 < uVar13) goto LAB_14207ac35;
LAB_14207ac6b:
          uVar14 = *(ulonglong *)(lVar7 + 0x128);
          if (uVar13 <= uVar14) goto LAB_14207acca;
LAB_14207ac81:
          if (*(longlong *)(*(longlong *)(lVar9 + 0x38) + uVar14 * 8) == 0) goto LAB_14207acca;
          uVar14 = *(ulonglong *)(lVar7 + 0x130);
        }
        else {
          uVar14 = *(ulonglong *)(lVar7 + 0x118);
          if (uVar14 < uVar13) goto LAB_14207abea;
LAB_14207ac20:
          uVar14 = *(ulonglong *)(lVar7 + 0x120);
          if (uVar13 <= uVar14) goto LAB_14207ac6b;
LAB_14207ac35:
          if (*(longlong *)(*(longlong *)(lVar9 + 0x38) + uVar14 * 8) == 0) goto LAB_14207ac6b;
          uVar14 = *(ulonglong *)(lVar7 + 0x128);
          if (uVar14 < uVar13) goto LAB_14207ac81;
LAB_14207acca:
          uVar14 = *(ulonglong *)(lVar7 + 0x130);
        }
        if ((uVar14 < uVar13) && (*(longlong *)(*(longlong *)(lVar9 + 0x38) + uVar14 * 8) != 0)) {
          local_f0 = 0;
        }
        else {
          local_f0 = 0;
        }
      }
    }
  } while( true );
LAB_14207af67:
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
  if (bVar10) {
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_03 == 1) {
      plVar12 = extraout_RAX_03 + 1;
    }
    else if ((*extraout_RAX_03 != 0) ||
            (std::sys::thread_local::native::lazy::Storage::
             initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                       (), plVar12 = extraout_RAX_04, extraout_RAX_04 == (longlong *)0x0)) {
LAB_14207b044:
                    // WARNING: Subroutine does not return
      std::thread::local::panic_access_error();
    }
    if (*plVar12 != 0) {
LAB_14207b050:
                    // WARNING: Subroutine does not return
      core::cell::panic_already_borrowed();
    }
    *plVar12 = -1;
    puffin::ThreadProfiler::end_scope();
    *plVar12 = *plVar12 + 1;
  }
  return;
}

