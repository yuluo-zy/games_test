// address 0x140991c50
// name owned_subset_mut
// signature void __cdecl owned_subset_mut(void)

void __cdecl country_core::render::nani::gpu_world::GpuWorld::owned_subset_mut(void)

{
  longlong *plVar1;
  longlong lVar2;
  longlong in_RCX;
  byte in_DL;
  bool bVar3;
  
  plVar1 = (longlong *)(in_RCX + (ulonglong)((uint)in_DL * 0xc0));
  LOCK();
  bVar3 = *plVar1 == 0;
  if (bVar3) {
    *plVar1 = 8;
  }
  UNLOCK();
  if (!bVar3) {
    parking_lot::raw_rwlock::RawRwLock::lock_exclusive_slow();
  }
  parking_lot_core::parking_lot::deadlock_impl::acquire_resource();
  parking_lot_core::parking_lot::deadlock_impl::acquire_resource();
  lVar2 = *(longlong *)(in_RCX + (ulonglong)((uint)in_DL * 0xc0) + 8);
  if (SBORROW8(0,lVar2)) {
                    // WARNING: Subroutine does not return
    core::panicking::panic_fmt();
  }
  if (lVar2 == -0x7fffffffffffffff) {
                    // WARNING: Subroutine does not return
    core::option::unwrap_failed();
  }
  return;
}

