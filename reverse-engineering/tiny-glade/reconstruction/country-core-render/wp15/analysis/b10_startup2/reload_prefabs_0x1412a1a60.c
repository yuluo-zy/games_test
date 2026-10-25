// addr=0x1412a1a60 name=reload_prefabs
// signature=void __cdecl reload_prefabs(void)


// WARNING: Removing unreachable block (ram,0x0001412a1b60)
// WARNING: Removing unreachable block (ram,0x0001412a1bae)
// WARNING: Removing unreachable block (ram,0x0001412a1bbe)

void __cdecl country_core::startup::load_prefabs::reload_prefabs(void)

{
  char *pcVar1;
  char cVar2;
  longlong lVar3;
  longlong *plVar4;
  undefined *puVar5;
  byte extraout_AL;
  byte bVar6;
  char extraout_AL_00;
  char extraout_AL_01;
  longlong *extraout_RAX;
  longlong *extraout_RAX_00;
  longlong *in_RCX;
  longlong in_RDX;
  bool bVar7;
  
  lVar3 = *(longlong *)(*in_RCX + 0x30);
  pcVar1 = (char *)(lVar3 + 0x10);
  LOCK();
  bVar7 = *(char *)(lVar3 + 0x10) == '\0';
  if (bVar7) {
    *(char *)(lVar3 + 0x10) = '\x01';
  }
  UNLOCK();
  if (!bVar7) {
    std::sys::sync::mutex::futex::Mutex::lock_contended();
  }
  puVar5 = __imp__ZN3std9panicking11panic_count18GLOBAL_PANIC_COUNT17h923f4fc2d5dc75a1E;
  if ((*(ulonglong *)__imp__ZN3std9panicking11panic_count18GLOBAL_PANIC_COUNT17h923f4fc2d5dc75a1E &
      0x7fffffffffffffff) == 0) {
    bVar6 = 0;
    cVar2 = *(char *)(lVar3 + 0x11);
  }
  else {
    std::panicking::panic_count::is_zero_slow_path();
    bVar6 = extraout_AL ^ 1;
    cVar2 = *(char *)(lVar3 + 0x11);
  }
  if (cVar2 != '\0') {
                    // WARNING: Subroutine does not return
    core::result::unwrap_failed();
  }
  if (*(longlong *)(lVar3 + 0x30) == 0) {
    if (((bVar6 == 0) && ((*(ulonglong *)puVar5 & 0x7fffffffffffffff) != 0)) &&
       (std::panicking::panic_count::is_zero_slow_path(), extraout_AL_01 == '\0')) {
      *(undefined1 *)(lVar3 + 0x11) = 1;
    }
    LOCK();
    cVar2 = *pcVar1;
    *pcVar1 = '\0';
    UNLOCK();
    if (cVar2 == '\x02') {
      std::sys::sync::mutex::futex::Mutex::wake();
      return;
    }
  }
  else {
    if (((bVar6 == 0) && ((*(ulonglong *)puVar5 & 0x7fffffffffffffff) != 0)) &&
       (std::panicking::panic_count::is_zero_slow_path(), extraout_AL_00 == '\0')) {
      *(undefined1 *)(lVar3 + 0x11) = 1;
    }
    LOCK();
    cVar2 = *pcVar1;
    *pcVar1 = '\0';
    UNLOCK();
    if (cVar2 == '\x02') {
      std::sys::sync::mutex::futex::Mutex::wake();
      core::ops::function::FnOnce::
      call_once<std::hash::random::impl_0::new::KEYS::constant_0::closure_env_1,tuple_<enum2_<core::option::Option<ref_mut_<enum2_<core::option::Option<core::cell::Cell<tuple_<u64,u64>_>_>_>_>_>_>_>_>
                ();
      plVar4 = extraout_RAX_00;
    }
    else {
      core::ops::function::FnOnce::
      call_once<std::hash::random::impl_0::new::KEYS::constant_0::closure_env_1,tuple_<enum2_<core::option::Option<ref_mut_<enum2_<core::option::Option<core::cell::Cell<tuple_<u64,u64>_>_>_>_>_>_>_>_>
                ();
      plVar4 = extraout_RAX;
    }
    if (plVar4 == (longlong *)0x0) {
                    // WARNING: Subroutine does not return
      std::thread::local::panic_access_error();
    }
    *plVar4 = *plVar4 + 1;
    **(undefined4 **)(in_RDX + 0x10) = *(undefined4 *)(in_RDX + 0x1c);
    *(undefined4 *)in_RCX[2] = *(undefined4 *)((longlong)in_RCX + 0x1c);
    load_prefabs_impl();
    hashbrown::raw::RawTableInner::
    drop_inner_table<tuple_<asset::prefab::PrefabName,tuple_<>_>,alloc::alloc::Global>();
  }
  return;
}


