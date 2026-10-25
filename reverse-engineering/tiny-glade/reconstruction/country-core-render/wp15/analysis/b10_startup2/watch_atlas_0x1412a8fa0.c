// addr=0x1412a8fa0 name=watch_atlas
// signature=void __cdecl watch_atlas(void)


void __cdecl country_core::startup::load_prefabs::PrefabWatcher::watch_atlas(void)

{
  longlong lVar1;
  longlong *plVar2;
  code *pcVar3;
  int *in_RCX;
  longlong *in_RDX;
  longlong local_50;
  
  if (*in_RCX != 3) {
    plVar2 = *(longlong **)(in_RCX + 0xe);
    LOCK();
    lVar1 = *plVar2;
    *plVar2 = *plVar2 + 1;
    UNLOCK();
    if (*plVar2 == 0 || SCARRY8(lVar1,1) != *plVar2 < 0) {
                    // WARNING: Does not return
      pcVar3 = (code *)invalidInstructionException();
      (*pcVar3)();
    }
    hotwatch::GenericHotwatch::
    watch<notify::windows::ReadDirectoryChangesWatcher,ref_<std::path::PathBuf>,country_core::startup::load_prefabs::impl_1::watch_atlas::closure_env_0>
              ();
    if (local_50 != -0x7ffffffffffffffc) {
                    // WARNING: Subroutine does not return
      core::result::unwrap_failed();
    }
  }
  if (*in_RDX == 0) {
    return;
  }
  std::alloc::__default_lib_allocator::__rust_dealloc();
  return;
}


