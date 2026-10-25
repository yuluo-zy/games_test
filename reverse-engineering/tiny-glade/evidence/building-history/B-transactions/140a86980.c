
/* WARNING: Removing unreachable block (ram,0x000140a86a46) */
/* WARNING: Removing unreachable block (ram,0x000140a86a8a) */
/* WARNING: Removing unreachable block (ram,0x000140a86a93) */
/* WARNING: Removing unreachable block (ram,0x000140a86a58) */

void __cdecl country_core::resources::history::History::finish_potential_mutation(void)

{
  longlong lVar1;
  longlong *plVar2;
  undefined8 extraout_RAX;
  longlong in_RCX;
  longlong lVar3;
  char in_DL;
  undefined8 *puVar4;
  bool bVar5;
  
  if (in_DL != '\0') {
    puVar4 = *(undefined8 **)(in_RCX + 0x20);
    lVar1 = *(longlong *)(in_RCX + 0x28);
    *(undefined8 *)(in_RCX + 0x28) = 0;
    if (lVar1 != 0) {
      lVar3 = 1;
      do {
        plVar2 = (longlong *)*puVar4;
        LOCK();
        *plVar2 = *plVar2 + -1;
        UNLOCK();
        if (*plVar2 == 0) {
          alloc::sync::Arc::
          drop_slow<lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,tiny_glade::crash_pickup::crash_payload::CrashMiscInfo>,alloc::alloc::Global>
                    ();
        }
        puVar4 = puVar4 + 2;
        bVar5 = lVar3 != lVar1;
        lVar3 = lVar3 + 1;
      } while (bVar5);
    }
    std::thread::local::LocalKey::
    with<fastrand::Rng,fastrand::u64::closure_env_0<core::ops::range::RangeInclusive<u64>_>,u64>();
    *(undefined8 *)(in_RCX + 0x60) = extraout_RAX;
  }
  return;
}

