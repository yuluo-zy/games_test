
void __cdecl tiny_glade::screenshot::finish_screenshot_capture(void)

{
  byte *pbVar1;
  byte bVar2;
  byte bVar3;
  byte bVar4;
  bool bVar5;
  char extraout_AL;
  uint uVar6;
  longlong extraout_RAX;
  longlong extraout_RAX_00;
  longlong *extraout_RAX_01;
  longlong *extraout_RAX_02;
  longlong *extraout_RAX_03;
  longlong *extraout_RAX_04;
  longlong *plVar7;
  ulonglong uVar8;
  longlong lVar9;
  undefined *puVar10;
  longlong lVar11;
  longlong lVar12;
  bool bVar13;
  
  puffin::are_scopes_on();
  if (extraout_AL == '\0') {
    bVar5 = false;
  }
  else {
    uVar8 = 0x31;
    core::str::impl$0::rfind<ref_<str_>_>();
    if (extraout_RAX == 1) {
      if (uVar8 != 0) {
        if (uVar8 < 0x31) {
          if ((char)(&DAT_1429bc60e)[uVar8] < -0x40) {
LAB_14031ea84:
                    // WARNING: Subroutine does not return
            core::str::slice_error_fail();
          }
        }
        else if (uVar8 != 0x31) goto LAB_14031ea84;
      }
      core::str::impl$0::rfind<ref_<str_>_>();
      if ((extraout_RAX_00 == 1) && (uVar8 != 0xfffffffffffffffe)) {
        if (uVar8 + 2 < 0x31) {
          if ((char)(&UNK_1429bc610)[uVar8] < -0x40) {
LAB_14031eae2:
                    // WARNING: Subroutine does not return
            core::str::slice_error_fail();
          }
        }
        else if (uVar8 + 2 != 0x31) goto LAB_14031eae2;
      }
    }
    puVar10 = &DAT_1429bc763;
    do {
      bVar2 = puVar10[-1];
      uVar6 = (uint)(char)bVar2;
      if ((char)bVar2 < '\0') {
        bVar3 = puVar10[-2];
        if ((char)bVar3 < -0x40) {
          bVar4 = puVar10[-3];
          if ((char)bVar4 < -0x40) {
            pbVar1 = puVar10 + -4;
            puVar10 = puVar10 + -4;
            uVar6 = bVar4 & 0x3f | (*pbVar1 & 7) << 6;
          }
          else {
            puVar10 = puVar10 + -3;
            uVar6 = bVar4 & 0xf;
          }
          uVar6 = bVar3 & 0x3f | uVar6 << 6;
        }
        else {
          puVar10 = puVar10 + -2;
          uVar6 = bVar3 & 0x1f;
        }
        uVar6 = (uint)(bVar2 & 0x3f) | uVar6 << 6;
        if (uVar6 != 0x5c) goto LAB_14031eba4;
LAB_14031ebb6:
        if (puVar10 == &UNK_1429bc73f) break;
        if (puVar10 + -0x1429bc73f < (undefined *)0x23) {
          if (-0x41 < (char)puVar10[1]) break;
        }
        else if (puVar10 + -0x1429bc73f == (undefined *)0x23) break;
                    // WARNING: Subroutine does not return
        core::str::slice_error_fail();
      }
      puVar10 = puVar10 + -1;
      if (uVar6 == 0x5c) goto LAB_14031ebb6;
LAB_14031eba4:
      if (uVar6 == 0x2f) goto LAB_14031ebb6;
    } while (puVar10 != &DAT_1429bc740);
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_01 == 1) {
      plVar7 = extraout_RAX_01 + 1;
    }
    else if ((*extraout_RAX_01 != 0) ||
            (std::sys::thread_local::native::lazy::Storage::
             initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                       (), plVar7 = extraout_RAX_04, extraout_RAX_04 == (longlong *)0x0))
    goto LAB_14031eda9;
    if (*plVar7 != 0) goto LAB_14031edb5;
    *plVar7 = -1;
    puffin::ThreadProfiler::begin_scope();
    *plVar7 = *plVar7 + 1;
    bVar5 = true;
  }
  LOCK();
  bVar13 = ACTIVE_SCREENSHOT_REQUESTS == '\0';
  if (bVar13) {
    ACTIVE_SCREENSHOT_REQUESTS = '\x01';
  }
  UNLOCK();
  if (!bVar13) {
    parking_lot::raw_mutex::RawMutex::lock_slow();
  }
  parking_lot_core::parking_lot::deadlock_impl::acquire_resource();
  lVar9 = DAT_14362b068 * 0x98 + DAT_14362b060;
  lVar12 = DAT_14362b060;
  while (lVar11 = lVar12, lVar11 != lVar9) {
    lVar12 = lVar11 + 0x98;
    if ((*(char *)(lVar11 + 0x96) != '\x06') && (*(int *)(lVar11 + 0x50) != 2)) {
      if (*(int *)(lVar11 + 0x50) == 1) {
        rhapsody::raw::device::GraphicsDevice::wait_idle();
        read_back_screenshot();
        *(undefined4 *)(lVar11 + 0x50) = 2;
      }
      else if (*(int *)(lVar11 + 0x54) == 0) {
        read_back_screenshot();
        *(undefined4 *)(lVar11 + 0x50) = 2;
      }
      else {
        *(int *)(lVar11 + 0x54) = *(int *)(lVar11 + 0x54) + -1;
      }
    }
  }
  alloc::vec::Vec::
  retain<tiny_glade::screenshot::ActiveScreenshotRequest,alloc::alloc::Global,tiny_glade::screenshot::finish_screenshot_capture::closure_env_0>
            ();
  parking_lot_core::parking_lot::deadlock_impl::release_resource();
  LOCK();
  bVar13 = ACTIVE_SCREENSHOT_REQUESTS == '\x01';
  if (bVar13) {
    ACTIVE_SCREENSHOT_REQUESTS = '\0';
  }
  UNLOCK();
  if (!bVar13) {
    parking_lot::raw_mutex::RawMutex::unlock_slow();
  }
  if (bVar5) {
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_02 == 1) {
      plVar7 = extraout_RAX_02 + 1;
    }
    else if ((*extraout_RAX_02 != 0) ||
            (std::sys::thread_local::native::lazy::Storage::
             initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                       (), plVar7 = extraout_RAX_03, extraout_RAX_03 == (longlong *)0x0)) {
LAB_14031eda9:
                    // WARNING: Subroutine does not return
      std::thread::local::panic_access_error();
    }
    if (*plVar7 != 0) {
LAB_14031edb5:
                    // WARNING: Subroutine does not return
      core::cell::panic_already_borrowed();
    }
    *plVar7 = -1;
    puffin::ThreadProfiler::end_scope();
    *plVar7 = *plVar7 + 1;
  }
  return;
}

