
void __cdecl country_core::systems::ui::resources::ui_state::UiState::add_with_id(void)

{
  longlong *plVar1;
  void *_Dst;
  longlong *extraout_RAX;
  longlong in_RCX;
  void *in_R8;
  bool bVar2;
  undefined8 local_110;
  undefined8 local_108;
  undefined1 local_100 [176];
  void *local_50;
  longlong *local_48;
  undefined1 local_39;
  undefined8 local_38;
  
  local_38 = 0xfffffffffffffffe;
  LOCK();
  bVar2 = *(longlong *)(in_RCX + 0x60) == 0;
  if (bVar2) {
    *(longlong *)(in_RCX + 0x60) = 8;
  }
  UNLOCK();
  local_50 = in_R8;
  if (!bVar2) {
    local_39 = 1;
    parking_lot::raw_rwlock::RawRwLock::lock_exclusive_slow();
  }
  local_39 = 1;
  parking_lot_core::parking_lot::deadlock_impl::acquire_resource();
  local_39 = 1;
  parking_lot_core::parking_lot::deadlock_impl::acquire_resource();
  local_48 = (longlong *)(in_RCX + 0x60);
  memcpy(local_100,local_50,0xb0);
  local_110 = 1;
  local_108 = 1;
  std::alloc::__default_lib_allocator::__rust_alloc();
  if (_Dst == (void *)0x0) {
                    // WARNING: Subroutine does not return
    alloc::alloc::handle_alloc_error();
  }
  memcpy(_Dst,&local_110,0xc0);
  hashbrown::map::HashMap::
  insert<country_core::systems::ui::resources::ui_state::WidgetId,alloc::sync::Arc<country_core::systems::ui::resources::widget_params::WidgetParams,alloc::alloc::Global>,std::hash::random::RandomState,alloc::alloc::Global>
            ();
  if (extraout_RAX != (longlong *)0x0) {
    LOCK();
    *extraout_RAX = *extraout_RAX + -1;
    UNLOCK();
    if (*extraout_RAX == 0) {
      alloc::sync::Arc::
      drop_slow<country_core::systems::ui::resources::widget_params::WidgetParams,alloc::alloc::Global>
                ();
    }
  }
  plVar1 = local_48;
  local_39 = 0;
  parking_lot_core::parking_lot::deadlock_impl::release_resource();
  local_39 = 0;
  parking_lot_core::parking_lot::deadlock_impl::release_resource();
  LOCK();
  bVar2 = *plVar1 == 8;
  if (bVar2) {
    *plVar1 = 0;
  }
  UNLOCK();
  if (!bVar2) {
    local_39 = 0;
    parking_lot::raw_rwlock::RawRwLock::unlock_exclusive_slow();
  }
  return;
}

