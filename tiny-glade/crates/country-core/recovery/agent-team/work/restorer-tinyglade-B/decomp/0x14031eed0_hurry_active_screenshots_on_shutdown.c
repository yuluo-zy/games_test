// hurry_active_screenshots_on_shutdown @ 0x14031eed0
// sig: void __cdecl hurry_active_screenshots_on_shutdown(void)


// WARNING: Removing unreachable block (ram,0x00014031f0e9)
// WARNING: Removing unreachable block (ram,0x00014031f0f6)
// WARNING: Removing unreachable block (ram,0x00014031f11b)

void __cdecl tiny_glade::screenshot::hurry_active_screenshots_on_shutdown(void)

{
  longlong *plVar1;
  ulonglong *puVar2;
  longlong lVar3;
  ulonglong uVar4;
  ulonglong uVar5;
  longlong *plVar6;
  longlong extraout_RAX;
  undefined8 *in_RCX;
  longlong lVar7;
  ulonglong uVar8;
  longlong lVar9;
  longlong *plVar10;
  ulonglong uVar11;
  ulonglong uVar12;
  ulonglong uVar13;
  ulonglong uVar14;
  bool bVar15;
  
  puVar2 = (ulonglong *)*in_RCX;
  lVar3 = in_RCX[1];
  uVar4 = *puVar2;
  uVar11 = 0;
  uVar12 = uVar4 - *(ulonglong *)(lVar3 + 0x18);
  if (uVar4 < *(ulonglong *)(lVar3 + 0x18)) {
    uVar12 = uVar11;
  }
  uVar8 = uVar4 - *(ulonglong *)(lVar3 + 0x38);
  if (uVar4 < *(ulonglong *)(lVar3 + 0x38)) {
    uVar8 = uVar11;
  }
  uVar4 = *(ulonglong *)(lVar3 + 0x30);
  uVar13 = uVar4 - uVar8;
  if (uVar4 < uVar8) {
    uVar13 = uVar11;
  }
  uVar5 = *(ulonglong *)(lVar3 + 0x10);
  uVar14 = uVar5 - uVar12;
  if (uVar5 < uVar12) {
    uVar14 = uVar11;
  }
  uVar11 = *(ulonglong *)(lVar3 + 0x40);
  *puVar2 = uVar11 - (uVar14 + uVar13);
  if ((uVar5 < uVar12 || uVar5 - uVar12 == 0) && (uVar4 < uVar8 || uVar4 - uVar8 == 0)) {
    return;
  }
  *puVar2 = uVar11;
  LOCK();
  bVar15 = ACTIVE_SCREENSHOT_REQUESTS == '\0';
  if (bVar15) {
    ACTIVE_SCREENSHOT_REQUESTS = '\x01';
  }
  UNLOCK();
  if (!bVar15) {
    parking_lot::raw_mutex::RawMutex::lock_slow();
  }
  parking_lot_core::parking_lot::deadlock_impl::acquire_resource();
  lVar3 = DAT_14362b060;
  if (DAT_14362b068 != 0) {
    lVar7 = DAT_14362b068 * 0x98;
    lVar9 = 0;
    do {
      *(undefined4 *)(lVar3 + 0x50 + lVar9) = 1;
      lVar9 = lVar9 + 0x98;
    } while (lVar7 - lVar9 != 0);
  }
  parking_lot_core::parking_lot::deadlock_impl::release_resource();
  LOCK();
  bVar15 = ACTIVE_SCREENSHOT_REQUESTS == '\x01';
  if (bVar15) {
    ACTIVE_SCREENSHOT_REQUESTS = '\0';
  }
  UNLOCK();
  if (!bVar15) {
    parking_lot::raw_mutex::RawMutex::unlock_slow();
  }
  if (DAT_14362b070 == 2) {
    LOCK();
    bVar15 = DAT_14362b080 == '\0';
    if (bVar15) {
      DAT_14362b080 = '\x01';
    }
    UNLOCK();
  }
  else {
    once_cell::imp::OnceCell::
    initialize<lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,alloc::vec::Vec<std::thread::JoinHandle<tuple_<>_>,alloc::alloc::Global>_>,once_cell::sync::impl_6::get_or_init::closure_env_0<lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,alloc::vec::Vec<std::thread::JoinHandle<tuple_<>_>,alloc::alloc::Global>_>,once_cell::sync::impl_11::force::closure_env_0<lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,alloc::vec::Vec<std::thread::JoinHandle<tuple_<>_>,alloc::alloc::Global>_>,lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,alloc::vec::Vec<std::thread::JoinHandle<tuple_<>_>,alloc::alloc::Global>_>_(*)()>_>,enum2_<once_cell::sync::impl_6::get_or_init::Void>_>
              ();
    LOCK();
    bVar15 = DAT_14362b080 == '\0';
    if (bVar15) {
      DAT_14362b080 = '\x01';
    }
    UNLOCK();
  }
  if (!bVar15) {
    parking_lot::raw_mutex::RawMutex::lock_slow();
  }
  parking_lot_core::parking_lot::deadlock_impl::acquire_resource();
  lVar3 = DAT_14362b098;
  DAT_14362b098 = 0;
  plVar1 = DAT_14362b090 + lVar3 * 3;
  plVar6 = DAT_14362b090;
  do {
    plVar10 = plVar6;
    if (plVar10 == plVar1) {
LAB_14031f122:
      parking_lot_core::parking_lot::deadlock_impl::release_resource();
      LOCK();
      bVar15 = DAT_14362b080 == '\x01';
      if (bVar15) {
        DAT_14362b080 = '\0';
      }
      UNLOCK();
      if (bVar15) {
        return;
      }
      parking_lot::raw_mutex::RawMutex::unlock_slow();
      return;
    }
    plVar6 = plVar10 + 3;
    if (*plVar10 == 0) {
      if (plVar1 != plVar6) {
        core::ptr::drop_in_place<slice2_<std::thread::JoinHandle<tuple_<>_>_>_>();
      }
      goto LAB_14031f122;
    }
    std::thread::JoinHandle::join<tuple_<>_>();
    if (extraout_RAX != 0) {
      if ((code *)*plVar10 != (code *)0x0) {
        (*(code *)*plVar10)(extraout_RAX);
      }
      if (plVar10[1] != 0) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
    }
  } while( true );
}

