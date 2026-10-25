
void __cdecl tabloid::Subscriber::take<country_core::utils::dirty_tiles::DirtyTiles>(void)

{
  ulonglong *puVar1;
  ulonglong uVar2;
  longlong lVar3;
  longlong lVar4;
  ulonglong uVar5;
  longlong *in_RCX;
  longlong *in_RDX;
  bool bVar6;
  
  lVar3 = *in_RDX;
  LOCK();
  bVar6 = *(char *)(lVar3 + 0x10) == '\0';
  if (bVar6) {
    *(char *)(lVar3 + 0x10) = '\x01';
  }
  UNLOCK();
  if (!bVar6) {
    parking_lot::raw_mutex::RawMutex::lock_slow();
  }
  parking_lot_core::parking_lot::deadlock_impl::acquire_resource();
  lVar4 = *(longlong *)(lVar3 + 0x20);
  *(undefined8 *)(lVar3 + 0x20) = 0;
  if (lVar4 == 0) {
    *in_RCX = 0;
    goto LAB_141247df7;
  }
  puVar1 = (ulonglong *)(lVar4 + 0x10);
  uVar5 = *(ulonglong *)(lVar4 + 0x10);
  if (uVar5 < 0xfffffffffffffff0 && (char)(((uint)uVar5 & 8) >> 3) == '\0') {
    LOCK();
    uVar2 = *puVar1;
    if (uVar5 == uVar2) {
      *puVar1 = uVar5 + 0x10;
    }
    UNLOCK();
    if (uVar5 != uVar2) goto LAB_141247cff;
  }
  else {
LAB_141247cff:
    parking_lot::raw_rwlock::RawRwLock::lock_shared_slow();
  }
  parking_lot_core::parking_lot::deadlock_impl::acquire_resource();
  parking_lot_core::parking_lot::deadlock_impl::acquire_resource();
  *(undefined8 *)(lVar3 + 0x18) = *(undefined8 *)(lVar4 + 0x218);
  parking_lot_core::parking_lot::deadlock_impl::release_resource();
  parking_lot_core::parking_lot::deadlock_impl::release_resource();
  LOCK();
  uVar5 = *puVar1;
  *puVar1 = *puVar1 - 0x10;
  UNLOCK();
  if ((uVar5 & 0xfffffffffffffff2) == 0x12) {
    parking_lot::raw_rwlock::RawRwLock::unlock_shared_slow();
  }
  puVar1 = (ulonglong *)(lVar4 + 0x10);
  uVar5 = *(ulonglong *)(lVar4 + 0x10);
  if (uVar5 < 0xfffffffffffffff0 && (char)(((uint)uVar5 & 8) >> 3) == '\0') {
    LOCK();
    uVar2 = *puVar1;
    if (uVar5 == uVar2) {
      *puVar1 = uVar5 + 0x10;
    }
    UNLOCK();
    if (uVar5 != uVar2) goto LAB_141247da7;
  }
  else {
LAB_141247da7:
    parking_lot::raw_rwlock::RawRwLock::lock_shared_slow();
  }
  parking_lot_core::parking_lot::deadlock_impl::acquire_resource();
  parking_lot_core::parking_lot::deadlock_impl::acquire_resource();
  *in_RCX = (longlong)puVar1;
  in_RCX[1] = lVar4 + 0x18;
  in_RCX[2] = (longlong)puVar1;
LAB_141247df7:
  parking_lot_core::parking_lot::deadlock_impl::release_resource();
  LOCK();
  bVar6 = *(char *)(lVar3 + 0x10) == '\x01';
  if (bVar6) {
    *(char *)(lVar3 + 0x10) = '\0';
  }
  UNLOCK();
  if (!bVar6) {
    parking_lot::raw_mutex::RawMutex::unlock_slow();
  }
  return;
}

