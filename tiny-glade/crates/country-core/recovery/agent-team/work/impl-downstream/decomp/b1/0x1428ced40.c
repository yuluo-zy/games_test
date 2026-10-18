
// WARNING: Removing unreachable block (ram,0x0001428cede8)
// WARNING: Removing unreachable block (ram,0x0001428cedf2)

void __cdecl
std::sys::thread_local::native::lazy::Storage::
initialize<alloc::sync::Arc<bevy_tasks::thread_executor::ThreadExecutor,alloc::alloc::Global>,tuple_<>,alloc::sync::Arc<bevy_tasks::thread_executor::ThreadExecutor,alloc::alloc::Global>_(*)()>
          (void)

{
  longlong lVar1;
  longlong *plVar2;
  undefined8 *extraout_RAX;
  undefined8 *puVar3;
  longlong *in_RCX;
  undefined8 *in_RDX;
  undefined8 local_38;
  undefined8 uStack_30;
  
  if (in_RDX != (undefined8 *)0x0) {
    puVar3 = (undefined8 *)*in_RDX;
    *in_RDX = 0;
    if (puVar3 != (undefined8 *)0x0) goto LAB_1428ced91;
  }
  bevy_tasks::thread_executor::ThreadExecutor::new();
  std::alloc::__default_lib_allocator::__rust_alloc();
  if (extraout_RAX == (undefined8 *)0x0) {
                    // WARNING: Subroutine does not return
    ::alloc::alloc::handle_alloc_error();
  }
  extraout_RAX[2] = local_38;
  extraout_RAX[3] = uStack_30;
  *extraout_RAX = 1;
  extraout_RAX[1] = 1;
  puVar3 = extraout_RAX;
LAB_1428ced91:
  lVar1 = *in_RCX;
  plVar2 = (longlong *)in_RCX[1];
  *in_RCX = 1;
  in_RCX[1] = (longlong)puVar3;
  if (lVar1 == 0) {
    destructors::list::register();
  }
  else if (lVar1 == 1) {
    LOCK();
    *plVar2 = *plVar2 + -1;
    UNLOCK();
    if (*plVar2 == 0) {
      ::alloc::sync::Arc::
      drop_slow<bevy_tasks::thread_executor::ThreadExecutor,alloc::alloc::Global>();
    }
  }
  return;
}

