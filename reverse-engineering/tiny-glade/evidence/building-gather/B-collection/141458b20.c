
void __cdecl
country_core::utils::dirty_tiles::impl$3::
iter_dirty_tiles<country_core::systems::wall::wall_state::publish_public_wall_state::gather_lazy_update_sources::closure_env_0>
          (void)

{
  longlong *plVar1;
  ulonglong uVar2;
  undefined8 local_58;
  undefined8 local_48;
  
  tabloid::Subscriber::take<country_core::utils::dirty_tiles::DirtyTiles>();
  if (local_58 != (ulonglong *)0x0) {
    DirtyTiles::
    iter_tiles<country_core::systems::wall::wall_state::publish_public_wall_state::gather_lazy_update_sources::closure_env_0>
              ();
    parking_lot_core::parking_lot::deadlock_impl::release_resource();
    parking_lot_core::parking_lot::deadlock_impl::release_resource();
    LOCK();
    uVar2 = *local_58;
    *local_58 = *local_58 - 0x10;
    UNLOCK();
    if ((uVar2 & 0xfffffffffffffff2) == 0x12) {
      parking_lot::raw_rwlock::RawRwLock::unlock_shared_slow();
    }
    LOCK();
    plVar1 = (longlong *)(local_48 + -0x10);
    *plVar1 = *plVar1 + -1;
    UNLOCK();
    if (*plVar1 == 0) {
      alloc::sync::Arc::
      drop_slow<lock_api::rwlock::RwLock<parking_lot::raw_rwlock::RawRwLock,tabloid::Publication<country_core::utils::dirty_tiles::DirtyTiles>_>,alloc::alloc::Global>
                ();
    }
  }
  return;
}

