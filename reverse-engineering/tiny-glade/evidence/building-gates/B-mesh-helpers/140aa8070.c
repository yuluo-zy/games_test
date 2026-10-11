
void __cdecl
country_core::components::transient_mesh::TransientMeshRegistry::deferred_delete_mesh(void)

{
  char cVar1;
  longlong lVar2;
  undefined *puVar3;
  byte extraout_AL;
  char extraout_AL_00;
  char *in_RCX;
  undefined8 in_RDX;
  byte local_34;
  
  LOCK();
  cVar1 = *in_RCX;
  if (cVar1 == '\0') {
    *in_RCX = '\x01';
  }
  UNLOCK();
  if (cVar1 != '\0') {
    std::sys::sync::mutex::futex::Mutex::lock_contended();
  }
  puVar3 = __imp__ZN3std9panicking11panic_count18GLOBAL_PANIC_COUNT17h923f4fc2d5dc75a1E;
  if ((*(ulonglong *)__imp__ZN3std9panicking11panic_count18GLOBAL_PANIC_COUNT17h923f4fc2d5dc75a1E &
      0x7fffffffffffffff) == 0) {
    local_34 = 0;
    cVar1 = in_RCX[1];
  }
  else {
    std::panicking::panic_count::is_zero_slow_path();
    local_34 = extraout_AL ^ 1;
    cVar1 = in_RCX[1];
  }
  if (cVar1 != '\0') {
                    /* WARNING: Subroutine does not return */
    core::result::unwrap_failed();
  }
  lVar2 = *(longlong *)(in_RCX + 0x18);
  if (lVar2 == *(longlong *)(in_RCX + 8)) {
    alloc::raw_vec::RawVec::
    grow_one<bevy_ecs::event::base::EventInstance<tiny_glade::app_config::WriteSettingsFileCmd>,alloc::alloc::Global>
              ();
  }
  *(undefined8 *)(*(longlong *)(in_RCX + 0x10) + lVar2 * 8) = in_RDX;
  *(longlong *)(in_RCX + 0x18) = lVar2 + 1;
  if (((local_34 == 0) && ((*(ulonglong *)puVar3 & 0x7fffffffffffffff) != 0)) &&
     (std::panicking::panic_count::is_zero_slow_path(), extraout_AL_00 == '\0')) {
    in_RCX[1] = '\x01';
  }
  LOCK();
  cVar1 = *in_RCX;
  *in_RCX = '\0';
  UNLOCK();
  if (cVar1 != '\x02') {
    return;
  }
  std::sys::sync::mutex::futex::Mutex::wake();
  return;
}

