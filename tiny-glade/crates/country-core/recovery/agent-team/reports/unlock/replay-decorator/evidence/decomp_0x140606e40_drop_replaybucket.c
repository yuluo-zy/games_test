
void __cdecl
core::ptr::drop_in_place<country_core::resources::history::replay_cmd_buffer::ReplayBucket>(void)

{
  longlong lVar1;
  longlong *plVar2;
  longlong *in_RCX;
  longlong local_38;
  
  lVar1 = in_RCX[2];
  local_38 = 0;
  while (lVar1 != local_38) {
    local_38 = local_38 + 1;
    drop_in_place<enum2_<country_core::resources::history::replay_cmd_buffer::ReplayCmd>_>();
  }
  if (*in_RCX != 0) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  plVar2 = (longlong *)in_RCX[3];
  if (plVar2 != (longlong *)0x0) {
    LOCK();
    *plVar2 = *plVar2 + -1;
    UNLOCK();
    if (*plVar2 == 0) {
      ::alloc::sync::Arc::
      drop_slow<slice2_<country_core::resources::walls::decorator::DecoratorId>,alloc::alloc::Global>
                ();
    }
  }
  plVar2 = (longlong *)in_RCX[5];
  if (plVar2 != (longlong *)0x0) {
    LOCK();
    *plVar2 = *plVar2 + -1;
    UNLOCK();
    if (*plVar2 == 0) {
      ::alloc::sync::Arc::
      drop_slow<country_core::resources::history::decorator_edit::DecoratorHistoryDiff,alloc::alloc::Global>
                ();
    }
  }
  plVar2 = (longlong *)in_RCX[6];
  if (plVar2 != (longlong *)0x0) {
    LOCK();
    *plVar2 = *plVar2 + -1;
    UNLOCK();
    if (*plVar2 == 0) {
      ::alloc::sync::Arc::
      drop_slow<country_core::resources::stairs::stairs_state::StairsHistoryDiff,alloc::alloc::Global>
                ();
      return;
    }
  }
  return;
}

